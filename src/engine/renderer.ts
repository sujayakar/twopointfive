import { Mat4, Vec3 } from "../core/math";
import { BVH } from "../scene/bvh";
import {
  BOX_STRIDE_F32, LIGHT_STRIDE_F32, MATERIAL_STRIDE_F32, Light, Material, SceneBuilder,
  packBoxes, packLights, packMaterials,
} from "../scene/scene";
import { buildPatches } from "../scene/radiosity";
import { halfToFloat } from "../core/half";
import { WorkCounters } from "./counters";
import {
  FluidSim, OccupancyBaker, FLUID_SOURCE_STRIDE, FLUID_MAX_SOURCES,
} from "./fluid";
import { GPUContext } from "./gpu";
import { GpuProfiler } from "./profiler";
import { SHADERS, createShaderModule } from "./shaders";

/**
 * Boxes per coarse dynamic group; must match DYN_GROUP_SIZE in common.wgsl.
 *
 * This is one character's box count, so each group is exactly one character.
 * That alignment is load-bearing twice over: the group AABB only rejects well
 * if a group is one spatially-compact object, and the light probe skips its own
 * character by group index, which silently under-skips if a character spans two
 * groups. main.ts asserts the player's actual box count against this.
 */
export const DYN_GROUP_SIZE = 26;
const DYN_GROUPS = 8;
/** Sentinel matching DYN_GROUP_NONE in common.wgsl: skip no dynamic group. */
const DYN_GROUP_NONE = 0xffffffff;
// 304 bytes of scalars plus two arrays of DYN_GROUPS vec4f for the group bounds.
// After the dyn-group arrays: 16 bytes of restir/flashmap scalars, 16 bytes of
// fog scalars, then bytes 592-847: the retired smoke-puff arrays, dead now
// that the fluid simulation carries all smoke, kept only so every later field
// keeps its offset. That hole is held open by `_deadPuffPosR` and
// `_deadPuffParams` (sized by `MAX_PUFFS = 8u`) in common.wgsl; there is no
// longer a TS-side constant, so grep those names before touching the layout.
// Then the trailing vec4 (radiosity flag, reservoir parity, counters flag)
// ending at byte 864. Bytes 864-879 are the radiosity-hybrid block
// (indirectMode u32 at 864, radPatchCount u32 at 868, two pads), 880-943
// the volumetric block, and 944-959 the previous camera position (three f32
// plus a pad), so the buffer ends at 960.
/** Byte offset of the volumetric block within the uniform buffer. */
const UNIFORM_VOL_OFFSET = 880;
// 992: the fine-lattice block is appended at byte 960 so that every existing
// offset — and every shader that reads one — is untouched.
const UNIFORM_SIZE = 992;
/** Bytes per ReSTIR reservoir; must match the WGSL struct. */
const RESERVOIR_BYTES = 32;
/** Bytes per ReSTIR GI reservoir; four vec3f/f32 pairs. */
const GI_RESERVOIR_BYTES = 64;
/**
 * Upper bound on animated boxes. Traced linearly, so it is a real per-ray cost.
 * A Character is 26 boxes, so the player plus four guards is 130; the rest is
 * headroom for particles. DYN_GROUPS * DYN_GROUP_SIZE caps it at 208.
 */
const MAX_DYN_BOXES = 208;
/**
 * Headroom in the light buffer for lights that move (guard torches). Static
 * lights occupy the head of the buffer and are uploaded once; these follow.
 */
const MAX_DYN_LIGHTS = 16;
const ATROUS_ITERS = 4;
/** Layers of the trace pass's radiance array: direct, indirect, transient, volume. */
const RAW_ILLUM_LAYERS = 4;
/** Torch depth map resolution — must match FLASHMAP_RES in common.wgsl. */
const FLASHMAP_RES = 128;
/** Depth-map layers (player + guard torches) — must match common.wgsl. */
const TORCH_LAYERS = 8;
const ATROUS_STRIDE = 256; // dynamic uniform offset alignment
/** Bytes of AtrousParams actually bound; must match the struct in atrous.wgsl. */
const ATROUS_PARAM_SIZE = 32;
/**
 * Reproject parameter slots: [0] direct, [1] indirect, [2] direct reference,
 * [3] indirect reference, [4] volume, [5] volume reference. The reference
 * slots disable every heuristic — firefly ceiling, history clamp, history
 * cap, alpha floor — so the pass degenerates to an honest 1/n running average.
 */
const REPROJECT_STRIDE = 256;
const REPROJECT_SLOTS = 6;
/** A-trous parameter slots: 4 direct, 4 indirect, 2 transient, then the volume chain. */
const TRANS_ATROUS_ITERS = 2;
/** Volume chain iterations — the medium is smooth, so it filters wide. */
const VOL_ATROUS_ITERS = 4;
const ATROUS_SLOTS = ATROUS_ITERS * 2 + TRANS_ATROUS_ITERS + VOL_ATROUS_ITERS;
const BLOOM_MIPS = 5;
/**
 * Smoke density grid — the interface Track B2b's fluid simulation fills, and
 * the single source of truth for the medium's box. Cubic 0.25 m cells over
 * the room's air: 208 x 13 x 144 (x, y, z) → 52 x 3.25 x 36 m. See the
 * contract at the top of the volumetric section in pathtrace.wgsl.
 *
 * Everything volumetric — the march AABB, the baked light volume, the CPU
 * readback, the debug filler — is derived from ORIGIN, CELL and DIMS below,
 * so a resize is these three constants plus the texture reallocation; the
 * uniforms carry origin + cell, dims come from the texture. Cells are cubic
 * (one scalar cell in the uniforms) and the box is dims x cell exactly.
 *
 * The box top (13 x 0.25 = 3.25) sits 5 cm above the ceiling underside
 * (3.2): 0.25 m cells cannot tile 3.2, so grid row 12 (y 3.0-3.25) straddles
 * the ceiling. B2b does NOT treat y >= 3.2 as solid, though the contract in
 * pathtrace.wgsl asks consumers to: its lattice top face at 3.25 is the wall
 * and row 12 stays fluid, because the row is 80% real air and solidifying it
 * would delete the layer ceiling-hugging smoke lives in. So up to 5 cm of
 * smoke can sit inside the invisible slab, and the camera march pays it
 * (~1.5% of a floor-to-ceiling column). See B2b-fluid.md's Below-100% list.
 */
export const MEDIUM_ORIGIN: [number, number, number] = [-26, 0, -18];

/**
 * The fine local lattice: 4 x 3.25 x 4 m at 3.125 cm.
 *
 * y spans the medium's full 3.25 m slab exactly (3.25 / 0.03125 = 104) rather
 * than being cubic, which is why the vertical never has to move and why floor
 * and ceiling stay real boundaries on both lattices. x and z are exact 8:1
 * refinements of the coarse 25 cm cell, so every resample between the two is
 * an integer operation and an fp16 copy is bit-exact.
 *
 * HALF THE REACH FOR TWICE THE DETAIL, and the trade is deliberate.
 *
 * This was 8 x 3.25 x 8 at 6.25 cm, sized so a canister cloud — 5.25 x 1.50 x
 * 6.50 m by t = 10 s — stayed inside it. But resolution, not extent, is what
 * decides whether smoke has structure: below roughly a hundred cells across a
 * source there is nothing for vorticity to curl and every scheme produces the
 * same lozenge. Every emitter in game/effects.ts was tuned on /demo/dynamics
 * at 3.125 cm, and several of them do not survive coarsening — a 0.06 m spark
 * trail is 1.9 cells here and 0.96 at 6.25 cm, which is below what the grid
 * can represent at all.
 *
 * 128 x 104 x 128 is 1.70 M cells against the old 852 k: exactly 2x, and it
 * still only steps while an event is anchored, so it stays an event tax rather
 * than a standing one. What leaves the smaller box is not lost — the coarse
 * lattice restricts the fine result into itself every step and advects it
 * onward at 25 cm, which is what a cloud four metres from its source should
 * look like anyway.
 */
export const FINE_CELL = 0.03125;
export const FINE_DIMS: [number, number, number] = [128, 104, 128];
export const SMOKE_CELL = 0.25;
export const SMOKE_DIMS: [number, number, number] = [208, 13, 144];

/**
 * Replaces the smoke lattice's geometry. Demo pages only.
 *
 * The rendered smoke is one 3D texture and the tracer takes its dimensions
 * from the texture and its cell size from a uniform, so the *image* is only
 * ever as detailed as this lattice — however fine the simulation underneath
 * runs. At the level's 25 cm a thrown grenade is a handful of voxels across,
 * which is why it reads as a blob no matter what the solver does.
 *
 * A showcase page does not need to cover a 52 x 36 m office. Spending the same
 * cell budget on a small box buys that resolution back directly.
 */
export interface SmokeLattice {
  dims: [number, number, number];
  origin: [number, number, number];
  cell: number;
}
export const MEDIUM_SIZE: [number, number, number] = [
  SMOKE_DIMS[0] * SMOKE_CELL, SMOKE_DIMS[1] * SMOKE_CELL, SMOKE_DIMS[2] * SMOKE_CELL,
];
/** Static-light volume: 0.5 m cells in x/z, 3.25/8 m in y, over the same box. */
/**
 * Radiance cascade 0. Must match CASCADE_DIRS / CASCADE_SPACING_CELLS in
 * common.wgsl — the trace and the shading lookup index the same texels, so a
 * disagreement is silent and wrong rather than a validation error.
 */
// 6, not 5. Five cascades reach w*(2^5 - 1) = 15.5 m, and the office is
// 52 x 36 m — light transported further than that was simply absent, which
// measured as a systematic energy deficit (relBias -0.401 against a 5-bounce
// reference, worse than either radiosity mode). A sixth reaches 31.5 m.
const CASCADE_LEVELS = 6;
/** Octahedral direction grid side at cascade 0; doubles per cascade. */
const CASCADE0_DIR_RES = 4;
const CASCADE_SPACING_CELLS = 2;

/** Per-cascade geometry, derived once — see the table in cascades.wgsl. */
interface CascadeLevel {
  res: number;
  probes: [number, number, number];
  spacing: [number, number, number];
  tStart: number;
  tEnd: number;
  dims: [number, number, number];
}

function cascadeLevels(): CascadeLevel[] {
  const w = SMOKE_CELL * CASCADE_SPACING_CELLS;
  const base: [number, number, number] = [
    Math.ceil(SMOKE_DIMS[0] / CASCADE_SPACING_CELLS),
    Math.ceil(SMOKE_DIMS[1] / CASCADE_SPACING_CELLS),
    Math.ceil(SMOKE_DIMS[2] / CASCADE_SPACING_CELLS),
  ];
  const out: CascadeLevel[] = [];
  for (let i = 0; i < CASCADE_LEVELS; i++) {
    const step = 1 << i;
    const res = CASCADE0_DIR_RES << i;
    // Coarsened in x/z only: the playable volume is a 3.25 m slab, so halving
    // y as well would be down to two layers by cascade 2.
    const probes: [number, number, number] = [
      Math.ceil(base[0] / step), base[1], Math.ceil(base[2] / step),
    ];
    out.push({
      res,
      probes,
      spacing: [w * step, w, w * step],
      // Intervals tile the line without overlap: [w(2^i - 1), w(2^(i+1) - 1)).
      tStart: w * (step - 1),
      tEnd: w * (step * 2 - 1),
      dims: [probes[0] * res, probes[1] * res, probes[2]],
    });
  }
  return out;
}
/** CascadeParams: vec3+u32, vec3u+pad, vec3+f32, f32+u32+2 pad = 64 B. */
const CASCADE_PARAM_BYTES = 64;

const LIGHT_VOL_DIMS: [number, number, number] = [104, 8, 72];
/**
 * Coarse CPU-side smoke density (guard line of sight): the smoke volume
 * box-averaged 4x in x and z, full resolution in y — 1 m cells over the room.
 * Read back a few frames behind, every SMOKE_READ_EVERY frames.
 */
const SMOKE_COARSE_FACTOR = 4;
const SMOKE_COARSE_DIMS: [number, number, number] = [
  SMOKE_DIMS[0] / SMOKE_COARSE_FACTOR, SMOKE_DIMS[1], SMOKE_DIMS[2] / SMOKE_COARSE_FACTOR,
];

/**
 * The readback grid for whatever lattice is actually in use.
 *
 * SMOKE_COARSE_DIMS above is the level's; a demo page that replaces the
 * lattice would otherwise dispatch a reduction whose destination is the wrong
 * size — silently, since the compute pass would simply write out of range.
 */
function coarseDimsFor(d: [number, number, number]): [number, number, number] {
  return [
    Math.max(1, Math.ceil(d[0] / SMOKE_COARSE_FACTOR)),
    d[1],
    Math.max(1, Math.ceil(d[2] / SMOKE_COARSE_FACTOR)),
  ];
}
const SMOKE_READ_EVERY = 4;
/** Gameplay light probes; must match MAX_PROBES in probe.wgsl. */
const MAX_PROBES = 4;
const WG = 8;
// Workgroup size was tuned by measurement at 1152x720 / 2 bounces / leaf<=1:
//   8x4  (32 threads)  16.30 ms
//   8x8  (64 threads)  13.12 ms  <- best
//   16x8 (128 threads) 13.73 ms
// 64 threads wins on Apple silicon here; do not "optimise" without re-measuring.

/**
 * How indirect light is estimated. See docs/campaign/tracks/B1-radiosity-hybrid.md.
 *   traced        — bounce rays only; the radiosity solve does not run.
 *   radiosityRead — static primary hits read the patch solve directly (no
 *                   bounce ray); dynamic hits keep tracing. The old behaviour.
 *   gather        — static primary hits trace bounce 1 for real and read the
 *                   solve at that vertex (final gather at x1): characters now
 *                   shadow the bounce, the solve is only ever a bounce away.
 *   patchRIS      — the patches are resampled as emitters at the primary hit,
 *                   one shadow ray to the survivor; serves dynamic hits too.
 *   cascades      — DEFAULT. Radiance cascades: a world-space directional
 *                   probe volume, traced fresh every frame and merged from a
 *                   coarse hierarchy. No patches, no form factors, no N^2
 *                   term, and no static/dynamic split, so dynamic geometry
 *                   receives and occludes bounce like anything else. Multi-
 *                   bounce comes from feeding last frame's probes back in.
 *                   See cascades.wgsl.
 */
/**
 * Live denoiser controls, for bisecting an indirect artefact by ablation:
 * is what you are seeing the signal, the temporal accumulator, or the blur?
 */
export interface DenoiseTuning {
  /**
   * A-trous passes that actually filter, per chain. The remaining passes still
   * run, at stride 0 — every tap of the 5x5 then lands on the centre pixel, so
   * the pass is an exact copy. Truncating the loop instead would leave the
   * result in whichever scratch texture the ping-pong happened to reach, and
   * the composite reads a fixed one.
   */
  atrousDirect: number;
  atrousIndirect: number;
  /** Frames of indirect history (SVGF alpha = 1/n until n is reached). */
  indHistory: number;
  /** Sigma multiplier and absolute widening of the indirect history clamp. */
  indClampK: number;
  indClampFloor: number;
  /** Floor under the temporal blend weight; 1 discards history outright. */
  indAlphaFloor: number;
}

/** The values the chain was hardcoded to before these became tunable. */
export const DEFAULT_DENOISE: DenoiseTuning = {
  atrousDirect: ATROUS_ITERS,
  atrousIndirect: ATROUS_ITERS,
  indHistory: 48,
  indClampK: 6,
  indClampFloor: 0.5,
  indAlphaFloor: 0.02,
};

/** Live radiance-cascade controls. Cascade count is not here yet — see cascades.wgsl. */
export interface CascadeTuning {
  /**
   * Multiplier on every cascade's ray interval. 1 is the tiling the hierarchy
   * is defined by; anything else double-counts or leaves gaps, so this is a
   * diagnostic for seeing how far the field reaches, not a setting.
   */
  reach: number;
  /** Temporal blend of this frame's trace, 1 = no history. */
  alpha: number;
  /**
   * RIS candidates per probe hit. The per-pixel path uses 8; a probe is
   * averaged over ~8 frames and then interpolated across its neighbours, so it
   * tolerates far more variance for far less work. See probeDirect.
   */
  candidates: number;
}

// candidates: 2, not 8. Measured in the office against a 5-bounce reference,
// 8 -> 1 moved relBias by 0.0015 (noise) and relRmse by 3.7%, while the trace
// went 9.91 -> 7.56 ms. 2 rather than 1 because at a single candidate RIS
// degenerates — there is nothing to resample against, so bright-light
// importance is lost entirely and the firefly clamp starts doing real work.
export const DEFAULT_CASCADES: CascadeTuning = { reach: 1, alpha: 0.12, candidates: 2 };

export type IndirectMode =
  | "traced" | "radiosityRead" | "gather" | "patchRIS" | "cascades";
/** Panel order; also the index the settings store round-trips. */
export const INDIRECT_MODES: IndirectMode[] =
  ["traced", "radiosityRead", "gather", "patchRIS", "cascades"];

