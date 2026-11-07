// Guard AI: hearing (the same propagation the mixer uses), vision (the player's light-meter irradiance, marched through smoke), patrol / suspicious / alert / search state machine, path following, torch handling, and their barks.
// (Split out of game.ts mechanically: plain functions taking the Game as their first argument; Game keeps one-line delegators for
//  anything other files call.)
import { men, eachSeat, orderOf, ordersOf, viewOfBody, seatCount, budgetWithSeats } from './seats';
import { SideMap, SideSet, GameSideMap, rollbackVar, rollbackRoot } from '../math/side';
import { openingStart } from './opening';
import { Vec3, v3, clamp, damp, lerp, smoothstep, DEG } from '../math/vec';
import { PatrolRoute, Chokepoint, RoomClearPlan } from '../scene/level';
import { Character, HOLD } from './character';
import { manLeaves } from '../sim/men/store';
import './character';
import { BoxFlag } from '../scene/boxes';
import { BAKE_Y, type StaticCollision, type LampRows } from '../scene/collision';
import { SENSE, type LightField } from './sense';   // (read inside functions only: the light model's switch and its field)
import { FOG, fogThrough } from './fogofwar';
import { Door } from './doors';
import { rigLightPose, rigTorchPos, torchCarryDir, torchCarryWant, torchLightDef, RIG, sc20kLightPose } from './rigProps';
import { radioHand, swapHeld } from './handwork';
import './mission';
import { chronLost, chronAlarm, chronRadio } from './chronicle';
import './combat';
import { lensClear } from './lighting';
import './gunwall';
import type { Game, GameEvent, DownKind, DownBy, Temperament, PairId } from './game';
import { Guard } from './game';
import { Clearing, inBounds, endGuardScript } from './squad';
import './frag';
import './morale';
import { conditionCalm } from './condition';
import './runner';
import './darkdoor';
import './netquiet';
import { pairSmoke, fightLive, lampGlare, closeLane, ROOMFIGHT } from './roomfight';
import { isOffice, temperamentOf } from './dialogue';
import { WORDS, w, wg } from './words';
import './cues';
import { SQUADNET } from './squadnet';
import './suppression';
import './aim';
import { isHostile, spokenName } from './teams';
import type { Team } from './teams';
import type { WeaponId } from './weapons';
import { nvgSight, nvgDark } from './nvg';
import { silentDark, silentHush } from './silent';
import { pairGripped } from './pairorders';
import './picture';   // (asked inside colleagueFirePos only: src/game is one import cycle)
import { MAX_LIGHTS } from '../render/lights';
import { closeNearK } from './close';
import { calmTorchDark, TORCH } from './calmtorch';
import { darkFoeLit } from './darkfoe';
import { powerOutLit } from './powerout';
import { whistleSoft } from './whistle';
import { hearPath, strokeLight } from './weather';
import { attentionSideK } from './attention';
import { tacticsPenalty } from './tactics';
import { lanePenalty } from './lanes';
import { propPenalty, propWay } from './orders';
import './doorway';
import './needs';
import { dozeDark, dozeAsleep } from './doze';
import './coverfire';
import { startleTorch } from './startle';
import './doorforce';
import './glass';
import { shotEnergy } from './glasspen';
import { kitPistol } from './kit';
import './gunslots';   // (called inside startReloadBeat only)
import { hourLight, moonLight } from './hours';
import './handleside';
import './gunhold';
import { teamAtWalk } from './teampace';   // (teampace.ts: a teammate's walk is a share of the player's, and he runs only for a reason — all read inside functions)
import { loadSeen } from './kitload';   // what a man's card weighs: his pace, the notice he builds, his footfall (1 each with its switch off)
import './tapstep';
import { simRandom } from '../math/dice';   // the simulation's die (math/dice.ts): the Game's own when it has one, Math.random otherwise
import './rails';   // (a tutorial mission's spared teammate is a man down to the enemy's eye: asked inside acquireFoe only)
import { LIGHT_IN, ROW_BIT } from '../sim/lightshome';
import { RUST_PASS, RUST_SURFACE, crossToRust, goneWithTick } from './crossing';

/** A/B switches for the AI passes (the QA probes flip them to print before / after off the same build; play never touches them): motion acuity in the sight
 *  gain, the search that deals hide spots and reads the renderer's light (off: the old sweep-and-wander), threshold discipline at doorways (off: straight through). */
export const AI_TUNE = { motionAcuity: true, darkSearch: true, pieing: true };

// ------------------------------------------------------------ guards
/** HOW A 'glass' EVENT REACHES AN EAR (hearingCheck below, civilians.ts): the usual propagation from the event's own point — and, where the event carries the
 *  pane's two faces (glass.ts glassEvent under GLASS.bothFaces), from each of those too, the one that arrives best standing (the glass rule's own weight: reach × carry inside GLASS.hearM; the event's own
 *  point wins a tie, so nobody hears a pane worse for the rule). No `ears` = hearPath of the point, exactly as before. */
export function glassHeard(g: Game, e: GameEvent, ear: Vec3): ReturnType<typeof hearPath> { return crossToRust('glassHeard', undefined, [g, e, ear]) as any; }
export function hearingCheck(g: Game, gd: Guard, ear = 1, eyes = true, foot = ear): void { return crossToRust('hearingCheck', undefined, [g, gd, ear, eyes, foot]) as any; }

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
const groundTrack = new SideMap<Game, { pos: Vec3; t: number; vel: Vec3 }>('guards.groundTrack');
/** a restart (cleanrestart.ts): the fresh man has not moved yet — his start is not a step from where the last one fell */
export function groundFresh(g: Game): void { groundTrack.delete(g); }
export function playerGroundVel(g: Game): Vec3 { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('playerGroundVel', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
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
export function motionFactor(g: Game, gd: Guard, dist: number): number { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('motionFactor', undefined, [g, gd, dist]); if (__r !== RUST_PASS) return __r as any; } }
  return motionCore(gd, g.player.char.pos, playerGroundVel(g), g.player.visibility, dist);
}
/** motionFactor's arithmetic for any watched man at `pp` moving at `v` over the ground, lit `lit` (0‥1), `dist` off: Sam's numbers come from playerGroundVel and
 *  his meter (above, unchanged to the last bit); an AI target's from his own velocity — a walking AI man's vel IS his ground velocity — and his meter (teams). */
function motionCore(gd: Guard, pp: Vec3, v: Vec3, lit: number, dist: number): number {
  const gp = gd.char.pos;
  let rx = gp[0] - pp[0], rz = gp[2] - pp[2]; const rl = Math.hypot(rx, rz) || 1; rx /= rl; rz /= rl;   // unit, Sam → the eye (flat)
  const vr = Math.abs(v[0] * rx + v[2] * rz), vt = Math.abs(v[0] * rz - v[2] * rx); const sp = Math.hypot(v[0], v[2]);
  const still = lerp(MOTION_STILL, 1, smoothstep(MOTION_LIT_LO, MOTION_LIT_HI, lit));
  let m = lerp(still, 1, smoothstep(MOTION_LO, MOTION_HI, vt + MOTION_RADIAL * vr)); if (sp > MOTION_SPRINT_MS) m *= MOTION_SPRINT;
  if (dist < MOTION_NEAR_M) m = Math.max(m, MOTION_NEAR_FLOOR);
  if (gd.state === 'alert' || gd.state === 'search') m = Math.max(m, MOTION_HUNT_FLOOR);
  return m;
}

/** Awareness per second a VISIBLE player builds in this man at `dist` (canSee said yes): the light on the player, distance, crouch, how he is MOVING across this
 *  man's view (motionFactor: a still shape in the half-dark fills the meter at a third of the rate of one walking across it, a sprint faster again), the smoke
 *  between them, the alarmed floor's keenness — point-blank is certain — and a man already hunting is twice as quick on the uptake. One formula for updateGuard
 *  and for the scripted men's senses (squad.ts perceive). Above 0.05 it moves his fix; above 0.25 it is a real sighting (the licence to shoot). */
export function sightGain(g: Game, gd: Guard, dist: number): number { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('sightGain', undefined, [g, gd, dist]); if (__r !== RUST_PASS) return __r as any; } }
  const T = g.tune; const light = g.player.visibility; const distF = distTerm(dist);
  const moving = AI_TUNE.motionAcuity ? motionFactor(g, gd, dist) : (Math.hypot(g.player.char.vel[0], g.player.char.vel[2]) > 2 ? 1.5 : 1);   // (the switch is the QA table's before / after; play runs the acuity)
  let rate = T.detectRate * Math.pow(light, 1.5) * distF * (g.player.crouch ? 0.75 : 1) * moving * clamp((gd.smokeTrans - 0.3) / 0.5, 0, 1) * (escalationOf(g) >= 1 ? KEEN : 1);   // (an alarmed floor is actually looking: a quarter quicker on the uptake)
  rate *= attentionSideK(g, gd, g.player.char.pos) * loadSeen(g, 'player');   // (attention.ts: 1 — but for a man whose attention is on a thing, of somebody outside what he is looking at; kitload.ts: what his card makes of his shape — 1 with its switch off or no card on him; the point-blank floor below stands whole)
  if (dist < 1.8) rate = Math.max(rate, 3.5 * closeNearK(g, gd, dist));   // (close.ts: 1 unless its switch is on — then a shape at his elbow in the DARK is not seen for being near)
  if (gd.state === 'alert' || gd.state === 'search') rate *= 2.2;
  return rate;
}
/** sightGain for an AI target `o` (teams: an operative's eye on a hostile, a hostile's on an operative — canSeeChar said yes): the SAME formula off HIS light meter
 *  (Guard.visibility: the renderer's irradiance at him when it answers, the CPU estimate of the same direct light otherwise — unitLightMeter), his crouch, his
 *  motion across this man's view, the alarmed floor's keenness and the hunting man's ×2.2. Two things it cannot have: the smoke between two AI men is not marched
 *  (the probe's per-guard segment runs to Sam's chest only — the daytime showcase has no smoke; a v1 limit, named), and Sam's sandbox invisibility is his alone. */
export function sightGainOn(g: Game, gd: Guard, o: Guard, dist: number): number { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('sightGainOn', undefined, [g, gd, o, dist]); if (__r !== RUST_PASS) return __r as any; } }
  const T = g.tune; const light = Math.max(o.visibility, lampGlare(g, gd.char, o), nvgSight(g, gd, o)); const distF = distTerm(dist);   // (the light ON him — or, in a room fight, his own lit weapon light pointed at this eye: roomfight.ts lampGlare, 0 elsewhere — or what this eye's goggles make of it: nvg.ts, 0 with none down)
  const v = o.char.vel;
  const moving = AI_TUNE.motionAcuity ? motionCore(gd, o.char.pos, v, light, dist) : (Math.hypot(v[0], v[2]) > 2 ? 1.5 : 1);
  const smoke = pairSmoke(g, gd.char, o.char);   // the smoke between them thins the sight of him as it does of Sam, in the same place in the formula (roomfight.ts marches the pair; 1 = clear / not marched)
  let rate = T.detectRate * Math.pow(light, 1.5) * distF * (o.char.anim.crouch > 0.5 ? 0.75 : 1) * moving * (smoke < 1 ? clamp((smoke - 0.3) / 0.5, 0, 1) : 1) * (escalationOf(g) >= 1 ? KEEN : 1);
  rate *= attentionSideK(g, gd, o.char.pos) * loadSeen(g, o);   // (attention.ts, as for Sam: the partner beside him in a quiet two is seen by the same held eye; kitload.ts, as for Sam: a teammate's card — 1 for a man no staging put a card on)
  if (dist < 1.8) rate = Math.max(rate, 3.5);
  if (gd.state === 'alert' || gd.state === 'search') rate *= 2.2;
  return rate;
}

/** A dead colleague lying in this man's view: within 12 m, inside 60° of where his chest points, nothing solid between his eyes and the body — and (tune.bodyLight,
 *  the default) light enough on the body to make it out, no plume between thick enough to hide it (bodyLit below). (updateGuard raises the alarm on it; the
 *  scripted men's senses in squad.ts / qrf.ts only need to know to break off toward it — updateGuard does the rest the frame after.) */
export function bodyInView(g: Game, gd: Guard): Guard | null { return crossToRust('bodyInView', undefined, [g, gd]) as any; }
/** bodyInView's test of ONE body, whoever asks (enemysquads.ts asks it of a squad's men for a man of theirs who went quiet): within 12 m of his eyes, inside
 *  60° of where his chest points, nothing solid between, and light enough on it to make out. `around`: a man who has walked to the spot to look
 *  and looks about him — no cone; a number = also how far round his FEET (m, flat) a body is at his feet in any light, as the
 *  2 m rule below has it for a man who walks onto one (enemysquads.ts's lookers stop short of the post, and a body thrown by a round lies a metre off it) */
export function bodySeen(g: Game, gd: Guard, o: Guard, around: boolean | number = false): boolean { return crossToRust('bodySeen', undefined, [g, gd, o, around]) as any; }

// ------------------------------------------------------------ bodies are found by the light on them (tune.bodyLight — review A6.2, Chaos Theory's rule)
// A body in the dark is HIDDEN; one under a lamp, in a lit doorway or in the pool of a torch — the finder's own included — is found. What bodyInView asks on top of
// its range / cone / line: the light where the body lies, read the way every eye in the game reads a man — the renderer's irradiance at the body's trunk (a probe
// query keyed by the body, answered a frame or two later) and until it answers (the browser's first frames; headless, always) the CPU estimate of the same direct
// light (lightEstimateAt: fixtures, lamps, the torches and weapon lights in the set — his own beam on it is simply one of those lights) — through the meter's own
// curve (Game.visibilityOf), eased like everybody's meter; and the smoke between his eye and it (the probe's segment transmittance, as canSee marches it to Sam).
// Off (Tab ▸ Sandbox) = range, cone and line alone: found in the pitch dark at twelve metres, as before.
/** the meter reading (0‥1) at the body at or above which it is plain from FAR_M and beyond (0.22 ≈ the fringe of a corridor panel's pool as the meter reads it; a
 *  torch beam squarely on it reads 0.7+, open floor between two panels at night ~0.35, a dark office 0); inside NEAR_M he all but trips on it — found in any light
 *  — and from NEAR to FAR the bar eases up from LIT_NEAR to LIT; SMOKE: eye → body transmittance under which the plume hides it (canSee's own line for a standing man) */
export const BODY_LIGHT = { lit: 0.22, litNear: 0.04, nearM: 2.0, farM: 5.0, smoke: 0.3 };
export const BODY_LIGHT_DEFAULTS = { ...BODY_LIGHT };
/** the body's light meter, per body (the light ON it is whoever's-looking-independent): eased reading + when; and the last irradiance behind it for the panel */
const bodyMeters = new SideMap<Guard, { vis: number; E: number; t: number }>('guards.bodyMeters');
/** probe queries these tests may spend a frame between them (light points + smoke segments): they run inside the guards' update, AHEAD of Sam's own two meter
 *  queries in the probe's 32-slot frame, and a floor of bodies under three pairs of eyes must never crowd those out — past it a body keeps its eased reading /
 *  the CPU estimate and a pair its last transmittance for the frame */
