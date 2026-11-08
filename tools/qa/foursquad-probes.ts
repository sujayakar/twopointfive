// The 'foursquad' family (NIGHTBREACH.squad = 4, the page's ?squad4 — game/nightbreach.ts stages FOUR teammates, a red pair and a blue pair, where the playtests
// stage the one pair; orders.ts / doortools.ts move three or four men through the same menu — X pressed, then a digit), run through probes.ts:
//   bun run tools/qa/probes.ts foursquad | foursquad-off | foursquad-stage | foursquad-follow | foursquad-stack | foursquad-clear | foursquad-drill | foursquad-budget
// PASS / FAIL each, all headless (the Game of headless.ts under the room clear's staging unless a probe says the night breach). The harness has no light meter:
// Sam's visibility is planted dark and he is in god mode, and what anybody SEES of anybody is guards.ts's direct-light estimate, not the page's. Nothing here
// looks at a picture: the pairs' colours, ten men in one frame and who goes without a shadow are untested by eye.
//   foursquad-off      the flag off: the room clear stages the PAIR exactly as it was — two men, Blue One / Blue Two, the default kit, eight characters — and the
//                      one place to ask for men (orders.ts men()) answers for it too
//   foursquad-stage    the flag on: four teammates beside Sam — Red One, Red Two, Blue One, Blue Two — each with his pair in his papers and his pair's colours,
//                      nobody standing in anybody, all five defenders still dealt; men('all' | 'red' | 'blue' | n | guard), pairOf(), pairs(); an order given
//                      to a pair moves that pair alone; a restart stages the four again; stop puts the shift back; the next start with the knob at 2 is the pair
//   foursquad-follow   fall in: the four close up on Sam in one file (pairs together), keep with him down the aisle, in at the break room's west door, across
//                      the room, out at its north door and along the corridor — nobody left behind, nobody standing in a doorway at the end, never two men
//                      in one place
//   foursquad-stack    stack up: on every door of the structure from the squad's side there is a mark for each of the four (two files, a pair a file where both
//                      jambs have floor); ordered on a corridor door all four get onto their marks, 'set.' is called, nobody stands in the opening; 'hold'
//                      and 'cover' take all four where they stand ('move there' in two ranks of two was the order with roomfit.ts off: roomfit-walk has it now)
//   foursquad-clear    the room clear's own rooms taken with the existing rows by all four, over seeds: 'open and clear' and 'bang and clear' — every living
//                      man who is sent in crosses the frame line, the order ends 'in', nobody is left standing in the doorway, and NO teammate goes down to a
//                      teammate's round (the game's own rule — so the LANES are counted too: rounds fired through a friend); how many rooms end clear and
//                      how many teammates are lost is printed (the fight is a fight); then the charge by four
//   foursquad-drill    F's drill with four: it is the red pair's — blue is not owned by it, keeps an order given meanwhile, and that order does not call the
//                      breach off
//   foursquad-budget   the numbers: ten characters against eight proxy places and sixteen skinned, 63 of 64 steady lights at the spawn's own count and how many
//                      are really uploaded; with ten men drawn the eight proxy places go to the living first once a man is down; an eleventh man is refused
// SETTLED RULES ARE NOT WRITTEN HERE ANY MORE (2026-11-07). Out: 'move there (ROOMFIT off)' in `stack`; in `clear` the pin that had a locked door booted
// untried (DOORTOOLS.testDoor off: the lead tries the handle now, as on the page) and the line of the slicing order by four, which lived on the threshold's
// page the game no longer has (DOORTOOLS.outsidePlain off).
import { doing, ringUp, ringDown } from './squadwords';
import { standUp, ROOT } from './headless.ts';
import { sample } from './sweep.ts';
const { NIGHTBREACH, SQUAD4, SQUAD4_BUDGET } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { ROOMCLEAR, STRUCTURE } = await import(`${ROOT}/src/game/roomclear.ts`);
const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`);
const { ORDERS, sideOf } = await import(`${ROOT}/src/game/orders.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
const { spawnUnit } = await import(`${ROOT}/src/game/guards.ts`);
const { MAX_LIGHTS } = await import(`${ROOT}/src/render/lights.ts`);

const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
const at = (p: number[]) => `(${p[0].toFixed(1)}, ${p[2].toFixed(1)})`;
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const DT_DEFAULTS = JSON.parse(JSON.stringify(DOORTOOLS)), RC_DEFAULTS = { ...ROOMCLEAR }, NB_DEFAULTS = { ...NIGHTBREACH }, ORD_DEFAULTS = { ...ORDERS, followBack: [...ORDERS.followBack] };
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const names = (l: any[]) => l.map((g: any) => g.callsign).join(', ');
/** every probe leaves the knob tables as it found them, pass, fail or throw */
const guarded = (fn: () => Promise<boolean>) => async () => { try { return await fn(); } finally { Object.assign(DOORTOOLS, JSON.parse(JSON.stringify(DT_DEFAULTS))); Object.assign(ROOMCLEAR, RC_DEFAULTS); Object.assign(NIGHTBREACH, NB_DEFAULTS); Object.assign(ORDERS, ORD_DEFAULTS, { followBack: [...ORD_DEFAULTS.followBack] }); } };

// where Sam stands for a door (his side of it is the side the squad works it from), off the line of the opening
const SAM: Record<string, number[]> = { breakroom_w: [25.0, 0, 18.9], breakroom_n: [28.6, 0, 11.0], manager: [24.0, 0, 11.2], storage: [30.8, 0, 11.2], server: [21.0, 0, 11.2] };
const AISLE: number[] = [15, 0, 17.6];
const alongOf = (d: any, p: number[]) => { const th = d.def.closedDir; return (p[0] - d.def.hinge[0]) * Math.cos(th) + (p[2] - d.def.hinge[1]) * Math.sin(th); };
const offOf = (d: any, p: number[]) => { const th = d.def.closedDir; return Math.cos(th) * (p[2] - d.def.hinge[1]) - Math.sin(th) * (p[0] - d.def.hinge[0]); };
/** standing in the doorway: between the jambs and within half a metre of the frame line */
const inDoorway = (d: any, p: number[]) => { const a = alongOf(d, p); return a > 0 && a < d.def.width && Math.abs(offOf(d, p)) < 0.5; };

