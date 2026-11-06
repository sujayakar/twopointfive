// THE BRIDGE RUN BACKWARDS: a Rust-exported state applied onto a live TypeScript Game in place, so that whatever outside the simulation holds a guard, a door or a box goes on holding it and reads the new
// values (blitz/SWAP.md). It reads the same schema export.ts writes by, so a field added to the game needs nothing here. A row keeps its object while its table row lives (the book the export keeps is the
// map both ways); a row new to the state is made with its class's prototype and its carried fields, and `hooks.made` is told; what the bridge cannot carry on a new row (a GPU handle, a sound voice) is that
// hook's. A row no list names any more is dropped by the lists themselves, being values: `hooks.gone` is for a class that owns something beyond the row. Nothing here runs a rule.
//
// A FIELD'S WAY BACK: a plain field is assigned. The schema's expressions go by their form: `o`, a path, `o.x ?? d`, `B.top`/`B.side` (a module's own variable: rollbackVar's setter, a container in place),
// `B.serialOf(o)` (the object takes that serial), `B.knob` (constant). What is left is one line each in INVERSE below, and a field with neither is read past and counted in `report.noWay`.
//
//   applyGame(g, blob, knobs, { only?: Set of 'mod::Type.field', hooks? }) → report
//   `only`: fields not named are read past; a struct is entered only if it holds a named one. The bytes are still walked: what bounds the cost of a tick is what Rust chooses to export.
import { SCHEMA, parse, bookOf, seedBook, helpersOf, compiledExpr, LOOSE } from './export';
import { rawKey, PUT_BACKS } from '../../src/math/side';

export type Hooks = { made?: Record<string, (row: any, g: any) => void>; gone?: Record<string, (row: any, g: any) => void>; classes?: Record<string, { prototype: object }>; values?: Record<string, any> };
export type Puppet = { lists: { step: Set<string>; start: Set<string> }; start: boolean; rows?: any; ctx?: any; slowTable?: { type: string; field: string }[]; vals?: string[] };
export type ApplyOpts = { only?: Set<string>; hooks?: Hooks; puppet?: Puppet };
export type Report = { out?: { arm: string; e: any }[]; phases: Record<string, number>; counts: Record<string, number>; made: Record<string, number>; noWay: Map<string, number>; noSetter: Map<string, number>; noWindow: Set<string>; deadSerials: number; bytes: number; ms: number };

const DEC = new TextDecoder();
type N = any;
const isTyped = (v: any) => ArrayBuffer.isView(v) && !(v instanceof DataView);
const isSide = (v: any) => v && typeof v.liveKeys === 'function';
const FRESH = Symbol('fresh');
const differs = (a: unknown, b: unknown) => !Object.is(a, b) && !(a == null && b == null);

// ---- reading past a value
type Keep = { field: (q: string, ty: string) => boolean; desc: (ty: string | N) => boolean };
function skipper(b: Uint8Array, I: { p: number; K?: Keep | null }) { const dv = new DataView(b.buffer, b.byteOffset, b.byteLength); I.p = 0;
  const sk = (n: N): void => { switch (n.k) {
    case 'f64': case 'u64': case 'i64': I.p += 8; return; case 'f32': case 'u32': case 'i32': case 'usize': case 'id': I.p += 4; return; case 'bool': case 'u8': case 'i8': I.p += 1; return; case 'u16': case 'i16': I.p += 2; return; case 'unit': return;
    case 'str': I.p += 4 + dv.getUint32(I.p, true); return;
    case 'opt': if (b[I.p++]) sk(n.of); return;
    case 'vec': { const L = dv.getUint32(I.p, true); I.p += 4; const w = I.K && I.K.desc(n.of) ? -1 : fixedWidth(n.of); if (w >= 0) { I.p += L * w; return; } for (let k = 0; k < L; k++) sk(n.of); return; }
    case 'arr': { const w = I.K && I.K.desc(n.of) ? -1 : fixedWidth(n.of); if (w >= 0) { I.p += n.n * w; return; } for (let k = 0; k < n.n; k++) sk(n.of); return; }
    case 'tup': for (const x of n.of) sk(x); return;
    case 'map': { const L = dv.getUint32(I.p, true); I.p += 4; for (let k = 0; k < L; k++) { sk(n.K); sk(n.V); } return; }
    case 'set': { const L = dv.getUint32(I.p, true); I.p += 4; for (let k = 0; k < L; k++) sk(n.K); return; }
    case 'type': { const T = SCHEMA.types[n.path]; if (T.k === 'struct') { const K = I.K; if (K && !K.desc(n)) { I.K = null; sk(n); I.K = K; return; } if (!K) { for (const f of T.fields) sk(parse(f.ty)); return; } for (const P of planOf(T)) { if (!K.field(P.q, P.f.ty)) continue; if (!K.desc(P.N)) { I.K = null; sk(P.N); I.K = K; continue; } sk(P.N); } return; } if (T.k === 'enum') { I.p += 2; return; } const a = dv.getUint16(I.p, true); I.p += 2; if (T.k === 'wordEnum') { for (const f of T.arms[a].fields) sk(parse(f.ty)); return; } const t = armType(T, a); if (t) sk(parse(t)); return; } }
    throw new Error(`import: no way past ${n.k}`); };
  return { I, sk, dv }; }
