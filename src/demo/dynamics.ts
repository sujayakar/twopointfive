import { mat4Invert, mat4LookAt, mat4Mul, mat4Perspective, v3 } from "../core/math";
import { GPUContext, initGPU } from "../engine/gpu";
import { FLUID_MAX_SOURCES, FLUID_SOURCE_STRIDE, FluidSim } from "../engine/fluid";
import { Smoke } from "../game/smoke";
import { ControlSpec, GroupSpec, TweakPanel } from "../ui/panel";
import viewSrc from "../shaders/smokeview.wgsl?raw";

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

const VIEW_BYTES = 176;

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

/** The emitter under test. Deliberately one source, not a preset. */
const jet = {
  kind: 0,          // 0 plume, 1 burst, 2 directed vent
  radius: 0.22,
  density: 160,
  temp: 6,
  speed: 2.5,
  push: 12,
  expand: 0,
  seconds: 30,
  yaw: 0,
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
  function viewProjInverse(): { inv: Float32Array; eye: number[] } {
    const cp = Math.cos(cam.pitch), sp = Math.sin(cam.pitch);
    const eye = v3(
      Math.sin(cam.yaw) * cp * cam.distance,
      cam.height + sp * cam.distance,
      Math.cos(cam.yaw) * cp * cam.distance,
    );
    const view = mat4LookAt(eye, v3(0, cam.height, 0), v3(0, 1, 0));
    const proj = mat4Perspective(1.05, canvas.width / Math.max(canvas.height, 1), 0.05);
    const inv = mat4Invert(mat4Mul(proj, view));
    return { inv, eye: [eye.x, eye.y, eye.z] };
  }

  // ---- emitters -----------------------------------------------------------
  function fire(): void {
    const ax = Math.sin(jet.yaw), az = Math.cos(jet.yaw);
    if (jet.kind === 1) {
      smoke.spawn({
        pos: v3(0, 1.0, 0), radius: jet.radius, density: jet.density,
        temp: jet.temp, expand: jet.expand || 70, life: 0.25, attack: 0.02,
      });
      return;
    }
    if (jet.kind === 2) {
      smoke.spawn({
        pos: v3(0, 0.25, 0), radius: jet.radius,
        vel: v3(ax * jet.speed, 0.3, az * jet.speed), push: jet.push,
        density: jet.density, temp: jet.temp, expand: jet.expand,
        life: jet.seconds, attack: 0.12,
      });
      return;
    }
    // Plume: a small hot source near the floor, which is the case that shows
    // buoyancy, vorticity and dissipation all at once.
    smoke.spawn({
      pos: v3(0, 0.18, 0), radius: jet.radius,
      vel: v3(0, jet.speed, 0), push: jet.push,
      density: jet.density, temp: jet.temp, expand: jet.expand,
      life: jet.seconds, attack: 0.12,
    });
  }

  function reset(): void {
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
    smoke.update(dt, true);
    Object.assign(fluid.tune, solver);

    const enc = d.createCommandEncoder({ label: "dyn" });
    fluid.step(enc, dt, smoke.count > 0 ? smoke.packed : empty, smoke.count, () => undefined);

    const { inv, eye } = viewProjInverse();
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
  const groups: GroupSpec[] = [
    {
      title: "emitter  (space to fire · R reset)",
      items: [
        {
          kind: "select", label: "kind", options: ["plume", "burst", "vent"],
          get: () => jet.kind, set: (v) => (jet.kind = v),
        },
        sl("radius", 0.05, 1, 0.01, () => jet.radius, (v) => (jet.radius = v)),
        sl("density /s", 0, 600, 5, () => jet.density, (v) => (jet.density = v)),
        sl("temp /s", 0, 40, 0.5, () => jet.temp, (v) => (jet.temp = v)),
        sl("speed m/s", 0, 20, 0.1, () => jet.speed, (v) => (jet.speed = v)),
        sl("push 1/s", 0, 120, 1, () => jet.push, (v) => (jet.push = v)),
        sl("expand 1/s", 0, 300, 5, () => jet.expand, (v) => (jet.expand = v)),
        sl("seconds", 0.1, 60, 0.1, () => jet.seconds, (v) => (jet.seconds = v)),
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

  const help = document.getElementById("help");
  if (help) {
    help.textContent = "space fire · R reset · O orbit · ` panel\n"
      + `${DIMS[0]}x${DIMS[1]}x${DIMS[2]} @ ${CELL * 100} cm`;
  }

  addEventListener("keydown", (e) => {
    if (e.code === "Backquote") { panel.toggleVisible(); e.preventDefault(); return; }
    if (e.code === "Space") { fire(); e.preventDefault(); }
    if (e.code === "KeyR") reset();
    if (e.code === "KeyO") cam.orbit = !cam.orbit;
  });

  /** Grading surface; see tools/headless/scenarios/smoke-strip.js. */
  Object.assign(window, {
    __dyn: {
      fluid, smoke,
      params: { jet, solver, look, cam },
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