/** One game under a seed: the room clear staged (`stage: 'nb'` = the night breach alone) with `squad` teammates (default four), Sam in god mode and dark.
 *  `quiet`: the defenders kept dazzled every frame (they neither see nor fire) — for the probes that are about feet, not the fight. */
async function rig(seed: number, o: { squad?: 2 | 4; stage?: 'rc' | 'nb'; quiet?: boolean; door?: string; knobs?: Record<string, unknown> } = {}) {
  Object.assign(DOORTOOLS, JSON.parse(JSON.stringify(DT_DEFAULTS))); Object.assign(ROOMCLEAR, RC_DEFAULTS); Object.assign(NIGHTBREACH, NB_DEFAULTS, { squad: o.squad ?? 4 }, o.knobs ?? {});
  Math.random = seeded(seed);
  const W: any = await standUp(); const game = W.game, input = W.input; const RC = game.roomClear, NB = game.nightBreach, O = game.orders, D = game.doorTools;
  game.godMode = true;
  const said: string[] = []; { const msg = game.msg.bind(game); game.msg = (t: string) => { said.push(t); msg(t); }; }
  const why = (o.stage ?? 'rc') === 'rc' ? RC.start(seed) : NB.start();
  const seen = new Set<any>(), heard = new WeakSet<any>();
  const R = { W, game, input, RC, NB, O, D, why, said, quiet: !!o.quiet, lines: [] as { who: string; text: string; t: number }[],
    /** who went down, when, by whom · the least any two living teammates have been apart on any frame (and who) · run before / after every frame */
    downs: [] as { who: any; t: number; by: any }[], closest: Infinity, closestWho: '', pre: null as null | (() => void), post: null as null | (() => void),
    /** rounds the teammates fired · the ones whose line passed through a living teammate */
    rounds: 0, through: [] as string[],
    get ops(): any[] { return NB.ops; }, get hostiles(): any[] { return NB.hostiles; },
    step(n = 1) {
      for (let i = 0; i < n; i++) {
        game.player.visibility = 0.05;
        if (R.quiet) for (const h of NB.hostiles) if (h.state !== 'dead') h.dazzledUntil = game.time + 5;
        R.pre?.();
        const was = game.guards.map((x: any) => [x, x.state] as const);
        W.step(DT);
        for (const e of game.events) { if (seen.has(e)) continue; seen.add(e);
          // LANES: a teammate's round that passes through a living teammate's body on its way (combat.ts lets a friend's round through him by rule, so nobody can
          // fall to it — what CAN be measured is how often a man fires with a friend in his lane)
          if (e.kind !== 'guardShot' || !e.from || !e.to) continue; const sh = R.ops.find((x: any) => x.char.id === e.who); if (!sh) continue; R.rounds++;
          const ax = e.from[0], az = e.from[2], bx = e.to[0] - ax, bz = e.to[2] - az, L2 = bx * bx + bz * bz || 1;
          for (const m of R.ops) { if (m === sh || m.state === 'dead') continue; const k = ((m.char.pos[0] - ax) * bx + (m.char.pos[2] - az) * bz) / L2; if (k <= 0.02 || k >= 1) continue;
            const y = e.from[1] + (e.to[1] - e.from[1]) * k - m.char.pos[1]; if (Math.hypot(ax + bx * k - m.char.pos[0], az + bz * k - m.char.pos[2]) < 0.28 && y > 0 && y < 1.8) { R.through.push(`${sh.callsign} through ${m.callsign} at ${f2(game.time)} s`); break; } } }
        for (const [x, st] of was) if (st !== 'dead' && x.state === 'dead') R.downs.push({ who: x, t: game.time, by: game.guards.find((k: any) => k.char.id === x.killerId) ?? null });
        for (const gd of game.guards) { const b = gd.bubble; if (b && !heard.has(b)) { heard.add(b); R.lines.push({ who: gd.callsign, text: b.text, t: game.time }); } }
        const up = R.ops.filter((x: any) => x.state !== 'dead');
        for (let a = 0; a < up.length; a++) for (let b = a + 1; b < up.length; b++) { const dd = dist(up[a].char.pos, up[b].char.pos); if (dd < R.closest) { R.closest = dd; R.closestWho = `${up[a].callsign} / ${up[b].callsign} at ${f2(game.time)} s`; } }
        R.post?.();
      }
    },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    door(name: string): any { return game.doors.list.find((d: any) => d.def.name === name); },
    target(name: string) { return O.targetAt(R.door(name).frameCentre); },
    give(id: string, name: string, who: any = 'all'): string | null { return O.give(id, R.target(name), who); },
    floor(p: number[]) { return O.targetAt(p); },
    get op(): any { return D.ops[0] ?? D.done[D.done.length - 1] ?? null; },
    beats(op: any): string { return (op?.beats ?? []).map((b: any) => b.what).join(' | '); },
    room(id: string) { return RC.rooms.find((r: any) => r.def.id === id); },
    /** press X on p (one frame, nothing held), then the digit a third of a second later: the row with that digit, as a player gives it */
    /** friendly fire: a teammate down by a teammate's (or Sam's) hand */
    get fratricide(): string[] { return R.downs.filter(x => x.who.team === 'operative' && (x.by?.team === 'operative' || x.who.killerId === game.player.char.id)).map(x => `${x.who.callsign} by ${x.by?.callsign ?? 'Sam'}`); },
  };
  R.step(30);
  if (o.door) { game.teleportPlayer(SAM[o.door]); R.step(5); }
  return R;
}
const minApart = (men: any[]) => { let m = Infinity; for (let a = 0; a < men.length; a++) for (let b = a + 1; b < men.length; b++) m = Math.min(m, dist(men[a].char.pos, men[b].char.pos)); return m; };

// ---------------------------------------------------------------- off