type Plan = { f: any; q: string; N: N };
const planOf = (T: any): Plan[] => T.plan ??= T.fields.map((f: any) => ({ f, q: `${T.mod}::${T.name}.${f.rust}`, N: parse(f.ty) }));
const W = new Map<N, number>();
function fixedWidth(n: N): number { let w = W.get(n); if (w !== undefined) return w; switch (n.k) { case 'f64': case 'u64': case 'i64': w = 8; break; case 'f32': case 'u32': case 'i32': case 'usize': case 'id': w = 4; break; case 'bool': case 'u8': case 'i8': w = 1; break; case 'u16': case 'i16': w = 2; break; case 'unit': w = 0; break; case 'arr': { const e = fixedWidth(n.of); w = e < 0 ? -1 : e * n.n; break; } case 'tup': { w = 0; for (const x of n.of) { const e = fixedWidth(x); if (e < 0) { w = -1; break; } w += e; } break; } case 'type': { const T = SCHEMA.types[n.path]; if (T.k === 'enum') w = 2; else if (T.k === 'struct') { w = 0; W.set(n, -1); for (const f of T.fields) { const e = fixedWidth(parse(f.ty)); if (e < 0) { w = -1; break; } w += e; } } else w = -1; break; } default: w = -1; } W.set(n, w!); return w!; }
const armType = (T: any, a: number): string | null => T.k === 'data' ? T.vars[a].ty ?? null : (T.arms[a].match(/\((.*)\)/)?.[1] ?? null);

