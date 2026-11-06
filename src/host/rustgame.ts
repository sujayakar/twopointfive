// THE PAGE PLAYS THE RUST GAME (`?rust=1`). The simulation's tick is the Rust game's (rust/game, in the module rust/game_face builds: tools/blitz/FACE.md);
// the TypeScript `Game` stays as a PUPPET that everything outside the tick goes on reading: the picture, the sound, the HUD, the menus. Without the word
// nothing here is loaded and the page is the page it was.
//
// A STEP (`Game.update` runs its own host lines as ever: the record sampled from the Input and the camera, hostBefore, hostAfter. The step between them,
// `stepSeats`, is this file's `step`, hung on the game object):
//   the host's half   the knobs the module reads · the light probe's answers (the GPU's, a frame or more old) · each seat's record with every camera and
//                     Input row (the bridge's `hostHalf` → `face_host_half`)
//   the tick          `face_step`: the box world listed again first where the picture has uploaded it since the last step, the tick, the lights' update,
//                     a rails level's host view
//   the apply         `face_puppet`: only what the page reads and only what changed (tools/blitz/PUPPET.md) → `applyGame` onto the puppet, in place
//   the out events    what the tick's rules told the host (a sound, particles, a decal, smoke): the generated dispatcher makes today's calls on the
//                     page's real objects
//   the dress step    src/host/rustdress.ts: what the TypeScript tick did for the picture only
//   the probe         what the tick asked of the light probe is put in the probe's table for the frame's GPU pass
//
// ONE BLOCK. The module is rust/sim's too (the same C exports over the same memory), and src/sim binds to THIS instance (sim.ts `simAdopt`, before anything
// has asked for the module). The puppet's `SimState` stands in a slot of the module's and the Rust game is made over the block in that slot
// (`face_new_at`), so every accessor of src/sim reads what the Rust tick wrote, and nothing is copied. The module's memory grows in play (the game
// allocates): after every call that may have grown it the puppet's `SimState` views are made again.
//
// A START. The TypeScript stands the game as it always did (the level, the mission, the squad, the kit: a restart, a load), then the whole state goes
// across once (the bridge's `exportGame` → `face_new_at`) and the first filtered export is applied. Whatever stands or restores the game from outside a
// tick (`restartEncounter`, a checkpoint loaded) marks the Rust game stale, and the next step starts it again from the puppet.
//
// Every special case here is marked STAND-IN with its reason and where it belongs.
import { simAdopt, type SimExports } from '../sim/sim';
import { BoxMirror } from '../scene/boxMirror';
import { PUT_BACKS } from '../math/side';
import { knobRegistry } from '../game/knobs';
import { knobsToRust } from '../game/rustknobs';
import { DC } from '../sim/doorshome';
import { takeSnapshot, restoreSnapshot, type Snapshot } from '../game/rollback';
import type { Game } from '../game/game';
import type { PlayerCmd } from '../game/cmd';
import { dressStep, DRESS_MS } from './rustdress';
import { exportGame, hostHalf, bookOf, report, SCHEMA } from 'tps-bridge/export';
import { applyGame } from 'tps-bridge/import';
import { classTable, valueTable } from 'tps-bridge/classes';
import { puppetRows, SLOW, VALS } from 'tps-bridge/puppet-apply';
import { OUT } from 'tps-bridge/out-dispatch';
import { installAsks } from 'tps-bridge/ask';
import LISTS from 'tps-bridge/puppet-lists.json';

/** the address word: `?rust=1` (any other value, or no word: the TypeScript game) */
export const rustAsked = (q: URLSearchParams): boolean => q.get('rust') === '1';

type Face = SimExports & Record<string, (...a: number[]) => number>;
let X: Face | null = null, MEM: WebAssembly.Memory | null = null, BUF: ArrayBufferLike | null = null;
/** STAND-IN: the Rust game's copy of the box world is world `g.sim` of the module (rust/game scene_box_mirror `box_mirror_take`), and the puppet's own
 *  copy (scene/boxMirror.ts: what the TypeScript's rays and lines between ticks are asked of) would take the same world and write the puppet's boxes over
 *  the Rust game's rows. This mirror is made before any game and holds that world for the Rust game, so the puppet's copy stands in another. Belongs in the
 *  face (the host says which world, as it says which slot) or in boxMirror.ts (a copy that reads the Rust game's world and writes nothing). */
