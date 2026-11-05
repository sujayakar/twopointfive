#!/usr/bin/env bun
// THE WEATHER THAT PLAY READS, HELD (rust/sim/src/wind.rs, weather.rs, air.rs, snow.rs; src/sim/weatherhome.ts is the seam; docs/agents/weather.md says what is gameplay and what is dressing):
//   bun run tools/qa/weather-ident.ts                 every run below on the page's stand, in a child of its own, each with every answer of the module said to the seam and folded (tools/qa/ident-pin.ts), held to the recording
//   WEATHER_BREAK=1 bun run tools/qa/weather-ident.ts THE RED PATH: every answer is bent as it is folded (a single answer goes on to the game bent too), and every name's word must move from the same runs played plain: what holds a name reaches it
//   … --record                                        the page's stand's recording of every name's word (tools/qa/ident-pin.ts): a rule that moves the weather's answers records them again, and says why
//   … --against <root>                                the same run as another checkout plays it, every count and word the same
// The runs reach every name: each weather kind on a level with a yard, a kind under the level's own roofs, a snow level, the night storm with men going in and out of it, the hour, and
// wind through a broken pane; `asks` puts what no headless host asks of its own (the air at a point, a sky over each man, lying snow underfoot) to the module on the way.
// Exit 0 = every verdict passed · 1 = one did not (with WEATHER_BREAK=1: 1 = the red path is red).
import { resolve } from 'node:path';
import { spawn } from 'node:child_process';
const ROOT = resolve(import.meta.dir ?? '.', '../..'), args = process.argv.slice(2), BREAK = process.env.WEATHER_BREAK === '1';
const flag = (n: string) => { const i = args.indexOf(n); return i >= 0 ? args[i + 1] : undefined; };
process.env.TPS_PROFILE ??= 'shipped';   // (one stand: the page's)
const RUNS = ['compound-four:1::', 'compound-four:1:clear:', 'compound-four:1:overcast:', 'compound-four:1:rain:', 'compound-four:1:downpour:', 'compound-four:1:fog:', 'compound-four:1:dust:',
  'pass-four:1::', 'warehouse-four:1:rain:', 'block-squads:1:downpour:', 'hostage-rescue:1::', 'compound-four:1:rain:midday', 'compound-four:1::midday'];
/** every name a run of this check must reach */
const NAMES = ['wind.jet', 'wind.storm', 'wind.stormRoofs', 'wind.noise', 'fog.tau', 'snow.bog', 'weather.roofsOf', 'weather.strokeMean', 'weather.underRoof', 'weather.rainBed', 'weather.hearMask', 'weather.airTrans', 'weather.strike', 'weather.thunder', 'weather.lot', 'weather.gap', 'weather.strokeDue', 'wet.openSky', 'wet.step', 'air.settle', 'air.update'];
const CAP = 80;   // seconds of a run