export interface RenderSettings {
  /** Internal render resolution as a fraction of the canvas backing size. */
  resolutionScale: number;
  spp: number;
  bounces: number;
  /**
   * Density scale of the participating medium: extinction coefficient at unit
   * density, per metre. Scattering albedo is 1, so this is also what
   * scatters. 0 removes the medium.
   */
  volumetric: number;
  /** Henyey-Greenstein asymmetry for the medium, -1..1. */
  volPhaseG: number;
  /** Smoke self-shadowing strength against the baked static lights; 0 = off. */
  smokeShadow: number;
  /** The medium absorbs as well as scatters. Off keeps in-scatter only. */
  volExtinction: boolean;
  /**
   * Sub-grid smoke detail added at march time: modulation amplitude, 0 = off.
   *
   * The solver cannot resolve below its 0.25 m cell and its gather is smooth,
   * so a simulated cloud is a featureless lozenge in the transmittance view.
   * This draws the missing high frequencies instead of simulating them —
   * zero-mean and multiplicative, so it redistributes the cloud rather than
   * changing how much smoke there is.
   */
  smokeDetail: number;
  /**
   * Spatial frequency of that detail, 1/m.
   *
   * Bounded from ABOVE by `volumetricSteps`, not by the 0.25 m grid — this was
   * measured, and it is counter-intuitive. The march samples the medium at
   * jittered intervals, so any frequency finer than its step size averages to
   * the modulation's mean (which is 1) and vanishes. Raising this from 3.6 to
   * 9 at 16 steps made the cloud LOOK SMOOTHER, not more detailed. If you want
   * finer detail, raise volumetricSteps with it: 5/m reads well at 48 steps
   * and costs ~2.3 ms of pathtrace at 1152x720.
   */
  smokeDetailFreq: number;
  exposure: number;
  bloomIntensity: number;
  bloomThreshold: number;
  vignette: number;
  grain: number;
  chromatic: number;
  saturation: number;
  skyIntensity: number;
  /** Constant floor added to demodulated illumination, for readability. */
  ambient: number;
  /** Ray-march steps for the beam. Easily the most expensive single knob. */
  volumetricSteps: number;
  /**
   * Mean density of the ambient haze, 0..1 (the drifting noise is only its
   * texture). 0 removes the fog: no shafts, no veil — only smoke scatters.
   */
  fogAmount: number;
  /** Fresh ReSTIR candidates per pixel before reuse. */
  restirCandidates: number;
  /**
   * Temporal reservoir reuse across frames.
   *
   * Off by default since spatial taps landed: measured worse alone than
   * spatial-only (relRmse 1.01 vs 0.65, relBias +0.23 vs -0.02), and worse
   * still combined with taps, because borrowed temporally-fed streams carry
   * the temporal merge's W inflation into every neighbour that borrows them.
   * Kept as a toggle for A/B and for scenes where it might win again.
   */
  restirTemporal: boolean;
  /**
   * ReSTIR GI for the indirect bounce.
   *
   * Costs ~4% of the trace. An earlier reading of "5x" was a measurement
   * artefact: the rAF loop and the benchmark were both driving frameBody, so
   * work was double-counted non-deterministically.
   *
   * Validated against a converged reference (__compareToReference): relative
   * bias 0.181 vs 0.150 for ReSTIR DI alone, i.e. no detectable additional bias
   * from the reconnection shift. It is also not yet a measurable *win* at 1-2
   * bounces, where the denoiser dominates the residual error.
   */
  restirGI: boolean;
  /** Cap on reused M — lower stays responsive, higher converges further. */
  restirMCap: number;
  /**
   * Spatial reservoir taps per pixel (DI and GI both), 0 disables.
   *
   * Merges the PREVIOUS frame's reservoirs at jittered offsets around the
   * reprojected pixel — spatial reuse with one frame of propagation delay,
   * which needs no extra pass and no shading restructure. See the field docs
   * in common.wgsl for why prev-frame neighbours rather than same-frame.
   */
  restirSpatialTaps: number;
  /** Spatial tap offset radius in pixels. */
  restirSpatialRadius: number;
  /**
   * Flashlight depth-map visibility in the ReSTIR target function.
   *
   * Unbiased regardless of map quality — the survivor still gets a real
   * shadow ray; the map only steers which candidate survives.
   */
  flashVisTarget: boolean;
  /**
   * Volumetric march reads the flashlight depth map instead of firing a
   * shadow ray per step. An approximation (the map is 128^2 and biased by its
   * comparison slack), unlike flashVisTarget — which is why it is a separate
   * switch and is forced off in reference mode.
   */
  flashVisVolumetric: boolean;
  /**
   * Where indirect light comes from — the radiosity patch solve, traced
   * bounces, or a hybrid. See IndirectMode. The solve runs whenever a mode
   * other than "traced" is selected; reference mode forces "traced" so it
   * never validates the approximation against itself. Transient (muzzle
   * flash) bounce light is always traced per pixel — a warm-started patch
   * solve could only smear a 3-frame event.
   */
  indirectMode: IndirectMode;
  /**
   * Shadow rays per muzzle flash on the primary hit.
   *
   * The transient signal is never temporally accumulated, so this is its only
   * real variance control. Paid for a few frames per shot.
   */
  transientSamples: number;
  /**
   * How the transient (muzzle flash) signal is spatially filtered.
   *
   * 0 off, 1 widen, 2 glow. See AtrousParams.hintMode in atrous.wgsl — the
   * two are different approaches to the same problem and the choice between
   * them is a look, not a correctness question, so it is on the panel.
   */
  transientFilter: number;
  /** Distance at which a flash's lighting is filtered as hard as it goes. */
  transientBlurDist: number;
  /** How much bounced flash light counts toward that. */
  transientBounceWeight: number;
  /** Stride multiplier the hint may push the transient filter to. */
  transientBlurStride: number;
  /** Glow only: how much blur a full hint blends in. */
  transientBlurStrength: number;
  /**
   * Fraction of pixels tracing indirect per frame. 0.5 = tile checkerboard.
   *
   * Off by default. The saving scales with how much work sits behind the first
   * bounce, and measured at 976x553 it is only 0.21ms (2%) at 1 bounce versus
   * 2.79ms (19%) at 2 — while the cost in visible noise while moving is the
   * same either way. It is only worth enabling at higher bounce counts.
   */
  indirectRate: number;
  /**
   * Ground-truth accumulation mode.
   *
   * Turns off every estimator and filter that trades bias for speed — ReSTIR
   * reuse, the firefly clamps, the history clamp, the a-trous chain — leaving a
   * plain progressive average of brute-force path tracing. Only meaningful with
   * a frozen camera, and only useful for comparing against.
   */
  reference: boolean;
  /** Image-intensifier mode. Replaces the AgX/grade path entirely. */
  nightVision: boolean;
  nvGain: number;
  /** 0 = white phosphor, 1 = classic green. */
  nvPhosphor: number;
  debugView: number;
  /**
   * Work counters (rays, node visits, RIS candidates...) into __stats.counters.
   * A measurement mode, not a preference: never persisted, and while it is on
   * the frame's milliseconds are inflated by atomic contention.
   */
  counters: boolean;
  /**
   * Debug-only ablation of the dynamic-geometry reprojection test: 0 shipped
   * (depth identity), 1 fork-point behaviour (static depth+normal test on
   * dynamic pixels), 2 loose absolute band, 3 accept every tap. Set by the
   * temporal-audit scenarios so their before/after run from one build.
   */
  debugTapMode: number;
  /** The smoke fluid simulation steps each frame. Off freezes the field. */
  fluidSim: boolean;
  // Solver tuning (Jacobi count, vorticity, buoyancy, ...) is not here: it
  // lives on FluidSim.tune, which is its only owner. A second copy here was
  // pushed into the solver every frame and silently overrode anything a
  // scenario or slider wrote directly.
}

// Defaults match quality preset 1 ("performance"): it is the setting that fits
// inside a single 120Hz vblank, so it is the one that feels right out of the box.
export const DEFAULT_SETTINGS: RenderSettings = {
  resolutionScale: 0.5,
  // 1 is enough now that indirect bounces resample across all lights; before
  // that fix the indirect noise made 2 look necessary, at double the cost.
  spp: 1,
  bounces: 1,
  // Extinction per metre at unit density — now a SMOKE coefficient.
  //
  // 0.05 was chosen against the room haze (mean density 0.55, giving a mean
  // sigma of ~0.028/m) and was necessarily a compromise: one coefficient had
  // to serve a haze that fills 52 m and a cloud two metres thick, so it was
  // set where the haze looked right and the smoke came out a soft glow with
  // no structure. With fogAmount at 0 that compromise is gone and this can be
  // set for the cloud, which is the only thing it now multiplies.
  volumetric: 0.55,
  // Forward scattering, which is what smoke does. 0.55 was already hardcoded
  // in the torch path; lifting it to a setting applies it to every light and
  // makes it tunable. Energy-conserving, so pushing it up trades front-lit
  // brightness for back-lit rim.
  volPhaseG: 0.55,
  smokeShadow: 1.0,
  volExtinction: true,
  // With the haze gone the march concentrates every step on the cloud, so the
  // detail is worth paying for now. `smokeDetail` drives an erosion with a
  // contrast curve rather than a zero-mean modulation, so it thins the cloud
  // as it sharpens it — extinction above is what puts the density back.
  smokeDetail: 1.35,
  smokeDetailFreq: 9.0,
  // Deliberately low. At 1.9 the unlit areas sat at a readable grey, which
  // undercuts the whole premise: dark has to actually be dark for the beam to
  // carry the scene. Bright regions still resolve because AgX handles the
  // highlight rolloff.
  //
  // 0.35 rather than the 0.10 this was first tuned to, by way of 0.25. 0.10
  // was dialled in at night and is unplayable in a lit room; 0.25 was a guess
  // at the correction. 0.35 is measured — it is where the calibration ladder
  // puts the threshold patch on a display in a lit room, and it matches where
  // playtesting landed twice independently. Still only a starting point: the
  // player calibrates it and the choice persists. See ui/brightness.ts.
  exposure: 0.35,
  bloomIntensity: 0.6,
  bloomThreshold: 1.0,
  vignette: 0.5,
  grain: 0.022,
  chromatic: 0.0022,
  saturation: 1.12,
  // Values below were dialled in by hand against the real display; they are
  // much darker than a first guess suggests because it is meant to read as
  // night.
  skyIntensity: 0.04,
  // Zero, deliberately. This was a readability hack for when the indirect
  // estimate was too noisy to lean on; now that bounce light is clean it does
  // the same job physically, and a constant floor only greys out the blacks
  // that make the scene read as night. Kept on the panel as an escape hatch.
  ambient: 0.0,
  // 24, up from 12. 12 was chosen when the only volumetric content was fog
  // noise, where the march just needs to not band. Smoke is different: the
  // sim now carries detail at the cell scale, and the march is what decides
  // whether any of it survives to the image. Over the level's box a 12-step
  // march is a ~4 m step, which cannot resolve a 0.25 m cell no matter how
  // good the solver is — every advection improvement below is invisible at 12.
  // The march concentrates its steps on the occupied span, so with smoke as
  // the only medium the cost is well under the +2.1 ms the old 8 -> 24
  // measurement suggested — the steps are spent on two metres of cloud rather
  // than on fifty metres of box.
  volumetricSteps: 24,
  // No room haze.
  //
  // It cost more than it earned: it forced `volumetric` to be a compromise
  // between a 52 m haze and a 2 m cloud (see above), and it filled the whole
  // density box, so the march could never concentrate its steps and smoke was
  // sampled every ~2 m however much detail the solver produced.
  //
  // KNOWN LOSS: fog is the medium the torch beam scattered in, so the visible
  // beam shaft and the soft skirt around the moon pools go with it. If those
  // are wanted back they should come from something local to the beam rather
  // than from a global constant that every smoke setting has to negotiate with.
  fogAmount: 0.0,
  restirCandidates: 8,
  // Off since spatial taps landed — see the restirSpatialTaps comment below.
  // Temporal reservoir reuse both measures worse than spatial-only and goes
  // stale when the flashlight sweeps; the SVGF history (reproject.wgsl) is a
  // separate mechanism and stays on.
  restirTemporal: false,
  restirGI: true,
  restirMCap: 20,
  // Spatial-only reuse, measured against the converged reference at 1152x720
  // (400 ref frames / 120 test frames, single runs but the ordering held
  // across four independent runs):
  //   temporal only            relRmse 1.01   relBias +0.23
  //   temporal + 3 taps        relRmse 1.08   relBias +0.32
  //   spatial only, 6 taps r8  relRmse 0.65   relBias -0.02
  // Spatial taps borrow from fresh unbiased RIS streams, so they multiply
  // effective candidates without inheriting the temporal merge's W inflation —
  // stacking them on temporally-fed streams multiplies exposure to it instead,
  // which is why "both" is the worst of the three. Radius swept at 4/8/12/16/24:
  // flat bottom around 8. Cost of 6 taps: +1.3ms at 1152x720 (+5.4%).
  restirSpatialTaps: 6,
  restirSpatialRadius: 8,
  flashVisTarget: true,
  flashVisVolumetric: true,
  // Radiance cascades. Measured in the office at 1152x720, non-serial bench:
  //
  //   mode           fps    pathtrace  structure   bias vs 5-bounce ref
  //   radiosityRead  40.2   13.09      2.81        +0.315
  //   cascades       29.0   13.14     11.71        -0.161
  //   gather         24.4   33.74      2.87        +0.260
  //
  // Cascades is chosen on quality and on structure, not on speed: it is the
  // closest to a converged reference of any mode, it has no per-face grid to
  // quantise against (which is what made small props light as flat slabs), and
  // it refines by adding a cascade at flat cost rather than by squaring a
  // form-factor matrix — 3413 patches already cost 44.4 MB, and 0.2 m patches
  // would be 711 MB.
  //
  // It costs 28% of the frame rate against radiosityRead, and that is the open
  // item. The cost is concentrated in the per-hit NEE inside the probe trace
  // (8 RIS candidates plus a shadow ray, on ~4.4M probe rays), not in the
  // cascade structure itself.
  //
  // `gather` is dominated and kept only for A/B: worse bias than cascades AND
  // slower, because it traces bounce 1 per pixel while both other modes skip it.
  indirectMode: "cascades",
  transientSamples: 8,
  // Glow by default: measured, widening the stride alone makes the far field
  // worse rather than better. See atrous.wgsl.
  transientFilter: 2,
  transientBlurDist: 12,
  transientBounceWeight: 1,
  transientBlurStride: 4,
  transientBlurStrength: 1,
  indirectRate: 1.0,
  reference: false,
  nightVision: false,
  nvGain: 475,
  nvPhosphor: 0.09,
  debugView: 0,
  counters: false,
  debugTapMode: 0,
  fluidSim: true,
};

export interface FrameState {
  invViewProj: Mat4;
  prevViewProj: Mat4;
  camPos: Vec3;
  /** Camera position that produced prevViewProj. */
  prevCamPos: Vec3;
  flashPos: Vec3;
  flashDir: Vec3;
  flashColor: Vec3;
  flashIntensity: number;
  flashRadius: number;
  flashCosInner: number;
  flashCosOuter: number;
  time: number;
  mouseX: number;
  mouseY: number;
  /** 0..1 detection pulse for the post pass — see PostParams.seenPulse. */
  seenPulse: number;
  /** Game time this frame advanced, seconds; the fluid step integrates it. */
  dt: number;
  /**
   * Live smoke sources for the fluid solver, packed by src/game/smoke.ts as
   * FLUID_SOURCE_STRIDE floats each (see fluid.wgsl's Source struct).
   */
  smokeSources: Float32Array;
  smokeSourceCount: number;
}

/** Bind-group entries accept either; helpers below take whichever is on hand. */
type TexOrView = GPUTexture | GPUTextureView;

interface Targets {
  width: number;
  height: number;
  albedo: GPUTexture;
  normalDepth: [GPUTexture, GPUTexture];
  pos: GPUTexture;
  /**
   * The three raw radiance signals the trace pass emits — direct, indirect,
   * transient — as layers 0..2 of ONE array texture. Bound as a single
   * texture_storage_2d_array in the trace pass, they cost one storage-texture
   * slot instead of three, which keeps that pass inside WebGPU's default
   * limit of 4 per stage (SwiftShader, most mobile GPUs). The *Raw fields
   * below are single-layer 2D views onto it, so every consumer still reads
   * a plain texture_2d.
   */
  illumArray: GPUTexture;
  illumRaw: GPUTextureView;
  accumIllum: GPUTexture;
  illumHist: [GPUTexture, GPUTexture];
  momentsHist: [GPUTexture, GPUTexture];
  scratch: [GPUTexture, GPUTexture];
  momentsScratch: [GPUTexture, GPUTexture];
  /** Parallel chain for the indirect signal, filtered on its own terms. */
  indRaw: GPUTextureView;
  indAccum: GPUTexture;
  indHist: [GPUTexture, GPUTexture];
  indMoments: [GPUTexture, GPUTexture];
  indScratch: [GPUTexture, GPUTexture];
  indMomentsScratch: [GPUTexture, GPUTexture];
  /**
   * Transient lighting. No history and no moments: this signal is never
   * temporally accumulated, so there is nothing to carry between frames and the
   * a-trous passes run with the luminance weight off, exactly like indirect.
   */
  transRaw: GPUTextureView;
  transScratch: [GPUTexture, GPUTexture];
  /**
   * Written and ignored. The a-trous pass always emits moments, and binding one
   * texture as both its input and its output in a single pass is a validation
   * error that invalidates the entire command buffer — so the transient chain
   * ping-pongs a throwaway pair rather than aliasing one.
   */
  transMoments: [GPUTexture, GPUTexture];
  /**
   * The medium: in-scattered radiance (rgb) + camera-ray transmittance (a),
   * on its own accumulation and filter chain. It is smooth and low-frequency,
   * so the chain runs a short clamped history and wide a-trous strides.
   */
  volRaw: GPUTextureView;
  volAccum: GPUTexture;
  volHist: [GPUTexture, GPUTexture];
  volMoments: [GPUTexture, GPUTexture];
  volScratch: [GPUTexture, GPUTexture];
  volMomentsScratch: [GPUTexture, GPUTexture];
  hdr: GPUTexture;
  bloomDown: GPUTexture[];
  bloomUp: GPUTexture[];
  /**
   * Per-pixel ReSTIR reservoirs for BOTH frames of the ping-pong: one buffer
   * of two W*H halves, addressed by frame parity in the shader (a uniform, not
   * a rebound buffer). Half `parity` is written this frame; the other half is
   * last frame's and is read-only by convention. One read_write binding
   * instead of a read/read_write pair — the trace pass's storage-buffer
   * budget is what this buys back.
   */
  reservoirs: GPUBuffer;
  /** Per-pixel ReSTIR GI reservoirs, likewise merged. */
  giReservoirs: GPUBuffer;
}

export class Renderer {
  private device: GPUDevice;
  private targets: Targets | null = null;
  private parity = 0;
  private frameIndex = 0;

  // Buffers
  private uniformBuffer!: GPUBuffer;
  private uniformData = new ArrayBuffer(UNIFORM_SIZE);
  private uniformF32 = new Float32Array(this.uniformData);
  private uniformU32 = new Uint32Array(this.uniformData);
  private atrousBuffer!: GPUBuffer;
  // Scratch for the per-frame uniform writes. These used to be allocated fresh
  // every frame — ~18 typed arrays plus their u32 aliases, i.e. a few thousand
  // short-lived objects a second for data that is the same size every time.
  private compositeScratch = new Float32Array(8);
  private compositeScratchU32 = new Uint32Array(this.compositeScratch.buffer);
  private postScratch = new Float32Array(16);
  private postScratchU32 = new Uint32Array(this.postScratch.buffer);
  private bloomScratch = new Float32Array(8);
  private bloomScratchU32 = new Uint32Array(this.bloomScratch.buffer);
  /**
   * Bloom params depend only on texture sizes and the threshold, so they change
   * on resize or a settings tweak — not per frame. Nine queue writes a frame
   * were going out for data that had not moved.
   */
  private bloomParamsDirty = true;
  private bloomThresholdWritten = NaN;
  /**
   * Preallocated single-element dynamic-offset arrays.
   *
   * setBindGroup takes a sequence, so every call site was building a fresh
   * array literal — 12 a frame on top of the tuple arrays the old closure took.
   */
  private readonly reprojectOffsets: number[][] =
    Array.from({ length: REPROJECT_SLOTS }, (_, i) => [i * REPROJECT_STRIDE]);
  private readonly atrousOffsets: number[][] =
    Array.from({ length: ATROUS_SLOTS }, (_, i) => [i * ATROUS_STRIDE]);
  /** Pass labels, so the per-frame loop is not building template strings. */
  private readonly atrousLabels: string[] =
    Array.from({ length: ATROUS_ITERS }, (_, i) => `atrous${i}`);
  private readonly atrousIndLabels: string[] =
    Array.from({ length: ATROUS_ITERS }, (_, i) => `atrousInd${i}`);
  private readonly atrousTransLabels: string[] =
    Array.from({ length: TRANS_ATROUS_ITERS }, (_, i) => `atrousTrans${i}`);
  private readonly atrousVolLabels: string[] =
    Array.from({ length: VOL_ATROUS_ITERS }, (_, i) => `atrousVol${i}`);
  private reprojectBuffer!: GPUBuffer;
  private compositeBuffer!: GPUBuffer;
  private bloomBuffers: GPUBuffer[] = [];
  private postBuffer!: GPUBuffer;

  // Layouts
  private sceneLayout!: GPUBindGroupLayout;
  private ptLayout!: GPUBindGroupLayout;
  private flashmapLayout!: GPUBindGroupLayout;
  private radiosityLayout!: GPUBindGroupLayout;
  private cascadeLayout!: GPUBindGroupLayout;
  private cascadeTracePipeline!: GPUComputePipeline;
  private cascadeMergePipeline!: GPUComputePipeline;
  /**
   * Radiance cascade 0: probe radiance, ping-ponged for the temporal average.
   * Outside Targets — the lattice is world space and has nothing to do with
   * how many pixels are being traced, so a resize must not disturb it.
   */
  private cascadeLevels: CascadeLevel[] = [];
  private cascadeRaw: GPUTexture[] = [];
  private cascadeRawViews: GPUTextureView[] = [];
  private cascadeHistViews: GPUTextureView[] = [];
  private cascadeHist: GPUTexture[] = [];
  private cascadeMergedViews: GPUTextureView[] = [];
  private cascadeParams: GPUBuffer[] = [];
  private cascadeTraceGroups: GPUBindGroup[] = [];
  private cascadeMergeGroups: GPUBindGroup[] = [];
  private cascadeIrradiancePipeline!: GPUComputePipeline;
  private cascadeShView!: GPUTextureView;
  private cascadeIrradianceGroup!: GPUBindGroup;
  private cascadeFrame = 0;
  private reprojectLayout!: GPUBindGroupLayout;
  private atrousLayout!: GPUBindGroupLayout;
  private compositeLayout!: GPUBindGroupLayout;
  private lightVolLayout!: GPUBindGroupLayout;
  private bloomLayout!: GPUBindGroupLayout;
  private postLayout!: GPUBindGroupLayout;
  private probeLayout!: GPUBindGroupLayout;

