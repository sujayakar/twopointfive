// THE EXPECTED REDS: probes that are red for a reason somebody has written down, by PROBE name (the name `probes.ts <name>` runs alone; a family's word is not
// a row). probes.ts prints its reds against this list after the verdict line (EXPECTED RED with the row / NEW RED / GREEN AGAIN), and gate.sh carries those
// lines. The list changes NO verdict: a listed probe still fails its run. It is there so a red gate can be read at a glance (which reds are yesterday's, which
// one is yours) and so a red that somebody fixed is noticed (GREEN AGAIN).
// ONE LIST. A row is of the page's stand, the stand a tool has with no word (src/host/standEnv.ts), unless its `on` says the old stand too ('both') or the old
// stand alone ('old'): a run is read against the rows of its own stand. Until 2026-11-06 there were two tables here, one for either stand and one for the page's.
// A run started with a word of the stand given by hand (a rule raised or held off by name) is not the stand as it opens, and a run of one case of a probe is
// not the probe: neither is read against any row (probes.ts says so in one line where it has a red).
// A run started with a break word (stand.ts BREAK_WORDS) IS read against the rows, and probes.ts adds a line on whether the word bit: a row can hide it.
// tools/qa/holds-check.ts (8) holds the rows: every name is a probe the runner runs, every word of a `bisect` is one the stand can raise, none is a word the
// probe's own holds put off, and no row that is red on the old stand carries a bisect. What is prose in a row is as right as its writer.
import { RED_WORLD } from './expected-red-world.ts';
import { RED_KIT } from './expected-red-kit.ts';
import { RED_DOORS } from './expected-red-doors.ts';
import { RED_STEALTH } from './expected-red-stealth.ts';
/** WHAT GAME THE ROWS WERE LAST READ ON, said under the closing table on the page's stand while it is not the game a tool stands today ('' once they are).
 *  The rows below were read on the TypeScript game with nothing folded. Since 2026-11-07 a tool with no word stands the Rust game, where the settled switch
 *  rows are constants: a probe that staged itself on one of those rows fails there for that, whatever its row says. */
