// WORK MADE IN STEPS (the picture's planners: render/wallSides.ts; scene/openings.ts with engine.ts lookOpeningLights; render/planPace.ts is the page's pace).
// A planner that reads the whole level (which side of each wall a cell is on, where the level is open to the sky) runs when a wall changes and at no
// other time, so in a browser it runs COLD every time: measured in Chromium on the office, 13 to 18 ms for the wall sides and 9 to 15 for the openings
// where the same code run warm takes under 2. All of it landed on the frame a wall was blown, or the frame the walls were read after it. Tuning such
// code does not help (it is slow because it is rare, not because of what it does), so the rule is about WHEN: a planner is a generator that stops every
// so often, a frame gives the planners an allowance between them (`Pace`), and what a planner last made serves until what it is making is whole.
// WHAT A PLANNER OWES for that: everything that can change under it is taken before its first stop, in the call that begins it. Nothing half made is
// ever shown, and a plan made over ten frames is the plan made in one, of the level as it stood when it began.
// THE ALLOWANCE IS TIME in the page (`ms`): warm code is then whole in the frame that asked, as it always was, and cold code takes the frames it needs.
// A check gives a count of stops (`left`), which no clock can change. A JOB IN HAND IS NEVER HELD UP: every call takes one step at least, whatever is
// left of the allowance, so nothing a frame's other planners do, however often, can keep one from being finished.
// No generator is kept where the simulation's state is (a job half done is in no snapshot: tools/qa/rollback-check.ts). The simulation does read the
// picture through its sensors (the light meter reads the cache, and the cache keeps to a wall's side by the words): what a planner makes a few frames
// late, a sensor sees a few frames late, as it sees the cache itself come round to a hole over many more.

/** a frame's allowance, spent by every planner that is given it: stops left, and (the page's) milliseconds left. Infinity stops and no milliseconds = a
 *  planner runs to its end (a tool, a check, a level's first frame). */
export interface Pace { left: number; ms?: number }
/** an allowance that is never spent */
export const WHOLE: Pace = Object.freeze({ left: Infinity });
/** does the allowance stop a planner at all */
export const paced = (p: Pace): boolean => p.left !== Infinity || p.ms !== undefined;
/** the clock an allowance of time is kept by (ms) */
export const clock = (): number => typeof performance !== 'undefined' ? performance.now() : 0;
/** planning done outside a job (a plan that is still made whole) is charged to the frame's allowance: `ms` of it are gone */
export function spend(p: Pace, ms: number): void { if (p.ms !== undefined) p.ms = Math.max(0, p.ms - ms); }

/** a planner run to its end, every stop passed */
export function whole<T>(it: Generator<void, T, void>): T { for (;;) { const r = it.next(); if (r.done) return r.value; } }

/** one job in hand. `T` is never undefined: go() answers undefined for 'not yet'. */
export class Steps<T> {
  private it: Generator<void, T, void> | null = null;
  get busy(): boolean { return this.it !== null; }
  begin(it: Generator<void, T, void>): void { this.it = it; }
  drop(): void { this.it = null; }
  /** on with it: one step at least, then more until it is whole or the allowance is spent. What it made, or undefined. A step that throws ends the job
   *  (the throw is the caller's). */
  go(pace: Pace): T | undefined {
    const it = this.it; if (it === null) return undefined;
    if (!paced(pace)) { this.it = null; return whole(it); }
    const timed = pace.ms !== undefined, t0 = timed ? clock() : 0;
    try {
      for (;;) {
        const r = it.next(); if (r.done) { this.it = null; return r.value; }
        if (pace.left !== Infinity && pace.left > 0) pace.left--;
        if (!(pace.left > 0) || (timed && clock() - t0 >= pace.ms!)) return undefined;
      }
    } catch (e) { this.it = null; throw e; }
    finally { if (timed) spend(pace, clock() - t0); }
  }
}
