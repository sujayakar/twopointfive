// The 'grab' probe family (run through probes.ts: `bun run tools/qa/probes.ts grab | grab-gate | grab-walk | grab-rollcall | grab-talk | grab-choke | grab-release |
// grab-auto`): slice 1 of docs/internal/grab-interrogate-design.md under the row grammar (interact.ts, verbs.ts): the 'grab him' row on a living guard from
// behind takes him — a Space TAP on it grabs on the tap's frame, Space HELD opens the list and grabs on the release (player.ts startGrab / updateHold; the silent
// takedown is a left click with the hands empty), the pair moves as one with his key opening keyed doors, his 'question him' row (a tap: it is the highlighted
// one) runs the dialogue engine on its own pacing, his 'choke him out' row chosen and tapped puts him out (killGuard 'choked': out cold, breathing, the
// ragdoll) unless a second tap stops it, there is no letting him go (E draws the pistol to his head instead — the gun variant from there, gunhold-probes.ts
// hold-draw); a held man is off the AI and off the net (guards.ts held branch / offNet). Each stands the full game up headless (headless.ts), stages one
// situation, drives it through the REAL input path where the grammar is the point (input.keys / pressed / clicked, the rows), prints what happened, PASS /
// FAIL. All of it with the pistol HOLSTERED first (armStandUp): the grab takes the arm variant — the one this family is about; the pistol-to-the-head variant a
// drawn gun would make of the same press has its own family (gunhold-probes.ts).
import { standUp, ROOT } from './headless.ts';
import { holdRelease, endThenStop, endFor } from './verbs.ts';
const IK = 'Space';
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { hitPlayer } = await import(`${ROOT}/src/game/combat.ts`);
const playerMod = await import(`${ROOT}/src/game/player.ts`);
const { startGrab, buildInteractables } = playerMod;
/** how the grab's Space goes down in these probes by default: a one-frame tap (0 — down and up inside the frame, the tour's and the harness's kind), which grabs
 *  in that very frame as the old press-is-the-grab did, so every pose / timing number downstream is staged exactly as it was; pass a length to grabViaHold for
 *  the hold-and-let-go path instead (the grab then lands on the release — and the staged man is free and walking, so keep it short or he is out of reach). */
const PRESS_SECS = 0;
const { HOLD } = await import(`${ROOT}/src/game/character.ts`);
const { isBreathing } = await import(`${ROOT}/src/game/game.ts`);
const guardsMod = await import(`${ROOT}/src/game/guards.ts`);
const { rollcallState, witnessSummary, offNet } = guardsMod;
const Dlg = await import(`${ROOT}/src/game/dialogue.ts`);
/** the row would take the next question without cutting anything: nothing asked, no line of Sam's up this hold, and the man's answer down or in its last half
 *  second (the rule's own function went with the tick; this reads the hold and the two bubbles) */
export const heldSettled = (game: any, H: any): boolean => { const b = H.g.bubble, sb = game.player.bubble, T = H.g.talk;
  const speaking = b && T.lastT >= 0 && Math.abs(b.t - T.lastT) < 1e-6 && game.time - b.t < b.dur ? b : null;
  return !H.ask && !(sb && sb.t >= H.since - 1e-6 && game.time - sb.t < sb.dur - 1e-9) && (!speaking || game.time + 1e-6 >= speaking.t + speaking.dur - 0.5); };

type Vec3 = [number, number, number];
const DT = 1 / 60;
/** the game stood up with Sam's pistol holstered (E), so a grab is the ARM variant (player.ts grabVariant: drawn → 'gun', holstered / canister → 'arm') */
async function armStandUp() {
  const W = await standUp(); W.game.player.holstered = true;
  return W;
}
/** WRITTEN BEFORE TWO SETTLED RULES. grab-gate, grab-release and controls-behind were written before the close-in mode (close.ts CLOSE.on: the knife /
 *  knockout pair of clicks, the take from in front, the alert man still in reach of a blow) and read the one-button rows of the build without it; and every
 *  probe here read the list on a held Space as it was built before the ring that stays (handring.ts). Until 2026-11-07 armStandUp wrote both rules off. Both
 *  are settled on and the writes were refused, so they are out: the lines are asked of the rows the page has. A line that fails for the old rows is one to
 *  restage on the mode's own rows (family `close`) and the ring's (probes.ts handring), or to delete. */
const { KITANIM } = await import(`${ROOT}/src/game/kitanim.ts`);
/** QA: controls-behind's red path — CONTROLSBEHIND_BREAK=1 makes the click of (d) and of (f) come after the rule's own window has shut, so the round and the throw
 *  are LATE: both lines must fail. */
const LATE_BREAK = process.env.CONTROLSBEHIND_BREAK === '1';
/** how many frames after a click its round (a gun still coming up) or its throw (a can still coming to the fist) may leave: the click's own frame — or, with
 *  the kit's moves played, the longest the rule keeps such a click (KITANIM.clickKeep / throwKeep, read from the game's table) and a frame for the hand-over */
const dueFrames = (keepSecs: number): number => 1 + (KITANIM.on ? Math.ceil(keepSecs / DT) + 1 : 0);
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
function place(gd: any, p: Vec3, yaw: number) { gd.char.pos = v3.copy(p); gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.char.bodyYaw = gd.char.aimYaw = yaw; gd.char.aimPitch = 0; gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.char.update(DT); }
/** parked out on the lot east of the building, held (the scripted flag), facing east — deaf too unless said otherwise */
function park(gd: any, i: number, deaf = true) { gd.hold = true; place(gd, [37.5, 0, 6 + i * 1.5], Math.PI / 2); if (deaf) gd.heardUpTo = 1e12; }
function tap(game: any) {
  const lines: { t: number; who: string; text: string; radio: boolean; held: boolean }[] = []; const msgs: { t: number; text: string }[] = [];
  const origSay = guardsMod.say;
  const gsay = (game as any).say.bind(game);
  game.say = (gd: any, text: string, radio = false) => { const r = gsay(gd, text, radio); if (gd.bubble && gd.bubble.text === text && gd.bubble.t === game.time) lines.push({ t: game.time, who: gd.callsign, text, radio, held: !!gd.held }); return r; };   // (only what was actually SAID: guards.ts say drops a held man's barks)
  const origMsg = game.msg.bind(game); game.msg = (text: string) => { msgs.push({ t: game.time, text }); return origMsg(text); };
  void origSay;
  return { lines, msgs };
}
function tapKey(W: any, code: string) { W.input.pressed.add(code); }
/** how far a body's circle sits inside the statics right now (its own push-out re-run on a copy) */
function penetration(col: any, p: Vec3, r: number, y0: number, y1: number): number { const q = v3.copy(p); col.collideCircle(q, r, y0, y1, 3); return v3.distXZ(q, p); }
const f2 = (x: number) => x.toFixed(2), f3 = (x: number) => x.toFixed(3);
const posS = (p: Vec3) => `(${f2(p[0])}, ${f2(p[2])})`;

/** The one staging most of these share: A (the corridor man, Kowalski — steady) planted at `at` facing `yaw`, calm, free (not the scripted hold); B and C parked deaf on
 *  the lot; Sam crouched dark 0.8 m behind A; one settled frame. */
async function stage(seed: number, at: Vec3 = [14, 0, 11.2], yaw = Math.PI / 2, opts: { parkOthers?: boolean } = {}) {
  Math.random = seeded(seed);
  const W = await armStandUp(); const { game } = W;
  game.player.visibility = 0.02; game.godMode = true;
  for (let i = 0; i < 5; i++) W.step(DT);
  const [A, B, C] = game.guards;
  place(A, at, yaw); A.wp = 1;
  if (opts.parkOthers !== false) { park(B, 0); park(C, 1); }
  const behind: Vec3 = [at[0] - Math.sin(yaw) * 0.8, 0, at[2] - Math.cos(yaw) * 0.8];
  game.teleportPlayer(behind); game.player.crouch = true; W.cam.yaw = 0; W.cam.rebuild?.(); W.step(DT); game.player.visibility = 0.02;
  return { W, game, A, B, C, ...tap(game) };
}
/** grab A through the real press: the cursor on him, Space down for `secs`, then up — the grab lands on that release frame; returns the frame the hold began (−1 never) */
function grabViaHold(S: any, secs = PRESS_SECS): number {
  const { W, game, A } = S; let at = -1;
  W.cursorAt(v3.add(A.char.pos, [0, 1.35, 0]));
  if (secs <= DT) { W.input.pressed.add(IK); for (let i = 0; i < 3; i++) { W.step(DT); if (at < 0 && game.player.holding) at = Math.round(game.time / DT); } }   // a one-frame tap: down and up inside the frame — it grabs in that frame, the frame the old press did (the pose probes' numbers stay put)
  else holdRelease(W, IK, secs, () => { if (at < 0 && game.player.holding) at = Math.round(game.time / DT); });
  return at;
}
/** step until the hold proper has begun (phase 'held'), at most 2 s */
function untilHeld(S: any): boolean { const { W, game } = S; for (let i = 0; i < 120; i++) { if (game.player.holding?.phase === 'held') return true; W.step(DT); } return game.player.holding?.phase === 'held'; }

