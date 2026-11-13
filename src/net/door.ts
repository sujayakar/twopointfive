// THE DOOR OF A NETWORKED GAME, for any page: its address and its seat's key, the socket and the connection made worse on purpose, the hello up to the welcome or the
// words that say why there is none, HOW FAST the confirmed ticks are walked, the look ahead, and the one line a player is shown while something is not as it
// should be. Nothing here knows a game: whoever holds one stands it from what the welcome carries and steps it (host/drawnet.ts).
// THE SEAT'S KEY comes on the address once and is kept for the tab (sessionStorage); it is taken off the address before anything else is asked, so nothing
// that reads the address later (a visit's record, a replay's head) reads the key — refused or not.
// HOW FAST: a page tick walks one confirmed tick, two while more than NET.lead wait (the page's clock and the server's are two clocks). A page far behind
// (NETDOOR.far ticks: it joined late, its tab slept) CATCHES UP as a replay seeks — the mixer shut, as many ticks a page tick as fit in NETDOOR.farMs, its
// hands off the keys, nothing drawn between two of those ticks.
import { NETPLAY, NET_GAME, NET_KEY, NET_JOIN, NET_PREDICT, NET_LAG, NET_JITTER, NET_LOSS } from './address';
import { NetClient, type ClientHost } from './client';
import { onWay, wayOff, type Way } from './delay';
import { anotherVersion } from './lobby';
import type { AheadView } from '../view/read';
import { FORESEE, Foresee, type Foreseen } from './foresee';
import { NET, type WireCmd } from './wire';
import { built, pageOld } from '../host/buildname';

const KEEP = 'tps.net.seat', KEEP_TRY = 'tps.net.try';
/** `helloMs`: how long the page waits for the server's welcome. `walkMs`: past it a page tick that is level does not walk its second tick (one is always
 *  walked, two at most: see HOW FAST).
 *  `far`: confirmed ticks waiting from which the page catches up as a seek does, `farMs` a page tick at it. `noteMs`: how long a passing line stays. */
export const NETDOOR = { helloMs: 10000, walkMs: 8, far: 120, farMs: 12, noteMs: 3000 };

// THIS PAGE'S CONNECTION, MADE WORSE ON PURPOSE (2026-11-02; delay.ts has what each number does). Two players on one good network see none of what a real
// connection does, so the debugger has a Network page on a networked game (ui/netrows.ts) whose rows move these numbers LIVE, and the look ahead's switch
// beside them. A page's numbers are its own: each player's page sets its own connection, and what one player does reaches the other through both.
// WHERE A TRY IS KEPT: for the tab (KEEP_TRY), because joining again is a reload and a try that a reload forgets is set again every time; never longer,
// because a connection left bad by a test a week ago would be a trap. An address word wins over what is kept on every load of that address.
/** the page's own way to its server (all zero = the wire as it is) */
export const NETWAY: Way = wayOff();
export interface NetTry { lagMs: number; jitterMs: number; lossPct: number; predict: boolean }
/** (the most each takes: with all three at their most a hello and its welcome still make the trip inside NETDOOR.helloMs) */
const TRY_WORDS: [word: string, key: 'lagMs' | 'jitterMs' | 'lossPct', most: number][] = [[NET_LAG, 'lagMs', 1000], [NET_JITTER, 'jitterMs', 1000], [NET_LOSS, 'lossPct', 20]];
const within = (v: unknown, most: number): number => { const n = Number(v); return n > 0 ? Math.min(n, most) : 0; };
export const netTried = (): NetTry => ({ lagMs: NETWAY.lagMs, jitterMs: NETWAY.jitterMs, lossPct: NETWAY.lossPct, predict: FORESEE.on });
/** the debugger's rows: this page's connection and its look ahead changed now, and kept for the tab */
export function netTry(p: Partial<NetTry>): void {
  for (const [, k, most] of TRY_WORDS) if (p[k] !== undefined) NETWAY[k] = within(p[k], most);
  if (p.predict !== undefined) FORESEE.on = p.predict;
  if (keeping === null) keeping = setTimeout(() => { keeping = null; try { sessionStorage.setItem(KEEP_TRY, JSON.stringify(netTried())); } catch { /* a browser that keeps nothing */ } }, 200);   // (a row dragged writes once it rests)
}
let keeping: ReturnType<typeof setTimeout> | null = null;
/** nothing passes for that long, both ways (a connection that goes quiet and comes back; never kept; 0 = a stall taken back) */
export function netStall(ms: number): void { NETWAY.stallUntil = ms > 0 ? performance.now() + within(ms, 10000) : 0; }
/** a load's try: what the tab keeps, under what the address says (`words`: the address's own, by word) */
function tryFrom(words: Record<string, string>): void {
  let kept: Partial<NetTry> = {}; try { const k = JSON.parse(sessionStorage.getItem(KEEP_TRY) ?? 'null') as unknown; if (k && typeof k === 'object') kept = k as Partial<NetTry>; } catch { /* nothing kept */ }
  for (const [w, k, most] of TRY_WORDS) NETWAY[k] = within(w in words ? words[w] : kept[k], most);
  FORESEE.on = NET_PREDICT in words ? !/^(0|off|false|no)$/i.test(words[NET_PREDICT]) : kept.predict !== false;
}

