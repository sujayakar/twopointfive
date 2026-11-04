// A WAY MADE LONGER, to look at lag on one machine (the page's ?netlag / ?netjitter: page.ts; the scripted client's --lag / --jitter: tools/mp/bot.ts).
// One queue a direction, let go by the clock: what goes in comes out that much later and IN THE ORDER IT WENT IN, as a socket keeps its order — a timer a
// message does not (two timers due the same millisecond fire as the browser likes, and a tick message that overtakes the one before it reads as a hole in
// the stream). A close goes through the same queue as the messages before it, so a goodbye is read before the socket it came on is seen shut.

/** `fn`, called `ms()` later (asked once a call; 0 with nothing waiting = at once), never out of order. `shut`: a time (performance.now's clock) before
 *  which NOTHING is let go, asked each time something could be (a stall: what is on its way waits too, and a stall taken back lets it go within POLL) */
export function delayed<T>(fn: (x: T) => void, ms: () => number, shut: () => number = () => 0): (x: T) => void {
  const POLL = 50, q: { at: number; x: T }[] = []; let last = 0, timer: ReturnType<typeof setTimeout> | null = null;
  const wait = (now: number): number => Math.max(0, Math.min(Math.max(q[0].at, shut()) - now, shut() > now ? POLL : Infinity));
  const drain = (): void => {
    timer = null; const now = performance.now();
    // (a handler that throws takes its own message with it and no other: what is behind it still goes, by this queue's own timer and not by the next message's.
    // One timer a queue: a handler that put a message into its own queue has armed it already)
    try { if (!(shut() > now)) while (q.length && q[0].at <= now) fn(q.shift()!.x); }
    finally { if (q.length && timer === null) timer = setTimeout(drain, wait(performance.now())); }
  };
  return x => {
    const d = ms(), now = performance.now(); if (!(d > 0) && !q.length && !(shut() > now)) { fn(x); return; }
    const at = Math.max(last, now + Math.max(0, d)); last = at; q.push({ at, x });
    if (timer === null) timer = setTimeout(drain, wait(now));
  };
}

// WHAT A WORSE CONNECTION DOES TO THAT WAY (the debugger's Network page: ui/netrows.ts; the page's ?netlag / ?netjitter / ?netloss; the bot's --lag,
// --jitter, --loss, --stall). One table for both directions of one client:
//   lagMs     every message that much later
//   jitterMs  …and up to that much more, message by message
//   lossPct   the share of messages whose packet is lost. The wire is a WebSocket, so a lost packet is not a lost message: it is sent again, and the message
//             AND EVERY MESSAGE BEHIND IT waits for that. The queue above keeps its order, so one long wait holds the rest: that is the freeze a player
//             feels on a lossy link, and why loss here is a hold and never a hole in the stream. THE WAIT is a round trip and `resendMs` more: in a steady
//             stream the sender learns of the loss from the messages after it (three of them at 60 a second: resendMs) and the resend then makes the trip.
//             (A stream that has gone quiet waits for a timer instead, a quarter of a second and up: not modelled — the game's stream is never quiet.)
//   stallUntil  nothing passes before that time (performance.now's clock), what was already on its way too: a connection that goes quiet for a moment and
//             comes back with everything at once. It is the queue's `shut`, not a longer wait, so a stall taken back is over
// All zero = the wire as it is: `delayed` calls straight through.
export interface Way { lagMs: number; jitterMs: number; lossPct: number; resendMs: number; stallUntil: number }
export const wayOff = (): Way => ({ lagMs: 0, jitterMs: 0, lossPct: 0, resendMs: 60, stallUntil: 0 });
/** how long this message waits on that way (`roll`: a die in [0, 1); asked once for the jitter, and once more only where there is loss) */
export function wayMs(W: Way, roll: () => number = Math.random): number {
  const d = W.lagMs + roll() * W.jitterMs;
  return W.lossPct > 0 && roll() * 100 < W.lossPct ? d + 2 * W.lagMs + W.resendMs : d;
}
/** one direction of a client's way: `fn` behind that table's waits and its stall */
export const onWay = <T>(W: Way, fn: (x: T) => void, roll?: () => number): ((x: T) => void) => delayed(fn, () => wayMs(W, roll), () => W.stallUntil);
