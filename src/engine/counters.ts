// ---------------------------------------------------------------------------
// Work counters: rays, shadow rays, BVH node visits, box tests, RIS
// candidates and the like, per frame. Machine-independent — the same scene at
// the same resolution produces the same counts on SwiftShader and on real
// silicon — which is what lets algorithmic changes be judged on a box with no
// GPU. Counting only, never timing: with counters on, atomic contention makes
// the frame's milliseconds meaningless.
// ---------------------------------------------------------------------------

/**
 * Counter slots, in buffer order. shaders.ts generates the WGSL constants
 * (CT_<name>) from this list, so the two cannot drift out of step.
 *
 * raysDepth0 is the primary ray; raysDepthN is a trace at bounce depth N and
 * raysDepth6plus takes N >= 6 (the panel slider stops at 6; the raw setting
 * does not). primaryHits is the primary rays with a valid hit — the base the
 * per-hit ratios reconcile against. Shadow rays are split by what they serve:
 * the direct-light reservoir survivor, the indirect-bounce RIS winner, a
 * reused GI sample's revisibility test, transient (muzzle flash) NEE, the
 * volumetric march's fallback rays, and the gameplay light-probe pass.
 * obbTests are leaf ray-vs-oriented-box tests; slabTests are the BVH-child
 * and dynamic-group AABB slab tests. RIS candidates are split direct
 * (restirDirect) from indirect (sampleIndirectRIS). radiosityGathers is the
 * trace pass's per-pixel reads of the solved patch texture — the per-frame
 * solve itself is patchCount^2 MADs and traces no rays, so it needs no
 * counter. flashmapRays and shadowProbe come from passes that report only
 * their own ray counts; their traversal is not folded into
 * bvhNodeVisits/obbTests/slabTests so those stay per-image-pixel quantities.
 * risCandidatesPatch / shadowPatch / patchCdfTaps belong to the patchRIS
 * indirect mode: patch-as-emitter proposals per pixel, the one shadow ray
 * to the survivor, and the CDF/data texture reads that carry its non-ray
 * cost.
 */
export const COUNTER_SLOTS = [
  "raysDepth0", "raysDepth1", "raysDepth2", "raysDepth3",
  "raysDepth4", "raysDepth5", "raysDepth6plus",
  "primaryHits",
  "shadowDirect", "shadowIndirect", "shadowGI",
  "shadowTransient", "shadowVolumetric", "shadowProbe",
  "bvhNodeVisits", "obbTests", "slabTests",
  "risCandidatesDirect", "risCandidatesIndirect",
  "volumeSteps", "radiosityGathers", "flashmapRays",
  "risCandidatesPatch", "shadowPatch", "patchCdfTaps",
] as const;

export type CounterName = (typeof COUNTER_SLOTS)[number];
const BUFFER_BYTES = Math.max(COUNTER_SLOTS.length * 4, 16);

export interface CounterFrame {
  /** Renderer frame index these totals belong to. Slots recycle only across
   *  event-loop turns, so a synchronous frame loop can strand this on an
   *  early frame — the index says which one it is. */
  frame: number;
  /** Whole-frame totals, one u32 per slot. */
  totals: Record<CounterName, number>;
  /** Internal pixel count of the frame these totals belong to. */
  pixels: number;
  /** totals / pixels — the unit for comparing algorithmic changes. */
  perPixel: Record<CounterName, number>;
}

interface Readback {
  buffer: GPUBuffer;
  inFlight: boolean;
  /** Frame index and pixel count of the frame this slot carries. */
  frame: number;
  pixels: number;
}

/**
 * Owns the counters storage buffer and its readback, mirroring GpuProfiler:
 * clear at frame start, copy out after the trace pass, mapAsync after submit.
 * Totals land in `latest` a frame or two late.
 */
export class WorkCounters {
  /** Bound as array<atomic<u32>> by every pass that flushes counts. */
  readonly buffer: GPUBuffer;
  private readbacks: Readback[] = [];
  private armed: Readback | null = null;
  private pending: Promise<void> | null = null;
  /** Off means an in-flight readback must not resurrect `latest`. */
  private on = false;
  /** Most recent completed frame, or null while counters are off. */
  latest: CounterFrame | null = null;

  constructor(device: GPUDevice) {
    this.buffer = device.createBuffer({
      label: "work-counters",
      size: BUFFER_BYTES,
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST,
    });
    for (let i = 0; i < 3; i++) {
      this.readbacks.push({
        buffer: device.createBuffer({
          label: `work-counters-readback-${i}`,
          size: BUFFER_BYTES,
          usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
        }),
        inFlight: false,
        frame: 0,
        pixels: 0,
      });
    }
  }

  /** Frame start. Off drops the stale totals rather than leaving them to read as current. */
  begin(enc: GPUCommandEncoder, on: boolean): void {
    this.armed = null;
    this.on = on;
    if (!on) {
      this.latest = null;
      return;
    }
    enc.clearBuffer(this.buffer);
  }

  /**
   * After the last counting pass: stage this frame's totals for readback.
   * With all three slots in flight the frame is skipped — slots free only on
   * an event-loop turn, hence CounterFrame.frame.
   */
  resolve(enc: GPUCommandEncoder, on: boolean, frame: number, pixels: number): void {
    if (!on) return;
    const slot = this.readbacks.find((r) => !r.inFlight);
    if (!slot) return;
    enc.copyBufferToBuffer(this.buffer, 0, slot.buffer, 0, BUFFER_BYTES);
    slot.frame = frame;
    slot.pixels = pixels;
    // Not marked inFlight until afterSubmit(), for the same reason as the
    // profiler: a throw between here and submit must not strand the slot.
    this.armed = slot;
  }

  afterSubmit(): void {
    const slot = this.armed;
    if (!slot) return;
    this.armed = null;
    slot.inFlight = true;
    let mapped: Promise<undefined>;
    try {
      mapped = slot.buffer.mapAsync(GPUMapMode.READ);
    } catch {
      slot.inFlight = false;
      return;
    }
    this.pending = mapped.then(
      () => {
        try {
          const raw = new Uint32Array(slot.buffer.getMappedRange().slice(0));
          slot.buffer.unmap();
          if (this.on) this.latest = decode(raw, slot.frame, slot.pixels);
        } finally {
          slot.inFlight = false;
        }
      },
      () => { slot.inFlight = false; },
    );
  }

  /** Resolves once the most recently staged readback has landed. */
  async flush(): Promise<CounterFrame | null> {
    await this.pending;
    return this.latest;
  }
}

function decode(raw: Uint32Array, frame: number, pixels: number): CounterFrame {
  const totals = {} as Record<CounterName, number>;
  const perPixel = {} as Record<CounterName, number>;
  const px = Math.max(pixels, 1);
  COUNTER_SLOTS.forEach((name, i) => {
    totals[name] = raw[i];
    perPixel[name] = +(raw[i] / px).toFixed(4);
  });
  return { frame, totals, pixels, perPixel };
}
