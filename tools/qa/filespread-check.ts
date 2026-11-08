// 'Fall in', Sam stopped: no two places of the file fall on one spot (game/orders.ts ORDERS.fileSpread, apartSlot).
//   bun run tools/qa/filespread-check.ts [--verbose]      (probes.ts filespread, in family squadorders)
// Found by the wall charge's own check on block-squads: Sam put down at (13.6, 10.3) with no trail behind him and the four told to fall in, Blue One's and Blue
// Two's places came out at (11.69, 7.37) and (11.55, 7.16): the file's places run straight back from Sam past the end of his trail and both stop where the floor does.
//   1. that staging: every man still, no two places within a bare body's width and no two men within 0.6 m, everybody arrived
//   2. no floor and no stopping short: he holds, he is not sent into a mate. room-clear-four's storage corner, Sam at (33.00, 7.03), staged: the storage door stood
//      open, Blue One and Blue Two put in the pocket. Nobody moving, no two places within 0.6 m, everybody arrived
// Headless: nothing here says how it looks.
import { ROOT } from './headless.ts';
import { asProbe } from './holds.ts';   // (run alone, each part runs as the probe the runner calls it by: one stand both ways)
import { guarded, family } from './thresholdmenu-probes.ts';
const { rig } = (await import('./outerwall-probes.ts')) as any;
const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
const { DOORMANNERS } = await import(`${ROOT}/src/game/doorway.ts`);
const VERBOSE = process.argv.includes('--verbose');
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const fmt = (p: number[]) => `(${p[0].toFixed(2)}, ${p[2].toFixed(2)})`;

interface Out { ok: boolean; moved: number[]; movers: number; nearGoals: number; tight: number; bare: number; nearMen: number; far: number; fell: number; arrived: boolean; goals: number[][]; men: number[][]; say: string }
/** Sam put down at `at` with no trail, the squad told to fall in, twenty seconds given, then five watched */
async function stand(scn: string, at: number[], corner: { door?: number[]; men: [number, number[]][] } | null = null): Promise<Out> {
  const R = await rig(scn, 5), g = R.game, O = R.O;
  for (const h of R.hostiles) if (h.state !== 'dead') R.put(h, [h.char.pos[0] + 400, 0, h.char.pos[2]]);
  R.gather(at, 20); R.step(60);
  if (corner) { const cd = corner.door; if (cd) for (const d of g.doors.list as any[]) if (flat(d.frameCentre, cd) < 0.5) { d.latched = false; d.angle = -1.92; }   // (the leaf as the page's own walk in leaves it)
    for (const [i, p] of corner.men) if (R.up[i]) R.put(R.up[i], p); R.step(600); }
  const men = R.up as any[], last = men.map(m => [...m.char.pos]), moved = men.map(() => 0);
  for (let f = 0; f < 300; f++) { R.step(); men.forEach((m, i) => { moved[i] += flat(m.char.pos, last[i]); last[i] = [...m.char.pos]; }); }
  const sam = g.player.char.pos, open = (a: number[], b: number[]) => !g.col.solidBlocked([a[0], 0.45, a[2]], [b[0], 0.45, b[2]]);
  const goals = men.map(m => [...(O.of(m)?.goal ?? m.char.pos)]), pos = men.map(m => [...m.char.pos]);
  let nearGoals = 0, tight = 0, bare = 0, nearMen = 0;   // (nearGoals: on one spot · tight: inside the manners' 'standing on a mate', the bar a man is sent on to his exact place by)
  for (let i = 0; i < men.length; i++) for (let j = i + 1; j < men.length; j++) {
    if (flat(goals[i], goals[j]) < 0.6 - 0.02 && open(goals[i], goals[j])) nearGoals++;   // (the same 2 cm the two bars below are given: the hold steps a man out 5 cm at a time, and a place it puts at 0.60 m reads 0.5999…)
    if (flat(goals[i], goals[j]) < DOORMANNERS.apart - 0.02 && open(goals[i], goals[j])) tight++;
    if (flat(goals[i], goals[j]) < ORDERS.fileTight - 0.02 && open(goals[i], goals[j])) bare++;   // (bare: nearer than the rule's own last bar, a bare body's width — what it asks where the floor has no `fileApart`, and what a held man is stepped out to)
    if (flat(pos[i], pos[j]) < 0.6 && open(pos[i], pos[j])) nearMen++; }
  const movers = moved.filter(x => x > 0.15).length, far = Math.max(...pos.map(p => flat(p, sam))), arrived = men.every(m => !!O.of(m)?.arrived);
  // (fell: how far from Sam the file's own places reach in THIS run before the rule touches one — orders.ts slot(), the trail run straight on and, where a wall
  //  cuts it short, turned through an open door: the rule is judged against the places it was handed, not against another run's Sam, shoved a hand's width the other way)
  const fell = Math.max(0, ...men.map(m => { const A = O.of(m); if (!A || A.post !== null) return 0; try { return flat(O.slot(O.backOf(A.follow) + O.clearBack(A.follow)), sam); } catch { return 0; } }));
  const say = men.map((m, i) => `${m.callsign} ${fmt(pos[i])} → ${fmt(goals[i])} moved ${moved[i].toFixed(2)}`).join(' · ');
  return { ok: men.length > 0, moved, movers, nearGoals, tight, bare, nearMen, far, fell, arrived, goals, men: pos, say };
}

