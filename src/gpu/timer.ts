// GPU pass timing via timestamp queries (falls back to no-op when unsupported).
export class GpuTimer {
  private querySet: GPUQuerySet | null = null;
  private resolveBuf: GPUBuffer | null = null;
  private readBufs: GPUBuffer[] = [];
  private capacity = 64; // pairs
  private names: string[] = [];
  private pending: { buf: GPUBuffer; names: string[] } [] = [];
  /** Smoothed ms per pass name. */
  results = new Map<string, number>();
  private lastSeen = new Map<string, number>(); private frameNo = 0;
  enabled: boolean;

  constructor(private device: GPUDevice, enabled: boolean) {
    this.enabled = enabled;
    if (!enabled) return;
    this.querySet = device.createQuerySet({ type: 'timestamp', count: this.capacity * 2 });
    this.resolveBuf = device.createBuffer({ size: this.capacity * 16, usage: GPUBufferUsage.QUERY_RESOLVE | GPUBufferUsage.COPY_SRC });
  }

  beginFrame() {
    this.names.length = 0; this.frameNo++;
    // forget passes that stopped running (retired smoke domains, removed cascades, idle probes)
    for (const [k, f] of this.lastSeen) if (this.frameNo - f > 30) { this.lastSeen.delete(k); this.results.delete(k); }
  }

  /** Returns timestampWrites for a pass descriptor (or undefined when disabled/full). */
  pass(name: string): GPUComputePassTimestampWrites | undefined {
    if (!this.enabled || this.names.length >= this.capacity) return undefined;
    const i = this.names.length; this.names.push(name); this.lastSeen.set(name, this.frameNo);
    return { querySet: this.querySet!, beginningOfPassWriteIndex: i * 2, endOfPassWriteIndex: i * 2 + 1 };
  }

  /** Call after encoding all passes, before submit. */
  resolve(enc: GPUCommandEncoder) {
    if (!this.enabled || this.names.length === 0) return;
    const n = this.names.length;
    enc.resolveQuerySet(this.querySet!, 0, n * 2, this.resolveBuf!, 0);
    let rb = this.readBufs.pop();
    if (!rb) rb = this.device.createBuffer({ size: this.capacity * 16, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
    enc.copyBufferToBuffer(this.resolveBuf!, 0, rb, 0, n * 16);
    this.pending.push({ buf: rb, names: this.names.slice() });
  }

  /** Call after submit. Maps the oldest pending readback asynchronously. */
  afterSubmit() {
    if (!this.enabled) return;
    while (this.pending.length > 2) { const p = this.pending.shift()!; this.readBufs.push(p.buf); }
    const p = this.pending.shift(); if (!p) return;
    p.buf.mapAsync(GPUMapMode.READ).then(() => {
      const t = new BigInt64Array(p.buf.getMappedRange().slice(0));
      p.buf.unmap(); this.readBufs.push(p.buf);
      const totals = new Map<string, number>();
      for (let i = 0; i < p.names.length; i++) {
        const ms = Number(t[i * 2 + 1] - t[i * 2]) / 1e6;
        if (ms < 0 || ms > 1000) continue;
        totals.set(p.names[i], (totals.get(p.names[i]) ?? 0) + ms);
      }
      for (const [k, v] of totals) this.results.set(k, (this.results.get(k) ?? v) * 0.9 + v * 0.1);
    }).catch(() => { /* device lost or unmapped */ });
  }

  total(exclude: string[] = []): number { let s = 0; for (const [k, v] of this.results) if (!exclude.includes(k)) s += v; return s; }
}
