// AN EVEN PACE ON A FAST DISPLAY, AND A FRAME RATE READOUT THAT IS TRUE — the check on src/host/pace.ts (EVENPACE) and its place in the page's frame loop.
//   bun run tools/qa/pace-check.ts            (and again with TPS_PROFILE=shipped in front: the switch's two stands)
//   EVENPACE_BREAK=1 bun run tools/qa/pace-check.ts   must FAIL: the pacer passes over a frame that came on time, and the readout is the last frame's own rate
//   GIVEWAY_BREAK=1 bun run tools/qa/pace-check.ts    must FAIL: a window nobody is at draws as a focused one does (the give-way rule off), so it takes the whole GPU
// What it does, with no page and no display: plays frame stamps through the pacer as a display would hand them over (120, 144, 240 and 60 Hz, a wobble on the
// stamps, frames the browser held back, frames in pairs from the first one, a window moved to a slower display and back) and reads which were worked and what
// rate the pacer read off them; plays the gaps of a real recording through the readout; reads the wiring off the page's source as text; stands a game up and
// reads the switch on this stand.
// What it cannot say: that a fast display's picture is even. Nobody here has one, and a headless browser's display runs at 60, where no frame is passed over.
// Giving way (EVENPACE.giveway) is played the same way: a GPU that takes so long over a frame, the queue's word coming back when it is done, the focus going
// and coming back. What it cannot say there: that a second window of a real browser on a real GPU starts beside the first (docs/agents/render.md has what
// two windows of headless Chromium did on the software adapter).
// exit 0 = all pass · 1 = one failed
import { readFileSync } from 'node:fs';
import { ROOT, standUp } from './headless';
import { EVENPACE, pacer, paceEvery, periodOf, rateMeter } from '../../src/host/pace';

