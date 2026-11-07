// THE LOBBY: how a page gets INTO a networked game (page.ts is the page once it is in one). No DOM and no game here: the words, the control plane's three
// calls (cloud/convex/games.ts make · join · get) and the addresses. The screen is ui/coop.ts.
//   A CO-OP GAME HAS ONE DOOR: `Lobby.seat(code)`. Its address is  /?join=<CODE>  (the link a friend opens; a game's address pasted into a tab that holds no
//   seat comes through it too); the title's Host and Join go through the same call with no page load of their own. Whoever comes through it is given a seat
//   by the control plane (`games:join`: the first free one, or the one this tab already holds), waits for the game's server to answer, and is sent on to the server's own
//   address — `?net=…&netgame=…&netkey=…`, page.ts — with `join=<CODE>` still on it, so the page in the game can show the code and a reload finds the game.
//   WHO A SEAT'S HOLDER IS: the TAB (an id kept in sessionStorage). Two windows of one browser on one machine are two players, a tab loaded again is the same
//   one. (The browser's own id — cloud/cloud.ts cloudPid — is one for every window of it: the second window would be handed the first one's seat.)
//   WHAT THE HOST HANDS OVER (the setup record): the mission's own address as the starting page would load it — its level, hour, weather, way in and
//   difficulty — with the team of four and two seats on it (`coopScenario`: each player brings two teammates, game/seats.ts dealSeats) and the levers this
//   machine keeps written out (the server keeps nothing: a lever the page would read back from its store has to ride on the address), and a fresh die.
//   NOT CARRIED: the kit this machine keeps (both players play the kit as issued) and knob tables changed in the debugger.
// THE SWITCH: COOP.on — on for a real page, `?coop=0` = the page as it was (no Co-op row on the title, a `?join=` on the address is not read). Nothing a tick
// reads: a game is the server's recipe whatever this says.
import type { Wire } from '../cloud/wire';
import { NETPLAY, NET_GAME, NET_KEY, NET_JOIN } from './page';
import { bare } from '../scene/scenarioList';

export const COOP = {
  on: typeof window !== 'undefined' && typeof window.addEventListener === 'function',
  /** players in a co-op game, and the teammates the two of them bring between them */
  seats: 2, squad: 4,
  /** how often a waiting page asks the control plane how its game is, and how long it waits for a server before it says so (the control plane's own clock
   *  for a start is two minutes: cloud/convex/games.ts policy) */
  pollMs: 500, startMs: 150_000,
};
try { if (/^(0|off|false|no)$/i.test(new URLSearchParams(location.search).get('coop') ?? '')) COOP.on = false; } catch { /* no page (the harness) */ }

/** a code's letters and length, as the control plane deals them (cloud/convex/games.ts MP: no vowels, no digits) */
export const CODE = { letters: 'BCDFGHJKLMNPQRSTVWXZ', len: 5 };
/** what was typed, as far as it is a code: its letters in capitals, at most a code's length */
export function codeTyped(text: string): string { let out = ''; for (const c of text.toUpperCase()) if (CODE.letters.includes(c) && out.length < CODE.len) out += c; return out; }
export const codeWhole = (code: string): boolean => code.length === CODE.len && codeTyped(code) === code;

/** what a page is told of a game (cloud/convex/games.ts `view`): `seatKey`, `address` and `game` only for a tab that holds a seat */
export interface GameView { code: string; state: 'starting' | 'waiting' | 'playing' | 'ended' | 'failed'; why: string; build: string; seats: boolean[]; connected: boolean[]; seat: number; seatKey: string; address: string; game: string; outcome: unknown }

/** THE MISSION AS A CO-OP GAME'S SETUP: `search` is the address the starting page would load for it; `kept` the settings this machine keeps off the address
 *  (difficulty levers: their address word and value). The team is the four, the seats two; a `loadout` or a `pick` word is the page's own and does not ride */
export function coopScenario(search: string, kept: [string, string][] = []): string {
  const q = new URLSearchParams(search.replace(/^[^?]*\?/, ''));
  for (const k of ['squad4', 'loadout', 'pick', NET_JOIN, NETPLAY, NET_GAME, NET_KEY]) q.delete(k);
  q.set('squad', String(COOP.squad)); q.set('seats', String(COOP.seats));
  for (const [k, v] of kept) if (!q.has(k)) q.set(k, v);
  return bare(q);   // (bare flags left bare: `?roomclear`, as the starting page writes them)
}
/** THE DOOR'S ADDRESS, and the link to send: this page's own address with the code and nothing else of this visit's */
export const joinLink = (origin: string, path: string, code: string): string => `${origin}${path}?${NET_JOIN}=${code}`;
/** …and the address of the game's server for a tab that holds a seat (page.ts reads it): this visit's words, the server, the game's name, the seat's key */
export function netSearch(now: URLSearchParams | string, v: Pick<GameView, 'address' | 'game' | 'seatKey' | 'code'>): string {
  const q = new URLSearchParams(now); q.set(NETPLAY, v.address); q.set(NET_GAME, v.game); q.set(NET_KEY, v.seatKey); q.set(NET_JOIN, v.code); return '?' + q.toString();
}

