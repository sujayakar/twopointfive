// AN EMPTY GUN IS RELOADED WITHOUT BEING ASKED (the maintainer, 10-10: "character should auto reload when empty").
//
// The player's man used to stand over a dry gun until R: the last round said 'empty — press R' and every click after it was a dry click. Now the gun in his hands,
// run dry with a magazine left for it, starts its reload by itself a beat after the last round — through THE SAME line of player.ts the R key goes through, so every
// condition that line has (gun up, not sprinting, not on the back-to-the-wall hold), the staged beat, its sounds and whatever the animator plays for an empty gun
// are the R key's own. R is still his for a magazine that is only part spent.
//   · the rule is a READING of one player's state, asked every frame: nothing is latched, nothing new is saved. A reload something took out of his hands (a sprint,
//     the holster, a takedown) starts again by itself once the gun is back up and still dry — also one broken off after the magazine went in and before the
//     rack (a full magazine under an empty chamber) — as does a gun drawn dry;
//   · never with nothing to seat (the line says so instead), never on unlimited ammo (that gun does not run dry), never under a show or the puppet (their beats
//     press their own keys);
//   · teammates and defenders have always reloaded by themselves (guards.ts startReloadBeat, roomfight.ts reloadUntil): nothing of theirs is touched.
// Type-only game imports: nothing here is built from another game module's exports at load (src/game is one import cycle).
import type { Game, Player } from './game';
import type { Firearm } from './weapons';
import { goneWithTick } from './crossing';

/** the switch and its one number: off in the module, raised by the page (host/profile.ts pageSwitches; ?noautoreload = a dry gun waits for R, as it did) and by the
 *  shipped stand; the harness's frozen stand leaves it off (TPS_AUTORELOAD=1 turns it on). `delay` = seconds after the last round before the hands move (UNTUNED by eye: the round's recoil is seen first). */
/** `lastMag` = THE LAST MAGAZINE, SEATED AND NOT RACKED, IS STILL A RELOAD: a reload broken off between 'magazine in' and the rack with no spare left stood as a
 *  full magazine under an empty chamber that R refused ('no spare magazines') and nothing restarted — a gun that never fired again. On, R and the rule above
 *  take it: the magazine comes out, goes back in and the handle is racked (weapons.ts startReload). Off in the module and on the frozen stand
 *  (TPS_LASTMAG=1), on in the page (?nolastmag = the build as it was). */
export const RELOAD = { auto: false, delay: 0.25, lastMag: false };

/** would THIS player's `gun` (the one in his hands: the caller's own test) start its reload by itself this frame? */
export function autoReloadDue(g: Game, pl: Player, gun: Firearm): boolean { return goneWithTick('autoReloadDue') as any; }