  // Pipelines
  private ptPipeline!: GPUComputePipeline;
  private flashmapPipeline!: GPUComputePipeline;
  private radBakeFFPipeline!: GPUComputePipeline;
  private radBakeNormPipeline!: GPUComputePipeline;
  private radBakeVisPipeline!: GPUComputePipeline;
  private radBakeSkyPipeline!: GPUComputePipeline;
  private radInjectPipeline!: GPUComputePipeline;
  private radSolvePipeline!: GPUComputePipeline;
  /** Per-frame patch-emitter CDF (patchRIS mode only). */
  private radCdfPipeline!: GPUComputePipeline;
  private reprojectPipeline!: GPUComputePipeline;
  private atrousPipeline!: GPUComputePipeline;
  private compositePipeline!: GPUComputePipeline;
  private lightVolPipeline!: GPUComputePipeline;
  private bloomDownPipeline!: GPUComputePipeline;
  private bloomUpPipeline!: GPUComputePipeline;
  private postPipeline!: GPURenderPipeline;
  private probePipeline!: GPUComputePipeline;

  // Bind groups
  private sceneBindGroup!: GPUBindGroup;
  private ptBindGroups: GPUBindGroup[] = [];
  /**
   * The flashlight depth map. Fixed 128x128 regardless of render resolution,
   * so it lives outside Targets and survives resizes.
   */
  private flashmapTexture!: GPUTexture;
  private flashmapView!: GPUTextureView;
  private flashmapBindGroup!: GPUBindGroup;
  /** Per-layer owner dynamic group for the flashmap self-skip. */
  private flashmapParamsBuffer!: GPUBuffer;
  private torchGroups = new Uint32Array(TORCH_LAYERS);
  /**
   * Static light volume — see lightvolume.wgsl. Baked at init and re-baked
   * on the next frame after any static light intensity changes.
   */
  private lightVolTexture!: GPUTexture;
  private lightVolView!: GPUTextureView;
  private lightVolBindGroup!: GPUBindGroup;
  private lightVolDirty = false;
  /**
   * Smoke density volume — the density interface (see the top of the
   * volumetric section in pathtrace.wgsl). Allocated zeroed; the fluid
   * simulation writes it as its last pass every frame.
   */
  smokeVolume!: GPUTexture;
  private smokeVolumeView!: GPUTextureView;
  /** The smoke fluid simulation feeding the volume above. */
  fluid!: FluidSim;
  /**
   * The fine local lattice. Allocated once at startup at fixed dims and never
   * reallocated; it is repositioned to an event and stepped only while one is
   * live, so the resting cost of the whole mechanism is zero dispatches.
   */
  fluidFine!: FluidSim;
  /** True while the fine lattice is anchored to a live event and stepping. */
  fineActive = false;

  /**
   * Anchor the fine lattice on a world point and start stepping it.
   *
   * The origin is snapped to whole COARSE cells, which is what keeps the two
   * lattices in exact 2:1 registration; without it every resample between them
   * would need filtering and the fp16 round-trip would stop being exact. The
   * lattice starts empty and ramps in, so placement is visually continuous and
   * the volumetric reprojection never sees a discontinuity.
   */
  activateFine(x: number, z: number): void {
    const half = (FINE_DIMS[0] * FINE_CELL) / 2;
    const snap = (v: number, o: number) =>
      o + Math.round((v - half - o) / this.smokeCellSize) * this.smokeCellSize;
    this.fineOrigin = [
      snap(x, this.smokeOrigin[0]),
      this.smokeOrigin[1],
      snap(z, this.smokeOrigin[2]),
    ];
    // Declare the destination BEFORE resetting: buildGrid bakes occupancy at
    // simOrigin, so assigning it afterwards would bake the old position.
    this.fluidFine.nextOrigin = [...this.fineOrigin];
    this.fluidFine.reset();
    this.fluid.peerOrigin = [...this.fineOrigin];
    this.fluid.peerMode = 1;
    // reset() reallocated the fine lattice's textures, so the COARSE lattice's
    // bind group — which holds the fine density as its restriction source — is
    // now pointing at a destroyed texture. Same hazard as the tracer's, one
    // level down, and equally silent: the submit fails and the frame is lost.
    this.fluid.peerDensity = this.fluidFine.densityTexture;
    this.fluid.rebuildBindGroups();
    this.fineActive = true;
    this.fineBlend = 0;
  }

  /** Stop stepping the fine lattice; the ramp runs out first. */
  deactivateFine(): void {
    this.fineActive = false;
    // Keep restricting while the ramp runs out, so the cloud lands in the
    // coarse field rather than vanishing from it.
    if (this.fineBlend <= 0) { this.fluid.peerMode = 0; }
  }
  /** Activation ramp, 0..1, so a lattice arriving or leaving does not pop. */
  fineBlend = 0;
  private fineOrigin: [number, number, number] = [0, 0, 0];
  /** Scratch for splitting the frame's sources between the two lattices. */
  private srcFine = new Float32Array(FLUID_MAX_SOURCES * FLUID_SOURCE_STRIDE);
  private srcCoarse = new Float32Array(FLUID_MAX_SOURCES * FLUID_SOURCE_STRIDE);

  /**
   * Split the frame's sources by which lattice owns them.
   *
   * Each source must go to exactly ONE lattice. Feeding both injects the same
   * smoke twice — once at each resolution — and the restriction then adds the
   * fine copy on top of the coarse one, so the cloud is roughly twice the mass
   * it was asked for and the two lattices disagree about a field they are
   * supposed to share. The fine lattice claims anything inside its box, inset
   * by the rim where the restriction is already ramping down.
   */
  private routeSources(src: Float32Array, count: number): { nf: number; nc: number } {
    const S = FLUID_SOURCE_STRIDE;
    let nf = 0, nc = 0;
    const inset = FINE_CELL * 4;
    const lo = this.fineOrigin;
    const hi: [number, number, number] = [
      lo[0] + FINE_DIMS[0] * FINE_CELL,
      lo[1] + FINE_DIMS[1] * FINE_CELL,
      lo[2] + FINE_DIMS[2] * FINE_CELL,
    ];
    const live = this.fineActive || this.fineBlend > 0;
    for (let i = 0; i < count; i++) {
      const o = i * S;
      const x = src[o], y = src[o + 1], z = src[o + 2];
      const mine = live
        && x > lo[0] + inset && x < hi[0] - inset
        && z > lo[2] + inset && z < hi[2] - inset
        && y >= lo[1] && y <= hi[1];
      const dst = mine ? this.srcFine : this.srcCoarse;
      const n = mine ? nf++ : nc++;
      dst.set(src.subarray(o, o + S), n * S);
    }
    return { nf, nc };
  }
  /**
   * The `fluidFine.generation` the pathtrace bind groups were built against.
   *
   * Without this a single reset() leaves them pointing at a destroyed texture
   * and every subsequent submit fails validation — silently, as far as the
   * image is concerned, because the whole frame is dropped.
   */
  private fineGeneration = -1;
  /**
   * Denoiser knobs, live. Every value is the one the chain was hardcoded to,
   * so the defaults are a no-op.
   *
   * Here rather than in RenderSettings for the reason FluidSim.tune is: one
   * owner. A second copy in the settings object gets pushed at the buffer
   * every frame and silently overrides whatever a scenario or a slider wrote.
   * Consequently these are not persisted — a debug session cannot leave the
   * denoiser detuned across a reload.
   */
  readonly denoise: DenoiseTuning = { ...DEFAULT_DENOISE };
  readonly cascades: CascadeTuning = { ...DEFAULT_CASCADES };
  // Coarse smoke readback (gameplay LOS). Lags the frame; see the probes.
  private smokeProbePipeline!: GPUComputePipeline;
  private smokeProbeLayout!: GPUBindGroupLayout;
  private smokeProbeBindGroup!: GPUBindGroup;
  private smokeCoarseBuffer!: GPUBuffer;
  private smokeCoarseStaging!: GPUBuffer;
  private smokeCoarseBusy = false;
  private smokeCoarseArmed = false;
  /** Latest coarse smoke density, smokeCoarseDims in (x, y, z), a few frames old. */
  private smokeCoarse = new Float32Array(
    SMOKE_COARSE_DIMS[0] * SMOKE_COARSE_DIMS[1] * SMOKE_COARSE_DIMS[2],
  );
  /** Radiosity: patch solve state. G + sky ride a texture so the trace pass
   *  pays no storage-buffer slots for them. */
  private radPatchCount = 0;
  private radDynBuffer!: GPUBuffer;
  private radStaticBuffer!: GPUBuffer;
  private radGSkyView!: GPUTextureView;
  private radFaceView!: GPUTextureView;
  /** Two groups differing only in which B half is in/out. */
  private radBindGroups: GPUBindGroup[] = [];
  private reprojectBindGroups: GPUBindGroup[] = [];
  private atrousBindGroups: GPUBindGroup[][] = [];
  private indReprojectBindGroups: GPUBindGroup[] = [];
  private indAtrousBindGroups: GPUBindGroup[][] = [];
  private transAtrousBindGroups: GPUBindGroup[][] = [];
  private volReprojectBindGroups: GPUBindGroup[] = [];
  private refVolReprojectBindGroups: GPUBindGroup[] = [];
  private volAtrousBindGroups: GPUBindGroup[][] = [];
  private compositeBindGroups: GPUBindGroup[] = [];
  /** Reads the accumulators directly, bypassing the a-trous chain. */
  private refCompositeBindGroups: GPUBindGroup[] = [];
  /**
   * Reference-mode reproject, writing straight into the colour history.
   *
   * Normally a-trous iteration 0 produces next frame's history; reference mode
   * skips a-trous entirely, so reproject has to close that loop itself or the
   * history is never written and nothing accumulates at all.
   */
  private refReprojectBindGroups: GPUBindGroup[] = [];
  private refIndReprojectBindGroups: GPUBindGroup[] = [];
  private bloomDownBindGroups: GPUBindGroup[] = [];
  private bloomUpBindGroups: GPUBindGroup[] = [];
  private postBindGroup!: GPUBindGroup;
  private probeBindGroup!: GPUBindGroup;
  private probeParamsBuffer!: GPUBuffer;
  private probeOutBuffer!: GPUBuffer;
  private probeStaging!: GPUBuffer;
  private probeStagingBusy = false;
  private probeParams = new Float32Array(MAX_PROBES * 4 + 4);
  private probeCount = 0;
  /** Latest readback, one luminance per probe. Lags the frame by 1-2. */
  readonly probeLuma = new Float32Array(MAX_PROBES);
  /** Probe 0's raw output: [lightCount, unoccludedSum, blockedCount, luminance]. */
  readonly probeDebug = new Float32Array(4);

  private sampler!: GPUSampler;
  /** World placement of the two volumes; written to the uniform block. */
  /**
   * The rendered smoke lattice. Defaults to the level-wide one; a demo page can
   * replace all three via Renderer.create's `smoke` argument.
   */
  private smokeDims: [number, number, number] = [...SMOKE_DIMS];
  private smokeCellSize = SMOKE_CELL;
  private smokeCoarseDims: [number, number, number] = [...SMOKE_COARSE_DIMS];
  private smokeOrigin: [number, number, number] = [...MEDIUM_ORIGIN];
  private readonly lightVolOrigin: [number, number, number] = MEDIUM_ORIGIN;
  private readonly lightVolCell: [number, number, number] = [
    MEDIUM_SIZE[0] / LIGHT_VOL_DIMS[0],
    MEDIUM_SIZE[1] / LIGHT_VOL_DIMS[1],
    MEDIUM_SIZE[2] / LIGHT_VOL_DIMS[2],
  ];
  readonly profiler: GpuProfiler;
  /** Machine-independent work counters; totals in `workCounters.latest`. */
  readonly workCounters: WorkCounters;
  private lightCount = 0;
  /** Static lights, i.e. the offset at which the dynamic tail begins. */
  private staticLightCount = 0;
  private lightBuffer!: GPUBuffer;
  private matBuffer!: GPUBuffer;
  /**
   * The material list as it was at pack time, and its length then.
   *
   * The material buffer is sized to exactly what the scene held when `init`
   * ran and is never grown or re-uploaded — `setMaterialEmissive` only patches
   * slots that already exist. A material registered after that point is
   * therefore out of bounds, and WGSL's robust access *clamps* the read rather
   * than faulting, so the geometry silently shades as the last packed material
   * instead of failing. That is a very quiet bug (it cost this project three
   * invisible particle effects), so the count is checked rather than trusted.
   */
  private packedMaterials: Material[] = [];
  private packedMaterialCount = 0;
  private materialGrowthWarned = false;
  private matScratch3 = new Float32Array(3);
  /** Scratch for the dynamic tail, allocated once. */
  private dynLightData = new Float32Array(MAX_DYN_LIGHTS * LIGHT_STRIDE_F32);
  private lightScratch: Light[] = [];
  private lightScratch1 = new Float32Array(1);
  /**
   * First transient light index. Everything from here on is a muzzle flash or a
   * burst: sampled by plain NEE into its own un-accumulated signal, and
   * deliberately excluded from ReSTIR, since a reservoir must never hold a
   * light that is about to stop existing.
   */
  private transientStart = 0;
  private dynBuffer!: GPUBuffer;
  private prevDynBuffer!: GPUBuffer;
  /** Last frame's packed dynamic boxes, retained for motion vectors. */
  private prevDynData = new Float32Array(MAX_DYN_BOXES * BOX_STRIDE_F32);
  private dynCount = 0;
  private dynGroupCount = 0;
  /** xyz per group, w unused — laid out as vec4f to match the WGSL array. */
  private dynGroupMin = new Float32Array(DYN_GROUPS * 4);
  private dynGroupMax = new Float32Array(DYN_GROUPS * 4);

  private constructor(private ctx: GPUContext) {
    this.device = ctx.device;
    this.profiler = new GpuProfiler(ctx.device, ctx.features.has("timestamp-query"));
    this.workCounters = new WorkCounters(ctx.device);
  }

  /**
   * @param bakeOccupancy voxelises the static level for the fluid solver;
   *   omitted, the solver sees an empty room (only its six walls).
   */
  static async create(
    ctx: GPUContext, scene: SceneBuilder, bvh: BVH,
    bakeOccupancy: OccupancyBaker | null = null,
    smoke?: SmokeLattice,
  ): Promise<Renderer> {
    const r = new Renderer(ctx);
    if (smoke) {
      r.smokeDims = [...smoke.dims];
      r.smokeCellSize = smoke.cell;
      r.smokeOrigin = [...smoke.origin];
      r.smokeCoarseDims = coarseDimsFor(smoke.dims);
      r.smokeCoarse = new Float32Array(
        r.smokeCoarseDims[0] * r.smokeCoarseDims[1] * r.smokeCoarseDims[2],
      );
    }
    await r.init(scene, bvh, bakeOccupancy);
    return r;
  }

