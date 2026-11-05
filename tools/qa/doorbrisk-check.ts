// Nobody sprints to a stack (game/teampace.ts TEAMPACE.doorBrisk; teamDash, asked by game/orders.ts of a man walking to the mark a stack order gave him).
//   bun run tools/qa/doorbrisk-check.ts [--verbose]      (probes.ts doorbrisk, in family squadorders; DOORBRISK_BREAK=1 must FAIL)
// Found on pass-four as shipped, from the maintainer's play: with a fight on, men ordered to stack on a door ran to it at the player's sprint with their
// guns low — the team's run to cover, taken on the way to a door. Scripted runs of the mission had it on every seed read, always on a stack order.
//   1. THE FOUND CASE, staged the same way on either stand: the tea room's door, a man stood on open floor a known enemy has a line on, three metres or
//      more from the stack mark the order deals him (out of that line, in plain sight of him), loud in his ears. Rule off: he runs the leg — at the run's
//      pace, gun low, a run to cover written down. Rule on: he is never at a run and no run is written; he goes at the fight's pace, faster than the walk,
//      and is on his mark sooner than the same leg takes him with the fight cold.
//   2. THE FIGHT COLD: rule on, the same leg with nothing heard — he walks it.
//   3. A SPOT ON THE FLOOR: rule on, the same man sent across the same floor by a move order (the order deals him his own spot by the mark) — he runs
//      for cover as he did.
//   4. the journal: a recording from before the key replays with it off; one that carries it keeps its own word, on or off
// The team's pace and the squad's picture of the enemy are raised for the staging on either stand (the frozen stand holds them off; the page has them).
// Headless: nothing here says how it looks.
import { ROOT } from './headless.ts';
import { guarded, family } from './thresholdmenu-probes.ts';
const { rig } = (await import('./outerwall-probes.ts')) as any;
const { PACE } = await import(`${ROOT}/src/game/guards.ts`);
const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { WALLCHARGE } = await import(`${ROOT}/src/game/wallcharge.ts`);
const { PICTURE, pictureTell, exposedAt } = await import(`${ROOT}/src/game/picture.ts`);
const { TEAMPACE, TEAMPACE_STATS, teamWalk, teamWalkOf, teamSprinting } = await import(`${ROOT}/src/game/teampace.ts`);
const VERBOSE = process.argv.includes('--verbose');
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : '—');

interface Leg { found: boolean; leg: number; top: number; low: number; runs: number; there: number; walk: number; order: string }
/** the mission stood up with the other side far off the floor, a man found a place in a known enemy's line with his stack mark out of it, and the leg walked
 *  for seven seconds: `on` the rule, `kind` the order that sends him, `hot` loud in his ears every frame of it */
async function leg(on: boolean, kind: 'stack' | 'move', hot: boolean): Promise<Leg> {
  TEAMPACE.on = true; PICTURE.on = true;
  const R = await rig('pass-four', 1), g = R.game, O = R.O as any; TEAMPACE.on = true; TEAMPACE.doorBrisk = on; PICTURE.on = true;   // (after: standing a game up puts the stand's own rows back)
  for (const h of R.hostiles) R.put(h, [h.char.pos[0] + 400, 0, h.char.pos[2]]);
  const door = g.doors.list.find((d: any) => d.def.name === 'tea'); if (!door) throw new Error('doorbrisk: pass-four has no door named tea');
  const men = O.squad().filter((m: any) => m.state !== 'dead'), H = R.hostiles[0], open = (a: number[], b: number[]) => !g.col.solidBlocked([a[0], 0.45, a[2]], [b[0], 0.45, b[2]]);
  { const why = O.give('stack', O.targetAt(door.frameCentre), 'all'); if (why) throw new Error(`doorbrisk: 'stack' refused — ${why}`); R.step(2); }
  const first = O.of(men[0])?.post ? [...O.of(men[0]).post] : [...door.frameCentre];
  let found: { m: any; p: number[]; to: number[]; E: number[] } | null = null;
  for (const m of men) { if (found) break;
    for (let q = 3.2; q <= 8 && !found; q += 0.6) for (let j = 0; j < 24 && !found; j++) { const p = [first[0] + Math.cos(j * Math.PI / 12) * q, 0, first[2] + Math.sin(j * Math.PI / 12) * q];
      if (g.col.nav.isBlocked(p[0], p[2])) continue; R.put(m, p); if (O.give('stack', O.targetAt(door.frameCentre), 'all')) continue; R.step(1); R.put(m, p);   // (the order given from where he stands: his way is laid from there)
      const A = O.of(m); if (!A?.post || A.via.length || A.thru) continue; const to = [...A.post];
      if (flat(p, to) < 3 || !open(p, to) || g.fight.roomAt(p) !== g.fight.roomAt(to)) continue;
      for (let r = 5; r <= 12 && !found; r += 1) for (let i = 0; i < 24 && !found; i++) { const E = [p[0] + Math.cos(i * Math.PI / 12) * r, 0, p[2] + Math.sin(i * Math.PI / 12) * r];
        if (g.col.nav.isBlocked(E[0], E[2]) || !open(p, E)) continue; R.put(H, E); pictureTell(g, m, H, E);
        if (exposedAt(g, m, p) >= TEAMPACE.dashSeen && exposedAt(g, m, to) <= 0) found = { m, p, to, E }; } } }
  if (!found) return { found: false, leg: 0, top: 0, low: 0, runs: 0, there: -1, walk: teamWalk(g), order: '-' };
  const { m, p, to, E } = found; R.put(H, [E[0] + 400, 0, E[2]]);   // (he is known of, not there: nobody fights)
  if (kind === 'move') { const why = O.give('move', O.targetAt(to), 'all'); if (why) throw new Error(`doorbrisk: 'move' refused — ${why}`); }
  R.put(m, p); pictureTell(g, m, H, E); const n0 = TEAMPACE_STATS.cover, t0 = g.time, d0 = flat(p, O.of(m)?.post ?? to); let top = 0, low = 0, there = -1;
  for (let f = 0; f < 420; f++) { g.fight.hotT = hot ? g.time : -1e9; R.step(); top = Math.max(top, m.speed); if (teamSprinting(g, m)) low++;
    const A = O.of(m); if (there < 0 && A?.post && flat(m.char.pos, A.post) < 0.4) there = g.time - t0; }
  const out = { found: true, leg: d0, top, low, runs: TEAMPACE_STATS.cover - n0, there, walk: teamWalkOf(g, m) as number, order: String(O.of(m)?.order) };
  if (VERBOSE) console.log(`    ${kind}, rule ${on}, ${hot ? 'hot' : 'cold'}: ${m.callsign} at (${f2(p[0])}, ${f2(p[2])}) → (${f2(to[0])}, ${f2(to[2])}) ${JSON.stringify(out)}`);
  return out;
}
const say = (L: Leg) => `${f2(L.leg)} m, top ${f2(L.top)} m/s, ${L.low} frames at a run, ${L.runs} run${L.runs === 1 ? '' : 's'} written, there after ${L.there < 0 ? '—' : f2(L.there) + ' s'}`;

