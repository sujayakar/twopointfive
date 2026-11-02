// THE GAME SERVER'S CORE: one game, stepped with authority (see wire.ts for what travels and why). Host-neutral — the process that owns the sockets, the clock
// and the stood game hands them in (tools/mp/server.ts under bun; a check hands in fakes) — so nothing here knows of bun, of a WebSocket or of where it runs.
//   STATES: `waiting` until every seat has somebody in it whose client has stood its game and said `ready` (no tick runs: the fight starts when both players
//   can play it) → `playing` (a tick every 1 / hz of the host's clock, each seat's record decided by its line, timeline.ts) → `ended` when the game's own
//   verdict says so (the game runs on for a grace, then the server is done). A seat whose player drops holds nothing until he is back. A WATCHER (a hello with
//   no key) is sent every tick and sends none; NET.watchers of them at most.
//   THE RUN SO FAR is kept as the text of its tape messages, NET.tapeChunk ticks each, and every connection has a place in it (`at`): one that is up to date is
//   sent each tick as it runs; one that is behind — a joiner, a seat come back, a socket that would not take a tick — is sent a stretch of the run on each call
//   of `advance`, and none while its socket says it is backed up (`drained` lets it go on). So a hello costs nothing however long the game has run, and no tick
//   is ever lost on the way to a slow reader.
//   WHAT A CLIENT SENDS IS CHECKED before it is kept: a message's length, a record's every field (wire.ts wireOk). A record that is not one is not taken.
//   IT ENDS ITSELF (`done`): nobody connected for `idleS`, the mission over and the grace run, or `maxS` since it was made. The host exits on it.
//   BEHIND: a host that cannot keep the rate runs at most NET.catchUp ticks in one go, and past NET.slipAt behind lets the game's clock slip rather than run a
//   spiral (`slips` counts; a client follows the slip by the ticks it hears, client.ts).
import type { PlayerCmd } from '../game/cmd';
import type { Recipe, NetState, Outcome, WireCmd, Hello, ToServer, ToClient, Tick, State } from './wire';
import { NET, netText, netParse, fromWire, heldOf, wireOk, withEdges } from './wire';
import { SeatLine } from './timeline';

/** what the host gives: its clock (ms, never going back), one tick of its game on the seats' records (and whatever it does between two ticks), the game's
 *  chained state word after the last tick, and how the mission ended (null while it has not) */
export interface ServerHost { now(): number; step(cmds: PlayerCmd[]): void; word(): string; ended(): Outcome | null; }
/** a connection as the core sees it. `send` answers what became of the text: above 0 it went, below 0 it was taken but the socket is backed up (the host says
 *  `drained` when it no longer is), 0 it was NOT taken */
export interface Conn { send(text: string): number; close(why: string): void; }
export interface ServerOpts { game: string; build: string; recipe: Recipe; seatKeys: string[]; idleS?: number; maxS?: number; graceS?: number; }
export interface Health { game: string; build: string; state: NetState; tick: number; seats: boolean[]; watchers: number; slips: number; lost: number; outcome?: Outcome; done?: string; }

interface Sitter { line: SeatLine; conn: Conn | null; ready: boolean; prev: WireCmd | null; }
/** a connection that has said hello: its seat (−1 a watcher), the next tick it is to be sent, whether its socket is backed up (and since when), whether it has
 *  said its game parted from the server's */
interface Peer { seat: number; at: number; slow: boolean; slowAt: number; lost: boolean; }
type Row = { c: (WireCmd | 0)[]; w?: string };

export class NetServer {
  state: NetState = 'waiting'; tick = 0;
  /** why the server is finished ('' = it is not): the host exits */
  done = '';
  /** ticks the clock was let slip because the host fell behind · seated clients that said their state word differed */
  slips = 0; lost = 0;
  outcome: Outcome | null = null;
  /** called on every change a control plane would want to hear of at once (a state, a seat filled or left, a client whose game parted, the end) */
  onChange: (() => void) | null = null;
  private seats: Sitter[]; private peers = new Map<Conn, Peer>();
  /** the run so far: the text of every full stretch's tape message, and the rows of the stretch being run */
  private full: string[] = []; private cur: Row[] = [];
  private born: number; private t0 = 0; private emptySince: number; private endedAt = 0;
  private idleMs: number; private maxMs: number; private graceMs: number;

