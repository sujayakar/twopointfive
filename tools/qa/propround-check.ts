// A teammate's way goes round a chair where there is floor to (game/orders.ts ORDERS.propRound, propPenalty; asked by game/guards.ts goTo).
//   bun run tools/qa/propround-check.ts [--verbose]      (probes.ts propround, in family squadorders)
// Found on compound-four as shipped, in a replay of the maintainer's: a chair of the lived-in rooms stood a stride and a half short of the sala's side door
// a_s, on the straight way to it. The walk grid is the statics alone, so one man's way ran through the chair: his body shoved it ahead of him into the
// doorway, where it stood pinned against the jamb, and the next man walked on the spot against it for four and a half seconds until a third pushed it through.
//   1. the found case: that chair stood where the replay had it, one man put where the replay's man came from and sent to the floor in front of the door, 10 s:
//      the chair is not moved (5 cm at most) and he gets there.
//   2. the only way: the same chair stood square in front of the doorway, the man sent through from straight behind it: he is not kept out, the chair is pushed
//      through and he is beyond the door inside 12 s.
//   3. not the enemy: a hostile's search is handed no price of this rule's, a teammate's is.
// Headless: nothing here says how it looks.
import { ROOT } from './headless.ts';
import { guarded, family } from './thresholdmenu-probes.ts';
const { rig } = (await import('./outerwall-probes.ts')) as any;
const { propPenalty } = await import(`${ROOT}/src/game/orders.ts`);
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);

const CHAIR = [22.76, 0, 10.12], FROM = [22.55, 0, 9.04], FRONT = [23.25, 0, 10.9], BEYOND = [22.46, 0, 12.13];   // (where the replay had the chair and the man who walked through it; the floor in front of the door he was making for; the room beyond it)
async function stage() {
  const R = await rig('compound-four', 5), g = R.game, O = R.O as any;
  for (const h of R.hostiles) if (h.state !== 'dead') R.put(h, [h.char.pos[0] + 400, 0, h.char.pos[2]]);
  const chair = [...g.props.props].sort((a: any, b: any) => Math.hypot(a.x - CHAIR[0], a.z - CHAIR[2]) - Math.hypot(b.x - CHAIR[0], b.z - CHAIR[2]))[0];
  const men: any[] = O.squad().filter((x: any) => x.state !== 'dead'), man = men[0], idx = O.squad().indexOf(man);
  O.give('hold', O.targetAt(g.player.char.pos), 'all'); R.step(3);
  return { R, g, O, chair, man, idx };
}
interface Case { why: string | null; moved: number; thru: number; spot: number }
/** one man sent past the chair — `at`: where the chair is stood first · `from`: where he starts · `to`: where he is sent.
 *  `moved`: the farthest the chair gets from where it stood · `thru`: seconds until he
 *  is there (beyond the door, or within half a metre of the spot; NaN: never) · `spot`: seconds his walk was on with his feet going nowhere */
async function walk(at: number[], from: number[], to: number[], secs: number): Promise<Case> {
  const { R, O, chair, man, idx } = await stage();
  { chair.x = at[0]; chair.z = at[2]; chair.vx = 0; chair.vz = 0; chair.w = 0; chair.wake?.(); chair.place?.(); }
  R.put(man, from); R.step(2); const c0 = [chair.x, 0, chair.z];
  const why = O.give('move', { kind: 'floor', at: to }, idx); const trail: number[][] = []; let moved = 0, thru = NaN, spot = 0;
  for (let f = 0; f < secs * 60; f++) {
    R.step(1); const p = man.char.pos; trail.push([p[0], 0, p[2]]); moved = Math.max(moved, flat([chair.x, 0, chair.z], c0));
    if (man.speed > 0.5 && trail.length > 36 && flat(trail[trail.length - 37], p) < 0.12) spot++;
    if (isNaN(thru) && (to === BEYOND ? p[2] > 11.9 : flat(p, to) < 0.5)) thru = (f + 1) / 60;
  }
  return { why, moved, thru, spot: spot / 60 };
}

export const proproundCheck = guarded(async () => {
  const F = family('propround: a teammate walks round a chair where there is floor to');
  const keep = { rnd: Math.random };
  try {
    // ---- 1. the found case
    { const on = await walk(CHAIR, FROM, FRONT, 10);
      F.verdict(!on.why && on.moved <= 0.05 && !isNaN(on.thru), 'the found case: sent to the floor in front of the sala side door, the chair is not moved (5 cm at most), and he gets there', `the chair shoved ${on.moved.toFixed(2)} m at the most · there ${isNaN(on.thru) ? 'never in 10 s' : `at ${on.thru.toFixed(1)} s`} · on the spot ${on.spot.toFixed(2)} s`); }
    // ---- 2. the only way
    { const on = await walk([23.3, 0, 11.0], [23.3, 0, 9.6], BEYOND, 12);
      F.verdict(!on.why && !isNaN(on.thru) && on.moved > 0.2, 'the chair stood square in front of the doorway and the man sent through from behind it: he is not kept out — the chair is pushed and he is beyond the door inside 12 s', `the chair pushed ${on.moved.toFixed(2)} m · beyond the door ${isNaN(on.thru) ? 'never in 12 s' : `at ${on.thru.toFixed(1)} s`} · on the spot ${on.spot.toFixed(2)} s`); }
    // ---- 4. not the enemy
    { const { g, R, man } = await stage(), foe = R.hostiles.find((h: any) => h.state !== 'dead'), a = propPenalty(g, man, undefined), b = foe ? propPenalty(g, foe, undefined) : 'no hostile';
      F.verdict(typeof a === 'function' && b === undefined, "a teammate's search is handed this rule's price, a hostile's is handed none", `teammate: ${typeof a} · hostile: ${typeof b === 'string' ? b : typeof b} · ${g.props.props.length} movable things on the level`);
    }
  } finally { Math.random = keep.rnd; }
  return F.done();
});

if (import.meta.main) process.exit((await proproundCheck()) ? 0 : 1);