const BODY_PROBE_BUDGET = 6;
const bodyProbeN = new SideMap<Game, { t: number; n: number }>('guards.bodyProbeN');
function bodyProbeSlot(g: Game): boolean { let b = bodyProbeN.get(g); if (!b || b.t !== g.time) { b = { t: g.time, n: 0 }; bodyProbeN.set(g, b); } return b.n++ < BODY_PROBE_BUDGET; }
/** What the meter reads at this body now (0‥1): probe query `bodyL<id>` at its trunk (its own capsules skipped), the CPU estimate until answered, eased at the meter's rate. */
export function bodyVisibility(g: Game, o: Guard): number {
  let m = bodyMeters.get(o); const now = g.time;
  if (m && m.t === now) return m.vis;
  const c = o.char, id = c.id; const hp = c.bones.hips ?? c.pos;
  const at: Vec3 = [hp[0], Math.max(hp[1], 0.12) + 0.22, hp[2]];   // the trunk of a man lying down: a hand's height over the pelvis (a query flush with the floor reads the floor's own shadow terminator)
  const key = `bodyL${id}`; if (bodyProbeSlot(g)) g.lightQuery(key, at, id);
  const r = g.lightAt(key);
  const E = r !== undefined && Number.isFinite(r) ? r : lightEstimateAt(g, at);
  const dt = m ? Math.min(0.1, Math.max(0, now - m.t)) : 0;
  const target = g.visibilityOf(E); const vis = m && dt > 0 ? damp(m.vis, target, 6, dt) : target;   // (first look: the reading as it stands — a torch swung onto it a frame ago has already lit it)
  m = { vis, E, t: now }; bodyMeters.set(o, m); return vis;
}
/** the switch + its numbers kept per machine (Tab ▸ Sandbox ▸ 'bodies found by light'), like the tactics' clocks */
const BODY_LIGHT_LS = 'twopointsix.bodyLight';
export function loadBodyLight(tune: { bodyLight: boolean }) {
  try {
    const j = JSON.parse(localStorage.getItem(BODY_LIGHT_LS) ?? 'null'); if (!j || typeof j !== 'object') return;
    if (typeof j.on === 'boolean') tune.bodyLight = j.on;
    for (const k of ['lit', 'litNear', 'nearM', 'farM'] as const) { const v = j[k]; if (typeof v === 'number' && Number.isFinite(v) && v >= 0 && v <= 20) BODY_LIGHT[k] = v; }
  } catch { /* no storage (the harness, a locked-down page): the shipped values stand */ }
}
export function saveBodyLight(tune: { bodyLight: boolean }) { try { localStorage.setItem(BODY_LIGHT_LS, JSON.stringify({ on: tune.bodyLight, lit: BODY_LIGHT.lit, litNear: BODY_LIGHT.litNear, nearM: BODY_LIGHT.nearM, farM: BODY_LIGHT.farM })); } catch { /* nothing to keep it in */ } }
export function resetBodyLight(tune: { bodyLight: boolean }) { tune.bodyLight = true; Object.assign(BODY_LIGHT, BODY_LIGHT_DEFAULTS); try { localStorage.removeItem(BODY_LIGHT_LS); } catch { /* none */ } }
/** Panel / QA: what each unfound body on the floor reads right now — 'bodies ▸ Reyes 0.61 (E 0.412) · Novak 0.02 (E 0.001)' */
export function bodyLightSummary(g: Game): string { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('bodyLightSummary', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
  const parts: string[] = [];
  for (const o of g.guards) { if (o.state !== 'dead' || o.found) continue; const v = bodyVisibility(g, o); const m = bodyMeters.get(o); parts.push(`${o.callsign} ${v.toFixed(2)} (E ${m ? m.E.toFixed(3) : '?'})`); }
  return `bodies by light ▸ ${g.tune.bodyLight ? 'on' : 'OFF'} · bar ${BODY_LIGHT.lit} from ${BODY_LIGHT.farM} m (any light inside ${BODY_LIGHT.nearM} m) · ${parts.join(' · ') || 'nobody down unfound'}`;
}

/** A world point in this man's open view: within `range` of his eyes, inside `halfDeg` of where his chest points (flat), nothing solid between — bodyInView's test
 *  against any point. Lighting is deliberately not asked (the things it is used for carry their own light: a colleague with a torch going down). */
export function pointInView(g: Game, gd: Guard, p: Vec3, range = 12, halfDeg = 60): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('pointInView', undefined, [g, gd, p, range, halfDeg]); if (__r !== RUST_PASS) return __r as any; } }
  if (dozeAsleep(gd)) return false;   // (doze.ts: eyes shut — every reader of a man's view takes it here)
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
    if (o === victim || o.state === 'dead' || g.time < o.dazzledUntil || dozeAsleep(o)) continue;   // (doze.ts: a man asleep saw nothing)
    if ((!g.playerInvisible && pointInView(g, o, at)) || o.sawPlayerThisFrame) o.witness.sawAct = { kind: how === 'wounded' ? 'shot' : how, victim: victim.callsign, t: g.time };   // (a man shot down WOUNDED was seen shot: the questions asked of the witness are the shooting's)
  }
}
/** One compact line of a man's witness record for the panel / the QA probes ('' = nothing yet): peak awareness, what made him alert, what he heard, what he saw. */
export function witnessSummary(g: Game, gd: Guard): string {
  const W = gd.witness, H = W.heard; const parts: string[] = []; const ago = (t: number) => `${Math.max(0, Math.round(g.time - t))} s ago`;
  if (W.peakAwareness >= 0.05) parts.push(`peak ${W.peakAwareness.toFixed(2)}`);
  if (W.alertedBy) parts.push(`alert by ${W.alertedBy} ${ago(W.alertT)}`);
  const heard = [H.shots ? `${H.shots} shot${H.shots > 1 ? 's' : ''}` : '', H.kick ? `${H.kick} kick${H.kick > 1 ? 's' : ''}` : '', H.bang ? `${H.bang} bang${H.bang > 1 ? 's' : ''}` : '', H.small ? `${H.small} small${H.smallWhere ? ` (last @${H.smallWhere[0].toFixed(0)},${H.smallWhere[2].toFixed(0)})` : ''}` : ''].filter(Boolean);
  if (heard.length) parts.push(`heard ${heard.join(', ')}`);
  if (W.sawAct) parts.push(`SAW the player ${W.sawAct.kind === 'shot' ? 'shoot' : W.sawAct.kind === 'struck' ? 'drop' : W.sawAct.kind === 'choked' ? 'choke out' : 'grab'} ${W.sawAct.victim} ${ago(W.sawAct.t)}`);
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

/** The alert chase is over without him: down to 'search' from where he stands — awareness under the alert threshold (re-alerting takes a fresh sighting or sound,
 *  or alert ↔ search flapped every reactT), every chase clock zeroed for the next one, and the line if there is one. */
export function dropToSearch(g: Game, gd: Guard, bark: string | null) {
  gd.state = 'search'; gd.searchT = 0; gd.awareness = Math.min(gd.awareness, 0.8); gd.searchPlan = null;
  resetChase(gd); gd.path = []; gd.pathGoal = null;   // (the chase's path is not the search's: he sweeps from where he stands, then deals himself the hiding places round the fix — searchPlanFor)
  if (bark) g.say(gd, bark);
  chronLost(g, gd);   // the debrief's row: he lost you
}
/** Every clock the alert chase keeps, back to a fresh chase: how near he has got to the fix and how long without getting nearer (and which fix that was), how long
 *  without covering ground, how long without eyes on him, how long his line of fire has been blocked, and the side-step's memory of which way went nowhere.
 *  Called entering 'alert', on every fresh sight of Sam, leaving for 'search' — and by the QA probes staging an alert man (tools/qa/probes.ts armAlert). */
export function resetChase(gd: Guard) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('resetChase', undefined, [gd]); if (__r !== RUST_PASS) return __r as any; } } gd.alertBest = 1e9; gd.alertStallT = 0; gd.alertRef = null; gd.chaseT = 0; gd.shotBlockedT = 0; gd.lineInT = 0; gd.stallRef = null; gd.stallT = 0; gd.lineSide = 0; gd.lineGain = -1; gd.lineStuckT = 0; gd.lineFail = null; }


/** Nothing a round would stop in — wall, jamb, mullion, desk, a door leaf, a GLASS PANE — between this man's gun shoulder and `to` (less the body's breadth
 *  at the far end), so a pistol hand round the jamb reads the same whichever way the muzzle happens to point this frame. Loose furniture is concealment, not
 *  cover (fireWeapon lets the round through to whoever stands behind it), so it is not asked. This is the line of FIRE (col.shotBlocked); canSee is the line
 *  of SIGHT, from the eye (col.segmentBlocked, which passes glass) — at a doorway the two differ by exactly the hand's breadth that had him shooting the frame,
 *  and across the conference glazing they differ outright: he sees Sam through the pane and has no shot from there (the round stars the glass and stops in
 *  it), so the alert branch reads `blocked` and takes him round through the door instead of planting him on the corridor emptying magazines into the pane. */
/** Where a man's line of fire is judged FROM (clearShot; the carbine probes measure the muzzle against the same point): the gun shoulder — fixed to the body,
 *  so the answer does not swing with a muzzle still coming round — shifted, for a shouldered long gun, by the carriage's own inboard hold. */
export function shotOrigin(c: Character): Vec3 {
  // The pistol at arm's length: the gun shoulder — fixed to the body, so the answer does not swing with a muzzle still coming round (once it is on,
  // shoulder → target is the round's line give or take the arm). A shouldered LONG gun: the trigger-hand wrist itself, which IS on the bore wherever the
  // maintainer's carriage puts it (anchor between chest and shoulder, held inboard) — re-deriving that geometry from outside drifted 9–11 cm off the real
  // bore; and the swing concern is the aim model's now (aim.ts gates the round on a settled track), not this line's.
  if (c.anim.longGun && c.bones.handR) return v3.copy(c.bones.handR);
  return c.bones.upperArmR ?? v3.add(c.pos, [0, 1.4, 0]);
}
export function clearShot(g: Game, c: Character, to: Vec3): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('clearShot', undefined, [g, c, to]); if (__r !== RUST_PASS) return __r as any; } }
  const from = shotOrigin(c);   // (see shotOrigin: the gun shoulder for the pistol, the trigger-hand wrist on the bore for a long gun)
  const d = v3.sub(to, from); const L = v3.len(d); if (L < 0.35) return true;   // (pressed against him: nothing can be in between)
  return !g.col.shotBlocked(from, v3.mad(from, d, (L - 0.3) / L), 0.02, shotEnergy(g, c));   // (GLASS PENETRATION, glasspen.ts: with rounds going through glass the panes across the line are judged by what HIS gun's round has for them — off: undefined, the solid walk)   // 0.3: the body's own breadth round the aim point — anything nearer than that is him, not cover   // (shotBlocked, not segmentBlocked: a pane between them is no shot — see above)
}

// ------------------------------------------------------------ muzzle discipline: nobody points a pistol at a colleague — it goes to the low ready until the line is clear
/** metres off a living colleague's body axis inside which a line of aim counts as ON him (widened for a man on the move by MUZZLE_LEAD_SECS of his pace, capped:
 *  the gun comes down for a man about to walk into the line, not as he meets it); how far past the aim point the line still counts (a man standing just beyond
 *  the spot being covered is being pointed at too); the reach of 'straight ahead' when there is no point to aim at; and the beat a cleared line stays down before
 *  the gun comes back up (so a man crossing does not flick it down-up-down) */
const MUZZLE_CLEAR_M = 0.35, MUZZLE_LEAD_SECS = 0.3, MUZZLE_LEAD_MAX_M = 0.4, MUZZLE_PAST_M = 1.0;
/** QA hook: set `log` and the alert branch's footwork round a colleague in the line (lineStep: the geometry, each candidate step and what became of it; the
 *  branch the fight took this frame; what the statics and the colleagues pushed back) says what it did, per man per frame (tools/qa: soak --trace, the scrum probe) */
export const LINE_DEBUG: { log: ((gd: Guard, s: string) => void) | null } = { log: null };

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
  let best: Guard | null = null, bt = Infinity; const near = gd.team === 'operative' && !!gd.unit && fightLive(g);   // (the team in a room fight: roomfight.ts ROOMFIGHT.closeLane)
  for (const o of g.guards) {
    if (o === gd || o === except || o.state === 'dead' || !o.char.alive || isHostile(gd.team, o.team)) continue;   // (teams: an ENEMY in the line is no reason to lower the gun — he is what it is up for; his own side and neutrals are colleagues)
    const G = lineGeom(from, to, o); if (!G || G.t < 0 || G.t > tMax || G.t >= bt || G.d > (near ? closeLane(G.clear, G.t, L) : G.clear)) continue;   // behind his shoulder, further off than the point (and its margin), or simply clear of it
    bt = G.t; best = o;
  }
  return best;
}
/** the point MUZZLE_REACH_M out along where his gun is being held (aim yaw / pitch) from the gun shoulder — or where that line meets the floor, if it does first (a
 *  low carry pools a few metres out: the line ends there, it does not run on under the men beyond): what 'ahead' means for a man covering nothing in particular */
export function aimAhead(c: Character): Vec3 { return goneWithTick('aimAhead') as any; }

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
 *  the man is locked on it), the aim point on Sam's axis, the pair's velocity; null when nobody is held */
export interface Pair { key: object; held: Guard; sam: Vec3; heldAxis: Vec3; centre: Vec3; facing: number; aim: Vec3; vel: Vec3; }
export function pairOf(g: Game): Pair | null { return goneWithTick('pairOf') as any; }
/** holds already called in on the net (one call per hold, whoever sees it first), keyed by the hold itself; the floor's last standoff line (anybody's); each man's last line */
const holdsCalled = new SideSet<object>('guards.holdsCalled'); const floorBarkT = new SideMap<Game, number>('guards.floorBarkT'); const lastBark = new SideMap<Guard, string>('guards.lastBark');
/** a checkpoint loaded (game/savestate.ts) puts Game.time BACK: the floor's standoff-bark clock must not sit in the future muting the lines */
export function forgetGuardClocks(g: Game) { floorBarkT.delete(g); }
/** the held man's own light meter while he is held (staff carry none otherwise): two probe queries at his chest and head like a unit's, the CPU estimate of the same
 *  direct light until the GPU answers; damped like everyone's. Kept per HOLD (the hold object is the key), and STARTED OVER — the probe's last answer under his keys
 *  forgotten, the damp begun again from the reading as it stands — for a new hold and for a pair nobody has metered for `HELD_METER.gapSecs` that has moved
 *  `HELD_METER.movedM` since: the probe keeps a key's last answer for ever and ids are dealt again, so the last hold's light (or this hold's, a room ago) was read
 *  as this one's until the probe answered afresh, and the damp then carried it on for a third of a second. A pair re-met where it stood keeps its answer. */
export const HELD_METER = { gapSecs: 0.5, movedM: 1 };
const heldMeters = new SideMap<object, { vis: number; t: number; at: Vec3 }>('guards.heldMeters');
/** The fire rule (see the constants): may this man's round go, at Sam, past the colleague in his arm? ok, plus the three measures for whoever prints them: how far
 *  off the pair's rear he stands (rad), the held man's clearance off the round's segment (m), his distance from the pair (m). */
export function flankShot(g: Game, gd: Guard, P: Pair): { ok: boolean; rear: number; clear: number; dist: number } { return crossToRust('flankShot', undefined, [g, gd, P]) as any; }
/** Placeholder lines (a proper writing pass comes later), by moment and temperament ('*' for anybody); {held} = the man in the arm, {him} / {he} = the same man where a line
 *  does not name him (a woman in the arm is 'her': standoffBark fills them — 'him' written out is Sam). */
type StandoffMoment = 'enter' | 'noShot' | 'line' | 'cornered' | 'dropped' | 'freed' | 'lost' | 'hitHim';
const STANDOFF_LINES: Record<StandoffMoment, Partial<Record<Temperament | '*', string[]>>> = {
  enter: { steady: ['{held}, hold still — we\'ve got you', 'easy. easy. nobody has to get hurt here'], nervy: ['{held}! —jesus— okay, okay, nobody shoot!', 'oh god— he\'s got {held}—'], hard: ['let {him} go. now.', 'you. hands off {him}.'] },
  noShot: { '*': ['let {him} go!', 'drop {him} — NOW', '{held}, don\'t move', 'I don\'t have a shot', 'no shot — {held}\'s square in front of him'], steady: ['let {him} go and we talk about it', '{held}, stay loose — I\'m coming round'], nervy: ['{held}! —I can\'t— I haven\'t got a line—', 'don\'t you hurt {him}— don\'t—'], hard: ['you\'ve got nowhere to take {him}', 'the second {he} slips, you\'re done'] },
  line: { '*': ['I have him.', 'got a line —'], nervy: ['I— I think I have him—'], hard: ['there you are.'] },
  cornered: { '*': ['can\'t get round him — hold what you\'ve got', 'no way round. {held}, hang on', 'nowhere for either of us to go — let {him} walk and this ends quiet'], nervy: ['just— just let {him} go, please—'], hard: ['one way out of this for you. through us.'] },
  dropped: { '*': ['he\'s dropped {held} — TAKE HIM', '{held}\'s down — light him up!'], nervy: ['{held}! —he\'s— shoot, SHOOT—'] },
  freed: { '*': ['{held}, DOWN — get clear!', 'he\'s loose — {held}, out of the way!'] },
  lost: { '*': ['lost them — where\'d he take {him}?', 'I\'ve lost sight — moving up'], nervy: ['where— where did they go—'] },
  hitHim: { '*': ['…christ— I hit {him}. I hit {held}—'], nervy: ['no— no no no— {held}—'], hard: ['…{held}\'s hit. that\'s on him. keep on him—'] },
};
/** (WORDS) a standoff line as it is said now, by the line it stands for (read in standoffBark, where the pool is built) */
const STANDOFF_NOW: Record<string, string> = {
  '{held}, stay loose — I\'m coming round': '{held}, stay loose — I\'m coming around', '{held}! —I can\'t— I haven\'t got a line—': '{held}! —I can\'t— I don\'t have a shot—',
  'I have him.': 'I\'ve got him.', 'I— I think I have him—': 'I— I think I\'ve got him—',
  'can\'t get round him — hold what you\'ve got': 'can\'t get around him — hold what you\'ve got', 'no way round. {held}, hang on': 'no way around. {held}, hang on',
};
/** … and the moments whose every line names him, for a man in the arm whose own side has no name to say (teams.ts spokenName: a scenario's defender) */
const STANDOFF_NAMELESS: Partial<Record<StandoffMoment, string>> = { dropped: 'he\'s dropped {him} — TAKE HIM', freed: 'he\'s loose — get clear!', hitHim: '…christ— I hit {him}—' };
/** One standoff line from this man if the clocks allow (his own BARK_SECS, the floor's FLOOR_SECS, and never over his own radio call) — `force` skips the two
 *  clocks (the edges: entering, the line opening, the man dropping), 'always' the radio courtesy too (his own round in the man: that gets said); never the line
 *  he said last. */
function standoffBark(g: Game, gd: Guard, moment: StandoffMoment, force: boolean | 'always' = false, held: Guard | null = gd.standoff?.pair ?? null) {
  const S = gd.standoff, now = g.time;
  if (!force && ((S && now - S.lastBarkT < STANDOFF_BARK_SECS) || now - (floorBarkT.get(g) ?? -100) < STANDOFF_BARK_FLOOR_SECS)) return;
  if (force !== 'always' && gd.bubble?.radio && now - gd.bubble.t < 3) return;
  const M = STANDOFF_LINES[moment]; let pool = [...(M[temperamentOf(gd)] ?? []), ...(M['*'] ?? [])];   // (temperamentOf: a staff man's own, a unit's the hard man's under the dialogue pass — the same voice the hold gives him)
  if (held && spokenName(held) === null) { pool = pool.filter(l => !l.includes('{held}')); const l = STANDOFF_NAMELESS[moment]; if (!pool.length && l) pool = [l]; }   // (never with the words switch off)
  pool = wg(pool.map(l => STANDOFF_NOW[l] ?? l), pool);
  if (!pool.length) return;
  let i = Math.floor(simRandom() * pool.length) % pool.length; if (pool.length > 1 && pool[i] === lastBark.get(gd)) i = (i + 1) % pool.length;
  const text = pool[i].replace(/\{held\}/g, held?.callsign ?? 'him').replace(/\{him\}/g, w(held?.civ?.her, undefined) ?? 'him').replace(/\{he\}/g, w(held?.civ?.she, undefined) ?? 'he');
  lastBark.set(gd, pool[i]); if (S) S.lastBarkT = now; floorBarkT.set(g, now);
  g.say(gd, text);
}
/** Side-step / back-pedal along the current nav path (followPath's tolerances and easing, but the body is not turned to it — the standoff keeps his chest to the
 *  pair — and no stuck side-step of its own: notGettingAnywhere watches the leg). True once the path is consumed. */
export function stepAlongPath(g: Game, gd: Guard, dt: number, speed: number): boolean { return crossToRust('stepAlongPath', undefined, [g, gd, dt, speed]) as any; }
/** The friendly-fire line (combat.ts fireWeapon: a colleague's round found the man in Sam's arm): the shooter says so, whatever the clocks. */
export function hostageHitBark(g: Game, shooter: Guard, victim: Guard) { standoffBark(g, shooter, 'hitHim', 'always', victim); }
/** Panel / QA: one line per man in a standoff — distance, degrees off the pair's rear, the held man's clearance off his line, side, what his feet are doing, rounds. */
export function standoffSummary(g: Game): string { return crossToRust('standoffSummary', undefined, [g]) as any; }

