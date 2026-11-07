// HANDS ON THE WORLD — picking a body up, and working a door by hand. The player's two everyday chores done with his hands and his feet instead of by a state
// flip: `HANDS.on` (the page: on, `?nohands` = as it was; the frozen stand: off, `TPS_HANDS=1`). Off = nothing here runs and every caller takes its old path.
//
// A BODY (HANDS.body). Taking hold of one is a MOVE, not a toggle:
//   · it is taken by the END HE IS AT — the ankles, or under the arms from the head end (the two shoulder particles) — and it is in reach from either end
//     (bodyGripDist: the old reach knew the ankles and the hips only, so a man come back to a body from its head was offered nothing: the 'second time' bug);
//   · reach → he steps in to arm's length, crouches, bows, and both hands go down onto the two limbs where they lie (character.ts haulLayer);
//   · rise  → the pins come on where the limbs are and travel up into his hands at carry height (a lift, not a snap);
//   · carry → as ever: the two particles pinned at his hands, the rest trailing — his hands stay ON them (the layer follows the ragdoll's own particles);
//   · set   → the 'let go of him' row lowers them to the floor before the pins come off. Everything that TEARS a body out of his hands (a sprint, a wedge, a
//     round, a grab) is the old instant release.
//   CARRIED, NOT DRAGGED (HANDS.carry; off = hauled along the floor by the end he took, the four beats above — which is also what he does where there is no
//   room to stand). The same four beats, another hold: he steps in to its HIPS, both hands go down onto it, and it comes up off the floor onto his LEFT
//   SHOULDER (hoistSecs) as he stands — belly on the shoulder, trunk down his back, thighs down his chest under his left arm, the rest of it hanging by its own
//   weight (the ragdoll's hips, spine and knees pinned to spots in HIS frame: a rigid lie, see Lie; every length is the body's own, so no stick is strained).
//   He then WALKS with it, upright, facing the way he goes, at carryPace × his walk; 'put him down' kneels and lays it on its back on clear floor beside or
//   before him (layLie: never through a wall — no clear floor in reach = it is dropped off the shoulder, as a sprint drops it).
//   State: Player.haul (null whenever Player.dragging is). Under a puppet (the showcases) there is none: their drags are the old ones on the old clock.
//   With both hands on a body his gun is put away for the haul (character.ts hideHeldItem) — and its rail light with it, as for any hidden item.
//
// A DOOR (HANDS.door). The 'push it open' / 'pull it open' / 'pull it shut' / 'push it shut' / 'unlock it' rows start a DOOR MOVE (Player.doorMove) instead of
// flinging the leaf:
//   · his free hand goes onto the leaf (game.ts handsOnDoors takes the move's door as it takes an eased one) and the leaf comes round UNDER it at a hand's
//     pace — the same creak / key clack on the way open (Doors.use says them), the same sprung last hand's breadth and latch on the way shut (Doors.pullTo);
//   · the leaf never goes into him: where its sweep crosses the spot he stands on he STEPS CLEAR first (clearSpot: the nearest standing room on his side of
//     the wall that the sweep misses), and the leaf waits for his body rather than pass through it or stall against it;
//   · a leaf that opens TOWARD him is never left standing between him and the doorway: from the hinge side he comes out round its free end as it comes
//     (clearSpot's `toward` half, HANDS.roundM; the leaf waits clear of his way) and ends on the opening's side of it, a stride from walking in;
//   · no such room (a cupboard) → the move is refused and the row does what it always did (the leaf comes past him), so nobody is ever shut in;
//   · a step of his own takes his feet back at once; a leaf that makes no way for half a second (a body, a chair) is left where it stands;
//   · (two-way doors) a limp body lying in the leaf's swing → the move is refused and the row does what it always did: there only a flung leaf leaving its
//     latch moves a body aside (doors.ts shoves one for a hand on one-way doors alone), so a leaf brought round by hand stopped a few degrees open on a limb;
//   · his gun stays up and on the cursor while the leaf is where it already points (doorMoveLowers); a leaf beside or behind his aim turns him to it;
//   · (one-way doors) a leaf brought round toward him passes the teammates stood with him, as the flung one did (Door.pullT) — only ever while clear of HIM.
//   Not under a puppet: the showcases' doors keep their own timings.
//
// Habits: nothing here rolls a die or reads the camera, the canvas or a graphics setting; every input is a row performed (Game.act); all state is the
// Player's (per player), short-lived, and counted as 'busy' by the checkpoints (savestate.ts playerBusy), so it needs no slice of its own.
import './seats';
import { Vec3, v3, clamp } from '../math/vec';
import type { Game, Guard } from './game';
import type { Door } from './doors';
import './doors';
import { RAG } from './ragdoll';

