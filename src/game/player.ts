// Player controller: movement / crouch / sprint under real or puppet input, the pistol and rail light, throws, silent takedown and body drag, plus the cursor: what is under it and what F would do (interactables).
// (Split out of game.ts mechanically: plain functions taking the Game as their first argument; Game keeps one-line delegators for
//  anything other files call.)
import { Vec3, v3, clamp, wrapAngle, approachAngle, damp, DEG } from '../math/vec';
import { Input } from './input';
import { FollowCamera } from './camera';
import { Doors, Door } from './doors';
import { RAG } from './ragdoll';
import { ThrowSolution } from './throwables';
import * as fx from './effects';
import { rigLightPose } from './rigProps';
import { PLAYER_ID, DRIVE_REACH, DRIVE_SECS } from './consts';
import { rackDist } from './mission';
import { fireWeapon, killGuard } from './combat';
import type { Game, Guard, ThrowKind, Interactable } from './game';

/** the player's movement circle: wider than the body so arms / gun don't clip into walls */
const BODY_R = 0.42;
/** living guards are solid to you out to this (centre to centre) */
const KEEP_OFF = 0.55;

/** The player's own collision: the BODY_R circle pushed out of the statics between shin height and `y1` (standing 1.7 / crouched 1.0 — under a desk top only on
 *  your knees), then held on the world's floor rectangle (the lot has no kerb: guards are kept in by the nav grid, you by this). */
function collidePlayer(g: Game, y1: number) { const p = g.player.char.pos; g.col.collideCircle(p, BODY_R, 0.2, y1); g.col.clampToWorld(p, BODY_R); }

