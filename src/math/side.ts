// SIDE TABLES A SNAPSHOT CAN SEE. The simulation keeps a great deal of its state beside its objects: `const states = new WeakMap<Guard, St>()` at the top of a module,
// a `seen = new WeakSet<GameEvent>()` on a class, a module `let`. A WeakMap cannot be listed, and a module's own variables cannot be reached from outside it, so a
// snapshot of the whole game (game/rollback.ts: take / restore, what rollback netcode is built on) could not hold any of it. This file is what the game's modules use instead:
//   SideMap / SideSet   a WeakMap / WeakSet that can ALSO be listed (its live keys, in the order first set) — everything else is the WeakMap it extends: same calls, same
//                       garbage collection (the keys are held weakly; the list is of WeakRefs, swept as it grows). A table at the TOP of a module is given its name
//                       (`new SideMap<Guard, St>('doorway.states')`): named tables are kept in a list, since nothing else points at them. A table made inside a function or
//                       on an object takes no name: the snapshot finds it where it hangs.
//   rollbackVar         a module's own `let`: its name, how to read it, how to write it back
//   rollbackRoot        a module-level Map / Set / array / object the game writes to while it runs: its name and the thing itself (restored in place)
// THE RULE (tools/qa/rollback-check.ts is red otherwise): no `new WeakMap` / `new WeakSet` in the simulation's folders; every top-level table there is named; every
// module `let` and every module-level container the simulation writes while it runs is registered here, or listed in the check with the reason it need not be.
// Nothing here is in src/game's import cycle (it imports nothing), so a module may call it in a top-level statement.

type Table = SideMap<object, unknown> | SideSet<object>;
const NAMED: Table[] = [];
let memos = 0;
const NAMES = new Set<string>();
function named(t: Table, name: string): void {
  if (NAMES.has(name)) throw new Error(`side table '${name}' is named twice`);
  NAMES.add(name); NAMED.push(t);
}

/** the weak keys of one table, listable: WeakRefs in the order first set, with a WeakSet beside them so a key is listed once however often it is deleted and set again */
class Keys {
  refs: WeakRef<object>[] = [];
  private listed = new WeakSet<object>();
  private sweepAt = 64;
  add(k: object, has: (k: object) => boolean): void {
    if (this.listed.has(k)) return;
    this.listed.add(k); this.refs.push(new WeakRef(k));
    if (this.refs.length >= this.sweepAt) { this.sweep(has); this.sweepAt = Math.max(64, 2 * this.refs.length); }
  }
  /** drop the refs whose key is gone (collected, or deleted from the table) */
  sweep(has: (k: object) => boolean): void {
    const r = this.refs; let n = 0;
    for (let i = 0; i < r.length; i++) { const k = r[i].deref(); if (k === undefined) continue; if (has(k)) r[n++] = r[i]; else this.listed.delete(k); }
    r.length = n;
  }
}

export class SideMap<K extends object, V> extends WeakMap<K, V> {
  /** the table's name when it stands at the top of a module ('module.variable'), undefined on one that hangs off an object */
  readonly sideName: string | undefined;
  /** a MEMO: what a pure function of its key gave last time — a hit gives what a miss would compute, whichever game in the process filled it. Not state: no snapshot
   *  holds it, nothing compares it. Said of a table in the statement after it (`const hasNvg = new SideMap<…>('cast.hasNvg'); hasNvg.asMemo();`), with the reason */
  sideMemo = false;
  asMemo(): this { this.sideMemo = true; memos++; return this; }
  private readonly sideKeys = new Keys();
  private readonly sideHas = (k: object): boolean => super.has(k as K);
  constructor(name?: string) { super(); this.sideName = name; if (name !== undefined) named(this as unknown as Table, name); }
  override set(k: K, v: V): this { super.set(k, v); this.sideKeys.add(k, this.sideHas); return this; }   // (listed AFTER it is in: the list's sweep drops what the table does not hold)
  /** every key the table holds now, in the order first set */
  liveKeys(out: K[] = []): K[] { for (const r of this.sideKeys.refs) { const k = r.deref() as K | undefined; if (k !== undefined && super.has(k)) out.push(k); } return out; }
}

