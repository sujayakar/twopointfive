// THE HEADLESS HOST OF THE RUST GAME (PLAY.md): a TypeScript `Game` made a PUPPET of a game that steps in the wasm face (rust/game_face). From the first
// tick the Rust game is the only game that steps: `Game.update`'s own lines run as they are (the record sampled, hostBefore, hostAfter), the step between
// them is `RustHost.step`, hung on the game object in place of `stepSeats`, and after it the Rust game's state is applied onto the TypeScript objects in
// place, so whatever reads the game (the scripted player, a tally, a probe) reads what the Rust game holds.
//
//   await RustHost.ready(); const H = RustHost.attach(game, opts)      before the game's first update; every later `game.update(…)` steps the Rust game
//
// TWO WAYS TO KEEP THE PUPPET AND THE RUST GAME ONE GAME:
//   fast (the default)   after a step the FILTERED export (`face_puppet`: the listed fields of the rows that changed) is applied; what the TypeScript
//                        round a step does to the game goes to the Rust game as a call when it is made (an order, a pick of a verb, the light meter, a
//                        checkpoint, a question that floods a room, the box world's listing, the camera's ask). Nothing is exported from the puppet.
//                        A WHOLE apply is made where something is about to read what the filtered list does not carry: before a save is taken, when a
//                        man goes down, a door order begins or ends, the mission's stage moves or it ends (the reads the filtered list is known to
//                        lack), after a room is flooded, and every `wholeEvery` ticks if asked.
//   audit                beside the filtered apply, the WHOLE state is applied every tick, and as each tick opens the puppet is exported whole and held
//                        to the bytes last applied: what differs outside the host's half is something TypeScript wrote into the game between two
//                        ticks. It is named, counted and handed to the Rust game whole (`face_import_into`). Slow (two walks of the state a tick):
//                        the way to find a write the fast way has no call for.
import { readFileSync, existsSync, writeFileSync, appendFileSync } from 'node:fs';

export interface HostOpts {
  /** the module (default: rust/game_face's release build) */
  wasm?: string;
  audit?: boolean;
  /** fast: a whole apply every N ticks beside the filtered one (0: only where the host decides one is owed) */
  wholeEvery?: number;
  /** audit: the ticks on which the headless host lists the box world (headless.ts `uploadEvery`), made on the Rust side before the apply */
  uploadEvery?: number;
  /** false: the par clock, the verdict and the autosave are not run on the puppet */
  dress?: boolean;
  /** the seed of the game's own die where it has none (mission.ts stands its game the same way); null: the game is left as it is */
  die?: number | null;
  /** false: the dress rows are left as they stand (default: all down, as mission.ts stands the game it compares) */
  dressDown?: boolean;
  /** how many times the start is applied through the every-tick path before the first tick, to warm it */
  warm?: number;
  /** false: the order rows on the puppet are not given the closures that ask the Rust game (ask.ts); the scripted player's orders then go by `face_give` */
  asks?: boolean;
  /** audit: every N ticks the whole puppet is handed to the Rust game whether anything was written or not (HB_AT=<ticks, comma> names the ticks; HB_DEBUG=<dir>
   *  keeps the Rust game's own dump before and after each) */
  handback?: number;
  /** fast: the scripted player's orders go through the menu's own `give` on the puppet (its rows ask the Rust game) even where the face could give them */
  menuGive?: boolean;
  /** how many steps a second game made from the start is stepped before the first tick (default 0: measured to buy nothing) */
  warmSteps?: number;
  /** megabytes asked of the module and given back before play, so that no step has to grow the memory */
  reserveMb?: number;
  /** audit: `rust` holds the write-back to the Rust bytes last applied, `puppet` to the puppet exported as the apply left it */
  baseline?: 'rust' | 'puppet';
  /** audit: every N ticks the puppet is exported again right after the apply and held to the Rust bytes */
  applyCheck?: number;
  /** a folder for writeback.txt and gives.txt */
  out?: string;
  quiet?: boolean;
  say?: (s: string) => void;
  /** as a tick opens: the puppet as the TypeScript round the last step left it, before the host's half is handed over */
  beforeHost?: (H: RustHost, cmds: readonly any[]) => void;
  /** as `face_step` returns, before the host's steps after a tick */
  afterStep?: (H: RustHost) => void;
}

// ---- the module: one instance a process, as many games as rust/sim has slots
export interface Face { X: any; mem: WebAssembly.Memory; NAMES: string[]; u8(): Uint8Array; u32(): Uint32Array; outText(): string; outView(): Uint8Array; panicText(): string; hand<T>(b: Uint8Array, f: (p: number, n: number) => T): T; partHashes(slot: number): number[] }
const FACES = new Map<string, Face>();
export const DEFAULT_WASM = new URL('../../rust/game_face/target/wasm32-unknown-unknown/release/game_face.wasm', import.meta.url).pathname;
export function loadFace(wasm = DEFAULT_WASM, fresh = false): Face {
  let F = fresh ? undefined : FACES.get(wasm); if (F) return F;
  if (!existsSync(wasm)) throw new Error(`rust-host: no module at ${wasm} (cd rust/game_face && cargo build --target wasm32-unknown-unknown --release)`);
  const X: any = new WebAssembly.Instance(new WebAssembly.Module(readFileSync(wasm)), {}).exports, mem: WebAssembly.Memory = X.memory, dec = new TextDecoder();
  const u8 = () => new Uint8Array(mem.buffer), u32 = () => new Uint32Array(mem.buffer);
  // (an address past 2 GB comes back from the module as a negative number: `>>> 0`)
  const outView = () => { const p = X.face_out_ptr() >>> 0; return u8().subarray(p, p + (X.face_out_len() >>> 0)); };
  const outText = () => dec.decode(outView());
  const NAMES: string[] = []; for (let i = 0; i < X.face_parts(); i++) { X.face_part_name(i); NAMES.push(outText()); }
  F = { X, mem, NAMES, u8, u32, outText, outView,
    panicText: () => { try { const p = X.face_panic_ptr() >>> 0; return dec.decode(u8().subarray(p, p + X.face_panic_len())); } catch { return ''; } },
    hand<T>(b: Uint8Array, f: (p: number, n: number) => T): T { const p = X.face_alloc(b.length) >>> 0; u8().set(b, p); try { return f(p, b.length); } catch (e) { const w = F.panicText(); if (w) console.error('the Rust game panicked:', w); throw e; } finally { X.face_free(p, b.length); } },
    partHashes: (slot: number) => { const at = (X.face_part_hashes(slot) >>> 0) >>> 2; return Array.from(u32().subarray(at, at + NAMES.length)); } };
  if (!fresh) FACES.set(wasm, F); return F; }

// ---- the bridge, loaded once and after the game's own modules (classes.ts enters src/game by game.ts, the one door)
let BR: any = null;
async function bridge(): Promise<any> {
  if (BR) return BR;
  const E = await import('./export.ts'), I = await import('./import.ts'), AK = await import('./ask.ts'), C = await import('./classes.ts'), P = await import('./puppet-apply.ts'), O = await import('./out-dispatch.ts'), A = await import('./audit.ts');
  const K = await import('../../src/game/knobs.ts'), D = await import('../../src/math/dice.ts'), DR = await import('../../src/game/dress.ts');
  const LJ = JSON.parse(readFileSync(new URL('./puppet-lists.json', import.meta.url).pathname, 'utf8'));
  BR = { ...E, installAsks: AK.installAsks, applyGame: I.applyGame, hooks: { classes: C.classTable(), values: C.valueTable() }, puppetRows: P.puppetRows, SLOW: P.SLOW, VALS: P.VALS, OUT: O.OUT, A, reg: K.knobRegistry(), newDice: D.newDice, dressAll: DR.dressAll,
    lists: { step: new Set<string>(LJ.step), start: new Set<string>(LJ.start) } };
  // (every walk of a game by the bridge is made with that game as the one whose rows a weak table lists: `ownKeys` below)
  for (const f of ['exportGame', 'applyGame', 'hostHalf', 'exportPart', 'hostHalfParts']) { const was = BR[f]; BR[f] = (g: any, ...a: any[]) => { const c = CUR, m = MODE; CUR = g; MODE = OFF.has('own') ? null : MODE_OF.get(g) ?? null; try { return was(g, ...a); } finally { CUR = c; MODE = m; } }; }
  ownKeys(await import('../../src/math/side.ts'));
  return BR; }

// ---- STAND-IN: A WEAK TABLE LISTS THE ROWS OF EVERY GAME OF THE PROCESS. The simulation keeps much of its state in tables beside its objects (src/math/side.ts:
// `doorway.states` keyed by a man, `opening.born`, `kitload.manLoad` …), ONE table a process, each row found only by its key, so two games never meet each
// other's rows. The bridge's export lists a table by `liveKeys()`: every key still alive, whichever game it is of. A game stood after another was so
// exported with the other game's men, routes and players as rows of its own (seats-run's second game rolled its die one time fewer at tick 787), the
// table of one thing's own row gave the first game's row (`K[0]`), and an apply that put a table's keys in the Rust game's order (import.ts `orderSide`)
// struck every other game's keys off the list for good. The other game need not have stepped, or have a host: the game server's first game, stood only
// to write its recipe, was in its second game's start (tools/qa/mpserver-check.ts: the server's words parted from its clients').
// Here: while the bridge walks a game, a table lists only THAT GAME'S keys: the keys a walk from the game reaches. A start finds them (`reach`): the
// game is exported with every table listing only the keys the walk has numbered so far (and the game itself and its views), again until an export
// numbers no key it had left out; what the book then holds is the game's. The book is put back as it was and the start's own export lists exactly those
// keys, so its rows are numbered as a process with no other game would number them. After a start: those keys, and a key first seen since (a thing made
// in play) unless a start left it out or it is in another game's book. Outside the bridge's walks `liveKeys()` is as it was.
// Belongs in the generated export.ts and import.ts: `keysOf`, the once-keyed and game-keyed reads, `orderSide` and every `liveKeys()` there should see
// the keys of the game being walked and no other.
type KeyMode = { kind: 'reach' } | { kind: 'exact' | 'own'; mine: WeakSet<object>; not?: WeakSet<object> };
let CUR: any = null, MODE: KeyMode | null = null, DROPPED: Set<object> | null = null, RAWKEY: (k: any) => any = k => k;
const MODE_OF = new WeakMap<object, KeyMode>(), OWNER = new WeakMap<object, object>();
const inBook = (bk: any, k: object): boolean => { if (!bk) return false; for (const M of bk.ids.values() as Iterable<Map<object, number>>) if (M.has(k)) return true; return bk.serial.has(k); };
const foreign = (k: any): boolean => {
  const M = MODE; if (!M || k === CUR || RAWKEY(k) === CUR) return false;
  if (M.kind === 'reach') { if (inBook(BR.bookOf(CUR), k)) return false; DROPPED?.add(k); return true; }
  if (M.mine.has(k)) return false; if (M.kind === 'exact') return true;
  if (M.not!.has(k)) return true; const o = OWNER.get(k); return o !== undefined && o !== CUR; };
