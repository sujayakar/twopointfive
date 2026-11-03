// AN EVEN PACE ON A FAST DISPLAY, and the frame rate as it is (the game's own frame loop, main.ts frame()).
//
// THE FAULT, from two recordings off a 120 Hz laptop on High (GPU about 17 ms a frame): the display asks for a frame every 8.3 ms, nothing in the loop waits
// for the GPU, so the page hands over two frames back to back and is then held by the browser until the GPU has room: the gaps between frames ran
// 3, 30, 3, 29, 4, 27 … ms. That is 54 frames a second in pairs, each pair one long step of the world and one short one, and it reads as lag. With the GPU at
// 10 ms the same loop gave 5, 20, 5, 20. Only under about 8 ms was the pace even. And the readout said 170: it was a smoothed 1 / dt, the mean of the rates,
// which counts a 3 ms gap as 333.
//
// THE RULE: while the quality ladder holds a frame rate (ui/quality.ts: `bands.target`, 60 unless the player picked another), the page draws at that rate
// and not at the display's: on a display that shows a whole number of frames for each of the ladder's, the frames between are passed over whole. Every
// second one at 120, 144 and 165 Hz, every third at 180, every fourth at 240; every one at 60 to 100 Hz, at an aim of the display's own rate, at 'as now',
// with the ladder off (a pinned preset with Options ▸ Hold frame rate off), and while the page is measuring the machine (the boot's calibration, a benchmark,
// an F8 sweep), which wants the frames as they come. At 120, 180 and 240 Hz the paced frame IS the ladder's 60th, so the ladder holds the GPU's work inside it
// as it does on a 60 Hz display, the case it was built and measured on. At 144 and 165 Hz the paced frame is shorter than the ladder's (72 and 82 a second
// asked, 60 held): work that fits it is even, work between the two still misses a display frame now and then, as it did.
//
// ONLY AN EARLY FRAME IS PASSED OVER: a display frame that comes when the next drawn one is due, or late, is always worked. So a machine that cannot hold the
// pace is never made slower by this than the browser already made it, and a late frame moves the beat rather than doubling the wait.
//
// THE DISPLAY'S PERIOD is read here, off the stamps of the frame callbacks themselves, every one of them, passed over or not: the fastest gaps that agree,
// then every gap that is a whole number of those over the periods they span (the kit view's reading, host/kitView.ts `fastest`). A callback after work that
// ran long comes a whole number of periods later, so a page that is slow on every frame reads a slower display than it has; that page is given every frame,
// which is what it had. Frames in pairs show the period in the short gap of each pair, and a paced page shows it at every frame passed over, so a window
// dragged to a slower display is read again within a quarter of a second. (Not ui/quality.ts's `displayHz`: that one is only ever raised.)
//
// WHY NOT THE KIT VIEW'S PACER (kitViewPacer): that one also steps a slow machine DOWN to an even 45 or 30, and to do it passes frames over at boot and now
// and then after to see the period. In the game the ladder is what brings the work inside the frame, a pace under the ladder's cut would read to it as a slow
// machine, and a frame passed over to look is a hitch in a fight. This one never passes over a frame that was due.
//
// The page's alone, as the frame loop's others are (game/knobs.ts VISIT_OWN): nothing in the simulation reads it and a run is the same run either way. The
// step still takes the real time since the last worked frame. ON in the page, `?pace=0` = every display frame worked, as it was; OFF here and on a harness
// stand, where no display asks for anything. Never seen on a fast display by whoever wrote it: the check (tools/qa/pace-check.ts) plays stamps through it.
export const EVENPACE = {
  on: false,
  /** a rate a hair under a whole multiple still counts as that multiple (119.88 Hz is every second frame for 60), but never so far under that the pace would
   *  fall below the ladder's own cut (0.965 of its target: 116 Hz halved is 58) */
  edge: 0.05,
  /** how much of one display frame a stamp may come short of the due time and still be worked (stamps wobble; the frame before the due one is a whole frame short) */
  slack: 0.4,
  /** the period's reading: how many of the last gaps between callbacks are kept, how few make a reading, and how close the fastest three must agree */
  gaps: 24, least: 6, near: 1.12,
  /** the frame rate readout: the frames of the last this many seconds, over the time they took */
  window: 1,
};