if (args[0] === '--run') {
  const [scn, seed, kind, hour] = args[1].split(':');
  const { Fold } = await import(`${ROOT}/tools/qa/ident-pin.ts`);
  const { WX_WATCH } = await import(`${ROOT}/src/sim/weatherhome.ts`);
  const { Game } = await import(`${ROOT}/src/game/game.ts`);
  const PT: any = await import(`${ROOT}/tools/qa/playthrough.ts`);
  const W: any = await import(`${ROOT}/src/game/wind.ts`), WX: any = await import(`${ROOT}/src/game/weatherKinds.ts`), WT: any = await import(`${ROOT}/src/game/wet.ts`), SB: any = await import(`${ROOT}/src/game/snowBog.ts`), WK: any = await import(`${ROOT}/src/game/weather.ts`);
  const HR: any = await import(`${ROOT}/src/game/hours.ts`);
  const F = new Fold();
  const fair = { wetted: 0, hour: hour ? 0 : 1 };
  const bent = (v: number) => BREAK ? (v === 0 ? 1 : v === 1 ? 0 : v * (1 + 1e-6) + 1e-9) : v;
  WX_WATCH.on = true;
  WX_WATCH.said = (name: string, answer: number, asked: readonly number[]) => {
    const v = bent(answer), e = F.call(name); F.doubles(e, asked); F.double(e, v);
    return v;
  };
  WX_WATCH.saidMany = (name: string, answers: number[], asked: readonly number[]) => {
    if (answers.length) answers[0] = bent(answers[0]);
    const e = F.call(name); F.doubles(e, asked); F.doubles(e, answers);
  };
  let n = 0;
  const update = Game.prototype.update;
  Game.prototype.update = function (this: any, ...a: unknown[]) {
    if (hour && n === 0) { HR.applyHour(this, hour); fair.hour = 1; }
    const r = (update as (...x: unknown[]) => unknown).apply(this, a);
    if (++n % 30 === 1) asks(this, n);
    return r;
  } as typeof update;
  /** what no host without a picture asks of its own: the air at a point, under the sources and the storm as they stand; each man's sky; the snow under him */
  function asks(g: any, k: number) {
    const men = [g.player.char, ...g.guards.map((gd: any) => gd.char)], p = men[k % men.length].pos, at: number[] = [p[0], 1.2, p[2]], o: number[] = [0, 0, 0];
    g.wind.at(at, o); W.jetVel([p[0], 3, p[2]], [0, -1, 0], 2 + (k % 7), 2.5, 0.3, 0.2, 6, 0, [p[0] + (k % 5) * 0.1, 0.3 + (k % 11) * 0.1, p[2] - 0.2], o); W.jetVel([p[0], 2, p[2]], [1, 0, 0], 3, 2, 0.2, 0.1, 5, -1e9, [p[0] + 1.5, 2.1, p[2] + 0.1], o);
    const wind: [number, number, number] = [g.weather.wind[0], 0, g.weather.wind[1]]; W.stormVel(wind, WK.INTERIOR, WK.ANNEX, at, o); W.stormVelRoofs(wind, g.weather.roofs.length ? g.weather.roofs : [[1e6, 1e6, -1e6, -1e6]], at, o); W.vnoise(hashOf(g.time), g.time / 2.5);
    if (WT.openSky(g, men[k % men.length])) fair.wetted++;
    SB.snowBogAt(g, p[0], 0, p[2]); SB.snowBogAt(g, p[0], 0.5, p[2]);
    g.weather.rainBed(p[0], p[2]); g.weather.hearMask(at); g.weather.airTrans(at, [p[0] + 8, 1.5, p[2] + 3]); g.weather.underRoof(p[0], p[2]); if (g.weather.fog) WX.fogTau(g.weather.fog, at, [p[0] + 8, 1.5, p[2] + 3]);
    if (k % 60 === 1 && g.weather.enabled && !g.weather.flash) g.weather.strike();   // (a stroke of its own: a run's first would not come for a good while)
    if (k % 30 === 1 && g.weather.kind) { const R = WK.roofsOf(g.level.boxes); void R; }
  }
  const hashOf = (t: number) => (Math.floor(t * 7) * 2654435761) >>> 0;
  const r = await PT.play(scn, Number(seed), CAP, false, kind ? { WEATHER: { kind } } : {});
  console.log('WEATHER-RUN ' + JSON.stringify({ run: args[1], ticks: n, end: String(r.end), words: F.words(), wetted: fair.wetted, hour: fair.hour, profile: process.env.TPS_PROFILE ?? 'frozen' }));
  process.exit(0);
}

interface Run { run: string; ticks: number; end: string; words: Record<string, { asked: number; word: string }>; wetted: number; hour: number; profile: string }
const one = (run: string): Promise<Run | string> => new Promise(done => {
  const p = spawn('bun', ['run', 'tools/qa/weather-ident.ts', '--run', run], { cwd: ROOT, env: process.env as Record<string, string> }); let out = '', err = '';
  p.stdout.on('data', (d: unknown) => { out += d; }); p.stderr.on('data', (d: unknown) => { err += d; });
  p.on('close', code => { const l = out.split('\n').find(x => x.startsWith('WEATHER-RUN ')); done(l ? JSON.parse(l.slice(12)) as Run : `${run} left nothing (exit ${code}): ${err.trim().split('\n').slice(-3).join(' / ').slice(0, 300)}`); });
});
async function pool<T>(jobs: (() => Promise<T>)[], n: number): Promise<T[]> { const out: T[] = new Array(jobs.length); let i = 0; await Promise.all(Array.from({ length: n }, async () => { while (i < jobs.length) { const k = i++; out[k] = await jobs[k](); } })); return out; }

