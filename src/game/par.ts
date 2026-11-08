// A PAR: a room clear run AGAINST A CLOCK (the Door Kickers star rating). A level file carries a `scenarios.par` block (scene/scenarios.ts ParData:
// gold / silver / bronze seconds and `lossSecs`); while that level's room clear is up, this module times the run and, once, names the medal.
//
//   THE CLOCK STARTS on the first thing anybody DOES: the squad's first order (a row on orders.log), Sam's first stride off the mark the staging stood him on
//   (PAR_START_M), or his first round or throw — whichever comes first: reading the house from the start line is free, doing anything is not.
//   IT STOPS when the room clear ends 'taken' (roomclear.ts `end`, at its own `endAt`). A run that ends 'wiped' has no time.
//   EVERY TEAMMATE LOST (down at the end: dead or out cold — the floor keeps both as state 'dead') adds `lossSecs`: the total is what the medals
//   are held against, so a run that loses men ranks under a clean one of the same pace.
//   THE MEDAL IS JUDGED ON WHAT IS SHOWN: the total in whole seconds, as the clock words it (65.4 s reads 1:05 and is inside a 1:05 gold).
//
// WHOSE IT IS: THE RUST GAME'S (rust/game_rules/src/par). The tick arms the clock, starts it, stops it, files the result and says its line; `Game.par` is its
// record, and the bridge carries all of it to the page (tools/wire: `par::Par`). Nothing here writes that record: this file is what
// the page reads it WITH (is a par live, has the clock started, the result). The room clear's row with the clock on it is the Rust game's too
// (`par::par_objective`, through `mission::objective`, in the view).
// A BEST TIME is the page's to keep, a machine's own (main.ts + ui/parBest.ts): it hands its words in through `best()`, and they ride the result where
// THIS page shows it (the card, the debrief) and never the game's log, which every machine of one game reads the same.
//
// (src/game is one import cycle: nothing here is built from another game module's exports at module load.)
import type { Game } from './game';
import type { ParData } from '../scene/scenarios';
import type { Vec3 } from '../math/vec';

export const PAR_STATS = { started: 0, finished: 0 };
/** how far Sam walks from where the staging stood him before the clock is his (metres): more than a shuffle, less than the way to any door */
export const PAR_START_M = 1.0;

export type Medal = 'gold' | 'silver' | 'bronze' | 'none';
/** a finished run against its par: `secs` on the clock, `lost` teammates × `lossSecs` = `penalty`, `total` = their sum, `shown` = the total in whole seconds
 *  (what the player reads, and what the medals are held against); `next` = the medal above the one earned and how many seconds faster it needed (none on gold:
 *  the bridge leaves it unset) */
export interface ParResult { secs: number; lost: number; of: number; penalty: number; total: number; shown: number; medal: Medal; next?: { medal: Medal; bar: number; short: number } | null; line: string }

/** m:ss */
export function clock(s: number): string { const n = Math.max(0, Math.floor(s)); return `${Math.floor(n / 60)}:${String(n % 60).padStart(2, '0')}`; }

export class Par {
  // ---- the Rust game's record, as the bridge writes it (the fields of `par::Par`, by name)
  /** the mission this clock is for, by the Rust game's serial for it */
  run: number | null = null;
  /** every seat's mark as the clock was armed: where its man stood, and the last row of its orders log then */
  marks: { origin: Vec3; order0: number | null }[] = [];
  fired0 = 0;
  /** when the clock started on the game's clock (NaN: not yet) */
  t0 = NaN;
  /** the finished run (null: not taken yet, wiped, or another mission's) */
  res: ParResult | null = null;
  /** a par is live: the level has the block, its room clear is up, no show runs */
  live = false;
  // ---- the page's own: this machine's words about its best time, and the run they are about (presentation: `best()`)
  private bestWords = '';
  private bestRun: number | null = null;
  constructor(private g: Game) {}

  /** the level's par while it is live (null = stood down) */
  get data(): ParData | null { return this.live ? this.g.level.scenarios?.par ?? null : null; }
  /** has the clock started */
  get started(): boolean { return this.live && Number.isFinite(this.t0); }
  /** seconds on the clock now (0 before it starts; held at the end's own time once the room clear has ended) */
  get secs(): number { if (!this.started) return 0; const rc = this.g.roomClear, t = rc?.end ? rc.endAt : this.g.time; return Math.max(0, t - this.t0); }
  /** the finished run (null: not taken yet, wiped, or no par here) */
  result(): ParResult | null { return this.live ? this.res : null; }
  /** the result's line as this page shows it: with its words about the best time, once it has handed them in */
  get shown(): string | null { const R = this.result(); return R ? R.line + (this.bestSaid && this.bestWords ? ` · ${this.bestWords}` : '') : null; }
  /** has this page said its words about THIS run's result (they are about one run: another mission's clock does not show them) */
  get bestSaid(): boolean { return this.bestRun !== null && this.bestRun === this.run; }
  /** THE PAGE's words about this machine's best time for this run (main.ts, once, when the result is there; '' = it files none for this run) */
  best(words: string) { if (!this.result()) return; this.bestWords = words; this.bestRun = this.run; }
}
