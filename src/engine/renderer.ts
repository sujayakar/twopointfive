import { Mat4, Vec3 } from "../core/math";
import { BVH } from "../scene/bvh";
import {
  BOX_STRIDE_F32, LIGHT_STRIDE_F32, Light, SceneBuilder,
  packBoxes, packLights, packMaterials,
} from "../scene/scene";
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
// 304 bytes of scalars plus two arrays of DYN_GROUPS vec4f for the group bounds.
const UNIFORM_SIZE = 304 + DYN_GROUPS * 16 * 2;
/** Bytes per ReSTIR reservoir; must match the WGSL struct. */
const RESERVOIR_BYTES = 32;
/** Bytes per ReSTIR GI reservoir; four vec3f/f32 pairs. */
const GI_RESERVOIR_BYTES = 64;
/**
 * Upper bound on animated boxes. Traced linearly, so it is a real per-ray cost.
 * A Character is 24 boxes, so the player plus four patrolling guards needs 120.
 */
const MAX_DYN_BOXES = 160;
/**
 * Headroom in the light buffer for lights that move (guard torches). Static
 * lights occupy the head of the buffer and are uploaded once; these follow.
 */
const MAX_DYN_LIGHTS = 16;
const ATROUS_ITERS = 4;
const ATROUS_STRIDE = 256; // dynamic uniform offset alignment
/**
 * Reproject parameter slots: [0] direct, [1] indirect, [2] direct reference,
 * [3] indirect reference. The reference pair disables every heuristic — firefly
 * ceiling, history clamp, history cap, alpha floor — so the pass degenerates to
 * an honest 1/n running average.
 */
const REPROJECT_STRIDE = 256;
const REPROJECT_SLOTS = 4;
/** A-trous parameter slots: 4 direct, 4 indirect, then 2 transient. */
const TRANS_ATROUS_ITERS = 2;
const BLOOM_MIPS = 5;
/** Gameplay light probes; must match MAX_PROBES in probe.wgsl. */
const MAX_PROBES = 4;
const WG = 8;
// Workgroup size was tuned by measurement at 1152x720 / 2 bounces / leaf<=1:
//   8x4  (32 threads)  16.30 ms
//   8x8  (64 threads)  13.12 ms  <- best
//   16x8 (128 threads) 13.73 ms
// 64 threads wins on Apple silicon here; do not "optimise" without re-measuring.

export interface RenderSettings {
  /** Internal render resolution as a fraction of the canvas backing size. */
  resolutionScale: number;
  spp: number;
  bounces: number;
  volumetric: number;
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
  /** Fresh ReSTIR candidates per pixel before reuse. */
  restirCandidates: number;
  /** Temporal reservoir reuse across frames. */
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
   * Shadow rays per muzzle flash on the primary hit.
   *
   * The transient signal is never temporally accumulated, so this is its only
   * real variance control. Paid for a few frames per shot.
   */
  transientSamples: number;
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
}

// Defaults match quality preset 1 ("performance"): it is the setting that fits
// inside a single 120Hz vblank, so it is the one that feels right out of the box.
export const DEFAULT_SETTINGS: RenderSettings = {
  resolutionScale: 0.5,
  // 1 is enough now that indirect bounces resample across all lights; before
  // that fix the indirect noise made 2 look necessary, at double the cost.
  spp: 1,
  bounces: 1,
  volumetric: 0.4,
  // Deliberately low. At 1.9 the unlit areas sat at a readable grey, which
  // undercuts the whole premise: dark has to actually be dark for the beam to
  // carry the scene. Bright regions still resolve because AgX handles the
  // highlight rolloff.
  exposure: 0.1,
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
  volumetricSteps: 8,
  restirCandidates: 8,
  restirTemporal: true,
  restirGI: true,
  restirMCap: 20,
  transientSamples: 8,
  indirectRate: 1.0,
  reference: false,
  nightVision: false,
  nvGain: 475,
  nvPhosphor: 0.09,
  debugView: 0,
};

export interface FrameState {
  invViewProj: Mat4;
  prevViewProj: Mat4;
  camPos: Vec3;
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
}

