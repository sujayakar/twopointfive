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
//
// A WINDOW NOBODY IS AT GIVES WAY (`giveway`, 11-03). THE FAULT: two windows of the game in one browser, for a co-op game on one laptop, and the second sat on
// 'starting the GPU…'. A browser has ONE process that talks to the GPU for all of its windows, and nothing in this loop waits for the GPU: a window hands over
// frames as fast as the browser will take them, whoever is looking, so the first window's title (its rooms live behind the card) kept that process busy and the
// second window's first asks of it, for the adapter and the device, waited their turn behind every frame. Seen here on the software adapter (two windows of
// headless Chromium: the two asks took 49 s and 196 s beside a first window drawing, 34 ms and 5 ms alone); on a GPU nobody has measured it.
// THE RULE: a window that does not have the system's focus (the window that takes the keys is another one, or another program) has ONE frame with the GPU at
// a time: the next is not begun until the queue says the last one's work is done. And it takes at most `share` of the GPU's time: it begins the next no
// sooner than that frame's work took, over `share`, after the last was begun (half and half at 0.5), though never more than `longest` after it: the loop's
// step takes a twentieth of a second at the most (main.ts frame), so a longer wait would run the window's own clock slow and a picture nobody is at would
// stop looking alive. A window with room to spare never notices (work that takes a third of its frame is under the share already); one whose work fills
// its frame draws about every second frame while nobody is at it, and is back to its own pace on the frame the focus comes back. The window somebody IS at
// is never held by this: its frames go as they always did. Not while the page measures the machine or records a take.
// THE MEASURE of a frame's work is the GPU's own timing of its passes where the page has one (`done`'s `own`: the ladder's number), and never more than the
// time the queue took to answer; where there are no timings it is the queue's answer alone, counted from when the GPU was free of this window's frame
// before it. The answer by itself would count the work of the OTHER windows ahead of this one in the queue, and a window beside a busy one would starve
// itself. The wait runs from the last worked frame's own stamp, which a stall does not forget (the even pace's `at` is forgotten across a stall, so that a
// late frame moves the beat).
// ON in the page, `?giveway=0` = an unfocused window draws as a focused one does, as it was; OFF here and on a harness stand.
//
// THE PAGE WAITS FOR THE GPU BEFORE IT STARTS A FRAME (`flight`, 11-03). The root of both faults above is one sentence of them: nothing in this loop waits for
// the GPU. The even pace cures frames in pairs only where the ladder's rate divides the display's and the work fits the paced frame; past that (a machine that
// cannot hold the rate, a 60 Hz display, a pinned preset) frames still pile up behind the GPU and the browser lets them go in bursts. The look page had a trial
// of the plain cure (its `&pace`: a display frame is passed over whole while two frames are still with the GPU, told by the queue's own
// `onSubmittedWorkDone`), and the maintainer on his 120 Hz laptop: '&pace looks good, let's make it the default'. So it is this pacer's rule for every window:
// a frame is not begun while `flight` frames are still with the GPU. Two = one being drawn and one waiting behind it: the GPU never idles, so no frame a
// second is lost to the wait, and a frame is at most two old. One (`?flight=1`) = a frame is begun only when the last has left: the freshest picture, and
// the GPU idle until the display's next tick. A window nobody is at keeps to one, as above. The even pace stays under it: it says WHICH display frames are
// asked for, this says whether the GPU has room for the one asked. With work inside the paced frame nothing is ever held and the pace is the even one;
// with work over it, frames come as the GPU finishes them, one or two display frames apart, never back to back. A frame held for this is the machine's own
// pace, so the ladder reads its gap as it reads any other. Not while the page measures the machine, as above: a benchmark's frames are as they always were.
// And not under ?kitview: the kit picture's pacer reads how late its own frames come and steps down to an even pace by that; a frame held here after that
// pacer had taken it would hide the lateness from it. The queue is asked once for every frame that DREW (a card that hides the picture draws nothing and
// asks nothing). A FRAME THE QUEUE NEVER ANSWERS FOR is given up after `lost` ms and the page goes on without it: a window held at its limit for good by
// one lost answer would be a frozen picture with no card to say why. (Ten seconds: a frame on the software adapter is seconds long and is not lost.)
// ON in the page at two (`FLIGHT_PAGE`), `?flight=0` = nothing waits, as it was; OFF here and on a harness stand.
/** how many frames the PAGE lets be with the GPU when another is begun (the page's boot, the shipped stand and the check read this one number) */
export const FLIGHT_PAGE = 2;
export const EVENPACE = {
  on: false,
  /** the page waits for the GPU (the block above): how many frames may be with the GPU when another is begun (0 = the rule off), and how long a frame may
   *  go unanswered before it is given up as lost (ms) */
  flight: 0, lost: 10000,
  /** a window nobody is at gives way (the block above): the switch, and the most of the GPU's time such a window takes */
  giveway: false, share: 0.5,
  /** …the longest it waits to begin its next frame (ms: the loop's own cap on a step), and how much of one display frame short of its wait a stamp may come
   *  and the frame still be worked (stamps wobble; `slack` would forgive a third of the wait at 60 Hz) */
  longest: 50, wobble: 0.1,
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
  /** a frame callback stamped `ts` (ms), with the rate being held (0 = none: every frame is worked, and the period is still read). `away` = nobody is at this
   *  window (it gives way, under `giveway`). `room` = how many frames may be with the GPU when this one is begun (0 = any number: the boot's
   *  calibration, and ?kitview; left out = `flight`). true = pass this display frame over whole (touch nothing: the next worked frame takes the real time). */
  skip(ts: number | undefined, target: number, away?: boolean, room?: number): boolean;
  /** a worked frame has handed its work to the GPU, at `now` (ms): the word to give `done` back when the queue says that frame is finished, at `now`, with
   *  the GPU's own timing of a frame's passes where the page has one (`own`, ms; 0 = none). An answer for a frame already given up as lost changes nothing */
  sent(now: number): number;
  done(word: number, now: number, own?: number): void;
  /** a pause: the gap across it is nobody's (the period is kept) */
  reset(): void;
  /** the display's rate as read (0 = not yet), how many display frames a drawn frame takes right now, and how many were passed over and worked since the page began */
  readonly hz: number; readonly every: number; readonly skipped: number; readonly worked: number;
  /** waiting for the GPU: how many frames are with it now, and how many display frames were passed over because it had no room (`held`: a window somebody is
   *  at; `gave`: one giving way, its share's wait included); the GPU's work over the last frame it answered for (ms, 0 = not yet) */
  readonly flying: number; readonly held: number; readonly gave: number; readonly gpuMs: number;
  /** frames the queue never answered for in time, given up */
  readonly lost: number;
}

