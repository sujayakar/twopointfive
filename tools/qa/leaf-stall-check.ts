// A teammate walking on the spot against a leaf nobody can go round has it out of his way at once (game/doorway.ts DOORMANNERS.leafStall, leafStalled).
//   bun run tools/qa/leaf-stall-check.ts [--verbose]      (probes.ts leafstall, in family squadorders; LEAFSTALL_BREAK=1 must FAIL)
// Found on compound-four as shipped, in a replay of the maintainer's: the office door c_n stood open into the corridor, which with the lived-in rooms'
// things in it is narrower than the leaf and a man. A man sent through that door from along the corridor met the back of the leaf; `leafRound` slid him to
// its free end, there was no floor there, and AT THE TEAM'S WALK (teampace.ts) his touch on the leaf came and went — so the leaf's own count of an unbroken
// press (doors.ts ROUND_SECS) did not run out, and he walked on the spot against it for four seconds, the rest of the team held behind him. Twice in one
// five-minute run. (At the pace men had before the team's walk the press is unbroken and the leaf turns after its 1.2 s: measured here, 0.7 s on the spot.)
//   1. THE FOUND CASE, staged the same way on either stand: c_n stood open at 1.84 rad, one man in the corridor sent to the room beyond it, the other three
//      held on the start, 12 s. Rule off: he walks on the spot (his walk on, his feet within 12 cm over the last 0.6 s) for a second and a half or more.
//      Rule on: under a second of it in all, the rule counted him, and he is through the frame no later than with the rule off.
//   2. ONLY THE LEAF THAT HOLDS HIM: the rule's own question, asked for a second of a man kept on one spot by the same leaf with his walk on. At its back
//      and walking at it: yes. At its back and walking away from it (a mate is what holds him), and at its front walking at it (his body opens it wider):
//      never. And a man sent the other way down the corridor is not counted, and the leaf stands where it was.
//   3. the journal: a recording from before the key replays with it off; one that carries it keeps its own word, on or off
// The doorway manners, 'doors stay where left', one-way doors, the lived-in rooms and the team's walk are raised for the staging on either stand (the
// frozen stand holds them off; the page has them): a leaf that swings both ways, or that its closer brings home, is never in a man's way like this, the
// bare corridor has room to go round it, and at the old pace the leaf's own count runs out.
// Headless: nothing here says how it looks.
import { ROOT } from './headless.ts';
import { guarded, family } from './thresholdmenu-probes.ts';
const { rig } = (await import('./outerwall-probes.ts')) as any;
const { DOORMANNERS, DOORMANNERS_STATS, leafStalled } = await import(`${ROOT}/src/game/doorway.ts`);
const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { WALLCHARGE } = await import(`${ROOT}/src/game/wallcharge.ts`);
const { DOORSTAY, DOORWAY } = await import(`${ROOT}/src/game/doors.ts`);
const { TEAMPACE } = await import(`${ROOT}/src/game/teampace.ts`);
const { LIVED } = await import(`${ROOT}/src/scene/lived.ts`);
const VERBOSE = process.argv.includes('--verbose');
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);

const OPEN = 1.84, FROM = [29.3, 0, 15.3], BEYOND = [30.75, 0, 12.75], BY_LEAF = [29.72, 0, 15.4], WEST = [27.4, 0, 15.3];
interface Case { why: string | null; spot: number; thru: number; counted: number; least: number; end: number[] }
/** one man by c_n's open leaf — `what`: sent through the door, told to hold by the leaf, or sent away from it. `spot`: seconds his walk was on with his feet
 *  going nowhere · `thru`: seconds until he is through the frame (NaN: never) · `counted`: times the rule took him for stalled · `least`: the leaf's least angle */