let PLACE: BoxMirror | null = null;
const dec = new TextDecoder(), enc = new TextEncoder();
const u8 = (): Uint8Array => new Uint8Array(MEM!.buffer);
const outBytes = (): Uint8Array => { const p = X!.face_out_ptr(), n = X!.face_out_len(); return u8().subarray(p, p + n); };
const outText = (): string => dec.decode(outBytes());
const panicText = (): string => { try { const p = X!.face_panic_ptr(), n = X!.face_panic_len(); return dec.decode(u8().subarray(p, p + n)); } catch { return ''; } };
function hand<T>(b: Uint8Array, f: (p: number, n: number) => T): T { const p = X!.face_alloc(b.length); u8().set(b, p); try { return f(p, b.length); } finally { X!.face_free(p, b.length); } }
/** a call into the face: a trap is the module's abort on a panic, whose words it kept */
function call<T>(what: string, f: () => T): T { try { return f(); } catch (e) { const w = panicText(); throw new Error(w ? `the Rust game panicked in ${what}: ${w}` : `${what}: ${String((e as Error)?.message ?? e)}`); } }

/** what the page says of the Rust game, on `window.__rust` */
export const RUST_GAME = {
  on: false, bytes: 0, fetchMs: 0, compileMs: 0, startMs: 0, startBytes: 0, handle: 0, slot: -1, ticks: 0, starts: 0, grew: 0, memory: 0,
  /** ms of the last step and summed over every step: the host's half, the tick, the filtered export, the apply, the out events, the dress step */
  last: { host: 0, step: 0, export: 0, apply: 0, out: 0, dress: 0, bytes: 0 }, sum: { host: 0, step: 0, export: 0, apply: 0, out: 0, dress: 0, bytes: 0 }, worst: { step: 0, apply: 0, at: 0 },
  pokes: 0, lightsMade: 0, wholes: 0, wholeMs: 0, rowsKept: 0, backs: 0, snapMs: 0,
  outEvents: 0, outArms: {} as Record<string, number>, outThrew: {} as Record<string, string>, gives: [] as string[], probeAsks: 0,
  dress: DRESS_MS,
  /** what the forward export could not carry at the last start (the bridge's own report) */
  notCarried: [] as string[],
};

/** THE MODULE, fetched and compiled off the page's thread (Chrome compiles nothing over 8 MB on it), started, and handed to src/sim as the simulation's
 *  one instance. Before anything has asked src/sim for its module. Throws in plain words where there is no module to fetch. */
export async function rustBoot(): Promise<void> {
  const t0 = performance.now();
  let res: Response;
  try { res = await fetch(new URL('game_face.wasm', location.href), { cache: 'no-store' }); } catch (e) { throw new Error(`?rust=1: the Rust game's module could not be fetched (${String((e as Error)?.message ?? e)}). Without the word the page plays the TypeScript game.`); }
  if (!res.ok) { const why = (await res.text().catch(() => '')).slice(0, 400); throw new Error(`?rust=1: there is no Rust game to play here. ${res.status === 404 ? 'This build was made without game_face.wasm (cargo and the wasm32-unknown-unknown target build it: tools/blitz/face-build.ts).' : why} Without the word the page plays the TypeScript game.`); }
  let mod: WebAssembly.Module;
  const t1 = performance.now();
  if (typeof WebAssembly.compileStreaming === 'function' && (res.headers.get('content-type') ?? '').startsWith('application/wasm')) { RUST_GAME.bytes = Number(res.headers.get('content-length') ?? 0); mod = await WebAssembly.compileStreaming(res); }
  else { const b = await res.arrayBuffer(); RUST_GAME.bytes = b.byteLength; mod = await WebAssembly.compile(b); }
  const inst = await WebAssembly.instantiate(mod, {});
  RUST_GAME.fetchMs = t1 - t0; RUST_GAME.compileMs = performance.now() - t1;
  X = inst.exports as unknown as Face; MEM = X.memory; BUF = MEM.buffer;
  for (const n of ['face_new_at', 'face_puppet', 'face_host_half', 'face_step', 'face_ask', 'face_probe_asks', 'face_lights_out', 'face_poke']) if (typeof X[n] !== 'function') throw new Error(`?rust=1: game_face.wasm has no ${n}: it was built from another tree (tools/blitz/face-build.ts builds this one's)`);
  simAdopt(X, RUST_GAME.compileMs);
  PLACE = new BoxMirror(); (PLACE as unknown as { take(): boolean }).take();
  RUST_GAME.on = true; RUST_GAME.memory = MEM.buffer.byteLength;
  console.info(`rust=1: the Rust game's module, ${RUST_GAME.bytes} bytes, fetched in ${RUST_GAME.fetchMs.toFixed(0)} ms and compiled in ${RUST_GAME.compileMs.toFixed(0)} ms; memory ${(RUST_GAME.memory / 1048576).toFixed(0)} MB`);
}

