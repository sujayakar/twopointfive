// Sized well above what the renderer issues (14 passes: pathtrace, reproject
// x2, atrous x8, composite, bloom, post) so adding a direct/indirect pair does
// not silently start dropping timings. Each slot costs 16 bytes of readback.
const MAX_PASSES = 32;

interface Readback {
  buffer: GPUBuffer;
  inFlight: boolean;
  /**
   * Labels for the frame this slot carries. Filled in place rather than copied
   * from `pending`, which would allocate an array every frame. Safe because a
   * slot is only ever re-armed after its map callback has consumed them.
   */
  labels: string[];
}

/**
 * Per-pass GPU timing via timestamp queries.
 *
 * Results lag by a frame or two because readback is async — that is fine for a
 * HUD, and it means profiling never stalls the submit. Falls back to a no-op
 * when the adapter lacks `timestamp-query`.
 */
export class GpuProfiler {
  readonly enabled: boolean;
  private querySet: GPUQuerySet | null = null;
  private resolveBuffer: GPUBuffer | null = null;
  private readbacks: Readback[] = [];
  /** Labels for the frame being encoded; reused, `pendingCount` is the length. */
  private pending: string[] = [];
  private pendingCount = 0;
  /** One descriptor per query pair, built once — see `pass()`. */
  private writes: GPUComputePassTimestampWrites[] = [];
  private armed: Readback | null = null;
  private overflowWarned = false;
  /** Smoothed per-pass milliseconds. */
  readonly timings = new Map<string, number>();

  constructor(device: GPUDevice, enabled: boolean) {
    this.enabled = enabled;
    if (!enabled) return;

    this.querySet = device.createQuerySet({
      label: "pass-timers",
      type: "timestamp",
      count: MAX_PASSES * 2,
    });
    this.resolveBuffer = device.createBuffer({
      label: "timestamp-resolve",
      size: MAX_PASSES * 2 * 8,
      usage: GPUBufferUsage.QUERY_RESOLVE | GPUBufferUsage.COPY_SRC,
    });
    for (let i = 0; i < 3; i++) {
      this.readbacks.push({
        buffer: device.createBuffer({
          label: `timestamp-readback-${i}`,
          size: MAX_PASSES * 2 * 8,
          usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
        }),
        inFlight: false,
        labels: [],
      });
    }
    // `pass()` runs once per pass per frame and the descriptor it returns is
    // read synchronously by beginComputePass, so one immutable object per query
    // pair serves every frame. Building them here keeps the render loop free of
    // per-pass garbage.
    for (let i = 0; i < MAX_PASSES; i++) {
      this.writes.push({
        querySet: this.querySet,
        beginningOfPassWriteIndex: i * 2,
        endOfPassWriteIndex: i * 2 + 1,
      });
    }
  }

  /** Call at the start of each frame. */
  begin(): void {
    this.pendingCount = 0;
    // A slot armed by the previous frame but never handed to afterSubmit() —
    // render() threw between the two — is stale: its copy belongs to a command
    // buffer from another frame. Drop it rather than map it late.
    this.armed = null;
  }

  /**
   * Returns the `timestampWrites` descriptor for a pass, or undefined when
   * profiling is off or the frame is already full.
   */
  pass(label: string): GPUComputePassTimestampWrites | undefined {
    if (!this.enabled || !this.querySet) return undefined;
    if (this.pendingCount >= MAX_PASSES) {
      // Once, not per frame: at 60fps a per-frame warning is 60 lines a second
      // and buries whatever else the console is saying.
      if (!this.overflowWarned) {
        this.overflowWarned = true;
        console.warn(
          `[profiler] more than ${MAX_PASSES} timestamped passes in one frame; ` +
            `"${label}" and any pass after it goes untimed, so reported GPU ` +
            `totals under-report. Raise MAX_PASSES in profiler.ts.`,
        );
      }
      return undefined;
    }
    const i = this.pendingCount++;
    this.pending[i] = label;
    return this.writes[i];
  }

  /** Records the query resolve. Call after all passes, before submitting. */
  resolve(encoder: GPUCommandEncoder): void {
    if (!this.enabled || !this.querySet || !this.resolveBuffer) return;
    const n = this.pendingCount;
    if (n === 0) return;

    encoder.resolveQuerySet(this.querySet, 0, n * 2, this.resolveBuffer, 0);

    const slot = this.readbacks.find((r) => !r.inFlight);
    if (!slot) return; // all readbacks busy — skip this frame's numbers

    encoder.copyBufferToBuffer(this.resolveBuffer, 0, slot.buffer, 0, n * 2 * 8);
    slot.labels.length = n;
    for (let i = 0; i < n; i++) slot.labels[i] = this.pending[i];
    // Deliberately *not* marked inFlight yet — see afterSubmit().
    this.armed = slot;
  }

  /**
   * Kicks off the readback. Must run *after* queue.submit(): a buffer may not be
   * mapped while a submitted command buffer still references it.
   */
  afterSubmit(): void {
    const slot = this.armed;
    if (!slot) return;
    this.armed = null;

    // inFlight is raised here rather than in resolve() so that a throw between
    // the two cannot strand the slot: nothing is marked busy until the map is
    // genuinely outstanding. The flag still spans the whole async window, which
    // is the part that matters — it is what stops a later submit writing into a
    // buffer while it is mapped. Three stranded slots used to stop profiling
    // permanently and silently, which reads as a perf change, not a bug.
    slot.inFlight = true;
    let mapped: Promise<undefined>;
    try {
      mapped = slot.buffer.mapAsync(GPUMapMode.READ);
    } catch {
      slot.inFlight = false;
      return;
    }

    void mapped.then(
      () => {
        try {
          const data = new BigUint64Array(slot.buffer.getMappedRange().slice(0));
          slot.buffer.unmap();
          const labels = slot.labels;
          for (let i = 0; i < labels.length; i++) {
            const t0 = data[i * 2];
            const t1 = data[i * 2 + 1];
            if (t1 <= t0) continue;
            const ms = Number(t1 - t0) / 1e6;
            const prev = this.timings.get(labels[i]);
            // Exponential smoothing — raw per-frame numbers are too jittery to read.
            this.timings.set(labels[i], prev === undefined ? ms : prev * 0.9 + ms * 0.1);
          }
        } finally {
          // Last, and unconditionally: the slot's label array is fair game for
          // the next frame the moment this clears.
          slot.inFlight = false;
        }
      },
      () => {
        slot.inFlight = false;
      },
    );
  }

  total(): number {
    let t = 0;
    for (const v of this.timings.values()) t += v;
    return t;
  }

  destroy(): void {
    this.querySet?.destroy();
    this.resolveBuffer?.destroy();
    for (const r of this.readbacks) r.buffer.destroy();
  }
}
