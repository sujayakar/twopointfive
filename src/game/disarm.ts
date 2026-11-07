// THE GRAB DISARMS HIM — the maintainer, 10-23: "we should also make the grab super satisfying. cool animation where sam disarms him and satisfying sounds",
// and of the whole close-in: "it should feel very smooth and slick and almost like a rhythm game".
//
// A man taken from behind with a gun in his hand used to have it knocked out on the grab's first frame (HOLD.dropGun: an item thrown clear with a roll of the
// dice, a magazine's clatter at half voice). With HOLD.disarm on, the grab is three beats:
//   THE CATCH   frame 0, as ever: the left arm goes round his throat (player.ts startGrab; the hold's own clock and phases are untouched — he is `held` on the
//               frame he always was). Its voice is `grabCatch`.
//   THE STRIP   HOLD.strip.take seconds in: Sam's right hand is on the gun and it is out of the man's fist — an item from that frame, `carried` in Sam's hand
//               (throwables.ts: placed here every frame, no physics); the man's hands are empty (Guard.gunLostAt, as HOLD.dropGun always stamped it). Voice
//               `gunStrip`. The pistol hold (Sam's right hand has his own gun) raps it out with the frame instead: it falls from where the man held it.
//   THE GUN DOWN  HOLD.strip.drop: he lets it fly from a straight arm out at his right. It turns flat as it goes (a fixed rate), lands most of a metre clear of
//               the pair with next to no bounce, and lies there: no dice. Voice `gunDown`, once, as it lands.
// then the hand comes back up behind the man's shoulder and onto its station on him (character.ts holdLayer; the knife is in that fist only from
// HOLD.strip.knife on — combat.ts asks disarmHand).
//
// WHAT THE ROOM HEARS is what it heard before: the grab's one 'prop' event at player.ts GRAB_NOISE, on the grab's frame. The strip and the gun on the floor
// are voices for the player's ears and no event of their own, as the gun knocked out of his hand never was one (combat.ts dropFromHand, quiet).
//
// However the hold ends under the move, nothing is left hanging: before the take the gun is still his (killGuard drops it, a freed man keeps it); after it the
// item is his `dropped` like any gun lost at a grab (a freed man has it back: guards.ts gunBack), and one still in Sam's hand with no hold to carry it for
// falls from where it is. A checkpoint is not written with an item off the floor (savestate.ts), and a hold is not filed at all.
//
// State: Hold.strip (player.ts), gone with the hold; Item.carried. No module state, no dice of its own (an item's spawn draws the four it always draws).
import './seats';
import type { Game, Guard } from './game';
import type { Item } from './throwables';
import './character';
import type { DropKind } from './weapons';
import { HOLD } from './character';
import { inHand } from './guards';
import { Vec3, v3 } from '../math/vec';

/** the move while it runs: its own clock, s (the hold's, but for HOLD.stripFirst, which runs it ahead until the gun is let go) · what he had (the item's kind, and whether it wore a can) · the man's gun hand as last posed
 *  (WORLD; held where it was once the gun is taken) · the gun while it is in Sam's hand · the pistol hold's (rapped out, never carried) · it has left him */
export interface Strip { t: number; kind: DropKind; supp: boolean; at: Vec3; item: Item | null; rap: boolean; gone: boolean; yaw: number; }
/** the three voices' gains (audio/sfx.ts grabCatch, gunStrip, gunDown) and the rate a long gun's strip and fall are played at (one buffer each: lower = heavier) */
export const DISARM_GAIN = { catch: 0.5, strip: 0.45, down: 0.4, long: 0.8 };

/** player.ts startGrab, once the hold exists: the gun still in his hand is the disarm's to take (HOLD.disarm, under HOLD.dropGun — with either off, nothing) */
export function disarmBegin(g: Game): void {
  const H = g.player.holding; if (!H || !HOLD.disarm || !HOLD.dropGun) return;
  const gd = H.g; if (inHand(gd) !== 'weapon') return;
  H.strip = { t: 0, kind: gd.weapon.def.drop, supp: gd.weapon.def.suppressed, at: handOf(gd), item: null, rap: H.variant === 'gun', gone: false, yaw: v3.yawOf(gd.char.gunDir) };
  if (leads(H.strip)) g.player.catchDue = g.time + HOLD.first.catch;
}
/** the grab's first voice (player.ts startGrab, every grab): the catch with the move's two keys on, the scuffle it always was without — so one condition
 *  decides all three voices. Its rate is pinned: the beat does not wander */
export function disarmCatch(g: Game, at: Vec3): void {
  if (leads(g.player.holding?.strip)) return;   // (HOLD.stripFirst: the strip is heard first, and the catch as the arm lands — disarmTick)
  if (HOLD.disarm && HOLD.dropGun) g.audio.play('grabCatch', at, DISARM_GAIN.catch, { rate: 1 }); else g.audio.play('grabScuffle', at, 0.4);
}
/** THE DISARM LEADS (HOLD.stripFirst) for this move: a gun to take with an empty hand (the pistol hold's rap comes down with the frame, as it did) */
const leads = (S: Strip | null | undefined): boolean => !!S && HOLD.stripFirst && !S.rap;


const handOf = (gd: Guard): Vec3 => v3.copy(gd.char.bones.handR ?? v3.add(gd.char.pos, [0, 1.05, 0]));

