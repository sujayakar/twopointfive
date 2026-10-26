// SMALL RECORDS OF A PAGE VISIT, sent to the backend's `telemetry:put` (cloud/convex/telemetry.ts). Five kinds, each a flat bag of numbers and short words:
//   visit   once, when the page is up: the machine (ui/bench.ts machineFacts: adapter, browser, OS, screen, refresh rate, threads), the page's query
//   run     when a mission ends: how (done / failed), its seconds and its own tally (game/mission.ts stats), the quality it was drawn at
//   perf    once every `perfSecs` of LIVE play: frames, their mean and 50th / 95th / 99th / worst time in ms, how many ran past 33 / 50 / 100 ms, the quality tier
//           and render scale at the end of the stretch (menus, pause cards and a hidden tab are not play and are not counted), the picture's size, and `gpu`:
//           THE GPU'S OWN TIME over the stretch (below)
//   error   an error the page caught: its words (cut short), and how many times it has come — the first `errors` distinct ones of a visit
//   end     as the page goes: seconds of live play and frames this visit
// THE GPU'S TIME costs the frame nothing new. The engine already times its passes every frame on an adapter that can (gpu/timer.ts, what the quality ladder
// reads), and main.ts already folds a frame's compute passes into one number for that ladder: that number is handed in here, where it is one add into a table
// of quarter-milliseconds. No query, no readback and no pass is added for it. One frame in `gpuEvery` the per-pass times the engine holds are added up too (a
// walk of a couple of dozen numbers, about twice a second), and the record carries each pass's mean. An adapter with no timestamp queries has no `gpu`.
// NOTHING ELSE: no key pressed, no name, no address. Records wait in a short queue and go up every `secs`, and as the page is hidden.
// No game code is imported: the page hands in closures (cloud.ts).
import type { Wire } from './wire';

export const TELEMETRY = { secs: 30, perfSecs: 60, errors: 8, queue: 200, batch: 40, gpuEvery: 30, gpuPasses: 32 };

export interface PlayFacts {
  scenario: string;
  /** the mission's stage word (game/game.ts MissionStage) and whether a run is live in the player's hands */
  stage: string; live: boolean;
  stats: Record<string, unknown>;
  preset: string; scale: number; auto: boolean;
  /** the picture as it is drawn (the engine's own size) and the canvas it is shown on, in pixels */
  res?: [number, number]; out?: [number, number];
}
/** the engine's per-pass GPU times of the last frame it read back, ms by pass name (gpu/timer.ts `lastFrame`): read here, never asked for */
export type GpuPasses = () => { forEach(fn: (ms: number, name: string) => void): void };
interface Event { at: number; kind: string; scenario?: string; data: unknown }

const KEY = /^[A-Za-z][A-Za-z0-9_]{0,63}$/;
/** a bag as the backend takes it: plain keys only, short strings, finite numbers, shallow */
export function clean(v: unknown, depth = 0): unknown {
  if (v === null || v === undefined) return null;
  if (typeof v === 'number') return Number.isFinite(v) ? Math.round(v * 1000) / 1000 : null;
  if (typeof v === 'boolean') return v;
  if (typeof v === 'string') return v.slice(0, 400);
  if (depth >= 4) return null;
  if (Array.isArray(v)) return v.slice(0, 64).map(x => clean(x, depth + 1));
  if (typeof v === 'object') { const o: Record<string, unknown> = {}; for (const [k, x] of Object.entries(v as object)) if (KEY.test(k) && x !== undefined && typeof x !== 'function') o[k] = clean(x, depth + 1); return o; }
  return null;
}