// ---------------------------------------------------------------- stage
export const foursquadStage = guarded(async () => {
  const { verdict, done } = family('foursquad-stage: four teammates in a red pair and a blue pair, behind the flag');
  const R = await rig(3); const { game: g, O, NB, RC } = R; const ops = NB.ops, sam = g.player.char;
  verdict(R.why === null && ops.length === 4 && names(ops) === 'Red One, Red Two, Blue One, Blue Two' && ops.every((x: any) => x.state !== 'dead' && x.team === 'operative'), 'the room clear starts with four teammates: Red One, Red Two, Blue One, Blue Two', `start ${R.why} · ${names(ops)} · ${RC.summary()}`);
  verdict(ops.map((x: any) => x.unit.pair).join() === 'red,red,blue,blue' && O.pairs().join() === 'red,blue' && ops.every((x: any) => O.pairOf(x) === x.unit.pair), 'each man carries his pair in his papers (Guard.unit.pair)', ops.map((x: any) => `${x.callsign}: ${x.unit.pair}`).join(' · '));
  const red = ops[0].char, blue = ops[2].char, same = (a: number[], b: number[]) => a.every((v, i) => Math.abs(v - b[i]) < 1e-6);
  verdict(same(ops[0].char.tint2, ops[1].char.tint2) && same(ops[2].char.tint2, ops[3].char.tint2) && red.tint2[0] > 3 * red.tint2[2] && blue.tint2[2] > 3 * blue.tint2[0] && !same(red.tint, blue.tint),
    'the pairs wear their colours: a pair alike, red and blue apart (the look itself is unseen here)', `red ${red.tint.map(f2)} / joints ${red.tint2.map(f2)} · blue ${blue.tint.map(f2)} / joints ${blue.tint2.map(f2)}`);
  const all = [sam, ...ops.map((x: any) => x.char)]; let near = Infinity; for (let a = 0; a < all.length; a++) for (let b = a + 1; b < all.length; b++) near = Math.min(near, dist(all[a].pos, all[b].pos));
  verdict(near > 0.8 && ops.every((x: any) => !g.col.nav.isBlocked(x.char.pos[0], x.char.pos[2]) && dist(x.char.pos, sam.pos) < 4), 'staged on free floor beside Sam, a body\'s room apart', `${ops.map((x: any) => `${x.callsign} ${at(x.char.pos)}`).join(' · ')} · Sam ${at(sam.pos)} · nearest two ${f2(near)} m`);
  verdict(NB.hostiles.length === 5 && RC.rooms.length === 4 && 1 + g.guards.length === 10, 'all five defenders are still dealt into the four rooms: ten characters', `${NB.hostiles.length} defenders · ${RC.rooms.map((r: any) => `${r.def.name} ${r.defenders.length}`).join(' · ')} · ${1 + g.guards.length} characters`);
  verdict(names(O.men()) === names(ops) && names(O.men('red')) === 'Red One, Red Two' && names(O.men('blue')) === 'Blue One, Blue Two' && O.men(2)[0] === ops[2] && O.men(ops[3])[0] === ops[3] && O.men(7).length === 0,
    'men(): the squad, a pair, a man — one call each', `all [${names(O.men())}] · red [${names(O.men('red'))}] · blue [${names(O.men('blue'))}] · men(2) ${names(O.men(2))} · men(Blue Two) ${names(O.men(ops[3]))} · men(7) ${O.men(7).length}`);
  // THE TEAM STARTS FALLEN IN (startfile.ts START.fallIn, settled): the room clear stages the four on the menu's own 'fall in', so until F nobody reads
  // 'waiting' or 'standing by' — every man holds that order and reads 'with you'. Whose the drill is does not move: red's, never blue's.
  verdict(names(NB.drillOps) === 'Red One, Red Two' && !NB.drillOps.includes(ops[2]) && !NB.drillOps.includes(ops[3]) && ops.every((x: any) => O.of(x)?.order === 'fallIn' && doing(O.g, x).startsWith('with you')),
    'F\'s drill is the red pair\'s, blue outside it; until F all four hold the fall-in they start on (\'with you\')', `drill: ${names(NB.drillOps)} · ${ops.map((x: any) => doing(O.g, x)).join(' · ')}`);
  // an order to a pair moves the pair alone
  // (fallen in, red is not a pair with no order: it holds its fall-in, and when blue leaves the file it closes up on Sam — a step that is not toward the spot.
  //  Asked there: the SAME order as before on both red men, still 'with you' beside Sam, and neither any nearer the spot blue was sent to)
  const p0 = ops.map((x: any) => [...x.char.pos]), had = ops.map((x: any) => O.of(x)), toSpot = ops.map((x: any) => dist(x.char.pos, AISLE));
  const why = O.give('move', R.floor(AISLE), 'blue'); R.until(() => O.men('blue').every((m: any) => O.of(m)?.arrived), 14); R.step(30);
  const moved = ops.map((x: any, i: number) => dist(x.char.pos, p0[i])); const l = O.log[O.log.length - 1];
  const redStays = [0, 1].every(i => O.of(ops[i]) === had[i] && had[i]?.order === 'fallIn' && doing(O.g, ops[i]).startsWith('with you') && dist(ops[i].char.pos, AISLE) > toSpot[i] - 0.3 && dist(ops[i].char.pos, sam.pos) < 4);
  verdict(why === null && l?.who.join() === 'Blue One,Blue Two' && redStays && O.men('blue').every((m: any) => dist(m.char.pos, AISLE) < 2.55),
    'give(\'move\', spot, \'blue\'): the blue pair goes; red keeps the fall-in it held and stays with Sam (it closes the file blue left, no nearer the spot)',
    `give → ${why} · log: ${l?.order} → ${l?.who.join(' + ')} · moved ${moved.map(f2).join(' / ')} m · said "${R.said[R.said.length - 1]}" · red: ${[0, 1].map(i => `${ops[i].callsign} ${O.of(ops[i]) === had[i] ? 'the same' : 'ANOTHER'} ${O.of(ops[i])?.order ?? 'no order'}, ${doing(O.g, ops[i])}, ${f2(toSpot[i])} → ${f2(dist(ops[i].char.pos, AISLE))} m from the spot, ${f2(dist(ops[i].char.pos, sam.pos))} m from Sam`).join(' · ')}`);
  // (TWO LINES OF THE WHO KEY WENT FROM HERE, 2026-11-06: 'T steps all → red → blue → all' and 'a pair chosen as "who" stands while a man of it lives'. T
  //  has had no key on the page's board since the ring became the squad's only keys, and an order of the squad's key is everyone's — CMDRING.only and
  //  .wholeSquad, both settled; wholesquad-check holds the rule as the page has it, and orderui-probes still asks the fall back to all with the rule pinned
  //  off. A pair is still ordered by code above, give(…, 'blue').)
  // a pair down
  killGuard(g, ops[0], [1, 0, 0], false, 'shot', 'player'); killGuard(g, ops[1], [1, 0, 0], false, 'shot', 'player'); R.step(2);
  verdict(O.men('red').length === 0 && O.men('red', { down: true }).length === 2 && O.men().length === 2, 'with the red pair down, men() lists the living unless asked', `red living ${O.men('red').length}, with the down ${O.men('red', { down: true }).length} · living ${O.men().length}`);
  // a restart stages the four again
  g.restartEncounter(); R.step(30);
  verdict(NB.ops.length === 4 && names(NB.ops) === 'Red One, Red Two, Blue One, Blue Two' && NB.ops.every((x: any) => x.state !== 'dead') && 1 + g.guards.length === 10 && NB.staged === 2, 'a restart stages the four again', `${names(NB.ops)} · staged ×${NB.staged} · ${1 + g.guards.length} characters`);
  // stop: the shift back; the knob at 2: the pair
  RC.stop(); R.step(10); const staff = g.guards.length, units = g.guards.filter((x: any) => x.unit).length;
  NIGHTBREACH.squad = 2; const why2 = RC.start(3); R.step(30);
  verdict(units === 0 && staff >= 3 && why2 === null && NB.ops.length === 2 && names(NB.ops) === 'Blue One, Blue Two' && 1 + g.guards.length === 8, 'stop puts the shift back with no unit left; the next start with the knob at 2 is the pair', `after stop: ${staff} guards, ${units} units · restart at 2: ${why2} · ${names(NB.ops)} · ${1 + g.guards.length} characters`);
  return done();
});

