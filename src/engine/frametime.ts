// ---------------------------------------------------------------------------
// Frame pacing instrumentation.
//
// Average FPS is close to useless for judging whether a game feels smooth. A
// build alternating 16ms / 25ms and one holding a flat 20ms both report "about
// 50 fps", and the first is visibly juddery while the second is fine.
//
// What actually matters on a fixed-refresh display is how many vblank intervals
// each frame occupies, and whether that count is *stable*. A frame time sitting
// right on a vblank boundary is the worst place to be: tiny variance flips it
// between 2 and 3 intervals every frame. So this tracks the distribution and
// the interval count, not the mean.
// ---------------------------------------------------------------------------

const CAPACITY = 240;
const SPARK = "▁▂▃▄▅▆▇█";
/** Deltas per refresh estimate — ~2s of frames. */
const REFRESH_WINDOW = 120;

export class FrameTimer {
  private times = new Float32Array(CAPACITY);
  private count = 0;
  private head = 0;
  private sorted = new Float32Array(CAPACITY);
  /**
   * percentiles() costs a 240-element sort, and report() plus the p95 getter
   * both want it. Cache it and invalidate on push, so it runs at most once per
   * frame instead of once per caller.
   */
  private stats = { p50: 0, p95: 0, p99: 0, max: 0 };
  private statsDirty = true;

  /** Estimated display refresh interval in ms. */
  refreshMs = 1000 / 60;
  private refreshSamples: number[] = [];
  private refreshEstimated = false;
  private presentCap = 1;

  push(ms: number): void {
    this.times[this.head] = ms;
    this.head = (this.head + 1) % CAPACITY;
    if (this.count < CAPACITY) this.count++;
    this.statsDirty = true;
    this.estimateRefresh(ms);
  }

  /**
   * Estimate the refresh interval from the shortest frames we ever see: when
   * the GPU finishes early, rAF is pinned exactly to vblank, so the minimum is
   * a much better estimator than the mean.
   *
   * The estimate is only ever revised *downward*. Every way this can go wrong —
   * a present cap, a slow start-up, a hitch — makes deltas longer; nothing
   * makes a delta shorter than a real vblank. So a shorter observation is
   * always the better evidence and a longer one is never a reason to move.
   * That is what makes it safe to keep estimating forever instead of locking:
   * the old code locked permanently after the first 120 samples, so a present
   * cap engaged during those two seconds pinned a 120Hz panel to "60Hz" for the
   * rest of the session — after which the HUD and the adaptive-resolution
   * budget were both working from twice the real vblank.
   */
  private estimateRefresh(ms: number): void {
    // Under a cap each rendered frame spans `presentCap` vblanks, so normalise
    // before judging plausibility as well as before estimating.
    const v = ms / this.presentCap;
    if (v <= 1 || v >= 40) return;

    this.refreshSamples.push(v);
    if (this.refreshSamples.length < REFRESH_WINDOW) return;
    const s = this.refreshSamples;
    this.refreshSamples = [];

    s.sort((a, b) => a - b);
    // 5th percentile of observed deltas ~ one vblank.
    const est = s[Math.floor(s.length * 0.05)];
    // Snap to a plausible refresh rate.
    const candidates = [1000 / 240, 1000 / 165, 1000 / 144, 1000 / 120, 1000 / 90, 1000 / 60];
    let best = candidates[0];
    for (const c of candidates) {
      if (Math.abs(c - est) < Math.abs(best - est)) best = c;
    }
    if (!this.refreshEstimated || best < this.refreshMs) {
      this.refreshMs = best;
      this.refreshEstimated = true;
    }
  }

  /**
   * Vblank intervals per rendered frame under the present-rate cap; 1 (the
   * default) means uncapped.
   *
   * Optional. The downward-only rule above already makes a cap engaged after a
   * good estimate harmless. This closes the remaining hole: a cap active before
   * the first estimate is ever made, where capped deltas are an exact multiple
   * of the vblank and 60Hz is indistinguishable from 120Hz-every-other.
   */
  setPresentCap(vblanksPerFrame: number): void {
    const cap = Math.max(1, Math.round(vblanksPerFrame) || 1);
    if (cap === this.presentCap) return;
    this.presentCap = cap;
    // Samples in flight were normalised against the old cap; pooling the two
    // scales would bias the percentile. Re-arm instead — and allow the next
    // estimate to move in either direction, since the current one may have been
    // derived under a cap we now know about.
    this.refreshSamples = [];
    this.refreshEstimated = false;
  }

