// SMALL RECORDS OF A PAGE VISIT, sent to the backend's `telemetry:put` (cloud/convex/telemetry.ts). Five kinds, each a flat bag of numbers and short words:
//   visit   once, when the page is up: the machine (ui/bench.ts machineFacts: adapter, browser, OS, screen, refresh rate, threads), the page's query
//   run     when a mission ends: how (done / failed), its seconds and its own tally (game/mission.ts stats), the quality it was drawn at
//   perf    once every `perfSecs` of LIVE play: frames, their mean and 50th / 95th / 99th / worst time in ms, how many ran past 33 / 50 / 100 ms, the quality tier
//           and render scale at the end of the stretch (menus, pause cards and a hidden tab are not play and are not counted)
//   error   an error the page caught: its words (cut short), and how many times it has come — the first `errors` distinct ones of a visit
//   end     as the page goes: seconds of live play and frames this visit
// NOTHING ELSE: no key pressed, no name, no address. Records wait in a short queue and go up every `secs`, and as the page is hidden.
// No game code is imported: the page hands in closures (cloud.ts).
import type { Wire } from './wire';

export const TELEMETRY = { secs: 30, perfSecs: 60, errors: 8, queue: 200, batch: 40 };

export interface PlayFacts {
  scenario: string;
  /** the mission's stage word (game/game.ts MissionStage) and whether a run is live in the player's hands */
  stage: string; live: boolean;
  stats: Record<string, unknown>;
  preset: string; scale: number; auto: boolean;
}
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
export class Telemetry {
  readonly stats = { sent: 0, calls: 0, failed: 0, dropped: 0, lastError: '' };
  private q: Event[] = [];
  private bins = new Uint32Array(BINS); private n = 0; private sum = 0; private worst = 0; private secs = 0;
  private played = 0; private frames = 0;
  private stage = ''; private errs = new Map<string, number>();
  private busy: Promise<void> | null = null;
  constructor(private w: Wire, private o: { pid: string; sid: string; build?: () => string | undefined; may?: () => boolean; now?: () => number }) {}
  private now(): number { return (this.o.now ?? Date.now)(); }

  add(kind: string, data: unknown, scenario?: string): void {
    if (this.o.may && !this.o.may()) return;
    if (this.q.length >= TELEMETRY.queue) { this.q.shift(); this.stats.dropped++; }
    this.q.push({ at: this.now(), kind, ...(scenario ? { scenario: scenario.slice(0, 120) } : {}), data: clean(data) });
  }
  /** one drawn frame: `rawDt` its wall-clock seconds, `facts` the game as it stands (asked every frame: a closure over the page's own objects) */
  frame(rawDt: number, facts: PlayFacts): void {
    // (a browser that does not send counts nothing: what was played while its switch was off is in no later record)
    if (this.o.may && !this.o.may()) { if (this.n || this.frames || this.q.length) { this.bins.fill(0); this.n = 0; this.sum = 0; this.worst = 0; this.secs = 0; this.played = 0; this.frames = 0; this.q.length = 0; } this.stage = facts.stage; return; }
    // a run that has just ended, by the stage word moving into an end
    if (facts.stage !== this.stage) {
      const was = this.stage; this.stage = facts.stage;
      if (was && (facts.stage === 'done' || facts.stage === 'failed')) this.add('run', { stage: facts.stage, stats: facts.stats, preset: facts.preset, scale: facts.scale, auto: facts.auto }, facts.scenario);
    }
    if (!facts.live || !(rawDt > 0) || rawDt > 1) return;   // (a frame past a second is a tab that slept, not a slow frame)
    const ms = rawDt * 1000; this.bins[Math.min(BINS - 1, Math.round(ms))]++; this.n++; this.sum += ms; if (ms > this.worst) this.worst = ms;
    this.secs += rawDt; this.played += rawDt; this.frames++;
    if (this.secs >= TELEMETRY.perfSecs) this.perf(facts);
  }
  private pct(p: number): number { let need = Math.ceil(this.n * p), i = 0; for (; i < BINS; i++) { need -= this.bins[i]; if (need <= 0) break; } return Math.min(i, BINS - 1); }
  private over(ms: number): number { let c = 0; for (let i = ms + 1; i < BINS; i++) c += this.bins[i]; return c; }
  private perf(facts: PlayFacts): void {
    if (this.n > 0) this.add('perf', { secs: this.secs, frames: this.n, mean: this.sum / this.n, p50: this.pct(0.5), p95: this.pct(0.95), p99: this.pct(0.99), worst: this.worst, over33: this.over(33), over50: this.over(50), over100: this.over(100), preset: facts.preset, scale: facts.scale, auto: facts.auto }, facts.scenario);
    this.bins.fill(0); this.n = 0; this.sum = 0; this.worst = 0; this.secs = 0;
  }
  /** an error the page caught: its first coming is a record, and its count rides on the visit's `end` */
  error(msg: unknown): void {
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
