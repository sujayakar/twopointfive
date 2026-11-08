// THE DIFF of two whole states (a TypeScript export and a Rust one) in O(file) memory, walking both by the schema side by side: no list of every path is built
// 
//   bun run tools/blitz/diff.ts <a.bin> <b.bin> [--show 30] [--out file.tsv] [--canon BoxId,LightId | --canon-file canon.txt] [--known known.txt[,more.txt]] [--blocks s.bin ns.bin] [--why] [--dump <path regex> [--dump-max 400]] [--dynboxes] [--leave-out skin,look]
//   bun run tools/blitz/diff.ts <a.bin> <a.bin> --tables     how many rows each table of a state holds
//   exit 0 the same but for what is known · 1 they part
// CANON: a table named in --canon (default BoxId) is compared BY REACH, not by row number: each side's rows are numbered in the order one fixed walk first meets them
// (the root, then every other table's rows in order, then the rows those reach), and a row nothing reaches is not compared. The TypeScript's walk numbers a made box
// when an export first meets it and the Rust game when it is made; with an export only every so many ticks those numbers differ and mean nothing. A path of such a
// row reads `g.tab.boxes[c12]`, and an ID's value `BoxId(c12)`.
// --live: BY REACH FROM THE LIVE GAME, for two states the die and the block call equal. Three rules, each with what it hides:
//  1. A map or set keyed by a row of a canon table is not walked for reach until its key is reached some other way. The TypeScript keeps such tables
//     weakly, by the object, and lists a row for a player, a gun or an order's assignment that is out of the game (before a fresh deal, a restart, a
//     load) for as long as the collector leaves it; numbered through that row, every gun after it compared crosswise. The Rust game keeps a row by its
//     ID until something clears the table. Neither side's row under a key NOTHING ELSE REACHES can be asked by a rule (a weak table has no list of its
//     keys), so the rows only a key reaches are named by their CONTENT (`Id(~1x9k2)`: a hash of the row with its IDs canonical and its identity serials
//     blank); an entry keyed by one is compared where both sides hold a row of that content and COUNTED, by side and table, where one side does.
//     HIDES: a strong map or set keyed by a row nothing else holds, where the two rows differ.
//  2. A table keyed by an identity serial (schema `serial`, `<field>{}`: a weak set of event objects, a table keyed by an object) cannot be matched by
//     key. Compared by place it read crosswise as soon as one side held a member the other had dropped (the events of an abandoned future after a load).
//     A map's entries are matched as a multiset of their values: what the Rust side holds and the TypeScript's does not is reported; what only the
//     TypeScript holds is counted. A set is held to its count, and only the Rust side's surplus is reported.
//     HIDES: which key holds which value; a member only the TypeScript holds.
//  3. `WallchargeMod.stand_ins` (a declared memo) is not walked for reach.
// A Vec of two lengths: `.len` differs, the common rows are compared, the rest is counted as one path `[n..]`. A map / set is compared by key.
import { readFileSync, writeFileSync } from 'node:fs';
const { SCHEMA, parse } = await import('./export.ts');
type Node = { k: string; [x: string]: any };
const argv = process.argv.slice(2), opt = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const SHOW = +opt('--show', '30'), OUT = opt('--out', ''), CANON = new Set((opt('--canon-file', '') ? readFileSync(opt('--canon-file', ''), 'utf8').trim() : opt('--canon', 'BoxId')).split(',').filter(Boolean));
// (`--known a.txt,b.txt`: several files. A line's THIRD column narrows it by the two values: `bits:<mask>` = known only where the two numbers differ in
//  those bits and no other; `ts:<regex>` / `rs:<regex>` = known only where the TypeScript's / the Rust's value matches (several, a space between; no
//  space inside one). With no third column the path alone decides.)
const KNOWN = opt('--known', `${import.meta.dir}/known.txt`).split(',').flatMap(f => readFileSync(f, 'utf8').split('\n')).filter(l => l && !l.startsWith('#')).map(l => { const [re, why, only] = l.split('\t');
  const W = (only ?? '').split(' '), of = (k: string) => W.find(x => x.startsWith(k))?.slice(k.length);
  const bits = of('bits:') !== undefined ? Number(of('bits:')) : -1, ts = of('ts:') !== undefined ? new RegExp(of('ts:')!) : null, rs = of('rs:') !== undefined ? new RegExp(of('rs:')!) : null;
  return { re: new RegExp(re), why, n: 0, ok: (a: string, b: string) => (bits < 0 || ((Number(a.split(' ')[0]) ^ Number(b.split(' ')[0])) & ~bits) === 0) && (!ts || ts.test(a)) && (!rs || rs.test(b)) }; });
