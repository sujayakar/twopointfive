// A NETWORKED GAME AS A HEADLESS MACHINE OF IT HOLDS IT (a game server, a scripted client), however it was stood (modstand.ts, stand.ts).
import { appendFileSync } from 'node:fs';
import { resolve } from 'node:path';
import type { WireCmd, Outcome } from '../../src/net/wire.ts';
import type { Ahead } from '../../src/net/foresee.ts';

export interface Stood {
  readonly seats: number; readonly warnings: string[]; readonly ticks: number;
  /** one tick on the seats' records, the state word folded after it */
  step(cmds: WireCmd[]): void;
  /** a run of steps between which the caller reads nothing of the game begins / ends: a host that keeps a copy of the game brings it up as the run ends */
  quiet(on: boolean): void;
  /** the chained state word after the last tick (src/net/words.ts) */
  word(): string;
  /** the wider word after the last word tick: null before the first, and where the host has no whole words */
  watch(): string | null;
  /** how the mission ended, by the game's own verdict: null while it is being played */
  ended(): Outcome | null;
  /** the game that steps makes its start NOW, so that the first tick costs a tick. Called once by whoever stood the game */
  begin(): void;
  /** which seat this machine shows: nothing a tick reads */
  show(seat: number): void;
  /** where a seat's man would be after records the game has not had; with none, where the game has him */
  ahead(seat: number, cmds: WireCmd[]): Ahead;
}

/** MP_WORDS=<a file's path up to its last dot>: this process writes the whole game's words after every word tick to `<that>.<pid>.words`, a line a tick with
 *  the parts' names first, for two machines' files to be held to each other (words-diff.ts). null = nobody asked */
export function wordsKept(): ((tick: number, A: { names: string[]; words: number[] }) => void) | null {
  const file = process.env.MP_WORDS ? `${process.env.MP_WORDS}.${process.pid}.words` : ''; let named = false;
  return file ? (tick, A) => { if (!named) { named = true; appendFileSync(file, `names ${A.names.join(' ')}\n`); } appendFileSync(file, `${tick} ${A.words.map(x => x.toString(36)).join(' ')}\n`); } : null;
}
/** the build this checkout is: its commit, or '' when it has uncommitted changes (those are no commit's build) or is no checkout */
export function buildName(): string {
  const ROOT = resolve(import.meta.dir, '../..');
  try { const git = (...a: string[]) => { const r = Bun.spawnSync(['git', ...a], { cwd: ROOT }); if (!r.success) throw new Error('git'); return new TextDecoder().decode(r.stdout).trim(); };
    return git('status', '--porcelain') ? '' : git('rev-parse', '--short=12', 'HEAD'); } catch { return ''; }
}