export function updatePlayer(g: Game, dt: number, input: Input, cam: FollowCamera) {
  const pl = g.player; const c = pl.char; const T = g.tune; const gun = pl.pistol;
  if (input.hit('Enter') && (pl.down || input.down('ShiftLeft') || input.down('ShiftRight'))) { g.restartEncounter(); return; }
  if (pl.down) { c.vel = [0, 0, 0]; pl.light.enabled = false; pl.throwPreview = null; c.anim.lockpick = false; c.update(dt); g.bodyThud(c); return; }
  gun.update(dt); pl.ocp.update(dt);
  pl.noise *= Math.exp(-2.2 * dt);   // sound meter decay
  if (pl.takedown) { updateTakedown(g, dt); c.anim.speed = 0; c.anim.upper = 'none'; c.update(dt); pl.light.enabled = false; return; }   // locked into the takedown beat: no movement / aim / fire until it resolves
  if (pl.kick) { updateKick(g, dt); return; }              // rooted square to the leaf until the foot is back down (the door goes at KICK_IMPACT)
  if (pl.picking) { updatePicking(g, dt, input); return; }  // planted at the lock while F stays down
  c.anim.lockpick = false;                                   // (whatever ended a pick, the hands come off the tools)
  // equipment selection / toggles
  if (input.hit('Digit1')) { pl.slot = 1; g.msg('Five-seveN'); g.audio.play('equip', null, 0.5); }
  if (input.hit('Digit2')) { if (pl.canisters > 0) { pl.slot = 2; pl.throwKind = 'smoke'; g.msg(`smoke canister ×${pl.canisters}`); g.audio.play('equip', null, 0.5); } else g.msg('no smoke canisters left'); }
  if (input.hit('Digit3')) { if (pl.flashbangs > 0) { pl.slot = 3; pl.throwKind = 'flash'; g.msg(`stun canister ×${pl.flashbangs} — goes off ${fx.bang.fuse}s after it lands`); g.audio.play('equip', null, 0.5); } else g.msg('no stun canisters left'); }
  if (input.hit('KeyL')) { gun.lightOn = !gun.lightOn; g.audio.play('click', c.muzzle, 0.6); }
  if (input.hit('KeyN')) { pl.nv = !pl.nv; g.audio.play(pl.nv ? 'nvOn' : 'nvOff', null, 0.7); g.msg(pl.nv ? 'night vision on' : 'night vision off'); }
  pl.nvAmount = damp(pl.nvAmount, pl.nv ? 1 : 0, pl.nv ? 5 : 9, dt);
  const sprintKey = (input.down('ShiftLeft') || input.down('ShiftRight')) && (input.down('KeyW') || input.down('KeyA') || input.down('KeyS') || input.down('KeyD'));
  if (input.hit('KeyR') && pl.slot === 1 && !sprintKey) {
    if (gun.startReload({
      magOut: (rounds, dropped) => { g.audio.play('magOut', c.pos, 0.7); if (dropped) { const h = c.bones.handL ?? c.pos; const it = g.items.spawn('mag', [h[0], Math.max(0.3, h[1]), h[2]], [(Math.random() - 0.5) * 0.6, -0.2, (Math.random() - 0.5) * 0.6], { fill: 0, who: PLAYER_ID }); it.onBounce = (sp, p) => g.audio.play('magDrop', p, Math.min(1, sp / 3)); } },
      magIn: () => g.audio.play('magIn', c.pos, 0.8),
      rack: () => g.audio.play('rack', c.pos, 0.8),
    })) { c.anim.reload(); g.msg('reloading'); } else if (gun.spare.length === 0) g.msg('no spare magazines');
  }
  // movement input
  const { fwd, right } = cam.planarBasis();
  let mx = 0, mz = 0;
  if (input.down('KeyW')) { mx += fwd[0]; mz += fwd[2]; }
  if (input.down('KeyS')) { mx -= fwd[0]; mz -= fwd[2]; }
  if (input.down('KeyD')) { mx += right[0]; mz += right[2]; }
  if (input.down('KeyA')) { mx -= right[0]; mz -= right[2]; }
  if (g.puppet) {   // scripted movement: straight at the goal (the tour picks goals with a clear line), stop within 10 cm
    mx = 0; mz = 0; const gl = g.puppet.goal;
    if (gl) { const dx = gl[0] - c.pos[0], dz = gl[2] - c.pos[2]; const dl = Math.hypot(dx, dz); if (dl > 0.1) { mx = dx / dl; mz = dz / dl; } }
    if (g.puppet.crouch !== undefined && !pl.dragging) pl.crouch = g.puppet.crouch;
  }
  const ml = Math.hypot(mx, mz); if (ml > 0) { mx /= ml; mz /= ml; }
  if ((input.hit('KeyC') || input.hit('CtrlTap')) && !pl.dragging) pl.crouch = !pl.crouch;   // a solo Ctrl tap doubles as crouch (embeds that eat the C key); Ctrl chords do not
  const wantSprint = (input.down('ShiftLeft') || input.down('ShiftRight')) && ml > 0;
  if (wantSprint && pl.dragging) g.toggleDrag();          // breaking into a sprint lets go of the body
  if (wantSprint && pl.crouch) pl.crouch = false;
  if (pl.dragging) pl.crouch = true;
  pl.sprinting = wantSprint;
  if (pl.sprinting && gun.reloading) { gun.cancelReload(); c.anim.cancelReload(); }
  let maxSpeed = ml === 0 ? 0 : pl.sprinting ? T.playerSprint : pl.dragging ? T.playerCrouch * 0.6 : pl.crouch ? T.playerCrouch : (input.down('AltLeft') ? T.playerWalk * 0.5 : T.playerWalk);   // hauling 80 kg along the floor is slow
  if (g.puppet?.walk !== undefined && ml > 0) maxSpeed *= g.puppet.walk;
  pl.speedSm = damp(pl.speedSm, maxSpeed, maxSpeed > pl.speedSm ? 6 : 12, dt);
  if (ml > 0) { c.vel = [mx * pl.speedSm, 0, mz * pl.speedSm]; } else { c.vel = v3.scale(c.vel, Math.exp(-14 * dt)); }
  c.pos = v3.mad(c.pos, c.vel, dt);
  const headroom = pl.crouch ? 1.0 : 1.7;
  collidePlayer(g, headroom);
  keepOffGuards(g, dt, headroom);
  const speed = Math.hypot(c.vel[0], c.vel[2]);
  // aim
  const ap = g.aimPoint; const toAim: Vec3 = [ap[0] - c.pos[0], 0, ap[2] - c.pos[2]];
  const aimDist = Math.hypot(toAim[0], toAim[2]);
  pl.aimHeld = input.rmb() && !pl.sprinting && !pl.dragging;
  const recentlyFired = g.time - pl.lastFireT < 1.6;
  const aiming = pl.slot === 1 && (pl.aimHeld || recentlyFired || gun.lightOn) && !pl.sprinting && !pl.dragging;   // light on = weapon up: the rail light is bolted to the gun, so pointing the light at the cursor means pointing the gun there
  if (pl.dragging) {
    // dragging: you have it by the ankles — the ragdoll's two ankle particles are pinned to a point at your hands (DRAG_HANDS ahead, shin height) and the rest
    // of it trails through its own constraints, sliding on the floor with friction, bumping round door jambs and desk legs on its own collision. You face
    // its hips, so moving away from it IS a backward crouch-walk. Torn out of your hands if it wedges (the legs cannot follow the hands any more:
    // Ragdoll.strain) or if it somehow got away from you.
    const body = pl.dragging.char; const rag = body.ragdoll;
    const hips = body.bones.hips ?? body.pos; const dx = hips[0] - c.pos[0], dz = hips[2] - c.pos[2]; const d = Math.hypot(dx, dz);
    if (d > 1e-4) pl.dragYaw = Math.atan2(dx, dz);
    pl.dragStuckT = rag && rag.strain > 0.2 ? pl.dragStuckT + dt : 0;
    if (!rag || d > 2.2 || pl.dragStuckT > 0.25) { const wedged = rag && pl.dragStuckT > 0.25; g.toggleDrag(); if (wedged) g.msg('body dropped — it was wedged'); }
    else {
      const DRAG_HANDS = 0.35, f = c.forward(); const hx = c.pos[0] + f[0] * DRAG_HANDS, hz = c.pos[2] + f[2] * DRAG_HANDS;
      rag.setPin(RAG.ankL, [hx + f[2] * 0.1, 0.42, hz - f[0] * 0.1]); rag.setPin(RAG.ankR, [hx - f[2] * 0.1, 0.42, hz + f[0] * 0.1]);   // an ankle in each hand, 20 cm apart across your facing
      c.aimYaw = pl.dragYaw; c.aimPitch = damp(c.aimPitch, -0.9, 6, dt);
      c.bodyYaw = approachAngle(c.bodyYaw, pl.dragYaw, 10 * dt);
      pl.backpedal = speed > 0.2 && Math.cos(wrapAngle(pl.dragYaw - v3.yawOf(c.vel))) < 0; c.anim.reverse = pl.backpedal;
      // it is not silent: a soft periodic scrape while the body is actually sliding (guards close by will come and look)
      if (speed > 0.3 && !rag.sleeping) { pl.dragNoiseT -= dt; if (pl.dragNoiseT <= 0) { pl.dragNoiseT = 0.55; g.audio.play('propScrape', hips, 0.25, { rate: 0.7 }); g.events.push({ kind: 'prop', pos: [hips[0], 0, hips[2]], time: g.time, loud: false, level: 0.22, who: PLAYER_ID }); pl.noise = Math.max(pl.noise, 0.22); } }
    }
  } else if (pl.sprinting) {
    // CoD-style sprint: gun down, body follows the run direction, no aiming
    const moveYaw = v3.yawOf(c.vel);
    c.bodyYaw = approachAngle(c.bodyYaw, moveYaw, 14 * dt); c.aimYaw = c.bodyYaw; c.aimPitch = damp(c.aimPitch, -0.3, 6, dt);
    pl.backpedal = false; c.anim.reverse = false;
  } else {
    if (aimDist > 0.25) c.aimYaw += wrapAngle(v3.yawOf(toAim) - c.aimYaw) * (1 - Math.exp(-16 * dt));   // eased, not snapped: the arms swing through to a new bearing instead of teleporting
    // aim (gun, rail light, shot) AT the world point under the cursor — floor included — unless a fixture or a guard is targeted
    let targetY = ap[1] + 0.03;
    if (g.aimTarget) targetY = clamp(g.aimTarget.pos[1], 0.3, 2.95);
    else if (g.aimGuard) targetY = g.aimGuard.char.pos[1] + 1.2;
    const gunY = c.pos[1] + (pl.crouch ? 0.95 : 1.4);
    c.aimPitch = damp(c.aimPitch, clamp(Math.atan2(targetY - gunY, Math.max(aimDist, 0.5)), -60 * DEG, 70 * DEG), 12, dt);
    if (speed > 0.2) {
      const moveYaw = v3.yawOf(c.vel);
      const d = Math.cos(wrapAngle(c.aimYaw - moveYaw));
      if (pl.backpedal ? d > -0.05 : d < -0.35) pl.backpedal = !pl.backpedal;
      // hips follow the legs' travel direction (or its reverse) but yield up to ~45° toward the aim so the spine twist stays natural
      const base = pl.backpedal ? wrapAngle(moveYaw + Math.PI) : moveYaw;
      const bodyTarget = base + clamp(wrapAngle(c.aimYaw - base), -0.8, 0.8) * (aiming ? 1 : 0.6);
      c.bodyYaw = approachAngle(c.bodyYaw, bodyTarget, 9 * dt);
      c.anim.reverse = pl.backpedal;
    } else {
      pl.backpedal = false; c.anim.reverse = false;
      const off = wrapAngle(c.aimYaw - c.bodyYaw);
      const limit = aiming ? 0.6 : 1.2;
      if (Math.abs(off) > limit) c.bodyYaw = approachAngle(c.bodyYaw, c.aimYaw - Math.sign(off) * limit * 0.7, 7 * dt);
    }
  }
  c.anim.speed = speed; c.anim.crouchTarget = pl.crouch ? 1 : 0;
  c.anim.upper = pl.sprinting || pl.dragging ? 'none' : aiming ? 'aim' : pl.slot === 1 ? 'relaxed' : 'none';
  // --- slot actions
  pl.fireCd -= dt;
  if (pl.slot === 1 && !pl.sprinting && !pl.dragging) {
    if (input.lmbHit() && pl.fireCd <= 0) {
      if (gun.fire()) {
        pl.fireCd = 0.22; pl.lastFireT = g.time;
        const pt = g.puppet?.target && g.puppet.target.state !== 'dead' ? g.puppet.target : null;
        const tgt: Vec3 = pt ? v3.add(pt.char.pos, [0, 1.25, 0]) : g.aimTarget ? v3.copy(g.aimTarget.pos) : g.aimGuard ? v3.add(g.aimGuard.char.pos, [0, 1.25, 0]) : [ap[0], ap[1] + 0.03, ap[2]];   // a fixture under the reticle IS the target (that is what 'click: shoot' promises); else a guard under the cursor; else the world point under the cursor (the same point the gun / rail light pitch to)
        fireWeapon(g, c, tgt, true);
        if (gun.roundsReady === 0) g.msg('empty — press R');
      } else { pl.fireCd = 0.25; g.audio.play('dryFire', c.muzzle, 0.7); if (!gun.reloading) g.msg(gun.roundsReady === 0 ? 'click — empty (R to reload)' : 'reloading…'); }
    }
    if ((input.hit('KeyF') || input.mmbHit()) && (!g.hover || g.hover.kind === 'light' || (g.hover.kind === 'guard' && !g.hover.inReach))) g.ocp(g.aimTarget);   // (a guard marker out of takedown reach still lets F fry his torch)   // F on a light marker (or on nothing: the guard under the cursor) = OCP; doors / bodies handle F in updateDoors / below
  }
  // throwables: preview while slot 2 / 3 is up or G is held; throw on LMB (slot 2 / 3) or on G release. G throws the throwable selected last
  // (smoke until 3 is pressed), falling back to the other kind when that one has run out
  const gHeld = input.down('KeyG') || input.hit('KeyG');   // hit(): a tap that began and ended inside this frame still counts as held for one frame (so its release next frame throws)
  const prevPreview = pl.throwPreview;              // on the release frame G is up, so the arc from the last held frame is the one to throw
  pl.throwPreview = null;
  if (g.throwCount(pl.throwKind) <= 0 && g.throwCount(pl.throwKind === 'flash' ? 'smoke' : 'flash') > 0) pl.throwKind = pl.throwKind === 'flash' ? 'smoke' : 'flash';
  const slotKind: ThrowKind | null = pl.slot === 2 ? 'smoke' : pl.slot === 3 ? 'flash' : null;
  // a wind-up remembers what it was aimed with: spending the last smoke can on LMB mid-hold flips `throwKind` (and clears the slot) a
  // frame later, and the G release must not then cost a stun charge nobody selected. An explicit 2 / 3 press during the hold still switches it.
  const kind: ThrowKind = slotKind ?? (pl.throwHeld ? pl.heldKind : pl.throwKind);
  pl.heldKind = kind;
  if (!pl.sprinting && g.throwCount(kind) > 0 && (slotKind || gHeld)) {
    const tp = g.puppet?.throwAt ?? ap;
    pl.throwPreview = g.items.solve(throwOrigin(g), [tp[0], Math.max(0.05, Math.min(tp[1], 2.6)), tp[2]]);
    if (slotKind && input.lmbHit() && pl.fireCd <= 0) { doThrow(g, pl.throwPreview, kind); pl.fireCd = 0.6; }
  }
  if (pl.throwHeld && !gHeld && !pl.sprinting) { const sol = pl.throwPreview ?? prevPreview; if (sol) doThrow(g, sol, kind); }   // quick-throw on G release
  pl.throwHeld = gHeld && (pl.throwHeld || g.throwCount(kind) > 0);   // a gesture that ran its kind dry stays this kind until G comes up (it just throws nothing)
  if (pl.pendingThrow) { pl.pendingThrow.t -= dt; if (pl.pendingThrow.t <= 0) {
    const { sol, kind: k2 } = pl.pendingThrow;
    const it = g.items.spawn(k2 === 'flash' ? 'flash' : 'smoke', throwOrigin(g), sol.v0, k2 === 'flash' ? { fuse: fx.bang.fuse, who: PLAYER_ID } : { who: PLAYER_ID });
    it.onBounce = (sp, p) => { g.audio.play('canisterBounce', p, Math.min(1, sp / 4)); g.events.push({ kind: 'shot', pos: v3.copy(p), time: g.time, loud: false }); };
    pl.pendingThrow = null; if ((pl.slot === 2 && pl.canisters === 0) || (pl.slot === 3 && pl.flashbangs === 0)) pl.slot = 1;
  } }
  c.update(dt);
  // pistol light: physically mounted under the barrel — its position and direction ARE the gun's, aiming or not (rigProps.RIG.pistol.light*)
  gun.infinite = g.infiniteAmmo;
  const L = pl.light; L.enabled = gun.lightOn && pl.slot === 1 && !pl.sprinting && !c.anim.hideHeldItem; L.peakIntensity = L.intensity = T.playerLight; L.radius = g.engine.settings.flashlightRadius * 0.7;   // sprinting: gun down, light dark; hands still coming off the lock tools: the gun (and its light) is on the thigh   // weapon light bezel is a bit smaller than the guards' hand torches
  if (L.enabled) { const lp = rigLightPose(c); L.pos = lp.pos; L.dir = lp.dir; }
  // footsteps (for audio + AI hearing): step events from the locomotion phase
  const prevPhase = pl.stepPhase; pl.stepPhase = c.anim.phase;
  if (speed > 0.4 && ((prevPhase < 0.5) !== (pl.stepPhase < 0.5))) {   // either crossing of the half-cycle: the cycle runs backwards while backpedalling
    const loud = pl.sprinting ? 1 : pl.crouch ? 0.18 : speed > 2 ? 0.6 : 0.35;
    g.audio.footstep(c.pos, loud, true);
    g.events.push({ kind: 'step', pos: v3.copy(c.pos), time: g.time, loud: false, level: loud }); pl.noise = Math.max(pl.noise, loud);
  }
}

