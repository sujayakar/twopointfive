// Guard AI: hearing (the same propagation the mixer uses), vision (the player's light-meter irradiance, marched through smoke), patrol / suspicious / alert / search state machine, path following, torch handling, and their barks.
// (Split out of game.ts mechanically: plain functions taking the Game as their first argument; Game keeps one-line delegators for
//  anything other files call.)
import { Vec3, v3, clamp, wrapAngle, approachAngle, damp, lerp, DEG, quat } from '../math/vec';
import { Engine } from '../engine';
import { Level, PatrolRoute, Chokepoint, RoomClearPlan } from '../scene/level';
import { StaticCollision } from '../scene/collision';
import { GltfCharacter } from '../anim/gltf';
import { CharacterRenderer } from '../anim/characters';
import { Character } from './character';
import { Input } from './input';
import { FollowCamera } from './camera';
import { RtLight } from '../render/lights';
import { IrradianceProbe } from '../render/probe';
import { SmokeEmitter, SmokeSystemLike } from '../smoke/types';
import { Box, BoxFlag, makeBox } from '../scene/boxes';
import { Pistol, Ocp } from './weapons';
import { Doors, Door, DoorEvent, DoorContact } from './doors';
import { RAG } from './ragdoll';
import { Throwables, ThrowSolution, Item } from './throwables';
import { PropSystem, PropEvent, PropContact } from './props';
import { GameAudioLike, nullAudio } from './audio_iface';
import * as fx from './effects';
import { Sparks } from './sparks';
import { rigBoxes, rigLightPose, rigTorchPos, torchCarryDir, railLightDef, torchLightDef, holsterBoxes, RIG } from './rigProps';
import { PLAYER_ID, DRIVE_REACH, DRIVE_SECS } from './consts';
import { missionLive, tally, rackDist, updateMission, drivePulled, sendRackCheck } from './mission';
import { objectivePos, objectiveText } from './mission';
import { fireWeapon, killGuard, dropFromHand, hitPlayer, handProps, laserBoxes, updateItems, smokeParams, startCanister, detonateStun } from './combat';
import { dropThrowable } from './combat';
import type { Game, Player } from './game';
import { Guard } from './game';
import { runGuardScript, Clearing, inBounds } from './squad';

// ------------------------------------------------------------ guards
export function hearingCheck(g: Game, gd: Guard) {
  const keen = escalationOf(g) >= 1 ? KEEN : 1;   // an alarmed floor listens harder: the small noises (a creak, a scrape, a footfall) count for a quarter more
  for (const e of g.events) {
    if (e.id === undefined) e.id = ++g.eventSeq;          // sequence ids: events raised by guards later in the update order are still heard next frame
    if (e.id <= gd.heardUpTo) continue;
    gd.heardUpTo = Math.max(gd.heardUpTo, e.id);
    const dStraight = v3.dist(e.pos, gd.char.pos);
    if (e.kind === 'lightOut') { if (dStraight < 10 && gd.awareness < 0.4) { gd.awareness = 0.4; gd.lastKnown = v3.copy(e.pos); gd.reactT = 0.8; } continue; }   // seen, not heard
    const gunfire = (e.kind === 'shot' || e.kind === 'guardShot') && (e.level ?? 0) >= 1;
    const kick = e.kind === 'kick';                                         // a door kicked in: one crack as loud as anything short of a shot — the floor, not the car park
    if (dStraight > (e.kind === 'bang' || gunfire ? 80 : kick ? 40 : 22)) continue;   // gunfire and stun charges: the whole floor (and the lot); everything else has a horizon
    // sound reaches the guard the way it reaches the player's ears: straight through air, or around corners along the
    // walkable space (longer, a bit muffled — a lot if a closed door is across the doorway), or barely through the mass
    const prop = g.col.sound.propagate(v3.add(e.pos, [0, 1, 0]), v3.add(gd.char.pos, [0, 1.5, 0]));
    const d = prop.pathLen, carry = prop.carry;                                  // routed & open ≈ 0.72, through a closed door ≈ 0.32, sealed 0.25
    const heardAt: Vec3 = [prop.via[0], 0, prop.via[2]];                       // where to go and look: the source if heard directly, else the doorway / corner it came through
    if (e.kind === 'shot' && !gunfire && d < 7 * carry + 0.5) { gd.awareness = Math.max(gd.awareness, 0.5); gd.lastKnown = v3.copy(e.pos); }   // a canister's pop / skitter: worth a look nearby
    if (gunfire || e.kind === 'bang' || kick) {
      // a gunshot — the suppressor buys ambiguity about WHERE, not WHETHER — or a stun charge going off, or a door being kicked in, puts every guard on the floor
      // on high alert: weapons out, converging on where it came from (the source if the path is open, else the doorway / corner it spilled through), the
      // furthest ones a beat later. A guard's own shots tell the others where the intruder IS.
      if (e.kind === 'guardShot' && dStraight < 0.1) continue;                 // his own shot
      const wasCalm = gd.state === 'patrol' || gd.state === 'suspicious';
      gd.awareness = Math.max(gd.awareness, 0.92);
      // where to converge: a colleague's shots always say where the intruder IS (even to men already moving on an older fix); your shot: exact if
      // the path was open, otherwise the right area — placed more loosely the further and more walled-off it was — and an alert guard only
      // re-targets on it if he has not had eyes on you for a second (he trusts his own sighting over an echo)
      if (e.kind === 'guardShot') gd.lastKnown = v3.copy(g.player.char.pos);
      else if (gd.state !== 'alert' || !gd.lastKnown || prop.direct || g.time - gd.lastSeenT > 1.0) {
        const err = prop.direct ? 0 : Math.min(4, 0.6 + d * 0.08) * (1.3 - 0.3 * carry); const a = Math.random() * Math.PI * 2; const cand: Vec3 = [e.pos[0] + Math.cos(a) * err, 0, e.pos[2] + Math.sin(a) * err];
        const reachable = !g.col.nav.isBlocked(cand[0], cand[2]) && !!g.col.nav.findPath(gd.char.pos, cand);   // a free-but-unreachable pocket would have him grind a wall forever
        gd.lastKnown = reachable ? cand : v3.copy(e.pos);
      }
      if (wasCalm) gd.reactT = Math.max(gd.reactT, 0.25 + Math.min(1.2, d / 25) + Math.random() * 0.4);   // the far end of the floor takes a moment to place it (once: a colleague's continuing fire must not keep re-rooting men already on the move)
      if (wasCalm && !e.announced) { e.announced = true; g.say(gd, e.kind === 'bang' ? 'flashbang! go, go!' : kick ? 'that was a door going in — someone\'s inside!' : e.kind === 'guardShot' ? 'shots fired! on me!' : 'that was a shot — find him!', e.kind !== 'bang'); }
    }
    if ((e.kind === 'door' || e.kind === 'prop') && d < 12 && e.who !== gd.char.id) {
      // doors / furniture: a creak, latch or scraping chair nearby that no guard caused is worth a look; a slam, kicked door or a chair sent flying alerts the floor
      const byGuard = (e.who ?? -1) >= 10;
      const a = (e.level ?? 0.2) * clamp(1 - d / 12, 0, 1) * (byGuard ? 0.0 : 1.0) * carry * keen;
      if (a > 0.08) { gd.awareness = Math.min(1, Math.max(gd.awareness, Math.min(e.loud ? 0.75 : 0.45, gd.awareness + a))); if (!gd.lastKnown || gd.state !== 'alert') gd.lastKnown = heardAt; if (gd.state === 'patrol') gd.reactT = 0.6; }
    }
    if (e.kind === 'step' && d < 9 && !(g.playerInvisible && (e.who ?? PLAYER_ID) === PLAYER_ID)) {
      const a = (e.level ?? 0.3) * clamp(1 - d / 9, 0, 1) * carry * keen;
      if (a > 0.12) { gd.awareness = Math.min(1, Math.max(gd.awareness, Math.min(0.6, gd.awareness + a * 0.5))); if (!gd.lastKnown || gd.state === 'patrol') gd.lastKnown = heardAt; }
    }
  }
}

/** Awareness per second a VISIBLE player builds in this man at `dist` (canSee said yes): the light on the player, distance, crouch, speed, the smoke between them,
 *  the alarmed floor's keenness — point-blank is certain — and a man already hunting is twice as quick on the uptake. One formula for updateGuard and for the
 *  scripted men's senses (squad.ts perceive). Above 0.05 it moves his fix; above 0.25 it is a real sighting (the licence to shoot). */
export function sightGain(g: Game, gd: Guard, dist: number): number {
  const T = g.tune; const light = g.player.visibility; const distF = clamp(1 - (dist - 3) / 13, 0.2, 1);
  let rate = T.detectRate * Math.pow(light, 1.5) * distF * (g.player.crouch ? 0.75 : 1) * (Math.hypot(g.player.char.vel[0], g.player.char.vel[2]) > 2 ? 1.5 : 1) * clamp((gd.smokeTrans - 0.3) / 0.5, 0, 1) * (escalationOf(g) >= 1 ? KEEN : 1);   // (an alarmed floor is actually looking: a quarter quicker on the uptake)
  if (dist < 1.8) rate = Math.max(rate, 3.5);
  if (gd.state === 'alert' || gd.state === 'search') rate *= 2.2;
  return rate;
}

/** A dead colleague lying in this man's view: within 12 m, inside 60° of where his chest points, nothing solid between his eyes and the body. (updateGuard raises the
 *  alarm on it; the scripted men's senses in squad.ts only need to know to break off toward it — updateGuard does the rest the frame after.) */
export function bodyInView(g: Game, gd: Guard): Guard | null {
  const eye = gd.char.bones.head ?? v3.add(gd.char.pos, [0, 1.65, 0]); const look = gd.char.chestDir;
  for (const o of g.guards) {
    if (o === gd || o.state !== 'dead' || o.found) continue;   // (a body somebody has already called in is known: it startles nobody a second time)
    const body = o.char.bones.hips ?? o.char.pos;
    const to = v3.sub(body, eye); if (v3.len(to) > 12) continue;
    if (v3.dot([look[0], 0, look[2]], v3.normalize([to[0], 0, to[2]])) < Math.cos(60 * DEG)) continue;
    if (g.col.segmentBlocked(eye, v3.add(body, [0, 0.3, 0]))) continue;
    return o;
  }
  return null;
}

