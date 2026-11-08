// HANDS THAT DO THE WORK — the three biggest gaps on the missing-animations audit, closed with small layers (character.ts workLayer / radioLayer / swapLayer;
// nothing here is a clip):
//
//   WORK   one "working at a thing" pose, fed wherever a man already stands at something and uses it: Sam keying a door code (a forefinger on the pad's own keys,
//          the beeps on its presses), a charge going on a door or a wall (Sam's and a teammate's), the mains breaker (Sam's hand throws the lever — the lights go
//          WITH the hand; a guard re-arming it), the objective coming out of its rack (both hands, then away at his hip). Sam is walked the step in to arm's length
//          and squared to the thing for as long as the verb runs, as he already was at a lock and a keypad; the trigger lets go of the drive and a charge — the
//          lever's half second is committed, like a kick (its row's deed is filed on the tap).
//   RADIO  a line that goes over the net (guards.ts say) lifts the speaker's left hand to the handset at his collar for about as long as the line.
//   SWAP   a guard's torch ↔ sidearm change is a move of his hand — torch to the belt, hand to the holster, gun up; and back — instead of a change on the frame.
//          The RULE (guards.ts handCarry) is untouched: what he may do with the gun, and when, is exactly what it was, and his BORE and MUZZLE are read off the
//          arm UNDER the swap (character.ts handPre): a line of fire and a round's start are where they are with it off; a shot mid-draw jumps the move's end.
//
// One switch, HANDWORK.on — the page: on (`?nohandwork` off); the frozen harness stand: off (`TPS_HANDWORK=1` on) — and its three halves for A/B on the panel.
// What changes for the SIMULATION with it on (so the frozen stand holds it off): Sam's breaker takes the half second his hand needs, a code waits for his finger
// to reach the pad, and he is planted while he pulls the drive / sets a wall charge / throws the breaker. The radio hand and the guards' swap move nothing of
// the game but what the picture shows: the torch's beam goes to the belt with the hand for the swap's third of a second, and a man whose gun is DOWN tips his
// head to his handset as he does for a tell (his eyes go with it); with his sights up the head stays. A man dropped mid-swap falls from the arm UNDER the swap
// (character.ts underPose: he lies where he lies with it off); one dropped with his hand at his radio falls as he was drawn. The animator's clocks are not in a
// checkpoint (as no layer's are): a load mid-swap finds the hand already there.
//
// State: none of its own beyond two fields on the Player (breakerMove, handWait — a save is refused while either runs: savestate.ts playerBusy); everything else
// is fed a frame at a time and lives in the animator, like every other layer's clocks.
import type { Game, Guard } from './game';
import type { Character, WorkKind, WorkPose } from './character';
import './character';
import { Vec3 } from '../math/vec';
import { Box, BoxFlag, makeBox } from '../scene/boxes';
import './mission';
import { quietOf } from './quiet';
import './lighting';
import { switchHand } from './lightswitch';
import './samstun';
import { INHAND, BELT_TORCH } from './inhand';
import { goneWithTick } from './crossing';

export const HANDWORK = {
  /** THE switch — the page: on (`?nohandwork` off); the frozen stand: off (`TPS_HANDWORK=1` on) */
  on: false,
  /** its three halves, for A/B on the panel */
  work: true, radio: true, swap: true,
  /** m off the thing's face he is stood to work at it · how he is walked there (an ease, 1/s, at no more than this pace, m/s) · m to one side, so the working shoulder is before the spot */
  standM: 0.52, stepRate: 7, stepPace: 1.7, standSide: 0.1,
  /** s a verb's clock waits, at most, for his hand to reach the thing (a hand that cannot get there keys nothing — the verb still goes) */
  waitSecs: 0.7,
  /** Sam's breaker: s the move takes once his hand is on the lever · s into it the lever goes over (the lights with it) */
  leverSecs: 0.5, leverAt: 0.3,
  /** the objective: s the hand takes putting it away once it is out */
  stowSecs: 0.45,
  /** the radio hand: × the line's own time on screen, and its bounds (s) */
  radioK: 0.6, radioMin: 0.9, radioMax: 2.4,
};
/** QA: how often each was asked for (probes read them; nothing else does) */
export const HANDWORK_STATS = { work: 0, radio: 0, swaps: 0, levers: 0 };

