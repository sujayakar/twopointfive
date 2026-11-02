// THE PAGE AS A CLIENT OF A GAME SERVER (the browser's side of src/net/: see wire.ts for what travels, client.ts for the client's core).
//   /?net=<ws address>&netgame=<the game's id>&netkey=<a seat's key>        (no key: a watcher)
// HOW IT STANDS THE GAME: as the engine replay stands a tape's (replay/recipestand.ts) — a networked game is a run tape written live. The page says hello, takes
// the server's RECIPE from the welcome, and is first sent to the recipe's own address with its seed, so the boot is that game's boot; then main.ts lays the
// recipe's knob tables and kit over this machine's at each point of the boot, exactly where it does for a replay. Once the game stands every seat is refereed
// (step.ts netSeats) and the server is told `ready`.
// WHO STEPS THE GAME: this file, through the loop's `tick` seam (host/loop.ts LoopCtx.tick). On each tick of the page the player's input is sampled into a
// record (game/cmd.ts sampleCmd, off the page's own camera) and SENT; the game itself is stepped only through the ticks the server has confirmed, each on the
// records the server decided, by step.ts netStep — the one tick every machine runs. So this page's game is the server's to the bit, and its state word is held
// against the server's every NET.wordEvery ticks. Round those ticks it does the host's half of one (Game.hostBefore / hostAfter: update's own lines).
// HOW FAST: a page tick walks one confirmed tick, two while more than NET.lead wait (the page's clock and the server's are two clocks). A page far behind
// (NETPAGE.far ticks: it joined late, its tab slept) CATCHES UP as a replay seeks — the mixer shut, as many ticks a page tick as fit in NETPAGE.farMs, its
// hands off the keys, nothing drawn between two of those ticks, and the picture's queues let go when it is level again (engineplay.ts unseenDone). Nothing smooths a tick that came late: no playout buffer is built.
// WHAT THE PLAYER FEELS TODAY: his man answers his keys when the server's tick comes back, a few ticks after the press (the way up, the server's buffer, the
// way down). Nothing is predicted yet: `client.pending` holds the records a prediction of his own man will play ahead.
// THE GAME IS THE SERVER'S: nothing on this page writes to it outside a confirmed tick. A card's Restart and a checkpoint's load are refused in words, Quit
// leaves the page (main.ts), no try, streak or best is filed for such a run, a page that parts from the server lets go of its seat, the sandbox flags are the
// recipe's before every tick, the loop's frozen frame leaves the launcher to the tick (host/loop.ts). A debugger row that pokes the game still would: the next
// state word says so.
// HANDS OFF: under a card that freezes the page (the pause card) and when the tab is hidden the page sends one record that holds nothing and carries the game's own
// word for it (wire.ts restOf: 'Paused'), so the server does not walk the man on with his last keys or send a held round; the confirmed ticks are still walked under a card (the game is live: it is not this player's to stop).
// THE SEAT'S KEY comes on the address once and is kept for the tab (sessionStorage); it is taken off the address before anything else is asked, so nothing
// that reads the address later (a visit's record, a replay's head) reads the key — refused or not.
// Nothing here is read by the game. The page without ?net never calls into this file past `netFromUrl`'s first line.
import type { Game } from '../game/game';
import type { FollowCamera } from '../game/camera';
import type { Input } from '../game/input';
import type { View } from '../host/hud';
import type { PlayerCmd } from '../game/cmd';
import { sampleCmd } from '../game/cmd';                    // (game exports are called inside functions only: src/game is one import cycle)
import { activeScenario } from '../game/scenarioRun';
import { rawBits, RAW_BITS_SEED } from '../game/digest';
import type { Stand } from '../host/ahead';
import { RecipeStand } from '../replay/recipestand';
import { recipeSearch, unseenDone } from '../replay/engineplay';
import { NetClient } from './client';
import type { Recipe } from './wire';
import { NET, recipeWhy } from './wire';
import { FORESEE, Foresee } from './foresee';
import { delayed } from './delay';
import { netStep, netSeats, type StepHost } from './step';

/** the address's words: the server's WebSocket address, the game's id, the seat's key (read once, then kept for the tab under KEEP) */
export const NETPLAY = 'net', NET_GAME = 'netgame', NET_KEY = 'netkey';
/** …and three for whoever tries it on one machine: `netpredict=0` draws his own man where the game has him (foresee.ts FORESEE.on off), `netlag=<ms>` and
 *  `netjitter=<ms>` put that long a way between this page and the server, each way (NETPAGE.lagMs, jitterMs) */
