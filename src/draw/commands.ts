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
  /** a save goes back, as the next step ends, by its key (one that will not is said then) */
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
  /** the cards as the player's machine keeps them (before a game is made: what its men are staged in) */
  kit(kept: string): void { this.u32(18); this.text(kept); }
  /** the rung a quiet mission is played under, as this machine keeps it (a word that is no rung is nothing) */
  quietFog(rung: string): void { this.u32(19); this.text(rung); }
  /** the cameras a quiet mission hands out, as this machine keeps them */
  quietCams(on: number): void { this.u32(20); this.u32(on); }
  /** encounter back */
  aside(on: number, x: number, z: number): void { this.u32(21); this.u32(on); this.f64(x); this.f64(z); }
  /** what each man is thinking is said over him (and the meters' raw numbers with it) */
  labels(on: number): void { this.u32(22); this.u32(on); }
  /** the hour over the level, for a visit (a word that is no hour: the level as it was built) */
  hour(id: string): void { this.u32(23); this.text(id); }
  /** how much of the sky the picked weather covers, and the share of the hour's own sun that comes through whatever the cover */
  cover(share: number): void { this.u32(24); this.f64(share); }
  /**  */
  sun(share: number): void { this.u32(25); this.f64(share); }
  /** the level's moon: its strength, its height and bearing in degrees (a number that is none leaves it) */
  moon(k: number, elev: number, bearing: number): void { this.u32(26); this.f64(k); this.f64(elev); this.f64(bearing); }
  /** the kit picture: the team's cards (as Kit's text) and how many come (under 0: as it stands), staged again */
  kitStageCards(kept: string, size: number): void { this.u32(30); this.text(kept); this.i32(size); }
  /** whose card is open (0 the player) and the row under its highlight */
  kitStageOpen(who: number, field: string): void { this.u32(31); this.u32(who); this.text(field); }
  /** foot (a number that is none: not said) */
  kitStageFrame(aspect: number, cx: number, cy: number, h: number, w: number, man: number, door: number, left: number, right: number, keep: number, top: number, bottom: number): void { this.u32(32); this.f64(aspect); this.f64(cx); this.f64(cy); this.f64(h); this.f64(w); this.f64(man); this.f64(door); this.f64(left); this.f64(right); this.f64(keep); this.f64(top); this.f64(bottom); }
  /** the kit picture: the page has laid its copy over the picture, as the view's `veil` asked */
  kitStageUnder(): void { this.u32(33); }
  /** the kit picture: the page has shown its first picture: the acts may begin */
  kitStageShown(): void { this.u32(34); }
  /** one of the panel's actions, by its name (actions.rs), with what its row picked */
  act(name: string, value: number, word: string): void { this.u32(35); this.text(name); this.f64(value); this.text(word); }
  /** an address's words, on what stands no game yet: the desk, whose kit rules read the numbers they turn */
  words(text: string): void { this.u32(36); this.text(text); }
}
