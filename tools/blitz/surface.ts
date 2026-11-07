// THE CALL SURFACE (bridge/SURFACE.md, written by surface.py). A function of the tick that something outside the tick calls has a guard at the top of its TypeScript body, put there as the file loads (expose.ts, page-plugin.ts). While a host of the Rust game stands, the guard hands the call to the Rust game by its row: the arguments as bytes by the schema (a thing with an ID by its ID, a value by its bytes), `io::call`, the filtered apply, the answer back. No file changes.
//   installSurface();             once, before the first game is stood. `__tpsCall` answers PASS for a game with no host, so the TypeScript runs as it was
//   surfaceHost(H);               a game's host stands: H.surfaceCall(row, bytes) => bytes, H.surfaceOpen() => is a call to cross now (no Rust game yet, or inside the host's own lines: no)
// INSIDE THE HOST'S OWN LINES nothing crosses: a guard reached from `Game.update`'s lines, an apply or the dress step runs as the TypeScript it is.
// A CALL THROUGH A TOOL'S HOOK: an out event's dispatch calls the method on the puppet as it stands, so a tool's replacement fires, and its call through to the original does nothing: the Rust game has already done it (`dispatching`).
import { SCHEMA, parse, bookOf, exportPart } from './export';
import CALLS from './calls.json';
import TAPS from './taps.json';
import { WEAPONS } from '../../src/game/weapons';
import { simRandom } from '../../src/math/dice';
const PASS = Symbol('run the TypeScript');
const DIE: { f: (() => number) | null } = { f: null }; (globalThis as any).__tpsRoll = () => { if (!DIE.f) throw new Error('the call surface: the Rust game asked a die no caller handed in'); return DIE.f(); };
const DEC = new TextDecoder();
export interface SurfaceHost { g: any; knobs: Record<string, unknown>; surfaceOpen(): boolean; surfaceDispatching(): boolean; surfaceCall(row: number, bytes: Uint8Array): Uint8Array; count?(s: string): void }
const TALLY: any = (globalThis as any).__tpsTally ?? { crossed: 0, pass() {} };   // (expose.ts keeps it, from before any game is stood, and prints it)
const passed = (key: string, how: 'noHost' | 'noRow' | 'unlisted' | 'closed') => TALLY.pass(key, how);
const hosts = new Set<SurfaceHost>(); let current: SurfaceHost | null = null;
export const surfaceHost = (H: SurfaceHost | null, gone?: SurfaceHost) => { if (H) TALLY.standing = false; if (gone) { hosts.delete(gone); if (current === gone) current = null; } if (H) { hosts.add(H); current = H; } };
// A CALL'S OWN GAME: the receiver, or an argument, that is a game or holds one (`o.g`, game-ref.json). What holds none goes to the game stood last
const gameOf = (o: any): SurfaceHost | null => !o || typeof o !== 'object' ? null : o.rustSurface ?? o.g?.rustSurface ?? null;
const hostOf = (self: any, args: any[]): SurfaceHost | null => { const H = gameOf(self); if (H) return H; for (const a of args) { const A = gameOf(a); if (A) return A; } return current; };
class In { p = 0; dv: DataView; constructor(readonly b: Uint8Array) { this.dv = new DataView(b.buffer, b.byteOffset, b.byteLength); } }
function val(n: any, I: In, bk: any): any { const dv = I.dv; switch (n.k) {
  case 'f64': { const v = dv.getFloat64(I.p, true); I.p += 8; return v; } case 'f32': { const v = dv.getFloat32(I.p, true); I.p += 4; return v; }
  case 'bool': return I.b[I.p++] !== 0; case 'u8': return I.b[I.p++]; case 'i8': { const v = dv.getInt8(I.p); I.p += 1; return v; } case 'u16': { const v = dv.getUint16(I.p, true); I.p += 2; return v; } case 'i16': { const v = dv.getInt16(I.p, true); I.p += 2; return v; }
  case 'u32': case 'usize': { const v = dv.getUint32(I.p, true); I.p += 4; return v; } case 'i32': { const v = dv.getInt32(I.p, true); I.p += 4; return v; } case 'u64': case 'i64': { const v = dv.getFloat64(I.p, true); I.p += 8; return v; }
  case 'str': { const L = dv.getUint32(I.p, true); I.p += 4; const s = DEC.decode(I.b.subarray(I.p, I.p + L)); I.p += L; return s; } case 'unit': return undefined;
  case 'id': { const v = dv.getUint32(I.p, true); I.p += 4; return v === 0xffffffff ? null : bk.listOf(n.id)[v] ?? null; }
  case 'opt': return I.b[I.p++] ? val(n.of, I, bk) : null;
  case 'vec': { const L = dv.getUint32(I.p, true); I.p += 4; const a: any[] = []; for (let i = 0; i < L; i++) a.push(val(n.of, I, bk)); return a; }
  case 'arr': { const a: any[] = []; for (let i = 0; i < n.n; i++) a.push(val(n.of, I, bk)); return a; } case 'tup': return n.of.map((x: any) => val(x, I, bk));
  case 'map': { const L = dv.getUint32(I.p, true); I.p += 4; const m = new Map(); for (let i = 0; i < L; i++) { const k = val(n.K, I, bk); m.set(k, val(n.V, I, bk)); } return m; }
  case 'set': { const L = dv.getUint32(I.p, true); I.p += 4; const m = new Set(); for (let i = 0; i < L; i++) m.add(val(n.K, I, bk)); return m; }
  case 'type': { const T = SCHEMA.types[n.path]; if (T.k === 'enum') { const v = dv.getUint16(I.p, true); I.p += 2; return T.words[v]; }
    if (T.k === 'struct') { const o: any = {}; for (const f of T.fields) { const v = val(parse(f.ty), I, bk); const key = f.ts ?? f.expr?.match(/^o\.([\w$]+)/)?.[1] ?? f.rust; o[key] = v; } return o; }
    if (T.k === 'data' || T.k === 'handEnum') { const a = dv.getUint16(I.p, true); I.p += 2; const V = T.vars?.[a]; const ty = V?.ty ?? (T.arms?.[a]?.match(/\((.*)\)/)?.[1]); if (!ty) return V?.str ?? (V?.kw !== undefined ? { kind: V.kw } : V?.tag ?? T.arms?.[a]); const x = val(parse(ty), I, bk); if (V?.kw !== undefined && x && typeof x === 'object') x.kind = V.kw; return x; } }
  } throw new Error(`surface: no way to read a ${n.k} ${n.path ?? ''}`); }