  constructor(private host: ServerHost, private opts: ServerOpts) {
    this.seats = opts.seatKeys.map(() => ({ line: new SeatLine(), conn: null, ready: false, prev: null }));
    this.born = this.emptySince = host.now();
    this.idleMs = (opts.idleS ?? 60) * 1000; this.maxMs = (opts.maxS ?? 3600) * 1000; this.graceMs = (opts.graceS ?? 15) * 1000;
  }

  private get watching(): number { let n = 0; for (const p of this.peers.values()) if (p.seat < 0) n++; return n; }
  health(): Health { return { game: this.opts.game, build: this.opts.build, state: this.state, tick: this.tick, seats: this.seats.map(s => !!s.conn), watchers: this.watching, slips: this.slips, lost: this.lost, ...(this.outcome ? { outcome: this.outcome } : {}), ...(this.done ? { done: this.done } : {}) }; }
  /** how each seat's records have come: taken in, late, ticks on its own record, ticks on a held one */
  lines(): { got: number; late: number; own: number; held: number }[] { return this.seats.map(s => ({ got: s.line.got, late: s.line.late, own: s.line.own, held: s.line.held })); }

  // ---- connections
  message(conn: Conn, text: string): void {
    if (text.length > NET.maxText) return;
    let m: ToServer; try { m = netParse(text) as ToServer; } catch { return; }
    if (!m || typeof m !== 'object') return;
    const P = this.peers.get(conn);
    if (!P) { if (m.k === 'hello') this.hello(conn, m); else this.bye(conn, 'say hello first'); return; }
    if (P.seat < 0) return;   // (a watcher sends nothing the game hears)
    const S = this.seats[P.seat];
    if (m.k === 'in') { if (this.state !== 'waiting' && Number.isInteger(m.t) && wireOk(m.c)) S.line.put(m.t, withEdges(m.c, [], 0, 0), this.tick); }
    else if (m.k === 'ready') { if (!S.ready) { S.ready = true; this.begin(); } }
    else if (m.k === 'lost') { if (!P.lost) { P.lost = true; this.lost++; this.onChange?.(); } }
  }
  closed(conn: Conn): void {
    const P = this.peers.get(conn); this.peers.delete(conn);
    if (!P) return;
    if (P.seat >= 0 && this.seats[P.seat].conn === conn) this.leave(this.seats[P.seat]);
    if (!this.peers.size) this.emptySince = this.host.now();
    this.said();
  }
  /** the host's socket for `conn` is no longer backed up */
  drained(conn: Conn): void { const P = this.peers.get(conn); if (P) P.slow = false; }
  private leave(S: Sitter): void { S.conn = null; S.ready = false; S.line.reset(); }
  private hello(conn: Conn, m: Hello): void {
    const O = this.opts;
    if (m.proto !== NET.proto) return this.bye(conn, `this server speaks wire ${NET.proto}, you speak ${m.proto}`);
    if (m.game !== O.game) return this.bye(conn, 'this server runs another game');
    if (O.build && m.build !== O.build) return this.bye(conn, `this game is on build ${O.build}, you are on ${typeof m.build === 'string' && m.build ? m.build.slice(0, 40) : 'one with no name'}`);
    let seat = -1;
    if (m.key !== undefined) { seat = O.seatKeys.indexOf(m.key); if (seat < 0) return this.bye(conn, 'no seat has that key'); }
    else if (this.watching >= NET.watchers) return this.bye(conn, 'no room to watch');
    if (seat >= 0) { const S = this.seats[seat]; if (S.conn) { const old = S.conn; this.peers.delete(old); this.leave(S); old.close('the seat was taken again'); } S.conn = conn; }
    this.peers.set(conn, { seat, at: 0, slow: false, slowAt: 0, lost: false });
    this.send(conn, { k: 'welcome', proto: NET.proto, game: O.game, build: O.build, seat, seats: this.seats.length, hz: NET.hz, recipe: O.recipe, tick: this.tick, state: this.state });
    this.said();
  }
  /** the fight starts once every seat is sat in by a client that has stood its game */
  private begin(): void { if (this.state === 'waiting' && this.seats.every(s => s.conn && s.ready)) { this.state = 'playing'; this.t0 = this.host.now(); this.said(); } }
  private bye(conn: Conn, why: string): void { this.send(conn, { k: 'bye', why }); conn.close(why); }
  private put(conn: Conn, text: string): number { try { return conn.send(text); } catch { return 0; /* a socket that died under the send: its close follows */ } }
  private send(conn: Conn, m: ToClient): void { this.put(conn, netText(m)); }
  private all(text: string): void { for (const c of this.peers.keys()) this.put(c, text); }
  private said(): void { const m: State = { k: 'state', state: this.state, tick: this.tick, seats: this.seats.map(s => !!s.conn), ...(this.outcome ? { outcome: this.outcome } : {}) }; this.all(netText(m)); this.onChange?.(); }

