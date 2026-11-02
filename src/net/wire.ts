// THE WIRE OF A NETWORKED GAME (two-player co-op: a game server with authority, src/net/server.ts, and its clients, src/net/client.ts). What travels is the
// INPUT TIMELINE, never the state: the game plays the same fight to the bit from its recipe and its seats' records (tools/qa/coop-check.ts, samefight-page.cjs)
// once the senses are the CPU's and the maths the game's own, so the server decides which record each seat's tick ran on and tells everybody; a state word
// (game/digest.ts rawBits, chained) twice a second is the alarm. Messages are JSON text over a WebSocket, numbers to the bit (cmd.ts numWord / wordNum).
//   A RECORD ON THE WIRE (WireCmd) is a PlayerCmd minus what the game works out again for a seat it does not trust (cmd.ts checkCmd: the eye, the forward, both
//   view matrices, from the look-at, the angles, the distance and the lens). EVERY seat of a networked game is such a seat on EVERY machine — the server and each
//   client, a client's own seat too — so one working makes the view everywhere and about a hundred bytes travel a tick, not a kilobyte.
//   THE RECIPE (replay/format.ts RunTape.recipe) is the game's setup: the die, the page's address, every knob table, the kit. The server makes it of the game it
//   stood and every client stands its own game from it, as a replay's re-run does from a tape's. A networked game IS a run tape written live by the server.
// Host-neutral: no DOM, no bun; game imports are used inside functions only (src/game is one import cycle).
import type { PlayerCmd } from '../game/cmd';
import { numWord, wordNum } from '../game/cmd';
import type { RunTape } from '../replay/format';

/** `proto`: the wire's version (a hello of another is refused). `hz`: ticks a second (the fixed step's). `wordEvery`: ticks between state words. `lead`: how many
 *  of a seat's records the server likes to hold ahead of the tick it is running (the jitter buffer: Rocket League's input buffer) and `leadMax` the most before
 *  it tells the client to slow. `marginMax`: the furthest past the server's tick a client aims its records. `paceEvery`: ticks between the notes that tell a
 *  client how its records are arriving. `burst`: how far past the tick the clock names a client's bunched beats may run on. `clockMs`: a client knows the
 *  server's tick by the least-delayed tick message of the last two spans this long (so a server that let its clock slip, or a way that got longer, is followed
 *  inside two of them). `horizon`: a record for a tick further ahead than this is not kept. `catchUp`: the most ticks a host runs in one go when it is behind,
 *  and `slipAt` how far behind the server lets its game's clock slip instead. `tapeChunk`: ticks a message when a joiner is sent the run so far. `watchers`: the
 *  most connections with no seat. `maxText`: the longest message a server reads from a client (a record is a few hundred characters). `retryMs`: how long a
 *  server waits on a backed-up connection that never says it has drained before it tries the run so far again. */
export const NET = { proto: 1, hz: 60, wordEvery: 30, lead: 2, leadMax: 6, marginMax: 60, paceEvery: 10, burst: 2, clockMs: 1000, horizon: 180, catchUp: 4, slipAt: 30, tapeChunk: 600, watchers: 4, maxText: 4096, retryMs: 250 };

export type Recipe = RunTape['recipe'];
/** the knob tables a networked game stands on, whatever its maker says: the game's own maths (the same bits on every engine), the senses on the CPU (no GPU
 *  asked), every seat's record checked (the view worked out again from its pose: fromWire sends none), and the run's verdict kept (how a server knows the
 *  mission is over). Whoever makes a recipe forces them; whoever stands one refuses a recipe without them (`recipeWhy`) */
