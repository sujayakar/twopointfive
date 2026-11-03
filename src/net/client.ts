// A CLIENT OF A GAME SERVER, its core (see wire.ts). Host-neutral: the page or a headless bot owns the socket, the stood game and the beat, and hands them in.
//   THE WORLD RUNS ON CONFIRMED TICKS ONLY: every tick the server says ran is queued and `walk` steps the host's game through them, in order, on the records the
//   server decided. That game is the server's to the bit; the state word that rides every NET.wordEvery ticks is held against the host's own, and the first
//   one that differs is kept (`bad`) and told to the server.
//   THE SEAT'S OWN RECORDS GO AHEAD OF THE SERVER: once a beat the host hands in the record its player made, and it is sent for a tick a little in the server's
//   future so that it is there when that tick runs. WHICH TICK is worked out from time, not counted: the client keeps the server's tick against its own clock
//   (the least-delayed tick message of the last two NET.clockMs says it best: `ahead` — so a server that let its clock slip, or a way that got longer, is
//   followed), and a record goes to that tick plus `margin`. So a host whose beats come late or not at all (a hitch, a tab asleep) skips ticks — the server
//   holds his keys through them — and is never left sending into the past. The server's pace notes say how many of his records were waiting when a tick ran;
//   `margin` grows when that is under NET.lead (the way up got longer) and shrinks when it is over NET.leadMax. A note speaks of the way up only when records
//   were on it: one about a tick this client had sent nothing past (it was standing its game, its tab slept, its beat hitched) is not acted on. Beats that come
//   in a bunch (two ticks of one drawn frame) go to the ticks that follow, up to NET.burst ahead; past that a beat is held, its edges riding the next record.
//   What was sent and is not confirmed yet is kept (`pending`): the records a prediction of the seat's own man plays ahead of the confirmed world.
import type { PlayerCmd } from '../game/cmd';
import type { Hello, ToClient, Welcome, WireCmd, NetState, Outcome } from './wire';
import { NET, netText, netParse, fromWire, toWire, heldOf, restOf, withEdges } from './wire';

/** the host's game as the client drives it: one confirmed tick on every seat's record, and the chained state word after the last */
export interface ClientHost { step(cmds: PlayerCmd[]): void; word(): string; }

export class NetClient {
  welcome: Welcome | null = null;
  /** the seat this client sits in (−1: a watcher, or not welcomed yet) */
  seat = -1;
  state: NetState = 'waiting'; seats: boolean[] = []; outcome: Outcome | null = null;
  /** why the server let this client go ('' = it has not) */
  bye = ''; byeCode = '';
  /** the stream of ticks has a hole in it ('' = it has none): a socket never loses or reorders a message, so the host should connect again */
  broken = '';
  /** ticks the host's game has been stepped through · ticks the server has said ran (the queue is the difference) */
  confirmed = 0; heard = 0;
  /** state words that matched · the first that did not: the tick it was after, the server's and the host's own */
  wordsOk = 0; bad: { t: number; server: string; mine: string } | null = null;
  /** the next tick a record may be sent for · records sent · beats held (the host beat faster than the server ticks) · ticks skipped (the beat came late) · the last pace note's depth */
  sendTick = 0; sent = 0; holds = 0; skips = 0; lead = NaN;
  /** walked ticks the server ran on a HELD record for this seat (its own came late, or none was sent): what a look ahead could not have known */
  heldRun = 0;
  /** how many ticks past the server's own a record is aimed (the way up and the jitter buffer: the pace notes move it) */
  margin = NET.lead + 1;
  /** the seat's records sent for ticks not confirmed yet, oldest first: [tick, record] */
  pending: [number, WireCmd][] = [];
  /** the server's tick minus this host's clock in ticks, by the least-delayed tick message of this span of NET.clockMs and the one before; NaN before the first */
  private ahead = NaN;
  private span = -1; private aheadNow = NaN; private aheadWas = NaN;
  private queue: { c: (WireCmd | 0)[]; w?: string }[] = [];
  private prev: (WireCmd | null)[] = [];
  /** pace notes about ticks before this one are not acted on (the margin moved, or the sending began again, after they were sent for) */
  private settle = 0;
  private carry: { p: string[]; c: number; w: number } = { p: [], c: 0, w: 0 };

