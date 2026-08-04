import { mat4Invert, mat4LookAt, mat4Mul, mat4Perspective, v3 } from "../core/math";
import { GPUContext, initGPU } from "../engine/gpu";
import { FLUID_MAX_SOURCES, FLUID_SOURCE_STRIDE, FluidSim } from "../engine/fluid";
import { Smoke } from "../game/smoke";
import { ControlSpec, GroupSpec, TweakPanel } from "../ui/panel";
import viewSrc from "../shaders/smokeview.wgsl?raw";
import sparkSrc from "../shaders/sparks.wgsl?raw";

// ---------------------------------------------------------------------------
// /demo/dynamics — the fluid solver and nothing else.
//
// No path tracer, no BVH, no denoiser, no GI, no level. One fluid lattice and
// one fragment shader that marches it. That is the whole page.
//
// It exists because tuning smoke MOTION through the real renderer does not
// work. Measured: 2.9 s per frame under software rasterisation with bounces at
// zero and reservoir reuse off, because ~14 compute passes a frame cost what
// they cost regardless of resolution. And the tracer's output then goes through
// temporal accumulation and an a-trous filter whose purpose is to destroy
// high-frequency spatial detail — the exact thing that distinguishes smoke from
// porridge. So the dynamics were being judged through a machine designed to
// hide them, one look every few minutes.
//
// Here a frame is milliseconds and the density field is shown as it is. When
// the motion is right, the numbers move to game/smoke.ts and the look is the
// renderer's problem again.
// ---------------------------------------------------------------------------

/**
 * 4 x 3 x 4 m at 3.125 cm — 128 x 96 x 128.
 *
 * Twice the linear resolution of the grenade page's lattice over a quarter of
 * the volume. Resolution is the dominant lever on whether a cloud has internal
 * structure at all: below roughly a hundred cells across the source there is
 * nothing for vorticity to curl and every scheme produces the same lozenge.
 */
const DIMS: [number, number, number] = [128, 96, 128];
const CELL = 0.03125;
const ORIGIN: [number, number, number] = [
  -(DIMS[0] * CELL) / 2, 0, -(DIMS[2] * CELL) / 2,
];

const VIEW_BYTES = 208;

/**
 * Absorption is per unit density per metre, and the solver's densities are
 * ~100, not ~1.
 *
 * The first pass used 1.4, which is the right order for a normalised field and
 * gives an optical depth of ~155 per metre here: opaque within one step, fully
 * self-shadowed, and therefore a black silhouette against a dark sky. The
 * cloud was there — mass 5.1, peak 111 — and completely invisible.
 *
 * 0.035 puts a 0.5 m thick core at an optical depth around 2, which is where a
 * cloud reads as dense but still lets its lit rim through.
 */
const look = {
  steps: 96,
  shadowSteps: 6,
  absorption: 0.035,
  scatter: 1.0,
  // Chosen by looking, not by theory: at 0.12/1.1 the cloud was technically
  // present and visually absent. These are the values the first frame that
  // actually read as smoke was captured at.
  ambient: 0.28,
  exposure: 2.4,
  densityScale: 1.0,
};

const solver = {
  vorticity: 6.0,
  buoyancy: 6.0,
  weight: 0.3,
  dissipation: 0.22,
  cooling: 3.0,
  jacobi: 20,
};

/**
 * One parameter block per emitter, not one shared block.
 *
 * They were shared, and the shared set was really the plume's: `speed`, `push`
 * and `seconds` did nothing at all for a burst, whose life was hardcoded, and
 * `expand` fell back to 70 whenever it was set to zero — so half the panel was
 * inert and one control lied. A burst and a plume have different mechanisms;
 * giving them one set of sliders only hides which ones matter.
 */
// Burst is the default: it is the effect under development, and opening on
// the plume meant every session started by changing a dropdown.
let kind = 1; // 0 plume, 1 burst, 2 vent

const plume = {
  radius: 0.22, density: 160, temp: 6, speed: 2.5, push: 12,
  expand: 0, seconds: 30,
};

/**
 * The flashbang. A detonation is expansion, not momentum: it has no jet
 * velocity and no push, it redistributes what it creates outward and is over
 * in a quarter second. The wisp is a second, much weaker source that survives
 * it, because one source cannot be both the bang and what is left afterwards.
 */
const burst = {
  // 450/s was tuned against a 25 cm lattice. At 3.125 cm the same rate packs
  // 512x more into a cell, and the measured result was peak density 1135 —
  // an optical depth of ~40/m, so the cloud rendered as an opaque surface with
  // every bit of its structure hidden behind it.
  radius: 0.5, density: 140, temp: 30, expand: 90, life: 0.12,
  wispRadius: 0.4, wispDensity: 26, wispTemp: 7, wispRise: 1.2, wispLife: 6,
  /**
   * The canister, not a ball of gas.
   *
   * A flashbang is a cylinder that vents through ports at its ends, so what
   * leaves it is two opposed axial jets and a weaker skirt around the body —
   * never an isotropic sphere. Spawning one spherical source could only ever
   * make a balloon, and no amount of expand or vorticity fixes the shape of
   * something whose shape was decided at the emitter.
   *
   * `seed` picks the can's resting orientation and the per-detonation
   * asymmetry. Deterministic rather than random because this page exists to
   * compare one burst against another, and a burst that differs every run
   * cannot be compared with the one before it — reroll by moving the slider.
   */
  seed: 3,
  /** Half the can's length: how far apart the two end jets sit. */
  halfLength: 0.09,
  /** Speed the end ports vent at. */
  ventSpeed: 7,
  /** 0 both ends equal, 1 one end does everything. Real cans are uneven. */
  asymmetry: 0.45,
  /** Weaker vents around the cylinder's waist. */
  bodyVents: 4,
  bodyFraction: 0.35,
  /** Tilt of the axis away from horizontal, radians. A can lies down. */
  tilt: 0.25,
  // On the ground, not in mid-air.
  //
  // Reference footage of live flashbangs is unanimous and this was the biggest
  // single error: the charge goes off on a surface. The fireball spreads
  // sideways because it cannot go down, the first second is a low wide bank of
  // smoke rather than a ball, and the column that rises does so FROM the
  // ground. Detonating at 1 m gives a free sphere, which is why it read as a
  // balloon however it was tuned.
  height: 0.12,
};

