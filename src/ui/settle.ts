// THE START WAITS FOR THE PICTURE (2026-10-28). The maintainer, after a first evening on the build whose start card goes live on the first frame's shaders:
// "i also quite dislike the 'popin' that happens from starting the game too early when some shaders haven't finished".
//
// A mission's page went live the moment the level stood (main.ts `interactive`): the brief's START took a press before one frame of the level had been drawn,
// and a press kept from the boot started the run at once. So the run's first seconds were the renderer's own first seconds: a pass still on its stand-in
// while its program compiled (`gpu/device.ts` PIPE_QUEUE.later), the second set of the tracing programs compiling in the background (render/cutSwap.ts),
// every history empty (the cascades, the reservoirs, the denoiser, the upscale), and auto quality measuring the machine in view, then changing the preset
// and the render scale in one step about a second in. Each of those is a picture that changes under the player, and a GPU that is busy with something
// other than the frame.
//
// Now the brief HOLDS between the moment the level stands and the moment it is live (ui/loadbrief.ts `hold`): the world is still, the frames are drawn
// behind it, the block is not live and a press is kept as it is while the page boots. It goes live when
//   1. every program is in: none of the later set is out and the second (cut) set is built or has failed — NOTHING COMPILES ONCE THE RUN IS LIVE; then
//   2. `frames` frames have been drawn with those programs and auto quality has made its pick (a pinned preset: the frames alone).
// Two ways out that are not that: `programSecs` of waiting on programs (a compile that never comes back must not keep a mission shut: the wait ends and the
// picture's turn begins, as the build did before this) and `maxSecs` of the picture's turn (a page too slow to measure: auto quality's own measure gives
// up on frames of half a second). The time a tab spent in the background is not the wait's: main.ts starts its clock again when the tab is back.
// `frames` is half the 90 that main.ts gives a still behind a page that covers it (`behind.warm`): that one must be right for as long as it is looked at,
// this one only has to have the histories filled when the brief fades. Unmeasured: nobody has looked at frame 45 on a GPU.
// While it waits the compile queue of a Windows browser runs as it does during the boot (`pipeQueue.rush`): nobody sees the frames it was sparing.
// THE SWITCH: SETTLE.on — ON in the page for a mission's brief where a person sits (main.ts; `?settle=0` = live the moment the level stands, as the
// build was), OFF in the module and on a page a script drives (navigator.webdriver: the page tools press START the moment the level stands; `?settle=1`
// raises it there, a number over 1 is `frames`, `?settlewait=N` is `programSecs`). A page switch like the brief itself: no game rule reads it and the
// harness has no page.
// This file imports nothing. tools/qa/settle-check.ts holds the rule; `SETTLE_BREAK=1` in front of it = the start live at once, and it must FAIL.

export const SETTLE = { on: false, /** frames drawn with every program in and the pick made before the start is live */ frames: 45, /** seconds the picture's turn may take */ maxSecs: 12, /** seconds the wait on programs may take */ programSecs: 120 };

/** what the page says each step: `out` = programs still compiling (every pipeline asked for and not in, the later set's, the second set while it builds), `measuring` = auto quality has not made its pick */
export interface SettleFacts { out: number; measuring: boolean }

const BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.SETTLE_BREAK ?? '') === '1'; } catch { return false; } })();

/** one mission's wait: `step` once a display frame until it answers true */
export class Settle {
  /** seconds waited on programs */
  waited = 0;
  /** seconds and drawn frames of the picture's turn (both 0 while a program is out) */
  pictured = 0; frames = 0;
  done = false;
  /** the programs are in (or their wait ran out) and the picture is having its turn */
  get picture(): boolean { return this.pictured > 0; }
  /** `dt` = seconds since the last step, `drawn` = frames the renderer drew in them. True = the start may go live (and stays true) */
  step(dt: number, drawn: number, f: SettleFacts): boolean {
    if (this.done || BREAK) return this.done = true;
    const d = Math.max(dt, 0);
    if (f.out > 0 && this.waited < SETTLE.programSecs) { this.waited += d; this.pictured = 0; this.frames = 0; return false; }   // (a program that lands late starts the picture's turn again: its frames were a stand-in's)
    this.pictured += d;
    if (f.measuring) this.frames = 0; else this.frames += Math.max(0, drawn);   // (the pick changes the preset and the render scale in one step: the frames that count are the ones drawn after it)
    return this.done = this.frames >= SETTLE.frames || this.pictured >= SETTLE.maxSecs;
  }
}
