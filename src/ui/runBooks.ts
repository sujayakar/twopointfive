// THE BOOKS OF THE RUNS PLAYED ON THIS MACHINE: each mission's ways won and tries (ui/verdictStore.ts) and each course's best time (ui/parBest.ts). The game says what there is
// to keep of a run (view RunView: whose, under which difficulty, how it ended); a page tells the books as its try begins and as it ends, and lays the note on its end card.
import type { KeptNote } from '../game/verdict';
import type { RunView } from '../view/read';
import { keptWays, recordOf, runEnded, tryStarted, verdictKey, waysWon, type MissionRecord } from './verdictStore';
import { parBest } from './parBest';

export class RunBooks {
  /** what the end card is told of the run that ended (null: none has) */
  note: KeptNote | null = null;
  /** how its time stands against this machine's best on that course (empty: no par, a cheat, or nowhere to keep one) */
  best = '';
  private key = ''; private streak = 0; private begun = false; private over = false;
  /** another run is the one that stands: a restart, a load (a try of its own), another mission */
  anew(): void { this.note = null; this.best = ''; this.key = ''; this.streak = 0; this.begun = this.over = false; }
  /** is there anything to be told of a run that is thus far on */
  due(begun: boolean, over: boolean): boolean { return (begun && !this.begun) || (over && !this.over); }
  /** THE RUN AS IT STANDS. A try is counted as it BEGINS, so one given up is a try all the same; never under a cheat, and never a lesson's. Its key is of that moment: a difficulty
   *  turned in the middle of a run is the next one's */
  tell(r: RunView): void {
    if (r.begun && !this.begun) { this.begun = true; this.key = verdictKey(r.mission, r.level); this.streak = r.void || r.lesson ? 0 : tryStarted(this.key); }
    if (!r.over || this.over) return;
    this.over = true;
    if (r.lesson) this.note = { streak: 0, fresh: false, gold: false, ways: {}, lesson: true };
    else if (r.void || !this.streak) this.note = noteOf(this.key ? recordOf(this.key) : null, 0, false);
    else { const K = runEnded(this.key, this.streak, { won: r.won, approach: r.way, clean: r.clean, secs: r.secs }); this.note = noteOf(K.rec, K.streak, K.fresh); }
    this.best = r.par === null || r.void ? '' : parBest(r.course, r.par);
  }
}
const noteOf = (rec: MissionRecord | null, streak: number, fresh: boolean): KeptNote => ({ streak, fresh, gold: !!rec && waysWon(rec) >= 2, ways: keptWays(rec) });
