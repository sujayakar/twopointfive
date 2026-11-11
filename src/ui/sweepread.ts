// A perf SWEEP take (src/ui/perfsweep.ts, F8 in the game) read back: the recorder's 'sweep' events cut the frames into segments — base · flip · base · flip … — and each
// flip is priced against the base segments either side of it, round by round. What a row GIVES BACK = base − flipped (positive = the flip is cheaper), in GPU time (the
// UNION of the compute passes on the GPU's timeline — the per-pass sum double-counts passes that ran side by side), in frame time and in fps; the spread is the lowest and
// highest single round. The noise line is what a flip of nothing would read: every base segment against the mean of its two base neighbours.
// A real machine has slow stretches (another program on the GPU for a few seconds, the whole frame ~15 % slower while the game stands frozen: take 4), so beside the mean
// each row carries the MEDIAN of its rounds and the rounds themselves, a round whose two base segments disagree is marked DISTURBED (★), and where the plain rms of
// the noise is more than twice what the median difference gives, the noise is read off the median (the rms printed beside it): one slow round then neither moves a
// row's verdict (the median round against the noise) nor widens every other row's noise band. On a calm take the noise is the plain rms, as it always was.
// The look panel's Copy carries each row's price (ui/perfsend.ts); tools/qa/perf-report.ts prints the tables (tools/qa/perf-sweep.ts).
export type Frame = { t: number; dt: number; fps: number; cpu: { update: number; render: number; other: number }; gpu: Record<string, number>; gpuSum: number; gpuUnion?: number; res: [number, number]; eff: number; preset: string; adaptive: boolean; cam: number[]; n: Record<string, number | string> };
export type Ev = { t: number; frame: number; kind: string; text: string };
export type Dump = { frames: Frame[]; events: Ev[]; smokeUnit?: number; query?: string; settingsAtStart?: Record<string, unknown> };   // smokeUnit = the recorder's stamp: bricks to one smoke count (src/ui/perfrec.ts)
export type Begin = { op: 'begin'; mode: string; rounds: number; settleMs: number; measureMs: number; settleFrames: number; measureFrames: number; preset: string; res: [number, number]; scale: number; smoke: number; timers: boolean; rows: { id: string; label: string; from: Record<string, unknown>; to: Record<string, unknown> ; by?: 'frame' }[]; skipped: { id: string; why: string }[] };
type SegEv = { op: 'seg'; i: number; of: number; round: number; id: string; state: 'base' | 'flip'; settleMs?: number };   // (settleMs: only on a segment that drops more of its start than the take's own — a slow row's flip and the baseline after it)
const RENDER = /^(gbuffer|composite|post|fxaa|pdraw)$/;

interface SegStat { id: string; state: 'base' | 'flip'; round: number; n: number; dropped: number; hitches: number; union: number; dt: number; sum: number; pass: Record<string, number> }
export interface RowPrice { id: string; label: string; change: string; rounds: number; union: Spread | null; frame: Spread; fps: Spread; sum: Spread | null; movers: { pass: string; back: number }[]; inNoise: boolean;
  /** the rounds one by one (GPU union given back; frame time in a take with no GPU timers), in the order they were taken; `disturbed[i]` = that round's two base segments disagree by more than the noise allows */
  per: number[]; disturbed: boolean[]; median: number }
export interface Spread { mean: number; lo: number; hi: number }
export interface SweepResult { begin: Begin; done: boolean; why: string; segments: number; baseFrames: number; base: { union: number | null; dt: number; fps: number; sum: number | null; pass: Record<string, number> }; noise: { union: number | null; dt: number; n: number; rmsUnion: number | null; rmsDt: number; gap: number }; disturbed: number; rows: RowPrice[] }

export const mean = (a: number[]) => a.length ? a.reduce((x, y) => x + y, 0) / a.length : NaN;
const spread = (a: number[]): Spread => ({ mean: mean(a), lo: Math.min(...a), hi: Math.max(...a) });
const median = (a: number[]) => { if (!a.length) return NaN; const s = [...a].sort((x, y) => x - y), m = s.length >> 1; return s.length % 2 ? s[m] : (s[m - 1] + s[m]) / 2; };
const ROBUST_MIN = 12, SLOW_K = 2;
/** the sd of the median of n normal samples, over the sd of their mean (1 for one or two rounds, 1.16 for three, toward 1.2533 for many): what a median's wider scatter costs the noise test */
const medianK = (n: number) => [1, 1, 1, 1.160, 1.092, 1.198, 1.135, 1.214, 1.160][n] ?? 1.2533;
/** how far apart a flip's two base segments may read before the round counts as taken across a slow stretch: 3 sd of the difference of two base segments, from the
 *  robust noise (which is one segment against the mean of two: var 1.5 σ², so σ = noise ∕ √1.5 and the difference's sd = noise × √(2 ∕ 1.5)); never under 0.1 ms */