interface Targets {
  width: number;
  height: number;
  albedo: GPUTexture;
  normalDepth: [GPUTexture, GPUTexture];
  pos: GPUTexture;
  illumRaw: GPUTexture;
  accumIllum: GPUTexture;
  illumHist: [GPUTexture, GPUTexture];
  momentsHist: [GPUTexture, GPUTexture];
  scratch: [GPUTexture, GPUTexture];
  momentsScratch: [GPUTexture, GPUTexture];
  /** Parallel chain for the indirect signal, filtered on its own terms. */
  indRaw: GPUTexture;
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
  transRaw: GPUTexture;
  transScratch: [GPUTexture, GPUTexture];
  /**
   * Written and ignored. The a-trous pass always emits moments, and binding one
   * texture as both its input and its output in a single pass is a validation
   * error that invalidates the entire command buffer — so the transient chain
   * ping-pongs a throwaway pair rather than aliasing one.
   */
  transMoments: [GPUTexture, GPUTexture];
  hdr: GPUTexture;
  bloomDown: GPUTexture[];
  bloomUp: GPUTexture[];
  /** Per-pixel ReSTIR reservoirs, ping-ponged across frames. */
  reservoir: [GPUBuffer, GPUBuffer];
  /** Per-pixel ReSTIR GI reservoirs, likewise. */
  giReservoir: [GPUBuffer, GPUBuffer];
}

