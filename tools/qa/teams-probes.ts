// The 'teams' probe family (run through probes.ts: `bun run tools/qa/probes.ts teams | teams-table | teams-fight | teams-player | teams-dark | teams-despawn
// [--verbose]`): SIDES for the AI (guards.ts teams — the 'Breach' showcase's two operatives clearing a room of hostiles, all AI) —
// who is hostile to whom, AI men perceiving / targeting / shooting hostile AI men through the same senses they use on Sam, the spawn / despawn / engage seam
// the stage lane calls, and above all IDENTITY: with no unit spawned the guards-versus-Sam game is the game it was, frame for frame and round for round.
//   teams-table     the hostility table, both ways round, for every pair of sides.
//   teams-fight     two operatives (FN2000) and two hostiles (AKS-74U) spawned nine metres apart up the cubicles' north aisle, lit, in each other's sight, all four
//                   put in the fight by engage(): inside six seconds somebody is dead, nobody ever had a man of his own side as his foe, and the three staff guards
//                   parked on the lot said and radioed nothing and the floor's alarm never moved. The engagement's record is the TypeScript's own (the Rust tick
//                   never closes the object engage() hands back): how a fight ended is asked of the men, one side down inside the 40 s. Until 2026-11-07 it also
//                   asked for rounds from both sides (red on the last TypeScript game too) and that the winners come off the alert (the Rust game's own
//                   engagement, which engage() from a check does not open).
//   teams-player    an operative two metres in front of a lit Sam for five seconds never takes him as a foe (friendly) and never fires; a hostile put in the same
//                   place has him as his foe inside a second.
//   teams-dark      a hostile standing in the dark (his room's lights out) round a partition from an alert operative: not acquired in five seconds; the moment he
//                   fires the operative has a fix on him (heard); lights back on and in the open, acquired by sight inside two seconds.
//   teams-despawn   spawn four, despawn them: the guards list, the light list, the character ids and the probe keys are back to what they were. (Until 2026-11-07 a
//                   whole showcase tour was run after it, two minutes of CPU: tools/qa/tour.ts asks that.)
import { standUp, ROOT } from './headless.ts';
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const guardsMod = await import(`${ROOT}/src/game/guards.ts`);
const { resetChase } = guardsMod;
const { fireWeapon } = await import(`${ROOT}/src/game/combat.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60;
const VERBOSE = process.argv.includes('--verbose');
const f1 = (x: number) => x.toFixed(1), f2 = (x: number) => x.toFixed(2);
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
function verdict(ok: boolean, label: string, detail: string) { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}: ${detail}`); return ok; }


