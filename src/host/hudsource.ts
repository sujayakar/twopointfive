// THE PAGE THAT HOLDS A GAME, AS WHAT A HUD IS HANDED (view/hud.ts): the record its host keeps, the men as they are drawn, what the page says of itself. Its one place: so the HUD reads nothing else of the game.
import type { Character } from '../game/character';
import type { FollowCamera } from './camera';
import type { Game } from '../game/game';
import { CKPT } from '../game/savestate';
import { men } from '../game/seats';
import type { HudPage, HudSource } from '../view/hud';
import type { View } from './hud';
import type { LoopCtx } from './loop';

/** what this page tells the HUD of itself: the engine replay's lifted fog, the debug read-out, the tag of its save controller for as long as it shows */
function pageOf(g: Game): HudPage {
  const F = g.saves.flash, age = F ? g.time - F.t : -1;
  return { bared: g.fogOfWar.bared, debug: g.showDebug, saved: F && age >= 0 && age < CKPT.flashSecs ? F.text : '' };
}
interface Held extends HudSource { people: Map<number, Character>; page: HudPage }
const made = new WeakMap<Game, Held>();
/** THIS FRAME'S SOURCE: one a game, since what the HUD keeps by it (a way's kept legs) outlasts a frame and goes with the game; its people and the page's words as they are now */
export function hudSource(g: Game): HudSource {
  let S = made.get(g);
  if (!S) made.set(g, S = {
    get view() { return g.view; },
    get ask() { return g.ask; },
    get time() { return g.time; },
    people: new Map(),
    at(id) { return this.people.get(id)?.pos ?? null; },
    head(id) { return this.people.get(id)?.bones.head ?? null; },
    page: pageOf(g),
  });
  const P = S.people; P.clear();
  for (const m of g.civilians) P.set(m.char.id, m.char);
  for (const m of g.guards) P.set(m.char.id, m.char);
  for (const p of men(g)) P.set(p.char.id, p.char);
  S.page = pageOf(g);
  return S;
}
/** A FRAME'S HUD: the one way this page hands its game to what draws over the picture */
export function drawHud(g: Game, L: Pick<LoopCtx, 'hud' | 'bubbles' | 'dev'>, cam: FollowCamera, view: View): void {
  const src = hudSource(g);
  L.hud?.update(src, cam, view); L.bubbles?.update(src, cam, view); L.dev?.update(g, cam, view);
}