/** …and one the lobby put there (lobby.ts JOIN: the code of the co-op game this is, so the page in the game can show it and a reload finds the game) */
export const NET_JOIN = 'join';
export const NET_PREDICT = 'netpredict', NET_LAG = 'netlag', NET_JITTER = 'netjitter';
const KEEP = 'tps.net.seat';
/** `helloMs`: how long the page waits for the server's welcome. `walkMs`: past it a page tick that is level does not walk its second tick (one is always
 *  walked, two at most: see HOW FAST).
 *  `far`: confirmed ticks waiting from which the page catches up as a seek does, `farMs` a page tick at it. `noteMs`: how long a passing line stays.
 *  `lagMs`, `jitterMs`: a way of that length (and up to that much more, message by message) each way, for a try on one machine. */
export const NETPAGE = { helloMs: 10000, walkMs: 8, far: 120, farMs: 12, noteMs: 3000, lagMs: 0, jitterMs: 0 };

/** what the page hands over once it stands (main.ts): the game, and what a tick's end and a catch-up's tidy touch */
export interface NetHost {
  game: Game;
  engine: StepHost & { settings: Record<string, unknown>; particles?: { clear(): void } | null; resetTaa?(why?: string): void };
  smoke: { update(dt: number): void; emitters: { kind: string }[] } | null;
  audio: { enabled: boolean; setVolumes(): void };
  loop: { tween?: { stale: boolean; rest(): void } | null };
  /** the picture's half of the prediction (host/ahead.ts AheadDraw): told, each page tick, where the game has the player's man and where his records take him */
  ahead?: { capture(id: number, from: Stand, to: Stand): void; rest(): void; last?: unknown; readonly draws: number } | null;
}
export interface NetPageState { seat: number; state: string; confirmed: number; heard: number; behind: number; far: boolean; wordsOk: number; bad: { t: number; server: string; mine: string } | null; word: string;
  sent: number; holds: number; skips: number; margin: number; lead: number; pending: number; broken: string; bye: string; refuse: string; outcome: unknown; walkMs: number;
  /** his own man ahead of the game (null: drawn where the game has him): records played, where they take him and how far that is from where the game has him
   *  (m), what is left of a correction (m), corrections so far past a centimetre and the longest (m), and what a look ahead costs (ms, eased) */
  ahead: { n: number; pos: number[]; move: number; off: number; fixes: number; fixMax: number; ms: number } | null; }

/** main()'s question, asked where it asks a replay's. null = no networked game asked for; 'moved' = the page is on its way to the game's own address (stop
 *  booting); else the client, or a refusal the page shows (`refuse` set: the game boots as itself behind the words). */
export async function netFromUrl(q: URLSearchParams): Promise<NetPlay | 'moved' | null> {
  const url = q.get(NETPLAY); if (url === null) return null;
  const game = q.get(NET_GAME) ?? 'local', seat = seatKey(q, url, game);
  const tries: Record<string, string> = {}; for (const k of [NET_PREDICT, NET_LAG, NET_JITTER]) { const v = q.get(k); if (v !== null) tries[k] = v; }
  FORESEE.on = !/^(0|off|false|no)$/i.test(tries[NET_PREDICT] ?? ''); NETPAGE.lagMs = ms(tries[NET_LAG]); NETPAGE.jitterMs = ms(tries[NET_JITTER]);
  const P = new NetPlay(url, game, seat.key === null ? undefined : seat.key);
  await P.open();
  if (P.refuse || !P.recipe) { if (q.has(NET_KEY)) keyOff(); return P; }   // (a refused page boots as itself: the key it came with is not its address's to keep)
  const want = recipeSearch(P.recipe, q, { [NETPLAY]: url, [NET_GAME]: game, ...(seat.kept || seat.key === null ? {} : { [NET_KEY]: seat.key }), ...(q.has(NET_JOIN) ? { [NET_JOIN]: q.get(NET_JOIN) ?? '' } : {}), ...tries });
  if (new URLSearchParams(location.search).toString() !== new URLSearchParams(want).toString()) { P.close(); location.replace(location.pathname + want + location.hash); return 'moved'; }
  return P;
}
/** the address without the game's words: where a player who leaves a networked game goes (main.ts, the cards' Quit) */
export function netLeave(search: string): string { const u = new URLSearchParams(search); for (const k of [NETPLAY, NET_GAME, NET_KEY, NET_JOIN, NET_PREDICT, NET_LAG, NET_JITTER]) u.delete(k); const s = u.toString(); return s ? '?' + s : ''; }
const ms = (v: string | undefined): number => { const n = Number(v); return n > 0 && n <= 2000 ? n : 0; };
/** how much longer this page's way to its server is, each way (ms: NETPAGE.lagMs and up to jitterMs more — delay.ts; neither = no longer) */
const way = (): number => NETPAGE.lagMs + Math.random() * NETPAGE.jitterMs;
/** does this tab hold a seat's key for the game its address names: on the address, or put away for the tab? (A game's address with no key is a watcher's:
 *  one that also names a co-op game's code goes through the lobby's door first — net/lobby.ts, main.ts) */
