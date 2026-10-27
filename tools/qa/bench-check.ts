// The benchmark's own stepping held to a fake page (src/ui/bench.ts): nothing while unasked; the opening picture, the scene live, its heavy moment held still under
// each preset, the sweep inside the same take, the scene's tail; everything put back at the end and when stopped; a slow machine cut short; the text of the report;
// a run over a set of scenes, one page each (the lines handed from page to page: src/ui/benchrun.ts on a fake tab), and the scenes' own table.
// No browser, no GPU.   bun run tools/qa/bench-check.ts   (exit 0 = all pass; BENCH_BREAK=1 leaves a preset on at the end and must fail)
import { Bench, benchReport, benchSaid, benchNotice, type BenchWent, BENCH_FRAMES, type BenchResult, type BenchScene, type BenchSweep, type BenchLeg } from '../../src/ui/bench';
import { SHADER_ROW_WORDS } from '../../src/ui/shaderrows';
import { SAVED, type Fate, type SavedFile } from '../../src/host/savefile';
import { AUTO_LADDER } from '../../src/ui/quality';
import { readFileSync } from 'node:fs';
const BREAK = (globalThis as any).process?.env?.BENCH_BREAK === '1';
let now = 0; (performance as any).now = () => now;
let red = 0; const check = (ok: boolean, w: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };

/** a frame's cost on the fake machine, ms: the preset's own, less what the sweep's flipped row gives back, times how slow the machine is */
const COST: Record<string, number> = { verylow: 6, low: 9, medium: 14, high: 22, ultra: 40 };
function page(o: { slow?: number; scene?: 'none' | 'short' | 'full' | 'calm'; timers?: boolean; awayDep?: boolean; settled?: boolean; hz?: number; rows?: string[] } = {}) {
  const slow = o.slow ?? 1;
  const S: any = { renderScale: 0.8, sharpDirect: true, rtReflect: true, rcPeriods: [1, 1, 2, 4], exposure: 1.5 };
  const applied: string[] = [];
  const Q: any = { current: 'medium', auto: true, calibrating: false, climbed: ['a rung'], failed: new Map([['x', 1]]), failCount: new Map([['x', 2]]),
    get label() { return this.auto ? `auto (${this.current})` : this.current; },
    apply(n: string) { applied.push(n); this.current = n; S.renderScale = n === 'verylow' ? 0.5 : n === 'low' ? 0.6 : n === 'medium' ? 0.75 : 1; S.sharpDirect = n === 'high'; S.rcPeriods = n === 'high' ? [1, 1, 1, 2] : [1, 1, 2, 4]; this.climbed = []; this.failed.clear(); this.failCount.clear(); } };
  const frames: any[] = [], events: { kind: string; text: string }[] = []; let dumps = 0, asked = 0, covered: string | null = null; const went: BenchWent[] = [], updates: BenchWent[] = [];
  const rec: any = { recording: false, lastSent: false, lastLocal: false, cap: 36000, commit: 'abcdef0123456789', get frameCount() { return frames.length; }, get framesLeft() { return this.cap - frames.length; }, frameAt: (f: number) => frames[f],
    start(_note: string, n: number) { this.recording = true; frames.length = 0; asked = n; }, stop(deliver = true) { this.recording = false; if (deliver) dumps++; return { commit: this.commit, frames: [...frames], events }; }, event(kind: string, text: string) { events.push({ kind, text }); } };
  const smoke = { params: { rbIters: 8 } };
  let sceneT = -1, sceneStops = 0; const LEN = o.scene === 'short' ? 3000 : 20000;
  const scene: BenchScene | null = o.scene === 'none' ? { start: () => 'units are on the floor', active: () => false, stop() {}, heavy: () => false }
    : { start: () => { sceneT = 0; return null; }, active: () => sceneT >= 0 && sceneT < LEN, stop() { sceneStops++; sceneT = -1; }, heavy: () => o.scene !== 'calm' && sceneT >= 5000, said: () => `over at ${Math.round(Math.min(Math.max(sceneT, 0), LEN) / 1000)} s` };
  const SEQ = ['base', 'reflect', 'base', 'sharp', 'base']; let sw = -1, swFrames = 0, swStops = 0, swOpts: any = null;
  const sweep: BenchSweep = { start(op) { swOpts = op; sw = 0; swFrames = 0; return 'sweeping'; }, active: () => sw >= 0, flipped: () => sw >= 0 ? SEQ[sw] : '', ...(o.settled ? { settled: () => sw >= 0 && swFrames >= 6 } : {}), progress: () => sw >= 0 ? `${sw + 1}/${SEQ.length}` : 'done', stop() { swStops++; sw = -1; } };
  const shown: string[] = []; let veil: string | null = null, closeCard: (() => void) | null = null;
  const E: any = { settings: S, width: 1280, height: 720, timer: { enabled: o.timers !== false }, gpu: { adapterInfo: 'fake metal-3' } };
  let hidden = false, live = 0;
  const bench = new Bench(E, Q, rec, { smoke, scene, sweep, ...(o.awayDep ? { away: () => { const a = hidden; hidden = false; return a; } } : {}), view: { progress: t => { veil = t; }, update: w => { updates.push({ ...w }); }, show: (r, close, w) => { shown.push(r); went.push(w ?? { result: false, take: null }); closeCard = close; } }, covered: () => covered, ...(o.hz ? { hz: () => o.hz! } : {}), ...(o.rows ? { rows: () => o.rows! } : {}),
    boot: () => ({ marks: { interactive: 12400 }, spans: { beforeScript: 300, pipelines: 9100, firstFrameCpu: 400, firstFrameGpu: 1700 }, pipelines: { total: 212, done: 212, failed: 0 }, shaderText: { modules: 60, before: 12_000_000, after: 2_900_000, ms: 40 }, slowest: [{ label: 'composite', ms: 8800 }], longTasks: [], maxLongTask: 5100, maxRafGap: 6200, rafGapAt: 0, userAgent: 'x' }),
    machine: () => ({ now: { webgpu: 'fake metal-3', webgl: 'ANGLE (Fake, ANGLE Metal Renderer: Fake M9 Pro, Unspecified Version)', browser: 'Chrome 154', os: 'macOS 15.1.0 arm', userAgent: 'x', dpr: 2, threads: 10, memoryGb: 8, screen: [1728, 1117], view: [1512, 860], ...(o.hz ? {} : { hz: 120 }), power: 'on battery (67 %)' }, more: Promise.resolve() }) });
  const states: string[] = [], frozenIn = new Set<string>(), liveIn = new Set<string>();
  /** one presented frame: the world steps unless held, the recorder takes the frame, the sweep and the benchmark tick */
  const frame = (ms?: number) => {
    const giveBack = sw >= 0 ? (SEQ[sw] === 'reflect' ? 2 : SEQ[sw] === 'sharp' ? -1 : 0) : 0;
    const cost = ms ?? (COST[Q.current] - giveBack) * slow;
    now += cost;
    if (bench.active) (bench.freezing ? frozenIn : liveIn).add(bench.state);
    if (bench.active && !bench.freezing && rec.recording) live++;
    if (!bench.freezing && sceneT >= 0) sceneT += cost;
    if (rec.recording) frames.push({ t: now, dt: cost, fps: 1000 / cost, cpu: { update: 1, render: 1, other: 0.5 }, gpu: { direct: cost * 0.5, gather: cost * 0.3 }, gpuSum: cost * 0.8, ...(E.timer.enabled ? { gpuUnion: +(cost * 0.9).toFixed(3) } : {}), res: [E.width, E.height], eff: S.renderScale, preset: Q.label, adaptive: true, cam: [0, 0, 0, 0, 0], n: {} });
    if (sw >= 0 && ++swFrames >= 12) { swFrames = 0; if (++sw >= SEQ.length) sw = -1; }
    bench.tick(cost / 1000);
    if (states[states.length - 1] !== bench.state) states.push(bench.state);
  };
  const run = (cap = 200000) => { for (let i = 0; i < cap && bench.active; i++) frame(); };
  return { get closeCard() { return closeCard; }, cover(why: string | null) { covered = why; }, went, updates, get asked() { return asked; }, hide() { hidden = true; }, get live() { return live; }, frames, S, Q, rec, bench, frame, run, applied, events, shown, states, frozenIn, liveIn, smoke, get veil() { return veil; }, get dumps() { return dumps; }, get sceneStops() { return sceneStops; }, get swStops() { return swStops; }, get swOpts() { return swOpts; } };
}
const before = (p: ReturnType<typeof page>) => JSON.stringify([p.S, p.Q.current, p.Q.auto, p.Q.climbed, [...p.Q.failed], [...p.Q.failCount], p.smoke.params.rbIters]);

