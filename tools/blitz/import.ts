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
//
// AN ECHO (schema `echo`, hand2.cjs H.bridge.echo): a field Rust owns that the TypeScript cannot hold (a place in a level script, a memo a module keeps where the walk cannot write it, the copy of the box world in the module that
// holds it). A whole apply keeps a copy of its bytes beside the object and writes nothing; the puppet's export writes those bytes back (export.ts). So a re-export after an apply is Rust's bytes everywhere, and such a field needs no class of its own in a host's compare.
import GAME_REF_LIST from './game-ref.json';
import { SCHEMA, parse, bookOf, seedBook, helpersOf, compiledExpr, LOOSE, ECHO, LENS, forGame, classWord, isGameSide } from './export';
import { rawKey, PUT_BACKS, isSeatTable } from '../../src/math/side';

export type Hooks = { made?: Record<string, (row: any, g: any) => void>; gone?: Record<string, (row: any, g: any) => void>; classes?: Record<string, { prototype: object }>; values?: Record<string, any> };
export type Puppet = { lists: { step: Set<string>; start: Set<string> }; start: boolean; rows?: any; ctx?: any; slowTable?: { type: string; field: string }[]; vals?: string[] };
export type ApplyOpts = { only?: Set<string>; hooks?: Hooks; puppet?: Puppet };
export type Report = { changed?: boolean; out?: { arm: string; e: any }[]; phases: Record<string, number>; counts: Record<string, number>; made: Record<string, number>; noWay: Map<string, number>; noSetter: Map<string, number>; noWindow: Set<string>; deadSerials: number; bytes: number; ms: number };

const DEC = new TextDecoder();
const PROF = !!process.env.PUP_PROF, DEBUG = !!process.env.PUP_DEBUG;
type N = any;
const isTyped = (v: any) => ArrayBuffer.isView(v) && !(v instanceof DataView);
const isSide = (v: any) => v && typeof v.liveKeys === 'function';
const FRESH = Symbol('fresh');
const NUMERIC = new Set(['f64', 'f32', 'u8', 'i8', 'u16', 'i16', 'u32', 'i32', 'usize', 'u64', 'i64']);
const differs = (a: unknown, b: unknown) => !Object.is(a, b) && !(a == null && b == null);

// ---- reading past a value
type Keep = { field: (q: string, ty: string) => boolean; desc: (ty: string | N) => boolean };
export function skipper(b: Uint8Array, I: { p: number; K?: Keep | null }) { const dv = new DataView(b.buffer, b.byteOffset, b.byteLength); I.p = 0;
  const sk = (n: N): void => { switch (n.k) {
    case 'f64': case 'u64': case 'i64': I.p += 8; return; case 'f32': case 'u32': case 'i32': case 'usize': case 'id': I.p += 4; return; case 'bool': case 'u8': case 'i8': I.p += 1; return; case 'u16': case 'i16': I.p += 2; return; case 'unit': return;
    case 'str': I.p += 4 + dv.getUint32(I.p, true); return;
    case 'opt': if (b[I.p++]) sk(n.of); return;
    case 'vec': { const L = dv.getUint32(I.p, true); I.p += 4; const w = I.K && I.K.desc(n.of) ? -1 : fixedWidth(n.of); if (w >= 0) { I.p += L * w; return; } for (let k = 0; k < L; k++) sk(n.of); return; }
    case 'arr': { const w = I.K && I.K.desc(n.of) ? -1 : fixedWidth(n.of); if (w >= 0) { I.p += n.n * w; return; } for (let k = 0; k < n.n; k++) sk(n.of); return; }
    case 'tup': for (const x of n.of) sk(x); return;
    case 'map': { const L = dv.getUint32(I.p, true); I.p += 4; for (let k = 0; k < L; k++) { sk(n.K); sk(n.V); } return; }
    case 'set': { const L = dv.getUint32(I.p, true); I.p += 4; for (let k = 0; k < L; k++) sk(n.K); return; }
    case 'type': { const T = SCHEMA.types[n.path]; if (T.k === 'struct') { const K = I.K; if (K && !K.desc(n)) { I.K = null; sk(n); I.K = K; return; } if (!K) { for (const f of T.fields) sk(parse(f.ty)); return; } for (const P of planOf(T)) { if (!K.field(P.q, P.f.ty)) continue; if (!K.desc(P.N)) { I.K = null; sk(P.N); I.K = K; continue; } sk(P.N); } return; } if (T.k === 'enum') { I.p += 2; return; } const a = dv.getUint16(I.p, true); I.p += 2; if (a === 0xffff) return; if (T.k === 'wordEnum') { for (const f of T.arms[a].fields) sk(parse(f.ty)); return; } const t = armType(T, a); if (t) sk(parse(t)); return; } }
    throw new Error(`import: no way past ${n.k}`); };
  return { I, sk, dv }; }
type Plan = { f: any; q: string; N: N; way?: Way };
const planOf = (T: any): Plan[] => T.plan ??= T.fields.map((f: any) => ({ f, q: `${T.mod}::${T.name}.${f.rust}`, N: parse(f.ty) }));
const W = new Map<N, number>();
function fixedWidth(n: N): number { let w = W.get(n); if (w !== undefined) return w; switch (n.k) { case 'f64': case 'u64': case 'i64': w = 8; break; case 'f32': case 'u32': case 'i32': case 'usize': case 'id': w = 4; break; case 'bool': case 'u8': case 'i8': w = 1; break; case 'u16': case 'i16': w = 2; break; case 'unit': w = 0; break; case 'arr': { const e = fixedWidth(n.of); w = e < 0 ? -1 : e * n.n; break; } case 'tup': { w = 0; for (const x of n.of) { const e = fixedWidth(x); if (e < 0) { w = -1; break; } w += e; } break; } case 'type': { const T = SCHEMA.types[n.path]; if (T.k === 'enum') w = 2; else if (T.k === 'struct') { w = 0; W.set(n, -1); for (const f of T.fields) { const e = fixedWidth(parse(f.ty)); if (e < 0) { w = -1; break; } w += e; } } else w = -1; break; } default: w = -1; } W.set(n, w!); return w!; }
const armType = (T: any, a: number): string | null => T.k === 'data' ? T.vars[a].ty ?? null : (T.arms[a].match(/\((.*)\)/)?.[1] ?? null);

