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
import { NO_VIEW } from '../../src/view/seatview';
import { useDice } from '../../src/math/dice';
import { faceHere, FACE_WASM } from './face-build.ts';
import { recorded } from './record';
import { Drawn, drawRefused } from '../../src/draw/drawn';
import { Want } from '../../src/draw/sections';
import { fogFrom } from '../../src/draw/fog';
import { readFileSync, existsSync, writeFileSync, appendFileSync } from 'node:fs';

export interface HostOpts {
  /** 'page': the short list; else the harness's, every field the tick moves (PuppetMemo.wide) */
  list?: 'page' | 'harness';
  /** the module (default: rust/game_face's release build) */
  wasm?: string;
  audit?: boolean;
  /** fast: a whole apply every N ticks beside the filtered one (0: only where the host decides one is owed) */
  wholeEvery?: number;
  /** false: the verdict and the autosave are not run on the puppet */
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
  /** false: the puppet is not walked for a tool's writes (scan.ts) before a step or a crossing. For a stand where NOTHING writes the puppet between
   *  two ticks: a networked game's machine, whose every write is a seat's record through `stepSeats` (tools/mp/stand.ts). A write made there all the
   *  same never reaches the Rust game */
  scan?: boolean;
  /** false: the dressing rows are up FOR THE GAME'S SAKE and nothing is drawn here, so the page's own dress step is not run on the puppet (the men
   *  handed to a renderer, the wind's hand-over, the air): a game server and a scripted client. The verdict still ticks */
  draws?: boolean;
}

// ---- the module: one instance a process, as many games as rust/sim has slots
export interface Face { X: any; mem: WebAssembly.Memory; rec: ReturnType<typeof recorded> | null; NAMES: string[]; u8(): Uint8Array; u32(): Uint32Array; outText(): string; outView(): Uint8Array; panicText(): string; hand<T>(b: Uint8Array, f: (p: number, n: number) => T): T; partHashes(slot: number): number[] }
const FACES = new Map<string, Face>();
/** the module a tool plays with no `--wasm`: the page's own choice (face-build.ts: a build of this machine that is up to date, else the checked-in module of these
 *  sources), so a machine with no cargo plays the same bytes the page serves; with neither, the release build's path, which `loadFace` then says is missing or which is stale by the builder's word */
export const DEFAULT_WASM = faceHere() ?? FACE_WASM;
export function loadFace(wasm = DEFAULT_WASM, fresh = false): Face {
  let F = fresh ? undefined : FACES.get(wasm); if (F) return F;
  if (!existsSync(wasm)) throw new Error(`rust-host: no module at ${wasm} (cd rust/game_face && cargo build --target wasm32-unknown-unknown --release; or the checked-in one, which is taken only where it is of this checkout's sources: bun run tools/blitz/face-build.ts --where)`);
  let rec: ReturnType<typeof recorded> | null = null; const X0: any = new WebAssembly.Instance(new WebAssembly.Module(new Uint8Array(readFileSync(wasm))), { env: { face_roll: () => { const v = (globalThis as any).__tpsRoll(); rec?.roll(v); return v; } } }).exports, mem: WebAssembly.Memory = X0.memory, dec = new TextDecoder();
  if (process.env.TPS_GAME_RECORD) rec = recorded(X0, process.env.TPS_GAME_RECORD);   // (record.ts: every call into the game, to play the run again alone)
  const X: any = rec ? rec.X : X0;
  const u8 = () => new Uint8Array(mem.buffer), u32 = () => new Uint32Array(mem.buffer);
  // (an address past 2 GB comes back from the module as a negative number: `>>> 0`)
  const outView = () => { const p = X.face_out_ptr() >>> 0; return u8().subarray(p, p + (X.face_out_len() >>> 0)); };
  const outText = () => dec.decode(outView());
  const NAMES: string[] = []; for (let i = 0; i < X.face_parts(); i++) { X.face_part_name(i); NAMES.push(outText()); }
  F = { X, mem, rec, NAMES, u8, u32, outText, outView,
    panicText: () => { try { const p = X.face_panic_ptr() >>> 0; return dec.decode(u8().subarray(p, p + X.face_panic_len())); } catch { return ''; } },
    hand<T>(b: Uint8Array, f: (p: number, n: number) => T): T { const p = X.face_alloc(b.length) >>> 0; u8().set(b, p); try { return f(p, b.length); } catch (e) { const w = F!.panicText(); if (w) console.error('the Rust game panicked:', w); throw e; } finally { X.face_free(p, b.length); } },
    partHashes: (slot: number) => { const at = (X.face_part_hashes(slot) >>> 0) >>> 2; return Array.from(u32().subarray(at, at + NAMES.length)); } };
  const no = drawRefused(X); if (no) throw new Error(`rust-host: ${no}`);
  if (!fresh) FACES.set(wasm, F); return F; }

// ---- the bridge, loaded once and after the game's own modules (classes.ts enters src/game by game.ts, the one door)
let BR: any = null;
async function bridge(): Promise<any> {
  if (BR) return BR;
  const SU = await import('./surface.ts'), SC = await import('./scan.ts'), WA = await import('../../src/view/worldask.ts'), RV = await import('../../src/view/read.ts'), CU = await import('../../src/view/cursor.ts'); SU.installSurface(); const E = await import('./export.ts'), I = await import('./import.ts'), AK = await import('./ask.ts'), C = await import('./classes.ts'), P = await import('./puppet-apply.ts'), O = await import('./out-dispatch.ts'), A = await import('./audit.ts');
  const K = await import('../../src/game/knobs.ts'), D = await import('../../src/math/dice.ts'), DR = await import('../../src/game/dress.ts'); const PD = await import('../../src/host/rustdress.ts');
  const LJ = JSON.parse(readFileSync(new URL('./puppet-lists.json', import.meta.url).pathname, 'utf8'));
  O.OUT_HOST.tap = SU.fireTap; O.OUT_HOST.endCard = (g: any) => { g.endCards = (g.endCards ?? 0) + 1; };   /* (a tool has no card: it counts them, `game.endCards`) */ BR = { ...E, pageDress: PD.dressStep, surfaceHost: SU.surfaceHost, readTickView: RV.readTickView, readCursorView: RV.readCursorView, readUnder: RV.readUnder, readDebriefView: RV.readDebriefView, CURSOR_DOUBLES: CU.CURSOR_DOUBLES, sampleCursor: CU.sampleCursor, heardOf: WA.heardOf, makeTaps: SU.makeTaps, makeScan: SC.makeScan, installAsks: AK.installAsks, applyGame: I.applyGame, hooks: { classes: C.classTable(), values: C.valueTable(), made: Object.fromEntries(MEN.map(c => [c, (o: object) => { MADE_MEN.add(o); MADE.n++; }])) }, puppetRows: P.puppetRows, SLOW: P.SLOW, VALS: P.VALS, OUT: O.OUT, A, reg: K.knobRegistry(), newDice: D.newDice, dressAll: DR.dressAll, DRESS: DR.DRESS,
    lists: { step: new Set<string>(LJ.step), start: new Set<string>(LJ.start) } };
  BR.PUT_BACKS = (await import('../../src/math/side.ts')).PUT_BACKS;
  { const SS = await import('../../src/game/savestate.ts'); snaps(SS); BR.SNAPS = SS.SNAPS; BR.SAVETIMELINE = SS.SAVETIMELINE; }
  return BR; }

// ---- A GAME KEPT IS THE RUST GAME'S OWN, one path with the page (src/host/rustgame.ts; rust/game_face/src/page.rs `face_keep` / `face_back` / `face_loaded`).
// The save controller (src/game/savestate.ts) takes and puts back through `SNAPS`, one seam a process. Here it asks the game's host: a game that has one
// keeps a clone of the whole Rust `Game` and its block inside the module (nothing of the puppet is walked) and the controller's slot holds its number; a
// game with none (a check the table keeps on the TypeScript tick, stood in the same process) goes through rollback.ts as it always did. `own` (every save
// is one in memory; no file of the puppet's fields is written, offered or loaded) is up only while a hosted game's controller runs.
interface KeptGame { game: any; rustKept: number; host: RustHost }
let OWN = 0;
function snaps(SS: any): void { const N = SS.SNAPS, take = N.take, put = N.put; let own: boolean = N.own;
  N.take = (g: any, since?: any, label?: string) => { const H: RustHost | undefined = g?.rustHost; return H?.keeps() ? H.snapTake(label) : take(g, since?.rustKept ? undefined : since); };
  N.put = (k: any) => { if (k?.rustKept) (k.host as RustHost).snapPut(k); else put(k); };
  Object.defineProperty(N, 'own', { configurable: true, enumerable: true, get: () => own || OWN > 0, set: (v: boolean) => { own = v; } }); }
/** the tool's line that asked: the first frame of the stack under tools/ that is not the harness's or the bridge's own */
const toolLine = (): string => String(new Error().stack ?? '').split('\n').map(l => l.match(/tools\/[\w./-]+\.ts:\d+/)?.[0]).find(x => !!x && !/rust-host\.ts|headless\.ts|surface\.ts|scan\.ts/.test(x)) ?? 'a line outside tools/';
/** a harness snapshot (tools/qa/headless.ts `W.snapshot`) and the Rust game kept with it; the kept game is let go when the tool lets the snapshot go */
const TOOL_KEPT = new WeakMap<object, { k: number; sig: bigint | number; host: RustHost }>();
const TOOL_GONE = new FinalizationRegistry<{ k: number; cell: Cell }>(e => { const i = e.cell.tool.indexOf(e.k); if (i >= 0) { e.cell.tool.splice(i, 1); e.cell.X.face_kept_drop(e.k); } });
/** THE GAMES THAT GAVE THEIR SLOT UP AND KEEP THEIR BYTES (a clone in the module, 10 to 30 MB each): the last few; an older one's are let go and its re-step is refused */
const OFF_KEPT: Cell[] = [], OFF_CAP = 6;
/** WHAT A HOST HOLDS IN THE MODULE: its game's handle, the clone kept as its slot went, the games kept for saves and for a tool's snapshots. It is kept
 *  apart from the host, and every list of this file holds a host by its cell, so that NOTHING HERE KEEPS A GAME THE TOOL HAS LET GO: once the collector has
 *  taken a host, `reap` (as the next game is attached, and before a slot is taken from a game still held) gives back what it held in the module. */
type Cell = { h: number; keptOff: number; shelf: Set<number>; tool: number[]; who: WeakRef<RustHost>; X: any };
const CELLS = new Set<Cell>();
function letGo(c: Cell): void { const X = c.X; CELLS.delete(c); if (!X) return; for (const k of c.shelf) X.face_kept_drop(k); c.shelf.clear(); for (const k of c.tool) X.face_kept_drop(k); c.tool.length = 0;
  if (c.keptOff > 0) { X.face_kept_drop(c.keptOff); const i = OFF_KEPT.indexOf(c); if (i >= 0) OFF_KEPT.splice(i, 1); } c.keptOff = 0; if (c.h) { X.face_puppet_drop?.(c.h); X.face_drop(c.h); c.h = 0; } }
const REAPED = { n: 0 };
function reap(): number { let n = 0; for (const c of [...CELLS]) if (!c.who.deref()) { letGo(c); n++; } REAPED.n += n; return n; }
/** (for a check: how many hosts hold something in the module, and how many were given back because nothing held them) */
export const moduleHolds = () => ({ held: CELLS.size, reaped: REAPED.n });
/** the classes of a man: a row of one of these that the APPLY made (a man the Rust game spawned) has no store row and no pose store on the TypeScript side */
const MEN = ['Character', 'Guard', 'Player'], MADE_MEN = new WeakSet<object>(), MADE = { n: 0 };

const now = () => performance.now();
/** RH_WHO=1: the fiftieth crossing of each row of the call surface is counted with the source lines that led to it (who asks a rule between two ticks) */
const WHO = process.env.RH_WHO === '1';
/** RH_SCAN_AUDIT=<n> (64): one tick in n the walks between it and the next encode every structured leaf (1: every walk, as before the cheap word; 0: never) */
const SCAN_AUDIT = +(process.env.RH_SCAN_AUDIT ?? 64);
// THE ROWS OF THE CALL SURFACE WHOSE RUST FUNCTION CANNOT WRITE THE GAME (tools/blitz/reads.json: the keys of calls.json's rows whose function takes the
// game by a shared borrow, by the compiler's proof; made by the program that demotes `&mut Game` wherever the crate still compiles; EMPTY until it
// has run). After a crossing of such a row nothing can have moved: no export, no apply, no second walk. The walk BEFORE it stays: what a tool wrote
// must reach the Rust game before any rule reads it. Never a guess: a row is on the list or the puppet takes the state again.
// A SECOND LIST, BY HAND (tools/blitz/reads-contract.json): a row whose function takes `&mut Game` and puts back what it takes, by its own contract.
// Each row there carries the sentence of that contract and the check that holds it; the host treats it as a row of the first list.
let READS: Set<number> | null = null;
function reads(): Set<number> { if (READS) return READS; READS = new Set(); try { const dir = new URL('.', import.meta.url).pathname, keys = new Set<string>(JSON.parse(readFileSync(`${dir}reads.json`, 'utf8')).rows ?? []); try { for (const r of JSON.parse(readFileSync(`${dir}reads-contract.json`, 'utf8')).rows as { key: string }[]) keys.add(r.key); } catch { /* no second list */ } if (keys.size) for (const r of JSON.parse(readFileSync(`${dir}calls.json`, 'utf8')).rows as { id: number; key: string }[]) if (keys.has(r.key)) READS.add(r.id); } catch { /* no list: every row may write */ } return READS; }
const ASKS = ['label', 'can', 'only', 'valid', 'when', 'gone', 'mark', 'given', 'issue', 'give'];
/** how many hosted games of the process are inside their own update now (see `surfaceOpen`) */
const BUSY = { n: 0 };
/** a field hung on a game's object that a walk of its own keys does not meet */
const hang = (o: any, k: string, v: unknown) => { Object.defineProperty(o, k, { value: v, writable: true, configurable: true, enumerable: false }); };
/** the objects whose calls the host has already taken (a start may meet new ones: a mission started by the tool makes its own menu, fight and saves) */
const TAPPED = new WeakSet<object>();
/** RH_OFF=<names, comma>: a stand-in left out, to see whether a run still needs it: ends · doors · marks · own (a weak table lists every game's keys) · waiting (a game takes its first start at its first step, whatever came between) · standing (calls cross while a game stands) */
const OFF = new Set((process.env.RH_OFF ?? '').split(','));
const enc = new TextEncoder();
type Tally = Map<string, { n: number; eg: string; ticks: number; last: number }>;

