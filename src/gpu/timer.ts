// GPU pass timing via timestamp queries (falls back to no-op when unsupported).
/** THE RENDER PASSES, by the names the page stamps them under. The GPU's busy span (GpuTimer.unionMs) is taken over the OTHERS wherever the page states a
 *  GPU time, because a render pass that draws to the display has stamps that span the wait for it. The ladder's and the backend's GPU ms (host/drawpage.ts,
 *  host/drawcloud.ts), and the pricing run (ui/pricerun.ts) read this list. The read-out's own union (host/drawpage.ts) and a recording's (ui/perfrec.ts)
 *  still spell theirs, without `present`: not changed here */
export const RENDER_PASSES: readonly string[] = ['composite', 'post', 'fxaa', 'gbuffer', 'pdraw', 'present'];
/** … the one of them that draws into the page's own textures, so its stamp follows its own work (read on an M4 Max under Metal against 50,000 recorded frames,
 *  2026-11-04) · and the rest, which draw to the display or after it: THEIR stamps are the ones that hold the wait */
export const SURFACE_PASS = 'gbuffer', PRESENT_PASSES: readonly string[] = RENDER_PASSES.filter(p => p !== SURFACE_PASS);
/** A FRAME NOBODY ASKS FOR IS ONE IN `EVERY` READ BACK: mapping a frame's stamps and adding them up costs the page as much as a pass of its own does, and the ladder
 *  judges a second at a time. Five, for a pass's own rhythm is a half, a quarter or an eighth of the frames: each of its frames comes round in turn. A slow page is
 *  read sooner, after `SLOW` ms and an odd number of frames, so that a second of it always holds a few. Whoever reads frame by frame asks (`ask`), and every frame
 *  is read for `KEEPS` ms after the last asking. */
const EVERY = 5, SLOW = 100, KEEPS = 500;
/** frames a GPU may have in hand: two that are further apart than this do not overlap */
const IN_HAND = 3;
/** One pass's raw interval in the last resolved frame: ms from that frame's earliest timestamp (see GpuTimer.lastRaw / unionMs). */
export interface PassInterval { name: string; begin: number; end: number }
export class GpuTimer {
  private querySet: GPUQuerySet | null = null;
  private resolveBuf: GPUBuffer | null = null;
  private readBufs: GPUBuffer[] = [];
  private capacity = 64; // pairs
  private names: string[] = [];
  /** passes declared idle THIS frame (idle()): reported as an explicit 0 without a query slot */
  private idles: string[] = [];
  private pending: { buf: GPUBuffer; names: string[]; idles: string[]; frame: number } [] = [];
  /** Smoothed ms per pass name. */
  results = new Map<string, number>();
  /** Unsmoothed ms per pass name of the last readback that resolved, and how many readbacks have resolved so far — for scripted measurement. */
  lastFrame = new Map<string, number>(); resolved = 0;
  /** The raw begin/end of every stamped pass of that same readback, in ms from the frame's earliest stamp, in encode order. Passes are NOT serialized on
   *  the GPU: Metal runs independent compute passes concurrently when the queue is saturated (the `direct` pass beside the whole cascade chain is the
   *  normal state at play cadence), and a pass's end − begin then spans time the GPU spent mostly on another pass. So a
   *  SUM of per-pass durations (lastFrame summed) double-counts overlapped spans — by 35–55 % of the frame at high / medium in the field —
   *  while unionMs() is the wall span the GPU was actually busy with the listed passes. Per-pass numbers stay what they are (fine for a pass measured
   *  with frames serialized; misleading for a small pass that straddled a big one). */
  lastRaw: PassInterval[] = [];
  /** the earliest and the latest tick (ns, device timeline) of the last resolved readback, and its frame: a pair older than the one (the other, of frames too far apart to
   *  overlap) in the NEXT readback was not written that frame */
  private lastBeginTick: bigint = 0n; private lastEndTick: bigint = 0n; private lastFrameNo = 0;
  private lastSeen = new Map<string, number>(); private frameNo = 0;
  /** this frame's stamps are written and read back */
  private reads = false; private readAt = 0; private readMs = -1e9; private askedMs = -1e9;
  enabled: boolean;

