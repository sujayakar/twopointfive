// THE STATE BRIDGE, THE TYPESCRIPT'S SIDE: a live Game as bytes, walked by bridge/schema.json (written by the skeleton's program from the same tables as rust/game's
// types), in the order of types.rs's fields. rust/game/src/io.rs walks the same order. Numbers are raw doubles (f32 where the field is one), a bool a byte, an Option a
// tag byte, a string its length and utf-8, a Vec / JsMap / JsSet a count and its items in the TypeScript's own order, a word a u16, a thing with an ID a u32.
// IDS: a table is first filled from the TypeScript's own list where there is one (`seeds`), then by first reach in the walk; the map from object to ID is kept per game,
// so two exports of one game agree. TABLES COME LAST because walking a row can reach a new thing of another table.
// exportGame(g) never writes to the game. What it could not carry is in `report()`: every kind of value that did not fit the schema's type, by field.
import { readFileSync } from 'node:fs';
import { sideRegistry, rawKey, SideMap, SideSet, GameSideMap, GameSideSet } from '../../src/math/side';

const here = import.meta.dir;
export const SCHEMA: any = JSON.parse(readFileSync(`${here}/schema.json`, 'utf8'));

// ---- a type's text, parsed
export type Node = { k: string; [x: string]: any };
const memo = new Map<string, Node>();
export function splitTop(s: string): string[] { const out: string[] = []; let d = 0, st = 0; for (let i = 0; i < s.length; i++) { const c = s[i]; if (c === '<' || c === '(' || c === '[') d++; else if (c === '>' && s[i - 1] !== '-' || c === ')' || c === ']') d--; else if (c === ',' && d === 0) { out.push(s.slice(st, i).trim()); st = i + 1; } } const l = s.slice(st).trim(); if (l) out.push(l); return out; }
const PRIM = new Set(['f64', 'f32', 'bool', 'u8', 'i8', 'u16', 'i16', 'u32', 'i32', 'u64', 'i64', 'usize']);
const IDS = new Set<string>(SCHEMA.ids);
export function parse(t0: string): Node {
  const hit = memo.get(t0); if (hit) return hit;
  const t = t0.trim(); let n: Node; let m: RegExpMatchArray | null;
  if (PRIM.has(t)) n = { k: t };
  else if (t === 'String' || t === "&'static str" || t === '&str') n = { k: 'str' };
  else if (t === 'V3') n = { k: 'arr', of: parse('f64'), n: 3 };
  else if (t === 'Quat') n = { k: 'arr', of: parse('f64'), n: 4 };
  else if (t === 'crate::v3::m4::Mat4') n = { k: 'arr', of: parse('f32'), n: 16 };
  else if (t === '()' || t === 'crate::Opaque' || t === 'crate::FnField') n = { k: 'unit' };
  else if (t === 'crate::Num') n = { k: 'f64' };
  else if ((m = t.match(/^Option<(.*)>$/))) n = { k: 'opt', of: parse(m[1]) };
  else if ((m = t.match(/^Vec<(.*)>$/))) n = { k: 'vec', of: parse(m[1]) };
  else if ((m = t.match(/^Tab<(.*)>$/))) n = { k: 'vec', of: parse(splitTop(m[1])[1]) };
  else if ((m = t.match(/^alloc::boxed::Box<(.*)>$/))) n = parse(m[1]);
  else if ((m = t.match(/^JsMap<(.*)>$/))) { const [K, V] = splitTop(m[1]); n = { k: 'map', K: parse(K), V: parse(V) }; }
  else if ((m = t.match(/^JsSet<(.*)>$/))) n = { k: 'set', K: parse(m[1]) };
  else if ((m = t.match(/^\[(.+); (\d+)\]$/))) n = { k: 'arr', of: parse(m[1]), n: +m[2] };
  else if ((m = t.match(/^\((.*)\)$/))) n = { k: 'tup', of: splitTop(m[1]).map(parse) };
  else if (IDS.has(t)) n = { k: 'id', id: t };
  else if (SCHEMA.types[t]) { memo.set(t0, n = { k: 'type', path: t }); return n; }
  else throw new Error(`bridge: no type ${t}`);
  memo.set(t0, n); return n;
}
const fieldFn = new Map<string, (o: any, g: any, B: any) => any>();
const compiled = (expr: string) => { let f = fieldFn.get(expr); if (!f) { f = new Function('o', 'g', 'B', `return ${expr}`) as any; fieldFn.set(expr, f!); } return f!; };