export class RustHost {
  /** ticks this host has stepped */
  k = 0;
  /** the Rust game's handle and rust/sim's slot it stands in */
  readonly cell: Cell = { h: 0, keptOff: 0, shelf: new Set<number>(), tool: [], who: new WeakRef(this), X: null };
  get h(): number { return this.cell.h; } set h(v: number) { this.cell.h = v; }
  slot = 0;
  readonly audit: boolean;
  /** the ms spent, by what */
  T = { wbExport: 0, wbCompare: 0, wbImport: 0, hostHalf: 0, step: 0, after: 0, exportStep: 0, applyStep: 0, exportWhole: 0, applyWhole: 0,  block: 0, out: 0, dress: 0, check: 0, calls: 0, start: 0, scan: 0 };
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
  /** the whole Rust state last applied (audit), and the puppet as the last tick opened */
  lastRust: Uint8Array | null = null; openState: Uint8Array | null = null;
  // ---- the audit's books
  WB: Tally = new Map(); HOSTW: Tally = new Map(); OWNW: Tally = new Map(); WB_REFUSED: Tally = new Map(); APPLY_MISS: Tally = new Map(); CUTW = new Map<string, number>(); BLOCKW = new Map<string, number>();
  wb = { fired: 0, meter: 0, block: 0, hostOnly: 0, refused: 0 }; handbacks = 0; handbacksInBeat = 0; applyChecks = 0; applyUnequal = 0;
  private lastState: Uint8Array | null = null; private lastBlock: Uint8Array | null = null;
  private pup: any; private say: (s: string) => void;
  private hoverApplied: unknown = null; private visApplied = NaN; private wholeNow = false;
  private lastChanged = true; private missionRef: any = null; private loadsRef = 0; private stageRef: any = null; private downRef = -1; private outQ: { arm: string; e: any }[] = [];
  private asks: { after(): void } | null = null; asked = 0; asksRefused = 0;
  private threw: Record<string, number> = {}; private warmDue = false;
  private dieRef: any = undefined; dieMoved = 0; private ticksFile = '';
  private memPages = 0; grew = 0; private inStep = false; private lastCmds: readonly any[] | null = null;
  /** ms spent in applies and exports so far (what a clock round a call or the dress step takes out of its own count) */
  private nest = 0;

  // ---- THE CALL SURFACE (surface.ts) AND THE SCAN (scan.ts): a tool's call of a rule goes to the Rust game by its row; a tool's write by name is found by a walk and goes by `io::poke`
  private busy = 0; private dispatching = false; private scan: any = null;
  get knobs() { return this.B.reg; }
  /** the fields only the harness reads go too (PuppetMemo.wide). `list: 'page'` leaves them out */
  private giveList(): void { this.F.X.face_puppet_wide?.(this.h, (this.o.list ?? process.env.TPS_GAME_LIST) === 'page' ? 0 : 1); }
  private tapsOn: any = null;
  /** after the host's half is taken: the view's matrix is the host camera's as the host left it (game_run::view_live), so a call made between two
   *  ticks (an order: on-screen-or-not) reads the camera of now and not the record of the tick before */
  private viewLive(): void { const X = this.F.X; if (!X.face_view_live || OFF.has('view')) return; try { X.face_view_live(this.h); } catch (e) { throw new Error(`the Rust game panicked taking the host's view (face_view_live): ${this.F.panicText() || String(e)}`); } }
  // A call's game is named by the call (surface.ts `hostOf`). Nothing crosses while any hosted game of the process is inside its update or the host's own lines (`BUSY`): the modules' tables are one, and they hold that game's state.
  surfaceOpen(): boolean { return !!this.h && this.busy === 0 && BUSY.n === 0 && !this.audit && !this.startDue && !RustHost.standing; }
  /** A CALL TO A FUNCTION WITH NO TYPESCRIPT BODY on a game that stands and has never started starts it, then crosses. A function with a body runs on the puppet, which the start takes across, and a game that has given up its slot or whose puppet has taken over starts by its own road. */
  surfaceStart(): boolean { if (this.starting || this.busy !== 0 || BUSY.n !== 0 || this.audit || this.h || this.starts > 0) return false; this.startNow(); return this.surfaceOpen(); }
  private starting = false;
  /** the Rust game stands and is this puppet's game (not let go, not waiting for a new start) */
  private live(): boolean { return !!this.h && !this.startDue; }
  surfaceDispatching(): boolean { return false; }
  inHook(): boolean { return this.dispatching; }
  /** which seat this host shows is its own fact: the Rust game hears it before an apply and before the host's half of a tick */
  private told = -1;
  private tellShown(): void { const g = this.g; if (!this.h || !(g?.seats?.length > 1)) return; const k = this.B.shownSeat(g); if (k === this.told) return; const was = this.told; this.told = k; this.F.X.face_show_seat(this.h, k);
    if (was >= 0 && !this.inApply) { this.applyStep(); this.scanRebase(); } }   // (the puppet's accessors answer another seat now: it takes that seat's values, and what it then holds is no tool's write)
  surfaceCall(row: number, bytes: Uint8Array, view = -1): Uint8Array { const X = this.F.X; if (WHO) { const n = (this.whoN.get(row) ?? 0) + 1; this.whoN.set(row, n); if (n === 50) this.count(`who calls row ${row}: ${String(new Error().stack ?? '').split('\n').slice(2).map(l => l.match(/((?:src|tools)\/[\w./-]+\.ts:\d+)/)?.[1]).filter(x => !!x && !/rust-host|surface\.ts/.test(x!)).slice(0, 6).join(' < ')}`); } this.scanFlush(); this.busy++; this.N.cross++; let t0 = now(); try { if (this.lastCmds) { this.tellShown(); const hb: Uint8Array = this.B.hostHalf(this.g, this.B.reg, this.lastCmds); this.F.hand(hb, (p, n) => X.face_host_half(this.h, p, n)); this.viewLive(); this.N.crossHalfMs += now() - t0; t0 = now(); }   /* (NO TICK YET, a start made before play — `startNow`, a save: the half is a record a seat, and there is none; the start has just carried every camera and Input row) */   /* (a rule that crosses reads the cursor and the cameras: they come with the host's half, and the tool has moved them since the tick) */ this.F.hand(bytes, (p, n) => X.face_call(this.h, row, view, p, n)); const out = this.F.outView().slice(); this.N.crossCallMs += now() - t0; t0 = now(); if (reads().has(row) && !OFF.has('reads')) { this.N.crossReads++; this.lastChanged = false; return out; } this.applyStep(); this.dispatch(); this.N.crossApplyMs += now() - t0; if (this.lastChanged) this.N.crossChanged++; return out; } finally { this.busy--; if (this.lastChanged || OFF.has('skipwalk')) this.scanRebase(); } }   /* (a crossing whose apply changed nothing and sent no event left the puppet as the walk last saw it: no second walk. `RH_OFF=skipwalk` walks as before.) */
  /** what the tool wrote since the walk last looked goes to the Rust game */
  private scanFlush(): void { if (this.audit || !this.h) return; this.tellShown(); { const t = now(); this.tapsOn?.sweep(); this.N.tapsMs += now() - t; const t1 = now(); this.blockBack(); this.N.blockBackMs += now() - t1; } if (this.o.scan === false || process.env.TPS_GAME_SCAN === '0') return; const t = now(); if (!this.scan) { this.scan = this.B.makeScan(this.g, this.B.reg, (b: Uint8Array) => { this.F.hand(b, (p, n) => this.F.X.face_write(this.h, p, n)); }, (q: string) => { if (q !== 'game::Game.cmd_buf') this.poked++; this.count(`scan: ${q}`); }, false, { traps: !OFF.has('traps'), audit: SCAN_AUDIT, every: process.env.RH_SCAN_EVERY === '1' /* (a measuring word: every carried field walked, whatever the tools name) */ }); this.scan.rebase(this.auditGap); } else this.scan.flush(this.auditGap); { const e = now() - t; this.T.scan += e; this.N.flush++; this.N.flushMs += e; }
    if (this.poked) { this.poked = 0; if (!OFF.has('pokesync') && !this.inApply) { this.pokeSyncs++; this.applyStep(); if (this.lastChanged) this.scanRebase(); } } }
  /** STAND-IN (belongs with the scan: a write by name of a field that lives in the block): WHAT THE TOOL WROTE INTO rust/sim's BLOCK. A door's swing and latch, a man's place and the die live in the block (the puppet's objects read and
   *  write its copy of it: `d.vel = 2`, `d.latched = false`), and no walk of the schema's fields meets them. After every apply the puppet's copy IS the
   *  Rust game's; where a part of it differs as a tick opens or a call is about to cross, the tool (or TypeScript run on the puppet) wrote it, and that
   *  part goes to the Rust game whole. `poses` is left out: a pose is posed again on either side and the two differ by design from the first death. */
  blockWrites = new Map<string, number>(); private partsAt: { name: string; off: number; len: number }[] | null = null;
  private blockBack(): void { if (OFF.has('block') || this.inApply) return; const X = this.F.X, mine = this.tsBlock(), theirs = this.rustBlock(), n = Math.min(mine.length, theirs.length), A = this.B.A;
    if (A.buf(mine.subarray(0, n)).equals(A.buf(theirs.subarray(0, n)))) return;
    this.partsAt ??= this.F.NAMES.map((name, i) => ({ name, off: (X.face_part_addr(0, i) >>> 0) - (X.state_addr(0) >>> 0), len: X.face_part_len(i) }));
    for (const P of this.partsAt) { if (P.name === 'poses' || P.off + P.len > n) continue; const a = mine.subarray(P.off, P.off + P.len); if (A.buf(a).equals(A.buf(theirs.subarray(P.off, P.off + P.len)))) continue;
      theirs.set(a, P.off); this.F.rec?.mem(theirs.byteOffset + P.off, a); this.blockWrites.set(P.name, (this.blockWrites.get(P.name) ?? 0) + 1); this.count(`the block: ${P.name} written on the puppet since the last apply (handed to the Rust game)`); } }
  private inApply = false; private whoN = new Map<number, number>();
  /** THE WALK'S AUDIT (scan.ts): from the walk after one tick in `SCAN_AUDIT` to the walk as the next opens, every walk encodes every structured leaf as
   *  every walk once did, so a write the cheap word did not see is sent, and counted on the exit line */
  private auditGap = false;
  /** counts for RH_TIMES: the walks by kind and their ms, the crossings, those whose apply changed the puppet, and the ms of a crossing's parts */
  N = { flush: 0, flushMs: 0, rebase: 0, rebaseMs: 0, cross: 0, crossReads: 0, crossChanged: 0, crossHalfMs: 0, crossCallMs: 0, crossApplyMs: 0, blockBackMs: 0, tapsMs: 0 };
  // STAND-IN (belongs in the generated poke: a write by path brings the memo's row of that path up to date): A FIELD THE TOOL WROTE AND A RULE THEN PUT
  // BACK. The walk pokes the tool's value into the Rust game; the every-tick export's memo still holds what the puppet was SENT before the tool wrote.
  // Where a rule then sets the field to that old value (the soak crouches the player by name and calls `startGrab`, which stands him up) nothing differs
  // from the memo, nothing is sent, and the puppet keeps the tool's value for good: the soak read "crouch true while holding" off the puppet with the
  // Rust game's player standing. So a walk that poked is followed at once by one every-tick export and apply: it carries exactly the poked fields (the
  // puppet holds them already) and leaves the memo holding them. The record of the tick (`Game.cmd_buf`, which the game's own update writes on the
  // puppet before every step and the walk reads as a write) does not count: it goes with the host's half. `RH_OFF=pokesync`: as before.
  // (MY WRONG TURN: sending EVERY row after a poke. The record is poked every tick, so every export was the long one, and the soak's seed 7 parted at
  // tick 1322 with the host's half differing from 1244.)
  private poked = 0; pokeSyncs = 0;
  /** an apply moved the puppet: that is what the next walk is held to */
  scanRebase(): void { if (!this.scan) return; const t = now(); this.scan.rebase(this.auditGap); const e = now() - t; this.T.scan += e; this.N.rebase++; this.N.rebaseMs += e; }

