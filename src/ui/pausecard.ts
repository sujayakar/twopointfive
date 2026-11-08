// THE CARDS OVER A RUN of a page that holds no game (#menu, styled in index.html): paused (Resume, the saves, Restart, Options, the way out) and ended (how, and what to do about it).
// The world holds still under either.
import { menuTheme } from './theme';
import { closeOptions, openOptions } from './options';
import { button, checkpointButton, el, layStripAgain, saveStrip, type RunSaves } from './runcard';

export interface PauseDeps {
  /** what is paused, under the wordmark */
  title: string;
  saves: RunSaves;
  /** how the run ended, or null */
  verdict(): { won: boolean; word: string; why: string } | null;
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
  end(): void { if (this.up) this.close(); this.ended = true; this.open(); }
  open(): void {
    if (!this.d.may()) { this.ended = false; return; }
    this.lay();   // (it asks the game: a game lost to the asking is no card)
    this.up = true; this.root.classList.add('on', 'paused'); this.d.turned(true);
  }
  close(): void { closeOptions(); this.up = false; this.ended = false; this.root.classList.remove('on', 'paused'); this.root.textContent = ''; this.d.turned(false); }
  private press = { armed: (): boolean => true, gone: (): void => this.close(), again: (): void => this.lay() };
  private stripAt: HTMLElement | null = null;   // (what the strip of saves sits before)
  private lay(): void {
    const r = this.root, card = el('div', 'card'), side = el('div', 'side'), head = el('div', 'head'), actions = el('div', 'actions'), gap = (): void => { actions.appendChild(el('div', 'gap')); };
    r.textContent = ''; r.appendChild(card); card.appendChild(side);
    { const t1 = el('div', 't1'), t2 = el('div', 't2'); t1.innerHTML = 'two<b>point</b>six'; const v = this.ended ? this.d.verdict() : null; t2.textContent = v ? `${v.word}${v.why ? ` · ${v.why}` : ''}` : this.d.title ? `paused · ${this.d.title}` : 'paused'; head.append(t1, t2); }
    side.appendChild(head);
    // (an end card comes up mid-stride: for half a second no press is taken for its buttons)
    const at = performance.now(), armed = (): boolean => !this.ended || performance.now() - at >= 500, press = this.press = { armed, gone: () => this.close(), again: () => this.lay() };
    const resume = button(this.ended ? 'Keep playing' : 'Resume', () => { if (armed()) this.close(); }, this.ended ? '' : 'primary');
    const again = button(this.ended ? 'Again' : 'Restart', () => { if (!armed()) return; this.d.restart(); this.close(); }, this.ended ? 'primary' : '');
    // (lost with a save of this run: going back to it leads; else starting again does)
    const lost = this.ended && !(this.d.verdict()?.won ?? false), b = checkpointButton(this.d.saves, press, lost ? 'primary' : ''), strip = saveStrip(this.d.saves, press);
    this.stripAt = again; if (lost && b) again.classList.remove('primary');
    for (const x of !this.ended ? [resume, b, strip, again] : lost && b ? [b, again, strip, resume] : [again, b, strip, resume]) if (x) actions.appendChild(x);
    { const watch = this.ended ? this.d.replay() : null; if (watch) actions.appendChild(button('Replay', () => { if (armed()) watch(); })); }
    gap();
    if (!this.ended) { actions.appendChild(button('Options', () => openOptions())); gap(); }
    actions.appendChild(button('Quit to missions', () => { if (armed()) this.d.quit(); }, 'quit'));
    side.appendChild(actions);
    { const foot = el('div', 'foot'); foot.textContent = this.ended ? 'Esc keeps playing · W S choose' : 'Esc resumes · W S choose'; side.appendChild(foot); }
    setTimeout(() => (!this.ended ? resume : lost && b ? b : again).focus(), 0);
  }
}