// (a field the two sides are KNOWN to hold differently is not walked for reach, or the two numberings would part there: BoxMirror.things_of, a reference the TypeScript's walk writes as a value)
const NOREACH = new Set(opt('--noreach', 'things_of,lamp_pick,WorkThings.cache,DoortoolsMod.sky').split(','));   // (… the fog's lamps' list, picture-only and not filled by the Rust tick; the work things' memo the test driver's own asks fill)
const LIVE = argv.includes('--live');
// STAND-IN (belongs in schema.json's `serial`, as `<field>{}`): tables keyed by an object's identity serial that the schema does not mark so. A gun hold's
// key is an object (gunhold.ts `P.key`), written as the serial each side deals it: under --live such a table is compared by place, as the marked ones are.
const KEYED_BY_SERIAL = new Set(opt('--keyed-by-serial', 'gunhold::GunholdMod.knowns,guards::GuardsMod.holds_called').split(','));
// STAND-IN (belongs in schema.json's `serial`, as `dialogue::RadioState.run` is there): fields that hold an object's identity serial as a plain number.
// A mission's own module keeps the mission it last saw (`this.run === g.mission`; the Rust keeps the mission's serial), and each side deals serials by
// its own count of objects, so the number is not compared; whether one is held still is.
const SERIAL_COPIES = new Set(opt('--serial-copies', 'rails::Rails.run,lesson::Lesson.run').split(','));
/** --live: the entries of maps and sets keyed by a canon row, met by the reach walk and held back until their key is live */
type Held = { K: Node; V: Node | null; kp: number; vp: number; done: boolean };
let HELD: Held[] | null = null;
/** while a value's canonical bytes are taken: the places of its identity serials and scratch fields, blanked */
let HOLES: [number, number][] | null = null;
if (LIVE) NOREACH.add('WallchargeMod.stand_ins');   // (a declared memo, wallcharge.ts:303: the row a menu asks for in a digit's place, made when the row list is first asked; a tool that asks the menu between two ticks makes rows on the TypeScript's side that no tick made)
const DEC = new TextDecoder();
// --dump <regex>: every leaf whose path matches, both sides' values, the same or not (to read a piece of state)
const DUMP = opt('--dump', '') ? new RegExp(opt('--dump', '')) : null; let dumped = 0; const DUMP_MAX = +opt('--dump-max', '400');
class Side { liveN = new Map<string, number>(); name2 = new Map<string, Map<number, string>>(); dv: DataView; p = 0; serialAt = 0; left = new Map<number, boolean>(); leftN = 0; rows = new Map<string, number[]>(); canon = new Map<string, Map<number, number>>(); order = new Map<string, number[]>(); tableAt = new Map<string, number>(); tableLen = new Map<string, number>();
  constructor(public b: Uint8Array) { this.dv = new DataView(b.buffer, b.byteOffset, b.byteLength); }
  u8() { return this.b[this.p++]; } u16() { const x = this.dv.getUint16(this.p, true); this.p += 2; return x; } u32() { const x = this.dv.getUint32(this.p, true); this.p += 4; return x; }
  hex(n: number) { let s = ''; for (let i = 0; i < n; i++) s += this.b[this.p + i].toString(16).padStart(2, '0'); this.p += n; return s; }
  cid(id: string, v: number): string { if (MIX[id]) { const [t, r] = unmix(id, v); return this.cid(t, r); } const M = this.canon.get(id); if (!M || v === 0xffffffff) return `${id}(${v})`; const c = M.get(v); return c === undefined ? `${id}(unreached)` : c >= (this.liveN.get(id) ?? Infinity) ? `${id}(~${nameOf(this, id, v)})` : `${id}(c${c})`; } }
