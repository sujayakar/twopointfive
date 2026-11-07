// The 'same-way' probe: A PAIR GOES ROUND ONE WAY (orders.ts ORDERS.sameWay — sameWays). Run through probes.ts:
//   bun run tools/qa/probes.ts same-way        (family: ordereddoor)
// PASS / FAIL each, headless. The queueing tutorial's level (levels/tut-queue.json) stood up as its start deals it (the four men; the level's own enemies are at their posts — see rig()): a pair is stood side by side at a
// row of spots along the front of the hall and told to stack on the hall's back door, as a player tells it. The probe reads each man's planned way (the
// corners his order was handed, joined by the game's own path search) and measures how far apart the two ways get, with its own ruler:
//   · at no spot are the two ways a building apart (one man round the hall by each side)
//   · walked, from the spot that straddles the hall's middle: the two are never far from each other and both stand on their marks
//   · all four told at once: each pair's two ways keep together
// THE SWITCH IS SETTLED AND NOT WRITTEN HERE (2026-11-07), nor is the lesson engine's. Out with the off side: 'with the switch off the flaw is there', '… at a
// walk the rule allows, and where the two agreed already nothing is handed out anew' and 'its bounds are real' (each measured against the off row: no keeper),
// the off halves of the two walks (they were walked from the off row's worst spot), 'off, nothing reads it', the stand's word and the recording.
// What it cannot say: what the walk looks like (never seen on a GPU, nobody has played it), or what it does to a fight — that is playthrough.ts's tally.
// The probe's own red path: SAMEWAY_BREAK=1 bun run tools/qa/probes.ts same-way must FAIL (a staging break: in the two walks the pair's second man is
// stood 12 m from the first when the order is given, so the two are apart from the first frame; the word is also the name of an old hook of the
// TypeScript orders, which the Rust tick does not read).
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { ROOT, standUp } from './headless.ts';
const OM: any = await import(`${ROOT}/src/game/orders.ts`);
const GDM: any = await import(`${ROOT}/src/game/guards.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const ORDERS = OM.ORDERS;

type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const LEVEL = 'tut-queue', BACK = [30.8, 19.0], OUT = [30.8, 0, 20.6], DT = 1 / 60;
/** the probe's own ruler: how far way `a` gets from way `b`, asked every quarter metre of `a` */
function apart(a: number[][], b: number[][]): number {
  const off = (x: number, z: number) => { let m = Infinity; for (let i = 1; i < b.length; i++) { const p = b[i - 1], q = b[i], ux = q[0] - p[0], uz = q[2] - p[2], L = ux * ux + uz * uz, k = L > 1e-9 ? Math.max(0, Math.min(1, ((x - p[0]) * ux + (z - p[2]) * uz) / L)) : 0; m = Math.min(m, Math.hypot(x - p[0] - ux * k, z - p[2] - uz * k)); } return m; };
  let d = 0; for (let i = 1; i < a.length; i++) { const p = a[i - 1], q = a[i], n = Math.max(1, Math.ceil(flat(p, q) / 0.25)); for (let s = 0; s <= n; s++) d = Math.max(d, off(p[0] + (q[0] - p[0]) * s / n, p[2] + (q[2] - p[2]) * s / n)); }
  return d; }
const lenOf = (w: number[][]) => { let m = 0; for (let i = 1; i < w.length; i++) m += flat(w[i - 1], w[i]); return m; };

interface Spot { x: number; z: number; gap: number; len: number[]; via: string; refused: string | null }
interface Rig { g: any; W: any; men: any[]; door: any; way: (m: any) => number[][] | null; put: (m: any, p: number[]) => void; tell: (who: any) => string | null }
async function rig(): Promise<Rig> {
  const W: any = await quiet(() => standUp({ uploadEvery: 4, level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', `${LEVEL}.json`), 'utf8'))), seed: 1 })), g = W.game;
  const started: any = await quiet(() => startFromQuery(g, `?levelFile=${LEVEL}.json&roomclear=1&squad4`)); if (!started.spec || started.why) throw new Error(`the level did not start: ${started.why}`);
  for (const h of [...g.guards]) if (h.team !== 'operative') GDM.despawnUnit(g, h);   // (NOT an empty floor: a man taken off the roster has the start deal the floor afresh on its next frame, so the level's eight are back at their posts when the men below are looked up. Left as it is: every figure of this probe was taken so)
  for (let i = 0; i < 30; i++) W.step(DT);
  const men = g.guards.filter((m: any) => m.team === 'operative' && m.unit && m.state !== 'dead'), door = g.doors.list.find((d: any) => Math.hypot(d.pos[0] - BACK[0], d.pos[2] - BACK[1]) < 0.7);
  if (!door || men.length < 4) throw new Error(`the staging is not there: ${men.length} men, the back door ${door ? 'found' : 'not found'}`);
  const nav = g.col.nav;
  const way = (m: any): number[][] | null => { const A = g.orders.of(m); if (!A?.post) return null; const w: number[][] = [[m.char.pos[0], 0, m.char.pos[2]]]; let q = w[0]; for (const v of [...A.via, A.post]) { const leg = nav.findPath(q, v); if (!leg) return null; w.push(...leg); q = v; } return w; };
  const put = (m: any, p: number[]) => { m.char.pos[0] = p[0]; m.char.pos[2] = p[2]; m.char.vel[0] = m.char.vel[2] = 0; if (m.path) m.path.length = 0; m.pathI = 0; };
  const tell = (who: any) => g.orders.give('stack', { kind: 'door', at: door.frameCentre, door, side: OM.sideOf(door, OUT) }, who);
  return { g, W, men, door, way, put, tell };
}
/** a pair stood side by side at each spot of a row along the hall's front and told to stack on its back door: what each man was handed */
async function row(): Promise<Spot[]> {
  const R = await rig(), { g, men } = R;
  const pair = men[2].unit.pair, two = men.filter((m: any) => m.unit.pair === pair), out: Spot[] = [];
  const rest = men.filter((m: any) => !two.includes(m)); rest.forEach((m: any, i: number) => R.put(m, [16 + i * 1.2, 0, 9]));
  for (let x = 25; x <= 33.001; x += 0.5) for (const z of [8, 10]) { const p = [[x, 0, z], [x + 1.4, 0, z]]; if (p.some(q => g.col.nav.isBlocked(q[0], q[2]))) continue;
    R.put(two[0], p[0]); R.put(two[1], p[1]); const no = await quiet(() => R.tell(pair)), a = R.way(two[0]), b = R.way(two[1]);
    if (no || !a || !b) { out.push({ x, z, gap: -1, len: [0, 0], via: '', refused: no ?? 'no way' }); continue; }
    out.push({ x, z, gap: Math.max(apart(a, b), apart(b, a)), len: [lenOf(a), lenOf(b)], via: JSON.stringify(two.map((m: any) => g.orders.of(m).via)), refused: null }); }
  return out;
}
/** the pair walked from one spot until both stand on their marks (or a minute is up): the furthest they got from each other, and whether they arrived */
async function walk(x: number, z: number, all: boolean): Promise<{ far: number; farB: number; arrived: number; secs: number; of: number }> {
  const R = await rig(), { g, men, W } = R;
  const pair = men[2].unit.pair, two = men.filter((m: any) => m.unit.pair === pair), rest = men.filter((m: any) => !two.includes(m));
  R.put(two[0], [x, 0, z]); R.put(two[1], [x + (process.env.SAMEWAY_BREAK === '1' ? 12 : 1.4), 0, z]);
  if (all) { R.put(rest[0], [x, 0, z - 1.4]); R.put(rest[1], [x + 1.4, 0, z - 1.4]); } else rest.forEach((m: any, i: number) => R.put(m, [16 + i * 1.2, 0, 9]));
  const no = await quiet(() => R.tell(all ? 'all' : pair)); if (no) throw new Error(`the order was refused: ${no}`);
  const who = all ? men : two; let far = 0, farB = 0, secs = 0;
  for (let f = 0; f < 60 * 60; f++) { await quiet(() => W.step(DT)); secs += DT; far = Math.max(far, flat(two[0].char.pos, two[1].char.pos)); if (all) farB = Math.max(farB, flat(rest[0].char.pos, rest[1].char.pos));
    if (who.every((m: any) => g.orders.of(m)?.arrived)) break; }
  return { far, farB, arrived: who.filter((m: any) => g.orders.of(m)?.arrived).length, secs, of: who.length };
}

export async function sameWay(): Promise<boolean> {
  const F = family('same-way: two men of a pair handed one order on one door from one spot walk round a building by the same side');
  const keep = { ...ORDERS }, back = () => { Object.assign(ORDERS, keep); };
  try {
    const A = ORDERS.sameApartM, on = await row(); back();
    const still = on.filter(r => !r.refused && r.gap > A);
    F.verdict(on.length > 0 && still.length === 0 && on.every(r => !r.refused), 'at no spot along the hall\'s front do the pair\'s two ways part round it: the second man takes the first man\'s way', `${on.length} spots, the pair stood 1.4 m apart, told to stack on the hall's back door · spots where the two ways get over ${A} m apart: ${still.length} (want 0) · the widest ${Math.max(...on.map(r => r.gap)).toFixed(1)} m · refused: ${on.filter(r => r.refused).length}`);
    // (walked from the spot whose pair straddles the hall's middle, where a way round by each side is as long as the other: the first such spot of the row)
    const mid = on.filter(r => !r.refused).sort((a, b) => Math.abs(a.x + 0.7 - BACK[0]) - Math.abs(b.x + 0.7 - BACK[0]) || a.z - b.z)[0];
    if (mid) { const wx = mid.x, wz = mid.z;
      const wOn = await walk(wx, wz, false); back();
      F.verdict(wOn.arrived === 2 && wOn.far <= A, `walked from the spot straddling the hall's middle (${wx},${wz}): the two keep together all the way and both stand on their marks`, `the furthest the two got from each other: ${wOn.far.toFixed(1)} m (${wOn.arrived} of 2 on their marks in ${wOn.secs.toFixed(0)} s; want at most ${A} m)`);
      const fOn = await walk(wx, wz, true); back();
      F.verdict(fOn.arrived === 4 && fOn.far <= A && fOn.farB <= A, 'all four told at once: each pair\'s two men keep together', `the furthest apart, this pair / the other: ${fOn.far.toFixed(1)} / ${fOn.farB.toFixed(1)} m (${fOn.arrived} of 4 on their marks; want at most ${A} m each)`); }
    else F.verdict(false, 'walked from the spot straddling the hall\'s middle', 'no spot took the order: nothing to walk');
    return F.done();
  } finally { back(); }
}