/** The light the bang throws. See flashAt() in smokeview.wgsl. */
const flash = {
  power: 26,
  duration: 0.14,
  shadowSteps: 5,
  /** Seconds since it went off; >= duration means dark. */
  age: 1e9,
};

/**
 * The burning fragments.
 *
 * Ballistic, not fluid: they have their own gravity and drag and do not touch
 * the density field at all. In the reference footage they are what makes the
 * first fifth of a second read as an explosion — several hundred fine bright
 * filaments arcing out and falling — and the smoke that follows is a separate,
 * slower event.
 */
const sparks = {
  count: 260,
  speed: 14,
  spread: 0.55,
  gravity: 16,
  drag: 1.6,
  life: 0.9,
  /** Multiplies the emitted colour; sparks are meant to clip. */
  brightness: 2.6,
  /** Fraction of `life` spent at full brightness before fading out. */
  hold: 0.15,
  /**
   * Tilts the throw from outward (0) toward straight up (1).
   *
   * `spread` already narrows the cone, but narrowing and aiming are different
   * things: a low spread still throws sideways, it just throws sideways in a
   * tighter band. This rotates the whole distribution toward vertical, which
   * is what a can venting against the ground actually does.
   */
  lift: 0.45,
};

/**
 * Smoke trailing the sparks.
 *
 * This replaced a separate "fragment" system that laid its jets out on a
 * Fibonacci sphere. Even coverage was the point of that lattice and it was
 * also its problem: evenly spaced arms read as a symmetrical starburst, and
 * real debris is clumped and uneven. The sparks already have hashed
 * directions, varied speeds, gravity and a floor bounce — they are irregular
 * for free — so hanging the smoke off them costs nothing and removes the
 * symmetry at the source.
 *
 * Only the first `count` sparks smoke, because FLUID_MAX_SOURCES is 32 and the
 * core and wisp have already taken two.
 */
const trail = {
  count: 48,
  /**
   * The radius, and nothing else touches it.
   *
   * It used to be a floor under a speed-derived term — `max(radius, speed *
   * dt * 0.6)` — meant to stop fast trails beading. At 14 m/s that computes
   * 0.42 m, so a slider set to 0.03 silently produced a 0.84 m blob and the
   * trails rendered as giant sheets an order of magnitude bigger than the
   * sparks drawing them. A control that quietly overrides itself is worse than
   * one that is simply too coarse.
   *
   * The beading it was papering over is real: a moving source deposits once
   * per frame, so a trail is continuous only while `speed * dt` stays under
   * about `2 * radius`. At 50 ms frames that is 0.05 * speed — a 0.06 m radius
   * holds together to ~2.4 m/s and dashes above it. That is now a trade the
   * sliders expose rather than one made silently, and dashes are not
   * necessarily wrong: debris trails are broken in the reference footage too.
   */
  radius: 0.05,
  density: 45,
  temp: 5,
  /**
   * Matched to the sparks' life, so the trail draws the whole arc.
   *
   * The sparks are ballistic — gravity 16, drag, floor bounce — and measured,
   * their mean vertical velocity crosses zero between 0.2 s and 0.4 s and
   * their peak height falls from 2.62 m back to the floor by 1.0 s. At the old
   * 0.5 s the trails stopped emitting almost exactly at the apex, so the smoke
   * recorded the throw and none of the fall, and the arc that is right there
   * in the motion never reached the image.
   */
  life: 0.9,
};
const MAX_SPARKS = 512;

const vent = {
  radius: 0.55, density: 200, temp: 0, speed: 9, push: 45,
  expand: 0, seconds: 30, yaw: 0, height: 0.3,
};

const cam = { yaw: 0.6, pitch: 0.22, distance: 4.2, height: 1.2, orbit: true, speed: 0.25 };

function fatal(title: string, body: string): void {
  const el = document.getElementById("fatal");
  if (!el) return;
  document.getElementById("fatal-title")!.textContent = title;
  document.getElementById("fatal-body")!.textContent = body;
  el.classList.add("show");
}

const sl = (
  label: string, min: number, max: number, step: number,
  get: () => number, set: (v: number) => void,
): ControlSpec => ({ kind: "slider", label, min, max, step, get, set });

const tg = (
  label: string, get: () => boolean, set: (v: boolean) => void,
): ControlSpec => ({ kind: "toggle", label, get, set });

