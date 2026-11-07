// WHAT A START PUTS BACK. A game's state lives on the Game, and a start writes all of it. What the TypeScript still keeps at the top of a module is the
// PROCESS's, so a second game stood in one process found it as the first had left it: its men's unit numbers counted on from the first game's, and the
// first game's tallies and logs stood in the second's start (the Rust game is handed them as state: g.stats, g.m). A tool that stands several games
// compared a game with one that began elsewhere. Seen as: the same night breach stood twice in one process, the second's state another from tick 1.
// THE RULE, said once here and run by the harness's stand-up before every game (tools/qa/headless.ts standUp). Put back, as the process's FIRST start
// found it (so a word a tool set before it stood anything stays set):
//   what a game writes while it runs, kept at a module's top   the registry's variables and roots (src/math/side.ts rollbackVar / rollbackRoot)
//   what a game is handed as its state from a module's top     every name the state bridge carries (tools/blitz/schema.json `top`)
// but for what is the process's ON PURPOSE, and that is said by the thing itself, never by a list here:
//   a setting                 a knob table (src/game/knobs.ts knobRegistry), or a row of tools/qa/rollback-exempt.ts that is a CONFIG, a CACHE, a CONST
//                             or a HOST's (a tool turns it between games, or it is a memo of a pure function). A TALLY's row and a SCRATCH's (what
//                             one call left in it: nothing stands in it between ticks) are a game's, and put back
//   a table keyed weakly      by a game's own objects: a new game's objects have no row in it
//   THE_PROCESS_S             two registered names that describe the process's tables, each with its reason
// A carried name with NO row and no registration is one the TypeScript does not write (tools/qa/rollback-check.ts --static is red on a written one):
// if it has changed, a Rust game's state was written into it through the bridge, so it is a game's, and it is put back.
// A name is reached through the bridge's window on a module's own variables (tools/blitz/expose.ts), so with no bridge in the process only the registry
// is put back: no state is handed to a Rust game there. Only what has CHANGED is written, and in place (whoever keeps the object keeps it).
// The check that keeps it: tools/blitz/start-levels.ts --again (a second start in one process, after a game was stood and played, is the first's).
// FRESH_BREAK=1 in front of it puts nothing back, and every row must be red.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { sideRegistry } from '../../src/math/side.ts';
import { knobRegistry } from '../../src/game/knobs.ts';
import { ROLLBACK_EXEMPT } from './rollback-exempt.ts';

/** registered names that describe the PROCESS's tables, not a game: put back, they would belie the tables they describe */
export const THE_PROCESS_S: Record<string, string> = {
  'nightbreach.applied': 'which level\'s team numbers the knob tables hold now: the tables are the process\'s, and a start asks this to know whether to write them',
  'edgeLook.EDGE': 'a table of settings, registered because a failed build puts its `on` down in the middle of a frame: the numbers are a tool\'s to turn',
};

const plain = (v: unknown): v is Record<string, unknown> => { if (typeof v !== 'object' || v === null) return false; const p = Object.getPrototypeOf(v); return p === Object.prototype || p === null; };
const typed = (v: unknown): v is Float32Array => ArrayBuffer.isView(v) && !(v instanceof DataView);
/** a copy of plain data (arrays, plain objects, Maps, Sets, typed arrays); anything else is held as it is */
function copy(v: unknown): unknown {
  if (Array.isArray(v)) return v.map(copy);
  if (typed(v)) return v.slice();
  if (v instanceof Map) return new Map([...v].map(([k, x]) => [k, copy(x)]));
  if (v instanceof Set) return new Set(v);
  if (plain(v)) { const o: Record<string, unknown> = {}; for (const k of Object.keys(v)) o[k] = copy(v[k]); return o; }
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
  if (plain(a) && plain(b)) { const K = Object.keys(a); return K.length === Object.keys(b).length && K.every(k => k in b && same(a[k], b[k])); }
  return false;
}
/** `live` written IN PLACE to what `was` holds; false = it is no container of that kind, and the caller writes the value itself */
function putBack(live: unknown, was: unknown): boolean {
  if (Array.isArray(live) && Array.isArray(was)) { live.length = Math.min(live.length, was.length); for (let i = 0; i < was.length; i++) if (i >= live.length || !putBack(live[i], was[i])) live[i] = copy(was[i]); return true; }
  if (typed(live) && typed(was) && live.length === was.length) { live.set(was); return true; }
  if (live instanceof Map && was instanceof Map) { for (const k of [...live.keys()]) if (!was.has(k)) live.delete(k); for (const [k, x] of was) if (!live.has(k) || !putBack(live.get(k), x)) live.set(k, copy(x)); return true; }
  if (live instanceof Set && was instanceof Set) { live.clear(); for (const x of was) live.add(x); return true; }
  if (plain(live) && plain(was)) {
    for (const k of Object.keys(live)) if (!(k in was)) delete live[k];
    for (const k of Object.keys(was)) if (!putBack(live[k], was[k])) live[k] = copy(was[k]);
    return true;
  }
  return false;
}

