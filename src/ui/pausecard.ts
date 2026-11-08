// THE CARDS OVER A RUN of a page that holds no game (#menu, styled in index.html): paused (Resume, the saves, Restart, Options, the way out) and ended (how, and what to do about it).
// The world holds still under either.
import type { RunEnd } from '../view/runend';
import { renderDebrief } from './debrief';
import { menuTheme } from './theme';
import { closeOptions, openOptions } from './options';
import { button, checkpointButton, el, layStripAgain, saveStrip, type RunSaves } from './runcard';

interface Next { title: string; start(): void }
export interface PauseDeps {
  /** what is paused, under the wordmark */
  title: string;
  saves: RunSaves;
  /** the run as a card of it is drawn (ui/debrief.ts), as it stands */
  end(): RunEnd;
  /** the lesson after this one, and the next of the list this page is of (null: there is none, or what is up is not its) */
  lesson(): Next | null;
  pick(): Next | null;
  /** may a card go up: a run is on, and not lost to a panic (which has a card of its own) */
  may(): boolean;
  restart(): void;
  /** watch the run that ended; null: it is not kept */
  replay(): (() => void) | null;
  /** leave the mission */
  quit(): void;
  /** Escape puts this away first: true = it did, and that is the press's whole job */
  away(): boolean;
  /** a save by hand, written as play goes on; false = it would not be made */
  save(): boolean;
  /** the card went up, or came down */
  turned(up: boolean): void;
}
export class PauseCard {
  up = false;
  private ended = false;
  private root = document.getElementById('menu')!;
  constructor(private d: PauseDeps) {
    menuTheme();
    this.root.addEventListener('mousedown', e => e.stopPropagation());   // (a click on the card is no shot)
    this.root.addEventListener('contextmenu', e => e.preventDefault());
    addEventListener('keydown', e => { if (e.code === 'Escape' && !e.repeat) { e.preventDefault(); if (!this.d.away()) this.toggle(); }
      else if (e.code === 'F5' && !e.repeat && !e.metaKey && !e.ctrlKey && !e.shiftKey && !e.altKey && this.up && !this.ended) { e.preventDefault(); if (this.d.save()) layStripAgain(this.root, on => saveStrip(this.d.saves, this.press, on), this.stripAt); } }, { capture: true });
  }
  toggle(): void { if (this.up) this.close(); else this.open(); }
  /** a save's picture came after its tile was laid: the tile takes it */
  pictured(save: string, url: string): void { for (const t of this.root.querySelectorAll<HTMLElement>('.saves .tile')) if (t.dataset.save === save) t.style.backgroundImage = `url("${url}")`; }
  /** the run's end card */
  end(): void { if (this.up) this.close(); this.ended = true; try { this.open(); } finally { if (!this.up) this.ended = false; } }
  open(): void {
    if (!this.d.may()) { this.ended = false; return; }
    this.lay();   // (it asks the game: a game lost to the asking is no card)
    this.up = true; this.root.classList.add('on'); this.root.classList.toggle('paused', !this.ended); this.root.classList.toggle('ended', this.ended); this.d.turned(true);
  }
  close(): void { closeOptions(); this.up = false; this.ended = this.soFar = false; this.root.classList.remove('on', 'paused', 'ended'); this.root.textContent = ''; this.d.turned(false); }
  private press = { armed: (): boolean => true, gone: (): void => this.close(), again: (): void => this.lay() };
  private stripAt: HTMLElement | null = null;   // (what the strip of saves sits before)
  /** the run as it stands, beside the menu of a paused one */
  private soFar = false;
  private lay(): void {
    const r = this.root, card = el('div', 'card'); r.textContent = ''; r.appendChild(card);
    if (this.ended) this.layEnd(card); else this.layPause(card);
  }
  private layPause(card: HTMLElement): void {
    const side = el('div', 'side'), head = el('div', 'head'), actions = el('div', 'actions'), gap = (): void => { actions.appendChild(el('div', 'gap')); }, press = this.press = { armed: () => true, gone: () => this.close(), again: () => this.lay() };
    card.appendChild(side);
    { const t1 = el('div', 't1'), t2 = el('div', 't2'); t1.innerHTML = 'two<b>point</b>six'; t2.textContent = this.d.title ? `paused · ${this.d.title}` : 'paused'; head.append(t1, t2); }
    side.appendChild(head);
    const resume = button('Resume', () => this.close(), 'primary'), again = this.stripAt = button('Restart', () => { this.d.restart(); this.close(); }), b = checkpointButton(this.d.saves, press), strip = saveStrip(this.d.saves, press);
    for (const x of [resume, b, strip, again]) if (x) actions.appendChild(x);
    gap();
    actions.appendChild(button('Options', () => openOptions()));
    actions.appendChild(button('Run so far', () => { this.soFar = !this.soFar; this.lay(); }, this.soFar ? 'on' : ''));
    gap();
    actions.appendChild(button('Quit to missions', () => this.d.quit(), 'quit'));
    side.appendChild(actions);
    { const foot = el('div', 'foot'); foot.textContent = 'Esc resumes · W S choose'; side.appendChild(foot); }
    if (this.soFar) { card.classList.add('wide'); const body = el('div', 'body'); card.appendChild(body); renderDebrief(body, this.d.end()); }   // (nothing to press on it but 'copy summary')
    setTimeout(() => resume.focus(), 0);
  }
  /** the run's end: how it went, and what to do about it */
  private layEnd(card: HTMLElement): void {
    const E = this.d.end(), V = E.verdict, lost = V ? !V.won || E.down : E.view.debrief.state !== 'done';
    // (it comes up mid-stride: for half a second no press is taken for its buttons)
    const at = performance.now(), armed = (): boolean => performance.now() - at >= 500, press = this.press = { armed, gone: () => this.close(), again: () => this.lay() };
    card.classList.add('wide'); this.stripAt = null;
    const lead = V && V.won && !lost && E.kept?.lesson ? this.d.lesson() : null, pick = this.d.pick(), go = (n: Next, cls: string): HTMLElement => button(`Next: ${n.title} ›`, () => { if (armed()) n.start(); }, cls);
    const first = lead ? go(lead, 'primary next') : null, b = lost ? checkpointButton(this.d.saves, press, first ? '' : 'primary') : null, strip = lost ? saveStrip(this.d.saves, press) : null;
    const again = button(lost ? 'Restart' : 'Again', () => { if (!armed()) return; this.d.restart(); this.close(); }, first || b ? '' : 'primary'), keep = button(lost ? 'Back to the floor' : 'Keep playing', () => { if (armed()) this.close(); });
    const watch = this.d.replay(), actions = [first, b, again, keep, strip, button('Quit to missions', () => { if (armed()) this.d.quit(); }, 'quit'), pick ? go(pick, 'next') : null, watch ? button('Replay', () => { if (armed()) watch(); }, 'replay') : null].filter((x): x is HTMLElement => !!x);
    const body = el('div', 'body'); card.appendChild(body); renderDebrief(body, E, actions);
    { const foot = el('div', 'foot'); foot.textContent = lost ? (b ? 'Press Enter to go back to the checkpoint · press Esc to close this' : 'Press Enter to start again · press Esc to close this') : 'Press Esc to keep playing · Esc ▸ Run so far shows this for a run in progress'; card.appendChild(foot); }
    setTimeout(() => (first ?? b ?? again).focus(), 0);
  }
}
