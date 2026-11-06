// THE HEADLESS HOST OF THE RUST GAME (PLAY.md): a mission played by tools/qa's scripted player, where THE RUST GAME IS THE ONLY GAME THAT STEPS and the
// TypeScript `Game` is a PUPPET: the state the Rust game exports after each step is applied onto it in place (import.ts), the scripted player reads the
// puppet to decide its keys, and what it presses goes to the Rust game as the tick's record. The TypeScript tick (`stepSeats`' body) never runs.
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/play.ts <mission>:<seed> --ref DIR [--out DIR] [--ticks N] [--wasm FILE] [--baseline puppet|rust]
//                                                                 [--apply-check N] [--upload-every N] [--no-dress] [--go-on] [--quiet] [--lesson-json]
// --ref DIR is the same mission and seed played by the plain TypeScript game (mission.ts wrote it: post.txt, the die's count and a hash of each part of
// rust/sim's block after every tick). AFTER EVERY TICK the Rust game's die and parts are held to that line. If the puppet shows the scripted player what
// the TypeScript game would have shown, the player presses the same keys and the two stay equal to the mission's end. The first tick that parts is the
// stop (--go-on plays on), and the puppet as that tick opened is written to --out for diff.ts against the TypeScript's state of the same tick.
//
// A TICK, in the order things happen:
//   the scripted player   presses its keys, moves the mouse, gives orders through the game's own calls, and steps its world (headless.ts `W.step`)
//   `Game.update`         the host's lines round a step run as they are (the record sampled from the Input and the camera, hostBefore, hostAfter); the
//                         step itself is this file's `puppetStep`, hung on the game object in place of `stepSeats`
//   the write-back        the puppet is exported and held to the bytes it had when the last apply ended. A difference outside the host's half is
//                         something TypeScript wrote into the game by name between two ticks (the player's own writes, an order's rows): the Rust game
//                         takes the puppet's state (`face_import_into`). rust/sim's block is held and handed over the same way
//   the host's half       `face_host`: the record, every camera and Input row (the bytes mission.ts writes as h<k>.bin)
//   `face_step`           the tick; then the die and the parts' hashes are read and held to the reference; then the lights' update, the host's view of a
//                         level on rails and, on the ticks the headless host lists the box world, the listing, all on the Rust side
//   the apply             `face_export` (the WHOLE state) onto the puppet, and the Rust game's block over the puppet's
//   the dress step        what the tick does that has no Rust body and that a reader of the puppet needs: the par clock, the verdict, the autosave
// Exit: 0 equal to the end and ended the same way · 1 parted · 2 the Rust tick panicked · 3 a side could not run · 4 equal every tick, the end differs.
import '../qa/stand.ts';
import { readFileSync, writeFileSync, existsSync, mkdirSync, appendFileSync } from 'node:fs';
const argv = process.argv.slice(2), has = (n: string) => argv.includes(n), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const say = (s = '') => process.stdout.write(s + '\n');
if (!argv[0] || argv[0].startsWith('--')) { say('play.ts <mission>:<seed> --ref DIR [--out DIR] [--ticks N] [--wasm FILE] [--baseline puppet|rust] [--apply-check N] [--upload-every N] [--no-dress] [--go-on] [--quiet]'); process.exit(64); }
const [scn, seedS] = argv[0].split(':'), seed = +(seedS ?? 1);
const REF = arg('--ref', ''), OUT = arg('--out', REF ? `${REF}/play` : ''), TICKS = +arg('--ticks', '1000000'), DIE = +arg('--die', '20942');
const BASE = arg('--baseline', 'rust'), APPLY_CHECK = +arg('--apply-check', '50'), ONLY = !has('--apply-all'), UP_EVERY = +arg('--upload-every', '4'), DRESS = !has('--no-dress'), GO_ON = has('--go-on'), QUIET = has('--quiet');
const WASM = arg('--wasm', new URL('../../rust/game_face/target/wasm32-unknown-unknown/release/game_face.wasm', import.meta.url).pathname);
if (!existsSync(WASM)) { say(`play: no module at ${WASM} (cd rust/game_face && cargo build --target wasm32-unknown-unknown --release)`); process.exit(3); }
if (OUT) { mkdirSync(OUT, { recursive: true }); for (const f of ['writeback.txt', 'gives.txt']) writeFileSync(`${OUT}/${f}`, ''); }
const LESSON = scn.startsWith('tut-');
if (LESSON) process.env.TUT_SQUAD_SKY = '0';   // (as mission.ts stands a tutorial: the estimate as every other tool has it)

// ---- the reference: the plain TypeScript play of the same mission and seed
const refLines = (n: string) => REF && existsSync(`${REF}/${n}`) ? readFileSync(`${REF}/${n}`, 'utf8').split('\n').filter(Boolean) : [];
const post = refLines('post.txt').map(l => l.split(' ')), head = post.shift() ?? [];
const refEnd = refLines('ts.log').find(l => l.startsWith('TypeScript:')) ?? '(no line)';
if (!post.length) say('play: NO REFERENCE (--ref DIR of a mission.ts run): the Rust game is played and NOTHING IS COMPARED');