export interface NetPageState { seat: number; state: string; confirmed: number; heard: number; behind: number; far: boolean; wordsOk: number; bad: { t: number; server: string; mine: string } | null; word: string;
  sent: number; holds: number; skips: number; /** a message from the server waited this long in the page's own queue (ms, eased: a worse connection asked for) */ heldMs: number; margin: number; lead: number; pending: number; broken: string; bye: string; refuse: string; outcome: unknown; walkMs: number;
  /** his own man ahead of the game (null: drawn where the game has him): records played, where they take him and how far that is from where the game has him
   *  (m), what is left of a correction (m), corrections so far past a centimetre and the longest (m), and what a look ahead costs (ms, eased) */
  ahead: { n: number; pos: number[]; move: number; off: number; fixes: number; fixMax: number; ms: number } | null; }

/** THE DOOR AN ADDRESS NAMES, opened: null = it names no networked game. `name`: what this browser's player calls himself. `own`: the words the page's address is to
 *  carry beside the game's own, whatever the welcome says the game is */
export async function doorFromUrl(q: URLSearchParams, name: string): Promise<{ door: NetDoor; own: Record<string, string> } | null> {
  const url = q.get(NETPLAY); if (url === null) return null;
  const game = q.get(NET_GAME) ?? 'local', seat = seatKey(q, url, game);
  const tries: Record<string, string> = {}; for (const k of TRY_ALL) { const v = q.get(k); if (v !== null) tries[k] = v; }
  tryFrom(tries);
  const door = new NetDoor(url, game, seat.key === null ? undefined : seat.key, name);
  await door.open();
  return { door, own: { [NETPLAY]: url, [NET_GAME]: game, ...(seat.kept || seat.key === null ? {} : { [NET_KEY]: seat.key }), ...(q.has(NET_JOIN) ? { [NET_JOIN]: q.get(NET_JOIN) ?? '' } : {}), ...tries } };
}
/** the page is sent to the game's own address, where it is not on it: true = it is on its way (stop booting) */
export function movedTo(door: NetDoor, want: string): boolean {
  if (new URLSearchParams(location.search).toString() === new URLSearchParams(want).toString()) return false;
  door.close(); location.replace(location.pathname + want + location.hash); return true;
}
/** the address without the game's words: where a player who leaves a networked game goes (the cards' Quit) */
export function netLeave(search: string): string { const u = new URLSearchParams(search); for (const k of [NETPLAY, NET_GAME, NET_KEY, NET_JOIN, ...TRY_ALL]) u.delete(k); const s = u.toString(); return s ? '?' + s : ''; }
const TRY_ALL = [NET_PREDICT, ...TRY_WORDS.map(w => w[0])];
/** does this tab hold a seat's key for the game its address names: on the address, or put away for the tab? (A game's address with no key is a watcher's:
 *  one that also names a co-op game's code goes through the lobby's door first — net/lobby.ts) */
export function netKeyed(q: URLSearchParams): boolean {
  if (q.has(NET_KEY)) return true;
  try { const k = JSON.parse(sessionStorage.getItem(KEEP) ?? 'null') as unknown; return Array.isArray(k) && k[0] === q.get(NETPLAY) && k[1] === (q.get(NET_GAME) ?? 'local') && typeof k[2] === 'string'; } catch { return false; }
}
/** the key off the address (a refused page boots as itself: the key it came with is not its address's to keep) */
export function keyOff(): void { try { const u = new URLSearchParams(location.search); u.delete(NET_KEY); history.replaceState(null, '', location.pathname + '?' + u.toString() + location.hash); } catch { /* a sandboxed frame */ } }
/** the seat's key: off the address (and put away for the tab, so the address the page moves to need not carry it), else the one put away for this game.
 *  `kept` = the tab holds it (a browser that keeps nothing leaves the key on the address) */