interface Kept { name: string; get: () => unknown; set: ((v: unknown) => void) | null; was: unknown }
const KEPT = new Map<string, Kept>();
let CARRIED: [string, string][] | null = null;
/** the names the bridge carries that no row says are the process's: [file, name] */
// (KNOWN TO BE LEFT: a mission's start writes its team's numbers into the process's KNOB TABLES (NIGHTBREACH, RESCUE: where Sam and the men stand, how
// many come), and a later game whose start does not write that table finds them as left. A knob table is a setting and a start here cannot tell a
// game's write from a tool's, so a tool that runs missions of several kinds puts the tables back itself (knobsCopy / knobsBack, as playthrough.ts
// does around a run). The mend is a mission's numbers on the Game: rust/game_start/README.md, the team's start.)
function carried(): [string, string][] {
  if (CARRIED) return CARRIED;
  const top: Record<string, string[]> = JSON.parse(readFileSync(resolve(import.meta.dir, '../blitz/schema.json'), 'utf8')).top;
  CARRIED = []; for (const [file, list] of Object.entries(top)) for (const n of new Set(list)) { const row = ROLLBACK_EXEMPT[`${file} ${n}`]; if (row === undefined || row.startsWith('TALLY ·') || row.startsWith('SCRATCH ·')) CARRIED.push([file, n]); }
  return CARRIED;
}

/** how many starts this process has made, how many names are kept, and the names the last start put back */
export const FRESH = { starts: 0, kept: 0, back: [] as string[] };
/** before a game is made: the first call keeps every such name as it stands, every later one puts back each that has changed (a module loaded since the
 *  first start is kept from the start that first finds it) */
export function freshStart(): void {
  const had = new Set(KEPT.keys()), R = sideRegistry();
  const keep = (name: string, get: () => unknown, set: ((v: unknown) => void) | null) => { if (!KEPT.has(name)) KEPT.set(name, { name, get, set, was: copy(get()) }); };
  for (const v of R.vars) if (!(v.name in THE_PROCESS_S)) keep(v.name, v.get, v.set as (v: unknown) => void);
  for (const r of R.roots) if (!(r.name in THE_PROCESS_S)) keep(r.name, () => r.obj, null);
  const T = (globalThis as { __tpsTop?: Record<string, object> }).__tpsTop;
  if (T) {
    const registered = new Set<unknown>(R.roots.map(r => r.obj)), knobs = new Set<unknown>(Object.values(knobRegistry())), vars = new Set(R.vars.map(v => v.name));
    for (const [file, n] of carried()) {
      const O = T[file], d = O && Object.getOwnPropertyDescriptor(O, n); if (!d?.get || KEPT.has(`${file} ${n}`)) continue;
      if (vars.has(`${file.slice(file.lastIndexOf('/') + 1, -3)}.${n}`)) continue;   // (a registered variable: kept above, under its registered name)
      const v = d.get.call(O); if (v instanceof WeakMap || v instanceof WeakSet || typeof v === 'function' || registered.has(v) || knobs.has(v)) continue;
      keep(`${file} ${n}`, d.get.bind(O), d.set ? d.set.bind(O) : null);
    }
  }
  FRESH.starts++; FRESH.kept = KEPT.size; FRESH.back = [];
  if (process.env.FRESH_BREAK === '1') return;   // (the check's red path: nothing is put back, as it was)
  for (const k of KEPT.values()) {
    if (!had.has(k.name)) continue;
    const live = k.get(); if (same(live, k.was)) continue;
    if (putBack(live, k.was)) { FRESH.back.push(k.name); continue; }
    // (a table keyed weakly by a game's own objects, or any object that is no plain data, is held as it is)
    if (typeof k.was === 'object' && k.was !== null) continue;
    if (k.set) { k.set(k.was); FRESH.back.push(k.name); }
  }
}
