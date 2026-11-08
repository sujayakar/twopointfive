// THE TEAM & KIT PICTURE, the page's half of it: this window is a frame of the kit screen (ui/kitView.ts), on the very address Deploy would load plus `kitview`, and the game in its
// module is that mission with its team stood at its way in (the module stages it: `kitstage`). What the kit screen says of the cards, of whose card is open and of where the men should
// sit is commanded; what the window says back is `boot` (it listens), `ready` (the picture is fit to show) and `none` (there is nothing to stage here, or the page has died). A team
// staged again is staged under a copy of the last picture, which fades.
import type { Commands } from '../draw/commands';

/** drawn frames before a first picture is shown · frames drawn under the copy before it fades, and how long it fades (ms) */
const READY = 8, SETTLE = 8, FADE_MS = 250;
/** the view section's word: nothing to stage */
const NONE = 2;

/** a page opened from disk has no origin to name: a message goes anywhere, and arrives as 'null' */
const onDisk = (): boolean => location.protocol === 'file:' || location.origin === 'null';
const post = (kv: string): void => { try { if (window.parent !== window) window.parent.postMessage({ kv }, onDisk() ? '*' : location.origin); } catch { /* */ } };

/** there is no picture here: the kit screen drops the frame, shown or not (nothing new said twice) */
export function kitNone(): void { post('none'); }

/** a `frame` message's numbers, as the command has them (the module keeps the rule of each) */
const SAT = ['cx', 'cy', 'h', 'w', 'man', 'door', 'left', 'right', 'keep', 'top', 'bottom'] as const;

export class KitPage {
  /** the kit screen's step is not showing: nothing is stepped or drawn (but for the first picture, which it waits for) */
  get rests(): boolean { return this.gone || (this.paused && this.said); }
  private paused = false; private said = false; private gone = false; private drawn = 0;
  private frame: number[] = SAT.map(() => NaN); private aspect = 0; private who = 0;
  /** the last picture's copy: laid over it as a team is staged again (`left` frames drawn under it yet, 0 as it fades) */
  private veil: HTMLCanvasElement | null = null; private left = -1; private due: (() => void)[] = [];

  constructor(private readonly view: HTMLCanvasElement, private readonly commands: Commands) {
    window.addEventListener('message', e => {
      if (e.source !== window.parent || !(e.origin === location.origin || (onDisk() && e.origin === 'null'))) return;
      const m = e.data as Record<string, unknown> | null; if (!m || typeof m !== 'object') return;
      if (m.kv === 'kit') this.kit(m); else if (m.kv === 'sel') this.sel(m); else if (m.kv === 'pause') this.paused = m.on === true; else if (m.kv === 'frame') this.sit(m);
    });
    // (whatever kills this window's page: nobody can see a card in here)
    for (const ev of ['error', 'unhandledrejection'] as const) window.addEventListener(ev, e => { if (!/ResizeObserver loop/.test((e as ErrorEvent).message ?? '')) this.none(); });
    post('boot');
  }
  private kit(m: Record<string, unknown>): void {
    const n = m.squad, size = typeof n === 'number' && Number.isInteger(n) && n >= 0 && n <= 4 ? n : -1, kept = JSON.stringify({ v: 2, own: true, stealth: true, members: Array.isArray(m.members) ? m.members : [] });
    this.under(() => this.commands.kitStageCards(kept, size));
  }
  private sel(m: Record<string, unknown>): void {
    const w = Number(m.who), who = Number.isInteger(w) && w >= 1 && w <= 4 ? w : 0, field = typeof m.field === 'string' ? m.field : '', open = (): void => this.commands.kitStageOpen(who, field);
    if (who === this.who) { open(); return; }
    this.who = who; this.under(open);   // (another card: its man is stood at the head of the file)
  }
  private sit(m: Record<string, unknown>): void { this.frame = SAT.map(k => typeof m[k] === 'number' ? m[k] as number : NaN); this.tellFrame(); }
  private tellFrame(): void { const [cx, cy, h, w, man, door, left, right, keep, top, bottom] = this.frame; this.commands.kitStageFrame(this.aspect, cx, cy, h, w, man, door, left, right, keep, top, bottom); }
  /** before a frame is stepped: the window's shape, where it has changed */
  before(aspect: number): void { if (Math.abs(aspect - this.aspect) < 1e-4) return; this.aspect = aspect; this.tellFrame(); }

  /** `work` done under a copy of the picture as it stands: at once where none has been shown */
  private under(work: () => void): void { if (!this.said || this.gone) { work(); return; } this.due.push(work); }
  /** a frame has been drawn, in the task that drew it; `kit` = the view section's word */
  after(kit: number): void {
    if (this.gone) return;
    if (kit === NONE) { this.none(); return; }
    this.drawn++;
    if (!this.said && this.drawn >= READY) { this.said = true; post('ready'); }
    if (this.due.length) { this.cover(); for (const w of this.due.splice(0)) w(); this.left = SETTLE; return; }
    if (this.left > 0 && --this.left === 0 && this.veil) { const v = this.veil; void v.offsetWidth; v.style.transition = `opacity ${FADE_MS}ms linear`; v.style.opacity = '0'; }
  }
  private cover(): void {
    try {
      const v = this.veil ??= (() => { const c = document.createElement('canvas'); c.style.cssText = 'position:fixed;pointer-events:none;z-index:1'; c.style.setProperty('display', 'block', 'important'); (this.view.parentElement ?? document.body).appendChild(c); return c; })();
      const r = this.view.getBoundingClientRect(), g = v.getContext('2d'); if (!g || !(this.view.width > 0 && this.view.height > 0)) return;
      if (v.width !== this.view.width || v.height !== this.view.height) { v.width = this.view.width; v.height = this.view.height; }
      g.globalCompositeOperation = 'copy'; g.drawImage(this.view, 0, 0);
      Object.assign(v.style, { left: `${r.left}px`, top: `${r.top}px`, width: `${r.width}px`, height: `${r.height}px`, transition: 'none', opacity: '1' });
    } catch { /* no copy to be had: a cut */ }
  }
  /** no picture, for good */
  none(): void { this.gone = true; if (this.veil) this.veil.style.setProperty('display', 'none', 'important'); kitNone(); }
}
