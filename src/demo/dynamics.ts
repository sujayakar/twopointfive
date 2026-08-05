import { mat4Invert, mat4LookAt, mat4Mul, mat4Perspective, v3 } from "../core/math";
import { GPUContext, initGPU } from "../engine/gpu";
import { FLUID_MAX_SOURCES, FLUID_SOURCE_STRIDE, FluidSim } from "../engine/fluid";
import { Smoke } from "../game/smoke";
import {
  EmitterKind, FlashLight, SparkField, burst, fire as fireEffect,
  flash, muzzle, plume, sparks, trail, vent,
} from "../game/effects";
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
  /**
   * Fraction of the backing store actually rendered.
   *
   * This page is pure raymarch and its cost is per PIXEL: `steps` samples a
   * ray, and every sample whose density clears the threshold pays
   * `shadowSteps` more toward the key light and `flash.shadowSteps` more
   * toward the bang. At devicePixelRatio 2 a 1280x720 window is a 2560x1440
   * backing store, so 3.7 M pixels x 96 steps is the whole frame budget and
   * nothing else on the page is close.
   *
   * Which also means the cost depends on what is ON SCREEN, not just on the
   * settings: the march breaks once transmittance drops below 0.003 and skips
   * the shadow taps in empty cells, so a dense compact cloud is CHEAPER than
   * the thin haze that fills the box a few seconds after a burst. The worst
   * frame is the one that looks like the least.
   */
  renderScale: 1.0,
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
 * Knobs that are not `FluidSim.tune` fields, kept out of `solver` so the
 * blanket `Object.assign(fluid.tune, solver)` stays a clean copy.
 *
 * `openFaces` is here rather than hardcoded because the lateral boundary is
 * the one part of the solver this page exercises that the game's coarse
 * lattice does not, and "does the bug survive closing the walls" is the single
 * measurement that localises anything wrong with it.
 */
const dbg = { openFaces: 0xf };

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
let kind: EmitterKind = 1; // 0 plume, 1 burst, 2 vent, 3 muzzle