// ================================================================ shared staging for the behaviour probes
const teamsMod = await import(`${ROOT}/src/game/teams.ts`);
/** QA: teams-dark's red path — TEAMSDARK_BREAK=1 stands the hostile in the corridor with the operative, no wall between them: the wall's line must fail. */
const DARK_BREAK = process.env.TEAMSDARK_BREAK === '1';
const { spawnUnit, despawnUnit, despawnUnits, units, engage, canSeeChar } = guardsMod;
const chestOf = (c: any): Vec3 => c.bones.chest ?? v3.add(c.pos, [0, 1.1, 0]);
const eyeOf = (c: any): Vec3 => c.bones.head ?? v3.add(c.pos, [0, 1.65, 0]);
/** the three staff parked on the lot east of the building — held, pinned, blind and deaf (standoff-probes' parking): out of every unit's sight and hearing */
function parkStaff(game: any) { game.guards.forEach((gd: any, i: number) => { if (gd.unit) return; gd.hold = true; gd.pinned = true; gd.dazzledUntil = 1e9; gd.heardUpTo = 1e12; gd.char.pos = [37.5, 0, 6 + i * 1.5]; gd.char.vel = [0, 0, 0]; gd.path = []; gd.pathGoal = null; gd.char.update(0); }); }
/** what everybody says (game.say) and radios, by side, from here on */
function tapSay(game: any) {
  const lines: { t: number; who: string; team: string; text: string; radio: boolean }[] = [];
  const orig = game.say.bind(game);
  game.say = (gd: any, text: string, radio = false) => { const r = orig(gd, text, radio); if (gd.bubble && gd.bubble.text === text && gd.bubble.t === game.time) lines.push({ t: game.time, who: gd.callsign, team: gd.team, text, radio }); return r; };
  return lines;
}
/** gunfire events raised since the last call, by the side of who fired (events are re-filtered every frame: identity set) */
function shotCounter(game: any) {
  const seen = new Set<any>(); const byTeam: Record<string, number> = {}; const byWho: Record<string, number> = {};
  return {
    pump() { for (const e of game.events) { if (seen.has(e) || (e.kind !== 'shot' && e.kind !== 'guardShot') || (e.level ?? 0) < 1) continue; seen.add(e); const t = e.kind === 'shot' ? 'player' : teamsMod.teamOfId(game, e.who ?? -1); byTeam[t] = (byTeam[t] ?? 0) + 1; const w = e.kind === 'shot' ? 'Sam' : (game.guards.find((x: any) => x.char.id === e.who)?.callsign ?? `#${e.who}`); byWho[w] = (byWho[w] ?? 0) + 1; } },
    byTeam, byWho,
  };
}