  private constructor(readonly g: any, readonly F: Face, readonly B: any, readonly o: HostOpts) {
    this.cell.X = F.X; this.audit = !!o.audit; this.say = o.say ?? ((s: string) => { process.stdout.write(s + '\n'); });
    const ctx: any = { p: 0 }; this.pup = { lists: B.lists, rows: B.puppetRows(ctx), ctx, slowTable: B.SLOW, vals: B.VALS };
  }
  /** the bridge and the module loaded: once, before the first `attach` */
  static async ready(wasm?: string): Promise<Face> { await bridge(); return loadFace(wasm); }
  /** before the game's first update (after `await RustHost.ready()`): from then on `game.update` steps the Rust game */
  static attach(g: any, o: HostOpts = {}): RustHost {
    if (!BR) throw new Error('rust-host: `await RustHost.ready()` first');
    // (RH_FRESH_FACE=1: a module of its own for every game, to tell what a second game of a process owes to the module's own memory from what it owes
    // to the TypeScript modules' tables)
    reap(); const B = BR, F = loadFace(o.wasm, process.env.RH_FRESH_FACE === '1'), H = new RustHost(g, F, B, o); if ((F.X.face_layout?.() >>> 0) !== B.SCHEMA.layout) throw new Error(`rust-host: this module is of another generation than this checkout's bridge (its layout ${F.X.face_layout?.() >>> 0}, the bridge's ${B.SCHEMA.layout}): bun run tools/blitz/face-build.ts builds this checkout's`);
    if (o.reserveMb !== 0) { const n = (o.reserveMb ?? 48) * 1048576, p = F.X.face_alloc(n); F.X.face_free(p, n); }
    if (o.dressDown !== false && !OFF.has('dress')) dressDown(B); else (globalThis as any).__tpsDressBack?.();   // (a game asked for with the rows up finds them up, whatever the game before it left)
    if (o.die !== null && !g.dice) g.dice = B.newDice(o.die ?? 20942);
    // (hung as fields no walk of the game's own keys meets: rollback.ts `stateHash` walks them, and a stand's word must not change for being a puppet)
    hang(g, 'stepSeats', function (dt: number, cmds: readonly any[]) { H.step(dt, cmds); });
    hang(g, 'rustHost', H); hang(g, 'debriefAsk', () => H.debriefView()); hang(g, 'rustSurface', H); hang(g, 'stateWords', () => ({ die: H.dieN(), parts: H.hashes() })); hang(g, 'wholeWords', () => H.wholeWords()); hang(g, 'wholeNames', () => H.wholeNames()); hang(g, 'verdictNow', () => H.verdict()); hang(g, 'viewAsk', () => H.view()); hang(g, 'cursorAsk', (i: any, c: any, v: any, s?: number) => H.cursor(i, c, v, s)); hang(g, 'underAsk', (p: any, s?: number) => H.under(p, s)); Object.defineProperty(g, 'askHost', { configurable: true, enumerable: false, get: () => H.ask() }); hang(g, 'quietRun', (on: boolean) => { if (on) H.quietBegin(); else H.quietEnd(); }); B.surfaceHost(H); (globalThis as any).__tpsCall.inHook = () => H.inHook(); { const up = g.update; hang(g, 'update', function (this: any, ...a: any[]) { H.busy++; BUSY.n++; try { return up.apply(this, a); } finally { H.busy--; BUSY.n--; H.auditGap = SCAN_AUDIT > 0 && H.k % SCAN_AUDIT === 0; H.scanRebase(); if (STALE_EVERY && process.env.RH_STALE && H.k > 0 && H.k % STALE_EVERY === 0 && H.busy === 0) H.staleLook(); } }); } H.taps(g);
    if (process.env.TPS_GAME_SAY) process.on('exit', () => { const L = [...H.calls].sort((a, b) => b[1] - a[1]).slice(0, 60); process.stderr.write(`rust-host: ${H.k} ticks; scan ${(H.T.scan / Math.max(1, H.k)).toFixed(2)} ms a tick; ${H.pokeSyncs} exports after a walk that poked; ${H.scan?.stats?.().deepMissed ?? 0} WRITES THE WALK'S CHEAP WORD DID NOT SEE (found by its audit: ${(H.scan?.stats?.().missed ?? []).join(', ')}); ${H.asksRefused} asks refused; ${H.scan?.stats?.().refused ?? 0} WRITES NOT SENT (a leaf that named a row the Rust game has not got: ${(H.scan?.stats?.().refusedLeaves ?? []).join(', ')}); ${JSON.stringify(H.scan?.stats?.() ?? null)}\n` + L.map(([n, c]) => `  ${c} ${n}\n`).join('')); });
    if (o.out) for (const f of ['writeback.txt', 'gives.txt']) writeFileSync(`${o.out}/${f}`, '');
    RustHost.waiting.add(new WeakRef(H)); RustHost.last = null;
    return H; }
  /** THE GAMES STOOD AND NOT YET STARTED. A first start reads what the TypeScript keeps at its modules' tops (the kit's staged men, a tally, a log), and those
   *  are the process's: the next stand-up writes them for ITS game, and another game's step writes them with its rows. So a game takes its start while they
   *  are its own: before another game is stood (`standComing`, as a stand-up opens) and before another game steps. A game that started later than either was
   *  handed the other's men and tallies as its own: rows of a Rust game with no poses in its block, a man who stands with no skeleton and aims a ray of no
   *  number (thrice --weave). Held weakly, and the process's other holds on it let go first: a game the tool has dropped is not started for nothing.
   *  FROM A STAND-UP TO THE NEXT START NO CALL CROSSES. A stand asks about things a process shares (the cast's rig, numbered in the book of the game that met it
   *  first, so the call names that game), and the answer's apply wrote the older game's rows over what `freshStart` had just put back: its men, some of them
   *  made in Rust with no rig, stood in the new game's start (`SkeletonInstance.rig: no object for RigId`). So a stand and its staging run their TypeScript,
   *  as the process's first stand does (tools/blitz/stand-after.ts). */
  private static waiting = new Set<WeakRef<RustHost>>(); private static standing = false; private startThrew: unknown = null;
  private beCurrent(): void { useDice(this.g.dice); this.g.quiet?.current?.(); }   // (what `Game.update` opens with: a start outside its own game's update reads this game's)
  static standComing(collect: () => void): void { (globalThis as any).__tpsCall?.newGame?.(); const c = (globalThis as any).__tpsCall; if (c) c.inHook = undefined; RustHost.last = null; collect();
    if (!OFF.has('waiting')) RustHost.startWaiting(); if (!OFF.has('standing')) RustHost.standing = true; }
  /** a stand-up that ended with no game on this word (refused, or stood with no word): nothing stands */
  static standEnded(): void { RustHost.standing = false; }
  static startWaiting(but: RustHost | null = null): void { let any = false; for (const r of [...RustHost.waiting]) { const H = r.deref(); if (H === but) continue; RustHost.waiting.delete(r); if (!H || H.starts > 0) continue; any = true;
    H.beCurrent(); try { H.startNow(); } catch (e) { H.startThrew = e; H.count(`THE START THREW as another game came: ${String((e as any)?.message ?? e).slice(0, 100)}`); } }
    if (any) but?.beCurrent(); }
  private startDue = false;
  /** the Rust game is let go (its slot is free for another) and the game steps in TypeScript again */
  detach(): void { for (const r of [...RustHost.waiting]) if (r.deref() === this) RustHost.waiting.delete(r); this.unkeepAll(); letGo(this.cell); if (RustHost.last === this) RustHost.last = null; delete this.g.stepSeats; delete this.g.rustHost; delete this.g.debriefAsk; delete this.g.stateWords; delete this.g.wholeWords; delete this.g.wholeNames; delete this.g.verdictNow; delete this.g.viewAsk; delete this.g.cursorAsk; delete this.g.underAsk; delete this.g.askHost; delete this.g.quietRun; delete this.g.rustSurface; delete this.g.update; delete this.g.restartEncounter; this.B.surfaceHost(null, this); }
  /** the hosts whose Rust game stands in a slot, and when each last stepped */
  private static clock = 0; private static last: RustHost | null = null; private stamp = 0; evicted = 0;
  /** this game's slot goes to another: the puppet takes the whole state, the Rust game is let go; a later step starts a new one from the puppet */
  private giveUp(): void { const X = this.F.X; if (!this.h) return;
    try { this.scanFlush(); this.applyWhole('its slot went to another game'); } catch (e) { this.count(`THE WHOLE APPLY THREW as the slot was given up: ${String((e as any)?.message ?? e).slice(0, 100)}`); }
    // THE GAME'S BYTES ARE KEPT (a clone of the Rust `Game` and its block, in the module): if the tool steps this game again it stands again as it was
    // (`restep`), with no export of the puppet. Only the last few games that gave a slot up keep theirs; an older one's re-step is refused.
    if (X.face_keep && !OFF.has('keep')) { this.keptOff = X.face_keep(this.h, 0) || -1; if (this.keptOff > 0) { OFF_KEPT.push(this.cell); while (OFF_KEPT.length > OFF_CAP) { const o = OFF_KEPT.shift()!; X.face_kept_drop(o.keptOff); o.keptOff = -1; } } }
    X.face_puppet_drop?.(this.h); X.face_drop(this.h); this.h = 0; if (!this.keptOff) this.scan = null; if (RustHost.last === this) RustHost.last = null; this.evicted++; if (this.rec) this.rec.evicted = this.evicted; }
  /** the number of this game's clone kept as its slot was given up (0 none, -1 let go since: a re-step is refused) */
  get keptOff(): number { return this.cell.keptOff; } set keptOff(v: number) { this.cell.keptOff = v; }
  resteps = 0;
  /** A GAME THAT GAVE ITS SLOT UP IS STEPPED AGAIN: a handle is made from the kept game's own bytes and the clone stands in it (`face_back`); the puppet,
   *  which took the whole state as the slot went, takes it again (the modules' own tables are a process's, and another game's applies have written them since). */
  private restep(): void { const X = this.F.X, k = this.keptOff, t = now(); this.keptOff = 0; { const i = OFF_KEPT.indexOf(this.cell); if (i >= 0) OFF_KEPT.splice(i, 1); }
    X.face_kept_state(k); const state = this.F.outView().slice(); X.face_kept_block(k); const block = this.F.outView().slice();
    this.make(state, block);
    if (!X.face_back(this.h, k)) throw new Error('rust-host: the game kept as its slot was given up is gone');
    X.face_kept_drop(k); this.seated(); this.resteps++; if (this.rec) this.rec.resteps = this.resteps;
    this.applyWhole('its Rust game stands again in a slot'); this.tellShown(); X.face_puppet(this.h, 1); this.refs(); this.T.start += now() - t;
    this.count('a game that had given its slot up was stepped again: it stands again from its own kept bytes'); }
  /** a handle from a start's bytes; where every slot is taken, the game stepped longest ago gives its slot up */
  private make(state: Uint8Array, block: Uint8Array): void { this.told = -1; const X = this.F.X, mk = () => this.F.hand(state, (p, n) => this.F.hand(block, (bp, bn) => X.face_new(p, n, bp, bn)));
    this.h = mk(); if (!this.h && reap()) this.h = mk();   // (a game nothing holds gives its slot back before one still held is asked to)
    while (!this.h) { let old: RustHost | null = null; for (const c of CELLS) { const o = c.h ? c.who.deref() : undefined; if (o && o !== this && (!old || o.stamp < old.stamp)) old = o; } if (!old) break; old.giveUp(); this.h = mk(); }
    if (!this.h) throw new Error('face_new: no free slot'); }
  /** a Rust game stands in a slot for this host: the list's word, the taps, the slot */
  private seated(): void { const X = this.F.X, g = this.g, B = this.B;
    CELLS.add(this.cell); this.stamp = ++RustHost.clock; this.giveList(); if (!this.audit) { this.tapsOn ??= B.makeTaps(g, B.hooks.classes, (id: number, on: boolean) => { X.face_tap(this.h, id, on ? 1 : 0); }); this.tapsOn.reset(); }
    this.slot = X.face_slot(this.h); }
  /** what a later step holds the puppet to, to tell a game the TypeScript put in the puppet's place from the one the Rust game steps */
  private refs(): void { const g = this.g; this.missionRef = g.mission; this.t0Ref = g.mission?.t0; this.loadsRef = g.chronicle?.counts?.loads ?? 0; this.putRef = this.B.PUT_BACKS.n; this.clockRef = this.clock(); this.stageRef = this.endMark(); this.downRef = this.downCount(); }
  private clock(): string { const g = this.g; return `${g.time}|${g.eventSeq}|${g.dice?.n}`; }
  private t0Ref: unknown = undefined; private putRef = 0; private clockRef = ''; private dueWhy = '';
  /** IS A NEW START OWED as a tick opens, and why (null: the Rust game goes on; 'back': it stands again from its own kept bytes).
   *  A START IS WHAT THE STAGE SAYS BEGAN: the mission's clock began again (`mission.t0`), a checkpoint's file was loaded on the puppet, a snapshot was put
   *  back on it. A mission RECORD swapped for a copy of itself (the old tour: `g.mission = { ...saved }` for a beat, the saved one back
   *  as it ends) is the same run: no start. */
  private startWhy(): string | null { const g = this.g, m = g.mission;
    if (this.starts === 0) return 'the first step';
    if (this.startDue) return this.dueWhy || 'a start was asked for';
    if ((g.chronicle?.counts?.loads ?? 0) !== this.loadsRef) return 'a checkpoint was loaded on the puppet';
    // (the count of put-backs is a process's: another game's snapshot moves it too. This game's was put back if its own clock is no longer the Rust game's)
    if (this.B.PUT_BACKS.n !== this.putRef) { this.putRef = this.B.PUT_BACKS.n; if (this.h && this.clock() !== this.clockRef) return 'a snapshot was put back on the puppet (rollback.ts restoreSnapshot)'; }
    if (m !== this.missionRef) { if (m && this.missionRef && m.t0 === this.t0Ref && !OFF.has('copy')) { this.missionRef = m; this.count('the mission record was swapped for a copy of itself (not a new start)'); } else return 'the mission began again on the puppet'; }
    if (!this.h) { if (this.keptOff > 0) return 'back'; return this.keptOff < 0 ? 'refused' : 'its Rust game was let go'; }
    return null; }

