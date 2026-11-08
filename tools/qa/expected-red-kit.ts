// THE EXPECTED REDS OF THE KIT AND GUN CHECK FAMILIES ON THE RUST GAME: a table of tools/qa/expected-red.ts (its header says what a row is and is not).
// Rows are printed by tools/qa/red-rows.ts from a run's output, for probes whose failing lines are all the page's own side, and read before they are kept.
// Read 2026-11-07 on the head of branch twopointsix at c9140fe2 (the short families and the runs by name) and at 93bb8fc0, fourteen commits earlier with the same Rust module (cover, flashweight, frag, kit, kitanim, loadout, pellets, shotgun and the runs by name of kitpack, backturn, kit-play). Each probe's row is of its run by its own name where it has one, else of its family's run. NOT SORTED into
// the game's and the check's beyond the rows that say so (THE CHECK'S, THE TOOLS', ORDER OF THE RUN). A probe with a row in the older table has none here.
import type { RedRow } from './expected-red.ts';
/** the Rust game's owners: a fault of the game to trace, or a call the check makes that the game no longer answers */
const RUST = "the Rust game's";
/** the switch cleanup's: the check asks through something the Rust game does not have, or its answer hangs on the run's order, and is to be restated */
const CHECK = "the switch cleanup's (tools)";
/** the tools' call surface (tools/blitz): what a tool is answered across it is not what the page's own TypeScript answers; nothing of the game's */
const SURFACE = "the Rust game's owners (the tools' call surface)";
export const RED_KIT: Record<string, RedRow> = {
  'cover-settle': { is: 'maybe', rust: true, line: "read 2026-11-07 · 1 stack, behind the stack 0.0 s, on the door 14.0 s, no rear man found; ref 3 stacks, behind 40.8 s", whose: RUST },
  'cover-step': { is: 'maybe', rust: true, line: "read 2026-11-07 · 0 of 3 stagings dealt a man the doorway; not sure the check reaches the game (a leaf set by hand)", whose: RUST },
};
