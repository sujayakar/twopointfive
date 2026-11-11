// WHERE A CLIENT'S OWN MAN IS BY THE RECORDS STILL ON THEIR WAY (the look ahead: what "local prediction" is here). Host-neutral, like the client's core: the
// page calls it at the end of each of its ticks (host/drawnet.ts) and so does a headless bot (tools/mp/bot.ts), on the game each has stood.
//   A client's game stands at the last tick the server confirmed (client.ts), so the player's man answers a key only when its tick comes back — the way up,
//   the server's buffer, the way down. But the records the seat has SENT are known here, and what they do to his walk and his aim does not wait on anybody
//   else. So once a tick his walk is worked out through his seat's records the game has not had yet (client.ts unplayed) by the module's `ahead`: a
//   pure working on a body of its own, by the tick's own lines for a man's legs and aim, that writes nothing to the game (no round, no door, no order, no
//   throw: those are the game's to play). What it answers is where the picture draws him (rust/game_draw lead.rs) and how far ahead the camera looks; the WORLD is
//   never ahead, and nothing of this is read by a tick. Where his legs are not his own (he is down, on a wall, his hands at work: `free`) the
//   answer is where the game has him.
//   A CORRECTION: the last look ahead ended on some tick; this one passes through the same tick, and where it has him THERE is the newer word (the server
//   played another record than the one sent, somebody stood in his way, a round moved him). The difference is not shown at once: the drawn man keeps his
//   place and is brought over, FORESEE.ease of the distance left a tick later; past FORESEE.snap it is a cut.
//   NOT AHEAD: a man who is down, a page that is catching up or has parted from its server, a watcher.
import type { NetClient } from './client';
import type { AheadView } from '../view/read';
import type { WireCmd } from './wire';

/** a man's place and the way his gun points */
export interface Stand { pos: ArrayLike<number>; yaw: number }
/** `on`: the switch — off here as every switch is, raised by a networked page (door.ts tryFrom; `?netpredict=0` leaves it off) and by the scripted
 *  client's `--predict`. Off = his own man is drawn where the game has him. `max`: the most records a look ahead plays. `ease`: what
 *  is left, a tick later, of a correction's distance. `snap` (m): a correction longer than this is a cut. */
export const FORESEE = { on: false, max: 30, ease: 0.85, snap: 1.5 };

/** a look ahead: the man, where the game has him (`from`), where he is drawn (`to`: where his records take him, a correction's rest laid on), how far the
 *  look-at of a camera on him moves with that (`by`), the point his newest record aims at, how many records were played and the tick after the last of them.
 *  `free` false: his legs are not his own this tick (on a wall, hands at work, crouched under the tap step, stalking…) — nothing was walked
 *  and `to` is where the game has him, with what is left of a correction */
export interface Foreseen { id: number; from: Stand; to: Stand; by: number[]; aim: [number, number, number] | null; n: number; end: number; pos: number[]; free: boolean }

export class Foresee {
  /** corrections past a centimetre so far, and the longest (m) · what a look ahead costs (ms, eased) · the last one (null: nobody is ahead) */
  fixes = 0; fixMax = 0; ms = 0; last: Foreseen | null = null;
  private off = [0, 0, 0];
  /** what is left of a correction (m) */
  get rest(): number { return Math.hypot(this.off[0], this.off[1], this.off[2]); }

  /** the end of a tick of the host. `walk`: the host's game asked where its seat's man would be. `held`: the host has its hands off the picture (it is
   *  catching up). null = nobody is ahead */
  look(walk: ((cmds: WireCmd[]) => AheadView) | null, c: NetClient, held = false): Foreseen | null {
    const off = this.off, wire = !FORESEE.on || !walk || c.seat < 0 || held || c.state !== 'playing' || c.bad || c.broken || c.bye ? null : c.unplayed(FORESEE.max);
    const t0 = performance.now(), now = wire && walk ? walk(wire.cmds) : null;
    if (!wire || !now || now.down) { off[0] = off[1] = off[2] = 0; return this.last = null; }
    const was = this.last, n = wire.cmds.length, path = now.path;
    const k = was ? was.end - wire.t : -1;   // (how many of these records the last look ahead played too; under 0: the game has passed where that one ended)
    const m = k === 0 ? now.at : k > 0 && k <= n ? path[k - 1] : null;   // (where THIS look has him on the tick the last one ended on)
    if (was && m) { const fix = Math.hypot(was.pos[0] - m[0], was.pos[1] - m[1], was.pos[2] - m[2]);
      for (let i = 0; i < 3; i++) off[i] += was.pos[i] - m[i];
      if (fix > 0.01) { this.fixes++; if (fix > this.fixMax) this.fixMax = fix; } }
    for (let i = 0; i < 3; i++) off[i] *= FORESEE.ease;
    if (this.rest > FORESEE.snap) off[0] = off[1] = off[2] = 0;
    const his = now.follow[0] === now.pos[0] && now.follow[1] === now.pos[1] && now.follow[2] === now.pos[2];   // (the camera's on HIM: not on a man it is watching, not on a lesson's rail. Elsewhere it is led nowhere)
    this.ms += (performance.now() - t0 - this.ms) * (this.ms ? 0.1 : 1);
    return this.last = { id: now.id, from: { pos: now.at, yaw: now.yaw }, to: { pos: [now.pos[0] + off[0], now.pos[1] + off[1], now.pos[2] + off[2]], yaw: now.aimYaw },
      by: his ? [now.pos[0] + off[0] - now.at[0], now.pos[1] + off[1] - now.at[1], now.pos[2] + off[2] - now.at[2]] : [0, 0, 0], aim: his ? now.aim : null, n, end: wire.t + n, free: now.free, pos: now.pos };
  }
}
