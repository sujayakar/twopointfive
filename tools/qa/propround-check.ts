// A teammate's way goes round a chair where there is floor to (game/orders.ts ORDERS.propRound, propPenalty; asked by game/guards.ts goTo).
//   bun run tools/qa/propround-check.ts [--verbose]      (probes.ts propround, in family squadorders; PROPROUND_BREAK=1 must FAIL)
// Found on compound-four as shipped, in a replay of the maintainer's: a chair of the lived-in rooms stood a stride and a half short of the sala's side door
// a_s, on the straight way to it. The walk grid is the statics alone, so one man's way ran through the chair: his body shoved it ahead of him into the
// doorway, where it stood pinned against the jamb, and the next man walked on the spot against it for four and a half seconds until a third pushed it through.
//   1. THE FOUND CASE, staged the same way on either stand: that chair stood where the replay had it (it had been shoved once already), one man put where
//      the replay's man came from and sent to the floor in front of the door, the other three held, 10 s. Rule off: the chair is shoved 0.3 m or more. Rule
//      on: it is not moved (5 cm at most), and he is there no more than a second and a half later than with the rule off.
//   2. THE ONLY WAY: the same chair stood square in front of the doorway, the man sent through from straight behind it. Rule on: he is not kept out — the
//      chair is pushed through and he is beyond the door inside 12 s.
//   3. EVERY OTHER WAY: the ways goTo lays for the four from where they start to every door of the level, rule off and on: some pass a movable thing, and
//      one that passes none is the same length with the rule on.
//   4. NOT THE ENEMY: with the rule on a hostile's search is handed no price of this rule's; a teammate's is.
//   5. the journal: a recording from before the key replays with it off; one that carries it keeps its own word, on or off
// The lived-in rooms are raised for the staging on either stand (the frozen stand holds them off; the page has them): the chair is theirs.
// Headless: nothing here says how it looks.
import { ROOT } from './headless.ts';
import { guarded, family } from './thresholdmenu-probes.ts';
const { rig } = (await import('./outerwall-probes.ts')) as any;
const { ORDERS, propPenalty } = await import(`${ROOT}/src/game/orders.ts`);
const { goTo } = await import(`${ROOT}/src/game/guards.ts`);
const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { WALLCHARGE } = await import(`${ROOT}/src/game/wallcharge.ts`);
const { LIVED } = await import(`${ROOT}/src/scene/lived.ts`);
const VERBOSE = process.argv.includes('--verbose');
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);