const KEYED = new WeakSet<object>();
function ownKeys(SD: any): void {
  RAWKEY = SD.rawKey;
  for (const P of [SD.SideMap.prototype, SD.SideSet.prototype]) { const was = P.liveKeys; P.liveKeys = function (this: any, out: object[] = []) { const n0 = out.length, K: object[] = was.call(this, out); if (!MODE) return K; let n = n0; for (let i = n0; i < K.length; i++) if (!foreign(K[i])) K[n++] = K[i]; K.length = n; return K; }; }
  // (a list of keys written over while a game is walked keeps the other games' keys in front of it)
  const guard = () => { for (const t of SD.sideRegistry().tables as any[]) { const K = t.sideKeys; if (!K || KEYED.has(K)) continue; KEYED.add(K); let refs: WeakRef<object>[] = K.refs;
    Object.defineProperty(K, 'refs', { configurable: true, enumerable: true, get: () => refs, set: (v: WeakRef<object>[]) => { if (!MODE) { refs = v; return; } const keep = refs.filter(r => { const k = r.deref(); return k !== undefined && foreign(k); }); refs = keep.length ? keep.concat(v) : v; } }); } };
  guard(); GUARD = guard; }
let GUARD: () => void = () => {};
/** every object in a game's book is that game's, for the other games of the process (the first game to number it) */
function owned(g: any, bk: any): void { const put = (k: object) => { if (!OWNER.has(k)) OWNER.set(k, g); };
  put(g); if (!bk) return; for (const M of bk.ids.values() as Iterable<Map<object, number>>) for (const k of M.keys()) put(k); for (const k of (bk.serials as Map<object, number>).keys()) put(k); }

const now = () => performance.now();
/** a field hung on a game's object that a walk of its own keys does not meet */
const hang = (o: any, k: string, v: unknown) => { Object.defineProperty(o, k, { value: v, writable: true, configurable: true, enumerable: false }); };
/** the objects whose calls the host has already taken (a start may meet new ones: a mission started by the tool makes its own menu, fight and saves) */
const TAPPED = new WeakSet<object>();
const TAPPED_LIST = new WeakSet<object>();
/** RH_OFF=<names, comma>: a stand-in left out, to see whether a run still needs it: ends · doors · marks · last · own (a weak table lists every game's keys) */
const OFF = new Set((process.env.RH_OFF ?? '').split(','));
const enc = new TextEncoder();
type Tally = Map<string, { n: number; eg: string; ticks: number; last: number }>;

export class RustHost {
  /** ticks this host has stepped */
  k = 0;
  /** the Rust game's handle and rust/sim's slot it stands in */
  h = 0; slot = 0;
  readonly audit: boolean;
  /** the ms spent, by what */
  T = { wbExport: 0, wbCompare: 0, wbImport: 0, hostHalf: 0, step: 0, after: 0, exportStep: 0, applyStep: 0, exportWhole: 0, applyWhole: 0,  block: 0, out: 0, dress: 0, check: 0, calls: 0, start: 0 };
  worstStep = 0; worstStepAt = 0; worstApply = 0; hostBytes = 0; stepBytes = 0; wholeBytes = 0;
  /** how many whole applies, by why */
  wholes = new Map<string, number>(); steps = 0; starts = 0;
  /** the driver's calls that went to the Rust game, by name */
  calls = new Map<string, number>();
  /** … and each in the replay loop's own words (mission.ts gives.txt, hostwrites.txt), for a check to hold to the plain play's */
  gave: string[] = []; wrote: string[] = [];
  /** out events dispatched, by arm, and those whose dispatch threw on the puppet */
  outN = new Map<string, number>(); outThrew = new Map<string, string>(); outNoHand = new Map<string, number>(); outTicks = 0;
  made: Record<string, number> = {}; noWay = new Map<string, number>(); noSetter = new Map<string, number>(); noWindow = new Set<string>();
  standIns = { marks: 0 };
  uploads = { rust: 0, late: 0, missed: 0 };
  /** the whole Rust state last applied (audit), and the puppet as the last tick opened */
  lastRust: Uint8Array | null = null; openState: Uint8Array | null = null;
  // ---- the audit's books
  WB: Tally = new Map(); HOSTW: Tally = new Map(); OWNW: Tally = new Map(); WB_REFUSED: Tally = new Map(); APPLY_MISS: Tally = new Map(); CUTW = new Map<string, number>(); BLOCKW = new Map<string, number>();
  wb = { fired: 0, meter: 0, block: 0, hostOnly: 0, refused: 0 }; handbacks = 0; handbacksInBeat = 0; applyChecks = 0; applyUnequal = 0;
  private lastState: Uint8Array | null = null; private lastBlock: Uint8Array | null = null;
  private pup: any; private say: (s: string) => void;
  private hoverApplied: unknown = null; private visApplied = NaN; private wholeNow = false; private uploadDone = false;
  private missionRef: any = null; private loadsRef = 0; private stageRef: any = null; private downRef = -1; private outQ: { arm: string; e: any }[] = [];
  private asks: { after(): void } | null = null; asked = 0;
  private threw: Record<string, number> = {}; private warmDue = false;
  private dieRef: any = undefined; dieMoved = 0; private ticksFile = '';
  private memPages = 0; grew = 0; private inStep = false; private lastCmds: readonly any[] | null = null;
  /** ms spent in applies and exports so far (what a clock round a call or the dress step takes out of its own count) */
  private nest = 0;

  private constructor(readonly g: any, readonly F: Face, readonly B: any, readonly o: HostOpts) {
    this.audit = !!o.audit; this.say = o.say ?? ((s: string) => { process.stdout.write(s + '\n'); });
    const ctx: any = { p: 0 }; this.pup = { lists: B.lists, rows: B.puppetRows(ctx), ctx, slowTable: B.SLOW, vals: B.VALS };
  }
  /** the bridge and the module loaded: once, before the first `attach` */
  static async ready(wasm?: string): Promise<Face> { await bridge(); return loadFace(wasm); }
  /** before the game's first update (after `await RustHost.ready()`): from then on `game.update` steps the Rust game */
  static attach(g: any, o: HostOpts = {}): RustHost {
    if (!BR) throw new Error('rust-host: `await RustHost.ready()` first');
    // (RH_FRESH_FACE=1: a module of its own for every game, to tell what a second game of a process owes to the module's own memory from what it owes
    // to the TypeScript modules' tables)
    const B = BR, F = loadFace(o.wasm, process.env.RH_FRESH_FACE === '1'), H = new RustHost(g, F, B, o);
    if (o.reserveMb !== 0) { const n = (o.reserveMb ?? 48) * 1048576, p = F.X.face_alloc(n); F.X.face_free(p, n); }
    if (o.dressDown !== false) B.dressAll(false);
    if (o.die !== null && !g.dice) g.dice = B.newDice(o.die ?? 20942);
    // (hung as fields no walk of the game's own keys meets: rollback.ts `stateHash` walks them, and a stand's word must not change for being a puppet)
    hang(g, 'stepSeats', function (dt: number, cmds: readonly any[]) { H.step(dt, cmds); });
    hang(g, 'rustHost', H); H.taps(g);
    if (o.out) for (const f of ['writeback.txt', 'gives.txt']) writeFileSync(`${o.out}/${f}`, '');
    return H; }
  /** the Rust game is let go (its slot is free for another) and the game steps in TypeScript again */
  detach(): void { const X = this.F.X; if (this.h) { X.face_puppet_drop?.(this.h); X.face_drop(this.h); this.h = 0; } RustHost.live.delete(this); delete this.g.stepSeats; delete this.g.rustHost; }
  /** the hosts whose Rust game stands in a slot, and when each last stepped */
  private static live = new Set<RustHost>(); private static dirty = new Set<RustHost>(); private static clock = 0; private static last: RustHost | null = null; private stamp = 0; evicted = 0;
  /** this game's slot goes to another: the puppet takes the whole state, the Rust game is let go; a later step starts a new one from the puppet */
  private giveUp(): void { const X = this.F.X; if (!this.h) return;
    try { this.applyWhole('its slot went to another game'); } catch (e) { this.count(`THE WHOLE APPLY THREW as the slot was given up: ${String((e as any)?.message ?? e).slice(0, 100)}`); }
    X.face_puppet_drop?.(this.h); X.face_drop(this.h); this.h = 0; if (RustHost.last === this) RustHost.last = null; this.evicted++; if (this.rec) this.rec.evicted = this.evicted; RustHost.live.delete(this); }
  /** RH_TRACE=<file>: a line a game as the process ends (who stood it, how far it stepped, what it took), for a table of what a tool does with its games */
  rec: any = null;

