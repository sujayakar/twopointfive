// The orders ring at the cursor (the Rust game's menu says WHEN and WHAT, `view.ring`; this file draws it): a wedge a digit, clockwise from the top,
// the lit one filled, a greyed one for a row that cannot be given now, its words outside it, the middle left open so what is pointed at shows through.
// Under it, small, who the order goes to and what the ring was opened on. Sam's own ring (Space, `view.hand`) is the same
// drawing in his own colour, the thing's name under it. A ring with ONE row draws it as a pill beside a small ring round the middle (ui/ringlone.ts).
// index.html `.cmdring …` is the look. Nothing is built until there is something to show; an attribute is written only when it changes.
// The layer is zoomed with the page's words (ui/textsize.ts): a place in the view's px is divided by that factor, and the ring is drawn at the game's own
// (the game's ring zoom, told in the view), so the wedge under the cursor is the wedge the game lights. How big the wheel is and how many places it has are the numbers the game reads the
// cursor by (`view.look.ring`).
import type { HudCam } from '../view/hud';
import type { RingLook, RingPin, RingWedge, TickView } from '../view/read';
import { lonePill, loneWedge, wedgeMid, wedgeWords, type LonePill, type RingPlace } from './ringlone';

/** a ring as it is drawn this frame */
interface RingDrawn { x: number; y: number; zoom: number; lit: number; tone: string | undefined; caption: string; wedges: RingWedge[] }

const NS = 'http://www.w3.org/2000/svg';
const RN = 30.5, RT = 53, RC = 86;   // px: where a digit sits, where a wedge's words start, how far under the middle the caption sits
const GAP = 3;   // degrees left open each side of a wedge

/** a wedge's elements, its place on the wheel, and what it is placed as now ('': nothing yet) */
interface WedgeEls { g: SVGGElement; path: SVGPathElement; num: SVGTextElement; text: SVGTextElement; home: RingPlace; as: string }

