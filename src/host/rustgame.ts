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
// A START. The TypeScript stands the game as it always did (the level, the mission, the squad, the kit), then the whole state goes across once (the
// bridge's `exportGame` → `face_new_at`) and the first filtered export is applied. Whatever stands the game again from outside a tick (a mission's own
// staging, a show) marks the Rust game stale, and the next step starts it again from the puppet.
//
// A GAME KEPT. A checkpoint is THE RUST GAME'S OWN: the save controller (game/savestate.ts, which runs here on the puppet with its slots, labels and
// rhythm as ever) takes and puts back through `SNAPS`, and what it holds in a slot is a number of a game kept inside the module (`face_keep`: a clone of
// the whole Rust `Game` and its block; nothing of the puppet is walked, so a save costs no apply). A load, a retry from a checkpoint and a restart are
// `face_back` (the kept game stands again) and ONE whole apply onto the puppet; the lines the controller then runs on the puppet (the run's counts, the
// chronicle's row, a tutorial's program to be started again) are made on the Rust side too (`face_loaded`). A RESTART is the game kept as the first step
// opened. A FILE is the bridge's bytes of a kept game and its block under a version word (`rustFile` / `rustFileIn`).
//
// Every special case here is marked STAND-IN with its reason and where it belongs.
import { simAdopt, type SimExports } from '../sim/sim';
import { BoxMirror } from '../scene/boxMirror';
import { PUT_BACKS } from '../math/side';
import { knobRegistry } from '../game/knobs';
import { knobsToRust } from '../game/rustknobs';
import { DC } from '../sim/doorshome';
import { SNAPS, SAVETIMELINE, levelFingerprint, type SlotInfo } from '../game/savestate';
import { CLEANRESTART } from '../game/cleanrestart';
import type { Snapshot } from '../game/rollback';
import type { Game } from '../game/game';
import { sampleCmd, stillCmd, type PlayerCmd } from '../game/cmd';
import { dressStep, DRESS_MS } from './rustdress';
import { exportGame, hostHalf, bookOf, report, SCHEMA } from 'tps-bridge/export';
import { applyGame } from 'tps-bridge/import';
import { classTable, valueTable } from 'tps-bridge/classes';
import { puppetRows, SLOW, VALS } from 'tps-bridge/puppet-apply';
import { OUT, OUT_HOST } from 'tps-bridge/out-dispatch';
import { makePoke } from 'tps-bridge/poke';
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
// (an address past 2 GB comes back from the module as a negative number: `>>> 0`)
const outBytes = (): Uint8Array => { const p = X!.face_out_ptr() >>> 0, n = X!.face_out_len() >>> 0; return u8().subarray(p, p + n); };
const outText = (): string => dec.decode(outBytes());
const panicText = (): string => { try { const p = X!.face_panic_ptr() >>> 0, n = X!.face_panic_len() >>> 0; return dec.decode(u8().subarray(p, p + n)); } catch { return ''; } };
function hand<T>(b: Uint8Array, f: (p: number, n: number) => T): T { const p = X!.face_alloc(b.length) >>> 0; u8().set(b, p); try { return f(p, b.length); } finally { X!.face_free(p, b.length); } }
/** a call into the face: a trap is the module's abort on a panic, whose words it kept */
function call<T>(what: string, f: () => T): T { try { return f(); } catch (e) { const w = panicText(); throw new Error(w ? `the Rust game panicked in ${what}: ${w}` : `${what}: ${String((e as Error)?.message ?? e)}`); } }

/** what the page says of the Rust game, on `window.__rust` */
export const RUST_GAME = {
  on: false, bytes: 0, fetchMs: 0, compileMs: 0, startMs: 0, startBytes: 0, handle: 0, slot: -1, ticks: 0, starts: 0, grew: 0, memory: 0,
  /** ms of the last step and summed over every step: the host's half, the tick, the filtered export, the apply, the out events, the dress step */
  last: { host: 0, step: 0, export: 0, apply: 0, out: 0, dress: 0, bytes: 0 }, sum: { host: 0, step: 0, export: 0, apply: 0, out: 0, dress: 0, bytes: 0 }, worst: { step: 0, apply: 0, at: 0 },
  pokes: 0, poked: {} as Record<string, number>, pokeRefused: {} as Record<string, string>, lightsMade: 0, wholes: 0, wholeMs: 0, rowsKept: 0, nulls: 0, lightsWritten: 0,
  /** games kept (a save, the start) and their cost: how many were taken, ms of the last take and the worst, how many the module holds now; games put back,
   *  ms of the last (the kept game stood again, the whole apply, the hand-over of the controller's lines) */
  /** the warm-up behind a start card (`rustWarm`): ms of the Rust game's start, of the applies and of the ticks stepped and thrown away, and how many */
  warm: { n: 0, startMs: 0, applyMs: 0, tickMs: 0, ticks: 0, firstTickMs: 0, lastTickMs: 0 },
  keeps: 0, keepMs: 0, keepWorst: 0, kept: 0, backs: 0, backMs: { back: 0, whole: 0, hand: 0 }, restarts: 0, shelfSent: 0, keepAs: 0 as 0 | 1,
  /** WHERE A STEP'S TIME GOES, finer (`prof.on`, for a check: the tick is then four calls into the module where play makes one). Sums in ms over `n` steps:
   *  the host's half (the knobs · the light probe's answers handed in · the bridge's hand-over built and taken), the listing, the tick, the lights'
   *  update, the rails' view, the filtered export, the apply (the bridge's own walk, by its phases · the menu's questions · the doors' stand-in · the
   *  lights' stand-in), the out events, the dress step, the probe's asks read back, the save controller */
  prof: { on: false, n: 0, knobs: 0, probeIn: 0, hostHalf: 0, upload: 0, tick: 0, lights: 0, view: 0, asked: 0, export: 0, applyWalk: 0, applyRows: 0, applyMain: 0, applyTables: 0, asks: 0, doors: 0, lightsIn: 0, out: 0, dress: 0, probeOut: 0, saves: 0, phases: {} as Record<string, number> },
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
  for (const n of ['face_new_at', 'face_puppet', 'face_host_half', 'face_step', 'face_ask', 'face_probe_asks', 'face_lights_out', 'face_poke', 'face_write', 'face_keep', 'face_back', 'face_loaded', 'face_saves_in', 'face_saves_asked']) if (typeof X[n] !== 'function') throw new Error(`?rust=1: game_face.wasm has no ${n}: it was built from another tree (tools/blitz/face-build.ts builds this one's)`);
  simAdopt(X, RUST_GAME.compileMs);
  PLACE = new BoxMirror(); (PLACE as unknown as { take(): boolean }).take();
  RUST_GAME.on = true; RUST_GAME.memory = MEM.buffer.byteLength;
  console.info(`rust=1: the Rust game's module, ${RUST_GAME.bytes} bytes, fetched in ${RUST_GAME.fetchMs.toFixed(0)} ms and compiled in ${RUST_GAME.compileMs.toFixed(0)} ms; memory ${(RUST_GAME.memory / 1048576).toFixed(0)} MB`);
}

