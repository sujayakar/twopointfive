// Directed headless probes for things the soak surfaced — each stands the full game up (headless.ts), stages one situation exactly, steps it, and prints
// what happened, so a finding comes with a deterministic repro instead of a seed + frame.
//   bun run tools/qa/probes.ts [leaf-wedge|crush|dead-script|silent-repair|takedown-wall|body-clear|corpse|picked-door|follower-corpse|reclear|world-edge|teleport-cancel|
//                               wall-clip|wall-clip-speed|wall-clip-doors|wall-clip-shove|wall-clip-moves|wall-clip-ends|
//                               wall-snap|wall-snap-engage|wall-snap-slide|wall-snap-release|wall-push|wall-snap-sweep|player|
//                               aim-door [n]|aim-door-chase|aim-door-sweep|far-refix|door-rules|doors|
//                               muzzle|missing|motion|ai-pass|darkness|pieing|ai-pass-2|ko-kinds|ko-rollcall|witness|ko-witness|wall-shelf-pop|
//                               dialogue-none|dialogue-sawhurt|dialogue-heard|dialogue-intel|dialogue-lint|dialogue-seed|dialogue|
//                               grab-gate|grab-walk|grab-rollcall|grab-talk|grab-choke|grab-release|grab-auto|grab|
//                               gunhold-variant|gunhold-fire|gunhold-whip|gunhold-dialogue|gunhold|
//                               standoff-see|standoff-flank|standoff-ff|standoff-end|standoff-lost|standoff-door|standoff-tour|standoff|all] [--verbose]
//   ('player' = the player-side PASS/FAIL set: the wall-clip family minus its 4-minute doors sweep, wall-snap minus its 2-minute grid sweep, world-edge,
//    teleport-cancel, crush — ≈ 5 min; 'wall-clip' = all five wall-clip probes ≈ 8 min: every way the player's 0.42 m circle could end up inside / beyond a
//    static, brute-forced; 'wall-snap' = the back-to-the-wall (Q) family incl. the grid sweep ≈ 3 min; 'doors' = the alert-man-at-a-doorway PASS/FAIL set:
//    aim-door, aim-door-chase, aim-door-sweep, far-refix, door-rules)
import { standUp, ROOT } from './headless.ts';
const IK = 'Space';   // the game's interact key (src/game/consts.ts INTERACT_KEY)
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
const { bodyPos, resetChase } = await import(`${ROOT}/src/game/guards.ts`);
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
    startPicking(game, d); input.keys.add(IK);
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
    input.keys.delete(IK); W.step(DT); W.step(DT);
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
    startPicking(game, d); input.keys.add(IK); for (let i = 0; i < 120; i++) W.step(DT);
    game.restartEncounter(); for (let i = 0; i < 30; i++) W.step(DT); input.keys.delete(IK); for (let i = 0; i < 5; i++) W.step(DT);
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

// ---------------------------------------------------------------- wall-clip: every way the player's circle could end up inside / beyond static geometry
const { BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);
const PR = 0.42;   // player.ts BODY_R

/** Per-frame watcher of the player's circle against the statics: `pen` = what one exact push-out would still move him at the end of the frame (what gets drawn),
 *  `inside` = his CENTRE lies in a static footprint within the collision band, `crossed` = he went from one side of `wall`'s mid-plane to the other while
 *  level with it (through it, not round its end), `through` = a static now separates him (knee AND hip height) from where he was a frame ago. */
function clipWatch(game: any, wall: { b: any; n: [number, number]; t: [number, number]; L: number } | null = null) {
  const col = game.col; const boxes = col.boxes; const buf = new Int32Array(64);
  const S = { maxPen: 0, maxPenAt: '', inside: '', crossed: '', through: '', frames: 0, worstStep: 0, cornerCut: 0 };
  let prev: Vec3 | null = null;
  const distOf = (p: Vec3) => wall ? (p[0] - wall.b.c[0]) * wall.n[0] + (p[2] - wall.b.c[2]) * wall.n[1] : 0;   // signed, from the mid-plane
  const alongOf = (p: Vec3) => wall ? (p[0] - wall.b.c[0]) * wall.t[0] + (p[2] - wall.b.c[2]) * wall.t[1] : 0;
  return {
    S,
    reset() { prev = null; },
    frame(tag = '') {
      const pl = game.player; const p: Vec3 = pl.char.pos; const y1 = pl.crouch ? 1.0 : 1.7; S.frames++;
      const q = v3.copy(p); col.collideCircle(q, PR, 0.2, y1, 1); const pen = v3.distXZ(q, p);
      if (pen > S.maxPen) { S.maxPen = pen; S.maxPenAt = `${game.time.toFixed(2)}s (${p[0].toFixed(2)}, ${p[2].toFixed(2)})${tag}`; }
      if (!S.inside) { const n = col.gather(p[0] - 0.01, p[2] - 0.01, p[0] + 0.01, p[2] + 0.01, 0.2, y1, buf); for (let k = 0; k < n; k++) { const b = boxes[buf[k]]; const c = Math.cos(b.yaw), s = Math.sin(b.yaw); const dx = p[0] - b.c[0], dz = p[2] - b.c[2]; const lx = c * dx - s * dz, lz = s * dx + c * dz; if (Math.abs(lx) < b.h[0] - 1e-3 && Math.abs(lz) < b.h[2] - 1e-3) { S.inside = `${game.time.toFixed(2)}s centre INSIDE ${b.name ?? 'box'} c(${b.c[0].toFixed(2)},${b.c[1].toFixed(2)},${b.c[2].toFixed(2)}) h(${b.h[0]},${b.h[1]},${b.h[2]}) at (${p[0].toFixed(2)}, ${p[2].toFixed(2)})${tag}`; break; } } }
      if (wall && prev && !S.crossed) {   // the centre went from one side of the mid-plane to the other between two frames: where along the wall did it pass? inside its length = through it; within a body radius past an end = the corner cut between frames (counted, not failed)
        const d0 = distOf(prev), d1 = distOf(p);
        if (d0 * d1 < 0) { const t = d0 / (d0 - d1); const a = alongOf(prev) + (alongOf(p) - alongOf(prev)) * t; if (Math.abs(a) < wall.L + 0.1) S.crossed = `${game.time.toFixed(2)}s CROSSED the mid-plane at along ${a.toFixed(2)} / ±${wall.L.toFixed(2)}: (${prev[0].toFixed(2)}, ${prev[2].toFixed(2)}) → (${p[0].toFixed(2)}, ${p[2].toFixed(2)})${tag}`; else if (Math.abs(a) < wall.L + PR) S.cornerCut++; }
      }
      if (prev) { const st = v3.distXZ(prev, p); S.worstStep = Math.max(S.worstStep, st); if (!S.through && st > 1e-4 && col.segmentBlocked([prev[0], 0.45, prev[2]], [p[0], 0.45, p[2]], 0) && col.segmentBlocked([prev[0], 0.9, prev[2]], [p[0], 0.9, p[2]], 0) && !col.segmentBlockedDynamic([prev[0], 0.45, prev[2]], [p[0], 0.45, p[2]])) S.through = `${game.time.toFixed(2)}s a static between (${prev[0].toFixed(2)}, ${prev[2].toFixed(2)}) and (${p[0].toFixed(2)}, ${p[2].toFixed(2)}) one frame apart${tag}`; }
      prev = v3.copy(p);
    },
    bad(penTol = 0.02) { return !!(S.inside || S.crossed || S.through || S.maxPen > penTol); },
    line(penTol = 0.02) { const f: string[] = []; if (S.crossed) f.push(S.crossed); if (S.through) f.push('THROUGH: ' + S.through); if (S.inside) f.push(S.inside); if (S.maxPen > penTol) f.push(`max frame-end penetration ${S.maxPen.toFixed(3)} m at ${S.maxPenAt}`); return f.join(' · '); },
  };
}

/** a fresh headless world with the guards parked on the grass outside (held, and deaf: AI off — a kicked door is heard floor-wide and 'hold' does not hold an
 *  alert man; scripted men still walk, squad.ts runs them regardless), the player a ghost in god mode */
async function clipWorld(seed = 90) {
  Math.random = seeded(seed);
  const W = await standUp(); const { game } = W;
  game.playerInvisible = true; game.godMode = true; game.aiEnabled = false;
  for (let i = 0; i < 5; i++) W.step(DT);
  game.guards.forEach((gd: any, i: number) => { gd.hold = true; gd.char.pos = [38.5, 0, 1.5 + i * 1.2]; gd.char.vel = [0, 0, 0]; gd.path = []; gd.pathGoal = null; gd.char.update(0); });
  return W;
}

/** (1) tunnelling by speed: every thin static in the collision band (partitions, exterior walls, sills, door jambs, cubicle panels, desk panels…), both long faces,
 *  a sample every metre: stand him 0.6 m off the face and drive him straight (and at 45°) into it for 1.5 s at walk / sprint / crouch, at 60 / 30 / 20 Hz. */
async function wallClipSpeed(opts: { verbose?: boolean } = {}) {
  console.log('\n=== wall-clip/speed: driven straight (and diagonally) into every thin static at walk / sprint / crouch, 60 / 30 / 20 Hz ===');
  const W = await clipWorld(); const { game, input } = W; const col = game.col; const boxes: any[] = col.boxes;
  const walls: { b: any; i: number; n: [number, number]; t: [number, number]; L: number; h: number }[] = [];
  boxes.forEach((b: any, i: number) => {
    if (b.flags & (BoxFlag.NoShadow | BoxFlag.Dynamic)) return;
    const thinX = b.h[0] <= b.h[2]; const h = thinX ? b.h[0] : b.h[2], L = thinX ? b.h[2] : b.h[0];
    if (h > 0.2 || L < 0.25) return;
    if (b.c[1] - b.h[1] > 0.9 || b.c[1] + b.h[1] < 0.3) return;   // must sit in the crouched band (0.2‥1.0) to be a mover's obstacle at all
    if (b.c[1] + b.h[1] > 3.2 && b.c[1] - b.h[1] > 0.5) return;
    const c = Math.cos(b.yaw), s = Math.sin(b.yaw); const ax: [number, number] = [c, -s], az: [number, number] = [s, c];   // box local X / Z in world XZ
    walls.push({ b, i, n: thinX ? ax : az, t: thinX ? az : ax, L, h });
  });
  type Mode = { name: string; hz: number; sprint?: boolean; crouch?: boolean; diag?: number };
  const modes: Mode[] = [
    { name: 'walk@60', hz: 60 }, { name: 'sprint@60', hz: 60, sprint: true }, { name: 'crouch@60', hz: 60, crouch: true },
    { name: 'sprint@30', hz: 30, sprint: true }, { name: 'walk@20', hz: 20 }, { name: 'sprint@20', hz: 20, sprint: true }, { name: 'crouch@20', hz: 20, crouch: true },
    { name: 'diag+45 sprint@60', hz: 60, sprint: true, diag: 1 }, { name: 'diag+45 sprint@20', hz: 20, sprint: true, diag: 1 }, { name: 'diag−45 sprint@20', hz: 20, sprint: true, diag: -1 },
  ];
  const perMode = new Map<string, { runs: number; bad: number }>(); const offenders: string[] = []; let runs = 0, skipped = 0, faces = 0;
  for (const w of walls) for (const face of [1, -1]) {
    const samples: number[] = []; if (w.L <= 0.75) samples.push(0); else for (let a = -w.L + 0.5; a <= w.L - 0.5 + 1e-6; a += 1.0) samples.push(a);
    if (samples.length > 8) { const keep: number[] = []; for (let k = 0; k < 8; k++) keep.push(samples[Math.round(k * (samples.length - 1) / 7)]); samples.length = 0; samples.push(...keep); }
    for (const a of samples) {
      const fx = w.b.c[0] + w.t[0] * a + w.n[0] * (w.h + 0.6) * face, fz = w.b.c[2] + w.t[1] * a + w.n[1] * (w.h + 0.6) * face;
      if (fx < 0.6 || fx > 39.4 || fz < 0.6 || fz > 27.4) { skipped++; continue; }
      { const q: Vec3 = [fx, 0, fz]; col.collideCircle(q, PR, 0.2, 1.7, 3); if (v3.distXZ(q, [fx, 0, fz]) > 0.02) { skipped++; continue; } }   // furniture where he would start: not a fair run
      faces++;
      for (const m of modes) {
        const dt = 1 / m.hz; const dir: [number, number] = [-w.n[0] * face, -w.n[1] * face];   // into the wall
        let gx = dir[0], gz = dir[1]; if (m.diag) { gx = (dir[0] + w.t[0] * m.diag) * Math.SQRT1_2; gz = (dir[1] + w.t[1] * m.diag) * Math.SQRT1_2; }   // (n ⊥ t: half into the wall, half along it)
        game.teleportPlayer([fx, 0, fz]); game.player.crouch = !!m.crouch; game.player.speedSm = 0;
        if (v3.distXZ(game.player.char.pos, [fx, 0, fz]) > 0.05) { skipped++; continue; }
        game.puppet = { goal: [fx + gx * 4, 0, fz + gz * 4], aim: null, crouch: !!m.crouch }; if (m.sprint) input.keys.add('ShiftLeft'); else input.keys.delete('ShiftLeft');
        const watch = clipWatch(game, w); watch.frame();
        for (let f = 0; f < Math.round(1.5 * m.hz); f++) { W.step(dt); watch.frame(); }
        game.puppet = null; input.keys.delete('ShiftLeft');
        runs++; const pm = perMode.get(m.name) ?? { runs: 0, bad: 0 }; pm.runs++;
        if (watch.bad()) { pm.bad++; offenders.push(`${(w.b.name ?? 'box') + '#' + w.i} c(${w.b.c[0].toFixed(2)},${w.b.c[2].toFixed(2)}) ${(2 * w.h).toFixed(2)} thick, face ${face > 0 ? '+' : '−'} along ${a.toFixed(1)} · ${m.name}: ${watch.line()}`); }
        perMode.set(m.name, pm);
      }
    }
  }
  console.log(`  ${walls.length} thin statics, ${faces} start spots (${skipped} skipped: off the world or in furniture), ${runs} runs`);
  console.log('  mode                 runs   offenders');
  for (const [k, v] of perMode) console.log(`  ${k.padEnd(20)} ${String(v.runs).padStart(5)}   ${String(v.bad).padStart(5)}`);
  if (offenders.length) { console.log(`  offenders (${offenders.length}):`); for (const o of offenders.slice(0, opts.verbose ? 1000 : 60)) console.log('   ✗ ' + o); if (!opts.verbose && offenders.length > 60) console.log(`   … ${offenders.length - 60} more (--verbose)`); }
  const pass = offenders.length === 0;
  console.log(`=== wall-clip/speed: ${pass ? 'PASS' : `FAIL (${offenders.length} of ${runs} runs)`} ===`);
  return pass;
}

/** unit direction from p INTO the nearest static face within a metre at shin height (what 'pressing yourself against the wall' means there), or null on open floor */
function intoNearestWall(col: any, p: Vec3): [number, number] | null {
  let best: [number, number] | null = null, bt = 1.0;
  for (let k = 0; k < 16; k++) { const a = k * Math.PI / 8; const d: Vec3 = [Math.sin(a), 0, Math.cos(a)]; const hit = col.raycast([p[0], 0.45, p[2]], d, bt); if (hit && hit.index < 1e6 && hit.t < bt) { bt = hit.t; best = [d[0], d[2]]; } }
  return best;
}

/** (2) pushed by others — door leaves. Every door, a grid of standing spots within the leaf's reach on both faces of its wall (only spots the statics leave
 *  him, most of them against a wall or a jamb), and four things that move the leaf hard: kicked in from either side (9 rad/s to the stop), and a scripted man
 *  walking through it either way (the leaf held against the player by his key: the exact, uncapped shove). Idle at 60 and 20 Hz, and at 20 Hz sprinting INTO
 *  the wall at his back while it happens (his own step on top of the shove). */
async function wallClipDoors(opts: { verbose?: boolean } = {}) {
  console.log('\n=== wall-clip/doors: leaves kicked / walked through onto a player standing about the doorway, 60 / 20 Hz, idle and pressing into the wall ===');
  const W = await clipWorld(91); const { game, input } = W; const col = game.col; const doors = game.doors;
  type Ev = { name: string; run: (d: any, gd: any) => void };
  const sideName = (s: number) => s > 0 ? '+' : '−';
  const perEv = new Map<string, { runs: number; bad: number }>(); const offenders: string[] = []; let runs = 0, spots = 0;
  const gd = game.guards[0];
  for (const d of doors.list) {
    const [hx, hz] = d.def.hinge; const Wd = d.def.width; const u0: [number, number] = [Math.cos(d.def.closedDir), Math.sin(d.def.closedDir)]; const n0: [number, number] = [-u0[1], u0[0]];   // closed leaf direction, +perp normal
    const mid: Vec3 = [hx + u0[0] * Wd * 0.5, 0, hz + u0[1] * Wd * 0.5];
    // through-points 1.1 m either side of the doorway middle for the walking man
    const A: Vec3 = [mid[0] + n0[0] * 1.1, 0, mid[2] + n0[1] * 1.1], B: Vec3 = [mid[0] - n0[0] * 1.1, 0, mid[2] - n0[1] * 1.1];
    const evs: Ev[] = [
      { name: 'kicked from +', run: (dd) => doors.kickIn(dd, A, PLAYER_ID) }, { name: 'kicked from −', run: (dd) => doors.kickIn(dd, B, PLAYER_ID) },
      { name: 'man walks +→−', run: (_dd, g) => { g.char.pos = v3.copy(A); g.char.bodyYaw = Math.atan2(B[0] - A[0], B[2] - A[2]); g.char.update(0); g.hold = false; g.script = { goal: [B[0] - n0[0] * 1.5, 0, B[2] - n0[1] * 1.5], speed: 1.3, upper: 'relaxed' }; } },
      { name: 'man walks −→+', run: (_dd, g) => { g.char.pos = v3.copy(B); g.char.bodyYaw = Math.atan2(A[0] - B[0], A[2] - B[2]); g.char.update(0); g.hold = false; g.script = { goal: [A[0] + n0[0] * 1.5, 0, A[2] + n0[1] * 1.5], speed: 1.3, upper: 'relaxed' }; } },
      { name: 'man RUNS +→− (alert: bashes it)', run: (_dd, g) => { g.char.pos = v3.copy(A); g.char.bodyYaw = Math.atan2(B[0] - A[0], B[2] - A[2]); g.char.update(0); g.hold = false; g.state = 'alert'; g.script = { goal: [B[0] - n0[0] * 1.5, 0, B[2] - n0[1] * 1.5], speed: 2.7, upper: 'relaxed' }; } },
    ];
    // standing spots: polar grid about the hinge out to leaf + body, both faces, kept if the statics leave him (settled ≤ 2 cm off) and he is not IN the doorway line itself
    const cand: Vec3[] = [];
    for (let r = 0.45; r <= Wd + 0.5; r += 0.3) for (let k = 0; k < 24; k++) { const a = k * Math.PI / 12; cand.push([hx + Math.cos(a) * r, 0, hz + Math.sin(a) * r]); }
    const kept: Vec3[] = [];
    for (const c of cand) {
      if (c[0] < 0.6 || c[0] > 39.4 || c[2] < 0.6 || c[2] > 27.4) continue;
      const q = v3.copy(c); col.collideCircle(q, PR, 0.2, 1.7, 4); if (v3.distXZ(q, c) > 0.25) continue;   // deep in a wall
      const q2 = v3.copy(q); col.collideCircle(q2, PR, 0.2, 1.7, 1); if (v3.distXZ(q2, q) > 0.01) continue; // did not settle
      if (kept.some(k2 => v3.distXZ(k2, q) < 0.2)) continue;
      kept.push(q);
    }
    spots += kept.length;
    for (const spot of kept) {
      const into = intoNearestWall(col, spot);
      const behaviours: { name: string; hz: number; press: boolean }[] = [{ name: 'idle@60', hz: 60, press: false }, { name: 'idle@20', hz: 20, press: false }];
      if (into) behaviours.push({ name: 'pressing into the wall, sprint@20', hz: 20, press: true }, { name: 'pressing into the wall, walk@60', hz: 60, press: true });
      for (const ev of evs) for (const bh of behaviours) {
        const dt = 1 / bh.hz;
        for (const dd of doors.list) dd.reset(); d.locked = false;   // (an unlocked leaf: the kick / the man must actually move it)
        gd.script = null; gd.hold = true; gd.char.pos = [38.5, 0, 1.5]; gd.char.vel = [0, 0, 0]; gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.state = 'patrol'; gd.awareness = 0;
        game.teleportPlayer(v3.copy(spot)); game.player.crouch = false; W.step(dt);
        if (v3.distXZ(game.player.char.pos, spot) > 0.1) continue;   // (teleport put him elsewhere: an unfair spot after all)
        if (bh.press && into) { game.puppet = { goal: [spot[0] + into[0] * 3, 0, spot[2] + into[1] * 3], aim: null, crouch: false }; if (bh.name.includes('sprint')) input.keys.add('ShiftLeft'); }
        const watch = clipWatch(game); watch.frame();
        ev.run(d, gd);
        for (let f = 0; f < Math.round(2.2 * bh.hz); f++) { W.step(dt); watch.frame(); }
        game.puppet = null; input.keys.delete('ShiftLeft'); gd.script = null; gd.hold = true;
        runs++; const key = `${ev.name} · ${bh.name}`; const pm = perEv.get(key) ?? { runs: 0, bad: 0 }; pm.runs++;
        if (watch.bad()) { pm.bad++; offenders.push(`${d.def.name} spot (${spot[0].toFixed(2)}, ${spot[2].toFixed(2)}) [face ${sideName(doors.side(d, spot))}] · ${key}: ${watch.line()}`); }
        perEv.set(key, pm);
      }
    }
  }
  console.log(`  ${doors.list.length} doors, ${spots} standing spots, ${runs} runs`);
  console.log('  event · behaviour                                        runs   offenders');
  for (const [k, v] of perEv) console.log(`  ${k.padEnd(55)} ${String(v.runs).padStart(5)}   ${String(v.bad).padStart(5)}`);
  if (offenders.length) { console.log(`  offenders (${offenders.length}):`); for (const o of offenders.slice(0, opts.verbose ? 2000 : 50)) console.log('   ✗ ' + o); if (!opts.verbose && offenders.length > 50) console.log(`   … ${offenders.length - 50} more (--verbose)`); }
  const pass = offenders.length === 0;
  console.log(`=== wall-clip/doors: ${pass ? 'PASS' : `FAIL (${offenders.length} of ${runs} runs)`} ===`);
  return pass;
}

/** (2) pushed by others — men and furniture. A scripted man walked (and run) into a player who has a wall at his back, square and at 45°, at 60 and 20 Hz; and the
 *  same with a chair / a carton between them (the prop solver's hold-back is its own uncapped position write). */