  private async init(
    scene: SceneBuilder, bvh: BVH, bakeOccupancy: OccupancyBaker | null,
  ): Promise<void> {
    const d = this.device;
    this.lightCount = scene.lights.length;
    this.staticLightCount = scene.lights.length;
    this.transientStart = scene.lights.length;

    // ---- scene buffers ----------------------------------------------------
    const boxData = packBoxes(scene.boxes, bvh.order, scene.materials);
    const matData = packMaterials(scene.materials);
    const lightData = packLights(scene.lights);

    const storage = (label: string, data: Float32Array<ArrayBuffer>) => {
      const buf = d.createBuffer({
        label,
        size: Math.max(data.byteLength, 16),
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
      });
      d.queue.writeBuffer(buf, 0, data);
      return buf;
    };

    const boxBuffer = storage("boxes", boxData);
    const matBuffer = storage("materials", matData);
    this.matBuffer = matBuffer;
    this.packedMaterials = scene.materials;
    this.packedMaterialCount = scene.materials.length;
    const bvhBuffer = storage("bvh", bvh.nodes);

    // Oversized so moving lights can be appended after the static ones without
    // reallocating or disturbing lights[0], which the key-light channel needs.
    this.lightBuffer = d.createBuffer({
      label: "lights",
      size: (scene.lights.length + MAX_DYN_LIGHTS) * LIGHT_STRIDE_F32 * 4,
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    });
    d.queue.writeBuffer(this.lightBuffer, 0, lightData);
    const lightBuffer = this.lightBuffer;

    this.dynBuffer = d.createBuffer({
      label: "dynamic-boxes",
      size: MAX_DYN_BOXES * BOX_STRIDE_F32 * 4,
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    });
    this.prevDynBuffer = d.createBuffer({
      label: "dynamic-boxes-prev",
      size: MAX_DYN_BOXES * BOX_STRIDE_F32 * 4,
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    });

    this.uniformBuffer = d.createBuffer({
      label: "uniforms",
      size: UNIFORM_SIZE,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });

    this.atrousBuffer = d.createBuffer({
      label: "atrous-params",
      size: ATROUS_STRIDE * ATROUS_SLOTS,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });
    this.reprojectBuffer = d.createBuffer({
      label: "reproject-params",
      size: REPROJECT_STRIDE * REPROJECT_SLOTS,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });
    {
      //          outlierK, outlierFloor, clampK, clampFloor, validity, maxHist, alphaFloor
      const direct = new Float32Array([3.0, 0.05, 3.0, 0.0, 0.0, 48.0, 0.02, 0]);
      // Reference: no rejection of any kind, unbounded history, no alpha floor.
      // Every heuristic here exists to trade bias for responsiveness, which is
      // exactly what a ground-truth accumulator must not do.
      const BIG = 1e9;
      const refDirect = new Float32Array([BIG, BIG, BIG, BIG, 0.0, BIG, 0.0, 0]);
      const refIndirect = new Float32Array([BIG, BIG, BIG, BIG, 1.0, BIG, 0.0, 0]);
      // Volume: a short history the neighbourhood clamp keeps honest — a
      // swept beam or a fired flash must fade in a handful of frames, not a
      // second. The alpha floor caps convergence at ~1/12.
      const volume = new Float32Array([4.0, 0.02, 2.5, 0.02, 0.0, 12.0, 0.08, 0]);
      d.queue.writeBuffer(this.reprojectBuffer, 0, direct);
      d.queue.writeBuffer(this.reprojectBuffer, REPROJECT_STRIDE * 2, refDirect);
      d.queue.writeBuffer(this.reprojectBuffer, REPROJECT_STRIDE * 3, refIndirect);
      d.queue.writeBuffer(this.reprojectBuffer, REPROJECT_STRIDE * 4, volume);
      d.queue.writeBuffer(this.reprojectBuffer, REPROJECT_STRIDE * 5, refDirect);
    }
    // The indirect slot and both a-trous chains come from `denoise`, so the
    // panel can move them; at the defaults this writes what was hardcoded.
    this.applyDenoise();
    // The medium's chain: wide strides, no luminance edge-stopping (the
    // signal is jittered-march noise, not detail) and loose geometric terms —
    // in-scattered light barely respects the surface behind it.
    for (let i = 0; i < VOL_ATROUS_ITERS; i++) {
      const volume = new ArrayBuffer(ATROUS_PARAM_SIZE);
      new Int32Array(volume, 0, 1)[0] = 2 << i;
      new Float32Array(volume, 4, 2).set([0.0, 4.0]);
      d.queue.writeBuffer(
        this.atrousBuffer,
        (ATROUS_ITERS * 2 + TRANS_ATROUS_ITERS + i) * ATROUS_STRIDE, volume,
      );
    }

    this.compositeBuffer = d.createBuffer({
      label: "composite-params",
      size: 32,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });
    this.postBuffer = d.createBuffer({
      label: "post-params",
      size: 64,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });

    this.sampler = d.createSampler({
      label: "linear-clamp",
      magFilter: "linear",
      minFilter: "linear",
      addressModeU: "clamp-to-edge",
      addressModeV: "clamp-to-edge",
    });

    // ---- layouts ----------------------------------------------------------
    const C = GPUShaderStage.COMPUTE;
    const F = GPUShaderStage.FRAGMENT;
    const ro = { type: "read-only-storage" as const };
    const tex = (sampleType: GPUTextureSampleType = "float") => ({ sampleType });
    const stTex = (format: GPUTextureFormat) => ({ access: "write-only" as const, format });

    this.sceneLayout = d.createBindGroupLayout({
      label: "scene",
      entries: [
        { binding: 0, visibility: C, buffer: { type: "uniform" } },
        { binding: 1, visibility: C, buffer: ro },
        { binding: 2, visibility: C, buffer: ro },
        { binding: 3, visibility: C, buffer: ro },
        { binding: 4, visibility: C, buffer: ro },
        { binding: 5, visibility: C, buffer: ro },
        { binding: 6, visibility: C, buffer: ro },
      ],
    });

    this.ptLayout = d.createBindGroupLayout({
      label: "pathtrace-targets",
      entries: [
        { binding: 0, visibility: C, storageTexture: stTex("rgba8unorm") },
        { binding: 1, visibility: C, storageTexture: stTex("rgba16float") },
        { binding: 2, visibility: C, storageTexture: stTex("rgba32float") },
        // Direct, indirect and transient radiance: one 2d-array binding, three
        // layers, so the pass fits WebGPU's default 4-storage-texture budget.
        {
          binding: 3, visibility: C,
          storageTexture: { access: "write-only", format: "rgba16float", viewDimension: "2d-array" },
        },
        { binding: 4, visibility: C, texture: tex() },
        // Merged reservoir buffers (DI, GI): both parity halves behind one
        // read_write binding each.
        { binding: 5, visibility: C, buffer: { type: "storage" } },
        { binding: 8, visibility: C, buffer: { type: "storage" } },
        // Work counters (atomic u32 slots). Always bound; the shader only
        // touches it while countersOn is set.
        { binding: 6, visibility: C, buffer: { type: "storage" } },
        // Torch depth maps. r32float is not filterable without an optional
        // feature, and the PCF taps are textureLoads anyway.
        {
          binding: 11, visibility: C,
          texture: { sampleType: "unfilterable-float", viewDimension: "2d-array" },
        },
        // Radiosity gather + sky rows, and the face table — as textures, so
        // they cost nothing against the storage-buffer budget.
        { binding: 12, visibility: C, texture: tex("unfilterable-float") },
        { binding: 13, visibility: C, texture: tex("uint") },
        // Volumetrics: baked static light volume (10), the smoke density
        // volume Track B2b's simulation fills (14) and their shared trilinear
        // sampler (15). Bindings 16/17 belong to the radiosity track.
        { binding: 10, visibility: C, texture: { sampleType: "float", viewDimension: "3d" } },
        { binding: 14, visibility: C, texture: { sampleType: "float", viewDimension: "3d" } },
        // The fine smoke lattice. Binding 7 was the only free slot in group 1.
        { binding: 7, visibility: C, texture: { sampleType: "float", viewDimension: "3d" } },
        { binding: 15, visibility: C, sampler: { type: "filtering" } },
        // Radiosity patch geometry (patchRIS): the pass's 10th and last
        // storage buffer.
        { binding: 16, visibility: C, buffer: ro },
        // Radiance cascade 0, sampled. Costs nothing against the storage
        // budgets, which are both already at their ceiling.
        { binding: 9, visibility: C, texture: { sampleType: "float", viewDimension: "3d" } },
      ],
    });

    this.lightVolLayout = d.createBindGroupLayout({
      label: "lightvolume",
      entries: [
        { binding: 0, visibility: C, buffer: { type: "uniform" } },
        {
          binding: 1, visibility: C,
          storageTexture: { access: "write-only", format: "rgba16float", viewDimension: "3d" },
        },
        // Work counters — the bake reports only its shadow-ray count.
        { binding: 2, visibility: C, buffer: { type: "storage" } },
      ],
    });

    this.smokeProbeLayout = d.createBindGroupLayout({
      label: "smokeprobe",
      entries: [
        { binding: 0, visibility: C, buffer: { type: "storage" } },
        { binding: 1, visibility: C, buffer: { type: "uniform" } },
        { binding: 2, visibility: C, texture: { sampleType: "float", viewDimension: "3d" } },
      ],
    });

    this.flashmapLayout = d.createBindGroupLayout({
      label: "flashmap",
      entries: [
        {
          binding: 0, visibility: C,
          storageTexture: {
            access: "write-only", format: "r32float", viewDimension: "2d-array",
          },
        },
        // Work counters — the depth-map pass reports its own ray count.
        { binding: 1, visibility: C, buffer: { type: "storage" } },
        // Per-layer owner group table (self-skip); see setTorchGroups.
        { binding: 2, visibility: C, buffer: { type: "uniform" } },
      ],
    });

    // Two packed storage buffers, not eight: the scene group already holds 6
    // of the 8 storage-buffer slots WebGPU guarantees per stage.
    this.radiosityLayout = d.createBindGroupLayout({
      label: "radiosity",
      entries: [
        { binding: 0, visibility: C, buffer: { type: "uniform" } },
        { binding: 1, visibility: C, buffer: { type: "storage" } },
        { binding: 2, visibility: C, buffer: { type: "storage" } },
        {
          binding: 3, visibility: C,
          texture: { sampleType: "unfilterable-float", viewDimension: "2d-array" },
        },
        {
          binding: 4, visibility: C,
          storageTexture: { access: "write-only", format: "rgba32float" },
        },
      ],
    });

    this.cascadeLayout = d.createBindGroupLayout({
      label: "cascades",
      entries: [
        { binding: 0, visibility: C, buffer: { type: "uniform" } },
        {
          binding: 1, visibility: C,
          storageTexture: {
            access: "write-only", format: "rgba16float", viewDimension: "3d",
          },
        },
        // 2 = this cascade's history (trace), 3 = its raw trace and
        // 4 = the coarser merged cascade (merge). One layout for both entry
        // points; each pass binds the two it does not read to something valid.
        {
          binding: 2, visibility: C,
          texture: { sampleType: "float", viewDimension: "3d" },
        },
        {
          binding: 3, visibility: C,
          texture: { sampleType: "float", viewDimension: "3d" },
        },
        {
          binding: 4, visibility: C,
          texture: { sampleType: "float", viewDimension: "3d" },
        },
        // 5 = the SH irradiance volume, read by the trace for multi-bounce.
        {
          binding: 5, visibility: C,
          texture: { sampleType: "float", viewDimension: "3d" },
        },
      ],
    });

    this.reprojectLayout = d.createBindGroupLayout({
      label: "reproject",
      entries: [
        { binding: 0, visibility: C, texture: tex() },
        // gPos is rgba32float — sampling it requires float32-filterable, but we
        // only ever textureLoad, so declare it unfilterable.
        { binding: 1, visibility: C, texture: tex("unfilterable-float") },
        { binding: 2, visibility: C, texture: tex() },
        { binding: 3, visibility: C, texture: tex() },
        { binding: 4, visibility: C, texture: tex() },
        { binding: 5, visibility: C, texture: tex() },
        { binding: 6, visibility: C, storageTexture: stTex("rgba16float") },
        { binding: 7, visibility: C, storageTexture: stTex("rgba16float") },
        { binding: 8, visibility: C, buffer: { type: "uniform", hasDynamicOffset: true, minBindingSize: 32 } },
      ],
    });

    this.atrousLayout = d.createBindGroupLayout({
      label: "atrous",
      entries: [
        { binding: 0, visibility: C, texture: tex() },
        { binding: 1, visibility: C, texture: tex() },
        { binding: 2, visibility: C, texture: tex() },
        { binding: 3, visibility: C, storageTexture: stTex("rgba16float") },
        { binding: 4, visibility: C, storageTexture: stTex("rgba16float") },
        { binding: 5, visibility: C, buffer: { type: "uniform", hasDynamicOffset: true, minBindingSize: ATROUS_PARAM_SIZE } },
      ],
    });

    this.probeLayout = d.createBindGroupLayout({
      label: "probe",
      entries: [
        { binding: 0, visibility: C, buffer: { type: "storage" } },
        { binding: 1, visibility: C, buffer: { type: "uniform" } },
        // Work counters — the probe pass reports its own shadow-ray count.
        { binding: 2, visibility: C, buffer: { type: "storage" } },
      ],
    });

    this.compositeLayout = d.createBindGroupLayout({
      label: "composite",
      entries: [
        { binding: 0, visibility: C, buffer: { type: "uniform" } },
        { binding: 1, visibility: C, texture: tex() },
        { binding: 2, visibility: C, texture: tex() },
        { binding: 3, visibility: C, texture: tex() },
        { binding: 4, visibility: C, texture: tex() },
        { binding: 5, visibility: C, texture: tex() },
        { binding: 6, visibility: C, storageTexture: stTex("rgba16float") },
        { binding: 7, visibility: C, texture: tex() },
        { binding: 8, visibility: C, texture: tex() },
        // The medium: denoised in-scatter (rgb) + transmittance (a).
        { binding: 9, visibility: C, texture: tex() },
      ],
    });

    this.bloomLayout = d.createBindGroupLayout({
      label: "bloom",
      entries: [
        { binding: 0, visibility: C, sampler: { type: "filtering" } },
        { binding: 1, visibility: C, texture: tex() },
        { binding: 2, visibility: C, texture: tex() },
        { binding: 3, visibility: C, storageTexture: stTex("rgba16float") },
        { binding: 4, visibility: C, buffer: { type: "uniform" } },
      ],
    });

    this.postLayout = d.createBindGroupLayout({
      label: "post",
      entries: [
        { binding: 0, visibility: F, sampler: { type: "filtering" } },
        { binding: 1, visibility: F, texture: tex() },
        { binding: 2, visibility: F, texture: tex() },
        { binding: 3, visibility: F, buffer: { type: "uniform" } },
      ],
    });

    // ---- pipelines --------------------------------------------------------
    const [ptMod, rpMod, atMod, cpMod, blMod, poMod, prMod, fmMod, radMod, lvMod, spMod,
      cascadeMod] =
      await Promise.all([
        createShaderModule(d, "pathtrace", SHADERS.pathtrace),
        createShaderModule(d, "reproject", SHADERS.reproject),
        createShaderModule(d, "atrous", SHADERS.atrous),
        createShaderModule(d, "composite", SHADERS.composite),
        createShaderModule(d, "bloom", SHADERS.bloom),
        createShaderModule(d, "post", SHADERS.post),
        createShaderModule(d, "probe", SHADERS.probe),
        createShaderModule(d, "flashmap", SHADERS.flashmap),
        createShaderModule(d, "radiosity", SHADERS.radiosity),
        createShaderModule(d, "lightvolume", SHADERS.lightVolume),
        createShaderModule(d, "smokeprobe", SHADERS.smokeProbe),
        createShaderModule(d, "cascades", SHADERS.cascades),
      ]);

    const pl = (label: string, layouts: GPUBindGroupLayout[]) =>
      d.createPipelineLayout({ label, bindGroupLayouts: layouts });

    // Pipeline creation failures are asynchronous, so without a scope the only
    // symptom is "invalid due to a previous error" repeating once per frame
    // with the actual message long since scrolled out of the console.
    d.pushErrorScope("validation");

    this.ptPipeline = d.createComputePipeline({
      label: "pathtrace",
      layout: pl("pathtrace", [this.sceneLayout, this.ptLayout]),
      compute: { module: ptMod, entryPoint: "main" },
    });
    this.flashmapPipeline = d.createComputePipeline({
      label: "flashmap",
      layout: pl("flashmap", [this.sceneLayout, this.flashmapLayout]),
      compute: { module: fmMod, entryPoint: "main" },
    });
    const radPl = pl("radiosity", [this.sceneLayout, this.radiosityLayout]);
    const radPipe = (label: string, entryPoint: string) =>
      d.createComputePipeline({ label, layout: radPl, compute: { module: radMod, entryPoint } });
    this.radBakeFFPipeline = radPipe("rad-bake-ff", "bakeFF");
    this.radBakeNormPipeline = radPipe("rad-bake-norm", "bakeNorm");
    this.radBakeVisPipeline = radPipe("rad-bake-vis", "bakeVis");
    this.radBakeSkyPipeline = radPipe("rad-bake-sky", "bakeSky");
    this.radInjectPipeline = radPipe("rad-inject", "inject");
    this.radSolvePipeline = radPipe("rad-solve", "solve");
    this.radCdfPipeline = radPipe("rad-cdf", "buildPatchCdf");
    const cascadePl = pl("cascades", [this.sceneLayout, this.cascadeLayout]);
    this.cascadeTracePipeline = d.createComputePipeline({
      label: "cascade-trace",
      layout: cascadePl,
      compute: { module: cascadeMod, entryPoint: "traceCascade" },
    });
    this.cascadeMergePipeline = d.createComputePipeline({
      label: "cascade-merge",
      layout: cascadePl,
      compute: { module: cascadeMod, entryPoint: "merge" },
    });
    this.cascadeIrradiancePipeline = d.createComputePipeline({
      label: "cascade-irradiance",
      layout: cascadePl,
      compute: { module: cascadeMod, entryPoint: "irradiance" },
    });
    this.reprojectPipeline = d.createComputePipeline({
      label: "reproject",
      layout: pl("reproject", [this.sceneLayout, this.reprojectLayout]),
      compute: { module: rpMod, entryPoint: "main" },
    });
    this.atrousPipeline = d.createComputePipeline({
      label: "atrous",
      layout: pl("atrous", [this.sceneLayout, this.atrousLayout]),
      compute: { module: atMod, entryPoint: "main" },
    });
    this.compositePipeline = d.createComputePipeline({
      label: "composite",
      layout: pl("composite", [this.compositeLayout]),
      compute: { module: cpMod, entryPoint: "main" },
    });
    this.lightVolPipeline = d.createComputePipeline({
      label: "lightvolume",
      layout: pl("lightvolume", [this.sceneLayout, this.lightVolLayout]),
      compute: { module: lvMod, entryPoint: "bake" },
    });
    this.smokeProbePipeline = d.createComputePipeline({
      label: "smokeprobe",
      layout: pl("smokeprobe", [this.sceneLayout, this.smokeProbeLayout]),
      compute: { module: spMod, entryPoint: "main" },
    });
    this.bloomDownPipeline = d.createComputePipeline({
      label: "bloom-down",
      layout: pl("bloom", [this.bloomLayout]),
      compute: { module: blMod, entryPoint: "downsample" },
    });
    this.bloomUpPipeline = d.createComputePipeline({
      label: "bloom-up",
      layout: pl("bloom", [this.bloomLayout]),
      compute: { module: blMod, entryPoint: "upsample" },
    });
    this.probePipeline = d.createComputePipeline({
      label: "probe",
      layout: pl("probe", [this.sceneLayout, this.probeLayout]),
      compute: { module: prMod, entryPoint: "main" },
    });
    this.postPipeline = d.createRenderPipeline({
      label: "post",
      layout: pl("post", [this.postLayout]),
      vertex: { module: poMod, entryPoint: "vs" },
      fragment: { module: poMod, entryPoint: "fs", targets: [{ format: this.ctx.format }] },
      primitive: { topology: "triangle-list" },
    });

    const plErr = await d.popErrorScope();
    if (plErr) throw new Error(`pipeline creation failed: ${plErr.message}`);

    // Torch depth maps: layer 0 the player, 1..N-1 the guard torches. Must
    // match FLASHMAP_RES / TORCH_LAYERS in common.wgsl.
    this.flashmapTexture = d.createTexture({
      label: "flashmap",
      size: [FLASHMAP_RES, FLASHMAP_RES, TORCH_LAYERS],
      format: "r32float",
      // COPY_SRC for readFlashmapLayer: a layer's coverage is a diagnostic the
      // headless harness reads, not something the frame consumes.
      usage: GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.TEXTURE_BINDING |
        GPUTextureUsage.COPY_SRC,
    });
    this.flashmapView = this.flashmapTexture.createView({ dimension: "2d-array" });
    // Owner groups: layer 0 is the player (group 0 by the fixed-stride
    // packing), the rest unowned until the game says otherwise.
    this.torchGroups.fill(DYN_GROUP_NONE);
    this.torchGroups[0] = 0;
    this.flashmapParamsBuffer = d.createBuffer({
      label: "flashmap-params",
      size: TORCH_LAYERS * 4,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });
    d.queue.writeBuffer(this.flashmapParamsBuffer, 0, this.torchGroups);
    this.flashmapBindGroup = d.createBindGroup({
      label: "flashmap",
      layout: this.flashmapLayout,
      entries: [
        { binding: 0, resource: this.flashmapView },
        { binding: 1, resource: { buffer: this.workCounters.buffer } },
        { binding: 2, resource: { buffer: this.flashmapParamsBuffer } },
      ],
    });

    // Static light volume: a 3D grid over the room's air, baked below once
    // the scene bind group exists and re-baked when a static light changes.
    this.lightVolTexture = d.createTexture({
      label: "light-volume",
      dimension: "3d",
      size: { width: LIGHT_VOL_DIMS[0], height: LIGHT_VOL_DIMS[1], depthOrArrayLayers: LIGHT_VOL_DIMS[2] },
      format: "rgba16float",
      usage: GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.TEXTURE_BINDING,
    });
    this.lightVolView = this.lightVolTexture.createView({ dimension: "3d" });
    // Smoke density volume (the fluid track's target). Storage-writable so a
    // compute solver can fill it in place; textures start zeroed, so until
    // then it reads as no smoke.
    this.smokeVolume = d.createTexture({
      label: "smoke-volume",
      dimension: "3d",
      size: {
        width: this.smokeDims[0], height: this.smokeDims[1],
        depthOrArrayLayers: this.smokeDims[2],
      },
      format: "rgba16float",
      usage: GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.TEXTURE_BINDING |
        GPUTextureUsage.COPY_DST | GPUTextureUsage.COPY_SRC,
    });
    this.smokeVolumeView = this.smokeVolume.createView({ dimension: "3d" });
    this.fluid = await FluidSim.create(d, {
      volume: this.smokeVolume,
      dims: this.smokeDims,
      origin: this.smokeOrigin,
      cell: this.smokeCellSize,
    }, bakeOccupancy);

    // The fine lattice: 8 x 3.25 x 8 m at 6.25 cm. y spans the full slab
    // exactly (3.25 / 0.0625 = 52) and both lateral axes are exact 4:1
    // refinements of the coarse lattice, so every resample between them is an
    // integer operation and an fp16 copy is bit-exact.
    //
    // 851,968 cells against the coarse lattice's 389,376 — 2.19x — but it
    // steps ONLY while an event is anchored to it, so this is an event tax
    // rather than a standing one, and the resting cost stays exactly what it
    // was. 52 rows where the coarse lattice has 13.
    {
      const fdims: [number, number, number] = [FINE_DIMS[0], FINE_DIMS[1], FINE_DIMS[2]];
      this.fluidFine = await FluidSim.create(d, {
        volume: this.smokeVolume,   // never written: F dispatches no writeVolume
        dims: fdims,
        origin: MEDIUM_ORIGIN,
        cell: FINE_CELL,
      }, bakeOccupancy);
      this.fluidFine.openFaces = 0xf;
      this.fluidFine.writesInterface = false;

      // The fine lattice bakes its OWN box, not a level-wide field.
      //
      // Level-wide was the original plan and it does not scale: measured, a
      // 12.5 cm field over the whole level takes 233 ms to bake, and the
      // 6.25 cm field this is meant to grow into would be 8x that — about two
      // seconds of startup hitch. Baking only the 8 x 3.25 x 8 m box is ~30x
      // less volume, so it stays affordable at 6.25 cm and can simply be
      // redone each time the lattice is placed, which happens once per
      // canister rather than once per frame.
      this.fluidFine.occMode = 0;
      // The coarse lattice restricts the fine one's result into itself, which
      // is what puts fine smoke in front of the gameplay concealment path.
      this.fluid.peerDensity = this.fluidFine.densityTexture;
      this.fluid.peerCell = [FINE_CELL, FINE_CELL, FINE_CELL];
      this.fluid.peerDims = [...FINE_DIMS];
      this.fluid.rebuildBindGroups();
    }
    {
      const coarseBytes = this.smokeCoarse.byteLength;
      this.smokeCoarseBuffer = d.createBuffer({
        label: "smoke-coarse", size: coarseBytes,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC,
      });
      this.smokeCoarseStaging = d.createBuffer({
        label: "smoke-coarse-staging", size: coarseBytes,
        usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
      });
      const spParams = d.createBuffer({
        label: "smokeprobe-params", size: 16,
        usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
      });
      d.queue.writeBuffer(spParams, 0, new Uint32Array([
        this.smokeCoarseDims[0], this.smokeCoarseDims[1], this.smokeCoarseDims[2],
        SMOKE_COARSE_FACTOR,
      ]));
      this.smokeProbeBindGroup = d.createBindGroup({
        label: "smokeprobe",
        layout: this.smokeProbeLayout,
        entries: [
          { binding: 0, resource: { buffer: this.smokeCoarseBuffer } },
          { binding: 1, resource: { buffer: spParams } },
          { binding: 2, resource: this.smokeVolumeView },
        ],
      });
    }
    {
      // LightVolParams: origin vec3f, count u32, cell vec3f, rays u32, dims vec3u, pad.
      const lvParams = d.createBuffer({
        label: "lightvol-params", size: 48,
        usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
      });
      const pb = new ArrayBuffer(48);
      const pf = new Float32Array(pb);
      const pu = new Uint32Array(pb);
      pf[0] = this.lightVolOrigin[0]; pf[1] = this.lightVolOrigin[1]; pf[2] = this.lightVolOrigin[2];
      pu[3] = this.staticLightCount;
      pf[4] = this.lightVolCell[0]; pf[5] = this.lightVolCell[1]; pf[6] = this.lightVolCell[2];
      // Six visibility rays per light per voxel: trilinear filtering across
      // 0.5 m cells hides the rest of the shot noise in the pool edges.
      pu[7] = 6;
      pu[8] = LIGHT_VOL_DIMS[0]; pu[9] = LIGHT_VOL_DIMS[1]; pu[10] = LIGHT_VOL_DIMS[2];
      d.queue.writeBuffer(lvParams, 0, pb);
      this.lightVolBindGroup = d.createBindGroup({
        label: "lightvolume",
        layout: this.lightVolLayout,
        entries: [
          { binding: 0, resource: { buffer: lvParams } },
          { binding: 1, resource: this.lightVolView },
          { binding: 2, resource: { buffer: this.workCounters.buffer } },
        ],
      });
    }

    this.probeParamsBuffer = d.createBuffer({
      label: "probe-params",
      size: this.probeParams.byteLength,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });
    this.probeOutBuffer = d.createBuffer({
      label: "probe-out",
      size: MAX_PROBES * 16,
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC,
    });
    this.probeStaging = d.createBuffer({
      label: "probe-staging",
      size: MAX_PROBES * 16,
      usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
    });
    this.probeBindGroup = d.createBindGroup({
      label: "probe",
      layout: this.probeLayout,
      entries: [
        { binding: 0, resource: { buffer: this.probeOutBuffer } },
        { binding: 1, resource: { buffer: this.probeParamsBuffer } },
        { binding: 2, resource: { buffer: this.workCounters.buffer } },
      ],
    });

    this.sceneBindGroup = d.createBindGroup({
      label: "scene",
      layout: this.sceneLayout,
      entries: [
        { binding: 0, resource: { buffer: this.uniformBuffer } },
        { binding: 1, resource: { buffer: boxBuffer } },
        { binding: 2, resource: { buffer: matBuffer } },
        { binding: 3, resource: { buffer: lightBuffer } },
        { binding: 4, resource: { buffer: bvhBuffer } },
        { binding: 5, resource: { buffer: this.dynBuffer } },
        { binding: 6, resource: { buffer: this.prevDynBuffer } },
      ],
    });

    // ---- radiosity ----------------------------------------------------------
    // Patch the static faces, then bake form factors and light/sky visibility
    // in one blocking submit. The bake traces against the static scene only:
    // the dynamic-box uniforms are still zero here, which is exactly right —
    // characters must not be baked into permanent shadows.
    {
      const rad = buildPatches(scene.boxes, bvh.order, scene.materials);
      this.radPatchCount = rad.patchCount;
      const N = rad.patchCount;
      const S = this.staticLightCount;

      // Packed layouts — must match the offset functions in radiosity.wgsl:
      // static [patches 16N][skyE 4N][vis N*S][ff N*N], dyn [inject 4N][b0 4N][b1 4N].
      const radStatic = d.createBuffer({
        label: "rad-static",
        size: Math.max(16, (20 * N + N * S + N * N) * 4),
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
      });
      d.queue.writeBuffer(radStatic, 0, rad.patches);
      // The trace pass reads patch geometry from here (patchRIS mode), in
      // its one remaining storage-buffer slot.
      this.radStaticBuffer = radStatic;
      const radDyn = d.createBuffer({
        label: "rad-dyn",
        size: Math.max(16, 12 * N * 4),
        // COPY_SRC for readRadiosity: injected energy and B are diagnostics
        // the headless harness reads to verify what the solve was fed.
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC,
      });
      this.radDynBuffer = radDyn;

      // Four rows: gathered G, sky, emitter radiosity B, and the RIS CDF —
      // see radGSkyOut in radiosity.wgsl. A texture, not a buffer, because
      // the trace pass's storage-buffer budget is spent.
      const gsky = d.createTexture({
        label: "rad-gsky",
        size: [Math.max(1, N), 4],
        format: "rgba32float",
        usage: GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.TEXTURE_BINDING,
      });
      this.radGSkyView = gsky.createView();
      const faces = d.createTexture({
        label: "rad-faces",
        size: [Math.max(1, rad.faceTable.length / 4), 1],
        format: "rgba32uint",
        usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST,
      });
      d.queue.writeTexture(
        { texture: faces }, rad.faceTable, {},
        [rad.faceTable.length / 4, 1],
      );
      this.radFaceView = faces.createView();

      // Radiance cascades. The probe lattice rides the density interface's
      // box so the shaders derive it from U.smokeOrigin/U.smokeCell without
      // any uniform of its own.
      //
      // Three textures per level. `raw` is what the trace wrote, `hist` is
      // last frame's copy of it for the temporal average, `merged` is raw with
      // the coarser cascade folded in. raw and merged must be separate because
      // a pass cannot read the storage texture it writes; hist is a copy
      // rather than a ping-pong because the trace bind groups are built once
      // and an alternating binding would go stale.
      this.cascadeLevels = cascadeLevels();
      const cascadeTex = (label: string, lv: CascadeLevel, storage: boolean) =>
        d.createTexture({
          label,
          dimension: "3d",
          size: {
            width: lv.dims[0], height: lv.dims[1], depthOrArrayLayers: lv.dims[2],
          },
          format: "rgba16float",
          usage: GPUTextureUsage.TEXTURE_BINDING
            | (storage
              ? GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.COPY_SRC
              : GPUTextureUsage.COPY_DST),
        });
      const mergedTex: GPUTexture[] = [];
      for (let i = 0; i < this.cascadeLevels.length; i++) {
        const lv = this.cascadeLevels[i];
        const raw = cascadeTex(`cascade${i}-raw`, lv, true);
        const hist = cascadeTex(`cascade${i}-hist`, lv, false);
        const merged = d.createTexture({
          label: `cascade${i}-merged`,
          dimension: "3d",
          size: {
            width: lv.dims[0], height: lv.dims[1], depthOrArrayLayers: lv.dims[2],
          },
          format: "rgba16float",
          usage: GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.TEXTURE_BINDING,
        });
        this.cascadeRaw.push(raw);
        this.cascadeHist.push(hist);
        mergedTex.push(merged);
        this.cascadeRawViews.push(raw.createView({ dimension: "3d" }));
        this.cascadeHistViews.push(hist.createView({ dimension: "3d" }));
        this.cascadeMergedViews.push(merged.createView({ dimension: "3d" }));
        this.cascadeParams.push(d.createBuffer({
          label: `cascade-params-${i}`,
          size: CASCADE_PARAM_BYTES,
          usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
        }));
      }
      // Second-bounce irradiance, on cascade 0's probe grid: four texels per
      // probe (A, then B.x/B.y/B.z of the order-1 SH).
      const c0 = this.cascadeLevels[0];
      const shTex = d.createTexture({
        label: "cascade-sh",
        dimension: "3d",
        size: {
          width: c0.probes[0] * 4, height: c0.probes[1],
          depthOrArrayLayers: c0.probes[2],
        },
        format: "rgba16float",
        usage: GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.TEXTURE_BINDING,
      });
      this.cascadeShView = shTex.createView({ dimension: "3d" });
      this.cascadeIrradianceGroup = d.createBindGroup({
        label: "cascade-irradiance",
        layout: this.cascadeLayout,
        entries: [
          { binding: 0, resource: { buffer: this.cascadeParams[0] } },
          { binding: 1, resource: this.cascadeShView },
          { binding: 2, resource: this.cascadeHistViews[0] },
          { binding: 3, resource: this.cascadeHistViews[0] },
          // Reads last frame's merged cascade 0; the merge rewrites it later
          // in the frame, which is a different pass and so a real dependency
          // rather than a same-scope conflict.
          { binding: 4, resource: this.cascadeMergedViews[0] },
          { binding: 5, resource: this.cascadeHistViews[0] },
        ],
      });

      for (let i = 0; i < this.cascadeLevels.length; i++) {
        // The top cascade has nothing coarser. It cannot bind its own merged
        // texture as the coarse input — that is writable and readable in one
        // synchronisation scope, which WebGPU rejects — so it binds its history
        // instead, which the shader never reads because hasCoarser is 0.
        const top = i + 1 >= this.cascadeLevels.length;
        this.cascadeTraceGroups.push(d.createBindGroup({
          label: `cascade-trace-${i}`,
          layout: this.cascadeLayout,
          entries: [
            { binding: 0, resource: { buffer: this.cascadeParams[i] } },
            { binding: 1, resource: this.cascadeRawViews[i] },
            { binding: 2, resource: this.cascadeHistViews[i] },
            { binding: 3, resource: this.cascadeHistViews[i] },
            { binding: 4, resource: this.cascadeHistViews[i] },
            { binding: 5, resource: this.cascadeShView },
          ],
        }));
        this.cascadeMergeGroups.push(d.createBindGroup({
          label: `cascade-merge-${i}`,
          layout: this.cascadeLayout,
          entries: [
            { binding: 0, resource: { buffer: this.cascadeParams[i] } },
            { binding: 1, resource: this.cascadeMergedViews[i] },
            { binding: 2, resource: this.cascadeHistViews[i] },
            { binding: 3, resource: this.cascadeRawViews[i] },
            {
              binding: 4,
              resource: top ? this.cascadeHistViews[i] : this.cascadeMergedViews[i + 1],
            },
            { binding: 5, resource: this.cascadeShView },
          ],
        }));
      }
      // The two groups differ only in which half of radDyn is B-in vs B-out.
      for (let p = 0; p < 2; p++) {
        const params = d.createBuffer({
          label: `rad-params-${p}`, size: 16,
          usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
        });
        d.queue.writeBuffer(params, 0, new Uint32Array([
          N, S,
          p === 0 ? 4 * N : 8 * N,
          p === 0 ? 8 * N : 4 * N,
        ]));
        this.radBindGroups[p] = d.createBindGroup({
          label: `radiosity-${p}`,
          layout: this.radiosityLayout,
          entries: [
            { binding: 0, resource: { buffer: params } },
            { binding: 1, resource: { buffer: radStatic } },
            { binding: 2, resource: { buffer: radDyn } },
            { binding: 3, resource: this.flashmapView },
            { binding: 4, resource: this.radGSkyView },
          ],
        });
      }

      const t0 = performance.now();
      const enc = d.createCommandEncoder({ label: "radiosity-bake" });
      const pass = enc.beginComputePass({ label: "radiosity-bake" });
      pass.setBindGroup(0, this.sceneBindGroup);
      pass.setBindGroup(1, this.radBindGroups[0]);
      pass.setPipeline(this.radBakeFFPipeline);
      pass.dispatchWorkgroups(Math.ceil(N / 8), Math.ceil(N / 8), 1);
      pass.setPipeline(this.radBakeNormPipeline);
      pass.dispatchWorkgroups(Math.ceil(N / 64), 1, 1);
      pass.setPipeline(this.radBakeVisPipeline);
      pass.dispatchWorkgroups(Math.ceil(N / 64), this.staticLightCount, 1);
      pass.setPipeline(this.radBakeSkyPipeline);
      pass.dispatchWorkgroups(Math.ceil(N / 64), 1, 1);
      pass.end();
      d.queue.submit([enc.finish()]);
      await d.queue.onSubmittedWorkDone();
      console.log(
        `[radiosity] ${N} patches (${rad.patchSize.toFixed(2)}m), ` +
        `bake ${(performance.now() - t0).toFixed(0)}ms`,
      );
    }

    // ---- static light volume ------------------------------------------------
    {
      const t0 = performance.now();
      const enc = d.createCommandEncoder({ label: "lightvolume-bake" });
      this.encodeLightVolBake(enc);
      d.queue.submit([enc.finish()]);
      await d.queue.onSubmittedWorkDone();
      console.log(
        `[lightvolume] ${LIGHT_VOL_DIMS.join("x")} voxels, ` +
        `${this.staticLightCount} static lights, bake ${(performance.now() - t0).toFixed(0)}ms`,
      );
    }
  }

