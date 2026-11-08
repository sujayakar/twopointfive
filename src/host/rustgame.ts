// THE PAGE PLAYS THE RUST GAME (the page's game with no word; `?rust=0` = the TypeScript game while that is still here; src/main.ts says which pages
// stay with it). The simulation's tick is the Rust game's (rust/game, in the module rust/game_face builds: tools/blitz/FACE.md);
// the TypeScript `Game` stays as a PUPPET that everything outside the tick goes on reading: the picture, the sound, the HUD, the menus. Under `?rust=0`
// nothing here is loaded and the page is the page it was.
//
// A STEP (`Game.update` runs its own host lines as ever: the record sampled from the Input and the camera, hostAfter. The step between them,
// `stepSeats`, is this file's `step`, hung on the game object):
//   the host's half   the knobs the module reads · the light probe's answers (the GPU's, a frame or more old) · each seat's record with every camera and
//                     Input row (the bridge's `hostHalf` → `face_host_half`)
//   the tick          `face_step`: the tick, the lights' update,
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
import { FaceError, fetchFace } from './face';
import { simAdopt, type SimExports } from '../sim/sim';
import { BoxMirror } from '../scene/boxMirror';
import { PUT_BACKS } from '../math/side';
import { knobRegistry } from '../game/knobs';
import { knobsToRust } from '../game/rustknobs';
import { SETTLED, settledAt } from './settled';
import { DC } from '../sim/doorshome';
import { SNAPS, levelFingerprint, type SlotInfo } from '../game/savestate';
import type { Snapshot } from '../game/rollback';
import type { Game } from '../game/game';
import { NO_VIEW, type TickView } from '../view/seatview';
import { readCursorView, readDebriefView, readTickView, readUnder, type CursorView } from '../view/read';
import { CURSOR_DOUBLES, sampleCursor } from '../view/cursor';
import { heardOf, type WorldAsk } from '../view/worldask';
import { pageSay } from '../view/pagesaid';
import { CharacterAnimator } from '../game/character';
import { SkeletonInstance } from '../anim/skeleton';
import { sampleCmd, stillCmd, type PlayerCmd } from '../game/cmd';
import { dressStep, DRESS_MS } from './rustdress';
import { warmRainFx } from '../game/rainFx';
import { markHeld } from './held';
import { wallFeelPicture } from '../game/blastFeel';
import { exportGame, hostHalf, shownSeat, bookOf, report, parse, compiledExpr, helpersOf, SCHEMA } from 'tps-bridge/export';
import { applyGame } from 'tps-bridge/import';
import { classTable, valueTable } from 'tps-bridge/classes';
import { puppetRows, SLOW, VALS } from 'tps-bridge/puppet-apply';
import { OUT, OUT_HOST } from 'tps-bridge/out-dispatch';
import { NO_DEBRIEF } from '../view/debriefview';
import { END_CARD } from './endcard';
import { makePoke } from 'tps-bridge/poke';
import { makeScan } from 'tps-bridge/scan';
import { installAsks, askNothing } from 'tps-bridge/ask';
import { crossing, surfaceHost } from 'tps-bridge/surface';
import { Drawn, drawRefused, type DrawFace } from '../draw/drawn';
import { Want } from '../draw/sections';
import { fogFrom } from '../draw/fog';
import LISTS from 'tps-bridge/puppet-lists.json';


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
function call<T>(what: string, f: () => T): T { try { return f(); } catch (e) { if (e instanceof RustStopped) throw e; const w = panicText(); throw new Error(w ? `the Rust game panicked in ${what}: ${w}` : `${what}: ${String((e as Error)?.message ?? e)}`); } }

// ---- a panic
// A PANIC IN THE RUST GAME DOES NOT END THE SESSION. The module aborts on a panic (an index past a table, an `unwrap` on nothing, a `todo!()` body) and
// the host sees a trap. What is lost by it is the game of that call alone: the face takes a game out of its table for the length of a call, so no
// borrow is left behind, and every game KEPT in the module (the start, the checkpoints) is whole. So:
//   · every export of the face is called through `guard`: a trap is noted ONCE (the panic's own words, the call, the tick, the mission's clock, the
//     address, the last second of input: one line in the console that a person can paste), the game is STOPPED, and the page shows a card;
//   · while it is stopped nothing calls into the module (a call answers `RustStopped`; a step is no step; the menu's questions answer "nothing"), and the
//     puppet stands as the last apply left it, so the picture is still drawn;
//   · Enter on the card (the tick's own record of the keys) puts the run's newest checkpoint back, through the save controller's own load; with none,
//     or with Shift held, or where that same checkpoint has already led to a panic once, the mission's start; where the start has too, or there is
//     nothing kept, the page is loaded again. The pause card's Restart and its checkpoint row do the same by their own roads;
//   · the game that stands again is a NEW game of the same module made from the kept start (`face_stand`), and whoever asked puts the kept game back
//     over it (`face_back`: the clone, nothing lost of it) and applies it whole.
// WHAT IS LOST: the play since that checkpoint. NO LOOP: nothing is put back without a key, and a kept game that leads straight back to a panic (inside
// two seconds of play) is not tried a second time.
// A THROW OF THE APPLY (the bridge's own, in JavaScript: a value the puppet cannot take) is not a panic: the Rust game is whole. The puppet takes the
// whole state once; if that throws too, the game is stopped like a panic.
export class RustStopped extends Error { constructor(what: string) { super(`the Rust game is stopped after a panic (${what} was not called)`); this.name = 'RustStopped'; } }
interface Dead { text: string; what: string; tick: number; clock: number; stage: string; line: string; key: string }
export const RUST_PANIC = {
  /** the panic the game is stopped by, or null */
  dead: null as Dead | null,
  /** how many panics, how many put right, every one's line; applies that threw and were made whole */
  n: 0, recovered: 0, lines: [] as string[], applyThrew: 0, applyLast: '', applyRow: 0, applyAt: -9, frozen: 0,
  /** what the last recovery put back ('' = the start) and how often each kept game has led to a panic again with nothing newer kept since */
  last: null as { key: string; tick: number } | null, again: {} as Record<string, number>,
  /** the planted panics still to come (`?rustpanic=<tick>[,<tick>…]`, `window.__rust.panic.plant(tick, how)`): the step that would be that tick panics instead */
  planted: [] as { tick: number; how: number }[],
  plant(tick: number, how = 0): void { RUST_PANIC.planted.push({ tick, how }); },
  /** what Enter on the card would do now: 'checkpoint' | 'start' | 'reload', and the checkpoint's row */
  plan: null as { way: 'checkpoint' | 'start' | 'reload'; label: string; lost: number } | null,
};
/** the last second of input (the tick's own records, newest last) */
const INPUT_RING: PlayerCmd[] = [];
/** the exports a stopped game may still be asked: the panic's words, the last answer's bytes, the count of kept games */
const STOPPED_OK = new Set(['face_panic_ptr', 'face_panic_len', 'face_out_ptr', 'face_out_len', 'face_kept_n', 'face_mem']);
function guard(name: string, f: (...a: number[]) => number): (...a: number[]) => number {
  return (...a: number[]) => {
    if (RUST_PANIC.dead && !STOPPED_OK.has(name)) throw new RustStopped(name);
    try { return f(...a); } catch (e) { if (e instanceof WebAssembly.RuntimeError || e instanceof RangeError) died(name, e); throw e; }   // (a trap; or the stack run out inside the module)
  };
}
function inputLine(): string {
  let was = '', out = ''; let run = 0;
  for (const c of INPUT_RING) { const k = `${c.keys.join('+')}${c.pressed.length ? ` !${c.pressed.join('+')}` : ''}${c.buttons ? ` b${c.buttons}` : ''}${c.clicked ? ` c${c.clicked}` : ''}${c.wheel ? ` w${c.wheel}` : ''} @${Math.round(c.mx)},${Math.round(c.my)}`; if (k === was) { run++; continue; } if (was) out += `[${was}]x${run} `; was = k; run = 1; }
  return was ? `${out}[${was}]x${run}` : 'none';
}
/** A TRAP: noted once, the game stopped, the card shown */
function died(what: string, e: unknown): void {
  if (RUST_PANIC.dead) return;
  const S = H, G = (S?.g ?? {}) as unknown as Record<string, any>;
  const text = panicText() || String((e as Error)?.message ?? e), clock = Number(G.mission?.stats?.elapsed ?? G.time ?? 0);
  const where = typeof location !== 'undefined' ? location.search : '';
  const line = `rust=1 PANIC | ${text} | in ${what} | tick ${RUST_GAME.ticks} | mission clock ${clock.toFixed(2)} s, stage ${String(G.mission?.stage ?? '?')} | ${where || '(no address words)'} | the last second of input, oldest first: ${inputLine()}`;
  const L = RUST_PANIC.last;
  // (the game put back last led STRAIGHT back here, inside two seconds of play and with nothing newer kept: it is not tried a second time. A panic that
  //  comes later is another one, and the same checkpoint is offered again: the card always needs a key, so nothing loops)
  if (L && RUST_GAME.ticks - L.tick < 120) { let newest = ''; try { const ck = G.saves?.newest?.(true); newest = ck && G.saves.isThisRun(ck) ? ck.key : ''; } catch { /* the controller is the puppet's */ } if (newest === L.key) RUST_PANIC.again[L.key] = (RUST_PANIC.again[L.key] ?? 0) + 1; }
  RUST_PANIC.dead = { text, what, tick: RUST_GAME.ticks, clock, stage: String(G.mission?.stage ?? ''), line, key: '' };
  RUST_PANIC.n++; RUST_PANIC.lines.push(line); if (RUST_PANIC.lines.length > 16) RUST_PANIC.lines.shift();
  console.error(line);
  if (S) { S.inStep = false; panicCard(S); }
}
/** what Enter would put back now */
function panicPlan(S: Held): { way: 'checkpoint' | 'start' | 'reload'; key: string; label: string; lost: number } {
  const G = S.g as unknown as Record<string, any>, D = RUST_PANIC.dead, A = RUST_PANIC.again;
  if (!S.startKept || !S.kept.has(S.startKept) || typeof X!.face_stand !== 'function') return { way: 'reload', key: '', label: '', lost: D?.clock ?? 0 };
  let ck: SlotInfo | null = null; try { const n = G.saves?.newest?.(true) as SlotInfo | null; if (n && G.saves.isThisRun(n) && shelfOf(S)?.all().some(e => e.info.key === n.key && S.kept.has((e.snap as KeptGame)?.rust | 0))) ck = n; } catch { ck = null; }
  if (ck && !(A[ck.key] > 0)) return { way: 'checkpoint', key: ck.key, label: String(ck.label ?? ''), lost: Math.max(0, (D?.clock ?? 0) - Number(ck.missionT)) };
  if (!(A[''] > 0)) return { way: 'start', key: '', label: '', lost: D?.clock ?? 0 };
  return { way: 'reload', key: '', label: '', lost: D?.clock ?? 0 };
}
/** THE CARD, in the menu's own colours (index.html's `--m-` rows): the panic's words, what is lost, what Enter does. It stands over the HUD and UNDER the
 *  pause card (`#menu`, 20), so Esc still brings that card up over it and its Restart is the way back to the start */
