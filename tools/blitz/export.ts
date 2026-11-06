// THE STATE BRIDGE, THE TYPESCRIPT'S SIDE: a live Game as bytes, walked by bridge/schema.json (written by the skeleton's program from the same tables as rust/game's
// types), in the order of types.rs's fields. rust/game/src/io.rs walks the same order. Numbers are raw doubles (f32 where the field is one), a bool a byte, an Option a
// tag byte, a string its length and utf-8, a Vec / JsMap / JsSet a count and its items in the TypeScript's own order, a word a u16, a thing with an ID a u32.
// IDS: a table is first filled from the TypeScript's own list where there is one (`seeds`), then by first reach in the walk; the map from object to ID is kept per game,
// so two exports of one game agree. TABLES COME LAST because walking a row can reach a new thing of another table.
// exportGame(g) never writes to the game. What it could not carry is in `report()`: every kind of value that did not fit the schema's type, by field.
import { readFileSync } from 'node:fs';
import { sideRegistry, rawKey } from '../../src/math/side';

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
  else if (t === '()') n = { k: 'unit' };
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
class Book { ids = new Map<string, Map<object, number>>(); lists = new Map<string, object[]>(); serial = new WeakMap<object, number>(); nextSerial = 1; serials = new Map<object, number>();
  /** (for the importer: every object that has a serial) */ serialOfObjects() { return this.serials; }
  first = new Map<object, string>();
  /** poses that have no place in the pool, in the order first met: their floats go at the end of the bytes */
  loose = new Map<object, number>();
  idOf(id: string, o: object): number { let M = this.ids.get(id); if (!M) { M = new Map(); this.ids.set(id, M); this.lists.set(id, []); } let i = M.get(o); if (i === undefined) { i = M.size; M.set(o, i); this.lists.get(id)!.push(o); this.first.set(o, AT); } return i; }
  listOf(id: string): object[] { return this.lists.get(id) ?? []; }
  serialOf(o: object): number { let s = this.serial.get(o); if (s === undefined) { s = this.nextSerial++; this.serial.set(o, s); this.serials.set(o, s); } return s; } }
