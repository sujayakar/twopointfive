// A RUN'S SAVES ON A CARD, for whichever page holds the run: the button that goes back to the newest, and the strip of them all. What a card asks of the run is the
// interface here; nothing below knows the game.
import { pageSay } from '../view/pagesaid';

export interface SaveRow { key: string; slot: 'auto' | 'manual'; label: string; missionT: number; savedAt: number; regret?: string | null }
export interface RunSaves {
  /** the mission's clock */
  elapsed(): number;
  /** this run's saves in the mission clock's order, oldest first */
  timeline(): SaveRow[];
  /** the one the button goes back to, or none */
  offer(): SaveRow | null;
  /** how long ago, in words */
  age(row: SaveRow): string;
  /** its picture (a data URL), or none */
  picture(row: SaveRow): string | null;
  /** go back to it: why not, or null */
  load(key: string): string | null;
  /** a save has been asked for on the card that is up, and is written as play goes on */
  due(): boolean;
}
/** what a press does besides: `armed` = may it yet (a card that comes up under a press), `gone` = the save stands again, `again` = it was refused and the card is laid again */
export interface Press { armed?: () => boolean; gone: () => void; again: () => void }

function back(run: RunSaves, key: string, p: Press): void {
  if (p.armed && !p.armed()) return;
  const why = run.load(key); if (why) { pageSay(`checkpoint: ${why}`); p.again(); return; }
  p.gone();
}
/** 'Restart from checkpoint (40 s ago)', or null with none to offer */
export function checkpointButton(run: RunSaves, p: Press, cls = ''): HTMLElement | null {
  const info = run.offer(); if (!info) return null;
  return button(`Restart from checkpoint (${run.age(info)})`, () => back(run, info.key, p), cls);
}
/** THE STRIP: this run's saves in the order they were made, each a tile — its picture, how long ago, a mark on the ones made by hand, a regretted autosave dimmed. A
 *  click, or Enter on the tile the arrows (A D too) have walked to, goes back to it. Null with fewer than two (one save is the button above it) unless one is asked for on this card. */
export function saveStrip(run: RunSaves, p: Press): HTMLElement | null {
  const due = run.due(), line = run.timeline(); if (line.length < 2 && !due) return null;
  const now = run.elapsed(), offered = run.offer();
  const strip = el('div', 'saves'); strip.setAttribute('role', 'group'); strip.setAttribute('aria-label', 'saves of this run');
  const tiles = line.map(info => {
    const t = el('button', 'tile' + (info.slot === 'manual' ? ' hand' : '') + (info.regret ? ' regret' : '') + (info.missionT > now + 0.5 ? ' ahead' : '')) as HTMLButtonElement; t.type = 'button';
    const pic = run.picture(info); if (pic) t.style.backgroundImage = `url("${pic}")`; t.dataset.save = `${info.key}@${info.savedAt}`;
    const age = el('span', 'age'); age.textContent = tileAge(now - info.missionT); t.appendChild(age);
    if (info.slot === 'manual') t.appendChild(el('i'));
    t.title = info.label || (info.slot === 'manual' ? 'by hand' : 'autosave'); t.setAttribute('aria-label', `${info.slot === 'manual' ? 'saved by hand' : 'autosave'}, ${run.age(info)}`);
    t.tabIndex = -1; t.onclick = () => back(run, info.key, p);
    strip.appendChild(t); return t;
  });
  if (due) { const t = el('div', 'tile hand due'); t.appendChild(el('i')); t.setAttribute('aria-label', 'a save by hand, written as play goes on'); strip.insertBefore(t, tiles[line.findIndex(x => x.missionT > now + 0.5)] ?? null); }   // (before the saves a load has left ahead of this moment)
  if (!tiles.length) return strip;
  let cur = Math.max(0, offered ? line.findIndex(x => x.key === offered.key) : line.length - 1); tiles[cur].tabIndex = 0;   // (one stop for W S and Tab: the tile the button above would load)
  strip.addEventListener('keydown', e => {
    const d = e.code === 'ArrowLeft' || e.code === 'KeyA' ? -1 : e.code === 'ArrowRight' || e.code === 'KeyD' ? 1 : 0; if (!d || e.ctrlKey || e.metaKey || e.altKey) return;
    tiles[cur].tabIndex = -1; cur = Math.max(0, Math.min(tiles.length - 1, cur + d)); tiles[cur].tabIndex = 0; tiles[cur].focus(); e.preventDefault(); e.stopPropagation();
  });
  return strip;
}

/** a save's age on its tile: '40 s', '3 min' — '+40 s' for one AHEAD of the clock (a later save of this run, after going back past it) */
function tileAge(d: number): string { const a = Math.abs(d), w = a < 90 ? `${Math.round(a)} s` : `${Math.round(a / 60)} min`; return d < -0.5 ? `+${w}` : w; }
/** how long ago on the mission's clock, in the words of a card */
export const agoWords = (secs: number): string => { const d = Math.max(0, Math.round(secs)); return d < 90 ? `${d} s ago` : `${Math.round(d / 60)} min ago`; };
export function el(tag: string, cls?: string) { const e = document.createElement(tag); if (cls) e.className = cls; return e; }
export function button(text: string, run: () => void, cls = '') { const b = el('button', 'btn ' + cls) as HTMLButtonElement; b.textContent = text; b.onclick = run; return b; }

// W S / ↑ ↓ walk the title screen's and the pause card's menu (Enter and Space are the focused button's own): it only ever moves the focus, and stands down while a
// screen is up over the card (#menu is inert then) or the focus is somewhere else
export function walkMenu(e: KeyboardEvent, typed: (t: EventTarget | null) => boolean): void {
  const d = e.code === 'KeyW' || e.code === 'ArrowUp' ? -1 : e.code === 'KeyS' || e.code === 'ArrowDown' ? 1 : 0; if (!d || e.ctrlKey || e.metaKey || e.altKey) return;
  const m = document.getElementById('menu'); if (!m || !m.classList.contains('on') || !(m.classList.contains('title') || m.classList.contains('paused')) || m.hasAttribute('inert')) return;
  const a = document.activeElement as HTMLElement | null; if (a && a !== document.body && !m.contains(a)) return;
  if (typed(a) && e.code.startsWith('Key')) return;   // (a letter typed into a field is the field's: the arrows still walk)
  const bs = [...m.querySelectorAll('.actions .btn:not(:disabled), .actions input[type="text"]:not([data-walk="off"]), .actions .saves .tile[tabindex="0"]')] as HTMLElement[]; if (!bs.length) return;   // (the strip of saves is one stop: its own arrows walk it)
  const i = a ? bs.indexOf(a) : -1; bs[i < 0 ? (d > 0 ? 0 : bs.length - 1) : (i + d + bs.length) % bs.length].focus(); e.preventDefault();
}
