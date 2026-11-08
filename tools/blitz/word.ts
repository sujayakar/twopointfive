// THE ONE SWITCH: WHICH GAME A HEADLESS TOOL'S `standUp` STANDS. Everything that asks goes through here: the harness (tools/qa/headless.ts) and the bridge's
// preload (tools/blitz/preload.ts: whether a process needs the bridge at all). It imports nothing: it is read before any source file loads, and under node.
//
//   TPS_GAME=rust (unset: the same)   a game stood is the Rust game: the tool holds a copy of it
//   TPS_GAME=audit                    the same, audited
//   TPS_GAME=none                     nothing of the Rust game is loaded: what is stood is read and never stepped

export const DEFAULT_WORD: string = 'rust';

export type Word = '' | 'rust' | 'audit';
/** the word asked for ('' = none) */
export function asked(): Word {
  const w = process.env.TPS_GAME ?? DEFAULT_WORD; if (w === 'none' || w === 'off') return '';
  if (w !== 'rust' && w !== 'audit') throw new Error(`TPS_GAME=${w}: rust (the Rust game steps), audit, or none (read, never stepped)`);
  return w;
}
/** which game is stood now. '' where the run is on the old frozen stand (TPS_PROFILE=frozen: its rows are constants of the Rust game) and under an engine that is not bun (it has no bridge preload). */
export function resolve(frozen = false): { word: Word } { return { word: frozen || !(globalThis as { Bun?: unknown }).Bun ? '' : asked() }; }