const ROWS: any[] = (CALLS as any).rows;
function call(row: number, self: any, args: any[]): any { const H = hostOf(self, args); if (!H) { passed(ROWS[row].key, 'noHost'); return PASS; } if (H.surfaceDispatching()) return undefined; if (!H.surfaceOpen()) { passed(ROWS[row].key, 'closed'); return PASS; }
  // A DIE THE CALLER HANDS IN (`rnd`): the game's own is no argument at all. A tool's own is asked as the Rust function asks it, by the face's import `face_roll`
  const C = ROWS[row], g = H.g, parts: [string, unknown][] = []; let die: (() => number) | null = null; for (const a of C.args) { if (a.ty === 'rnd') { const f = args[a.from]; die = typeof f === 'function' && f !== simRandom ? f : null; parts.push(['bool', die !== null]); continue; } parts.push([a.ty, a.from === 'this' ? self : args[a.from]]); }
  // AN ID THE FUNCTION DOES NOT TAKE AS OPTIONAL, AND NOTHING THERE: the TypeScript would throw on it, so this throws; in Rust it would be a panic, and the game lost
  for (const [ty, v] of parts) if ((v === null || v === undefined) && parse(ty).k === 'id') throw new TypeError(`${C.key}: an argument of type ${ty} is ${v}`);
  // A ROW THE RUST GAME HAS NOT GOT (a tour's own RoomClear, made in TypeScript since the last start): the TypeScript runs, on its own row
  { const bk0 = bookOf(g); for (const [ty, v] of parts) { const n = parse(ty); if (n.k === 'id' && bk0 && !((bk0.ids.get(n.id)?.get(v as object) ?? Infinity) < ((bk0 as any).known?.get(n.id) ?? Infinity))) { passed(C.key, 'noRow'); H.count?.(`passed, a row Rust has not got: ${C.key}`); return PASS; } } }
  TALLY.crossed++; H.count?.(`crossed: ${C.key}`); const dieWas = DIE.f; DIE.f = die; let out: Uint8Array; try { out = H.surfaceCall(row, exportPart(g, H.knobs, parts)); } finally { DIE.f = dieWas; } const bk = bookOf(g), I = new In(out); let r: any;
  if (C.result) { if (I.b[I.p++]) { throw new Error(val({ k: 'str' }, I, bk)); } } if (C.ret) r = val(parse(C.ret), I, bk);
  for (const a of C.args) if (a.mut) { const v = val(parse(a.ty), I, bk), t = a.from === 'this' ? self : args[a.from]; if (Array.isArray(t) && Array.isArray(v)) { t.length = v.length; for (let i = 0; i < v.length; i++) t[i] = v[i]; } else if (t && typeof t === 'object' && v && typeof v === 'object') Object.assign(t, v); }
  return r; }
