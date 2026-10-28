// SMALL RECORDS OF A PAGE VISIT, sent to the backend's `telemetry:put` (cloud/convex/telemetry.ts). Five kinds, each a flat bag of numbers and short words:
//   visit   once, when the page is up. ALL OF IT, since it is what makes one browser look unlike another: `machine` (ui/bench.ts machineFacts — the WebGPU
//           adapter's words, WebGL's name for the chip, the FULL user agent string, browser and major version, OS, threads, memory in GB as the browser
//           rounds it, the screen's and the window's size, pixel ratio, the display's rate, software fallback or not), `gpu` (timestamp queries or not,
//           the canvas format, the device's features and four limits), `bootMs`, the page's query string (without `player`), the browser's language,
//           and `power` (on its battery and how full, or plugged in) where the browser has answered by then
//   boot    once, when the boot is over: its timeline as the page already takes it (ui/boot.ts) — ms to interactive, the ms it waited on shader pipelines, how
//           many there were and how many failed, the three slowest by name, the longest frozen stretch, the first frame's GPU wait, and whether the
//           visit was a reload (a browser keeps compiled shaders between loads, or does not: that is what this record is for)
//   run     when a mission ends: how (done / failed), its seconds and its own tally (game/mission.ts stats), the quality it was drawn at, and `over`: the
//           run's verdict in brief (game/verdict.ts: its kind, the word, won or lost, seconds, the way it was won, a cheat on or not, men lost of how many,
//           defenders down of how many, hostages alive of how many). THE END IS THE VERDICT'S where one is given (`endStage`): only the stealth mission
//           moves the mission's stage word into an end, so a house taken, a rescue and a team wiped wrote no record at all until the page read the verdict
//   perf    once every `perfSecs` of LIVE play: frames, their mean and 50th / 95th / 99th / worst time in ms, how many ran past 33 / 50 / 100 ms, the quality tier
//           and render scale at the end of the stretch (menus, pause cards and a hidden tab are not play and are not counted), the picture's size, `hz` (the
//           display's rate as the page has seen it by then: a `visit` is written before a returning browser has shown a second of frames, so its `machine`
//           carries the rate only on a first visit, where the notice held the record back; a `run` carries it too), and `gpu`:
//           THE GPU'S OWN TIME over the stretch (below). `stalls` / `stall`: the frames past a second while the tab was up, and the longest in ms — a
//           freeze (shaders compiling after the load), which the frame times above leave out. `sharp` / `shed`: sharp direct as drawn, and whether the
//           ladder holds it off — a minute's GPU time means little without the picture it was taken at
//           `lag`: THE GPU'S WAIT ON A FRAME, a stand-in for a frozen picture. A background shader compile can freeze what the browser shows for
//           seconds while animation frames keep coming at full rate (seen on Windows, in a browser's own trace of presented frames): the frame times
//           above look normal all through it, and a page cannot read presented frames. What it can read is how long the GPU takes to say a frame's
//           work is done after the frame was handed over (cloud.ts asks `onSubmittedWorkDone`, one question at a time, on live frames): `n` waits,
//           their `mean` and `worst` ms, and how many ran past 100 / 500 / 2000 ms. UNPROVEN: nobody has yet set a real freeze beside it
//           `power`: the machine's power as the record is written, in the benchmark report's words ("plugged in", "on battery (62 %)"), where
//           the browser says: a laptop on its battery can run the same build far slower. On a `run` and on the `visit` too
//           `cpu`: THE MAIN THREAD'S TIME ON A FRAME. A real Mac played a round at 47 ms a frame with the GPU's timed work at 10 ms, and no record
//           said where the other 35 went: the page's own script, or the GPU outside the timed passes. The loop already times itself every frame
//           for the F9 recorder (main.ts: `perfrec.cpuFrameStart`, the game's update, the engine's render); those numbers are handed in here.
//           Over the stretch's played frames (`n`; a frame the world is held still on, behind a card, is not one): `render` (the engine building
//           the frame's commands) and `whole` (from the frame's start to the records' own call, which comes after the update and the render:
//           update + render + the rest), each as mean / p50 / p95 / worst ms; `update` the same over the frames that ran a game tick (its own
//           `n`: a display faster than the tick rate draws frames between ticks, and those ran no update); `ticks` (game ticks a frame: mean
//           and max — a slow frame runs several). `whole` near the frame's time = the script is the limit; `whole` small beside a long frame
//           = the frame waits on the GPU or the browser
//           `rung`: WHERE THE QUALITY STOOD, AND WHEN IT MOVED. `preset` / `scale` / `sharp` / `shed` above are where the stretch ENDED; the ladder
//           moves inside it (a real Mac's first minute ended on medium at half size, and nothing said whether it fell there in the first seconds,
//           while programs were still building, or was pushed there by the play). `from` = the page's word for its rung as the stretch began
//           (main.ts: preset, scale, the size drawn, 'sharp', 'shed', 'pinned') and `why` = the ladder's own last line by then (ui/quality.ts
//           `lastChange`: 'calibrated (…): 10.0 ms compute at high → low ×0.60 …'); `moves` = [seconds of live play into the stretch, the new word,
//           the ladder's line for it] for each change of either, the first `moves` of them, `more` = how many after that. A fourth entry, 1, marks a
//           change made WHILE PLAY WAS NOT LIVE (a menu, a held brief): its second is where play stood, not when it happened. The page is asked a
//           few times a second on live frames, never every frame
//   error   an error the page caught or was told of (src/cloud/report.ts: `reportError`, and the net under it, every `console.error` and `console.warn`
//           the page makes): its words (cut short), `where` it was said (the reporter's name for the place; 'console.error' for the net;
//           'window.error' / 'unhandledrejection' for what nothing caught), the top of its stack, the place's own small `data`, and `level: 'warning'`
//           for one the game went on from — the first `errors` distinct errors and the first `warnings` distinct warnings of a visit; the count of
//           each rides on the visit's `end`
//   end     as the page goes: seconds of live play and frames this visit
// THE GPU'S TIME costs the frame nothing new. The engine already times its passes every frame on an adapter that can (gpu/timer.ts, what the quality ladder
// reads), and main.ts already folds a frame's compute passes into one number for that ladder: that number is handed in here, where it is one add into a table
// of quarter-milliseconds. No query, no readback and no pass is added for it. One frame in `gpuEvery` the per-pass times the engine holds are added up too (a
// walk of a couple of dozen numbers, about twice a second), and the record carries each pass's mean — in TWO lists: `passes`, the compute passes the frame's
// GPU time is the union of, and `render`, the render passes that union leaves out because a render pass's timestamps are not to be trusted on Metal (the
// first real records had them on top, at twice the frame). An adapter with no timestamp queries has no `gpu`.
// NOTHING ELSE: no key pressed, no name, no address — but a visit's record is keyed to the browser's random id and is, taken together, close to a
// fingerprint of the machine. What of it is kept is the maintainer's choice (docs/agents/workflow.md). Records wait in a short queue and go up every `secs`, and as the page is hidden.
// No game code is imported: the page hands in closures (cloud.ts).
import type { Wire } from './wire';

