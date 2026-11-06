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
//                        man goes down or the mission's stage moves (the end's reads), after a room is flooded, every `wholeEvery` ticks if asked,
//                        and every tick on a tutorial (its player meters the floor's lamps, and no lamp is on the list).
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
  /** false: on a level on rails the whole state is applied every tick, not only the records its player reads */
  lamps?: boolean;
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
export function loadFace(wasm = DEFAULT_WASM): Face {
  let F = FACES.get(wasm); if (F) return F;
  if (!existsSync(wasm)) throw new Error(`rust-host: no module at ${wasm} (cd rust/game_face && cargo build --target wasm32-unknown-unknown --release)`);
  const X: any = new WebAssembly.Instance(new WebAssembly.Module(readFileSync(wasm)), {}).exports, mem: WebAssembly.Memory = X.memory, dec = new TextDecoder();
  const u8 = () => new Uint8Array(mem.buffer), u32 = () => new Uint32Array(mem.buffer);
  // (an address past 2 GB comes back from the module as a negative number: `>>> 0`)
  const outView = () => { const p = X.face_out_ptr() >>> 0; return u8().subarray(p, p + (X.face_out_len() >>> 0)); };
  const outText = () => dec.decode(outView());
  const NAMES: string[] = []; for (let i = 0; i < X.face_parts(); i++) { X.face_part_name(i); NAMES.push(outText()); }
  F = { X, mem, NAMES, u8, u32, outText, outView,
    panicText: () => { try { const p = X.face_panic_ptr() >>> 0; return dec.decode(u8().subarray(p, p + X.face_panic_len())); } catch { return ''; } },
    hand<T>(b: Uint8Array, f: (p: number, n: number) => T): T { const p = X.face_alloc(b.length) >>> 0; u8().set(b, p); try { return f(p, b.length); } finally { X.face_free(p, b.length); } },
    partHashes: (slot: number) => { const at = (X.face_part_hashes(slot) >>> 0) >>> 2; return Array.from(u32().subarray(at, at + NAMES.length)); } };
  FACES.set(wasm, F); return F; }

// ---- the bridge, loaded once and after the game's own modules (classes.ts enters src/game by game.ts, the one door)
let BR: any = null;
async function bridge(): Promise<any> {
  if (BR) return BR;
  const E = await import('./export.ts'), I = await import('./import.ts'), C = await import('./classes.ts'), P = await import('./puppet-apply.ts'), O = await import('./out-dispatch.ts'), A = await import('./audit.ts');
  const K = await import('../../src/game/knobs.ts'), D = await import('../../src/math/dice.ts'), DR = await import('../../src/game/dress.ts'), SD = await import('../../src/math/side.ts');
  const LJ = JSON.parse(readFileSync(new URL('./puppet-lists.json', import.meta.url).pathname, 'utf8'));
  // THE WHOLE APPLY LEAVES THE BOX MIRROR ALONE: its fields describe the copy of the box world in the module that holds it (the TypeScript game's is another
  // module than the Rust game's), so the puppet keeps its own and rebuilds it when the apply says the boxes were written (PUT_BACKS)
  const only = new Set<string>(); for (const T0 of Object.values<any>(E.SCHEMA.types)) if (T0.k === 'struct' && T0.mod !== 'scene_box_mirror') for (const f of T0.fields) only.add(`${T0.mod}::${T0.name}.${f.rust}`);
  // (the lamps: every field of render/lights.ts's records. No lamp is on the filtered list)
  // A LEVEL ON RAILS: what its scripted player (tools/qa/lesson-run.ts) reads that the filtered list lacks, by module or record, found by playing (the
  // host's half held to the plain play's): the lamps (lesson-run.ts:228 → guards.ts lightEstimateAt), the player's own records (his wall hold), the
  // lesson's, the rails', the director's, the nearby list's rows, a man's own record. RH_RAILS=<modules and mod::Type, comma> names another list; RH_RAILS=most takes every
  // module but the heavy tables (the boxes, the poses and clips, the ragdolls).
  const RW = process.env.RH_RAILS ?? 'render_lights,lesson,rails,director,cleared,player,game::Player,game::Interactable,character::Character', NOT = new Set(['scene_box_mirror', 'scene_boxes', 'anim_skeleton', 'anim_gltf', 'ragdoll']), INC = new Set(RW.split(','));
  const lamps = new Set<string>(); for (const T0 of Object.values<any>(E.SCHEMA.types)) if (T0.k === 'struct' && (RW === 'most' ? !NOT.has(T0.mod) : INC.has(T0.mod) || INC.has(`${T0.mod}::${T0.name}`))) for (const f of T0.fields) lamps.add(`${T0.mod}::${T0.name}.${f.rust}`);
  BR = { ...E, applyGame: I.applyGame, hooks: { classes: C.classTable(), values: C.valueTable() }, puppetRows: P.puppetRows, SLOW: P.SLOW, VALS: P.VALS, OUT: O.OUT, A, reg: K.knobRegistry(), newDice: D.newDice, dressAll: DR.dressAll,
    lists: { step: new Set<string>(LJ.step), start: new Set<string>(LJ.start) }, only, lamps, rawKey: SD.rawKey };
  return BR; }

