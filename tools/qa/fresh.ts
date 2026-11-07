// WHAT A START PUTS BACK. A game's state lives on the Game, and a start writes all of it. What the TypeScript still keeps at the top of a module is the
// PROCESS's, so a second game stood in one process found it as the first had left it: its men's unit numbers counted on from the first game's, and the
// first game's tallies and logs stood in the second's start (the Rust game is handed them as state: g.stats, g.m). A tool that stands several games
// compared a game with one that began elsewhere. Seen as: the same night breach stood twice in one process, the second's state another from tick 1.
// THE RULE, said once here and run by the harness's stand-up before every game (tools/qa/headless.ts standUp). Put back, as the process's FIRST start
// found it (so a word a tool set before it stood anything stays set), is what is KNOWN to be a game's, each known by what says so:
//   what a game writes while it runs, kept at a module's top   the registry's variables and roots (src/math/side.ts rollbackVar / rollbackRoot),
//                                                              but for THE_PROCESS_S (two that describe the process's tables)
//   what is counted of a game, or left by one call             a name the state bridge carries whose row of tools/qa/rollback-exempt.ts is a TALLY
//                                                              or a SCRATCH
//   what the Rust game itself keeps, and hands back            a carried name with no row there (the TypeScript no longer writes it, so the
//                                                              static pass lets it have none) that the bridge's schema reads into one of the
//                                                              Rust game's `…Stats` structs, or that comes back every tick (tools/wire/fields.json)
// EVERYTHING ELSE IS LEFT ALONE: a knob table, a row that is a CONFIG, a CACHE, a CONST or a HOST's, a table keyed weakly by a game's objects, and
// ANY CARRIED NAME NOTHING CLASSES. That last is on purpose. A name with no row may be a setting a tool turns between two games (the kit's cards, a
// pace switch), and a start that put it back would undo the tool's hand and leave its check green on nothing. So such a name is never written; if it
// has changed since the first start it is NAMED (FRESH.unlisted; start.ts --again prints it and the table is red on it), and whoever reads that gives
// it a row: `wire.ts list` if a game's state is written into it, in rollback-exempt.ts as a CONFIG if a tool or a mission's start sets it.
// A name is reached through the bridge's window on a module's own variables (tools/blitz/expose.ts), so with no bridge in the process only the registry
// is put back: no game is stepped there (only the Rust game steps), so no tally moves. Only what DIFFERS is written, in place, and never through a getter.
// The check that keeps it: tools/blitz/start-levels.ts --again (a second start in one process, after a game was stood and played, is the first's).
// STANDFRESH_BREAK=1 in front of it puts nothing back, and every row must be red. STANDFRESH_SAY=1 in front of any tool names what each stand-up put back.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { sideRegistry } from '../../src/math/side.ts';
import { knobRegistry } from '../../src/game/knobs.ts';
import { ROLLBACK_EXEMPT } from './rollback-exempt.ts';

/** registered names that describe the PROCESS's tables, not a game: put back, they would belie the tables they describe */
export const THE_PROCESS_S: Record<string, string> = {
  'src/game/kit.ts KIT': 'the kit cards a menu or a tool sets and the next start reads: a function with no game writes them here, and the scan carries them to a game that stands',
  'nightbreach.applied': 'which level\'s team numbers the knob tables hold now: the tables are the process\'s, and a start asks this to know whether to write them',
  'edgeLook.EDGE': 'a table of settings, registered because a failed build puts its `on` down in the middle of a frame: the numbers are a tool\'s to turn',
};

