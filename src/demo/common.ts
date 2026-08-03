import { Vec3, v3 } from "../core/math";
import { GPUInitError, initGPU } from "../engine/gpu";
import {
  DEFAULT_DENOISE, DEFAULT_SETTINGS, INDIRECT_MODES, RenderSettings, Renderer,
  SmokeLattice,
} from "../engine/renderer";
import { Camera } from "../game/camera";
import { Canisters } from "../game/canister";
import { Flashes } from "../game/flashes";
import { Smoke } from "../game/smoke";
import { LevelInfo } from "../scene/level";
import { SceneBuilder, TORCH_TINT } from "../scene/scene";
import { bakeOccupancy, bakeOccupancyGather } from "../scene/occupancy";
import { buildBVH } from "../scene/bvh";
import { ControlSpec, GroupSpec, TweakPanel } from "../ui/panel";

// ---------------------------------------------------------------------------
// Shared bootstrap for the /demo pages.
//
// Deliberately a transcription of main()'s render path rather than an
// abstraction over it. main() owns the frame loop that all 33 headless
// scenarios drive and that compareToReference uses as its measurement harness;
// parameterising it so a demo could share it would fork the one function whose
// behaviour the project's benchmark numbers are defined against. Copying ~100
// lines is the cheaper mistake.
//
// What a demo gets: GPU, a small scene, the renderer, an orbit camera, a torch
// on the cursor, and the panel groups that matter for looking at one subsystem.
// What it does NOT get: player, guards, detection, equipment, objectives, the
// HUD, or the brightness gate. None of those are load-bearing for the renderer
// (every game->render call is either defaulted or gated on a count of zero).
// ---------------------------------------------------------------------------

/** A torch with the same defaults as the player's, since it is the same light. */
export interface DemoTorch {
  on: boolean;
  intensity: number;
  radius: number;
  innerDeg: number;
  outerDeg: number;
  colorR: number;
  colorG: number;
  colorB: number;
  /** Where it emits from and what it points at — the demo drives these. */
  pos: Vec3;
  dir: Vec3;
}

export interface DemoDeps {
  canvas: HTMLCanvasElement;
  renderer: Renderer;
  settings: RenderSettings;
  camera: Camera;
  torch: DemoTorch;
  smoke: Smoke;
  canisters: Canisters;
  /** Transient lights (muzzle flashes, flashbangs) — see game/flashes.ts. */
  flashes: Flashes;
  scene: SceneBuilder;
  level: LevelInfo;
  /** Ground point under the cursor, refreshed every frame. */
  cursor: Vec3;
  /** What the camera looks at; WASD walks it around the floor. */
  focus: Vec3;
  /** How fast `focus` is currently moving, m/s — the stand-in for a body. */
  walkVel: Vec3;
  /**
   * Stops the frame loop re-aiming the torch from the cursor.
   *
   * Scripted captures set torch.pos/dir explicitly and then step frames; without
   * this the very next frame overwrites both from wherever the mouse happens to
   * be, which silently invalidates the shot.
   */
  torchLock: boolean;
  panel: TweakPanel;
  resize(): void;
}

export interface DemoOptions {
  build(s: SceneBuilder): LevelInfo;
  /** Demo-specific panel groups, shown above the shared render groups. */
  groups?(d: DemoDeps): GroupSpec[];
  /** Runs once, after everything is built and before the first frame. */
  init?(d: DemoDeps): void;
  /** Runs each frame before the render call. */
  step?(d: DemoDeps, dt: number, elapsed: number): void;
  /** Keydown handler for demo-specific bindings (`code`, e.g. "Digit1"). */
  key?(d: DemoDeps, code: string): void;
  /** Replaces the help line at the bottom of the page. */
  help?: string;
  /**
   * Replaces the smoke lattice. See Renderer.create.
   *
   * A page that only ever shows one effect in one small room should not be
   * paying for a lattice sized to a 52 m office, and more to the point should
   * not be *rendering* at that lattice's resolution.
   */
  smoke?: SmokeLattice;
}

function fatal(title: string, body: string): void {
  const el = document.getElementById("fatal");
  if (!el) return;
  document.getElementById("fatal-title")!.textContent = title;
  document.getElementById("fatal-body")!.textContent = body;
  el.classList.add("show");
}

export const sl = (
  label: string, min: number, max: number, step: number,
  get: () => number, set: (v: number) => void, onChange?: () => void,
): ControlSpec => ({ kind: "slider", label, min, max, step, get, set, onChange });

