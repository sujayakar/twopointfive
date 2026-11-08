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
  'wand-kit': { is: 'maybe', leans: 'stale', rust: true, line: "read 2026-11-07 · 2 of 7 lines fail, nothing refused · first: the row is on YOUR card, the last of your kit rows (after the 40 mm row, and after your charge rows where your card has them: owncharge.ts) · SORTED BY HAND: candidate to delete: the card's row order is from before the kit tree (the night-vision row sits after the wand's now); the store line reads a stand-in store the game does not write", whose: CHECK },
  'controls-behind': { is: 'maybe', rust: true, line: "read 2026-11-07 · 1 of 2 lines fail, nothing refused · first: (f) a canister up, LMB behind him: a throw, not the takedown (the row says holster first)", whose: RUST },
  'squaddoors': { is: 'maybe', rust: true, line: "read 2026-11-07 · 1 of 6 lines fail, nothing refused · first: a teammate within reach with a wall stood between him and the leaf (this door has no place within reach behind a wall of the level's) does not see it move", whose: RUST },
  'spotted-caught': { is: 'real', rust: true, line: "read 2026-11-07 · 1 of 5 lines fails: the squad line's word shows it while it lasts · the Rust posture word has no 'SILENT · contact' (spotted.ts wrapped P.word; not carried over)", whose: RUST },
  'spotted-suspicious': { is: 'real', rust: true, line: "read 2026-11-07 · 1 of 6 lines fails: he says it once, over the net, and the squad line's word shows it · the Rust posture word has no 'SILENT · low' (spotted.ts wrapped P.word; not carried over)", whose: RUST },
  'needs-interrupts': { is: 'real', rust: true, line: "read 2026-11-07 · throws: undefined is not an object (evaluating 'o.ring.add') · in the tools' host (tools/blitz/import.ts:61, schema OtherCast.ring), at a minute with the QRF requested · 0 of 3 lines failed before it", whose: RUST },
  'qrfsquad': { is: 'real', rust: true, line: "read 2026-11-07 · throws: undefined is not an object (evaluating 'o.ring.add') · in the tools' host (tools/blitz/import.ts:61, schema OtherCast.ring), at the element's first door", whose: RUST },
};