export function canSee(g: Game, gd: Guard): { visible: boolean; dist: number; inCone: boolean } { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('canSee', undefined, [g, gd]); if (__r !== RUST_PASS) return __r as any; } } g = viewOfBody(g, gd.char); return canSeeChar(g, gd, g.player.char, true); }   // (the player he has in mind: seats.ts)
/** canSee against any man `pc` — Sam (`isPlayer`: his sandbox invisibility, his down flag, the smoke marched between this man's eye and HIS chest) or an AI man of a
 *  side hostile to this one (teams: alive; the same cone, near field, range and line of sight off the same bones; no smoke term — the probe's segment is to Sam
 *  only — and the magnesium in his eyes hides everybody). One test, so an operative sees a hostile exactly as a guard sees Sam — but for one thing, in a room
 *  fight only: a man with his long gun in the shoulder ALSO sees an AI man through the same eye laid along his aim, its cone ROOMFIGHT.sightsDeg (roomfight.ts
 *  ROOMFIGHT.sights for the teammates, on; sightsFoe for every other side, off). Sam is never read through it. */
export function canSeeChar(g: Game, gd: Guard, pc: Character, isPlayer: boolean): { visible: boolean; dist: number; inCone: boolean } { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('canSeeChar', undefined, [g, gd, pc, isPlayer]); if (__r !== RUST_PASS) return __r as any; } }
  const eye = gd.char.bones.head ?? v3.add(gd.char.pos, [0, 1.65, 0]);
  const look = v3.normalize(v3.lerp(gd.char.chestDir, gd.char.headDir(), 0.5)), E: Eye = { pos: eye, look, range: reachFor(g, gd, pc, isPlayer, eye), halfDeg: GUARD_EYE.halfDeg, nearM: GUARD_EYE.nearM, nearDeg: GUARD_EYE.nearDeg };   // (reachFor: a man standing in light is seen from past `range`; `range` itself otherwise)
  const through = eyeThrough(gd);   // (FOG.mates: a teammate's eye passes the thin things the fog's lines pass)
  let r = eyeContact(g, E, pc, isPlayer, g.time < gd.dazzledUntil || pairGripped(g, gd) || dozeAsleep(gd), gd.smokeTrans, through);   // dazzled: eyes full of magnesium (pairorders.ts: or a teammate's hands on him; doze.ts: or asleep in his chair)
  if (!r.inCone && r.dist < E.range && !isPlayer && (gd.team === 'operative' ? ROOMFIGHT.sights : ROOMFIGHT.sightsFoe) && fightLive(g)) {   // (roomfight.ts ROOMFIGHT.sights: a long gun in the shoulder and level along his aim — he ALSO sees what is along his sights, the same eye laid on the aim)
    const c = gd.char, ax = Math.sin(c.gunYaw), az = Math.cos(c.gunYaw);   // (gunYaw: aimYaw — or, turning under weight, where the gun has got to)
    let lv = c.gunDir[0] * ax + c.gunDir[2] * az;   // how level along his aim the bore lies, in plan
    const lift = c.anim.sightsLift; if (lift > 0) { const gh = Math.hypot(c.gunDir[0], c.gunDir[2]); if (gh > 1e-6) lv *= Math.cos(lift) - c.gunDir[1] / gh * Math.sin(lift); }   // (character.ts MOVES.entryEyes: the entry dip's own pitch put back — the question asked of his gun line as if it were not down past a friend; 0 unless that switch is on and he is dipped)
    if (c.anim.longShown === 'shoulder' && !c.anim.atWell && lv > SIGHTS_COS) { E.look = [ax, 0, az]; E.halfDeg = Math.min(E.halfDeg, ROOMFIGHT.sightsDeg); r = eyeContact(g, E, pc, false, g.time < gd.dazzledUntil || pairGripped(g, gd) || dozeAsleep(gd), gd.smokeTrans, through); }
  }
  if (!isPlayer && r.visible && pairSmoke(g, gd.char, pc) < 0.3) r.visible = false;   // an AI man lost in a plume, by the same line as Sam (roomfight.ts marches the pair's segment; 1 = clear wherever it does not)
  return r;
}
/** a man's eye: 16 m of reach, 58° either side of where chest and head point between them, and inside 3.5 m a near field of 100° (a shape at his elbow is seen) */
export const GUARD_EYE = { range: 16, halfDeg: 58, nearM: 3.5, nearDeg: 100,
  /** A LIT MAN IS SEEN FROM FURTHER (eyeReach below): on in the page (?nofarsight), off here and on the frozen harness stand (TPS_FARSIGHT=1) = every eye stops
   *  at `range` whatever the light, as it was. One rule for every man's eye on every man: a guard's on Sam, a defender's on a teammate, a teammate's on a defender */
  far: false,
  /** A FAR GLIMPSE IS NOT YET A PLACE TO WALK TO (farFaint below; nothing without `far`): on in the page (?nofarcalm), off here and on the frozen harness stand
   *  (TPS_FARCALM=1) = every glimpse past `range` writes his fix the frame it lands, as `far` first shipped */
  farCalm: false,
  /** …the awareness from which a WARY man's glimpse from past `range` is his fix (under it he has not watched long enough to know where) */
  farFixAt: 0.5,
  /** …how far the eye reaches for a man in full light (m); the light on him (his meter, 0‥1) under which the reach is `range` and nothing changes, and from which it is whole */
  farRange: 26, farLo: 0.4, farHi: 0.8,
  /** …and how slowly the notice builds out there: sightGain's distance term runs on from its 0.2 at `range` down to this at `farRange` */
  farK: 0.1 };
/** How far a man's eye reaches for a man with `lit` (his light meter, 0‥1) on him: `range` in the dark — under `farLo` nothing is easier to see than it was — easing
 *  out to `farRange` for a man standing in light (`farHi` and over). `range` with the switch off and under a show. */
export function eyeReach(g: Game, lit: number): number {
  if (!GUARD_EYE.far || g.touring) return GUARD_EYE.range;
  return GUARD_EYE.range + Math.max(0, GUARD_EYE.farRange - GUARD_EYE.range) * smoothstep(GUARD_EYE.farLo, GUARD_EYE.farHi, lit);
}
/** …for the man `pc` in this man's view: Sam by his meter; an AI man by his own (Guard.visibility) or by his lit weapon light pointed at this eye (roomfight.ts
 *  lampGlare) — the light sightGainOn reads, less the goggles' (a tube makes a dark man out, it does not carry further). Nobody is looked up inside the old reach. */
function reachFor(g: Game, gd: Guard, pc: Character, isPlayer: boolean, eye: Vec3): number {
  if (!GUARD_EYE.far || g.touring) return GUARD_EYE.range;
  if (isPlayer) return eyeReach(g, g.player.visibility);
  const t = pc.bones.chest ?? pc.pos; if (Math.hypot(t[0] - eye[0], t[2] - eye[2]) < GUARD_EYE.range - 1) return GUARD_EYE.range;
  const o = g.guards.find(x => x.char === pc); return o ? eyeReach(g, Math.max(o.visibility, lampGlare(g, gd.char, o))) : GUARD_EYE.range;
}
/** the distance term of a sighting's rate: 1 inside 3 m down to 0.2 from 13.4 m — and (GUARD_EYE.far) past the old reach on down to `farK` at `farRange`: a man
 *  is seen out there only standing in light, and the further out, the slower the notice builds. Inside `range` the term is what it always was. */
function distTerm(dist: number): number {
  if (dist > GUARD_EYE.range && GUARD_EYE.far) return lerp(0.2, GUARD_EYE.farK, clamp((dist - GUARD_EYE.range) / Math.max(1e-3, GUARD_EYE.farRange - GUARD_EYE.range), 0, 1));
  return clamp(1 - (dist - 3) / 13, 0.2, 1);
}
/** WHAT THE TEAM'S OWN EYES PASS (col.segmentBlocked's `through`): under FOG.mates, what the fog's lines pass (fogofwar.ts fogThrough: glass, and with
 *  FOG.thin the statics marked thin and free) — what the picture shows through a window's bar, the teammate standing there sees; with the row off,
 *  glass alone, as it was. `teamThrough` = the team's (the player's own line for the squad's net: picture.ts lookSam); `eyeThrough` = a man's, which is
 *  the team's for an operative and glass alone for every other side: no other eye is ever handed the marks. */
export function teamThrough(): number { return FOG.mates ? fogThrough() : BoxFlag.Glass; }
export function eyeThrough(gd: Guard): number { return gd.team === 'operative' ? teamThrough() : BoxFlag.Glass; }
/** …and that man's own line to a point, asked bare (roomfight.ts: which teammate has a line to the man who fired, to a mate's target; the lost-behind-
 *  something line below): is anything his eye stops at between `a` and `b`. With the row off this is col.segmentBlocked(a, b), as each caller had it. */
export function eyeBlocked(g: Game, gd: Guard, a: Vec3, b: Vec3): boolean { return crossToRust('eyeBlocked', undefined, [g, gd, a, b]) as any; }
/** cos 30°: how level along his aim (in plan, the bore's own pitch counted) a shouldered long gun must lie for its owner's eyes to be on its sights */
const SIGHTS_COS = 0.866;
/** An eye — a man's (canSeeChar fills it off his bones) or a device's (devices.ts: a camera's lens): where it is, the unit direction it looks along, how far it
 *  reaches, its half-angle in plan (and the wider near-field one inside `nearM`), and — a lens only — the half-angle in ELEVATION about the look direction's
 *  own pitch (`vHalfDeg`; a man's cone is a flat wedge, floor to ceiling, and leaves it undefined). */
export interface Eye { pos: Vec3; look: Vec3; range: number; halfDeg: number; nearM: number; nearDeg: number; vHalfDeg?: number; }
/** THE line-of-sight test, whoever's eye it is: the target's chest inside the eye's cone and reach (in plan; plus the elevation window when the eye has one), then
 *  nothing solid — statics or a leaf — between the eye and his chest AND his head, and (Sam only) the smoke marched between them not yet opaque (`smokeTrans`, the
 *  probe's transmittance along eye → chest: under 0.3 he is a shape lost in the plume). `blind` = the eye itself is out (magnesium in it; a device says false and
 *  gates on its power instead). Sam's sandbox invisibility and his lying downed hide him from every eye; an AI target only has to be alive. Returns the distance and
 *  whether he was in the cone at all, seen or not, as canSee always did. (fogofwar.ts lineToMan repeats this function's two lines for the team's eyes — his chest,
 *  then his head behind a blocked chest: change the two together.) `through` = the box flags the line passes: glass, and for a teammate's own eye what
 *  `eyeThrough` adds. */
export function eyeContact(g: Game, E: Eye, pc: Character, isPlayer: boolean, blind: boolean, smokeTrans: number, through: number = BoxFlag.Glass): { visible: boolean; dist: number; inCone: boolean } { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('eyeContact', undefined, [g, E, pc, isPlayer, blind, smokeTrans, through]); if (__r !== RUST_PASS) return __r as any; } }
  const tgt = pc.bones.chest ?? v3.add(pc.pos, [0, 1.1, 0]); const tgt2 = pc.bones.head ?? v3.add(pc.pos, [0, 1.6, 0]);
  const smoked = isPlayer && smokeTrans < 0.3;
  // ONE QUESTION TO THE RUST MODULE, and the module's alone (rust/sim/src/sight.rs, through collision.ts eyeContact): the cone and the reach in plan, the
  // elevation window where the eye is a lens, then the two lines, his chest first and his head behind a blocked chest. WHAT HIDES SAM IS READ LAST, of a
  // man the eye would otherwise see, and never before the cone: a game of two seats has a man under every eye of the floor, and "the player" can be asked
  // only where a seat's view was handed in (seats.ts). So the module is told of the eye that is out and of an AI man who is not alive, and Sam's own two
  // are asked here.
  const r = g.col.eyeContact(E, tgt, tgt2, blind || (!isPlayer && !pc.alive), smoked, through);
  if (r.visible && isPlayer && (g.playerInvisible || g.player.down)) r.visible = false;
  return r;
}

// ------------------------------------------------------------ un-stick: a living man standing where the nav grid has no way out
// A man can end up standing where his capsule fits but the bake calls the floor blocked with nothing free BESIDE it either — the hand's breadth between a desk's
// end and a partition that a hold walked him into and then ended in (Sam teleported off him, Sam shot), a shove off a leaf into a corner. A* cannot leave a cell
// whose four neighbours are shut (a diagonal needs both its orthogonals), so every plan from there came back empty: goTo handed followPath nothing, which reads
// as 'arrived' — the patrol ticked through its waypoints on the spot, the search 'looked into' a dozen spots from it, the post man bee-lined the wall — and he
// never walked again (the review lane's heavy grab soak: seed 6, the cubicles man at (22.66, 21.88) from 202 s to the end). So: when a plan is asked for and
// the cell under him is one of those, he first side-steps to the nearest free cell within reach that his body can be walked straight to — on his own feet,
// through the ordinary movement and push-out, at a walk — and plans again from there; genuinely wedged (nothing walkable within reach, or a second on the leg
// without getting nearer) he is stood on the nearest free cell in the room outright, once, with a line to the QA log and not a word. Only a failed plan or
// the frame a hold ends ever asks: a man on a free cell never comes near any of it.
/** REACH: the side-step looks this far for a free cell — the nook that started this is 0.55 m off one, but a sweep of every stranded-yet-standable spot on the
 *  floor (107: corners behind an office chair, the planter by the north wall, the strip between a parked car and the fence…) found the nearest walkable one up
 *  to ~1.9 m off, and two strides walked beat a snap (at 2.4 the sweep walks off 105, snaps 2: the manager's corner boxed in by his chair, and behind the van
 *  out on the lot); STALL: this long stepping without getting nearer and he is snapped; DONE: this near the centre is there (a free cell's centre stands a
 *  0.34 m circle clear, so a 0.3 m man always fits ON it — short of it he may still be over the line in the blocked cell he came from); SNAP_REACH: how far
 *  the snap looks for somewhere to put him; BODY_R: the circle the leg is swept with — half his own: anything nearer the line than that he would walk INTO
 *  (a desk top end-on, a cabinet), anything further off it the push-out slides him round (at his full 0.3 the same sweep snapped 14 that walk fine). */
const UNSTICK_REACH_M = 2.4, UNSTICK_SNAP_REACH_M = 3.5, UNSTICK_BODY_R = 0.15;
/** to: the free cell's centre; best: the nearest he has yet been to it; stallT: seconds OF STEPPING since that last improved (the leg may be dealt on the frame a
 *  hold ends and only walked once his stagger and his CONTACT beat are over — standing is not failing to get there) */
interface UnstickLeg { to: Vec3; best: number; stallT: number; }
const unstickLegs = new SideMap<Guard, UnstickLeg>('guards.unstickLegs');
/** not before this game time does a man with nowhere at all to be put get looked at again (never met on this floor; the guard against a per-frame scan) */
const unstickRetryAt = new SideMap<Guard, number>('guards.unstickRetryAt');
/** men in Sam's arm as of their last update: the first frame back on their own feet gets the stranded check (the hand-back itself lives in player.ts) */
const heldLastFrame = new SideSet<Guard>('guards.heldLastFrame');
/** QA counters (tools/qa: the unstick probe, the soak's smell line): side-steps begun, snaps made, and when / whom the last snap moved (the soak's one-frame
 *  displacement net lets that man through that frame, as it does its own teleports) */
export const UNSTICK = { legs: 0, snaps: 0, lastSnapT: -1, lastSnapWho: null as Guard | null };
rollbackRoot('guards.UNSTICK', UNSTICK);   // (the last snap's man and time are read by the same frame's sweep)
/** Is the nav cell under (x, z) one no plan can leave: blocked itself and all four orthogonal neighbours blocked (the world's edge counting as blocked)? Five
 *  byte reads. A blocked cell with a free neighbour is NOT stranded — A* steps straight into the neighbour and the path's first point is its centre. */
export function strandedAt(g: Game, x: number, z: number): boolean {
  const nav = g.col.nav, cs = nav.cell;
  return nav.isBlocked(x, z) && nav.isBlocked(x + cs, z) && nav.isBlocked(x - cs, z) && nav.isBlocked(x, z + cs) && nav.isBlocked(x, z - cs);
}
/** Can his body be walked straight from `from` to `to`: no leaf across the line, and a circle (UNSTICK_BODY_R) swept along it every 15 cm meets no static in the
 *  movement band (the same knee-to-shoulder band the push-out and the bake use)? A thin ray would thread the slot between two desk backs, or pass under a desk
 *  top between its end panels, and walk him into the wood for a second before the snap; the swept circle answers for what is squarely in his way and leaves
 *  what he merely brushes to the push-out. His own first 15 cm are not asked (he stands where he stands; the frame's push-out has that). */
function legClear(g: Game, from: Vec3, to: Vec3): boolean {
  const col = g.col; const dx = to[0] - from[0], dz = to[2] - from[2]; const L = Math.hypot(dx, dz); if (L < 1e-3) return true;
  if (col.segmentBlockedDynamic([from[0], 0.45, from[2]], [to[0], 0.45, to[2]]) || col.segmentBlockedDynamic([from[0], 1.2, from[2]], [to[0], 1.2, to[2]])) return false;
  const n = Math.ceil(L / 0.15); const q: Vec3 = [0, 0, 0];
  for (let i = 1; i <= n; i++) { const t = i / n; q[0] = from[0] + dx * t; q[1] = 0; q[2] = from[2] + dz * t; if (col.collideCircle(q, UNSTICK_BODY_R, 0.2, 1.5, 1)) return false; }
  return true;
}
/** The free cell centre nearest (as the crow flies) to `from` within `reach` that passes `test`: for the leg ('walk'), one his body can be walked straight to
 *  (legClear — the far side of a partition is near, and no side-step); for the snap ('room'), one in the same room (nothing static across the line at 2 m, over
 *  the furniture and the partitions and under the ceiling: put down past the chair that boxed him in, not through the wall into the next office) — and failing
 *  any in the room, the nearest at all. null: no free cell within reach (or, 'walk', none that walks). */