// ---- the module
const mod = new WebAssembly.Module(readFileSync(WASM));
const X: any = new WebAssembly.Instance(mod, {}).exports, mem: WebAssembly.Memory = X.memory;
const u8 = () => new Uint8Array(mem.buffer), u32 = () => new Uint32Array(mem.buffer);
const dec = new TextDecoder(), enc = new TextEncoder();
const outText = () => dec.decode(u8().subarray(X.face_out_ptr(), X.face_out_ptr() + X.face_out_len()));
const panicText = () => { try { return dec.decode(u8().subarray(X.face_panic_ptr(), X.face_panic_ptr() + X.face_panic_len())); } catch { return ''; } };
function hand<T>(b: Uint8Array, f: (p: number, n: number) => T): T { const p = X.face_alloc(b.length); u8().set(b, p); try { return f(p, b.length); } finally { X.face_free(p, b.length); } }
const NAMES: string[] = []; for (let i = 0; i < X.face_parts(); i++) { X.face_part_name(i); NAMES.push(outText()); }
if (post.length && (head[0] !== 'parts' || head.slice(1).join(' ') !== NAMES.join(' '))) { say(`play: the reference names the block's parts [${head.slice(1)}] and this module [${NAMES}]: one side is of another tree`); process.exit(3); }
const POSES = NAMES.indexOf('poses');
const partHashes = (slot: number): number[] => { const at = X.face_part_hashes(slot) >>> 2; return Array.from(u32().subarray(at, at + NAMES.length)); };
// (room asked for and given back before the game is made: the allocator keeps it, so a step need not grow the memory to find it)
{ const n = 48 * 1048576, p = X.face_alloc(n); X.face_free(p, n); }

// ---- the bridge and the game
const { exportGame, exportPart, fieldAt, bookOf, SCHEMA, parse, hostHalfParts } = await import('./export.ts');
const { applyGame } = await import('./import.ts');
const { installAsks } = await import('./ask.ts');
const { classTable, valueTable } = await import('./classes.ts');
const { knobRegistry } = await import('../../src/game/knobs.ts');
const { Game } = await import('../../src/game/game.ts');
const { play, entryOf } = await import('../qa/playthrough.ts');
const { newDice } = await import('../../src/math/dice.ts');
const { dressAll } = await import('../../src/game/dress.ts');
if (!LESSON && !entryOf(scn)) { say(`play: the scripted player has no mission '${scn}'`); process.exit(64); }
const reg = knobRegistry(), HOOKS = { classes: classTable(), values: valueTable() };
// THE APPLY LEAVES THE BOX MIRROR ALONE: its fields describe the copy of the box world in the module that holds it (the TypeScript game's is another module
// than the Rust game's), so the puppet keeps its own and rebuilds it when the apply says the boxes were written (PUT_BACKS)
const APPLY_ONLY = new Set<string>(); for (const T0 of Object.values<any>(SCHEMA.types)) if (T0.k === 'struct' && T0.mod !== 'scene_box_mirror') for (const f of T0.fields) APPLY_ONLY.add(`${T0.mod}::${T0.name}.${f.rust}`);
class Stop extends Error {}
const NO_ASKS = has('--no-asks'); let asks: { after(): void } | null = null;
const HANDBACK = +arg('--handback', '0'); let handbacks = 0, handbacksInBeat = 0;

// ---- where in a state a byte is: the bridge's walk, entered only where one of the asked places lies. A value of a fixed width that holds none is stepped over.
type N = any;
const WID = new Map<N, number>();
function width(n: N): number { let w = WID.get(n); if (w !== undefined) return w;
  switch (n.k) { case 'f64': case 'u64': case 'i64': w = 8; break; case 'f32': case 'u32': case 'i32': case 'usize': case 'id': w = 4; break; case 'bool': case 'u8': case 'i8': w = 1; break; case 'u16': case 'i16': w = 2; break; case 'unit': w = 0; break;
    case 'arr': { const e = width(n.of); w = e < 0 ? -1 : e * n.n; break; }
    case 'tup': { w = 0; for (const x of n.of) { const e = width(x); if (e < 0) { w = -1; break; } w += e; } break; }
    case 'type': { const T = SCHEMA.types[n.path]; if (T.k === 'enum') w = 2; else if (T.k === 'struct') { w = 0; WID.set(n, -1); for (const f of T.fields) { const e = width(parse(f.ty)); if (e < 0) { w = -1; break; } w += e; } } else w = -1; break; }
    default: w = -1; }
  WID.set(n, w!); return w!; }
type Place = { off: number; path: string; q: string };
type Span = { q: string; s: number; e: number };
// FIELDS THAT ARE EACH SIDE'S OWN, cut out of every compare and never handed across (PLAY.md says why for each): the box mirror is a module's own copy of the
// box world with its own counters; the other three the export works out or reads from a module variable that the apply does not write back, so the puppet's
// are not the Rust game's. All four stand in the game's own fields (none in a table's row), so one walk of the root finds them.
const CUT = new Set<string>();
/** the field each of `offs` (ascending) lies in: its path in the state and the struct field that declares it. `spans`: where each CUT field stands; with
 *  `rootOnly` the walk ends with the game's own fields (the tables are not entered) */