// ---- what a game's walk keeps between exports
// AN ID WRITTEN FOR A ROW THE RUST GAME HAS NOT GOT. `known` (import.ts, after every apply) is how many rows of each table the Rust game has; an object the
// TypeScript made since has a number past that, and bytes that hold it name a row that is not there: a list of the Rust game's would index past its
// table's end on the next tick. Every such id written is counted here; the scan reads the count round a leaf's bytes and does not send them (scan.ts).
export const PAST = { n: 0, id: '' };
// WHICH GAME A ROW IS OF: the book that gave it its number, so that a call handed a row is that row's game's call and no other's (surface.ts). A row two books hold (a thing every game of a process shares) names no game.
const ROW_BOOK = new WeakMap<object, Book | null>();
export const gameOfRow = (o: object): object | null => ROW_BOOK.get(o)?.game ?? null;
class Book { constructor(readonly game: object) {} /** rows of each table the Rust game has, once an apply has said (import.ts) */ known?: Map<string, number>; ids = new Map<string, Map<object, number>>(); lists = new Map<string, object[]>(); serial = new WeakMap<object, number>(); nextSerial = 1; serials = new Map<object, number>(); /** once the Rust game deals the serials (import.ts takes its counter with every apply), what this side deals comes from a range of its own: two things must never share a number */ rustDeals = false; ownSerial = 0x40000000;
  /** (for the importer: every object that has a serial) */ serialOfObjects() { return this.serials; }
  first = new Map<object, string>();
  /** poses that have no place in the pool, in the order first met: their floats go at the end of the bytes */
  loose = new Map<object, number>();
  /** an ID of either of two tables (schema `mixes`, io.rs AnyBoxId): the row of the second table if it has this object, else of the first */
  idOf(id: string, o: object): number { const mix = SCHEMA.mixes?.[id]; if (mix) { const f = this.ids.get(mix[1])?.get(o); return f !== undefined ? 2 * f + 1 : 2 * this.idOf(mix[0], o); } if (this.ofTheOther(id, o)) return PAST_ANY; let M = this.ids.get(id); if (!M) { M = new Map(); this.ids.set(id, M); this.lists.set(id, []); } let i = M.get(o); if (i === undefined) { i = M.size; M.set(o, i); this.lists.get(id)!.push(o); this.first.set(o, AT); const was = ROW_BOOK.get(o); ROW_BOOK.set(o, was === undefined || was === this ? this : null); } return i; }
  /** a row of the other table of a mix, asked for by this table's ID: a row of neither Rust can read, and never a new row of this table */
  private ofTheOther(id: string, o: object): boolean { for (const m of Object.values<string[]>(SCHEMA.mixes ?? {})) { const k = m.indexOf(id); if (k >= 0 && this.ids.get(m[1 - k])?.has(o)) return true; } return false; }
  /** is this number, as `idOf` answered it, past what the Rust game has? */
  past(id: string, i: number): boolean { const mix = SCHEMA.mixes?.[id]; if (mix) return this.past(mix[i & 1], i >>> 1); return i >= (this.known?.get(id) ?? (i === PAST_ANY ? 0 : Infinity)); }
  /** has the Rust game this object as a row of this table, or of either table of a mix? */
  hasRow(id: string, o: object): boolean { const mix = SCHEMA.mixes?.[id]; if (mix) return this.hasRow(mix[0], o) || this.hasRow(mix[1], o); const i = this.ids.get(id)?.get(o); return i !== undefined && i < (this.known?.get(id) ?? Infinity); }
  /** a mix's list has each table's rows at the numbers the wire gives them: the first's at the even places, the second's at the odd. Filled as the two grow */
  private mixed = new Map<string, { a: object[]; n: [number, number] }>();
  listOf(id: string): object[] { const mix = SCHEMA.mixes?.[id]; if (!mix) return this.lists.get(id) ?? []; let m = this.mixed.get(id); if (!m) this.mixed.set(id, m = { a: [], n: [0, 0] }); for (let k = 0; k < 2; k++) { const L = this.listOf(mix[k]); while (m.n[k] < L.length) { m.a[2 * m.n[k] + k] = L[m.n[k]]; m.n[k]++; } } return m.a; }
  serialOf(o: object): number { let s = this.serial.get(o); if (s === undefined) { s = this.rustDeals ? this.ownSerial++ : this.nextSerial++; this.serial.set(o, s); this.serials.set(o, s); } return s; } }
/** what `idOf` answers for an object of the wrong table: past every table */
const PAST_ANY = 0x7fffffff;
const BOOKS = new WeakMap<object, Book>();
// ---- A WEAK TABLE LISTS THE KEYS OF THE GAME BEING WALKED. `liveKeys()` is every key alive in the process, whichever game's, so a second game of a process would be walked with the first game's men, routes and players as rows of its own, a table of one thing's own row would give the first game's (`K[0]`), and an apply that puts a table's keys in the Rust game's order would strike every other game's off the list. Every walk of a game by the bridge goes through `forGame`: while it runs a table lists only that game's keys, the keys a walk from the game reaches. A start finds them (`startExport`): the game is exported with every table listing only the keys the walk has numbered so far (and the game and its views), again until an export numbers no key it had left out; what the book then holds is the game's, the book is put back as it was, and the start's own export lists exactly those. After a start: those keys, and a key first seen since (a thing made in play) unless a start left it out or it is in another game's book. Outside the walks `liveKeys()` is as it was
type KeyMode = { kind: 'reach' } | { kind: 'exact' | 'own'; mine: WeakSet<object>; not?: WeakSet<object> };
let CUR: any = null, MODE: KeyMode | null = null, DROPPED: Set<object> | null = null;
const MODE_OF = new WeakMap<object, KeyMode>(), OWNER = new WeakMap<object, object>(), WALKED = new Set<any>(), KEYED = new WeakSet<object>();
const inBook = (bk: any, k: object): boolean => { if (!bk) return false; for (const M of bk.ids.values() as Iterable<Map<object, number>>) if (M.has(k)) return true; return bk.serial.has(k); };
const foreign = (k: any): boolean => { const M = MODE; if (!M || k === CUR || rawKey(k) === CUR) return false; if (M.kind === 'reach') { if (inBook(BOOKS.get(CUR), k)) return false; DROPPED?.add(k); return true; } if (M.mine.has(k)) return false; if (M.kind === 'exact') return true; if (M.not!.has(k)) return true; const o = OWNER.get(k); return o !== undefined && o !== CUR; };
let keysHeld = false;
const holdKeys = () => { if (!keysHeld) { keysHeld = true; for (const P of [SideMap.prototype, SideSet.prototype] as any[]) { const was = P.liveKeys; P.liveKeys = function (this: any, out: object[] = []) { const n0 = out.length, K: object[] = was.call(this, out); if (!MODE) return K; let n = n0; for (let i = n0; i < K.length; i++) if (!foreign(K[i])) K[n++] = K[i]; K.length = n; return K; }; } }
  // (a list of keys written over while a game is walked keeps the other games' keys in front of it)
  for (const t of sideRegistry().tables as any[]) { const K = t.sideKeys; if (!K || KEYED.has(K)) continue; KEYED.add(K); let refs: WeakRef<object>[] = K.refs; Object.defineProperty(K, 'refs', { configurable: true, enumerable: true, get: () => refs, set: (v: WeakRef<object>[]) => { if (!MODE) { refs = v; return; } const keep = refs.filter(r => { const k = r.deref(); return k !== undefined && foreign(k); }); refs = keep.length ? keep.concat(v) : v; } }); } };
