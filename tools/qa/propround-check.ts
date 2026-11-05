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
//   3. EVERY OTHER WAY: the ways goTo lays for the four from where they start to every door of the level, rule off and on: some pass a movable thing and
//      some of those are taken round it; one that passes none is the same length with the rule on; with the rule on every way is laid, none is more
//      than `propRoundMore` (2 m) longer than the way over the statics alone, and none crosses a door the plain way does not (or misses one it does).
//   4. NOT THE ENEMY, NOT A MAN GOING IN: with the rule on a hostile's search is handed no price of this rule's, a teammate's is; and a teammate with the
//      fight's pace on his legs (teampace.ts teamFight: an entry, a step of the room fight's), or one a drill has taken (the night breach's F: its marks
//      were set on the plain ways), is laid the very way he is with the rule off.
//      And a thing more than `propRoundDoor` from every door's frame is not priced: the same chair on open floor, the man sent straight over it.
//   5. the journal: a recording from before the key replays with it off; one that carries it keeps its own word, on or off
// The lived-in rooms and the team's walk are raised for the staging on either stand (the frozen stand holds them off; the page has them): the chair is
// the one's, and the rule is for a man at the other's walk.
// Headless: nothing here says how it looks.
import { ROOT } from './headless.ts';
import { guarded, family } from './thresholdmenu-probes.ts';
const { rig } = (await import('./outerwall-probes.ts')) as any;
const { ORDERS, propPenalty } = await import(`${ROOT}/src/game/orders.ts`);
const { goTo, crossesDoor } = await import(`${ROOT}/src/game/guards.ts`);
const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { WALLCHARGE } = await import(`${ROOT}/src/game/wallcharge.ts`);
const { LIVED } = await import(`${ROOT}/src/scene/lived.ts`);
const { TEAMPACE, teamFight, teamAtWalk } = await import(`${ROOT}/src/game/teampace.ts`);
const VERBOSE = process.argv.includes('--verbose');
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);