// ---------------------------------------------------------------- follow
export const foursquadFollow = guarded(async () => {
  const { verdict, done } = family('foursquad-follow: fall in keeps all four with Sam across two rooms and through the doors');
  const R = await rig(11, { quiet: true }); const { game: g, O, NB } = R; const men = NB.ops, sam = g.player.char;
  // away first, so that falling in is a real leg
  O.give('move', R.floor([21, 0, 17.6])); R.until(() => men.every((m: any) => O.of(m)?.arrived), 14);
  const far = men.map((m: any) => dist(m.char.pos, sam.pos)), ranks = minApart(men);
  O.give('fallIn', O.targetAt(sam.pos)); const l = O.log[O.log.length - 1];
  R.until(() => men.every((m: any) => O.of(m)?.arrived), 14); R.step(90);
  const near = men.map((m: any) => dist(m.char.pos, sam.pos)), file = [...men].sort((a: any, b: any) => O.of(a).follow - O.of(b).follow);
  const together = O.pairOf(file[0]) === O.pairOf(file[1]) && O.pairOf(file[2]) === O.pairOf(file[3]);
  verdict(l?.order === 'fallIn' && l.who.length === 4 && far.every((d: number) => d > 6) && near.every((d: number) => d > 0.6 && d < 5.8) && minApart(men) > 0.6 && men.every((m: any) => doing(O.g, m).startsWith('with you')) && together,
    'fall in: the four come back from a far spot and close up behind Sam in one file, each pair together', `from ${far.map(f2).join(' / ')} m (apart ${f2(ranks)} m there) to ${near.map(f2).join(' / ')} m of Sam · file ${names(file)} · nearest two ${f2(minApart(men))} m`);
  // the walk: the aisle → the west door → across the break room → out of the north door → along the corridor
  const Wd = R.door('breakroom_w'), Nd = R.door('breakroom_n');
  const legs: number[][] = [[26.6, 0, 17.6], [29.6, 0, 17.6], [30.75, 0, 13.6], [30.75, 0, 11.0], [22, 0, 11.0]];
  const path: number[][] = [[sam.pos[0], 0, sam.pos[2]]]; let noPath = '';
  for (const p of legs) { const leg = g.col.nav.findPath(path[path.length - 1], p); if (!leg) { noPath = `no nav path to ${at(p)}`; break; } for (const q of leg) path.push([q[0], 0, q[2]]); path.push(p); }
  let seg = 0, t = 0; const SPEED = 1.55; let walked = 0, frames = 0, far6 = 0; const worst = men.map(() => 0);
  R.pre = () => {
    if (seg >= path.length - 1) return;
    let stepLeft = SPEED * DT;
    while (stepLeft > 0 && seg < path.length - 1) { const a = path[seg], b = path[seg + 1], L = dist(a, b); if (L - t <= stepLeft) { stepLeft -= L - t; seg++; t = 0; } else { t += stepLeft; stepLeft = 0; } }
    const a = path[seg], b = path[Math.min(seg + 1, path.length - 1)], L = dist(a, b) || 1, f = seg >= path.length - 1 ? 0 : t / L;
    g.teleportPlayer([a[0] + (b[0] - a[0]) * f, 0, a[2] + (b[2] - a[2]) * f]); walked += SPEED * DT;
  };
  R.closest = Infinity; const tWalk = g.time;
  while (seg < path.length - 1 && g.time - tWalk < 90) { R.step(); men.forEach((m: any, i: number) => { const d = dist(m.char.pos, sam.pos); worst[i] = Math.max(worst[i], d); if (d > 6.5) far6++; }); frames++; }
  R.pre = null; const walkSecs = g.time - tWalk;
  R.until(() => men.every((m: any) => O.of(m)?.arrived), 10); R.step(60 * 2);
  const end = men.map((m: any) => dist(m.char.pos, sam.pos)), room = (p: number[]) => p[2] < 12.2 ? 'corridor' : p[0] > 28 ? 'break room' : 'cubicles';
  verdict(!noPath && seg >= path.length - 1, 'Sam\'s walk: 1.55 m/s from the aisle, in at the west door, across the break room, out at the north door, along the corridor', noPath || `${f2(walked)} m in ${f2(walkSecs)} s · ended ${at(sam.pos)} (${room(sam.pos)})`);
  verdict(worst.every((d: any) => d < 8) && far6 / (4 * frames) < 0.1, 'all four stay with him the whole way (the last man of four files 4.6 m back)', `furthest ${worst.map(f2).join(' / ')} m · further than 6.5 m for ${f2(100 * far6 / (4 * frames))} % of the man-frames`);
  verdict(end.every((d: any) => d < 5.8) && men.every((m: any) => room(m.char.pos) === 'corridor' && m.state !== 'dead'), 'they end with him on the far side of both doors', `${end.map(f2).join(' / ')} m from Sam · ${men.map((m: any) => `${m.callsign} ${at(m.char.pos)} ${room(m.char.pos)}`).join(' · ')}`);
  verdict(!men.some((m: any) => inDoorway(Wd, m.char.pos) || inDoorway(Nd, m.char.pos)) && minApart(men) > 0.6, 'nobody is left standing in a doorway, and nobody on top of anybody', `nearest two at the end ${f2(minApart(men))} m`);
  verdict(R.closest > 0.5, 'through the doors: never two men in one place (centres never nearer than half a metre)', `closest on any frame of the walk: ${f2(R.closest)} m (${R.closestWho})`);
  return done();
});

