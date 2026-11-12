// A SEAT'S LINE ON THE SERVER: the records a seat has sent for ticks that have not run yet, and the one rule that decides what each tick runs on. The server's
// authority over input is this file (src/net/server.ts asks `take` once a seat a tick, in seat order, and what it answers is what every machine plays):
//   - the seat's own record for that tick, if it came in time;
//   - else the newest record the seat is known by, with its one-tick edges off (the keys and buttons stay held, the view stays where it looked);
//   - a record that came LATE (for a tick already run) is not lost: it becomes the newest known, and its edges — a key gone down, a click, the wheel — land on
//     the next tick that runs, with the keys held under them;
//   - a seat nobody is sitting in holds nothing.
// Pure data and arithmetic: no clock, no socket, nothing of the game.
import type { Edges, WireCmd } from './wire';
import { FIRST, NO_EDGES, anyEdge, edgesOf, heldOf, idleOf, withEdges, NET } from './wire';

export class SeatLine {
  /** records for ticks still to run, by tick */
  private ahead = new Map<number, WireCmd>();
  /** the newest record the seat is known by, and the tick it was for */
  private known: WireCmd = FIRST; private knownT = -1;
  /** edges of late records, owed to the next tick that runs */
  private owed: Edges = NO_EDGES;
  /** records taken in · of them, late ones · ticks that ran on the seat's own record for that tick · ticks that ran on a held one */
  got = 0; late = 0; own = 0; held = 0;

  /** a record for tick `t` has come; `next` = the next tick the server will run. A second record for one tick is not taken (a client never sends a tick twice) */
  put(t: number, c: WireCmd, next: number): void {
    if (!(t >= 0) || t > next + NET.horizon) return;
    this.got++;
    if (t >= next) { if (!this.ahead.has(t)) this.ahead.set(t, c); return; }
    this.late++;
    this.owed = edgesOf(this.owed, c);
    if (t > this.knownT) { this.known = c; this.knownT = t; }
  }
  /** the record tick `t` runs on for this seat (see the head). `sat` = somebody is connected to the seat */
  take(t: number, sat: boolean): WireCmd {
    const mine = this.ahead.get(t);
    let c: WireCmd;
    if (mine) { this.ahead.delete(t); this.own++; c = mine; this.known = mine; this.knownT = t; }
    else { this.held++; c = heldOf(this.known); }
    if (!sat) c = idleOf(c);
    else if (anyEdge(this.owed)) c = withEdges(c, this.owed);
    this.owed = NO_EDGES;
    return c;
  }
  /** how many records are held for ticks after `t` (what the pace note tells the client) */
  depth(t: number): number { let n = 0; for (const k of this.ahead.keys()) if (k > t) n++; return n; }
  /** a new sitter: what the last one left held is let go */
  reset(): void { this.ahead.clear(); this.known = idleOf(this.known); this.owed = NO_EDGES; }
}