  // ---- A GAME KEPT (the page's path: `face_keep`, `face_back`, `face_loaded`)
  /** kept games of this host: the save controller's (swept against its shelf) and the tool's own snapshots */
  private get keptShelf(): Set<number> { return this.cell.shelf; } private get keptTool(): number[] { return this.cell.tool; } keepsN = 0; backs = 0;
  private backDue: { label: string; loads: number; saves: number } | null = null; private tookRow = false; private ctl = 0;
  /** does this host keep its saves as Rust games */
  keeps(): boolean { return !this.audit && typeof this.F.X.face_keep === 'function' && !OFF.has('keep'); }
  private keep(): number { const X = this.F.X, t = now(), k = X.face_keep(this.h, 0); if (!k) throw new Error('rust-host: the face kept no game'); this.keepsN++; if (this.rec) this.rec.keeps = this.keepsN; this.T.calls += now() - t; return k; }
  unkeep(k: number): void { if (this.keptShelf.delete(k)) this.F.X.face_kept_drop(k); else { const i = this.keptTool.indexOf(k); if (i >= 0) { this.keptTool.splice(i, 1); this.F.X.face_kept_drop(k); } } }
  private unkeepAll(): void { for (const k of this.keptShelf) this.F.X.face_kept_drop(k); this.keptShelf.clear(); for (const k of this.keptTool) this.F.X.face_kept_drop(k); this.keptTool.length = 0; if (this.startKept) { this.F.X.face_kept_drop(this.startKept); this.startKept = 0; } this.backDue = null; this.tookRow = false; }
  /** THE KEPT GAME STANDS AGAIN and the puppet takes its whole state: one whole apply; what the filtered export last sent is the whole state */
  private backTo(k: number, why: string): void { const X = this.F.X;
    if (!this.h || !X.face_back(this.h, k)) throw new Error(`rust-host: that kept game is gone (it was of a game that has been stood again since); asked by ${toolLine()}`);
    // THE HOST'S ROWS OF THE SAVED MOMENT. The TypeScript's put-back writes the camera and the Input back as they stood when the save was taken (the game
    // reaches both: the orders menu's view, the director), so the puppet's take the kept game's rows with the rest, and the host's rows of NOW are not
    // handed over first (a whole apply between two ticks does that, for a flooded room: here it would lay the abandoned future's camera over the saved
    // moment's, which is what parted the scripted player's first round after a load on a squad mission). `snapTake` hands them in before the keep.
    // (What may go on differing in the host's half afterwards is the lens's serial, the bridge's own count of the camera's `desired` arrays. No rule reads it.)
    this.told = -1; this.hostNow = OFF.has('backcam'); try { this.applyWhole(why); } finally { this.hostNow = true; } this.tellShown(); X.face_puppet(this.h, 1); this.refs(); this.scanRebase(); this.shelfSig = ''; this.backs++; if (this.rec) this.rec.backs = this.backs; }
  /** A SAVE (savestate.ts writeMem, through SNAPS.take). The chronicle's row is made on the Rust side first, under the save's label (`face_checkpoint`), so the kept game holds it. No state is walked in TypeScript and nothing is applied whole. */
  /** THE START MADE NOW, by whoever stood the game, where its first tick must cost a tick: a networked game's machine (tools/mp/stand.ts `begin`),
   *  whose server's clock runs from the first record. Left to the first step it is paid there: the whole export, the Rust start and the puppet's
   *  first apply, over a second on a small box. A start that is not owed is not made */
  startNow(): void { const why = this.startWhy(); if (why === 'back') this.restep(); else if (why !== null) this.start(why); }
  /** THE PAGE'S RESTART (src/host/rustgame.ts toStart): the game kept as its first step opened stands again, clock and die and all, and what the player and
   *  this machine have set since (the sandbox flags, the tune) stays as it is now. Kept only where a tool asks for it, before the first step: a walk of a page's tape
   *  (tools/qa/rerun.ts). With none kept, a restart asked inside a tick is the Rust game's own */
  startKept = 0;
  keepStart(): void { if (!this.keeps()) return; this.startNow(); if (!OFF.has('backcam')) this.handHalf(); this.startKept = this.keep(); }
  toStart(): void { const g = this.g, flags = { godMode: g.godMode, aiEnabled: g.aiEnabled, infiniteAmmo: g.infiniteAmmo, playerInvisible: g.playerInvisible }, tune = { ...g.tune };
    this.backTo(this.startKept, 'the start put back'); this.backDue = null; this.tookRow = false; g.saves?.startedAgain(); Object.assign(g, flags); Object.assign(g.tune, tune); this.count('a restart: the kept start stands again (face_back), one whole apply'); }
  snapTake(label?: string): KeptGame { const g = this.g, X = this.F.X;
    { const why = this.startWhy(); if (why === 'back') this.restep(); else if (why !== null) this.start(why); }
    if (!this.inStep) this.scanFlush();
    // (A SAVE BETWEEN TWO TICKS keeps the host's camera and Input as the host has moved them since the tick: the TypeScript's snapshot walks them as they
    // stand, so the kept game takes them first. Inside a tick they are the rows the tick opened with on both sides.)
    if (!OFF.has('backcam')) this.handHalf();
    if (label !== undefined) { this.F.hand(enc.encode(label), (p, n) => X.face_checkpoint(this.h, p, n)); this.tookRow = true; }
    // (every kept game no slot of the controller's names any more is let go: a ring's slot taken again, a new run's shelf)
    { const sh = g.saves?.shelf, live = new Set<number>(); if (sh?.all) { for (const e of sh.all()) live.add(e.snap?.rustKept | 0); live.add(sh.last?.rustKept ?? 0); for (const k of [...this.keptShelf]) if (!live.has(k)) { X.face_kept_drop(k); this.keptShelf.delete(k); } } }
    const k = this.keep(); this.keptShelf.add(k); this.count('a save: the Rust game kept in the module (face_keep)');
    return { game: g, rustKept: k, host: this }; }
  /** A LOAD (savestate.ts loadMem, through SNAPS.put). The controller goes on to write the puppet (the run's counts of saves and loads, the chronicle's
   *  row, the rings put away, a tutorial's program to be started again): those lines are made on the Rust side by `face_loaded` once it has (`settle`). */
  snapPut(K: KeptGame): void { const g = this.g, C = g.chronicle?.counts, e = g.saves?.shelf?.all?.().find((x: any) => x.snap === K);
    if (K.host !== this || !this.keptShelf.has(K.rustKept)) throw new Error(`rust-host: that save is not a game this host keeps; asked by ${toolLine()}`);
    // (the counts of the moment that is being abandoned: the run's, not the saved moment's. Read before the kept game's are on the puppet.)
    const back = { label: String(e?.info?.label ?? ''), loads: C?.loads | 0, saves: C?.saves | 0 };
    this.backTo(K.rustKept, 'a checkpoint put back'); this.backDue = back; this.count('a load: the kept game stands again (face_back), one whole apply'); }
  /** what follows a save or a load on the Rust side, once the controller's own lines have run on the puppet. NOT TAKEN ACROSS (as on the page): the
   *  tune's rows as they stand now, which the controller lays over the saved moment's on the puppet (savestate.ts:1129); its three sandbox flags are. */
  private settle(): boolean { const X = this.F.X; if (!this.h) return false;
    if (this.backDue) { const b = this.backDue; this.backDue = null; this.tookRow = false; { const g = this.g; X.face_poke(this.h, 0, g.godMode ? 1 : 0); X.face_poke(this.h, 1, g.aiEnabled ? 1 : 0); X.face_poke(this.h, 2, g.infiniteAmmo ? 1 : 0); } this.F.hand(enc.encode(b.label), (p, n) => X.face_loaded(this.h, p, n, b.loads, b.saves)); this.applyStep(); this.refs(); return true; }
    if (this.tookRow) { this.tookRow = false; this.applyStep(); return true; }
    return false; }
  private cutDue = false; private shelfSig = '';
  /** THE CONTROLLER'S SHELF AS THE TICK READS IT (Enter once down asks for this run's newest save, player.ts:493; the Rust game's `g.saves` is no part of
   *  the bridge): handed over as a step opens whenever it has changed. The page's own lines (src/host/rustgame.ts `shelfIn`), for `face_saves_in`. */
  private shelfIn(): void { const sv = this.g.saves, sh = sv?.shelf, X = this.F.X; if (!sv || !sh?.all || !X.face_saves_in) return;
    const rows: any[] = sh.all().map((e: any) => e.info), sig = `${sv.runId}|${rows.map(r => `${r.key}:${r.savedAt}:${r.regret ?? ''}`).join(',')}`; if (sig === this.shelfSig) return; this.shelfSig = sig;
    const parts: Uint8Array[] = [], u32 = (v: number) => { const b = new Uint8Array(4); new DataView(b.buffer).setUint32(0, v, true); parts.push(b); }, f64 = (v: number) => { const b = new Uint8Array(8); new DataView(b.buffer).setFloat64(0, v, true); parts.push(b); },
      text = (t: string) => { const b = enc.encode(t); u32(b.length); parts.push(b); }, byte = (v: number) => parts.push(Uint8Array.of(v));
    text(String(sv.runId ?? '')); u32(rows.length);
    for (const r of rows) { text(r.key); byte(r.slot === 'manual' ? 1 : 0); f64(r.n); text(r.label ?? ''); f64(r.savedAt); f64(r.missionT); text(r.stage ?? ''); text(r.runId ?? ''); if (r.regret) { byte(1); text(r.regret); } else byte(0); }
    let n = 0; for (const b of parts) n += b.length; const all = new Uint8Array(n); n = 0; for (const b of parts) { all.set(b, n); n += b.length; }
    this.F.hand(all, (p, m) => X.face_saves_in(this.h, p, m)); this.count('the controller\'s shelf handed to the Rust game (face_saves_in)'); }
  /** THE HARNESS'S OWN SNAPSHOT (tools/qa/headless.ts `W.snapshot`): the file of the puppet's fields is made as ever (a tool reads it), and the Rust game is
   *  kept beside it. `W.restore` of that same file, unchanged, is the kept game standing again; anything else is the TypeScript's `loadState` on the puppet. */
  keptFor(s: object): void { if (!this.keeps() || !this.live() || this.inStep) return; this.scanFlush(); if (!OFF.has('backcam')) this.handHalf();
    const k = this.keep(); this.keptTool.push(k); while (this.keptTool.length > 16) this.F.X.face_kept_drop(this.keptTool.shift()!);
    const sig = Bun.hash(JSON.stringify(s)); TOOL_KEPT.set(s, { k, sig, host: this }); this.keptSig.set(sig, k); TOOL_GONE.register(s, { k, cell: this.cell }); this.count('a snapshot: the Rust game kept beside the file (face_keep)'); }
  /** A COPY OF THE FILE WITH THE SAME BYTES IS THE SAME FILE (a tool that writes a snapshot out and reads it back, as a checkpoint's load does: `W.restore(
   *  JSON.parse(JSON.stringify(save)))`): the kept game is found by the file's content where the object is not the one that was kept, while the tool still
   *  holds the snapshot it took (its kept game goes when the collector takes that) */
  private keptSig = new Map<bigint | number, number>();
  backFor(s: object): boolean { let e = TOOL_KEPT.get(s); if (!e) { const sig = Bun.hash(JSON.stringify(s)), k = this.keptSig.get(sig); if (k !== undefined && this.keptTool.includes(k)) e = { k, sig, host: this }; } if (!e || e.host !== this || !this.keeps() || !this.live() || this.inStep || this.startWhy() !== null) return false;
    if (!this.keptTool.includes(e.k)) { this.count('a snapshot whose kept game was let go (more than 16 held): loaded by the TypeScript on the puppet'); return false; }
    if (Bun.hash(JSON.stringify(s)) !== e.sig) { this.count('a snapshot the tool changed since it was taken: loaded by the TypeScript on the puppet'); return false; }
    this.backTo(e.k, 'a snapshot put back'); this.count('a snapshot put back: the kept game stands again (face_back), one whole apply'); return true; }
  /** THE TYPESCRIPT IS ABOUT TO PUT ANOTHER GAME IN THE PUPPET'S PLACE (a file's `loadState`): from here to the next step the puppet is the game.
   *  Nothing crosses the call surface and no tap of the host's hands anything over or applies a state that is no longer the puppet's (`live`); the next
   *  step starts a new Rust game from the puppet. (The Rust game that stood is let go only then: an order row's questions, tools/blitz/ask.ts, are asked
   *  of whatever stands.) */
  private puppetTakesOver(why: string): void { if (!this.h || this.startDue) return; this.unkeepAll(); this.startDue = true; this.dueWhy = why; this.count(why); }
  /** RH_TRACE=<file>: a line a game as the process ends (who stood it, how far it stepped, what it took), for a table of what a tool does with its games */
  rec: any = null;

  /** the puppet exported whole and held to the Rust game's whole state, the fields in which they differ added to this game's tally (the host's own half
   *  and the bridge's serials left out). A whole apply puts every field back, so the look is taken every `RH_STALE_EVERY` ticks and not only at the end */
  private staleT = new Map<string, { n: number; looks: number; eg: string }>(); private staleLooks = 0;
  staleLook(): void { if (!this.h || this.audit || this.startDue || !this.k || this.inApply) return; const X = this.F.X, B = this.B, A = B.A; X.face_export(this.h); const rust = this.F.outView().slice(), mine: Uint8Array = B.exportGame(this.g, B.reg), seen = new Set<string>(); this.staleLooks++;
    for (const x of A.fieldsDiffering(rust, mine, [], 400000) as { q: string; path: string }[]) { if (A.HOST_Q(x) || A.OWN_Q(x)) continue; let e = this.staleT.get(x.q); if (!e) this.staleT.set(x.q, e = { n: 0, looks: 0, eg: x.path }); e.n++; if (!seen.has(x.q)) { seen.add(x.q); e.looks++; } } }
  staleSaid(): string { const tool = `${(process.argv[1] ?? '').split('/').pop()} ${process.argv.slice(2).filter(a => !a.includes('/')).join(' ').slice(0, 80)}`; this.staleLook();
    if (!this.staleLooks) return `# game\t${tool}\tticks ${this.k}\tnot held: ${!this.k ? 'never stepped' : 'no Rust game stood for it at a look'}\n`;
    return `# game\t${tool}\tticks ${this.k}\t${this.staleLooks} looks\n` + [...this.staleT].map(([q, e]) => `${q}\t${e.n}\t${e.eg}\t${e.looks}\n`).join(''); }
  private count(name: string) { this.calls.set(name, (this.calls.get(name) ?? 0) + 1); }
  private tsBlock(): Uint8Array { const w: Uint32Array = this.g.sim.w; return new Uint8Array(w.buffer, w.byteOffset, w.byteLength); }
  rustBlock(): Uint8Array { const X = this.F.X, at = X.state_addr(this.slot) >>> 0; return this.F.u8().subarray(at, at + X.state_len()); }
  time(): number { return this.F.X.face_time(this.h); }
  dieN(): number { return this.F.X.face_die_n(this.h); }
  /** the whole game's words, one a part, with what a host hands in left out (face_state_words; src/net/step.ts netWord folds them on a word tick), and the parts' names. null / none on a module that has not got them */
  wholeWords(): Uint32Array | null { const X = this.F.X; if (!X.face_state_words || !this.h) return null; X.face_state_words(this.h); return new Uint32Array(this.F.outView().slice().buffer); }
  private stateNames: string[] | null = null;
  wholeNames(): string[] { const X = this.F.X; if (!X.face_state_n) return []; if (!this.stateNames) { this.stateNames = []; for (let i = 0; i < X.face_state_n(); i++) { X.face_state_name(i); this.stateNames.push(this.F.outText()); } } return this.stateNames; }
  /** WHAT A HUD IS TOLD, asked of the Rust game as it stands, with whatever the tool has written since the tick sent first (src/view/seatview.ts: the page draws
   *  the same record). Blank while no Rust game of this host stands: before the first tick, with a new start due, after this game's slot was given up */
  view(): any { if (!this.h || this.startDue || this.audit) return NO_VIEW; if (!this.inApply && !this.inStep && !this.quietOn && this.busy === 0) this.scanFlush(); this.tellShown();
    try { this.F.X.face_tick_view(this.h); } catch (e) { throw new Error(`the Rust game's view panicked: ${this.F.panicText() || String(e)}`); } return this.B.readTickView(this.F.outView().slice()); }
  /** WHAT THE CURSOR IS ON, AND WHAT A POINT IS (src/view/cursor.ts), asked of the Rust game, with whatever the tool has written since sent first (a tool moves men and doors between asks) */
  cursor(input: any, cam: any, canvas: any, seat = -1): any { if (!this.pointed()) return { under: null }; const N = new Float64Array(this.B.CURSOR_DOUBLES); this.B.sampleCursor(N, input, cam, canvas);
    try { this.F.hand(new Uint8Array(N.buffer), (p: number) => this.F.X.face_cursor_view(this.h, seat, p)); } catch (e) { throw new Error(`the Rust game's cursor panicked: ${this.F.panicText() || String(e)}`); } return this.B.readCursorView(this.F.outView().slice()); }
  under(p: any, seat = -1): any { if (!this.pointed()) return null;
    try { this.F.X.face_under(this.h, seat, p[0], p[1], p[2]); } catch (e) { throw new Error(`the Rust game's point panicked: ${this.F.panicText() || String(e)}`); } return this.B.readUnder(this.F.outView().slice()); }
  /** a question of a point may be asked: the Rust game stands (a tool that asks before its first tick starts it, as a call that crosses does), and has what the tool wrote */
  private pointed(): boolean { if (this.audit) return false; if (!this.h && !this.surfaceStart()) { if (this.starts > 0 && !this.startDue) throw new Error(`rust-host: what a point or the cursor is on is asked of a game that gave its slot up to another. Asked by ${toolLine()}`); return false; } if (this.startDue) return false; if (!this.inApply && !this.inStep && !this.quietOn && this.busy === 0) this.scanFlush(); this.tellShown(); return true; }
  /** WHAT THE END CARD SHOWS OF THE RUN, asked of the Rust game as it stands (src/view/debriefview.ts: the page draws the same record) */
  debriefView(): any { if (!this.h || this.startDue || this.audit) return null; if (!this.inApply && !this.inStep && !this.quietOn && this.busy === 0) this.scanFlush(); this.tellShown();
    try { this.F.X.face_debrief_view(this.h); } catch (e) { throw new Error(`the Rust game's debrief panicked: ${this.F.panicText() || String(e)}`); } return this.B.readDebriefView(this.F.outView().slice()); }
  /** THE BOX WORLD ASKED of the Rust game (src/view/worldask.ts), with whatever the tool has written since the tick sent first; null while no Rust game of this host stands */
  ask(): any { if (!this.h || this.startDue || this.audit || this.inApply) return null; const X = this.F.X, out = (n: number) => new Float64Array(this.F.outView().slice().buffer, 0, n);
    const ask = <T>(f: () => T): T => { if (this.askedAt !== this.stamp && !this.inStep && !this.quietOn && this.busy === 0) { this.askedAt = this.stamp; this.scanFlush(); } try { return f(); } catch (e) { throw new Error(`a question of the Rust game's boxes panicked: ${this.F.panicText() || String(e)}`); } };   /* (a tool's writes are sent once a step: a walk a question would be a walk a line) */
    return { blocked: (a: any, b: any) => ask(() => X.face_line(this.h, 0, a[0], a[1], a[2], b[0], b[1], b[2]) !== 0), solid: (a: any, b: any) => ask(() => X.face_line(this.h, 1, a[0], a[1], a[2], b[0], b[1], b[2]) !== 0),
      ray: (o: any, d: any, max: number) => ask(() => { if (!X.face_ray(this.h, o[0], o[1], o[2], d[0], d[1], d[2], max)) return null; const r = out(5); return { t: r[0], n: [r[1], r[2], r[3]] }; }),
      hear: (s: any, d: any) => ask(() => { X.face_hear(this.h, s[0], s[1], s[2], d[0], d[1], d[2]); return this.B.heardOf(out(10)); }), together: (f: any) => f() }; }
  private askedAt = -1;
  /** how the mission ended, off the record the Rust game's own tick latches (`face_verdict`: null while it is played). For a host with no puppet, and
   *  for holding the puppet's own verdict to it. `lines` is the whole record as the module writes it (game::verdict::verdict_lines). Hung as `verdictNow` */
  verdict(): { won: boolean; word: string; why: string; lines: string[] } | null { const r = this.F.X.face_verdict(this.h); if (!r) return null; const L = this.F.outText().split('\n'); return { won: r === 2, word: L[0], why: L[1], lines: L }; }
  /** THE GAME'S OWN DIE PUT AT THE START OF `seed`'S STREAM, ON THE RUST SIDE (tools/qa/headless.ts `seedGame`, which has put the puppet's: a tool that wants a
   *  given stream after the game is stood seeds the game's die; nothing lets the Rust tick draw a tool's `Math.random`). The die is the `dice` part of the
   *  game's block (rust/sim/src/dice.rs: the count of draws as a double, then the mulberry32 word), written in place.
   *  STAND-IN: the host writes the block by its layout (the part found by its name, the two fields by their offsets). Belongs in the face: one call
   *  that seeds the die of the game in a slot.
   *  Before the first tick there is no Rust game yet, and its start takes the puppet's die as it stands. */
  seedDie(seed: number): void { this.dieSeeds++; if (this.rec) this.rec.dieSeeds = this.dieSeeds; if (!this.h) return;
    const X = this.F.X, i = this.F.NAMES.indexOf('dice'); if (i < 0) throw new Error('rust-host: the block has no part named dice');
    const put = (buf: ArrayBufferLike, at: number) => { const v = new DataView(buf); v.setFloat64(at, 0, true); v.setUint32(at + 8, seed >>> 0, true); };
    put(this.F.mem.buffer, X.face_part_addr(this.slot, i) >>> 0);
    { const at = X.face_part_addr(this.slot, i) >>> 0; this.F.rec?.mem(at, new Uint8Array(this.F.mem.buffer, at, 12)); }
    const w: Uint32Array = this.g.sim.w; put(w.buffer, w.byteOffset + (X.face_part_addr(0, i) >>> 0) - (X.state_addr(0) >>> 0));   // (the puppet's copy of the block, as the next apply would leave it)
    this.dieRef = this.g.dice?.n; }
  dieSeeds = 0;
  /** the clock at this game's first step and at its last one's opening (RH_TIMES: what a tick costs all in, the tool's own code between two steps included) */
  wall0 = 0; wallLast = 0;
  hashes(): number[] { return this.F.partHashes(this.slot); }