export const tg = (
  label: string, get: () => boolean, set: (v: boolean) => void,
): ControlSpec => ({ kind: "toggle", label, get, set });

const DEBUG_VIEWS = [
  "off", "albedo", "normal", "variance/history", "raw 1spp",
  "indirect only", "direct only", "transient only",
  "volume in-scatter", "volume transmittance",
];

/**
 * The render groups both demos want. Gameplay groups (movement, detection) are
 * omitted because the systems behind them are not constructed here.
 */
function renderGroups(d: DemoDeps): GroupSpec[] {
  const { settings, renderer, torch, camera } = d;
  return [
    {
      title: "debug",
      items: [
        {
          kind: "select", label: "view", options: DEBUG_VIEWS,
          get: () => settings.debugView, set: (v) => (settings.debugView = v),
        },
        tg("reference (converge)", () => settings.reference,
          (v) => (settings.reference = v)),
        sl("resolution scale", 0.25, 1.0, 0.05,
          () => settings.resolutionScale,
          (v) => (settings.resolutionScale = v), () => d.resize()),
        sl("bounces", 1, 6, 1, () => settings.bounces, (v) => (settings.bounces = v)),
      ],
    },
    {
      title: "indirect",
      items: [
        {
          kind: "select", label: "indirect mode", options: INDIRECT_MODES as string[],
          get: () => INDIRECT_MODES.indexOf(settings.indirectMode),
          set: (v) => (settings.indirectMode = INDIRECT_MODES[v]),
        },
        tg("restir GI", () => settings.restirGI, (v) => (settings.restirGI = v)),
        sl("spatial taps", 0, 8, 1,
          () => settings.restirSpatialTaps, (v) => (settings.restirSpatialTaps = v)),
        sl("atrous passes (indirect)", 0, 4, 1,
          () => renderer.denoise.atrousIndirect,
          (v) => (renderer.denoise.atrousIndirect = v), () => renderer.applyDenoise()),
        sl("indirect history (frames)", 1, 64, 1,
          () => renderer.denoise.indHistory,
          (v) => (renderer.denoise.indHistory = v), () => renderer.applyDenoise()),
        sl("indirect clamp k", 0.5, 20, 0.5,
          () => renderer.denoise.indClampK,
          (v) => (renderer.denoise.indClampK = v), () => renderer.applyDenoise()),
        sl("indirect alpha floor", 0.02, 1, 0.02,
          () => renderer.denoise.indAlphaFloor,
          (v) => (renderer.denoise.indAlphaFloor = v), () => renderer.applyDenoise()),
        // Cascades only. Interval past one probe spacing is a diagnostic, not
        // a setting — see CascadeTuning.
        sl("cascade reach (x)", 0.25, 4, 0.25,
          () => renderer.cascades.reach, (v) => (renderer.cascades.reach = v)),
        sl("cascade alpha", 0.02, 1, 0.02,
          () => renderer.cascades.alpha, (v) => (renderer.cascades.alpha = v)),
        sl("cascade RIS candidates", 1, 8, 1,
          () => renderer.cascades.candidates, (v) => (renderer.cascades.candidates = v)),
      ],
    },
    {
      title: "smoke fluid",
      items: [
        tg("simulate", () => settings.fluidSim, (v) => (settings.fluidSim = v)),
        sl("jacobi iterations", 4, 200, 2,
          () => renderer.fluid.tune.jacobi, (v) => (renderer.fluid.tune.jacobi = v)),
        sl("vorticity", 0, 20, 0.1,
          () => renderer.fluid.tune.vorticity, (v) => (renderer.fluid.tune.vorticity = v)),
        sl("buoyancy", 0, 20, 0.1,
          () => renderer.fluid.tune.buoyancy, (v) => (renderer.fluid.tune.buoyancy = v)),
        sl("density weight", 0, 1.5, 0.01,
          () => renderer.fluid.tune.weight, (v) => (renderer.fluid.tune.weight = v)),
        sl("dissipation", 0, 1, 0.01,
          () => renderer.fluid.tune.dissipation,
          (v) => (renderer.fluid.tune.dissipation = v)),
        // No slider in the game panel; a demo is exactly where it belongs.
        sl("cooling", 0, 12, 0.1,
          () => renderer.fluid.tune.cooling, (v) => (renderer.fluid.tune.cooling = v)),
      ],
    },
    {
      title: "medium / lighting",
      items: [
        sl("phase g (fwd scatter)", -0.9, 0.9, 0.05,
          () => settings.volPhaseG, (v) => (settings.volPhaseG = v)),
        sl("smoke self-shadow", 0, 4, 0.05,
          () => settings.smokeShadow, (v) => (settings.smokeShadow = v)),
        sl("medium extinction", 0, 1.5, 0.01,
          () => settings.volumetric, (v) => (settings.volumetric = v)),
        sl("volumetric steps", 2, 96, 1,
          () => settings.volumetricSteps, (v) => (settings.volumetricSteps = v)),
        sl("ambient fog", 0, 1, 0.05,
          () => settings.fogAmount, (v) => (settings.fogAmount = v)),
        sl("smoke detail", 0, 4, 0.05,
          () => settings.smokeDetail, (v) => (settings.smokeDetail = v)),
        sl("smoke detail freq", 0.5, 48, 0.5,
          () => settings.smokeDetailFreq, (v) => (settings.smokeDetailFreq = v)),
        sl("sky / moonlight", 0, 3, 0.02,
          () => settings.skyIntensity, (v) => (settings.skyIntensity = v)),
        sl("exposure", 0.02, 0.5, 0.005,
          () => settings.exposure, (v) => (settings.exposure = v)),
      ],
    },
    {
      title: "torch",
      items: [
        tg("on  (F)", () => torch.on, (v) => (torch.on = v)),
        sl("intensity", 0, 800, 5, () => torch.intensity, (v) => (torch.intensity = v)),
        sl("lens radius", 0.005, 0.3, 0.005,
          () => torch.radius, (v) => (torch.radius = v)),
        sl("inner angle", 2, 60, 1, () => torch.innerDeg, (v) => (torch.innerDeg = v)),
        sl("outer angle", 3, 80, 1, () => torch.outerDeg, (v) => (torch.outerDeg = v)),
      ],
    },
    {
      title: "camera",
      collapsed: true,
      items: [
        sl("distance", 3, 40, 0.5, () => camera.distance, (v) => (camera.distance = v)),
        sl("pitch", 0.05, 1.5, 0.01, () => camera.pitch, (v) => (camera.pitch = v)),
        sl("yaw", -3.2, 3.2, 0.01, () => camera.yaw, (v) => (camera.yaw = v)),
        sl("fov", 0.15, 1.2, 0.01, () => camera.fovY, (v) => (camera.fovY = v)),
      ],
    },
  ];
}