const plain = (v: unknown): v is Record<string, unknown> => { if (typeof v !== 'object' || v === null) return false; const p = Object.getPrototypeOf(v); return p === Object.prototype || p === null; };
const typed = (v: unknown): v is Float32Array => ArrayBuffer.isView(v) && !(v instanceof DataView);
/** the keys of a plain object that HOLD a value (a key with a getter is worked out from something else, and is neither kept nor written) */
const held = (o: Record<string, unknown>): string[] => Object.keys(o).filter(k => 'value' in Object.getOwnPropertyDescriptor(o, k)!);
/** a copy of plain data (arrays, plain objects, Maps, Sets, typed arrays); anything else is held as it is */
function copy(v: unknown): unknown {
  if (Array.isArray(v)) return v.map(copy);
  if (typed(v)) return v.slice();
  if (v instanceof Map) return new Map([...v].map(([k, x]) => [k, copy(x)]));
  if (v instanceof Set) return new Set(v);
  if (plain(v)) { const o: Record<string, unknown> = {}; for (const k of held(v)) o[k] = copy(v[k]); return o; }
  return v;
}
/** plain data held alike (anything else: the same object) */
function same(a: unknown, b: unknown): boolean {
  if (a === b) return true;
  if (typeof a === 'number' && typeof b === 'number') return Number.isNaN(a) && Number.isNaN(b);
  if (Array.isArray(a) && Array.isArray(b)) return a.length === b.length && a.every((x, i) => same(x, b[i]));
  if (typed(a) && typed(b)) return a.length === b.length && a.every((x, i) => Object.is(x, b[i]));
  if (a instanceof Map && b instanceof Map) return a.size === b.size && [...a].every(([k, x]) => b.has(k) && same(x, b.get(k)));
  if (a instanceof Set && b instanceof Set) return a.size === b.size && [...a].every(x => b.has(x));
  if (plain(a) && plain(b)) { const K = held(a); return K.length === held(b).length && K.every(k => Object.hasOwn(b, k) && same(a[k], b[k])); }
  return false;
}
/** `live` written IN PLACE to what `was` holds, a leaf only where it differs; false = it is no container of that kind, and the caller writes the value itself */
function putBack(live: unknown, was: unknown): boolean {
  if (Array.isArray(live) && Array.isArray(was)) { if (live.length > was.length) live.length = was.length; for (let i = 0; i < was.length; i++) if (i >= live.length || !(same(live[i], was[i]) || putBack(live[i], was[i]))) live[i] = copy(was[i]); return true; }
  if (typed(live) && typed(was) && live.length === was.length) { live.set(was); return true; }
  if (live instanceof Map && was instanceof Map) { for (const k of [...live.keys()]) if (!was.has(k)) live.delete(k); for (const [k, x] of was) if (!live.has(k) || !(same(live.get(k), x) || putBack(live.get(k), x))) live.set(k, copy(x)); return true; }
  if (live instanceof Set && was instanceof Set) { live.clear(); for (const x of was) live.add(x); return true; }
  if (plain(live) && plain(was)) {
    for (const k of held(live)) if (!Object.hasOwn(was, k)) delete live[k];
    for (const k of Object.keys(was)) if (!(same(live[k], was[k]) || putBack(live[k], was[k]))) live[k] = copy(was[k]);
    return true;
  }
  return false;
}