  private encodeLightVolBake(enc: GPUCommandEncoder): void {
    const pass = enc.beginComputePass({ label: "lightvolume-bake" });
    pass.setPipeline(this.lightVolPipeline);
    pass.setBindGroup(0, this.sceneBindGroup);
    pass.setBindGroup(1, this.lightVolBindGroup);
    pass.dispatchWorkgroups(
      Math.ceil(LIGHT_VOL_DIMS[0] / 4),
      Math.ceil(LIGHT_VOL_DIMS[1] / 4),
      Math.ceil(LIGHT_VOL_DIMS[2] / 4),
    );
    pass.end();
  }

  /**
   * One radiance-cascade step: retrace every probe, average against history,
   * then publish by copying live -> hist.
   *
   * Every probe every frame, with no update budget or round-robin. That is
   * affordable because a probe is one short ray per direction and nothing here
   * is quadratic in probe count — the property the patch solve does not have.
   */
  private encodeCascades(
    enc: GPUCommandEncoder,
    compute: (
      label: string, pipeline: GPUComputePipeline, bg1: GPUBindGroup | null,
      off: number[] | null, x: number, y: number,
      bg0?: GPUBindGroup, z?: number,
    ) => void,
  ): void {
    const levels = this.cascadeLevels;
    const wg = 4;
    const groups = (lv: CascadeLevel) => [
      Math.ceil(lv.dims[0] / wg), Math.ceil(lv.dims[1] / wg), Math.ceil(lv.dims[2] / wg),
    ] as const;

    for (let i = 0; i < levels.length; i++) {
      const lv = levels[i];
      const f = new Float32Array(CASCADE_PARAM_BYTES / 4);
      const u = new Uint32Array(f.buffer);
      // MEDIUM_ORIGIN, not the smoke lattice's. The cascades are sized by
      // cascadeLevels() from SMOKE_DIMS/SMOKE_CELL — the level's — so their
      // probe grid covers the level's box and the origin has to match it. A
      // demo page that shrinks the smoke lattice must not drag this with it:
      // doing so put every probe in the wrong place and the screen went black.
      f[0] = MEDIUM_ORIGIN[0]; f[1] = MEDIUM_ORIGIN[1]; f[2] = MEDIUM_ORIGIN[2];
      u[3] = lv.res;
      f[4] = lv.spacing[0]; f[5] = lv.spacing[1]; f[6] = lv.spacing[2];
      f[7] = lv.tStart;
      u[8] = lv.probes[0]; u[9] = lv.probes[1]; u[10] = lv.probes[2];
      f[11] = lv.tEnd * this.cascades.reach;
      // First frame has no history to average against, so take the trace whole.
      f[12] = this.cascadeFrame === 0 ? 1.0 : this.cascades.alpha;
      u[13] = this.cascadeFrame;
      u[14] = i + 1 < levels.length ? 1 : 0;
      u[15] = Math.max(1, Math.round(this.cascades.candidates));
      this.device.queue.writeBuffer(this.cascadeParams[i], 0, f);
    }
    this.cascadeFrame++;

    // Second-bounce irradiance FIRST, from last frame's merged cascade 0. It
    // has to precede the traces that consume it, and it reads a texture the
    // merge below rewrites — one frame of lag, which is the recursion.
    {
      const c0 = levels[0];
      compute(
        "cascadeIrradiance", this.cascadeIrradiancePipeline,
        this.cascadeIrradianceGroup, null,
        Math.ceil(c0.probes[0] / wg), Math.ceil(c0.probes[1] / wg),
        this.sceneBindGroup, Math.ceil(c0.probes[2] / wg),
      );
    }

    // Trace every level, then publish each raw field as next frame's history.
    for (let i = 0; i < levels.length; i++) {
      const [gx, gy, gz] = groups(levels[i]);
      compute(
        `cascadeTrace${i}`, this.cascadeTracePipeline, this.cascadeTraceGroups[i],
        null, gx, gy, this.sceneBindGroup, gz,
      );
    }
    for (let i = 0; i < levels.length; i++) {
      const lv = levels[i];
      enc.copyTextureToTexture(
        { texture: this.cascadeRaw[i] }, { texture: this.cascadeHist[i] },
        { width: lv.dims[0], height: lv.dims[1], depthOrArrayLayers: lv.dims[2] },
      );
    }
    // Merge from the top down: level i reads level i+1's MERGED result, so the
    // far cascades have to be finished before the near ones consume them.
    for (let i = levels.length - 1; i >= 0; i--) {
      const [gx, gy, gz] = groups(levels[i]);
      compute(
        `cascadeMerge${i}`, this.cascadeMergePipeline, this.cascadeMergeGroups[i],
        null, gx, gy, this.sceneBindGroup, gz,
      );
    }
  }

  /**
   * Pushes `denoise` into the a-trous and reproject uniform slots.
   *
   * Called once at init and from the panel on change, not per frame: these are
   * five small writes and nothing about them is frame-varying, so paying for
   * them every frame would only re-introduce the duplicate-owner bug the fluid
   * tuning already ran into.
   */
  applyDenoise(): void {
    const d = this.device;
    const t = this.denoise;
    // Indirect fireflies are already clamped at the source (luminance 3), so
    // this pass only has to catch the extremes, and its band must not collapse
    // when the local neighbourhood happens to be empty.
    d.queue.writeBuffer(this.reprojectBuffer, REPROJECT_STRIDE, new Float32Array([
      6.0, 2.0, t.indClampK, t.indClampFloor, 1.0, t.indHistory, t.indAlphaFloor, 0,
    ]));

    // The first ATROUS_ITERS slots are the direct chain, the next ATROUS_ITERS
    // the indirect chain. Indirect drops luminance edge-stopping entirely and
    // relaxes the geometric terms, which is the whole point of separating the
    // signals; it also strides wider, because bounce light is low frequency.
    // A pass past its chain's count gets stride 0 — see DenoiseTuning.
    for (let i = 0; i < ATROUS_ITERS; i++) {
      const direct = new ArrayBuffer(ATROUS_PARAM_SIZE);
      new Int32Array(direct, 0, 1)[0] = i < t.atrousDirect ? 1 << i : 0;
      new Float32Array(direct, 4, 2).set([1.0, 1.0]);
      d.queue.writeBuffer(this.atrousBuffer, i * ATROUS_STRIDE, direct);

      const indirect = new ArrayBuffer(ATROUS_PARAM_SIZE);
      new Int32Array(indirect, 0, 1)[0] = i < t.atrousIndirect ? 2 << i : 0;
      new Float32Array(indirect, 4, 2).set([0.0, 3.0]);
      d.queue.writeBuffer(
        this.atrousBuffer, (ATROUS_ITERS + i) * ATROUS_STRIDE, indirect,
      );
    }
  }

