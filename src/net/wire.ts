// THE WIRE OF A NETWORKED GAME (two-player co-op: a game server with authority, src/net/server.ts, and its clients, src/net/client.ts). What travels is the
// INPUT TIMELINE, never the state: the game plays the same fight to the bit from its start and its seats' records on every machine (tools/qa/mpclient-page.cjs holds a
// Chromium page to a bun server state word by state word), so the server decides which record each seat's tick ran on and tells everybody; a state word
// (words.ts foldWord: every part of the game) twice a second is the alarm. Messages are JSON text over a WebSocket, numbers to the bit (numtext.ts).
//   A RECORD ON THE WIRE (WireCmd) is what a player's hand did, minus what the game works out again for a seat it does not trust (rust/game_rules cmd: the eye, the forward, both
//   view matrices, from the look-at, the angles, the distance and the lens). EVERY seat of a networked game is such a seat on EVERY machine — the server and each
//   client, a client's own seat too — so one working makes the view everywhere and about a hundred bytes travel a tick, not a kilobyte.
//   WHAT A GAME IS STOOD FROM rides the welcome: a START (start.ts: the level's file, the address's words, the die, the seats, what the host's machine keeps), which every machine
//   hands the game's own start in the module.
// Host-neutral: no DOM, no bun, no game.
import { SeatInput } from '../draw/input';
import { numWord, wordNum } from './numtext';
import type { GameKey } from '../draw/names';
import type { NetStart } from './start';

/** `proto`: the wire's version (a hello of another is refused). `hz`: ticks a second (the fixed step's). `wordEvery`: ticks between state words. `lead`: how many
 *  of a seat's records the server likes to hold ahead of the tick it is running (the jitter buffer: Rocket League's input buffer) and `leadMax` the most before
 *  it tells the client to slow. `marginMax`: the furthest past the server's tick a client aims its records. `paceEvery`: ticks between the notes that tell a
 *  client how its records are arriving. `burst`: how far past the tick the clock names a client's bunched beats may run on. `clockMs`: a client knows the
 *  server's tick by the least-delayed tick message of the last two spans this long (so a server that let its clock slip, or a way that got longer, is followed
 *  inside two of them). `horizon`: a record for a tick further ahead than this is not kept. `catchUp`: the most ticks a host runs in one go when it is behind,
 *  and `slipAt` how far behind the server lets its game's clock slip instead. `tapeChunk`: ticks a message when a joiner is sent the run so far. `watchers`: the
 *  most connections with no seat. `maxText`: the longest message a server reads from a client (a record is a few hundred characters). `retryMs`: how long a
 *  server waits on a backed-up connection that never says it has drained before it tries the run so far again. */
export const NET = { proto: 3, hz: 60, wordEvery: 30, lead: 2, leadMax: 6, marginMax: 60, paceEvery: 10, burst: 2, clockMs: 1000, horizon: 180, catchUp: 4, slipAt: 30, tapeChunk: 600, watchers: 4, maxText: 4096, retryMs: 250, nameMax: 12 };
/** A PLAYER'S NAME AS IT MAY TRAVEL AND BE SHOWN (Hello.name, State.names; src/host/myname.ts): text with something to read in it — a letter, a digit or a
 *  picture — no control characters, no format characters but the two joiners a script or an emoji is written with (no direction overrides, no zero
 *  width space), none of the letters that draw as nothing, its spaces single, at most NET.nameMax characters as a reader counts them. Anything else — no
 *  name, a record, a blank — is no name: ''. Asked at BOTH trust boundaries: a server of a client's hello, a client of a server's state; and a page
 *  always writes one as text, never as markup. (The wire spells four numbers as words — numtext.ts numWord — and reads them back as numbers wherever they
 *  stand: a player called 'nan' or 'inf' is given his word back here.) */
const BLANKS = /[\u115F\u1160\u2800\u3164\uFFA0]/g, READS = /[\p{L}\p{N}\p{Extended_Pictographic}]/u;
export function cleanName(raw: unknown): string {
  if (typeof raw === 'number') raw = numWord('', raw);
  if (typeof raw !== 'string') return '';
  const text = raw.replace(/[\p{Cc}\p{Zl}\p{Zp}]/gu, ' ').replace(/[\p{Cs}\p{Co}\p{Cn}]/gu, '').replace(/(?![\u200C\u200D])\p{Cf}/gu, '').replace(BLANKS, '').replace(/\s+/g, ' ').trim();
  const Seg = (Intl as unknown as { Segmenter?: new (l?: string, o?: { granularity: 'grapheme' }) => { segment(t: string): Iterable<{ segment: string }> } }).Segmenter;
  const parts = Seg ? Array.from(new Seg(undefined, { granularity: 'grapheme' }).segment(text), x => x.segment) : [...text];
  const cut = parts.slice(0, NET.nameMax).join('').replace(/^[\u200C\u200D\s]+|[\u200C\u200D\s]+$/g, '');
  return READS.test(cut) ? cut : '';
}

