// THE VIEW: what the HUD is told of the game beside what it draws. It is the Rust game's answer (rust/game_rules/src/view `tick_view`), asked by the host that
// steps the game and read here by name: `game.view.mission.text`. Nothing in this folder works anything out, and nothing of it is kept by the game: a host
// asks again after a tick (or after anything else that moved the Rust game) and hands over the new record.
//
// Its types are the Rust structs' (rust/game_types/src/view), and read.ts, written from them, reads their bytes: no table and no book at run time. A game no
// host steps (a workbench's floor, the moment before the first start) has the blank one.
//
// NOTHING UNDER src/view IMPORTS THE GAME (src/game, src/sim, src/scene, src/anim, the bridge): a page that holds no game reads the same record
// (tools/qa/view-check.ts). So a man is named by his character's id, a door by its name, and never by an object of the page's.
import type { TickView } from './read';
export type { MissionOf, MissionView, TickView } from './read';

/** what a game no host steps is told */
export const NO_VIEW: TickView = { mission: null };