function panicCard(S: Held): void {
  const D = RUST_PANIC.dead; if (!D) return;
  const P = panicPlan(S); RUST_PANIC.plan = { way: P.way, label: P.label, lost: P.lost };
  if (typeof document === 'undefined') return;
  let el = document.getElementById('rustpanic');
  if (!el) { el = document.createElement('div'); el.id = 'rustpanic';
    el.style.cssText = 'position:absolute;inset:0;z-index:19;display:flex;align-items:center;justify-content:center;background:radial-gradient(ellipse at 50% 55%, rgba(var(--m-base-rgb, 12,11,9),0.4), rgba(var(--m-base-rgb, 12,11,9),0.88) 70%);color:var(--m-text, #d8d2c4);font:13px/1.45 var(--m-font, system-ui, sans-serif);';
    (document.getElementById('app') ?? document.body).appendChild(el); }
  el.textContent = ''; el.style.display = 'flex';
  const card = document.createElement('div');
  card.style.cssText = 'box-sizing:border-box;width:min(620px,calc(100% - 40px));max-height:calc(100% - 48px);overflow:auto;padding:24px 28px 16px;background:rgba(var(--m-panel-rgb, 21,19,16),0.95);border:1px solid rgba(var(--m-edge, 255,255,255),0.24);border-top:3px solid var(--m-accent, #c8a04a);box-shadow:0 18px 60px rgba(0,0,0,0.6);border-radius:2px;';
  const row = (css: string, text: string) => { const d = document.createElement('div'); d.style.cssText = css; d.textContent = text; card.appendChild(d); return d; };
  row('font:700 30px/1 var(--m-head, inherit);letter-spacing:0.02em;text-transform:uppercase;color:var(--m-bright, #fff);', 'The game stopped');
  row('margin-top:8px;color:var(--m-soft, #b8b2a4);', `The simulation hit a fault it could not go on from${RUST_PANIC.n > 1 ? ` (the ${RUST_PANIC.n === 2 ? 'second' : RUST_PANIC.n === 3 ? 'third' : `${RUST_PANIC.n}th`} this session)` : ''}. Nothing kept is lost.`);
  row('margin-top:14px;padding:8px 10px;font:11px/1.4 var(--m-mono, ui-monospace, monospace);color:var(--m-text, #d8d2c4);background:rgba(0,0,0,0.35);white-space:pre-wrap;word-break:break-word;user-select:text;', `${D.text}\n(in ${D.what}, tick ${D.tick}, ${D.clock.toFixed(1)} s into the mission; the whole line is in the console)`);
  const secs = `${P.lost < 10 ? P.lost.toFixed(1) : Math.round(P.lost)} s of play`;
  row('margin-top:14px;color:var(--m-bright, #fff);', P.way === 'checkpoint' ? `Enter: back to the last checkpoint${P.label ? ` (${P.label})` : ''}. ${secs} since it are lost.` : P.way === 'start' ? `Enter: the mission from its start${RUST_PANIC.again[RUST_PANIC.last?.key ?? '\u0000'] > 0 ? ' (the last checkpoint led straight back here)' : ' (no checkpoint of this run is kept)'}. ${secs} are lost.` : 'Enter: load the page again (nothing kept can be put back).');
  row('margin-top:12px;font:11px var(--m-mono, ui-monospace, monospace);color:var(--m-faint, #8a8474);letter-spacing:0.06em;', P.way === 'checkpoint' ? 'ENTER  the checkpoint  \u00b7  SHIFT+ENTER  the mission from its start' : 'ENTER');
  el.appendChild(card);
}
/** THE GAME OF THE TRAPPED CALL IS GONE: a new game of this module stands in its slot, made from the kept start, under a new number. Whoever called goes
 *  on to put a kept game back over it (`stoodAgain`). The planted panics that are due by now are let go: the same tick is never planted twice. */
function revive(S: Held): void {
  const D = RUST_PANIC.dead; if (!D) return;
  RUST_PANIC.dead = null;
  try {
    X!.face_panic_clear();
    // (the game of the trapped call is already out of the face's table; one stopped for a fault outside the module still stands there, and two games
    //  would share the slot: it is let go either way, with what its host was last sent)
    X!.face_drop(S.h); X!.face_puppet_drop(S.h);
    const h = X!.face_stand(S.startKept, S.slot);
    if (!h) throw new Error('rust=1: the kept start is gone: no game could be stood after the panic');
    S.h = h; S.told = -1; RUST_GAME.handle = h; S.back = null; S.took = false; S.shelfSig = '';
    grown(S.g);
  } catch (e) { RUST_PANIC.dead ??= D; throw e; }
}
/** ENTER ON THE CARD (Shift: the mission's start) */
function recover(S: Held, shift: boolean): void {
  const D = RUST_PANIC.dead; if (!D) return;
  const G = S.g as unknown as Record<string, any>, P = panicPlan(S), way = shift && P.way === 'checkpoint' ? 'start' : P.way, key = way === 'checkpoint' ? P.key : '';
  if (way === 'reload') { if (typeof location !== 'undefined') location.reload(); return; }
  const t = performance.now();
  try {
    if (way === 'checkpoint') { const why = G.saves.load(S.g, key) as string | null; if (why) throw new Error(`the checkpoint would not load: ${why}`); }
    else toStart(S, 'the mission from its start');
    if (RUST_PANIC.dead) throw new Error('the game was not stood again');
    RUST_PANIC.last = { key, tick: RUST_GAME.ticks }; RUST_PANIC.recovered++; RUST_PANIC.plan = null;
    const el = typeof document !== 'undefined' ? document.getElementById('rustpanic') : null; if (el) el.style.display = 'none';
    console.info(`rust=1: after the panic at tick ${D.tick}: ${way === 'checkpoint' ? `the checkpoint '${P.label}' stands again` : 'the mission stands again from its start'} (${(performance.now() - t).toFixed(0)} ms); ${P.lost.toFixed(1)} s of play were lost`);
  } catch (e) {
    // (the way back failed: that kept game is not tried again, and the card says what Enter does next)
    RUST_PANIC.again[key] = (RUST_PANIC.again[key] ?? 0) + 1;
    RUST_PANIC.dead ??= { ...D, text: `${D.text}\nand the way back failed: ${String((e as Error)?.message ?? e).slice(0, 300)}` };
    console.error(`rust=1: the way back after the panic failed (${way}): ${String((e as Error)?.stack ?? e).slice(0, 900)}`);
    S.inStep = false; panicCard(S);
  }
}

/** what the page says of the Rust game, on `window.__rust` */
export const RUST_GAME = {
  on: false, bytes: 0, fetchMs: 0, compileMs: 0, startMs: 0, startBytes: 0, handle: 0, slot: -1, ticks: 0, starts: 0, grew: 0, memory: 0,
  /** rows the game MADE at run time, counted by class over the steps' applies; what the made men and guns took from a twin of their class's own
   *  constructor (fields and classes of parts the bridge does not carry); a class whose twin could not be built, and why */
  made: {} as Record<string, number>, madeTook: {} as Record<string, number>, madeThrew: {} as Record<string, string>,
  /** the walk of a page a tool drives (`?paused`): flushes, rebases, their ms summed, and the fields it sent to the Rust game by the schema's name */
  scan: { flushes: 0, rebases: 0, ms: 0, wrote: {} as Record<string, number> },
  madeLast: {} as Record<string, object>, madeAtRunTime: (row: object): boolean => MADE_ROWS.has(row),
  /** ms of the last step and summed over every step: the host's half, the tick, the filtered export, the apply, the out events, the dress step */
  last: { host: 0, step: 0, export: 0, apply: 0, out: 0, dress: 0, bytes: 0 }, sum: { host: 0, step: 0, export: 0, apply: 0, out: 0, dress: 0, bytes: 0 }, worst: { step: 0, apply: 0, at: 0 },
  probeForgot: 0, wallFeels: 0, pokes: 0, poked: {} as Record<string, number>, pokeRefused: {} as Record<string, string>, askRefused: { n: 0, rows: {} } as { n: number; rows: Record<string, string> }, crossed: {} as Record<string, number>, named: 0, wholes: 0, wholeMs: 0, rowsKept: 0, nulls: 0,
  /** games kept (a save, the start) and their cost: how many were taken, ms of the last take and the worst, how many the module holds now; games put back,
   *  ms of the last (the kept game stood again, the whole apply, the hand-over of the controller's lines) */
  /** the last start, ms by part: the block's settling, the whole export, the Rust game made from it, the whole apply, the start kept, the menu's questions with the settings' first walk */
  startParts: { settle: 0, export: 0, make: 0, apply: 0, keep: 0, asks: 0 },
  /** the warm-up before the first frame (`rustWarm`): ms of the Rust game's start, of the applies, of the ticks stepped and thrown away, of the picture's own step, and how many */
  warm: { n: 0, startMs: 0, applyMs: 0, tickMs: 0, dressMs: 0, ticks: 0, firstTickMs: 0, lastTickMs: 0 },
  keeps: 0, keepMs: 0, keepWorst: 0, kept: 0, backs: 0, backMs: { back: 0, whole: 0, hand: 0 }, restarts: 0, shelfSent: 0, keepAs: 0 as 0 | 1,
  /** WHERE A STEP'S TIME GOES, finer (`prof.on`, for a check: the tick is then four calls into the module where play makes one). Sums in ms over `n` steps:
   *  the host's half (the knobs · the light probe's answers handed in · the bridge's hand-over built and taken), the tick, the lights'
   *  update, the rails' view, the filtered export, the apply (the bridge's own walk, by its phases · the menu's questions · the doors' stand-in), the out events, the dress step, the probe's asks read back, the save controller */
  prof: { on: false, n: 0, knobs: 0, probeIn: 0, hostHalf: 0, tick: 0, lights: 0, view: 0, asked: 0, export: 0, applyWalk: 0, applyRows: 0, applyMain: 0, applyTables: 0, asks: 0, doors: 0, out: 0, dress: 0, probeOut: 0, saves: 0, phases: {} as Record<string, number> },
  /** quiet runs of steps (`rustQuiet`): how many ended in an apply, and the steps taken inside them */
  quiet: { runs: 0, ticks: 0 },
  outEvents: 0, outArms: {} as Record<string, number>, outThrew: {} as Record<string, string>, gives: [] as string[], probeAsks: 0,
  dress: DRESS_MS,
  /** what the forward export could not carry at the last start (the bridge's own report) */
  notCarried: [] as string[],
  /** a panic in the module: whether the game is stopped by one, how many there were and were put right, the planted ones (above: RUST_PANIC) */
  panic: RUST_PANIC,
};