/** a walk of `g`: its tables list its keys */
export function forGame<T>(g: any, f: () => T): T { const c = CUR, m = MODE; CUR = g; MODE = MODE_OF.get(g) ?? null; WALKED.add(g); try { return f(); } finally { CUR = c; MODE = m; } }
/** every object in a game's book is that game's, for the other games of the process (the first game to number it) */
function owned(g: any, bk: any): void { const put = (k: object) => { if (!OWNER.has(k)) OWNER.set(k, g); }; put(g); if (!bk) return; for (const M of bk.ids.values() as Iterable<Map<object, number>>) for (const k of M.keys()) put(k); for (const k of (bk.serials as Map<object, number>).keys()) put(k); }
/** A START'S EXPORT, with every weak table listing this game's keys and no other game's. `say` is told what was left out */
export function startExport(g: any, knobs: Record<string, unknown>, say?: (s: string) => void): Uint8Array { holdKeys(); for (const o of WALKED) if (o !== g) owned(o, BOOKS.get(o)); WALKED.clear();
  const FIELDS = ['ids', 'lists', 'first', 'loose', 'serials', 'serial', 'nextSerial', 'rustDeals', 'ownSerial'] as const; const fresh = (bk: any) => { bk.ids = new Map(); bk.lists = new Map(); bk.first = new Map(); bk.loose = new Map(); bk.serials = new Map(); bk.serial = new WeakMap(); bk.nextSerial = 1; bk.rustDeals = false; bk.ownSerial = 0x40000000; };
  const bk0: any = BOOKS.get(g), saved: any = bk0 ? Object.fromEntries(FIELDS.map(f => [f, bk0[f]])) : null; if (bk0) fresh(bk0); MODE_OF.set(g, { kind: 'reach' });
  let out: Uint8Array, left = new Set<object>(), passes = 0, found = 0;
  for (;;) { DROPPED = new Set(); out = exportGame(g, knobs); left = DROPPED; DROPPED = null; passes++; const bk = BOOKS.get(g); let again = 0; for (const k of left) if (inBook(bk, k)) again++; found += again; if (!again) break; if (passes >= 8) { say?.(`A START'S REACH DID NOT SETTLE in 8 exports (${again} keys still found late)`); break; } }
  const bk: any = BOOKS.get(g), mine = new WeakSet<object>(), not = new WeakSet<object>(); let nNot = 0; for (const M of bk.ids.values() as Iterable<Map<object, number>>) for (const k of M.keys()) mine.add(k); for (const k of (bk.serials as Map<object, number>).keys()) mine.add(k); for (const k of left) if (!mine.has(k)) { not.add(k); nNot++; }
  // (the book as it was: a start after the first keeps every number it has given; the first start whose one export left out no key it then met IS the export)
  if (saved) Object.assign(bk, saved); else if (passes > 1) fresh(bk);
  if (saved || passes > 1) { MODE_OF.set(g, { kind: 'exact', mine }); out = exportGame(g, knobs); }
  MODE_OF.set(g, { kind: 'own', mine, not }); owned(g, BOOKS.get(g));
  if (nNot || passes > 1) say?.(`a start's export: ${nNot} keys of other games' rows left out of the weak tables; ${passes} exports to find this game's own (${found} keys met late)`);
  return out!; }
/** A FIELD RUST OWNS AND TYPESCRIPT CANNOT HOLD (schema `echo`): the apply keeps its bytes beside the object (import.ts), and the walk writes them back. So a re-export after an apply is Rust's own bytes there, and the walk's expression is only for a game that has never been applied onto */
export const ECHO = new WeakMap<object, Map<string, Uint8Array>>();
export const hasEcho = (o: object, q: string) => !!ECHO.get(o)?.has(q);
/** A LENS'S CUT (cmd.ts:191 takeCut): a new `desired` array is a cut. Rust counts them (FollowCamera.desired_serial); here the count stands beside the lens and moves when the array is another one */
export const LENS = new WeakMap<object, { arr: unknown; n: number }>();
const lensSerial = (c: any): number => { let r = LENS.get(c); if (!r) LENS.set(c, r = { arr: c.desired, n: 1 }); else if (r.arr !== c.desired) { r.arr = c.desired; r.n = (r.n + 1) >>> 0; } return r.n; };
/** the seat's mark as a number of that count: the array's own if it has taken it, else one behind */
const seatMark = (s: any): number => { const n = lensSerial(s.cam); return s.mark === s.cam.desired ? n : (n - 1) >>> 0; };

