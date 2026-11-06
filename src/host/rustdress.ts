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
// THE FRAME'S LIST. The Rust tick makes the frame's boxes and the fog of war cuts them last; the apply hands that list to the box world. The boxes only
// the picture has (the moths, a set wall charge's look, a broken fixture's pieces) are made here on a side list, passed through a small filter of their
// own, and appended. The fog of war's own pass cannot be run over a part of the list: its tail forgets every keyed thing it was not shown.
import type { Game } from '../game/game';
import type { Box } from '../scene/boxes';
import type { Character } from '../game/character';
import { DRESS } from '../game/dress';
import { updateAirSim } from '../game/airSim';
import { updateRainFx } from '../game/rainFx';
import { fireDraws } from '../game/fire';
import { statureTick, drawnBoxes } from '../game/stature';
import { updateParticleFx } from '../game/particleFx';
import { blastLookTick } from '../game/blastLook';
import { blastFeelTick } from '../game/blastFeel';
import { fixtureBreakBoxes } from '../game/fixtureBreak';
import { updateSnowBreath } from '../game/snowBreath';
import { recapTick } from '../game/deathrecap';
import { civiliansShown } from '../game/civilians';

/** ms of the last dress step by line, and the lines that threw (each named once: a picture-only line that throws must not stop the game) */
export const DRESS_MS = { last: {} as Record<string, number>, threw: {} as Record<string, string> };
export interface SideList { list: unknown[]; of: unknown; base: number }

export function dressStep(g: Game, dt: number, side: SideList): void {
  const G = g as unknown as Record<string, any>, S = g.seats, me = g.hostSeat(), T = DRESS_MS.last;
  const run = (name: string, f: () => void) => { const t = performance.now(); try { f(); } catch (e) { DRESS_MS.threw[name] ??= String((e as Error)?.stack ?? e).slice(0, 700); } T[name] = performance.now() - t; };
  // ---- stepSeats, after the rules (game.ts: the lines from `if (DRESS.wind)` down)
  if (DRESS.wind) run('wind', () => g.wind.update(dt));
  if (DRESS.air) run('air', () => updateAirSim(g, dt));
  if (DRESS.moths) run('moths', () => G.moths.update(dt));
  if (DRESS.snow) run('snowLook', () => G.syncSnowLook());
  if (DRESS.rain) run('rain', () => updateRainFx(g, dt, me.cam));
  run('wetGround', () => G.wetGround.follow(g));
  if (DRESS.fires) run('fires', () => fireDraws(g));
  const all: Character[] = [...S.map(s => s.view.player.char), ...g.guards.map(x => x.char), ...civiliansShown(g).map(c => c.char)];
  // ---- stepFxAndDynBoxes: the picture's own boxes go to the side list
  const out: Box[] = [];
  if (DRESS.moths) run('mothBoxes', () => G.moths.boxes(out));
  let statured = false;
  if (DRESS.stature) run('stature', () => { statured = statureTick(g, all, dt); });
  // (his rig and a can in his fist are drawn where his drawn hands are: Box.dc / dh of the boxes the Rust tick made, from where the men's begin)
  if (statured) run('drawnBoxes', () => drawnBoxes(g, all, g.dynBoxes, g.fogOfWar.rigFrom));
  if (DRESS.cases) run('cases', () => updateParticleFx(g));
  run('blastLook', () => { for (const s of S) blastLookTick(s.view as unknown as Game, out); });
  if (DRESS.blastFeel) run('blastFeel', () => { for (const s of S) blastFeelTick(s.view as unknown as Game, s.cam); });
  if (DRESS.fixtures) run('fixtures', () => fixtureBreakBoxes(g, out, dt));
  // ---- back in stepSeats
  if (DRESS.wake) run('wake', () => G.smokeWake(all));
  // (drawCharacters: the men handed to the renderer. The Rust tick deals what each wears and stops there, as the tick does with the `men` row down)
  run('men', () => { G.menAll = all; G.menStatured = statured; G.menBare = false; G.drawMen(all, statured, false, DRESS.men); });
  run('snowBreath', () => updateSnowBreath(g, dt));
  if (DRESS.snow) run('snowTracks', () => G.stepSnowTracks(dt));
  if (DRESS.net) run('net', () => G.squadNet.tick());
  if (DRESS.cues) run('cues', () => G.cues.tick());
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