export class SideSet<K extends object> extends WeakSet<K> {
  readonly sideName: string | undefined;
  sideMemo = false;
  asMemo(): this { this.sideMemo = true; memos++; return this; }
  private readonly sideKeys = new Keys();
  private readonly sideHas = (k: object): boolean => super.has(k as K);
  constructor(name?: string) { super(); this.sideName = name; if (name !== undefined) named(this as unknown as Table, name); }
  override add(k: K): this { super.add(k); this.sideKeys.add(k, this.sideHas); return this; }
  liveKeys(out: K[] = []): K[] { for (const r of this.sideKeys.refs) { const k = r.deref() as K | undefined; if (k !== undefined && super.has(k)) out.push(k); } return out; }
}

/** A VIEW STANDS FOR ITS GAME AS A KEY. A seat's view of the game (game/seats.ts) is another object than the Game it looks at, and a table keyed by "the game" that
 *  was handed a view would keep a second row for it: the frags in the air, the lanes, the glass would each exist once a seat. A view answers its game under `RAW`,
 *  and the two classes below — the same tables, for state that is THE GAME'S whoever asks — look every key up by it. A plain SideMap keyed by a game keeps a row
 *  for each view it is handed, which is right for state that is one PLAYER'S (the tap step, the creep, his own lamp's clock): the row is the seat's.
 *  `watchViewKeys()` (a check's: tools/qa/seats-check.ts) notes, from then on, every named table asked or written under a VIEW (`VIEW_KEYS.seen`) and every one
 *  asked or written under a Game of more than one seat ITSELF (`VIEW_KEYS.raw`). A table is one or the other: the check holds the first list against the tables
 *  that are a seat's (game/seats.ts SEAT_TABLES) and the second against the rest — a world table read through a view finds no row, a seat's read through the
 *  Game finds the wrong one, and neither throws. Nothing is noted, and no call costs anything more, until it is asked. */
export const RAW: unique symbol = Symbol('raw');
/** set on the view another cast of the squad's code is run over (game/squadcast.ts): what tells it from a Game and from a seat's view */
export const CAST_VIEW: unique symbol = Symbol('cast view');
export const rawKey = <K extends object>(k: K): K => (k as { [RAW]?: K })[RAW] ?? k;
export const VIEW_KEYS = { watch: false, seen: new Set<string>(), raw: new Set<string>() };
export class GameSideMap<K extends object, V> extends SideMap<K, V> {
  override get(k: K): V | undefined { return super.get(rawKey(k)); }
  override has(k: K): boolean { return super.has(rawKey(k)); }
  override set(k: K, v: V): this { return super.set(rawKey(k), v); }
  override delete(k: K): boolean { return super.delete(rawKey(k)); }
}
export class GameSideSet<K extends object> extends SideSet<K> {
  override has(k: K): boolean { return super.has(rawKey(k)); }
  override add(k: K): this { return super.add(rawKey(k)); }
  override delete(k: K): boolean { return super.delete(rawKey(k)); }
}

export function watchViewKeys(): void {
  if (VIEW_KEYS.watch) return; VIEW_KEYS.watch = true;
  const note = (t: { sideName?: string }, k: unknown): void => {
    if (t.sideName === undefined || typeof k !== 'object' || k === null) return;
    if ((k as { [RAW]?: object })[RAW] !== undefined) VIEW_KEYS.seen.add(t.sideName);
    else { const S = (k as { seats?: unknown; stepSeats?: unknown }); if (typeof S.stepSeats === 'function' && Array.isArray(S.seats) && S.seats.length > 1) VIEW_KEYS.raw.add(t.sideName); }
  };
  type Calls = Record<string, (this: { sideName?: string }, k: unknown, ...r: unknown[]) => unknown>;
  for (const P of [SideMap.prototype, SideSet.prototype] as unknown as Calls[]) for (const m of ['get', 'has', 'delete', 'set', 'add']) {
    const was = P[m]; if (typeof was !== 'function') continue;
    P[m] = function (k, ...r) { note(this, k); return was.call(this, k, ...r); };
  }
}