const gapOf = (noise: number) => Math.max(0.1, 3 * Math.sqrt(2 / 1.5) * noise);
const fmtV = (v: unknown) => Array.isArray(v) ? v.join(',') : typeof v === 'number' ? String(+v.toFixed(3)) : String(v);

/** every sweep in the dump (one per 'begin' event), priced */
export function readSweeps(D: Dump): SweepResult[] {
  const F = D.frames; const evs = D.events.filter(e => e.kind === 'sweep').map(e => { try { return { e, j: JSON.parse(e.text) as { op: string } }; } catch { return null; } }).filter((x): x is { e: Ev; j: { op: string } } => !!x);
  const out: SweepResult[] = [];
  for (let b = 0; b < evs.length; b++) {
    if (evs[b].j.op !== 'begin') continue;
    const begin = evs[b].j as Begin; let endAt = evs.length, done = false, why = 'the recording ended first';
    for (let k = b + 1; k < evs.length; k++) { if (evs[k].j.op === 'begin') { endAt = k; break; } if (evs[k].j.op === 'end') { endAt = k; const j = evs[k].j as unknown as { done: boolean; why: string }; done = !!j.done; why = j.why; break; } }
    const segs: SegStat[] = [];
    for (let k = b + 1; k < endAt; k++) {
      const s = evs[k].j as SegEv; if (s.op !== 'seg') continue;
      const from = evs[k].e.frame, to = k + 1 < evs.length ? evs[k + 1].e.frame : F.length;   // (an event's frame = the index of the first frame recorded after it)
      let fr: Frame[] = []; let dropped = 0;
      for (let i = from; i < Math.min(to, F.length); i++) { if (i < from + begin.settleFrames || F[i].t < evs[k].e.t + (s.settleMs ?? begin.settleMs)) dropped++; else fr.push(F[i]); }
      const med = [...fr.map(f => f.dt)].sort((x, y) => x - y)[Math.floor(fr.length / 2)] ?? 0; const all = fr.length; fr = fr.filter(f => f.dt <= Math.max(5 * med, med + 50));   // (a collector pause or a tab switch is not the feature)
      const fu = fr.filter(f => typeof f.gpuUnion === 'number'), withU = fu.length > 0 && fu.length * 2 >= fr.length;   // (a frame whose timer readback had not landed carries no union: the rest still price the segment)
      const pass: Record<string, number> = {}; const names = new Set<string>(); for (const f of fr) for (const p of Object.keys(f.gpu)) if (!RENDER.test(p)) names.add(p);
      for (const p of names) pass[p] = mean(fr.map(f => f.gpu[p] ?? 0));   // (a pass that did not run that frame cost nothing that frame)
      segs.push({ id: s.id, state: s.state, round: s.round, n: fr.length, dropped, hitches: all - fr.length, union: withU ? mean(fu.map(f => f.gpuUnion!)) : NaN, dt: mean(fr.map(f => f.dt)), sum: begin.timers ? mean(fr.map(f => f.gpuSum)) : NaN, pass });
    }
    const need = Math.max(1, Math.min(begin.measureFrames, Math.ceil(begin.measureFrames / 2)));   // (a segment cut short — F8 pressed again, the tab closed — counts only with half its frames in)
    const ok = (s: SegStat | undefined): s is SegStat => !!s && s.n >= need;
    // a segment's base = the mean of the base segments either side of it (they are two apart in the plan: base · flip · base)
    const baseOf = (k: number, step: number, f: (s: SegStat) => number): number => { const nb = [segs[k - step], segs[k + step]].filter(s => ok(s) && s.state === 'base').map(f).filter(Number.isFinite); return nb.length ? mean(nb) : NaN; };
    const per = new Map<string, { du: number[]; dd: number[]; df: number[]; ds: number[]; dp: Map<string, number[]>; gu: number[]; gd: number[] }>();   // (gu / gd: how far apart that round's two base segments read, beside du / dd)
    const gapAt = (k: number, f: (s: SegStat) => number): number => { const nb = [segs[k - 1], segs[k + 1]].filter(s => ok(s) && s.state === 'base').map(f).filter(Number.isFinite); return nb.length === 2 ? Math.abs(nb[0] - nb[1]) : 0; };
    segs.forEach((s, k) => {
      if (s.state !== 'flip' || !ok(s)) return; const bd = baseOf(k, 1, x => x.dt); if (!Number.isFinite(bd)) return;
      let r = per.get(s.id); if (!r) per.set(s.id, r = { du: [], dd: [], df: [], ds: [], dp: new Map(), gu: [], gd: [] });
      r.dd.push(bd - s.dt); r.df.push(1000 / s.dt - 1000 / bd); r.gd.push(gapAt(k, x => x.dt));
      const bu = baseOf(k, 1, x => x.union); if (Number.isFinite(bu) && Number.isFinite(s.union)) { r.du.push(bu - s.union); r.gu.push(gapAt(k, x => x.union)); }
      const bs = baseOf(k, 1, x => x.sum); if (Number.isFinite(bs) && Number.isFinite(s.sum)) r.ds.push(bs - s.sum);
      const names = new Set<string>([...Object.keys(s.pass), ...Object.keys(segs[k - 1]?.pass ?? {}), ...Object.keys(segs[k + 1]?.pass ?? {})]);
      for (const p of names) { const bp = baseOf(k, 1, x => x.pass[p] ?? 0); if (!Number.isFinite(bp)) continue; let a = r.dp.get(p); if (!a) r.dp.set(p, a = []); a.push(bp - (s.pass[p] ?? 0)); }
    });
    // the noise: a base segment against its two base neighbours (what a row would read with nothing flipped)
    const nu: number[] = [], nd: number[] = [];
    segs.forEach((s, k) => { if (s.state !== 'base' || !ok(s)) return; if (!ok(segs[k - 2]) || !ok(segs[k + 2])) return; const bd = baseOf(k, 2, x => x.dt); nd.push(bd - s.dt); const bu = baseOf(k, 2, x => x.union); if (Number.isFinite(bu) && Number.isFinite(s.union)) nu.push(bu - s.union); });
    const sd = (a: number[]) => a.length ? Math.sqrt(mean(a.map(x => x * x))) : NaN;
    // (the noise: on a calm take the plain rms, the steadiest figure there is. Where that is more than SLOW_K times a normal spread's sd read off the MEDIAN absolute
    //  difference (1.4826 × it: up to half the base segments sitting in slow stretches cannot move it), stretches of the take ran slow and the median's figure is the
    //  noise. A median of a handful is itself noise, so a take with fewer than ROBUST_MIN such differences — a few rows, or one cut short — keeps the rms whatever)
    const mad = (a: number[]) => 1.4826 * median(a.map(Math.abs));
    const robust = (a: number[]) => !a.length ? NaN : a.length >= ROBUST_MIN && sd(a) > SLOW_K * mad(a) ? mad(a) : sd(a);
    const rmsU = nu.length ? sd(nu) : null, rmsD = sd(nd), noiseU = nu.length ? robust(nu) : null, noiseD = robust(nd);
    const gap = gapOf(noiseU !== null ? noiseU : Number.isFinite(noiseD) ? noiseD : 0); let disturbedRounds = 0;
    const bases = segs.filter(s => s.state === 'base' && ok(s)); const wmean = (f: (s: SegStat) => number) => { let a = 0, n = 0; for (const s of bases) { const v = f(s); if (Number.isFinite(v)) { a += v * s.n; n += s.n; } } return n ? a / n : NaN; };
    const basePass: Record<string, number> = {}; for (const s of bases) for (const p of Object.keys(s.pass)) basePass[p] = 0; for (const p of Object.keys(basePass)) basePass[p] = wmean(s => s.pass[p] ?? 0);
    const bu = wmean(s => s.union), bdt = wmean(s => s.dt), bsum = wmean(s => s.sum);
    const rows: RowPrice[] = [];
    for (const row of begin.rows) {
      const r = per.get(row.id); if (!r || !r.dd.length) continue;
      const movers = [...r.dp.entries()].map(([pass, a]) => ({ pass, back: mean(a) })).filter(m => Math.abs(m.back) >= 0.02).sort((x, y) => Math.abs(y.back) - Math.abs(x.back)).slice(0, 3);
      const union = r.du.length ? spread(r.du) : null, frame = spread(r.dd);
      const byU = !!union && noiseU !== null && row.by !== 'frame'   /* (a row that says its cost is in no pass's stamp is judged by the frame's length: ui/perfsweep.ts readBy) */, each = byU ? r.du : r.dd, disturbed = (byU ? r.gu : r.gd).map(g => g > gap), med = median(each), noise = byU ? noiseU! : noiseD;
      const inNoise = Number.isFinite(noise) ? Math.abs(med) < 2 * medianK(each.length) * noise / Math.sqrt(each.length) : false;   // (the MEDIAN round against the robust noise: one slow round decides nothing)
      disturbedRounds += disturbed.filter(Boolean).length;
      rows.push({ id: row.id, label: row.by === 'frame' ? `[by frame ms] ${row.label}` : row.label   /* (its median, its rounds and its noise are the frame's, whatever the column's head says) */, change: Object.keys(row.to).map(k => `${k} ${fmtV(row.from[k])} → ${fmtV(row.to[k])}`).join(', '), rounds: r.dd.length, union, frame, fps: spread(r.df), sum: r.ds.length ? spread(r.ds) : null, movers, inNoise, per: each, disturbed, median: med });
    }
    out.push({ begin, done, why, segments: segs.length, baseFrames: bases.reduce((a, s) => a + s.n, 0), base: { union: Number.isFinite(bu) ? bu : null, dt: bdt, fps: 1000 / bdt, sum: Number.isFinite(bsum) ? bsum : null, pass: basePass }, noise: { union: noiseU, dt: noiseD, n: nd.length, rmsUnion: rmsU, rmsDt: rmsD, gap }, disturbed: disturbedRounds, rows });
  }
  return out;
}