const live = (half: 'work' | 'radio' | 'swap') => HANDWORK.on && HANDWORK[half];
export const handworkOn = live;

// ---------------------------------------------------------------- the work pose
/** feed one man's work pose for this frame (see WorkPose) — nothing with the switch off */
export function workAt(c: Character, kind: WorkKind, p: Vec3, n: Vec3, t: number, dur: number, more?: Pick<WorkPose, 'keys' | 'dir' | 'hand'>) {
  if (!live('work')) return;
  c.anim.work = { kind, p, n, t, dur, ...more }; HANDWORK_STATS.work++;
}
/** a level normal out of a thing's face toward a man, when the code that owns the thing keeps none: the axis nearest the line from the spot to him */
export function faceToward(spot: Vec3, from: Vec3): Vec3 { const dx = from[0] - spot[0], dz = from[2] - spot[2]; return Math.abs(dx) > Math.abs(dz) ? [Math.sign(dx) || 1, 0, 0] : [0, 0, Math.sign(dz) || 1]; }
/** WHICH OF SAM'S HANDS works a thing — the one rule, for the step in (workStand) and the pose (as the hint: the animator cannot tell a pistol in a hand that
 *  a plant has lowered): the left while the right has the pistol, the rifle's grip or a can in its fist, else the right */
export const samWorkHand = (g: Game): 0 | 1 => { const pl = g.player; return pl.pistolUp || pl.rifleUp || pl.char.anim.grip[1] ? 0 : 1; };
/** …as workAt is told it: the off hand named where his own is full (a plant puts his gun's pose away, so the picture cannot tell by itself) */
export const samHand = (g: Game): 0 | undefined => samWorkHand(g) ? undefined : 0;



// ---------------------------------------------------------------- Sam's plants: the breaker, the drive, a wall charge
export interface SamWork { kind: 'breaker' | 'drive' | 'wall' | 'door' | 'switch'; p: Vec3; n: Vec3; /** where the verb's owner thinks he stands (the wall charge breaks off if he shifts from it): kept on him as he is walked in */ at?: Vec3 }
/** the face of the breaker panel toward a man: the panel box's thin axis when it has one, else the axis nearest him */
function breakerFace(g: Game, from: Vec3): { p: Vec3; n: Vec3 } {
  const B = g.level.breaker, b: Box | undefined = g.engine.world.statics[B.boxes[0]];
  if (b && !b.yaw && !b.rot && Math.min(b.h[0], b.h[2]) < 0.5 * Math.max(b.h[0], b.h[2])) {
    const n: Vec3 = b.h[2] < b.h[0] ? [0, 0, Math.sign(from[2] - b.c[2]) || 1] : [Math.sign(from[0] - b.c[0]) || 1, 0, 0];
    return { p: [b.c[0] + n[0] * b.h[0], b.c[1], b.c[2] + n[2] * b.h[2]], n };
  }
  return { p: B.pos, n: faceToward(B.pos, from) };
}
/** the face of the objective's holder toward a man, as mission.ts rackDist has it: the office's rack is a width along x that faces ±z (so a man reaching it from
 *  off to one side still squares to the FACE, not to the line from its middle to him); a quiet mission's holder faces any way — the way he came at it */
function rackFace(g: Game, from: Vec3): { p: Vec3; n: Vec3 } {
  const rf = g.level.mission.rack.front;
  return { p: rf, n: quietOf(g) ? faceToward(rf, from) : [0, 0, Math.sign(from[2] - rf[2]) || 1] };
}
/** what Sam's hands are on, if a verb of his that plants him is running (player.ts asks before his own movement; null with the switch off) */
export function samWork(g: Game): SamWork | null {
  if (!live('work') || g.puppet) return null;
  const pl = g.player, p = pl.char.pos;
  if (pl.breakerMove) return { kind: 'breaker', ...breakerFace(g, p) };
  if (g.mission.stage === 'drive' && g.mission.driveT >= 0) return { kind: 'drive', ...rackFace(g, p) };
  const J = g.wallCharge?.samJob; if (J) return { kind: 'wall', p: J.p, n: J.n, at: J.at };
  const D = g.doorTools?.samJob; if (D) return { kind: 'door', p: D.p, n: D.n, at: D.at };   // (a door charge by hand: owncharge.ts)
  const S = switchHand(g); if (S) return { kind: 'switch', ...S };   // (a room's wall switch: handring.ts `turn` — the lights went on the press, he is turned to the plate for his hand's move)
  return null;
}

