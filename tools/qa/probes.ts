// Directed headless probes for things the soak surfaced — each stands the full game up (headless.ts), stages one situation exactly, steps it, and prints
// what happened, so a finding comes with a deterministic repro instead of a seed + frame.
//   bun run tools/qa/probes.ts [leaf-wedge|crush|dead-script|silent-repair|takedown-wall|body-clear|corpse|picked-door|follower-corpse|reclear|world-edge|teleport-cancel|player|all]
//   ('player' = the player-side PASS/FAIL set: world-edge, teleport-cancel, crush)
import { standUp, ROOT } from './headless.ts';
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
const { bodyPos } = await import(`${ROOT}/src/game/guards.ts`);
const { PLAYER_ID } = await import(`${ROOT}/src/game/consts.ts`);

type Vec3 = [number, number, number];
const which = process.argv[2] ?? 'all';
const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }

/** A door leaf standing at (or pushed to) its stop across a walking line: an alert guard whose nav path runs through it. Does he ever get past? */
async function leafWedge() {
  console.log('\n=== leaf-wedge: alert guards vs a leaf held against its stop (nav ignores leaves; the contact model cannot yield at the stop) ===');
  const cases: { door: string; angle: number; guardAt: Vec3; goal: Vec3; note: string }[] = [
    { door: 'breakroom_ext', angle: 1.92, guardAt: [31.5, 0, 24.8], goal: [19.4, 0, 24.64], note: 'exterior break-room door swung out into the lot; chase runs west along the south wall' },
    { door: 'breakroom_w', angle: 1.92, guardAt: [27.1, 0, 17.3], goal: [27.6, 0, 15.6], note: 'break-room side door pushed past its authored 1.35 to the stop, folded into the cubicle aisle; guard wants north past it' },
    { door: 'breakroom_n', angle: -1.7, guardAt: [30.5, 0, 11.8], goal: [28.6, 0, 11.7], note: 'break-room corridor door swung out into the corridor; guard walks the corridor west' },
    { door: 'conference', angle: -1.92, guardAt: [10.6, 0, 9.3], goal: [8.0, 0, 9.4], note: 'conference door (authored -1.25, no closer) at its stop inside the room; #1-style move west along the south glass' },
  ];
  for (const c of cases) {
    Math.random = seeded(7);
    const W = await standUp(); const { game } = W;
    game.playerInvisible = true; game.godMode = true; game.teleportPlayer([4.9, 0, 23.2]);   // nobody to see: this is about feet, not eyes
    for (let i = 0; i < 30; i++) W.step(DT);
    const d = game.doors.byName(c.door); d.latched = false; d.angle = c.angle; d.vel = 0; d.closing = false; d.quietT = 0; d.place();
    const gd = game.guards[0]; for (const o of game.guards.slice(1)) { o.hold = true; }   // the others stay out of it
    gd.char.pos = v3.copy(c.guardAt); gd.char.update(0); gd.state = 'alert'; gd.awareness = 0.95; gd.lastKnown = v3.copy(c.goal); gd.lastSeenT = -100; gd.reactT = 0; gd.path = []; gd.pathGoal = null;
    const p0 = v3.copy(gd.char.pos); let maxFromStart = 0; let arrivedAt = -1; const angles: string[] = []; let sidesteps = 0; let lastPathLen = 0;
    for (let f = 0; f < 60 * 25; f++) {
      W.step(DT);
      if (gd.path.length > lastPathLen + 0 && gd.path.length !== lastPathLen) { if (gd.path.length === lastPathLen + 1) sidesteps++; lastPathLen = gd.path.length; }
      maxFromStart = Math.max(maxFromStart, v3.distXZ(gd.char.pos, p0));
      if (arrivedAt < 0 && (gd.state === 'search' || v3.distXZ(gd.char.pos, c.goal) < 1.0)) arrivedAt = game.time;
      if (f % 120 === 0) angles.push(`${(f / 60).toFixed(0)}s:door ${d.angle.toFixed(2)} guard (${gd.char.pos[0].toFixed(2)},${gd.char.pos[2].toFixed(2)}) ${gd.state} spd ${gd.speed.toFixed(2)} stuckT ${gd.stuckT.toFixed(2)} path ${gd.pathI}/${gd.path.length}`);
    }
    console.log(`\n- ${c.door} @ ${c.angle} — ${c.note}`);
    console.log(`  guard from (${p0[0]}, ${p0[2]}) toward (${c.goal[0]}, ${c.goal[2]}): ${arrivedAt >= 0 ? `got there / gave up chasing at ${arrivedAt.toFixed(1)} s` : 'NEVER got past in 25 s'}; furthest from start ${maxFromStart.toFixed(2)} m; sidestep insertions ~${sidesteps}; door now ${d.angle.toFixed(2)} (stop ${d.angle > 0 ? d.maxA : d.minA})`);
    console.log('  ' + angles.join('\n  '));
  }
}

/** Guards do not collide with the player, only the player yields (player.ts keepOffGuards) — a man walking into a player who has a wall at his back, or who simply
 *  stands in his way (the tour's invisible Sam, a sandbox player): the player must never be left inside the statics (let alone come out of a partition's far side),
 *  must not be carried along ahead of him for metres, and the guard — AI or scripted — must get past. Per frame: how deep the player's collision circle sits in the
 *  statics (re-running his own collideCircle on a copy), his one-frame jumps, how far he ends up from where he stood and how much of that was along the man's travel;
 *  and whether the man covered the ground beyond him. PASS / FAIL per case. */
