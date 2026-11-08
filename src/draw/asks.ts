// Written by tools/draw/gen.ts from rust/game_run/src/desk.rs. To change an ask, change it there.
import { Writer } from './writer';
import { readBriefView, readCardView, readPressed, readSheetView, readShortView, readStartView, readTeamView, type BriefView, type CardView, type Pressed, type SheetView, type ShortView, type StartView, type TeamView } from '../view/read';

/** an answer that is one word: its length, then its bytes */
const readWord = (b: Uint8Array): string => new TextDecoder().decode(b.subarray(4));

/** an answer that is one number */
const readNumber = (b: Uint8Array): number => new DataView(b.buffer, b.byteOffset, b.byteLength).getUint32(0, true);

/** what the cards before a game show and press; `answer` is the module's, to the bytes of an ask */
export class Asks extends Writer {
  constructor(private readonly answer: (asked: Uint8Array) => Uint8Array) { super(); }
  /** a press, as it went */
  protected told(p: Pressed): Pressed { return p; }
  /** a man's card, by his place (0 the player) */
  card(who: number): CardView { this.u32(0); this.u32(who); return readCardView(this.answer(this.take())); }
  /** one row of it a step on (1) or back (-1) */
  step(who: number, field: string, dir: number): Pressed { this.u32(1); this.u32(who); this.text(field); this.i32(dir); return this.told(readPressed(this.answer(this.take()))); }
  /** a gun's row at one of its picks, fitted and refused as a paste is */
  choose(who: number, field: string, value: string): Pressed { this.u32(2); this.u32(who); this.text(field); this.text(value); return this.told(readPressed(this.answer(this.take()))); }
  /** the whole card as issued */
  reset(who: number): Pressed { this.u32(3); this.u32(who); return this.told(readPressed(this.answer(this.take()))); }
  /** his card is the copied one */
  copy(who: number): Pressed { this.u32(4); this.u32(who); return this.told(readPressed(this.answer(this.take()))); }
  /** the copied card put on him */
  paste(who: number): Pressed { this.u32(5); this.u32(who); return this.told(readPressed(this.answer(this.take()))); }
  /** the mission the cards are for, the link it would be played by, the kit its level issues (the level file's own block; empty: none) and the wall charges it hands out */
  for(mission: string, link: string, issue: string, walls: number): Pressed { this.u32(6); this.text(mission); this.text(link); this.text(issue); this.u32(walls); return this.told(readPressed(this.answer(this.take()))); }
  /** no mission: your own cards stand again */
  done(): Pressed { this.u32(7); return this.told(readPressed(this.answer(this.take()))); }
  /** the team as it would go */
  team(): TeamView { this.u32(8); return readTeamView(this.answer(this.take())); }
  /** how many come */
  size(n: number): Pressed { this.u32(9); this.u32(n); return this.told(readPressed(this.answer(this.take()))); }
  /** a named kit on all five cards (going alone: on yours) */
  named(id: string): Pressed { this.u32(10); this.text(id); return this.told(readPressed(this.answer(this.take()))); }
  /** the visitor's address is this from now on: the rows of the list in the page's order, what it names */
  start(search: string): StartView { this.u32(11); this.text(search); return readStartView(this.answer(this.take())); }
  /** a mission's rows of settings, in the sheet's order */
  sheet(mission: string): SheetView { this.u32(12); this.text(mission); return readSheetView(this.answer(this.take())); }
  /** one choice of one row pressed: refused where the row is not drawn or not live */
  set(mission: string, key: string, to: string): Pressed { this.u32(13); this.text(mission); this.text(key); this.text(to); return this.told(readPressed(this.answer(this.take()))); }
  /** the id of the mission an address starts (empty: none) */
  mission(search: string): string { this.u32(14); this.text(search); return readWord(this.answer(this.take())); }
  /** the link a row of the short list loads, at its own difficulty (-1), standard (0) or hard (1); empty: no such row */
  short(id: string, search: string, hard: number): string { this.u32(15); this.text(id); this.text(search); this.i32(hard); return readWord(this.answer(this.take())); }
  /** on a page a row of the short list started: that row, the row after it and the page load that starts it */
  next(search: string): ShortView { this.u32(16); this.text(search); return readShortView(this.answer(this.take())); }
  /** the address with everything that started something taken off */
  leave(search: string): string { this.u32(17); this.text(search); return readWord(this.answer(this.take())); }
  /** what a page says of the mission its address starts while it loads (id empty: it starts none of the list) */
  brief(search: string): BriefView { this.u32(18); this.text(search); return readBriefView(this.answer(this.take())); }
  /** 1 where the link starts what a page at this address already stands, so that it needs no page load; else 0 */
  same(link: string, page: string): number { this.u32(19); this.text(link); this.text(page); return readNumber(this.answer(this.take())); }
}
