// THE VIEW: what the HUD is told of the game beside what it draws. It is the Rust game's answer (rust/game_rules/src/view `tick_view`), asked by the host that
// steps the game and read here by name: `game.view.mission.text`. Nothing in this folder works anything out, and nothing of it is kept by the game: a host
// asks again after a tick (or after anything else that moved the Rust game) and hands over the new record.
//
// Its types are the Rust structs' (rust/game_types/src/view), and read.ts, written from them, reads their bytes: no table and no book at run time. A game no
// host steps (a workbench's floor, the moment before the first start) has the blank one.
//
// NOTHING UNDER src/view IMPORTS THE GAME (src/game, src/sim, src/scene, src/anim, the bridge): a page that holds no game reads the same record
// (tools/qa/view-check.ts). So a man is named by his character's id, a door by its name, and never by an object of the page's.
import type { TickView, WornView } from './read';
export type { BubbleView, KitGun, KitSetup, KitSlot, KitThing, KitView, ManMark, MissionOf, MissionView, RadioRow, TickView } from './read';

/** what a game no host steps is told */
export const NO_VIEW: TickView = { mission: null, radio: [], bubbles: [], me: -1, place: '',
  kit: { slots: [], gun: null, setup: null, takes: [], big: null, hitsLeft: 0, hitsMax: 0, down: false, god: false, nightVision: null, keyTag: null },
  marks: { men: [], seen: [], cams: [], frags: [], blind: 0, light: 0, irradiance: 0, noise: 0, stalk: null, stalkWaits: false },
  aim: { ring: null, dot: null, tooClose: false, throw: null },
  worn: [] };

/** what this man wears or carries that the drawer draws */
export function wornBy(v: TickView, man: number): WornView | undefined { return v.worn.find(w => w.man === man); }

/** the fog hides this man from the seat's team */
export function fogHides(v: TickView, man: number): boolean { return v.marks.men.some(m => m.man === man && m.hidden); }
