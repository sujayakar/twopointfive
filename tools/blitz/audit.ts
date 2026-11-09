// THE AUDIT'S WALKS (PLAY.md): where two whole states of the game differ, field by field, by the bridge's schema. The headless host
// (rust-host.ts) uses them to say what TypeScript wrote into the puppet between two ticks; nothing here writes a game.
import { SCHEMA, parse } from './export.ts';
// ---- where in a state a byte is: the bridge's walk, entered only where one of the asked places lies. A value of a fixed width that holds none is stepped over.
export type N = any;
const WID = new Map<N, number>();
function width(n: N): number { let w = WID.get(n); if (w !== undefined) return w;
  switch (n.k) { case 'f64': case 'u64': case 'i64': w = 8; break; case 'f32': case 'u32': case 'i32': case 'usize': case 'id': w = 4; break; case 'bool': case 'u8': case 'i8': w = 1; break; case 'u16': case 'i16': w = 2; break; case 'unit': w = 0; break;
    case 'arr': { const e = width(n.of); w = e < 0 ? -1 : e * n.n; break; }
    case 'tup': { w = 0; for (const x of n.of) { const e = width(x); if (e < 0) { w = -1; break; } w += e; } break; }
    case 'type': { const T = SCHEMA.types[n.path]; if (T.k === 'enum') w = 2; else if (T.k === 'struct') { w = 0; WID.set(n, -1); for (const f of T.fields) { const e = width(parse(f.ty)); if (e < 0) { w = -1; break; } w += e; } } else w = -1; break; }
    default: w = -1; }
  WID.set(n, w!); return w!; }
export type Place = { off: number; path: string; q: string };
export type Span = { q: string; s: number; e: number };
// FIELDS THAT ARE EACH SIDE'S OWN, cut out of every compare and never handed across (PLAY.md says why for each): the box mirror is a module's own copy of the
// box world with its own counters; the other three the export works out or reads from a module variable that the apply does not write back, so the puppet's
// are not the Rust game's. All four stand in the game's own fields (none in a table's row), so one walk of the root finds them.
export const CUT = new Set<string>();   // (none since the bridge's echo: a field the Rust game owns and TypeScript cannot hold is kept beside the object by a whole apply and written back by the export, tools/blitz/IMPORT.md)
/** the field each of `offs` (ascending) lies in: its path in the state and the struct field that declares it. `spans`: where each CUT field stands; with
 *  `rootOnly` the walk ends with the game's own fields (the tables are not entered) */
export function locate(b: Uint8Array, offs: number[], spans: Span[] | null = null, rootOnly = false, rootTy: string = SCHEMA.root, rootPath = 'g'): Place[] {
  const dv = new DataView(b.buffer, b.byteOffset, b.byteLength), out: Place[] = []; let p = 0, ti = 0;
  const take = (end: number, path: string, q: string) => { let first = true; while (ti < offs.length && offs[ti] < end) { if (first) out.push({ off: offs[ti], path, q }); first = false; ti++; } };
  const go = (n: N, path: string, q: string): void => {
    if (p > b.length) throw new Error(`play: the walk ran past the state's end at ${path}`);
    const w = width(n);
    if (w >= 0 && (ti >= offs.length || offs[ti] >= p + w)) { p += w; return; }
    switch (n.k) {
      case 'str': { const L = dv.getUint32(p, true); take(p + 4 + L, path, q); p += 4 + L; return; }
      case 'opt': { take(p + 1, path, q); if (b[p++]) go(n.of, path, q); return; }
      case 'vec': case 'arr': { let L = n.n; if (n.k === 'vec') { L = dv.getUint32(p, true); take(p + 4, path + '.len', q); p += 4; } const e = width(n.of);
        if (e >= 0) { const base = p, end = base + L * e; while (ti < offs.length && offs[ti] < end) { const i = Math.floor((offs[ti] - base) / e); p = base + i * e; go(n.of, `${path}[${i}]`, q); } p = end; return; }
        for (let i = 0; i < L; i++) go(n.of, `${path}[${i}]`, q); return; }
      case 'tup': n.of.forEach((x: N, i: number) => go(x, `${path}.${i}`, q)); return;
      case 'map': { const L = dv.getUint32(p, true); take(p + 4, path + '.len', q); p += 4; for (let i = 0; i < L; i++) { go(n.K, `${path}{#${i}}`, q); go(n.V, `${path}{#${i}}`, q); } return; }
      case 'set': { const L = dv.getUint32(p, true); take(p + 4, path + '.len', q); p += 4; for (let i = 0; i < L; i++) go(n.K, `${path}{#${i}}`, q); return; }
      case 'type': { const T = SCHEMA.types[n.path];
        if (T.k === 'struct') { for (const f of T.fields) { const fq = `${T.mod}::${T.name}.${f.rust}`; if (spans && CUT.has(fq)) { const s = p; go(parse(f.ty), `${path}.${f.rust}`, fq); spans.push({ q: fq, s, e: p }); } else go(parse(f.ty), `${path}.${f.rust}`, fq); } return; }
        if (T.k === 'enum') { take(p + 2, path, q); p += 2; return; }
        const a = dv.getUint16(p, true); take(p + 2, path + '#', q); p += 2; if (T.k === 'wordEnum') { for (const f of T.arms[a].fields) go(parse(f.ty), `${path}.${f.rust}`, q); return; }
        const t = armTy(T, a); if (t) go(parse(t), path, q); return; }
      default: take(p + w, path, q); p += w; return; }
  };
  go(parse(rootTy), rootPath, '');
  if (rootOnly) return out;
  for (const t of SCHEMA.tables) { if (ti >= offs.length) break; const L = dv.getUint32(p, true); take(p + 4, `g.${t.table}.len`, `table ${t.id}`); p += 4; const Nn = parse(t.ty); for (let i = 0; i < L; i++) go(Nn, `g.${t.table}[${i}]`, `table ${t.id}`); }
  take(b.length, 'g.<the tail: the serial counter and the loose poses>', 'tail');
  return out; }
