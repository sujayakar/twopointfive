// THE KNOBS RUST CODE READS (rust/sim/src/knobs.rs) — one list in the crate, by the path each knob has in the game's registered tables (game/knobs.ts
// knobRegistry: `MV.gait.lean`). A knob is config: tuned in the panel, carried by a recording, put on or off by a stand; the same for every game in the
// process and no state of a game, so it is in no block and no snapshot. THIS FILE IS THE ONE WRITER: as a tick opens, before anything of the tick has read a
// knob, every value of the list is written into the module's table, so the Rust code of that tick reads what the TypeScript of that tick reads, and a knob
// changed between two ticks (the panel, journal.ts applyKnobs, a recipe's tables, a check) is the module's by the next. OUTSIDE A TICK nothing has opened:
// what runs the module's knob-reading code there (an animator's update: a teleport's re-pose) hands the list over first (`knobsNow`), so a table changed
// since the last tick is read as it stands there too. A path that is not a number of a registered table is an error the first time the list is read, with
// its name. Nothing here is a rule of play.
import { sim, f64, u8, RUST } from '../sim/sim';
import { LAYERS } from '../anim/skeleton';
import { knobRegistry } from './knobs';
import { animTables } from './character';

interface Line { path: string; root: Record<string, unknown>; keys: string[] }
/** `lines` = the list as found in the registry, made the first time it is read, and `at` where the module keeps the values; `written` = how many times the
 *  list has been handed over; `tick` = a tick is open and has handed it; `hand` = the value handed over for line `i` (what a check's red path turns) */
export const RUST_KNOBS = { lines: null as Line[] | null, at: 0, written: 0, tick: false, hand(_i: number, v: number): number { return v; } };

/** a list of knobs.rs as the module has it: 1 the knobs' paths · 3 the roles' names · 5 the node lists' names · 8 the names of the rig's tables of numbers */
export function rustList(table: 1 | 3 | 5 | 8): string[] {
  const x = sim(), at = x.knob_addr(table), n = x.knob_len(table), b = u8(); let s = '';
  for (let i = 0; i < n; i++) s += String.fromCharCode(b[at + i]);
  return s.split('\n').filter(l => l.length > 0);
}
function value(l: Line): unknown { let v: unknown = l.root; for (const k of l.keys) v = v === null || typeof v !== 'object' ? undefined : (v as Record<string, unknown>)[k]; return v; }
const num = (v: unknown): number | null => typeof v === 'number' ? v : typeof v === 'boolean' ? +v : null;
/** the list, each path found in the registry (the tables are the game's own objects: found once, read every tick) */
export function rustKnobLines(): readonly { path: string; value: number | null }[] {
  if (!RUST_KNOBS.lines) {
    // (where a path's first word is looked up: a registered knob table, or `ANIM` = the animator's own tables of numbers that no panel tunes and no
    // recording carries — character.ts animTables. Either way the number is read where the TypeScript reads it, so there is one of it.)
    const R = { ...knobRegistry(), ANIM: animTables() } as unknown as Record<string, Record<string, unknown>>, x = sim();
    const L = rustList(1).map(path => { const [root, ...keys] = path.split('.'); return { path, root: R[root], keys }; });
    if (L.length !== x.knob_len(0)) throw new Error(`rust knobs: the module names ${L.length} paths for ${x.knob_len(0)} knobs`);
    for (const l of L) if (num(value(l)) === null) throw new Error(`rust knobs: "${l.path}" (rust/sim/src/knobs.rs) is not a number of a registered knob table (game/knobs.ts) or, under ANIM, of the animator's tables (character.ts animTables)`);
    RUST_KNOBS.lines = L; RUST_KNOBS.at = x.knob_addr(0) / 8;
  }
  return RUST_KNOBS.lines.map(l => ({ path: l.path, value: num(value(l)) }));
}
/** AS A TICK OPENS (Game.stepSeats, while a row of RUST that reads knobs is on): every knob of the list, as it stands now, into the module */
export function knobsToRust(): void {
  if (!RUST_KNOBS.lines) rustKnobLines();
  const L = RUST_KNOBS.lines!, m = f64(), AT = RUST_KNOBS.at;
  for (let i = 0; i < L.length; i++) { const v = num(value(L[i])); if (v === null) throw new Error(`rust knobs: "${L[i].path}" is no longer a number`); m[AT + i] = RUST_KNOBS.hand(i, v); }
  RUST_KNOBS.written++;
}
/** BEFORE THE MODULE'S KNOB-READING CODE RUNS OUTSIDE A TICK (CharacterAnimator.update calls it through anim/skeleton.ts LAYERS.knobs — the animator does not
 *  import this module — while a row of RUST that reads knobs is on): the list again, as it stands now. Inside a tick it is the tick's already and nothing
 *  is written. */
export function knobsNow(): void { if (RUST.walk && !RUST_KNOBS.tick) knobsToRust(); }
LAYERS.knobs = knobsNow;