function seatKey(q: URLSearchParams, url: string, game: string): { key: string | null; kept: boolean } {
  const on = q.get(NET_KEY);
  try {
    if (on !== null) { sessionStorage.setItem(KEEP, JSON.stringify([url, game, on])); return { key: on, kept: true }; }
    const k = JSON.parse(sessionStorage.getItem(KEEP) ?? 'null') as unknown;
    if (Array.isArray(k) && k[0] === url && k[1] === game && typeof k[2] === 'string') return { key: k[2], kept: true };
  } catch { /* a browser that keeps nothing */ }
  return { key: on, kept: false };
}

/** what a walk of confirmed ticks goes round: `far`: ticks nobody sees begin / are over (the mixer shut; its tidy when the page is level) · `run`: a run of ticks between
 *  which nothing of the page reads the game begins / ends */
export interface WalkRound { far(on: boolean): void; run?(on: boolean): void }

export class NetDoor {
  /** why this page is not a client of the game ('' = it is) */
  refuse = '';
  readonly client: NetClient;
  /** catching up: confirmed ticks are walked that nobody sees */
  far = false;
  /** how long a message from the server really waited in this page's own queue before it was read (ms, eased; 0 = the connection as it is: delay.ts), and what a walked tick cost */
  heldMs = 0; walkMs = 0;
  readonly fore = new Foresee();
  private ws: WebSocket | null = null; private line: HTMLElement | null = null; private said = ''; private note = ''; private noteUntil = 0;

  constructor(readonly url: string, readonly game: string, readonly key: string | undefined, name: string) {
    this.client = new NetClient(onWay<string>(NETWAY, text => { const ws = this.ws; if (ws && ws.readyState === 1) ws.send(text); }), { game, build: '', ...(key !== undefined ? { key, name } : {}) });   // (a watcher says no name)
  }

  /** connect, say hello, wait for the welcome (or for why there is none) */
  async open(): Promise<void> {
    const c = this.client, named = built();   // (asked for while the socket opens)
    await new Promise<void>(done => {
      let ws: WebSocket; try { ws = this.ws = new WebSocket(this.url); } catch { this.refuse = 'This page can’t connect to that game’s address.'; done(); return; }
      let met = false; const settle = (why: string | Promise<string>) => { if (met) return; met = true; clearTimeout(t); void Promise.resolve(why).then(w => { if (w) this.refuse ||= w; done(); }); };
      const t = setTimeout(() => settle('The game’s server didn’t answer.'), NETDOOR.helloMs);
      ws.onopen = () => void named.then(b => { c.who.build = b.made; c.hello(); });
      const down = onWay<() => void>(NETWAY, f => f());   // (what comes down the socket, its close too, in the order it came)
      ws.onmessage = ev => { const came = performance.now(); down(() => { const now = performance.now(); this.heldMs += (now - came - this.heldMs) * (this.heldMs ? 0.1 : 1); c.message(String(ev.data), now); if (c.byeCode === 'seat') forget(); if (!met && (c.welcome || c.bye)) settle(c.welcome ? '' : refusal(c.byeCode)); }); };
      ws.onclose = () => down(() => { if (this.ws !== ws) return; if (!met) settle(c.bye ? refusal(c.byeCode) : 'This page couldn’t reach the game’s server.'); else if (!c.bye) c.broken ||= 'the connection closed'; });
      ws.onerror = () => { /* its close follows */ };
    });
    if (this.refuse) this.turn(this.refuse);
  }
  close(): void { const ws = this.ws; this.ws = null; if (ws) { ws.onclose = null; try { ws.close(); } catch { /* gone */ } } }
  /** this page is no client of the game, and says why */
  turn(why: string): void { this.refuse = why; this.close(); this.say(why); }

