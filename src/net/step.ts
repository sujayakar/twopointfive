// ONE TICK OF A NETWORKED GAME, the same on the server and on every client: the seats' records, then what a host does between two ticks that the next tick
// reads — the lights aged (a flash lives by the tick), the world's boxes listed again (the cursor's ray walks the list; the pack for the picture waits for a draw) and an expired
// speech bubble let go (the HUD's one write to the game: journal.ts hudExpireBubbles). The order is the page's own between two ticks of one frame
// (host/loop.ts betweenTicks) and a re-run's (tools/qa/rerun.ts). dt is exactly 1 / NET.hz.
import type { Game } from '../game/game';
import type { PlayerCmd } from '../game/cmd';
import { hudExpireBubbles } from '../game/journal';   // (called inside netStep only: src/game is one import cycle)
import { dressAll } from '../game/dress';   // (called inside netStand only)
import { NET } from './wire';
import { NET_AGREED } from './agreed';

/** the two things of a host's engine a tick's end touches (the page's engine, the headless stand-in) */
export interface StepHost { lights: { update(dt: number, t: number): void }; world: { relist(): void }; }

export function netStep(g: Game, host: StepHost, cmds: readonly PlayerCmd[]): void {
  const dt = 1 / NET.hz;
  g.stepSeats(dt, cmds);
  host.lights.update(dt, g.time); host.world.relist(); hudExpireBubbles(g);   // (the list alone: a tick is not a draw, and a server never draws — scene/boxes.ts relist; a page's next draw packs what it is handed)
}
/** WHAT THE HOST OF THE GAME THAT STEPS HANGS ON THE GAME OBJECT beside its `stepSeats` (src/host/rustgame.ts in the page, tools/blitz/rust-host.ts in bun):
 *    stateWords()   the game's own words for where things are, cheap enough for every tick: its die's count of draws and the hash of every part of its
 *                   block, in the block's own order (every body, door and thing; not a seat's aim, a man's mind or an order book)
 *    wholeWords()   the game's own words for the WHOLE of it, one a part (each field of the game, each module, each table), with what a host hands in
 *                   left out. Milliseconds a call. `wholeNames()` names the parts. NOT IN THE ALARM YET (netWhole below says why)
 *    quietRun(on)   a run of steps between which none of the caller's code reads the game: nothing is brought across to the host's copy of the game
 *                   (what the picture, the HUD and a tool read) until the run ends, and then once. A walk of one tick is the same step as ever. */
export interface StateWords { die: number; parts: ArrayLike<number>; }
interface Stepper { stateWords?(): StateWords | null; wholeWords?(): ArrayLike<number> | null; wholeNames?(): string[]; quietRun?(on: boolean): void; }
export const NET_WORD_SEED = 0x811C9DC5;
/** true for a tick (0-based) after which the server says its state word (wire.ts `wordEvery`) */
export const wordTick = (t: number): boolean => (t + 1) % NET.wordEvery === 0;
/** THE STATE'S WORD after a tick, chained from the word before it (wire.ts: after a word tick it is the alarm). The game's own words, never a walk of the
 *  host's copy of it, which inside a quiet run is not the game. */
export function netWord(g: Game, from: number): number {
  const W = (g as unknown as Stepper).stateWords?.();
  if (!W) throw new Error('netWord: no game steps here (a host hangs stateWords on the game beside stepSeats)');
  let h = Math.imul((from ^ W.die) >>> 0, 0x01000193) >>> 0;
  for (let i = 0; i < W.parts.length; i++) h = Math.imul(h ^ W.parts[i], 0x01000193) >>> 0;
  return h;
}
/** THE WHOLE GAME'S WORDS as they stand, with the parts' names: for whoever asks WHICH part two machines disagree on (tools/mp/words-diff.ts, a check).
 *  They are NOT the alarm: a page and a game server do not agree on every part yet (docs/agents/net.md: the picture's own carried in the game's
 *  record, rows numbered differently). The parts every kind of host does agree on are folded into the wider word, watched: netWatch below. */
export function netWhole(g: Game): { names: string[]; words: number[] } | null { const S = g as unknown as Stepper, A = S.wholeWords?.(); return A ? { names: S.wholeNames?.() ?? [], words: Array.from(A) } : null; }
/** THE WIDER WORD, WATCHED: the whole game's words of the parts on the one list of parts every kind of host has agreed on (net/agreed.ts), folded into
 *  one word, by NAME (a part the module has not got is passed over; two machines of one build pass over the same). Not chained: each word tick's stands
 *  alone, so a tick that differs is one tick counted. The server sends it beside the state word; a client holds its own against it and COUNTS
 *  (client.ts watchOk / watchOff): it is no alarm until real pages on real GPUs have shown the list holds. null where the host has no whole words */
let WATCH: { names: readonly string[]; ix: number[] } | null = null;
export function netWatch(g: Game): string | null {
  const S = g as unknown as Stepper, A = S.wholeWords?.(); if (!A) return null;
  const N = S.wholeNames?.() ?? []; if (!WATCH || WATCH.names !== N) { const on = new Set(NET_AGREED); WATCH = { names: N, ix: N.map((n, i) => on.has(n) ? i : -1).filter(i => i >= 0) }; }
  let h = NET_WORD_SEED; for (const i of WATCH.ix) h = Math.imul(h ^ A[i], 0x01000193) >>> 0;
  return h.toString(36);
}
/** a quiet run of networked ticks begins / ends (a server between two state words, a client's walk of the ticks its server has confirmed) */
export function netQuiet(g: Game, on: boolean): void { (g as unknown as Stepper).quietRun?.(on); }
/** A NETWORKED GAME IS STOOD THE SAME WAY ON EVERY MACHINE: once, when the game is stood, by the server, a scripted client and a page alike.
 *  Every seat is refereed (wire.ts: the view is worked out again from the pose). Every dressing row is up (game/dress.ts): no recipe carries that
 *  table, a page has it up and a host that draws nothing may have put it down, and in the game that steps the rows are not picture alone (a debris
 *  chunk and a glass plate are rows of the game's box table; a storm's wind source). So the stand does not leave it to how a host happened to be stood */
export function netStand(g: Game): void { for (const s of g.seats) s.remote = true; dressAll(true); }