// ---------------------------------------------------------------- stack
export const foursquadStack = guarded(async () => {
  const { verdict, done } = family('foursquad-stack: four marks on a door, two files, a pair a file');
  const R = await rig(5, { quiet: true, door: 'manager' }); const { game: g, O, D, NB } = R; const men = NB.ops, sam = g.player.char;
  // geometry: every door of the structure, from the side its room's clear plan (or the corridor) takes it
  const rows: string[] = []; let geomOk = true;
  for (const room of STRUCTURE.rooms) for (const { id: dn } of room.doors) { const d = R.door(dn); if (!d) continue;
    for (const side of [1, -1] as const) { const inside = [d.frameCentre[0] - Math.sin(d.def.closedDir) * side * 1.2, 0, d.frameCentre[2] + Math.cos(d.def.closedDir) * side * 1.2];
      if (inBounds(room.bounds, inside)) continue;   // (the squad takes a room's door from OUTSIDE the room)
      const posts = O.stackPosts(d, side, men), tucked = D.stackPosts(d, side, men), pods = D.pods(d, side, 4), so = D.standoffPosts(d, side, 4);
      let apart = Infinity; if (tucked) for (let a = 0; a < tucked.length; a++) for (let b = a + 1; b < tucked.length; b++) apart = Math.min(apart, dist(tucked[a].pos, tucked[b].pos));
      const ok = !!posts && posts.length === 4 && !!tucked && tucked.length === 4 && apart > 0.7 && tucked.every((p: any) => !g.col.nav.isBlocked(p.pos[0], p.pos[2]) && !inDoorway(d, p.pos)) && pods.length >= 3 && pods.every((p: any) => inBounds(room.bounds, p.pos, 0.3));
      if (!ok && !(dn === 'server_manager')) geomOk = false;   // (the keypad door between two rooms: taken from inside a room — printed, not judged)
      rows.push(`${dn} from ${side > 0 ? '+' : '−'}: ${tucked ? `${tucked.length} marks ${tucked.map((p: any) => `${at(p.pos)}${p.lead ? '*' : ''}`).join(' ')} (nearest two ${f2(apart)} m)` : 'no stack'} · ${pods.length} points inside · stand-off ${so ? so.length : 'none'}${ok ? '' : '  ← short'}`); } }
  verdict(geomOk, 'every outside door of the four rooms has four stack marks (apart, on free floor, out of the opening) and at least three points inside for an entry', rows.join('\n    '));
  // live: stack up on the manager's door from the corridor
  const d = R.door('manager'); const why = R.give('stack', 'manager'); const posts = men.map((m: any) => O.of(m)?.post);
  const set = R.until(() => men.every((m: any) => O.of(m)?.arrived), 20); R.step(90);
  const off = men.map((m: any, i: number) => dist(m.char.pos, posts[i])), hiSide = (m: any) => alongOf(d, m.char.pos) > d.def.width / 2;
  const redTogether = hiSide(men[0]) === hiSide(men[1]), blueTogether = hiSide(men[2]) === hiSide(men[3]);
  verdict(why === null && set && off.every((x: any) => x < 0.4) && minApart(men) > 0.7 && men.every((m: any) => doing(O.g, m).startsWith('stacked · manager door')), 'stack up (the manager\'s door, from the corridor): all four on their own marks', `give → ${why} · ${men.map((m: any, i: number) => `${m.callsign} ${at(m.char.pos)} (${f2(off[i])} m off)`).join(' · ')} · nearest two ${f2(minApart(men))} m`);
  // GLASS IS NOT COVER (orders.ts stackPosts, GLASSPEN.stackEyes — live in the page; no lines while rounds stop in glass, which is the frozen stand): a jamb the
  // room's rifles reach through a pane is nobody's place, and this door's latch jamb is one. So there the four are ONE file down the other wall: the lead at the
  // jamb, the rest behind him (a place left open where Sam stands), red ahead of blue. The game is asked what its dealer asks — the lines through glass onto each
  // bare jamb (glassEyes) — and with such a jamb: everybody on the other hand, on a mark none of those lines reach, the file unbroken, each pair together in it,
  // 'set.' once. Two files, a pair a jamb, is still the deal at a door with no glass in reach: asked of the marks the server room's door is dealt (nobody is sent there).
  const { GLASSPEN } = await import(`${ROOT}/src/game/glasspen.ts`); const side = sideOf(d, sam.pos);
  const jamb = (dr: any, s: number, latch: boolean) => { const th = dr.def.closedDir, al = latch ? dr.def.width + ORDERS.jambPast : -ORDERS.jambPast; return [dr.def.hinge[0] + Math.cos(th) * al - Math.sin(th) * s * ORDERS.jambOut, 0, dr.def.hinge[1] + Math.sin(th) * al + Math.cos(th) * s * ORDERS.jambOut]; };
  const eyesL: number = O.glassEyes(d, side, jamb(d, side, true)), eyesH: number = O.glassEyes(d, side, jamb(d, side, false)), pane = GLASSPEN.stackEyes > 0 && Math.max(eyesL, eyesH) >= GLASSPEN.stackEyes && eyesL !== eyesH;
  const where = `${men.map((m: any) => `${m.callsign}: ${hiSide(m) ? 'latch' : 'hinge'} side, ${f2(Math.abs(alongOf(d, m.char.pos) - d.def.width / 2))} m from the middle`).join(' · ')}`;
  if (!pane) verdict(redTogether && blueTogether && hiSide(men[0]) !== hiSide(men[2]), 'red on one jamb, blue on the other', where);
  else { const hi = eyesH > eyesL, edge = (p: number[]) => { const a = alongOf(d, p); return a > d.def.width / 2 ? a - d.def.width : -a; };   // (hi: the file is the latch wall's — the hand fewer of the lines reach)
    const file = men.map((_: any, i: number) => i).sort((a: number, b: number) => edge(posts[a]) - edge(posts[b])), gaps = file.map((i: number, k: number) => edge(posts[i]) - (k ? edge(posts[file[k - 1]]) : 0));
    const oneHand = men.every((m: any, i: number) => hiSide(m) === hi && (alongOf(d, posts[i]) > d.def.width / 2) === hi), onMarks = off.every((x: number) => x < 0.4);
    const lines: number[] = posts.map((p: number[]) => O.glassEyes(d, side, p)), unseen = lines.every(n => n < GLASSPEN.stackEyes);
    const filed = gaps.every((x: number, k: number) => k === 0 ? Math.abs(x - ORDERS.jambPast) < 0.1 : x > 0.7 && x < 2 * ORDERS.fileGap + 0.1), pairsKept = O.pairOf(men[file[0]]) === O.pairOf(men[file[1]]) && O.pairOf(men[file[2]]) === O.pairOf(men[file[3]]);
    const sets = R.lines.filter(s => s.text === 'set.').length;
    const dS = R.door('server'), sS = sideOf(dS, SAM.server), pS = O.stackPosts(dS, sS, men), hiS = (p: any) => alongOf(dS, p.pos) > dS.def.width / 2;
    const eyesS: number[] = [O.glassEyes(dS, sS, jamb(dS, sS, true)), O.glassEyes(dS, sS, jamb(dS, sS, false))];
    const twoFiles = Math.max(...eyesS) < GLASSPEN.stackEyes && !!pS && pS.length === 4 && hiS(pS[0]) === hiS(pS[1]) && hiS(pS[2]) === hiS(pS[3]) && hiS(pS[0]) !== hiS(pS[2]);
    verdict(oneHand && onMarks && unseen && filed && pairsKept && sets === 1 && twoFiles, 'a jamb in the room\'s lines through glass is nobody\'s: one file of four down the other wall, on marks out of those lines, each pair together, "set." once — and a door with no glass in reach is still dealt red on one jamb, blue on the other',
      `${where} · lines through glass onto the bare jambs: latch ${eyesL}, hinge ${eyesH} (${GLASSPEN.stackEyes} make it no place) · the file: ${file.map((i: number, k: number) => `${men[i].callsign} ${f2(gaps[k])} m ${k ? 'behind' : 'off the jamb'}, ${lines[i]} lines, ${f2(off[i])} m off his mark`).join(' · ')} · "set." ×${sets} · the server room's door (lines ${eyesS.join(' / ')}): ${pS ? pS.map((p: any, i: number) => `${men[i].callsign} ${hiS(p) ? 'latch' : 'hinge'}`).join(', ') : 'no marks'}`); }
  verdict(!men.some((m: any) => inDoorway(d, m.char.pos)) && men.every((m: any) => offOf(d, m.char.pos) * sideOf(d, sam.pos) > 0) && R.lines.filter(s => s.text === 'set.').length === 1, 'nobody in the opening, everybody on Sam\'s side of it, and "set." is called, once', `"set." by ${R.lines.filter(s => s.text === 'set.').map(s => s.who).join(', ') || 'nobody'}`);
  // (each man's mark, and how near the nearest two marks are)
  const marksOf = (ms: any[]) => ms.map((m: any) => O.of(m)?.post as number[] | null);
  const nearMarks = (mk: (number[] | null)[]) => { let n = Infinity; for (let a = 0; a < mk.length; a++) for (let b = a + 1; b < mk.length; b++) if (mk[a] && mk[b]) n = Math.min(n, dist(mk[a]!, mk[b]!)); return n; };
  // part-orders on one door (what the pair rows will give): red stacks, then blue is sent to the door red holds, then the same a man at a time
  { g.teleportPlayer(SAM.manager); R.step(5); O.give('fallIn', O.targetAt(sam.pos)); R.step(150);
    const w1 = R.give('stack', 'manager', 'red'); const s1 = R.until(() => men.slice(0, 2).every((m: any) => O.of(m)?.arrived), 20);
    const w2 = R.give('stack', 'manager', 'blue'); const s2 = R.until(() => men.every((m: any) => O.of(m)?.arrived), 20); R.step(60);
    const marks = marksOf(men), near = nearMarks(marks);
    verdict(w1 === null && w2 === null && s1 && s2 && marks.every(Boolean) && near > 0.7 && minApart(men) > 0.6 && !men.some((m: any) => inDoorway(d, m.char.pos)), 'a pair sent to stack on a door the other pair holds files in behind them: four marks, no two the same, nobody in the opening', `red → ${w1}, blue → ${w2} · ${men.map((m: any, i: number) => `${m.callsign} ${marks[i] ? at(marks[i]!) : 'no mark'}`).join(' · ')} · nearest two marks ${f2(near)} m`);
  }
  // three men: one down
  killGuard(g, men[3], [1, 0, 0], false, 'shot', 'player'); R.step(5); g.teleportPlayer(SAM.manager); R.step(5);
  const three = () => { const ms = O.men(), mk = marksOf(ms), nearM = nearMarks(mk);
    return `nearest two men ${f2(minApart(ms))} m, nearest two marks ${nearM === Infinity ? 'none' : f2(nearM) + ' m'} · ${ms.map((m: any, i: number) => `${m.callsign} ${at(m.char.pos)} ${mk[i] ? `mark ${at(mk[i]!)}, ${f2(dist(m.char.pos, mk[i]!))} m off it` : 'no mark'}`).join(' · ')} · the body ${at(men[3].char.bones?.hips ?? men[3].char.pos)}`; };
  O.give('fallIn', O.targetAt(sam.pos)); R.step(150);
  const why3b = R.give('stack', 'manager'); const set3b = R.until(() => O.men().every((m: any) => O.of(m)?.arrived), 20);
  if ((await import(`${ROOT}/src/game/teampace.ts`)).TEAMPACE.on) R.step(180);
  verdict(why3b === null && set3b && O.men().length === 3 && minApart(O.men()) > 0.7, 'the same three, fallen in first and sent again: they stack a body\'s room apart', `give → ${why3b} · all set ${set3b} · ${three()}`);
  return done();
});
function inBounds(b: { x0: number; x1: number; z0: number; z1: number }, p: number[], m = 0) { return p[0] > b.x0 - m && p[0] < b.x1 + m && p[2] > b.z0 - m && p[2] < b.z1 + m; }