// ================================================================ (a) the hostility table
export async function teamsTable() {
  console.log('\n=== teams-table: who is hostile to whom (teams.ts HOSTILE) — the design\'s table, every pair both ways round ===');
  const { isHostile, TEAMS, HOSTILE } = teamsMod;
  // The FIFTH side is 'civilian' since the civilians merged (d7c3289, teams.ts's own head comment: "hostile to NOBODY and nobody hostile to them … the row is
  // here so a held / downed civilian's Guard record has a side to name"). The sixteen pairs of the four fighting sides are exactly as designed before them; the
  // nine civilian pairs are all neutral — a probe written for four sides read the fifth as a regression, which it is not.
  const want: Record<string, boolean> = {
    'guard→player': true, 'player→guard': true, 'operative→hostile': true, 'hostile→operative': true, 'hostile→player': true, 'player→hostile': true,
    'operative→player': false, 'player→operative': false, 'guard→hostile': false, 'hostile→guard': false, 'guard→operative': false, 'operative→guard': false,
    'guard→guard': false, 'operative→operative': false, 'hostile→hostile': false, 'player→player': false,
    'civilian→guard': false, 'civilian→operative': false, 'civilian→hostile': false, 'civilian→player': false, 'civilian→civilian': false,
    'guard→civilian': false, 'operative→civilian': false, 'hostile→civilian': false, 'player→civilian': false,
  };
  let allPass = true; const rows: string[] = [];
  for (const a of TEAMS) { rows.push(`${a.padEnd(9)} → ` + TEAMS.map((b: string) => `${b} ${isHostile(a, b) ? 'HOSTILE' : '·      '}`).join('  ')); for (const b of TEAMS) { const k = `${a}→${b}`; if (isHostile(a, b) !== want[k]) { allPass = false; console.log(`    ✗ ${k}: ${isHostile(a, b)}, want ${want[k]}`); } } }
  for (const r of rows) console.log('    ' + r);
  allPass = verdict(allPass, 'the table', `${Object.keys(want).length} ordered pairs as designed: staff ↔ Sam hostile (the game as it was), operative ↔ hostile both ways, hostiles would shoot Sam and he them, operatives are Sam's friends, staff and the showcase's sides never meet (neutral), civilians are nobody's enemy and have none, nobody is his own enemy`) && allPass;
  const civRow = TEAMS.every((t: string) => !isHostile('civilian', t) && !isHostile(t, 'civilian'));
  allPass = verdict(civRow, 'the civilian row and column are neutral through and through (nobody acquires, lines up on or is shot at by a cleaner)', TEAMS.map((t: string) => `civilian↔${t} ${isHostile('civilian', t) || isHostile(t, 'civilian') ? 'HOSTILE' : 'neutral'}`).join(' · ')) && allPass;
  allPass = verdict(Object.keys(HOSTILE).length === 5 && TEAMS.length === 5 && TEAMS.join(',') === 'guard,operative,hostile,player,civilian', 'five sides, no more (the four that fight + the civilians)', TEAMS.join(', ')) && allPass;
  console.log(`=== teams-table: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (b) two on two up the cubicles' north aisle
/** One aisle fight off `seed`: the three staff parked, Sam shut in the conference room dark, two operatives at the west end of the cubicles' north aisle and two
 *  hostiles nine metres east of them, everybody facing the other side, half a second for poses and meters to settle, engage() (fair: no surprise), then run to
 *  the engagement's own end (40 s cap). Everything the verdicts need comes back. */
async function aisleFight(seed: number, opts: { surprise?: number } = {}) {
  Math.random = seeded(seed);
  const W = await standUp(); const { game } = W;
  if ((game as any).civilians) { (game.tune as any).civilians = false; (game as any).civilians.length = 0; }   // (civilians scream and run under gunfire: this family's frozen numbers predate them — switched off like the other identity probes)
  game.aiEnabled = true; game.godMode = true;
  for (let i = 0; i < 5; i++) W.step(DT);
  parkStaff(game); game.teleportPlayer([8.0, 0, 7.0]); game.player.visibility = 0.02;   // Sam in the conference room behind its shut door, dark: no part of it
  W.step(DT); W.step(DT);
  const lines = tapSay(game); const shots = shotCounter(game);
  const guards0 = game.guards.length, esc0 = game.escalation;
  const ops = [spawnUnit(game, { team: 'operative', pos: [14.2, 0, 13.35], yaw: Math.PI / 2, name: 'Archer' }), spawnUnit(game, { team: 'operative', pos: [14.0, 0, 12.65], yaw: Math.PI / 2, name: 'Bishop' })];
  const hos = [spawnUnit(game, { team: 'hostile', pos: [23.2, 0, 13.35], yaw: -Math.PI / 2, name: 'Vasquez' }), spawnUnit(game, { team: 'hostile', pos: [23.4, 0, 12.7], yaw: -Math.PI / 2, name: 'Drake' })];
  for (let i = 0; i < 30; i++) W.step(DT);   // posed, and half a second for the meters to settle (eased at Sam's rate from nought)
  const all = [...ops, ...hos];
  const kit = all.map((u: any) => `${u.callsign} ${u.team} ${u.weapon.def.id} rig ${guardsMod.rigOf(u)} longGun ${u.char.anim.longGun} vis ${f2(u.visibility)}`);
  const kitOk = ops.every((u: any) => u.weapon.def.id === 'fn2000' && guardsMod.rigOf(u) === 'sc20k' && u.char.anim.longGun) && hos.every((u: any) => u.weapon.def.id === 'aks74u' && guardsMod.rigOf(u) === 'carbine' && u.char.anim.longGun);
  const los = ops.map((a: any) => hos.map((b: any) => !game.col.segmentBlocked(eyeOf(a.char), chestOf(b.char)) || !game.col.segmentBlocked(eyeOf(a.char), eyeOf(b.char))));
  const stagingOk = los.flat().every(Boolean) && all.every((u: any) => u.visibility > 0.3) && all.every((u: any) => u.state === 'patrol' && u.awareness < 0.3);   // (calm on their marks until the word: nobody has jumped the gun)
  const staging = `LOS ${JSON.stringify(los)}; ${all.map((u: any) => `${u.callsign} vis ${f2(u.visibility)} ${u.state} aw ${f2(u.awareness)}`).join(', ')}; ${f1(v3.distXZ(ops[0].char.pos, hos[0].char.pos))} m apart`;
  const E = engage(game, ops, hos, { timeout: 40, surprise: opts.surprise ?? 0 });
  const foes0 = all.map((u: any) => `${u.callsign}→${u.foe === 'player' ? 'Sam' : u.foe ? u.foe.callsign : '—'}`).join(' ');
  const engagedOk = all.every((u: any) => u.state === 'alert' && u.foe && u.foe !== 'player' && u.foe.team !== u.team && !u.hold && !u.script);
  let sameSide = 0, sameSideWho = ''; let firstKillT = -1; let staffMoved = false; const staffPos = game.guards.filter((x: any) => !x.unit).map((x: any) => v3.copy(x.char.pos));
  const roundsAt6: Record<string, number> = {};
  const decided = () => ops.every((u: any) => u.state === 'dead') || hos.every((u: any) => u.state === 'dead');
  for (let f = 0; f < 60 * 41 && !(decided() && f > 60 * 6); f++) {
    W.step(DT); shots.pump();
    for (const u of all) if (u.foe && u.foe !== 'player' && u.foe.team === u.team) { sameSide++; sameSideWho = `${u.callsign}→${u.foe.callsign}`; }
    if (firstKillT < 0 && all.some((u: any) => u.state === 'dead')) firstKillT = game.time - E.t0;
    game.guards.filter((x: any) => !x.unit).forEach((x: any, i: number) => { if (v3.distXZ(x.char.pos, staffPos[i]) > 0.05 || x.state !== 'patrol') staffMoved = true; });
    if (f === 60 * 6 - 1) Object.assign(roundsAt6, shots.byTeam);
  }
  const after = all.map((u: any) => `${u.callsign}:${u.state}`).join(' ');
  const outcome = hos.every((u: any) => u.state === 'dead') ? 'cleared' : ops.every((u: any) => u.state === 'dead') ? 'wiped' : 'undecided', operativesLeft = ops.filter((u: any) => u.state !== 'dead').length;
  return { game, W, ops, hos, E, lines, shots, kit, kitOk, stagingOk, staging, foes0, engagedOk, sameSide, sameSideWho, firstKillT, staffMoved, roundsAt6, guards0, esc0, after, outcome, operativesLeft };
}
export async function teamsFight() {
  console.log('\n=== teams-fight: two operatives (FN2000) v two hostiles (AKS-74U), 9 m apart up the cubicles\' north aisle, lit, in sight, engage() with no surprise either way — three seeds: a kill inside 6 s every time, nobody\'s foe is ever his own side, the parked staff say / radio / escalate nothing, every fight is decided ===');
  let allPass = true;
  for (const seed of [611, 612, 613]) {
    const R = await aisleFight(seed);
    const { game, E } = R;
    console.log(`  --- seed ${seed} ---`);
    allPass = verdict(R.kitOk, 'spawned as ordered (FN2000s rigged as the bullpup, AKS-74Us as the carbine, long guns armed)', R.kit.join(' · ')) && allPass;
    allPass = verdict(R.stagingOk, 'staging: every operative has a line of sight to every hostile, all four stand lit (meter > 0.3) and calm on their marks', R.staging) && allPass;
    allPass = verdict(R.engagedOk, 'engage(): all four alert, off their marks, each on an ENEMY', R.foes0) && allPass;
    allPass = verdict(R.firstKillT >= 0 && R.firstKillT <= 6, 'inside six seconds somebody is dead', R.firstKillT >= 0 ? `first man down at +${f2(R.firstKillT)} s` : 'nobody died in 6 s') && allPass;
    allPass = verdict(!R.roundsAt6.player && !R.roundsAt6.guard && ((R.roundsAt6.operative ?? 0) + (R.roundsAt6.hostile ?? 0)) > 0, 'the rounds in those six seconds are the units\' — none from Sam or the staff', `${JSON.stringify(R.roundsAt6)} · by man ${JSON.stringify(R.shots.byWho)}`) && allPass;
    allPass = verdict(R.sameSide === 0, 'nobody ever had a man of his own side as his foe', R.sameSide ? `${R.sameSide} guard-frames, e.g. ${R.sameSideWho}` : `0 frames over ${f1(game.time - E.t0)} s`) && allPass;
    const staffLines = R.lines.filter((l: any) => l.team === 'guard');
    allPass = verdict(staffLines.length === 0 && game.escalation === R.esc0 && !game.alarm.episode && guardsMod.rollcallState(game).stage === 'idle' && !R.staffMoved, 'the parked staff: not a word, not a radio call, no alarm episode, no lockdown, nobody moved', `staff lines ${staffLines.length}${staffLines.length ? ` (${staffLines.slice(0, 3).map((l: any) => `${l.who}: ${l.text}`).join(' | ')})` : ''}; escalation ${game.escalation}; episode ${game.alarm.episode}; radio check ${guardsMod.rollcallState(game).stage}; moved ${R.staffMoved}`) && allPass;
    allPass = verdict(R.outcome !== 'undecided', 'the fight is decided inside its 40 s: one side down', `${R.outcome} · ${R.after}`) && allPass;
    const unitLines = R.lines.filter((l: any) => l.team !== 'guard');
    console.log(`    (the units said: ${unitLines.slice(0, 10).map((l: any) => `${l.who} "${l.text}"`).join(' | ')}${unitLines.length > 10 ? ' …' : ''})`);
    if (VERBOSE) for (const u of [...R.ops, ...R.hos]) console.log(`    ${u.callsign} (${u.team}): ${u.state}${u.state === 'dead' ? ` killer #${u.killerId}` : ''} at ${f1(u.char.pos[0])},${f1(u.char.pos[2])} · rounds ready ${u.weapon.roundsReady} · sup ${f2(u.sup.v)}`);
    allPass = verdict(game.guards.length === R.guards0 + 4, 'all four still in the guards list (bodies stay until despawned)', `${game.guards.length} guards`) && allPass;
  }
  // and the stage's knob: the room caught flat (surprise 1.0 s) is the operatives' to win
  { const R = await aisleFight(611, { surprise: 1.0 }); allPass = verdict(R.outcome === 'cleared' && R.operativesLeft === 2, 'engage(…, { surprise: 1 }): the same aisle with the room a second slow — cleared, both operatives standing', `${R.outcome} · ${R.after}`) && allPass; }
  console.log(`=== teams-fight: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (c) an operative never takes Sam; a hostile does
export async function teamsPlayer() {
  console.log('\n=== teams-player: a lit Sam two metres in front of an ALERT operative for 5 s — never his foe, never a round; a hostile stood in the same place has him inside a second and fires ===');
  Math.random = seeded(612);
  const W = await standUp(); const { game } = W; const pl = game.player;
  game.aiEnabled = true; game.godMode = true;
  for (let i = 0; i < 5; i++) W.step(DT);
  parkStaff(game);
  game.teleportPlayer([21.5, 0, 11.3]); pl.slot = 'pistol'; pl.holstered = false; pl.crouch = false; W.step(DT); W.step(DT); pl.visibility = 0.9;
  const shots = shotCounter(game);
  let allPass = true;
  // the operative: alert (as engage would leave him after a fight), nobody acquired, Sam lit at 2 m dead ahead
  const op = spawnUnit(game, { team: 'operative', pos: [19.5, 0, 11.3], yaw: Math.PI / 2, name: 'Archer' });
  op.hold = false; op.state = 'alert'; op.awareness = 1; op.reactT = 0; op.fireCd = 0.2; W.step(DT);
  let everFoe = false, hits = 0, sawSam = false, visibleFrames = 0;
  for (let f = 0; f < 60 * 5; f++) { pl.visibility = 0.9; W.step(DT); shots.pump(); if (op.foe) everFoe = true; if (op.sawPlayerThisFrame || op.sawFoeThisFrame) sawSam = true; if (canSeeChar(game, op, pl.char, true).visible) visibleFrames++; if (pl.hitFlash > 0.9) hits++; pl.hitFlash = 0; }
  allPass = verdict(!everFoe && !sawSam && (shots.byTeam.operative ?? 0) === 0 && hits === 0, 'operative: Sam in plain sight (the sight test itself says visible), yet never a foe, never "seen", not one round, Sam untouched', `visible to the sight test on ${visibleFrames} of 300 frames · foe ever ${everFoe} · saw ${sawSam} · operative rounds ${shots.byTeam.operative ?? 0} · hits ${hits} · his state at the end ${op.state}`) && allPass;
  allPass = verdict(visibleFrames >= 60, 'staging: the operative COULD see him for at least a second of it (so the verdict above is the table, not a wall or his back)', `dist ${f1(v3.distXZ(op.char.pos, pl.char.pos))} m, Sam vis ${f2(pl.visibility)}, ${visibleFrames} frames`) && allPass;
  despawnUnit(game, op); W.step(DT);
  // the hostile in the same place, calm on his mark (patrol, held): his eyes do the rest
  const ho = spawnUnit(game, { team: 'hostile', pos: [19.5, 0, 11.3], yaw: Math.PI / 2, name: 'Vasquez' });
  W.step(DT);
  let acquiredAt = -1, alertAt = -1, firedAt = -1; const t0 = game.time; const before = shots.byTeam.hostile ?? 0;
  for (let f = 0; f < 60 * 5; f++) {
    pl.visibility = 0.9; W.step(DT); shots.pump();
    if (acquiredAt < 0 && ho.foe === 'player' && ho.sawFoeThisFrame) acquiredAt = game.time - t0;
    if (alertAt < 0 && ho.state === 'alert') alertAt = game.time - t0;
    if (firedAt < 0 && (shots.byTeam.hostile ?? 0) > before) firedAt = game.time - t0;
    pl.hitFlash = 0;
  }
  allPass = verdict(acquiredAt >= 0 && acquiredAt <= 1.0 && alertAt >= 0 && alertAt <= 1.5, 'hostile: eyes on Sam inside a second, CONTACT inside a second and a half', `saw him +${f2(acquiredAt)} s, alert +${f2(alertAt)} s`) && allPass;
  allPass = verdict(firedAt >= 0, 'hostile: and he fires on him', firedAt >= 0 ? `first burst +${f2(firedAt)} s (${shots.byTeam.hostile} rounds in 5 s)` : 'no round in 5 s') && allPass;
  despawnUnits(game);
  console.log(`=== teams-player: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (d) line of sight and light gate acquisition exactly as they gate a guard's sight of Sam
export async function teamsDark() {
  console.log('\n=== teams-dark: an alert operative and a hostile behind a wall: never acquired, until the hostile fires and is placed by ear ===');
  Math.random = seeded(613);
  const W = await standUp(); const { game } = W;
  game.aiEnabled = true; game.godMode = true;
  for (let i = 0; i < 5; i++) W.step(DT);
  parkStaff(game); game.teleportPlayer([37.0, 0, 26.0]); game.player.visibility = 0; W.step(DT); W.step(DT);   // Sam out on the lot's far corner: no part of it
  let allPass = true;
  // ---- (1) the wall: operative in the corridor facing west, hostile in the server room beyond the corridor's north wall
  {
    const op = spawnUnit(game, { team: 'operative', pos: [22.0, 0, 11.3], yaw: -Math.PI / 2, name: 'Archer', light: false });
    const ho = spawnUnit(game, { team: 'hostile', pos: DARK_BREAK ? [16.5, 0, 11.3] : [16.5, 0, 7.5], yaw: Math.PI / 2, name: 'Vasquez', light: false });
    op.hold = false; op.pinned = true; op.state = 'alert'; op.awareness = 1; op.reactT = 0; op.lastKnown = null;   // alert with nothing, planted (pinned: he covers, he does not wander off to search)
    ho.heardUpTo = 1e12; ho.hold = true; ho.pinned = true; ho.state = 'patrol'; ho.awareness = 0;   // the hostile is a prop here: deaf behind his wall, he stands, calm (a dazzled man fires blind, and this one fires only below)
    W.step(DT); W.step(DT);
    const wall = game.col.segmentBlocked(eyeOf(op.char), chestOf(ho.char)) && game.col.segmentBlocked(eyeOf(op.char), eyeOf(ho.char));
    let everFoe = false;
    for (let f = 0; f < 60 * 3; f++) { W.step(DT); if (op.foe) everFoe = true; }
    allPass = verdict(wall && !everFoe && !op.lastKnown, '(1) a wall between them: three seconds, never acquired, no fix', `wall ${wall} · foe ever ${everFoe} · fix ${op.lastKnown ? 'set' : 'none'} · hostile lit ${f2(ho.visibility)}`) && allPass;
    // the hostile fires one round into the wall in front of him: the operative hears an ENEMY's report and places it
    fireWeapon(game, ho.char, [19.0, 1.2, 7.5], false, { spread: 0, steady: true });
    for (let f = 0; f < 30; f++) W.step(DT);
    const fix = op.lastKnown; const dFix = fix ? v3.distXZ(fix, ho.char.pos) : Infinity;
    allPass = verdict(!!fix && dFix < 4.5 && op.witness.heard.shots >= 1 && !op.foe, '(1) he fires: heard as a threat — a fix within a few metres of him (placed by ear through the doorway), still no foe (nothing seen)', `fix ${fix ? `(${f1(fix[0])}, ${f1(fix[2])})` : 'none'} ${f1(dFix)} m off him · shots heard ${op.witness.heard.shots} · foe ${op.foe ? 'set' : 'none'}`) && allPass;
    despawnUnits(game); W.step(DT);
  }
  // (Until 2026-11-07 a part (2) asked SIGHT BY THE LIGHT METER ALONE: the two in the open with the floor's lights out, "five seconds and never acquired", then
  //  lights on. It was asked "with nobody's goggles on" (NVG.on written off): the goggles are settled on, the team sees a dark man through the tube, and the
  //  question was a build from before them. Out by ruling; probes.ts nvg has what the goggles add.)
  console.log(`=== teams-dark: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (e) despawn cleans up; the tour runs clean in the same process after
export async function teamsDespawn() {
  console.log('\n=== teams-despawn: spawn four, fight a little, despawn — guards / steady lights / ids / meter keys back as they were, the budget errors say what they mean ===');
  Math.random = seeded(614);
  const W = await standUp(); const { game } = W;
  game.aiEnabled = true; game.godMode = true;
  for (let i = 0; i < 5; i++) W.step(DT);
  let allPass = true;
  const steady = () => game.engine.lights.lights.filter((l: any) => l.ttl < 0).length;
  const ids = () => game.guards.map((x: any) => x.char.id).join(',');
  const g0 = game.guards.length, L0 = steady(), ids0 = ids();
  parkStaff(game); game.teleportPlayer([37.0, 0, 26.0]); W.step(DT);
  const ops = [spawnUnit(game, { team: 'operative', pos: [14.2, 0, 13.35], yaw: Math.PI / 2 }), spawnUnit(game, { team: 'operative', pos: [14.0, 0, 12.65], yaw: Math.PI / 2 })];
  const hos = [spawnUnit(game, { team: 'hostile', pos: [23.2, 0, 13.35], yaw: -Math.PI / 2 }), spawnUnit(game, { team: 'hostile', pos: [23.4, 0, 12.7], yaw: -Math.PI / 2 })];
  allPass = verdict(game.guards.length === g0 + 4 && steady() === L0 + 4 && units(game).length === 4 && new Set(game.guards.map((x: any) => x.char.id)).size === game.guards.length, 'spawned: +4 guards, +4 steady lights, four distinct fresh ids', `guards ${g0} → ${game.guards.length} · steady lights ${L0} → ${steady()} · ids ${ids()}`) && allPass;
  // the character budget: Sam + 3 staff + 4 units = 8 — a ninth must be refused, and say why
  let err = ''; try { spawnUnit(game, { team: 'hostile', pos: [20, 0, 18] }); } catch (e: any) { err = String(e?.message ?? e); }
  allPass = verdict(/budget/.test(err) && game.guards.length === g0 + 4, 'a ninth character is refused with the budget named, nothing half-spawned', err ? `"${err.slice(0, 110)}…"` : 'no error thrown') && allPass;
  let errP = ''; try { spawnUnit(game, { team: 'player', pos: [20, 0, 18] }); } catch (e: any) { errP = String(e?.message ?? e); }
  allPass = verdict(/the player alone/.test(errP), "the 'player' side cannot be spawned", errP ? `"${errP.slice(0, 80)}…"` : 'no error thrown') && allPass;
  { const was = game.tune.teams; game.tune.teams = false; let errT = ''; try { spawnUnit(game, { team: 'hostile', pos: [20, 0, 18] }); } catch (e: any) { errT = String(e?.message ?? e); } game.tune.teams = was;
    allPass = verdict(/switched off/.test(errT) && was === true, 'the kill switch: tune.teams is on by default, and off it refuses any spawn', errT ? `default ${was} · "${errT.slice(0, 70)}…"` : 'no error thrown') && allPass; }
  engage(game, ops, hos, { timeout: 20 });
  for (let f = 0; f < 60 * 4; f++) W.step(DT);
  const dead = [...ops, ...hos].filter((u: any) => u.state === 'dead').length;
  game.spectate = game.guards.indexOf(hos[1]);   // the camera on a unit: despawn must not leave it dangling
  despawnUnits(game);
  const dangling = game.guards.some((o: any) => (o.foe && o.foe !== 'player' && !game.guards.includes(o.foe)) || (o.standoff && !game.guards.includes(o.standoff.pair)));
  allPass = verdict(game.guards.length === g0 && steady() === L0 && ids() === ids0 && units(game).length === 0 && game.spectate === -1 && !dangling, 'despawned: guards, steady lights and ids exactly as before, camera back on Sam, no reference left behind', `guards ${game.guards.length} · steady lights ${steady()} · ids ${ids()} · spectate ${game.spectate} · (${dead} of them had died in the 4 s)`) && allPass;
  for (let f = 0; f < 10; f++) W.step(DT);
  // ids do not leak: a fresh spawn starts again at the bottom of the unit range
  const again = spawnUnit(game, { team: 'hostile', pos: [23.2, 0, 13.35] }); const idAgain = again.char.id; despawnUnit(game, again);
  allPass = verdict(idAgain === 20, 'ids come back: the next spawn is #20 again', `#${idAgain}`) && allPass;
  // resetGuards under living units: they go, the staff come back fresh
  spawnUnit(game, { team: 'operative', pos: [14.2, 0, 13.35] }); spawnUnit(game, { team: 'hostile', pos: [23.2, 0, 13.35] }); W.step(DT);
  game.restartEncounter(); W.step(DT);
  allPass = verdict(game.guards.length === g0 && steady() === L0 && units(game).length === 0 && game.guards.every((x: any) => x.team === 'guard' && x.foe === 'player' && x.state === 'patrol'), 'restartEncounter with units standing: units gone, staff fresh on their routes on Sam', `guards ${game.guards.length} · steady lights ${steady()} · ${game.guards.map((x: any) => `${x.callsign}:${x.team}/${x.state}`).join(' ')}`) && allPass;
  console.log(`=== teams-despawn: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}
