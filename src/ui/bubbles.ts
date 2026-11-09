// Speech bubbles: what is being said on the floor, as the game tells the seat's HUD (game.view.bubbles: who says it, how it sounds and where it goes). What is
// worked out here is where that falls on this page's screen, by its own camera and the men as the picture drew them.
// One absolutely-positioned element per speaker inside the overlay root; projection matches the state markers so the bubble sits
// just above the ! / ? glyph. Kept deliberately small and monochrome — a caption, not a comic balloon.
import type { Vec3 } from '../math/vec';
import type { HudCam, HudSource } from '../view/hud';
import type { BubbleView } from '../view/seatview';
import { NET_ROW, netPlace } from './netplace';
import type { BubblesLike } from '../host/hud';
import { rectZoom, textZoom } from './textsize';

/** the HUD cards a bubble must never sit under (the maintainer's rule): the interaction panel while it is up ('on'), and the kit column,
 *  the meters, the objective line and the radio strip whenever they show anything (review U9.12) */
const BLOCKER_IDS_ON = ['interact'], BLOCKER_IDS = ['equip', 'meters', 'objective', 'log'];
/** the blockers' rects are MEASURED only when what they show changes (or the view's size does), and at most every so often besides — not three forced layouts a
 *  frame on the heels of the overlay's writes (review U9.4) */
const BLOCKER_RECHECK_MS = 1000;

/** on this view: where a line is on the screen or off it, a little inside the border */
function inView(vp: ArrayLike<number>, p: Vec3): boolean {
  const w = vp[3] * p[0] + vp[7] * p[1] + vp[11] * p[2] + vp[15]; if (w <= 0) return false;
  const x = (vp[0] * p[0] + vp[4] * p[1] + vp[8] * p[2] + vp[12]) / w, y = (vp[1] * p[0] + vp[5] * p[1] + vp[9] * p[2] + vp[13]) / w;
  return x > -0.92 && x < 0.92 && y > -0.9 && y < 0.9;
}

export class Bubbles implements BubblesLike {
  private els: HTMLElement[] = [];
  /** the cached blocker rects (canvas-relative), the key they were measured under, and when */
  private blockers: DOMRect[] = []; private blockKey = ''; private blockAt = -1e9;
  private blockEls: (HTMLElement | null)[] | null = null;
  constructor(private root: HTMLElement) {}

  private make(): HTMLElement { const el = document.createElement('div'); el.className = 'bubble'; el.style.display = 'none'; this.root.appendChild(el); return el; }

  /** the rects to dodge this frame: re-measured when a card's class / content or the view's size changed (a cheap string key off the DOM, no layout read), or every BLOCKER_RECHECK_MS */
  private blockerRects(canvas: HTMLCanvasElement, w: number, h: number): DOMRect[] {
    this.blockEls ??= [...BLOCKER_IDS_ON, ...BLOCKER_IDS].map(id => document.getElementById(id));
    let key = `${w}x${h}`;
    for (const e of this.blockEls) key += !e ? '|-' : e.id === 'meters' ? `|${e.className}` : `|${e.className}:${e.childElementCount}:${(e.textContent ?? '').length}`;   // (the meters' numbers move every frame but their card never changes size: its class is key enough)
    const now = performance.now();
    if (key === this.blockKey && now - this.blockAt < BLOCKER_RECHECK_MS) return this.blockers;
    this.blockKey = key; this.blockAt = now;
    const cr = canvas.getBoundingClientRect(), tz = rectZoom(); const out: DOMRect[] = [];
    this.blockEls.forEach((e, i) => {
      if (!e) return;
      if (i < BLOCKER_IDS_ON.length && !e.classList.contains('on')) return;   // (the interaction panel: only while up)
      const r = e.getBoundingClientRect(); if (r.width > 0 && r.height > 0) out.push(new DOMRect((r.left - cr.left) / tz, (r.top - cr.top) / tz, r.width / tz, r.height / tz));
    });
    return this.blockers = out;
  }