let GAMES: Set<string> | null = null;
/** the carried names that are a game's by the wire's own word: every `B.top(file, NAME)` a field of a struct named `…Stats` is read from, and every one that comes back every tick (tools/wire/fields.json `everyTick`: the Rust game writes it, and the apply lays it on the module's copy) */
function ofTheGame(): Set<string> {
  if (GAMES) return GAMES;
  GAMES = new Set();
  const types: Record<string, { k?: string; mod?: string; name?: string; fields?: { rust?: string; expr?: string }[] }> = JSON.parse(readFileSync(resolve(import.meta.dir, '../blitz/schema.json'), 'utf8')).types, every = new Set<string>(JSON.parse(readFileSync(resolve(import.meta.dir, '../wire/fields.json'), 'utf8')).everyTick);
  for (const t of Object.values(types)) if (t.k === 'struct') for (const f of t.fields ?? []) if (t.name?.endsWith('Stats') || every.has(`${t.mod}::${t.name}.${f.rust}`)) for (const m of (f.expr ?? '').matchAll(/B\.top\("([^"]+)", "([^"]+)"/g)) GAMES.add(`${m[1]} ${m[2]}`);
  return GAMES;
}

interface Kept { name: string; get: () => unknown; set: ((v: unknown) => void) | null; was: unknown }
const KEPT = new Map<string, Kept>(), WATCHED = new Map<string, Kept>();
// (KNOWN TO BE LEFT: a mission's start writes its team's numbers into the process's KNOB TABLES (NIGHTBREACH, RESCUE: where Sam and the men stand, how
// many come), and a later game whose start does not write that table finds them as left. A knob table is a setting, and a start here cannot tell a
// game's write to one from a tool's. The mend is a mission's numbers on the Game: rust/game_start/README.md, the team's start.)

/** how many starts this process has made, how many names are kept, the names the last start put back, and every carried name nothing classes that has
 *  changed since the process's first start (left as it is: see the header) */
export const FRESH = { starts: 0, kept: 0, back: [] as string[], unlisted: new Set<string>() };
/** before a game is made: the first call keeps every such name as it stands, every later one puts back each that has changed (a module loaded since the
 *  first start is kept from the start that first finds it) */
export function freshStart(): void {
  const had = new Set(KEPT.keys()), R = sideRegistry();
  const keep = (M: Map<string, Kept>, name: string, get: () => unknown, set: ((v: unknown) => void) | null) => { if (!M.has(name)) M.set(name, { name, get, set, was: copy(get()) }); };
  for (const v of R.vars) if (!(v.name in THE_PROCESS_S)) keep(KEPT, v.name, v.get, v.set as (v: unknown) => void);
  for (const r of R.roots) if (!(r.name in THE_PROCESS_S)) keep(KEPT, r.name, () => r.obj, null);
  const T = (globalThis as { __tpsTop?: Record<string, object> }).__tpsTop;
  if (T) {
    const registered = new Set<unknown>(R.roots.map(r => r.obj)), knobs = new Set<unknown>(Object.values(knobRegistry())), vars = new Set(R.vars.map(v => v.name));
    for (const file of Object.keys(T)) for (const [n, d] of Object.entries(Object.getOwnPropertyDescriptors(T[file]))) {
      const name = `${file} ${n}`; if (!d.get || KEPT.has(name) || WATCHED.has(name)) continue;
      if (vars.has(`${file.slice(file.lastIndexOf('/') + 1, -3)}.${n}`)) continue;   // (a registered variable: kept above, under its registered name)
      const row = ROLLBACK_EXEMPT[name], get = d.get.bind(T[file]), set = d.set ? d.set.bind(T[file]) : null;
      if (row !== undefined ? row.startsWith('TALLY ·') || row.startsWith('SCRATCH ·') : ofTheGame().has(name)) { keep(KEPT, name, get, set); continue; }
      if (row !== undefined || name in THE_PROCESS_S) continue;
      const v = get(); if (v instanceof WeakMap || v instanceof WeakSet || typeof v === 'function' || registered.has(v) || knobs.has(v)) continue;
      keep(WATCHED, name, get, set);
    }
  }
  FRESH.starts++; FRESH.kept = KEPT.size; FRESH.back = [];
  for (const k of WATCHED.values()) if (!FRESH.unlisted.has(k.name) && !same(k.get(), k.was)) FRESH.unlisted.add(k.name);
  if (process.env.STANDFRESH_BREAK === '1') return;   // (the check's red path: nothing is put back, as it was)
  for (const k of KEPT.values()) {
    if (!had.has(k.name)) continue;
    const live = k.get(); if (same(live, k.was)) continue;
    if (putBack(live, k.was)) { FRESH.back.push(k.name); continue; }
    // (a variable that holds a table keyed weakly by a game's own objects, made afresh by its module, is held as it is: nothing of it is a row to put
    //  back. Anything else that could not be written in place is NAMED, never passed over in silence)
    if (typeof k.was === 'object' && k.was !== null) { if (!(k.was instanceof WeakMap || k.was instanceof WeakSet)) FRESH.unlisted.add(`${k.name} (not written back: it is no longer the kind of thing the first start found)`); continue; }
    if (k.set) { k.set(k.was); FRESH.back.push(k.name); }
  }
  if (process.env.STANDFRESH_SAY === '1' && FRESH.back.length) process.stderr.write(`tools/qa/fresh.ts: stand-up ${FRESH.starts} put back ${FRESH.back.join(', ')}\n`);
}