const T = (path: string) => SCHEMA.types[path];
/** an ID of either of two tables (schema `mixes`): which table, and its row */
const MIX: Record<string, string[]> = SCHEMA.mixes ?? {}, unmix = (id: string, v: number): [string, number] => MIX[id] ? [MIX[id][v & 1], v >>> 1] : [id, v];
/** walk past a value; `onId` hears every ID of a canon table met */
function skip(n: Node, s: Side, onId: ((id: string, v: number) => void) | null): void {
  switch (n.k) {
    case 'f64': case 'u64': case 'i64': s.p += 8; return; case 'f32': case 'u32': case 'i32': case 'usize': s.p += 4; return; case 'bool': case 'u8': case 'i8': s.p += 1; return; case 'u16': case 'i16': s.p += 2; return;
    case 'id': { const v = s.u32(); if (onId) { const [id, r] = unmix(n.id, v); if (CANON.has(id)) onId(id, r); } return; }
    case 'str': { const L = s.u32(); s.p += L; return; } case 'unit': return;
    case 'opt': if (s.u8()) skip(n.of, s, onId); return;
    case 'vec': { const L = s.u32(); const of = n.of; if (!onId || !hasId(of)) { const w = fixed(of); if (w >= 0) { s.p += w * L; return; } } for (let k = 0; k < L; k++) skip(of, s, onId); return; }
    case 'arr': { const w = (!onId || !hasId(n.of)) ? fixed(n.of) : -1; if (w >= 0) { s.p += w * n.n; return; } for (let k = 0; k < n.n; k++) skip(n.of, s, onId); return; }
    case 'tup': for (const x of n.of) skip(x, s, onId); return;
    case 'map': { const L = s.u32(); if (onId && HELD && hasId(n.K)) { for (let k = 0; k < L; k++) { const kp = s.p; skip(n.K, s, null); const vp = s.p; skip(n.V, s, null); HELD.push({ K: n.K, V: n.V, kp, vp, done: false }); } return; } for (let k = 0; k < L; k++) { skip(n.K, s, onId); skip(n.V, s, onId); } return; }
    case 'set': { const L = s.u32(); if (onId && HELD && hasId(n.K)) { for (let k = 0; k < L; k++) { const kp = s.p; skip(n.K, s, null); HELD.push({ K: n.K, V: null, kp, vp: 0, done: false }); } return; } for (let k = 0; k < L; k++) skip(n.K, s, onId); return; }
    case 'type': { const t = T(n.path);
      if (t.k === 'struct') { const F = fieldsOf(t); for (let i = 0; i < F.length; i++) {
          if (HOLES) { const q = `${t.mod}::${t.name}.${t.fields[i].rust}`; if (SCHEMA.serial?.[q] || SERIAL_COPIES.has(q) || SCHEMA.scratch?.[q] || SCHEMA.serial?.[`${q}{}`]) { const a = s.p; skip(F[i], s, null); HOLES.push([a, s.p]); continue; } }
          skip(F[i], s, onId && (NOREACH.has(t.fields[i].rust) || NOREACH.has(`${t.name}.${t.fields[i].rust}`)) ? null : onId); } return; }
      if (t.k === 'enum') { s.p += 2; return; }
      if (t.k === 'data') { const a = t.vars[s.u16()]; if (a.ty) skip(parse(a.ty), s, onId); return; }
      if (t.k === 'handEnum') { const a = t.arms[s.u16()]; const ty = a.match(/\((.*)\)/)?.[1]; if (ty) skip(parse(ty), s, onId); return; }
      // (an arm chosen by the object's own word, each arm with fields of its own: a lesson step's test)
      if (t.k === 'wordEnum') { const a = t.arms[s.u16()]; for (const f of a.fields) skip(parse(f.ty), s, onId); return; }
      throw new Error(`diff: no skip for the kind ${t.k} (${n.path})`); } }
  throw new Error(`diff: no skip for ${n.k}`);
}
const FIELDS = new Map<any, Node[]>(); const fieldsOf = (t: any): Node[] => { let F = FIELDS.get(t); if (!F) { F = t.fields.map((f: any) => parse(f.ty)); FIELDS.set(t, F!); } return F!; };
const FIX = new Map<Node, number>(); function fixed(n: Node): number { let w = FIX.get(n); if (w !== undefined) return w; FIX.set(n, -1);
  switch (n.k) { case 'f64': case 'u64': case 'i64': w = 8; break; case 'f32': case 'u32': case 'i32': case 'usize': case 'id': w = 4; break; case 'bool': case 'u8': case 'i8': w = 1; break; case 'u16': case 'i16': w = 2; break; case 'unit': w = 0; break;
    case 'arr': { const x = fixed(n.of); w = x < 0 ? -1 : x * n.n; break; } case 'tup': { w = 0; for (const x of n.of) { const y = fixed(x); if (y < 0) { w = -1; break; } w += y; } break; }
    case 'type': { const t = T(n.path); if (t.k === 'enum') w = 2; else if (t.k === 'struct') { w = 0; for (const f of fieldsOf(t)) { const y = fixed(f); if (y < 0) { w = -1; break; } w += y; } } else w = -1; break; }
    default: w = -1; }
  FIX.set(n, w!); return w!; }
const HAS = new Map<Node, boolean>(); function hasId(n: Node): boolean { let h = HAS.get(n); if (h !== undefined) return h; HAS.set(n, false);
  switch (n.k) { case 'id': h = CANON.has(n.id) || (MIX[n.id] ?? []).some(x => CANON.has(x)); break; case 'opt': case 'vec': case 'arr': h = hasId(n.of); break; case 'tup': h = n.of.some(hasId); break; case 'map': h = hasId(n.K) || hasId(n.V); break; case 'set': h = hasId(n.K); break;
    case 'type': { const t = T(n.path); h = t.k === 'struct' ? fieldsOf(t).some(hasId) : t.k === 'data' ? t.vars.some((a: any) => a.ty && hasId(parse(a.ty))) : t.k === 'handEnum' ? t.arms.some((a: string) => { const ty = a.match(/\((.*)\)/)?.[1]; return ty ? hasId(parse(ty)) : false; }) : t.k === 'wordEnum' ? t.arms.some((a: any) => a.fields.some((f: any) => hasId(parse(f.ty)))) : false; break; }
    default: h = false; }
  HAS.set(n, h!); return h!; }
/** A VALUE'S CANONICAL WORD (--live): a hash of its bytes with every canon ID written as its canon number (a row only a key reaches: its own content's
 *  word), every identity serial and scratch field blank. Leaves `s.p` past the value. */
function canonWord(n: Node, s: Side): string {
  const p0 = s.p, ids: [number, string, number][] = [], holes0 = HOLES; HOLES = []; const held0 = HELD; HELD = null;
  skip(n, s, (id, v) => { ids.push([s.p - 4, id, v]); }); const holes = HOLES; HOLES = holes0; HELD = held0; const p1 = s.p;
  const b = s.b.slice(p0, p1), dv = new DataView(b.buffer);
  for (const [x, y] of holes) b.fill(0, x - p0, y - p0);
  for (const [at, id, v] of ids) { if (v === 0xffffffff) continue; const M = s.canon.get(id)!, c = M.get(v); let w: number;
    if (c === undefined) w = 0xfffffffe; else if (c < (s.liveN.get(id) ?? Infinity)) w = c; else w = (Number(BigInt.asUintN(32, BigInt(Bun.hash(nameOf(s, id, v))))) | 0x80000000) >>> 0;
    if (at - p0 >= 0 && at - p0 + 4 <= b.length) dv.setUint32(at - p0, w, true); }
  s.p = p1; return Bun.hash(b).toString(36); }