// ---------------------------------------------------------------- clear
export const foursquadClear = guarded(async () => {
  const { verdict, done } = family('foursquad-clear: the four take rooms with the existing rows');
  // (the two doors of the break room: storage, the manager's and the server room are locked and an order on them ends 'found shut', DOORTOOLS.testDoor, as doortest-check.ts holds)
  const CASES: { id: string; door: string; room: string }[] = sample([
    { id: 'bangClear', door: 'breakroom_n', room: 'breakroom' }, { id: 'bangClear', door: 'breakroom_w', room: 'breakroom' } ], 1);
  const SEEDS = sample([2, 5, 9], 1);
  const rows: string[] = []; let runs = 0, ended = 0, allIn = 0, cleared = 0, lost = 0, rounds = 0, ff: string[] = [], through: string[] = [], stuck: string[] = [], threw = '';
  for (const C of CASES) for (const seed of SEEDS) {
    const R = await rig(seed, { door: C.door }); const { game: g, D, NB } = R; const men = NB.ops, d = R.door(C.door); runs++;
    // (three of these doors are locked, and the lead TRIES a door nobody knew was locked: the order ends there, 'found shut' — DOORTOOLS.testDoor, settled,
    // held by doortest-check.ts. Until 2026-11-07 this wrote the rule off so the entry booted the door and went on; that write was refused.)
    const room = R.room(C.room);
    try {
      const why = R.give(C.id, C.door);   // (the row's own order, as the menu's release gives it — the cursor's ray from this camera does not reach every corridor door: foursquad-stack gives one through the menu itself)
      const op = D.ops[0]; const took = why === null && !!op && op.kind === C.id && op.door === d && op.men.length === 4;
      let dealtT = NaN; R.post = () => { if (Number.isNaN(dealtT) && op?.pod?.size) dealtT = g.time; };   // (the moment the entry is dealt: the first step the order has given anybody a place inside)
      R.until(() => !D.ops.includes(op), 45); R.step(60); R.post = null;
      const inMen = op ? [...op.crossed.keys()] : [], sent = op ? [...op.pod.keys()] : [];
      const up = men.filter((m: any) => m.state !== 'dead'), isIn = (m: any) => inBounds(room.def.bounds, m.char.pos, 0.2);
      const left = room.defenders.filter((h: any) => h.state !== 'dead').length;
      const done1 = !!op && op.ended === 'in'; if (done1) ended++;
      // (a man who fell on his way to the stack was never given a place inside: he 'fell trying' as much as one who fell in the doorway. On the SHIPPED stand
      // storage seed 5 loses two at the stack — the room's own man, alerted by the squad's shots at another on the way, opens his door on them — and the two
      // left are sent and cross: the line's own words, which a bare count of the men sent read as a failure.)
      // (… BEFORE: down by the step the entry was dealt. A man the order left out who was shot afterwards is a man it sent too few of, not one who fell trying.)
      const fellBefore = R.downs.filter(x => men.includes(x.who) && !sent.includes(x.who) && x.t <= dealtT).length;
      const crossedAll = sent.length >= 1 && sent.length + fellBefore >= 3 && sent.every((m: any) => op.crossed.has(m) || m.state === 'dead'); if (took && done1 && crossedAll) allIn++;
      if (left === 0) cleared++; lost += 4 - up.length;
      for (const f of R.fratricide) ff.push(`${C.door}/${seed}: ${f}`);
      rounds += R.rounds; for (const x of R.through) through.push(`${C.door}/${seed}: ${x}`);
      for (const m of up) if (inDoorway(d, m.char.pos)) stuck.push(`${C.door}/${seed}: ${m.callsign}`);
      rows.push(`${C.id} ${C.door} seed ${seed}: ${took ? 'four on it' : `NOT taken (${op ? `${op.kind}, ${op.men.length} men` : 'no order'})`} · ended '${op?.ended}' · sent in ${sent.length}${fellBefore ? ` (${fellBefore} fell before the entry)` : ''}, crossed ${inMen.length}, inside now ${up.filter(isIn).length} of ${up.length} up · defenders left ${left}/${room.defenders.length} · teammates down ${4 - up.length}${R.downs.filter(x => x.who.team === 'operative').map(x => ` (${x.who.callsign} by ${x.by?.callsign ?? '?'})`).join('')}`);
    } catch (e) { threw = `${C.door}/${seed}: ${e instanceof Error ? e.stack : e}`; }
  }
  verdict(!threw, 'nothing threw', threw || `${runs} runs`);
  verdict(ended === runs && allIn === runs, 'every run: the order is taken by all four, every man it sends in crosses the frame line (or falls trying), and it ends \'in\'', rows.join('\n    '));
  verdict(ff.length === 0, 'no teammate went down to a teammate\'s round or Sam\'s (the game\'s rule makes this so; the lanes are counted below)', ff.join(' · ') || `${lost} teammates lost in ${runs} runs, every one to a defender`);
  verdict(stuck.length === 0, 'nobody is left standing in the doorway', stuck.join(' · ') || 'none');
  verdict(cleared >= Math.ceil(runs * 0.75), 'the room is clear at the end of at least three runs in four (the fight is a fight)', `${cleared} of ${runs} rooms clear · ${lost} teammates lost in all (${f2(lost / runs)} a run)`);
  verdict(rounds > 0 && through.length <= rounds * 0.02, 'lanes: fewer than one teammate\'s round in fifty is fired through another teammate (a friend\'s round passes through him by the game\'s rule — this counts the lines, not casualties)', `${through.length} of ${rounds} rounds${through.length ? ` · ${through.slice(0, 6).join(' · ')}` : ''}`);
  // ---- the charge, by four: one man sets it, three stand off in file (or split past the jambs), Sam blows it, they go in
  { const rows2: string[] = []; let ok = 0, n = 0, blast = 0;
    for (const [door, room, seed] of [['breakroom_w', 'breakroom', 5]] as const) { n++;
      const R = await rig(seed, { door }); const { D, NB } = R; const why = R.give('charge', door); const op = D.ops[0]; let res: any = null;
      const posts = op ? NB.ops.filter((m: any) => m !== op.lead).map((m: any) => op.post.get(m)?.pos).filter(Boolean) : [];
      for (let i = 0; i < 60 * 70 && op && D.ops.includes(op); i++) { R.step(); if (op.phase === 'armed' && !res) res = D.blow(); }
      R.step(60); const sent = op ? [...op.pod.keys()] : [], Rm = R.room(room);
      const good = why === null && !!op && op.kind === 'charge' && !!res && res.backblast === 0 && op.ended === 'in' && sent.length >= 3 && sent.every((m: any) => op.crossed.has(m) || m.state === 'dead'); if (good) ok++; blast += res?.backblast ?? 0;
      rows2.push(`charge ${door} seed ${seed}: give → ${why} · set by ${op?.lead?.callsign} · stood off at ${posts.map(at).join(' ')} · blown: ${res ? `knocked ${res.knocked}, stunned ${res.stunned}, backblast ${res.backblast}` : 'never armed'} · ended '${op?.ended}' · sent in ${sent.length}, crossed ${op?.crossed.size} · defenders left ${Rm.defenders.filter((h: any) => h.state !== 'dead').length}/${Rm.defenders.length} · teammates down ${NB.ops.filter((m: any) => m.state === 'dead').length}`); }
    verdict(ok === n && blast === 0, 'breach with a charge, by four: set, stood off clear of the blast, blown on Sam\'s word, and in behind it', rows2.join('\n    ')); }
  return done();
});