// ---- a field's way back, decided once
type Way = { kind: 'assign' | 'self' | 'path' | 'default' | 'top' | 'knob' | 'serial' | 'inverse' | 'skip' | 'none'; path?: string[]; dflt?: unknown; file?: string; name?: string; side?: string; fn?: Inverse };
type Ctx = { g: any; B: any; BK: any; fix: (() => void)[]; R: Report; read: (n: N, cur: any) => any; sk: (n: N) => void; I: { p: number }; dv: DataView };
type Inverse = (o: any, v: any, c: Ctx) => void;
const skipField: Inverse = () => {};
/** the fields whose expression is no assignment. Each says how the value gets back, or why it need not: read `skipField` as "the export works this out from what is carried, or it is set up once and never changes" */
const INVERSE: Record<string, Inverse> = {
  'game::Game.view_proj': (o, v) => { o.viewProj = v; }, 'seats::SeatSet.view_proj': (o, v) => { o.viewProj = v; },
  'game::Game.door_owns': skipField,   // (worked out from which leaf holds a hand: the leaves are rows)
  'character::Character.who': skipField,   // (whoever holds this man as his `char`: rows)
  'character::Character.man_row': (o, v, c) => { if (v !== 0xffffffff) { o.S = c.g.men; o.r = v; } },
  'character::Character.man_kept': (o, v) => { if (v) { o.owner = v[0]; o.rank = v[1]; o.reason = v[2]; o.since = v[3]; } },
  'weapons::Firearm.def': (o, v, c) => { if (o.def?.id !== v) o.def = (c.R as any).values?.WEAPONS?.[v] ?? o.def; },
  'weapons::Firearm.kind': (o, v, c) => { const P = CLASSES.get(({ pistol: 'weapons::Pistol', rifle: 'weapons::Rifle', plain: 'weapons::Firearm' } as any)[v]); if (P && Object.getPrototypeOf(o) !== P) Object.setPrototypeOf(o, P); },
  'scene_collision::StaticCollision.pane_rule': skipField, 'scene_collision::StaticCollision.pane_rows': skipField, 'scene_collision::StaticCollision.glass_walkable': skipField, 'scene_collision::StaticCollision.fire_pass': skipField,   // (set up with the level)
  'destruction::Destruction.blast_listeners': skipField, 'wallbreak::WallBreak.listeners': skipField, 'destruction::RateCap.n': skipField, 'anim_skeleton::ClipSide.at': skipField, 'anim_skeleton::SkeletonInstance.rig': skipField, 'scene_level_load::SceneLevelLoadMod.level_names': skipField,
  'chronicle::Chronicle.heard_of': (o, v, c) => { c.fix.push(() => { const T = o.heardOf; const put = (k: object, r: unknown) => T.set(k, r); if (T.clear) T.clear(); else for (const k of T.liveKeys()) T.delete(k); for (const [s, i] of v as Map<number, number>) put(objOfSerial(c, s), o.events[i]); }); },
  'fogofwar::FogOfWar.fired_n': (o, v, c) => { const T = o.firedN; const L = c.BK.listOf('RoomfightOpId'); const want = new Map<object, number>(); for (const [i, n] of v as Map<number, number>) want.set(L[i], n); for (const k of T.liveKeys()) if (!want.has(k)) T.delete(k); for (const [k, n] of want) T.set(k, n); },
  'anim_skeleton::Pose.at': (o, v, c) => { if (v >= LOOSE) { looseAt(c, o, v - LOOSE); return; } o.at = v; if (!o.home) { o.home = c.g.sim; o.own = null; } },
  'anim_skeleton::Pose.nodes': (o, v) => { if (o.numNodes !== v) { const C = Object.getPrototypeOf(o).constructor; const P = new C(v); o.numNodes = v; o.rows = P.rows; if (!o.home) o.own = P.own; } },
  'character::RagDrive.hips': (o, v, c) => { c.fix.push(() => { o.hips = o.ov[v]; }); },
  'character::RagDrive.limb_ov': (o, v, c) => { c.fix.push(() => { o.limbOv = (v as number[]).map(i => o.ov[i]); }); },
  'doorpath::DoorPathLastOfValue.w': (o, v, c) => { if (!v) { o.W = null; return; } const w = o.W ?? (o.W = {}); w.gd = v[0]; serialIs(c, w, v[1]); },
};
const CLASSES = new Map<string, object>();
/** a row whose numbers are the block's columns reads them through these: what its constructor would have set, the row itself coming with the fields */
const BLOCK_ROW: Record<string, (o: any, g: any) => void> = { Item: (o, g) => { o.S = g.items.store; o.things = g.props.all; o.kept = null; } };
let looseOrder: object[] = [];
const looseAt = (_c: Ctx, o: object, i: number) => { looseOrder[i] = o; };
const bySerial = new WeakMap<object, Map<number, object>>();
function serialIs(c: Ctx, o: object, s: number) { const BK = c.BK; const old = BK.serial.get(o); if (old === s) return; BK.serial.set(o, s); serialMap(BK).set(s, o); if (s >= BK.nextSerial) BK.nextSerial = s + 1; }
function serialMap(BK: any): Map<number, object> { let M = bySerial.get(BK); if (!M) { bySerial.set(BK, M = new Map()); const O = BK.serialOfObjects?.(); if (O) for (const [k, v] of O) M.set(v, k); } return M; }
function objOfSerial(c: Ctx, s: number): object { const M = serialMap(c.BK); let o = M.get(s); if (!o) { o = { deadSerial: s }; c.R.deadSerials++; M.set(s, o); c.BK.serial.set(o, s); } return o; }
const wayCache = new Map<string, Way>();
function wayOf(q: string, f: any): Way { let w = wayCache.get(q); if (w) return w; const e: string | undefined = f.expr; let m: RegExpMatchArray | null;
  if (INVERSE[q]) w = { kind: 'inverse', fn: INVERSE[q] };
  else if (e === undefined) w = f.ts !== undefined ? { kind: 'assign', path: [f.ts] } : { kind: 'skip' };
  else if (e === 'o') w = { kind: 'self' };
  else if ((m = e.match(/^o((?:\.[A-Za-z_$][\w$]*|\["[^"]+"\])+)$/))) w = { kind: 'path', path: [...m[1].matchAll(/\.([A-Za-z_$][\w$]*)|\["([^"]+)"\]/g)].map(x => x[1] ?? x[2]) };
  else if ((m = e.match(/^o\.([A-Za-z_$][\w$]*) \?\? (.+)$/))) w = { kind: 'default', path: [m[1]], dflt: JSON.parse(m[2]) };
  else if ((m = e.match(/^B\.top\("([^"]+)", "([^"]+)", "([^"]*)"\)$/))) w = { kind: 'top', file: m[1], name: m[2], side: m[3] };
  else if (/^B\.knob\(/.test(e)) w = { kind: 'knob' };
  else if (e === 'B.serialOf(o)') w = { kind: 'serial' };
  else w = { kind: 'none' };
  wayCache.set(q, w); return w; }

// ---- which types hold a named field
const keepMemo = new WeakMap<Set<string>, Keep>();
function keepOf(only: Set<string>): Keep { let K = keepMemo.get(only); if (K) return K;
  const hold = new Map<string, boolean>(); for (const k of Object.keys(SCHEMA.types)) hold.set(k, false);
  const walk = (n: N): boolean => { switch (n.k) { case 'opt': case 'vec': case 'arr': return walk(n.of); case 'tup': return n.of.some(walk); case 'map': return walk(n.V); case 'type': return hold.get(n.path)!; default: return false; } };
  for (let grew = true; grew;) { grew = false; for (const [k, T] of Object.entries<any>(SCHEMA.types)) { if (hold.get(k)) continue; let r = false; if (T.k === 'struct') r = T.fields.some((f: any) => only.has(`${T.mod}::${T.name}.${f.rust}`) || walk(parse(f.ty))); else if (T.k === 'data') r = T.vars.some((a: any) => a.ty && walk(parse(a.ty))); if (r) { hold.set(k, true); grew = true; } } }
  const memoF = new Map<string, boolean>(), memoD = new Map<N, boolean>();
  const desc = (t: string | N): boolean => { const n = typeof t === 'string' ? parse(t) : t; let d = memoD.get(n); if (d === undefined) memoD.set(n, d = walk(n)); return d; };
  K = { field: (q, ty) => { let v = memoF.get(q); if (v === undefined) memoF.set(q, v = only.has(q) || desc(ty)); return v; }, desc }; keepMemo.set(only, K); return K; }

// ---- the walk
export function applyGame(g: any, blob: Uint8Array, knobs: Record<string, unknown>, opts: ApplyOpts = {}): Report {
  const t0 = performance.now(); const hooks = opts.hooks ?? {}; const BK = bookOf(g); if (!BK) throw new Error('import: this game has never been exported, so no row has an ID: export it once first');
  seedBook(g, knobs); for (const [k, v] of Object.entries(hooks.classes ?? {})) if (v?.prototype) CLASSES.set(k, v.prototype);
  const R: Report & { values?: any } = { phases: {}, counts: { read: 0, assign: 0, seen: 0 }, values: hooks.values, made: {}, noWay: new Map(), noSetter: new Map(), noWindow: new Set(), deadSerials: 0, bytes: blob.length, ms: 0 };
  const pup = opts.puppet; const I: { p: number; K?: Keep | null } = pup?.ctx ?? { p: 0 }; I.K = null; const { sk, dv } = skipper(blob, I); const B = helpersOf(g, knobs); let keep: Keep | null = pup ? keepOf(pup.start ? pup.lists.start : pup.lists.step) : opts.only ? keepOf(opts.only) : null; looseOrder = [];
  const madeRows = new Set<object>();
  const G = rawKey(g); const c: Ctx = { g, B, BK, fix: [], R, read: null as any, sk, I, dv };
  // ---- where the tables stand, and every row that is new
  const t1 = performance.now(); const heads: { t: any; n: number; at: number; N: N; whole: number; entries: number }[] = []; let serialNext = 0, tail = 0;
  if (pup) { const mainLen = dv.getUint32(0, true); I.p = 4 + mainLen; for (const t of SCHEMA.tables) { const n = dv.getUint32(I.p, true), whole = dv.getUint32(I.p + 4, true), entries = dv.getUint32(I.p + 8, true), len = dv.getUint32(I.p + 12, true); I.p += 16; heads.push({ t, n, at: I.p, N: parse(t.ty), whole, entries }); I.p += len; } }
  else { sk({ k: 'type', path: SCHEMA.root }); for (const t of SCHEMA.tables) { const n = dv.getUint32(I.p, true); I.p += 4; const N = parse(t.ty); const at = I.p; for (let k = 0; k < n; k++) sk(N); heads.push({ t, n, at, N, whole: n, entries: n }); } }
  serialNext = dv.getUint32(I.p, true); I.p += 4; tail = I.p;
  const t2 = performance.now(); R.phases.skim = t2 - t1; for (const { t, n } of heads) { const L = BK.listOf(t.id); if (L.length >= n) continue; const T0 = SCHEMA.types[t.ty.replace(/^alloc::boxed::Box<(.*)>$/, '$1')]; const proto = CLASSES.get(`${T0?.mod}::${T0?.name}`) ?? (L.length ? Object.getPrototypeOf(L[0]) : Object.prototype); const made: any[] = []; while (BK.listOf(t.id).length < n) { const o = Object.create(proto); BK.idOf(t.id, o); madeRows.add(o); made.push(o); } R.made[t.ts] = made.length; (c as any)[`made:${t.id}`] = made; }

  // ---- a new row that stands in the game's block, before its fields go in
  for (const { t } of heads) { const M = (c as any)[`made:${t.id}`]; const f = BLOCK_ROW[t.ts]; if (M && f) for (const o of M) f(o, g); }
  // ---- values
  const mapGet = (m: any, k: any) => m instanceof Map || isSide(m) ? m.get?.(k) : m?.[k];
  const orderSide = (T: any, keys: object[]) => { const K = T.sideKeys; if (!K) return; const now = T.liveKeys(); if (now.length === keys.length && now.every((x: object, i: number) => x === keys[i])) return; K.refs = keys.map(k => new WeakRef(k)); };
  const rKey = (K: N, cur: any): any => { if (K.k === 'u32' && cur === 'object-key') { const s = dv.getUint32(I.p, true); I.p += 4; return { serial: s }; } return read(K, undefined); };
  /** an object reached a second time in this apply stands in two places: the same bytes keep the one object, other bytes give this holder its own (the TypeScript shares by reference what Rust holds as values) */
  const seen = new Map<object, [number, number]>();
  function read(n: N, cur: any): any { R.counts.read++; if (process.env.PUP_PROF) { const T = ((globalThis as any).__byQ ??= new Map()); T.set(curQ, (T.get(curQ) ?? 0) + 1); }
    if (cur === FRESH) cur = n.k === 'vec' ? (dv.getUint32(I.p, true) === 0 ? [] : undefined) : n.k === 'map' ? new Map() : n.k === 'set' ? new Set() : n.k === 'bool' ? false : n.k === 'str' ? '' : n.k === 'arr' || n.k === 'tup' || n.k === 'type' || n.k === 'opt' || n.k === 'id' ? undefined : 0;
    else if (cur && typeof cur === 'object' && (n.k === 'arr' || n.k === 'tup' || n.k === 'vec' || (n.k === 'type' && SCHEMA.types[n.path].k === 'struct'))) { const at = seen.get(cur); const p0 = I.p; if (at) { sk(n); const L = I.p - p0; let eq = L === at[1] - at[0]; if (eq) for (let i = 0; i < L; i++) if (blob[p0 + i] !== blob[at[0] + i]) { eq = false; break; } if (eq) return cur; I.p = p0; return read(n, FRESH); } const v = readInner(n, cur); seen.set(cur, [p0, I.p]); return v; }
    return readInner(n, cur); }
  function readInner(n: N, cur: any): any {
    // a table keyed by the game, or by the one thing of its kind: what is applied is its row
    if (isSide(cur) && n.k !== 'opt' && n.k !== 'bool') {
      const P = cur.constructor.name.startsWith('GameSide') || (!(n.k === 'map' && n.K.k === 'id' && n.K.id === 'PlayerId') && cur.liveKeys().includes(G));
      if (P) { const v = read(n, cur.get ? cur.get(G) : cur.has(G)); if (cur.set) cur.set(G, v); return cur; }
      const K = cur.liveKeys(); if (K.length && SCHEMA.onceKeys.includes(K[0].constructor?.name)) { const v = read(n, cur.get ? cur.get(K[0]) : true); if (cur.set) cur.set(K[0], v); return cur; }
    }
    switch (n.k) {
      case 'f64': { const v = dv.getFloat64(I.p, true); I.p += 8; if (typeof cur === 'boolean') return v !== 0; if (cur === undefined && v === 0) return undefined; if (typeof cur === 'string') return String(v); return v; }
      case 'f32': { const v = dv.getFloat32(I.p, true); I.p += 4; if (cur === undefined && v === 0) return undefined; return v; }
      case 'bool': { const v = blob[I.p++] !== 0; if (isSide(cur)) { if (v) cur.add?.(G); else cur.delete(G); return cur; } if (cur === undefined && !v) return undefined; if (cur === null) { if (!v) return null; } else if (typeof cur === 'object') return v ? cur : null; return v; }
      case 'u8': return blob[I.p++]; case 'i8': { const v = dv.getInt8(I.p); I.p += 1; return v; } case 'u16': { const v = dv.getUint16(I.p, true); I.p += 2; return v; } case 'i16': { const v = dv.getInt16(I.p, true); I.p += 2; return v; }
      case 'i32': { const v = dv.getInt32(I.p, true); I.p += 4; return v; }
      case 'usize': { const v = dv.getUint32(I.p, true); I.p += 4; return cur === undefined && v === 0 ? undefined : v; }
      case 'u32': { const v = dv.getUint32(I.p, true); I.p += 4; if (cur && typeof cur === 'object') return objOfSerial(c, v); return cur === undefined && v === 0 ? undefined : v; }
      case 'u64': case 'i64': { const v = dv.getBigInt64(I.p, true); I.p += 8; return typeof cur === 'bigint' ? v : Number(v); }
      case 'str': { const L = dv.getUint32(I.p, true); I.p += 4; const s = DEC.decode(blob.subarray(I.p, I.p + L)); I.p += L; return cur === undefined && s === '' ? undefined : s; }
      case 'unit': return undefined;
      case 'opt': { const t = blob[I.p++]; if (isSide(cur)) { const K = cur.liveKeys(); if (!(n.of.k === 'map' && n.of.K.k === 'id') && n.of.k !== 'set') { const at = K.includes(G) ? G : K[0] ?? G; if (!t) { cur.delete(at); return cur; } const v = read(n.of, cur.get ? cur.get(at) ?? FRESH : true); if (cur.set) cur.set(at, v); else if (cur.add) cur.add(at); return cur; } if (!t) { for (const k of K) cur.delete(k); return cur; } if (!K.length && n.of.k === 'map' && dv.getUint32(I.p, true) === 0) { I.p += 4; cur.set(G, new Map()); return cur; } return read(n.of, cur); } if (!t) return n.of.k === 'opt' || cur === undefined ? undefined : null; return read(n.of, cur === null || cur === undefined ? FRESH : cur); }
      case 'vec': { const L = dv.getUint32(I.p, true); I.p += 4; if (isTyped(cur)) { const a = cur.length === L ? cur : new cur.constructor(L); for (let i = 0; i < L; i++) a[i] = readN(n.of); return a; } if (cur === null || cur === undefined) { if (!L) return cur; cur = n.of.k === 'f32' ? new Float32Array(L) : n.of.k === 'u8' ? new Uint8Array(L) : n.of.k === 'i32' ? new Int32Array(L) : n.of.k === 'u32' ? new Uint32Array(L) : []; if (isTyped(cur)) { for (let i = 0; i < L; i++) cur[i] = readN(n.of); return cur; } }
        const ser = serialField(n.of); const a = cur as any[]; const was = ser ? a.slice() : null; for (let i = 0; i < L; i++) { let x = a[i] === undefined ? FRESH : a[i]; if (ser) { const s = peekSerial(n.of, ser); const hit = serialMap(BK).get(s); x = hit ?? (x === FRESH || (was!.includes(x) && BK.serial.get(x) !== s) ? FRESH : x); } a[i] = read(n.of, x); } a.length = L; return a; }
      case 'arr': { if (pup && !pup.start && Array.isArray(cur) && fixedWidth(n.of) >= 0 && n.of.k !== 'type') { const a: any[] = new Array(n.n); let same = cur.length === n.n; for (let i = 0; i < n.n; i++) { a[i] = read(n.of, FRESH); if (same && !Object.is(a[i], cur[i])) same = false; } return same ? cur : a; }   // (a step does not visit every holder of a shared array: a changed one is a new one, as in the compiled rows)
        if (cur && typeof cur.length === 'number') { for (let i = 0; i < n.n; i++) { const v = read(n.of, cur[i] ?? FRESH); if (!Object.is(v, cur[i])) cur[i] = v; } return cur; } const a: any[] = []; for (let i = 0; i < n.n; i++) a[i] = read(n.of, FRESH); return a; }
      case 'tup': { const a: any[] = Array.isArray(cur) ? cur : []; n.of.forEach((x: N, i: number) => { a[i] = read(x, a[i] ?? FRESH); }); return a; }
      case 'map': return readMap(n, cur);
      case 'set': return readSet(n, cur);
      case 'id': return readId(n, cur);
      case 'type': return readType(n, cur);
    }
    throw new Error(`import: no reader for ${n.k}`);
  }
  const readN = (n: N): number => { switch (n.k) { case 'f32': { const v = dv.getFloat32(I.p, true); I.p += 4; return v; } case 'f64': { const v = dv.getFloat64(I.p, true); I.p += 8; return v; } default: return read(n, 0); } };
  function readId(n: N, cur: any): any { const v = dv.getUint32(I.p, true); I.p += 4; if (v === 0xffffffff) return cur && typeof cur === 'object' ? null : cur; if (typeof cur === 'number') return v; const e = SCHEMA.idExpr[n.id]; const viaExpr = e && cur && typeof cur === 'object' ? compiledExpr(e)(cur, g, B) : undefined; if (typeof viaExpr === 'number') { if (viaExpr === v) return cur; const k = `id ${n.id} at ${curQ}`; R.noWay.set(k, (R.noWay.get(k) ?? 0) + 1); return cur; } const L = BK.listOf(n.id); if (v >= L.length) { const k = `id ${n.id} past its table`; R.noWay.set(k, (R.noWay.get(k) ?? 0) + 1); return cur; } return L[v]; }
  function keyOf(K: N, curMap: any): any { if (K.k === 'u32') { const v = dv.getUint32(I.p, true); I.p += 4; const first = keysIn(curMap)[0]; if (first && typeof first === 'object') return objOfSerial(c, v); if (isSide(curMap)) return objOfSerial(c, v); return v; }
    if (K.k === 'id' && K.id === 'PlayerId') { const row = readId(K, undefined); for (const k of keysIn(curMap)) if (k && typeof k === 'object' && (k as any).player === row) return k; return row; }
    if (K.k === 'f64') { const v = dv.getFloat64(I.p, true); I.p += 8; if (curMap && !(curMap instanceof Map) && !isSide(curMap) && !(curMap instanceof Set)) return String(v); const first = keysIn(curMap)[0]; return typeof first === 'string' ? String(v) : v; }
    return read(K, undefined); }
  const keysIn = (m: any): unknown[] => m instanceof Map ? [...m.keys()] : m instanceof Set ? [...m] : isSide(m) ? m.liveKeys() : m && typeof m === 'object' ? Object.keys(m) : [];
  function readMap(n: N, cur: any): any { const L = dv.getUint32(I.p, true); I.p += 4; if (cur === null || cur === undefined) { if (!L) return cur; cur = new Map(); } const keys: any[] = [], vals: any[] = [];
    for (let i = 0; i < L; i++) { const k = keyOf(n.K, cur); keys.push(k); vals.push(read(n.V, mapGet(cur, k))); }
    if (cur instanceof Map) { const now = [...cur.keys()]; if (now.length === L && now.every((k, i) => k === keys[i])) { for (let i = 0; i < L; i++) if (cur.get(keys[i]) !== vals[i]) cur.set(keys[i], vals[i]); } else { cur.clear(); for (let i = 0; i < L; i++) cur.set(keys[i], vals[i]); } return cur; }
    if (isSide(cur)) { const want = new Set(keys); for (const k of cur.liveKeys()) if (!want.has(k)) cur.delete(k); for (let i = 0; i < L; i++) if (keys[i] && typeof keys[i] === 'object') cur.set(keys[i], vals[i]); orderSide(cur, keys); return cur; }
    if (typeof cur === 'object') { const want = new Set(keys.map(String)); for (const k of Object.keys(cur)) if (!want.has(k)) delete cur[k]; for (let i = 0; i < L; i++) cur[keys[i]] = vals[i]; return cur; }
    return cur; }
  function readSet(n: N, cur: any): any { const L = dv.getUint32(I.p, true); I.p += 4; if (cur === null || cur === undefined) { if (!L) return cur; cur = new Set(); } const keys: any[] = []; for (let i = 0; i < L; i++) keys.push(keyOf(n.K, cur));
    if (cur instanceof Set) { const now = [...cur]; if (!(now.length === L && now.every((k, i) => k === keys[i]))) { cur.clear(); for (const k of keys) cur.add(k); } return cur; }
    if (isSide(cur)) { const want = new Set(keys); for (const k of cur.liveKeys()) if (!want.has(k)) cur.delete(k); for (const k of keys) if (k && typeof k === 'object') cur.add(k); orderSide(cur, keys); return cur; }
    if (typeof cur === 'object') { for (const k of Object.keys(cur)) delete cur[k]; for (const k of keys) cur[k] = true; return cur; }
    return cur; }
  function newOf(T: any): any { const P = CLASSES.get(`${T.mod}::${T.name}`); const o = P ? Object.create(P) : {}; fresh.add(o); return o; }
  const fresh = new WeakSet<object>();
  function readType(n: N, cur: any): any { const T = SCHEMA.types[n.path];
    if (T.k === 'struct') { const o = cur && typeof cur === 'object' ? cur : newOf(T); const K = keep; if (K && !K.desc(n)) { keep = null; I.K = null; readStruct(T, o); keep = K; I.K = pup ? K : null; return o; } readStruct(T, o); return o; }
    if (T.k === 'enum') { const w = dv.getUint16(I.p, true); I.p += 2; return T.words[w]; }
    const a = dv.getUint16(I.p, true); I.p += 2;
    if (T.k === 'wordEnum') { for (const f of T.arms[a].fields) sk(parse(f.ty)); return cur; }   // (authored: a lesson's test stands in the TypeScript as it was written)
    if (T.k === 'handEnum') { const t = armType(T, a); if (t) { sk(parse(t)); return cur; } return T.words ? T.words[a] : cur; }
    const A = T.vars[a]; if (!A.ty) return A.str ?? (cur && typeof cur === 'object' && (cur.kind ?? cur.type) === A.kw ? cur : A.kw !== undefined ? { kind: A.kw } : cur);
    const AT = SCHEMA.types[A.ty]; const same = (() => { if (cur === null || cur === undefined || typeof cur !== 'object') return false; if (AT?.k === 'struct') { if (A.kw !== undefined) return (cur.kind ?? cur.type) === A.kw; if (A.sn && cur.constructor?.name === A.sn) return true; return AT.fields.filter((f: any) => f.ts).every((f: any) => f.ts in cur); } return true; })();
    if (A.sn && !AT && same && cur.constructor?.name !== A.sn) return read(parse(A.ty), FRESH);
    return read(parse(A.ty), same ? cur : FRESH); }
  let curQ = '';
  /** one field of a struct, its bytes next */
  function readField(T: any, P: Plan, o: any) { const { f, q, N } = P; curQ = q;
      const w = wayOf(q, f);
      switch (w.kind) {
        case 'assign': { const key = w.path![0]; const cur = o[key]; const v = read(N, cur === undefined && fresh.has(o) ? FRESH : cur); if (differs(v, cur)) { R.counts.assign++; try { o[key] = v; } catch { R.noSetter.set(q, (R.noSetter.get(q) ?? 0) + 1); } } break; }
        case 'self': readStruct(SCHEMA.types[N.path], o); break;
        case 'path': { let h = o; const P = w.path!; for (let i = 0; i < P.length - 1; i++) { h = h?.[P[i]]; } if (h === null || h === undefined) { sk(N); break; } const key = P[P.length - 1]; const cur = h[key]; const v = read(N, cur); if (differs(v, cur)) { try { h[key] = v; } catch { R.noSetter.set(q, (R.noSetter.get(q) ?? 0) + 1); } } break; }
        case 'default': { const key = w.path![0]; const cur = o[key]; const v = read(N, cur === undefined ? w.dflt : cur); if (cur !== undefined || JSON.stringify(v) !== JSON.stringify(w.dflt)) o[key] = v; break; }
        case 'top': { const T0 = (globalThis as any).__tpsTop?.[w.file!]; const V = B.varOf(w.side!); const cur = T0 && w.name! in T0 ? T0[w.name!] : V ? V.get() : B.rootOf(w.side!); if (cur === undefined && !V && !(T0 && w.name! in T0)) { R.noWindow.add(w.side!); sk(N); break; } const v = read(N, cur); if (differs(v, cur)) { if (V) V.set(v as never); else if (T0 && Object.getOwnPropertyDescriptor(T0, w.name!)?.set) T0[w.name!] = v; else { const k = `module ${w.side}: no setter`; R.noWay.set(k, (R.noWay.get(k) ?? 0) + 1); } } break; }
        case 'knob': sk(N); break;
        case 'skip': sk(N); break;
        case 'serial': { const s = dv.getUint32(I.p, true); I.p += 4; serialIs(c, o, s); break; }
        case 'inverse': { const v = read(N, N.k === 'map' ? new Map() : undefined); w.fn!(o, v, c); break; }
        default: R.noWay.set(q, (R.noWay.get(q) ?? 0) + 1); sk(N); }
  }
  function readStruct(T: any, o: any) { for (const P of planOf(T)) { const K = keep; if (process.env.PUP_DEBUG && I.p > blob.length) throw new Error('past the end at ' + curQ); if (K && (pup || !fresh.has(o))) { if (!K.field(P.q, P.f.ty)) { if (!pup) sk(P.N); continue; } if (!K.desc(P.N)) { keep = null; I.K = null; readField(T, P, o); keep = K; I.K = pup ? K : null; continue; } } readField(T, P, o); } }
  /** a struct that carries `B.serialOf(o)`: found by its serial wherever it now stands in a list */
  const serMemo = new Map<string, any>();
  function serialField(n: N): { before: N[] } | null { if (n.k !== 'type') return null; let s = serMemo.get(n.path); if (s !== undefined) return s; s = null; const T = SCHEMA.types[n.path]; if (T.k === 'struct') { const before: N[] = []; for (const f of T.fields) { if (f.expr === 'B.serialOf(o)') { s = { before }; break; } before.push(parse(f.ty)); } } serMemo.set(n.path, s); return s; }
  function peekSerial(n: N, s: { before: N[] }): number { const p0 = I.p; if (I.K) { for (const P of planOf(SCHEMA.types[n.path])) { if (P.f.expr === 'B.serialOf(o)') break; if (!I.K.field(P.q, P.f.ty)) continue; if (!I.K.desc(P.N)) { const K = I.K; I.K = null; sk(P.N); I.K = K; continue; } sk(P.N); } } else for (const x of s.before) sk(x); const v = dv.getUint32(I.p, true); I.p = p0; return v; }
  c.read = read; for (const o of madeRows) fresh.add(o);

  // ---- the state: the game's own fields, then every row
  const t3 = performance.now(); R.phases.makeRows = t3 - t2; I.K = pup ? keep : null;
  if (pup) { I.p = 4; const G0 = SCHEMA.types[SCHEMA.root], M0 = SCHEMA.types['crate::game::Mods']; const part = (T: any, P: Plan) => { const K = keep!; if (!K.field(P.q, P.f.ty)) return; if (!blob[I.p++]) return; if (!K.desc(P.N)) { keep = null; I.K = null; readField(T, P, g); keep = K; I.K = K; return; } readField(T, P, g); }; try { for (const P of planOf(G0)) { if (P.f.rust === 'm' && M0) { for (const P2 of planOf(M0)) part(M0, P2); continue; } part(G0, P); } } catch (e) { throw new Error(`in the game's fields at ${curQ}, byte ${I.p} of ${blob.length}: ${e}`); } if (I.p !== 4 + dv.getUint32(0, true)) throw new Error(`the game's fields ended at ${I.p}, not ${4 + dv.getUint32(0, true)}`); }
  else { I.p = 0; readStruct(SCHEMA.types[SCHEMA.root], g); }
  const t4 = performance.now(); R.phases.main = t4 - t3;
  if (pup) { if (pup.rows && pup.ctx) { pup.rows.__set(dv, blob); const ST = pup.slowTable!; pup.ctx.slow = (i: number, o: any) => { const E = ST[i]; const T = SCHEMA.types[E.type]; const P = planOf(T).find(x => x.f.rust === E.field)!; const K = keep; if (K && !K.desc(P.N)) { keep = null; I.K = null; readField(T, P, o); keep = K; I.K = K; return; } readField(T, P, o); }; pup.ctx.list = (id: string) => BK.listOf(id); pup.ctx.val = (k: number, cur: any) => read(parse(pup.vals![k]), cur === undefined || cur === null ? FRESH : cur); }
    (R as any).lastT = performance.now(); for (const { t, at, N, whole, entries } of heads) { if (!entries) continue; I.p = at; const L = BK.listOf(t.id); const fast = !pup.start && pup.rows?.[t.id]; if (process.env.PUP_DEBUG) process.stderr.write(`table ${t.id} at ${at}, ${entries} entries\n`); for (let e = 0; e < entries; e++) { const row = dv.getUint32(I.p, true); I.p += 4; if (row >= whole) { const K = keep; keep = null; I.K = null; read(N, L[row]); keep = K; I.K = K; continue; } if (fast) fast(L[row]); else read(N, L[row]); } if (process.env.PUP_PROF) { const nw = performance.now(); (R.phases as any)['t:' + t.ts] = nw - (R as any).lastT; (R as any).lastT = nw; } } }
  else for (const { t, n, at, N } of heads) { if (keep && !keep.desc(t.ty) && !(c as any)[`made:${t.id}`]) continue; I.p = at; const L = BK.listOf(t.id); const made = new Set((c as any)[`made:${t.id}`] ?? []); for (let i = 0; i < n; i++) { if (keep && !made.has(L[i]) && !keep.desc(t.ty)) { sk(N); continue; } read(N, L[i]); } }
  const t5 = performance.now(); R.phases.tables = t5 - t4; for (const f of c.fix) f();
  // ---- what the export keeps at the end
  I.K = null; BK.nextSerial = serialNext; I.p = tail; const nl = dv.getUint32(I.p, true); I.p += 4; if (!pup) BK.loose.clear();
  for (let i = 0; i < nl; i++) { const nodes = dv.getUint32(I.p, true); I.p += 4; const len = dv.getUint32(I.p, true); I.p += 4; const p: any = looseOrder[i]; if (p) { if (!p.own || p.own.length !== len) p.own = new Float32Array(len); p.home = null; for (let k = 0; k < len; k++) p.own[k] = dv.getFloat32(I.p + 4 * k, true); p.numNodes = nodes; BK.loose.set(p, i); } I.p += 4 * len; }
  if (pup) { const OE = SCHEMA.types['crate::out_events::OutEvent']; const n = dv.getUint32(I.p, true); I.p += 4; const out: { arm: string; e: any }[] = []; for (let i = 0; i < n; i++) { const a = dv.getUint16(I.p, true); I.p += 2; const A: string = OE.arms[a]; const m = A.match(/^(\w+)(?:\((.*)\))?$/)!; out.push({ arm: m[1], e: m[2] ? read(parse(m[2]), FRESH) : null }); } R.out = out; }
  for (const { t } of heads) { const M = (c as any)[`made:${t.id}`]; const h = hooks.made?.[t.ts]; if (M && h) for (const o of M) h(o, g); }
  PUT_BACKS.n++;   // (written from outside the game's own code: a copy kept current by being told of writes reads this, scene/boxMirror.ts)
  R.ms = performance.now() - t0; return R;
}