/** Living guards are solid to you and never step aside (updateGuard / runGuardScript separate guards from each other only — a man must not be blocked or steered by a
 *  player he may not even know is there), so you are the one who yields, and every shove is collided with the statics and the world edge AGAIN, so none ever
 *  leaves you standing in a wall for the frame (or, pinned to a 12 cm partition, popped out of its far side):
 *  - a man who STANDS (or whom you run into): pushed straight back out to KEEP_OFF — capped above sprint speed, so solid to anything you do, while a man a script
 *    drops on top of you eases you off over a few frames instead of popping you half a metre; squeezing between him and a wall you slide along the wall past him;
 *  - a man WALKING into you (you within ~50° of dead ahead of the leg he is on): shouldered SIDEWAYS off his line of travel — to the side of it you are on if the
 *    statics leave your body the room to clear him there, else across it if the other side does, else to whichever side has the more room (he brushes through
 *    what is left of the overlap), and only where neither has any to speak of (a narrow doorway) does he simply walk through you — never straight ahead of him,
 *    which would carry you the length of the corridor; less squarely in front, straight away from him minus the share that is only his stride closing; and
 *    wherever the statics eat the shove (pinned to the wall beside his track) you stay where you stand rather than the leftover feeding you along the wall ahead
 *    of him frame after frame. In live play a man coming on has seen you long before he touches you; this is what the tour's unseen Sam and the sandbox get.
 *  `y1` = the headroom updatePlayer collided you with this frame. */
