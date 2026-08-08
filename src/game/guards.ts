// Guard AI: hearing (the same propagation the mixer uses), vision (the player's light-meter irradiance, marched through smoke), patrol / suspicious / alert / search state machine, path following, torch handling, and their barks.
// (Split out of game.ts mechanically: plain functions taking the Game as their first argument; Game keeps one-line delegators for
//  anything other files call.)
import { Vec3, v3, clamp, wrapAngle, approachAngle, damp, lerp, smoothstep, DEG, quat } from '../math/vec';
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
import { Box, BoxFlag, makeBox, rayBox } from '../scene/boxes';
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
import { fireWeapon, killGuard, dropFromHand, hitPlayer, handProps, laserBoxes, updateItems, smokeParams, startCanister, detonateStun, armThrough } from './combat';
import { dropThrowable } from './combat';
import type { Game, Player, DownKind, DownBy, Temperament } from './game';
import { Guard, isBreathing, TEMPERAMENT } from './game';
import { runGuardScript, Clearing, inBounds } from './squad';
import { roomName } from './dialogue';

/** A/B switches for the AI passes (the QA probes flip them to print before / after off the same build; play never touches them): motion acuity in the sight
 *  gain, the search that deals hide spots and reads the renderer's light (off: the old sweep-and-wander), threshold discipline at doorways (off: straight through). */
export const AI_TUNE = { motionAcuity: true, darkSearch: true, pieing: true };

// ------------------------------------------------------------ guards
export function hearingCheck(g: Game, gd: Guard) {
  const keen = escalationOf(g) >= 1 ? KEEN : 1;   // an alarmed floor listens harder: the small noises (a creak, a scrape, a footfall) count for a quarter more
  const H = gd.witness.heard;   // what reached him and counted goes in his memory too (Witness): the same gates as the awareness it raises, nothing more
  const small = (where: Vec3) => { H.small++; H.smallWhere = v3.copy(where); H.lastT = g.time; };
  for (const e of g.events) {
    if (e.id === undefined) e.id = ++g.eventSeq;          // sequence ids: events raised by guards later in the update order are still heard next frame
    if (e.id <= gd.heardUpTo) continue;
    gd.heardUpTo = Math.max(gd.heardUpTo, e.id);
    const dStraight = v3.dist(e.pos, gd.char.pos);
    if (e.kind === 'lightOut') { if (dStraight < 10 && gd.awareness < 0.4) { gd.awareness = 0.4; gd.lastKnown = v3.copy(e.pos); gd.reactT = 0.8; small(e.pos); } continue; }   // seen, not heard (but remembered with the small things: a light dying near him)
    const gunfire = (e.kind === 'shot' || e.kind === 'guardShot') && (e.level ?? 0) >= 1;
    const kick = e.kind === 'kick';                                         // a door kicked in: one crack as loud as anything short of a shot — the floor, not the car park
    if (dStraight > (e.kind === 'bang' || gunfire ? 80 : kick ? 40 : 22)) continue;   // gunfire and stun charges: the whole floor (and the lot); everything else has a horizon
    // sound reaches the guard the way it reaches the player's ears: straight through air, or around corners along the
    // walkable space (longer, a bit muffled — a lot if a closed door is across the doorway), or barely through the mass
    const prop = g.col.sound.propagate(v3.add(e.pos, [0, 1, 0]), v3.add(gd.char.pos, [0, 1.5, 0]));
    const d = prop.pathLen, carry = prop.carry;                                  // routed & open ≈ 0.72, through a closed door ≈ 0.32, sealed 0.25
    const heardAt: Vec3 = [prop.via[0], 0, prop.via[2]];                       // where to go and look: the source if heard directly, else the doorway / corner it came through
    if (e.kind === 'shot' && !gunfire && d < 7 * carry + 0.5) { gd.awareness = Math.max(gd.awareness, 0.5); gd.lastKnown = v3.copy(e.pos); small(e.pos); }   // a canister's pop / skitter: worth a look nearby
    if (gunfire || e.kind === 'bang' || kick) {
      // a gunshot — the suppressor buys ambiguity about WHERE, not WHETHER — or a stun charge going off, or a door being kicked in, puts every guard on the floor
      // on high alert: weapons out, converging on where it came from (the source if the path is open, else the doorway / corner it spilled through), the
      // furthest ones a beat later. A guard's own shots tell the others where the intruder IS.
      if (e.kind === 'guardShot' && dStraight < 0.1) continue;                 // his own shot
      if (gunfire) { H.shots++; H.lastShotT = g.time; } else if (kick) H.kick++; else H.bang++;   // (a colleague's rounds are gunfire heard too: 'shots' is what reached his ears, whoever fired)
      H.lastT = g.time;
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
      if (a > 0.08) { gd.awareness = Math.min(1, Math.max(gd.awareness, Math.min(e.loud ? 0.75 : 0.45, gd.awareness + a))); if (!gd.lastKnown || gd.state !== 'alert') gd.lastKnown = heardAt; if (gd.state === 'patrol') gd.reactT = 0.6; small(heardAt); }
    }
    if (e.kind === 'step' && d < 9 && !(g.playerInvisible && (e.who ?? PLAYER_ID) === PLAYER_ID)) {
      const a = (e.level ?? 0.3) * clamp(1 - d / 9, 0, 1) * carry * keen;
      if (a > 0.12) { gd.awareness = Math.min(1, Math.max(gd.awareness, Math.min(0.6, gd.awareness + a * 0.5))); if (!gd.lastKnown || gd.state === 'patrol') gd.lastKnown = heardAt; small(heardAt); }
    }
  }
}

// ------------------------------------------------------------ motion acuity: the eye catches movement long before it resolves a still shape in the dark
/** what a dead-still man is worth against one walking across the view (× the sight gain) IN THE DARK, and the apparent speed (m/s) by which the full rate is
 *  reached; the still-man discount fades as the light on him comes up (by MOTION_LIT_HI he is as plain standing as moving: a lit shape needs no movement to be
 *  read — it is the half-seen one in the shadows that the eye only catches when it moves) */
const MOTION_STILL = 0.35, MOTION_LO = 0.05, MOTION_HI = 1.2, MOTION_LIT_LO = 0.3, MOTION_LIT_HI = 0.75;
/** how much of the motion straight toward / away from the eye counts beside the motion across it (looming reads far weaker than something crossing) */
const MOTION_RADIAL = 0.4;
/** a sprinting man (over SPRINT_MS by the ground he covers) is that much more again; the floors: at arm's length a still man is still plain, and a man already
 *  hunting is LOOKING for a shape, not waiting for it to move */
const MOTION_SPRINT = 1.6, MOTION_SPRINT_MS = 3.5, MOTION_NEAR_M = 2.5, MOTION_NEAR_FLOOR = 0.8, MOTION_HUNT_FLOOR = 0.7;

/** Sam's velocity over the ground this frame, as anybody watching sees it: the displacement of his root since the frame before (not the pace he is asking of his
 *  legs — held against a desk or a leaf he 'sprints' on the spot; a teleport reads as one capped spike). Worked out once a frame, whoever asks first. */
const groundTrack = new WeakMap<Game, { pos: Vec3; t: number; vel: Vec3 }>();
export function playerGroundVel(g: Game): Vec3 {
  const p = g.player.char.pos; let P = groundTrack.get(g);
  if (!P) { P = { pos: v3.copy(p), t: g.time, vel: [0, 0, 0] }; groundTrack.set(g, P); return P.vel; }
  if (g.time !== P.t) {
    const since = g.time - P.t;
    if (since > 1e-4 && since < 0.25) { const vx = (p[0] - P.pos[0]) / since, vz = (p[2] - P.pos[2]) / since; const sp = Math.hypot(vx, vz); const k = sp > 6 ? 6 / sp : 1; P.vel = [vx * k, 0, vz * k]; } else P.vel = [0, 0, 0];
    P.pos = v3.copy(p); P.t = g.time;
  }
  return P.vel;
}
/** × the sight gain for how Sam is moving as THIS man sees it: the ground he covers split into the part across the man's line of sight (what the eye jumps to) and
 *  the part along it (counted at MOTION_RADIAL), eased from the still-man's worth (MOTION_STILL in the dark, rising to 1 as the light on him does) up to 1 at a
 *  walk across the view, × MOTION_SPRINT at a sprint; floored near to and for a man already hunting. */
export function motionFactor(g: Game, gd: Guard, dist: number): number {
  const v = playerGroundVel(g); const gp = gd.char.pos, pp = g.player.char.pos;
  let rx = gp[0] - pp[0], rz = gp[2] - pp[2]; const rl = Math.hypot(rx, rz) || 1; rx /= rl; rz /= rl;   // unit, Sam → the eye (flat)
  const vr = Math.abs(v[0] * rx + v[2] * rz), vt = Math.abs(v[0] * rz - v[2] * rx); const sp = Math.hypot(v[0], v[2]);
  const still = lerp(MOTION_STILL, 1, smoothstep(MOTION_LIT_LO, MOTION_LIT_HI, g.player.visibility));
  let m = lerp(still, 1, smoothstep(MOTION_LO, MOTION_HI, vt + MOTION_RADIAL * vr)); if (sp > MOTION_SPRINT_MS) m *= MOTION_SPRINT;
  if (dist < MOTION_NEAR_M) m = Math.max(m, MOTION_NEAR_FLOOR);
  if (gd.state === 'alert' || gd.state === 'search') m = Math.max(m, MOTION_HUNT_FLOOR);
  return m;
}

/** Awareness per second a VISIBLE player builds in this man at `dist` (canSee said yes): the light on the player, distance, crouch, how he is MOVING across this
 *  man's view (motionFactor: a still shape in the half-dark fills the meter at a third of the rate of one walking across it, a sprint faster again), the smoke
 *  between them, the alarmed floor's keenness — point-blank is certain — and a man already hunting is twice as quick on the uptake. One formula for updateGuard
 *  and for the scripted men's senses (squad.ts perceive). Above 0.05 it moves his fix; above 0.25 it is a real sighting (the licence to shoot). */