/** `perfMin`: a tab that is only hidden (it may come back) writes the open stretch of play as a record only once it is this long, s — the first real
 *  visits wrote a record of a tenth of a second at every switch of tabs */
export const TELEMETRY = { secs: 30, perfSecs: 60, perfMin: 5, errors: 16, warnings: 8, queue: 200, batch: 40, gpuEvery: 30, gpuPasses: 32, rungSecs: 0.25, moves: 16 };

export interface PlayFacts {
  scenario: string;
  /** the mission's stage word (game/game.ts MissionStage) and whether a run is live in the player's hands */
  stage: string; live: boolean;
  stats: Record<string, unknown>;
  preset: string; scale: number; auto: boolean;
  /** sharp direct is on as drawn, and the quality ladder is holding it off (ui/quality.ts SHARP SHED) */
  sharp?: boolean; shed?: boolean;
  /** the picture as it is drawn (the engine's own size) and the canvas it is shown on, in pixels */
  res?: [number, number]; out?: [number, number];
  /** the display's rate as the page has seen it, Hz (0 or absent = not seen yet) */
  hz?: number;
  /** the run's verdict in brief, once it has one (a flat bag: the head comment's `over`) */
  over?: Record<string, unknown> | null;
}
/** (the check's red path: CLOUD_BREAK=1 in front of tools/qa/cloud-check.ts — a run's end is read off the mission's stage word alone, as it was; a page has no process) */
const BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.CLOUD_BREAK ?? '') === '1'; } catch { return false; } })();
const ENDS = (stage: string): boolean => stage === 'done' || stage === 'failed';
/** THE STAGE WORD A FRAME IS COUNTED UNDER (the page hands it to `frame`): the run's verdict where it has one, else the mission's own word. A squad's mission
 *  ends by its own rule (a house taken, hostages out, the team wiped) and leaves the mission's word where it was; the verdict knows every kind's end */