function keepOffGuards(g: Game, dt: number, y1: number) {
  const p = g.player.char.pos;
  const room = (sx: number, sz: number) => { const hit = g.col.raycast([p[0], 0.45, p[2]], [sx, 0, sz], 1.5); return (hit ? hit.t : 1.5) - BODY_R; };   // how far your centre can travel that way before your circle meets a static (or a leaf) at shin height
  for (const gd of g.guards) {
    if (gd.state === 'dead') continue;
    const dx = p[0] - gd.char.pos[0], dz = p[2] - gd.char.pos[2]; const l = Math.hypot(dx, dz);
    if (l >= KEEP_OFF) continue;
    const gv = gd.char.vel; const gs = Math.hypot(gv[0], gv[2]); const walking = gs > 0.2;
    let nx: number, nz: number;   // straight away from him (dead on his centre: to the right of wherever he is heading)
    if (l > 1e-4) { nx = dx / l; nz = dz / l; } else if (gs > 1e-3) { nx = gv[2] / gs; nz = -gv[0] / gs; } else { nx = 1; nz = 0; }
    let cap = 6, fx = 0, fz = 0, ahead = 0;   // his line of travel — the leg to his next waypoint (steady while he pivots onto it through you), else his velocity — and how squarely you are in front on it (cos)
    if (walking) {
      const wp = gd.pathI < gd.path.length ? gd.path[gd.pathI] : null; const wx = wp ? wp[0] - gd.char.pos[0] : 0, wz = wp ? wp[2] - gd.char.pos[2] : 0, wl = Math.hypot(wx, wz);
      if (wl > 0.3) { fx = wx / wl; fz = wz / wl; } else { fx = gv[0] / gs; fz = gv[2] / gs; }
      ahead = nx * fx + nz * fz;
      if (ahead > 0.64) {
        let sx = nx - ahead * fx, sz = nz - ahead * fz; const sl = Math.hypot(sx, sz); let b = l * sl;   // your side of his line, and how far off it you already are
        if (b >= 0.04) { sx /= sl; sz /= sl; } else { sx = fz; sz = -fx; b = 0; }                      // (square in his way: his right)
        const near = b + room(sx, sz), far = room(-sx, -sz) - b;                                        // how far off his line you can get on your side / by crossing to the other
        if (near < KEEP_OFF && (far >= KEEP_OFF || far > near + 0.25)) { if (far < 0.25) continue; sx = -sx; sz = -sz; } else if (near < 0.25) continue;   // (crossing his line only for clearly more room; none worth having either way — a narrow doorway — he walks through you)
        nx = sx; nz = sz; cap = 4; ahead = 0;
      }
    }
    const push = Math.min(KEEP_OFF - l, cap * dt); const bx = p[0], bz = p[2];
    p[0] += nx * push; p[2] += nz * push;
    if (ahead > 0) { const his = ahead * Math.min(push, gs * dt * ahead); p[0] -= fx * his; p[2] -= fz * his; }   // (the forward share of a straight shove that is only his stride closing on you this frame is not yours to travel: half beside him you would still ride along)
    collidePlayer(g, y1);
    if (walking && (p[0] - bx) * nx + (p[2] - bz) * nz < 0.5 * push) { p[0] = bx; p[2] = bz; }   // the statics ate it: pinned — stay put (bx, bz is already a collided spot) and let him brush through
  }
}

