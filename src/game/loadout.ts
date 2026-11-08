// THE LOADOUT: how many teammates come with you — none to four — on whichever scenario is picked (the maintainer, 10-07: "a "loadout" screen or something like that for
// picking how many squad members you want … think like mgsv did!"; 10-06: "solo quiet, squad quiet, solo loud, squad loud are all options"). This file is the
// size itself and everything pure about it: the page's `?squad=N` (`?squad4` kept, = `squad=4` — but never beside `?rescue`, which did not read that flag), which sizes a scenario can stage (and why not, where it cannot),
// the roster a size stands up, the words for it, the query string Deploy loads, and what this machine picked last (localStorage `twopointsix.loadout`).
// ui/loadout.ts is the screen; the staging that makes a size real is nightbreach.ts stage() (the night breach and the room clear) and rescue.ts stage().
//
// A PICK STAYS A PAGE LOAD: Deploy loads the row's own URL with `squad=N` on it (only when N is not the scenario's own size — at its own size the URL, and so the
// run, is exactly the one the row always loaded), and game/scenarioRun.ts starts it from the query. So a pasted URL starts the same run, and the store below only
// remembers what the screen should show next time: it never changes a run by itself.
//
// WHAT A SIZE STANDS UP (nightbreach.ts SQUAD4 is the four's roster; the pair is 'Blue One' / 'Blue Two' as it always was):
//   0  nobody: you alone. No F drill (it takes two men), no order menu — the scenario's rooms, clock and ends are the same
//   1  Blue One. No F drill; he is the order menu's from the first frame (X: door tools on any door, one man's worth)
//   2  the pair, exactly as ever — F's drill on a level with a breach plan
//   3  Red One, Red Two, Blue One: the red pair and one blue man. F's drill is the red pair's; orders go to gold, red, blue (one man) or any man
//   4  the four, exactly as ever
// THE SIZE IN FORCE is not kept here: it is the men — game/scenarioRun.ts teamOf(g) / game.orders.squad() — and, as asked for, ScenarioSpec.squad and the two staging
// knobs startScenario writes (NIGHTBREACH.squad, RESCUE.squad). The roster below repeats nightbreach.ts SQUAD4's names (this file cannot import it: the cycle);
// tools/qa/loadout-probes.ts loadout-stage holds the two together, man for man at every size.
// Types and pure functions only: nothing here imports the game (src/game is one import cycle) or touches the DOM beyond localStorage, guarded.
import type { PairId } from './game';
import { soloFogRow } from './quietfog';   // (called inside noteOf only: src/game is one import cycle)
import { lessonGroup } from '../scene/scenarios';
import { kitPacks } from './kit';   // (called inside kitAsIssued only: src/game is one import cycle)

export { SQUAD_SIZES, asSquadSize, sizeTag, sizeWords, squadOfQuery, squadSearch, type SquadSize } from '../scene/squadsize';
import { asSquadSize, SQUAD_SIZES, type SquadSize } from '../scene/squadsize';

/** one teammate as a size stages him: his callsign and the pair he is dealt into */
export interface RosterMan { name: string; pair: PairId }
const PAIR: RosterMan[] = [{ name: 'Blue One', pair: 'blue' }, { name: 'Blue Two', pair: 'blue' }];
const FOUR: RosterMan[] = [{ name: 'Red One', pair: 'red' }, { name: 'Red Two', pair: 'red' }, { name: 'Blue One', pair: 'blue' }, { name: 'Blue Two', pair: 'blue' }];
/** THE ROSTER OF A SIZE, in staging order (member i + 1 of the loadout screen; the order T walks; KIT.members[i + 1]): up to two men are the pair as it always
 *  was ('Blue One', 'Blue Two'); three or four are the four-man squad's first three or all of it (nightbreach.ts SQUAD4: the red pair first). */
export function rosterOf(n: SquadSize): RosterMan[] { return (n > 2 ? FOUR : PAIR).slice(0, n); }

/** one size as a scenario can (or cannot) stage it */
export interface SquadChoice { n: SquadSize; ok: boolean;
  /** the scenario's own size: what its row loads with nothing changed */
  own: boolean;
  /** a line under the choice: why it is greyed (ok false), or what is different about it (ok true; '' = nothing to say) */
  note: string }