function locate(b: Uint8Array, offs: number[], spans: Span[] | null = null, rootOnly = false, rootTy: string = SCHEMA.root, rootPath = 'g'): Place[] {
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
const buf = (b: Uint8Array) => Buffer.from(b.buffer, b.byteOffset, b.byteLength);
/** where two states of one length differ: the first byte of every run of differing bytes, at most `cap` */
function diffOffsets(a: Uint8Array, b: Uint8Array, cap = 4000): number[] { const out: number[] = []; let run = false; for (let i = 0; i < a.length && out.length < cap; i++) { if (a[i] !== b[i]) { if (!run) out.push(i); run = true; } else run = false; } return out; }
type Tally = Map<string, { n: number; eg: string; ticks: number; last: number }>;
const tallyAdd = (T: Tally, q: string, eg: string, k: number) => { const e = T.get(q); if (e) { e.n++; if (e.last !== k) { e.ticks++; e.last = k; } } else T.set(q, { n: 1, eg, ticks: 1, last: k }); };
const spanMemo = new WeakMap<Uint8Array, Span[]>();
const spansOf = (b: Uint8Array): Span[] => { let s = spanMemo.get(b); if (!s) { s = []; locate(b, [], s, true); spanMemo.set(b, s); } return s; };
const armTy = (T: any, a: number): string | null => T.k === 'data' ? T.vars[a].ty ?? null : (T.arms[a].match(/\((.*)\)/)?.[1] ?? null);
const fieldsOf = (T: any): { N: N; q: string; dot: string }[] => T.__play ??= T.fields.map((f: any) => ({ N: parse(f.ty), q: `${T.mod}::${T.name}.${f.rust}`, dot: `.${f.rust}` }));
/** where the value of type `n` that starts at `p` ends */
function skipIn(b: Uint8Array, dv: DataView, p: number, n: N): number {
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
function fieldsDiffering(a: Uint8Array, b: Uint8Array, cut: string[] | null = null, cap = 2000): { q: string; path: string }[] {
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
function withCutsOf(b: Uint8Array, a: Uint8Array): Uint8Array { const A = spansOf(a), Bs = spansOf(b), parts: Uint8Array[] = []; let pb = 0;
  for (let i = 0; i < Bs.length; i++) { parts.push(b.subarray(pb, Bs[i].s), a.subarray(A[i].s, A[i].e)); pb = Bs[i].e; } parts.push(b.subarray(pb));
  const out = new Uint8Array(parts.reduce((n, x) => n + x.length, 0)); let o = 0; for (const x of parts) { out.set(x, o); o += x.length; } return out; }
// THE HOST'S HALF, as paths of a state: what `face_host` hands over every tick, so no write-back is owed for it
// (… and what `Game.update`'s hostAfter sets after every step and the tick sets again as it opens: the seat's live matrix, the menu's view of the host's own
// Input and camera)
const HOST_Q = (x: { q: string; path: string }) => x.path.startsWith('g.cmd_buf') || /^g\.tab\.(camera|input)\b/.test(x.path) || x.q === 'orders::SquadOrdersView.input' || x.q === 'orders::SquadOrdersView.cam' || x.q === 'game::Game.view_proj';
// NOT A WRITE: an identity serial (the two sides number objects apart by design: schema.serial), a clip's place in the Rust game's own table (the export
// writes a constant for it), `Player.kit_was` (the apply leaves the puppet's at nothing where the Rust game holds one: seen on tut-solo every tick), and the counts of collision and sense asks (collision.ts ANYTIME, SENSED: counters of the module that was asked, read by no rule)
const OWN_Q = (x: { q: string; path: string }) => !!SCHEMA.serial?.[x.q] || !!SCHEMA.scratch?.[x.q] || x.q === 'tail' || /^scene_collision::SceneCollisionMod(ANYTIME|SENSED)\./.test(x.q);

// ---- the run's books
let k = 0, h = 0, slot = 0, lastState: Uint8Array | null = null, lastBlock: Uint8Array | null = null, lastRust: Uint8Array | null = null;
let hostBytes = 0;
const T = { hostHold: 0, driver: 0, hostTs: 0, wbExport: 0, wbCompare: 0, wbImport: 0, hostHalf: 0, step: 0, after: 0, export: 0, apply: 0, block: 0, check: 0, dress: 0 };
let worstStep = 0, worstStepAt = 0, worstApply = 0, tLeft = 0, outEvents = 0, outTicks = 0;
const WB: Tally = new Map(), HOSTW: Tally = new Map(), OWNW: Tally = new Map(), CUTW = new Map<string, number>(), APPLY_MISS: Tally = new Map(), BLOCKW = new Map<string, number>(), CALLS = new Map<string, number>();
const WB_REFUSED: Tally = new Map(); let wbRefused = 0;
let wbFired = 0, wbMeter = 0, wbBlock = 0, wbHostOnly = 0, wbNone = 0, applyChecks = 0, applyUnequal = 0, uploadsRust = 0, uploadsLate = 0, uploadsMissed = 0, made: Record<string, number> = {};
const noWay = new Map<string, number>(), noSetter = new Map<string, number>(), noWindow = new Set<string>();
const S = { parted: 0, poolParted: 0, dieParted: 0, partedIn: [] as string[], compared: 0 };
const count = (name: string) => CALLS.set(name, (CALLS.get(name) ?? 0) + 1);
const now = () => performance.now();
const tsBlock = (g: any): Uint8Array => { const w: Uint32Array = g.sim.w; return new Uint8Array(w.buffer, w.byteOffset, w.byteLength); };
const rustBlock = (): Uint8Array => u8().subarray(X.state_addr(slot), X.state_addr(slot) + X.state_len());
/** the host's half, the bytes handed over, held to the plain play's (mission.ts h<k>.bin) */
function hostHalfHeld(g: any, cmds: readonly any[]): Uint8Array { const bk: any = bookOf(g), P = hostHalfParts(g, cmds);
  const mine = exportPart(g, reg, P);
  // WHAT THE SCRIPTED PLAYER PRESSED, held to what it pressed in the plain TypeScript play (the reference's h<k>.bin, where the reference kept them): the
  // first tick the two differ is the first tick the puppet showed the player something the TypeScript game did not, whatever the block says by then
  if (REF && !hostParted && existsSync(`${REF}/h${k}.bin`)) { const ref = new Uint8Array(readFileSync(`${REF}/h${k}.bin`)); hostCompared++;
    if (ref.length !== mine.length || !buf(ref).equals(buf(mine))) { hostParted = k; const where: string[] = []; let at = 0;
      if (ref.length !== mine.length) where.push(`the lengths differ (${mine.length} here, ${ref.length})`);
      // (piece by piece: the record, then each camera row, then each Input row; a piece of one length on both sides is named field by field)
      else for (let i = 0; i < P.length; i++) { const n = exportPart(g, reg, [P[i]]).length, a = ref.subarray(at, at + n), b = mine.subarray(at, at + n); at += n; if (P[i][0] === 'u32' || buf(a).equals(buf(b))) continue;
        const name = P[i][0].endsWith('PlayerCmd') ? `record[${i - 1}]` : `${P[i][0].split('::').pop()}[${bk.listOf(P[i][0].endsWith('FollowCamera') ? 'CameraId' : 'InputId').indexOf(P[i][1])}]`;
        for (const x of locate(b, diffOffsets(a, b), null, true, P[i][0], name)) { const dv = (u: Uint8Array) => new DataView(u.buffer, u.byteOffset + x.off, Math.min(8, u.length - x.off)); const f = (u: Uint8Array) => { try { return dv(u).getFloat64(0, true); } catch { return NaN; } }; where.push(`${x.path} (${f(b)} here, ${f(a)})`); } }
      if (OUT) writeFileSync(`${OUT}/host${k}.bin`, mine);
      say(`TICK ${k}: THE HOST'S HALF PARTS from the plain TypeScript play's (what the player pressed, where its camera stood): ${where.slice(0, 40).join(' · ')}`); } }
  return mine; }
let hostParted = 0, hostCompared = 0;
/** the Rust game's whole state onto the puppet, and its block over the puppet's */
function applyRust(g: any): Uint8Array {
  let t = now(); X.face_export(h); const blob = u8().slice(X.face_out_ptr(), X.face_out_ptr() + X.face_out_len()); T.export += now() - t;
  t = now(); const R = applyGame(g, blob, reg, { hooks: HOOKS }); const el = now() - t; T.apply += el; if (el > worstApply) worstApply = el;
  asks?.after();   // (every order row the apply made, and the posture, ask the Rust game: ask.ts)
  for (const [q, n] of R.noWay) noWay.set(q, (noWay.get(q) ?? 0) + n); for (const [q, n] of R.noSetter) noSetter.set(q, (noSetter.get(q) ?? 0) + n); for (const q of R.noWindow) noWindow.add(q);
  for (const [c, n] of Object.entries(R.made)) if (n) made[c] = (made[c] ?? 0) + n;
  t = now(); const mine = tsBlock(g), theirs = rustBlock(); mine.set(theirs.subarray(0, mine.length)); lastBlock = mine.slice(); T.block += now() - t;
  lastRust = blob; return blob; }

// ---- the taps: what the scripted player calls between ticks that WRITES the game and is no input. Each still runs, on the puppet, and is counted by name;
// the write-back hands what it wrote to the Rust game.
function taps(g: any) {
  // THE SCRIPTED PLAYER'S ORDERS GO TO THE RUST GAME, not through the menu's TypeScript: a row of the menu is closures (`can`, `issue`, `label`), and a row
  // the Rust game registered inside a tick (silent.ts, doortools.ts, pairorders.ts and others register rows as the fight goes) arrives on the puppet with
  // none, so `def.issue` on the puppet would be no function. The order is given on the Rust side at once, in the words the replay loop gives it
  // (rust/game_run `give`), and the puppet takes the state again so the player reads what the order did before the next tick. What TypeScript wrote
  // since the last apply is handed over first. An order the face has no words for (a wall, a thing, options) still runs on the puppet and is written back.
  const O = g.orders, give = O.give; O.give = function (this: any, id: string, t: any, who: any, opts: any) {
    const T0 = t ?? this.targetAt(g.aimPoint), w = who === undefined ? '-' : typeof who === 'object' ? `g${g.guards.indexOf(who)}` : String(who);
    const tail = T0.kind === 'door' ? ` door ${g.doors.list.indexOf(T0.door)} ${T0.side} book${bookOf(g).ids.get('DoorId')?.get(T0.door)}` : T0.kind === 'none' && T0.man ? ` man ${g.guards.indexOf(T0.man)}` : T0.kind === 'foe' ? ` man ${g.guards.indexOf(T0.foe)}` : (T0.wall || T0.thing) ? ' UNSUPPORTED-wall-or-thing' : '';
    const line = `${k + 1} ${id} ${w} ${T0.kind} ${T0.at.join(' ')}${tail}${opts ? ' UNSUPPORTED-opts' : ''}`;
    if (line.includes('UNSUPPORTED') || /^g\d/.test(w)) { count(`orders.give(${id}) on the puppet (the face has no words for it)`); return give.call(this, id, t, who, opts); }
    count(`orders.give(${id})`); writeBack(g, k + 1);
    hand(enc.encode(line), (p, n) => X.face_give(h, k + 1, p, n)); const said = outText(), m = said.match(/-> (None|Some\("((?:[^"\\]|\\.)*)"\))\s*$/);
    if (OUT) appendFileSync(`${OUT}/gives.txt`, `${line}\n   ${said.trim()}\n`);
    if (!m) { say(`play: the face's answer to an order was not understood: ${said}`); }
    applyRust(g); lastState = BASE === 'rust' ? lastRust : exportGame(g, reg);
    return m && m[1] !== 'None' ? JSON.parse(`"${m[2]}"`) : null; };
  if (g.fight?.roomAt) { const F = g.fight, ask = F.roomAt; F.roomAt = function (this: any, p: any) { const n0 = F.rooms.length; const r = ask.call(this, p); if (F.rooms.length !== n0) count('fight.roomAt (flooded a new room)'); return r; }; }
  // (a lesson's player picks a row of the nearby list between ticks and reads the answer at once: made on the Rust side, as an order is)
  const wordAt = (ty: string, w: unknown) => w === undefined ? -1 : SCHEMA.types[ty].words.indexOf(w);
  g.selectVerb = function (this: any, kind: string, verb?: string) { count('game.selectVerb'); writeBack(g, k + 1);
    const r = hand(enc.encode(`${k + 1} select_verb ${wordAt('crate::game::InteractableKind', kind)} ${wordAt('crate::game::Verb', verb)} ${kind} ${verb}`), (p, n) => X.face_hostwrite(h, k + 1, p, n)), said = outText();
    if (r !== 1) say(`play: the face did not take selectVerb(${kind}, ${verb})`);
    applyRust(g); lastState = BASE === 'rust' ? lastRust : exportGame(g, reg); return / -> true\s*$/.test(said); };
  // THE HOST'S STEPS ROUND A TICK THAT THE RUST SIDE NOW MAKES: the lights' update and the box world's listing run on the Rust game before the apply, so the
  // puppet already holds what they leave. Run again in TypeScript they would be made twice (a flash's envelope stepped twice).
  const E = g.engine, L = E.lights, W = E.world;
  L.update = function () { count('engine.lights.update (not run: the Rust side made it)'); };
  W.upload = function () { if (uploadDone) { uploadDone = false; return; }
    // (not on the ticks expected: made now on the Rust side, and the puppet takes the state again so the player reads the listed world)
    uploadsLate++; X.face_upload(h); uploadsRust++; applyRust(g); lastState = exportGame(g, reg); }; }
let uploadDone = false;

/** THE WRITE-BACK: what TypeScript wrote into the puppet since the last apply goes to the Rust game. `at`: the tick that opens next. */
function writeBack(g: any, at: number): void {
  const t = now();
  {
    const mine = exportGame(g, reg); let t1 = now(); T.wbExport += t1 - t;
    let owed: { q: string; path: string }[] = [];
    if (mine.length !== lastState!.length || !buf(mine).equals(buf(lastState!))) { const D = fieldsDiffering(lastState!, mine); for (const x of D) { if (HOST_Q(x)) tallyAdd(HOSTW, x.q, x.path, at); else if (OWN_Q(x)) tallyAdd(OWNW, x.q, x.path, at); else owed.push(x); } }
    const blk = tsBlock(g), blockOwed = !buf(blk).equals(buf(lastBlock!));
    T.wbCompare += now() - t1; t1 = now();
    if (owed.length) {
      for (const x of owed) tallyAdd(WB, x.q, x.path, at);
      // (the one write the scripted player makes every tick, his own light meter, has a small call of its own in the face)
      if (owed.length === 1 && owed[0].path === 'g.pl[0].visibility' && g.seats.length === 1) { X.face_meter(h, g.player.visibility); wbMeter++; }
      // (a level script's beat holds its place in a generator no state carries: the Rust import refuses a state with a beat running, by a panic. What
      // TypeScript wrote while one runs cannot be handed over whole; it is named and counted, and the next apply puts the Rust game's own back)
      else { hand(withCutsOf(mine, lastRust!), (p, n) => X.face_import_into(h, p, n)); wbFired++; if (OUT && wbFired <= 400) appendFileSync(`${OUT}/writeback.txt`, `${at} ${[...new Set(owed.map(x => x.q))].join(' ')}\n   eg ${owed[0].path}\n`); }
    } else wbHostOnly++;
    if (HANDBACK && (process.env.HB_AT ? process.env.HB_AT.split(',').includes(String(at)) : at % HANDBACK === 0)) { const dbg = () => { X.face_debug(h); return outText(); }; const before = process.env.HB_DEBUG ? dbg() : ''; hand(mine, (p, n) => X.face_import_into(h, p, n)); if (process.env.HB_DEBUG) { const after = dbg(); writeFileSync(`${process.env.HB_DEBUG}/before${at}.txt`, before); writeFileSync(`${process.env.HB_DEBUG}/after${at}.txt`, after); } handbacks++; if ((bookOf(g).listOf('DirectorLiveBeatId') as any[]).some(r => r.state === 'running')) handbacksInBeat++; }   // (a whole hand-over whether anything was written or not: what a host must be able to do at any tick, a beat running or not)
    if (blockOwed) { wbBlock++; const lay: any[] = LAYOUT ??= layoutOf(); const a = lastBlock!; for (const p of lay) { if (!buf(blk.subarray(p.off, p.off + p.len)).equals(buf(a.subarray(p.off, p.off + p.len)))) BLOCKW.set(p.name, (BLOCKW.get(p.name) ?? 0) + 1); } rustBlock().set(blk.subarray(0, X.state_len())); lastBlock = blk.slice(); }
    T.wbImport += now() - t1; openState = mine; lastState = mine;
  }
}

// ---- the step, in place of `stepSeats`
function puppetStep(g: any, dt: number, cmds: readonly any[]): void {
  let t = now();
  if (k === 1) {
    g.sim.settle(g);
    const start = exportGame(g, reg), block = tsBlock(g).slice();
    if (OUT) { writeFileSync(`${OUT}/start.bin`, start); }
    h = hand(start, (p, n) => hand(block, (bp, bn) => X.face_new(p, n, bp, bn))); if (!h) throw new Error('face_new: no free slot');
    slot = X.face_slot(h); if (outText() && !QUIET) say(outText());
    if (LESSON && has('--lesson-json')) { const les = enc.encode(JSON.stringify((g.level.scenarios?.lesson?.steps ?? []).map((s: any) => s.done))); hand(les, (p, n) => X.face_lesson(h, p, n)); if (outText() && !QUIET) say(outText()); }
    if (X.state_len() !== block.length) say(`play: rust/sim's block is ${X.state_len()} bytes in the module and ${block.length} in the TypeScript game`);
    lastState = start; lastBlock = block; lastRust = start;
    if (!NO_ASKS) asks = installAsks(g, reg, (kind, row, seat, b) => { hand(b, (p, n) => X.face_ask(h, kind, row, seat, p, n)); return u8().slice(X.face_out_ptr(), X.face_out_ptr() + X.face_out_len()); }, () => { applyRust(g); });
    say(`play: ${scn}:${seed}; the start ${start.length} bytes, the block ${block.length}; slot ${slot}; memory ${(mem.buffer.byteLength / 1048576).toFixed(0)} MB`);
  } else writeBack(g, k);
  // THE HOST'S HALF: each seat's record, every camera and Input row, in the bridge's own small buffer (export.ts hostHalf, io::take_host_half)
  t = now(); const hb = hostHalfHeld(g, cmds); T.hostHold += now() - t; t = now();
  hostBytes = hb.length; hand(hb, (p, n) => X.face_host_half(h, p, n));
  T.hostHalf += now() - t;
  t = now(); X.face_step(h, dt, 0); const el = now() - t; T.step += el; if (el > worstStep) { worstStep = el; worstStepAt = k; }
  { const said = outText(); if (said && !QUIET) say(said.trimEnd()); }
  // ---- held to the plain TypeScript play, where its hashes were taken: as the tick returns, before the host's steps
  const p = post[k - 1];
  if (p) { const got = partHashes(slot), die = X.face_die_n(h); S.compared++;
    let off = NAMES.filter((_, i) => p[4 + i] !== String(got[i])); const poolOff = off.includes('poses'); off = off.filter(n => n !== 'poses');
    if (!off.length && poolOff && !S.poolParted) { S.poolParted = k; say(`TICK ${k}: the part \`poses\` parts and every other part is the same (a dead man's pose is in the pool on the Rust side: README.md)`); }
    if (p[1] !== String(die) && !S.dieParted) { S.dieParted = k; say(`TICK ${k}: THE DIE'S COUNT PARTS: ${die} here, ${p[1]} in the TypeScript play`); }
    if (off.length && !S.parted) { S.parted = k; S.partedIn = off; say(`TICK ${k}: THE BLOCK PARTS in [${off}]; time ${X.face_time(h).toFixed(4)}; die n=${die} (TypeScript ${p[1]})`); }
  } else if (post.length && !S.parted && !tLeft) { tLeft = k; say(`TICK ${k}: the TypeScript play ended at tick ${post.length}: this play goes on past it`); }
  // ---- the host's steps after a tick, on the Rust side (wasm-run.ts makes them in this order)
  t = now(); X.face_lights(h, dt); X.face_view(h);
  if (k % UP_EVERY === 1 % UP_EVERY) { X.face_upload(h); uploadsRust++; if (uploadDone) uploadsMissed++; uploadDone = true; } else if (uploadDone) { uploadsMissed++; uploadDone = false; }
  if (X.face_out_n) { const n = X.face_out_n(h); if (n) { outEvents += n; outTicks++; X.face_out_clear(h); } }
  T.after += now() - t;
  const rust0 = applyRust(g);
  // ---- the dress step: what the tick does that has no Rust body and a reader of the puppet needs
  // (THE AUTOSAVE is the host's and decides off the frame as it now stands, so it runs here, on the puppet. Its one write into the game is the
  // chronicle's row, and on a level on rails it stops the level's program for the take: both are made on the Rust side by `face_checkpoint`, as the
  // replay loop makes them, and the puppet takes the state again.)
  t = now(); let rust = rust0;
  if (DRESS) { g.par?.tick(); g.verdict?.tick();
    const s0 = g.chronicle.counts.saves; try { g.saves?.tick(g, dt); } catch (e) { count(`saves.tick THREW on the puppet: ${String((e as any)?.message ?? e).slice(0, 120)}`); }
    if (g.chronicle.counts.saves !== s0) { const E = g.chronicle.events, e = E[E.length - 1], label = e?.kind === 'checkpoint' ? String(e.label ?? e.data?.label ?? '') : ''; count('saves.tick took a checkpoint (handed by face_checkpoint)');
      hand(enc.encode(label), (p, n) => X.face_checkpoint(h, p, n)); rust = applyRust(g); } }
  T.dress += now() - t;
  // ---- the puppet as the apply left it: what the next write-back is held to. `--baseline rust` takes the Rust bytes themselves and exports nothing here
  t = now();
  if (BASE === 'rust' && !(APPLY_CHECK > 0 && (k <= 3 || k % APPLY_CHECK === 0))) lastState = rust;
  else { const mine = exportGame(g, reg); lastState = BASE === 'rust' ? rust : mine;
    if (APPLY_CHECK > 0 && (k <= 3 || k % APPLY_CHECK === 0)) { applyChecks++; if (mine.length !== rust.length || !buf(mine).equals(buf(rust))) { const cut: string[] = [], D = fieldsDiffering(rust, mine, cut).filter(x => !SCHEMA.serial?.[x.q] && x.q !== 'tail'); for (const q of cut) CUTW.set(q, (CUTW.get(q) ?? 0) + 1);
      if (D.length) { applyUnequal++; for (const x of D) tallyAdd(APPLY_MISS, x.q, x.path, k); if (OUT && applyUnequal === 1) { writeFileSync(`${OUT}/apply-rust${k}.bin`, rust); writeFileSync(`${OUT}/apply-puppet${k}.bin`, mine); } } } } }
  T.check += now() - t;
  if (S.parted === k && OUT) { if (openState) writeFileSync(`${OUT}/open${k}.bin`, openState); writeFileSync(`${OUT}/rust${k}.bin`, rust); writeFileSync(`${OUT}/block${k}.bin`, rustBlock()); }
  if (!QUIET && (k % 250 === 0)) say(`tick ${k}; time ${X.face_time(h).toFixed(3)}; die n=${X.face_die_n(h)}; ${S.parted ? `PARTED at ${S.parted}` : 'equal so far'}; state ${rust.length} bytes; write-backs ${wbFired}; ms a tick: step ${(T.step / k).toFixed(2)} export ${(T.export / k).toFixed(1)} apply ${(T.apply / k).toFixed(1)} write-back ${((T.wbExport + T.wbCompare + T.wbImport) / k).toFixed(1)} check ${(T.check / k).toFixed(1)}`);
}
let openState: Uint8Array | null = null, LAYOUT: any[] | null = null;
const layoutOf = () => NAMES.map((name, i) => ({ name, off: X.face_part_addr(0, i) - X.state_addr(0), len: X.face_part_len(i) }));

// ---- `Game.update` on the puppet: its own lines round the step run; the step is `puppetStep`
const was = Game.prototype.update; let tOut = 0, wall0 = 0;
Game.prototype.update = function (this: any, ...a: any[]) {
  const g = this; k++; const t0 = now(); if (k > 1) T.driver += t0 - tOut; else wall0 = t0;
  if (k === 1) { dressAll(false); if (!g.dice) g.dice = newDice(DIE); taps(g); }
  let inStep = 0, err: unknown = null;
  g.stepSeats = function (dt: number, cmds: readonly any[]) { const s = now(); try { puppetStep(g, dt, cmds); } finally { inStep = now() - s; } };
  try { (was as any).apply(this, a); } catch (e) { err = e; } finally { delete g.stepSeats; }
  T.hostTs += now() - t0 - inStep; tOut = now();
  if (err) { const words = panicText(); if (words) { say(`PANIC in tick ${k}: ${words}`); finish(g, 2); } say(`TICK ${k} THREW on the puppet: ${(err as any)?.stack ?? err}`); finish(g, 3); }
  if ((S.parted && !GO_ON) || k >= TICKS) { stopped = true; throw new Stop(); }
} as any;

function table(T0: Tally, n = 30): string { return [...T0].sort((a, b) => b[1].ticks - a[1].ticks).slice(0, n).map(([q, e]) => `    ${q}: ${e.n} places over ${e.ticks} ticks, e.g. ${e.eg.slice(0, 160)}`).join('\n') || '    (none)'; }
let res: any = null, les: any = null, ended = false, stopped = false;
function finish(g: any, code: number | null = null): never {
  if (ended) process.exit(code ?? 3); ended = true; console.log = LOG; console.error = ERR; console.warn = WARN;
  // (where the play stopped short of its end, the last state of both sides is kept: what the player read against what the Rust game held)
  if (OUT && lastRust && g && (les?.code || res?.end === 'error')) { try { writeFileSync(`${OUT}/last-rust.bin`, lastRust); writeFileSync(`${OUT}/last-puppet.bin`, exportGame(g, reg)); } catch { /* the puppet may not export where it threw */ } }
  const wall = (now() - wall0) / 1000, per = (x: number) => (x / Math.max(1, k)).toFixed(3);
  const mineEnd = res ? `${res.end} (${String(res.why).split('\n')[0]}); t ${res.t}; team ${res.team} of ${res.of}; hostiles ${res.hostiles} of ${res.hostilesOf}` : les ? `${les.code === 0 ? 'every step went by and the run ended' : `STOPPED (${les.why})`}; ${les.steps.length} steps gone by: ${les.steps.map((s: any) => s.id).join(' ')}` : '(the play was stopped before its end)';
  say(''); say(`THE RUST GAME PLAYED BY THE SCRIPTED PLAYER THROUGH THE PUPPET: ${scn}:${seed}, ${k} ticks`);
  say(`  held to the plain TypeScript play after every tick: ${S.compared} ticks x (the die's count + ${NAMES.length - 1} parts' hashes; \`poses\` apart)${post.length ? '' : '  NOTHING COMPARED: no reference'}`);
  say(`  the host's half (the record, the cameras, the Input rows) held to the plain play's: ${hostCompared ? (hostParted ? `PARTED AT TICK ${hostParted}` : `equal on every one of ${hostCompared} ticks`) : 'not compared (the reference kept no h<k>.bin)'}`);
  say(`  the block: ${S.parted ? `PARTED AT TICK ${S.parted} in [${S.partedIn}]` : `equal after every one of ${S.compared} ticks`}; the die's count: ${S.dieParted ? `PARTED AT TICK ${S.dieParted}` : 'equal after every tick'}; \`poses\` ${S.poolParted ? `parts from tick ${S.poolParted}` : 'equal'}`);
  say(`  the end here:           ${mineEnd} at tick ${k}`);
  say(`  the end in TypeScript:  ${refEnd.replace(/^TypeScript: /, '')}`);
  say(`  ms a tick (mean of ${k}): the scripted player and its world's own lines ${per(T.driver)} · Game.update's own lines ${per(T.hostTs)} · write-back: the puppet's export ${per(T.wbExport)}, the compare ${per(T.wbCompare)}, the hand-over ${per(T.wbImport)} · the host's half ${per(T.hostHalf)} (${hostBytes} bytes; held to the reference's first: ${per(T.hostHold)}) · face_step ${per(T.step)} (worst ${worstStep.toFixed(1)} at tick ${worstStepAt}) · lights, view, listing ${per(T.after)} · face_export ${per(T.export)} · the apply ${per(T.apply)} (worst ${worstApply.toFixed(0)}) · the block's copy ${per(T.block)} · the dress step ${per(T.dress)} · the puppet exported again and held to the Rust bytes ${per(T.check)}`);
  say(`  wall: ${wall.toFixed(1)} s for ${k} ticks (${(wall * 1000 / Math.max(1, k)).toFixed(1)} ms a tick); the module's memory ${(mem.buffer.byteLength / 1048576).toFixed(0)} MB; the last state ${lastRust?.length ?? 0} bytes`);
  say(`  THE WRITE-BACK (held to ${BASE === 'rust' ? 'the Rust bytes last applied' : 'the puppet exported as the apply left it'}): of ${Math.max(0, k - 1)} compares, ${wbHostOnly} found nothing but the host's half, ${wbMeter} only the player's light meter (handed by face_meter), ${wbFired} something else (the whole puppet handed by face_import_into); the block differed ${wbBlock} times (${[...BLOCKW].map(([n, c]) => `${n} ${c}`).join(', ') || '-'})`);
  say(`  what TypeScript wrote into the game between ticks, by field (not the host's half):`); say(table(WB));
  if (wbRefused) { say(`  NOT HANDED OVER, ${wbRefused} times: a beat of the level's script was running, and the Rust import refuses such a state. By field:`); say(table(WB_REFUSED)); }
  say(`  the host's half as the compare saw it:`); say(table(HOSTW, 8));
  say(`  calls of the scripted player and its world that write the game and are no input: ${[...CALLS].map(([n, c]) => `${n} x${c}`).join(' · ') || '(none)'}`);
  say(`  seen by the compare and not a write (identity serials, a clip's place in the Rust game's own table):`); say(table(OWNW, 8));
  say(`  THE APPLY, held to the Rust bytes it was given (the puppet exported again right after it): ${applyChecks} ticks checked (the first 3, then every ${APPLY_CHECK}th), ${applyUnequal} with a field outside the cut ones not as Rust has it`); if (applyUnequal) say(table(APPLY_MISS));
  say(`    of the fields cut out of every compare, the puppet's differed from the Rust game's on: ${[...CUTW].map(([q, n]) => `${q} ${n} of ${applyChecks}`).join(' · ') || 'none'}`);
  say(`  the apply made new rows: ${JSON.stringify(made)}; no way back: ${[...noWay].map(([q, n]) => `${q} x${n}`).join(', ') || '-'}; no setter: ${[...noSetter].map(([q, n]) => `${q} x${n}`).join(', ') || '-'}; no window: ${[...noWindow].join(', ') || '-'}`);
  if (HANDBACK) say(`  HANDED BACK WHOLE every ${HANDBACK} ticks: ${handbacks} times, ${handbacksInBeat} with a beat of the level's script running`);
  say(`  the box world listed on the Rust side ${uploadsRust} times (${uploadsLate} asked for on a tick not expected, ${uploadsMissed} expected and not asked for); out events: ${X.face_out_n ? `${outEvents} over ${outTicks} ticks, cleared and not dispatched (the whole export does not carry them)` : 'this module cannot say'}`);
  const sameEnd = !post.length ? false : (res ? refEnd.includes(`: ${res.end} (`) && refEnd.includes(`t ${res.t};`) && refEnd.includes(`team ${res.team} of ${res.of}`) && refEnd.includes(`PLAYED OUT at tick ${k} `) : les ? refEnd.includes(` at tick ${k}:`) && refEnd.includes(les.steps.map((s: any) => s.id).join(' ')) : false);
  if (code === null) code = S.parted || S.dieParted ? 1 : !post.length ? 3 : sameEnd ? 0 : (res || les) ? 4 : 0;
  say(`  ${code === 0 ? (res || les ? 'EQUAL TO THE END, and it ended the same way' : `equal for the ${k} ticks played (stopped by --ticks)`) : code === 1 ? 'PARTED' : code === 4 ? 'equal every tick, BUT THE END IS NOT WORDED THE SAME' : `exit ${code}`}`);
  process.exit(code); }
const LOG = console.log, ERR = console.error, WARN = console.warn; let G0: any = null;
{ const u = Game.prototype.update; Game.prototype.update = function (this: any, ...a: any[]) { G0 = this; return (u as any).apply(this, a); } as any; }
try { if (LESSON) les = await (await import('../qa/lesson-run.ts')).runLesson(scn, { seed, sky: false, glow: false, floor: 0 }); else res = await play(scn, seed, 300, false, {}); }
catch (e) { if (!(e instanceof Stop)) { console.log = LOG; say(`play: ${(e as any)?.stack ?? e}`); finish(G0, 3); } }
// (playthrough.ts turns whatever is thrown under it into a run that ended 'error': this file's own stop is not the player's)
if (stopped) { res = null; les = null; } else if (les && les.code !== 0) say(`play: THE LESSON'S PLAYER STOPPED ON THE PUPPET at tick ${k}: ${String(les.why).slice(0, 600)}`); else if (res?.end === 'error') say(`play: THE SCRIPTED PLAYER THREW ON THE PUPPET at tick ${k}: ${res.why}`);
finish(G0);