// ---- a game
/** what a slot of the save controller holds here in a snapshot's place: the number of a game kept in the module, and the records of the game that are
 *  the PAGE'S OWN (`OWN`, below), which no kept Rust game holds */
interface KeptGame { game: Game; rust: number; own: unknown[] }
/** THE RECORDS OF THE GAME THAT ONLY THE PAGE WRITES (the dress step's: SWAP.md, "the par clock, the verdict"): the Rust game does not hold them and no apply
 *  writes them, so a game kept without them comes back under the verdict and the par of the moment that was abandoned (seen: after a load the save
 *  controller refused every save, 'the run is over', by the verdict of the death the load undid). Each is copied with a kept game and written back with
 *  it: its own numbers, lists, maps and sets by value; what it holds OF the game (a man, the mission, a row of the orders' log) by reference. */
const OWN = ['verdict', 'par'] as const;
function ownCopy(v: any, depth = 0): any {
  if (v === null || typeof v !== 'object') return v;
  if (Array.isArray(v)) return depth > 4 ? v : v.map(x => ownCopy(x, depth + 1));
  if (ArrayBuffer.isView(v)) return (v as unknown as { slice(): unknown }).slice();
  if (v instanceof Map) return new Map([...v].map(([k, x]) => [k, ownCopy(x, depth + 1)]));
  if (v instanceof Set) return new Set(v);
  const P = Object.getPrototypeOf(v); if ((P === Object.prototype || P === null) && depth <= 4) { const o: Record<string, unknown> = {}; for (const k of Object.keys(v)) o[k] = ownCopy(v[k], depth + 1); return o; }
  return v;
}
function ownKeep(g: Game): unknown[] { const G = g as unknown as Record<string, any>; return OWN.map(name => { const o = G[name]; if (!o) return null; const c: Record<string, unknown> = {}; for (const k of Object.keys(o)) if (k !== 'g') c[k] = ownCopy(o[k]); return c; }); }
function ownPut(g: Game, K: unknown[] | undefined): void { const G = g as unknown as Record<string, any>; OWN.forEach((name, i) => { const o = G[name], c = K?.[i] as Record<string, unknown> | null | undefined; if (!o || !c) return; for (const k of Object.keys(c)) o[k] = ownCopy(c[k]); }); }
interface Held { g: Game; lastCmds: readonly PlayerCmd[] | null; lightsWas: Uint8Array | null; kept: Set<number>; startKept: number; startOwn: unknown[]; back: { label: string; loads: number; saves: number } | null; took: boolean; saves: number; shelfSig: string; doorKept: number[]; lightOf: Map<number, Record<string, any>>; poke: ((obj: unknown, path: string | (string | number)[], value: unknown) => void) | null; watch: Watched[]; launch: unknown; putBacks: number; loads: number; h: number; slot: number; stale: string; uploaded: boolean; ctx: { p: number }; rows: unknown; inStep: boolean; side: { list: unknown[]; of: unknown; base: number }; asks: { after(): void } | null }
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
  H = { g, lastCmds: null, lightsWas: null, kept: new Set(), startKept: 0, startOwn: [], back: null, took: false, saves: 0, shelfSig: '', doorKept: [], lightOf: new Map(), poke: null, watch: [], launch: null, putBacks: -1, loads: 0, h: 0, slot: -1, stale: 'the game was stood', uploaded: true, ctx: { p: 0 }, rows: null, inStep: false, side: { list: [], of: null, base: 0 }, asks: null };
  const held = H;
  G.stepSeats = (dt: number, cmds: readonly PlayerCmd[]) => step(held, dt, cmds);
  // A START FROM OUTSIDE A TICK: the TypeScript stands or restores the game on the puppet as it always did; the Rust game is started again from it as the
  // next step opens. (Inside a step these are the Rust game's own and never run here.)
  // (`restartEncounter` from outside a tick, while a Rust game stands, is the restart below and the TypeScript's lines do not run: they make the player's
  //  man anew and give the squad its 'fall in' through the menu's rows, and under the word those rows ask the Rust game (tools/blitz/ask.ts), which has
  //  never seen that man. Measured: the ask panics in the Rust row's `issue` on a man's ID past its table, and the game is lost with it.)
  for (const name of ['restartEncounter', 'loadState']) {
    const was = G[name]; if (typeof was !== 'function') continue;
    G[name] = function (this: unknown, ...a: unknown[]) {
      if (name === 'restartEncounter' && !held.inStep && held.h && !held.stale && held.startKept) { toStart(held); return; }
      const r = was.apply(this, a); if (!held.inStep) held.stale = name; return r; };
  }
  // THE PLAYER'S RESTART (a card's button, the panel's: `Game.restart`): the game kept as the first step opened stands again
  { const was = G.restart; G.restart = function (this: unknown, said?: string) { if (held.inStep || !held.h || held.stale || !held.startKept) return was.call(this, said); toStart(held, said); }; }
  // A SAVE AND A LOAD are the Rust game's (game/savestate.ts SNAPS): the controller's slot holds the number of a game kept in the module
  SNAPS.own = true;
  SNAPS.take = gg => { if (gg !== g) throw new Error('rust=1: a save of another game than the one the Rust game steps'); return take(held) as unknown as Snapshot; };
  SNAPS.put = s => { const k = s as unknown as KeptGame; if (k.game !== g || !k.rust) throw new Error('rust=1: that save is not a game kept by the Rust game'); put(held, k); };
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
      // (before the Rust game is started, and while it waits to be started again, the puppet IS the game: a mission's own staging gives its orders
      //  here, and the start takes them across with everything else)
      if (held.stale || !held.h) return give.call(this, id, t, who, opts);
      settle(held);
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
  const state = exportGame(g, reg());
  RUST_GAME.notCarried = [...report()].map(([k, v]) => `${k} x${v.n}`);
  const h = call('face_new_at', () => hand(state, (p, n) => X!.face_new_at(p, n, slot)));
  if (!h) throw new Error('rust=1: the face made no game (no such slot)');
  S.lightOf = new Map((bookOf(g).listOf('LightId') as Record<string, any>[]).map((l, i) => [i, l])); S.lightsWas = null;
  S.h = h; S.slot = slot; S.stale = ''; S.uploaded = true; S.ctx = { p: 0 }; S.rows = puppetRows(S.ctx); S.back = null; S.took = false; S.shelfSig = '';
  grown(g);
  S.asks = null;
  apply(S, true);
  // THE START KEPT: what a restart goes back to. (Every game kept of the Rust game that stood before is gone with it: a slot that still names one is refused.)
  for (const k of S.kept) X!.face_kept_drop(k); S.kept.clear();
  S.startKept = keep(S); S.startOwn = ownKeep(g);
  // (the menu's questions: asked of this Rust game by its rows' IDs in this book, so installed anew at every start)
  S.asks = installAsks(g, reg(), (kind, row, seat, b) => { call('face_ask', () => hand(b, (p, n) => X!.face_ask(S.h, kind, row, seat, p, n))); return outBytes().slice(); }, () => { grown(g); if (!S.inStep) apply(S, false); });
  // (the write by path: by the rows' IDs in this book too)
  S.poke = makePoke(g, reg(), b => { call('face_write', () => hand(b, (p, n) => X!.face_write(S.h, p, n))); });
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
  const PF = RUST_GAME.prof; let tp = PF.on ? performance.now() : 0;
  S.asks?.after();
  if (PF.on) { const n = performance.now(); PF.asks += n - tp; tp = n; }
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
  if (PF.on) { const n = performance.now(); PF.doors += n - tp; tp = n; }
  lightsIn(S);
  if (PF.on) PF.lightsIn += performance.now() - tp;
  S.watch = watched(g); S.launch = G.player?.launchHeld ?? null;   // (what the page's panels write, as the Rust game has it now: what differs by the next step, the page wrote)
  S.putBacks = PUT_BACKS.n;   // (the apply counts itself as a put-back, for the box world's copy: any other count by the next step is a snapshot somebody put back)
  S.loads = G.chronicle?.counts?.loads | 0;   // (…and a checkpoint read from a file counts itself here: savestate.ts loadState)
  S.saves = G.chronicle?.counts?.saves | 0;   // (the chronicle's count of saves as the Rust game has it: one more on the puppet is the controller's row, not yet made there)
}
/** every optional field of a Box (scene/boxes.ts `boxOf`, which names them all) */
/** THE HOST'S ROWS ARE THE HOST'S, AS THEY STAND NOW (its camera, its Input: the bridge's hand-over). An apply made BETWEEN two ticks would write the Rust
 *  game's copy of them over the page's own: a frame old after an order, the saved moment's after a load (seen on tut-squad: the page's camera put back
 *  to where it stood at the save, and the man came to rest 2.5 cm aside with the die and every part of the block equal; on tut-solo: the camera's cut
 *  to the man undone by the long list). So before any apply that is not a tick's own the host's half is handed over first. */