/** a fresh id for a seat's holder (40 characters off 160 bits of the browser's dice: the shape the control plane takes for a browser's id) */
export function freshId(): string { const a = new Uint8Array(20); crypto.getRandomValues(a); let id = ''; for (const b of a) id += (b % 36).toString(36) + (b >> 4).toString(16); return id; }
/** a die for a new game: a whole number the server takes (tools/mp/stand.ts: up to 4294967295) */
export function freshSeed(): number { const a = new Uint32Array(1); crypto.getRandomValues(a); return a[0]; }

/** WHY NOT, as the player is told it. The control plane's refusals begin with a word (`full:` · `build:` · `code:`); anything else did not get there */
export function lobbyWhy(e: unknown, doing: 'host' | 'join'): string {
  const m = String((e as { message?: unknown } | null)?.message ?? e), refused = !!(e as { refused?: boolean } | null)?.refused;
  if (/\bfull:/.test(m)) return doing === 'host' ? 'Every game server is busy right now. Try again in a minute.' : 'That game already has two players.';
  if (/\bbuild:/.test(m)) return doing === 'host' ? 'This page is out of date. Reload it and try again.' : 'That game is on another version. Reload the page and try again.';
  if (/\bcode:/.test(m)) return doing === 'host' ? 'The game couldn’t be made. Try again.' : 'There’s no game with that code. Check the code with your friend.';
  return refused ? (doing === 'host' ? 'The game couldn’t be made. Try again.' : 'You can’t join that game.') : 'This page couldn’t reach the game service. Check your connection and try again.';
}
/** …and why a game that was made is not one to play ('' = it is, or may yet be) */
export function gameWhy(v: GameView | null): string {
  if (!v) return 'There’s no game with that code. Check the code with your friend.';
  if (v.state === 'failed') return 'The game’s server didn’t start. Host a new game.';
  if (v.state === 'ended') return 'That game is over.';
  return v.seat < 0 ? 'That game already has two players.' : '';
}

/** THE CONTROL PLANE'S THREE CALLS, for one tab (`pid`) of one build */
export class Lobby {
  constructor(readonly wire: Wire, readonly pid: string, readonly build: string) {}
  /** a game of `scenario` (coopScenario) on a fresh die: the host holds its first seat */
  make(scenario: string, seed: number = freshSeed()): Promise<GameView> {
    return this.wire.mutation<GameView>('games:make', { pid: this.pid, build: this.build, setup: { scenario, seed, knobs: {}, seats: Array.from({ length: COOP.seats }, () => ({})) } });
  }
  join(code: string): Promise<GameView> { return this.wire.mutation<GameView>('games:join', { pid: this.pid, code, build: this.build }); }
  get(code: string): Promise<GameView | null> { return this.wire.query<GameView | null>('games:get', { pid: this.pid, code }); }
  /** THROUGH THE DOOR: a seat in the game `code` names, and its server's address once the server answers. Resolves to the view (`address` set) or rejects
   *  with the words for why not. `told` hears each state on the way; `sleep` and `now` are the host's clock (a check hands its own) */
  async seat(code: string, told: (v: GameView) => void = () => { /* */ }, sleep: (ms: number) => Promise<void> = ms => new Promise(r => setTimeout(r, ms)), now: () => number = () => Date.now()): Promise<GameView> {
    let v: GameView | null;
    try { v = await this.join(code); } catch (e) { throw new Error(lobbyWhy(e, 'join')); }
    for (const until = now() + COOP.startMs; ;) {
      const why = gameWhy(v); if (why) throw new Error(why);
      told(v!);
      if (v!.address && (v!.state === 'waiting' || v!.state === 'playing')) return v!;
      if (now() > until) throw new Error('The game’s server didn’t start. Host a new game.');
      await sleep(COOP.pollMs);
      try { v = await this.get(code); } catch (e) { if ((e as { refused?: boolean } | null)?.refused) throw new Error(lobbyWhy(e, 'join')); }   // (a call that did not get there is asked again: the game is still being started)
    }
  }
}