/** THE SETTLED ROWS, HELD (tools/qa/switches.json, kind "settled": 866 switch rows of the knob tables that are constants of the game now; the Rust game has
 *  no such row, only the branch the page takes). The TypeScript page still has them as switches, and three things can move one on a player's machine: a
 *  row of the debugger's panel KEPT in the browser's store and read back at every visit (pressure.ts, digin.ts and the rest of game.ts's
 *  loaders), a word on the address (`?noautopilot`), a click in the panel. On a puppet such a row would turn only the half of the game the page still
 *  runs (the HUD's rows, the menu, the picture's dressing) and not the tick. So under the word each settled row is held at its value on the page, as
 *  tools/qa/fold.ts holds it for a headless run: a read answers that value, a write of another is refused, and what was refused or stood off its value
 *  is named ONCE in the console and stays on the debugger panel's first line. Rows that are still switches (an open decision, a player's own, the
 *  difficulty's) are not touched: they cross to the Rust game with the knob tables as a game starts.
 *  Laid by main.ts after the page's switches are written (host/settled.ts raised these rows first) and before the game is stood (the kept rows are read as the Game is made).
 *  What it cannot hold (as the fold): a table replaced whole, a row that is already an accessor, a value a module copied before this ran. */
export const RUST_SETTLED = { held: 0, absent: 0, skipped: 0, off: [] as string[], refused: new Map<string, { value: unknown; n: number }>(), said: 0 };
const sameRow = (a: unknown, b: unknown): boolean => Object.is(a, b) || (typeof a === 'object' && typeof b === 'object' && JSON.stringify(a) === JSON.stringify(b));
const rowWord = (v: unknown): string => typeof v === 'number' || typeof v === 'boolean' || typeof v === 'string' ? String(v) : JSON.stringify(v);
export function rustSettledHold(): void {
  const R = RUST_SETTLED, tables = knobRegistry() as Record<string, any>;
  for (const r of SETTLED) { const at = settledAt(r.row, tables); if (!at) { R.absent++; continue; }
    const o = at.table, key = at.key;
    const d = Object.getOwnPropertyDescriptor(o, key); if (!d || d.get || d.set || !d.configurable) { R.skipped++; continue; }
    const page = r.page as unknown; if (!sameRow(d.value, page)) R.off.push(`${r.row} (${rowWord(d.value)}; held at ${rowWord(page)})`);
    Object.defineProperty(o, key, { enumerable: true, configurable: true, get: () => page, set: (v: unknown) => { if (sameRow(v, page)) return; const e = R.refused.get(r.row) ?? { value: v, n: 0 }; e.value = v; e.n++; R.refused.set(r.row, e); } });
    markHeld(o, key); R.held++; }
}
/** what the hold has turned away so far, in a line (the debugger panel's first; '' with nothing), said in the console once each time it grows */
export function rustSettledLine(): string {
  const R = RUST_SETTLED, n = R.off.length + R.refused.size; if (!n) return '';
  const names = [...R.off, ...[...R.refused].map(([row, e]) => `${row} (${rowWord(e.value)})`)];
  const line = `${n} kept row${n === 1 ? '' : 's'} or address word${n === 1 ? '' : 's'} ${n === 1 ? 'is' : 'are'} settled in this build and ${n === 1 ? 'is' : 'are'} not applied: ${names.slice(0, 12).join(', ')}${names.length > 12 ? `, and ${names.length - 12} more (window.__rust.settled)` : ''}`;
  if (R.said !== n) { R.said = n; console.info(`rust=1: ${line}`); }
  return line;
}
/** THE MODULE, fetched and compiled off the page's thread (Chrome compiles nothing over 8 MB on it), started, and handed to src/sim as the simulation's
 *  one instance. Before anything has asked src/sim for its module. Throws in plain words where there is no module to fetch. */
export async function rustBoot(): Promise<void> {
  const say = (line: string) => { const el = typeof document !== 'undefined' ? document.getElementById('loading') : null; if (el) el.textContent = line; };
  // (the module asks no die of a page)
  const M = await fetchFace(say, () => { throw new Error('rust=1: the Rust game asked a die the page never hands in'); });
  RUST_GAME.bytes = M.bytes; RUST_GAME.fetchMs = M.fetchMs; RUST_GAME.compileMs = M.compileMs;
  // (every export of the face goes through `guard`: a trap stops the game and shows the card; rust/sim's own exports, called a thousand times a frame
  //  by the page's readers, are handed on as they are)
  { const raw = M.exports as unknown as Record<string, unknown>, W: Record<string, unknown> = {}; for (const k of Object.keys(raw)) { const f = raw[k]; W[k] = typeof f === 'function' && k.startsWith('face_') ? guard(k, f as (...a: number[]) => number) : f; } X = W as unknown as Face; }
  MEM = X.memory; BUF = MEM.buffer;
  for (const n of ['face_new_at', 'face_show_seat', 'face_puppet', 'face_host_half', 'face_step', 'face_ask', 'face_tick_view', 'face_debrief_view', 'face_line', 'face_ray', 'face_hear', 'face_cursor_view', 'face_under', 'face_probe_asks', 'face_poke', 'face_write', 'face_keep', 'face_back', 'face_loaded', 'face_saves_in', 'face_saves_asked']) if (typeof X[n] !== 'function') throw new FaceError(`?rust=1: game_face.wasm has no ${n}: it was built from another tree (tools/blitz/face-build.ts builds this one's)`);
  { const no = drawRefused(X as unknown as DrawFace); if (no) throw new FaceError(`?rust=1: ${no} (tools/blitz/face-build.ts builds this checkout's)`); }
  // (the module and this page's half of the bridge must be of one generation: a module of another reads garbage)
  if ((X.face_layout?.() >>> 0) !== SCHEMA.layout) throw new FaceError(`?rust=1: game_face.wasm is of another generation than this page's bridge (its layout ${(X.face_layout?.() ?? 0) >>> 0}, the page's ${SCHEMA.layout}): the checkout's module is stale, tools/blitz/face-build.ts builds this one's`);
  // STAND-IN: EVERY CLASS OF THE SIMULATION'S FOLDERS GETS ITS OWN NAME BACK (tools/blitz/page-plugin.ts keeps each beside its class). The bridge tells
  // some classes by `constructor.name`, and a minified bundle has renamed them all: a rifle crossed as a plain firearm, and a whole apply (a restart,
  // a load) then made the page's rifle a plain one, whose weapon card threw. On the dev server's bundle the names are already theirs and nothing is
  // written. Belongs in the bridge: a class told by the class table's own entry, never by its name.
  { let n = 0; for (const f of ((globalThis as any).__tpsClasses ?? []) as (() => [Function, string][])[]) for (const [c, name] of f()) if (c.name !== name) { Object.defineProperty(c, 'name', { value: name, configurable: true }); n++; } RUST_GAME.named = n; }
  simAdopt(X, RUST_GAME.compileMs);
  PLACE = new BoxMirror(); (PLACE as unknown as { take(): boolean }).take();
  RUST_GAME.on = true; RUST_GAME.memory = MEM.buffer.byteLength;
  console.info(`rust=1: the Rust game's module, ${RUST_GAME.bytes} bytes, fetched in ${RUST_GAME.fetchMs.toFixed(0)} ms and compiled in ${RUST_GAME.compileMs.toFixed(0)} ms; memory ${(RUST_GAME.memory / 1048576).toFixed(0)} MB`);
}

// ---- a game
/** what a slot of the save controller holds here in a snapshot's place: the number of a game kept in the module */
interface KeptGame { game: Game; rust: number }
let SETTLED_SAID = false;
interface Held { drawn: Drawn; /** how often the Rust game has been moved (an apply, a whole apply, a write into it), the count the view was asked at, and the view */ moved: number; viewAt: number; view: TickView; told: number; toolScan: boolean; inRun: boolean; settings?: { flush(): void; rebase(): void; stats(): { settledLeft: number } & Record<string, unknown> } | null; g: Game; lastCmds: readonly PlayerCmd[] | null; kept: Set<number>; startKept: number; back: { label: string; loads: number; saves: number } | null; took: boolean; shelfSig: string; poke: ((obj: unknown, path: string | (string | number)[], value: unknown) => void) | null; watch: Watched[]; launch: unknown; putBacks: number; loads: number; h: number; slot: number; stale: string; ctx: { p: number }; rows: unknown; inStep: boolean; side: { list: unknown[]; of: unknown; base: number }; asks: { after(): void; give(S: any, id: string, t?: any, who?: any, o?: any): string | null; refused: { n: number; rows: Record<string, string> } } | null; quiet: boolean; owed: number; quietDt: number }
let H: Held | null = null;
const REG = { v: null as Record<string, object> | null };
const reg = (): Record<string, unknown> => (REG.v ??= knobRegistry()) as Record<string, unknown>;
let HOOKS: { classes: Record<string, { prototype: object }>; values: Record<string, unknown>; made: Record<string, (row: any, g: any) => void> } | null = null;

/** A ROW THE APPLY MAKES IS ITS CLASS'S PROTOTYPE AND ITS CARRIED FIELDS: the picture's half of it is not carried, being no part of the game's state.
 *  The start has that half from the class's own constructor; a row made at run time is given it here by THE SAME FUNCTION the constructor calls
 *  (`picture()`: skeleton.ts SkeletonInstance and PoseFK, character.ts CharacterAnimator), on the made row itself. Those functions draw no die and
 *  write no state field, so nothing of the game is touched and no second man is built. A gun needs none: the one field a made `Firearm` lacks
 *  (`ev`) is read by nothing.
 *  A made man's parts have their classes and his typed leaves are typed (tools/wire/class-defaults.ts). His seven named poses are not yet the set's:
 *  no code the page runs calls those on a made man (the animator's update is the Rust tick's). */