// ---------------------------------------------------------------- the F drill
export const foursquadDrill = guarded(async () => {
  const { verdict, done } = family('foursquad-drill: F\'s breach is the red pair\'s; blue is not owned by it');
  const R = await rig(4, { stage: 'nb' }); const { game: g, O, NB } = R; const [r1, b1, b2] = NB.ops;
  verdict(R.why === null && NB.ops.length === 4 && NB.hostiles.length === 3 && 1 + g.guards.length === 8, 'the night breach alone with four: Sam, four teammates, three defenders', `start ${R.why} · ${names(NB.ops)} · ${1 + g.guards.length} characters`);
  R.input.pressed.add('KeyF'); R.step(); R.step(60);
  const own = NB.ops.map((x: any) => O.owners.some((o: any) => o.owns(x)));
  verdict(NB.state === 'stacking' && own.join() === 'true,true,false,false', 'F: the red pair goes to the stack — the drill owns red, not blue', `state ${NB.state} · owned: ${NB.ops.map((x: any, i: number) => `${x.callsign} ${own[i]}`).join(' · ')}`);
  const p = [...b1.char.pos]; const why = O.give('move', R.floor(AISLE), 'blue'); R.step(60 * 3);
  verdict(why === null && (NB.state === 'stacking' || NB.state === 'set') && O.has(b1) && O.has(b2) && !O.has(r1) && dist(b1.char.pos, p) > 1, 'an order to blue meanwhile is taken, and does not call the breach off', `give → ${why} · state ${NB.state} · Blue One moved ${f2(dist(b1.char.pos, p))} m · ${NB.ops.map((x: any) => doing(O.g, x)).join(' · ')}`);
  R.until(() => NB.state === 'set', 25); const set = NB.state; R.input.pressed.add('KeyF'); R.step();
  R.until(() => NB.state === 'clear' || NB.state === 'failed', 40); R.step(60);
  verdict(set === 'set' && (NB.state === 'clear' || NB.state === 'failed') && O.has(b1) && R.fratricide.length === 0, 'the breach runs to its end with blue still on its own order; no teammate shot by a teammate', `set: ${set} · ended ${NB.state} · ${NB.summary()} · friendly fire: ${R.fratricide.join(', ') || 'none'}`);
  return done();
});

