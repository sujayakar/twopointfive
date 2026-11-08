// A MISSION'S START WAITS FOR ITS PICTURE (the rule: ui/settle.ts), on whichever page plays it: the brief is up over a world held still, its block not yet live, until every program is in (the later
// set too: nothing compiles once the run is live) and the frames behind it have been drawn with them and measured. A press kept from the boot starts the run then.
import { laterProgress, pipelineProgress, pipeQueue } from '../gpu/device';
import type { LoadBrief } from './loadbrief';
import { SETTLE, Settle } from './settle';

/** the address's words for the wait */
export function settleOfQuery(q: URLSearchParams): void {
  const v = q.get('settle'), w = Number(q.get('settlewait'));
  SETTLE.on = v === null ? navigator.webdriver !== true : v !== '0'; if (v && /^\d+$/.test(v) && +v > 1) SETTLE.frames = +v; if (w > 0) SETTLE.programSecs = w;
}
export interface StartWait {
  /** frames the renderer has drawn */
  drawn(): number;
  /** auto quality has not made its pick */
  measuring(): boolean;
  /** the wait needs the frames behind the brief drawn, or no longer does */
  settling(on: boolean): void;
  /** the wait is over: the block is live from here */
  settled(): void;
  /** the player has started the run */
  run(): void;
}
export function startWaits(b: LoadBrief, o: StartWait): void {
  if (!SETTLE.on) { o.settled(); b.live(o.run); return; }
  b.hold(); pipeQueue.rush(true);
  const P = pipelineProgress, S = new Settle(); let laterMost = laterProgress.pending, last = performance.now(), drawnAt = o.drawn();
  const back = (): void => { last = performance.now(); };   // (a tab that comes back: the time it was away is not the wait's)
  const end = (): void => { o.settling(false); pipeQueue.rush(false); document.removeEventListener('visibilitychange', back); };
  document.addEventListener('visibilitychange', back);
  const step = (): void => {
    if (!b.held) { end(); return; }
    const now = performance.now(), out = P.total - P.done + laterProgress.pending; laterMost = Math.max(laterMost, laterProgress.pending);
    const done = S.step((now - last) / 1000, o.drawn() - drawnAt, { out, measuring: o.measuring() }); last = now; drawnAt = o.drawn();
    if (done) { end(); o.settled(); b.live(o.run); return; }
    o.settling(S.picture);   // (only the picture's turn needs its frames: while programs compile the still behind the brief goes undrawn after its warm frames)
    b.status(S.picture ? 'getting the picture ready …' : `compiling shaders ${P.done + laterMost - laterProgress.pending} / ${P.total + laterMost} …`);
    requestAnimationFrame(step);
  };
  requestAnimationFrame(step);
}