export const HANDS = {
  /** THE switch — the page: on (`?nohands` off); the frozen stand: off (`TPS_HANDS=1` on) */
  on: false,
  /** its two halves, for A/B on the panel */
  body: true, door: true,
  // ---- a body
  /** s for the hands to go down onto it · s for it to come up into them · s to lay it down */
  reachSecs: 0.3, liftSecs: 0.35, setSecs: 0.3,
  /** he steps in until the grip is this near (m), and gives up stepping after this long (s): the pins then gather it in as they always did */
  gripM: 0.55, stepSecs: 1.0,
  /** the head end is taken when it is this much nearer than the feet (m) */
  headBias: 0.1,
  /** m the grip stands ahead of him: the ankles · the shoulders (further: his head and arms hang on this side of them, and are not to hang in his knees) · carry
   *  height of the ankles / the shoulders */
  ahead: 0.35, aheadHead: 0.48, carryFeet: 0.42, carryHead: 0.46,
  /** the head end has no leg sticks to stretch: it is 'wedged' when a held shoulder trails its hand by this (m) */
  slackM: 0.4,
  // ---- a body carried
  /** over the shoulder, not along the floor (false = the haul by the end he took, as it was) */
  carry: true,
  /** s it takes from the floor onto his shoulder · s back down onto the floor (a pin travels at most 2.4 m/s — ragdoll.ts PIN_STEP — and both are eased: the
   *  peak is 1.5 × the 1.5 m over the time, so neither may be much under a second or the five pins fall out of step and the body is let go still in the air) */
  hoistSecs: 1.0, lowerSecs: 1.1,
  /** s he may take stepping round to its hips before the lift (past it: lifted from where he is if that is near enough, else hauled along the floor) · how near is near enough beyond gripM (m) */
  carryStepSecs: 2.4, carryNearM: 0.35,
  /** where the middle of its hips rides, in his frame (m): toward his left · ahead · above his soles */
  carrySide: 0.2, carryFwd: -0.02, carryUp: 1.56,
  /** how its trunk hangs down his back (m back per m down) · how its thighs hang down his chest (m ahead per m down) */
  carryBack: 1.35, carryLegs: 1.1,
  /** his pace under it, × his walk · how fast he comes round with it (rad/s: the pins are kinematic and travel at most 2.4 m/s) */
  carryPace: 0.6, carryTurn: 5,
  /** how much of his own step and turn its free limbs are taken through with him each frame (ragdoll.ts ride): 0 = none, as it was — the body's damping is against
   *  the floor, and every arm, shin and the head stream out behind each stride and turn, 40–75° off the hang · 1 = all of it: they hang off him as off a still man */
  carryRide: 0.85,
  /** laid down: how far off his feet its hips go, on the side he lays it (m) */
  layAhead: 0.62,
  // ---- a door
  /** s the hand has at least before the leaf starts · and at most, when it cannot reach it · rad/s under the hand · where an opened leaf is left (rad, short of its stop) */
  doorReach: 0.12, handSecs: 0.3, doorRate: 2.6, openTo: 1.5,
  /** clearance kept between his centre and the leaf's line beyond their own sizes (m) · how far he will step to clear a sweep (m) */
  clearM: 0.06, stepM: 1.3,
  /** a leaf brought round TOWARD him: how far he will walk to end on the OPENING's side of where it will stand, rather than in the pocket behind it */
  roundM: 1.8,
  /** opening a leaf away from him from further off than a hand reaches, he steps in to this far off its face (m) as his hand goes out — if that step is no longer than inM */
  handM: 0.5, inM: 0.75,
  /** s without the leaf making way before the move is dropped */
  stallSecs: 0.5,
  /** rad off his aim within which the leaf he works leaves his gun up and on the cursor */
  aimArc: 1.2,
  // ---- a door somebody else works
  /** every other man's hand on a leaf he works WITHOUT walking it open (teamDoorHand; false = the leaf goes by itself, as it was) · m from the handle spot the
   *  door tools sent him to at which his hand comes up · m from the leaf within which a hand stays on one he has just shoved or is pulling to · the pace he
   *  must be under for a shove to be his hand's (a man walking a leaf open has it already: game.ts handsOnDoors) · rad/s under which a leaf is no longer moving */
  team: true, teamPreM: 0.55, teamReachM: 0.95, teamPace: 0.9, teamVel: 0.25,
};

/** the pick-up of the body in Player.dragging (see the head) */
export interface Haul { end: 'feet' | 'head'; phase: 'reach' | 'rise' | 'carry' | 'set'; t: number; iL: number; iR: number; half: number; from: [Vec3, Vec3]; /** hips ↔ shoulders as he lay (m): the head end's wedge test */ trunk: number;
  /** carried over the shoulder (see the head): the body's own measures — hip to hip, hips to spine, the two thighs (m) — the lie the lift / the laying-down
   *  set out from, the lie on the floor the laying-down ends at, and his facing held through both */
  carry?: { dim: [number, number, number, number]; lie: Lie; to: Lie | null; yaw: number; /** where he stood and faced last frame of the carry (x, z, yaw): what its limbs ride */ at?: [number, number, number]; /** where he steps to for the lift: beside its hips, on his side of it (null: no standing room there — as near its hips as his line to them takes him) */ spot: Vec3 | null }; }