/** the kinds of mission the screen preps: a scenario's (scene/scenarios.ts ScenarioEntry.kind), or 'solo' — the plain night, which stages nobody */
export type LoadoutKind = 'nightbreach' | 'roomclear' | 'rescue' | 'quiet' | 'solo';
const SOLO_ONLY = 'The solo night is yours alone: nobody else is on the roster.';
const QUIET_ONLY = 'A quiet mission is for one or two: you, or you and one partner.';
/** a lesson's row (scenarioList.ts LESSON_MISSIONS: the tutorial's missions) is its own size and no other: its lines name that team's keys */
const LESSON_ONLY = 'A tutorial mission is played at its own size.';
/** …and with the kit AS ISSUED, whatever this machine keeps: its steps ask for the issue kit's own things, and a kept card without one would leave a step that can never pass.
 *  BUT NOT a lesson the player packs for (kit.ts ISSUE.picked, the row's `packs`: the tutorial's squad mission): its screen shows the cards and its page
 *  stages what this machine keeps. Its SIZE stays its own (squadChoices below reads the group), and its levers stay off (main.ts reads the group). */
export function kitAsIssued(e: { group?: string; packs?: unknown } | null | undefined): boolean { return lessonGroup(e?.group) && !kitPacks(e); }
/** WHICH SIZES A SCENARIO CAN STAGE — a pure function of its row (kind, own size, level), since the level itself is not loaded until Deploy. Every kind stages
 *  every size today (tools/qa/loadout-probes.ts boots each one and counts the men); the notes say what a size changes. `CANNOT` is where a size that truly
 *  cannot work on a scenario is greyed with its reason: keyed by entry id or by kind. A row that carries `only` (scenarios.ts ScenarioEntry: The two charges, The long hall)
 *  is offered at its own size alone, with that line on the others — a mission built for one size, as a Training row is; such a size still STAGES. */
export function squadChoices(e: { id?: string; kind: LoadoutKind; squad: number; level?: string; group?: string; only?: string }): SquadChoice[] {
  return SQUAD_SIZES.map(n => {
    const no = (e.id && CANNOT[e.id]?.[n]) || CANNOT[e.kind]?.[n] || (lessonGroup(e.group) && n !== e.squad ? LESSON_ONLY : '') || (e.only && n !== e.squad ? e.only : '') || (e.kind === 'solo' && n > 0 ? SOLO_ONLY : e.kind === 'quiet' && n > 1 ? QUIET_ONLY : '');
    return { n, ok: !no, own: n === e.squad, note: no || noteOf(e.kind, n, !e.level, e.squad) };
  });
}
/** sizes a scenario cannot stage, and why in a line (by entry id, or by kind) */
export const CANNOT: Record<string, Partial<Record<SquadSize, string>>> = {};
/** what a size changes on a kind of scenario (`office`: the bundled level, the one with a breach plan — F's drill) */
function noteOf(kind: LoadoutKind, n: SquadSize, office: boolean, own = 2): string {
  if (kind === 'solo') return soloFogRow() ? 'Alone on the office floor.' : 'Alone on the office floor: your own kit is all there is.';   // (under the fog the mission hands out its cameras and wand — quietfog.ts QUIETFOG.solo: the kit tiles show them)
  if (kind === 'quiet') return n === 0 ? 'Alone: your own kit, and nobody to cover a door.' : n === 1 ? 'One partner, silent from the start: he follows low in your steps and takes a man on your mark (X on the man, then F).' : '';
  if (n === 0) return kind === 'rescue' ? 'Alone: nobody to stack or flow with you. The clock is the same.' : 'Alone: no orders to give. Every room is yours to take.';
  if (n === 1) return kind === 'rescue' ? 'One man: he takes one door on your F; nobody for the side door.' : office ? 'One man: no F drill (it takes two). He takes your orders with X.' : 'One man: he takes your orders with X.';
  if (n === 3) return kind === 'rescue' ? 'Three: the red pair and one blue man.' : 'Three: the red pair and one blue man. Orders go to all, a pair or one man.';
  if (n === 4 && kind === 'rescue') return own === 4 ? 'Four: a red pair and a blue pair. A pair for each door.' : 'Four: a red pair and a blue pair. Nobody has balanced the rescue for four.';
  if (n === 2 && kind === 'rescue' && own === 4) return 'The pair: a man for each door. This floor was laid out for four.';
  return '';
}

// ───────────────────────────────────────────── what this machine picked last (the screen's memory, never a run's) ─────────────────────────────────────────────
type Store = Pick<Storage, 'getItem' | 'setItem'>;