// ---- a field's way back, decided once
type Way = { kind: 'ref' | 'assign' | 'self' | 'path' | 'default' | 'top' | 'topField' | 'knob' | 'serial' | 'inverse' | 'skip' | 'none'; path?: string[]; dflt?: unknown; file?: string; name?: string; side?: string; fn?: Inverse; words?: string[]; zeroed?: boolean };
type Ctx = { g: any; B: any; BK: any; fix: (() => void)[]; R: Report; read: (n: N, cur: any) => any; sk: (n: N) => void; I: { p: number }; dv: DataView };
type Inverse = (o: any, v: any, c: Ctx) => void;
const skipField: Inverse = () => {};
/** the fields whose expression is no assignment. Each says how the value gets back, or why it need not: read `skipField` as "the export works this out from what is carried, or it is set up once and never changes" */
const INVERSE: Record<string, Inverse> = {
  // (a cast's ring stands as its maker gave it: a tick never changes it)
  'orders::OtherCast.ring': () => {},
  // (the row of `nearby` he is on, the object itself: a reader asks `nearby.includes(hover)`. Rust keeps a copy, which is that row by its place and what it is)
  'game::Game.hover': (o, v, c) => { c.fix.push(() => { o.hover = v ? (o.nearby as any[]).find(r => r.kind === v.kind && r.verb === v.verb && r.pos?.[0] === v.pos?.[0] && r.pos?.[2] === v.pos?.[2]) ?? v : null; }); },
  // (a yes or no on Rust's side, and the seat's camera on the puppet's, which the page sets and no read of the tick's finds. As a yes it would be a value that takes no field: `w.cam.peekOffset = null` throws)
  'player::WallHold.cam': () => {},
  'game::Game.view_proj': (o, v) => { o.viewProj = v; }, 'seats::SeatSet.view_proj': (o, v) => { o.viewProj = v; },
  'character::Character.who': skipField,   // (whoever holds this man as his `char`: rows)
  'character::Character.man_row': (o, v, c) => { if (v !== 0xffffffff) { o.S = c.g.men; o.r = v; } },
  'character::Character.man_kept': (o, v) => { if (v) { o.owner = v[0]; o.rank = v[1]; o.reason = v[2]; o.since = v[3]; } },
  'weapons::Firearm.def': (o, v, c) => { if (o.def?.id !== v) o.def = (c.R as any).values?.WEAPONS?.[v] ?? o.def; },
  'weapons::Firearm.kind': (o, v, c) => { const P = CLASSES.get(({ pistol: 'weapons::Pistol', rifle: 'weapons::Rifle', plain: 'weapons::Firearm' } as any)[v]); if (P && Object.getPrototypeOf(o) !== P) { Object.setPrototypeOf(o, P); if (v === 'rifle') { o.attachments ??= []; o.attIdx ??= 0; o.rings ??= 0; o.ringCd ??= 0; o.shells ??= 0; o.cams ??= 0; } } },   // (a row the Rust game made has no constructor run: what a `?? 0` field would read past as its default must stand on a rifle, whose readers count on it)
  'scene_collision::StaticCollision.pane_rule': skipField, 'scene_collision::StaticCollision.pane_rows': skipField, 'scene_collision::StaticCollision.glass_walkable': skipField, 'scene_collision::StaticCollision.fire_pass': skipField,   // (set up with the level)
  'destruction::Destruction.blast_listeners': skipField, 'destruction::RateCap.n': skipField, 'anim_skeleton::SkeletonInstance.rig': skipField, 'scene_level_load::SceneLevelLoadMod.level_names': skipField,
  'scene_boxes::BoxWorld.glass_open': skipField,   // (set up with the level)
  'glassbreak::GlassBreak.on_release': (o, v, c) => { o.onRelease = v ? c.g.shards.onRelease : null; },
  'nightbreach::NightBreach.bang_by': (o, v) => { o.bangBy = v ? v.view.doorTools : null; },
  // a cut is a new array. A count that moved says so; the array's numbers came in place just before, in this row. The seat's mark is the host's own unless Rust has taken the cut itself (a restart on its side)
  'camera::FollowCamera.desired_serial': (o, v) => { const r = LENS.get(o); if (!r) { LENS.set(o, { arr: o.desired, n: v }); return; } if (r.n !== v) { o.desired = [o.desired[0], o.desired[1], o.desired[2]]; r.n = v; } r.arr = o.desired; },
  'kitanim::KitanimStats.nv': (_o, v) => { const Q = (globalThis as any).__tpsTop?.['src/game/kitanim.ts']?.KITANIM_STATS?.nv; if (Q) { Q.hand = v.get(true) ?? 0; Q.nod = v.get(false) ?? 0; } },
  'cmd::Seat.mark': (o, v, c) => { c.fix.push(() => { const r = o.cam && LENS.get(o.cam); if (r && r.n === v) o.mark = o.cam.desired; }); },
  'chronicle::Chronicle.heard_of': (o, v, c) => { c.fix.push(() => { const T = o.heardOf; const put = (k: object, r: unknown) => T.set(k, r); if (T.clear) T.clear(); else for (const k of T.liveKeys()) T.delete(k); for (const [s, i] of v as Map<number, number>) put(objOfSerial(c, s), o.events[i]); }); },
  'fogofwar::FogOfWar.fired_n': (o, v, c) => { const T = o.firedN; const L = c.BK.listOf('RoomfightOpId'); const want = new Map<object, number>(); for (const [i, n] of v as Map<number, number>) want.set(L[i], n); for (const k of T.liveKeys()) if (!want.has(k)) T.delete(k); for (const [k, n] of want) T.set(k, n); },
  'anim_skeleton::Pose.at': (o, v, c) => { if (v >= LOOSE) { looseAt(c, o, v - LOOSE); return; } o.at = v; if (!o.home) { o.home = c.g.sim; o.own = null; } },
  'anim_skeleton::Pose.nodes': (o, v) => { if (o.numNodes !== v) { const C = Object.getPrototypeOf(o).constructor; const P = new C(v); o.numNodes = v; o.rows = P.rows; if (!o.home) o.own = P.own; } },
  'character::RagDrive.hips': (o, v, c) => { c.fix.push(() => { o.hips = o.ov[v]; }); },
  'character::RagDrive.limb_ov': (o, v, c) => { c.fix.push(() => { o.limbOv = (v as number[]).map(i => o.ov[i]); }); },
  'doorpath::DoorPathLastOfValue.w': (o, v, c) => { if (!v) { o.W = null; return; } const w = o.W ?? (o.W = {}); w.gd = v[0]; serialIs(c, w, v[1]); },
};
/** the classes that hold their game: a row of one that the apply makes has run no constructor, and its methods read `this.g` (gameref.py) */
const GAME_REF = new Set<string>(GAME_REF_LIST as string[]);
const CLASSES = new Map<string, object>();
/** a row whose numbers are the block's columns reads them through these: what its constructor would have set, the row itself coming with the fields */
const BLOCK_ROW: Record<string, (o: any, g: any) => void> = { Item: (o, g) => { o.S = g.items.store; o.things = g.props.all; o.kept = null; } };
let looseOrder: object[] = [];
const looseAt = (_c: Ctx, o: object, i: number) => { looseOrder[i] = o; };
const bySerial = new WeakMap<object, Map<number, object>>();
function serialIs(c: Ctx, o: object, s: number) { const BK = c.BK; const old = BK.serial.get(o); if (old === s) return; BK.serial.set(o, s); serialMap(BK).set(s, o); if (s >= BK.nextSerial && s < 0x40000000) BK.nextSerial = s + 1; }
function serialMap(BK: any): Map<number, object> { let M = bySerial.get(BK); if (!M) { bySerial.set(BK, M = new Map()); const O = BK.serialOfObjects?.(); if (O) for (const [k, v] of O) M.set(v, k); } return M; }
function objOfSerial(c: Ctx, s: number): object { const M = serialMap(c.BK); let o = M.get(s); if (!o) { o = { deadSerial: s }; c.R.deadSerials++; M.set(s, o); c.BK.serial.set(o, s); } return o; }
const AFTER_APPLY: ((g: any) => void)[] = (SCHEMA.afterApply as string[] ?? []).map(e => new Function('g', e) as (g: any) => void);
const ROW_BYTES = new WeakMap<object, Uint8Array>(); const sameBytes = (a: Uint8Array | undefined, b: Uint8Array) => { if (!a || a.length !== b.length) return false; for (let k = 0; k < a.length; k++) if (a[k] !== b[k]) return false; return true; };
/** everything `a` says, `b` says too: `a` was read from a walk that may name fewer fields than `b` has. An object's own place in a table compares by identity, since a row reads as its object */
function partOf(a: any, b: any, depth: number): boolean { if (a === b || a === undefined) return true; if (typeof a === 'number') return Object.is(a, b) || (a === 0 && (b === undefined || b === 0)); if (typeof a !== 'object' || a === null || b === null || typeof b !== 'object') return a === null ? b === null || b === undefined : false; if (depth > 24) return false;
  if (ArrayBuffer.isView(a)) { const x: any = a, y: any = b; if (x.length !== y.length) return false; for (let i = 0; i < x.length; i++) if (!Object.is(x[i], y[i])) return false; return true; }
  if (Array.isArray(a) || ArrayBuffer.isView(b)) { const y: any = b; if (a.length !== y.length) return false; for (let i = 0; i < a.length; i++) if (!partOf(a[i], y[i], depth + 1)) return false; return true; }
  if (a instanceof Map) { if (!(b instanceof Map) || a.size !== b.size) return false; for (const [k, v] of a) if (!b.has(k) || !partOf(v, b.get(k), depth + 1)) return false; return true; }
  if (a instanceof Set) { if (!(b instanceof Set) || a.size !== b.size) return false; for (const k of a) if (!b.has(k)) return false; return true; }
  for (const k of Object.keys(a)) if (!partOf(a[k], b[k], depth + 1)) return false; return true; }
