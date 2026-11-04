// THE LOOK PAGE'S SHAPES, ON EVERY LEVEL (src/scene/geoTry.ts: the look page's geometry trial; ?geo=0 leaves it out; off on every other page).
// Two rules rebuild what a level's own boxes already say, and neither knows a level by name:
//   a ROUND THING is the kit's octagon (four slabs 45° apart) made one twelve-sided prism;
//   a BROKEN WALL TOP is a bite out of a YARD WALL, and a yard wall is a wall with free floor under the open sky on both hands, on the renderer's own floor
//   plan (render/placeField.ts skyPlan). Until 11-02 the test was a coping slab on the wall, which one kit lays: the levels built before it had none to find.
// Held here to a yard built by hand and to the levels as their files have them. No GPU: the picture is never SEEN here.
//   bun run tools/qa/geotry-check.ts                  exit 0 = every line passed, 1 = one failed (the same on both stands: it reads level files and no switch)
//   GEOTRY_BREAK=1 bun run tools/qa/geotry-check.ts   every spot is taken for open sky: must FAIL
import { readFileSync } from 'node:fs';
import { ROOT } from './headless.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { skyPlan, placeMask, PLACE_W, PLACE_RES } = await import(`${ROOT}/src/render/placeField.ts`);
const { WORLD } = await import(`${ROOT}/src/scene/world.ts`);
const { LOOK_LEVELS } = await import(`${ROOT}/src/scene/look.ts`);
const { geoTryLevel, breakWallTops } = await import(`${ROOT}/src/scene/geoTry.ts`);
const { BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);
const BRK = process.env.GEOTRY_BREAK === '1'; let red = false;
const check = (ok: boolean, what: string, detail = '') => { red ||= !ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
console.log(`\n=== geotry: the look page's shapes on every level${BRK ? ' (every spot taken for open sky: must fail)' : ''} ===`);
type B = { c: number[]; h: number[]; yaw: number; flags: number; albedo: number[]; cut?: unknown[]; cutOld?: boolean; cutRound?: number; rot?: unknown; marks?: unknown };
type Bite = { x: number; z: number; alongX: boolean; s0: number; s1: number; T: number; H: number; D: number };
const box = (c: number[], h: number[], flags = 0): B => ({ c, h, yaw: 0, flags, albedo: [0.4, 0.38, 0.34] });
const tried = (L: B[], lights: { pos: number[] }[] = []) => breakWallTops(L, lights, BRK ? () => true : skyPlan(L)) as Bite[];
/** a bite's own boxes: the cut stretches that stand in it */
const stretches = (L: B[], q: Bite) => L.filter(b => b.cut && b.cutOld && !(b.flags & BoxFlag.NoShadow) && Math.abs(b.c[q.alongX ? 2 : 0] - (q.alongX ? q.z : q.x)) < 1e-6 && b.c[q.alongX ? 0 : 2] > q.s0 && b.c[q.alongX ? 0 : 2] < q.s1);
/** what is drawn in the way of a bite (in its gap or hung on the wall's face beside it), its own stretches apart */
const inGap = (L: B[], q: Bite) => L.filter(b => !(b.flags & BoxFlag.NoPrimary) && !(b.cut && b.cutOld) && b.c[1] + b.h[1] > q.H - q.D && b.c[1] - b.h[1] < q.H + 0.3
  && Math.abs(b.c[q.alongX ? 0 : 2] - (q.s0 + q.s1) / 2) < (q.s1 - q.s0) / 2 - 0.01 + b.h[q.alongX ? 0 : 2] && Math.abs(b.c[q.alongX ? 2 : 0] - (q.alongX ? q.z : q.x)) < q.T + 0.3 + b.h[q.alongX ? 2 : 0]);
/** is a roof over this spot, asked of the boxes themselves (not of the plan the rule reads): a slab over head height, wide both ways */
const roofOver = (L: B[], x: number, z: number) => L.some(b => !b.rot && (b.flags & 3) !== 3 && b.c[1] - b.h[1] >= 1.9 && Math.min(b.h[0], b.h[2]) >= 0.45 && Math.abs(x - b.c[0]) <= b.h[0] && Math.abs(z - b.c[2]) <= b.h[2]);

// ---- a yard built by hand: two long walls, a house against one of them, a slab that caps two walls
{ const ground = () => box([20, -0.1, 14], [20, 0.1, 14]), yard = () => [ground(), box([10, 1.5, 6], [6, 1.5, 0.12]), box([10, 1.5, 20], [6, 1.5, 0.12])];
  const a = yard(), n = tried(a);
  check(n.length === 2 && n.every(q => stretches(a, q).length >= 3), 'a wall with the sky on both hands loses a bite out of its top, and needs no coping to be found', `${n.length} of two bare walls broken`);
  // a roof over the ground on one hand of the first wall, its whole length, a hand over the wall's top and stopping 0.6 m short of it (so it is the sky test
  // that says no, not "something tall against the wall"): that wall is a house's now
  const b = yard(); b.push(box([10, 3.3, 3.15], [6.5, 0.1, 2.15])); const nb = tried(b);
  check(nb.length === 1 && Math.abs(nb[0].z - 20) < 0.01, 'a wall with a roof on one hand is a house\'s wall, and is left whole', `${nb.length} broken, at z ${nb.map(q => q.z.toFixed(1)).join(', ')}`);
  // one coping slab over two walls end to end, and one stretch of snow on it: each is broken with each wall, and nothing is left lying over a bite
  const c = [ground(), box([8, 1.2, 10], [5, 1.2, 0.12]), box([24, 1.2, 10], [11, 1.2, 0.12]), box([19, 2.44, 10], [16, 0.04, 0.15]), box([19, 2.51, 10], [16, 0.03, 0.12], BoxFlag.NoShadow)]; const nc = tried(c);
  const slabs = c.filter(o => !o.cut && o.flags === 0 && Math.abs(o.c[1] - 2.44) < 1e-6), run = slabs.reduce((t, o) => t + 2 * o.h[0], 0), gone = nc.reduce((t, q) => t + q.s1 - q.s0, 0);
  const snow = c.filter(o => o.flags === BoxFlag.NoShadow && Math.abs(o.c[1] - 2.51) < 1e-6 && o.h[0] > 0.05), left = nc.flatMap(q => inGap(c, q));
  check(nc.length === 2 && left.length === 0 && slabs.length === 3 && snow.length === 3 && Math.abs(run + gone - 32) < 1e-6, 'a coping and the snow on it are broken with the wall, whatever their length: one slab over two walls is three stretches and two gaps',
    `${nc.length} broken, ${slabs.length} stretches of coping ${run.toFixed(2)} m + ${gone.toFixed(2)} m of bite, ${snow.length} of snow, ${left.length} boxes left over a bite`);
  // posts against the wall, lamps over it, an alarm's housing on its face (two drawn-only boxes), another bite near it
  const d = [ground(), box([10, 1.5, 6], [2.2, 1.5, 0.12])]; for (let x = 8; x <= 12; x += 0.5) d.push(box([x, 1.2, 6.3], [0.1, 1.2, 0.1])); const nd = tried(d).length;
  const e = yard().slice(0, 2), ne = tried(e, [6, 8, 10, 12, 14].map(x => ({ pos: [x, 2.6, 6.2] }))).length;
  const g = yard().slice(0, 2); for (let x = 5; x <= 15; x += 1) g.push(box([x, 2.7, 6.17], [0.08, 0.06, 0.05], BoxFlag.NoShadow)); const ng = tried(g).length;
  const f = [ground(), box([10, 1.5, 6], [6, 1.5, 0.12]), box([10, 1.5, 9], [6, 1.5, 0.12])], nf = tried(f).length;
  check(nd === 0 && ne === 0 && ng === 0 && nf === 1, 'no bite where something tall stands against the wall, where a lamp or an alarm hangs, or near another bite', `posts along the wall ${nd}, lamps ${ne}, alarm housings ${ng}, two walls three metres apart ${nf}`);
}

// ---- the levels as their files have them (each in the look the page loads it in: scene/look.ts LOOK_LEVELS)
const R = `${ROOT}/levels`;
const load = (name: string): { L: B[]; lights: { pos: number[] }[]; markTexel?: number } => { const f = (LOOK_LEVELS as Record<string, string>)[`${name}.json`] ?? `${name}.json`;
  const lv = loadLevel(parseLevelData(readFileSync(`${R}/${f}`, 'utf8')), { lived: true, dressing: true }); return { L: lv.boxes as B[], lights: lv.lights ?? [], markTexel: lv.markTexel }; };
const WANT: Record<string, [rounds: number, walls: number]> = { pass: [19, 3], compound: [3, 4], crossing: [28, 6], 'police-post': [19, 5], bazaar: [6, 3], depot: [0, 2], 'tut-pair': [35, 4], 'tut-solo': [29, 5], block: [0, 0], records: [0, 0] };
const got: string[] = [], off: string[] = [], low: string[] = [], near: string[] = [], indoors: string[] = [], hung: string[] = [], holed: string[] = [], walls: Record<string, number> = {}; let all = 0;
for (const name of Object.keys(WANT)) {
  const { L, lights, markTexel } = load(name), before = L.map(b => ({ ...b })), n0 = L.length, m0 = placeMask(before, []) as Uint8Array;
  const r = geoTryLevel({ boxes: L, lights, markTexel }, BRK ? () => true : skyPlan(before)) as { rounds: number; walls: number; bites: Bite[] }; all += r.walls; walls[name] = r.walls;
  got.push(`${name} ${r.rounds} round, ${r.walls} bites, ${L.length - n0} boxes more`);
  if (r.rounds !== WANT[name][0] || r.walls !== WANT[name][1]) off.push(`${name}: ${r.rounds} round things and ${r.walls} bites, not ${WANT[name].join(' and ')}`);
  for (const q of r.bites) { const say = `${name} (${q.x.toFixed(1)}, ${q.z.toFixed(1)})`, st = stretches(L, q);
    if ([-1, 1].some(sd => [0.4, 0.8].some(dd => [q.s0, (q.s0 + q.s1) / 2, q.s1].some(u => roofOver(before, q.alongX ? u : q.x + sd * (q.T + dd), q.alongX ? q.z + sd * (q.T + dd) : u))))) indoors.push(say);
    if (q.H - q.D < 2 - 1e-6 || q.D > 0.4 + 1e-6 || st.length < 3 || st.some(b => b.c[1] + b.h[1] < q.H - q.D - 1e-3 || b.c[1] - b.h[1] > 1e-3)) low.push(`${say} to ${(q.H - q.D).toFixed(2)} m in ${st.length} stretches`);
    if (r.bites.some(o => o !== q && Math.hypot(o.x - q.x, o.z - q.z) < 10)) near.push(say);
    if (inGap(L, q).length) hung.push(`${say}: ${inGap(L, q).length}`);
  }
  // the plan the light of a place is kept apart by: a bite is still wall on it (a stretch under 0.4 m long was a post to it, and the bite a gap)
  const m1 = placeMask(L, []) as Uint8Array; let freed = 0;
  for (let k = 0; k < m0.length; k++) { if (m0[k] !== 255 || m1[k] === 255) continue; const x = WORLD.rcMin[0] + (k % PLACE_W + 0.5) / PLACE_RES, z = WORLD.rcMin[2] + (Math.floor(k / PLACE_W) + 0.5) / PLACE_RES;
    if (r.bites.some(q => { const u = q.alongX ? x : z, v = q.alongX ? z - q.z : x - q.x; return u > q.s0 - 0.01 && u < q.s1 + 0.01 && Math.abs(v) < 0.45; })) freed++; }   // (a cell in a bite's own stretch of the wall; a wall's far end that falls on a cell's very middle is the split's rounding)
  if (freed) holed.push(`${name}: ${freed} cells`);
  if (L.some(b => b.cutRound && b.cut?.length !== 8)) off.push(`${name}: a round thing that is not a prism of twelve sides`);
}
check(off.length === 0, 'every level gets what its own boxes say: the kit\'s octagons made round, its yard walls broken', off.length ? off.join('; ') : got.join(' · '));
check(indoors.length === 0 && all > 0, 'no bite is in a wall with a roof on either hand (a room keeps its walls)', indoors.length ? indoors.join('; ') : `${all} bites, no slab over head height two cells out on either hand`);
check(low.length === 0 && near.length === 0, 'no bite goes under the 2 m line or deeper than 0.4 m, and none is nearer than 10 m to another', [...low, ...near].join('; '));
check(hung.length === 0, 'nothing drawn is left in a bite\'s gap or hanging on the wall\'s face beside it (a coping, snow, an alarm\'s housing)', hung.join('; '));
check(holed.length === 0, 'a bite is still wall on the floor plan the light of a place is kept apart by: no cell of it is free floor after the trial', holed.join('; '));
check(walls.compound > 0 && walls.depot > 0, 'a level built before the kit laid copings has broken walls too', `the compound ${walls.compound}, the depot ${walls.depot}`);

console.log(red ? '=== geotry: FAILED ===' : '=== geotry: ALL PASS ===');
process.exit(red ? 1 : 0);