{ // unasked: nothing
  const p = page(), was = before(p);
  for (let i = 0; i < 200; i++) p.frame();
  check(!p.rec.recording && !p.applied.length && before(p) === was && p.veil === null && !p.bench.active && !p.bench.freezing, 'unasked: no recording, no preset applied, no setting written, nothing on the screen');
}
{ // a whole run
  const p = page(), was = before(p);
  console.log(p.bench.start());
  check(p.bench.active && p.bench.freezing && !p.rec.recording, 'started: the world is held, and nothing is recorded while the page settles');
  if (BREAK) { const real = p.Q.apply.bind(p.Q); let n = 0; p.Q.apply = (name: string) => { if (++n <= AUTO_LADDER.length) real(name); }; }   // (the apply that puts the page's own preset back is lost)
  p.run();
  check(!p.bench.active && p.bench.last === 'done', `ran to its end by itself (${p.bench.last})`);
  check(p.states.join(' ') === 'waiting opened scene held sweep tail idle' && p.bench.acts.join(' ') === p.states.join(' '), `the acts in order, and the run's own list of them agrees: ${p.states.join(' ')}`);
  check(['waiting', 'opened', 'held', 'sweep'].every(s => p.frozenIn.has(s)) && p.liveIn.has('scene') && p.liveIn.has('tail') && p.liveIn.size === 2, 'the world is held for the windows and the sweep, and runs for the scene');
  const R = p.bench.result as BenchResult, ids = R.steps.map(s => s.id).join(' ');
  check(ids === 'opened scene held verylow low medium high', `a line per window: ${ids}`);
  check(R.rows === undefined && !/^shaders/m.test(p.bench.report), 'a page that says nothing of its shader rows: the report has no line of them');
  { const sc = R.steps[1]; check(sc.frames === p.live && sc.quality === 'auto (medium)' && sc.scale === 0.8 && R.steps[3].scale === 0.5 && R.steps[3].quality === 'verylow', `the scene's line is every live frame (${sc.frames}), and each line says what its own frames were drawn at`); }
  const ms = (id: string) => R.steps.find(s => s.id === id)!.ms;
  check(ms('opened') === 14 && ms('held') === 14 && ms('verylow') === 6 && ms('low') === 9 && ms('medium') === 14 && ms('high') === 22, 'each window carries its own preset\'s frame time');
  check(p.applied.join(' ') === 'verylow low medium high medium', `the presets from the bottom up, then the page's own again: ${p.applied.join(' ')}`);
  check(before(p) === was, 'everything is back as it stood: the settings, auto, the ladder\'s rungs, the smoke row');
  const F = Object.fromEntries((R.features ?? []).map(f => [f.id, f]));
  check(!!F.reflect && Math.abs(F.reflect.ms - 2) < 0.01 && Math.abs((F.reflect.gpu ?? 0) - 1.8) < 0.01 && Math.abs(F.sharp.ms + 1) < 0.01 && F.reflect.label === 'reflections off', `each feature against the baselines beside it: reflections give back ${F.reflect?.ms}, sharp ${F.sharp?.ms}`);
  check(p.swOpts?.inside === true && p.swOpts.rounds === 1 && p.asked === BENCH_FRAMES && BENCH_FRAMES >= 6 * 60 * 240, 'the sweep runs inside the take: one round, the recording left running, and the take has room for six minutes on a fast display');
  check(p.dumps === 1 && p.events.filter(e => e.kind === 'bench').length === 1 && JSON.parse(p.events.find(e => e.kind === 'bench')!.text).steps.length === 7, 'the take is stopped once, with the report in it as an event');
  const T = p.shown[0] ?? '', lines = T.split('\n');
  console.log(T.split('\n').map(l => '    | ' + l).join('\n'));
  check(p.shown.length === 1 && p.veil === null && T === p.bench.report && p.went[0].take === false && p.went[0].result === false && !p.went[0].local, 'the card shows the report and is told the take and the result went nowhere (nobody listens here), the veil is gone');
  check(/Fake M9 Pro/.test(T) && /WebGPU says: fake metal-3/.test(T) && /Chrome 154 · macOS 15\.1\.0 arm/.test(T) && /on battery/.test(T) && /build abcdef01/.test(T), 'the report names the GPU, the browser, the system, the power and the build');
  check(/load\s+12\.4 s to the start card · shaders 9\.1 s \(212 of 212\) · longest freeze 6\.2 s · first frame 2\.1 s/.test(T), 'the report carries the boot: the start card, the shaders, the longest freeze, the first frame');
  check(lines.length <= 40 && lines.every(l => l.length <= 150) && /reflections off/.test(T), `the report is short enough to paste (${lines.length} lines, longest ${Math.max(...lines.map(l => l.length))})`);
}
{ // a slow machine: the ladder is cut short and the features are not flipped
  const p = page({ slow: 20 }), was = before(p);
  p.bench.start(); p.run();
  const R = p.bench.result!, by = Object.fromEntries(R.steps.map(s => [s.id, s]));
  check(p.bench.last === 'done' && !by.verylow.skipped && !by.low.skipped && /low ran at/.test(by.medium.skipped ?? '') && /low ran at/.test(by.high.skipped ?? '') && p.applied.join(' ') === 'verylow low medium', `under eight frames a second ends the ladder: ${by.high.skipped}`);
  check(p.swOpts === null && !R.features && R.notes.some(n => /not timed one by one/.test(n)), 'a held picture under four frames a second: the features are left out, and the report says so');
  check(before(p) === was && p.applied[p.applied.length - 1] === 'medium', 'everything back');
}
{ // stopped while a preset stands
  const p = page(), was = before(p);
  p.bench.start(); for (let i = 0; i < 100000 && p.Q.current !== 'low'; i++) p.frame();
  check(p.bench.state === 'held' && p.Q.current === 'low' && p.Q.auto === false, 'mid-run: a preset of the benchmark\'s stands, auto off');
  p.bench.stop('Esc');
  check(!p.bench.active && before(p) === was && p.sceneStops === 1 && p.bench.last === 'stopped: Esc', 'stopped: everything back, the scene ended');
  check(p.shown.length === 1 && /stopped early \(Esc\)/.test(p.shown[0]) && p.dumps === 1, 'what was timed is still shown and kept');
}
{ // stopped during the sweep
  const p = page(), was = before(p);
  p.bench.start(); for (let i = 0; i < 100000 && p.bench.state !== 'sweep'; i++) p.frame(); for (let i = 0; i < 15; i++) p.frame();
  p.bench.stop('Esc');
  check(p.swStops === 1 && !p.bench.sweeping && before(p) === was && !p.bench.result!.features, 'stopped in the sweep: the sweep is told, everything back; no row was through yet, none is priced');
}
{ // stopped later in the sweep: the rows it got through are priced
  const p = page();
  p.bench.start(); for (let i = 0; i < 100000 && p.bench.state !== 'sweep'; i++) p.frame(); for (let i = 0; i < 30; i++) p.frame();
  p.bench.stop('Esc');
  const F = p.bench.result!.features ?? [];
  check(F.length === 1 && F[0].id === 'reflect' && Math.abs(F[0].ms - 2) < 0.01, `stopped two rows into the sweep: the row that was through is priced (${F.map(f => f.id).join(' ') || 'none'})`);
}
{ // a sweep that says when a segment has settled: its frames from there are the ones priced
  const p = page({ settled: true });
  p.bench.start(); p.run();
  const F = Object.fromEntries((p.bench.result!.features ?? []).map(f => [f.id, f]));
  check(!!F.reflect && Math.abs(F.reflect.ms - 2) < 0.01 && Math.abs(F.sharp.ms + 1) < 0.01, 'the sweep\'s own word for settled is taken: the same prices');
}
{ // the display's rate is not known when a run starts with the page: it is asked again at the end
  const p = page({ hz: 60 });
  p.bench.start(); p.run();
  check(p.bench.result!.machine.hz === 60 && / 60 Hz/.test(p.bench.report), 'the display\'s rate is in the report though the page did not know it at the start');
}
{ // a screen of the page's own comes up over the title while it is being timed
  const p = page(), was = before(p);
  p.bench.start(); for (let i = 0; i < 100000 && p.bench.state !== 'opened'; i++) p.frame(); for (let i = 0; i < 5; i++) p.frame();
  p.cover('a screen came up over it'); p.frame();
  check(!p.bench.active && p.bench.last === 'stopped: a screen came up over it' && !p.shown.length && !p.dumps && before(p) === was, 'a screen up over the title\'s window: the run stops, nothing shown, nothing kept');
}
{ // stopped before anything was timed
  const p = page(); p.bench.start(); p.frame(); p.bench.stop('Esc');
  check(!p.bench.active && !p.shown.length && !p.dumps && p.veil === null, 'stopped while waiting: nothing shown, nothing kept');
}
{ // stopped inside the first window: there is no number yet
  const p = page(); p.bench.start(); for (let i = 0; i < 100000 && p.bench.state !== 'opened'; i++) p.frame(); for (let i = 0; i < 5; i++) p.frame();
  p.bench.stop('Esc');
  check(!p.bench.active && !p.rec.recording && !p.shown.length && !p.dumps && p.veil === null, 'stopped inside the first window: the recording is dropped, no card, no file');
}
{ // the recording fills up under the sweep
  const p = page(), was = before(p);
  p.bench.start(); for (let i = 0; i < 100000 && p.bench.state !== 'sweep'; i++) p.frame();
  p.rec.cap = p.frames.length + 70; p.run();
  check(p.bench.last === 'stopped: the recording was full' && p.swStops === 1 && p.dumps === 1 && p.events.filter(e => e.kind === 'bench').length === 1 && p.frames.length < p.rec.cap && before(p) === was, 'the recording nearly full under the sweep: the run ends first, with its report in the file, everything back');
}
{ // a scene whose heavy moment never comes
  const p = page({ scene: 'calm' });
  p.bench.start({ heavyCapMs: 3000 }); p.run();
  check(p.bench.last === 'done' && p.bench.acts.join(' ') === 'waiting opened scene held sweep tail idle' && p.bench.result!.notes.some(n => /heavy moment had not come 3 s in/.test(n)), 'no heavy moment in the scene: it is held at the cap, and the report says so');
}
{ // no scene on this floor
  const p = page({ scene: 'none' }), was = before(p);
  p.bench.start(); p.run();
  check(p.bench.last === 'done' && p.states.join(' ') === 'waiting opened held sweep idle' && !p.liveIn.size, `no scene: the opening picture is the one held (${p.states.join(' ')})`);
  check(p.bench.result!.notes.some(n => /no scene was played \(units are on the floor\)/.test(n)) && !p.bench.result!.steps.some(s => s.id === 'scene') && before(p) === was, 'the report says why, and has no scene line');
}
{ // a scene that ends before its heavy moment
  const p = page({ scene: 'short' });
  p.bench.start(); p.run();
  check(p.bench.last === 'done' && p.states.join(' ') === 'waiting opened scene held sweep idle' && p.bench.result!.steps[1].id === 'scene', 'a scene over before its smoke: what it left is held, and the run still ends');
}
{ // no GPU timings
  const p = page({ timers: false });
  p.bench.start(); p.run();
  const R = p.bench.result!;
  check(R.steps.every(s => s.gpu === null) && R.features!.every(f => f.gpu === null) && /frame ms given back/.test(p.bench.report) && R.notes.some(n => /no GPU timings/.test(n)), 'no GPU timings: the frame time stands in, and the report says so');
}
{ // the tab away in the middle of a window
  const p = page();
  p.bench.start(); for (let i = 0; i < 100000 && p.bench.state !== 'opened'; i++) p.frame(); for (let i = 0; i < 160; i++) p.frame();
  p.frame(9000); p.run();
  const o = p.bench.result!.steps[0];
  check(o.id === 'opened' && o.p95 === 14 && p.bench.result!.notes.some(n => /the tab was away 1 time/.test(n)), 'a nine-second frame is the tab away: that window starts over, and no such frame is in its numbers');
}
{ // a page that says when it was hidden: the gap alone is not the tab away, and a hidden frame of the scene is left out
  const slow = page({ slow: 300, awayDep: true });
  slow.bench.start(); slow.run();
  check(slow.bench.last === 'done' && !slow.bench.result!.notes.some(n => /tab was away/.test(n)), 'a machine whose every frame takes four seconds is not a tab left away');
  const p = page({ awayDep: true });
  p.bench.start(); for (let i = 0; i < 100000 && p.bench.state !== 'scene'; i++) p.frame(); for (let i = 0; i < 20; i++) p.frame();
  p.hide(); p.frame(9000); p.run();
  const sc = p.bench.result!.steps[1];
  check(sc.id === 'scene' && sc.frames === p.live - 1 && p.bench.result!.notes.some(n => /the tab was away 1 time/.test(n)), `the tab hidden during the scene: that frame is not in the scene's numbers (${sc.frames} of ${p.live})`);
  const w = page({ awayDep: true });
  w.bench.start(); for (let i = 0; i < 25; i++) w.frame(); w.hide(); w.frame(60000); for (let i = 0; i < 25; i++) w.frame();
  const still = w.bench.state; for (let i = 0; i < 10; i++) w.frame();
  check(still === 'waiting' && w.bench.state === 'opened', 'opened in a tab nobody looked at for a minute: the wait starts over when it is looked at');
}
{ // A RUN OVER A SET OF SCENES, one page each: the first page times its opening picture and its scene and hands on; a middle page its scene; the last holds, sweeps and shows every line
  const SET = 'test-set', defs = [{ id: 'a', name: 'floor a' }, { id: 'b', name: 'floor b' }, { id: 'c', name: 'floor c' }];
  let carried: any = { steps: [], notes: [], scenes: [] }, overs = 0, closes = 0; const handed: any[] = [];
  const legOf = (i: number): BenchLeg => ({ set: SET, scene: defs[i], index: i, of: 3, secs: 20, ...carried, onward: n => { handed.push(n); carried = JSON.parse(JSON.stringify(n)); }, over: () => { overs++; }, closed: () => { closes++; } });
  const p0 = page({ scene: 'short' }); p0.bench.start({}, legOf(0)); p0.run();
  check(p0.states.join(' ') === 'waiting opened scene idle' && p0.bench.last === 'next scene' && handed.length === 1 && handed[0].steps.map((s: any) => s.id).join(' ') === 'opened scene:a' && handed[0].steps[1].label === 'floor a', `the first page: its opening picture, its scene, handed on (${p0.states.join(' ')} · ${handed[0]?.steps.map((s: any) => s.id).join(' ')})`);
  check(!p0.rec.recording && !p0.dumps && !p0.shown.length && p0.veil !== null && !overs && p0.applied.length === 0 && handed[0].scenes.length === 1 && handed[0].scenes[0].end === 'over at 3 s' && handed[0].boot?.toCard === 12400, 'handing on: no file, no card, the veil left up over the load, no preset touched; the scene\'s own word for its end and the first page\'s boot go along');
  const p1 = page({ scene: 'short' }); p1.bench.start({}, legOf(1)); p1.run();
  check(p1.states.join(' ') === 'waiting scene idle' && handed.length === 2 && handed[1].steps.map((s: any) => s.id).join(' ') === 'opened scene:a scene:b' && !p1.frozenIn.has('opened'), `a middle page: its scene alone, after the lines it was handed (${handed[1]?.steps.map((s: any) => s.id).join(' ')})`);
  const p2 = page(), was = before(p2); p2.bench.start({}, legOf(2)); p2.run();
  const R = p2.bench.result as BenchResult, ids = R.steps.map(s => s.id).join(' ');
  check(p2.states.join(' ') === 'waiting scene held sweep tail idle' && p2.bench.last === 'done' && ids === 'opened scene:a scene:b scene:c held verylow low medium high', `the last page: its scene, the heavy moment held, the sweep, the tail; every line of the run (${ids})`);
  check(R.set === SET && R.scenes?.map(x => x.id).join(' ') === 'a b c' && R.scenes!.every(x => typeof x.query === 'string') && R.boot?.toCard === 12400 && handed.length === 2 && overs === 1 && before(p2) === was && p2.dumps === 1, 'the result names the set and its scenes, the boot is the first page\'s, the run is over once, everything back, one file');
  check(/scenes test-set/.test(p2.bench.report) && /^floors\s+a over at 3 s · b over at 3 s/m.test(p2.bench.report) && /floor a/.test(p2.bench.report) && /floor c/.test(p2.bench.report) && /the scene held still/.test(p2.bench.report) && p2.bench.report.split('\n').every(l => l.length <= 150), 'the text names the set and carries a line per scene');
  console.log(p2.bench.report.split('\n').map(l => '    | ' + l).join('\n'));
  // the percentage on the veil climbs across the pages, and a later page's scene is live from its first timed frame
  { const q = page({ scene: 'short' }); carried = { steps: [], notes: [], scenes: [] }; const seen: number[] = []; q.bench.start({}, legOf(1)); for (let i = 0; i < 100000 && q.bench.active; i++) { q.frame(); const m = /(\d+) %/.exec(q.veil ?? ''); if (m) seen.push(+m[1]); }
    check(seen.length > 0 && Math.min(...seen) >= 33 && Math.max(...seen) <= 67, `the veil's percentage on the second page of three stays in its own third (${Math.min(...seen)} to ${Math.max(...seen)})`); }
  // stopped on a middle page: what the earlier pages measured is still shown, and nothing is carried further
  { carried = JSON.parse(JSON.stringify(handed[0])); overs = 0; const n = handed.length; const q = page(); q.bench.start({}, legOf(1)); for (let i = 0; i < 100000 && q.bench.state !== 'scene'; i++) q.frame(); for (let i = 0; i < 30; i++) q.frame();
    q.bench.stop('Esc');
    check(!q.bench.active && overs === 1 && handed.length === n && q.shown.length === 1 && /floor a/.test(q.shown[0]) && /floor b/.test(q.shown[0]) && /stopped early \(Esc\)/.test(q.shown[0]) && q.sceneStops === 1, 'Esc on a middle page: the card shows the lines so far, the scene is ended, the run is over'); }
  // stopped on the first page before anything was timed: no card to close, so the page is sent back by itself to where the run was started
  { carried = { steps: [], notes: [], scenes: [] }; overs = 0; const c0 = closes; const q = page({ scene: 'short' }); q.bench.start({}, legOf(0)); q.frame(); q.frame();
    q.bench.stop('Esc');
    check(!q.bench.active && overs === 1 && closes === c0 + 1 && !q.shown.length && !q.dumps && !q.rec.recording, 'Esc on the first page before a window was timed: no card, no file, the run over, the page sent back'); }
  // a scene that will not start on a middle page: said, and the run goes on
  { carried = JSON.parse(JSON.stringify(handed[0])); const n = handed.length; const q = page({ scene: 'none' }); q.bench.start({}, legOf(1)); q.run();
    check(handed.length === n + 1 && handed[n].notes.some((x: string) => /floor b was not played \(units are on the floor\)/.test(x)) && handed[n].steps.map((s: any) => s.id).join(' ') === 'opened scene:a', 'a scene that will not start: the report says so and the next page is loaded all the same'); }
  // the card's Close on the last page is the page's to act on
  { const view = (p2 as any).closeCard as (() => void) | undefined, c0 = closes; view?.(); check(closes === c0 + 1, 'the card\'s Close is handed to the page (which goes back to where the run was started)'); }
}
{ // THE PAGES OF A RUN (src/ui/benchrun.ts) on a fake tab: the record in the session store, the addresses, a reload in the middle
  const store = new Map<string, string>(); (globalThis as any).sessionStorage = { getItem: (k: string) => store.get(k) ?? null, setItem: (k: string, v: string) => { store.set(k, v); }, removeItem: (k: string) => { store.delete(k); } };
  const { benchBegin, benchLegOfPage, benchSceneSearch } = await import('../../src/ui/benchrun');
  const { BENCH_SCENES, BENCH_SET } = await import('../../src/host/benchScenes');
  const { SCENARIOS } = await import('../../src/scene/scenarioList');
  const tab = (search: string) => { const L: any = { pathname: '/', search, hash: '', went: [] as string[], how: [] as string[], assign(u: string) { this.went.push(u); this.how.push('assign'); }, replace(u: string) { this.went.push(u); this.how.push('replace'); } }; return L; };
  const searchOf = (u: string) => u.slice(u.indexOf('?'));
  const t0 = tab('?levelFile=depot.json&roomclear&squad4&bench=2&benchcap=30&nomusic&hard=0&hour=noon&quality=low');
  check(benchBegin(true, t0) === null && t0.went.length === 1 && t0.how[0] === 'assign' && t0.went[0] === '/' + benchSceneSearch(BENCH_SCENES[0], 0, ['nomusic', 'quality=low']) && !/hard=0|hour=|depot/.test(t0.went[0]), `begun from the title: the first scene's address is loaded, the visitor's sound and quality flags on it and nothing that would change the fight (${t0.went[0]})`);
  { const u = new URLSearchParams(searchOf(t0.went[0])), S = BENCH_SCENES[0], E = SCENARIOS.find(e => e.id === S.scn)!;
    check(u.get('levelFile') === E.level && u.get('roomclear') === String(S.seed) && u.get('seed') === String(S.seed) && u.has('squad4') && u.get('difficulty') === (S.hard ? 'hard' : 'standard') && u.has('noverdict') && u.has('nokeep') && u.get('benchrun') === '0' && !u.has('bench'), 'the address pins the level, the deal, the die and the difficulty, keeps no verdict and no replay of the try, and does not carry ?bench'); }
  const l0 = benchLegOfPage(tab(searchOf(t0.went[0])));
  check(!!l0 && l0.scene.id === BENCH_SCENES[0].id && l0.secs === '2' && l0.cap === 30 && l0.leg.index === 0 && l0.leg.of === BENCH_SCENES.length && l0.leg.set === BENCH_SET && l0.leg.notes.some(n => /started from the title/.test(n)), 'the first scene\'s page finds its part: the seconds a window, the short cap, and the note that its load is a second one');
  check(benchLegOfPage(tab(searchOf(t0.went[0]))) === null && store.size === 0, 'the same page loaded again in the middle is a plain mission page, and the record is dropped');
  // a run through to the end
  const t1 = tab('?quality=low&bench'); check(benchBegin(false, t1) === null && t1.how[0] === 'replace', 'begun from the address: that address is replaced, so Back does not ask for another run');
  let at = searchOf(t1.went[0]), steps = 0; const closedTo: string[] = [];
  for (let i = 0; i < BENCH_SCENES.length; i++) { const pg = tab(at), l = benchLegOfPage(pg)!; steps += l ? 1 : 0;
    if (i < BENCH_SCENES.length - 1) { l.leg.onward({ steps: [...l.leg.steps, { id: `scene:${l.scene.id}` } as any], notes: l.leg.notes, scenes: l.leg.scenes, boot: null, away: 0 }); at = searchOf(pg.went[0]); }
    else { check(l.leg.steps.length === BENCH_SCENES.length - 1 && !l.leg.notes.some(n => /started from the title/.test(n)), 'the last page is handed every earlier line; a run begun from the address has no second-load note'); l.leg.over(); l.leg.closed!(); closedTo.push(pg.went[0]); } }
  check(steps === BENCH_SCENES.length && store.size === 0 && closedTo[0] === '/?quality=low', `every scene got its page, the record is gone at the end, and Close goes back to where the run was started (${closedTo[0]})`);
  check(benchLegOfPage(tab(at)) === null, 'a scene\'s address opened with no run under way is a plain mission page');
  // the shader rows on trial ride along (asked for beside ?bench, a row is what the run was meant to measure: dropped at the first scene, the numbers were the plain build's)
  { const t2 = tab(`?bench&hour=noon&fogpass=old&pipequeue=0&shadertrim=0&downloads=1&cloud=0&cloudfiles=0&cloudprefs=0&${SHADER_ROW_WORDS.map((w, i) => `${w}=${i % 2}`).join('&')}`); benchBegin(false, t2);
    const u = new URLSearchParams(searchOf(t2.went[0]));
    check(SHADER_ROW_WORDS.length > 0 && SHADER_ROW_WORDS.every((w, i) => u.get(w) === String(i % 2)) && u.get('pipequeue') === '0' && u.get('shadertrim') === '0' && u.get('downloads') === '1' && u.get('cloud') === '0' && u.get('cloudfiles') === '0' && u.get('cloudprefs') === '0' && !u.has('hour') && !u.has('fogpass'), `every shader row's word, on or off, the compile queue's and the trim's are on the first scene's address (${t2.went[0]})`);
    const pg = tab(searchOf(t2.went[0])), l = benchLegOfPage(pg)!;
    check(l.leg.notes.filter(n => /^the scenes were played without hour, fogpass from the address/.test(n)).length === 1, `the words no scene is played with are named in the run's notes (${l.leg.notes.join(' | ')})`);
    const notesOf = (search: string) => { store.clear(); const t = tab(search); benchBegin(false, t); return benchLegOfPage(tab(searchOf(t.went[0])))!.leg.notes.filter(n => /played without/.test(n)); };
    const mission = notesOf('?levelFile=block.json&roomclear=1&squad4&seed=1&difficulty=hard&noverdict&nokeep&bench&hour=dusk&hour=noon&=x&quality=low');
    check(mission.length === 1 && /^the scenes were played without hour from the address/.test(mission[0]), `asked on a mission's own address: only the visitor's flag is named, once, and no word a scene's address says itself (${mission[0]})`);
    const many = notesOf('?bench&' + Array.from({ length: 14 }, (_, i) => `nosomethingquitelong${i}`).join('&'));
    check(many.length === 1 && many[0].length <= 150 && / and \d+ more from the address/.test(many[0]) && notesOf('?bench&mute&quality=low&compactshaders=1').length === 0, `a long address: the note still fits a line of the report; nothing left behind: no note (${many[0]})`);
    l.leg.onward({ steps: [{ id: `scene:${l.scene.id}` } as any], notes: l.leg.notes, scenes: [], boot: null, away: 0 });
    const u1 = new URLSearchParams(searchOf(pg.went[0]));
    check(u1.get('benchrun') === '1' && SHADER_ROW_WORDS.every((w, i) => u1.get(w) === String(i % 2)), `and on the next scene's (${pg.went[0]})`);
    store.clear(); }
  // the scenes themselves
  const rows = BENCH_SCENES.map(S => SCENARIOS.find(e => e.id === S.scn));
  check(BENCH_SCENES.length >= 2 && rows.every(e => !!e && e.kind === 'roomclear' && e.squad === 4 && !!e.level) && new Set(BENCH_SCENES.map(S => S.id)).size === BENCH_SCENES.length && BENCH_SCENES.every(S => S.name.length <= 21 && Number.isInteger(S.seed) && S.secs >= 60), `every scene is a four-man room clear of the mission list on its own level, with a name that fits its column: ${BENCH_SCENES.map(S => `${S.id} = ${S.scn} seed ${S.seed}`).join(' · ')}`);
  // the page's wiring, read as text
  const main = readFileSync(new URL('../../src/main.ts', import.meta.url), 'utf8'), loop = readFileSync(new URL('../../src/host/loop.ts', import.meta.url), 'utf8');
  check(/new HandsOff\(/.test(main) && /benchLegOfPage\(/.test(main) && /benchBegin\(/.test(main) && /new Bench\([^\n]*\brows: shaderRowsOn\b/.test(main) && /beforeTick/.test(main) && (loop.match(/ctx\.beforeTick(\?\.)?\(/g) ?? []).length === 2, 'the page plays a scene with the hands-off player, ticked before every game tick on either step');
}
{ // the Shaders rows the page was built with
  const p = page({ rows: ['compactshaders', 'a-later-build'] }); p.bench.start(); p.run();
  const R = p.bench.result!;
  check(p.bench.last === 'done' && JSON.stringify(R.rows) === '["compactshaders","a-later-build"]' && /^shaders\s+on: Compact shaders, a-later-build$/m.test(p.bench.report) && !R.notes.some(n => /after the start/.test(n)), `the result keeps the rows by their words and the report names them as Options does (${JSON.stringify(R.rows)})`);
  const l = page({ rows: ['compactshaders', 'firstframe', 'cutwhenneeded'] }); l.bench.start(); l.run();
  const late = l.bench.result!.notes.filter(n => /after the start/.test(n));
  check(late.length === 1 && /^a Shaders row that builds some shaders after the start is on/.test(late[0]) && late[0].length <= 150 && l.bench.report.includes(late[0]), `a row that builds shaders after the start is on: the report says its lines may not be the build's (${late[0]})`);
  const q = page({ rows: [] }); q.bench.start(); q.run();
  check(JSON.stringify(q.bench.result!.rows) === '[]' && /^shaders\s+no Shaders row is on \(Options, Graphics\)$/m.test(q.bench.report) && !q.bench.result!.notes.some(n => /after the start/.test(n)), 'no row on: the report says so, and has no such note');
}
{ // the result and the take go to whoever listens at the door (src/host/savefile.ts), each by itself, and the card says what became of them
  const told: SavedFile[] = []; SAVED.sink = f => { told.push(f); return f.kind === 'bench' ? 'sent' : false; };
  const p = page(); p.bench.start(); p.run(); SAVED.sink = null;
  const R = p.bench.result!, text = told.length ? await told[0].blob.text() : '';
  check(told.length === 1 && told[0].kind === 'bench' && /^bench-\d{4}-.*\.json$/.test(told[0].name) && text === JSON.stringify(R) && p.went[0].result === 'sent', `a listener at the door is handed the result as a file of its own, and the card is told it is being sent (${told.map(f => `${f.kind} ${f.name}`).join(', ')})`);
  const q = page(); q.bench.start(); q.run();
  check(q.went[0].result === false && q.went[0].take === false && !q.went[0].local && benchSaid(q.went[0]) === "Your results weren't sent, so press Copy if you want to share them." && benchNotice(q.went[0]), `nobody listening: the card says nothing was sent and points at Copy (${benchSaid(q.went[0])})`);
  SAVED.sink = () => 'held'; const r = page(); (r.rec as any).lastSent = 'held'; r.bench.start(); r.run();
  check(r.went[0].result === 'held' && benchSaid(r.went[0]) === 'Your results will be sent to the developer.' && benchNotice(r.went[0]), "a listener that keeps it for later (its browser has not seen the notice): the card says it will be sent, and carries the notice's line");
  const t = page(); (t.rec as any).lastSent = 'sent'; SAVED.sink = () => false; t.bench.start(); t.run(); SAVED.sink = null;
  check(t.went[0].take === 'sent' && t.went[0].result === false && benchSaid(t.went[0]) === 'Your results are being sent to the developer.' && !benchNotice(t.went[0]), 'a listener that took the take alone: the take holds the result, so the card says the results are going');
  const d = page(); (d.rec as any).lastLocal = true; d.bench.start(); d.run();
  check(d.went[0].local === true && /The full recording is in your downloads\.$/.test(benchSaid(d.went[0])), 'a take the browser was handed as a download (downloads put back): the card says where it is');
  { // each file's end is heard later, and the card's line follows
    const ends: ((ok: boolean) => void)[] = []; SAVED.sink = (_f, done) => { if (done) ends.push(done); return 'sent'; };
    const e = page(); let takeEnd: ((f: Fate) => void) | null = null; const stop0 = e.rec.stop; e.rec.stop = (...a: any[]) => { if (a[0] !== false) { takeEnd = a[1]; e.rec.lastSent = 'sent'; } return stop0.apply(e.rec, a); };
    e.bench.start(); e.run(); SAVED.sink = null; const w = e.went[0], first = benchSaid(w);
    ends[0]?.(true); const mid = benchSaid(w); (takeEnd as any)?.('arrived'); const end = benchSaid(w);
    check(ends.length === 1 && !!takeEnd && first === 'Your results are being sent to the developer.' && mid === first && end === 'Your results were sent to the developer.' && e.updates.length === 2 && e.updates[1].result === 'arrived' && e.updates[1].take === 'arrived',
      `the card's line follows the files to their end: being sent while one is on its way, sent once both have arrived (${first} / ${end})`);
    const f = page(); SAVED.sink = (_f, done) => { if (done) ends.push(done); return 'sent'; }; f.bench.start(); f.run(); SAVED.sink = null; ends[ends.length - 1]?.(false);
    check(f.updates.length === 1 && f.updates[0].result === false && /^Your results weren't sent, so press Copy/.test(benchSaid(f.went[0])), 'a result that was let go after all: the card says it was not sent and points at Copy');
  }
  const W = (result: Fate, take: Fate | null, local = false) => benchSaid({ result, take, local });
  check(W('sent', 'sent') === 'Your results are being sent to the developer.' && W('sent', null) === W('sent', 'sent') && W('sent', false) === "Your results are being sent to the developer. The full recording wasn't sent." && W('held', false) === "Your results will be sent to the developer. The full recording wasn't sent."
    && W(false, null) === W(false, false) && W('arrived', 'arrived') === 'Your results were sent to the developer.' && W('arrived', 'sent') === W('sent', 'sent') && W('arrived', 'held') === W('arrived', 'arrived') && W('held', 'held') === 'Your results will be sent to the developer.' && W('arrived', false) === "Your results were sent to the developer. The full recording wasn't sent." && W(false, 'arrived') === W('arrived', null)
    && W(false, false, true) === "Your results weren't sent, so press Copy if you want to share them. The full recording is in your downloads." && W('sent', 'sent', true) === 'Your results are being sent to the developer. The full recording is in your downloads.',
    'the card\'s line for every way a run can go: both going, the result alone, nothing, and a take that was downloaded as well');
  check(!benchNotice({ result: 'sent', take: 'sent' }) && !benchNotice({ result: 'arrived', take: 'arrived' }) && benchNotice({ result: 'held', take: 'held' }) && !benchNotice({ result: 'sent', take: false }) && benchNotice({ result: 'sent', take: 'held' }) && benchNotice({ result: false, take: null }), "the notice's line is on the card unless the run is already going with nothing held back for it");
  for (const w of ['sent', 'held', 'arrived', false] as Fate[]) for (const k of [null, 'sent', 'held', 'arrived', false] as (Fate | null)[]) if (!/^[A-Z][^\n]*\.$/.test(W(w, k)) || /download/i.test(W(w, k))) check(false, `a line that speaks of a download, or is not a sentence: ${W(w, k)}`);
}
{ // the recorder stopped by hand, and a run refused
  const p = page(), was = before(p);
  p.bench.start(); for (let i = 0; i < 100000 && p.Q.current !== 'verylow'; i++) p.frame();
  p.rec.recording = false; p.frame();
  check(!p.bench.active && /the recorder stopped/.test(p.bench.last) && before(p) === was, 'the recorder stopped under it: the run ends, everything back');
  p.rec.recording = true; check(/^not now/.test(p.bench.start()) && !p.bench.active, 'a recording under way: a run is refused'); p.rec.recording = false;
}
{ // the text on its own
  const r: BenchResult = { version: 1, at: '2026-01-02T03:04:05.000Z', query: '', machine: { webgpu: 'x y', browser: '', os: '', userAgent: 'UA/1', dpr: 1, fallback: true }, boot: null, timers: true, steps: [], notes: ['a note'] };
  const T = benchReport(r);
  check(/SOFTWARE RENDERING/.test(T) && /\(no flags\)/.test(T) && /browser\s+UA\/1/.test(T) && /a note$/.test(T) && !/undefined|NaN/.test(T), 'a bare result still reads: the software adapter is said, nothing undefined');
  check(!/^shaders/m.test(T) && benchReport({ ...r, rows: [...SHADER_ROW_WORDS] }).split('\n').every(l => l.length <= 150), 'an old result with no word of its shader rows has no line of them; with every row on the line still fits the card');
}
{ // (the page's card is not built here: only that its source still puts the notice's line on a card that does not say a copy is being sent)
  const src = (await import('node:fs')).readFileSync(new URL('../../src/ui/bench.ts', import.meta.url), 'utf8');
  check(/if \(benchNotice\(went\)\) cloudNote\(box, undefined, \{\}\);/.test(src) && /said\.textContent = benchSaid\(went\);/.test(src) && /update\(went\) \{ const said = went === shown \? card\?\.querySelector\('#bench-said'\) : null; if \(said\) said\.textContent = benchSaid\(went\); \}/.test(src) && /if \(waiting \|\| !went\.idle \|\| !benchGoing\(went\)\) \{ leave\(\); return; \}/.test(src) && /import \{ cloudNote \} from '\.\/cloudnote';/.test(src), "the result card says where the run went and carries the sharing notice's line unless all of it is already going (read in the source)");
}
console.log(red ? `\n${red} FAILED` : '\nall pass');
(globalThis as any).process?.exit(red ? 1 : 0);
