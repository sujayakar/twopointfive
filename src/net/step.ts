// ONE TICK OF A NETWORKED GAME, the same on the server and on every client: the seats' records, then what a host does between two ticks that the next tick
// reads — the lights aged (a flash lives by the tick) and the world's boxes listed again (the cursor's ray walks the list; the pack for the picture waits
// for a draw). The order is the page's own between two ticks of one frame
// (host/loop.ts betweenTicks) and a re-run's (tools/qa/rerun.ts). dt is exactly 1 / NET.hz.
import type { Game } from '../game/game';
import type { PlayerCmd, Seat } from '../game/cmd';
import { dressAll } from '../game/dress';   // (called inside netStand only)
import { walkAhead } from '../game/walkahead';   // (called inside netAhead only: src/game is one import cycle)
import { legsOwn } from '../game/player';
import { NET, type WireCmd } from './wire';
import { fromWire } from './recipe';
import type { Ahead } from './foresee';
import { SIM_PARTS } from '../sim/state';
import { agreedOf, foldWhole, foldWord } from './words';

/** the two things of a host's engine a tick's end touches (the page's engine, the headless stand-in) */
export interface StepHost { lights: { update(dt: number, t: number): void }; world: { relist(): void }; }

export function netStep(g: Game, host: StepHost, cmds: readonly PlayerCmd[]): void {
  const dt = 1 / NET.hz;
  g.stepSeats(dt, cmds);
  host.lights.update(dt, g.time); host.world.relist();   // (the list alone: a tick is not a draw, and a server never draws — scene/boxes.ts relist; a page's next draw packs what it is handed)
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
/** THE STATE'S WORD after a tick, chained from the word before it (wire.ts: after a word tick it is the alarm). The game's own words, never a walk of the
 *  host's copy of it, which inside a quiet run is not the game. */
export function netWord(g: Game, from: number): number {
  const W = (g as unknown as Stepper).stateWords?.();
  if (!W) throw new Error('netWord: no game steps here (a host hangs stateWords on the game beside stepSeats)');
  return foldWord(from, W.die, W.parts);
}
/** THE WHOLE GAME'S WORDS as they stand, with the parts' names: for whoever asks WHICH part two machines disagree on (tools/mp/words-diff.ts, a check).
 *  They are NOT the alarm: a page and a game server do not agree on every part yet (docs/agents/net.md: the picture's own carried in the game's
 *  record, rows numbered differently). The parts every kind of host does agree on are folded into the wider word, watched: netWatch below. After them
 *  the ALARM'S OWN, as `block.<part>` (the block is not in the whole game's words): when the alarm fires, these files say which of its parts it was. */
export function netWhole(g: Game): { names: string[]; words: number[] } | null {
  const S = g as unknown as Stepper, A = S.wholeWords?.(), B = S.stateWords?.(); if (!A) return null;
  return { names: [...S.wholeNames?.() ?? [], 'block.die', ...SIM_PARTS.map(p => `block.${p}`)], words: [...Array.from(A), B?.die ?? 0, ...SIM_PARTS.map((_, i) => B?.parts[i] ?? 0)] };
}
/** THE WIDER WORD, WATCHED: the whole game's words of the parts on the one list of parts every kind of host has agreed on (net/agreed.ts), folded into
 *  one word, by NAME (a part the module has not got is passed over; two machines of one build pass over the same). Not chained: each word tick's stands
 *  alone, so a tick that differs is one tick counted. The server sends it beside the state word; a client holds its own against it and COUNTS
 *  (client.ts watchOk / watchOff): it is no alarm until real pages on real GPUs have shown the list holds. null where the host has no whole words */
let WATCH: { names: readonly string[]; ix: number[] } | null = null;
export function netWatch(g: Game): string | null {
  const S = g as unknown as Stepper, A = S.wholeWords?.(); if (!A) return null;
  const N = S.wholeNames?.() ?? []; if (!WATCH || WATCH.names !== N) WATCH = { names: N, ix: agreedOf(N) };
  return foldWhole(A, WATCH.ix);
}
/** a quiet run of networked ticks begins / ends (a server between two state words, a client's walk of the ticks its server has confirmed) */
export function netQuiet(g: Game, on: boolean): void { (g as unknown as Stepper).quietRun?.(on); }
/** A NETWORKED GAME IS STOOD THE SAME WAY ON EVERY MACHINE: once, when the game is stood, by the server, a scripted client and a page alike.
 *  Every seat is refereed (wire.ts: the view is worked out again from the pose). Every dressing row is up (game/dress.ts): no recipe carries that
 *  table, a page has it up and a host that draws nothing may have put it down, and in the game that steps the rows are not picture alone (a debris
 *  chunk and a glass plate are rows of the game's box table; a storm's wind source). So the stand does not leave it to how a host happened to be stood */
export function netStand(g: Game): void { for (const s of g.seats) s.remote = true; dressAll(true); }
/** where a seat's man would be after records the game has not had, off a host's own copy of the game (foresee.ts) */
export function netAhead(g: Game, seat: Seat, cmds: readonly WireCmd[]): Ahead {
  const v = seat.view, pl = v.player, ch = pl.char, path: [number, number, number][] = [], three = (p: ArrayLike<number>): [number, number, number] => [p[0], p[1], p[2]];
  let near = Infinity; for (const c of [...g.guards.map(x => x.char), ...g.civilians.map(x => x.char), ...g.seats.map(x => x.view.player.char)]) if (c !== ch) near = Math.min(near, Math.hypot(c.pos[0] - ch.pos[0], c.pos[2] - ch.pos[2]));
  // (no record: where the copy has him, and the game that steps is not asked)
  const now = cmds.length ? walkAhead(seat, cmds.map(fromWire), 1 / NET.hz, path as never) : { free: legsOwn(v), pos: ch.pos, aimYaw: ch.aimYaw, follow: v.followPos(), aim: v.aimPoint };
  return { id: ch.id, down: pl.down, at: three(ch.pos), yaw: ch.aimYaw, wall: pl.wall ? [pl.wall.n[0], pl.wall.n[2]] : null, near, free: now.free, pos: three(now.pos), aimYaw: now.aimYaw, follow: three(now.follow), aim: now.aim ? three(now.aim) : null, path };
}