export function sightGain(g: Game, gd: Guard, dist: number): number {
  const T = g.tune; const light = g.player.visibility; const distF = clamp(1 - (dist - 3) / 13, 0.2, 1);
  const moving = AI_TUNE.motionAcuity ? motionFactor(g, gd, dist) : (Math.hypot(g.player.char.vel[0], g.player.char.vel[2]) > 2 ? 1.5 : 1);   // (the switch is the QA table's before / after; play runs the acuity)
  let rate = T.detectRate * Math.pow(light, 1.5) * distF * (g.player.crouch ? 0.75 : 1) * moving * clamp((gd.smokeTrans - 0.3) / 0.5, 0, 1) * (escalationOf(g) >= 1 ? KEEN : 1);   // (an alarmed floor is actually looking: a quarter quicker on the uptake)
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

/** A world point in this man's open view: within `range` of his eyes, inside `halfDeg` of where his chest points (flat), nothing solid between — bodyInView's test
 *  against any point. Lighting is deliberately not asked (the things it is used for carry their own light: a colleague with a torch going down). */
export function pointInView(g: Game, gd: Guard, p: Vec3, range = 12, halfDeg = 60): boolean {
  const eye = gd.char.bones.head ?? v3.add(gd.char.pos, [0, 1.65, 0]); const look = gd.char.chestDir;
  const to = v3.sub(p, eye); const L = v3.len(to); if (L > range) return false; if (Math.hypot(to[0], to[2]) < 0.3) return true;   // (on top of it: his)
  if (v3.dot([look[0], 0, look[2]], v3.normalize([to[0], 0, to[2]])) < Math.cos(halfDeg * DEG)) return false;
  return !g.col.segmentBlocked(eye, p);
}

// ------------------------------------------------------------ witness: what each man has personally perceived (game.ts Witness) — written here, read by the dialogue to come
/** A man is going down at Sam's hand (combat.ts killGuard, before the victim's state flips): every OTHER living, undazzled man who has the victim in his open view
 *  as he drops — or his eyes on Sam himself this instant (the light-gated sighting the AI already made) — remembers seeing it done, and to whom. A colleague's
 *  round or the world dropping a man is nobody's 'I saw you do it'; a Sam the sandbox has made invisible did nothing anyone can name; with the AI off nobody is home. */
export function witnessKill(g: Game, victim: Guard, how: DownKind, by: DownBy) {
  if (!g.aiEnabled || by !== 'player') return;
  const at = victim.char.bones.chest ?? v3.add(victim.char.pos, [0, 1.1, 0]);
  for (const o of g.guards) {
    if (o === victim || o.state === 'dead' || g.time < o.dazzledUntil) continue;
    if ((!g.playerInvisible && pointInView(g, o, at)) || o.sawPlayerThisFrame) o.witness.sawAct = { kind: how, victim: victim.callsign, t: g.time };
  }
}
/** What took a man over the alert line just now (updateGuard's transition): eyes on Sam (a real sighting this frame or a beat ago, or glimpses that added up
 *  within the second), a sound that placed him this very frame (only gunfire, a stun charge or a door going in jump that high), a body he is standing over,
 *  else word over the net (a colleague's contact breaking him off a drill, an order). */
function alertCause(g: Game, gd: Guard, seen: boolean): 'sight' | 'sound' | 'radio' | 'body' {
  if (seen || g.time - gd.lastSeenT < 0.5) return 'sight';
  if (gd.witness.heard.lastT >= g.time - 1e-6) return 'sound';
  if (g.time - gd.sightT < 1.0) return 'sight';
  if (gd.bodyDuty || g.time - gd.sawBodyT < 2) return 'body';
  return 'radio';
}
/** One compact line of a man's witness record for the panel / the QA probes ('' = nothing yet): peak awareness, what made him alert, what he heard, what he saw. */
export function witnessSummary(g: Game, gd: Guard): string {
  const W = gd.witness, H = W.heard; const parts: string[] = []; const ago = (t: number) => `${Math.max(0, Math.round(g.time - t))} s ago`;
  if (W.peakAwareness >= 0.05) parts.push(`peak ${W.peakAwareness.toFixed(2)}`);
  if (W.alertedBy) parts.push(`alert by ${W.alertedBy} ${ago(W.alertT)}`);
  const heard = [H.shots ? `${H.shots} shot${H.shots > 1 ? 's' : ''}` : '', H.kick ? `${H.kick} kick${H.kick > 1 ? 's' : ''}` : '', H.bang ? `${H.bang} bang${H.bang > 1 ? 's' : ''}` : '', H.small ? `${H.small} small${H.smallWhere ? ` (last @${H.smallWhere[0].toFixed(0)},${H.smallWhere[2].toFixed(0)})` : ''}` : ''].filter(Boolean);
  if (heard.length) parts.push(`heard ${heard.join(', ')}`);
  if (W.sawAct) parts.push(`SAW Sam ${W.sawAct.kind === 'shot' ? 'shoot' : W.sawAct.kind === 'struck' ? 'drop' : W.sawAct.kind === 'choked' ? 'choke out' : 'grab'} ${W.sawAct.victim} ${ago(W.sawAct.t)}`);
  if (W.sawBody) parts.push(`found ${W.sawBody.victim} ${W.sawBody.breathing ? 'out cold' : 'dead'} ${ago(W.sawBody.t)}`);
  if (W.calledToBody) parts.push(`heard ${W.calledToBody.victim} called in ${W.calledToBody.breathing ? 'out cold' : 'dead'}`);
  if (W.sawHeld) parts.push(`saw ${W.sawHeld.who} held`);
  if (W.wasHeld) parts.push(`held ×${W.wasHeld}`);
  if (W.dazzled) parts.push(`dazzled ×${W.dazzled}`);
  return parts.join(' · ');
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
/** The alert chase is over without him: down to 'search' from where he stands — awareness under the alert threshold (re-alerting takes a fresh sighting or sound,
 *  or alert ↔ search flapped every reactT), every chase clock zeroed for the next one, and the line if there is one. */
function dropToSearch(g: Game, gd: Guard, bark: string | null) {
  gd.state = 'search'; gd.searchT = 0; gd.awareness = Math.min(gd.awareness, 0.8); gd.searchPlan = null;
  resetChase(gd); gd.path = []; gd.pathGoal = null;   // (the chase's path is not the search's: he sweeps from where he stands, then deals himself the hiding places round the fix — searchPlanFor)
  if (bark) g.say(gd, bark);
}
/** Every clock the alert chase keeps, back to a fresh chase: how near he has got to the fix and how long without getting nearer (and which fix that was), how long
 *  without covering ground, how long without eyes on him, how long his line of fire has been blocked. Called entering 'alert', on every fresh sight of Sam,
 *  leaving for 'search' — and by the QA probes staging an alert man (tools/qa/probes.ts armAlert). */
export function resetChase(gd: Guard) { gd.alertBest = 1e9; gd.alertStallT = 0; gd.alertRef = null; gd.chaseT = 0; gd.shotBlockedT = 0; gd.lineInT = 0; gd.stallRef = null; gd.stallT = 0; }
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
          if (D.role !== 'finder' && !B.bodyRemarked && !(gd.bubble?.radio && g.time - gd.bubble.t < 5)) { B.bodyRemarked = true; g.say(gd, isBreathing(B) ? "…he's out cold. someone did this by hand. — stay sharp" : '…christ. — stay sharp'); }   // (not on top of his own radio call — with two men left, the one beside the finder is also the one who gave the order)
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

/** The fix a man is LEFT with the frame Sam goes out of his sight: where he last had him (`from`), led along the way he was moving then (`vel`) — 0.9 s of it,
 *  and only as far as the floor is open that way (a slim body marched through the statics: walls and jambs stop it; a LATCHED leaf stops it too — one Sam is
 *  only pushing at, or one locked against him that he will never be through — but a leaf he has off the latch and on the swing does not: that one he is going
 *  through). Standing still when he vanished, that is the spot itself; moving, it runs ahead with his pace — a stride at a creep, four at a sprint. So when Sam
 *  runs out of a man's sight through a doorway (the leaf swinging to behind him) or round a corner, the fix lies inside where he went, and the chase carries
 *  through the door — he has a key — and searches that room, instead of ending on the threshold with 'where did he go?' and a sweep of the corridor he is
 *  standing in while Sam stands lit in the middle of the room beyond. (The alarm's fix follows it in too: the pair sent to clear rooms starts with that one.)
 *  While Sam is IN view nothing is led — eyes, muzzle, torch and feet go to the man himself (updateGuard keeps his true position as the fix and only remembers
 *  the velocity for this). The point handed back always sits in a free nav cell on the near side of whatever stopped the march, so the path to it can never
 *  be snapped to the far side of a partition. */
export function leadFix(g: Game, from: Vec3, vel: Vec3): Vec3 {
  const p: Vec3 = [from[0], 0, from[2]]; const vx = vel[0], vz = vel[2]; const sp = Math.hypot(vx, vz);
  if (sp < 0.5) return p;   // (below a creep: standing, turning on the spot, the last of a stop's slide)
  const nav = g.col.nav; const shut = g.doors.list.filter(d => d.latched).map(d => d.box); const rd: Vec3 = [vx / sp, 0, vz / sp];
  let fix = p; const q: Vec3 = [0, 0, 0]; let prev: Vec3 = [p[0], 1.0, p[2]];
  for (let s = 0.05; s < 0.93; s += 0.05) {   // ≤ 0.24 m a sample at a sprint: overlapping 0.15 m circles cannot step over a 12 cm partition
    q[0] = p[0] + vx * s; q[1] = 0; q[2] = p[2] + vz * s;
    const at: Vec3 = [q[0], 1.0, q[2]];
    if (g.col.collideCircle(q, 0.15, 0.25, 1.5, 1) || g.col.clampToWorld(q, 0.15) || shut.some(b => rayBox(prev, rd, b, sp * 0.05))) break;   // (the first two nudge q — it is scratch, `at` holds the sample)
    prev = at;
    if (!nav.isBlocked(at[0], at[2])) fix = [at[0], 0, at[2]];   // (a sample in a cell the bake calls blocked — the last hand's breadth before a wall, the skirt of a jamb — is marched through but never handed back)
  }
  return fix;
}

/** Nothing solid — wall, jamb, mullion, desk, a door leaf — between this man's gun shoulder and `to` (less the body's breadth at the far end), so a pistol
 *  hand round the jamb reads the same whichever way the muzzle happens to point this frame. Loose furniture is concealment, not cover (fireWeapon lets the
 *  round through to whoever stands behind it), so it is not asked. This is the line of FIRE; canSee is the line of sight, from the eye, on the same segment
 *  test — at a doorway the two differ by exactly the hand's breadth that had him shooting the frame. */
export function clearShot(g: Game, c: Character, to: Vec3): boolean {
  const from = c.bones.upperArmR ?? v3.add(c.pos, [0, 1.4, 0]);   // the gun shoulder: fixed to the body, so the answer does not swing about with a muzzle still coming round onto the target (once it is on, muzzle → target is a piece of this same segment)
  const d = v3.sub(to, from); const L = v3.len(d); if (L < 0.35) return true;   // (pressed against him: nothing can be in between)
  return !g.col.segmentBlocked(from, v3.mad(from, d, (L - 0.3) / L));         // 0.3: the body's own breadth round the aim point — anything nearer than that is him, not cover
}

// ------------------------------------------------------------ muzzle discipline: nobody points a pistol at a colleague — it goes to the low ready until the line is clear
/** metres off a living colleague's body axis inside which a line of aim counts as ON him (widened for a man on the move by MUZZLE_LEAD_SECS of his pace, capped:
 *  the gun comes down for a man about to walk into the line, not as he meets it); how far past the aim point the line still counts (a man standing just beyond
 *  the spot being covered is being pointed at too); the reach of 'straight ahead' when there is no point to aim at; and the beat a cleared line stays down before
 *  the gun comes back up (so a man crossing does not flick it down-up-down) */
const MUZZLE_CLEAR_M = 0.35, MUZZLE_LEAD_SECS = 0.3, MUZZLE_LEAD_MAX_M = 0.4, MUZZLE_PAST_M = 1.0, MUZZLE_REACH_M = 8, MUZZLE_UP_SECS = 0.2;
/** in a fight: the margin a side-step opens the line by beyond the bare clearance, the furthest he will step for it, inside which distance of Sam he will not
 *  push past a colleague to get his line, and the least often anyone says anything about it */
const LINE_STEP_MARGIN_M = 0.12, LINE_STEP_MAX_M = 2.2, LINE_PUSH_MIN_M = 2.5, LINE_BARK_SECS = 10;

/** where a colleague stands against the aim segment `from` → `to`: t = how far along it his axis projects (0 at the shoulder, 1 at the point), e = his signed
 *  offset from it (metres, along the flat unit square off the line (−dz, dx)/L — the same side basis lineStep steps along), d = his distance from it in 3D (the
 *  axis runs soles to crown about his pelvis, so a line over a kneeling man or under one on a desk measures clear), clear = the clearance he is owed
 *  (MUZZLE_CLEAR_M, more for a man on the move) */
function lineGeom(from: Vec3, to: Vec3, o: Guard): { t: number; e: number; d: number; clear: number } | null {
  const dx = to[0] - from[0], dy = to[1] - from[1], dz = to[2] - from[2]; const L2 = dx * dx + dz * dz; if (L2 < 1e-4) return null;
  const h = o.char.bones.hips ?? o.char.pos; const ax = h[0], az = h[2];
  const t = ((ax - from[0]) * dx + (az - from[2]) * dz) / L2;
  const px = from[0] + dx * t, pz = from[2] + dz * t, py = from[1] + dy * t;
  const L = Math.sqrt(L2); const e = ((ax - from[0]) * -dz + (az - from[2]) * dx) / L;
  const y0 = o.char.pos[1] + 0.05, y1 = o.char.bones.headTop?.[1] ?? o.char.pos[1] + 1.75; const ddy = py < y0 ? y0 - py : py > y1 ? py - y1 : 0;
  const clear = MUZZLE_CLEAR_M + Math.min(MUZZLE_LEAD_MAX_M, MUZZLE_LEAD_SECS * Math.hypot(o.char.vel[0], o.char.vel[2]));
  return { t, e, d: Math.hypot(px - ax, pz - az, ddy), clear };
}
/** A living colleague standing in this man's line of aim: the segment from his gun shoulder to `to` (and MUZZLE_PAST_M beyond it) passing within the man's
 *  clearance of his body axis. The nearest such man along the line, or null. The INTENDED line (shoulder → the point he means to cover), not this frame's
 *  muzzle: a gun still swinging round onto its mark, or already lowered, answers the same. `except`: one man the caller has already answered for by better
 *  geometry than this — the colleague in Sam's arm, when the shooter stands behind the pair and the round stops in Sam a body's breadth short of him (the
 *  standoff's flankShot): the 'just beyond the point' rule assumes nothing solid AT the point, and there Sam is. */
export function colleagueInLine(g: Game, gd: Guard, to: Vec3, except: Guard | null = null): Guard | null {
  const c = gd.char; const from = c.bones.upperArmR ?? v3.add(c.pos, [0, 1.4, 0]);
  const L = Math.hypot(to[0] - from[0], to[2] - from[2]); if (L < 0.01) return null; const tMax = 1 + MUZZLE_PAST_M / L;
  let best: Guard | null = null, bt = Infinity;
  for (const o of g.guards) {
    if (o === gd || o === except || o.state === 'dead' || !o.char.alive) continue;
    const G = lineGeom(from, to, o); if (!G || G.t < 0 || G.t > tMax || G.t >= bt || G.d > G.clear) continue;   // behind his shoulder, further off than the point (and its margin), or simply clear of it
    bt = G.t; best = o;
  }
  return best;
}
/** The last look before the round goes: a living colleague within a body's breadth of the segment the ROUND will fly — this frame's muzzle to the aim point. The
 *  shoulder line above decides the carry (and must, or a lowered gun would clear its own line and come straight back up); but pressed in a scrum a metre off
 *  Sam the bore sits a hand to the side of the shoulder line, and a man just clear of the one can be grazed by the other. This only ever holds a round for a
 *  frame (fireCd stays spent: it goes the moment he is clear) — it never moves the gun. */
export function roundPastColleague(g: Game, gd: Guard, to: Vec3, except: Guard | null = null): boolean {
  const from = gd.char.muzzle; const dx = to[0] - from[0], dy = to[1] - from[1], dz = to[2] - from[2]; const L2 = dx * dx + dz * dz; if (L2 < 0.0025) return false;
  const tMax = 1 + 0.3 / Math.sqrt(L2);
  for (const o of g.guards) {
    if (o === gd || o === except || o.state === 'dead' || !o.char.alive) continue;
    const h = o.char.bones.hips ?? o.char.pos;
    const t = clamp(((h[0] - from[0]) * dx + (h[2] - from[2]) * dz) / L2, 0, tMax);   // (clamped at the muzzle end: a man whose shoulder the gun is practically resting on is too close to it, whichever side of the muzzle his spine falls)
    const px = from[0] + dx * t, py = from[1] + dy * t, pz = from[2] + dz * t;
    const y0 = o.char.pos[1] + 0.05, y1 = o.char.bones.headTop?.[1] ?? o.char.pos[1] + 1.75; const ddy = py < y0 ? y0 - py : py > y1 ? py - y1 : 0;
    const clear = Math.max(0.32, MUZZLE_CLEAR_M + Math.min(MUZZLE_LEAD_MAX_M, MUZZLE_LEAD_SECS * Math.hypot(o.char.vel[0], o.char.vel[2])));
    if (Math.hypot(px - h[0], pz - h[2], ddy) < clear) return true;
  }
  return false;
}
/** the point MUZZLE_REACH_M out along where his gun is being held (aim yaw / pitch) from the gun shoulder — or where that line meets the floor, if it does first (a
 *  low carry pools a few metres out: the line ends there, it does not run on under the men beyond): what 'ahead' means for a man covering nothing in particular */
export function aimAhead(c: Character): Vec3 {
  const from = c.bones.upperArmR ?? v3.add(c.pos, [0, 1.4, 0]); const cp = Math.cos(c.aimPitch), sy = Math.sin(c.aimPitch);
  let reach = MUZZLE_REACH_M; if (sy < -1e-3) reach = Math.min(reach, (from[1] - 0.02) / -sy);
  return [from[0] + Math.sin(c.aimYaw) * cp * reach, from[1] + sy * reach, from[2] + Math.cos(c.aimYaw) * cp * reach];
}
/** This frame's muzzle discipline for a man whose pistol is up (`to` = the point he means to cover, null = the pistol is not up at all): a colleague in the line
 *  puts it straight down (Guard.muzzleDown), a line clear again for MUZZLE_UP_SECS lets it back up. Returns the man in the line (null when clear or lowered on
 *  the hysteresis alone). Whoever poses him reads muzzleDown: upper 'none' + stance 'lowReady' instead of the aim (updateGuard, squad.ts runGuardScript).
 *  `except` as colleagueInLine has it (the standoff's shot past the pair, from behind). */
export function muzzleCheck(g: Game, gd: Guard, to: Vec3 | null, dt: number, except: Guard | null = null): Guard | null {
  gd.muzzleEvalT = g.time;
  const o = to ? colleagueInLine(g, gd, to, except) : null;
  if (o) { gd.muzzleDown = true; gd.muzzleClearT = 0; }
  else if (gd.muzzleDown && (!to || (gd.muzzleClearT += dt) > MUZZLE_UP_SECS)) { gd.muzzleDown = false; gd.muzzleClearT = 0; }
  return o;
}
/** In a fight, a colleague `o` in his line to `aimAt` and nothing else keeping the round in: the step to the side that opens the line — worked out, not groped
 *  for: moving himself sideways by s moves the line at the man by s·(1 − t), so the two offsets that put the man his clearance (and a margin) off it are known;
 *  the shorter first, the other if that one is in a wall or off the walkable floor — taken as a side-step that keeps his chest and gun toward Sam. Returns 'step'
 *  while he is taking one (this moves him), 'none' when no step from here opens it (a corridor too narrow for the angle, the man hard by the target: the caller
 *  pushes past him instead, or waits), 'clear' when there is nobody to step round (lowered on the hysteresis alone — the gun is already on its way up). One of
 *  the two says so, at most every LINE_BARK_SECS for either. */
function lineStep(g: Game, gd: Guard, aimAt: Vec3, o: Guard | null, dt: number): 'step' | 'none' | 'clear' {
  const c = gd.char; const nav = g.col.nav;
  if (o && g.time - gd.lineBarkT > LINE_BARK_SECS && g.time - o.lineBarkT > LINE_BARK_SECS) {
    gd.lineBarkT = o.lineBarkT = g.time;
    if (Math.hypot(o.char.vel[0], o.char.vel[2]) > 0.4) g.say(o, 'crossing!'); else g.say(gd, "move — you're in my line");
  }
  // a step already under way (a one-point path of his own): ease onto the spot exactly, feet cycling, facing kept — unless it is going nowhere (somebody on it)
  if (gd.path.length === 1 && gd.pathI === 0 && gd.pathGoal && v3.distXZ(gd.pathGoal, gd.path[0]) < 1e-3) {
    const wp = gd.path[0]; const dx = wp[0] - c.pos[0], dz = wp[2] - c.pos[2]; const d = Math.hypot(dx, dz);
    if (d < 0.05) { gd.path = []; gd.pathGoal = null; gd.stallRef = null; }
    else if (notGettingAnywhere(gd, dt, 1.2)) { gd.path = []; gd.pathGoal = null; gd.stallRef = null; gd.stuckN++; }
    else { gd.speed = damp(gd.speed, g.tune.guardInvestigate * clamp(d / 0.45, 0.35, 1), 6, dt); const step = Math.min(d, gd.speed * dt); c.pos[0] += dx / d * step; c.pos[2] += dz / d * step; c.vel = [dx / d * gd.speed, 0, dz / d * gd.speed]; return 'step'; }
  }
  if (!o) return 'clear';
  const from = c.bones.upperArmR ?? v3.add(c.pos, [0, 1.4, 0]); const G = lineGeom(from, aimAt, o); if (!G || G.t > 0.97) return 'none';   // (the man is AT the target: no angle opens that)
  const dx = aimAt[0] - c.pos[0], dz = aimAt[2] - c.pos[2]; const L = Math.hypot(dx, dz); if (L < 0.3) return 'none';
  const lx = -dz / L, lz = dx / L;   // unit, square off the line (the side lineGeom's `e` is signed by)
  const need = G.clear + LINE_STEP_MARGIN_M; const k = 1 / Math.max(0.03, 1 - G.t);
  const cands = [(G.e - need) * k, (G.e + need) * k].filter(s => Math.abs(s) <= LINE_STEP_MAX_M).sort((a, b) => Math.abs(a) - Math.abs(b));
  if (gd.stuckN & 1) cands.reverse();   // (a step that went nowhere last time: try the other way first)
  for (const s of cands) {
    const cand: Vec3 = [c.pos[0] + lx * s, 0, c.pos[2] + lz * s];
    if (nav.isBlocked(cand[0], cand[2]) || !nav.walkable(c.pos, cand) || g.col.segmentBlocked([c.pos[0], 0.5, c.pos[2]], [cand[0], 0.5, cand[2]])) continue;
    const from2: Vec3 = [from[0] + lx * s, from[1], from[2] + lz * s]; let other = false;   // and the line from THERE clear of everybody else — with two colleagues either side of it, the step round one is a step in front of the other, and he see-sawed between the two spots for as long as the scrum held (the QA soak's man on the lot): then it is 'none', and he waits or pushes past
    for (const p of g.guards) { if (p === gd || p === o || p.state === 'dead' || !p.char.alive) continue; const G2 = lineGeom(from2, aimAt, p); if (G2 && G2.t >= 0 && G2.t <= 1 + MUZZLE_PAST_M / L && G2.d <= G2.clear + LINE_STEP_MARGIN_M) { other = true; break; } }
    if (other) continue;
    gd.path = [cand]; gd.pathI = 0; gd.pathGoal = v3.copy(cand); gd.stallRef = null;
    return 'step';
  }
  return 'none';
}

// ------------------------------------------------------------ the human shield: a colleague held in Sam's arm, in view — the standoff (docs/internal/grab-interrogate-design.md §2.2)
// A man who can see the pair — the colleague with a forearm across his throat, or Sam behind him: they stand as one, and the man's own torch is waving over the
// ceiling, so lighting is not asked (bodyInView's precedent) — goes alert on Sam like any sighting, but the alert branch hands him to standoffAlert instead of the
// chase and the shot: keep off them (the ring, MIN‥MAX), pistol up but lowered for the man in the line as muzzle discipline already has it, work round the ring
// toward Sam's BACK — the one place a round stops in Sam a body's breadth short of the colleague (flankShot) — two men going round opposite ways so whichever
// way Sam turns the shield one of them gains it, single aimed rounds at a slow cadence once it is there, and the talking. The first to see it calls it in: the
// floor locks down on the call and everybody else comes to WHERE at the double (they know the place from the net, not that they can see it — arriving, their
// own eyes put them in the standoff). It ends the moment the hold does (choked out in front of them: found there and then, and the rounds follow; let go: the
// plain fight, the freed man in everybody's line for a second; Sam down: the aftermath), or four seconds after he last had them in view (back to the plain chase
// on where they were). Nothing of it under the tour, for the tour's held / pinned men, or for a Sam the sandbox has made invisible.
/** metres: nearer than MIN he backs off to the ring, further than MAX he closes at the investigate jog (inside it, a walk); the ring he circles on; how near dead
 *  astern of the pair counts as round (he comes in or out to the ring on his own bearing from there); the furthest he fires from (the guards' spread at rest keeps
 *  every round inside Sam's body cylinder up to about this — a hand further and a round wide of Sam is a round in the man beyond him) */
const STANDOFF_MIN_M = 3.5, STANDOFF_MAX_M = 7, STANDOFF_ORBIT_M = 4.5, STANDOFF_SETTLE_DEG = 22, STANDOFF_FIRE_MAX_M = 5.0;
/** the fire rule (flankShot): he stands inside REAR_DEG of the pair's rear, AND the segment his round flies — gun shoulder to where it stops in Sam's near side —
 *  passes at least CLEAR_M from the held man's body axis (at 70° off their rear that clearance is 0.45 m to the centimetre with the man 0.27 ahead of Sam: the two
 *  say the same thing, one by angle, one by the bones), AND nothing solid is in the way; then single rounds at least SHOT_SECS apart, aimed AIM_Y up Sam's own axis
 *  (the base of his neck: head and chest both, and the middle of the cylinder a round has to find) */
const STANDOFF_REAR_DEG = 70, STANDOFF_CLEAR_M = 0.45, STANDOFF_SHOT_SECS = 1.2, STANDOFF_AIM_Y = 1.42;
/** Sam's hit cylinder about his pelvis (Character.rayHit): where a true round stops short of his axis */
const SAM_R = 0.28;
/** in view: range and half-angle of the flat cone off where chest and head point between them (canSee's, and its wide near field inside 3.5 m); seconds out of
 *  his view before he gives the standoff up for the plain chase; seconds between orbit re-plans (the pair turns under him); seconds of no headway on a leg before
 *  he drops it (a colleague on the spot, a leaf held against him) and stands a moment */
const STANDOFF_VIEW_M = 16, STANDOFF_VIEW_DEG = 60, STANDOFF_LOSE_SECS = 4, STANDOFF_REPLAN_SECS = 1.0, STANDOFF_STALL_SECS = 2.5, STANDOFF_REST_SECS = 1.5;
/** the talking: at least BARK_SECS between one man's lines, FLOOR_SECS between anybody's */
const STANDOFF_BARK_SECS = 6, STANDOFF_BARK_FLOOR_SECS = 3;

/** Per observer, while it lasts (Guard.standoff). pair: the held colleague; side: which way round the ring he works (+1 = anticlockwise seen from above, his
 *  bearing from the pair increasing) — the second man takes the other; orbitGoal: the ring point he is walking to now (null = planted: he has his line, or is
 *  cornered, or resting a beat after a leg that went nowhere); replanT / restT: clocks for those; seenT: when he last had the pair in view; lastBarkT: his last
 *  line; lineOpen: the flank line stood open last frame (the 'I have him' is said on the edge); shots / orbitM: rounds fired and metres walked in this standoff
 *  (the QA soak reads both); way: the way round he is actually going now (0 = not yet; kept between re-plans so a walled arc's edge cannot see-saw him);
 *  cornered: no ring point either way round last time he looked; backing: inside the ring's edge last frame, backing off. */
export interface Standoff { pair: Guard; since: number; side: -1 | 1; way: -1 | 0 | 1; orbitGoal: Vec3 | null; replanT: number; restT: number; seenT: number; lastBarkT: number; lineOpen: boolean; shots: number; orbitM: number; stalls: number; cornered: boolean; backing: boolean; }
/** QA hook: set `log` and orbitGoalFor says what became of every ring point it looked at (the probes print it for a second or two round a re-plan they are chasing) */
export const STANDOFF_DEBUG: { log: ((s: string) => void) | null } = { log: null };
/** the pair as the observers reason about it this frame: the held man, Sam's hit axis and his, the point between (the ring's centre), the pair's facing (Sam's:
 *  the man is locked on it), the aim point on Sam's axis, the pair's velocity; null when nobody is held (or the shove is already sending him off) */
export interface Pair { key: object; held: Guard; sam: Vec3; heldAxis: Vec3; centre: Vec3; facing: number; aim: Vec3; vel: Vec3; }
export function pairOf(g: Game): Pair | null {
  const H = g.player.holding; if (!H || H.phase === 'release' || g.player.down) return null;
  const held = H.g; if (held.state === 'dead' || !held.char.alive || held.held?.by !== 'player' || !g.guards.includes(held)) return null;
  const pc = g.player.char, hc = held.char; const sh = pc.bones.hips ?? pc.pos, hh = hc.bones.hips ?? hc.pos;
  const sam: Vec3 = [sh[0], 0, sh[2]], heldAxis: Vec3 = [hh[0], 0, hh[2]];
  return { key: H, held, sam, heldAxis, centre: v3.lerp(sam, heldAxis, 0.5), facing: pc.bodyYaw, aim: [sam[0], pc.pos[1] + STANDOFF_AIM_Y, sam[2]], vel: v3.copy(pc.vel) };
}
/** holds already called in on the net (one call per hold, whoever sees it first), keyed by the hold itself; the floor's last standoff line (anybody's); each man's last line */
const holdsCalled = new WeakSet<object>(); const floorBarkT = new WeakMap<Game, number>(); const lastBark = new WeakMap<Guard, string>();
/** The pair in this man's open view: either body's chest or head (they stand as one) inside canSee's cone and range with nothing solid between — no light asked
 *  (the colleague is plain, and lit by his own torch), but smoke thick enough to hide Sam hides them both, magnesium in the eyes hides everything, and a Sam the
 *  sandbox has made invisible is holding nobody anyone can name. */
function pairInView(g: Game, gd: Guard, P: Pair): boolean {
  if (g.playerInvisible || g.time < gd.dazzledUntil || gd.smokeTrans < 0.3) return false;
  const c = gd.char; const eye = c.bones.head ?? v3.add(c.pos, [0, 1.65, 0]);
  const look = v3.lerp(c.chestDir, c.headDir(), 0.5); const ll = Math.hypot(look[0], look[2]) || 1; const lx = look[0] / ll, lz = look[2] / ll;
  const pc = g.player.char, hc = P.held.char;
  for (const p of [hc.bones.chest ?? v3.add(hc.pos, [0, 1.1, 0]), hc.bones.head ?? v3.add(hc.pos, [0, 1.6, 0]), pc.bones.chest ?? v3.add(pc.pos, [0, 1.1, 0]), pc.bones.head ?? v3.add(pc.pos, [0, 1.6, 0])]) {
    const tx = p[0] - eye[0], ty = p[1] - eye[1], tz = p[2] - eye[2]; if (Math.hypot(tx, ty, tz) > STANDOFF_VIEW_M) continue;
    const fl = Math.hypot(tx, tz); if (fl > 0.3 && (tx * lx + tz * lz) / fl < Math.cos((fl < 3.5 ? 100 : STANDOFF_VIEW_DEG) * DEG)) continue;
    if (!g.col.segmentBlocked(eye, p)) return true;
  }
  return false;
}
/** Perception's share (updateGuard, AI on): is a held colleague in his view this frame? If so he is IN the standoff (entered now if he was not: the record, his
 *  memory of it, the call or his first line) and everything the AI reads as 'eyes on Sam right now' is refreshed off the pair — awareness full, the fix and the
 *  sighting on Sam, nothing to lead. Returns whether the pair was in view (updateGuard counts it as seeing Sam). */
export function standoffPerceive(g: Game, gd: Guard): boolean {
  if (g.quietUtility || gd.hold || gd.pinned) return false;   // (the tour's men stay on their marks; upkeep drops a record made before the flag went up)
  const P = pairOf(g); if (!P || P.held === gd) return false;
  if (!pairInView(g, gd, P)) return false;
  const now = g.time, pp = g.player.char.pos;
  let S = gd.standoff;
  if (!S || S.pair !== P.held) {
    S = gd.standoff = { pair: P.held, since: now, side: pickSide(g, gd, P), way: 0, orbitGoal: null, replanT: 0, restT: 0, seenT: now, lastBarkT: -100, lineOpen: false, shots: 0, orbitM: 0, stalls: 0, cornered: false, backing: false };
    gd.witness.sawHeld = { who: P.held.callsign, t: now };   // he saw a colleague held, and whom (the dialogue's 'sawHurt': he watched Sam do it)
    gd.fireCd = Math.max(gd.fireCd, 0.8); gd.path = []; gd.pathGoal = null; gd.stallRef = null; gd.pieing = null; gd.searchPlan = null;   // (whatever walk he was on ends here; the first round waits for the gun to come round properly)
    if (!holdsCalled.has(P.key)) { holdsCalled.add(P.key); hostageCall(g, gd, P); } else standoffBark(g, gd, 'enter', true);
  }
  S.seenT = now;
  gd.awareness = 1; gd.lastKnown = v3.copy(pp); gd.lastSeenT = now; gd.sightPos = v3.copy(pp); gd.sightVel = v3.copy(P.vel); gd.sightT = now; gd.ledT = now;   // (ledT: nothing to lead — he is looking straight at them)
  gd.alertBest = 1e9; gd.alertStallT = 0; gd.alertRef = null; gd.chaseT = 0;
  return true;
}
/** The first man to see it calls it in — who, and where — and the floor answers: everybody else living and free comes to the place at the double, weapons up (as
 *  they would to gunfire: the far end a beat later), and the alarm goes straight to lockdown round the pair (raiseEscalation 'hostage': somebody else gives the
 *  order so the caller's own line stays up; no rooms are dealt for clearing — they know exactly where he is). Once per hold, floor-wide. */
function hostageCall(g: Game, gd: Guard, P: Pair) {
  const c = gd.char; const where = roomName(g.level, P.sam);
  g.say(gd, `he's got ${P.held.callsign} — ${where}! hold your fire, nobody push him`, true); g.audio.play('radio', c.bones.head ?? c.pos, 0.9);
  if (g.time - g.lastStingT > 8) { g.audio.play('alertSting', null, 0.75); g.lastStingT = g.time; }
  for (const o of g.guards) {
    if (o === gd || o === P.held || o.state === 'dead' || o.held) continue;
    const wasCalm = o.state === 'patrol' || o.state === 'suspicious';
    o.awareness = Math.max(o.awareness, 0.92); o.lastKnown = v3.copy(P.sam);
    if (wasCalm) o.reactT = Math.max(o.reactT, 0.25 + Math.min(1.2, v3.dist(o.char.pos, P.sam) / 25) + Math.random() * 0.4);   // (the far end of the floor takes a moment to place it, as for a shot)
  }
  g.alarm.pos[0] = P.sam[0]; g.alarm.pos[1] = 0; g.alarm.pos[2] = P.sam[2]; g.alarm.placed = true;
  raiseEscalation(g, 2, gd, 'hostage');
}
/** Which way round the ring this man works: the other way from a colleague already in the standoff on this pair; else the short way to their rear if the ring is
 *  open that way all the way round to it, else whichever way has more open ring (walkable floor with the pair still in sight from it, sampled every 20°). */
function pickSide(g: Game, gd: Guard, P: Pair): -1 | 1 {
  for (const o of g.guards) if (o !== gd && o.state !== 'dead' && o.standoff && o.standoff.pair === P.held) return (o.standoff.side > 0 ? -1 : 1);
  const nav = g.col.nav, C = P.centre, c = gd.char;
  const brg = Math.atan2(c.pos[0] - C[0], c.pos[2] - C[2]); const toRear = wrapAngle(P.facing + Math.PI - brg); const short: -1 | 1 = toRear < 0 ? -1 : 1;
  const room = (d: number) => { const span = d === short ? Math.abs(toRear) : 2 * Math.PI - Math.abs(toRear); let n = 0, full = true; for (let a = 20 * DEG; a <= span + 1e-6; a += 20 * DEG) { const b = brg + d * a; const p: Vec3 = [C[0] + Math.sin(b) * STANDOFF_ORBIT_M, 0, C[2] + Math.cos(b) * STANDOFF_ORBIT_M]; if (nav.isBlocked(p[0], p[2]) || g.col.segmentBlocked([p[0], 1.2, p[2]], [C[0], 1.2, C[2]])) { full = false; break; } n++; } return { n, full }; };
  const rs = room(short), rl = room(-short);
  return rs.full || rs.n >= rl.n ? short : (short > 0 ? -1 : 1);
}
/** metres between colleague `o`'s body axis (soles to crown about his pelvis) and the SEGMENT `from` → `to`: lineGeom's measure, but a man beyond either end is as
 *  far off it as he is from that end (the flat foot of the perpendicular clamped onto the segment; these segments are near enough level for that) */
function axisToSegment(from: Vec3, to: Vec3, o: Guard): number {
  const dx = to[0] - from[0], dy = to[1] - from[1], dz = to[2] - from[2]; const L2 = dx * dx + dz * dz;
  const h = o.char.bones.hips ?? o.char.pos; const ax = h[0], az = h[2];
  const t = L2 > 1e-6 ? clamp(((ax - from[0]) * dx + (az - from[2]) * dz) / L2, 0, 1) : 0;
  const px = from[0] + dx * t, py = from[1] + dy * t, pz = from[2] + dz * t;
  const y0 = o.char.pos[1] + 0.05, y1 = o.char.bones.headTop?.[1] ?? o.char.pos[1] + 1.75; const ddy = py < y0 ? y0 - py : py > y1 ? py - y1 : 0;
  return Math.hypot(px - ax, pz - az, ddy);
}
/** The fire rule (see the constants): may this man's round go, at Sam, past the colleague in his arm? ok, plus the three measures for whoever prints them: how far
 *  off the pair's rear he stands (rad), the held man's clearance off the round's segment (m), his distance from the pair (m). */
export function flankShot(g: Game, gd: Guard, P: Pair): { ok: boolean; rear: number; clear: number; dist: number } {
  const c = gd.char; const from = c.bones.upperArmR ?? v3.add(c.pos, [0, 1.4, 0]);
  const dist = v3.distXZ(c.pos, P.centre);
  const brg = Math.atan2(c.pos[0] - P.sam[0], c.pos[2] - P.sam[2]); const rear = Math.abs(wrapAngle(brg - (P.facing + Math.PI)));
  const to = P.aim; const d = v3.sub(to, from); const L = v3.len(d); const impact = L > SAM_R + 0.05 ? v3.mad(from, d, (L - SAM_R) / L) : to;   // where a true round stops: Sam's near side
  const clear = axisToSegment(from, impact, P.held);
  const ok = rear < STANDOFF_REAR_DEG * DEG && clear >= STANDOFF_CLEAR_M && dist <= STANDOFF_FIRE_MAX_M && dist >= STANDOFF_MIN_M - 0.6 && clearShot(g, c, to);
  return { ok, rear, clear, dist };
}
/** Placeholder lines (a proper writing pass comes later), by moment and temperament ('*' for anybody); {held} = the man in the arm. */
type StandoffMoment = 'enter' | 'noShot' | 'line' | 'cornered' | 'dropped' | 'freed' | 'lost' | 'hitHim';
const STANDOFF_LINES: Record<StandoffMoment, Partial<Record<Temperament | '*', string[]>>> = {
  enter: { steady: ['{held}, hold still — we\'ve got you', 'easy. easy. nobody has to get hurt here'], nervy: ['{held}! —jesus— okay, okay, nobody shoot!', 'oh god— he\'s got {held}—'], hard: ['let him go. now.', 'you. hands off him.'] },
  noShot: { '*': ['let him go!', 'drop him — NOW', '{held}, don\'t move', 'I don\'t have a shot', 'no shot — {held}\'s square in front of him'], steady: ['let him go and we talk about it', '{held}, stay loose — I\'m coming round'], nervy: ['{held}! —I can\'t— I haven\'t got a line—', 'don\'t you hurt him— don\'t—'], hard: ['you\'ve got nowhere to take him', 'the second he slips, you\'re done'] },
  line: { '*': ['I have him.', 'got a line —'], nervy: ['I— I think I have him—'], hard: ['there you are.'] },
  cornered: { '*': ['can\'t get round him — hold what you\'ve got', 'no way round. {held}, hang on', 'nowhere for either of us to go — let him walk and this ends quiet'], nervy: ['just— just let him go, please—'], hard: ['one way out of this for you. through us.'] },
  dropped: { '*': ['he\'s dropped {held} — TAKE HIM', '{held}\'s down — light him up!'], nervy: ['{held}! —he\'s— shoot, SHOOT—'] },
  freed: { '*': ['{held}, DOWN — get clear!', 'he\'s loose — {held}, out of the way!'] },
  lost: { '*': ['lost them — where\'d he take him?', 'I\'ve lost sight — moving up'], nervy: ['where— where did they go—'] },
  hitHim: { '*': ['…christ— I hit him. I hit {held}—'], nervy: ['no— no no no— {held}—'], hard: ['…{held}\'s hit. that\'s on him. keep on him—'] },
};
/** One standoff line from this man if the clocks allow (his own BARK_SECS, the floor's FLOOR_SECS, and never over his own radio call) — `force` skips the two
 *  clocks (the edges: entering, the line opening, the man dropping), 'always' the radio courtesy too (his own round in the man: that gets said); never the line
 *  he said last. */
function standoffBark(g: Game, gd: Guard, moment: StandoffMoment, force: boolean | 'always' = false, held: Guard | null = gd.standoff?.pair ?? null) {
  const S = gd.standoff, now = g.time;
  if (!force && ((S && now - S.lastBarkT < STANDOFF_BARK_SECS) || now - (floorBarkT.get(g) ?? -100) < STANDOFF_BARK_FLOOR_SECS)) return;
  if (force !== 'always' && gd.bubble?.radio && now - gd.bubble.t < 3) return;
  const M = STANDOFF_LINES[moment]; const pool = [...(M[TEMPERAMENT[gd.callsign] ?? 'steady'] ?? []), ...(M['*'] ?? [])]; if (!pool.length) return;
  let i = Math.floor(Math.random() * pool.length) % pool.length; if (pool.length > 1 && pool[i] === lastBark.get(gd)) i = (i + 1) % pool.length;
  const text = pool[i].replace(/\{held\}/g, held?.callsign ?? 'him');
  lastBark.set(gd, pool[i]); if (S) S.lastBarkT = now; floorBarkT.set(g, now);
  g.say(gd, text);
}
/** Side-step / back-pedal along the current nav path (followPath's tolerances and easing, but the body is not turned to it — the standoff keeps his chest to the
 *  pair — and no stuck side-step of its own: notGettingAnywhere watches the leg). True once the path is consumed. */
function stepAlongPath(gd: Guard, dt: number, speed: number): boolean {
  const c = gd.char;
  while (gd.pathI < gd.path.length) {
    const wp = gd.path[gd.pathI]; const dx = wp[0] - c.pos[0], dz = wp[2] - c.pos[2]; const d = Math.hypot(dx, dz);
    if (d < (gd.pathI === gd.path.length - 1 ? 0.25 : 0.45)) { gd.pathI++; continue; }
    gd.speed = damp(gd.speed, speed * clamp((d + (gd.path.length - 1 - gd.pathI)) / 0.8, 0.35, 1), 4, dt);
    c.vel = [dx / d * gd.speed, 0, dz / d * gd.speed]; c.pos = v3.mad(c.pos, c.vel, dt);
    return false;
  }
  gd.speed = damp(gd.speed, 0, 10, dt); c.vel = [0, 0, 0];
  return true;
}
/** The next point on the ring for this man: from his bearing off the pair, an arc's step (≤ 50°–70°) round toward dead astern of them — the short way once he is
 *  past their flank, his dealt side while he is still in front (two men split there) — at the ring's radius or near it, on walkable floor he can reach without
 *  touring the building or cutting through the pair, with the pair still in sight from it, and not onto a colleague's spot; the other way round if his way is
 *  walled off. Round already (inside SETTLE of astern): in or out to the ring on his own bearing, or null if he is standing on it. null with nothing either way =
 *  cornered: he holds where he is and keeps talking. */
function orbitGoalFor(g: Game, gd: Guard, S: Standoff, P: Pair): Vec3 | null {
  const nav = g.col.nav, c = gd.char, C = P.centre; const dbg = STANDOFF_DEBUG.log;
  const dist = v3.distXZ(c.pos, C); const brg = Math.atan2(c.pos[0] - C[0], c.pos[2] - C[2]);
  const toRear = wrapAngle(P.facing + Math.PI - brg); const short = toRear < 0 ? -1 : 1;
  const round = Math.abs(toRear) < STANDOFF_SETTLE_DEG * DEG;
  if (round && Math.abs(dist - STANDOFF_ORBIT_M) < 0.6) { S.cornered = false; dbg?.(`${gd.callsign}: round (toRear ${Math.round(toRear / DEG)}°, ${dist.toFixed(1)} m) — plant`); return null; }   // there: on the ring behind them
  // which way round: the way he is already going (a man who re-decides every second at the edge of a walled arc see-saws on it), unless he has plainly ended up on
  // the wrong side of them (Sam turned the pair: the short way is now the other, and by a lot); not yet going: the short way — or, square in front where both
  // are as long, his dealt side (two men split there)
  if (S.way && Math.abs(toRear) < 120 * DEG && short !== S.way) S.way = short;
  const dir = round ? short : S.way || (Math.abs(toRear) < 150 * DEG ? short : S.side);
  if (dist > STANDOFF_MAX_M) { const ap = approachPoint(g, gd, C); if (ap) { S.cornered = false; dbg?.(`${gd.callsign}: from ${dist.toFixed(1)} m — straight over, to (${ap[0].toFixed(1)}, ${ap[2].toFixed(1)})`); return ap; } }   // from across the floor he comes over first, up his own path to them as far as the ring; the flanking starts there
  const others = g.guards.filter(o => o !== gd && o.state !== 'dead' && o.standoff && o.standoff.pair === P.held);
  let plans = 0;
  for (const way of (S.way ? [dir] : [dir, -dir]) as (-1 | 1)[]) {   // (once he is going one way, a wall across it corners him THERE — he does not walk back round the front to try the other flank while Sam watches; not yet going, either way is a way)
    for (const stepDeg of round ? [0] : [50, 30, 70]) {   // (no shorter: a step that gains no real angle on them is a shuffle — where the walls allow no more than that he is cornered, and stands)
      const span = way === short ? Math.abs(toRear) : 2 * Math.PI - Math.abs(toRear);   // (never on past dead astern: that is where he is going)
      const b = brg + way * Math.min(stepDeg * DEG, span);
      for (const r of [STANDOFF_ORBIT_M, 4.0, 5.2, 3.9, 3.6, 5.8]) {
        const p: Vec3 = [C[0] + Math.sin(b) * r, 0, C[2] + Math.cos(b) * r]; const tag = dbg ? `${gd.callsign}: way ${way > 0 ? '+' : '−'} ${stepDeg}° r${r} → (${p[0].toFixed(1)}, ${p[2].toFixed(1)})` : '';
        if (nav.isBlocked(p[0], p[2]) || v3.distXZ(p, c.pos) < 0.5) { dbg?.(`${tag}: ${nav.isBlocked(p[0], p[2]) ? 'blocked cell' : 'where he stands'}`); continue; }
        if (g.col.segmentBlocked([p[0], 1.2, p[2]], [C[0], 1.2, C[2]])) { dbg?.(`${tag}: no sight of them`); continue; }                      // he must still have them from there
        if (g.col.segmentBlocked([(p[0] + c.pos[0]) / 2, 1.2, (p[2] + c.pos[2]) / 2], [C[0], 1.2, C[2]])) { dbg?.(`${tag}: out of their sight on the way`); continue; }   // …and, near enough, on the way there (a ring point seen through a doorway from the next room is not a flank: he would walk out of sight of them to reach it)
        if (others.some(o => v3.distXZ(o.standoff!.orbitGoal ?? o.char.pos, p) < 1.5)) { dbg?.(`${tag}: a colleague's spot`); continue; }   // not onto (or on the way onto) a colleague's spot
        if (++plans > 12) { S.cornered = false; dbg?.(`${tag}: out of plans`); return dist > STANDOFF_MAX_M ? approachPoint(g, gd, C, false) : null; }   // (A* is the dear part: a dozen, then he stands this second out — or just comes over, from afar)
        const path = nav.findPath(c.pos, p); if (!path || !path.length) { dbg?.(`${tag}: no path`); continue; }
        let L = 0, q: Vec3 = c.pos, cuts = false; for (const w of path) { L += v3.distXZ(q, w); q = w; if (v3.distXZ(w, C) < STANDOFF_MIN_M - 0.7) cuts = true; }
        if (cuts || L > 2.5 * v3.distXZ(c.pos, p) + 2.5) { dbg?.(`${tag}: ${cuts ? 'path cuts through them' : `path too long (${L.toFixed(1)} m)`}`); continue; }   // round the ring — not through them, not round the building
        S.cornered = false; if (!round) S.way = way; dbg?.(`${tag}: TAKEN (toRear ${Math.round(toRear / DEG)}°, from ${dist.toFixed(1)} m at brg ${Math.round(brg / DEG)}°)`); return p;
      }
    }
    if (round) break;
  }
  dbg?.(`${gd.callsign}: nothing either way (toRear ${Math.round(toRear / DEG)}°, ${dist.toFixed(1)} m) — ${dist > STANDOFF_MAX_M ? 'approach' : 'cornered'}`);
  // no ring to speak of (a corridor: the walls take both flanks): from across the floor he still comes over — up his path to them, stopping the ring's radius
  // short; already there, he is cornered with them: he holds where he stands and keeps talking
  if (dist > STANDOFF_MAX_M) { S.cornered = false; return approachPoint(g, gd, C, false); }   // (no sight of them from where his path meets the ring — a partition: he comes up to it all the same and looks again from there)
  if (dist > STANDOFF_ORBIT_M + 1.0) { const ap = approachPoint(g, gd, C, false); if (ap && v3.distXZ(ap, c.pos) > 0.6) { S.cornered = true; dbg?.(`${gd.callsign}: no ring here — up to it, to (${ap[0].toFixed(1)}, ${ap[2].toFixed(1)})`); return ap; } }   // no flank to be had (a corridor), but he is not yet AT the ring: up to it, square on, and there he holds
  S.cornered = true; return null;
}
/** the point on this man's own nav path to `C` where it first comes within the ring's radius of it (found along the leg that crosses the ring, not just at a corner
 *  of the path — the last leg up a corridor is one long stride), if they are in sight from there; null = no path, or no sight of them from that point (a
 *  partition between: the ring search proper deals with that) */
function approachPoint(g: Game, gd: Guard, C: Vec3, needSight = true): Vec3 | null {
  const path = g.col.nav.findPath(gd.char.pos, C); if (!path?.length) return null;
  const R = STANDOFF_ORBIT_M; let a: Vec3 = gd.char.pos;
  for (const b of path) {
    if (v3.distXZ(b, C) <= R) {   // this leg crosses the ring: bisect for the crossing (distance to C falls monotonically enough along one leg for that)
      let lo = 0, hi = 1; for (let i = 0; i < 16; i++) { const m = (lo + hi) / 2; const q: Vec3 = [a[0] + (b[0] - a[0]) * m, 0, a[2] + (b[2] - a[2]) * m]; if (v3.distXZ(q, C) > R) lo = m; else hi = m; }
      const p: Vec3 = [a[0] + (b[0] - a[0]) * hi, 0, a[2] + (b[2] - a[2]) * hi];
      const free = !g.col.nav.isBlocked(p[0], p[2]) ? p : g.col.nav.nearestFreePoint(p[0], p[2]);
      if (!free || v3.distXZ(free, C) < STANDOFF_MIN_M) return null;
      return needSight && g.col.segmentBlocked([free[0], 1.2, free[2]], [C[0], 1.2, C[2]]) ? null : [free[0], 0, free[2]];
    }
    a = b;
  }
  return null;   // (the path never comes within the ring of them: cannot happen for a path TO them, bar a nav pocket — the ring search has it)
}
/** Where a man too close to the pair backs off to: the ring's inner edge on his own bearing from them for choice, else round to either side of that as far as
 *  square off it (along the corridor when the wall is at his back), straight over open floor from where he stands; failing the ring, any spot a stride or two
 *  off that puts a metre more between them. null = boxed in: he stands his ground there. */
function retreatGoal(g: Game, gd: Guard, P: Pair): Vec3 | null {
  const nav = g.col.nav, c = gd.char, C = P.centre; const brg = Math.atan2(c.pos[0] - C[0], c.pos[2] - C[2]);
  const open = (p: Vec3) => !nav.isBlocked(p[0], p[2]) && nav.walkable(c.pos, p) && !g.col.segmentBlocked([c.pos[0], 0.5, c.pos[2]], [p[0], 0.5, p[2]]);
  for (const da of [0, 0.5, -0.5, 1.0, -1.0, 1.5, -1.5]) for (const r of [STANDOFF_MIN_M + 0.7, STANDOFF_MIN_M + 0.2, STANDOFF_MIN_M + 1.4]) {
    const a = brg + da; const p: Vec3 = [C[0] + Math.sin(a) * r, 0, C[2] + Math.cos(a) * r];
    if (open(p)) return p;
  }
  const d0 = v3.distXZ(c.pos, C); let best: Vec3 | null = null, bd = d0 + 0.8;   // no way onto the ring from here: whatever gains the most ground off them within a couple of strides
  for (let i = 0; i < 16; i++) for (const s of [2.0, 1.2]) {
    const a = i * Math.PI / 8; const p: Vec3 = [c.pos[0] + Math.sin(a) * s, 0, c.pos[2] + Math.cos(a) * s];
    const d = v3.distXZ(p, C); if (d > bd && open(p)) { bd = d; best = p; }
  }
  return best;
}
/** The alert branch for a man in the standoff (updateGuard: after his CONTACT beat, in place of the chase and the shot; upkeep has just confirmed the hold is on
 *  and it is this pair). Feet: back off inside MIN; plant once flankShot says the line is his; otherwise the next ring point toward their rear (re-planned every
 *  second as the pair turns, at a walk inside MAX and the investigate jog beyond it), side-stepping with his chest kept to them — except on the jog over from
 *  beyond MAX, where the legs have the body and he looks up his path whenever it turns its back on them; a leg that goes nowhere is dropped and he rests a beat.
 *  Gun: aimed at the base of Sam's neck; muzzle discipline exactly as everywhere (the held man lowers it) except that with the flank line open the held man no
 *  longer counts — then single rounds, SHOT_SECS apart, squarely aimed. Mouth: the line for the moment. Returns his move speed and the point he looks / aims at. */
function standoffAlert(g: Game, gd: Guard, dt: number, P: Pair): { move: number; aim: Vec3 } {
  const S = gd.standoff!, c = gd.char, T = g.tune;
  const aim = P.aim; const dist = v3.distXZ(c.pos, P.centre);
  const fs = flankShot(g, gd, P);
  muzzleCheck(g, gd, aim, dt, fs.ok ? P.held : null);
  if (fs.ok && !S.lineOpen) standoffBark(g, gd, 'line', g.time - S.lastBarkT > 2.5);   // (the edge is worth saying — not every other second while Sam swings the man to and fro)
  S.lineOpen = fs.ok;
  // --- feet ---
  let move = 0; const far = dist > STANDOFF_MAX_M; const before0 = c.pos[0], before2 = c.pos[2];
  S.replanT -= dt; S.restT = Math.max(0, S.restT - dt);
  const setGoal = (p: Vec3 | null) => { if (!p) { if (S.orbitGoal) { gd.path = []; gd.pathGoal = null; } S.orbitGoal = null; return; } if (!S.orbitGoal || v3.distXZ(S.orbitGoal, p) > 0.3) gd.stallRef = null; S.orbitGoal = p; };
  const backing = dist < STANDOFF_MIN_M - 0.3;   // (a hand inside the ring's edge is standing on it, not inside it: no see-saw between backing off and coming in)
  const pace = far || backing ? T.guardInvestigate : T.guardWalk;   // the jog over from across the floor, and off a man walking his hostage at him (Sam hauls the pair at a slow walk: a walk backwards would never open the gap); the ring itself he walks
  if (backing) { if (S.replanT <= 0 || !S.orbitGoal || !S.backing) { S.replanT = 0.5; setGoal(retreatGoal(g, gd, P)); } }   // too close: back off to the ring (or as far as the walls let him), facing them — looked at again twice a second while it lasts
  else if (fs.ok && (!S.orbitGoal || fs.rear < (STANDOFF_REAR_DEG - 15) * DEG)) setGoal(null);            // his line: plant and use it (the leg he is on he finishes first, unless he is already well inside — a line found on the sector's very edge goes again with a sway of Sam's)
  else if (S.restT > 0) setGoal(null);                                                                      // a leg just went nowhere: stand a beat
  else if (S.replanT <= 0 || !S.orbitGoal) { S.replanT = STANDOFF_REPLAN_SECS; setGoal(orbitGoalFor(g, gd, S, P)); }
  S.backing = backing;
  let jogging = false; let lookAt = aim;   // on the jog over from across the floor: the legs have the body, and where his eyes and gun go (the pair if the run keeps them anywhere ahead of him, else up his path)
  if (S.orbitGoal) {
    if (backing) { if (gd.pathGoal !== S.orbitGoal) { gd.path = [S.orbitGoal]; gd.pathI = 0; gd.pathGoal = S.orbitGoal; } }   // (the back-off is a straight step over floor retreatGoal has checked, not a planned walk)
    else goTo(g, gd, S.orbitGoal);
    const done = far ? followPath(g, gd, dt, pace) : stepAlongPath(gd, dt, pace); move = gd.speed;   // from across the floor he simply comes over; inside the ring's reach he side-steps with his chest to them
    if (done) { setGoal(null); S.replanT = Math.min(S.replanT, 0.25); }
    else if (notGettingAnywhere(gd, dt, STANDOFF_STALL_SECS)) { setGoal(null); gd.stallRef = null; S.restT = STANDOFF_REST_SECS; if (++S.stalls % 2 === 0) { S.side = S.side > 0 ? -1 : 1; S.way = S.way > 0 ? -1 : S.way < 0 ? 1 : 0; } }   // (twice running: try the other way round)
    else if (far && gd.pathI < gd.path.length) {
      // The jog over: followPath turns him up the leg he is on and will not carry a man faced more than ~84° off it above a creep (0.15 of the pace) until he
      // has come round — so nothing here may turn him back to face the pair meanwhile. It used to ('chest to them' whenever he was under 0.3 m/s, and the
      // generic stand-and-face-your-aim in updateGuard on top): with the way to them leading off through the door behind him the two turns beat followPath's,
      // and he hung at exactly that creep, 0.26 m/s sideways along his facing into the wall, for as long as the hold lasted (the soak's 'standoff-far'). His
      // eyes and gun stay on the pair while the run keeps them anywhere ahead of him; a leg that turns its back on them he runs looking where he is going
      // (four seconds out of his view and it is the plain chase on where they were, as from anywhere — he cannot watch them through the doorway behind him).
      jogging = true;
      const wp = gd.path[gd.pathI]; const yawPath = Math.atan2(wp[0] - c.pos[0], wp[2] - c.pos[2]);
      if (Math.abs(wrapAngle(Math.atan2(aim[0] - c.pos[0], aim[2] - c.pos[2]) - yawPath)) > 75 * DEG) lookAt = [c.pos[0] + Math.sin(yawPath) * 4, aim[1], c.pos[2] + Math.cos(yawPath) * 4];
    }
  } else { gd.speed = damp(gd.speed, 0, 6, dt); c.vel = [0, 0, 0]; gd.stallRef = null; }
  S.orbitM += Math.hypot(c.pos[0] - before0, c.pos[2] - before2);
  if (!jogging) c.bodyYaw = approachAngle(c.bodyYaw, Math.atan2(aim[0] - c.pos[0], aim[2] - c.pos[2]), 4 * dt);   // chest to them, always: the legs side-step and back-pedal under it — bar the jog over from across the floor (above)
  // --- the round: single aimed shots, slow, at Sam only, only down the flank line ---
  gd.fireCd -= dt;
  const facingErr = Math.abs(wrapAngle(Math.atan2(aim[0] - c.pos[0], aim[2] - c.pos[2]) - c.aimYaw));
  if (gd.reloadT >= 0) { gd.reloadT -= dt; if (gd.reloadT < 0) gd.shots = 0; }
  else if (fs.ok && !backing && gd.fireCd <= 0 && facingErr < 0.12 && !gd.muzzleDown && !armThrough(g, c) && !roundPastColleague(g, gd, aim, P.held)) {   // (not on the jog backwards off them: an aimed round is a planted or side-stepping man's)
    gd.fireCd = STANDOFF_SHOT_SECS + Math.random() * 0.6; fireWeapon(g, c, aim, false); gd.shots++; S.shots++;
    if (gd.shots >= 12) { gd.reloadT = 1.7; c.anim.reload(); g.audio.play('magOut', c.pos, 0.5); g.say(gd, 'reloading!'); }
  }
  // --- the talking ---
  if (!fs.ok) standoffBark(g, gd, S.cornered && !S.orbitGoal ? 'cornered' : 'noShot');
  return { move, aim: lookAt };
}
/** Every frame for a man carrying a standoff record (updateGuard, after the transitions, AI on or off): is it still one? Dropped when the tour flag / a scripted
 *  hold has come up or the AI has been switched off (nobody home to play it: taken up afresh when it comes back on), when he is somehow no longer alert, when the hold has ended — the man dead in or from the arm (with this one watching: found there and then,
 *  and his next round comes as fast as the gun comes round), or let go (the plain fight; the freed man is in everyone's line for a second and muzzle discipline
 *  holds it) — or when he has not had the pair in view for LOSE_SECS (the plain chase on where they were, then the search). Returns the pair when the standoff
 *  stands, for the alert branch. */
function standoffUpkeep(g: Game, gd: Guard): Pair | null {
  const S = gd.standoff; if (!S) return null;
  const P = pairOf(g);
  const why = g.quietUtility || gd.hold || gd.pinned || !g.aiEnabled ? 'off' : gd.state !== 'alert' ? 'state' : !P || P.held !== S.pair ? (S.pair.state === 'dead' ? 'dead' : g.player.down ? 'samDown' : 'free') : g.time - S.seenT > STANDOFF_LOSE_SECS ? 'lost' : null;   // (samDown: the hold ended because Sam went down with the man in his arm — nothing to shout to a colleague already stumbling clear of a body; the alert branch covers the rest)
  if (!why) return P;
  gd.standoff = null; gd.path = []; gd.pathGoal = null; gd.stallRef = null; resetChase(gd);
  const watched = g.time - S.seenT < 0.6;
  if (why === 'dead' && watched) {
    const B = S.pair;
    if (g.guards.includes(B) && B.state === 'dead') {
      if (!B.found) { B.found = true; tally(g, 'bodies'); }   // he watched him go down: nobody needs to walk onto this one to know
      gd.witness.sawBody = { victim: B.callsign, t: g.time, breathing: isBreathing(B) };
      if (B.downBy !== 'guard') standoffBark(g, gd, 'dropped', true, B);   // (their own round: the man who fired it has said so)
    }
    gd.fireCd = Math.min(gd.fireCd, 0.15 + Math.random() * 0.3);   // the shield is gone
  } else if (why === 'free' && watched) { standoffBark(g, gd, 'freed', true, S.pair); gd.fireCd = Math.max(gd.fireCd, 0.4); }
  else if (why === 'lost') { standoffBark(g, gd, 'lost', true, S.pair); gd.lastSeenT = Math.min(gd.lastSeenT, S.seenT); }
  return null;
}
/** The friendly-fire line (combat.ts fireWeapon: a colleague's round found the man in Sam's arm): the shooter says so, whatever the clocks. */
export function hostageHitBark(g: Game, shooter: Guard, victim: Guard) { standoffBark(g, shooter, 'hitHim', 'always', victim); }
/** Panel / QA: one line per man in a standoff — distance, degrees off the pair's rear, the held man's clearance off his line, side, what his feet are doing, rounds. */
export function standoffSummary(g: Game): string {
  const P = pairOf(g); const parts: string[] = [];
  for (const gd of g.guards) {
    const S = gd.standoff; if (!S || gd.state === 'dead') continue;
    const fs = P && P.held === S.pair ? flankShot(g, gd, P) : null;
    parts.push(`${gd.callsign}: ${fs ? `${fs.dist.toFixed(1)} m, ${Math.round(fs.rear / DEG)}° off their rear, clear ${fs.clear.toFixed(2)}${fs.ok ? ' LINE' : ''}` : 'pair gone'} · side ${S.side > 0 ? '+' : '−'} · ${S.orbitGoal ? `to (${S.orbitGoal[0].toFixed(1)}, ${S.orbitGoal[2].toFixed(1)})` : S.cornered ? 'cornered' : S.restT > 0 ? 'resting' : 'planted'} · ${S.shots} shot${S.shots === 1 ? '' : 's'}, ${S.orbitM.toFixed(1)} m walked, seen ${(g.time - S.seenT).toFixed(1)} s ago`);
  }
  return parts.join(' | ');
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
  // no route (the fix is inside a wall / a sealed pocket): only fall back to a bee-line when the straight line is actually WALKABLE for a man his width — not merely
  // clear to a thin ray: a ray threads the hand-wide slot between the server racks that his shoulders (just) squeeze through and the nav bake rightly calls shut,
  // and a man bee-lined through it after a noise was marooned in the strip behind the racks for the rest of the night, no route back to anything — otherwise
  // an empty path, which followPath reports as 'arrived' so the state machine moves on (search here) instead of grinding a partition for the rest of the encounter
  const direct = !p && g.col.nav.walkable(gd.char.pos, goal) && !g.col.segmentBlocked([gd.char.pos[0], 0.5, gd.char.pos[2]], [goal[0], 0.5, goal[2]]);
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

// ------------------------------------------------------------ the search that reasons about darkness
// A man dropping to 'search' round a fix used to sweep from where he stood and then wander to a couple of random points three metres off it. Now he deals himself
// the places round the fix a man would actually hide in — floor hard against walls and in corners (nav cells with statics about them), the wells behind desks
// and chairs and cartons (cells beside props), the pocket behind a leaf standing open, the far side of a partition from the fix, and a few spots of open floor
// as controls — and visits them best-first by DARK × COVER × PLAUSIBLE, sweeping his light INTO each as he comes up to it: dark as the RENDERER has it (the
// same GPU irradiance queries the light meter is made of, mapped through the meter's own curve — a spot dark for the meter is dark for him), cover from what
// stands about the cell, plausible = Sam could have got there from the fix at a sprint in the time since it was placed (not through a leaf locked against him,
// and through one standing latched again only once there has been time to ease it, slip through and let it swing to). Two men searching split the spots
// between them over the net (nobody takes a spot a colleague is on or has looked into). Headless (the QA harness) no reading ever arrives: every spot stays
// at 'no idea' (0.5) and the order falls to cover and plausibility — the old behaviour, better aimed.
/** hide spots are dealt within this many metres' walk of the fix; cover spots + open-floor controls (≤ 16); no two nearer each other than SPACING */
const SEARCH_RADIUS_M = 9, SEARCH_SPOTS = 12, SEARCH_CONTROLS = 3, SEARCH_SPACING_M = 1.5;
/** the first look round from where he stands (also the light readings' head start); the torch held into a spot on arrival; how near, with the spot in plain
 *  sight of his eyes, counts as 'at' it (he looks into a corner from the mouth of it, not standing on it) */
const SEARCH_SWEEP0_SECS = 1.2, SEARCH_LOOK_SECS = 1.5, SEARCH_LOOK_DIST_M = 1.9;
/** the search never ends before MIN (the old fixed length) and never runs past CAP; between the two it ends when the plan is done (spots all looked into or none
 *  left worth the walk); plausibility (the flood) is re-worked every REFLOOD seconds — leaves open and shut, and time passes */
const SEARCH_MIN_SECS = 12, SEARCH_CAP_SECS = 26, SEARCH_REFLOOD_SECS = 2;
/** light queries per frame across ALL searchers (the meter itself spends 2 + a smoke segment per living guard of the probe's 32), read half a metre off the
 *  floor (a crouched man's chest; the shade under a desk top); a reading lives WINDOW seconds and the darkest live one counts — a torch sweeping past lights a
 *  spot for a moment, three seconds of torch on it is a man looking into it */
const PROBE_BUDGET = 8, PROBE_HEIGHT = 0.5, DARK_WINDOW_SECS = 3;
/** what slipping through a leaf that stands LATCHED again now would have cost Sam: ease it off the latch, through, and the closer's swing back till it catches
 *  (doors.ts: ~1 s to crack, 2 s untouched before the closer, ~2 s of swing) — inside that of the loss, nobody is behind that door */
const LATCH_SECS = 5.5;
/** a spot Sam could not QUITE have reached yet is not nothing (the fix itself is a guess): full weight up to SLACK past what a sprint allows, fading over RAMP */
const PLAUS_SLACK_SECS = 1.2, PLAUS_RAMP_SECS = 2.5;

export interface SearchSpot {
  pos: Vec3; cell: number; key: string;
  /** what makes it a hiding place: three statics about it / one or two / a chair, carton, plant or bin beside it / an open leaf to stand behind / plain floor (control) */
  kind: 'corner' | 'wall' | 'prop' | 'leaf' | 'open';
  /** out of sight of the fix (a partition, a desk, a jamb between): where a man who ducked away from THERE would be */
  hidden: boolean;
  cover: number;   // 0‥1 from the above
  dark: number;    // 0‥1: 1 − the meter's visibility for the irradiance read there (0.5 while nothing has been read)
  /** the irradiance behind `dark` (W/m², darkest reading inside the window) — undefined = no reading (headless, or not asked yet) */
  E: number | undefined;
  need: number;    // seconds Sam needs to get here from the fix at a sprint, doors included (Infinity: not from there he didn't)
  plaus: number; score: number;
  visited: boolean; by: number;   // by: char id of the man who looked into it (or whose look covered it), −1
  unreachable: boolean;           // he could not get to it (three seconds of no headway): counts as done
  samples: { t: number; E: number }[];
}
export interface SearchPlan {
  fix: Vec3; t0: number;          // the fix it was dealt round and when that fix was placed (Guard.fixT: how long Sam has had)
  serial: number; builtAt: number; floodAt: number;
  spots: SearchSpot[]; current: number;   // index of the spot he is walking to / looking into, −1
  phase: 'sweep' | 'walk' | 'look' | 'done'; phaseT: number;
  visits: number; rr: number; said: boolean;
}

/** the doorway cells of every door: nav cell index → the door whose closed frame line runs through it (worked out once per level; doors do not move) */
const doorCellsOf = new WeakMap<Game, Map<number, Door>>();
function doorCells(g: Game): Map<number, Door> {
  let M = doorCellsOf.get(g); if (M) return M;
  M = new Map(); const nav = g.col.nav;
  for (const d of g.doors.list) {
    const [hx, hz] = d.def.hinge, ux = Math.cos(d.def.closedDir), uz = Math.sin(d.def.closedDir);
    for (let a = 0.1; a < d.def.width - 0.05; a += 0.2) for (const off of [0, 0.18, -0.18]) {   // (a hair either side of the line too: a frame line lying exactly on a cell edge must still own a cell)
      const x = hx + ux * a - uz * off, z = hz + uz * a + ux * off; if (nav.isBlocked(x, z)) continue;
      M.set(nav.idx(Math.floor(x / nav.cell), Math.floor(z / nav.cell)), d);
    }
  }
  doorCellsOf.set(g, M); return M;
}
/** Dijkstra over the nav grid outward from `from`, in SECONDS of Sam's sprint: a step costs its metres over the sprint speed; stepping INTO a doorway cell whose
 *  leaf stands latched costs LATCH_SECS on top — or everything, if that leaf is locked against the side he would come at it from (no key; a pick is four loud
 *  seconds, a kick a gunshot's worth of noise: neither is vanishing). `keyed` (a guard's own walk): doors cost nothing and stop nobody. Not expanded past `maxM`
 *  metres of path. secs / metres per cell, Infinity = not reached. */
function floodFrom(g: Game, from: Vec3, maxM: number, keyed = false): { secs: Float32Array; metres: Float32Array } {
  const nav = g.col.nav, W = nav.W, H = nav.H, N = W * H, cs = nav.cell; const sprint = Math.max(1, g.tune.playerSprint);
  const secs = new Float32Array(N).fill(Infinity), metres = new Float32Array(N).fill(Infinity); const closed = new Uint8Array(N);
  const DC = doorCells(g);
  let sx = Math.floor(from[0] / cs), sz = Math.floor(from[2] / cs);
  if (sx < 0 || sz < 0 || sx >= W || sz >= H) return { secs, metres };
  if (nav.blocked[nav.idx(sx, sz)]) { const p = nav.nearestFreePoint(from[0], from[2]); if (!p) return { secs, metres }; sx = Math.floor(p[0] / cs); sz = Math.floor(p[2] / cs); }
  const start = nav.idx(sx, sz); secs[start] = 0; metres[start] = 0;
  const open: number[] = [start];
  while (open.length) {
    let mi = 0; for (let i = 1; i < open.length; i++) if (secs[open[i]] < secs[open[mi]]) mi = i;   // (linear pop-min, as findPath: a few hundred live entries at most)
    const cur = open[mi]; open[mi] = open[open.length - 1]; open.pop();
    if (closed[cur]) continue; closed[cur] = 1;
    if (metres[cur] > maxM) continue;
    const cx = cur % W, cz = (cur / W) | 0; const curDoor = DC.get(cur);
    for (let dz = -1; dz <= 1; dz++) for (let dx = -1; dx <= 1; dx++) {
      if (!dx && !dz) continue; const nx = cx + dx, nz = cz + dz;
      if (nx < 0 || nz < 0 || nx >= W || nz >= H) continue;
      const ni = nav.idx(nx, nz); if (nav.blocked[ni] || closed[ni]) continue;
      if (dx && dz && (nav.blocked[nav.idx(cx + dx, cz)] || nav.blocked[nav.idx(cx, cz + dz)])) continue;   // no corner cutting (as findPath)
      const step = (dx && dz ? Math.SQRT2 : 1) * cs; let extra = 0;
      const d = keyed ? undefined : DC.get(ni);
      if (d && d !== curDoor && d.latched) {   // onto a doorway whose leaf is shut: from which face?
        if (d.locked && !d.lockBroken && g.doors.side(d, [(cx + 0.5) * cs, 0, (cz + 0.5) * cs]) === d.keySide) continue;   // locked against him from there: not this way
        extra = LATCH_SECS;
      }
      const ns = secs[cur] + step / sprint + extra;
      if (ns < secs[ni]) { secs[ni] = ns; metres[ni] = metres[cur] + step; open.push(ni); }
    }
  }
  return { secs, metres };
}

/** Deal the plan's spots round P.fix (see the block comment): flood for reach and time, count what stands about every reached cell (blocked neighbours = statics,
 *  props within reach, an open leaf beside it), keep the best-covered forty a spacing apart, ask the sight line from the fix for those (hidden = the far side of
 *  something), take the top SEARCH_SPOTS by cover and up to SEARCH_CONTROLS well-spread cells of open floor. Keys srch:<guard>:<i>, stale answers dropped. */
function dealSpots(g: Game, gd: Guard, P: SearchPlan) {
  const nav = g.col.nav, W = nav.W, Hn = nav.H, cs = nav.cell, N = W * Hn; const gi = Math.max(0, g.guards.indexOf(gd));
  const { secs, metres } = floodFrom(g, P.fix, SEARCH_RADIUS_M);
  // props: every cell within reach of a chair / carton / plant / bin counts it (a man folds himself in beside and behind such things)
  const propN = new Uint8Array(N);
  for (const p of g.props.props) {
    const r = p.radius + 0.7; const x0 = Math.max(0, Math.floor((p.x - r) / cs)), x1 = Math.min(W - 1, Math.floor((p.x + r) / cs)), z0 = Math.max(0, Math.floor((p.z - r) / cs)), z1 = Math.min(Hn - 1, Math.floor((p.z + r) / cs));
    for (let z = z0; z <= z1; z++) for (let x = x0; x <= x1; x++) if (Math.hypot((x + 0.5) * cs - p.x, (z + 0.5) * cs - p.z) < r) propN[z * W + x] = Math.min(255, propN[z * W + x] + 1);
  }
  // open leaves: the cells close along a leaf standing well open, off the doorway itself — behind the door
  const leafN = new Uint8Array(N);
  for (const d of g.doors.list) {
    if (Math.abs(d.angle) < 0.7) continue;
    const [hx, hz] = d.def.hinge, u = d.dir(), fc = d.frameCentre;
    for (let a = 0.2; a <= d.def.width; a += 0.25) for (const s of [-0.55, 0.55]) {
      const x = hx + u[0] * a - u[1] * s, z = hz + u[1] * a + u[0] * s;
      if (Math.hypot(x - fc[0], z - fc[2]) < 0.7 || nav.isBlocked(x, z)) continue;
      leafN[nav.idx(Math.floor(x / cs), Math.floor(z / cs))] = 1;
    }
  }
  // every reached cell a stride or more off the fix: what stands about it
  type Cand = { cell: number; pos: Vec3; kind: SearchSpot['kind']; cheap: number; m: number; hidden: boolean; cover: number };
  const cands: Cand[] = [], opens: Cand[] = [];
  for (let i = 0; i < N; i++) {
    if (!Number.isFinite(secs[i]) || metres[i] > SEARCH_RADIUS_M || metres[i] < 1.5) continue;
    const cx = i % W, cz = (i / W) | 0; let nWall = 0;
    for (let dz = -1; dz <= 1; dz++) for (let dx = -1; dx <= 1; dx++) { if (!dx && !dz) continue; const nx = cx + dx, nz = cz + dz; if (nx < 0 || nz < 0 || nx >= W || nz >= Hn || nav.blocked[nav.idx(nx, nz)]) nWall++; }
    const nProp = propN[i], nLeaf = leafN[i];
    const cheap = 0.12 * Math.min(nWall, 5) + 0.2 * Math.min(nProp, 2) + 0.45 * nLeaf;   // (+0.3 hidden from the fix, below — on the half-metre grid a cell hard against a straight run has the three cells behind it blocked, an inside corner five: a wall 0.36 / 0.66 hidden, a corner 0.6 / 0.9 hidden, more with a chair in it; a lone chair in the aisle 0.2; behind a leaf 0.8+; open floor 0.1)
    const kind: SearchSpot['kind'] = nLeaf ? 'leaf' : nWall >= 5 ? 'corner' : nProp ? 'prop' : nWall ? 'wall' : 'open';
    const c: Cand = { cell: i, pos: [(cx + 0.5) * cs, 0, (cz + 0.5) * cs], kind, cheap, m: metres[i], hidden: false, cover: 0 };
    (cheap > 0 ? cands : opens).push(c);
  }
  cands.sort((a, b) => (b.cheap - a.cheap) || (a.m - b.m));
  const kept: Cand[] = []; const farFrom = (p: Vec3, list: Cand[], r: number) => list.every(k => v3.distXZ(k.pos, p) >= r);
  for (const c of cands) { if (kept.length >= 40) break; if (farFrom(c.pos, kept, SEARCH_SPACING_M)) kept.push(c); }
  const eyeFix: Vec3 = [P.fix[0], 1.0, P.fix[2]];
  for (const c of kept) { c.hidden = g.col.segmentBlocked(eyeFix, [c.pos[0], 0.6, c.pos[2]]); c.cover = clamp(c.cheap + (c.hidden ? 0.3 : 0), 0.05, 1); }
  kept.sort((a, b) => (b.cover - a.cover) || (a.m - b.m));
  const chosen = kept.slice(0, SEARCH_SPOTS);
  opens.sort((a, b) => b.m - a.m);   // controls: open floor, as far out and as far apart as the place allows
  let controls = 0;
  for (const c of opens) { if (controls >= SEARCH_CONTROLS) break; if (farFrom(c.pos, chosen, 2.5)) { c.hidden = g.col.segmentBlocked(eyeFix, [c.pos[0], 0.6, c.pos[2]]); c.cover = c.hidden ? 0.3 : 0.1; chosen.push(c); controls++; } }
  P.spots = chosen.map((c, i): SearchSpot => {
    const key = `srch:${gi}:${i}`; g.lightForget(key);   // (the last plan's answer under this key was for another spot)
    return { pos: c.pos, cell: c.cell, key, kind: c.kind, hidden: c.hidden, cover: c.cover, dark: 0.5, E: undefined, need: secs[c.cell], plaus: 1, score: 0, visited: false, by: -1, unreachable: false, samples: [] };
  });
  P.floodAt = g.time; P.current = -1; P.rr = 0;
}

/** The plan for this man's search, built on its first frame round his fix (his own feet if he has none) and rebuilt if the fix has since moved a couple of
 *  metres (a creak placed somewhere else while he searched); plausibility re-flooded every SEARCH_REFLOOD_SECS. */
export function searchPlanFor(g: Game, gd: Guard): SearchPlan {
  const fix: Vec3 = gd.lastKnown ? [gd.lastKnown[0], 0, gd.lastKnown[2]] : [gd.char.pos[0], 0, gd.char.pos[2]];
  let P = gd.searchPlan;
  if (P && v3.distXZ(P.fix, fix) > 2.0) P = null;   // a new place to search: deal again round it
  if (!P) {
    P = { fix, t0: gd.lastKnown ? gd.fixT : g.time, serial: (gd.searchPlan?.serial ?? 0) + 1, builtAt: g.time, floodAt: g.time, spots: [], current: -1, phase: 'sweep', phaseT: 0, visits: 0, rr: 0, said: false };
    dealSpots(g, gd, P); gd.searchPlan = P;
  } else if (g.time - P.floodAt > SEARCH_REFLOOD_SECS) {   // time has passed, leaves have moved: how long Sam would need to each spot NOW
    const { secs } = floodFrom(g, P.fix, SEARCH_RADIUS_M); for (const s of P.spots) s.need = secs[s.cell]; P.floodAt = g.time;
  }
  return P;
}

/** This frame's light readings for a plan: his share of the frame's PROBE_BUDGET queued round-robin over the spots still worth asking about (all of them once
 *  those are done), and whatever answers have arrived folded in — each kept DARK_WINDOW_SECS, the darkest live one mapped through the meter's own curve. */
export function searchProbes(g: Game, gd: Guard, P: SearchPlan) {
  const n = P.spots.length; if (!n) return;
  let searchers = 0; for (const o of g.guards) if (o.state === 'search' && !o.script && o.searchPlan) searchers++;
  const share = Math.max(1, Math.floor(PROBE_BUDGET / Math.max(1, searchers)));
  const live = P.spots.some(s => !s.visited && !s.unreachable);
  for (let k = 0, tries = 0; k < share && g.searchProbeN < PROBE_BUDGET && tries < n; tries++) {
    const s = P.spots[P.rr % n]; P.rr++;
    if (live && (s.visited || s.unreachable)) continue;   // (the budget goes on the spots still in play while any are)
    g.lightQuery(s.key, [s.pos[0], PROBE_HEIGHT, s.pos[2]]); g.searchProbeN++; k++;
  }
  for (const s of P.spots) {
    const E = g.lightAt(s.key);
    if (E !== undefined && Number.isFinite(E)) { s.samples.push({ t: g.time, E }); g.lightForget(s.key); }
    while (s.samples.length && g.time - s.samples[0].t > DARK_WINDOW_SECS) s.samples.shift();
    if (s.samples.length > 200) s.samples.splice(0, s.samples.length - 200);
    if (s.samples.length) { let m = Infinity; for (const x of s.samples) if (x.E < m) m = x.E; s.E = m; s.dark = 1 - g.visibilityOf(m); }
    else { s.E = undefined; s.dark = 0.5; }
  }
}

/** Score every spot for THIS man now — dark × cover × plausible, eased toward the ones the shorter walk from him (`walk`: metres of his own path to each cell,
 *  from a keyed flood at pick time; straight-line otherwise — equal spots resolve near-first, a much darker one across the room still wins) — folding in what the
 *  other searchers know: a spot a colleague has looked into (or one within a metre of it) is done for everybody. */
export function scoreSpots(g: Game, gd: Guard, P: SearchPlan, walk: Float32Array | null = null) {
  const elapsed = g.time - P.t0; const others = g.guards.filter(o => o !== gd && o.state === 'search' && !o.script && o.searchPlan);
  for (const s of P.spots) {
    if (!s.visited) for (const o of others) { const v = o.searchPlan!.spots.find(q => q.visited && !q.unreachable && v3.distXZ(q.pos, s.pos) < 1.0); if (v) { s.visited = true; s.by = v.by; break; } }
    s.plaus = Number.isFinite(s.need) ? clamp(1 - (s.need - elapsed - PLAUS_SLACK_SECS) / PLAUS_RAMP_SECS, 0, 1) : 0;
    const m = walk ? walk[s.cell] : v3.distXZ(gd.char.pos, s.pos);
    s.score = Number.isFinite(m) ? Math.max(s.dark, 0.05) * s.cover * s.plaus / (1 + 0.08 * m) : 0;   // (no way there for him from here: nothing)
  }
}
/** a spot some other searcher is walking to or looking into right now (within a spot's spacing of it): his, not ours */
function spotTaken(g: Game, gd: Guard, s: SearchSpot): boolean {
  for (const o of g.guards) {
    if (o === gd || o.state !== 'search' || o.script || !o.searchPlan) continue;
    const Q = o.searchPlan; if (Q.current < 0 || (Q.phase !== 'walk' && Q.phase !== 'look')) continue;
    const q = Q.spots[Q.current]; if (q && v3.distXZ(q.pos, s.pos) < SEARCH_SPACING_M) return true;
  }
  return false;
}
/** On to the best spot still worth it (scored this frame), or the plan is done. */
function pickNextSpot(g: Game, gd: Guard, P: SearchPlan) {
  scoreSpots(g, gd, P, floodFrom(g, gd.char.pos, SEARCH_RADIUS_M * 2 + 4, true).metres);   // (his own walk to each, doors and all: round the partitions, not through them)
  let best = -1, bs = 0.004;   // (below that — lit open floor at the edge of plausibility — is not worth the walk)
  P.spots.forEach((s, i) => { if (s.visited || s.unreachable || s.plaus < 0.02 || s.score <= bs || spotTaken(g, gd, s)) return; bs = s.score; best = i; });
  P.current = best; P.phase = best >= 0 ? 'walk' : 'done'; P.phaseT = 0;
  gd.path = []; gd.pathGoal = null; gd.stallRef = null;
  if (best >= 0 && !P.said && !(gd.bubble && g.time - gd.bubble.t < 2.5) && !g.guards.some(o => o !== gd && o.state === 'search' && o.searchPlan?.said && g.time - o.searchPlan.builtAt < 30)) {   // (one man says it for a search, not each of them a second apart)
    P.said = true; g.say(gd, P.spots[best].dark > 0.6 ? "he'll have gone somewhere dark — check the corners" : P.spots.some(s => s.kind === 'prop') ? 'check the corners — under the desks, behind the chairs' : 'check the corners');
  }
}
/** One line for the panel / debugging: phase, the spot he is on, how many looked into, the top few by score with their readings. */
export function searchSummary(g: Game, gd: Guard): string {
  const P = gd.searchPlan; if (!P) return '';
  const cur = P.current >= 0 ? P.spots[P.current] : null;
  const top = [...P.spots].filter(s => !s.visited && !s.unreachable).sort((a, b) => b.score - a.score).slice(0, 3).map(s => `${s.kind}@${s.pos[0].toFixed(1)},${s.pos[2].toFixed(1)} d${s.dark.toFixed(2)}${s.E === undefined ? '?' : ''} c${s.cover.toFixed(2)} p${s.plaus.toFixed(2)} =${s.score.toFixed(3)}`).join(' · ');
  return `search ${P.phase}${cur ? ` → ${cur.kind}@${cur.pos[0].toFixed(1)},${cur.pos[2].toFixed(1)}` : ''} · ${P.visits}/${P.spots.length} looked into · ${Math.round(g.time - P.t0)} s since the fix · next: ${top || '—'}`;
}

// ------------------------------------------------------------ threshold discipline: pieing a doorway on the way through
// A hunting man (alert on a fix, or searching) whose path runs through a doorway does not jog through it square: two metres short he comes down to a walk and
// over toward the hinge side of the opening, his eyes and light slicing across the opening from what shows past the latch jamb round to the hinge side as he
// closes (the pie), and as he steps through he snaps seventy degrees to the corner behind the hinge jamb — the one place the slice could not see — then picks
// his pace back up. Not with eyes on Sam (no time for drills), not at the door he has just come through, not under the tour, not for a man pinned to his mark.
// No pathfinding of its own: it caps followPath's speed, bends the approach marks already on the path, and hands back a look point.
/** metres of path ahead within which a doorway is picked up; metres short of the crossing point at which the brake goes on (he is at a walk a stride later) and at
 *  which the slice starts; the slice's length; the corner check's; the same door is not pied again inside AGAIN seconds of going through it; a pie that has not
 *  completed in CAP seconds (wedged, shoved off his line) is dropped */
const PIE_DETECT_M = 3.2, PIE_SLOW_M = 1.5, PIE_SLICE_M = 1.7, PIE_SLICE_SECS = 1.0, PIE_CORNER_SECS = 0.4, PIE_AGAIN_SECS = 6, PIE_CAP_SECS = 7;
/** with a sighting fresher than this he is chasing, not clearing */
export const PIE_EYES_SECS = 1.0;
export interface Pieing {
  door: Door; phase: 'approach' | 'slice' | 'cross';
  side: 1 | -1;        // the face he comes at it from (+1 = the CCW side of the closed frame line, Doors.side's convention)
  at: Vec3;            // where his path cuts the frame line (distances are taken to this, not to the line: a man walking the corridor a metre off the wall is not a metre from the door)
  near: 0 | 1;         // the jamb on his side of the approach — 0 the hinge, 1 the latch — settled as the slice begins: the slice runs from it across to the far corner, the blind corner behind it gets the check
  t: number; t0: number; pathRef: Vec3[]; bent: boolean;
}
/** the closed frame of a door: hinge, unit along the frame hinge → latch, unit normal (+ = CCW side), width */
function frameOf(d: Door) { const ux = Math.cos(d.def.closedDir), uz = Math.sin(d.def.closedDir); return { hx: d.def.hinge[0], hz: d.def.hinge[1], ux, uz, nx: -uz, nz: ux, W: d.def.width }; }
/** where segment a→b crosses d's frame line within the opening: t along the segment, the side a is on, the crossing point — or null */
function crossesDoor(a: Vec3, b: Vec3, d: Door): { t: number; side: 1 | -1; at: Vec3 } | null {
  const F = frameOf(d);
  const da = (a[0] - F.hx) * F.nx + (a[2] - F.hz) * F.nz, db = (b[0] - F.hx) * F.nx + (b[2] - F.hz) * F.nz;
  if (da * db >= 0 || Math.abs(da - db) < 1e-6) return null;
  const t = da / (da - db); const ix = a[0] + (b[0] - a[0]) * t, iz = a[2] + (b[2] - a[2]) * t; const along = (ix - F.hx) * F.ux + (iz - F.hz) * F.uz;
  if (along < -0.1 || along > F.W + 0.1) return null;
  return { t, side: da > 0 ? 1 : -1, at: [ix, 0, iz] };
}
/** Doorways a hunting man goes through, pied or not: his stride since the last frame looked at cutting a frame line (the one behind him is then not pied again
 *  for PIE_AGAIN_SECS; a pie in progress on that door is on its corner check from here). Once a frame at most (updateGuard's alert branch and pieDoorway both ask). */
export function noteDoorCrossings(g: Game, gd: Guard) {
  const pos = gd.char.pos; if (gd.piePrevT === g.time) return;
  if (gd.piePrev && g.time - gd.piePrevT < 0.5 && v3.distXZ(gd.piePrev, pos) > 1e-4) for (const d of g.doors.list) if (crossesDoor(gd.piePrev, pos, d)) { gd.pieLast = d; gd.pieLastT = g.time; if (gd.pieing?.door === d && gd.pieing.phase !== 'cross') { gd.pieing.phase = 'cross'; gd.pieing.t = 0; } }
  gd.piePrev = v3.copy(pos); gd.piePrevT = g.time;
}
/** This frame's threshold discipline for a man followPath is about to move at `speed` (see the block comment): keeps Guard.pieing, notes the doorways he goes
 *  through. `eyesOn`: Sam in (or a second out of) his sight — no drill, he is chasing. Returns the speed to walk at and a look point (null = the caller's own). */
export function pieDoorway(g: Game, gd: Guard, dt: number, speed: number, eyesOn: boolean): { speed: number; look: Vec3 | null } {
  const pos = gd.char.pos;
  noteDoorCrossings(g, gd);
  if (!AI_TUNE.pieing || g.quietUtility || gd.pinned || eyesOn) { gd.pieing = null; return { speed, look: null }; }
  let P = gd.pieing;
  if (P && (g.time - P.t0 > PIE_CAP_SECS || (P.phase !== 'cross' && gd.path !== P.pathRef && !pathCrosses(gd, P.door)))) P = gd.pieing = null;   // stale, or re-planned some other way
  if (!P) {   // a doorway on the path within PIE_DETECT_M?
    let acc = 0, prev: Vec3 = pos;
    for (let i = gd.pathI; i < gd.path.length && acc < PIE_DETECT_M; i++) {
      const wp = gd.path[i]; const L = v3.distXZ(prev, wp);
      let hit: { d: Door; t: number; side: 1 | -1; at: Vec3 } | null = null;
      for (const d of g.doors.list) { if (d === gd.pieLast && g.time - gd.pieLastT < PIE_AGAIN_SECS) continue; const x = crossesDoor(prev, wp, d); if (x && (!hit || x.t < hit.t)) hit = { d, ...x }; }
      if (hit && acc + L * hit.t <= PIE_DETECT_M) { P = gd.pieing = { door: hit.d, phase: 'approach', side: hit.side, at: hit.at, near: 0, t: 0, t0: g.time, pathRef: gd.path, bent: false }; break; }
      acc += L; prev = wp;
    }
    if (!P) return { speed, look: null };
  }
  const d = P.door, F = frameOf(d), sg = P.side; const tx = -sg * F.nx, tz = -sg * F.nz;   // unit, the way through
  const perp = ((pos[0] - F.hx) * F.nx + (pos[2] - F.hz) * F.nz) * sg;                      // metres off the frame line on his side (− = through)
  const before = perp > 0 ? Math.max(perp, v3.distXZ(pos, P.at)) : perp;                     // metres still to walk to the sill — or, through, how far past the line
  if (!P.bent) { P.bent = true; bendApproach(g, gd, d, sg); P.pathRef = gd.path; refreshCrossing(gd, P); }
  P.t += dt;
  if (P.phase === 'approach' && before < PIE_SLICE_M) { P.phase = 'slice'; P.t = 0; const a = (pos[0] - F.hx) * F.ux + (pos[2] - F.hz) * F.uz; P.near = a > 0.6 * F.W ? 1 : 0; }   // (coming at it along the wall from the latch side, that jamb is the near one; head-on or from the hinge side — where the approach was bent to — the hinge is)
  if (P.phase === 'slice' && before < 0.15) { P.phase = 'cross'; P.t = 0; }
  if ((P.phase === 'cross' && P.t > PIE_CORNER_SECS) || before < -1.5 || before > PIE_DETECT_M + 1.5) { gd.pieLast = d; gd.pieLastT = g.time; gd.pieing = null; return { speed, look: null }; }
  let out = speed;
  if (before < PIE_SLOW_M + 0.3 && before > -0.1) { out = Math.min(speed, g.tune.guardWalk); if (gd.speed > out) gd.speed = damp(gd.speed, out, 14, dt); }   // the brake: a stride and he is at a walk (followPath's own easing would carry the jog to the sill); through, his feet pick up again under the corner check
  // the look pivots about the near jamb's edge: from just his side of straight-through (the jamb itself) round past the far jamb into the room as he closes (the
  // slice), then, stepping through, hard back round to the blind corner behind the near jamb — as bearings off the way through, so it sweeps whatever his line
  const jn = P.near ? F.W : 0, far = P.near ? -1 : 1;   // the near jamb's place along the frame; which way along it the far jamb lies
  const ex = F.hx + F.ux * jn, ez = F.hz + F.uz * jn;    // the pivot: the near jamb's edge
  const bearing = (deg: number, reach: number, y: number): Vec3 => { const a = deg * DEG, ca = Math.cos(a), sa = Math.sin(a) * far; return [ex + (tx * ca + F.ux * sa) * reach, y, ez + (tz * ca + F.uz * sa) * reach]; };   // deg: + toward the far side, − the near
  let look: Vec3 | null = null;
  if (P.phase === 'slice') look = bearing(lerp(-25, 80, smoothstep(0, 1, clamp(P.t / PIE_SLICE_SECS, 0, 1))), 2.5, 0.95);   // (held past the far jamb till he is on the sill)
  else if (P.phase === 'cross') look = bearing(-70, 1.6, 0.8);
  return { speed: out, look };
}
/** does the man's remaining path still run through this door */
function pathCrosses(gd: Guard, d: Door): boolean { let prev = gd.char.pos; for (let i = gd.pathI; i < gd.path.length; i++) { if (crossesDoor(prev, gd.path[i], d)) return true; prev = gd.path[i]; } return false; }
/** the crossing point again, off the path as it now runs (the approach marks have just been bent) */
function refreshCrossing(gd: Guard, P: Pieing) { let prev = gd.char.pos; for (let i = gd.pathI; i < gd.path.length; i++) { const x = crossesDoor(prev, gd.path[i], P.door); if (x) { P.at = x.at; return; } prev = gd.path[i]; } }
/** Bend the approach already on his path toward the hinge side: the last mark short of the line (the nav path's own square-on point through a two-cell doorway, or
 *  threadPath's) slides to 0.42 of the width from the hinge, and — unless he is coming along the wall from the latch side (no crossing in front of the opening
 *  to get there) and if the sill is still a couple of metres off — a mark 1.5 m out and a quarter-width from the hinge jamb goes in ahead of it, so he comes at
 *  the opening on the diagonal that shows him the latch side of the room first. Only onto free cells he can walk between. */
function bendApproach(g: Game, gd: Guard, d: Door, sg: 1 | -1) {
  const nav = g.col.nav, F = frameOf(d); const pos = gd.char.pos;
  const bef = (p: Vec3) => ((p[0] - F.hx) * F.nx + (p[2] - F.hz) * F.nz) * sg, alg = (p: Vec3) => (p[0] - F.hx) * F.ux + (p[2] - F.hz) * F.uz;
  const at = (a: number, b: number): Vec3 => [F.hx + F.ux * a + F.nx * sg * b, 0, F.hz + F.uz * a + F.nz * sg * b];
  let crossI = -1, X: Vec3 | null = null; { let prev = pos; for (let i = gd.pathI; i < gd.path.length; i++) { const x = crossesDoor(prev, gd.path[i], d); if (x) { crossI = i; X = x.at; break; } prev = gd.path[i]; } }
  if (crossI < 0 || !X) return;
  let nearI = -1;   // the near mark: the waypoint just short of the line, if it sits in front of the opening
  if (crossI - 1 >= gd.pathI) { const w = gd.path[crossI - 1]; const b = bef(w), a = alg(w); if (b > 0.3 && b < 1.4 && a > 0.15 * F.W && a < 0.9 * F.W) nearI = crossI - 1; }
  if (nearI >= 0) {
    const w = gd.path[nearI]; const q = at(0.42 * F.W, clamp(bef(w), 0.55, 0.8)); const from = nearI > gd.pathI ? gd.path[nearI - 1] : pos;
    if (!nav.isBlocked(q[0], q[2]) && nav.walkable(from, q)) gd.path[nearI] = q;
  }
  if (v3.distXZ(pos, X) > 2.1 && alg(pos) < 0.6 * F.W + 0.5) {   // the far mark (not for a man coming along the wall from the latch side: he pies from the line he is on)
    const q = at(0.25 * F.W, 1.5); const ins = nearI >= 0 ? nearI : crossI;
    const from = ins > gd.pathI ? gd.path[ins - 1] : pos, to = gd.path[ins];
    if (!nav.isBlocked(q[0], q[2]) && v3.distXZ(from, q) > 0.5 && nav.walkable(from, q) && (bef(to) < 0 || nav.walkable(q, to))) gd.path.splice(ins, 0, q);
  }
}

export function updateGuard(g: Game, gd: Guard, dt: number) {
  const c = gd.char; const T = g.tune; const route = g.level.routes[gd.routeI];
  playerGroundVel(g);   // (sampled once a frame whoever sees him or not, so the frame somebody does the difference is one frame's worth)
  if (gd.state === 'dead') {
    gd.muzzleDown = false; gd.pieing = null; gd.searchPlan = null; gd.held = null; gd.standoff = null; gd.speed = 0; c.vel = [0, 0, 0]; c.anim.speed = 0; c.update(dt); g.bodyThud(c); attachFlashlight(g, gd); return;   // (update = the ragdoll; free once it sleeps)
  }
  if (gd.held) {   // in Sam's arm (player.ts updateHold has already placed, turned and posed him this frame, and set his speed / vel to the pair's): no senses, no
                   // decisions, no feet of his own — the animator, the torch in his raised hand (its beam wherever updateHold pointed his aim) and his shuffling
                   // steps only. His state, awareness and fix stand exactly as they were: what he knew he still knows when he is let go (player.ts handBack).
    gd.pieing = null; gd.searchPlan = null; gd.bodyDuty = null; gd.standoff = null; gd.muzzleDown = false; gd.sawPlayerThisFrame = false; gd.path = []; gd.pathGoal = null; gd.stallRef = null;
    c.update(dt); attachFlashlight(g, gd);
    const prevPhase = gd.stepPhase; gd.stepPhase = c.anim.phase;
    if (gd.speed > 0.4 && ((prevPhase < 0.5) !== (gd.stepPhase < 0.5))) g.audio.footstep(c.pos, 0.3, false);   // (presentation only, like every guard's tread: the pair's noise is Sam's own step events)
    gd.lastPos = v3.copy(c.pos);
    return;
  }
  if (gd.script) { gd.pieing = null; gd.searchPlan = null; gd.standoff = null; gd.drawn = (gd.script.upper ?? 'aim') === 'aim'; runGuardScript(g, gd, dt); return; }   // choreographed (tour beat / squad drill): squad.ts owns him this frame — and what is in his hand follows the ordered pose ('aim' = the pistol), not the AI state; the drill has its own threshold work and its own search
  const lvl = escalationOf(g);   // the floor's alarm level (0 calm · 1 heightened · 2 lockdown; always 0 under the tour): how he carries himself while calm
  gd.drawn = lvl >= 1;           // heightened: the sidearm is out even on patrol — weapon light instead of the hand torch (handProps / attachFlashlight / killGuard)
  // --- perception ---
  let seen = false; let dist = 99;
  if (g.aiEnabled) {
    hearingCheck(g, gd);
    const vis = canSee(g, gd); dist = vis.dist;
    if (vis.visible) {
      const rate = sightGain(g, gd, dist);
      if (rate > 0.05) {   // a faint glimpse updates where to look — and which way he is actually going: displacement over the last sighting, not the pace he is asking of his legs (held by a leaf or a desk he 'sprints' on the spot) — only a real sighting refreshes the licence to shoot
        const pp = g.player.char.pos; const since = g.time - gd.sightT;
        gd.sightVel = since > 1e-4 && since < 0.1 ? [(pp[0] - gd.sightPos[0]) / since, 0, (pp[2] - gd.sightPos[2]) / since] : [0, 0, 0];
        gd.awareness = clamp(gd.awareness + rate * dt, 0, 1); gd.lastKnown = v3.copy(pp); gd.sightPos = v3.copy(pp); gd.sightT = g.time; seen = rate > 0.25; if (seen) gd.lastSeenT = g.time;
        gd.alertBest = 1e9; gd.alertStallT = 0; gd.alertRef = null; gd.chaseT = 0;   // (eyes on him: whatever chase follows starts its no-progress clocks from here, not from a stale count)
      }
    }
    // a colleague held in Sam's arm, in his view (standoffPerceive): he is in the standoff from this frame — and looking straight at Sam, whatever the meter says
    if (standoffPerceive(g, gd)) seen = true;
    gd.sawPlayerThisFrame = seen;
    // The frame he goes out of sight (had him a moment ago, not now) BEHIND something — the eye's line to where Sam now is has a wall or a leaf across it: through a
    // doorway, round a corner; not his own head sweeping off him, the range running out, a dazzle or the light dropping — lead the fix he leaves behind along the way
    // Sam was going (leadFix). Once per sighting, only a fresh one, and only if it still IS the sight fix (a noise placed since is its own place, not a point to march from).
    if (gd.sightT < g.time && gd.sightT > gd.ledT) {
      if (g.time - gd.sightT < 0.25 && gd.lastKnown && v3.distXZ(gd.lastKnown, gd.sightPos) < 0.01) {
        const eye = c.bones.head ?? v3.add(c.pos, [0, 1.65, 0]); const pc = g.player.char;
        if (g.col.segmentBlocked(eye, pc.bones.chest ?? v3.add(pc.pos, [0, 1.1, 0])) && g.col.segmentBlocked(eye, pc.bones.head ?? v3.add(pc.pos, [0, 1.6, 0]))) gd.lastKnown = leadFix(g, gd.sightPos, gd.sightVel);
      }
      gd.ledT = gd.sightT;
    }
    // a downed colleague in view → cover him and radio it in, the alarm goes up a step around where he lies, and the floor's short script for a found body begins
    // (bodyAftermath): the finder walks up short of him and covers, the others come over — the second man there gets the one line — and every one of them then
    // widens out to a search point away from the body instead of milling on it and talking himself calm over the corpse. (A short refractory per finder: two men
    // lying together are one 'man down', not two calls in two frames; a body already called in startles nobody — bodyInView skips it.)
    if (gd.state !== 'alert' && !gd.bodyDuty && g.time - gd.sawBodyT > 20) {
      const o = bodyInView(g, gd);
      if (o) {
        const body = bodyPos(o); const eye = gd.char.bones.head ?? v3.add(gd.char.pos, [0, 1.65, 0]);
        gd.sawBodyT = g.time; gd.awareness = Math.max(gd.awareness, 0.7); gd.lastKnown = v3.copy(body); gd.reactT = 1.2;
        gd.witness.sawBody = { victim: o.callsign, t: g.time, breathing: isBreathing(o) };   // he found him, and found him breathing or not
        const before = g.guards.filter(x => x.state === 'dead' && x.found).length;   // (bodies already called in earlier: the order's wording, raiseEscalation)
        let n = 0, cold = 0; for (const x of g.guards) if (x.state === 'dead' && !x.found && (x === o || (v3.distXZ(bodyPos(x), body) < 3 && !g.col.segmentBlocked(eye, v3.add(bodyPos(x), [0, 0.3, 0]))))) { x.found = true; tally(g, 'bodies'); n++; if (isBreathing(x)) cold++; }   // a body counts once, however many find it — and men lying together in his sight are found together (one call, one script, not a second 'man down' the moment the first one's ends; one hidden through the wall stays hidden)
        if (gd.state === 'patrol' || gd.state === 'search') { gd.state = 'suspicious'; gd.searchT = 0; gd.path = []; gd.pathGoal = null; }   // (a searcher too: he has something concrete to stand over now)
        // what he calls in says what he found: a man shot dead, or one put out cold and left breathing (isBreathing: the takedown) — the floor treats both alike, the words do not
        g.say(gd, n > 1 ? (cold === n ? `${n} men down — out cold, ${n > 2 ? 'all' : 'both'} breathing. someone's in here — radioing it in` : `${n} men down! — radioing it in`)
          : cold ? `it's ${o.callsign} — he's down, out cold but breathing. someone dropped him — radioing it in` : 'man down! — radioing it in', true); g.audio.play('radio', eye, 0.9);
        if (!g.quietUtility) {   // (the tour's staged bodies get the call and nothing more: its guards stay on their marks)
          g.alarm.pos[0] = body[0]; g.alarm.pos[1] = 0; g.alarm.pos[2] = body[2]; g.alarm.placed = true;
          raiseEscalation(g, lvl >= 1 || g.alarm.episode ? 2 : 1, gd, 'body', before > 0);   // a body is as real as an alarm gets: a calm floor goes to heightened on it, an alarmed one (or one still hunting somebody) to lockdown, held around where he lies, its pair sent round the rooms nearest him (somebody else gives the order so the finder's own line stays up) — first, so a pair it pulls off a clear hears the call below as fresh men
          gd.task = null; gd.bodyDuty = { body: o, role: 'finder', stage: 'to', t: 0, t0: g.time, spot: null };   // (a colleague on the floor outranks the breaker / the rack he was walking to)
        }
        for (const other of g.guards) if (other !== gd && other.state !== 'dead') {
          other.awareness = Math.max(other.awareness, 0.45);
          other.witness.calledToBody = { victim: o.callsign, t: g.time, breathing: isBreathing(o) };   // everybody on the net heard the call, and heard what was found (errand or not, sent over or not)
          if (other.task) continue;   // on an errand (the breaker, the rack): he hears it on the net, sharpens up, and keeps going — his fix stays the errand's
          other.lastKnown = v3.copy(body);
          if (!g.quietUtility && !other.hold && !other.held && !other.bodyDuty && (other.state === 'patrol' || other.state === 'suspicious' || !!other.script)) other.bodyDuty = { body: o, role: 'called', stage: 'to', t: 0, t0: g.time, spot: null };   // (a scripted man — the clearing pair — breaks off on the news and comes over like the rest; an alert or searching one keeps hunting, he only learns where; the man in Sam's arm goes nowhere)
        }
      }
    }
    // lockdown pair: the man he is trailing stops dead and brings his weapon up at something ('huh?') — he takes it exactly as seriously, a step to that man's
    // side of the spot, without a 'huh?' of his own (only while the leader's start is fresh, reactT > 0: one look per noise, not a loop of re-inheriting it)
    if (lvl >= 2 && gd.leader && gd.state === 'patrol' && !gd.hold && gd.leader.state === 'suspicious' && gd.leader.reactT > 0 && gd.leader.lastKnown && !gd.leader.task) {   // (not an errand's start: a man setting off for the breaker or round a silent colleague's route was not startled by anything — his no. 2 just trails him there)
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
  if (gd.awareness >= 0.85 && gd.state !== 'alert') {
    resetChase(gd); gd.state = 'alert'; gd.reactT = Math.max(gd.reactT, 0.35); gd.fireCd = 0.6 + Math.random() * 0.4; if (!gd.standoff && !(gd.bubble?.radio && g.time - gd.bubble.t < 1.0)) g.say(gd, 'CONTACT!'); g.audio.play('radio', c.bones.head ?? c.pos, 0.8); if (g.time - g.lastStingT > 8) { g.audio.play('alertSting', null, 0.75); g.lastStingT = g.time; }   // (a man walking in on the hold has said what he saw — the call, or his first line to Sam — not 'contact'; nor does the man giving the order this second shout it over his own radio line)
    if (g.aiEnabled) { gd.witness.alertedBy = alertCause(g, gd, seen); gd.witness.alertT = g.time; }   // and he remembers what it was that did it (nothing is remembered with the AI off: a sandbox poke is nobody's perception)
  }
  if (g.aiEnabled && gd.awareness > gd.witness.peakAwareness) gd.witness.peakAwareness = gd.awareness;
  else if (gd.awareness >= 0.3 && gd.state === 'patrol') {
    gd.state = 'suspicious'; gd.searchT = 0;
    if (gd.bodyDuty) gd.reactT = 0.5 + Math.random() * 0.5;   // called to a body over the net: no 'huh?' — a beat to take it in, then over to him (bodyAftermath)
    else { gd.reactT = 0.9; g.say(gd, 'huh?'); g.audio.play('radio', c.bones.head ?? c.pos, 0.45, { rate: 1.15 }); }
  }
  if (prev !== gd.state) { gd.path = []; gd.pathGoal = null; gd.stallRef = null; if (gd.state !== 'suspicious') gd.bodyDuty = null; }   // (the body script lives inside 'suspicious': contact or calm ends it)
  if (gd.state !== 'search') gd.searchPlan = null;   // (the plan lives inside 'search': whatever ends the search — contact, a body, calm, a script — drops it, and the next search deals afresh)
  const pair = standoffUpkeep(g, gd);   // the standoff he is in, if it still is one (the hold on, this pair, seen lately, he alert and free): the alert branch plays it out; else the record went just now
  gd.trackFix(g.time);   // when the fix he works from was placed (the search asks how far Sam can have got since)
  const dazzled = g.time < gd.dazzledUntil;
  if (dazzled) gd.reactT = Math.max(gd.reactT, gd.dazzledUntil - g.time);   // the transitions above reset reactT; a dazzled guard stays rooted (suspicious / alert both hold still while reactT > 0)
  // (alarm episodes are counted in updateMission: someone going to alert while nobody was)       // mission stats: every time a man goes to alert (from patrol, suspicion or a lapsed search alike)

  // --- behaviour ---
  c.anim.lookYawExtra = 0; gd.lookPhase += dt;
  let moveSpeed = 0; let upper: 'none' | 'relaxed' | 'aim' = 'none'; let aimAt: Vec3 | null = null;
  let pied = false;   // a branch that walks a hunting path through doorways runs pieDoorway itself; every other leaves no pie standing (below the switch)
  switch (gd.state) {
    case 'patrol': {
      // heightened (lvl ≥ 1): the pistol is out at a low ready instead of the torch, the route is walked ~15 % brisker with shorter pauses and a wider sweep of the
      // muzzle into the corners; lockdown (2): a paired follower trails his leader instead of walking his own route, the spare man holds his junction (updateEscalation
      // hands those out), and anyone passing an open or unlocked door deals with it
      const target = route.points[gd.wp];
      if (gd.drawn) upper = 'aim';
      if (gd.bodyDuty && gd.awareness < 0.3) gd.bodyDuty = null;   // the call never took (the AI was off, he was reset under it): it must not steer some later, unrelated suspicion to an old body
      if (gd.task?.kind === 'checkOn') gd.task = null;              // (likewise a route walk he was talked out of under AI-off: a man back on his own route is not on that errand — the next radio check sends somebody afresh)
      const walk = T.guardWalk * (lvl >= 1 ? 1.15 : 1);
      if (gd.hold) { gd.sweep = damp(gd.sweep, Math.sin(gd.lookPhase * 0.5) * 0.2, 2, dt); c.vel = [0, 0, 0]; }
      else if (lvl >= 2 && gd.leader) { partnerWatch(g, gd); if (followLeader(g, gd, dt, walk)) moveSpeed = gd.speed; }   // (a leader taken down quietly is still 'his leader' until somebody knows — superviseLockdown — so he closes up to his station behind the body, and finds him; one who has simply vanished from his sight for good gets called, and reported: partnerWatch)
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
      if (gd.task?.kind === 'checkOn') checkOnTick(g, gd);   // walking a silent colleague's route: on to his next waypoint after a look round each — or over (found since, reset, walked it all: 'not at his post')
      if (gd.task) { // errand (e.g. go and look at the breaker): don't talk yourself out of it halfway down the corridor
        if (!gd.lastKnown || v3.dist(gd.lastKnown, gd.task.pos) > 0.5) { if (gd.awareness < 0.45) gd.lastKnown = v3.copy(gd.task.pos); }
        gd.awareness = Math.max(gd.awareness, 0.2);
        if (gd.task.kind !== 'checkOn' && gd.searchT > 4) gd.task = null;   // (the route walk ends on its own count of waypoints, not on one look round)
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
          if ((!gd.task || gd.task.kind === 'checkOn') && notGettingAnywhere(gd, dt, 4)) {   // (the breaker / rack errands are worth grinding toward, and their block above puts the fix straight back)
            if (gd.task) advanceCheckOn(g, gd);   // a waypoint of the missing man's he cannot get to counts as looked at: the next
            else { gd.lastKnown = v3.copy(c.pos); gd.stallRef = null; gd.path = []; gd.pathGoal = null; }
          }
        }
      }
      if (gd.awareness < 0.12) {   // talked himself calm. The line depends on what it was: over a found body one man closes it for everybody, and nobody within a few metres of a
        const D = gd.bodyDuty;     // body the floor knows about ever calls it 'nothing' / 'clear' (that read as the men cycling platitudes over a dead colleague); once alarmed, nothing is 'nothing'
        gd.state = 'patrol'; gd.path = []; gd.pathGoal = null; gd.bodyDuty = null;
        if (g.player.down) { if (!g.stoodDown) { g.stoodDown = true; g.say(gd, 'back to your posts', true); } }
        else if (D) { if (!D.body.bodyClosed && D.body.bodyVisits > 0 && v3.distXZ(c.pos, bodyPos(D.body)) < 12) { D.body.bodyClosed = true; g.say(gd, isBreathing(D.body) ? `no sign of him. ${D.body.callsign} is breathing, he'll keep — whoever dropped him is still close` : "no sign of him — eyes open, he's close"); } }   // (once, by a man thereabouts, and only if anybody got to the body at all — a called man giving up two rooms off says nothing; over a man out cold it says so)
        else if (!nearKnownBody(g, c.pos, 4) && !(gd.bubble?.radio && g.time - gd.bubble.t < 4)) g.say(gd, lvl >= 1 ? 'clear here — staying on it' : 'must have been nothing');   // (and not on top of his own radio line of a moment ago — 'not at his post', 'checking the breaker': that said it)
      }
      c.aimPitch = damp(c.aimPitch, -0.12, 3, dt);
      break;
    }
    case 'alert': {
      upper = 'aim';
      noteDoorCrossings(g, gd);   // (a doorway run through with eyes on Sam is still the one he has just come through, should he lose him beyond it and turn back)
      const pc = g.player.char; const canShoot = !g.player.down && !dazzled && (g.time - gd.lastSeenT < 0.5);   // half a second of grace after a real sighting; a dazzled guard cannot shoot at all
      if (gd.reactT > 0 || !canShoot) { gd.shotBlockedT = 0; gd.lineInT = 0; }   // (the blocked-line clocks belong to one engagement: a fresh sighting at another doorway starts by holding the round again, not on the last one's count)
      if (gd.reactT > 0 || g.player.down || gd.pinned) { gd.stallRef = null; gd.chaseT = 0; }   // (seconds spent covering, reacting or holding a post are not seconds of getting nowhere; eyes on him resets the clocks in the perception above — unless his line is blocked and there is nowhere to step, below, when they must run)
      if (gd.reactT > 0) { gd.reactT -= dt; aimAt = (dazzled || g.time - gd.lastSeenT > 0.5) && gd.lastKnown ? v3.add(gd.lastKnown, [0, 1.1, 0]) : v3.add(pc.pos, [0, 1.2, 0]); gd.speed = damp(gd.speed, 0, 8, dt); }   // heard, not seen: he covers where he thinks it came from, not your true position through three walls   // a dazzled guard covers where he last knew you were, not where you are
      else if (gd.standoff && pair) { const r = standoffAlert(g, gd, dt, pair); moveSpeed = r.move; aimAt = r.aim; gd.shotBlockedT = 0; gd.lineInT = 0; }   // a colleague in Sam's arm, in his view: the ring, the flank, the slow aimed round, the talking — never the push (standoffAlert)
      else if (canShoot) {
        // Where the rounds go: the chest (lower when crouched) — or, when something solid stands between his GUN and it that his eye clears (canSee said yes: the
        // jamb on his gun-hand side, a leaf standing between them, a mullion, the desk or partition Sam is hull-down behind), the head if that can be reached; and
        // when neither can from where he stands, he closes along the path — through the doorway, round the desk — until one can, instead of planting on his line
        // of SIGHT and emptying magazines into the door frame (the 'aim stuck in the door' of the playtest: eye line a hand inside the jamb, muzzle line on it, a
        // man rooted at the doorway firing at wood for as long as Sam stood there). He never fires a line he knows is blocked. A pinned man (the tour's firefight)
        // fires from where he was put at what he was always given, cover or no cover — that beat is built on rounds meeting the end caps.
        aimAt = v3.add(pc.pos, [0, g.player.crouch ? 0.8 : 1.25, 0]);
        let blocked = false;
        if (!gd.pinned && !clearShot(g, c, aimAt)) {
          const head = pc.bones.head ?? v3.add(pc.pos, [0, g.player.crouch ? 1.05 : 1.6, 0]);
          if (clearShot(g, c, head)) aimAt = head; else blocked = true;
        }
        const facingErr = Math.abs(wrapAngle(Math.atan2(aimAt[0] - c.pos[0], aimAt[2] - c.pos[2]) - c.aimYaw));
        const poked = !gd.pinned && armThrough(g, c);   // his pistol out through the far face of the leaf he is pressed to, or into the jamb (fireWeapon puts that round into the wood): never fired, and once his aim has settled it counts as no line from here — he steps clear
        if (poked && facingErr < 0.3) blocked = true;
        gd.shotBlockedT = blocked ? gd.shotBlockedT + dt : 0;
        const inLine = muzzleCheck(g, gd, aimAt, dt);   // a colleague between his gun and Sam (or just beyond him): the pistol goes to the low ready and stays there while he is in the line — worked out here, before the round, on the line he means to fire
        const crossingMan = !!inLine && Math.hypot(inLine.char.vel[0], inLine.char.vel[2]) > 0.4;
        gd.lineInT = inLine ? gd.lineInT + dt : 0;
        // keep some distance, otherwise hold position and fire (a pinned guard fires from where he was put) — unless nothing fired from here can reach him: then he
        // closes in along the path (to Sam himself: it threads the doorway square-on and clears the leaf) and plants again the moment a line opens. Blocked with
        // no way of closing at all (in plain sight somewhere no route reaches — a slot no man fits): he holds the round and the chase clocks run, and after a few
        // seconds of it he lets the shot go and searches from where he is rather than stand at '!' on a line he cannot use. A colleague in the line and nothing
        // else keeping the round in: a man walking across it is waited out for a beat (he will be through), one standing in it — or still in it after the beat —
        // is stepped round to the side that opens the line (lineStep), and where no step can open it he pushes up the path past the man: a fight never stalls on manners.
        let closing = false, stepping = false;
        if (!gd.pinned && (dist > 9 || blocked)) { goTo(g, gd, pc.pos); closing = !followPath(g, gd, dt, T.guardChase); moveSpeed = gd.speed; }
        else if (!gd.pinned && gd.muzzleDown && !(crossingMan && gd.lineInT < 0.45)) {
          const st = lineStep(g, gd, aimAt, inLine, dt);
          if (st === 'step') { stepping = true; moveSpeed = gd.speed; }
          else if (st === 'none' && dist > LINE_PUSH_MIN_M) { goTo(g, gd, pc.pos); followPath(g, gd, dt, T.guardInvestigate); moveSpeed = gd.speed; }   // nowhere to step that opens it (the corridor too narrow for the angle): up the path past the man until it does
          else { gd.speed = damp(gd.speed, 0, 6, dt); c.vel = [0, 0, 0]; }
        }
        else { gd.speed = damp(gd.speed, 0, 6, dt); c.vel = [0, 0, 0]; }
        if (!(blocked && !closing)) { if (!stepping) gd.stallRef = null; gd.chaseT = 0; }   // (the side-step keeps its own no-headway watch on stallRef)
        else if (gd.shotBlockedT > 4) { dropToSearch(g, gd, 'no shot — lost him'); gd.awareness = Math.min(gd.awareness, 0.5); break; }
        gd.fireCd -= dt;
        const twisted = moveSpeed > 0.3 && Math.abs(wrapAngle(c.bodyYaw - c.aimYaw)) > 0.9;   // legs following a path that has turned away from Sam (back through a doorway to come at him): he keeps moving and holds the round rather than fire across his own back
        if (gd.reloadT >= 0) { gd.reloadT -= dt; if (gd.reloadT < 0) { gd.shots = 0; } }
        else if (gd.fireCd <= 0 && facingErr < 0.25 && !blocked && !twisted && !poked && !gd.muzzleDown && !roundPastColleague(g, gd, aimAt)) {   // (muzzleDown: the pistol is not even up — a colleague is, or a beat ago was, in the line; roundPastColleague: the bore's own line this frame, the last look in a scrum)
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
        goTo(g, gd, gd.lastKnown);
        const pie = pieDoorway(g, gd, dt, T.guardChase, g.time - gd.lastSeenT < PIE_EYES_SECS); pied = true;   // a doorway on the way to the fix: down to a walk, slice it, check the corner as he goes through (not with Sam a second out of his sight: then he is chasing)
        let arrived = followPath(g, gd, dt, pie.speed); moveSpeed = gd.speed;
        aimAt = arrived ? null : pie.look ?? v3.add(gd.lastKnown, [0, 1.1, 0]);
        // no way of getting nearer (a leaf across an alley, a carton shoved over the spot, a fix inside a wall): after a few seconds of no progress he searches
        // from where he is rather than staying 'alert' for ever — which also kept the whole floor's alarm episode (and its step-down) from ever ending.
        // Progress is measured against THIS fix: a new one (a colleague's shots placing Sam across the floor, a glimpse of him two rooms on) starts the count
        // again — measured against the old one, the run to it read as 'five seconds without getting nearer than he once was' and he gave up half way down the
        // corridor — and four seconds of covering no ground at all (wedged on a leaf somebody holds shut, a colleague planted in the doorway) ends it too, whatever
        // the distance to the fix is doing; and no chase runs past 25 s without a sight of him or a new fix, however long the path (a fix that can be walked to
        // is walked to well inside that; one that keeps a man marching for half a minute is not worth the floor's alarm staying up for).
        if (!gd.alertRef || v3.distXZ(gd.alertRef, gd.lastKnown) > 0.05) { gd.alertRef = v3.copy(gd.lastKnown); gd.alertBest = 1e9; gd.alertStallT = 0; gd.chaseT = 0; }   // (the ground watchdog's own reference is left alone: a fix that keeps moving must not keep a wedged man from ever timing out)
        const dFix = v3.distXZ(gd.lastKnown, c.pos);
        if (dFix < gd.alertBest - 0.5) { gd.alertBest = dFix; gd.alertStallT = 0; } else if ((gd.alertStallT += dt) > 5) arrived = true;
        if (!arrived && notGettingAnywhere(gd, dt, 4)) arrived = true;
        if (!arrived && (gd.chaseT += dt) > 25) arrived = true;
        if (arrived) dropToSearch(g, gd, 'where did he go?');
      } else if (!gd.pinned && !gd.hold && g.time - gd.lastSeenT > 2) dropToSearch(g, gd, null);   // alert with nothing to go on and nobody's orders to hold: everything that takes awareness this high leaves a fix, so this should not happen — but a free man must never be able to stand at '!' for good (a pinned / held one is standing where the beat put him)
      break;
    }
    case 'search': {
      upper = 'aim';
      if (g.player.down) { gd.state = 'alert'; gd.reactT = 0; gd.searchT = 0; gd.searchPlan = null; break; }   // a body on the floor is not a search: go and cover it (alert branch above)
      gd.searchT += dt; gd.sweep = Math.sin(gd.lookPhase * 1.2) * 1.3; c.anim.lookYawExtra = Math.sin(gd.lookPhase * 2.1) * 0.45;
      let sweepK = 1;   // the wide sweep of eyes and muzzle while he stands (half as wide on the move) — put on his aim below unless something this frame gives him a point to hold (a spot, a doorway's slice)
      const P = AI_TUNE.darkSearch ? searchPlanFor(g, gd) : null;   // the hide spots round the fix, dark × cover × plausible (see searchPlanFor); null = the old sweep-and-wander (QA before / after)
      let over = false;
      if (!P || P.phase !== 'walk' || dazzled) gd.speed = damp(gd.speed, 0, 6, dt);   // (the old wander ambles against this brake, as it always did; the walk to a spot is a purposeful one at the investigate pace)
      if (!P) {
        // wander to a couple of nearby points (not while dazzled: a blind man does not go for a walk)
        if (!dazzled && gd.searchT > 3 && gd.lastKnown && (!gd.path.length || gd.pathI >= gd.path.length)) {
          const a = Math.random() * Math.PI * 2; const cand: Vec3 = [gd.lastKnown[0] + Math.cos(a) * 3, 0, gd.lastKnown[2] + Math.sin(a) * 3];
          if (!g.col.nav.isBlocked(cand[0], cand[2])) { goTo(g, gd, cand, true); gd.stallRef = null; }
        }
        if (!dazzled && gd.path.length && gd.pathI < gd.path.length) {
          const pie = pieDoorway(g, gd, dt, T.guardInvestigate, false); pied = true;
          followPath(g, gd, dt, pie.speed); moveSpeed = gd.speed; sweepK = 0.5; if (pie.look) { aimAt = pie.look; c.anim.lookYawExtra = 0; }
          // a wander point he cannot actually get to (behind a leaf standing at its stop, say): three seconds of no headway and he drops it and sweeps from where he is
          if (notGettingAnywhere(gd, dt, 3)) { gd.path = []; gd.pathGoal = null; gd.stallRef = null; }
        }
        over = gd.searchT > SEARCH_MIN_SECS;
      } else {
        searchProbes(g, gd, P);   // his share of this frame's light readings: asked of the renderer, and the answers that have come back folded in
        if (dazzled) { /* rooted and sweeping (above): a blind man does not go for a walk */ }
        else if (P.phase === 'sweep') { if (gd.searchT > SEARCH_SWEEP0_SECS) pickNextSpot(g, gd, P); }   // the first look round from where he lost him — and a second's readings in hand before choosing
        else if (P.phase === 'walk') {
          const s = P.spots[P.current];
          goTo(g, gd, s.pos);
          const pie = pieDoorway(g, gd, dt, T.guardInvestigate, false); pied = true;   // a doorway between him and the spot gets sliced on the way
          const arrived = followPath(g, gd, dt, pie.speed); moveSpeed = gd.speed; sweepK = 0.5;
          const dS = v3.distXZ(c.pos, s.pos); const eye = c.bones.head ?? v3.add(c.pos, [0, 1.65, 0]);
          const inSight = dS < 4.5 && !g.col.segmentBlocked(eye, [s.pos[0], 0.4, s.pos[2]]);
          if (pie.look) { aimAt = pie.look; c.anim.lookYawExtra = 0; }
          else if (inSight) { aimAt = [s.pos[0], 0.35, s.pos[2]]; c.anim.lookYawExtra = 0; }   // coming up on it: the light goes down INTO the well / the corner ahead of his feet
          if ((inSight && dS < SEARCH_LOOK_DIST_M) || arrived) { P.phase = 'look'; P.phaseT = 0; gd.path = []; gd.pathGoal = null; gd.stallRef = null; }   // at the mouth of it (or as near as the floor lets him)
          else if (notGettingAnywhere(gd, dt, 3)) { s.unreachable = true; s.by = c.id; pickNextSpot(g, gd, P); }   // three seconds of no headway (a leaf held against him, a colleague planted in the gap): that one is as looked-at as it will get
        }
        else if (P.phase === 'look') {   // planted, torch held into the spot low, a slow inch across it; then it is done — for everybody searching — and on to the next
          const s = P.spots[P.current]; P.phaseT += dt;
          gd.speed = damp(gd.speed, 0, 8, dt); c.vel = [0, 0, 0]; c.anim.lookYawExtra = 0;
          const w = Math.sin(P.phaseT * 4.0) * 0.35; const to = v3.sub(s.pos, c.pos); const L = Math.hypot(to[0], to[2]) || 1;
          aimAt = [s.pos[0] - to[2] / L * w, 0.3, s.pos[2] + to[0] / L * w];
          if (P.phaseT > SEARCH_LOOK_SECS) { s.visited = true; s.by = c.id; P.visits++; pickNextSpot(g, gd, P); }
        }
        // 'done': nothing left worth the walk — he sweeps from where he stands (above) until the search's clock lets him go
        over = gd.searchT > SEARCH_MIN_SECS && (P.phase === 'done' || gd.searchT > SEARCH_CAP_SECS);
        if (over) gd.lastKnown = v3.copy(c.pos);   // the look round that follows ('suspicious' for a couple of seconds) happens where the search ended, not back at a fix he has just combed round
      }
      if (over) { gd.state = 'suspicious'; gd.awareness = 0.25; gd.searchT = 0; gd.reactT = 0; gd.searchPlan = null; g.say(gd, 'returning to patrol'); }
      if (!aimAt) { c.aimYaw = c.bodyYaw + gd.sweep * sweepK; c.aimPitch = damp(c.aimPitch, -0.1, 3, dt); }   // (a point to hold goes through the aim easing below instead — the sweep written over it every frame would have the torch waving past the spot he means to look into)
      break;
    }
  }
  if (!pied || (gd.state !== 'alert' && gd.state !== 'search')) gd.pieing = null;   // (only a hunting walk pies a doorway; anything else this frame — contact, a body, calm, orders, the search ending under it — leaves none half done)
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
  // muzzle discipline: with the pistol up (every armed state — the heightened patrol's low carry, a fix being covered, the search's sweep), a living colleague in
  // the line he means it along puts it to the low ready until he is out of it (the alert branch has already asked, on the line of the shot itself)
  if (gd.muzzleEvalT !== g.time) muzzleCheck(g, gd, upper === 'aim' ? (aimAt ?? aimAhead(c)) : null, dt);
  c.anim.speed = moveSpeed; c.anim.upper = gd.muzzleDown ? 'none' : upper; c.anim.crouchTarget = 0; c.anim.stance = gd.muzzleDown ? 'lowReady' : 'none';   // (the one tactical stance the AI holds by itself: muzzle down for a colleague — otherwise none, and a script that just ended must not leave one on him)
  c.anim.reverse = moveSpeed > 0.2 && c.vel[0] * Math.sin(c.bodyYaw) + c.vel[2] * Math.cos(c.bodyYaw) < -0.3 * moveSpeed;   // moving against his facing (the standoff's back-off and side-step round the ring keep his chest to the pair): the cycle plays backwards, as the player's back-pedal does — and worked out afresh every frame, so nothing (a hold that ended mid-haul) leaves it stuck on him
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

/** A guard says something: a bubble over his head if he is on screen, otherwise (or if it went out over the radio) a line in the log. A man held in Sam's arm has
 *  no voice of his own — whatever the floor's logic would have him bark or radio is dropped (the callers that matter pick somebody else: spokesman, radioCaller,
 *  pickChecker skip him) — except his answers to Sam, which come `force`d (Game.interrogate) and go to the log under his name. */
export function say(g: Game, gd: Guard, text: string, radio = false, force = false) {
  if (gd.held && !force) return;
  gd.bubble = { text, t: g.time, dur: Math.min(4, Math.max(2, 1.5 + text.length * 0.055)), radio };
  if (radio || !onScreen(g, gd.char.bones.head ?? v3.add(gd.char.pos, [0, 1.7, 0]))) g.msg((radio ? 'radio: ' : force ? `${gd.callsign}: ` : 'guard: ') + text);
}

/** Fresh guards at the start of their routes (keeps their flashlight lights and tints). */
export function resetGuards(g: Game) {
  if (g.player) { g.player.dragging = null; g.player.takedown = null; g.player.holding = null; g.player.guardHold = 0; g.player.char.anim.holdPose = null; }   // the old bodies / targets are gone — the man in his arm with them
  g.guards = g.guards.map(gd => new Guard(g.ch, gd.char.id, g.level.routes[gd.routeI], gd.routeI, gd.flashlight, gd.char.tint));
  for (const gd of g.guards) gd.char.update(0);          // fresh characters get a pose (bones) immediately — reset can happen mid-frame
  g.breakerFixT = 0; g.aimGuard = null;
  g.escalation = 0; g.escalationT = -1; g.alarm.episode = false; g.alarm.placed = false; g.alarm.apartT = 0;   // fresh men have no memory of an alarm: calm floor, torches out (the pairing / posts lived on the old Guard objects)
  g.clearing = null; g.clearedAt.clear();   // (so did the scripts of a pair that was clearing rooms, and their ledger of rooms done)
  const R = rollcallState(g); R.stage = 'idle'; R.nextAt = -1; R.caller = null; R.missing = null;   // and the net's radio check starts its clock again with them
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
  const R = rollcallState(g);   // the net's radio check: what it is doing, or when the next is due
  if (R.stage !== 'idle') s += ` · radio check: ${R.stage === 'called' ? 'counting in' : `asking after ${R.missing?.callsign ?? '?'}`}`; else if (R.nextAt >= 0 && !g.quietUtility) s += ` · radio check in ${Math.max(0, Math.ceil(R.nextAt - g.time))} s`;
  for (const gd of g.guards) if (gd.state !== 'dead' && gd.task?.kind === 'checkOn') s += ` · ${name(gd)} walks ${gd.task.who.callsign}'s route (${gd.task.left} to go)`;
  return s;
}
/** The second panel line: what the pair is clearing right now (squad.ts Clearing.summary), or nothing. */
export function clearingSummary(g: Game): string { return g.clearing?.summary() ?? ''; }

/** the living man best placed to radio something about `pos` (or about nothing in particular): the nearest who is not mid-fight, else the nearest at all — never `except`
 *  (somebody whose own line should stay on screen: a finder left alone on the floor radios his 'man down' and nobody answers with orders to himself) */
function spokesman(g: Game, pos: Vec3 | null, except: Guard | null = null): Guard | null {
  let best: Guard | null = null, bd = Infinity;
  for (let pass = 0; pass < 2 && !best; pass++) for (const gd of g.guards) {
    if (offNet(gd) || gd === except || (pass === 0 && gd.state === 'alert')) continue;   // (a man with a forearm across his throat radios nothing)
    const d = pos ? v3.dist(gd.char.pos, pos) : 0; if (d < bd) { bd = d; best = gd; }
  }
  return best;
}

/** Raise the floor to at least `to` and (re)arm that level's clock. One radio line per actual change (a lockdown re-armed by yet another alarm just says so), from the
 *  calm man nearest the last fix — `except` the one whose own line should stay up; `cause` picks the wording for a first raise (a search that found nobody, a
 *  body, a man who neither answers the net nor is anywhere on his route, or a colleague seen held in Sam's arm — the standoff's call). A lockdown deals the duties
 *  out at once so the post man's own line follows the order; for a hostage no rooms are dealt for clearing (they know exactly where he is — the pair is what
 *  everybody converges on; the alarm coming to nothing later plans the sweep as ever). */
export function raiseEscalation(g: Game, to: 1 | 2, except: Guard | null = null, cause: 'search' | 'body' | 'missing' | 'hostage' = 'search', another = false) {   // (another: bodies had been called in before this one — the order says so)
  const prev = g.escalation; const lvl: 0 | 1 | 2 = to > prev ? to : prev;
  g.escalation = lvl; g.escalationT = g.time + (lvl >= 2 ? LOCKDOWN_SECS : HEIGHTENED_SECS);
  const again = g.time - g.alarm.raisedAt < 1.5; g.alarm.raisedAt = g.time;
  if (again && lvl <= prev && cause !== 'body') return;   // just raised (a searcher finding a body ends his hunt, and with it the episode, in the very frame the body raised the floor): the clock is re-armed, nothing else is new — no 'still nothing' on top of 'man down', no re-deal (a body always re-plans: the rooms round HIM)
  const sp = spokesman(g, g.alarm.placed ? g.alarm.pos : null, except);
  const line = lvl <= prev ? (cause === 'body' ? `${another ? 'another man down' : 'man down confirmed'} — stay locked down, clear everything round him` : cause === 'missing' ? "and a man we can't raise — it stays locked down, find him" : cause === 'hostage' ? 'copy — he has one of ours. it stays locked down: get round him, nobody fires till he has him clean' : 'still nothing — it stays locked down')
    : lvl >= 2 ? (cause === 'hostage' ? 'he has one of ours — lock it down: weapons out, everyone to him, box him in and hold your fire' : `${cause === 'body' ? (another ? 'another man down — ' : 'man down — ') : cause === 'missing' ? "a man missing on top of it — " : ''}lock it down: pairs, weapons out, nobody wanders`)
    : cause === 'body' ? 'we have a man down — weapons out, everyone sharp' : cause === 'missing' ? "a man off the net and not on his route — weapons out, eyes open for him, treat it as real" : 'nothing here — stay sharp, check your corners';
  if (sp) { g.say(sp, line, true); g.audio.play('radio', sp.char.bones.head ?? sp.char.pos, 0.7); }
  if (lvl >= 2) { assignLockdown(g, prev >= 2, except); if (cause !== 'hostage') planClears(g, false, cause); }   // (re-dealt on a re-arm too: they have just converged on a new fix, so pair whoever is together now and post the spare man by IT) — and the pair clears the rooms nearest that fix (more of them, and whatever they were on dropped, for a body)
}

/** Panel / debug: the floor calm again at once — level 0, clock off, pair and post dissolved, a clear in progress called off — without touching the men themselves (whoever is suspicious stays so). */
export function standDownEscalation(g: Game) { g.escalation = 0; g.escalationT = -1; g.alarm.episode = false; g.alarm.placed = false; g.clearing?.cancel(); g.clearing = null; g.clearedAt.clear(); clearLockdown(g); }

// ------------------------------------------------------------ the net's radio check: a man who does not answer becomes a stimulus in himself
// Nobody reacts to a death nobody has discovered — this is how an undiscovered one becomes known, late: every so often (sooner on an alarmed floor) somebody calls
// a radio check and the living count in; a name that does not come back is asked for, then somebody free is sent round the silent man's route ('checkOn', an
// errand like the breaker's). Coming on the body on the way is the ordinary discovery ('man down!', the aftermath); walking the whole route without him is its
// own alarm: 'not at his post' raises the floor a step, once per missing man. A lockdown follower who loses the man he trails for good reports him the same way
// at once (partnerWatch). None of it under the tour (quietUtility) or with the AI off.
/** seconds between radio checks on a calm floor / an alarmed one (heightened or locked down), each jittered ± ROLLCALL_JITTER of itself */
const ROLLCALL_CALM_SECS = 90, ROLLCALL_ALARMED_SECS = 45, ROLLCALL_JITTER = 0.1;
/** after the call: the living have counted in by ANSWER; a silent name is asked for at ASK (and from then on the others count him missing); at SEND somebody goes */
const ROLLCALL_ANSWER_SECS = 2.2, ROLLCALL_ASK_SECS = 4, ROLLCALL_SEND_SECS = 8;
/** a check due while a fight, a hunt or a room clear is on, or hard on the heels of an alarm (QUIET_AFTER an escalation raise), waits this long and asks again;
 *  and how long the net keeps trying to find a free man to send before leaving it to the next check */
const ROLLCALL_BUSY_RETRY_SECS = 15, ROLLCALL_QUIET_AFTER_SECS = 30, ROLLCALL_SEND_GIVEUP_SECS = 40;
/** the errand: seconds' look round at each of the man's waypoints (one he cannot reach is given up by the suspicious branch's 4 s no-headway rule) and the cap on the whole walk */
const CHECK_LOOK_SECS = 1.2, CHECK_CAP_SECS = 150;
/** partner awareness (lockdown pair): the man he trails out of his sight this long AND nowhere within GONE_M of where he last saw him → called, and reported missing */
const PARTNER_UNSEEN_SECS = 20, PARTNER_GONE_M = 8;

export interface Rollcall { nextAt: number; stage: 'idle' | 'called' | 'asked'; t0: number; caller: Guard | null; missing: Guard | null; answered: boolean; checks: number; }
const rollcalls = new WeakMap<Game, Rollcall>();
/** A man who cannot answer the net: down (dead or out cold), or held with Sam's arm across his throat (Guard.held). Every radio-check predicate asks this, not
 *  'dead' — so a held man fails the sound-off exactly like a corpse: asked for by name while you hold him, somebody sent round his route toward you — and one
 *  let go answers again from the next frame (checkOnTick / the 'asked' stage drop him as they would a man walked onto alive). */
export function offNet(x: Guard): boolean { return x.state === 'dead' || !!x.held; }
/** The net's radio-check state for this game (made on first touch; the panel / probes read it: when the next check is due, what stage a running one is at). */
export function rollcallState(g: Game): Rollcall {
  let R = rollcalls.get(g); if (!R) { R = { nextAt: -1, stage: 'idle', t0: 0, caller: null, missing: null, answered: false, checks: 0 }; rollcalls.set(g, R); }
  return R;
}
function rollcallInterval(g: Game): number { return (escalationOf(g) >= 1 ? ROLLCALL_ALARMED_SECS : ROLLCALL_CALM_SECS) * (1 + (Math.random() * 2 - 1) * ROLLCALL_JITTER); }
/** who calls the check / asks after the silent man: a living man with his hands free for the radio — not mid-fight, not choreographed (the pair on a door), not
 *  walking up to or standing over a body — one on his route for choice, else whoever is left; null = nobody can just now */
function radioCaller(g: Game): Guard | null {
  let best: Guard | null = null, br = -1;
  for (const x of g.guards) {
    if (offNet(x) || x.state === 'alert' || x.script || (x.bodyDuty && x.bodyDuty.stage !== 'fan')) continue;
    const r = x.state === 'patrol' ? 2 : 1; if (r > br) { br = r; best = x; }
  }
  return best;
}
/** somebody already out walking this man's route */
function checkerFor(g: Game, M: Guard): Guard | null { return g.guards.find(x => x.state !== 'dead' && x.task?.kind === 'checkOn' && x.task.who === M) ?? null; }
/** A silent man is being looked for right now — somebody walks his route, or the net has asked for him and is about to send somebody: the floor's alarm level
 *  holds meanwhile (updateEscalation does not step down under it; the QA soak's clock invariant knows the same rule). */
export function stillLooking(g: Game): boolean { return rollcallState(g).stage === 'asked' || g.guards.some(x => x.state !== 'dead' && x.task?.kind === 'checkOn'); }
/** the living man best placed to go round M's route: calm (patrolling), free of orders, errands and bodies — and of Sam's arm — nearest any point of it; `prefer` first if he qualifies */
function pickChecker(g: Game, M: Guard, prefer: Guard | null = null): Guard | null {
  const pts = g.level.routes[M.routeI]?.points ?? [M.char.pos];
  const free = (x: Guard) => x !== M && x.state === 'patrol' && !x.hold && !x.script && !x.task && !x.bodyDuty && !x.held;
  if (prefer && free(prefer)) return prefer;
  let best: Guard | null = null, bd = Infinity;
  for (const x of g.guards) { if (!free(x)) continue; let d = Infinity; for (const p of pts) d = Math.min(d, v3.distXZ(p, x.char.pos)); if (d < bd) { bd = d; best = x; } }
  return best;
}
/** Send `gd` round M's route: the checkOn errand from the waypoint of M's nearest him, all the way round (the suspicious branch walks it: checkOnTick / advanceCheckOn). */
function sendCheckOn(g: Game, gd: Guard, M: Guard, line: string) {
  const pts = g.level.routes[M.routeI]?.points ?? [];
  let i0 = 0, bd = Infinity; pts.forEach((p, i) => { const d = v3.distXZ(p, gd.char.pos); if (d < bd) { bd = d; i0 = i; } });
  const pos: Vec3 = pts.length ? v3.copy(pts[i0]) : v3.copy(g.level.routes[M.routeI]?.points[0] ?? M.char.pos);
  gd.task = { kind: 'checkOn', pos, who: M, wp: i0, left: Math.max(1, pts.length), t0: g.time };
  gd.lastKnown = v3.copy(pos); gd.awareness = Math.max(gd.awareness, 0.42);
  if (gd.state === 'patrol') { gd.state = 'suspicious'; gd.reactT = 0.8; }   // (straight there, no 'huh?': he knows exactly what he is about)
  gd.searchT = 0; gd.path = []; gd.pathGoal = null; gd.bodyDuty = null; gd.stallRef = null;
  g.say(gd, line, true); g.audio.play('radio', gd.char.bones.head ?? gd.char.pos, 0.5, { rate: 1.05 });
}
/** One frame of the errand's bookkeeping (before the suspicious branch moves him): over if the man has been found since (or never was missing — a reset), capped,
 *  or on to the next waypoint once he has had his look round this one (searchT counts from arrival). */
function checkOnTick(g: Game, gd: Guard) {
  const K = gd.task; if (!K || K.kind !== 'checkOn') return; const M = K.who;
  if (M.found || !offNet(M) || !g.guards.includes(M)) {   // known now — somebody walked onto him — so the walk is over: he goes over to the man like anybody else called to a body (the discovery skipped him, being on an errand); or he never was missing (a reset) / is back on the net (let go from a hold): it just decays
    gd.task = null;
    if (M.found && M.state === 'dead' && g.guards.includes(M) && !gd.bodyDuty && !g.quietUtility) { gd.bodyDuty = { body: M, role: 'called', stage: 'to', t: 0, t0: g.time, spot: null }; gd.lastKnown = bodyPos(M); gd.awareness = Math.max(gd.awareness, 0.45); gd.reactT = 0.5; gd.searchT = 0; gd.path = []; gd.pathGoal = null; }
    return;
  }
  if (g.time - K.t0 > CHECK_CAP_SECS) { checkOnDone(g, gd, M, true); return; }
  if (gd.reactT <= 0 && gd.lastKnown && v3.distXZ(gd.lastKnown, K.pos) < 0.6 && gd.searchT > CHECK_LOOK_SECS) advanceCheckOn(g, gd);
}
/** the next of the missing man's waypoints, or — the last one looked at — he is not on his route */
function advanceCheckOn(g: Game, gd: Guard) {
  const K = gd.task; if (!K || K.kind !== 'checkOn') return; const pts = g.level.routes[K.who.routeI]?.points ?? [];
  K.left--;
  if (K.left <= 0 || !pts.length) { checkOnDone(g, gd, K.who, false); return; }
  K.wp = (K.wp + 1) % pts.length; K.pos = v3.copy(pts[K.wp]); gd.lastKnown = v3.copy(K.pos); gd.searchT = 0; gd.path = []; gd.pathGoal = null; gd.stallRef = null;
  if (K.left === Math.floor(pts.length / 2)) { g.say(gd, `halfway round — nothing yet. ${K.who.callsign}, if you can hear this, sound off`, true); g.audio.play('radio', gd.char.bones.head ?? gd.char.pos, 0.4, { rate: 1.05 }); }
}
/** The walk is over without him: the line, and — once per missing man — the floor goes up a step on it (raiseEscalation 'missing'; the alarm's place is his post). */
function checkOnDone(g: Game, gd: Guard, M: Guard, capped: boolean) {
  gd.task = null; gd.searchT = 0; gd.awareness = clamp(gd.awareness, 0.2, 0.3);   // (he lets it go from here like any suspicion: 'clear here — staying on it' once the floor is up)
  g.say(gd, capped ? `been round most of it — no sign of ${M.callsign}. he's not at his post` : `walked his whole route — ${M.callsign} is not at his post, no sign of him anywhere`, true);
  g.audio.play('radio', gd.char.bones.head ?? gd.char.pos, 0.7);
  if (!M.missingRaised) {
    M.missingRaised = true;
    const post = g.level.routes[M.routeI]?.points[0] ?? M.char.pos; g.alarm.pos[0] = post[0]; g.alarm.pos[1] = 0; g.alarm.pos[2] = post[2]; g.alarm.placed = true;
    raiseEscalation(g, escalationOf(g) >= 1 ? 2 : 1, gd, 'missing');
  }
}
/** Once a frame (updateEscalation): the radio check's clock and its short exchange. Idle and unarmed under the tour or with the AI off (re-armed from scratch after). */
export function updateRollcall(g: Game, dt: number) {
  const R = rollcallState(g);
  if (g.quietUtility || !g.aiEnabled) { R.stage = 'idle'; R.nextAt = -1; return; }   // (the tour's minutes are nobody's silence; resetGuards zeroes it too)
  if (R.nextAt < 0) R.nextAt = g.time + rollcallInterval(g);
  const living = g.guards.filter(x => x.state !== 'dead');
  const callerOk = (x: Guard | null) => !!x && !offNet(x) && x.state !== 'alert' && !x.script && g.guards.includes(x);
  switch (R.stage) {
    case 'idle': {
      if (escalationOf(g) >= 1) R.nextAt = Math.min(R.nextAt, g.time + ROLLCALL_ALARMED_SECS * (1 + ROLLCALL_JITTER));   // the floor went up since it was scheduled: the alarmed interval applies from now
      if (g.time < R.nextAt) return;
      if (!living.length || g.player.down) { R.nextAt = g.time + rollcallInterval(g); return; }
      if (g.alarm.episode || living.some(x => x.state === 'alert') || (g.clearing && g.clearing.stage !== 'done') || g.time - g.alarm.raisedAt < ROLLCALL_QUIET_AFTER_SECS) { R.nextAt = g.time + ROLLCALL_BUSY_RETRY_SECS; return; }   // nobody runs a radio check over a fight, a live hunt or a pair on a door, nor straight after the floor has just been told something real
      const caller = radioCaller(g); if (!caller) { R.nextAt = g.time + ROLLCALL_BUSY_RETRY_SECS; return; }
      if (!g.guards.some(x => x !== caller && (!offNet(x) || (!x.found && !x.missingRaised && !checkerFor(g, x))))) { R.nextAt = g.time + rollcallInterval(g); return; }   // nobody left who would answer, and no silence that would be news: a man alone with what he already knows does not keep calling into the dark
      R.stage = 'called'; R.t0 = g.time; R.caller = caller; R.missing = null; R.answered = false; R.checks++;
      g.say(caller, 'radio check — sound off', true); g.audio.play('radio', caller.char.bones.head ?? caller.char.pos, 0.5);
      return;
    }
    case 'called': {
      const el = g.time - R.t0;
      if (!R.answered && el >= ROLLCALL_ANSWER_SECS) {   // the living count in, in roster order (one log line, not a bubble each); the dead nobody has found simply do not — a found man is known, nobody waits on him
        R.answered = true;
        const answers = g.guards.filter(x => x !== R.caller && !offNet(x)).map(x => `…${x.callsign}.`);   // (a held man does not: Sam's arm is across his throat)
        if (answers.length) g.msg('radio: ' + answers.join(' '));
        const silent = (x: Guard) => offNet(x) && !x.found && !checkerFor(g, x);
        R.missing = g.guards.find(x => silent(x) && !x.missingRaised) ?? g.guards.find(silent) ?? null;   // one name at a time: the first on the roster nobody is already out looking for — a fresh silence before one already reported
        if (!R.missing) { R.stage = 'idle'; R.nextAt = g.time + rollcallInterval(g); }
        return;
      }
      if (R.missing && el >= ROLLCALL_ASK_SECS) {
        const M = R.missing; if (!callerOk(R.caller)) R.caller = radioCaller(g);
        if (M.found || !offNet(M) || !g.guards.includes(M)) { R.stage = 'idle'; R.nextAt = g.time + rollcallInterval(g); return; }   // (walked onto in the two seconds since — or let go and back on the net: that answers it)
        if (M.missingRaised) {   // already reported missing and his route walked: the check notes it, nobody is sent round the same empty route again
          if (R.caller) { g.say(R.caller, `and still nothing from ${M.callsign} — keep looking for him`, true); g.audio.play('radio', R.caller.char.bones.head ?? R.caller.char.pos, 0.5); }
          R.stage = 'idle'; R.nextAt = g.time + rollcallInterval(g); return;
        }
        if (R.caller) { g.say(R.caller, `${M.callsign}? … ${M.callsign}, sound off`, true); g.audio.play('radio', R.caller.char.bones.head ?? R.caller.char.pos, 0.55); }
        if (M.missedAt < 0) M.missedAt = g.time;   // from here on the others count him missing (superviseLockdown stops dealing his corpse a role)
        R.stage = 'asked';
      }
      return;
    }
    case 'asked': {
      const M = R.missing; const el = g.time - R.t0;
      if (!M || !offNet(M) || M.found || !g.guards.includes(M) || checkerFor(g, M)) { R.stage = 'idle'; R.nextAt = g.time + rollcallInterval(g); return; }   // (found meanwhile — the answer to where he is — or reset, let go, or somebody is already going)
      if (el < ROLLCALL_SEND_SECS) return;
      if (!callerOk(R.caller)) R.caller = radioCaller(g);
      const checker = pickChecker(g, M, null);
      if (!checker) {   // everybody busy with something louder: keep trying for a while, then leave it to the next check (which asks for him again)
        if (el > ROLLCALL_SEND_GIVEUP_SECS) { R.stage = 'idle'; R.nextAt = g.time + ROLLCALL_BUSY_RETRY_SECS; }
        return;
      }
      if (R.caller && R.caller !== checker) { g.say(R.caller, `nothing from ${M.callsign}. ${checker.callsign}, go round his route — check on him`, true); g.audio.play('radio', R.caller.char.bones.head ?? R.caller.char.pos, 0.55); }
      sendCheckOn(g, checker, M, R.caller === checker ? `nothing from ${M.callsign} — going round his route to check on him` : `copy — going to check on ${M.callsign}`);
      R.stage = 'idle'; R.nextAt = g.time + rollcallInterval(g);
      return;
    }
  }
}
/** Lockdown follower, each frame on patrol: keep track of when he last had his leader in plain sight (no wall between, inside 16 m — no cone: the man on whose
 *  shoulder he walks is kept by ear and the corner of the eye; this is about walls and distance). PARTNER_UNSEEN_SECS without him AND the man nowhere within
 *  PARTNER_GONE_M of where he last was: he calls him — a living leader answers and that is that; a dead one cannot, and the follower reports him missing there
 *  and then (the radio check's 'asked' stage: SEND − ASK seconds of silence, then whoever is nearest the route — he, walking it behind the man — is sent round it). */
function partnerWatch(g: Game, gd: Guard) {
  const L = gd.leader; if (!L) return;
  const c = gd.char; const eye = c.bones.head ?? v3.add(c.pos, [0, 1.65, 0]);
  const at = L.state !== 'dead' ? (L.char.bones.chest ?? v3.add(L.char.pos, [0, 1.1, 0])) : v3.add(bodyPos(L), [0, 0.4, 0]);   // (a body: the line to where it lies, not to a chest bone under a desk)
  const seen = v3.dist(eye, at) < 16 && !g.col.segmentBlocked(eye, at);
  if (seen || gd.leaderSeenT < 0) { gd.leaderSeenT = g.time; gd.leaderSeenPos = v3.copy(L.char.pos); return; }
  if (!g.aiEnabled || g.time - gd.leaderSeenT < PARTNER_UNSEEN_SECS || v3.distXZ(L.char.pos, gd.leaderSeenPos) < PARTNER_GONE_M) return;   // (AI off: he keeps track, he just does nothing about it)
  gd.leaderSeenT = g.time;   // (whatever comes of it, not again for another spell)
  if (!offNet(L)) {   // alive and free to answer, just long out of his sight and well away: he asks, the man answers, they close up (superviseLockdown re-deals a split pair anyway)
    g.say(gd, `${L.callsign}, where've you got to?`, true); g.say(L, `${gd.callsign} — on me, close up`, true); g.audio.play('radio', eye, 0.5);
    return;
  }
  // dead — or held in Sam's arm somewhere out of his sight: either way nothing comes back, and he reports him
  const R = rollcallState(g);
  if (R.stage !== 'idle' || L.found || checkerFor(g, L)) return;   // the net is mid-check (it will get to him), or he is no longer a question
  g.say(gd, `${L.callsign}? …I've lost ${L.callsign} — he was just ahead of me. ${L.callsign}, sound off`, true); g.audio.play('radio', eye, 0.6);
  if (L.missedAt < 0) L.missedAt = g.time;
  R.stage = 'asked'; R.t0 = g.time - ROLLCALL_ASK_SECS; R.caller = gd; R.missing = L; R.answered = true; R.checks++;
}

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
export function planClears(g: Game, force = false, cause: 'search' | 'body' | 'panel' | 'missing' = 'search'): boolean {
  if (g.quietUtility || !g.aiEnabled || escalationOf(g) < 2) return false;
  if (g.clearing) { if (!force && cause !== 'body') return false; g.clearing.cancel(); g.clearing = null; }
  let a: Guard | null = null, b: Guard | null = null;
  for (const gd of g.guards) if (gd.state !== 'dead' && !gd.held && gd.leader && gd.leader.state !== 'dead' && !gd.leader.held) { a = gd.leader; b = gd; }   // (a pair with a man in Sam's arm in it is no pair to send round the rooms — nor can he take the order)
  if (!a) { const alive = g.guards.filter(x => x.state !== 'dead' && !x.held); if (alive.length !== 1) return false; a = alive[0]; }
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
  updateRollcall(g, dt);   // (idles itself under the tour / AI off)
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
  if (g.escalation > 0 && !A.episode && !hunting && !stillLooking(g) && g.time >= g.escalationT) stepDownEscalation(g);   // (nor while a man is out walking a silent colleague's route or the net is still asking after one: nobody stands the floor down in the middle of that)
  if (g.escalation >= 2) superviseLockdown(g, dt, living);
  if (g.clearing && !g.clearing.tick(dt)) g.clearing = null;   // the pair clearing rooms: their approach, drill and senses this frame (it dissolves itself when done, broken off, or the floor changed under it)
}

/** Keep the lockdown duties dealt: (again) when there is no pair but two men live, when the leader died, when the pair has been > 8 m apart for 5 s while both were
 *  calm (stuck, or split by a chase), when a third man lives but nobody holds a post, or when only one man is left (he just patrols, drawn). */
function superviseLockdown(g: Game, dt: number, living: number) {
  // Roles react to what the men KNOW, not to ground truth: a man taken down quietly keeps 'his' role (his corpse holds it) until the body is found, he has failed to
  // answer the net (missedAt), or the floor is in an alarm episode anyway — otherwise a silent takedown had the man on the junction radio 'you're with me' and walk
  // off it seconds before anyone knew.
  const A = g.alarm; const knownDead = (x: Guard) => x.state === 'dead' && (x.found || A.episode || x.missedAt >= 0);
  let follower: Guard | null = null, postman: Guard | null = null, standing = 0;
  for (const gd of g.guards) { if (knownDead(gd) || gd.held) continue; standing++; if (gd.leader) follower = gd; if (gd.post) postman = gd; }   // (a man in Sam's arm holds no role and is dealt none — assignLockdown — so he is not a man standing for this count either, or three alive with one held re-dealt for the missing post man every frame)
  const posts = (g.level.chokepoints?.length ?? 0) > 0;   // (a level without chokepoints never has a post man — that must not read as 'deal again' every frame)
  let redo = standing >= 2 ? (!follower || knownDead(follower.leader!) || (standing >= 3 && living >= 3 && posts && !postman)) : !!(follower || postman);   // (living ≥ 3 too: a corpse nobody knows about keeps the post it held, but once a re-deal has taken it off him there is no third living man to hand it to — asking again every frame for one dealt nothing, and reset the follower's plan and the pair's apart-clock sixty times a second)
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
  const alive = g.guards.filter(x => x.state !== 'dead' && !x.held); if (alive.length < 2) return;   // (a man held in Sam's arm is dealt nothing: he can neither trail, lead nor hold a junction, and everybody the call has reached knows why)
  let a = alive[0], b = alive[1], bd = Infinity;
  const C = g.clearing;   // a pair in the middle of clearing a room stays the pair (a re-deal around them only moves the post)
  if (C && C.b && alive.includes(C.a) && alive.includes(C.b)) { a = C.a; b = C.b; }
  else for (let i = 0; i < alive.length; i++) for (let j = i + 1; j < alive.length; j++) { const d = v3.dist(alive[i].char.pos, alive[j].char.pos); if (d < bd) { bd = d; a = alive[i]; b = alive[j]; } }
  const lead = C && a === C.a ? a : routeLength(g, a) >= routeLength(g, b) ? a : b, follow = lead === a ? b : a;
  follow.leader = lead; follow.followT = 0; follow.leaderStillT = 0; follow.path = []; follow.pathGoal = null; follow.leaderSeenT = g.time; follow.leaderSeenPos = v3.copy(lead.char.pos);   // (dealt over the net: he knows where the man is as of now)
  if (redeal && (lead !== oldLead || follow !== oldFollow) && lead !== except && lead.state !== 'alert' && !(lead.bubble && g.time - lead.bubble.t < 0.5)) g.say(lead, 'you\'re with me — close up');   // (not over his own order of a moment ago — with two men left the spokesman IS the new lead; his man closes up regardless — and not from a man mid-fight: the pairing stands, the chatter waits)
  const third = alive.find(x => x !== a && x !== b); const cps = g.level.chokepoints ?? [];
  if (third && cps.length) {
    const ref = g.alarm.placed ? g.alarm.pos : third.char.pos; let best = cps[0], cd = Infinity;
    for (const cp of cps) { const d = v3.dist(cp.pos, ref); if (d < cd) { cd = d; best = cp; } }
    third.post = best; third.followT = 0; third.path = []; third.pathGoal = null;
    if ((best !== oldPost || third !== oldPostMan) && third !== except && third.state !== 'alert') g.say(third, `holding the ${best.name}`, true);   // over the radio, so it reaches the log: where the spare man will be standing (a man mid-fight takes it up when the fight is over: holdPost is the calm patrol's)
  }
}
function clearLockdown(g: Game) { for (const gd of g.guards) { if (gd.leader || gd.post) { gd.path = []; gd.pathGoal = null; } gd.leader = null; gd.post = null; } }
function routeLength(g: Game, gd: Guard): number { const pts = g.level.routes[gd.routeI]?.points ?? []; let s = 0; for (let i = 0; i < pts.length; i++) s += v3.dist(pts[i], pts[(i + 1) % pts.length]); return s; }

/** seconds a follower who has lost sight of his leader still knows where the man is going (heard him, saw him turn in at the door) and trails his true position;
 *  past it he holds what he last saw — the spot — rather than follow a man (or a body being hauled off) through walls he cannot see through */
const PARTNER_TRUST_SECS = 3;
/** Lockdown follower on patrol: keep station ~1.4 m behind-left of the leader — as he KNOWS him: where the man is while he has him in sight (partnerWatch keeps
 *  that) and for a few seconds after, else where he last saw him — re-planned at most twice a second and only when the station has actually moved; close up
 *  briskly when left behind, and when the leader stops (or the spot is all he has), plant at his shoulder facing the way he faced and sweep the rear-left quarter.
 *  True while walking. */
function followLeader(g: Game, gd: Guard, dt: number, walk: number): boolean {
  const L = gd.leader!, c = gd.char, goal = gd.followGoal, nav = g.col.nav;
  const fresh = gd.leaderSeenT < 0 || g.time - gd.leaderSeenT < PARTNER_TRUST_SECS; const lp = fresh ? L.char.pos : gd.leaderSeenPos;
  gd.followT -= dt; gd.leaderStillT = !fresh || Math.hypot(L.char.vel[0], L.char.vel[2]) < 0.15 ? gd.leaderStillT + dt : 0;   // (a spot does not move: planted by it long enough he turns and looks at it, as at a leader who never moves again)
  if (gd.followT <= 0) {
    gd.followT = 0.5;
    const f = L.char.forward(), lv: Vec3 = fresh ? L.char.vel : [0, 0, 0];
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
