// A teammate paces himself on the player (game/teampace.ts TEAMPACE.pacing; teamWalkOf, asked by teamCap for every leg a teammate's legs are capped on).
//   bun run tools/qa/pacing-check.ts [--verbose]      (probes.ts pacing, in family squadorders; PACING_BREAK=1 must FAIL)
// From the maintainer's play on pass-four: the squad split for two wall charges, and the second pair "walk[ed] very slowly to it" — the team's flat walk,
// four fifths of his own, 22 s over the 22 m from the start. His rule: "if you're walking with the squad but behind them, they slow down to let you
// catch up (and match your speed when you're ahead of them?)" — "and then we remove the static 0.8 multiplier".
//   1. THE FOUND CASE, staged the same way on either stand: the pair sent to the wall from the mission's start, the player left standing. Rule off: the
//      lead walks the leg at the team's flat walk. Rule on: at the player's own walk and never faster, and he is there two seconds or more sooner.
//   2. THE RULE ITSELF, asked of the same man mid-leg with the player stood by hand (teamWalkOf reads where the two are and how the player moves):
//      behind him and walking after him — the flat share (he lets the player catch up), half of it behind a player creeping at half a walk; behind him
//      and standing, walking away, or walking off to the side — the player's walk; level with him — the player's walk; ahead of him at a walk — the
//      player's walk; ahead of him and sprinting for the mark — the fight's pace, under the run's threshold (an order asks no more than its own brisk
//      pace of his legs: the leg is not walked here); ahead and sprinting off sideways, or coming from further than `paceNear` — the player's walk.
//      Rule off: the flat share in every one.
//   3. WALKED: the same leg with the player kept three seconds of the man's own track behind him and moving after him — the man goes no faster than the
//      flat share while that lasts, and takes longer over the leg than with the player left standing.
//   4. the journal: a recording from before the key replays with it off; one that carries it keeps its own word, on or off
// The team's pace is raised for the staging on either stand (the frozen stand holds it off; the page has it).
// Headless: nothing here says how it looks.
import { ROOT } from './headless.ts';
import { guarded, family } from './thresholdmenu-probes.ts';
const { rig } = (await import('./outerwall-probes.ts')) as any;
const { PACE } = await import(`${ROOT}/src/game/guards.ts`);
const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { WALLCHARGE, WALL_ORDER } = await import(`${ROOT}/src/game/wallcharge.ts`);
const { TEAMPACE, teamWalk, teamWalkOf, teamCap } = await import(`${ROOT}/src/game/teampace.ts`);
const VERBOSE = process.argv.includes('--verbose');
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : '—');
const WALL = [30.0, 1.05, 11.0];   // (pass-four: the wall the second pair was sent to)

/** the mission stood up with the other side far off the floor and the blue pair sent to the wall from where the mission starts them */
async function stand(on: boolean) {
  TEAMPACE.on = true;
  const R = await rig('pass-four', 1), g = R.game, O = R.O as any; TEAMPACE.on = true; TEAMPACE.pacing = on;   // (after: standing a game up puts the stand's own rows back)
  for (const h of R.hostiles) R.put(h, [h.char.pos[0] + 400, 0, h.char.pos[2]]);
  const why = O.give(WALL_ORDER, O.targetAt(WALL), 'blue'); if (why) throw new Error(`pacing: the wall charge was refused — ${why}`); R.step(2);
  const m = O.squad().find((x: any) => x.state !== 'dead' && O.of(x)?.order === WALL_ORDER && O.of(x)?.post); if (!m) throw new Error('pacing: nobody took the wall charge');
  return { R, g, O, m, M: [...O.of(m).post] as number[] };
}
/** the leg walked to within half a metre of his mark (40 s at most). `trail`: the player is kept that many seconds of the man's own track behind him,
 *  moving after him, for as long as the man is more than 6 m from his mark */
