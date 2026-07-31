import { GPUInitError, initGPU } from "./engine/gpu";
import {
  DYN_GROUP_SIZE, DEFAULT_SETTINGS, INDIRECT_MODES, RenderSettings, Renderer,
} from "./engine/renderer";
import { loadInto, needsCalibration, resetSettings, SettingsPersister } from "./ui/settings-store";
import { Camera } from "./game/camera";
import { Input } from "./game/input";
import { characterMaterialSpec } from "./game/character";
import { DEFAULT_PATROLS, Guards } from "./game/guards";
import { Detection, detectionTuning } from "./game/detection";
import { NavGrid } from "./game/nav";
import { Flashes } from "./game/flashes";
import { Raycaster } from "./game/raycast";
import { Visibility } from "./game/visibility";
import { Player, PlayerMaterials, movementTuning } from "./game/player";
import { Rig } from "./anim/rig";
import { TRAVERSAL_STACK_DEPTH, buildBVH } from "./scene/bvh";
import { buildOffice } from "./scene/level";
import { BOX_STRIDE_F32, FLAG_NO_CAMERA, SceneBuilder, TORCH_TINT } from "./scene/scene";
import { lerpAngle, v3 } from "./core/math";
import { ControlSpec, TweakPanel } from "./ui/panel";
import { AmmoReadout, DetectionMeter, EquipmentBar, LightGauge } from "./ui/gauge";
import { EndCard, Objective } from "./ui/overlay";
import { Brightness, EXPOSURE_MAX, EXPOSURE_MIN } from "./ui/brightness";
import { Equipment, SLOTS } from "./game/equipment";
import { Particles } from "./game/particles";
import { Smoke } from "./game/smoke";
import { Canisters } from "./game/canister";
import { selfTest as physicsSelfTest } from "./game/physics";
import { bakeOccupancy } from "./scene/occupancy";
import { FrameTimer } from "./engine/frametime";

const QUALITY_PRESETS: Record<string, Partial<RenderSettings>> = {
  // spp stays at 1 everywhere on purpose. With temporal accumulation running,
  // a second sample per frame costs a full extra trace for far less benefit
  // than spending the same time on resolution or another bounce.
  //
  // These are chosen for vblank headroom rather than for the highest resolution
  // that "hits 60". Measured on an M1 Max at 1920x1200 output: preset 1 is
  // 7.2ms so it fits a single 120Hz vblank, and preset 2 is 12.3ms which clears
  // the 16.67ms two-vblank budget by 4.3ms. Pushing preset 2 to 0.7 scale
  // measured 16.67ms — exactly on the boundary, which is the worst place to sit
  // and is what made the build feel juddery despite reporting 60fps.
  Digit1: { resolutionScale: 0.5, spp: 1, bounces: 1, indirectRate: 1.0 },
  Digit2: { resolutionScale: 0.6, spp: 1, bounces: 2, indirectRate: 1.0 },
  Digit3: { resolutionScale: 0.8, spp: 1, bounces: 3, indirectRate: 0.5 },
  Digit4: { resolutionScale: 1.0, spp: 1, bounces: 4, indirectRate: 1.0 },
};
/**
 * Flashlight rig. Intensity is in inverse-square units, so it has to be large
 * for the beam to still read at 10m — this is the number that most controls
 * whether the scene feels like a stealth game or a grey box.
 */
const DEFAULT_FLASH = {
  intensity: 240,
  radius: 0.05,
  colorR: TORCH_TINT[0],
  colorG: TORCH_TINT[1],
  colorB: TORCH_TINT[2],
  innerDeg: 13,
  outerDeg: 26,
};

/** Hoisted: Object.keys allocates, and this was running every frame. */
const QUALITY_PRESET_KEYS = Object.keys(QUALITY_PRESETS);

const QUALITY_NAMES: Record<string, string> = {
  Digit1: "performance",
  Digit2: "balanced",
  Digit3: "high",
  Digit4: "ultra",
};

function fatal(title: string, body: string): void {
  const el = document.getElementById("fatal")!;
  document.getElementById("fatal-title")!.textContent = title;
  document.getElementById("fatal-body")!.textContent = body;
  el.classList.add("show");
}

/** Flash filter modes, index-matched to RenderSettings.transientFilter. */
const TRANSIENT_FILTERS = ["off", "widen", "glow"];

/** Debug view names, index-matched to RenderSettings.debugView. */
const DEBUG_VIEWS = [
  "off", "albedo", "normal", "variance/history", "raw 1spp",
  "indirect only", "direct only", "transient only",
  "volume in-scatter", "volume transmittance",
];

/**
 * How far short of a lamp its line-of-sight check stops.
 *
 * A light sits inside its own fixture, and the CPU raycaster has no emissive
 * filtering the way the shader's occluded() does, so without a margin every
 * lamp occludes itself and nothing can ever be aimed at.
 *
 * Measured over 95 clear-line approaches to every light in the level, counting
 * how many a given margin falsely reports as blocked:
 *
 *   0.00   85/95      0.15   23/95      0.30    6/95
 *   0.05   35/95      0.20   13/95      0.45    2/95
 *   0.10   29/95      0.25    6/95
 *
 * 0.45 rather than something larger because the margin is also a hole: it is
 * the depth of geometry in front of a fixture that this check cannot see, and
 * several lights here are wall-mounted.
 */
const FIXTURE_LOS_MARGIN = 0.45;

/** Fixed benchmark resolution, so runs are comparable across window sizes. */
const BENCH_WIDTH = 1152;
const BENCH_HEIGHT = 720;
/**
 * Session override of the above (__benchResolution). Only for hardware where
 * the standard resolution is impractical — a software rasteriser at ~14s per
 * 1152x720 frame — so a small-frame smoke check can still exercise __bench and
 * __compareToReference end to end.
 */
let benchWidth = BENCH_WIDTH;
let benchHeight = BENCH_HEIGHT;