export const filespreadCheck = guarded(async () => {
  const F = family('filespread: fall in, Sam stopped — no two places of the file on one spot');
  const keep = { rnd: Math.random };   // (the rig seeds Math.random)
  try {
    // ---- 1, 2. the staging it was found on
    // (blue put where the frozen stand's own walk in leaves the pair, on either stand: the page's lived-in mess table, and where it stands, decides how the two come to the corner — moved
    //  clear of the stack marks (levels/block-squads.json, 10-13), the walk in ended with one man stopped 'near enough' short of the shared place and the fault not showing, though it is there)
    { const at = [13.6, 0, 10.3], blue = { men: [[2, [11.36, 0, 6.99]], [3, [11.73, 0, 7.44]]] as [number, number[]][] }, on = await stand('block-squads', at, blue);
      if (VERBOSE) console.log(`    on:  ${on.say}`);
      F.verdict(on.ok && on.movers === 0 && on.bare === 0 && on.nearMen === 0 && on.arrived, `every man stands still, no two places nearer than a bare body's width (${ORDERS.fileTight} m: the rebuilt block's corner has no floor for more, and a pair that near sends nobody on while nobody moves), no two men within 0.6 m`, `${on.movers} moving, ${on.bare} pair of places within ${ORDERS.fileTight} m (${on.tight} within 0.85 m) and ${on.nearMen} of men within 0.6 m, furthest ${on.far.toFixed(1)} m · ${on.say}`); }
    // ---- 4. the corner with no floor, staged the same on either stand: the key holds him where he stands
    { const at = [33.0, 0, 7.03], corner = { door: [32.75, 0, 10], men: [[2, [30.89, 0, 4.46]], [3, [31.28, 0, 4.91]]] as [number, number[]][] };
      const on = await stand('room-clear-four', at, corner);
      if (VERBOSE) console.log(`    key on:  ${on.say}`);
      F.verdict(on.ok && on.movers === 0 && on.nearGoals === 0 && on.arrived, 'room-clear-four, the storage corner staged (door open, the last two put in the pocket): he holds where he stands, a step off the mate\'s place — nobody moving, no two places on one spot, everybody arrived', `${on.movers} moving, ${on.nearGoals} pair of places within 0.6 m (within 0.85 m: ${on.tight}), ${on.nearMen} of men within 0.6 m · ${on.say}`); }
  } finally { Math.random = keep.rnd; }
  return F.done();
});

if (import.meta.main) process.exit((await asProbe('filespread', filespreadCheck)) ? 0 : 1);