  constructor(private device: GPUDevice, enabled: boolean, private now: () => number = () => performance.now()) {
    this.enabled = enabled;
    if (!enabled) return;
    this.querySet = device.createQuerySet({ type: 'timestamp', count: this.capacity * 2 });
    this.resolveBuf = device.createBuffer({ size: this.capacity * 16, usage: GPUBufferUsage.QUERY_RESOLVE | GPUBufferUsage.COPY_SRC });
  }

  /** every frame is wanted, from the next on */
  ask() { this.askedMs = this.now(); }
  beginFrame() {
    this.names.length = 0; this.idles.length = 0; this.frameNo++;
    const now = this.now(), since = this.frameNo - this.readAt; this.reads = now - this.askedMs <= KEEPS || since >= EVERY || since % 2 === 1 && now - this.readMs >= SLOW;
    if (this.reads) { this.readAt = this.frameNo; this.readMs = now; }
    // forget passes that stopped running (a smoke solver gone idle, removed cascades, idle probes)
    for (const [k, f] of this.lastSeen) if (this.frameNo - f > 30) { this.lastSeen.delete(k); this.results.delete(k); }
  }

  /** Returns timestampWrites for a pass descriptor (or undefined when disabled/full, and on a frame that is not read back). */
  pass(name: string): GPUComputePassTimestampWrites | undefined {
    if (!this.enabled || this.names.length >= this.capacity) return undefined;
    this.lastSeen.set(name, this.frameNo); if (!this.reads) return undefined;
    const i = this.names.length; this.names.push(name);
    return { querySet: this.querySet!, beginningOfPassWriteIndex: i * 2, endOfPassWriteIndex: i * 2 + 1 };
  }
  /** Declare that `name` deliberately did no GPU work this frame (a cache resting on its off frame): it counts as having run, at an explicit 0 ms, so the
   *  smoothed `results` average the pass at the rate it really runs at. Do NOT open an empty timed pass for this instead: an empty compute pass never
   *  writes its timestamp pair, the slot keeps whatever ticks an earlier frame left there, and their difference reads as a plausible phantom cost (the
   *  half-rate cascades read at ~93 % of full rate that way until 08-10). */
  idle(name: string) { if (!this.enabled) return; if (this.reads) this.idles.push(name); this.lastSeen.set(name, this.frameNo); }