async function leaf(on: boolean, what: 'thru' | 'hold' | 'away', secs: number): Promise<Case> {
  DOORWAY.oneWay = true; LIVED.on = true; TEAMPACE.on = true;   // (before the level is built: the leaves take their stops from the one, the corridor its things from the other)
  const R = await rig('compound-four', 5), g = R.game, O = R.O as any; DOORSTAY.on = true; TEAMPACE.on = true; DOORMANNERS.on = true; DOORMANNERS.leafRound = true; DOORMANNERS.leafStall = on;   // (after: standing a game up puts the stand's own rows back)
  for (const h of R.hostiles) if (h.state !== 'dead') R.put(h, [h.char.pos[0] + 400, 0, h.char.pos[2]]);
  const d = g.doors.list.find((x: any) => x.def.name === 'c_n'), men: any[] = O.squad().filter((x: any) => x.state !== 'dead'), man = men[0], idx = O.squad().indexOf(man);
  const open = () => { d.latched = false; d.closing = false; d.locked = false; d.angle = OPEN; d.vel = 0; d.pullT = 0; d.pressT = 0; d.passers = 0; d.openT = 10; d.place?.(); };
  O.give('hold', O.targetAt(g.player.char.pos), 'all'); R.step(3);
  open(); R.put(man, what === 'hold' ? BY_LEAF : FROM); R.step(2); open();
  const n0 = DOORMANNERS_STATS.leafStall, to = what === 'thru' ? BEYOND : what === 'hold' ? BY_LEAF : WEST;
  const why = O.give(what === 'hold' ? 'hold' : 'move', { kind: 'floor', at: to }, idx);
  const trail: number[][] = []; let spot = 0, thru = NaN, least = Math.abs(d.angle);
  for (let f = 0; f < secs * 60; f++) {
    R.step(1); const p = man.char.pos; trail.push([p[0], 0, p[2]]); least = Math.min(least, Math.abs(d.angle));
    if (man.speed > 0.5 && trail.length > 36 && flat(trail[trail.length - 37], p) < 0.12) spot++;
    if (isNaN(thru) && p[2] < 14.2 && p[0] > 30.3) thru = (f + 1) / 60;
  }
  return { why, spot: spot / 60, thru, counted: DOORMANNERS_STATS.leafStall - n0, least, end: [man.char.pos[0], 0, man.char.pos[2]] };
}

/** 2: the rule's own question, asked every frame for a second of a man kept at `at` (told to hold: the game itself does not ask it of a man who stands) with
 *  his walk given as `vel` — did it once say the leaf is his to push? */
async function asked(at: number[], vel: number[]): Promise<boolean> {
  DOORWAY.oneWay = true; LIVED.on = true; TEAMPACE.on = true;
  const R = await rig('compound-four', 5), g = R.game, O = R.O as any; DOORSTAY.on = true; TEAMPACE.on = true; DOORMANNERS.on = true; DOORMANNERS.leafRound = true; DOORMANNERS.leafStall = true;
  for (const h of R.hostiles) if (h.state !== 'dead') R.put(h, [h.char.pos[0] + 400, 0, h.char.pos[2]]);
  const d = g.doors.list.find((x: any) => x.def.name === 'c_n'), man = O.squad().filter((x: any) => x.state !== 'dead')[0];
  O.give('hold', O.targetAt(g.player.char.pos), 'all'); R.step(3); let yes = false;
  for (let f = 0; f < 60; f++) {
    d.latched = false; d.closing = false; d.angle = OPEN; d.vel = 0; d.place?.(); R.put(man, at); R.step(1); R.put(man, at);
    const s0 = man.speed, v0 = man.char.vel; man.speed = Math.hypot(vel[0], vel[2]); man.char.vel = [vel[0], 0, vel[2]];
    if (leafStalled(g, man)) yes = true; man.speed = s0; man.char.vel = v0;
  }
  return yes;
}