  // ---- a start: the puppet as it stands is the game. The first tick's; and again wherever the TypeScript round a step has put another game in the
  // puppet's place (a checkpoint put back, a restart): no stepper can follow that from its own state.
  /** the host's own lines: what they call runs on the puppet, and starts nothing */
  private start(why = 'the first step'): void { const was = this.starting; this.starting = true; try { this.startBody(why); } finally { this.starting = was; } }
  private startBody(why: string): void {
    RustHost.standing = false;
    const g = this.g, X = this.F.X, B = this.B, t = now();
    // (iii) A GAME WHOSE KEPT BYTES WERE LET GO (it gave its slot up more than a few games ago) IS NOT STEPPED AGAIN: its puppet is what the last whole
    // apply left, and a start from it would be another game than the one that stood
    if (why === 'refused') throw new Error(`rust-host: this game gave its slot up to another and its kept bytes have been let go since (the last ${OFF_CAP} are kept): its re-step is refused. Asked by ${toolLine()}`);
    // (iv) A NEW START FROM A PUPPET THAT HOLDS A MAN THE APPLY MADE is refused: a man the Rust game made (a response team, a restart made on the Rust side)
    // has no store row and no pose store on the TypeScript side, and the walk of the puppet throws deep inside (`h.f32`, skeleton.ts) or exports a man with no body
    if (this.starts > 0 && !OFF.has('made')) { const men: string[] = [], look = (o: any, who: string) => { if (o && (MADE_MEN.has(o) || (o.char && MADE_MEN.has(o.char)))) men.push(who); };
      for (const x of (g.guards ?? []) as any[]) look(x, x.callsign ?? x.name ?? 'a man'); for (const s of (g.seats ?? []) as any[]) look(s.player ?? s.own?.player, 'a seat\'s player'); look(g.player, 'the player');
      if (men.length) throw new Error(`rust-host: a new start is asked for (${why}) from a puppet that holds ${men.length} men the apply made (${[...new Set(men)].slice(0, 6).join(', ')}): a man the Rust game made has no store row or pose store on the TypeScript side, so this puppet cannot be the start of a game. Asked by ${toolLine()}`); }
    this.startDue = false; this.dueWhy = ''; this.unkeepAll(); if (this.keptOff > 0) { X.face_kept_drop(this.keptOff); const i = OFF_KEPT.indexOf(this.cell); if (i >= 0) OFF_KEPT.splice(i, 1); } this.keptOff = 0; this.made = {};
    this.B.surfaceHost(this); this.taps(g);   // (the tool may have started another mission since the game was stood: its menu, its fight and its saves are new objects)
    g.sim.settle(g);
    const start: Uint8Array = this.B.startExport(g, this.B.reg, (w: string) => this.count(w));
    const block = this.tsBlock().slice();
    if (this.h) { X.face_puppet_drop?.(this.h); X.face_drop(this.h); }
    // THE SLOTS (rust/sim has four): a tool that stands games back to back in one process (the probe runner: hundreds) never says it has done with one.
    // Where every slot is taken, the game stepped longest ago gives its slot up (`make`): its puppet takes the whole state and its bytes are kept, so
    // that if the tool does step it again it stands again as it was (`restep`).
    this.make(start, block); { const said = this.F.outText(); this.seated(); if (said && !this.o.quiet) this.say(said); }
    if (X.state_len() !== block.length) this.say(`rust-host: rust/sim's block is ${X.state_len()} bytes in the module and ${block.length} in the TypeScript game`);
    // (the start's own bytes, 10 MB and more, are read again only by the audit and by the step's warm-up: the fast way lets them go, or each of the four
    // games that stand in a slot would hold its start for as long as it stands there)
    { const keep = this.audit || (this.o.warmSteps ?? 0) > 0; this.lastState = keep ? start : null; this.lastBlock = keep ? block : null; this.lastRust = keep ? start : null; }
    this.starts++; if (this.rec) this.rec.starts = this.starts; this.warmDue = this.starts === 1; this.shelfSig = ''; this.scan = null; this.scanStart = this.starts;
    // THE MENU'S QUESTIONS: every order row and the squad's posture ask the Rust game by the row's id (ask.ts, `face_ask`); a row's `issue` gives the order
    // on the Rust side and the puppet takes the state again
    if (this.starts === 1 && this.o.asks !== false && X.face_ask) this.asks = B.installAsks(g, B.reg, (kind: number, row: number, seat: number, b: Uint8Array) => { this.asked++; const one = (r: number): number => this.F.hand(b, (p, n) => X.face_ask(this.h, kind, r, seat, p, n)) | 0; let no = one(row);
      // (`face_ask` answers a number: 0 answered; 1 the Rust game has no such order row; 2 no such seat; 3 no such question. A row object a tool's own call
      // of the menu made asks by the Rust game's row of the same id where it has one; a refused ask reads as NOTHING, never as empty bytes: ask.ts answers it.)
      if (no === 1) { const L = B.bookOf(g)?.listOf('OrdersOrderDefId') as any[] | undefined, id = L?.[row]?.id, twin = id === undefined || !L ? -1 : L.findIndex((x, i) => i !== row && x?.id === id); if (twin >= 0 && one(twin) === 0) { this.count('an ask by the Rust game\'s row of the same id (the puppet\'s row is one a tool\'s call made)'); return this.F.outView().slice(); } }
      if (no) { this.asksRefused++; this.count(`AN ASK THE RUST GAME REFUSED (${ASKS[kind] ?? kind}: ${no === 1 ? `no order row ${row}` : no === 2 ? `no seat ${seat}` : `no question ${kind}`}): answered as nothing`); return no; }
      return this.F.outView().slice(); }, () => { this.reapply('an order row issued'); });
    // THE MEMO OF THE FILTERED EXPORT is filled by a start's export. Its bytes are not applied: the puppet IS that state (the Rust game was made from it).
    // The every-tick path is then run over every row a few times, so that the first ticks do not pay for its compiling: `face_puppet_warm` sends each
    // row as if it had changed, and applying a row that holds what the puppet holds changes nothing.
    this.tellShown(); X.face_puppet(this.h, 1);
    const warm = this.o.warm ?? 4; if (X.face_puppet_warm) for (let i = 0; i < warm; i++) { X.face_puppet_warm(this.h); const R = B.applyGame(g, this.F.outView(), B.reg, { hooks: B.hooks, puppet: { ...this.pup, start: false } }); this.after(R); }
    this.visApplied = g.player?.visibility; this.hoverApplied = g.hover; this.refs();
    this.memPages = this.F.mem.buffer.byteLength; this.T.start += now() - t;
    if (!this.inStep) this.scanFlush();   // (a start made outside a step, for a save: the walk's base is the puppet as this start found it)
    if (!this.o.quiet) this.say(`rust-host: ${this.starts === 1 ? 'the start' : `A NEW START as tick ${this.k} opens (${why})`}: ${start.length} bytes, the block ${block.length}; slot ${this.slot}; memory ${(this.F.mem.buffer.byteLength / 1048576).toFixed(0)} MB`);
  }
  /** what the scripted player's `ended()` reads (playthrough.ts:733), the mission's stage, and how many door orders are under way: when one ends the
   *  scripted commander asks the door tools what the team knows of that door (playthrough.ts:998 `knownShut` → doortools.ts:1019 `knows`, the tools'
   *  `tried`, which is not on the filtered list: warehouse-four's and barracks-four's second `bangClear` was not given without it) */
  private endMark(): string { const g = this.g; return `${g.mission?.stage}|${g.endCards}|${g.roomClear?.end}|${g.rescue?.result?.state}|${OFF.has('doors') ? 0 : g.doorTools?.ops?.length}`; }
  private downCount(): number { const g = this.g; let n = g.player?.down ? 1 : 0; for (const x of g.guards as any[]) if (x.state === 'dead' || x.down) n++; return n; }

