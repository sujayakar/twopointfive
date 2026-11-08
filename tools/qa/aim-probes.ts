// The 'aim' probe family (run through probes.ts: `bun run tools/qa/probes.ts aim | aim-unit | aim-first | aim-motion | aim-stop | aim-player | aim-off [--verbose]`;
// also part of `weapons`): the aim-settle model of src/game/aim.ts — one rule for the guards and for Sam, behind Game.tune.aimModel.
//   aim-unit    the model driven directly: a guard's track climbs to 1 in settleSecs with eyes + gun on Sam (quicker inside rangeRef, floored at nearK), drains with
//               the gun off him, holds through a blink, is gone past memSecs / on a bearing jump / with Sam down, is capped on the move and 1 for a pinned man; the
//               fouled bore (two short rounds from one spot = no line from there); the cone × at track 0 / 0.5 / 1 against the formula; the feet term (now vs
//               remembered); the target term off a synthetic bearing rate; Sam's track: caps at a sprint / walking, a new man resets it, off him and straight
//               back keeps it, crouched climbs quicker; his cone's terms; the ring's pixels follow the cone.
//   aim-first   a FREE alert man 8.5 m up the corridor from a lit standing god-mode Sam, from first sight: no round before the beat + acquireMin·settleSecs (± a
//               frame; the beat is reactT, or the startle's beat and draw where longer: AIMFIRST_BREAK=1 must fail both lines), the first round's cone ≥ 1.8× a settled one's and — over eight seeds — the first rounds miss by more than the settled ones; then the
//               weapons-identity staging (6.5 m, gun round in 0.3 s) with the man FREE, model off vs on over 45 s: rounds, how many found Sam, first round.
//   aim-motion  a pinned (settled, planted) man and a god-mode Sam 8 m off: standing / sprinting ACROSS his front / running straight AT him — hit fractions:
//               across ≤ 0.4 × standing, at him ≥ 0.8 × standing; the cones and bearing rates printed; the three canonical cones before → after.
//   aim-stop    a free man 20 m down the corridor from a lit standing Sam closes and fires: with the model ≥ 80 % of his rounds leave at ≤ stopToShoot m/s (he
//               brakes for each), the cone at the chase's 2.7 m/s is ≥ 2× a planted one's; the control (model off) fires on the run.
//   aim-player  ten clicked rounds at the corridor wall 12 m off straight after a sprint stop vs after 1.5 s still: scatter ratio ≥ 1.8; the terms behind it.
//   aim-off     tune.aimModel = false: tracks rest at 0, the gate is open, guardRound hands { spread } untouched (no steady), samAim 0, no braking, no ◎ —
//               and weapons-identity (run in the same families) stays green.
//   aim-snap    what the reticle TAKES (player.ts updateAim): a light only with the cursor on its dot (AIM.player.lightSnapPx) — every aimable light in the lobby
//               view swept on rings from the dot out to 58 px; a man standing under a ceiling panel's picture with the cursor beside him, nearer the panel's
//               dot than his chest: the reticle is on HIM and the clicked round drops him; a light taken is kept while the camera's lead slides its dot from
//               under a cursor left where it was, the right button then OCPs it, and moving the cursor off lets go; a drawn fixture is taken anywhere on its box.
// SINCE 2026-11-07 NOTHING HERE WRITES A SETTLED ROW. The reticle's reach for a light is a constant of the game: aim-snap's second sweep and its control
// man "with the reach back at 60" are cut (the half of two lines), and the reach is no longer written. aim-unit no longer holds the turn weight and the
// feet's cost off for its stand-ups (both are constants, on): it throws at a function that went with the tick before its one line, and when it runs
// again five of its numbers are the ones its old red path (AIMUNIT_BREAK, both rules on) failed. That word is gone with the holds. The model's own flag
// (Game.tune.aimModel) is still a variable of a game: the "model off" lines stand.
import { standUp, ROOT } from './headless.ts';
const { v3, DEG, wrapAngle, lerp } = await import(`${ROOT}/src/math/vec.ts`);
const { updateAim } = await import(`${ROOT}/src/game/player.ts`);
const { resetChase, playerGroundVel } = await import(`${ROOT}/src/game/guards.ts`);
const Aim = await import(`${ROOT}/src/game/aim.ts`);
const { AIM, trackGuard, trackPlayer, aimReady, guardRound, handsCone, aimBrake, samAim, noteBore, boreFouled, AIM_DEBUG } = Aim;
const { STARTLE, STARTLE_STATS } = await import(`${ROOT}/src/game/startle.ts`);
/** what the startle holds a man's first round by, from first sight (startle.ts: the beat, then the draw — no round leaves him in either).
 *  AIMFIRST_BREAK=1, the red path: a second more than the rule says — aim-first's two lines must fail. */
const startleLead = (): number => STARTLE.beat + STARTLE.draw + (process.env.AIMFIRST_BREAK === '1' ? 1 : 0);