/** how a carried body LIES: the middle of its hips, the unit line left hip → right hip, the unit line hips → spine (square to that), where each thigh points */
export interface Lie { p: Vec3; u: Vec3; t: Vec3; kl: Vec3; kr: Vec3; }
/** a door being worked by hand (see the head): toward `target` rad; `goal` = where his feet are going to be clear of it (null: they are) */
export interface DoorMove { d: Door; open: boolean; target: number; t: number; stage: 'hand' | 'swing'; goal: Vec3 | null; stallT: number; best: number; /** the step being taken for him: the nearest it has got to `goal`, and for how long it has got no nearer */ stepBest: number; stepStall: number; /** the step takes him round the leaf's free end from behind where it will stand: the leaf waits clear of the whole of his way, not only of him */ round?: boolean; }

const bodyOn = () => HANDS.on && HANDS.body;
const mid = (a: Vec3, b: Vec3): Vec3 => [(a[0] + b[0]) / 2, (a[1] + b[1]) / 2, (a[2] + b[2]) / 2];

// ------------------------------------------------------------------------------------------------------------------------------------------ a body

/** floor distance from the player to an END of a body — the middle of its ankles / of its shoulders — or Infinity when a static stands between (a clear line at shin
 *  height, like player.ts bodyDist's own terms) */
function endDist(g: Game, gd: Guard, end: Haul['end']): number {
  const rag = gd.char.ragdoll; if (!rag) return Infinity;
  const p = g.player.char.pos, m = end === 'feet' ? mid(rag.point(RAG.ankL), rag.point(RAG.ankR)) : mid(rag.point(RAG.shoL), rag.point(RAG.shoR));
  return g.col.solidBlocked([p[0], 0.5, p[2]], [m[0], 0.5, m[2]]) ? Infinity : v3.distXZ(m, p);
}
/** player.ts bodyDist's extra term: the reach to a body's HEAD end, Infinity when the switch is off (or it is nowhere near: no line tested) */
export function bodyGripDist(g: Game, gd: Guard, within: number): number {
  const rag = gd.char.ragdoll; if (!bodyOn() || !rag) return Infinity;
  if (v3.distXZ(mid(rag.point(RAG.shoL), rag.point(RAG.shoR)), g.player.char.pos) >= within) return Infinity;
  return endDist(g, gd, 'head');
}

/** player.ts toggleDrag, the body just taken (Player.dragging set): which end, which hand has which limb, and the move's first beat */
export function haulBegin(g: Game, gd: Guard) {
  const pl = g.player, c = pl.char, rag = gd.char.ragdoll; pl.haul = null; if (!bodyOn() || !rag || g.puppet) return;
  if (HANDS.carry && standRoom(g, c.pos)) {   // over the shoulder: taken by the hips, from whichever side he is at
    const hl = rag.point(RAG.hipL), hr = rag.point(RAG.hipR), pm = mid(hl, hr);
    if (!g.col.solidBlocked([c.pos[0], 0.5, c.pos[2]], [pm[0], 0.5, pm[2]])) {
      const dim: [number, number, number, number] = [v3.dist(hl, hr), v3.dist(pm, rag.point(RAG.spine)), v3.dist(hl, rag.point(RAG.kneeL)), v3.dist(hr, rag.point(RAG.kneeR))];
      // he lifts it from BESIDE its hips (not stood on its legs or its chest): the spot a grip's length off them square to its trunk, on his side
      const ax = v3.sub(rag.point(RAG.spine), pm), n = unit([-ax[2], 0, ax[0]], [1, 0, 0]), sgn = n[0] * (c.pos[0] - pm[0]) + n[2] * (c.pos[2] - pm[2]) < 0 ? -1 : 1;
      let spot: Vec3 | null = [pm[0] + n[0] * sgn * HANDS.gripM, c.pos[1], pm[2] + n[2] * sgn * HANDS.gripM];
      if (g.col.solidBlocked([pm[0], 0.5, pm[2]], [spot[0], 0.5, spot[2]]) || !standRoom(g, spot)) spot = null;
      pl.haul = { end: 'feet', phase: 'reach', t: 0, iL: RAG.hipL, iR: RAG.hipR, half: 0.1, from: [v3.copy(hl), v3.copy(hr)], trunk: 0.45, carry: { dim, lie: lieOf(rag), to: null, yaw: c.bodyYaw, spot } };
      return;
    }
  }
  pl.haul = dragHaul(g, gd);
}
/** the haul along the floor: which end, which hand has which limb */
function dragHaul(g: Game, gd: Guard): Haul {
  const c = g.player.char, rag = gd.char.ragdoll!;
  const dh = endDist(g, gd, 'head'), df = endDist(g, gd, 'feet');   // (an end behind a wall is no end to take it by: the body is not to come through the wall to his hands)
  const end: Haul['end'] = dh + HANDS.headBias < df ? 'head' : 'feet';   // (Infinity: no line to that end)
  let iL: number = end === 'feet' ? RAG.ankL : RAG.shoL, iR: number = end === 'feet' ? RAG.ankR : RAG.shoR;
  const a = rag.point(iL), b = rag.point(iR), m = mid(a, b);
  // the limb on his right goes to his right hand (seen from where he stands, facing the grip)
  const fx = m[0] - c.pos[0], fz = m[2] - c.pos[2];   // his line to it; his right on the floor is (−fz, fx)
  if ((a[0] - m[0]) * -fz + (a[2] - m[2]) * fx > 0) { const k = iL; iL = iR; iR = k; }
  const trunk = Math.max(0.45, v3.dist(rag.point(RAG.pelvis), mid(rag.point(RAG.shoL), rag.point(RAG.shoR))));
  return { end, phase: 'reach', t: 0, iL, iR, half: end === 'feet' ? 0.1 : clamp(v3.dist(a, b) / 2, 0.1, 0.24), from: [v3.copy(rag.point(iL)), v3.copy(rag.point(iR))], trunk };
}