export function netKeyed(q: URLSearchParams): boolean {
  if (q.has(NET_KEY)) return true;
  try { const k = JSON.parse(sessionStorage.getItem(KEEP) ?? 'null') as unknown; return Array.isArray(k) && k[0] === q.get(NETPLAY) && k[1] === (q.get(NET_GAME) ?? 'local') && typeof k[2] === 'string'; } catch { return false; }
}
function keyOff(): void { try { const u = new URLSearchParams(location.search); u.delete(NET_KEY); history.replaceState(null, '', location.pathname + '?' + u.toString() + location.hash); } catch { /* a sandboxed frame */ } }
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

/** the build this page is, as the dev server and a hosted build both say it ('' = it has no name: uncommitted changes, a static copy) */
export async function buildName(): Promise<string> {
  try { const r = await fetch('/qa/commit', { cache: 'no-store', signal: AbortSignal.timeout(3000) }); const c = r.ok ? (await r.text()).trim() : ''; return /^[0-9a-f]{7,40}$/.test(c) ? c : ''; } catch { return ''; }
}

export class NetPlay {
  /** why this page is not a client of the game ('' = it is) */
  refuse = '';
  readonly client: NetClient;
  private ws: WebSocket | null = null; private rs: RecipeStand | null = null; private host: NetHost | null = null;
  private word: number = RAW_BITS_SEED; private first = true; private walkMs = 0; private line: HTMLElement | null = null; private said = '';
  /** catching up (NETPAGE.far): the mixer's own switch as it was, null = not catching up */
  private audioWas: boolean | null = null;
  private note = ''; private noteUntil = 0;
  /** inside a confirmed tick */
  private stepping = false;
  private cmd: PlayerCmd | null = null;
  /** refused once the page stood (attach): the game it plays on as, by the loop's own tick */
  private alone: NetHost | null = null;
  /** the host's draws (AheadDraw.draws) when the world was last packed for a tick */
  private drawsAt = 0;

  constructor(readonly url: string, readonly game: string, readonly key: string | undefined) {
    this.client = new NetClient(delayed<string>(text => { const ws = this.ws; if (ws && ws.readyState === 1) ws.send(text); }, way), { game, build: '', ...(key !== undefined ? { key } : {}) });
  }
  get recipe(): Recipe | null { return this.client.welcome?.recipe ?? null; }

  /** connect, say hello, wait for the welcome (or for why there is none) */
  async open(): Promise<void> {
    const c = this.client, named = buildName();   // (asked for while the socket opens)
    await new Promise<void>(done => {
      let ws: WebSocket; try { ws = this.ws = new WebSocket(this.url); } catch { this.refuse = 'This page can’t connect to that game’s address.'; done(); return; }
      let met = false; const settle = (why: string) => { if (met) return; met = true; clearTimeout(t); if (why) this.refuse ||= why; done(); };
      const t = setTimeout(() => settle('The game’s server didn’t answer.'), NETPAGE.helloMs);
      ws.onopen = () => void named.then(b => { c.who.build = b; c.hello(); });
      const down = delayed<() => void>(f => f(), way);   // (what comes down the socket, its close too, in the order it came)
      ws.onmessage = ev => down(() => { c.message(String(ev.data), performance.now()); if (!met && (c.welcome || c.bye)) settle(c.welcome ? '' : refusal(c.byeCode)); });
      ws.onclose = () => down(() => { if (this.ws !== ws) return; if (!met) settle(c.bye ? refusal(c.byeCode) : 'This page couldn’t reach the game’s server.'); else if (!c.bye) c.broken ||= 'the connection closed'; });
      ws.onerror = () => { /* its close follows */ };
    });
    if (!this.refuse) { if (!this.recipe || recipeWhy(this.recipe)) this.refuse = CANT_PLAY; else this.rs = new RecipeStand(this.recipe); }
    if (this.refuse) { this.close(); this.say(this.refuse); }
  }
  close(): void { const ws = this.ws; this.ws = null; if (ws) { ws.onclose = null; try { ws.close(); } catch { /* gone */ } } }