  /** BEFORE a page tick's record: a page that wakes far behind sends no key made at a picture seconds old */
  wake(round: WalkRound): void { if (this.client.behind >= NETDOOR.far && !this.far) { this.far = true; round.far(true); } }
  /** the confirmed ticks that wait, as many as this page tick walks */
  walk(host: ClientHost, round: WalkRound): number {
    const c = this.client, t0 = performance.now(); let n = 0;
    this.wake(round);
    const far = this.far, most = far ? Infinity : c.behind > NET.lead ? 2 : 1, ms = far ? NETDOOR.farMs : NETDOOR.walkMs, level = (): void => { this.far = false; round.far(false); };
    // (ONE RUN A WALK: nothing of a page reads the game between two ticks of a walk, so a host that keeps a copy of it brings the copy up once, as the walk ends)
    round.run?.(true);
    try { while (c.behind && n < most && (n < 1 || performance.now() - t0 < ms)) n += c.walk(host, 1); } finally { round.run?.(false); }
    if (n) this.walkMs = (performance.now() - t0) / n;
    if (far && c.behind <= NET.lead) level();
    if ((c.bad || c.broken) && this.ws) { if (this.far) level(); this.close(); }   // (parted from the server: the seat is let go — the server holds no key for a seat nobody sits in — and the page stands where it is)
    this.status();
    return n;
  }
  /** the look ahead, at the end of a page tick (foresee.ts) */
  look(walk: ((cmds: WireCmd[]) => AheadView) | null): Foreseen | null { return this.fore.look(walk, this.client, this.far); }

  /** how the page is doing (`word`: the state word after the last tick) */
  state(word: string): NetPageState {
    const c = this.client, F = this.fore;
    return { seat: c.seat, state: c.state, confirmed: c.confirmed, heard: c.heard, behind: c.behind, far: this.far, wordsOk: c.wordsOk, bad: c.bad, word, sent: c.sent, holds: c.holds, skips: c.skips, heldMs: this.heldMs,
      margin: c.margin, lead: c.lead, pending: c.pending.length, broken: c.broken, bye: c.bye, refuse: this.refuse, outcome: c.outcome, walkMs: this.walkMs,
      ahead: F.last ? { n: F.last.n, pos: Array.from(F.last.to.pos), move: Math.hypot(F.last.by[0], F.last.by[1], F.last.by[2]), off: F.rest, fixes: F.fixes, fixMax: F.fixMax, ms: F.ms } : null };
  }

  // ---- the one line the player is shown while something is not as it should be
  private status(): void {
    const c = this.client;
    this.say(c.bad ? 'Your game went out of step with the server. Reload the page to join again.'
      : c.byeCode === 'seat' ? TAKEN
      : c.bye ? (c.outcome ? 'The mission is over.' : 'The game is over.')
      : c.broken ? 'You lost the connection to the game. Reload the page to join again.'
      : c.state === 'waiting' ? 'The other player isn’t here yet.'
      : this.far ? 'You’re catching up with the game.'
      : performance.now() < this.noteUntil ? this.note : '');
  }
  /** a line that passes (NETDOOR.noteMs) */
  pass(text: string): void { this.note = text; this.noteUntil = performance.now() + NETDOOR.noteMs; this.status(); }
  say(text: string): void {
    if (text === this.said || typeof document === 'undefined') return; this.said = text;
    if (!this.line) { const d = this.line = document.createElement('div'); d.id = 'netline'; d.style.cssText = 'position:fixed;left:50%;top:14%;transform:translateX(-50%);z-index:40;padding:8px 16px;border-radius:6px;background:rgba(10,12,14,.82);color:#e8e6df;font:500 15px/1.3 system-ui,sans-serif;pointer-events:none;max-width:80vw;text-align:center'; document.body.appendChild(d); }
    this.line.textContent = text; this.line.style.display = text ? 'block' : 'none';
  }
}

/** this key opens no seat any more: the tab forgets it, so a page loaded again comes by the game's code (net/lobby.ts) or watches */
const forget = (): void => { try { sessionStorage.removeItem(KEEP); } catch { /* a browser that keeps nothing */ } };
const TAKEN = 'Another player has your seat.';
export const CANT_PLAY = 'This page can’t play that game.';
/** a server's goodbye before any welcome (wire.ts ByeCode), as the player is told it */
async function refusal(code: string): Promise<string> {
  return code === 'build' || code === 'proto' ? anotherVersion(await pageOld())
    : code === 'seat' ? TAKEN
    : code === 'full' ? 'There’s no room left to watch that game.'
    : 'You can’t join that game.';
}