async function main(): Promise<void> {
  const app = document.getElementById("app")!;
  const canvas = document.createElement("canvas");
  app.appendChild(canvas);

  let ctx;
  try {
    ctx = await initGPU(canvas, {
      // Without this a lost device is the worst possible failure: WebGPU turns
      // most calls into no-ops rather than throwing, so the frame loop's
      // try/catch never fires and the user just sees the last frame, frozen,
      // forever, with nothing in the console.
      onDeviceLost: (info) =>
        fatal("GPU device lost", info.message || String(info.reason)),
      onUncapturedError: (error) => console.error("[webgpu]", error.message),
    });
  } catch (e) {
    if (e instanceof GPUInitError) fatal(e.message, e.detail);
    else fatal("Startup failed", String(e));
    return;
  }

  // ---- scene -------------------------------------------------------------
  const scene = new SceneBuilder();
  const level = buildOffice(scene);

  // The palette lives with the rig in character.ts, since the NVG lens slots
  // have to agree with the geometry that emits from them. Skin stays brighter
  // than a realistic value on purpose: the character spends most of its time
  // standing in the dark behind its own beam, and a charcoal outfit alone would
  // make the silhouette vanish.
  const playerMats = {} as PlayerMaterials;
  for (const m of characterMaterialSpec()) {
    playerMats[m.name] = scene.material(
      v3(...m.albedo),
      m.roughness,
      m.metallic,
      m.emissive ? v3(...m.emissive) : v3(0, 0, 0),
    );
  }

  // Guards wear the same rig in a different uniform. The lit NVG lenses are the
  // player's visual signature, so guards get the same geometry unlit — at this
  // camera distance the goggles are a couple of pixels and the glow is the only
  // thing that reads, which is exactly why it should stay unique to the player.
  const guardMats = {} as PlayerMaterials;
  for (const m of characterMaterialSpec()) {
    const guard = m.name === "cloth"
      ? ([0.13, 0.15, 0.13] as [number, number, number])
      : m.name === "clothDark"
        ? ([0.08, 0.09, 0.08] as [number, number, number])
        : m.albedo;
    const emissive = m.name === "nvgLens" ? undefined : m.emissive;
    guardMats[m.name] = scene.material(
      v3(...guard),
      m.roughness,
      m.metallic,
      emissive ? v3(...emissive) : v3(0, 0, 0),
    );
  }

  // The shader samples lights[0] as the key light on a dedicated channel.
  // If the level ever registers something before the moon, the moon silently
  // falls into the RIS pool and the whole scene goes noisy — catch it here.
  const brightest = scene.lights.reduce((a, b) => (b.intensity > a.intensity ? b : a));
  if (scene.lights[0] !== brightest) {
    console.warn(
      "[scene] lights[0] is not the brightest light. The key-light channel " +
        "expects the moon first; expect noise until the level order is fixed.",
    );
  }

  const t0 = performance.now();
  // ?leaf=N lets the BVH leaf size be A/B'd without an edit-rebuild cycle.
  const leafParam = Number(new URLSearchParams(location.search).get("leaf"));
  const bvh = buildBVH(scene.boxes, Number.isFinite(leafParam) && leafParam > 0 ? leafParam : undefined);
  const bvhMs = performance.now() - t0;
  console.log(
    `[scene] ${scene.boxes.length} boxes, ${scene.materials.length} materials, ` +
      `${scene.lights.length} lights | BVH ${bvh.nodeCount} nodes, depth ${bvh.maxDepth}, ` +
      `built in ${bvhMs.toFixed(1)}ms`,
  );

  // The WGSL traversal silently drops nodes once its fixed stack is full, which
  // would show up as geometry randomly missing from shadows rather than as an
  // error. Fail loudly instead.
  if (bvh.maxDepth >= TRAVERSAL_STACK_DEPTH - 1) {
    console.error(
      `[scene] BVH depth ${bvh.maxDepth} exceeds the traversal stack ` +
        `(${TRAVERSAL_STACK_DEPTH}). Geometry will be silently missing from ` +
        `traces. Raise the stack in common.wgsl or the leaf size.`,
    );
  }

  // Participating-medium smoke: the fluid simulation's source list, and the
  // canister physics that feeds it. Both need the static geometry — the
  // canister for collision, the solver for its baked occupancy (via the same
  // BVH query) — so they exist before the renderer. The canister material
  // must too: materials are packed at renderer init.
  const smoke = new Smoke();
  const canisters = new Canisters(scene.boxes, bvh, smoke);
  const canisterMat = scene.material(v3(0.055, 0.062, 0.055), 0.42, 0.85);
  // A steaming coffee cup left on the conference table, and the warm exhaust
  // rising off the middle server rack: two always-on wisps that show the
  // medium is simulated before anyone fires a shot. Positions come from the
  // level's own furniture (see scene/level.ts).
  smoke.coffee(v3(-21.4, 0.82, 1.9));
  smoke.serverExhaust(v3(13.9, 2.05, 8.5));

  let renderer: Renderer;
  try {
    renderer = await Renderer.create(
      ctx, scene, bvh,
      (dims, origin, cell) =>
        bakeOccupancy(scene.boxes, canisters.query, dims, origin, cell).data,
    );
  } catch (e) {
    fatal("Shader compilation failed", String(e));
    console.error(e);
    return;
  }

  // ---- character rig -----------------------------------------------------
  let rig: Rig;
  try {
    rig = await Rig.load();
    console.log(
      `[rig] ${rig.boneCount} bones, ${rig.clips.size} clips: ` +
        `${[...rig.clips.keys()].join(", ")}`,
    );
  } catch (e) {
    fatal("Character rig failed to load", `${e}\n\nRun: node tools/extract-rig.mjs`);
    console.error(e);
    return;
  }

  // ---- game state --------------------------------------------------------
  const input = new Input(canvas);
  const camera = new Camera();
  const player = new Player(level.spawn, level.colliders, rig);
  const guards = new Guards(rig, DEFAULT_PATROLS);
  const flashes = new Flashes();
  const visibility = new Visibility();
  const gauge = new LightGauge();
  const ammo = new AmmoReadout(Player.SPARE_MAGS, Player.MAG_SIZE);
  const equipment = new Equipment();
  /** The body currently over the player's shoulder, if any. */
  let carried: ReturnType<Guards["nearestBody"]> = null;
  let takedowns = 0;
  const equipBar = new EquipmentBar(SLOTS.map((s) => s.label));
  const particles = new Particles({
    // Blood and chips are lit; sparks emit. Smoke is not a particle any more —
    // see the volumetric puffs below.
    blood: scene.material(v3(0.24, 0.02, 0.02), 0.7, 0),
    spark: scene.material(v3(0, 0, 0), 1, 0, v3(26, 14, 5)),
    debris: scene.material(v3(0.30, 0.29, 0.27), 0.85, 0),
  });
  /**
   * The same BVH the image is traced from, so a bullet stops at the wall you can
   * actually see rather than at a separate collision proxy.
   */
  const raycaster = new Raycaster(scene.boxes, bvh);
  /**
   * Coarse walkable grid over the floor plate, only for guard excursions —
   * chasing a last-known position and walking back. Patrols stay authored.
   */
  const nav = new NavGrid(level.bounds, level.colliders, scene.boxes);
  const detection = new Detection(guards, raycaster, nav);
  const detectMeter = new DetectionMeter(detectionTuning.suspiciousAt, detectionTuning.alertAt);
  const endCard = new EndCard();
  new Objective("EXFIL — EAST END OF THE CORRIDOR");
  /**
   * The exfil zone: the far end of the polished corridor strip. Guard one's
   * lane runs right through it, so getting out clean means timing his loop.
   */
  const EXFIL = { minX: 22.5, maxX: 25.4, minZ: -1.6, maxZ: 1.6 };
  /** Set once the run has an ending; only R does anything after that. */
  let ended: "dead" | "win" | null = null;
  /** Smoothed 0..1 the post pass pulses the frame edge with. */
  let seenPulse = 0;

  /**
   * One packed buffer for every animated box in the scene. Sized to the
   * renderer's cap so an overflowing guard is dropped by Guards.buildBoxes
   * rather than silently truncated further down.
   */
  const dynBoxes = new Float32Array(208 * BOX_STRIDE_F32);
  const dynBoxFlags = new Uint32Array(dynBoxes.buffer);
  /** Player boxes packed this frame; the temporal probe projects exactly these. */
  let playerBoxCount = 0;
  /** Debug probe switch: player boxes get FLAG_NO_CAMERA when set. */
  let hidePlayerFromCamera = false;
  const settings: RenderSettings = { ...DEFAULT_SETTINGS };
  // Before anything reads settings: the renderer sizes its targets from
  // resolutionScale, and the panel captures the current values as it builds.
  loadInto(settings);
  const persister = new SettingsPersister();
  const firstRun = needsCalibration();
  let qualityKey = "Digit1";
  let qualityName = QUALITY_NAMES.Digit1;

  const flash = { ...DEFAULT_FLASH };

  const hud = document.getElementById("hud")!;
  let lastResize = 0;
  /**
   * Internal resolution a scenario pinned (__pinResolution). A bare
   * `__renderer.resize()` does not survive: the canvas ResizeObserver fires at
   * startup, and the deferred resize below consumes that notification on the
   * next frame the scenario steps — re-deriving the size from the canvas and
   * silently tracing 4x the pixels the scenario asked for. While pinned, the
   * deferred resize re-asserts the pin instead.
   */
  let pinnedRes: { w: number; h: number } | null = null;
  let groupSizeWarned = false;
  /** Frame-time and pass-cost overlay. Off by default; it is a developer tool. */
  let showStats = false;

  const frameTimer = new FrameTimer();

  /**
   * Present-rate cap.
   *
   * rAF fires at the display rate, so on a 120Hz panel an uncapped loop that
   * cannot hold 120 lands on an uneven mix of 1- and 2-vblank frames. Rendering
   * on every Nth callback instead gives a cadence locked to vblank, which is
   * what actually feels smooth. 0 means uncapped.
   */
  const pacing = { targetFps: 60, tick: 0 };
  /** Suppressed while benchmarking, which submits frames back to back. */
  let recordFrameTimes = true;

  /**
   * Adaptive resolution.
   *
   * On a 120Hz panel the only frame times that feel smooth are the ones that
   * land consistently inside a whole number of vblank intervals. A build
   * averaging 60fps but oscillating around the 16.6ms boundary flips between 2
   * and 3 intervals every frame, which reads as much worse than a stable 20ms.
   * Rather than pick a fixed resolution and hope, regulate p95 frame time
   * against the budget and let the internal resolution float.
   */
  const adaptive = {
    // Off by default. Every resolution change reallocates the render targets,
    // which throws away the temporal history and dumps a burst of noise — and
    // that burst changes the frame time, which makes the controller act again.
    // The presets now have enough headroom that regulating is not needed, so
    // the stable choice is to leave it alone.
    enabled: false,
    /** Vblank intervals to fit inside. 1 = every frame, 2 = every other. */
    targetIntervals: 2,
    scale: DEFAULT_SETTINGS.resolutionScale,
    cooldown: 0,
  };

  function updateAdaptiveResolution(dt: number): void {
    if (!adaptive.enabled) return;
    adaptive.cooldown -= dt;
    if (adaptive.cooldown > 0) return;

    const p95 = frameTimer.p95;
    if (p95 <= 0.5) return;

    // Aim comfortably under the boundary; landing exactly on it is the failure
    // mode we are trying to escape.
    const budget = adaptive.targetIntervals * frameTimer.refreshMs * 0.86;
    const prev = adaptive.scale;

    if (p95 > budget) {
      adaptive.scale *= Math.max(0.88, budget / p95);
    } else if (p95 < budget * 0.72) {
      adaptive.scale *= 1.04;
    }
    adaptive.scale = Math.min(1.0, Math.max(0.3, adaptive.scale));

    // Reallocating every render target is expensive, so only act on changes
    // big enough to be worth it.
    if (Math.abs(adaptive.scale - prev) > 0.015) {
      settings.resolutionScale = Math.round(adaptive.scale * 100) / 100;
      resize();
      panel.refresh();
      adaptive.cooldown = 0.4;
      frameTimer.reset();
    } else {
      adaptive.cooldown = 0.25;
    }
  }

  function resize(): void {
    // Cap DPR: on a 3024x1964 Retina panel, native resolution would mean
    // path tracing ~6M pixels, which is not the right trade.
    const dpr = Math.min(window.devicePixelRatio || 1, 1.5);
    // The canvas fills the viewport, so fall back to the window when layout has
    // not happened yet. Trusting clientWidth alone leaves the backing store
    // stuck at its initial size forever, which presents as a black screen with
    // a suspiciously fast GPU time rather than as an error.
    const cw = canvas.clientWidth || window.innerWidth;
    const ch = canvas.clientHeight || window.innerHeight;
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
  }

  window.addEventListener("resize", () => {
    lastResize = performance.now();
  });
  // Observe the element itself. Startup races layout, so a window-resize
  // listener alone can leave the canvas stuck at its initial size forever.
  new ResizeObserver(() => { lastResize = performance.now(); }).observe(canvas);
  resize();

  // Aim up-screen by default. The mouse starts at (0,0) otherwise, which points
  // the character back at the camera and puts the lens flare in your face on
  // the very first frame.
  input.mouseX = canvas.width * 0.5;
  input.mouseY = canvas.height * 0.3;

  // ---- debug panel -------------------------------------------------------
  const sl = (
    label: string,
    min: number,
    max: number,
    step: number,
    get: () => number,
    set: (v: number) => void,
    onChange?: () => void,
  ): ControlSpec => ({ kind: "slider", label, min, max, step, get, set, onChange });

  const tg = (label: string, get: () => boolean, set: (v: boolean) => void): ControlSpec =>
    ({ kind: "toggle", label, get, set });

  const panel = new TweakPanel(
    [
      {
        title: "debug",
        items: [
          tg("stats overlay", () => showStats, (v) => {
            showStats = v;
            if (!v) hud.textContent = "";
          }),
          // Machine-independent work counters, read from __stats.counters.
          // Counting only: while on, timings are contention-inflated garbage.
          tg("work counters", () => settings.counters, (v) => (settings.counters = v)),
          {
            kind: "select",
            label: "quality",
            options: QUALITY_PRESET_KEYS.map((k) => QUALITY_NAMES[k]),
            get: () => QUALITY_PRESET_KEYS.indexOf(qualityKey),
            set: (v) => {
              qualityKey = QUALITY_PRESET_KEYS[v];
              Object.assign(settings, QUALITY_PRESETS[qualityKey]);
              qualityName = QUALITY_NAMES[qualityKey];
              resize();
              panel.refresh();
            },
          },
          {
            kind: "select",
            label: "view",
            options: DEBUG_VIEWS,
            get: () => settings.debugView,
            set: (v) => (settings.debugView = v),
          },
        ],
      },
      {
        title: "quality",
        items: [
          sl("resolution scale", 0.25, 1.0, 0.05,
            () => settings.resolutionScale, (v) => (settings.resolutionScale = v), resize),
          sl("bounces", 1, 6, 1, () => settings.bounces, (v) => (settings.bounces = v)),
          sl("samples / px", 1, 4, 1, () => settings.spp, (v) => (settings.spp = v)),
          sl("indirect rate", 0.25, 1, 0.25,
            () => settings.indirectRate, (v) => (settings.indirectRate = v)),
        ],
      },
      {
        title: "restir (direct light)",
        items: [
          tg("temporal reuse", () => settings.restirTemporal,
            (v) => (settings.restirTemporal = v)),
          tg("restir GI", () => settings.restirGI,
            (v) => (settings.restirGI = v)),
          sl("fresh candidates", 1, 16, 1,
            () => settings.restirCandidates, (v) => (settings.restirCandidates = v)),
          sl("spatial taps", 0, 8, 1,
            () => settings.restirSpatialTaps, (v) => (settings.restirSpatialTaps = v)),
          sl("spatial radius", 2, 32, 1,
            () => settings.restirSpatialRadius, (v) => (settings.restirSpatialRadius = v)),
          tg("flashmap sampling", () => settings.flashVisTarget,
            (v) => (settings.flashVisTarget = v)),
          tg("flashmap beam", () => settings.flashVisVolumetric,
            (v) => (settings.flashVisVolumetric = v)),
          {
            kind: "select",
            label: "indirect mode",
            options: INDIRECT_MODES,
            get: () => INDIRECT_MODES.indexOf(settings.indirectMode),
            set: (v) => (settings.indirectMode = INDIRECT_MODES[v]),
          },
          // Lower stays responsive to the moving flashlight; higher converges
          // further but smears when the light sweeps.
          sl("flash rays", 1, 16, 1,
            () => settings.transientSamples, (v) => (settings.transientSamples = v)),
          // The two flash filters are different answers to the same problem
          // and the choice is a look, so it lives here rather than in a
          // constant. "widen" measured worse than "off" on firefly ratio; it
          // is on the panel to be judged by eye, not because the numbers
          // recommend it.
          {
            kind: "select",
            label: "flash filter",
            options: TRANSIENT_FILTERS,
            get: () => settings.transientFilter,
            set: (v) => (settings.transientFilter = v),
          },
          sl("flash blur dist", 2, 40, 1,
            () => settings.transientBlurDist, (v) => (settings.transientBlurDist = v)),
          sl("flash bounce blur", 0, 2, 0.05,
            () => settings.transientBounceWeight,
            (v) => (settings.transientBounceWeight = v)),
          sl("flash blur reach", 1, 12, 0.5,
            () => settings.transientBlurStride,
            (v) => (settings.transientBlurStride = v)),
          sl("flash blur amount", 0, 1, 0.02,
            () => settings.transientBlurStrength,
            (v) => (settings.transientBlurStrength = v)),
          sl("reuse cap (M)", 1, 60, 1,
            () => settings.restirMCap, (v) => (settings.restirMCap = v)),
        ],
      },
      {
        title: "tonemap",
        items: [
          // Same range as the player-facing control, so the two cannot
          // disagree about what a legal exposure is.
          sl("exposure", EXPOSURE_MIN, EXPOSURE_MAX, 0.005,
            () => settings.exposure,
            (v) => { settings.exposure = v; brightness.sync(v); }),
          sl("saturation", 0, 2, 0.01, () => settings.saturation, (v) => (settings.saturation = v)),
          sl("bloom", 0, 2, 0.01,
            () => settings.bloomIntensity, (v) => (settings.bloomIntensity = v)),
          sl("bloom threshold", 0.1, 4, 0.05,
            () => settings.bloomThreshold, (v) => (settings.bloomThreshold = v)),
          sl("vignette", 0, 1, 0.01, () => settings.vignette, (v) => (settings.vignette = v)),
          sl("grain", 0, 0.1, 0.002, () => settings.grain, (v) => (settings.grain = v)),
          sl("chromatic ab.", 0, 0.02, 0.0002,
            () => settings.chromatic, (v) => (settings.chromatic = v)),
        ],
      },
      {
        title: "lighting",
        items: [
          sl("ambient floor", 0, 0.2, 0.002, () => settings.ambient, (v) => (settings.ambient = v)),
          sl("sky / moonlight", 0, 3, 0.02,
            () => settings.skyIntensity, (v) => (settings.skyIntensity = v)),
          // Extinction per metre at unit density; scattering albedo is 1.
          sl("medium extinction", 0, 0.3, 0.005,
            () => settings.volumetric, (v) => (settings.volumetric = v)),
          sl("volumetric steps", 2, 24, 1,
            () => settings.volumetricSteps, (v) => (settings.volumetricSteps = v)),
          sl("ambient fog", 0, 1, 0.05,
            () => settings.fogAmount, (v) => (settings.fogAmount = v)),
          tg("extinction", () => settings.volExtinction,
            (v) => (settings.volExtinction = v)),
        ],
      },
      {
        title: "smoke fluid",
        items: [
          tg("simulate", () => settings.fluidSim, (v) => (settings.fluidSim = v)),
          // Solver tuning lives on the solver; these steer the medium's
          // character (projection quality, curl, lift, pooling, lifetime) live.
          sl("jacobi iterations", 4, 200, 2,
            () => renderer.fluid.tune.jacobi, (v) => (renderer.fluid.tune.jacobi = v)),
          sl("vorticity", 0, 6, 0.1,
            () => renderer.fluid.tune.vorticity, (v) => (renderer.fluid.tune.vorticity = v)),
          sl("buoyancy", 0, 6, 0.1,
            () => renderer.fluid.tune.buoyancy, (v) => (renderer.fluid.tune.buoyancy = v)),
          sl("density weight", 0, 0.3, 0.005,
            () => renderer.fluid.tune.weight, (v) => (renderer.fluid.tune.weight = v)),
          sl("dissipation", 0, 1, 0.01,
            () => renderer.fluid.tune.dissipation, (v) => (renderer.fluid.tune.dissipation = v)),
        ],
      },
      {
        title: "flashlight",
        items: [
          sl("intensity", 0, 800, 5, () => flash.intensity, (v) => (flash.intensity = v)),
          // Lens radius drives penumbra width — the single best "is this real?" knob.
          sl("lens radius", 0.005, 0.3, 0.005, () => flash.radius, (v) => (flash.radius = v)),
          sl("inner angle", 2, 60, 1, () => flash.innerDeg, (v) => (flash.innerDeg = v)),
          sl("outer angle", 3, 80, 1, () => flash.outerDeg, (v) => (flash.outerDeg = v)),
          sl("warmth", 0, 1, 0.01,
            () => 1 - flash.colorB,
            (v) => { flash.colorG = 1 - v * 0.16; flash.colorB = 1 - v; }),
        ],
      },
      {
        title: "night vision",
        items: [
          tg("enabled  (N)", () => settings.nightVision, (v) => (settings.nightVision = v)),
          sl("tube gain", 5, 1200, 5, () => settings.nvGain, (v) => (settings.nvGain = v)),
          sl("green <- -> white", 0, 1, 0.01,
            () => 1 - settings.nvPhosphor, (v) => (settings.nvPhosphor = 1 - v)),
        ],
      },
      {
        title: "frame pacing",
        items: [
          sl("present cap (fps, 0=off)", 0, 120, 30,
            () => pacing.targetFps, (v) => { pacing.targetFps = v; frameTimer.reset(); }),
          tg("adaptive resolution", () => adaptive.enabled, (v) => {
            adaptive.enabled = v;
            adaptive.scale = settings.resolutionScale;
          }),
          // 1 = fit inside a single vblank, 2 = every other, and so on.
          sl("target vblanks", 1, 4, 1,
            () => adaptive.targetIntervals, (v) => (adaptive.targetIntervals = v)),
        ],
      },
      {
        title: "movement / turning",
        items: [
          sl("max twist (deg)", 10, 110, 5,
            () => movementTuning.maxTwistDeg, (v) => (movementTuning.maxTwistDeg = v)),
          sl("max twist crouched", 10, 90, 5,
            () => movementTuning.maxTwistCrouchDeg,
            (v) => (movementTuning.maxTwistCrouchDeg = v)),
          // Above this angle between aim and travel the character backs up
          // rather than turning away from the cursor.
          sl("backpedal enter (deg)", 60, 175, 5,
            () => movementTuning.backwardEnterDeg,
            (v) => (movementTuning.backwardEnterDeg = v)),
          sl("backpedal exit (deg)", 30, 170, 5,
            () => movementTuning.backwardExitDeg,
            (v) => (movementTuning.backwardExitDeg = v)),
          // Turn rates are "fraction of the error remaining after one second",
          // so smaller is snappier.
          sl("turn rate moving", 0.00005, 0.02, 0.00005,
            () => movementTuning.turnRateMoving, (v) => (movementTuning.turnRateMoving = v)),
          sl("turn rate crouched", 0.00005, 0.02, 0.00005,
            () => movementTuning.turnRateCrouch, (v) => (movementTuning.turnRateCrouch = v)),
          sl("turn rate standing", 0.0005, 0.08, 0.0005,
            () => movementTuning.turnRateStanding,
            (v) => (movementTuning.turnRateStanding = v)),
        ],
      },
      {
        title: "detection",
        collapsed: true,
        items: [
          sl("view range", 5, 40, 1,
            () => detectionTuning.viewRange, (v) => (detectionTuning.viewRange = v)),
          sl("fov half-angle", 10, 90, 1,
            () => detectionTuning.fovDeg, (v) => (detectionTuning.fovDeg = v)),
          sl("seen rate", 0.1, 4, 0.05,
            () => detectionTuning.seenRate, (v) => (detectionTuning.seenRate = v)),
          sl("beam rate", 0.1, 6, 0.05,
            () => detectionTuning.beamRate, (v) => (detectionTuning.beamRate = v)),
          sl("decay rate", 0.01, 0.5, 0.005,
            () => detectionTuning.decayRate, (v) => (detectionTuning.decayRate = v)),
          sl("suspicious at", 0.05, 0.6, 0.01,
            () => detectionTuning.suspiciousAt, (v) => (detectionTuning.suspiciousAt = v)),
          sl("alert at", 0.3, 0.95, 0.01,
            () => detectionTuning.alertAt, (v) => (detectionTuning.alertAt = v)),
          sl("footstep range", 0, 20, 0.5,
            () => detectionTuning.footstepRange, (v) => (detectionTuning.footstepRange = v)),
          sl("gunshot range", 5, 60, 1,
            () => detectionTuning.gunshotRange, (v) => (detectionTuning.gunshotRange = v)),
          sl("callout range", 0, 40, 1,
            () => detectionTuning.calloutRange, (v) => (detectionTuning.calloutRange = v)),
          sl("search time", 2, 30, 1,
            () => detectionTuning.searchTime, (v) => (detectionTuning.searchTime = v)),
          sl("fire reaction", 0, 3, 0.05,
            () => detectionTuning.fireReaction, (v) => (detectionTuning.fireReaction = v)),
          sl("fire interval", 0.25, 2, 0.05,
            () => detectionTuning.fireInterval, (v) => (detectionTuning.fireInterval = v)),
          sl("fire spread (deg)", 0, 15, 0.5,
            () => detectionTuning.fireSpreadDeg, (v) => (detectionTuning.fireSpreadDeg = v)),
        ],
      },
      {
        title: "camera",
        collapsed: true,
        items: [
          sl("distance", 8, 60, 0.5, () => camera.distance, (v) => (camera.distance = v)),
          sl("pitch", 0.2, 1.5, 0.01, () => camera.pitch, (v) => (camera.pitch = v)),
          sl("yaw", -3.2, 3.2, 0.01, () => camera.yaw, (v) => (camera.yaw = v)),
          sl("fov", 0.15, 1.2, 0.01, () => camera.fovY, (v) => (camera.fovY = v)),
        ],
      },
    ],
    () => {
      Object.assign(settings, DEFAULT_SETTINGS);
      Object.assign(flash, DEFAULT_FLASH);
      resize();
    },
  );

  // Constructed after the panel on purpose: its constructor applies the stored
  // exposure immediately, and that callback refreshes the panel.
  const brightness = new Brightness(
    settings.exposure,
    (v) => {
      settings.exposure = v;
      panel.refresh();
    },
    // Dismissing calibration is what counts as having seen it — whether they
    // tuned it or hit DONE straight away, asking again next launch would be
    // nagging.
    () => persister.markCalibrated(settings),
  );
  if (firstRun) brightness.open();

  // ---- loop --------------------------------------------------------------
  let prev = performance.now();
  let fpsAccum = 0;
  let elapsed = 0;

  let loopBroken = false;
  /**
   * Benchmark guard.
   *
   * bench() drives frameBody directly and pins the internal resolution well
   * above the interactive one. A run that overruns, or a caller that abandons
   * the promise, used to leave the app rendering at bench resolution
   * indefinitely — which on a heavy build is enough to saturate the GPU and
   * make the whole machine feel bad. So the restore is not left to the promise:
   * the run carries a hard deadline it checks itself, and the rAF loop stands
   * down while it is active rather than interleaving with it.
   */
  const benchGuard = { active: false, abortAt: 0, aborted: false };
  /** No single bench may hold the app hostage for longer than this. */
  const BENCH_MAX_MS = 8000;
  /** Same guard for the reference comparison, which converges for far longer. */
  const COMPARE_MAX_MS = 120_000;
  /**
   * Session overrides of the two caps (__benchResolution). The caps protect an
   * interactive session; a scripted headless run on a software rasteriser is
   * ~100x slower per frame and would otherwise hit them before rendering
   * anything worth measuring.
   */
  let benchMaxMs = BENCH_MAX_MS;
  let compareMaxMs = COMPARE_MAX_MS;
  /**
   * Scenario clock. While set, the rAF loop stands down and only __renderStill
   * advances the world, by a fixed step, so a headless assert measures game
   * time rather than however fast this machine happened to trace frames.
   */
  let paused = false;
  /**
   * Frozen clock (__freezeClock). While set, every frame is fed this same
   * timestamp, so dt is exactly 0 and nothing animates while the frame index
   * still advances — the state a still-image A/B needs, since two captures of
   * a moving character are two different scenes. The compareToReference
   * freeze predates this and stays local to that hook.
   */
  let frozenClock: number | null = null;

  function frame(now: number): void {
    // A bench owns frameBody while it runs; stepping it from here too would
    // double the work and corrupt the timings. A paused scenario likewise.
    if (benchGuard.active || paused) {
      requestAnimationFrame(frame);
      return;
    }
    // Skip callbacks to hold the requested present rate. Locking the cadence to
    // a whole number of vblanks matters more than raw throughput.
    if (pacing.targetFps > 0) {
      const every = Math.max(1, Math.round(1000 / frameTimer.refreshMs / pacing.targetFps));
      // Declare the cap so the refresh estimator can divide it out. Without it,
      // a cap active before the first estimate is ambiguous: 16.67ms deltas are
      // genuinely 60Hz *or* 120Hz rendered every other vblank.
      frameTimer.setPresentCap(every);
      if (every > 1 && pacing.tick++ % every !== 0) {
        requestAnimationFrame(frame);
        return;
      }
    }
    try {
      frameBody(frozenClock ?? now);
    } catch (e) {
      if (!loopBroken) {
        loopBroken = true;
        console.error("[frame] loop aborted:", e);
        fatal("Render loop error", String(e));
      }
      return;
    }
    requestAnimationFrame(frame);
  }

  // Exposed for automated inspection / profiling from the console.
  const stats = {
    frames: 0,
    lastFrameMs: 0,
    avgFrameMs: 0,
    /**
     * Built on read rather than per frame. This is only ever inspected from the
     * devtools console, and materialising the map every frame was ~4,800 short-
     * lived objects a second for something nobody was looking at.
     */
    get gpu(): Record<string, number> {
      return Object.fromEntries(renderer.profiler.timings);
    },
    /**
     * Work counters for the most recent completed frame — raw totals plus
     * per-pixel — a frame or two behind, like the profiler. Null while
     * settings.counters is off.
     */
    get counters() {
      return renderer.workCounters.latest;
    },
  };
  /**
   * Runs N frames back to back and waits for the GPU to drain. Does not depend
   * on requestAnimationFrame, which the browser throttles when the tab is not
   * being composited — that makes rAF useless for measurement here.
   */
  /**
   * Converges a ground-truth image, then measures a candidate configuration
   * against it.
   *
   * `refFrames` brute-force samples establish the reference; each entry in
   * `configs` is then accumulated for `testFrames` and diffed against it.
   *
   * `refOverrides` must pin any setting that changes the transport being
   * measured — `bounces` above all. The reference inherits current settings
   * otherwise, so testing a 3-bounce config against a reference left at 1 bounce
   * silently measures truncation error and reports it as estimator bias.
   *
   * Existing to answer the one question eyeballing cannot: a resampling
   * estimator with a wrong Jacobian or wrong MIS weights still converges
   * smoothly — it just converges somewhere else. Relative error against a
   * reference is the only thing that distinguishes that from correct.
   */
  async function compareToReference(
    configs: Record<string, Partial<RenderSettings>>,
    refFrames = 400,
    testFrames = 120,
    refOverrides: Partial<RenderSettings> = {},
    // Software rendering (headless SwiftShader) cannot afford the bench
    // resolution; the measurement is per-pixel, so a smaller frame is the
    // same test at lower cost. It also needs a longer wall clock than the
    // 2-minute default, hence the budget knob.
    dims: { width: number; height: number } = { width: benchWidth, height: benchHeight },
    maxMs = compareMaxMs,
  ): Promise<string> {
    if (benchGuard.active) throw new Error("a benchmark is already running");
    const saved = { ...settings };
    const wasAdaptive = adaptive.enabled;
    adaptive.enabled = false;
    recordFrameTimes = false;
    benchGuard.active = true;
    benchGuard.aborted = false;
    // Convergence needs far longer than a perf run; this is still a hard cap.
    benchGuard.abortAt = performance.now() + maxMs;
    try {
      // A frozen scene is a precondition, not a nicety: the accumulator has no
      // way to tell a moving light from a noisy estimate, and the guards keep
      // patrolling otherwise — so the reference and the candidate would be
      // measuring different scenes.
      //
      // Freezing is done by feeding frameBody the *same* timestamp every call,
      // which makes dt exactly 0: nothing animates, but the frame counter still
      // advances, so each frame draws fresh random samples of the same scene.
      // That is precisely the condition a progressive average needs.
      pinBenchPose();
      const frozen = performance.now();

      const accumulate = async (n: number) => {
        renderer.resize(dims.width, dims.height);
        // Force a fresh history so the previous config cannot bleed in.
        renderer.resize(dims.width, dims.height + 1);
        renderer.resize(dims.width, dims.height);
        for (let i = 0; i < n; i++) {
          if (benchExpired()) break;
          frameBody(frozen);
        }
        await ctx!.device.queue.onSubmittedWorkDone();
        return renderer.readHDR();
      };

      Object.assign(settings, refOverrides, { reference: true });
      const ref = await accumulate(refFrames);

      const results: Record<string, unknown> = {};
      for (const [name, cfg] of Object.entries(configs)) {
        Object.assign(settings, saved, cfg, { reference: false });
        const test = await accumulate(testFrames);
        results[name] = imageError(ref.data, test.data);
      }
      return JSON.stringify({
        res: `${ref.width}x${ref.height}`,
        ...benchResFields(),
        refBounces: (refOverrides.bounces ?? saved.bounces),
        refFrames,
        testFrames,
        truncated: benchGuard.aborted,
        results,
      }, null, 2);
    } finally {
      benchGuard.active = false;
      Object.assign(settings, saved);
      adaptive.enabled = wasAdaptive;
      recordFrameTimes = true;
      frameTimer.reset();
      resize();
    }
  }

  /**
   * Error of `test` against `ref`.
   *
   * Reported relative to the reference's own mean rather than absolutely, since
   * this scene is mostly near-black and an absolute MSE would be dominated by
   * the handful of pixels inside the beam.
   */
  function imageError(ref: Float32Array, test: Float32Array): Record<string, number> {
    let se = 0, refSum = 0, n = 0, maxRel = 0;
    let biasSum = 0;
    for (let i = 0; i < ref.length; i += 4) {
      for (let c = 0; c < 3; c++) {
        const r = ref[i + c], t = test[i + c];
        if (!Number.isFinite(r) || !Number.isFinite(t)) continue;
        const d = t - r;
        se += d * d;
        biasSum += d;
        refSum += r;
        n++;
        // Ignore near-black pixels: relative error there is meaningless.
        if (r > 0.01) maxRel = Math.max(maxRel, Math.abs(d) / r);
      }
    }
    const mean = refSum / Math.max(n, 1);
    return {
      rmse: +Math.sqrt(se / Math.max(n, 1)).toFixed(6),
      // The headline number: RMSE as a fraction of the reference's mean level.
      relRmse: +(Math.sqrt(se / Math.max(n, 1)) / Math.max(mean, 1e-6)).toFixed(4),
      // Signed, so a systematically brighter or darker result is visible.
      // A correct estimator should sit near zero; a bad Jacobian will not.
      relBias: +(biasSum / Math.max(n, 1) / Math.max(mean, 1e-6)).toFixed(4),
      maxRelErr: +maxRel.toFixed(3),
      refMean: +mean.toFixed(5),
    };
  }

  /**
   * Overrides the benchmark resolution (and optionally the deadline guards)
   * for this session; see benchWidth. The reply names any departure from the
   * standard resolution so a small-frame result is never read as comparable.
   */
  function benchResolution(w: number, h: number, capSeconds?: number): string {
    benchWidth = Math.max(2, Math.floor(w));
    benchHeight = Math.max(2, Math.floor(h));
    if (capSeconds && capSeconds > 0) {
      benchMaxMs = capSeconds * 1000;
      compareMaxMs = capSeconds * 1000;
    }
    const std = benchWidth === BENCH_WIDTH && benchHeight === BENCH_HEIGHT;
    return `bench resolution ${benchWidth}x${benchHeight}` +
      (std ? "" : ` (non-standard — the standard is ${BENCH_WIDTH}x${BENCH_HEIGHT}; ` +
        `pixel counts and timings do not compare across resolutions)`) +
      (capSeconds && capSeconds > 0 ? `; deadline ${capSeconds}s` : "");
  }

  /** Result-blob marker, present only when the resolution is overridden. */
  function benchResFields(): { nonStandardRes?: true } {
    return benchWidth === BENCH_WIDTH && benchHeight === BENCH_HEIGHT
      ? {} : { nonStandardRes: true };
  }

  async function bench(n = 90, serial = false): Promise<string> {
    if (benchGuard.active) {
      throw new Error("a benchmark is already running; wait for it to finish");
    }
    // Back-to-back submits say nothing about frame pacing.
    recordFrameTimes = false;
    const wasAdaptive = adaptive.enabled;
    adaptive.enabled = false;
    benchGuard.active = true;
    benchGuard.aborted = false;
    benchGuard.abortAt = performance.now() + benchMaxMs;
    try {
      return await benchInner(n, serial);
    } finally {
      benchGuard.active = false;
      recordFrameTimes = true;
      adaptive.enabled = wasAdaptive;
      frameTimer.reset();
      resize();
    }
  }

  /** True once the run has outstayed its deadline; loops below bail on it. */
  function benchExpired(): boolean {
    if (benchGuard.aborted) return true;
    if (performance.now() > benchGuard.abortAt) {
      benchGuard.aborted = true;
      console.warn(
        `[bench] exceeded its deadline and was cut short. The result is ` +
          `from fewer frames than requested — treat it as indicative only.`,
      );
      return true;
    }
    return false;
  }

  /**
   * Pin a fixed pose AND a fixed equipment state. Cost and noise both vary a
   * lot with what is on screen, so without this every measurement is against
   * a different scene and numbers are not comparable between runs.
   *
   * The equipment pin exists because the flashlight is only live while a
   * weapon is drawn and nothing is carried. A benchmark that happens to run
   * from a fresh spawn (slot "hands") measures a scene without the hero light
   * — which once made three flashmap configs return bit-identical images and
   * read as "no effect" when the real difference was large.
   */
  function pinBenchPose(): void {
    player.pos.x = -2;
    player.pos.z = -11;
    player.yaw = -0.6;
    player.velX = 0;
    player.velZ = 0;
    player.flashlightOn = true;
    player.carrying = false;
    equipment.select(1);
    input.mouseX = canvas.width * 0.36;
    input.mouseY = canvas.height * 0.42;
    camera.distance = 26;
    for (let i = 0; i < 3; i++) camera.update(0.5, player.pos, 16 / 9);
  }

  async function benchInner(n: number, serial: boolean): Promise<string> {
    pinBenchPose();

    // Pin the internal resolution absolutely, not as a fraction of the canvas.
    // The window can differ between runs, and scaling off it silently changes
    // the pixel count being measured — which makes two runs look like a large
    // speedup when they simply rendered different numbers of pixels.
    renderer.resize(benchWidth, benchHeight);

    renderer.profiler.timings.clear();
    // Warm up so shader/pipeline caches and the temporal history are settled.
    for (let i = 0; i < 20; i++) {
      if (benchExpired()) break;
      frameBody(performance.now());
    }
    await ctx!.device.queue.onSubmittedWorkDone();

    // Serial mode drains the GPU between frames. Per-pass timestamps are only
    // meaningful this way: with frames in flight the passes overlap and every
    // timestamp absorbs queue wait. Wall-clock throughput, conversely, is only
    // meaningful in pipelined mode.
    const t0 = performance.now();
    let ran = 0;
    for (let i = 0; i < n; i++) {
      if (benchExpired()) break;
      frameBody(performance.now());
      ran++;
      if (serial) await ctx!.device.queue.onSubmittedWorkDone();
    }
    await ctx!.device.queue.onSubmittedWorkDone();
    const wall = (performance.now() - t0) / Math.max(ran, 1);

    const gpu = Object.fromEntries(
      [...renderer.profiler.timings].map(([k, v]) => [k, +v.toFixed(3)]),
    );
    const gpuTotal = renderer.profiler.total();
    // Counters land asynchronously. The pipelined loop above never turns the
    // event loop, so no readback slot frees mid-run and the newest staged
    // frame is an early one; take one turn so the ring drains, then render an
    // untimed frame to report end-of-run counts (the block carries its frame
    // index either way). Serial mode already drained per frame.
    let counters: Record<string, unknown> = {};
    if (settings.counters) {
      if (!serial) {
        await new Promise((r) => setTimeout(r, 0));
        frameBody(performance.now());
        await ctx!.device.queue.onSubmittedWorkDone();
      }
      counters = { counters: await renderer.workCounters.flush() };
    }
    return JSON.stringify({
      res: `${renderer.renderWidth}x${renderer.renderHeight}`,
      ...benchResFields(),
      // Surfaced so a run cut short by the deadline cannot be mistaken for a
      // full one. Compare against the requested count before trusting a delta.
      frames: ran,
      truncated: ran < n,
      spp: settings.spp,
      bounces: settings.bounces,
      volumetric: settings.volumetric,
      wallMsPerFrame: +wall.toFixed(2),
      fps: +(1000 / wall).toFixed(1),
      gpuTotalMs: +gpuTotal.toFixed(2),
      gpu,
      ...counters,
    });
  }

  /**
   * Renders with the character genuinely moving.
   *
   * A pinned-pose benchmark cannot see temporal artefacts, because a static
   * camera is exactly the condition under which reprojection and accumulation
   * work perfectly. Anything to do with motion has to be judged like this.
   */
  /**
   * Renders frames with nothing moving.
   *
   * renderMotion walks the character in a 1.8m circle, which is right for
   * exercising reprojection but has silently invalidated several still-frame
   * measurements — anything comparing two captures needs the camera and the
   * subject to be where they were left.
   */
  async function renderStill(frames = 30, fixedDtMs = 0): Promise<string> {
    recordFrameTimes = false;
    if (fixedDtMs > 0) {
      // Deterministic stepping for scenario asserts: each frame is exactly
      // fixedDtMs of game time whatever the tracer cost, and the caller is
      // expected to hold __pause so the rAF loop is not stepping in between.
      let t = prev;
      for (let i = 0; i < frames; i++) { t += fixedDtMs; frameBody(frozenClock ?? t); }
    } else {
      for (let i = 0; i < frames; i++) frameBody(frozenClock ?? performance.now());
    }
    await ctx!.device.queue.onSubmittedWorkDone();
    recordFrameTimes = true;
    return `rendered ${frames} still frames`;
  }

  /**
   * Freezes or releases the game clock for every subsequent frame (rAF and
   * __renderStill alike). See `frozenClock`.
   */
  function freezeClock(on: boolean): string {
    frozenClock = on ? performance.now() : null;
    return on ? "clock frozen (dt = 0 every frame)" : "clock running";
  }

  /**
   * Motion-audit render: walks the player through a scripted move for `frames`
   * simulated frames.
   * @param turn  yaw rate of the walking direction in rad per simulated second;
   *              0 walks a straight line, the default a ~2.9 m circle at walk
   *              speed. The move is fed to the player as intent (see
   *              Player.scriptedMove), so gait, clip and speed are the game's
   *              own, and the simulated clock steps a fixed 1/fps regardless
   *              of how long a software-rendered frame really takes.
   */
  async function renderMotion(frames = 90, turn = 0.55, fps = 60): Promise<string> {
    recordFrameTimes = false;
    let ang = Math.atan2(player.velX, player.velZ);
    for (let i = 0; i < frames; i++) {
      ang += turn / fps;
      player.scriptedMove = { x: Math.sin(ang), z: Math.cos(ang) };
      frameBody(prev + 1000 / fps);
    }
    player.scriptedMove = null;
    await ctx!.device.queue.onSubmittedWorkDone();
    recordFrameTimes = true;
    return `rendered ${frames} frames with the character in motion`;
  }

  /**
   * Screen-space rects of the player's boxes at the current internal
   * resolution, one per box, in reprojection's pixel convention (y down,
   * ndc.z = 1 near).
   *
   * Per-box rather than one bounding rect so the union hugs the silhouette: the
   * temporal-history probe wants character pixels, not the background that a
   * single AABB around a walking figure is mostly made of.
   */
  function playerScreenRects(): Array<{ x0: number; y0: number; x1: number; y1: number }> {
    const w = renderer.renderWidth;
    const h = renderer.renderHeight;
    const m = camera.viewProj;
    const rects: Array<{ x0: number; y0: number; x1: number; y1: number }> = [];
    for (let b = 0; b < playerBoxCount; b++) {
      const o = b * BOX_STRIDE_F32;
      const cx = dynBoxes[o], cy = dynBoxes[o + 1], cz = dynBoxes[o + 2];
      let x0 = Infinity, y0 = Infinity, x1 = -Infinity, y1 = -Infinity;
      for (let corner = 0; corner < 8; corner++) {
        let px = cx, py = cy, pz = cz;
        for (let a = 0; a < 3; a++) {
          const s = ((corner >> a) & 1) === 0 ? -1 : 1;
          const half = dynBoxes[o + 4 + a] * s;
          const r = o + 8 + a * 4;
          px += dynBoxes[r] * half;
          py += dynBoxes[r + 1] * half;
          pz += dynBoxes[r + 2] * half;
        }
        const cw = m[3] * px + m[7] * py + m[11] * pz + m[15];
        if (cw <= 1e-6) continue;
        const nx = (m[0] * px + m[4] * py + m[8] * pz + m[12]) / cw;
        const ny = (m[1] * px + m[5] * py + m[9] * pz + m[13]) / cw;
        const sx = (nx * 0.5 + 0.5) * w;
        const sy = (0.5 - ny * 0.5) * h;
        x0 = Math.min(x0, sx); y0 = Math.min(y0, sy);
        x1 = Math.max(x1, sx); y1 = Math.max(y1, sy);
      }
      if (!Number.isFinite(x0)) continue;
      rects.push({
        x0: Math.max(0, Math.floor(x0)), y0: Math.max(0, Math.floor(y0)),
        x1: Math.min(w, Math.ceil(x1)), y1: Math.min(h, Math.ceil(y1)),
      });
    }
    return rects;
  }

  type Rect = { x0: number; y0: number; x1: number; y1: number };
  /**
   * Which pixels a history statistic covers. `cls` selects by the G-buffer's
   * surface class (the animated geometry's own texels versus the static
   * background), `rects` further clips to screen rectangles, and `age` states
   * how many frames the selected subject could legitimately have accumulated
   * — a pixel reading longer than `age` has taken history from something else.
   */
  interface HistoryProbe {
    rects?: Rect[] | null;
    cls?: "dynamic" | "static" | null;
    age?: number;
  }

  const CLS_MISS = 0, CLS_STATIC = 1, CLS_DYNAMIC = 2;
  async function readHistoryMasked(p: HistoryProbe = {}) {
    const { width, height, data } = await renderer.readMoments();
    const inMask = new Uint8Array(width * height);
    for (const r of p.rects ?? [{ x0: 0, y0: 0, x1: width, y1: height }]) {
      const x0 = Math.max(0, Math.floor(r.x0)), y0 = Math.max(0, Math.floor(r.y0));
      const x1 = Math.min(width, Math.ceil(r.x1)), y1 = Math.min(height, Math.ceil(r.y1));
      for (let y = y0; y < y1; y++) inMask.fill(1, y * width + x0, y * width + x1);
    }
    let cls: Uint8Array | null = null;
    if (p.cls) {
      const pos = await renderer.readPos();
      cls = new Uint8Array(width * height);
      for (let i = 0; i < width * height; i++) {
        const w = pos.data[i * 4 + 3];
        cls[i] = w > 1.5 ? CLS_DYNAMIC : w > 0.5 ? CLS_STATIC : CLS_MISS;
      }
    }
    const want = p.cls === "dynamic" ? CLS_DYNAMIC : CLS_STATIC;
    return { width, height, data, inMask, cls, want };
  }

  /**
   * Temporal history-length statistics over the selected pixels, from the
   * direct-signal moments the last frame wrote — the number behind the
   * variance/history debug view. With `age` given, `fracOver` counts pixels
   * carrying more history than the subject can own: reprojection reusing
   * some other surface's accumulation, which the length distribution alone
   * cannot tell from history correctly kept.
   */
  async function historyStats(p: HistoryProbe = {}): Promise<Record<string, number>> {
    const { width, height, data, inMask, cls, want } = await readHistoryMasked(p);
    const lens: number[] = [];
    let sum = 0;
    for (let i = 0; i < width * height; i++) {
      if (!inMask[i]) continue;
      if (cls && cls[i] !== want) continue;
      const len = data[i * 4 + 2];
      if (!Number.isFinite(len)) continue;
      lens.push(len);
      sum += len;
    }
    const n = lens.length;
    if (n === 0) return { pixels: 0 };
    lens.sort((a, b) => a - b);
    const at = (q: number) => lens[Math.min(n - 1, Math.floor(q * n))];
    let short = 0, fresh = 0, over = 0;
    const cap = p.age !== undefined ? p.age + 1.5 : Infinity;
    for (const l of lens) { if (l < 4) short++; if (l <= 1.5) fresh++; if (l > cap) over++; }
    const out: Record<string, number> = {
      pixels: n,
      mean: +(sum / n).toFixed(2),
      p10: +at(0.1).toFixed(2),
      median: +at(0.5).toFixed(2),
      p90: +at(0.9).toFixed(2),
      max: +at(1).toFixed(2),
      fracUnder4: +(short / n).toFixed(3),
      fracReset: +(fresh / n).toFixed(3),
    };
    if (p.age !== undefined) out.fracOver = +(over / n).toFixed(3);
    return out;
  }

  /**
   * The per-pixel map `historyStats` aggregates over one rect, row-major:
   * history length plus surface class (0 miss / 1 static / 2 dynamic), for
   * when the shape of a defect matters — a limb-shaped hole and a seam that
   * has soaked up floor history read very differently at the same statistic.
   */
  async function historyRect(r: Rect): Promise<{ w: number; h: number; z: number[]; cls: number[] }> {
    const { width, height, data } = await renderer.readMoments();
    const pos = await renderer.readPos();
    const x0 = Math.max(0, Math.floor(r.x0)), y0 = Math.max(0, Math.floor(r.y0));
    const x1 = Math.min(width, Math.ceil(r.x1)), y1 = Math.min(height, Math.ceil(r.y1));
    const z: number[] = [];
    const cls: number[] = [];
    for (let y = y0; y < y1; y++) {
      for (let x = x0; x < x1; x++) {
        z.push(data[(y * width + x) * 4 + 2]);
        const w = pos.data[(y * width + x) * 4 + 3];
        cls.push(w > 1.5 ? CLS_DYNAMIC : w > 0.5 ? CLS_STATIC : CLS_MISS);
      }
    }
    return { w: Math.max(0, x1 - x0), h: Math.max(0, y1 - y0), z, cls };
  }

  Object.assign(window as object, {
    __renderMotion: renderMotion,
    __renderStill: renderStill,
    __freezeClock: freezeClock,
    __historyRect: historyRect,
    __historyStats: historyStats,
    __playerScreenRects: playerScreenRects,
    /** Camera-invisible player boxes (shadows and box indices kept). */
    __hidePlayer: (hidden: boolean) => { hidePlayerFromCamera = hidden; },
    /** The pinned, weapon-drawn pose the benches use — a lit, known spot. */
    __pinBenchPose: pinBenchPose,
    // A scripted probe drives frameBody itself and must stop the rAF loop
    // interleaving free frames of its own: silent static frames between
    // scripted motion frames rebuild exactly the history a motion audit is
    // trying to catch being lost.
    __benchGuard: benchGuard,
    __stats: stats,
    __settings: settings,
    __resize: resize,
    /**
     * Pins the internal render resolution for a scenario; `null` releases it and
     * hands the size back to the canvas. Returns what is now pinned.
     *
     * A half-specified pin throws rather than quietly releasing: the whole point
     * of the hook is that a scenario's resolution cannot change without it
     * knowing, and `__pinResolution(384)` falling back to the canvas would be
     * that same silent change with an easier trigger.
     */
    __pinResolution: (w: number | null, h?: number) => {
      if (w === null || w === 0) {
        pinnedRes = null;
        resize();
      } else {
        if (!h) throw new Error("__pinResolution needs a height (pass null to release)");
        pinnedRes = { w: Math.max(1, Math.floor(w)), h: Math.max(1, Math.floor(h)) };
        renderer.resize(pinnedRes.w, pinnedRes.h);
      }
      return `${renderer.renderWidth}x${renderer.renderHeight}`;
    },
    __renderer: renderer,
    __player: player,
    __guards: guards,
    __detection: detection,
    __nav: nav,
    __restart: () => restart(),
    /** Hold the live loop off while a scenario steps the world itself. */
    __pause: (on: boolean) => { paused = on; },
    __equipment: equipment,
    __particles: particles,
    __raycaster: raycaster,
    __input: input,
    __scene: scene,
    __flashes: flashes,
    __visibility: visibility,
    __camera: camera,
    __flash: flash,
    __bench: bench,
    __benchResolution: benchResolution,
    __readFlashmap: (layer = 0) => renderer.readFlashmapLayer(layer),
    __readRadiosity: () => renderer.readRadiosity(),
    __compareToReference: compareToReference,
    __frameTimer: frameTimer,
    __adaptive: adaptive,
    // Escape hatch. Stored settings now survive a reload, so a value that
    // makes the game unusable would otherwise survive with them.
    __resetSettings: () => { resetSettings(); location.reload(); },
    __persister: persister,
    __calibrate: () => brightness.open(),
    // Gameplay's view of the smoke: coarse, a few frames behind, CPU-side.
    __sampleSmokeDensity: (x: number, y: number, z: number) =>
      renderer.sampleSmokeDensityCPU(x, y, z),
    // Fluid simulation + sources: the solver (tuning, scale, readbacks), the
    // source list, the canister world, a scripted throw for scenarios, an
    // instant density blob (the old __smokeTest, now carried by the sim), and
    // the physics module's self-test.
    __fluid: renderer.fluid,
    __smoke: smoke,
    __canisters: canisters,
    // Optional `from` releases from a fixed world point instead of the posed
    // weapon hand — a determinism check must not depend on animation state.
    __throwCanister: (tx: number, tz: number, from?: [number, number, number]) =>
      canisters.throw(
        from ? v3(from[0], from[1], from[2]) : player.muzzle().pos,
        v3(tx, 0.12, tz),
      ),
    __smokePuff: (x: number, y: number, z: number, r: number, amount: number) =>
      smoke.puff(x, y, z, r, amount),
    __physicsSelfTest: () => physicsSelfTest(),
  });

  /**
   * One key, three meanings, resolved by what is in reach: drop what you are
   * carrying, pick up a body, or take a guard down. A player never has to
   * choose between them because only one is ever possible at a time.
   *
   */
  function interact(): void {
    if (player.dead || ended) return;
    if (carried) {
      carried.carried = false;
      carried = null;
      player.carrying = false;
      return;
    }
    const live = guards.nearestLive(player.pos, Player.REACH);
    if (live && player.swing()) {
      // The kill lands with the swing rather than at the end of it: the
      // guard's knockback and the player's hook have to overlap for the two
      // clips to read as one exchange.
      live.kill(true);
      takedowns++;
      // Quiet, not silent: the body hits the floor.
      detection.noise(live.pos, "thud");
      return;
    }
    const body = guards.nearestBody(player.pos, Player.REACH);
    if (body) {
      body.carried = true;
      carried = body;
      player.carrying = true;
    }
  }

  /**
   * The whole level state back to its opening frame. Not a checkpoint: dead
   * guards stand back up, magazines refill, everyone forgets you.
   */
  function restart(): void {
    player.reset(level.spawn);
    guards.reset();
    detection.reset();
    smoke.reset();
    canisters.reset();
    renderer.fluid.reset();
    if (carried) { carried.carried = false; carried = null; }
    takedowns = 0;
    ended = null;
    seenPulse = 0;
    endCard.hide();
    equipment.select(1);
    // The lamps you shot out and the OCP's clock are level state too: a second
    // attempt in a darker room is a different level, not a restart.
    equipment.reset(scene.lights, scene.materials, (idx, intensity, mat, emissive) => {
      renderer.setStaticLightIntensity(idx, intensity);
      if (mat >= 0 && emissive) {
        renderer.setMaterialEmissive(mat, emissive[0], emissive[1], emissive[2]);
      }
    });
  }

  /**
   * Transmittance of the smoke over the player's head: exp of the density
   * integral through a 2.6 m column above the chest probe, read off the
   * coarse CPU smoke field the guards' line-of-sight also uses. 1 in clear
   * air; a canister cloud drops it to a fraction and the gauge with it.
   */
  function smokeTransmittanceAbove(probeH: number): number {
    const y0 = player.pos.y + probeH;
    const taps = 6;
    const seg = 2.6 / taps;
    let od = 0;
    for (let i = 0; i < taps; i++) {
      od += renderer.sampleSmokeDensityCPU(player.pos.x, y0 + (i + 0.5) * seg, player.pos.z);
    }
    return Math.exp(-settings.volumetric * od * seg);
  }

  /** Two ways to finish: everyone down, or slip out the east end untouched. */
  function checkOutcome(): void {
    if (ended || player.dead) return;
    const allDown = guards.all.every((g) => g.dead);
    const exfil =
      player.pos.x >= EXFIL.minX && player.pos.x <= EXFIL.maxX &&
      player.pos.z >= EXFIL.minZ && player.pos.z <= EXFIL.maxZ;
    if (!allDown && !exfil) return;
    ended = "win";
    // GHOST: nobody ever knew you were there. CLEAN: they knew, and you got
    // out anyway.
    const rating = detection.everAlerted ? "CLEAN" : "GHOST";
    const how = allDown
      ? `${guards.count} DOWN / ${takedowns} SILENT`
      : "EXFILTRATED";
    endCard.show(rating, `${how}  —  R TO RUN IT AGAIN`, true);
  }

  function frameBody(now: number): void {
    const rawMs = now - prev;
    const dt = Math.min(rawMs / 1000, 0.05);
    prev = now;
    elapsed += dt;
    stats.frames++;
    stats.lastFrameMs = rawMs;
    if (recordFrameTimes && rawMs > 0 && rawMs < 500) frameTimer.push(rawMs);
    stats.avgFrameMs = stats.avgFrameMs === 0 ? rawMs : stats.avgFrameMs * 0.9 + rawMs * 0.1;

    // Never while a benchmark drives frameBody directly: the deferred resize
    // would override the pinned internal resolution mid-run, and the numbers
    // would silently be for however many pixels the window happened to have.
    if (!benchGuard.active) {
      if (pinnedRes) {
        // Same hazard, scenario flavour. resize() is a no-op at the same size.
        lastResize = 0;
        renderer.resize(pinnedRes.w, pinnedRes.h);
      } else if ((lastResize && now - lastResize > 120) || renderer.renderWidth < 2) {
        lastResize = 0;
        resize();
      }
    }

    // ---- input -----------------------------------------------------------
    // The number keys select equipment; quality presets live in the debug
    // panel, which is where a rendering setting belongs anyway.
    for (let i = 0; i < SLOTS.length; i++) {
      if (input.pressed(`Digit${i + 1}`)) equipment.select(i);
    }
    player.weaponLive = equipment.slot === "pistol";
    // The OCP is a pistol attachment, so it counts as drawn; empty hands do
    // not. The canister slot is drawn too: the arms-up aim pose is the throw
    // stance, so the character telegraphs the throw at the cursor.
    player.weaponDrawn = equipment.slot !== "none";
    // Both hands are on the body while dragging, so nothing can be held.
    if (player.carrying) equipment.select(0);
    if (input.pressed("KeyG")) {
      settings.debugView = (settings.debugView + 1) % DEBUG_VIEWS.length;
      panel.refresh();
    }
    if (input.pressed("KeyN")) {
      settings.nightVision = !settings.nightVision;
      panel.refresh();
    }
    if (input.pressed("Backquote")) panel.toggleVisible();
    // R restarts once the run has an ending; the reload it also maps to is
    // inert then (dead hands, or a full magazine after the reset).
    if ((ended || player.dead) && input.pressed("KeyR")) restart();

    // ---- simulate --------------------------------------------------------
    player.update(dt, input, camera, canvas.width, canvas.height);

    // Bias the camera slightly toward where the player is aiming — it lets you
    // see a little further into the direction you care about.
    const focus = v3(
      player.pos.x + (player.aimPoint.x - player.pos.x) * 0.16,
      player.pos.y + 1.0,
      player.pos.z + (player.aimPoint.z - player.pos.z) * 0.16,
    );
    camera.update(dt, focus, canvas.width / canvas.height);

    // Order matters: the rig is shared mutable state, and computeWorld writes
    // into it, so a pose is only valid until the next character updates. Pack
    // the player before touching the guards or it renders wearing a guard pose.
    const { data, count } = player.buildBoxes(playerMats);
    if (count !== DYN_GROUP_SIZE && !groupSizeWarned) {
      groupSizeWarned = true;
      console.error(
        `[dyn] a character packs ${count} boxes but DYN_GROUP_SIZE is ` +
          `${DYN_GROUP_SIZE}. Groups no longer line up with characters: the ` +
          `group AABBs will reject poorly, and the light probe will only skip ` +
          `part of its own body and read as permanently shadowed. Update ` +
          `DYN_GROUP_SIZE in renderer.ts and common.wgsl.`,
      );
    }
    dynBoxes.set(data.subarray(0, count * BOX_STRIDE_F32), 0);
    playerBoxCount = count;
    if (hidePlayerFromCamera) {
      // Absent for camera rays only: the character keeps its shadows and its
      // box indices, so a probe can make it *appear* into an already-settled
      // scene without changing anything else the pixels see.
      for (let b = 0; b < count; b++) dynBoxFlags[b * BOX_STRIDE_F32 + 7] |= FLAG_NO_CAMERA;
    }
    guards.update(dt);
    // The brain reads this frame's poses and steers next frame's bodies.
    // Perception is on its own 15 Hz cadence inside; this call is cheap. A won
    // run stops the hunt: guards firing at a player already reading GHOST would
    // be the game arguing with its own verdict.
    if (ended !== "win") detection.update(dt, player, visibility.level);
    if (detection.playerHit && !ended) {
      player.kill();
      ended = "dead";
      endCard.show("COMPROMISED", "PRESS R TO RESTART", false);
    }
    checkOutcome();
    const guardBoxes = guards.buildBoxes(dynBoxes, count, guardMats);
    // Particles and canisters pack last: the renderer treats everything from
    // this index on as identity-unstable (see Renderer.updateDynamic).
    const particleStart = count + guardBoxes;
    const particleBoxes = particles.buildBoxes(dynBoxes, particleStart);
    const canisterBoxes = canisters.buildBoxes(
      dynBoxes, particleStart + particleBoxes, canisterMat,
    );
    renderer.updateDynamic(dynBoxes, particleStart + particleBoxes + canisterBoxes, particleStart);
    // Torch depth maps skip their owner's body. The player is group 0 (the
    // same assumption setProbes makes); guards fill groups 1.. in pack order.
    renderer.setTorchGroups(0, guards.torchGroups(1));
    // Measure how lit the player actually is. Chest height, because that is
    // the body a guard's eye lands on - and the chest drops with the stance
    // (crouch or dragging a body), so the meter and the line-of-sight test
    // agree about which body is there.
    const probeH = player.crouching || player.carrying ? detectionTuning.probeCrouch : detectionTuning.probeStand;
    renderer.setProbes([v3(player.pos.x, player.pos.y + probeH, player.pos.z)]);
    // Inside smoke you are dimmer than the probe says: its shadow rays test
    // geometry, not the medium (see the fluid track report), so attenuate
    // by the density integral over the column the room's light arrives
    // through — the fixtures overhead and the moon's descending shafts.
    visibility.update(renderer.probeLuma[0] * smokeTransmittanceAbove(probeH), dt);
    gauge.update(visibility.level, visibility.band);
    const detect = detection.summary();
    detectMeter.update(detect.level, detect.label);
    // Eased so a one-tick LOS flicker does not strobe the frame.
    seenPulse += ((detect.seen ? 1 : 0) - seenPulse) * (1 - Math.exp(-dt / 0.15));
    ammo.update(player.rounds, player.spares, player.reloading);

    equipment.update(
      dt,
      (i, intensity, mat, emissive) => {
        renderer.setStaticLightIntensity(i, intensity);
        if (mat >= 0 && emissive) {
          renderer.setMaterialEmissive(mat, emissive[0], emissive[1], emissive[2]);
        }
      },
      (at, burst) => particles.sparks(at, burst ? 14 : 3),
    );
    particles.update(dt);
    canisters.update(dt);
    smoke.update(dt, settings.fluidSim);
    equipBar.update(
      equipment.active, [1, 1, equipment.ocpCharge, 1], player.flashlightOn,
      [null, null, null, equipment.canisters],
    );

    // ---- takedown and body carrying ---------------------------------------
    if (input.pressed("KeyE")) interact();
    if (carried) {
      // Dragged along the floor, not carried. The body trails at arm's length
      // in front, the player faces it, and moving away from it is therefore a
      // backward walk — which is what dragging a body actually is, and which
      // keeps the body on the ground where it cannot float or swing through a
      // wall the way a shouldered one did.
      const dx = carried.pos.x - player.pos.x;
      const dz = carried.pos.z - player.pos.z;
      const d = Math.hypot(dx, dz);
      if (d > 1e-4) player.dragYaw = Math.atan2(dx, dz);

      // The body is pulled along rather than pinned: it only moves once the
      // player has stretched past the drag distance, so it slides in fits the
      // way something heavy on carpet does.
      const DRAG = 0.85;
      if (d > DRAG) {
        const pull = (d - DRAG) / d;
        carried.pos.x -= dx * pull;
        carried.pos.z -= dz * pull;
      }
      carried.pos.y = 0;
      // Feet-first, so it trails in line with the direction of travel.
      carried.yaw = lerpAngle(carried.yaw, player.dragYaw, 1 - Math.exp(-8 * dt));
    }

    /**
     * The ray the camera casts through the cursor, normalised.
     *
     * Rebuilt per shot rather than cached: it depends on the camera, which
     * moves every frame, and on the mouse.
     */
    const cursorRay = (): { x: number; y: number; z: number } => {
      const r = camera.screenRay(input.mouseX, input.mouseY, canvas.width, canvas.height);
      const inv = 1 / Math.max(Math.hypot(r.x, r.y, r.z), 1e-6);
      return v3(r.x * inv, r.y * inv, r.z * inv);
    };

    // The canister is thrown at the ground point under the cursor, released
    // from the weapon hand the aim pose has already raised toward it.
    if (equipment.slot === "smoke" && input.pressed("Mouse0") && !player.dead) {
      if (equipment.useCanister()) {
        canisters.throw(
          player.muzzle().pos,
          camera.screenToGround(input.mouseX, input.mouseY, canvas.width, canvas.height, 0.12),
        );
      }
    }

    // The OCP shares the trigger; only the pistol consumes ammunition.
    if (equipment.slot === "ocp" && input.pressed("Mouse0") && equipment.ocpReady) {
      const m = player.muzzle();
      const idx = equipment.fireOCP(
        scene.lights, scene.lights.length, m.pos,
        camera.pos, cursorRay(), scene.materials,
        (at) => raycaster.blocked(m.pos, at, FIXTURE_LOS_MARGIN),
      );
      if (idx !== null) {
        renderer.setStaticLightIntensity(idx, 0);
        // Darken the fixture too, or an exit sign stays green while casting
        // nothing — which reads as a rendering bug rather than a dead light.
        const mat = equipment.matFor(idx);
        if (mat >= 0) renderer.setMaterialEmissive(mat, 0, 0, 0);
      }
    }

    if (player.justFired) {
      const m = player.muzzle();
      flashes.spawn(m.pos);
      // Everyone still standing hears it. This is the only thing that makes a
      // gunshot cost anything: without it the loud option and the silent one
      // are the same option.
      detection.noise(m.pos, "gunshot");
      // The flash comes off the barrel, but the bullet follows the cursor: the
      // weapon sits 14-15 degrees off the aim while moving, because the pistol
      // clip is authored in a bladed stance and the offset lives in the pelvis,
      // which cannot join the aim mask. Shooting down the barrel would mean
      // visibly missing what you are pointing at.
      const ax = player.aimPoint.x - m.pos.x;
      const az = player.aimPoint.z - m.pos.z;
      // Slightly downward from the muzzle toward torso height at range, so a
      // shot does not sail over a guard standing a few metres away.
      const ay = 1.15 - m.pos.y;
      const inv = 1 / Math.max(Math.hypot(ax, ay, az), 1e-4);
      const dir = v3(ax * inv, ay * inv, az * inv);
      // Trace the world first, then look for guards only in front of whatever
      // it struck. Cleaner than testing every guard and asking afterwards
      // whether a wall was in the way, and it gives the wall impact its point
      // and normal for free.
      const world = raycaster.raycast(m.pos, dir, 45);
      const reach = world ? world.t : 45;
      const hit = guards.hitScan(m.pos, dir, reach);
      if (hit) {
        hit.kill();
        // Spray back along the shot, so it reads as an exit from the body.
        particles.impact(
          v3(m.pos.x + dir.x * 1.0, 1.15, m.pos.z + dir.z * 1.0),
          v3(-dir.x, 0.35, -dir.z),
        );
      } else {
        // Nothing living in the way, so ask whether the player was pointing at
        // a fixture. The level shot above cannot answer that: it travels at
        // torso height and lands on a wall, nowhere near a lamp. See
        // equipment.shootOut.
        //
        const shot = equipment.shootOut(
          scene.lights, scene.lights.length, m.pos,
          camera.pos, cursorRay(),
          (at) => raycaster.blocked(m.pos, at, FIXTURE_LOS_MARGIN),
          // The dead fixture smoulders — a thin warm plume for ~20 s.
          (at) => smoke.smolder(at),
        );
        if (shot) {
          renderer.setStaticLightIntensity(shot.index, 0);
          if (shot.mat >= 0) renderer.setMaterialEmissive(shot.mat, 0, 0, 0);
          particles.sparks(scene.lights[shot.index].pos, 16);
        } else if (world) {
          const at = v3(
            m.pos.x + dir.x * world.t, m.pos.y + dir.y * world.t,
            m.pos.z + dir.z * world.t,
          );
          particles.debris(at, world.normal);
          // Pull the dust cloud slightly off the surface so its whole volume
          // sits in air the solver can move, thrown back along the normal.
          smoke.impact(v3(
            at.x + world.normal.x * 0.3,
            at.y + world.normal.y * 0.3,
            at.z + world.normal.z * 0.3,
          ), world.normal);
        }
      }
      smoke.muzzle(m.pos, dir);
    }
    for (const g of guards.all) {
      if (!g.justFired) continue;
      const gm = g.muzzle();
      flashes.spawn(gm.pos);
      smoke.muzzle(gm.pos, gm.dir);
    }
    flashes.update(dt);
    // Steady and transient are handed over separately; the renderer owns the
    // ordering the shader's index split depends on.
    renderer.updateLights(guards.lights(), flashes.lights());

    const flashOn = player.flashlightOn && !player.dead;
    renderer.render(
      {
        invViewProj: camera.invViewProj,
        prevViewProj: camera.prevViewProj,
        camPos: camera.pos,
        prevCamPos: camera.prevPos,
        flashPos: player.flashlightOrigin(),
        flashDir: player.flashlightDir(),
        flashColor: v3(flash.colorR, flash.colorG, flash.colorB),
        flashIntensity: flashOn ? flash.intensity : 0,
        seenPulse,
        flashRadius: flash.radius,
        flashCosInner: Math.cos((flash.innerDeg * Math.PI) / 180),
        flashCosOuter: Math.cos((flash.outerDeg * Math.PI) / 180),
        time: elapsed,
        mouseX: input.mouseX,
        mouseY: input.mouseY,
        dt,
        smokeSources: smoke.packed,
        smokeSourceCount: smoke.count,
      },
      settings,
    );

    input.endFrame();

    // ---- hud -------------------------------------------------------------
    updateAdaptiveResolution(dt);
    // After the adaptive controller, so a scale it just settled on is the one
    // that gets written rather than the value from the frame before.
    persister.poll(settings, now);

    fpsAccum += dt;
    // The overlay is a developer tool, off unless asked for. The light gauge
    // above is the only UI the game itself needs.
    if (showStats && fpsAccum >= 0.25) {
      fpsAccum = 0;

      const t = renderer.profiler.timings;
      const traced = renderer.profiler.total() - (t.get("post") ?? 0);
      const parts: string[] = [];
      // The final render pass absorbs swapchain acquire wait, so its timestamp
      // is not work we can optimise. Flag it rather than silently mislead.
      for (const [k, v] of t) {
        const note = k === "post" ? "  (incl. present wait)" : "";
        parts.push(`  ${k.padEnd(11)}${v.toFixed(2)}ms${note}`);
      }

      hud.textContent = [
        frameTimer.report(renderer.profiler.enabled ? traced : undefined),
        "",
        `${renderer.renderWidth}x${renderer.renderHeight} -> ${canvas.width}x${canvas.height}` +
          `  x${settings.resolutionScale.toFixed(2)}${adaptive.enabled ? " auto" : ""}`,
        `${qualityName}  ${settings.spp}spp  ${settings.bounces}b`,
        renderer.profiler.enabled
          ? `gpu ${traced.toFixed(2)}ms excl. present`
          : "gpu timing unavailable",
        ...parts,
        `visibility ${visibility.meter()} ${visibility.band}` +
          `  (${visibility.illuminance.toFixed(4)} lx)`,
        `detection ${detect.label}  ${detect.level.toFixed(2)}`,
        settings.debugView > 0 ? `debug: ${DEBUG_VIEWS[settings.debugView]}` : "",
      ].filter(Boolean).join("\n");
    }
  }

  requestAnimationFrame(frame);
}

void main();
