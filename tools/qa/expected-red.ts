// THE EXPECTED REDS: probes left red for a reason somebody has read, by PROBE name (a family's word is not a row). probes.ts prints its reds against this list
// after the verdict (EXPECTED RED with the row / NEW RED / GREEN AGAIN) and gate.sh carries those lines. The list changes NO verdict: a listed probe still
// fails its run. A row is a fault of the game, placed or waiting for its owner's read; a check's own red is mended, cut or deleted, never listed. A row goes
// the day its check is green. (Until 2026-11-08 this was five tables of 475 rows, most of them the checks' own.)
export interface RedRow {
  /** 'real' = a fault of the game, traced in the row · 'maybe' = the check reached the game and its owner's read is owed */
  is: 'real' | 'maybe';
  /** what the failing line says, and what was read off it */
  line: string;
  /** whose it is to turn green */
  whose: string;
}
/** A row as one line for the runner's table (the gate cuts a line short: the short fields come first). */
export function rowSays(r: RedRow): string { return `${r.is === 'real' ? 'REAL' : 'COULD BE REAL'} · ${r.whose} · ${r.line}`; }
const RUST = "the Rust game's";
export const EXPECTED_RED: Record<string, RedRow> = {
  'aim-door': { is: 'real', whose: "the enemies' owner (the defenders at a door: docs/agents/enemies.md)", line: "[8], the break room's corridor door shut, Sam dark and planted right behind the leaf: the guard opens it at 3 s and the leaf stops on Sam's body; from 5 s he stands in the frame 0.9 m from Sam, alert and not moving, and fires nothing until 11 to 12 s" },
  'pair-mark': { is: 'maybe', whose: RUST, line: "the only way to the marked man's back lies across his eyes: the partner should wait short of it and give up after a few seconds; he moves up at once, 36 s round, 'set', and is seen a second later (way.exposed comes out false; perhaps the eye change moved the walk's cost)" },
  'pairup-clatter': { is: 'maybe', whose: RUST, line: "the partner's quiet takedown files one 'prop' event, level 0.27, 1.1 s after the man is down; none was filed before. Perhaps the body's own thud" },
  'wallbreak-checkpoint': { is: 'maybe', whose: RUST, line: "two seconds after a load the list of marks holds the undone future's too: 114 against 86. A whole apply does not rebuild the picture's decals" },
};
