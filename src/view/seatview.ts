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
export type { BubbleView, EmptyView, HandRingView, KitGun, KitSetup, KitSlot, KitThing, KitView, LessonView, ManMark, MateLine, MissionOf, MissionView, Note, Pair, PlanView, RadioRow, RailCard, RailPrompt, RailsView, RingView, SquadView, ThrowLine, TickView } from './read';

/** what a game no host steps is told */
export const NO_VIEW: TickView = { mission: null, lesson: null, radio: [], bubbles: [], me: -1, place: '',
  kit: { slots: [], gun: null, setup: null, big: null, hitsLeft: 0, hitsMax: 0, down: false, god: false, nightVision: null, keyTag: null },
  marks: { men: [], seen: [], cams: [], frags: [], blind: 0, light: 0, irradiance: 0, noise: 0, stalk: null, stalkWaits: false, floor: null },
  aim: { ring: null, dot: null, tooClose: false, throw: null },
  worn: [], squad: null, corner: [], throws: [], ring: null, hand: null, empty: null, plan: { own: { men: [], who: [], you: null, doors: [], throws: [], mode: false }, others: [] }, live: false, quiet: null,
  interact: { rows: [], lit: -1, more: 0, open: false, corner: true, ring: null, dots: [] }, alarm: null, stage: 'infiltrate', attention: [],
  look: { ring: { places: 0, hole: 0, rim: 0, boxX: 0, boxY: 0, lone: { r: 0, h: 0, pad: 0, letter: 0, max: 0, lead: 0 } }, plan: { freshSecs: 0, walkDim: 0, restDim: 0, ringM: 0, sectorM: 0, sectorDeg: 0, reaskSecs: 0, others: 0 }, net: { edge: false, near: 0, edgeIn: 0 }, attention: { ringM: 0, wayReask: 0 }, lineClear: false, markClear: false, back: 'esc' } };


/** is this action shut on the mission that is up: the press does nothing, and the HUD names no key of it (nothing is locked off a tutorial mission) */
export function locked(v: TickView, action: string): boolean { const R = v.lesson?.rails; return !!R && !R.open.includes(action); }
/** does the interaction key offer rows of this verb (every verb off a tutorial mission) */
export function offered(v: TickView, verb: string): boolean { const R = v.lesson?.rails?.rows; return !R || R.includes(verb); }