function freeCellNear(g: Game, from: Vec3, reach: number, test: 'walk' | 'room'): Vec3 | null {
  const nav = g.col.nav, cs = nav.cell; const R = Math.ceil(reach / cs);
  const cx = Math.floor(from[0] / cs), cz = Math.floor(from[2] / cs);
  const cands: { q: Vec3; d: number }[] = [];
  for (let dz = -R; dz <= R; dz++) for (let dx = -R; dx <= R; dx++) {
    const x = cx + dx, z = cz + dz; if (x < 0 || z < 0 || x >= nav.W || z >= nav.H || nav.blocked[nav.idx(x, z)]) continue;
    const q: Vec3 = [(x + 0.5) * cs, 0, (z + 0.5) * cs]; const d = v3.distXZ(from, q); if (d <= reach) cands.push({ q, d });
  }
  cands.sort((a, b) => a.d - b.d);   // nearest first: the first that passes is the one (a handful of tests, once, at the start of a leg)
  if (test === 'walk') { for (const c of cands) if (legClear(g, from, c.q)) return c.q; return null; }
  for (const c of cands) if (!g.col.solidBlocked([from[0], 2.0, from[2]], [c.q[0], 2.0, c.q[2]])) return c.q;   // (solid: a cell across an intact glass pane is another room)
  return cands.length ? cands[0].q : null;
}
/** Put him ON the free cell centre `to` (or the nearest free one in the room / at all, failing that) here and now — the last resort, counted and logged, never spoken. */
function unstickSnap(g: Game, gd: Guard, to: Vec3 | null, why: string) {
  const c = gd.char; const p = to ?? freeCellNear(g, c.pos, UNSTICK_SNAP_REACH_M, 'room') ?? g.col.nav.nearestFreePoint(c.pos[0], c.pos[2]);
  if (!p) return;   // (nothing free within rings of him: not a floor this level has)
  LINE_DEBUG.log?.(gd, `UNSTICK snap ${v3.distXZ(c.pos, p).toFixed(2)} m (${c.pos[0].toFixed(2)},${c.pos[2].toFixed(2)}) → (${p[0].toFixed(2)},${p[2].toFixed(2)}): ${why}`);
  c.pos[0] = p[0]; c.pos[2] = p[2]; c.vel = [0, 0, 0]; gd.speed = 0; gd.lastPos = v3.copy(c.pos); gd.stallRef = null; gd.stuckT = 0;
  UNSTICK.snaps++; UNSTICK.lastSnapT = g.time; UNSTICK.lastSnapWho = gd;
}
/** The side-step he has going, if it still means anything: dropped here (null) once he stands on free floor outside its target cell — carried or shoved there,
 *  or walked off by the fight's own footwork — since a plan from where he is will do and there is nothing at the old spot to walk back for. */
function liveLeg(g: Game, gd: Guard): UnstickLeg | null {
  const U = unstickLegs.get(gd); if (!U) return null;
  if (v3.distXZ(gd.char.pos, U.to) > 0.36 && !g.col.nav.isBlocked(gd.char.pos[0], gd.char.pos[2])) { unstickLegs.delete(gd); return null; }
  return U;
}
/** If he stands stranded (strandedAt) and has no side-step going, begin one: the leg to the nearest reachable free cell — or, nothing free within reach, the snap.
 *  Returns 'leg' (one is under way, new or old: followPath / stepAlongPath walk it before anything else), 'snapped' (he has just been moved: plan afresh from
 *  where he now stands) or 'no' (a free cell under him, or one beside it: nothing to do — the common case, five byte reads). */
function unstickBegin(g: Game, gd: Guard): 'leg' | 'snapped' | 'no' {
  const c = gd.char;
  if (liveLeg(g, gd)) return 'leg';
  if (gd.state === 'dead' || gd.held || !strandedAt(g, c.pos[0], c.pos[2]) || g.time < (unstickRetryAt.get(gd) ?? -1)) return 'no';
  const to = freeCellNear(g, c.pos, UNSTICK_REACH_M, 'walk');
  if (!to) {
    unstickSnap(g, gd, null, 'stranded, no free cell within reach he can walk to');
    if (strandedAt(g, c.pos[0], c.pos[2])) { unstickRetryAt.set(gd, g.time + 2); return 'no'; }   // (nowhere within rings of him to put him either — no floor this level has, but a plan failing every frame must not scan for one every frame)
    return 'snapped';
  }
  unstickLegs.set(gd, { to, best: v3.distXZ(c.pos, to), stallT: 0 }); UNSTICK.legs++;
  LINE_DEBUG.log?.(gd, `UNSTICK leg from (${c.pos[0].toFixed(2)},${c.pos[2].toFixed(2)}) to the free cell at (${to[0].toFixed(2)},${to[2].toFixed(2)}), ${v3.distXZ(c.pos, to).toFixed(2)} m`);
  return 'leg';
}

export function goTo(g: Game, gd: Guard, goal: Vec3, force = false) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('goTo', undefined, [g, gd, goal, force]); if (__r !== RUST_PASS) return __r as any; } }
  if (!force && gd.pathGoal && v3.dist(gd.pathGoal, goal) < 0.75 && gd.path.length) return;
  const U0 = liveLeg(g, gd);
  if (U0) { gd.path = [v3.copy(U0.to)]; gd.pathI = 0; gd.pathGoal = v3.copy(goal); gd.repathT = 0; gd.lineSide = 0; return; }   // a side-step off a stranded cell under way: it is finished first (a couple of seconds at most), the goal kept for the plan that follows it (unstickFirst)
  // no route (the fix is inside a wall / a sealed pocket): only fall back to a bee-line when the straight line is actually WALKABLE for a man his width — not merely
  // clear to a thin ray: a ray threads the hand-wide slot between the server racks that his shoulders (just) squeeze through and the nav bake rightly calls shut,
  // and a man bee-lined through it after a noise was marooned in the strip behind the racks for the rest of the night, no route back to anything — otherwise
  // an empty path, which followPath reports as 'arrived' so the state machine moves on (search here) instead of grinding a partition for the rest of the encounter
  const pen = lanePenalty(g, gd, tacticsPenalty(g, gd, goal));   // (pursuit tactics, tactics.ts: kill-zone doorways on his side and a flanker's dear corridor priced into the search — undefined, i.e. plain A*, for nearly everybody and always with tune.tactics off)
  const round = teamAtWalk(g, gd) ? propPenalty(g, gd, pen) : pen;   // (ORDERS.propRound, orders.ts: a teammate at the walk goes round a chair where that costs him little — `pen` itself with the rule off or the fight on his legs, and then the one search below is the search it was)
  const plan = (): Vec3[] | null => { const from = gd.char.pos; let p = g.col.nav.findPath(from, goal, 6000, pen); if (p && round !== pen) p = propWay(g, from, p, g.col.nav.findPath(from, goal, 6000, round)); if (p) return p; return g.col.nav.walkable(from, goal) && !g.col.solidBlocked([from[0], 0.5, from[2]], [goal[0], 0.5, goal[2]]) ? [v3.copy(goal)] : null; };
  let p = plan();
  if (!p) {   // …unless there is no route because no plan can leave the cell he stands on (un-stick, above): side-step off it first — or, snapped clear of it just now, plan from there
    const u = unstickBegin(g, gd);
    if (u === 'leg') { gd.path = [v3.copy(unstickLegs.get(gd)!.to)]; gd.pathI = 0; gd.pathGoal = v3.copy(goal); gd.repathT = 0; gd.lineSide = 0; return; }
    if (u === 'snapped') p = plan();
  }
  gd.path = g.doors.threadPath(p ?? [], gd.char.pos, (x, z) => !g.col.nav.isBlocked(x, z)); gd.pathI = 0; gd.pathGoal = v3.copy(goal); gd.repathT = 0;
  gd.lineSide = 0;   // (whatever this path is, it is not the fight's side-step round a colleague — lineStep knows its own by that)
}

/** HOW THE TEAM CARRIES ITSELF. A teammate (an operative unit, while the room fight is live — a scenario, never a showcase's cast) is a man with a rifle and a
 *  plate carrier: `max` is the most his legs are ever asked for, whoever asks (an order's leg, a drill's flow, the fight AI's chase, a step off a doorway) — a
 *  purposeful walk with some jog in it, none of the sprint clip (character.ts BLEND: the walk carries to 1.6 m/s, the sprint starts past 2.7) — and he LEANS INTO a
 *  move: his speed rises by no more than `accel` m/s² (he stops as crisply as ever: a man arriving on a mark is on it). And his GUN TRAVELS: the runner
 *  (squad.ts) put a scripted man's muzzle on its aim point the frame the point changed — an order given, a corner checked, the fight's own read taking over
 *  from behind his shoulder: a rifle, a head and a weapon light's cone through 60‥150° in one frame, a few times a minute per man (tools/qa/playthrough.ts
 *  counted them) — now it comes round to a point an ORDER (or the unasked cover) gives it at `turn` rad/s at most (0 = on the point at once, as it was): 8 is
 *  the operatives' own body turn (roomfight.ts FIGHT_SKILL.operative.turn) — a quarter turn in a fifth of a second. A point his own EYES have given him
 *  (GuardScript.over: the room fight's read, which eases itself — roomfight.ts point()) is not slowed: throttled too, the pair lost three more rooms in six.
 *  What that point may not do is TELEPORT: when it passes behind his back the waist's twist clamp flips from one shoulder to the other, 150° and more in a
 *  frame — that crossing is swung through his front at `swing` rad/s (0 = the flip, as it was; 30 is a half turn in a tenth of a second).
 *  `on` false = everybody as they were. */
export const PACE = { on: true, max: 2.2, accel: 2.6, turn: 8, swing: 30 };
/** LIGHT DISCIPLINE (the maintainer, 10-05, off the merged build: "friendlies also need to have flashlight discipline"): a teammate under one of Sam's orders in
 *  a live room fight burns his weapon light only while it helps — while he SEARCHES (the order sent him into a room or has him slicing one from outside, and
 *  he has not yet stood on his mark with nothing new to look at for `afterSecs`), while his own eyes have his gun (roomfight's read: a man seen, a look at a
 *  shot or a call), while the fight AI has him, and for `hotSecs` after a shot, a charge or a boot last reached his ears (never that last while he is posted
 *  on a door he has not gone through: light under a door is the knock), for `stepSecs` after an ENEMY's footfall reached them along open air (roomfight's
 *  Op.stepT — a man holding a room in the dark, or stacked with his back to a corridor, hears the man coming and lights him), and while he WALKS SOMEWHERE ON HIS OWN — to a mark an order gave him, more than
 *  `onM` from it yet, or back to Sam from more than `aloneM` away: a man's eyes are what his light is on (sightGainOn), and a pair sent down a dark corridor
 *  to a door with their lights off walked up to the man standing in it and were shot at the stack (rescue-probes, seed 14). In file behind Sam, on his mark,
 *  stacked on a door, holding a corridor: dark (it was: on from the first entry and never off again — on the next door's stack, in every file after).
 *  `tail` keeps it from flickering. Off (`on` false): the light as the order's flag and the unit's switch have it, as it was. Men a drill or the rescue's flow
 *  moves (no order of Sam's on them) are not ruled: their lights are the drill's. */
export const LIGHTS = { on: true, afterSecs: 2.5, hotSecs: 5, stepSecs: 4, tail: 0.6, onM: 0.8, aloneM: 7,
  /** a man on his mark with his gun up who stands LIT himself (his own meter ≥ lookLit) and looks into the DARK (the floor along his gun, out to lookM or the
   *  first wall, under lookDark with the team's own lights left out) burns his light: the dark hides only the other side. Off = dark on every hold, as it was —
   *  four men holding a lit room were shot from a dark doorway eight metres off by a man none of them could see. SHIPS OFF, a row to try: it keeps a man
   *  holding a taken room lit for as long as he holds (the hold is meant to be dark, and a lit man is seen), it reads the floor by the CPU's direct-light
   *  estimate (no bounce, no moon unless sense.ts SENSE.moon) where everything else a man does by light goes by the meter, and two lit mates can keep each other's lights on */
  lookDark: false, lookLit: 0.25, lookBelow: 0.12, lookM: 10,
  /** THE PLACE DECIDES (the footage: lights off in lit halls and rooms whatever the men are doing, on in the dark): a man who stands on lit floor with lit
   *  floor all along his gun — every metre out to `litM` (his light's reach) or the first wall, and every third metre down both edges of its cone — whose man,
   *  if his eyes have one, stands lit, and whose mark, if he is within `litMarkM` of one he is walking to, is lit: every sample at `litAt` or brighter on the
   *  meter's scale, nothing anybody carries counted. He keeps his light off for every reason above: it shows him nothing the room's lights do not, and pointed
   *  at an enemy's eyes it shows the enemy him (roomfight.ts lampGlare). One dark sample — a doorway onto a dark room, the corner a metre inside it, the pod an
   *  entry is sending him to — and the rule above has it again. `litAt` 0.5 is where a standing man is a sighting to cold eyes at nine metres by the room's
   *  light alone (sightGainOn); under it the weapon light is what shows him. Read off the CPU's direct-light estimate (no bounce: it errs dark, so toward
   *  the light on). false = lit or dark, the same rule, as it was */
  litOff: true, litAt: 0.5, litM: 15, litMarkM: 6,
  /** DARK ON A LIT MAN (one switch, two halves — each measured to cost men without the other, so there is no row for either alone): while his eyes have a
   *  man who stands lit by the place itself (`litAt` or brighter; nothing anybody carries and no flash, tracer, lightning or turning beacon counted) and he himself stands in the dark (under
   *  `litAt` the same way), (1) his weapon light stays off: the man is plain to see without it, and from the dark the lens is the one thing that would show
   *  the man where he is (roomfight.ts lampGlare); and (2) if he is WALKING on an order with Sam in his line to that man — his gun down for it — his feet wait
   *  where he is, in the dark, until the line is clear (roomfight.ts holdShort, ROOMFIGHT.holdShortSecs at most a hold; only a walk that CLOSES on the man —
   *  an order that takes him away from him is walked) instead of carrying him, gun lowered, up into the man's light. The rule above (`litOff`) wants his own floor lit too; this is the other side of it: a man in a dark room with an enemy under a
   *  lamp outside the door shoots from the dark. Asked four times a second per man with a man in his eyes (or, for the light alone, out of them for under a
   *  second and a half: the blink roomfight.ts keeps his aim through — the walk is held only with the man in his eyes); the light is the other
   *  rules' again the frame he has no such man, the walk within a quarter of a second. The estimate is the CPU's direct light with no bounce and no moon (unless sense.ts SENSE.moon): it errs toward calling HIM dark. No range limit, by measurement: held to the nine metres `litAt` is a sighting at, his light came up from the dark on the far man and the lens gave him away (624 against 653 of 864 teammates up over 432 night breaches). false = as it was: sent on
   *  past Sam at a man under a lamp, his light came up with the word and he walked at him with his gun down (the night breach's last hunt) */
  litMan: true,
  /** NOT INTO THE WALL AT HIS MUZZLE (wallDark, below, has the rule): `wallOff` the switch — off here, on in the page (host/profile.ts; ?walllight=0 puts it
   *  off) · `wallM` how near the whole beam must end for the light to go dark, `wallBackM` how far any ray must run clear for it to burn again, `wallSecs`
   *  how long the beam must have ended that near before it goes dark */
  wallOff: false, wallM: 1.5, wallBackM: 1.9, wallSecs: 0.3 };
const litSeen = new SideMap<Guard, { t: number; lit: boolean; A: unknown; tgt: unknown }>('guards.litSeen');
/** is where he stands, what his gun is on, the man he has and the mark he is nearly at lit by the place itself (LIGHTS.litOff; asked twice a second per man,
 *  and again at once on a new order or a new man) */
