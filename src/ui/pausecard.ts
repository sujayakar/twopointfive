// THE CARDS OVER A RUN of a page that holds no game (#menu, styled in index.html): paused (Resume, the saves, Restart, Options, the way out) and ended (how, and what to do about it).
// The world holds still under either.
import { typedInto } from '../host/keymap';
import { menuTheme } from './theme';
import { closeOptions, openOptions } from './options';
import { button, checkpointButton, el, saveStrip, walkMenu, type RunSaves } from './runcard';

export interface PauseDeps {
  /** what is paused, under the wordmark */
  title: string;
  saves: RunSaves;
  /** how the run ended, or null */
  verdict(): { won: boolean; word: string; why: string } | null;
  /** may a card go up (the game may be lost to a panic, which has a card of its own) */
  may(): boolean;
  restart(): void;
  /** leave the mission */
  quit(): void;
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
    addEventListener('keydown', e => { if (e.code === 'Escape' && !e.repeat) { e.preventDefault(); this.toggle(); } else if (this.up) walkMenu(e, typedInto); }, { capture: true });
  }
  toggle(): void { if (this.up) this.close(); else this.open(); }
  /** the run's end card */
  end(): void { if (this.up) this.close(); this.ended = true; this.open(); }
  open(): void {
    if (!this.d.may()) { this.ended = false; return; }
    this.lay();   // (it asks the game: a game lost to the asking is no card)
    this.up = true; this.root.classList.add('on', 'paused'); this.d.turned(true);
  }
  close(): void { closeOptions(); this.up = false; this.ended = false; this.root.classList.remove('on', 'paused'); this.root.textContent = ''; this.d.turned(false); }
  private lay(): void {
    const r = this.root, card = el('div', 'card'), side = el('div', 'side'), head = el('div', 'head'), actions = el('div', 'actions'), gap = (): void => { actions.appendChild(el('div', 'gap')); };
    r.textContent = ''; r.appendChild(card); card.appendChild(side);
    { const t1 = el('div', 't1'), t2 = el('div', 't2'); t1.innerHTML = 'two<b>point</b>six'; const v = this.ended ? this.d.verdict() : null; t2.textContent = v ? `${v.word}${v.why ? ` · ${v.why}` : ''}` : this.d.title ? `paused · ${this.d.title}` : 'paused'; head.append(t1, t2); }
    side.appendChild(head);
    // (an end card comes up mid-stride: for half a second no press is taken for its buttons)
    const at = performance.now(), armed = (): boolean => !this.ended || performance.now() - at >= 500, press = { armed, gone: () => this.close(), again: () => this.lay() };
    const resume = button(this.ended ? 'Keep playing' : 'Resume', () => { if (armed()) this.close(); }, this.ended ? '' : 'primary');
    const again = button(this.ended ? 'Again' : 'Restart', () => { if (!armed()) return; this.d.restart(); this.close(); }, this.ended ? 'primary' : '');
    // (lost with a save of this run: going back to it leads; else starting again does)
    const lost = this.ended && !(this.d.verdict()?.won ?? false), b = checkpointButton(this.d.saves, press, lost ? 'primary' : ''), strip = saveStrip(this.d.saves, press);
    if (lost && b) again.classList.remove('primary');
    for (const x of !this.ended ? [resume, b, strip, again] : lost && b ? [b, again, strip, resume] : [again, b, strip, resume]) if (x) actions.appendChild(x);
    gap();
    if (!this.ended) { actions.appendChild(button('Options', () => openOptions())); gap(); }
    actions.appendChild(button('Quit to missions', () => { if (armed()) this.d.quit(); }, 'quit'));
    side.appendChild(actions);
    { const foot = el('div', 'foot'); foot.textContent = this.ended ? 'Esc keeps playing · W S choose' : 'Esc resumes · W S choose'; side.appendChild(foot); }
    setTimeout(() => (!this.ended ? resume : lost && b ? b : again).focus(), 0);
  }
}