  /** where a point BEHIND the eye's plane lies on the screen's plane, for a way to it only: the undivided clip direction pushed far off the view
   *  (dividing by a w below zero would mirror it) — as ui/overlay.ts pins the objective's mark */
  private behind(cam: HudCam, p: Vec3, w: number, h: number): [number, number] {
    const vp = cam.viewProj, nx = vp[0] * p[0] + vp[4] * p[1] + vp[8] * p[2] + vp[12], ny = vp[1] * p[0] + vp[5] * p[1] + vp[9] * p[2] + vp[13], l = Math.hypot(nx, ny) || 1;
    return [(nx / l * 9 * 0.5 + 0.5) * w, (1 - (ny / l * 9 * 0.5 + 0.5)) * h];
  }
  /** the objective's mark while it is drawn (ui/overlay.ts puts it by left / top, in this layer's pixels): its diamond and the metres under it */
  private objEl: HTMLElement | null = null;
  private objRect(): { left: number; right: number; top: number; bottom: number }[] {
    this.objEl ??= document.querySelector<HTMLElement>('.objmark');
    const e = this.objEl; if (!e || e.style.display === 'none') return [];
    const x = parseFloat(e.style.left), y = parseFloat(e.style.top); return Number.isFinite(x) && Number.isFinite(y) ? [{ left: x - 26, right: x + 26, top: y - 10, bottom: y + 24 }] : [];
  }