const TABLE_TY = new Map<string, Node>();
/** the content's word of a row that only a key reaches (a row that names itself, round a ring of such rows: `ring`) */
function nameOf(s: Side, id: string, v: number): string { let M = s.name2.get(id); if (!M) { M = new Map(); s.name2.set(id, M); } let w = M.get(v); if (w !== undefined) return w;
  const at = s.rows.get(id)!; if (v >= at.length) { M.set(v, 'norow'); return 'norow'; } M.set(v, 'ring');
  if (!TABLE_TY.size) for (const t of SCHEMA.tables) TABLE_TY.set(t.id, parse(t.ty));
  const p = s.p; s.p = at[v]; w = canonWord(TABLE_TY.get(id)!, s); s.p = p; M.set(v, w); return w; }
const ROOT: Node = { k: 'type', path: SCHEMA.root };
/** index a side: where every table's rows start, and the canon tables' rows numbered by reach */
function index(s: Side): void {
  const queue: [string, number][] = []; for (const id of CANON) { s.canon.set(id, new Map()); s.order.set(id, []); }
  // where every table's rows start, first: the reach walk asks a box's own key
  s.p = 0; skip(ROOT, s, null);
  for (const t of SCHEMA.tables) { const L = s.u32(), N = parse(t.ty), at: number[] = new Array(L); s.tableLen.set(t.id, L); for (let k = 0; k < L; k++) { at[k] = s.p; skip(N, s, null); } s.rows.set(t.id, at); }
  s.serialAt = s.p; s.p += 4;
  // (then the floats of the poses no pool holds, each `row, n, n floats`: carried for an import and compared by nothing)
  for (let n = s.u32(); n > 0; n--) { s.p += 4; const L = s.u32(); s.p += L * 4; }
  if (s.p !== s.b.length) throw new Error(`diff: ${s.b.length - s.p} bytes left over`);
  const onId = (id: string, v: number) => { if (v === 0xffffffff) return; const M = s.canon.get(id)!; if (M.has(v) || leftOut(s, id, v)) return; M.set(v, M.size); s.order.get(id)!.push(v); queue.push([id, v]); };
  HELD = LIVE ? [] : null;
  s.p = 0; skip(ROOT, s, onId);
  for (const t of SCHEMA.tables) { if (CANON.has(t.id)) continue; const N = parse(t.ty); for (const at of s.rows.get(t.id)!) { s.p = at; skip(N, s, onId); } }
  const ty = new Map<string, Node>(SCHEMA.tables.map((t: any) => [t.id, parse(t.ty)]));
  let i = 0; const drain = () => { for (; i < queue.length; i++) { const [id, v] = queue[i]; const at = s.rows.get(id)!; if (v >= at.length) continue; s.p = at[v]; skip(ty.get(id)!, s, onId); } };
  drain();
  // (--live: an entry held back is walked once every canon row its key names is reached; the ready ones of a round in the order of their keys' canon
  //  words, so that the two sides' maps, listed in two orders, number what they reach alike. Round after round until none is made ready.)
  if (HELD) { const H = HELD; for (;;) { const ready: { e: Held; w: string }[] = [];
      for (const e of H) { if (e.done) continue; let live = true; s.p = e.kp; skip(e.K, s, (id, v) => { if (v !== 0xffffffff && !s.canon.get(id)!.has(v)) live = false; }); if (!live) continue; s.p = e.kp; ready.push({ e, w: word(e.K, s) }); }
      if (!ready.length) break; ready.sort((a, b) => a.w < b.w ? -1 : a.w > b.w ? 1 : 0);
      for (const { e } of ready) { e.done = true; if (e.V) { s.p = e.vp; skip(e.V, s, onId); } } drain(); }
    // (what is left is keyed by a row nothing but a key reaches: numbered now, after every live row, in the file's order)
    HELD = null; for (const id of CANON) s.liveN.set(id, s.canon.get(id)!.size);
    for (const e of H) { if (e.done) continue; s.p = e.kp; skip(e.K, s, onId); if (e.V) { s.p = e.vp; skip(e.V, s, onId); } } drain(); }
}
// LEFT OUT (--leave-out skin,look; nothing by default): boxes of a lane one side does not run, to see the rest. A box keyed MKEY.skin is what stands of a holed
// wall's paint (src/game/wallskin.ts); one keyed MKEY.look is a LOOK piece, drawn and in no collision list (a broken edge's rubble and bars: src/game/edgeLook.ts).
// Both lanes run on both sides and are compared. A box left out is in no reach numbering and is dropped from every list of boxes before the lists are
// compared; how many on each side is said at the end.
const MKEY_FAMILY: Record<string, number> = { look: 9, skin: 10 };
const LEAVE = new Set(opt('--leave-out', '').split(',').filter(Boolean).map(w => { if (!(w in MKEY_FAMILY)) throw new Error(`diff: --leave-out knows ${Object.keys(MKEY_FAMILY).join(', ')}, not ${w}`); return MKEY_FAMILY[w]; }));
const BOX_T: any = SCHEMA.tables.find((t: any) => t.id === 'BoxId'), BOX_N: any = BOX_T ? T(parse(BOX_T.ty).path) : null, MKEY_AT = BOX_N ? BOX_N.fields.findIndex((f: any) => f.rust === 'mkey') : -1;
function leftOut(s: Side, id: string, v: number): boolean {
  if ((id !== 'BoxId' && id !== 'FrameBoxId') || !LEAVE.size || MKEY_AT < 0) return false;
  const key = id === 'BoxId' ? v : -1 - v; let out = s.left.get(key); if (out !== undefined) return out;
  const at = s.rows.get(id)!; out = false;
  if (v < at.length) { const p0 = s.p, F = fieldsOf(BOX_N); s.p = at[v]; for (let i = 0; i < MKEY_AT; i++) skip(F[i], s, null); if (s.u8()) out = LEAVE.has(s.dv.getFloat64(s.p, true) >>> 24); s.p = p0; }
  s.left.set(key, out); if (out) s.leftN++; return out;
}
// ---- the comparison
// BY RULE (schema.json): a field in `schema.scratch` is a memo no rule reads back and is not compared; a field in `schema.serial` holds an object's
// identity serial, which the two sides deal apart by design: its value is not compared (whether it is there still is), and a map or set marked `<field>{}`
// is keyed by one and is compared by place. How many leaves each rule passed over is said at the end.
let SER = false, KEYSER = false, serialN = 0, scratchN = 0, nullN = 0, rowN = 0; const STALE = new Map<string, [number, number]>(); const stale = (q: string, a: number, b: number) => { const c = STALE.get(q) ?? [0, 0]; c[0] += a; c[1] += b; STALE.set(q, c); };
let n = 0, unknown = 0; const by = new Map<string, number>(), shown: string[] = [], all: string[] = [];
const P: string[] = [];
function rep(mod: string, a: string, b: string, tail = '') { n++; const path = P.join('') + tail; const k = KNOWN.find(k => k.re.test(path) && k.ok(a, b)); if (k) { k.n++; return; } unknown++; by.set(mod, (by.get(mod) ?? 0) + 1); const l = `${mod}\t${path}\t${a}\t${b}`; if (shown.length < SHOW) shown.push(l); if (OUT && all.length < 200000) all.push(l); }
function leaf(n: Node, s: Side): string {
  switch (n.k) { case 'f64': { const v = s.dv.getFloat64(s.p, true); return `${v} ${s.hex(8)}`; } case 'f32': { const v = s.dv.getFloat32(s.p, true); return `${v} ${s.hex(4)}`; }
    case 'bool': return s.u8() ? 'true' : 'false'; case 'u8': case 'i8': return String(s.u8()); case 'u16': case 'i16': return String(s.u16()); case 'u32': case 'i32': case 'usize': return String(s.u32()); case 'u64': case 'i64': return s.hex(8);
    case 'id': return s.cid(n.id, s.u32()); case 'str': { const L = s.u32(); const x = DEC.decode(s.b.subarray(s.p, s.p + L)); s.p += L; return JSON.stringify(x); } }
  throw new Error('diff: not a leaf ' + n.k);
}
/** a value as a short word (a map's key; one side of an Option) */
function word(n: Node, s: Side): string { switch (n.k) { case 'f64': case 'f32': case 'bool': case 'u8': case 'i8': case 'u16': case 'i16': case 'u32': case 'i32': case 'usize': case 'u64': case 'i64': case 'id': case 'str': { const x = leaf(n, s); return n.k === 'f64' || n.k === 'f32' ? x.split(' ')[0] : x; }
    case 'type': { const t = T(n.path); if (t.k === 'enum') return `${t.name}::${t.words[s.u16()]}`; break; } case 'tup': return n.of.map((x: Node) => word(x, s)).join(','); case 'arr': { const o: string[] = []; for (let k = 0; k < n.n; k++) o.push(word(n.of, s)); return o.join(','); } }
  skip(n, s, null); return '(…)'; }
