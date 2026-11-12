// A NETWORKED GAME IN THE MODULE (rust/game_face net.rs): stepped on its seats' records, asked for its words, its end and where a seat's man would be. The same on
// a game server, a scripted client and a page; whoever starts the game (start.ts) and holds its number hands both in.
import type { Face, GameNo } from '../draw/face';
import { seatInput, type Doubles } from '../draw/input';
import type { AheadView } from '../view/read';
import type { WireCmd, Outcome } from './wire';
import { NET } from './wire';
import { NET_WORD_SEED, foldWord, wordTick } from './words';

/** a key's code, as the referee has one: anything else is no key, and is left out before the codes become one text */
const CODE = /^[A-Za-z0-9]+$/;
const codes = (k: readonly string[] | undefined): string => k ? k.filter(c => CODE.test(c)).join(' ') : '';
/** a record's keys: those held (and those held under a press it carries), a line feed, those that went down · its numbers, as a row */
const keysOf = (w: WireCmd): string => `${codes(w.u ? [...w.k, ...w.u] : w.k)}\n${codes(w.p)}`;
const rowOf = (w: WireCmd): Doubles<'SeatInput'> => { const r = seatInput(); r.set(w.n); return r; };

export class NetGame {
  /** ticks stepped */
  ticks = 0;
  private w = NET_WORD_SEED; private names: string[] | null = null;

  constructor(private readonly face: Face, readonly h: GameNo) { face.networked(h); }

  /** one tick on the seats' records, its word folded after it. What the tick told its host stays to be read (a page), until `clear` */
  step(cmds: readonly WireCmd[]): void {
    const { face, h } = this;
    cmds.forEach((c, seat) => face.record(h, seat, rowOf(c), keysOf(c)));
    face.step(h, 1 / NET.hz, 0);   // (the saves are a game of one's)
    this.w = foldWord(this.w, [face.tickWord(h)]);
    if (wordTick(this.ticks++)) this.w = foldWord(this.w, this.whole().words);
  }
  /** what the ticks told their host is let go */
  clear(): void { this.face.outClear(this.h); }
  /** the state word after the last tick (words.ts) */
  word(): string { return this.w.toString(36); }
  /** the whole game's words, one a part; `named`: with the parts' names, and after them what is in the word of every tick, as `tick.<what>` */
  whole(named = false): { names: string[]; words: number[] } {
    const { face, h } = this, words = Array.from(face.stateWords(h));
    if (!named) return { names: [], words };
    this.names ??= [...face.stateNames(), 'tick.word'];
    return { names: this.names, words: [...words, face.tickWord(h)] };
  }
  /** how the mission ended (null while it is being played) */
  ended(): Outcome | null { const r = this.face.verdict(this.h); return r && { won: r.won, end: r.word, why: r.why }; }

  /** where the man of a seat would be after records the game has not had; with none, where the game has him */
  ahead(seat: number, cmds: readonly WireCmd[]): AheadView { return this.face.ahead(this.h, seat, cmds.map(rowOf), cmds.map(keysOf)); }
}