export const leafstallCheck = guarded(async () => {
  const F = family('leafstall: a leaf nobody can go round — no walking on the spot against it');
  const keep = { m: DOORMANNERS.on, r: DOORMANNERS.leafRound, s: DOORMANNERS.leafStall, st: DOORSTAY.on, ow: DOORWAY.oneWay, lv: LIVED.on, tp: TEAMPACE.on, rnd: Math.random, d: DESTRUCTION.on, w: WALLCHARGE.on, o: WALLCHARGE.outer };
  try {
    // ---- 1. the found case
    { const off = await leaf(false, 'thru', 12), on = await leaf(true, 'thru', 12);
      const say = (c: Case) => `${c.why ? `refused: ${c.why} · ` : ''}on the spot ${c.spot.toFixed(2)} s · through the frame ${isNaN(c.thru) ? 'never in 12 s' : `at ${c.thru.toFixed(1)} s`} · the leaf from ${OPEN} to ${c.least.toFixed(2)} rad at the least · counted ${c.counted} · ends (${c.end[0].toFixed(2)}, ${c.end[2].toFixed(2)})`;
      if (VERBOSE) { console.log(`  off: ${say(off)}`); console.log(`  on:  ${say(on)}`); }
      F.verdict(!off.why && off.spot >= 1.5, 'the found case, rule off: sent through the office door from along the corridor, he walks on the spot against the back of its open leaf (a second and a half or more)', say(off));
      F.verdict(!on.why && on.spot < 1.0 && on.counted >= 1 && !isNaN(on.thru) && on.thru <= (isNaN(off.thru) ? 12 : off.thru) + 0.25, 'the found case, rule on: under a second on the spot in all, the rule took the leaf for his to push, and he is through the frame no later than with the rule off', `${say(on)} (rule off: on the spot ${off.spot.toFixed(2)} s, through ${isNaN(off.thru) ? 'never' : `at ${off.thru.toFixed(1)} s`})`); }
    // ---- 2. only the leaf that holds him
    { const at = await asked([29.73, 0, 15.40], [1.2, 0, -0.2]), off = await asked([29.73, 0, 15.40], [-1.2, 0, 0.2]), front = await asked([30.6, 0, 15.0], [-1.2, 0, 0]), away = await leaf(true, 'away', 4);
      F.verdict(at && !off && !front && away.counted === 0 && away.least > OPEN - 0.05 && away.end[0] < 28.2, 'rule on, a man kept on one spot by the leaf with his walk on: at its back walking at it the leaf is his to push; at its back walking away from it, and at its front, it never is; and a man sent the other way down the corridor is not counted', `at its back, walking at it: ${at} · at its back, walking away: ${off} · at its front, walking at it: ${front} · sent away: counted ${away.counted}, leaf least ${away.least.toFixed(2)} rad, ends (${away.end[0].toFixed(2)}, ${away.end[2].toFixed(2)})`); }
    // ---- 3. the journal
    { const J = await import(`${ROOT}/src/game/journal.ts`), R = await rig('room-clear-four', 5), snap = () => JSON.parse(JSON.stringify(J.knobsSnapshot(R.game)));
      DOORMANNERS.leafStall = true; const old = snap(); if (old.tables.DOORMANNERS) delete old.tables.DOORMANNERS.leafStall; J.applyKnobs(R.game, old); const a = DOORMANNERS.leafStall;
      DOORMANNERS.leafStall = true; const yes = snap(); DOORMANNERS.leafStall = false; J.applyKnobs(R.game, yes); const b = DOORMANNERS.leafStall;
      DOORMANNERS.leafStall = false; const no = snap(); DOORMANNERS.leafStall = true; J.applyKnobs(R.game, no); const c = DOORMANNERS.leafStall;
      F.verdict(a === false && b === true && c === false, 'the journal: a recording from before the key replays with it off, one made with it on replays on, one made with it off replays off', `old → ${a} · recorded on → ${b} · recorded off → ${c}`); }
  } finally { DOORMANNERS.on = keep.m; DOORMANNERS.leafRound = keep.r; DOORMANNERS.leafStall = keep.s; DOORSTAY.on = keep.st; DOORWAY.oneWay = keep.ow; LIVED.on = keep.lv; TEAMPACE.on = keep.tp; Math.random = keep.rnd; DESTRUCTION.on = keep.d; WALLCHARGE.on = keep.w; WALLCHARGE.outer = keep.o; }
  return F.done();
});

if (import.meta.main) process.exit((await leafstallCheck()) ? 0 : 1);