export const buf = (b: Uint8Array) => Buffer.from(b.buffer, b.byteOffset, b.byteLength);
/** where two states of one length differ: the first byte of every run of differing bytes, at most `cap` */
export function diffOffsets(a: Uint8Array, b: Uint8Array, cap = 4000): number[] { const out: number[] = []; let run = false; for (let i = 0; i < a.length && out.length < cap; i++) { if (a[i] !== b[i]) { if (!run) out.push(i); run = true; } else run = false; } return out; }
export type Tally = Map<string, { n: number; eg: string; ticks: number; last: number }>;
export const tallyAdd = (T: Tally, q: string, eg: string, k: number) => { const e = T.get(q); if (e) { e.n++; if (e.last !== k) { e.ticks++; e.last = k; } } else T.set(q, { n: 1, eg, ticks: 1, last: k }); };
const spanMemo = new WeakMap<Uint8Array, Span[]>();
export const spansOf = (b: Uint8Array): Span[] => { let s = spanMemo.get(b); if (!s) { s = []; locate(b, [], s, true); spanMemo.set(b, s); } return s; };
const armTy = (T: any, a: number): string | null => T.k === 'data' ? T.vars[a].ty ?? null : (T.arms[a].match(/\((.*)\)/)?.[1] ?? null);
export const fieldsOf = (T: any): { N: N; q: string; dot: string }[] => T.__play ??= T.fields.map((f: any) => ({ N: parse(f.ty), q: `${T.mod}::${T.name}.${f.rust}`, dot: `.${f.rust}` }));
/** where the value of type `n` that starts at `p` ends */
export function skipIn(b: Uint8Array, dv: DataView, p: number, n: N): number {
  const w = width(n); if (w >= 0) return p + w;
  switch (n.k) {
    case 'str': return p + 4 + dv.getUint32(p, true);
    case 'opt': return b[p] ? skipIn(b, dv, p + 1, n.of) : p + 1;
    case 'vec': { const L = dv.getUint32(p, true); p += 4; const e = width(n.of); if (e >= 0) return p + L * e; for (let i = 0; i < L; i++) p = skipIn(b, dv, p, n.of); return p; }
    case 'arr': for (let i = 0; i < n.n; i++) p = skipIn(b, dv, p, n.of); return p;
    case 'tup': for (const x of n.of) p = skipIn(b, dv, p, x); return p;
    case 'map': { const L = dv.getUint32(p, true); p += 4; for (let i = 0; i < L; i++) { p = skipIn(b, dv, p, n.K); p = skipIn(b, dv, p, n.V); } return p; }
    case 'set': { const L = dv.getUint32(p, true); p += 4; for (let i = 0; i < L; i++) p = skipIn(b, dv, p, n.K); return p; }
    case 'type': { const T = SCHEMA.types[n.path]; if (T.k === 'struct') { for (const f of fieldsOf(T)) p = skipIn(b, dv, p, f.N); return p; }
      const a = dv.getUint16(p, true); p += 2; if (T.k === 'wordEnum') { for (const f of T.arms[a].fields) p = skipIn(b, dv, p, parse(f.ty)); return p; }
      const t = armTy(T, a); return t ? skipIn(b, dv, p, parse(t)) : p; } }
  throw new Error(`play: no way past ${n.k}`); }