async function crush() {
  console.log('\n=== crush: guards walking into / through the player — never into a wall, never bulldozed, the guard always gets past ===');
  type Case = { note: string; player: Vec3; crouch?: boolean; guardFrom: Vec3; how: 'fix' | 'script' | 'patrol'; to: Vec3; maxDisp: number };
  const cases: Case[] = [
    { note: 'break room, back to the west wall (x=28, 12 cm), a suspicious man walks to a fix just short of him from the east, then leaves north along that wall', player: [28.5, 0, 20.6], guardFrom: [31.5, 0, 20.6], how: 'fix', to: [28.6, 0, 20.6], maxDisp: 1.2 },
    { note: 'server room, back to the server/manager partition (x=22) between the rack bank and the side door, fix from the west a step short of him', player: [21.5, 0, 7.6], guardFrom: [17.0, 0, 7.7], how: 'fix', to: [21.1, 0, 7.6], maxDisp: 1.2 },
    { note: 'corridor, back to the conference glazing (z=10), fix from the south; he then walks off west along the glazing through the spot', player: [12.0, 0, 10.55], guardFrom: [12.0, 0, 12.5], how: 'fix', to: [12.0, 0, 10.65], maxDisp: 1.2 },
    { note: 'SCRIPTED man (squad runner, blind) ordered straight along the corridor\'s north wall through a player standing against the glazing', player: [12.0, 0, 10.55], guardFrom: [15.5, 0, 10.7], how: 'script', to: [7.5, 0, 10.7], maxDisp: 1.2 },
    { note: 'SCRIPTED man ordered through a player standing square in his way on open floor (cubicle aisle) — shouldered aside, not carried', player: [19.9, 0, 15.6], guardFrom: [19.9, 0, 13.0], how: 'script', to: [19.9, 0, 18.6], maxDisp: 1.0 },
    { note: 'SCRIPTED man through a player crouched in the west cubicle doorway (x 15‥16.4 at z=12.2): boxed in by the jambs, carried the depth of the doorway then aside', player: [15.7, 0, 12.2], crouch: true, guardFrom: [15.7, 0, 11.0], how: 'script', to: [15.7, 0, 14.2], maxDisp: 1.6 },
    { note: 'the corridor PATROL (AI, route corridor) meets a player standing on his line at (20, 11.1) head-on', player: [20.0, 0, 11.1], guardFrom: [14.0, 0, 11.1], how: 'patrol', to: [34, 0, 11.1], maxDisp: 1.0 },
    { note: 'player in the corridor\'s north-west corner (two walls), a scripted man sent into the corner spot itself and back out', player: [4.6, 0, 10.5], guardFrom: [7.0, 0, 11.2], how: 'script', to: [4.7, 0, 10.6], maxDisp: 1.2 },
  ];
  let allPass = true;
  for (const c of cases) {
    Math.random = seeded(3);
    const W = await standUp(); const { game } = W;
    game.godMode = true; game.playerInvisible = true; game.aiEnabled = true;
    for (let i = 0; i < 10; i++) W.step(DT);
    game.teleportPlayer(v3.copy(c.player)); game.player.crouch = !!c.crouch; W.step(DT); const start = v3.copy(game.player.char.pos);
    const gd = game.guards[0]; for (const o of game.guards.slice(1)) { o.hold = true; o.char.pos = [33, 0, 6 + game.guards.indexOf(o)]; o.char.update(0); }   // the others parked in storage, out of it
    gd.char.pos = v3.copy(c.guardFrom); gd.char.bodyYaw = gd.char.aimYaw = Math.atan2(c.to[0] - c.guardFrom[0], c.to[2] - c.guardFrom[2]); gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.char.update(DT);
    if (c.how === 'fix') { gd.state = 'suspicious'; gd.awareness = 0.6; gd.reactT = 0; gd.searchT = 0; gd.lastKnown = v3.copy(c.to); gd.task = { kind: 'rack', pos: v3.copy(c.to) }; }   // (task: keeps him on it)
    else if (c.how === 'script') gd.script = { goal: v3.copy(c.to), speed: 1.3, upper: 'relaxed' };
    else { gd.state = 'patrol'; gd.wp = 1; gd.wait = 0; }   // corridor route: wp 1 = (34, 11.1), straight through (20, 11.1)
    const y1 = () => game.player.crouch ? 1.0 : 1.7;
    let maxPen = 0, maxStep = 0, maxOff = 0, minD = 99, carried = 0; let through = false; let prev = v3.copy(start); const trace: string[] = [];
    let contactAt = -1; let guardAtContact: Vec3 | null = null; let guardTravelAfter = 0; let backOut = false;
    for (let f = 0; f < 60 * 16; f++) {
      W.step(DT);
      if (c.how === 'script' && gd.script?.arrived && !backOut && v3.distXZ(c.to, c.player) < 0.5) { backOut = true; gd.script = { goal: v3.copy(c.guardFrom), speed: 1.3, upper: 'relaxed' }; }   // the corner case: in, then back out through him again
      const p = game.player.char.pos; const gp = gd.char.pos;
      const q = v3.copy(p); game.col.collideCircle(q, 0.42, 0.2, y1(), 3); maxPen = Math.max(maxPen, v3.distXZ(q, p));
      const step = v3.distXZ(p, prev); maxStep = Math.max(maxStep, step);
      const d = v3.distXZ(p, gp); minD = Math.min(minD, d);
      if (d < 0.56 && contactAt < 0) { contactAt = game.time; guardAtContact = v3.copy(gp); }
      if (guardAtContact) guardTravelAfter = Math.max(guardTravelAfter, v3.distXZ(gp, guardAtContact));
      const gs = Math.hypot(gd.char.vel[0], gd.char.vel[2]); if (gs > 0.2 && d < 0.7) carried += ((p[0] - prev[0]) * gd.char.vel[0] + (p[2] - prev[2]) * gd.char.vel[2]) / gs;   // player motion along the man's travel while in contact
      maxOff = Math.max(maxOff, v3.distXZ(p, start));
      if (!through && game.col.segmentBlocked([start[0], 0.6, start[2]], [p[0], 0.6, p[2]]) && game.col.segmentBlocked([start[0], 1.2, start[2]], [p[0], 1.2, p[2]])) { through = true; trace.push(`    ${game.time.toFixed(2)}s: player now on the FAR side of static geometry at (${p[0].toFixed(2)}, ${p[2].toFixed(2)})`); }
      if (f % 60 === 0 || (step > 0.05)) trace.push(`    ${game.time.toFixed(2)}s player (${p[0].toFixed(2)}, ${p[2].toFixed(2)}) guard (${gp[0].toFixed(2)}, ${gp[2].toFixed(2)}) ${gd.script ? 'scripted' + (gd.script.arrived ? ' arrived' : '') : gd.state} d=${d.toFixed(2)}${step > 0.05 ? `  ← ${step.toFixed(3)} m this frame` : ''}`);
      prev = v3.copy(p);
    }
    const gotPast = c.how === 'fix' ? guardTravelAfter > 1.5 || v3.distXZ(gd.char.pos, c.to) < 0.6 : guardTravelAfter > 2.0 || v3.distXZ(gd.char.pos, c.how === 'script' && backOut ? c.guardFrom : c.to) < 0.6;
    const fails: string[] = [];
    if (maxPen > 0.01) fails.push(`player ${maxPen.toFixed(3)} m inside static geometry at worst`);
    if (through) fails.push('player came out on the far side of a wall');
    if (maxOff > c.maxDisp) fails.push(`player displaced ${maxOff.toFixed(2)} m (> ${c.maxDisp})`);
    if (Math.abs(carried) > 0.8) fails.push(`carried ${carried.toFixed(2)} m along the man's travel`);
    if (maxStep > 0.11) fails.push(`one-frame jump of ${maxStep.toFixed(3)} m`);
    if (contactAt >= 0 && !gotPast) fails.push(`the guard never got past (covered ${guardTravelAfter.toFixed(2)} m after contact, now ${v3.distXZ(gd.char.pos, c.to).toFixed(2)} m from his goal, stuckT ${gd.stuckT.toFixed(2)})`);
    if (contactAt < 0) fails.push('no contact ever happened (probe staging is off)');
    const pass = fails.length === 0; allPass &&= pass;
    console.log(`\n- ${pass ? 'PASS' : 'FAIL'} — ${c.note}\n    contact at ${contactAt >= 0 ? contactAt.toFixed(1) + ' s' : 'never'}; player: max static penetration ${maxPen.toFixed(3)} m, displaced ${maxOff.toFixed(2)} m (carried along ${carried.toFixed(2)} m), biggest one-frame move ${maxStep.toFixed(3)} m, closest approach ${minD.toFixed(2)} m; guard covered ${guardTravelAfter.toFixed(2)} m after contact${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
    console.log(trace.filter((_, i) => i < 40).join('\n'));
  }
  console.log(`\n=== crush: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

/** The lot has no kerb: the ground slab, the nav grid and the traceable world all end at 40 × 28. Guards are held in by the nav grid; the player used to walk (or be
 *  teleported) straight off the edge. Walk him hard at every edge and corner (puppet goal far beyond it, sprint held), and teleport him beyond each: his circle
 *  (0.42) must stay on the slab every frame. PASS / FAIL. */
async function worldEdge() {
  console.log('\n=== world-edge: walking / sprinting / teleporting off the 40 × 28 slab ===');
  const R = 0.42, EPS = 1e-3;
  const inside = (p: Vec3) => p[0] >= R - EPS && p[0] <= 40 - R + EPS && p[2] >= R - EPS && p[2] <= 28 - R + EPS;
  Math.random = seeded(4);
  const W = await standUp(); const { game, input } = W;
  game.playerInvisible = true; game.godMode = true; game.aiEnabled = true;
  for (let i = 0; i < 10; i++) W.step(DT);
  let allPass = true;
  const walks: { note: string; from: Vec3; goal: Vec3; sprint?: boolean; crouch?: boolean }[] = [
    { note: 'west edge, across the grass', from: [2.2, 0, 14.0], goal: [-6, 0, 14.0] },
    { note: 'east edge past the dumpster, sprinting', from: [38.0, 0, 18.0], goal: [48, 0, 18.5], sprint: true },
    { note: 'north edge', from: [20.0, 0, 2.0], goal: [20, 0, -6] },
    { note: 'south edge of the car park between the parked cars, sprinting', from: [26.0, 0, 26.3], goal: [26.5, 0, 36], sprint: true },
    { note: 'south-west corner, diagonally', from: [2.0, 0, 26.0], goal: [-5, 0, 33] },
    { note: 'north-east corner, crouched', from: [38.5, 0, 1.5], goal: [46, 0, -4], crouch: true },
    { note: 'along the south edge heading east (sliding on the rim)', from: [34.0, 0, 27.4], goal: [50, 0, 29] },
  ];
  for (const w of walks) {
    game.teleportPlayer(v3.copy(w.from)); game.player.crouch = !!w.crouch;
    game.puppet = { goal: v3.copy(w.goal), aim: null, crouch: !!w.crouch }; if (w.sprint) input.keys.add('ShiftLeft');
    let worst: Vec3 = v3.copy(game.player.char.pos); let bad = 0; let maxSpeed = 0;
    for (let f = 0; f < 60 * 6; f++) {
      const before = v3.copy(game.player.char.pos); W.step(DT); const p = game.player.char.pos;
      maxSpeed = Math.max(maxSpeed, v3.distXZ(p, before) / DT);
      if (!inside(p)) { bad++; if (bad === 1) worst = v3.copy(p); }
      if (!Number.isFinite(p[0]) || !Number.isFinite(p[2])) { bad++; break; }
    }
    input.keys.delete('ShiftLeft'); game.puppet = null;
    const p = game.player.char.pos; const pass = bad === 0; allPass &&= pass;
    console.log(`- ${pass ? 'PASS' : 'FAIL'} — ${w.note}: from (${w.from[0]}, ${w.from[2]}) toward (${w.goal[0]}, ${w.goal[2]}) for 6 s (peak ${maxSpeed.toFixed(1)} m/s) → ends at (${p[0].toFixed(2)}, ${p[2].toFixed(2)})${bad ? `; ${bad} frames off the slab, first at (${worst[0].toFixed(3)}, ${worst[2].toFixed(3)})` : ''}`);
  }
  const ports: { to: Vec3; note: string }[] = [
    { to: [-3, 0, 14], note: 'west of the world' }, { to: [44, 0, 30], note: 'past the south-east corner' }, { to: [20, 0, -2], note: 'north of the world' },
    { to: [41.5, 0, 14.0], note: 'just east of the edge behind the dumpster' }, { to: [1e6, 0, -1e6], note: 'absurdly far' }, { to: [39.9, 0, 27.9], note: 'inside the margin at the corner' },
  ];
  for (const t of ports) {
    game.teleportPlayer(v3.copy(t.to)); const p = v3.copy(game.player.char.pos); W.step(DT); const q = game.player.char.pos;
    const c = v3.copy(q); const pen = game.col.collideCircle(c, R, 0.2, 1.7, 3) ? v3.distXZ(c, q) : 0;
    const pass = inside(p) && inside(q) && pen < 0.01 && !game.col.nav.isBlocked(q[0], q[2]); allPass &&= pass;
    console.log(`- ${pass ? 'PASS' : 'FAIL'} — teleport ${t.note} (${t.to[0]}, ${t.to[2]}) → (${p[0].toFixed(2)}, ${p[2].toFixed(2)}), after a frame (${q[0].toFixed(2)}, ${q[2].toFixed(2)})${pen >= 0.01 ? `, ${pen.toFixed(2)} m inside geometry` : ''}${game.col.nav.isBlocked(q[0], q[2]) ? ', on a BLOCKED nav cell' : ''}`);
  }
  console.log(`=== world-edge: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

/** teleportPlayer (panel 'teleport to cursor', every tour beat's enter, Demo.stop, the harness) in the middle of a kick, a pick, a takedown, a drag, a wind-up:
 *  the action must be dropped cleanly — nothing reels him back to the door (updateKick / updatePicking plant him at it), no door gets kicked in or picked from
 *  across the map, the man he was strangling lives and walks on, the body stays where it lay with its pins off, the canister goes back on the belt — and the
 *  player stands free at the destination with working controls. PASS / FAIL per case. */
async function teleportCancel() {
  console.log('\n=== teleport-cancel: teleported mid-kick / mid-pick / mid-takedown / mid-drag / mid-throw ===');
  const { startKick, startPicking, startTakedown, doThrow, throwOrigin } = await import(`${ROOT}/src/game/player.ts`);
  const DEST: Vec3 = [6.5, 0, 19.0];   // the lobby, by the couch: no door, no body, nobody
  let allPass = true;
  const report = (name: string, fails: string[], extra: string) => { const pass = fails.length === 0; allPass &&= pass; console.log(`- ${pass ? 'PASS' : 'FAIL'} — ${name}: ${extra}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`); };
  const fresh = async (seed: number) => { Math.random = seeded(seed); const W = await standUp(); W.game.godMode = true; W.game.playerInvisible = true; for (let i = 0; i < 10; i++) W.step(DT); for (const o of W.game.guards) o.hold = true; return W; };
  const stray = (W: any) => { const p = W.game.player.char.pos; return v3.distXZ(p, DEST); };
  // --- kick: broken off in the chamber (before KICK_IMPACT) and again a hair before the impact frame
  for (const frames of [8, 21]) {
    const W = await fresh(71); const { game } = W; const d = game.doors.byName('storage'); const kicks0 = W.audioCounts.get('lockBreak') ?? 0;
    const spot = game.doors.kickSpot(d, [32.6, 0, 10.9]); game.teleportPlayer([spot.pos[0] + 0.15, 0, spot.pos[2] + 0.1]); W.step(DT);
    startKick(game, d); for (let i = 0; i < frames; i++) W.step(DT);
    const kt = game.player.char.anim.kickTime; const wasKicking = !!game.player.kick;
    game.teleportPlayer(v3.copy(DEST));
    let maxStray = 0; for (let i = 0; i < 90; i++) { W.step(DT); maxStray = Math.max(maxStray, stray(W)); }
    const fails: string[] = [];
    if (!wasKicking) fails.push('staging: the kick never started');
    if (game.player.kick) fails.push('player.kick still set');
    if (maxStray > 0.3) fails.push(`reeled ${maxStray.toFixed(2)} m off the destination`);
    if (d.lockBroken || !d.locked || !d.latched) fails.push(`the storage door was kicked in from the lobby (locked=${d.locked} broken=${d.lockBroken} latched=${d.latched})`);
    if ((W.audioCounts.get('lockBreak') ?? 0) > kicks0) fails.push('a lock-break sound played');
    if (game.player.char.anim.kicking) fails.push('the kick animation is still running 1.5 s later');
    // and the leg does not eat the next kick: F on the door again works once the anim is done
    game.teleportPlayer([spot.pos[0], 0, spot.pos[2]]); W.step(DT); startKick(game, d); let broke = false; for (let i = 0; i < 80 && !broke; i++) { W.step(DT); broke = d.lockBroken; }
    if (!broke) fails.push('a fresh kick after the cancelled one never landed');
    report(`kick cancelled at kickTime ${kt.toFixed(2)} s (${frames} frames in)`, fails, `stray ≤ ${maxStray.toFixed(2)} m, door locked=${game.doors.byName('storage').locked || broke} → re-kick landed=${broke}`);
  }
  // --- pick: planted and clicking, F still held through the teleport
  {
    const W = await fresh(72); const { game, input } = W; const d = game.doors.byName('server');
    const spot = game.doors.workSpot(d, [17.6, 0, 10.9]); game.teleportPlayer([spot.pos[0] + 0.2, 0, spot.pos[2] + 0.15]); W.step(DT);
    startPicking(game, d); input.keys.add('KeyF');
    for (let i = 0; i < 150; i++) W.step(DT);   // 2.5 s: planted, tools in, cylinder part way
    const pick0 = d.pick; const wasPicking = game.player.picking === d && game.player.char.anim.lockpick;
    game.teleportPlayer(v3.copy(DEST));
    let maxStray = 0; for (let i = 0; i < 90; i++) { W.step(DT); maxStray = Math.max(maxStray, stray(W)); }
    const fails: string[] = [];
    if (!wasPicking || pick0 <= 0) fails.push(`staging: never got to work the lock (pick ${pick0.toFixed(2)})`);
    if (game.player.picking) fails.push('player.picking still set');
    if (game.player.char.anim.lockpick) fails.push('anim.lockpick still up (hands on tools in the lobby)');
    if (maxStray > 0.3) fails.push(`reeled ${maxStray.toFixed(2)} m off the destination`);
    if (d.pick > pick0 + 1e-6 || !d.locked) fails.push(`the server lock kept turning from the lobby (pick ${pick0.toFixed(2)} → ${d.pick.toFixed(2)}, locked=${d.locked})`);
    if (!game.player.fHeld) fails.push('fHeld not set although F is still down (a door at the destination would start listening to the same press)');
    input.keys.delete('KeyF'); W.step(DT); W.step(DT);
    if (game.player.fHeld) fails.push('fHeld did not clear once F came up');
    // walking works again (not rooted): stand him up (a pick leaves you on your knee) and puppet him a couple of metres
    if (!game.player.crouch) fails.push('not crouched after the pick was broken off (picking plants you on a knee and leaves you there)');
    game.player.crouch = false; game.puppet = { goal: [DEST[0] + 2, 0, DEST[2]], aim: null }; for (let i = 0; i < 90; i++) W.step(DT); game.puppet = null;
    if (v3.distXZ(game.player.char.pos, DEST) < 1.5) fails.push(`could not walk off afterwards (moved ${v3.distXZ(game.player.char.pos, DEST).toFixed(2)} m in 1.5 s)`);
    report('pick cancelled 2.5 s in, F held through', fails, `door pick now ${d.pick.toFixed(2)} (was ${pick0.toFixed(2)} at the teleport; an idle cylinder settles back), stray ≤ ${maxStray.toFixed(2)} m`);
  }
  // --- takedown: broken off before the cross lands (td.t < 0.28) — he lives and carries on
  {
    const W = await fresh(73); const { game } = W;
    const gd = game.guards[1]; gd.hold = false;   // the cubicle man walks his route
    for (let i = 0; i < 60; i++) W.step(DT);
    const gp = gd.char.pos, yaw = gd.char.bodyYaw; game.teleportPlayer([gp[0] - Math.sin(yaw) * 0.8, 0, gp[2] - Math.cos(yaw) * 0.8]); game.player.crouch = true; W.step(DT);
    startTakedown(game, gd); for (let i = 0; i < 6; i++) W.step(DT);   // 0.1 s into the lunge
    const wasOn = !!game.player.takedown;
    game.teleportPlayer(v3.copy(DEST));
    let maxStray = 0; const g0 = v3.copy(gd.char.pos); for (let i = 0; i < 240; i++) { W.step(DT); maxStray = Math.max(maxStray, stray(W)); }
    const fails: string[] = [];
    if (!wasOn) fails.push('staging: the takedown never started');
    if (game.player.takedown) fails.push('player.takedown still set');
    if (gd.state === 'dead') fails.push('the guard died anyway — strangled from the lobby');
    if (maxStray > 0.3) fails.push(`player pulled ${maxStray.toFixed(2)} m off the destination (station-keeping on a man rooms away)`);
    if (v3.distXZ(gd.char.pos, g0) < 0.5) fails.push(`the guard never moved again (${v3.distXZ(gd.char.pos, g0).toFixed(2)} m in 4 s, state ${gd.state}, speed ${gd.speed.toFixed(2)})`);
    if (game.player.char.anim.striking) fails.push('the cross is still playing 4 s later');
    report('takedown cancelled 0.1 s into the lunge', fails, `guard ${gd.state}, walked ${v3.distXZ(gd.char.pos, g0).toFixed(2)} m since; stray ≤ ${maxStray.toFixed(2)} m`);
  }
  // --- takedown target reset away under the lunge (P for the tour / the panel's guard reset): no corpse out of a man who no longer exists
  {
    const W = await fresh(74); const { game } = W;
    const gd = game.guards[1];
    const gp = gd.char.pos, yaw = gd.char.bodyYaw; game.teleportPlayer([gp[0] - Math.sin(yaw) * 0.8, 0, gp[2] - Math.cos(yaw) * 0.8]); W.step(DT);
    startTakedown(game, gd); for (let i = 0; i < 6; i++) W.step(DT);
    const items0 = game.items.items.length; game.clearAftermath(); game.resetGuards();
    let threw: string | null = null; try { for (let i = 0; i < 60; i++) W.step(DT); } catch (e: any) { threw = String(e?.message ?? e); }
    const fails: string[] = [];
    if (threw) fails.push(`exception: ${threw}`);
    if (game.player.takedown) fails.push('player.takedown still set after resetGuards');
    if (gd.state === 'dead' || !gd.char.alive) fails.push('the stale guard object was killed anyway');
    if (game.items.items.length > items0) fails.push(`${game.items.items.length - items0} item(s) dropped out of thin air (a dead man's torch / pistol)`);
    if (game.guards.some((x: any) => x.state !== 'patrol')) fails.push(`fresh guards not all on patrol: ${game.guards.map((x: any) => x.state).join('/')}`);
    report('guards reset 0.1 s into a takedown', fails, `stale target alive=${gd.char.alive}, items ${items0} → ${game.items.items.length}`);
  }
  // --- drag: hauling a body, teleported away — the body stays, unpinned; hands free at the destination
  {
    const W = await fresh(75); const { game } = W;
    const gd = game.guards[2]; killGuard(game, gd, [1, 0, 0], true); for (let i = 0; i < 90; i++) W.step(DT);
    const hips0 = v3.copy(gd.char.bones.hips ?? gd.char.pos);
    game.teleportPlayer([hips0[0] + 0.7, 0, hips0[2] + 0.4]); W.step(DT); game.toggleDrag(gd);
    const grabbed = game.player.dragging === gd;
    game.puppet = { goal: [hips0[0] + 3, 0, hips0[2] + 0.4], aim: null }; for (let i = 0; i < 60; i++) W.step(DT); game.puppet = null;   // haul it a little way
    const hips1 = v3.copy(gd.char.bones.hips ?? gd.char.pos); const rag: any = gd.char.ragdoll;
    game.teleportPlayer(v3.copy(DEST));
    let maxStray = 0; for (let i = 0; i < 120; i++) { W.step(DT); maxStray = Math.max(maxStray, stray(W)); }
    const hips2 = v3.copy(gd.char.bones.hips ?? gd.char.pos);
    const fails: string[] = [];
    if (!grabbed) fails.push('staging: never got hold of the body');
    if (game.player.dragging) fails.push('player.dragging still set');
    if (rag?.anyPin) fails.push('the ragdoll is still pinned to hands 15 m away');
    if (v3.distXZ(hips2, hips1) > 1.0) fails.push(`the body travelled ${v3.distXZ(hips2, hips1).toFixed(2)} m after the teleport (dragged through the walls)`);
    if (!Number.isFinite(hips2[0]) || !Number.isFinite(hips2[1]) || !Number.isFinite(hips2[2])) fails.push(`body position not finite: ${hips2}`);
    if (maxStray > 0.3) fails.push(`player pulled ${maxStray.toFixed(2)} m off the destination`);
    game.player.crouch = false; game.puppet = { goal: [DEST[0] + 2, 0, DEST[2]], aim: null }; for (let i = 0; i < 90; i++) W.step(DT); game.puppet = null;
    if (v3.distXZ(game.player.char.pos, DEST) < 1.5) fails.push('could not walk off at normal speed afterwards');
    report('drag cancelled mid-haul', fails, `body moved ${v3.distXZ(hips1, hips0).toFixed(2)} m while hauled, ${v3.distXZ(hips2, hips1).toFixed(2)} m after; stray ≤ ${maxStray.toFixed(2)} m`);
  }
  // --- wind-up: a canister committed (count spent, leaves the hand in 0.16 s) and the player teleported inside that window
  {
    const W = await fresh(76); const { game } = W;
    game.teleportPlayer([20.0, 0, 11.2]); W.step(DT);
    const cans0 = game.player.canisters; const sol = game.items.solve(throwOrigin(game), [24, 0.05, 11.2]);
    doThrow(game, sol, 'smoke'); const pending = !!game.player.pendingThrow; const cans1 = game.player.canisters;
    game.teleportPlayer(v3.copy(DEST)); for (let i = 0; i < 60; i++) W.step(DT);
    const fails: string[] = [];
    if (!pending || cans1 !== cans0 - 1) fails.push(`staging: the throw did not commit (pending ${pending}, cans ${cans0} → ${cans1})`);
    if (game.player.pendingThrow) fails.push('pendingThrow still set');
    if (game.items.items.some((it: any) => it.kind === 'smoke')) fails.push('a canister came out of his hand in the lobby');
    if (game.player.canisters !== cans0) fails.push(`canister not back on the belt (${cans0} → ${game.player.canisters})`);
    report('throw wind-up cancelled', fails, `canisters ${cans0} → ${cans1} → ${game.player.canisters}, live items ${game.items.items.length}`);
  }
  // --- restartEncounter mid-everything: a fresh Player, nothing of the old one's doings left on the level or the input
  {
    const W = await fresh(77); const { game, input } = W; const d = game.doors.byName('server');
    const spot = game.doors.workSpot(d, [17.6, 0, 10.9]); game.teleportPlayer([spot.pos[0], 0, spot.pos[2]]); W.step(DT);
    startPicking(game, d); input.keys.add('KeyF'); for (let i = 0; i < 120; i++) W.step(DT);
    game.restartEncounter(); for (let i = 0; i < 30; i++) W.step(DT); input.keys.delete('KeyF'); for (let i = 0; i < 5; i++) W.step(DT);
    const pl = game.player; const fails: string[] = [];
    if (pl.picking || pl.kick || pl.takedown || pl.dragging || pl.pendingThrow || pl.throwHeld || pl.fHeld || pl.doorHold !== 0 || pl.doorCracking || pl.crouch || pl.sprinting || pl.char.anim.lockpick || pl.char.anim.kicking) fails.push(`fresh player carries state: picking ${!!pl.picking} kick ${!!pl.kick} fHeld ${pl.fHeld} doorHold ${pl.doorHold} crouch ${pl.crouch} lockpick ${pl.char.anim.lockpick}`);
    if (d.pick !== 0 || !d.locked || d.picking) fails.push(`server door not restored: pick ${d.pick} locked ${d.locked} picking ${d.picking}`);
    if (v3.distXZ(pl.char.pos, game.level.playerSpawn) > 0.05) fails.push(`player not at the spawn: (${pl.char.pos[0].toFixed(2)}, ${pl.char.pos[2].toFixed(2)})`);
    report('restartEncounter() 2 s into a pick with F held', fails, `door pick ${d.pick}, player at (${pl.char.pos[0].toFixed(2)}, ${pl.char.pos[2].toFixed(2)})`);
  }
  console.log(`=== teleport-cancel: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

/** A man of the clearing pair dies mid-clear: what is left on the corpse, and what does the survivor do. */
async function deadScript() {
  console.log('\n=== dead-script: kill the point man during a live clear ===');
  Math.random = seeded(11);
  const W = await standUp(); const { game } = W;
  game.playerInvisible = true; game.godMode = true; game.teleportPlayer([4.9, 0, 23.2]);
  for (let i = 0; i < 30; i++) W.step(DT);
  game.clearNearestRoom();
  let killed = false;
  for (let f = 0; f < 60 * 60 && !killed; f++) { W.step(DT); const C = game.clearing; if (C && C.stage === 'drill' && C.drill?.phase === 'enter') { killGuard(game, C.a, [1, 0, 0], false); killed = true; console.log(`  killed ${game.level.routes[C.a.routeI].name} at ${game.time.toFixed(1)} s in phase enter`); } }
  for (let i = 0; i < 120; i++) W.step(DT);
  for (const gd of game.guards) console.log(`  ${game.level.routes[gd.routeI].name}: state ${gd.state} script ${gd.script ? JSON.stringify({ goal: gd.script.goal?.map((x: number) => +x.toFixed(2)), stance: gd.script.stance, upper: gd.script.upper }) : null} anim.stance ${gd.char.anim.stance}`);
  console.log(`  clearing: ${game.clearingSummary() || 'none'}; messages: ${game.messages.map((m: any) => m.text).join(' | ')}`);
}

/** Lockdown pair: the trailing man is taken down silently behind his leader, out of everyone's sight. Who reacts, when, and to what? */
async function silentRepair() {
  console.log('\n=== silent-repair: quiet takedown of the lockdown follower, unseen ===');
  Math.random = seeded(5);
  const W = await standUp(); const { game } = W;
  game.playerInvisible = true; game.godMode = true; game.teleportPlayer([4.9, 0, 23.2]);
  const lines: string[] = []; const origSay = game.say.bind(game); game.say = (gd: any, text: string, radio = false) => { lines.push(`${game.time.toFixed(2)}s ${game.level.routes[gd.routeI].name}${radio ? ' (radio)' : ''}: ${text}`); return origSay(gd, text, radio); };
  for (let i = 0; i < 30; i++) W.step(DT);
  game.escalate(); game.escalate();   // straight to lockdown: pair + post dealt
  if (game.clearing) { game.clearing.cancel(); game.clearing = null; }
  for (let i = 0; i < 60 * 20; i++) W.step(DT);   // let the pair form up and walk
  const follower = game.guards.find((g: any) => g.leader); const leader = follower?.leader; const postman = game.guards.find((g: any) => g.post);
  if (!follower) { console.log('  no pair formed?', game.escalationSummary()); return; }
  // isolate the pair in the break room (doors shut), leader facing north with his man behind him: nobody can have eyes on the follower
  leader.char.pos = [33, 0, 20]; leader.char.bodyYaw = leader.char.aimYaw = Math.PI; leader.path = []; leader.pathGoal = null; leader.wait = 3; leader.char.update(0);
  follower.char.pos = [33, 0, 21.6]; follower.char.bodyYaw = follower.char.aimYaw = Math.PI; follower.path = []; follower.pathGoal = null; follower.char.update(0);
  for (let i = 0; i < 20; i++) W.step(DT);
  console.log(`  before: ${game.escalationSummary()} — follower ${game.level.routes[follower.routeI].name} ${follower.state} at (${follower.char.pos[0].toFixed(1)}, ${follower.char.pos[2].toFixed(1)}), leader at (${leader.char.pos[0].toFixed(1)}, ${leader.char.pos[2].toFixed(1)}), post man at (${postman?.char.pos[0].toFixed(1)}, ${postman?.char.pos[2].toFixed(1)})`);
  const tKill = game.time; killGuard(game, follower, [Math.sin(follower.char.bodyYaw), 0, Math.cos(follower.char.bodyYaw)], true);
  lines.push(`${game.time.toFixed(2)}s --- follower taken down silently ---`);
  let foundAt = -1; const postStart = postman ? [postman.char.pos[0], postman.char.pos[2]] : null; let postLeftAt = -1;
  for (let i = 0; i < 60 * 40; i++) {
    W.step(DT);
    if (foundAt < 0 && follower.found) { foundAt = game.time; lines.push(`${game.time.toFixed(2)}s --- body discovered (Guard.found) ---`); }
    if (postman && postLeftAt < 0 && postStart && Math.hypot(postman.char.pos[0] - postStart[0], postman.char.pos[2] - postStart[1]) > 1.5) { postLeftAt = game.time; lines.push(`${game.time.toFixed(2)}s --- the post man has left his junction (now trailing ${postman.leader ? game.level.routes[postman.leader.routeI].name : 'nobody'}) ---`); }
  }
  console.log('  ' + lines.filter(l => parseFloat(l) >= tKill - 0.1).join('\n  '));
  console.log(`  after: ${game.escalationSummary()}; body found ${foundAt >= 0 ? `at +${(foundAt - tKill).toFixed(1)} s` : 'never (40 s)'}; re-pair ordered at +0.0 s if the "close up" line is stamped at the kill time`);
}

/** The takedown prompt needs 'behind him, within 1.05 m' — but is a wall between you and his back checked? Guard in the corridor with his back to the
 *  conference glazing / a 12 cm partition, player on the other side of it. */
async function takedownThroughWall() {
  console.log('\n=== takedown-through-wall: F on a guard whose back is against a thin wall, from the far side of it ===');
  const { buildInteractables, startTakedown } = await import(`${ROOT}/src/game/player.ts`);
  const cases: { guard: Vec3; yaw: number; player: Vec3; note: string }[] = [
    { guard: [12.0, 0, 10.45], yaw: 0, player: [12.0, 0, 9.6], note: 'corridor guard, back to the conference glazing (z=10); player inside the conference room' },
    { guard: [22.4, 0, 6.0], yaw: Math.PI / 2, player: [21.55, 0, 6.0], note: "guard in the manager's office, back to the server-room partition (x=22); player in the server room" },
    { guard: [30.5, 0, 12.65], yaw: 0, player: [30.5, 0, 11.75], note: 'guard just inside the break room, back to its north wall (z=12.2); player in the corridor' },
  ];
  for (const c of cases) {
    Math.random = seeded(9);
    const W = await standUp(); const { game, cam, canvas } = W;
    game.godMode = true; game.aiEnabled = true; game.player.visibility = 0.02;
    for (let i = 0; i < 5; i++) W.step(DT);
    const gd = game.guards[0]; for (const o of game.guards.slice(1)) o.hold = true;
    gd.char.pos = v3.copy(c.guard); gd.char.bodyYaw = gd.char.aimYaw = c.yaw; gd.hold = true; gd.char.update(0);
    game.teleportPlayer(v3.copy(c.player)); game.player.crouch = true; W.step(DT);
    const p = game.player.char.pos, gp = gd.char.pos;
    const wall = game.col.segmentBlocked([p[0], 1.0, p[2]], [gp[0], 1.0, gp[2]]);
    buildInteractables(game, cam, 0, 0, canvas.clientWidth, canvas.clientHeight);
    const it = game.interactables.find((x: any) => x.kind === 'guard' && x.guard === gd);
    console.log(`\n- ${c.note}\n  player (${p[0].toFixed(2)}, ${p[2].toFixed(2)}) ↔ guard (${gp[0].toFixed(2)}, ${gp[2].toFixed(2)}): ${v3.distXZ(p, gp).toFixed(2)} m, wall between: ${wall}; takedown marker: ${it ? `"${it.line1}" / "${it.line2}" inReach=${it.inReach}` : 'none'}`);
    if (it?.inReach) {
      startTakedown(game, gd);
      for (let i = 0; i < 60; i++) W.step(DT);
      const q = game.player.char.pos;
      console.log(`  → after F: guard ${gd.state}; player now at (${q[0].toFixed(2)}, ${q[2].toFixed(2)}) — ${game.col.segmentBlocked([c.player[0], 1.0, c.player[2]], [q[0], 1.0, q[2]]) ? 'ON THE OTHER SIDE of the wall he started behind' : 'same side as before'}; messages: ${game.messages.map((m: any) => m.text).join(' | ')}`);
    }
  }
}

/** A body found on a heightened floor raises the lockdown, which deals a room clear on the spot (raiseEscalation → planClears). The pair it is dealt to are the
 *  men reacting to that body. Clearing.muster gives them 15 s to be back on 'patrol'. Do they ever make it, i.e. does a body-triggered clear ever run? */
async function bodyLockdownClear() {
  console.log('\n=== body-lockdown-clear: does the clear dealt by a found body ever get past muster? ===');
  for (const seed of [21, 22, 23]) {
    Math.random = seeded(seed);
    const W = await standUp(); const { game } = W;
    game.playerInvisible = true; game.godMode = true; game.teleportPlayer([4.9, 0, 23.2]);
    for (let i = 0; i < 30; i++) W.step(DT);
    game.escalate();   // heightened
    for (let i = 0; i < 60 * 5; i++) W.step(DT);
    // kill the corridor man quietly where the lobby/break man's route will bring him past
    const victim = game.guards[0]; killGuard(game, victim, [1, 0, 0], true);
    const log: string[] = []; let lastSumm = ''; let started = -1; let ended = ''; let musterAt = -1;
    const origSay = game.say.bind(game); game.say = (gd: any, text: string, radio = false) => { log.push(`${game.time.toFixed(1)}s ${game.level.routes[gd.routeI].name}: ${text}`); return origSay(gd, text, radio); };
    for (let i = 0; i < 60 * 180 && !(ended && game.time > musterAt + 45); i++) {
      W.step(DT);
      const s = game.clearingSummary();
      if (s && musterAt < 0) musterAt = game.time;
      if (s.replace(/ · \d+ s$/, '') !== lastSumm.replace(/ · \d+ s$/, '')) { lastSumm = s; log.push(`${game.time.toFixed(1)}s [${s || 'no clearing'}] esc=${game.escalation} states=${game.guards.map((g: any) => g.state + (g.task ? '+task' : '')).join('/')}`); if (s.includes('approach') && started < 0) started = game.time; if (!s && musterAt >= 0) ended = 'gone'; }
    }
    console.log(`\n- seed ${seed}: clear dealt at ${musterAt >= 0 ? musterAt.toFixed(1) + ' s' : 'never'}; ${started >= 0 ? `approach began at ${started.toFixed(1)} s (+${(started - musterAt).toFixed(1)} s)` : 'NEVER got past muster'}`);
    console.log('  ' + log.slice(0, 24).join('\n  '));
  }
}

/** The found-body script (guards.ts bodyAftermath): a man taken down quietly on the corridor where a colleague's loop brings him past. Who says what and how far
 *  from the body, does anybody stand on the corpse, does a generic 'clear here' / 'must have been nothing' ever get said within 4 m of it, and where does it
 *  leave the floor — on a calm floor (→ heightened), a heightened one (→ lockdown: the pair musters from widening out and clears the three rooms nearest him),
 *  and mid-clear (the pair walking to a room comes on the third man's body: they drop the room, run the script, and are re-dealt the rooms round the body). */
async function corpse() {
  console.log('\n=== corpse: a found body — the lines near it, lingering on it, and what the floor does next ===');
  const GENERIC = ['clear here — staying on it', 'must have been nothing'];
  type Prep = { label: string; seed: number; prep: (game: any, W: any) => any };
  const cases: Prep[] = [
    { label: 'calm floor: the corridor man taken down at 5 s, the lobby/break man walks onto him', seed: 31, prep: (game, W) => { for (let i = 0; i < 60 * 5; i++) W.step(DT); const v = game.guards[0]; killGuard(game, v, [1, 0, 0], true); return v; } },
    { label: 'heightened floor: same takedown → lockdown on discovery, pair + post dealt, the clear planned round the body', seed: 32, prep: (game, W) => { game.escalate(); for (let i = 0; i < 60 * 5; i++) W.step(DT); const v = game.guards[0]; killGuard(game, v, [1, 0, 0], true); return v; } },
    ...(['corridor', 'room'] as const).map((where, k) => ({ label: where === 'corridor' ? 'lockdown, mid-clear: the post man dies quietly on the corridor a few metres ahead of the pair walking to their first room' : 'lockdown, mid-clear: the post man lies dead INSIDE the room the pair is about to take (found as they enter)', seed: 33 + k, prep: (game: any, W: any) => {
      game.escalate(); game.escalate(); if (game.clearing) { game.clearing.cancel(); game.clearing = null; }
      for (let i = 0; i < 60 * 12; i++) W.step(DT);   // pair formed and walking, post man on his junction
      game.teleportPlayer([20.5, 0, 11.3]); game.clearNearestRoom(); game.teleportPlayer([4.9, 0, 23.2]);   // fix mid-corridor: server room + manager's office
      let third: any = null; for (let i = 0; i < 60 * 30 && !third; i++) { W.step(DT); const C = game.clearing; if (C && C.stage === 'approach') third = game.guards.find((x: any) => x.state !== 'dead' && x !== C.a && x !== C.b) ?? null; }
      const C = game.clearing; if (!third || !C) { console.log('  (no approach / no third man)'); return null; }
      const a = C.a; let q: Vec3; let dir: Vec3;
      if (where === 'corridor') {   // three and a half metres ahead of the point man along the corridor toward the stack, kept inside the corridor strip
        const sx = Math.sign(C.cur.stack[0][0] - a.char.pos[0]) || 1; dir = [sx, 0, 0]; q = [a.char.pos[0] + sx * 3.5, 0, Math.min(11.8, Math.max(10.6, a.char.pos[2]))];
      } else { const B = C.cur.bounds; dir = [0, 0, -1]; q = [(B.x0 + B.x1) / 2 - 1.2, 0, B.z1 - 2.2]; }   // a couple of metres inside the door, off centre
      if (game.col.nav.isBlocked(q[0], q[2])) { console.log(`  (spot (${q[0].toFixed(1)}, ${q[2].toFixed(1)}) blocked)`); return null; }
      third.post = null; third.char.pos = q; third.char.vel = [0, 0, 0]; third.char.bodyYaw = Math.atan2(dir[0], dir[2]); third.path = []; third.pathGoal = null; third.char.update(0); third.char.update(DT);   // (two pose bakes at the new spot: the ragdoll seeds its momentum from the last two, and a 10 m 'stride' would fling the corpse across the corridor)
      killGuard(game, third, dir, true); return third;
    } })),
  ];
  for (const c of cases) {
    Math.random = seeded(c.seed);
    const W = await standUp(); const { game } = W;
    game.playerInvisible = true; game.godMode = true; game.teleportPlayer([4.9, 0, 23.2]);
    const name = (gd: any) => game.level.routes[gd.routeI].name;
    for (let i = 0; i < 30; i++) W.step(DT);
    const victim = c.prep(game, W);
    if (!victim) continue;
    const tKill = game.time;
    const lines: { t: number; who: string; text: string; d: number; radio: boolean }[] = [];
    const origSay = game.say.bind(game); game.say = (gd: any, text: string, radio = false) => { lines.push({ t: game.time, who: name(gd), text, d: v3.distXZ(gd.char.pos, bodyPos(victim)), radio }); return origSay(gd, text, radio); };
    let foundAt = -1; const linger = new Map<any, { cur: number; max: number }>(); const summ: string[] = []; let lastS = ''; const stages = new Map<any, string>();
    for (let f = 0; f < 60 * 160; f++) {
      W.step(DT);
      if (foundAt < 0 && victim.found) foundAt = game.time;
      if (foundAt > 0 && game.time > foundAt + 3) for (const gd of game.guards) {
        if (gd.state === 'dead') continue;
        const L = linger.get(gd) ?? { cur: 0, max: 0 }; const d = v3.distXZ(gd.char.pos, bodyPos(victim));
        L.cur = d < 1.5 ? L.cur + DT : 0; L.max = Math.max(L.max, L.cur); linger.set(gd, L);
      }
      for (const gd of game.guards) { const st = gd.state === 'dead' ? 'dead' : gd.bodyDuty ? `${gd.bodyDuty.role}:${gd.bodyDuty.stage}` : gd.script ? 'scripted' : gd.state; if (stages.get(gd) !== st) { stages.set(gd, st); if (foundAt > 0) summ.push(`${(game.time - foundAt).toFixed(1)}s ${name(gd)} → ${st}`); } }
      const s = (game.clearingSummary() || '').replace(/ · \d+ s$/, ''); if (s !== lastS) { lastS = s; summ.push(`${foundAt > 0 ? (game.time - foundAt).toFixed(1) : '-' + (game.time - tKill).toFixed(1)}s [${s || 'no clearing'}] esc=${game.escalation}`); }
      if (foundAt > 0 && game.time > foundAt + 110) break;
    }
    console.log(`\n- ${c.label} (seed ${c.seed}): ${name(victim)} killed at ${tKill.toFixed(1)} s, found ${foundAt > 0 ? `at +${(foundAt - tKill).toFixed(1)} s` : 'NEVER'}; alarm now ${game.escalationSummary()}`);
    const after = lines.filter(l => l.t >= (foundAt > 0 ? foundAt : tKill) - 0.01);
    console.log('  lines (time since found · who · metres from the body):\n    ' + after.map(l => `+${(l.t - foundAt).toFixed(1)}s ${l.who}${l.radio ? ' (radio)' : ''} @${l.d.toFixed(1)} m: ${l.text}`).join('\n    '));
    const genericNear = after.filter(l => GENERIC.includes(l.text) && l.d < 4);
    const christ = after.filter(l => l.text.startsWith('…christ')).length, closing = after.filter(l => l.text.startsWith('no sign of him')).length;
    console.log(`  generic 'clear here'/'nothing' within 4 m of the body: ${genericNear.length}${genericNear.length ? ' ← FAIL' : ''} · '…christ' ×${christ} · closing line ×${closing} · lines total ${after.length} in ${(game.time - foundAt).toFixed(0)} s`);
    console.log('  longest continuous stay within 1.5 m of the body after +3 s: ' + [...linger.entries()].map(([gd, L]) => `${name(gd)} ${L.max.toFixed(1)} s`).join(', '));
    console.log('  duty / clearing timeline:\n    ' + summ.join('\n    '));
  }
}

/** Doors that give the intruder away, met mid-clear (squad.ts Clearing.readDoors + guards.ts planClears): the server door picked, a clear forced from the corridor.
 *  (a) the fix at the picked door itself; (b) the fix further east so the manager's office ranks first and the pair, coming from the west, passes the picked server
 *  door on the way — it should jump the queue with the line; (c) the storage door kicked in and the fix at the far west end — the kicked room ranks first at plan time. */
async function pickedDoor() {
  console.log('\n=== picked-door: tampered doors re-rank the clear and get remarked on ===');
  const cases: { label: string; seed: number; setup: (game: any) => void; fix: Vec3 }[] = [
    { label: 'server door picked, fix right at it (18, 11)', seed: 41, setup: (game) => { game.doors.byName('server').locked = false; }, fix: [18.0, 0, 11.0] },
    { label: 'server door picked, fix east of it (22.5, 11.3): manager\'s office nearest, the pair comes past the server door from the west', seed: 42, setup: (game) => { game.doors.byName('server').locked = false; }, fix: [22.5, 0, 11.3] },
    { label: 'storage door kicked in (lockBroken), fix at the west end (8, 11.2)', seed: 43, setup: (game) => { const d = game.doors.byName('storage'); d.locked = false; d.lockBroken = true; }, fix: [8.0, 0, 11.2] },
  ];
  for (const c of cases) {
    Math.random = seeded(c.seed);
    const W = await standUp(); const { game } = W;
    game.playerInvisible = true; game.godMode = true;
    const name = (gd: any) => game.level.routes[gd.routeI].name;
    for (let i = 0; i < 30; i++) W.step(DT);
    c.setup(game);
    const lines: string[] = []; const origSay = game.say.bind(game); game.say = (gd: any, text: string, radio = false) => { lines.push(`${game.time.toFixed(1)}s ${name(gd)} @(${gd.char.pos[0].toFixed(1)}, ${gd.char.pos[2].toFixed(1)}): ${text}`); return origSay(gd, text, radio); };
    game.teleportPlayer(v3.copy(c.fix)); game.clearNearestRoom(); game.teleportPlayer([4.9, 0, 23.2]);
    const planned = game.clearing ? game.clearing.rooms.map((r: any) => r[0].room).join(' ▸ ') : 'none';
    const drilled: string[] = []; let lastQ = ''; const queues: string[] = [];
    for (let f = 0; f < 60 * 150 && game.clearing; f++) {
      W.step(DT);
      const C = game.clearing; if (!C) break;
      if (C.stage === 'drill' && C.drill && (drilled[drilled.length - 1] !== C.cur.room)) drilled.push(C.cur.room);
      const q = game.clearQueueSummary(); if (q !== lastQ) { lastQ = q; if (!/\d+ s ago/.test(q) || queues.length < 12) queues.push(`${game.time.toFixed(1)}s ${q}`); }
    }
    console.log(`\n- ${c.label} (seed ${c.seed})\n  planned: ${planned}\n  drilled in order: ${drilled.join(' ▸ ') || 'none'}\n  door lines: ${lines.filter(l => /forced|picked|locking/.test(l)).join(' | ') || 'NONE'}`);
    console.log('  queue over time:\n    ' + queues.slice(0, 10).join('\n    '));
    console.log('  all lines:\n    ' + lines.join('\n    '));
  }
}

/** Knowledge rule, follower side: the lockdown leader is taken down quietly where the man trailing him does not see it happen. (a) the follower has dropped eight
 *  metres back and is looking the other way: he should close up to his station behind the body and SEE it (bodyInView → 'man down!') within seconds. (b) the
 *  leader dies with his back a step off a wall, so 'behind him' is in the wall and the follower's station collapses onto the body itself — standing on a corpse
 *  is exactly where the view test is least sure of itself; the still-leader fallback (turn and look at a leader who has not moved for 6 s) must close it. */
async function followerCorpse() {
  console.log('\n=== follower-corpse: the leader dies quietly where his follower is not looking ===');
  for (const wall of [false, true]) {
    Math.random = seeded(51);
    const W = await standUp(); const { game } = W;
    game.playerInvisible = true; game.godMode = true; game.teleportPlayer([4.9, 0, 23.2]);
    const name = (gd: any) => game.level.routes[gd.routeI].name;
    const lines: string[] = []; const origSay = game.say.bind(game); game.say = (gd: any, text: string, radio = false) => { lines.push(`${game.time.toFixed(2)}s ${name(gd)}${radio ? ' (radio)' : ''}: ${text}`); return origSay(gd, text, radio); };
    for (let i = 0; i < 30; i++) W.step(DT);
    game.escalate(); game.escalate(); if (game.clearing) { game.clearing.cancel(); game.clearing = null; }
    for (let i = 0; i < 60 * 25; i++) W.step(DT);   // pair formed up and walking the leader's route
    const follower = game.guards.find((x: any) => x.leader && x.state !== 'dead'); if (!follower) { console.log('  no pair?', game.escalationSummary()); continue; }
    const leader = follower.leader; const nav = game.col.nav;
    const place = (gd: any, p: Vec3, yaw: number) => { gd.char.pos = v3.copy(p); gd.char.vel = [0, 0, 0]; gd.char.bodyYaw = gd.char.aimYaw = yaw; gd.path = []; gd.pathGoal = null; gd.speed = 0; gd.char.update(0); gd.char.update(DT); };
    if (wall) { place(leader, [33.6, 0, 13.1], 0); place(follower, [30.5, 0, 16.5], 0); }   // leader just inside the break room, back to its north wall, facing into the room; follower by the west door looking south — his back to him
    else {   // follower eight metres back up the leader's track (the first free, walkable spot), turned about
      const lp = leader.char.pos, f = leader.char.forward(); let spot: Vec3 | null = null;
      for (const k of [8, 7, 6, 5, 4]) { const q: Vec3 = [lp[0] - f[0] * k, 0, lp[2] - f[2] * k]; if (!nav.isBlocked(q[0], q[2]) && nav.findPath(q, lp)) { spot = q; break; } }
      place(follower, spot ?? follower.char.pos, leader.char.bodyYaw + Math.PI);
    }
    W.step(DT);
    const gap0 = v3.distXZ(follower.char.pos, leader.char.pos);
    killGuard(game, leader, [Math.sin(leader.char.bodyYaw), 0, Math.cos(leader.char.bodyYaw)], true);
    const tKill = game.time; lines.push(`${tKill.toFixed(2)}s --- ${name(leader)} (leader) taken down quietly ${gap0.toFixed(1)} m from ${name(follower)}, who is looking the other way${wall ? '; the body lies a step off the wall it had its back to' : ''} ---`);
    let foundAt = -1, redealt = -1; const trace: string[] = [];
    for (let f = 0; f < 60 * 40; f++) {
      W.step(DT);
      if (foundAt < 0 && leader.found) foundAt = game.time;
      if (redealt < 0 && follower.leader !== leader) redealt = game.time;
      if (f % 30 === 0 && game.time - tKill < 14) trace.push(`+${(game.time - tKill).toFixed(1)}s ${name(follower)} ${follower.state}${follower.bodyDuty ? '/' + follower.bodyDuty.stage : ''} at (${follower.char.pos[0].toFixed(1)}, ${follower.char.pos[2].toFixed(1)}) gap ${v3.distXZ(follower.char.pos, leader.char.pos).toFixed(2)} m yaw ${follower.char.bodyYaw.toFixed(2)} (to body ${Math.atan2(leader.char.pos[0] - follower.char.pos[0], leader.char.pos[2] - follower.char.pos[2]).toFixed(2)}) stillT ${follower.leaderStillT.toFixed(1)} leader=${follower.leader ? name(follower.leader) + (follower.leader.state === 'dead' ? '(dead)' : '') : 'none'}`);
    }
    console.log(`\n- ${wall ? '(b) back to the wall: the station collapses onto the body' : '(a) eight metres back, turned about'}: body found ${foundAt > 0 ? `+${(foundAt - tKill).toFixed(1)} s after the kill` : 'NEVER in 40 s ← FAIL'}; follower re-dealt off the corpse ${redealt > 0 ? `at +${(redealt - tKill).toFixed(1)} s` : 'never'}; now: ${game.escalationSummary()}`);
    console.log('  ' + trace.join('\n  '));
    console.log('  ' + lines.filter(l => parseFloat(l) >= tKill - 0.1).slice(0, 14).join('\n  '));
  }
  {   // (c) the fallback on its own: a living leader who simply stands (hold) — his follower, planted at his shoulder, should turn and look AT him once he has been still 6 s
    Math.random = seeded(52);
    const W = await standUp(); const { game } = W;
    game.playerInvisible = true; game.godMode = true; game.teleportPlayer([4.9, 0, 23.2]);
    for (let i = 0; i < 30; i++) W.step(DT);
    game.escalate(); game.escalate(); if (game.clearing) { game.clearing.cancel(); game.clearing = null; }
    for (let i = 0; i < 60 * 20; i++) W.step(DT);
    const follower = game.guards.find((x: any) => x.leader && x.state !== 'dead'); if (!follower) { console.log('  (c) no pair?'); return; }
    const leader = follower.leader; leader.hold = true;
    const wrap = (a: number) => Math.atan2(Math.sin(a), Math.cos(a));
    const sample: string[] = [];
    for (let f = 0; f <= 60 * 12; f++) {
      W.step(DT);
      if (f % 120 === 0) { const toL = Math.atan2(leader.char.pos[0] - follower.char.pos[0], leader.char.pos[2] - follower.char.pos[2]); sample.push(`+${(f / 60).toFixed(0)}s gap ${v3.distXZ(follower.char.pos, leader.char.pos).toFixed(2)} stillT ${follower.leaderStillT.toFixed(1)} |yaw − to leader| ${Math.abs(wrap(follower.char.bodyYaw - toL)).toFixed(2)} |yaw − (leader yaw + 0.45)| ${Math.abs(wrap(follower.char.bodyYaw - leader.char.bodyYaw - 0.45)).toFixed(2)}`); }
    }
    const toL = Math.atan2(leader.char.pos[0] - follower.char.pos[0], leader.char.pos[2] - follower.char.pos[2]);
    console.log(`\n- (c) living leader held still: after 12 s the follower faces ${Math.abs(wrap(follower.char.bodyYaw - toL)) < 0.25 ? 'HIM (fallback fired)' : 'elsewhere ← fallback did not fire'}\n  ` + sample.join('\n  '));
  }
}

/** The per-lockdown ledger of cleared rooms (Game.clearedAt, guards.ts planClears): clear the two rooms nearest a corridor fix to the end, then ask again from the
 *  same spot inside the minute (both should be skipped for the next nearest), then ask with the fix INSIDE the first room (it comes back: a new stimulus from in there). */
async function reclear() {
  console.log('\n=== reclear: rooms called clear are not sent round again within a minute unless the fix is inside ===');
  Math.random = seeded(61);
  const W = await standUp(); const { game } = W;
  game.playerInvisible = true; game.godMode = true;
  for (let i = 0; i < 30; i++) W.step(DT);
  const runClear = (fix: Vec3, maxS: number) => {
    game.teleportPlayer(v3.copy(fix)); game.clearNearestRoom(); game.teleportPlayer([4.9, 0, 23.2]);
    const planned = game.clearing ? game.clearing.rooms.map((r: any) => r[0].room).join(' ▸ ') : '(nothing planned)';
    const t0 = game.time; while (game.clearing && game.time - t0 < maxS) W.step(DT);
    return `${planned}  [${game.clearing ? 'still running' : 'finished'} after ${(game.time - t0).toFixed(0)} s]  ledger: ${game.clearQueueSummary() || '—'}`;
  };
  console.log('  1) fix on the corridor at (12, 11.2):            ' + runClear([12, 0, 11.2], 160));
  console.log('  2) same fix again, straight after:               ' + runClear([12, 0, 11.2], 1));
  if (game.clearing) { game.clearing.cancel(); game.clearing = null; }
  console.log('  3) fix INSIDE the conference room (8, 7):        ' + runClear([8, 0, 7], 1));
  if (game.clearing) { game.clearing.cancel(); game.clearing = null; }
  for (let i = 0; i < 60 * 65; i++) W.step(DT);
  console.log('  4) corridor fix again, 65 s later:               ' + runClear([12, 0, 11.2], 1));
  console.log(`  messages: ${game.messages.map((m: any) => m.text).join(' | ')}`);
}

if (which === 'world-edge' || which === 'player' || which === 'all') await worldEdge();
if (which === 'teleport-cancel' || which === 'player' || which === 'all') await teleportCancel();
if (which === 'crush' || which === 'player') await crush();   // ('all' runs it below, in its old place)
if (which === 'corpse' || which === 'all') await corpse();
if (which === 'picked-door' || which === 'all') await pickedDoor();
if (which === 'follower-corpse' || which === 'all') await followerCorpse();
if (which === 'reclear' || which === 'all') await reclear();
if (which === 'takedown-wall' || which === 'all') await takedownThroughWall();
if (which === 'body-clear' || which === 'all') await bodyLockdownClear();
if (which === 'leaf-wedge' || which === 'all') await leafWedge();
if (which === 'silent-repair' || which === 'all') await silentRepair();
if (which === 'all') await crush();
if (which === 'dead-script' || which === 'all') await deadScript();
