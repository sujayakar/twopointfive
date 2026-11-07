// THE FIXED STEP: the game is stepped in whole ticks of 1 / hz whatever the display's rate; this says how many a frame runs and what is owed after them (the
// frame order round them is the host's: loop.ts, drawpage.ts).
/** the fixed step's knobs. `on`: the switch (off = the variable step, the loop as it was). `hz`: ticks a second. `snap` (s): a frame time within this of a whole number of
 *  refreshes of a common display is taken as exactly that (a 60 Hz display's 16.6‥16.8 ms frames are each ONE tick, never the 0-then-2 a raw accumulator beats out).
 *  `maxTicks`: the most ticks one frame may run (a hitch longer than that is dropped, not caught up). */
export const FIXEDSTEP = { on: false, hz: 60, snap: 0.0004, maxTicks: 4 };
const RATES = [50, 60, 72, 75, 90, 100, 120, 144, 165, 240];
/** `dt` moved onto the nearest whole number (1‥3) of refreshes of a common display when it is within `tol` of one; otherwise as given */
export function snapDt(dt: number, tol = FIXEDSTEP.snap): number {
  let best = dt, err = tol;
  for (const r of RATES) for (let m = 1; m <= 3; m++) { const e = Math.abs(dt - m / r); if (e < err) { err = e; best = m / r; } }
  return best;
}
/** the pure plan: `acc` seconds owed + this frame's `dt` → whole ticks of 1 / hz to run now and what is owed after them.
 *  `drift` = the wall time the snap has put aside, leaking 1 % a frame: jitter round a true refresh sums to nothing and a display a hair off a listed rate (59.94 Hz) stays
 *  one tick a frame for good — but a display the list does not have (70 Hz snapping to 72, 85 to 82.5) builds half a tick of it inside half a second and gets it back
 *  as time, so the game's clock never runs steadily off the wall's. */
export function planTicks(acc: number, dt: number, K: { hz: number; snap: number; maxTicks: number } = FIXEDSTEP, drift = 0): { ticks: number; acc: number; drift: number } {
  const step = 1 / K.hz; dt = Math.max(0, dt);
  let use = snapDt(dt, K.snap);
  drift = drift * 0.99 + (dt - use);
  if (Math.abs(drift) > step / 2) { use += drift; drift = 0; }
  acc += use;
  if (acc < 0) { drift += acc; acc = 0; }   // (time handed back that this frame cannot pay — a display a little FASTER than a listed rate, 170 Hz against 165: it stays owed; an accumulator under zero planned a frame of −1 ticks)
  let ticks = Math.max(0, Math.floor(acc / step + 1e-6));
  acc -= ticks * step; if (Math.abs(acc) < step * 1e-6) acc = 0;
  if (ticks > K.maxTicks) { ticks = K.maxTicks; acc = 0; drift = 0; }
  return { ticks, acc, drift };
}
