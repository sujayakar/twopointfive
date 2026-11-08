// THE GAME STOPPED: a panic in the rules loses the game of that call, and nothing it kept. The card says what happened and what Enter does; the session goes on.
import type { GameLost } from './drawgame';

export interface WayBack {
  /** what Enter would put back: a save's label, '' the run's start, null nothing (the page is loaded again) */
  kept(): string | null;
  /** it stands again (`newest` false: from the run's start): what did, or null; a throw = that way back failed */
  back(newest: boolean): string | null;
}

/** put up over the picture until one of its keys is pressed. `straight` = the save put back last led straight back here: it is not offered again */
export function lostCard(app: HTMLElement, e: GameLost, way: WayBack, straight: boolean): void {
  console.error(e.message);
  const save = straight ? '' : way.kept(), card = document.createElement('div');
  card.style.cssText = 'position:absolute;inset:0;z-index:19;display:flex;flex-direction:column;gap:14px;align-items:center;justify-content:center;padding:24px;background:rgba(12,11,9,0.82);color:#d2c9b5;text-align:center;';
  const row = (text: string, css = ''): void => { const d = document.createElement('div'); d.textContent = text; d.style.cssText = css; card.append(d); };
  row('The game stopped', 'font-size:26px;text-transform:uppercase;');
  row('A fault in the simulation it could not go on from. Nothing kept is lost.');
  row(`${e.words}\n(${e.what}, tick ${e.tick})`, 'max-width:90ch;font-size:11px;white-space:pre-wrap;word-break:break-word;user-select:text;opacity:0.8;');
  if (straight) row('The last checkpoint led straight back here.');
  row(save === null ? 'ENTER  load the page again' : save === '' ? 'ENTER  the mission from its start' : `ENTER  back to the last checkpoint (${save})  \u00b7  SHIFT+ENTER  the mission from its start`, 'font-size:12px;');
  const key = (ev: KeyboardEvent): void => {
    if (ev.code !== 'Enter' || ev.repeat) return;
    ev.preventDefault(); ev.stopImmediatePropagation();
    const newest = !!save && !ev.shiftKey;
    let stood: string | null = null;
    try { stood = way.back(newest); } catch (again) { removeEventListener('keydown', key, true); card.remove(); if (newest) lostCard(app, e, way, true); else { console.error(again); location.reload(); } return; }
    removeEventListener('keydown', key, true); card.remove();
    if (stood === null) location.reload();
  };
  addEventListener('keydown', key, true);
  app.append(card);
}
