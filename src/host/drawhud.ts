// THE HUD of a page that draws a game it does not hold: what the module tells the shown seat (view/read.ts), laid over the men as the rows place them; Tab's card; and what the save controller says.
import { Section, View } from '../draw/sections';
import type { FollowCamera } from './camera';
import { Bubbles } from '../ui/bubbles';
import { Overlay } from '../ui/overlay';
import { tabCardHtml, tabCardOf } from '../ui/tabcard';
import { NO_PAGE, type HudPage, type HudSource } from '../view/hud';
import { pageSay } from '../view/pagesaid';
import { NO_VIEW } from '../view/seatview';
import type { WorldAsk } from '../view/worldask';
import type { DrawGame } from './drawgame';
import type { ViewSize } from './viewsize';

export class DrawHud {
  readonly overlay = new Overlay();
  private bubbles: Bubbles;
  private tab = document.getElementById('tabcard')!;
  readonly page: HudPage = { ...NO_PAGE };
  readonly source: HudSource;
  /** nothing of the game is said (the title, over a floor nobody plays) */
  blank = false;
  /** Tab's card is up: the world holds still under it */
  tabUp = false;
  constructor(private game: DrawGame, ask: WorldAsk, canvas: HTMLCanvasElement) {
    const hud = this; this.bubbles = new Bubbles(document.getElementById('overlay')!, canvas);
    this.source = { get view() { return hud.blank ? NO_VIEW : game.view(); }, ask, get time() { return game.time; }, at: id => game.at(id), head: id => game.head(id), page: this.page };
  }
  private word(of: View): number { const D = this.game.drawn; return D.u[D.at[Section.view] + of]; }
  /** a card of the HUD's holds the world still: a lesson's, or Tab's */
  get holds(): boolean { return this.overlay.cardUp || this.tabUp; }
  /** Tab, on a frame nothing else holds still (`live`): the card up, or away. False = the press is the game's: it has something the back key puts away, in the tick */
  tabKey(live: boolean): boolean {
    if (this.tabUp) { this.tabShow(false); return true; }
    if (!live || this.word(View.backs)) return false;
    this.tabShow(true); return true;
  }
  tabShow(on: boolean): void {
    if (on === this.tabUp) return;
    this.tabUp = on; this.tab.classList.toggle('on', on); if (on) this.tab.innerHTML = tabCardHtml(tabCardOf(this.game.view()));   // (laid once: the game is held under it)
  }
  /** after a tick, and after anything done between two */
  told(): void { const t = this.game.savesTold(); this.page.saved = t.tag; this.page.debug = this.word(View.labels) !== 0; for (const line of t.lines) pageSay(line); }
  /** over the picture as it has just been drawn, through the camera it was drawn by */
  draw(cam: FollowCamera, size: ViewSize): void { this.overlay.update(this.source, cam, size); this.bubbles.update(this.source, cam, size); }
}