const ownCopy = (o: any): any => Array.isArray(o) ? o.slice() : ArrayBuffer.isView(o) ? (o as any).slice() : Object.assign(Object.create(Object.getPrototypeOf(o)), o);
const wayCache = new Map<string, Way>();
function wayOf(q: string, f: any): Way { let w = wayCache.get(q); if (w) return w; const e: string | undefined = f.expr; let m: RegExpMatchArray | null;
  if (INVERSE[q]) w = { kind: 'inverse', fn: INVERSE[q] };
  else if (e === undefined && f.ts !== undefined && SCHEMA.serial[q] && (f.ty === 'u32' || f.ty === 'Option<u32>')) w = { kind: 'ref', path: [f.ts] };
  else if (e === undefined) w = f.ts !== undefined ? { kind: 'assign', path: [f.ts] } : { kind: 'skip' };
  else if (e === 'o') w = { kind: 'self' };
  else if ((m = e.match(/^o((?:\.[A-Za-z_$][\w$]*|\["[^"]+"\])+)$/))) w = { kind: 'path', path: [...m[1].matchAll(/\.([A-Za-z_$][\w$]*)|\["([^"]+)"\]/g)].map(x => x[1] ?? x[2]) };
  else if ((m = e.match(/^o\.([A-Za-z_$][\w$]*) \?\? (.+)$/))) w = { kind: 'default', path: [m[1]], dflt: JSON.parse(m[2]) };
  else if ((m = e.match(/^B\.top\("([^"]+)", "([^"]+)", "([^"]*)"\)$/))) w = { kind: 'top', file: m[1], name: m[2], side: m[3] };
  else if ((m = e.match(/^B\.top\("([^"]+)", "([^"]+)", "([^"]*)"\)\.([\w$]+)$/))) w = { kind: 'topField', file: m[1], name: m[2], side: m[3], path: [m[4]] };
  else if ((m = e.match(/^B\.keysOf\(B\.top\("([^"]+)", "([^"]+)", "([^"]*)"\)\.([\w$]+), (\[.*\])\)$/))) w = { kind: 'topField', file: m[1], name: m[2], side: m[3], path: [m[4]], words: JSON.parse(m[5]) };
  else if ((m = e.match(/^B\.nonZero\(B\.top\("([^"]+)", "([^"]+)", "([^"]*)"\)\.([\w$]+)\)$/))) w = { kind: 'topField', file: m[1], name: m[2], side: m[3], path: [m[4]], zeroed: true };
  else if (/^B\.knob\(/.test(e)) w = { kind: 'knob', name: e.match(/^B\.knob\("(\w+)"\)/)?.[1] };
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
  const deep = (t: string | N): boolean => { const n = typeof t === 'string' ? parse(t) : t; let d = memoD.get(n); if (d === undefined) memoD.set(n, d = walk(n)); return d; };
  /** is this value walked filtered? Only a struct or a tuple: a value inside a list, an Option or a map goes whole (puppetgen.cjs) */
  const desc = (t: string | N): boolean => { const n = typeof t === 'string' ? parse(t) : t; return n.k !== 'type' && n.k !== 'tup' ? false : deep(n); };
  K = { field: (q, ty) => { let v = memoF.get(q); if (v === undefined) memoF.set(q, v = only.has(q) || deep(ty)); return v; }, desc }; keepMemo.set(only, K); return K; }

// ---- the walk
export function applyGame(g: any, blob: Uint8Array, knobs: Record<string, unknown>, opts: ApplyOpts = {}): Report { return forGame(g, () => g.col.writes(() => applyGameNow(g, blob, knobs, opts))); }
function applyGameNow(g: any, blob: Uint8Array, knobs: Record<string, unknown>, opts: ApplyOpts = {}): Report {
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
  { const K: Map<string, number> = (BK as any).known ??= new Map(); for (const h of heads) K.set(h.t.id, h.n); }   // (how many rows of each table the Rust game has: a row past that was made in TypeScript since, and neither the scan nor a crossing may name it)
  serialNext = dv.getUint32(I.p, true); I.p += 4; tail = I.p;
  // (WHETHER THE RUST GAME'S STATE MOVED AT ALL since the last export: no part and no row sent, the same table sizes. A host then has no need to look at the puppet again)
  if (pup && !pup.start) { const mainLen = dv.getUint32(0, true); let any = heads.some(h => h.entries > 0 || h.n !== h.whole); for (let k = 4; !any && k < 4 + mainLen; k++) if (blob[k]) any = true; R.changed = any; } else R.changed = true;
  const t2 = performance.now(); R.phases.skim = t2 - t1; for (const { t, n } of heads) { const L = BK.listOf(t.id); if (L.length >= n) continue; const T0 = SCHEMA.types[t.ty.replace(/^alloc::boxed::Box<(.*)>$/, '$1')]; const proto = CLASSES.get(`${T0?.mod}::${T0?.name}`) ?? (L.length ? Object.getPrototypeOf(L[0]) : Object.prototype); const made: any[] = []; while (BK.listOf(t.id).length < n) { const o = Object.create(proto); BK.idOf(t.id, o); madeRows.add(o); made.push(o); } R.made[t.ts] = made.length; (c as any)[`made:${t.id}`] = made; }

  // ---- a new row that stands in the game's block, before its fields go in
  for (const { t } of heads) { const M = (c as any)[`made:${t.id}`]; const f = BLOCK_ROW[t.ts]; if (M && f) for (const o of M) f(o, g); }
  // ---- values
  const mapGet = (m: any, k: any) => m instanceof Map || isSide(m) ? m.get?.(k) : m?.[k];
  const orderSide = (T: any, keys: object[]) => { const K = T.sideKeys; if (!K) return; const now = T.liveKeys(); if (now.length === keys.length && now.every((x: object, i: number) => x === keys[i])) return; K.refs = keys.map(k => new WeakRef(k)); };
  const rKey = (K: N, cur: any): any => { if (K.k === 'u32' && cur === 'object-key') { const s = dv.getUint32(I.p, true); I.p += 4; return { serial: s }; } return read(K, undefined); };
  /** AN OBJECT REACHED A SECOND TIME IN THIS APPLY stands in two places. The same bytes keep the one object. Other bytes are of two kinds. One value under another filter (a row's lists walk it, another holder's do not): everything the second walk says, the object already says, and it stays one object. A value that truly differs (the TypeScript shares by reference what Rust holds as values, and they have parted): this holder gets its own copy of the object, with all the object had and this walk's values over it, so a field no list carries is not lost; from then on the holder has its own, and nothing is met again */
  const seen = new Map<object, [number, number]>();
  function read(n: N, cur: any): any { R.counts.read++; if (n.k === 'type' && cur && typeof cur === 'object' && cur !== FRESH) { const ser = serialField(n); if (ser) { const was = BK.serial.get(cur); if (was !== undefined) { const s = peekSerial(n, ser); if (s !== was) cur = serialMap(BK).get(s) ?? FRESH; } } }   /* (a value with a serial that is another thing now, not this one written over: a new clear, a new mission. A reader holds it by identity) */ if (typeof cur === 'number' && n.k === 'f64') { const v = dv.getFloat64(I.p, true); I.p += 8; return v; } if (PROF) { const T = ((globalThis as any).__byQ ??= new Map()); T.set(curQ, (T.get(curQ) ?? 0) + 1); }
    if (cur === FRESH) cur = n.k === 'vec' ? (dv.getUint32(I.p, true) === 0 ? [] : undefined) : n.k === 'map' ? new Map() : n.k === 'set' ? new Set() : n.k === 'bool' ? false : n.k === 'str' ? '' : n.k === 'arr' || n.k === 'tup' || n.k === 'type' || n.k === 'opt' || n.k === 'id' ? undefined : 0;
    else if (cur && typeof cur === 'object' && (n.k === 'arr' || n.k === 'tup' || n.k === 'vec' || (n.k === 'type' && SCHEMA.types[n.path].k === 'struct'))) { const at = seen.get(cur); const p0 = I.p; if (at) { sk(n); const L = I.p - p0; let eq = L === at[1] - at[0]; if (eq) for (let i = 0; i < L; i++) if (blob[p0 + i] !== blob[at[0] + i]) { eq = false; break; } if (eq) return cur; I.p = p0; const made = read(n, FRESH); if (partOf(made, cur, 0)) { R.metSame = (R.metSame ?? 0) + 1; return cur; } I.p = p0; R.metOther = (R.metOther ?? 0) + 1; return readInner(n, ownCopy(cur)); } const v = readInner(n, cur); seen.set(cur, [p0, I.p]); return v; }
    return readInner(n, cur); }
  function readInner(n: N, cur: any): any {
    // a table keyed by the game, or by the one thing of its kind: what is applied is its row
    if (isSide(cur) && n.k !== 'opt' && n.k !== 'bool') {
      const P = isGameSide(cur) || (!(n.k === 'map' && n.K.k === 'id' && n.K.id === 'PlayerId') && cur.liveKeys().includes(G));
      if (P) { const v = read(n, cur.get ? cur.get(G) : cur.has(G)); if (cur.set) cur.set(G, v); return cur; }
      const K = cur.liveKeys(); if (K.length && SCHEMA.onceKeys.includes(classWord(K[0]))) { const v = read(n, cur.get ? cur.get(K[0]) : true); if (cur.set) cur.set(K[0], v); return cur; }
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
      case 'opt': { const t = blob[I.p++]; if (isSide(cur)) { const K = cur.liveKeys(); if (!(n.of.k === 'map' && n.of.K.k === 'id') && n.of.k !== 'set') { const at = K.includes(G) ? G : K[0] ?? G; if (!t) { cur.delete(at); return cur; } const v = read(n.of, cur.get ? cur.get(at) ?? FRESH : true); if (cur.set) cur.set(at, v); else if (cur.add) cur.add(at); return cur; } if (!t) { for (const k of K) cur.delete(k); return cur; } if (!K.length && n.of.k === 'map' && dv.getUint32(I.p, true) === 0) { I.p += 4; cur.set(G, new Map()); return cur; } return read(n.of, cur); } if (!t) return n.of.k === 'opt' || cur === undefined ? undefined : null; if (n.of.k === 'opt') { if (!blob[I.p++]) return null; return read(n.of.of, cur === null || cur === undefined ? FRESH : cur); }   /* Option<Option<T>>: None is undefined, Some(None) is null (kitanim.ts, doortools.ts) */ return read(n.of, cur === null || cur === undefined ? FRESH : cur); }
      case 'vec': { const L = dv.getUint32(I.p, true); I.p += 4; if (isTyped(cur)) { const a = cur.length === L ? cur : new cur.constructor(L); for (let i = 0; i < L; i++) a[i] = readN(n.of); return a; } if (cur === null || cur === undefined) { if (!L && cur === null) return cur; cur = n.of.k === 'f32' ? new Float32Array(L) : n.of.k === 'u8' ? new Uint8Array(L) : n.of.k === 'i32' ? new Int32Array(L) : n.of.k === 'u32' ? new Uint32Array(L) : []; if (isTyped(cur)) { for (let i = 0; i < L; i++) cur[i] = readN(n.of); return cur; } }
        if (n.of.k === 'type' && SCHEMA.keyedRows?.[n.of.path]) { const a = cur as any[], was = a.slice(), used = new Set<any>(); for (let i = 0; i < L; i++) { const p0 = I.p; sk(n.of); const B0 = blob.subarray(p0, I.p); let x = was.find(o => !used.has(o) && sameBytes(ROW_BYTES.get(o), B0)); if (x) { used.add(x); a[i] = x; continue; } I.p = p0; x = read(n.of, FRESH); ROW_BYTES.set(x, blob.slice(p0, I.p)); used.add(x); a[i] = x; } a.length = L; return a; }
        const ser = serialField(n.of); const a = cur as any[]; const was = ser ? a.slice() : null; for (let i = 0; i < L; i++) { let x = a[i] === undefined ? FRESH : a[i]; if (ser) { const s = peekSerial(n.of, ser); const hit = serialMap(BK).get(s); x = hit ?? (x === FRESH || (was!.includes(x) && BK.serial.get(x) !== s) ? FRESH : x); } a[i] = read(n.of, x); } a.length = L; return a; }
      case 'arr': { if (pup && !pup.start && !SCHEMA.inPlace?.[curQ] && Array.isArray(cur) && fixedWidth(n.of) >= 0 && n.of.k !== 'type') { const a: any[] = new Array(n.n); let same = cur.length === n.n; for (let i = 0; i < n.n; i++) { a[i] = read(n.of, FRESH); if (same && !Object.is(a[i], cur[i])) same = false; } return same ? cur : a; }   // (a step does not visit every holder of a shared array: a changed one is a new one, as in the compiled rows)
        if (cur && typeof cur.length === 'number') { if (n.of.k === 'f64') { for (let i = 0; i < n.n; i++) { const v = dv.getFloat64(I.p, true); I.p += 8; if (!Object.is(v, cur[i])) cur[i] = v; } return cur; } for (let i = 0; i < n.n; i++) { const v = read(n.of, cur[i] ?? FRESH); if (!Object.is(v, cur[i])) cur[i] = v; } return cur; } const a: any[] = []; for (let i = 0; i < n.n; i++) a[i] = read(n.of, FRESH); return a; }
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
  const firstKey = (m: any): unknown => { if (m instanceof Map || m instanceof Set) { for (const k of m instanceof Map ? m.keys() : m) return k; return undefined; } return keysIn(m)[0]; };
  function keyOf(K: N, curMap: any): any { if (K.k === 'u32') { const v = dv.getUint32(I.p, true); I.p += 4; const first = firstKey(curMap); if (first && typeof first === 'object') return objOfSerial(c, v); if (isSide(curMap)) return objOfSerial(c, v); return v; }
    if (K.k === 'id' && K.id === 'PlayerId') { const row = readId(K, undefined); if (isSeatTable(curMap)) { for (const s of g.seats ?? []) if (s.view?.player === row) return s.view; return g; }   /* (a seat's table: keyed by the view the TypeScript asks by, whatever the table holds yet) */ for (const k of keysIn(curMap)) if (k && typeof k === 'object' && (k as any).player === row) return k; return row; }
    if (K.k === 'f64') { const v = dv.getFloat64(I.p, true); I.p += 8; if (curMap && !(curMap instanceof Map) && !isSide(curMap) && !(curMap instanceof Set)) return String(v); const first = firstKey(curMap); return typeof first === 'string' ? String(v) : v; }
    return read(K, undefined); }
  const keysIn = (m: any): unknown[] => m instanceof Map ? [...m.keys()] : m instanceof Set ? [...m] : isSide(m) ? m.liveKeys() : m && typeof m === 'object' ? Object.keys(m) : [];
  function readMap(n: N, cur: any): any { const L = dv.getUint32(I.p, true); I.p += 4; if (cur === null || cur === undefined) { if (!L && cur === null) return cur; cur = new Map(); } const keys: any[] = [], vals: any[] = [];
    for (let i = 0; i < L; i++) { const k = keyOf(n.K, cur); keys.push(k); vals.push(read(n.V, mapGet(cur, k))); }
    if (cur instanceof Map) { const now = [...cur.keys()]; if (now.length === L && now.every((k, i) => k === keys[i])) { for (let i = 0; i < L; i++) if (cur.get(keys[i]) !== vals[i]) cur.set(keys[i], vals[i]); } else { cur.clear(); for (let i = 0; i < L; i++) cur.set(keys[i], vals[i]); } return cur; }
    if (isSide(cur)) { const want = new Set(keys); for (const k of cur.liveKeys()) if (!want.has(k)) cur.delete(k); for (let i = 0; i < L; i++) if (keys[i] && typeof keys[i] === 'object') cur.set(keys[i], vals[i]); orderSide(cur, keys); return cur; }
    if (typeof cur === 'object') { const want = new Set(keys.map(String)); for (const k of Object.keys(cur)) if (!want.has(k)) delete cur[k]; for (let i = 0; i < L; i++) cur[keys[i]] = vals[i]; return cur; }
    return cur; }
  function readSet(n: N, cur: any): any { const L = dv.getUint32(I.p, true); I.p += 4; if (cur === null || cur === undefined) { if (!L && cur === null) return cur; cur = new Set(); } const keys: any[] = []; for (let i = 0; i < L; i++) keys.push(keyOf(n.K, cur));
    if (cur instanceof Set) { const now = [...cur]; if (!(now.length === L && now.every((k, i) => k === keys[i]))) { cur.clear(); for (const k of keys) cur.add(k); } return cur; }
    if (isSide(cur)) { const want = new Set(keys); for (const k of cur.liveKeys()) if (!want.has(k)) cur.delete(k); for (const k of keys) if (k && typeof k === 'object') cur.add(k); orderSide(cur, keys); return cur; }
    if (typeof cur === 'object') { for (const k of Object.keys(cur)) delete cur[k]; for (const k of keys) cur[k] = true; return cur; }
    return cur; }
  function newOf(T: any): any { const P = CLASSES.get(`${T.mod}::${T.name}`); const o = P ? Object.create(P) : {}; if (P && GAME_REF.has(T.name)) o.g = g; fresh.add(o); return o; }
  const fresh = new WeakSet<object>();
  function readType(n: N, cur: any): any { const T = SCHEMA.types[n.path];
    if (T.k === 'struct') { const o = cur && typeof cur === 'object' ? cur : newOf(T); const K = keep; if (K && !K.desc(n)) { keep = null; I.K = null; readStruct(T, o); keep = K; I.K = pup ? K : null; return o; } readStruct(T, o); return o; }
    if (T.k === 'enum') { const w = dv.getUint16(I.p, true); I.p += 2; return T.words[w] ?? cur; }   /* (0xffff: an arm the schema does not know: the page keeps what it had) */
    const a = dv.getUint16(I.p, true); I.p += 2; if (a === 0xffff) return cur;
    // (authored: a lesson's test stands in the TypeScript as it was written. WHERE NOTHING STANDS the value is made, in the shape the export reads one in: its
    //  word and its arm's fields, a number that is 0 kept as 0. What a smoke source rides is such a value, new with each out event: without it every source
    //  that rides a barrel, an ejection port or a spark stayed where it was filed)
    if (T.k === 'wordEnum') { const A = T.arms[a]; if (cur !== null && cur !== undefined && cur !== FRESH) { for (const f of A.fields) sk(parse(f.ty)); return cur; }
      const o: any = { [T.word]: A.word }; for (const f of A.fields) { const N = parse(f.ty); o[f.ts] = read(N, N.k === 'f64' || N.k === 'f32' || N.k === 'usize' || N.k === 'u32' ? 0 : N.k === 'bool' ? false : N.k === 'str' ? '' : FRESH); } return o; }
    if (T.k === 'handEnum') { const t = armType(T, a); if (t) { sk(parse(t)); return cur; } return T.words ? T.words[a] : cur; }
    const A = T.vars[a]; if (!A.ty) return A.str ?? (cur && typeof cur === 'object' && (cur.kind ?? cur.type) === A.kw ? cur : A.kw !== undefined ? { kind: A.kw } : cur);
    const AT = SCHEMA.types[A.ty]; const same = (() => { if (cur === null || cur === undefined || typeof cur !== 'object') return false; if (AT?.k === 'struct') { if (A.kw !== undefined) return (cur.kind ?? cur.type) === A.kw; if (A.sn && classWord(cur) === A.sn) return true; return AT.fields.filter((f: any) => f.ts).every((f: any) => f.ts in cur); } return true; })();
    if (A.sn && !AT && same && classWord(cur) !== A.sn) return read(parse(A.ty), FRESH);
    const made = read(parse(A.ty), same ? cur : FRESH);
    if (A.kw !== undefined && made !== null && typeof made === 'object' && (made.kind ?? made.type) !== A.kw) made.kind = A.kw;   // (an arm's keyword is no field of its struct: an object the apply made carries the word as the TypeScript's own does, or a reader that asks `t.kind` reads the last arm's answer)
    return made; }
  let curQ = '';
  /** one field of a struct, its bytes next */
  function readField(T: any, P: Plan, o: any) { const { f, q, N } = P; curQ = q;
      if (f.echo) { const p0 = I.p; sk(N); if (pup && N.k === 'type') return;   /* (a struct on a filtered apply came filtered: only whole bytes are an echo. A list, an Option or a map goes whole) */ let E = ECHO.get(o); if (!E) ECHO.set(o, E = new Map()); const e0 = E.get(q), L0 = I.p - p0; if (!e0 || e0.length !== L0 || !sameBytes(e0, blob.subarray(p0, I.p))) E.set(q, blob.slice(p0, I.p)); return; }
      const w = P.way ??= wayOf(q, f);
      switch (w.kind) {
        case 'ref': { const key = w.path![0]; const has = N.k === 'opt' ? blob[I.p++] : 1; let v = 0; if (has) { v = dv.getUint32(I.p, true); I.p += 4; } const cur = o[key];   // (the object of that serial, once every serial of this apply stands: a row later in the walk may be the one that names it)
          if (typeof cur === 'number') { if (cur !== v) o[key] = v; break; } c.fix.push(() => { o[key] = v ? objOfSerial(c, v) : (o[key] === undefined ? undefined : null); }); break; }
        case 'assign': { const key = w.path![0]; const cur = o[key]; if (f.none && N.k === 'opt' && !blob[I.p]) { I.p++; if (f.keepNone && cur) break; const v = f.none === 'null' ? null : undefined; if (!Object.is(cur, v)) o[key] = v; break; }   // (None, as this field's type says it: a reader tests the one it admits, boxes.ts:841 `b.marks !== undefined`)
  const v = read(N, f.rec && (cur === undefined || cur === null) ? {} : cur === undefined && fresh.has(o) ? FRESH : cur); if (N.k === 'opt' && N.of.k === 'opt' ? !Object.is(v, cur) : differs(v, cur)) { R.counts.assign++; try { o[key] = v; } catch { R.noSetter.set(q, (R.noSetter.get(q) ?? 0) + 1); } } break; }
        case 'self': readStruct(SCHEMA.types[N.path], o); break;
        case 'path': { let h = o; const P = w.path!; for (let i = 0; i < P.length - 1; i++) { h = h?.[P[i]]; } if (h === null || h === undefined) { sk(N); break; } const key = P[P.length - 1]; const cur = h[key]; const v = read(N, cur === undefined && fresh.has(h) ? FRESH : cur); if (differs(v, cur)) { try { h[key] = v; } catch { R.noSetter.set(q, (R.noSetter.get(q) ?? 0) + 1); } } break; }
        case 'default': { const key = w.path![0]; const cur = o[key]; const v = read(N, cur === undefined ? w.dflt : cur); if (cur !== undefined || JSON.stringify(v) !== JSON.stringify(w.dflt)) o[key] = v; break; }
        case 'top': { const T0 = (globalThis as any).__tpsTop?.[w.file!]; const V = B.varOf(w.side!); const cur = T0 && w.name! in T0 ? T0[w.name!] : V ? V.get() : B.rootOf(w.side!); if (cur === undefined && !V && !(T0 && w.name! in T0)) { R.noWindow.add(w.side!); sk(N); break; } const v = read(N, cur); if (differs(v, cur)) { if (V) V.set(v as never); else if (T0 && Object.getOwnPropertyDescriptor(T0, w.name!)?.set) T0[w.name!] = v; else { const k = `module ${w.side}: no setter`; R.noWay.set(k, (R.noWay.get(k) ?? 0) + 1); } } break; }
        case 'topField': { const T = (globalThis as any).__tpsTop?.[w.file!]?.[w.name!]; if (!T || typeof T !== 'object') { R.noWindow.add(w.side!); sk(N); break; } const key = w.path![0]; if (w.zeroed) { const Q = T[key], v: Map<any, number> = read(N, new Map()); for (const k of Object.keys(Q)) Q[k] = 0; for (const [k, x] of v) Q[k] = x; break; } if (w.words) { const Q = T[key], v = read(N, w.words.map((k: string) => Q[k])); w.words.forEach((k: string, j: number) => { Q[k] = v[j]; }); break; } const cur = T[key]; const v = read(N, cur); if (differs(v, cur)) T[key] = v; break; }
        case 'knob': { const K = knobs[w.name!]; if (!K || typeof K !== 'object' || N.k !== 'type' || (globalThis as any).__settingsBreak) sk(N); else readStruct(SCHEMA.types[N.path], K); break; }   // (a table stands where it stands, for the host and for the next read: its rows are written in place)
        case 'skip': sk(N); break;
        case 'serial': { const s = dv.getUint32(I.p, true); I.p += 4; serialIs(c, o, s); break; }
        case 'inverse': { const v = read(N, N.k === 'map' ? new Map() : NUMERIC.has(N.k) ? 0 : undefined); w.fn!(o, v, c); break; }   // (a number stands whatever it is: read over nothing a 0 reads as nothing, and the first man's row is 0)
        default: R.noWay.set(q, (R.noWay.get(q) ?? 0) + 1); sk(N); }
  }
  function readStruct(T: any, o: any) { for (const P of planOf(T)) { const K = keep; if (DEBUG && I.p > blob.length) throw new Error('past the end at ' + curQ); if (K && (pup || !fresh.has(o))) { if (!K.field(P.q, P.f.ty)) { if (!pup) sk(P.N); continue; } if (!K.desc(P.N)) { keep = null; I.K = null; readField(T, P, o); keep = K; I.K = pup ? K : null; continue; } } readField(T, P, o); } }
  /** a struct that carries `B.serialOf(o)`: found by its serial wherever it now stands in a list */
  const serMemo = new Map<string, any>();
  function serialField(n: N): { before: N[] } | null { if (n.k !== 'type') return null; let s = serMemo.get(n.path); if (s !== undefined) return s; s = null; const T = SCHEMA.types[n.path]; if (T.k === 'struct') { const before: N[] = []; for (const f of T.fields) { if (f.expr === 'B.serialOf(o)') { s = { before }; break; } before.push(parse(f.ty)); } } serMemo.set(n.path, s); return s; }
  function peekSerial(n: N, s: { before: N[] }): number { const p0 = I.p; if (I.K) { for (const P of planOf(SCHEMA.types[n.path])) { if (P.f.expr === 'B.serialOf(o)') break; if (!I.K.field(P.q, P.f.ty)) continue; if (!I.K.desc(P.N)) { const K = I.K; I.K = null; sk(P.N); I.K = K; continue; } sk(P.N); } } else for (const x of s.before) sk(x); const v = dv.getUint32(I.p, true); I.p = p0; return v; }
  c.read = read; for (const o of madeRows) fresh.add(o);

  // ---- the state: the game's own fields, then every row
  const t3 = performance.now(); R.phases.makeRows = t3 - t2; I.K = pup ? keep : null;
  if (pup) { if (pup.rows && pup.ctx) { pup.rows.__set(dv, blob); const ST = pup.slowTable!; pup.ctx.slow = (i: number, o: any) => { const E = ST[i]; const T = SCHEMA.types[E.type]; const P = planOf(T).find(x => x.f.rust === E.field)!; const K = keep; if (K && !K.desc(P.N)) { keep = null; I.K = null; readField(T, P, o); keep = K; I.K = K; return; } readField(T, P, o); }; pup.ctx.list = (id: string) => BK.listOf(id); pup.ctx.made = (p: string) => newOf(SCHEMA.types[p]); pup.ctx.val = (k: number, cur: any) => read(parse(pup.vals![k]), cur === undefined || cur === null ? FRESH : cur); } }
  if (pup) { I.p = 4; const G0 = SCHEMA.types[SCHEMA.root], M0 = SCHEMA.types['crate::game::Mods'], S0 = SCHEMA.types['crate::game::Stats']; const part = (T: any, P: Plan) => { const K = keep!; if (!K.field(P.q, P.f.ty)) return; if (!blob[I.p++]) return; const ft = !pup.start && pup.rows?.__top?.[P.q]; if (ft) { curQ = P.q; ft(g); return; } if (!K.desc(P.N)) { keep = null; I.K = null; readField(T, P, g); keep = K; I.K = K; return; } readField(T, P, g); }; try { for (const P of planOf(G0)) { if (P.f.rust === 'm' && M0) { for (const P2 of planOf(M0)) part(M0, P2); continue; } if (P.f.rust === 'stats' && S0) { for (const P2 of planOf(S0)) part(S0, P2); continue; } part(G0, P); } } catch (e) { throw new Error(`in the game's fields at ${curQ}, byte ${I.p} of ${blob.length}: ${e}`); } if (I.p !== 4 + dv.getUint32(0, true)) throw new Error(`the game's fields ended at ${I.p}, not ${4 + dv.getUint32(0, true)}`); }
  else { I.p = 0; readStruct(SCHEMA.types[SCHEMA.root], g); }
  const t4 = performance.now(); R.phases.main = t4 - t3;
  if (pup) { 
    (R as any).lastT = performance.now(); for (const { t, at, N, whole, entries } of heads) { if (!entries) continue; I.p = at; const L = BK.listOf(t.id); const fast = !pup.start && pup.rows?.[t.id]; if (DEBUG) process.stderr.write(`table ${t.id} at ${at}, ${entries} entries\n`); for (let e = 0; e < entries; e++) { const row = dv.getUint32(I.p, true); I.p += 4; if (row >= whole) { const K = keep; keep = null; I.K = null; try { read(N, L[row]); } catch (e) { throw new Error(`in a new row of ${t.id}, row ${row}, at ${curQ}, byte ${I.p} of ${blob.length}: ${(e as any)?.message ?? e}`); } keep = K; I.K = K; continue; } try { if (fast) fast(L[row]); else read(N, L[row]); } catch (e) { throw new Error(`in table ${t.id}, row ${row} of ${L.length}, ${entries} entries, byte ${I.p} of ${blob.length}: ${(e as any)?.message ?? e}`); } } if (PROF) { const nw = performance.now(); (R.phases as any)['t:' + t.ts] = nw - (R as any).lastT; (R as any).lastT = nw; } } }
  else for (const { t, n, at, N } of heads) { if (keep && !keep.desc(t.ty) && !(c as any)[`made:${t.id}`]) continue; I.p = at; const L = BK.listOf(t.id); const made = new Set((c as any)[`made:${t.id}`] ?? []); for (let i = 0; i < n; i++) { if (keep && !made.has(L[i]) && !keep.desc(t.ty)) { sk(N); continue; } read(N, L[i]); } }
  const t5 = performance.now(); R.phases.tables = t5 - t4; for (const f of c.fix) f();
  // A MAN MADE BY THE RUST GAME: his Guard or Player reads the same store row as his Character (game.ts sameMan), which the tables above have just given it
  for (const { t } of heads) if (t.ts === 'Guard' || t.ts === 'Player') for (const o of (c as any)[`made:${t.id}`] ?? []) if (o.char) { o.S = o.char.S; o.r = o.char.r; }
  for (const { t } of heads) if (t.ts === 'PoseSet') for (const o of (c as any)[`made:${t.id}`] ?? []) if (o.home === undefined) o.home = null;   // (skeleton.ts holds null for a set with no block; a made row has run no field's initial)
  for (const f of AFTER_APPLY) f(g);
  // THE ONE CLOSURE A MADE RECORD HOLDS: the second cast's `mine` (qrf.ts squadCast's call). Rust answers it by the team's rows and carries no closure
  { const Q = g.qrf, O = Q?.cast?.orders?.cast; if (O && !O.mine) { O.mine = (gd: any) => Q.men.includes(gd); if (O.ring?.has('*')) O.ring.has = () => true; } }   // (orders.ts EVERY_KEY: a set that holds every key, carried as its one word)
  // ---- what the export keeps at the end
  I.K = null; BK.nextSerial = serialNext; BK.rustDeals = true; I.p = tail; const nl = dv.getUint32(I.p, true); I.p += 4; if (!pup) BK.loose.clear();
  for (let i = 0; i < nl; i++) { const nodes = dv.getUint32(I.p, true); I.p += 4; const len = dv.getUint32(I.p, true); I.p += 4; const p: any = looseOrder[i]; if (p) { if (!p.own || p.own.length !== len) p.own = new Float32Array(len); p.home = null; for (let k = 0; k < len; k++) p.own[k] = dv.getFloat32(I.p + 4 * k, true); p.numNodes = nodes; BK.loose.set(p, i); } I.p += 4 * len; }
  if (pup) { const OE = SCHEMA.types['crate::out_events::OutEvent']; const n = dv.getUint32(I.p, true); I.p += 4; const out: { arm: string; e: any }[] = []; for (let i = 0; i < n; i++) { const a = dv.getUint16(I.p, true); I.p += 2; const A: string = OE.arms[a]; const m = A.match(/^(\w+)(?:\((.*)\))?$/)!; out.push({ arm: m[1], e: m[2] ? read(parse(m[2]), FRESH) : null }); } R.out = out; }
  for (const { t } of heads) { const M = (c as any)[`made:${t.id}`]; const h = hooks.made?.[t.ts]; if (M && h) for (const o of M) h(o, g); }
  PUT_BACKS.n++;   // (written from outside the game's own code: a copy kept current by being told of writes reads this, scene/boxMirror.ts)
  R.ms = performance.now() - t0; return R;
}