const now = () => performance.now();
// STAND-IN: AN EMPTY LIST THE APPLY NEVER MAKES. A value the FILTERED apply makes (a record inside a row or a module's state, new in a step) has only the
// listed fields; and the importer, reading an empty list, map or set into a field that is undefined on an object it did not itself make in that apply,
// leaves it undefined (import.ts `if (!L) return cur`), so a WHOLE apply afterwards does not complete it either. Readers then throw on
// `undefined.values()` (laststand.ts:611 `st.named`, from tools/qa/playthrough.ts:1712) or `for (… of undefined)` (tools/qa/hole-watch.ts:49 `r.opened`).
// After a whole apply this walks the puppet by the schema and gives every list, map and set that the state holds (not an Option of one) and the object
// lacks an empty one. Belongs in import.ts (a whole apply completes an object another apply made) or in the filtered export (a value new in a step goes whole).
function completer(B: any): (g: any) => number {
  const S = B.SCHEMA, parse = B.parse, TOP = /^B\.top\("([^"]+)", "([^"]+)", "([^"]*)"\)$/, holdMemo = new Map<any, boolean>(), plans = new Map<any, any[]>();
  const typed = (n: any) => n.k === 'vec' && ['f32', 'u8', 'i32', 'u32'].includes(n.of.k);
  /** does a value of this type hold, at any depth short of a table's row, a list that could be missing */
  const holds = (n: any): boolean => { let h = holdMemo.get(n); if (h !== undefined) return h; holdMemo.set(n, false);
    switch (n.k) { case 'vec': case 'map': case 'set': h = true; break; case 'opt': case 'arr': h = holds(n.of); break; case 'tup': h = n.of.some(holds); break;
      case 'type': { const T = S.types[n.path]; h = T.k === 'struct' && T.fields.some((f: any) => holds(parse(f.ty))); break; } default: h = false; }
    holdMemo.set(n, h); return h; };
  const plan = (T: any): any[] => { let P = plans.get(T); if (!P) plans.set(T, P = T.fields.map((f: any) => ({ f, N: parse(f.ty), top: f.expr?.match(TOP) ?? null })).filter((x: any) => holds(x.N) && (x.top || x.f.expr === 'o' || (x.f.expr === undefined && x.f.ts !== undefined)))); return P; };
  let n = 0, G: any = null, seen = new Set<any>();
  const val = (N: any, v: any): void => { if (v === null || v === undefined || typeof v !== 'object') return;
    switch (N.k) { case 'opt': val(N.of, v); return;
      case 'vec': case 'arr': if (Array.isArray(v) && holds(N.of)) for (const x of v) val(N.of, x); return;
      case 'map': if (v instanceof Map && holds(N.V)) for (const x of v.values()) val(N.V, x); return;
      case 'type': { const T = S.types[N.path]; if (T.k === 'struct') struct(T, v); return; } } };
  const struct = (T: any, o: any, same = false): void => { if (!same) { if (seen.has(o)) return; seen.add(o); }
    for (const { f, N, top } of plan(T)) {
      if (f.expr === 'o') { struct(S.types[N.path], o, true); continue; }   // (a record whose fields stand on the object that holds it: the game's `m`)
      if (top) { const cur = (globalThis as any).__tpsTop?.[top[1]]?.[top[2]]; if (cur && typeof cur.liveKeys === 'function') { if (N.k === 'opt' && N.of.k === 'type' && typeof cur.get === 'function') val(N.of, cur.get(G)); } else val(N, cur); continue; }
      const v = o[f.ts];
      if (v === undefined || v === null) { if (v === undefined && !typed(N)) { if (N.k === 'vec') { o[f.ts] = []; n++; } else if (N.k === 'map') { o[f.ts] = new Map(); n++; } else if (N.k === 'set') { o[f.ts] = new Set(); n++; } } continue; }
      val(N, v); } };
  return (g: any) => { n = 0; G = B.rawKey(g); seen = new Set(); struct(S.types[S.root], g); const BK = B.bookOf(g);
    for (const t of S.tables) { const N = parse(t.ty.replace(/^alloc::boxed::Box<(.*)>$/, '$1')); if (holds(N)) for (const row of BK.listOf(t.id) as any[]) val(N, row); }
    return n; }; }
const enc = new TextEncoder();
type Tally = Map<string, { n: number; eg: string; ticks: number; last: number }>;