// ---- a game
interface Held { g: Game; snap0: Snapshot | null; doorKept: number[]; lightOf: Map<number, Record<string, any>>; flags: boolean[]; launch: unknown; putBacks: number; loads: number; h: number; slot: number; stale: string; uploaded: boolean; ctx: { p: number }; rows: unknown; inStep: boolean; side: { list: unknown[]; of: unknown; base: number }; asks: { after(): void } | null }
let H: Held | null = null;
const REG = { v: null as Record<string, object> | null };
const reg = (): Record<string, unknown> => (REG.v ??= knobRegistry()) as Record<string, unknown>;
let HOOKS: { classes: Record<string, { prototype: object }>; values: Record<string, unknown> } | null = null;
const PUP = { step: null as Set<string> | null, start: null as Set<string> | null };
const lists = () => ({ step: PUP.step ??= new Set<string>(LISTS.step), start: PUP.start ??= new Set<string>(LISTS.start) });

/** the module's memory grew under a call: every view over the old buffer is detached. src/sim's own views make themselves again (a detached view has no
 *  length); the puppet's block views are made here */
function grown(g: Game): void {
  if (MEM!.buffer === BUF) return;
  BUF = MEM!.buffer; RUST_GAME.grew++; RUST_GAME.memory = BUF.byteLength;
  (g.sim as unknown as { view(): void }).view();
}

/** THE PAGE'S GAME IS A PUPPET OF THE RUST GAME from here: its tick is `step`, and whatever stands it again from outside a tick marks the Rust game stale.
 *  Called once, when the game is stood. */
export function rustAttach(g: Game): void {
  if (!X) throw new Error('rustAttach before rustBoot');
  const G = g as unknown as Record<string, any>;
  H = { g, snap0: null, doorKept: [], lightOf: new Map(), flags: [], launch: null, putBacks: -1, loads: 0, h: 0, slot: -1, stale: 'the game was stood', uploaded: true, ctx: { p: 0 }, rows: null, inStep: false, side: { list: [], of: null, base: 0 }, asks: null };
  const held = H;
  G.stepSeats = (dt: number, cmds: readonly PlayerCmd[]) => step(held, dt, cmds);
  // A START FROM OUTSIDE A TICK: the TypeScript stands or restores the game on the puppet as it always did; the Rust game is started again from it as the
  // next step opens. (Inside a step these are the Rust game's own and never run here.)
  for (const name of ['restartEncounter', 'loadState']) {
    const was = G[name]; if (typeof was !== 'function') continue;
    G[name] = function (this: unknown, ...a: unknown[]) { if (name === 'restartEncounter' && !held.inStep && held.h && !held.stale) backToStart(held); const r = was.apply(this, a); if (!held.inStep) held.stale = name; return r; };
  }
  // THE BOX WORLD LISTED for the picture (engine.world.upload, between ticks) is also where the Rust game lists its own and frees the frame boxes gone by:
  // a listing since the last step is made there as the next step opens
  { const W = G.engine.world; for (const name of ['upload', 'relist']) { const was = W[name]; if (typeof was === 'function') W[name] = function (this: unknown, ...a: unknown[]) { held.uploaded = true; return was.apply(this, a); }; } }   // (`relist`: an undrawn step of a script's burst lists the world and packs nothing: main.ts)
  // THE LIGHTS' UPDATE (a flash's envelope and its retirement) is made on the Rust side as its step ends (`face_step`'s LIGHTS flag); the lights are rows of
  // the one block, so made here too it would be made twice
  { const L = G.engine.lights; L.update = function () { /* the Rust side made it */ }; }
  // THE LIGHT PROBE'S FRAME (render/probe.ts encode: the table sent to the GPU and emptied): the Rust game's table is emptied with it
  { const P = G.sensors?.probeOf?.(); if (P && typeof P.encode === 'function') { const was = P.encode; P.encode = function (this: any, ...a: unknown[]) { const n = this.pending.length; const r = was.apply(this, a); if (n && held.h) call('face_probe_took', () => X!.face_probe_took(held.h)); return r; }; } }
  // AN ORDER GIVEN FROM OUTSIDE A TICK (a tool's `__orders.give`, a panel's button) goes to the Rust game at once, in the words the headless host gives
  // it (rust/game_run `give`), and the puppet takes the state again. Inside a tick the menu's own keys are in the record and are the Rust game's.
  // A wall, a thing, options and one man by object have no words there: such an order runs through the menu's own `give` on the puppet, whose rows ask the
  // Rust game (tools/blitz/ask.ts: `issue` is made there, and the puppet takes the state again); what `give` itself writes beside the row's `issue` (its
  // line of the log, a man's word) is then the puppet's alone until the next apply. STAND-IN for a `give` of the face that takes a target as bytes.
  { const O = G.orders, give = O.give; O.give = function (this: any, id: string, t: any, who: any, opts: any) {
      if (held.inStep) return give.call(this, id, t, who, opts);
      if (held.stale || !held.h) start(held);
      const T0 = t ?? this.targetAt(g.aimPoint), w = who === undefined ? '-' : typeof who === 'object' ? `g${g.guards.indexOf(who)}` : String(who);
      const tail = T0.kind === 'door' ? ` door ${g.doors.list.indexOf(T0.door)} ${T0.side} book${bookOf(g).ids.get('DoorId')?.get(T0.door)}` : T0.kind === 'none' && T0.man ? ` man ${g.guards.indexOf(T0.man)}` : T0.kind === 'foe' ? ` man ${g.guards.indexOf(T0.foe)}` : (T0.wall || T0.thing) ? ' UNSUPPORTED-wall-or-thing' : '';
      const line = `${RUST_GAME.ticks + 1} ${id} ${w} ${T0.kind} ${T0.at.join(' ')}${tail}${opts ? ' UNSUPPORTED-opts' : ''}`;
      if (line.includes('UNSUPPORTED') || /^g\d/.test(w)) { RUST_GAME.gives.push(`BY THE MENU'S QUESTIONS ${line}`); return give.call(this, id, t, who, opts); }
      call('face_give', () => hand(enc.encode(line), (p, n) => X!.face_give(held.h, RUST_GAME.ticks + 1, p, n)));
      const said = outText(), m = said.match(/-> (None|Some\("((?:[^"\\]|\\.)*)"\))\s*$/);
      RUST_GAME.gives.push(`${line} ${said.trim().split('->').pop()}`); if (RUST_GAME.gives.length > 64) RUST_GAME.gives.shift();
      grown(g); apply(held, false);
      return m && m[1] !== 'None' ? JSON.parse(`"${m[2]}"`) : null; }; }
}