export async function bootDemo(opts: DemoOptions): Promise<void> {
  const app = document.getElementById("app")!;
  const canvas = document.createElement("canvas");
  app.appendChild(canvas);
  if (opts.help) {
    const h = document.getElementById("help");
    if (h) h.textContent = opts.help;
  }

  let ctx;
  try {
    ctx = await initGPU(canvas, {
      onDeviceLost: (info) => fatal("GPU device lost", info.message || String(info.reason)),
      onUncapturedError: (error) => console.error("[webgpu]", error.message),
    });
  } catch (e) {
    if (e instanceof GPUInitError) fatal(e.message, e.detail);
    else fatal("Startup failed", String(e));
    return;
  }

  // Scene first, and every material registered inside build(): the material
  // buffer is packed and sized exactly at renderer init and never grows, so a
  // material added afterwards indexes out of bounds.
  const scene = new SceneBuilder();
  const level = opts.build(scene);
  const bvh = buildBVH(scene.boxes);

  const smoke = new Smoke();
  const canisters = new Canisters(scene.boxes, bvh, smoke);
  const flashes = new Flashes();

  let renderer: Renderer;
  try {
    renderer = await Renderer.create(
      ctx, scene, bvh,
      (dims, origin, cell) => {
        (window as unknown as Record<string, unknown>).__occScene =
          { boxes: scene.boxes, query: canisters.query };
        return bakeOccupancy(scene.boxes, canisters.query, dims, origin, cell).data;
      },
      opts.smoke,
    );
  } catch (e) {
    fatal("Shader compilation failed", String(e));
    console.error(e);
    return;
  }

  // No settings-store load and no Brightness: a demo must not inherit or write
  // the game's calibration state, and its exposure is a knob here, not a
  // one-time measurement of the player's monitor.
  const settings: RenderSettings = { ...DEFAULT_SETTINGS };
  const camera = new Camera();
  camera.distance = 14;
  camera.pitch = 0.62;

  const torch: DemoTorch = {
    on: true,
    intensity: 240,
    radius: 0.05,
    innerDeg: 13,
    outerDeg: 26,
    colorR: TORCH_TINT[0],
    colorG: TORCH_TINT[1],
    colorB: TORCH_TINT[2],
    pos: v3(0, 1.5, 0),
    dir: v3(0, 0, 1),
  };

  const resize = (): void => {
    const dpr = Math.min(window.devicePixelRatio || 1, 1.5);
    // Falling back to a fixed size rather than bailing, unlike the game's
    // resize. A context that reports no layout at all — an offscreen or
    // never-presented tab, which is exactly where the headless harness and a
    // background preview live — would otherwise leave `targets` null forever,
    // and every render call returns early with no error. That presents as a
    // simulation that silently never steps.
    const cw = canvas.clientWidth || window.innerWidth || 1280;
    const ch = canvas.clientHeight || window.innerHeight || 720;
    if (cw < 2 || ch < 2) return;
    const w = Math.max(1, Math.floor(cw * dpr));
    const h = Math.max(1, Math.floor(ch * dpr));
    if (canvas.width !== w || canvas.height !== h) {
      canvas.width = w;
      canvas.height = h;
    }
    renderer.resize(
      Math.floor(w * settings.resolutionScale),
      Math.floor(h * settings.resolutionScale),
    );
  };

  const deps: DemoDeps = {
    canvas, renderer, settings, camera, torch, smoke, canisters, flashes, scene, level,
    cursor: v3(level.spawn.x, 0, level.spawn.z),
    focus: v3(level.spawn.x, 1, level.spawn.z),
    walkVel: v3(0, 0, 0),
    torchLock: false,
    panel: null as unknown as TweakPanel,
    resize,
  };
  deps.panel = new TweakPanel(
    [...(opts.groups?.(deps) ?? []), ...renderGroups(deps)],
    () => {
      Object.assign(settings, DEFAULT_SETTINGS);
      Object.assign(renderer.denoise, DEFAULT_DENOISE);
      renderer.applyDenoise();
      resize();
    },
  );

  opts.init?.(deps);
  deps.panel.refresh();
  resize();
  new ResizeObserver(() => resize()).observe(canvas);

  // ---- input: orbit-drag, wheel zoom, WASD walk, cursor-aimed torch --------
  const held = new Set<string>();
  let mouseX = canvas.width / 2, mouseY = canvas.height / 2;
  let dragging = false;
  addEventListener("keydown", (e) => {
    if (e.code === "Backquote") { deps.panel.toggleVisible(); e.preventDefault(); return; }
    if (e.code === "KeyF") torch.on = !torch.on;
    held.add(e.code);
    opts.key?.(deps, e.code);
  });
  addEventListener("keyup", (e) => held.delete(e.code));
  canvas.addEventListener("pointermove", (e) => {
    const dpr = canvas.width / (canvas.clientWidth || 1);
    if (dragging) {
      camera.yaw -= e.movementX * 0.005;
      camera.pitch = Math.min(1.5, Math.max(0.05, camera.pitch + e.movementY * 0.004));
    }
    mouseX = e.offsetX * dpr;
    mouseY = e.offsetY * dpr;
  });
  canvas.addEventListener("pointerdown", (e) => { if (e.button === 2) dragging = true; });
  addEventListener("pointerup", () => { dragging = false; });
  canvas.addEventListener("contextmenu", (e) => e.preventDefault());
  canvas.addEventListener("wheel", (e) => {
    camera.distance = Math.min(40, Math.max(3, camera.distance + e.deltaY * 0.01));
    e.preventDefault();
  }, { passive: false });

  // ---- loop ---------------------------------------------------------------
  let prev = performance.now();
  let elapsed = 0;
  let paused = false;
  let frozenClock: number | null = null;

  function frameBody(now: number): void {
    // Same 50 ms clamp the game uses: the solver integrates this dt, and a
    // tab-switch spike would otherwise advect the field halfway across the room.
    const dt = Math.min((now - prev) / 1000, 0.05);
    prev = now;
    elapsed += dt;

    // WASD walks the focus point along the camera's ground basis.
    const { forward, right } = camera.groundBasis();
    let mx = 0, mz = 0;
    if (held.has("KeyW")) { mx += forward.x; mz += forward.z; }
    if (held.has("KeyS")) { mx -= forward.x; mz -= forward.z; }
    if (held.has("KeyD")) { mx += right.x; mz += right.z; }
    if (held.has("KeyA")) { mx -= right.x; mz -= right.z; }
    const ml = Math.hypot(mx, mz);
    if (ml > 1e-4) {
      const speed = held.has("ShiftLeft") ? 9 : 4;
      deps.walkVel = v3((mx / ml) * speed, 0, (mz / ml) * speed);
      deps.focus.x += deps.walkVel.x * dt;
      deps.focus.z += deps.walkVel.z * dt;
    } else {
      deps.walkVel = v3(0, 0, 0);
    }
    camera.update(dt, deps.focus, canvas.width / Math.max(canvas.height, 1));

    // The torch emits from head height at the focus and points at whatever the
    // cursor is over — the same "sweep a beam across a nearby wall" gesture the
    // game's aim produces, without needing a character to hold it.
    const ground = camera.screenToGround(mouseX, mouseY, canvas.width, canvas.height, 0);
    deps.cursor = ground;
    if (!deps.torchLock) {
      torch.pos = v3(deps.focus.x, 1.5, deps.focus.z);
      const tdx = ground.x - torch.pos.x;
      const tdz = ground.z - torch.pos.z;
      const tl = Math.hypot(tdx, -1.5, tdz) || 1;
      torch.dir = v3(tdx / tl, -1.5 / tl, tdz / tl);
    }

    opts.step?.(deps, dt, elapsed);
    smoke.update(dt, settings.fluidSim);
    canisters.update(dt);
    // Transients every frame, not only when one is live: the renderer splits
    // its light array by index, so the tail has to be rewritten each frame or
    // a spent flash keeps lighting the room.
    flashes.update(dt);
    renderer.updateLights([], flashes.lights());

    renderer.render(
      {
        invViewProj: camera.invViewProj,
        prevViewProj: camera.prevViewProj,
        camPos: camera.pos,
        prevCamPos: camera.prevPos,
        flashPos: torch.pos,
        flashDir: torch.dir,
        flashColor: v3(torch.colorR, torch.colorG, torch.colorB),
        flashIntensity: torch.on ? torch.intensity : 0,
        flashRadius: torch.radius,
        flashCosInner: Math.cos((torch.innerDeg * Math.PI) / 180),
        flashCosOuter: Math.cos((torch.outerDeg * Math.PI) / 180),
        time: elapsed,
        mouseX,
        mouseY,
        seenPulse: 0,
        dt,
        smokeSources: smoke.packed,
        smokeSourceCount: smoke.count,
      },
      settings,
    );
  }

  function frame(now: number): void {
    if (!paused) frameBody(frozenClock ?? now);
    else prev = now;
    requestAnimationFrame(frame);
  }

  // Mirror the game's hook names so the headless scenarios' idioms port over.
  Object.assign(window, {
    __renderer: renderer,
    __settings: settings,
    __fluid: renderer.fluid,
    __smoke: smoke,
    __canisters: canisters,
    __camera: camera,
    __scene: scene,
    __demo: deps,
    __pause: (on: boolean) => { paused = on; },
    __freezeClock: (on: boolean) => { frozenClock = on ? performance.now() : null; },
    __renderStill: async (frames = 30, fixedDtMs = 0): Promise<string> => {
      let t = prev;
      for (let i = 0; i < frames; i++) {
        if (fixedDtMs > 0) t += fixedDtMs;
        frameBody(frozenClock ?? (fixedDtMs > 0 ? t : performance.now()));
      }
      await ctx.device.queue.onSubmittedWorkDone();
      return `rendered ${frames} frames`;
    },
  });

  requestAnimationFrame(frame);
}