export const endStage = (stage: string, over: { won: boolean } | null | undefined): string => over && !BREAK ? over.won ? 'done' : 'failed' : stage;
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
const CBINS = 401, CPER = 2;   // (a frame's main-thread ms in halves, 200 ms = that or longer)
/** the value at share `p` of `n` counts laid in a table of `per` bins a unit (the last bin = that or more) */
const pctOf = (bins: Uint32Array, n: number, p: number, per = 1): number => { let need = Math.ceil(n * p), i = 0; for (; i < bins.length; i++) { need -= bins[i]; if (need <= 0) break; } return Math.min(i, bins.length - 1) / per; };
const GBINS = 201, GPER = 4;   // (a frame's GPU ms in quarters, 50 ms = that or longer)
export class Telemetry {
  readonly stats = { sent: 0, calls: 0, failed: 0, dropped: 0, lastError: '' };
  private q: Event[] = [];
  private bins = new Uint32Array(BINS); private n = 0; private sum = 0; private worst = 0; private secs = 0;
  private played = 0; private frames = 0;
  /** records that wait to go up */
  get waiting(): number { return this.q.length; }
  /** the visit so far: seconds of live play and frames drawn in them */
  get totals(): { played: number; frames: number } { return { played: this.played, frames: this.frames }; }
  private gbins = new Uint32Array(GBINS); private gn = 0; private gsum = 0; private gworst = 0; private gtick = 0; private glooks = 0;
  private gpass = new Map<string, number>();
  private addPass = (ms: number, name: string): void => { if (ms >= 0 && (this.gpass.has(name) || this.gpass.size < TELEMETRY.gpuPasses)) this.gpass.set(name, (this.gpass.get(name) ?? 0) + ms); };
  private stalls = 0; private stall = 0; private wasAway = false;
  private stage = ''; private errs = new Map<string, number>(); private warns = new Set<string>();
  private busy: Promise<void> | null = null;
  constructor(private w: Wire, private o: { pid: string; sid: string; build?: () => string | undefined; may?: () => boolean; now?: () => number; passes?: GpuPasses; render?: readonly string[]; rung?: () => readonly [string, string]; cpu?: (out: Float64Array) => void; power?: () => string }) {}
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
    if (this.o.may && !this.o.may()) { if (this.n || this.frames || this.q.length) { this.zero(); this.played = 0; this.frames = 0; this.endFrames = 0; this.q.length = 0; } this.stage = stage; return; }
    // a run that has just ended, by the stage word moving into an end
    if (stage !== this.stage) {
      const was = this.stage; this.stage = stage;
      // (one end is one record: a word that moves from an end to the other end is the same run, e.g. a mission failed whose verdict then says won)
      if (was && ENDS(stage) && !ENDS(was)) { const F = facts(); this.add('run', { stage, stats: F.stats, preset: F.preset, scale: F.scale, auto: F.auto, ...(F.sharp !== undefined ? { sharp: F.sharp } : {}), ...(F.shed !== undefined ? { shed: F.shed } : {}), ...(F.hz ? { hz: F.hz } : {}), ...(F.over ? { over: F.over } : {}), ...this.powerNow() }, F.scenario); }
    }
    // (the main thread's time on this frame, as the loop took it: update, render, whole, ticks — numbers into tables, nothing made. Before the freeze
    // test below: a frame that follows a long one took its own time whatever the gap before it — but not the first frame after the tab was away,
    // which runs its catch-up ticks and is nothing to the frame times either. The update is counted on the frames that ran a game
    // tick: on a display faster than the tick rate most frames run none, and their zero would be the median)
    if (live && rawDt > 0 && !(rawDt > 1 && this.wasAway) && this.o.cpu && !BREAK) {
      const c = this.cpuIn; c[0] = c[1] = c[2] = c[3] = 0; try { this.o.cpu(c); } catch { c[2] = NaN; }
      if (c[2] >= 0) {   // (a frame the page could not time, or one it holds still behind a card, is left out whole)
        const tk = c[3] >= 0 ? c[3] : 0; this.cn++; this.ticksSum += tk; if (tk > this.ticksMax) this.ticksMax = tk; if (tk >= 1) this.cun++;
        for (let k = tk >= 1 ? 0 : 1; k < 3; k++) { const v = c[k] >= 0 ? c[k] : 0; this.cbins[k][Math.min(CBINS - 1, Math.round(v * CPER))]++; this.csum[k] += v; if (v > this.cworst[k]) this.cworst[k] = v; }
      }
    }
    // (a frame past a second is not a slow frame. With the tab up all the while it is a FREEZE, counted by itself; after the tab was away it is nothing)
    if (rawDt > 1) { if (live && !this.wasAway) { this.stalls++; if (rawDt * 1000 > this.stall) this.stall = Math.min(600000, rawDt * 1000); } this.wasAway = false; return; }
    this.wasAway = false;
    if (!live || !(rawDt > 0)) { if (!live) this.rungGap = true; return; }
    const ms = rawDt * 1000; this.bins[Math.min(BINS - 1, Math.round(ms))]++; this.n++; this.sum += ms; if (ms > this.worst) this.worst = ms;
    this.secs += rawDt; this.played += rawDt; this.frames++;
    if (gpuMs > 0) {
      this.gbins[Math.min(GBINS - 1, Math.round(gpuMs * GPER))]++; this.gn++; this.gsum += gpuMs; if (gpuMs > this.gworst) this.gworst = gpuMs;
      if (this.o.passes && ++this.gtick >= TELEMETRY.gpuEvery) { this.gtick = 0; this.glooks++; this.o.passes().forEach(this.addPass); }
    }
    // (where the quality stands and the ladder's last line, asked every `rungSecs` of live play and on the first live frame after play was held: a
    // change of either is a move of this stretch)
    if (this.o.rung && !BREAK && ((this.rungIn -= rawDt) <= 0 || this.rungGap)) {
      const away = this.rungGap; this.rungIn = TELEMETRY.rungSecs; this.rungGap = false;
      let r: readonly [string, string] | null = null; try { r = this.o.rung(); } catch { /* the page could not say: asked again next time, and no move is made of it */ }
      if (r) {
        const w = String(r[0]).slice(0, 60), why = String(r[1] ?? '').slice(0, 140), t = Math.round(this.secs * 10) / 10;
        if (this.rungFrom === null) { this.rungFrom = w; this.rungFromWhy = why; }
        else if (w !== this.rungNow || why !== this.rungWhy) { if (this.moves.length < TELEMETRY.moves) this.moves.push(away ? [t, w, why, 1] : [t, w, why]); else this.movesMore++; }
        this.rungNow = w; this.rungWhy = why;
      }
    }
    if (this.secs >= TELEMETRY.perfSecs) this.perf(facts());
  }
  private cpuIn = new Float64Array(4); private cn = 0; private cun = 0; private cbins = [new Uint32Array(CBINS), new Uint32Array(CBINS), new Uint32Array(CBINS)]; private csum = [0, 0, 0]; private cworst = [0, 0, 0]; private ticksSum = 0; private ticksMax = 0;
  /** the machine's power as the record is written (cloud.ts powerAsker), where the browser says */
  private powerNow(): { power?: string } { let w = ''; try { w = String(this.o.power?.() ?? '').slice(0, 40); } catch { /* none */ } return w ? { power: w } : {}; }
  /** the stretch's main-thread time: see the head comment's `cpu` */
  private cpu(): unknown {
    const one = (k: number, n: number) => n ? { mean: this.csum[k] / n, p50: pctOf(this.cbins[k], n, 0.5, CPER), p95: pctOf(this.cbins[k], n, 0.95, CPER), worst: Math.min(600000, this.cworst[k]) } : {};
    return { n: this.cn, update: { n: this.cun, ...one(0, this.cun) }, render: one(1, this.cn), whole: one(2, this.cn), ticks: { mean: this.ticksSum / this.cn, max: this.ticksMax } };
  }
  private rungIn = 0; private rungGap = false; private rungFrom: string | null = null; private rungFromWhy = ''; private rungNow = ''; private rungWhy = ''; private moves: (number | string)[][] = []; private movesMore = 0;
  private lagN = 0; private lagSum = 0; private lagWorst = 0; private lag100 = 0; private lag500 = 0; private lag2000 = 0;
  /** the GPU said a live frame's work was done `ms` after the frame was handed over (cloud.ts, one wait at a time): into the minute's record */
  lag(ms: number): void {
    if ((this.o.may && !this.o.may()) || !(ms >= 0)) return;
    this.lagN++; this.lagSum += ms; if (ms > this.lagWorst) this.lagWorst = Math.min(600000, ms); if (ms > 100) this.lag100++; if (ms > 500) this.lag500++; if (ms > 2000) this.lag2000++;
  }
  /** the tab went out of sight (cloud.ts, the page's visibilitychange): the next frame's long gap is not a freeze */
  away(): void { this.wasAway = true; }
  private zero(): void { this.lagN = 0; this.lagSum = 0; this.lagWorst = 0; this.lag100 = 0; this.lag500 = 0; this.lag2000 = 0; this.stalls = 0; this.stall = 0; this.bins.fill(0); this.n = 0; this.sum = 0; this.worst = 0; this.secs = 0; this.gbins.fill(0); this.gn = 0; this.gsum = 0; this.gworst = 0; this.gtick = 0; this.glooks = 0; this.gpass.clear(); this.cn = 0; this.cun = 0; for (let k = 0; k < 3; k++) { this.cbins[k].fill(0); this.csum[k] = 0; this.cworst[k] = 0; } this.ticksSum = 0; this.ticksMax = 0; this.rungFrom = null; this.rungIn = 0; this.moves = []; this.movesMore = 0; }
  private gpct(p: number): number { return pctOf(this.gbins, this.gn, p, GPER); }
  /** the stretch's GPU time: the frames that had a new reading, their mean / 50th / 95th / 99th / worst ms, and each pass's mean over the `looks` taken */
  private gpu(): unknown {
    if (!this.gn) return null;
    const all = this.glooks ? [...this.gpass].map(([k, v]) => [k.slice(0, 40), v / this.glooks] as [string, number]).sort((x, y) => y[1] - x[1]) : [];
    const skip = this.o.render ?? [], passes = all.filter(p => !skip.includes(p[0])), render = all.filter(p => skip.includes(p[0]));
    return { frames: this.gn, mean: this.gsum / this.gn, p50: this.gpct(0.5), p95: this.gpct(0.95), p99: this.gpct(0.99), worst: this.gworst, looks: this.glooks, passes, ...(render.length ? { render } : {}) };
  }
  private pct(p: number): number { return pctOf(this.bins, this.n, p); }
  private over(ms: number): number { let c = 0; for (let i = ms + 1; i < BINS; i++) c += this.bins[i]; return c; }
  private perf(facts: PlayFacts): void {
    if (this.n > 0 || this.stalls > 0 || this.lagN > 0) this.add('perf', { secs: this.secs, frames: this.n, mean: this.n ? this.sum / this.n : 0, p50: this.pct(0.5), p95: this.pct(0.95), p99: this.pct(0.99), worst: this.worst, over33: this.over(33), over50: this.over(50), over100: this.over(100), stalls: this.stalls, stall: this.stall, preset: facts.preset, scale: facts.scale, auto: facts.auto, ...(facts.sharp !== undefined ? { sharp: facts.sharp } : {}), ...(facts.shed !== undefined ? { shed: facts.shed } : {}), ...(facts.res ? { res: facts.res } : {}), ...(facts.out ? { out: facts.out } : {}), ...(facts.hz ? { hz: facts.hz } : {}), ...(this.lagN ? { lag: { n: this.lagN, mean: this.lagSum / this.lagN, worst: this.lagWorst, over100: this.lag100, over500: this.lag500, over2000: this.lag2000 } } : {}), ...(this.cn ? { cpu: this.cpu() } : {}), ...this.powerNow(), ...(this.rungFrom !== null ? { rung: { from: this.rungFrom, ...(this.rungFromWhy ? { why: this.rungFromWhy } : {}), ...(this.moves.length ? { moves: this.moves } : {}), ...(this.movesMore ? { more: this.movesMore } : {}) } } : {}), gpu: this.gpu() }, facts.scenario);
    this.zero();
  }
  /** an error the page caught: its first coming is a record, and its count rides on the visit's `end` */
  error(msg: unknown, more?: { where?: string; level?: string; stack?: string[]; data?: unknown }): void {
    if (this.o.may && !this.o.may()) return;   // (a browser that does not send counts nothing: the error's next coming, once it sends, is its first)
    const text = String((msg as Error | undefined)?.message ?? msg).slice(0, 300), where = String(more?.where ?? '').slice(0, 80), warn = more?.level === 'warning';
    // (one error is one whatever numbers its words carry: a size or an address that changes at each coming does not take a new place each time)
    const same = text.replace(/0x[0-9a-f]+|\d+/gi, '#'), key = where ? `${where}|${same}` : same, had = this.errs.get(key) ?? 0;
    // (warnings have a count of their own: a page that warns a lot does not use up the errors' places)
    if (!had && (warn ? this.warns.size >= TELEMETRY.warnings : this.errs.size - this.warns.size >= TELEMETRY.errors)) return;
    this.errs.set(key, had + 1); if (warn) this.warns.add(key);
    // (the stack as a list of lines, as a dead page's is: each line keeps its length)
    const stack = (more?.stack?.length ? more.stack : String((msg as Error | undefined)?.stack ?? '').split('\n')).slice(0, 6).map(l => String(l).slice(0, 200));
    if (!had) this.add('error', { message: text, stack, ...(where ? { where } : {}), ...(warn ? { level: 'warning' } : {}), ...(more?.data !== undefined && more.data !== null ? { data: more.data } : {}) });
  }
  /** send what waits. `going`: the page is leaving — the visit's `end` and the stretch of play still open go with it. `hidden`: the tab is only out of
   *  sight and may come back — a stretch shorter than `perfMin` stays open, and an `end` is written only where play was added since the last. Never rejects */
  flush(going = false, facts?: PlayFacts, hidden = false): Promise<void> {
    if (this.o.may && !this.o.may()) { this.q.length = 0; return Promise.resolve(); }   // (the switch was turned off with records waiting: they are dropped, not sent)
    if (going && !this.left) {
      // (a going flush with no facts — the device was lost, a dead page's card — sends what waits and the visit's `end`, and leaves the open stretch of
      // play to the leave that has the facts: it used to close the leave, and the stretch before every lost device was in no record)
      if (facts) { this.left = true; if (!(hidden && this.secs < TELEMETRY.perfMin)) this.perf(facts); }
      if (this.frames && this.frames !== this.endFrames) { this.endFrames = this.frames; this.add('end', { played: this.played, frames: this.frames, errors: [...this.errs].reduce((a, [k, n]) => a + (this.warns.has(k) ? 0 : n), 0), ...(this.warns.size ? { warnings: [...this.errs].reduce((a, [k, n]) => a + (this.warns.has(k) ? n : 0), 0) } : {}) }); }
    }
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
  private left = false; private endFrames = 0;
  summary(): string { const S = this.stats; return `telemetry: ${S.sent} records sent (${S.calls} calls), ${this.q.length} waiting${S.failed ? ` · ${S.failed} failed (${S.lastError})` : ''}`; }
}