async function wallClipShove(opts: { verbose?: boolean } = {}) {
  console.log('\n=== wall-clip/shove: a man walks / runs into a wall-backed player, with and without a chair or carton between them, 60 / 20 Hz ===');
  const W = await clipWorld(92); const { game } = W; const col = game.col; const props = game.props;
  const gd = game.guards[0];
  // wall-backed spots: (player, unit normal out of the wall) — a 12 cm partition, the exterior wall, the conference sill, a door jamb, an inside corner
  const backs: { note: string; p: Vec3; n: [number, number] }[] = [
    { note: 'break room, back to the 12 cm west partition (x=28)', p: [28.5, 0, 20.6], n: [1, 0] },
    { note: 'server room, back to the server/manager partition (x=22)', p: [21.5, 0, 7.0], n: [-1, 0] },
    { note: 'corridor, back to the conference sill + mullions (z=10)', p: [12.0, 0, 10.5], n: [0, 1] },
    { note: 'corridor, back to the north wall right beside the server door\'s east jamb', p: [18.55, 0, 10.5], n: [0, 1] },
    { note: 'lobby, back to the exterior west wall (24 cm) by the glazing sill', p: [4.7, 0, 15.0], n: [1, 0] },
    { note: 'storage, pushed into the inside corner (x=30 partition × z=10 wall) from the north-east', p: [30.5, 0, 9.5], n: [Math.SQRT1_2, -Math.SQRT1_2] },
    { note: 'cubicles, back to a 8 cm cubicle end panel (x=14.1, z 14.3‥16.1)', p: [13.62, 0, 15.2], n: [-1, 0] },
    { note: 'conference room, back to the glazing sill from the room side (z=10)', p: [7.5, 0, 9.5], n: [0, -1] },
  ];
  type How = { name: string; speed: number; angle: number; prop: null | 'chair' | 'carton'; hz: number };
  const hows: How[] = [];
  for (const hz of [60, 20]) for (const prop of [null, 'chair', 'carton'] as const) for (const speed of [1.3, 2.7]) for (const angle of [0, Math.PI / 4]) {
    if (prop && angle) continue;   // (props: square on only)
    hows.push({ name: `${prop ? prop + ' between, ' : ''}${speed > 2 ? 'runs' : 'walks'} in ${angle ? 'at 45°' : 'square'} @${hz}`, speed, angle, prop, hz });
  }
  const chair = props.props.find((p: any) => p.def.kind === 'chair'); const carton = props.props.find((p: any) => p.def.kind === 'cardboard' && p.def.mass < 8);
  const offenders: string[] = []; let runs = 0; const perHow = new Map<string, { runs: number; bad: number }>();
  for (const bk of backs) for (const h of hows) {
    const dt = 1 / h.hz;
    props.reset(); for (const dd of game.doors.list) dd.reset();
    game.teleportPlayer(v3.copy(bk.p)); game.player.crouch = false; W.step(dt); const start = v3.copy(game.player.char.pos);
    // approach direction: from out in the room toward him (rotated by `angle` about him), 2.4 m out; goal 0.6 m PAST him into the wall (a fix on the far side / a man who has not seen him)
    const ax = bk.n[0] * Math.cos(h.angle) - bk.n[1] * Math.sin(h.angle), az = bk.n[0] * Math.sin(h.angle) + bk.n[1] * Math.cos(h.angle);
    const from: Vec3 = [start[0] + ax * 2.4, 0, start[2] + az * 2.4]; const goal: Vec3 = [start[0] - bk.n[0] * 0.6, 0, start[2] - bk.n[1] * 0.6];
    { const q = v3.copy(from); col.collideCircle(q, 0.3, 0.2, 1.5, 3); if (v3.distXZ(q, from) > 0.05) { continue; } }
    let pr: any = null;
    if (h.prop) { pr = h.prop === 'chair' ? chair : carton; if (!pr) continue; pr.x = start[0] + ax * 0.95; pr.z = start[2] + az * 0.95; pr.yaw = Math.atan2(ax, az); pr.vx = pr.vz = pr.w = 0; pr.wake(); pr.dirty = true; pr.place(); }
    gd.script = null; gd.hold = false; gd.state = 'patrol'; gd.awareness = 0; gd.char.pos = v3.copy(from); gd.char.vel = [0, 0, 0]; gd.char.bodyYaw = gd.char.aimYaw = Math.atan2(-ax, -az); gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.char.update(dt);
    gd.script = { goal, speed: h.speed, upper: 'relaxed' };
    const watch = clipWatch(game); watch.frame();
    let backOut = false;
    for (let f = 0; f < Math.round(6 * h.hz); f++) {
      W.step(dt); watch.frame();
      if (!backOut && (gd.script?.arrived || f > 3.5 * h.hz)) { backOut = true; gd.script = { goal: v3.copy(from), speed: h.speed, upper: 'relaxed' }; }   // and back out through him
    }
    gd.script = null; gd.hold = true; gd.char.pos = [38.5, 0, 1.5]; gd.char.update(0);
    runs++; const pm = perHow.get(h.name) ?? { runs: 0, bad: 0 }; pm.runs++;
    const disp = v3.distXZ(game.player.char.pos, start);
    if (watch.bad() || disp > 1.6) { pm.bad++; offenders.push(`${bk.note} · ${h.name}: ${watch.line() || ''}${disp > 1.6 ? ` displaced ${disp.toFixed(2)} m` : ''}`); }
    perHow.set(h.name, pm);
  }
  console.log(`  ${backs.length} wall-backed spots × ${hows.length} approaches = ${runs} runs`);
  console.log('  approach                                   runs   offenders');
  for (const [k, v] of perHow) console.log(`  ${k.padEnd(42)} ${String(v.runs).padStart(5)}   ${String(v.bad).padStart(5)}`);
  if (offenders.length) { console.log(`  offenders (${offenders.length}):`); for (const o of offenders.slice(0, opts.verbose ? 1000 : 50)) console.log('   ✗ ' + o); }
  const pass = offenders.length === 0;
  console.log(`=== wall-clip/shove: ${pass ? 'PASS' : `FAIL (${offenders.length} of ${runs} runs)`} ===`);
  return pass;
}

/** (3) state transitions that write the position or change the collision band: standing up from a crouch everywhere on the floor (0.5 m grid: under every
 *  overhang there is), the takedown lunge onto a man standing near / along / facing a wall from every bearing it is offered from, the kick and the pick
 *  planting him from every spot in reach of each corridor door (jambs, the wall beside it), letting go of / taking hold of a body against a wall — 60 and 20 Hz. */
async function wallClipMoves(opts: { verbose?: boolean } = {}) {
  console.log('\n=== wall-clip/moves: stand up everywhere; takedown lunges, kicks and picks from every spot they are offered; 60 / 20 Hz ===');
  const { buildInteractables, startTakedown, startKick, startPicking } = await import(`${ROOT}/src/game/player.ts`);
  const W = await clipWorld(93); const { game, input, cam, canvas } = W; const col = game.col; const doors = game.doors;
  const offenders: string[] = []; const per = new Map<string, { runs: number; bad: number }>();
  const tally = (k: string, bad: boolean, line: string) => { const pm = per.get(k) ?? { runs: 0, bad: 0 }; pm.runs++; if (bad) { pm.bad++; offenders.push(`${k}: ${line}`); } per.set(k, pm); };
  // --- stand up from a crouch on a 0.5 m grid over the building and a margin round it
  for (const hz of [60, 20]) {
    const dt = 1 / hz;
    for (let x = 3.0; x <= 37.0; x += 0.5) for (let z = 3.0; z <= 25.0; z += 0.5) {
      const c: Vec3 = [x, 0, z]; const q = v3.copy(c); col.collideCircle(q, PR, 0.2, 1.0, 4); if (v3.distXZ(q, c) > 0.3) continue; const q2 = v3.copy(q); if (col.collideCircle(q2, PR, 0.2, 1.0, 1) && v3.distXZ(q2, q) > 0.01) continue;   // a spot a CROUCHED body settles at
      game.teleportPlayer(v3.copy(q)); game.player.crouch = true; W.step(dt);
      if (v3.distXZ(game.player.char.pos, q) > 0.3) continue;
      const p0 = v3.copy(game.player.char.pos);
      const watch = clipWatch(game); watch.frame();
      game.puppet = { goal: null, aim: null, crouch: false };   // stand
      for (let f = 0; f < 8; f++) { W.step(dt); watch.frame(' (standing up)'); }
      game.puppet = null;
      const pop = v3.distXZ(game.player.char.pos, p0);
      tally(`stand up @${hz}`, watch.bad() || pop > 0.3, `crouched at (${p0[0].toFixed(2)}, ${p0[2].toFixed(2)}) → (${game.player.char.pos[0].toFixed(2)}, ${game.player.char.pos[2].toFixed(2)})${pop > 0.3 ? ` POPPED ${pop.toFixed(2)} m` : ''}${game.player.crouch ? ' (stayed crouched)' : ''} ${watch.line()}`);
    }
  }
  // --- takedown lunges: a held man planted near walls at assorted facings; the player offered the takedown from 16 bearings × 3 ranges, crouched and standing
  let gd = game.guards[0]; for (const o of game.guards.slice(1)) o.hold = true;
  const men: { note: string; p: Vec3; yaw: number }[] = [];
  const DEG = Math.PI / 180;
  const wallSpots: { p: Vec3; out: [number, number]; note: string }[] = [
    { p: [28.4, 0, 20.6], out: [1, 0], note: 'break room west partition' }, { p: [21.6, 0, 7.0], out: [-1, 0], note: 'server/manager partition, server side' },
    { p: [12.0, 0, 10.42], out: [0, 1], note: 'corridor, conference sill' }, { p: [12.0, 0, 9.55], out: [0, -1], note: 'conference room, the sill from inside' },
    { p: [30.45, 0, 9.5], out: [Math.SQRT1_2, -Math.SQRT1_2], note: 'storage inside corner' }, { p: [13.7, 0, 15.2], out: [-1, 0], note: 'cubicle end panel (8 cm)' },
    { p: [18.55, 0, 10.45], out: [0, 1], note: 'corridor north wall by the server door jamb' }, { p: [4.55, 0, 15.0], out: [1, 0], note: 'lobby exterior wall' },
    { p: [16.5, 0, 12.65], out: [0, 1], note: 'cubicles, south face of the corridor wall by the west opening' },
  ];
  for (const ws of wallSpots) for (const rel of [0, Math.PI, Math.PI / 2, -Math.PI / 2, Math.PI / 4]) men.push({ note: `${ws.note}, facing ${rel === 0 ? 'away from it' : rel === Math.PI ? 'into it' : rel === Math.PI / 4 ? '45° off it' : 'along it'}`, p: ws.p, yaw: Math.atan2(ws.out[0], ws.out[1]) + rel });
  for (const hz of [60, 20]) {
    const dt = 1 / hz;
    for (const m of men) for (let k = 0; k < 16; k++) for (const range of [0.62, 0.8, 1.0]) for (const crouch of [true, false]) {
      const a = k * Math.PI / 8; const ps: Vec3 = [m.p[0] + Math.sin(a) * range, 0, m.p[2] + Math.cos(a) * range];
      { const q = v3.copy(ps); col.collideCircle(q, PR, 0.2, crouch ? 1.0 : 1.7, 1); if (v3.distXZ(q, ps) > 0.01) continue; }
      gd.hold = true; gd.state = 'patrol'; gd.awareness = 0; gd.script = null; gd.char.pos = v3.copy(m.p); { const q = gd.char.pos; col.collideCircle(q, 0.3, 0.2, 1.5, 4); } gd.char.vel = [0, 0, 0]; gd.char.bodyYaw = gd.char.aimYaw = m.yaw; gd.path = []; gd.pathGoal = null; gd.char.update(0);
      if (gd.state === 'dead' || !gd.char.alive) { break; }
      game.teleportPlayer(v3.copy(ps)); game.player.crouch = crouch; W.step(dt);
      buildInteractables(game, cam, 0, 0, canvas.clientWidth, canvas.clientHeight);
      const it = game.interactables.find((x: any) => x.kind === 'guard' && x.guard === gd);
      if (!it?.inReach) continue;   // not offered from here (not behind him / a wall between / too far)
      const p0 = v3.copy(game.player.char.pos);
      const watch = clipWatch(game); watch.frame();
      startTakedown(game, gd);
      for (let f = 0; f < Math.round(0.9 * hz); f++) { W.step(dt); watch.frame(' (takedown)'); }
      const p1 = game.player.char.pos; const far = col.segmentBlocked([p0[0], 0.5, p0[2]], [p1[0], 0.5, p1[2]], 0) && col.segmentBlocked([p0[0], 1.0, p0[2]], [p1[0], 1.0, p1[2]], 0);
      tally(`takedown @${hz}`, watch.bad() || far, `${m.note}; from bearing ${Math.round(a / DEG)}° at ${range} m ${crouch ? 'crouched' : 'standing'} (${p0[0].toFixed(2)}, ${p0[2].toFixed(2)}) → (${p1[0].toFixed(2)}, ${p1[2].toFixed(2)})${far ? ' — LANDED BEYOND A STATIC from where he struck' : ''} ${watch.line()}`);
      // revive the man for the next go (killGuard ragdolls him): a fresh guard set, parked again
      game.clearAftermath(); game.resetGuards(); gd = game.guards[0]; for (const o of game.guards) { o.hold = true; o.char.pos = [38.5, 0, 1.5 + game.guards.indexOf(o) * 1.2]; o.char.update(0); }
    }
  }
  // --- kicks and picks from every reachable spot on the keyed side of each lockable door (and the unlocked ones: kickIn works on any leaf in reach)
  for (const hz of [60, 20]) {
    const dt = 1 / hz;
    for (const d of doors.list) {
      const [hx, hz0] = d.def.hinge;
      for (let r = 0.4; r <= 1.9; r += 0.25) for (let k = 0; k < 24; k++) for (const act of ['kick', 'pick'] as const) {
        const a = k * Math.PI / 12; const ps: Vec3 = [hx + Math.cos(a) * r, 0, hz0 + Math.sin(a) * r];
        if (ps[0] < 0.6 || ps[0] > 39.4 || ps[2] < 0.6 || ps[2] > 27.4) continue;
        { const q = v3.copy(ps); col.collideCircle(q, PR, 0.2, act === 'pick' ? 1.0 : 1.7, 1); if (v3.distXZ(q, ps) > 0.01) continue; }
        for (const dd of doors.list) dd.reset(); d.angle = 0; d.vel = 0; d.latched = true; d.locked = true; d.place();
        game.teleportPlayer(v3.copy(ps)); game.player.crouch = act === 'pick'; W.step(dt);
        if (doors.side(d, game.player.char.pos) !== d.keySide) continue;
        buildInteractables(game, cam, 0, 0, canvas.clientWidth, canvas.clientHeight);
        const it = game.interactables.find((x: any) => x.kind === 'door' && x.door === d);
        if (!it?.inReach) continue;
        const p0 = v3.copy(game.player.char.pos);
        const watch = clipWatch(game); watch.frame();
        if (act === 'kick') startKick(game, d); else { startPicking(game, d); input.keys.add(IK); }
        for (let f = 0; f < Math.round(1.3 * hz); f++) { W.step(dt); watch.frame(` (${act})`); }
        input.keys.delete(IK); W.step(dt); game.player.picking = null; game.player.kick = null;
        const p1 = game.player.char.pos; const far = col.segmentBlocked([p0[0], 0.5, p0[2]], [p1[0], 0.5, p1[2]], 0) && col.segmentBlocked([p0[0], 1.0, p0[2]], [p1[0], 1.0, p1[2]], 0) && !col.segmentBlockedDynamic([p0[0], 0.5, p0[2]], [p1[0], 0.5, p1[2]]);
        tally(`${act} @${hz}`, watch.bad() || far, `${d.def.name} from (${p0[0].toFixed(2)}, ${p0[2].toFixed(2)}) → (${p1[0].toFixed(2)}, ${p1[2].toFixed(2)})${far ? ' — PLANTED BEYOND A STATIC' : ''} ${watch.line()}`);
      }
    }
  }
  // --- the kick / the pick with a man coming through that door from the room side at the same moment (his key unlatches it and he shoves the leaf through its
  //     arc while the beat has the player planted square to it): the plant must not follow the leaf round its hinge — through the doorway or the wall beside it
  {
    let g0 = game.guards[0];
    for (const hz of [60, 20]) {
      const dt = 1 / hz;
      for (const d of doors.list) {
        if (d.def.exterior) continue;
        const [hx, hz0] = d.def.hinge; const Wd = d.def.width; const u0: [number, number] = [Math.cos(d.def.closedDir), Math.sin(d.def.closedDir)]; const n0: [number, number] = [-u0[1], u0[0]];
        const mid: Vec3 = [hx + u0[0] * Wd * 0.5, 0, hz0 + u0[1] * Wd * 0.5];
        const roomSide: Vec3 = [mid[0] - n0[0] * d.keySide * 1.2, 0, mid[2] - n0[1] * d.keySide * 1.2], beyond: Vec3 = [mid[0] + n0[0] * d.keySide * 2.0, 0, mid[2] + n0[1] * d.keySide * 2.0];
        for (const act of ['kick', 'pick'] as const) for (const pace of ['walks', 'RUNS (alert: bashes it)'] as const) for (const lead of [-0.3, 0, 0.25, 0.5]) for (const along of [0.5, 0.8, 1.05]) for (const off of [0.7, 1.0]) {
          const ps: Vec3 = [hx + u0[0] * Wd * along + n0[0] * d.keySide * off, 0, hz0 + u0[1] * Wd * along + n0[1] * d.keySide * off];
          { const q = v3.copy(ps); col.collideCircle(q, PR, 0.2, act === 'pick' ? 1.0 : 1.7, 1); if (v3.distXZ(q, ps) > 0.01) continue; }
          for (const dd of doors.list) dd.reset(); d.angle = 0; d.vel = 0; d.latched = true; d.locked = true; d.place();
          g0.script = null; g0.hold = true; g0.state = 'patrol'; g0.awareness = 0; g0.char.pos = v3.copy(roomSide); g0.char.vel = [0, 0, 0]; g0.char.bodyYaw = g0.char.aimYaw = Math.atan2(beyond[0] - roomSide[0], beyond[2] - roomSide[2]); g0.path = []; g0.pathGoal = null; g0.char.update(0); g0.char.update(dt);
          game.teleportPlayer(v3.copy(ps)); game.player.crouch = act === 'pick'; W.step(dt);
          buildInteractables(game, cam, 0, 0, canvas.clientWidth, canvas.clientHeight);
          const it = game.interactables.find((x: any) => x.kind === 'door' && x.door === d); if (!it?.inReach) continue;
          const p0 = v3.copy(game.player.char.pos); const watch = clipWatch(game); watch.frame();
          const startMan = () => { g0.hold = false; if (pace !== 'walks') g0.state = 'alert'; g0.script = { goal: v3.copy(beyond), speed: pace === 'walks' ? 1.3 : 2.7, upper: 'relaxed' }; };
          let spot0: Vec3 = v3.copy(p0);   // where the beat means to plant him: off the leaf AS IT STANDS when the action starts
          const startAct = () => { spot0 = v3.copy((act === 'kick' ? doors.kickSpot(d, game.player.char.pos) : doors.workSpot(d, game.player.char.pos)).pos); if (act === 'kick') startKick(game, d); else { startPicking(game, d); input.keys.add(IK); } };
          if (lead < 0) { startMan(); for (let f = 0; f < Math.round(-lead * hz); f++) { W.step(dt); watch.frame(); } startAct(); } else { startAct(); for (let f = 0; f < Math.round(lead * hz); f++) { W.step(dt); watch.frame(` (${act})`); } startMan(); }
          const across = (p: Vec3) => ((p[0] - hx) * n0[0] + (p[2] - hz0) * n0[1]) * d.keySide;   // metres out from the CLOSED leaf line on the keyed side (< 0: in the room)
          let minAcross = across(game.player.char.pos);
          for (let f = 0; f < Math.round(2.0 * hz); f++) { W.step(dt); watch.frame(` (${act}, man coming through)`); if (game.player.kick || game.player.picking) minAcross = Math.min(minAcross, across(game.player.char.pos)); }
          input.keys.delete(IK); W.step(dt); game.player.picking = null; game.player.kick = null; g0.script = null; g0.hold = true; g0.char.pos = [38.5, 0, 1.5]; g0.char.update(0);
          const p1 = game.player.char.pos; const far = col.segmentBlocked([p0[0], 0.5, p0[2]], [p1[0], 0.5, p1[2]], 0) && col.segmentBlocked([p0[0], 1.0, p0[2]], [p1[0], 1.0, p1[2]], 0) && !col.segmentBlockedDynamic([p0[0], 0.5, p0[2]], [p1[0], 0.5, p1[2]]);
          const inRoom = across(p1) < -0.3; const reeled = minAcross < 0.3;   // the plant stands him 0.76 (kick) / 0.5 (pick) out from the leaf line: a man barging out may shoulder him further OUT or aside, but nothing may draw him IN toward the doorway / the wall line (the spot orbiting the hinge with a swung leaf did exactly that)
          tally(`${act}, man comes through @${hz}`, watch.bad() || far || inRoom || reeled, `${d.def.name}, man ${pace} through ${lead < 0 ? `${-lead} s ahead of` : lead > 0 ? `${lead} s after` : 'with'} the ${act}, from (${p0[0].toFixed(2)}, ${p0[2].toFixed(2)}) → (${p1[0].toFixed(2)}, ${p1[2].toFixed(2)})${far ? ' — BEYOND A STATIC' : ''}${inRoom ? ' — ENDED IN THE ROOM (reeled through the doorway)' : ''}${reeled ? ` — drawn in to ${minAcross.toFixed(2)} m off the wall line while planted (spot ${spot0[0].toFixed(2)}, ${spot0[2].toFixed(2)})` : ''} ${watch.line()}`);
        }
      }
    }
  }
  // --- a body against a wall: take hold, haul it into the corner, let go, take hold again (toggleDrag next to statics)
  for (const hz of [60, 20]) {
    const dt = 1 / hz;
    game.clearAftermath(); game.resetGuards(); const v = game.guards[0]; for (const o of game.guards.slice(1)) { o.hold = true; o.char.pos = [38.5, 0, 3]; o.char.update(0); }
    v.char.pos = [28.45, 0, 13.9]; v.char.bodyYaw = Math.PI; v.char.update(0); v.char.update(dt); killGuard(game, v, [-1, 0, 0], true);
    for (let i = 0; i < Math.round(1.5 * hz); i++) W.step(dt);
    const hips = v.char.bones.hips ?? v.char.pos;
    game.teleportPlayer([hips[0] + 0.5, 0, hips[2] + 0.6]); W.step(dt);
    const watch = clipWatch(game); watch.frame();
    game.toggleDrag(v); const got = game.player.dragging === v;
    game.puppet = { goal: [28.3, 0, 12.4], aim: null }; for (let i = 0; i < Math.round(4 * hz); i++) { W.step(dt); watch.frame(' (hauling into the vending-machine corner)'); }
    game.toggleDrag(); for (let i = 0; i < 10; i++) { W.step(dt); watch.frame(' (let go)'); }
    game.toggleDrag(v); for (let i = 0; i < 10; i++) { W.step(dt); watch.frame(' (took hold again)'); }
    game.puppet = { goal: [29.5, 0, 16], aim: null }; for (let i = 0; i < Math.round(2 * hz); i++) { W.step(dt); watch.frame(' (hauling back out)'); }
    if (game.player.dragging) game.toggleDrag(); game.puppet = null;
    tally(`drag by the wall @${hz}`, watch.bad() || !got, `${got ? '' : 'never got hold · '}${watch.line()}`);
  }
  console.log('  move                     runs   offenders');
  for (const [k, v2] of per) console.log(`  ${k.padEnd(24)} ${String(v2.runs).padStart(5)}   ${String(v2.bad).padStart(5)}`);
  if (offenders.length) { console.log(`  offenders (${offenders.length}):`); for (const o of offenders.slice(0, opts.verbose ? 2000 : 60)) console.log('   ✗ ' + o); if (!opts.verbose && offenders.length > 60) console.log(`   … ${offenders.length - 60} more (--verbose)`); }
  const pass = offenders.length === 0;
  console.log(`=== wall-clip/moves: ${pass ? 'PASS' : `FAIL (${offenders.length})`} ===`);
  return pass;
}