const BINS = 251;   // (a frame's ms, whole, 250 = that or longer)
const GBINS = 201, GPER = 4;   // (a frame's GPU ms in quarters, 50 ms = that or longer)
export class Telemetry {
  readonly stats = { sent: 0, calls: 0, failed: 0, dropped: 0, lastError: '' };
  private q: Event[] = [];
  private bins = new Uint32Array(BINS); private n = 0; private sum = 0; private worst = 0; private secs = 0;
  private played = 0; private frames = 0;
  private gbins = new Uint32Array(GBINS); private gn = 0; private gsum = 0; private gworst = 0; private gtick = 0; private glooks = 0;
  private gpass = new Map<string, number>();
  private addPass = (ms: number, name: string): void => { if (ms >= 0 && (this.gpass.has(name) || this.gpass.size < TELEMETRY.gpuPasses)) this.gpass.set(name, (this.gpass.get(name) ?? 0) + ms); };
  private stage = ''; private errs = new Map<string, number>();
  private busy: Promise<void> | null = null;
  constructor(private w: Wire, private o: { pid: string; sid: string; build?: () => string | undefined; may?: () => boolean; now?: () => number; passes?: GpuPasses }) {}
  private now(): number { return (this.o.now ?? Date.now)(); }

  add(kind: string, data: unknown, scenario?: string): void {
    if (this.o.may && !this.o.may()) return;
    if (this.q.length >= TELEMETRY.queue) { this.q.shift(); this.stats.dropped++; }
    this.q.push({ at: this.now(), kind, ...(scenario ? { scenario: scenario.slice(0, 120) } : {}), data: clean(data) });
  }
  /** one drawn frame: `rawDt` its wall-clock seconds, `live` a run is in the player's hands, `stage` the mission's stage word, `gpuMs` the GPU time of the
   *  frame the engine last read back where that is NEW this frame (else NaN or 0: no timings, or none new). `facts` is asked only when a record is written.
   *  Numbers in, no object made: this runs every frame */
  frame(rawDt: number, live: boolean, stage: string, gpuMs: number, facts: () => PlayFacts): void {
    // (a browser that does not send counts nothing: what was played while its switch was off is in no later record)
    if (this.o.may && !this.o.may()) { if (this.n || this.frames || this.q.length) { this.zero(); this.played = 0; this.frames = 0; this.q.length = 0; } this.stage = stage; return; }
    // a run that has just ended, by the stage word moving into an end
    if (stage !== this.stage) {
      const was = this.stage; this.stage = stage;
      if (was && (stage === 'done' || stage === 'failed')) { const F = facts(); this.add('run', { stage, stats: F.stats, preset: F.preset, scale: F.scale, auto: F.auto }, F.scenario); }
    }
    if (!live || !(rawDt > 0) || rawDt > 1) return;   // (a frame past a second is a tab that slept, not a slow frame)
    const ms = rawDt * 1000; this.bins[Math.min(BINS - 1, Math.round(ms))]++; this.n++; this.sum += ms; if (ms > this.worst) this.worst = ms;
    this.secs += rawDt; this.played += rawDt; this.frames++;
    if (gpuMs > 0) {
      this.gbins[Math.min(GBINS - 1, Math.round(gpuMs * GPER))]++; this.gn++; this.gsum += gpuMs; if (gpuMs > this.gworst) this.gworst = gpuMs;
      if (this.o.passes && ++this.gtick >= TELEMETRY.gpuEvery) { this.gtick = 0; this.glooks++; this.o.passes().forEach(this.addPass); }
    }
    if (this.secs >= TELEMETRY.perfSecs) this.perf(facts());
  }
  private zero(): void { this.bins.fill(0); this.n = 0; this.sum = 0; this.worst = 0; this.secs = 0; this.gbins.fill(0); this.gn = 0; this.gsum = 0; this.gworst = 0; this.gtick = 0; this.glooks = 0; this.gpass.clear(); }
  private gpct(p: number): number { let need = Math.ceil(this.gn * p), i = 0; for (; i < GBINS; i++) { need -= this.gbins[i]; if (need <= 0) break; } return Math.min(i, GBINS - 1) / GPER; }
  /** the stretch's GPU time: the frames that had a new reading, their mean / 50th / 95th / 99th / worst ms, and each pass's mean over the `looks` taken */
  private gpu(): unknown {
    if (!this.gn) return null;
    const passes = this.glooks ? [...this.gpass].map(([k, v]) => [k.slice(0, 40), v / this.glooks] as [string, number]).sort((x, y) => y[1] - x[1]) : [];
    return { frames: this.gn, mean: this.gsum / this.gn, p50: this.gpct(0.5), p95: this.gpct(0.95), p99: this.gpct(0.99), worst: this.gworst, looks: this.glooks, passes };
  }
  private pct(p: number): number { let need = Math.ceil(this.n * p), i = 0; for (; i < BINS; i++) { need -= this.bins[i]; if (need <= 0) break; } return Math.min(i, BINS - 1); }
  private over(ms: number): number { let c = 0; for (let i = ms + 1; i < BINS; i++) c += this.bins[i]; return c; }
  private perf(facts: PlayFacts): void {
    if (this.n > 0) this.add('perf', { secs: this.secs, frames: this.n, mean: this.sum / this.n, p50: this.pct(0.5), p95: this.pct(0.95), p99: this.pct(0.99), worst: this.worst, over33: this.over(33), over50: this.over(50), over100: this.over(100), preset: facts.preset, scale: facts.scale, auto: facts.auto, ...(facts.res ? { res: facts.res } : {}), ...(facts.out ? { out: facts.out } : {}), gpu: this.gpu() }, facts.scenario);
    this.zero();
  }
  /** an error the page caught: its first coming is a record, and its count rides on the visit's `end` */
  error(msg: unknown): void {
    if (this.o.may && !this.o.may()) return;   // (a browser that does not send counts nothing: the error's next coming, once it sends, is its first)
    const text = String((msg as Error | undefined)?.message ?? msg).slice(0, 300), had = this.errs.get(text) ?? 0;
    if (!had && this.errs.size >= TELEMETRY.errors) return;
    this.errs.set(text, had + 1);
    if (!had) this.add('error', { message: text, stack: String((msg as Error | undefined)?.stack ?? '').split('\n').slice(0, 6).join('\n').slice(0, 400) });
  }
  /** send what waits. `going`: the page is leaving — the visit's `end` and the stretch of play still open go with it. Never rejects */
  flush(going = false, facts?: PlayFacts): Promise<void> {
    if (this.o.may && !this.o.may()) { this.q.length = 0; return Promise.resolve(); }   // (the switch was turned off with records waiting: they are dropped, not sent)
    if (going && !this.left) { this.left = true; if (facts) this.perf(facts); if (this.frames) this.add('end', { played: this.played, frames: this.frames, errors: [...this.errs.values()].reduce((a, b) => a + b, 0) }); }
    if (this.busy && !going) return this.busy;
    const run = (async () => {
      while (this.q.length) {
        // (a batch LEAVES the queue as it is asked, and comes back to its front where the call did not get there: two flushes at once — the tab hidden and
        // the page going fire together — never send one record twice)
        const events = this.q.splice(0, TELEMETRY.batch), build = this.o.build?.();
        try { await this.w.mutation('telemetry:put', { pid: this.o.pid, sid: this.o.sid, ...(build ? { build } : {}), events }, { keepalive: going }); this.stats.sent += events.length; this.stats.calls++; }
        catch (e) { this.stats.failed++; this.stats.lastError = String((e as Error | undefined)?.message ?? e).slice(0, 200); if ((e as { refused?: boolean })?.refused) this.stats.dropped += events.length; else { this.q.unshift(...events); break; } }
      }
    })();
    const p: Promise<void> = run.finally(() => { if (this.busy === p) this.busy = null; }); this.busy = p;
    return p;
  }
  /** (a page the browser brings back from its cache is the same visit going on: it may say `end` again when it next goes) */
  back(): void { this.left = false; }
  private left = false;
  summary(): string { const S = this.stats; return `telemetry: ${S.sent} records sent (${S.calls} calls), ${this.q.length} waiting${S.failed ? ` · ${S.failed} failed (${S.lastError})` : ''}`; }
}