  /**
   * Smoke density at a world point, from the coarse GPU readback.
   *
   * Trilinear over the 1 m x 0.25 m x 1 m coarse grid; 0 outside the room's
   * air. This is the gameplay view of the medium (guard line-of-sight, the
   * light gauge): it lags the frame by a few and it is smoke ONLY — the
   * ambient fog is texture, not concealment. Zero everywhere until the fluid
   * simulation (or the test blob) writes the smoke volume.
   */
  sampleSmokeDensityCPU(x: number, y: number, z: number): number {
    const [nx, ny, nz] = this.smokeCoarseDims;
    const cx = this.smokeCellSize * SMOKE_COARSE_FACTOR;
    const cy = this.smokeCellSize;
    // Grid coordinates of the sample point, in cell units, centre-aligned.
    const gx = (x - this.smokeOrigin[0]) / cx - 0.5;
    const gy = (y - this.smokeOrigin[1]) / cy - 0.5;
    const gz = (z - this.smokeOrigin[2]) / cx - 0.5;
    if (gx < -0.5 || gy < -0.5 || gz < -0.5 || gx > nx - 0.5 || gy > ny - 0.5 || gz > nz - 0.5) {
      return 0;
    }
    const x0 = Math.max(0, Math.min(nx - 2, Math.floor(gx)));
    const y0 = Math.max(0, Math.min(ny - 2, Math.floor(gy)));
    const z0 = Math.max(0, Math.min(nz - 2, Math.floor(gz)));
    const fx = Math.min(1, Math.max(0, gx - x0));
    const fy = Math.min(1, Math.max(0, gy - y0));
    const fz = Math.min(1, Math.max(0, gz - z0));
    const d = this.smokeCoarse;
    const at = (i: number, j: number, k: number) => d[(k * ny + j) * nx + i];
    const c00 = at(x0, y0, z0) * (1 - fx) + at(x0 + 1, y0, z0) * fx;
    const c10 = at(x0, y0 + 1, z0) * (1 - fx) + at(x0 + 1, y0 + 1, z0) * fx;
    const c01 = at(x0, y0, z0 + 1) * (1 - fx) + at(x0 + 1, y0, z0 + 1) * fx;
    const c11 = at(x0, y0 + 1, z0 + 1) * (1 - fx) + at(x0 + 1, y0 + 1, z0 + 1) * fx;
    return (c00 * (1 - fy) + c10 * fy) * (1 - fz) + (c01 * (1 - fy) + c11 * fy) * fz;
  }

  /**
   * Re-bakes the static light volume at the start of the next frame.
   *
   * Called whenever a static light's intensity changes (the OCP darkening a
   * lamp, a shot-out fixture, a restore); several changes in one frame
   * coalesce into a single whole-volume dispatch.
   */
  rebakeLightVolume(): void {
    this.lightVolDirty = true;
  }

  /**
   * Uploads this frame's animated geometry. Packed with the same Box layout.
   *
   * Reprojection reads prevDynBoxes[i] as "where box i was last frame", which
   * only holds while the packing order is stable across frames: the player's
   * group first, guards after it in a fixed order, particles last. Boxes from
   * `unstableFrom` onward are the particle range — swap-remove reshuffles them
   * every frame, so index i last frame is usually a different particle.
   */
  updateDynamic(
    data: Float32Array<ArrayBuffer>, count: number, unstableFrom = count,
  ): void {
    const first = this.dynCount === 0;
    this.dynCount = Math.min(count, MAX_DYN_BOXES);
    if (this.dynCount === 0) return;
    const floats = this.dynCount * BOX_STRIDE_F32;

    // On the first frame there is no history; treat the character as having
    // been stationary rather than reprojecting against uninitialised memory.
    if (first) this.prevDynData.set(data.subarray(0, floats));

    // Particles have no stable identity, so reproject each against itself
    // (current as previous): they lose their motion history but never inherit
    // an unrelated particle's transform.
    const unstable = Math.max(0, Math.min(unstableFrom, this.dynCount)) * BOX_STRIDE_F32;
    if (unstable < floats) this.prevDynData.set(data.subarray(unstable, floats), unstable);

    this.device.queue.writeBuffer(this.prevDynBuffer, 0, this.prevDynData, 0, floats);
    this.prevDynData.set(data.subarray(0, floats));
    this.device.queue.writeBuffer(this.dynBuffer, 0, data, 0, floats);

    // Per-group AABBs, so a ray that misses a character skips all 24 of its
    // boxes with one slab test. Groups are a fixed stride rather than real
    // character boundaries: buildBoxes writes each character contiguously so
    // they line up anyway, and this keeps the renderer from having to know
    // anything about how the game packs its characters.
    this.dynGroupCount = Math.min(
      Math.ceil(this.dynCount / DYN_GROUP_SIZE), DYN_GROUPS,
    );
    const gLo = this.dynGroupMin;
    const gHi = this.dynGroupMax;
    for (let g = 0; g < this.dynGroupCount; g++) {
      const start = g * DYN_GROUP_SIZE;
      const end = Math.min(start + DYN_GROUP_SIZE, this.dynCount);
      const b = g * 4;
      gLo[b] = Infinity; gLo[b + 1] = Infinity; gLo[b + 2] = Infinity;
      gHi[b] = -Infinity; gHi[b + 1] = -Infinity; gHi[b + 2] = -Infinity;
      for (let i = start; i < end; i++) {
        const o = i * BOX_STRIDE_F32;
        for (let a = 0; a < 3; a++) {
          // World extent along axis a is the half-extents projected through the
          // box's rotation, matching boxBounds() on the CPU side.
          const e =
            Math.abs(data[o + 8 + a]) * data[o + 4] +
            Math.abs(data[o + 12 + a]) * data[o + 5] +
            Math.abs(data[o + 16 + a]) * data[o + 6];
          gLo[b + a] = Math.min(gLo[b + a], data[o + a] - e);
          gHi[b + a] = Math.max(gHi[b + a], data[o + a] + e);
        }
      }
    }
  }

  // -------------------------------------------------------------------------

  /**
   * Overrides a material's emissive colour.
   *
   * The companion to setStaticLightIntensity: a light and the fixture it shines
   * out of are separate objects, so darkening one without the other leaves a
   * sign that still glows but illuminates nothing.
   */
  setMaterialEmissive(index: number, r: number, g: number, b: number): void {
    this.matScratch3[0] = r;
    this.matScratch3[1] = g;
    this.matScratch3[2] = b;
    this.device.queue.writeBuffer(
      this.matBuffer,
      (index * MATERIAL_STRIDE_F32 + 4) * 4,
      this.matScratch3,
    );
  }

  /**
   * Overrides a static light's intensity, for the OCP.
   *
   * Static lights are uploaded once at init, so this rewrites a single slot in
   * place. The light stays in the array at zero rather than being compacted
   * out: the indices are load-bearing (lights[0] is the moon, on its own
   * channel) and shifting them would invalidate every reservoir carrying one.
   *
   * The cost of that choice is that a disabled light still occupies a candidate
   * slot in the RIS pool. With a handful disabled out of ~30 that is a few
   * percent of wasted candidates, which is cheaper than the alternative.
   */
  setStaticLightIntensity(index: number, intensity: number): void {
    if (index < 0 || index >= this.staticLightCount) return;
    this.lightScratch1[0] = intensity;
    this.device.queue.writeBuffer(
      this.lightBuffer,
      (index * LIGHT_STRIDE_F32 + 11) * 4,
      this.lightScratch1,
    );
    // The baked light volume holds this light at its old intensity.
    this.rebakeLightVolume();
  }

  /**
   * Sets which dynamic group owns each torch depth-map layer, so the map's
   * trace can skip its owner (flashmap.wgsl). `playerGroup` owns layer 0;
   * `guardGroups[i]` owns layer i+1, in the same order updateLights receives
   * the steady lights. Layers past the list skip nothing.
   */
  setTorchGroups(playerGroup: number, guardGroups: number[]): void {
    const t = this.torchGroups;
    t.fill(DYN_GROUP_NONE);
    t[0] = playerGroup;
    const n = Math.min(guardGroups.length, TORCH_LAYERS - 1);
    for (let i = 0; i < n; i++) t[i + 1] = guardGroups[i];
    this.device.queue.writeBuffer(this.flashmapParamsBuffer, 0, t);
  }

  /**
   * Replaces the dynamic tail of the light buffer.
   *
   * Static lights are written once at init and never touched; this only rewrites
   * the region past them, so the moon stays at index 0 where the key-light
   * channel expects it. Call once per frame, before render().
   */
  updateLights(steady: Light[], transient: Light[] = []): void {
    // Transients must occupy a contiguous tail: the shader splits the array by
    // index rather than testing a per-light flag, so the split point has to be
    // a single number. Writing them here rather than trusting the caller to
    // concatenate in the right order is what keeps that true.
    const nS = Math.min(steady.length, MAX_DYN_LIGHTS);
    const nT = Math.min(transient.length, MAX_DYN_LIGHTS - nS);
    const want = steady.length + transient.length;
    if (nS + nT < want) {
      console.warn(
        `[renderer] ${want} dynamic lights exceeds MAX_DYN_LIGHTS ` +
          `(${MAX_DYN_LIGHTS}); ${want - nS - nT} dropped.`,
      );
    }
    const lights = this.lightScratch;
    lights.length = 0;
    for (let i = 0; i < nS; i++) lights.push(steady[i]);
    for (let i = 0; i < nT; i++) lights.push(transient[i]);
    const n = lights.length;
    this.lightCount = this.staticLightCount + n;
    this.transientStart = this.staticLightCount + nS;
    if (n === 0) return;

    // packLights allocates, so pack into the preallocated scratch by hand.
    const o = this.dynLightData;
    const u = new Uint32Array(o.buffer);
    for (let i = 0; i < n; i++) {
      const l = lights[i];
      const b = i * LIGHT_STRIDE_F32;
      o[b + 0] = l.pos.x; o[b + 1] = l.pos.y; o[b + 2] = l.pos.z;
      u[b + 3] = l.kind;
      o[b + 4] = l.dir.x; o[b + 5] = l.dir.y; o[b + 6] = l.dir.z;
      o[b + 7] = l.radius;
      o[b + 8] = l.color.x; o[b + 9] = l.color.y; o[b + 10] = l.color.z;
      o[b + 11] = l.intensity;
      o[b + 12] = l.cosInner;
      o[b + 13] = l.cosOuter;
      o[b + 16] = l.halfExtents ? l.halfExtents.x : 0;
      o[b + 17] = l.halfExtents ? l.halfExtents.y : 0;
      o[b + 18] = l.halfExtents ? l.halfExtents.z : 0;
    }
    this.device.queue.writeBuffer(
      this.lightBuffer,
      this.staticLightCount * LIGHT_STRIDE_F32 * 4,
      o, 0, n * LIGHT_STRIDE_F32,
    );
  }

  /**
   * Reads the composited HDR image back to the CPU as linear RGB.
   *
   * This is what makes "is it correct?" answerable instead of a judgement call:
   * two configurations can be diffed numerically rather than compared by eye,
   * which matters most for errors that converge smoothly to the wrong answer.
   */
  /**
   * Rewrites the transient chain's filter parameters.
   *
   * Unlike the other two chains this is driven per frame rather than written
   * once, because the mode and its shape are on the debug panel — the choice
   * between the two approaches is a look, and looks have to be A/B'd live.
   *
   * Transient is spatial only, so it leans harder on the filter than either
   * accumulated signal: there is no history to fall back on.
   */
  private writeTransientParams(settings: RenderSettings): void {
    for (let i = 0; i < TRANS_ATROUS_ITERS; i++) {
      const trans = new ArrayBuffer(ATROUS_PARAM_SIZE);
      new Int32Array(trans, 0, 1)[0] = 1 << i;
      new Float32Array(trans, 4, 5).set([
        0.0,                            // lumaWeight: never, on this signal
        4.0,                            // edgeRelax
        settings.transientFilter,
        settings.transientBlurStride,
        settings.transientBlurStrength,
      ]);
      this.device.queue.writeBuffer(
        this.atrousBuffer, (ATROUS_ITERS * 2 + i) * ATROUS_STRIDE, trans,
      );
    }
  }

  async readHDR(): Promise<{ width: number; height: number; data: Float32Array }> {
    const t = this.targets;
    if (!t) throw new Error("no render targets");
    return this.readF16(t.hdr, t.width, t.height);
  }

  /**
   * The direct-signal moments written by the most recent frame's
   * reprojection: (mean luma, mean luma^2, history length, variance).
   *
   * A debug probe for temporal accumulation — z is the effective sample
   * count of the history blended into the pixel (a bilinear blend of the taps'
   * counts, not a private per-pixel timer), so against a scenario whose
   * subject has a known age it exposes both shed history (too short) and
   * borrowed history (longer than the subject has existed). Parity has already
   * flipped, so the just-written buffer is the "previous" slot.
   */
  async readMoments(): Promise<{ width: number; height: number; data: Float32Array }> {
    const t = this.targets;
    if (!t) throw new Error("no render targets");
    return this.readF16(t.momentsHist[1 - this.parity], t.width, t.height);
  }

  /**
   * The current frame's reprojection G-buffer: xyz previous-frame world
   * position, w surface class (0 miss / 1 static / 2 dynamic). A debug probe:
   * the class channel is what lets a history statistic be taken over the
   * animated geometry's own pixels rather than a screen box around it.
   */
  async readPos(): Promise<{ width: number; height: number; data: Float32Array }> {
    const t = this.targets;
    if (!t) throw new Error("no render targets");
    const w = t.width, h = t.height;
    const bpr = Math.ceil((w * 16) / 256) * 256;
    const staging = this.device.createBuffer({
      label: "f32-readback",
      size: bpr * h,
      usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
    });
    const enc = this.device.createCommandEncoder({ label: "readback" });
    enc.copyTextureToBuffer(
      { texture: t.pos }, { buffer: staging, bytesPerRow: bpr }, { width: w, height: h },
    );
    this.device.queue.submit([enc.finish()]);
    await staging.mapAsync(GPUMapMode.READ);
    const raw = new Float32Array(staging.getMappedRange().slice(0));
    staging.unmap();
    staging.destroy();
    const out = new Float32Array(w * h * 4);
    const strideF32 = bpr / 4;
    for (let y = 0; y < h; y++) {
      out.set(raw.subarray(y * strideF32, y * strideF32 + w * 4), y * w * 4);
    }
    return { width: w, height: h, data: out };
  }

  private async readF16(
    tex: GPUTexture, w: number, h: number,
  ): Promise<{ width: number; height: number; data: Float32Array }> {
    // Texture-to-buffer copies need rows aligned to 256 bytes.
    const bpr = Math.ceil((w * 8) / 256) * 256;
    const staging = this.device.createBuffer({
      label: "f16-readback",
      size: bpr * h,
      usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
    });
    const enc = this.device.createCommandEncoder({ label: "readback" });
    enc.copyTextureToBuffer(
      { texture: tex }, { buffer: staging, bytesPerRow: bpr }, { width: w, height: h },
    );
    this.device.queue.submit([enc.finish()]);
    await staging.mapAsync(GPUMapMode.READ);
    const raw = new Uint16Array(staging.getMappedRange().slice(0));
    staging.unmap();
    staging.destroy();

    const out = new Float32Array(w * h * 4);
    const strideU16 = bpr / 2;
    for (let y = 0; y < h; y++) {
      for (let x = 0; x < w * 4; x++) {
        out[y * w * 4 + x] = halfToFloat(raw[y * strideU16 + x]);
      }
    }
    return { width: w, height: h, data: out };
  }

  /**
   * Reads one torch depth-map layer back as FLASHMAP_RES^2 radial depths.
   *
   * Diagnostic for the headless harness: the map's coverage (how much of a
   * layer reads near-zero depth) is the quantity the crouch bug is about, and
   * it is otherwise invisible except through the light it kills.
   */
  async readFlashmapLayer(layer = 0): Promise<Float32Array> {
    const n = FLASHMAP_RES;
    const bpr = Math.ceil((n * 4) / 256) * 256;
    const staging = this.device.createBuffer({
      label: "flashmap-readback",
      size: bpr * n,
      usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
    });
    const enc = this.device.createCommandEncoder({ label: "flashmap-readback" });
    enc.copyTextureToBuffer(
      { texture: this.flashmapTexture, origin: [0, 0, layer] },
      { buffer: staging, bytesPerRow: bpr },
      { width: n, height: n },
    );
    this.device.queue.submit([enc.finish()]);
    await staging.mapAsync(GPUMapMode.READ);
    const raw = new Float32Array(staging.getMappedRange().slice(0));
    staging.unmap();
    staging.destroy();
    const out = new Float32Array(n * n);
    const stride = bpr / 4;
    for (let y = 0; y < n; y++) out.set(raw.subarray(y * stride, y * stride + n), y * n);
    return out;
  }

  /**
   * Reads the radiosity solve state back: injected energy E and the two B
   * ping-pong halves, each patchCount vec3s (4-float stride).
   *
   * Diagnostic for the headless harness — "how much light did the flashlight
   * actually inject into the patches" is the crouch bug's real quantity, and
   * the image only shows its far downstream effect.
   */
  async readRadiosity(): Promise<{ count: number; data: Float32Array }> {
    const n = this.radPatchCount;
    const bytes = Math.max(16, 12 * n * 4);
    const staging = this.device.createBuffer({
      label: "rad-readback",
      size: bytes,
      usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
    });
    const enc = this.device.createCommandEncoder({ label: "rad-readback" });
    enc.copyBufferToBuffer(this.radDynBuffer, 0, staging, 0, bytes);
    this.device.queue.submit([enc.finish()]);
    await staging.mapAsync(GPUMapMode.READ);
    const data = new Float32Array(staging.getMappedRange().slice(0));
    staging.unmap();
    staging.destroy();
    return { count: n, data };
  }

  /**
   * Sets the world points to measure illumination at on the next frame.
   *
   * `skipGroup` is the dynamic-box group the probe belongs to, excluded from
   * the shadow test so the character does not shadow its own probe.
   *
   * Results appear in `probeLuma` a frame or two later — the readback is
   * asynchronous so it never stalls the pipeline. Anything needing a
   * frame-exact answer must not use this.
   */
  setProbes(points: Vec3[], skipGroup = 0): void {
    const n = Math.min(points.length, MAX_PROBES);
    this.probeCount = n;
    for (let i = 0; i < n; i++) {
      this.probeParams[i * 4] = points[i].x;
      this.probeParams[i * 4 + 1] = points[i].y;
      this.probeParams[i * 4 + 2] = points[i].z;
    }
    const u = new Uint32Array(this.probeParams.buffer);
    u[MAX_PROBES * 4] = n;
    // The player is dynamic group 0 — see updateDynamic's fixed-stride grouping.
    u[MAX_PROBES * 4 + 1] = skipGroup;
    this.device.queue.writeBuffer(this.probeParamsBuffer, 0, this.probeParams);
  }

  private makeTex(
    label: string, w: number, h: number, format: GPUTextureFormat, copySrc = false,
  ): GPUTexture {
    return this.device.createTexture({
      label,
      size: { width: Math.max(1, w), height: Math.max(1, h) },
      format,
      // COPY_SRC only where it is actually needed. It is not free on every
      // target; the composited HDR image and the direct moments (temporal
      // history probe) are the only ones ever read back.
      usage: GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.TEXTURE_BINDING |
        (copySrc ? GPUTextureUsage.COPY_SRC : 0),
    });
  }