/** floor point under a body: its pelvis once the ragdoll has it (Character.pos rides it anyway), i.e. where the men walk up to and look */
export function bodyPos(o: Guard): Vec3 { const h = o.char.bones.hips ?? o.char.pos; return [h[0], 0, h[2]]; }
/** A found (called-in) body lying within `r` metres of `p`, if any — the men know it is there, so nothing near it is 'nothing'. */
export function nearKnownBody(g: Game, p: Vec3, r = 4): Guard | null {
  for (const o of g.guards) if (o.state === 'dead' && o.found && v3.distXZ(bodyPos(o), p) < r) return o;
  return null;
}
/** What a keyed door gives away about the intruder: its keep kicked out of the jamb ('forced' — plain to anyone passing), or standing unlocked when staff always throw
 *  it ('picked' — or so they read it; found by a hand on the lever). Plain doors say nothing. */
export function doorTamper(d: Door): 'forced' | 'picked' | null { return !d.def.locked ? null : d.lockBroken ? 'forced' : !d.locked ? 'picked' : null; }

// ------------------------------------------------------------ a found body: the floor's short script (finder covers, second man's one line, everyone widens out)
/** seconds the body script may hold a man before he lets it go like any other suspicion (a called man who cannot get there, say) */
const BODY_DUTY_CAP = 45;
/** Where a man stops by a body: `stand` metres short of it on his own side of it (working round toward either flank when that is in a wall or taken by a colleague),
 *  on a free cell he can walk to with the body in plain view from there. Falls back to the reachable cell nearest the body. */
function coverSpot(g: Game, gd: Guard, bp: Vec3, stand: number): Vec3 {
  const nav = g.col.nav, from = gd.char.pos;
  const reach = nav.findPath(from, bp); const foot: Vec3 = reach?.length ? reach[reach.length - 1] : v3.copy(from);   // the free cell nearest him a path actually gets to (the pelvis itself may lie under a desk edge / against a wall)
  const base = Math.atan2(from[0] - bp[0], from[2] - bp[2]);   // bearing from the body back toward him
  for (const da of [0, 0.5, -0.5, 1.0, -1.0, 1.6, -1.6, 2.3, -2.3, Math.PI]) for (const r of [stand, stand + 0.5, Math.max(1.0, stand - 0.4)]) {
    const a = base + da; const p: Vec3 = [bp[0] + Math.sin(a) * r, 0, bp[2] + Math.cos(a) * r];
    if (nav.isBlocked(p[0], p[2]) || !nav.walkable(foot, p) || g.col.segmentBlocked([p[0], 1.0, p[2]], [bp[0], 0.4, bp[2]])) continue;
    let taken = false; for (const o of g.guards) if (o !== gd && o.state !== 'dead' && o.bodyDuty?.spot && v3.distXZ(o.bodyDuty.spot, p) < 0.9) { taken = true; break; }
    if (!taken) return p;
  }
  return v3.copy(foot);
}
/** Where a man widens out to from a body: five to eight metres off along his own arm of a fan dealt round it (the first away goes back out the way he came up, the
 *  next two take the thirds either side, then the gaps), on a free cell he can reach without a detour through half the floor, at least four metres from the body and
 *  not beside a colleague's point. null if the place offers nothing (a cupboard): he lets it go where he stands. */
function fanSpot(g: Game, gd: Guard, B: Guard, bp: Vec3): Vec3 | null {
  const nav = g.col.nav, from = gd.char.pos; const k = B.bodyFan++;
  const base = Math.atan2(from[0] - bp[0], from[2] - bp[2]) + [0, 2.1, -2.1, 1.05, -1.05, Math.PI][k % 6];
  let paths = 0;   // (A* is the dear part: a dozen tries, then he lets it go where he stands)
  for (const da of [0, 0.35, -0.35, 0.7, -0.7, 1.1, -1.1, 1.6, -1.6]) for (const r of [6, 7.5, 5, 4.5]) {
    const a = base + da; const p: Vec3 = [bp[0] + Math.sin(a) * r, 0, bp[2] + Math.cos(a) * r];
    if (nav.isBlocked(p[0], p[2])) continue;
    let taken = false; for (const o of g.guards) if (o !== gd && o.state !== 'dead' && o.bodyDuty?.stage === 'fan' && o.bodyDuty.spot && v3.distXZ(o.bodyDuty.spot, p) < 2.5) { taken = true; break; }
    if (taken) continue;
    if (++paths > 12) return null;
    const path = nav.findPath(from, p); if (!path?.length || v3.distXZ(path[path.length - 1], bp) < 4) continue;
    let L = 0, q: Vec3 = from; for (const w of path) { L += v3.distXZ(q, w); q = w; }
    if (L < 16) return p;   // ('six metres off' by way of two rooms and back is not widening out, it is leaving)
  }
  return null;
}
/** Movement watchdog for a man who is supposed to be walking somewhere: true once he has covered less than half a metre in `secs` seconds (a leaf somebody holds
 *  against him, a colleague planted in his way, a mark he cannot quite reach) — measured on the ground he covers, not on his distance to the goal, so the long way
 *  round a partition is not a stall. Callers drop the leg when it fires and reset it (stallRef = null) whenever the leg changes; a man standing on purpose must not call it. */
function notGettingAnywhere(gd: Guard, dt: number, secs: number): boolean {
  const p = gd.char.pos;
  if (!gd.stallRef || v3.distXZ(p, gd.stallRef) > 0.5) { gd.stallRef = v3.copy(p); gd.stallT = 0; return false; }
  return (gd.stallT += dt) > secs;
}
/** One frame of a man's part in the found-body script (his state is 'suspicious' throughout; returns his move speed and aim point for updateGuard, or null when the
 *  script no longer applies and ordinary suspicion should take over — an errand, a scripted hold, the cap, or a fresh noise that put his fix somewhere else):
 *  'to'    — a beat to take it in (reactT), then briskly to his own spot short of the body (coverSpot), muzzle coming down onto it once he can see it;
 *            there: the finder covers; the second man to reach it says the one line; later ones just look;
 *  'cover' — planted, muzzle low on the body: the finder until somebody has joined him (or long enough alone), the others for a moment;
 *  'fan'   — off to his search point away from the body (fanSpot), sweeping as he goes, a few seconds' look round there, then he lets it go — and the closing line,
 *            said once for everybody, replaces 'clear here' (updateGuard). The lockdown pair are collected from this stage by their room clear (squad.ts muster). */
function bodyAftermath(g: Game, gd: Guard, dt: number): { move: number; aim: Vec3 | null } | null {
  const D = gd.bodyDuty!, B = D.body, c = gd.char, T = g.tune;
  const errand = !!gd.task && !(D.role === 'finder' && D.stage !== 'fan');   // an errand handed to him since (the breaker) takes him off it — except the finder walking up to / standing over the body: he finishes that first and goes when he would have widened out
  if (gd.hold || errand || !g.aiEnabled || B.state !== 'dead' || !g.guards.includes(B) || g.time - D.t0 > BODY_DUTY_CAP) { gd.bodyDuty = null; return null; }   // (AI off: everybody talks himself calm at once, as ever)
  const bp = bodyPos(B);
  if (!gd.task && gd.lastKnown && v3.distXZ(gd.lastKnown, D.spot ?? bp) > 2.5 && v3.distXZ(gd.lastKnown, bp) > 2.5) { gd.bodyDuty = null; return null; }   // hearingCheck has moved his fix onto a noise somewhere else: that is what he looks into now (an errand's fix on a covering finder is not that: it waits)
  const eye = c.bones.head ?? v3.add(c.pos, [0, 1.65, 0]); const dB = v3.distXZ(c.pos, bp);
  const seeBody = dB < 8 && !g.col.segmentBlocked(eye, [bp[0], 0.4, bp[2]]); const onBody: Vec3 = [bp[0], 0.35, bp[2]];
  gd.awareness = Math.max(gd.awareness, D.stage === 'fan' ? 0.16 : 0.3);   // the script's own clocks end it, not the suspicion decay
  let move = 0, aim: Vec3 | null = null;
  switch (D.stage) {
    case 'to': {
      if (!D.spot) { D.spot = coverSpot(g, gd, bp, D.role === 'finder' ? 1.5 : 2.1); gd.lastKnown = v3.copy(D.spot); gd.path = []; gd.pathGoal = null; gd.stallRef = null; }
      if (gd.reactT > 0) { gd.reactT -= dt; gd.speed = damp(gd.speed, 0, 8, dt); c.vel = [0, 0, 0]; aim = seeBody ? onBody : [bp[0], 1.0, bp[2]]; break; }
      goTo(g, gd, D.spot); const arrived = followPath(g, gd, dt, T.guardInvestigate) || notGettingAnywhere(gd, dt, 3); move = gd.speed;   // (a leg that is going nowhere ends where he stands)
      aim = seeBody ? onBody : null;
      if (arrived || (dB < 2.6 && seeBody && v3.distXZ(c.pos, D.spot) < 1.5)) {
        D.t = 0; gd.path = []; gd.pathGoal = null;
        if ((arrived && dB < 3.5) || seeBody) {   // actually at him (not merely at the end of a path that could not reach him)
          B.bodyVisits++; B.bodyVisitT = g.time; D.stage = 'cover';
          if (D.role !== 'finder' && !B.bodyRemarked && !(gd.bubble?.radio && g.time - gd.bubble.t < 5)) { B.bodyRemarked = true; g.say(gd, '…christ. — stay sharp'); }   // (not on top of his own radio call — with two men left, the one beside the finder is also the one who gave the order)
        } else { D.stage = 'fan'; D.spot = fanSpot(g, gd, B, bp); gd.stallRef = null; gd.searchT = 0; if (D.spot) gd.lastKnown = v3.copy(D.spot); }
      }
      break;
    }
    case 'cover': {
      gd.speed = damp(gd.speed, 0, 8, dt); c.vel = [0, 0, 0]; aim = onBody; D.t += dt;
      const done = D.role === 'finder' ? D.t > 6 || (D.t > 3 && B.bodyVisits >= 2 && g.time - B.bodyVisitT > 1.5) : D.t > 1.8;   // the finder holds it until somebody has joined him and had his look (or long enough alone); the others look and go
      if (done) { D.stage = 'fan'; D.t = 0; D.spot = fanSpot(g, gd, B, bp); gd.stallRef = null; gd.searchT = 0; gd.path = []; gd.pathGoal = null; if (D.spot) gd.lastKnown = v3.copy(D.spot); }
      break;
    }
    case 'fan': {
      if (!D.spot) { gd.awareness = Math.min(gd.awareness, 0.1); break; }
      let arrived = gd.searchT > 0;   // (once the look round has begun he stays put: a shove off the cell must not restart the walk)
      if (!arrived) { goTo(g, gd, D.spot); arrived = followPath(g, gd, dt, T.guardWalk * 1.3) || notGettingAnywhere(gd, dt, 3); move = gd.speed; }
      if (!arrived) { gd.sweep = damp(gd.sweep, Math.sin(gd.lookPhase * 0.9) * 0.7, 2, dt); c.aimYaw = c.bodyYaw + gd.sweep; }
      else { gd.speed = damp(gd.speed, 0, 8, dt); c.vel = [0, 0, 0]; gd.searchT += dt; gd.sweep = Math.sin(gd.lookPhase * 1.1) * 1.1; c.anim.lookYawExtra = Math.sin(gd.lookPhase * 1.9) * 0.4; c.aimYaw = c.bodyYaw + gd.sweep; if (gd.searchT > 4) gd.awareness = Math.min(gd.awareness, 0.1); }
      break;
    }
  }
  return { move, aim };
}