function litPlace(g: Game, gd: Guard, A: ReturnType<Game['orders']['of']>, tgt: Guard | null, pt: Vec3 | null): boolean {
  if (!LIGHTS.litOff) return false;
  const key = tgt ?? (pt ? 'pt' : null), was = litSeen.get(gd); if (was && was.A === A && was.tgt === key && g.time - was.t >= 0 && g.time - was.t < 0.5) return was.lit;
  const c = gd.char, eye: Vec3 = [c.pos[0], 1.5, c.pos[2]];
  const carried = () => true;   // (the place's own lights only: nothing anybody carries)
  const dark = (x: number, z: number) => g.visibilityOf(lightEstimateAt(g, [x, 1.2, z], carried)) < LIGHTS.litAt;
  let lit = !dark(c.pos[0], c.pos[2]);
  if (lit && tgt) lit = !dark(tgt.char.pos[0], tgt.char.pos[2]);
  if (lit && pt) lit = !dark(pt[0], pt[2]);
  if (lit && A?.post && !A.arrived && v3.distXZ(c.pos, A.post) < LIGHTS.litMarkM) lit = !dark(A.post[0], A.post[2]);
  for (const [off, step] of [[0, 1], [-0.2, 3], [0.2, 3]]) { if (!lit) break; const fx = Math.sin(c.aimYaw + off), fz = Math.cos(c.aimYaw + off);
    for (let r = step; r <= LIGHTS.litM + 1e-6; r += step) { const P: Vec3 = [eye[0] + fx * r, 1.2, eye[2] + fz * r]; if (g.col.segmentBlocked(eye, P)) break;
      if (dark(P[0], P[2])) { lit = false; break; } } }
  litSeen.set(gd, { t: g.time, lit, A, tgt: key }); return lit;
}
const darkSeen = new SideMap<Guard, { t: number; dark: boolean }>('guards.darkSeen');
/** is what his gun is on dark, him lit (LIGHTS.lookDark; asked twice a second per man) */
function looksIntoDark(g: Game, gd: Guard, A: ReturnType<Game['orders']['of']>): boolean {
  if (!LIGHTS.lookDark || !A || !A.arrived || !A.post || gd.muzzleDown || A.low || gd.visibility < LIGHTS.lookLit) return false;
  if (!(A.search || A.order === 'hold' || A.order === 'cover') || gd.script?.upper === 'none') return false;   // (men an entry left in a room, men Sam posted: never a stack, never the file)
  const was = darkSeen.get(gd); if (was && g.time - was.t >= 0 && g.time - was.t < 0.5) return was.dark;
  const c = gd.char, eye: Vec3 = [c.pos[0], 1.5, c.pos[2]], fx = Math.sin(c.aimYaw), fz = Math.cos(c.aimYaw); let dark = false;
  const ours = (owner: number) => g.guards.some(x => x.char.id === owner && x.team === 'operative') || owner === g.player.char.id;
  for (let r = 4; r <= LIGHTS.lookM + 1e-6; r += 3) { const P: Vec3 = [eye[0] + fx * r, 1.2, eye[2] + fz * r]; if (g.col.segmentBlocked(eye, P)) break;
    if (g.visibilityOf(lightEstimateAt(g, P, ours)) < LIGHTS.lookBelow) { dark = true; break; } }
  darkSeen.set(gd, { t: g.time, dark }); return dark;
}
/** what the rule itself last made of his place, while that reading is fresh (QA: the probes judge a dark light by it) */
export function placeLitSeen(g: Game, gd: Guard): boolean { const s = litSeen.get(gd); return LIGHTS.litOff && !!s && g.time - s.t >= 0 && g.time - s.t < 0.5 + 1e-6 && s.lit; }
const litUntil = new SideMap<Guard, number>('guards.litUntil');
const WALL_BREAK = (globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.WALLLIGHT_BREAK === '1';   // (QA: the rule's own check must fail with this set)
const wallSeen = new SideMap<Guard, { t: number; since: number; dark: boolean; out?: boolean }>('guards.wallSeen');   // (`out`: the reading put out a light that was otherwise burning — poseFlashlight says so)
/** seven directions across a beam about `d` (game.ts tune.flashOuter is 19 degrees): its axis, four turned about the vertical, one tipped up and one down */
export function beamFan(d: Vec3): Vec3[] {
  const out: Vec3[] = []; for (const a of [0, -0.17, 0.17, -0.33, 0.33]) { const c = Math.cos(a), k = Math.sin(a); out.push([d[0] * c + d[2] * k, d[1], d[2] * c - d[0] * k]); }
  for (const e of [-0.3, 0.3]) out.push(v3.normalize([d[0], d[1] + e, d[2]])); return out;
}
const FLOOR_BREAK = (globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.WALLFLOOR_BREAK === '1';   // (QA: the ground is something solid again, as it was; the checks of the reading below must fail with this set)
const FLOOR_TOP = BAKE_Y - 1e-3;   // (a hair under the top of the ground's band: a thing that tops out AT the band's top is in the way, as it is to the walk grid's bake)
/** WHAT ONE RAY OF A BEAM ENDS ON inside `m` of the lens: 3 = something that stands, met above the ground's band · 2 = the foot of such a thing, met inside
 *  the band · 1 = the ground (it is down in the band inside `m` with nothing standing in the rest of its way) · 0 = nothing. The ground's band is what the walk grid calls floor (scene/collision.ts BAKE_Y: whatever tops out under it, feet
 *  cross): the ray is followed as it flies while it is above the band, and from there to where it meets the ground along the band's top, where a wall, a
 *  leaf or a crate is in the way down to its foot and the slab, a kerb or a blast's rubble is not. */
function rayEnd(g: Game, o: Vec3, r: Vec3, m: number): 0 | 1 | 2 | 3 {
  const dip = r[1] < 0 ? Math.max(0, (o[1] - FLOOR_TOP) / -r[1]) : Infinity, a = Math.min(m, dip);   // (how far along it the ray is above the band)
  if (a > 0 && g.col.segmentBlocked(o, [o[0] + r[0] * a, o[1] + r[1] * a, o[2] + r[2] * a])) return 3;
  if (dip >= m) return 0;
  const run = Math.min(m, o[1] / -r[1]) - dip, p: Vec3 = [o[0] + r[0] * dip, FLOOR_TOP, o[2] + r[2] * dip];
  if (run > 0 && g.col.segmentBlocked(p, [p[0] + r[0] * run, FLOOR_TOP, p[2] + r[2] * run])) return 2;
  return 1;   // (down in the ground's band inside `m`: it has ended on the floor, a few centimetres of lens height either way)
}
/** IS A BEAM INTO A WALL — the one reading the two wall rules below both ask, of the beam from `o` across `fan` (its axis first) at distance `m`: the
 *  MIDDLE of the beam ends on something that stands inside `m`, and no ray of it runs clear to `m`. THE FLOOR IS NOT A WALL: a ray that comes down to the
 *  ground is neither a wall nor clear, so a lowered gun over open floor (its middle on the ground) burns, and the same gun a step from a wall (its middle
 *  on the wall, its lower edge on the floor at the wall's foot) is into the wall. `held` = it read as a wall a moment ago (the rules ask their far distance
 *  then): the middle still counts while it is on the thing's FOOT, inside the ground's band; to start, it must be on it above the band. That is the same
 *  give for a lowered beam, whose middle moves between wall and floor with no change of distance, as the two distances are for a level one — a man who
 *  sways at the edge does not blink. (Before: the ground counted as "something solid" and every ray had to end inside `m`. A beam at low ready meets the
 *  ground 1.50 to 1.55 m from the lens, between a weapon light's two distances, so one brush past a wall kept a teammate dark for the rest of his walk,
 *  in the open.) */
function beamWalled(g: Game, o: Vec3, fan: Vec3[], m: number, held: boolean): boolean {
  if (FLOOR_BREAK) return fan.every(r => g.col.segmentBlocked(o, [o[0] + r[0] * m, o[1] + r[1] * m, o[2] + r[2] * m]));
  for (let i = 0; i < fan.length; i++) { const e = rayEnd(g, o, fan[i], m); if (e === 0 || (i === 0 && !(e === 3 || (held && e === 2)))) return false; }
  return true;
}
/** …asked of a beam laid by hand, from `o` along `d` — and what one ray of one ends on (QA: the lights' checks read beams ray by ray with the rule's own
 *  line test, at a margin either side of its distances: tools/qa/beam-reading.ts) */
export function beamWalledAt(g: Game, o: Vec3, d: Vec3, m: number, held = false): boolean { return crossToRust('beamWalledAt', undefined, [g, o, d, m, held]) as any; }
export function beamRayEnd(g: Game, o: Vec3, r: Vec3, m: number): 0 | 1 | 2 | 3 { return crossToRust('beamRayEnd', undefined, [g, o, r, m]) as any; }
/** where a carried light will burn: at its lens `p`, or this side of the wall the lens is carried through from `held` (attachFlashlight, lighting.ts lensClear) */
function burnsFrom(g: Game, held: Vec3, p: Vec3): Vec3 { return g.col.segmentBlocked(held, p) ? lensClear(g, held, p, g.engine.settings.flashlightRadius) : p; }
/** …a living man's weapon light, for the checks (nothing before he has a pose) */
export function beamLens(g: Game, gd: Guard): Vec3 | null { return crossToRust('beamLens', undefined, [g, gd]) as any; }
/** LIGHTS.wallOff (the maintainer, 10-22, off the first tutorial mission: "weird that they are shining the flashlight on the wall immediately in front of
 *  them"): a teammate's weapon light is dark while its beam is into something that stands within `wallM` of the lens — the wall he walks up to a stack
 *  along, the truck he kneels behind; never the ground (beamWalled, above: a gun at low ready over open floor lights the floor ahead of him, and burns). Such a light shows him nothing and is the brightest thing in the picture. Asked of the beam as posed
 *  this frame (the lens — where it will burn: this side of a wall it is pressed through — and where the gun points): the axis must end on it and
 *  none of the six rays across the cone may run clear of `wallM` — a jamb met edge-on with the room open beyond it to part of the cone is not a wall, and that light
 *  burns. It goes dark once that has held `wallSecs` (a muzzle swept past a corner does not blink) and burns again the frame any ray runs clear to
 *  `wallBackM` (the door opens: the room is lit at once). The other rules still decide whether he wants a light at all; this one only says it would be
 *  wasted — and with anybody who is not of his side standing in the beam this side of the wall it is not: that light burns. Whoever is not one of Sam's
 *  team is never asked. Off: the light as those rules have it, as it was. */
function wallDark(g: Game, gd: Guard, held: Vec3 | undefined): boolean {
  if (!LIGHTS.wallOff || WALL_BREAK || gd.team !== 'operative' || !gd.unit || gd.state === 'dead' || !held) return false;
  const p = gd.flashlight.pos, d = gd.flashlight.dir, t = g.time;
  const o = burnsFrom(g, held, p), fan = beamFan(d);   // (a lens carried through the wall he stands at lights from this side of it — attachFlashlight, lighting.ts: asked from where it will burn)
  const hl = Math.hypot(d[0], d[2]) || 1, manIn = (m: number) => g.guards.some(x => { if (x === gd || x.state === 'dead' || x.team === 'operative') return false;   // (somebody not of his side in the beam, this side of what it ends on: a light with a man in it is not wasted — the walls are asked of the statics only)
    const vx = x.char.pos[0] - o[0], vz = x.char.pos[2] - o[2], l = Math.hypot(vx, vz); return l < m + 0.5 && (l < 0.3 || (vx * d[0] + vz * d[2]) / (l * hl) > 0.9); });
  const ends = (m: number, kept: boolean) => beamWalled(g, o, fan, m, kept) && !manIn(m);
  const was = wallSeen.get(gd), s = was && t >= was.t && t - was.t <= 0.25 ? was : { t, since: t - LIGHTS.wallSecs, dark: false };   // (no reading a moment ago — a fresh man, a clock put back: this frame's reading stands at once)
  const near = ends(s.dark ? LIGHTS.wallBackM : LIGHTS.wallM, s.dark), since = near ? s.since : t, dark = near && (s.dark || t - since >= LIGHTS.wallSecs - 1e-9);
  wallSeen.set(gd, { t, since, dark }); return dark;
}
const TORCH_WALL_BREAK = (globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.TORCHWALL_BREAK === '1';   // (QA: the rule's own check must fail with this set)
const torchLowSeen = new SideMap<Guard, { t: number; low: boolean }>('guards.torchLowSeen');
/** TORCH.wallLow (calmtorch.ts; the maintainer, 10-22, off the stealth tutorial: "another example of character shining flashlight at a small point in front
 *  of them. feels a bit weird but i'm not sure what the natural thing is here"): a man carrying a HAND torch — a patrol, a man gone to look — whose beam, as
 *  the relaxed carry would point it (his heading and chest, his aim's pitch), would end whole on something that stands within `wallM` of his hand (beamWalled: the ground is not a wall) has nothing in
 *  front of him to light: he lets the torch hang, its beam on the floor at his feet, until the carry runs clear to `wallBackM` again (he has turned, or
 *  walked on). Asked of the carry's own direction, never of the tipped beam (which always ends on the floor), so the answer does not chase itself. A weapon
 *  light is not a hand torch and is never asked: it points where the gun points. Off: the carry as it was. */
function torchLow(g: Game, gd: Guard): boolean {
  if (!(TORCH.wallLow || TORCH.wallDark) || TORCH_WALL_BREAK) return false;
  const c = gd.char, hand = c.bones.handR, t = g.time; if (!hand) return false;
  const d = torchCarryWant(c), was = torchLowSeen.get(gd), low0 = !!was && t >= was.t && t - was.t <= 0.25 && was.low, m = low0 ? TORCH.wallBackM : TORCH.wallM;
  const low = beamWalled(g, hand, beamFan(d), m, low0);
  torchLowSeen.set(gd, { t, low }); return low;
}
/** what LIGHTS.wallOff last made of his beam, while that reading is fresh (QA: the probe judges the rule by it) */
export function wallDarkSeen(g: Game, gd: Guard): boolean { const s = wallSeen.get(gd); return LIGHTS.wallOff && !!s && g.time - s.t >= 0 && g.time - s.t <= 0.25 && s.dark; }
/** …and did that reading put OUT a light the other rules had burning (QA: a check that excuses a dark man for the wall at his muzzle excuses only him — the
 *  rule's clock runs on a man whose light is off for any other reason too) */
export function wallPutOut(g: Game, gd: Guard): boolean { return wallDarkSeen(g, gd) && !!wallSeen.get(gd)?.out; }
/** is this man's weapon light this rule's to decide */
export function lightRuled(g: Game, gd: Guard): boolean { return LIGHTS.on && gd.team === 'operative' && !!gd.unit && fightLive(g) && !!orderOf(g, gd); }
const litManSeen = new SideMap<Guard, { t: number; tgt: Guard; v: boolean }>('guards.litManSeen');
/** LIGHTS.litMan: the man his eyes have stands lit by the place, he himself in the dark */
function darkOnLitMan(g: Game, gd: Guard, tgt: Guard): boolean {
  const was = litManSeen.get(gd); if (was && was.tgt === tgt && g.time - was.t >= 0 && g.time - was.t < 0.25) return was.v;
  const carried = () => true, lit = (p: Vec3) => g.visibilityOf(lightEstimateAt(g, [p[0], 1.2, p[2]], carried, true)) >= LIGHTS.litAt;   // (steady lights only: the flash of a round landing by him — nobody's light, a twentieth of a second — lit him 'by the place', and the team's lights went dark on a man standing in the dark)
  const v = lit(tgt.char.pos) && !lit(gd.char.pos);
  litManSeen.set(gd, { t: g.time, tgt, v }); return v;
}
/** LIGHTS.litMan, asked of a teammate the light rule has: is he, this frame, in the dark on a man who stands lit (the man he is on, or was until a blink ago:
 *  roomfight.ts keeps his aim through that — a jamb passed, a leaf swung — and his light does not come back in it, unless something else has turned him) */
export function gunOnLitMan(g: Game, gd: Guard): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('gunOnLitMan', undefined, [g, gd]); if (__r !== RUST_PASS) return __r as any; } }
  if (!LIGHTS.litMan || !lightRuled(g, gd)) return false;
  const O = g.fight.opOf(gd), t = g.time; if (!O) return false;
  const man = O.tgt ?? (!O.turn && gd.script && O.kept && O.kept.h.state !== 'dead' && t - O.kept.t >= 0 && t - O.kept.t < 1.5 ? O.kept.h : null);   // (not while a cue has him turning to somebody else, nor once the fight is the alert AI's: the light is the other rules' then)
  return !!man && darkOnLitMan(g, gd, man);
}
/** does it burn this frame (asked only of a man the rule has: lightRuled) */
export function teamLit(g: Game, gd: Guard): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('teamLit', undefined, [g, gd]); if (__r !== RUST_PASS) return __r as any; } }
  if (gunOnLitMan(g, gd)) return false;
  const t = g.time, A = g.orders.of(gd), O = g.fight.opOf(gd);
  const want = (!gd.script && (gd.state === 'alert' || gd.state === 'search')) || !!gd.script?.over || (!!O && (!!O.tgt || !!O.turn || (t - O.hotT < LIGHTS.hotSecs && !(A && A.door && !A.search))))
    || (!!A && !!A.search && !(A.arrived && t - Math.max(A.arrT, A.searchT ?? -Infinity) > LIGHTS.afterSecs))
    || (!!O && t - O.stepT < LIGHTS.stepSecs)
    || (!!A && (A.post ? v3.distXZ(gd.char.pos, A.post) > LIGHTS.onM : v3.distXZ(gd.char.pos, g.player.char.pos) > LIGHTS.aloneM))
    || looksIntoDark(g, gd, A);
  if (want && !litPlace(g, gd, A, O?.tgt ?? null, O?.turn?.pt ?? null)) litUntil.set(gd, t + LIGHTS.tail);
  const u = litUntil.get(gd); return u !== undefined && t < u && u - t <= LIGHTS.tail + 1e-6;   // (a stamp from a clock since put back is no reason)
}
/** is this man one of the team whose pace PACE governs right now */
export function teamPaced(g: Game, gd: Guard): boolean { return PACE.on && gd.team === 'operative' && !!gd.unit && fightLive(g); }
/** the speed his legs may be asked for (PACE.max for the team; anybody else: as asked) */
export function teamPace(g: Game, gd: Guard, speed: number): number { return crossToRust('teamPace', undefined, [g, gd, speed]) as any; }   // (… and a SILENT squad's walk under that: silent.ts)
/** the speed his legs GET for a leg asked at `speed` (already teamPace's): × what his card weighs (kitload.ts) — on top of the caps, which so scale with the
 *  man (a light man is the quieter one too). teamPace is a cap and may be asked twice; this is a factor — ONE call a leg, where the leg is walked
 *  (followPath, stepAlongPath, a scripted man's strafe and settle). As asked for a man with nothing weighed on him */
export function teamLegs(g: Game, gd: Guard, speed: number): number { return goneWithTick('teamLegs') as any; }   // (teampace.ts: at a run a card weighs as it does on the player's sprint — null with its switch off)

