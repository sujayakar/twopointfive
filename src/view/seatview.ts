// THE VIEW: what the HUD is told of the game beside what it draws. It is the Rust game's answer (rust/game_rules/src/view `tick_view`), asked by the host that
// steps the game and read here by name: `game.view.mission.text`. Nothing in this file works anything out, and nothing of it is kept by the game: a host
// asks again after a tick (or after anything else that moved the Rust game) and hands over the new record.
//
// Each section's fields are the Rust structs' (game_types view/types.rs), by the bridge's names. A game no host steps (a workbench's floor, the moment before
// the first start) has the blank one.
//
// NOTHING UNDER src/view IMPORTS THE GAME (src/game, src/sim, src/scene, src/anim, the bridge): a page that holds no game reads the same record
// (tools/qa/view-check.ts). So a man is named by his character's id, a door by its name, and never by an object of the page's.
import type { Vec3 } from '../math/vec';

/** whose words the mission's line is */
export type MissionOf = 'stages' | 'script' | 'lesson' | 'defuse' | 'snatch' | 'roomclear' | 'rescue' | 'nightbreach';
/** the mission's one line and the place its marker stands on */
export interface MissionView { of: MissionOf; text: string; pos: Vec3 | null }
/** a tutorial mission's one big line (`n` of `of`: its pips · `note`: the few words under it after a tap the step refused · `watch`: it asks for nothing, so no mark to press on) */
export interface RailPrompt { line: string; n: number; of: number; note: string | null; watch: boolean }
/** a tutorial step's card */
export interface RailCard { title: string | null; text: string }
/** what a tutorial mission's frame lets the player do and tells him, for the step that is up */
export interface RailsView {
  /** null: the run is over */ prompt: RailPrompt | null; card: RailCard | null;
  /** the place the step marks: a cursor on it is on the place, whatever stands between */ mark: Vec3 | null;
  /** the mark is an arrow, pointing here */ arrow: Vec3 | null;
  /** the player's own men on the floor, by character id: the big line keeps clear of them */ lineMen: number[];
  /** the actions that work now. An action not here is locked (`locked`, below) */ open: string[];
  /** the verbs the interaction key offers rows of (null: every row) */ rows: string[] | null;
  /** the only orders on the squad's menu (null: every order) */ orders: string[] | null;
  /** a step has taken the held click off him */ stalkShut: boolean;
  /** the step teaches the squad's key held: its tap is not offered */ teachesHold: boolean;
  /** no teammate shows an aiming laser */ noLasers: boolean;
  /** nobody on the floor is the player: where the view looks (null: he is on the floor) */ sky: Vec3 | null;
}
/** the lesson that is live on the mission that is up */
export interface LessonView {
  /** the step that is up, from 0, of how many */ step: number; of: number;
  /** its id (null: every step is done) */ id: string | null; finished: boolean;
  /** the run is over, whichever way */ over: boolean;
  /** the step's deed is done and the moment is playing out */ settling: boolean;
  /** the step asks him to move the view: nothing else moves it meanwhile */ asksView: boolean;
  /** null: a lesson with no frame over it */ rails: RailsView | null;
}
export interface TickView { /** null: a show runs */ mission: MissionView | null; /** null: no lesson is live */ lesson: LessonView | null }

/** what a game no host steps is told */
export const NO_VIEW: TickView = { mission: null, lesson: null };

/** is this action shut on the mission that is up: the press does nothing, and the HUD names no key of it (nothing is locked off a tutorial mission) */
export function locked(v: TickView, action: string): boolean { const R = v.lesson?.rails; return !!R && !R.open.includes(action); }
/** does the interaction key offer rows of this verb (every verb off a tutorial mission) */
export function offered(v: TickView, verb: string): boolean { const R = v.lesson?.rails?.rows; return !R || R.includes(verb); }