const BOOKS = new WeakMap<object, Book>();
/** A FIELD RUST OWNS AND TYPESCRIPT CANNOT HOLD (schema `echo`): the apply keeps its bytes beside the object (import.ts), and the walk writes them back. So a re-export after an apply is Rust's own bytes there, and the walk's expression is only for a game that has never been applied onto */
export const ECHO = new WeakMap<object, Map<string, { b: Uint8Array; p0: number; p1: number }>>();
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
const isOnceKeyed = (v: any) => { const K = v.liveKeys(); return K.length > 0 && SCHEMA.onceKeys.includes(K[0].constructor?.name); };
function wr(n: Node, v: any, o: Out): void {
  if (typeof v === 'number' && n.k === 'f64') { o.f64(v); return; }
  // a table keyed by the game: the game's own row, never a cast's or a view's
  if (v && typeof v.liveKeys === 'function' && (v.constructor.name.startsWith('GameSide') || (!(n.k === 'map' && n.K.k === 'id' && n.K.id === 'PlayerId') && v.liveKeys().includes(G)))) { const R0 = rawKey(G); v = v.get ? v.get(R0) : v.has(R0); if (v === undefined && (n.k === 'map' || n.k === 'set' || n.k === 'vec')) { o.u32(0); return; } }
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
    case 'map': { const E = keysOf(v); if (!E) { miss('not a map', v); o.u32(0); return; } o.u32(E.keys.length); for (const k of E.keys) { wKey(n.K, k, o); wr(n.V, E.get(k), o); } return; }
    case 'set': { const E = keysOf(v); if (!E) { miss('not a set', v); o.u32(0); return; } o.u32(E.keys.length); for (const k of E.keys) wKey(n.K, k, o); return; }
    case 'id': { if (v && typeof v === 'object') { const e = SCHEMA.idExpr[n.id]; if (e) { const r = compiled(e)(v, G, B); if (typeof r === 'number') { o.u32(r); return; } } o.u32(BK.idOf(n.id, v)); return; } if (typeof v === 'number') { o.u32(v); return; } miss(`nothing for ${n.id}`, v); o.u32(0xffffffff); return; }
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
    const was = AT; for (const f of T.fields) { const q: string = f.q ??= `${T.mod}::${T.name}.${f.rust}`; AT = q; if (f.echo) { const e = ECHO.get(v)?.get(q); if (e) { o.bytes(e.b.subarray(e.p0, e.p1)); continue; } } const x = f.expr !== undefined ? (f.fn ??= compiled(f.expr))(v, G, B) : (f.ts !== undefined ? v[f.ts] : undefined); const n0 = o.n; wr(f.N ??= parse(f.ty), x, o); const d = o.n - n0; if (d > 4096) SIZES.set(q, (SIZES.get(q) ?? 0) + d); } AT = was; DEPTH--; return;
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
  else if (v && typeof v === 'object') { const kw = v.kind ?? v.type; if (typeof kw === 'string') arm = vars.findIndex(a => a.kw === kw); if (arm < 0) arm = vars.findIndex(a => a.sn && a.sn === v.constructor?.name); if (arm < 0) { let best = -1; vars.forEach((a, i) => { const t = a.ty && SCHEMA.types[a.ty]; if (t?.k !== 'struct') return; let ok = true, has = 0; for (const f of t.fields as any[]) { if (!f.ts || f.expr !== undefined) continue; const there = f.ts in v; if (there) has++; else if (!/^Option</.test(f.ty)) ok = false; const E = SCHEMA.types[f.ty]; if (there && E?.k === 'enum' && !E.words.includes(v[f.ts])) ok = false; } if (ok && has > best) { best = has; arm = i; } }); } if (arm < 0) arm = vars.findIndex(a => a.ty && /^JsMap</.test(a.ty));   /* an arm is the struct whose required fields are all here and whose words all fit (a literal `k` says which); a plain object with no such arm is the map's */ if (arm < 0) arm = vars.findIndex(a => a.ty && IDS.has(a.ty)); }
  if (arm < 0) { miss(`no arm of ${T.name}`, v); zero(n, o); return; }
  o.u16(arm); if (vars[arm].ty) wr(parse(vars[arm].ty), v, o);
}

// ---- helpers for the hand's expressions
function helpers(g: any, knobs: Record<string, unknown>) {
  const reg = sideRegistry(); const side = new Map<string, unknown>();
  for (const t of reg.tables as any[]) side.set(t.sideName, t); for (const r of reg.roots) side.set(r.name, r.obj);
  const vars = new Map(reg.vars.map(v => [v.name, v]));
  const seen = new Set<string>(); const ragOwner = (rd: any) => (BK.listOf('CharId') as any[]).find(c => c.rd === rd);
  return { ragOwner, lensSerial, seatMark, knob: (n: string) => knobs[n], side: (n: string) => { const V = vars.get(n); if (V) return V.get(); if (side.has(n)) { const t: any = side.get(n); return t; } if (!seen.has(n)) { seen.add(n); miss(`not registered: ${n}`); } return undefined; }, top: (file: string, name: string, sideName: string) => { const T = (globalThis as any).__tpsTop?.[file]; if (T && name in T) return T[name]; const V = vars.get(sideName); if (V) return V.get(); if (side.has(sideName)) return side.get(sideName); if (!seen.has(sideName)) { seen.add(sideName); miss(`no window on ${sideName}: run with --preload tools/blitz/expose.ts`); } return undefined; }, whoOf: (c: any) => { for (const gd of [...(g.guards ?? []), ...BK.listOf('GuardId')] as any[]) if (gd.char === c) return { arm: 0, ty: 'GuardId', v: gd }; for (const s of g.seats ?? []) { const p = s.own?.player; if (p?.char === c) return { arm: 1, ty: 'PlayerId', v: p }; } if (g.player?.char === c) return { arm: 1, ty: 'PlayerId', v: g.player }; for (const p of BK.listOf('PlayerId') as any[]) if (p.char === c) return { arm: 1, ty: 'PlayerId', v: p }; return null; }, coversOf: (gd: any) => { const v = gd.covers; if (!v) return null; const D = g.fight?.defs?.find?.((d: any) => d.gd === gd); return D && D.pt === v ? { arm: 0 } : { arm: 1, ty: 'V3', v }; }, boxIx: (b: object) => { let M = BOXIX.get(g.col); if (!M) { M = new Map(g.col.boxes.map((x: object, i: number) => [x, i])); BOXIX.set(g.col, M!); } return M!.get(b); }, mapVals: (m: any, f: (r: unknown) => unknown) => { const out = new Map(); for (const k of m.liveKeys ? m.liveKeys() : m.keys()) out.set(k, f(m.get(k))); return out; }, ownPose: (p: any) => { let i = BK.loose.get(p); if (i === undefined) { i = BK.loose.size; BK.loose.set(p, i); } return LOOSE + i; }, serialOf: (o: object) => { SERIALS.add(AT); return BK.serialOf(o); }, idOf: (id: string, o: object) => BK.idOf(id, o), raw: rawKey, g };
}

