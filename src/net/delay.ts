// A WAY MADE LONGER, to look at lag on one machine (the page's ?netlag / ?netjitter: page.ts; the scripted client's --lag / --jitter: tools/mp/bot.ts).
// One queue a direction, let go by the clock: what goes in comes out that much later and IN THE ORDER IT WENT IN, as a socket keeps its order — a timer a
// message does not (two timers due the same millisecond fire as the browser likes, and a tick message that overtakes the one before it reads as a hole in
// the stream). A close goes through the same queue as the messages before it, so a goodbye is read before the socket it came on is seen shut.

/** `fn`, called `ms()` later (asked once a call; 0 with nothing waiting = at once), never out of order */
export function delayed<T>(fn: (x: T) => void, ms: () => number): (x: T) => void {
  const q: { at: number; x: T }[] = []; let last = 0, timer: ReturnType<typeof setTimeout> | null = null;
  const drain = (): void => {
    timer = null; const now = performance.now();
    while (q.length && q[0].at <= now) fn(q.shift()!.x);
    if (q.length) timer = setTimeout(drain, Math.max(0, q[0].at - performance.now()));
  };
  return x => {
    const d = ms(); if (!(d > 0) && !q.length) { fn(x); return; }
    const at = Math.max(last, performance.now() + Math.max(0, d)); last = at; q.push({ at, x });
    if (timer === null) timer = setTimeout(drain, Math.max(0, at - performance.now()));
  };
}