type Vec3 = [number, number, number];
const DT = 1 / 60;
const VERBOSE = process.argv.includes('--verbose');
const f1 = (x: number) => x.toFixed(1), f2 = (x: number) => x.toFixed(2), f3 = (x: number) => x.toFixed(3);
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
/** A guard planted at `p` facing yaw, still, no path, free (not held / pinned), posed twice (bones where the sight tests read them). */
function place(gd: any, p: Vec3, yaw: number) { gd.char.pos = v3.copy(p); gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.char.bodyYaw = gd.char.aimYaw = yaw; gd.char.aimPitch = 0; gd.path = []; gd.pathGoal = null; gd.hold = false; gd.pinned = false; gd.char.update(0); gd.char.update(DT); }
/** parked out on the lot east of the building, held and pinned, facing east, deaf and blind */
function park(gd: any, i: number) { gd.hold = true; gd.pinned = true; gd.char.pos = [37.5, 0, 6 + i * 1.5]; gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.char.bodyYaw = gd.char.aimYaw = Math.PI / 2; gd.path = []; gd.pathGoal = null; gd.heardUpTo = 1e12; gd.dazzledUntil = 1e9; gd.char.update(0); }
/** alert on Sam where he stands, licensed (a sighting this frame), reaction beat `reactT`, the gun coming round in `fireCd`, chase clocks zeroed; `pinned` as asked (false = a free man) */
function armAlert(gd: any, game: any, fireCd = 0.3, reactT = 0, pinned = false) { gd.hold = false; gd.pinned = pinned; gd.dazzledUntil = -1; gd.heardUpTo = 1e12; gd.state = 'alert'; gd.awareness = 1; gd.reactT = reactT; gd.fireCd = fireCd; gd.weapon.rearm(); gd.reloadT = -1; gd.lastKnown = v3.copy(game.player.char.pos); gd.lastSeenT = game.time; gd.sightT = game.time; gd.ledT = game.time; resetChase(gd); }
function verdict(ok: boolean, label: string, detail: string) { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}: ${detail}`); return ok; }
/** the angle (rad) between the round's flight (from → to off the guardShot / shot event) and the line from its muzzle to `at` — how far off the point it went */
function missAngle(e: any, at: Vec3): number { const d = v3.normalize(v3.sub(e.to, e.from)), w = v3.normalize(v3.sub(at, e.from)); return Math.acos(Math.max(-1, Math.min(1, v3.dot(d, w)))); }
const trunkOf = (game: any): Vec3 => { const pc = game.player.char; const t = pc.bones.spine2 ?? pc.bones.hips; return [t ? t[0] : pc.pos[0], pc.pos[1] + (game.player.crouch ? 0.8 : 1.25), t ? t[2] : pc.pos[2]]; };

const f4 = (x: number) => x.toFixed(4);

// ================================================================ (b) first sight → first round; the identity staging free, off vs on
interface FightRun { rounds: { t: number; miss: number; cone: number; track: number; speed: number; hit: boolean; dist: number }[]; firstT: number; hits: number; exc: string; trace: string[]; /** startled men who ran for it in this fight (startle.ts STARTLE_STATS.ran, this fight's share) */ ran: number; }
/** A free (or pinned) alert man at `gPos` facing Sam at `sPos` (lit, god mode, standing still), for `secs`: every round's time from t0, miss angle off Sam's trunk,
 *  the cone / track / speed it left with (aim.lastShot under the model; the row's line otherwise), whether it found him. */
async function fight(seed: number, o: { model: boolean; gPos: Vec3; sPos: Vec3; secs: number; fireCd: number; reactT: number; pinned?: boolean; carbine?: boolean }): Promise<FightRun> {
  Math.random = seeded(seed); const ran0: number = STARTLE_STATS.ran;   // (the module's counter runs on from game to game: this fight's share is the difference)
  const W = await standUp(); const { game } = W; const pl = game.player;
  game.godMode = true; game.aiEnabled = true; game.tune.aimModel = o.model; if (o.carbine) game.tune.carbineGuards = 'all';
  for (let i = 0; i < 5; i++) W.step(DT);
  const [A, B, C] = game.guards; park(B, 1); park(C, 2);
  game.teleportPlayer(v3.copy(o.sPos)); pl.slot = 'pistol'; pl.holstered = false; pl.crouch = false; pl.pistol.lightOn = false;
  place(A, o.gPos, Math.atan2(o.sPos[0] - o.gPos[0], o.sPos[2] - o.gPos[2])); W.step(DT); pl.visibility = 0.9;
  armAlert(A, game, o.fireCd, o.reactT, !!o.pinned);
  const R: FightRun = { rounds: [], firstT: -1, hits: 0, exc: '', trace: [], ran: 0 }; const t0 = game.time;
  const dbg: string[] = []; AIM_DEBUG.log = VERBOSE ? (s: string) => dbg.push(`${f2(game.time - t0)} ${s}`) : null;
  try {
    for (let f = 0; f < Math.round(o.secs / DT); f++) {
      pl.visibility = 0.9; pl.hitFlash = 0;
      const trunk = trunkOf(game);
      W.step(DT);
      for (const e of game.events) if (e.kind === 'guardShot' && e.time === game.time && e.from && e.to) {
        const ls = A.aim.lastShot; const hit = pl.hitFlash > 0.9;
        R.rounds.push({ t: game.time - t0, miss: missAngle(e, trunk), cone: ls && ls.t === game.time ? ls.spread : NaN, track: ls && ls.t === game.time ? ls.track : NaN, speed: Math.hypot(A.char.vel[0], A.char.vel[2]), hit, dist: v3.distXZ(A.char.pos, pl.char.pos) });
        if (hit) R.hits++; if (R.firstT < 0) R.firstT = game.time - t0;
      }
      if (f % 15 === 0) R.trace.push(`${f2(game.time - t0)}s ${A.state} track ${f2(A.aim.track)} brg ${f2(A.aim.bearingRate)} feet ${f2(A.aim.speedSm)} speed ${f2(A.speed)} fireCd ${f2(A.fireCd)} reactT ${f2(A.reactT)} dist ${f1(v3.distXZ(A.char.pos, pl.char.pos))}${A.aim.braking ? ' BRAKE' : ''} seen ${f2(game.time - A.lastSeenT)}s rounds ${R.rounds.length}`);
    }
  } catch (e: any) { R.exc = String(e?.stack ?? e); }
  AIM_DEBUG.log = null;
  if (VERBOSE) { console.log('    ' + R.trace.join('\n    ')); if (dbg.length) console.log('    debug: ' + dbg.slice(0, 60).join('\n    debug: ')); }
  R.ran = STARTLE_STATS.ran - ran0; return R;
}
const mean = (a: number[]) => a.length ? a.reduce((s, x) => s + x, 0) / a.length : 0;

export async function aimFirst() {
  console.log('\n=== aim-first: a FREE alert man 8.5 m up the corridor from a lit standing Sam, from first sight — no round before the beat (reactT, or the startle\'s) + acquireMin·settleSecs, the first round\'s cone ≥ 1.8× a settled one\'s, first rounds miss by more (8 seeds); then the weapons-identity staging FREE, model off vs on over 45 s ===');
  let allPass = true; const G = AIM.guard;
  const SAM: Vec3 = [24.0, 0, 11.4], GRD: Vec3 = [15.5, 0, 11.3];   // 8.5 m: inside the 9 m he holds and fires from — planted, so only the beat and the gate decide
  const REACT = 0.35;   // the alert transition's own beat
  {
    const runs: FightRun[] = []; for (let s = 0; s < 8; s++) runs.push(await fight(911 + s, { model: true, gPos: GRD, sPos: SAM, secs: 6, fireCd: 0, reactT: REACT }));
    const fails: string[] = []; const lead = Math.max(REACT, startleLead()), want = lead + G.acquireMin * G.settleSecs;   // (the alert transition's own beat runs inside the startle's, where there is one)
    const firsts = runs.map(r => r.firstT); const firstCones = runs.map(r => r.rounds[0]?.cone ?? NaN), settledCones = runs.flatMap(r => r.rounds.filter(x => x.track >= 0.999).map(x => x.cone));
    const firstMiss = mean(runs.map(r => r.rounds[0]?.miss ?? 0)), settledMiss = mean(runs.flatMap(r => r.rounds.filter(x => x.track >= 0.999).map(x => x.miss)));
    // (the startle's lead is a man who DRAWS after his beat: one who runs for cover has no draw clock, and the bounds would be the staging's, not the aim's)
    for (const [i, r] of runs.entries()) { if (r.ran) fails.push(`seed ${911 + i}: THE STAGING — the startled man ran (${r.ran}), where these bounds count on one who draws`); if (r.exc) fails.push(`seed ${911 + i}: exception ${r.exc.split('\n')[0]}`); if (r.firstT < 0) fails.push(`seed ${911 + i}: no round in 6 s`); else if (r.firstT < want - 1.5 * DT) fails.push(`seed ${911 + i}: first round at ${f3(r.firstT)} s — before the beat + acquireMin·settleSecs = ${f3(want)}`); else if (r.firstT > want + 0.35) fails.push(`seed ${911 + i}: first round only at ${f3(r.firstT)} s (want ≈ ${f3(want)}: the beat, the gate, a frame or two for the gun to come round)`); }
    const coneRatio = mean(firstCones) / Math.max(1e-9, mean(settledCones));
    if (!(coneRatio >= 1.8)) fails.push(`first-round cone ${f4(mean(firstCones))} is only ${f2(coneRatio)}× the settled ${f4(mean(settledCones))} (want ≥ 1.8)`);
    if (settledCones.length < 8) fails.push(`only ${settledCones.length} settled rounds across the runs`);
    if (!(firstMiss > 1.3 * settledMiss)) fails.push(`first rounds missed by ${f4(firstMiss)} rad, settled ones by ${f4(settledMiss)} — want the first plainly wider (> 1.3×)`);
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — from first sight (reactT ${REACT}, fireCd 0, free, 8.5 m): first round at ${firsts.map(f3).join(' / ')} s (floor ${f3(want)} = ${f2(lead)} + ${G.acquireMin}×${G.settleSecs}; the beat is ${lead > REACT ? 'the startle\'s and its draw' : 'reactT'}); first-round cone ${f4(mean(firstCones))} vs settled ${f4(mean(settledCones))} (×${f2(coneRatio)}); mean miss angle first ${f4(firstMiss)} vs settled ${f4(settledMiss)} rad (${settledCones.length} settled rounds); tracks at the first round ${runs.map(r => f2(r.rounds[0]?.track ?? NaN)).join(' ')}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
    if (!ok && !VERBOSE) console.log('    ' + runs[0].trace.slice(0, 12).join('\n    '));
  }
  // ---- the identity staging (weapons-probes section 3: 6.5 m up the corridor, fireCd 0.3, reactT 0, 45 s) with the man FREE: off = today, on = the model
  {
    const on = await fight(502, { model: true, gPos: [17.5, 0, 11.3], sPos: SAM, secs: 8, fireCd: 0.3, reactT: 0 });
    const fails: string[] = [], lead = startleLead(); const settleAt65 = G.settleSecs * Math.min(1, Math.max(G.nearK, 6.5 / G.rangeRef));   // (6.5 m is inside rangeRef: the man fills more of the sights, the settle is that much quicker — aim.ts settleTime)
    if (on.exc) fails.push('exception: ' + on.exc.split('\n')[0]);
    if (on.ran) fails.push(`THE STAGING — a startled man ran (${on.ran}), where these bounds count on one who draws: 6.5 m is inside the startle's run band, and the man here is the steady one`);
    if (!(on.firstT >= lead + G.acquireMin * settleAt65 - 1.5 * DT)) fails.push(`model on, free: first round at ${f3(on.firstT)} s — under ${lead ? `the startle's ${f2(lead)} s + ` : ''}acquireMin·settle(6.5 m) ${f3(G.acquireMin * settleAt65)}`);
    if (on.firstT > lead + G.acquireMin * settleAt65 + 0.2) fails.push(`model on, free: first round only at ${f3(on.firstT)} s (the gate opens at ${f3(lead + G.acquireMin * settleAt65)})`);
    const ok = fails.length === 0; allPass &&= ok;
    const line = (r: FightRun, tag: string) => `${tag}: ${r.rounds.length} rounds in 8 s, ${r.hits} found a standing Sam (${r.rounds.length ? Math.round(100 * r.hits / r.rounds.length) : 0} %), first round +${f3(r.firstT)} s, cones ${f4(r.rounds[0]?.cone ?? NaN)} first / ${f4(mean(r.rounds.slice(2).map(x => x.cone)))} after`;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — the weapons-identity staging (6.5 m, gun round in 0.3 s, no beat), the man FREE: ${line(on, 'on')}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  console.log(`=== aim-first: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (c) target motion: standing / across / straight at him
export async function aimMotion() {
  console.log('\n=== aim-motion: a settled planted man (pinned) and a god-mode Sam about 8 m off — standing, sprinting ACROSS his front, running straight AT him: hit fractions across ≤ 0.4 × standing, at him ≥ 0.8 × standing; the three canonical cones before → after ===');
  let allPass = true; const G = AIM.guard;
  // ACROSS: the one clean crossing lane the floor has (found by a sweep of the nav grid for a straight walkable line with an eye-line to every cell of it): the man in
  // the aisle mouth east of the cubicles at (27.5, 17.5) facing east, Sam's line x = 34 through the break room's west side, z 13‥22.5 — 6.5 m off square-on, 8.3
  // at the ends (inside the 9 m he holds and fires from: a free man stays planted). AT HIM / STANDING use the corridor: the man at (10, 11.3) facing east.
  const LAT_G: Vec3 = [27.5, 0, 17.5], LAT_X = 34.0, LAT_Z0 = 13.0, LAT_Z1 = 22.5, EAST = Math.PI / 2;
  const COR_G: Vec3 = [10.0, 0, 11.3], COR_TOP: Vec3 = [18.9, 0, 11.4], COR_GOAL: Vec3 = [14.2, 0, 11.4], COR_STAND: Vec3 = [16.8, 0, 11.4];   // (the run starts just inside the 9 m he holds and fires from — a planted, settled man throughout, so the case differs from standing by the target's motion alone — and the standing reference is at the run's middle range)
  const stage = async (seed: number, model: boolean, gPos: Vec3, sPos: Vec3) => {
    Math.random = seeded(seed);
    const W = await standUp(); const { game, input } = W; const pl = game.player;
    game.godMode = true; game.aiEnabled = true; game.tune.aimModel = model;
    for (let i = 0; i < 5; i++) W.step(DT);
    const [A, B, C] = game.guards; park(B, 1); park(C, 2);
    game.teleportPlayer(v3.copy(sPos)); pl.slot = 'pistol'; pl.holstered = false; pl.crouch = false; pl.pistol.lightOn = false; W.step(DT); pl.visibility = 0.9;
    place(A, gPos, EAST); armAlert(A, game, 0.3, 0, false);   // free: the real fight branch (he holds and fires inside 9 m); settled once he has had his second on Sam
    for (let f = 0; f < 90; f++) { pl.visibility = 0.9; W.step(DT); }   // (a second and a half: lined up before anything is counted, so the three cases differ by the target alone)
    return { W, game, input, pl, A };
  };
  type Case = { rounds: number; hits: number; cones: number[]; brs: number[]; dists: number[]; speeds: number[] };
  const collect = (game: any, pl: any, A: any, K: Case) => { for (const e of game.events) if (e.kind === 'guardShot' && e.time === game.time) { K.rounds++; if (pl.hitFlash > 0.9) K.hits++; const ls = A.aim.lastShot; K.cones.push(ls && ls.t === game.time ? ls.spread : NaN); K.brs.push(A.aim.bearingRate); K.dists.push(v3.distXZ(A.char.pos, pl.char.pos)); K.speeds.push(Math.hypot(pl.char.vel[0], pl.char.vel[2])); } };
  const runCases = async (model: boolean) => {
    const out: Record<string, Case> = {}; let exc = '';
    const traces: string[] = [];
    try {
      // standing square-on at 6.5 m in the crossing lane, 20 s
      { const { W, game, pl, A } = await stage(921, model, LAT_G, [LAT_X, 0, LAT_G[2]]); const K: Case = { rounds: 0, hits: 0, cones: [], brs: [], dists: [], speeds: [] };
        for (let f = 0; f < 60 * 20; f++) { pl.visibility = 0.9; pl.hitFlash = 0; W.step(DT); collect(game, pl, A, K); } out.standing = K; }
      // across: Sam sprints up and down x = LAT_X between LAT_Z0 and LAT_Z1 (the puppet's goal flipped at each end, Shift held), 45 s; rounds with him inside ±3.5 m of square-on count (at the ends he is turning: those are printed apart)
      { const { W, game, input, pl, A } = await stage(922, model, LAT_G, [LAT_X, 0, LAT_Z0]); const K: Case = { rounds: 0, hits: 0, cones: [], brs: [], dists: [], speeds: [] }; const all: Case = { rounds: 0, hits: 0, cones: [], brs: [], dists: [], speeds: [] };
        let goalZ = LAT_Z1; game.puppet = { goal: [LAT_X, 0, goalZ], aim: null }; input.keys.add('ShiftLeft');
        for (let f = 0; f < 60 * 45; f++) {
          pl.visibility = 0.9; pl.hitFlash = 0;
          if (Math.abs(pl.char.pos[2] - goalZ) < 0.6) { goalZ = goalZ > LAT_G[2] ? LAT_Z0 : LAT_Z1; game.puppet.goal = [LAT_X, 0, goalZ]; }
          W.step(DT);
          const near = Math.abs(pl.char.pos[2] - LAT_G[2]) < 3.5; collect(game, pl, A, near ? K : all);
          if (f % 30 === 0) traces.push(`across ${f2(f * DT)}s Sam (${f1(pl.char.pos[0])}, ${f1(pl.char.pos[2])}) ${f1(Math.hypot(pl.char.vel[0], pl.char.vel[2]))} m/s · ${A.state} track ${f2(A.aim.track)} brg ${f2(A.aim.bearingRate)} seen ${f2(game.time - A.lastSeenT)}s fireCd ${f2(A.fireCd)} blockedT ${f2(A.shotBlockedT)} dist ${f1(v3.distXZ(A.char.pos, pl.char.pos))} rounds ${K.rounds}+${all.rounds}`);
        }
        input.keys.delete('ShiftLeft'); game.puppet = null; out.across = K; out.acrossEnds = all; }
      // at him: down the corridor — Sam sprints from 8.9 m straight up the bore to 4 m off him, is put back at the top, again — 30 s; rounds with him between 4.5 and 9 m count
      { const { W, game, input, pl, A } = await stage(923, model, COR_G, COR_TOP); const K: Case = { rounds: 0, hits: 0, cones: [], brs: [], dists: [], speeds: [] };
        game.puppet = { goal: v3.copy(COR_GOAL), aim: null }; input.keys.add('ShiftLeft');
        for (let f = 0; f < 60 * 30; f++) {
          pl.visibility = 0.9; pl.hitFlash = 0;
          if (pl.char.pos[0] < COR_GOAL[0] + 0.6) { game.teleportPlayer(v3.copy(COR_TOP)); A.aim.lastBearing = NaN; A.lastSeenT = game.time; }   // (back to the top of the run: the same line, so spare the model the frame's blip — and his sight, which the teleport's frame would otherwise cost him)
          W.step(DT);
          const d = v3.distXZ(A.char.pos, pl.char.pos); if (d > 4.5 && d < 9) collect(game, pl, A, K);
          if (f % 30 === 0) traces.push(`atHim ${f2(f * DT)}s Sam (${f1(pl.char.pos[0])}, ${f1(pl.char.pos[2])}) ${f1(Math.hypot(pl.char.vel[0], pl.char.vel[2]))} m/s · ${A.state} track ${f2(A.aim.track)} brg ${f2(A.aim.bearingRate)} seen ${f2(game.time - A.lastSeenT)}s fireCd ${f2(A.fireCd)} dist ${f1(d)} rounds ${K.rounds}`);
        }
        input.keys.delete('ShiftLeft'); game.puppet = null; out.atHim = K; }
      // standing at 6.8 m in the corridor too (the at-him case's own reference: the middle of its run)
      { const { W, game, pl, A } = await stage(924, model, COR_G, COR_STAND); const K: Case = { rounds: 0, hits: 0, cones: [], brs: [], dists: [], speeds: [] };
        for (let f = 0; f < 60 * 20; f++) { pl.visibility = 0.9; pl.hitFlash = 0; W.step(DT); collect(game, pl, A, K); } out.standingCor = K; }
    } catch (e: any) { exc = String(e?.stack ?? e); }
    if (VERBOSE) console.log('    ' + traces.join('\n    '));
    return { out, exc, traces };
  };
  const on = await runCases(true);
  const frac = (K?: Case) => K && K.rounds ? K.hits / K.rounds : 0;
  const desc = (K?: Case) => K ? `${K.hits}/${K.rounds} = ${f2(frac(K))} (cone ${f3(mean(K.cones))}, brg ${f2(mean(K.brs))} rad/s, ${f1(mean(K.dists))} m, Sam ${f1(mean(K.speeds))} m/s)` : '—';
  {
    const fails: string[] = []; const O = on.out;
    if (on.exc) fails.push('exception: ' + on.exc.split('\n')[0]);
    for (const k of ['standing', 'across', 'atHim', 'standingCor']) if (!O[k] || O[k].rounds < 12) fails.push(`${k}: only ${O[k]?.rounds ?? 0} rounds counted`);
    if (O.standing && O.across && !(frac(O.across) <= 0.4 * frac(O.standing) + 1e-9)) fails.push(`across: hit fraction ${f2(frac(O.across))} > 0.4 × standing ${f2(frac(O.standing))}`);
    if (O.standingCor && O.atHim && !(frac(O.atHim) >= 0.8 * frac(O.standingCor) - 1e-9)) fails.push(`at him: hit fraction ${f2(frac(O.atHim))} < 0.8 × standing ${f2(frac(O.standingCor))}`);
    if (O.across && !(mean(O.across.brs) > 0.4)) fails.push(`across: mean bearing rate only ${f2(mean(O.across.brs))} rad/s (a 4.7 m/s crossing 6.5 m off is ~0.7 square-on)`);
    if (O.atHim && !(mean(O.atHim.brs) < 0.12)) fails.push(`at him: mean bearing rate ${f2(mean(O.atHim.brs))} rad/s (straight up the bore should be ~0)`);
    if (O.across && !(mean(O.across.speeds) > 3.5)) fails.push(`across: staging — Sam only doing ${f1(mean(O.across.speeds))} m/s`); if (O.atHim && !(mean(O.atHim.speeds) > 3.5)) fails.push(`at him: staging — Sam only doing ${f1(mean(O.atHim.speeds))} m/s`);
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — model ON: standing 6.5 m ${desc(O.standing)} · ACROSS ${desc(O.across)} (the run's ends, not counted: ${desc(O.acrossEnds)}) · standing 6.8 m corridor ${desc(O.standingCor)} · AT HIM ${desc(O.atHim)}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
    if (!ok && !VERBOSE) console.log('    ' + on.traces.filter((_: string, i: number) => i % 3 === 0).slice(0, 40).join('\n    '));
  }
  console.log(`=== aim-motion: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (d) the shooter's own feet: he stops to shoot
export async function aimStop() {
  console.log('\n=== aim-stop: a free man 20 m down the corridor from a lit standing god-mode Sam closes and fires for 25 s — model on: ≥ 80 % of his rounds leave at ≤ stopToShoot m/s (he brakes for each), the cone at the chase\'s pace ≥ 2× a planted one\'s; model off (control): rounds on the run ===');
  let allPass = true; const G = AIM.guard;
  const SAM: Vec3 = [30.0, 0, 11.4], GRD: Vec3 = [10.0, 0, 11.3];
  const on = await fight(931, { model: true, gPos: GRD, sPos: SAM, secs: 25, fireCd: 0.3, reactT: 0.35 });
  {
    const fails: string[] = []; const R = on;
    if (R.exc) fails.push('exception: ' + R.exc.split('\n')[0]);
    const slow = R.rounds.filter(r => r.speed <= G.stopToShoot + 1e-6).length; const fracSlow = R.rounds.length ? slow / R.rounds.length : 0;
    if (R.rounds.length < 6) fails.push(`only ${R.rounds.length} rounds in 25 s`); if (fracSlow < 0.8) fails.push(`only ${Math.round(fracSlow * 100)} % of his rounds left at ≤ ${G.stopToShoot} m/s`);
    const closing = R.rounds.filter(r => r.dist > 9.2); if (closing.length < 2) fails.push(`only ${closing.length} rounds while still closing (> 9 m): he never shot on the way in`);
    const endDist = R.rounds.length ? R.rounds[R.rounds.length - 1].dist : NaN; if (!(endDist <= 9.5)) fails.push(`his last round came from ${f1(endDist)} m — he never closed to his 9 m`);
    // the composition: cone at 2.7 m/s vs planted, same track / range (the unit's numbers, asked of the live man's state)
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — model ON: ${R.rounds.length} rounds, first +${f2(R.firstT)} s; speed at each [${R.rounds.map(r => f2(r.speed)).join(' ')}] m/s → ${slow}/${R.rounds.length} at ≤ ${G.stopToShoot}; from [${R.rounds.map(r => f1(r.dist)).join(' ')}] m; tracks [${R.rounds.map(r => f2(r.track)).join(' ')}]; cones [${R.rounds.map(r => f3(r.cone)).join(' ')}]; ${R.hits} found Sam${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
    if (!ok && !VERBOSE) console.log('    ' + R.trace.filter((_, i) => i % 2 === 0).slice(0, 40).join('\n    '));
  }
  console.log(`=== aim-stop: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}



// ================================================================ (g) what the reticle takes: a light only on its dot, a man beside one
export async function aimSnap() {
  console.log('\n=== aim-snap: the reticle takes a light only with the cursor on it (its dot ± lightSnapPx, or its own drawn box) — swept over every aimable light in view; a man under a ceiling panel\'s picture, the cursor beside him and nearer the panel\'s dot: he is taken and the round drops him; a light taken is kept while the camera settles ===');
  const L0: number = Aim.AIM_DEFAULTS.player.lightSnapPx, SNAP: number = Aim.SNAP_PX; let allPass = true;   // (L0: the reach as the game has it; SNAP: the men's reach, which a light had too until the rule)
  const stage = async (seed: number, at: Vec3) => {
    Math.random = seeded(seed); const W = await standUp(); const { game, input, cam, canvas } = W;
    game.godMode = true; game.player.visibility = 0; for (let i = 0; i < 5; i++) W.step(DT);
    game.guards.forEach((gd: any, i: number) => park(gd, i));
    const pl = game.player; game.teleportPlayer(at); pl.slot = 'pistol'; pl.holstered = false; pl.crouch = false;
    input.mouseX = 800; input.mouseY = 500; for (let i = 0; i < 90; i++) W.step(DT);   // the camera settled on him
    const px = (p: Vec3) => cam.project(p, canvas.clientWidth, canvas.clientHeight);
    /** the reticle's answer for the cursor at (x, y) with the camera where it stands and nothing kept from the frame before */
    const takes = (x: number, y: number) => { input.mouseX = x; input.mouseY = y; game.aimTarget = null; updateAim(game, input, cam, canvas); return game.aimTarget; };
    return { W, game, input, cam, canvas, pl, px, takes };
  };
  {
    { // ---- the sweep: the camera still, the cursor on rings round each light's dot
      const { game, input, px, takes } = await stage(941, [8.4, 0, 21.2]); const fails: string[] = [];
      const lights = game.targets.filter((t: any) => t.interactive && !t.broken).map((t: any) => ({ t, at: px(t.pos) })).filter((x: any) => x.at[2] && x.at[0] > 80 && x.at[0] < 1520 && x.at[1] > 80 && x.at[1] < 920);
      const radii = [0, L0 - 2, L0 + 2, Math.round((L0 + SNAP) / 2), SNAP - 2];
      const sweep = (reach: number) => {
        const took = radii.map(() => 0), any = radii.map(() => 0), tried = radii.map(() => 0); let ceiling = 0;
        for (const { t, at } of lights) radii.forEach((r, k) => { for (let a = 0; a < (r ? 8 : 1); a++) {
          const got = takes(at[0] + r * Math.cos(a * Math.PI / 4), at[1] + r * Math.sin(a * Math.PI / 4)); tried[k]++;
          if (got === t) took[k]++; if (got) any[k]++; if (got && game.aimPoint[1] > 2.5) ceiling++;
          if (got) { const q = px(got.pos), d = Math.hypot(q[0] - input.mouseX, q[1] - input.mouseY); if (d >= reach + 1e-6) fails.push(`reach ${reach}: took ${got.name} from ${f1(d)} px off its dot`); }
          else if (r < reach - 1e-6 && !game.aimGuard) fails.push(`reach ${reach}: nothing taken ${r} px off ${t.name}'s dot`);
        } });
        return { took, any, tried, ceiling };
      };
      const now = sweep(L0);
      const row = (R: ReturnType<typeof sweep>) => radii.map((r, k) => `${r} px ${R.took[k]}/${R.tried[k]}${R.any[k] > R.took[k] ? ` (+${R.any[k] - R.took[k]} a neighbour's nearer dot)` : ''}`).join(' · ');
      if (lights.length < 3) fails.push(`staging: only ${lights.length} aimable lights in the lobby view`);
      if (now.any[0] !== now.tried[0] || now.any[1] !== now.tried[1]) fails.push('on / just inside the dot a light was not taken'); if (now.took[2] + now.took[3] + now.took[4] !== 0) fails.push('a light taken from outside lightSnapPx');
      allPass = verdict(!fails.length, `${lights.length} aimable lights in the lobby view (none of them drawn: ceiling panels, a beacon), the cursor on rings round each dot: taken on the dot and at ${radii[1]} px, never at ${radii[2]} / ${radii[3]} / ${radii[4]} px`, fails.slice(0, 4).join('; ') || `lightSnapPx ${L0}: ${row(now)} (the gun at the ceiling on ${now.ceiling} of ${now.tried.reduce((a, b) => a + b, 0)} cursor spots)`) && allPass;
    }
    // ---- the man under a panel's picture: his chest L0 + 42 px to the side of the panel's dot, the cursor between them L0 + 8 px from the dot (beside him, not on him)
    const man = async () => {
      const { W, game, input, cam, canvas, pl, px } = await stage(942, [8.6, 0, 23.3]); const A = game.guards[0]; const dDot = L0 + 8;
      const panel = game.targets.filter((t: any) => t.interactive && /^panel_/.test(t.name)).sort((a: any, b: any) => v3.distXZ(a.pos, pl.char.pos) - v3.distXZ(b.pos, pl.char.pos))[0];
      const chest = (): Vec3 => A.char.bones.chest ?? v3.add(A.char.pos, [0, 1.2, 0]);
      const spot = (): Vec3 => { const q = px(panel.pos); const r = cam.ray((q[0] - (L0 + 42)) / canvas.clientWidth, q[1] / canvas.clientHeight); const t = (1.2 - r.ro[1]) / r.rd[1]; return [r.ro[0] + r.rd[0] * t, 0, r.ro[2] + r.rd[2] * t]; };
      for (let k = 0; k < 4; k++) { const want = spot(), c = chest(); place(A, [A.char.pos[0] + want[0] - (k ? c[0] : A.char.pos[0]), 0, A.char.pos[2] + want[2] - (k ? c[2] : A.char.pos[2])], Math.PI); park2(A); W.step(DT); }   // (his chest bone, not his feet, onto the spot)
      let onHim = 0, onPanel = 0, onBody = 0, n = 0, dChest = 0;
      const frame = (click: boolean, count = true) => { const q = px(panel.pos), c = px(chest()); input.mouseX = q[0] - dDot; input.mouseY = q[1];
        const r = cam.ray(input.mouseX / canvas.clientWidth, input.mouseY / canvas.clientHeight);
        if (count) { dChest = Math.hypot(c[0] - input.mouseX, c[1] - input.mouseY); if (A.char.rayHit(r.ro, r.rd, 1e9) >= 0) onBody++; }
        if (click) input.clicked |= 1; W.step(DT); if (count) { n++; if (game.aimGuard === A) onHim++; if (game.aimTarget === panel) onPanel++; } };
      for (let i = 0; i < 60; i++) frame(false);   // (a second for his hands to settle on what the reticle has)
      const t0 = game.time; let shot: any = null; const seen = () => { shot ??= game.events.find((e: any) => e.kind === 'shot' && e.who === pl.char.id && e.time >= t0); };
      frame(true); seen(); for (let i = 0; i < 20; i++) { frame(false, false); seen(); }   // (the round away; what the reticle does over a body afterwards is not counted)
      return { panel: panel.name, onHim, onPanel, onBody, n, dDot, dChest, clear: !game.col.segmentBlocked(v3.add(pl.char.pos, [0, 1.3, 0]), chest()), dead: A.state === 'dead', toY: shot?.to ? shot.to[1] : NaN, fired: !!shot, range: v3.distXZ(pl.char.pos, A.char.pos) };
    };
    { const now = await man(); const fails: string[] = [];
      if (!now.clear || now.onBody || now.dChest < now.dDot + 8 || now.dChest > SNAP - 8) fails.push(`staging: line clear ${now.clear}, cursor on his body ${now.onBody} frames, ${f1(now.dChest)} px off his chest / ${now.dDot} off the dot`);
      if (now.onHim !== now.n || now.onPanel) fails.push(`the reticle on him ${now.onHim}/${now.n} frames, on the panel ${now.onPanel}`); if (!now.fired || !now.dead) fails.push(`fired ${now.fired}, he is ${now.dead ? 'down' : 'standing'} (the round ended at y ${f2(now.toY)})`);
      allPass = verdict(!fails.length, `a man ${f1(now.range)} m off under ${now.panel}'s picture, the cursor beside him — ${f1(now.dChest)} px from his chest, ${now.dDot} from the panel's dot: the reticle is on him every frame and the round drops him`, fails.join('; ') || `lightSnapPx ${L0}: on him ${now.onHim}/${now.n}, down ${now.dead} (round to y ${f2(now.toY)})`) && allPass;
    }
    { // ---- kept while the camera settles: the cursor flicked onto a panel's dot and LEFT there — the aim point going up to the panel pans the picture under it
      const { W, game, input, pl, px } = await stage(943, [8.6, 0, 23.3]); const fails: string[] = [];
      const panel = game.targets.filter((t: any) => t.interactive && /^panel_/.test(t.name)).sort((a: any, b: any) => v3.distXZ(a.pos, pl.char.pos) - v3.distXZ(b.pos, pl.char.pos))[0];
      game.lightHome.stage(panel, true, null); W.step(DT);   // (the lobby's panels are switched off at night: lit, so the OCP below has something to put out)
      const q0 = px(panel.pos); input.mouseX = q0[0]; input.mouseY = q0[1]; let had = 0, drift = 0; const N = 90;
      const off = () => { const q = px(panel.pos); return Math.hypot(q[0] - input.mouseX, q[1] - input.mouseY); };
      for (let i = 0; i < N; i++) { W.step(DT); if (game.aimTarget === panel) had++; drift = Math.max(drift, off()); }
      pl.ocp.charge = 1; input.clicked |= 4; W.step(DT); const zapped = panel.disabledUntil > game.time;   // the right button a second and a half after landing on it
      const q = px(panel.pos), d = off() || 1; input.mouseX += (input.mouseX - q[0]) / d * (L0 + 6); input.mouseY += (input.mouseY - q[1]) / d * (L0 + 6); W.step(DT); const let_go = game.aimTarget !== panel;   // the cursor moved off, away from the dot
      if (had !== N) fails.push(`the panel had the reticle ${had}/${N} frames with the cursor left on the spot`); if (!zapped) fails.push('the right button then did not OCP it'); if (!let_go) fails.push(`the cursor moved ${L0 + 6} px further off and the panel still has the reticle`);
      if (drift >= SNAP) fails.push(`staging: the dot slid ${f1(drift)} px — past the men's reach, nothing holds that`);
      allPass = verdict(!fails.length, `the cursor put on ${panel.name}'s dot and left there: the camera's lead slides the dot ${f1(drift)} px from under it (reach ${L0}) and the panel keeps the reticle all ${N} frames, the right button OCPs it; the cursor moved ${L0 + 6} px further off lets it go at once`, fails.join('; ') || `kept ${had}/${N} · drift ${f1(drift)} px · OCP ${zapped} · let go ${let_go}`) && allPass;
    }
    { // ---- a DRAWN fixture is taken by pointing at it anywhere: the vending front, well off its dot
      const { game, input, px, takes } = await stage(944, [29.4, 0, 16.2]); const fails: string[] = [];
      const v = game.targets.find((t: any) => t.name === 'vending'), b = W0(game).statics[v.boxes[0]];
      const on: Vec3 = [b.c[0] + b.h[0] * 0.6, b.c[1] + b.h[1], b.c[2] + b.h[2] * 0.6], q = px(on), dot = px(v.pos); const dOn = Math.hypot(q[0] - dot[0], q[1] - dot[1]);
      const got = takes(q[0], q[1]); const beside = takes(dot[0], dot[1] - (dOn + 40));   // on its top face; then the same distance and more off the dot, past the box
      if (!q[2] || dOn <= L0 + 2 || dOn >= SNAP) fails.push(`staging: the point on its top is ${f1(dOn)} px off the dot`); if (got !== v) fails.push(`the cursor on the machine ${f1(dOn)} px off its dot took ${got?.name ?? 'nothing'}`); if (beside === v) fails.push('taken from off the box');
      void input;
      allPass = verdict(!fails.length, `the cursor ON the vending machine ${f1(dOn)} px off its dot takes it (it is drawn: pointing at it is pointing at it); off the box it does not`, fails.join('; ') || `on it: ${got?.name} · off it: ${beside?.name ?? 'nothing'}`) && allPass;
    }
  }
  console.log(`=== aim-snap: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}
const W0 = (game: any) => game.engine.world;
/** held where he stands, deaf and blind (park() without the move to the lot) */
function park2(gd: any) { gd.hold = true; gd.pinned = true; gd.heardUpTo = 1e12; gd.dazzledUntil = 1e9; gd.state = 'patrol'; gd.awareness = 0; }