/** The light currently in the room, whatever lit it. Replaced by fire(). */
let flashLive: FlashLight = {
  x: 0, y: 0, z: 0, power: 0, duration: 0.14, r: 1, g: 1, b: 1, age: 1e9,
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
  fluid.openFaces = dbg.openFaces;

  const smoke = new Smoke();

  // ---- sparks -------------------------------------------------------------
  // The simulation lives in game/effects.ts so both this page and the traced
  // one advance identical sparks; what stays here is only how they are DRAWN.
  const sparkField = new SparkField();

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
    size: sparkField.verts.byteLength,
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
    const dpr = Math.min(devicePixelRatio || 1, 2) * look.renderScale;
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

  /**
   * Fires the selected emitter and adopts whatever light it throws.
   *
   * An emitter with no light of its own leaves the room dark rather than
   * inheriting the last one's — `age` past `duration` is the off state, and
   * carrying a stale flash across a kind change was how the vent briefly came
   * with a flashbang attached.
   */
  function fire(): void {
    const lit = fireEffect(kind, smoke, sparkField);
    flashLive = lit ?? { ...flashLive, power: 0, age: 1e9 };
  }

  function reset(): void {
    sparkField.clear();
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
    flashLive.age += dt;
    smoke.update(dt, true);
    Object.assign(fluid.tune, solver);
    // Lateral outflow: the four side walls stop being walls.
    //
    // A closed box conserves everything, so a burst that reaches the sides
    // piles up against them and the pressure solve pushes it back in — energy
    // the room never had, arriving from a boundary that is an arbitrary cut
    // through open air. Bits 0-3 are -x/+x/-z/+z. Floor and ceiling stay solid
    // on purpose: those are real surfaces, the burst sits on one and the
    // column should still spread against the other.
    fluid.openFaces = dbg.openFaces;

    const enc = d.createCommandEncoder({ label: "dyn" });
    fluid.step(enc, dt, smoke.count > 0 ? smoke.packed : empty, smoke.count, () => undefined);

    const { inv, fwd, eye } = viewProjInverse();
    const spVertCount = sparkField.step(dt);
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
    const u = flashLive.age / Math.max(flashLive.duration, 1e-3);
    const env = u >= 1 ? 0 : (1 - u) * (1 - u);
    vf[44] = flashLive.x; vf[45] = flashLive.y; vf[46] = flashLive.z;
    vf[47] = flashLive.power * env;
    vf[48] = flashLive.r; vf[49] = flashLive.g; vf[50] = flashLive.b;
    vf[51] = flash.shadowSteps;
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
      d.queue.writeBuffer(spBuf, 0, sparkField.verts, 0, spVertCount * 6);
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

  // ---- frame-time readout -------------------------------------------------
  //
  // A number, because "it feels faster when I do X" cannot be acted on and
  // this page has several plausible values of X: resolution, march steps,
  // shadow taps, and how much of the box currently has smoke in it. The last
  // one is not obvious and is often the answer — the march breaks early
  // against a dense cloud and runs its full length through a thin one, so the
  // frame gets slower as the smoke *fades*.
  //
  // Median over the window, not mean: one 200 ms hitch from a shader compile
  // or a GC would otherwise dominate the average and hide the steady state.
  const FRAME_WINDOW = 90;
  const frameMs: number[] = [];
  let hudDue = 0;
  const hudEl = document.getElementById("hud");

  function updateHud(now: number): void {
    if (!hudEl || now < hudDue) return;
    hudDue = now + 250;
    if (frameMs.length < 8) return;
    const s = [...frameMs].sort((a, b) => a - b);
    const med = s[s.length >> 1];
    const p90 = s[Math.min(s.length - 1, Math.floor(s.length * 0.9))];
    const mp = (canvas.width * canvas.height) / 1e6;
    hudEl.textContent =
      `${med.toFixed(1)} ms  ${(1000 / Math.max(med, 0.01)).toFixed(0)} fps`
      + `   p90 ${p90.toFixed(1)} ms\n`
      + `${canvas.width}x${canvas.height}  ${mp.toFixed(1)} Mpx`
      + `  x${look.renderScale.toFixed(2)}  ${look.steps}+${look.shadowSteps} steps`;
  }

  let prev = performance.now();
  function frame(now: number): void {
    const raw = now - prev;
    // The sim's dt is capped; the readout's is not. Capping it here would
    // report 50 ms as the ceiling and a page running at 8 fps would read as
    // if it were running at 20.
    const dt = Math.min(raw / 1000, 0.05);
    prev = now;
    if (!paused) {
      frameBody(dt);
      frameMs.push(raw);
      if (frameMs.length > FRAME_WINDOW) frameMs.shift();
    }
    updateHud(now);
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
      kind: ["plume", "burst", "vent", "muzzle"][kind],
      lattice: { dims: DIMS, cell: CELL },
      burst: round(burst as unknown as Record<string, number>),
      trail: round(trail as unknown as Record<string, number>),
      // `age` is live state, not a setting; it would be noise in a paste.
      sparks: round({ ...sparks } as unknown as Record<string, number>),
      flash: round({ power: flash.power, duration: flash.duration,
                     shadowSteps: flash.shadowSteps }),
      plume: round(plume as unknown as Record<string, number>),
      vent: round(vent as unknown as Record<string, number>),
      muzzle: round(muzzle as unknown as Record<string, number>),
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
          kind: "select", label: "kind",
          options: ["plume", "burst", "vent", "muzzle"],
          get: () => kind,
          set: (v) => { kind = v as EmitterKind; queueMicrotask(() => panel.refresh()); },
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
        // Directly under `density`, because it is the same axis: this is the
        // multiplier that gets past the slider's own ceiling.
        sl("stack (presses)", 1, 8, 1, () => plume.stack, (v) => (plume.stack = v)),
      ],
    },
    {
      title: "muzzle — flash",
      show: () => kind === 3,
      items: [
        sl("power", 0, 200, 1,
          () => muzzle.flashPower, (v) => (muzzle.flashPower = v)),
        sl("duration s", 0.01, 0.5, 0.005,
          () => muzzle.flashDuration, (v) => (muzzle.flashDuration = v)),
        sl("shadow steps", 0, 12, 1,
          () => flash.shadowSteps, (v) => (flash.shadowSteps = v)),
      ],
    },
    {
      title: "muzzle — gas",
      show: () => kind === 3,
      items: [
        sl("height", 0.2, 2.5, 0.05, () => muzzle.height, (v) => (muzzle.height = v)),
        sl("yaw", 0, 6.28, 0.02, () => muzzle.yaw, (v) => (muzzle.yaw = v)),
        sl("standoff", 0, 1, 0.01,
          () => muzzle.standoff, (v) => (muzzle.standoff = v)),
        sl("radius", 0.03, 0.6, 0.01, () => muzzle.radius, (v) => (muzzle.radius = v)),
        sl("density /s", 0, 1200, 10,
          () => muzzle.density, (v) => (muzzle.density = v)),
        sl("temp /s", 0, 80, 1, () => muzzle.temp, (v) => (muzzle.temp = v)),
        sl("speed m/s", 0, 40, 0.5, () => muzzle.speed, (v) => (muzzle.speed = v)),
        sl("push 1/s", 0, 200, 1, () => muzzle.push, (v) => (muzzle.push = v)),
        sl("expand 1/s", 0, 120, 1, () => muzzle.expand, (v) => (muzzle.expand = v)),
        sl("life s", 0.01, 0.4, 0.005, () => muzzle.life, (v) => (muzzle.life = v)),
        sl("ports", 0, 4, 1, () => muzzle.ports, (v) => (muzzle.ports = v)),
        sl("port fraction", 0, 1, 0.05,
          () => muzzle.portFraction, (v) => (muzzle.portFraction = v)),
        sl("port angle", 0, 1.6, 0.05,
          () => muzzle.portAngle, (v) => (muzzle.portAngle = v)),
      ],
    },
    {
      title: "muzzle — wisp",
      show: () => kind === 3,
      items: [
        sl("radius", 0.03, 0.8, 0.01,
          () => muzzle.wispRadius, (v) => (muzzle.wispRadius = v)),
        sl("density /s", 0, 300, 2,
          () => muzzle.wispDensity, (v) => (muzzle.wispDensity = v)),
        sl("temp /s", 0, 40, 0.5,
          () => muzzle.wispTemp, (v) => (muzzle.wispTemp = v)),
        sl("rise m/s", 0, 4, 0.05,
          () => muzzle.wispRise, (v) => (muzzle.wispRise = v)),
        sl("life s", 0.1, 12, 0.1,
          () => muzzle.wispLife, (v) => (muzzle.wispLife = v)),
      ],
    },
    {
      title: "muzzle — powder",
      show: () => kind === 3,
      items: [
        sl("count", 0, 200, 2,
          () => muzzle.sparkCount, (v) => (muzzle.sparkCount = v)),
        sl("speed m/s", 0, 30, 0.5,
          () => muzzle.sparkSpeed, (v) => (muzzle.sparkSpeed = v)),
        sl("cone rad", 0.02, 1.4, 0.02,
          () => muzzle.sparkCone, (v) => (muzzle.sparkCone = v)),
        sl("life s", 0.05, 1.5, 0.01,
          () => muzzle.sparkLife, (v) => (muzzle.sparkLife = v)),
        sl("seed (reroll)", 0, 64, 1, () => muzzle.seed, (v) => (muzzle.seed = v)),
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
        sl("port jitter", 0, 1.5, 0.05, () => burst.jitter, (v) => (burst.jitter = v)),
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
        // First, because it is the biggest lever on frame time by a wide
        // margin: cost is linear in pixels and 0.5 is a quarter of them.
        sl("render scale", 0.25, 1, 0.05,
          () => look.renderScale, (v) => { look.renderScale = v; resize(); }),
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
      params: {
        plume, burst, trail, sparks, vent, muzzle, flash, solver, look, cam, dbg,
      },
      settings: settingsBlob,
      kind: () => kind,
      setKind: (k: number) => { kind = k as EmitterKind; },
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
      sparkStats: () => sparkField.stats(),
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