export const doorbriskCheck = guarded(async () => {
  const F = family('doorbrisk: nobody sprints to a stack');
  const keep = { b: TEAMPACE.doorBrisk, tp: TEAMPACE.on, pic: PICTURE.on, rnd: Math.random, d: DESTRUCTION.on, w: WALLCHARGE.on, o: WALLCHARGE.outer };
  try {
    // ---- 1. the found case
    const off = await leg(false, 'stack', true), on = await leg(true, 'stack', true), cold = await leg(true, 'stack', false);
    F.verdict(off.found && off.order === 'stack' && off.top >= TEAMPACE.runAt && off.low > 0 && off.runs > 0,
      'rule off, the found case — sent to stack on the tea room\'s door from floor a known enemy has a line on, loud in his ears: he runs the leg with his gun low, and a run to cover is written', off.found ? say(off) : 'no such floor found');
    F.verdict(on.found && on.order === 'stack' && Math.abs(on.leg - off.leg) < 0.05 && on.low === 0 && on.runs === 0 && on.top <= PACE.max + 0.05 && on.top > on.walk + 0.3,
      'rule on, the same leg: he is never at a run and no run is written — he goes at the fight\'s pace, faster than the team\'s walk', on.found ? `${say(on)} (walk ${f2(on.walk)}, the fight's pace ${f2(PACE.max)}, a run from ${f2(TEAMPACE.runAt)})` : 'no such floor found');
    // ---- 2. the fight cold
    F.verdict(cold.found && Math.abs(cold.leg - on.leg) < 0.05 && cold.top <= cold.walk + 0.06 && cold.low === 0 && on.there > 0 && cold.there > on.there + 0.5,
      'rule on, the same leg with nothing heard: he walks it — and with the fight on he is on his mark half a second or more sooner than that', cold.found ? `cold: ${say(cold)} · hot: there after ${on.there < 0 ? '—' : f2(on.there) + ' s'}` : 'no such floor found');
    // ---- 3. a spot on the floor
    { const mv = await leg(true, 'move', true);
      F.verdict(mv.found && mv.order === 'move' && mv.top >= TEAMPACE.runAt && mv.low > 0 && mv.runs > 0, 'rule on, the same man sent across the same floor by a move order: he runs for cover as he did', mv.found ? say(mv) : 'no such floor found'); }
    // ---- 4. the journal
    { const J = await import(`${ROOT}/src/game/journal.ts`), R = await rig('room-clear-four', 5), snap = () => JSON.parse(JSON.stringify(J.knobsSnapshot(R.game)));
      TEAMPACE.doorBrisk = true; const old = snap(); if (old.tables.TEAMPACE) delete old.tables.TEAMPACE.doorBrisk; J.applyKnobs(R.game, old); const a = TEAMPACE.doorBrisk;
      TEAMPACE.doorBrisk = true; const yes = snap(); TEAMPACE.doorBrisk = false; J.applyKnobs(R.game, yes); const b = TEAMPACE.doorBrisk;
      TEAMPACE.doorBrisk = false; const no = snap(); TEAMPACE.doorBrisk = true; J.applyKnobs(R.game, no); const c = TEAMPACE.doorBrisk;
      F.verdict(a === false && b === true && c === false, 'the journal: a recording from before the key replays with it off, one made with it on replays on, one made with it off replays off', `old → ${a} · recorded on → ${b} · recorded off → ${c}`); }
  } finally { TEAMPACE.doorBrisk = keep.b; TEAMPACE.on = keep.tp; PICTURE.on = keep.pic; Math.random = keep.rnd; DESTRUCTION.on = keep.d; WALLCHARGE.on = keep.w; WALLCHARGE.outer = keep.o; }
  return F.done();
});

if (import.meta.main) process.exit((await doorbriskCheck()) ? 0 : 1);