/** Silent takedown: lock in behind the guard, throw the cross, he drops with a soft thud only the room hears. */
export function startTakedown(g: Game, gd: Guard) {
  const pl = g.player; if (gd.state === 'dead' || pl.down) return;   // (F + fire in the same frame: the round got there first)
  if (pl.dragging) g.toggleDrag();
  pl.takedown = { g: gd, t: 0 }; pl.sprinting = false; pl.char.anim.strike(); pl.pistol.cancelReload(); pl.char.anim.cancelReload();
  pl.throwHeld = false; pl.throwPreview = null; pl.pendingThrow = null;   // a wind-up does not survive the lunge
  gd.speed = 0; gd.char.vel = [0, 0, 0]; gd.path = []; gd.pathGoal = null; gd.reactT = Math.max(gd.reactT, 1.0);   // he freezes for the beat it takes
  g.audio.play('bodyHit', v3.add(gd.char.pos, [0, 1.3, 0]), 0.55, { rate: 0.8 });
}

export function updateTakedown(g: Game, dt: number) {
  const pl = g.player; const td = pl.takedown; if (!td) return;
  const gd = td.g; const c = pl.char;
  if (!g.guards.includes(gd)) { pl.takedown = null; return; }   // the shift was reset under the lunge (P for the tour, the panel): nobody there to finish on — and never a corpse, a thud and a dropped torch out of a man who no longer exists
  // hold station 0.55 m behind him, facing his back (until he drops: his pos then rides the falling pelvis, and you stand where you struck) — the statics still apply: squeezed behind a man near a wall you strike from as close as the wall allows
  const yaw = gd.char.bodyYaw; c.vel = [0, 0, 0];
  if (gd.state !== 'dead') { const want: Vec3 = [gd.char.pos[0] - Math.sin(yaw) * 0.55, 0, gd.char.pos[2] - Math.cos(yaw) * 0.55]; c.pos = v3.lerp(c.pos, want, 1 - Math.exp(-14 * dt)); collidePlayer(g, pl.crouch ? 1.0 : 1.7); }
  c.bodyYaw = approachAngle(c.bodyYaw, yaw, 14 * dt); c.aimYaw = c.bodyYaw; c.aimPitch = damp(c.aimPitch, 0.1, 8, dt);
  td.t += dt;
  if (td.t >= 0.28 && gd.state !== 'dead') {                    // the cross lands
    killGuard(g, gd, [Math.sin(yaw), 0, Math.cos(yaw)], true);
    g.events.push({ kind: 'prop', pos: v3.copy(gd.char.pos), time: g.time, loud: false, level: 0.3, who: PLAYER_ID });   // the fall: a thud the room hears, not the floor
    pl.noise = Math.max(pl.noise, 0.3);
  }
  if (td.t > 0.62) pl.takedown = null;                        // control back as the cross recovers
}

// ------------------------------------------------------------ locked doors: pick it (quiet, slow) or kick it in (instant, floor-wide loud)
/** F held on a door locked against you (Game.updateDoors): drop whatever the hands were doing and start working the lock — updatePicking runs the beat. */
export function startPicking(g: Game, d: Door) {
  const pl = g.player, c = pl.char; if (pl.down || pl.takedown || pl.kick) return;
  if (pl.dragging) g.toggleDrag();
  pl.pistol.cancelReload(); c.anim.cancelReload(); pl.throwHeld = false; pl.throwPreview = null; pl.sprinting = false;
  pl.picking = d; pl.crouch = true;   // you work a lock on your knee — and stay down there afterwards
  if (d.pick <= 0) g.msg('picking the lock — keep F held; moving breaks off');
}

/** Working the lock while F stays down: eased onto the spot square in front of the keyway (Doors.workSpot), crouched and still, body and eyes on the lock,
 *  hands on the tools (anim.lockpick — once they are there hideHeldItem sends the pistol to the thigh and the rail light with it), and only then does the
 *  cylinder advance (Doors.pick: soft clicks the room hears, a louder snap as it goes). Letting go of F stops with the progress kept on the door for a couple
 *  of seconds; a movement key, the tour taking over, or the leaf unlatching under your hands (a guard and his key coming through) break it off too — and
 *  any of those spends the F press (fHeld) so a held key does not re-plant you mid-stride. */
export function updatePicking(g: Game, dt: number, input: Input) {
  const pl = g.player, c = pl.char, d = pl.picking!;
  const stop = (text?: string) => { pl.picking = null; c.anim.lockpick = false; pl.fHeld = input.down('KeyF'); pl.speedSm = 0; if (text) g.msg(text); };
  const moveKey = input.down('KeyW') || input.down('KeyA') || input.down('KeyS') || input.down('KeyD');
  if (!input.down('KeyF') || moveKey || g.puppet) stop();
  else if (!g.doors.lockedOut(d, c.pos)) stop(d.locked ? 'someone is coming through' : undefined);   // unlatched from the far side while still locked: a key, not you
  else {
    const spot = g.doors.workSpot(d, c.pos);
    c.vel = [0, 0, 0]; c.pos = v3.lerp(c.pos, spot.pos, 1 - Math.exp(-10 * dt)); collidePlayer(g, 1.0); keepOffGuards(g, dt, 1.0);   // (a guard walking into you shoves you off the spot: the work pauses until you settle back)
    c.bodyYaw = approachAngle(c.bodyYaw, spot.yaw, 12 * dt); c.aimYaw = c.bodyYaw; c.aimPitch = damp(c.aimPitch, 0, 8, dt);   // square on, no chest twist: the hands are solved in front of the body
    pl.crouch = true; pl.sprinting = false; pl.backpedal = false; c.anim.reverse = false;
    c.anim.lockpick = true; c.anim.speed = 0; c.anim.crouchTarget = 1; c.anim.upper = 'none';
    const planted = v3.distXZ(c.pos, spot.pos) < 0.12 && c.anim.hideHeldItem;   // there, and the tools are in the keyway
    if (planted && g.doors.pick(d, PLAYER_ID, dt)) stop('the lock gives — it opens like any other door now');
  }
  c.update(dt);
  pl.light.enabled = pl.pistol.lightOn && pl.slot === 1 && !c.anim.hideHeldItem;   // the light rides the gun: lit only while the gun is still (or again) in the hand
  if (pl.light.enabled) { const lp = rigLightPose(c); pl.light.pos = lp.pos; pl.light.dir = lp.dir; }
}