function madeHooks(): Record<string, (row: any, g: any) => void> {
  const took = (cls: string, n: number) => { RUST_GAME.madeTook[cls] = (RUST_GAME.madeTook[cls] ?? 0) + n; };
  const guard = (cls: string, f: (row: any, g: any) => number) => (row: any, g: any) => { try { took(cls, f(row, g)); } catch (e) { const why = String((e as Error)?.message ?? e).slice(0, 300); if (!RUST_GAME.madeThrew[cls]) console.error(`rust=1: a ${cls} the game made could not be given its picture half: ${why}`); RUST_GAME.madeThrew[cls] = why; } };
  const own: Record<string, (row: any, g: any) => void> = {
    Character: guard('Character', (row, g) => {
      const ch = row.ch ?? g.ch; row.ch ??= ch;
      (row.skel as SkeletonInstance).picture(ch);
      row.anim.ch ??= ch; (row.anim as CharacterAnimator).picture();
      return 1;
    }),
  };
  // (every class the apply makes a row of is told here, whether or not it has a picture half: the last made row of each is kept for a reader, and which
  //  rows were made at run time can be asked: `__rust.madeLast`, `__rust.madeAtRunTime(row)`)
  const all: Record<string, (row: any, g: any) => void> = {};
  return new Proxy(own, { get: (_t, cls) => typeof cls !== 'string' ? undefined : (all[cls] ??= (row: any, g: any) => { RUST_GAME.madeLast[cls] = row; MADE_ROWS.add(row); own[cls]?.(row, g); }) });
}
const MADE_ROWS = new WeakSet<object>();
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
  H = { drawn: new Drawn(X as unknown as DrawFace, MEM!, Want.fog), moved: 0, viewAt: -1, view: NO_VIEW, told: -1, toolScan: false, inRun: false, g, lastCmds: null, kept: new Set(), startKept: 0, back: null, took: false, shelfSig: '', poke: null, watch: [], launch: null, putBacks: -1, loads: 0, h: 0, slot: -1, stale: 'the game was stood', ctx: { p: 0 }, rows: null, inStep: false, side: { list: [], of: null, base: 0 }, asks: null, quiet: false, owed: 0, quietDt: 1 / 60 };
  const held = H;
  G.stepSeats = (dt: number, cmds: readonly PlayerCmd[]) => step(held, dt, cmds);
  // (for a networked game, src/net/step.ts: a quiet run of steps, and the game's own words for its state, as `rustHashes` reads them by name for a check)
  G.quietRun = rustQuiet;
  G.wholeWords = () => { if (!X || !held.h || typeof X.face_state_words !== 'function') return null; X.face_state_words(held.h); return new Uint32Array(outBytes().slice().buffer); };
  { let names: string[] | null = null; G.wholeNames = () => { if (!X || typeof X.face_state_n !== 'function') return []; if (!names) { names = []; for (let i = 0; i < X.face_state_n(); i++) { X.face_state_name(i); names.push(outText()); } } return names; }; }
  G.stateWords = () => { if (!X || !held.h) return null; const n = X.face_parts(), at = X.face_part_hashes(held.slot) >>> 0; return { die: X.face_die_n(held.h), parts: new Uint32Array(MEM!.buffer, at, n) }; };
  // (`?rustpanic=<tick>[,<tick>…]`: a panic planted in the step that would be that tick, for the check of the way back. A tick with `x` after it is
  //  planted outside any game.)
  if (typeof location !== 'undefined') for (const w of (new URLSearchParams(location.search).get('rustpanic') ?? '').split(',')) { const n = parseInt(w, 10); if (Number.isFinite(n) && n >= 0 && !RUST_PANIC.planted.some(p => p.tick === n)) RUST_PANIC.plant(n, /x$/.test(w) ? 1 : 0); }
  // A START FROM OUTSIDE A TICK: the TypeScript stands or restores the game on the puppet as it always did; the Rust game is started again from it as the
  // next step opens. (Inside a step these are the Rust game's own and never run here.)
  // (`restartEncounter` from outside a tick, while a Rust game stands, is the restart below and the TypeScript's lines do not run: they make the player's
  //  man anew and give the squad its 'fall in' through the menu's rows, and under the word those rows ask the Rust game (tools/blitz/ask.ts), which has
  //  never seen that man. Measured: the ask panics in the Rust row's `issue` on a man's ID past its table, and the game is lost with it.)
  for (const name of ['restartEncounter', 'loadState']) {
    const was = G[name]; if (typeof was !== 'function') continue;
    G[name] = function (this: unknown, ...a: unknown[]) {
      if (RUST_PANIC.dead && !held.inStep) { if (name === 'restartEncounter') recover(held, true); return; }   // (stopped by a panic: the restart is the card's Shift+Enter; a state poured over the puppet would be over a game that is gone)
      if (name === 'restartEncounter' && !held.inStep && held.h && !held.stale && held.startKept) { toStart(held); return; }
      const r = was.apply(this, a); if (!held.inStep) held.stale = name; return r; };
  }
  // THE PLAYER'S RESTART (a card's button, the panel's: `Game.restart`): the game kept as the first step opened stands again
  { const was = G.restart; G.restart = function (this: unknown, said?: string) { if (RUST_PANIC.dead && !held.inStep) { recover(held, true); return; } if (held.inStep || !held.h || held.stale || !held.startKept) return was.call(this, said); toStart(held, said); }; }
  // A SAVE AND A LOAD are the Rust game's (game/savestate.ts SNAPS): the controller's slot holds the number of a game kept in the module
  SNAPS.own = true;
  SNAPS.take = (gg, _since, label) => { if (gg !== g) throw new Error('rust=1: a save of another game than the one the Rust game steps'); return take(held, label) as unknown as Snapshot; };
  SNAPS.put = s => { const k = s as unknown as KeptGame; if (k.game !== g || !k.rust) throw new Error('rust=1: that save is not a game kept by the Rust game'); put(held, k); };
  // THE LIGHTS' UPDATE (a flash's envelope and its retirement) is made on the Rust side as its step ends (`face_step`'s LIGHTS flag); the lights are rows of
  // the one block, so made here too it would be made twice
  { const L = G.engine.lights; L.update = function () { /* the Rust side made it */ }; }
  // THE LIGHT PROBE'S FRAME (render/probe.ts encode: the table sent to the GPU and emptied): the Rust game's table is emptied with it
  { const P = G.sensors?.probeOf?.(); if (P && typeof P.encode === 'function') { const was = P.encode; P.encode = function (this: any, ...a: unknown[]) { const n = this.pending.length; const r = was.apply(this, a); if (n && held.h && !RUST_PANIC.dead) { try { call('face_probe_took', () => X!.face_probe_took(held.h)); } catch (e) { if (!RUST_PANIC.dead) throw e; } }   // (this is the frame's own GPU pass: a trap here stops the game, never the frame)
      return r; }; } }
  // AN ORDER GIVEN FROM OUTSIDE A TICK (a tool's `__orders.give`, a panel's button) goes to the Rust game at once, in the words the headless host gives
  // it (rust/game_run `give`), and the puppet takes the state again. Inside a tick the menu's own keys are in the record and are the Rust game's.
  // A thing, options and one man by object have no words there: such an order runs through the menu's own `give` on the puppet, whose rows ask the
  // Rust game (tools/blitz/ask.ts: `issue` is made there, and the puppet takes the state again); what `give` itself writes beside the row's `issue` (its
  // line of the log, a man's word) is then the puppet's alone until the next apply. STAND-IN for a `give` of the face that takes a target as bytes.
  { const O = G.orders, give = O.give; O.give = function (this: any, id: string, t: any, who: any, opts: any) {
      if (held.inStep) return give.call(this, id, t, who, opts);
      if (RUST_PANIC.dead) return 'the game is stopped';
      if (held.stale || !held.h) return give.call(this, id, t, who, opts);
      // (before the Rust game is started, and while it waits to be started again, the puppet IS the game: a mission's own staging gives its orders
      //  here, and the start takes them across with everything else)
      if (held.inStep || held.stale || !held.h) return give.call(this, id, t, who, opts);
      settle(held);
      return held.asks!.give(this, id, t ?? this.targetAt(g.aimPoint), who, opts); }; }
  // THE MENU'S OWN CALLS FROM OUTSIDE A TICK (a tool's `__orders.open()`, `.pick()`, `.close()`, a panel): the same road as the key. The key's press is in
  // the record and the Rust tick opens the menu; a call from outside ran the TypeScript on the puppet, which built rows of its own that the Rust game had
  // never seen (and asked it about them). Each now crosses by the call table's row (tools/blitz/surface.ts) to the Rust function the tick itself runs, and
  // the puppet takes the state again. Inside a tick, and while the puppet IS the game (before a start, while one is due), the method runs as it was.
  { const O = G.orders; surfaceHost({ g, knobs: reg(), surfaceOpen: () => !!held.h && !held.inStep && !held.stale, surfaceDispatching: () => false,
      surfaceCall: (row: number, bytes: Uint8Array, view = -1) => { settle(held); pokes(held); if (held.toolScan) scanFlush(held); else held.settings?.flush(); handHost(held); call('face_call', () => hand(bytes, (p, n) => X!.face_call(held.h, row, view, p, n))); const out = outBytes().slice(); grown(g); apply(held, false); return out; } });
    for (const m of ['open', 'close', 'turn', 'pageBack', 'pick', 'cycleWho', 'address']) { const was = O[m]; if (typeof was !== 'function') continue;
      O[m] = function (this: any, ...a: unknown[]) { if (RUST_PANIC.dead) return undefined; if (held.inStep || held.stale || !held.h) return was.apply(this, a); RUST_GAME.crossed[m] = (RUST_GAME.crossed[m] ?? 0) + 1; try { return crossing(`SquadOrders.${m}`, this, a); } catch (e) { if (RUST_PANIC.dead) return undefined; throw e; } }; } }
  // WHAT THE HUD IS TOLD (src/view/seatview.ts): asked of the Rust game once after whatever moved it, and kept till the next (`viewAt`: an apply, a whole apply
  // and a write into the game each put it off). While no Rust game stands, or one is stopped, the last word stands. The HUD reads it every drawn frame: a
  // trap inside the question stops the game, and is no throw of the frame's (the frame goes on, or the panic card's key would never be read)
  g.viewAsk = () => { if (!held.h || held.stale || RUST_PANIC.dead) return held.view;
    try { settle(held); if (held.toolScan && !held.inRun && !held.inStep) scanFlush(held); tellShown(held);
      if (held.viewAt !== held.moved) { call('face_tick_view', () => X!.face_tick_view(held.h)); const r = readTickView(outBytes()); grown(g); held.view = r; held.viewAt = held.moved; }
    } catch (e) { if (!RUST_PANIC.dead) throw e; }
    return held.view; };
  // WHAT THE CURSOR IS ON, AND WHAT A POINT IS (src/view/cursor.ts): asked once a cursor, camera and moment of the game, kept till one of them moves
  { const N = new Float64Array(CURSOR_DOUBLES), bytes = new Uint8Array(N.buffer); let at = -1, whose = -1, view: CursorView = { under: null };
    const ask = <T>(f: () => T, none: () => T): T => { if (!held.h || held.stale || RUST_PANIC.dead) return none(); try { settle(held); if (held.toolScan && !held.inRun && !held.inStep) scanFlush(held); tellShown(held); const r = f(); grown(g); return r; } catch (e) { if (!RUST_PANIC.dead) throw e; return none(); } };
    g.cursorAsk = (input, cam, canvas, seat = -1) => ask(() => { const moved = sampleCursor(N, input, cam, canvas); if (moved || at !== held.moved || whose !== seat) { at = -1; call('face_cursor_view', () => hand(bytes, p => X!.face_cursor_view(held.h, seat, p))); view = readCursorView(outBytes()); at = held.moved; whose = seat; } return view; }, () => view);
    g.underAsk = (p, seat = -1) => ask(() => { call('face_under', () => X!.face_under(held.h, seat, p[0], p[1], p[2])); return readUnder(outBytes()); }, () => null); }
  // WHAT THE END CARD SHOWS (src/view/debriefview.ts): asked as the card is built, and kept by nobody
  g.debriefAsk = () => { if (!held.h || held.stale || RUST_PANIC.dead) return NO_DEBRIEF;
    try { settle(held); if (held.toolScan && !held.inRun && !held.inStep) scanFlush(held); tellShown(held); call('face_debrief_view', () => X!.face_debrief_view(held.h)); const r = readDebriefView(outBytes()); grown(g); return r; }
    catch (e) { if (!RUST_PANIC.dead) throw e; return NO_DEBRIEF; } };   // (a trap here is no throw of the frame's, or the panic card's key would never be read: as `viewAsk`)
  // THE BOX WORLD ASKED (src/view/worldask.ts): the Rust game's, as its boxes stand. While none stands, or one is stopped, the page's own copy answers
  { const ask = rustAsk(held); Object.defineProperty(g, 'askHost', { configurable: true, enumerable: false, get: () => held.h && !held.stale && !RUST_PANIC.dead ? ask : null }); }
  // THE GAME'S OWN METHODS A TOOL OR A PANEL CALLS BETWEEN TICKS (`GAME_CROSSES`): run as TypeScript they move the puppet only, and the Rust game keeps
  // the man where he was (seen: a check put the player inside a door by `teleportPlayer`, and the interaction key then worked nothing, the Rust game's
  // man still standing at his spawn). Each crosses by its row of the call table like the orders menu's calls above.
  for (const m of GAME_CROSSES) { const was = G[m]; if (typeof was !== 'function') continue;
    G[m] = function (this: any, ...a: unknown[]) { if (RUST_PANIC.dead) return undefined; if (held.inStep || held.stale || !held.h) return was.apply(this, a); RUST_GAME.crossed[m] = (RUST_GAME.crossed[m] ?? 0) + 1; try { return crossing(`Game.${m}`, this, a); } catch (e) { if (RUST_PANIC.dead) return undefined; throw e; } }; }
}