// ---- WHAT A GREEN WAS MADE OF: how many calls crossed to the Rust game, and how many ran the TypeScript's body with no host standing, on a row Rust has not got, or as a function not on the surface. Printed as the process ends, so no green is read as Rust's that was not
// ---- A TAP BY FUNCTION (taps.json, written by taps.py). A tool replaces a method on the puppet, or a table's entry: the host sees it (`sweep`) and sets the function's bit; the Rust function then reports its call, arguments and answer, as an out event, and the dispatch calls the method ON THE PUPPET AS IT STANDS. Behind the replacement the original answers from the record. A replacement's own answer does not reach the tick: the tick has already run
const TAP: any[] = TAPS as any; let firing: { id: number; answer: any } | null = null; const CLASS_BASE = new Map<number, unknown>();
const recvOf = (g: any, path: string) => { let o = g; for (const k of path.split('.').slice(1)) { o = o?.[k]; if (o === undefined || o === null) return null; } return o; };
export function makeTaps(g: any, classes: Record<string, any>, set: (id: number, on: boolean) => void) { const base = new Map<number, unknown>(), sent = new Set<number>();
  const now = (T: any) => { if (T.recv === 'self') return classes[T.cls]?.prototype?.[T.m]; return recvOf(g, T.recv)?.[T.m]; };
  for (const T of TAP) { const v = now(T); if (T.recv === 'self') { if (!CLASS_BASE.has(T.id)) CLASS_BASE.set(T.id, v); } else base.set(T.id, v); }
  return { sweep(): void { for (const T of TAP) { const v = now(T), was = T.recv === 'self' ? CLASS_BASE.get(T.id) : (base.has(T.id) && base.get(T.id) !== undefined ? base.get(T.id) : (base.set(T.id, v), v)), on = v !== undefined && v !== was; if (on !== sent.has(T.id)) { set(T.id, on); if (on) sent.add(T.id); else sent.delete(T.id); } } }, reset(): void { sent.clear(); } }; }
export function fireTap(g: any, row: number, args: number[], answer: number[]): void { const T = TAP[row], bk = bookOf(g), I = new In(Uint8Array.from(args)), a: any[] = []; for (const p of T.args) { if (!p.ty) { a.push(undefined); continue; } const v = val(parse(p.ty), I, bk); a.push(p.def ? (WEAPONS as any)[v] : v); }
  const A = T.ret ? val(parse(T.ret), new In(Uint8Array.from(answer)), bk) : undefined; const o = T.recv === 'self' ? a.shift() : recvOf(g, T.recv); if (!o || typeof o[T.m] !== 'function') return; const was = firing; firing = { id: T.id, answer: A }; try { o[T.m](...a); } finally { firing = was; } }
export function installSurface(): void { const c: any = call; c.PASS = PASS; c.newGame = () => { current = null; TALLY.standing = true; }; c.tapFiring = (id: number) => firing?.id === id; c.tapAnswer = () => firing!.answer; c.passed = (key: string) => { if (firing) return; if (!current) passed(key, 'noHost'); else if (current.surfaceOpen()) passed(key, 'unlisted'); }; (globalThis as any).__tpsCall = c; }
export const surfaceRows = ROWS;
// ---- THE CROSSING AS A PLAIN FUNCTION: what a function of the tick is once its TypeScript body is gone (blitz/skeleton/STUBS.md): `return crossing('DoorTools.order', this, [o, who]) as any`. By the row's `stub` word, which does not move with a cut. There is no TypeScript to run: where the guard would have run it this says so
let ROW_OF: Map<string, number> | null = null, TAP_OF: Map<string, number> | null = null;
export function crossing(key: string, self: unknown, args: unknown[]): unknown {
  TAP_OF ??= new Map(TAP.map(T => [T.key, T.id])); const t = TAP_OF.get(key); if (t !== undefined && firing?.id === t) return firing.answer;
  ROW_OF ??= new Map(ROWS.map((r, i) => [r.stub, i])); const row = ROW_OF.get(key); if (row === undefined) throw new Error(`the call surface: no row ${key}`);
  const r = call(row, self, args as any[]); if (r === PASS) throw new Error(`the call surface: ${key} was called where it cannot cross (no game stands, the host's own lines are running, or an argument is a row Rust has not got)`); return r; }
/** a tapped function with no row: all it still does is answer a tool that replaced it and called through */
export function tapAnswer(key: string): unknown { TAP_OF ??= new Map(TAP.map(T => [T.key, T.id])); return firing?.id === TAP_OF.get(key) ? firing!.answer : undefined; }