  update(src: HudSource, cam: HudCam, canvas: HTMLCanvasElement) {
    const tz = textZoom(), w = canvas.clientWidth / tz, h = canvas.clientHeight / tz;   // (the layer's own pixels: ui/textsize.ts)
    // screen rects the bubbles must dodge this frame (canvas-relative): the interaction panel while it lists something, the kit column, the meters, the objective line, the radio strip
    const blockers = this.blockerRects(canvas, w, h);
    const V = src.view, net = V.look.net;
    /** draw `el` as bubble `b` over `head` in the tone class `tone`; false = off screen this frame (the caller hides it) */
    const place = (el: HTMLElement, b: BubbleView, head: Vec3): boolean => {
      const { age } = b, tone = b.tone === 'own' ? 'sam' : b.tone;
      let [x, y, vis] = cam.project([head[0], head[1] + 0.62, head[2]], w, h, 1.05);   // a hand above the state glyph (which rides at +0.45); gone once 5 % of the view past its edge
      if (!vis) return false;
      // never under the HUD: if the bubble's box (drawn up-and-centred from x,y) would land inside the top-left interaction panel, the kit
      // column, the meters, the objective line or the radio strip, hang it BELOW his head instead (flipped tail), and keep it inside the view's width either way
      const bw = Math.min(190, 12 + b.text.length * 5.6), bh = b.text.length > 34 ? 36 : 22;   // (a cheap estimate of the box; the real one is measured after layout, too late for this frame)
      let below = false;
      for (const r of blockers) { if (x + bw / 2 > r.left && x - bw / 2 < r.right && y > r.top && y - bh < r.bottom + 4) { below = true; break; } }
      if (below) { const [, yb] = cam.project([head[0], head[1] - 0.55, head[2]], w, h, Infinity); y = yb + bh + 10; }   // chest height, box hanging under it
      x = Math.max(bw / 2 + 6, Math.min(w - bw / 2 - 6, x));
      // ease in over 0.12 s (rise 6 px + fade), hold, fade over the last 0.35 s — a short line (Sam's beat, ~1 s) fades over its last quarter instead, so it stands legible for most of its life
      const aIn = Math.min(1, age / 0.12), aOut = Math.min(1, (b.dur - age) / Math.max(0.05, Math.min(0.35, 0.25 * b.dur))); const a = Math.max(0, Math.min(aIn, aOut));
      const text = (b.radio ? '·)) ' : '') + b.text;
      if (el.textContent !== text) el.textContent = text;
      const cls = 'bubble ' + tone + (b.radio ? ' radio' : '') + (below ? ' below' : '');
      if (el.className !== cls) el.className = cls;
      el.style.display = 'block'; el.style.left = `${Math.round(x)}px`; el.style.top = `${Math.round(y - (1 - aIn) * 6)}px`; el.style.opacity = a.toFixed(2);
      return true;
    };
    // A TEAMMATE'S LINE FROM OFF THE SCREEN (rust/game_rules/src/squadnet): drawn at the edge of the view on the side he is on (`net.edge` — off: a short way out
    // from you on that side), under his name, with an arrow his way: `line` = the words with his name. Where you are not on the screen yourself (the
    // plan's camera, a tutorial's, a man watched) the way to him is laid from the middle of the view instead. netplace.ts says where, and
    // keeps it off the lines already drawn this frame and the HUD's cards; at the edge it is kept off your own figure and the objective's mark too.
    // False = he stands where the line would be drawn from (no side to draw it on).
    const nearAt: [number, number, number][] = [];   // (the far lines drawn so far this frame: x, y, width)
    const near = (el: HTMLElement, b: BubbleView, at: Vec3): boolean => {
      const pl = src.at(V.me) ?? at, [sx0, sy0, sOn] = cam.project([pl[0], pl[1] + 1.1, pl[2]], w, h, 0.9), px = sOn ? sx0 : w / 2, py = sOn ? sy0 : h / 2;
      let [mx, my, front] = cam.project(at, w, h, Infinity); const { age } = b, bh = 22;
      if (!front) { if (!net.edge) return false; [mx, my] = this.behind(cam, at, w, h); }   // (behind the eye's plane, far down the screen with the view zoomed in: at the edge his way, not dropped)
      const sx = mx - px, sy = my - py;
      const text = `${'→↘↓↙←↖↑↗'[((Math.round(Math.atan2(sy, sx) / (Math.PI / 4)) % 8) + 8) % 8]} ${b.radio ? '·)) ' : ''}${b.edge}`, bw = 14 + text.length * 5.8;
      const keep: { left: number; right: number; top: number; bottom: number }[] = net.edge ? this.objRect() : [];
      if (net.edge && sOn) keep.push({ left: px - 16, right: px + 16, top: py - 30, bottom: py + 30 });   // (your own figure, about)
      const over = (r: { left: number; right: number; top: number; bottom: number }, x: number, y: number) => x + bw / 2 > r.left && x - bw / 2 < r.right && y > r.top && y - bh < r.bottom + 4;
      const hits = (x: number, y: number) => nearAt.some(q => Math.abs(q[1] - y) < NET_ROW - (net.edge ? 0.5 : 0) && Math.abs(q[0] - x) < (q[2] + bw) / 2 + 4) || blockers.some(r => over(r, x, y)) || keep.some(r => over(r, x, y));
      const spot = netPlace(net, w, h, px, py, mx, my, bw, bh, hits); if (!spot) return false;
      const [x, y] = spot;
      nearAt.push([x, y, bw]);
      const aIn = Math.min(1, age / 0.12), aOut = Math.min(1, (b.dur - age) / Math.max(0.05, Math.min(0.35, 0.25 * b.dur))), a = Math.max(0, Math.min(aIn, aOut));
      if (el.textContent !== text) el.textContent = text;
      const cls = 'bubble net ' + b.tone + (b.radio ? ' radio' : ''); if (el.className !== cls) el.className = cls;
      el.style.display = 'block'; el.style.left = `${Math.round(x)}px`; el.style.top = `${Math.round(y)}px`; el.style.opacity = a.toFixed(2);
      return true;
    };
    const rows = V.bubbles;
    while (this.els.length < rows.length) this.els.push(this.make());
    for (let i = rows.length; i < this.els.length; i++) this.els[i].style.display = 'none';
    rows.forEach((b, i) => { const el = this.els[i], man = b.man, feet = man === null ? null : src.at(man), bone = man === null || !feet ? null : src.head(man);
      const at = feet ?? b.at, head = bone ?? (man === null ? [at[0], at[1] + 0.9, at[2]] as Vec3 : at);
      // (the point a man's line is on or off the screen by: his head, or where it would be)
      const onScreen = () => inView(cam.viewProj, bone ?? (man === null ? head : [at[0], at[1] + 1.7, at[2]]));
      const drawn = b.place === 'over' ? place(el, b, head) : b.place === 'overOrEdge' && onScreen() ? place(el, b, head) : near(el, b, at);
      if (!drawn) el.style.display = 'none'; });
  }
}
