// Written by tools/draw/gen.ts from rust/game_run/src/command.rs. To change a command, change it there.
import { Writer } from './writer';

/** what a player or a panel does that is not input, in its order, until a step takes it */
export class Commands extends Writer {
  /** the player cannot be hurt */
  god(on: number): void { this.u32(0); this.u32(on); }
  /** the others think */
  ai(on: number): void { this.u32(1); this.u32(on); }
  /** rounds are not counted */
  ammo(on: number): void { this.u32(2); this.u32(on); }
  /** nobody sees the player */
  unseen(on: number): void { this.u32(3); this.u32(on); }
  /** the fog of war */
  fog(on: number): void { this.u32(4); this.u32(on); }
  /** the view is this man's, by his place in the list; under 0 the player's own */
  spectate(who: number): void { this.u32(5); this.i32(who); }
  /** the mission from its start */
  restart(said: string): void { this.u32(6); this.text(said); }
  /** a save by hand, as the next step ends */
  save(): void { this.u32(7); }
  /** a save goes back, as the next step ends, by its key */
  load(key: string): void { this.u32(8); this.text(key); }
  /** the run saves itself */
  autosave(on: number): void { this.u32(9); this.u32(on); }
  /** a card came up over the game: what the hands were lining up is let go unsent */
  letGo(): void { this.u32(10); }
  /** whatever is up is backed out of */
  escape(): void { this.u32(11); }
}