function handHost(S: Held): void { if (S.inStep || !S.lastCmds || !S.h) return; const hb = hostHalf(S.g, reg(), S.lastCmds); hand(hb, (p, n) => X!.face_host_half(S.h, p, n)); }
/** THE FILTERED EXPORT ONTO THE PUPPET, its out events dispatched. Answers the ms of the export, the apply and the dispatch. */
function apply(S: Held, first: boolean): { ex: number; ap: number; out: number; bytes: number } {
  const g = S.g, G = g as unknown as Record<string, any>;
  handHost(S);
  let t = performance.now();
  call('face_puppet', () => X!.face_puppet(S.h, first ? 1 : 0)); grown(g);
  const blob = outBytes(), ex = performance.now() - t;
  t = performance.now();
  const R = applyGame(g, blob, reg(), { hooks: HOOKS, puppet: { lists: lists(), start: first, rows: S.rows, ctx: S.ctx, slowTable: SLOW, vals: VALS } });
  if (RUST_GAME.prof.on && !first) { const PF = RUST_GAME.prof, ph = R.phases ?? {}; PF.applyWalk += R.ms ?? 0; PF.applyRows += ph.makeRows ?? 0; PF.applyMain += ph.main ?? 0; PF.applyTables += ph.tables ?? 0; for (const k in ph) PF.phases[k] = (PF.phases[k] ?? 0) + ph[k]; }
  after(S);
  const ap = performance.now() - t; t = performance.now();
  const ev: { arm: string; e: any }[] = R.out ?? [];
  if (ev.length && !first) { RUST_GAME.outEvents += ev.length;
    for (const x of ev) { RUST_GAME.outArms[x.arm] = (RUST_GAME.outArms[x.arm] ?? 0) + 1;
      // (one event that throws must not take the frame with it: it is named once and the rest go on)
      try { (OUT_OWN[x.arm] ?? OUT[x.arm])(g, x.e); } catch (e) { RUST_GAME.outThrew[x.arm] ??= String((e as Error)?.stack ?? e).slice(0, 600); } } }
  return { ex, ap, out: performance.now() - t, bytes: blob.length };
}
// ---- a game kept
/** the game as it stands, kept in the module: its number. (`RUST_GAME.keepAs` 1 = kept as the bridge's bytes, what a file holds, for a check of that road.) */
function keep(S: Held): number {
  const t = performance.now(), k = call('face_keep', () => X!.face_keep(S.h, RUST_GAME.keepAs)); grown(S.g);
  if (!k) throw new Error('rust=1: the face kept no game');
  S.kept.add(k);
  const ms = performance.now() - t; RUST_GAME.keeps++; RUST_GAME.keepMs = ms; if (ms > RUST_GAME.keepWorst) RUST_GAME.keepWorst = ms; RUST_GAME.kept = S.kept.size;
  return k;
}
/** the controller's shelf as the page holds it (private there; read here, never written but through its own methods) */
const shelfOf = (S: Held): { all(): { info: SlotInfo; snap: unknown }[]; last: unknown } | null => (S.g as unknown as Record<string, any>).saves?.shelf ?? null;
/** every kept game no slot names any more is let go (a ring's slot taken again, a new run's shelf) */
function sweep(S: Held): void {
  const sh = shelfOf(S), live = new Set<number>([S.startKept]);
  if (sh) { for (const e of sh.all()) live.add((e.snap as KeptGame)?.rust | 0); live.add((sh.last as KeptGame | null)?.rust ?? 0); }
  for (const k of [...S.kept]) if (!live.has(k)) { X!.face_kept_drop(k); S.kept.delete(k); }
  RUST_GAME.kept = S.kept.size;
}
/** A SAVE (savestate.ts writeMem, through SNAPS.take). The controller has just written the chronicle's row on the puppet; that row is made on the Rust
 *  side first (`face_checkpoint`: on a tutorial mission it also stops the level's program for the take and has the frame start it again, in the
 *  controller's own order, savestate.ts:991), so the kept game holds it. No state is walked in TypeScript and nothing is applied whole. */