async function walk(on: boolean, trail = 0) {
  const { R, g, m, M } = await stand(on), sam = g.player.char, t0 = g.time, d0 = flat(m.char.pos, M), track: number[][] = [];
  let top = 0, sum = 0, n = 0, there = -1, topTrailed = 0, trailed = 0;
  for (let f = 0; f < 60 * 40 && there < 0; f++) {
    track.push([m.char.pos[0], 0, m.char.pos[2]]); const far = flat(m.char.pos, M) > 6, back = track[track.length - 1 - trail * 60];
    if (trail && far && back && flat(back, m.char.pos) > 1.5) { g.teleportPlayer(back); const dx = m.char.pos[0] - back[0], dz = m.char.pos[2] - back[2], L = Math.hypot(dx, dz) || 1; sam.vel[0] = dx / L * g.tune.playerWalk; sam.vel[2] = dz / L * g.tune.playerWalk; trailed++; }
    R.step(); if (m.speed > 0.15) { sum += m.speed; n++; top = Math.max(top, m.speed); } if (trail && far && trailed > 90) topTrailed = Math.max(topTrailed, m.speed);
    if (flat(m.char.pos, M) < 0.5) there = g.time - t0; }
  const out = { leg: d0, top, mean: sum / Math.max(1, n), there, topTrailed, trailed: trailed / 60, flat: teamWalk(g) as number, own: g.tune.playerWalk as number };
  if (VERBOSE) console.log(`    rule ${on}, trail ${trail}: ${m.callsign} ${JSON.stringify(out)}`);
  return out;
}