  // ---- standing the game as the server's recipe says (main.ts asks where it asks a replay's tape)
  tables(): void { this.rs?.tables(); }
  env(settings: Record<string, unknown>): void { this.rs?.env(settings); }
  stood(g: Game): void { this.rs?.stood(g); }

  /** the page stands. Held against the recipe first (a page that booted another mission is no client of this one); then every seat is refereed, a card's
   *  Restart is refused, and the server is told this seat can play */
  attach(h: NetHost): void {
    const g = h.game, c = this.client;
    if (activeScenario(g) !== this.recipe?.scenario || g.seats.length !== c.welcome?.seats) { this.alone = h; this.refuse = CANT_PLAY; this.close(); this.say(this.refuse); return; }   // (…and plays on as its own game under the words: tick below)
    this.host = h; netSeats(g);
    if (c.seat >= 0) g.localSeat = g.seats[c.seat];   // (the seat this page plays is the one it shows — game/seats.ts: its man under the camera, its HUD, its record sampled; a watcher keeps the first. The simulation never reads it)
    const restart = g.restart.bind(g);   // (the player's restart: inside a tick — a key in a record — it is every machine's; from OUTSIDE one — a card's button — it is this page's alone, so not this page's to do)
    g.restart = said => { if (this.stepping) restart(said); else this.pass('You can’t restart a co-op game from here.'); };
    const load = g.saves.load.bind(g.saves);   // (…and a checkpoint's load, the same: the words are the card's own 'why it would not load')
    g.saves.load = (...a: Parameters<typeof load>) => this.stepping ? load(...a) : 'You can’t load a checkpoint in a co-op game.';
    if (c.seat >= 0) c.ready();
    document.addEventListener('visibilitychange', () => { if (document.hidden) c.rest(performance.now()); });
    const me = this; (window as unknown as { __net: unknown }).__net = { get state() { return me.state(); }, client: c, ahead: h.ahead ?? null };
  }

  /** ONE TICK OF THE PAGE (the loop's `tick`): the player's record sent, the confirmed ticks walked, the host's half of a tick round them (Game.update's) */
  tick(_dt: number, input: Input, cam: FollowCamera, view: View): void {
    const h = this.host; if (!h) { const a = this.alone; if (a) { a.game.update(_dt, input, cam, view); a.engine.lights.update(_dt, a.game.time); } return; }   // (refused once stood: the loop's own tick)
    const g = h.game, c = this.client, now = performance.now();
    g.hostBefore(input);
    if (c.behind >= NETPAGE.far) this.catching(true);   // (BEFORE the record: a page that wakes far behind sends no key made at a picture seconds old)
    if (c.seat >= 0 && c.state !== 'waiting') { if (this.audioWas === null) c.beat(this.cmd = sampleCmd(g.hostSeat().view, input, cam, view, this.cmd), now); else c.rest(now); }
    this.walk(); this.foresee();
    g.hostAfter(input, cam, view);
  }
  /** a frame the page is frozen on (a card is up): the hands come off, the game goes on */
  rest(input: Input, cam: FollowCamera, view: View): void { const h = this.host; if (!h) return; this.client.rest(performance.now()); this.walk(); this.foresee(); h.game.hostAfter(input, cam, view); }

  /** the look ahead, at the end of a page tick (foresee.ts): the picture is told where the game has his man and where his records take him (host/ahead.ts) */
  private foresee(): void { const h = this.host!, F = this.fore.look(h.game, this.client, this.audioWas !== null); if (F) h.ahead?.capture(F.id, F.from, F.to); else h.ahead?.rest(); }
  private fore = new Foresee();
  /** how far ahead of where the game has him the player's man is drawn, and where his newest record aims (the loop's `lead`: host/loop.ts); null = he is drawn where the game has him */
  lead(): { by: number[]; aim: [number, number, number] | null } | null { return this.fore.last; }