/** Kick a door in: F tapped standing on a door locked against you, or a sprint straight into one (Game.updateDoors). One committed second: updateKick. */
export function startKick(g: Game, d: Door) {
  const pl = g.player, c = pl.char; if (pl.kick || pl.down || pl.takedown || c.anim.kicking) return;   // (anim.kicking without pl.kick: the leg is still coming down from a kick a teleport broke off — its clock would time this one's impact wrong)
  if (pl.dragging) g.toggleDrag();
  pl.picking = null; c.anim.lockpick = false; pl.sprinting = false; pl.crouch = false; pl.throwHeld = false; pl.throwPreview = null; pl.fHeld = true; pl.speedSm = 0;
  pl.pistol.cancelReload(); c.anim.cancelReload();
  pl.kick = { door: d }; c.anim.kickDoor();
}

/** The kick beat (CharacterAnimator.kickDoor, 1.0 s): eased back / up to the kicking distance square to the leaf by the lock (Doors.kickSpot) over the
 *  wind-up, rooted there; on the impact frame the lock goes (Doors.kickIn: keep torn out, leaf flung to its stop, the crack of it through the whole floor);
 *  control returns when the animator has the foot planted again. The rail light stays on the gun throughout (pulled into the high carry, it sweeps the door). */
export function updateKick(g: Game, dt: number) {
  const pl = g.player, c = pl.char, d = pl.kick!.door;
  const spot = g.doors.kickSpot(d, c.pos);
  c.vel = [0, 0, 0];
  if (c.anim.kickTime < 0.45) { c.pos = v3.lerp(c.pos, spot.pos, 1 - Math.exp(-12 * dt)); collidePlayer(g, 1.7); }   // set the distance during the chamber; from the strike on, the planted foot stays where it is
  keepOffGuards(g, dt, 1.7);
  c.bodyYaw = approachAngle(c.bodyYaw, spot.yaw, 16 * dt); c.aimYaw = c.bodyYaw; c.aimPitch = damp(c.aimPitch, 0, 8, dt);
  pl.sprinting = false; pl.crouch = false; pl.backpedal = false; c.anim.reverse = false; c.anim.lockpick = false;
  c.anim.speed = 0; c.anim.crouchTarget = 0; c.anim.upper = pl.slot === 1 ? 'relaxed' : 'none';
  c.update(dt);
  if (c.anim.kickImpact) { g.doors.kickIn(d, c.pos, PLAYER_ID); g.msg('kicked it in — the whole floor heard that'); }
  if (!c.anim.kicking) pl.kick = null;
  pl.light.enabled = pl.pistol.lightOn && pl.slot === 1;
  if (pl.light.enabled) { const lp = rigLightPose(c); pl.light.pos = lp.pos; pl.light.dir = lp.dir; }
}

/** Planar distance from the player to a corpse, wherever the ragdoll left it: the nearer of its ankles (what you take hold of) and its hips. */
export function bodyDist(g: Game, gd: Guard): number {
  const p = g.player.char.pos, b = gd.char.bones;
  const a: Vec3 = [p[0], 0.5, p[2]];
  const reach = (q: Vec3 | undefined) => { if (!q) return 99; const t: Vec3 = [q[0], 0.5, q[2]]; return g.col.segmentBlocked(a, t) || g.col.segmentBlockedDynamic(a, t) ? 99 : v3.distXZ(q, p); };   // not through a partition or a shut door
  return Math.min(reach(b.footL), reach(b.footR), reach(b.hips) + 0.3);   // by the hips still means gathering the legs in first: count it a little further
}

/** Throw origin (right hand-ish) */
export function throwOrigin(g: Game): Vec3 { const c = g.player.char; const f = c.forward(); return [c.pos[0] + f[0] * 0.32, c.pos[1] + (g.player.crouch ? 1.05 : 1.5), c.pos[2] + f[2] * 0.32]; }

export function doThrow(g: Game, sol: ThrowSolution, kind: ThrowKind) {
  const pl = g.player; if (g.throwCount(kind) <= 0) { g.msg(kind === 'flash' ? 'no stun canisters left' : 'no smoke canisters left'); return; }
  if (pl.pendingThrow) return;                                   // one in the air per wind-up (double-tap G would otherwise eat a canister)
  if (kind === 'flash') pl.flashbangs--; else pl.canisters--;
  pl.char.anim.throwItem(); pl.pendingThrow = { t: 0.16, sol, kind };
  g.audio.play('throw', throwOrigin(g), 0.7);
}

export function updateAim(g: Game, input: Input, cam: FollowCamera, canvas: HTMLCanvasElement) {
  if (g.puppet?.aim) {   // scripted aim: put the cursor where the point projects (anywhere in front of the eye, on the view or off it)
    const [sx, sy, front] = cam.project(g.puppet.aim, canvas.clientWidth, canvas.clientHeight, Infinity, 1e-3);
    if (front) { input.mouseX = sx; input.mouseY = sy; }
  }
  const u = input.mouseX / Math.max(1, canvas.clientWidth), v = input.mouseY / Math.max(1, canvas.clientHeight);
  const ray = cam.ray(u, v);
  const hit = g.engine.world.raycast(ray.ro, ray.rd, 200, undefined, PLAYER_ID);
  let p: Vec3 | null = hit ? v3.mad(ray.ro, ray.rd, hit.t) : cam.groundPoint(u, v, 0);
  if (!p) p = v3.add(v3.add(g.player.char.pos, g.player.char.forward()), [0, 1.15, 0]);   // cursor ray missed everything: aim level, straight ahead
  g.aimPoint = p;
  buildInteractables(g, cam, input.mouseX, input.mouseY, canvas.clientWidth, canvas.clientHeight);
  g.aimTarget = g.hover?.kind === 'light' && !(g.puppet && !g.puppet.fixtures) ? g.hover.target! : null;   // (the tour's puppet only means a fixture when the beat says so — a marker behind the man it is aiming at must not steal the shot)
  g.aimGuard = null; let bestT = hit ? hit.t : 1e9;
  for (const gd of g.guards) { const t = gd.char.rayHit(ray.ro, ray.rd, bestT + 0.5); if (t >= 0 && t < bestT + 0.5) { g.aimGuard = gd; bestT = t; } }
}