class Out { buf = new Uint8Array(1 << 16); dv = new DataView(this.buf.buffer); n = 0;
  room(k: number) { if (this.n + k <= this.buf.length) return; let c = this.buf.length * 2; while (c < this.n + k) c *= 2; const b = new Uint8Array(c); b.set(this.buf.subarray(0, this.n)); this.buf = b; this.dv = new DataView(b.buffer); }
  f64(x: number) { this.room(8); this.dv.setFloat64(this.n, x, true); this.n += 8; } f32(x: number) { this.room(4); this.dv.setFloat32(this.n, x, true); this.n += 4; }
  u8(x: number) { this.room(1); this.buf[this.n++] = x; } u16(x: number) { this.room(2); this.dv.setUint16(this.n, x, true); this.n += 2; } u32(x: number) { this.room(4); this.dv.setUint32(this.n, x >>> 0, true); this.n += 4; } i32(x: number) { this.room(4); this.dv.setInt32(this.n, x | 0, true); this.n += 4; }
  bytes(b: Uint8Array) { this.room(b.length); this.buf.set(b, this.n); this.n += b.length; } view() { return this.buf.subarray(0, this.n); } }
const UTF8 = new TextEncoder();
const BOXIX = new WeakMap<object, Map<object, number>>();

// ---- report: what did not fit, by the field it was in
export type Report = Map<string, { n: number; eg: string }>;
let REP: Report = new Map(); let AT = ''; let ROWAT = '';
// A CLASS IS TOLD BY ITS ENTRY IN THE CLASS TABLE (classes.ts), never by its own name: a minified bundle renames them all. What is not in the table goes by its name: the language's own, which keep theirs, or something no word of the schema names
const CLASS_WORDS = new Map<object, string>();
export function nameClasses(rows: [{ prototype: object } | undefined, string][]): void { for (const [c, name] of rows) if (c?.prototype) CLASS_WORDS.set(c.prototype, name); }
export const classWord = (o: any): string | undefined => { const P = Object.getPrototypeOf(o); return P === null ? undefined : CLASS_WORDS.get(P) ?? o.constructor?.name; };
export const isGameSide = (v: any): boolean => v instanceof GameSideMap || v instanceof GameSideSet;
const miss = (why: string, v?: unknown) => { const key = `${AT}: ${why}${ROWAT ? ` (a row first reached at ${ROWAT})` : ''}`; const r = REP.get(key); if (r) r.n++; else { let eg = ''; try { eg = typeof v === 'object' ? (v === null ? 'null' : (v as object).constructor?.name ?? 'object') : String(v); } catch { eg = '?'; } REP.set(key, { n: 1, eg: eg.slice(0, 40) }); } };
export const report = (): Report => REP;
/** bytes by field, where a field held more than 4 KB (a parent counts what its fields hold) */
export const SIZES = new Map<string, number>();
/** every table's rows by the class they are, and the field that first reached one */
export function rowsReport(g: object): Map<string, { n: number; first: string }> { const bk = BOOKS.get(g)!, R = new Map<string, { n: number; first: string }>(); for (const t of SCHEMA.tables) for (const r of bk.listOf(t.id)) { const k = `${t.id} ${(r as object).constructor?.name}`; const e = R.get(k); if (e) e.n++; else R.set(k, { n: 1, first: bk.first.get(r) ?? '' }); } return R; }

function zero(n: Node, o: Out): void { switch (n.k) { case 'f64': o.f64(0); return; case 'f32': o.f32(0); return; case 'bool': case 'u8': case 'i8': o.u8(0); return; case 'u16': case 'i16': o.u16(0); return; case 'u32': case 'i32': case 'usize': case 'id': o.u32(0); return; case 'u64': case 'i64': o.f64(0); return; case 'str': case 'vec': case 'map': case 'set': o.u32(0); return; case 'opt': o.u8(0); return; case 'unit': return; case 'arr': for (let i = 0; i < n.n; i++) zero(n.of, o); return; case 'tup': for (const x of n.of) zero(x, o); return; case 'type': { const T = SCHEMA.types[n.path]; if (T.k === 'struct') for (const f of T.fields) zero(parse(f.ty), o); else if (T.k === 'enum') o.u16(0); else if (T.k === 'wordEnum') { o.u16(0); for (const f of T.arms[0].fields) zero(parse(f.ty), o); } else if (T.k === 'data' || T.k === 'handEnum') { o.u16(0); const a = T.k === 'data' ? T.vars[0]?.ty : (T.arms[0].match(/\((.*)\)/)?.[1]); if (a) zero(parse(a), o); } return; } } }