function take(S: Held): KeptGame {
  const g = S.g, G = g as unknown as Record<string, any>;
  if (S.stale || !S.h) start(S);
  settle(S);
  const C = G.chronicle?.counts;
  if (C && (C.saves | 0) !== S.saves) { const E = G.chronicle.events, e = E[E.length - 1], label = e?.kind === 'checkpoint' ? String(e.label ?? e.data?.label ?? '') : '';
    call('face_checkpoint', () => hand(enc.encode(label), (p, n) => X!.face_checkpoint(S.h, p, n))); grown(g); S.took = true; }
  sweep(S);
  const k: KeptGame = { game: g, rust: keep(S), own: ownKeep(g) };
  if (S.took && !S.inStep) { S.took = false; apply(S, false); }   // (a save from a panel, between ticks: the row comes back to the puppet now)
  return k;
}
/** THE KEPT GAME STANDS AGAIN and the puppet takes its whole state: ONE whole apply. The page's own records of the game (`OWN`) go back with it. */
function stoodAgain(S: Held, k: number, own?: unknown[]): void {
  const g = S.g, G = g as unknown as Record<string, any>;
  let t = performance.now();
  if (!S.kept.has(k) || !call('face_back', () => X!.face_back(S.h, k))) throw new Error('rust=1: that kept game is gone (it was of a game that has been stood again since)');
  grown(g);
  RUST_GAME.backMs.back = performance.now() - t; t = performance.now();
  S.uploaded = true; S.shelfSig = '';   // (the kept game's own `g.saves` is the shelf as it was handed over before that save: the shelf as it stands is handed over again)
  whole(S);
  ownPut(g, own);
  RUST_GAME.backMs.whole = performance.now() - t; RUST_GAME.backs++;
}
/** A LOAD (savestate.ts loadMem, through SNAPS.put). The controller goes on to write the puppet (this run's counts of saves and loads, the chronicle's
 *  row, the rings put away, a tutorial's program to be started again: savestate.ts:1124 to 1134): those lines are made on the Rust side too
 *  (`face_loaded`, in `settle`) before anything else is asked of the Rust game. */
function put(S: Held, k: KeptGame): void {
  if (S.stale || !S.h) start(S);
  const G = S.g as unknown as Record<string, any>, C = G.chronicle?.counts, e = shelfOf(S)?.all().find(x => x.snap === (k as unknown));
  // (the counts of the moment that is being abandoned: the run's, not the saved moment's. Read before the kept game's are on the puppet.)
  const back = { label: String(e?.info.label ?? ''), loads: C?.loads | 0, saves: C?.saves | 0 };
  stoodAgain(S, k.rust, k.own);
  S.back = back;
}
/** WHAT FOLLOWS A LOAD ON THE RUST SIDE, as the controller's own lines have by now run on the puppet: the host's switches as they stand now (the
 *  controller put them back over the saved moment's), `face_loaded`, and the filtered export started over (the long list, which fills what was last sent
 *  and brings the chronicle's row to the puppet).
 *  NOT TAKEN ACROSS: the tune's rows as they stand now (savestate.ts:1129 `Object.assign(g.tune, tune)`): a row of the debugger's changed between the save
 *  and the load is the saved moment's on the Rust side. One of the writes by path the bridge has no call for yet.
 *  WHY NOT THE BRIDGE'S OWN EXPORT OF THE PUPPET (`face_import_into`), which would carry every such line: the bridge's book still lists the rows the
 *  abandoned moment made after the save, and one of them is a beat of a tutorial's program that 'runs' with no place the Rust game ever gave it; the
 *  import refuses the state (seen on tut-squad: "the beat 'picket' is running"). */
function settle(S: Held): void {
  const b = S.back; if (!b || !S.h || S.stale) return;
  S.back = null;
  const g = S.g, t = performance.now();
  pokes(S);
  call('face_loaded', () => hand(enc.encode(b.label), (p, n) => X!.face_loaded(S.h, p, n, b.loads, b.saves))); grown(g);
  apply(S, true);
  // (the HUD's lines are the saved moment's again, by the long list; the controller's own line of the load, savestate.ts:1132, said after them)
  { const M = (g as unknown as Record<string, any>).messages as { text: string; t: number }[]; M.push({ text: `checkpoint loaded${b.label ? ` \u2014 ${b.label}` : ''}`, t: g.time }); if (M.length > 5) M.shift(); }
  RUST_GAME.backMs.hand = performance.now() - t;
}
/** THE HOST'S SWITCHES written between ticks (the options' and the pause card's god mode / AI / ammunition rows; the pause card letting go of the launcher's
 *  held button: host/loop.ts; the debugger's panel): what differs from what the last apply left goes over by the write by path */
function pokes(S: Held): void {
  const G = S.g as unknown as Record<string, any>; if (!S.poke) return;
  const now = watched(S.g), was = S.watch;
  if (was.length === now.length) for (let i = 0; i < now.length; i++) if (!Object.is(now[i][2], was[i][2])) send(S, now[i]);
  S.watch = now;
  // (the pause card letting go of the launcher's held button, host/loop.ts; `face_poke` 4 where the write by path has no way)
  if (S.launch !== null && (G.player?.launchHeld ?? null) === null) { if (!send(S, [G.player, ['launchHeld'], null])) { X!.face_poke(S.h, 4, 0); RUST_GAME.pokes++; } S.launch = null; }
}
/** THE PLAYER'S RESTART: the game kept as the first step opened stands again (`face_back`, one whole apply), the controller begins a new run, the host
 *  forgets what it drew and played of the try that ended (the hook a save put back calls: host/putback.ts, main.ts), and the picture starts over.
 *  WHY NOT THE TYPESCRIPT'S `restartEncounter` ON THE PUPPET, OR THE RUST GAME'S OWN `game_restart`: both make the player's man anew (game.ts:1496
 *  `new Player`), and a man made on the Rust side has no object the page can draw (the importer has no hook that gives a new Character its rig, its parts
 *  and its row of the men's columns: tools/blitz/IMPORT.md, "A new man"), while the TypeScript's own lines, run on a puppet after play, meet values the
 *  filtered apply made with their listed fields only. WHAT IS LOST BY IT: the TypeScript's restart keeps the clock and the die's place and deals a quiet
 *  mission afresh; this one is the same deal again, from the same die. Belongs in the Rust game (`game_restart`), once a new man has his hook. */