export const bookOf = (g: object) => BOOKS.get(g);
export const compiledExpr = (e: string) => compiled(e);
/** the walk's helpers for a game, and a module variable or root by its registered name */
export function helpersOf(g: any, knobs: Record<string, unknown>) { G = g; BK = BOOKS.get(g)!; const b = helpers(g, knobs); const reg = sideRegistry(); const vars = new Map(reg.vars.map(v => [v.name, v])); const roots = new Map<string, unknown>(); for (const t of reg.tables as any[]) roots.set(t.sideName, t); for (const r of reg.roots) roots.set(r.name, r.obj); return { ...b, varOf: (n: string) => vars.get(n), rootOf: (n: string) => roots.get(n) }; }
/** a book's seeds, as an export would take them (a row its list has since gained has an ID before anything else asks) */
export function seedBook(g: any, knobs: Record<string, unknown>) { helpersOf(g, knobs); B = helpers(g, knobs); }
/** the bytes of values of the given schema types, written by the walk's own writers against the game's book (the book must exist: one exportGame first) */
export function exportPart(g: any, knobs: Record<string, unknown>, parts: [string, any][]): Uint8Array { G = g; BK = BOOKS.get(g)!; B = helpers(g, knobs); const o = new Out(); for (const [ty, v] of parts) wr(parse(ty), v, o); return o.view().slice(); }
/** WHAT THE HOST HANDS THE RUST GAME BEFORE EACH STEP, in about 2 KB (rust: io::take_host_half): each seat's record in seat order (`cmds`, as `Game.update` makes them), then every camera and Input row, then each seat's mark (the cut the host last took). The book must exist. */
export function hostHalfParts(g: any, cmds: readonly any[]): [string, any][] { const bk = BOOKS.get(g)!, P: [string, any][] = [['u32', cmds.length]]; for (const c of cmds) P.push(['crate::cmd::PlayerCmd', c]);
  for (const [id, ty] of [['CameraId', 'crate::camera::FollowCamera'], ['InputId', 'crate::input::Input']]) { const L: any[] = bk.listOf(id); P.push(['u32', L.length]); for (const row of L) P.push([ty, row]); }
  { const L: any[] = bk.listOf('SeatId'); P.push(['u32', L.length]); for (const s of L) P.push(['u32', seatMark(s)]); }
  return P; }