/** (4) corners and seams: every thin static's two ENDS from both faces — aimed at the end zone at lateral offsets −0.3 ‥ +0.42 m about the corner (the circle
 *  meets a box corner, or the post / jamb / cross wall that ends it), sprinting straight in and at 45° toward the end, 60 and 20 Hz. */
async function wallClipEnds(opts: { verbose?: boolean } = {}) {
  console.log('\n=== wall-clip/ends: sprint at every wall end / jamb edge / inside corner, straight and at 45°, 60 / 20 Hz ===');
  const W = await clipWorld(94); const { game, input } = W; const col = game.col; const boxes: any[] = col.boxes;
  const offenders: string[] = []; const per = new Map<string, { runs: number; bad: number }>(); let runs = 0, skipped = 0;
  for (let i = 0; i < boxes.length; i++) {
    const b = boxes[i]; if (b.flags & (BoxFlag.NoShadow | BoxFlag.Dynamic)) continue;
    const thinX = b.h[0] <= b.h[2]; const h = thinX ? b.h[0] : b.h[2], L = thinX ? b.h[2] : b.h[0];
    if (h > 0.2 || L < 0.4) continue; if (b.c[1] - b.h[1] > 0.9 || b.c[1] + b.h[1] < 0.3) continue;
    const c = Math.cos(b.yaw), s = Math.sin(b.yaw); const ax: [number, number] = [c, -s], az: [number, number] = [s, c];
    const n = thinX ? ax : az, t = thinX ? az : ax; const w = { b, n, t, L };
    for (const end of [1, -1]) for (const face of [1, -1]) for (const off of [-0.3, 0, 0.2, 0.42]) for (const m of [{ name: 'straight sprint@60', hz: 60, diag: 0 }, { name: 'straight sprint@20', hz: 20, diag: 0 }, { name: '45° toward the end sprint@20', hz: 20, diag: 1 }, { name: '45° toward the end sprint@60', hz: 60, diag: 1 }]) {
      const a = end * (L + off);
      const fx = b.c[0] + t[0] * a + n[0] * (h + 0.6) * face, fz = b.c[2] + t[1] * a + n[1] * (h + 0.6) * face;
      if (fx < 0.6 || fx > 39.4 || fz < 0.6 || fz > 27.4) { skipped++; continue; }
      { const q: Vec3 = [fx, 0, fz]; col.collideCircle(q, PR, 0.2, 1.7, 3); if (v3.distXZ(q, [fx, 0, fz]) > 0.02) { skipped++; continue; } }
      const dt = 1 / m.hz; const dir: [number, number] = [-n[0] * face, -n[1] * face];
      let gx = dir[0], gz = dir[1]; if (m.diag) { gx = (dir[0] + t[0] * end) * Math.SQRT1_2; gz = (dir[1] + t[1] * end) * Math.SQRT1_2; }
      game.teleportPlayer([fx, 0, fz]); game.player.crouch = false; game.player.speedSm = 0;
      if (v3.distXZ(game.player.char.pos, [fx, 0, fz]) > 0.05) { skipped++; continue; }
      game.puppet = { goal: [fx + gx * 4, 0, fz + gz * 4], aim: null, crouch: false }; input.keys.add('ShiftLeft');
      const watch = clipWatch(game, w); watch.frame();
      for (let f = 0; f < Math.round(1.5 * m.hz); f++) { W.step(dt); watch.frame(); }
      game.puppet = null; input.keys.delete('ShiftLeft');
      runs++; const pm = per.get(m.name) ?? { runs: 0, bad: 0 }; pm.runs++;
      if (watch.bad()) { pm.bad++; offenders.push(`${(b.name ?? 'box') + '#' + i} c(${b.c[0].toFixed(2)},${b.c[2].toFixed(2)}) ${(2 * h).toFixed(2)} thick · end ${end > 0 ? '+' : '−'} face ${face > 0 ? '+' : '−'} offset ${off} · ${m.name}: ${watch.line()}`); }
      per.set(m.name, pm);
    }
  }
  console.log(`  ${runs} runs (${skipped} skipped)`);
  console.log('  mode                           runs   offenders');
  for (const [k, v] of per) console.log(`  ${k.padEnd(30)} ${String(v.runs).padStart(5)}   ${String(v.bad).padStart(5)}`);
  if (offenders.length) { console.log(`  offenders (${offenders.length}):`); for (const o of offenders.slice(0, opts.verbose ? 2000 : 60)) console.log('   ✗ ' + o); if (!opts.verbose && offenders.length > 60) console.log(`   … ${offenders.length - 60} more (--verbose)`); }
  const pass = offenders.length === 0;
  console.log(`=== wall-clip/ends: ${pass ? 'PASS' : `FAIL (${offenders.length} of ${runs} runs)`} ===`);
  return pass;
}

// ---------------------------------------------------------------- wall-snap: back to the wall (Q) — take hold / refuse, sidling, the ends, letting go, 60 and 20 Hz, and a brute-force sweep
const wrapA = (a: number) => Math.atan2(Math.sin(a), Math.cos(a));
/** the guards on the grass, AI off, the player a ghost, the camera square to the building (yaw 0: W = north, D = east — the stick is camera-relative) */
async function wallWorld(seed: number) { const W = await clipWorld(seed); W.cam.yaw = 0; W.cam.rebuild(); return W; }
/** What a settled hold must look like, measured from outside the controller: a static face straight behind him (a knee-height ray along −n) with the body circle
 *  0.03 ± 0.01 m off it (`board`: up to 0.13 — something proud of the wall at chest height is holding him off), his back square to it (± 3°), the animator in
 *  'wall', the circle not in any static. */
function wallCheck(game: any, opts: { n?: [number, number]; board?: boolean; yawTol?: number } = {}): string[] {
  const f: string[] = []; const pl = game.player, w = pl.wall, p: Vec3 = pl.char.pos, col = game.col;
  if (!w) return ['not on a wall'];
  if (opts.n && (Math.abs(w.n[0] - opts.n[0]) > 0.01 || Math.abs(w.n[2] - opts.n[1]) > 0.01)) f.push(`face normal (${w.n[0].toFixed(2)}, ${w.n[2].toFixed(2)}), expected (${opts.n[0]}, ${opts.n[1]})`);
  let back: any = null;   // the nearest static straight behind his centre or a hand either side of it (he may be parked over the crack between two racks)
  for (const s of [0, -0.2, 0.2]) { const h = col.raycast([p[0] + w.along[0] * s, 0.45, p[2] + w.along[2] * s], [-w.n[0], 0, -w.n[2]], 1.5); if (h && h.index < 1e6 && (!back || h.t < back.t)) back = h; }
  const gap = back ? back.t - PR : NaN;
  if (!back) f.push('no static face behind him within 1.5 m'); else if (gap < 0.02 || gap > (opts.board ? 0.13 : 0.04)) f.push(`body ${gap.toFixed(3)} m off the face behind him (want 0.03 ± 0.01${opts.board ? ', ≤ 0.13 where a board holds him off' : ''})`);
  const dyaw = Math.abs(wrapA(pl.char.bodyYaw - Math.atan2(w.n[0], w.n[2]))) / DEG_; if (dyaw > (opts.yawTol ?? 3)) f.push(`body yaw ${dyaw.toFixed(1)}° off 'back to the wall'`);
  if (pl.char.anim.stance !== 'wall') f.push(`animator stance '${pl.char.anim.stance}', not 'wall'`);
  const q = v3.copy(p); col.collideCircle(q, PR, 0.2, pl.crouch ? 1.0 : 1.7, 1); if (v3.distXZ(q, p) > 0.01) f.push(`${v3.distXZ(q, p).toFixed(3)} m inside static geometry`);
  if (!Number.isFinite(p[0]) || !Number.isFinite(p[2]) || !Number.isFinite(pl.char.bodyYaw) || !Number.isFinite(pl.char.aimYaw)) f.push('position / yaw not finite');
  return f;
}
const DEG_ = Math.PI / 180;

/** (1) Q beside a sample of faces: the corridor walls (standing, crouched, walked INTO the wall), a 1.4 m cubicle end panel, the server racks' fronts (four racks 9 cm
 *  apart, bridged into one face), the conference glazing from both sides and the lobby's exterior glazing (sill + header, no wall at chest height: allowed), a
 *  break-room partition, the spot just past a door jamb (a short sidestep onto the wall's end) — all must take hold: settled 0.45 m off the face at the expected
 *  station, back to it, 'wall'; and a desk's end panel, the reception counter, the kitchen counter, a shut door leaf square on, open floor — all must refuse
 *  (canWallSnap false, Q leaves him free with the 'nothing…' line). */