async function main(): Promise<void> {
  const app = document.getElementById("app")!;
  const canvas = document.createElement("canvas");
  app.appendChild(canvas);

  let ctx: GPUContext;
  try {
    ctx = await initGPU(canvas, {
      onDeviceLost: (i) => fatal("GPU device lost", i.message || String(i.reason)),
      onUncapturedError: (e) => console.error("[webgpu]", e.message),
    });
  } catch (e) {
    fatal("Startup failed", String(e));
    return;
  }
  const d = ctx.device;

  // The interface texture the solver writes its density into. Nothing samples
  // it but this page, so it is sized to the lattice exactly.
  const volume = d.createTexture({
    label: "dyn-volume",
    dimension: "3d",
    size: { width: DIMS[0], height: DIMS[1], depthOrArrayLayers: DIMS[2] },
    format: "rgba16float",
    usage: GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.TEXTURE_BINDING
      | GPUTextureUsage.COPY_DST | GPUTextureUsage.COPY_SRC,
  });

  // No occupancy baker: there is no level here, so the solver sees an empty
  // box with six walls, which is what a smoke box should be.
  const fluid = await FluidSim.create(d, {
    volume, dims: DIMS, origin: ORIGIN, cell: CELL,
  }, null);
  Object.assign(fluid.tune, solver);

  const smoke = new Smoke();

  // ---- sparks -------------------------------------------------------------
  // Flat arrays rather than objects: this is rebuilt into a vertex buffer every
  // frame, and a few hundred allocations per frame is exactly the shape of
  // garbage that shows up as a stutter rather than as a slow frame.
  const spPos = new Float32Array(MAX_SPARKS * 3);
  const spPrev = new Float32Array(MAX_SPARKS * 3);
  const spVel = new Float32Array(MAX_SPARKS * 3);
  const spAge = new Float32Array(MAX_SPARKS);
  const spLife = new Float32Array(MAX_SPARKS);
  let spCount = 0;
  /** Two vertices per spark, each xyz + rgb. */
  const spVerts = new Float32Array(MAX_SPARKS * 2 * 6);

  function emitSparks(at: { x: number; y: number; z: number }): void {
    spCount = Math.min(sparks.count, MAX_SPARKS);
    for (let i = 0; i < spCount; i++) {
      // Deterministic: this page is for comparing one burst against another,
      // and Math.random would make every run a different explosion.
      // Seeded from the burst, so rerolling the can rerolls the throw with it.
      // Without this the sparks were identical across every seed, which made
      // the bbox identical too and briefly looked like the seed did nothing.
      const h = (k: number): number => {
        const t = Math.sin((i + 1) * 12.9898 + k * 78.233 + burst.seed * 53.17)
          * 43758.5453;
        return t - Math.floor(t);
      };
      // Upward hemisphere, widened by `spread`. A ground burst throws out and
      // up; nothing goes down through the floor.
      const theta = h(1) * Math.PI * 2;
      let cy = Math.pow(h(2), 1 + sparks.spread * 3) * 0.9 + 0.05;
      let r = Math.sqrt(Math.max(0, 1 - cy * cy));
      // Rotate toward vertical rather than just narrowing: lift 1 puts every
      // spark straight up, 0 leaves the raw hemisphere.
      cy = cy + (1 - cy) * sparks.lift;
      r = Math.sqrt(Math.max(0, 1 - cy * cy));
      const sp = sparks.speed * (0.45 + 0.55 * h(3));
      const o = i * 3;
      spPos[o] = at.x; spPos[o + 1] = at.y; spPos[o + 2] = at.z;
      spPrev[o] = at.x; spPrev[o + 1] = at.y; spPrev[o + 2] = at.z;
      spVel[o] = Math.cos(theta) * r * sp;
      spVel[o + 1] = cy * sp;
      spVel[o + 2] = Math.sin(theta) * r * sp;
      spAge[i] = 0;
      spLife[i] = sparks.life * (0.5 + 0.7 * h(4));
    }
  }

  function stepSparks(dt: number): number {
    let verts = 0;
    const decay = Math.exp(-sparks.drag * dt);
    for (let i = 0; i < spCount; i++) {
      if (spAge[i] >= spLife[i]) { continue; }
      const o = i * 3;
      spPrev[o] = spPos[o]; spPrev[o + 1] = spPos[o + 1]; spPrev[o + 2] = spPos[o + 2];
      spVel[o + 1] -= sparks.gravity * dt;
      spVel[o] *= decay; spVel[o + 1] *= decay; spVel[o + 2] *= decay;
      spPos[o] += spVel[o] * dt;
      spPos[o + 1] += spVel[o + 1] * dt;
      spPos[o + 2] += spVel[o + 2] * dt;
      // Bounce off the floor, losing most of the energy. In the indoor footage
      // the sparks skitter along the ground for as long as they fly.
      if (spPos[o + 1] < 0.01) { spPos[o + 1] = 0.01; spVel[o + 1] *= -0.25; }
      spAge[i] += dt;

      const u = spAge[i] / spLife[i];
      // Hold, then fall away fast. An ember does not fade linearly; it is
      // bright and then it is embers.
      const f = u < sparks.hold ? 1 : Math.max(0, 1 - (u - sparks.hold) / (1 - sparks.hold));
      const e = f * f * sparks.brightness;
      // White-hot to orange to red as it cools.
      const cr = e, cg = e * (0.35 + 0.5 * f), cb = e * (0.08 + 0.25 * f * f);
      const v = verts * 6;
      spVerts[v] = spPrev[o]; spVerts[v + 1] = spPrev[o + 1]; spVerts[v + 2] = spPrev[o + 2];
      spVerts[v + 3] = cr * 0.25; spVerts[v + 4] = cg * 0.25; spVerts[v + 5] = cb * 0.25;
      spVerts[v + 6] = spPos[o]; spVerts[v + 7] = spPos[o + 1]; spVerts[v + 8] = spPos[o + 2];
      spVerts[v + 9] = cr; spVerts[v + 10] = cg; spVerts[v + 11] = cb;
      verts += 2;
    }
    return verts;
  }

  // ---- the view pass ------------------------------------------------------
  const mod = d.createShaderModule({ label: "smokeview", code: viewSrc });
  // Surfaced explicitly. A shader that fails to compile makes the pipeline
  // invalid, which invalidates the whole command buffer at submit — including
  // the fluid step sharing it. The symptom is a solver that silently does
  // nothing, which is a long way from the cause.
  {
    const info = await mod.getCompilationInfo();
    const bad = info.messages.filter((m) => m.type === "error");
    for (const m of info.messages) {
      console[m.type === "error" ? "error" : "warn"](
        `[smokeview] ${m.type} ${m.lineNum}:${m.linePos} ${m.message}`);
    }
    if (bad.length) {
      fatal("smokeview failed to compile", bad.map((m) => `${m.lineNum}: ${m.message}`).join("\n"));
      return;
    }
  }
  const viewBuf = d.createBuffer({
    label: "smokeview-params",
    size: VIEW_BYTES,
    usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
  });
  const sampler = d.createSampler({
    magFilter: "linear", minFilter: "linear",
    addressModeU: "clamp-to-edge", addressModeV: "clamp-to-edge",
    addressModeW: "clamp-to-edge",
  });
  const layout = d.createBindGroupLayout({
    label: "smokeview",
    entries: [
      { binding: 0, visibility: GPUShaderStage.FRAGMENT, buffer: { type: "uniform" } },
      {
        binding: 1, visibility: GPUShaderStage.FRAGMENT,
        texture: { sampleType: "float", viewDimension: "3d" },
      },
      { binding: 2, visibility: GPUShaderStage.FRAGMENT, sampler: { type: "filtering" } },
    ],
  });
  const pipeline = d.createRenderPipeline({
    label: "smokeview",
    layout: d.createPipelineLayout({ bindGroupLayouts: [layout] }),
    vertex: { module: mod, entryPoint: "vs" },
    fragment: { module: mod, entryPoint: "fs", targets: [{ format: ctx.format }] },
    primitive: { topology: "triangle-list" },
  });
  // The solver ping-pongs its scalar textures, so the density view can change
  // identity across a reset. Rebuilt whenever that happens rather than cached.
  let bindGroup = makeBindGroup();
  function makeBindGroup(): GPUBindGroup {
    return d.createBindGroup({
      label: "smokeview",
      layout,
      entries: [
        { binding: 0, resource: { buffer: viewBuf } },
        { binding: 1, resource: fluid.densityTexture.createView({ dimension: "3d" }) },
        { binding: 2, resource: sampler },
      ],
    });
  }

  // ---- spark pipeline -----------------------------------------------------
  const spMod = d.createShaderModule({ label: "sparks", code: sparkSrc });
  {
    const info = await spMod.getCompilationInfo();
    const bad = info.messages.filter((m) => m.type === "error");
    for (const m of bad) console.error(`[sparks] ${m.lineNum}: ${m.message}`);
    if (bad.length) { fatal("sparks failed to compile", bad[0].message); return; }
  }
  const spBuf = d.createBuffer({
    label: "spark-verts",
    size: spVerts.byteLength,
    usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
  });
  const spUniform = d.createBuffer({
    label: "spark-view", size: 64,
    usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
  });
  const spLayout = d.createBindGroupLayout({
    label: "sparks",
    entries: [{ binding: 0, visibility: GPUShaderStage.VERTEX, buffer: { type: "uniform" } }],
  });
  const spBind = d.createBindGroup({
    label: "sparks", layout: spLayout,
    entries: [{ binding: 0, resource: { buffer: spUniform } }],
  });
  const spPipeline = d.createRenderPipeline({
    label: "sparks",
    layout: d.createPipelineLayout({ bindGroupLayouts: [spLayout] }),
    vertex: {
      module: spMod, entryPoint: "vs",
      buffers: [{
        arrayStride: 24,
        attributes: [
          { shaderLocation: 0, offset: 0, format: "float32x3" },
          { shaderLocation: 1, offset: 12, format: "float32x3" },
        ],
      }],
    },
    fragment: {
      module: spMod, entryPoint: "fs",
      targets: [{
        format: ctx.format,
        // Additive: an ember is emissive, so nothing it crosses should dim it.
        blend: {
          color: { srcFactor: "one", dstFactor: "one", operation: "add" },
          alpha: { srcFactor: "one", dstFactor: "one", operation: "add" },
        },
      }],
    },
    primitive: { topology: "line-list" },
  });

  const viewData = new ArrayBuffer(VIEW_BYTES);
  const vf = new Float32Array(viewData);

  function resize(): void {
    const dpr = Math.min(devicePixelRatio || 1, 2);
    canvas.width = Math.max(2, Math.floor(canvas.clientWidth * dpr));
    canvas.height = Math.max(2, Math.floor(canvas.clientHeight * dpr));
  }
  resize();
  new ResizeObserver(resize).observe(canvas);

  // ---- camera -------------------------------------------------------------
  // Built with the project's own matrix helpers rather than by hand.
  //
  // The first pass derived inv(proj) analytically and got it wrong in a way
  // that is easy to miss and hard to read: every pixel came out with the same
  // ray direction, so the frame was a single flat colour with no gradient and
  // no box hit. It looked like the volume was empty. Compose the forward
  // matrix from the same lookAt/perspective the game uses and invert it — the
  // one operation here that has already been verified elsewhere.
  function viewProjInverse(): { inv: Float32Array; fwd: Float32Array; eye: number[] } {
    const cp = Math.cos(cam.pitch), sp = Math.sin(cam.pitch);
    const eye = v3(
      Math.sin(cam.yaw) * cp * cam.distance,
      cam.height + sp * cam.distance,
      Math.cos(cam.yaw) * cp * cam.distance,
    );
    const view = mat4LookAt(eye, v3(0, cam.height, 0), v3(0, 1, 0));
    const proj = mat4Perspective(1.05, canvas.width / Math.max(canvas.height, 1), 0.05);
    const fwd = mat4Mul(proj, view);
    return { inv: mat4Invert(fwd), fwd, eye: [eye.x, eye.y, eye.z] };
  }

  // ---- emitters -----------------------------------------------------------
  function fire(): void {
    if (kind === 1) {
      const at = v3(0, burst.height, 0);
      // The light and the smoke are the same event and start on the same
      // frame; a flash that leads or trails its own cloud reads as two things.
      flash.age = 0;

      // ---- the canister ---------------------------------------------------
      const rnd = (k: number): number => {
        const t = Math.sin(burst.seed * 37.31 + k * 91.7) * 43758.5453;
        return t - Math.floor(t);
      };
      // A can lying on the ground: axis mostly horizontal, yaw wherever it
      // came to rest, tilted a little because floors are not billiard tables.
      const yaw = rnd(1) * Math.PI * 2;
      const tilt = (rnd(2) - 0.5) * 2 * burst.tilt;
      const ax = Math.cos(tilt) * Math.sin(yaw);
      const ay = Math.sin(tilt);
      const az = Math.cos(tilt) * Math.cos(yaw);

      // Two opposed end jets, deliberately unequal. In footage one port
      // almost always dominates — the can is against something, or its ends
      // did not open evenly.
      const bias = (rnd(3) - 0.5) * 2 * burst.asymmetry;
      for (const side of [1, -1]) {
        const w = 1 + side * bias;
        if (w <= 0.02) { continue; }
        smoke.spawn({
          pos: v3(at.x + ax * burst.halfLength * side,
                  at.y + ay * burst.halfLength * side,
                  at.z + az * burst.halfLength * side),
          radius: burst.radius,
          vel: v3(ax * burst.ventSpeed * side, ay * burst.ventSpeed * side,
                  az * burst.ventSpeed * side),
          push: 40,
          density: burst.density * w,
          temp: burst.temp * w,
          expand: burst.expand,
          life: burst.life, attack: 0.02,
        });
      }

      // Skirt: the body ports, weaker and perpendicular to the axis. This is
      // what stops the two jets reading as a dumbbell.
      const up = Math.abs(ay) > 0.9 ? v3(1, 0, 0) : v3(0, 1, 0);
      const px = up.y * az - up.z * ay, py = up.z * ax - up.x * az;
      const pz = up.x * ay - up.y * ax;
      const pl = Math.hypot(px, py, pz) || 1;
      const qx = ay * (pz / pl) - az * (py / pl);
      const qy = az * (px / pl) - ax * (pz / pl);
      const qz = ax * (py / pl) - ay * (px / pl);
      for (let i = 0; i < burst.bodyVents; i++) {
        const a2 = (i / Math.max(1, burst.bodyVents)) * Math.PI * 2 + rnd(10 + i) * 1.2;
        const c = Math.cos(a2), sn = Math.sin(a2);
        const dx = (px / pl) * c + qx * sn;
        const dy = (py / pl) * c + qy * sn;
        const dz = (pz / pl) * c + qz * sn;
        const w = burst.bodyFraction * (0.5 + rnd(30 + i));
        smoke.spawn({
          pos: at,
          radius: burst.radius * 0.7,
          vel: v3(dx * burst.ventSpeed * 0.6, dy * burst.ventSpeed * 0.6,
                  dz * burst.ventSpeed * 0.6),
          push: 30,
          density: burst.density * w,
          temp: burst.temp * w,
          expand: burst.expand * 0.5,
          life: burst.life, attack: 0.02,
        });
      }
      smoke.spawn({
        pos: at, radius: burst.wispRadius, vel: v3(0, burst.wispRise, 0), push: 5,
        density: burst.wispDensity, temp: burst.wispTemp,
        life: burst.wispLife, attack: 0.3,
      });
      emitSparks(at);
      // Smoke hung off the sparks themselves. `follow` reads the spark's live
      // position every frame, so each trail inherits that spark's direction,
      // speed, arc and floor bounce — none of which is symmetric.
      const n = Math.min(trail.count, FLUID_MAX_SOURCES - 2, spCount);
      for (let i = 0; i < n; i++) {
        const o = i * 3;
        smoke.spawn({
          pos: v3(spPos[o], spPos[o + 1], spPos[o + 2]),
          follow: () => v3(spPos[o], spPos[o + 1], spPos[o + 2]),
          radius: trail.radius,
          density: trail.density,
          temp: trail.temp,
          life: trail.life,
          attack: 0.01,
        });
      }
      return;
    }
    if (kind === 2) {
      const ax = Math.sin(vent.yaw), az = Math.cos(vent.yaw);
      smoke.spawn({
        pos: v3(0, vent.height, 0), radius: vent.radius,
        vel: v3(ax * vent.speed, 0.3, az * vent.speed), push: vent.push,
        density: vent.density, temp: vent.temp, expand: vent.expand,
        life: vent.seconds, attack: 0.12,
      });
      return;
    }
    // Plume: a small hot source near the floor, which is the case that shows
    // buoyancy, vorticity and dissipation all at once.
    smoke.spawn({
      pos: v3(0, 0.18, 0), radius: plume.radius,
      vel: v3(0, plume.speed, 0), push: plume.push,
      density: plume.density, temp: plume.temp, expand: plume.expand,
      life: plume.seconds, attack: 0.12,
    });
  }

  function reset(): void {
    spCount = 0;
    smoke.reset(true);
    fluid.reset();
    bindGroup = makeBindGroup();
  }

  // ---- frame --------------------------------------------------------------
  let paused = false;
  let simTime = 0;
  const empty = new Float32Array(FLUID_MAX_SOURCES * FLUID_SOURCE_STRIDE);

  function frameBody(dt: number): void {
    if (cam.orbit) cam.yaw += dt * cam.speed;
    flash.age += dt;
    smoke.update(dt, true);
    Object.assign(fluid.tune, solver);

    const enc = d.createCommandEncoder({ label: "dyn" });
    fluid.step(enc, dt, smoke.count > 0 ? smoke.packed : empty, smoke.count, () => undefined);

    const { inv, fwd, eye } = viewProjInverse();
    const spVertCount = stepSparks(dt);
    vf.set(inv, 0);
    vf[16] = eye[0]; vf[17] = eye[1]; vf[18] = eye[2]; vf[19] = look.steps;
    vf[20] = ORIGIN[0]; vf[21] = ORIGIN[1]; vf[22] = ORIGIN[2];
    vf[23] = look.shadowSteps;
    vf[24] = ORIGIN[0] + DIMS[0] * CELL;
    vf[25] = ORIGIN[1] + DIMS[1] * CELL;
    vf[26] = ORIGIN[2] + DIMS[2] * CELL;
    vf[27] = look.absorption;
    // Key light, from above and to one side: a plume needs a lit face and a
    // shadowed one or it reads as a flat card whatever the solver is doing.
    const ll = Math.hypot(0.4, 0.85, 0.3);
    vf[28] = 0.4 / ll; vf[29] = 0.85 / ll; vf[30] = 0.3 / ll; vf[31] = look.scatter;
    vf[32] = 1.0; vf[33] = 0.97; vf[34] = 0.92; vf[35] = look.ambient;
    vf[36] = 0.09; vf[37] = 0.11; vf[38] = 0.14; vf[39] = look.exposure;
    vf[40] = 0.02; vf[41] = 0.02; vf[42] = 0.03; vf[43] = look.densityScale;
    // Flash. Squared falloff in time as well as distance: a linear fade reads
    // as a lamp being turned down, where a bang is almost all over in its
    // first third.
    const u = flash.age / Math.max(flash.duration, 1e-3);
    const env = u >= 1 ? 0 : (1 - u) * (1 - u);
    vf[44] = 0; vf[45] = burst.height; vf[46] = 0;
    vf[47] = flash.power * env;
    vf[48] = 1.0; vf[49] = 0.98; vf[50] = 0.95; vf[51] = flash.shadowSteps;
    d.queue.writeBuffer(viewBuf, 0, viewData);

    const pass = enc.beginRenderPass({
      label: "smokeview",
      colorAttachments: [{
        view: ctx.context.getCurrentTexture().createView(),
        loadOp: "clear", storeOp: "store",
        clearValue: { r: 0, g: 0, b: 0, a: 1 },
      }],
    });
    pass.setPipeline(pipeline);
    pass.setBindGroup(0, bindGroup);
    pass.draw(3);

    // Sparks last, into the same pass, so they sit over the volume. They are
    // not depth-tested against it — an ember bright enough to see through
    // smoke is the behaviour the footage shows, not a bug being papered over.
    if (spVertCount > 0) {
      d.queue.writeBuffer(spBuf, 0, spVerts, 0, spVertCount * 6);
      d.queue.writeBuffer(spUniform, 0, fwd as Float32Array<ArrayBuffer>);
      pass.setPipeline(spPipeline);
      pass.setBindGroup(0, spBind);
      pass.setVertexBuffer(0, spBuf);
      pass.draw(spVertCount);
    }
    pass.end();

    d.queue.submit([enc.finish()]);
    simTime += dt;
  }

  let prev = performance.now();
  function frame(now: number): void {
    const dt = Math.min((now - prev) / 1000, 0.05);
    prev = now;
    if (!paused) frameBody(dt);
    requestAnimationFrame(frame);
  }

  // ---- panel --------------------------------------------------------------
  /**
   * Every live value, in one blob.
   *
   * Tuning ends with somebody moving these numbers into game/smoke.ts, and the
   * panel now has upwards of fifty sliders across seven groups. Reading them
   * off the screen one at a time is how a good set gets landed slightly wrong;
   * this is the same act done by copy.
   */
  function settingsBlob(): string {
    const round = (o: Record<string, number>): Record<string, number> => {
      const out: Record<string, number> = {};
      for (const [k, v] of Object.entries(o)) {
        out[k] = typeof v === "number" ? +v.toFixed(4) : v;
      }
      return out;
    };
    return JSON.stringify({
      kind: ["plume", "burst", "vent"][kind],
      lattice: { dims: DIMS, cell: CELL },
      burst: round(burst as unknown as Record<string, number>),
      trail: round(trail as unknown as Record<string, number>),
      // `age` is live state, not a setting; it would be noise in a paste.
      sparks: round({ ...sparks } as unknown as Record<string, number>),
      flash: round({ power: flash.power, duration: flash.duration,
                     shadowSteps: flash.shadowSteps }),
      plume: round(plume as unknown as Record<string, number>),
      vent: round(vent as unknown as Record<string, number>),
      solver: round(solver as unknown as Record<string, number>),
      look: round(look as unknown as Record<string, number>),
      cam: round({ yaw: cam.yaw, pitch: cam.pitch, distance: cam.distance,
                   height: cam.height }),
    }, null, 2);
  }

  function copySettings(): void {
    const text = settingsBlob();
    // Always log as well: clipboard access needs a user gesture and a secure
    // context, and a button that silently fails is worse than no button.
    console.log(text);
    void navigator.clipboard?.writeText(text).then(
      () => console.log("[dynamics] settings copied to clipboard"),
      (e) => console.warn("[dynamics] clipboard refused, use the log above:", e),
    );
  }

  const groups: GroupSpec[] = [
    {
      title: "emitter  (space fires · R resets)",
      items: [
        { kind: "button", label: "copy all settings (C)", onClick: copySettings },
        {
          kind: "select", label: "kind", options: ["plume", "burst", "vent"],
          get: () => kind,
          set: (v) => { kind = v; queueMicrotask(() => panel.refresh()); },
        },
      ],
    },
    {
      title: "plume",
      show: () => kind === 0,
      items: [
        sl("radius", 0.05, 1, 0.01, () => plume.radius, (v) => (plume.radius = v)),
        sl("density /s", 0, 600, 5, () => plume.density, (v) => (plume.density = v)),
        sl("temp /s", 0, 40, 0.5, () => plume.temp, (v) => (plume.temp = v)),
        sl("speed m/s", 0, 20, 0.1, () => plume.speed, (v) => (plume.speed = v)),
        sl("push 1/s", 0, 120, 1, () => plume.push, (v) => (plume.push = v)),
        sl("expand 1/s", 0, 300, 5, () => plume.expand, (v) => (plume.expand = v)),
        sl("seconds", 0.1, 60, 0.1, () => plume.seconds, (v) => (plume.seconds = v)),
      ],
    },
    {
      title: "burst — flash",
      show: () => kind === 1,
      items: [
        sl("power", 0, 200, 1, () => flash.power, (v) => (flash.power = v)),
        sl("duration s", 0.02, 1, 0.01, () => flash.duration, (v) => (flash.duration = v)),
        sl("shadow steps", 0, 12, 1,
          () => flash.shadowSteps, (v) => (flash.shadowSteps = v)),
      ],
    },
    {
      title: "burst — core",
      show: () => kind === 1,
      items: [
        sl("radius", 0.05, 2, 0.01, () => burst.radius, (v) => (burst.radius = v)),
        sl("density /s", 0, 1200, 10, () => burst.density, (v) => (burst.density = v)),
        sl("temp /s", 0, 80, 1, () => burst.temp, (v) => (burst.temp = v)),
        sl("expand 1/s", 0, 300, 5, () => burst.expand, (v) => (burst.expand = v)),
        sl("life s", 0.02, 2, 0.01, () => burst.life, (v) => (burst.life = v)),
        sl("height", 0.02, 2.5, 0.02, () => burst.height, (v) => (burst.height = v)),
        sl("seed (reroll)", 0, 64, 1, () => burst.seed, (v) => (burst.seed = v)),
        sl("can half-length", 0.01, 0.4, 0.01,
          () => burst.halfLength, (v) => (burst.halfLength = v)),
        sl("vent speed m/s", 0, 25, 0.5,
          () => burst.ventSpeed, (v) => (burst.ventSpeed = v)),
        sl("end asymmetry", 0, 1, 0.02,
          () => burst.asymmetry, (v) => (burst.asymmetry = v)),
        sl("body vents", 0, 10, 1, () => burst.bodyVents, (v) => (burst.bodyVents = v)),
        sl("body strength", 0, 1.5, 0.05,
          () => burst.bodyFraction, (v) => (burst.bodyFraction = v)),
        sl("axis tilt", 0, 1.4, 0.02, () => burst.tilt, (v) => (burst.tilt = v)),
      ],
    },
    {
      title: "burst — spark trails",
      show: () => kind === 1,
      items: [
        sl("smoking sparks", 0, 90, 1, () => trail.count, (v) => (trail.count = v)),
        sl("radius", 0.02, 0.4, 0.005, () => trail.radius, (v) => (trail.radius = v)),
        sl("density /s", 0, 400, 5, () => trail.density, (v) => (trail.density = v)),
        sl("temp /s", 0, 40, 0.5, () => trail.temp, (v) => (trail.temp = v)),
        sl("life s", 0.05, 3, 0.05, () => trail.life, (v) => (trail.life = v)),
      ],
    },
    {
      title: "burst — sparks",
      show: () => kind === 1,
      items: [
        sl("count", 0, 512, 8, () => sparks.count, (v) => (sparks.count = v)),
        sl("speed m/s", 1, 40, 0.5, () => sparks.speed, (v) => (sparks.speed = v)),
        sl("spread", 0, 1, 0.02, () => sparks.spread, (v) => (sparks.spread = v)),
        sl("gravity", 0, 40, 0.5, () => sparks.gravity, (v) => (sparks.gravity = v)),
        sl("drag 1/s", 0, 8, 0.05, () => sparks.drag, (v) => (sparks.drag = v)),
        sl("life s", 0.1, 4, 0.05, () => sparks.life, (v) => (sparks.life = v)),
        sl("brightness", 0, 8, 0.1,
          () => sparks.brightness, (v) => (sparks.brightness = v)),
        sl("hold", 0, 0.8, 0.02, () => sparks.hold, (v) => (sparks.hold = v)),
        sl("lift", 0, 1, 0.02, () => sparks.lift, (v) => (sparks.lift = v)),
      ],
    },
    {
      title: "burst — wisp",
      show: () => kind === 1,
      items: [
        sl("radius", 0.05, 1.5, 0.05,
          () => burst.wispRadius, (v) => (burst.wispRadius = v)),
        sl("density /s", 0, 200, 2,
          () => burst.wispDensity, (v) => (burst.wispDensity = v)),
        sl("temp /s", 0, 40, 0.5, () => burst.wispTemp, (v) => (burst.wispTemp = v)),
        sl("rise m/s", 0, 6, 0.1, () => burst.wispRise, (v) => (burst.wispRise = v)),
        sl("life s", 0, 20, 0.5, () => burst.wispLife, (v) => (burst.wispLife = v)),
      ],
    },
    {
      title: "vent",
      show: () => kind === 2,
      items: [
        sl("radius", 0.05, 1.5, 0.01, () => vent.radius, (v) => (vent.radius = v)),
        sl("density /s", 0, 600, 5, () => vent.density, (v) => (vent.density = v)),
        sl("temp /s", 0, 20, 0.2, () => vent.temp, (v) => (vent.temp = v)),
        sl("speed m/s", 0, 25, 0.5, () => vent.speed, (v) => (vent.speed = v)),
        sl("push 1/s", 0, 120, 1, () => vent.push, (v) => (vent.push = v)),
        sl("yaw", -3.15, 3.15, 0.05, () => vent.yaw, (v) => (vent.yaw = v)),
        sl("seconds", 0.1, 60, 0.1, () => vent.seconds, (v) => (vent.seconds = v)),
      ],
    },
    {
      title: "solver",
      items: [
        sl("vorticity", 0, 30, 0.25, () => solver.vorticity, (v) => (solver.vorticity = v)),
        sl("buoyancy", 0, 30, 0.25, () => solver.buoyancy, (v) => (solver.buoyancy = v)),
        sl("weight", 0, 3, 0.01, () => solver.weight, (v) => (solver.weight = v)),
        sl("dissipation", 0, 2, 0.005,
          () => solver.dissipation, (v) => (solver.dissipation = v)),
        sl("cooling", 0, 12, 0.1, () => solver.cooling, (v) => (solver.cooling = v)),
        sl("jacobi", 4, 80, 2, () => solver.jacobi, (v) => (solver.jacobi = v)),
      ],
    },
    {
      title: "view",
      items: [
        sl("march steps", 16, 256, 8, () => look.steps, (v) => (look.steps = v)),
        sl("shadow steps", 0, 16, 1, () => look.shadowSteps, (v) => (look.shadowSteps = v)),
        sl("absorption", 0.002, 0.4, 0.002, () => look.absorption, (v) => (look.absorption = v)),
        sl("density scale", 0.1, 6, 0.05,
          () => look.densityScale, (v) => (look.densityScale = v)),
        sl("scatter", 0, 3, 0.05, () => look.scatter, (v) => (look.scatter = v)),
        sl("ambient", 0, 0.5, 0.005, () => look.ambient, (v) => (look.ambient = v)),
        sl("exposure", 0.1, 4, 0.05, () => look.exposure, (v) => (look.exposure = v)),
        tg("orbit", () => cam.orbit, (v) => (cam.orbit = v)),
        sl("orbit speed", 0, 1.5, 0.05, () => cam.speed, (v) => (cam.speed = v)),
        sl("distance", 1, 10, 0.1, () => cam.distance, (v) => (cam.distance = v)),
        sl("pitch", -0.4, 1.4, 0.02, () => cam.pitch, (v) => (cam.pitch = v)),
      ],
    },
  ];
  const panel = new TweakPanel(groups);
  // Apply the show predicates once, so the page opens with only the selected
  // emitter's controls rather than all three.
  panel.refresh();

  const help = document.getElementById("help");
  if (help) {
    help.textContent = "space fire · R reset · O orbit · ` panel\n"
      + `${DIMS[0]}x${DIMS[1]}x${DIMS[2]} @ ${CELL * 100} cm · C copies settings`;
  }

  addEventListener("keydown", (e) => {
    if (e.code === "Backquote") { panel.toggleVisible(); e.preventDefault(); return; }
    if (e.code === "Space") { fire(); e.preventDefault(); }
    if (e.code === "KeyR") reset();
    if (e.code === "KeyO") cam.orbit = !cam.orbit;
    if (e.code === "KeyC") copySettings();
  });

  /** Grading surface; see tools/headless/scenarios/smoke-strip.js. */
  Object.assign(window, {
    __dyn: {
      fluid, smoke,
      params: { plume, burst, trail, sparks, vent, flash, solver, look, cam },
      settings: settingsBlob,
      kind: () => kind,
      setKind: (k: number) => { kind = k; },
      fire, reset,
      pause: (on: boolean) => { paused = on; },
      /** Steps exactly `n` frames of `dtMs` each, however long each takes. */
      step: async (n: number, dtMs = 50): Promise<string> => {
        for (let i = 0; i < n; i++) frameBody(dtMs / 1000);
        await d.queue.onSubmittedWorkDone();
        return `stepped ${n}`;
      },
      resize: (w: number, h: number) => {
        canvas.style.width = `${w}px`;
        canvas.style.height = `${h}px`;
        resize();
      },
      simTime: () => simTime,
      /** Spark state, for checking the ballistic arc is doing what it claims. */
      sparkStats: () => {
        let alive = 0, maxY = 0, meanVy = 0, maxR = 0;
        for (let i = 0; i < spCount; i++) {
          if (spAge[i] >= spLife[i]) { continue; }
          const o = i * 3;
          alive++;
          maxY = Math.max(maxY, spPos[o + 1]);
          meanVy += spVel[o + 1];
          maxR = Math.max(maxR, Math.hypot(spPos[o], spPos[o + 2]));
        }
        return { alive, maxY, meanVy: alive ? meanVy / alive : 0, maxR };
      },
    },
    __renderer: { renderWidth: () => canvas.width },
    __renderStill: async (n = 30, dtMs = 50): Promise<string> => {
      for (let i = 0; i < n; i++) frameBody(dtMs / 1000);
      await d.queue.onSubmittedWorkDone();
      return `rendered ${n}`;
    },
  });

  requestAnimationFrame(frame);
}

void main();