export const hostHalf = (g: any, knobs: Record<string, unknown>, cmds: readonly any[]): Uint8Array => exportPart(g, knobs, hostHalfParts(g, cmds));
/** a field of the game by its Rust path (['m', 'dialogue', 'room_rects']): its schema type and its value now */
export function fieldAt(g: any, knobs: Record<string, unknown>, path: string[]): { ty: string; v: any } { G = g; BK = BOOKS.get(g)!; B = helpers(g, knobs); let ty: string = SCHEMA.root, v: any = g;
  for (const p of path) { const T = SCHEMA.types[ty.replace(/^alloc::boxed::Box<(.*)>$/, '$1')]; if (!T || T.k !== 'struct') throw new Error('fieldAt: ' + ty + ' is no struct'); const f = T.fields.find((x: any) => x.rust === p); if (!f) throw new Error('fieldAt: no ' + p + ' in ' + ty);
    v = f.expr !== undefined ? compiled(f.expr)(v, G, B) : (f.ts !== undefined ? v[f.ts] : undefined); ty = f.ty; }
  return { ty, v }; }
export function exportGame(g: any, knobs: Record<string, unknown>): Uint8Array {
  G = g; let bk = BOOKS.get(g); if (!bk) { bk = new Book(); BOOKS.set(g, bk); } BK = bk; REP = new Map(); AT = ''; B = helpers(g, knobs);
  for (const [id, e] of Object.entries(SCHEMA.seeds as Record<string, string>)) { let L: any; try { L = compiled(e)(g, g, B); } catch { L = null; } if (L && Symbol.iterator in Object(L)) for (const x of L) if (x && typeof x === 'object') BK.idOf(id, x); else miss(`a seed of ${id} not a list`); }
  const main = new Out(); wType({ k: 'type', path: SCHEMA.root }, g, main);
  // tables: a row can reach a new thing of another table, so go round until none grows
  const buf = new Map<string, Out>(), done = new Map<string, number>(); for (const t of SCHEMA.tables) { buf.set(t.id, new Out()); done.set(t.id, 0); }
  for (let grew = true; grew;) { grew = false; for (const t of SCHEMA.tables) { const L = BK.listOf(t.id); let i = done.get(t.id)!; const o = buf.get(t.id)!; const N = parse(t.ty); while (i < L.length) { AT = `${t.id}[row]`; ROWAT = BK.first.get(L[i]) ?? ''; wr(N, L[i], o); i++; grew = true; } ROWAT = ''; done.set(t.id, i); } }
  for (const t of SCHEMA.tables) { const b = buf.get(t.id)!; if (b.n > 4096) SIZES.set(`table ${t.table}`, b.n); }
  // an order row the import has no closures for (rust's io::deal_closures): it would stand at its blank there
  for (const r of BK.listOf('OrdersOrderDefId') as any[]) if (!SCHEMA.dealtRows.includes(r.id)) { AT = 'orders::OrderDef'; miss(`no closures by id for the row ${r.id}: add it to hand2.cjs ROWS`, r.id); }
  // a beat under way whose place Rust never wrote: rust's import refuses this state by name (io::refuse_mid_script), because a generator's place cannot be read
  for (const r of BK.listOf('DirectorLiveBeatId') as any[]) if (r.state === 'running' && !hasEcho(r, 'director::LiveBeat.it')) { AT = 'director::LiveBeat'; miss(`the beat ${r.name} is running: this state cannot be imported`, r.name); }
  const out = new Out(); out.bytes(main.view());
  for (const t of SCHEMA.tables) { const b = buf.get(t.id)!; out.u32(done.get(t.id)!); out.bytes(b.view()); }
  out.u32(BK.nextSerial);
  // the poses no pool holds, last, because walking anything can meet one: nodes, then the floats (rust: Game.loose_poses, read by no comparison)
  out.u32(BK.loose.size); for (const p of BK.loose.keys() as Iterable<any>) { out.u32(p.numNodes); const c: Float32Array = p.cols; out.u32(c.length); for (let i = 0; i < c.length; i++) out.f32(c[i]); }
  return out.view().slice();
}