  // ---- the applies
  private took(R: any): void {
    for (const [q, n] of R.noWay) this.noWay.set(q, (this.noWay.get(q) ?? 0) + n); for (const [q, n] of R.noSetter) this.noSetter.set(q, (this.noSetter.get(q) ?? 0) + n); for (const q of R.noWindow) this.noWindow.add(q);
    for (const [c, n] of Object.entries<number>(R.made)) if (n) this.made[c] = (this.made[c] ?? 0) + n; }
  /** after every apply: the rows and the posture the apply made get the closures that ask the Rust game (ask.ts) */
  // STAND-IN: THE HIGHLIGHTED ROW IS ONE OF THE NEARBY LIST'S ROWS in the TypeScript (player.ts pickHover: `g.hover = rows[pl.nearSel]`), and a reader
  // holds it to the list by identity (the soak's `selector-highlight`; the scripted player's own highlight). The bridge carries `Game.hover` as a value,
  // so the apply leaves a row of its own beside the list. Put back on the list's row here. Belongs in the schema: `hover` a reference into `nearby`.
  private hoverRow(): void { const g = this.g, hv = g.hover, N = g.nearby; if (!hv || !Array.isArray(N) || N.includes(hv) || OFF.has('hoverrow')) return; const r = N[g.player?.nearSel ?? 0]; if (r && r.kind === hv.kind && r.verb === hv.verb) g.hover = r; }
  /** WHAT THE PICTURE IS TOLD BY SECTION (src/draw): the fog's, as the page he plays asks */
  get drawn(): Drawn { return this.drawnOf ??= new Drawn(this.F.X, this.F.mem, Want.fog); } private drawnOf: Drawn | null = null;
  private after(R: any): void { this.drawn.take(this.h); fogFrom(this.g.fogOfWar.frame, this.drawn); this.asks?.after(); this.took(R); this.hoverRow(); this.visApplied = this.g.player?.visibility; this.hoverApplied = this.g.hover; this.dieRef = this.g.dice?.n; this.clockRef = this.clock(); this.putRef = this.B.PUT_BACKS.n; if (!this.startDue) this.loadsRef = this.g.chronicle?.counts?.loads ?? 0; }
  private copyBlock(): void { const t = now(), mine = this.tsBlock(), theirs = this.rustBlock(); mine.set(theirs.subarray(0, mine.length)); if (this.audit) this.lastBlock = mine.slice(); this.T.block += now() - t; }
  /** THE FILTERED APPLY: the listed fields of what changed since the last one, and the step's out events */
  applyStep(): void {
    const g = this.g, X = this.F.X, B = this.B, t0 = now(), made0 = MADE.n; let t = t0; this.mBefore = g.mission;
    this.tellShown(); X.face_puppet(this.h, 0); const blob = this.F.outView(); this.stepBytes += blob.length; this.T.exportStep += now() - t;
    t = now(); const R = B.applyGame(g, blob, B.reg, { hooks: B.hooks, puppet: { ...this.pup, start: false } }); const el = now() - t; this.T.applyStep += el; if (el > this.worstApply) this.worstApply = el; this.missionRef = g.mission;   // (a mission the Rust game wrote is no start of a new game: only one a tool wrote behind it is)
    this.lastChanged = R.changed !== false || !!R.out?.length; if (R.out?.length) for (const e of R.out) this.outQ.push(e);
    this.after(R); this.copyBlock(); this.steps++; this.nest += now() - t0;
    // (STAND-IN, belongs in the importer (a made row is filled whole): A MAN THE APPLY MADE, the Rust game having restarted or spawned (inside its tick, or
    // by a call that crossed), holds only the fields of the every-tick list: his rifle's rows, his kit, read `undefined` to whatever reads the puppet
    // next. The puppet takes the whole state at once.)
    this.newRun();   // (before the whole apply below, which takes the record as it finds it)
    if (MADE.n !== made0 && !this.audit) this.applyWhole('the apply made a man');
    this.newClear();
  }
  // STAND-IN (belongs in the schema: `Game.clearing` a row with an ID, or a serial the importer reads): A ROOM CLEAR ENDED AND ANOTHER BEGAN INSIDE ONE
  // TICK. The TypeScript makes a new `Clearing` (squad.ts) and a reader knows a clear began by that identity (tools/qa/soak.ts:1760
  // `game.clearing !== last`: its respite and its one scheduled interrupt hang on it); the bridge carries the clear as a value under `Game.clearing`,
  // so the apply wrote the new clear's fields into the object that stood. A clear whose own clock (`total`) went back is another clear: the puppet is
  // given a new object of the same class.
  private clearRef: any = null; private clearT = 0;
  private newClear(): void { const g = this.g, c = g.clearing;
    if (c && c === this.clearRef && typeof c.total === 'number' && c.total < this.clearT - 1e-9 && !OFF.has('newclear')) { g.clearing = Object.assign(Object.create(Object.getPrototypeOf(c)), c); this.count('a room clear began as another ended: the puppet\'s record made anew'); }
    this.clearRef = g.clearing; this.clearT = typeof g.clearing?.total === 'number' ? g.clearing.total : 0; }
  // STAND-IN (belongs in the importer: the record as a row, or a restart's out event): THE RUST GAME BEGAN ITS MISSION AGAIN (the player's own restart
  // inside the tick, Enter once down or Shift+Enter; a `restartEncounter` that crossed). The TypeScript's restart makes a new mission RECORD, and that
  // identity is how everything outside the tick knows a run began (the save controller's `missionRef`: a fresh run, an empty shelf; a driver's own
  // `game.mission !== before`). The apply wrote the fields into the record that stood, so the puppet is given a new one here.
  // (since the importer makes a new record itself where the Rust game dealt a new serial, this only fires for a bridge that did not; and a record the
  // APPLY replaced is the Rust game's own doing, not a game the TypeScript put in the puppet's place: the host's reference follows it)
  private mBefore: any = null;
  private newRun(): void { const g = this.g, m = g.mission;
    if (m !== this.mBefore) { this.missionRef = m; this.t0Ref = m?.t0; this.mBefore = m; return; }   // (the apply made the record anew: followed, with its clock's start)
    if (!m || m !== this.missionRef || m.t0 === this.t0Ref || OFF.has('newrun')) return;
    g.mission = { ...m, stats: { ...m.stats } }; this.missionRef = g.mission; this.t0Ref = m.t0; this.count('the Rust game began its mission again: the puppet\'s mission record made anew'); }
  /** THE WHOLE APPLY: every field the bridge carries (a field the Rust game owns and TypeScript cannot hold, the box mirror among them, is kept beside the
   *  object as an echo: tools/blitz/IMPORT.md) */
  applyWhole(why: string): Uint8Array { const r = this.applyWholeNow(why); this.newRun(); this.newClear(); this.scanRebase(); return r; }
  private applyWholeNow(why: string): Uint8Array {
    const g = this.g, X = this.F.X, B = this.B, t0 = now(); let t = t0;
    // (BETWEEN TWO TICKS the host has moved its own camera and Input on since the Rust game was last handed them (headless.ts `cam.update` after
    // `game.update`); the whole state would put them back a frame. depot-four parted so at tick 1578, a frame after a room was flooded. The host's rows
    // are handed over first, as a tick's opening hands them.)
    if (this.hostNow) this.handHalf();
    // (a copy out of the module: a whole apply keeps the bytes of the fields the Rust game owns beside their objects, as places in this very blob (the
    // bridge's echo), and the module's memory is written again by the next call)
    this.mBefore = g.mission; X.face_export(this.h); const blob = this.F.outView().slice(); this.wholeBytes = blob.length; this.T.exportWhole += now() - t;
    t = now(); const R = B.applyGame(g, blob, B.reg, { hooks: B.hooks }); const el = now() - t; this.T.applyWhole += el; if (el > this.worstApply) this.worstApply = el;
    this.after(R); this.copyBlock();
    this.wholes.set(why, (this.wholes.get(why) ?? 0) + 1); this.wholeNow = true; if (this.audit) this.lastRust = blob; this.nest += now() - t0;
    return blob; }
  /** BETWEEN TWO TICKS: the host's rows (each seat's record, the cameras, the Input) as they stand on the puppet NOW go to the Rust game, as a tick's opening hands them */
  private hostNow = true;
  private handHalf(): void { if (this.inStep || !this.lastCmds || !this.h) return; this.tellShown(); const hb: Uint8Array = this.B.hostHalf(this.g, this.B.reg, this.lastCmds); this.F.hand(hb, (p, n) => this.F.X.face_host_half(this.h, p, n)); this.viewLive(); }
  /** after a call that changed the Rust game between two ticks: the puppet takes the state again */
  private reapply(why: string): void { if (this.audit) { this.applyStep(); this.lastState = this.applyWhole(why); } else this.applyStep(); this.scanRebase(); }

  // ---- the taps: what the TypeScript round a step calls that WRITES the game and is no input
  private taps(g: any): void {
    const H = this, X = this.F.X, B = this.B, F = this.F;
    // THE SCRIPTED PLAYER'S ORDERS. With the menu's questions installed (ask.ts) `orders.give` runs as it is on the puppet: every closure of a row it calls
    // asks the Rust game, and the row's `issue` gives the order there and has the puppet take the state again. The call is written down in the replay
    // loop's own words for a check to hold to the plain play's. Without them (`asks: false`) the order is given on the Rust side at once by `face_give`
    // where the face has words for its target, and runs on the puppet where it has none.
    const O = g.orders, give = O.give; if (O && !TAPPED.has(O)) { TAPPED.add(O); hang(O, 'give', function (this: any, id: string, t: any, who: any, opts: any) {
      if (!H.live()) return give.call(this, id, t, who, opts);
      const T0 = t ?? this.targetAt(g.aimPoint), w = who === undefined ? '-' : typeof who === 'object' ? `g${g.guards.indexOf(who)}` : String(who);
      const tail = T0.kind === 'door' ? ` door ${g.doors.list.indexOf(T0.door)} ${T0.side} book${B.bookOf(g).ids.get('DoorId')?.get(T0.door)}` : T0.kind === 'none' && T0.man ? ` man ${g.guards.indexOf(T0.man)}` : T0.kind === 'foe' ? ` man ${g.guards.indexOf(T0.foe)}` : (T0.wall || T0.thing) ? ' UNSUPPORTED-wall-or-thing' : '';
      const line = `${H.k + 1} ${id} ${w} ${T0.kind} ${T0.at.join(' ')}${tail}${opts ? ' UNSUPPORTED-opts' : ''}`, noWords = line.includes('UNSUPPORTED') || /^g\d/.test(w);
      H.gave.push(line); const t0 = now(), n0 = H.nest; H.handWrites(H.k + 1); H.scanFlush();   // (what the driver wrote by name before this order goes first: the driver's own order)
      if (OFF.has('tapgive')) { H.count(`orders.give(${id}) by the call surface`); const r = give.call(this, id, t, who, opts); H.T.calls += now() - t0 - (H.nest - n0); if (H.o.out) appendFileSync(`${H.o.out}/gives.txt`, `${line}\n   -> ${JSON.stringify(r)}\n`); return r; }
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
      if (H.live() && !H.audit && X.face_room_at) { const t0 = now(), n0 = H.nest; H.scanFlush(); if (X.face_room_at(H.h, p[0], p[1], p[2])) { H.wrote.push(`${H.k + 1} room_at ${p[0]} ${p[1]} ${p[2]}`); H.count('fight.roomAt (the Rust game flooded a new room; the puppet took the whole state)'); H.applyWhole('a room flooded'); } H.T.calls += now() - t0 - (H.nest - n0); }
      const n0 = Fi.rooms.length, r = ask.call(this, p); if (Fi.rooms.length !== n0) { if (H.audit) H.wrote.push(`${H.k + 1} room_at ${p[0]} ${p[1]} ${p[2]}`); H.count(H.audit ? 'fight.roomAt (flooded a new room on the puppet)' : 'fight.roomAt FLOODED A ROOM ON THE PUPPET THAT THE RUST GAME DID NOT'); } return r; }); }
    // (a lesson's player picks a row of the nearby list between ticks and reads the answer at once: made on the Rust side, as an order is)
    const wordAt = (ty: string, w: unknown) => w === undefined ? -1 : B.SCHEMA.types[ty].words.indexOf(w);
    const pick = g.selectVerb; if (!TAPPED.has(g) && !OFF.has('tapsel')) { TAPPED.add(g); hang(g, 'selectVerb', function (this: any, kind: string, verb?: string) {
      if (!H.live()) return pick.call(this, kind, verb);
      H.count('game.selectVerb'); const t0 = now(), n0 = H.nest; H.handWrites(H.k + 1); H.scanFlush();
      const line = `${H.k + 1} select_verb ${wordAt('crate::game::InteractableKind', kind)} ${wordAt('crate::game::Verb', verb)} ${kind} ${verb}`; H.wrote.push(line);
      const r = F.hand(enc.encode(line), (p, n) => X.face_hostwrite(H.h, H.k + 1, p, n)), said = F.outText();
      if (r !== 1) H.say(`rust-host: the face did not take selectVerb(${kind}, ${verb})`);
      H.reapply('a pick of a verb'); H.T.calls += now() - t0 - (H.nest - n0); return / -> true\s*$/.test(said); }); }
    // (the scripted player's word to the hostages is the module's own call between two ticks: made on the Rust side, as a pick is.
    //  The puppet's own body does not run: what it would write the apply brings.)
    for (const [name, E] of [['rescue_word', g.rescue?.ending]] as [string, any][]) if (E?.word && !TAPPED.has(E) && !OFF.has('tapword')) { const word = E.word; TAPPED.add(E); hang(E, 'word', function (this: any, by: any) {
      if (!H.live() || g.ticking) return word.call(this, by);
      const who = by === g.player.char ? 'player' : by ? null : 'none'; if (who === null) { H.say(`rust-host: ${name} for a man the face has no word for; run on the puppet`); return word.call(this, by); }
      H.count(`${name} (the module's own word, made on the Rust side)`); const t0 = now(), n0 = H.nest; H.handWrites(H.k + 1); H.scanFlush();
      const line = `${H.k + 1} ${name} ${who}`; H.wrote.push(line);
      const r = F.hand(enc.encode(line), (p, n) => X.face_hostwrite(H.h, H.k + 1, p, n)), said = F.outText();
      if (r !== 1) H.say(`rust-host: the face did not take ${name}(${who})`);
      H.reapply('a word to follow'); H.T.calls += now() - t0 - (H.nest - n0); return / -> true\s*$/.test(said); }); }
    // THE HOST'S STEPS ROUND A TICK THAT THE RUST SIDE MAKES TOO.
    // The lights' update: made on the Rust side after every step. Audit, and a fast tick that took the whole state: the puppet already holds what it
    // left, and run again in TypeScript a flash's envelope would be stepped twice. A fast tick that took only the filtered state: no lamp is on that list,
    // so the puppet's own lamps are stepped by the TypeScript's own update, as the Rust game's are by its own.
    const E = g.engine, L = E.lights, W = E.world, lightsUpdate = L.update;
    if (!TAPPED.has(L)) { TAPPED.add(L); hang(L, 'update', function (this: any, ...a: any[]) { if (!H.live() || (!H.audit && !H.wholeNow)) return lightsUpdate.apply(this, a); H.count('engine.lights.update (not run on the puppet: it holds what the Rust side\'s left)'); }); }
    // The box world's listing. The Rust game lists its own as a tick opens. Audit: the puppet holds that listed world. Fast: the list of every box and the ray grid
    // are not on the filtered list (they are the TypeScript's own there), so the TypeScript's listing runs as it is.
    const upload = W.upload; this.tsUpload = () => { W.upload(); };
    if (!TAPPED.has(W)) { TAPPED.add(W); hang(W, 'upload', function (this: any, ...a: any[]) {
      if (!H.live()) return upload.apply(this, a);
      if (!H.audit) { if (H.quietOn) return;
        // STAND-IN: A BOX WHOSE MARKINGS WENT is `marks: null` after an apply (the importer writes a None as null), and the listing tests
        // `b.marks !== undefined` (scene/boxes.ts:841) and then reads its length: it threw at tick 1817 of tut-solo. Belongs in import.ts: the None of a
        // field the TypeScript declares optional is `undefined`.
        for (const L of [this.statics, this.dynamics, g.dynBoxes] as any[][]) if (!OFF.has('marks')) for (const b of L) if (b.marks === null) { b.marks = undefined; H.standIns.marks++; }
        return upload.apply(this, a); }
    }); }
    // THE SAVE CONTROLLER KEEPS THE PUPPET (a save in memory is the TypeScript objects as they stand; a file is their serialisation), so a save taken off
    // a puppet that holds only the filtered state would keep fields that stopped moving ticks ago. Before a take the puppet takes the whole state.
    // WITH KEPT GAMES (`keeps`): a save is the Rust game's own clone, so no whole apply is made for one; while the controller runs, every save is one in
    // memory (`SNAPS.own`, as on the page) and nothing of its lines crosses the call surface (they are the host's own, made on the Rust side by
    // `face_checkpoint` and `face_loaded`); as its outermost call ends between two ticks the puppet takes what those made (`settle`).
    const S = g.saves; if (S && !TAPPED.has(S)) { TAPPED.add(S); const P = Object.getPrototypeOf(S);
      for (const name of Object.getOwnPropertyNames(P)) { const d = Object.getOwnPropertyDescriptor(P, name); if (name === 'constructor' || !d || typeof d.value !== 'function') continue; const was = d.value;
        hang(S, name, function (this: any, ...a: any[]) {
          if (!H.keeps()) { if ((name === 'write' || name === 'writeMem') && H.h && !H.audit && !H.wholeNow) H.applyWhole('a save taken'); return was.apply(this, a); }
          // (the slot 'harness' is a file of the puppet's own fields, handed back to the tool: the puppet takes the whole state first)
          if (name === 'write' && a[1] === 'harness' && H.live() && !H.wholeNow && H.ctl === 0) H.applyWhole('a file of the puppet\'s fields is made');
          OWN++; H.busy++; H.ctl++; try { return was.apply(this, a); } finally { OWN--; H.busy--; H.ctl--; if (H.ctl === 0 && !H.inStep && H.settle()) H.scanRebase(); } }); } }
    // A FILE LOADED ON THE PUPPET (savestate.ts `loadState`, called by a tool with a file it holds: no seam names it). It begins by restarting the
    // encounter and then pours the file into the puppet's objects. `Game.restartEncounter` is on the call surface: crossing, the Rust game would restart
    // and make its men anew, and the file would then be poured over rows the apply made. So a restart asked from inside `loadState` hands the puppet
    // the game: the Rust game is let go, the TypeScript runs whole, and the next step starts a new Rust game from the puppet.
    // STAND-IN: known by the caller's name on the stack. Belongs on the call surface: `loadState` a row the host answers.
    { const was = g.restartEncounter; if (typeof was === 'function' && !Object.prototype.hasOwnProperty.call(g, 'restartEncounter')) hang(g, 'restartEncounter', function (this: any, ...a: any[]) {
      if (H.live() && !H.inStep && H.busy === 0 && !H.audit && !OFF.has('load') && /\bloadState\b/.test(String(new Error().stack ?? ''))) H.puppetTakesOver(`a file is loaded on the puppet by the TypeScript (loadState, from ${toolLine()}): the Rust game is let go until the next step`);
      return was.apply(this, a); }); }
  }