export function canSee(g: Game, gd: Guard): { visible: boolean; dist: number; inCone: boolean } {
  const pc = g.player.char; const eye = gd.char.bones.head ?? v3.add(gd.char.pos, [0, 1.65, 0]);
  const tgt = pc.bones.chest ?? v3.add(pc.pos, [0, 1.1, 0]); const tgt2 = pc.bones.head ?? v3.add(pc.pos, [0, 1.6, 0]);
  const to = v3.sub(tgt, eye); const dist = v3.len(to); const dirTo = v3.scale(to, 1 / Math.max(dist, 1e-4));
  const look = v3.normalize(v3.lerp(gd.char.chestDir, gd.char.headDir(), 0.5));
  const cosA = v3.dot([look[0], 0, look[2]], v3.normalize([dirTo[0], 0, dirTo[2]]));
  const fov = dist < 3.5 ? Math.cos(100 * DEG) : Math.cos(58 * DEG);
  const inCone = cosA > fov && dist < 16;
  if (!inCone || g.playerInvisible || g.player.down || g.time < gd.dazzledUntil) return { visible: false, dist, inCone };   // dazzled: eyes full of magnesium
  const blocked = (g.col.segmentBlocked(eye, tgt) && g.col.segmentBlocked(eye, tgt2)) || gd.smokeTrans < 0.3;
  return { visible: !blocked, dist, inCone };
}

export function goTo(g: Game, gd: Guard, goal: Vec3, force = false) {
  if (!force && gd.pathGoal && v3.dist(gd.pathGoal, goal) < 0.75 && gd.path.length) return;
  const p = g.col.nav.findPath(gd.char.pos, goal);
  // no route (the fix is inside a wall / a sealed pocket): only fall back to a bee-line when the straight segment is actually clear — otherwise an empty path,
  // which followPath reports as 'arrived' so the state machine moves on (search here) instead of grinding a partition for the rest of the encounter
  const direct = !p && !g.col.segmentBlocked([gd.char.pos[0], 0.5, gd.char.pos[2]], [goal[0], 0.5, goal[2]]);
  gd.path = g.doors.threadPath(p ?? (direct ? [v3.copy(goal)] : []), gd.char.pos, (x, z) => !g.col.nav.isBlocked(x, z)); gd.pathI = 0; gd.pathGoal = v3.copy(goal); gd.repathT = 0;
}

export function followPath(g: Game, gd: Guard, dt: number, speed: number): boolean { // returns true when arrived
  const c = gd.char;
  // progress watchdog: if we wanted to move but barely did (wedged on a prop corner / door edge), sidestep, then repath
  const moved = Math.hypot(c.pos[0] - gd.lastPos[0], c.pos[2] - gd.lastPos[2]); gd.lastPos = v3.copy(c.pos);
  if (gd.speed > 0.3 && moved < gd.speed * dt * 0.25) gd.stuckT += dt; else gd.stuckT = Math.max(0, gd.stuckT - dt * 2);
  if (gd.stuckT > 0.7 && gd.pathI < gd.path.length) {
    const wp = gd.path[gd.pathI]; const dx = wp[0] - c.pos[0], dz = wp[2] - c.pos[2]; const d = Math.hypot(dx, dz) || 1;
    const side = (gd.stuckN++ & 1) ? 1 : -1; const px = -dz / d * side, pz = dx / d * side;
    const cand: Vec3 = [c.pos[0] + px * 0.7 + dx / d * 0.2, 0, c.pos[2] + pz * 0.7 + dz / d * 0.2];
    if (!g.col.nav.isBlocked(cand[0], cand[2])) gd.path.splice(gd.pathI, 0, cand);
    else if (gd.pathGoal) { const goal = gd.pathGoal; gd.pathGoal = null; goTo(g, gd, goal, true); }
    gd.stuckT = 0;
  }
  while (gd.pathI < gd.path.length) {
    const wp = gd.path[gd.pathI]; const d = Math.hypot(wp[0] - c.pos[0], wp[2] - c.pos[2]);
    if (d < (gd.pathI === gd.path.length - 1 ? 0.25 : 0.45)) { gd.pathI++; continue; }
    const dir: Vec3 = [(wp[0] - c.pos[0]) / d, 0, (wp[2] - c.pos[2]) / d];
    // slow down when facing away from the movement direction and near the goal
    const yawTo = Math.atan2(dir[0], dir[2]); const facing = Math.cos(wrapAngle(yawTo - c.bodyYaw));
    const remaining = d + (gd.path.length - 1 - gd.pathI) * 1.0;
    const tgtSpeed = speed * clamp(facing * 1.5, 0.15, 1) * clamp(remaining / 0.8, 0.35, 1);
    gd.speed = damp(gd.speed, tgtSpeed, 4, dt);
    c.bodyYaw = approachAngle(c.bodyYaw, yawTo, 5.5 * dt);
    c.vel = [Math.sin(c.bodyYaw) * gd.speed, 0, Math.cos(c.bodyYaw) * gd.speed];
    c.pos = v3.mad(c.pos, c.vel, dt);
    return false;
  }
  gd.speed = damp(gd.speed, 0, 10, dt); c.vel = [0, 0, 0];
  return true;
}