/** the box world's questions, asked of the Rust game (a trap inside one stops the game and answers "nothing": the camera and the sound ask every drawn frame) */
function rustAsk(S: Held): WorldAsk {
  const f64 = (n: number): Float64Array => new Float64Array(outBytes().slice().buffer, 0, n);
  const ask = <T>(f: () => T, none: T): T => { try { const r = f(); grown(S.g); return r; } catch (e) { if (!RUST_PANIC.dead) throw e; return none; } };
  return {
    blocked: (a, b) => ask(() => call('face_line', () => X!.face_line(S.h, 0, a[0], a[1], a[2], b[0], b[1], b[2])) !== 0, false),
    solid: (a, b) => ask(() => call('face_line', () => X!.face_line(S.h, 1, a[0], a[1], a[2], b[0], b[1], b[2])) !== 0, false),
    ray: (o, d, max) => ask(() => { if (!call('face_ray', () => X!.face_ray(S.h, o[0], o[1], o[2], d[0], d[1], d[2], max))) return null; const r = f64(5); return { t: r[0], n: [r[1], r[2], r[3]] as [number, number, number] }; }, null),
    hear: (from, to) => ask(() => { call('face_hear', () => X!.face_hear(S.h, from[0], from[1], from[2], to[0], to[1], to[2])); return heardOf(f64(10)); }, { direct: true, routed: false, pathLen: 0, via: [from[0], from[1], from[2]], apparent: [from[0], from[1], from[2]], muffle: 0, carry: 1 }),
    together: f => f(),
  };
}

/** THE RUST GAME STARTED FROM THE PUPPET AS IT STANDS: the whole state across once, over the block that already stands in the puppet's slot */
function start(S: Held): void {
  const g = S.g, t0 = performance.now(), P = RUST_GAME.startParts; let tp = t0; const part = (k: keyof typeof P): void => { const n = performance.now(); P[k] = n - tp; tp = n; };
  HOOKS ??= { classes: classTable(), values: valueTable(), made: madeHooks() };
  g.sim.settle(g);   // (as a tick's own opening: the men's poses at home, the block bound)
  const slot = g.sim.bind();
  if ((PLACE as unknown as { slot: number }).slot !== slot) throw new Error(`rust=1: the game's block stands in slot ${slot} of the module and the world kept for the Rust game is ${(PLACE as unknown as { slot: number }).slot}: the two must be one number (rust/game box_mirror_take)`);
  if (S.h) { X!.face_drop(S.h); S.h = 0; }
  // (the bridge's book of which object has which row is a game's for as long as the Rust game it was exported into: a new Rust game, a new book)
  { const bk = bookOf(g); if (bk) { bk.ids.clear(); bk.lists.clear(); bk.first.clear(); bk.loose.clear(); } }
  part('settle');
  const state = g.col.writes(() => exportGame(g, reg()));   // (no copy of the box world stands as the game is handed over: a start inside a frame's window starts as one outside it)
  part('export');
  RUST_GAME.notCarried = [...report()].map(([k, v]) => `${k} x${v.n}`);
  const h = call('face_new_at', () => hand(state, (p, n) => X!.face_new_at(p, n, slot)));
  if (!h) throw new Error('rust=1: the face made no game (no such slot)');
  part('make');
  S.h = h; S.told = -1; S.slot = slot; S.stale = ''; S.ctx = { p: 0 }; S.rows = puppetRows(S.ctx); S.back = null; S.took = false; S.shelfSig = '';
  grown(g);
  S.asks = null;
  apply(S, true); part('apply');
  // THE START KEPT: what a restart goes back to. (Every game kept of the Rust game that stood before is gone with it: a slot that still names one is refused.)
  for (const k of S.kept) X!.face_kept_drop(k); S.kept.clear();
  S.startKept = keep(S);
  part('keep');
  // (the menu's questions: asked of this Rust game by its rows' IDs in this book, so installed anew at every start)
  S.asks = installAsks(g, reg(), (kind, row, seat, b) => { if (RUST_PANIC.dead) return askNothing(kind, '');
    // (the HUD asks every drawn frame: a trap inside a question stops the game and answers "nothing", and is no throw of the frame's)
    let no: number; try { no = call('face_ask', () => hand(b, (p, n) => X!.face_ask(S.h, kind, row, seat, p, n))) | 0; } catch (e) { if (RUST_PANIC.dead) return askNothing(kind, ''); throw e; }
    return no || outBytes().slice(); }, () => { if (RUST_PANIC.dead) return; try { grown(g); if (!S.inStep) apply(S, false); } catch (e) { if (!RUST_PANIC.dead) throw e; } });
  RUST_GAME.askRefused = S.asks.refused;
  // (the write by path: by the rows' IDs in this book too)
  S.poke = makePoke(g, reg(), b => { S.moved++; call('face_write', () => hand(b, (p, n) => X!.face_write(S.h, p, n))); });
  // THE SETTINGS, written here by whatever writes them (a panel's row, a lever, a difficulty, an address word) and read by the Rust game: held to what the last walk saw, as a tick opens. A settled row is no setting and is not walked
  // A PAGE A TOOL DRIVES (`?paused`: its frames are stepped by a script, never by the clock) walks more than the settings: every carried field (the
  // scan's `every`: no reading of the tools' sources; about 32 ms a walk on the first tutorial, the software adapter's box, two a run of frames), so a
  // check's `game.lesson.i = 3` or `door.locked = false` between two runs of frames reaches the Rust game as it does on the headless tools' host:
  // the same scan, flushed as a script's run of frames or a crossing opens and held again to the puppet as the run closes (rustToolsWrote /
  // rustToolsHold). THE RULE: what a script writes on the puppet BETWEEN its calls to step is carried into the game. A player's page pays none of
  // it: its walk is the settings', every sixteenth tick, as before.
  { const tools = TOOL_DRIVEN; 
    const scan = makeScan(g, reg(), (b: Uint8Array) => { S.moved++; call('face_write', () => hand(b, (p, n) => X!.face_write(S.h, p, n))); }, tools ? (q: string) => { RUST_GAME.scan.wrote[q] = (RUST_GAME.scan.wrote[q] ?? 0) + 1; } : () => {}, !tools, tools ? { traps: true, every: true } : {}); scan.rebase(); S.settings = scan; S.toolScan = tools;
  if (!SETTLED_SAID) { SETTLED_SAID = true; console.info(`rust=1: ${scan.stats().settledLeft} settled rows of the knob tables are the game's constants, and what a page writes to them does not reach it`); } }
  // (room asked for and given back once the game stands: the allocator keeps it, so a step seldom has to grow the memory, which detaches every view over it)
  part('asks');   // (the menu's questions, the write by path and the settings' first walk)
  X!.face_reserve(64 * 1048576); grown(g);
  RUST_GAME.handle = h; RUST_GAME.slot = slot; RUST_GAME.starts++; RUST_GAME.startMs = performance.now() - t0; RUST_GAME.startBytes = state.length;
  console.info(`rust=1: the Rust game started from the page's (${state.length} bytes of state, slot ${slot}) in ${RUST_GAME.startMs.toFixed(0)} ms${RUST_GAME.notCarried.length ? `; the bridge could not carry ${RUST_GAME.notCarried.length} kinds of value (window.__rust.notCarried)` : ''}`);
}

/** WHAT STANDS ROUND EVERY APPLY, the filtered one and the whole one: the menu's questions for the rows an apply made, the doors' stand-in, and what the
 *  host's writes between ticks are told by */