const LEAF = new Set(['f64', 'u64', 'i64', 'f32', 'u32', 'i32', 'usize', 'id', 'bool', 'u8', 'i8', 'u16', 'i16']);
/** THE FIELDS IN WHICH STATE `b` DIFFERS FROM `a`, each with an example path, the CUT fields apart (`cut`: which of those differ, by name). The two states
 *  are walked side by side by the bridge's schema: a stretch of a fixed width that is the same bytes on both sides is stepped over, a list of two lengths
 *  is named by its length and compared over the items both have. Nothing is decoded to text unless it differs, so a compare of two states of megabytes
 *  holds no more than the two states. */
export function fieldsDiffering(a: Uint8Array, b: Uint8Array, cut: string[] | null = null, cap = 2000): { q: string; path: string }[] {
  const out: { q: string; path: string }[] = []; if (a.length === b.length && buf(a).equals(buf(b))) return out;
  const da = new DataView(a.buffer, a.byteOffset, a.byteLength), db = new DataView(b.buffer, b.byteOffset, b.byteLength); let pa = 0, pb = 0; const st: (string | number)[] = [];
  const at = () => 'g' + st.map(x => typeof x === 'number' ? `[${x}]` : x).join('');
  const add = (q: string, path: string) => { if (out.length < cap) out.push({ q, path }); };
  const eq = (x: number, y: number, w: number): boolean => { if (w <= 24) { for (let i = 0; i < w; i++) if (a[x + i] !== b[y + i]) return false; return true; } return x + w <= a.length && y + w <= b.length && buf(a.subarray(x, x + w)).equals(buf(b.subarray(y, y + w))); };
  const go = (n: N, q: string): void => {
    const w = width(n);
    if (w >= 0) { if (eq(pa, pb, w)) { pa += w; pb += w; return; } if (LEAF.has(n.k) || (n.k === 'type' && SCHEMA.types[n.path].k === 'enum')) { add(q, at()); pa += w; pb += w; return; } }
    switch (n.k) {
      case 'str': { const la = da.getUint32(pa, true), lb = db.getUint32(pb, true); if (la !== lb || !eq(pa + 4, pb + 4, la)) add(q, at()); pa += 4 + la; pb += 4 + lb; return; }
      case 'opt': { const ta = a[pa++], tb = b[pb++]; if (ta !== tb) { add(q, `${at()} ${ta ? 'Some | None' : 'None | Some'}`); if (ta) pa = skipIn(a, da, pa, n.of); if (tb) pb = skipIn(b, db, pb, n.of); return; } if (ta) go(n.of, q); return; }
      case 'vec': case 'arr': { let la = n.n, lb = n.n; if (n.k === 'vec') { la = da.getUint32(pa, true); lb = db.getUint32(pb, true); pa += 4; pb += 4; if (la !== lb) add(q, `${at()}.len ${la} | ${lb}`); }
        const m = Math.min(la, lb), e = width(n.of); if (e >= 0 && eq(pa, pb, m * e)) { pa += la * e; pb += lb * e; return; }
        for (let i = 0; i < m; i++) { st.push(i); go(n.of, q); st.pop(); }
        for (let i = m; i < la; i++) pa = skipIn(a, da, pa, n.of); for (let i = m; i < lb; i++) pb = skipIn(b, db, pb, n.of); return; }
      case 'tup': n.of.forEach((x: N, i: number) => { st.push(`.${i}`); go(x, q); st.pop(); }); return;
      case 'map': case 'set': { const la = da.getUint32(pa, true), lb = db.getUint32(pb, true); pa += 4; pb += 4; if (la !== lb) add(q, `${at()}.len ${la} | ${lb}`); const m = Math.min(la, lb);
        for (let i = 0; i < m; i++) { st.push(`{#${i}}`); go(n.K, q); if (n.k === 'map') go(n.V, q); st.pop(); }
        for (let i = m; i < la; i++) { pa = skipIn(a, da, pa, n.K); if (n.k === 'map') pa = skipIn(a, da, pa, n.V); } for (let i = m; i < lb; i++) { pb = skipIn(b, db, pb, n.K); if (n.k === 'map') pb = skipIn(b, db, pb, n.V); } return; }
      case 'type': { const T = SCHEMA.types[n.path];
        if (T.k === 'struct') { for (const f of fieldsOf(T)) {
          if (CUT.has(f.q)) { const sa = pa, sb = pb; pa = skipIn(a, da, pa, f.N); pb = skipIn(b, db, pb, f.N); if (cut && (pa - sa !== pb - sb || !eq(sa, sb, pa - sa))) cut.push(f.q); continue; }
          st.push(f.dot); go(f.N, f.q); st.pop(); } return; }
        const xa = da.getUint16(pa, true), xb = db.getUint16(pb, true); pa += 2; pb += 2;
        // (a record told apart by a word of its own, each arm with fields of its own: a lesson step's test)
        if (T.k === 'wordEnum') { if (xa !== xb) { add(q, `${at()}# arm ${xa} | ${xb}`); for (const f of T.arms[xa].fields) pa = skipIn(a, da, pa, parse(f.ty)); for (const f of T.arms[xb].fields) pb = skipIn(b, db, pb, parse(f.ty)); return; } for (const f of T.arms[xa].fields) { st.push(`.${f.rust}`); go(parse(f.ty), q); st.pop(); } return; }
        const ta = armTy(T, xa), tb = armTy(T, xb);
        if (xa !== xb) { add(q, `${at()}# arm ${xa} | ${xb}`); if (ta) pa = skipIn(a, da, pa, parse(ta)); if (tb) pb = skipIn(b, db, pb, parse(tb)); return; }
        if (ta) go(parse(ta), q); return; } }
    throw new Error(`play: no compare of ${n.k}`); };
  go({ k: 'type', path: SCHEMA.root }, '');
  for (const t of SCHEMA.tables) { const la = da.getUint32(pa, true), lb = db.getUint32(pb, true), Nn = parse(t.ty), q = `table ${t.id}`; pa += 4; pb += 4; if (la !== lb) add(q, `g.${t.table}.len ${la} | ${lb}`); const m = Math.min(la, lb);
    st.length = 0; st.push(`.${t.table}`); for (let i = 0; i < m; i++) { st.push(i); go(Nn, q); st.pop(); }
    for (let i = m; i < la; i++) pa = skipIn(a, da, pa, Nn); for (let i = m; i < lb; i++) pb = skipIn(b, db, pb, Nn); }
  // (the tail: the serial counter, then the floats of poses outside the pool)
  if (a.length - pa !== b.length - pb || !eq(pa, pb, a.length - pa)) add('tail', 'g.<the serial counter and the poses outside the pool>');
  return out; }
