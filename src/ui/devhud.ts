// THE DEVELOPER'S PICTURES OVER THE HUD, kept on the page that holds a game: one man's picture of the fight (ui/pictureviz.ts, an AI's beliefs) and the wind's field (ui/windviz.ts, its sources). Each reads what only
// the game's TypeScript holds and neither is a thing a player is shown, so neither is part of the HUD: a page with no game has neither.
import type { FollowCamera } from '../game/camera';
import type { Game } from '../game/game';
import type { View } from '../host/hud';
import { PictureViz } from './pictureviz';
import { textZoom } from './textsize';
import { WindViz } from './windviz';

const EDGE = 1.1;   // (as the HUD's marks: drawn until they are 10 % of the view past its edge)

export class DevHud {
  /** the wind field's debug view (Tab ▸ Weather & wind ▸ Wind — field view): a canvas of its own in #overlay, drawn only while one of its switches is on */
  readonly windViz: WindViz;
  private picture: PictureViz;
  /** `traj`: the HUD's floor layer: these go under everything else drawn there */
  constructor(root: HTMLElement, traj: SVGSVGElement) {
    this.windViz = new WindViz(root);
    this.picture = new PictureViz(traj, traj.firstChild as Element | null);
  }
  update(game: Game, cam: FollowCamera, view: View) {
    const tz = textZoom(), w = view.clientWidth / tz, h = view.clientHeight / tz;   // (the layer's own pixels: ui/textsize.ts)
    this.picture.update(game, cam, w, h, EDGE);
    this.windViz.update(game, cam, w, h);
  }
}