const CHAIR = [22.76, 0, 10.12], FROM = [22.55, 0, 9.04], FRONT = [23.25, 0, 10.9], BEYOND = [22.46, 0, 12.13];   // (where the replay had the chair and the man who walked through it; the floor in front of the door he was making for; the room beyond it)
async function stage(on: boolean) {
  LIVED.on = true;   // (before the level is built: the chair is one of the lived-in rooms' things)
  const R = await rig('compound-four', 5), g = R.game, O = R.O as any; ORDERS.propRound = on;
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
async function walk(on: boolean, at: number[], from: number[], to: number[], secs: number): Promise<Case> {
  const { R, O, chair, man, idx } = await stage(on);
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

/** 3: goTo's ways from where the four start to every door of the level — [asked, pass a movable thing with the rule off, pass none but differ in length] */
async function ways(): Promise<[number, number, number]> {
  const { g, O } = await stage(false), nav = g.col.nav, men: any[] = O.squad().filter((x: any) => x.state !== 'dead'); let asked = 0, dear = 0, differ = 0;
  const len = (a: number[], w: number[][]) => { let l = 0, q = a; for (const c of w) { l += flat(q, c); q = c; } return l; };
  for (const m of men) for (const d of g.doors.list) {
    const to = nav.nearestFreePoint(d.frameCentre[0], d.frameCentre[2]); if (!to) continue;
    ORDERS.propRound = false; goTo(g, m, to, true); const a: number[][] = m.path.map((p: number[]) => [p[0], 0, p[2]]); if (!a.length) continue;
    ORDERS.propRound = true; const price = propPenalty(g, m, undefined); goTo(g, m, to, true); const b: number[][] = m.path.map((p: number[]) => [p[0], 0, p[2]]);
    asked++; let n = 0, q: number[] = m.char.pos; if (price) for (const c of a) { n += nav.dearAlong(q, c, price); q = c; }
    if (n > 0) dear++; else if (Math.abs(len(m.char.pos, a) - len(m.char.pos, b)) > 0.01) differ++;
    m.path = []; m.pathGoal = null;
  }
  return [asked, dear, differ];
}

export const proproundCheck = guarded(async () => {
  const F = family('propround: a teammate walks round a chair where there is floor to');
  const keep = { p: ORDERS.propRound, lv: LIVED.on, rnd: Math.random, d: DESTRUCTION.on, w: WALLCHARGE.on, o: WALLCHARGE.outer };
  try {
    // ---- 1. the found case
    { const off = await walk(false, CHAIR, FROM, FRONT, 10), on = await walk(true, CHAIR, FROM, FRONT, 10);
      const say = (c: Case) => `${c.why ? `refused: ${c.why} · ` : ''}the chair shoved ${c.moved.toFixed(2)} m at the most · there ${isNaN(c.thru) ? 'never in 10 s' : `at ${c.thru.toFixed(1)} s`} · on the spot ${c.spot.toFixed(2)} s`;
      if (VERBOSE) { console.log(`  off: ${say(off)}`); console.log(`  on:  ${say(on)}`); }
      F.verdict(!off.why && off.moved >= 0.3, 'the found case, rule off: sent to the floor in front of the sala side door, his way runs through the chair and his body shoves it (0.3 m or more)', say(off));
      F.verdict(!on.why && on.moved <= 0.05 && !isNaN(on.thru) && on.thru <= (isNaN(off.thru) ? 10 : off.thru) + 1.5, 'the found case, rule on: the chair is not moved (5 cm at most), and he is there no more than a second and a half later than with the rule off', `${say(on)} (rule off: shoved ${off.moved.toFixed(2)} m, there ${isNaN(off.thru) ? 'never' : `at ${off.thru.toFixed(1)} s`})`); }
    // ---- 2. the only way
    { const on = await walk(true, [23.3, 0, 11.0], [23.3, 0, 9.6], BEYOND, 12);
      F.verdict(!on.why && !isNaN(on.thru) && on.moved > 0.2, 'rule on, the chair stood square in front of the doorway and the man sent through from behind it: he is not kept out — the chair is pushed and he is beyond the door inside 12 s', `the chair pushed ${on.moved.toFixed(2)} m · beyond the door ${isNaN(on.thru) ? 'never in 12 s' : `at ${on.thru.toFixed(1)} s`} · on the spot ${on.spot.toFixed(2)} s`); }
    // ---- 3. every other way
    { const [asked, dear, differ] = await ways();
      F.verdict(asked > 40 && dear > 0 && dear < asked && differ === 0, "the ways laid for the four to every door of the level: some pass a movable thing, and a way that passes none is the same length with the rule on", `${asked} ways · ${dear} pass a movable thing with the rule off · ${differ} that pass none differ in length`); }
    // ---- 4. not the enemy
    { const { g, R, man } = await stage(true), foe = R.hostiles.find((h: any) => h.state !== 'dead'), a = propPenalty(g, man, undefined), b = foe ? propPenalty(g, foe, undefined) : 'no hostile';
      F.verdict(typeof a === 'function' && b === undefined, "rule on: a teammate's search is handed this rule's price, a hostile's is handed none", `teammate: ${typeof a} · hostile: ${typeof b === 'string' ? b : typeof b} · ${g.props.props.length} movable things on the level`); }
    // ---- 5. the journal
    { const J = await import(`${ROOT}/src/game/journal.ts`), R = await rig('room-clear-four', 5), snap = () => JSON.parse(JSON.stringify(J.knobsSnapshot(R.game)));
      ORDERS.propRound = true; const old = snap(); if (old.tables.ORDERS) delete old.tables.ORDERS.propRound; J.applyKnobs(R.game, old); const a = ORDERS.propRound;
      ORDERS.propRound = true; const yes = snap(); ORDERS.propRound = false; J.applyKnobs(R.game, yes); const b = ORDERS.propRound;
      ORDERS.propRound = false; const no = snap(); ORDERS.propRound = true; J.applyKnobs(R.game, no); const c = ORDERS.propRound;
      F.verdict(a === false && b === true && c === false, 'the journal: a recording from before the key replays with it off, one made with it on replays on, one made with it off replays off', `old → ${a} · recorded on → ${b} · recorded off → ${c}`); }
  } finally { ORDERS.propRound = keep.p; LIVED.on = keep.lv; Math.random = keep.rnd; DESTRUCTION.on = keep.d; WALLCHARGE.on = keep.w; WALLCHARGE.outer = keep.o; }
  return F.done();
});

if (import.meta.main) process.exit((await proproundCheck()) ? 0 : 1);
