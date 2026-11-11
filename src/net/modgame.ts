// A NETWORKED GAME IN THE MODULE (rust/game_face net.rs): stepped on its seats' records, asked for its words, its end and where a seat's man would be. The same on
// a game server, a scripted client and a page; whoever starts the game (start.ts) and holds its number hands both in.
import type { Face, GameNo } from '../draw/face';
import type { AheadView } from '../view/read';
import type { WireCmd, Outcome } from './wire';
import { NET } from './wire';
import { NET_WORD_SEED, foldWord, wordTick } from './words';

/** a record's numbers, as the module is handed them (rust/game_face net.rs RECORD) */
const RECORD = 18;

/** a key's code, as the referee has one: anything else is no key, and is left out before the codes become one text */
const CODE = /^[A-Za-z0-9]+$/;
const codes = (k: readonly string[] | undefined): string => k ? k.filter(c => CODE.test(c)).join(' ') : '';
/** a record's numbers into `N` at `at`; answers its keys' text: those held, a line feed, those that went down */
export function recordTo(N: Float64Array, at: number, w: WireCmd): string {
  N[at] = w.b; N[at + 1] = w.c ?? 0; N[at + 2] = w.w ?? 0; N[at + 3] = w.m[0]; N[at + 4] = w.m[1]; N[at + 5] = w.s[0]; N[at + 6] = w.s[1]; N[at + 7] = w.s[2]; N[at + 8] = w.s[3];
  for (let i = 0; i < 9; i++) N[at + 9 + i] = w.v[i];
  return `${codes(w.k)}\n${codes(w.p)}`;
}

export class NetGame {
  /** ticks stepped */
  ticks = 0;
  private w = NET_WORD_SEED; private names: string[] | null = null;

  constructor(private readonly face: Face, readonly h: GameNo) { face.networked(h); }

  /** records' numbers, one after another, and their keys' text */
  private handed(cmds: readonly WireCmd[]): [numbers: Float64Array, keys: string] { const N = new Float64Array(cmds.length * RECORD), keys = cmds.map((c, i) => recordTo(N, i * RECORD, c)).join('\n'); return [N, keys]; }

  /** one tick on the seats' records, its word folded after it. What the tick told its host stays to be read (a page), until `clear` */
  step(cmds: readonly WireCmd[]): void {
    const { face, h } = this;
    cmds.forEach((c, seat) => face.record(h, seat, ...this.handed([c])));
    face.step(h, 1 / NET.hz, 0);   // (the saves are a game of one's)
    this.w = foldWord(foldWord(this.w, [face.dieN(h)]), face.partHashes(h));
    if (wordTick(this.ticks++)) this.w = foldWord(this.w, this.whole().words);
  }
  /** what the ticks told their host is let go */
  clear(): void { this.face.outClear(this.h); }
  /** the state word after the last tick (words.ts) */
  word(): string { return this.w.toString(36); }
  /** the whole game's words, one a part; `named`: with the parts' names, and after them what is in the word of every tick, as `block.<part>` */
  whole(named = false): { names: string[]; words: number[] } {
    const { face, h } = this, words = Array.from(face.stateWords(h));
    if (!named) return { names: [], words };
    this.names ??= [...face.stateNames(), 'block.die', ...face.partNames().map(n => `block.${n}`)];
    return { names: this.names, words: [...words, face.dieN(h), ...face.partHashes(h)] };
  }
  /** how the mission ended (null while it is being played) */
  ended(): Outcome | null { const r = this.face.verdict(this.h); return r && { won: r.won, end: r.word, why: r.why }; }

  /** where the man of a seat would be after records the game has not had; with none, where the game has him */
  ahead(seat: number, cmds: readonly WireCmd[]): AheadView { return this.face.ahead(this.h, seat, ...this.handed(cmds)); }
}
