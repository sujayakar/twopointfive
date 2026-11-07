// THE PLAYER AND HIS OWN SIDE (the maintainer playing main, 10-11): two small rules about Sam's gun and the men who are not his enemies.
//
//   noSnap    "also shouldn't have any targeting overlays on friendlies" — the reticle used to take ANY living man of `g.guards` within its snap reach, so
//             a cursor near a teammate drew the red ring on him, laid Sam's aim on his chest and handed him to everything that reads `g.aimGuard` (the
//             ring airfoil's point, the OCP's torch test). On: a man who is not hostile to the player is never the reticle's — not by the loose snap, not
//             wounded, not with the cursor's own ray on his body. The aim is then the free aim along the cursor, as over empty floor.
//   lowReady  "we should have muzzle discipline for our character too! if we're aiming towards a friendly and not in combat, we go to low ready" — a rule
//             like the one his teammates have had (guards.ts muzzleCheck; its own numbers: a flat lane, no taper), for him: a gun that is up, a teammate (or
//             a rescue's hostage) standing in the line it is held along, NO FARTHER THAN WHAT IT IS AIMED AT (+ `pastM`: a friend beyond the man or the spot
//             under the reticle is not being pointed at) and within `reachM`, with nothing solid between — or, the aim free, the cursor on a teammate's body
//             — and nobody in a fight → the low ready, through the same seam a wall lowers it by (player.ts `walled`: the rifle's low carriage, the
//             pistol's stance, no round or launcher while it is down, a line that says why on a click). Down at once, up `upSecs` after the line is
//             clear, and up at once when the floor goes loud. IN COMBAT = inside `quietS` of Sam's last round or of the last loud thing in a living
//             teammate's ears (roomfight.ts hotOf: a shot, a charge, a boot), or a man hostile to him alert or searching. Never on the wall hold (the
//             stack's rifle is the wall's), with a man in his arm, hauling, or under a show / puppet.
//
// Both ON in the page (`?friendsnap` / `?nomuzzle` = as it was), both OFF in the module and on the frozen harness stand (`TPS_FRIENDS=1` / `TPS_MUZZLE=1`):
// FROZEN_HOLDS rows, in the knob registry and a journal's tables; a recording from before the table replays with both off. State: two fields on the Player
// (`friendLow`, `friendT` = the game's clock when a friend was last in the line) — a quarter second of hysteresis that runs out by itself through a beat that
// does not ask (a kick, a takedown), not saved (a load starts with the gun where the next frame's test puts it). No dice.
import type { Game, Guard } from './game';
import './teams';
import { goneWithTick } from './crossing';

export const FRIENDS = { noSnap: false, lowReady: false, reachM: 8, pastM: 0.3, clearM: 0.35, leadS: 0.3, leadMaxM: 0.4, upSecs: 0.25, quietS: 5 };
export const FRIENDS_STATS = { lowered: 0, loweredS: 0 };

/** the reticle passes this man over: he is not the player's enemy (player.ts updateAim, all three of its loops) */
export function friendNoSnap(gd: Guard): boolean { return goneWithTick('friendNoSnap') as any; }

/** is anybody in a fight, as far as Sam's own gun is concerned (see the head comment) */
export function friendsInFight(g: Game): boolean { return goneWithTick('friendsInFight') as any; }