  constructor(private send: (text: string) => void, readonly who: { game: string; build: string; key?: string }) {}

  /** the host has stood its game from the welcome's recipe: a seat's fight does not start before */
  ready(): void { this.send(netText({ k: 'ready' })); }
  hello(): void { const m: Hello = { k: 'hello', proto: NET.proto, build: this.who.build, game: this.who.game, ...(this.who.key !== undefined ? { key: this.who.key } : {}) }; this.send(netText(m)); }

  /** a message from the server; `now` = the host's clock (ms, never going back) as it arrived */
  message(text: string, now: number): void {
    let m: ToClient; try { m = netParse(text) as ToClient; } catch { return; }
    if (!m || typeof m !== 'object') return;
    if (m.k === 'welcome') { this.welcome = m; this.seat = m.seat; this.state = m.state; this.prev = new Array<WireCmd | null>(m.seats).fill(null); }
    else if (m.k === 't') { if (m.t === this.heard) { this.queue.push({ c: m.c, ...(m.w !== undefined ? { w: m.w } : {}) }); this.heard++; this.clock(now); } else if (m.t > this.heard) this.broken ||= `tick ${m.t} came where tick ${this.heard} was due`; }
    else if (m.k === 'tape') { if (m.from > this.heard) this.broken ||= `the run from tick ${m.from} came where tick ${this.heard} was due`; for (let i = 0; i < m.ticks.length; i++) if (m.from + i === this.heard) { this.queue.push(m.ticks[i]); this.heard++; } }
    else if (m.k === 'pace') { this.lead = m.l;
      if (!this.sent || m.t < this.settle || m.t >= this.sendTick) { /* a note about a tick sent for before the margin last moved or the sending last began, or one nothing was sent past: not about the way up */ }
      else if (m.l < NET.lead && this.margin < NET.marginMax) { this.margin = Math.min(NET.marginMax, this.margin + (m.l <= 0 ? NET.lead : 1)); this.settle = this.sendTick; }
      else if (m.l > NET.leadMax && this.margin > 1) { this.margin--; this.settle = this.sendTick; } }
    else if (m.k === 'state') { this.state = m.state; this.seats = m.seats; if (m.outcome) this.outcome = m.outcome; }
    else if (m.k === 'bye') { this.bye = m.why || 'bye'; this.byeCode = typeof m.code === 'string' ? m.code : ''; }
  }
  /** a live tick was heard at `now`: the server had run `heard` ticks no later than that */
  private clock(now: number): void {
    const d = this.heard - now * NET.hz / 1000, span = Math.floor(now / NET.clockMs);
    if (span !== this.span) { this.aheadWas = span === this.span + 1 ? this.aheadNow : NaN; this.aheadNow = d; this.span = span; } else if (d > this.aheadNow) this.aheadNow = d;
    this.ahead = this.aheadWas > this.aheadNow ? this.aheadWas : this.aheadNow;
  }

