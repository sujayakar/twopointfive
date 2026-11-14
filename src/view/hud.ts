// WHAT A HUD IS HANDED: the game's record (`view`), the world asked (`ask`), and what only a page that draws can say: the clock between ticks, where it drew a man, and what the page itself tells the HUD. A page
// with no game makes one of these from what it draws. Types and one blank; nothing under them imports the game (tools/qa/view-check.ts).
import type { Vec3 } from '../math/vec';
import type { TickView } from './read';
import type { WorldAsk } from './worldask';

/** WHAT THE PAGE ITSELF TELLS THE HUD, none of it the game's */
export interface HudPage {
  /** the viewer's own picture has the fog lifted: every man it hides is shown, and none of its own marks */
  bared: boolean;
  /** the meters give the raw numbers too */
  debug: boolean;
  /** the word of the page's save controller, while it is up ('': none) */
  saved: string;
  /** a game of more players than one, and nobody is in the other seats */
  alone: boolean;
}
export const NO_PAGE: HudPage = { bared: false, debug: false, saved: '', alone: false };

export interface HudSource {
  /** what the game tells the seat this page shows */
  readonly view: TickView;
  readonly ask: WorldAsk;
  /** the picture's clock, between ticks */
  readonly time: number;
  /** a man's feet where he is drawn, by his character's id (null: nothing is drawn of him) */
  at(id: number): Vec3 | null;
  /** his head where it is drawn (null: no pose of him to say: the HUD goes by his feet) */
  head(id: number): Vec3 | null;
  readonly page: HudPage;
}

/** the camera as the HUD uses it: where a point of the floor falls on the screen */
export interface HudCam {
  readonly viewProj: ArrayLike<number>;
  /** the look-at's distance and the lens, which say how many metres a pixel covers there */
  readonly distance: number; readonly fov: number;
  /** x, y in the `w` by `h` layer, and whether it is in view: past the view's edge by the share `margin`, in front of the eye by `nearW` */
  project(p: Vec3, w: number, h: number, margin?: number, nearW?: number): [number, number, boolean];
}
