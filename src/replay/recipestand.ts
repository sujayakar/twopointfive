// A GAME STOOD AS A RECIPE SAYS, ON THE PAGE. A recipe (format.ts RunTape.recipe; game/runtape.ts recipeOf) is a run's setup: the die, the page's address, the
// journal's knobs and env, every other knob table, the kit cards. Two things stand this page's game from one: the engine replay (engineplay.ts, a saved run's
// tape) and a networked game's client (src/net/page.ts, the recipe its server made) — a networked game is a run tape written live. The page is first sent to the
// recipe's own address with its seed, so the boot is that run's boot; then, through that boot, main.ts asks this file to lay the recipe over whatever this
// machine keeps: `tables` before the level is loaded, before and after the game is stood and round the mission's staging (each a plain overwrite), `env` before
// the game is built and before every step, `stood` once the game exists, `tune` at the first step. tools/qa/rerun.ts does the same headless: keep the two in step.
// Nothing here is read by the game.
import type { RunTape } from './format';
import type { Game } from '../game/game';
import * as J from '../game/journal';
import * as K from '../game/knobs';
import * as Kt from '../game/kit';
import { EXACT } from '../math/exact';
import { SENSE } from '../game/sense';
import { CMDCHECK } from '../game/cmd';
import { carLampsTape } from '../game/carlamps';
import { roundTape } from '../scene/round';
import { rowsetTape } from '../game/rowset';
import { memoryStore } from '../game/savestate';
import { resetCivilians } from '../game/civilians';

export type Plain = Record<string, unknown>;
export const plain = (v: unknown): v is Plain => !!v && typeof v === 'object' && !Array.isArray(v);
/** rerun.ts `put`: a recipe's table laid over the live one — only keys the table has, nulls written, deep */
export const put = (dst: Plain, src: Plain): void => { for (const k of Object.keys(src)) { if (k === '__proto__' || k === 'constructor' || k === 'prototype' || !(k in dst)) continue; const a = dst[k], b = src[k];
  if (plain(a) && plain(b)) put(a, b); else if (typeof a === typeof b || a === null || b === null) dst[k] = b !== null && typeof b === 'object' ? JSON.parse(JSON.stringify(b)) : b; } };

/** THE SANDBOX FLAGS A STEP RUNS UNDER, written to the game (a recipe's, a tape's later row's) */
export function layFlags(g: Game, fl: Plain): void { g.godMode = !!fl.godMode; g.aiEnabled = fl.aiEnabled !== false; g.infiniteAmmo = !!fl.infiniteAmmo; g.playerInvisible = !!fl.playerInvisible; }

export class RecipeStand {
  constructor(readonly recipe: RunTape['recipe']) {}

  /** the recipe's knob tables and kit cards over this machine's (rerun.ts `tables` / `kitOf`) */
  tables(): void {
    const rc = this.recipe;
    const kn = rc.knobs as { tables?: Plain } | null, src: Plain = { ...(kn?.tables ?? {}), ...(rc.tables ?? {}) }, into = (rc.tables ? K.knobsPlayed() : J.knobTables()) as Record<string, Plain>;
    for (const [name, Tb] of Object.entries(into)) { const t = src[name]; if (plain(t)) { if (rc.tables) put(Tb, t); else K.assignDeep(Tb, t); } }
    if (!plain(src.EXACT)) EXACT.on = false;   // (a tape from before the game's own maths was recorded on the engine's functions: said here, BEFORE the level and the game are built on them — math/exact.ts)
    if (!plain(src.SENSE)) SENSE.cpu = false;  // (… and one from before these keys was played on the renderer's senses, its records believed as they came)
    if (!plain(src.CMDCHECK)) CMDCHECK.on = false;
    rowsetTape(src);   // (… and one from before a row had an hour and weather of its own: main.ts lays them on its query before the game exists — game/rowset.ts)
    roundTape(src);   // (… and one from before round things were round: the level's boxes are read as it is stood up — scene/round.ts)
    carLampsTape(src);   // (… and one from before a vehicle's lamps were light targets: the Game reads that switch as it is built — game/carlamps.ts)
    this.kit(rc.kit);
  }
  /** the kit cards (the recipe's, or a later row of a tape's) */
  kit(k: unknown): void {
    const c = k as { on?: boolean; members?: unknown[]; own?: unknown; stealth?: unknown } | undefined; if (!c || !Array.isArray(c.members)) return;
    Kt.kitMission(null);   // (kit.ts KITDEAL.stealth: FIRST — a stealth card standing first in the table goes back to its own place before the tape's cards are written, or this machine's loud card would land over the tape's)
    Kt.kitOwn(c.own === true); Kt.KIT.on = c.on !== false; const cards = Array.from({ length: Kt.KIT_MEMBERS }, (_, i) => Kt.kitSanitize(i, c.members![i])); Kt.KIT.members.length = 0; Kt.KIT.members.push(...cards); Kt.kitStealthSet(c.stealth);   // (…and the stealth card the tape's run wore, or none — never this machine's)
  }
  /** what the simulation reads off the ENGINE, as the recording's engine had it (journal.ts EnvSnap) — before the game is built, and again every step (a quality preset rewrites the debris cap) */
  env(settings: Record<string, unknown>): void { const e = this.recipe.env; if (e) J.applyEnvSettings(settings as never, e as never); }
  /** the game has just been stood: its tune, flags and tables, the fixtures' own stagger */
  stood(g: Game): void {
    const rc = this.recipe, civ = !!g.tune.civilians;
    if (rc.knobs) J.applyKnobs(g, rc.knobs as never); this.tables(); if (rc.env) J.applyEnvGame(g, rc.env as never);
    if (!!g.tune.civilians !== civ) resetCivilians(g);   // (the game was built on this machine's kept switch: the floor's civilians are the recipe's switch's)
    g.saves.store = memoryStore();   // (such a run's autosaves are its own: this browser's checkpoints are not written over)
  }
  /** the recipe's sandbox flags over the game's: before every step of a host whose page could have flipped one since (net/page.ts) */
  flags(g: Game): void { const fl = (this.recipe.knobs as { flags?: Plain } | null)?.flags; if (fl) layFlags(g, fl); }
  /** the recipe's tune over the game's, at the first step: the last word over what the boot wrote after the staging (this machine's sandbox row, its kept feel rows) */
  tune(g: Game): void { const kn = this.recipe.knobs as { tune?: Plain } | null; if (kn?.tune) K.assignDeep(g.tune as never, kn.tune); }
}
