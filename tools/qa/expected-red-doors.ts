// THE EXPECTED REDS OF THE DOOR AND ROOM CHECK FAMILIES ON THE RUST GAME: a table of tools/qa/expected-red.ts (its header says what a row is and is not).
// Rows are printed by tools/qa/red-rows.ts from a run's output, for probes whose failing lines are all the page's own side (nothing refused by the fold, no line
// made with the other side of a settled switch), and read before they are kept. A red probe of these families with no row here has its row in the older table,
// or is new. A 'maybe' row is not sorted between the game and the check: several of these checks read lines through hooks they wrap on the tools' copy of the game.
import type { RedRow } from './expected-red.ts';
/** the Rust game's owners: a fault of the game to trace, or a call the check makes that the game no longer answers */
const RUST = "the Rust game's";
/** the switch cleanup's: the check asks through something the Rust game does not have (a row made by hand on the tools' copy of the game, a hook wrapped on it), and is to be restated */
const CHECK = "the switch cleanup's (tools)";
export const RED_DOORS: Record<string, RedRow> = {
  'breaks-furniture': { is: 'maybe', rust: true, line: "read 2026-11-07 · throws: null is not an object (evaluating 'c.mat')", whose: RUST },
  'breaks-wall': { is: 'maybe', rust: true, line: "read 2026-11-07 · 6 of 13 lines fail, nothing refused · first: pool 64: thrown wall pieces and the dealt heap inside the blast's share", whose: RUST },
  'off-hinges-after': { is: 'stale', rust: true, line: "read 2026-11-07 · throws: rust-host: this game gave its slot up to another and its kept bytes have been let go since (the last 6 are kept): its re-step is refused. Asked by tools/qa/door · 1 of 2 lines failed before it · first: the probe threw · THE CHECK'S: the check steps a game it stood earlier after standing more than six others: the tools' host keeps the bytes of the last six only. Cure: stand the game again where it is stepped", whose: CHECK },
  'pair-mark': { is: 'maybe', rust: true, line: "read 2026-11-07 · throws: P.input is not a function. (In 'P.input(R.input)', 'P.input' is undefined) · 4 of 4 lines failed before it · first: given (B on the man): the partner has the job and answers", whose: RUST },
  'pairup-clatter': { is: 'maybe', rust: true, line: "read 2026-11-07 · 1 of 3 lines fail, nothing refused · first: the partner's takedown on a take order: no 'prop' event from his hands going on to three seconds after the man is down", whose: RUST },
};