  resize(width: number, height: number): void {
    const w = Math.max(1, Math.floor(width));
    const h = Math.max(1, Math.floor(height));
    if (this.targets && this.targets.width === w && this.targets.height === h) return;

    if (this.targets) {
      for (const t of this.allTextures(this.targets)) t.destroy();
      this.targets.reservoirs.destroy();
      this.targets.giReservoirs.destroy();
    }

    const f16 = "rgba16float" as const;
    const bloomDown: GPUTexture[] = [];
    const bloomUp: GPUTexture[] = [];
    for (let i = 0; i < BLOOM_MIPS; i++) {
      const s = 1 << (i + 1);
      bloomDown.push(this.makeTex(`bloom-down-${i}`, Math.ceil(w / s), Math.ceil(h / s), f16));
    }
    for (let i = 0; i < BLOOM_MIPS - 1; i++) {
      const s = 1 << (i + 1);
      bloomUp.push(this.makeTex(`bloom-up-${i}`, Math.ceil(w / s), Math.ceil(h / s), f16));
    }

    // Direct / indirect / transient radiance as three layers of one texture:
    // one storage-texture slot in the trace pass instead of three.
    const illumArray = this.device.createTexture({
      label: "illum-raw-array",
      size: { width: w, height: h, depthOrArrayLayers: RAW_ILLUM_LAYERS },
      format: f16,
      usage: GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.TEXTURE_BINDING,
    });
    const layer = (i: number, label: string) => illumArray.createView({
      label, dimension: "2d", baseArrayLayer: i, arrayLayerCount: 1,
    });

    const t: Targets = {
      width: w,
      height: h,
      illumArray,
      albedo: this.makeTex("g-albedo", w, h, "rgba8unorm"),
      normalDepth: [
        this.makeTex("g-nd-0", w, h, f16),
        this.makeTex("g-nd-1", w, h, f16),
      ],
      pos: this.makeTex("g-pos", w, h, "rgba32float", true),
      illumRaw: layer(0, "illum-raw"),
      accumIllum: this.makeTex("illum-accum", w, h, f16),
      illumHist: [this.makeTex("illum-hist-0", w, h, f16), this.makeTex("illum-hist-1", w, h, f16)],
      momentsHist: [
        this.makeTex("moments-hist-0", w, h, f16, true), this.makeTex("moments-hist-1", w, h, f16, true),
      ],
      scratch: [this.makeTex("scratch-0", w, h, f16), this.makeTex("scratch-1", w, h, f16)],
      momentsScratch: [this.makeTex("m-scratch-0", w, h, f16), this.makeTex("m-scratch-1", w, h, f16)],
      transRaw: layer(2, "trans-raw"),
      transScratch: [
        this.makeTex("trans-s-0", w, h, f16), this.makeTex("trans-s-1", w, h, f16),
      ],
      transMoments: [
        this.makeTex("trans-m-0", w, h, f16), this.makeTex("trans-m-1", w, h, f16),
      ],
      indRaw: layer(1, "ind-raw"),
      indAccum: this.makeTex("ind-accum", w, h, f16),
      indHist: [this.makeTex("ind-hist-0", w, h, f16), this.makeTex("ind-hist-1", w, h, f16)],
      indMoments: [this.makeTex("ind-m-0", w, h, f16), this.makeTex("ind-m-1", w, h, f16)],
      indScratch: [this.makeTex("ind-s-0", w, h, f16), this.makeTex("ind-s-1", w, h, f16)],
      indMomentsScratch: [
        this.makeTex("ind-ms-0", w, h, f16), this.makeTex("ind-ms-1", w, h, f16),
      ],
      volRaw: layer(3, "vol-raw"),
      volAccum: this.makeTex("vol-accum", w, h, f16),
      volHist: [this.makeTex("vol-hist-0", w, h, f16), this.makeTex("vol-hist-1", w, h, f16)],
      volMoments: [this.makeTex("vol-m-0", w, h, f16), this.makeTex("vol-m-1", w, h, f16)],
      volScratch: [this.makeTex("vol-s-0", w, h, f16), this.makeTex("vol-s-1", w, h, f16)],
      volMomentsScratch: [
        this.makeTex("vol-ms-0", w, h, f16), this.makeTex("vol-ms-1", w, h, f16),
      ],
      hdr: this.makeTex("hdr", w, h, f16, true),
      bloomDown,
      bloomUp,
      // Two parity halves each — see Targets.reservoirs.
      reservoirs: this.device.createBuffer({
        label: "restir-reservoirs",
        size: w * h * RESERVOIR_BYTES * 2,
        usage: GPUBufferUsage.STORAGE,
      }),
      giReservoirs: this.device.createBuffer({
        label: "restir-gi-reservoirs",
        size: w * h * GI_RESERVOIR_BYTES * 2,
        usage: GPUBufferUsage.STORAGE,
      }),
    };
    this.targets = t;
    this.frameIndex = 0;
    this.bloomParamsDirty = true;
    this.buildBindGroups(t);
  }

  private allTextures(t: Targets): GPUTexture[] {
    return [
      t.albedo, ...t.normalDepth, t.pos, t.illumArray, t.accumIllum,
      ...t.illumHist, ...t.momentsHist, ...t.scratch, ...t.momentsScratch,
      t.indAccum, ...t.indHist, ...t.indMoments,
      ...t.indScratch, ...t.indMomentsScratch,
      ...t.transScratch, ...t.transMoments,
      t.volAccum, ...t.volHist, ...t.volMoments,
      ...t.volScratch, ...t.volMomentsScratch,
      t.hdr, ...t.bloomDown, ...t.bloomUp,
    ];
  }

  private buildBindGroups(t: Targets): void {
    const d = this.device;
    // The pathtrace group holds a view of the fine lattice's density texture,
    // and buildGrid DESTROYS and reallocates that texture on reset(). Record
    // which allocation these groups were built against so `frame` can notice.
    this.fineGeneration = this.fluidFine ? this.fluidFine.generation : -1;
    /** Textures get a default view; a value that is already a view passes through. */
    const v = (tex: TexOrView) => ("createView" in tex ? tex.createView() : tex);

    this.ptBindGroups = [];
    this.reprojectBindGroups = [];
    this.atrousBindGroups = [];
    this.indReprojectBindGroups = [];
    this.indAtrousBindGroups = [];
    this.transAtrousBindGroups = [];
    this.volReprojectBindGroups = [];
    this.refVolReprojectBindGroups = [];
    this.volAtrousBindGroups = [];
    this.compositeBindGroups = [];
    this.refCompositeBindGroups = [];
    this.refReprojectBindGroups = [];
    this.refIndReprojectBindGroups = [];

    for (let p = 0; p < 2; p++) {
      const cur = p;
      const prev = 1 - p;

      this.ptBindGroups[p] = d.createBindGroup({
        label: `pathtrace-${p}`,
        layout: this.ptLayout,
        entries: [
          { binding: 0, resource: v(t.albedo) },
          { binding: 1, resource: v(t.normalDepth[cur]) },
          { binding: 2, resource: v(t.pos) },
          { binding: 3, resource: t.illumArray.createView({ dimension: "2d-array" }) },
          { binding: 4, resource: v(t.normalDepth[prev]) },
          { binding: 5, resource: { buffer: t.reservoirs } },
          { binding: 6, resource: { buffer: this.workCounters.buffer } },
          { binding: 8, resource: { buffer: t.giReservoirs } },
          { binding: 11, resource: this.flashmapView },
          { binding: 12, resource: this.radGSkyView },
          { binding: 13, resource: this.radFaceView },
          { binding: 10, resource: this.lightVolView },
          { binding: 14, resource: this.smokeVolumeView },
          {
            binding: 7,
            resource: this.fluidFine.densityTexture.createView({ dimension: "3d" }),
          },
          { binding: 15, resource: this.sampler },
          { binding: 16, resource: { buffer: this.radStaticBuffer } },
          { binding: 9, resource: this.cascadeMergedViews[0] },
        ],
      });

      this.reprojectBindGroups[p] = d.createBindGroup({
        label: `reproject-${p}`,
        layout: this.reprojectLayout,
        entries: [
          { binding: 0, resource: t.illumRaw },
          { binding: 1, resource: v(t.pos) },
          { binding: 2, resource: v(t.normalDepth[cur]) },
          { binding: 3, resource: v(t.illumHist[prev]) },
          { binding: 4, resource: v(t.momentsHist[prev]) },
          { binding: 5, resource: v(t.normalDepth[prev]) },
          { binding: 6, resource: v(t.accumIllum) },
          { binding: 7, resource: v(t.momentsHist[cur]) },
          { binding: 8, resource: { buffer: this.reprojectBuffer, size: 32 } },
        ],
      });

      this.indReprojectBindGroups[p] = d.createBindGroup({
        label: `reproject-ind-${p}`,
        layout: this.reprojectLayout,
        entries: [
          { binding: 0, resource: t.indRaw },
          { binding: 1, resource: v(t.pos) },
          { binding: 2, resource: v(t.normalDepth[cur]) },
          { binding: 3, resource: v(t.indHist[prev]) },
          { binding: 4, resource: v(t.indMoments[prev]) },
          { binding: 5, resource: v(t.normalDepth[prev]) },
          { binding: 6, resource: v(t.indAccum) },
          { binding: 7, resource: v(t.indMoments[cur]) },
          { binding: 8, resource: { buffer: this.reprojectBuffer, size: 32 } },
        ],
      });

      this.volReprojectBindGroups[p] = d.createBindGroup({
        label: `reproject-vol-${p}`,
        layout: this.reprojectLayout,
        entries: [
          { binding: 0, resource: t.volRaw },
          { binding: 1, resource: v(t.pos) },
          { binding: 2, resource: v(t.normalDepth[cur]) },
          { binding: 3, resource: v(t.volHist[prev]) },
          { binding: 4, resource: v(t.volMoments[prev]) },
          { binding: 5, resource: v(t.normalDepth[prev]) },
          { binding: 6, resource: v(t.volAccum) },
          { binding: 7, resource: v(t.volMoments[cur]) },
          { binding: 8, resource: { buffer: this.reprojectBuffer, size: 32 } },
        ],
      });

      // a-trous chain. Iteration 0's output doubles as next frame's colour
      // history, which is what makes SVGF converge in a handful of frames
      // rather than dozens.
      const mkChain = (
        accum: TexOrView, mCur: TexOrView, hist: TexOrView,
        sc: [TexOrView, TexOrView], ms: [TexOrView, TexOrView],
      ): Array<[TexOrView, TexOrView, TexOrView, TexOrView]> => [
        [accum, mCur, hist, ms[0]],
        [hist, ms[0], sc[0], ms[1]],
        [sc[0], ms[1], sc[1], ms[0]],
        [sc[1], ms[0], sc[0], ms[1]],
      ];
      const mkGroups = (
        chain: Array<[TexOrView, TexOrView, TexOrView, TexOrView]>, tag: string,
      ) =>
        chain.map(([si, mi, so, mo], i) =>
          d.createBindGroup({
            label: `atrous-${tag}-${p}-${i}`,
            layout: this.atrousLayout,
            entries: [
              { binding: 0, resource: v(si) },
              { binding: 1, resource: v(mi) },
              { binding: 2, resource: v(t.normalDepth[cur]) },
              { binding: 3, resource: v(so) },
              { binding: 4, resource: v(mo) },
              { binding: 5, resource: { buffer: this.atrousBuffer, size: ATROUS_PARAM_SIZE } },
            ],
          }),
        );

      this.atrousBindGroups[p] = mkGroups(
        mkChain(
          t.accumIllum, t.momentsHist[cur], t.illumHist[cur],
          t.scratch, t.momentsScratch,
        ),
        "dir",
      );
      this.transAtrousBindGroups[p] = mkGroups(
        [
          [t.transRaw, t.transMoments[0], t.transScratch[0], t.transMoments[1]],
          [t.transScratch[0], t.transMoments[1], t.transScratch[1], t.transMoments[0]],
        ] as Array<[TexOrView, TexOrView, TexOrView, TexOrView]>,
        "trans",
      );
      this.indAtrousBindGroups[p] = mkGroups(
        mkChain(
          t.indAccum, t.indMoments[cur], t.indHist[cur],
          t.indScratch, t.indMomentsScratch,
        ),
        "ind",
      );
      this.volAtrousBindGroups[p] = mkGroups(
        mkChain(
          t.volAccum, t.volMoments[cur], t.volHist[cur],
          t.volScratch, t.volMomentsScratch,
        ),
        "vol",
      );

      const reprojectEntries = (
        raw: TexOrView, histIn: GPUTexture, momIn: GPUTexture,
        out: GPUTexture, momOut: GPUTexture,
      ) => [
        { binding: 0, resource: v(raw) },
        { binding: 1, resource: v(t.pos) },
        { binding: 2, resource: v(t.normalDepth[cur]) },
        { binding: 3, resource: v(histIn) },
        { binding: 4, resource: v(momIn) },
        { binding: 5, resource: v(t.normalDepth[prev]) },
        { binding: 6, resource: v(out) },
        { binding: 7, resource: v(momOut) },
        { binding: 8, resource: { buffer: this.reprojectBuffer, size: 32 } },
      ];
      this.refReprojectBindGroups[p] = d.createBindGroup({
        label: `reproject-ref-${p}`,
        layout: this.reprojectLayout,
        entries: reprojectEntries(
          t.illumRaw, t.illumHist[prev], t.momentsHist[prev],
          t.illumHist[cur], t.momentsHist[cur],
        ),
      });
      this.refIndReprojectBindGroups[p] = d.createBindGroup({
        label: `reproject-ref-ind-${p}`,
        layout: this.reprojectLayout,
        entries: reprojectEntries(
          t.indRaw, t.indHist[prev], t.indMoments[prev],
          t.indHist[cur], t.indMoments[cur],
        ),
      });
      this.refVolReprojectBindGroups[p] = d.createBindGroup({
        label: `reproject-ref-vol-${p}`,
        layout: this.reprojectLayout,
        entries: reprojectEntries(
          t.volRaw, t.volHist[prev], t.volMoments[prev],
          t.volHist[cur], t.volMoments[cur],
        ),
      });

      const compositeEntries = (
        direct: GPUTexture, indirect: GPUTexture, volume: GPUTexture,
      ) => [
        { binding: 0, resource: { buffer: this.compositeBuffer } },
        { binding: 1, resource: v(direct) },
        { binding: 2, resource: v(t.albedo) },
        { binding: 3, resource: v(t.normalDepth[cur]) },
        { binding: 4, resource: v(t.momentsHist[cur]) },
        { binding: 5, resource: t.illumRaw },
        { binding: 6, resource: v(t.hdr) },
        { binding: 7, resource: v(indirect) },
        { binding: 8, resource: v(t.transScratch[1]) },
        { binding: 9, resource: v(volume) },
      ];
      // Reference mode skips a-trous entirely, so it composites straight from
      // the accumulators.
      this.refCompositeBindGroups[p] = d.createBindGroup({
        label: `composite-ref-${p}`,
        layout: this.compositeLayout,
        entries: compositeEntries(t.illumHist[cur], t.indHist[cur], t.volHist[cur]),
      });

      this.compositeBindGroups[p] = d.createBindGroup({
        label: `composite-${p}`,
        layout: this.compositeLayout,
        // The final a-trous iteration of each chain lands in its scratch[0].
        entries: compositeEntries(t.scratch[0], t.indScratch[0], t.volScratch[0]),
      });
    }

    // ---- bloom ------------------------------------------------------------
    if (this.bloomBuffers.length === 0) {
      for (let i = 0; i < BLOOM_MIPS * 2; i++) {
        this.bloomBuffers.push(
          this.device.createBuffer({
            label: `bloom-params-${i}`,
            size: 32,
            usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
          }),
        );
      }
    }

    this.bloomDownBindGroups = [];
    for (let i = 0; i < BLOOM_MIPS; i++) {
      const src = i === 0 ? t.hdr : t.bloomDown[i - 1];
      this.bloomDownBindGroups.push(
        d.createBindGroup({
          label: `bloom-down-${i}`,
          layout: this.bloomLayout,
          entries: [
            { binding: 0, resource: this.sampler },
            { binding: 1, resource: v(src) },
            { binding: 2, resource: v(src) },
            { binding: 3, resource: v(t.bloomDown[i]) },
            { binding: 4, resource: { buffer: this.bloomBuffers[i] } },
          ],
        }),
      );
    }

    // Upsample walks back down the chain: up[i] = tent(smaller) + down[i].
    this.bloomUpBindGroups = [];
    for (let i = BLOOM_MIPS - 2; i >= 0; i--) {
      const lower = i === BLOOM_MIPS - 2 ? t.bloomDown[BLOOM_MIPS - 1] : t.bloomUp[i + 1];
      this.bloomUpBindGroups.push(
        d.createBindGroup({
          label: `bloom-up-${i}`,
          layout: this.bloomLayout,
          entries: [
            { binding: 0, resource: this.sampler },
            { binding: 1, resource: v(t.bloomDown[i]) },
            { binding: 2, resource: v(lower) },
            { binding: 3, resource: v(t.bloomUp[i]) },
            { binding: 4, resource: { buffer: this.bloomBuffers[BLOOM_MIPS + i] } },
          ],
        }),
      );
    }

    this.postBindGroup = d.createBindGroup({
      label: "post",
      layout: this.postLayout,
      entries: [
        { binding: 0, resource: this.sampler },
        { binding: 1, resource: v(t.hdr) },
        { binding: 2, resource: v(t.bloomUp[0]) },
        { binding: 3, resource: { buffer: this.postBuffer } },
      ],
    });
  }

  // -------------------------------------------------------------------------

  /** The indirect mode actually rendered: reference and no-patch both force traced. */
  private effectiveIndirectMode(settings: RenderSettings): IndirectMode {
    if (settings.reference) return "traced";
    // The patch-count guard is about the patch solve having nothing to read,
    // so it must not reach cascades — those probes exist whatever the geometry
    // diced into, and a scene with no patches is exactly where they should win.
    if (this.radPatchCount === 0 && settings.indirectMode !== "cascades") return "traced";
    return settings.indirectMode;
  }