export function updateGuard(g: Game, gd: Guard, dt: number) {
  const c = gd.char; const T = g.tune; const route = g.level.routes[gd.routeI];
  if (gd.state === 'dead') {
    gd.speed = 0; c.vel = [0, 0, 0]; c.anim.speed = 0; c.update(dt); g.bodyThud(c); attachFlashlight(g, gd); return;   // (update = the ragdoll; free once it sleeps)
  }
  if (gd.script) { gd.drawn = (gd.script.upper ?? 'aim') === 'aim'; runGuardScript(g, gd, dt); return; }   // choreographed (tour beat / squad drill): squad.ts owns him this frame — and what is in his hand follows the ordered pose ('aim' = the pistol), not the AI state
  const lvl = escalationOf(g);   // the floor's alarm level (0 calm · 1 heightened · 2 lockdown; always 0 under the tour): how he carries himself while calm
  gd.drawn = lvl >= 1;           // heightened: the sidearm is out even on patrol — weapon light instead of the hand torch (handProps / attachFlashlight / killGuard)
  // --- perception ---
  let seen = false; let dist = 99;
  if (g.aiEnabled) {
    hearingCheck(g, gd);
    const vis = canSee(g, gd); dist = vis.dist;
    if (vis.visible) {
      const rate = sightGain(g, gd, dist);
      if (rate > 0.05) { gd.awareness = clamp(gd.awareness + rate * dt, 0, 1); gd.lastKnown = v3.copy(g.player.char.pos); seen = rate > 0.25; if (seen) gd.lastSeenT = g.time; }   // a faint glimpse updates where to look, only a real sighting refreshes the licence to shoot
    }
    gd.sawPlayerThisFrame = seen;
    // a downed colleague in view → cover him and radio it in, the alarm goes up a step around where he lies, and the floor's short script for a found body begins
    // (bodyAftermath): the finder walks up short of him and covers, the others come over — the second man there gets the one line — and every one of them then
    // widens out to a search point away from the body instead of milling on it and talking himself calm over the corpse. (A short refractory per finder: two men
    // lying together are one 'man down', not two calls in two frames; a body already called in startles nobody — bodyInView skips it.)
    if (gd.state !== 'alert' && !gd.bodyDuty && g.time - gd.sawBodyT > 20) {
      const o = bodyInView(g, gd);
      if (o) {
        const body = bodyPos(o); const eye = gd.char.bones.head ?? v3.add(gd.char.pos, [0, 1.65, 0]);
        gd.sawBodyT = g.time; gd.awareness = Math.max(gd.awareness, 0.7); gd.lastKnown = v3.copy(body); gd.reactT = 1.2;
        const before = g.guards.filter(x => x.state === 'dead' && x.found).length;   // (bodies already called in earlier: the order's wording, raiseEscalation)
        let n = 0; for (const x of g.guards) if (x.state === 'dead' && !x.found && (x === o || (v3.distXZ(bodyPos(x), body) < 3 && !g.col.segmentBlocked(eye, v3.add(bodyPos(x), [0, 0.3, 0]))))) { x.found = true; tally(g, 'bodies'); n++; }   // a body counts once, however many find it — and men lying together in his sight are found together (one call, one script, not a second 'man down' the moment the first one's ends; one hidden through the wall stays hidden)
        if (gd.state === 'patrol' || gd.state === 'search') { gd.state = 'suspicious'; gd.searchT = 0; gd.path = []; gd.pathGoal = null; }   // (a searcher too: he has something concrete to stand over now)
        g.say(gd, n > 1 ? `${n} men down! — radioing it in` : 'man down! — radioing it in', true); g.audio.play('radio', eye, 0.9);
        if (!g.quietUtility) {   // (the tour's staged bodies get the call and nothing more: its guards stay on their marks)
          g.alarm.pos[0] = body[0]; g.alarm.pos[1] = 0; g.alarm.pos[2] = body[2]; g.alarm.placed = true;
          raiseEscalation(g, lvl >= 1 || g.alarm.episode ? 2 : 1, gd, 'body', before > 0);   // a body is as real as an alarm gets: a calm floor goes to heightened on it, an alarmed one (or one still hunting somebody) to lockdown, held around where he lies, its pair sent round the rooms nearest him (somebody else gives the order so the finder's own line stays up) — first, so a pair it pulls off a clear hears the call below as fresh men
          gd.task = null; gd.bodyDuty = { body: o, role: 'finder', stage: 'to', t: 0, t0: g.time, spot: null };   // (a colleague on the floor outranks the breaker / the rack he was walking to)
        }
        for (const other of g.guards) if (other !== gd && other.state !== 'dead') {
          other.awareness = Math.max(other.awareness, 0.45);
          if (other.task) continue;   // on an errand (the breaker, the rack): he hears it on the net, sharpens up, and keeps going — his fix stays the errand's
          other.lastKnown = v3.copy(body);
          if (!g.quietUtility && !other.hold && !other.bodyDuty && (other.state === 'patrol' || other.state === 'suspicious' || !!other.script)) other.bodyDuty = { body: o, role: 'called', stage: 'to', t: 0, t0: g.time, spot: null };   // (a scripted man — the clearing pair — breaks off on the news and comes over like the rest; an alert or searching one keeps hunting, he only learns where)
        }
      }
    }
    // lockdown pair: the man he is trailing stops dead and brings his weapon up at something ('huh?') — he takes it exactly as seriously, a step to that man's
    // side of the spot, without a 'huh?' of his own (only while the leader's start is fresh, reactT > 0: one look per noise, not a loop of re-inheriting it)
    if (lvl >= 2 && gd.leader && gd.state === 'patrol' && !gd.hold && gd.leader.state === 'suspicious' && gd.leader.reactT > 0 && gd.leader.lastKnown) {
      const L = gd.leader, lk = L.lastKnown!, f = L.char.forward();
      const side: Vec3 = [lk[0] + f[2] * 1.1, 0, lk[2] - f[0] * 1.1];   // a metre to the left of it as the leader sees it (his left = [f.z, 0, −f.x] in this handedness)
      gd.state = 'suspicious'; gd.awareness = Math.max(gd.awareness, Math.min(L.awareness, 0.6), 0.34); gd.reactT = L.reactT + 0.3; gd.searchT = 0;
      gd.lastKnown = g.col.nav.walkable(lk, side) ? side : v3.copy(lk); gd.path = []; gd.pathGoal = null;   // (walkable from the spot itself: 'a metre to the left' must not mean the next room)
    }
    // a kicked-in lock, seen up close on a calm pass (within a couple of metres, his side of the doorway in his field of view): worth one look per door per
    // encounter — he draws, walks up to it, looks about, and talks himself out of it like any other unexplained thing (a broken lock cannot be relocked, lockdown or not)
    if (gd.state === 'patrol' && !gd.hold && !g.quietUtility) for (const d of g.doors.list) {   // (not under the tour: its guards stay on their marks)
      if (!d.lockBroken || d.noticed) continue;
      const f = d.frameCentre; const dx = f[0] - c.pos[0], dz = f[2] - c.pos[2]; const dd = Math.hypot(dx, dz); if (dd > 2.2 || dd < 1e-3) continue;
      const look = c.chestDir; if ((look[0] * dx + look[2] * dz) / dd < Math.cos(75 * DEG)) continue;
      const spot = g.doors.workSpot(d, c.pos, 0.45, d.def.width * 0.5).pos;   // just his side of the doorway's middle (short of the leaf, so the leaf itself never hides 'the door')
      const eye = c.bones.head ?? v3.add(c.pos, [0, 1.65, 0]); if (g.col.segmentBlocked(eye, [spot[0], 1.0, spot[2]])) continue;
      d.noticed = true; d.noticedT = g.time; gd.state = 'suspicious'; gd.awareness = Math.max(gd.awareness, 0.45); gd.reactT = 1.1; gd.searchT = 0; gd.lastKnown = spot; gd.path = []; gd.pathGoal = null;
      g.say(gd, 'this lock\'s been kicked in…'); g.audio.play('radio', eye, 0.4, { rate: 1.1 });
      break;
    }
    if (!seen) gd.awareness = Math.max(0, gd.awareness - dt * (gd.state === 'alert' ? 0.03 : gd.state === 'patrol' ? 0.1 : 0.06));
  } else { gd.awareness = Math.max(0, gd.awareness - dt * 0.5); }

  // --- state transitions ---
  const prev = gd.state;
  if (gd.awareness >= 0.85 && gd.state !== 'alert') { gd.alertBest = 1e9; gd.alertStallT = 0; gd.state = 'alert'; gd.reactT = Math.max(gd.reactT, 0.35); gd.fireCd = 0.6 + Math.random() * 0.4; g.say(gd, 'CONTACT!'); g.audio.play('radio', c.bones.head ?? c.pos, 0.8); if (g.time - g.lastStingT > 8) { g.audio.play('alertSting', null, 0.75); g.lastStingT = g.time; } }
  else if (gd.awareness >= 0.3 && gd.state === 'patrol') {
    gd.state = 'suspicious'; gd.searchT = 0;
    if (gd.bodyDuty) gd.reactT = 0.5 + Math.random() * 0.5;   // called to a body over the net: no 'huh?' — a beat to take it in, then over to him (bodyAftermath)
    else { gd.reactT = 0.9; g.say(gd, 'huh?'); g.audio.play('radio', c.bones.head ?? c.pos, 0.45, { rate: 1.15 }); }
  }
  if (prev !== gd.state) { gd.path = []; gd.pathGoal = null; gd.stallRef = null; if (gd.state !== 'suspicious') gd.bodyDuty = null; }   // (the body script lives inside 'suspicious': contact or calm ends it)
  const dazzled = g.time < gd.dazzledUntil;
  if (dazzled) gd.reactT = Math.max(gd.reactT, gd.dazzledUntil - g.time);   // the transitions above reset reactT; a dazzled guard stays rooted (suspicious / alert both hold still while reactT > 0)
  // (alarm episodes are counted in updateMission: someone going to alert while nobody was)       // mission stats: every time a man goes to alert (from patrol, suspicion or a lapsed search alike)

  // --- behaviour ---
  c.anim.lookYawExtra = 0; gd.lookPhase += dt;
  let moveSpeed = 0; let upper: 'none' | 'relaxed' | 'aim' = 'none'; let aimAt: Vec3 | null = null;
  switch (gd.state) {
    case 'patrol': {
      // heightened (lvl ≥ 1): the pistol is out at a low ready instead of the torch, the route is walked ~15 % brisker with shorter pauses and a wider sweep of the
      // muzzle into the corners; lockdown (2): a paired follower trails his leader instead of walking his own route, the spare man holds his junction (updateEscalation
      // hands those out), and anyone passing an open or unlocked door deals with it
      const target = route.points[gd.wp];
      if (gd.drawn) upper = 'aim';
      if (gd.bodyDuty && gd.awareness < 0.3) gd.bodyDuty = null;   // the call never took (the AI was off, he was reset under it): it must not steer some later, unrelated suspicion to an old body
      const walk = T.guardWalk * (lvl >= 1 ? 1.15 : 1);
      if (gd.hold) { gd.sweep = damp(gd.sweep, Math.sin(gd.lookPhase * 0.5) * 0.2, 2, dt); c.vel = [0, 0, 0]; }
      else if (lvl >= 2 && gd.leader) { if (followLeader(g, gd, dt, walk)) moveSpeed = gd.speed; }   // (a leader taken down quietly is still 'his leader' until somebody knows — superviseLockdown — so he closes up to his station behind the body, and finds him)
      else if (lvl >= 2 && gd.post) { if (holdPost(g, gd, dt, walk)) moveSpeed = gd.speed; }
      else if (gd.wait > 0) {
        gd.wait -= dt; gd.sweep = Math.sin(gd.lookPhase * 0.9) * 0.7; c.anim.lookYawExtra = Math.sin(gd.lookPhase * 1.3) * 0.5;
        if (gd.wait <= 0) { gd.wp = (gd.wp + 1) % route.points.length; }
      } else {
        goTo(g, gd, target);
        let pace = walk;
        if (lvl >= 2) for (const o of g.guards) if (o.leader === gd && o.state === 'patrol' && v3.dist(o.char.pos, c.pos) > 4) pace = walk * 0.55;   // leading a pair: don't walk off from the man who is supposed to be on your shoulder (a turnabout leaves him ahead of you for a moment)
        if (followPath(g, gd, dt, pace)) { gd.wait = (route.wait[gd.wp] ?? 0) * (lvl >= 1 ? 0.6 : 1); if (gd.wait <= 0) gd.wp = (gd.wp + 1) % route.points.length; }
        gd.sweep = damp(gd.sweep, Math.sin(gd.lookPhase * 0.7) * (gd.drawn ? 0.4 : 0.25), 2, dt);
        moveSpeed = gd.speed;
      }
      c.aimYaw = c.bodyYaw + gd.sweep; c.aimPitch = damp(c.aimPitch, gd.drawn ? -0.26 : -0.35, 3, dt);   // drawn: muzzle (and its light) at a low ready, pooling on the floor five metres out
      if (lvl >= 2 && !gd.hold) lockdownDoors(g, gd);
      break;
    }
    case 'suspicious': {
      upper = 'aim';
      if (gd.task) { // errand (e.g. go and look at the breaker): don't talk yourself out of it halfway down the corridor
        if (!gd.lastKnown || v3.dist(gd.lastKnown, gd.task.pos) > 0.5) { if (gd.awareness < 0.45) gd.lastKnown = v3.copy(gd.task.pos); }
        gd.awareness = Math.max(gd.awareness, 0.2);
        if (gd.searchT > 4) gd.task = null;
      }
      const duty = gd.bodyDuty ? bodyAftermath(g, gd, dt) : null;   // a found body has its own short script (drops itself, → null, the moment it no longer applies)
      if (duty) { moveSpeed = duty.move; aimAt = duty.aim; }
      else if (gd.reactT > 0) { gd.reactT -= dt; gd.speed = damp(gd.speed, 0, 8, dt); if (gd.lastKnown) aimAt = v3.add(gd.lastKnown, [0, 1.0, 0]); }
      else if (gd.lastKnown && gd.hold) { gd.speed = damp(gd.speed, 0, 8, dt); c.vel = [0, 0, 0]; aimAt = v3.add(gd.lastKnown, [0, 1.0, 0]); gd.searchT += dt; if (gd.searchT > 6) gd.awareness = Math.min(gd.awareness, 0.1); }   // scripted hold: watch it from where you stand, then let it go
      else if (gd.lastKnown) {
        goTo(g, gd, gd.lastKnown);
        const arrived = followPath(g, gd, dt, T.guardInvestigate); moveSpeed = gd.speed;
        aimAt = v3.add(gd.lastKnown, [0, 0.9, 0]);
        if (arrived) { gd.stallRef = null; gd.searchT += dt; gd.sweep = Math.sin(gd.lookPhase * 1.1) * 1.1; c.anim.lookYawExtra = Math.sin(gd.lookPhase * 1.9) * 0.4; aimAt = null; c.aimYaw = c.bodyYaw + gd.sweep; if (gd.searchT > 5) { gd.awareness = Math.min(gd.awareness, 0.2); } }
        else {
          gd.searchT = 0;   // still on the way (possibly to a NEW noise): the look-around clock starts on arrival, a stale one must not cut the search short
          // …but four seconds of getting nowhere (a leaf somebody is holding against him, a colleague planted on the spot, a fix just inside a wall): he looks about from
          // where he stands instead of grinding at it until the suspicion wears off — his fix becomes the spot he is on (the alert state has its own such escape)
          if (!gd.task && notGettingAnywhere(gd, dt, 4)) { gd.lastKnown = v3.copy(c.pos); gd.stallRef = null; gd.path = []; gd.pathGoal = null; }   // (not on an errand: its block above puts the fix straight back, and the breaker is worth grinding toward)
        }
      }
      if (gd.awareness < 0.12) {   // talked himself calm. The line depends on what it was: over a found body one man closes it for everybody, and nobody within a few metres of a
        const D = gd.bodyDuty;     // body the floor knows about ever calls it 'nothing' / 'clear' (that read as the men cycling platitudes over a dead colleague); once alarmed, nothing is 'nothing'
        gd.state = 'patrol'; gd.path = []; gd.pathGoal = null; gd.bodyDuty = null;
        if (g.player.down) { if (!g.stoodDown) { g.stoodDown = true; g.say(gd, 'back to your posts', true); } }
        else if (D) { if (!D.body.bodyClosed && D.body.bodyVisits > 0 && v3.distXZ(c.pos, bodyPos(D.body)) < 12) { D.body.bodyClosed = true; g.say(gd, "no sign of him — eyes open, he's close"); } }   // (once, by a man thereabouts, and only if anybody got to the body at all — a called man giving up two rooms off says nothing)
        else if (!nearKnownBody(g, c.pos, 4)) g.say(gd, lvl >= 1 ? 'clear here — staying on it' : 'must have been nothing');
      }
      c.aimPitch = damp(c.aimPitch, -0.12, 3, dt);
      break;
    }
    case 'alert': {
      upper = 'aim';
      const pc = g.player.char; const canShoot = !g.player.down && !dazzled && (g.time - gd.lastSeenT < 0.5);   // half a second of grace after a real sighting; a dazzled guard cannot shoot at all
      if (gd.reactT > 0) { gd.reactT -= dt; aimAt = (dazzled || g.time - gd.lastSeenT > 0.5) && gd.lastKnown ? v3.add(gd.lastKnown, [0, 1.1, 0]) : v3.add(pc.pos, [0, 1.2, 0]); gd.speed = damp(gd.speed, 0, 8, dt); }   // heard, not seen: he covers where he thinks it came from, not your true position through three walls   // a dazzled guard covers where he last knew you were, not where you are
      else if (canShoot) {
        aimAt = v3.add(pc.pos, [0, g.player.crouch ? 0.8 : 1.25, 0]);
        // keep some distance, otherwise hold position and fire (a pinned guard fires from where he was put)
        if (dist > 9 && !gd.pinned) { goTo(g, gd, pc.pos); followPath(g, gd, dt, T.guardChase); moveSpeed = gd.speed; } else { gd.speed = damp(gd.speed, 0, 6, dt); c.vel = [0, 0, 0]; }
        gd.fireCd -= dt;
        const facingErr = Math.abs(wrapAngle(Math.atan2(aimAt[0] - c.pos[0], aimAt[2] - c.pos[2]) - c.aimYaw));
        if (gd.reloadT >= 0) { gd.reloadT -= dt; if (gd.reloadT < 0) { gd.shots = 0; } }
        else if (gd.fireCd <= 0 && facingErr < 0.25) {
          gd.fireCd = 0.55 + Math.random() * 0.5; fireWeapon(g, c, aimAt, false); gd.shots++;
          if (gd.shots >= 12) { gd.reloadT = 1.7; c.anim.reload(); g.audio.play('magOut', c.pos, 0.5); g.say(gd, 'reloading!'); }
        }
      } else if (g.player.down) {
        // the target is down: close to a couple of metres, cover the body, one of them calls it in — nobody 'searches' for a man lying in front of them
        const body = pc.pos; const dB = Math.hypot(body[0] - c.pos[0], body[2] - c.pos[2]);
        if (dB > 2.2) { goTo(g, gd, body); followPath(g, gd, dt, T.guardInvestigate); moveSpeed = gd.speed; } else { gd.speed = damp(gd.speed, 0, 6, dt); c.vel = [0, 0, 0]; }
        aimAt = v3.add(body, [0, 0.3, 0]); gd.searchT += dt;
        if (gd.searchT > 4 && !g.calledIn) { g.calledIn = true; g.say(gd, 'target is down. send someone up.', true); g.audio.play('radio', c.bones.head ?? c.pos, 0.5, { rate: 0.95 }); }
        if (gd.searchT > 14) { gd.state = 'suspicious'; gd.awareness = 0.3; gd.searchT = 0; gd.reactT = 0; gd.lastKnown = v3.copy(body); }   // eventually they lower their weapons and mill about the body
      } else if (gd.lastKnown && gd.pinned) {   // scripted: hold the position, cover where he was last seen
        gd.speed = damp(gd.speed, 0, 6, dt); c.vel = [0, 0, 0]; aimAt = v3.add(gd.lastKnown, [0, 1.1, 0]);
      } else if (gd.lastKnown) {
        goTo(g, gd, gd.lastKnown); let arrived = followPath(g, gd, dt, T.guardChase); moveSpeed = gd.speed;
        aimAt = arrived ? null : v3.add(gd.lastKnown, [0, 1.1, 0]);
        // no way of getting nearer (a leaf across an alley, a carton shoved over the spot, a fix inside a wall): after a few seconds of no progress he searches
        // from where he is rather than staying 'alert' for ever — which also kept the whole floor's alarm episode (and its step-down) from ever ending
        const dFix = v3.distXZ(gd.lastKnown, c.pos);
        if (dFix < gd.alertBest - 0.5) { gd.alertBest = dFix; gd.alertStallT = 0; } else if ((gd.alertStallT += dt) > 5) { arrived = true; gd.alertStallT = 0; }
        if (arrived) { gd.state = 'search'; gd.searchT = 0; gd.awareness = Math.min(gd.awareness, 0.8); gd.alertBest = 1e9; g.say(gd, 'where did he go?'); }   // below the alert threshold: re-alerting needs a fresh sighting / sound, otherwise alert↔search flaps every reactT
      }
      break;
    }
    case 'search': {
      upper = 'aim';
      if (g.player.down) { gd.state = 'alert'; gd.reactT = 0; gd.searchT = 0; break; }   // a body on the floor is not a search: go and cover it (alert branch above)
      gd.searchT += dt; gd.sweep = Math.sin(gd.lookPhase * 1.2) * 1.3; c.anim.lookYawExtra = Math.sin(gd.lookPhase * 2.1) * 0.45;
      c.aimYaw = c.bodyYaw + gd.sweep; gd.speed = damp(gd.speed, 0, 6, dt);
      // wander to a couple of nearby points (not while dazzled: a blind man does not go for a walk)
      if (!dazzled && gd.searchT > 3 && gd.lastKnown && (!gd.path.length || gd.pathI >= gd.path.length)) {
        const a = Math.random() * Math.PI * 2; const cand: Vec3 = [gd.lastKnown[0] + Math.cos(a) * 3, 0, gd.lastKnown[2] + Math.sin(a) * 3];
        if (!g.col.nav.isBlocked(cand[0], cand[2])) { goTo(g, gd, cand, true); gd.stallRef = null; }
      }
      if (!dazzled && gd.path.length && gd.pathI < gd.path.length) {
        followPath(g, gd, dt, T.guardInvestigate); moveSpeed = gd.speed; c.aimYaw = c.bodyYaw + gd.sweep * 0.5;
        // a wander point he cannot actually get to (behind a leaf standing at its stop, say): three seconds of no headway and he drops it and sweeps from where he is
        if (notGettingAnywhere(gd, dt, 3)) { gd.path = []; gd.pathGoal = null; gd.stallRef = null; }
      }
      if (gd.searchT > 12) { gd.state = 'suspicious'; gd.awareness = 0.25; gd.searchT = 0; gd.reactT = 0; g.say(gd, 'returning to patrol'); }
      c.aimPitch = damp(c.aimPitch, -0.1, 3, dt);
      break;
    }
  }
  if (aimAt) {
    const yawTo = Math.atan2(aimAt[0] - c.pos[0], aimAt[2] - c.pos[2]);
    c.aimYaw = approachAngle(c.aimYaw, yawTo, 6 * dt);
    const gunY = c.pos[1] + 1.4; const hd = Math.hypot(aimAt[0] - c.pos[0], aimAt[2] - c.pos[2]);
    c.aimPitch = damp(c.aimPitch, Math.atan2(aimAt[1] - gunY, Math.max(hd, 0.5)), 5, dt);
    // turn body if aim exceeds twist range or when standing
    const off = wrapAngle(c.aimYaw - c.bodyYaw);
    if (moveSpeed < 0.3 && Math.abs(off) > 0.5) c.bodyYaw = approachAngle(c.bodyYaw, c.aimYaw, 5 * dt);
  }
  // clamp aim into twist range
  const off = wrapAngle(c.aimYaw - c.bodyYaw); if (Math.abs(off) > 1.5) c.aimYaw = c.bodyYaw + Math.sign(off) * 1.5;
  // collisions
  g.col.collideCircle(c.pos, c.radius, 0.2, 1.5);   // same height band as the nav grid bake
  for (const o of g.guards) { if (o === gd || o.state === 'dead') continue; const d = v3.sub(c.pos, o.char.pos); d[1] = 0; const l = v3.len(d); if (l < 0.6 && l > 1e-4) c.pos = v3.mad(c.pos, d, (0.6 - l) / l * 0.5); }
  c.anim.speed = moveSpeed; c.anim.upper = upper; c.anim.crouchTarget = 0; c.anim.stance = 'none';   // (the AI never holds a tactical stance yet — a script that just ended must not leave one on him)
  c.update(dt);
  attachFlashlight(g, gd);
  const prevPhase = gd.stepPhase; gd.stepPhase = c.anim.phase;
  if (moveSpeed > 0.4 && ((prevPhase < 0.5) !== (gd.stepPhase < 0.5))) g.audio.footstep(c.pos, moveSpeed > 2 ? 0.8 : 0.45, false);
}