/** Every marker the F key can act on this frame, plus which one the cursor is on (within a few px of the ring). */
export function buildInteractables(g: Game, cam: FollowCamera, mx: number, my: number, w: number, h: number) {
  const pl = g.player; const list: Interactable[] = []; const p = pl.char.pos;
  for (const t of g.targets) {
    if (t.broken || !t.interactive) continue;
    list.push({ kind: 'light', pos: t.pos, line1: `${t.name}${t.factor > 0 ? '' : ' (off)'}`, line2: 'F  OCP  ·  click  shoot', inReach: true, off: t.factor <= 0, target: t });
  }
  if (!pl.down) {
    for (const d of g.doors.list) {
      const dp = d.pos; const near = Math.min(v3.distXZ(dp, p), v3.distXZ(d.frameCentre, p)) < Doors.USE_REACH; const reach = near && !pl.sprinting;
      const shut = d.isClosed(); const pct = `${Math.min(99, Math.round(d.pick * 100))}%`;
      let line1: string, line2: string, progress: number | undefined;
      if (g.doors.lockedOut(d, p)) {   // the lock is against you: pick it (quiet, slow, F held) or kick it in (instant, and the whole floor hears) — crouched, a tap only tries the handle
        line1 = d.rattleT > 0 ? "door · locked — won't budge" : pl.picking === d ? 'door · locked · picking it' : d.pick > 0 ? `door · locked · ${pct} picked` : 'door · locked';
        line2 = pl.picking === d ? `picking… ${pct}  ·  keep F held, stay still` : pl.kick?.door === d ? 'kicking it in…' : reach ? (pl.crouch ? 'hold F  pick the lock  ·  F  try it  ·  stand + F  kick it in (LOUD)' : 'hold F  pick the lock  ·  F or sprint into it  kick it in (LOUD)') : pl.sprinting && near ? 'run into it  kick it in (LOUD)' : 'step closer';
        progress = d.pick > 0 ? d.pick : undefined;
      } else {
        line1 = `door · ${shut ? 'shut' : 'open'}${d.lockBroken ? ' · lock kicked in' : d.def.locked && !d.locked ? ' · picked' : ''}`;
        line2 = reach ? (shut || Math.abs(d.angle) < 0.62 ? 'F  push open  ·  hold F  crack it silently' : 'F  pull it shut') : 'step closer';
      }
      list.push({ kind: 'door', pos: dp, line1, line2, inReach: reach, off: false, progress, door: d });
    }
    for (const gd of g.guards) {
      if (gd.state === 'dead' || pl.takedown) continue;
      // silent takedown: close behind a guard who has not clocked you (not alert, not looking your way)
      const gp = gd.char.pos; const dx = p[0] - gp[0], dz = p[2] - gp[2]; const dd = Math.hypot(dx, dz);
      if (dd < 2.4) {
        const behind = Math.cos(wrapAngle(Math.atan2(dx, dz) - gd.char.bodyYaw)) < -0.25;   // you are in his rear half-space
        const can = dd < 1.05 && behind && gd.state !== 'alert' && !pl.sprinting && !g.col.segmentBlocked([p[0], 1.0, p[2]], [gp[0], 1.0, gp[2]]) && !g.col.segmentBlockedDynamic([p[0], 1.0, p[2]], [gp[0], 1.0, gp[2]]);   // and nothing between you: no takedown through a partition or a shut leaf (the lunge would carry you through it)
        list.push({ kind: 'guard', pos: v3.add(gp, [0, 1.35, 0]), line1: gd.state === 'alert' ? 'guard · alert' : behind ? 'guard' : 'guard · facing you', line2: can ? 'F  takedown' : gd.state === 'alert' ? 'no takedown while he is onto you' : !behind ? 'get behind him' : pl.sprinting ? 'stop sprinting' : 'get closer', inReach: can, off: false, guard: gd });
      }
    }
    for (const it of g.items.items) {   // a dropped sidearm still holds rounds: take its magazine
      if (it.kind !== 'pistol' || !it.settled || it.rounds <= 0) continue;
      const d = v3.distXZ(it.pos, p); if (d > 3.5) continue;
      const reach = d < 1.2 && !pl.dragging;
      list.push({ kind: 'pistol', pos: [it.pos[0], it.pos[1] + 0.1, it.pos[2]], line1: `${it.fill ? 'your Five-seveN' : 'sidearm'} · ${it.rounds} rnd`, line2: reach ? 'F  take magazine' : pl.dragging ? 'hands full' : 'step closer', inReach: reach, off: false, item: it });
    }
    for (const gd of g.guards) {
      if (gd.state !== 'dead') continue;
      const hips = gd.char.bones.hips ?? gd.char.pos; const mid: Vec3 = [hips[0], hips[1] + 0.12, hips[2]];   // on the pelvis, wherever the ragdoll left it (over a desk, half through a doorway)
      const dragging = pl.dragging === gd; const reach = dragging || bodyDist(g, gd) < 1.3;
      list.push({ kind: 'body', pos: mid, line1: 'body', line2: dragging ? 'F  let go  ·  sprinting drops it' : reach ? 'F  drag' : 'step closer', inReach: reach, off: false, guard: gd });
    }
    // the mission's rack while the drive is still in it: reach = close enough to lay hands on its face, hands free (updateMission acts on the F)
    const m = g.mission, M = g.level.mission;
    if (m.stage === 'drive' && (!g.quietUtility || g.missionInTour)) {
      const reach = rackDist(g, p) < DRIVE_REACH && !pl.dragging && !pl.sprinting && !pl.takedown;
      list.push({ kind: 'objective', pos: M.rack.front, line1: `rack ${M.rack.index + 1} · the drive`, line2: m.driveT >= 0 ? `pulling… ${Math.min(99, Math.round(m.driveT / DRIVE_SECS * 100))}%  ·  stay on it` : reach ? 'F  pull the drive' : pl.dragging ? 'hands full' : 'step closer', inReach: reach, off: false, progress: m.driveT >= 0 ? Math.min(1, m.driveT / DRIVE_SECS) : undefined });
    }
  }
  g.interactables = list;
  // hover: nearest marker within a hair of the ring's radius (11 px ring → 15 px), lights and bodies before doors on ties
  let best: Interactable | null = null, bd = 15;
  const prev = g.hover;
  for (const it of list) {
    const q = it.pos; const [sx, sy, front] = cam.project(q, w, h); if (!front) continue;   // (behind the eye: nothing to hover)
    let dd = Math.hypot(sx - mx, sy - my);
    if (prev && prev.kind === it.kind && prev.pos[0] === q[0] && prev.pos[2] === q[2] && dd < 34) dd = Math.min(dd, 14.9);   // hysteresis: what you are on stays on while the camera glides (acquire at 15 px, release past 34)
    if (dd < bd) { bd = dd; best = it; }
  }
  if (pl.dragging && (!best || !best.inReach)) best = list.find(it => it.guard === pl.dragging) ?? best;   // while hauling a body its marker stays 'live' so F always lets go (an out-of-reach marker under the cursor must not eat the key)
  const busyDoor = pl.picking ?? pl.kick?.door;
  if (busyDoor) best = list.find(it => it.door === busyDoor) ?? best;   // working a lock / kicking: that door's marker (and its progress) stays up even though planting you moved the view off the cursor
  g.hover = best;
  g.useDoor = best?.kind === 'door' && best.inReach ? best.door! : null;
}