  private walk(): void {
    const h = this.host!, g = h.game, c = this.client, t0 = performance.now(); let n = 0;
    if (c.behind >= NETPAGE.far) this.catching(true);
    const far = this.audioWas !== null, most = far ? Infinity : c.behind > NET.lead ? 2 : 1, ms = far ? NETPAGE.farMs : NETPAGE.walkMs;
    const step = (cmds: PlayerCmd[]) => {
      if (this.first) { this.first = false; this.rs?.tables(); g.fogOfWar.pageLook(); this.rs?.tune(g); }   // (the last word over what the boot wrote after the staging, as a replay's first step has it)
      this.rs?.env(h.engine.settings); this.rs?.flags(g);
      { const d = h.ahead?.draws ?? 0; if (d !== this.drawsAt) { this.drawsAt = d; h.engine.world.relist(); } }   // (THE LIST A TICK READS IS THE LAST TICK'S, as on the server: a draw since then listed the boxes as it DREW them — between two ticks, his own man ahead — and the world's ray reads that list. So the first tick after a draw lists them again as the game has them)
      this.stepping = true; try { netStep(g, h.engine, cmds); } finally { this.stepping = false; }
      this.word = rawBits(g, this.word);
      if (far) { h.smoke?.update(1 / NET.hz); const tw = h.loop.tween; if (tw && !tw.stale) tw.rest(); }   // (unseen ticks: the smoke keeps the game's time, and nothing is drawn between two of them — as under a replay's seek)
    };
    while (c.behind && n < most && (n < 1 || performance.now() - t0 < ms)) n += c.walk({ step, word: () => this.word.toString(36) }, 1);
    if (n) this.walkMs = (performance.now() - t0) / n;
    if (far && c.behind <= NET.lead) this.catching(false);
    if ((c.bad || c.broken) && this.ws) { if (far) this.catching(false); this.close(); }   // (parted from the server: the seat is let go — the server holds no key for a seat nobody sits in — and the page stands where it is)
    this.status();
  }
  /** catching up begins / is over (a replay's seek: replay/engineplay.ts) — the mixer shut over ticks nobody sees, and its tidy when the page is level */
  private catching(on: boolean): void {
    const h = this.host!;
    if (on) { if (this.audioWas === null) { this.audioWas = h.audio.enabled; h.audio.enabled = false; h.audio.setVolumes(); } return; }
    if (this.audioWas === null) return;
    unseenDone(h, this.audioWas, 'net'); this.audioWas = null;
  }

  state(): NetPageState {
    const c = this.client, F = this.fore;
    return { seat: c.seat, state: c.state, confirmed: c.confirmed, heard: c.heard, behind: c.behind, far: this.audioWas !== null, wordsOk: c.wordsOk, bad: c.bad, word: this.word.toString(36), sent: c.sent, holds: c.holds, skips: c.skips,
      margin: c.margin, lead: c.lead, pending: c.pending.length, broken: c.broken, bye: c.bye, refuse: this.refuse, outcome: c.outcome, walkMs: this.walkMs,
      ahead: F.last ? { n: F.last.n, pos: Array.from(F.last.to.pos), move: Math.hypot(F.last.by[0], F.last.by[1], F.last.by[2]), off: F.rest, fixes: F.fixes, fixMax: F.fixMax, ms: F.ms } : null };
  }

  // ---- the one line the player is shown while something is not as it should be
  private status(): void {
    const c = this.client;
    this.say(c.bad ? 'Your game went out of step with the server. Reload the page to join again.'
      : c.bye ? (c.outcome ? 'The mission is over.' : 'The game is over.')
      : c.broken ? 'You lost the connection to the game. Reload the page to join again.'
      : c.state === 'waiting' ? 'The other player isn’t here yet.'
      : this.audioWas !== null ? 'You’re catching up with the game.'
      : performance.now() < this.noteUntil ? this.note : '');
  }
  /** a line that passes (NETPAGE.noteMs) */
  private pass(text: string): void { this.note = text; this.noteUntil = performance.now() + NETPAGE.noteMs; this.status(); }
  private say(text: string): void {
    if (text === this.said || typeof document === 'undefined') return; this.said = text;
    if (!this.line) { const d = this.line = document.createElement('div'); d.id = 'netline'; d.style.cssText = 'position:fixed;left:50%;top:14%;transform:translateX(-50%);z-index:40;padding:8px 16px;border-radius:6px;background:rgba(10,12,14,.82);color:#e8e6df;font:500 15px/1.3 system-ui,sans-serif;pointer-events:none;max-width:80vw;text-align:center'; document.body.appendChild(d); }
    this.line.textContent = text; this.line.style.display = text ? 'block' : 'none';
  }
}

const CANT_PLAY = 'This page can’t play that game.';
/** a server's goodbye before any welcome (wire.ts ByeCode), as the player is told it */
function refusal(code: string): string {
  return code === 'build' ? 'That game is on another version. Reload the page and join again.'
    : code === 'seat' ? 'That seat isn’t yours.'
    : code === 'full' ? 'There’s no room left to watch that game.'
    : 'You can’t join that game.';
}
