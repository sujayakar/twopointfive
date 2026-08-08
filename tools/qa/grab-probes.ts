// The 'grab' probe family (run through probes.ts: `bun run tools/qa/probes.ts grab | grab-gate | grab-walk | grab-rollcall | grab-talk | grab-choke | grab-release |
// grab-auto`): slice 1 of docs/internal/grab-interrogate-design.md — HOLD Space on a living guard's row from behind grabs him (player.ts startGrab / updateHold),
// the pair moves as one with his key opening keyed doors, Space taps question him through the dialogue engine on its own pacing, Space held chokes him out
// (killGuard 'choked': out cold, breathing, the ragdoll), E shoves him off an instant witness; a held man is off the AI and off the net (guards.ts held branch /
// offNet). Each stands the full game up headless (headless.ts), stages one situation, drives it through the REAL input path where the grammar is the point
// (input.keys / pressed, the hover row), prints what happened, PASS / FAIL. All of it with the pistol HOLSTERED first (armStandUp): the grab takes the arm
// variant — the one this family is about; the pistol-to-the-head variant a drawn gun would make of the same press has its own family (gunhold-probes.ts).
import { standUp, ROOT } from './headless.ts';
const IK = 'Space';
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { hitPlayer } = await import(`${ROOT}/src/game/combat.ts`);
const playerMod = await import(`${ROOT}/src/game/player.ts`);
const { startGrab, buildInteractables, GRAB_HOLD_SECS } = playerMod;
const { HOLD } = await import(`${ROOT}/src/game/character.ts`);
const { isBreathing } = await import(`${ROOT}/src/game/game.ts`);
const guardsMod = await import(`${ROOT}/src/game/guards.ts`);
const { rollcallState, witnessSummary, offNet } = guardsMod;
const Dlg = await import(`${ROOT}/src/game/dialogue.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60;
/** the game stood up with Sam's pistol holstered (E), so a grab is the ARM variant (player.ts grabVariant: drawn → 'gun', holstered / canister → 'arm') */
async function armStandUp() { const W = await standUp(); W.game.player.holstered = true; return W; }
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
/** hold `code` down for `secs` of frames (keys + the press edge on the first), then let it up; returns frames stepped */
function holdKey(W: any, code: string, secs: number, each?: () => void): number { const n = Math.max(1, Math.round(secs / DT)); W.input.keys.add(code); W.input.pressed.add(code); for (let i = 0; i < n; i++) { W.step(DT); each?.(); } W.input.keys.delete(code); return n; }
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
/** grab A through the real press: the cursor on him, Space held past GRAB_HOLD_SECS; returns the frame the hold began (−1 never) */
function grabViaHold(S: any, secs = GRAB_HOLD_SECS + 0.15): number {
  const { W, game, A } = S; let at = -1;
  W.cursorAt(v3.add(A.char.pos, [0, 1.35, 0]));
  holdKey(W, IK, secs, () => { if (at < 0 && game.player.holding) at = Math.round(game.time / DT); });
  return at;
}
/** step until the hold proper has begun (phase 'held'), at most 2 s */
function untilHeld(S: any): boolean { const { W, game } = S; for (let i = 0; i < 120; i++) { if (game.player.holding?.phase === 'held') return true; W.step(DT); } return game.player.holding?.phase === 'held'; }

// ================================================================ (1) the gate: wherever the takedown is offered the grab is, and nowhere else — never through a wall or a leaf, never from the front, never on an alert man
export async function grabGate() {
  console.log('\n=== grab-gate: HOLD Space grabs exactly where a tap would take him down — from behind, in reach, nothing solid between, not alert ===');
  let allPass = true;
  const cases: { note: string; guard: Vec3; yaw: number; player: Vec3; state?: string; expect: boolean }[] = [
    { note: 'open corridor, crouched 0.8 m behind a calm man', guard: [14, 0, 11.2], yaw: Math.PI / 2, player: [13.2, 0, 11.2], expect: true },
    { note: 'the same man SUSPICIOUS (looking into a noise ahead of him): still grabbable from behind', guard: [14, 0, 11.2], yaw: Math.PI / 2, player: [13.2, 0, 11.2], state: 'suspicious', expect: true },
    { note: 'the same man ALERT: refused (no grab while he is onto you)', guard: [14, 0, 11.2], yaw: Math.PI / 2, player: [13.2, 0, 11.2], state: 'alert', expect: false },
    { note: 'in FRONT of him, 0.8 m: refused (get behind him)', guard: [14, 0, 11.2], yaw: -Math.PI / 2, player: [13.2, 0, 11.2], expect: false },
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
    if (c.state === 'suspicious') { gd.state = 'suspicious'; gd.awareness = 0.5; gd.lastKnown = [c.guard[0] + Math.sin(c.yaw) * 4, 0, c.guard[2] + Math.cos(c.yaw) * 4]; gd.reactT = 0; }
    if (c.state === 'alert') { gd.state = 'alert'; gd.awareness = 1; gd.lastKnown = v3.copy(c.player); gd.lastSeenT = -100; gd.reactT = 5; gd.pinned = true; }
    game.teleportPlayer(v3.copy(c.player)); game.player.crouch = true; W.step(DT);
    const p = game.player.char.pos, gp = gd.char.pos; const p0 = v3.copy(p);
    const wall = game.col.segmentBlocked([p[0], 1.0, p[2]], [gp[0], 1.0, gp[2]]);
    buildInteractables(game, cam, 0, 0, canvas.clientWidth, canvas.clientHeight);
    const it = game.interactables.find((x: any) => x.kind === 'guard' && x.guard === gd);
    const offersTd = !!it?.inReach && /takedown/.test(it.line2), offersGrab = !!it?.inReach && /grab/.test(it.line2);
    // the real press: cursor on him, Space down for half a second
    W.cursorAt(v3.add(gp, [0, 1.35, 0]));
    holdKey(W, IK, 0.5);
    const grabbed = !!game.player.holding || !!gd.held;
    for (let i = 0; i < 60; i++) W.step(DT);
    const q = game.player.char.pos; const crossed = game.col.segmentBlocked([p0[0], 1.0, p0[2]], [q[0], 1.0, q[2]]) && game.col.segmentBlocked([p0[0], 0.5, p0[2]], [q[0], 0.5, q[2]]);
    const fails: string[] = [];
    if (offersTd !== offersGrab) fails.push(`row offers takedown=${offersTd} but grab=${offersGrab} ("${it?.line2}")`);
    if (offersGrab !== c.expect) fails.push(`grab offered=${offersGrab}, expected ${c.expect} ("${it?.line1}" / "${it?.line2}")`);
    if (grabbed !== c.expect) fails.push(`after 0.5 s of Space on him: holding=${grabbed}, expected ${c.expect}`);
    if (crossed) fails.push(`Sam ended on the far side of static geometry: ${posS(p0)} → ${posS(q)}`);
    if (!c.expect && gd.state === 'dead') fails.push('refused, yet he went down (the tap fell through to a takedown through the wall?)');
    if (c.expect && grabbed) {
      const d = v3.distXZ(q, gd.char.pos), dyaw = Math.abs(Math.atan2(Math.sin(game.player.char.bodyYaw - gd.char.bodyYaw), Math.cos(game.player.char.bodyYaw - gd.char.bodyYaw)));
      if (Math.abs(d - HOLD.dist) > 0.03) fails.push(`pair ${f3(d)} m apart after a second (want ${HOLD.dist} ± 0.03)`);
      if (dyaw > 0.02) fails.push(`yaws differ by ${f3(dyaw)} rad`);
      if (game.player.holding?.phase !== 'held' || gd.held?.phase !== 'held') fails.push(`phase ${game.player.holding?.phase} / ${gd.held?.phase} (want held)`);
      if (gd.state === 'dead') fails.push('grabbed man is dead');
    }
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${c.note}\n    ${f2(v3.distXZ(p0, c.guard))} m, wall between ${wall}; row: ${it ? `"${it.line1}" / "${it.line2}" inReach=${it.inReach}` : 'none'}; after the hold-press: holding=${grabbed}${grabbed ? ` (${game.player.holding?.phase ?? '—'}, ${f3(v3.distXZ(q, gd.char.pos))} m apart, guard ${gd.state})` : ''}, Sam ${posS(q)}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  // and a TAP is still the takedown (the tour's beat presses Space for one frame with no key held)
  {
    const S = await stage(12); const { W, game, A } = S;
    W.cursorAt(v3.add(A.char.pos, [0, 1.35, 0])); W.step(DT);
    const hv = game.hover; tapKey(W, IK); W.step(DT);
    const td = !!game.player.takedown; for (let i = 0; i < 60; i++) W.step(DT);
    const fails: string[] = [];
    if (hv?.kind !== 'guard' || !hv.inReach) fails.push(`staging: hover ${hv?.kind} inReach ${hv?.inReach}`);
    if (!td) fails.push('a one-frame Space press did not start the takedown');
    if (A.state !== 'dead' || A.downKind !== 'struck') fails.push(`A ${A.state}/${A.downKind} after the tap (want dead/struck)`);
    if (game.player.holding || A.held) fails.push('a tap grabbed him');
    // and a short real press (5 frames down, then up) is a takedown too, on the release
    const S2 = await stage(13); const W2 = S2.W, g2 = S2.game, A2 = S2.A;
    W2.cursorAt(v3.add(A2.char.pos, [0, 1.35, 0])); W2.step(DT);
    holdKey(W2, IK, 5 * DT); const tdDuring = !!g2.player.takedown; W2.step(DT); const tdAfter = !!g2.player.takedown || A2.state === 'dead';
    for (let i = 0; i < 60; i++) W2.step(DT);
    if (tdDuring) fails.push('a 5-frame press started the takedown while the key was still down (should go on release)');
    if (!tdAfter || A2.state !== 'dead') fails.push(`a 5-frame press then release: takedown=${tdAfter}, A ${A2.state}`);
    if (g2.player.holding) fails.push('a 5-frame press grabbed');
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — a TAP on his row is still the takedown: one-frame synthetic press → takedown ${td}, A ${A.state}/${A.downKind}; 5-frame press → nothing while down (${!tdDuring}), takedown on release (${tdAfter})${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
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
      maxPenP = Math.max(maxPenP, penetration(col, p, 0.42, 0.2, 1.7)); maxPenG = Math.max(maxPenG, penetration(col, gp, 0.3, 0.2, 1.5));
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
    for (let f = 0; f < 60 * 3; f++) { W.step(DT); const p = game.player.char.pos, gp = A.char.pos; maxDev = Math.max(maxDev, Math.abs(v3.distXZ(p, gp) - HOLD.dist)); maxPenP = Math.max(maxPenP, penetration(game.col, p, 0.42, 0.2, 1.7)); maxPenG = Math.max(maxPenG, penetration(game.col, gp, 0.3, 0.2, 1.5)); }
    const stopZ = A.char.pos[2];
    game.puppet = { goal: [26, 0, 10.4], aim: [20, 1.0, 4.0] };   // now sideways along it, still facing it
    let slid = 0; const x0 = A.char.pos[0];
    for (let f = 0; f < 60 * 3; f++) { W.step(DT); const p = game.player.char.pos, gp = A.char.pos; maxDev = Math.max(maxDev, Math.abs(v3.distXZ(p, gp) - HOLD.dist)); maxPenP = Math.max(maxPenP, penetration(game.col, p, 0.42, 0.2, 1.7)); maxPenG = Math.max(maxPenG, penetration(game.col, gp, 0.3, 0.2, 1.5)); }
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
  // ---- let him go: E on his row
  W.cursorAt(v3.add(victim.char.pos, [0, 1.6, 0])); W.step(DT);
  const hv = game.hover?.kind;
  tapKey(W, 'KeyE');
  let relPhaseSeen = false, tFree = -1; const p0 = v3.copy(victim.char.pos);
  for (let i = 0; i < 90; i++) { W.step(DT); if (game.player.holding?.phase === 'release') relPhaseSeen = true; if (tFree < 0 && !victim.held) tFree = game.time; }
  const stumble = v3.distXZ(victim.char.pos, p0);
  const contact = lines.find(l => l.who === victim.callsign && /CONTACT/.test(l.text));
  if (hv !== 'held') fails.push(`hover while holding is ${hv} (want the held row)`);
  if (!relPhaseSeen) fails.push('E did not start the release phase');
  if (tFree < 0) fails.push('he was never freed');
  if (game.player.holding) fails.push('Player.holding still set 1.5 s after E');
  if (victim.held || game.guards.some((x: any) => x.held)) fails.push('a Guard.held still set');
  if (game.player.char.anim.holdPose || victim.char.anim.heldPose) fails.push(`animator flags left: holdPose ${!!game.player.char.anim.holdPose} heldPose ${!!victim.char.anim.heldPose}`);
  if (victim.state !== 'alert') fails.push(`freed man is ${victim.state} (want alert)`);
  if (victim.awareness < 0.99) fails.push(`awareness ${f2(victim.awareness)}`);
  if (!victim.lastKnown || v3.distXZ(victim.lastKnown, game.player.char.pos) > 0.6) fails.push(`his fix ${victim.lastKnown ? posS(victim.lastKnown) : 'none'} is not on Sam ${posS(game.player.char.pos)}`);
  if (victim.witness.wasHeld !== 1) fails.push(`witness.wasHeld ${victim.witness.wasHeld} (want 1)`);
  if (victim.witness.alertedBy !== 'sight') fails.push(`witness.alertedBy ${victim.witness.alertedBy} (want sight: he knows exactly who)`);
  if (Dlg.knowledgeOf(game, victim) !== 'saw') fails.push(`dialogue knowledge ${Dlg.knowledgeOf(game, victim)} (want saw)`);
  if (victim.talk.heldSince !== -1) fails.push(`talk.heldSince ${victim.talk.heldSince} after release (want −1)`);
  if (stumble < 0.4 || stumble > 1.4) fails.push(`he stumbled ${f2(stumble)} m off the shove (want ~0.6–1.2)`);
  if (!contact) fails.push(`no CONTACT bark from him after release (his lines: ${lines.filter(l => l.who === victim.callsign && l.t > tGrab).map(l => l.text).join(' | ') || 'none'})`);
  if (offNet(victim)) fails.push('still offNet after release');
  // the checker's errand: he is back on the net, so the walk round his route is dropped (checkOnTick) within a frame or two of the checker's next suspicious tick
  for (let i = 0; i < 30; i++) W.step(DT);
  const stillChecking = others.some((x: any) => x.task?.kind === 'checkOn' && x.task.who === victim && x.state === 'suspicious');
  if (stillChecking) fails.push('somebody is still walking his route half a second after he came back on the net');
  if (victim.missedAt !== -1) fails.push(`missedAt ${victim.missedAt} after release (want −1: nothing about him is missing)`);
  const ok = fails.length === 0; allPass &&= ok;
  console.log(`- ${ok ? 'PASS' : 'FAIL'} — held through the check, then E: release phase ${relPhaseSeen}, freed at ${tFree >= 0 ? f2(tFree) + ' s' : 'never'}, stumbled ${f2(stumble)} m; now ${victim.state} aw ${f2(victim.awareness)} fix ${victim.lastKnown ? posS(victim.lastKnown) : '—'} (Sam ${posS(game.player.char.pos)}); wasHeld ${victim.witness.wasHeld}, alertedBy ${victim.witness.alertedBy}, K ${Dlg.knowledgeOf(game, victim)}; bark: "${contact?.text ?? '—'}"; checker still on it: ${stillChecking}\n    witness now: ${witnessSummary(game, victim)}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  console.log(`=== grab-rollcall: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (4) questions: taps on his row go through Game.interrogate on the engine's pacing — one stage per accepted tap, a tap too soon cuts the line instead
export async function grabTalk() {
  console.log('\n=== grab-talk: Space taps on the held row — one stage per accepted tap, the pacing rule (1.2 s AND his answer inside its last half second; a tap sooner cuts it short) ===');
  let allPass = true;
  const S = await stage(41); const { W, game, A, msgs } = S;
  const at = grabViaHold(S); const heldOk = untilHeld(S);
  W.step(DT);   // (Space came up at the end of the grab press: fHeld clears)
  const fails: string[] = [];
  if (at < 0 || !heldOk) fails.push(`staging: grab at frame ${at}, held ${heldOk}`);
  const rec: string[] = []; const stageAt = (label: string) => rec.push(`${label}: stage ${A.talk.stage} presses ${A.talk.presses} bubble ${A.bubble ? `"${A.bubble.text.slice(0, 34)}…" ${f2(A.bubble.dur - (game.time - A.bubble.t))} s left` : '—'} · row "${game.hover?.line2?.split('  ·  ')[0] ?? '—'}"`);
  const press = (label: string, thenSecs: number) => { tapKey(W, IK); W.step(DT); stageAt(label); for (let i = 0; i < Math.round(thenSecs / DT) - 1; i++) W.step(DT); };
  stageAt('before');
  const row0 = game.hover?.line2 ?? '';
  press('tap 1 (fresh: accepted → stage 1)', 0.4);
  const s1 = A.talk.stage;
  press('tap 2 at +0.4 s (too soon: cuts the line, no stage)', 0.5);
  const s2 = A.talk.stage; const cut = A.bubble ? A.bubble.dur : -1;
  press('tap 3 at +0.9 s (still inside 1.2 s: nothing)', 0.45);
  const s3 = A.talk.stage;
  press('tap 4 at +1.35 s (gap passed, line was cut: accepted → stage 2)', 0.2);
  const s4 = A.talk.stage;
  // now wait out a full answer without cutting it: the next tap is only good once it is inside its last half second
  const b = A.bubble; const fullDur = b ? b.dur : 0; const spokeAt = b ? b.t : game.time;
  while (game.time < spokeAt + Math.max(1.2, fullDur - 0.5) - 0.15) W.step(DT);
  tapKey(W, IK); W.step(DT); const s5 = A.talk.stage; stageAt(`tap 5 just BEFORE ready (${f2(game.time - spokeAt)} s after a ${f2(fullDur)} s answer: cuts, no stage)`);
  for (let i = 0; i < 80; i++) W.step(DT);
  tapKey(W, IK); W.step(DT); const s6 = A.talk.stage; stageAt('tap 6 at +1.3 s more (accepted → stage 3: the useful thing or the refusal)');
  for (let i = 0; i < 80; i++) W.step(DT); tapKey(W, IK); W.step(DT); const s7 = A.talk.stage; for (let i = 0; i < 80; i++) W.step(DT); tapKey(W, IK); W.step(DT); const s8 = A.talk.stage; stageAt('tap 7 at +1.3 s (mid-answer: cuts it), tap 8 at +1.3 s more (→ 4: nothing left, the x pool)');
  const rowSpent = game.hover?.line2 ?? '';
  if (!/make him talk/.test(row0)) fails.push(`first row question: "${row0}" (want “make him talk”)`);
  if (s1 !== 1) fails.push(`tap 1: stage ${s1} (want 1)`);
  if (s2 !== 1) fails.push(`tap 2 (0.4 s later) advanced to ${s2}`); if (!(cut > 0 && cut < 1.0)) fails.push(`tap 2 did not cut the line short (dur now ${f2(cut)})`);
  if (s3 !== 1) fails.push(`tap 3 (0.9 s) advanced to ${s3}`);
  if (s4 !== 2) fails.push(`tap 4 (1.35 s, line cut): stage ${s4} (want 2)`);
  if (s5 !== 2) fails.push(`tap 5 (before the answer's tail): stage ${s5} (want still 2)`);
  if (s6 !== 3) fails.push(`tap 6: stage ${s6} (want 3)`);
  if (s7 !== 3) fails.push(`tap 7 (mid-answer): stage ${s7} (want still 3 — it only cuts the line)`);
  if (s8 !== 4) fails.push(`tap 8: stage ${s8} (want 4: the exhausted pool, and it keeps counting from there)`);
  if (!/said his piece/.test(rowSpent)) fails.push(`row once dry: "${rowSpent.split('  ·  ')[0]}" (want "he's said his piece — ask anyway")`);
  if (A.talk.heldSince < 0 || Math.abs(A.talk.heldSince - (game.player.holding?.since ?? -9)) > 0.5 + HOLD.secs.grab) fails.push(`talk.heldSince ${f2(A.talk.heldSince)} vs hold since ${f2(game.player.holding?.since ?? -1)}`);
  const you = msgs.filter(m => /^you ▸/.test(m.text)).map(m => m.text.replace('you ▸ ', ''));
  if (you.length !== 3 || you[0] !== 'make him talk' || you[1] !== 'lean on him' || you[2] !== 'ask what he knows') fails.push(`Sam's logged questions: ${you.join(' | ')} (want the three staged prompts, none once dry)`);
  if (!game.player.holding || A.state === 'dead') fails.push('the hold ended / he died under questioning');
  const ok = fails.length === 0; allPass &&= ok;
  console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${A.callsign} (${Dlg.temperamentOf(A)}, K ${Dlg.knowledgeOf(game, A)}): stages ${[s1, s2, s3, s4, s5, s6, s7, s8].join('/')}\n    ${rec.join('\n    ')}\n    Sam's side in the log: ${you.join(' | ') || '—'}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  console.log(`=== grab-talk: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (5) the choke: Space held through HOLD.secs.choke → 'choked', breathing, the torch dies, the ragdoll sleeps; found later as out cold. Let go of Space sooner and he breathes.
export async function grabChoke() {
  console.log('\n=== grab-choke: Space held on his row — the choke starts at 0.35 s, he goes at +2.6 s out cold and breathing, torch dark, ragdoll asleep; eased off sooner he lives ===');
  let allPass = true;
  // ---- (a) all the way
  {
    const S = await stage(51); const { W, game, A, B, lines, msgs } = S;
    grabViaHold(S); untilHeld(S); W.step(DT); W.step(DT);
    const torchLit0 = A.flashlight.enabled;
    let chokeAt = -1, deadAt = -1; const t0 = game.time;
    W.input.keys.add(IK); W.input.pressed.add(IK);
    for (let i = 0; i < 60 * 4 && deadAt < 0; i++) { W.step(DT); if (chokeAt < 0 && game.player.holding?.phase === 'choke') chokeAt = game.time - t0; if (deadAt < 0 && A.state === 'dead') deadAt = game.time - t0; }
    const heldStill = !!game.player.holding, fHeld = game.player.fHeld;
    for (let i = 0; i < 30; i++) W.step(DT);   // Space still down half a second: nothing must come of it (spent)
    W.input.keys.delete(IK);
    let darkAt = -1, sleptAt = -1;
    for (let i = 0; i < 60 * 8; i++) { W.step(DT); if (darkAt < 0 && !A.flashlight.enabled) darkAt = game.time - t0 - deadAt; if (sleptAt < 0 && A.char.ragdoll?.sleeping) sleptAt = game.time - t0 - deadAt; }
    const st = game.mission.stats;
    const fails: string[] = [];
    if (chokeAt < 0 || Math.abs(chokeAt - HOLD.secs.chokeHold) > 0.05) fails.push(`choke began at +${f2(chokeAt)} s of Space (want ${HOLD.secs.chokeHold})`);
    if (deadAt < 0 || Math.abs(deadAt - (HOLD.secs.chokeHold + HOLD.secs.choke)) > 0.06) fails.push(`went down at +${f2(deadAt)} s (want ${f2(HOLD.secs.chokeHold + HOLD.secs.choke)})`);
    if (A.downKind !== 'choked' || A.downBy !== 'player' || !isBreathing(A)) fails.push(`A ${A.state}/${A.downKind}/${A.downBy} breathing ${isBreathing(A)} (want dead/choked/player, breathing)`);
    if (heldStill || A.held) fails.push('hold not cleared on the frame he went');
    if (!fHeld) fails.push('the Space that choked him was not marked spent (fHeld)');
    if (game.player.takedown || game.player.holding) fails.push('the still-held Space started something else');
    if (!A.char.ragdoll) fails.push('no ragdoll'); if (sleptAt < 0) fails.push('ragdoll never slept in 8 s');
    if (!torchLit0) fails.push('staging: his torch was not lit in the hold'); if (A.dropped?.kind !== 'torch') fails.push(`dropped ${A.dropped?.kind ?? 'nothing'} (want the torch)`); if (darkAt < 0 || darkAt > 1.2) fails.push(`torch dark at ${darkAt < 0 ? 'never' : '+' + f2(darkAt) + ' s'} after he went (want < 1 s)`);
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
  // ---- (b) eased off at 1.5 s: back to held, alive; then a full one still works
  {
    const S = await stage(52); const { W, game, A, msgs } = S;
    grabViaHold(S); untilHeld(S); W.step(DT); W.step(DT);
    holdKey(W, IK, 1.5);
    const phaseDuring = A.held?.phase; W.step(DT); W.step(DT);
    const phaseAfter = game.player.holding?.phase;
    const fails: string[] = [];
    if (phaseDuring !== 'choke') fails.push(`1.5 s of Space: phase ${phaseDuring} (want choke)`);
    if (phaseAfter !== 'held' || A.state === 'dead') fails.push(`after letting go: phase ${phaseAfter}, A ${A.state} (want held, alive)`);
    if (!msgs.some(m => /eased off/.test(m.text))) fails.push('no eased-off line');
    for (let i = 0; i < 40; i++) W.step(DT);
    const ck0 = (A.char.anim as any).heCk?.x ?? -1;
    holdKey(W, IK, HOLD.secs.chokeHold + HOLD.secs.choke + 0.1);
    if (A.state !== 'dead' || A.downKind !== 'choked') fails.push(`the second, full choke: A ${A.state}/${A.downKind}`);
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — (b) eased off at 1.5 s: phase ${phaseDuring} → ${phaseAfter}, alive; the animator's choke amount had come back to ${ck0 >= 0 ? f2(ck0) : '?'} before the second; the full one then: ${A.state}/${A.downKind}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  console.log(`=== grab-choke: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (6) letting go: E → the shove, he stumbles ~1 m, alert on Sam within a frame of coming free, wasHeld, nothing of the hold left anywhere; and no re-grab of an alert man
export async function grabRelease() {
  console.log('\n=== grab-release: E on his row — shove, stumble, instant witness (alert on Sam, wasHeld 1), nothing residual; the alert man cannot be grabbed again ===');
  let allPass = true;
  const S = await stage(61); const { W, game, A, lines } = S; const { cam, canvas } = W;
  grabViaHold(S); untilHeld(S); W.step(DT);
  for (let i = 0; i < 60; i++) W.step(DT);
  const p0 = v3.copy(A.char.pos), yaw0 = A.char.bodyYaw;
  tapKey(W, 'KeyE');
  let freeFrame = -1, alertFrame = -1; const trace: string[] = [];
  for (let i = 0; i < 120; i++) {
    W.step(DT);
    if (freeFrame < 0 && !A.held) freeFrame = i;
    if (alertFrame < 0 && A.state === 'alert') alertFrame = i;
    if (i % 10 === 0 || i === freeFrame || i === alertFrame) trace.push(`f${i} ${game.player.holding?.phase ?? 'free'} him ${posS(A.char.pos)} ${A.state} aw ${f2(A.awareness)} reactT ${f2(A.reactT)} spd ${f2(A.speed)}`);
  }
  const stumble = v3.distXZ(A.char.pos, p0); const fwd: Vec3 = [Math.sin(yaw0), 0, Math.cos(yaw0)]; const along = (A.char.pos[0] - p0[0]) * fwd[0] + (A.char.pos[2] - p0[2]) * fwd[2];
  const fails: string[] = [];
  if (freeFrame < 0 || Math.abs(freeFrame - Math.round(HOLD.secs.release / DT)) > 2) fails.push(`freed at frame ${freeFrame} (want ≈ ${Math.round(HOLD.secs.release / DT)})`);
  if (alertFrame < 0 || alertFrame - freeFrame > 1) fails.push(`alert at frame ${alertFrame}, freed at ${freeFrame} (want within a frame)`);
  if (stumble < 0.5 || stumble > 1.4 || along < 0.4) fails.push(`stumbled ${f2(stumble)} m (${f2(along)} m forward) — want ~0.6–1.2 m off the front`);
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
    grabViaHold(S); untilHeld(S); W.step(DT); W.input.keys.add(IK); W.input.pressed.add(IK); for (let i = 0; i < 60; i++) W.step(DT);   // a second into a choke
    const phase = game.player.holding?.phase;
    game.restartEncounter(); W.input.keys.delete(IK); for (let i = 0; i < 3; i++) W.step(DT);
    const fails: string[] = []; if (phase !== 'choke') fails.push(`staging: phase ${phase}`); const r = residue(game); if (r.length) fails.push('residue: ' + r.join(', ')); if (game.guards.some((x: any) => x.state !== 'patrol' || x.awareness !== 0 || x.witness.wasHeld)) fails.push(`fresh guards not fresh: ${game.guards.map((x: any) => `${x.state}/${f2(x.awareness)}/held×${x.witness.wasHeld}`).join(' ')}`); if (game.player.guardHold !== 0 || game.player.fHeld) fails.push(`player guardHold ${game.player.guardHold} fHeld ${game.player.fHeld}`);
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
  // (f) Q / Shift / C / LMB while holding do nothing they should not; E lets go only from 'held'
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