const runs = flag('--runs')?.split(',') ?? RUNS, got = await pool(runs.map(r => () => one(r)), Number(flag('--jobs') ?? 4));
let fails = 0;
const verdict = (ok: boolean, label: string, detail: string) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}${detail ? ': ' + detail : ''}`); if (!ok) fails++; };
const broken = got.filter(r => typeof r === 'string') as string[], done = got.filter(r => typeof r !== 'string') as Run[];
verdict(broken.length === 0, 'runs', broken.length ? broken.join(' · ') : `${done.length} runs, ${done.reduce((a, r) => a + r.ticks, 0)} ticks`);
const words: Record<string, { asked: number; word: string }> = {};
for (const r of done) for (const [k, v] of Object.entries(r.words)) words[`${r.run}|${k}`] = v;
const reached = new Set(Object.keys(words).map(k => k.split('|')[1])), missing = NAMES.filter(n => !reached.has(n));
if (!args.includes('--words')) verdict(missing.length === 0 || !!flag('--runs'), 'reach: the runs reach every name', missing.length ? `NOT REACHED: ${missing.join(', ')}` : `${NAMES.length} names`);
if (!args.includes('--words') && !flag('--runs')) {
  const wetted = done.filter(r => r.wetted > 0).length, hours = done.filter(r => r.hour > 0).length;
  verdict(wetted >= 8 && hours === done.length, 'fair: men stood under the open sky, and the hour was laid on the runs that name one', `${wetted} of ${done.length} runs had a man under open sky · ${hours} of ${done.length} at the hour they name`);
}
if (!args.includes('--words')) {   // (a knob a rule reads and a question outside a tick does not hand over would be read stale, and no run of a default knob would show it)
  const { readFileSync } = await import('node:fs'), text = (f: string) => readFileSync(`${ROOT}/rust/sim/src/${f}`, 'utf8'), read = new Set(['weather.rs', 'air.rs', 'snow.rs', 'wind.rs'].flatMap(f => [...text(f).matchAll(/\bk\(([A-Z0-9_]+)\)/g)].map(m => m[1])));
  const named = new Set(/pub const READS: [^=]*= \[([^\]]*)\]/.exec(text('weather.rs'))?.[1].split(',').map(x => x.trim()).filter(Boolean) ?? []), missed = [...read].filter(n => !named.has(n)), spare = [...named].filter(n => !read.has(n));
  verdict(read.size > 0 && !missed.length && !spare.length, 'knobs: weather.rs READS names every knob the weather\'s rules read', missed.length || spare.length ? `READ AND NOT NAMED: ${missed.join(', ') || 'none'} · NAMED AND NOT READ: ${spare.join(', ') || 'none'}` : `${read.size} knobs`);
}
const { hold } = await import(`${ROOT}/tools/qa/ident-pin.ts`);
const end = done.map(r => `${r.run}=${r.end}`).join(' ');
const H = hold({ root: ROOT, tool: 'tools/qa/weather-ident.ts', fixture: 'weather-words.json', stand: 'shipped', run: runs.join(' '), runArgs: flag('--runs') ? ['--runs', runs.join(',')] : [], end, words, breakWord: 'WEATHER_BREAK', unfit: broken.length ? 'a run left nothing' : '', verdict, say: (l: string) => console.log(l) });
if (args.includes('--words')) process.exit(0);
if (H.record && fails === 0) H.record();
if (BREAK) { const moved = new Set(H.off.map((k: string) => k.split('|')[1])), stayed = NAMES.filter(n => !moved.has(n)); console.log(`- ${stayed.length ? 'NOT RED' : 'RED'} — the seam's answers bent: ${moved.size} names moved from the same runs played plain${stayed.length ? ` · THE BREAK DID NOT REACH ${stayed.join(', ')}` : ', every one of them'}`); process.exit(stayed.length || broken.length ? 0 : 1); }
console.log(fails ? `FAIL — ${fails} verdicts` : H.to ? 'PASS — the weather is the module\'s and the same' : args.includes('--record') ? 'PASS — recorded' : 'PASS — the runs reach every name and are fair, and THE ANSWERS WERE HELD TO NOTHING on this stand');
process.exit(fails ? 1 : 0);