export function followPath(g: Game, gd: Guard, dt: number, speed: number): boolean { return crossToRust('followPath', undefined, [g, gd, dt, speed]) as any; }


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
const doorCellsOf = new GameSideMap<Game, Map<number, Door>>('guards.doorCellsOf');
export function doorCells(g: Game): Map<number, Door> {
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




/** Score every spot for THIS man now — dark × cover × plausible, eased toward the ones the shorter walk from him (`walk`: metres of his own path to each cell,
 *  from a keyed flood at pick time; straight-line otherwise — equal spots resolve near-first, a much darker one across the room still wins) — folding in what the
 *  other searchers know: a spot a colleague has looked into (or one within a metre of it) is done for everybody. */
export function scoreSpots(g: Game, gd: Guard, P: SearchPlan, walk: Float32Array | null = null): void { return crossToRust('scoreSpots', undefined, [g, gd, P, walk]) as any; }
/** One line for the panel / debugging: phase, the spot he is on, how many looked into, the top few by score with their readings. */
export function searchSummary(g: Game, gd: Guard): string {
  const P = gd.searchPlan; if (!P) return '';
  const cur = P.current >= 0 ? P.spots[P.current] : null;
  const top = [...P.spots].filter(s => !s.visited && !s.unreachable).sort((a, b) => b.score - a.score).slice(0, 3).map(s => `${s.kind}@${s.pos[0].toFixed(1)},${s.pos[2].toFixed(1)} d${s.dark.toFixed(2)}${s.E === undefined ? '?' : ''} c${s.cover.toFixed(2)} p${s.plaus.toFixed(2)} =${s.score.toFixed(3)}`).join(' · ');
  return `search ${P.phase}${cur ? ` → ${cur.kind}@${cur.pos[0].toFixed(1)},${cur.pos[2].toFixed(1)}` : ''} · ${P.visits}/${P.spots.length} looked into · ${Math.round(g.time - P.t0)} s since the fix · next: ${top || '—'}`;
}

export interface Pieing {
  door: Door; phase: 'approach' | 'slice' | 'cross';
  side: 1 | -1;        // the face he comes at it from (+1 = the CCW side of the closed frame line, Doors.side's convention)
  at: Vec3;            // where his path cuts the frame line (distances are taken to this, not to the line: a man walking the corridor a metre off the wall is not a metre from the door)
  near: 0 | 1;         // the jamb on his side of the approach — 0 the hinge, 1 the latch — settled as the slice begins: the slice runs from it across to the far corner, the blind corner behind it gets the check
  t: number; t0: number; pathRef: Vec3[]; bent: boolean;
}
/** the closed frame of a door: hinge, unit along the frame hinge → latch, unit normal (+ = CCW side), width */
export function frameOf(d: Door) { const ux = Math.cos(d.def.closedDir), uz = Math.sin(d.def.closedDir); return { hx: d.def.hinge[0], hz: d.def.hinge[1], ux, uz, nx: -uz, nz: ux, W: d.def.width }; }
/** where segment a→b crosses d's frame line within the opening: t along the segment, the side a is on, the crossing point — or null */
export function crossesDoor(a: Vec3, b: Vec3, d: Door): { t: number; side: 1 | -1; at: Vec3 } | null {
  const F = frameOf(d);
  const da = (a[0] - F.hx) * F.nx + (a[2] - F.hz) * F.nz, db = (b[0] - F.hx) * F.nx + (b[2] - F.hz) * F.nz;
  if (da * db >= 0 || Math.abs(da - db) < 1e-6) return null;
  const t = da / (da - db); const ix = a[0] + (b[0] - a[0]) * t, iz = a[2] + (b[2] - a[2]) * t; const along = (ix - F.hx) * F.ux + (iz - F.hz) * F.uz;
  if (along < -0.1 || along > F.W + 0.1) return null;
  return { t, side: da > 0 ? 1 : -1, at: [ix, 0, iz] };
}




/** The CPU's estimate of the renderer's light meter at world point P (probe.wgsl: the best, over the six axis normals, of the luminance of the SHADOWED DIRECT
 *  irradiance) — every enabled analytic light the engine holds this frame (fixtures' area lights, lamps, torches and weapon lights, the beacons, muzzle flashes)
 *  through the shader's own terms (evalLight: 1/max(d², r²), the range window, the spot cone's smoothstep, the panel's emitter cosine, the receiving cosine),
 *  shadowed by one segment test against the statics and the door leaves (collision.ts — the CPU's stand-in for occluded()). What it has not got, named: no
 *  bounce (the probe adds the radiance cache's; a room lit only round a corner reads darker here than on screen), no moon unless sense.ts SENSE.moon says
 *  (kind 2: unshadowed it would light everybody — hours.ts moonLight gives it the sun's one shadow test), no bodies' shadows (capsules are GPU-side), no smoke, no fixture flicker. Used for one thing: how lit
 *  an AI TARGET is to an AI eye (unitLightMeter) when the renderer has not answered — headless always, the browser's first frame or two. Sam's meter reads it
 *  only with the senses on the CPU (sense.ts SENSE.cpu, game.ts lightMeter). ~one segment test per light in range: a few dozen per target per frame.
 *  `field` = the light model (sense.ts SENSE.bounce), where one stands: each lamp is then dimmed by the grid's smoke on its way, and the bounce of the
 *  square metre the point stands in is added. null = the estimate as above, whatever the switch says (the error tool asks both). `moon` = the level's
 *  moon is in it too (the switch's word unless the asker says). `leave` = lamps of the level's own the asker leaves out, by what feeds them
 *  (sim/lightshome.ts ROW_BIT: a sleeper's face leaves the screens'). */
export function lightEstimateAt(g: Game, P: Vec3, skipOwner?: (owner: number) => boolean, steadyOnly = false, field: LightField | null = lightField(g), moon: boolean = SENSE.moon, leave = 0): number {
  // (the lamps are one question in several calls, and the sun's, the moon's and the stroke's lines follow it: asked outside a tick — an order given
  //  between ticks prices its walk by the light of the floor — the whole of it stands in one window over the module's copy, scene/collision.ts window)
  return g.col.mirror.live ? lightEstimateIn(g, P, skipOwner, steadyOnly, field, moon, leave) : g.col.window(() => lightEstimateIn(g, P, skipOwner, steadyOnly, field, moon, leave));
}
function lightEstimateIn(g: Game, P: Vec3, skipOwner: ((owner: number) => boolean) | undefined, steadyOnly: boolean, field: LightField | null, moon: boolean, leave: number): number {
  const acc = [0, 0, 0, 0, 0, 0];
  lampsOn(g, P, skipOwner, steadyOnly, acc, field, leave);   // (the lamps: one question a point to the Rust module, rust/sim/src/light.rs)
  hourLight(g, P, acc);   // the level started at another hour (hours.ts): its sun and its sky on the same six faces — nothing at night
  if (moon) moonLight(g, P, acc);   // …and at night the moon where it has a line to P (sense.ts SENSE.moon; the error tool asks both ways)
  if (!steadyOnly) strokeLight(g, P, acc);   // a stroke of lightning under a picked weather (weather.ts): the renderer's meter reads it, so its stand-in does — nothing with no weather picked
  const direct = Math.max(acc[0], acc[1], acc[2], acc[3], acc[4], acc[5]);
  return field ? direct + field.grid.glowAt(g.col, P) : direct;   // (the light that comes round reaches every face alike: it is added to the brightest)
}
/** THE LIGHT MODEL'S FIELD for this game (sense.ts SENSE.bounce): its own smoke, where the senses are the game's, the switch is up and a grid stands.
 *  With it the estimate above has the smoke's shadow on every lamp and the light that comes round; null = the estimate as it always was. */
export function lightField(g: Game): LightField | null { const C = g.cpuSmoke; return SENSE.cpu && SENSE.grid && SENSE.bounce && C.grid && C.grid.stands ? C as LightField : null; }
type Lamp = Game['engine']['lights']['lights'][number];
/** WHICH OF THE LAMPS NO FIXTURE FEEDS IS ONE OF A LIGHT QUESTION'S (a man's torch, a flash, a tracer's lamp): on, not the moon,
 *  not one of an owner the asker leaves out, and, to who asks what the PLACE lights (`steadyOnly`, LIGHTS.litMan), nobody's passing flash: a round's
 *  impact, a tracer's lamp, a stun charge. The level's own lamps have these gates where they stand, in the module (rust/sim/src/light.rs `row`: a
 *  blackout beacon sweeping past is passing by its row's own bit). */
function lampAsked(l: Lamp, skipOwner: ((owner: number) => boolean) | undefined, steadyOnly: boolean): boolean {
  if (!l.enabled || l.intensity <= 0 || l.kind === 2 || (skipOwner && l.owner && skipOwner(l.owner))) return false;
  return !(steadyOnly && (l.ttl >= 0 || l.transient));
}
/** what a light question says of the level's own lamps (collision.ts LampRows): this game's rows, less the passing ones and each as its steady self to
 *  who asks what the PLACE lights (a fixture somebody flickers is its authored self there), less whatever else the asker leaves out */
const rowsOf = (g: Game, steadyOnly: boolean, leave: number): LampRows => ({ st: g.lightHome.block, leave: steadyOnly ? leave | ROW_BIT.passing : leave, steady: steadyOnly });
/** A LIGHT QUESTION'S OWN LAMPS written to the module (collision.ts lamp), each as the question takes it and with how many of the level's rows stand
 *  before it in the list: the module reads those where they stand and takes the two as one list in the list's order (light.rs `each`). The rows are
 *  the rows' in that order (sim/lightshome.ts stand), and a list that is not is a mistake thrown by name.
 *  `P`: a lamp whose reach cannot hold that point is not sent (a lamp further off along one axis than its range is further off than its range: the
 *  sum's own first test would pass it by). False = no question stands and the table is full: the sweep's (steadyLamps). */
function writeLamps(g: Game, col: StaticCollision, P: Vec3 | null, skipOwner: ((owner: number) => boolean) | undefined, steadyOnly: boolean): boolean {
  const lights = g.engine.lights.lights; let rows = 0;
  for (let i = 0; i < lights.length; i++) {
    const l = lights[i];
    if (l.row !== undefined) { if (l.row !== rows) throw new Error(`light: the lamp '${l.name}' is row ${l.row} of the level's lights and stands after ${rows} of them in the engine's list: the rows are that list's order (sim/lightshome.ts stand)`); rows++; continue; }
    if (!lampAsked(l, skipOwner, steadyOnly)) continue;
    const sp = steadyOnly ? l.steady : undefined, lp = sp ? sp.pos : l.pos, R = l.range;
    if (P) { const dx = lp[0] - P[0], dy = lp[1] - P[1], dz = lp[2] - P[2]; if (dx > R || -dx > R || dy > R || -dy > R || dz > R || -dz > R) continue; }
    if (!col.lamp(lp, R, sp ? sp.radius : l.radius, l.kind, l.dir, l.outerDeg, l.innerDeg, l.color, sp ? sp.intensity : l.intensity, rows)) return false;
  }
  return true;
}
const CARRIED = (): boolean => true;
/** THE LAMPS THE LIGHT THAT COMES ROUND IS SWEPT FROM (glow.rs): what the PLACE lights, less every lamp somebody carries (it moves faster than a sweep,
 *  and lights a man directly in the estimate). The level's own are its rows, read by the module where they stand: this writes the few of nobody's
 *  fixture and answers with how the rows are read. False = more of those than the module's table holds: the field stands as it was. */
export function steadyLamps(g: Game, col: StaticCollision): LampRows | false { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('steadyLamps', undefined, [g, col]); if (__r !== RUST_PASS) return __r as any; } } return writeLamps(g, col, null, CARRIED, true) && rowsOf(g, true, 0); }
/** THE LAMPS' PART OF lightEstimateAt: the shadowed direct light of every lamp, onto the six faces in `acc`. The Rust module's (rust/sim/src/light.rs):
 *  the level's own lamps are read where they stand and the question's own are written, and the module sums the two as one list in the engine's order, in
 *  as many parts as the written ones need (collision.ts lamp). A lamp there: the shader's own terms (1/max(d², r²), the range window, the spot
 *  cone's smoothstep, the panel's emitter cosine, the probe's floor for a light worth a shadow ray), then one line to it — a wall, a partition, a desk,
 *  a shut leaf between him and it: dark from that one — and the receiving cosine on each face. The smoke's shadow (`field`, SENSE.bounce) is the grid's
 *  cells along that very line and, where the grid cannot answer, the field's own blobs (smoke.rs). A grid nothing was let out into dims nothing: not asked. */
function lampsOn(g: Game, P: Vec3, skipOwner: ((owner: number) => boolean) | undefined, steadyOnly: boolean, acc: number[], field: LightField | null, leave: number): void {
  const col = g.col; col.lampsBegin(P, field !== null && field.grid.fed ? field : null, SENSE.smoke, rowsOf(g, steadyOnly, leave)); writeLamps(g, col, P, skipOwner, steadyOnly); col.lampsOn(acc);
}

// ------------------------------------------------------------ the right hand: what his state puts in it, what is actually there, and the reload beat
/** Rifles (docs/internal/sc20k-design.md §3, lane R1): which RIG is in his hands — the carbine when the sandbox knob has dealt him one over his sidearm row
 *  (Game.tune.carbineGuards → Guard.carbine, game.ts each frame: rig + poses only, he still fires his 9 mm row) or when the row he carries is itself a long gun
 *  (weapons.ts WeaponDef.rig 'carbine': R2 hands Reyes the aks74u). A rifleman has it in both hands in every living state and no hand torch — its weapon light
 *  is his light — so handCarry below never says 'torch' for him; handProps draws it, attachFlashlight puts the light at its lens, game.ts arms the animator. */
export function rigOf(gd: Guard): 'pistol' | 'carbine' | 'sc20k' { return gd.carbine || gd.weapon.def.rig === 'carbine' ? 'carbine' : gd.weapon.def.rig === 'sc20k' ? 'sc20k' : 'pistol'; }   // ('sc20k': a man handed Sam's bullpup row — the showcase's operatives — carries it in both hands like the carbine, drawn with RIG.sc20k, its light on the fore-end)
/** What this man's state has him CARRY in the gun hand: the torch on a calm patrol (state 'patrol' with the sidearm holstered — not `drawn`), his weapon in every
 *  other state (an alarmed floor patrols drawn; suspicious / alert / search have it up; a dead man reads 'weapon' too, as the old expression did — his hand is
 *  inHand's business) — and always his weapon if that is the carbine (rigOf: no torch for a rifleman). The one exception: a man walking over to a WHISTLE he is
 *  answering (whistle.ts whistleSoft: suspicious, on a calm floor, and nothing but the whistle has reached him since) keeps the torch — a whistle is not worth
 *  the pistol until a footfall, a glimpse or a second whistle makes it so. The one rule handProps, attachFlashlight, killGuard, the grab and the hold used to
 *  spell out each for itself. */
export function handCarry(gd: Guard): 'torch' | 'weapon' { return startleTorch(gd) || ((gd.state === 'patrol' || (gd.state === 'suspicious' && whistleSoft(gd))) && !gd.drawn && rigOf(gd) === 'pistol' && !kitPistol(gd)) ? 'torch' : 'weapon'; }   // (kit.ts: a teammate whose card hands him a pistol carries no hand torch)
/** What is physically in the hand: what he carries — unless it has left it: he is down (it lies on the floor as gd.dropped), or the torch was knocked out of it at
 *  the grab (dropped while he still carries 'torch': his hands are empty until the floor has him draw). */
/** (HOLD.dropGun) A living man on his own feet has the gun that was knocked out of his hand at the grab again: it is off the floor and in his fist. Called where
 *  a hold hands him back (player.ts handBack) and, for any other road out of an arm with his hands empty (a checkpoint saved in a hold loads him free: the hold
 *  itself is not filed), on his first update of his own. */
export function gunBack(g: Game, gd: Guard) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('gunBack', undefined, [g, gd]); if (__r !== RUST_PASS) return __r as any; } }
  const i = gd.dropped && gd.dropped.kind !== 'torch' ? g.items.items.indexOf(gd.dropped) : -1; if (i >= 0) { g.items.items.splice(i, 1); gd.dropped = null; }
  gd.gunLostAt = -1;
}
export function inHand(gd: Guard): 'torch' | 'weapon' | null {
  if (gd.fled) return null;   // (runner.ts, false with its switch off: he dropped it and ran — his hands are empty for good, and nothing more falls from them)
  if (gd.gunLostAt >= 0) return null;   // (HOLD.dropGun, player.ts startGrab: his gun was knocked out of his hand as the arm went round — empty hands until he is loose and has it again)
  if (gd.state === 'dead' || gd.civ) return null;   // (a civilian's hands are empty: nothing to knock out of them at a grab, nothing to fall from them — civilians.ts)
  const k = handCarry(gd);
  return k === 'torch' && (gd.dropped || (HOLD.dropGun && gd.torchLostAt >= 0)) ? null : k;   // (…|| torchLostAt, HOLD.dropGun: the floor gun of a later grab took `dropped`'s place and went back to his hand — the torch still lies where the first grab left it)
}
/** The magazine is empty: he stands there changing it for the row's reload.dur (the beat the fire sites count down — no round leaves meanwhile), says so, and the
 *  slide-back clack plays at him. */
export function startReloadBeat(g: Game, gd: Guard): void { return crossToRust('startReloadBeat', undefined, [g, gd]) as any; }
/** …counted down each frame he could otherwise fire; the frame it runs out the gun is simply full again (his spares are bottomless) and the NEXT frame may fire.
 *  A row that stages its reload (reload.out — the long guns; the sidearm's beat is one undivided 1.7 s and drops nothing) lets the empty go the frame the beat
 *  crosses `out`: a rifleman's thirty-round box clatters down out of the receiver (a 'mag' item with fill 2: throwables draws the long bakelite box) — physical,
 *  heard at arm's length like any magazine hitting the floor, and no more ammunition to anybody than the rest of what lies about. */
export function tickReloadBeat(g: Game, gd: Guard, dt: number): void { return crossToRust('tickReloadBeat', undefined, [g, gd, dt]) as any; }

/** The trigger of a row with a BurstSpec (the guards' carbine), asked once a frame at the alert branch's fire site in place of the sidearm's single round. No burst
 *  in hand: once `startOk` (every gate the single round has always had — the beat, the facing, a clear line, not twisted, not poked, the gun up, no shield) and no
 *  colleague on the bore's own line, one BEGINS: n = min‥max rounds drawn off the game's dice, the first out this same frame; and if it is his OPENING burst at a
 *  Sam SUPPRESS_FIRST_MIN_M or more off (tune.suppressFirst), the whole burst is laid beside Sam instead of on him — toward the nearer static within reach either
 *  side of him (else a side off the dice), 0.6–1.0 m out and a hand higher, dead steady (no hands in it: a tight group where he meant it), each round a few
 *  centimetres up the wall from the last. Burst in hand: the per-ROUND gates (`roundOk`: shield, gun lowered for a colleague, poked, twisted — and the last look
 *  down the bore) end it on the spot with nothing more out of it and BURST_BROKEN_CD in hand; otherwise a round every `interval` (600 rpm = every 6th frame at 60 Hz;
 *  never two in a frame however long the frame), the k-th of an aimed burst opening the cone by spreadPerRound·k (capped) over his hands' own spread and tight
 *  again next burst; the last round (or the box running dry) ends it, draws the gap to the next (gapMin‥gapMax) and, dry, starts the reload beat. The facing and
 *  the clear line are asked at the START only: a burst commits to the line it began on and follows the aim point as it tracks Sam — a Sam who steps behind the
 *  mullion mid-burst has the rest of it hit the mullion, which is what bursts do. A burst the trigger did not service LAST frame (he lost the licence, went to the
 *  standoff, reacted, was grabbed) is dropped before anything else: no round of it arrives late. `except` as roundPastColleague has it. */
export function burstTrigger(g: Game, gd: Guard, dt: number, aimAt: Vec3, startOk: boolean, roundOk: boolean, except: Guard | null = null): void { return crossToRust('burstTrigger', undefined, [g, gd, dt, aimAt, startOk, roundOk, except]) as any; }
/** Panel / QA: one line per living rifleman — rounds ready, the burst in hand (k of n, opening or aimed), bursts let go, the beat. */
export function burstSummary(g: Game): string {
  const parts: string[] = [];
  for (const gd of g.guards) {
    if (gd.state === 'dead' || !gd.weapon.def.burst) continue;
    const w = gd.weapon, b = w.burst;
    parts.push(`${gd.callsign}: ${w.roundsReady}/${w.def.magCapacity}${gd.reloadT >= 0 ? ` reloading ${gd.reloadT.toFixed(1)} s` : ''} · ${b ? `burst ${b.idx}/${b.n}${b.off ? ' (opening, wide)' : ''}` : `cd ${Math.max(0, gd.fireCd).toFixed(2)}`} · ${w.bursts} bursts`);
  }
  return parts.length ? 'rifles ▸ ' + parts.join(' | ') : `rifles ▸ nobody carries one (${g.tune.carbineGuards})`;
}