const BREAK = process.env.EVENPACE_BREAK === '1', GIVE_BREAK = process.env.GIVEWAY_BREAK === '1';
let failed = 0;
const check = (ok: boolean, what: string) => { if (!ok) failed++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}`); };
const shipped = process.env.TPS_PROFILE === 'shipped';

// 0. the switch on this stand: off on the frozen one and in the module, on where the page's own boot ran (the shipped stand)
{ const log = console.log; console.log = () => {}; try { await standUp({ seed: 1 }); } finally { console.log = log; }
  check(EVENPACE.on === shipped, `the switch on the ${shipped ? 'shipped' : 'frozen'} stand is ${shipped ? 'on' : 'off'} (${EVENPACE.on})`);
  check(EVENPACE.giveway === shipped, `giving way on the ${shipped ? 'shipped' : 'frozen'} stand is ${shipped ? 'on' : 'off'} (${EVENPACE.giveway})`); }

EVENPACE.on = true; EVENPACE.giveway = !GIVE_BREAK;
if (BREAK) { EVENPACE.slack = -0.2; EVENPACE.window = 0; }   // (the break: a frame is 'early' until a fifth of a display frame AFTER it was due, so the one that came on time is passed over; and the readout's window holds one frame)

// 1. how many display frames one drawn frame takes
{ const rows: [number, number, number][] = [[120, 60, 2], [119.88, 60, 2], [144, 60, 2], [165, 60, 2], [180, 60, 3], [240, 60, 4], [240, 120, 2], [60, 60, 1], [59.94, 60, 1], [75, 60, 1], [100, 60, 1], [116, 60, 1], [120, 120, 1], [120, 90, 1], [0, 60, 1], [120, 0, 1]];
  const bad = rows.filter(([hz, aim, n]) => paceEvery(hz, aim) !== n);
  check(!bad.length, `every second frame at 120, 144 and 165 Hz, every third at 180, every fourth at 240; every one at 60 to 116 (halved, that would be under the ladder's cut), at an aim of the display's own rate, with no rate read and with no rate held${bad.length ? ` — wrong: ${bad.map(([hz, aim, n]) => `${hz}/${aim} → ${paceEvery(hz, aim)}, not ${n}`).join('; ')}` : ''}`); }

// a display: stamps a period apart with a wobble, some of them never handed over (the browser holding the page while the GPU works). `hz` may change on the way
// (the window dragged to another display). The first `warm` callbacks are the pacer reading the period: not judged.
let seedv = 12345; const rnd = () => { seedv = (seedv * 1103515245 + 12345) & 0x7fffffff; return seedv / 0x7fffffff; };
interface Played { gaps: number[]; skipped: number; worked: number; run: number; earlyWorked: number; dueSkipped: number; hz: number; every: number; tail: { skipped: number; worked: number } }
function play(hzOf: number | ((i: number) => number), aim: number, n: number, opts: { wobble?: number; held?: (i: number) => boolean; on?: boolean; warm?: number; tail?: number } = {}): Played {
  const P = pacer(), was = EVENPACE.on, warm = opts.warm ?? 12; if (opts.on === false) EVENPACE.on = false;
  const gaps: number[] = [], tail = { skipped: 0, worked: 0 }; let lastWork = -1, run = 0, maxRun = 0, earlyWorked = 0, dueSkipped = 0, t = 1000;
  for (let i = 0; i < n; i++) {
    const hz = typeof hzOf === 'number' ? hzOf : hzOf(i), period = 1000 / hz; t += period;
    if (opts.held?.(i)) continue;
    const ts = t + (rnd() - 0.5) * 2 * (opts.wobble ?? 0), every = paceEvery(hz, aim);
    const skip = P.skip(ts, aim), since = lastWork < 0 ? Infinity : ts - lastWork, judged = i >= warm;
    if (i >= n - (opts.tail ?? 0)) { if (skip) tail.skipped++; else tail.worked++; }
    if (skip) { run++; maxRun = Math.max(maxRun, run); if (judged && since >= (every - 0.25) * period) dueSkipped++; }
    else { run = 0; if (judged && since < (every - 0.75) * period) earlyWorked++; if (judged && lastWork >= 0) gaps.push(since); lastWork = ts; }
  }
  EVENPACE.on = was;
  return { gaps, skipped: P.skipped, worked: P.worked, run: maxRun, earlyWorked, dueSkipped, hz: P.hz, every: P.every, tail };
}
const span = (g: number[]) => `${Math.min(...g).toFixed(1)} to ${Math.max(...g).toFixed(1)} ms`, about = (a: number, b: number, tol = 0.02) => Math.abs(a - b) <= tol * b;

// 2. 120 Hz, the ladder holding 60: the pacer reads the display's rate off the stamps and the worked frames are 16.7 ms apart, every one, with the stamps wobbling
{ const r = play(120, 60, 2400, { wobble: 0.6 });
  check(about(r.hz, 120) && r.gaps.length > 1150 && r.gaps.every(g => Math.abs(g - 1000 / 60) < 1.3) && Math.abs(r.skipped - r.worked) <= 12, `120 Hz and an aim of 60: the display reads ${r.hz.toFixed(1)} Hz, ${r.worked} frames worked ${span(r.gaps)} apart, ${r.skipped} passed over`); }
// 3. 144 and 240 Hz
{ const a = play(144, 60, 2880, { wobble: 0.4 }), b = play(240, 60, 4800, { wobble: 0.3 });
  check(about(a.hz, 144) && about(b.hz, 240) && a.gaps.every(g => Math.abs(g - 2000 / 144) < 0.9) && b.gaps.every(g => Math.abs(g - 1000 / 60) < 0.7) && b.skipped >= 3 * b.worked - 40, `144 Hz (read ${a.hz.toFixed(1)}): worked ${span(a.gaps)} apart, every second frame; 240 Hz (read ${b.hz.toFixed(1)}): ${span(b.gaps)}, every fourth`); }
// 4. nothing is passed over at 60 Hz, at an aim of the display's own rate, with no rate held (the ladder off, the page measuring the machine), or with the switch off
{ const a = play(60, 60, 600, { wobble: 0.6 }), b = play(120, 120, 600, { wobble: 0.6 }), c = play(120, 60, 600, { on: false }), d = play(120, 0, 600);
  check(a.skipped === 0 && b.skipped === 0 && c.skipped === 0 && d.skipped === 0 && about(d.hz, 120), `no frame is passed over at 60 Hz (${a.skipped}), at an aim of 120 on 120 Hz (${b.skipped}), with the switch off (${c.skipped}) or with no rate held (${d.skipped}; the display is still read: ${d.hz.toFixed(1)} Hz)`); }
// 5. a machine that cannot hold the pace: the browser hands frames over late and unevenly. No frame that came on time or late is passed over, no early one is worked,
//    and never two in a row passed over at every-second
{ const held = (i: number) => { const k = i % 23; return k === 3 || k === 4 || k === 5 || k === 11 || k === 16 || k === 17; };
  const r = play(120, 60, 4600, { wobble: 0.5, held, warm: 30 });
  check(about(r.hz, 120) && r.dueSkipped === 0 && r.earlyWorked === 0 && r.run <= 1 && r.gaps.every(g => g >= 1000 / 60 - 1.3), `frames held back by the browser: none that was due is passed over (${r.dueSkipped}), none is worked early (${r.earlyWorked}), never two passed over in a row (${r.run}), no gap under a 60th (${span(r.gaps)})`); }
// 6. the worked example: after a late frame the beat moves, the wait is not doubled
{ const P = pacer(), p = 1000 / 120; for (let k = 0; k <= 10; k++) P.skip(1000 + k * p, 60);
  const got = [11, 12, 13, 16, 17, 18, 19, 20].map(k => P.skip(1000 + k * p, 60) ? 's' : 'W').join('');
  check(got === 'WsWWsWsW', `display frames 11 12 13 · 16 17 18 19 20: ${got} (worked, passed, worked · worked at once though late, then every second one)`); }
// 7. THE FAULT ITSELF, from the first frame: a 120 Hz display that hands frames over in pairs (two, then two held back). The short gap of each pair shows the
//    period, so the pace is taken up; and a page slow on EVERY frame (one handed over, one held, always) reads a 60 Hz display and is given every frame, as it was
{ const pairs = play(120, 60, 2400, { held: i => i % 4 >= 2, warm: 40 }), slow = play(120, 60, 1200, { held: i => i % 2 === 1 });
  check(about(pairs.hz, 120) && pairs.every === 2 && pairs.skipped > 0.45 * (pairs.skipped + pairs.worked) && pairs.earlyWorked === 0, `frames in pairs from the first one: the display reads ${pairs.hz.toFixed(1)} Hz and the second of each pair is passed over (${pairs.skipped} of ${pairs.skipped + pairs.worked}; worked ${span(pairs.gaps)} apart)`);
  check(about(slow.hz, 60) && slow.skipped === 0, `a page handed every second frame of a 120 Hz display, always: it reads ${slow.hz.toFixed(1)} Hz and no frame is passed over (${slow.skipped})`); }
// 8. the window dragged to a slower display, and back: the period is read again off the frames themselves, within half a second each way
{ const down = play(i => i < 600 ? 120 : 100, 60, 1200, { wobble: 0.3, tail: 540 }), up = play(i => i < 600 ? 100 : 120, 60, 1200, { wobble: 0.3, tail: 540 });
  check(about(down.hz, 100) && down.tail.skipped === 0 && about(up.hz, 120) && Math.abs(up.tail.skipped - up.tail.worked) <= 1, `120 Hz then 100: it reads ${down.hz.toFixed(1)} Hz and passes nothing over from half a second on (${down.tail.skipped} of ${down.tail.skipped + down.tail.worked}); 100 Hz then 120: reads ${up.hz.toFixed(1)}, every second frame (${up.tail.skipped} of ${up.tail.skipped + up.tail.worked})`);
  const odd = periodOf([8.3, 25.1, 8.4, 24.9, 8.2, 33.4, 8.35, 16.7]), none = periodOf([8, 16, 31]);
  check(about(odd, 1000 / 120, 0.01) && none !== none, `the period off gaps of one, two, three and four frames: ${odd.toFixed(2)} ms; off three gaps that do not agree: none`); }

// 9. the readout on a real recording's gaps (frames in pairs: 54 a second drawn), on an even 60, on a page at two frames a second, and across a hitch
{ const gapsOf = [3, 30, 3, 29, 4, 27, 6, 44, 5, 33], real = 1000 * gapsOf.length / gapsOf.reduce((a, b) => a + b, 0);
  const M = rateMeter(); let t = 0, shown = 0, old = 60, lo = Infinity, hi = 0;
  for (let i = 0; i < 600; i++) { const dt = gapsOf[i % gapsOf.length]; t += dt; shown = M.push(t); old = old * 0.95 + (1000 / dt) * 0.05; if (i > 100) { lo = Math.min(lo, shown); hi = Math.max(hi, shown); } }
  check(lo > 0.9 * real && hi < 1.1 * real, `frames in pairs, ${real.toFixed(0)} a second drawn: the readout stays between ${lo.toFixed(0)} and ${hi.toFixed(0)} (it said ${old.toFixed(0)})`);
  const E = rateMeter(); let te = 0, ev = 0; for (let i = 0; i < 300; i++) { te += 1000 / 60; ev = E.push(te); }
  te += 300; const hit = E.push(te); let back = hit; for (let i = 0; i < 66; i++) { te += 1000 / 60; back = E.push(te); }
  const S = rateMeter(); let ts2 = 0, two = 0; for (let i = 0; i < 12; i++) { ts2 += 500; two = S.push(ts2); }
  check(about(ev, 60, 0.005) && hit > 40 && hit < 46 && about(back, 60, 0.005) && about(two, 2, 0.01), `an even 60 reads ${ev.toFixed(1)}; a 300 ms hitch reads ${hit.toFixed(0)} (the frames of that second) and ${back.toFixed(1)} a second on; two frames a second read ${two.toFixed(1)}`); }

// 10. the wiring, read off the source: where the pass-over sits in the frame, what holds it off, the address word, the benchmark and a tape's player
{ const main = readFileSync(`${ROOT}/src/main.ts`, 'utf8'), fr = main.slice(main.indexOf('  function frame(ts?: number) {'));
  const at = fr.indexOf('pace.skip(ts,'), now = fr.indexOf('const now = performance.now(); const rawDt'), line = at >= 0 ? fr.slice(fr.lastIndexOf('\n', at), fr.indexOf('\n', at)) : '';
  check(at > 0 && now > at && /if \(pace\.skip\(ts, !kitView && /.test(line) && /requestAnimationFrame\(frame\); return;/.test(line), "the frame loop passes a display frame over before it takes the time or steps anything, and holds no rate of its own under ?kitview");
  check(/const measuring = quality\.calibrating \|\| bench\.active \|\| perfsweep\.active,/.test(fr) && /!kitView && !measuring && \(quality\.adaptive \|\| quality\.auto\) \? quality\.bands\.target : 0/.test(line), "the rate asked for is the ladder's, and none while the boot measures the machine, in a benchmark, in a sweep, or with the ladder off");
  check(/fpsSm = rate\.push\(now\);/.test(fr) && !/fpsSm \* 0\.95/.test(main), 'the frame rate shown is the frames of the last second over their time, and the mean of rates is gone');
  const prof = readFileSync(`${ROOT}/src/host/profile.ts`, 'utf8'), run = readFileSync(`${ROOT}/src/ui/benchrun.ts`, 'utf8'), key = readFileSync(`${ROOT}/src/ui/replaykey.ts`, 'utf8');
  check(/EVENPACE\.on = q\.get\('pace'\) !== '0';/.test(prof) && /'EVENPACE\.on': 'TPS_EVENPACE=1/.test(prof), "on in the page, ?pace=0 puts it off, and the stands' book has its row");
  check(/const CARRIED = new RegExp\(`\^\([^`]*\|pace\|/.test(run) && /VISIT_FLAGS = \[[^\]]*'pace'/.test(key), "a benchmark's scene pages keep the visitor's ?pace word (no note says it was left behind), and a tape's player leaves the word to the viewer"); }

// ---- A WINDOW NOBODY IS AT GIVES WAY ----
// a display and a GPU: stamps a period apart; a worked frame hands its work over `cpu` ms after its stamp, the GPU takes `gpu` ms over it once it is free of
// this window's last frame and of `ahead` ms of ANOTHER window's work queued in front, and the queue's word that a frame worked while away is done reaches the
// pacer before the first callback after it, with the GPU's own timing of the frame (`timed`) or without. `away(i)`: nobody is at the window at callback i.
// `late(i)`: callback i never comes (a busy page), so the one after it is a stall later.
interface Gave { gaps: number[]; share: number; overlap: number; gave: number; worked: number; backSkips: number }
function playAway(hz: number, aim: number, n: number, gpu: number, away: (i: number) => boolean, o: { cpu?: number; ahead?: number; timed?: boolean; late?: (i: number) => boolean } = {}): Gave {
  const P = pacer(), period = 1000 / hz, gaps: number[] = [], cpu = o.cpu ?? 2, ahead = o.ahead ?? 0; let t = 1000, free = 0, flying = false, doneAt = 0, busy = 0, span = 0, overlap = 0, lastWork = -1, backSkips = 0;
  for (let i = 0; i < n; i++) {
    t += period; if (o.late?.(i)) continue;
    if (flying && doneAt <= t) { P.done(doneAt, o.timed === false ? 0 : gpu); flying = false; }
    const a = away(i), judged = i >= 60 && a && away(i - 60);   // (a second into the away spell: the pacer has its measure)
    const before = P.gave, skip = P.skip(t, aim, a);
    if (judged) span += t - (lastT < 0 ? t : lastT); lastT = t;
    if (!a && P.gave !== before) backSkips++;
    if (skip) continue;
    if (a && free > t) overlap++;   // (begun while the GPU still had this window's last frame)
    const start = Math.max(t + cpu, free) + ahead; free = start + gpu; if (judged) busy += gpu;
    if (a) { P.sent(t + cpu); flying = true; doneAt = free; }
    if (judged && lastWork >= 0) gaps.push(t - lastWork); lastWork = t;
  }
  lastT = -1;
  return { gaps, share: span ? busy / span : 0, overlap, gave: P.gave, worked: P.worked, backSkips };
}
let lastT = -1;
const mean = (g: number[]) => g.reduce((a, b) => a + b, 0) / Math.max(1, g.length);
// 11. the window somebody is at is never held by it: the GPU twice over its frame, frames piling up, and not one display frame is passed over for giving way
{ const r = playAway(60, 60, 1200, 30, () => false), q = playAway(120, 60, 2400, 11, () => false);
  check(r.gave === 0 && q.gave === 0, `a window somebody is at, the GPU at 30 ms a frame on 60 Hz and at 11 ms on 120 Hz: no frame is passed over for giving way (${r.gave}, ${q.gave})`); }
// 12. nobody at it, the work filling two thirds of the frame (where the ladder holds a machine): about every second frame, one with the GPU at a time, the GPU left half free
{ const r = playAway(60, 60, 2400, 11, () => true), q = playAway(120, 60, 4800, 11, () => true);
  check(r.overlap === 0 && r.share <= 0.5 && about(mean(r.gaps), 2000 / 60, 0.03) && r.gave > 0, `nobody at it, the GPU 11 ms a frame on 60 Hz: a frame every ${mean(r.gaps).toFixed(1)} ms, never begun with the last still on the GPU (${r.overlap}), the GPU's time taken ${(r.share * 100).toFixed(0)} %`);
  check(q.overlap === 0 && q.share <= 0.5 && about(mean(q.gaps), 3000 / 120, 0.03), `…and on 120 Hz with the ladder's 60 held: a frame every ${mean(q.gaps).toFixed(1)} ms, ${(q.share * 100).toFixed(0)} % of the GPU's time`); }
// 13. …work up to the longest wait's worth: still no more than half. Past it (a machine far under the rate) the wait stops at a twentieth of a second, so the
//     window's clock runs true and its picture moves, and it is still one frame with the GPU at a time
{ const r = playAway(120, 60, 4800, 25, () => true), q = playAway(60, 60, 3600, 40, () => true), w = playAway(60, 60, 3600, 70, () => true);
  check(r.overlap === 0 && r.share <= 0.5 && r.share > 0.4, `the GPU 25 ms a frame on 120 Hz: a frame every ${mean(r.gaps).toFixed(1)} ms, ${(r.share * 100).toFixed(0)} % of its time, never two with the GPU (${r.overlap})`);
  check(q.overlap === 0 && q.gaps.every(g => g <= 50.1) && w.overlap === 0 && w.gaps.every(g => g > 70 && g <= 83.4), `40 ms a frame on 60 Hz: a frame every ${mean(q.gaps).toFixed(1)} ms, never over a twentieth of a second apart, never two with the GPU (${q.overlap}); 70 ms a frame: the next is begun at the first display frame after the last is done (${mean(w.gaps).toFixed(1)} ms apart, ${w.overlap} begun early)`); }
// 14. …and with room to spare it does not notice: work that takes under half its frame is under the share already
{ const r = playAway(60, 60, 1200, 5, () => true);
  check(r.gave === 0 && r.gaps.every(g => Math.abs(g - 1000 / 60) < 0.01), `nobody at it, the GPU 5 ms a frame on 60 Hz: every frame is drawn, none passed over (${r.gave})`); }
// 15. the focus comes back: from that display frame on nothing is passed over for it, though the last away frame is still with the GPU
{ const r = playAway(60, 60, 1200, 40, i => i < 600);
  const P = pacer(); for (let k = 0; k < 30; k++) { if (!P.skip(1000 + k * 16.7, 60, true)) P.sent(1000 + k * 16.7 + 2); }
  const held = P.flying && P.skip(1000 + 30 * 16.7, 60, true), back = P.skip(1000 + 31 * 16.7, 60, false);
  check(r.backSkips === 0 && held && !back, `the focus back after ten seconds away: no frame is passed over from then on (${r.backSkips}); a frame held while the last is with the GPU (${held}) is worked the moment somebody is at the window (${!back})`); }
// 16. the measure: the GPU's own timing of the frame where there is one, never more than the queue's answer took; the answer alone where there is none
{ const P = pacer(); P.sent(10); P.done(45, 8); const own = P.gpuMs; P.sent(100); P.done(104, 8); const quick = P.gpuMs; P.sent(200); P.done(225); const bare = P.gpuMs; P.done(999, 1);
  check(own === 8 && quick === 4 && bare === 25 && P.gpuMs === 25 && !P.flying, `an answer after 35 ms with the passes timed at 8: ${own} ms; an answer after 4 ms: ${quick}; no timing: ${bare}; an answer nobody waits for changes nothing (${P.gpuMs})`); }
// 17. beside a BUSY window: 16 ms of the other window's work goes in front of each of this one's frames. By the GPU's own timing this window's 5 ms is still
//     under its share and it draws as often as its turn comes; by the queue's answer alone it would count the other window's work as its own
{ const r = playAway(60, 60, 2400, 5, () => true, { ahead: 16 }), bare = playAway(60, 60, 2400, 5, () => true, { ahead: 16, timed: false });
  check(r.overlap === 0 && about(mean(r.gaps), 2000 / 60, 0.03) && mean(bare.gaps) > mean(r.gaps) + 10, `5 ms of its own work behind 16 ms of another window's, on 60 Hz: a frame every ${mean(r.gaps).toFixed(1)} ms, as its turn comes (${r.overlap} begun early); with no timings, every ${mean(bare.gaps).toFixed(1)} ms`); }
// 18. a busy page: one callback in sixty never comes, so the next is a stall late (the even pace forgets its beat there). The wait is not forgotten
{ const calm = playAway(60, 60, 3600, 2000, () => true), r = playAway(60, 60, 3600, 2000, () => true, { late: i => i % 60 < 18 });
  const P = pacer(); P.skip(1000, 60, true); P.sent(1002); P.done(1012, 10); P.skip(1300, 60, false); const soon = P.skip(1310, 60, true);
  check(calm.overlap === 0 && r.overlap === 0 && about(mean(r.gaps), mean(calm.gaps), 0.2), `the GPU 2 s a frame, with a third of a second of every second lost to the page: a frame every ${mean(r.gaps).toFixed(0)} ms (${mean(calm.gaps).toFixed(0)} with none lost), never two with the GPU (${r.overlap})`);
  check(soon, `a frame begun 10 ms ago by a window that has just lost the focus: the next waits its 20 ms (${soon})`); }
// 19. a long answer once (a compile, a save: 3 s) does not leave the window dark: the next frame is begun within a twentieth of a second of the answer
{ const P = pacer(); let t = 1000; for (let k = 0; k < 12; k++) { t += 1000 / 60; P.skip(t, 60, false); }
  P.skip(t += 1000 / 60, 60, true); P.sent(t + 2); const sentT = t; let waited = 0; t += 3000; P.done(t - 1);
  for (let k = 0; k < 12; k++) { t += 1000 / 60; if (!P.skip(t, 60, true)) break; waited++; }
  check(P.gpuMs > 2990 && waited === 0 && t - sentT > 3000, `an answer that took ${Math.round(P.gpuMs)} ms: the next display frame is worked (${waited} passed over after the answer)`); }
// 20. the wiring, read off the source
{ const main = readFileSync(`${ROOT}/src/main.ts`, 'utf8'), fr = main.slice(main.indexOf('  function frame(ts?: number) {')), prof = readFileSync(`${ROOT}/src/host/profile.ts`, 'utf8');
  const aw = fr.indexOf('away = EVENPACE.giveway'), awLine = aw >= 0 ? fr.slice(fr.lastIndexOf('\n', aw), fr.indexOf('\n', aw)) : '', sk = fr.indexOf('pace.skip(ts,'), st = fr.indexOf('      step(dt);'), se = fr.indexOf('pace.sent(');
  check(aw > 0 && aw < sk && /const measuring = quality\.calibrating \|\| bench\.active \|\| perfsweep\.active, away = EVENPACE\.giveway && !measuring && !perfrec\.recording && !attended\(\);/.test(awLine) && /quality\.bands\.target : 0, away\)\)/.test(fr), 'the frame loop asks who is at the window before it passes a frame over, and never gives way while the page measures the machine or records a take');
  check(st > 0 && se > st && /if \(away\) \{ pace\.sent\(performance\.now\(\)\);/.test(fr) && /pace\.done\(performance\.now\(\), engine\.timer\.enabled \? engine\.timer\.unionMs\(GPU_RENDER_PASSES\) : 0\)/.test(fr) && /onSubmittedWorkDone\(\)\.then\(end, end\)/.test(fr), "a frame worked while away tells the pacer when its work went to the GPU and when the queue says it is done, with the GPU's own timing of a frame, and only such a frame asks the queue");
  check(/const rested = pace\.gave !== gaveLast; gaveLast = pace\.gave;/.test(fr) && /&& !rested && rawDt < 0\.5/.test(fr) && /!document\.hidden && !rested, endStage/.test(fr), "the quality ladder and the visit's frame records do not read a frame whose gap held a display frame passed over for giving way, and read every other frame of an unfocused window");
  check(/if \(topAsks\) try \{ return \(window\.top \?\? window\)\.document\.hasFocus\(\); \} catch \{ topAsks = false; \} return true; \};/.test(main) && /const t = window\.__testFocus; if \(typeof t === 'boolean'\) return t;/.test(main), "who is at the window is the top window's own focus (the game's page is a frame of the kept page); a top that cannot be asked counts as somebody and is asked once");
  check(/EVENPACE\.giveway = q\.get\('giveway'\) !== '0';/.test(prof) && /'EVENPACE\.giveway': 'TPS_GIVEWAY=1/.test(prof), "on in the page, ?giveway=0 puts it off, and the stands' book has its row"); }

console.log(failed ? `=== pace-check: FAILED (${failed}) ===` : '=== pace-check: ALL PASS ===');
process.exit(failed ? 1 : 0);