  private count(name: string) { this.calls.set(name, (this.calls.get(name) ?? 0) + 1); }
  private tsBlock(): Uint8Array { const w: Uint32Array = this.g.sim.w; return new Uint8Array(w.buffer, w.byteOffset, w.byteLength); }
  rustBlock(): Uint8Array { const X = this.F.X, at = X.state_addr(this.slot) >>> 0; return this.F.u8().subarray(at, at + X.state_len()); }
  time(): number { return this.F.X.face_time(this.h); }
  dieN(): number { return this.F.X.face_die_n(this.h); }
  /** THE GAME'S OWN DIE PUT AT THE START OF `seed`'S STREAM, ON THE RUST SIDE (tools/qa/headless.ts `seedGame`, which has put the puppet's: a tool that wants a
   *  given stream after the game is stood seeds the game's die; nothing lets the Rust tick draw a tool's `Math.random`). The die is the `dice` part of the
   *  game's block (rust/sim/src/dice.rs: the count of draws as a double, then the mulberry32 word), written in place: no call of the face's is needed.
   *  Before the first tick there is no Rust game yet, and its start takes the puppet's die as it stands. */
  seedDie(seed: number): void { this.dieSeeds++; if (this.rec) this.rec.dieSeeds = this.dieSeeds; if (!this.h) return;
    const X = this.F.X, i = this.F.NAMES.indexOf('dice'); if (i < 0) throw new Error('rust-host: the block has no part named dice');
    const put = (buf: ArrayBufferLike, at: number) => { const v = new DataView(buf); v.setFloat64(at, 0, true); v.setUint32(at + 8, seed >>> 0, true); };
    put(this.F.mem.buffer, X.face_part_addr(this.slot, i) >>> 0);
    const w: Uint32Array = this.g.sim.w; put(w.buffer, w.byteOffset + (X.face_part_addr(0, i) >>> 0) - (X.state_addr(0) >>> 0));   // (the puppet's copy of the block, as the next apply would leave it)
    this.dieRef = this.g.dice?.n; }
  dieSeeds = 0;
  /** the clock at this game's first step and at its last one's opening (RH_TIMES: what a tick costs all in, the tool's own code between two steps included) */
  wall0 = 0; wallLast = 0;
  hashes(): number[] { return this.F.partHashes(this.slot); }