export function attachFlashlight(g: Game, gd: Guard) {
  const c = gd.char; const fl = gd.flashlight; const T = g.tune;
  if (!c.bones.handR) return;   // no pose yet
  let dir = c.gunDir;
  const torch = gd.state === 'patrol' && !gd.drawn;   // the hand torch on a calm patrol; the weapon light whenever the pistol is out (drawn on an alarmed floor, or any other state)
  if (torch && c.alive) { gd.beamDir = torchCarryDir(c, gd.beamDir); dir = gd.beamDir; }   // relaxed carry: beam follows the chest/aim heading, smoothed so the arm swing only bobs the origin
  else { gd.beamDir = v3.copy(dir); }
  fl.pos = rigTorchPos(c, dir, torch); fl.dir = dir;   // at the torch lens on patrol, at the weapon-light slot when drawn
  if (gd.state === 'dead' && gd.dropped) { const it = gd.dropped; const hd: Vec3 = [Math.sin(it.yaw) * Math.cos(it.tumble), -Math.sin(it.tumble), Math.cos(it.yaw) * Math.cos(it.tumble)]; fl.pos = v3.mad(it.pos, hd, 0.1); fl.dir = hd; }   // the beam spins with the tumbling torch / pistol until it dies
  if (gd.state === 'dead' && g.time >= gd.lightDeadUntil) {   // a dropped torch dies with its owner: a couple of sputters as it hits the floor, then dark (a light lying IN smoke at floor level is also the worst case for the volumetric estimate)
    const t = g.time - gd.diedAt; fl.enabled = t < 0.9 && (t < 0.35 || Math.sin(t * 60) > 0.3); fl.intensity = fl.peakIntensity * clamp(1 - t / 0.9, 0, 1); return;
  }
  if (g.time < gd.lightDeadUntil) { fl.enabled = false; }
  else if (gd.lightDeadUntil > 0 && g.time < gd.lightDeadUntil + 1.0) { fl.enabled = Math.sin(g.time * 50) > 0.2; }   // sputters back
  else fl.enabled = true;
  fl.peakIntensity = T.flashIntensity; fl.intensity = T.flashIntensity; fl.innerDeg = T.flashInner; fl.outerDeg = T.flashOuter; fl.radius = g.engine.settings.flashlightRadius;
}