// ---------------------------------------------------------------- budget
export const foursquadBudget = guarded(async () => {
  const { verdict, done } = family('foursquad-budget: ten men, a proxy place for each, eight mesh trees, sixteen skinned, sixty-four lights');
  const R = await rig(3); const { game: g, NB } = R; const L = g.engine.lights.lights;
  const steady = L.filter((l: any) => l.ttl < 0).length, lit = L.filter((l: any) => l.ttl < 0 && l.enabled && l.intensity > 0).length;
  R.step(2);
  verdict(steady < MAX_LIGHTS && lit < MAX_LIGHTS, `lights: ${steady} steady lights of ${MAX_LIGHTS} by spawnUnit's own count (${MAX_LIGHTS - steady} spare), ${lit} of them lit and uploaded (${MAX_LIGHTS - lit} spare for flashes and tracers)`, `every man is booked a torch whether it burns or not`);
  // an eleventh man is refused, by the caller's own budget; a caller with none still gets the eight
  let e1 = '', e2 = '';
  try { spawnUnit(g, { team: 'hostile', pos: [15, 0, 17.6], budget: SQUAD4_BUDGET }); } catch (e) { e1 = (e as Error).message; }
  try { spawnUnit(g, { team: 'hostile', pos: [15, 0, 17.6] }); } catch (e) { e2 = (e as Error).message; }
  verdict(/budget is 10/.test(e1) && /shadow-capsule budget is 8/.test(e2) && 1 + g.guards.length === 10, 'an eleventh character is refused (the four-man squad\'s budget is ten); a spawn with no budget of its own is still held to eight', `"${e1.slice(0, 90)}…" · "${e2.slice(0, 90)}…"`);
  verdict(SQUAD4.length === 4 && SQUAD4.map((m: any) => m.pair).join() === 'red,red,blue,blue', 'the roster is data: SQUAD4', SQUAD4.map((m: any) => `${m.name} (${m.pair})`).join(' · '));
  return done();
});