  // ---- a start: the puppet as it stands is the game. The first tick's; and again wherever the TypeScript round a step has put another game in the
  // puppet's place (a checkpoint put back, a restart): no stepper can follow that from its own state.
  private start(): void {
    const g = this.g, X = this.F.X, B = this.B, t = now();
    this.taps(g);   // (the tool may have started another mission since the game was stood: its menu, its fight and its saves are new objects)
    // (ANOTHER GAME OF THIS PROCESS STEPPED LAST: the modules' own tables and variables, which every game of a process shares and a start reads, hold what
    // the filtered apply left of it. Its puppet takes the whole state first, so that this start reads what the TypeScript tick would have left there.)
    { const L = RustHost.last; if (L && L !== this && L.h && !L.audit && !OFF.has('last')) try { L.applyWhole('another game of the process starts'); } catch (e) { L.count(`THE WHOLE APPLY THREW as another game started: ${String((e as any)?.message ?? e).slice(0, 100)}`); } }
    g.sim.settle(g);
    const start = this.reach();
    const block = this.tsBlock().slice();
    if (this.h) { X.face_puppet_drop?.(this.h); X.face_drop(this.h); }
    const make = () => this.F.hand(start, (p, n) => this.F.hand(block, (bp, bn) => X.face_new(p, n, bp, bn)));
    this.h = make();
    // THE SLOTS (rust/sim has four): a tool that stands games back to back in one process (the probe runner: hundreds) never says it has done with one.
    // Where every slot is taken, the game stepped longest ago gives its slot up: its puppet takes the whole state first, so that if the tool does step
    // it again it starts anew from a puppet that holds all of it (the same path as a checkpoint put back).
    while (!this.h && RustHost.live.size) { let old: RustHost | null = null; for (const o of RustHost.live) if (o !== this && (!old || o.stamp < old.stamp)) old = o; if (!old) break; old.giveUp(); this.h = make(); }
    if (!this.h) throw new Error('face_new: no free slot');
    RustHost.live.add(this); this.stamp = ++RustHost.clock;
    this.slot = X.face_slot(this.h); { const said = this.F.outText(); if (said && !this.o.quiet) this.say(said); }
    if (X.state_len() !== block.length) this.say(`rust-host: rust/sim's block is ${X.state_len()} bytes in the module and ${block.length} in the TypeScript game`);
    this.lastState = start; this.lastBlock = block; this.lastRust = start; this.starts++; if (this.rec) this.rec.starts = this.starts; this.warmDue = this.starts === 1;
    // THE MENU'S QUESTIONS: every order row and the squad's posture ask the Rust game by the row's id (ask.ts, `face_ask`); a row's `issue` gives the order
    // on the Rust side and the puppet takes the state again
    if (this.starts === 1 && this.o.asks !== false && X.face_ask) this.asks = B.installAsks(g, B.reg, (kind: number, row: number, seat: number, b: Uint8Array) => { this.asked++; this.F.hand(b, (p, n) => X.face_ask(this.h, kind, row, seat, p, n)); return this.F.outView().slice(); }, () => { this.reapply('an order row issued'); });
    // THE MEMO OF THE FILTERED EXPORT is filled by a start's export. Its bytes are not applied: the puppet IS that state (the Rust game was made from it).
    // The every-tick path is then run over every row a few times, so that the first ticks do not pay for its compiling: `face_puppet_warm` sends each
    // row as if it had changed, and applying a row that holds what the puppet holds changes nothing.
    X.face_puppet(this.h, 1);
    const warm = this.o.warm ?? 4; if (X.face_puppet_warm) for (let i = 0; i < warm; i++) { X.face_puppet_warm(this.h); const R = B.applyGame(g, this.F.outView(), B.reg, { hooks: B.hooks, puppet: { ...this.pup, start: false } }); this.after(R); }
    this.visApplied = g.player?.visibility; this.hoverApplied = g.hover; this.missionRef = g.mission; this.loadsRef = g.chronicle?.counts?.loads ?? 0; this.stageRef = this.endMark(); this.downRef = this.downCount();
    this.memPages = this.F.mem.buffer.byteLength; this.T.start += now() - t;
    if (!this.o.quiet) this.say(`rust-host: ${this.starts === 1 ? 'the start' : `A NEW START as tick ${this.k} opens (the TypeScript round the last step put another game in the puppet's place)`}: ${start.length} bytes, the block ${block.length}; slot ${this.slot}; memory ${(this.F.mem.buffer.byteLength / 1048576).toFixed(0)} MB`);
  }
  /** A START'S EXPORT, with every weak table listing this game's keys and no other game's (the stand-in `ownKeys`): the keys a walk from the game
   *  reaches are found on a book of their own, the game's book is put back as it was, and the export is made listing exactly those */
  private reach(): Uint8Array {
    const g = this.g, B = this.B; GUARD();
    for (const o of RustHost.dirty) if (o !== this) owned(o.g, B.bookOf(o.g)); RustHost.dirty.clear();
    if (OFF.has('own')) return B.exportGame(g, B.reg);
    const FIELDS = ['ids', 'lists', 'first', 'loose', 'serials', 'serial', 'nextSerial'] as const;
    const fresh = (bk: any) => { bk.ids = new Map(); bk.lists = new Map(); bk.first = new Map(); bk.loose = new Map(); bk.serials = new Map(); bk.serial = new WeakMap(); bk.nextSerial = 1; };
    const bk0 = B.bookOf(g), saved: any = bk0 ? Object.fromEntries(FIELDS.map(f => [f, bk0[f]])) : null; if (bk0) fresh(bk0);
    MODE_OF.set(g, { kind: 'reach' });
    let out: Uint8Array, left = new Set<object>(), passes = 0, found = 0;
    for (;;) { DROPPED = new Set(); out = B.exportGame(g, B.reg); left = DROPPED; DROPPED = null; passes++; const bk = B.bookOf(g); let again = 0; for (const k of left) if (inBook(bk, k)) again++; found += again; if (!again) break;
      if (passes >= 8) { this.count(`A START'S REACH DID NOT SETTLE in 8 exports (${again} keys still found late)`); break; } }
    const bk = B.bookOf(g), mine = new WeakSet<object>(), not = new WeakSet<object>(); let nNot = 0;
    for (const M of bk.ids.values() as Iterable<Map<object, number>>) for (const k of M.keys()) mine.add(k); for (const k of (bk.serials as Map<object, number>).keys()) mine.add(k);
    for (const k of left) if (!mine.has(k)) { not.add(k); nNot++; }
    // (the book as it was: a start after the first keeps every number it has given; the first start whose one export left out no key it then met IS the export)
    if (saved) Object.assign(bk, saved); else if (passes > 1) fresh(bk);
    if (saved || passes > 1) { MODE_OF.set(g, { kind: 'exact', mine }); out = B.exportGame(g, B.reg); }
    MODE_OF.set(g, { kind: 'own', mine, not }); owned(g, B.bookOf(g));
    if (nNot || passes > 1) this.count(`a start's export: ${nNot} keys of other games' rows left out of the weak tables; ${passes} exports to find this game's own (${found} keys met late)`);
    return out; }
  /** what the scripted player's `ended()` reads (playthrough.ts:733), the mission's stage, and how many door orders are under way: when one ends the
   *  scripted commander asks the door tools what the team knows of that door (playthrough.ts:998 `knownShut` → doortools.ts:1019 `knows`, the tools'
   *  `tried`, which is not on the filtered list: warehouse-four's and barracks-four's second `bangClear` was not given without it) */
  private endMark(): string { const g = this.g; return `${g.mission?.stage}|${g.mission?.debrief}|${g.roomClear?.end}|${g.rescue?.result?.state}|${g.snatch?.end}|${OFF.has('doors') ? 0 : g.doorTools?.ops?.length}`; }
  private downCount(): number { const g = this.g; let n = g.player?.down ? 1 : 0; for (const x of g.guards as any[]) if (x.state === 'dead' || x.down) n++; return n; }

  // ---- the applies
  private took(R: any): void {
    for (const [q, n] of R.noWay) this.noWay.set(q, (this.noWay.get(q) ?? 0) + n); for (const [q, n] of R.noSetter) this.noSetter.set(q, (this.noSetter.get(q) ?? 0) + n); for (const q of R.noWindow) this.noWindow.add(q);
    for (const [c, n] of Object.entries<number>(R.made)) if (n) this.made[c] = (this.made[c] ?? 0) + n; }
  /** after every apply: the rows and the posture the apply made get the closures that ask the Rust game (ask.ts) */
  private after(R: any): void { this.asks?.after(); this.took(R); this.visApplied = this.g.player?.visibility; this.hoverApplied = this.g.hover; this.dieRef = this.g.dice?.n; }
  private copyBlock(): void { const t = now(), mine = this.tsBlock(), theirs = this.rustBlock(); mine.set(theirs.subarray(0, mine.length)); if (this.audit) this.lastBlock = mine.slice(); this.T.block += now() - t; }
  /** THE FILTERED APPLY: the listed fields of what changed since the last one, and the step's out events */
  applyStep(): void {
    const g = this.g, X = this.F.X, B = this.B, t0 = now(); let t = t0;
    X.face_puppet(this.h, 0); const blob = this.F.outView(); this.stepBytes += blob.length; this.T.exportStep += now() - t;
    t = now(); const R = B.applyGame(g, blob, B.reg, { hooks: B.hooks, puppet: { ...this.pup, start: false } }); const el = now() - t; this.T.applyStep += el; if (el > this.worstApply) this.worstApply = el;
    if (R.out?.length) for (const e of R.out) this.outQ.push(e);
    this.after(R); this.copyBlock(); this.steps++; this.nest += now() - t0;
  }
  /** THE WHOLE APPLY: every field the bridge carries (a field the Rust game owns and TypeScript cannot hold, the box mirror among them, is kept beside the
   *  object as an echo: tools/blitz/IMPORT.md) */
  applyWhole(why: string): Uint8Array {
    const g = this.g, X = this.F.X, B = this.B, t0 = now(); let t = t0;
    // (BETWEEN TWO TICKS the host has moved its own camera and Input on since the Rust game was last handed them (headless.ts `cam.update` after
    // `game.update`); the whole state would put them back a frame. depot-four parted so at tick 1578, a frame after a room was flooded. The host's rows
    // are handed over first, as a tick's opening hands them.)
    if (!this.inStep && this.lastCmds) { const hb: Uint8Array = B.hostHalf(g, B.reg, this.lastCmds); this.F.hand(hb, (p, n) => X.face_host_half(this.h, p, n)); }
    // (a copy out of the module: a whole apply keeps the bytes of the fields the Rust game owns beside their objects, as places in this very blob (the
    // bridge's echo), and the module's memory is written again by the next call)
    X.face_export(this.h); const blob = this.F.outView().slice(); this.wholeBytes = blob.length; this.T.exportWhole += now() - t;
    t = now(); const R = B.applyGame(g, blob, B.reg, { hooks: B.hooks }); const el = now() - t; this.T.applyWhole += el; if (el > this.worstApply) this.worstApply = el;
    this.after(R); this.copyBlock();
    this.wholes.set(why, (this.wholes.get(why) ?? 0) + 1); this.wholeNow = true; if (this.audit) this.lastRust = blob; this.nest += now() - t0;
    return blob; }
  /** after a call that changed the Rust game between two ticks: the puppet takes the state again */
  private reapply(why: string): void { if (this.audit) { this.applyStep(); this.lastState = this.applyWhole(why); } else this.applyStep(); }

  // ---- the taps: what the TypeScript round a step calls that WRITES the game and is no input
  private taps(g: any): void {
    const H = this, X = this.F.X, B = this.B, F = this.F;
    // THE SCRIPTED PLAYER'S ORDERS. With the menu's questions installed (ask.ts) `orders.give` runs as it is on the puppet: every closure of a row it calls
    // asks the Rust game, and the row's `issue` gives the order there and has the puppet take the state again. The call is written down in the replay
    // loop's own words for a check to hold to the plain play's. Without them (`asks: false`) the order is given on the Rust side at once by `face_give`
    // where the face has words for its target, and runs on the puppet where it has none.
    const O = g.orders, give = O.give; if (O && !TAPPED.has(O)) { TAPPED.add(O); hang(O, 'give', function (this: any, id: string, t: any, who: any, opts: any) {
      if (!H.h) return give.call(this, id, t, who, opts);
      const T0 = t ?? this.targetAt(g.aimPoint), w = who === undefined ? '-' : typeof who === 'object' ? `g${g.guards.indexOf(who)}` : String(who);
      const tail = T0.kind === 'door' ? ` door ${g.doors.list.indexOf(T0.door)} ${T0.side} book${B.bookOf(g).ids.get('DoorId')?.get(T0.door)}` : T0.kind === 'none' && T0.man ? ` man ${g.guards.indexOf(T0.man)}` : T0.kind === 'foe' ? ` man ${g.guards.indexOf(T0.foe)}` : (T0.wall || T0.thing) ? ' UNSUPPORTED-wall-or-thing' : '';
      const line = `${H.k + 1} ${id} ${w} ${T0.kind} ${T0.at.join(' ')}${tail}${opts ? ' UNSUPPORTED-opts' : ''}`, noWords = line.includes('UNSUPPORTED') || /^g\d/.test(w);
      H.gave.push(line); const t0 = now(), n0 = H.nest; H.handWrites(H.k + 1);
      // STAND-IN (the fast way): THE MENU'S OWN `give` READS THE PUPPET before it asks a row anything (orders.ts:2285 prices the walk by the floor the team
      // has seen, `doorTools.seen`, floorseen::FloorSeen.at / .asked / .frame, none of it on the filtered list): on room-clear-four it refused `stack` at
      // tick 559 with "server door is locked." where the Rust game's own give takes it. So where the face has words for the order's target the fast way
      // gives it on the Rust side (`face_give`), and the menu's own call is for the audit, for a target the face has no words for, and for `menuGive`.
      // Belongs on the generated list (what orders.ts `give` and its helpers read).
      const menu = !!H.asks && (H.audit || noWords || !!H.o.menuGive);
      if (menu || noWords) { H.count(H.asks ? `orders.give(${id}) (the menu's own call; its rows ask the Rust game)` : `orders.give(${id}) ON THE PUPPET (the face has no words for its target${H.audit ? '' : ': NOT HANDED to the Rust game'})`);
        const r = give.call(this, id, t, who, opts); H.T.calls += now() - t0 - (H.nest - n0); if (H.o.out) appendFileSync(`${H.o.out}/gives.txt`, `${line}\n   -> ${JSON.stringify(r)}\n`); return r; }
      H.count(`orders.give(${id}) by face_give`);
      F.hand(enc.encode(line), (p, n) => X.face_give(H.h, H.k + 1, p, n)); const said = F.outText(), m = said.match(/-> (None|Some\("((?:[^"\\]|\\.)*)"\))\s*$/);
      if (H.o.out) appendFileSync(`${H.o.out}/gives.txt`, `${line}\n   ${said.trim()}\n`);
      if (!m) H.say(`rust-host: the face's answer to an order was not understood: ${said}`);
      H.reapply('an order'); H.T.calls += now() - t0 - (H.nest - n0);
      return m && m[1] !== 'None' ? JSON.parse(`"${m[2]}"`) : null; }); }
    // THE DRIVER'S QUESTION THAT FLOODS A ROOM AND KEEPS IT (`fight.roomAt(p)` between two ticks). Fast: the Rust game is asked first; where that flooded a
    // room the puppet takes the WHOLE state (a room's rows are not on the filtered list), so the TypeScript's own ask then finds the room and floods none.
    // Audit: the TypeScript floods, and the write-back hands the rooms over.
    if (g.fight?.roomAt && !TAPPED.has(g.fight)) { const Fi = g.fight, ask = Fi.roomAt; TAPPED.add(Fi); hang(Fi, 'roomAt', function (this: any, p: any) {
      if (H.h && !H.audit && X.face_room_at) { const t0 = now(), n0 = H.nest; if (X.face_room_at(H.h, p[0], p[1], p[2])) { H.wrote.push(`${H.k + 1} room_at ${p[0]} ${p[1]} ${p[2]}`); H.count('fight.roomAt (the Rust game flooded a new room; the puppet took the whole state)'); H.applyWhole('a room flooded'); } H.T.calls += now() - t0 - (H.nest - n0); }
      const n0 = Fi.rooms.length, r = ask.call(this, p); if (Fi.rooms.length !== n0) { if (H.audit) H.wrote.push(`${H.k + 1} room_at ${p[0]} ${p[1]} ${p[2]}`); H.count(H.audit ? 'fight.roomAt (flooded a new room on the puppet)' : 'fight.roomAt FLOODED A ROOM ON THE PUPPET THAT THE RUST GAME DID NOT'); } return r; }); }
    // (a lesson's player picks a row of the nearby list between ticks and reads the answer at once: made on the Rust side, as an order is)
    const wordAt = (ty: string, w: unknown) => w === undefined ? -1 : B.SCHEMA.types[ty].words.indexOf(w);
    const pick = g.selectVerb; if (!TAPPED.has(g)) { TAPPED.add(g); hang(g, 'selectVerb', function (this: any, kind: string, verb?: string) {
      if (!H.h) return pick.call(this, kind, verb);
      H.count('game.selectVerb'); const t0 = now(), n0 = H.nest; H.handWrites(H.k + 1);
      const line = `${H.k + 1} select_verb ${wordAt('crate::game::InteractableKind', kind)} ${wordAt('crate::game::Verb', verb)} ${kind} ${verb}`; H.wrote.push(line);
      const r = F.hand(enc.encode(line), (p, n) => X.face_hostwrite(H.h, H.k + 1, p, n)), said = F.outText();
      if (r !== 1) H.say(`rust-host: the face did not take selectVerb(${kind}, ${verb})`);
      H.reapply('a pick of a verb'); H.T.calls += now() - t0 - (H.nest - n0); return / -> true\s*$/.test(said); }); }
    // (the scripted player's word to the hostages, or to the man in hand, is the module's own call between two ticks: made on the Rust side, as a pick is.
    //  The puppet's own body does not run: what it would write the apply brings.)
    for (const [name, E] of [['rescue_word', g.rescue?.ending], ['snatch_word', g.snatch?.out]] as [string, any][]) if (E?.word && !TAPPED.has(E)) { const word = E.word; TAPPED.add(E); hang(E, 'word', function (this: any, by: any) {
      if (!H.h || g.ticking) return word.call(this, by);
      const who = by === g.player.char ? 'player' : by ? null : 'none'; if (who === null) { H.say(`rust-host: ${name} for a man the face has no word for; run on the puppet`); return word.call(this, by); }
      H.count(`${name} (the module's own word, made on the Rust side)`); const t0 = now(), n0 = H.nest; H.handWrites(H.k + 1);
      const line = `${H.k + 1} ${name} ${who}`; H.wrote.push(line);
      const r = F.hand(enc.encode(line), (p, n) => X.face_hostwrite(H.h, H.k + 1, p, n)), said = F.outText();
      if (r !== 1) H.say(`rust-host: the face did not take ${name}(${who})`);
      H.reapply('a word to follow'); H.T.calls += now() - t0 - (H.nest - n0); return / -> true\s*$/.test(said); }); }
    // THE HOST'S STEPS ROUND A TICK THAT THE RUST SIDE MAKES TOO.
    // The lights' update: made on the Rust side after every step. Audit, and a fast tick that took the whole state: the puppet already holds what it
    // left, and run again in TypeScript a flash's envelope would be stepped twice. A fast tick that took only the filtered state: no lamp is on that list,
    // so the puppet's own lamps are stepped by the TypeScript's own update, as the Rust game's are by its own.
    const E = g.engine, L = E.lights, W = E.world, lightsUpdate = L.update;
    if (!TAPPED.has(L)) { TAPPED.add(L); hang(L, 'update', function (this: any, ...a: any[]) { if (!H.h || (!H.audit && !H.wholeNow)) return lightsUpdate.apply(this, a); H.count('engine.lights.update (not run on the puppet: it holds what the Rust side\'s left)'); }); }
    // The box world's listing. Audit: made on the Rust side before the apply on the ticks the headless host lists (`uploadEvery`), and the puppet holds the
    // listed world; a listing asked for on another tick is made then and the puppet takes the state again. Fast: the list of every box and the ray grid
    // are not on the filtered list (they are the TypeScript's own there), so the TypeScript's listing runs as it is, and the Rust game lists its own
    // at the same moment.
    const upload = W.upload; this.tsUpload = () => { this.tsOnly = true; try { W.upload(); } finally { this.tsOnly = false; } };
    if (!TAPPED.has(W)) { TAPPED.add(W); hang(W, 'upload', function (this: any, ...a: any[]) {
      if (!H.h) return upload.apply(this, a);
      if (!H.audit) { if (!H.tsOnly) { X.face_upload(H.h); H.uploads.rust++; } if (H.quietOn) return;
        // STAND-IN: A BOX WHOSE MARKINGS WENT is `marks: null` after an apply (the importer writes a None as null), and the listing tests
        // `b.marks !== undefined` (scene/boxes.ts:841) and then reads its length: it threw at tick 1817 of tut-solo. Belongs in import.ts: the None of a
        // field the TypeScript declares optional is `undefined`.
        for (const L of [this.statics, this.dynamics, g.dynBoxes] as any[][]) if (!OFF.has('marks')) for (const b of L) if (b.marks === null) { b.marks = undefined; H.standIns.marks++; }
        return upload.apply(this, a); }
      if (H.uploadDone) { H.uploadDone = false; return; }
      H.uploads.late++; X.face_upload(H.h); H.uploads.rust++; H.applyStep(); H.applyWhole('a listing off its tick'); H.lastState = B.exportGame(g, B.reg); }); }
    // THE LIST ALONE (`relist`: a tick that is not a draw; src/net/step.ts makes one after every tick of a networked game, where the headless world's own
    // step calls `upload`). The Rust game lists its world at the same moment here too. Without it the Rust game never listed: its state grew by the
    // frame's boxes every tick (35 KB a tick on the game server's room clear) and the every-tick export, which walks them, slowed tick by tick (9 ms a
    // tick over the first hundred ticks, 41 over the sixth).
    const relist = W.relist;
    if (typeof relist === 'function' && !TAPPED_LIST.has(W)) { TAPPED_LIST.add(W); hang(W, 'relist', function (this: any, ...a: any[]) {
      if (H.h && !H.audit && !H.tsOnly && !OFF.has('relist')) { X.face_upload(H.h); H.uploads.rust++; }
      return relist.apply(this, a); }); }
    // THE SAVE CONTROLLER KEEPS THE PUPPET (a save in memory is the TypeScript objects as they stand; a file is their serialisation), so a save taken off
    // a puppet that holds only the filtered state would keep fields that stopped moving ticks ago. Before a take the puppet takes the whole state.
    const S = g.saves; if (S && !TAPPED.has(S)) { TAPPED.add(S); for (const name of ['write', 'writeMem'] as const) { const was = S[name]; if (typeof was !== 'function') continue;
      hang(S, name, function (this: any, ...a: any[]) { if (H.h && !H.audit && !H.wholeNow) H.applyWhole('a save taken'); return was.apply(this, a); }); } }
  }

  // ---- what TypeScript wrote into the puppet since the last apply goes to the Rust game. `at`: the tick that opens next.
  /** fast: the writes that have a call. The lesson's player feeds the light meter before every tick (lesson-run.ts:228) */
  private handWrites(at: number): void {
    if (this.audit) { this.writeBack(at); return; }
    const g = this.g, v = g.player?.visibility;
    if (v !== this.visApplied && typeof v === 'number') { this.F.X.face_meter(this.h, v); this.visApplied = v; this.wb.meter++; }
    // (the scripted player put the highlight on a row itself: it writes `player.nearSel` and calls pickHover, which hands `g.hover` a row of the
    //  puppet's own list. The highlight's number is not on the filtered list, so the pick is known by the hover: another object than the apply left,
    //  and one of the list's rows. The row's number goes to the Rust game, whose own pickHover takes it.)
    const hv = g.hover, row = hv && hv !== this.hoverApplied ? (g.nearby ?? []).indexOf(hv) : -1;
    if (row >= 0) { const line = `${at} near_sel ${row}`; this.wrote.push(line); if (this.F.hand(enc.encode(line), (p, n) => this.F.X.face_hostwrite(this.h, at, p, n)) !== 1) this.say(`rust-host: the face did not take the highlight on row ${row}`); this.count('the highlight put on a row by the tool (made on the Rust side)'); }
    this.hoverApplied = hv;
  }
  /** audit: the whole puppet held to the bytes last applied */
  private writeBack(at: number): void {
    const g = this.g, X = this.F.X, B = this.B, A = B.A, t = now();
    const mine: Uint8Array = B.exportGame(g, B.reg); let t1 = now(); this.T.wbExport += t1 - t;
    const owed: { q: string; path: string }[] = [], last = this.lastState!;
    if (mine.length !== last.length || !A.buf(mine).equals(A.buf(last))) { const D = A.fieldsDiffering(last, mine); for (const x of D) { if (A.HOST_Q(x)) A.tallyAdd(this.HOSTW, x.q, x.path, at); else if (A.OWN_Q(x)) A.tallyAdd(this.OWNW, x.q, x.path, at); else owed.push(x); } }
    const blk = this.tsBlock(), blockOwed = !A.buf(blk).equals(A.buf(this.lastBlock!));
    this.T.wbCompare += now() - t1; t1 = now();
    if (owed.length) {
      for (const x of owed) A.tallyAdd(this.WB, x.q, x.path, at);
      // (the one write the lesson's player makes every tick, his own light meter, has a small call of its own in the face)
      if (owed.length === 1 && owed[0].path === 'g.pl[0].visibility' && g.seats.length === 1) { X.face_meter(this.h, g.player.visibility); this.wb.meter++; }
      else { this.F.hand(A.withCutsOf(mine, this.lastRust!), (p, n) => X.face_import_into(this.h, p, n)); this.wb.fired++; if (this.o.out && this.wb.fired <= 400) appendFileSync(`${this.o.out}/writeback.txt`, `${at} ${[...new Set(owed.map(x => x.q))].join(' ')}\n   eg ${owed[0].path}\n`); }
    } else this.wb.hostOnly++;
    // (a whole hand-over whether anything was written or not: what a host must be able to do at any tick, a beat of the level's script running or not)
    { const N = this.o.handback ?? 0; if (N && (process.env.HB_AT ? process.env.HB_AT.split(',').includes(String(at)) : at % N === 0)) { const dump = () => { X.face_debug(this.h); return this.F.outText(); }, D = process.env.HB_DEBUG, before = D ? dump() : '';
      this.F.hand(mine, (p, n) => X.face_import_into(this.h, p, n)); if (D) { writeFileSync(`${D}/before${at}.txt`, before); writeFileSync(`${D}/after${at}.txt`, dump()); }
      this.handbacks++; if ((B.bookOf(g).listOf('DirectorLiveBeatId') as any[]).some(r => r.state === 'running')) this.handbacksInBeat++; } }
    if (blockOwed) { this.wb.block++; const a = this.lastBlock!; for (let i = 0; i < this.F.NAMES.length; i++) { const off = X.face_part_addr(0, i) - X.state_addr(0), len = X.face_part_len(i); if (!A.buf(blk.subarray(off, off + len)).equals(A.buf(a.subarray(off, off + len)))) this.BLOCKW.set(this.F.NAMES[i], (this.BLOCKW.get(this.F.NAMES[i]) ?? 0) + 1); } this.rustBlock().set(blk.subarray(0, X.state_len())); this.lastBlock = blk.slice(); }
    this.T.wbImport += now() - t1; this.openState = mine; this.lastState = mine;
  }

  // ---- the out events of a step, on the puppet's host objects, in the order the rules made them
  private dispatch(): void {
    const Q = this.outQ; if (!Q.length) return; const t = now(), g = this.g, OUT = this.B.OUT; this.outTicks++;
    for (const { arm, e } of Q) { this.outN.set(arm, (this.outN.get(arm) ?? 0) + 1);
      // (the harness's stand-ins have not every method a page's host has: `audio.rain`, `smoke.obstaclesChanged`. The generated dispatcher calls them
      // unguarded; such an arm is counted and not called again)
      const no = this.outNoHand.get(arm); if (no !== undefined) { this.outNoHand.set(arm, no + 1); continue; }
      try { OUT[arm](g, e); } catch (x) { const w = String((x as any)?.message ?? x); if (/is not a function/.test(w)) this.outNoHand.set(arm, 1); else if (!this.outThrew.has(arm)) { this.outThrew.set(arm, `tick ${this.k}: ${w.slice(0, 160)}`);
        // (RH_OUT_SAY=1: the first throw of an arm with its event and where it threw)
        if (process.env.RH_OUT_SAY) { const flat = (v: any): any => v && typeof v === 'object' ? (ArrayBuffer.isView(v) || Array.isArray(v) ? Array.from(v as any).slice(0, 4) : `${v.constructor?.name}{${Object.keys(v).slice(0, 12).join(',')}}`) : v; process.stderr.write(`out event ${arm} threw at tick ${this.k}: ${w}\n  ${JSON.stringify(Object.fromEntries(Object.entries(e ?? {}).map(([k, v]) => [k, flat(v)])))}\n  ${String((x as any)?.stack ?? '').split('\n').slice(0, 5).join('\n  ')}\n`); } } } }
    Q.length = 0; this.T.out += now() - t; }

  // ---- THE STEP, in place of `stepSeats`
  step(dt: number, cmds: readonly any[]): void {
    const g = this.g, X = this.F.X, B = this.B, k = ++this.k; let t = now();
    if (!this.wall0) this.wall0 = t; this.wallLast = t;
    this.wholeNow = false; this.inStep = true; this.lastCmds = cmds; this.stamp = ++RustHost.clock; RustHost.last = this; RustHost.dirty.add(this); if (this.rec) { this.rec.k = k; this.rec.quiet = this.quietSteps; if (SITES) site(this.rec); } if (CAP_AT && (k & 31) === 0 && Date.now() > CAP_AT) process.exit(143);
    // (THE GAME'S DIE ROLLED BETWEEN TWO TICKS: a rule of the tick ran in TypeScript on the puppet, called by the tool. Counted: such a tool leans on code
    // the Rust game has in its place)
    if (FOLDED && this.rec) folded(this.rec);
    if (this.h && g.dice && g.dice.n !== this.dieRef) { this.dieMoved++; if (this.rec) this.rec.dieMoved = this.dieMoved; }
    if (!this.h || g.mission !== this.missionRef || (g.chronicle?.counts?.loads ?? 0) !== this.loadsRef) this.start(); else this.handWrites(k);
    this.o.beforeHost?.(this, cmds);
    // THE HOST'S HALF: each seat's record, every camera and Input row, in the bridge's own small buffer (export.ts hostHalf, io::take_host_half)
    t = now(); const hb: Uint8Array = B.hostHalf(g, B.reg, cmds); this.hostBytes = hb.length; this.F.hand(hb, (p, n) => X.face_host_half(this.h, p, n)); this.T.hostHalf += now() - t;
    // THE STEP'S OWN WARM-UP (`warmSteps`, off by default): a second game made from the same start, stepped with this tick's record and let go before
    // the game's own first step. MEASURED TO BUY NOTHING: the first tick is the worst of a play (about 20 ms on a quiet machine) with three such steps
    // before it as without (19.8 against 19.5 ms, night-breach:1), and no step grows the memory once room is reserved. So the first tick's cost is the
    // game's own (what a game builds the first time it is asked), not the module's compiling.
    if (this.warmDue) { this.warmDue = false; const t1 = now(), n = this.o.warmSteps ?? 0, h2 = n > 0 && this.lastState && this.lastBlock ? this.F.hand(this.lastState, (p, m) => this.F.hand(this.lastBlock!, (bp, bn) => X.face_new(p, m, bp, bn))) : 0;
      if (h2) { for (let i = 0; i < n; i++) { this.F.hand(hb, (p, m) => X.face_host_half(h2, p, m)); X.face_step(h2, dt, 0); X.face_lights(h2, dt); X.face_puppet(h2, i === 0 ? 1 : 0); } X.face_puppet_drop?.(h2); X.face_drop(h2); }
      this.T.start += now() - t1; }
    this.memPages = this.F.mem.buffer.byteLength;
    t = now(); X.face_step(this.h, dt, 0); const el = now() - t; this.T.step += el; if (el > this.worstStep) { this.worstStep = el; this.worstStepAt = k; }
    if (this.F.mem.buffer.byteLength !== this.memPages) this.grew++;
    if (!this.o.quiet) { const said = this.F.outText(); if (said) this.say(said.trimEnd()); }
    this.o.afterStep?.(this);
    // (RH_HALF=<ticks, comma>: the host's half of those ticks, a line a field's value, for two runs to be held to each other by `diff`)
    if (process.env.RH_HALF?.split(',').includes(String(k))) { const A = B.A; for (const [ty, v] of B.hostHalfParts(g, cmds) as [string, any][]) { if (ty === 'u32') continue; const b: Uint8Array = B.exportPart(g, B.reg, [[ty, v]]); const offs: number[] = []; for (let i = 0; i < b.length; i += 8) offs.push(i);
      for (const x of A.locate(b, offs, null, true, ty, ty.split('::').pop())) { let f = NaN; try { f = new DataView(b.buffer, b.byteOffset + x.off, 8).getFloat64(0, true); } catch { /* the last bytes */ } process.stderr.write(`half ${k} ${x.path} ${f}\n`); } } }
    if (TICKS_DIR) { if (k === 1 || this.ticksFile === '') { this.ticksFile = ticksFile(this.audit ? 'audit' : 'rust'); writeFileSync(this.ticksFile, ''); } tickLine(this.F, this.ticksFile, k, this.dieN(), this.rustBlock(), hb, this.gave.map(l => { const w = l.split(' '); return `${w[0]}:${w[1]}`; }).join(',')); }
    // ---- the host's steps after a tick, on the Rust side: the lights' update, the host's view of a level on rails, the camera's ask
    t = now(); X.face_lights(this.h, dt); X.face_view(this.h); X.face_follow?.(this.h);
    if (this.audit) { const U = this.o.uploadEvery ?? 4; if (k % U === 1 % U) { X.face_upload(this.h); this.uploads.rust++; if (this.uploadDone) this.uploads.missed++; this.uploadDone = true; } else if (this.uploadDone) { this.uploads.missed++; this.uploadDone = false; } }
    this.T.after += now() - t;
    // ---- A QUIET STEP (`quietBegin`): the tool that asked for this run of steps has said none of its code runs between them, so nothing of the state
    // goes to the puppet until the run ends. The host's own camera needs two things of the game after every tick: asked for by `face_look`.
    this.lastDt = dt;
    if (this.quietOn) { const at = (X.face_look(this.h) >>> 0) >>> 3, L = new Float64Array(this.F.mem.buffer).subarray(at, at + 6); this.lookNow = { follow: [L[0], L[1], L[2]], aim: [L[3], L[4], L[5]] };
      this.quietOwed++; this.quietSteps++; this.inStep = false; return; }
    const rust = this.tail(dt, k);
    this.loadsRef = g.chronicle?.counts?.loads ?? 0; this.missionRef = g.mission; this.inStep = false; this.dieRef = g.dice?.n;
    this.auditAfter(rust, k);
  }
  /** after a step, or a quiet run of them: the apply, the out events, the dress step. Returns the whole Rust state where the audit took one */
  private tail(dt: number, k: number): Uint8Array | null {
    const g = this.g, X = this.F.X; let t = now();
    // ---- the apply
    this.applyStep();
    let rust: Uint8Array | null = null;
    if (this.audit) rust = this.applyWhole('audit');
    else { const why = this.wholeOwed(k); if (why) this.applyWhole(why); }
    this.dispatch();
    // ---- the dress step: what the tick does that has no Rust body and a reader of the puppet needs
    // (THE AUTOSAVE is the host's and decides off the frame as it now stands, so it runs here, on the puppet. Its one write into the game is the
    // chronicle's row, and on a level on rails it stops the level's program for the take: both are made on the Rust side by `face_checkpoint`, as the
    // replay loop makes them, and the puppet takes the state again.)
    t = now(); const nest0 = this.nest;
    if (this.o.dress !== false) {
      // (the par clock and the verdict are the debrief's: a throw of theirs on the puppet is counted and is not the play's end)
      for (const n of ['par', 'verdict'] as const) try { g[n]?.tick(); } catch (e) { this.count(`${n}.tick THREW on the puppet (first at tick ${this.threw[n] ??= k}): ${String((e as any)?.message ?? e).slice(0, 100)}`); }
      const S = g.saves;
      // (a take is due this tick: an authored checkpoint a rule asked for, a save by hand, a load. The puppet takes the whole state first. An autosave
      // the controller decides on inside its tick takes the whole state as the take starts: `taps`. Its gate and its triggers read the puppet as the
      // filtered apply left it.)
      // AN AUTHORED CHECKPOINT ON A LEVEL ON RAILS: the save controller stops the level's program for the take (savestate.ts:991 `D.stop(false);
      // g.rails.resume()`), and the program is the Rust game's: a run the apply made on the puppet has none of its own (director.ts:525 threw on
      // `R.drills`). The Rust side stops it first and the puppet takes that, so the controller finds no program running and keeps the floor as it is.
      if (S?.authored && X.face_stop_program?.(this.h)) { this.count('an authored checkpoint: the level\'s program stopped on the Rust side before the take'); this.applyStep(); if (this.audit) rust = this.applyWhole('audit'); }
      if (S && !this.audit && !this.wholeNow && (S.authored || S.hand || S.due)) this.applyWhole('a save is due');
      const s0 = g.chronicle.counts.saves; try { S?.tick(g, dt); } catch (e) { this.count(`saves.tick THREW on the puppet: ${String((e as any)?.message ?? e).slice(0, 120)}`); }
      if (g.chronicle.counts.saves !== s0) { const Ev = g.chronicle.events, e = Ev[Ev.length - 1], label = e?.kind === 'checkpoint' ? String(e.label ?? e.data?.label ?? '') : ''; this.count('saves.tick took a checkpoint (made on the Rust side by face_checkpoint)'); this.wrote.push(`${k} chron_save ${JSON.stringify(e?.kind === 'checkpoint' ? (e.label ?? e.data?.label ?? '') : 'NOT-LAST')}`);
        this.F.hand(enc.encode(label), (p, n) => X.face_checkpoint(this.h, p, n)); this.applyStep(); if (this.audit) rust = this.applyWhole('audit'); this.dispatch(); } }
    this.T.dress += now() - t - (this.nest - nest0);
    return rust; }
  // ---- audit: the puppet as the apply left it is what the next write-back is held to
  private auditAfter(rust: Uint8Array | null, k: number): void { const g = this.g, B = this.B; let t = now();
    if (this.audit && rust) { t = now(); const N = this.o.applyCheck ?? 50, chk = N > 0 && (k <= 3 || k % N === 0), A = B.A;
      if ((this.o.baseline ?? 'rust') === 'rust' && !chk) this.lastState = rust;
      else { const mine: Uint8Array = B.exportGame(g, B.reg); this.lastState = (this.o.baseline ?? 'rust') === 'rust' ? rust : mine;
        if (chk) { this.applyChecks++; if (mine.length !== rust.length || !A.buf(mine).equals(A.buf(rust))) { const cut: string[] = [], D = A.fieldsDiffering(rust, mine, cut).filter((x: any) => !B.SCHEMA.serial?.[x.q] && x.q !== 'tail'); for (const q of cut) this.CUTW.set(q, (this.CUTW.get(q) ?? 0) + 1);
          if (D.length) { this.applyUnequal++; for (const x of D) A.tallyAdd(this.APPLY_MISS, x.q, x.path, k); if (this.o.out && this.applyUnequal === 1) { writeFileSync(`${this.o.out}/apply-rust${k}.bin`, rust); writeFileSync(`${this.o.out}/apply-puppet${k}.bin`, mine); } } } } }
      this.T.check += now() - t; }
  }
  // ---- A QUIET RUN OF STEPS (tools/qa/headless.ts `W.steps`): between `quietBegin` and `quietEnd` a step is the host's half handed over, the Rust tick,
  // its own steps after it, and the camera's two numbers: no export, no apply, no out event dispatched, no dress step. As the run ends the puppet takes
  // what changed over the whole run in ONE filtered apply (the export's memo holds what was last sent), the out events of every tick of the run are
  // dispatched in their order, and the dress step runs once. WHAT DIFFERS from the same steps taken one by one: an autosave that fell due inside the
  // run is taken as it ends; the par clock and the verdict tick once for the run; an out event reaches the host's objects late.
  quietOn = false; quietSteps = 0; quietRuns = 0; private quietOwed = 0; private lastDt = 1 / 60; private lookNow: { follow: number[]; aim: number[] } | null = null;
  private tsUpload: (() => void) | null = null; tsOnly = false;
  /** true where the host can step quietly (the fast way, a face that answers `face_look`) */
  // NOT YET THE SAME PLAY, so it is asked for by name (RH_QUIET=1): the Rust tick writes the lens's own row at times (a start snaps it onto the man), a
  // quiet run does not bring that across, and the host hands its own lens back with the next record. tools/blitz/quiet-check.ts: the die and the
  // block equal on every one of 2,340 ticks, the host's half not (FollowCamera.target at tick 2). It wants the camera's rows on the quiet path.
  quietBegin(): boolean { if (this.audit || !this.F.X.face_look || OFF.has('quiet') || process.env.RH_QUIET !== '1') return false; this.quietOn = true; this.quietOwed = 0; return true; }
  quietEnd(): void { if (!this.quietOn) return; this.quietOn = false; this.lookNow = null; if (!this.quietOwed || !this.h) return; this.quietRuns++;
    const g = this.g; this.inStep = true; this.tail(this.lastDt, this.k);
    this.loadsRef = g.chronicle?.counts?.loads ?? 0; this.missionRef = g.mission; this.inStep = false; this.dieRef = g.dice?.n; this.quietOwed = 0;
    g.engine.world.dynamics = g.dynBoxes; this.tsUpload?.(); }
  /** in a quiet run: where the host's camera is to follow and the seat's aim point, as the Rust game has them */
  look(): { follow: number[]; aim: number[] } | null { return this.quietOn ? this.lookNow : null; }
  /** fast: is a whole apply owed this tick, and why */
  private wholeOwed(k: number): string | null {
    const g = this.g, N = this.o.wholeEvery ?? 0;
    if (N > 0 && k % N === 0) return `every ${N} ticks`;
    // STAND-IN: THE END'S READS. The scripted player's report reads the game as its last tick left it, and much of what it reads is not on the filtered
    // list. A man down, the mission's stage moved or one of the ends the scripted player asks for set is where an end, or a tally's note of a death, is read. Belongs on the generated list.
    const st = this.endMark(), dn = this.downCount();
    if (OFF.has('ends')) return null;
    if (st !== this.stageRef || dn !== this.downRef) { this.stageRef = st; this.downRef = dn; return 'a man went down, a door order began or ended, the mission moved on or ended'; }
    return null; }

  /** every ms this host has spent, the driver's calls apart (they are made outside `Game.update`) */
  spent(): number { const T = this.T; return T.wbExport + T.wbCompare + T.wbImport + T.hostHalf + T.step + T.after + T.exportStep + T.applyStep + T.exportWhole + T.applyWhole +  T.block + T.out + T.dress + T.check + T.start; }
  /** the table of where a tick's time went, per tick, as PLAY.md has it */
  times(): string { const k = Math.max(1, this.k), per = (x: number) => (x / k).toFixed(3), T = this.T, W = [...this.wholes.values()].reduce((a, b) => a + b, 0);
    return (this.audit ? `write-back: the puppet's export ${per(T.wbExport)}, the compare ${per(T.wbCompare)}, the hand-over ${per(T.wbImport)} · ` : '') +
      `the host's half ${per(T.hostHalf)} (${this.hostBytes} bytes) · face_step ${per(T.step)} (worst ${this.worstStep.toFixed(1)} at tick ${this.worstStepAt}; the memory grew inside ${this.grew} steps) · lights, view, follow${this.audit ? ', listing' : ''} ${per(T.after)} · the filtered export ${per(T.exportStep)} (${(this.stepBytes / Math.max(1, this.steps) / 1024).toFixed(0)} KB a time) · the filtered apply ${per(T.applyStep)} · the whole export ${per(T.exportWhole)} and apply ${per(T.applyWhole)} (${W} whole applies, ${(this.wholeBytes / 1048576).toFixed(1)} MB the last; worst apply of either kind ${this.worstApply.toFixed(0)}) · the block's copy ${per(T.block)} · out events ${per(T.out)} · the dress step ${per(T.dress)} · the driver's calls ${per(T.calls)}` +
      (this.audit ? ` · the puppet exported again and held to the Rust bytes ${per(T.check)}` : '') + ` · the start, with the warm-up, once: ${T.start.toFixed(0)} ms`; }
}

// ---- THE HARNESS'S WORD (tools/qa/headless.ts `standUp`): TPS_GAME=rust the game the harness stands is a puppet of the Rust game; TPS_GAME=audit the
// same with the audit; TPS_GAME=ts the SAME STAND (the game's own die, the dress rows down) stepped by the TypeScript tick, for a tally to hold the Rust
// game's beside. Unset: nothing here is loaded. (Not TPS_RUST: src/sim/sim.ts refuses that word as a retired one.)
export async function standRust(g: any, word: string): Promise<RustHost | null> {
  if (!(globalThis as any).__tpsTop) throw new Error(`TPS_GAME=${word} needs the bridge's preload in this process and its children: BUN_OPTIONS=--preload=./tools/blitz/expose.ts (tools/blitz/rust.sh sets both)`);
  // (THE DIE: the game's own, seeded off the tool's `Math.random` as it stands when the game is stood, so a tool that seeds a run by its seed number
  // (playthrough.ts:1646) rolls another die a seed, and the same die under `ts` and `rust`)
  const die = g.dice ? null : (Math.random() * 4294967296) >>> 0;
  hang(g, 'gameWord', word);   // (the word this game was stood under: tools/qa/headless.ts `seedGame` reads it)
  if (process.env.RH_DIE_SAY) appendFileSync(process.env.RH_DIE_SAY, `${process.argv.slice(2).join(' ')} die ${die}\n`);   // (to play the same game again by hand: mission.ts --die, play.ts --die)
  if (word === 'ts') { const B = await bridge(); B.dressAll(false); if (die !== null) g.dice = B.newDice(die);
    // (the trace under `ts`: the same line a game, its ticks counted round the TypeScript tick itself)
    if (TICKS_DIR) { const F = loadFace(), file = ticksFile(word), was = g.stepSeats, gave: string[] = []; let k = 0; writeFileSync(file, ''); const B = await bridge(); g.stepSeats = function (this: any, ...a: any[]) { let half: Uint8Array | null = null; try { if (!k) { g.sim.settle(g); B.exportGame(g, B.reg); const O = g.orders, was2 = O?.give; if (was2) hang(O, 'give', function (this: any, id: string, ...r: any[]) { gave.push(`${k + 1}:${id}`); return was2.call(this, id, ...r); }); }   /* (the bridge's book, as the host's start makes it) */ half = B.hostHalf(g, B.reg, a[1]); } catch { /* a game the bridge has no book for yet */ } const r = was.apply(this, a); const w: Uint32Array = g.sim.w; tickLine(F, file, ++k, g.dice?.n, new Uint8Array(w.buffer, w.byteOffset, w.byteLength), half, gave.join(',')); return r; }; }
    if (process.env.RH_TRACE) { const rec = traceRec(word), was = g.stepSeats; g.stepSeats = function (this: any, ...a: any[]) { rec.k++; if (SITES) site(rec); if (FOLDED) folded(rec); if (CAP_AT && (rec.k & 31) === 0 && Date.now() > CAP_AT) process.exit(143); return was.apply(this, a); }; }
    return null; }
  if (word !== 'rust' && word !== 'audit') throw new Error(`TPS_GAME=${word}: rust (the Rust game steps), audit, or ts (the same stand, the TypeScript tick)`);
  await RustHost.ready(); const H = RustHost.attach(g, { audit: word === 'audit', quiet: true, die, wholeEvery: +(process.env.TPS_GAME_WHOLE ?? 0) });
  if (process.env.RH_TRACE) trace(H, word);
  if (process.env.RH_TIMES) timed(H);
  return H;
}
// ---- RH_TIMES=<file>: as the process ends, a line a game stood on the Rust game: the ticks it stepped, the clock from its first step to its last (the
// tool's own code between two steps is in it), and every ms the host spent by what it did (the same sums `times()` prints per tick)
const TIMED: RustHost[] = [];
function timed(H: RustHost): void {
  if (!TIMED.length) { process.on('SIGTERM', () => process.exit(143)); process.on('exit', () => { try { appendFileSync(process.env.RH_TIMES!, TIMED.filter(h => h.k > 0).map(h => JSON.stringify({ pid: process.pid, tool: (process.argv[1] ?? '').split('/').pop(), args: process.argv.slice(2, 6).join(' ').slice(0, 80), probe: (globalThis as any).__probeNow ?? process.env.RH_PROBE ?? null,
    k: h.k, wall: +(h.wallLast - h.wall0).toFixed(1), spent: +h.spent().toFixed(1), T: Object.fromEntries(Object.entries(h.T).map(([n, v]) => [n, +(v as number).toFixed(1)])), starts: h.starts, wholes: Object.fromEntries(h.wholes), calls: Object.fromEntries(h.calls), evicted: h.evicted })).join('\n') + '\n'); } catch { /* the line is not the tool's */ } }); }
  TIMED.push(H); }
// (TPS_GAME=table, and a run forced off its word: tools/blitz/word.ts, the one switch; `standRust` is handed the word that came out of it)
// ---- RH_TRACE=<file>: as the process ends, a line a game stood under the word: the tool's file that stood it (the first frame of the stack under tools/
// that is not the harness's own), the probe running then (probes.ts names it), the ticks it stepped, its whole applies, its calls, what threw
// ---- RH_TICKS=<folder>: a line a tick under either word (the tick, the die's count, a hash of each part of rust/sim's block but `poses`), in a file
// named by the word and the tool's own arguments: two plays of one tool under `ts` and `rust` are then held to each other tick by tick, where the tool's
// own stand (its die, its own draws) is not one `play.ts` can make again
const TICKS_DIR = process.env.RH_TICKS ?? '';
function tickLine(F: Face, file: string, k: number, die: unknown, block: Uint8Array, half: Uint8Array | null, gave = ''): void { const X = F.X, base = X.state_addr(0) >>> 0; let l = `${k} ${die} ${half ? Bun.hash(half).toString(36) : '-'}|${gave}`;   // (the third word: the host's half handed in as the tick opened)
  for (let i = 0; i < F.NAMES.length; i++) { if (F.NAMES[i] === 'poses') continue; const off = (X.face_part_addr(0, i) >>> 0) - base, len = X.face_part_len(i); l += ` ${Bun.hash(block.subarray(off, off + len)).toString(36)}`; }
  appendFileSync(file, l + '\n'); }
const ticksFile = (word: string) => `${TICKS_DIR}/${word}-${process.argv.slice(2).filter(a => !a.startsWith('-') && !a.includes('/') && !a.startsWith('{') && !/^\d+$/.test(a)).join('_').replace(/[^\w.-]+/g, '-') || 'run'}.txt`;
const TRACED: any[] = [];
/** RH_SITES=1 (with RH_TRACE): every tick is counted by the place in the tool that asked for it (the first frame of the stack under tools/ that is not the
 *  harness's own), for a count of how a tool steps: in a loop of its own with nothing between two steps, or with its own code between */
const SITES = process.env.RH_SITES === '1';
function site(rec: any): void { const st = String(new Error().stack ?? '').split('\n'), F: string[] = [];
  for (const l of st) { const m = l.match(/tools\/[\w./-]+\.ts:\d+:\d+/); if (m && !/rust-host\.ts|headless\.ts/.test(m[0])) { F.push(m[0]); if (F.length === 4) break; } }
  const at = F.join(' < ') || '?'; (rec.sites ??= {})[at] = (rec.sites[at] ?? 0) + 1; }
/** RH_FOLDED=<port-list.json> (with RH_TRACE): as every tick opens, each SETTLED switch row (a constant in the Rust game: the page's branch was
 *  translated and the other dropped) is held to its value on the page; a row that stands on its other side while the game ticks is counted in the game's
 *  line, `folded: { "TABLE.key=value": ticks }`. What a check pins off, as it runs: such a check asks the Rust game for a game it does not have. */
const FOLDED = process.env.RH_FOLDED ?? ''; let FROWS: { row: string; page: unknown; get: () => unknown }[] | null = null;
function folded(rec: any): void {
  if (!FROWS) { FROWS = []; const reg: any = BR?.reg; if (!reg) return; for (const r of JSON.parse(readFileSync(FOLDED, 'utf8')).rows as any[]) { if (r.kind !== 'settled' || (typeof r.page !== 'boolean' && typeof r.page !== 'number')) continue; const [t, ...path] = String(r.row).split('.'); const T = reg[t]; if (!T) continue;
    const get = () => { let v: any = T; for (const k of path) v = v?.[k]; return v; }; const v = get(); if (typeof v !== 'boolean' && typeof v !== 'number') continue; FROWS.push({ row: r.row, page: r.page, get }); } }
  // (a yes/no row kept as 0 / 1 in one place and false / true in the other is the same side)
  for (const R of FROWS) { const v = R.get(); if (v === R.page || (typeof R.page === 'boolean' && typeof v === 'number' && !!v === R.page) || (typeof R.page === 'number' && typeof v === 'boolean' && v === !!R.page && (R.page === 0 || R.page === 1))) continue; const k = `${R.row}=${v}`; (rec.folded ??= {})[k] = (rec.folded[k] ?? 0) + 1; } }
/** RH_CAP_AT=<ms since 1970>: a traced tool ends itself at its next tick after that moment (its trace is written as it ends) */
const CAP_AT = +(process.env.RH_CAP_AT ?? 0);
function traceRec(word: string): any {
  const st = String(new Error().stack ?? '').split('\n').map(l => l.match(/tools\/[\w./-]+\.ts:\d+/)?.[0]).filter((x): x is string => !!x && !/rust-host\.ts|headless\.ts/.test(x));
  const rec = { word, probe: (globalThis as any).__probeNow ?? process.env.RH_PROBE ?? null, by: st[0] ?? '?', k: 0, evicted: 0, starts: 0, wholes: new Map<string, number>(), calls: new Map<string, number>() };
  // (a tool stopped by a time cap still leaves its lines: the cap's signal becomes an ordinary exit)
  if (!TRACED.length) process.on('SIGTERM', () => process.exit(143));
  if (!TRACED.length) process.on('exit', () => { try { appendFileSync(process.env.RH_TRACE!, TRACED.map(r => JSON.stringify({ pid: process.pid, ...r, wholes: Object.fromEntries(r.wholes), calls: Object.fromEntries(r.calls), outThrew: r.outThrew ? [...r.outThrew.keys()] : undefined, WB: r.WB ? { fields: r.WB.size, top: [...r.WB].sort((a: any, b: any) => b[1].ticks - a[1].ticks).slice(0, 10).map(([q, e]: any) => `${q} x${e.ticks} (${e.eg})`) } : undefined })).join('\n') + '\n'); } catch { /* the trace is not the tool's */ } });
  TRACED.push(rec); return rec; }
function trace(H: RustHost, word: string): void { const rec = H.rec = traceRec(word); rec.wholes = H.wholes; rec.calls = H.calls; rec.wb = H.wb; rec.WB = H.WB; rec.outThrew = H.outThrew; }
