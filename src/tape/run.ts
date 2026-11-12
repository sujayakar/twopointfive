// THE RUN BEING PLAYED, KEPT: its tape goes to this browser's store every few seconds (keep.ts), at once as a mission ends, as the page goes and as the game is lost. F7 says it is kept; Shift+F7 and the end card's
// Replay leave the game to watch it, in this window.
import { typedInto } from '../host/keymap';
import { clock, leaveFor } from './format';
import { ReplayKeeper } from './keeper';
import { KEEP, keepBudget, pruneKept } from './keepstore';
import { keptStore } from './keptui';
import { reportWarning } from '../cloud/report';
import { PLAYTAPE, visitPairs } from './address';
import { HAND } from '../draw/input';
import { HANDTAPE, type HandTape } from './handtape';
import { handSource, type HandsHead } from './keep';
import { MODULE } from './module';

export interface RunDeps {
  /** the visit's name */
  sid: string;
  /** the run's tape, ended or not; null: nobody's run is on · it is still being written */
  tape(): HandTape | null; on(): boolean;
  head(): Omit<HandsHead, 'game' | 'hands' | 'hand' | 'module' | 'url' | 'userAgent'>;
  /** the run is over · somebody has it in hand */
  over(): boolean; live(): boolean;
  /** the tick the try that is on began after */
  began(): number;
  /** said to the player */
  say(words: string): void;
  /** the end of a run was kept */
  ended(): void;
}
/** how often the page says whether the run is over · how long its end waits for an idle moment · how long Shift+F7 stays asked · how long a leaving is let take */
const LOOK_MS = 250, IDLE_MS = 300, ASK_MS = 6000, LEAVE_MS = 8000;
const LS_KEEP = 'twopointsix.keep';

export class RunKeep {
  readonly keeper: ReplayKeeper;
  private asked = 0; private leaving = 0;
  constructor(private d: RunDeps) {
    const store = keptStore(), idle = (f: () => void, timeout: number): void => { if (window.requestIdleCallback) window.requestIdleCallback(() => f(), { timeout }); else f(); };
    const K = this.keeper = new ReplayKeeper(handSource(d.tape), { sid: d.sid, store, played: () => (d.tape()?.ticks ?? 0) > 0, say: words => d.say(words),
      head: (): HandsHead => ({ game: 'twopointsix', hands: HANDTAPE.version, hand: HAND, module: MODULE.name, url: location.search, userAgent: navigator.userAgent, ...d.head() }) });
    setInterval(() => idle(() => K.beat(), 2000), KEEP.secs * 1000);
    setInterval(() => { if (K.watch(d.over(), performance.now())) idle(() => { void K.endBeat().then(() => d.ended()); }, IDLE_MS); }, LOOK_MS);
    window.addEventListener('pagehide', () => { void K.leave(); });
    window.addEventListener('pageshow', e => { this.leaving = 0; if (e.persisted) K.back(); });
    document.addEventListener('visibilitychange', () => { if (document.visibilityState === 'hidden') void K.flush(); else K.back(); });
    void keepBudget().then(max => pruneKept(store, max, d.sid)).catch(e => reportWarning('replay.keep.prune', e));
    try { void navigator.storage?.persist?.().catch(() => { /* asked */ }); } catch { /* a sandboxed frame */ }
    window.addEventListener('keydown', e => { if (e.code !== 'F7' || e.repeat || e.metaKey || e.ctrlKey || e.altKey || typedInto(e.target)) return; e.preventDefault(); if (e.shiftKey) this.askLeave(); else void this.save(); });
  }
  /** what there is of the run is in the store now; what came of it is said */
  async save(): Promise<boolean> {
    const T = this.d.tape(), K = this.keeper; if (!T?.ticks) { this.d.say('There’s nothing to replay yet.'); return false; }
    const was = K.stats.dropped; await K.flush();
    if (K.broke || !K.stats.pieces || K.stats.dropped !== was) { this.d.say('This browser wouldn’t keep the replay.'); return false; }
    this.d.say(this.d.on() ? `Your replay is saved, and it’s ${clock(T.seconds)} long. Press Shift+F7 to leave the game and watch it.` : `Your replay is saved, but it stops ${clock(T.seconds)} in. What you’ve played since then isn’t in it.`); return true;
  }
  /** Shift+F7: with a run in hand the first press only says what the second costs */
  private askLeave(): void {
    const now = performance.now();
    if (this.d.live() && now > this.asked) { this.asked = now + ASK_MS; this.d.say('Press Shift+F7 again to leave this run and watch it. F7 alone saves it and stays here.'); return; }
    this.asked = 0; this.watch();
  }
  /** this window goes to watch the run, from where the try that is on began */
  watch(): void {
    if (performance.now() < this.leaving) return; this.leaving = Infinity;
    void this.save().then(kept => {
      if (!kept) { this.leaving = 0; return; }
      this.leaving = performance.now() + LEAVE_MS;
      const q = new URLSearchParams(); q.set(PLAYTAPE, this.d.sid); if (this.d.began() > 0) q.set('at', String(this.d.began()));
      for (const [k, v] of visitPairs(new URLSearchParams(location.search))) if (k !== 'paused') q.append(k, v);
      leaveFor(`${location.pathname}?${q}`, this.d.sid);
    });
  }
  /** the run can be watched to where it stands now */
  get whole(): boolean { return this.d.on() && !this.keeper.broke; }
  /** the game was lost: what there is of its run is kept before anything else is asked of the page */
  lost(): void { void this.keeper.flush(); }
}
/** is anything kept on this page: not where its address or this machine says not, nor where there is no store */
export function keeps(q: URLSearchParams): boolean { try { return !q.has('noreplay') && !q.has('nokeep') && localStorage.getItem(LS_KEEP) !== '0' && typeof indexedDB !== 'undefined'; } catch { return false; } }
