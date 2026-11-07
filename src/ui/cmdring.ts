// The orders ring at the cursor (game/cmdring.ts says WHEN, orders.ts `ringView` says WHAT; this file draws it): a wedge a digit, clockwise from the top,
// the lit one filled, a greyed one for a row that cannot be given now, its words outside it, the middle left open so what is pointed at shows through.
// Under it, small, who the order goes to and what the ring was opened on. Sam's own ring (Space, CMDRING.hands — interact.ts `handRingView`) is the same
// drawing in his own colour, the thing's name under it. A ring with ONE row draws it as a pill beside a small ring round the middle (game/ringlone.ts).
// Before the key, with CMDRING.hint, a small glyph beside the cursor and an outline on the floor say what the ring would open on (orders.ts `hintView`).
// index.html `.cmdring …` and `.cmdhint …` are the look. Nothing is built until there is something to show; an attribute is written only when it changes.
// The layer is zoomed with the page's words (ui/textsize.ts): a place in the view's px is divided by that factor, and the ring is drawn at the game's own
// (cmdring.ts ringZoom), so the wedge under the cursor is the wedge the game lights.
import type { Game } from '../game/game';
import type { FollowCamera } from '../game/camera';
import type { Vec3 } from '../math/vec';
import { CMDRING, ringPin, type RingHint, type RingState, type RingView } from '../game/cmdring';
import { handRingView } from '../game/interact';
import { hoverKept } from './hoverkept';
import { HANDRING } from '../game/handring';
import { RING_BOX, RING_HOLE, RING_RIM, lonePill, loneWedge, wedgeMid, wedgeWords, type LonePill, type RingPlace } from '../game/ringlone';

const NS = 'http://www.w3.org/2000/svg';
const R0 = RING_HOLE, R1 = RING_RIM, RN = 30.5, RT = 53, RC = 86;   // px: the hole, the rim, where a digit sits, where a wedge's words start, how far under the middle the caption sits
const GAP = 3;   // degrees left open each side of a wedge

const HINT_OFF = 15, HINT_N = 28, NEAR_W = 0.5;   // px: how far up and right of the cursor the glyph rides (the other way at the view's edge) · points on a floor ring · a point this near the camera's plane is not drawn
/** a glyph a kind, drawn in a 16 px box about 0,0: a leaf with its knob · courses of brick · head and shoulders · a pin on a spot · a thing to work · a man who is not ours · the squad itself */
const GLYPH: Record<RingHint, string> = {
  door: 'M-4,-6.5h8v13h-8z M1.6,0.4a0.9,0.9 0 1 0 1.8,0a0.9,0.9 0 1 0 -1.8,0',
  wall: 'M-7,-5h14v10h-14z M-7,0h14 M-1,-5v5 M3,0v5 M-4,0v5',
  man: 'M-2.6,-3.6a2.6,2.6 0 1 0 5.2,0a2.6,2.6 0 1 0 -5.2,0 M-5.5,6a5.5,5.5 0 0 1 11,0',
  floor: 'M0,6.5C-5.4,0.6 -5.2,-6 0,-6C5.2,-6 5.4,0.6 0,6.5Z M-1.7,-1.6a1.7,1.7 0 1 0 3.4,0a1.7,1.7 0 1 0 -3.4,0',
  thing: 'M0,-6l6,6l-6,6l-6,-6z M-2,0h4',
  foe: 'M0,-6l6,11h-12z',
  squad: 'M-6.4,2a1.9,1.9 0 1 0 3.8,0a1.9,1.9 0 1 0 -3.8,0 M2.6,2a1.9,1.9 0 1 0 3.8,0a1.9,1.9 0 1 0 -3.8,0 M-1.9,-3a1.9,1.9 0 1 0 3.8,0a1.9,1.9 0 1 0 -3.8,0',
};
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
  private build(n: number) {
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
    const hub = this.hub = document.createElementNS(NS, 'circle') as SVGCircleElement; hub.setAttribute('class', 'ch'); hub.setAttribute('r', String(RING_HOLE)); hub.style.display = 'none'; g.appendChild(hub);
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
  draw(V: RingView | null, tz: number) {
    if (!V) { if (this.was) { this.was = ''; if (this.g) this.g.style.display = 'none'; } return; }
    const sc = V.zoom / tz, key = `${V.x.toFixed(0)},${V.y.toFixed(0)},${sc.toFixed(2)}|${V.lit}|${V.tone ?? ''}|${V.caption}|${HANDRING.lone}${HANDRING.loneWrap}|${V.wedges.map(w => `${w.key}${w.off ? '-' : ''}${w.opens ? '>' : ''}${w.text}`).join('|')}`;
    if (key === this.was) return; this.was = key;
    if (!this.g || this.els.length !== CMDRING.wedges) { this.g?.remove(); this.els = []; this.build(CMDRING.wedges); }
    const g = this.g!; g.style.display = ''; g.setAttribute('transform', `translate(${(V.x / tz).toFixed(1)},${(V.y / tz).toFixed(1)}) scale(${sc.toFixed(3)})`);
    g.setAttribute('class', `cmdring${V.tone && V.tone !== 'gold' ? ` t-${V.tone}` : ''}`);
    if (this.cap) this.cap.textContent = V.caption;
    const one = loneWedge(V); if (this.hub) this.hub.style.display = one ? '' : 'none';
    this.els.forEach((e, k) => { const w = V.wedges.find(x => x.key === k + 1);
      e.g.style.display = w ? '' : 'none'; if (!w) return;
      e.g.setAttribute('class', `${V.lit === w.key ? 'lit' : ''}${w.off ? ' off' : ''}${one ? ' lone' : ''}`.trim());
      const words = wedgeWords(w);
      if (one) { const as = `pill|${HANDRING.loneWrap ? 2 : 1}|${words}`; if (e.as !== as) this.place(e, lonePill(w, this.els.length), as, words); }   // (the pill is worked out only when its words change)
      else { e.text.textContent = words; this.place(e, e.home, 'wheel'); } });
  }
}