  /** one beat of the host's own clock (1 / hz) at `now`: `cmd` = the record its player made this beat (null: a watcher, or nothing to send). While the server
   *  waits for the other seat, and before its first live tick is heard, nothing is sent. */
  beat(cmd: PlayerCmd | null, now: number): void { if (cmd) this.out(toWire(cmd), now); }
  /** the host's hands are off (a card is up over its game, its tab is hidden): one record that holds nothing and says so (wire.ts restOf), so the server
   *  does not hold the seat's last keys down for as long as that lasts. Nothing is sent when nothing was held */
  rest(now: number): void { const l = this.last; if (l && (l.k.length || l.b)) this.out(restOf(l), now, true); }
  /** the last record sent */
  private last: WireCmd | null = null;
  /** `must`: a record that is never held back for the beat's pace (the hands coming off: the host may not beat again for as long as that lasts) */
  private out(w: WireCmd, now: number, must = false): void {
    if (this.seat < 0 || this.state === 'waiting' || this.bye || this.broken || this.bad || this.ahead !== this.ahead) return;   // (…nor once the stream is gone or this game has parted from the server's: nothing would hear it, or it would be aimed at a world that is not there)
    if (this.carry.p.length || this.carry.c || this.carry.w) { w = withEdges(w, this.carry.p, this.carry.c, this.carry.w); this.carry = { p: [], c: 0, w: 0 }; }   // (a held beat's edges ride this record)
    const due = Math.floor(now * NET.hz / 1000 + this.ahead) + this.margin, t = Math.max(due, this.sendTick);
    if (t > due + NET.burst && !must) { this.holds++; if (w.p) for (const k of w.p) if (!this.carry.p.includes(k)) this.carry.p.push(k); this.carry.c |= w.c ?? 0; this.carry.w += w.w ?? 0; return; }
    if (this.sent && t > this.sendTick) this.skips += t - this.sendTick;
    if (!this.sent || t > this.sendTick + NET.burst) this.settle = t;   // (the sending begins, or begins again after a gap: notes about ticks before this one are not about these records)
    this.sendTick = t + 1; this.pending.push([t, w]); this.sent++; this.last = w; this.send(netText({ k: 'in', t, c: w }));
  }

  /** step the host's game through confirmed ticks, at most `max` of them; answers how many it stepped */
  walk(host: ClientHost, max: number = NET.catchUp): number {
    let n = 0;
    while (n < max && this.queue.length) {
      const row = this.queue.shift()!, wire = row.c.map((c, i) => c === 0 ? heldOf(this.prev[i]!) : c);
      for (let i = 0; i < wire.length; i++) this.prev[i] = wire[i];
      if (this.seat >= 0 && row.c[this.seat] === 0) this.heldRun++;
      host.step(wire.map(fromWire));
      const t = this.confirmed++; n++;
      while (this.pending.length && this.pending[0][0] <= t) this.pending.shift();
      if (row.w !== undefined) { const mine = host.word(); if (mine === row.w) this.wordsOk++; else if (!this.bad) { this.bad = { t, server: row.w, mine }; this.send(netText({ k: 'lost', t, w: mine })); } }
    }
    return n;
  }
  /** THE SEAT'S OWN RECORDS ITS HOST'S GAME HAS NOT HAD YET, one a tick from tick `t` on (the next tick that game will be stepped through, or the first tick
   *  after it the seat has a record for — before its first record nothing is known of its hands): the server's own word for the ticks it has confirmed and
   *  the host has not walked, then what was sent for the ticks after them. A tick nothing was sent for holds the record before it, as the server plays
   *  such a tick (wire.ts heldOf). What a prediction of the seat's own man plays ahead of the confirmed world; the first `max`. */
  unplayed(max: number): { t: number; cmds: WireCmd[] } {
    const cmds: WireCmd[] = [], P = this.pending, Q = this.queue, s = this.seat; let from = this.confirmed; if (s < 0) return { t: from, cmds };
    let last = this.prev[s] ?? null, i = 0;
    const end = P.length ? P[P.length - 1][0] + 1 : 0;
    for (let t = from; cmds.length < max && (t < this.heard || t < end); t++) {
      const said = t < this.heard ? Q[t - this.confirmed]?.c[s] : undefined;
      while (i < P.length && P[i][0] < t) i++;
      const mine = said !== undefined ? said : i < P.length && P[i][0] === t ? P[i][1] : 0;
      if (mine !== 0) last = mine; else if (last) last = heldOf(last); else { from = t + 1; continue; }
      cmds.push(last);
    }
    return { t: from, cmds };
  }
  /** confirmed ticks waiting to be stepped */
  get behind(): number { return this.queue.length; }
}