export function onScreen(g: Game, p: Vec3): boolean {
  const vp = g.viewProj; if (!vp) return false;
  const w = vp[3] * p[0] + vp[7] * p[1] + vp[11] * p[2] + vp[15]; if (w <= 0) return false;
  const x = (vp[0] * p[0] + vp[4] * p[1] + vp[8] * p[2] + vp[12]) / w, y = (vp[1] * p[0] + vp[5] * p[1] + vp[9] * p[2] + vp[13]) / w;
  return x > -0.92 && x < 0.92 && y > -0.9 && y < 0.9;
}

/** A guard says something: a bubble over his head if he is on screen, otherwise (or if it went out over the radio) a line in the log. */
export function say(g: Game, gd: Guard, text: string, radio = false) {
  gd.bubble = { text, t: g.time, dur: Math.min(4, Math.max(2, 1.5 + text.length * 0.055)), radio };
  if (radio || !onScreen(g, gd.char.bones.head ?? v3.add(gd.char.pos, [0, 1.7, 0]))) g.msg((radio ? 'radio: ' : 'guard: ') + text);
}

/** Fresh guards at the start of their routes (keeps their flashlight lights and tints). */
export function resetGuards(g: Game) {
  if (g.player) { g.player.dragging = null; g.player.takedown = null; }   // the old bodies / targets are gone
  g.guards = g.guards.map(gd => new Guard(g.ch, gd.char.id, g.level.routes[gd.routeI], gd.routeI, gd.flashlight, gd.char.tint));
  for (const gd of g.guards) gd.char.update(0);          // fresh characters get a pose (bones) immediately — reset can happen mid-frame
  g.breakerFixT = 0; g.aimGuard = null;
  g.escalation = 0; g.escalationT = -1; g.alarm.episode = false; g.alarm.placed = false; g.alarm.apartT = 0;   // fresh men have no memory of an alarm: calm floor, torches out (the pairing / posts lived on the old Guard objects)
  g.clearing = null; g.clearedAt.clear();   // (so did the scripts of a pair that was clearing rooms, and their ledger of rooms done)
  g.note('guards reset');
}

// ------------------------------------------------------------ alarm escalation
// The floor remembers having been alarmed. An alarm EPISODE starts when anybody goes to 'alert' and runs until nobody is alert or searching any more; when it ends
// with nobody found (everyone who was hunting has given up or is dead, and the player is not lying at their feet) the level goes up: calm → heightened (pistols out
// on patrol, brisker routes, keener eyes and ears), and an episode that ends while already heightened → lockdown (pairs, a man on the junction nearest the last
// fix, doors pulled to and relocked). A body found is worth one step on its own (calm → heightened, heightened → lockdown). Each level holds for its clock and
// then steps down one at a time, but only while nobody is alert or searching. One radio line per change.
/** seconds each level holds before it may step down: heightened after an alarm that found nobody (or a body found on a calm floor), lockdown after a second inside
 *  that window (or a body found while heightened), and the shorter heightened spell a lapsed lockdown relaxes through on its way back to calm */
const HEIGHTENED_SECS = 75, LOCKDOWN_SECS = 120, STEPDOWN_SECS = 45;
/** how much harder an alarmed floor looks and listens: × the visual detection rate and the small-noise awareness gains (so 'huh?' comes that much sooner) */
const KEEN = 1.25;

/** The alarm level the guards act on: the game's, except under the showcase tour (quietUtility) whose staged firefights are nobody's memory (it is 0 there anyway —
 *  resetGuards zeroes it as the tour starts — this just makes the picture independent of that ordering). */
export function escalationOf(g: Game): 0 | 1 | 2 { return g.quietUtility ? 0 : g.escalation; }
export function escalationName(l: number): string { return l >= 2 ? 'lockdown' : l >= 1 ? 'heightened' : 'calm'; }

/** One line for the Tab panel / debugging: level, seconds left on its clock, whether an episode is running, and the lockdown duties. */
export function escalationSummary(g: Game): string {
  const lvl = g.escalation; const name = (gd: Guard) => g.level.routes[gd.routeI]?.name ?? `guard ${gd.char.id - 10}`;
  let s = `alarm level: ${escalationName(lvl)}${g.quietUtility && lvl ? ' (tour: ignored)' : ''}`;
  if (lvl > 0) s += ` · ${Math.max(0, Math.ceil(g.escalationT - g.time))} s`;
  if (g.alarm.episode) s += ' · episode running';
  for (const gd of g.guards) { if (gd.state === 'dead') continue; if (gd.leader) s += ` · pair ${name(gd.leader)} ▸ ${name(gd)}`; if (gd.post) s += ` · ${name(gd)} holds the ${gd.post.name}`; }
  return s;
}
/** The second panel line: what the pair is clearing right now (squad.ts Clearing.summary), or nothing. */
export function clearingSummary(g: Game): string { return g.clearing?.summary() ?? ''; }

/** the living man best placed to radio something about `pos` (or about nothing in particular): the nearest who is not mid-fight, else the nearest at all — never `except`
 *  (somebody whose own line should stay on screen: a finder left alone on the floor radios his 'man down' and nobody answers with orders to himself) */
function spokesman(g: Game, pos: Vec3 | null, except: Guard | null = null): Guard | null {
  let best: Guard | null = null, bd = Infinity;
  for (let pass = 0; pass < 2 && !best; pass++) for (const gd of g.guards) {
    if (gd.state === 'dead' || gd === except || (pass === 0 && gd.state === 'alert')) continue;
    const d = pos ? v3.dist(gd.char.pos, pos) : 0; if (d < bd) { bd = d; best = gd; }
  }
  return best;
}

/** Raise the floor to at least `to` and (re)arm that level's clock. One radio line per actual change (a lockdown re-armed by yet another alarm just says so), from the
 *  calm man nearest the last fix — `except` the one whose own line should stay up; `cause` picks the wording for a first raise (a search that found nobody, or a
 *  body). A lockdown deals the duties out at once so the post man's own line follows the order. */
export function raiseEscalation(g: Game, to: 1 | 2, except: Guard | null = null, cause: 'search' | 'body' = 'search', another = false) {   // (another: bodies had been called in before this one — the order says so)
  const prev = g.escalation; const lvl: 0 | 1 | 2 = to > prev ? to : prev;
  g.escalation = lvl; g.escalationT = g.time + (lvl >= 2 ? LOCKDOWN_SECS : HEIGHTENED_SECS);
  const again = g.time - g.alarm.raisedAt < 1.5; g.alarm.raisedAt = g.time;
  if (again && lvl <= prev && cause !== 'body') return;   // just raised (a searcher finding a body ends his hunt, and with it the episode, in the very frame the body raised the floor): the clock is re-armed, nothing else is new — no 'still nothing' on top of 'man down', no re-deal (a body always re-plans: the rooms round HIM)
  const sp = spokesman(g, g.alarm.placed ? g.alarm.pos : null, except);
  const line = lvl <= prev ? (cause === 'body' ? `${another ? 'another man down' : 'man down confirmed'} — stay locked down, clear everything round him` : 'still nothing — it stays locked down')
    : lvl >= 2 ? `${cause === 'body' ? (another ? 'another man down — ' : 'man down — ') : ''}lock it down: pairs, weapons out, nobody wanders`
    : cause === 'body' ? 'we have a man down — weapons out, everyone sharp' : 'nothing here — stay sharp, check your corners';
  if (sp) { g.say(sp, line, true); g.audio.play('radio', sp.char.bones.head ?? sp.char.pos, 0.7); }
  if (lvl >= 2) { assignLockdown(g, prev >= 2, except); planClears(g, false, cause); }   // (re-dealt on a re-arm too: they have just converged on a new fix, so pair whoever is together now and post the spare man by IT) — and the pair clears the rooms nearest that fix (more of them, and whatever they were on dropped, for a body)
}

/** Panel / debug: the floor calm again at once — level 0, clock off, pair and post dissolved, a clear in progress called off — without touching the men themselves (whoever is suspicious stays so). */
export function standDownEscalation(g: Game) { g.escalation = 0; g.escalationT = -1; g.alarm.episode = false; g.alarm.placed = false; g.clearing?.cancel(); g.clearing = null; g.clearedAt.clear(); clearLockdown(g); }

// ------------------------------------------------------------ room clearing (squad.ts Clearing / RoomClear, plans in level.ts)
/** seconds within which a room the pair has called clear is not sent round again in the same lockdown — unless the new fix is inside it */
const RECLEAR_SECS = 60;
/** seconds a picked lock they have found (and maybe locked back up) keeps putting its room at the front of a sweep */
const TAMPER_MEMORY_SECS = 120;
/** Locked down, and the alarm has just come to nothing or a body has been found (raiseEscalation, `cause`) — or the panel asks (`force`): hand the pair the rooms whose
 *  doors are nearest the last fix (the alarm's / the body; the point man's own spot if there never was one) to clear one after another — a body is worth the three
 *  nearest, an alarm two (`clearRooms` > 0 overrides both). Rooms whose door gives the intruder away (doorTamper: kicked in — the whole floor heard which one went —
 *  or a lock they have already found picked) jump the queue whatever their distance; a room called clear inside the last minute of this lockdown is left out unless
 *  the fix is in it. The pair as dealt: leader = point man, follower = his cover; a man left alone on the floor clears by himself. Never under the tour, with the AI
 *  off, or below lockdown; a clear already running is left be unless forced — or a body was found: that drops whatever they were on. */