/** state `b` with every CUT field as state `a` has it: what the Rust game is handed, so that it keeps its own of those */
export function withCutsOf(b: Uint8Array, a: Uint8Array): Uint8Array { const A = spansOf(a), Bs = spansOf(b), parts: Uint8Array[] = []; let pb = 0;
  for (let i = 0; i < Bs.length; i++) { parts.push(b.subarray(pb, Bs[i].s), a.subarray(A[i].s, A[i].e)); pb = Bs[i].e; } parts.push(b.subarray(pb));
  const out = new Uint8Array(parts.reduce((n, x) => n + x.length, 0)); let o = 0; for (const x of parts) { out.set(x, o); o += x.length; } return out; }
// THE HOST'S HALF, as paths of a state: what `face_host` hands over every tick, so no write-back is owed for it
// (… and what `Game.update`'s hostAfter sets after every step and the tick sets again as it opens: the seat's live matrix, the menu's view of the host's own
// Input and camera)
export const HOST_Q = (x: { q: string; path: string }) => x.path.startsWith('g.cmd_buf') || /^g\.tab\.(camera|input)\b/.test(x.path) || x.q === 'orders::SquadOrdersView.input' || x.q === 'orders::SquadOrdersView.cam' || x.q === 'game::Game.view_proj' || x.q === 'cmd::Seat.mark';
// NOT A WRITE: an identity serial (the two sides number objects apart by design: schema.serial), `Player.kit_was` (the apply leaves the puppet's at nothing where the Rust game holds one: seen on tut-solo every tick), and the counts of collision and sense asks (collision.ts ANYTIME, SENSED: counters of the module that was asked, read by no rule)
export const OWN_Q = (x: { q: string; path: string }) => !!SCHEMA.serial?.[x.q] || !!SCHEMA.scratch?.[x.q] || x.q === 'tail' || /^scene_collision::SceneCollisionMod(ANYTIME|SENSED)\./.test(x.q);
