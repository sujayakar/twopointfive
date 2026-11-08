// WHERE THE VERDICT IS WORKED OUT: in the Rust game's own tick (rust/game/src/verdict/p_01.rs), which latches it. This file is its words for the page: the
// record's types, the reader the record is applied onto, and the card's line. What follows describes the rule, which is the same rule.
//
// How a run went, said once it is over (the end card's record: ui/verdictCard.ts draws it, ui/verdictStore.ts keeps what this machine keeps of it).
//
// THE MODULE ONLY READS THE GAME (as par.ts and lesson.ts do): it moves nobody, draws no die and writes nothing the simulation reads, so a run is the same run
// with `VERDICT.on` false, digest for digest, on either stand. `tick()` comes after the scenarios, and the mission have had this
// frame, so every end they wrote is in.
//
//   A TRY starts at the squad's first order, Sam's first stride (PAR_START_M off the mark he was staged on) or the first round or throw — the par clock's rule,
//     for every kind of mission: restarting before that is free. `started` goes up then and the run's clock runs from there.
//   THE END is each mission kind's own (a room clear taken or wiped, a hostage rescue's result, the night breach's room, the
//     quiet game's exfil) or a player down. One `RunVerdict` is made then and never again for that run: won or lost in one word, the seconds, one row per man
//     (who was up at the end, when and where a man fell and to whom, when he dropped somebody, where he walked), the defenders, and — for a win — THE WAY it
//     was won, read off the run, one way a run:
//       quiet   no defender ever went alert or searching and no alarm was raised
//       dark    the mains were cut (a hand at the breaker, the OCP, a round into it) and were still out when the team was first found
//       walls   a man of the team went through a hole a blast opened in a wall
//       doors   anything else
//     in that order, so two pips on a mission card are two different plans.
//   CHEATS (god mode, no AI, unlimited ammo, invisible) on at any frame of the try: `void` — the card still shows, nothing is kept of it.
//   THE CARD: the stealth mission sends its own (the game's out event EndCard); for every other end this module says when (`takeDue()`, read by the page).
//
// Per-player by construction: the men are `humans(g)` (today the one Player) then the AI teammates; nothing here asks for "the" player by name but that list.
// Not in a checkpoint: a load is the SAME try (`resumed`), and for the kinds that can be loaded (the quiet game alone) the way won is read off the chronicle,
// which a checkpoint does carry. `kept` is the page's note to the card (which try of the streak this was, the ways won so far) — display only.
import type { Game } from './game';
import { activeScenario, runningScenario } from './scenarioRun';
import { levelNameOf } from '../scene/levelLoad';   // (asked inside rowFile only)
import { LOOK_LEVELS } from '../scene/look';   // (a table of file names: read inside rowFile only)

export const VERDICT = {
  on: true,
  /** a mission of the stealth game's kind that leans LOUD is not graded as a sneak (`loudRow` below): its end card, the copied line and the two lines the
   *  mission says at its end carry no rating word, no score out of a hundred, no badges, and the sneak's numbers on the card stand untoned. ON in the page
   *  (`?loudcard=0` = the stealth game's rating on every card it stands on, as it was), off here and on the frozen stand (TPS_LOUDCARD=1). Reads only: the
   *  run is the same either way. A recording carries it (knobs.ts JOURNAL_KNOBS: the mission's two end lines are worded by it), and one from before the
   *  key replays with it off. */
  loudCard: false,
  /** seconds between a squad mission's end and its card (the stealth mission's own card keeps its own beat) */
  winSecs: 2.0, lossSecs: 2.6,
};

/** The file a mission row names for the level this game stands on (null: the bundled office, which rows name by naming none). The game is not told the
 *  page's `levelFile`, so it is read back off the level's own name: `<name>.json`, or the file whose look that is (scene/look.ts LOOK_LEVELS: the page
 *  loads `barracks-gwot.json` for the row on `barracks.json`). `probes.ts debrief-loud` holds every row's file and its look's file to this. */
export function rowFile(g: Game): string | null {
  const name = levelNameOf(g.level); if (!name || name === 'office') return null;
  const file = `${name}.json`; for (const k in LOOK_LEVELS) if (LOOK_LEVELS[k] === file) return k;
  return file;
}
/** The mission that is up is the STEALTH GAME'S kind (a quiet mission: the one kind whose card carries the rating beside a verdict) on a row of the list
 *  that LEANS LOUD (scene/scenarios.ts `lean`: the mission sheet's 'Approach'), asked while VERDICT.loudCard is up. The stealth game's rating prices every
 *  man shot and every time he was seen, and a mission that is meant to be shot through reads as failed by it ('ASSAULT 14' on a won tutorial fight) — so
 *  on such a row nothing grades the run as a sneak. A row that leans either way, or quiet, keeps its rating; no row up (the plain night) keeps it; a
 *  squad's kinds never showed it on their end card and are not asked. */
export function loudRow(g: Game): boolean {
  if (!VERDICT.loudCard) return false;
  const run = runningScenario(g, rowFile(g));
  return run?.kind === 'quiet' && run.entry?.lean === 'loud';
}