function after(S: Held): void {
  // (what the picture is told by section, src/draw: no field of it is on the bridge)
  call('face_draw', () => S.drawn.take(S.h)); fogFrom(S.g.fogOfWar.frame, S.drawn);
  const g = S.g, G = g as unknown as Record<string, any>;
  if (S.toolScan && !S.inStep && !S.inRun) scanRebase(S);   // (an apply between a script's runs of frames, a crossing's or a load's: the puppet as it now stands is the Rust game's, and no tool's write)
  // THE MENU'S QUESTIONS (tools/blitz/ask.ts): every order row and the squad's posture have closures that ask the Rust game by the row's ID; a row or a
  // posture this apply made gets its own here
  const PF = RUST_GAME.prof; let tp = PF.on ? performance.now() : 0;
  S.asks?.after();
  if (PF.on) { const n = performance.now(); PF.asks += n - tp; tp = n; }
  // STAND-IN: A DOOR'S KEPT CENTRE is the Rust step's, in two halves: the angle it was made for and its x and z are columns of the block (sim/doorshome.ts
  // posAngle, posX, posZ), the array `Door.posAt` an object's field that is on neither list of the filtered export. Nothing here reads the array; it is
  // made from the columns after an apply, for a puppet that is handed over whole. Belongs in the filtered export (`doors::Door.pos_at`).
  for (const dr of (G.doors?.list ?? []) as Record<string, any>[]) { const x = dr.g(DC.posX), z = dr.g(DC.posZ), at = dr.posAt; if (!Number.isNaN(dr.g(DC.posAngle)) && (at[0] !== x || at[2] !== z)) dr.posAt = [x, 1.0, z]; }
  if (PF.on) { const n = performance.now(); PF.doors += n - tp; tp = n; }
  S.watch = watched(g); S.launch = G.player?.launchHeld ?? null;   // (what the page's panels write, as the Rust game has it now: what differs by the next step, the page wrote)
  S.putBacks = PUT_BACKS.n;   // (the apply counts itself as a put-back, for the box world's copy: any other count by the next step is a snapshot somebody put back)
  S.loads = G.chronicle?.counts?.loads | 0;   // (…and a checkpoint read from a file counts itself here: savestate.ts loadState)
}
/** every optional field of a Box (scene/boxes.ts `boxOf`, which names them all) */
/** THE HOST'S ROWS ARE THE HOST'S, AS THEY STAND NOW (its camera, its Input: the bridge's hand-over). An apply made BETWEEN two ticks would write the Rust
 *  game's copy of them over the page's own: a frame old after an order, the saved moment's after a load (seen on tut-squad: the page's camera put back
 *  to where it stood at the save, and the man came to rest 2.5 cm aside with the die and every part of the block equal; on tut-solo: the camera's cut
 *  to the man undone by the long list). So before any apply that is not a tick's own the host's half is handed over first. */
/** which seat this page shows is its own fact: the Rust game hears it before an apply and before the host's half of a tick */
function tellShown(S: Held): void { const g = S.g as unknown as { seats?: readonly unknown[] }; if (!S.h || !g.seats || g.seats.length < 2) return; const k = shownSeat(S.g); if (k === S.told) return; const was = S.told; S.told = k; X!.face_show_seat(S.h, k);
  if (was >= 0 && !S.inStep) apply(S, false); }   // (the puppet's accessors answer another seat now: it takes that seat's values)
