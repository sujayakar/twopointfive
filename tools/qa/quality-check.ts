// THE QUALITY LADDER'S ONE CHECK: what src/ui/quality.ts decides, which preset at which scale and when it steps. No GPU: the real class over a stub engine, fed frame times and GPU unions.
//
//   bun run tools/qa/quality-check.ts      every check, on synthetic machines (exit 1 on a FAIL). ADAPT_BREAK, LADDERHELD_BREAK, ONEGRID_BREAK, ONESOLVE_BREAK, FIRESHED_BREAK, FOGROWS_BREAK, LADDERSHED_BREAK, LADDERHOLD_BREAK = 1: each must fail
//   bun run tools/qa/quality-check.ts --plan [--budget 15] [--k 4.5 --css 1440x900 --dpr 2]     the cost table and plan()'s picks for machines (the flags add one of your own)
//   bun run tools/qa/quality-check.ts <perf-….json>… [--aim asnow|60|90|120|display|all] [--hz N] [--paced on|off] [--held on|off] [--shed on|off] [--sharpk N] [--sweeps] [--check] [--timeline] [--hold k=v,…]
//
// THE REPLAY OVER AN F9 RECORDING.
// An F9 dump (ui/perfrec.ts) carries, per frame, the frame time, the compute passes' GPU union, the resolution and the preset, and the ladder's own moves as events. That is everything
// `Quality.sample()` is fed in the page, so the real class can be driven by a recording:
//
//   · while the replayed ladder sits where the recording's sat, a frame is fed back exactly as recorded (dt, union; NaN where the readback was stale — two frames with the same pass table);
//   · where it sits elsewhere (another aim cut the scale, dropped a preset, climbed a rung) the frame is RE-PRICED by the cost model: union' = union × model(replayed state) ÷ model(recorded
//     state), frame' = max(display interval, union' + the frame's own overhead), and the recorded second is replayed as however many of those frames fit in it (the scene's timeline stays the
//     recording's). MODEL below = quality.ts presetCost + two corrections measured on the Mac's sweep take (take 3): a fixed 1.6 model-ms the table does not carry, and ×1.07 on presets under
//     high. Scale steps at high then land within 1 % of the sweep's measured flips; medium within the correction; low / verylow are the model alone.
//
//   --check   as-now only: the replayed moves must be the recording's own 'ladder' events (same moves, same order, each within 2.5 s). The ladder's counters at the moment F9 was pressed are
//             not in a dump, so the window phase and the roomy count are tried over their few values and the best fit is named. Exit 1 if no start state reproduces the take.
import { readFileSync } from 'node:fs';
import { PRESETS, REF, SCALE_FLOOR, LADDER, CLIMB, HOLD, PACED, SHED, sharpForce, AIM_DISPLAY, PLAN_BUDGET_MS, QUALITY_NAMES, ladderBands, ladderAimOf, visitAdaptive, displayRateOf, presetCost, plan, costTable, COST, AUTO_LADDER, SCALE_STEP, STANDS_IN, type QualityName, type Preset } from '../../src/ui/quality.ts';
import type { PerfDump, PerfFrame } from '../../src/ui/perfrec.ts';

const G = globalThis as unknown as Record<string, any>;
G.GPUBufferUsage ??= { MAP_READ: 1, MAP_WRITE: 2, COPY_SRC: 4, COPY_DST: 8, INDEX: 16, VERTEX: 32, UNIFORM: 64, STORAGE: 128, INDIRECT: 256, QUERY_RESOLVE: 512 };
G.GPUTextureUsage ??= { COPY_SRC: 1, COPY_DST: 2, TEXTURE_BINDING: 4, STORAGE_BINDING: 8, RENDER_ATTACHMENT: 16 };
G.GPUShaderStage ??= { VERTEX: 1, FRAGMENT: 2, COMPUTE: 4 }; G.GPUMapMode ??= { READ: 1, WRITE: 2 };
const store = new Map<string, string>();
G.localStorage = { getItem: (k: string) => store.get(k) ?? null, setItem: (k: string, v: string) => { store.set(k, String(v)); }, removeItem: (k: string) => { store.delete(k); } };
G.document = { hidden: false }; G.window = { devicePixelRatio: 2, __testVisible: true };
const { Quality } = await import('../../src/ui/quality.ts');
const { smokeLookStep } = await import('../../src/render/smokeLook.ts');   // the march's step as the page marches it: the preset's row in proportion to the look table
const { defaultRcConfig } = await import('../../src/render/rc.ts');
const { laterProgress } = await import('../../src/gpu/device.ts');
const { PerfRecorder } = await import('../../src/ui/perfrec.ts');
const defaults = (await import('../../src/engine.ts')).defaultSettings as unknown as () => Record<string, any>;

// ---- the response model: what the union does when the ladder sits somewhere else -------------------------------------------------------------------------------------------------------------
/** measured on the Mac's sweep take (take 3: High ×0.95 gather ½, 13.9 ms union): 0.95 → 0.90 gave back 0.94 ms (ratio 0.932), 0.95 → 0.77 gave
 *  3.27 ms (0.765), all of Medium 7.62 ms (0.452). presetCost alone says 0.924 / 0.747 / 0.380: its fixed part is short of what a frame carries today (fog of war, the mesh refit, the
 *  probe's traffic). `extraFixed` model-ms on every preset puts the two scale steps at 0.930 / 0.765; `under` on the presets below high puts Medium at 0.452. */
const MODEL = { extraFixed: 1.6, under: 1.07, overheadMs: 2.8, sharpK: 1 };   // (sharpK: sharp direct's part × this — the same Mac's sweep of 10-25 priced it at 4.9 of 15.5 ms at High ×0.5, well over presetCost's share: the fog pass, the g-buffer and the composite at the output size ride with it; 1 = the model's own, as every machine here before SHARP SHED ran)
interface State { preset: QualityName; scale: number; gatherThird: boolean; volQuarter: boolean; maxH: number; sharp: boolean }
interface Win { cssW: number; cssH: number; dpr: number }
const rowsOf = (s: State, w: Win) => Math.min(s.maxH, w.cssH * w.dpr) * s.scale;
function modelCost(s: State, w: Win, base: Record<string, any>): number {
  const pc = PRESETS[s.preset];   // (a take is priced on the rows the presets have NOW: one from before 2026-11-07 ran the froxel path under high, and its ladder is replayed on today's prices)
  const live: Preset = { ...pc, lossy: { ...pc.lossy, gatherThird: s.gatherThird, volQuarter: s.volQuarter }, maxH: s.maxH, scale: s.scale, iq: { ...pc.iq, sharp: s.sharp && base.taau !== false } };
  const c = presetCost(live, base.oneCacheGrid === true); const rows = rowsOf(s, w); const mpx = rows * rows * (w.cssW / w.cssH) / 1e6;   // (a take from before ONE CACHE GRID carries no such setting: its low and verylow ran the coarse grid and are priced on it, as replay() has the ladder price them)
  return (s.preset === 'high' ? 1 : MODEL.under) * (c.fixed + (MODEL.sharpK - 1) * c.passes.sharp + MODEL.extraFixed + c.pixel * mpx / REF.mpx);
}

// ---- a stub engine on the recording's window ---------------------------------------------------------------------------------------------------------------------------------------------------
function mkEngine(w: Win, settings: Record<string, any>) {
  G.window.devicePixelRatio = w.dpr;
  const E: any = { settings: { ...defaults(), ...settings }, rc: { cfg: { ...defaultRcConfig } }, rebuildRc(cfg: any) { this.rc.cfg = cfg; }, gpu: { canvas: { clientHeight: w.cssH, clientWidth: w.cssW } },
    get capScale() { return Math.min(1, this.settings.maxInternalHeight / (w.cssH * w.dpr)); }, get effectiveScale() { return this.capScale * Math.min(1, this.settings.renderScale); },
    timer: { enabled: true, results: new Map<string, number>(), total(excl: string[]) { let s = 0; for (const [n, v] of this.results) if (!excl.includes(n)) s += v; return s; }, unionMs() { return 0; } } };
  return E;
}
/** a Quality standing where a session stands once its boot is behind it: `preset` applied, calibration done, auto or pinned-with-adaptive, then the recording's own settings laid over */
function mkQuality(w: Win, preset: QualityName, auto: boolean, settings: Record<string, any>, aim: number, hz: number, shed0 = false) {
  store.clear();
  const E = mkEngine(w, {}); const q: any = new Quality(E as never, { params: { rbIters: 4 } } as never);
  q.auto = auto; q.apply(preset); q.adaptive = true; q.climbAllowed = true; q.calib = { ...q.calib, done: true };
  for (const k of ['renderScale', 'gatherThird', 'volQuarter', 'ladderClimb', 'ladderPaced', 'ladderHeld', 'oneCacheGrid', 'taau', 'sharpDirect', 'rcHalfRate', 'maxInternalHeight']) if (k in settings) E.settings[k] = settings[k];
  E.settings.ladderShed = settings.ladderShed === true;   // (SHARP SHED only where a machine or a take says so: every case and every take from before the switch runs the ladder as it was)
  if (shed0 && auto && E.settings.ladderShed && !E.settings.sharpDirect) q.shed = true;   // (a take begun with the shed standing, PerfDump.shedAtStart: the ladder knows sharp direct is its own to bring back)
  E.settings.ladderAim = aim; q.displayHzPin = hz; q.cooldown = 0; q.lastChange = '';
  return { E, q };
}
const stateOf = (q: any, E: any): State => ({ preset: q.current, scale: E.settings.renderScale, gatherThird: !!E.settings.gatherThird, volQuarter: !!E.settings.volQuarter, maxH: E.settings.maxInternalHeight, sharp: !!E.settings.sharpDirect });

// ---- the replay --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
interface Move { t: number; text: string; state: State; rows: number }
interface Result { aim: number; target: number; moves: Move[]; secs: number; meanFps: number; held: number; worst1: number; worst2: number; rowsMean: number; rowsMin: number; rowsMax: number; presetShare: Record<string, number>; scaleShare: Record<string, number>;
  timeline: { t0: number; fps: number; worst: number; union: number; preset: string; scale: number; rows: number }[]; overhead: number; hz: number; win: Win; recorded: { meanFps: number; worst1: number; worst2: number; held: (fps: number) => number } }
interface Warm { acc: number; roomy: number }   // the ladder's state when F9 was pressed, as far as a dump cannot say: how far into its 1-s window it was, how many roomy windows it had counted (its last step taken long ago)

function loadDump(path: string): PerfDump { return JSON.parse(readFileSync(path, 'utf8')) as PerfDump; }
const presetOfLabel = (label: string): { preset: QualityName; auto: boolean } => { const m = /^auto \((\w+)\)$/.exec(label); return m ? { preset: m[1] as QualityName, auto: true } : { preset: label as QualityName, auto: false }; };
const gpuKey = (f: PerfFrame) => { let s = ''; for (const k in f.gpu) s += k + ':' + f.gpu[k] + ','; return s; };
function windowOf(d: PerfDump): Win {
  const f = d.frames.find(x => x.eff > 0 && x.res?.[1] > 0)!; const dpr = d.dpr || 1;
  const devH = f.res[1] / f.eff, devW = f.res[0] / f.eff; return { cssW: Math.round(devW / dpr), cssH: Math.round(devH / dpr), dpr };
}
/** the display's rate as the take shows it: the page's own estimator (quality.ts displayRateOf — the fastest eight consecutive frames) over the recorded frame times */
function hzOf(d: PerfDump): number { return displayRateOf(d.frames.map(f => f.dt).filter(x => x > 2 && x < 100), 0.08) || 60; }   // (8 %: a dump's dt is the callback's clock, a millisecond looser than the rAF timestamps the page reads)

/** --paced on|off: the as-now paced check (settings.ladderPaced) forced for a replay; unset = the take's own setting — a take from before the switch has none and replays under the rule as it was */
let pacedForce: boolean | undefined;
/** --held on|off: the same check under an aim (settings.ladderHeld) forced for a replay; unset = the take's own setting, and a take from before the switch (10-14) has none and replays
 *  with it OFF, as its page ran — `--held on` asks what the rule would have done with that take */
let heldForce: boolean | undefined;
/** --shed on|off: SHARP SHED (settings.ladderShed) forced for a replay; unset = the take's own setting, and a take from before the switch (10-25) replays with it OFF, as its page ran —
 *  `--shed on` asks what the rule would have done with that take (`--sharpk N` = the response model's price of sharp direct, MODEL.sharpK) */
let shedForce: boolean | undefined;
function replay(d: PerfDump, aim: number, opts: { hz?: number; warm?: Warm } = {}): Result {
  const w = windowOf(d); const hz = opts.hz ?? hzOf(d); const S0 = d.settingsAtStart as Record<string, any>;
  const first = d.frames[0]; const { preset: p0, auto } = presetOfLabel(first.preset);
  const { E, q } = mkQuality(w, p0, auto, S0, aim, hz, d.shedAtStart === true); q.adaptive = first.adaptive !== false;   // (the take's own row 'Hold frame rate': a take recorded with it off, `?adapt=0`, is replayed with it off)
  if (pacedForce !== undefined) E.settings.ladderPaced = pacedForce; if (heldForce !== undefined) E.settings.ladderHeld = heldForce; else if (!('ladderHeld' in S0)) E.settings.ladderHeld = false;
  if (shedForce !== undefined) E.settings.ladderShed = shedForce;
  if (!('oneCacheGrid' in S0)) E.settings.oneCacheGrid = false;   // (a take from before ONE CACHE GRID: its low and verylow ran the coarse grid — the replayed ladder prices them so, as modelCost does)
  if (opts.warm) { q.sinceClimb = 99; q.acc = opts.warm.acc; q.frames = Math.max(1, Math.round(opts.warm.acc / (first.dt / 1000))); q.roomy = opts.warm.roomy; q.unionAcc = (first.gpuUnion ?? 0) * q.frames; q.unionN = first.gpuUnion ? q.frames : 0; }
  // the recorded ladder's state per frame: the scale off the resolution, the divisors off the 'settings' events, the preset off the label
  const rec: State = { preset: p0, scale: S0.renderScale, gatherThird: !!S0.gatherThird, volQuarter: !!S0.volQuarter, maxH: S0.maxInternalHeight, sharp: 'sharpDirect' in S0 ? !!S0.sharpDirect : !!E.settings.sharpDirect };   // (a take from before the key: as the replayed preset places it, so the two sides price alike)
  const setEv = d.events.filter(e => e.kind === 'settings'); let se = 0;
  const applySettings = (text: string) => { for (const m of text.matchAll(/(\w+) (\S+?) → ([^\s,]+)/g)) { const [, k, , v] = m; if (k === 'gatherThird') rec.gatherThird = v === 'true'; else if (k === 'volQuarter') rec.volQuarter = v === 'true'; else if (k === 'sharpDirect') rec.sharp = v === 'true'; else if (k === 'maxInternalHeight') rec.maxH = +v; } };
  // the frame's own overhead past the union, for frames the display paced (a 120 Hz frame with 4 ms of GPU work is not '4.3 ms of overhead'): the take's median over its GPU-bound frames
  // (… and not the frames something else HELD: one more than a whole refresh + HOLD.pacedMs over its union is no vsync the GPU missed — a browser holding the page to 30 fps, a CPU hitch.
  // Such a frame stays as long as it was whatever the ladder renders, until the re-priced work itself outgrows it; before 10-14 its whole wait counted as overhead that moved with the union)
  const heldOut = (dt: number, u: number) => u > 0 && dt - u > 1000 / hz + HOLD.pacedMs;
  const over: number[] = []; for (const f of d.frames) if (f.gpuUnion && f.dt > 1000 / hz * 1.15 && f.dt < 60 && !heldOut(f.dt, f.gpuUnion)) over.push(f.dt - f.gpuUnion);
  over.sort((a, b) => a - b); const overMed = over.length > 50 ? over[over.length >> 1] : MODEL.overheadMs;
  const B = ladderBands(aim, hz); const target = B.hold ? B.target : 60;
  const moves: Move[] = []; let last = q.lastChange; let carry = 0; let lastUnion = 0; let prevKey = '';
  let tSum = 0, nFrames = 0, heldT = 0, rowsT = 0, rowsMin = 1e9, rowsMax = 0; const presetT: Record<string, number> = {}, scaleT: Record<string, number> = {};
  const wins: { t: number; n: number; r: number; u: number; rows: number; rT: number; rN: number }[] = [];   // 1-s windows of the scene's timeline: frames shown, time, mean union; the recording's own beside
  for (const f of d.frames) {
    while (se < setEv.length && setEv[se].t <= f.t) applySettings(setEv[se++].text);
    { const labelNow = presetOfLabel(f.preset).preset; if (labelNow !== rec.preset) { rec.preset = labelNow; rec.maxH = PRESETS[labelNow].maxH; rec.gatherThird = PRESETS[labelNow].lossy.gatherThird; rec.volQuarter = PRESETS[labelNow].lossy.volQuarter; } }   // (a recorded preset hop: its rows — the 'settings' event of a hop lists only its first eight keys)
    if (f.res?.[1] > 0) rec.scale = +(f.res[1] / Math.min(rec.maxH, w.cssH * w.dpr)).toFixed(2);
    if (!(f.dt < 500)) continue;   // (host/drawpage.ts feeds no frame of half a second or more)
    const key = gpuKey(f); const stale = key === prevKey; prevKey = key;
    const union = f.gpuUnion ?? 0; if (union > 0 && !stale) lastUnion = union;
    const sim = stateOf(q, E);
    const same = sim.preset === rec.preset && Math.abs(sim.scale - rec.scale) < 0.004 && sim.gatherThird === rec.gatherThird && sim.volQuarter === rec.volQuarter && sim.maxH === rec.maxH && sim.sharp === rec.sharp;
    let dt2 = f.dt, union2 = union;
    if (!same) {
      const R = modelCost(sim, w, S0) / modelCost(rec, w, S0); const u = union > 0 ? union : lastUnion;
      union2 = union > 0 ? union * R : 0;
      const paced = f.dt <= 1000 / hz * 1.15; const o = paced ? Math.min(Math.max(0, f.dt - u), overMed) : Math.max(0, f.dt - u);
      dt2 = u > 0 ? (heldOut(f.dt, u) ? Math.max(f.dt, u * R + overMed) : Math.max(1000 / hz, u * R + o)) : f.dt;
    }
    // the scene's f.dt of wall time, shown as frames of dt2
    const rows = rowsOf(sim, w); const sec = Math.floor(f.t / 1000);
    const wn = (wins[sec] ??= { t: 0, n: 0, r: 0, u: 0, rows: 0, rT: 0, rN: 0 }); wn.t += f.dt; wn.n += f.dt / dt2; wn.u += union2 * f.dt; wn.rows += rows * f.dt; wn.rT += f.dt; wn.rN += 1;
    tSum += f.dt; nFrames += f.dt / dt2; rowsT += rows * f.dt; rowsMin = Math.min(rowsMin, rows); rowsMax = Math.max(rowsMax, rows);
    presetT[sim.preset] = (presetT[sim.preset] ?? 0) + f.dt; { const k = `${sim.preset} ×${sim.scale.toFixed(2)}`; scaleT[k] = (scaleT[k] ?? 0) + f.dt; }
    carry += f.dt / dt2;
    while (carry >= 1) {
      carry -= 1; q.sample(dt2 / 1000, E.timer.enabled ? (union <= 0 ? 0 : stale ? NaN : union2) : 0);
      if (q.lastChange !== last) { last = q.lastChange; const st = stateOf(q, E); moves.push({ t: f.t / 1000, text: last, state: st, rows: rowsOf(st, w) }); }
    }
  }
  const W = wins.filter(x => x && x.t > 500); const fpsW = W.map(x => 1000 * x.n / x.t); const recW = W.map(x => 1000 * x.rN / x.rT);
  for (const x of W) if (1000 * x.n / x.t >= HOLD.cut * target) heldT += x.t;
  const worst2 = (a: number[]) => { let m = 1e9; for (let i = 0; i + 1 < a.length; i++) m = Math.min(m, (a[i] + a[i + 1]) / 2); return m; };
  const timeline: Result['timeline'] = []; const step = 15;
  for (let i = 0; i < wins.length; i += step) { const seg = wins.slice(i, i + step).filter(x => x && x.t > 500); if (!seg.length) continue; const t = seg.reduce((a, x) => a + x.t, 0);
    const lastMove = [...moves].reverse().find(m => m.t <= i + step); const st = lastMove?.state ?? { preset: p0, scale: S0.renderScale } as State;
    timeline.push({ t0: i, fps: 1000 * seg.reduce((a, x) => a + x.n, 0) / t, worst: Math.min(...seg.map(x => 1000 * x.n / x.t)), union: seg.reduce((a, x) => a + x.u, 0) / t, preset: st.preset, scale: st.scale, rows: seg.reduce((a, x) => a + x.rows, 0) / t }); }
  const share = (m: Record<string, number>) => Object.fromEntries(Object.entries(m).map(([k, v]) => [k, v / tSum]));
  const totT = W.reduce((a, x) => a + x.t, 0);
  return { aim, target, moves, secs: tSum / 1000, meanFps: 1000 * nFrames / tSum, held: heldT / totT, worst1: Math.min(...fpsW), worst2: worst2(fpsW), rowsMean: rowsT / tSum, rowsMin, rowsMax, presetShare: share(presetT), scaleShare: share(scaleT), timeline, overhead: overMed, hz, win: w,
    recorded: { meanFps: 1000 * d.frames.filter(f => f.dt < 500).length / tSum, worst1: Math.min(...recW), worst2: worst2(recW), held: (fps: number) => W.reduce((a, x, i) => a + (recW[i] >= fps ? x.t : 0), 0) / totT } };
}