export class RustHost {
  /** ticks this host has stepped */
  k = 0;
  /** the Rust game's handle and rust/sim's slot it stands in */
  h = 0; slot = 0;
  readonly audit: boolean;
  /** the ms spent, by what */
  T = { wbExport: 0, wbCompare: 0, wbImport: 0, hostHalf: 0, step: 0, after: 0, exportStep: 0, applyStep: 0, exportWhole: 0, applyWhole: 0, exportMost: 0, applyMost: 0, complete: 0, block: 0, out: 0, dress: 0, check: 0, calls: 0, start: 0 };
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
  standIns = { posture: 0, defs: 0, defNoTwin: 0, cut: 0, run: 0, marks: 0, lists: 0 };
  uploads = { rust: 0, late: 0, missed: 0 };
  /** the whole Rust state last applied (audit), and the puppet as the last tick opened */
  lastRust: Uint8Array | null = null; openState: Uint8Array | null = null;
  // ---- the audit's books
  WB: Tally = new Map(); HOSTW: Tally = new Map(); OWNW: Tally = new Map(); WB_REFUSED: Tally = new Map(); APPLY_MISS: Tally = new Map(); CUTW = new Map<string, number>(); BLOCKW = new Map<string, number>();
  wb = { fired: 0, meter: 0, block: 0, hostOnly: 0, refused: 0 }; applyChecks = 0; applyUnequal = 0;
  private lastState: Uint8Array | null = null; private lastBlock: Uint8Array | null = null;
  private pup: any; private say: (s: string) => void;
  private visApplied = NaN; private cutSerial: number[] = []; private wholeNow = false; private uploadDone = false;
  private missionRef: any = null; private loadsRef = 0; private stageRef: any = null; private downRef = -1; private outQ: { arm: string; e: any }[] = [];
  private DEFS = { orig: new Set<any>(), byId: new Map<string, any>() };
  private threw: Record<string, number> = {};
  private memPages = 0; grew = 0; lampsN = 0; private lampsNow = false; private complete: ((g: any) => number) | null = null; private inStep = false; private lastCmds: readonly any[] | null = null;
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
    const B = BR, F = loadFace(o.wasm), H = new RustHost(g, F, B, o);
    if (o.reserveMb !== 0) { const n = (o.reserveMb ?? 48) * 1048576, p = F.X.face_alloc(n); F.X.face_free(p, n); }
    if (o.dressDown !== false) B.dressAll(false);
    if (o.die !== null && !g.dice) g.dice = B.newDice(o.die ?? 20942);
    g.stepSeats = function (dt: number, cmds: readonly any[]) { H.step(dt, cmds); };
    g.rustHost = H; H.taps(g);
    if (o.out) for (const f of ['writeback.txt', 'gives.txt']) writeFileSync(`${o.out}/${f}`, '');
    return H; }
  /** the Rust game is let go (its slot is free for another) and the game steps in TypeScript again */
  detach(): void { const X = this.F.X; if (this.h) { X.face_puppet_drop?.(this.h); X.face_drop(this.h); this.h = 0; } delete this.g.stepSeats; delete this.g.rustHost; }

  private count(name: string) { this.calls.set(name, (this.calls.get(name) ?? 0) + 1); }
  private tsBlock(): Uint8Array { const w: Uint32Array = this.g.sim.w; return new Uint8Array(w.buffer, w.byteOffset, w.byteLength); }
  rustBlock(): Uint8Array { const X = this.F.X, at = X.state_addr(this.slot) >>> 0; return this.F.u8().subarray(at, at + X.state_len()); }
  time(): number { return this.F.X.face_time(this.h); }
  dieN(): number { return this.F.X.face_die_n(this.h); }
  hashes(): number[] { return this.F.partHashes(this.slot); }

  // ---- a start: the puppet as it stands is the game. The first tick's; and again wherever the TypeScript round a step has put another game in the
  // puppet's place (a checkpoint put back, a restart): no stepper can follow that from its own state.
  private start(): void {
    const g = this.g, X = this.F.X, B = this.B, t = now();
    g.sim.settle(g);
    const start: Uint8Array = B.exportGame(g, B.reg), block = this.tsBlock().slice();
    if (this.h) { X.face_puppet_drop?.(this.h); X.face_drop(this.h); }
    this.h = this.F.hand(start, (p, n) => this.F.hand(block, (bp, bn) => X.face_new(p, n, bp, bn))); if (!this.h) throw new Error('face_new: no free slot');
    this.slot = X.face_slot(this.h); { const said = this.F.outText(); if (said && !this.o.quiet) this.say(said); }
    if (X.state_len() !== block.length) this.say(`rust-host: rust/sim's block is ${X.state_len()} bytes in the module and ${block.length} in the TypeScript game`);
    this.lastState = start; this.lastBlock = block; this.lastRust = start; this.starts++;
    if (this.starts === 1) for (const d of g.orders.defs as any[]) { this.DEFS.orig.add(d); this.DEFS.byId.set(d.id, d); }
    // THE MEMO OF THE FILTERED EXPORT is filled by a start's export. Its bytes are not applied: the puppet IS that state (the Rust game was made from it).
    // The every-tick path is then run over every row a few times, so that the first ticks do not pay for its compiling: `face_puppet_warm` sends each
    // row as if it had changed, and applying a row that holds what the puppet holds changes nothing.
    X.face_puppet(this.h, 1);
    const warm = this.o.warm ?? 4; if (X.face_puppet_warm) for (let i = 0; i < warm; i++) { X.face_puppet_warm(this.h); const posture = g.orders.posture, R = B.applyGame(g, this.F.outView(), B.reg, { hooks: B.hooks, puppet: { ...this.pup, start: false } }); this.after(R, posture); }
    this.readCuts(false); this.visApplied = g.player?.visibility; this.missionRef = g.mission; this.loadsRef = g.chronicle?.counts?.loads ?? 0; this.stageRef = this.endMark(); this.downRef = this.downCount();
    this.memPages = this.F.mem.buffer.byteLength; this.T.start += now() - t;
    if (!this.o.quiet) this.say(`rust-host: ${this.starts === 1 ? 'the start' : `A NEW START as tick ${this.k} opens (the TypeScript round the last step put another game in the puppet's place)`}: ${start.length} bytes, the block ${block.length}; slot ${this.slot}; memory ${(this.F.mem.buffer.byteLength / 1048576).toFixed(0)} MB`);
  }
  /** what the scripted player's `ended()` reads (playthrough.ts:733), the mission's stage, and the count of the walls' changes (tools/qa/hole-watch.ts:42
   *  reads each new row of the walls' log whole, the tick it is made) */
  private endMark(): string { const g = this.g; return `${g.mission?.stage}|${g.mission?.debrief}|${g.roomClear?.end}|${g.rescue?.result?.state}|${g.snatch?.end}|${g.destr?.walls?.serial}`; }
  private downCount(): number { const g = this.g; let n = g.player?.down ? 1 : 0; for (const x of g.guards as any[]) if (x.state === 'dead' || x.down) n++; return n; }

  // ---- the applies
  /** each seat's lens: a whole write of `desired` on the Rust side (a cut) is a new array on the puppet, which is how cmd.ts `takeCut` knows one */
  private readCuts(apply: boolean): void {
    const X = this.F.X, g = this.g; if (!X.face_cuts) return; X.face_cuts(this.h); const v = this.F.outView(), dv = new DataView(v.buffer, v.byteOffset, v.byteLength);
    (g.seats as any[]).forEach((s, i) => { const serial = dv.getFloat64(i * 32 + 24, true);
      if (apply && this.cutSerial[i] !== serial) { s.cam.desired = [dv.getFloat64(i * 32, true), dv.getFloat64(i * 32 + 8, true), dv.getFloat64(i * 32 + 16, true)]; this.standIns.cut++; }
      this.cutSerial[i] = serial; }); }
  private took(R: any): void {
    for (const [q, n] of R.noWay) this.noWay.set(q, (this.noWay.get(q) ?? 0) + n); for (const [q, n] of R.noSetter) this.noSetter.set(q, (this.noSetter.get(q) ?? 0) + n); for (const q of R.noWindow) this.noWindow.add(q);
    for (const [c, n] of Object.entries<number>(R.made)) if (n) this.made[c] = (this.made[c] ?? 0) + n; }
  /** what an apply cannot put right by itself (each a STAND-IN with where it belongs) */
  private after(R: any, posture: any): void {
    const g = this.g;
    // STAND-IN: THE LESSON'S RUN IS THE MISSION BY IDENTITY (lesson.ts:348 `settling`: `this.run === this.g.mission`). The bridge carries `Lesson.run` as an
    // identity serial; the Rust game deals its own serials, so the apply puts whatever TypeScript object has that number (or a placeholder) there,
    // `settling` answers false, and the lesson's player stops (tut-solo tick 652). Here a run that the Rust game holds at all is taken to be the mission the
    // puppet holds: wrong for the ticks between a restart's new Mission and the lesson filing it. Belongs in the bridge (the run carried as "is the mission's").
    if (g.lesson && g.lesson.run && g.lesson.run !== g.mission) { g.lesson.run = g.mission; this.standIns.run++; }
    // STAND-IN: THE SQUAD'S POSTURE is three closures in TypeScript (silent.ts:246) and a struct of no fields in the bridge; the apply puts a bare object
    // where the Rust game holds one, and orders.ts:885 (`this.posture?.given(def)`) then throws. The puppet keeps the object it had. Belongs in import.ts.
    if (posture && g.orders.posture && g.orders.posture !== posture) { g.orders.posture = posture; this.standIns.posture++; }
    // STAND-IN: AN ORDER ROW THE RUST GAME REGISTERED INSIDE A TICK is a new row of the table, so the apply makes a new object for it with none of the
    // row's closures (`can`, `go`, …). A new row takes the functions of the row of the same id that the TypeScript game registered, where there is one.
    // Belongs in the generated rows (closures that ask the Rust game by the row's id).
    if (R.made.OrderDef) for (const d of g.orders.defs as any[]) { if (this.DEFS.orig.has(d)) continue; this.DEFS.orig.add(d); const o = this.DEFS.byId.get(d.id); if (!o) { this.standIns.defNoTwin++; continue; } for (const key of Object.keys(o)) if (typeof o[key] === 'function' && typeof d[key] !== 'function') d[key] = o[key]; this.standIns.defs++; }
    this.took(R); this.visApplied = g.player?.visibility;
  }
  private copyBlock(): void { const t = now(), mine = this.tsBlock(), theirs = this.rustBlock(); mine.set(theirs.subarray(0, mine.length)); if (this.audit) this.lastBlock = mine.slice(); this.T.block += now() - t; }
  /** THE FILTERED APPLY: the listed fields of what changed since the last one, and the step's out events */
  applyStep(): void {
    const g = this.g, X = this.F.X, B = this.B, t0 = now(); let t = t0;
    X.face_puppet(this.h, 0); const blob = this.F.outView(); this.stepBytes += blob.length; this.T.exportStep += now() - t;
    const posture = g.orders.posture;
    t = now(); const R = B.applyGame(g, blob, B.reg, { hooks: B.hooks, puppet: { ...this.pup, start: false } }); const el = now() - t; this.T.applyStep += el; if (el > this.worstApply) this.worstApply = el;
    if (R.out?.length) for (const e of R.out) this.outQ.push(e);
    this.readCuts(true); this.after(R, posture); this.copyBlock(); this.steps++; this.nest += now() - t0;
  }
  /** THE WHOLE APPLY: every field the bridge carries but the box mirror's */
  applyWhole(why: string, most = false): Uint8Array {
    const g = this.g, X = this.F.X, B = this.B, t0 = now(); let t = t0;
    // (BETWEEN TWO TICKS the host has moved its own camera and Input on since the Rust game was last handed them (headless.ts `cam.update` after
    // `game.update`); the whole state would put them back a frame. depot-four parted so at tick 1578, a frame after a room was flooded. The host's rows
    // are handed over first, as a tick's opening hands them.)
    if (!this.inStep && this.lastCmds) { const hb: Uint8Array = B.hostHalf(g, B.reg, this.lastCmds); this.F.hand(hb, (p, n) => X.face_host_half(this.h, p, n)); }
    X.face_export(this.h); const blob = this.audit ? this.F.outView().slice() : this.F.outView(); this.wholeBytes = blob.length; this.T[most ? 'exportMost' : 'exportWhole'] += now() - t;
    // STAND-IN: A SEAT'S CUT IS AN IDENTITY (cmd.ts:191 `takeCut`: a cut is pending while the lens's `desired` is not the array the seat last took, `mark`).
    // The whole apply writes `desired` in place and puts into `mark` whatever object has the serial the Rust seat holds there (a count of writes), so
    // `takeCut` would answer a cut EVERY tick and game.ts:1555 would snap the host's camera every tick. The seat keeps its own `mark`; a cut the Rust game
    // made is read off its own count (`face_cuts`). Belongs in the bridge (`desired_serial` carried, `mark` not).
    const posture = g.orders.posture, marks = (g.seats as any[]).map(s => s.mark);
    t = now(); const R = B.applyGame(g, blob, B.reg, { hooks: B.hooks, only: most ? B.lamps : B.only }); const el = now() - t; this.T[most ? 'applyMost' : 'applyWhole'] += el; if (el > this.worstApply) this.worstApply = el;
    (g.seats as any[]).forEach((s, i) => { s.mark = marks[i]; });
    this.readCuts(true); this.after(R, posture); this.copyBlock();
    if (most) { this.lampsN++; this.lampsNow = true; this.nest += now() - t0; return blob; }
    // (the empty lists no apply makes: `completer`, above)
    if (!this.audit) { const t1 = now(); this.standIns.lists += (this.complete ??= completer(B))(g); this.T.complete += now() - t1; }
    this.wholes.set(why, (this.wholes.get(why) ?? 0) + 1); this.wholeNow = true; if (this.audit) this.lastRust = blob; this.nest += now() - t0;
    return blob; }
  /** A LEVEL ON RAILS, every tick: out of the whole state, the records its scripted player reads and the filtered list lacks (the importer's `only`: the
   *  whole export is made and walked, the rest is read past) */
  applyMost(): void { this.applyWhole('', true); }
  /** after a call that changed the Rust game between two ticks: the puppet takes the state again */
  private reapply(why: string): void { if (this.audit) { this.applyStep(); this.lastState = this.applyWhole(why); } else this.applyStep(); }

  // ---- the taps: what the TypeScript round a step calls that WRITES the game and is no input
  private taps(g: any): void {
    const H = this, X = this.F.X, B = this.B, F = this.F;
    // THE SCRIPTED PLAYER'S ORDERS GO TO THE RUST GAME, not through the menu's TypeScript: a row of the menu is closures (`can`, `issue`, `label`), and a row
    // the Rust game registered inside a tick arrives on the puppet with none. The order is given on the Rust side at once, in the words the replay loop
    // gives it (rust/game_run `give`), and the puppet takes the state again so the player reads what the order did before the next tick. An order the
    // face has no words for (a wall, a thing, options, one man by object) still runs on the puppet; the audit hands what it wrote, the fast way cannot.
    const O = g.orders, give = O.give; O.give = function (this: any, id: string, t: any, who: any, opts: any) {
      if (!H.h) return give.call(this, id, t, who, opts);
      const T0 = t ?? this.targetAt(g.aimPoint), w = who === undefined ? '-' : typeof who === 'object' ? `g${g.guards.indexOf(who)}` : String(who);
      const tail = T0.kind === 'door' ? ` door ${g.doors.list.indexOf(T0.door)} ${T0.side} book${B.bookOf(g).ids.get('DoorId')?.get(T0.door)}` : T0.kind === 'none' && T0.man ? ` man ${g.guards.indexOf(T0.man)}` : T0.kind === 'foe' ? ` man ${g.guards.indexOf(T0.foe)}` : (T0.wall || T0.thing) ? ' UNSUPPORTED-wall-or-thing' : '';
      const line = `${H.k + 1} ${id} ${w} ${T0.kind} ${T0.at.join(' ')}${tail}${opts ? ' UNSUPPORTED-opts' : ''}`;
      if (line.includes('UNSUPPORTED') || /^g\d/.test(w)) { H.gave.push(line); H.count(`orders.give(${id}) ON THE PUPPET (the face has no words for its target${H.audit ? '' : ': NOT HANDED to the Rust game'})`); return give.call(this, id, t, who, opts); }
      H.count(`orders.give(${id})`); H.gave.push(line); const t0 = now(), n0 = H.nest; H.handWrites(H.k + 1);
      F.hand(enc.encode(line), (p, n) => X.face_give(H.h, H.k + 1, p, n)); const said = F.outText(), m = said.match(/-> (None|Some\("((?:[^"\\]|\\.)*)"\))\s*$/);
      if (H.o.out) appendFileSync(`${H.o.out}/gives.txt`, `${line}\n   ${said.trim()}\n`);
      if (!m) H.say(`rust-host: the face's answer to an order was not understood: ${said}`);
      H.reapply('an order'); H.T.calls += now() - t0 - (H.nest - n0);
      return m && m[1] !== 'None' ? JSON.parse(`"${m[2]}"`) : null; };
    // THE DRIVER'S QUESTION THAT FLOODS A ROOM AND KEEPS IT (`fight.roomAt(p)` between two ticks). Fast: the Rust game is asked first; where that flooded a
    // room the puppet takes the WHOLE state (a room's rows are not on the filtered list), so the TypeScript's own ask then finds the room and floods none.
    // Audit: the TypeScript floods, and the write-back hands the rooms over.
    if (g.fight?.roomAt) { const Fi = g.fight, ask = Fi.roomAt; Fi.roomAt = function (this: any, p: any) {
      if (H.h && !H.audit && X.face_room_at) { const t0 = now(), n0 = H.nest; if (X.face_room_at(H.h, p[0], p[1], p[2])) { H.wrote.push(`${H.k + 1} room_at ${p[0]} ${p[1]} ${p[2]}`); H.count('fight.roomAt (the Rust game flooded a new room; the puppet took the whole state)'); H.applyWhole('a room flooded'); } H.T.calls += now() - t0 - (H.nest - n0); }
      const n0 = Fi.rooms.length, r = ask.call(this, p); if (Fi.rooms.length !== n0) { if (H.audit) H.wrote.push(`${H.k + 1} room_at ${p[0]} ${p[1]} ${p[2]}`); H.count(H.audit ? 'fight.roomAt (flooded a new room on the puppet)' : 'fight.roomAt FLOODED A ROOM ON THE PUPPET THAT THE RUST GAME DID NOT'); } return r; }; }
    // (a lesson's player picks a row of the nearby list between ticks and reads the answer at once: made on the Rust side, as an order is)
    const wordAt = (ty: string, w: unknown) => w === undefined ? -1 : B.SCHEMA.types[ty].words.indexOf(w);
    const pick = g.selectVerb; g.selectVerb = function (this: any, kind: string, verb?: string) {
      if (!H.h) return pick.call(this, kind, verb);
      H.count('game.selectVerb'); const t0 = now(), n0 = H.nest; H.handWrites(H.k + 1);
      const line = `${H.k + 1} select_verb ${wordAt('crate::game::InteractableKind', kind)} ${wordAt('crate::game::Verb', verb)} ${kind} ${verb}`; H.wrote.push(line);
      const r = F.hand(enc.encode(line), (p, n) => X.face_hostwrite(H.h, H.k + 1, p, n)), said = F.outText();
      if (r !== 1) H.say(`rust-host: the face did not take selectVerb(${kind}, ${verb})`);
      H.reapply('a pick of a verb'); H.T.calls += now() - t0 - (H.nest - n0); return / -> true\s*$/.test(said); };
    // THE HOST'S STEPS ROUND A TICK THAT THE RUST SIDE MAKES TOO.
    // The lights' update: made on the Rust side after every step. Audit, and a fast tick that took the whole state: the puppet already holds what it
    // left, and run again in TypeScript a flash's envelope would be stepped twice. A fast tick that took only the filtered state: no lamp is on that list,
    // so the puppet's own lamps are stepped by the TypeScript's own update, as the Rust game's are by its own.
    const E = g.engine, L = E.lights, W = E.world, lightsUpdate = L.update;
    L.update = function (this: any, ...a: any[]) { if (!H.h || (!H.audit && !H.wholeNow && !H.lampsNow)) return lightsUpdate.apply(this, a); H.count('engine.lights.update (not run on the puppet: it holds what the Rust side\'s left)'); };
    // The box world's listing. Audit: made on the Rust side before the apply on the ticks the headless host lists (`uploadEvery`), and the puppet holds the
    // listed world; a listing asked for on another tick is made then and the puppet takes the state again. Fast: the list of every box and the ray grid
    // are not on the filtered list (they are the TypeScript's own there), so the TypeScript's listing runs as it is, and the Rust game lists its own
    // at the same moment.
    const upload = W.upload;
    W.upload = function (this: any, ...a: any[]) {
      if (!H.h) return upload.apply(this, a);
      if (!H.audit) { X.face_upload(H.h); H.uploads.rust++;
        // STAND-IN: A BOX WHOSE MARKINGS WENT is `marks: null` after an apply (the importer writes a None as null), and the listing tests
        // `b.marks !== undefined` (scene/boxes.ts:841) and then reads its length: it threw at tick 1817 of tut-solo. Belongs in import.ts: the None of a
        // field the TypeScript declares optional is `undefined`.
        for (const L of [this.statics, this.dynamics, g.dynBoxes] as any[][]) for (const b of L) if (b.marks === null) { b.marks = undefined; H.standIns.marks++; }
        return upload.apply(this, a); }
      if (H.uploadDone) { H.uploadDone = false; return; }
      H.uploads.late++; X.face_upload(H.h); H.uploads.rust++; H.applyStep(); H.applyWhole('a listing off its tick'); H.lastState = B.exportGame(g, B.reg); };
    // THE SAVE CONTROLLER KEEPS THE PUPPET (a save in memory is the TypeScript objects as they stand; a file is their serialisation), so a save taken off
    // a puppet that holds only the filtered state would keep fields that stopped moving ticks ago. Before a take the puppet takes the whole state.
    const S = g.saves; if (S) for (const name of ['write', 'writeMem'] as const) { const was = S[name]; if (typeof was !== 'function') continue;
      S[name] = function (this: any, ...a: any[]) { if (H.h && !H.audit && !H.wholeNow) H.applyWhole('a save taken'); return was.apply(this, a); }; }
  }

  // ---- what TypeScript wrote into the puppet since the last apply goes to the Rust game. `at`: the tick that opens next.
  /** fast: the writes that have a call. The lesson's player feeds the light meter before every tick (lesson-run.ts:228) */
  private handWrites(at: number): void {
    if (this.audit) { this.writeBack(at); return; }
    const g = this.g, v = g.player?.visibility;
    if (v !== this.visApplied && typeof v === 'number') { this.F.X.face_meter(this.h, v); this.visApplied = v; this.wb.meter++; }
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
      // (a level script's beat holds its place in a generator no state carries: the Rust import refuses a state with a beat running, by a panic. What
      // TypeScript wrote while one runs cannot be handed over whole; it is named and counted, and the next apply puts the Rust game's own back)
      else if ((B.bookOf(g).listOf('DirectorLiveBeatId') as any[]).some(r => r.state === 'running')) { this.wb.refused++; for (const x of owed) A.tallyAdd(this.WB_REFUSED, x.q, x.path, at); if (owed.some(x => x.path === 'g.pl[0].visibility')) { X.face_meter(this.h, g.player.visibility); this.wb.meter++; } }
      else { this.F.hand(A.withCutsOf(mine, this.lastRust!), (p, n) => X.face_import_into(this.h, p, n)); this.wb.fired++; if (this.o.out && this.wb.fired <= 400) appendFileSync(`${this.o.out}/writeback.txt`, `${at} ${[...new Set(owed.map(x => x.q))].join(' ')}\n   eg ${owed[0].path}\n`); }
    } else this.wb.hostOnly++;
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
      try { OUT[arm](g, e); } catch (x) { const w = String((x as any)?.message ?? x); if (/is not a function/.test(w)) this.outNoHand.set(arm, 1); else if (!this.outThrew.has(arm)) this.outThrew.set(arm, `tick ${this.k}: ${w.slice(0, 160)}`); } }
    Q.length = 0; this.T.out += now() - t; }

  // ---- THE STEP, in place of `stepSeats`
  step(dt: number, cmds: readonly any[]): void {
    const g = this.g, X = this.F.X, B = this.B, k = ++this.k; let t = now();
    this.wholeNow = false; this.lampsNow = false; this.inStep = true; this.lastCmds = cmds;
    if (!this.h || g.mission !== this.missionRef || (g.chronicle?.counts?.loads ?? 0) !== this.loadsRef) this.start(); else this.handWrites(k);
    this.o.beforeHost?.(this, cmds);
    // THE HOST'S HALF: each seat's record, every camera and Input row, in the bridge's own small buffer (export.ts hostHalf, io::take_host_half)
    t = now(); { const hb: Uint8Array = B.hostHalf(g, B.reg, cmds); this.hostBytes = hb.length; this.F.hand(hb, (p, n) => X.face_host_half(this.h, p, n)); } this.T.hostHalf += now() - t;
    this.memPages = this.F.mem.buffer.byteLength;
    t = now(); X.face_step(this.h, dt, 0); const el = now() - t; this.T.step += el; if (el > this.worstStep) { this.worstStep = el; this.worstStepAt = k; }
    if (this.F.mem.buffer.byteLength !== this.memPages) this.grew++;
    if (!this.o.quiet) { const said = this.F.outText(); if (said) this.say(said.trimEnd()); }
    this.o.afterStep?.(this);
    // ---- the host's steps after a tick, on the Rust side: the lights' update, the host's view of a level on rails, the camera's ask
    t = now(); X.face_lights(this.h, dt); X.face_view(this.h); X.face_follow?.(this.h);
    if (this.audit) { const U = this.o.uploadEvery ?? 4; if (k % U === 1 % U) { X.face_upload(this.h); this.uploads.rust++; if (this.uploadDone) this.uploads.missed++; this.uploadDone = true; } else if (this.uploadDone) { this.uploads.missed++; this.uploadDone = false; } }
    this.T.after += now() - t;
    // ---- the apply
    this.applyStep();
    let rust: Uint8Array | null = null;
    if (this.audit) rust = this.applyWhole('audit');
    else { const why = this.wholeOwed(k); if (why) this.applyWhole(why); else if (g.rails?.live && this.o.lamps !== false) this.applyMost(); }
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
    this.loadsRef = g.chronicle?.counts?.loads ?? 0; this.missionRef = g.mission; this.inStep = false;
    // ---- audit: the puppet as the apply left it is what the next write-back is held to
    if (this.audit && rust) { t = now(); const N = this.o.applyCheck ?? 50, chk = N > 0 && (k <= 3 || k % N === 0), A = B.A;
      if ((this.o.baseline ?? 'rust') === 'rust' && !chk) this.lastState = rust;
      else { const mine: Uint8Array = B.exportGame(g, B.reg); this.lastState = (this.o.baseline ?? 'rust') === 'rust' ? rust : mine;
        if (chk) { this.applyChecks++; if (mine.length !== rust.length || !A.buf(mine).equals(A.buf(rust))) { const cut: string[] = [], D = A.fieldsDiffering(rust, mine, cut).filter((x: any) => !B.SCHEMA.serial?.[x.q] && x.q !== 'tail'); for (const q of cut) this.CUTW.set(q, (this.CUTW.get(q) ?? 0) + 1);
          if (D.length) { this.applyUnequal++; for (const x of D) A.tallyAdd(this.APPLY_MISS, x.q, x.path, k); if (this.o.out && this.applyUnequal === 1) { writeFileSync(`${this.o.out}/apply-rust${k}.bin`, rust); writeFileSync(`${this.o.out}/apply-puppet${k}.bin`, mine); } } } } }
      this.T.check += now() - t; }
  }
  /** fast: is a whole apply owed this tick, and why */
  private wholeOwed(k: number): string | null {
    const g = this.g, N = this.o.wholeEvery ?? 0;
    if (N > 0 && k % N === 0) return `every ${N} ticks`;
    // STAND-IN: A TUTORIAL'S PLAYER METERS THE FLOOR'S LAMPS HIMSELF before every tick (tools/qa/lesson-run.ts:228 → guards.ts lightEstimateAt reads
    // `engine.lights`), and no lamp (render_lights::RtLight, LightSet) is on the filtered list; and its steps read the player's own records (his wall
    // hold, `Player.wall.*`: tut-solo's keys parted at tick 884 without them), the lesson's, the rails', the nearby list's. On a level on rails those
    // records are applied every tick out of the whole state (`applyMost`); `lamps: false` takes the whole state instead. Belongs on the generated list.
    if (g.rails?.live && this.o.lamps === false) return 'a level on rails (its player meters the lamps)';
    // STAND-IN: THE END'S READS. The scripted player's report reads the game as its last tick left it, and much of what it reads is not on the filtered
    // list. A man down, the mission's stage moved or one of the ends the scripted player asks for set is where an end, or a tally's note of a death, is read. Belongs on the generated list.
    const st = this.endMark(), dn = this.downCount();
    if (st !== this.stageRef || dn !== this.downRef) { this.stageRef = st; this.downRef = dn; return 'a man went down, a wall changed, the mission moved on or ended'; }
    return null; }

  /** every ms this host has spent, the driver's calls apart (they are made outside `Game.update`) */
  spent(): number { const T = this.T; return T.wbExport + T.wbCompare + T.wbImport + T.hostHalf + T.step + T.after + T.exportStep + T.applyStep + T.exportWhole + T.applyWhole + T.exportMost + T.applyMost + T.complete + T.block + T.out + T.dress + T.check + T.start; }
  /** the table of where a tick's time went, per tick, as PLAY.md has it */
  times(): string { const k = Math.max(1, this.k), per = (x: number) => (x / k).toFixed(3), T = this.T, W = [...this.wholes.values()].reduce((a, b) => a + b, 0);
    return (this.audit ? `write-back: the puppet's export ${per(T.wbExport)}, the compare ${per(T.wbCompare)}, the hand-over ${per(T.wbImport)} · ` : '') +
      `the host's half ${per(T.hostHalf)} (${this.hostBytes} bytes) · face_step ${per(T.step)} (worst ${this.worstStep.toFixed(1)} at tick ${this.worstStepAt}; the memory grew inside ${this.grew} steps) · lights, view, follow${this.audit ? ', listing' : ''} ${per(T.after)} · the filtered export ${per(T.exportStep)} (${(this.stepBytes / Math.max(1, this.steps) / 1024).toFixed(0)} KB a time) · the filtered apply ${per(T.applyStep)} · the whole export ${per(T.exportWhole)} and apply ${per(T.applyWhole)} (${W} whole applies; the walk that gives them their empty lists ${per(T.complete)}), a level on rails' whole export ${per(T.exportMost)} and the apply of its player's records out of it ${per(T.applyMost)} (${this.lampsN} of them; ${(this.wholeBytes / 1048576).toFixed(1)} MB the last; worst apply of either kind ${this.worstApply.toFixed(0)}) · the block's copy ${per(T.block)} · out events ${per(T.out)} · the dress step ${per(T.dress)} · the driver's calls ${per(T.calls)}` +
      (this.audit ? ` · the puppet exported again and held to the Rust bytes ${per(T.check)}` : '') + ` · the start, with the warm-up, once: ${T.start.toFixed(0)} ms`; }
}

// ---- THE HARNESS'S WORD (tools/qa/headless.ts `standUp`): TPS_RUST=1 the game the harness stands is a puppet of the Rust game; TPS_RUST=audit the same
// with the audit; TPS_RUST=ts the SAME STAND (the game's own die, the dress rows down) stepped by the TypeScript tick, for a tally to hold the Rust
// game's beside. Unset: nothing here is loaded.
export async function standRust(g: any, word: string): Promise<RustHost | null> {
  if (word === 'ts') { const B = await bridge(); B.dressAll(false); if (!g.dice) g.dice = B.newDice(20942); return null; }
  if (word !== '1' && word !== 'audit') throw new Error(`TPS_RUST=${word}: 1 (the Rust game steps), audit, or ts (the same stand, the TypeScript tick)`);
  await RustHost.ready(); return RustHost.attach(g, { audit: word === 'audit', quiet: true, wholeEvery: +(process.env.TPS_RUST_WHOLE ?? 0) });
}