// ================================================================ (1) the gate: wherever the takedown is offered the grab is, and nowhere else — never through a wall or a leaf, never from the front, never on an alert man
export async function grabGate() {
  console.log('\n=== grab-gate: the grab row is not offered, and a Space hold-and-release grabs nobody, out of reach or through glass, a partition or a wall ===');
  let allPass = true;
  const cases: { note: string; guard: Vec3; yaw: number; player: Vec3; expect: boolean }[] = [
    { note: 'behind him but 1.6 m off: refused (get closer)', guard: [14.8, 0, 11.2], yaw: Math.PI / 2, player: [13.2, 0, 11.2], expect: false },
    { note: 'corridor man with his back to the conference glazing (z=10), Sam inside the conference room behind the glass: refused', guard: [12.0, 0, 10.45], yaw: 0, player: [12.0, 0, 9.6], expect: false },
    { note: "man in the manager's office, back to the server-room partition (x=22), Sam in the server room: refused", guard: [22.4, 0, 6.0], yaw: Math.PI / 2, player: [21.55, 0, 6.0], expect: false },
    { note: 'man just inside the break room, back to its north wall (z=12.2), Sam in the corridor: refused', guard: [30.5, 0, 12.65], yaw: 0, player: [30.5, 0, 11.75], expect: false },
  ];
  for (const c of cases) {
    Math.random = seeded(11);
    const W = await armStandUp(); const { game, cam, canvas } = W;
    game.godMode = true; game.player.visibility = 0.02;
    for (let i = 0; i < 5; i++) W.step(DT);
    const gd = game.guards[0]; for (const o of game.guards.slice(1)) park(o, game.guards.indexOf(o));
    place(gd, c.guard, c.yaw); gd.hold = true;
    game.teleportPlayer(v3.copy(c.player)); game.player.crouch = true; W.step(DT);
    const p = game.player.char.pos, gp = gd.char.pos; const p0 = v3.copy(p);
    const wall = game.col.segmentBlocked([p[0], 1.0, p[2]], [gp[0], 1.0, gp[2]]);
    buildInteractables(game, cam, 0, 0, canvas.clientWidth, canvas.clientHeight);
    const it = game.interactables.find((x: any) => x.kind === 'guard' && x.guard === gd);
    const offersTd = !!it?.inReach && /takedown/.test(it.line2), offersGrab = !!it?.inReach && /grab/.test(it.line2);
    // the real press: cursor on him, Space down for half a second, then up (the grab lands on the release)
    W.cursorAt(v3.add(gp, [0, 1.35, 0]));
    holdRelease(W, IK, 0.5);
    const grabbed = !!game.player.holding || !!gd.held;
    for (let i = 0; i < 60; i++) W.step(DT);
    const q = game.player.char.pos; const crossed = game.col.segmentBlocked([p0[0], 1.0, p0[2]], [q[0], 1.0, q[2]]) && game.col.segmentBlocked([p0[0], 0.5, p0[2]], [q[0], 0.5, q[2]]);
    const fails: string[] = [];
    if (offersTd !== offersGrab) fails.push(`row offers takedown=${offersTd} but grab=${offersGrab} ("${it?.line2}")`);
    if (offersGrab !== c.expect) fails.push(`grab offered=${offersGrab}, expected ${c.expect} ("${it?.line1}" / "${it?.line2}")`);
    if (grabbed !== c.expect) fails.push(`after 0.5 s of Space on him: holding=${grabbed}, expected ${c.expect}`);
    if (crossed) fails.push(`Sam ended on the far side of static geometry: ${posS(p0)} → ${posS(q)}`);
    if (!c.expect && gd.state === 'dead') fails.push('refused, yet he went down (the tap fell through to a takedown through the wall?)');
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${c.note}\n    ${f2(v3.distXZ(p0, c.guard))} m, wall between ${wall}; row: ${it ? `"${it.line1}" / "${it.line2}" inReach=${it.inReach}` : 'none'}; after the hold-press: holding=${grabbed}${grabbed ? ` (${game.player.holding?.phase ?? '—'}, ${f3(v3.distXZ(q, gd.char.pos))} m apart, guard ${gd.state})` : ''}, Sam ${posS(q)}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  console.log(`=== grab-gate: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (2) the pair's kinematics: locked HOLD.dist apart on one facing through a 5 s walk that takes a keyed door on HIS key
export async function grabWalk() {
  console.log('\n=== grab-walk: the pair walks 5 s up the corridor and through the (locked, keyed) server-room door on his key — offset, pace, walls, the leaf ===');
  let allPass = true;
  for (const [label, hz] of [['60 Hz', 60], ['20 Hz', 20]] as [string, number][]) {
    const dt = 1 / hz;
    Math.random = seeded(21);
    const W = await armStandUp(); const { game } = W; const { msgs } = tap(game);
    game.godMode = true; game.player.visibility = 0.02;
    for (let i = 0; i < 5; i++) W.step(dt);
    const [A, B, C] = game.guards; park(B, 0); park(C, 1);
    const door = game.doors.byName('server'); const fc = door.frameCentre;
    place(A, [17.62, 0, 11.05], Math.PI); A.wp = 1;   // a metre south of the server door, facing it (north = −z = yaw π)
    game.teleportPlayer([17.62, 0, 11.85]); game.player.crouch = true; W.step(dt);
    const lockedBefore = door.locked && door.latched && game.doors.lockedOut(door, game.player.char.pos);
    startGrab(game, A);
    for (let i = 0; i < Math.round(0.6 * hz); i++) W.step(dt);
    const phase0 = game.player.holding?.phase;
    // drive the pair north through the door and three metres into the server room: the puppet walks Sam at a goal and puts the cursor beyond it (the pair turns to the cursor)
    const goal: Vec3 = [17.62, 0, 7.3];
    game.puppet = { goal, aim: [17.62, 1.0, 4.5] };
    let maxDev = 0, maxDevAt = '', maxSpeed = 0, maxPenP = 0, maxPenG = 0, crossedP = false, crossedG = false, maxYawDiff = 0, unlatchedAt = -1, throughAt = -1, maxStep = 0;
    let prevP = v3.copy(game.player.char.pos), prevG = v3.copy(A.char.pos); const col = game.col;
    const T = 6.5; const frames = Math.round(T * hz); const devLog: string[] = [];
    for (let f = 0; f < frames; f++) {
      W.step(dt);
      const p = game.player.char.pos, gp = A.char.pos; const H = game.player.holding;
      if (!H) { devLog.push(`${f2(game.time)}s hold ENDED (A ${A.state}, held ${!!A.held})`); break; }
      const d = v3.distXZ(p, gp); const dev = Math.abs(d - HOLD.dist);
      if (dev > maxDev) { maxDev = dev; maxDevAt = `${f2(game.time)}s at ${posS(gp)} (door ${f2(door.angle)})`; }
      const sp = Math.hypot(game.player.char.vel[0], game.player.char.vel[2]); maxSpeed = Math.max(maxSpeed, sp, v3.distXZ(p, prevP) / dt);
      maxPenP = Math.max(maxPenP, penetration(col, p, playerMod.playerBodyR(game, p), 0.2, 1.7)); maxPenG = Math.max(maxPenG, penetration(col, gp, 0.3, 0.2, 1.5));
      if (v3.distXZ(p, prevP) > 1e-3 && col.segmentBlocked([prevP[0], 0.45, prevP[2]], [p[0], 0.45, p[2]], 0) && !col.segmentBlockedDynamic([prevP[0], 0.45, prevP[2]], [p[0], 0.45, p[2]])) crossedP = true;
      if (v3.distXZ(gp, prevG) > 1e-3 && col.segmentBlocked([prevG[0], 0.45, prevG[2]], [gp[0], 0.45, gp[2]], 0) && !col.segmentBlockedDynamic([prevG[0], 0.45, prevG[2]], [gp[0], 0.45, gp[2]])) crossedG = true;
      maxStep = Math.max(maxStep, v3.distXZ(gp, prevG));
      const dy = Math.abs(Math.atan2(Math.sin(game.player.char.bodyYaw - A.char.bodyYaw), Math.cos(game.player.char.bodyYaw - A.char.bodyYaw))); maxYawDiff = Math.max(maxYawDiff, dy);
      if (unlatchedAt < 0 && !door.latched) unlatchedAt = game.time;
      if (throughAt < 0 && p[2] < 9.7 && gp[2] < 9.7) throughAt = game.time;
      if (f % Math.round(hz / 2) === 0) devLog.push(`${f2(game.time)}s Sam ${posS(p)} him ${posS(gp)} d ${f3(d)} v ${f2(sp)} door ${f2(door.angle)}${door.latched ? 'L' : ''}${door.locked ? 'k' : ''}`);
      prevP = v3.copy(p); prevG = v3.copy(gp);
    }
    game.puppet = null;
    const fails: string[] = [];
    if (!lockedBefore) fails.push('staging: the server door was not locked against Sam from the corridor');
    if (phase0 !== 'held') fails.push(`0.6 s after startGrab the phase is ${phase0} (want held)`);
    if (!game.player.holding || !A.held) fails.push('the hold ended during the walk');
    if (maxDev > 0.03) fails.push(`pair offset strayed ${f3(maxDev)} m from ${HOLD.dist} (> 0.03) — worst ${maxDevAt}`);
    if (maxSpeed > 0.85 + 0.06) fails.push(`pair reached ${f2(maxSpeed)} m/s (hold pace is 0.85)`);
    if (maxPenP > 0.03) fails.push(`Sam's circle ${f3(maxPenP)} m into static geometry at worst`); if (maxPenG > 0.03) fails.push(`the held man's circle ${f3(maxPenG)} m into static geometry at worst`);
    if (crossedP) fails.push('Sam crossed static geometry between frames'); if (crossedG) fails.push('the held man crossed static geometry between frames');
    if (maxStep > Math.max(0.3, 12 * dt + 0.06)) fails.push(`the held man popped ${f3(maxStep)} m in one frame`);
    if (maxYawDiff > 0.02) fails.push(`yaws differed by up to ${f3(maxYawDiff)} rad`);
    if (unlatchedAt < 0) fails.push('the locked leaf never unlatched for his key'); if (throughAt < 0) fails.push(`the pair never got through the doorway (Sam ${posS(game.player.char.pos)}, him ${posS(A.char.pos)})`);
    if (A.state !== 'patrol' || A.awareness > 0.01) fails.push(`the held man's AI moved: ${A.state} aw ${f2(A.awareness)}`);
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}: offset within ${f3(maxDev)} m of ${HOLD.dist} (worst ${maxDevAt || '—'}); top speed ${f2(maxSpeed)} m/s; static penetration Sam ${f3(maxPenP)} / him ${f3(maxPenG)} m; yaw diff ≤ ${f3(maxYawDiff)}; leaf unlatched at ${unlatchedAt >= 0 ? f2(unlatchedAt) + ' s' : 'never'}, both through at ${throughAt >= 0 ? f2(throughAt) + ' s' : 'never'}; door now ${f2(door.angle)}${door.locked ? ' (locked again behind them)' : ''}\n    ${devLog.join('\n    ')}\n    log: ${msgs.map(m => m.text).join(' | ')}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  // and into a plain wall: the pair stops with him at the wall, Sam a station behind, nobody inside it; then along it
  {
    Math.random = seeded(22);
    const W = await armStandUp(); const { game } = W;
    game.godMode = true; game.player.visibility = 0.02;
    for (let i = 0; i < 5; i++) W.step(DT);
    const [A, B, C] = game.guards; park(B, 0); park(C, 1);
    place(A, [20, 0, 11.0], Math.PI); game.teleportPlayer([20, 0, 11.8]); game.player.crouch = true; W.step(DT);   // corridor, facing its north wall (z=10) a metre off
    startGrab(game, A); for (let i = 0; i < 40; i++) W.step(DT);
    game.puppet = { goal: [20, 0, 8.0], aim: [20, 1.0, 4.0] };   // straight into the wall
    let maxDev = 0, maxPenP = 0, maxPenG = 0;
    for (let f = 0; f < 60 * 3; f++) { W.step(DT); const p = game.player.char.pos, gp = A.char.pos; maxDev = Math.max(maxDev, Math.abs(v3.distXZ(p, gp) - HOLD.dist)); maxPenP = Math.max(maxPenP, penetration(game.col, p, playerMod.playerBodyR(game, p), 0.2, 1.7)); maxPenG = Math.max(maxPenG, penetration(game.col, gp, 0.3, 0.2, 1.5)); }
    const stopZ = A.char.pos[2];
    game.puppet = { goal: [26, 0, 10.4], aim: [20, 1.0, 4.0] };   // now sideways along it, still facing it
    let slid = 0; const x0 = A.char.pos[0];
    for (let f = 0; f < 60 * 3; f++) { W.step(DT); const p = game.player.char.pos, gp = A.char.pos; maxDev = Math.max(maxDev, Math.abs(v3.distXZ(p, gp) - HOLD.dist)); maxPenP = Math.max(maxPenP, penetration(game.col, p, playerMod.playerBodyR(game, p), 0.2, 1.7)); maxPenG = Math.max(maxPenG, penetration(game.col, gp, 0.3, 0.2, 1.5)); }
    slid = A.char.pos[0] - x0; game.puppet = null;
    const fails: string[] = [];
    if (stopZ < 10.29 || stopZ > 10.45) fails.push(`the held man stopped at z ${f2(stopZ)} against the z=10 wall (want his 0.3 m circle resting on it: ≈ 10.3–10.4)`);
    if (maxDev > 0.03) fails.push(`offset strayed ${f3(maxDev)} m pressed to the wall`);
    if (maxPenP > 0.03 || maxPenG > 0.03) fails.push(`penetration Sam ${f3(maxPenP)} / him ${f3(maxPenG)} m`);
    if (slid < 1.5) fails.push(`slid only ${f2(slid)} m along the wall in 3 s`);
    if (!game.player.holding) fails.push('hold ended');
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — walked into the corridor's north wall: he stops with his circle on it (z ${f2(stopZ)}), the pair rigid (dev ≤ ${f3(maxDev)} m), nobody inside it (pen ${f3(maxPenP)} / ${f3(maxPenG)}); then ${f2(slid)} m sideways along it in 3 s${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  console.log(`=== grab-walk: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (3) the radio check: a held man cannot answer — asked for by name while you hold him, somebody sent round his route; he perceives nothing meanwhile; let go, he answers again
export async function grabRollcall() {
  console.log('\n=== grab-rollcall: a man held through a radio check is silent on the net (asked for, a checker sent) and deaf-blind in the arm; let go, he is alert on Sam and back on the net ===');
  let allPass = true;
  Math.random = seeded(201);
  const W = await armStandUp(); const { game } = W;
  game.godMode = true; game.player.visibility = 0.02;
  const { lines, msgs } = tap(game);
  for (let i = 0; i < 60 * 5; i++) W.step(DT);
  const victim = game.guards[2];   // lobby_break (Reyes)
  place(victim, [33.2, 0, 6.5], Math.PI); victim.wp = 1;   // in the storage room, its door shut, facing the back wall: nobody's route looks in there (out on the floor a colleague walking past would SEE the hold now — the human shield's standoff — and no radio check runs over that)
  game.teleportPlayer([33.2, 0, 7.3]); game.player.crouch = true; W.step(DT); game.player.visibility = 0.02;
  startGrab(game, victim);
  for (let i = 0; i < 40; i++) W.step(DT);
  const tGrab = game.time; const others = game.guards.filter((x: any) => x !== victim);
  const heard0 = victim.heardUpTo, aw0 = victim.awareness, state0 = victim.state, fix0 = victim.lastKnown ? v3.copy(victim.lastKnown) : null;
  let tCall = -1, tAsk = -1, tSend = -1; let sawEver = false, awMax = aw0, heardMax = heard0; let checker: any = null;
  // small noises right on top of him while he is held (a footfall, a creak: the room's, not the floor's — nobody else is near enough) — none of it may reach HIM
  let noiseAt = tGrab + 20;
  for (let f = 0; f < 60 * 200; f++) {
    if (game.time >= noiseAt) { noiseAt += 25; game.events.push({ kind: 'step', pos: v3.copy(victim.char.pos), time: game.time, loud: false, level: 0.35, who: 1 }); game.events.push({ kind: 'door', pos: v3.copy(victim.char.pos), time: game.time, loud: false, level: 0.3, who: 1 }); }
    W.step(DT); const t = game.time;
    sawEver ||= victim.sawPlayerThisFrame; awMax = Math.max(awMax, victim.awareness); heardMax = Math.max(heardMax, victim.heardUpTo);
    if (tCall < 0) { const l = lines.find(x => /sound off/.test(x.text) && x.t > tGrab); if (l) tCall = l.t; }
    if (tCall >= 0 && tAsk < 0) { const l = lines.find(x => x.t > tCall && x.text.includes(`${victim.callsign}?`)); if (l) tAsk = l.t; }
    if (tCall >= 0 && tSend < 0) { checker = others.find((x: any) => x.task?.kind === 'checkOn' && x.task.who === victim) ?? null; if (checker) tSend = t; }
    if (tSend >= 0 && t > tSend + 2) break;
    if (!game.player.holding) break;
  }
  const answers = msgs.filter(m => /^radio: …/.test(m.text) && m.t > tGrab).map(m => m.text).join(' / ');
  const hisLines = lines.filter(l => l.who === victim.callsign && l.t > tGrab);
  const fails: string[] = [];
  if (!game.player.holding || !victim.held) fails.push(`the hold did not last (holding ${!!game.player.holding}, held ${!!victim.held}, victim ${victim.state})`);
  if (!offNet(victim)) fails.push('offNet(victim) false while held');
  if (tCall < 0) fails.push('no radio check was called in 200 s');
  if (tAsk < 0 || Math.abs(tAsk - tCall - 4) > 0.8) fails.push(`asked for at ${tAsk < 0 ? 'never' : '+' + f2(tAsk - tCall) + ' s'} (want ≈ +4 s after the call)`);
  if (tSend < 0 || Math.abs(tSend - tCall - 8) > 0.8) fails.push(`checker sent at ${tSend < 0 ? 'never' : '+' + f2(tSend - tCall) + ' s'} (want ≈ +8 s)`);
  if (answers.includes(victim.callsign)) fails.push(`he answered the count-in from inside the arm: ${answers}`);
  if (hisLines.length) fails.push(`he spoke while held: ${hisLines.map(l => l.text).join(' | ')}`);
  if (sawEver) fails.push('sawPlayerThisFrame went true while held');
  if (heardMax !== heard0) fails.push(`his hearing ran while held (heardUpTo ${heard0} → ${heardMax})`);
  if (awMax > aw0 + 1e-6 && awMax > 0.46) fails.push(`his awareness rose while held: ${f2(aw0)} → ${f2(awMax)} (only the net's call-out floor of 0.45 may touch it)`);
  if (victim.state !== state0) fails.push(`his state changed while held: ${state0} → ${victim.state}`);
  console.log(`- staging: grabbed ${victim.callsign} at ${f2(tGrab)} s in the storage room; radio check ${tCall >= 0 ? '+' + f2(tCall - tGrab) + ' s' : 'never'} · asked for ${tAsk >= 0 ? '+' + f2(tAsk - tCall) + ' s after it' : 'never'} · ${checker ? checker.callsign : 'nobody'} sent ${tSend >= 0 ? '+' + f2(tSend - tCall) + ' s after it' : 'never'}\n    count-in: ${answers || '—'}\n    while held: saw Sam ${sawEver}, heardUpTo ${heard0}→${heardMax}, awareness ${f2(aw0)}→${f2(awMax)}, state ${state0}→${victim.state}, fix ${fix0 ? posS(fix0) : 'none'}→${victim.lastKnown ? posS(victim.lastKnown) : 'none'}, his lines: ${hisLines.length}; witness: ${witnessSummary(game, victim) || '—'}`);
  // ---- E does not let him go (nothing does): it draws the pistol to his head — still held, the gun variant now
  W.cursorAt(v3.add(victim.char.pos, [0, 1.6, 0])); W.step(DT);
  const hv = game.hover?.kind;
  tapKey(W, 'KeyE');
  for (let i = 0; i < 30; i++) W.step(DT);
  if (!game.player.holding || game.player.holding.phase !== 'held' || !victim.held || game.player.holding.variant !== 'gun' || game.player.holstered) fails.push(`E on his row: hold ${game.player.holding?.phase ?? 'gone'} / ${game.player.holding?.variant}, holstered ${game.player.holstered} (want still held — drawn on him, the gun variant; there is no letting him go)`);
  // the one way a held man still comes free with Sam alive enough to watch: Sam is shot down (combat.ts hitPlayer → freeHeld('shot') → the same hand-back)
  game.freeHeld('shot');
  let relPhaseSeen = true, tFree = -1; const p0 = v3.copy(victim.char.pos);
  for (let i = 0; i < 90; i++) { W.step(DT); if (tFree < 0 && !victim.held) tFree = game.time; }
  const stumble = v3.distXZ(victim.char.pos, p0);
  const contact = lines.find(l => l.who === victim.callsign && /CONTACT/.test(l.text));
  if (hv !== 'held') fails.push(`hover while holding is ${hv} (want the held row)`);
  if (!relPhaseSeen) fails.push('unreachable');
  if (tFree < 0) fails.push('he was never freed');
  if (game.player.holding) fails.push('Player.holding still set 1.5 s after freeHeld');
  if (victim.held || game.guards.some((x: any) => x.held)) fails.push('a Guard.held still set');
  if (game.player.char.anim.holdPose || victim.char.anim.heldPose) fails.push(`animator flags left: holdPose ${!!game.player.char.anim.holdPose} heldPose ${!!victim.char.anim.heldPose}`);
  if (victim.state !== 'alert') fails.push(`freed man is ${victim.state} (want alert)`);
  if (victim.awareness < 0.99) fails.push(`awareness ${f2(victim.awareness)}`);
  if (!victim.lastKnown || v3.distXZ(victim.lastKnown, game.player.char.pos) > 0.6) fails.push(`his fix ${victim.lastKnown ? posS(victim.lastKnown) : 'none'} is not on Sam ${posS(game.player.char.pos)}`);
  if (victim.witness.wasHeld !== 1) fails.push(`witness.wasHeld ${victim.witness.wasHeld} (want 1)`);
  if (victim.witness.alertedBy !== 'sight') fails.push(`witness.alertedBy ${victim.witness.alertedBy} (want sight: he knows exactly who)`);
  if (Dlg.knowledgeOf(game, victim) !== 'saw') fails.push(`dialogue knowledge ${Dlg.knowledgeOf(game, victim)} (want saw)`);
  if (victim.talk.heldSince !== -1) fails.push(`talk.heldSince ${victim.talk.heldSince} after release (want −1)`);
  if (stumble > 0.3) fails.push(`he moved ${f2(stumble)} m on being freed (freeHeld hands him back where he stands — there is no shove any more)`);
  if (!contact) fails.push(`no CONTACT bark from him after release (his lines: ${lines.filter(l => l.who === victim.callsign && l.t > tGrab).map(l => l.text).join(' | ') || 'none'})`);
  if (offNet(victim)) fails.push('still offNet after release');
  // the checker's errand: he is back on the net, so the walk round his route is dropped (checkOnTick) within a frame or two of the checker's next suspicious tick
  for (let i = 0; i < 30; i++) W.step(DT);
  const stillChecking = others.some((x: any) => x.task?.kind === 'checkOn' && x.task.who === victim && x.state === 'suspicious');
  if (stillChecking) fails.push('somebody is still walking his route half a second after he came back on the net');
  if (victim.missedAt !== -1) fails.push(`missedAt ${victim.missedAt} after release (want −1: nothing about him is missing)`);
  const ok = fails.length === 0; allPass &&= ok;
  console.log(`- ${ok ? 'PASS' : 'FAIL'} — held through the check, then E (draws: still held), then torn loose: release phase ${relPhaseSeen}, freed at ${tFree >= 0 ? f2(tFree) + ' s' : 'never'}, stumbled ${f2(stumble)} m; now ${victim.state} aw ${f2(victim.awareness)} fix ${victim.lastKnown ? posS(victim.lastKnown) : '—'} (Sam ${posS(game.player.char.pos)}); wasHeld ${victim.witness.wasHeld}, alertedBy ${victim.witness.alertedBy}, K ${Dlg.knowledgeOf(game, victim)}; bark: "${contact?.text ?? '—'}"; checker still on it: ${stillChecking}\n    witness now: ${witnessSummary(game, victim)}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  console.log(`=== grab-rollcall: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (4) questions: taps on his 'question him' row are EXCHANGES — Sam's line over Sam, the man's answer through Game.interrogate when it ends — on the pacing rule: a
//                                                                    tap while Sam speaks does nothing, one inside the answer's first 1.2 s only cuts it short, one once it is gone or past that cuts in with the next question (dialogue-exchange has the engine's side)
export async function grabTalk() {
  console.log('\n=== grab-talk: Space taps on the held man\'s question row — each an exchange (Sam\'s line, then his answer a beat later), one stage per answer; the pacing rule (nothing while Sam speaks · the answer\'s first 1.2 s: cut short only · gone or later: the next) and the row\'s wording through it ===');
  let allPass = true;
  const S = await stage(41); const { W, game, A, msgs } = S;
  const at = grabViaHold(S); const heldOk = untilHeld(S);
  W.step(DT);
  const fails: string[] = [];
  if (at < 0 || !heldOk) fails.push(`staging: grab at frame ${at}, held ${heldOk}`);
  const rowQ = () => game.nearby.find((r: any) => r.verb === 'question')?.line2 ?? '—';   // the question row's wording (the highlighted one: a plain tap does it)
  const rec: string[] = []; const stageAt = (label: string) => rec.push(`${label}: stage ${A.talk.stage} presses ${A.talk.presses} · Sam ${game.player.bubble && game.time - game.player.bubble.t < game.player.bubble.dur ? `"${game.player.bubble.text.slice(0, 30)}…"` : '—'} · him ${A.bubble && game.time - A.bubble.t < A.bubble.dur ? `"${A.bubble.text.slice(0, 30)}…" ${f2(A.bubble.dur - (game.time - A.bubble.t))} s left` : '—'} · row "${rowQ()}"`);
  const press = (label: string, thenSecs: number) => { if (game.hover?.verb !== 'question') fails.push(`${label}: the highlighted row is ${game.hover?.kind}/${game.hover?.verb} (want the question — a plain tap talks)`); tapKey(W, IK); W.step(DT); stageAt(label); for (let i = 0; i < Math.round(thenSecs / DT) - 1; i++) W.step(DT); };
  /** step until his answer to the exchange in flight has landed (talk.stage reaches `st`), at most 3 s; returns the seconds it took from now */
  const untilStage = (st: number) => { const t0 = game.time; for (let i = 0; i < 180 && A.talk.stage < st; i++) W.step(DT); return game.time - t0; };
  stageAt('before');
  const row0 = rowQ(); const rows0 = game.nearby.map((r: any) => r.verb).join('/');
  press('tap 1 (fresh: Sam\'s opener goes up)', 0.4);
  const asked1 = !!game.player.holding?.ask, sam1 = game.player.bubble?.text ?? '', s1a = A.talk.stage, rowAsking = rowQ();
  press('tap 2 at +0.4 s (Sam mid-line: nothing)', 0.05);
  const sam2 = game.player.bubble?.text ?? '', s2 = A.talk.stage;
  const beat1 = untilStage(1) + 0.45; const s1 = A.talk.stage;   // his first answer lands on Sam's beat
  const rowFresh = (() => { W.step(DT); return rowQ(); })();
  press('tap 3 the frame after the answer began (cut short only)', 0.5);
  const s3 = A.talk.stage; const cut = A.bubble ? A.bubble.dur : -1; const asked3 = !!game.player.holding?.ask;
  const rowGone = rowQ();   // the cut line is over by now (+0.3 s), inside the old 1.2 s: the row must not offer to cut it — it offers the next question
  press('tap 4 at +0.55 s of the answer (the cut line gone: the second exchange opens)', 0.05);
  const asked4 = !!game.player.holding?.ask; untilStage(2); const s4 = A.talk.stage;
  // let the second answer run 1.4 s uncut (past the gap, mid-line), then tap: CUT IN — his line ends, Sam's third goes up, the third answer (the useful thing or the refusal) a beat later
  for (let i = 0; i < 84; i++) W.step(DT);
  const rowCutIn = rowQ(); const live5 = !!(A.bubble && game.time - A.bubble.t < A.bubble.dur - 0.3);
  press('tap 5 at +1.4 s of answer 2 (cut in → exchange 3)', DT);   // (measured on the tap's own frame)
  const left5 = A.bubble ? A.bubble.dur - (game.time - A.bubble.t) : NaN; const asked5 = !!game.player.holding?.ask;   // (the tap was read inside the step, after game.time advanced: his line was cut to exactly that 'now')
  untilStage(3); const s5 = A.talk.stage; const gave = A.talk.last?.kind ?? null;
  // wait this one out entirely (settled: nothing left to cut), then two more exchanges into the dry pool; the row says he is spent once he is
  for (let i = 0; i < 600 && !(game.player.holding && heldSettled(game, game.player.holding)); i++) W.step(DT);
  const rowSettled3 = rowQ();
  press('tap 6 (settled → exchange 4: nothing left, the x pool)', 0.05); untilStage(4); const s6 = A.talk.stage;
  for (let i = 0; i < 600 && !(game.player.holding && heldSettled(game, game.player.holding)); i++) W.step(DT);
  const rowSpent = rowQ();
  press('tap 7 (→ 5: it keeps counting)', 0.05); untilStage(5); const s7 = A.talk.stage; stageAt('after the fifth answer');
  if (rows0 !== 'question/choke') fails.push(`held rows: ${rows0} (want question/choke)`);
  if (!/^Space {2}“make him talk”$/.test(row0)) fails.push(`first row question: "${row0}" (want Space  “make him talk”)`);
  if (!asked1 || !sam1 || s1a !== 0) fails.push(`tap 1: ask ${asked1}, Sam "${sam1}", stage ${s1a} (want Sam's line up and the stage still 0 — he answers a beat later)`);
  if (!/asking/.test(rowAsking)) fails.push(`row while Sam speaks: "${rowAsking}" (want "you're asking — he answers in a beat")`);
  if (sam2 !== sam1 || s2 !== 0) fails.push(`tap 2 (Sam mid-line) changed something: Sam "${sam2}", stage ${s2}`);
  if (s1 !== 1) fails.push(`no first answer within 3 s (stage ${s1})`); if (beat1 < 0.9 - 1e-6 || beat1 > 1.4 + DT + 1e-6) fails.push(`the first answer came ${f2(beat1)} s after the tap (want Sam's beat, 0.9‥1.4 s)`);
  if (!/answering/.test(rowFresh)) fails.push(`row just after his answer began: "${rowFresh}" (want "he's answering — tap to cut it short")`);
  if (s3 !== 1 || asked3) fails.push(`tap 3 (the answer just begun): stage ${s3}, asked ${asked3} (want a cut only)`); if (!(cut > 0 && cut < 1.0)) fails.push(`tap 3 did not cut the line short (dur now ${f2(cut)})`);
  if (/answering|cut/.test(rowGone) || !/lean on him|ask what he knows/.test(rowGone)) fails.push(`row once the cut line is gone (+0.5 s, inside the old gap): "${rowGone}" (want the next question, nothing about cutting a line nobody can see)`);
  if (!asked4 || s4 !== 2) fails.push(`tap 4 (0.55 s in, the line gone): asked ${asked4}, then stage ${s4} (want the second exchange → 2)`);
  if (!live5) fails.push('staging: answer 2 was not mid-line at +1.4 s');
  if (!/lean on him|ask what he knows/.test(rowCutIn) || !/cuts in/.test(rowCutIn)) fails.push(`row mid-answer past the gap: "${rowCutIn}" (want the next verb + "— cuts in")`);
  if (!asked5 || !(Math.abs(left5) < 1e-6) || s5 !== 3) fails.push(`tap 5 (cut-in): asked ${asked5}, his line left ${f3(left5)} s, then stage ${s5} (want Sam up at once, the answer over that frame, → 3)`);
  if (Dlg.temperamentOf(A) === 'steady' && !gave) fails.push(`the third answer gave nothing (steady man, calm floor: want his useful thing)`);
  if (/cuts in|answering|asking/.test(rowSettled3) || !/he's got nothing left|said his piece/.test(rowSettled3)) fails.push(`row settled after the third answer: "${rowSettled3}" (want the spent verb, nothing to cut)`);
  if (s6 !== 4) fails.push(`tap 6: stage ${s6} (want 4: the exhausted pool)`);
  if (!/said his piece/.test(rowSpent)) fails.push(`row once dry: "${rowSpent}" (want "he's said his piece — ask anyway")`);
  if (s7 !== 5) fails.push(`tap 7: stage ${s7} (want 5: it keeps counting)`);
  if (A.talk.heldSince < 0 || Math.abs(A.talk.heldSince - (game.player.holding?.since ?? -9)) > 0.5 + HOLD.secs.grab) fails.push(`talk.heldSince ${f2(A.talk.heldSince)} vs hold since ${f2(game.player.holding?.since ?? -1)}`);
  const you = msgs.filter(m => /^you ▸/.test(m.text)).map(m => m.text.replace('you ▸ ', ''));
  const samTexts = new Set(Dlg.SAM_LINES.map((l: any) => l.text));
  if (you.length !== 5 || you.some(t => !samTexts.has(t)) || new Set(you).size !== 5) fails.push(`Sam's logged lines: ${you.join(' | ')} (want five, each a SAM_LINES text, none twice)`);
  if (!game.player.holding || A.state === 'dead') fails.push('the hold ended under questioning');
  const ok = fails.length === 0; allPass &&= ok;
  console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${A.callsign} (${Dlg.temperamentOf(A)}, K ${Dlg.knowledgeOf(game, A)}): stages ${[s1, s3, s4, s5, s6, s7].join('/')} · first answer +${f2(beat1)} s · gave ${gave ?? 'nothing'}\n    ${rec.join('\n    ')}\n    Sam's side in the log: ${you.join(' | ') || '—'}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  console.log(`=== grab-talk: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (5) the choke: its row chosen and tapped, HOLD.secs.chokeHold to commit and HOLD.secs.choke of it → 'choked', breathing, the torch dies, the ragdoll sleeps; found later as out cold. A second tap sooner and he breathes.
export async function grabChoke() {
  console.log('\n=== grab-choke: the choke row tapped — the arm closes at 0.35 s, he goes at +2.6 s out cold and breathing, torch dark, ragdoll asleep; stopped sooner (a second tap) he lives ===');
  let allPass = true;
  // ---- (a) all the way
  {
    const S = await stage(51); const { W, game, A, B, lines, msgs } = S;
    grabViaHold(S); untilHeld(S); W.step(DT); W.step(DT);
    const torchLit0 = A.flashlight.enabled;
    let chokeAt = -1, deadAt = -1; const t0 = game.time;
    const chosen = game.selectVerb('held', 'choke'); W.input.pressed.add(IK);   // the row under the question, tapped (its frame is the first stepped below)
    for (let i = 0; i < 60 * 4 && deadAt < 0; i++) { W.step(DT); if (chokeAt < 0 && game.player.holding?.phase === 'choke') chokeAt = game.time - t0; if (deadAt < 0 && A.state === 'dead') deadAt = game.time - t0; }
    const heldStill = !!game.player.holding;
    for (let i = 0; i < 30; i++) W.step(DT);
    let darkAt = -1, sleptAt = -1;
    for (let i = 0; i < 60 * 8; i++) { W.step(DT); if (darkAt < 0 && !A.flashlight.enabled) darkAt = game.time - t0 - deadAt; if (sleptAt < 0 && A.char.ragdoll?.sleeping) sleptAt = game.time - t0 - deadAt; }
    const st = game.mission.stats;
    const fails: string[] = [];
    if (!chosen) fails.push('staging: no choke row to choose');
    if (chokeAt < 0 || Math.abs(chokeAt - HOLD.secs.chokeHold) > 0.05) fails.push(`choke began at +${f2(chokeAt)} s of the tap (want ${HOLD.secs.chokeHold})`);
    if (deadAt < 0 || Math.abs(deadAt - (HOLD.secs.chokeHold + HOLD.secs.choke)) > 0.06) fails.push(`went down at +${f2(deadAt)} s (want ${f2(HOLD.secs.chokeHold + HOLD.secs.choke)})`);
    if (A.downKind !== 'choked' || A.downBy !== 'player' || !isBreathing(A)) fails.push(`A ${A.state}/${A.downKind}/${A.downBy} breathing ${isBreathing(A)} (want dead/choked/player, breathing)`);
    if (heldStill || A.held) fails.push('hold not cleared on the frame he went');
    if (game.player.takedown || game.player.holding) fails.push('something else started after he went');
    if (!A.char.ragdoll) fails.push('no ragdoll'); if (sleptAt < 0) fails.push('ragdoll never slept in 8 s');
    if (torchLit0) fails.push('his torch was still lit in the hold — it is meant to be knocked out of his hand at the grab and die on the floor'); if (A.dropped?.kind !== 'torch') fails.push(`dropped ${A.dropped?.kind ?? 'nothing'} (want the torch)`); if (darkAt < 0 || darkAt > 1.2) fails.push(`torch dark at ${darkAt < 0 ? 'never' : '+' + f2(darkAt) + ' s'} after he went (want < 1 s)`);
    if (st.knockouts !== 1 || st.kills !== 0) fails.push(`stats ko ${st.knockouts} kills ${st.kills} (want 1 / 0)`);
    if (!msgs.some(m => /choked out/.test(m.text))) fails.push(`no 'choked out' log line: ${msgs.map(m => m.text).join(' | ')}`);
    if (game.player.char.anim.holdPose) fails.push('Sam\'s holdPose still set');
    const hips = A.char.bones.hips; if (!hips || hips[1] > 0.35) fails.push(`his hips at y ${hips ? f2(hips[1]) : '?'} (want on the floor)`);
    const dSam = v3.distXZ(A.char.pos, game.player.char.pos);
    // somebody finds him: B walked in from the east
    game.teleportPlayer([37.5, 0, 26.5]); game.player.crouch = false;
    B.hold = false; B.heardUpTo = game.eventSeq; place(B, [A.char.pos[0] + 5, 0, 11.2], -Math.PI / 2); B.state = 'patrol'; B.awareness = 0; B.wp = 3;
    let foundAt = -1; for (let f = 0; f < 60 * 30; f++) { W.step(DT); if (foundAt < 0 && A.found) foundAt = game.time; }
    const call = lines.find(l => l.radio && l.who === B.callsign && /radioing it in/.test(l.text));
    if (foundAt < 0) fails.push('B never found him'); if (!call || !/out cold but breathing/.test(call.text)) fails.push(`finder's call "${call?.text ?? 'none'}" (want out cold but breathing)`);
    if (!B.witness.sawBody?.breathing) fails.push(`B.witness.sawBody ${JSON.stringify(B.witness.sawBody)}`);
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — (a) choke through: began +${f2(chokeAt)} s, down +${f2(deadAt)} s as ${A.downKind}/${A.downBy} breathing ${isBreathing(A)}; torch dropped ${A.dropped?.kind ?? '—'} dark +${darkAt >= 0 ? f2(darkAt) : '—'} s; ragdoll asleep +${sleptAt >= 0 ? f2(sleptAt) : '—'} s, hips y ${hips ? f2(hips[1]) : '?'}, ${f2(dSam)} m from Sam; card ko ${st.knockouts} kills ${st.kills}; found ${foundAt >= 0 ? 'at ' + f2(foundAt) + ' s' : 'never'}: "${call?.text ?? '—'}"${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  // ---- (b) stopped at 1.5 s (a second tap): back to held, alive; then a full one still works
  {
    const S = await stage(52); const { W, game, A, msgs } = S;
    grabViaHold(S); untilHeld(S); W.step(DT); W.step(DT);
    let phaseDuring: string | undefined; endThenStop(W, 1.5, () => { if (game.player.holding?.phase === 'choke') phaseDuring = 'choke'; });
    W.step(DT); W.step(DT);
    const phaseAfter = game.player.holding?.phase;
    const fails: string[] = [];
    if (phaseDuring !== 'choke') fails.push(`1.5 s after choosing the choke: phase never reached choke`);
    if (phaseAfter !== 'held' || A.state === 'dead') fails.push(`after the stopping tap: phase ${phaseAfter}, A ${A.state} (want held, alive)`);
    if (!msgs.some(m => /eased off/.test(m.text))) fails.push('no eased-off line');
    if (game.nearby.map((r: any) => r.verb).join('/') !== 'question/choke') fails.push(`rows after easing off: ${game.nearby.map((r: any) => r.verb).join('/')} (want question/choke back)`);
    for (let i = 0; i < 40; i++) W.step(DT);
    const ck0 = (A.char.anim as any).heCk?.x ?? -1;
    endFor(W, HOLD.secs.chokeHold + HOLD.secs.choke + 0.1);
    if (A.state !== 'dead' || A.downKind !== 'choked') fails.push(`the second, full choke: A ${A.state}/${A.downKind}`);
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — (b) stopped at 1.5 s: phase ${phaseDuring} → ${phaseAfter}, alive; the animator's choke amount had come back to ${ck0 >= 0 ? f2(ck0) : '?'} before the second; the full one then: ${A.state}/${A.downKind}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  console.log(`=== grab-choke: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (6) coming free: there is no release verb (E draws on him instead) — torn loose when Sam goes down (freeHeld 'shot') he is alert on Sam within a frame, wasHeld, nothing of the hold left anywhere; and no re-grab of an alert man
export async function grabRelease() {
  console.log('\n=== grab-release: E on his row draws (still held); freeHeld — instant witness (alert on Sam, wasHeld 1), no stumble, nothing residual; the alert man cannot be grabbed again ===');
  let allPass = true;
  const S = await stage(61); const { W, game, A, lines } = S; const { cam, canvas } = W;
  grabViaHold(S); untilHeld(S); W.step(DT);
  for (let i = 0; i < 60; i++) W.step(DT);
  const p0 = v3.copy(A.char.pos), yaw0 = A.char.bodyYaw;
  tapKey(W, 'KeyE'); for (let i = 0; i < 20; i++) W.step(DT);
  if (!A.held || game.player.holding?.phase !== 'held' || game.player.holding.variant !== 'gun' || A.held.variant !== 'gun') { console.log(`- FAIL — E on the held row: hold now ${game.player.holding?.phase ?? 'gone'} / ${game.player.holding?.variant} (want still held, drawn on him — there is no release verb)`); allPass = false; }
  else console.log('- pass — E on the held row draws the pistol to his head: still held 0.33 s later, the gun variant');
  game.freeHeld('shot');   // the hand-back path that remains (Sam shot down while holding): instant witness, alert on Sam, wasHeld — asserted below exactly as the shove used to be
  let freeFrame = -1, alertFrame = -1; const trace: string[] = [];
  for (let i = 0; i < 120; i++) {
    W.step(DT);
    if (freeFrame < 0 && !A.held) freeFrame = i;
    if (alertFrame < 0 && A.state === 'alert') alertFrame = i;
    if (i % 10 === 0 || i === freeFrame || i === alertFrame) trace.push(`f${i} ${game.player.holding?.phase ?? 'free'} him ${posS(A.char.pos)} ${A.state} aw ${f2(A.awareness)} reactT ${f2(A.reactT)} spd ${f2(A.speed)}`);
  }
  const stumble = v3.distXZ(A.char.pos, p0); const fwd: Vec3 = [Math.sin(yaw0), 0, Math.cos(yaw0)]; const along = (A.char.pos[0] - p0[0]) * fwd[0] + (A.char.pos[2] - p0[2]) * fwd[2];
  const fails: string[] = [];
  if (freeFrame !== 0) fails.push(`freed at frame ${freeFrame} (freeHeld('shot') hands him back at once — want frame 0)`);
  if (alertFrame < 0 || alertFrame - freeFrame > 1) fails.push(`alert at frame ${alertFrame}, freed at ${freeFrame} (want within a frame)`);
  if (stumble > 0.3) fails.push(`moved ${f2(stumble)} m (${f2(along)} m forward) on being freed — freeHeld hands him back where he stands, no shove`); void along;
  if (penetration(game.col, A.char.pos, 0.3, 0.2, 1.5) > 0.03) fails.push('he ended inside static geometry');
  if (game.player.holding || A.held || game.player.char.anim.holdPose || A.char.anim.heldPose) fails.push(`residue: holding ${!!game.player.holding} held ${!!A.held} holdPose ${!!game.player.char.anim.holdPose} heldPose ${!!A.char.anim.heldPose}`);
  if (A.witness.wasHeld !== 1) fails.push(`wasHeld ${A.witness.wasHeld}`);
  if (A.awareness < 0.99 || !A.lastKnown || v3.distXZ(A.lastKnown, game.player.char.pos) > 0.6) fails.push(`awareness ${f2(A.awareness)} fix ${A.lastKnown ? posS(A.lastKnown) : 'none'} vs Sam ${posS(game.player.char.pos)}`);
  if (!lines.some((l: any) => l.who === A.callsign && /CONTACT/.test(l.text))) fails.push('no CONTACT! from him');
  if (A.talk.heldSince !== -1) fails.push(`talk.heldSince ${A.talk.heldSince}`);
  if (game.player.char.anim.hideHeldItem) { for (let i = 0; i < 30; i++) W.step(DT); if (game.player.char.anim.hideHeldItem) fails.push('Sam\'s pistol still hidden 2.5 s after letting go'); }
  // no re-grab: behind him again, the row refuses
  const gp = A.char.pos; const back: Vec3 = [gp[0] - Math.sin(A.char.bodyYaw) * 0.8, 0, gp[2] - Math.cos(A.char.bodyYaw) * 0.8];
  game.teleportPlayer(back); W.step(DT);
  buildInteractables(game, cam, 0, 0, canvas.clientWidth, canvas.clientHeight);
  const it = game.interactables.find((x: any) => x.kind === 'guard' && x.guard === A);
  startGrab(game, A);
  if (it?.inReach) fails.push(`the alert man's row still offers "${it.line2}"`);
  if (game.player.holding || A.held) fails.push('startGrab took an alert man');
  const ok = fails.length === 0; allPass &&= ok;
  console.log(`- ${ok ? 'PASS' : 'FAIL'} — freed at frame ${freeFrame}, alert at frame ${alertFrame}; stumbled ${f2(stumble)} m (${f2(along)} forward); now ${A.state} aw ${f2(A.awareness)}, wasHeld ${A.witness.wasHeld}, alertedBy ${A.witness.alertedBy}; behind him again: row "${it?.line2 ?? 'none'}" inReach ${it?.inReach}\n    ${trace.join('\n    ')}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  console.log(`=== grab-release: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (7) everything else that ends it: Sam shot down in the hold, a teleport, a restart, a guard reset, the man killed in the arm by somebody else
export async function grabAuto() {
  console.log('\n=== grab-auto: the hold ends cleanly when Sam goes down, teleports, restarts, the guards reset, or the man dies in the arm ===');
  let allPass = true; const verdict = (ok: boolean, label: string, detail: string) => { allPass &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  const residue = (game: any) => { const bad: string[] = []; if (game.player.holding) bad.push('holding'); if (game.player.char.anim.holdPose) bad.push('holdPose'); for (const x of game.guards) { if (x.held) bad.push(`${x.callsign}.held`); if (x.char.anim.heldPose) bad.push(`${x.callsign}.heldPose`); if (x.state !== 'dead' && x.talk.heldSince !== -1) bad.push(`${x.callsign}.heldSince ${x.talk.heldSince}`); } return bad; };
  // (a) shot down (god mode off, a round's worth through hitPlayer)
  {
    const S = await stage(71); const { W, game, A } = S;
    grabViaHold(S); untilHeld(S); for (let i = 0; i < 30; i++) W.step(DT);
    game.godMode = false; hitPlayer(game, [1, 0, 0]);
    const downNow = game.player.down, freedNow = !A.held && !game.player.holding;
    for (let i = 0; i < 5; i++) W.step(DT);
    const fails: string[] = []; if (!downNow) fails.push('Sam not down'); if (!freedNow) fails.push('not freed inside hitPlayer'); const r = residue(game); if (r.length) fails.push('residue: ' + r.join(', ')); if (A.state !== 'alert') fails.push(`freed man ${A.state} (want alert — covering the body)`); if (A.witness.wasHeld !== 1) fails.push(`wasHeld ${A.witness.wasHeld}`);
    verdict(fails.length === 0, '(a) Sam shot down with him in the arm: freed inside hitPlayer, alert, nothing left of the hold', `down ${downNow}, freed ${freedNow}, A ${A.state} aw ${f2(A.awareness)}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  // (b) teleport mid-hold (cancelActions): he stays, free and alert; Sam elsewhere
  {
    const S = await stage(72); const { W, game, A } = S;
    grabViaHold(S); untilHeld(S); for (let i = 0; i < 30; i++) W.step(DT);
    const gp0 = v3.copy(A.char.pos);
    game.teleportPlayer([30, 0, 20]); for (let i = 0; i < 3; i++) W.step(DT);
    const fails: string[] = []; const r = residue(game); if (r.length) fails.push('residue: ' + r.join(', ')); if (v3.distXZ(A.char.pos, gp0) > 0.3) fails.push(`he moved ${f2(v3.distXZ(A.char.pos, gp0))} m with the teleport`); if (A.state !== 'alert') fails.push(`A ${A.state}`);
    verdict(fails.length === 0, '(b) teleport mid-hold: the man stays where he was, freed and alert', `A ${A.state} at ${posS(A.char.pos)} (was ${posS(gp0)}), Sam ${posS(game.player.char.pos)}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  // (c) restartEncounter mid-hold, and mid-choke
  {
    const S = await stage(73); const { W, game } = S;
    grabViaHold(S); untilHeld(S); W.step(DT); endFor(W, 1.0);   // a second into a choke (0.35 s to commit, then the arm on)
    const phase = game.player.holding?.phase;
    game.restartEncounter(); for (let i = 0; i < 3; i++) W.step(DT);
    const fails: string[] = []; if (phase !== 'choke') fails.push(`staging: phase ${phase}`); const r = residue(game); if (r.length) fails.push('residue: ' + r.join(', ')); if (game.guards.some((x: any) => x.state !== 'patrol' || x.awareness !== 0 || x.witness.wasHeld)) fails.push(`fresh guards not fresh: ${game.guards.map((x: any) => `${x.state}/${f2(x.awareness)}/held×${x.witness.wasHeld}`).join(' ')}`); if (game.player.fHeld || 'guardHold' in game.player) fails.push(`player fHeld ${game.player.fHeld}${'guardHold' in game.player ? ' · a guardHold field is back (the press-and-hold grab timer went with the Space-only grab)' : ''}`);
    verdict(fails.length === 0, '(c) restartEncounter a second into a choke: fresh player, fresh guards, nothing of it anywhere', `${fails.length ? '✗ ' + fails.join('\n    ✗ ') : 'clean'}`);
  }
  // (d) resetGuards mid-hold (the tour's start): Sam's side cleared, the new men untouched
  {
    const S = await stage(74); const { W, game } = S;
    grabViaHold(S); untilHeld(S); for (let i = 0; i < 30; i++) W.step(DT);
    game.clearAftermath(); game.resetGuards(); for (let i = 0; i < 3; i++) W.step(DT);
    const fails: string[] = []; const r = residue(game); if (r.length) fails.push('residue: ' + r.join(', '));
    let threw = ''; try { for (let i = 0; i < 60; i++) W.step(DT); } catch (e: any) { threw = String(e?.message ?? e); }
    if (threw) fails.push('stepping after the reset threw: ' + threw);
    verdict(fails.length === 0, '(d) resetGuards mid-hold: the hold is simply gone, a second of frames runs clean', `${fails.length ? '✗ ' + fails.join('\n    ✗ ') : 'clean'}`);
  }
  // (e) the man killed in the arm by somebody else's round (killGuard from outside): the hold drops, he is a corpse, Sam free
  {
    const S = await stage(75); const { W, game, A } = S; const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
    grabViaHold(S); untilHeld(S); for (let i = 0; i < 30; i++) W.step(DT);
    killGuard(game, A, [0, 0, 1], false, 'shot', 'guard');
    for (let i = 0; i < 3; i++) W.step(DT);
    const fails: string[] = []; const r = residue(game); if (r.length) fails.push('residue: ' + r.join(', ')); if (A.state !== 'dead' || A.downKind !== 'shot' || A.downBy !== 'guard') fails.push(`A ${A.state}/${A.downKind}/${A.downBy}`); if (game.mission.stats.kills !== 0) fails.push('counted on Sam\'s card');
    let threw = ''; try { for (let i = 0; i < 120; i++) W.step(DT); } catch (e: any) { threw = String(e?.message ?? e); } if (threw) fails.push('threw: ' + threw);
    verdict(fails.length === 0, "(e) the man shot in the arm by a colleague's round: the hold drops, Sam is free, the corpse is nobody's hostage", `A ${A.state}/${A.downKind}/${A.downBy}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  // (f) Q / Shift / C / LMB while holding do nothing they should not (E — the draw — is hold-draw's business)
  {
    const S = await stage(76); const { W, game, A } = S;
    grabViaHold(S); untilHeld(S); W.step(DT);
    tapKey(W, 'KeyQ'); tapKey(W, 'KeyC'); W.input.clicked |= 1; W.input.keys.add('ShiftLeft'); W.input.keys.add('KeyW');
    for (let i = 0; i < 60; i++) W.step(DT);
    W.input.keys.delete('ShiftLeft'); W.input.keys.delete('KeyW');
    const fails: string[] = [];
    if (game.player.wall) fails.push('Q put his back to a wall with a man in his arm'); if (game.player.crouch) fails.push('C crouched him'); if (game.player.sprinting) fails.push('Shift sprinted'); if (game.mission.stats.shots > 0 || game.player.takedown) fails.push('LMB fired / struck');
    if (!game.player.holding || A.state === 'dead') fails.push(`hold ended (holding ${!!game.player.holding}, A ${A.state})`);
    const sp = game.player.speedSm; if (sp > 0.86) fails.push(`pace ${f2(sp)} with Shift+W (want ≤ 0.85)`);
    verdict(fails.length === 0, '(f) Q, C, Shift, LMB with a man in the arm: no wall, no crouch, no sprint, no shot — the pair just walks', `speed ${f2(sp)}, wall ${!!game.player.wall}, crouch ${game.player.crouch}, sprint ${game.player.sprinting}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  console.log(`=== grab-auto: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (8) the controls behind a man (the maintainer's note: "the only option behind a guard should be grabbing him, not instant takedown. if you want to do that,
//                                                                    you need to holster and primary click"): the one row is the grab — a Space TAP takes him on its frame, Space HELD opens the list on it and takes him on
//                                                                    the release — in the variant the hand decides; the takedown's one input is a click with the hands empty; drawn, a click is a round; a canister up, a
//                                                                    throw; and none of it from in front or out of reach
export async function controlsBehind() {
  console.log('\n=== controls-behind: the grab row (tap: at once; hold: on letting go), arm / gun by the hand; holstered + LMB = the silent takedown; drawn + LMB = a round; nothing from in front / out of reach ===');
  let allPass = true; const verdict = (ok: boolean, label: string, detail: string) => { allPass &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  /** the shared staging with the hand set: holstered (hands empty) or the Five-seveN drawn; A rooted on the scripted hold so nothing walks off mid-check; one settled frame (hover, rows) */
  const hands = async (seed: number, drawn: boolean) => { const S = await stage(seed); const { W, game, A } = S; game.player.slot = 'pistol'; game.player.holstered = !drawn; A.hold = true; W.step(DT); return S; };
  /** step n frames collecting every GameEvent stamped this frame (kind, level, loud, who) */
  const stepCollect = (W: any, game: any, n: number, out: any[]) => { for (let i = 0; i < n; i++) { W.step(DT); for (const e of game.events) if (e.time === game.time) out.push({ kind: e.kind, level: e.level ?? 0, loud: !!e.loud, who: e.who }); } };
  const rowOf = (game: any, A: any) => game.interactables.find((x: any) => x.kind === 'guard' && x.guard === A);
  // ---- (d) pistol DRAWN + a left click behind him: that is a round (the cursor on him: it is his), loud on the whole floor — never the takedown, never a grab
  {
    const S = await hands(805, true); const { W, game, A } = S;
    W.cursorAt((A.char.bones.chest ?? v3.add(A.char.pos, [0, 1.2, 0])) as Vec3); W.step(DT);
    const ev: any[] = [];
    // (the round is due by the rule's own clock, counted in frames from the click: see dueFrames. It is watched for a second past that too: one round, never two)
    const due = dueFrames(KITANIM.clickKeep), lateAt = due + 30, shots0 = game.mission.stats.shots; let leftAt = -1;
    if (!LATE_BREAK) W.input.clicked |= 1; stepCollect(W, game, 1, ev); if (game.mission.stats.shots > shots0) leftAt = 1;
    const td0 = !!game.player.takedown, hold0 = !!game.player.holding;
    for (let f = 2; f <= lateAt + due + 60; f++) { if (LATE_BREAK && f === lateAt) W.input.clicked |= 1; stepCollect(W, game, 1, ev); if (leftAt < 0 && game.mission.stats.shots > shots0) leftAt = f; }
    const shots1 = game.mission.stats.shots - shots0;
    const loudShot = ev.some(e => e.kind === 'shot' );   // (a SHOT event from Sam — 'loud' no longer required: the suppressed pistol reports at level 0.6 / not loud by design since 08-09; the rule under test is 'drawn = a round, not a takedown')
    const fails: string[] = [];
    if (td0 || game.player.takedown) fails.push('a click with the gun out started the takedown');
    if (hold0 || game.player.holding) fails.push('a click with the gun out grabbed him');
    if (leftAt < 0 || leftAt > due) fails.push(`the round left ${leftAt < 0 ? 'never' : `${leftAt} frames after the click`} (due by frame ${due}: ${KITANIM.on ? `a click on a gun coming up is kept ${KITANIM.clickKeep} s` : 'the click\'s own frame'})`);
    if (shots1 !== 1) fails.push(`card shots ${shots1} by the end (want 1: it fired, once)`);
    if (!loudShot) fails.push('no loud \'shot\' event off the click');
    if (A.state === 'dead' && A.downKind !== 'shot') fails.push(`A went down '${A.downKind}' (a round makes 'shot')`);
    verdict(fails.length === 0, '(d) pistol drawn, LMB behind him: fires (a shot event, quiet as the pistol now is), no takedown, no grab', `shots ${shots1}, the round left ${leftAt < 0 ? 'never' : `${leftAt} frame${leftAt === 1 ? '' : 's'} after the click`} (due by ${due}), loud shot event ${loudShot}, takedown ${td0}, A ${A.state}/${A.downKind ?? '—'}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  // ---- (f) a smoke can up (the smoke slot selected, not holstered) + a click behind him: that is a throw — the row says holster for the takedown; Space would still be the arm grab (gunhold-variant (c))
  {
    const S = await hands(807, true); const { W, game, A } = S; game.player.slot = 'smoke'; W.step(DT);
    const row = rowOf(game, A); const cans0 = game.player.canisters;
    // (a click made while the can is still coming is kept and thrown as it arrives — kitanim.ts kitClick; KITANIM.throwKeep is the longest such a click may wait)
    const most = dueFrames(KITANIM.throwKeep), thrown = () => !!game.player.pendingThrow || game.player.canisters < cans0; let threwAt = -1; const lateAt = most + 2;
    if (!LATE_BREAK) W.input.clicked |= 1; W.step(DT); if (thrown()) threwAt = 1;
    const td0 = !!game.player.takedown;
    for (let f = 2; f <= 2 * most + 120; f++) { if (LATE_BREAK && f === lateAt) W.input.clicked |= 1; W.step(DT); if (threwAt < 0 && thrown()) threwAt = f; }
    const threw = threwAt >= 1 && threwAt <= most;
    const fails: string[] = [];
    if (!row?.inReach || !/^Space\s+grab him\s+·/.test(row.line2) || !/E\s+holster/.test(row.line2) || /pistol/.test(row.line2)) fails.push(`can-up row: "${row?.line2}" (want the plain grab, and E to holster for the takedown; no pistol)`);
    if (td0 || game.player.takedown || A.state === 'dead') fails.push(`the click with a can up took him down (takedown ${td0}, A ${A.state})`);
    if (!threw) fails.push(`the can left ${threwAt < 0 ? 'never' : `${threwAt} frames after the click`} (want inside the ${most} a kept click may wait)`);
    if (game.player.holding) fails.push('grabbed');
    verdict(fails.length === 0, '(f) a canister up, LMB behind him: a throw, not the takedown (the row says holster first)', `row "${row?.line2}" · threw ${threw}, ${threwAt < 0 ? 'never' : `${threwAt} frame${threwAt === 1 ? '' : 's'} after the click`} (inside ${most}; cans ${cans0} → ${game.player.canisters}) · takedown ${td0} · A ${A.state}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  console.log(`=== controls-behind: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}