/** IEEE 754 half -> float. JS has no native f16, and rgba16float is what we store. */
function halfToFloat(h: number): number {
  const sign = (h & 0x8000) ? -1 : 1;
  const exp = (h >> 10) & 0x1f;
  const frac = h & 0x3ff;
  if (exp === 0) return sign * frac * 2 ** -24;
  if (exp === 31) return frac ? NaN : sign * Infinity;
  return sign * (1 + frac / 1024) * 2 ** (exp - 15);
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
    Array.from(
      { length: ATROUS_ITERS * 2 + TRANS_ATROUS_ITERS },
      (_, i) => [i * ATROUS_STRIDE],
    );
  /** Pass labels, so the per-frame loop is not building template strings. */
  private readonly atrousLabels: string[] =
    Array.from({ length: ATROUS_ITERS }, (_, i) => `atrous${i}`);
  private readonly atrousIndLabels: string[] =
    Array.from({ length: ATROUS_ITERS }, (_, i) => `atrousInd${i}`);
  private readonly atrousTransLabels: string[] =
    Array.from({ length: TRANS_ATROUS_ITERS }, (_, i) => `atrousTrans${i}`);
  private reprojectBuffer!: GPUBuffer;
  private compositeBuffer!: GPUBuffer;
  private bloomBuffers: GPUBuffer[] = [];
  private postBuffer!: GPUBuffer;

  // Layouts
  private sceneLayout!: GPUBindGroupLayout;
  private ptLayout!: GPUBindGroupLayout;
  private reprojectLayout!: GPUBindGroupLayout;
  private atrousLayout!: GPUBindGroupLayout;
  private compositeLayout!: GPUBindGroupLayout;
  private bloomLayout!: GPUBindGroupLayout;
  private postLayout!: GPUBindGroupLayout;
  private probeLayout!: GPUBindGroupLayout;

  // Pipelines
  private ptPipeline!: GPUComputePipeline;
  private reprojectPipeline!: GPUComputePipeline;
  private atrousPipeline!: GPUComputePipeline;
  private compositePipeline!: GPUComputePipeline;
  private bloomDownPipeline!: GPUComputePipeline;
  private bloomUpPipeline!: GPUComputePipeline;
  private postPipeline!: GPURenderPipeline;
  private probePipeline!: GPUComputePipeline;

  // Bind groups
  private sceneBindGroup!: GPUBindGroup;
  private ptBindGroups: GPUBindGroup[] = [];
  private reprojectBindGroups: GPUBindGroup[] = [];
  private atrousBindGroups: GPUBindGroup[][] = [];
  private indReprojectBindGroups: GPUBindGroup[] = [];
  private indAtrousBindGroups: GPUBindGroup[][] = [];
  private transAtrousBindGroups: GPUBindGroup[][] = [];
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
  readonly profiler: GpuProfiler;
  private lightCount = 0;
  /** Static lights, i.e. the offset at which the dynamic tail begins. */
  private staticLightCount = 0;
  private lightBuffer!: GPUBuffer;
  /** Scratch for the dynamic tail, allocated once. */
  private dynLightData = new Float32Array(MAX_DYN_LIGHTS * LIGHT_STRIDE_F32);
  private lightScratch: Light[] = [];
  /**
   * First transient light index. Everything from here on is a muzzle flash or a
   * detonation: sampled by plain NEE into its own un-accumulated signal, and
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
  }

  static async create(ctx: GPUContext, scene: SceneBuilder, bvh: BVH): Promise<Renderer> {
    const r = new Renderer(ctx);
    await r.init(scene, bvh);
    return r;
  }

  private async init(scene: SceneBuilder, bvh: BVH): Promise<void> {
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
      size: ATROUS_STRIDE * (ATROUS_ITERS * 2 + TRANS_ATROUS_ITERS),
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
      // Indirect fireflies are already clamped at the source (luminance 3), so
      // this pass only has to catch the extremes, and its band must not collapse
      // when the local neighbourhood happens to be empty.
      const indirect = new Float32Array([6.0, 2.0, 6.0, 0.5, 1.0, 48.0, 0.02, 0]);
      // Reference: no rejection of any kind, unbounded history, no alpha floor.
      // Every heuristic here exists to trade bias for responsiveness, which is
      // exactly what a ground-truth accumulator must not do.
      const BIG = 1e9;
      const refDirect = new Float32Array([BIG, BIG, BIG, BIG, 0.0, BIG, 0.0, 0]);
      const refIndirect = new Float32Array([BIG, BIG, BIG, BIG, 1.0, BIG, 0.0, 0]);
      d.queue.writeBuffer(this.reprojectBuffer, 0, direct);
      d.queue.writeBuffer(this.reprojectBuffer, REPROJECT_STRIDE, indirect);
      d.queue.writeBuffer(this.reprojectBuffer, REPROJECT_STRIDE * 2, refDirect);
      d.queue.writeBuffer(this.reprojectBuffer, REPROJECT_STRIDE * 3, refIndirect);
    }

    // Two parameter sets: the first ATROUS_ITERS are the direct chain, the next
    // ATROUS_ITERS the indirect chain. Indirect drops luminance edge-stopping
    // entirely and relaxes the geometric terms, which is the whole point of
    // separating the signals.
    for (let i = 0; i < ATROUS_ITERS; i++) {
      const direct = new ArrayBuffer(16);
      new Int32Array(direct, 0, 1)[0] = 1 << i;
      new Float32Array(direct, 4, 2).set([1.0, 1.0]);
      d.queue.writeBuffer(this.atrousBuffer, i * ATROUS_STRIDE, direct);

      const indirect = new ArrayBuffer(16);
      // Wider strides: bounce light is low frequency, so reach further.
      new Int32Array(indirect, 0, 1)[0] = 2 << i;
      new Float32Array(indirect, 4, 2).set([0.0, 3.0]);
      d.queue.writeBuffer(
        this.atrousBuffer, (ATROUS_ITERS + i) * ATROUS_STRIDE, indirect,
      );
    }
    // Transient: spatial only, so it leans harder on the filter than either
    // accumulated signal does — there is no history to fall back on.
    for (let i = 0; i < TRANS_ATROUS_ITERS; i++) {
      const trans = new ArrayBuffer(16);
      new Int32Array(trans, 0, 1)[0] = 1 << i;
      new Float32Array(trans, 4, 2).set([0.0, 4.0]);
      d.queue.writeBuffer(
        this.atrousBuffer, (ATROUS_ITERS * 2 + i) * ATROUS_STRIDE, trans,
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
        { binding: 3, visibility: C, storageTexture: stTex("rgba16float") },
        { binding: 4, visibility: C, texture: tex() },
        { binding: 5, visibility: C, buffer: { type: "read-only-storage" } },
        { binding: 6, visibility: C, buffer: { type: "storage" } },
        { binding: 7, visibility: C, storageTexture: stTex("rgba16float") },
        { binding: 8, visibility: C, buffer: { type: "read-only-storage" } },
        { binding: 9, visibility: C, buffer: { type: "storage" } },
        { binding: 10, visibility: C, storageTexture: stTex("rgba16float") },
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
        { binding: 5, visibility: C, buffer: { type: "uniform", hasDynamicOffset: true, minBindingSize: 16 } },
      ],
    });

    this.probeLayout = d.createBindGroupLayout({
      label: "probe",
      entries: [
        { binding: 0, visibility: C, buffer: { type: "storage" } },
        { binding: 1, visibility: C, buffer: { type: "uniform" } },
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
    const [ptMod, rpMod, atMod, cpMod, blMod, poMod, prMod] = await Promise.all([
      createShaderModule(d, "pathtrace", SHADERS.pathtrace),
      createShaderModule(d, "reproject", SHADERS.reproject),
      createShaderModule(d, "atrous", SHADERS.atrous),
      createShaderModule(d, "composite", SHADERS.composite),
      createShaderModule(d, "bloom", SHADERS.bloom),
      createShaderModule(d, "post", SHADERS.post),
      createShaderModule(d, "probe", SHADERS.probe),
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
  }

  /** Uploads this frame's animated geometry. Packed with the same Box layout. */
  updateDynamic(data: Float32Array<ArrayBuffer>, count: number): void {
    const first = this.dynCount === 0;
    this.dynCount = Math.min(count, MAX_DYN_BOXES);
    if (this.dynCount === 0) return;
    const floats = this.dynCount * BOX_STRIDE_F32;

    // On the first frame there is no history; treat the character as having
    // been stationary rather than reprojecting against uninitialised memory.
    if (first) this.prevDynData.set(data.subarray(0, floats));

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
  async readHDR(): Promise<{ width: number; height: number; data: Float32Array }> {
    const t = this.targets;
    if (!t) throw new Error("no render targets");
    const w = t.width, h = t.height;
    // Texture-to-buffer copies need rows aligned to 256 bytes.
    const bpr = Math.ceil((w * 8) / 256) * 256;
    const staging = this.device.createBuffer({
      label: "hdr-readback",
      size: bpr * h,
      usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
    });
    const enc = this.device.createCommandEncoder({ label: "readback" });
    enc.copyTextureToBuffer(
      { texture: t.hdr }, { buffer: staging, bytesPerRow: bpr }, { width: w, height: h },
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
      // target, and only the composited HDR image is ever read back.
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
      for (const b of this.targets.reservoir) b.destroy();
      for (const b of this.targets.giReservoir) b.destroy();
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

    const t: Targets = {
      width: w,
      height: h,
      albedo: this.makeTex("g-albedo", w, h, "rgba8unorm"),
      normalDepth: [
        this.makeTex("g-nd-0", w, h, f16),
        this.makeTex("g-nd-1", w, h, f16),
      ],
      pos: this.makeTex("g-pos", w, h, "rgba32float"),
      illumRaw: this.makeTex("illum-raw", w, h, f16),
      accumIllum: this.makeTex("illum-accum", w, h, f16),
      illumHist: [this.makeTex("illum-hist-0", w, h, f16), this.makeTex("illum-hist-1", w, h, f16)],
      momentsHist: [this.makeTex("moments-hist-0", w, h, f16), this.makeTex("moments-hist-1", w, h, f16)],
      scratch: [this.makeTex("scratch-0", w, h, f16), this.makeTex("scratch-1", w, h, f16)],
      momentsScratch: [this.makeTex("m-scratch-0", w, h, f16), this.makeTex("m-scratch-1", w, h, f16)],
      transRaw: this.makeTex("trans-raw", w, h, f16),
      transScratch: [
        this.makeTex("trans-s-0", w, h, f16), this.makeTex("trans-s-1", w, h, f16),
      ],
      transMoments: [
        this.makeTex("trans-m-0", w, h, f16), this.makeTex("trans-m-1", w, h, f16),
      ],
      indRaw: this.makeTex("ind-raw", w, h, f16),
      indAccum: this.makeTex("ind-accum", w, h, f16),
      indHist: [this.makeTex("ind-hist-0", w, h, f16), this.makeTex("ind-hist-1", w, h, f16)],
      indMoments: [this.makeTex("ind-m-0", w, h, f16), this.makeTex("ind-m-1", w, h, f16)],
      indScratch: [this.makeTex("ind-s-0", w, h, f16), this.makeTex("ind-s-1", w, h, f16)],
      indMomentsScratch: [
        this.makeTex("ind-ms-0", w, h, f16), this.makeTex("ind-ms-1", w, h, f16),
      ],
      hdr: this.makeTex("hdr", w, h, f16, true),
      bloomDown,
      bloomUp,
      reservoir: [
        this.device.createBuffer({
          label: "restir-reservoir-0",
          size: w * h * RESERVOIR_BYTES,
          usage: GPUBufferUsage.STORAGE,
        }),
        this.device.createBuffer({
          label: "restir-reservoir-1",
          size: w * h * RESERVOIR_BYTES,
          usage: GPUBufferUsage.STORAGE,
        }),
      ],
      giReservoir: [
        this.device.createBuffer({
          label: "restir-gi-0",
          size: w * h * GI_RESERVOIR_BYTES,
          usage: GPUBufferUsage.STORAGE,
        }),
        this.device.createBuffer({
          label: "restir-gi-1",
          size: w * h * GI_RESERVOIR_BYTES,
          usage: GPUBufferUsage.STORAGE,
        }),
      ],
    };
    this.targets = t;
    this.frameIndex = 0;
    this.bloomParamsDirty = true;
    this.buildBindGroups(t);
  }

  private allTextures(t: Targets): GPUTexture[] {
    return [
      t.albedo, ...t.normalDepth, t.pos, t.illumRaw, t.accumIllum,
      ...t.illumHist, ...t.momentsHist, ...t.scratch, ...t.momentsScratch,
      t.indRaw, t.indAccum, ...t.indHist, ...t.indMoments,
      ...t.indScratch, ...t.indMomentsScratch,
      t.transRaw, ...t.transScratch, ...t.transMoments,
      t.hdr, ...t.bloomDown, ...t.bloomUp,
    ];
  }

  private buildBindGroups(t: Targets): void {
    const d = this.device;
    const v = (tex: GPUTexture) => tex.createView();

    this.ptBindGroups = [];
    this.reprojectBindGroups = [];
    this.atrousBindGroups = [];
    this.indReprojectBindGroups = [];
    this.indAtrousBindGroups = [];
    this.transAtrousBindGroups = [];
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
          { binding: 3, resource: v(t.illumRaw) },
          { binding: 4, resource: v(t.normalDepth[prev]) },
          { binding: 5, resource: { buffer: t.reservoir[prev] } },
          { binding: 6, resource: { buffer: t.reservoir[cur] } },
          { binding: 7, resource: v(t.indRaw) },
          { binding: 8, resource: { buffer: t.giReservoir[prev] } },
          { binding: 9, resource: { buffer: t.giReservoir[cur] } },
          { binding: 10, resource: v(t.transRaw) },
        ],
      });

      this.reprojectBindGroups[p] = d.createBindGroup({
        label: `reproject-${p}`,
        layout: this.reprojectLayout,
        entries: [
          { binding: 0, resource: v(t.illumRaw) },
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
          { binding: 0, resource: v(t.indRaw) },
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

      // a-trous chain. Iteration 0's output doubles as next frame's colour
      // history, which is what makes SVGF converge in a handful of frames
      // rather than dozens.
      const mkChain = (
        accum: GPUTexture, mCur: GPUTexture, hist: GPUTexture,
        sc: [GPUTexture, GPUTexture], ms: [GPUTexture, GPUTexture],
      ): Array<[GPUTexture, GPUTexture, GPUTexture, GPUTexture]> => [
        [accum, mCur, hist, ms[0]],
        [hist, ms[0], sc[0], ms[1]],
        [sc[0], ms[1], sc[1], ms[0]],
        [sc[1], ms[0], sc[0], ms[1]],
      ];
      const mkGroups = (
        chain: Array<[GPUTexture, GPUTexture, GPUTexture, GPUTexture]>, tag: string,
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
              { binding: 5, resource: { buffer: this.atrousBuffer, size: 16 } },
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
        ] as Array<[GPUTexture, GPUTexture, GPUTexture, GPUTexture]>,
        "trans",
      );
      this.indAtrousBindGroups[p] = mkGroups(
        mkChain(
          t.indAccum, t.indMoments[cur], t.indHist[cur],
          t.indScratch, t.indMomentsScratch,
        ),
        "ind",
      );

      const reprojectEntries = (
        raw: GPUTexture, histIn: GPUTexture, momIn: GPUTexture,
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

      const compositeEntries = (direct: GPUTexture, indirect: GPUTexture) => [
        { binding: 0, resource: { buffer: this.compositeBuffer } },
        { binding: 1, resource: v(direct) },
        { binding: 2, resource: v(t.albedo) },
        { binding: 3, resource: v(t.normalDepth[cur]) },
        { binding: 4, resource: v(t.momentsHist[cur]) },
        { binding: 5, resource: v(t.illumRaw) },
        { binding: 6, resource: v(t.hdr) },
        { binding: 7, resource: v(indirect) },
        { binding: 8, resource: v(t.transScratch[1]) },
      ];
      // Reference mode skips a-trous entirely, so it composites straight from
      // the accumulators.
      this.refCompositeBindGroups[p] = d.createBindGroup({
        label: `composite-ref-${p}`,
        layout: this.compositeLayout,
        entries: compositeEntries(t.illumHist[cur], t.indHist[cur]),
      });

      this.compositeBindGroups[p] = d.createBindGroup({
        label: `composite-${p}`,
        layout: this.compositeLayout,
        entries: [
          { binding: 0, resource: { buffer: this.compositeBuffer } },
          // The final a-trous iteration lands in scratch[0].
          { binding: 1, resource: v(t.scratch[0]) },
          { binding: 2, resource: v(t.albedo) },
          { binding: 3, resource: v(t.normalDepth[cur]) },
          { binding: 4, resource: v(t.momentsHist[cur]) },
          { binding: 5, resource: v(t.illumRaw) },
          { binding: 6, resource: v(t.hdr) },
          // The indirect chain's final a-trous iteration lands here.
          { binding: 7, resource: v(t.indScratch[0]) },
          { binding: 8, resource: v(t.transScratch[1]) },
        ],
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
    // f[64], f[66] and f[68..70] are the old whole-list dynamic AABB, dead but
    // kept so later fields keep their offsets. f[65] is transientStart.
    // Below two bounces there is almost nothing behind the first bounce to
    // skip, so checkerboarding is pure noise for no gain. Make it inert rather
    // than letting the slider do harm.
    f[67] = settings.bounces <= 1 ? 1.0 : settings.indirectRate;
    u[75] = this.dynGroupCount;
    // vec4f arrays are 16-byte aligned, so these land at byte 304 = f32 76.
    f.set(this.dynGroupMin, 76);
    f.set(this.dynGroupMax, 76 + DYN_GROUPS * 4);
    this.device.queue.writeBuffer(this.uniformBuffer, 0, this.uniformData);

    // composite params
    const cp = this.compositeScratch;
    cp[0] = s.flashColor.x; cp[1] = s.flashColor.y; cp[2] = s.flashColor.z;
    // The marched scalar already carries flashIntensity; this is only the
    // scattering coefficient, so it must NOT scale with intensity again.
    cp[3] = 0.055;
    this.compositeScratchU32[4] = settings.debugView;
    // Reference mode never sees transients: they are a lighting event, not part
    // of the steady scene the accumulator is converging to.
    // index 6, not 5: debugView is 4 and _pad0 is 5. See CompositeParams.
    cp[6] = this.transientStart < this.lightCount && !settings.reference ? 1 : 0;
    this.device.queue.writeBuffer(this.compositeBuffer, 0, cp);

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

    this.writeUniforms(s, settings, t);
    const p = this.parity;
    const enc = this.device.createCommandEncoder({ label: "frame" });
    this.profiler.begin();

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
      pass.dispatchWorkgroups(x, y, 1);
      pass.end();
    };

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