  // ---- the clock
  /** run what is due. The host calls it often (every few ms); it returns at once when nothing is */
  advance(): void {
    if (this.done) return;
    const now = this.host.now();
    if (now - this.born > this.maxMs) return this.finish('its longest life was up');
    if (!this.peers.size && now - this.emptySince > this.idleMs) return this.finish('nobody was connected');
    if (this.state === 'ended' && now - this.endedAt > this.graceMs) return this.finish('the game ended');
    if (this.state === 'waiting') return;
    this.catchUp(now);
    let due = Math.floor((now - this.t0) * NET.hz / 1000) - this.tick;
    if (due > NET.slipAt) { const drop = due - NET.catchUp; this.t0 += drop * 1000 / NET.hz; this.slips += drop; }
    due = Math.min(due, NET.catchUp);
    for (let i = 0; i < due; i++) this.runTick();
  }
  /** every connection that is behind the run is sent its next stretch (the whole stretch its tick is in: a reader passes over what it has) */
  private catchUp(now: number): void {
    for (const [conn, P] of this.peers) {
      if (P.at >= this.tick) continue;
      if (P.slow) { if (now - P.slowAt < NET.retryMs) continue; P.slow = false; }
      const i = Math.floor(P.at / NET.tapeChunk), whole = i < this.full.length;
      const r = this.put(conn, whole ? this.full[i] : netText({ k: 'tape', from: i * NET.tapeChunk, ticks: this.cur }));
      if (r !== 0) P.at = whole ? (i + 1) * NET.tapeChunk : this.tick;
      if (r <= 0) { P.slow = true; P.slowAt = now; }
    }
  }
  private runTick(): void {
    const t = this.tick, wire: WireCmd[] = [], sent: (WireCmd | 0)[] = [];
    for (const S of this.seats) { const c = S.line.take(t, !!S.conn); wire.push(c); sent.push(S.prev && netText(c) === netText(heldOf(S.prev)) ? 0 : c); S.prev = c; }
    this.host.step(wire.map(fromWire));
    this.tick = t + 1;
    const row: Row = { c: sent };
    if (this.tick % NET.wordEvery === 0) row.w = this.host.word();
    this.cur.push(row);
    const m: Tick = { k: 't', t, ...row }, text = netText(m);
    for (const [conn, P] of this.peers) if (P.at === t && this.put(conn, text) !== 0) P.at = t + 1;   // (one that would not take it is behind now: catchUp sends it the stretch)
    if (this.cur.length === NET.tapeChunk) { this.full.push(netText({ k: 'tape', from: this.tick - NET.tapeChunk, ticks: this.cur })); this.cur = []; }
    if (this.tick % NET.paceEvery === 0) for (const S of this.seats) if (S.conn) this.send(S.conn, { k: 'pace', t, l: S.line.depth(t) });
    if (this.state === 'playing') { const e = this.host.ended(); if (e) { this.state = 'ended'; this.outcome = e; this.endedAt = this.host.now(); this.said(); } }
  }
  /** the host ends the game from outside (its control plane no longer has it) */
  stop(why: string): void { if (!this.done) this.finish(why); }
  private finish(why: string): void { this.done = why; this.all(netText({ k: 'bye', why })); for (const c of [...this.peers.keys()]) c.close(why); this.peers.clear(); for (const S of this.seats) this.leave(S); this.onChange?.(); }
}