  private percentiles(): { p50: number; p95: number; p99: number; max: number } {
    if (!this.statsDirty) return this.stats;
    this.statsDirty = false;
    const st = this.stats;
    if (this.count === 0) {
      st.p50 = st.p95 = st.p99 = st.max = 0;
      return st;
    }
    const s = this.sorted.subarray(0, this.count);
    s.set(this.times.subarray(0, this.count));
    s.sort();
    const at = (q: number) => s[Math.min(this.count - 1, Math.floor(this.count * q))];
    st.p50 = at(0.5);
    st.p95 = at(0.95);
    st.p99 = at(0.99);
    st.max = s[this.count - 1];
    return st;
  }

  /**
   * Fraction of recent frames that did not land on the same number of vblank
   * intervals as the median. This is the number that correlates with "feels
   * janky" — it is high exactly when the frame time straddles a boundary.
   */
  private judderRatio(medianIntervals: number): number {
    if (this.count === 0) return 0;
    let bad = 0;
    for (let i = 0; i < this.count; i++) {
      const iv = Math.max(1, Math.round(this.times[i] / this.refreshMs));
      if (iv !== medianIntervals) bad++;
    }
    return bad / this.count;
  }

  private sparkline(width = 48): string {
    if (this.count === 0) return "";
    const n = Math.min(width, this.count);
    // Scale against three vblanks so the graph's meaning is fixed, not relative.
    const scale = this.refreshMs * 3;
    let out = "";
    for (let i = 0; i < n; i++) {
      const idx = (this.head - n + i + CAPACITY * 2) % CAPACITY;
      const f = Math.min(1, this.times[idx] / scale);
      out += SPARK[Math.min(SPARK.length - 1, Math.floor(f * SPARK.length))];
    }
    return out;
  }

  /**
   * @param gpuMs actual GPU work per frame, excluding present wait.
   *
   * Needed because a present-rate cap pins the measured frame time to the cap
   * itself. Comparing that against the vblank budget reports a deficit that is
   * not real — the question under a cap is whether the *work* fits, not whether
   * the capped interval does.
   */
  report(gpuMs?: number): string {
    const { p50, p95, p99, max } = this.percentiles();
    const hz = Math.round(1000 / this.refreshMs);
    const intervals = Math.max(1, Math.round(p50 / this.refreshMs));
    const judder = this.judderRatio(intervals);
    const paced = 1000 / (intervals * this.refreshMs);
    const budget = intervals * this.refreshMs;
    const against = gpuMs !== undefined && gpuMs > 0 ? gpuMs : p95;
    const headroom = budget - against;
    const basis = gpuMs !== undefined && gpuMs > 0 ? "gpu" : "p95";

    return [
      `frame  p50 ${p50.toFixed(1)}  p95 ${p95.toFixed(1)}  p99 ${p99.toFixed(1)}  max ${max.toFixed(1)} ms`,
      `display ${hz}Hz  vblank ${this.refreshMs.toFixed(1)}ms  -> ${intervals}x = ${paced.toFixed(0)}fps locked`,
      `headroom ${headroom >= 0 ? "+" : ""}${headroom.toFixed(1)}ms vs ${basis}   judder ${(judder * 100).toFixed(0)}%`,
      this.sparkline(),
    ].join("\n");
  }

  /** p95, the number adaptive resolution should regulate against. */
  get p95(): number {
    return this.percentiles().p95;
  }

  reset(): void {
    this.count = 0;
    this.head = 0;
    this.statsDirty = true;
    // Drop the partial refresh window too: reset() is called when something
    // about pacing or resolution just changed, and samples from either side of
    // that change should not be pooled into one estimate. The estimate itself
    // survives — it only ever moves downward, so a stale one is never worse
    // than falling back to the 60Hz default would be.
    this.refreshSamples = [];
  }
}