const unit = (v: Vec3, or: Vec3): Vec3 => { const l = v3.len(v); return l > 1e-6 ? v3.scale(v, 1 / l) : or; };
/** room for him to stand upright at p (player.ts canStand's own test, at a spot) */
function standRoom(g: Game, p: Vec3): boolean { const q: Vec3 = [p[0], p[1], p[2]]; g.col.collideCircle(q, 0.42, 1.0, 1.7, 2); return Math.hypot(q[0] - p[0], q[2] - p[2]) < 0.01; }
/** how the body lies now */
function lieOf(rag: { point(i: number): Vec3 }): Lie {
  const hl = rag.point(RAG.hipL), hr = rag.point(RAG.hipR), p = mid(hl, hr), u = unit(v3.sub(hr, hl), [1, 0, 0]);
  const t0 = v3.sub(rag.point(RAG.spine), p), t = unit(v3.mad(t0, u, -v3.dot(t0, u)), [0, 0, 1]);
  return { p, u, t, kl: unit(v3.sub(rag.point(RAG.kneeL), hl), [0, -1, 0]), kr: unit(v3.sub(rag.point(RAG.kneeR), hr), [0, -1, 0]) };
}
/** is he walking with it on his shoulder (player.ts: upright, facing the way he goes)? */
export function haulUpright(g: Game): boolean { const h = g.player.haul; return !!h?.carry && h.phase === 'carry'; }
/** player.ts: is he down on his heels for this beat? (along the floor: always; over the shoulder: until it is half way up, and from the moment he lays it down) */
export function haulCrouch(g: Game): boolean { const h = g.player.haul; return !h?.carry || h.phase === 'reach' || (h.phase === 'set' && h.t > HANDS.lowerSecs * 0.2) || (h.phase === 'rise' && h.t < HANDS.hoistSecs * 0.45); }



/** how fast his feet may go with a body: the step in to it at the crouch's pace, planted through the lift and the laying-down, the haul at its own (k × crouch) */
export function haulPace(g: Game): number { const h = g.player.haul; return !h || h.phase === 'carry' ? 0.6 : h.phase === 'reach' ? 1 : 0; }
/** …in m/s: the same along the floor; with it over his shoulder he walks, at carryPace × his walk */
export function haulSpeed(g: Game): number { return haulUpright(g) ? g.tune.playerWalk * HANDS.carryPace : g.tune.playerCrouch * haulPace(g); }

// ------------------------------------------------------------------------------------------------------------------------------------------ a door






/** player.ts cancelActions / a restart: hands off everything */
export function handsCancel(g: Game) { g.player.doorMove = null; }
export function handsSummary(g: Game): string {
  const pl = g.player, h = pl.haul, m = pl.doorMove;
  return `${HANDS.on ? 'on' : 'off'}${HANDS.on && !HANDS.body ? ' · bodies off' : ''}${HANDS.on && !HANDS.door ? ' · doors off' : ''}${h ? ` · body: ${h.end} ${h.phase}` : ''}${m ? ` · door: ${m.open ? 'opening' : 'shutting'}${m.goal ? ' (stepping clear)' : ''}` : ''}`;
}