async function wallSnapEngage(hz = 60) {
  console.log(`\n=== wall-snap/engage @${hz}: Q beside walls, partitions, racks, glazing (take hold) and desks, counters, a leaf, open floor (refuse) ===`);
  const { canWallSnap } = await import(`${ROOT}/src/game/player.ts`);
  const W = await wallWorld(95); const { game, input } = W; const dt = 1 / hz;
  type C = { note: string; p: Vec3; yaw?: number; crouch?: boolean; n: [number, number] | null; face?: number; side?: [0 | 2, number] };   // n null = must refuse; face = the plane's coordinate along n's axis; side = [axis, value] when Q should also move him ALONG the wall (else he keeps his own coordinate)
  const cases: C[] = [
    { note: 'corridor: the north wall (corridor face z 10.06) at his back', p: [20.0, 0, 10.7], yaw: 0, n: [0, 1], face: 10.06 },
    { note: 'corridor: FACING the north wall, nose to it — takes hold all the same and turns his back to it', p: [21.0, 0, 10.55], yaw: Math.PI, n: [0, 1], face: 10.06 },
    { note: 'corridor: the south wall (face z 12.14)', p: [18.0, 0, 11.5], yaw: Math.PI, n: [0, -1], face: 12.14 },
    { note: 'corridor: the south wall, crouched', p: [24.0, 0, 11.6], crouch: true, n: [0, -1], face: 12.14 },
    { note: 'cubicles: the NW cluster\'s west end panel (a 1.4 m partition, face x 14.06)', p: [13.5, 0, 15.2], n: [-1, 0], face: 14.06 },
    { note: 'server room: the west bank\'s rack fronts (2.1 m; four racks 9 cm apart = one face z 7.1)', p: [16.3, 0, 7.8], n: [0, 1], face: 7.1 },
    { note: 'corridor: the conference glazing — a 0.5 m sill and a header, glass between (allowed)', p: [12.0, 0, 10.7], n: [0, 1], face: 10.06 },
    { note: 'conference room: the same glazing from inside', p: [12.0, 0, 9.4], n: [0, -1], face: 9.94 },
    { note: 'lobby: the exterior west wall at its glazing (sill 0.5 m + header)', p: [4.8, 0, 20.0], n: [1, 0], face: 4.12 },
    { note: 'break room: the west partition by the table (face x 28.06)', p: [28.7, 0, 21.0], n: [1, 0], face: 28.06 },
    { note: 'corridor: a foot past the server door\'s east jamb (x 18.3) — sidesteps onto the wall\'s end (x 18.62)', p: [18.3, 0, 10.6], n: [0, 1], face: 10.06, side: [0, 18.62] },
    { note: 'corridor: north wall under the notice board (6 cm proud at chest height): holds him a few cm off', p: [28.5, 0, 10.7], n: [0, 1], face: 10.06 },
    { note: "manager's office: the desk's end panel, 0.72 m (REFUSE)", p: [28.0, 0, 6.3], n: null },
    { note: 'lobby: the reception counter, 1.15 m (REFUSE)', p: [8.2, 0, 16.2], n: null },
    { note: 'break room: the kitchen counter, 0.95 m, cupboards over it from 1.53 (REFUSE)', p: [34.6, 0, 15.0], n: null },
    { note: 'corridor: square in front of the shut server door — a leaf, both jambs out of reach (REFUSE)', p: [17.6, 0, 10.6], n: null },
    { note: 'cubicle aisle: open floor (REFUSE)', p: [19.9, 0, 18.0], n: null },
    { note: 'corridor: a step south of its middle (z 11.2) — the corridor is 2.08 m between faces, so one wall is always within the 0.7 m reach: the nearer (south, 0.52 m off him)', p: [20.5, 0, 11.2], n: [0, -1], face: 12.14 },
    { note: 'lot: the grass west of the building, nothing within reach (REFUSE)', p: [2.0, 0, 14.0], n: null },
  ];
  let allPass = true;
  for (const c of cases) {
    game.teleportPlayer(v3.copy(c.p)); game.player.crouch = !!c.crouch; if (c.yaw !== undefined) game.player.char.bodyYaw = game.player.char.aimYaw = c.yaw; W.step(dt);
    const p0 = v3.copy(game.player.char.pos); const can = canWallSnap(game); game.messages.length = 0; const msgs0 = 0;   // (the HUD log keeps five lines: emptied so this Q's line is findable)
    input.pressed.add('KeyQ'); W.step(dt);
    const took = !!game.player.wall;
    for (let i = 0; i < Math.round(0.6 * hz); i++) W.step(dt);   // settle (0.22 s) and the turn
    const pl = game.player; const p = pl.char.pos; const fails: string[] = [];
    if (c.n) {
      if (!can) fails.push('canWallSnap said no');
      if (!took) fails.push(`Q did not take hold (message: ${game.messages.slice(msgs0).map((m: any) => m.text).join(' | ') || 'none'})`);
      else if (!pl.wall) fails.push(`let go again within 0.6 s (message: ${game.messages.slice(msgs0).map((m: any) => m.text).join(' | ') || 'none'})`);
      else {
        fails.push(...wallCheck(game, { n: c.n, board: c.note.includes('notice board') }));
        const ax = c.n[0] !== 0 ? 0 : 2, other = ax === 0 ? 2 : 0; const want = c.face! + 0.45 * (c.n[0] + c.n[1]);
        if (!c.note.includes('notice board') && Math.abs(p[ax] - want) > 0.01) fails.push(`stands at ${'xz'[ax >> 1]} ${p[ax].toFixed(3)}, expected ${want.toFixed(3)} (face + 0.45)`);
        const wantOther = c.side ? c.side[1] : p0[other];
        if (Math.abs(p[other] - wantOther) > 0.02) fails.push(`station along the wall ${'xz'[other >> 1]} ${p[other].toFixed(3)}, expected ${wantOther.toFixed(3)}${c.side ? ' (the sidestep onto the wall end)' : ' (his own — Q must not carry him along the wall)'}`);
        if (pl.crouch !== !!c.crouch || pl.wall.crouched !== !!c.crouch) fails.push(`crouch ${pl.crouch} / wall.crouched ${pl.wall.crouched}, expected ${!!c.crouch}`);
        if (Math.hypot(pl.char.vel[0], pl.char.vel[2]) > 0.02) fails.push(`still moving at ${Math.hypot(pl.char.vel[0], pl.char.vel[2]).toFixed(2)} m/s parked`);
      }
    } else {
      if (can) fails.push('canWallSnap said yes');
      if (took || pl.wall) fails.push(`Q took hold of something: n (${pl.wall?.n[0].toFixed(2)}, ${pl.wall?.n[2].toFixed(2)}) d ${pl.wall?.d.toFixed(2)}, now at (${p[0].toFixed(2)}, ${p[2].toFixed(2)})`);
      if (!took && !game.messages.slice(msgs0).some((m: any) => /nothing to put your back to/.test(m.text))) fails.push('no "nothing to put your back to here" line');
      if (v3.distXZ(p, p0) > 0.01) fails.push(`moved ${v3.distXZ(p, p0).toFixed(3)} m on a refused Q`);
      if (pl.char.anim.stance !== 'none') fails.push(`stance '${pl.char.anim.stance}' after a refused Q`);
    }
    const pass = fails.length === 0; allPass &&= pass;
    console.log(`- ${pass ? 'PASS' : 'FAIL'} — ${c.note}: from (${p0[0].toFixed(2)}, ${p0[2].toFixed(2)})${c.crouch ? ' crouched' : ''} → ${pl.wall ? `on n (${pl.wall.n[0]}, ${pl.wall.n[2]}) face ${(pl.wall.d * (pl.wall.n[0] + pl.wall.n[2])).toFixed(2)}, run ${Math.min(Math.abs(pl.wall.lo), Math.abs(pl.wall.hi)).toFixed(2)}‥${Math.max(Math.abs(pl.wall.lo), Math.abs(pl.wall.hi)).toFixed(2)} (${(pl.wall.hi - pl.wall.lo).toFixed(2)} m, ends open ${pl.wall.openLo}/${pl.wall.openHi}), at (${p[0].toFixed(3)}, ${p[2].toFixed(3)}) yaw ${(pl.char.bodyYaw / DEG_).toFixed(1)}°` : `free at (${p[0].toFixed(2)}, ${p[2].toFixed(2)})`}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
    if (game.player.wall) { input.pressed.add('KeyQ'); W.step(dt); }
  }
  console.log(`=== wall-snap/engage @${hz}: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

/** (2) Sidling: take hold, hold the key toward one end for a few seconds, then the other. Every frame: on the plane (|off| < 1 cm — or riding out round something
 *  proud of the wall by at most `ride`, where the leg says so), never in / through a static (the wall-clip watcher), still holding, still 'wall', crouch as it
 *  was; peak speed = WALL_PACE 1.4 (crouched: crouch pace 1.0) and never above it; parked at each end at the expected station with the expected atEdge (an open
 *  end ±1 with the peek ramping to 1 while the key stays down; a walled end, a piece of furniture, the other wall of an inside corner: 0 and no peek). */
async function wallSnapSlide(hz = 60) {
  console.log(`\n=== wall-snap/slide @${hz}: sidle to both ends of the corridor walls (jambs, an opening, the extinguisher), the rack bank, the glazing, furniture, inside corners; standing and crouched ===`);
  const W = await wallWorld(96); const { game, input } = W; const dt = 1 / hz;
  type Leg = { key: string; secs: number; end: number | null; edge: -1 | 0 | 1; ride?: number; note: string };   // end: expected parked coordinate on the moving axis (null = don't check); ride: how far off the plane this leg may take him (default 0.01)
  type S = { note: string; p: Vec3; crouch?: boolean; axis: 0 | 2; pace: number; legs: Leg[] };
  const runs: S[] = [
    { note: 'corridor north wall from x 20', p: [20.0, 0, 10.7], axis: 0, pace: 1.4, legs: [
      { key: 'KeyD', secs: 4.5, end: 24.58, edge: -1, note: 'east to the manager door\'s west jamb: open (the doorway) → peek' },
      { key: 'KeyA', secs: 6.0, end: 18.62, edge: 1, note: 'west to the server door\'s east jamb: open → peek' }] },
    { note: 'corridor north wall from x 20, crouched', p: [20.0, 0, 10.7], crouch: true, axis: 0, pace: 1.0, legs: [
      { key: 'KeyD', secs: 6.0, end: 24.58, edge: -1, note: 'east to the manager jamb at crouch pace' },
      { key: 'KeyA', secs: 7.5, end: 18.62, edge: 1, note: 'west to the server jamb' }] },
    { note: 'corridor south wall from x 18', p: [18.0, 0, 11.5], axis: 0, pace: 1.4, legs: [
      { key: 'KeyA', secs: 2.0, end: 16.82, edge: -1, note: 'west to the cubicle opening (no door, no jamb): open → peek' },
      { key: 'KeyD', secs: 4.2, end: 21.58, edge: 1, ride: 0.10, note: 'east over the extinguisher box (12 cm proud at chest height: rides out ≤ 10 cm and back) to the east cubicle opening' }] },
    { note: 'server room, the west rack bank\'s fronts from x 16.3', p: [16.3, 0, 7.8], axis: 0, pace: 1.4, legs: [
      { key: 'KeyA', secs: 2.0, end: 15.29, edge: 1, note: 'west across three rack joints to the bank\'s end (open: the pocket by the wall)' },
      { key: 'KeyD', secs: 3.0, end: 17.36, edge: -1, note: 'east to the gap between the banks (open)' }] },
    { note: 'corridor, the conference glazing + wall from x 12', p: [12.0, 0, 10.7], axis: 0, pace: 1.4, legs: [
      { key: 'KeyA', secs: 2.0, end: 10.62, edge: 1, note: 'west along the sill to the conference door\'s east jamb (open)' },
      { key: 'KeyD', secs: 5.5, end: 16.58, edge: -1, ride: 0.10, note: 'east off the sill onto the solid wall (one face), out round the wall extinguisher (12 cm proud), to the server door\'s west jamb (open)' }] },
    { note: 'lobby, the exterior west wall from z 20 (one 20 m face through three rooms)', p: [4.8, 0, 20.0], axis: 2, pace: 1.4, legs: [
      { key: 'KeyW', secs: 5.0, end: 14.02, edge: 0, note: 'north to the plant pot in the corner by the arch: stopped dead by it (not an end: atEdge 0, no peek)' },
      { key: 'KeyS', secs: 6.5, end: 20.79, edge: 0, note: 'south to the couch\'s corner: stopped dead by it (atEdge 0, no peek)' }] },
    { note: 'conference room, the exterior west wall from z 8.6 toward the corridor wall', p: [4.8, 0, 8.6], axis: 2, pace: 1.4, legs: [
      { key: 'KeyS', secs: 1.6, end: 9.52, edge: 0, note: 'south into the inside corner: the corridor wall crosses the run — stopped by it (atEdge 0, no peek)' }] },
    { note: 'break room, the west partition from z 22.6 toward the south wall', p: [28.7, 0, 22.6], axis: 2, pace: 1.4, legs: [
      { key: 'KeyS', secs: 1.6, end: 23.46, edge: 0, note: 'south to the run\'s own end in the corner with the exterior wall: walled (atEdge 0, no peek)' },
      { key: 'KeyW', secs: 4.6, end: 18.62, edge: -1, note: 'north the length of the partition to its door\'s south jamb (open → peek)' }] },
  ];
  let allPass = true;
  for (const r of runs) {
    game.teleportPlayer(v3.copy(r.p)); game.player.crouch = !!r.crouch; W.step(dt);
    input.pressed.add('KeyQ'); W.step(dt); for (let i = 0; i < Math.round(0.5 * hz); i++) W.step(dt);
    const fails: string[] = []; const pl = game.player;
    if (!pl.wall) { fails.push('Q did not take hold'); }
    const legNotes: string[] = [];
    for (const leg of r.legs) {
      if (!game.player.wall) break;
      const watch = clipWatch(game); watch.frame();
      let maxOff = 0, peak = 0, lost = '', peekMax = 0, badStance = '', badCrouch = '';
      input.keys.add(leg.key);
      const F = Math.round(leg.secs * hz);
      for (let f = 0; f < F; f++) {
        W.step(dt); watch.frame(` (${leg.key})`);
        const w = game.player.wall; if (!w) { lost = `let go at ${game.time.toFixed(2)} s at (${pl.char.pos[0].toFixed(2)}, ${pl.char.pos[2].toFixed(2)}): ${game.messages.slice(-1).map((m: any) => m.text)}`; break; }
        const off = pl.char.pos[0] * w.n[0] + pl.char.pos[2] * w.n[2] - (w.d + 0.45); maxOff = Math.max(maxOff, Math.abs(off));
        peak = Math.max(peak, Math.hypot(pl.char.vel[0], pl.char.vel[2]));   // (commanded speed; the per-frame displacement is bounded below through the watcher's worst step)
        peekMax = Math.max(peekMax, w.peek);
        if (pl.char.anim.stance !== 'wall' && !badStance) badStance = `stance '${pl.char.anim.stance}' at ${game.time.toFixed(2)} s`;
        if ((pl.crouch !== !!r.crouch || pl.sprinting) && !badCrouch) badCrouch = `crouch ${pl.crouch} sprint ${pl.sprinting} at ${game.time.toFixed(2)} s`;
      }
      input.keys.delete(leg.key);
      const p = pl.char.pos; const w = game.player.wall; const ride = leg.ride ?? 0.01;
      if (lost) fails.push(`${leg.note}: ${lost}`);
      if (watch.bad(0.02)) fails.push(`${leg.note}: ${watch.line(0.02)}`);
      if (watch.S.worstStep > (r.pace + 0.05) * dt + ride) fails.push(`${leg.note}: a one-frame move of ${watch.S.worstStep.toFixed(4)} m (> pace × dt + ride = ${((r.pace + 0.05) * dt + ride).toFixed(4)})`);
      if (maxOff > ride + 1e-3) fails.push(`${leg.note}: ${maxOff.toFixed(3)} m off the plane at worst (allowed ${ride})`);
      if (peak < r.pace - 0.06 || peak > r.pace + 0.01) fails.push(`${leg.note}: peak speed ${peak.toFixed(2)} m/s, expected ${r.pace} (reached, never exceeded)`);
      if (leg.end !== null && Math.abs(p[r.axis] - leg.end) > 0.02) fails.push(`${leg.note}: parked at ${'xz'[r.axis >> 1]} ${p[r.axis].toFixed(3)}, expected ${leg.end}`);
      if (w && w.atEdge !== leg.edge) fails.push(`${leg.note}: atEdge ${w.atEdge} at the end of the leg, expected ${leg.edge}`);
      if (leg.edge !== 0 && peekMax < 0.99) fails.push(`${leg.note}: peek only reached ${peekMax.toFixed(2)} with the key held into the open end`);
      if (leg.edge === 0 && peekMax > 0.01) fails.push(`${leg.note}: peek rose to ${peekMax.toFixed(2)} at a walled end / an obstacle`);
      if (w && Math.hypot(pl.char.vel[0], pl.char.vel[2]) > 0.02) fails.push(`${leg.note}: parked but still commanding ${Math.hypot(pl.char.vel[0], pl.char.vel[2]).toFixed(2)} m/s`);
      if (badStance) fails.push(`${leg.note}: ${badStance}`); if (badCrouch) fails.push(`${leg.note}: ${badCrouch}`);
      legNotes.push(`${leg.key} ${leg.secs}s → (${p[0].toFixed(2)}, ${p[2].toFixed(2)}) atEdge ${w?.atEdge} peek≤${peekMax.toFixed(2)} peak ${peak.toFixed(2)} m/s, max off ${maxOff.toFixed(3)}, worst step ${watch.S.worstStep.toFixed(4)} m, corner cuts ${watch.S.cornerCut}`);
    }
    if (game.player.wall) { input.pressed.add('KeyQ'); W.step(dt); }
    const pass = fails.length === 0; allPass &&= pass;
    console.log(`- ${pass ? 'PASS' : 'FAIL'} — ${r.note}${r.crouch ? ' (crouched)' : ''}\n    ${legNotes.join('\n    ')}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  console.log(`=== wall-snap/slide @${hz}: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

/** (3) Every way off the wall: Q again, the stick held away 0.3 s, a sprint, a takedown / a kick / a pick / a drag started from it, a teleport, going down, a leaf
 *  kicked into him, a man shouldering through — each must leave wall null, the stance back to 'none', the neck un-craned, no velocity into (or along) the wall
 *  left over, and him outside the statics; and the ones that are NOT ways off it (the stick INTO the wall, a tap away shorter than 0.25 s, crouch toggled, a
 *  shot fired, a man brushing past who does not need his spot) must leave him holding. */
async function wallSnapRelease() {
  console.log('\n=== wall-snap/release: Q, walk off, sprint, takedown / kick / pick / drag from the wall, teleport, down, a kicked leaf, a man walking through — and what must NOT let go ===');
  const { startTakedown, startKick, startPicking } = await import(`${ROOT}/src/game/player.ts`);
  const { hitPlayer } = await import(`${ROOT}/src/game/combat.ts`);
  const W = await wallWorld(97); const { game, input } = W; const dt = DT;
  let allPass = true;
  const report = (name: string, fails: string[], extra = '') => { const pass = fails.length === 0; allPass &&= pass; console.log(`- ${pass ? 'PASS' : 'FAIL'} — ${name}${extra ? ': ' + extra : ''}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`); };
  /** on the corridor's north wall at x (face z 10.06, n +z), settled */
  const hold = (x = 20.0, crouch = false) => { for (const k of ['KeyW', 'KeyA', 'KeyS', 'KeyD', 'ShiftLeft', IK]) input.keys.delete(k); game.teleportPlayer([x, 0, 10.7]); game.player.crouch = crouch; W.step(dt); input.pressed.add('KeyQ'); W.step(dt); for (let i = 0; i < 40; i++) W.step(dt); return !!game.player.wall; };
  const freed = (label: string): string[] => {
    const f: string[] = []; const pl = game.player, c = pl.char;
    if (pl.wall) f.push(`${label}: still holding the wall`);
    if (!pl.down && c.anim.stance !== 'none') f.push(`${label}: stance '${c.anim.stance}'`);
    if (Math.abs(c.anim.lookYawExtra) > 1e-6) f.push(`${label}: neck still craned (lookYawExtra ${c.anim.lookYawExtra.toFixed(2)})`);
    if (W.cam.peekOffset) f.push(`${label}: camera peekOffset still set`);
    const q = v3.copy(c.pos); if (!pl.down && game.col.collideCircle(q, PR, 0.2, pl.crouch ? 1.0 : 1.7, 1) && v3.distXZ(q, c.pos) > 0.01) f.push(`${label}: ${v3.distXZ(q, c.pos).toFixed(3)} m inside static geometry`);
    return f;
  };
  // Q again
  { const f: string[] = []; if (!hold()) f.push('staging: no hold'); input.pressed.add('KeyQ'); W.step(dt); f.push(...freed('Q')); const v = game.player.char.vel; if (Math.hypot(v[0], v[2]) > 1e-6) f.push(`velocity (${v[0].toFixed(2)}, ${v[2].toFixed(2)}) left over`); for (let i = 0; i < 30; i++) W.step(dt); if (v3.distXZ(game.player.char.pos, [20, 0, 10.51]) > 0.01) f.push(`drifted to (${game.player.char.pos[0].toFixed(2)}, ${game.player.char.pos[2].toFixed(2)}) after letting go with no key down`); report('Q again, parked', f); }
  // Q again mid-slide: no coasting on along the wall
  { const f: string[] = []; hold(); input.keys.add('KeyD'); for (let i = 0; i < 60; i++) W.step(dt); input.keys.delete('KeyD'); const x0 = game.player.char.pos[0]; input.pressed.add('KeyQ'); W.step(dt); f.push(...freed('Q mid-slide')); for (let i = 0; i < 30; i++) W.step(dt); const dx = game.player.char.pos[0] - x0; if (Math.abs(dx) > 0.03) f.push(`coasted ${dx.toFixed(3)} m along the wall after Q with the key up`); report('Q again at full sidle, key released the same frame', f, `drift ${(game.player.char.pos[0] - x0).toFixed(3)} m`); }
  // stick away (S = south = +z = away from the north wall): lets go after 0.25 s, then walks off
  { const f: string[] = []; hold(); input.keys.add('KeyS'); let offAt = -1; for (let i = 0; i < 60; i++) { W.step(dt); if (!game.player.wall && offAt < 0) offAt = (i + 1) * dt; } input.keys.delete('KeyS'); if (offAt < 0.24 || offAt > 0.31) f.push(`let go after ${offAt.toFixed(3)} s of 'away' (want ≈ 0.25‥0.28)`); f.push(...freed('walk off')); if (game.player.char.pos[2] < 10.51 + 0.5) f.push(`only got to z ${game.player.char.pos[2].toFixed(2)} in the second after (should be walking south)`); report('stick held away (S)', f, `off the wall at +${offAt.toFixed(3)} s, a second later at z ${game.player.char.pos[2].toFixed(2)}`); }
  // a tap away shorter than the hold time: stays
  { const f: string[] = []; hold(); input.keys.add('KeyS'); for (let i = 0; i < 10; i++) W.step(dt); input.keys.delete('KeyS'); for (let i = 0; i < 30; i++) W.step(dt); if (!game.player.wall) f.push('a 0.17 s tap of S let go'); else f.push(...wallCheck(game, { n: [0, 1] })); report('a 0.17 s tap away: stays on', f); }
  // stick INTO the wall (W): nothing
  { const f: string[] = []; hold(); const p0 = v3.copy(game.player.char.pos); input.keys.add('KeyW'); for (let i = 0; i < 90; i++) W.step(dt); input.keys.delete('KeyW'); if (!game.player.wall) f.push('W (into the wall) let go'); else { f.push(...wallCheck(game, { n: [0, 1] })); if (v3.distXZ(game.player.char.pos, p0) > 0.005) f.push(`moved ${v3.distXZ(game.player.char.pos, p0).toFixed(3)} m pushing into the wall`); } report('stick held into the wall (W) 1.5 s: stays put', f); }
  // diagonal into-and-along (W+D): slides east, stays
  { const f: string[] = []; hold(); const x0 = game.player.char.pos[0]; input.keys.add('KeyW'); input.keys.add('KeyD'); for (let i = 0; i < 60; i++) W.step(dt); input.keys.delete('KeyW'); input.keys.delete('KeyD'); if (!game.player.wall) f.push('W+D let go'); else if (game.player.char.pos[0] - x0 < 1.0) f.push(`W+D slid only ${(game.player.char.pos[0] - x0).toFixed(2)} m east in 1 s`); report('W+D (into and along): sidles east, stays on', f, `${(game.player.char.pos[0] - x0).toFixed(2)} m in 1 s`); }
  // sprint (Shift + D): off at once, running east
  { const f: string[] = []; hold(); const x0 = game.player.char.pos[0]; input.keys.add('ShiftLeft'); input.keys.add('KeyD'); W.step(dt); if (game.player.wall) f.push('still holding a frame into the sprint'); for (let i = 0; i < 59; i++) W.step(dt); input.keys.delete('ShiftLeft'); input.keys.delete('KeyD'); f.push(...freed('sprint')); if (!(game.player.char.pos[0] - x0 > 2.5)) f.push(`ran only ${(game.player.char.pos[0] - x0).toFixed(2)} m in the second`); const q = v3.copy(game.player.char.pos); game.col.collideCircle(q, PR, 0.2, 1.7, 1); report('Shift + D from the wall: sprints off along it', f, `${(game.player.char.pos[0] - x0).toFixed(2)} m east in 1 s, z ${game.player.char.pos[2].toFixed(2)}`); }
  // crouch toggled on the wall: stays, wall.crouched follows, still on the plane
  { const f: string[] = []; hold(); input.pressed.add('KeyC'); W.step(dt); for (let i = 0; i < 30; i++) W.step(dt); const w = game.player.wall; if (!w) f.push('C let go'); else { if (!game.player.crouch || !w.crouched) f.push(`crouch ${game.player.crouch} wall.crouched ${w.crouched}`); f.push(...wallCheck(game, { n: [0, 1] })); } input.pressed.add('KeyC'); W.step(dt); for (let i = 0; i < 30; i++) W.step(dt); if (!game.player.wall || game.player.crouch) f.push(`after standing back up: wall ${!!game.player.wall} crouch ${game.player.crouch}`); report('C on the wall: down and up again, holding throughout', f); }
  // a shot fired from the wall (aim layer over the carry): stays
  { const f: string[] = []; hold(); W.cursorAt([20, 1.0, 14]); W.step(dt); input.buttons = 4; for (let i = 0; i < 10; i++) W.step(dt); input.clicked |= 1; W.step(dt); input.buttons = 0; for (let i = 0; i < 30; i++) W.step(dt); if (!game.player.wall) f.push('firing let go'); else f.push(...wallCheck(game, { n: [0, 1], yawTol: 3 })); if (game.player.pistol.roundsReady >= 20 && !(W.audioCounts.get('shot') || W.audioCounts.get('pistol'))) { /* (headless audio names vary; the hold is the point) */ } report('RMB + a round fired into the corridor from the wall: stays on, back still to it', f); }
  // takedown started from the wall (a held man planted with his back to Sam, in reach)
  { const f: string[] = []; hold(20.0, true); const gd = game.guards[0]; gd.hold = true; gd.char.pos = [20.0, 0, 11.35]; gd.char.vel = [0, 0, 0]; gd.char.bodyYaw = gd.char.aimYaw = 0; gd.path = []; gd.pathGoal = null; gd.char.update(0); W.step(dt);
    if (!game.player.wall) f.push('staging: the man planted 0.84 m off knocked him off the wall before the takedown'); startTakedown(game, gd); f.push(...freed('takedown')); for (let i = 0; i < 60; i++) W.step(dt); if (gd.state !== 'dead') f.push(`the takedown did not land (guard ${gd.state})`); f.push(...freed('after the takedown'));
    report('F takedown on a man in reach in front of the wall', f, `guard ${gd.state}`); game.clearAftermath(); game.resetGuards(); for (const o of game.guards) { o.hold = true; o.char.pos = [38.5, 0, 1.5 + game.guards.indexOf(o) * 1.2]; o.char.update(0); } }
  // kick / pick started from the wall right beside the server door (Q at x 18.5 sidesteps him onto the wall's end at the jamb, x 18.62)
  for (const act of ['kick', 'pick'] as const) { const f: string[] = []; const d = game.doors.byName('server'); for (const dd of game.doors.list) dd.reset(); hold(18.5, act === 'pick'); const held = !!game.player.wall && Math.abs(game.player.char.pos[0] - 18.62) < 0.02; if (!held) f.push(`staging: not parked at the jamb (x ${game.player.char.pos[0].toFixed(2)})`);
    if (act === 'kick') startKick(game, d); else { startPicking(game, d); input.keys.add(IK); } f.push(...freed(act)); const watch = clipWatch(game); watch.frame(); for (let i = 0; i < 90; i++) { W.step(dt); watch.frame(` (${act})`); } input.keys.delete(IK); if (watch.bad()) f.push(watch.line()); if (act === 'kick' && !d.lockBroken) f.push('the kick never landed'); if (act === 'pick' && d.pick <= 0) f.push('the pick never started turning'); f.push(...freed(`after the ${act}`)); game.player.picking = null;
    report(`${act} on the server door started from the wall beside its jamb`, f, act === 'kick' ? `lockBroken ${d.lockBroken}` : `pick ${d.pick.toFixed(2)}`); }
  // drag: a body by the wall, F from the wall
  { const f: string[] = []; game.clearAftermath(); game.resetGuards(); const v = game.guards[0]; for (const o of game.guards.slice(1)) { o.hold = true; o.char.pos = [38.5, 0, 3 + game.guards.indexOf(o)]; o.char.update(0); } v.char.pos = [21.2, 0, 11.0]; v.char.bodyYaw = Math.PI / 2; v.char.update(0); v.char.update(dt); killGuard(game, v, [1, 0, 0], true); for (let i = 0; i < 90; i++) W.step(dt);
    hold(20.6, true); if (!game.player.wall) f.push('staging: no hold beside the body'); game.toggleDrag(v); const got = game.player.dragging === v; if (!got) f.push(`never got hold of the body (bodyDist ${(game as any).player ? '' : ''})`); f.push(...freed('drag')); for (let i = 0; i < 30; i++) W.step(dt); if (game.player.dragging) game.toggleDrag(); report('F drag on a body lying by the wall, from the wall', f, `got hold ${got}`); game.clearAftermath(); game.resetGuards(); for (const o of game.guards) { o.hold = true; o.char.pos = [38.5, 0, 1.5 + game.guards.indexOf(o) * 1.2]; o.char.update(0); } }
  // teleport
  { const f: string[] = []; hold(); input.keys.add('KeyD'); for (let i = 0; i < 30; i++) W.step(dt); game.teleportPlayer([6.5, 0, 19.0]); input.keys.delete('KeyD'); f.push(...freed('teleport')); for (let i = 0; i < 30; i++) W.step(dt); if (v3.distXZ(game.player.char.pos, [6.5, 0, 19.0]) > 0.05) f.push(`reeled to (${game.player.char.pos[0].toFixed(2)}, ${game.player.char.pos[2].toFixed(2)}) after the teleport`); f.push(...freed('after the teleport')); report('teleported off the wall mid-sidle', f); }
  // restartEncounter while peeking
  { const f: string[] = []; hold(24.3); input.keys.add('KeyD'); for (let i = 0; i < 60; i++) W.step(dt); const peeking = (game.player.wall?.peek ?? 0) > 0.9 && !!W.cam.peekOffset; if (!peeking) f.push(`staging: not peeking at the manager jamb (peek ${game.player.wall?.peek?.toFixed(2)}, x ${game.player.char.pos[0].toFixed(2)})`); game.restartEncounter(); input.keys.delete('KeyD'); W.step(dt); f.push(...freed('restart')); const pl = game.player; if (pl.wall || pl.char.anim.stance !== 'none' || v3.distXZ(pl.char.pos, game.level.playerSpawn) > 0.05) f.push('fresh player not clean'); report('restartEncounter() at full peek', f); for (const o of game.guards) { o.hold = true; o.char.pos = [38.5, 0, 1.5 + game.guards.indexOf(o) * 1.2]; o.char.update(0); } game.aiEnabled = false; }
  // going down on the wall (god mode off, one round)
  { const f: string[] = []; hold(); game.godMode = false; hitPlayer(game, [0, 0, 1]); W.step(dt); if (!game.player.down) f.push('staging: not down'); f.push(...freed('down')); for (let i = 0; i < 60; i++) W.step(dt); f.push(...freed('a second after going down')); report('shot off the wall (down)', f); game.godMode = true; game.restartEncounter(); W.step(dt); for (const o of game.guards) { o.hold = true; o.char.pos = [38.5, 0, 1.5 + game.guards.indexOf(o) * 1.2]; o.char.update(0); } game.aiEnabled = false; }
  // a leaf flung into him: parked at the manager door's west jamb peeking (x 24.58), the leaf — authored ajar into the corridor — is kicked from inside the office
  // through the rest of its arc, which sweeps his spot: shoved west along the wall / off it — never into it; holding cleanly or let go cleanly
  { const f: string[] = []; for (const dd of game.doors.list) dd.reset(); const d = game.doors.byName('manager'); hold(24.3); input.keys.add('KeyD'); for (let i = 0; i < 60; i++) W.step(dt);
    const w = game.player.wall; if (!w || Math.abs(game.player.char.pos[0] - 24.58) > 0.02 || w.peek < 0.9) f.push(`staging: not peeking at the manager jamb (wall ${!!w}, x ${game.player.char.pos[0].toFixed(2)}, peek ${w?.peek.toFixed(2)})`);
    const p0 = v3.copy(game.player.char.pos); const watch = clipWatch(game); watch.frame(); game.doors.kickIn(d, [25.5, 0, 9.0], 10); let offAt = -1, maxOff = 0;
    for (let i = 0; i < 120; i++) { W.step(dt); watch.frame(' (leaf)'); const ww = game.player.wall; if (ww) maxOff = Math.max(maxOff, game.player.char.pos[2] - 10.51); else if (offAt < 0) offAt = game.time; }
    input.keys.delete('KeyD');
    if (watch.bad()) f.push(watch.line()); if (game.player.wall) f.push(...wallCheck(game, { n: [0, 1], yawTol: 60 }).map(s => 'still holding but ' + s)); else f.push(...freed('leaf'));
    if (Math.abs(d.angle - d.maxA) < 0.05 && v3.distXZ(game.player.char.pos, p0) < 0.05 && game.player.wall) f.push(`staging: the leaf swung clean through to its stop without touching him (door at ${d.angle.toFixed(2)}, he has not moved)`);
    report('the manager\'s door kicked through its arc onto him peeking at its jamb', f, `${offAt >= 0 ? `let go at +${(offAt - (game.time - 2)).toFixed(2)} s ("${game.messages.slice(-1).map((m: any) => m.text)}")` : `held on, moved ${v3.distXZ(game.player.char.pos, p0).toFixed(2)} m along, off the plane ≤ ${maxOff.toFixed(2)} m`}; door now ${d.angle.toFixed(2)} (stop ${d.maxA}); player at (${game.player.char.pos[0].toFixed(2)}, ${game.player.char.pos[2].toFixed(2)})`); }
  // the same leaf with a man behind it: an alert man runs out of the office through the ajar door while Sam peeks at its jamb — his key holds the leaf, his shoulder bashes it, and a held leaf moves the player
  { const f: string[] = []; for (const dd of game.doors.list) dd.reset(); const d = game.doors.byName('manager'); game.clearAftermath(); game.resetGuards(); const gd = game.guards[0]; for (const o of game.guards.slice(1)) { o.hold = true; o.char.pos = [38.5, 0, 3 + game.guards.indexOf(o)]; o.char.update(0); } game.aiEnabled = false;
    hold(24.3); input.keys.add('KeyD'); for (let i = 0; i < 60; i++) W.step(dt);
    if (!game.player.wall || Math.abs(game.player.char.pos[0] - 24.58) > 0.02) f.push(`staging: not parked at the manager jamb (x ${game.player.char.pos[0].toFixed(2)})`);
    const p0 = v3.copy(game.player.char.pos); const watch = clipWatch(game); watch.frame();
    gd.hold = false; gd.state = 'alert'; gd.awareness = 0; gd.char.pos = [25.6, 0, 8.3]; gd.char.vel = [0, 0, 0]; gd.char.bodyYaw = gd.char.aimYaw = 0; gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.char.update(dt);
    gd.script = { goal: [25.4, 0, 12.9], speed: 2.7, upper: 'relaxed' };
    let offAt = -1, maxOff = 0, minD = 9;
    for (let i = 0; i < 60 * 5; i++) { W.step(dt); watch.frame(' (man + leaf)'); minD = Math.min(minD, v3.distXZ(gd.char.pos, game.player.char.pos)); const ww = game.player.wall; if (ww) maxOff = Math.max(maxOff, game.player.char.pos[2] - 10.51); else if (offAt < 0) offAt = game.time; }
    input.keys.delete('KeyD'); gd.script = null; gd.hold = true;
    if (watch.bad()) f.push(watch.line()); if (game.player.wall) f.push(...wallCheck(game, { n: [0, 1], yawTol: 60 }).map(s => 'holding afterwards but ' + s)); else f.push(...freed('man + leaf'));
    if (!(v3.distXZ(gd.char.pos, [25.4, 0, 12.9]) < 0.8)) f.push(`the man never got out (now at (${gd.char.pos[0].toFixed(2)}, ${gd.char.pos[2].toFixed(2)}), door ${d.angle.toFixed(2)})`);
    report('an alert man runs out through the manager\'s door onto him peeking at its jamb', f, `${offAt >= 0 ? `let go ("${game.messages.slice(-1).map((m: any) => m.text)}")` : `held on, moved ${v3.distXZ(game.player.char.pos, p0).toFixed(2)} m, off the plane ≤ ${maxOff.toFixed(2)} m`}; door ${d.angle.toFixed(2)}; closest ${minD.toFixed(2)} m; player at (${game.player.char.pos[0].toFixed(2)}, ${game.player.char.pos[2].toFixed(2)})`);
    gd.char.pos = [38.5, 0, 1.5]; gd.char.update(0); }
  // a scripted man walking THROUGH his spot along the wall, and one brushing past a foot out: pushed along / held off then settling back — never into the wall; either still holding cleanly or let go cleanly
  for (const lane of [10.75, 11.05]) { const f: string[] = []; game.clearAftermath(); game.resetGuards(); const gd = game.guards[0]; for (const o of game.guards.slice(1)) { o.hold = true; o.char.pos = [38.5, 0, 3 + game.guards.indexOf(o)]; o.char.update(0); } game.aiEnabled = false;
    hold(); const p0 = v3.copy(game.player.char.pos);
    gd.hold = false; gd.state = 'patrol'; gd.awareness = 0; gd.char.pos = [23.5, 0, lane]; gd.char.vel = [0, 0, 0]; gd.char.bodyYaw = gd.char.aimYaw = -Math.PI / 2; gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.char.update(dt);
    gd.script = { goal: [16.0, 0, lane], speed: 1.3, upper: 'relaxed' };
    const watch = clipWatch(game); watch.frame(); let maxOff = 0, offAt = -1, minD = 9;
    for (let i = 0; i < 60 * 8; i++) { W.step(dt); watch.frame(' (man)'); const w = game.player.wall; minD = Math.min(minD, v3.distXZ(gd.char.pos, game.player.char.pos)); if (w) maxOff = Math.max(maxOff, game.player.char.pos[2] - 10.51); else if (offAt < 0) offAt = game.time; }
    gd.script = null; gd.hold = true;
    if (watch.bad()) f.push(watch.line()); if (minD > 0.9) f.push(`staging: the man never came within ${minD.toFixed(2)} m`);
    if (game.player.wall) { f.push(...wallCheck(game, { n: [0, 1] }).map(s => 'holding afterwards but ' + s)); } else f.push(...freed('man'));
    if (!(v3.distXZ(gd.char.pos, [16, 0, lane]) < 0.8)) f.push(`the man never got past (now at (${gd.char.pos[0].toFixed(2)}, ${gd.char.pos[2].toFixed(2)}))`);
    report(`a scripted man walks west along z ${lane} ${lane < 10.9 ? 'THROUGH his spot' : 'brushing his shoulder'}`, f, `${game.player.wall ? `still holding, displaced ${v3.distXZ(game.player.char.pos, p0).toFixed(2)} m along, held off the plane ≤ ${maxOff.toFixed(2)} m` : `let go at ${offAt.toFixed(2)} s`}; closest ${minD.toFixed(2)} m; man got to (${gd.char.pos[0].toFixed(1)}, ${gd.char.pos[2].toFixed(1)})`);
    gd.char.pos = [38.5, 0, 1.5]; gd.char.update(0); }
  console.log(`=== wall-snap/release: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

/** (4) Brute force: Q from every free cell of a 1 m grid over the building and its apron, standing and crouched, facing a random way. Wherever it takes hold:
 *  settled per wallCheck (any face, boards allowed), then sidle 1.2 s each way and let go with Q — every frame outside the statics (the wall-clip watcher), off the
 *  plane by no more than the ride-out allowance, and afterwards free and clean. Wherever it refuses: he has not moved. Counts holds / refusals; lists offenders. */
async function wallSnapSweep(hz = 60, opts: { verbose?: boolean } = {}) {
  console.log(`\n=== wall-snap/sweep @${hz}: Q on a 1 m grid over the whole floor, standing + crouched; hold → check, sidle both ways, let go ===`);
  const W = await wallWorld(98); const { game, input } = W; const dt = 1 / hz; const col = game.col; const rnd = seeded(5);
  let holds = 0, refusals = 0, skipped = 0; const offenders: string[] = []; const byNormal = new Map<string, number>(); const leanTops: number[] = [];
  for (const crouch of [false, true]) for (let x = 2.5; x <= 38.5; x += 1.0) for (let z = 2.5; z <= 26.5; z += 1.0) {
    const c: Vec3 = [x, 0, z]; const q = v3.copy(c); col.collideCircle(q, PR, 0.2, crouch ? 1.0 : 1.7, 4); if (v3.distXZ(q, c) > 0.45) { skipped++; continue; } const q2 = v3.copy(q); if (col.collideCircle(q2, PR, 0.2, crouch ? 1.0 : 1.7, 1) && v3.distXZ(q2, q) > 0.01) { skipped++; continue; }
    if (game.guards.some((gd: any) => v3.distXZ(gd.char.pos, q) < 1.3)) { skipped++; continue; }   // (the parked guards on the grass shove him: not what this measures)
    game.teleportPlayer(v3.copy(q)); game.player.crouch = crouch; game.player.char.bodyYaw = game.player.char.aimYaw = (rnd() * 2 - 1) * Math.PI; for (let i = 0; i < 6; i++) W.step(dt);   // (a few frames: a carton or a chair he was dropped onto pushes back first)
    const p0 = v3.copy(game.player.char.pos);
    input.pressed.add('KeyQ'); W.step(dt);
    if (!game.player.wall) { refusals++; for (let i = 0; i < 3; i++) W.step(dt); if (v3.distXZ(game.player.char.pos, p0) > 0.01 || game.player.char.anim.stance !== 'none') offenders.push(`refused at (${p0[0].toFixed(2)}, ${p0[2].toFixed(2)})${crouch ? ' crouched' : ''} but moved ${v3.distXZ(game.player.char.pos, p0).toFixed(3)} m / stance ${game.player.char.anim.stance}`); continue; }
    holds++; const w0 = game.player.wall; const key = `n(${w0.n[0].toFixed(2)},${w0.n[2].toFixed(2)})`; byNormal.set(key, (byNormal.get(key) ?? 0) + 1);
    const tag = `from (${p0[0].toFixed(2)}, ${p0[2].toFixed(2)})${crouch ? ' crouched' : ''} on ${key} face ${w0.d.toFixed(2)} run ${w0.lo.toFixed(2)}‥${w0.hi.toFixed(2)}`;
    { // what is he leaning on: the static at knee height behind the station he was given, and how tall the wall is there (something must reach 1.35 m within 12 cm of the face, or a header sit over it)
      const st: Vec3 = [w0.along[0] * w0.u + w0.n[0] * (w0.d + 0.3), 0.45, w0.along[2] * w0.u + w0.n[2] * (w0.d + 0.3)]; const into: Vec3 = [-w0.n[0], 0, -w0.n[2]];
      const kb = col.raycast(st, into, 0.36); const cb = col.raycast([st[0], 1.35, st[2]], into, 0.45); const hb = col.raycast([st[0], 2.7, st[2]], into, 0.45);
      const top = kb && kb.index < 1e6 ? col.boxes[kb.index].c[1] + col.boxes[kb.index].h[1] : NaN;
      leanTops.push(top); if (!(cb && cb.index < 1e6) && !(hb && hb.index < 1e6)) offenders.push(`${tag}: nothing at chest height nor above door height over the knee-height face (top ${top.toFixed(2)}) — leaning on something low`);   // (the box itself may be a sill or a car body: what vouches for the wall is what stands over it)
    }
    const watch = clipWatch(game); watch.frame();
    for (let i = 0; i < Math.round(0.6 * hz); i++) { W.step(dt); watch.frame(' (settling)'); }
    const fails: string[] = [];
    if (!game.player.wall) fails.push(`let go while settling: ${game.messages.slice(-1).map((m: any) => m.text)} (props within a metre: ${game.props.props.filter((pr: any) => Math.hypot(pr.x - game.player.char.pos[0], pr.z - game.player.char.pos[2]) < 1.0).map((pr: any) => `${pr.def.kind}@${pr.x.toFixed(2)},${pr.z.toFixed(2)}${Math.hypot(pr.x - pr.home[0], pr.z - pr.home[1]) > 0.05 ? ' (moved ' + Math.hypot(pr.x - pr.home[0], pr.z - pr.home[1]).toFixed(2) + ' m from home)' : ''}`).join(', ') || 'none'})`);
    else { fails.push(...wallCheck(game, { board: true })); if (v3.distXZ(game.player.char.pos, p0) > 0.35 + 0.7 + 0.05) fails.push(`carried ${v3.distXZ(game.player.char.pos, p0).toFixed(2)} m from where Q was pressed`); }
    let maxOff = 0;
    const alongZ = Math.abs(w0.n[0]) > Math.abs(w0.n[2]);   // the wall runs along z (normal mostly ±x): W / S sidle, else D / A; then the key INTO the wall (nothing), last the key AWAY from it (lets go — allowed, checked clean)
    const keys = alongZ ? ['KeyW', 'KeyS', w0.n[0] > 0 ? 'KeyA' : 'KeyD', w0.n[0] > 0 ? 'KeyD' : 'KeyA'] : ['KeyD', 'KeyA', w0.n[2] > 0 ? 'KeyW' : 'KeyS', w0.n[2] > 0 ? 'KeyS' : 'KeyW'];
    for (let ki = 0; ki < keys.length; ki++) {
      const kname = keys[ki]; if (!game.player.wall) break;
      if (ki === 2) for (let i = 0; i < Math.round(0.4 * hz); i++) { W.step(dt); watch.frame(' (coasting)'); }   // let the sidle's coast run out before judging the key INTO the wall
      const pk = v3.copy(game.player.char.pos);
      input.keys.add(kname);
      for (let i = 0; i < Math.round(1.2 * hz); i++) { W.step(dt); watch.frame(` (${kname})`); const w = game.player.wall; if (!w) break; maxOff = Math.max(maxOff, game.player.char.pos[0] * w.n[0] + game.player.char.pos[2] * w.n[2] - (w.d + 0.45)); }
      input.keys.delete(kname);
      if (ki === 2 && (!game.player.wall || v3.distXZ(game.player.char.pos, pk) > 0.02)) fails.push(`${kname} (into the wall) ${game.player.wall ? `moved him ${v3.distXZ(game.player.char.pos, pk).toFixed(3)} m` : 'let go'}`);
      if (ki === 3 && game.player.wall && Math.abs(alongZ ? w0.n[0] : w0.n[2]) > 0.9) fails.push(`${kname} (away from the wall) held 1.2 s did not let go`);
    }
    if (maxOff > 0.13) fails.push(`${maxOff.toFixed(3)} m off the plane at worst while sidling`);
    if (watch.bad(0.02)) fails.push(watch.line(0.02));
    if (game.player.wall) { input.pressed.add('KeyQ'); W.step(dt); }
    { const pl = game.player, cc = pl.char; const qq = v3.copy(cc.pos); if (pl.wall || cc.anim.stance !== 'none' || Math.abs(cc.anim.lookYawExtra) > 1e-6 || W.cam.peekOffset || (col.collideCircle(qq, PR, 0.2, pl.crouch ? 1.0 : 1.7, 1) && v3.distXZ(qq, cc.pos) > 0.01)) fails.push(`not clean after letting go: wall ${!!pl.wall} stance ${cc.anim.stance} look ${cc.anim.lookYawExtra.toFixed(2)} peekOffset ${!!W.cam.peekOffset} in-geometry ${v3.distXZ(qq, cc.pos).toFixed(3)}`); }
    if (fails.length) offenders.push(`${tag}: ${fails.join(' · ')}`);
  }
  const tops = new Map<string, number>(); for (const t of leanTops) { const k = Number.isFinite(t) ? t.toFixed(2) : 'none'; tops.set(k, (tops.get(k) ?? 0) + 1); }
  console.log(`  ${holds} holds, ${refusals} refusals, ${skipped} grid points skipped (inside things / by the parked guards); holds by face normal: ${[...byNormal.entries()].map(([k, n]) => `${k} ${n}`).join(', ')}\n  top of the box leant on (m → holds): ${[...tops.entries()].sort((a, b) => parseFloat(a[0]) - parseFloat(b[0])).map(([k, n]) => `${k}:${n}`).join(' ')}`);
  if (offenders.length) { console.log(`  offenders (${offenders.length}):`); for (const o of offenders.slice(0, opts.verbose ? 2000 : 60)) console.log('   ✗ ' + o); if (!opts.verbose && offenders.length > 60) console.log(`   … ${offenders.length - 60} more (--verbose)`); }
  const pass = offenders.length === 0;
  console.log(`=== wall-snap/sweep @${hz}: ${pass ? 'PASS' : `FAIL (${offenders.length})`} ===`);
  return pass;
}

/** (5) Pushed while holding — the take-hold ease and the settled hold against everything that moves him and that the face trace cannot see. Every frame from
 *  the Q: never displaced more than 0.2 m (the soak's 'popped' is 0.3; his own sidle is 2.3 cm a frame at 60 Hz, 7 at 20), never in / through a static (the
 *  wall-clip watcher), the furniture about him never fired off (≤ 2.5 m/s: a lean or a hip is not a kick); and it ends one of two clean ways — still holding
 *  with his back within the shove allowance of the plane and the animator in 'wall', or free with nothing of the hold left on him and a line saying why.
 *  (a) Q with something between him and the face: the carton stack against the cubicle wall from the very spot the soak popped him 0.42 m in a frame (seed 3
 *  @ 362 s — the ease wrote him a stride deeper into the cartons every frame regardless, fired one along the wall and spent the saved-up half metre the frame it
 *  cleared), standing and crouched, a chair rolled against the corridor wall, a bin at the edge of his station, a man planted on it, the manager's leaf ajar
 *  across it; (b) settled on the corridor wall, then: a carton walked into him along the wall by a man, a chair walked square into him from the room, a man
 *  running through his spot, the manager's leaf kicked through him at its jamb, and the same leaf walked open onto him by a keyed man — 60 and 20 Hz. */
async function wallPush() {
  console.log('\n=== wall-push: Q into furniture / a man / a leaf standing off the face, and a settled hold shoved by a carton, a chair, a runner, a leaf — ≤ 0.2 m a frame, nothing launched, never in a static, a clean hold or a clean release; 60 / 20 Hz ===');
  const W = await wallWorld(99); const { game, input } = W; const col = game.col; const props = game.props;
  let allPass = true;
  const gd = game.guards[0];
  const parkGuard = () => { gd.script = null; gd.hold = true; gd.state = 'patrol'; gd.awareness = 0; gd.char.pos = [38.5, 0, 1.5]; gd.char.vel = [0, 0, 0]; gd.path = []; gd.pathGoal = null; gd.char.update(0); };
  const plant = (p: Vec3, yaw: number) => { gd.script = null; gd.hold = true; gd.state = 'patrol'; gd.awareness = 0; gd.char.pos = v3.copy(p); gd.char.vel = [0, 0, 0]; gd.char.bodyYaw = gd.char.aimYaw = yaw; gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.char.update(DT); };
  const putProp = (kind: string, x: number, z: number, yaw = 0) => { const pr = props.props.find((p: any) => p.def.kind === kind && (kind !== 'cardboard' || p.def.mass < 8)); if (!pr) return null; pr.x = x; pr.z = z; pr.yaw = yaw; pr.vx = pr.vz = pr.w = 0; pr.wake(); pr.dirty = true; pr.place(); return pr; };
  const clearKeys = () => { for (const k of ['KeyW', 'KeyA', 'KeyS', 'KeyD', 'ShiftLeft', IK]) input.keys.delete(k); };
  /** run `secs` from now, watching him: worst per-frame step, the clip watcher, the fastest any prop within 2.5 m of him went, whether / why the hold dropped */
  const watchRun = (dt: number, secs: number, each?: (f: number) => void) => {
    const watch = clipWatch(game); watch.frame(); let prev = v3.copy(game.player.char.pos); let worst = 0, worstAt = '', worstHeld = 0, worstHeldAt = '', propV = 0, propWhich = ''; let dropped = ''; const hadWall = !!game.player.wall; let everWall = hadWall;
    for (let f = 0; f < Math.round(secs / dt); f++) {
      each?.(f); const msgs0 = game.messages.length; const heldBefore = !!game.player.wall; W.step(dt); watch.frame();
      const p = game.player.char.pos; const st = v3.distXZ(p, prev); prev = v3.copy(p); const at = `${game.time.toFixed(2)}s (${p[0].toFixed(2)}, ${p[2].toFixed(2)}) settle ${game.player.wall ? game.player.wall.settle.toFixed(2) : '—'}`;
      if (st > worst) { worst = st; worstAt = at; } if (heldBefore && game.player.wall && st > worstHeld) { worstHeld = st; worstHeldAt = at; }   // (a frame that began and ended on the wall: the ease's / the hold's own doing)
      for (const pr of props.props) { if (Math.hypot(pr.x - p[0], pr.z - p[2]) > 2.5) continue; const v = Math.hypot(pr.vx, pr.vz); if (v > propV) { propV = v; propWhich = `${pr.def.kind}@(${pr.x.toFixed(2)}, ${pr.z.toFixed(2)})`; } }
      if (game.player.wall) everWall = true; else if (everWall && !dropped) dropped = game.messages.slice(msgs0 ? msgs0 - 1 : 0).map((m: any) => m.text).filter((s: string) => /wall/.test(s)).slice(-1)[0] ?? '(no line)';
    }
    return { watch, worst, worstAt, worstHeld, worstHeldAt, propV, propWhich, dropped, everWall };
  };
  /** the two clean endings: holding — back within WALL_SHOVE_OFF (0.25, and a hair) of the plane, 'wall', not in a static; free — nothing of the hold left */
  const endState = (): string[] => {
    const f: string[] = []; const pl = game.player, c = pl.char, w = pl.wall;
    const q = v3.copy(c.pos); if (col.collideCircle(q, PR, 0.2, pl.crouch ? 1.0 : 1.7, 1) && v3.distXZ(q, c.pos) > 0.02) f.push(`${v3.distXZ(q, c.pos).toFixed(3)} m inside static geometry at the end`);
    if (w) { const off = c.pos[0] * w.n[0] + c.pos[2] * w.n[2] - (w.d + PR + 0.03); if (off < -0.07 || off > 0.27) f.push(`holding, but ${off.toFixed(3)} m off the plane`); if (c.anim.stance !== 'wall') f.push(`holding in stance '${c.anim.stance}'`); }
    else { if (c.anim.stance !== 'none') f.push(`free but stance '${c.anim.stance}'`); if (Math.abs(c.anim.lookYawExtra) > 1e-6 || W.cam.peekOffset) f.push('free but the neck / camera still peeking'); }
    return f;
  };
  /** per-frame displacement bounds: overall, the soak's own 'popped' bar at this rate less a margin (0.2 m at 60 Hz — a stride plus a capped shove scale with the
   *  frame, so 0.56 at 20); for a frame spent on the wall, the take-hold ease's own peak stride from full reach (≈ 6.8 × 0.85 m × dt) and a little */
  const judge = (r: ReturnType<typeof watchRun>, dt: number): string[] => {
    const f: string[] = []; const tol = Math.max(0.3, 12 * dt + 0.06) - 0.1, tolHeld = 0.12 + 3.6 * dt;
    if (r.worst > tol) f.push(`moved ${r.worst.toFixed(3)} m in one frame at ${r.worstAt} (bar ${tol.toFixed(2)} at this rate)`);
    if (r.worstHeld > tolHeld) f.push(`moved ${r.worstHeld.toFixed(3)} m in one frame ON the wall at ${r.worstHeldAt} (bar ${tolHeld.toFixed(2)}: the ease's own stride)`);
    if (r.watch.bad(0.03)) f.push(r.watch.line(0.03));
    if (r.propV > 2.5) f.push(`${r.propWhich} was sent off at ${r.propV.toFixed(2)} m/s`);
    f.push(...endState());
    return f;
  };
  const report = (name: string, fails: string[], extra: string) => { const pass = fails.length === 0; allPass &&= pass; console.log(`- ${pass ? 'PASS' : 'FAIL'} — ${name}: ${extra}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`); };
  for (const hz of [60, 20]) {
    const dt = 1 / hz;
    // ---- (a) Q with something standing between him and the face
    type A = { note: string; p: Vec3; crouch?: boolean; yaw?: number; setup?: () => void };
    const casesA: A[] = [
      { note: 'the carton stack against the cubicle wall, from the soak\'s spot (13.13, 13.67), standing', p: [13.126, 0, 13.674], yaw: Math.PI },
      { note: 'the same, crouched', p: [13.126, 0, 13.674], yaw: Math.PI, crouch: true },
      { note: 'a chair rolled against the corridor\'s north wall square behind him (the south wall out of reach)', p: [20.0, 0, 10.95], yaw: Math.PI, setup: () => { putProp('chair', 20.0, 10.42, 0); } },
      { note: 'a bin standing on the edge of the station he would take (against the wall, a foot east of square)', p: [20.0, 0, 10.95], yaw: Math.PI, setup: () => { putProp('bin', 20.33, 10.3, 0); } },
      { note: 'a man planted on the wall right behind him (the trace cannot see him: the ease meets him)', p: [20.0, 0, 11.0], yaw: Math.PI, setup: () => { plant([20.0, 0, 10.42], 0); } },
      { note: 'the manager\'s leaf ajar into the corridor across the wall behind him', p: [25.55, 0, 11.25], yaw: Math.PI, setup: () => { const d = game.doors.byName('manager'); d.reset(); } },
    ];
    for (const c of casesA) {
      props.reset(); for (const dd of game.doors.list) dd.reset(); parkGuard(); clearKeys();
      game.teleportPlayer(v3.copy(c.p)); game.player.crouch = !!c.crouch; if (c.yaw !== undefined) game.player.char.bodyYaw = game.player.char.aimYaw = c.yaw; W.step(dt);
      c.setup?.(); W.step(dt);
      const p0 = v3.copy(game.player.char.pos); game.messages.length = 0;
      input.pressed.add('KeyQ');
      const r = watchRun(dt, 1.2);
      const took = r.everWall; const line = game.messages.map((m: any) => m.text).filter((s: string) => /wall|back to/.test(s)).slice(-1)[0] ?? '—';
      const fails = judge(r, dt);
      if (!took && !/can't get to the wall|nothing to put your back/.test(line)) fails.push(`refused without a line (log: ${game.messages.map((m: any) => m.text).join(' | ') || 'empty'})`);
      if (took && r.dropped && !/can't get to the wall|pushed off the wall/.test(r.dropped)) fails.push(`let go without a line (${r.dropped})`);
      report(`(a) @${hz} Q · ${c.note}`, fails, `${took ? (game.player.wall ? `holding at (${game.player.char.pos[0].toFixed(2)}, ${game.player.char.pos[2].toFixed(2)}), ${v3.distXZ(game.player.char.pos, p0).toFixed(2)} m from the press` : `took hold, then let go: "${r.dropped}"`) : `refused: "${line}"`}; worst frame step ${r.worst.toFixed(3)} m (on the wall ${r.worstHeld.toFixed(3)}); fastest prop ${r.propV.toFixed(2)} m/s${r.propV > 0.05 ? ' (' + r.propWhich + ')' : ''}`);
      if (game.player.wall) { input.pressed.add('KeyQ'); W.step(dt); }
    }
    // ---- (b) settled on the corridor's north wall (face z 10.06, station z 10.51), then shoved
    const settle = (x: number, peekEast = false): boolean => { clearKeys(); game.teleportPlayer([x, 0, 10.7]); game.player.crouch = false; W.step(dt); input.pressed.add('KeyQ'); W.step(dt); for (let i = 0; i < Math.round(0.5 * hz); i++) W.step(dt); if (peekEast) { input.keys.add('KeyD'); for (let i = 0; i < Math.round(1.0 * hz); i++) W.step(dt); } return !!game.player.wall; };
    type B = { note: string; x: number; peek?: boolean; secs: number; go: () => void; during?: (f: number) => void; after?: () => string[] };
    const casesB: B[] = [
      { note: 'a carton walked into him along the wall by a man coming west down it', x: 20.0, secs: 7, go: () => { putProp('cardboard', 21.3, 10.42, 0); plant([23.5, 0, 10.72], -Math.PI / 2); gd.hold = false; gd.script = { goal: [16.0, 0, 10.72], speed: 1.3, upper: 'relaxed' }; } },
      { note: 'a chair walked square into him from across the corridor (the man wants the wall behind him)', x: 20.0, secs: 6, go: () => { putProp('chair', 20.0, 11.35, Math.PI); plant([20.0, 0, 11.95], Math.PI); gd.hold = false; gd.script = { goal: [20.0, 0, 10.3], speed: 1.3, upper: 'relaxed' }; }, during: (f) => { if (f === Math.round(3.5 * hz)) gd.script = { goal: [23.0, 0, 11.6], speed: 1.3, upper: 'relaxed' }; } },
      { note: 'a man RUNNING west through his spot along the wall', x: 20.0, secs: 5, go: () => { plant([23.5, 0, 10.75], -Math.PI / 2); gd.hold = false; gd.state = 'alert'; gd.script = { goal: [16.0, 0, 10.75], speed: 2.7, upper: 'relaxed' }; } },
      { note: 'peeking at the manager\'s west jamb, the leaf kicked through him from inside the office', x: 24.3, peek: true, secs: 2.5, go: () => { game.doors.kickIn(game.doors.byName('manager'), [25.5, 0, 9.0], 10); } },
      { note: 'peeking at the same jamb, an alert man runs out through the ajar leaf (his key holds it, his shoulder bashes it)', x: 24.3, peek: true, secs: 5, go: () => { plant([25.6, 0, 8.3], 0); gd.hold = false; gd.state = 'alert'; gd.script = { goal: [25.4, 0, 12.9], speed: 2.7, upper: 'relaxed' }; } },
    ];
    for (const c of casesB) {
      props.reset(); for (const dd of game.doors.list) dd.reset(); parkGuard();
      const held = settle(c.x, !!c.peek); const p0 = v3.copy(game.player.char.pos);
      const fails: string[] = []; if (!held) fails.push('staging: no settled hold on the corridor wall');
      c.go();
      const r = watchRun(dt, c.secs, c.during);
      clearKeys(); const gEnd = v3.copy(gd.char.pos); parkGuard();
      fails.push(...judge(r, dt));
      if (r.dropped && !/pushed off the wall|can't get to the wall/.test(r.dropped)) fails.push(`let go without its line (${r.dropped})`);
      report(`(b) @${hz} settled · ${c.note}`, fails, `${game.player.wall ? `still holding, ${v3.distXZ(game.player.char.pos, p0).toFixed(2)} m along from where he was` : `let go: "${r.dropped}"`}; worst frame step ${r.worst.toFixed(3)} m (on the wall ${r.worstHeld.toFixed(3)}); fastest prop ${r.propV.toFixed(2)} m/s${r.propV > 0.05 ? ' (' + r.propWhich + ')' : ''}; the man ended at (${gEnd[0].toFixed(1)}, ${gEnd[2].toFixed(1)})`);
      if (game.player.wall) { input.pressed.add('KeyQ'); W.step(dt); }
    }
  }
  console.log(`=== wall-push: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

const VERBOSE = process.argv.includes('--verbose');
if (which === 'wall-snap' || which === 'wall-snap-engage' || which === 'player' || which === 'all') { await wallSnapEngage(60); await wallSnapEngage(20); }
if (which === 'wall-snap' || which === 'wall-push' || which === 'player' || which === 'all') await wallPush();
if (which === 'wall-snap' || which === 'wall-snap-slide' || which === 'player' || which === 'all') { await wallSnapSlide(60); await wallSnapSlide(20); }
if (which === 'wall-snap' || which === 'wall-snap-release' || which === 'player' || which === 'all') await wallSnapRelease();
if (which === 'wall-snap' || which === 'wall-snap-sweep' || which === 'all') { await wallSnapSweep(60, { verbose: VERBOSE }); await wallSnapSweep(20, { verbose: VERBOSE }); }   // (≈ 2 min: not in the quick 'player' set)
if (which === 'wall-clip-speed' || which === 'wall-clip' || which === 'player' || which === 'all') await wallClipSpeed({ verbose: VERBOSE });
if (which === 'wall-clip-doors' || which === 'wall-clip' || which === 'all') await wallClipDoors({ verbose: VERBOSE });   // (≈ 4 min: 8000+ runs — not in the quick 'player' set)
if (which === 'wall-clip-shove' || which === 'wall-clip' || which === 'player' || which === 'all') await wallClipShove({ verbose: VERBOSE });
if (which === 'wall-clip-moves' || which === 'wall-clip' || which === 'player' || which === 'all') await wallClipMoves({ verbose: VERBOSE });
if (which === 'wall-clip-ends' || which === 'wall-clip' || which === 'player' || which === 'all') await wallClipEnds({ verbose: VERBOSE });
// ---------------------------------------------------------------- staging shared by the doorway probes below
/** Every guard but [0] out of it: parked on the lot east of the building — held, pinned and blind (dazzled for good), so [0]'s gunfire (which puts them on
 *  alert) neither brings them running nor has them put rounds of their own into Sam through the storage window. Returns [0]. */
function parkOthers(game: any): any {
  for (const o of game.guards.slice(1)) { o.hold = true; o.pinned = true; o.dazzledUntil = 1e9; o.char.pos = [37.5, 0, 6 + game.guards.indexOf(o)]; o.char.update(0); }
  return game.guards[0];
}
/** A leaf standing at `angle` (settled, closer idle, off the latch unless the angle is 0), unlocked unless `lock`. */
function stageLeaf(d: any, angle: number, lock = false) { d.angle = angle; d.vel = 0; d.closing = false; d.latched = Math.abs(angle) < 1e-3; d.locked = lock; d.quietT = 5; d.place(); }
/** A guard planted at `p` facing `face`, still, no path, pistol up, posed (two updates: bones and muzzle where clearShot / canSee will read them). */
function placeGuard(gd: any, p: Vec3, face: Vec3) { gd.char.pos = v3.copy(p); gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.char.bodyYaw = gd.char.aimYaw = Math.atan2(face[0] - p[0], face[2] - p[2]); gd.char.aimPitch = 0; gd.path = []; gd.pathGoal = null; gd.char.anim.upper = 'aim'; gd.char.update(0); gd.char.update(DT); }
/** A fresh 'alert' on `fix`, the sighting `seenAgo` seconds old (0 = eyes on him now: licensed to shoot), his CONTACT beat `react` seconds, every alert clock zeroed. */
function armAlert(gd: any, game: any, fix: Vec3, seenAgo = 0, react = 0, fireCd = 0.3) {
  gd.state = 'alert'; gd.awareness = 1; gd.reactT = react; gd.fireCd = fireCd; gd.shots = 0; gd.reloadT = -1; gd.lastKnown = v3.copy(fix); gd.lastSeenT = game.time - seenAgo;
  resetChase(gd); gd.stuckT = 0; gd.dazzledUntil = -1; gd.pinned = false; gd.hold = false; gd.ledT = game.time;   // (ledT: nothing staged before this counts as a sighting to lead from)
}

/** "The AI sometimes has its aim get stuck in doors": an alert man planted at a doorway, muzzle through the frame, Sam standing in the room beyond — neither
 *  coming in nor giving up. Stage the doorway situations one by one (leaf ajar / wide / shut / locked / held by Sam's body, the man on the jamb or square on,
 *  Sam lit or dark) and print a timeline: guard state / pos / distance to the doorway / does canSee say yes / is he licensed to shoot / rounds fired and rounds
 *  that reached Sam (god mode keeps him up) / speed / path / the leaf's angle / the alert stall clock. PASS per case unless he spends 3 s alert and planted with
 *  eyes on Sam and nothing he fires arriving, or 3 s alert and planted within reach of the doorway with nothing arriving. `aim-door 7` runs case 7 alone. */
async function aimDoor() {
  console.log('\n=== aim-door: alert guards at doorways — sight yes but no line of fire, shut / locked / held leaves, a fix through the frame ===');
  const { canSee } = await import(`${ROOT}/src/game/guards.ts`);
  type Case = {
    label: string; door: string; angle?: number | null; lock?: boolean;                 // leaf staged at `angle` (null: as authored); lock: latched + locked against the corridor
    guardAt: Vec3; sam: Vec3; samVis: number; samCrouch?: boolean;                      // the man (alert, a fresh sighting, fix on Sam) and Sam (standing lit in the room unless samVis is low)
    samHoldsLeaf?: boolean;                                                            // Sam plants himself in the leaf's arc just inside (his body is what the leaf rests on)
    seenT?: number;                                                                    // seconds since the sighting at t0 (default 0 = eyes on him this instant)
    secs?: number;
    expect: 'hit' | 'enter';                                                           // what must ALSO happen for a PASS: a round of his reaches Sam (lit cases), or he is inside the room with him within the run (dark ones)
  };
  const only = process.argv[3] ? Number(process.argv[3]) : -1;   // `aim-door 3` runs case 3 alone
  let allPass = true;
  const cases: Case[] = [
    /* 0 */ { label: 'server door ajar into the room (-0.9): guard on the corridor east of the frame, Sam lit in the aisle WEST of the leaf (the leaf hides him: he has to go in)', door: 'server', angle: -0.9, guardAt: [18.5, 0, 11.2], sam: [15.5, 0, 8.0], samVis: 0.8, expect: 'hit' },
    /* 1 */ { label: 'server door ajar (-0.9): guard square on the doorway, Sam lit in the aisle east — the eye line clears the east jamb by a hand, the gun hand is nearer it', door: 'server', angle: -0.9, guardAt: [17.5, 0, 11.4], sam: [18.9, 0, 7.6], samVis: 0.8, expect: 'hit' },
    /* 2 */ { label: 'server door swung out into the corridor (+1.7, out of the way): guard by the EAST jamb, Sam lit north-west in the aisle — eye line 15 cm inside the jamb', door: 'server', angle: 1.7, guardAt: [18.6, 0, 10.9], sam: [16.8, 0, 8.0], samVis: 0.8, expect: 'hit' },
    /* 3 */ { label: 'server door folded into the room (-1.92): guard by the WEST jamb, Sam lit north-east in the aisle', door: 'server', angle: -1.92, guardAt: [16.5, 0, 10.9], sam: [18.6, 0, 8.0], samVis: 0.8, expect: 'hit' },
    /* 4 */ { label: 'manager door as authored (+0.7, out into the corridor): guard west of it on the corridor, Sam lit inside the office (control: nothing in the way)', door: 'manager', angle: null, guardAt: [23.6, 0, 11.0], sam: [27.0, 0, 6.5], samVis: 0.8, expect: 'hit' },
    /* 5 */ { label: 'conference door as authored (-1.25): guard at the door, Sam lit behind the table — seen and shot through the glazing (control)', door: 'conference', angle: null, guardAt: [9.7, 0, 10.9], sam: [7.0, 0, 5.4], samVis: 0.8, expect: 'hit' },
    /* 6 */ { label: 'server door SHUT (latched, keyed against the corridor): guard on the corridor with a 1 s old fix on Sam inside, Sam lit in the aisle', door: 'server', angle: 0, lock: true, guardAt: [19.5, 0, 11.3], sam: [18.5, 0, 8.0], samVis: 0.8, seenT: 1.0, expect: 'hit' },
    /* 7 */ { label: 'break-room corridor door SHUT (latched, unlocked): guard on the corridor with a 1 s old fix on Sam inside, Sam DARK inside (no licence to shoot: he has to walk up to him)', door: 'breakroom_n', angle: 0, guardAt: [27.5, 0, 11.3], sam: [31.5, 0, 15.5], samVis: 0.08, seenT: 1.0, expect: 'enter' },
    /* 8 */ { label: 'break-room corridor door SHUT: Sam DARK planted right behind the leaf (his body is what it opens against)', door: 'breakroom_n', angle: 0, guardAt: [27.5, 0, 11.3], sam: [30.6, 0, 12.75], samVis: 0.08, seenT: 1.0, samHoldsLeaf: true, expect: 'enter' },
    /* 9 */ { label: 'server door ajar (-0.9): guard square on the doorway, Sam DARK in the aisle east (faint glimpses move the fix but never license a shot: he has to walk in through the leaf)', door: 'server', angle: -0.9, guardAt: [17.5, 0, 11.4], sam: [18.9, 0, 7.6], samVis: 0.12, expect: 'enter' },
    /* 10 */ { label: 'server door SHUT and LOCKED, guard IN the doorway a foot off the leaf facing it, fresh sighting, Sam lit just the other side of it (2 m)', door: 'server', angle: 0, lock: true, guardAt: [17.6, 0, 10.55], sam: [17.8, 0, 8.4], samVis: 0.8, seenT: 0.2, expect: 'hit' },
    /* 11 */ { label: 'break-room WEST door standing open as authored (+1.35 into the aisle): guard in the cubicle aisle west of it, Sam lit inside the break room east of the doorway', door: 'breakroom_w', angle: null, guardAt: [26.4, 0, 18.9], sam: [31.0, 0, 16.6], samVis: 0.8, expect: 'hit' },
  ];
  for (let ci = 0; ci < cases.length; ci++) {
    if (only >= 0 && ci !== only) continue;
    const c = cases[ci];
    Math.random = seeded(13);
    const W = await standUp(); const { game } = W;
    game.godMode = true; game.aiEnabled = true;
    for (let i = 0; i < 10; i++) W.step(DT);
    const d = game.doors.byName(c.door);
    if (c.angle !== null && c.angle !== undefined) stageLeaf(d, c.angle, !!c.lock); else if (c.lock) d.locked = true;
    const gd = parkOthers(game);
    game.teleportPlayer(v3.copy(c.sam)); game.player.crouch = !!c.samCrouch; game.player.visibility = c.samVis;
    const sam = v3.copy(game.player.char.pos);   // where the teleport actually stood him
    game.puppet = { goal: c.samHoldsLeaf ? v3.copy(sam) : null, aim: [c.guardAt[0], 1.2, c.guardAt[2]] };   // Sam faces the doorway, pistol up (the screenshot); holding the leaf: keeps walking back onto his spot
    placeGuard(gd, c.guardAt, sam); armAlert(gd, game, sam, c.seenT ?? 0);
    const reach = game.col.nav.findPath(gd.char.pos, sam);
    W.step(DT);
    const shots0 = W.audioCounts.get('pistolLoud') ?? 0; let hits = 0; let firstHitAt = -1; const t0 = game.time;
    const fc = d.frameCentre; const rows: string[] = []; let plantedNoFire = 0; let maxPlanted = 0; let lastShots = shots0; let lastShotT = game.time; let enteredAt = -1; const stateAt = new Map<string, number>();
    let alertNearDoor = 0, maxAlertNearDoor = 0;   // the smell: alert, within 2.2 m of the doorway's middle, covering < 0.25 m/s, and no round of his has reached Sam for 2 s
    let plantSeeing = 0, maxPlantSeeing = 0;       // and its core whatever the distance: alert, planted, eyes on Sam within the last half second, nothing arriving for 2 s
    let lastHitT = game.time, lastSawT = -100;
    const inRoom = (p: Vec3) => c.door.startsWith('breakroom') ? (p[0] > 28.3 && p[2] > 12.5) : p[2] < 9.7;
    for (let f = 0; f < 60 * (c.secs ?? 24); f++) {
      W.step(DT);
      if (game.player.hitFlash > 0.94) { hits++; lastHitT = game.time; if (firstHitAt < 0) firstHitAt = game.time - t0; }
      if (gd.sawPlayerThisFrame) lastSawT = game.time;
      const shots = (W.audioCounts.get('pistolLoud') ?? 0); if (shots !== lastShots) { lastShots = shots; lastShotT = game.time; }
      if (gd.state === 'alert' && gd.speed < 0.25 && game.time - lastShotT > 1.5) { plantedNoFire += DT; maxPlanted = Math.max(maxPlanted, plantedNoFire); } else plantedNoFire = 0;
      if (gd.state === 'alert' && gd.speed < 0.25 && v3.distXZ(gd.char.pos, fc) < 2.2 && game.time - lastHitT > 2) { alertNearDoor += DT; maxAlertNearDoor = Math.max(maxAlertNearDoor, alertNearDoor); } else alertNearDoor = 0;
      if (gd.state === 'alert' && gd.speed < 0.25 && game.time - lastSawT < 0.5 && game.time - lastHitT > 2) { plantSeeing += DT; maxPlantSeeing = Math.max(maxPlantSeeing, plantSeeing); } else plantSeeing = 0;
      if (enteredAt < 0 && inRoom(gd.char.pos)) enteredAt = game.time - t0;
      if (!stateAt.has(gd.state)) stateAt.set(gd.state, game.time - t0);
      if (f % 30 === 29) {
        const vis = canSee(game, gd); const p = gd.char.pos;
        rows.push(`    ${(game.time - t0).toFixed(1).padStart(4)}s ${gd.state.padEnd(10)} (${p[0].toFixed(2)}, ${p[2].toFixed(2)}) door ${v3.distXZ(p, fc).toFixed(2)} m · Sam ${v3.distXZ(p, game.player.char.pos).toFixed(1)} m · canSee ${vis.visible ? 'Y' : 'n'}${vis.inCone ? '' : '(cone)'} seen ${gd.sawPlayerThisFrame ? 'Y' : 'n'} lastSeen ${(game.time - gd.lastSeenT).toFixed(1)}s · shots ${shots - shots0} hits ${hits} · spd ${gd.speed.toFixed(2)} path ${gd.pathI}/${gd.path.length} stallT ${gd.alertStallT.toFixed(1)} · leaf ${d.angle.toFixed(2)}${d.latched ? 'L' : ''}${d.locked ? 'k' : ''}`);
      }
    }
    const shots = (W.audioCounts.get('pistolLoud') ?? 0) - shots0;
    const positive = c.expect === 'hit' ? firstHitAt >= 0 : enteredAt >= 0 || firstHitAt >= 0;   // (dark cases: in there with him — or already on him, which at arm's length he is even in the dark)
    const pass = maxPlantSeeing < 3 && maxAlertNearDoor < 3 && positive; allPass &&= pass;
    console.log(`\n- ${pass ? 'PASS' : 'FAIL'} [${ci}] ${c.label}\n  Sam stood at (${sam[0].toFixed(2)}, ${sam[2].toFixed(2)})${reach ? '' : ' — NO NAV PATH from the guard to him'}; ${shots} rounds fired, ${hits} reached Sam${firstHitAt >= 0 ? ` (first at ${firstHitAt.toFixed(1)} s)` : ''}; states reached: ${[...stateAt.entries()].map(([s, t]) => `${s}@${t.toFixed(1)}s`).join(' ')}; entered the room: ${enteredAt >= 0 ? enteredAt.toFixed(1) + ' s' : 'never'}\n  longest spell alert + planted with eyes on Sam and nothing reaching him: ${maxPlantSeeing.toFixed(1)} s${maxPlantSeeing >= 3 ? ' ✗' : ''}; alert + planted within 2.2 m of the doorway with nothing reaching him: ${maxAlertNearDoor.toFixed(1)} s${maxAlertNearDoor >= 3 ? ' ✗' : ''}; planted in 'alert' without even firing: ${maxPlanted.toFixed(1)} s${positive ? '' : `; ✗ expected ${c.expect === 'hit' ? 'a round to reach Sam' : 'him inside the room'} and it never happened`}; ends ${gd.state} at (${gd.char.pos[0].toFixed(2)}, ${gd.char.pos[2].toFixed(2)})`);
    console.log(rows.join('\n'));
  }
  console.log(`\n=== aim-door: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

/** The trap behind 'aim stuck in doors', counted over the whole floor: every doorway × a grid of guard spots on his side × Sam spots on the far side × a few leaf
 *  angles. A TRAP is a staging where canSee says yes (eye → chest or head clear) but nothing fired from where he stands can arrive (shoulder / muzzle → chest AND
 *  head both stopped by a jamb, the leaf, a mullion) and he is inside his 9 m stand-and-fire range — the old alert branch planted him there for as long as Sam
 *  stayed put. Each trap is then run live for 6 s from a fresh sighting with Sam lit and rooted (god mode): PASS if a round reaches Sam inside 4 s or he at least
 *  never spends 2.5 s planted with eyes on Sam and nothing arriving; FAIL otherwise (what every trap did before: 0.3‥1 rounds a second into the frame, for ever). */
async function aimDoorSweep() {
  console.log('\n=== aim-door-sweep: every doorway, guard spots × Sam spots × leaf angles — where sight and line of fire disagree, and what he does about it now ===');
  const { canSee, clearShot } = await import(`${ROOT}/src/game/guards.ts`);   // the trap is defined by the AI's own line-of-fire test: wherever that says 'no shot from here', he must not plant
  Math.random = seeded(17);
  const W = await standUp(); const { game } = W;
  game.godMode = true; game.aiEnabled = true;
  for (let i = 0; i < 10; i++) W.step(DT);
  let gd = parkOthers(game);
  const nav = game.col.nav;
  type Way = { door: string; guard: Vec3[]; sam: Vec3[]; angles: (number | null)[] };
  const span = (a: number, b: number, n: number) => Array.from({ length: n }, (_, i) => a + (b - a) * i / (n - 1));
  const northDoor = (name: string, x0: number): Way => ({ door: name, angles: [null, -0.9, -1.9, 0.9],
    guard: span(x0 - 1.4, x0 + 2.6, 9).flatMap(x => [10.45, 10.9, 11.5].map(z => [x, 0, z] as Vec3)),
    sam: span(x0 - 2.6, x0 + 3.8, 9).flatMap(x => [8.9, 7.9, 5.6].map(z => [x, 0, z] as Vec3)) });
  const ways: Way[] = [
    northDoor('conference', 9.0), northDoor('server', 17.0), northDoor('manager', 25.0), northDoor('storage', 32.0),
    { door: 'breakroom_n', angles: [null, 0.9, 1.5, -0.9], guard: span(28.6, 32.6, 9).flatMap(x => [11.75, 11.3, 10.7].map(z => [x, 0, z] as Vec3)), sam: span(28.6, 34.6, 9).flatMap(x => [13.3, 14.6, 17.0].map(z => [x, 0, z] as Vec3)) },
    { door: 'breakroom_w', angles: [null, 0, -1.2, 1.9], guard: span(15.4, 19.8, 9).flatMap(z => [27.5, 26.9, 26.2].map(x => [x, 0, z] as Vec3)), sam: span(14.6, 20.6, 9).flatMap(z => [28.9, 30.4, 32.5].map(x => [x, 0, z] as Vec3)) },
    { door: 'server_manager', angles: [null, 1.2, -1.2, 1.9], guard: span(6.6, 9.8, 7).flatMap(z => [21.5, 20.8].map(x => [x, 0, z] as Vec3)), sam: span(5.0, 9.4, 7).flatMap(z => [22.6, 23.6, 25.5].map(x => [x, 0, z] as Vec3)) },
  ];
  let configs = 0, seenN = 0, unroutable = 0, traps: { door: string; angle: number | null; g: Vec3; s: Vec3 }[] = [];
  for (const w of ways) {
    const d = game.doors.byName(w.door); const a0 = d.angle;   // (as authored: every door is reset() after its own angles are done)
    for (const ang of w.angles) {
      stageLeaf(d, ang ?? a0);
      for (const gp of w.guard) {
        if (nav.isBlocked(gp[0], gp[2])) continue;
        for (const sp of w.sam) {
          if (nav.isBlocked(sp[0], sp[2])) continue;
          configs++;
          game.player.char.pos = v3.copy(sp); game.player.crouch = false; game.player.char.bodyYaw = game.player.char.aimYaw = Math.atan2(gp[0] - sp[0], gp[2] - sp[2]); game.player.char.update(0);
          placeGuard(gd, gp, sp);
          const vis = canSee(game, gd); if (!vis.visible || vis.dist > 9) continue;
          seenN++;
          const pc = game.player.char; const chest = v3.add(pc.pos, [0, 1.25, 0]), head = pc.bones.head ?? v3.add(pc.pos, [0, 1.6, 0]);
          if (!clearShot(game, gd.char, chest) && !clearShot(game, gd.char, head)) { if (nav.findPath(gp, sp)) traps.push({ door: w.door, angle: ang, g: v3.copy(gp), s: v3.copy(sp) }); else unroutable++; }   // (in sight where no route reaches — Sam put behind the racks by the harness's hand — is door-rules' case 6, not a doorway a man can walk out of)
        }
      }
    }
    d.reset();
  }
  console.log(`  ${configs} stagings, ${seenN} with Sam in plain sight inside 9 m, ${traps.length} of those with NO line of fire from where the man stands (${(100 * traps.length / Math.max(1, seenN)).toFixed(1)} %${unroutable ? `; ${unroutable} more with no route to him at all, left to door-rules` : ''}) — by door: ${[...new Set(traps.map(t => t.door))].map(n => `${n} ${traps.filter(t => t.door === n).length}`).join(', ')}`);
  // live: each trap from a fresh sighting, Sam lit and rooted where he is
  let pass = 0, fail = 0; const failures: string[] = []; let worstPlant = 0; const firstHits: number[] = []; let shotsTotal = 0, twistedShots = 0; let worstTwist = 0;
  { const play = game.audio.play; game.audio.play = (n: string, ...rest: unknown[]) => { if (n === 'pistolLoud') { shotsTotal++; const tw = Math.abs(Math.atan2(Math.sin(gd.char.bodyYaw - gd.char.aimYaw), Math.cos(gd.char.bodyYaw - gd.char.aimYaw))); if (Math.hypot(gd.char.vel[0], gd.char.vel[2]) > 0.3) { worstTwist = Math.max(worstTwist, tw); if (tw > 0.9 + 1e-6) twistedShots++; } } return play(n, ...rest); }; }   // (7: moving = his feet actually carrying him this frame, not the walk-blend value easing out after he plants; read at the very moment the round leaves — fireWeapon plays the shot from inside the guard's update — so these are the yaws the fire gate saw)
  for (const t of traps) {
    const d = game.doors.byName(t.door); const a = t.angle ?? d.def.angle ?? 0;
    for (const dd of game.doors.list) dd.reset();
    stageLeaf(d, a);
    game.clearAftermath(); game.resetGuards(); game.player.hitFlash = 0; gd = parkOthers(game);   // fresh men every trap: nothing of the last one's reload, stuck clock or aim carries into this one
    game.teleportPlayer(v3.copy(t.s)); game.player.visibility = 0.85; game.puppet = { goal: v3.copy(t.s), aim: [t.g[0], 1.2, t.g[2]] };
    placeGuard(gd, t.g, t.s); armAlert(gd, game, t.s, 0, 0, 0.2);
    let firstHit = -1, plant = 0, maxPlant = 0, ground = 0; const t0 = game.time; let last = v3.copy(gd.char.pos);
    for (let f = 0; f < 60 * 6 && firstHit < 0; f++) {
      W.step(DT);
      if (game.player.hitFlash > 0.94 && firstHit < 0) firstHit = game.time - t0;
      if (gd.state === 'alert' && gd.speed < 0.25 && gd.sawPlayerThisFrame) { plant += DT; maxPlant = Math.max(maxPlant, plant); } else plant = 0;
      ground += v3.distXZ(gd.char.pos, last); last = v3.copy(gd.char.pos);
    }
    worstPlant = Math.max(worstPlant, maxPlant); if (firstHit >= 0) firstHits.push(firstHit);
    const ok = (firstHit >= 0 && firstHit < 5) || (maxPlant < 2.5 && ground > 1.0);   // a round on him inside five seconds — or, failing that, never planted with eyes on him AND actually on the move (a metre of ground: closing for a line, or in through the door after losing him) — not merely 'did nothing wrong standing still'
    if (ok) pass++; else { fail++; if (failures.length < 12) failures.push(`${t.door} @ ${t.angle === null ? 'authored' : t.angle}: guard (${t.g[0].toFixed(2)}, ${t.g[2].toFixed(2)}) Sam (${t.s[0].toFixed(2)}, ${t.s[2].toFixed(2)}) — ${firstHit >= 0 ? `first hit ${firstHit.toFixed(1)} s` : 'nothing reached Sam in 6 s'}, planted seeing him ${maxPlant.toFixed(1)} s, covered ${ground.toFixed(1)} m, ends ${gd.state} at (${gd.char.pos[0].toFixed(2)}, ${gd.char.pos[2].toFixed(2)}) spd ${gd.speed.toFixed(2)} path ${gd.pathI}/${gd.path.length}`); }
  }
  game.puppet = null;
  firstHits.sort((x, y) => x - y); const med = firstHits.length ? firstHits[Math.floor(firstHits.length / 2)] : NaN; const p90 = firstHits.length ? firstHits[Math.floor(firstHits.length * 0.9)] : NaN; const mx = firstHits.length ? firstHits[firstHits.length - 1] : NaN;
  console.log(`  live, per trap (6 s each, fresh sighting, Sam lit and rooted): ${pass} PASS · ${fail} FAIL; a round reached Sam in ${firstHits.length}/${traps.length} (median ${med.toFixed(1)} s, p90 ${p90.toFixed(1)} s, max ${mx.toFixed(1)} s); worst spell planted with eyes on him and nothing arriving ${worstPlant.toFixed(1)} s`);
  if (failures.length) console.log('  ✗ ' + failures.join('\n  ✗ '));
  console.log(`  ${shotsTotal} rounds fired over the live traps, ${twistedShots} of them by a moving man more than 0.9 rad across his own chest${twistedShots ? ' ✗' : ''} (worst twist on the move ${worstTwist.toFixed(2)} rad)`);
  const enough = traps.length >= 100;   // (418 on the floor as authored: a sweep that finds next to none is a broken trap test — clearShot gone permissive — not a clean bill)
  if (!enough) console.log(`  ✗ only ${traps.length} traps found — the line-of-fire test has stopped disagreeing with the line of sight; the sweep proves nothing`);
  console.log(`=== aim-door-sweep: ${fail === 0 && enough && !twistedShots ? 'ALL PASS' : 'FAILURES above'} ===`);
  return fail === 0 && enough && !twistedShots;
}

/** The door shut in his face: Sam, seen on the corridor, runs into the break room and pulls the door to behind him (what his F tap does), then (a) goes dark deep
 *  in the room, (b) stands lit in the middle of it, (c) waits aside in the dark for the latch to catch and puts his back to the shut leaf. The alert man chasing
 *  him has a key: he should be through that door within a few seconds of Sam every time — pushing it open onto Sam if Sam is what it opens against — and then
 *  either have him (lit / at arm's length) or search the room, never stand covering the shut leaf from the corridor or sweep the corridor he is standing in.
 *  Each case over three seeds (the search wander and the spread are dice); PASS when, on every seed, he is inside — or a round of his has reached Sam — within
 *  8 s of Sam being where he was going. */
async function aimDoorChase() {
  console.log('\n=== aim-door-chase: Sam runs into the break room and shuts the door on an alert man chasing him ===');
  const { PLAYER_ID } = await import(`${ROOT}/src/game/consts.ts`);
  type Case = { label: string; then: Vec3; vis: number; holdLeaf?: boolean; react: number };   // react: his 'CONTACT!' beat before he moves (a long one buys the door time to latch for (c))
  const cases: Case[] = [
    { label: '(a) dark, deep in the room by the vending machines', then: [33.6, 0, 16.4], vis: 0.08, react: 0.4 },
    { label: '(b) lit, standing in the middle of the room facing the door', then: [32.0, 0, 15.2], vis: 0.8, react: 0.4 },
    { label: '(c) dark, back against the leaf once it has latched (the man held on his CONTACT beat for 3 s, so it has time to)', then: [30.62, 0, 12.72], vis: 0.08, holdLeaf: true, react: 3.0 },
  ];
  let allPass = true;
  for (const c of cases) {
    const results: string[] = []; let casePass = true; let firstRows: string[] = [];
    for (const seed of [23, 24, 25]) {
      Math.random = seeded(seed);
      const W = await standUp(); const { game, input } = W;
      game.godMode = true; game.aiEnabled = true;
      for (let i = 0; i < 10; i++) W.step(DT);
      const d = game.doors.byName('breakroom_n');
      const gd = parkOthers(game);
      game.teleportPlayer([27.2, 0, 11.3]); game.player.visibility = 0.9;
      placeGuard(gd, [19.5, 0, 11.2], game.player.char.pos); armAlert(gd, game, game.player.char.pos, 0, c.react, 0.8);
      // Sam legs it: to the doorway, through it a body clear of the swing — the door pulled to behind him there — then to his spot; with his back to go against the
      // leaf he steps aside first and waits for the latch to catch (a leaf swinging to onto him would only come to rest against him), then plants himself behind it
      const legs: Vec3[] = c.holdLeaf ? [[30.6, 0, 11.6], [30.6, 0, 13.4], [31.9, 0, 13.9], c.then] : [[30.6, 0, 11.6], [30.6, 0, 13.4], c.then];
      let leg = 0; let shutAt = -1, samIn = -1, samSet = -1, guardIn = -1, firstHit = -1; const t0 = game.time; const hitTimes: number[] = [];
      game.puppet = { goal: v3.copy(legs[0]), aim: null }; input.keys.add('ShiftLeft');
      const rows: string[] = []; let coverT = 0, maxCover = 0; const shots0 = W.audioCounts.get('pistolLoud') ?? 0; let hits = 0;
      for (let f = 0; f < 60 * 24; f++) {
        const p = game.player.char.pos;
        if (leg < legs.length && v3.distXZ(p, legs[leg]) < 0.35) { leg++; if (leg < legs.length) game.puppet!.goal = v3.copy(legs[leg]); else { game.puppet!.goal = c.holdLeaf ? v3.copy(c.then) : null; game.puppet!.aim = [30.6, 1.2, 11.0]; input.keys.delete('ShiftLeft'); game.player.visibility = c.vis; } }
        if (samIn < 0 && p[2] > 12.6 && p[0] > 29) samIn = game.time - t0;
        if (shutAt < 0 && samIn >= 0 && leg >= 2) { game.doors.pullTo(d, PLAYER_ID); shutAt = game.time - t0; game.player.visibility = c.vis; input.keys.delete('ShiftLeft'); }
        if (c.holdLeaf && leg === legs.length - 1) game.puppet!.goal = d.latched ? v3.copy(c.then) : null;   // aside, out of the swing, until it has caught
        if (samSet < 0 && leg >= legs.length - 1 && v3.distXZ(p, c.then) < 0.4) samSet = game.time - t0;    // Sam is where he was going (behind the latched leaf, for (c))
        W.step(DT);
        if (game.player.hitFlash > 0.94) { hits++; hitTimes.push(game.time - t0); if (firstHit < 0) firstHit = game.time - t0; }
        const q = gd.char.pos; if (guardIn < 0 && samIn >= 0 && q[2] > 12.55 && q[0] > 29.5 && q[0] < 31.6) guardIn = game.time - t0;   // (in after Sam, not ahead of him)
        if (gd.state === 'alert' && gd.speed < 0.25 && q[2] < 12.4 && v3.distXZ(q, d.frameCentre) < 2.0) { coverT += DT; maxCover = Math.max(maxCover, coverT); } else coverT = 0;   // planted on the corridor side within reach of the doorway
        if (f % 30 === 29) rows.push(`    ${(game.time - t0).toFixed(1).padStart(4)}s Sam (${p[0].toFixed(1)}, ${p[2].toFixed(1)}) vis ${game.player.visibility.toFixed(2)} · guard ${gd.state.padEnd(6)} (${q[0].toFixed(2)}, ${q[2].toFixed(2)}) spd ${gd.speed.toFixed(2)} seen ${gd.sawPlayerThisFrame ? 'Y' : 'n'} lastSeen ${(game.time - gd.lastSeenT).toFixed(1)}s fix (${gd.lastKnown?.[0].toFixed(1)}, ${gd.lastKnown?.[2].toFixed(1)}) path ${gd.pathI}/${gd.path.length} · shots ${(W.audioCounts.get('pistolLoud') ?? 0) - shots0} hits ${hits} · leaf ${d.angle.toFixed(2)}${d.latched ? 'L' : ''}${d.closing ? 'c' : ''}`);
      }
      input.keys.delete('ShiftLeft'); game.puppet = null;
      const shots = (W.audioCounts.get('pistolLoud') ?? 0) - shots0;
      const tRef = c.holdLeaf ? samSet : samIn;   // from Sam being inside — or, for (c), from Sam being planted behind the latched leaf
      const hitAfter = tRef >= 0 ? hitTimes.find(h => h >= tRef) ?? -1 : -1;   // a round that tagged him on the corridor before the door was between them says nothing about the door
      const pass = tRef >= 0 && (!c.holdLeaf || shutAt >= 0) && ((guardIn >= tRef && guardIn - tRef < 8) || (hitAfter >= 0 && hitAfter - tRef < 8)); casePass &&= pass;
      results.push(`    seed ${seed}: ${pass ? 'PASS' : 'FAIL'} — Sam inside at ${samIn >= 0 ? samIn.toFixed(1) + ' s' : 'never'}, door pulled to at ${shutAt >= 0 ? shutAt.toFixed(1) + ' s' : 'never'}${c.holdLeaf ? `, behind the latched leaf at ${samSet >= 0 ? samSet.toFixed(1) + ' s' : 'NEVER (staging)'}` : ''}; guard inside at ${guardIn >= 0 ? `${guardIn.toFixed(1)} s (+${(guardIn - tRef).toFixed(1)} s)` : 'NEVER'}; ${shots} rounds fired, ${hits} reached Sam${firstHit >= 0 ? ` (first at ${firstHit.toFixed(1)} s)` : ''}; longest spell planted alert on the corridor side of the doorway ${maxCover.toFixed(1)} s; ends ${gd.state} at (${gd.char.pos[0].toFixed(2)}, ${gd.char.pos[2].toFixed(2)})`);
      if (!firstRows.length) firstRows = rows;
    }
    allPass &&= casePass;
    console.log(`\n- ${casePass ? 'PASS' : 'FAIL'} — ${c.label}\n${results.join('\n')}\n    (seed 23 timeline)\n${firstRows.join('\n')}`);
  }
  console.log(`=== aim-door-chase: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

/** The alert no-progress exit measured against a fix that has MOVED: a man three metres short of an old fix on the west corridor when a colleague's shots put Sam
 *  at the east end. He should run the length of the corridor to it; measured against the old fix the run was 'five seconds without getting nearer than he had
 *  been' and he pulled up half way with 'where did he go?'. PASS if he gets within 3 m of the new fix (or only drops to 'search' there). */
async function farRefix() {
  console.log('\n=== far-refix: an alert man re-targeted across the floor must not give up half way there ===');
  Math.random = seeded(29);
  const W = await standUp(); const { game } = W;
  game.godMode = true; game.aiEnabled = true; game.playerInvisible = false;
  for (let i = 0; i < 10; i++) W.step(DT);
  const gd = parkOthers(game);
  game.teleportPlayer([8.0, 0, 11.2]); game.player.visibility = 0.02;   // where the old fix is; dark
  placeGuard(gd, [12.5, 0, 11.2], [8.0, 0, 11.2]); armAlert(gd, game, [8.0, 0, 11.2], 3);
  for (let i = 0; i < 40; i++) W.step(DT);   // on his way west, alertBest now a few metres
  const best0 = gd.alertBest;
  game.teleportPlayer([33.5, 0, 11.4]); game.player.visibility = 0.02;
  game.events.push({ kind: 'guardShot', pos: [30.0, 0, 11.3], time: game.time, loud: true, level: 1 });   // a colleague at the east end opens up on Sam: every guard's fix becomes Sam's position
  const NEW: Vec3 = [33.5, 0, 11.4]; const t0 = game.time; let searchAt = -1; let searchPos: Vec3 | null = null; let closest = 99; const rows: string[] = [];
  for (let f = 0; f < 60 * 16; f++) {
    if (f === 2) game.playerInvisible = true;   // (the fix is taken; from here this is about the run to it, not about finding Sam standing on it)
    W.step(DT);
    closest = Math.min(closest, v3.distXZ(gd.char.pos, NEW));
    if (searchAt < 0 && gd.state !== 'alert') { searchAt = game.time - t0; searchPos = v3.copy(gd.char.pos); }
    if (f % 60 === 59) rows.push(`    ${(game.time - t0).toFixed(0).padStart(3)}s ${gd.state.padEnd(6)} (${gd.char.pos[0].toFixed(1)}, ${gd.char.pos[2].toFixed(1)}) fix (${gd.lastKnown?.[0].toFixed(1)}, ${gd.lastKnown?.[2].toFixed(1)}) best ${gd.alertBest > 1e8 ? '—' : gd.alertBest.toFixed(1)} stallT ${gd.alertStallT.toFixed(1)} spd ${gd.speed.toFixed(2)}`);
  }
  const shortBy = searchPos ? v3.distXZ(searchPos, NEW) : closest;   // where he let the chase go (or how near he got if he never did)
  const pass = shortBy < 3;
  console.log(`- ${pass ? 'PASS' : 'FAIL'} — alertBest was ${best0.toFixed(1)} m against the old fix when the shots came; ${searchAt >= 0 ? `dropped to search at +${searchAt.toFixed(1)} s at (${searchPos![0].toFixed(1)}, ${searchPos![2].toFixed(1)}), ${shortBy.toFixed(1)} m short of the new fix` : `still alert after 16 s, ${closest.toFixed(1)} m from the new fix at the nearest`}`);
  console.log(rows.join('\n'));
  console.log(`=== far-refix: ${pass ? 'PASS' : 'FAIL'} ===`);
  return pass;
}

/** The rules round the doorway fixes, one staged check each (PASS / FAIL):
 *  1 shut-but-unlatched leaf between two pressed men: no contact kill through the wood; the same two with the leaf standing open: the round lands;
 *  2 a licensed man beside a wall with Sam clear 90° round inside arm's-length cone: he squares up and fires without stepping off his spot;
 *  3 a pinned alert man with no fix (a beat's posted shooter before his cue): stays alert, says nothing;
 *  4 losing Sam to his own head-sweep / turning away is not losing him behind anything: the fix stays on the man, not led up the corridor;
 *  5 a stacked pair (no. 2 half a metre behind no. 1, both covering the corridor ahead): no. 2's pistol goes to the low ready for the man in his line (muzzle
 *    discipline, guards.ts muzzleCheck) and his laser goes off with it; no. 1, nobody ahead of him, keeps his up and his beam runs the corridor;
 *  6 Sam in plain sight where no route reaches (the strip behind the server racks, put there by hand): never a round into the rack he cannot shoot past, and no
 *    standing at '!' for good — he lets it go inside a few seconds (and, seeing him still, may call it again: that cycle is the honest state of it);
 *  7 over the whole doorway sweep, no round ever leaves a moving man more than 0.9 rad across his own chest (aimDoorSweep counts it; asserted there). */
async function doorRules() {
  console.log('\n=== door-rules: contact through wood, squaring up in place, pinned with no fix, cone loss, stacked lasers, unroutable sight ===');
  const { fireWeapon, laserBoxes } = await import(`${ROOT}/src/game/combat.ts`);
  const { canSee, clearShot } = await import(`${ROOT}/src/game/guards.ts`);
  let allPass = true; const verdict = (ok: boolean, label: string, detail: string) => { allPass &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}: ${detail}`); };
  const fresh = async (seed: number) => { Math.random = seeded(seed); const W = await standUp(); W.game.godMode = true; W.game.aiEnabled = true; for (let i = 0; i < 10; i++) W.step(DT); return W; };
  // 1 — contact through a shut-but-unlatched leaf
  {
    const W = await fresh(81); const { game } = W; const gd = parkOthers(game); const d = game.doors.byName('breakroom_n');
    game.teleportPlayer([30.6, 0, 12.62]); game.player.visibility = 0.9; game.puppet = { goal: [30.6, 0, 12.62], aim: null };
    placeGuard(gd, [30.6, 0, 11.83], game.player.char.pos); armAlert(gd, game, game.player.char.pos, 0, 0, 99); gd.pinned = true;
    stageLeaf(d, 0.08); d.closing = true;   // pulled to and all but home, off the latch: a full panel between them
    for (let i = 0; i < 30; i++) { gd.lastSeenT = game.time; W.step(DT); d.angle = 0.08; d.latched = false; d.place(); }   // (held there: aim comes up onto Sam through it)
    const ax = Math.hypot(gd.char.muzzle[0] - game.player.char.pos[0], gd.char.muzzle[2] - game.player.char.pos[2]);
    const chest = () => v3.add(game.player.char.pos, [0, 1.25, 0]);
    let shut = 0; for (let k = 0; k < 12; k++) { fireWeapon(game, gd.char, chest(), false); if (game.player.hitFlash > 0.99) shut++; game.player.hitFlash = 0; }
    stageLeaf(d, 1.35); let open = 0; for (let k = 0; k < 12; k++) { fireWeapon(game, gd.char, chest(), false); if (game.player.hitFlash > 0.99) open++; game.player.hitFlash = 0; }
    verdict(shut === 0 && open >= 10, '1 contact through a shut-but-unlatched leaf', `centres ${v3.distXZ(gd.char.pos, game.player.char.pos).toFixed(2)} m, muzzle ${ax.toFixed(2)} m off Sam's axis (inside 0.28: ${ax < 0.28}); rounds landing through the leaf at 0.08 rad: ${shut}/12 (want 0), with it standing open: ${open}/12 (want ≥ 10)`);
    game.puppet = null;
  }
  // 2 — squaring up in place beside a wall
  {
    const W = await fresh(82); const { game } = W; const gd = parkOthers(game);
    game.teleportPlayer([20.4, 0, 10.9]); game.player.visibility = 0.9; game.puppet = { goal: [20.4, 0, 10.9], aim: [17.6, 1.2, 10.6] };
    placeGuard(gd, [17.6, 0, 10.6], [17.6, 0, 8.0]);   // in the server doorway's mouth, squared NORTH into the room, the east jamb at his right shoulder; Sam 2.8 m EAST down the corridor
    armAlert(gd, game, game.player.char.pos, 0, 0, 0.3);
    const p0 = v3.copy(gd.char.pos); let maxOff = 0, shots0 = W.audioCounts.get('pistolLoud') ?? 0, firstShot = -1; const t0 = game.time; let vis0 = false;
    for (let f = 0; f < 120; f++) { W.step(DT); if (f === 0) vis0 = canSee(game, gd).visible; maxOff = Math.max(maxOff, v3.distXZ(gd.char.pos, p0)); if (firstShot < 0 && (W.audioCounts.get('pistolLoud') ?? 0) > shots0) firstShot = game.time - t0; }
    verdict(vis0 && maxOff < 0.12 && firstShot >= 0, '2 licensed with Sam 90° round beside the jamb', `in his cone at the start: ${vis0}; furthest he got off his spot in 2 s: ${maxOff.toFixed(2)} m (want < 0.12); first round at ${firstShot >= 0 ? firstShot.toFixed(2) + ' s' : 'never'}; shoulder line clear: ${clearShot(game, gd.char, v3.add(game.player.char.pos, [0, 1.25, 0]))}`);
    game.puppet = null;
  }
  // 3 — pinned alert man with no fix
  {
    const W = await fresh(83); const { game } = W; const gd = parkOthers(game);
    game.teleportPlayer([6.0, 0, 20.0]); game.playerInvisible = true;
    placeGuard(gd, [20.0, 0, 11.2], [10, 0, 11.2]); armAlert(gd, game, [10, 0, 11.2], 5); gd.lastKnown = null; gd.pinned = true;
    let barks = 0; const orig = game.say.bind(game); game.say = (who: any, text: string, radio = false) => { if (who === gd) barks++; return orig(who, text, radio); };
    for (let f = 0; f < 60 * 6; f++) W.step(DT);
    verdict(gd.state === 'alert' && barks === 0 && v3.distXZ(gd.char.pos, [20, 0, 11.2]) < 0.05, '3 pinned at alert with no fix for 6 s', `state ${gd.state}, barks ${barks}, moved ${v3.distXZ(gd.char.pos, [20, 0, 11.2]).toFixed(2)} m`);
    // and the free man's version: not pinned, no fix, sighting stale → he does let it go (quietly)
    gd.pinned = false; for (let f = 0; f < 30; f++) W.step(DT);
    verdict(gd.state !== 'alert', '3b the same man freed', `state after half a second: ${gd.state}`);
  }
  // 4 — cone loss is not a loss behind anything: no lead
  {
    const W = await fresh(84); const { game } = W; const gd = parkOthers(game);
    game.teleportPlayer([16.0, 0, 11.6]); game.player.visibility = 0.5; game.puppet = { goal: [26.0, 0, 11.6], aim: null, walk: 1.0 };   // Sam walks east along the corridor at a walk, lit enough to be held in view
    placeGuard(gd, [21.0, 0, 10.6], [16.0, 0, 11.6]); gd.hold = true; gd.state = 'patrol'; gd.awareness = 0;   // held on the north wall watching him come (patrol + hold: he stands and looks)
    game.aiEnabled = true; let sawT = -1;
    for (let f = 0; f < 90; f++) { W.step(DT); if (gd.sightT === game.time && sawT < 0) sawT = game.time; gd.awareness = Math.min(gd.awareness, 0.6); }   // (capped under alert: this is about the fix, not the fight)
    const had = gd.sightT > 0 && game.time - gd.sightT < 0.05;
    // now swing his head and body clean off Sam for good: the next frame is a loss by cone, Sam still in the open corridor
    const samAtLoss = v3.copy(game.player.char.pos); let fixAfter: Vec3 | null = null;
    for (let f = 0; f < 20; f++) { gd.char.bodyYaw = gd.char.aimYaw = 0; W.step(DT); gd.char.bodyYaw = gd.char.aimYaw = 0; if (f === 2) fixAfter = gd.lastKnown ? v3.copy(gd.lastKnown) : null; gd.awareness = Math.min(gd.awareness, 0.6); }
    const off = fixAfter ? v3.distXZ(fixAfter, samAtLoss) : 99;
    verdict(had && off < 0.25, '4 lost to his own turn-away, Sam still in the open', `had him in view up to the turn: ${had}; fix just after the loss is ${off.toFixed(2)} m from where Sam was then (want < 0.25: on the man, not led ${'~1 m'} up the corridor)`);
    game.puppet = null;
  }
  // 5 — stacked pair: no. 2 lowers for the man ahead (his laser off with it), no. 1 keeps his beam down the corridor
  {
    const { handProps } = await import(`${ROOT}/src/game/combat.ts`); const { BoxFlag: BF } = await import(`${ROOT}/src/scene/boxes.ts`);
    const W = await fresh(85); const { game } = W; parkOthers(game); const [a, b] = game.guards;
    for (const o of [a, b]) { o.hold = false; o.pinned = true; o.dazzledUntil = -1; }
    placeGuard(a, [20.55, 0, 11.2], [34.0, 0, 11.2]); placeGuard(b, [20.0, 0, 11.2], [34.0, 0, 11.2]);   // in file facing east down the corridor, no. 2 half a metre behind no. 1, fourteen metres of open corridor ahead
    armAlert(a, game, [34, 0, 11.2], 3); armAlert(b, game, [34, 0, 11.2], 3); a.pinned = b.pinned = true;
    for (let f = 0; f < 40; f++) W.step(DT);
    const outA: any[] = []; laserBoxes(game, a, outA); const lenA = outA.length ? outA[0].h[2] * 2 : 0;
    const hp: any[] = []; handProps(game, hp); const beams = hp.filter((bx: any) => (bx.flags & BF.NoShadow) && bx.h[0] < 0.02 && bx.h[2] > 0.25 && bx.emissive[0] > 4).length;   // hair-thin long emissive NoShadow boxes = laser beams actually drawn this frame
    verdict(b.muzzleDown && !a.muzzleDown && b.char.anim.stance === 'lowReady' && b.char.gunDir[1] < -0.4 && lenA > 2.0 && beams === 1, '5 stacked pair, no. 2 half a metre behind no. 1', `no. 2 muzzleDown ${b.muzzleDown} (stance ${b.char.anim.stance}/${b.char.anim.upper}, gun axis y ${b.char.gunDir[1].toFixed(2)} — want lowReady, < −0.4), no. 1 muzzleDown ${a.muzzleDown} (want false); no. 1's beam ${lenA.toFixed(1)} m (want > 2); laser beams drawn by handProps: ${beams} (want 1: no. 2's is off while his gun is down)`);
  }
  // 6 — in plain sight where no route reaches
  {
    const W = await fresh(86); const { game } = W; let gd = parkOthers(game); const nav = game.col.nav;
    // hunt a staging: Sam somewhere along the strip north of the racks (the hand of the harness puts him there; no man's route does), a guard spot in the aisle that
    // sees him — and prefer one whose shoulder line is blocked (the trap proper); settle for a clear one (then he simply shoots him: nothing to test but no harm)
    let pick: { g: Vec3; s: Vec3; blocked: boolean } | null = null;
    search: for (const blockedWanted of [true, false]) for (let sx = 14.6; sx <= 21.4; sx += 0.4) for (const sz of [4.7, 5.2, 5.6]) for (let gx = 14.8; gx <= 21.2; gx += 0.4) for (const gz of [7.7, 8.3, 8.9]) {
      const sp: Vec3 = [sx, 0, sz], gp: Vec3 = [gx, 0, gz];
      if (nav.isBlocked(gp[0], gp[2]) || nav.findPath(gp, sp)) continue;   // want: NO route
      game.player.char.pos = v3.copy(sp); game.player.crouch = false; game.player.char.update(0); placeGuard(gd, gp, sp);
      const vis = canSee(game, gd); if (!vis.visible || vis.dist > 9) continue;
      const pc = game.player.char; const bl = !clearShot(game, gd.char, v3.add(pc.pos, [0, 1.25, 0])) && !clearShot(game, gd.char, pc.bones.head ?? v3.add(pc.pos, [0, 1.6, 0]));
      if (bl === blockedWanted) { pick = { g: gp, s: sp, blocked: bl }; break search; }
    }
    if (!pick) verdict(true, '6 in sight, no route', 'no such staging exists on this floor (nothing to trap him) — vacuous PASS');
    else {
      game.clearAftermath(); game.resetGuards(); gd = parkOthers(game);
      game.teleportPlayer(v3.copy(pick.s)); game.player.char.pos = v3.copy(pick.s); game.player.char.update(0); game.player.visibility = 0.9; game.puppet = { goal: v3.copy(pick.s), aim: [pick.g[0], 1.2, pick.g[2]] };
      placeGuard(gd, pick.g, pick.s); armAlert(gd, game, pick.s, 0, 0, 0.2);
      const shots0 = W.audioCounts.get('pistolLoud') ?? 0; let hits = 0, leftAlertAt = -1, blockedShots = 0; const t0 = game.time; let lastShots = shots0; let alertTime = 0;
      for (let f = 0; f < 60 * 12; f++) {
        W.step(DT);
        if (game.player.hitFlash > 0.94) hits++;
        const sh = W.audioCounts.get('pistolLoud') ?? 0; if (sh > lastShots) { lastShots = sh; if (gd.shotBlockedT > 0) blockedShots++; }
        if (gd.state === 'alert') alertTime += DT; else if (leftAlertAt < 0) leftAlertAt = game.time - t0;
      }
      const shots = (W.audioCounts.get('pistolLoud') ?? 0) - shots0;
      const ok = blockedShots === 0 && (shots > 0 || (leftAlertAt >= 0 && leftAlertAt < 7));   // never a round down a line he holds blocked; and either he had a line and used it, or he let the '!' go inside a few seconds — never twelve seconds rooted and silent
      verdict(ok, `6 in sight, no route (${pick.blocked ? 'staged with no line of fire' : 'line of fire clear'})`, `guard (${pick.g[0].toFixed(1)}, ${pick.g[2].toFixed(1)}) Sam (${pick.s[0].toFixed(1)}, ${pick.s[2].toFixed(1)}): ${shots} rounds (${blockedShots} on a blocked line — want 0), ${hits} reached Sam; first left 'alert' at ${leftAlertAt >= 0 ? leftAlertAt.toFixed(1) + ' s' : 'NEVER'}; alert for ${alertTime.toFixed(1)} of 12 s`);
      game.puppet = null;
    }
  }
  console.log(`=== door-rules: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

if (which === 'aim-door' || which === 'doors' || which === 'all') await aimDoor();   // ('doors' = the alert-man-at-a-doorway PASS/FAIL set: aim-door, aim-door-chase, aim-door-sweep, far-refix)
if (which === 'aim-door-chase' || which === 'doors' || which === 'all') await aimDoorChase();
if (which === 'aim-door-sweep' || which === 'doors' || which === 'all') await aimDoorSweep();
if (which === 'far-refix' || which === 'doors' || which === 'all') await farRefix();
if (which === 'door-rules' || which === 'doors' || which === 'all') await doorRules();
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
// ---------------------------------------------------------------- the AI pass: muzzle discipline, motion acuity, missing colleagues (ai-probes.ts)
if (which === 'muzzle' || which === 'ai-pass' || which === 'all') { const { muzzle } = await import('./ai-probes.ts'); await muzzle(); }
if (which === 'missing' || which === 'ai-pass' || which === 'all') { const { missing } = await import('./ai-probes.ts'); await missing(); }
if (which === 'motion' || which === 'ai-pass' || which === 'all') { const { motionTable } = await import('./ai-probes.ts'); await motionTable(); }
/** Found by the ai-pass-2 soak (seed 2, f33260) and reproduced identically on main — player.ts, not the AI: sprint east-south-east from the storage room's north
 *  wall into the END PANEL of the east shelf run (a 2 cm static at x 34.95‥35.45, z 5.52) so the body wedges at (35.05, 5.08), then press Q: the hold takes the
 *  exterior east wall behind the panel (x 35.88, inside WALL_REACH past the panel's end) and, four fifths through the settle, the push-out flips him to the far
 *  side of the panel — 0.35 m in one frame (the soak's 'popped'). Standalone (`probes.ts wall-shelf-pop`), not in the 'player' / 'all' sets: a repro handed to
 *  the wall-hold lane, PASS once the hold refuses a plane it cannot settle onto (or settles round the panel). */
async function wallShelfPop() {
  console.log('\n=== wall-shelf-pop: Q while wedged on the storage shelf\'s end panel — the hold must not pop him through it ===');
  Math.random = seeded(2);
  const W = await standUp(); const { game, input } = W;
  game.playerInvisible = true; game.godMode = true;
  for (let i = 0; i < 10; i++) W.step(DT);
  for (const o of game.guards) { o.hold = true; o.pinned = true; o.dazzledUntil = 1e9; o.char.pos = [6, 0, 20 + game.guards.indexOf(o)]; o.char.update(0); }   // nobody near: this is the player's own collision
  game.teleportPlayer([33.48, 0, 4.7]);
  game.puppet = { goal: [37.2, 0, 6.1], aim: null }; input.keys.add('ShiftLeft');
  let prev = v3.copy(game.player.char.pos), still = 0, phase: 'run' | 'q' = 'run', pop = '', wedged = '';
  for (let f = 0; f < 60 * 6 && !pop; f++) {
    W.step(DT);
    const p = game.player.char.pos; const d = v3.distXZ(p, prev);
    if (phase === 'run') { still = d < 0.02 && f > 30 ? still + 1 : 0; if (still > 25) { phase = 'q'; wedged = `(${p[0].toFixed(2)}, ${p[2].toFixed(2)})`; game.puppet = null; input.keys.delete('ShiftLeft'); input.pressed.add('KeyQ'); } }
    else if (d > 0.3) pop = `(${prev[0].toFixed(2)}, ${prev[2].toFixed(2)}) → (${p[0].toFixed(2)}, ${p[2].toFixed(2)}): ${d.toFixed(2)} m in one frame, hold ${game.player.wall ? `n (${game.player.wall.n[0]}, ${game.player.wall.n[2]}) d ${game.player.wall.d} settle ${game.player.wall.settle.toFixed(2)}` : 'none'}`;
    prev = v3.copy(p);
  }
  game.puppet = null; input.keys.delete('ShiftLeft');
  console.log(`- ${!pop && phase === 'q' ? 'PASS' : 'FAIL'} — wedged at ${wedged || 'nowhere (the sprint never stuck: staging drifted)'}; ${pop ? 'POPPED ' + pop : `no pop, ends at (${game.player.char.pos[0].toFixed(2)}, ${game.player.char.pos[2].toFixed(2)}) holding ${!!game.player.wall}`}`);
}
if (which === 'wall-shelf-pop') await wallShelfPop();
// ---------------------------------------------------------------- the second AI pass: the search that reads the light, threshold discipline (ai2-probes.ts)
if (which === 'darkness' || which === 'ai-pass-2' || which === 'all') { const { darkness } = await import('./ai2-probes.ts'); await darkness(); }
if (which === 'pieing' || which === 'ai-pass-2' || which === 'all') { const { pieing } = await import('./ai2-probes.ts'); await pieing(); }
// ---------------------------------------------------------------- KO vs dead + the witness record (ko-probes.ts): 'ko-witness' = the family
if (which === 'ko-kinds' || which === 'ko-witness' || which === 'all') { const { koKinds } = await import('./ko-probes.ts'); await koKinds(); }
if (which === 'ko-rollcall' || which === 'ko-witness' || which === 'all') { const { koRollcall } = await import('./ko-probes.ts'); await koRollcall(); }
if (which === 'witness' || which === 'ko-witness' || which === 'all') { const { witness } = await import('./ko-probes.ts'); await witness(); }
// ---------------------------------------------------------------- the interrogation dialogue engine (dialogue-probes.ts): 'dialogue' = the family
if (which === 'dialogue-none' || which === 'dialogue' || which === 'all') { const { dialogueNone } = await import('./dialogue-probes.ts'); await dialogueNone(); }
if (which === 'dialogue-sawhurt' || which === 'dialogue' || which === 'all') { const { dialogueSawHurt } = await import('./dialogue-probes.ts'); await dialogueSawHurt(); }
if (which === 'dialogue-heard' || which === 'dialogue' || which === 'all') { const { dialogueHeard } = await import('./dialogue-probes.ts'); await dialogueHeard(); }
if (which === 'dialogue-intel' || which === 'dialogue' || which === 'all') { const { dialogueIntel } = await import('./dialogue-probes.ts'); await dialogueIntel(); }
if (which === 'dialogue-lint' || which === 'dialogue' || which === 'all') { const { dialogueLint } = await import('./dialogue-probes.ts'); await dialogueLint(); }
if (which === 'dialogue-seed' || which === 'dialogue' || which === 'all') { const { dialogueSeed } = await import('./dialogue-probes.ts'); await dialogueSeed(); }
// ---------------------------------------------------------------- the grab / hold / interrogate / choke / release (grab-probes.ts): 'grab' = the family
if (which === 'grab-gate' || which === 'grab' || which === 'all') { const { grabGate } = await import('./grab-probes.ts'); await grabGate(); }
if (which === 'grab-walk' || which === 'grab' || which === 'all') { const { grabWalk } = await import('./grab-probes.ts'); await grabWalk(); }
if (which === 'grab-rollcall' || which === 'grab' || which === 'all') { const { grabRollcall } = await import('./grab-probes.ts'); await grabRollcall(); }
if (which === 'grab-talk' || which === 'grab' || which === 'all') { const { grabTalk } = await import('./grab-probes.ts'); await grabTalk(); }
if (which === 'grab-choke' || which === 'grab' || which === 'all') { const { grabChoke } = await import('./grab-probes.ts'); await grabChoke(); }
if (which === 'grab-release' || which === 'grab' || which === 'all') { const { grabRelease } = await import('./grab-probes.ts'); await grabRelease(); }
if (which === 'grab-auto' || which === 'grab' || which === 'all') { const { grabAuto } = await import('./grab-probes.ts'); await grabAuto(); }
// ---------------------------------------------------------------- the pistol-to-the-head hold (gunhold-probes.ts): 'gunhold' = the family
if (which === 'gunhold-variant' || which === 'gunhold' || which === 'all') { const { gunholdVariant } = await import('./gunhold-probes.ts'); await gunholdVariant(); }
if (which === 'gunhold-fire' || which === 'gunhold' || which === 'all') { const { gunholdFire } = await import('./gunhold-probes.ts'); await gunholdFire(); }
if (which === 'gunhold-whip' || which === 'gunhold' || which === 'all') { const { gunholdWhip } = await import('./gunhold-probes.ts'); await gunholdWhip(); }
if (which === 'gunhold-dialogue' || which === 'gunhold' || which === 'all') { const { gunholdDialogue } = await import('./gunhold-probes.ts'); await gunholdDialogue(); }
// ---------------------------------------------------------------- the human shield: colleagues' standoff round the held man (standoff-probes.ts): 'standoff' = the family
if (which === 'standoff-see' || which === 'standoff' || which === 'all') { const { standoffSee } = await import('./standoff-probes.ts'); await standoffSee(); }
if (which === 'standoff-flank' || which === 'standoff' || which === 'all') { const { standoffFlank } = await import('./standoff-probes.ts'); await standoffFlank(); }
if (which === 'standoff-ff' || which === 'standoff' || which === 'all') { const { standoffFF } = await import('./standoff-probes.ts'); await standoffFF(); }
if (which === 'standoff-end' || which === 'standoff' || which === 'all') { const { standoffEnd } = await import('./standoff-probes.ts'); await standoffEnd(); }
if (which === 'standoff-lost' || which === 'standoff' || which === 'all') { const { standoffLost } = await import('./standoff-probes.ts'); await standoffLost(); }
if (which === 'standoff-door' || which === 'standoff' || which === 'all') { const { standoffDoor } = await import('./standoff-probes.ts'); await standoffDoor(); }
if (which === 'standoff-tour' || which === 'standoff' || which === 'all') { const { standoffTour } = await import('./standoff-probes.ts'); await standoffTour(); }