/** a seat's record as it travels: keys held · keys gone down · his input's numbers, as they are handed (src/draw/input.ts SeatInput). The one-tick EDGES are `p`, the buttons gone down
 *  and the wheel */
export interface WireCmd { k: string[]; p?: string[]; n: number[]; }
/** a record's numbers, by what each is */
export function inputNumbers(o: { buttons?: number; clicked?: number; wheel?: number; mouse: readonly number[]; size: readonly number[]; target: ArrayLike<number>; yaw: number; pitch: number; distance: number; fov: number; aspect: number; aimLead?: number }): number[] {
  const n = new Array<number>(SeatInput.row).fill(0);
  n[SeatInput.buttons] = o.buttons ?? 0; n[SeatInput.clicked] = o.clicked ?? 0; n[SeatInput.wheel] = o.wheel ?? 0; [n[SeatInput.mx], n[SeatInput.my]] = o.mouse; [n[SeatInput.w], n[SeatInput.h], n[SeatInput.pw], n[SeatInput.ph]] = o.size;
  for (let i = 0; i < 3; i++) n[SeatInput.target + i] = o.target[i];
  n[SeatInput.yaw] = o.yaw; n[SeatInput.pitch] = o.pitch; n[SeatInput.distance] = o.distance; n[SeatInput.fov] = o.fov; n[SeatInput.aspect] = o.aspect; n[SeatInput.aimLead] = o.aimLead ?? 0;
  return n;
}
/** numbers with these three in them (the same numbers, where they are) */
function edged(n: number[], buttons: number, clicked: number, wheel: number): number[] {
  if (n[SeatInput.buttons] === buttons && n[SeatInput.clicked] === clicked && n[SeatInput.wheel] === wheel) return n;
  const out = n.slice(); out[SeatInput.buttons] = buttons; out[SeatInput.clicked] = clicked; out[SeatInput.wheel] = wheel;
  return out;
}

const keysOk = (a: unknown): a is string[] => Array.isArray(a) && a.every(k => typeof k === 'string');
const numsOk = (a: unknown, n: number): a is number[] => Array.isArray(a) && a.length === n && a.every(x => typeof x === 'number' && Number.isFinite(x));
/** `c` is a record as the wire says one is: every field there, of its kind and finite. A server asks before it keeps what a client sent. (A SHAPE only: how
 *  many keys, how long a name, how far a view may be from its man are the game's own referee's to say, and a message's length is NET.maxText's) */
export function wireOk(c: unknown): c is WireCmd {
  const w = c as WireCmd | null;
  return !!w && typeof w === 'object' && keysOk(w.k) && (w.p === undefined || keysOk(w.p)) && numsOk(w.n, SeatInput.row) && Number.isInteger(w.n[SeatInput.buttons]) && Number.isInteger(w.n[SeatInput.clicked]);
}
/** `w` with one-tick edges laid over its own (keys gone down, clicks, the wheel's turn): how an edge that missed its tick rides a later record. Also the plain
 *  copy of a record in the wire's own field order, with nothing else on it (`withEdges(w, [], 0, 0)`) */
export function withEdges(w: WireCmd, p: readonly string[], c: number, wheel: number): WireCmd {
  const out: WireCmd = { k: w.k, n: edged(w.n, w.n[SeatInput.buttons], w.n[SeatInput.clicked] | c, w.n[SeatInput.wheel] + wheel) }, ps = w.p ? w.p.slice() : [];
  for (const k of p) if (!ps.includes(k)) ps.push(k);
  if (ps.length) out.p = ps;
  return out;
}
/** `w` with its one-tick edges taken off: what a seat's tick runs on when no record of its own came for it (the keys and the buttons stay held, the view stays) */
export function heldOf(w: WireCmd): WireCmd { return { k: w.k, n: edged(w.n, w.n[SeatInput.buttons], 0, 0) }; }
/** `w` with nothing held either: a seat nobody is sitting in (its view stays where it was) */
export function idleOf(w: WireCmd): WireCmd { return { k: [], n: edged(w.n, 0, 0, 0) }; }
/** THE HANDS COME OFF (a card went up over the player's game, his tab was hidden): nothing held, and the game's own one-tick word for it, 'Paused',
 *  so the tick that takes it lets go UNSENT what a button let go would have done — a held launcher's round, a held list's row */
