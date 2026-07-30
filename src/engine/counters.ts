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
 * raysDepth0 is the primary ray; raysDepthN is a trace at bounce depth N (the
 * bounces slider tops out at 6). Shadow rays are split by what they serve:
 * the direct-light reservoir survivor, the indirect-bounce RIS winner, a
 * reused GI sample's revisibility test, transient (muzzle flash) NEE, and the
 * volumetric march's fallback rays. flashmapRays is the depth-map pass's own
 * ray count; its traversal is not folded into bvhNodeVisits/boxTests so those
 * stay per-image-pixel quantities.
 */
export const COUNTER_SLOTS = [
  "raysDepth0", "raysDepth1", "raysDepth2", "raysDepth3",
  "raysDepth4", "raysDepth5", "raysDepth6",
  "shadowDirect", "shadowIndirect", "shadowGI",
  "shadowTransient", "shadowVolumetric",
  "bvhNodeVisits", "boxTests", "risCandidates",
  "volumeSteps", "radiosityGathers", "flashmapRays",
] as const;

export type CounterName = (typeof COUNTER_SLOTS)[number];
const BUFFER_BYTES = Math.max(COUNTER_SLOTS.length * 4, 16);

export interface CounterFrame {
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
  /** Pixel count of the frame this slot carries. */
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
        pixels: 0,
      });
    }
  }

  /** Frame start. Off drops the stale totals rather than leaving them to read as current. */
  begin(enc: GPUCommandEncoder, on: boolean): void {
    this.armed = null;
    if (!on) {
      this.latest = null;
      return;
    }
    enc.clearBuffer(this.buffer);
  }

  /** After the trace pass: stage this frame's totals for readback. */
  resolve(enc: GPUCommandEncoder, on: boolean, pixels: number): void {
    if (!on) return;
    const slot = this.readbacks.find((r) => !r.inFlight);
    if (!slot) return;
    enc.copyBufferToBuffer(this.buffer, 0, slot.buffer, 0, BUFFER_BYTES);
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
          this.latest = decode(raw, slot.pixels);
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

function decode(raw: Uint32Array, pixels: number): CounterFrame {
  const totals = {} as Record<CounterName, number>;
  const perPixel = {} as Record<CounterName, number>;
  const px = Math.max(pixels, 1);
  COUNTER_SLOTS.forEach((name, i) => {
    totals[name] = raw[i];
    perPixel[name] = +(raw[i] / px).toFixed(4);
  });
  return { totals, pixels, perPixel };
}