/** THE RUST GAME STARTED FROM THE PUPPET AS IT STANDS: the whole state across once, over the block that already stands in the puppet's slot */
function start(S: Held): void {
  const g = S.g, t0 = performance.now();
  HOOKS ??= { classes: classTable(), values: valueTable() };
  g.sim.settle(g);   // (as a tick's own opening: the men's poses at home, the block bound)
  const slot = g.sim.bind();
  if ((PLACE as unknown as { slot: number }).slot !== slot) throw new Error(`rust=1: the game's block stands in slot ${slot} of the module and the world kept for the Rust game is ${(PLACE as unknown as { slot: number }).slot}: the two must be one number (rust/game box_mirror_take)`);
  if (S.h) { X!.face_drop(S.h); S.h = 0; }
  // (the bridge's book of which object has which row is a game's for as long as the Rust game it was exported into: a new Rust game, a new book)
  { const bk = bookOf(g); if (bk) { bk.ids.clear(); bk.lists.clear(); bk.first.clear(); bk.loose.clear(); } }
  { const t = performance.now(); S.snap0 = takeSnapshot(g); RUST_GAME.snapMs = performance.now() - t; }   // (the game as the TypeScript stood it: what a restart from a card goes back to, `backToStart`)
  const state = exportGame(g, reg());
  RUST_GAME.notCarried = [...report()].map(([k, v]) => `${k} x${v.n}`);
  const h = call('face_new_at', () => hand(state, (p, n) => X!.face_new_at(p, n, slot)));
  if (!h) throw new Error('rust=1: the face made no game (no such slot)');
  S.lightOf = new Map((bookOf(g).listOf('LightId') as Record<string, any>[]).map((l, i) => [i, l]));
  S.h = h; S.slot = slot; S.stale = ''; S.uploaded = true; S.ctx = { p: 0 }; S.rows = puppetRows(S.ctx);
  grown(g);
  S.asks = null;
  apply(S, true);
  // (the menu's questions: asked of this Rust game by its rows' IDs in this book, so installed anew at every start)
  S.asks = installAsks(g, reg(), (kind, row, seat, b) => { call('face_ask', () => hand(b, (p, n) => X!.face_ask(S.h, kind, row, seat, p, n))); return outBytes().slice(); }, () => { grown(g); if (!S.inStep) apply(S, false); });
  // (room asked for and given back once the game stands: the allocator keeps it, so a step seldom has to grow the memory, which detaches every view over it)
  X!.face_reserve(64 * 1048576); grown(g);
  RUST_GAME.handle = h; RUST_GAME.slot = slot; RUST_GAME.starts++; RUST_GAME.startMs = performance.now() - t0; RUST_GAME.startBytes = state.length;
  console.info(`rust=1: the Rust game started from the page's (${state.length} bytes of state, slot ${slot}) in ${RUST_GAME.startMs.toFixed(0)} ms${RUST_GAME.notCarried.length ? `; the bridge could not carry ${RUST_GAME.notCarried.length} kinds of value (window.__rust.notCarried)` : ''}`);
}