function handHost(S: Held): void { if (S.inStep || !S.lastCmds || !S.h) return; tellShown(S); const hb = hostHalf(S.g, reg(), S.lastCmds); hand(hb, (p, n) => X!.face_host_half(S.h, p, n)); }
/** THE FILTERED EXPORT ONTO THE PUPPET, its out events dispatched. Answers the ms of the export, the apply and the dispatch. */
function apply(S: Held, first: boolean, then?: () => void): { ex: number; ap: number; out: number; bytes: number } {
  const g = S.g, G = g as unknown as Record<string, any>;
  S.moved++;
  handHost(S);
  let t = performance.now();
  tellShown(S); call('face_puppet', () => X!.face_puppet(S.h, first ? 1 : 0)); grown(g);
  const blob = outBytes(), ex = performance.now() - t;
  t = performance.now();
  const ap1 = () => applyGame(g, blob, reg(), { hooks: HOOKS, puppet: { lists: lists(), start: first, rows: S.rows, ctx: S.ctx, slowTable: SLOW, vals: VALS } }), R = ap1();
  if (!first) for (const k in R.made) if (R.made[k]) RUST_GAME.made[k] = (RUST_GAME.made[k] ?? 0) + R.made[k];   // (rows the game made at run time, by class: `__rust.made`)
  if (RUST_GAME.prof.on && !first) { const PF = RUST_GAME.prof, ph = R.phases ?? {}; PF.applyWalk += R.ms ?? 0; PF.applyRows += ph.makeRows ?? 0; PF.applyMain += ph.main ?? 0; PF.applyTables += ph.tables ?? 0; for (const k in ph) PF.phases[k] = (PF.phases[k] ?? 0) + ph[k]; }
  after(S);
  const ap = performance.now() - t; t = performance.now();
  const ev: { arm: string; e: any }[] = R.out ?? []; let out = 0;
  // ONE WINDOW ON THE BOX WORLD'S COPY (scene/collision.ts `window`; the frame's own, where a frame stepped: host/loop.ts) for what the host asks of it behind a step: the out events' marks, the dress step (`then`)
  g.col.window(() => { if (ev.length && !first) { RUST_GAME.outEvents += ev.length;
    for (const x of ev) { RUST_GAME.outArms[x.arm] = (RUST_GAME.outArms[x.arm] ?? 0) + 1;
      // (one event that throws must not take the frame with it: it is named once and the rest go on)
      try { OUT[x.arm](g, x.e); } catch (e) { RUST_GAME.outThrew[x.arm] ??= String((e as Error)?.stack ?? e).slice(0, 600); } } }
    out = performance.now() - t; then?.(); });
  return { ex, ap, out, bytes: blob.length };
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
/** A SAVE (savestate.ts writeMem, through SNAPS.take). The chronicle's row is made on the Rust side first, under the save's label (`face_checkpoint`: on a tutorial mission it also stops the level's program for the take and has the frame start it again, in the
 *  controller's own order, savestate.ts:991), so the kept game holds it. No state is walked in TypeScript and nothing is applied whole. */
function take(S: Held, label?: string): KeptGame {
  const g = S.g, G = g as unknown as Record<string, any>;
  if (S.stale || !S.h) start(S);
  settle(S);
  if (label !== undefined) { call('face_checkpoint', () => hand(enc.encode(label), (p, n) => X!.face_checkpoint(S.h, p, n))); grown(g); S.took = true; }
  sweep(S);
  const k: KeptGame = { game: g, rust: keep(S) };
  if (S.took && !S.inStep) { S.took = false; apply(S, false); }   // (a save from a panel, between ticks: the row comes back to the puppet now)
  return k;
}
/** THE KEPT GAME STANDS AGAIN and the puppet takes its whole state: ONE whole apply. */
function stoodAgain(S: Held, k: number): void {
  const g = S.g, G = g as unknown as Record<string, any>;
  let t = performance.now();
  revive(S);   // (stopped by a panic: the game of the trapped call is gone, and a new one stands for the kept game to be put back over)
  S.told = -1;
  if (!S.kept.has(k) || !call('face_back', () => X!.face_back(S.h, k))) throw new Error('rust=1: that kept game is gone (it was of a game that has been stood again since)');
  grown(g);
  RUST_GAME.backMs.back = performance.now() - t; t = performance.now();
  S.shelfSig = '';   // (the kept game's own `g.saves` is the shelf as it was handed over before that save: the shelf as it stands is handed over again)
  whole(S);
  RUST_GAME.backMs.whole = performance.now() - t; RUST_GAME.backs++;
}
/** A LOAD (savestate.ts loadMem, through SNAPS.put). The controller goes on to write the puppet (this run's counts of saves and loads, the chronicle's
 *  row, the rings put away, a tutorial's program to be started again: savestate.ts:1124 to 1134): those lines are made on the Rust side too
 *  (`face_loaded`, in `settle`) before anything else is asked of the Rust game. */
function put(S: Held, k: KeptGame): void {
  if (!RUST_PANIC.dead && (S.stale || !S.h)) start(S);
  const G = S.g as unknown as Record<string, any>, C = G.chronicle?.counts, e = shelfOf(S)?.all().find(x => x.snap === (k as unknown));
  // (the counts of the moment that is being abandoned: the run's, not the saved moment's. Read before the kept game's are on the puppet.)
  const back = { label: String(e?.info.label ?? ''), loads: C?.loads | 0, saves: C?.saves | 0 };
  stoodAgain(S, k.rust);
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
  RUST_GAME.backMs.hand = performance.now() - t;
}
/** THE HOST'S SWITCHES written between ticks (the options' and the pause card's god mode / AI / ammunition rows; the pause card letting go of the launcher's
 *  held button: host/loop.ts; the debugger's panel): what differs from what the last apply left goes over by the write by path */
function pokes(S: Held): void {
  const G = S.g as unknown as Record<string, any>; if (!S.poke) return;
  const now = watched(S.g), was = S.watch;
  const same = (a: unknown, b: unknown) => Object.is(a, b) || (Array.isArray(a) && Array.isArray(b) && a.length === b.length && a.every((v, k) => Object.is(v, b[k])));
  if (was.length === now.length) for (let i = 0; i < now.length; i++) if (!same(now[i][2], was[i][2])) send(S, now[i]);
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
  // WHAT IS NOT THE START'S TO TAKE BACK: the switches the player and this machine set on the game since (god mode, the AI, unlimited ammunition, the
  // tune's rows, the furniture's friction, the fog's switch): the TypeScript's restart leaves them as they are now, and so does a load (savestate.ts
  // loadMem). Seen on the page: god mode ticked, the pause card's restart, and the next frag killed him. They are read here, written back on the
  // puppet after the start stands, and sent to the Rust game by the write by path (`pokes`).
  const mine = watched(g).filter(w => w[0] === g && w[1][0] !== 'engine');
  stoodAgain(S, S.startKept);
  G.saves?.startedAgain();   // (a fresh run: a new name, an empty shelf, its triggers from nothing)
  apply(S, true); sweep(S);   // (what the filtered export last sent was forgotten with the game that stood: the long list again, the HUD's lines as the start had them)
  for (const [, path, v] of mine) { let o: any = g; for (let i = 0; i < path.length - 1 && o; i++) o = o[path[i]]; if (o && !Object.is(o[path[path.length - 1]], v)) o[path[path.length - 1]] = v; }
  pokes(S);
  if (said) pageSay(said);
  try { G.saves?.onPutBack?.({ key: '', slot: 'auto', n: 0, label: 'the start', kind: 'auto', savedAt: 0, missionT: 0, stage: String(G.mission?.stage ?? ''), runId: '', bytes: 0, mem: true }); } catch (e) { RUST_GAME.outThrew['restart.host'] ??= String((e as Error)?.stack ?? e).slice(0, 600); }
  G.engine?.restartPicture?.('level restarted');
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
/** the bridge's own words, for a page check that walks the puppet by the schema (tools/qa/readtrap-page.cjs) */
export const rustBridge = () => ({ SCHEMA, parse, compiledExpr, helpersOf, bookOf, lists: lists(), knobs: reg() });
/** THE WHOLE STATE ONTO THE PUPPET (`face_export` → the bridge's whole apply): after a kept game stood again, and for a tool that is about to read the
 *  puppet for something the page does not show (`window.__rust.whole()`). A third of a second, outside any tick. */
function whole(S: Held): void {
  const g = S.g, G = g as unknown as Record<string, any>, t = performance.now();
  handHost(S);
  S.moved++;
  call('face_export', () => X!.face_export(S.h)); grown(g);
  applyGame(g, outBytes(), reg(), { hooks: HOOKS });
  after(S);
  RUST_GAME.wholes++; RUST_GAME.wholeMs = performance.now() - t;
}
/** an arm the host carries out itself where the generated dispatcher's call does not fit a page (none today) */
/** A WALL CHARGE'S BLAST, as the Rust tick's report tells it (blast_feel `wall_report`): the picture's half (blastFeel.ts `wallFeelPicture`: the fireball, the dust ring, the pushes, the ceiling's dust; their later stages are let go by the dress step) */
OUT_HOST.endCard = () => { END_CARD.due = true; };
OUT_HOST.wallFeel = (g, at) => { RUST_GAME.wallFeels++; wallFeelPicture(g, at); };
/** A CHECKPOINT THE TICK ASKED FOR (tools/blitz/OUT-EVENTS.md: both arms carry only the words and call this): the save controller, whose take under the
 *  word is the Rust game's own (`SNAPS.take`: the level's program stopped on the Rust side, the chronicle's row made there, the game kept in the module) */
OUT_HOST.checkpoint = (g, what, e) => { if (what === 'keep') g.saves?.keep?.(e.label); else g.saves?.request?.(g, e.why, e.label); };

/** WHAT THE PAGE'S PANELS WRITE ON THE GAME BETWEEN TICKS (the options' and the pause card's rows, the debugger's panel: ui/bindings.ts), each as the
 *  object `poke` takes, its path and its value now. A step opens by sending what differs from what the last apply left, by the generated write by path
 *  (tools/blitz/poke.ts → `io::poke`): no call of the face a field. A write that has no way there is named once in `RUST_GAME.pokeRefused`. */
type Watched = [unknown, (string | number)[], unknown];
const GAME_FIELDS = ['godMode', 'aiEnabled', 'infiniteAmmo', 'playerInvisible', 'clearRooms', 'spectate', 'showDebug', 'touring', 'quietUtility', 'missionInTour'];   // (the last three say whose run this is: the Rust game's verdict asks them)
function watched(g: Game): Watched[] {
  const G = g as unknown as Record<string, any>, L: Watched[] = [];
  for (const k of GAME_FIELDS) L.push([g, [k], G[k]]);
  const T = G.tune as Record<string, unknown> | undefined; if (T) for (const k of Object.keys(T)) { const v = T[k]; if (v === null || typeof v !== 'object') L.push([g, ['tune', k], v]); }
  if (G.props) L.push([g, ['props', 'frictionScale'], G.props.frictionScale], [g, ['props', 'enabled'], G.props.enabled]);
  if (G.fogOfWar) L.push([g, ['fogOfWar', 'on'], G.fogOfWar.on]);
  // (a replay's viewer, or the kit's picture, may draw with the fog off: the tick then keeps the frame's list as it stood before the fog cut it. The picture's record is the other half of the page's fog object, named as the table names it)
  if (G.fogOfWar) L.push([g, ['fog_picture', 'viewer'], G.fogOfWar.viewer]);
  if (G.engine?.settings) L.push([g, ['engine', 'settings', 'gpuParticles'], G.engine.settings.gpuParticles]);
  // (where the picture looks, set by the engine once a render: host/loop.ts. The wind picks the jets for the particles round it, wind.ts `update`)
  { const vf = G.engine?.viewFocus; if (vf && vf.length === 3) L.push([g, ['engine', 'viewFocus'], [vf[0], vf[1], vf[2]]]); }   // (the options' particle row: particleFx.ts `gpuFx` reads it in the tick)
  const P = G.player; if (P) L.push([P, ['wand'], P.wand], [P, ['bigKit'], P.bigKit]);
  return L;
}
function send(S: Held, w: Watched): boolean {
  const name = w[1].join('.');
  try { S.poke!(w[0], w[1], w[2]); RUST_GAME.pokes++; RUST_GAME.poked[name] = (RUST_GAME.poked[name] ?? 0) + 1; return true; }
  catch (e) { RUST_GAME.pokeRefused[name] ??= String((e as Error)?.message ?? e).slice(0, 300); return false; }
}
const LIGHTS = 2, VIEW = 4;
/** ONE TICK, in place of `Game.stepSeats`: no tick while the game is stopped by a panic (Enter in the tick's own record puts a kept game back); a trap
 *  inside it stops the game and is no throw of the frame's; a throw of the apply is answered by the whole state once */
function step(S: Held, dt: number, cmds: readonly PlayerCmd[]): void {
  const mine = cmds[S.g.seats.indexOf(S.g.hostSeat())] ?? cmds[0];
  if (RUST_PANIC.dead) { RUST_PANIC.frozen++; if (mine?.pressed.includes('Enter')) recover(S, mine.keys.includes('ShiftLeft') || mine.keys.includes('ShiftRight')); return; }
  if (mine) { INPUT_RING.push(mine); if (INPUT_RING.length > 60) INPUT_RING.shift(); }
  try {
    { const P = RUST_PANIC.planted, i = P.findIndex(p => p.tick <= RUST_GAME.ticks); if (i >= 0 && S.h && !S.stale) { const how = P[i].how; P.splice(i, 1); X!.face_debug_panic(S.h, how); } }
    stepOne(S, dt, cmds);
  } catch (e) {
    S.inStep = false;
    if (RUST_PANIC.dead) return;
    if (e instanceof WebAssembly.RuntimeError || e instanceof RustStopped) { died('a step', e); return; }
    // (not a trap: the Rust game stands. The puppet takes the whole state, once; the fault is named in the console and counted)
    RUST_PANIC.applyThrew++; RUST_PANIC.applyLast = String((e as Error)?.stack ?? e).slice(0, 900);
    const row = RUST_PANIC.applyAt === RUST_GAME.ticks - 1 ? RUST_PANIC.applyRow + 1 : 1; RUST_PANIC.applyRow = row; RUST_PANIC.applyAt = RUST_GAME.ticks;
    if (row <= 3) console.error(`rust=1: a step threw outside the module at tick ${RUST_GAME.ticks} (${RUST_PANIC.applyThrew} so far); the puppet takes the whole state: ${RUST_PANIC.applyLast}`);
    if (!S.h || S.stale) throw e;
    // (three steps in a row: the whole state does not mend it, and the game is stopped as by a panic, with these words)
    if (row >= 3) { died(`a step, three times running: ${String((e as Error)?.message ?? e).slice(0, 200)}`, e); return; }
    try { whole(S); RUST_GAME.ticks++; } catch (e2) { if (!RUST_PANIC.dead) died('the whole apply after a step threw', e2); }
  }
}
/** AFTER A STEP, or a quiet run of them: the apply onto the puppet, the out events (inside `apply`), the dress step, what the tick asked of the light probe,
 *  the save controller */
function tail(S: Held, dt: number, cmds: readonly PlayerCmd[], tHost: number, tStep: number): void {
  const g = S.g, G = g as unknown as Record<string, any>, PF = RUST_GAME.prof; let t = performance.now(), tp = 0;
  const A = apply(S, false, () => { t = performance.now(); dressStep(g, dt, S.side); });
  if (PF.on) { tp = performance.now(); PF.dress += tp - t; PF.export += A.ex; PF.out += A.out; }
  // what the tick asked of the light probe, for the frame's GPU pass (the table is the Rust game's until the pass takes it: `face_probe_took`)
  { const P = G.sensors?.probeOf?.(); if (P && Array.isArray(P.pending)) { X!.face_probe_asks(S.h); const b = outBytes(), d = new DataView(b.buffer, b.byteOffset, b.byteLength); let at = 4; const n = d.getUint32(0, true); P.pending.length = 0;
      for (let i = 0; i < n; i++) { const kl = d.getUint32(at, true); at += 4; const key = dec.decode(b.subarray(at, at + kl)); at += kl; const pos = [d.getFloat64(at, true), d.getFloat64(at + 8, true), d.getFloat64(at + 16, true)]; at += 24; const skipOwner = d.getFloat64(at, true); at += 8;
        const hasB = b[at++]; let bb: number[] | undefined; if (hasB) { bb = [d.getFloat64(at, true), d.getFloat64(at + 8, true), d.getFloat64(at + 16, true)]; at += 24; } const rank = d.getFloat64(at, true); at += 8;
        P.pending.push(bb ? { key, pos, skipOwner, b: bb, rank } : { key, pos, skipOwner, rank }); }
      // (the answers the tick still holds: one it forgot inside the tick is let go here too, as `Sensors.forget` lets it go on the page that ticks,
      //  or the next step's hand-over would give the stale answer straight back)
      if (at + 4 <= b.length && P.results instanceof Map) { const kn = d.getUint32(at, true); at += 4; const held = new Set<string>();
        for (let i = 0; i < kn; i++) { const kl = d.getUint32(at, true); at += 4; held.add(dec.decode(b.subarray(at, at + kl))); at += kl; }
        for (const k of [...P.results.keys()]) if (!held.has(k)) { P.results.delete(k); RUST_GAME.probeForgot++; } }
      RUST_GAME.probeAsks += n; } }
  if (PF.on) { const n = performance.now(); PF.probeOut += n - tp; tp = n; }
  // THE SAVE CONTROLLER is the host's and decides off the frame as it now stands (savestate.ts Checkpoints.tick, the TypeScript tick's last line): on the
  // puppet. A save it takes is the Rust game kept (`take`), a load the kept game stood again (`put`).
  // STAND-IN: F5, the save by hand (player.ts:498, a line of the TypeScript tick the Rust tick does not have): read off the tick's own record here.
  // Belongs in the Rust tick (the controller's `hand`, read back with `face_saves_asked`).
  if (cmds[g.seats.indexOf(g.hostSeat())]?.pressed.includes('F5') && !g.player.down) G.saves?.askSave();
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
  if (A.ap > RUST_GAME.worst.apply) RUST_GAME.worst.apply = A.ap;
}
function stepOne(S: Held, dt: number, cmds: readonly PlayerCmd[]): void {
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
    pokes(S); if (!S.toolScan && (RUST_GAME.ticks & 15) === 0) S.settings?.flush();   // (a page a tool drives walks as the script's run of frames opens: rustToolsWrote)   // (a walk of the tables is about a millisecond, and a person's hand on a panel or a lever does not need it sooner than a quarter of a second)
    // the light probe's answers, as the GPU has handed them back by now
    { const P = G.sensors?.probeOf?.(), Rm: Map<string, number[]> | undefined = P?.results;
      if (Rm && Rm.size) { let n = 4; const keys: Uint8Array[] = []; for (const k of Rm.keys()) { const b = enc.encode(k); keys.push(b); n += 4 + b.length + 24; }
        const b = new Uint8Array(n), d = new DataView(b.buffer); d.setUint32(0, Rm.size, true); let at = 4, i = 0;
        for (const v of Rm.values()) { const k = keys[i++]; d.setUint32(at, k.length, true); at += 4; b.set(k, at); at += k.length; d.setFloat64(at, v[0], true); d.setFloat64(at + 8, v[1], true); d.setFloat64(at + 16, v[2], true); at += 24; }
        hand(b, (p, m) => X!.face_probe_results(S.h, p, m)); } }
    if (PF.on) { const n = performance.now(); PF.probeIn += n - tp; tp = n; }
    tellShown(S); const hb = hostHalf(g, reg(), cmds); hand(hb, (p, n) => X!.face_host_half(S.h, p, n));
    if (PF.on) { PF.hostHalf += performance.now() - tp; PF.n++; }
    const tHost = performance.now() - t; t = performance.now();
    const flags = LIGHTS | VIEW;
    if (PF.on) call('face_step', () => {   // (the same three things in the same order, each timed)
      let a = performance.now(), b = a;
      X!.face_step(S.h, dt, 0); b = performance.now(); PF.tick += b - a; a = b; X!.face_lights(S.h, dt); b = performance.now(); PF.lights += b - a; a = b; X!.face_view(S.h); PF.view += performance.now() - a; });
    else call('face_step', () => X!.face_step(S.h, dt, flags));
    grown(g);
    const tStep = performance.now() - t;
    // WHAT THE TICK ASKED OF THE SAVE CONTROLLER (player.ts:493, Enter once down or with Shift): a save in memory to go back to as this update ends
    // (the controller's `due`), or the player's restart, which the Rust game has just made by its own rule and the page cannot follow (`toStart`)
    { const asked = X!.face_saves_asked(S.h);
      if (asked & 2) { toStart(S); RUST_GAME.ticks++; return; }
      if ((asked & 1) && G.saves) G.saves.due = outText(); }
    // A QUIET RUN (`rustQuiet`): nothing goes to the puppet until the run ends
    if (S.quiet) { S.owed++; S.quietDt += dt; RUST_GAME.sum.host += tHost; RUST_GAME.sum.step += tStep; RUST_GAME.ticks++; RUST_GAME.quiet.ticks++; return; }
    tail(S, dt, cmds, tHost, tStep);
    RUST_GAME.ticks++;
    if (tStep > RUST_GAME.worst.step) { RUST_GAME.worst.step = tStep; RUST_GAME.worst.at = RUST_GAME.ticks; }
  } finally { S.inStep = false; }
}

// ---- A QUIET RUN OF STEPS, for a caller none of whose code reads the game between them (a networked page far behind its server, src/net/page.ts: a
// seek, nothing drawn between two ticks). Between `rustQuiet(true)` and `rustQuiet(false)` a step is the host's half handed over, the Rust tick and its
// own steps after it: no export, no apply, no out event, no dress step. As the run ends the puppet takes what changed over the whole run in ONE filtered
// apply (the export's memo holds what was last sent), the out events of every tick of the run are dispatched in their order, and the dress step (over
// the run's whole time), the probe's asks and the save controller run once. The tools' host has the same run (tools/blitz/rust-host.ts quietBegin / quietEnd).
export function rustQuiet(on: boolean): void {
  const S = H; if (!S || S.quiet === on) return;
  S.quiet = on; if (on) { S.owed = 0; S.quietDt = 0; return; }
  const owed = S.owed; S.owed = 0;
  if (!owed || !S.h || S.stale || RUST_PANIC.dead) return;
  RUST_GAME.quiet.runs++; S.inStep = true;
  try { tail(S, S.quietDt, S.lastCmds ?? [], 0, 0); }
  catch (e) { if (e instanceof WebAssembly.RuntimeError || e instanceof RustStopped) died('the apply after a quiet run', e); else try { whole(S); } catch (e2) { if (!RUST_PANIC.dead) died('the whole apply after a quiet run threw', e2); } }
  finally { S.inStep = false; }
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
  return G.saves.shelve(g, { game: g, rust: k } as unknown as Snapshot, String(head.label ?? 'from a file'), Number(head.missionT) || 0, String(head.stage ?? ''));
}

// ---- before play
/** BEFORE THE FIRST FRAME, ON EVERY ROAD STRAIGHT TO PLAY (main.ts), where a stall is a little more load and no hitch in play: everything a first second of play would otherwise
 *  pay for. THE RUST GAME IS STARTED (the forward export, the import, the first apply). THE EVERY-TICK APPLY is run over every row a few times, on a
 *  puppet that already holds those values (`face_puppet_warm`): its first runs cost many times its later ones. THE TICK ITSELF is stepped `ticks` times
 *  with no key held and thrown away: the module's functions are compiled as they are first called, and the game kept before those ticks is put back
 *  after them, block and all, so the game that plays is the game that was started (a check holds the die and the block's parts to a page that was
 *  never warmed). `host`: the page's own Input, camera and view, for the record those ticks are handed. */
export function rustWarm(host: { input: unknown; cam: unknown; canvas: unknown }, ticks = 24): void {
  const S = H; if (!X || !S || S.inStep || TOOL_DRIVEN) return;   // (a page a script steps: what the script stages before its first run of frames is the start's)
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
        tellShown(S); const hb = hostHalf(g, reg(), cmds); hand(hb, (p, n) => X!.face_host_half(S.h, p, n));
        call('face_step (warming)', () => X!.face_step(S.h, 1 / 60, LIGHTS | VIEW)); X!.face_saves_asked(S.h); X!.face_puppet(S.h, 0); X!.face_probe_asks(S.h); grown(g);
        const ms = performance.now() - a; if (i === 0) W.firstTickMs = ms; W.lastTickMs = ms; W.ticks++; }
    } finally {
      // THE GAME AS IT WAS STARTED stands again, and what the filtered export last sent is filled again from it (the puppet never took the ticks above)
      call('face_back', () => X!.face_back(S.h, k)); X!.face_kept_drop(k); call('face_puppet', () => X!.face_puppet(S.h, 1)); grown(g);
    }
    W.tickMs = performance.now() - t; t = performance.now();
    // THE PICTURE'S OWN STEP once, at no time: its first run is what the first frame would otherwise pay (the rain's drip lines, a few thousand rays, the men's first draw). The rain's lines are made whole whether or not it rains yet. A step of dt 0 bears nothing and writes no game state: the dress step only reads the game, and the next tick's runs again over the same puppet
    S.inStep = true; try { warmRainFx(g); dressStep(g, 0, S.side); } finally { S.inStep = false; }
    W.dressMs = performance.now() - t; W.n++;
  } catch (e) { RUST_GAME.outThrew['warm'] ??= String((e as Error)?.stack ?? e).slice(0, 600); S.stale = 'the warm-up threw'; }
}