  /** Call after encoding all passes, before submit. */
  resolve(enc: GPUCommandEncoder) {
    if (!this.enabled || (this.names.length === 0 && this.idles.length === 0)) return;   // (a frame that is not read back has neither)
    const n = this.names.length;
    let rb = this.readBufs.pop();
    if (!rb) rb = this.device.createBuffer({ size: this.capacity * 16, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
    if (n > 0) { enc.resolveQuerySet(this.querySet!, 0, n * 2, this.resolveBuf!, 0); enc.copyBufferToBuffer(this.resolveBuf!, 0, rb, 0, n * 16); }
    this.pending.push({ buf: rb, names: this.names.slice(), idles: this.idles.slice(), frame: this.frameNo });
  }

  /** Call after submit. Maps the oldest pending readback asynchronously. */
  afterSubmit() {
    if (!this.enabled) return;
    while (this.pending.length > 2) { const p = this.pending.shift()!; this.readBufs.push(p.buf); }
    const p = this.pending.shift(); if (!p) return;
    p.buf.mapAsync(GPUMapMode.READ).then(() => {
      const t = new BigInt64Array(p.buf.getMappedRange().slice(0));
      p.buf.unmap(); this.readBufs.push(p.buf);
      const totals = new Map<string, number>(); const raw: PassInterval[] = [];
      // A pass that took a slot and then encoded nothing (or an empty pass) leaves the ticks of whatever earlier frame last used that slot. Command buffers
      // START in order (though frames may overlap on the GPU), so a pair that begins before the first stamp of the frame READ BEFORE THIS ONE is at least two reads stale:
      // it cost nothing this frame and must not enter the timeline or set its origin. (Where the two are next to each other a one-read-stale pair cannot be told apart
      // from a frame that overlapped its predecessor — which is why resting passes declare idle() rather than open empty passes. Further apart, the last stamp is the line.)
      // A never-written slot resolves to zeros (the query set's initial state): the same nothing.
      const n = Math.max(1, p.frame - this.lastFrameNo), fence = n > IN_HAND ? this.lastEndTick : this.lastBeginTick; let t0: bigint | null = null; this.lastFrameNo = p.frame;
      const stale = (b: bigint, e: bigint) => b < fence || (b === 0n && e === 0n);
      for (let i = 0; i < p.names.length; i++) { const b = t[i * 2], e = t[i * 2 + 1]; if (e < b || stale(b, e)) continue; if (t0 === null || b < t0) t0 = b; if (e > this.lastEndTick) this.lastEndTick = e; }
      if (t0 !== null) this.lastBeginTick = t0;
      for (let i = 0; i < p.names.length; i++) {
        const b = t[i * 2], e = t[i * 2 + 1];
        if (stale(b, e)) { totals.set(p.names[i], totals.get(p.names[i]) ?? 0); continue; }   // stale / never-written pair (see above): an explicit nothing this frame
        const ms = Number(e - b) / 1e6;
        if (ms < 0 || ms > 1000) continue;
        totals.set(p.names[i], (totals.get(p.names[i]) ?? 0) + ms);
        const begin = Number(b - t0!) / 1e6; raw.push({ name: p.names[i], begin, end: begin + ms });
      }
      for (const k of p.idles) if (!totals.has(k)) totals.set(k, 0);   // resting passes: an honest zero this frame
      const keep = 0.9 ** n;   // (eased by the frames gone by, however few of them were read)
      for (const [k, v] of totals) this.results.set(k, (this.results.get(k) ?? v) * keep + v * (1 - keep));
      for (const k of this.results.keys()) if (!totals.has(k) && this.frameNo - (this.lastSeen.get(k) ?? -99) > 1) this.results.set(k, this.results.get(k)! * 0.5 ** n);   // not in this readback and not stamped lately: fade out instead of freezing at the last busy value
      this.lastFrame = totals; this.lastRaw = raw; this.resolved++;
    }).catch(() => { /* device lost or unmapped */ });
  }

  /** was `name` stamped THIS frame? (a pass that keeps its slot's old timestamps — one skipped for having no work, e.g. particles with nothing alive after a
   *  settings toggle — must not report the last busy frame's cost until the 30-frame expiry: the HUD read a phantom 6.9 ms 'particles' that way, 08-10) */
  ranThisFrame(name: string): boolean { return this.lastSeen.get(name) === this.frameNo; }
  /** GPU-busy wall span of the last resolved frame over the passes not in `exclude` (ms): the UNION of their raw intervals — what the frame really cost on
   *  the GPU, however the driver overlapped the passes (see lastRaw). Pass the render passes here as everywhere (their stamps span the drawable wait). */
  unionMs(exclude: readonly string[] = []): number {
    const iv = this.lastRaw.filter(r => !exclude.includes(r.name)).map(r => [r.begin, r.end]).sort((a, b) => a[0] - b[0]);
    let total = 0, cs = 0, ce = -1;
    for (const [b, e] of iv) { if (ce < 0) { cs = b; ce = e; } else if (b <= ce) ce = Math.max(ce, e); else { total += ce - cs; cs = b; ce = e; } }
    return ce < 0 ? 0 : total + (ce - cs);
  }
}