  // ---- what TypeScript wrote into the puppet since the last apply goes to the Rust game. `at`: the tick that opens next.
  /** fast: the writes that have a call. The lesson's player feeds the light meter before every tick (lesson-run.ts:228) */
  private handWrites(at: number): void {
    if (this.audit) { this.writeBack(at); return; }
    const g = this.g, v = g.player?.visibility;
    if (v !== this.visApplied && typeof v === 'number' && !OFF.has('meter')) { this.F.X.face_meter(this.h, v); this.visApplied = v; this.wb.meter++; }
    // (the scripted player put the highlight on a row itself: it writes `player.nearSel` and calls pickHover, which hands `g.hover` a row of the
    //  puppet's own list. The highlight's number is not on the filtered list, so the pick is known by the hover: another object than the apply left,
    //  and one of the list's rows. The row's number goes to the Rust game, whose own pickHover takes it.)
    const hv = g.hover, row = hv && hv !== this.hoverApplied ? (g.nearby ?? []).indexOf(hv) : -1;
    if (row >= 0 && !OFF.has('hover')) { const line = `${at} near_sel ${row}`; this.wrote.push(line); if (this.F.hand(enc.encode(line), (p, n) => this.F.X.face_hostwrite(this.h, at, p, n)) !== 1) this.say(`rust-host: the face did not take the highlight on row ${row}`); this.count('the highlight put on a row by the tool (made on the Rust side)'); }
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
    if (blockOwed) { this.wb.block++; const a = this.lastBlock!; for (let i = 0; i < this.F.NAMES.length; i++) { const off = X.face_part_addr(0, i) - X.state_addr(0), len = X.face_part_len(i); if (!A.buf(blk.subarray(off, off + len)).equals(A.buf(a.subarray(off, off + len)))) this.BLOCKW.set(this.F.NAMES[i], (this.BLOCKW.get(this.F.NAMES[i]) ?? 0) + 1); } { const b = this.rustBlock(), u = blk.subarray(0, X.state_len()); b.set(u); this.F.rec?.mem(b.byteOffset, u); } this.lastBlock = blk.slice(); }
    this.T.wbImport += now() - t1; this.openState = mine; this.lastState = mine;
  }

