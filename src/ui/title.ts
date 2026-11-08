// THE TITLE SCREEN'S PARTS, for whichever page puts it up (#menu, styled in index.html): the wordmark and the main menu down the left, the Extras list, and the boot card, which is the same screen
// while the renderer compiles. It holds no game: the starting page is the desk's (ui/startpage.ts), Options has whatever game it is handed, and what a page has beside them it
// says in TITLE.
import { keyText, typedInto } from '../host/keymap';
import { desk } from '../view/desk';
import { closeMissions, missionsBusy, missionsReopenAsked } from './missions';
import { closeOptions, openOptions, optionsOpen } from './options';
import { button, el, walkMenu } from './runcard';
import { openList, openStartPage, startPageFromQuery, startPageWanted } from './startpage';
import { menuTheme } from './theme';

/** what this page's title has beside Play, Options, Controls and Extras */
export const TITLE = {
  /** two players (it fills the column; null: not on this page) */
  coop: null as (() => ((host: HTMLElement, back: () => void, focus: boolean) => void) | null) | null,
  /** Extras' rows that are keys of this page (the benchmark) */
  ownKeys: true,
};

/** the title screen's head and its main menu (the boot card and the real start card build the same screen) */
function titleHead(): HTMLElement { const head = el('div', 'head'); head.innerHTML = `<div class="t1">two<b>point</b>six</div><div class="t2">${TITLE_LINE}</div>`; return head; }
/** EXTRAS (the title screen's fourth entry: the maintainer, 10-11 — "the showcase and editor and whatever other tools we have … should all be linked in the main menu"): the
 *  tool pages, each a LINK — a tool is its own page (the `.html` names: the dev server forwards them to its routes, the build has them as files); a row whose link is a key of this page (`?bench`) keeps the visitor's other flags (`showHref`) and, ON THE LIVE TITLE, STARTS IN PLACE
 *  (Menu.title hands `TitleShows`) — no page load, the click is the gesture the sound needs; the link is what such a row is while the page still boots, or in a new tab.
 *  (The two showcases that were this list's first section — the night and the day — were removed.) What is listed and what is not, and why: rebuild/menus/tools.md. */