export const NET_TABLES: Record<string, Record<string, unknown>> = { EXACT: { on: true }, SENSE: { cpu: true }, CMDCHECK: { on: true }, VERDICT: { on: true } };
/** why a networked game cannot be stood from `r` ('' = it can) */
export function recipeWhy(r: Recipe): string {
  if (!r || typeof r !== 'object' || !r.tables) return 'the recipe carries no knob tables';
  const T = { ...((r.knobs as { tables?: object } | null)?.tables ?? {}), ...r.tables } as Record<string, Record<string, unknown> | undefined>;   // (a recipe keeps its tables in two places: the journal's own and every other, game/runtape.ts recipeOf)
  if (!r.scenario) return 'the recipe has no mission up';
  for (const [name, want] of Object.entries(NET_TABLES)) for (const [k, v] of Object.entries(want)) if (T[name]?.[k] !== v) return `the recipe has ${name}.${k} at ${JSON.stringify(T[name]?.[k])}: a networked game needs ${JSON.stringify(v)}`;
  return '';
}

/** a seat's record as it travels: keys held · keys gone down · buttons held · buttons gone down · the wheel · the cursor · the view's size (CSS w h, backing w h) ·
 *  the view's pose (look-at x y z, yaw, pitch, distance, fov, aspect, aim lead). The one-tick EDGES are `p`, `c` and `w`: absent = none */
export interface WireCmd { k: string[]; p?: string[]; b: number; c?: number; w?: number; m: [number, number]; s: [number, number, number, number]; v: number[]; }

export function toWire(c: PlayerCmd): WireCmd {
  const V = c.view, out: WireCmd = { k: c.keys.slice(), b: c.buttons, m: [c.mx, c.my], s: [c.w, c.h, c.pw, c.ph], v: [V.target[0], V.target[1], V.target[2], V.yaw, V.pitch, V.distance, V.fov, V.aspect, V.aimLead] };
  if (c.pressed.length) out.p = c.pressed.slice(); if (c.clicked) out.c = c.clicked; if (c.wheel) out.w = c.wheel;
  return out;
}
/** the record a tick runs on, from the wire's: the matrices and the eye are left for the game to work out (NaN: cmd.ts checkCmd builds them for a remote seat,
 *  and for any seat whose matrices are not numbers) */
export function fromWire(w: WireCmd): PlayerCmd {
  const v = w.v;
  return { keys: w.k.slice(), pressed: w.p ? w.p.slice() : [], buttons: w.b, clicked: w.c ?? 0, wheel: w.w ?? 0, mx: w.m[0], my: w.m[1], w: w.s[0], h: w.s[1], pw: w.s[2], ph: w.s[3],
    view: { vp: new Array<number>(16).fill(NaN), ivp: new Array<number>(16).fill(NaN), pos: [0, 0, 0], forward: [0, 0, 0], target: [v[0], v[1], v[2]], yaw: v[3], pitch: v[4], distance: v[5], fov: v[6], aspect: v[7], aimLead: v[8] } };
}
const keysOk = (a: unknown): a is string[] => Array.isArray(a) && a.every(k => typeof k === 'string');
const numsOk = (a: unknown, n: number): a is number[] => Array.isArray(a) && a.length === n && a.every(x => typeof x === 'number' && Number.isFinite(x));
/** `c` is a record as the wire says one is: every field there, of its kind and finite. A server asks before it keeps what a client sent. (A SHAPE only: how
 *  many keys, how long a name, how far a view may be from its man are the game's own referee's to say, cmd.ts checkCmd, and a message's length is NET.maxText's) */
export function wireOk(c: unknown): c is WireCmd {
  const w = c as WireCmd | null;
  return !!w && typeof w === 'object' && keysOk(w.k) && (w.p === undefined || keysOk(w.p)) && Number.isInteger(w.b) && (w.c === undefined || Number.isInteger(w.c))
    && (w.w === undefined || (typeof w.w === 'number' && Number.isFinite(w.w))) && numsOk(w.m, 2) && numsOk(w.s, 4) && numsOk(w.v, 9);
}
/** `w` with one-tick edges laid over its own (keys gone down, clicks, the wheel's turn): how an edge that missed its tick rides a later record. Also the plain
 *  copy of a record in the wire's own field order, with nothing else on it (`withEdges(w, [], 0, 0)`) */