/** a module's own variable: read and written back through the two functions the module hands over */
export interface SideVar { name: string; get: () => unknown; set: (v: never) => void }
const VARS: SideVar[] = [];
/** register a module `let` the simulation writes while it runs: `rollbackVar('weather.clock', () => clock, v => { clock = v; })` */
export function rollbackVar<T>(name: string, get: () => T, set: (v: T) => void): void {
  if (NAMES.has(name)) throw new Error(`rollback variable '${name}' is named twice`);
  NAMES.add(name); VARS.push({ name, get, set: set as (v: never) => void });
}
const ROOTS: { name: string; obj: object }[] = [];
/** register a module-level Map / Set / array / object the simulation writes while it runs (restored in place); returns it, so a declaration can be wrapped */
export function rollbackRoot<T extends object>(name: string, obj: T): T {
  if (NAMES.has(name)) throw new Error(`rollback root '${name}' is named twice`);
  NAMES.add(name); ROOTS.push({ name, obj }); return obj;
}

/** fields that hold a reading of the WALL clock (a timer's `ms`, a run's name made of the time): kept and put back by a snapshot like any field, but never compared —
 *  two plays of one seed differ in them and in nothing else. Marked on the object that holds them, where it is made: `stats = wallClock({ awake: 0, ms: 0 }, 'ms')` */
const CLOCKED = new WeakMap<object, readonly string[]>();
export function wallClock<T extends object>(o: T, ...keys: (keyof T & string)[]): T { CLOCKED.set(o, keys); return o; }
export function wallClockKeys(o: object): readonly string[] | undefined { return CLOCKED.get(o); }

/** WHAT IS THE HOST'S INSIDE A GAME OBJECT: the GPU's own state a GPU object of the game's keeps beside what the simulation reads — buffers out on a readback, the
 *  ones free, what was last sent. A snapshot holds such an object as it is: never copied, never put back, never compared, and a restore into another game leaves
 *  that game its own. It is a class that extends this one (render/probe.ts `Plumbing`), so the snapshot knows it by its make and pays nothing per object. */
export class HostOwned {}
/** HOW MANY SNAPSHOTS HAVE BEEN PUT BACK in this process (game/rollback.ts restoreSnapshot counts). A put-back writes game objects from outside the game's own
 *  code, so a host-side copy of some of them that is kept current by being TOLD of writes (scene/boxMirror.ts: the grid's cells) reads this to know it was not
 *  told. The host's: no game reads it. */
export const PUT_BACKS = { n: 0 };

/** everything registered, for game/rollback.ts: the named tables (but the memos), the variables and the roots, each in the order of its NAME (the same on every machine, whatever order the modules ran in) */
export function sideRegistry(): { tables: readonly Table[]; vars: readonly SideVar[]; roots: readonly { name: string; obj: object }[] } {
  if (sorted !== NAMES.size || sortedMemos !== memos) {
    NAMED.sort((a, b) => a.sideName! < b.sideName! ? -1 : 1); VARS.sort((a, b) => a.name < b.name ? -1 : 1); ROOTS.sort((a, b) => a.name < b.name ? -1 : 1); sorted = NAMES.size;
    STATE = NAMED.filter(t => !t.sideMemo); sortedMemos = memos;
  }
  return { tables: STATE, vars: VARS, roots: ROOTS };
}
let sorted = 0, sortedMemos = 0, STATE: Table[] = [];