export const EXTRAS: { head: string; rows: { id: string; label: string; line: string; href: string }[] }[] = [
  { head: 'Tools', rows: [
    { id: 'replay', label: 'Replays', line: 'Scrub through the last minutes of a run on a map. Press F7 to keep one while you play.', href: 'replay.html' },
    { id: 'sounds', label: 'Sounds', line: 'Hear every sound in the game on one page.', href: 'sounds.html' },
    { id: 'bench', label: 'Benchmark', line: 'It plays three floors by itself, then times each quality level on this machine. You get text to copy.', href: '?bench' } ] },
];
/** the link of a row that is a key of this page (`?bench`): its key, then the visitor's own flags as every mission link keeps them (`?mute&quality=low` rides along; another mission's keys do not) */
function showHref(href: string): string { if (href[0] !== '?') return href; const rest = desk().leave(typeof location !== 'undefined' ? location.search ?? '' : ''); return href + (rest ? '&' + rest.slice(1) : ''); }
/** a row started in place, by its id: true = started (or refused aloud), false = not from here — the row's link loads it */
type TitleShows = Partial<Record<string, () => boolean>>;
/** which page of the title's menu is up: the main one, or EXTRAS (kept through the boot card handing over to the real title) */
let titleExtras = false;
/** …or CO-OP (the page's own rows fill the column; kept through the same handover, with what was typed) */
let titleCoop = false;
let titleShownAt = 0;   // (when the live title was last drawn: an Esc that ENDED a running programme drew it, and that same press must not also step its Extras list back)
let titleAgain: (() => void) | null = null;   // (the live title's own 'Options closed' — the boot card's buttons, made before there was a Menu, reach it through here)
function titleActions(onClose?: () => void, shows?: TitleShows): { actions: HTMLElement; play: HTMLElement } {
  const onOptionsClose = onClose ?? (() => titleAgain?.());
  const actions = el('div', 'actions'); let play!: HTMLElement;
  const fill = (focus?: 'first' | 'extras' | 'coop') => {
    actions.innerHTML = ''; actions.classList.toggle('extras', titleExtras);
    const coop = TITLE.coop?.() ?? null;
    if (titleCoop && coop && !titleExtras) { coop(actions, () => { titleCoop = false; fill('coop'); }, !!focus); play = actions.querySelector('.btn') as HTMLElement; return; }
    if (titleExtras) {
      for (const sec of EXTRAS) {
        const rows = sec.rows.filter(r => TITLE.ownKeys || r.href[0] !== '?'); if (!rows.length) continue;
        const h = el('div', 'xhead'); h.textContent = sec.head; actions.appendChild(h);
        for (const r of rows) {
          const a = el('a', 'btn xrow') as HTMLAnchorElement; a.href = showHref(r.href); a.dataset.extra = r.id; a.innerHTML = `<b>${r.label}</b>`; a.title = r.line;   // (show, not tell: a row is its name; its sentence is the tooltip)
          actions.appendChild(a);
          const go = shows?.[r.id]; if (go) a.addEventListener('click', e => { if (e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return; if (go()) e.preventDefault(); });   // (a plain click or Enter; a new-tab click stays the link's)
        }
      }
      const back = button('‹ Main menu', () => { titleExtras = false; fill('extras'); }, 'xback'); actions.appendChild(back);
      play = actions.querySelector('.xrow') as HTMLElement;
      if (focus) play.focus();
      return;
    }
    play = button(PLAY, () => { openList({ backLabel: '‹ Main menu' }); }, 'primary');
    actions.appendChild(play);
    if (coop) { const co = button('Co-op', () => { titleCoop = true; fill('first'); }); co.dataset.coop = 'open'; actions.appendChild(co); if (focus === 'coop') co.focus(); }   // (two players: host a game or join one by its code — ui/coop.ts fills this column and hands it back)
    actions.appendChild(button('Options', () => openOptions({ onClose: onOptionsClose })));
    actions.appendChild(button('Controls', () => openOptions({ tab: 'controls', onClose: onOptionsClose })));
    const extras = button('Extras', () => { titleExtras = true; fill('first'); }, 'xopen'); actions.appendChild(extras);
    if (focus === 'extras') extras.focus();
  };
  fill();
  return { actions, get play() { return play; } };
}

/** the title screen's line under the wordmark */
const TITLE_LINE = 'Squad tactics and stealth. Every light, shadow and wisp of smoke is live.';
const TITLE_FOOT = 'W S choose · Enter select · Esc back · ` the debugger';
/** the card's one button to the one starting page (ui/startpage.ts) */
const PLAY = 'Play ›';

/** THE BOOT CARD: the title screen, up from the first tick of main.ts while the GPU process compiles the renderer's kernels in the background (engine.ts init), with a
 *  one-line status ('compiling shaders 40 / 97 …') main.ts rewrites as they resolve. Menu.open('start') renders the live title over it the moment everything is ready; `off()`
 *  clears it for boots that open no card (?solo, a scenario). An address that names a mission with no load brief has nothing to choose: the head and the status line, no buttons. */
export function bootCard(): { status(text: string): void; off(): void } {
  menuTheme();
  const root = document.getElementById('menu')!;
  root.innerHTML = '';
  const card = el('div', 'card'); root.appendChild(card);
  const one = startPageWanted();   // (the address names nothing to play: PLAY, its one button to the starting page — ui/startpage.ts)
  const status = el('div', 'note bootstatus'), words = el('span'), fill = el('i'); words.textContent = 'starting the GPU…';
  { const bar = el('i', 'bbar'); bar.appendChild(fill); status.appendChild(words); status.appendChild(bar); }
  root.classList.add('title'); root.classList.remove('paused');
  card.appendChild(titleHead());
  if (one) { const ta = titleActions(); card.appendChild(ta.actions); setTimeout(() => { if (!missionsBusy() && !optionsOpen() && (document.activeElement ?? document.body) === document.body) ta.play.focus(); }, 0); }   // (PLAY, OPTIONS and CONTROLS are live from the first tick: a deploy is a page load, and the controls need nothing of the renderer; Play has the focus from that tick too, as on the live title, so Enter is Play)
  card.appendChild(el('div', 'tfill'));
  card.appendChild(status);
  if (one) { const foot = el('div', 'foot'); foot.textContent = keyText(TITLE_FOOT); card.appendChild(foot); }
  root.classList.add('on');
  // A FRESH VISIT OPENS ON THE TITLE. The starting page is up by itself only where the address or the last page asked for it: `?loadout=<id>` names a mission,
  // or a mission was quit by a page load ('Quit to scenarios' lands on the missions, not the title). Never under ?paused (the automation boot: scripts drive
  // the card; PLAY still opens the page)
  if (one) { const s = typeof location !== 'undefined' ? location.search ?? '' : ''; if (!startPageFromQuery(s) && !new URLSearchParams(s).has('paused') && missionsReopenAsked()) openStartPage(); }
  return { status(text: string) { words.textContent = text; const m = /(\d+) \/ (\d+)/.exec(text); if (fill.style) fill.style.width = m && +m[2] > 0 ? `${Math.min(100, Math.round(100 * +m[1] / +m[2]))}%` : /ready|setting up/.test(text) ? '100%' : '0'; },
    off() { root.classList.remove('on'); root.classList.remove('title'); root.innerHTML = ''; closeMissions(); closeOptions(); } };   // (no card = a run is live underneath: the list goes with it)
}

export interface TitleScreen {
  /** the cheats that are on, in words (none: no chip) */
  cheats(): string;
  /** Options was closed over the title */
  closed(cheatsMoved: boolean): void;
  /** rows of Extras that start in place */
  shows?: TitleShows;
  /** under the wordmark (a mission that was refused says why), and over the foot */
  said?(card: HTMLElement): void; note?(card: HTMLElement, options: () => void): void;
}
/** THE LIVE TITLE, into its card: the wordmark, the main menu and, only when one is on, the cheats, named, a click from where they are switched */
export function titleScreen(card: HTMLElement, o: TitleScreen): void {
  const was = o.cheats(), again = (): void => o.closed(o.cheats() !== was), options = (): void => openOptions({ tab: 'game', onClose: again });
  card.appendChild(titleHead()); o.said?.(card);
  titleAgain = again; titleShownAt = typeof performance !== 'undefined' ? performance.now() : 0;
  const ta = titleActions(again, o.shows); card.appendChild(ta.actions);
  card.appendChild(el('div', 'tfill'));
  if (was) { const c = button(`Cheats on: ${was}`, options); c.className = 'cheats'; c.title = 'switch them in Options ▸ Game'; card.appendChild(c); }
  o.note?.(card, options);
  const foot = el('div', 'foot'); foot.textContent = keyText(TITLE_FOOT); card.appendChild(foot);
  setTimeout(() => { if (!missionsBusy() && !optionsOpen()) ta.play.focus(); }, 0);   // (the starting page opened off the boot card is still up over this screen: the focus stays its own)
}
/** the cheats in the title's words */
export const cheatWords = (g: { god: boolean; thinks: boolean; ammo: boolean }): string => [g.god ? 'god mode' : '', !g.thinks ? 'enemies don\'t think' : '', g.ammo ? 'unlimited ammo' : ''].filter(Boolean).join(' · ');

// W S / ↑ ↓ walk the title screen's and the pause card's menu (Enter and Space are the focused button's own). Registered here, at module load, like the other
// screens' keys; it only ever moves the focus, and stands down while a screen is up over the card (#menu is inert then) or the focus is somewhere else.
if (typeof window !== 'undefined' && typeof window.addEventListener === 'function') window.addEventListener('keydown', e => {
  if (e.code === 'Escape' && (titleExtras || titleCoop)) { const m = document.getElementById('menu'); if (e.timeStamp >= titleShownAt && m?.classList.contains('on') && m.classList.contains('title') && !m.hasAttribute('inert')) { (m.querySelector('.actions .xback') as HTMLElement | null)?.click(); e.preventDefault(); } return; }   // (Esc on the title's EXTRAS or CO-OP page = back to its main menu; the title itself has nothing to back out of)
  if (e.code === 'Space' && !e.repeat && titleExtras) { const a = document.activeElement as HTMLElement | null; if (a?.classList?.contains('xrow') && document.getElementById('menu')?.contains(a)) { a.click(); e.preventDefault(); } return; }   // (a row of EXTRAS is a link, and Space does not follow a link: here it does, as it presses every other entry of this menu)
  walkMenu(e, typedInto);
}, { capture: true });