export function planClears(g: Game, force = false, cause: 'search' | 'body' | 'panel' = 'search'): boolean {
  if (g.quietUtility || !g.aiEnabled || escalationOf(g) < 2) return false;
  if (g.clearing) { if (!force && cause !== 'body') return false; g.clearing.cancel(); g.clearing = null; }
  let a: Guard | null = null, b: Guard | null = null;
  for (const gd of g.guards) if (gd.state !== 'dead' && gd.leader && gd.leader.state !== 'dead') { a = gd.leader; b = gd; }
  if (!a) { const alive = g.guards.filter(x => x.state !== 'dead'); if (alive.length !== 1) return false; a = alive[0]; }
  // rooms ranked tampered-first, then by how near any of their doors is to the fix; a room's plans (one per door it can be taken from) travel together — Clearing picks the variant on the day
  const ref = g.alarm.placed ? g.alarm.pos : a.char.pos;
  const byRoom = new Map<string, { plans: RoomClearPlan[]; d: number; tampered: boolean; inside: boolean }>();
  for (const p of g.level.clearPlans ?? []) {
    const d = g.doors.byName(p.door); if (!d) continue;
    const k = p.room ?? p.door, r = byRoom.get(k) ?? { plans: [], d: Infinity, tampered: false, inside: false };
    r.plans.push(p); r.d = Math.min(r.d, v3.dist(d.frameCentre, ref));
    if (doorTamper(d) === 'forced' || (d.def.locked && d.noticed && g.time - d.noticedT < TAMPER_MEMORY_SECS)) r.tampered = true;   // kicked in (plain, and it stays plain), or a lock of theirs read as picked lately — by the pair's hand on it or by whoever locked it back up
    if (p.bounds && inBounds(ref, p.bounds)) r.inside = true;
    byRoom.set(k, r);
  }
  const n = g.clearRooms > 0 ? g.clearRooms : cause === 'body' ? 3 : 2;
  const recent = (k: string) => { const t = g.clearedAt.get(k); return t !== undefined && g.time - t < RECLEAR_SECS; };
  const ranked = [...byRoom.entries()].filter(([k, r]) => !recent(k) || r.inside).sort(([, x], [, y]) => (+y.tampered - +x.tampered) || x.d - y.d);
  const rooms = ranked.slice(0, n).map(([, r]) => r.plans);
  if (!rooms.length) { g.note('nothing near to clear — those rooms were done a minute ago'); return false; }
  g.clearing = new Clearing(g, rooms, a, b);
  return true;
}
/** Panel: the rooms still ahead of the pair (current first) and the ones called clear lately in this lockdown, with how long ago. */
export function clearQueueSummary(g: Game): string {
  const C = g.clearing; const parts: string[] = [];
  if (C && C.stage !== 'done') parts.push('clear queue: ' + C.rooms.slice(C.roomI).map(v => v[0].room ?? v[0].door).join(' ▸ '));
  const done = [...g.clearedAt.entries()].sort((x, y) => y[1] - x[1]).map(([k, t]) => `${k} (${Math.round(g.time - t)} s ago${g.time - t < RECLEAR_SECS ? ', holds' : ''})`);
  if (done.length) parts.push('cleared this lockdown: ' + done.join(', '));   // 'holds' = inside the minute planClears will not send them round it again
  return parts.join(' · ');
}
/** Panel: 'clear nearest room now' — the fix put where the player stands, the floor locked down if it was not (which deals the pair and plans off that fix), else re-planned. */
export function forceClear(g: Game) {
  if (g.quietUtility) { g.msg('no clear under the tour'); return; }   // (nor a lockdown raised behind its back for later)
  const p = g.player.char.pos; g.alarm.pos[0] = p[0]; g.alarm.pos[1] = 0; g.alarm.pos[2] = p[2]; g.alarm.placed = true;
  if (g.escalation < 2) raiseEscalation(g, 2); else planClears(g, true, 'panel');
  if (!g.clearing) g.note(!g.aiEnabled ? 'no clear: guard AI is off' : 'no clear: nobody to send / no plans');
  else g.note(`clear ordered: ${g.clearing.rooms.map(r => r[0].room ?? r[0].door).join(', ')}`);
}

/** One level down (its clock has run out and the floor is quiet): lockdown → a shorter heightened spell → calm, torches back out. */
function stepDownEscalation(g: Game) {
  const lvl: 0 | 1 | 2 = g.escalation >= 2 ? 1 : 0;
  g.escalation = lvl; g.escalationT = lvl >= 1 ? g.time + STEPDOWN_SECS : -1;
  if (lvl < 2) { clearLockdown(g); g.clearedAt.clear(); }   // (the next lockdown starts its own ledger of cleared rooms)
  if (lvl === 0) g.alarm.placed = false;   // whatever comes next is a fresh alarm somewhere else
  const sp = spokesman(g, g.alarm.placed ? g.alarm.pos : null);
  if (sp) { g.say(sp, lvl >= 1 ? 'ease off — singles, back on your routes; weapons stay out' : 'stand down, back to normal', true); g.audio.play('radio', sp.char.bones.head ?? sp.char.pos, 0.6, { rate: 0.95 }); }
}

/** Once a frame, after the guards: track the alarm episode off this frame's states, settle the level when it ends, step down when the clock allows, and keep the
 *  lockdown duties sane. Nothing moves under the tour, with the AI off, or for a player lying dead at their feet (that is the restart flow's business). */
export function updateEscalation(g: Game, dt: number) {
  if (g.quietUtility) return;
  const A = g.alarm;
  let anyAlert = false, hunting = false, living = 0;
  for (const gd of g.guards) {
    if (gd.state === 'dead') continue; living++;
    if (gd.state === 'alert') { anyAlert = hunting = true; if (gd.lastKnown) { A.pos[0] = gd.lastKnown[0]; A.pos[1] = 0; A.pos[2] = gd.lastKnown[2]; A.placed = true; } }   // the latest fix anybody alert is working from
    else if (gd.state === 'search') hunting = true;
  }
  if (anyAlert) { if (g.aiEnabled) A.episode = true; }
  else if (A.episode && !hunting) {   // over: everybody who went to 'alert' has given up the search too (or is dead), and nobody has had eyes on him since
    A.episode = false;
    if (living > 0 && !g.player.down && g.aiEnabled) raiseEscalation(g, g.escalation >= 1 ? 2 : 1);
  }
  if (g.escalation > 0 && !A.episode && !hunting && g.time >= g.escalationT) stepDownEscalation(g);
  if (g.escalation >= 2) superviseLockdown(g, dt, living);
  if (g.clearing && !g.clearing.tick(dt)) g.clearing = null;   // the pair clearing rooms: their approach, drill and senses this frame (it dissolves itself when done, broken off, or the floor changed under it)
}

/** Keep the lockdown duties dealt: (again) when there is no pair but two men live, when the leader died, when the pair has been > 8 m apart for 5 s while both were
 *  calm (stuck, or split by a chase), when a third man lives but nobody holds a post, or when only one man is left (he just patrols, drawn). */
function superviseLockdown(g: Game, dt: number, living: number) {
  // Roles react to what the men KNOW, not to ground truth: a man taken down quietly keeps 'his' role (his corpse holds it) until the body is found or the floor is
  // in an alarm episode anyway — otherwise a silent takedown had the man on the junction radio 'you're with me' and walk off it seconds before anyone knew.
  const A = g.alarm; const knownDead = (x: Guard) => x.state === 'dead' && (x.found || A.episode);
  let follower: Guard | null = null, postman: Guard | null = null, standing = 0;
  for (const gd of g.guards) { if (knownDead(gd)) continue; standing++; if (gd.leader) follower = gd; if (gd.post) postman = gd; }
  const posts = (g.level.chokepoints?.length ?? 0) > 0;   // (a level without chokepoints never has a post man — that must not read as 'deal again' every frame)
  let redo = standing >= 2 ? (!follower || knownDead(follower.leader!) || (standing >= 3 && posts && !postman)) : !!(follower || postman);
  if (follower && !redo && follower.state !== 'dead') {
    const L = follower.leader!; const calm = follower.state === 'patrol' && L.state === 'patrol';
    A.apartT = calm && v3.dist(follower.char.pos, L.char.pos) > 8 ? A.apartT + dt : 0;
    if (A.apartT > 5) redo = true;
  }
  if (redo && living >= 1) assignLockdown(g, true);
}

/** Deal the lockdown duties: the two living men nearest each other pair up — the one with more route to cover leads (lobby / break-room loop > corridor > cubicle
 *  farm), the other trails him — and a third holds the corridor junction nearest the last alarm fix (nearest himself if there never was one). `redeal`: after a
 *  death or a separation the (new) leader tells his man to close up — only if the pairing actually changed, so a pair that keeps getting split cannot chatter. */