  // ---- the out events of a step, on the puppet's host objects, in the order the rules made them
  private dispatch(): void { this.dispatching = true; try { this.dispatchQ(); } finally { this.dispatching = false; } }
  private scanStart = -1;
  private dispatchQ(): void {
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
    if (this.startThrew) { const e = this.startThrew; this.startThrew = null; throw e; }
    RustHost.standing = false; this.stamp = ++RustHost.clock; if (RustHost.last !== this && !OFF.has('waiting')) RustHost.startWaiting(this);
    const g = this.g, X = this.F.X, B = this.B, k = ++this.k; let t = now();
    if (!this.wall0) this.wall0 = t; this.wallLast = t;
    this.wholeNow = false; this.inStep = true; this.lastCmds = cmds; this.stamp = ++RustHost.clock; if (RustHost.last !== this) B.surfaceHost(this); RustHost.last = this; if (this.rec) { this.rec.k = k; this.rec.quiet = this.quietSteps; if (SITES) site(this.rec); } if (CAP_AT && (k & 31) === 0 && Date.now() > CAP_AT) process.exit(143);
    // (THE GAME'S DIE ROLLED BETWEEN TWO TICKS: a rule of the tick ran in TypeScript on the puppet, called by the tool. Counted: such a tool leans on code
    // the Rust game has in its place)
    if (this.h && g.dice && g.dice.n !== this.dieRef) { this.dieMoved++; if (this.rec) this.rec.dieMoved = this.dieMoved; }
    { const why = this.startWhy(); if (why === 'back') { this.restep(); this.handWrites(k); } else if (why !== null) this.start(why); else this.handWrites(k); }
    if (this.starts > 0 && this.scanStart !== this.starts) { this.scanStart = this.starts; this.scan = null; } if (!this.quietOn) this.scanFlush();
    if (this.keeps()) this.shelfIn();
    this.o.beforeHost?.(this, cmds);
    // THE HOST'S HALF: each seat's record, every camera and Input row, in the bridge's own small buffer (export.ts hostHalf, io::take_host_half)
    t = now(); this.tellShown(); const hb: Uint8Array = B.hostHalf(g, B.reg, cmds); this.hostBytes = hb.length; this.F.hand(hb, (p, n) => X.face_host_half(this.h, p, n)); this.viewLive();   /* (the view's matrix is the host camera's as the host left it: game_run::view_live) */ this.T.hostHalf += now() - t;
    const hw = TICKS_DIR ? halfWord(B, g, cmds) : null;
    // THE STEP'S OWN WARM-UP (`warmSteps`, off by default): a second game made from the same start, stepped with this tick's record and let go before
    // the game's own first step. MEASURED TO BUY NOTHING: the first tick is the worst of a play (about 20 ms on a quiet machine) with three such steps
    // before it as without (19.8 against 19.5 ms, night-breach:1), and no step grows the memory once room is reserved. So the first tick's cost is the
    // game's own (what a game builds the first time it is asked), not the module's compiling.
    if (this.warmDue) { this.warmDue = false; const t1 = now(), n = this.o.warmSteps ?? 0, h2 = n > 0 && this.lastState && this.lastBlock ? this.F.hand(this.lastState, (p, m) => this.F.hand(this.lastBlock!, (bp, bn) => X.face_new(p, m, bp, bn))) : 0;
      if (h2) { for (let i = 0; i < n; i++) { this.F.hand(hb, (p, m) => X.face_host_half(h2, p, m)); X.face_step(h2, dt, 0); X.face_lights(h2, dt); X.face_puppet(h2, i === 0 ? 1 : 0); } X.face_puppet_drop?.(h2); X.face_drop(h2); }
      this.T.start += now() - t1; }
    this.memPages = this.F.mem.buffer.byteLength;
    t = now(); try { X.face_step(this.h, dt, 0); } catch (e) { throw new Error(`the Rust game's tick ${k} panicked: ${this.F.panicText() || String(e)}`); } const el = now() - t; this.T.step += el; if (el > this.worstStep) { this.worstStep = el; this.worstStepAt = k; }
    if (this.F.mem.buffer.byteLength !== this.memPages) this.grew++;
    if (!this.o.quiet) { const said = this.F.outText(); if (said) this.say(said.trimEnd()); }
    // WHAT THE TICK ASKED OF THE SAVE CONTROLLER (player.ts:493, Enter once down): a save in memory to go back to as this update ends. The controller is
    // the host's: its `due` is set on the puppet and its tick, in the dress step, puts the kept game back. (The page's path: src/host/rustgame.ts.)
    if (this.keeps() && X.face_saves_asked) { const asked = X.face_saves_asked(this.h); if ((asked & 1) && g.saves) { g.saves.due = this.F.outText(); this.count('the tick asked for a save in memory to be put back (Enter once down)'); }
      // (the same call puts the Rust game's flag of a restart's camera cut down; the puppet's own update takes the cut on its next frame, as the TypeScript's does)
      if (asked & 2) { if (this.startKept) { this.toStart(); this.refs(); this.inStep = false; this.dieRef = g.dice?.n; return; } this.cutDue = true; } }
    if (TICKS_DIR && DUMP_AT.has(k)) { X.face_export(this.h); writeFileSync(`${TICKS_DIR}/rust-t${k}.bin`, this.F.outView()); }
    this.o.afterStep?.(this);
    // (RH_HALF=<ticks, comma>: the host's half of those ticks, a line a field's value, for two runs to be held to each other by `diff`)
    if (process.env.RH_HALF?.split(',').includes(String(k))) halfSay(B, g, cmds, k);
    if (TICKS_DIR) { if (k === 1 || this.ticksFile === '') { this.ticksFile = ticksFile(this.audit ? 'audit' : 'rust'); writeFileSync(this.ticksFile, ''); } tickLine(this.F, this.ticksFile, k, this.dieN(), this.rustBlock(), hw, this.gave.map(l => { const w = l.split(' '); return `${w[0]}:${w[1]}`; }).join(',')); }
    // ---- the host's steps after a tick, on the Rust side: the lights' update, the host's view of a level on rails, the camera's ask
    t = now(); X.face_lights(this.h, dt); X.face_view(this.h); X.face_follow?.(this.h);
    this.T.after += now() - t;
    // ---- A QUIET STEP (`quietBegin`): the tool that asked for this run of steps has said none of its code runs between them, so nothing of the state
    // goes to the puppet until the run ends. The host's own camera needs two things of the game after every tick: asked for by `face_look`.
    this.lastDt = dt;
    if (this.quietOn) { const at = (X.face_look(this.h) >>> 0) >>> 3, L = new Float64Array(this.F.mem.buffer).subarray(at, at + 6); this.lookNow = { follow: [L[0], L[1], L[2]], aim: [L[3], L[4], L[5]] };
      this.quietOwed++; this.quietSteps++; this.inStep = false; return; }
    const backs0 = this.backs; const rust = this.tail(dt, k);
    // (a kept game stood again inside this update, Enter once down: the tick's line is the game as the update left it, as the TypeScript's own line is;
    //  a reader of the file takes a tick's LAST line)
    if (TICKS_DIR && this.backs !== backs0) tickLine(this.F, this.ticksFile, k, this.dieN(), this.rustBlock(), hw, this.gave.map(l => { const w = l.split(' '); return `${w[0]}:${w[1]}`; }).join(','));
    this.refs(); this.inStep = false; this.dieRef = g.dice?.n; if (this.busy === 0) this.scanRebase();
    this.auditAfter(rust, k);
    if (SAY_FN) sayLine(this.audit ? 'audit' : 'rust', g, k);
  }
  /** after a step, or a quiet run of them: the apply, the out events, the dress step. Returns the whole Rust state where the audit took one */
  /** the page's dress step's side list (boxes only the picture has) */
  private side = { list: [] as unknown[], of: null as unknown, base: 0 };
  private tail(dt: number, k: number): Uint8Array | null {
    const g = this.g, X = this.F.X, B = this.B; let t = now();
    // ---- the apply
    this.applyStep();
    let rust: Uint8Array | null = null;
    if (this.audit) rust = this.applyWhole('audit');
    else { const why = this.wholeOwed(k); if (why) this.applyWhole(why); }
    if (this.cutDue) { this.cutDue = false; g.restartCut = true; }
    // (one window on the box world's copy for what the host asks of it behind a step, as on the page: src/host/rustgame.ts apply)
    // THE PICTURE'S HALF runs in it, for a game stood with the dressing rows up (a check about what is drawn): the page's own dress step, one path with the
    // page (src/host/rustdress.ts: the men handed to the renderer, the wind's hand-over, the air, the stature). It ticks the par clock and the verdict itself
    const picture = this.o.dress !== false && this.o.dressDown === false && this.o.draws !== false && !OFF.has('pagedress');
    g.col.window(() => { this.dispatch(); if (!picture) return; const t0 = now(); try { B.pageDress(g, dt, this.side); } catch (e) { this.count(`the page's dress step THREW on the puppet: ${String((e as any)?.message ?? e).slice(0, 100)}`); } this.T.dress += now() - t0; });
    // ---- the dress step: what the tick does that has no Rust body and a reader of the puppet needs
    // (THE AUTOSAVE is the host's and decides off the frame as it now stands, so it runs here, on the puppet. Its one write into the game is the
    // chronicle's row, and on a level on rails it stops the level's program for the take: both are made on the Rust side by `face_checkpoint`, as the
    // replay loop makes them, and the puppet takes the state again.)
    t = now(); const nest0 = this.nest;
    if (this.o.dress !== false) {
      const S = g.saves;
      // (F5, the save by hand: off the tick's own record, as the page's host reads it, src/host/rustgame.ts)
      if (this.B.SAVETIMELINE.on && this.lastCmds?.[g.seats.indexOf(g.hostSeat())]?.pressed.includes('F5') && !g.player.down) S?.askSave();
      // (a take is due this tick: an authored checkpoint a rule asked for, a save by hand, a load. The puppet takes the whole state first. An autosave
      // the controller decides on inside its tick takes the whole state as the take starts: `taps`. Its gate and its triggers read the puppet as the
      // filtered apply left it.)
      // AN AUTHORED CHECKPOINT ON A LEVEL ON RAILS: the save controller stops the level's program for the take (savestate.ts:991 `D.stop(false);
      // g.rails.resume()`), and the program is the Rust game's: a run the apply made on the puppet has none of its own (director.ts:525 threw on
      // `R.drills`). The Rust side stops it first and the puppet takes that, so the controller finds no program running and keeps the floor as it is.
      if (S?.authored && X.face_stop_program?.(this.h)) { this.count('an authored checkpoint: the level\'s program stopped on the Rust side before the take'); this.applyStep(); if (this.audit) rust = this.applyWhole('audit'); }
      if (S && !this.audit && !this.wholeNow && !this.keeps() && (S.authored || S.hand || S.due)) this.applyWhole('a save is due');
      try { S?.tick(g, dt); } catch (e) { this.count(`saves.tick THREW on the puppet: ${String((e as any)?.message ?? e).slice(0, 120)}`); }
      // (a save kept as a Rust game made the row on the Rust side as it was taken, before the clone: `snapTake`)
      if (this.tookRow) { this.tookRow = false; this.count('saves.tick took a checkpoint (made on the Rust side by face_checkpoint)'); this.applyStep(); if (this.audit) rust = this.applyWhole('audit'); this.dispatch(); }
      // (a load inside the controller's tick, Enter once down: the kept game stands again and the puppet took it whole; the controller's lines after it)
      if (this.backDue) { this.settle(); this.dispatch(); } }
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
  // run is taken as it ends; the verdict ticks once for the run, over the run's whole time; an out event reaches the host's objects late.
  quietOn = false; quietSteps = 0; quietRuns = 0; private quietOwed = 0; private lastDt = 1 / 60; private lookNow: { follow: number[]; aim: number[] } | null = null;
  private tsUpload: (() => void) | null = null;
  /** true where the host can step quietly (the fast way, a face that answers `face_look`) */
  // NOT YET THE SAME PLAY, so it is asked for by name (RH_QUIET=1): the Rust tick writes the lens's own row at times (a start snaps it onto the man), a
  // quiet run does not bring that across, and the host hands its own lens back with the next record. tools/blitz/quiet-check.ts: the die and the
  // block equal on every one of 2,340 ticks, the host's half not (FollowCamera.target at tick 2). It wants the camera's rows on the quiet path.
  quietBegin(): boolean { if (this.audit || !this.F.X.face_look || OFF.has('quiet') || process.env.RH_QUIET !== '1') return false; this.quietOn = true; this.quietOwed = 0; return true; }
  quietEnd(): void { if (!this.quietOn) return; this.quietOn = false; this.lookNow = null; if (!this.quietOwed || !this.h) return; this.quietRuns++;
    const g = this.g; this.inStep = true; this.tail(this.lastDt * this.quietOwed, this.k);   // (the run's whole time: the dress step's clocks add what they are handed)
    this.refs(); this.inStep = false; this.dieRef = g.dice?.n; this.quietOwed = 0; this.scanRebase();
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

// ---- THE DRESS ROWS ARE ONE TABLE A PROCESS, and a game under the word is stood with them down. A check the table keeps on the TypeScript tick, stood
// later in the same process, must find them as they were: with them left down six of a family's eleven checks went red behind one game on the Rust
// game, each green alone. The harness puts them back as it stands a game with no word (`__tpsDressBack`).
// STAND-IN: rows of the process where the stand is a game's. Belongs with the stand: a game's own dress rows, or the word's stand not touching them.
let DRESS_WAS: Record<string, boolean> | null = null;
(globalThis as any).__tpsStandComing = (collect: () => void) => RustHost.standComing(collect); (globalThis as any).__tpsStandEnded = () => RustHost.standEnded();
function dressDown(B: any): void { DRESS_WAS ??= { ...B.DRESS }; B.dressAll(false); (globalThis as any).__tpsDressBack = () => { Object.assign(B.DRESS, DRESS_WAS); }; }
// ---- THE HARNESS'S WORD (tools/qa/headless.ts `standUp`): TPS_GAME=rust the game the harness stands is a puppet of the Rust game; TPS_GAME=audit the
// same with the audit; TPS_GAME=ts the SAME STAND (the game's own die, the dress rows down) stepped by the TypeScript tick, for a tally to hold the Rust
// game's beside. Unset: nothing here is loaded. (Not TPS_RUST: src/sim/sim.ts refuses that word as a retired one.)
export async function standRust(g: any, word: string, o: { /** the dressing rows up for this game (a check about one of them) */ dress?: boolean; /** false: no walk of the puppet for a tool's writes (HostOpts.scan) */ scan?: boolean; /** false: nothing is drawn (HostOpts.draws) */ draws?: boolean } = {}): Promise<RustHost | null> {
  if (!(globalThis as any).__tpsTop) throw new Error(`TPS_GAME=${word} needs the bridge's preload in this process and its children: BUN_OPTIONS=--preload=./tools/blitz/expose.ts (tools/blitz/rust.sh sets both)`);
  // (THE DIE: the game's own, seeded off the tool's `Math.random` as it stands when the game is stood, so a tool that seeds a run by its seed number
  // (playthrough.ts:1646) rolls another die a seed, and the same die under `ts` and `rust`)
  const die = g.dice ? null : (Math.random() * 4294967296) >>> 0;
  hang(g, 'gameWord', word);   // (the word this game was stood under: tools/qa/headless.ts `seedGame` reads it)
  if (process.env.RH_DIE_SAY) appendFileSync(process.env.RH_DIE_SAY, `${process.argv.slice(2).join(' ')} die ${die}\n`);   // (to play the same game again by hand: mission.ts --die, play.ts --die)
  await sayReady();
  if (word === 'ts') { const B = await bridge(); if (o.dress) (globalThis as any).__tpsDressBack?.(); else dressDown(B); if (die !== null) g.dice = B.newDice(die);
    // THE SAME STAND: under the Rust word every save is one in memory (the Rust game's own clone, as on the page); so under `ts` every save is the
    // TypeScript's snapshot in memory (rollback.ts, what a squad's save already is) and none a file. A file's load stages the encounter again and pours
    // the file in: not the saved moment to the bit, and no game could be held to it tick for tick.
    if (!OFF.has('keep')) B.SNAPS.own = true;
    // (the trace under `ts`: the same line a game, its ticks counted round the TypeScript tick itself)
    if (TICKS_DIR) { const F = loadFace(), file = ticksFile(word), was = g.stepSeats, gave: string[] = []; let k = 0; writeFileSync(file, ''); const B = await bridge(); g.stepSeats = function (this: any, ...a: any[]) { let half: string | null = null; try { if (!k) { g.sim.settle(g); B.exportGame(g, B.reg); const O = g.orders, was2 = O?.give; if (was2) hang(O, 'give', function (this: any, id: string, ...r: any[]) { gave.push(`${k + 1}:${id}`); return was2.call(this, id, ...r); }); }   /* (the bridge's book, as the host's start makes it) */ half = halfWord(B, g, a[1]); } catch { /* a game the bridge has no book for yet */ } if (half && process.env.RH_HALF?.split(',').includes(String(k + 1))) halfSay(B, g, a[1], k + 1); const r = was.apply(this, a); const w: Uint32Array = g.sim.w; if (SAY_FN) sayLine(word, g, k + 1); if (DUMP_AT.has(k + 1)) { g.sim.settle(g); writeFileSync(`${TICKS_DIR}/ts-t${k + 1}.bin`, B.exportGame(g, B.reg)); } tickLine(F, file, ++k, g.dice?.n, new Uint8Array(w.buffer, w.byteOffset, w.byteLength), half, gave.join(',')); return r; }; }
    if (process.env.RH_TRACE) { const rec = traceRec(word), was = g.stepSeats; g.stepSeats = function (this: any, ...a: any[]) { rec.k++; if (SITES) site(rec); if (CAP_AT && (rec.k & 31) === 0 && Date.now() > CAP_AT) process.exit(143); return was.apply(this, a); }; }
    return null; }
  if (word !== 'rust' && word !== 'audit') throw new Error(`TPS_GAME=${word}: rust (the Rust game steps), audit, or ts (the same stand, the TypeScript tick)`);
  await RustHost.ready(); const H = RustHost.attach(g, { audit: word === 'audit', quiet: true, die, dressDown: !o.dress, wholeEvery: +(process.env.TPS_GAME_WHOLE ?? 0), ...(o.scan === false ? { scan: false } : {}), ...(o.draws === false ? { draws: false } : {}) });
  if (process.env.RH_TRACE) trace(H, word);
  if (process.env.RH_TIMES) timed(H);
  if (process.env.RH_STALE) staleAtEnd(H);
  return H;
}
// ---- RH_TIMES=<file>: as the process ends, a line a game stood on the Rust game: the ticks it stepped, the clock from its first step to its last (the
// tool's own code between two steps is in it), and every ms the host spent by what it did (the same sums `times()` prints per tick)
// ---- RH_STALE=<file>: WHAT THE PUPPET HAS LET FALL BEHIND. As the process ends, every game that still stands on the Rust game has its puppet exported
// whole and held to the Rust game's whole state, and so every RH_STALE_EVERY ticks before it (300; a whole apply puts every field back, so the end alone
// would miss what fell behind between two): a line a field in which the two differed (how many places, one of them, in how many looks). A field on the every-tick list
// should never be there; a field on no list that something outside the tick reads is a fault waiting (tools/blitz/stale.ts makes the table).
// (The games are held here until the end: with this word a process frees none.)
const STALE: RustHost[] = [];
const STALE_EVERY = +(process.env.RH_STALE_EVERY ?? 300);
function staleAtEnd(H: RustHost): void {
  if (!STALE.length) process.on('exit', () => { for (const h of STALE) { let t: string; try { t = h.staleSaid(); } catch (e) { t = `# threw\t${String((e as Error)?.message ?? e).split('\n')[0].slice(0, 200)}\n`; } try { appendFileSync(process.env.RH_STALE!, t); } catch { /* the file is not the tool's */ } } });
  STALE.push(H); }
const TIMED: RustHost[] = [];
function timed(H: RustHost): void {
  if (!TIMED.length) { process.on('SIGTERM', () => process.exit(143)); process.on('exit', () => { try { appendFileSync(process.env.RH_TIMES!, TIMED.filter(h => h.k > 0).map(h => JSON.stringify({ pid: process.pid, tool: (process.argv[1] ?? '').split('/').pop(), args: process.argv.slice(2, 6).join(' ').slice(0, 80), probe: (globalThis as any).__probeNow ?? process.env.RH_PROBE ?? null,
    k: h.k, wall: +(h.wallLast - h.wall0).toFixed(1), spent: +h.spent().toFixed(1), N: Object.fromEntries(Object.entries(h.N).map(([n, v]) => [n, +(v as number).toFixed(1)])), scan: (h as any).scan?.stats?.() ?? null, T: Object.fromEntries(Object.entries(h.T).map(([n, v]) => [n, +(v as number).toFixed(1)])), starts: h.starts, wholes: Object.fromEntries(h.wholes), calls: Object.fromEntries(h.calls), evicted: h.evicted })).join('\n') + '\n'); } catch { /* the line is not the tool's */ } }); }
  TIMED.push(H); }
// (TPS_GAME=table, and a run forced off its word: tools/blitz/word.ts, the one switch; `standRust` is handed the word that came out of it)
// ---- RH_TRACE=<file>: as the process ends, a line a game stood under the word: the tool's file that stood it (the first frame of the stack under tools/
// that is not the harness's own), the probe running then (probes.ts names it), the ticks it stepped, its whole applies, its calls, what threw
// ---- RH_TICKS=<folder>: a line a tick under either word (the tick, the die's count, a hash of each part of rust/sim's block but `poses`), in a file
// named by the word and the tool's own arguments: two plays of one tool under `ts` and `rust` are then held to each other tick by tick, where the tool's
// own stand (its die, its own draws) is not one `play.ts` can make again
/** RH_HALF=<ticks, comma>: the host's half of those ticks, a line a field's value, under either word */
function halfSay(B: any, g: any, cmds: readonly any[], k: number): void { const A = B.A, word = g.rustHost ? 'rust' : 'ts', say = (t: string) => { if (TICKS_DIR) appendFileSync(`${TICKS_DIR}/${word}-half.txt`, t); else process.stderr.write(t); };   /* (a tally plays its games in children whose output goes nowhere: with RH_TICKS the lines go to `<word>-half.txt`) */ for (const [ty, v] of B.hostHalfParts(g, cmds) as [string, any][]) { if (ty === 'u32') { say(`half ${k} u32 ${v}\n`); continue; } const b: Uint8Array = B.exportPart(g, B.reg, [[ty, v]]); say(`half ${k} part ${ty} ${b.length} bytes ${Buffer.from(b).toString('hex')}\n`); const offs: number[] = []; for (let i = 0; i < b.length; i += 8) offs.push(i);
      for (const x of A.locate(b, offs, null, true, ty, ty.split('::').pop())) { let f = NaN; try { f = new DataView(b.buffer, b.byteOffset + x.off, 8).getFloat64(0, true); } catch { /* the last bytes */ } say(`half ${k} ${x.path} ${f}\n`); } } }
const TICKS_DIR = process.env.RH_TICKS ?? '';
/** RH_DUMP=<ticks, comma> (with RH_TICKS): the whole state as those ticks return, `<folder>/<word>-t<k>.bin`: the TypeScript game's export under `ts`, the
 *  Rust game's own under `rust` (tools/blitz/diff.ts holds the two to each other, a path a line) */
const DUMP_AT = new Set((process.env.RH_DUMP ?? '').split(',').filter(Boolean).map(Number));
// ---- RH_SAY=<an expression of g> (with RH_TICKS): beside the tick lines, a line a tick of what THE TOOL WOULD READ: the expression's value on the game
// under `ts`, on the puppet once the step's apply and dress step are done under `rust` (`<folder>/<word>-say.txt`). Two plays whose die and block are equal
// and whose `say` lines differ show a read of the puppet that is not the TypeScript game's. `RH_SAY=digest`: src/game/digest.ts's digest of the game, hashed;
// RH_SAY_EVERY=<n>: every n ticks (the digest is a walk).
const SAY_EXPR = process.env.RH_SAY ?? '', SAY_EVERY = +(process.env.RH_SAY_EVERY ?? (SAY_EXPR === 'digest' ? 30 : 1)); let SAY_FN: ((g: any) => unknown) | null = null, SAY_FILE = '';
async function sayReady(): Promise<void> { if (!SAY_EXPR || !TICKS_DIR || SAY_FN) return; if (SAY_EXPR === 'digest') { const D = await import('../../src/game/digest.ts'); SAY_FN = g => Bun.hash(JSON.stringify(D.digestGame(g))).toString(36); } else SAY_FN = new Function('g', `return (${SAY_EXPR});`) as (g: any) => unknown; }
function sayLine(word: string, g: any, k: number): void { if (!SAY_FN || k % SAY_EVERY) return; if (k === SAY_EVERY || !SAY_FILE) { SAY_FILE = `${TICKS_DIR}/${word}-say.txt`; if (k === SAY_EVERY) writeFileSync(SAY_FILE, ''); } let v: unknown; try { v = SAY_FN(g); } catch (e) { v = `THREW ${String((e as Error)?.message ?? e).slice(0, 80)}`; } appendFileSync(SAY_FILE, `${k} ${typeof v === 'string' ? v : JSON.stringify(v)}\n`); }
/** THE HOST'S HALF AS ONE WORD for a tick's line: every part's bytes but the lens's serial (`FollowCamera.desired_serial`, the last word of a camera's row:
 *  the bridge's own count of the camera's `desired` arrays, which after a load is the frame's count on the TypeScript side and the saved moment's plus
 *  one on a puppet; no rule reads it, and with it in the word every tick after a load read "the host's half differs") */
function halfWord(B: any, g: any, cmds: readonly any[]): string { let h = 0n; for (const [ty, v] of B.hostHalfParts(g, cmds) as [string, any][]) { if (ty === 'u32') { h = BigInt(Bun.hash(`${h}u${v}`)); continue; } const b: Uint8Array = B.exportPart(g, B.reg, [[ty, v]]); h = BigInt(Bun.hash(ty === 'crate::camera::FollowCamera' ? b.subarray(0, b.length - 4) : b, h)); } return h.toString(36); }
function tickLine(F: Face, file: string, k: number, die: unknown, block: Uint8Array, half: string | null, gave = ''): void { const X = F.X, base = X.state_addr(0) >>> 0; let l = `${k} ${die} ${half ?? '-'}|${gave}`;   // (the third word: the host's half handed in as the tick opened)
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