// ---- --check: as-now must make the recording's own moves ------------------------------------------------------------------------------------------------------------------------------------
const moveKey = (text: string) => text.replace(/\s*\(.*$/, '').replace(/\s+—.*$/, '').trim();   // 'climb ↑ scale 0.80 (gpu ∪ 8.1 ms of 16.7)' → 'climb ↑ scale 0.80'
const isStep = (text: string) => /^(climb|adaptive|auto|paced|shed|hold \d+) [↑↓]/.test(text);   // the ladder's steps (not its 'frame pacing' notes, a calibration's line, a hand's pin)
const laddersOf = (d: PerfDump) => d.events.filter(e => e.kind === 'ladder' && isStep(e.text)).map(e => ({ t: e.t / 1000, key: moveKey(e.text), text: e.text }));
const aimOfTake = (d: PerfDump) => Number((d.settingsAtStart as Record<string, unknown>).ladderAim) || 0;   // (a take recorded under an aim is checked under that aim)
/** the start state under which as-now replays closest to the recording's own moves */
function bestWarm(d: PerfDump, hz?: number) {
  const want = laddersOf(d);
  let best: { warm: Warm; r: Result; matched: number; err: number; extra: number } | null = null;
  for (let acc = 0; acc < 1; acc += 0.05) for (const roomy of [0, 1, 2, 3]) {
    const warm = { acc, roomy }; const r = replay(d, aimOfTake(d), { warm, hz }); const got = r.moves.filter(m => isStep(m.text));
    let matched = 0, err = 0; const used = new Set<number>();
    for (const wv of want) { const j = got.findIndex((g, i) => !used.has(i) && moveKey(g.text) === wv.key && Math.abs(g.t - wv.t) <= 2.5); if (j >= 0) { used.add(j); matched++; err += Math.abs(got[j].t - wv.t); } }
    const extra = got.length - used.size;
    if (!best || matched - extra > best.matched - best.extra || (matched - extra === best.matched - best.extra && err < best.err)) best = { warm, r, matched, err, extra };
  }
  return best!;
}
function checkAsNow(d: PerfDump, name: string, hz?: number): boolean {
  const want = laddersOf(d); const b = bestWarm(d, hz); const ok = b.matched === want.length && b.extra === 0;
  console.log(`\n${name}: ${aimName(aimOfTake(d))}, replayed against the recording's own ${want.length} ladder moves (start state tried over 20 window phases × 4 roomy counts; best: ${(b.warm.acc * 1000).toFixed(0)} ms into a window, roomy ${b.warm.roomy})`);
  const got = b.r.moves.filter(m => isStep(m.text));
  for (const wv of want) { const g = got.find(x => moveKey(x.text) === wv.key && Math.abs(x.t - wv.t) <= 2.5); console.log(`  ${g ? 'PASS' : 'FAIL'} — recorded ${wv.t.toFixed(1).padStart(6)} s  ${wv.text}${g ? `   ← replay ${g.t.toFixed(1)} s  ${g.text}` : '   ← no such move in the replay'}`); }
  for (const g of got) if (!want.some(wv => moveKey(g.text) === wv.key && Math.abs(g.t - wv.t) <= 2.5)) console.log(`  FAIL — the replay moved where the recording did not: ${g.t.toFixed(1)} s  ${g.text}`);
  console.log(`  ${ok ? 'PASS' : 'FAIL'} — ${b.matched} of ${want.length} recorded moves reproduced, ${b.extra} extra; mean |Δt| ${(b.matched ? b.err / b.matched : 0).toFixed(2)} s; replayed mean ${b.r.meanFps.toFixed(1)} fps vs recorded ${b.r.recorded.meanFps.toFixed(1)}`);
  return ok;
}

// ---- reporting -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
const aimName = (a: number) => a === 0 ? 'as now' : a >= AIM_DISPLAY ? 'hold display' : `hold ${a}`;
const pct = (x: number) => `${(100 * x).toFixed(0)} %`;
function report(name: string, d: PerfDump, rs: Result[], timeline: boolean) {
  const r0 = rs[0]; const w = r0.win;
  const head = `${name}: ${r0.secs.toFixed(0)} s, ${d.frames[0].preset}, window ${w.cssW}×${w.cssH} @${w.dpr} (${w.cssH * w.dpr} device rows), display taken as ${r0.hz} Hz, frame overhead past the GPU union ${r0.overhead.toFixed(1)} ms`;
  const recLine = `recorded: mean ${r0.recorded.meanFps.toFixed(1)} fps, worst second ${r0.recorded.worst1.toFixed(0)}, worst two seconds ${r0.recorded.worst2.toFixed(0)}; at or over 58 fps ${pct(r0.recorded.held(HOLD.cut * 60))} of the time, 87 fps ${pct(r0.recorded.held(HOLD.cut * 90))}, 116 fps ${pct(r0.recorded.held(HOLD.cut * 120))}`;
  const rows = rs.map(r => { const top = Object.entries(r.scaleShare).sort((a, b) => b[1] - a[1]).slice(0, 3).map(([k, v]) => `${k} ${pct(v)}`).join(', ');
    return [aimName(r.aim), r.aim ? `${r.target} fps` : '(60 Hz budget)', r.meanFps.toFixed(1), r.aim ? pct(r.held) : `${pct(r.held)} (of 58)`, r.worst1.toFixed(0), r.worst2.toFixed(0), `${r.rowsMean.toFixed(0)} (${r.rowsMin.toFixed(0)}‥${r.rowsMax.toFixed(0)})`, Object.entries(r.presetShare).sort((a, b) => b[1] - a[1]).map(([k, v]) => `${k} ${pct(v)}`).join(', '), top, String(r.moves.length)]; });
  const cols = ['aim', 'works to', 'mean fps', 'time at the aim', 'worst 1 s', 'worst 2 s', 'rows mean (min‥max)', 'preset', 'most of the time at', 'moves'];
  console.log(`\n${head}\n  ${recLine}`); const wd = cols.map((c, i) => Math.max(c.length, ...rows.map(r => r[i].length))); console.log('  ' + cols.map((c, i) => c.padEnd(wd[i])).join('  ')); for (const r of rows) console.log('  ' + r.map((c, i) => c.padEnd(wd[i])).join('  '));
  if (timeline) for (const r of rs) {
    console.log(`\n  ${aimName(r.aim)} — every 15 s: mean fps (worst second) · mean union · rows`);
    for (const x of r.timeline) console.log(`    ${String(x.t0).padStart(4)} s  ${x.fps.toFixed(0).padStart(4)} fps (${x.worst.toFixed(0).padStart(3)})  ∪ ${x.union.toFixed(1).padStart(5)} ms  ${x.rows.toFixed(0).padStart(5)} rows`);
    console.log(`  ${aimName(r.aim)} — moves:`);
    for (const m of r.moves) console.log(`    ${m.t.toFixed(1).padStart(6)} s  ${m.state.preset} ×${m.state.scale.toFixed(2)} = ${m.rows.toFixed(0)} rows  ${m.text}`);
  }
}

// ---- --plan: the cost table, and plan() on machines the cost model prices ---------------------------------------------------------------------------------------------------------------------------
// Machines are `k` × the reference M4 Max per unit of work on a window. The absolute numbers are only as good as REF / FACTOR: the point is the decision logic and the ratios.
interface Machine { name: string; k: number; cssW: number; cssH: number; dpr: number }
/** effective scale (internal / CSS×dpr) the preset's cap gives machine m's window at render scale 1.0 (engine.ts capScale) */
const capOf = (name: QualityName, m: Machine) => Math.min(1, PRESETS[name].maxH / (m.cssH * m.dpr));
/** effective scale of preset `name` at render scale `scale` — the CURRENT semantics: relative to the cap */
const effOf = (name: QualityName, scale: number, m: Machine) => capOf(name, m) * Math.min(1, scale);
/** …and the semantics until 08-11: the scale multiplied the window and was then capped */
const mpxOf = (eff: number, m: Machine) => (m.cssW * m.dpr * eff) * (m.cssH * m.dpr * eff) / 1e6;
/** compute ms the model says preset `name` costs on machine m at effective scale `eff` (per-pixel part scaled from the reference pixel count + the fixed part, × k) */
const msAtEff = (name: QualityName, eff: number, m: Machine) => m.k * (COST[name].pixel * mpxOf(eff, m) / REF.mpx + COST[name].fixed);
const trueMs = (name: QualityName, scale: number, m: Machine) => msAtEff(name, effOf(name, scale, m), m);
const fixedMsOf = (name: QualityName, m: Machine) => m.k * COST[name].fixed;
const MACHINES: Machine[] = [
  { name: 'M4 Max, reference window', k: 1, cssW: 1777, cssH: 1000, dpr: 1 },
  { name: 'M4 Max, 1440×900 HiDPI panel', k: 1, cssW: 1440, cssH: 900, dpr: 2 },
  { name: 'M-series Pro class (2.5×), 1512×982 @2', k: 2.5, cssW: 1512, cssH: 982, dpr: 2 },
  { name: 'M1 / M2 base (4.5×), 1440×900 @2', k: 4.5, cssW: 1440, cssH: 900, dpr: 2 },
  { name: 'M1 if it is 5.5×, 1440×900 @2', k: 5.5, cssW: 1440, cssH: 900, dpr: 2 },
  { name: 'M1 base on a 1080p monitor (4.5×) @1', k: 4.5, cssW: 1920, cssH: 1080, dpr: 1 },
  { name: 'iGPU (8×), 1920×1080 @1', k: 8, cssW: 1920, cssH: 1080, dpr: 1 },
  { name: 'iGPU (12×), 1366×768 @1', k: 12, cssW: 1366, cssH: 768, dpr: 1 },
];
const f1 = (v: number) => v.toFixed(1).padStart(5), f2 = (v: number) => v.toFixed(2);
/** what the boot's calibration picks on machine m, measuring at 'high' at its own start */
function pickOf(m: Machine, budget = PLAN_BUDGET_MS) { const gpu = trueMs('high', PRESETS.high.scale, m), fixed = fixedMsOf('high', m); return { gpu, fixed, pick: plan(gpu, fixed, 'high', effOf('high', PRESETS.high.scale, m), m.cssH, m.dpr, budget) }; }
function printPlan(budget: number, own: Machine[]) {
  console.log(`Cost model (src/ui/quality.ts COST): modelled compute ms on the reference M4 Max at ${REF.mpx} Mpx internal, per preset — per-pixel passes | fixed part | ratios plan() uses`);
  console.log(`  preset    direct softbl gather tempor volume bloom | cascad probe |  pixel (rel)   fixed (rel)   what it switches`);
  for (const n of QUALITY_NAMES) {
    const c = COST[n], P = c.passes, p = PRESETS[n], L = p.lossy;
    const sw = [`${p.restir.rays} rays`, `maxH ${p.maxH}`, `scale ${p.scale}`, ...(['gatherThird', 'rcHalfRate', 'volQuarter'] as const).filter(k => L[k])].join(', ');
    console.log(`  ${n.padEnd(8)} ${f1(P.direct)}  ${f1(P.softblur)}  ${f1(P.gather)}  ${f1(P.temporal)}  ${f1(P.volumetrics)} ${f1(P.bloom)} | ${f1(P.rc)}  ${f1(P.probe)} | ${f1(c.pixel)} (${f2(c.pixel / COST.high.pixel)})  ${f1(c.fixed)} (${f2(c.fixed / COST.high.fixed)})   ${sw}`);
  }
  console.log(`\nCalibration on a first visit (measuring at 'high' at its own start), budget ${budget} ms of compute: floor ${SCALE_FLOOR} of the cap, ${SCALE_STEP} grid; model = what the cost model says the pick costs at the rows it renders`);
  for (const m of [...MACHINES, ...own]) { const { gpu, fixed, pick: r } = pickOf(m, budget), e = effOf(r.preset, r.scale, m); console.log(`  ${m.name.padEnd(44)} ${f1(gpu)} ms (fixed ${f1(fixed)}) ×${f2(effOf('high', PRESETS.high.scale, m))} → ${r.preset.padEnd(7)} ×${f2(r.scale)} (${Math.floor(m.cssH * m.dpr * e)} rows)  model ${f1(trueMs(r.preset, r.scale, m))} ms  ${r.note}`); }
}

// ---- the machine the boot's and as-now's checks run on -------------------------------------------------------------------------------------------------------------------------------------------
const W2: Win = { cssW: 1440, cssH: 900, dpr: 2 };
/** a stub engine and a real Quality on a machine `k`× the reference on 1440×900 @2. `aim` = settings.ladderAim, null = the page's default. E.tick refreshes the timer for this frame: what the cost model says the current preset costs at the effective scale, its per-pixel part + its fixed part, × k */
function machine(k: number, aim: number | null = null) {
  const E = mkEngine(W2, aim === null ? {} : { ladderAim: aim }), q: any = new Quality(E as never, { params: { rbIters: 4 } } as never);
  E.tick = () => { const name = q.current as QualityName, eff = E.effectiveScale, mpx = (W2.cssW * W2.dpr * eff) * (W2.cssH * W2.dpr * eff) / 1e6, R = E.timer.results; R.clear(); R.set('direct', k * COST[name].pixel * mpx / REF.mpx); R.set('rc c0', k * COST[name].fixed * 0.9); R.set('probe', k * COST[name].fixed * 0.1); R.set('composite', 16); };   // (a render pass, excluded from the total like the real ones)
  return { E, q };
}
/** calibration done, the tier on, nothing else: pinned high with adaptive resolution on */
const calibrated = (q: any) => { q.calib = { ...q.calib, done: true }; };
const pinnedHigh = (aim: number | null = null) => { store.clear(); const x = machine(1, aim); x.q.auto = false; x.q.apply('high'); x.q.adaptive = true; calibrated(x.q); return x; };
  const pump = (E: any, q: any, frames: number) => { let decided = 0; for (let i = 0; i < frames; i++) { E.tick(); const was = q.calibrating; if (q.calibrating) q.calibrateTick(1 / 60); if (was && !q.calibrating) decided++; } return decided; };
/** the reference machine after its first calibration, then `preset` put on, under auto: the as-now ladder */
const settledOn = (preset: QualityName) => { store.clear(); const x = machine(1, 0); pump(x.E, x.q, 100); x.q.apply(preset); calibrated(x.q); return x; };

// ---- the checks: synthetic machines ----------------------------------------------------------------------------------------------------------------------------------------------------------------
function selftest(): boolean {
  let fails = 0; const check = (ok: boolean, what: string) => { console.log(`  ${ok ? 'PASS' : 'FAIL'} — ${what}`); if (!ok) fails++; };
  const w: Win = { cssW: 1456, cssH: 873, dpr: 2 }; const S0 = { sharpDirect: true, taau: true, oneCacheGrid: true };   // (the synthetic machines run the page as it opens: one cache grid, as the ladder's own cost table prices it)
  /** `secs` on a machine whose union is `k`(t) × the model's cost of what the ladder is rendering, shown on a `hz` display (`vrr`: a frame takes what it takes above the display's interval;
   *  else whole display intervals), with `paceMs` = something else holding every frame to at least that; returns the 1-s fps windows and the moves */
  const run = (q: any, E: any, secs: number, hz: number, k: (t: number) => number, o: { vrr?: boolean; paceMs?: number | ((t: number) => number); timed?: boolean; t0?: number; hitch?: () => number } = {}) => {
    const fps: number[] = []; const moves: string[] = []; let last = q.lastChange; let t = 0, wt = 0, wn = 0; const iv = 1000 / hz;
    while (t < secs * 1000) {
      const union = k((o.t0 ?? 0) + t / 1000) * modelCost(stateOf(q, E), w, S0); let dt = Math.max(iv, union + MODEL.overheadMs, typeof o.paceMs === 'function' ? o.paceMs((o.t0 ?? 0) + t / 1000) : o.paceMs ?? 0); if (o.vrr === false) dt = Math.ceil(dt / iv - 1e-9) * iv; { const h = o.hitch?.() ?? 0; if (h > 0) dt = h; }
      q.sample(dt / 1000, o.timed === false ? 0 : union); t += dt; wt += dt; wn++;
      if (q.lastChange !== last) { last = q.lastChange; moves.push(`${((o.t0 ?? 0) + t / 1000).toFixed(1)} s ${last}`); }
      if (wt >= 1000) { fps.push(1000 * wn / wt); wt = 0; wn = 0; }
    }
    return { fps, moves };
  };
  const base = modelCost({ preset: 'high', scale: PRESETS.high.scale, gatherThird: true, volQuarter: false, maxH: PRESETS.high.maxH, sharp: true }, w, S0);   // model-ms of high at its start on this window
  const at = (ms: number) => ms / base;   // the machine factor that makes high-at-its-start cost `ms` of union
  const mk = (aim: number, hz: number, auto = false, preset: QualityName = 'high') => mkQuality(w, preset, auto, { ladderClimb: true }, aim, hz);
  const minOf = (a: number[]) => Math.min(...a);
  const sc = (x: { E: any }) => x.E.settings.renderScale as number; const words = (m: string[]) => m.map(x => x.split(' (')[0].replace(/^[\d.]+ s /, '')).join(' | ') || 'none';

  console.log('\nThe bands each aim puts the ladder on:');
  { const b0 = ladderBands(0), b60 = ladderBands(60, 120), b90 = ladderBands(90, 120), b120 = ladderBands(120, 120), bd = ladderBands(AIM_DISPLAY, 144);
    check(b0.cutFps === 45 && b0.aimFps === 50 && b0.restoreFps === 58 && b0.starvedFps === 40 && b0.cruisingFps === 57 && b0.backoffFps === 55 && b0.budget === 1000 / 60 && b0.planBudget === 15 && b0.gpuFloor === 11 && !b0.hold, `as now: cut < ${b0.cutFps}, aim ${b0.aimFps}, restore > ${b0.restoreFps}, cruising ${b0.cruisingFps}, back-off ${b0.backoffFps} fps, budget ${b0.budget.toFixed(2)} ms, plan ${b0.planBudget} ms`);
    check(Math.abs(b60.budget - b0.budget) < 1e-9 && Math.abs(b60.planBudget - PLAN_BUDGET_MS) < 1e-9 && b60.cruisingFps > b60.cutFps && b60.cruisingFps < 60, `hold 60 climbs on today's bands: budget ${b60.budget.toFixed(2)} ms (headroom ${(CLIMB.headroom * b60.budget).toFixed(2)}, back-off ${(CLIMB.backoff * b60.budget).toFixed(2)}), hop / plan ${b60.planBudget.toFixed(2)} ms, cruising ${b60.cruisingFps.toFixed(1)} fps; but cuts under ${b60.cutFps.toFixed(1)} fps (as now: ${b0.cutFps})`);
    check(bd.target === 144 && ladderBands(AIM_DISPLAY, 0).target === 60 && ladderBands(120, 0).target === 60, `hold display = the rate seen (144 on a 144 Hz display); until the first frames have said, every aim is 60`);
    check(ladderBands(60, 29).target === 60 && ladderBands(120, 29).target === 60 && ladderBands(AIM_DISPLAY, 40).target === 60 && ladderBands(120, 90).target === 90, `a rate seen under 60 never lowers an aim (a slow machine's own frame rate is not its display): hold 60 seen at 29 = ${ladderBands(60, 29).target}, hold 120 seen at 29 = ${ladderBands(120, 29).target}, seen at 90 = ${ladderBands(120, 90).target}`);
    check(displayRateOf(Array.from({ length: 60 }, (_, i) => [16, 17, 17][i % 3])) === 60 && displayRateOf(Array.from({ length: 60 }, (_, i) => i % 20 === 0 ? 25 : i % 20 === 1 ? 8.4 : 16.67)) === 60 && displayRateOf(Array.from({ length: 60 }, (_, i) => i > 20 && i < 32 ? 8.33 : 14)) === 120, `the display's rate off frame times: 16/17/17 ms timestamps read 60, one short catch-up frame after a long one does not read 120, a second of 8.33 ms frames does`);
    check(ladderAimOf('asnow') === 0 && ladderAimOf('0') === 0 && ladderAimOf('60') === 60 && ladderAimOf('90') === 90 && ladderAimOf('120') === 120 && ladderAimOf('display') === AIM_DISPLAY && ladderAimOf('75') === null && ladderAimOf('') === null, `?aim= words: asnow (or 0) 0 · 60 · 90 · 120 · display ${AIM_DISPLAY} · anything else refused`);
    // THE PAGE'S DEFAULT is hold 60 (the maintainer's pick of 10-10): what the climb, the cut and the hop work to on a stub with no word of its own
    { const { E, q } = machine(1); check(E.settings.ladderAim === 60 && q.aimState.hold && q.aimState.target === 60, `the default holds 60: settings.ladderAim ${E.settings.ladderAim}, the ladder's target ${q.aimState.target}`); }
    // what the boot's plan picks: the reference machine high at its own start, an M1 / M2 base at the floor, and a pick's scale on the grid or its tier's own start
    { const picks = MACHINES.map(m => pickOf(m).pick), r = (p: { preset: string; scale: number }) => `${p.preset} ×${p.scale.toFixed(2)}`, onGrid = (p: { preset: QualityName; scale: number }) => Math.abs(p.scale / SCALE_STEP - Math.round(p.scale / SCALE_STEP)) < 1e-6 || Math.abs(p.scale - PRESETS[p.preset].scale) < 1e-6;
      check(r(picks[0]) === 'high ×0.77' && r(picks[3]) === 'verylow ×0.50' && picks.every(onGrid), `plan() at ${PLAN_BUDGET_MS} ms: ${MACHINES.map((m, i) => `${m.name} ${r(picks[i])}`).join(' · ')}`); }
  }

  console.log('\nA GPU-bound machine on a 120 Hz variable-rate display (frame = union + 2.8 ms), pinned high with adaptive resolution:');
  { // (1) a fight that takes high from 11 ms to 17 ms of union: as now rides it out over 45 fps; hold 60 cuts and holds
    const load = (t: number) => at(t < 20 ? 11 : t < 80 ? 17 : 11);
    const a = mk(0, 120); const ra = run(a.q, a.E, 80, 120, load);
    const h = mk(60, 120); const rh = run(h.q, h.E, 80, 120, load);
    const fightA = ra.fps.slice(26, 78), fightH = rh.fps.slice(26, 78); const cutAt = rh.moves.find(m => /hold 60 ↓/.test(m));
    check(minOf(fightA) > 45 && minOf(fightA) < 56 && !ra.moves.some(m => /adaptive ↓/.test(m)), `as now: the fight runs at ${minOf(fightA).toFixed(0)}‥${Math.max(...fightA).toFixed(0)} fps and nothing below the tier's start is cut (over 45 fps is fine by it); moves: ${ra.moves.map(m => m.split(' (')[0]).join(' | ') || 'none'}`);
    check(!!cutAt && parseFloat(cutAt) < 24.5 && minOf(fightH) >= HOLD.cut * 60 - 0.5, `hold 60: cut within ${cutAt ? (parseFloat(cutAt) - 20).toFixed(1) : '?'} s of the fight starting, then ${minOf(fightH).toFixed(0)}‥${Math.max(...fightH).toFixed(0)} fps to its end at ×${h.E.settings.renderScale.toFixed(2)} — ${rh.moves.map(m => m.split(' (')[0]).join(' | ')}`);
    const rh2 = run(h.q, h.E, 120, 120, load, { t0: 80 });
    check(h.E.settings.renderScale >= PRESETS.high.scale - 1e-6 && minOf(rh2.fps.slice(4)) >= HOLD.cut * 60 - 0.5, `the fight over: the pixels come back on measured headroom — ×${h.E.settings.renderScale.toFixed(2)} after two minutes, never under ${minOf(rh2.fps.slice(4)).toFixed(0)} fps on the way (${rh2.moves.length} moves: ${rh2.moves.map(m => m.split(' (')[0]).join(' | ')})`);
    // NOTHING ADAPTS (`?adapt=0`, ui/quality.ts visit: what two benchmark runs need to be held against each other), through a fight that takes high to 25 ms of
    // union (36 fps), under hold 60 and as now. With a tier named (`?quality=high&adapt=0`) nothing moves the picture's size or its tier. With NONE named the
    // word holds the tier the page stands on, at that tier's own start size: auto would otherwise shed sharp direct, give the tier up and open at the size the
    // last visit ended on. Then THE PAGE'S OWN ORDER: a ladder made from what the browser remembers, the address read, and Options' Reset (the defaults, then
    // the address again). ADAPT_BREAK=1: the word is left off every address and every line that plays it must FAIL
    { const off = process.env.ADAPT_BREAK === '1', word = off ? '?quality=high' : '?quality=high&adapt=0', bare = off ? '?bench' : '?bench&adapt=0';
      check(visitAdaptive('') && visitAdaptive('?quality=high') && visitAdaptive('?adapt=1') && visitAdaptive('?adapt') && !visitAdaptive('?adapt=0') && !visitAdaptive('?bench&quality=high&adapt=off') && !visitAdaptive('?adapt=false'), '?adapt= words: 0 (or off, false) is the ladder off; nothing, or anything else, leaves it on');
      const fight = (t: number) => at(t < 10 ? 11 : 25), low = (fps: number[]) => minOf(fps.slice(16, 78));
      for (const aim of [60, 0]) { const z = mk(aim, 120); z.q.visit(word); const s0 = sc(z); const rz = run(z.q, z.E, 80, 120, fight);
        check(sc(z) === s0 && z.q.current === 'high' && !rz.moves.some(m => /↓|↑/.test(m)) && low(rz.fps) < 40, `${word}, ${aim ? `hold ${aim}` : 'as now'}: through a fight at ${low(rz.fps).toFixed(0)} fps the picture stays at ×${sc(z).toFixed(2)} of high (${words(rz.moves.filter(m => /↓|↑/.test(m)))})`); }
      for (const aim of [60, 0]) {   // under auto, with sharp direct auto's to shed and a size the last visit left behind
        const z = mkQuality(w, 'high', true, { ladderClimb: true, ladderShed: true, sharpDirect: true, renderScale: 0.6 }, aim, 120), sharp0 = !!z.E.settings.sharpDirect; z.q.visit(bare);
        const s0 = sc(z), rz = run(z.q, z.E, 80, 120, fight);
        check(sharp0 && !z.q.auto && !z.q.adaptive && !z.q.climbAllowed && s0 === PRESETS.high.scale && sc(z) === s0 && z.q.current === 'high' && !!z.E.settings.sharpDirect === sharp0 && !z.q.shed && rz.moves.length === 0 && low(rz.fps) < 40,
          `${bare} under auto, ${aim ? `hold ${aim}` : 'as now'}: the tier auto stood on is held at its own start ×${s0.toFixed(2)} (not the ×0.60 it was left at), and through a fight at ${low(rz.fps).toFixed(0)} fps nothing moves, sharp direct included (${words(rz.moves)})`); }
      { const z = mkQuality(w, 'medium', true, { ladderClimb: true }, 60, 120); z.q.visit('?quality=low&adapt=0'); const named = z.q.current; z.q.adaptive = true; z.q.auto = true; z.q.climbAllowed = true; z.q.visit('?adapt=1');
        check(named === 'low' && z.q.auto && z.q.adaptive && z.q.climbAllowed && z.q.current === 'low', 'a tier named beside the word is the one held, and an address without either word leaves auto, the row and the climb as they stand'); }
      // the page's own order
      const MODE = 'twopointsix.quality', PICK = 'twopointsix.autoPreset', SIZE = 'twopointsix.autoScaleRel';
      const boot = (mem: Record<string, string>, search: string) => { store.clear(); for (const k of Object.keys(mem)) store.set(k, mem[k]); const E = mkEngine(w, {}), q: any = new Quality(E as never, { params: { rbIters: 4 } } as never);
        const made = JSON.stringify([...store]); q.visit(search); return { E, q, wrote: JSON.stringify([...store]) !== made }; };
      const is = (z: { E: any; q: any }, tier: QualityName) => !z.q.auto && !z.q.adaptive && !z.q.calibrating && !z.q.climbAllowed && z.q.current === tier && sc(z) === PRESETS[tier].scale;
      { const z = boot({ [PICK]: 'medium', [SIZE]: '0.6' }, bare), opened = is(z, 'medium') && !z.wrote; z.q.calibrateTick(1 / 60); const rz = run(z.q, z.E, 30, 120, () => at(40));
        check(opened && is(z, 'medium') && rz.moves.length === 0, `a browser whose auto last stood on medium at ×0.60, opened with ${bare}: medium is held at its own start ×${sc(z).toFixed(2)}, no calibration runs, the visit writes nothing as the machine's choice, and half a minute at ${minOf(rz.fps).toFixed(0)} fps moves nothing`);
        z.q.pin('verylow'); z.q.setAuto(); z.q.visit(bare);
        check(is(z, 'medium') && store.get(MODE) === 'auto', `a tier picked by hand and then Reset (the defaults, then the address again): medium held again, and the hand's pin is no longer the remembered choice (${z.q.current} ×${sc(z).toFixed(2)}, remembered ${store.get(MODE)})`); }
      { const fresh = boot({}, bare), named = boot({ [PICK]: 'medium', [SIZE]: '0.6' }, off ? '?quality=low' : '?quality=low&adapt=0'), pinned = boot({ [MODE]: 'low' }, bare), plain = boot({ [PICK]: 'medium', [SIZE]: '0.6' }, '?bench');
        check(is(fresh, 'high') && !fresh.wrote && is(named, 'low') && !named.wrote && is(pinned, 'low') && !pinned.wrote,
          `a browser that never ran auto is held on high (the tier nobody measured, which the benchmark's card says); a tier named on the address is the one held whatever is remembered; a tier pinned from the menu is held as it is`);
        check(plain.q.auto && plain.q.adaptive && plain.q.calibrating && plain.q.climbAllowed && plain.q.current === 'medium' && Math.abs(sc(plain) - 0.6) < 1e-9 && !plain.wrote, 'and an address without the word opens as it always did: auto, calibrating, at the size it remembered'); }
      store.clear(); }
    // (2) steady load: no oscillation
    const s = mk(60, 120); const rs = run(s.q, s.E, 300, 120, () => at(15.5)); const late = rs.moves.filter(m => parseFloat(m) > 30);
    check(minOf(rs.fps.slice(8)) >= HOLD.cut * 60 - 0.5 && late.length <= 6, `steady load (15.5 ms at the tier's start) for five minutes: settles at ×${s.E.settings.renderScale.toFixed(2)}, ${minOf(rs.fps.slice(8)).toFixed(0)}‥${Math.max(...rs.fps.slice(8)).toFixed(0)} fps, ${late.length} moves after the first 30 s (${late.map(m => m.split(' (')[0]).join(' | ') || 'none'})`);
  }

  console.log('\nHold 90 and hold 120 under auto on the same machine (the preset may move):');
  for (const aim of [90, 120]) {
    const h = mk(aim, 120, true); const r = run(h.q, h.E, 120, 120, () => at(11)); const tail = r.fps.slice(40);
    check(minOf(tail) >= HOLD.cut * aim - 1 || (h.q.current === 'verylow' && h.E.settings.renderScale <= SCALE_FLOOR + 0.01), `hold ${aim}, 11 ms at high's start: after 40 s ${h.q.current} ×${h.E.settings.renderScale.toFixed(2)} (${rowsOf(stateOf(h.q, h.E), w).toFixed(0)} rows) at ${minOf(tail).toFixed(0)}‥${Math.max(...tail).toFixed(0)} fps — ${r.moves.map(m => m.split(' (')[0]).join(' | ')}`);
    const late = r.moves.filter(m => parseFloat(m) > 60); check(late.length <= 4, `… and it stays put: ${late.length} moves in the second minute (${late.map(m => m.split(' (')[0]).join(' | ') || 'none'})`);
  }

  console.log('\nWhat an aim must NOT do:');
  { const h = mk(120, 60); const r = run(h.q, h.E, 60, 60, () => at(9), { vrr: false });
    check(h.E.settings.renderScale >= PRESETS.high.scale - 1e-6 && !r.moves.some(m => /↓/.test(m)), `hold 120 on a 60 Hz display with 9 ms of GPU work: no cut (×${h.E.settings.renderScale.toFixed(2)}; it holds 60 — ${r.moves.map(m => m.split(' (')[0]).join(' | ') || 'no moves'})`);
    const p = mk(60, 60); const rp = run(p.q, p.E, 30, 60, () => at(5), { paceMs: 33.3 });
    check(p.E.settings.renderScale >= PRESETS.high.scale - 1e-6 && rp.moves.some(m => /frame pacing/.test(m)), `hold 60, a page paced to 30 fps with the GPU 5 ms busy: resolution left alone (×${p.E.settings.renderScale.toFixed(2)}) — “${rp.moves.find(m => /pacing/.test(m))?.replace(/^\S+ s /, '')}”`);
    const b = mk(60, 120); const rb = run(b.q, b.E, 60, 120, () => at(17), { timed: false });
    check(b.E.settings.renderScale < PRESETS.high.scale - 1e-6 && minOf(rb.fps.slice(17)) >= HOLD.cut * 60 - 0.5 && rb.moves.length <= 3, `hold 60 with no GPU timings: cuts on the rate alone, tries one step back blind, takes it off again and holds it off (×${b.E.settings.renderScale.toFixed(2)}, ${minOf(rb.fps.slice(17)).toFixed(0)} fps from 17 s on — ${rb.moves.map(m => m.split(' (')[0]).join(' | ')})`);
    // the display seen at 120 earlier, the window now on a 60 Hz one (nothing pinned: the page has to find out)
    const mv = mk(120, 0, true); mv.q.displayHz = 120; const rmv = run(mv.q, mv.E, 150, 60, () => at(9), { vrr: false }); const told = rmv.moves.find(m => /shows no more/.test(m));
    check(!!told && parseFloat(told) < 40 && mv.q.displayHz === 60 && mv.q.current === 'high' && mv.E.settings.renderScale >= PRESETS.high.scale - 1e-6, `hold 120, seen at 120 Hz then moved to a 60 Hz display (9 ms of GPU work): cut as 120 would need, then at ${told ? parseFloat(told).toFixed(0) : '?'} s “${told?.replace(/^\S+ s /, '').split(' — ')[1] ?? ''}”, back at ${mv.q.current} ×${mv.E.settings.renderScale.toFixed(2)} by the end (lowest: ${rmv.moves.filter(m => /↓/.test(m)).map(m => (/(scale \d\.\d+|auto ↓ \w+)/.exec(m) ?? [''])[0]).join(', ')})`);
    // the CPU holds every frame to 19 ms with the GPU inside its budget; one long hitch a second
    const cp = mk(60, 120); const rcp = run(cp.q, cp.E, 30, 120, () => at(10.5), { paceMs: 19 });
    check(cp.E.settings.renderScale >= PRESETS.high.scale - 1e-6, `hold 60, the CPU pinning frames at 19 ms (53 fps) with 10.5 ms of GPU work: no pixels given up for nothing (×${cp.E.settings.renderScale.toFixed(2)}; ${rcp.moves.length} notes)`);
    const hi = mk(60, 60); let n = 0; const rhi = run(hi.q, hi.E, 30, 60, () => at(10.5), { vrr: false, hitch: () => (++n % 60 === 0 ? 60 : 0) });
    check(hi.E.settings.renderScale >= PRESETS.high.scale - 1e-6 && !rhi.moves.some(m => /↓/.test(m)), `hold 60 on a 60 Hz display, one 60 ms hitch a second: not a frame rate — nothing cut (×${hi.E.settings.renderScale.toFixed(2)})`);
    const pin = mk(120, 120); const rpin = run(pin.q, pin.E, 60, 120, () => at(14));
    check(pin.q.current === 'high' && Math.abs(pin.E.settings.renderScale - SCALE_FLOOR) < 1e-6, `hold 120 on a preset pinned from the menu that cannot make it: the scale stops at the floor ×${pin.E.settings.renderScale.toFixed(2)}, the preset is the player's (${pin.q.current}; ${rpin.fps[rpin.fps.length - 1].toFixed(0)} fps)`);
  }
  console.log('\nAs now, on a page something else paces (settings.ladderPaced; the maintainer\'s Mac 10-10: 30 fps shown, ×0.57 then ×0.50, GPU union 8.9 ms):');
  { const mkP = (on: boolean, auto = true, scale?: number) => mkQuality(w, 'high', auto, scale === undefined ? { ladderClimb: true, ladderPaced: on } : { ladderClimb: true, ladderPaced: on, renderScale: scale }, 0, 120);
    const words = (m: string[]) => m.map(x => x.split(' (')[0]).join(' | ') || 'none';
    // (1) THE FAULT, with the check off: 30 fps by pacing, 12.6 ms of union at high's start → ×0.57 (= 0.77 − 0.20: the one scale the as-now cut reaches off the 0.05 grid), kept; a heavier spell → the floor and a preset down, kept
    const o = mkP(false); const ro = run(o.q, o.E, 40, 120, () => at(12.6), { paceMs: 33.33 }); const s1 = sc(o);
    const ro2 = run(o.q, o.E, 80, 120, t => at(t < 50 ? 16.5 : 12.6), { paceMs: 33.33, t0: 40 });
    check(Math.abs(s1 - 0.57) < 1e-6 && minOf(ro.fps) > 29 && Math.max(...ro.fps.slice(4)) < 31 && sc(o) <= 0.57 + 1e-6 && Math.max(...ro2.fps) < 31, `the check OFF (the rule as it was): cut to ×${s1.toFixed(2)} for nothing (30 fps before and after) and kept; ten heavier seconds take the rest and the preset (${o.q.current} ×${sc(o).toFixed(2)}, still 30 fps), never given back in two minutes — ${words([...ro.moves, ...ro2.moves].filter(m => /↓|↑/.test(m)))}`);
    // (2) the same page with the check on: the one cut is judged and taken back; the heavier spell cuts nothing
    const n = mkP(true); const rn = run(n.q, n.E, 40, 120, () => at(12.6), { paceMs: 33.33 }); const back = rn.moves.find(m => /paced ↑/.test(m)); const sn1 = sc(n);
    const rn2 = run(n.q, n.E, 80, 120, t => at(t < 50 ? 16.5 : 12.6), { paceMs: 33.33, t0: 40 });
    check(!!back && parseFloat(back) < 12 && Math.abs(sn1 - PRESETS.high.scale) < 1e-6 && Math.abs(sc(n) - PRESETS.high.scale) < 1e-6 && n.q.current === 'high' && !rn2.moves.some(m => /↓/.test(m)), `the check ON: the cut is taken back at ${back ? parseFloat(back).toFixed(0) : '?'} s (“${back?.replace(/^\S+ s /, '').split(' (')[1]?.replace(/\)$/, '') ?? ''}”), ×${sc(n).toFixed(2)} from there through the heavier spell — ${words(rn.moves.filter(m => /↓|↑/.test(m)))}`);
    // (3) a scale already at the floor when the pacing is met (a fight before it): given back as far as the page stays PROVEN paced (12.6 ms at high's start: ×0.55 — 10 ms of union in a 33 ms frame), no further
    const f = mkP(true, true, SCALE_FLOOR); const rf = run(f.q, f.E, 40, 120, () => at(12.6), { paceMs: 33.33 }); const up = rf.moves.find(m => /paced ↑/.test(m));
    check(sc(f) > SCALE_FLOOR + 1e-6 && sc(f) < PRESETS.high.scale - 1e-6 && !!up && parseFloat(up) < 12 && !rf.moves.some(m => /↓/.test(m)), `starting at the floor on that page: ×${sc(f).toFixed(2)} by ${up ? parseFloat(up).toFixed(0) : '?'} s — what stays provably free, and held there (${words(rf.moves.filter(m => /↓|↑/.test(m)))})`);
    const fl = mkP(true, true, SCALE_FLOOR); run(fl.q, fl.E, 40, 120, () => at(8), { paceMs: 33.33 });
    check(Math.abs(sc(fl) - PRESETS.high.scale) < 1e-6, `… and with a lighter scene (8 ms at high's start) all the way: ×${sc(fl).toFixed(2)}`);
    const f0 = mkP(false, true, SCALE_FLOOR); run(f0.q, f0.E, 40, 120, () => at(12.6), { paceMs: 33.33 });
    check(Math.abs(sc(f0) - SCALE_FLOOR) < 1e-6, `… where the check off leaves it at ×${sc(f0).toFixed(2)} for good (the way up wants 58 fps)`);
    // (4) what it must NOT change: real load — the same moves with the check on and off
    const same = (name: string, secs: number, hz: number, k: (t: number) => number, opts: Parameters<typeof run>[5], auto = true) => {
      const a = mkP(true, auto), b = mkP(false, auto); const ra = run(a.q, a.E, secs, hz, k, opts), rb = run(b.q, b.E, secs, hz, k, opts);
      const eq = JSON.stringify(ra.moves) === JSON.stringify(rb.moves) && sc(a) === sc(b) && a.q.current === b.q.current;
      check(eq && ra.moves.some(m => /↓/.test(m)), `${name}: cut all the same, move for move with the check off (${ra.moves.filter(m => /↓|↑/.test(m)).length} moves, ends ${a.q.current} ×${sc(a).toFixed(2)})`);
    };
    same('a fixed 60 Hz display missing its vsync (14 ms of union shows as 30 fps)', 60, 60, () => at(14), { vrr: false });
    same('a GPU-bound fight on a 120 Hz display (24 ms of union, 37 fps)', 60, 120, t => at(t < 10 ? 11 : 24), {});
    same('a machine too slow for high (30 ms of union): down the presets', 90, 120, () => at(30), {});
    same('paced at 30 with 28 ms of union at high\'s start (the frame is nearly all GPU work: nothing to take back)', 120, 120, () => at(28), { paceMs: 33.33 });
    { const mk2 = (on: boolean) => { const x = mkP(on); const r = run(x.q, x.E, 300, 60, () => at(16), { vrr: false, paceMs: (t: number) => t >= 120 && t < 180 ? 33.33 : 0 }); return { x, r }; }; const a = mk2(true), b = mk2(false);
      const s60 = (r: { fps: number[] }, from: number, to: number) => r.fps.slice(from, to).filter(v => v > 57).length / Math.max(1, Math.min(to, r.fps.length) - from);
      check(a.x.q.current === 'high' && s60(a.r, 185, 300) >= s60(a.r, 30, 120) - 0.1 && b.x.q.current !== 'high', `a fixed 60 Hz display that settled under high's start (16 ms there), a minute paced to 30, then free: the page is back where it was before the spell (${a.x.q.current}, at 60 fps ${(100 * s60(a.r, 185, 300)).toFixed(0)} % of the time after, ${(100 * s60(a.r, 30, 120)).toFixed(0)} % before), no scale raised it cannot carry — the check off loses the preset to the spell for good (${b.x.q.current})`); }
    // (5) paced at 30, then load that really is slower than the pace: cut
    const l = mkP(true); const rl = run(l.q, l.E, 60, 120, t => at(t < 20 ? 12.6 : 40), { paceMs: 33.33 }); const cut = rl.moves.find(m => parseFloat(m) > 20 && /adaptive ↓|auto ↓/.test(m));
    check(!!cut && parseFloat(cut) < 26, `paced at 30, then 40 ms of union arrives at 20 s: that is load — cut at ${cut ? parseFloat(cut).toFixed(0) : '?'} s (ends ${l.q.current} ×${sc(l).toFixed(2)}, ${rl.fps[rl.fps.length - 1].toFixed(0)} fps)`);
    // (6) the pacing lifts: the ladder is the ladder again (climbs on the room it sees)
    const e = mkP(true); const re = run(e.q, e.E, 90, 120, () => at(9), { paceMs: t => t < 30 ? 33.33 : 0 });
    check(sc(e) > PRESETS.high.scale + 1e-6 && !re.moves.some(m => /↓/.test(m)), `paced for 30 s with 9 ms of union, then free: nothing cut, and the climb takes its steps once the frames come (×${sc(e).toFixed(2)})`);
    // (8) the review's cases
    let hn = 0; const hi2 = mkP(true); const rh = run(hi2.q, hi2.E, 120, 120, () => at(12.6), { paceMs: 33.33, hitch: () => (++hn % 30 === 0 ? 60 : 0) }); const stepsH = rh.moves.filter(m => /↓|↑/.test(m));
    check(Math.abs(sc(hi2) - PRESETS.high.scale) < 1e-6 && stepsH.length <= 2, `that page with a 90 ms hitch every second: the one judged cut and no more in two minutes (${stepsH.length} steps, ×${sc(hi2).toFixed(2)})`);
    const hv = mkP(true); const rv = run(hv.q, hv.E, 60, 120, () => at(14.5), { paceMs: 33.33 });
    check(Math.abs(sc(hv) - PRESETS.high.scale) < 1e-6 && hv.q.current === 'high', `a heavier scene on it (14.5 ms at high's start, 10.6 after the cut — under the old floor, short of proof): taken back all the same (×${sc(hv).toFixed(2)}; ${words(rv.moves.filter(m => /↓|↑/.test(m)))})`);
    const hp = mkP(true); const rp = run(hp.q, hp.E, 90, 120, () => at(17), { paceMs: 33.33 });
    check(Math.abs(sc(hp) - PRESETS.high.scale) < 1e-6 && hp.q.current === 'high' && rp.moves.some(m => /auto ↓/.test(m)), `heavier still under auto (17 ms: the cuts run into a preset down): the preset comes back with the scale (${hp.q.current} ×${sc(hp).toFixed(2)}; ${words(rp.moves.filter(m => /↓|↑/.test(m)))})`);
    const ao = mkP(true); ao.q.adaptive = false; const rao = run(ao.q, ao.E, 60, 120, () => at(12.6), { paceMs: 33.33 });
    check(ao.q.current === 'high', `adaptive resolution off, auto on: the preset is not starved away for good either (${ao.q.current}; ${words(rao.moves.filter(m => /↓|↑/.test(m)))})`);
    const fx = mkP(true, false), fx0 = mkP(false, false); const kfx = () => at(15), ofx = { vrr: false, paceMs: (t: number) => t < 30 ? 33.33 : 0 };
    const rfx = run(fx.q, fx.E, 400, 60, kfx, ofx), rfx0 = run(fx0.q, fx0.E, 400, 60, kfx, ofx); const t60 = rfx.fps.findIndex((x, i) => i > 30 && x > 57), t600 = rfx0.fps.findIndex((x, i) => i > 30 && x > 57);
    const share = (r: { fps: number[] }, from: number) => r.fps.slice(from).filter(x => x > 57).length / Math.max(1, r.fps.length - from);
    check(t60 > 0 && t60 < 30 + PACED.recheckSecs + 15 && share(rfx, 150) >= share(rfx0, 150) - 0.1, `a fixed 60 Hz display, paced to 30 for half a minute, then free with 15 ms of union missing its vsync at that same 30: the remembered pace is put to the test again — 60 fps from ${t60} s (the check off: from ${t600} s)`);
    // (7) an aim is untouched by the switch
    const h1 = mkQuality(w, 'high', true, { ladderClimb: true, ladderPaced: true }, 60, 120), h0 = mkQuality(w, 'high', true, { ladderClimb: true, ladderPaced: false }, 60, 120);
    const r1 = run(h1.q, h1.E, 60, 120, t => at(t < 20 ? 11 : 17), { paceMs: t => t > 40 ? 33.33 : 0 }), r0 = run(h0.q, h0.E, 60, 120, t => at(t < 20 ? 11 : 17), { paceMs: t => t > 40 ? 33.33 : 0 });
    check(JSON.stringify(r1.moves) === JSON.stringify(r0.moves), `hold 60 makes the same ${r1.moves.length} moves with the switch on and off (it has its own paced rule)`);
  }
  console.log('\nHold 60 as the default (settings.ladderAim 60 since 10-10): a slow machine, and a page something else paces:');
  {
    // (1) a machine too slow for High whose page has never shown more than 29 fps (the watched 'display rate' = its own load): the aim stays 60 and the ladder cuts, then drops the preset under auto
    const s = mk(60, 120, true); (s.q as any).displayHzPin = 0; (s.q as any).displayHz = 29; const rs = run(s.q, s.E, 40, 120, () => at(31));
    const a = mk(0, 120, true); const ra = run(a.q, a.E, 40, 120, () => at(31));
    check(s.q.aimState.target === 60 && rs.moves.some(m => /↓/.test(m)) && rs.fps[rs.fps.length - 1] >= ra.fps[ra.fps.length - 1] - 1 && rs.fps[rs.fps.length - 1] > rs.fps[0] + 5, `a page never seen over 29 fps, 31 ms of union at high's start: hold 60 still aims at ${s.q.aimState.target} and cuts — ${rs.fps[0].toFixed(0)} → ${rs.fps[rs.fps.length - 1].toFixed(0)} fps at ${s.q.current} ×${sc(s).toFixed(2)} (as now: ${ra.fps[ra.fps.length - 1].toFixed(0)} fps at ${a.q.current} ×${sc(a).toFixed(2)})`);
    // (2) a page paced to 30 fps (the browser's energy saver), 11 ms of union at high's start: nothing is cut for the pacing; a heavy spell (17 ms) is cut as load; and once it is over the pixels come back while the page is STILL at 30
    const p = mkQuality(w, 'high', false, { ladderClimb: true, ladderHeld: false }, 60, 120); const r1 = run(p.q, p.E, 20, 120, () => at(11), { paceMs: 33.33 }); const s1 = sc(p);   // (ladderHeld off: an aim's own rule, as it was)
    const r2 = run(p.q, p.E, 20, 120, () => at(17), { paceMs: 33.33, t0: 20 }); const s2 = sc(p);
    const r3 = run(p.q, p.E, 200, 120, () => at(11), { paceMs: 33.33, t0: 40 }); const s3 = sc(p);
    check(Math.abs(s1 - PRESETS.high.scale) < 1e-6 && !r1.moves.some(m => /↓/.test(m)) && s2 < s1 - 0.04 && Math.abs(s3 - PRESETS.high.scale) < 1e-6 && Math.max(...r3.fps) < 31 && p.q.current === 'high', `hold 60 on a page paced to 30 fps: left at ×${s1.toFixed(2)} (11 ms of union); 20 s at 17 ms cut it to ×${s2.toFixed(2)}; back to ×${s3.toFixed(2)} afterwards with the page still at ${r3.fps[r3.fps.length - 1].toFixed(0)} fps — ${words([...r2.moves, ...r3.moves].filter(m => /↓|↑/.test(m)))}`);
  }
  console.log('\nHold 60 on a page something else holds (settings.ladderHeld; the maintainer\'s Mac on a low battery, 10-14: 30 fps shown at 600 rows with the GPU clocked down — 11.6 ms of union there, 13.9 at high\'s start):');
  {
    const ON = process.env.LADDERHELD_BREAK !== '1';   // LADDERHELD_BREAK=1: these cases run with the switch off and must FAIL
    const mkH = (on: boolean, o: { scale?: number; auto?: boolean; hz?: number } = {}) => mkQuality(w, 'high', !!o.auto, o.scale === undefined ? { ladderClimb: true, ladderHeld: on } : { ladderClimb: true, ladderHeld: on, renderScale: o.scale }, 60, o.hz ?? 120);
    /** `secs` a second at a time: the share of them that END at the tier's start or above, the moves, the fps windows, the share of windows at 58 fps or more */
    const watch = (x: { q: any; E: any }, secs: number, hz: number, k: (t: number) => number, o: { paceMs?: number | ((t: number) => number); vrr?: boolean; t0?: number } = {}) => { let at77 = 0, scSum = 0; const moves: string[] = [], fps: number[] = []; for (let t = 0; t < secs; t++) { const r = run(x.q, x.E, 1, hz, k, { ...o, t0: (o.t0 ?? 0) + t }); moves.push(...r.moves); fps.push(...r.fps); scSum += sc(x); if (sc(x) >= PRESETS.high.scale - 1e-6) at77++; } return { share: at77 / secs, scale: scSum / secs, moves, fps, at60: fps.filter(f => f >= 58).length / Math.max(1, fps.length), mean: fps.reduce((a, b) => a + b, 0) / Math.max(1, fps.length) }; };
    /** the same with the frame's own time past the union at `ms` (the machines above all run MODEL.overheadMs = 2.8; the proof's floor allows HOLD.pacedMs = 6) */
    const withOver = <T,>(ms: number, f: () => T): T => { const o0 = MODEL.overheadMs; MODEL.overheadMs = ms; try { return f(); } finally { MODEL.overheadMs = o0; } };
    // (1) held to 30 from the first frame, the clocked-down GPU reading 13.9 ms at high's start (over the 13.3 back-off band, inside the 15 ms budget): as it was, stepped down for the band and never back; now the band's steps and then a cut to where the union fits the proof's floor are judged together, taken back when the frame is no shorter there, and the pace holds
    const o1 = mkH(false), n1 = mkH(ON); const ro1 = watch(o1, 300, 120, () => at(13.9), { paceMs: 33.33 }), rn1 = watch(n1, 300, 120, () => at(13.9), { paceMs: 33.33 });
    check(sc(o1) < PRESETS.high.scale - 0.04 && ro1.share < 0.05 && !ro1.moves.some(m => /↑/.test(m)), `the switch off (an aim as it was): five minutes held at 30 fps end at ×${sc(o1).toFixed(2)}, ${(100 * ro1.share).toFixed(0)} % of them at the tier's start — ${words(ro1.moves.filter(m => /↓|↑/.test(m)))}`);
    check(Math.abs(sc(n1) - PRESETS.high.scale) < 1e-6 && rn1.share > 0.85 && rn1.moves.some(m => /paced ↑/.test(m)) && Math.max(...rn1.fps) < 31, `on: the same five minutes end at ×${sc(n1).toFixed(2)}, ${(100 * rn1.share).toFixed(0)} % of them at the tier's start (one cut to where the union fits, judged and taken back; the re-tests are the rest) — first moves: ${words(rn1.moves.filter(m => /↓|↑/.test(m)).slice(0, 4))}`);
    // (2) the hold lifts (the GPU back on its own clock: 13.9 → 9.3 ms): the pace is forgotten on the first fast window, nothing is cut, the frames come at the display's rate
    const rl = watch(n1, 30, 120, () => at(13.9 / 1.5), { t0: 300 });
    check(!rl.moves.some(m => /↓/.test(m)) && (n1.q as any).pacedAt === 0 && minOf(rl.fps.slice(2)) > 58, `the hold lifts: no cut, the pace forgotten, ${minOf(rl.fps.slice(2)).toFixed(0)} fps and up from there (×${sc(n1).toFixed(2)})`);
    // (3) a page that ARRIVES cut (×0.60, as the take of 10-14 stood) and held: as it was the band never reads low enough to give a step back; now one judged cut finds the pace and the pixels come back on the frame's own room, each step judged
    const o3 = mkH(false, { scale: 0.6 }), n3 = mkH(ON, { scale: 0.6 }); watch(o3, 300, 120, () => at(13.9), { paceMs: 33.33 }); const rn3 = watch(n3, 120, 120, () => at(13.9), { paceMs: 33.33 });
    check(sc(o3) < PRESETS.high.scale - 0.04 && Math.abs(sc(n3) - PRESETS.high.scale) < 1e-6 && Math.max(...rn3.fps) < 31, `arriving at ×0.60 on a held page: off, still ×${sc(o3).toFixed(2)} after five minutes; on, ×${sc(n3).toFixed(2)} inside two — ${words(rn3.moves.filter(m => /↓|↑/.test(m)))}`);
    // (4) a fight on a page whose pace is known (held at 30 for the 20 s before it; 23 ms at high's start on the clocked-down GPU = take 4's fight × 1.5): nothing is cut for it at all — as it was: the floor for as long as it lasts
    const o4 = mkH(false), n4 = mkH(ON); watch(o4, 20, 120, () => at(11), { paceMs: 33.33 }); const ro4 = watch(o4, 60, 120, () => at(23), { paceMs: 33.33, t0: 20 }); watch(n4, 20, 120, () => at(11), { paceMs: 33.33 }); const r4 = watch(n4, 60, 120, () => at(23), { paceMs: 33.33, t0: 20 });
    check(sc(o4) <= SCALE_FLOOR + 1e-6 && ro4.share < 0.1 && Math.abs(sc(n4) - PRESETS.high.scale) < 1e-6 && !r4.moves.some(m => /↓/.test(m)) && r4.share === 1 && Math.max(...r4.fps) < 31, `a minute's fight (23 ms) on a page held at 30 for the 20 s before it: off, cut to ×${sc(o4).toFixed(2)}; on, no cut (×${sc(n4).toFixed(2)} throughout; the frame stays 10 ms over its GPU work)`);
    // (5) NOT a held page: a fixed 60 Hz display whose 17 ms of union misses its vsync (33 ms frames, the same numbers from here) — the cut buys the frame rate back and is kept, move for move as with the switch off
    const o5 = mkH(false, { hz: 60 }), n5 = mkH(ON, { hz: 60 }); const ro5 = watch(o5, 60, 60, () => at(17), { vrr: false }), rn5 = watch(n5, 60, 60, () => at(17), { vrr: false });
    check(JSON.stringify(rn5.moves) === JSON.stringify(ro5.moves) && rn5.fps[rn5.fps.length - 1] > 59 && sc(n5) < PRESETS.high.scale - 0.04, `a fixed 60 Hz display missing its vsync at 17 ms of union: the cut is kept (×${sc(n5).toFixed(2)}, ${rn5.fps[rn5.fps.length - 1].toFixed(0)} fps), the same ${rn5.moves.length} moves as with the switch off`);
    // (6) load that really is slower than the pace arrives on the held page (40 ms of union: the frame is the GPU's work): cut, as as-now's check does
    const n6 = mkH(true); watch(n6, 30, 120, () => at(13.9), { paceMs: 33.33 }); const r6 = watch(n6, 20, 120, () => at(40), { paceMs: 33.33, t0: 30 }); const cut6 = r6.moves.find(m => /hold 60 ↓/.test(m));
    check(!!cut6 && parseFloat(cut6) < 36, `held at 30, then 40 ms of union arrives at 30 s: that is load — cut at ${cut6 ? parseFloat(cut6).toFixed(0) : '?'} s (×${sc(n6).toFixed(2)})`);
    // (8) the re-test is a test: the pace of case (1) is dropped at PACED.recheckSecs and the judged cut is taken again (not re-learnt in the same window from numbers that prove nothing)
    check(rn1.moves.filter(m => /paced ↑/.test(m)).length >= 2 && rn1.moves.filter(m => /held page or a missed vsync/.test(m)).length >= 2, `the re-test of a remembered pace is a judged cut again: ${rn1.moves.filter(m => /paced ↑/.test(m)).length} take-backs in case (1)'s five minutes`);
    // (9) a take-back gives the failed-rung marks back with the pixels: after the re-tests of a held page whose cuts go by rate, the tier's start is not held off
    { const x = mkH(ON); watch(x, 900, 120, () => at(15.4), { paceMs: 33.33 }); const key = `scale:${PRESETS.high.scale.toFixed(2)}`; const fc = (x.q as any).failCount.get(key) ?? 0;
      check(Math.abs(sc(x) - PRESETS.high.scale) < 1e-6 && fc === 0 && !(x.q as any).failed.has(key), `fifteen minutes held with 15.4 ms at high's start (every re-test a cut by rate, taken back): ×${sc(x).toFixed(2)}, the tier's start not marked as a failed rung (count ${fc})`); }
    // (10) NOT held, off the 2.8 ms the other machines run: fixed 60 Hz displays whose frame runs 4.5 ms past its union, variable-rate ones 7 and 8 ms past it — on never shows fewer seconds at 58 fps, nor a lower mean
    for (const [name, over, hz, vrr, load] of [['a fixed 60 Hz display, 4.5 ms past the union, 10 → 19 → 15 ms', 4.5, 60, false, (t: number) => at(t < 20 ? 10 : t < 80 ? 19 : 15)], ['a fixed 60 Hz display, 4.5 ms past the union, a steady 12.8 ms (its vsync missed by a hair)', 4.5, 60, false, () => at(12.8)], ['a variable-rate 120 Hz display, 8 ms past the union, 16 ms', 8, 120, true, () => at(16)], ['a variable-rate 120 Hz display, 7 ms past the union, 12 ms', 7, 120, true, () => at(12)]] as const) {
      const o = mkH(false, { hz }), n = mkH(ON, { hz }); const ro = withOver(over, () => watch(o, 400, hz, load, { vrr })), rn = withOver(over, () => watch(n, 400, hz, load, { vrr }));
      check(rn.at60 >= ro.at60 - 0.02 && rn.mean >= ro.mean - 0.5, `${name}: at 58 fps or more ${(100 * ro.at60).toFixed(0)} % → ${(100 * rn.at60).toFixed(0)} % of 400 s, mean ${ro.mean.toFixed(1)} → ${rn.mean.toFixed(1)} fps (×${sc(o).toFixed(2)} → ×${sc(n).toFixed(2)})`);
    }
    // (11) auto on the held page: no preset is given up for the hold (as it was: the fight starves high away)
    { const o = mkH(false, { auto: true }), n = mkH(ON, { auto: true }); const k = (t: number) => at(t < 20 ? 11 : t < 80 ? 23 : 11); const ro = watch(o, 140, 120, k, { paceMs: 33.33 }), rn = watch(n, 140, 120, k, { paceMs: 33.33 });
      check(n.q.current === 'high' && !rn.moves.some(m => /auto ↓/.test(m)) && rn.share > 0.9, `auto on the held page through a minute's fight: stays high, at the tier's start ${(100 * rn.share).toFixed(0)} % of the time (off: ${o.q.current} ×${sc(o).toFixed(2)}, ${(100 * ro.share).toFixed(0)} %${ro.moves.some(m => /auto ↓/.test(m)) ? ', a preset given up' : ''})`); }
    // (12) a judged cut that tells nothing (a held page whose GPU time does not follow the pixels: 13 ms whatever the scale, so the cut never lands under the floor): the pixels go back at the end of its trial, no pace is learnt, and the next one is held off
    { const x = mkH(ON); const r = watch(x, 300, 120, () => 13 / modelCost(stateOf(x.q, x.E), w, S0), { paceMs: 33.33 }); const told = r.moves.filter(m => /told nothing/.test(m)).length;
      check(Math.abs(sc(x) - PRESETS.high.scale) < 1e-6 && r.share > 0.85 && told >= 1 && told <= 3 && (x.q as any).pacedAt === 0, `a cut that cannot land under the floor (13 ms at any scale): put back ${told} time${told === 1 ? '' : 's'} in five minutes with no pace learnt, ${(100 * r.share).toFixed(0)} % of them at the tier's start`); }
    // (13) NOT held, the union over the back-off band: a fixed 60 Hz display missing its vsync at 14.2 ms — the band's own steps bring it back as ever; the judged cut waits for them (it would go to the proof's floor and stay there)
    { const off = mkH(false, { hz: 60 }), on = mkH(true, { hz: 60 }); const a = watch(off, 300, 60, () => at(14.2), { vrr: false }), b = watch(on, 300, 60, () => at(14.2), { vrr: false });
      check(words(a.moves) === words(b.moves) && Math.abs(sc(off) - sc(on)) < 1e-6 && sc(on) >= 0.7 - 1e-6, `a fixed 60 Hz display nothing holds, 14.2 ms at high's start (over the band, its vsync missed): the band's steps, the same ${b.moves.length} moves off and on (×${sc(on).toFixed(2)}, ${b.mean.toFixed(0)} fps)`); }
    // (14) held, and the cost model wrong about this machine (35 % of its GPU time follows the pixels, the model says 60): the judged cut lands short of the floor — put back, held off, no cut / step up / cut loop
    { const real = (x: { E: any }) => 13.7 * (0.65 + 0.35 * (sc(x) / PRESETS.high.scale) ** 2); const one = (on: boolean) => { const x = mkH(on); const r = watch(x, 900, 120, () => real(x) / modelCost(stateOf(x.q, x.E), w, S0), { paceMs: 33.33 }); return { r, downs: r.moves.filter(m => /↓/.test(m)).length }; };
      const off = one(false), on = one(ON);
      check(on.downs <= 20 && on.r.scale >= off.r.scale - 0.02, `held at 30 with 13.7 ms that follows the pixels less than the model says: ${on.downs} steps down in fifteen minutes (${off.downs} with the switch off), mean ×${on.r.scale.toFixed(3)} against ×${off.r.scale.toFixed(3)}`); }
    // (15) NOT held: a fixed 60 Hz display, 5.5 ms past the union, 12.5 ms — the judged cut buys its 60 back and the level it left is a failed rung (no cut / climb back / cut every twenty seconds)
    { const one = (on: boolean) => withOver(5.5, () => { const x = mkH(on, { hz: 60 }); const r = watch(x, 600, 60, () => at(12.5), { vrr: false }); return { r, downs: r.moves.filter(m => /↓/.test(m)).length }; }); const off = one(false), on = one(ON);
      check(on.downs <= 6 && on.r.at60 >= 0.9 && on.r.at60 >= off.r.at60, `a fixed 60 Hz display, 5.5 ms past the union, 12.5 ms: at 58 fps or more ${(100 * off.r.at60).toFixed(0)} % → ${(100 * on.r.at60).toFixed(0)} % of ten minutes, ${on.downs} cuts (the level each left held off, doubling)`); }
    // (16) NOT held: the CPU is the limit and its frame wobbles (25 ms ± 8 % a second, 12.1 ms of union on a variable-rate display) — the judged cut is no more frequent than the re-test and the wobble is not read as a cut that helped
    { const wob = (t: number) => { const v = Math.sin((t + 1) * 12.9898) * 43758.5453; return v - Math.floor(v); }; const x = mkH(ON); const r = watch(x, 900, 120, () => at(12.1), { vrr: true, paceMs: t => 25 * (0.92 + 0.16 * wob(Math.floor(t))) }); const downs = r.moves.filter(m => /↓/.test(m)).length;
      check(downs <= 5 && r.share >= 0.95 && Math.abs(sc(x) - PRESETS.high.scale) < 1e-6, `the CPU the limit, 25 ms ± 8 % a second: ${downs} judged cuts in fifteen minutes, ${(100 * r.share).toFixed(0)} % of them at the tier's start (×${sc(x).toFixed(2)})`); }
    // (7) a GPU-bound machine nothing holds: the switch changes no move (the fight of the first block: 11 → 17 → 11 ms on a 120 Hz variable-rate display)
    const load = (t: number) => at(t < 20 ? 11 : t < 80 ? 17 : 11); const o7 = mkH(false), n7 = mkH(true); const ro7 = watch(o7, 200, 120, load), rn7 = watch(n7, 200, 120, load);
    check(JSON.stringify(rn7.moves) === JSON.stringify(ro7.moves) && rn7.moves.length > 0, `nothing holding the page (a fight on a GPU-bound machine): the same ${rn7.moves.length} moves with the switch on and off`);
  }
  console.log('\nOne light-cache grid at every preset (settings.oneCacheGrid: the light meter reads the cache, so its probe grid is part of what a guard sees):');
  {
    const ON = process.env.ONEGRID_BREAK !== '1';   // ONEGRID_BREAK=1: the walk runs with the switch off and the first three must FAIL
    const walk = (on: boolean) => {
      store.clear(); const E = mkEngine(w, { oneCacheGrid: on }); const grid = () => `${E.rc.cfg.nx}x${E.rc.cfg.ny}x${E.rc.cfg.nz}`;
      let rebuilds = 0; const real = E.rebuildRc; E.rebuildRc = function (cfg: any) { rebuilds++; real.call(this, cfg); };
      const parked = new Set<string>(); E.prewarmRc = (cfgs: any[]) => { for (const c of cfgs) parked.add(`${c.nx}x${c.ny}x${c.nz}`); };
      const q: any = new Quality(E as never, { params: { rbIters: 4 } } as never); const seen: Record<string, string> = {};
      for (const n of [...QUALITY_NAMES, ...[...QUALITY_NAMES].reverse()]) { q.apply(n); seen[n] = grid(); }   // (high down to verylow and back up: every step the ladder or a menu can take)
      return { seen, rebuilds, parked: [...parked] };
    };
    const on = walk(ON), off = walk(false), fine = '40x4x28', coarse = '32x3x22';
    check(QUALITY_NAMES.every(n => on.seen[n] === fine), `every preset reads the one grid: ${QUALITY_NAMES.map(n => `${n} ${on.seen[n]}`).join(' · ')}`);
    check(on.rebuilds === 0, `the cache is not rebuilt on any step from high down to verylow and back: ${on.rebuilds} rebuilds`);
    check(on.parked.length === 1 && on.parked[0] === fine, `the boot parks no second grid: ${on.parked.join(', ')}`);
    check(off.seen.low === coarse && off.seen.verylow === coarse && off.seen.high === fine && off.seen.medium === fine && off.rebuilds === 2 && off.parked.length === 2,
      `the switch off = the grids as they were: ${QUALITY_NAMES.map(n => `${n} ${off.seen[n]}`).join(' · ')}, ${off.rebuilds} rebuilds on the walk, ${off.parked.length} grids parked`);
    // the ladder prices the grid that runs: a 4.5 × machine's boot pick (61.1 ms at high, 25.0 of it fixed, 1440 × 900 at 2) on the ladder's own tables, and a live Quality's hop price
    const pick = (one: boolean) => { const r = plan(61.1, 25.0, 'high', 0.43, 900, 2, PLAN_BUDGET_MS, true, one); return `${r.preset} ×${r.scale.toFixed(2)}`; };
    const hop = (one: boolean) => { const x = mkQuality(w, 'low', true, { oneCacheGrid: one }, 60, 120); return x.q.hopRatio('medium') as number; };
    check(pick(false) === 'low ×0.50' && pick(ON) === 'verylow ×0.50' && hop(false) > hop(ON) + 0.1,
      `the ladder prices the grid that runs: a 4.5 × machine boots at ${pick(false)} with the switch off (as it was) and ${pick(ON)} with it on; low → medium priced ×${hop(false).toFixed(2)} off, ×${hop(ON).toFixed(2)} on`);
  }
  console.log('\nOne smoke pressure solve at every preset (settings.oneSmokeSolve: a sight line reads the solver\'s density, so its pairs are part of what smoke hides):');
  {
    const ON = process.env.ONESOLVE_BREAK !== '1';   // ONESOLVE_BREAK=1: the walk runs with the switch off and the first line must FAIL
    const walk = (on: boolean | undefined) => {
      store.clear(); const E = mkEngine(w, on === undefined ? {} : { oneSmokeSolve: on }); const sm = { params: { rbIters: 0 } }; if (on === undefined) delete E.settings.oneSmokeSolve;
      const q: any = new Quality(E as never, sm as never); const seen: Record<string, number> = {};
      for (const n of [...QUALITY_NAMES, ...[...QUALITY_NAMES].reverse()]) { q.apply(n); seen[n] = sm.params.rbIters; }
      return seen;
    };
    const on = walk(ON), off = walk(false), bare = walk(undefined), H = PRESETS.high.rbIters;
    check(QUALITY_NAMES.every(n => on[n] === H), `every preset solves with high's ${H} pairs: ${QUALITY_NAMES.map(n => `${n} ${on[n]}`).join(' · ')}`);
    check(QUALITY_NAMES.every(n => off[n] === PRESETS[n].rbIters) && off.high !== off.medium, `the switch off = the rows as they were: ${QUALITY_NAMES.map(n => `${n} ${off[n]}`).join(' · ')}`);
    check(QUALITY_NAMES.every(n => bare[n] === H), 'a settings object with no such key (a tool\'s mock) solves as the page does');
  }
  console.log('\nThe floor rung sheds an open fire\'s smoke (settings.fireSmokeShed: the solver need not stay awake for a brazier on verylow; every rung above keeps it):');
  {
    const ON = process.env.FIRESHED_BREAK !== '1';   // FIRESHED_BREAK=1: the walks run with the switch off and the first two lines must FAIL
    /** the presets of `path` put on in turn, by hand (auto off) or as the ladder does (auto on): what settings.fireSmoke says after each */
    const walk = (on: boolean | undefined, auto: boolean, path: QualityName[]) => {
      store.clear(); const E = mkEngine(w, on === undefined ? {} : { fireSmokeShed: on }); if (on === undefined) { delete E.settings.fireSmokeShed; delete E.settings.fireSmoke; }
      const q: any = new Quality(E as never, { params: { rbIters: 0 } } as never); q.auto = auto; if (on !== undefined) E.settings.fireSmoke = true;   // (the constructor's own first preset is not this walk's)
      return path.map(n => { q.apply(n); return E.settings.fireSmoke as boolean | undefined; });
    };
    const hand: QualityName[] = [...QUALITY_NAMES, ...[...QUALITY_NAMES].reverse()], climb: QualityName[] = ['medium', 'low', 'verylow', 'low', 'low', 'medium', 'low'];
    const row = (path: QualityName[], s: (boolean | undefined)[]) => path.map((n, i) => `${n} ${s[i] ? 'smoke' : 'none'}`).join(' → ');
    const byHand = walk(ON, false, hand), up = walk(ON, true, climb), offHand = walk(false, false, hand), offUp = walk(false, true, climb), bare = walk(undefined, false, hand);
    check(hand.every((n, i) => byHand[i] === (n !== 'verylow')), `put on by hand, verylow draws a fire without its smoke and every other preset with it, down the ladder and back up: ${row(hand.slice(0, 5), byHand)}`);
    check(up.join() === [true, true, false, false, false, true, true].join(), `moved by auto, the first rung up from the floor keeps the shed (the hop was priced without the smoke), the rung above it has the smoke back, and coming down to low keeps it: ${row(climb, up)}`);
    check(offHand.every(v => v === true) && offUp.every(v => v === true), `the switch off = every preset keeps the fire's smoke, as it was, by hand and under auto: ${row(climb, offUp)}`);
    check(hand.every((n, i) => bare[i] === (n !== 'verylow')), 'a settings object with no such key (a tool\'s mock) sheds as the page does');
  }
  console.log('\nFog and smoke are one march on every tier, and a rung differs by its numbers alone (ui/quality.ts ONE MARCH ON EVERY TIER: medium and low run high\'s march with two lights a step):');
  {
    const BREAK = process.env.FOGROWS_BREAK === '1';   // FOGROWS_BREAK=1: the walk puts verylow on where medium is asked and high where low is, and the first two lines must FAIL
    const put = (name: QualityName) => { store.clear(); const E = mkEngine(w, { oneSmokeSolve: false }); const q: any = new Quality(E as never, { params: { rbIters: 4 } } as never); q.auto = false; q.apply(BREAK && name === 'medium' ? 'verylow' : BREAK && name === 'low' ? 'high' : name); return { S: { ...E.settings } as Record<string, any>, q, E }; };
    const H = put('high').S, under: QualityName[] = ['medium', 'low'], V = put('verylow').S;
    const row = (S: Record<string, any>) => `${S.volQuarter ? 'quarter' : 'half'} size, ${S.volSteps} beam steps, ${S.volLightCap} lights a step, refresh ${S.volShadowEvery}, step ${S.volStepVoxels}`;
    check(under.every(n => { const S = put(n).S; return S.volQuarter === false && S.volSteps === H.volSteps && S.volStepVoxels === H.volStepVoxels && S.volShadowEvery === H.volShadowEvery; }) && H.volQuarter === false && H.volLightCap === 4,
      `medium and low march as high does, the pass at half size with its steps, shadow refresh and step length: ${under.map(n => `${n} ${row(put(n).S)}`).join(' · ')} (high: ${row(H)})`);
    check(under.every(n => put(n).S.volLightCap === 2), `… with two lights a step where high has four: ${under.map(n => `${n} ${put(n).S.volLightCap}`).join(' · ')}`);
    check(V.volQuarter === true && V.volSteps === 8 && V.volLightCap === 2 && PRESETS.verylow.caps.volStep === 2 && PRESETS.high.caps.volStep === 1.5 && V.volStepVoxels === smokeLookStep(2) && V.volStepVoxels > H.volStepVoxels, `verylow, the floor, keeps its own small rows: ${row(V)}`);
    { const T = costTable(true); check(Math.abs(T.medium.passes.volumetrics - T.high.passes.volumetrics) < 1e-9 && Math.abs(T.low.passes.volumetrics - T.high.passes.volumetrics) < 1e-9 && T.verylow.passes.volumetrics < T.low.passes.volumetrics && !('volFroxel' in T.high.passes),
      `the ladder prices the one march a pixel alike on high, medium and low (${T.high.passes.volumetrics.toFixed(2)} model-ms), less on verylow (${T.verylow.passes.volumetrics.toFixed(2)}), and no froxel term is left in any rung's fixed part`); }
    { const x = put('medium'); x.q.lastCut = { from: 0.7 }; x.q.upTrial = { from: 0.55 }; x.q.apply('low'); check(x.q.lastCut === null && x.q.upTrial === null, 'a preset put on still clears the judged cut and the judged step up'); }
  }
  console.log('\nAuto lets sharp direct go before it shrinks the picture (settings.ladderShed; the maintainer\'s pick of 10-25. His Mac\'s sweep priced sharp direct at 4.9 of 15.5 ms at High ×0.5, and the machine below is priced so):');
  {
    const ON = process.env.LADDERSHED_BREAK !== '1';   // LADDERSHED_BREAK=1: the 'on' machines run with the switch off and twenty lines must FAIL
    const hi = (scale: number, sharp = true): State => ({ preset: 'high', scale, gatherThird: true, volQuarter: false, maxH: PRESETS.high.maxH, sharp });
    const withK = <T,>(k: number, f: () => T): T => { const k0 = MODEL.sharpK; MODEL.sharpK = k; try { return f(); } finally { MODEL.sharpK = k0; } };
    const c5 = presetCost({ ...PRESETS.high, scale: 0.5 }, true), mpx5 = (1000 * 0.5) ** 2 * (w.cssW / w.cssH) / 1e6, share = 4.9 / 15.45;
    const macK = share * (c5.fixed - c5.passes.sharp + MODEL.extraFixed + c5.pixel * mpx5 / REF.mpx) / (c5.passes.sharp * (1 - share));   // the sharpK that makes sharp direct that share of High ×0.5 on this window
    const mkS = (on: boolean, o: { auto?: boolean; aim?: number; adaptive?: boolean; hz?: number } = {}) => { const x = mkQuality(w, 'high', o.auto ?? true, { ladderClimb: true, ladderShed: on }, o.aim ?? 60, o.hz ?? 120); if (o.adaptive === false) x.q.adaptive = false; return x; };
    /** `secs` a second at a time: the moves, the fps windows, and the ladder's state at the end of every second */
    const watch = (x: { q: any; E: any }, secs: number, k: (t: number) => number, o: { paceMs?: number; timed?: boolean; t0?: number; hz?: number; vrr?: boolean } = {}) => { const moves: string[] = [], fps: number[] = [], st: State[] = []; for (let t = 0; t < secs; t++) { const r = run(x.q, x.E, 1, o.hz ?? 120, k, { ...o, t0: (o.t0 ?? 0) + t }); moves.push(...r.moves); fps.push(...r.fps); st.push(stateOf(x.q, x.E)); } return { moves, fps, st }; };
    const steps = (m: string[]) => m.filter(x => /↓|↑/.test(x)), tOf = (m?: string) => m ? parseFloat(m) : NaN, first = (m: string[], re: RegExp) => m.find(x => re.test(x));
    const sharpOn = (x: { E: any }) => !!x.E.settings.sharpDirect;
    withK(macK, () => {
      const start = modelCost(hi(PRESETS.high.scale), w, S0), bare = modelCost(hi(PRESETS.high.scale, false), w, S0); const A = (ms: number) => ms / start;   // A(ms): the machine on which high at its start, sharp direct on, costs `ms` of union
      // (1) a fight under auto at High: 12 → 17 → 12 ms
      const fight = (t: number) => A(t < 20 ? 12 : t < 80 ? 17 : 12);
      const o = mkS(false), n = mkS(ON); const ro = watch(o, 300, fight), rn = watch(n, 300, fight);
      const lowO = Math.min(...ro.st.slice(20, 80).map(s => s.scale)), lowN = Math.min(...rn.st.slice(20, 80).map(s => s.scale)); const shedAt = tOf(first(rn.moves, /shed ↓/)), fightN = rn.fps.slice(27, 78);
      check(lowO < PRESETS.high.scale - 0.1 && ro.st.every(s => s.sharp) && !ro.moves.some(m => /shed/.test(m)), `the switch off (the ladder as it was): the fight cuts the picture to ×${lowO.toFixed(2)} with sharp direct on throughout — ${words(steps(ro.moves).slice(0, 3))}`);
      check(/shed ↓/.test(steps(rn.moves)[0] ?? '') && shedAt < 24.5 && Math.abs(lowN - PRESETS.high.scale) < 1e-6 && minOf(fightN) >= HOLD.cut * 60 - 0.5, `on: sharp direct goes ${(shedAt - 20).toFixed(1)} s into the fight and the picture keeps its size (×${lowN.toFixed(2)}, never under the tier's start) at ${minOf(fightN).toFixed(0)}‥${Math.max(...fightN).toFixed(0)} fps — ${words(steps(rn.moves).slice(0, 3))}`);
      const m2 = mkS(ON); watch(m2, 30, fight);
      check(!m2.E.settings.sharpDirect && !m2.E.settings.sharpDecouple && !m2.E.settings.gbaa && m2.q.shedState.shed, `… with its two riders (the decoupled jitter, GBAA) off too: sharpDirect ${m2.E.settings.sharpDirect}, sharpDecouple ${m2.E.settings.sharpDecouple}, gbaa ${m2.E.settings.gbaa}`);
      // (2) what the union showed when it went is the price of having it back
      const real = A(17) * (start - bare), seen = m2.q.shedState.seenMs as number; MODEL.sharpK = 1; const modelOnly = A(17) * bare * (modelCost(hi(PRESETS.high.scale), w, S0) / modelCost(hi(PRESETS.high.scale, false), w, S0) - 1); MODEL.sharpK = macK;
      check(Math.abs(seen - real) < 0.15 * real && seen > 1.5 * modelOnly, `this machine's own price for it is learnt off the shed: ${seen.toFixed(1)} ms seen (it costs ${real.toFixed(1)} here; the cost model alone says ${modelOnly.toFixed(1)})`);
      // (3) the fight over at 12 ms (9.3 without it): while it is held off the room goes to pixels, never past 0.95; then it comes back ON THE TIER'S OWN PICTURE — the climb's rungs come off for it — and stays
      const ups = rn.moves.filter(m => /shed ↑/.test(m)), upT = tOf(ups[0]), topSc = Math.max(...rn.st.map(s => s.scale)), afterUp = rn.moves.slice(rn.moves.indexOf(ups[0]) + 1);
      check(ups.length === 1 && upT - shedAt >= SHED.holdSecs && /rungs? off/.test(ups[0]) && topSc > PRESETS.high.scale + 0.04 && topSc <= 0.95 + 1e-6 && sharpOn(n) && Math.abs(sc(n) - PRESETS.high.scale) < 1e-6 && !afterUp.some(m => /shed ↓/.test(m)) && minOf(rn.fps.slice(84)) >= HOLD.cut * 60 - 0.5,
        `the fight over at 12 ms: held off, the room goes to pixels (×${topSc.toFixed(2)} at the most, never past ×0.95 so there is a size left for it); at ${upT.toFixed(0)} s it is back on the tier's own picture, the rungs off for it (“${ups[0]?.split('(').pop()?.replace(/\)$/, '') ?? ''}”), and stays: ×${sc(n).toFixed(2)} with it, ${minOf(rn.fps.slice(84)).toFixed(0)} fps and up (the switch off ends ×${sc(o).toFixed(2)})`);
      { const mk3 = (forget: boolean) => { const x = mkS(ON); const r1 = watch(x, 80, t => A(t < 20 ? 12 : 17)); if (forget) x.q.shedSeen = null; const r2 = watch(x, 900, () => A(13.5), { t0: 80 }); return { x, moves: [...r1.moves, ...r2.moves] }; }; const kept = mk3(false), lost = mk3(true);
        const upsOf = (m: string[]) => m.filter(x => /shed ↑/.test(x)), back = tOf(upsOf(lost.moves)[0]), backK = tOf(upsOf(kept.moves)[0]), again = lost.moves.find((m, i) => i > lost.moves.indexOf(upsOf(lost.moves)[0]) && /shed ↓/.test(m));
        check(upsOf(kept.moves).length === 1 && upsOf(lost.moves).length === 1 && backK > back + 30 && !!again && !sharpOn(kept.x) && !sharpOn(lost.x) && Math.abs(sc(kept.x) - PRESETS.high.scale) < 1e-6 && kept.x.q.shedState.seenMs > 0,
          `… where the cost model underprices it (13.5 ms after the fight: it prices 11.8, the machine pays 13.5, over the band): ONE try in fifteen minutes — at ${backK.toFixed(0)} s, once the price read off the fight is too old to stand (that price forgotten: at ${back.toFixed(0)} s, off again at ${tOf(again).toFixed(0)} s, ${/not back for (\d+) s/.exec(again ?? '')?.[1] ?? '?'} s held) — the try reads ${(kept.x.q.shedState.seenMs as number).toFixed(1)} ms for it in a calm scene, and from there it is left off (×${sc(kept.x).toFixed(2)}, the picture its own size throughout)`); }
      // (4) a fight that ends in a light scene: back, no sooner than the hold-off, with the tier at its own scale first
      const b = mkS(ON); const rb = watch(b, 400, t => A(t < 20 ? 12 : t < 80 ? 22 : 9)); const dn = tOf(first(rb.moves, /shed ↓/)), up = tOf(first(rb.moves, /shed ↑/)); const upI = Math.max(0, Math.floor(up) - 1);
      check(up - dn >= SHED.holdSecs && rb.st[upI]?.scale >= PRESETS.high.scale - 1e-6 && rb.st[upI]?.preset === 'high' && sharpOn(b) && b.E.settings.sharpDecouple && b.E.settings.gbaa && minOf(rb.fps.slice(Math.floor(up))) >= HOLD.cut * 60 - 0.5, `a fight that ends in a light scene (12 → 22 → 9 ms): off at ${dn.toFixed(0)} s, back at ${up.toFixed(0)} s with its riders — the tier at ×${rb.st[upI]?.scale.toFixed(2)} first — and never under ${minOf(rb.fps.slice(Math.floor(up))).toFixed(0)} fps after (ends ×${sc(b).toFixed(2)})`);
      // (5) down the presets and back: the shed rides the hops
      const slow = (t: number) => A(t < 90 ? 30 : 7); const po = mkS(false), pn = mkS(ON); const rpo = watch(po, 400, slow), rpn = watch(pn, 400, slow);
      const idx = (r: { st: State[] }) => Math.min(...r.st.map(s => QUALITY_NAMES.indexOf(s.preset) < 0 ? 0 : -QUALITY_NAMES.indexOf(s.preset))); const hopDn = tOf(first(rpn.moves, /auto ↓/)), hopUp = tOf(first(rpn.moves, /auto ↑ high/)), back5 = tOf(first(rpn.moves, /shed ↑/));
      const lowest = (r: { st: State[] }) => QUALITY_NAMES[-idx(r)];
      check(/shed ↓/.test(steps(rpn.moves)[0] ?? '') && hopDn > 0 && rpn.st.slice(Math.ceil(hopDn), Math.floor(hopUp)).every(s => !s.sharp) && rpn.st[Math.floor(hopUp) + 1]?.sharp === false && back5 > hopUp && sharpOn(pn) && pn.q.current === 'high' && -idx(rpn) <= -idx(rpo),
        `a machine too slow for High (30 ms), light again at 90 s: sharp direct first, then the cuts and ${lowest(rpn)} (the switch off goes down to ${lowest(rpo)}); it stays off across both hops (${hopDn.toFixed(0)} s down, ${hopUp.toFixed(0)} s up, priced without it) and comes back at ${back5.toFixed(0)} s on High`);
      // (6) fights that keep coming: each shed after a give-back holds it off twice as long
      const f = mkS(ON); const rf = watch(f, 900, t => A(t % 150 < 60 ? 17 : 10)); const holds = rf.moves.filter(m => /shed ↓/.test(m)).map(m => +(/not back for (\d+) s/.exec(m)?.[1] ?? 0));
      check(holds.length >= 3 && holds[0] === SHED.holdSecs && holds.every((h, i) => i === 0 || h === 2 * holds[i - 1]), `a fight every 150 s: held off ${holds.join(' → ')} s — it converges on left alone instead of flipping with every fight (${rf.moves.filter(m => /shed/.test(m)).length} sheds and returns in fifteen minutes)`);
      // (7) a page something else holds to 30 fps (13.9 ms: over the back-off band): the shed is judged with the cut after it and goes back with the pixels
      const h = mkS(ON); const rh = watch(h, 300, () => A(13.9), { paceMs: 33.33 }); const onShare = rh.st.filter(s => s.sharp).length / rh.st.length;
      check(sharpOn(h) && Math.abs(sc(h) - PRESETS.high.scale) < 1e-6 && rh.moves.some(m => /sharp direct with them/.test(m)) && onShare > 0.9 && h.q.shedState.sheds === 0, `held to 30 fps by something else: the shed buys no frame rate there, is taken back with the judged cut (“${first(rh.moves, /sharp direct with them/)?.split(': ').pop()?.replace(/\)$/, '') ?? ''}”) and counts for nothing — sharp direct on ${(100 * onShare).toFixed(0)} % of five minutes, the re-tests the rest`);
      // (8) the other roads to a smaller picture: adaptive resolution off (a preset would go), no GPU timings (the rate alone), the as-now bands
      const ad = mkS(ON, { adaptive: false }); const rad = watch(ad, 200, t => A(t < 60 ? 19 : 8));
      check(/shed ↓/.test(steps(rad.moves)[0] ?? '') && sharpOn(ad) && ad.q.current === 'high', `adaptive resolution off: sharp direct goes before the preset does, and is back on High afterwards — ${words(steps(rad.moves))}`);
      const bl = mkS(ON); const rbl = watch(bl, 200, t => A(t < 60 ? 19 : 8), { timed: false }); const blUp = tOf(first(rbl.moves, /shed ↑/));
      check(/shed ↓/.test(steps(rbl.moves)[0] ?? '') && blUp >= SHED.holdSecs && sharpOn(bl), `no GPU timings: off by the rate alone, back at ${blUp.toFixed(0)} s by the rate alone (${HOLD.blindWindows} good windows after the hold-off) — ${words(steps(rbl.moves))}`);
      const an = mkS(ON, { aim: 0 }); const ran = watch(an, 60, t => A(t < 10 ? 11 : 24));
      check(/shed ↓/.test(steps(ran.moves)[0] ?? ''), `as now (?aim=asnow), 24 ms: the cut under 45 fps lets it go first too — ${words(steps(ran.moves).slice(0, 3))}`);
      // (9) what it must NOT touch
      const same = (name: string, mk: (on: boolean) => { q: any; E: any }, k: (t: number) => number, secs = 120, want = (m: string[]) => m.some(x => /↓/.test(x))) => { const a = mk(true), z = mk(false); const ra = watch(a, secs, k), rz = watch(z, secs, k); check(JSON.stringify(ra.moves) === JSON.stringify(rz.moves) && !ra.moves.some(m => /shed/.test(m)) && want(ra.moves) && sharpOn(a) === sharpOn(z), `${name}: the same ${steps(ra.moves).length} moves with the switch on and off, sharp direct ${sharpOn(a) ? 'on' : 'off'}`); };
      same('a preset pinned by hand (High, adaptive resolution) through that fight', on => mkS(on, { auto: false }), fight);
      same('a machine with room (8 ms: it climbs)', on => mkS(on), () => A(8), 120, m => m.some(x => /climb ↑/.test(x)));
      sharpForce.v = true; try { same('?sharp (forced on)', on => { const x = mkS(on); x.q.apply('high'); x.q.cooldown = 0; x.q.lastChange = ''; return x; }, fight); } finally { sharpForce.v = null; }
      { const a = mkS(true), z = mkS(false); delete a.E.settings.ladderShed; const ra = watch(a, 120, fight), rz = watch(z, 120, fight); check(JSON.stringify(ra.moves) === JSON.stringify(rz.moves), 'a settings object with no such key (a tool\'s mock) = the ladder as it was'); }
      // (10) the shed ends with the switch, a pin, a new auto spell
      const sw = mkS(ON); watch(sw, 40, fight); const was = sharpOn(sw); sw.E.settings.ladderShed = false; const rsw = watch(sw, 2, fight, { t0: 40 });
      check(!was && sharpOn(sw) && sw.E.settings.gbaa && !sw.q.shedState.shed && rsw.moves.some(m => /shed ↑ sharp direct back \(the switch is off\)/.test(m)), `the switch put off mid-fight (the Tab row): sharp direct is back within two seconds — ${words(rsw.moves)}`);
      const pinned = mkS(ON); watch(pinned, 40, fight); const wasP = sharpOn(pinned); pinned.q.pin('medium'); const pinOn = sharpOn(pinned); pinned.q.setAuto(); pinned.q.calib = { ...pinned.q.calib, done: true };
      check(!wasP && pinOn && !pinned.q.shedState.shed && sharpOn(pinned) && pinned.q.shedState.sheds === 0, 'a preset pinned from the menu mid-fight has its own rows (sharp direct on), and auto chosen again starts with it on and nothing held off');
    });
    // (12) a spike that ends inside the second the price is read (17 ms for 3 s, then 10.2: the reading is the spike ending, not sharp direct): the reading does not stand for ever
    { const x = mkS(ON); const r = watch(x, 400, t => at(t < 20 ? 12 : t < 23 ? 17 : 10.2)); const dn = tOf(first(r.moves, /shed ↓/)), up = tOf(first(r.moves, /shed ↑/)), reads = (x.q.shedState.seenMs as number);
      check(dn > 0 && up - dn >= SHED.holdSecs && up - dn <= 3 * SHED.holdSecs && sharpOn(x) && r.moves.filter(m => /shed ↓/.test(m)).length === 1, `a three-second spike, then 10.2 ms: off at ${dn.toFixed(0)} s, and the price read off the spike's own end is not kept past twice the hold-off — back at ${up.toFixed(0)} s and it stays (it reads ${reads.toFixed(1)} ms for it there)`); }
    // (13) a fixed 60 Hz display, 5.5 ms of frame past the union, 12.5 ms at High's start: inside the band, its vsync missed (30 fps). An aim's judged cut would take pixels: sharp direct goes instead
    { const O0 = MODEL.overheadMs; MODEL.overheadMs = 5.5; try { const o = mkS(false, { hz: 60 }), n = mkS(ON, { hz: 60 }); const ro = watch(o, 200, () => at(12.5), { hz: 60, vrr: false }), rn = watch(n, 200, () => at(12.5), { hz: 60, vrr: false });
      check(sc(o) < PRESETS.high.scale - 0.04 && sharpOn(o) && Math.abs(sc(n) - PRESETS.high.scale) < 1e-6 && !sharpOn(n) && rn.fps[rn.fps.length - 1] > 59 && ro.fps[ro.fps.length - 1] > 59, `a fixed 60 Hz display missing its vsync with the union inside the band (12.5 ms, 5.5 ms of frame past it): the switch off cuts to ×${sc(o).toFixed(2)} with sharp direct on; on, it goes instead and every pixel stays (×${sc(n).toFixed(2)}, ${rn.fps[rn.fps.length - 1].toFixed(0)} fps) — ${words(steps(rn.moves).slice(0, 2))}`); } finally { MODEL.overheadMs = O0; } }
    // (14) down to Medium in a fight, then a light scene on a page something holds to 30 fps: the hop up can never be taken at that rate, and must not hold sharp direct off
    { const x = mkS(ON); const r = watch(x, 90, () => at(30)); const mid = x.q.current as string; const r2 = watch(x, 500, () => at(5), { t0: 90, paceMs: 33.33 });
      check(/shed ↓/.test(steps(r.moves)[0] ?? '') && mid !== 'high' && x.q.current === mid && sharpOn(x) && r2.moves.some(m => /shed ↑/.test(m)), `30 ms for 90 s (down to ${mid}, the shed kept), then 5 ms on a page held to 30 fps: no hop up at that rate, and sharp direct is back on ${x.q.current} at ${tOf(first(r2.moves, /shed ↑/)).toFixed(0)} s`); }
    // (15) put back on by hand while shed (the Tab row): the ladder holds no shed against it, and may let it go again
    { const x = mkS(ON); watch(x, 30, t => at(t < 5 ? 12 : 17)); const was = x.q.shedState.shed; x.E.settings.sharpDirect = true; watch(x, 1, () => at(12), { t0: 30 }); const cleared = !x.q.shedState.shed; const r = watch(x, 30, () => at(19), { t0: 31 });
      check(was && cleared && /shed ↓/.test(steps(r.moves)[0] ?? ''), 'sharp direct put back on by hand while shed: the ladder forgets its shed within a second, and in the next fight lets it go again before any pixel'); }
    // (16) a take begun with the shed standing (PerfDump.shedAtStart): the replayed ladder knows sharp direct is its own to bring back; without the stamp it is a row put off by hand, and stays off
    { const mk = (stamp: boolean) => { const x = mkQuality(w, 'high', true, { ladderClimb: true, ladderShed: ON, sharpDirect: false }, 60, 120, stamp); const r = watch(x, 200, () => at(8)); return { x, r }; }; const a = mk(true), z = mk(false);
      check(a.x.E.settings.sharpDirect === true && a.r.moves.some(m => /shed ↑/.test(m)) && z.x.E.settings.sharpDirect === false && !z.r.moves.some(m => /shed/.test(m)), `a take begun mid-shed replays with the shed standing (sharp direct back at ${tOf(first(a.r.moves, /shed ↑/)).toFixed(0)} s in a light scene); the same settings without the stamp = put off by hand: left off`); }
    // (11) the cost model's own machine, a steady 14.5 ms: the picture stays at High's start where it was cut a step
    { const o = mkS(false), n = mkS(ON); watch(o, 300, () => at(14.5)); const rn = watch(n, 300, () => at(14.5));
      check(sc(o) < PRESETS.high.scale - 0.04 && sharpOn(o) && Math.abs(sc(n) - PRESETS.high.scale) < 1e-6 && !sharpOn(n) && steps(rn.moves).length === 1 && minOf(rn.fps.slice(5)) >= HOLD.cut * 60 - 0.5, `on the cost model's own machine, a steady 14.5 ms: ×${sc(n).toFixed(2)} without sharp direct for five minutes in one move, ${minOf(rn.fps.slice(5)).toFixed(0)} fps and up (the switch off: ×${sc(o).toFixed(2)} with it)`); }
  }

  console.log(`\n'auto' chosen after boot (the real Quality over a stub engine with timers; a 4.5× machine on 1440×900 @2):`);
  // (A) boots pinned low, 'auto' on the pause card
  {
    store.clear(); store.set('twopointsix.quality', 'low');
    const { E, q } = machine(4.5); let fired = 0; q.onCalibrated = () => { fired++; };
    check(!q.auto && q.current === 'low' && !q.calibrating, `pinned boot: auto ${q.auto}, at ${q.current}, calibrating ${q.calibrating} (want false / low / false)`);
    const idle = pump(E, q, 120); check(idle === 0 && !q.calibrating, `two seconds pinned: no calibration runs (${idle} decisions)`);
    q.setAuto(); const startedAt = q.current; const note0 = q.lastChange;
    check(q.auto && q.calibrating && /calibrating/.test(note0), `setAuto → auto ${q.auto}, calibrating ${q.calibrating}, note “${note0}”`);
    const d1 = pump(E, q, 100); const settled = `${q.current} ×${E.settings.renderScale.toFixed(2)}`;
    check(d1 === 1 && !q.calibrating && /^calibrated/.test(q.lastChange) && fired === 1, `~1.3 s behind the card: decided ${d1}× (started at ${startedAt}, settled ${settled}), onCalibrated fired ${fired}×, note “${q.lastChange}”`);
    const d2 = pump(E, q, 300); check(d2 === 0 && `${q.current} ×${E.settings.renderScale.toFixed(2)}` === settled, `then quiet: ${d2} more decisions over 5 s, still ${q.current} ×${E.settings.renderScale.toFixed(2)}`);
    check(store.get('twopointsix.quality') === 'auto' && store.get('twopointsix.autoPreset') === q.current && Math.abs(Number(store.get('twopointsix.autoScaleRel')) - E.settings.renderScale) < 1e-6, `remembered: mode ${store.get('twopointsix.quality')}, autoPreset ${store.get('twopointsix.autoPreset')}, autoScaleRel ${store.get('twopointsix.autoScaleRel')}`);
  }
  // (B) auto boot on the same slow machine → settles; pin high; 'auto' again → starts from the settled pick, not high ×1.0, and re-measures once
  {
    store.clear();
    const { E, q } = machine(4.5);
    check(q.auto && q.calibrating && q.current === 'high', `first visit: auto, calibrating from ${q.current}`);
    const d0 = pump(E, q, 100); const first = { preset: q.current, scale: E.settings.renderScale };
    check(d0 === 1 && AUTO_LADDER.indexOf(first.preset as QualityName) < AUTO_LADDER.indexOf('high'), `boot calibration: decided ${d0}× → ${first.preset} ×${first.scale.toFixed(2)} (“${q.lastChange}”)`);
    q.pin('high'); const dPinned = pump(E, q, 120);
    check(!q.auto && q.current === 'high' && dPinned === 0, `pin high: auto ${q.auto}, ${q.current}, ${dPinned} decisions while pinned`);
    q.setAuto();
    check(q.current === first.preset && Math.abs(E.settings.renderScale - first.scale) < 1e-6 && q.calibrating, `setAuto: starts at ${q.current} ×${E.settings.renderScale.toFixed(2)} = what auto settled on here (not high ×1.00), calibrating ${q.calibrating} (“${q.lastChange}”)`);
    const d3 = pump(E, q, 100);
    check(d3 === 1 && !q.calibrating && q.current === first.preset && Math.abs(E.settings.renderScale - first.scale) < 0.02, `re-measured once behind the card: ${q.current} ×${E.settings.renderScale.toFixed(2)} — the same answer from the same machine, no crawl (“${q.lastChange}”)`);
  }
  // (C) a fast machine settles at high's own start on its first visit (high ×0.77 since TAAU: 0.59× the pixels puts the reference M4 Max back under the 15 ms budget at high; medium ×1.0 before), and auto chosen again from another tier settles there once more
  {
    store.clear();
    const { E, q } = machine(1);
    pump(E, q, 100); const settledAt = q.current as QualityName;
    q.auto = false; q.apply('medium'); q.setAuto(); const from = q.current, cal = q.calibrating;
    const dd = pump(E, q, 100); check(settledAt === 'high' && from === settledAt && cal && dd === 1 && q.current === settledAt && Math.abs(E.settings.renderScale - PRESETS[settledAt].scale) < 1e-6, `setAuto on the fast machine: from its remembered ${from}, calibrating ${cal}, settles once: ${q.current} ×${E.settings.renderScale.toFixed(2)} (${settledAt} at its own starting scale ×${PRESETS[settledAt].scale})`);
  }
  // (D1) STANDS IN: while a program asked for through the later lane is still compiling (gpu/device.ts laterProgress) the ladder takes no step and judges none — for at most STANDS_IN.maxSecs
  {
    const BREAK = process.env.LADDERHOLD_BREAK === '1';   // (LADDERHOLD_BREAK=1: the count is never raised — must FAIL)
    const slow = (q: any, secs: number) => { for (let i = 0; i < secs * 40; i++) q.sample(1 / 40); };
    { const { E, q } = pinnedHigh(); const sc0 = E.settings.renderScale, note0 = q.lastChange, clock0 = q.clock;
      if (!BREAK) laterProgress.pending = 2;
      slow(q, 8);
      check(E.settings.renderScale === sc0 && q.lastChange === note0 && q.clock === clock0 && Math.abs(q.standsInFor - 8) < 0.1, `a program still compiling: eight seconds of 40 fps frames and the ladder has not moved (scale ${sc0.toFixed(2)} → ${E.settings.renderScale.toFixed(2)}, its clock ${(q.clock - clock0).toFixed(1)} s on, waited ${q.standsInFor.toFixed(1)} s)`);
      laterProgress.pending = 0; slow(q, 8);
      check(E.settings.renderScale < sc0 - 0.04 && q.standsInFor === 0, `it is in: the same frames are judged and the ladder cuts (${sc0.toFixed(2)} → ${E.settings.renderScale.toFixed(2)}, “${q.lastChange}”)`); }
    { const { E, q } = pinnedHigh(); const sc0 = E.settings.renderScale;
      if (!BREAK) laterProgress.pending = 1;
      slow(q, STANDS_IN.maxSecs - 1); const held = E.settings.renderScale === sc0; slow(q, 9);
      check(held && E.settings.renderScale < sc0 - 0.04, `one that never arrives: held a second short of ${STANDS_IN.maxSecs} s (${held}), and past it the ladder runs again (${sc0.toFixed(2)} → ${E.settings.renderScale.toFixed(2)})`);
      laterProgress.pending = 0; }
    // … and the boot's own measure waits the same way and no longer (calibrateTick): a later set that takes a minute must not leave a slow machine a minute with no quality step
    { store.clear(); const { E, q } = machine(4.5); if (!q.auto) q.setAuto();
      if (!BREAK) laterProgress.pending = 3;
      const d0 = pump(E, q, 10 * 60), waiting = q.calibrating && !q.measuring;
      const d1 = pump(E, q, (STANDS_IN.maxSecs - 11) * 60), still = q.calibrating;
      const d2 = pump(E, q, 4 * 60);
      check(d0 === 0 && waiting && d1 === 0 && still && d2 === 1 && !q.calibrating && q.early === true, `the boot's measure with programs still compiling: ten seconds in it waits and reads no frame (${d0} decided, waiting ${waiting}), a second short of ${STANDS_IN.maxSecs} s still (${d1}, ${still}), and past it the picture as it is is measured once and marked early (${d2}, early ${q.early}; “${q.lastChange}”)`);
      q.remeasure(); const heldOn = !q.calibrating && q.early === true;   // (still out: nothing to measure again yet)
      laterProgress.pending = 0; q.remeasure(); const again = q.calibrating && q.early === false, d3 = pump(E, q, 3 * 60); q.remeasure();
      check(heldOn && again && d3 === 1 && !q.calibrating, `…and when the programs are in the page measures once more, on the settings' own picture, and only once (asked while still out: nothing ${heldOn}; in: calibrating ${again}, decided ${d3}×)`); }
    { store.clear(); const { E, q } = machine(4.5); if (!q.auto) q.setAuto();
      if (!BREAK) laterProgress.pending = 3;
      let stalled = 0; for (let i = 0; i < STANDS_IN.maxSecs + 2; i++) { E.tick(); q.calibrateTick(1.0); if (!q.calibrating) stalled++; }   // (a compile that stalls every frame a whole second: those frames measure nothing, and still count to the wait)
      const out = q.measuring === true && q.early === true;
      check(stalled === 0 && out, `frames a compile stalls for a second each still run the wait out (${STANDS_IN.maxSecs + 2} of them: measuring ${q.measuring}, early ${q.early}), and none of them is read (${stalled})`);
      laterProgress.pending = 0; }
    { store.clear(); const { E, q } = machine(4.5); if (!q.auto) q.setAuto();
      if (!BREAK) laterProgress.pending = 3;
      const d0 = pump(E, q, 5 * 60); laterProgress.pending = 0; const d1 = pump(E, q, 2 * 60);
      check(d0 === 0 && d1 === 1 && q.standsInFor === 0 && q.measuring === false, `they land five seconds in: the measure is taken then, on the settings' own picture (${d0} then ${d1})`); }
  }
  // (D), (E): the as-now ladder (?aim=asnow), the rules it keeps: cut under 45, restore over 58, starve by the union's floor. Hold 60's own rules are the blocks above
  // (D) the runtime ladder must actually CUT when frames are slow — and every cut must be a real cut in pixels (08-10 field dumps: eff sat pinned at the cap for
  //     whole sessions at 33–54 fps while lastChange announced 'adaptive ↓ scale 1.00' — a trailing comment had swallowed the renderScale assignment; and before
  //     08-11 the top of the scale's travel was dead under a HiDPI cap anyway). Also: steps ≥ SCALE_STEP, on the grid, never closer than LADDER.stepCooldown,
  //     never under the floor; and pixels come back when it is fast again.
  {
    const { E, q } = pinnedHigh(0);
    const eff0 = E.effectiveScale, rows0 = Math.floor(1800 * eff0);   // high's 1000-row cap on 900 CSS × 2 = eff 0.556 at render scale 1.0
    const steps: { t: number; scale: number; eff: number }[] = []; let t = 0, lastScale = E.settings.renderScale;
    const run = (secs: number, dt: number) => { for (let i = 0; i < Math.round(secs / dt); i++) { t += dt; q.sample(dt); if (E.settings.renderScale !== lastScale) { steps.push({ t, scale: E.settings.renderScale, eff: E.effectiveScale }); lastScale = E.settings.renderScale; } } };
    run(8, 1 / 40);                                                 // eight seconds of 40 fps frames
    const cuts = steps.length;
    check(E.settings.renderScale < 1 - 1e-3 && E.effectiveScale < eff0 - 0.01 && /adaptive ↓ scale 0\./.test(q.lastChange), `slow frames: the ladder really cuts, in pixels (render scale 1.00 → ${E.settings.renderScale.toFixed(2)}; ${rows0} → ${Math.floor(1800 * E.effectiveScale)} rows; “${q.lastChange}”)`);
    const gaps = steps.slice(1).map((s, i) => s.t - steps[i].t), sizes = steps.map((s, i) => Math.abs(s.scale - (i ? steps[i - 1].scale : 1)));
    const onGrid = steps.every(s => Math.abs(s.scale / SCALE_STEP - Math.round(s.scale / SCALE_STEP)) < 1e-6);
    check(cuts >= 1 && gaps.every(g => g >= LADDER.stepCooldown - 1e-6) && sizes.every(z => z >= SCALE_STEP - 1e-6) && onGrid && steps.every(s => s.scale >= SCALE_FLOOR - 1e-6),
      `${cuts} cut(s) at t = ${steps.map(s => s.t.toFixed(1)).join(' / ')} s → scales ${steps.map(s => s.scale.toFixed(2)).join(' / ')}: gaps ≥ ${LADDER.stepCooldown} s (${gaps.map(g => g.toFixed(1)).join(', ') || '—'}), steps ≥ ${SCALE_STEP}, on the grid, ≥ floor ${SCALE_FLOOR}`);
    const cut = E.settings.renderScale; const nCut = steps.length;
    run(12, 1 / 70);                                                // then twelve seconds comfortably fast: it gives pixels back, up to the preset's own scale
    const ups = steps.slice(nCut); const upGaps = ups.slice(1).map((s, i) => s.t - ups[i].t);
    check(E.settings.renderScale > cut + 0.01 && upGaps.every(g => g >= LADDER.stepCooldown - 1e-6) && ups.every(s => Math.abs(s.scale / SCALE_STEP - Math.round(s.scale / SCALE_STEP)) < 1e-6), `fast again: pixels come back (${cut.toFixed(2)} → ${E.settings.renderScale.toFixed(2)} in ${ups.length} step(s), gaps ${upGaps.map(g => g.toFixed(1)).join(', ') || '—'} s)`);
    run(30, 1 / 40);                                                // and a long starve pinned at high (auto off): down to the floor and no further
    check(Math.abs(E.settings.renderScale - SCALE_FLOOR) < 1e-6, `a 30 s starve pinned at high stops at the floor: render scale ${E.settings.renderScale.toFixed(2)} = ${Math.floor(1800 * E.effectiveScale)} rows (floor ${SCALE_FLOOR} of the ${rows0}-row cap)`);
  }
  // (E) THE HEADROOM-AWARE CLIMB (GI in motion, quality.ts CLIMB): pinned high, adaptive on, presented at 60 fps with the GPU union handed to sample(). With the union well
  //     under 60 % of the 16.7-ms budget it must climb — scale first, 0.05 a rung, ≥ climbGap apart, never past 1.0, then the gather's divisor — and with the union over
  //     80 % (or the frame rate under 55) it must undo the newest rung at once, hold that rung off for failSecs, and never oscillate; with no union at all it never acts;
  //     under auto below the top of the ladder the PRESET goes up instead.
  {
    console.log(`\nThe headroom-aware climb (CLIMB: headroom ${CLIMB.headroom} / divisors ${CLIMB.divisorRoom} / back-off ${CLIMB.backoff} of a ${CLIMB.budgetFps}-fps budget, ${CLIMB.windows} windows, gap ${CLIMB.climbGap} s, fail hold ${CLIMB.failSecs} s):`);
    const mk = () => pinnedHigh(0);
    type Ev = { t: number; scale: number; g3: boolean; note: string };
    /** CLOSED LOOP: the union re-derived every window from what is being RENDERED (presetCost of the live preset — scale, divisors, cap — × k, on the stub's 1440×900 @2 window), so a rung the
     *  climb takes makes the next window dearer and a hop is priced against a union that moved; `k` = the machine (ms of union per modelled ms). fps fixed (the rate is not modelled here). */
    const liveUnion = (q: any, E: any, k: number) => { const S = E.settings; const name = q.current as QualityName; const pc = PRESETS[name]; const live = { ...pc, lossy: { ...pc.lossy, gatherThird: !!S.gatherThird, volQuarter: !!S.volQuarter, rcHalfRate: !!S.rcHalfRate }, maxH: S.maxInternalHeight || pc.maxH, scale: S.renderScale }; const c = presetCost(live as any); const rows = Math.min(live.maxH, 900 * 2) * S.renderScale; const mpx = rows * rows * (1440 / 900) / 1e6; return k * (c.fixed + c.pixel * mpx / REF.mpx); };
    const driveLive = (q: any, E: any, secs: number, fps: number, k: number, log: Ev[], t0: { t: number }) => { const dt = 1 / fps; let last = JSON.stringify([q.current, E.settings.renderScale, E.settings.gatherThird, E.settings.volQuarter]); for (let w = 0; w < secs; w++) { const u = liveUnion(q, E, k); for (let i = 0; i < fps; i++) { t0.t += dt; q.sample(dt, u); const now = JSON.stringify([q.current, E.settings.renderScale, E.settings.gatherThird, E.settings.volQuarter]); if (now !== last) { log.push({ t: t0.t, scale: E.settings.renderScale, g3: E.settings.gatherThird, note: `${q.lastChange} [∪ ${u.toFixed(1)}]` }); last = now; } } } };
    const drive = (q: any, E: any, secs: number, fps: number, union: number, log: Ev[], t0: { t: number }) => { const dt = 1 / fps; let last = JSON.stringify([E.settings.renderScale, E.settings.gatherThird, E.settings.volQuarter, E.settings.maxInternalHeight]); for (let i = 0; i < Math.round(secs * fps); i++) { t0.t += dt; q.sample(dt, union); const now = JSON.stringify([E.settings.renderScale, E.settings.gatherThird, E.settings.volQuarter, E.settings.maxInternalHeight]); if (now !== last) { log.push({ t: t0.t, scale: E.settings.renderScale, g3: E.settings.gatherThird, note: q.lastChange }); last = now; } } };
    // E1: climbs with headroom — the scale to its TOP (0.95 while sharp direct rides under TAAU: at 1.0 there is nothing to decouple and the resolve configuration would switch off), then the divisors
    { const { E, q } = mk(); const log: Ev[] = []; const T = { t: 0 }; const s0 = E.settings.renderScale; const topWant = E.settings.sharpDirect && E.settings.taau !== false ? 0.95 : 1.0;
      drive(q, E, 60, 60, 8.0, log, T);   // a minute at 60 fps with the GPU 8 ms busy of 16.7 (48 %)
      const scaleRungs = log.filter(e => /climb ↑ scale/.test(e.note)); const gaps = log.slice(1).map((e, i) => e.t - log[i].t);
      const orderOk = (() => { let sawGather = false; for (const e of log) { if (/gather/.test(e.note)) sawGather = true; else if (/climb ↑ scale/.test(e.note) && sawGather) return false; } return true; })();
      check(scaleRungs.length >= 3 && Math.abs(E.settings.renderScale - topWant) < 1e-6 && log.every(e => e.scale <= topWant + 1e-9), `headroom (∪ 8 ms @ 60 fps): the scale climbs ${s0.toFixed(2)} → ${scaleRungs.map(e => e.scale.toFixed(2)).join(' → ')} and stops at ${topWant.toFixed(2)} (sharp direct ${E.settings.sharpDirect ? 'on: 1.0 would switch it off' : 'off'}), never above`);
      check(gaps.every(g => g >= CLIMB.climbGap - 1e-6) && log[0].t >= CLIMB.windows - 1e-6, `≥ ${CLIMB.climbGap} s between climbs (gaps ${gaps.map(g => g.toFixed(1)).join(', ')}), the first only after ${CLIMB.windows} roomy windows (t = ${log[0].t.toFixed(1)} s)`);
      check(!E.settings.gatherThird && orderOk && log.filter(e => /gather/.test(e.note)).length === 1, `then the divisors, in order: gather ⅓ → ½ once the scale is at its top (gatherThird ${E.settings.gatherThird}); no scale rung after a divisor rung; volQuarter untouched at high (${E.settings.volQuarter} — the preset never had it)`);
      // E2: back-off on the GPU: the union jumps to 14 ms (84 %) → the newest rung (the gather) is undone within about a window, then a scale rung per window + cooldown; failed rungs are held
      const nUp = log.length; drive(q, E, 1.6, 60, 14.0, log, T);
      const firstUndo = log[nUp];
      check(!!firstUndo && firstUndo.g3 === true && /undid gather/.test(firstUndo.note), `∪ 14 ms: the newest rung goes first and at once — “${firstUndo?.note}” at t = ${firstUndo?.t.toFixed(1)} s`);
      drive(q, E, 12, 60, 14.0, log, T);
      const downs = log.slice(nUp); const failedNow = q.climbState.failed as string[];
      check(downs.length >= 3 && downs.slice(1).every((e, i) => e.scale <= downs[i].scale + 1e-9) && E.settings.renderScale < topWant && failedNow.length >= 3, `held over the band: rungs come off one by one, newest first (${downs.map(e => /undid (\S+)/.exec(e.note)?.[1] ?? '?').join(', ')}), scale now ${E.settings.renderScale.toFixed(2)}, ${failedNow.length} rungs held off`);
      // E3: no oscillation: headroom straight back (8 ms) — the rungs that just failed stay off for their hold; nothing re-climbs inside it past the lowest failed scale; a rung failing AGAIN doubles its hold
      const nDown = log.length; const scaleBefore = E.settings.renderScale; drive(q, E, 40, 60, 8.0, log, T);
      const reclimbs = log.slice(nDown).filter(e => /climb ↑/.test(e.note));
      const lowestFailedScale = Math.min(...failedNow.filter(f => f.startsWith('scale:')).map(f => parseFloat(f.slice(6))));
      check(reclimbs.every(e => e.scale < lowestFailedScale - 1e-6 || !/scale/.test(e.note)) && !reclimbs.some(e => /gather/.test(e.note)), `headroom back for 40 s: no failed rung re-taken inside its hold (re-climbs: ${reclimbs.map(e => /scale/.test(e.note) ? e.scale.toFixed(2) : 'gather').join(', ') || 'none'}; lowest failed scale ${isFinite(lowestFailedScale) ? lowestFailedScale.toFixed(2) : '—'}, from ${scaleBefore.toFixed(2)})`);
      drive(q, E, 40, 60, 8.0, log, T);   // past the hold (60 s since the failures): it climbs again, and the check below is on what it climbed
      const n2 = log.length; drive(q, E, 4.2, 60, 14.0, log, T); const again = log.slice(n2).find(e => /undid/.test(e.note));
      check(!!again && /held 120 s/.test(again.note), `the same rung failing a second time is held twice as long — “${again?.note}”`);
    }
    // E4: fps dips count as no headroom: 50 fps with an idle GPU never climbs; and two windows under 55 with rungs standing drop ALL of them at once (a load spike must not wait a window per rung), the newest held off
    { const { E, q } = mk(); const log: Ev[] = []; const T = { t: 0 };
      drive(q, E, 20, 50, 6.0, log, T);
      check(log.filter(e => /climb ↑/.test(e.note)).length === 0, `50 fps with the GPU idle (∪ 6 ms): no climb (${log.length} changes: ${log.map(e => e.note).join(' | ') || 'none'})`);
      drive(q, E, 50, 60, 6.0, log, T); const up = log.filter(e => /climb ↑/.test(e.note)).length; const standing = (q.climbState.standing as string[]).length;
      const nB = log.length; drive(q, E, 3.2, 50, 6.0, log, T); const dropAll = log.slice(nB).find(e => /all rungs/.test(e.note));
      check(up >= 2 && standing >= 2 && !!dropAll && Math.abs(dropAll.scale - PRESETS.high.scale) < 1e-9 && (q.climbState.standing as string[]).length === 0 && (q.climbState.failed as string[]).length === 1, `then 60 fps: ${up} climbs (${standing} standing, the last well before the dip); then 50 fps: within two windows every rung comes off at once, back to the preset's ×${PRESETS.high.scale} — “${dropAll?.note}” — one rung (the newest) held off`);
      // E4b: a dip that begins RIGHT AFTER a climb is that rung's doing: only the newest comes off
      const { E: E2, q: q2 } = mk(); const log2: Ev[] = []; const T2 = { t: 0 };
      drive(q2, E2, 12, 60, 8.0, log2, T2); const st2 = (q2.climbState.standing as string[]).slice();
      const n2 = log2.length; drive(q2, E2, 3.2, 50, 8.0, log2, T2); const undo = log2.slice(n2).find(e => /undid/.test(e.note));
      check(st2.length >= 2 && !!undo && (q2.climbState.standing as string[]).length === st2.length - 1, `a dip straight after a climb (${st2.length} standing, the last ${'<'} 6 s old): only the newest comes off — “${undo?.note}” — ${(q2.climbState.standing as string[]).length} still standing`);
    }
    // E5: no GPU timings (union 0): never acts, however fast
    { const { E, q } = mk(); const log: Ev[] = []; const T = { t: 0 }; drive(q, E, 30, 60, 0, log, T); check(log.length === 0 && Math.abs(E.settings.renderScale - PRESETS.high.scale) < 1e-9, `no union handed in: nothing moves in 30 s at 60 fps (scale ${E.settings.renderScale.toFixed(2)})`); }
    // E6: the switch off = the ladder as it was: same 60 s of headroom, ladderClimb false → nothing climbs
    { const { E, q } = mk(); E.settings.ladderClimb = false; const log: Ev[] = []; const T = { t: 0 }; drive(q, E, 60, 60, 8.0, log, T); check(log.length === 0, `ladderClimb off: nothing climbs (${log.length} changes)`); }
    // E7: under AUTO at medium with headroom the preset goes up first (the cruising rule), not the scale — AT EXACTLY 60.0 fps: a 60 Hz display's rAF never averages better, and the old
    // 'avg > 62 fps' cruising test could not fire on one (auto ↑ only ever happened on faster panels; a machine calibrated low during a loaded boot stayed low all session)
    { const { E, q } = settledOn('medium');
      const log: Ev[] = []; const T = { t: 0 }; const p0 = q.current; drive(q, E, 20, 60.0, 6.0, log, T);
      check(p0 === 'medium' && q.current === 'high' && !log.some(e => /climb ↑ scale/.test(e.note) && e.t < (log.find(x => /preset|auto ↑/.test(x.note))?.t ?? 1e9)), `auto at medium, 60.0 fps (a 60 Hz panel), ∪ 6 ms: ${p0} → ${q.current} by the cruising rule before any scale climb (${log.map(e => e.note.split(' (')[0]).join(' | ')})`); }
    // E7b (CLOSED LOOP — the union follows what is rendered): a medium machine whose union is 9.5 ms at ×0.67: high is priced ~×1.7 = 16 ms → no hop; the scale climbs a rung or two on the
    // room there is, each rung making the window dearer, and — union × hopRatio being invariant under its own rungs — it never talks itself into the hop; a LOW machine at 9.5 ms likewise
    // (medium ≈ ×2.1: the flat '×2' rule hopped it, fell to ~41 fps, came back and cycled every ~15 s); WITHOUT timings (union 0) the rate alone decides, over twice the windows
    { const { E, q } = settledOn('medium');
      const k = 9.5 / liveUnion(q, E, 1); const log: Ev[] = []; const T = { t: 0 }; driveLive(q, E, 60, 60.0, k, log, T);
      check(q.current === 'medium' && E.settings.renderScale > PRESETS.medium.scale + 1e-6, `closed loop, medium, ∪ 9.5 ms at ×${PRESETS.medium.scale}: after 60 s still ${q.current}, scale ${E.settings.renderScale.toFixed(2)} (∪ now ${liveUnion(q, E, k).toFixed(1)}; high priced ×${q.hopRatio('high').toFixed(2)} = ${(liveUnion(q, E, k) * q.hopRatio('high')).toFixed(1)} ms vs ${(0.9 * PLAN_BUDGET_MS).toFixed(1)}) — ${log.map(e => e.note.split(' (')[0]).join(' | ') || 'no changes'}`);
      const { E: E4, q: q4 } = settledOn('low');
      const k4 = 9.5 / liveUnion(q4, E4, 1); const log4: Ev[] = []; const T4 = { t: 0 }; driveLive(q4, E4, 90, 60.0, k4, log4, T4);
      check(q4.current === 'low', `closed loop, low, ∪ 9.5 ms at ×${PRESETS.low.scale}: after 90 s still ${q4.current}, scale ${E4.settings.renderScale.toFixed(2)} (medium priced ×${q4.hopRatio('medium').toFixed(2)} = ${(liveUnion(q4, E4, k4) * q4.hopRatio('medium')).toFixed(1)} ms) — ${log4.map(e => e.note.split(' (')[0]).join(' | ') || 'no changes'}`);
      const { E: E3, q: q3 } = settledOn('medium');
      const log3: Ev[] = []; const T3 = { t: 0 }; drive(q3, E3, 8, 60.0, 0, log3, T3); const early = q3.current; drive(q3, E3, 10, 60.0, 0, log3, T3);
      check(early === 'medium' && q3.current === 'high', `no GPU timings, 60.0 fps: still medium after 8 s (${early}), high after 18 s (${q3.current}) — the rate alone, twice the windows`);
      // a hop that comes straight back down is held off (doubling) and the hold survives the apply()s; after the hold it is re-tried on REAL room (closed loop: medium's own rungs neither fake room nor block it)
      const { E: E5, q: q5 } = settledOn('medium');
      const k5 = 6.0 / liveUnion(q5, E5, 1); const log5: Ev[] = []; const T5 = { t: 0 }; driveLive(q5, E5, 12, 60.0, k5, log5, T5); const hopped = q5.current;
      for (let s2 = 0; s2 < 24 && q5.current === 'high'; s2++) drive(q5, E5, 1, 36, 14.0, log5, T5);   // (high turns out starved on this machine — a scene the model under-priced)
      const back = q5.current; const heldNote = log5.map(e => e.note).find(n => /auto ↓/.test(n)) ?? '';
      driveLive(q5, E5, 30, 60.0, k5, log5, T5); const during = q5.current;
      driveLive(q5, E5, 60, 60.0, k5, log5, T5);
      check(hopped === 'high' && back === 'medium' && /held off/.test(heldNote) && during === 'medium' && q5.current === 'high', `closed loop: medium ∪ 6 → ${hopped}; starved there → ${back} (“${heldNote.split(' [')[0]}”); ∪ 6-ish at medium for 30 s → still ${during} (held); a minute on → ${q5.current} (re-tried after the hold, over the rungs the climb put up meanwhile)`); }
  }

  // WHAT THE PAGE RECORDS TODAY: a take the recorder (ui/perfrec.ts) makes of the ladder on a machine, through the dump's own JSON, replays to its own moves
  { const realNow = performance.now.bind(performance); let now = 1000; (performance as { now: () => number }).now = () => now;
    const x = mkQuality(w, 'high', true, { ladderClimb: true }, 60, 120), T = x.E.timer; let union = 0, n = 0; x.q.lastChange = '';
    Object.assign(x.E, { lights: { activeCount: 1 }, world: { dynamics: [] }, passes: { reallocs: 0, lastRealloc: '' } });
    Object.assign(T, { lastFrame: new Map([['direct', 1]]), lastRaw: [1], unionMs: () => union, ranThisFrame: () => true });
    const rec = new PerfRecorder(x.E as never, { guards: [] } as never, { target: [0, 0, 0], yaw: 0, pitch: 0, distance: 1 } as never, x.q as never, () => {}); rec.watch = false; rec.start('a take');
    for (let t = 0; t < 120_000;) { const k = at(t < 20_000 ? 11 : t < 80_000 ? 17 : 11); union = +(k * modelCost(stateOf(x.q, x.E), w, S0)).toFixed(2); T.lastFrame.set('direct', union); T.lastFrame.set('other', ++n / 1000); const dt = Math.max(1000 / 120, union + MODEL.overheadMs); now += dt; t += dt; x.E.height = Math.round(rowsOf(stateOf(x.q, x.E), w)); x.E.width = Math.round(x.E.height * w.cssW / w.cssH); rec.frame(dt / 1000, 1000 / dt); x.q.sample(dt / 1000, union); }
    const d = JSON.parse(JSON.stringify(rec.stop(false))) as PerfDump; (performance as { now: () => number }).now = realNow;
    const want = laddersOf(d), b = bestWarm(d);
    check(want.length >= 2 && b.matched === want.length && b.extra === 0, `a take of a fight, ${d.frames.length} frames: its ${want.length} ladder moves reproduced by the replay, ${b.extra} extra (${want.map(m => m.key).join(' | ')})`); }
  if (fails) console.log(`\n${fails} check(s) FAILED`); else console.log('\n=== quality-check: ALL PASS ===');
  return fails === 0;
}

// ---- main --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// ---- main --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
const args = process.argv.slice(2); const flag = (n: string) => args.includes(n); const opt = (n: string) => { const i = args.indexOf(n); return i >= 0 ? args[i + 1] : undefined; };
const VALUED = ['--aim', '--hz', '--hold', '--paced', '--held', '--shed', '--sharpk', '--budget', '--k', '--css', '--dpr'], KNOWN = [...VALUED, '--check', '--timeline', '--sweeps', '--plan'];
for (const a of args) if (a.startsWith('--') && !KNOWN.includes(a)) { console.error(`unknown flag ${a} (known: ${KNOWN.join(' ')})`); process.exit(2); }
if (flag('--plan')) {
  const own: Machine[] = [];
  if (flag('--k')) { const [cssW, cssH] = (opt('--css') ?? '1440x900').split('x').map(Number), dpr = Number(opt('--dpr') ?? 1), k = Number(opt('--k')); if (![k, cssW, cssH, dpr].every(v => isFinite(v) && v > 0)) { console.error('usage: --k 4.5 --css 1440x900 --dpr 2 (all positive numbers)'); process.exit(2); } own.push({ name: `yours (${k}×), ${cssW}×${cssH} @${dpr}`, k, cssW, cssH, dpr }); }
  printPlan(Number(opt('--budget') ?? PLAN_BUDGET_MS), own); process.exit(0);
}
const onOff = (n: string): boolean | undefined => { const p = opt(n); if (p !== undefined && p !== 'on' && p !== 'off') { console.error(`${n} ${p}: on | off`); process.exit(2); } return p === undefined ? undefined : p === 'on'; };
pacedForce = onOff('--paced'); heldForce = onOff('--held'); shedForce = onOff('--shed');
{ const k = opt('--sharpk'); if (k !== undefined) { if (!(+k >= 0.2 && +k <= 10)) { console.error(`--sharpk ${k}: 0.2‥10`); process.exit(2); } MODEL.sharpK = +k; } }
{ const h = opt('--hold'); if (h) for (const kv of h.split(',')) { const [k, v] = kv.split('='); if (!(k in HOLD) || !Number.isFinite(+v)) { console.error(`--hold ${kv}: HOLD has ${Object.keys(HOLD).join(', ')}`); process.exit(2); } (HOLD as Record<string, number>)[k] = +v; } }   // tuning: replay with a HOLD constant moved (--hold restore=0.8,restoreWindows=6)
const files = args.filter((a, i) => !a.startsWith('--') && !VALUED.includes(args[i - 1]));
if (!files.length) process.exit(selftest() ? 0 : 1);
let ok = true;
const aimArg = opt('--aim') ?? 'all'; const aims = aimArg === 'all' ? [0, 60, 90, 120, AIM_DISPLAY] : aimArg.split(',').map(a => { const v = ladderAimOf(a); if (v === null) { console.error(`--aim ${a}: not one of asnow | 0 | 60 | 90 | 120 | display`); process.exit(2); } return v; });
const hzArg = opt('--hz') ? Number(opt('--hz')) : undefined; if (hzArg !== undefined && !(hzArg >= 24 && hzArg <= 480)) { console.error(`--hz ${opt('--hz')}: a display rate, 24‥480`); process.exit(2); }
for (const path of files) {
  const d = loadDump(path); const name = path.split('/').pop()!.replace(/\.json$/, '');
  if (!d.frames.length) { console.log(`\n${name}: no frames`); continue; }
  { const gone = [...new Set(d.frames.map(f => presetOfLabel(f.preset).preset))].filter(p => !(p in PRESETS)); if (gone.length) { console.log(`\n${name}: a take on ${gone.join(', ')}, a tier that is gone: nothing to replay it on`); continue; } }
  if (d.events.some(e => e.kind === 'sweep')) { if (!flag('--sweeps')) { console.log(`\n${name}: a sweep take (the ladder was held still) — nothing to replay (--sweeps replays it anyway)`); continue; } console.log(`\n${name}: A SWEEP TAKE replayed anyway (--sweeps): its rows flip features under the ladder, so read the moves and the rows, not the fps`); }
  if (flag('--check')) ok = checkAsNow(d, name, hzArg) && ok;
  else { const warm = bestWarm(d, hzArg).warm; report(name, d, aims.map(a => replay(d, a, { hz: hzArg, warm })), flag('--timeline')); }   // (every aim from the start state as-now's own replay fits best)
}
process.exit(ok ? 0 : 1);