const CHAIR = [22.76, 0, 10.12], FROM = [22.55, 0, 9.04], FRONT = [23.25, 0, 10.9], BEYOND = [22.46, 0, 12.13];   // (where the replay had the chair and the man who walked through it; the floor in front of the door he was making for; the room beyond it)
async function stage(on: boolean) {
  LIVED.on = true; TEAMPACE.on = true; ORDERS.propRound = on;   // (before the level is built: the chair is one of the lived-in rooms' things)
  const R = await rig('compound-four', 5), g = R.game, O = R.O as any; TEAMPACE.on = true; ORDERS.propRound = on;   // (after: standing a game up puts the stand's own rows back)
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

/** 3: goTo's ways from where the four start to every door of the level — [asked, pass a movable thing with the rule off, pass none but differ in length,
 *  with the rule on none laid or more than `propRoundMore` the longer, taken round] */
async function ways(): Promise<[number, number, number, number, number]> {
  const { g, O } = await stage(false), nav = g.col.nav, men: any[] = O.squad().filter((x: any) => x.state !== 'dead'); let asked = 0, dear = 0, differ = 0, bad = 0, round = 0;
  const len = (a: number[], w: number[][]) => { let l = 0, q = a; for (const c of w) { l += flat(q, c); q = c; } return l; };
  for (const m of men) for (const d of g.doors.list) {
    const to = nav.nearestFreePoint(d.frameCentre[0], d.frameCentre[2]); if (!to) continue;
    ORDERS.propRound = false; goTo(g, m, to, true); const a: number[][] = m.path.map((p: number[]) => [p[0], 0, p[2]]); if (!a.length) continue;
    ORDERS.propRound = true; const price = propPenalty(g, m, undefined); goTo(g, m, to, true); const b: number[][] = m.path.map((p: number[]) => [p[0], 0, p[2]]);
    asked++; let n = 0, q: number[] = m.char.pos; if (price) for (const c of a) { n += nav.dearAlong(q, c, price); q = c; }
    const thru = (w: number[][]) => { const out: string[] = []; let q: number[] = m.char.pos; for (const c of w) { for (const o of g.doors.list) if (crossesDoor(q, c, o) && !out.includes(o.def.name)) out.push(o.def.name); q = c; } return out.sort().join(); };
    const la = len(m.char.pos, a), lb = len(m.char.pos, b); if (!b.length || lb > la + ORDERS.propRoundMore + 0.01 || thru(a) !== thru(b)) bad++;
    if (n > 0) { dear++; if (Math.abs(la - lb) > 0.01) round++; } else if (Math.abs(la - lb) > 0.01) differ++;
    m.path = []; m.pathGoal = null;
  }
  return [asked, dear, differ, bad, round];
}

export const proproundCheck = guarded(async () => {
  const F = family('propround: a teammate walks round a chair where there is floor to');
  const keep = { p: ORDERS.propRound, by: ORDERS.propRoundDoor, lv: LIVED.on, tp: TEAMPACE.on, rnd: Math.random, d: DESTRUCTION.on, w: WALLCHARGE.on, o: WALLCHARGE.outer };
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
    { const [asked, dear, differ, bad, round] = await ways();
      F.verdict(asked > 40 && dear > 0 && dear < asked && round > 0 && differ === 0 && bad === 0, "the ways laid for the four to every door of the level: some pass a movable thing and some of those are taken round it; a way that passes none is the same length with the rule on; and with the rule on every way is laid, none is more than the knob's two metres the longer and each crosses the very doors its plain way does", `${asked} ways · ${dear} pass a movable thing with the rule off, ${round} of them taken round it · ${differ} that pass none differ in length · ${bad} not laid, too long or by other doors`); }
    // ---- 4. not the enemy
    { const { g, R, man } = await stage(true), foe = R.hostiles.find((h: any) => h.state !== 'dead'), a = propPenalty(g, man, undefined), b = foe ? propPenalty(g, foe, undefined) : 'no hostile';
      F.verdict(typeof a === 'function' && b === undefined, "rule on: a teammate's search is handed this rule's price, a hostile's is handed none", `teammate: ${typeof a} · hostile: ${typeof b === 'string' ? b : typeof b} · ${g.props.props.length} movable things on the level`);
      const way = () => { goTo(g, man, FRONT, true); const w = JSON.stringify(man.path.map((p: number[]) => [+p[0].toFixed(3), +p[2].toFixed(3)])); man.path = []; man.pathGoal = null; return w; };
      R.put(man, FROM); const chair = [...g.props.props].sort((x: any, y: any) => Math.hypot(x.x - CHAIR[0], x.z - CHAIR[2]) - Math.hypot(y.x - CHAIR[0], y.z - CHAIR[2]))[0]; chair.x = CHAIR[0]; chair.z = CHAIR[2]; chair.place?.();
      const calm = teamAtWalk(g, man), round = way(); ORDERS.propRound = false; const plain = way(); ORDERS.propRound = true;
      const drill = { owns: (m: any) => m === man, release: () => null }; g.orders.owners.push(drill); const taken = way(), price = propPenalty(g, man, undefined); g.orders.owners.splice(g.orders.owners.indexOf(drill), 1);
      teamFight(g, man, 1); const hot = !teamAtWalk(g, man), going = way();
      R.step(90);   // (the fight's pace, a second of it, has run out: he is at the walk again)
      { // a thing far from every door is not this rule's: the same chair stood on open floor more than three metres from any frame, the man sent straight over it
        const nav = g.col.nav, far = (x: number, z: number) => g.doors.list.every((d: any) => Math.hypot(d.frameCentre[0] - x, d.frameCentre[2] - z) > 3.2), by = ORDERS.propRoundDoor; let P: number[] | null = null, fOn = '', fOff = '', fAll = '';
        const over = (c: number[]): string => { chair.x = c[0]; chair.z = c[2]; chair.place?.(); R.put(man, [c[0] - 1.5, 0, c[2]]); goTo(g, man, [c[0] + 1.5, 0, c[2]], true); const w = JSON.stringify(man.path.map((q: number[]) => [+q[0].toFixed(3), +q[2].toFixed(3)])); man.path = []; man.pathGoal = null; return w; };
        // (the first such floor where the staging bites: with the door's reach thrown wide the way over it does go round)
        for (let z = 2; z < nav.H * nav.cell - 2 && !P; z += 0.5) for (let x = 2; x < nav.W * nav.cell - 2 && !P; x += 0.5) {
          if (!far(x, z) || !far(x - 1.5, z) || !far(x + 1.5, z) || !nav.walkable([x - 1.5, 0, z], [x + 1.5, 0, z]) || !g.props.props.every((q: any) => q === chair || Math.hypot(q.x - x, q.z - z) > 2.5)) continue;
          const c = [x, 0, z]; ORDERS.propRound = false; fOff = over(c); ORDERS.propRound = true; ORDERS.propRoundDoor = 99; fAll = over(c); ORDERS.propRoundDoor = by; if (fAll === fOff || fOff === '[]') continue;
          fOn = over(c); P = c; }
        ORDERS.propRound = true; ORDERS.propRoundDoor = by;
        F.verdict(!!P && fOn === fOff && fAll !== fOff, 'rule on, the same chair stood on open floor more than three metres from every door and the man sent straight over it: his way is the very way he is laid with the rule off (and with the knob for the door\'s reach thrown wide it is not: the staging bites)', P ? `the chair at (${P[0].toFixed(1)}, ${P[2].toFixed(1)}) · way the same as the plain way: ${fOn === fOff} · with the reach at 99 m it differs: ${fAll !== fOff}` : 'no such floor found'); }
      F.verdict(calm && hot && round !== plain && going === plain && taken === plain && price === undefined, "rule on, the found case's man and chair: at the walk his way is the way round; with leave to go faster than the walk (the fight's pace) on his legs, or taken by a drill, it is the very way he is laid with the rule off", `at the walk: ${calm}, way round differs from the plain way: ${round !== plain} · with the fight's pace: not at the walk ${hot}, way the same as the plain way: ${going === plain} · a drill's man: way the same as the plain way ${taken === plain}, price ${typeof price}`); }
    // ---- 5. the journal
    { const J = await import(`${ROOT}/src/game/journal.ts`), R = await rig('room-clear-four', 5), snap = () => JSON.parse(JSON.stringify(J.knobsSnapshot(R.game)));
      ORDERS.propRound = true; const old = snap(); if (old.tables.ORDERS) delete old.tables.ORDERS.propRound; J.applyKnobs(R.game, old); const a = ORDERS.propRound;
      ORDERS.propRound = true; const yes = snap(); ORDERS.propRound = false; J.applyKnobs(R.game, yes); const b = ORDERS.propRound;
      ORDERS.propRound = false; const no = snap(); ORDERS.propRound = true; J.applyKnobs(R.game, no); const c = ORDERS.propRound;
      F.verdict(a === false && b === true && c === false, 'the journal: a recording from before the key replays with it off, one made with it on replays on, one made with it off replays off', `old → ${a} · recorded on → ${b} · recorded off → ${c}`); }
  } finally { ORDERS.propRound = keep.p; ORDERS.propRoundDoor = keep.by; LIVED.on = keep.lv; TEAMPACE.on = keep.tp; Math.random = keep.rnd; DESTRUCTION.on = keep.d; WALLCHARGE.on = keep.w; WALLCHARGE.outer = keep.o; }
  return F.done();
});

if (import.meta.main) process.exit((await proproundCheck()) ? 0 : 1);