export function pacer(): Pacer {
  const gaps: number[] = [];
  const out: { word: number; at: number }[] = [];   // (the frames still with the GPU, oldest first: each one's word and when it was handed over)
  let lastTs = -1, at = -1, begun = -1, period = 0, every = 1, skipped = 0, worked = 0, gave = 0, held = 0, gpuMs = 0, lost = 0, words = 0, freeAt = 0;
  return {
    skip(ts, target, away = false, room = EVENPACE.flight) {
      if (typeof ts !== 'number') { every = 1; worked++; return false; }
      if (lastTs >= 0) { const d = ts - lastTs;
        if (d > 250 || d < 0) at = -1;   // (a stall, a tab that was away: not a gap of the display's)
        else if (d >= 1.5) { gaps.push(d); if (gaps.length > EVENPACE.gaps) gaps.shift(); if (gaps.length >= EVENPACE.least) { const p = periodOf(gaps); if (p === p) period = p; } } }
      lastTs = ts;
      while (out.length > 0 && ts - out[0].at > EVENPACE.lost) { out.shift(); lost++; }   // (a frame the queue never answered for: the page goes on without it)
      every = EVENPACE.on && period > 0 ? paceEvery(1000 / period, target) : 1;
      if (every > 1 && at >= 0 && ts - at < (every - EVENPACE.slack) * period) { skipped++; return true; }
      // (a window nobody is at: its last frame is still the GPU's, or the GPU has not yet had as long to itself as that frame took of it)
      if (away && EVENPACE.giveway) { if (out.length > 0 || (begun >= 0 && ts >= begun && ts - begun < Math.min(EVENPACE.longest, gpuMs / EVENPACE.share) - EVENPACE.wobble * period)) { gave++; return true; } }
      // (…and any window: the GPU still has as many of its frames as it may)
      else if (room > 0 && out.length >= room) { held++; return true; }
      at = begun = ts; worked++; return false;
    },
    sent(now) { out.push({ word: ++words, at: now }); return words; },
    done(word, now, own = 0) {
      const i = out.findIndex(o => o.word === word); if (i < 0) return;   // (given up as lost, or answered already)
      const took = Math.max(0, now - Math.max(out[i].at, freeAt)); out.splice(i, 1); freeAt = now;   // (its own time: from when the GPU had it and was free of the frame before it)
      gpuMs = own > 0 ? Math.min(own, took) : took;
    },
    reset() { lastTs = at = begun = -1; },
    get hz() { return period > 0 ? 1000 / period : 0; }, get every() { return every; }, get skipped() { return skipped; }, get worked() { return worked; },
    get flying() { return out.length; }, get held() { return held; }, get gave() { return gave; }, get gpuMs() { return gpuMs; }, get lost() { return lost; },
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