function toStart(S: Held, said?: string): void {
  const g = S.g, G = g as unknown as Record<string, any>;
  stoodAgain(S, S.startKept, S.startOwn);
  if (G.saves) G.saves.missionRef = null;   // (the controller's word for 'a fresh run': a new name, an empty shelf, its triggers from nothing. savestate.ts:983)
  apply(S, true); sweep(S);   // (what the filtered export last sent was forgotten with the game that stood: the long list again, the HUD's lines as the start had them)
  if (said) g.msg(said);
  try { G.saves?.onPutBack?.({ key: '', slot: 'auto', n: 0, label: 'the start', kind: 'auto', savedAt: 0, missionT: 0, stage: String(G.mission?.stage ?? ''), runId: '', bytes: 0, mem: true }); } catch (e) { RUST_GAME.outThrew['restart.host'] ??= String((e as Error)?.stack ?? e).slice(0, 600); }
  if (CLEANRESTART.on && CLEANRESTART.picture) G.engine?.restartPicture?.('level restarted');
  RUST_GAME.restarts++;
}
/** THE CONTROLLER'S SHELF AS THE TICK READS IT (Enter once down asks for this run's newest save, player.ts:493; the Rust game's `g.saves` is no part of
 *  the bridge): handed over as a step opens whenever it has changed */
function shelfIn(S: Held): void {
  const sv = (S.g as unknown as Record<string, any>).saves, sh = shelfOf(S); if (!sv || !sh) return;
  const rows = sh.all().map(e => e.info), sig = `${sv.runId}|${rows.map(r => `${r.key}:${r.savedAt}:${r.regret ?? ''}`).join(',')}`;
  if (sig === S.shelfSig) return;
  S.shelfSig = sig;
  const parts: Uint8Array[] = [], u32 = (v: number) => { const b = new Uint8Array(4); new DataView(b.buffer).setUint32(0, v, true); parts.push(b); }, f64 = (v: number) => { const b = new Uint8Array(8); new DataView(b.buffer).setFloat64(0, v, true); parts.push(b); },
    text = (s: string) => { const b = enc.encode(s); u32(b.length); parts.push(b); }, byte = (v: number) => parts.push(Uint8Array.of(v));
  text(String(sv.runId ?? '')); u32(rows.length);
  for (const r of rows) { text(r.key); byte(r.slot === 'manual' ? 1 : 0); f64(r.n); text(r.label ?? ''); f64(r.savedAt); f64(r.missionT); text(r.stage ?? ''); text(r.runId ?? ''); if (r.regret) { byte(1); text(r.regret); } else byte(0); }
  let n = 0; for (const b of parts) n += b.length; const all = new Uint8Array(n); n = 0; for (const b of parts) { all.set(b, n); n += b.length; }
  call('face_saves_in', () => hand(all, (p, m) => X!.face_saves_in(S.h, p, m))); RUST_GAME.shelfSent++;
}
/** THE WHOLE STATE ONTO THE PUPPET (`face_export` → the bridge's whole apply): after a kept game stood again, and for a tool that is about to read the
 *  puppet for something the page does not show (`window.__rust.whole()`). A third of a second, outside any tick. */
function whole(S: Held): void {
  const g = S.g, G = g as unknown as Record<string, any>, t = performance.now();
  handHost(S);
  call('face_export', () => X!.face_export(S.h)); grown(g);
  applyGame(g, outBytes(), reg(), { hooks: HOOKS });
  S.lightsWas = null;
  after(S);
  RUST_GAME.wholes++; RUST_GAME.wholeMs = performance.now() - t;
}
/** an arm the host carries out itself where the generated dispatcher's call does not fit a page (none today) */
const OUT_OWN: Record<string, (g: any, e: any) => void> = {};
/** A CHECKPOINT THE TICK ASKED FOR (tools/blitz/OUT-EVENTS.md: both arms carry only the words and call this): the save controller, whose take under the
 *  word is the Rust game's own (`SNAPS.take`: the level's program stopped on the Rust side, the chronicle's row made there, the game kept in the module) */
OUT_HOST.checkpoint = (g, what, e) => { if (what === 'keep') g.saves?.keep?.(e.label); else g.saves?.request?.(g, e.why, e.label); };

/** WHAT THE PAGE'S PANELS WRITE ON THE GAME BETWEEN TICKS (the options' and the pause card's rows, the debugger's panel: ui/bindings.ts), each as the
 *  object `poke` takes, its path and its value now. A step opens by sending what differs from what the last apply left, by the generated write by path
 *  (tools/blitz/poke.ts → `io::poke`): no call of the face a field. A write that has no way there is named once in `RUST_GAME.pokeRefused`. */
type Watched = [unknown, (string | number)[], unknown];
const GAME_FIELDS = ['godMode', 'aiEnabled', 'infiniteAmmo', 'playerInvisible', 'clearRooms', 'spectate', 'showDebug'];
function watched(g: Game): Watched[] {
  const G = g as unknown as Record<string, any>, L: Watched[] = [];
  for (const k of GAME_FIELDS) L.push([g, [k], G[k]]);
  const T = G.tune as Record<string, unknown> | undefined; if (T) for (const k of Object.keys(T)) { const v = T[k]; if (v === null || typeof v !== 'object') L.push([g, ['tune', k], v]); }
  if (G.props) L.push([g, ['props', 'frictionScale'], G.props.frictionScale], [g, ['props', 'enabled'], G.props.enabled]);
  if (G.fogOfWar) L.push([g, ['fogOfWar', 'on'], G.fogOfWar.on]);
  const P = G.player; if (P) L.push([P, ['wand'], P.wand], [P, ['bigKit'], P.bigKit]);
  return L;
}
function send(S: Held, w: Watched): boolean {
  const name = w[1].join('.');
  try { S.poke!(w[0], w[1], w[2]); RUST_GAME.pokes++; RUST_GAME.poked[name] = (RUST_GAME.poked[name] ?? 0) + 1; return true; }
  catch (e) { RUST_GAME.pokeRefused[name] ??= String((e as Error)?.message ?? e).slice(0, 300); return false; }
}
/** STAND-IN: THE PICTURE'S LIGHTS (page.rs face_lights_out says why): the engine's list made again in the Rust game's order, each light's numbers written
 *  onto the object the puppet has for that row, a new object for a row made inside a tick (a muzzle flash). `steady`, `portal` and the GPU's `slot` stay
 *  the puppet's own; the infrared list and the fog of war's team list are not carried. Belongs in the filtered export. */