export const pacingCheck = guarded(async () => {
  const F = family('pacing: a teammate paces himself on the player');
  const keep = { p: TEAMPACE.pacing, tp: TEAMPACE.on, rnd: Math.random, d: DESTRUCTION.on, w: WALLCHARGE.on, o: WALLCHARGE.outer };
  try {
    // ---- 1. the found case
    const off = await walk(false), on = await walk(true);
    F.verdict(off.there > 0 && off.top <= off.flat + 0.06, 'rule off, the found case — the pair sent to the wall from the start, the player left standing: the lead walks the leg at the flat share of the player\'s walk',
      `${f2(off.leg)} m, top ${f2(off.top)} m/s (the flat share ${f2(off.flat)}, the player's walk ${f2(off.own)}), there after ${f2(off.there)} s`);
    F.verdict(on.there > 0 && Math.abs(on.leg - off.leg) < 0.05 && on.top <= on.own + 0.06 && on.top > on.flat + 0.2 && off.there - on.there >= 2,
      'rule on, the same leg: he walks it at the player\'s own walk, never faster, and is there two seconds or more sooner', `${f2(on.leg)} m, top ${f2(on.top)} m/s, mean ${f2(on.mean)}, there after ${f2(on.there)} s (off: ${f2(off.there)} s)`);
    // ---- 2. the rule itself
    { const { R, g, m, M } = await stand(true), sam = g.player.char, own = g.tune.playerWalk as number, slow = teamWalk(g) as number, sprint = g.tune.playerSprint as number;
      R.step(240); const p = [...m.char.pos], dm = flat(p, M), ux = (M[0] - p[0]) / dm, uz = (M[2] - p[2]) / dm;   // (four seconds on: mid-leg, the mark well ahead)
      // (`along` his line to the mark, metres from him; the player's pace `v`, turned `deg` off that line)
      const ask = (along: number, v: number, deg = 0) => { const c = Math.cos(deg * Math.PI / 180), sn = Math.sin(deg * Math.PI / 180); sam.pos[0] = p[0] + ux * along; sam.pos[2] = p[2] + uz * along; sam.vel[0] = (ux * c - uz * sn) * v; sam.vel[2] = (uz * c + ux * sn) * v; return [teamWalkOf(g, m) as number, teamCap(g, m) as number]; };
      const cases: [string, number, number, number, number][] = [['behind and coming', -4, own, 0, slow], ['behind and creeping after him', -4, own * 0.5, 0, slow * 0.5], ['behind and walking off at 75°', -4, own, 75, own], ['behind and standing', -4, 0, 0, own], ['behind and walking away', -4, -own, 0, own], ['level', 0, own, 0, own], ['ahead at a walk', 4, own, 0, own], ['ahead at a sprint', 4, sprint, 0, PACE.max], ['ahead and sprinting off at 90°', 4, sprint, 90, own], ['coming from far behind', -(TEAMPACE.paceNear + 3), own, 0, own]];
      const keepP = [...sam.pos], keepV = [...sam.vel]; const got = cases.map(([, a, v, deg]) => ask(a, v, deg)); TEAMPACE.pacing = false; const flatAll = cases.map(([, a, v, deg]) => ask(a, v, deg)); TEAMPACE.pacing = true;
      sam.pos[0] = keepP[0]; sam.pos[2] = keepP[2]; sam.vel[0] = keepV[0]; sam.vel[2] = keepV[2];
      F.verdict(dm > 8 && cases.every(([, , , , want], i) => Math.abs(got[i][0] - want) < 1e-6 && Math.abs(got[i][1] - want) < 1e-6) && PACE.max < TEAMPACE.runAt,
        'rule on, the man mid-leg and the player stood by hand: behind him and walking after him he gets the flat share (half of it behind a player creeping at half a walk); behind and standing, walking away or off to the side, level, ahead at a walk, ahead and sprinting off sideways, or coming from further than paceNear, the player\'s walk; ahead and sprinting for the mark the fight\'s pace, under a run — and that is the cap his legs are given',
        `${f2(dm)} m to his mark · ` + cases.map(([name], i) => `${name} ${f2(got[i][0])}`).join(' · ') + ` (flat share ${f2(slow)}, player's walk ${f2(own)}, the fight's pace ${f2(PACE.max)}, a run from ${f2(TEAMPACE.runAt)})`);
      F.verdict(flatAll.every(([w, c]) => Math.abs(w - slow) < 1e-6 && Math.abs(c - slow) < 1e-6), 'rule off: the flat share in every one of those', flatAll.map(([w]) => f2(w)).join(' · ')); }
    // ---- 3. walked, the player trailing
    { const tr = await walk(true, 3);
      F.verdict(tr.trailed > 5 && tr.topTrailed > 0.5 && tr.topTrailed <= tr.flat + 0.06 && tr.there > on.there + 1,
        'rule on, the same leg with the player three seconds of the man\'s own track behind him and coming: while that lasts he goes no faster than the flat share, and he is longer over the leg than with the player left standing',
        `trailed for ${f2(tr.trailed)} s, his top then ${f2(tr.topTrailed)} m/s (flat share ${f2(tr.flat)}), there after ${f2(tr.there)} s (the player standing: ${f2(on.there)} s)`); }
    // ---- 4. the journal
    { const J = await import(`${ROOT}/src/game/journal.ts`), R = await rig('room-clear-four', 5), snap = () => JSON.parse(JSON.stringify(J.knobsSnapshot(R.game)));
      TEAMPACE.pacing = true; const old = snap(); if (old.tables.TEAMPACE) delete old.tables.TEAMPACE.pacing; J.applyKnobs(R.game, old); const a = TEAMPACE.pacing;
      TEAMPACE.pacing = true; const yes = snap(); TEAMPACE.pacing = false; J.applyKnobs(R.game, yes); const b = TEAMPACE.pacing;
      TEAMPACE.pacing = false; const no = snap(); TEAMPACE.pacing = true; J.applyKnobs(R.game, no); const c = TEAMPACE.pacing;
      F.verdict(a === false && b === true && c === false, 'the journal: a recording from before the key replays with it off, one made with it on replays on, one made with it off replays off', `old → ${a} · recorded on → ${b} · recorded off → ${c}`); }
  } finally { TEAMPACE.pacing = keep.p; TEAMPACE.on = keep.tp; Math.random = keep.rnd; DESTRUCTION.on = keep.d; WALLCHARGE.on = keep.w; WALLCHARGE.outer = keep.o; }
  return F.done();
});

if (import.meta.main) process.exit((await pacingCheck()) ? 0 : 1);
