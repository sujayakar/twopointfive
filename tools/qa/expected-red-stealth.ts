// THE EXPECTED REDS OF THE ENEMY AND STEALTH CHECK FAMILIES ON THE RUST GAME: a table of tools/qa/expected-red.ts (its header says what a row is and is not).
// Rows are printed by tools/qa/red-rows.ts from a run's output, for probes whose failing lines are all the page's own side (nothing refused by the fold, no line
// made with the other side of a settled switch), and read before they are kept. A red probe of these families with no row here is either new, or its check still
// writes a settled row and is to be cleared first (`close-duo` stands on tools/qa/pairup-probes.ts, which wrote one when these were read; `by-eye`, on the same rig, was read with nothing refused on the game of step 1908 and has its row).
// Read on the named head of 13:25Z (5dbe0d75, the module made again), every family of the group; the fog probes and `replayfog` read again on 0bdbdc95 (its child run ends there). SORTED BY HAND where a row says so: a line that reads what was said through a wrap on the tools' copy of the
// game, or a field off the check's own copy of a man, is the check's, not the game's; a "candidate to delete" asks a build from before a settled rule that
// the check held off until 2026-11-07 and whose own words do not name it. `teams-fight` and `snap-page` keep their rows in the older table.
import type { RedRow } from './expected-red.ts';
/** the Rust game's owners: a fault of the game to trace, or a call the check makes that the game no longer answers */
const RUST = "the Rust game's";
/** the switch cleanup's: the check asks through something the Rust game does not have (a row made by hand on the tools' copy of the game, a hook wrapped on it), and is to be restated */
const CHECK = "the switch cleanup's (tools)";
export const RED_STEALTH: Record<string, RedRow> = {
  'needs-interrupts': { is: 'real', rust: true, line: "read 2026-11-07 · throws: undefined is not an object (evaluating 'o.ring.add') · in the tools' host (tools/blitz/import.ts:61, schema OtherCast.ring), at a minute with the QRF requested · 0 of 3 lines failed before it", whose: RUST },
  'qrfsquad': { is: 'real', rust: true, line: "read 2026-11-07 · throws: undefined is not an object (evaluating 'o.ring.add') · in the tools' host (tools/blitz/import.ts:61, schema OtherCast.ring), at the element's first door", whose: RUST },
};