/** for a check: how the host stands */
/** a tool drives this page: its address says `paused` (the frames are a script's, `window.__runFrames`) */
const TOOL_DRIVEN = typeof location !== 'undefined' && new URLSearchParams(location.search).has('paused');
/** A SCRIPT'S RUN OF FRAMES OPENS AND CLOSES (`window.__runFrames`, src/main.ts). A tool writes the game by name only BETWEEN its runs; inside one the
 *  page's own lines write the puppet (the frame's camera and HUD, the host's half, the picture's module tables), and those are no tool's. So what
 *  differs from the last hold as a run opens goes to the Rust game, nothing is walked between the frames of a run, and the puppet is held again as the
 *  run closes. Nothing happens on a page no tool drives. */
export function rustToolsWrote(): void { const S = H; if (!S || !S.toolScan || !X || !S.h || S.stale || RUST_PANIC.dead) { if (S) S.inRun = true; return; } settle(S); pokes(S); scanFlush(S); S.inRun = true; }
export function rustToolsHold(): void { const S = H; if (!S) return; S.inRun = false; if (!S.toolScan || !X || !S.h || S.stale || RUST_PANIC.dead) return; scanRebase(S); }
function scanFlush(S: Held): void { tellShown(S); const t = performance.now(); S.settings?.flush(); const K = RUST_GAME.scan; K.flushes++; K.ms += performance.now() - t; }
function scanRebase(S: Held): void { const t = performance.now(); S.settings?.rebase(); const K = RUST_GAME.scan; K.rebases++; K.ms += performance.now() - t; }
/** the Game's methods that cross to the Rust game when called between ticks on the page (rustAttach) */
const GAME_CROSSES = ['teleportPlayer'];
/** A CALL BY NAME FROM A TOOL (`window.__rust.call('Game.teleportPlayer', game, [p])`): the Rust function the tick itself runs, by the call table's row
 *  (tools/blitz/surface.ts `crossing`), and the puppet takes the state again. It throws where no Rust game stands, inside a tick, or where the table has
 *  no such row: a method with no row runs a rule the Rust game cannot be asked for from here. */
export function rustCall(key: string, self: unknown, args: unknown[] = []): unknown {
  const S = H; if (!X || !S || !S.h || S.stale) throw new Error('rust=1: no Rust game stands');
  if (S.inStep) throw new Error('rust=1: a call by name inside a tick');
  RUST_GAME.crossed[key] = (RUST_GAME.crossed[key] ?? 0) + 1; return crossing(key, self, args);
}
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
