// A TAPE WALKED (tape/handtape.ts): the game it is of, started as it was and handed what it was handed. Who steps it asks for the next tick or for another place on it; the picture is whoever draws it.
import type { MomentNo } from '../draw/face';
import { ANOTHER, HandWalk, TapeRefused, figuresPart, type TapeFigures, type TapeMark, type TapePlace, type TapeRow } from '../tape/handtape';
import { GameLost, partsOf, type DrawGame, type Start } from './drawgame';

/** moments held for a seek back: one this many ticks apart, so many of them, each a copy of the whole game */
const MOMENT = { every: 600, most: 14 };

/** where a walk is no longer the run, and in what */
export interface Parted { tick: number; why: string }

type Step = Extract<TapeRow, { kind: 'step' }>;
export class DrawPlay {
  readonly walk: HandWalk;
  /** ticks on the tape, and why it stops there ('': it was taken off a run that went on) */
  readonly total: number; readonly ends: string;
  /** the ticks the mission began again after */
  readonly tries: number[] = [];
  /** the tick the game stands after */
  get at(): number { return this.game.ticks; }
  /** where what the game came to is first not what the tape says it came to: from there it is another run */
  get parted(): Parted | null { return this.same && this.words && (!this.play || this.words.tick <= this.play.tick) ? this.words : this.play; }
  /** where its die, or who stands where, is first not the tape's: another play of it, on any module */
  play: Parted | null = null;
  /** where a hash of its state is first not the tape's. Of the module that played it, that is another run; another module may keep the same play in other words */
  words: Parted | null = null;
  /** the walk was lost to a panic, as the run may have been (`ends`) */
  lost: GameLost | null = null;
  private rows: HandWalk; private moments = new Map<number, { k: MomentNo; place: TapePlace }>();

  /** `same`: it was played on this very module */
  constructor(readonly game: DrawGame, bytes: Uint8Array, private same = true) {
    this.walk = new HandWalk(bytes); this.rows = new HandWalk(bytes);
    let total = 0, ends = ''; for (let r = this.rows.next(); r; r = this.rows.next()) { if (r.kind === 'step') total++; else if (r.kind === 'try') this.tries.push(r.tick); else if (r.kind === 'end') ends = r.why; }
    this.total = total; this.ends = ends; this.rows = new HandWalk(bytes);
  }
  /** the game from the tape's start, on this build's own parts; a part that is another build's is refused by its name */
  start(own: Pick<Start, 'mannequin' | 'mannequinBin' | 'cast' | 'castLook'>): void {
    const S = this.walk.start, o: Start = { ...own, level: S.level, words: S.words, seed: S.seed, seats: S.seats }, other = this.walk.other(partsOf(o));
    if (other) throw new TapeRefused(ANOTHER);
    this.game.commands.take(); this.game.start({ ...o, kept: S.kept }); this.game.walked = true; this.hold();
  }
  /** what the hand of the next tick will be, read ahead: where the view goes from here (at the tape's end, its last) */
  get next(): Float64Array { return (this.lost ? null : this.ahead()?.hand) ?? this.rows.last; }
  private read: { row: Step; hand: Float64Array } | null = null; private due: TapeRow[] = [];
  private ahead(): { row: Step; hand: Float64Array } | null {
    while (!this.read) { const r = this.rows.next(); if (!r) return null; if (r.kind === 'step') this.read = { row: r, hand: r.hand }; else this.due.push(r); }
    return this.read;
  }
  /** ONE TICK of it (`seen` false: one nobody is shown). False: the tape is at its end */
  tick(seen = true): boolean {
    if (this.lost) return false;
    const a = this.ahead();
    // (a run lost to a panic is lost again here, and its tape ends with it: what stands before it can still be gone back to)
    try { this.rest(seen); if (!a) return false; this.read = null; this.game.feed(a.row, seen); } catch (e) { if (!(e instanceof GameLost)) throw e; this.lost = e; return false; }
    if (this.at % MOMENT.every === 0) this.hold();
    return true;
  }
  /** what is due before the next tick (the marks are of the ticks before it) */
  private rest(seen: boolean): void { for (const r of this.due) { if (r.kind === 'mark') this.held(r.mark); else if (r.kind === 'figures') this.stood(r.figures); else if (r.kind !== 'end') this.game.feed(r, seen); } this.due.length = 0; }
  private held(said: TapeMark): void {
    if (this.play || said.tick !== this.at) return;
    const tick = said.tick, mine = this.game.mark(said.state !== null && !this.words);
    if (mine.die !== said.die) this.play = { tick, why: `${mine.die} rolls of the die, ${said.die} on the tape` };
    if (!this.words && (mine.stands !== said.stands || (said.state !== null && mine.state !== null && mine.state !== said.state))) this.words = { tick, why: mine.stands !== said.stands ? 'where everybody stands' : 'the hash of every part of it' };
  }
  private stood(said: TapeFigures): void { if (this.play || said.tick !== this.at) return; const why = figuresPart(said, this.game.figures()); if (why) this.play = { tick: said.tick, why }; }
  /** the moment the game stands at is held for a seek back, the crowded ones let go first */
  private hold(): void {
    if (this.moments.has(this.at)) return;
    const k = this.game.moment(); if (!k) return;
    this.moments.set(this.at, { k, place: this.rows.place() });
    while (this.moments.size > MOMENT.most) { const at = [...this.moments.keys()].sort((a, b) => a - b); let worst = 1, least = Infinity; for (let i = 1; i < at.length - 1; i++) { const gap = at[i + 1] - at[i - 1]; if (gap < least) { least = gap; worst = i; } } this.game.momentDrop(this.moments.get(at[worst])!.k); this.moments.delete(at[worst]); }
  }
  /** to tick `to`, unseen: back to the moment before it where that is behind, then on. Goes on for at most `ms`; true = it is there (or lost on the way), and the picture is of it */
  seek(to: number, ms = Infinity): boolean {
    const t0 = performance.now(); to = Math.max(0, Math.min(this.total, to));
    if (to < this.at) {
      const from = Math.max(...[...this.moments.keys()].filter(t => t <= to)), M = this.moments.get(from)!; if (!this.game.momentBack(M.k, from)) throw new Error(`the moment at tick ${from} is gone`);
      this.lost = null; this.rows.back(M.place); this.read = null; this.due.length = 0;
      if (this.play && this.play.tick > from) this.play = null; if (this.words && this.words.tick > from) this.words = null;
    }
    while (this.at < to) { if (!this.tick(false)) break; if (performance.now() - t0 > ms) return this.at >= to; }
    if (!this.game.lost) this.game.show();
    return true;
  }
}