export function throwCount(g: Game, kind: ThrowKind) { return kind === 'flash' ? g.player.flashbangs : g.player.canisters; }

/** F: let go of the body you are dragging (its ankles fall where they are), or take hold of the nearest one within reach. */
export function toggleDrag(g: Game, which?: Guard) {
  const pl = g.player;
  if (pl.dragging) { releaseBody(g); g.msg('body dropped'); return; }
  if (pl.down || pl.sprinting) return;
  let best: Guard | null = which && which.state === 'dead' ? which : null;
  if (!best) { let bd = 1.3; for (const gd of g.guards) { if (gd.state !== 'dead') continue; const d = bodyDist(g, gd); if (d < bd) { bd = d; best = gd; } } }
  if (!best || !best.char.ragdoll) { g.msg('no body within reach'); return; }
  pl.dragging = best; pl.crouch = true; pl.dragNoiseT = 0.3; pl.dragStuckT = 0; best.char.ragdoll.wake(); g.msg('dragging the body — F to let go, sprint drops it');
}

/** Let go of the body in tow: the ankle pins come off and its feet fall where they are (the soft thud of that; the HUD line is the caller's). */
function releaseBody(g: Game) {
  const pl = g.player; if (!pl.dragging) return;
  const b = pl.dragging.char; b.ragdoll?.clearPins(); pl.dragging = null;
  g.audio.play('bodyFall', b.bones.footL ?? b.pos, 0.3);
}

/** Break off whatever the hands and feet are in the middle of that belongs to the SPOT, and stand ready: the kick (its door stays as it is), the pick (its progress
 *  stays on the door), a takedown that has not landed (he was only frozen for the beat: he carries on), a body in tow (dropped where it lies), a wind-up (the
 *  canister goes back on the belt), a door held. What a teleport needs — updateKick / updatePicking would otherwise reel you back to their door at 12 m a frame
 *  through every wall between, the cross would land on a man now rooms away, the body's ankles would come with you. A reload travels with you. The animator's
 *  one-shots (the kicking leg, the cross) play out their last fraction of a second where you arrive; nothing reads them any more (startKick waits for the leg). */
export function cancelActions(g: Game) {
  const pl = g.player, c = pl.char;
  if (pl.dragging) releaseBody(g);
  pl.takedown = null;
  if (pl.kick || pl.picking) pl.fHeld = true;                 // the F that started it, if it is still down, has to come up before a door listens to it again (updateDoors clears this the first frame it is up)
  pl.kick = null; pl.picking = null; c.anim.lockpick = false;
  pl.doorHold = 0; pl.doorCracking = false;
  if (pl.pendingThrow) { if (pl.pendingThrow.kind === 'flash') pl.flashbangs++; else pl.canisters++; pl.pendingThrow = null; }   // not yet out of the hand: keep it
  pl.throwHeld = false; pl.throwPreview = null;
  pl.sprinting = false; pl.speedSm = 0; c.vel = [0, 0, 0];
}

/** Sandbox / tour / harness: stand the player at p (pushed out of whatever is there, snapped to the nearest walkable cell if that is inside something big or off the
 *  map, always inside the world), free of anything he was in the middle of (cancelActions). A body on the floor stays where it fell. */
export function teleportPlayer(g: Game, p: Vec3) {
  const pl = g.player; if (pl.down) return;
  cancelActions(g);
  let q: Vec3 = [p[0], 0, p[2]];
  const settle = () => { g.col.clampToWorld(q, BODY_R); g.col.collideCircle(q, BODY_R, 0.05, 2.2, 6); const t = v3.copy(q); return !g.col.collideCircle(t, BODY_R, 0.05, 2.2, 1) || v3.distXZ(t, q) < 0.005; };   // pushed out of whatever is there — and did that converge (false: a gap the body cannot fit, the pushes just trade him back and forth)
  if (!settle() || g.col.nav.isBlocked(q[0], q[2])) {   // inside something big, in a gap too narrow for him, off the walkable floor: stand him on the nearest walkable cell instead
    const c = g.col.nav.nearestFreePoint(q[0], q[2]); if (c) { q = c; settle(); }
  }
  g.col.clampToWorld(q, BODY_R);
  pl.char.pos = q; pl.char.update(0);             // bones at the new spot now: the capsules, the light meter query and the cursor's markers this frame must not read the old room
  g.note('teleported');
}