/** the way a run was won, read off the run */
export type Approach = 'doors' | 'walls' | 'dark' | 'quiet';
export type ManState = 'up' | 'wounded' | 'out' | 'dead';
export type ManTone = 'you' | 'purple' | 'red' | 'blue' | null;

/** one man of the team at the run's end. Times are seconds from the try's start; `path` is x, z, t triples sampled as he walked. */
export interface VerdictMan {
  id: number; name: string; tone: ManTone; human: boolean;
  state: ManState; downAt: number; downRoom: string; downBy: string; downPos: [number, number] | null;
  drops: number[]; path: number[];
}
export interface RunVerdict {
  kind: ReturnType<typeof activeScenario>;
  won: boolean; word: string; why: string;
  /** seconds from the try's start to the end */
  secs: number;
  approach: Approach | null;
  /** a cheat was on during the try: nothing of it is kept */
  void: boolean;
  /** nobody of the team was lost */
  clean: boolean;
  men: VerdictMan[]; lost: number; of: number;
  /** the defenders: how many went down of how many, and where (x, z pairs) */
  foes: { down: number; of: number; fell: number[] };
  hostages: { alive: number; of: number } | null;
  /** where the team started (the map's mark) */
  start: [number, number];
}
/** the page's note to the card: which try of the streak this was (0 = not a counted try), the ways won on this mission so far */
export interface KeptNote { streak: number; fresh: boolean; gold: boolean; ways: Partial<Record<Approach, { first: boolean; clean: boolean }>>; /** a Training lesson's run: nothing is kept of it and the card grades nothing — no ways, no tries */ lesson?: boolean }

/** THE VERDICT ON A PUPPET IS A READER. The Rust game works the verdict out in its own tick and latches it (rust/game/src/verdict/p_01.rs: the first end
 *  of a run stays); what the page reads of the record is applied onto this object every tick: the run it is of (the mission record), the try's start,
 *  the result once there is one, when its card is due. Two things here are the PAGE'S OWN and are never applied: `kept`, the page's note to the card,
 *  and which run's card has been raised. */
export class Verdict {
  private run: object | null = null;
  private t0 = NaN;
  /** this run was begun before a checkpoint load: the same try, not counted again */
  resumed = false;
  /** a cheat was on at some frame of the try */
  cheat = false;
  private res: RunVerdict | null = null;
  private cardAt = -1;
  /** (what a blank record holds where that is not zero: a start hands the Rust game this object's bytes, and its own blank says "not yet" as -1) */
  private foundAt = -1; private darkAt = -1;
  /** the run whose card the page has raised (the game's `cardAt` is applied every tick, so "taken" cannot be written on it) */
  private taken: object | null = null;
  kept: KeptNote | null = null;

  constructor(readonly g: Game) {}

  /** a run a verdict can be given on: never a show's, the tour's or a puppet's */
  get live(): boolean { const g = this.g; return VERDICT.on && !g.touring && !g.quietUtility && !g.puppet && !g.missionInTour; }
  /** the run this module is reading (the page keys its own bookkeeping on it) */
  get current(): object | null { return this.run; }
  /** the try has started: the first order, stride or round has been (or the run was loaded from a checkpoint) */
  get started(): boolean { return Number.isFinite(this.t0); }
  /** the run's verdict, once it is over (null before, and for a run that is not this mission any more) */
  get result(): RunVerdict | null { return VERDICT.on && this.run === this.g.mission ? this.res : null; }
  /** seconds since the try started */
  get secs(): number { return this.started ? Math.max(0, this.g.time - this.t0) : 0; }
  /** The page asks once a frame: true the one frame the end card is due for an end the stealth mission's own card does not cover. */
  takeDue(): boolean {
    if (!this.res || this.cardAt < 0) { this.taken = null; return false; }   // (a restart that puts the same Mission back blanks the record: the next try's card is due again)
    if (this.run !== this.g.mission || this.taken === this.run || this.g.time < this.cardAt) return false;
    this.taken = this.run; return this.live;
  }
  summary(): string { const R = this.result; return R ? verdictLine(R, this.kept) : this.started ? `try running · ${clock(this.secs)}` : 'not started'; }
}

/** m:ss */
export function clock(s: number): string { const t = Math.max(0, Math.floor(s)); return `${Math.floor(t / 60)}:${String(t % 60).padStart(2, '0')}`; }
/** the verdict as one line (the copy button, a console read-out) */
export function verdictLine(R: RunVerdict, kept: KeptNote | null = null): string {
  const parts = [`${R.word} ${clock(R.secs)}`];
  if (R.approach && !kept?.lesson) parts.push(R.approach);
  if (R.of > 1) parts.push(`${R.of - R.lost} of ${R.of} up`);
  if (R.foes.of) parts.push(`${R.foes.down} of ${R.foes.of} defenders down`);
  if (R.hostages) parts.push(`${R.hostages.alive} of ${R.hostages.of} hostages alive`);
  if (kept?.streak) parts.push(`try ${kept.streak}`);
  if (R.void) parts.push('cheats on: not kept');
  return parts.join(' · ');
}
