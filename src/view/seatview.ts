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
export interface TickView { /** null: a show runs */ mission: MissionView | null }

/** what a game no host steps is told */
export const NO_VIEW: TickView = { mission: null };