function lightsIn(S: Held): void {
  const G = S.g as unknown as Record<string, any>, list: Record<string, any>[] | undefined = G.engine?.lights?.lights; if (!list) return;
  X!.face_lights_out(S.h); const b = outBytes(), d = new DataView(b.buffer, b.byteOffset, b.byteLength), n = d.getUint32(0, true), REC = 4 + 32 * 8;
  if (b.length !== 4 + n * REC) throw new Error(`rust=1: the lights' hand-over is ${b.length} bytes for ${n} lights`);
  // (a light whose record is the bytes it was last handed over as, in the same place of the list, is left as it stands: most lights of a level never move)
  const was = S.lightsWas, same = was !== null && was.length === b.length && list.length === n;
  const book: Record<string, any>[] = bookOf(S.g).listOf('LightId');
  if (!same) list.length = 0;
  for (let i = 0, at = 4; i < n; i++, at += REC) {
    if (same) { let k = at; const end = at + REC; while (k < end && was![k] === b[k]) k++; if (k === end) continue; }
    const id = d.getUint32(at, true), f = (k: number) => d.getFloat64(at + 4 + 8 * k, true), opt = (k: number): number | undefined => { const v = f(k); return Number.isNaN(v) ? undefined : v; };
    let l = S.lightOf.get(id);
    if (!l) { l = book[id] ?? { name: '', group: '', steady: undefined, slot: -1 }; S.lightOf.set(id, l); RUST_GAME.lightsMade++; }
    l.kind = f(0); l.pos = [f(1), f(2), f(3)]; l.dir = [f(4), f(5), f(6)]; l.color = [f(7), f(8), f(9)]; l.intensity = f(10); l.range = f(11); l.innerDeg = f(12); l.outerDeg = f(13); l.radius = f(14);
    l.volumetric = f(15); l.owner = f(16); l.enabled = f(17) !== 0; l.fixtureBox = f(18); l.flicker = f(19); { const r = opt(20); if (r !== undefined || l.row !== undefined) l.row = r; }
    l.ttl = f(21); l.age = f(22); l.peakIntensity = f(23); { const t = opt(24); if (t !== undefined || l.transient !== undefined) l.transient = t === undefined ? undefined : t !== 0; }
    l.broad = f(25) !== 0; { const m = opt(26); if (m !== undefined || l.markRef !== undefined) l.markRef = m; } l.card = f(27) !== 0;
    for (const [k, name] of [[28, 'glassMask'], [29, 'fog'], [30, 'fogGhost'], [31, 'fogBy']] as const) { const v = opt(k); if (v !== undefined || l[name] !== undefined) l[name] = v; }
    if (same) list[i] = l; else list.push(l);
    RUST_GAME.lightsWritten++;
  }
  if (was !== null && was.length === b.length) was.set(b); else S.lightsWas = b.slice();
}
const LIGHTS = 2, VIEW = 4, UPLOAD = 1;
/** ONE TICK, in place of `Game.stepSeats` */
function step(S: Held, dt: number, cmds: readonly PlayerCmd[]): void {
  const g = S.g, G = g as unknown as Record<string, any>;
  // (a kept game stood again since the last step: what the controller wrote on the puppet after it goes across first, and is then no sign of a stranger's write)
  settle(S);
  if (!S.stale && S.h && PUT_BACKS.n !== S.putBacks) S.stale = 'a snapshot was put back';
  if (!S.stale && S.h && ((G.chronicle?.counts?.loads | 0) !== S.loads)) S.stale = 'a checkpoint was loaded';
  if (S.stale || !S.h) start(S);
  S.inStep = true; S.lastCmds = cmds;
  try {
    let t = performance.now();
    shelfIn(S);
    { const P = PLACE as unknown as { take(): boolean; slot: number }; if (P.take() || P.slot !== S.slot) throw new Error(`rust=1: the world kept for the Rust game (${S.slot}) was taken by another copy of the box world`); }
    if (g.sim.bind() !== S.slot) throw new Error(`rust=1: the game's block moved out of slot ${S.slot} (another game took it): the Rust game stands there`);
    const PF = RUST_GAME.prof; let tp = PF.on ? performance.now() : 0;
    knobsToRust();
    if (PF.on) { const n = performance.now(); PF.knobs += n - tp; tp = n; }
    { const L = (G.doors?.list ?? []) as Record<string, any>[]; for (let i = 0; i < L.length && i < S.doorKept.length; i++) if (!Object.is(L[i].g(DC.posAngle), S.doorKept[i])) L[i].p(DC.posAngle, NaN); }
    pokes(S);
    // the light probe's answers, as the GPU has handed them back by now
    { const P = G.sensors?.probeOf?.(), Rm: Map<string, number[]> | undefined = P?.results;
      if (Rm && Rm.size) { let n = 4; const keys: Uint8Array[] = []; for (const k of Rm.keys()) { const b = enc.encode(k); keys.push(b); n += 4 + b.length + 24; }
        const b = new Uint8Array(n), d = new DataView(b.buffer); d.setUint32(0, Rm.size, true); let at = 4, i = 0;
        for (const v of Rm.values()) { const k = keys[i++]; d.setUint32(at, k.length, true); at += 4; b.set(k, at); at += k.length; d.setFloat64(at, v[0], true); d.setFloat64(at + 8, v[1], true); d.setFloat64(at + 16, v[2], true); at += 24; }
        hand(b, (p, m) => X!.face_probe_results(S.h, p, m)); } }
    if (PF.on) { const n = performance.now(); PF.probeIn += n - tp; tp = n; }
    const hb = hostHalf(g, reg(), cmds); hand(hb, (p, n) => X!.face_host_half(S.h, p, n));
    if (PF.on) { PF.hostHalf += performance.now() - tp; PF.n++; }
    const tHost = performance.now() - t; t = performance.now();
    const flags = LIGHTS | VIEW | (S.uploaded ? UPLOAD : 0); S.uploaded = false;
    if (PF.on) call('face_step', () => {   // (the same four things in the same order, each timed)
      let a = performance.now(), b = a; if (flags & UPLOAD) { X!.face_upload(S.h); b = performance.now(); PF.upload += b - a; a = b; }
      X!.face_step(S.h, dt, 0); b = performance.now(); PF.tick += b - a; a = b; X!.face_lights(S.h, dt); b = performance.now(); PF.lights += b - a; a = b; X!.face_view(S.h); PF.view += performance.now() - a; });
    else call('face_step', () => X!.face_step(S.h, dt, flags));
    grown(g);
    const tStep = performance.now() - t;
    // WHAT THE TICK ASKED OF THE SAVE CONTROLLER (player.ts:493, Enter once down or with Shift): a save in memory to go back to as this update ends
    // (the controller's `due`), or the player's restart, which the Rust game has just made by its own rule and the page cannot follow (`toStart`)
    { const asked = X!.face_saves_asked(S.h);
      if (asked & 2) { toStart(S); RUST_GAME.ticks++; return; }
      if ((asked & 1) && G.saves) G.saves.due = outText(); }
    const A = apply(S, false);
    t = performance.now();
    dressStep(g, dt, S.side);
    if (PF.on) { tp = performance.now(); PF.dress += tp - t; PF.export += A.ex; PF.out += A.out; }
    // what the tick asked of the light probe, for the frame's GPU pass (the table is the Rust game's until the pass takes it: `face_probe_took`)
    { const P = G.sensors?.probeOf?.(); if (P && Array.isArray(P.pending)) { X!.face_probe_asks(S.h); const b = outBytes(), d = new DataView(b.buffer, b.byteOffset, b.byteLength); let at = 4; const n = d.getUint32(0, true); P.pending.length = 0;
        for (let i = 0; i < n; i++) { const kl = d.getUint32(at, true); at += 4; const key = dec.decode(b.subarray(at, at + kl)); at += kl; const pos = [d.getFloat64(at, true), d.getFloat64(at + 8, true), d.getFloat64(at + 16, true)]; at += 24; const skipOwner = d.getFloat64(at, true); at += 8;
          const hasB = b[at++]; let bb: number[] | undefined; if (hasB) { bb = [d.getFloat64(at, true), d.getFloat64(at + 8, true), d.getFloat64(at + 16, true)]; at += 24; } const rank = d.getFloat64(at, true); at += 8;
          P.pending.push(bb ? { key, pos, skipOwner, b: bb, rank } : { key, pos, skipOwner, rank }); }
        RUST_GAME.probeAsks += n; } }
    if (PF.on) { const n = performance.now(); PF.probeOut += n - tp; tp = n; }
    // THE SAVE CONTROLLER is the host's and decides off the frame as it now stands (savestate.ts Checkpoints.tick, the TypeScript tick's last line): on the
    // puppet. A save it takes is the Rust game kept (`take`), a load the kept game stood again (`put`).
    // STAND-IN: F5, the save by hand (player.ts:498, a line of the TypeScript tick the Rust tick does not have): read off the tick's own record here.
    // Belongs in the Rust tick (the controller's `hand`, read back with `face_saves_asked`).
    if (SAVETIMELINE.on && cmds[g.seats.indexOf(g.hostSeat())]?.pressed.includes('F5') && !g.player.down) G.saves?.askSave();
    { const sv = G.saves, D = G.director, R = G.rails, authored = !!sv?.authored && !!R?.live, stop = D?.stop, resume = R?.resume;
      // (an authored checkpoint of a tutorial mission: the controller stops the level's program for the take, savestate.ts:991. That is the Rust game's
      //  to do, and `take` does it there in the same order; the TypeScript's two calls would run on a puppet's program, which holds no beat)
      if (authored) { D.stop = () => {}; R.resume = () => {}; }
      try { sv?.tick(g, dt); } catch (e) { RUST_GAME.outThrew['saves.tick'] ??= String((e as Error)?.stack ?? e).slice(0, 600); }
      finally { if (authored) { D.stop = stop; R.resume = resume; } }
      if (S.took) { S.took = false; apply(S, false); } }
    if (PF.on) PF.saves += performance.now() - tp;
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

// ---- a game kept, as a file
const FILE_MAGIC = 0x52535054 /* 'TPSR' */, FILE_VERSION = 1;
let BUILD = '';
/** this build's word for the bridge's bytes: they are read only by the schema that wrote them */
function buildWord(): string { if (BUILD) return BUILD; const s = JSON.stringify(SCHEMA); let h = 0x811c9dc5; for (let i = 0; i < s.length; i++) h = Math.imul(h ^ s.charCodeAt(i), 0x01000193) >>> 0; return BUILD = `${h.toString(16)}-${s.length}`; }
/** A SAVE AS A FILE: the bridge's bytes of a kept game and its block, under a version word. `key`: a slot of the controller's (default: the game as it
 *  stands now). 'TPSR', version (u32), then three counted parts: a JSON head (the level's fingerprint, this build's word for the bytes, the label, the
 *  mission's clock and stage), the state, the block. WHAT A FILE DOES NOT HOLD (the bridge does not carry it, tools/blitz/BRIDGE.md): a thrown can's
 *  bounce, a blow, a fall or a beat of the level's script in mid-flight, the light probe's table, the counters. A save in memory holds all of it. */
export function rustFile(key?: string): Uint8Array | null {
  const S = H; if (!X || !S || !S.h || S.stale) return null;
  settle(S);
  const g = S.g, G = g as unknown as Record<string, any>, e = key ? shelfOf(S)?.all().find(x => x.info.key === key) : null; if (key && !e) return null;
  const k = e ? (e.snap as KeptGame).rust : call('face_keep', () => X!.face_keep(S.h, 1));
  X.face_kept_state(k); const state = outBytes().slice(); X.face_kept_block(k); const block = outBytes().slice(); if (!e) X.face_kept_drop(k);
  const head = enc.encode(JSON.stringify({ level: levelFingerprint(g), build: buildWord(), label: e?.info.label ?? 'by hand', missionT: e?.info.missionT ?? G.mission.stats.elapsed, stage: e?.info.stage ?? G.mission.stage, savedAt: e?.info.savedAt ?? Date.now() }));
  const out = new Uint8Array(8 + 12 + head.length + state.length + block.length), d = new DataView(out.buffer); let at = 0;
  d.setUint32(0, FILE_MAGIC, true); d.setUint32(4, FILE_VERSION, true); at = 8;
  for (const b of [head, state, block]) { d.setUint32(at, b.length, true); out.set(b, at + 4); at += 4 + b.length; }
  return out;
}
/** …AND READ: checked (the word, the version, this build's schema, this level), then put on the controller's shelf as a save by hand of this run, to be
 *  loaded as any save is. Answers the slot, or why not. */
export function rustFileIn(b: Uint8Array): SlotInfo | string {
  const S = H; if (!X || !S) return 'no Rust game stands';
  if (S.stale || !S.h) start(S);
  const g = S.g, G = g as unknown as Record<string, any>, d = new DataView(b.buffer, b.byteOffset, b.byteLength);
  if (b.length < 20 || d.getUint32(0, true) !== FILE_MAGIC) return 'not a save of the Rust game';
  if (d.getUint32(4, true) !== FILE_VERSION) return `save version ${d.getUint32(4, true)} (this build reads ${FILE_VERSION})`;
  const parts: Uint8Array[] = []; let at = 8; for (let i = 0; i < 3; i++) { if (at + 4 > b.length) return 'the file is cut short'; const n = d.getUint32(at, true); at += 4; if (at + n > b.length) return 'the file is cut short'; parts.push(b.subarray(at, at + n)); at += n; }
  let head: Record<string, any>; try { head = JSON.parse(dec.decode(parts[0])); } catch { return 'the file’s head is not JSON'; }
  if (head.build !== buildWord()) return `made by another build of the game (${head.build}; this one ${buildWord()})`;
  const L = levelFingerprint(g); if (!head.level || head.level.fp !== L.fp) return `made on another level (${head.level?.boxes ?? '?'} boxes / ${head.level?.doors ?? '?'} doors; this one ${L.boxes} / ${L.doors})`;
  const k = call('face_keep_given', () => hand(parts[1], (p, n) => hand(parts[2], (q, m) => X!.face_keep_given(p, n, q, m)))); if (!k) return 'the module kept nothing';
  S.kept.add(k); RUST_GAME.kept = S.kept.size;
  return G.saves.shelve(g, { game: g, rust: k, own: ownKeep(g) } as unknown as Snapshot, String(head.label ?? 'from a file'), Number(head.missionT) || 0, String(head.stage ?? ''));
}

// ---- before play
/** BEHIND A START CARD, where a stall is nobody's (main.ts, beside the save controller's own warm-up): everything a first second of play would otherwise
 *  pay for. THE RUST GAME IS STARTED (the forward export, the import, the first apply). THE EVERY-TICK APPLY is run over every row a few times, on a
 *  puppet that already holds those values (`face_puppet_warm`): its first runs cost many times its later ones. THE TICK ITSELF is stepped `ticks` times
 *  with no key held and thrown away: the module's functions are compiled as they are first called, and the game kept before those ticks is put back
 *  after them, block and all, so the game that plays is the game that was started (a check holds the die and the block's parts to a page that was
 *  never warmed). `host`: the page's own Input, camera and view, for the record those ticks are handed. */
export function rustWarm(host: { input: unknown; cam: unknown; canvas: unknown }, ticks = 24): void {
  const S = H; if (!X || !S || S.inStep) return;
  const g = S.g, W = RUST_GAME.warm; let t = performance.now();
  try {
    if (S.stale || !S.h) start(S);
    settle(S);
    W.startMs = performance.now() - t; t = performance.now();
    for (let i = 0; i < 3; i++) { call('face_puppet_warm', () => X!.face_puppet_warm(S.h)); applyGame(g, outBytes(), reg(), { hooks: HOOKS, puppet: { lists: lists(), start: false, rows: S.rows, ctx: S.ctx, slowTable: SLOW, vals: VALS } }); after(S); }
    W.applyMs = performance.now() - t; t = performance.now();
    const k = call('face_keep', () => X!.face_keep(S.h, 0)); grown(g);
    try {
      const cmd = stillCmd(sampleCmd(g.hostSeat().view as never, host.input as never, host.cam as never, host.canvas as never, null)), cmds = g.seats.map(() => cmd);
      for (let i = 0; i < ticks; i++) { const a = performance.now();
        const hb = hostHalf(g, reg(), cmds); hand(hb, (p, n) => X!.face_host_half(S.h, p, n));
        call('face_step (warming)', () => X!.face_step(S.h, 1 / 60, LIGHTS | VIEW | UPLOAD)); X!.face_saves_asked(S.h); X!.face_puppet(S.h, 0); X!.face_probe_asks(S.h); X!.face_lights_out(S.h); grown(g);
        const ms = performance.now() - a; if (i === 0) W.firstTickMs = ms; W.lastTickMs = ms; W.ticks++; }
    } finally {
      // THE GAME AS IT WAS STARTED stands again, and what the filtered export last sent is filled again from it (the puppet never took the ticks above)
      call('face_back', () => X!.face_back(S.h, k)); X!.face_kept_drop(k); call('face_puppet', () => X!.face_puppet(S.h, 1)); grown(g); S.uploaded = true;
    }
    W.tickMs = performance.now() - t; W.n++;
  } catch (e) { RUST_GAME.outThrew['warm'] ??= String((e as Error)?.stack ?? e).slice(0, 600); S.stale = 'the warm-up threw'; }
}

/** for a check: how the host stands */
/** A WRITE BY PATH FROM A TOOL OR A PANEL (`window.__rust.poke(obj, path, value)`): the puppet and the Rust game at once; a reason where it cannot be */
export function rustPoke(obj: unknown, path: string | (string | number)[], value: unknown): string | null {
  const S = H; if (!X || !S || !S.h || S.stale || !S.poke) return 'no Rust game stands';
  if (S.inStep) return 'inside a tick';
  try { settle(S); S.poke(obj, path, value); RUST_GAME.pokes++; S.watch = watched(S.g); return null; } catch (e) { return String((e as Error)?.message ?? e); }
}
export function rustState(): Record<string, unknown> | null {
  const S = H; if (!S) return null; const sh = shelfOf(S);
  return { h: S.h, slot: S.slot, stale: S.stale, back: S.back, took: S.took, startKept: S.startKept, kept: [...S.kept], inModule: X ? X.face_kept_n() : 0, shelf: sh ? sh.all().map(e => `${e.info.key.replace('twopointsix.save.', '')}=${(e.snap as KeptGame)?.rust}`) : [], last: (sh?.last as KeptGame | null)?.rust ?? 0, sig: S.shelfSig };
}
