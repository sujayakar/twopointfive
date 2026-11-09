// THE SCORE, LEVEL BY LEVEL. The maintainer, 10-15: "we should curate the music for each level, not do it randomly". So every level has a short list of the score's pieces
// (pieces.ts) written down for it here — two: the one that opens it and the one that follows — and music.ts plays that list in its order instead of dealing from a
// shuffled bag: the first piece opens the level when play begins (off the menu, or on a page that loads straight in), the second takes over once a piece has had
// MUSIC.minutes at a calm phrase end, and round again. A retry keeps the music it has. The theme is on no list: it is the main menu's alone.
//   MUSIC.byLevel  the switch (pieces.ts): ON in the page, OFF in the module.
//                  The listener's ears only: no rule reads it.
//   the key        a level's file name without `.json` ('office' = the bundled level), and before it `<level>:<kind>` for a mission kind that wants its own on that level
//                  (the office holds four kinds of mission: the solo night's slow lean is not a rescue's clock).
//   a new level    is a row here.
// A level typed by hand that has no row (a test yard) plays SETLIST_ELSE. Names, not indices: PIECE_NAMES is read when a list is asked for, never at module load.
import { PIECE_NAMES } from './pieces';

/** level (or `level:kind`) → its pieces in playing order, by name (pieces.ts PIECE_NAMES). Each row's comment is why. */
export const SETLISTS: Record<string, readonly string[]> = {
  // ---- the office (the bundled level), by what is played on it
  'office': ['Night Shift', 'Paper Walls'],                // the solo night: a slow lean on an upright bass in an empty office, then the koto
  'office:nightbreach': ['Wire Act', 'Night Shift'],       // one room, four doors, a held breath: the twelve-eight tightrope
  'office:roomclear': ['Cold Storage', 'Night Shift'],     // the careful game, room by room: the bass line walks, the break waits behind the door
  'office:rescue': ['Dead Freight', 'Cold Storage'],       // a clock: the two fast ones
  // ---- raids and strongholds
  'records': ['Paper Walls', 'Cold Storage'],              // thin partitions and a strong room: the piece named for it
  'depot': ['Dead Freight', 'Wire Act'],                   // a motor pool: freight (E minor to A minor, a fifth: the easy handover)
  'block': ['Wire Act', 'Dead Freight'],                   // a lit leg and a dark turn in every room: balance, then drive
  'warehouse': ['Cold Storage', 'Dead Freight'],           // a racked hall and a loading dock
  'compound': ['Undertow', 'Glass Rain'],                  // eleven behind a wall at night: the bells, then frame drum and dulcimer
  'barracks': ['Wire Act', 'Undertow'],                    // two galleries that look across at each other
  'floodlit': ['Paper Walls', 'Wire Act'],                 // a lit yard to cross: both in twelve-eight, a fifth apart
  'hall': ['Undertow', 'Paper Walls'],                     // four men down a long bare hall under two guns: slow, heavy, led by drums
  'mousehole': ['Paper Walls', 'Undertow'],                // the walls are the way in
  'glasshouse': ['Glass Rain', 'Wire Act'],                // rooms walled in glass
  'pass': ['Undertow', 'Glass Rain'],                      // a teahouse under snow: bells, then the dulcimer
  'tut-team': ['Undertow', 'Glass Rain'],                  // the same station with the player on the ground: the same list
  'tut-road': ['Dead Freight', 'Undertow'],                // the tutorial's first: one man up a snowed-in road, loud from the first second: the drive, then the pass's bells
  'tut-kit': ['Dead Freight', 'Wire Act'],                 // the fuel halt again, alone with his kit and then with his four: the drive, then balance
  'tut-quiet': ['Night Shift', 'Glass Rain'],              // the tutorial's quiet house: the quiet missions' own piece, then the dulcimer
  'tut-pair': ['Night Shift', 'Undertow'],                 // the tutorial's truck park, as two: the quiet missions' own piece, then the bells
  // ---- on the clock
  'registry': ['Cold Storage', 'Wire Act'],
  'consulate': ['Wire Act', 'Cold Storage'],               // four rooms and no time to clear them: the quick twelve-eight, then the break
  'snatch': ['Dead Freight', 'Glass Rain'],                // a man who runs
  'defuse': ['Cold Storage', 'Dead Freight'],              // 1:15 on two charges: the fastest break (the second is for the retries)
  'killhouse': ['Dead Freight', 'Cold Storage'],           // a run against a medal time
  // ---- quiet
  'bazaar': ['Glass Rain', 'Paper Walls'],                 // a shuttered bazaar under curfew: two pieces with the same half step over the tonic
  'police-post': ['Night Shift', 'Wire Act'],
  'vault': ['Wire Act', 'Night Shift'],                    // tripwires
  'crossing': ['Undertow', 'Night Shift'],                 // two sentries across a lit road: slow, three against four
  'quiet-relay': ['Wire Act', 'Paper Walls'],
  'lights-out': ['Paper Walls', 'Night Shift'],
  'tut-queue': ['Wire Act', 'Undertow'],                   // the fourth tutorial mission, on the old plan mode mission's floor: its list
};
/** a level with no row of its own */
export const SETLIST_ELSE: readonly string[] = ['Night Shift', 'Wire Act'];

/** a level file's name as this table keys it: without `.json`, the office family (`office-*.json`) and no file at all = 'office' */
export function setlistLevel(levelFile: string | null | undefined): string {
  if (!levelFile || !/^[\w.-]+\.json$/.test(levelFile) || /^office(-[\w.-]+)?\.json$/.test(levelFile)) return 'office';
  return levelFile.replace(/\.json$/, '');
}
/** the row a level and a mission kind play, the key it is under, and whether it is one written for them (false = SETLIST_ELSE, under the level's name) */
export function setlistNames(level: string, kind: string | null = null): { key: string; names: readonly string[]; own: boolean } {
  for (const key of kind ? [`${level}:${kind}`, level] : [level]) if (Object.hasOwn(SETLISTS, key)) return { key, names: SETLISTS[key], own: true };   // (hasOwn: the level's name comes off a URL)
  return { key: level, names: SETLIST_ELSE, own: false };
}
/** …as PIECE_NAMES indices, what music.ts plays (a name that is no piece, and the theme, are dropped) */
export function setlistOf(level: string, kind: string | null = null): number[] {
  return setlistNames(level, kind).names.map(n => PIECE_NAMES.indexOf(n)).filter(i => i > 0);
}