/** one ring's elements, built when it is first shown */
class RingDraw {
  private g: SVGGElement | null = null;
  private cap: SVGTextElement | null = null;
  private hub: SVGCircleElement | null = null;   // the middle's own ring, drawn with a lone pill (ringlone.ts)
  private els: WedgeEls[] = [];
  private was = '';
  constructor(private svg: SVGSVGElement) { }
  private build(L: RingLook) {
    const n = L.places, R0 = L.hole, R1 = L.rim;
    const g = this.g = document.createElementNS(NS, 'g') as SVGGElement; g.setAttribute('class', 'cmdring'); this.svg.appendChild(g);
    const at = (r: number, deg: number) => { const a = deg * Math.PI / 180; return `${(r * Math.cos(a)).toFixed(2)},${(r * Math.sin(a)).toFixed(2)}`; };
    const step = 360 / n;
    for (let k = 0; k < n; k++) {
      const rad = wedgeMid(k + 1, n), mid = rad * 180 / Math.PI, a0 = mid - step / 2 + GAP, a1 = mid + step / 2 - GAP, c = Math.cos(rad), s = Math.sin(rad);
      const w = document.createElementNS(NS, 'g') as SVGGElement;
      const path = document.createElementNS(NS, 'path') as SVGPathElement; path.setAttribute('class', 'cw');
      const num = document.createElementNS(NS, 'text') as SVGTextElement; num.setAttribute('class', 'cn'); num.textContent = String(k + 1);
      const text = document.createElementNS(NS, 'text') as SVGTextElement; text.setAttribute('class', 'ct');
      const home: RingPlace = { d: `M${at(R1, a0)} A${R1},${R1} 0 0 1 ${at(R1, a1)} L${at(R0, a1)} A${R0},${R0} 0 0 0 ${at(R0, a0)} Z`, nx: RN * c, ny: RN * s + 4, tx: RT * c, ty: RT * s + (s < -0.5 ? -2 : s > 0.5 ? 9 : 3), anchor: Math.abs(c) < 0.2 ? 'middle' : c > 0 ? 'start' : 'end' };
      const e: WedgeEls = { g: w, path, num, text, home, as: '' }; this.place(e, home, 'wheel');
      w.append(path, num, text); g.appendChild(w); this.els.push(e);
    }
    const hub = this.hub = document.createElementNS(NS, 'circle') as SVGCircleElement; hub.setAttribute('class', 'ch'); hub.setAttribute('r', String(R0)); hub.style.display = 'none'; g.appendChild(hub);
    const cap = this.cap = document.createElementNS(NS, 'text') as SVGTextElement; cap.setAttribute('class', 'cc'); cap.setAttribute('y', String(RC)); g.appendChild(cap);
  }
  /** a wedge's outline and digit put where `at` says (its own place on the wheel, or a pill's: `as` names which), written only when that changes. A
   *  pill's words are written here too, each line set to the width counted for it (the game reads the same pill — ringlone.ts): one line, or a long
   *  label's two (loneLines), with the whole label kept as the element's name */
  private place(e: WedgeEls, at: RingPlace | LonePill, as: string, words = '') {
    if (e.as === as) return; e.as = as; const f = (v: number) => v.toFixed(1), lines = 'lines' in at ? at.lines : null, one = lines?.length === 1 ? lines[0] : null;
    e.path.setAttribute('d', at.d); e.num.setAttribute('x', f(at.nx)); e.num.setAttribute('y', f(at.ny)); e.text.setAttribute('x', f(at.tx)); e.text.setAttribute('y', f(at.ty)); e.text.setAttribute('text-anchor', at.anchor);
    if (one) e.text.setAttribute('textLength', f(one.w)); else e.text.removeAttribute('textLength');
    if (lines && !one) e.text.setAttribute('aria-label', words); else e.text.removeAttribute('aria-label');
    if (one) e.text.textContent = one.s;
    else if (lines) e.text.replaceChildren(...lines.map(l => { const t = document.createElementNS(NS, 'tspan'); t.textContent = l.s; t.setAttribute('x', f(at.tx)); t.setAttribute('y', f(l.y)); t.setAttribute('textLength', f(l.w)); return t; }));
  }
  draw(V: RingDrawn | null, tz: number, L: RingLook) {
    if (!V) { if (this.was) { this.was = ''; if (this.g) this.g.style.display = 'none'; } return; }
    const sc = V.zoom / tz, key = `${V.x.toFixed(0)},${V.y.toFixed(0)},${sc.toFixed(2)}|${V.lit}|${V.tone ?? ''}|${V.caption}|${V.wedges.map(w => `${w.key}${w.off ? '-' : ''}${w.opens ? '>' : ''}${w.text}`).join('|')}`;
    if (key === this.was) return; this.was = key;
    if (!this.g || this.els.length !== L.places) { this.g?.remove(); this.els = []; this.build(L); }
    const g = this.g!; g.style.display = ''; g.setAttribute('transform', `translate(${(V.x / tz).toFixed(1)},${(V.y / tz).toFixed(1)}) scale(${sc.toFixed(3)})`);
    g.setAttribute('class', `cmdring${V.tone && V.tone !== 'gold' ? ` t-${V.tone}` : ''}`);
    if (this.cap) this.cap.textContent = V.caption;
    const one = loneWedge(V.wedges); if (this.hub) this.hub.style.display = one ? '' : 'none';
    this.els.forEach((e, k) => { const w = V.wedges.find(x => x.key === k + 1);
      e.g.style.display = w ? '' : 'none'; if (!w) return;
      e.g.setAttribute('class', `${V.lit === w.key ? 'lit' : ''}${w.off ? ' off' : ''}${one ? ' lone' : ''}`.trim());
      const words = wedgeWords(w);
      if (one) { const as = `pill|${words}`; if (e.as !== as) this.place(e, lonePill(w, this.els.length, L.lone), as, words); }   // (the pill is worked out only when its words change)
      else { e.text.textContent = words; this.place(e, e.home, 'wheel'); } });
  }
}