export function restOf(w: WireCmd): WireCmd { return { k: [], p: ['Paused' satisfies GameKey], n: edged(w.n, 0, 0, 0) }; }
/** the record of a seat that has never sent one: nothing held, a plain view over the origin (the referee holds the look-at near the seat's own man) */
export const FIRST: WireCmd = { k: [], n: inputNumbers({ mouse: [640, 360], size: [1280, 720, 1280, 720], target: [0, 0, 0], yaw: 0, pitch: 0.96, distance: 21, fov: 0.6, aspect: 16 / 9 }) };

/** a message as text and back, numbers to the bit */
export const netText = (m: unknown): string => JSON.stringify(m, numWord);
export const netParse = (t: string): unknown => JSON.parse(t, wordNum);

/** the server's states: `waiting` until every seat is sat in and its client has stood its game (no tick runs), `playing`, `ended` (the mission's own end: the
 *  game runs on a short grace) */
export type NetState = 'waiting' | 'playing' | 'ended';
/** how the mission ended, by the game's own verdict: won or lost, its word ('CLEAR', 'RESCUED', 'DOWN', …) and its reason */
export interface Outcome { won: boolean; end: string; why: string; }

// ---- client → server
/** the first message: who is asking. `key` = a seat's key (the control plane hands one to each player); none = a watcher, who is sent every tick and sends none.
 *  `build` = the build the client runs: a server that names its own lets in no other */
export interface Hello { k: 'hello'; proto: number; build: string; game: string; key?: string; /** what a seat's player calls himself (cleanName; none = his seat's number) */ name?: string; }
/** a seat's client has stood its game from the welcome's start: the fight starts when every seat has said so */
export interface Ready { k: 'ready'; }
/** the seat's record for tick `t` (one a client tick; a client never sends a tick twice) */
export interface In { k: 'in'; t: number; c: WireCmd; }
/** the client's own state word after tick `t` did not match the server's */
export interface Lost { k: 'lost'; t: number; w: string; }
export type ToServer = Hello | Ready | In | Lost;

// ---- server → client
/** the answer to a hello: the seat (−1 a watcher) of how many, what to stand the game from, and how many ticks have run (they follow, as `tape` messages) */
export interface Welcome { k: 'welcome'; proto: number; game: string; build: string; seat: number; seats: number; hz: number; start: NetStart; tick: number; state: NetState; }
/** tick `t` as it ran: each seat's record (0 = the seat's record of the tick before with its edges off), and every `wordEvery` ticks the state word after it */
export interface Tick { k: 't'; t: number; c: (WireCmd | 0)[]; w?: string; }
/** a stretch of the run for whoever is behind (a joiner; a connection that was backed up): ticks `from` on, each as a Tick's `c` (and `w` where the tick carried
 *  one). A stretch may begin before the tick its reader is due: he passes over what he has */
export interface Tape { k: 'tape'; from: number; ticks: { c: (WireCmd | 0)[]; w?: string }[]; }
/** how your records are arriving: when tick `t` ran the server held `l` of yours for later ticks (under NET.lead: send sooner; over NET.leadMax: later) */
export interface Pace { k: 'pace'; t: number; l: number; }
/** `names`: each seat's player's name as his client last said it ('' = none), kept when he leaves */
export interface State { k: 'state'; state: NetState; tick: number; seats: boolean[]; names?: string[]; outcome?: Outcome; }
/** why a client is let go, as a word a page picks its line by (`why` is for a log): `proto` another wire, `game` another game's id, `build` another build,
 *  `seat` no seat has that key, `full` no room to watch, `hello` it spoke before saying hello, `done` the game is over or its server stopped */
export type ByeCode = 'proto' | 'game' | 'build' | 'seat' | 'full' | 'hello' | 'done';
export interface Bye { k: 'bye'; code: ByeCode; why: string; }
export type ToClient = Welcome | Tick | Tape | Pace | State | Bye;