/** WHAT STANDS ROUND EVERY APPLY, the filtered one and the whole one: the menu's questions for the rows an apply made, the stand-ins for what the
 *  bridge does not carry, the picture's lights, and what the host's writes between ticks are told by */
function after(S: Held): void {
  const g = S.g, G = g as unknown as Record<string, any>;
  // THE MENU'S QUESTIONS (tools/blitz/ask.ts): every order row and the squad's posture have closures that ask the Rust game by the row's ID; a row or a
  // posture this apply made gets its own here
  // STAND-IN: ask.ts `after` gives a row's `standsIn` its `when` where `when !== whens.get(d)`; for a row an apply has just made both are undefined, so
  // it is passed over and orders.ts rowsFor throws (`d.standsIn?.when is not a function`: seen on the row `loud`). A `when` that is missing is made
  // null first, which is not what the memo holds. Belongs in ask.ts (`typeof d.standsIn.when !== 'function' ||`).
  for (const d of (G.orders?.defs ?? []) as Record<string, any>[]) if (d.standsIn && d.standsIn.when === undefined) d.standsIn.when = null;
  S.asks?.after();
  // STAND-IN: A DOOR'S KEPT CENTRE is a memo in two halves: the angle it was made for and its x and z are columns of the block (sim/doorshome.ts
  // posAngle, posX, posZ), the array `Door.posAt` an object's field that is on neither list of the filtered export. The puppet's array stays as the
  // start left it, and doors.ts posAside (a look ahead between ticks) writes it back over the columns the Rust tick wrote: seen as doors at [0, 1, 0]
  // and the block's `doors` part parting from the TypeScript page's. After an apply the array is made from the columns. Belongs in the filtered export
  // (`doors::Door.pos_at`), or better in doors.ts: the array made from the columns where it is read.
  for (const dr of (G.doors?.list ?? []) as Record<string, any>[]) { const x = dr.g(DC.posX), z = dr.g(DC.posZ), at = dr.posAt; if (!Number.isNaN(dr.g(DC.posAngle)) && (at[0] !== x || at[2] !== z)) dr.posAt = [x, 1.0, z]; }
  // (…and the other way: a reader between ticks, the HUD's rows, asks a door's centre after its leaf has swung and makes the memo anew: the columns
  // then say 'kept for this angle' and the Rust game's own array is the old one. The angle each memo was kept for as the apply ends is noted; one that
  // differs as the next step opens was made here, and is put at no angle, so the Rust tick makes it again where it asks. Belongs in rust/game doors: the
  // array read from the columns.)
  S.doorKept = ((G.doors?.list ?? []) as Record<string, any>[]).map(dr => dr.g(DC.posAngle));
  lightsIn(S);
  S.flags = FLAGS.map(k => !!G[k]); S.launch = G.player?.launchHeld ?? null;   // (the host's switches as the Rust game has them now: what differs by the next step, the host wrote)
  S.putBacks = PUT_BACKS.n;   // (the apply counts itself as a put-back, for the box world's copy: any other count by the next step is a snapshot somebody put back)
  S.loads = G.chronicle?.counts?.loads | 0;   // (…and a checkpoint read from a file counts itself here: savestate.ts loadState)
}
/** THE FILTERED EXPORT ONTO THE PUPPET, its out events dispatched. Answers the ms of the export, the apply and the dispatch. */
function apply(S: Held, first: boolean): { ex: number; ap: number; out: number; bytes: number } {
  const g = S.g, G = g as unknown as Record<string, any>;
  let t = performance.now();
  call('face_puppet', () => X!.face_puppet(S.h, first ? 1 : 0)); grown(g);
  const blob = outBytes(), ex = performance.now() - t;
  t = performance.now();
  const R = applyGame(g, blob, reg(), { hooks: HOOKS, puppet: { lists: lists(), start: first, rows: S.rows, ctx: S.ctx, slowTable: SLOW, vals: VALS } });
  after(S);
  const ap = performance.now() - t; t = performance.now();
  const ev: { arm: string; e: any }[] = R.out ?? [];
  if (ev.length && !first) { RUST_GAME.outEvents += ev.length;
    for (const x of ev) { RUST_GAME.outArms[x.arm] = (RUST_GAME.outArms[x.arm] ?? 0) + 1;
      // (one event that throws must not take the frame with it: it is named once and the rest go on)
      try { (OUT_OWN[x.arm] ?? OUT[x.arm])(g, x.e); } catch (e) { RUST_GAME.outThrew[x.arm] ??= String((e as Error)?.stack ?? e).slice(0, 600); } } }
  return { ex, ap, out: performance.now() - t, bytes: blob.length };
}
/** STAND-IN: A RESTART FROM OUTSIDE A TICK (a card's button: `Game.restart` → `restartEncounter`) cannot run on the puppet as play left it. The filtered
 *  apply makes a value with its listed fields only, and neither apply makes a container the Rust game holds empty (the importer keeps `undefined`
 *  where the TypeScript had none and the Rust value is blank), so the TypeScript's own reset code meets values with fields missing: seen as
 *  `st.wedges is not iterable` (digin.ts digInReset) and 'row undefined of the men left twice' (the player's `Character.man_row` back as nothing).
 *  So the puppet's objects are first put back to THE GAME AS THE TYPESCRIPT STOOD IT at the last start (a snapshot taken there: game/rollback.ts), with the
 *  clock and the whole block (the die's place, the weather, the doors, the furniture) kept as play left them, as a TypeScript restart finds them; then `restartEncounter` runs as it always did, and the next step starts
 *  the Rust game from it. What else a TypeScript restart carries over in its objects from the try that ended (seen: the orders' log) is lost here. Belongs in the bridge (values made whole, a
 *  man's row) or in the Rust game (the restart made there, with the hooks a new man needs on the puppet). */
