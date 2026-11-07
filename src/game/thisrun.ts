// IS THIS RECORD OF THE GAME'S CURRENT MISSION. A module that keeps something for one run (the defuse's pass, the par clock, a lesson's step, the radio's
// beats) remembers the run it was for and asks, before it reads what it kept, whether that is still the mission that is up.
//
// A TypeScript game answers by the mission RECORD: a restart writes a new one, and the module holds the old. A puppet of the Rust game cannot always:
// the Rust game names a mission by its SERIAL, and a record's `run` that the Rust game wrote reaches the puppet as that number unless the bridge was told
// to hand it over as the record (schema.json `serial`: a lesson's and the radio's are, the defuse's is not). Asked by the record alone, a puppet's defuse was
// never "this mission's": the verdict skipped its BLOWN and DEFUSED ends, and a mission lost to the clock ended on the room clear's win.
//
// So the rule is said once, here, and it is the Rust game's: THE SAME MISSION IS THE SAME SERIAL. A host of the Rust game hangs `runSerial` on its puppet:
// the serial the record that is up has in the Rust game (the bridge's book knows it from the start; undefined where the host made the record anew
// and was not told the number, and then a number the Rust game wrote stands as its own word).
//
// This file goes when the verdict and the par clock are read off the Rust game's own records and nothing on a puppet asks the question any more.
import type { Game } from './game';

export function ofThisMission(g: Game, run: unknown): boolean {
  if (run === g.mission) return true;          // (a record this TypeScript wrote, or one the bridge handed over as the record)
  if (typeof run !== 'number') return false;   // (nothing yet, or another run's record)
  const s = (g as unknown as { runSerial?: () => number | undefined }).runSerial?.();
  return s === undefined || s === run;
}