export const LIST_READ_ON: string = 'the rows were last read on the TypeScript game, before its tick was deleted (2026-11-07), and not yet again on the Rust game: an EXPECTED RED may fail here by another line than its row says, and a GREEN AGAIN may be a staging that was refused';
export interface RedRow {
  /** WHAT IT IS. 'stale' = the check is wrong, or asks a path the page no longer has: the tools' to mend or delete, nothing of the game's · 'real' = a fault of
   *  the page, traced in the row · 'maybe' = could be real: not settled, and its owner's read is owed before anything copies the behaviour · 'open' = not sorted */
  is: 'stale' | 'real' | 'maybe' | 'open';
  /** which way an unsettled reading points, where it points anywhere ('maybe' and 'open' rows only) */
  leans?: 'stale';
  /** WHICH STAND IT IS RED ON. Absent = the page's alone · 'both' = the old stand (TPS_PROFILE=frozen) as well · 'old' = the old stand alone */
  on?: 'both' | 'old';
  /** WHICH RULES OF THE PAGE TURN IT RED, as measured (page-only rows: the probe is green on the old stand).
   *  `enough` = the stand's words that flip the probe's first failing line when raised on the old stand (tools/qa/stand-bisect.ts; a word written alone is
   *  raised to 1). ENOUGH, NOT ALL: the search stops at the first set that does it, so another rule may turn the same line as well; it raises a word over a
   *  probe's own hold; and a word read only as a game is stood up cannot be raised by it. `runs` = how many runs of the probe the search took.
   *  `needed` = the other way round: with those words held off on the page's stand the probe is green (true), or stays red, so something else turns it too
   *  (false). Absent = not tried. */
  bisect?: { enough: string[]; runs?: number; needed?: boolean };
  /** what the failing line says, and what was read off it */
  line: string;
  /** whose it is to turn green */
  whose: string;
  /** READ ON THE RUST GAME, the game every tool stands since 2026-11-07 (a row made by tools/qa/red-rows.ts from a run of it). Absent = an older row, read on
   *  the TypeScript game: LIST_READ_ON's line under the runner's table is about those. */
  rust?: true;
}
/** A row as one line for the runner's table: what it is, whose, the rules, then the reading (the gate cuts a line short: the short fields come first). */
export function rowSays(r: RedRow): string {
  const what = r.is === 'stale' ? 'STALE CHECK' : r.is === 'real' ? 'REAL' : `${r.is === 'maybe' ? 'COULD BE REAL' : 'NOT SORTED'}${r.leans ? ', leans stale' : ''}`;
  const b = r.bisect, by = b ? ` · enough to turn it: ${b.enough.join(' + ')}${b.runs ? ` (${b.runs} runs)` : ''}${b.needed === undefined ? '' : b.needed ? ', and needed' : ', and NOT all of it: red with them held off too'}` : '';
  return `${r.rust ? 'ON THE RUST GAME · ' : ''}${what}${r.on === 'both' ? ' (the old stand too)' : r.on === 'old' ? ' (the old stand alone)' : ''} · ${r.whose}${by} · ${r.line}`;
}
/** the switch cleanup's rows: a check to mend, restage or delete in the tools */
const TOOLS = "the switch cleanup's (tools)";
// The rows after the older ones were read on 2026-11-06, on step 1566's tree with the stand's flip and again on step 1582's, from a sweep of the 458 names the
// build's gate does not hold and from the gate's own reds. The list is what was read, not a promise that nothing else is red: a red with no row is NEW RED.
const OLDER: Record<string, RedRow> = {
  // --- older rows, each in its writer's own words
  'aim-door': { is: 'real', bisect: { enough: ['TPS_GUNWALL', 'TPS_CLOSE', 'ONEWAY'] }, line: "red on the page's stand on steps 1302, 1308 and 1310 · line [8], the break room's corridor door shut with Sam dark and planted right behind the leaf · THE FAULT, from the line's own trace: the guard opens the door at 3.0 s and the leaf stops at 1.28 rad on Sam's body, between the two of them; from 5.0 s he stands in the frame 0.9 m from Sam, alert, not moving (path 1/1, speed 0), his sight of Sam coming and going every half second round the leaf's edge, and fires nothing until 11.2 s — then a round a second or so, 10 in 24 s, where the frozen stand's guard walks in and fires 24 from 3.2 s. A defender who has opened a leaf onto the man he is after has no move for the leaf itself: he neither steps round it nor backs off it, and waits at arm's length for a line · three rules of the page turn it, and only together: guns do not go through walls (TPS_GUNWALL: his round is held while the leaf is in it), the close-in mode (TPS_CLOSE), one-way doors (ONEWAY: the leaf swings Sam's way and his body stops it) — tools/qa/stand-bisect.ts · which of his own rules plants him there is not traced", whose: "the enemies' owner (the defenders at a door: docs/agents/enemies.md)" },
  // --- could be real: an owner's read is owed
  'hourwx-storm': { is: 'maybe', on: 'both', line: "'9 wet (the lot and 8 sheets): NOT ALL UNDER OPEN SKY' on seven of the levels it walks: a sheet of the storm's wet ground with no sky over it, or the line's own reading of open sky against the level's own roofs (WX_OWN.roofs, an open decision). Not read", whose: "the weather's owner" },
  // --- stale checks: the tools' to mend or delete, nothing of the game's
  // --- not sorted
};
/** THE TABLES THE LIST IS MADE OF, by name. `older` = the rows above. The four after it are the Rust game's reds, one file a group of check families so that four
 *  hands-over do not meet in one file (which family is whose: the running family of a probe's line in probes.ts): rows printed by tools/qa/red-rows.ts from a
 *  run, for probes whose failing lines are all the page's own side. No name may be in two tables (holds-check (8)). */
export const RED_TABLES: Record<string, Record<string, RedRow>> = { older: OLDER, world: RED_WORLD, kit: RED_KIT, doors: RED_DOORS, stealth: RED_STEALTH };
export const EXPECTED_RED: Record<string, RedRow> = Object.assign({}, ...Object.values(RED_TABLES));