function backToStart(S: Held): void {
  if (!S.snap0) return;
  // (the block is whole wherever it stands, and a TypeScript restart resets its own parts of it: it is kept as play left it, the die's place with it)
  const g = S.g, time = g.time, block = g.sim.w.slice();
  restoreSnapshot(S.snap0);
  (g as unknown as { time: number }).time = time; g.sim.bind(); g.sim.w.set(block);
  RUST_GAME.backs++;
}
/** THE WHOLE STATE ONTO THE PUPPET (`face_export` → the bridge's whole apply), for a tool that is about to read the puppet for something the page does
 *  not show (`window.__rust.whole()`). A third of a second, outside any step. It does not make the puppet fit for a TypeScript start: `backToStart`. */
function whole(S: Held): void {
  const g = S.g, G = g as unknown as Record<string, any>, t = performance.now();
  call('face_export', () => X!.face_export(S.h)); grown(g);
  // STAND-IN: A MAN'S ROW OF THE BLOCK (sim/men/cols.ts ManCols `S`, `r`) is written by the whole apply from the bridge's `Character.man_row`, and for
  // the player's own character that came back as nothing after play (seen: `r` undefined). Rows change hands only in a start or inside a tick, so a
  // man keeps the row he had where the apply leaves him none. Belongs in the bridge.
  const men: Record<string, any>[] = []; for (const s of g.seats) men.push(s.view.player, s.view.player.char); for (const m of [...g.guards, ...(G.civilians ?? [])]) men.push(m, m.char);
  const rows = men.map(m => [m.S, m.r]);
  applyGame(g, outBytes(), reg(), { hooks: HOOKS });
  men.forEach((m, i) => { if (m.r === undefined || m.S === undefined) { m.S = rows[i][0]; m.r = rows[i][1]; RUST_GAME.rowsKept++; } });
  after(S);
  RUST_GAME.wholes++; RUST_GAME.wholeMs = performance.now() - t;
}
/** STAND-IN: the generated dispatcher calls `g.saves.request(why, label)`; the method is `request(g, why, label)` (savestate.ts). Belongs in out-dispatch.ts. */
const OUT_OWN: Record<string, (g: any, e: any) => void> = {
  CheckpointsRequest: (g, e) => { g.saves?.request(g, e.why, e.label); },
};

/** the host's switches `face_poke` knows, in its order (page.rs) */
const FLAGS = ['godMode', 'aiEnabled', 'infiniteAmmo', 'playerInvisible'] as const;
/** STAND-IN: THE PICTURE'S LIGHTS (page.rs face_lights_out says why): the engine's list made again in the Rust game's order, each light's numbers written
 *  onto the object the puppet has for that row, a new object for a row made inside a tick (a muzzle flash). `steady`, `portal` and the GPU's `slot` stay
 *  the puppet's own; the infrared list and the fog of war's team list are not carried. Belongs in the filtered export. */
