// TEAMMATES FALLING IN NEVER SHOVE YOU (NOSHOVE.on: on in the page, ?noshove=0; off here and on the frozen stand, TPS_NOSHOVE=1).
//
// What it answers: a man walking to his place under 'fall in' was as solid to the player as an enemy (player.ts keepOffGuards: "you are the one who yields"),
// so one whose place lay on the player's far side walked into him and shouldered him 0.2 m sideways. The squad then read that shove as the player WALKING
// (orders.ts measures his pace off the ground he covers): his place in the file moved, and a man keeping a taken room's door stepped out of "his way".
//
//   ROUND    (`round`) a man of the player's own under 'fall in', walked by his order (the fight has not got his feet), is the one who yields:
//              - his own step never takes him nearer the player than `ring` (or than he already stood): where the floor leaves his body room to pass on
//                one side he walks ROUND — the side of the player his way already lies on, else the other;
//              - his place itself inside the ring: he STOPS SHORT of it, at the ring;
//              - no room either side (a passage, a doorway): `through` on, he squeezes past as a man in a narrow doorway always did, and the player is not
//                moved; `through` off, he stops short and waits;
//              - left standing inside the player's own keep-off (a restart, a load, an enemy who shoved the player onto him): he steps back out at `back`.
//            Held short, his legs stop (squad.ts stands him: no walking on the spot, and the walker's own 'stuck' side-step is not woken).
//            The player, for his part, is never moved by such a man: only what his OWN legs closed on him this frame is taken back, so he still
//            cannot walk through a teammate. A man running from a live grenade is not such a man: he is as solid as he was.
//   OWN      (`own`) what the men he met moved the player by in a frame (anybody: a teammate on another order, an enemy walking into him) is not his
//            walking. orders.ts reads his pace and heading off his own step, less what the men took back of it: sliding round a man reads as the
//            ground he covers, a shove that only stops him counts as stopping, a shove of a standing player counts as nothing.
//
// Enemies are as solid as they were, and so is a teammate on any other order or in a fight. Off = the build as it was: every function here answers on its
// first line. State: three fields on the Player (`shoved`, `shovedT`, `shovedFrom`), written and read inside one update — nothing of them is carried from
// one frame to the next, so no save holds them. No dice.
import type { Game, Guard } from './game';
import { KEEP_OFF } from './player';
import { fragRunning } from './frag';
import { crossToRust, goneWithTick } from './crossing';

export const NOSHOVE = {
  on: false,
  /** a man falling in whose way lies through the player goes round him or stops short (false: he walks into him and the player is shouldered aside, as it was) */
  round: true,
  /** no room to go round: he squeezes past and the player is not moved (false: he stops short and waits) */
  through: true,
  /** how near the player's centre such a man's own steps take him (m; the player's own keep-off is 0.55) */
  ring: 0.6,
  /** a man left standing inside the player's keep-off steps back out at this pace (m/s) */
  back: 1.2,
};
/** frames a man was turned round the player · held short of him · let squeeze past · stepped back out; frames a shove was kept out of the player's pace */
export const NOSHOVE_STATS = { round: 0, short: 0, through: 0, back: 0, notWalked: 0 };

const BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.NOSHOVE_BREAK ?? '') === '1'; } catch { return false; } })();

/** is this a man the rule has: one of the player's own under 'fall in', on his order's legs (not on the legs a live grenade lends him: guards.ts runs
 *  that leg through the same walker under a borrowed script, and a man leaving a grenade goes straight) */
export function fallingInYields(g: Game, gd: Guard): boolean {
  if (!NOSHOVE.on || BREAK || !NOSHOVE.round || gd.state === 'dead' || !gd.script) return false;
  const A = g.orders.of(gd); return !!A && A.post === null && !fragRunning(g, gd);
}

/** what of a man the rule keeps, an update at a time: the player's own, or a look ahead's copy of him (walkahead.ts) */
export interface Shoved { char: { pos: ArrayLike<number> }; shoved: [number, number]; shovedT: number; shovedFrom: [number, number] }
/** player.ts updatePlayer, first thing: nobody has moved him yet this frame, and this is where his own legs start from */
export function noShoveBegin(g: Game, pl: Shoved = g.player): void {
  if (!NOSHOVE.on || BREAK) return;
  const p = pl.char.pos; pl.shoved[0] = pl.shoved[1] = 0; pl.shovedT = g.time; pl.shovedFrom[0] = p[0]; pl.shovedFrom[1] = p[2];
}
/** player.ts keepOffGuards: a man moved him by this much (after the walls had their say) */
export function noShoveNote(g: Game, dx: number, dz: number, pl: Shoved = g.player): void {
  if (!NOSHOVE.on || BREAK) return;
  if (pl.shovedT !== g.time) { pl.shoved[0] = pl.shoved[1] = 0; pl.shovedT = g.time; pl.shovedFrom[0] = pl.char.pos[0] - dx; pl.shovedFrom[1] = pl.char.pos[2] - dz; }
  pl.shoved[0] += dx; pl.shoved[1] += dz;
}
/** player.ts keepOffGuards, of a man the rule has who stands at (mx, mz), `l` from the player: how far the player is put back from him — what his OWN legs
 *  closed on the man this update, and no more (0: the man came to him). Measured from where he would stand had his legs not moved, so a walk past a man's
 *  shoulder never works its way inside the keep-off a frame at a time. */
export function noShoveBack(g: Game, mx: number, mz: number, l: number, pl: Shoved = g.player): number {
  if (pl.shovedT !== g.time) return 0;
  const d0 = Math.hypot(pl.shovedFrom[0] + pl.shoved[0] - mx, pl.shovedFrom[1] + pl.shoved[1] - mz);
  return Math.max(0, Math.min(KEEP_OFF, d0) - l);
}

/** orders.ts tick: the player has covered (dx, dz) since the last tick — the step his own legs took of it, or null: not asked (the switch is off).
 *  A frame in which no man moved him reads as it always did. One in which a man did reads the step his own legs took inside that update (what a door or
 *  a blast did to him since the last tick is not his walking either), less what the men took back of it — the part of their shove that runs against his
 *  step, and no more than they gave: sliding round a man is the ground he covers, walking into one who stands is standing still, and a man who walks
 *  into a standing player adds nothing. */
export function noShoveOwn(g: Game, dx: number, dz: number): [number, number] | null { return goneWithTick('noShoveOwn') as any; }

/** squad.ts runGuardScript, after his step and its collisions: he stood at (x0, z0) before it. See ROUND in the head. True: he is held short this frame
 *  (the caller stands his legs). */
export function noShoveKeep(g: Game, gd: Guard, x0: number, z0: number, dt: number): boolean { return crossToRust('noShoveKeep', undefined, [g, gd, x0, z0, dt]) as any; }

