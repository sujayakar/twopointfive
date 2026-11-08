// Written by tools/draw/gen.ts from rust/game_run/src/desk.rs. To change an ask, change it there.
import { Writer } from './writer';
import { readCardView, readPressed, readTeamView, type CardView, type Pressed, type TeamView } from '../view/read';

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
}