export function withEdges(w: WireCmd, p: readonly string[], c: number, wheel: number): WireCmd {
  const out: WireCmd = { k: w.k, b: w.b, m: w.m, s: w.s, v: w.v }, ps = w.p ? w.p.slice() : [];
  for (const k of p) if (!ps.includes(k)) ps.push(k);
  const cs = (w.c ?? 0) | c, ws = (w.w ?? 0) + wheel;
  if (ps.length) out.p = ps; if (cs) out.c = cs; if (ws) out.w = ws;
  return out;
}
/** `w` with its one-tick edges taken off: what a seat's tick runs on when no record of its own came for it (the keys and the buttons stay held, the view stays) */
export function heldOf(w: WireCmd): WireCmd { return { k: w.k, b: w.b, m: w.m, s: w.s, v: w.v }; }
/** `w` with nothing held either: a seat nobody is sitting in (its view stays where it was) */
export function idleOf(w: WireCmd): WireCmd { return { k: [], b: 0, m: w.m, s: w.s, v: w.v }; }
/** the record of a seat that has never sent one: nothing held, a plain view over the origin (the referee holds the look-at near the seat's own man) */
export const FIRST: WireCmd = { k: [], b: 0, m: [640, 360], s: [1280, 720, 1280, 720], v: [0, 0, 0, 0, 0.96, 21, 0.6, 16 / 9, 0] };

/** a message as text and back, numbers to the bit */
export const netText = (m: unknown): string => JSON.stringify(m, numWord);
export const netParse = (t: string): unknown => JSON.parse(t, wordNum);

/** the server's states: `waiting` until every seat is sat in and its client has stood its game (no tick runs), `playing`, `ended` (the mission's own end: the
 *  game runs on a short grace) */
export type NetState = 'waiting' | 'playing' | 'ended';
/** how the mission ended, by the game's own verdict (game/verdict.ts): won or lost, its word ('CLEAR', 'RESCUED', 'DOWN', …) and its reason */
export interface Outcome { won: boolean; end: string; why: string; }

// ---- client → server
/** the first message: who is asking. `key` = a seat's key (the control plane hands one to each player); none = a watcher, who is sent every tick and sends none.
 *  `build` = the build the client runs: a server that names its own lets in no other */
export interface Hello { k: 'hello'; proto: number; build: string; game: string; key?: string; }
/** a seat's client has stood its game from the welcome's recipe: the fight starts when every seat has said so */
export interface Ready { k: 'ready'; }
/** the seat's record for tick `t` (one a client tick; a client never sends a tick twice) */
export interface In { k: 'in'; t: number; c: WireCmd; }
/** the client's own state word after tick `t` did not match the server's */
export interface Lost { k: 'lost'; t: number; w: string; }
export type ToServer = Hello | Ready | In | Lost;

// ---- server → client
/** the answer to a hello: the seat (−1 a watcher) of how many, the recipe to stand the game from, and how many ticks have run (they follow, as `tape` messages) */
export interface Welcome { k: 'welcome'; proto: number; game: string; build: string; seat: number; seats: number; hz: number; recipe: Recipe; tick: number; state: NetState; }
/** tick `t` as it ran: each seat's record (0 = the seat's record of the tick before with its edges off), and every `wordEvery` ticks the state word after it */
export interface Tick { k: 't'; t: number; c: (WireCmd | 0)[]; w?: string; }
/** a stretch of the run for whoever is behind (a joiner; a connection that was backed up): ticks `from` on, each as a Tick's `c` (and `w` where the tick carried
 *  one). A stretch may begin before the tick its reader is due: he passes over what he has */
export interface Tape { k: 'tape'; from: number; ticks: { c: (WireCmd | 0)[]; w?: string }[]; }
/** how your records are arriving: when tick `t` ran the server held `l` of yours for later ticks (under NET.lead: send sooner; over NET.leadMax: later) */
export interface Pace { k: 'pace'; t: number; l: number; }
export interface State { k: 'state'; state: NetState; tick: number; seats: boolean[]; outcome?: Outcome; }
export interface Bye { k: 'bye'; why: string; }
export type ToClient = Welcome | Tick | Tape | Pace | State | Bye;
