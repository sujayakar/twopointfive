// THE DRESS STEP: what the TypeScript tick does FOR THE PICTURE ONLY, run on the puppet after each apply of the Rust game's state (src/host/rustgame.ts).
// The Rust tick runs every rule; the lines below are the ones of `Game.stepSeats`, `stepFxAndDynBoxes` and `drawCharacters` (src/game/game.ts) that it
// leaves out because they decide nothing: the rows of DRESS (src/game/dress.ts) and the picture-only calls beside them, in the tick's own order. They read
// the puppet as the apply left it, which is the end of the tick and not the moment each stood at inside it: measured on three missions, what a dress line
// reads that the tick then changes is the wind's gust, a box the air's obstacle map reads, and a man's place for the par clock, each one tick newer and
// none wrong.
//
// NOT HERE, because the Rust tick does them and their answer comes back in the apply or as out events: the lamps' paint (updateTargets), the doors',
// debris', devices', rings', sticky cameras', switches' and shells' boxes, what the men carry (handProps), metres per pixel, the fog of war's cut of the
// frame's list, and the WHOLE loop over `Game.fx` (the short-lived boxes: pushed, aged and removed there).
//
// AND NOT HERE ANY MORE, because the Rust tick runs these bodies under their dressing rows and a second run on the puppet would do each twice: the
// moths (their step and their boxes), the drawn places of a man's rig (`drawnBoxes`), the spent cases' sounds, the squad's net, the cues, a broken
// fixture's pieces, a set charge's look and its blast's (its boxes, its flash's colour and the fragments of the burst are all the Rust tick's:
// blast_look), the report and the camera's kick of a wall charge (its particles are started by the host on the report's out event and their
// later stages let go here: `blastFeelStages`), and the wind's own step and its two lists: only the hand-over of those lists to the particles and the smoke is made here (`windOut`).
// STILL HERE though the Rust tick has a body of the same name: `statureTick`, because the TypeScript's eased scale of each man lives in a table of
// stature.ts that the bridge does not carry and `drawMen` reads it.
//
// THE FRAME'S LIST. The Rust tick makes the frame's boxes and the fog of war cuts them last; the apply hands that list to the box world. A box only
// the picture has would be made here on a side list, passed through a small filter of its own, and appended; today no line here makes one (the moths,
// a set charge's look and a fixture's pieces are the Rust tick's), and the list's code stays for the next that does. The fog of war's own pass cannot be run over a part of the list: its tail forgets every keyed thing it was not shown.
import type { Game } from '../game/game';
import type { Box } from '../scene/boxes';
import type { Character } from '../game/character';
import { DRESS } from '../game/dress';
import { updateAirSim } from '../game/airSim';
import { updateRainFx } from '../game/rainFx';
import { fireDraws } from '../game/fire';
import { statureTick } from '../game/stature';
import { gpuFx } from '../game/particleFx';
import { WIND } from '../game/wind';
import { blastFeelStages } from '../game/blastFeel';
import { updateSnowBreath } from '../game/snowBreath';
import { recapTick } from '../game/deathrecap';
import { civiliansShown } from '../game/civilians';

/** ms of the last dress step by line, and the lines that threw (each named once: a picture-only line that throws must not stop the game) */
export const DRESS_MS = { last: {} as Record<string, number>, sum: {} as Record<string, number>, n: 0, threw: {} as Record<string, string> };
export interface SideList { list: unknown[]; of: unknown; base: number }

/** THE WIND'S HAND-OVER (wind.ts `update`, its last lines): the sources that blow go to the smoke's solver, and the ones the tick picked for the
 *  particles (the jets near the view, then the storm) to the particle pool. Both lists are the Rust tick's, as the host put them on the puppet
 *  (the filtered export); nothing is chosen here. */