export function attachFlashlight(g: Game, gd: Guard, dt = 1 / 60) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('attachFlashlight', undefined, [g, gd, dt]); if (__r !== RUST_PASS) return __r as any; } }   // (dt: the relaxed carry's easing is per second — rigProps.ts torchCarryDir; callers outside the frame loop, savestate / squad, take the sixtieth)
  const held = poseFlashlight(g, gd, dt), fl = gd.flashlight;
  if (held && fl.enabled) fl.pos = lensClear(g, held, fl.pos, g.engine.settings.flashlightRadius, fl);   // a burning lens carried through the wall he is pressed to lights from this side of it (lighting.ts LIGHTWALL; a dark light asks nothing)
}
/** the light at its lens, its beam and whether it burns; returns what the lens hangs off — his gun shoulder, or the dropped torch's own body (nothing before he has a pose) */
function poseFlashlight(g: Game, gd: Guard, dt: number): Vec3 | undefined {
  const c = gd.char; const fl = gd.flashlight; const T = g.tune;
  if (!c.bones.handR) return;   // no pose yet
  let dir = c.gunDir;
  const rk = rigOf(gd); const carbine = rk === 'carbine';
  const torch = handCarry(gd) === 'torch';   // the hand torch on a calm patrol; the weapon light whenever the pistol is out (drawn on an alarmed floor, or any other state) — or on the carbine, always (handCarry never says torch for a rifleman)
  const hangs = torch && c.alive && torchLow(g, gd);   // (TORCH.wallLow / wallDark: the wall is at his hand — false for everybody with both off)
  if (torch && c.alive) { gd.beamDir = torchCarryDir(c, gd.beamDir, dt, hangs && TORCH.wallLow ? { tan: TORCH.lowTan, rate: TORCH.lowRate } : undefined); dir = gd.beamDir; }   // relaxed carry: beam follows the chest/aim heading, smoothed so the arm swing only bobs the origin
  else { gd.beamDir = v3.copy(dir); }
  fl.pos = rk === 'sc20k' ? sc20kLightPose(c).pos : kitPistol(gd) && gd.weapon.def.rig === 'pistol' ? rigLightPose(c).pos : rigTorchPos(c, dir, torch ? swapHeld(gd, 'torch') === 'torch' : swapHeld(gd, 'weapon') === 'torch', RIG, carbine ? 'carbine' : 'pistol'); fl.dir = dir;   // at the torch lens on patrol, at the weapon-light slot when drawn, at the lens on the carbine's handguard (or the bullpup's fore-end): the light is where the muzzle points
  if (gd.fled) { fl.enabled = false; return c.bones.upperArmR; }   // (runner.ts, false with its switch off: his light went with the gun he dropped — dark in Sam's arm, off a grenade, and on the floor after him)
  let held: Vec3 | undefined = c.bones.upperArmR;   // (what the lens hangs off, for the wall rule below: his gun shoulder — or the dropped torch's own body)
  const lostLive = torch && gd.torchLostAt >= 0 ? gd.torchLostAt : gd.gunLostAt;   // when what carried his light left a LIVING hand (the torch at a grab on patrol · the gun at any grab, HOLD.dropGun), −1 if it has not
  if ((gd.state === 'dead' || lostLive >= 0) && gd.dropped) { const it = gd.dropped; const hd: Vec3 = [Math.sin(it.yaw) * Math.cos(it.tumble), -Math.sin(it.tumble), Math.cos(it.yaw) * Math.cos(it.tumble)]; fl.pos = v3.mad(it.pos, hd, 0.1); fl.dir = hd; held = it.pos; }   // the beam spins with the tumbling torch / pistol until it dies
  if (gd.state !== 'dead' && lostLive >= 0) {   // knocked out of a living hand (the grab): the same sputter-and-die on the floor; drawn later (alert), the weapon light is his light again
    if (!gd.lightLitAtLoss) { fl.enabled = false; return held; }   // (it was dark when it fell: it stays dark)
    const t = g.time - lostLive; fl.enabled = t < 0.9 && (t < 0.35 || Math.sin(t * 60) > 0.3); fl.intensity = fl.peakIntensity * clamp(1 - t / 0.9, 0, 1); return held;
  }
  if (gd.state === 'dead' && g.time >= gd.lightDeadUntil) {   // a dropped torch dies with its owner: a couple of sputters as it hits the floor, then dark (a light lying IN smoke at floor level is also the worst case for the volumetric estimate)
    // …unless it already died: a man grabbed on patrol lost his torch to the floor at torchLostAt and it went dark there; when he goes down later (choked, whipped,
    // shot in the arm) nothing leaves his hand and gd.dropped is still THAT torch — keying the sputter to diedAt flashed it back on at full power wherever it lay
    // (review finding, reproduced: 70/70 for 0.9 s, 5 m from the body). The sputter runs from whichever loss came first.
    if (!gd.lightLitAtLoss) { fl.enabled = false; return held; }   // (a light that was never burning — a breach hostile's, a unit spawned light:false — does not switch ON to sputter as the gun tumbles; review 08-10 #7)
    const lostAt = Math.min(gd.torchLostAt >= 0 ? gd.torchLostAt : Infinity, gd.gunLostAt >= 0 ? gd.gunLostAt : Infinity, gd.diedAt);
    const t = g.time - lostAt; fl.enabled = t < 0.9 && (t < 0.35 || Math.sin(t * 60) > 0.3); fl.intensity = fl.peakIntensity * clamp(1 - t / 0.9, 0, 1); return held;
  }
  if (g.time < gd.lightDeadUntil) { fl.enabled = false; }
  else if (gd.lightDeadUntil > 0 && g.time < gd.lightDeadUntil + 1.0) { fl.enabled = Math.sin(g.time * 50) > 0.2; }   // sputters back
  else fl.enabled = (!gd.unit || ((gd.unit.light || darkFoeLit(g, gd) || powerOutLit(g, gd)) && (!lightRuled(g, gd) || teamLit(g, gd)) && !nvgDark(g, gd) && !silentDark(g, gd))) && !(torch && calmTorchDark(g, gd, dt));   // (a unit spawned with its weapon light off — a daylight room — keeps it off: teams, spawnUnit; a teammate under Sam's orders burns his only while it helps: LIGHTS above — and not at all while the team is dark on its goggles: nvg.ts · a calm staff man's hand torch burns only where his place is dark: calmtorch.ts, nothing with its switch off · an enemy unit dealt a dark light burns it once he knows and is blind: darkfoe.ts, false with its switch off)
  if (hangs && TORCH.wallDark && !TORCH.wallLow) fl.enabled = false;   // (the other answer to the same reading, only where asked for: put out instead of tipped)
  if (wallDark(g, gd, held) && fl.enabled) { fl.enabled = false; const s = wallSeen.get(gd); if (s) s.out = true; }   // (LIGHTS.wallOff: a teammate's beam that would end on the wall or the truck at his muzzle — asked every frame so its clock runs whether or not the light burns; false for everybody with the switch off)
  if (fl.enabled && dozeDark(gd)) fl.enabled = false;   // (doze.ts: a man sat down to doze has put his torch out — false with its switch off)
  fl.peakIntensity = T.flashIntensity; fl.intensity = T.flashIntensity; fl.innerDeg = T.flashInner; fl.outerDeg = T.flashOuter; fl.radius = g.engine.settings.flashlightRadius;
  return held;
}

export function onScreen(g: Game, p: Vec3): boolean {   // (on a player's screen: with two, on either's — seats.ts)
  if (g.seats.length > 1) { for (const s of g.seats) if (inView(s.view.viewProj, p)) return true; return false; }
  return inView(g.viewProj, p);
}
function inView(vp: Float32Array | number[] | null, p: Vec3): boolean {
  if (!vp) return false;
  const w = vp[3] * p[0] + vp[7] * p[1] + vp[11] * p[2] + vp[15]; if (w <= 0) return false;
  const x = (vp[0] * p[0] + vp[4] * p[1] + vp[8] * p[2] + vp[12]) / w, y = (vp[1] * p[0] + vp[5] * p[1] + vp[9] * p[2] + vp[13]) / w;
  return x > -0.92 && x < 0.92 && y > -0.9 && y < 0.9;
}

/** how long a line stands over a man's head, by its length */
export const bubbleSecs = (text: string) => Math.min(4, Math.max(2, 1.5 + text.length * 0.055));
/** A guard says something: a bubble over his head if he is on screen, otherwise (or if it went out over the radio) a line in the log. A man held in Sam's arm has
 *  no voice of his own — whatever the floor's logic would have him bark or radio is dropped (the callers that matter pick somebody else: spokesman, radioCaller,
 *  pickChecker skip him) — except his answers to Sam, which come `force`d (Game.interrogate) and go to the log under his name. */
/** his head, or where it would be (what a line is on or off the screen by: say() below and ui/bubbles.ts ask the same point) */
export const headOf = (gd: Guard): Vec3 => gd.char.bones.head ?? v3.add(gd.char.pos, [0, 1.7, 0]);

export function say(g: Game, gd: Guard, text: string, radio = false, force = false, row = true) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('say', undefined, [g, gd, text, radio, force, row]); if (__r !== RUST_PASS) return __r as any; } }   // (`row` false = the bubble alone: no log row even out of sight)
  if (gd.held && !force) return;
  const net = radio || silentHush(g, gd);   // (a SILENT teammate calls nothing out: his line goes over the net — silent.ts)
  gd.bubble = { text, t: g.time, dur: bubbleSecs(text), radio: net };
  if (net) radioHand(gd, gd.bubble.dur);   // (handwork.ts: a line on the net lifts his hand to the handset — picture only)
  if (!net && row) g.cues?.overheard(gd, text);   // a shout from a man the fog hides, where an ear of the team's really gets it: a row of its own on the strip, his words alone (cues.ts, behind CUES.overheard — off: nothing asked)
  if (radio && gd.unit) g.cues.heard(gd, text);   // a room fight's defenders' net reaches the strip (cues.ts: ranked, rate-limited; nothing here reads it back)
  if (radio) chronRadio(g, gd, text);   // the debrief's row: what went over their net (the routine radio check is dropped there)
  if (row && (radio || g.fogOfWar.hidden(gd) || !onScreen(g, headOf(gd)))) g.msg((net ? (gd.unit ? `${gd.callsign} (radio): ` : 'radio: ') : force || gd.unit ? `${gd.callsign}: ` : 'guard: ') + text);   // (a unit's line goes to the log under his own name: he is not 'guard' — teams)
  if (SQUADNET.on) { const O = ordersOf(g, gd); if (O?.sams && O.mine(gd) && !onScreen(g, headOf(gd))) g.squadNet?.line(); }   // (squadnet.ts: a teammate's line said off your screen clicks in your ear, and the page draws it near you; nothing asked with its switch off. `sams`: the response team's men are 'mine' to their own cast of orders, and are not the squad)
}

/** Fresh guards at the start of their routes (keeps their flashlight lights and tints). Spawned units (teams: the showcase's operatives and hostiles) are not the
 *  level's to stand up again: they are despawned — their weapon lights taken back, their meter keys forgotten, an engagement running on them ended 'reset'. */
export function resetGuards(g: Game) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('resetGuards', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
  for (const pl of men(g)) if (pl) { pl.dragging = null; pl.takedown = null; pl.holding = null; pl.char.anim.holdPose = null; }   // the old bodies / targets are gone — the man in his arm with them
  for (const gd of g.guards) if (gd.unit) releaseUnit(g, gd);
  endEngagement(g, 'reset');
  const old = g.guards, fresh = old.filter(gd => !gd.unit).map(gd => ({ id: gd.char.id, route: gd.route, routeI: gd.routeI, fl: gd.flashlight, tint: gd.char.tint }));
  for (const gd of old) manLeaves(gd.char, gd);   // (the old shift's rows are free before the new shift takes them)
  g.guards = fresh.map(o => new Guard(g.men, g.ch, o.id, o.route, o.routeI, o.fl, o.tint));
  for (const gd of g.guards) gd.char.update(0);          // fresh characters get a pose (bones) immediately — reset can happen mid-frame
  openingStart(g);   // the night's opening (opening.ts): nobody of the fresh shift starts on top of the way in — nothing under a show's flag or with OPENING.on off
  g.lightHome.in(LIGHT_IN.fix, null, 0); eachSeat(g, v => { v.aimGuard = null; });
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
const HEIGHTENED_SECS = 75, LOCKDOWN_SECS = 120;
/** how much harder an alarmed floor looks and listens: × the visual detection rate and the small-noise awareness gains (so 'huh?' comes that much sooner) */
export const KEEN = 1.25;

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
    if (offNet(gd) || gd === except || (pass === 0 && (gd.state === 'alert' || dozeAsleep(gd))) || gd.team !== 'guard' || gd.unit) continue;   // (a man with a forearm across his throat radios nothing; a unit is not on the staff's net — teams — and a staff-side unit, the response team of qrf.ts, gives the shift no orders: it has its own voice)
    const d = pos ? v3.dist(gd.char.pos, pos) : 0; if (d < bd) { bd = d; best = gd; }
  }
  return best;
}

/** Raise the floor to at least `to` and (re)arm that level's clock. One radio line per actual change (a lockdown re-armed by yet another alarm just says so), from the
 *  calm man nearest the last fix — `except` the one whose own line should stay up; `cause` picks the wording for a first raise (a search that found nobody, a
 *  body, a man who neither answers the net nor is anywhere on his route, or a colleague seen held in Sam's arm — the standoff's call). A lockdown deals the duties
 *  out at once so the post man's own line follows the order; for a hostage no rooms are dealt for clearing (they know exactly where he is — the pair is what
 *  everybody converges on; the alarm coming to nothing later plans the sweep as ever). */
export function raiseEscalation(g: Game, to: 1 | 2, except: Guard | null = null, cause: 'search' | 'body' | 'missing' | 'hostage' | 'device' | 'civilian' = 'search', another = false) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('raiseEscalation', undefined, [g, to, except, cause, another]); if (__r !== RUST_PASS) return __r as any; } }   // (another: bodies had been called in before this one — the order says so; 'device': security's cameras / tripwires had him long enough to call it — devices.ts, control's own line goes out first; 'civilian': a cleaner / a late worker told a man what she saw or rang it in, or was found down — civilians.ts, the telling line goes out first)
  const prev = g.escalation; const lvl: 0 | 1 | 2 = to > prev ? to : prev;
  g.escalation = lvl; g.escalationT = g.time + (lvl >= 2 ? LOCKDOWN_SECS : HEIGHTENED_SECS);
  if (lvl > prev) chronAlarm(g, lvl, true, cause);   // the debrief's row + pin: the floor went up (a re-arm at the same level is not news)
  const again = g.time - g.alarm.raisedAt < 1.5; g.alarm.raisedAt = g.time;
  if (again && lvl <= prev && cause !== 'body') return;   // just raised (a searcher finding a body ends his hunt, and with it the episode, in the very frame the body raised the floor): the clock is re-armed, nothing else is new — no 'still nothing' on top of 'man down', no re-deal (a body always re-plans: the rooms round HIM)
  const sp = spokesman(g, g.alarm.placed ? g.alarm.pos : null, except);
  const line = lvl <= prev ? (cause === 'body' ? `${another ? 'another man down' : 'man down confirmed'} — stay locked down, clear everything ${wg('around', 'round')} him` : cause === 'missing' ? "and a man we can't raise — it stays locked down, find him" : cause === 'hostage' ? wg("copy — he's got one of ours. stay locked down. box him in. no shot unless it's clean", 'copy — he has one of ours. it stays locked down: get round him, nobody fires till he has him clean') : cause === 'device' ? 'copy, control — it stays locked down, get to it' : cause === 'civilian' ? w('copy — another one saw him. stay on it, get over there', 'copy — another one saw him. stay on it, get to where she says') : 'still nothing — it stays locked down')
    : lvl >= 2 ? (cause === 'hostage' ? wg("he's got one of ours — lock it down. weapons out, everyone box him in and hold your fire", 'he has one of ours — lock it down: weapons out, everyone to him, box him in and hold your fire') : `${cause === 'body' ? (another ? 'another man down — ' : 'man down — ') : cause === 'missing' ? "a man missing on top of it — " : cause === 'device' ? 'copy, control — ' : cause === 'civilian' ? "copy — that's a witness. " : ''}lock it down: pairs, weapons out, nobody wanders`)
    : cause === 'body' ? 'we have a man down — weapons out, everyone sharp' : cause === 'missing' ? wg("we've got a man off the net and off his route — weapons out, treat it as real", "a man off the net and not on his route — weapons out, eyes open for him, treat it as real") : cause === 'device' ? 'copy, control — weapons out, everyone sharp, somebody get eyes on it' : cause === 'civilian' ? (WORDS.on && !isOffice(g.level) ? "copy — somebody's here who shouldn't be. weapons out, treat it as real" : "copy — somebody's on the floor who shouldn't be. weapons out, treat it as real") : 'nothing here — stay sharp, check your corners';
  if (sp) { g.say(sp, line, true); g.audio.play('radio', sp.char.bones.head ?? sp.char.pos, 0.7); }
  if (lvl >= 2) { assignLockdown(g, prev >= 2, except); if (cause !== 'hostage') planClears(g, false, cause === 'device' || cause === 'civilian' ? 'search' : cause); }   // (re-dealt on a re-arm too: they have just converged on a new fix, so pair whoever is together now and post the spare man by IT) — and the pair clears the rooms nearest that fix (more of them, and whatever they were on dropped, for a body)
}

/** Panel / debug: the floor calm again at once — level 0, clock off, pair and post dissolved, a clear in progress called off — without touching the men themselves (whoever is suspicious stays so). */
export function standDownEscalation(g: Game) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('standDownEscalation', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } } g.escalation = 0; g.escalationT = -1; g.alarm.episode = false; g.alarm.placed = false; g.clearing?.cancel(); g.clearing = null; g.clearedAt.clear(); clearLockdown(g); }


export interface Rollcall { nextAt: number; stage: 'idle' | 'called' | 'asked'; t0: number; caller: Guard | null; missing: Guard | null; answered: boolean; checks: number; }
const rollcalls = new SideMap<Game, Rollcall>('guards.rollcalls');
/** A man who cannot answer the net: down (dead or out cold), or held with Sam's arm across his throat (Guard.held). Every radio-check predicate asks this, not
 *  'dead' — so a held man fails the sound-off exactly like a corpse: asked for by name while you hold him, somebody sent round his route toward you — and one
 *  let go answers again from the next frame (checkOnTick / the 'asked' stage drop him as they would a man walked onto alive). */
export function offNet(x: Guard): boolean { return x.state === 'dead' || !!x.held; }
/** The net's radio-check state for this game (made on first touch; the panel / probes read it: when the next check is due, what stage a running one is at). */
export function rollcallState(g: Game): Rollcall {
  let R = rollcalls.get(g); if (!R) { R = { nextAt: -1, stage: 'idle', t0: 0, caller: null, missing: null, answered: false, checks: 0 }; rollcalls.set(g, R); }
  return R;
}
/** A silent man is being looked for right now — somebody walks his route, or the net has asked for him and is about to send somebody: the floor's alarm level
 *  holds meanwhile (updateEscalation does not step down under it; the QA soak's clock invariant knows the same rule). */
export function stillLooking(g: Game): boolean { return rollcallState(g).stage === 'asked' || g.guards.some(x => x.state !== 'dead' && x.task?.kind === 'checkOn'); }

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
export function planClears(g: Game, force = false, cause: 'search' | 'body' | 'panel' | 'missing' = 'search'): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('planClears', undefined, [g, force, cause]); if (__r !== RUST_PASS) return __r as any; } }
  if (g.quietUtility || !g.aiEnabled || escalationOf(g) < 2) return false;
  if (g.clearing) { if (!force && cause !== 'body') return false; g.clearing.cancel(); g.clearing = null; }
  let a: Guard | null = null, b: Guard | null = null;
  for (const gd of g.guards) if (gd.state !== 'dead' && !gd.held && gd.leader && gd.leader.state !== 'dead' && !gd.leader.held) { a = gd.leader; b = gd; }   // (a pair with a man in Sam's arm in it is no pair to send round the rooms — nor can he take the order)
  if (!a) { const alive = g.guards.filter(x => x.state !== 'dead' && !x.held && x.team === 'guard' && !x.unit); if (alive.length !== 1) return false; a = alive[0]; }   // (teams: the staff's rooms are cleared by the staff — not by a staff-side unit: the response team clears its own way, qrf.ts)
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