  private writeUniforms(s: FrameState, settings: RenderSettings, t: Targets): void {
    const f = this.uniformF32;
    const u = this.uniformU32;
    f.set(s.invViewProj, 0);
    f.set(s.prevViewProj, 16);
    f[32] = s.camPos.x; f[33] = s.camPos.y; f[34] = s.camPos.z;
    u[35] = this.frameIndex;
    f[36] = t.width; f[37] = t.height;
    f[38] = 1 / t.width; f[39] = 1 / t.height;
    f[40] = s.flashPos.x; f[41] = s.flashPos.y; f[42] = s.flashPos.z;
    f[43] = s.flashRadius;
    f[44] = s.flashDir.x; f[45] = s.flashDir.y; f[46] = s.flashDir.z;
    f[47] = s.flashIntensity;
    f[48] = s.flashColor.x; f[49] = s.flashColor.y; f[50] = s.flashColor.z;
    f[51] = s.flashCosOuter;
    f[52] = s.flashCosInner;
    u[53] = this.lightCount;
    u[65] = this.transientStart;
    f[66] = settings.transientSamples;
    u[54] = settings.bounces;
    u[55] = settings.spp;
    f[56] = settings.skyIntensity;
    u[57] = this.dynCount;
    f[58] = settings.ambient;
    f[59] = settings.volumetricSteps;
    // Fine lattice, bytes 960-979 (f32 indices 240-244). fineCell 0 disables
    // it entirely and makes the tracer bit-identical to the coarse-only path.
    f[240] = this.fineOrigin[0];
    f[241] = this.fineOrigin[1];
    f[242] = this.fineOrigin[2];
    f[243] = this.fineActive || this.fineBlend > 0 ? FINE_CELL : 0;
    f[244] = this.fineBlend;
    f[245] = settings.volPhaseG;
    f[246] = settings.smokeShadow;
    f[60] = settings.exposure;
    f[61] = s.time;
    u[62] = settings.debugView;
    f[63] = settings.volumetric;
    f[71] = settings.restirCandidates;
    // Reference mode is brute force by definition: no reservoir reuse of any
    // kind, or it would be validating an estimator against itself.
    f[72] = settings.restirTemporal && !settings.reference ? 1 : 0;
    f[73] = settings.restirMCap;
    f[74] = settings.restirGI && !settings.reference ? 1 : 0;
    u[64] = settings.debugTapMode;
    f[236] = s.prevCamPos.x; f[237] = s.prevCamPos.y; f[238] = s.prevCamPos.z;  // bytes 944-955
    // Below two bounces there is almost nothing behind the first bounce to
    // skip, so checkerboarding is pure noise for no gain. Make it inert rather
    // than letting the slider do harm.
    f[67] = settings.bounces <= 1 ? 1.0 : settings.indirectRate;
    u[70] = this.staticLightCount;
    f[68] = settings.transientBlurDist;
    f[69] = settings.transientBounceWeight;
    u[75] = this.dynGroupCount;
    // vec4f arrays are 16-byte aligned, so these land at byte 304 = f32 76.
    f.set(this.dynGroupMin, 76);
    f.set(this.dynGroupMax, 76 + DYN_GROUPS * 4);
    // After the arrays: byte 560 = f32 140. Zeroed in reference mode along
    // with every other reuse path.
    f[140] = settings.reference ? 0 : settings.restirSpatialTaps;
    f[141] = settings.restirSpatialRadius;
    // flashVisTarget is unbiased but is still an estimator trick, and the
    // volumetric map is a genuine approximation: reference runs both off.
    f[142] = settings.flashVisTarget && !settings.reference ? 1 : 0;
    f[143] = settings.flashVisVolumetric && !settings.reference ? 1 : 0;
    f[144] = settings.fogAmount;
    // Wind speed is a look constant, not a knob — one fewer slider.
    f[145] = 0.4;
    // Bytes 592-847 (f32 148-211) are the retired puff arrays: dead, zero.
    // Reference mode brute-forces bounces: reading the patch solve would be
    // validating an approximation against itself.
    const mode = this.effectiveIndirectMode(settings);
    // Legacy mirror of the mode: 1 while the solve is live this frame. The
    // shader branches on indirectMode; this stays so the field's meaning holds.
    f[212] = mode !== "traced" ? 1 : 0;
    // Selects which half of the merged reservoir buffers is written; the
    // bind group's cur/prev textures follow the same parity, so they agree.
    u[213] = this.parity;
    f[214] = settings.counters ? 1 : 0;
    // Byte 864: this track's tail. IMODE_* constants in common.wgsl.
    u[216] = INDIRECT_MODES.indexOf(mode);
    u[217] = this.radPatchCount;
    // ---- volumetric block, bytes 880-943 (f32 index 220). Bytes 864-879 are
    // the radiosity track's and stay untouched here.
    const vf = UNIFORM_VOL_OFFSET / 4;
    f[vf] = this.smokeOrigin[0]; f[vf + 1] = this.smokeOrigin[1]; f[vf + 2] = this.smokeOrigin[2];
    f[vf + 3] = this.smokeCellSize;
    f[vf + 4] = this.lightVolOrigin[0];
    f[vf + 5] = this.lightVolOrigin[1];
    f[vf + 6] = this.lightVolOrigin[2];
    // The baked light volume is the estimator's approximation of the static
    // in-scatter; reference mode Monte Carlos the real thing instead.
    f[vf + 7] = settings.reference ? 1 : 0;
    f[vf + 8] = this.lightVolCell[0];
    f[vf + 9] = this.lightVolCell[1];
    f[vf + 10] = this.lightVolCell[2];
    f[vf + 11] = settings.volExtinction ? 1 : 0;
    // Bytes 928-935, the first two of the four spare f32s that used to be
    // `_volPad` inside the volumetric block — no uniform growth, no offset move.
    f[vf + 12] = settings.smokeDetail;
    f[vf + 13] = settings.smokeDetailFreq;
    this.device.queue.writeBuffer(this.uniformBuffer, 0, this.uniformData);
    this.writeTransientParams(settings);

    // composite params
    this.compositeScratchU32[0] = settings.debugView;
    // Reference mode never sees transients: they are a lighting event, not part
    // of the steady scene the accumulator is converging to.
    this.compositeScratch[1] =
      this.transientStart < this.lightCount && !settings.reference ? 1 : 0;
    this.device.queue.writeBuffer(this.compositeBuffer, 0, this.compositeScratch);

    // post params
    const pp = this.postScratch;
    pp[0] = this.ctx.canvas.width;
    pp[1] = this.ctx.canvas.height;
    pp[2] = s.mouseX;
    pp[3] = s.mouseY;
    pp[4] = settings.exposure;
    pp[5] = settings.bloomIntensity;
    pp[6] = settings.vignette;
    pp[7] = settings.grain;
    pp[8] = s.time;
    this.postScratchU32[9] = settings.debugView;
    pp[10] = settings.chromatic;
    pp[11] = settings.saturation;
    pp[12] = settings.nightVision ? 1 : 0;
    pp[13] = settings.nvGain;
    pp[14] = settings.nvPhosphor;
    pp[15] = s.seenPulse;
    this.device.queue.writeBuffer(this.postBuffer, 0, pp);

    // bloom params — texel sizes and the threshold only, so this is resize-and-
    // settings-driven rather than per frame.
    if (this.bloomParamsDirty || this.bloomThresholdWritten !== settings.bloomThreshold) {
      const b = this.bloomScratch;
      for (let i = 0; i < BLOOM_MIPS; i++) {
        const srcW = i === 0 ? t.width : t.bloomDown[i - 1].width;
        const srcH = i === 0 ? t.height : t.bloomDown[i - 1].height;
        b[0] = 1 / srcW; b[1] = 1 / srcH;
        b[2] = settings.bloomThreshold;
        b[3] = 1;
        this.bloomScratchU32[4] = i === 0 ? 1 : 0;
        this.device.queue.writeBuffer(this.bloomBuffers[i], 0, b);
      }
      this.bloomScratchU32[4] = 0;
      for (let i = 0; i < BLOOM_MIPS - 1; i++) {
        b[0] = 1 / t.bloomUp[i].width;
        b[1] = 1 / t.bloomUp[i].height;
        b[2] = 0;
        b[3] = 1;
        this.device.queue.writeBuffer(this.bloomBuffers[BLOOM_MIPS + i], 0, b);
      }
      this.bloomParamsDirty = false;
      this.bloomThresholdWritten = settings.bloomThreshold;
    }
  }

  render(s: FrameState, settings: RenderSettings): void {
    const t = this.targets;
    if (!t) return;

    // Latched, not thrown: the frame it would fire on still renders, just with
    // the wrong albedo somewhere, and a hard failure here would take out a
    // running session over a startup ordering mistake.
    if (!this.materialGrowthWarned
      && this.packedMaterials.length !== this.packedMaterialCount) {
      this.materialGrowthWarned = true;
      console.error(
        `[renderer] ${this.packedMaterials.length - this.packedMaterialCount} `
        + `material(s) were registered after Renderer.create; the material `
        + `buffer holds ${this.packedMaterialCount} and is never grown. `
        + `Indices ${this.packedMaterialCount}+ clamp to `
        + `${this.packedMaterialCount - 1} and will shade wrong. `
        + `Register every material before create.`,
      );
    }

    this.writeUniforms(s, settings, t);
    const p = this.parity;
    const enc = this.device.createCommandEncoder({ label: "frame" });
    this.profiler.begin();
    // Counters accumulate across the flashmap and trace passes; zero first.
    this.workCounters.begin(enc, settings.counters);

    const gx = Math.ceil(t.width / WG);
    const gy = Math.ceil(t.height / WG);

    const compute = (
      label: string,
      pipeline: GPUComputePipeline,
      bg1: GPUBindGroup | null,
      off: number[] | null,
      x: number,
      y: number,
      bg0: GPUBindGroup = this.sceneBindGroup,
      z = 1,
    ) => {
      const pass = enc.beginComputePass({ label, timestampWrites: this.profiler.pass(label) });
      pass.setPipeline(pipeline);
      pass.setBindGroup(0, bg0);
      if (bg1) {
        // Never pass an explicit undefined as the third argument. WebIDL
        // overload resolution for setBindGroup differs between Chrome builds,
        // and some try to convert undefined to a sequence and throw "The
        // provided value cannot be converted to a sequence."
        if (off) pass.setBindGroup(1, bg1, off);
        else pass.setBindGroup(1, bg1);
      }
      pass.dispatchWorkgroups(x, y, z);
      pass.end();
    };

    // A static light changed intensity since the last frame: the baked
    // light volume is stale, so re-bake it before the trace samples it.
    if (this.lightVolDirty) {
      this.lightVolDirty = false;
      this.encodeLightVolBake(enc);
    }

    // The depth maps must be current before the trace consumes them. One z
    // slice per torch layer; dead layers cost one branch and a store.
    const fmGroups = Math.ceil(FLASHMAP_RES / WG);
    compute(
      "flashmap", this.flashmapPipeline, this.flashmapBindGroup, null,
      fmGroups, fmGroups, this.sceneBindGroup, TORCH_LAYERS,
    );
    // Radiosity: inject from the fresh torch maps, then two warm-started
    // Jacobi steps (ping-pong via the two bind groups). The trace pass reads
    // the gather buffer the second step wrote. Runs for every non-traced
    // indirect mode.
    // Only the modes that actually READ the patch solve pay for it. Testing
    // against "traced" alone was right while every other mode was patch-based;
    // cascades is not, and left as it was it ran the whole solve every frame
    // and threw the result away.
    const imode = this.effectiveIndirectMode(settings);
    if (imode === "radiosityRead" || imode === "gather" || imode === "patchRIS") {
      const ng = Math.ceil(this.radPatchCount / 64);
      compute("radInject", this.radInjectPipeline, this.radBindGroups[0], null, ng, 1);
      compute("radSolveA", this.radSolvePipeline, this.radBindGroups[0], null, ng, 1);
      compute("radSolveB", this.radSolvePipeline, this.radBindGroups[1], null, ng, 1);
      // The emitter CDF is only sampled by patchRIS; one workgroup scans the
      // B half solveB just wrote (group 1's bOut).
      if (imode === "patchRIS") {
        compute("radCdf", this.radCdfPipeline, this.radBindGroups[1], null, 1, 1);
      }
    }
    // Radiance cascades. After flashmap (the probe hits shade with the fresh
    // torch maps) and before pathtrace (which reads the result this frame).
    if (imode === "cascades") {
      this.encodeCascades(enc, compute);
    }
    if (this.targets && this.fluidFine && this.fineGeneration !== this.fluidFine.generation) {
      this.buildBindGroups(this.targets);
    }

    // Smoke fluid step: injects the frame's sources, advances the sim by
    // the game dt, and writes the density interface texture the trace pass
    // samples — so it must land before the trace. A zero-dt frame (frozen
    // clock) leaves the field exactly as it was, which is what a still-image
    // A/B and the reference accumulator need.
    if (settings.fluidSim && s.dt > 0) {
      const { nf, nc } = this.routeSources(s.smokeSources, s.smokeSourceCount);
      this.fluid.step(
        enc, s.dt, this.srcCoarse, nc, (l) => this.profiler.pass(l),
      );
      // The fine lattice steps only while anchored to a live event, so the
      // resting cost of the whole mechanism is zero dispatches — it is an
      // event tax, not a standing one.
      if (this.fineActive || this.fineBlend > 0) {
        this.fluidFine.step(
          enc, s.dt, this.srcFine, nf, (l) => this.profiler.pass(l),
        );
      }
      // Ramp in over ~0.3 s and out over ~0.5 s.
      const target = this.fineActive ? 1 : 0;
      const rate = this.fineActive ? s.dt / 0.3 : s.dt / 0.5;
      this.fineBlend = target > this.fineBlend
        ? Math.min(target, this.fineBlend + rate)
        : Math.max(target, this.fineBlend - rate);
    }
    compute("pathtrace", this.ptPipeline, this.ptBindGroups[p], null, gx, gy);

    // Direct and indirect run the same two pipelines over separate textures.
    // The only difference is the a-trous parameter block: indirect filters with
    // wider strides and no luminance edge-stopping, which is the entire reason
    // the signals are kept apart.
    const ref = settings.reference;
    const refSlot = ref ? 2 : 0;
    compute(
      "reproject", this.reprojectPipeline,
      ref ? this.refReprojectBindGroups[p] : this.reprojectBindGroups[p],
      this.reprojectOffsets[refSlot], gx, gy,
    );
    compute(
      "reprojectInd", this.reprojectPipeline,
      ref ? this.refIndReprojectBindGroups[p] : this.indReprojectBindGroups[p],
      this.reprojectOffsets[refSlot + 1], gx, gy,
    );
    compute(
      "reprojectVol", this.reprojectPipeline,
      ref ? this.refVolReprojectBindGroups[p] : this.volReprojectBindGroups[p],
      this.reprojectOffsets[ref ? 5 : 4], gx, gy,
    );

    if (!ref) {
      for (let i = 0; i < ATROUS_ITERS; i++) {
        compute(
          this.atrousLabels[i], this.atrousPipeline,
          this.atrousBindGroups[p][i], this.atrousOffsets[i], gx, gy,
        );
        compute(
          this.atrousIndLabels[i], this.atrousPipeline,
          this.indAtrousBindGroups[p][i], this.atrousOffsets[ATROUS_ITERS + i], gx, gy,
        );
      }
      const volBase = ATROUS_ITERS * 2 + TRANS_ATROUS_ITERS;
      for (let i = 0; i < VOL_ATROUS_ITERS; i++) {
        compute(
          this.atrousVolLabels[i], this.atrousPipeline,
          this.volAtrousBindGroups[p][i], this.atrousOffsets[volBase + i], gx, gy,
        );
      }
    }

    // Skipped outright when no transient light exists, which is almost always.
    // The composite is told separately, so it ignores the stale texture rather
    // than us paying to clear it.
    if (this.transientStart < this.lightCount && !ref) {
      for (let i = 0; i < TRANS_ATROUS_ITERS; i++) {
        compute(
          this.atrousTransLabels[i], this.atrousPipeline,
          this.transAtrousBindGroups[p][i],
          this.atrousOffsets[ATROUS_ITERS * 2 + i], gx, gy,
        );
      }
    }

    const cbg = ref ? this.refCompositeBindGroups[p] : this.compositeBindGroups[p];
    compute("composite", this.compositePipeline, null, null, gx, gy, cbg);

    // Gameplay light probes. One thread each, so this is free next to the
    // trace, and it runs against exactly the lights and occluders the image was
    // rendered from.
    if (this.probeCount > 0) {
      const pass = enc.beginComputePass({ label: "probe" });
      pass.setPipeline(this.probePipeline);
      pass.setBindGroup(0, this.sceneBindGroup);
      pass.setBindGroup(1, this.probeBindGroup);
      pass.dispatchWorkgroups(1, 1, 1);
      pass.end();
      // Only stage a copy when the previous readback has landed; mapping a
      // buffer that is already mapped is an error, and dropping a frame of
      // probe data is harmless for what this feeds.
      if (!this.probeStagingBusy) {
        enc.copyBufferToBuffer(
          this.probeOutBuffer, 0, this.probeStaging, 0, MAX_PROBES * 16,
        );
      }
    }

    // Coarse smoke density for gameplay LOS, a few frames behind by design.
    // Every SMOKE_READ_EVERY frames, and only while the last readback landed:
    // dropping a frame of smoke data is harmless for a suspicion integral.
    if (this.frameIndex % SMOKE_READ_EVERY === 0 && !this.smokeCoarseBusy) {
      const pass = enc.beginComputePass({ label: "smokeprobe" });
      pass.setPipeline(this.smokeProbePipeline);
      pass.setBindGroup(0, this.sceneBindGroup);
      pass.setBindGroup(1, this.smokeProbeBindGroup);
      pass.dispatchWorkgroups(
        Math.ceil(this.smokeCoarseDims[0] / 4),
        Math.ceil(this.smokeCoarseDims[1] / 4),
        Math.ceil(this.smokeCoarseDims[2] / 4),
      );
      pass.end();
      enc.copyBufferToBuffer(
        this.smokeCoarseBuffer, 0, this.smokeCoarseStaging, 0, this.smokeCoarse.byteLength,
      );
      this.smokeCoarseArmed = true;
    }

    // Every ray-tracing pass — flashmap, trace, probe, and the light-volume
    // rebake on a frame that runs one — has flushed by here (the per-frame
    // radiosity solve traces none; the denoise, composite and smoke-readback
    // passes trace none), so the totals are complete. Per-pixel normalisation
    // uses this internal size.
    this.workCounters.resolve(
      enc, settings.counters, this.frameIndex, t.width * t.height,
    );

    // Bloom: one dispatch per mip in each direction. Each is tiny.
    {
      const pass = enc.beginComputePass({ label: "bloom", timestampWrites: this.profiler.pass("bloom") });
      pass.setPipeline(this.bloomDownPipeline);
      for (let i = 0; i < BLOOM_MIPS; i++) {
        pass.setBindGroup(0, this.bloomDownBindGroups[i]);
        pass.dispatchWorkgroups(
          Math.ceil(t.bloomDown[i].width / WG),
          Math.ceil(t.bloomDown[i].height / WG),
          1,
        );
      }
      pass.setPipeline(this.bloomUpPipeline);
      // bloomUpBindGroups was built from the smallest mip upward.
      for (let k = 0; k < this.bloomUpBindGroups.length; k++) {
        const i = BLOOM_MIPS - 2 - k;
        pass.setBindGroup(0, this.bloomUpBindGroups[k]);
        pass.dispatchWorkgroups(
          Math.ceil(t.bloomUp[i].width / WG),
          Math.ceil(t.bloomUp[i].height / WG),
          1,
        );
      }
      pass.end();
    }

    {
      const view = this.ctx.context.getCurrentTexture().createView();
      const pass = enc.beginRenderPass({
        label: "post",
        colorAttachments: [
          { view, loadOp: "clear", storeOp: "store", clearValue: { r: 0, g: 0, b: 0, a: 1 } },
        ],
        timestampWrites: this.profiler.pass("post"),
      });
      pass.setPipeline(this.postPipeline);
      pass.setBindGroup(0, this.postBindGroup);
      pass.draw(3);
      pass.end();
    }

    this.profiler.resolve(enc);
    this.device.queue.submit([enc.finish()]);
    this.profiler.afterSubmit();
    this.workCounters.afterSubmit();

    // Same rule for the coarse smoke readback: map only after the submit.
    if (this.smokeCoarseArmed) {
      this.smokeCoarseArmed = false;
      this.smokeCoarseBusy = true;
      this.smokeCoarseStaging.mapAsync(GPUMapMode.READ).then(
        () => {
          this.smokeCoarse.set(new Float32Array(this.smokeCoarseStaging.getMappedRange()));
          this.smokeCoarseStaging.unmap();
          this.smokeCoarseBusy = false;
        },
        // A rejected map must not freeze the gameplay view of the smoke at
        // its last snapshot: release the slot so the next cycle retries. On
        // a dead device the retry rejects immediately — one map per
        // SMOKE_READ_EVERY frames, no spin.
        () => { this.smokeCoarseBusy = false; },
      );
    }

    // Kick the probe readback after submit, never before — mapping a buffer
    // that is still referenced by a pending command buffer is exactly the
    // "used in submit while mapped" error the profiler was once bitten by.
    if (this.probeCount > 0 && !this.probeStagingBusy) {
      this.probeStagingBusy = true;
      this.probeStaging.mapAsync(GPUMapMode.READ).then(
        () => {
          const src = new Float32Array(this.probeStaging.getMappedRange());
          for (let i = 0; i < MAX_PROBES; i++) this.probeLuma[i] = src[i * 4 + 3];
          this.probeDebug.set(src.subarray(0, 4));
          this.probeStaging.unmap();
          this.probeStagingBusy = false;
        },
        // A device loss or a destroyed buffer rejects here. Leave the flag set
        // so we stop trying rather than spinning on a dead buffer.
        () => { /* readback abandoned */ },
      );
    }

    this.parity = 1 - this.parity;
    this.frameIndex++;
  }

  get renderWidth(): number { return this.targets?.width ?? 0; }
  get renderHeight(): number { return this.targets?.height ?? 0; }
}