function assignLockdown(g: Game, redeal: boolean, except: Guard | null = null) {   // (except: a man whose own line must stay up — the finder of a body — takes his role without announcing it)
  let oldLead: Guard | null = null, oldFollow: Guard | null = null, oldPost: Chokepoint | null = null, oldPostMan: Guard | null = null;
  for (const gd of g.guards) { if (gd.state === 'dead') continue; if (gd.leader) { oldFollow = gd; oldLead = gd.leader; } if (gd.post) { oldPost = gd.post; oldPostMan = gd; } }
  clearLockdown(g); g.alarm.apartT = 0;
  const alive = g.guards.filter(x => x.state !== 'dead'); if (alive.length < 2) return;
  let a = alive[0], b = alive[1], bd = Infinity;
  const C = g.clearing;   // a pair in the middle of clearing a room stays the pair (a re-deal around them only moves the post)
  if (C && C.b && alive.includes(C.a) && alive.includes(C.b)) { a = C.a; b = C.b; }
  else for (let i = 0; i < alive.length; i++) for (let j = i + 1; j < alive.length; j++) { const d = v3.dist(alive[i].char.pos, alive[j].char.pos); if (d < bd) { bd = d; a = alive[i]; b = alive[j]; } }
  const lead = C && a === C.a ? a : routeLength(g, a) >= routeLength(g, b) ? a : b, follow = lead === a ? b : a;
  follow.leader = lead; follow.followT = 0; follow.leaderStillT = 0; follow.path = []; follow.pathGoal = null;
  if (redeal && (lead !== oldLead || follow !== oldFollow) && lead !== except && !(lead.bubble && g.time - lead.bubble.t < 0.5)) g.say(lead, 'you\'re with me — close up');   // (not over his own order of a moment ago — with two men left the spokesman IS the new lead; his man closes up regardless)
  const third = alive.find(x => x !== a && x !== b); const cps = g.level.chokepoints ?? [];
  if (third && cps.length) {
    const ref = g.alarm.placed ? g.alarm.pos : third.char.pos; let best = cps[0], cd = Infinity;
    for (const cp of cps) { const d = v3.dist(cp.pos, ref); if (d < cd) { cd = d; best = cp; } }
    third.post = best; third.followT = 0; third.path = []; third.pathGoal = null;
    if ((best !== oldPost || third !== oldPostMan) && third !== except) g.say(third, `holding the ${best.name}`, true);   // over the radio, so it reaches the log: where the spare man will be standing
  }
}
function clearLockdown(g: Game) { for (const gd of g.guards) { if (gd.leader || gd.post) { gd.path = []; gd.pathGoal = null; } gd.leader = null; gd.post = null; } }
function routeLength(g: Game, gd: Guard): number { const pts = g.level.routes[gd.routeI]?.points ?? []; let s = 0; for (let i = 0; i < pts.length; i++) s += v3.dist(pts[i], pts[(i + 1) % pts.length]); return s; }

/** Lockdown follower on patrol: keep station ~1.4 m behind-left of the leader — re-planned at most twice a second and only when the station has actually moved —
 *  close up briskly when left behind, and when the leader stops, plant at his shoulder facing the way he faces and sweep the rear-left quarter. True while walking. */
function followLeader(g: Game, gd: Guard, dt: number, walk: number): boolean {
  const L = gd.leader!, c = gd.char, lp = L.char.pos, goal = gd.followGoal, nav = g.col.nav;
  gd.followT -= dt; gd.leaderStillT = Math.hypot(L.char.vel[0], L.char.vel[2]) < 0.15 ? gd.leaderStillT + dt : 0;
  if (gd.followT <= 0) {
    gd.followT = 0.5;
    const f = L.char.forward(), lv = L.char.vel;
    goal[0] = lp[0] - f[0] * 1.3 + f[2] * 0.45; goal[1] = 0; goal[2] = lp[2] - f[2] * 1.3 - f[0] * 0.45;                              // behind-left, ~1.4 m off (his left = [f.z, 0, −f.x] in this handedness; only half a step wide, or he clips the leaves standing ajar off the corridor's north wall)
    if (nav.isBlocked(goal[0], goal[2]) || !nav.walkable(lp, goal)) { goal[0] = lp[0] - f[0] * 1.2; goal[2] = lp[2] - f[2] * 1.2; }   // a partition there: straight behind him
    if (nav.isBlocked(goal[0], goal[2]) || !nav.walkable(lp, goal)) { goal[0] = lp[0]; goal[2] = lp[2]; }                            // his back to a wall: on him (the separation push keeps them a body apart)
    else if (!nav.isBlocked(goal[0] + lv[0] * 0.6, goal[2] + lv[2] * 0.6)) { goal[0] += lv[0] * 0.6; goal[2] += lv[2] * 0.6; }        // led by the leader's stride: aim at where the station will be when this plan is walked, or he settles a metre adrift, forever arriving
    const off = Math.hypot(goal[0] - c.pos[0], goal[2] - c.pos[2]);
    const stale = !gd.pathGoal || Math.hypot(gd.pathGoal[0] - goal[0], gd.pathGoal[2] - goal[2]) > 0.5 || (gd.pathI >= gd.path.length && off > 0.7);
    if (stale && off > 0.35) goTo(g, gd, goal, true);
  }
  const gap = Math.hypot(lp[0] - c.pos[0], lp[2] - c.pos[2]);
  const arrived = followPath(g, gd, dt, gap > 4.5 ? Math.max(walk, g.tune.guardInvestigate * 1.1) : gap > 2.0 ? walk * 1.3 : walk);
  if (arrived) {   // planted at his shoulder facing the way he faces, sweeping the rear-left quarter — unless the man has not moved for a while and is right there: then he turns and looks at him (which is how a leader taken down quietly under his nose gets found: bodyInView wants the body in his cone)
    const lookAt = gd.leaderStillT > 6 && gap < 2.2;
    c.bodyYaw = approachAngle(c.bodyYaw, lookAt ? Math.atan2(lp[0] - c.pos[0], lp[2] - c.pos[2]) : L.char.bodyYaw + 0.45, 3 * dt);
    gd.sweep = damp(gd.sweep, Math.sin(gd.lookPhase * 0.6) * (lookAt ? 0.25 : 0.6), 2, dt); c.anim.lookYawExtra = Math.sin(gd.lookPhase * 1.1) * (lookAt ? 0.15 : 0.4);
  }
  else gd.sweep = damp(gd.sweep, Math.sin(gd.lookPhase * 0.7) * 0.3, 2, dt);
  return !arrived;
}

/** Lockdown post on patrol: walk to the junction (one plan, to the exact spot; re-planned only if he gets shoved half a metre off it, at most twice a second), then stand on it
 *  squared along the corridor, muzzle and eyes sweeping it slowly. True while walking. */
function holdPost(g: Game, gd: Guard, dt: number, walk: number): boolean {
  const P = gd.post!, c = gd.char;
  gd.followT -= dt;
  if (gd.followT <= 0 && Math.hypot(P.pos[0] - c.pos[0], P.pos[2] - c.pos[2]) > 0.45 && (!gd.pathGoal || gd.pathI >= gd.path.length || Math.hypot(gd.pathGoal[0] - P.pos[0], gd.pathGoal[2] - P.pos[2]) > 0.5)) {
    gd.followT = 0.5; goTo(g, gd, P.pos, true);
    const last = gd.path[gd.path.length - 1]; if (!last || Math.hypot(last[0] - P.pos[0], last[2] - P.pos[2]) > 0.02) gd.path.push(v3.copy(P.pos));   // the spot itself, not its nav cell's centre
  }
  const arrived = followPath(g, gd, dt, walk);
  if (arrived) { c.bodyYaw = approachAngle(c.bodyYaw, P.yaw, 2.5 * dt); gd.sweep = damp(gd.sweep, Math.sin(gd.lookPhase * 0.45) * 0.75, 2, dt); c.anim.lookYawExtra = Math.sin(gd.lookPhase * 0.9) * 0.45; }
  else gd.sweep = damp(gd.sweep, Math.sin(gd.lookPhase * 0.7) * 0.3, 2, dt);
  return !arrived;
}

/** Lockdown housekeeping on a calm pass, one door a frame: a leaf standing open (or left a hand ajar, unlatched) within a couple of metres gets pulled to — only one
 *  beside or behind him (never the one he is about to walk through), not one anybody is handling right now, only from where the closing leaf cannot sweep him,
 *  and not while a colleague is at the doorway (last man through shuts it) — and a keyed door somebody left unlocked gets its bolt thrown again as he passes (a
 *  kicked-in keep cannot be relocked: that he remarks on instead, the notice in updateGuard). Exterior doors are left alone; any one door at most every 12 s. */
function lockdownDoors(g: Game, gd: Guard) {
  const c = gd.char, f = c.forward();
  for (const d of g.doors.list) {
    if (d.def.exterior || g.time - d.guardT < 12 || d.quietT < 0.6) continue;   // (quietT: somebody's hands or shoulder are on that leaf right now — his own included, pushing through it)
    const open = !d.latched && Math.abs(d.angle) > 0.03, relock = !!d.def.locked && !d.locked && !d.lockBroken;
    if (!open && !relock) continue;
    const fc = d.frameCentre; const dx = fc[0] - c.pos[0], dz = fc[2] - c.pos[2]; const dd = Math.hypot(dx, dz);
    if (dd > 2.0 || dd < 0.9 || (dx * f[0] + dz * f[2]) / dd > 0.55) continue;   // out of reach, standing in the doorway (that is a man passing through, not tidying), or ahead of him
    if (open) {   // the closing leaf sweeps the sector between its angle and the frame (angles about the hinge from the closed direction, CCW +): not from in there
      const px = c.pos[0] - d.def.hinge[0], pz = c.pos[2] - d.def.hinge[1]; const cd = d.def.closedDir;
      const th = Math.atan2(-Math.sin(cd) * px + Math.cos(cd) * pz, Math.cos(cd) * px + Math.sin(cd) * pz);
      if (th * d.angle > 0 && Math.abs(th) < Math.abs(d.angle) + 0.4 && Math.hypot(px, pz) < d.def.width + 0.45) continue;
    }
    let busy = false; for (const o of g.guards) if (o !== gd && o.state !== 'dead' && Math.hypot(o.char.pos[0] - fc[0], o.char.pos[2] - fc[2]) < 2.0) { busy = true; break; }
    if (busy) continue;
    d.guardT = g.time;
    if (open) g.doors.pullTo(d, c.id);   // a stiff sprung close the latch catches (hold-open doors too, and one left a crack open) — the corridor's light pinches off the room's floor as it goes
    if (relock) { g.say(gd, d.noticed ? 'locking this back up' : open ? 'shutting this — and it stays locked' : 'someone left this unlocked — locking it'); d.locked = true; d.noticed = true; d.noticedT = g.time; g.audio.play('lockOpen', d.pos, 0.6, { rate: 0.85 }); }   // (noticed already: the pair read that lock as picked and said so — squad.ts readDoors; either way it now counts as read, and planClears ranks its room first for a while)
    break;
  }
}

/** 0..1 tension for the score: patrol 0, suspicious ~0.5, search ~0.65, alert 1 — and an alarmed floor never quite settles (heightened 0.12, lockdown 0.22); a pair
 *  methodically clearing rooms (scripted, nominally 'search') sits at 0.3: purposeful, not alarmed. */
export function tension(g: Game): number {
  let x = Math.max(g.blackout.active ? 0.18 : 0, escalationOf(g) >= 2 ? 0.22 : escalationOf(g) >= 1 ? 0.12 : 0);
  for (const gd of g.guards) {
    if (gd.state === 'dead') continue;
    const s = gd.script && g.clearing && (gd === g.clearing.a || gd === g.clearing.b) ? 0.3 : gd.state === 'alert' ? 1 : gd.state === 'search' ? 0.68 : gd.state === 'suspicious' ? 0.45 + gd.awareness * 0.2 : gd.awareness * 0.35;
    x = Math.max(x, s);
  }
  return x;
}
