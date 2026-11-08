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
  /** how hard the defenders are, every lever back on it (before a game is made: what it is staged by) */
  difficulty(hard: number): void { this.u32(12); this.u32(hard); }
  /** one lever over that, by its key and its rung (an empty rung: back to the level's own) */
  lever(key: string, rung: string): void { this.u32(13); this.text(key); this.text(rung); }
  /** one of the two tables under the difficulty, by itself */
  pressure(on: number): void { this.u32(14); this.u32(on); }
  /**  */
  digIn(on: number): void { this.u32(15); this.u32(on); }
  /** the fog of war's cone, whatever the level says, for a visit */
  fogCone(on: number): void { this.u32(16); this.u32(on); }
  /** one knob, by its name (knobs.rs); a name no knob has is nothing */
  knob(name: string, value: number): void { this.u32(17); this.text(name); this.f64(value); }
}
