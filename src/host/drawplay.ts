// A TAPE WALKED (replay/handtape.ts): the game it is of, started as it was and handed what it was handed. Who steps it asks for the next tick or for another place on it; the picture is whoever draws it.
import { HAND } from '../draw/hand';
import { ANOTHER, HandWalk, TapeRefused, type TapeMark, type TapeRow } from '../replay/handtape';
import { GameLost, partsOf, type DrawGame, type Start } from './drawgame';

/** moments held for a seek back: one this many ticks apart, so many of them, each a copy of the whole game */
const MOMENT = { every: 600, most: 14 };

type Step = Extract<TapeRow, { kind: 'step' }>;
export class DrawPlay {
  readonly walk: HandWalk;
  /** ticks on the tape, and why it stops there ('': it was taken off a run that went on) */
  readonly total: number; readonly ends: string;
  /** the tick the game stands after */
  get at(): number { return this.game.ticks; }
  /** where what the game came to is first not what the tape says it came to: from there it is another run */
  parted: { tick: number; said: TapeMark; mine: TapeMark } | null = null;
  /** the walk was lost to a panic, as the run may have been (`ends`) */
  lost: GameLost | null = null;
  private rows: HandWalk; private moments = new Map<number, number>();

  constructor(private game: DrawGame, private bytes: Uint8Array) {
    this.walk = new HandWalk(bytes, HAND.count); this.rows = new HandWalk(bytes, HAND.count);
    let total = 0, ends = ''; for (let r = this.rows.next(); r; r = this.rows.next()) { if (r.kind === 'step') total++; else if (r.kind === 'end') ends = r.why; }
    this.total = total; this.ends = ends; this.rows = new HandWalk(bytes, HAND.count);
  }
  /** the game from the tape's start, on this build's own parts; a part that is another build's is refused by its name */
  start(own: Pick<Start, 'mannequin' | 'mannequinBin' | 'cast' | 'castLook'>): void {
    const S = this.walk.start, o: Start = { ...own, level: S.level, words: S.words, seed: S.seed, seats: S.seats }, other = this.walk.other(partsOf(o));
    if (other) throw new TapeRefused(ANOTHER);
    this.game.commands.take(); this.game.commands.put(S.commands); this.game.start(o); this.game.walked = true; this.hold();
  }
  /** what the hand of the next tick will be, read ahead: where the view goes from here (null at the tape's end) */
  get next(): Float64Array | null { return this.lost ? null : this.ahead()?.hand ?? null; }
  private read: { row: Step; hand: Float64Array } | null = null; private due: TapeRow[] = [];
  private ahead(): { row: Step; hand: Float64Array } | null {
    while (!this.read) { const r = this.rows.next(); if (!r) return null; if (r.kind === 'step') this.read = { row: r, hand: r.hand }; else this.due.push(r); }
    return this.read;
  }
  /** ONE TICK of it (`seen` false: one nobody is shown). False: the tape is at its end */
  tick(seen = true): boolean {
    if (this.lost) return false;
    const a = this.ahead(); if (!a) { this.rest(seen); return false; }
    this.rest(seen); this.read = null;
    // (a run lost to a panic is lost again here, and its tape ends with it: what stands before it can still be gone back to)
    try { this.game.feed(a.row, seen); } catch (e) { if (!(e instanceof GameLost)) throw e; this.lost = e; return false; }
    if (this.at % MOMENT.every === 0) this.hold();
    return true;
  }
  /** what is due before the next tick (the marks are of the ticks before it) */
  private rest(seen: boolean): void { for (const r of this.due) { if (r.kind === 'mark') this.held(r.mark); else if (r.kind !== 'end') this.game.feed(r, seen); } this.due.length = 0; }
  private held(said: TapeMark): void {
    if (this.parted || said.tick !== this.at) return;
    const mine = this.game.mark(said.state !== null); if (mine.die !== said.die || mine.parts !== said.parts || (said.state !== null && mine.state !== null && mine.state !== said.state)) this.parted = { tick: said.tick, said, mine };
  }
  /** the moment the game stands at is held for a seek back, the crowded ones let go first */
  private hold(): void {
    if (this.moments.has(this.at)) return;
    const k = this.game.moment(); if (!k) return;
    this.moments.set(this.at, k);
    while (this.moments.size > MOMENT.most) { const at = [...this.moments.keys()].sort((a, b) => a - b); let worst = 1, least = Infinity; for (let i = 1; i < at.length - 1; i++) { const gap = at[i + 1] - at[i - 1]; if (gap < least) { least = gap; worst = i; } } this.game.momentDrop(this.moments.get(at[worst])!); this.moments.delete(at[worst]); }
  }
  /** to tick `to`, unseen: back to the moment before it where that is behind, then on. Goes on for at most `ms`; true = it is there (or lost on the way), and the picture is of it */
  seek(to: number, ms = Infinity): boolean {
    const t0 = performance.now(); to = Math.max(0, Math.min(this.total, to));
    if (to < this.at) {
      const from = Math.max(...[...this.moments.keys()].filter(t => t <= to)); if (!this.game.momentBack(this.moments.get(from)!, from)) throw new Error(`the moment at tick ${from} is gone`);
      this.lost = null; this.rows = new HandWalk(this.bytes, HAND.count); this.read = null; this.due.length = 0; while (this.rows.ticks < from) this.rows.next();
      if (this.parted && this.parted.tick > from) this.parted = null;
    }
    while (this.at < to) { if (!this.tick(false)) break; if (performance.now() - t0 > ms) return this.at >= to; }
    if (!this.game.lost) this.game.show();
    return true;
  }
}