/** lighting.ts: a guard at the thrown breaker, re-arming it — his hand on the lever for the `dur` he fumbles, the throw at the end */
export function leverHands(g: Game, c: Character, t: number, dur: number) { if (!live('work')) return; const F = breakerFace(g, c.pos); workAt(c, 'lever', F.p, F.n, t, dur, { dir: -1 }); }



/** mission.ts, once a frame of the drive stage: both hands in the rack while it comes out, then the thing put away at his hip; `pulled` = this is the frame it came free */
export function driveHands(g: Game, secs: number, pulled: boolean) {
  if (!live('work')) return;
  const pl = g.player, c = pl.char, m = g.mission;
  if (pulled) { c.anim.workAfter(HANDWORK.stowSecs); return; }
  if (m.driveT < 0) return;
  const F = rackFace(g, c.pos);
  workAt(c, 'take', F.p, F.n, m.driveT, secs, { hand: samHand(g) });
}
/** the thing in his hand while it comes out and is put away: a caddy-sized box off the working hand (combat.ts handProps calls it for Sam) */
export function workProps(c: Character, out: Box[]): void { return goneWithTick('workProps') as any; }

// ---------------------------------------------------------------- the radio hand
/** guards.ts say, a line that goes over the net: his hand goes to the handset for about as long as the line stands (`secs` = its time on screen) */
export function radioHand(gd: Guard, secs: number) {
  if (!live('radio') || gd.state === 'dead' || gd.civ) return;
  HANDWORK_STATS.radio++;
  gd.char.anim.radio(Math.min(HANDWORK.radioMax, Math.max(HANDWORK.radioMin, secs * HANDWORK.radioK)));
}

/** what his gun hand VISIBLY holds this frame (combat.ts handProps, guards.ts for the light): the rule's answer, but for the frames of a swap */
export function swapHeld(gd: Guard, held: 'torch' | 'weapon' | null): 'torch' | 'weapon' | null {
  return live('swap') ? gd.char.anim.swapHeld(held) : held;
}
/** the torch on his belt while the sidearm is out (and for the beat of the swap the hand is empty): hung at the front of the right hip, lens down, dark */
export function beltTorch(c: Character, out: Box[]) {
  const hp = c.bones.hips; if (!hp) return;
  const f = c.forward(), r: Vec3 = [-f[2], 0, f[0]];   // (his right: model −X)
  const B = INHAND.on && INHAND.belt ? BELT_TORCH.at : BELT_TORCH.was, o = B[0], a = B[1];   // (inhand.ts: at 0.15 / 0.115 three quarters of it was under his shirt — measured on the worn mesh of every defender and responder; outside it at 0.21 / 0.16)
  const at: Vec3 = [hp[0] + r[0] * o + f[0] * a, hp[1] - 0.045, hp[2] + r[2] * o + f[2] * a];
  out.push(makeBox({ c: at, h: [0.017, 0.062, 0.017], yaw: Math.atan2(f[0], f[2]), albedo: [0.05, 0.05, 0.055], flags: BoxFlag.Dynamic | BoxFlag.NoShadow, owner: c.id & 0xff }));
  out.push(makeBox({ c: [at[0], at[1] - 0.07, at[2]], h: [0.021, 0.012, 0.021], yaw: Math.atan2(f[0], f[2]), albedo: [0.07, 0.07, 0.075], stuff: 'metal', flags: BoxFlag.Dynamic | BoxFlag.NoShadow, owner: c.id & 0xff }));   // (`stuff`: what the look made of this grey by its COLOUR until 11-04, kept so the picture did not move; nobody chose it. scene/stuff.ts drawnAs)
}