function lightsIn(S: Held): void {
  const G = S.g as unknown as Record<string, any>, list: Record<string, any>[] | undefined = G.engine?.lights?.lights; if (!list) return;
  X!.face_lights_out(S.h); const b = outBytes(), d = new DataView(b.buffer, b.byteOffset, b.byteLength), n = d.getUint32(0, true), REC = 4 + 32 * 8;
  if (b.length !== 4 + n * REC) throw new Error(`rust=1: the lights' hand-over is ${b.length} bytes for ${n} lights`);
  const book: Record<string, any>[] = bookOf(S.g).listOf('LightId');
  list.length = 0;
  for (let i = 0, at = 4; i < n; i++, at += REC) {
    const id = d.getUint32(at, true), f = (k: number) => d.getFloat64(at + 4 + 8 * k, true), opt = (k: number): number | undefined => { const v = f(k); return Number.isNaN(v) ? undefined : v; };
    let l = S.lightOf.get(id);
    if (!l) { l = book[id] ?? { name: '', group: '', steady: undefined, slot: -1 }; S.lightOf.set(id, l); RUST_GAME.lightsMade++; }
    l.kind = f(0); l.pos = [f(1), f(2), f(3)]; l.dir = [f(4), f(5), f(6)]; l.color = [f(7), f(8), f(9)]; l.intensity = f(10); l.range = f(11); l.innerDeg = f(12); l.outerDeg = f(13); l.radius = f(14);
    l.volumetric = f(15); l.owner = f(16); l.enabled = f(17) !== 0; l.fixtureBox = f(18); l.flicker = f(19); { const r = opt(20); if (r !== undefined || l.row !== undefined) l.row = r; }
    l.ttl = f(21); l.age = f(22); l.peakIntensity = f(23); { const t = opt(24); if (t !== undefined || l.transient !== undefined) l.transient = t === undefined ? undefined : t !== 0; }
    l.broad = f(25) !== 0; { const m = opt(26); if (m !== undefined || l.markRef !== undefined) l.markRef = m; } l.card = f(27) !== 0;
    for (const [k, name] of [[28, 'glassMask'], [29, 'fog'], [30, 'fogGhost'], [31, 'fogBy']] as const) { const v = opt(k); if (v !== undefined || l[name] !== undefined) l[name] = v; }
    list.push(l);
  }
}
const LIGHTS = 2, VIEW = 4, UPLOAD = 1;
/** ONE TICK, in place of `Game.stepSeats` */
function step(S: Held, dt: number, cmds: readonly PlayerCmd[]): void {
  const g = S.g, G = g as unknown as Record<string, any>;
  if (!S.stale && S.h && PUT_BACKS.n !== S.putBacks) S.stale = 'a snapshot was put back';
  if (!S.stale && S.h && ((G.chronicle?.counts?.loads | 0) !== S.loads)) S.stale = 'a checkpoint was loaded';
  if (S.stale || !S.h) start(S);
  S.inStep = true;
  try {
    let t = performance.now();
    { const P = PLACE as unknown as { take(): boolean; slot: number }; if (P.take() || P.slot !== S.slot) throw new Error(`rust=1: the world kept for the Rust game (${S.slot}) was taken by another copy of the box world`); }
    if (g.sim.bind() !== S.slot) throw new Error(`rust=1: the game's block moved out of slot ${S.slot} (another game took it): the Rust game stands there`);
    knobsToRust();
    { const L = (G.doors?.list ?? []) as Record<string, any>[]; for (let i = 0; i < L.length && i < S.doorKept.length; i++) if (!Object.is(L[i].g(DC.posAngle), S.doorKept[i])) L[i].p(DC.posAngle, NaN); }
    // THE HOST'S SWITCHES written between ticks (the options' and the pause card's god mode / AI / ammunition rows; the pause card letting go of the launcher's
    // held button: host/loop.ts): what differs from what the last apply left goes over by `face_poke`
    FLAGS.forEach((k, i) => { if (!!G[k] !== S.flags[i]) { X!.face_poke(S.h, i, G[k] ? 1 : 0); RUST_GAME.pokes++; } });
    if (S.launch !== null && (G.player?.launchHeld ?? null) === null) { X!.face_poke(S.h, 4, 0); RUST_GAME.pokes++; }
    // the light probe's answers, as the GPU has handed them back by now
    { const P = G.sensors?.probeOf?.(), Rm: Map<string, number[]> | undefined = P?.results;
      if (Rm && Rm.size) { let n = 4; const keys: Uint8Array[] = []; for (const k of Rm.keys()) { const b = enc.encode(k); keys.push(b); n += 4 + b.length + 24; }
        const b = new Uint8Array(n), d = new DataView(b.buffer); d.setUint32(0, Rm.size, true); let at = 4, i = 0;
        for (const v of Rm.values()) { const k = keys[i++]; d.setUint32(at, k.length, true); at += 4; b.set(k, at); at += k.length; d.setFloat64(at, v[0], true); d.setFloat64(at + 8, v[1], true); d.setFloat64(at + 16, v[2], true); at += 24; }
        hand(b, (p, m) => X!.face_probe_results(S.h, p, m)); } }
    const hb = hostHalf(g, reg(), cmds); hand(hb, (p, n) => X!.face_host_half(S.h, p, n));
    const tHost = performance.now() - t; t = performance.now();
    const flags = LIGHTS | VIEW | (S.uploaded ? UPLOAD : 0); S.uploaded = false;
    call('face_step', () => X!.face_step(S.h, dt, flags)); grown(g);
    const tStep = performance.now() - t;
    const A = apply(S, false);
    t = performance.now();
    dressStep(g, dt, S.side);
    // what the tick asked of the light probe, for the frame's GPU pass (the table is the Rust game's until the pass takes it: `face_probe_took`)
    { const P = G.sensors?.probeOf?.(); if (P && Array.isArray(P.pending)) { X!.face_probe_asks(S.h); const b = outBytes(), d = new DataView(b.buffer, b.byteOffset, b.byteLength); let at = 4; const n = d.getUint32(0, true); P.pending.length = 0;
        for (let i = 0; i < n; i++) { const kl = d.getUint32(at, true); at += 4; const key = dec.decode(b.subarray(at, at + kl)); at += kl; const pos = [d.getFloat64(at, true), d.getFloat64(at + 8, true), d.getFloat64(at + 16, true)]; at += 24; const skipOwner = d.getFloat64(at, true); at += 8;
          const hasB = b[at++]; let bb: number[] | undefined; if (hasB) { bb = [d.getFloat64(at, true), d.getFloat64(at + 8, true), d.getFloat64(at + 16, true)]; at += 24; } const rank = d.getFloat64(at, true); at += 8;
          P.pending.push(bb ? { key, pos, skipOwner, b: bb, rank } : { key, pos, skipOwner, rank }); }
        RUST_GAME.probeAsks += n; } }
    // THE AUTOSAVE is the host's and decides off the frame as it now stands (savestate.ts Checkpoints.tick, the TypeScript tick's last line): on the
    // puppet. Its one write into the game is the chronicle's row, made on the Rust side (`face_checkpoint`), and the puppet takes the state again.
    { const C = G.chronicle?.counts, s0 = C?.saves; try { G.saves?.tick(g, dt); } catch (e) { RUST_GAME.outThrew['saves.tick'] ??= String((e as Error)?.stack ?? e).slice(0, 600); }
      if (C && C.saves !== s0) { const E = G.chronicle.events, e = E[E.length - 1], label = e?.kind === 'checkpoint' ? String(e.label ?? e.data?.label ?? '') : '';
        call('face_checkpoint', () => hand(enc.encode(label), (p, n) => X!.face_checkpoint(S.h, p, n))); grown(g); apply(S, false); } }
    const tDress = performance.now() - t;
    const L = RUST_GAME.last, M = RUST_GAME.sum; L.host = tHost; L.step = tStep; L.export = A.ex; L.apply = A.ap; L.out = A.out; L.dress = tDress; L.bytes = A.bytes;
    M.host += tHost; M.step += tStep; M.export += A.ex; M.apply += A.ap; M.out += A.out; M.dress += tDress; M.bytes += A.bytes;
    RUST_GAME.ticks++;
    if (tStep > RUST_GAME.worst.step) { RUST_GAME.worst.step = tStep; RUST_GAME.worst.at = RUST_GAME.ticks; } if (A.ap > RUST_GAME.worst.apply) RUST_GAME.worst.apply = A.ap;
  } finally { S.inStep = false; }
}