/** a pose outside the pool has this place and its number in `loose` (rust: Pose::LOOSE) */
export const LOOSE = 0xffff0000;
/** every field and map key that held an object's identity serial on this export: serial-fields.json, schema.serial */
export const SERIALS = new Set<string>();
let G: any; let BK: Book; let B: any;
const isTyped = (v: any) => ArrayBuffer.isView(v) && !(v instanceof DataView);
/** a weak table's row of a player who is no longer in the game (the man before a mission's fresh deal) is that man's, and is not this game's state */
const isWeak = (v: any) => typeof v?.liveKeys === 'function', seated = (k: unknown): boolean => { const S = CUR?.seats; if (!S) return true; const r = rawKey(k as object); if (r === rawKey(CUR)) return true;   /* (a seat's table, keyed by the Game: in a game of one seat that is its view. wKey writes it as the first seat's man, the walk having the first seat standing) */ return rawKey(CUR.player) === r || S.some((s: any) => rawKey(s.view?.player) === r); };
function keysOf(v: any): { keys: unknown[]; get: (k: any) => unknown } | null {
  if (v instanceof Map) return { keys: [...v.keys()], get: k => v.get(k) };
  if (v instanceof Set) return { keys: [...v], get: k => k };
  if (v && typeof v.liveKeys === 'function') return { keys: v.liveKeys(), get: k => v.get ? v.get(k) : k };
  if (v && typeof v === 'object' && !Array.isArray(v)) return { keys: Object.keys(v), get: k => v[k] };
  return null;
}
function wKey(K: Node, k: unknown, o: Out): void {
  if (K.k === 'u32' && k && typeof k === 'object') { SERIALS.add(`${AT}{}`); o.u32(BK.serialOf(k)); return; }
  if (K.k === 'id' && K.id === 'PlayerId' && k && typeof k === 'object' && 'player' in (k as object) && (k as any).player && 'seats' in (k as object)) { wr(K, (k as any).player, o); return; }   // a table of a player's, keyed by his seat's view
  if (K.k === 'f64' && typeof k === 'string') { wr(K, Number(k), o); return; }
  wr(K, k, o);
}
const isOnceKeyed = (v: any) => { const K = v.liveKeys(); return K.length > 0 && SCHEMA.onceKeys.includes(classWord(K[0])); };
function wr(n: Node, v: any, o: Out): void {
  if (typeof v === 'number' && n.k === 'f64') { o.f64(v); return; }
  // a table keyed by the game: the game's own row, never a cast's or a view's
  if (v && typeof v.liveKeys === 'function' && (isGameSide(v) || (!(n.k === 'map' && n.K.k === 'id' && n.K.id === 'PlayerId') && v.liveKeys().includes(G)))) { const R0 = rawKey(G); v = v.get ? v.get(R0) : v.has(R0); if (v === undefined && (n.k === 'map' || n.k === 'set' || n.k === 'vec')) { o.u32(0); return; } }
  // a table of one thing's own row, keyed by that thing (a nav, a collision world) where the Rust keeps the one row
  if (v && typeof v.liveKeys === 'function' && n.k !== 'opt' && isOnceKeyed(v)) { const K = v.liveKeys(); v = K.length ? (v.get ? v.get(K[0]) : true) : undefined; if (v === undefined && (n.k === 'map' || n.k === 'set' || n.k === 'vec')) { o.u32(0); return; } }
  switch (n.k) {
    case 'f64': if (typeof v === 'number') o.f64(v); else { if (v !== undefined && typeof v === 'boolean') { o.f64(v ? 1 : 0); miss('a bool for a number', v); return; } miss('not a number', v); o.f64(0); } return;
    case 'f32': if (typeof v === 'number') o.f32(v); else { miss('not a number', v); o.f32(0); } return;
    case 'bool': if (v && typeof v.liveKeys === 'function') { o.u8(v.has(G) ? 1 : 0); return; } o.u8(v ? 1 : 0); return;
    case 'u8': o.u8(v | 0); return; case 'i8': o.u8(v & 255); return; case 'u16': o.u16(v | 0); return; case 'i16': o.u16(v & 65535); return;
    case 'i32': if (typeof v !== 'number') miss('not a number', v); o.i32(v | 0); return;
    case 'u32': if (v && typeof v === 'object') { SERIALS.add(AT); o.u32(BK.serialOf(v)); return; } if (typeof v !== 'number') { miss('not a number', v); o.u32(0); return; } o.u32(v); return;
    case 'usize': if (typeof v !== 'number' || !Number.isInteger(v)) { miss('not a whole number', v); o.u32(0); return; } o.u32(v); return;
    case 'u64': case 'i64': o.room(8); o.dv.setBigInt64(o.n, BigInt(typeof v === 'bigint' ? v : Math.trunc(v ?? 0)), true); o.n += 8; return;
    case 'str': { if (typeof v !== 'string') { miss('not a string', v); o.u32(0); return; } const b = UTF8.encode(v); o.u32(b.length); o.bytes(b); return; }
    case 'unit': return;
    case 'opt': if (v && typeof v.liveKeys === 'function' && v.liveKeys().length === 0) v = undefined;   // (a table with no row, a per-game table this game has none in: nothing, not an empty one)
      if (v && typeof v.liveKeys === 'function' && !(n.of.k === 'map' && n.of.K.k === 'id') && !(n.of.k === 'set')) { const K = v.liveKeys(); v = K.length ? (v.get ? v.get(K.includes(G) ? G : K[0]) : true) : undefined; } if (v === undefined || (v === null && n.of.k !== 'opt')) { o.u8(0); return; } o.u8(1); wr(n.of, v, o); return;   // (Option<Option<T>>: undefined is None, null is Some(None): two things in kitanim.ts, doortools.ts)
    case 'vec': { if (v === null || v === undefined) { miss('nothing for a list', v); o.u32(0); return; } if (!Array.isArray(v) && !isTyped(v)) { miss('not a list', v); o.u32(0); return; } const len = v.length; o.u32(len); for (let i = 0; i < len; i++) { const x = v[i]; if (x === undefined && !(i in v)) { zero(n.of, o); continue; } wr(n.of, x, o); } return; }
    case 'arr': { if (!v || typeof v.length !== 'number') { miss('not a fixed list', v); zero(n, o); return; } if (v.length !== n.n) miss(`a list of ${v.length} for ${n.n}`, v.length); for (let i = 0; i < n.n; i++) wr(n.of, v[i], o); return; }
    case 'tup': { if (!Array.isArray(v)) { miss('not a tuple', v); zero(n, o); return; } n.of.forEach((x: Node, i: number) => wr(x, v[i], o)); return; }
    case 'map': { const E = keysOf(v); if (!E) { miss('not a map', v); o.u32(0); return; } const K = n.K.k === 'id' && n.K.id === 'PlayerId' && isWeak(v) ? E.keys.filter(seated) : E.keys; o.u32(K.length); for (const k of K) { wKey(n.K, k, o); wr(n.V, E.get(k), o); } return; }
    case 'set': { const E = keysOf(v); if (!E) { miss('not a set', v); o.u32(0); return; } o.u32(E.keys.length); for (const k of E.keys) wKey(n.K, k, o); return; }
    case 'id': { if (v && typeof v === 'object') { const e = SCHEMA.idExpr[n.id]; if (e) { const r = compiled(e)(v, G, B); if (typeof r === 'number') { if (r >= (BK.known?.get(n.id) ?? Infinity) && r !== 0xffffffff) { PAST.n++; PAST.id = n.id; } o.u32(r); return; } } const i = BK.idOf(n.id, v); if (BK.past(n.id, i)) { PAST.n++; PAST.id = n.id; } o.u32(i); return; } if (typeof v === 'number') { o.u32(v); return; } throw new Error(`export: ${AT}: no object for ${n.id} (found ${String(v)})`); }
    case 'type': return wType(n, v, o);
  }
  throw new Error(`bridge: no writer for ${n.k}`);
}
let DEPTH = 0;
function wType(n: Node, v: any, o: Out): void {
  const T = SCHEMA.types[n.path];
  if (T.k === 'struct') {
    if (v === null || v === undefined || typeof v !== 'object') { miss(`nothing for ${T.name}`, v); zero(n, o); return; }
    if (++DEPTH > 64) { miss('walked 64 deep: a cycle of values', T.name); DEPTH--; zero(n, o); return; }
    const was = AT; for (const f of T.fields) { const q: string = f.q ??= `${T.mod}::${T.name}.${f.rust}`; AT = q; if (f.echo) { const e = ECHO.get(v)?.get(q); if (e) { o.bytes(e); continue; } } const x = f.expr !== undefined ? (f.fn ??= compiled(f.expr))(v, G, B) : (f.ts !== undefined ? v[f.ts] : undefined); const n0 = o.n; wr(f.N ??= parse(f.ty), x, o); const d = o.n - n0; if (d > 4096) SIZES.set(q, (SIZES.get(q) ?? 0) + d); } AT = was; DEPTH--; return;
  }
  if (T.k === 'enum') { const i = T.words.indexOf(v); if (i < 0) { miss(`no word ${String(v).slice(0, 20)} of ${T.name}`, v); o.u16(0); return; } o.u16(i); return; }
  if (T.k === 'handEnum') { const to = compiled(T.to ?? 'null')(v, G, B); if (T.words) { const i = T.words.indexOf(v); if (i < 0) { miss(`no word of ${T.name}`, v); o.u16(0); return; } o.u16(i); return; } if (!to) { miss(`no arm of ${T.name}`, v); zero(n, o); return; } o.u16(to.arm); if (to.ty) wr(parse(to.ty), to.v, o); return; }
  if (T.k === 'wordEnum') { if (v === null || v === undefined || typeof v !== 'object') { miss(`nothing for ${T.name}`, v); zero(n, o); return; } let i = T.arms.findIndex((x: any) => x.word === v[T.word]); if (i < 0) { i = T.arms.findIndex((x: any) => x.tag === T.other); miss(`no arm of ${T.name} for the word ${String(v[T.word])}`, v); } o.u16(i); for (const f of T.arms[i].fields) { const x = v[f.ts]; wr(parse(f.ty), x === undefined && f.ty === 'bool' ? false : x, o); } return; }
  // a union: which arm
  const vars = T.vars as any[]; let arm = -1;
  if (typeof v === 'string') { arm = vars.findIndex(a => a.str === v); if (arm < 0) arm = vars.findIndex(a => a.ty && (a.ty === 'String' || (SCHEMA.types[a.ty]?.k === 'enum' && SCHEMA.types[a.ty].words.includes(v)))); }
  else if (typeof v === 'number') arm = vars.findIndex(a => a.ty === 'f64');
  else if (typeof v === 'boolean') arm = vars.findIndex(a => a.ty === 'bool');
  else if (isTyped(v) && vars.some(a => a.ty && /^Vec<f32>/.test(a.ty))) arm = vars.findIndex(a => a.ty && /^Vec<f32>/.test(a.ty));   // (a Float32Array is the singles' arm where there is one)
  else if (Array.isArray(v) || isTyped(v)) arm = vars.findIndex(a => a.ty && /^(Vec<|\[|\(|V3|Quat)/.test(a.ty));
  else if (v && typeof v === 'object') { const kw = v.kind ?? v.type; if (typeof kw === 'string') arm = vars.findIndex(a => a.kw === kw); if (arm < 0) arm = vars.findIndex(a => a.sn && a.sn === classWord(v)); if (arm < 0) { let best = -1; vars.forEach((a, i) => { const t = a.ty && SCHEMA.types[a.ty]; if (t?.k !== 'struct') return; let ok = true, has = 0; for (const f of t.fields as any[]) { if (!f.ts || f.expr !== undefined) continue; const there = f.ts in v; if (there) has++; else if (!/^Option</.test(f.ty)) ok = false; const E = SCHEMA.types[f.ty]; if (there && E?.k === 'enum' && !E.words.includes(v[f.ts])) ok = false; } if (ok && has > best) { best = has; arm = i; } }); } if (arm < 0) arm = vars.findIndex(a => a.ty && /^JsMap</.test(a.ty));   /* an arm is the struct whose required fields are all here and whose words all fit (a literal `k` says which); a plain object with no such arm is the map's */ if (arm < 0) arm = vars.findIndex(a => a.ty && IDS.has(a.ty)); }
  if (arm < 0) { miss(`no arm of ${T.name}`, v); zero(n, o); return; }
  o.u16(arm); if (vars[arm].ty) wr(parse(vars[arm].ty), v, o);
}

// ---- helpers for the hand's expressions
function helpers(g: any, knobs: Record<string, unknown>) {
  const reg = sideRegistry(); const side = new Map<string, unknown>();
  for (const t of reg.tables as any[]) side.set(t.sideName, t); for (const r of reg.roots) side.set(r.name, r.obj);
  const vars = new Map(reg.vars.map(v => [v.name, v]));
  const seen = new Set<string>(); const ragOwner = (rd: any) => (BK.listOf('CharId') as any[]).find(c => c.rd === rd);
  // the animator's array: the body's own (`anim.posIn = this.pos`, three NaNs on the wire, rust: character::POS_IN_IS_POS), or one the body has since left behind
  const NO_POS = [NaN, NaN, NaN], posIn = (a: any) => (BK.listOf('CharId') as any[]).some(c => c.pos === a.posIn) ? NO_POS : a.posIn;
  return { classWord, ragOwner, posIn, keysOf: (o: any, K: string[]) => K.map(k => o[k]), nonZero: (o: any) => new Map(Object.entries(o).filter(([, x]) => x)), lensSerial, seatMark, knob: (n: string) => knobs[n], side: (n: string) => { const V = vars.get(n); if (V) return V.get(); if (side.has(n)) { const t: any = side.get(n); return t; } if (!seen.has(n)) { seen.add(n); miss(`not registered: ${n}`); } return undefined; }, top: (file: string, name: string, sideName: string) => { const T = (globalThis as any).__tpsTop?.[file]; if (T && name in T) return T[name]; const V = vars.get(sideName); if (V) return V.get(); if (side.has(sideName)) return side.get(sideName); if (!seen.has(sideName)) { seen.add(sideName); miss(`no window on ${sideName}: run with --preload tools/blitz/expose.ts`); } return undefined; }, whoOf: (c: any) => { for (const gd of [...(g.guards ?? []), ...BK.listOf('GuardId')] as any[]) if (gd.char === c) return { arm: 0, ty: 'GuardId', v: gd }; for (const s of g.seats ?? []) { const p = s.own?.player; if (p?.char === c) return { arm: 1, ty: 'PlayerId', v: p }; } if (g.player?.char === c) return { arm: 1, ty: 'PlayerId', v: g.player }; for (const p of BK.listOf('PlayerId') as any[]) if (p.char === c) return { arm: 1, ty: 'PlayerId', v: p }; return null; }, coversOf: (gd: any) => { const v = gd.covers; if (!v) return null; const D = g.fight?.defs?.find?.((d: any) => d.gd === gd); return D && D.pt === v ? { arm: 0 } : { arm: 1, ty: 'V3', v }; }, boxIx: (b: object) => { let M = BOXIX.get(g.col); if (!M) { M = new Map(g.col.boxes.map((x: object, i: number) => [x, i])); BOXIX.set(g.col, M!); } return M!.get(b); }, mapVals: (m: any, f: (r: unknown) => unknown) => { const out = new Map(); for (const k of m.liveKeys ? m.liveKeys() : m.keys()) out.set(k, f(m.get(k))); return out; }, ownPose: (p: any) => { let i = BK.loose.get(p); if (i === undefined) { i = BK.loose.size; BK.loose.set(p, i); } return LOOSE + i; }, serialOf: (o: object) => { SERIALS.add(AT); return BK.serialOf(o); }, idOf: (id: string, o: object) => BK.idOf(id, o), raw: rawKey, g };
}

export const bookOf = (g: object) => BOOKS.get(g);
export const compiledExpr = (e: string) => compiled(e);
/** the walk's helpers for a game, and a module variable or root by its registered name */
export function helpersOf(g: any, knobs: Record<string, unknown>) { G = g; BK = BOOKS.get(g)!; const b = helpers(g, knobs); const reg = sideRegistry(); const vars = new Map(reg.vars.map(v => [v.name, v])); const roots = new Map<string, unknown>(); for (const t of reg.tables as any[]) roots.set(t.sideName, t); for (const r of reg.roots) roots.set(r.name, r.obj); return { ...b, varOf: (n: string) => vars.get(n), rootOf: (n: string) => roots.get(n) }; }
/** a book's seeds, as an export would take them (a row its list has since gained has an ID before anything else asks) */
export function seedBook(g: any, knobs: Record<string, unknown>) { helpersOf(g, knobs); B = helpers(g, knobs); }
/** the bytes of values of the given schema types, written by the walk's own writers against the game's book (the book must exist: one exportGame first) */
/** one value's bytes at a time, for a walk that writes many: the game's book and helpers stand for the walk (scan.ts). The answer holds only until the next call */
export function leafBytes(g: any, knobs: Record<string, unknown>): (n: Node, v: unknown) => Uint8Array { G = g; BK = BOOKS.get(g)!; B = helpers(g, knobs); const o = new Out(); return (n, v) => { o.n = 0; wr(n, v, o); return o.view().slice(); }; }
export function exportPart(g: any, knobs: Record<string, unknown>, parts: [string | Node, any][]): Uint8Array { return forGame(g, () => { G = g; BK = BOOKS.get(g)!; B = helpers(g, knobs); const o = new Out(); for (const [ty, v] of parts) wr(typeof ty === 'string' ? parse(ty) : ty, v, o); return o.view().slice(); }); }
/** WHICH OF THE HALF'S CAMERA ROWS AND INPUT ROWS ARE THE HOST'S OWN (game.ts hostCam / hostInput, handed at the stand), as the text of `hostrows.txt`:
 *  their places in the book's two lists, the order the half lists them in. -1 = the game was handed none. Read by half-grow.ts. */
export function hostRowsText(g: any): string { const bk: any = BOOKS.get(g)!; return `cam ${bk.listOf('CameraId').indexOf(g.hostCam)}\ninput ${bk.listOf('InputId').indexOf(g.hostInput)}\n`; }
/** WHAT THE HOST HANDS THE RUST GAME BEFORE EACH STEP, in about 2 KB (rust: io::take_host_half): each seat's record in seat order (`cmds`, as `Game.update` makes them), then every camera and Input row, then each seat's mark (the cut the host last took). The book must exist. */
export function hostHalfParts(g: any, cmds: readonly any[]): [string, any][] { const bk = BOOKS.get(g)!, P: [string, any][] = [['u32', cmds.length]]; for (const c of cmds) P.push(['crate::cmd::PlayerCmd', c]);
  for (const [id, ty] of [['CameraId', 'crate::camera::FollowCamera'], ['InputId', 'crate::input::Input']]) { const L: any[] = bk.listOf(id); P.push(['u32', L.length]); for (const row of L) P.push([ty, row]); }
  { const L: any[] = bk.listOf('SeatId'); P.push(['u32', L.length]); for (const s of L) P.push(['u32', seatMark(s)]); }
  { const E = g.engine, Pt = E?.particles; P.push(['u32', (Pt ? 1 : 0) | (E?.reach ? 2 : 0) | (Pt?.collide ? 4 : 0) | (Pt?.sparkBounce ? 8 : 0)]); }
  return P; }
export const hostHalf = (g: any, knobs: Record<string, unknown>, cmds: readonly any[]): Uint8Array => exportPart(g, knobs, hostHalfParts(g, cmds));
/** a field of the game by its Rust path (['m', 'dialogue', 'room_rects']): its schema type and its value now */
export function fieldAt(g: any, knobs: Record<string, unknown>, path: string[]): { ty: string; v: any } { G = g; BK = BOOKS.get(g)!; B = helpers(g, knobs); let ty: string = SCHEMA.root, v: any = g;
  for (const p of path) { const T = SCHEMA.types[ty.replace(/^alloc::boxed::Box<(.*)>$/, '$1')]; if (!T || T.k !== 'struct') throw new Error('fieldAt: ' + ty + ' is no struct'); const f = T.fields.find((x: any) => x.rust === p); if (!f) throw new Error('fieldAt: no ' + p + ' in ' + ty);
    v = f.expr !== undefined ? compiled(f.expr)(v, G, B) : (f.ts !== undefined ? v[f.ts] : undefined); ty = f.ty; }
  return { ty, v }; }
/** the place in the list of seats of the seat this host shows (the Rust game is told it: `face_show_seat`) */
export function shownSeat(g: any): number { const i = g.seats?.indexOf?.(g.localSeat) ?? 0; return i > 0 ? i : 0; }
/** A GAME'S ROWS ARE NUMBERED THE SAME ON EVERY MACHINE. The walk meets the rows of the seat the accessors answer first, so it walks with the first seat standing, whichever seat this host shows */
export function exportGame(g: any, knobs: Record<string, unknown>): Uint8Array { return forGame(g, () => { if (!(g.seats?.length > 1)) return exportGameNow(g, knobs); const shown = g.localSeat; g.localSeat = g.seats[0]; try { return exportGameNow(g, knobs); } finally { g.localSeat = shown; } }); }
function exportGameNow(g: any, knobs: Record<string, unknown>): Uint8Array {
  G = g; let bk = BOOKS.get(g); if (!bk) { bk = new Book(g); BOOKS.set(g, bk); } BK = bk; REP = new Map(); AT = ''; B = helpers(g, knobs);
  for (const [id, e] of Object.entries(SCHEMA.seeds as Record<string, string>)) { let L: any; try { L = compiled(e)(g, g, B); } catch { L = null; } if (L && Symbol.iterator in Object(L)) for (const x of L) if (x && typeof x === 'object') BK.idOf(id, x); else miss(`a seed of ${id} not a list`); }
  const main = new Out(); wType({ k: 'type', path: SCHEMA.root }, g, main);
  // tables: a row can reach a new thing of another table, so go round until none grows
  const buf = new Map<string, Out>(), done = new Map<string, number>(); for (const t of SCHEMA.tables) { buf.set(t.id, new Out()); done.set(t.id, 0); }
  for (let grew = true; grew;) { grew = false; for (const t of SCHEMA.tables) { const L = BK.listOf(t.id); let i = done.get(t.id)!; const o = buf.get(t.id)!; const N = parse(t.ty); while (i < L.length) { AT = `${t.id}[row]`; ROWAT = BK.first.get(L[i]) ?? ''; wr(N, L[i], o); i++; grew = true; } ROWAT = ''; done.set(t.id, i); } }
  for (const t of SCHEMA.tables) { const b = buf.get(t.id)!; if (b.n > 4096) SIZES.set(`table ${t.table}`, b.n); }
  // an order row the import has no closures for (rust's io::deal_closures): it would stand at its blank there
  // a beat under way whose place Rust never wrote: rust's import refuses this state by name (io::refuse_mid_script), because a generator's place cannot be read
  for (const r of BK.listOf('DirectorLiveBeatId') as any[]) if (r.state === 'running' && !hasEcho(r, 'director::LiveBeat.it')) { AT = 'director::LiveBeat'; miss(`the beat ${r.name} is running: this state cannot be imported`, r.name); }
  const out = new Out(); out.bytes(main.view());
  for (const t of SCHEMA.tables) { const b = buf.get(t.id)!; out.u32(done.get(t.id)!); out.bytes(b.view()); }
  out.u32(BK.nextSerial);
  // the poses no pool holds, last, because walking anything can meet one: nodes, then the floats (rust: Game.loose_poses, read by no comparison)
  out.u32(BK.loose.size); for (const p of BK.loose.keys() as Iterable<any>) { out.u32(p.numNodes); const c: Float32Array = p.cols; out.u32(c.length); for (let i = 0; i < c.length; i++) out.f32(c[i]); }
  return out.view().slice();
}