/** how many display frames one drawn frame takes: 1 = every one. `displayHz` 0 = not known, `target` 0 = no rate is being held. */
export function paceEvery(displayHz: number, target: number): number {
  if (!(displayHz > 0) || !(target > 0)) return 1;
  return Math.max(1, Math.floor(displayHz / target + EVENPACE.edge));
}

/** the display's period, ms, off the gaps between frame callbacks (NaN = they do not say): the fastest three that agree say roughly what it is, then every gap
 *  that is a whole number of those, over the periods they span (the fastest alone read short: uneven stamps put the SHORT gaps in that bunch) */
export function periodOf(G: readonly number[]): number {
  const s = [...G].sort((a, b) => a - b), near = EVENPACE.near;
  for (let i = 0; i + 2 < s.length; i++) if (s[i + 2] <= s[i] * near) {
    let n = 0, sum = 0; for (let j = i; j < s.length && s[j] <= s[i] * near; j++, n++) sum += s[j];
    let p = sum / n;
    for (let pass = 0; pass < 2; pass++) { let k = 0, tot = 0; for (const g of G) { const m = Math.round(g / p); if (m >= 1 && Math.abs(g / p - m) <= 0.3) { k += m; tot += g; } } if (k) p = tot / k; }
    return p;
  }
  return NaN;
}

export interface Pacer {
  /** a frame callback stamped `ts` (ms), with the rate being held (0 = none: every frame is worked, and the period is still read). true = pass this display
   *  frame over whole (touch nothing: the next worked frame takes the real time). */
  skip(ts: number | undefined, target: number): boolean;
  /** a pause: the gap across it is nobody's (the period is kept) */
  reset(): void;
  /** the display's rate as read (0 = not yet), how many display frames a drawn frame takes right now, and how many were passed over and worked since the page began */
  readonly hz: number; readonly every: number; readonly skipped: number; readonly worked: number;
}

export function pacer(): Pacer {
  const gaps: number[] = [];
  let lastTs = -1, at = -1, period = 0, every = 1, skipped = 0, worked = 0;
  return {
    skip(ts, target) {
      if (typeof ts !== 'number') { every = 1; worked++; return false; }
      if (lastTs >= 0) { const d = ts - lastTs;
        if (d > 250 || d < 0) at = -1;   // (a stall, a tab that was away: not a gap of the display's)
        else if (d >= 1.5) { gaps.push(d); if (gaps.length > EVENPACE.gaps) gaps.shift(); if (gaps.length >= EVENPACE.least) { const p = periodOf(gaps); if (p === p) period = p; } } }
      lastTs = ts;
      every = EVENPACE.on && period > 0 ? paceEvery(1000 / period, target) : 1;
      if (every > 1 && at >= 0 && ts - at < (every - EVENPACE.slack) * period) { skipped++; return true; }
      at = ts; worked++; return false;
    },
    reset() { lastTs = at = -1; },
    get hz() { return period > 0 ? 1000 / period : 0; }, get every() { return every; }, get skipped() { return skipped; }, get worked() { return worked; },
  };
}

/** THE FRAME RATE READOUT: the frames of the last `window` seconds over the time they took. `push(now)` at every worked frame (ms on the wall clock) gives
 *  the rate; two frames at the least, so a page slower than the window still reads its own rate. */
export function rateMeter(): { push(nowMs: number): number; reset(): void } {
  const T: number[] = []; let fps = 60;
  return {
    push(nowMs) { T.push(nowMs); const w = EVENPACE.window * 1000; while (T.length > 2 && nowMs - T[0] > w) T.shift(); if (T.length >= 2 && nowMs > T[0]) fps = (T.length - 1) * 1000 / (nowMs - T[0]); return fps; },
    reset() { T.length = 0; },
  };
}