export class CmdRingViz {
  private orders: RingDraw; private hands: RingDraw;   // the squad's ring, and Sam's own (CMDRING.hands)
  constructor(private svg: SVGSVGElement) { this.orders = new RingDraw(svg); this.hands = new RingDraw(svg); }
  /** A ring that stays on its point, where THIS picture's camera puts it, in the view's px: the game's own place for it was worked out before the tick's camera step, and a ring
   *  drawn there swims a few px against the floor while the view moves. null = the game's place stands */
  private pinned(R: RingPin | null, cam: HudCam, w: number, h: number, tz: number): [number, number] | null {
    if (!R) return null;
    const [sx, sy, front] = cam.project(R.at, w * tz, h * tz); if (!front) return null;
    const cl = (v: number, m: number, max: number) => max > 2 * m ? Math.max(m, Math.min(max - m, v)) : v;
    return [cl(sx + R.ox, R.edgeX, w * tz), cl(sy + R.oy, R.edgeY, h * tz)];
  }
  /** `w`, `h`: the layer's own size · `tz`: the factor it is zoomed by (ui/textsize.ts) */
  update(view: TickView, cam: HudCam, w: number, h: number, tz: number) {
    const L = view.look.ring, R0 = view.ring, P = R0 ? this.pinned(R0.pin, cam, w, h, tz) : null, V: RingDrawn | null = R0 ? { x: P ? P[0] : R0.x, y: P ? P[1] : R0.y, zoom: R0.zoom, lit: R0.lit, tone: R0.tone ?? undefined, caption: R0.caption, wedges: R0.wedges } : null;
    const H0 = view.hand, HP = H0 ? this.pinned(H0.pin, cam, w, h, tz) : null, H: RingDrawn | null = H0 ? { x: HP ? HP[0] : H0.x, y: HP ? HP[1] : H0.y, zoom: H0.zoom, lit: H0.lit, tone: 'hands', caption: '', wedges: H0.wedges } : null;   // (his own ring on its thing)
    this.orders.draw(V, tz, L); this.hands.draw(H, tz, L); this.clear(V, H, tz, L);
    const D = this.discs; D.length = 0; for (const R of [V, H]) if (R) D.push(R.x / tz, R.y / tz, L.rim * R.zoom / tz);
  }
  /** the rings drawn now as discs of the layer, to the rim (x, y, r in threes) */
  private discs: number[] = [];
  /** is this point of the layer (the same px the overlay places its marks in) under a ring that is drawn (ui/overlay.ts: a tutorial step's mark gives way, RAILS.markClear) */
  covers(x: number, y: number): boolean { const D = this.discs; for (let i = 0; i < D.length; i += 3) if (Math.hypot(x - D[i], y - D[i + 1]) <= D[i + 2]) return true; return false; }
  private overKey = '';
  /** a ring is drawn (the tutorial's big line is at home under one, its step clear of your men stood down: ui/overlay.ts) */
  get ringUp(): boolean { return this.overKey !== ''; }
  private railShift = 0;
  /** the status column goes faint while the squad ring's words lie over it (a ring put up on a door near that corner was read through the column's plates);
   *  and the tutorial's big line steps out from under either ring (it lies over the ring's layer: with the cursor low in the middle, a ring's lower wedge
   *  and its caption were under the line's plate) — down if the view has the room, else up over the ring (index.html `--railshift`) */
  private clear(V: RingDrawn | null, H: RingDrawn | null, tz: number, L: RingLook) {
    const rail = V || H ? document.getElementById('railprompt') : null, k = (R: RingDrawn | null) => R ? `${(R.x / 4).toFixed(0)},${(R.y / 4).toFixed(0)},${R.zoom.toFixed(2)}` : '';   // (to 4 px: a ring that stays on its point moves a little every frame the view does, and what it stands over is asked of the layout)
    const key = V || H ? `${k(V)}|${k(H)}|${rail?.classList.contains('on') ? rail.textContent?.length : ''}|${innerHeight}` : ''; if (key === this.overKey) return; this.overKey = key;
    const s = key ? this.svg.getBoundingClientRect() : null;
    /** a ring's words as a box of the window: wider than tall, and down to under its caption when `cap` */
    const box = (R: RingDrawn, cap: boolean) => { const cx = s!.left + R.x / tz, cy = s!.top + R.y / tz, z = R.zoom / tz; return { l: cx - L.boxX * z, r: cx + L.boxX * z, t: cy - L.boxY * z, b: cy + (cap ? RC + 14 : L.boxY) * z }; };
    const col = V ? document.getElementById('interact')?.getBoundingClientRect() : null, v = V && col && col.width > 0 && col.height > 0 ? box(V, false) : null;
    document.body.classList.toggle('ringover', !!v && v.l < col!.right && v.r > col!.left && v.t < col!.bottom && v.b > col!.top);
    let shift = 0;
    if (rail?.classList.contains('on')) { const b = rail.getBoundingClientRect(), cur = new DOMMatrixReadOnly(getComputedStyle(rail).transform).m42 * tz, top = b.top - cur, bottom = b.bottom - cur;   // (where the line is at rest: measured less the step it is on now — it may be part way there)
      for (const R of [V, H]) { if (!R) continue; const q = box(R, !!R.caption); if (!(b.width > 0 && q.l < b.right && q.r > b.left && q.t < bottom && q.b > top)) continue;
        const down = q.b + 8 - top, up = q.t - 8 - bottom; shift = bottom + down <= innerHeight - 8 ? down : up; break; } }
    if (shift !== this.railShift) { this.railShift = shift; const el = document.getElementById('railprompt'); if (el) el.style.setProperty('--railshift', `${(shift / tz).toFixed(1)}px`); }
  }
}
