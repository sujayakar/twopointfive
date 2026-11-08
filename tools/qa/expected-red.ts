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
  'pair-mark': { is: 'maybe', whose: RUST, line: "the only way to the marked man's back lies across his eyes: the partner should wait short of it and give up after a few seconds; he moves up at once, 36 s round, 'set', and is seen a second later (way.exposed comes out false; perhaps the eye change moved the walk's cost)" },
  'wallbreak-checkpoint': { is: 'maybe', whose: RUST, line: "two seconds after a load the list of marks holds the undone future's too: 114 against 86. A whole apply does not rebuild the picture's decals" },
  'glasshouse': { is: 'maybe', whose: "the senses' or the level's owner", line: "both starts under 0.1 of light: 0.27 and 0.27 to 0.28 by the TypeScript estimate; wants one read of the game's own meter" },
  'pass': { is: 'maybe', whose: "the senses' or the level's owner", line: "both starts under 0.1 of light: 0.21 and 0.14 to 0.21 by the TypeScript estimate; wants one read of the game's own meter" },
  'floodlit': { is: 'maybe', whose: "the senses' or the level's owner", line: "mains out: the yard 0.13 to 0.28 where under 0.2 is asked, the hall 0.27, the radio room's tube 0.94 against the moon's; wants one read of the game's own meter" },
  'depot-stage': { is: 'maybe', whose: "the senses' or the level's owner", line: "mains out: the open park 0.28 where under 0.25 is asked; wants one read of the game's own meter" },
  'checkpoint-identity': { is: 'maybe', whose: RUST, line: "save(load(s)) is the identity: a mark's radius 0.167 comes back 0.166, the digests equal" },
  'checkpoint-turn': { is: 'maybe', whose: RUST, line: "a file loaded into a fresh game: one calm guard's torch comes on for about three seconds (off in the game that went on); places and yaws agree frame for frame. No torch or its clock is in the file but torchLostAt" },
  'furnblast-levels': { is: 'maybe', whose: RUST, line: "block.json: after a wall charge at each of eight pieces of furniture and a restart, four slats of one bench (boxes 344 to 347, the shop wall's) stay gone; every other box, prop and the nav as they were. Four other levels restore" },
};
