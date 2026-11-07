// ONE TICK OF A NETWORKED GAME, the same on the server and on every client: the seats' records, then what a host does between two ticks that the next tick
// reads — the lights aged (a flash lives by the tick), the world's boxes listed again (the cursor's ray walks the list; the pack for the picture waits for a draw) and an expired
// speech bubble let go (the HUD's one write to the game: journal.ts hudExpireBubbles). The order is the page's own between two ticks of one frame
// (host/loop.ts betweenTicks) and a re-run's (tools/qa/rerun.ts). dt is exactly 1 / NET.hz.
import type { Game } from '../game/game';
import type { PlayerCmd } from '../game/cmd';
import { hudExpireBubbles } from '../game/journal';   // (called inside netStep only: src/game is one import cycle)
import { NET } from './wire';

/** the two things of a host's engine a tick's end touches (the page's engine, the headless stand-in) */
export interface StepHost { lights: { update(dt: number, t: number): void }; world: { relist(): void }; }

export function netStep(g: Game, host: StepHost, cmds: readonly PlayerCmd[]): void {
  const dt = 1 / NET.hz;
  g.stepSeats(dt, cmds);
  host.lights.update(dt, g.time); host.world.relist(); hudExpireBubbles(g);   // (the list alone: a tick is not a draw, and a server never draws — scene/boxes.ts relist; a page's next draw packs what it is handed)
}
/** WHAT THE HOST OF THE GAME THAT STEPS HANGS ON THE GAME OBJECT beside its `stepSeats` (src/host/rustgame.ts in the page, tools/blitz/rust-host.ts in bun):
 *    stateWords()   the game's own words for its state: its die's count of draws and the hash of every part of its block, in the block's own order
 *                   (where every body, door and thing is: not a seat's aim, a man's mind or an order book, which are said once they move something)
 *    quietRun(on)   a run of steps between which none of the caller's code reads the game: nothing is brought across to the host's copy of the game
 *                   (what the picture, the HUD and a tool read) until the run ends, and then once. A walk of one tick is the same step as ever. */
export interface StateWords { die: number; parts: ArrayLike<number>; }
interface Stepper { stateWords?(): StateWords | null; quietRun?(on: boolean): void; }
export const NET_WORD_SEED = 0x811C9DC5;
/** THE STATE'S WORD after a tick, chained from the word before it (wire.ts: every `wordEvery` ticks it is the alarm). The game's own words, never a walk
 *  of the host's copy of it: inside a quiet run that copy is not the game */
export function netWord(g: Game, from: number): number {
  const W = (g as unknown as Stepper).stateWords?.();
  if (!W) throw new Error('netWord: no game steps here (a host hangs stateWords on the game beside stepSeats)');
  let h = Math.imul((from ^ W.die) >>> 0, 0x01000193) >>> 0;
  for (let i = 0; i < W.parts.length; i++) h = Math.imul(h ^ W.parts[i], 0x01000193) >>> 0;
  return h;
}
/** a quiet run of networked ticks begins / ends (a server between two state words, a client's walk of the ticks its server has confirmed) */
export function netQuiet(g: Game, on: boolean): void { (g as unknown as Stepper).quietRun?.(on); }
/** every seat of a networked game is refereed on every machine (wire.ts: the view is worked out again from the pose). Once, when the game is stood */
export function netSeats(g: Game): void { for (const s of g.seats) s.remote = true; }