// Scatter-vs-gather occupancy oracle. The scatter bake must agree CELL FOR
// CELL, not merely on the solid total — the plausible-looking failure mode
// (counting sub-samples instead of masking them) matches on totals in scenes
// without overlapping geometry and diverges at every wall junction.
(window as unknown as Record<string, unknown>).__occBakeCheck = (
  dims: [number, number, number],
  origin: [number, number, number],
  cell: [number, number, number],
) => {
  const w = window as unknown as { __occScene?: { boxes: unknown[]; query: unknown } };
  const sc = w.__occScene;
  if (!sc) return { error: "no scene captured" };
  const t0 = performance.now();
  const a = bakeOccupancy(sc.boxes as never, sc.query as never, dims, origin, cell);
  const t1 = performance.now();
  const b = bakeOccupancyGather(sc.boxes as never, sc.query as never, dims, origin, cell);
  const t2 = performance.now();
  let diff = 0;
  for (let i = 0; i < a.data.length; i++) if (a.data[i] !== b.data[i]) diff++;
  return {
    cells: a.data.length,
    scatterSolid: a.solidCells, gatherSolid: b.solidCells,
    differingCells: diff,
    scatterMs: +(t1 - t0).toFixed(1), gatherMs: +(t2 - t1).toFixed(1),
  };
};