function windOut(g: Game): void {
  const W = g.wind as unknown as { active: unknown[]; forParticles: unknown[] }, P = gpuFx(g), Sm = g.smoke as unknown as { setWind?: (a: unknown[], relax: number) => void };
  P?.setWind(W.forParticles as never, [WIND.sheetWake, WIND.sheetKick, WIND.debrisK]);
  Sm.setWind?.(W.active, WIND.smokeRelax);
}

export function dressStep(g: Game, dt: number, side: SideList): void {
  const G = g as unknown as Record<string, any>, S = g.seats, me = g.hostSeat(), T = DRESS_MS.last;
  const M = DRESS_MS.sum; DRESS_MS.n++;
  const run = (name: string, f: () => void) => { const t = performance.now(); try { f(); } catch (e) { DRESS_MS.threw[name] ??= String((e as Error)?.stack ?? e).slice(0, 700); } const ms = performance.now() - t; T[name] = ms; M[name] = (M[name] ?? 0) + ms; };
  // ---- stepSeats, after the rules (game.ts: the lines from `if (DRESS.wind)` down)
  if (DRESS.wind) run('wind', () => windOut(g));
  if (DRESS.air) run('air', () => updateAirSim(g, dt));
  if (DRESS.snow) run('snowLook', () => G.syncSnowLook());
  if (DRESS.rain) run('rain', () => updateRainFx(g, dt, me.cam));
  run('wetGround', () => G.wetGround.follow(g));
  if (DRESS.fires) run('fires', () => fireDraws(g));
  const all: Character[] = [...S.map(s => s.view.player.char), ...g.guards.map(x => x.char), ...civiliansShown(g).map(c => c.char)];
  // ---- stepFxAndDynBoxes: the picture's own boxes go to the side list
  const out: Box[] = [];
  let statured = false;
  if (DRESS.stature) run('stature', () => { statured = statureTick(g, all, dt); });
  // (the wall charge's feel: the Rust tick made the report and the kick; the charges that stand are remembered here for the next blast and the queued
  //  stages of the last one, its ring, soot and ceiling dust, are let go: the particles' half, started by the host on the blast's report)
  if (DRESS.blastFeel) run('blastFeel', () => { for (const s of S) blastFeelStages(s.view as unknown as Game); });
  // ---- back in stepSeats
  if (DRESS.wake) run('wake', () => G.smokeWake(all));
  // (drawCharacters: the men handed to the renderer. The Rust tick deals what each wears and stops there, as the tick does with the `men` row down)
  run('men', () => { G.menAll = all; G.menStatured = statured; G.menBare = false; G.drawMen(all, statured, false, DRESS.men); });
  run('snowBreath', () => updateSnowBreath(g, dt));
  if (DRESS.snow) run('snowTracks', () => G.stepSnowTracks(dt));
  // THE PAR CLOCK AND THE VERDICT stay TypeScript: they read the game and write only their own record
  run('par', () => G.par?.tick());
  run('verdict', () => G.verdict?.tick());
  if (DRESS.recap) run('recap', () => recapTick(g));
  // ---- the side list onto the frame's list, past a filter of its own: with the fog of war off everything; else a box with no thing's key where the team
  // sees its cell, and a keyed one as it is (the moths). STAND-IN: a fixture's piece is treated as unkeyed, so one that fell unseen is not kept where it
  // was last seen. Belongs in the Rust tick (these boxes made there, before its own fog pass), with WallSkin's boxes, which nothing here makes.
  run('side', () => {
    const W = G.engine.world, dyn: Box[] = W.dynamics, fog = g.fogOfWar as unknown as Record<string, any>;
    if (side.of === dyn && dyn.length === side.base + side.list.length) dyn.length = side.base;   // (the list was not dealt again this step: last step's side boxes come off first)
    const pass = (b: Box): boolean => !fog.on || (b as { mkey?: number }).mkey !== undefined || !!fog.cellSeenAt?.(b.c[0], b.c[2]);
    const kept = out.filter(pass);
    side.of = dyn; side.base = dyn.length; side.list = kept;
    for (const b of kept) dyn.push(b);
  });
}