export class CmdRingViz {
  private orders: RingDraw; private hands: RingDraw;   // the squad's ring, and Sam's own (CMDRING.hands)
  private hint: SVGGElement | null = null;
  private hintWorld: SVGPathElement | null = null;
  private hintGlyph: SVGPathElement | null = null;
  private hintWas = '';
  constructor(private svg: SVGSVGElement) { this.orders = new RingDraw(svg); this.hands = new RingDraw(svg); }
  /** the hint of what the ring would open on (orders.ts hintView): the thing's outline on the floor and a glyph beside the cursor — in the ring's middle
   *  once a ring is up */
  private updateHint(game: Game, cam: FollowCamera, w: number, h: number, tz: number) {
    const bc = document.body.classList, H = (CMDRING.on || CMDRING.hint) && !(bc.contains('blinded') || bc.contains('attract')) ? game.orders.hintView(hoverKept) : null;   // (hidden by the page under those: nothing asked)
    if (!H) { if (this.hintWas) { this.hintWas = ''; if (this.hint) this.hint.style.display = 'none'; } return; }
    const pt = (p: Vec3): string | null => { const [x, y, front] = cam.project(p, w, h, Infinity, NEAR_W); return front ? `${x.toFixed(1)},${y.toFixed(1)}` : null; };
    let d = '';
    if (H.seg) { const a = pt(H.seg[0]), b = pt(H.seg[1]); if (a && b) d += `M${a}L${b}`; }
    if (H.ring) { const out: string[] = []; for (let i = 0; i < HINT_N; i++) { const t = i / HINT_N * 2 * Math.PI, q = pt([H.ring.at[0] + Math.cos(t) * H.ring.r, H.ring.at[1], H.ring.at[2] + Math.sin(t) * H.ring.r]); if (!q) { out.length = 0; break; } out.push(q); }
      if (out.length) d += `M${out.join('L')}Z`; }
    // (the glyph: in the ring's middle once it is up; before, up and right of the cursor — clear of the reticle's own words — and the other way where the view ends)
    const mid = H.held && !H.press;   // (the ring up: the glyph is its middle · the key down and no ring yet: it stays beside the cursor)
    const P = mid ? this.pinned(CMDRING.pin ? game.orders.cmdRing : null, cam, w, h, tz) : null, s = H.zoom / tz, off = HINT_OFF * s, cx = (P ? P[0] : H.x) / tz, cy = (P ? P[1] : H.y) / tz;
    const gx = mid ? cx : cx + (cx + 2 * off > w ? -off : off), gy = mid ? cy : cy + (cy - 2 * off < 0 ? off : -off);
    const key = `${H.kind}|${H.tone ?? ''}|${H.held}|${gx.toFixed(0)},${gy.toFixed(0)},${s.toFixed(2)}|${d}`;
    if (key === this.hintWas) return; this.hintWas = key;
    if (!this.hint) { const g = this.hint = document.createElementNS(NS, 'g') as SVGGElement; this.svg.appendChild(g);
      this.hintWorld = document.createElementNS(NS, 'path') as SVGPathElement; this.hintWorld.setAttribute('class', 'hw');
      this.hintGlyph = document.createElementNS(NS, 'path') as SVGPathElement; this.hintGlyph.setAttribute('class', 'hg'); g.append(this.hintWorld, this.hintGlyph); }
    const g = this.hint; g.style.display = ''; g.setAttribute('class', `cmdhint k-${H.kind}${H.held ? ' held' : ''}${H.tone && H.tone !== 'gold' ? ` t-${H.tone}` : ''}`);
    this.hintWorld!.setAttribute('d', d); this.hintGlyph!.setAttribute('d', H.glyph ? GLYPH[H.kind] : ''); this.hintGlyph!.setAttribute('transform', `translate(${gx.toFixed(1)},${gy.toFixed(1)}) scale(${s.toFixed(3)})`);   // (cmdring.ts plainHint: no glyph, the outline alone)
  }
  /** A ring that stays on its point (CMDRING.pin), where THIS picture's camera puts it, in the view's px: the game's own place for it (RingState.ax / ay) was
   *  worked out before the tick's camera step, and a ring drawn there swims a few px against the floor while the view moves. null = the game's place stands */
  private pinned(R: RingState | null, cam: FollowCamera, w: number, h: number, tz: number): [number, number] | null {
    if (!R?.at) return null;
    const [sx, sy, front] = cam.project(R.at, w * tz, h * tz); if (!front) return null;
    const c = { ...R }; ringPin(c, sx, sy, w * tz, h * tz); return [c.ax, c.ay];
  }
  /** `w`, `h`: the layer's own size · `tz`: the factor it is zoomed by (ui/textsize.ts) */
  update(game: Game, cam: FollowCamera, w: number, h: number, tz: number) {
    this.updateHint(game, cam, w, h, tz);
    const V0 = CMDRING.on || game.orders.cmdRing ? game.orders.ringView() : null, P = V0 && CMDRING.pin ? this.pinned(game.orders.cmdRing, cam, w, h, tz) : null, V = V0 && P ? { ...V0, x: P[0], y: P[1] } : V0;
    const H0 = game.player.sel.ring ? handRingView(game) : null, HP = H0 ? this.pinned(game.player.sel.ring, cam, w, h, tz) : null, H = H0 && HP ? { ...H0, x: HP[0], y: HP[1] } : H0;   // (his own ring on its thing: handring.ts onThing)
    this.orders.draw(V, tz); this.hands.draw(H, tz); this.clear(V, H, tz);
    const D = this.discs; D.length = 0; for (const R of [V, H]) if (R) D.push(R.x / tz, R.y / tz, R1 * R.zoom / tz);
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
  private clear(V: RingView | null, H: RingView | null, tz: number) {
    const rail = V || H ? document.getElementById('railprompt') : null, k = (R: RingView | null) => R ? `${(R.x / 4).toFixed(0)},${(R.y / 4).toFixed(0)},${R.zoom.toFixed(2)}` : '';   // (to 4 px: a ring that stays on its point moves a little every frame the view does, and what it stands over is asked of the layout)
    const key = V || H ? `${k(V)}|${k(H)}|${rail?.classList.contains('on') ? rail.textContent?.length : ''}|${innerHeight}` : ''; if (key === this.overKey) return; this.overKey = key;
    const s = key ? this.svg.getBoundingClientRect() : null;
    /** a ring's words as a box of the window: wider than tall, and down to under its caption when `cap` */
    const box = (R: RingView, cap: boolean) => { const cx = s!.left + R.x / tz, cy = s!.top + R.y / tz, z = R.zoom / tz; return { l: cx - RING_BOX.x * z, r: cx + RING_BOX.x * z, t: cy - RING_BOX.y * z, b: cy + (cap ? RC + 14 : RING_BOX.y) * z }; };
    const col = V ? document.getElementById('interact')?.getBoundingClientRect() : null, v = V && col && col.width > 0 && col.height > 0 ? box(V, false) : null;
    document.body.classList.toggle('ringover', !!v && v.l < col!.right && v.r > col!.left && v.t < col!.bottom && v.b > col!.top);
    let shift = 0;
    if (rail?.classList.contains('on')) { const b = rail.getBoundingClientRect(), cur = new DOMMatrixReadOnly(getComputedStyle(rail).transform).m42 * tz, top = b.top - cur, bottom = b.bottom - cur;   // (where the line is at rest: measured less the step it is on now — it may be part way there)
      for (const R of [V, H]) { if (!R) continue; const q = box(R, !!R.caption); if (!(b.width > 0 && q.l < b.right && q.r > b.left && q.t < bottom && q.b > top)) continue;
        const down = q.b + 8 - top, up = q.t - 8 - bottom; shift = bottom + down <= innerHeight - 8 ? down : up; break; } }
    if (shift !== this.railShift) { this.railShift = shift; const el = document.getElementById('railprompt'); if (el) el.style.setProperty('--railshift', `${(shift / tz).toFixed(1)}px`); }
  }
}