// ---- for a check: the die's count and the block's parts, hashed as tools/blitz/mission.ts hashes them
/** the die's count of draws and the hash of each part of the game's block, by name (the Rust game's own words where it steps) */
export function rustHashes(): { die: number; time: number; parts: Record<string, number> } | null {
  if (!X || !H || !H.h) return null;
  const names: string[] = []; for (let i = 0; i < X.face_parts(); i++) { X.face_part_name(i); names.push(outText()); }
  const at = X.face_part_hashes(H.slot) >>> 2, w = new Uint32Array(MEM!.buffer), parts: Record<string, number> = {};
  names.forEach((n, i) => { parts[n] = w[at + i]; });
  return { die: X.face_die_n(H.h), time: X.face_time(H.h), parts };
}
/** FOR A TOOL THAT WROTE THE PUPPET BY NAME between ticks (a staging hook, a panel's button): the Rust game is started again from the puppet as the next
 *  step opens. Only as good as the puppet is whole: what the filtered export does not carry stands on the puppet as it stood at the last start. */
export function rustResync(why = 'a tool asked'): void { if (H) H.stale = why; }
/** …and the other way, before a tool READS the puppet for something the page does not show: the Rust game's whole state onto it */
export function rustWhole(): void { if (H && H.h && !H.stale && !H.inStep) whole(H); }
/** mean ms a step of each stage, over the steps so far */
export function rustMeans(): Record<string, number> { const n = Math.max(1, RUST_GAME.ticks), M = RUST_GAME.sum; return { ticks: RUST_GAME.ticks, host: M.host / n, step: M.step / n, export: M.export / n, apply: M.apply / n, out: M.out / n, dress: M.dress / n, bytes: M.bytes / n }; }