const nan = (x: string) => x.startsWith('NaN ');
// A POSE OF ITS OWN, outside the pool (a ragdoll's; a man's not yet at home): its place reads Pose::LOOSE + its ROW of g.loose_poses, and the two sides number
// the rows differently (the TypeScript's export deals a row as its walk first meets the pose, the Rust game as the pose is made). Both sides a row: not
// compared, counted. One side a row and the other a place in the pool IS a difference. The floats and what they drive are compared.
const ownRow = (x: string) => { const v = +x.split(' ')[0]; return v >= 0xffff0000 && v < 0xffffffff; };
const same = (A: Side, a: number, B: Side, b: number, L: number) => { for (let i = 0; i < L; i++) if (A.b[a + i] !== B.b[b + i]) return false; return true; };
function cmp(n: Node, A: Side, B: Side, mod: string): void {
  switch (n.k) {
    case 'f64': case 'f32': case 'bool': case 'u8': case 'i8': case 'u16': case 'i16': case 'u32': case 'i32': case 'usize': case 'u64': case 'i64': case 'str': case 'id': {
      if (DUMP) { const path = P.join(''); if (DUMP.test(path) && dumped++ < DUMP_MAX) { const a0 = A.p, b0 = B.p; const a = leaf(n, A), b = leaf(n, B); console.log(`  ${a === b ? ' ' : '≠'} ${path}\t${a.split(' ')[0]}\t${b.split(' ')[0]}`); A.p = a0; B.p = b0; } }
      if (n.k === 'f64') { const lo = A.dv.getUint32(A.p, true) === B.dv.getUint32(B.p, true), hi = A.dv.getUint32(A.p + 4, true) === B.dv.getUint32(B.p + 4, true); if (lo && hi) { A.p += 8; B.p += 8; return; } }
      const a = leaf(n, A), b = leaf(n, B); if (a !== b && !(nan(a) && nan(b))) { if (SER) serialN++; else if (n.k === 'u32' && ownRow(a) && ownRow(b) && P[P.length - 1] === '.at') rowN++; else rep(mod, a, b); } return; }
    case 'unit': return;
    case 'opt': { const a = A.u8(), b = B.u8(); if (!a && !b) return; if (a && b) { cmp(n.of, A, B, mod); return; }
      if (a) rep(mod, word(n.of, A), 'None'); else rep(mod, 'None', word(n.of, B)); return; }
    case 'vec': if (n.of.k === 'id' && (n.of.id === 'BoxId' || n.of.id === 'FrameBoxId' || n.of.id === 'AnyBoxId') && CANON.has('BoxId') && LEAVE.size) { const of = n.of.id; const rd = (s: Side) => { const L = s.u32(), o: number[] = []; for (let k = 0; k < L; k++) { const v = s.u32(); if (!leftOut(s, ...unmix(of, v))) o.push(v); } return o; };
        const a = rd(A), b = rd(B); if (a.length !== b.length) rep(mod, String(a.length), String(b.length), '.len'); const c = Math.min(a.length, b.length);
        for (let k = 0; k < c; k++) { const x = A.cid(of, a[k]), y = B.cid(of, b[k]); if (x !== y) { P.push(`[${k}]`); rep(mod, x, y); P.pop(); } }
        if (a.length !== b.length) rep(mod, a.length > c ? `${a.length - c} more` : '(absent)', b.length > c ? `${b.length - c} more` : '(absent)', `[${c}..]`); return; }
      { const a = A.u32(), b = B.u32(); if (DUMP) { const path = P.join(''); if (DUMP.test(path + '.len') && dumped++ < DUMP_MAX) console.log(`  ${a === b ? ' ' : '≠'} ${path}.len\t${a}\t${b}`); } if (a !== b) rep(mod, String(a), String(b), '.len'); const c = Math.min(a, b); for (let k = 0; k < c; k++) { P.push(`[${k}]`); cmp(n.of, A, B, mod); P.pop(); }
      if (a > c) { for (let k = c; k < a; k++) skip(n.of, A, null); rep(mod, `${a - c} more`, '(absent)', `[${c}..]`); } if (b > c) { for (let k = c; k < b; k++) skip(n.of, B, null); rep(mod, '(absent)', `${b - c} more`, `[${c}..]`); } return;  }
    case 'arr': for (let k = 0; k < n.n; k++) { P.push(`[${k}]`); cmp(n.of, A, B, mod); P.pop(); } return;
    case 'tup': n.of.forEach((x: Node, k: number) => { P.push(`.${k}`); cmp(x, A, B, mod); P.pop(); }); return;
    case 'map': case 'set': { const isMap = n.k === 'map', ks = KEYSER; KEYSER = false;
      const read = (s: Side) => { const L = s.u32(), M = new Map<string, number>(); for (let k = 0; k < L; k++) { let key = word(n.K, s); if (ks) key = `#${k}`; M.set(key, s.p); if (isMap) skip(n.V, s, null); } return { L, M, end: s.p }; };
      // (--live, rule 2: a table keyed by an identity serial: a map by the multiset of its values, a set by its count)
      if (LIVE && ks) { const q = P.join('').replace(/\[c?\d+\]/g, '[]').replace(/\{[^}]*\}/g, '{}');
        const vals = (s: Side) => { const L = s.u32(), W: string[] = []; for (let k = 0; k < L; k++) { skip(n.K, s, null); if (isMap) W.push(canonWord(n.V, s)); } return { L, W }; };
        const a = vals(A), b = vals(B);
        if (!isMap) { if (b.L > a.L) rep(mod, String(a.L), String(b.L), '.len'); else if (a.L > b.L) stale(q, a.L - b.L, 0); return; }
        const have = new Map<string, number>(); for (const w of a.W) have.set(w, (have.get(w) ?? 0) + 1);
        let lone = 0; for (const w of b.W) { const c = have.get(w) ?? 0; if (c > 0) have.set(w, c - 1); else lone++; }
        if (lone) rep(mod, `${a.L} rows, none of that value`, `${lone} of ${b.L} rows`, '{by value}');
        const left = a.L - (b.L - lone); if (left > 0) stale(q, left, 0); return; }
      const a = read(A), b = read(B);
      // (--live, rule 1: an entry keyed by a row that only a key reaches, which one side holds and the other does not: counted by side and table)
      let da = 0, db = 0; if (LIVE) { for (const key of [...a.M.keys()]) if (key.includes('(~') && !b.M.has(key)) { a.M.delete(key); da++; } for (const key of [...b.M.keys()]) if (key.includes('(~') && !a.M.has(key)) { b.M.delete(key); db++; }
        if (da || db) { const q = P.join('').replace(/\[c?\d+\]/g, '[]').replace(/\{[^}]*\}/g, '{}'); stale(q, da, db); } }
      if (a.L - da !== b.L - db) rep(mod, String(a.L - da), String(b.L - db), '.len');
      for (const [key, at] of a.M) { const bt = b.M.get(key); P.push(`{${key}}`); if (bt === undefined) rep(mod, isMap ? '(a row)' : 'in', '(absent)'); else if (isMap) { A.p = at; B.p = bt; cmp(n.V, A, B, mod); } P.pop(); }
      for (const key of b.M.keys()) if (!a.M.has(key)) { P.push(`{${key}}`); rep(mod, '(absent)', isMap ? '(a row)' : 'in'); P.pop(); }
      A.p = a.end; B.p = b.end; return; }
    case 'type': { const t = T(n.path);
      if (t.k === 'struct') { const F = fieldsOf(t); for (let i = 0; i < F.length; i++) { const q = `${t.mod}::${t.name}.${t.fields[i].rust}`;
          if (t.fields[i].echo && t.fields[i].ty === 'Vec<u8>') { skip(F[i], A, null); skip(F[i], B, null); continue; }   // (a blank field: Rust's own bytes, which the TypeScript's walk has none of)
          if (SCHEMA.scratch?.[q]) { const a0 = A.p, b0 = B.p; skip(F[i], A, null); skip(F[i], B, null); if (A.p - a0 !== B.p - b0 || !same(A, a0, B, b0, A.p - a0)) scratchN++; continue; }
          const s0 = SER, k0 = KEYSER; if (SCHEMA.serial?.[q] || SERIAL_COPIES.has(q)) SER = true; KEYSER = !!SCHEMA.serial?.[`${q}{}`] || (LIVE && KEYED_BY_SERIAL.has(q));
          P.push(`.${t.fields[i].rust}`); cmp(F[i], A, B, t.mod); P.pop(); SER = s0; KEYSER = k0; } return; }
      if (t.k === 'enum') { const a = A.u16(), b = B.u16(); if (DUMP) { const path = P.join(''); if (DUMP.test(path) && dumped++ < DUMP_MAX) console.log(`  ${a === b ? ' ' : '≠'} ${path}\t${t.words[a]}\t${t.words[b]}`); } if (a !== b) rep(mod, `${t.name}::${t.words[a]}`, `${t.name}::${t.words[b]}`); return; }
      if (t.k === 'data') { const a = t.vars[A.u16()], b = t.vars[B.u16()]; if (a !== b) { rep(mod, a.tag, b.tag, '#'); if (a.ty) skip(parse(a.ty), A, null); if (b.ty) skip(parse(b.ty), B, null); return; } if (a.ty) { P.push(`.${a.tag}`); cmp(parse(a.ty), A, B, mod); P.pop(); } return; }
      if (t.k === 'handEnum') { const a = t.arms[A.u16()], b = t.arms[B.u16()]; const ta = a.match(/\((.*)\)/)?.[1], tb = b.match(/\((.*)\)/)?.[1]; if (a !== b) { rep(mod, a, b, '#'); if (ta) skip(parse(ta), A, null); if (tb) skip(parse(tb), B, null); return; } if (ta) { P.push(`.${a.split('(')[0]}`); cmp(parse(ta), A, B, mod); P.pop(); } return; }
      if (t.k === 'wordEnum') { const a = t.arms[A.u16()], b = t.arms[B.u16()]; if (a !== b) { rep(mod, a.tag, b.tag, '#'); for (const f of a.fields) skip(parse(f.ty), A, null); for (const f of b.fields) skip(parse(f.ty), B, null); return; }
        for (const f of a.fields) { P.push(`.${f.rust}`); cmp(parse(f.ty), A, B, mod); P.pop(); } return; }
      throw new Error(`diff: no compare for the kind ${t.k} (${n.path})`); } }
  throw new Error(`diff: no compare for ${n.k}`);
}
const A = new Side(new Uint8Array(readFileSync(argv[0]))), B = new Side(new Uint8Array(readFileSync(argv[1])));
index(A); index(B);
// --tables: how many rows each table of the first state holds, a line a table (`<the TypeScript's name of a row>\t<rows>`), and nothing else
if (argv.includes('--tables')) { for (const t of SCHEMA.tables) console.log(`${t.ts}\t${A.tableLen.get(t.id) ?? 0}`); process.exit(0); }
A.p = 0; B.p = 0; P.push('g'); cmp(ROOT, A, B, 'game'); P.pop();
for (const t of SCHEMA.tables) { const N = parse(t.ty), mod = t.mod ?? 'game', ra = A.rows.get(t.id)!, rb = B.rows.get(t.id)!;
  if (CANON.has(t.id)) { const oa = A.order.get(t.id)!, ob = B.order.get(t.id)!; P.push(`g.${t.table}`);
    // (--live: the live rows are held to each other by their canon numbers; a row only a key reaches is named by its content, so two of one name are
    //  one content and there is nothing to compare: counted by side)
    if (LIVE) { const la = A.liveN.get(t.id)!, lb = B.liveN.get(t.id)!; if (la !== lb) rep(mod, String(la), String(lb), '.reached'); if (oa.length > la || ob.length > lb) stale(`g.${t.table} (rows only a key reaches)`, oa.length - la, ob.length - lb); P.pop();
      const c = Math.min(la, lb); for (let j = 0; j < c; j++) { if (oa[j] >= ra.length || ob[j] >= rb.length) { P.push(`g.${t.table}[c${j}]`); rep(mod, oa[j] >= ra.length ? '(no row)' : 'a row', ob[j] >= rb.length ? '(no row)' : 'a row'); P.pop(); continue; } A.p = ra[oa[j]]; B.p = rb[ob[j]]; P.push(`g.${t.table}[c${j}]`); cmp(N, A, B, mod); P.pop(); }
      continue; }
    if (oa.length !== ob.length) rep(mod, String(oa.length), String(ob.length), '.reached'); P.pop();
    const c = Math.min(oa.length, ob.length); for (let j = 0; j < c; j++) { if (oa[j] >= ra.length || ob[j] >= rb.length) { P.push(`g.${t.table}[c${j}]`); rep(mod, oa[j] >= ra.length ? '(no row)' : 'a row', ob[j] >= rb.length ? '(no row)' : 'a row'); P.pop(); continue; } A.p = ra[oa[j]]; B.p = rb[ob[j]]; P.push(`g.${t.table}[c${j}]`); cmp(N, A, B, mod); P.pop(); }
    continue; }
  P.push(`g.${t.table}`); if (ra.length !== rb.length) rep(mod, String(ra.length), String(rb.length), '.len'); P.pop();
  const c = Math.min(ra.length, rb.length); for (let k = 0; k < c; k++) { A.p = ra[k]; B.p = rb[k]; P.push(`g.${t.table}[${k}]`); cmp(N, A, B, mod); P.pop(); }
  if (ra.length !== rb.length) { P.push(`g.${t.table}`); rep(mod, ra.length > c ? `${ra.length - c} more` : '(absent)', rb.length > c ? `${rb.length - c} more` : '(absent)', `[${c}..]`); P.pop(); } }
{ const a = A.dv.getUint32(A.serialAt, true), b = B.dv.getUint32(B.serialAt, true); if (a !== b) { P.push('g.serial_next'); rep('game', String(a), String(b)); P.pop(); } }
// --dynboxes: the frame's boxes (g.dyn_boxes) of each side counted by owner / flags / the key's family, and the kinds one side has more of
if (argv.includes('--dynboxes')) {
  const BT = SCHEMA.tables.find((t: any) => t.id === 'BoxId'), BN = T(parse(BT.ty).path), GT = T(SCHEMA.root);
  const field = (t: any, s: Side, at: number, name: string): [Node, number] => { s.p = at; const F = fieldsOf(t); for (let i = 0; i < F.length; i++) { if (t.fields[i].rust === name) return [F[i], s.p]; skip(F[i], s, null); } throw new Error('no field ' + name); };
  const count = (s: Side) => { const [n, at] = field(GT, s, 0, 'dyn_boxes'); s.p = at; const L = s.u32(), ids: number[] = []; for (let k = 0; k < L; k++) ids.push(s.u32()); const by = new Map<string, number>();
    for (const id of ids) { const [tb, r] = unmix('AnyBoxId', id), row = s.rows.get(tb)![r]; const wordO = (n: Node): string => n.k === 'opt' ? (s.u8() ? wordO(n.of) : 'None') : word(n, s); const g1 = (nm: string) => { const [fn, fa] = field(BN, s, row, nm); s.p = fa; return wordO(fn); }; const mk = g1('mkey'); const k = `owner ${g1('owner')} flags ${g1('flags')} key ${mk === '(…)' || mk === 'None' ? mk : (Number(mk) >>> 24)} name ${g1('name')} h ${g1('h').split(',').map(x => (+x).toFixed(3)).join('x')} glow ${g1('emissive').split(',').some(x => +x > 0) ? 'yes' : 'no'}`; by.set(k, (by.get(k) ?? 0) + 1); } return by; };
  const a = count(A), b = count(B); for (const k of new Set([...a.keys(), ...b.keys()])) if ((a.get(k) ?? 0) !== (b.get(k) ?? 0)) console.log(`  dyn boxes ${a.get(k) ?? 0} / ${b.get(k) ?? 0}: ${k}`);
}
let bytes = -1; { const i = argv.indexOf('--blocks'); if (i >= 0) { const s = readFileSync(argv[i + 1]), t = readFileSync(argv[i + 2]); bytes = 0; for (let k = 0; k < Math.max(s.length, t.length); k++) if ((s[k] ?? 0) !== (t[k] ?? 0)) bytes++; } }
console.log(`${n} paths differ, ${n - unknown} known, ${unknown} NOT KNOWN${unknown ? ': ' + [...by].sort((x, y) => y[1] - x[1]).map(([m, c]) => `${m} ${c}`).join(' · ') : ''}${bytes >= 0 ? `; block ${bytes} bytes` : ''}${A.leftN || B.leftN ? `; boxes left out as the picture's ${A.leftN} / ${B.leftN}` : ''}${serialN || scratchN || nullN || rowN ? `; by rule not compared: ${serialN} identity serials that differ, ${scratchN} scratch fields that differ${nullN ? `, ${nullN} null beside undefined` : ''}${rowN ? `, ${rowN} rows of poses of their own` : ''}` : ''}; boxes reached ${A.order.get('BoxId')?.length ?? '-'} / ${B.order.get('BoxId')?.length ?? '-'} of ${A.tableLen.get('BoxId')} / ${B.tableLen.get('BoxId')}; ${(process.memoryUsage().rss / 1e9).toFixed(2)} GB`);
if (STALE.size) { let x = 0, y = 0; for (const [a, b] of STALE.values()) { x += a; y += b; } console.log(`   by reach from the live game, counted and not reported (the TypeScript's / the Rust game's): ${x} / ${y} entries under a key that only a key reaches, or under a serial only one side holds: ${[...STALE].sort((p, q) => (q[1][0] + q[1][1]) - (p[1][0] + p[1][1])).slice(0, 60).map(([q, c]) => `${q} ${c[0]}/${c[1]}`).join(' · ')}`); }
if (argv.includes('--why')) for (const k of KNOWN) if (k.n) console.log(`   known ${k.n}: ${k.why.slice(0, 150)}`);
for (const x of shown) console.log('     ' + x.slice(0, 230));
if (OUT) writeFileSync(OUT, all.join('\n') + '\n');
process.exit(unknown ? 1 : 0);   // (the block's bytes are the runner's alarm, part by part and after every tick: here they are only counted)
