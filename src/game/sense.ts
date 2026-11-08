// THE GAME SENSES FOR ITSELF: how lit a point is and how much smoke is on a line are worked out by the simulation, the same on a page, a server and a bare
// machine (rust/game_rules/src/sense, guards; rust/sim/src/bricks.rs, glow.rs, light.rs). The picture is asked nothing. What is here is the table of the
// model's numbers, and the page's and the tools' one way of asking the Rust game.
// (src/game is one import cycle: nothing here is built from another game module's export as the file loads.)
import type { Game } from './game';
import { crossToRust } from './crossing';

export const SENSE = {
  /** the light under a roof (glow.rs): the steps a sweep of the whole plan takes (a step is a tenth of a second) */
  glow: { sweep: 32 },
  smoke: {
    /** extinction per metre per unit of density (the renderer's: σt = 6/m per unit) */
    sigma: 6,
    /** what the smoke loses a second (the solver's densDecay) */
    decay: 0.14,
    /** how fast the smoke spreads (m²/s) */
    spread: 0.2,
    /** how far smoke rises above where it was let out (m), and how fast (m/s) */
    rise: 1, riseRate: 0.3,
    /** how much of its own spread SIDEWAYS the smoke keeps where the wind blows whole (bricks.rs step 4; upward it keeps none: the wind is flat). With none of it
     *  the smoke cannot get round what the wind pushes it against, and a plume never widens */
    inWind: 0.15,
    /** an emitter that lets out less than this in its whole life (density × volume, unit·m³) is the picture's alone: a muzzle's wisp, a spark's trail hide nobody */
    least: 0.05,
    /** a brick thinner than this (unit·m³) is gone */
    gone: 0.02,
    /** how many of the emitters a game holds it keeps back from `budget` */
    reserve: 12,
  },
};

/** a page's way to the Rust game by a call's name (host/rustgame.ts rustCall), handed in once by main.ts; null = no page: the call surface carries it */
export const SENSE_PAGE: { call: ((key: string, self: unknown, args: unknown[]) => unknown) | null } = { call: null };
/** ASKING THE SENSES FROM OUTSIDE A TICK: 0 the light on points (`xs`: 1 for what the place lights with nothing that passes, the level's lamps left out by what feeds
 *  them (sim/lightshome.ts ROW_BIT), how many owners follow, the owners whose carried lamps are left out, then x y z each) · 1 how much of the light along lines the
 *  smoke lets through (six numbers each) · 2 a canister's smoke let out at x y z, to the game and the picture alike · 3 the smoke cleared, the same. With no Rust
 *  game under it, it throws by name. */
export function senseTool(g: Game, what: 0 | 1 | 2 | 3, xs: number[]): number[] { return (SENSE_PAGE.call ?? crossToRust)('senseTool', undefined, [g, what, xs]) as number[]; }