/** Deal the lockdown duties: the two living men nearest each other pair up — the one with more route to cover leads (lobby / break-room loop > corridor > cubicle
 *  farm), the other trails him — and a third holds the corridor junction nearest the last alarm fix (nearest himself if there never was one). `redeal`: after a
 *  death or a separation the (new) leader tells his man to close up — only if the pairing actually changed, so a pair that keeps getting split cannot chatter. */
function assignLockdown(g: Game, redeal: boolean, except: Guard | null = null) {   // (except: a man whose own line must stay up — the finder of a body — takes his role without announcing it)
  let oldLead: Guard | null = null, oldFollow: Guard | null = null, oldPost: Chokepoint | null = null, oldPostMan: Guard | null = null;
  for (const gd of g.guards) { if (gd.state === 'dead') continue; if (gd.leader) { oldFollow = gd; oldLead = gd.leader; } if (gd.post) { oldPost = gd.post; oldPostMan = gd; } }
  clearLockdown(g); g.alarm.apartT = 0;
  const alive = g.guards.filter(x => x.state !== 'dead' && !x.held && x.team === 'guard' && !x.unit); if (alive.length < 2) return;   // (a man held in Sam's arm is dealt nothing: he can neither trail, lead nor hold a junction, and everybody the call has reached knows why; a unit is not staff — teams — nor is the response team paired or posted by the shift: qrf.ts)
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




/** 0..1 tension for the score: patrol 0, suspicious ~0.5, search ~0.65, alert 1 — and an alarmed floor never quite settles (heightened 0.12, lockdown 0.22); a pair
 *  methodically clearing rooms (scripted, nominally 'search') sits at 0.3: purposeful, not alarmed. `theirs` (the page's score passes MUSIC.theirs, audio/pieces.ts; absent = every
 *  man counts, as it was): in live play a teammate counts only while he is in a fight of his own ('alert') — the wariness a blast, a noise or a tripped breaker left
 *  on him, and the 'search' his order takes him back in (orders.ts), never drain while he is under a script (updateGuard returns into it before the decay) and would
 *  hold the score up with every defender down. While a SHOW runs its men's scripted states are the score's cue (the day showcase's pair hunting) and read as ever. */
export function tension(g: Game, theirs = false): number {
  let x = Math.max(g.blackout.active ? 0.18 : 0, escalationOf(g) >= 2 ? 0.22 : escalationOf(g) >= 1 ? 0.12 : 0);
  const live = theirs && !(g.touring || g.director.showing);
  for (const gd of g.guards) {
    if (gd.state === 'dead' || (live && gd.team === 'operative' && gd.state !== 'alert')) continue;
    const s = gd.script && g.clearing && (gd === g.clearing.a || gd === g.clearing.b) ? 0.3 : gd.state === 'alert' ? 1 : gd.state === 'search' ? 0.68 : gd.state === 'suspicious' ? 0.45 + gd.awareness * 0.2 : conditionCalm(gd) ? 0 : gd.awareness * 0.35;   // (condition.ts: one of the player's own at rest is not a wary guard — under an order nothing drains his number)
    x = Math.max(x, s);
  }
  return x;
}

// ================================================================ SIDES: spawned units, and the engagement seam (teams.ts has the table of sides; the functions are here)
// THE CONTRACT (for whoever stages men and calls this and nothing deeper — spawnUnit / despawnUnit: director.ts, qrf.ts, nightbreach.ts, rescue.ts, quiet.ts, stress.ts;
// engage(): the Director's drill.exchange alone in the game, and the probes under tools/qa directly):
//   spawnUnit(g, { team, pos, yaw?, weapon?, tint?, tint2?, carbineRig?, name?, light? }) → Guard
//       Stands an AI man of side `team` ('operative' | 'hostile' — or 'guard': extra staff off no route) at `pos` facing `yaw`, carrying `weapon` (default: an
//       operative Sam's FN2000 row, a hostile the AKS-74U carbine, staff the 9 mm; `carbineRig: true` = the AKS-74U whatever `weapon` says — in this codebase the
//       carbine in the hands IS the carbine row), jacket `tint` / trousers `tint2`, answering to `name` (default '<team>-<n>'), his weapon light on unless `light:
//       false`. He is a Guard in g.guards like any other — perceived, perceiving, shot and shooting through the same code, drawn, ragdolled, pushed by doors —
//       on a one-point route of his own (Guard.route, routeI −1), HELD on it (Guard.hold: he stands and looks about) until a script or engage() takes him,
//       state 'patrol', deaf to everything that happened before he existed. Throws, clearly, when a budget is spent: 8 characters' shadow capsules all told
//       (Sam + 7), 64 steady lights (his weapon light is one), the owner byte's ids — or when Game.tune.teams is off (the kill switch). Costs Math.random draws
//       (his idle clocks): a world with a unit in it is not the seeded world without.
//   despawnUnit(g, gd) → boolean · despawnUnits(g)
//       Takes a spawned man back out — his light, his meter keys, every reference the others hold to him (a foe, a standoff pair, the camera's spectate index,
//       Sam's arm or grip) — true if he was one. Level staff are never despawned (park them). resetGuards / restartEncounter despawn every unit themselves.
//   engage(g, operatives, hostiles, { timeout = 45, pin = false, surprise = 0 }) → Engagement { done, result, promise, ... }
//       Puts both groups in the fight NOW: any script on a man is ended, hold / pinned come off (pinned stays on with `pin`: they fire from where they stand),
//       state 'alert' at full awareness with a short staggered reaction beat (the hostiles' `surprise` seconds longer: the room caught flat by the door going
//       in — at 0 an open aisle is a coin flip on who fires first, and whoever does mostly wins), each man's foe = the enemy he can best see, else the nearest
//       living one, his fix on that man (licensed to shoot at once if he has him in sight). From there the ordinary AI runs it: acquisition by salience, the
//       aim model, bursts, muzzle discipline within a side, the side-step, suppression, reload beats, one-shot kills, the search when a man is lost; when the
//       last enemy of a side is down the survivors come off the alert ('clear.') and drift back to their marks. `promise` resolves (and `done` / `result` are
//       set — poll them from a tick if you prefer) with an EngageResult when every hostile is down ('cleared'), every operative is ('wiped'), `timeout`
//       seconds pass ('timeout'), the guards are reset under it ('reset'), or another engage() supersedes it ('superseded'). One engagement at a time per
//       Game. Nothing here touches the staff's alarm, radio net or lockdown: those are gated to team 'guard' throughout guards.ts.
//   BREACH_HOOKS bundles the same functions for a caller that wants one import.
/** What spawnUnit takes (see the contract above). */
export interface UnitOpts { team: Team; pos: Vec3; yaw?: number; weapon?: WeaponId; tint?: Vec3; tint2?: Vec3; carbineRig?: boolean; name?: string; light?: boolean;
  /** the pair a teammate is dealt into (UnitInfo.pair) */
  pair?: PairId;
  /** a caller that has decided to stand more men than cast shadows (the four-man squad, nightbreach.ts): the character count this spawn may reach, up to the
   *  skinning's MAX_SKINNED — every man drawn has his shadow / reflection / bounce proxies (BoxWorld.MAX_CAST); the men past the eighth DRAWN have no mesh tree,
   *  so to a ray they are their round cones (game.ts draws the living first once there are more). Absent = the eight, as ever. */
  budget?: number; }
/** the character budget a caller gets without asking for more, Sam included (the mesh trees' count; the shadow proxies hold every drawn man: scene/boxes.ts MAX_CAST) */
const MAX_CHARACTERS = 8;
/** … and the skinning holds this many (anim/characters.ts MAX_CHARS): the most a caller's own `budget` can ask for */
export const MAX_SKINNED = 16;
/** ids handed to units: clear of Sam's 1 and the staff's 10‥15, inside the owner byte */
const UNIT_ID0 = 20, UNIT_ID1 = 250;
let unitSerial = 0;
rollbackVar('guards.unitSerial', () => unitSerial, v => { unitSerial = v; });
const TEAM_TINT: Record<Team, Vec3> = { guard: [0.42, 0.36, 0.25], operative: [0.07, 0.09, 0.1], hostile: [0.36, 0.33, 0.24], player: [0.05, 0.06, 0.07], civilian: [0.75, 0.62, 0.08] };
const TEAM_TINT2: Record<Team, Vec3> = { guard: [0.02, 0.02, 0.025], operative: [0.05, 0.07, 0.06], hostile: [0.09, 0.08, 0.05], player: [0.1, 0.32, 0.18], civilian: [0.05, 0.06, 0.1] };
export function spawnUnit(g: Game, o: UnitOpts): Guard { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('spawnUnit', undefined, [g, o]); if (__r !== RUST_PASS) return __r as any; } }
  if (!g.tune.teams) throw new Error('spawnUnit: sides are switched off (Game.tune.teams) — no unit may be spawned');
  if (o.team === 'player') throw new Error("spawnUnit: the 'player' side is the player alone — spawn an 'operative' for a man on his side");
  if (o.team === 'civilian') throw new Error("spawnUnit: civilians are not units — civilians.ts stands them up from the level's civilians block (Game.civilians)");
  const chars = seatCount(g) + g.guards.length;   // (a man a seat, and everybody else who stands)
  const eight = budgetWithSeats(g, MAX_CHARACTERS), cap = Math.min(MAX_SKINNED, Math.max(eight, o.budget ?? 0));   // (the eight is written for one player: a seat more, a man more)
  if (chars + 1 > cap) throw new Error(cap > eight ? `spawnUnit: ${chars} characters already stand (the player + ${g.guards.length} guards/units) and this caller's budget is ${cap} — despawn a unit or reset the guards first`
    : `spawnUnit: ${chars} characters already stand (the player + ${g.guards.length} guards/units) and the shadow-capsule budget is ${eight} (scene/boxes.ts BoxWorld.addCharacterCapsules) — despawn a unit or reset the guards first`);
  const steady = g.engine.lights.lights.filter(l => l.ttl < 0).length;
  if (steady + 1 > MAX_LIGHTS) throw new Error(`spawnUnit: ${steady} steady lights already exist and the light buffer holds ${MAX_LIGHTS} (render/lights.ts) — his weapon light would never upload; despawn a unit first`);
  const used = new Set<number>([...men(g).map(pl => pl.char.id), ...g.guards.map(x => x.char.id)]);
  let id = UNIT_ID0; while (used.has(id) && id <= UNIT_ID1) id++;
  if (id > UNIT_ID1) throw new Error(`spawnUnit: no character id left in ${UNIT_ID0}‥${UNIT_ID1} (the owner byte) — despawn units first`);
  const n = ++unitSerial; const name = o.name ?? `${o.team}-${n}`;
  const weapon: WeaponId = o.carbineRig ? 'aks74u' : o.weapon ?? (o.team === 'operative' ? 'fn2000' : o.team === 'hostile' ? 'aks74u' : 'guard9mm');
  const route: PatrolRoute = { name, points: [[o.pos[0], 0, o.pos[2]]], wait: [0], weapon, keys: [] };   // his one-point stand: nothing to walk, nothing on his belt
  const T = g.tune; const fl = g.engine.lights.add(torchLightDef(`unitlight${id}`, id, T.flashIntensity, T.flashInner, T.flashOuter));
  const gd = new Guard(g.men, g.ch, id, route, -1, fl, o.tint ?? TEAM_TINT[o.team], { tint2: o.tint2 ?? TEAM_TINT2[o.team], yaw: o.yaw ?? 0, name, team: o.team });
  gd.unit = { name, spawnedAt: g.time, light: o.light !== false }; if (o.pair) gd.unit.pair = o.pair; gd.hold = true;
  gd.heardUpTo = g.eventSeq;   // (deaf to what was in the air before he existed; the ids run on from here)
  gd.char.update(0);           // a pose — bones — before anybody's sight test or the frame's capsule upload reads him
  g.guards.push(gd);
  return gd;
}
/** everything of a unit that lives outside the Guard object, taken back (despawnUnit / resetGuards): his weapon light out of the engine's list, his meter keys forgotten */
function releaseUnit(g: Game, gd: Guard) {
  g.engine.lights.remove(gd.flashlight);
  g.lightForget(`unitC${gd.char.id}`); g.lightForget(`unitH${gd.char.id}`);
}
export function despawnUnit(g: Game, gd: Guard): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('despawnUnit', undefined, [g, gd]); if (__r !== RUST_PASS) return __r as any; } }
  if (!gd.unit) return false;
  const i = g.guards.indexOf(gd); if (i < 0) return false;
  if (g.player.holding?.g === gd) g.freeHeld('cancel');           // out of Sam's arm first (player.ts hands him back on his feet; then he goes)
  if (g.player.dragging === gd) g.toggleDrag();
  if (g.player.takedown?.g === gd) g.player.takedown = null;
  releaseUnit(g, gd);
  g.guards.splice(i, 1);
  if (g.spectate === i) g.spectate = -1; else if (g.spectate > i) g.spectate--;   // (the camera follows by index)
  if (g.aimGuard === gd) g.aimGuard = null;
  for (const o of g.guards) {   // nobody keeps a hand on a man who is no longer there
    if (o.foe === gd) { o.foe = null; o.aim.clear(); }
    if (o.standoff?.pair === gd) o.standoff = null;
    if (o.bodyDuty?.body === gd) o.bodyDuty = null;
    if (o.leader === gd) o.leader = null;
    if (o.task?.kind === 'checkOn' && o.task.who === gd) o.task = null;
  }
  manLeaves(gd.char, gd);
  return true;
}
export function despawnUnits(g: Game): void { return crossToRust('despawnUnits', undefined, [g]) as any; }
/** every spawned unit standing (or lying) in the game right now */
export function units(g: Game): Guard[] { return g.guards.filter(x => !!x.unit); }

/** How an engagement ended, and what happened in it: seconds it ran, who went down (victim, his side, whose round, when — seconds in), rounds fired per side (the
 *  shot events, by who fired them), and who is left standing either side. */
export interface EngageResult { outcome: 'cleared' | 'wiped' | 'timeout' | 'reset' | 'superseded'; secs: number; kills: { victim: string; team: Team; by: string; t: number }[]; rounds: Partial<Record<Team, number>>; operativesLeft: number; hostilesLeft: number; }
export interface Engagement { operatives: Guard[]; hostiles: Guard[]; t0: number; timeout: number; done: boolean; result: EngageResult | null; promise: Promise<EngageResult>;
  /** bookkeeping: the kills so far, rounds so far, who was alive last frame */
  kills: EngageResult['kills']; rounds: Partial<Record<Team, number>>; alive: Set<Guard>; resolve: (r: EngageResult) => void; }
const engagements = new SideMap<Game, Engagement>('guards.engagements');
/** Arm one man for the fight against `foes` (see engage): off any script and off his marks, alert, a staggered beat (+ `delay`: the caught side's surprise), his enemy chosen and fixed. */
function armForFight(g: Game, gd: Guard, foes: Guard[], pin: boolean, delay = 0) {
  if (gd.state === 'dead') return;
  if (gd.script) endGuardScript(gd, 'alert');
  gd.hold = false; gd.pinned = pin; gd.state = 'alert'; gd.awareness = 1; gd.searchT = 0; gd.bodyDuty = null; gd.searchPlan = null; gd.standoff = null;
  gd.reactT = 0.15 + simRandom() * 0.35 + delay; gd.fireCd = 0.4 + simRandom() * 0.4; gd.reloadT = -1; resetChase(gd);
  gd.witness.alertedBy = 'radio'; gd.witness.alertT = g.time;   // (an order put him there)
  // his man: the enemy he sees best from where he stands, else the nearest living one — fixed, and licensed at once if actually in sight
  let best: Guard | null = null, bestRate = -1, seen = false;
  for (const f of foes) {
    if (f.state === 'dead' || !f.char.alive) continue;
    const vis = canSeeChar(g, gd, f.char, false);
    const rate = vis.visible ? 1 + sightGainOn(g, gd, f, vis.dist) : 1 / (1 + v3.distXZ(gd.char.pos, f.char.pos));   // (anything seen outranks anything not; among the unseen, the nearer)
    if (rate > bestRate) { bestRate = rate; best = f; seen = vis.visible; }
  }
  gd.foe = best; gd.aim.clear();
  if (best) { gd.lastKnown = v3.copy(best.char.pos); if (seen) { gd.lastSeenT = g.time; gd.sightPos = v3.copy(best.char.pos); gd.sightT = g.time; gd.ledT = g.time; } }
}
export function engage(g: Game, operatives: Guard[], hostiles: Guard[], opts: { timeout?: number; pin?: boolean; surprise?: number } = {}): Engagement {
  endEngagement(g, 'superseded');
  const pin = !!opts.pin;
  for (const gd of operatives) armForFight(g, gd, hostiles, pin);
  for (const gd of hostiles) armForFight(g, gd, operatives, pin, Math.max(0, opts.surprise ?? 0));   // (surprise: seconds the room is caught flat by the men coming through the door — 0 = a fair fight, and an open aisle at nine metres is a coin flip on who fires first)
  let resolve!: (r: EngageResult) => void; const promise = new Promise<EngageResult>(res => { resolve = res; });
  const E: Engagement = { operatives: [...operatives], hostiles: [...hostiles], t0: g.time, timeout: opts.timeout ?? 45, done: false, result: null, promise, kills: [], rounds: {}, alive: new Set([...operatives, ...hostiles].filter(x => x.state !== 'dead')), resolve };
  engagements.set(g, E);
  return E;
}
/** Close the running engagement with `outcome` (no-op when none is running or it is already done). */
function endEngagement(g: Game, outcome: EngageResult['outcome']) {
  const E = engagements.get(g); if (!E || E.done) return;
  const living = (xs: Guard[]) => xs.filter(x => x.state !== 'dead' && g.guards.includes(x) && !x.fled).length;   // (runner.ts: a man who dropped it and ran is nobody's living hostile)
  E.done = true; E.result = { outcome, secs: g.time - E.t0, kills: E.kills.slice(), rounds: { ...E.rounds }, operativesLeft: living(E.operatives), hostilesLeft: living(E.hostiles) };
  E.resolve(E.result);
}
