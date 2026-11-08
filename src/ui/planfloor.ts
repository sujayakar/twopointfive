// The squad's plan drawn on the floor (`view.plan` says WHAT; this file draws it): per man of ours with a mark — the dotted way he will walk, a ring
// where he will stand with his two-letter tag, a fan for what he watches, a thin line to the door he is on — in his pair's colour (the HUD's green for a
// squad with no pairs), and a ring at the feet of each man an open order menu goes to (and a gold one at the feet of whoever's menu it is). Floor geometry projected through the camera into the overlay's SVG,
// under the close-in windows and the throw arc; drawn through walls (it is our own plan). Something waiting on the player's key has a bar across its
// doorway (along its wall's foot), and the key on the one the key would send now. No words but that key and the tags.
// A GAME OF SEVERAL PLAYERS: the other players' squads' plans are drawn too (`plan.others`), FAINTER (`look.plan.others`) — all but a throw, whose arc and reach are as strong
// on every page: where a frag lands is everybody's to know. The maintainer, 11-03, playing two seats: "seeing your teammates squad path overlays (but probably fainter than yours) would be great".
// THE WAY A MAN WILL WALK IS THE ONE THING WORKED OUT HERE, and it is a search: his own path if the view has one for this order, else `src.ask.way` for each leg, the answer kept by the
// page's clock (`look.plan.reaskSecs`), never by the game.
// index.html `.plan …` is the look; `view.plan` none (`?noplan`) = the group stays empty. An attribute is written only when it changes. Its numbers are the PLAN table's, told in `view.look.plan`.
import type { Vec3 } from '../math/vec';
import type { HudCam, HudSource } from '../view/hud';
import type { PlanMan, PlanWay, SeatPlan, TickView } from '../view/read';

const NS = 'http://www.w3.org/2000/svg';
const Y = 0.04;          // just off the floor
const RING_N = 48, FAN_N = 5;   // (a ring is a floor circle through the camera: enough points that no corner shows at any zoom)
const NEAR_W = 0.5;      // a point this near the camera's own plane (or behind it) is not drawn: its projection is anywhere
const TAG_DROP = 15;     // px: a standing man's tag sits under his ring, clear of his body
const flat = (a: Vec3, b: Vec3) => Math.hypot(a[0] - b[0], a[2] - b[2]);
/** the floor's answer for one leg of a man's way, kept a while by whoever draws: the PICTURE'S own, a game drawn and the same game not drawn hold the same state */
interface PlanLeg { key: string; t: number; pts: Vec3[] | null; /** where the leg was asked from */ from: Vec3 }
export type KeptLegs = WeakMap<HudSource, Map<number, PlanLeg[]>>;
function leg(kept: KeptLegs, src: HudSource, reask: number, budget: number, man: number, k: number, a: Vec3, b: Vec3): PlanLeg {
  let G = kept.get(src); if (!G) kept.set(src, G = new Map());
  let L = G.get(man); if (!L) G.set(man, L = []);
  const key = `${b[0].toFixed(1)},${b[2].toFixed(1)}`, c = L[k], dt = c ? src.time - c.t : -1;
  // (dt < 0: the clock went back on a load · asked from somewhere else: an answer from a time line a restore left behind, the store being the picture's)
  if (c && c.key === key && dt >= 0 && dt < reask && flat(a, c.from) < 2.5) return c;
  return L[k] = { key, t: src.time, pts: src.ask.way(a, b, budget), from: [a[0], 0, a[2]] };   // (null: the floor has no way, nothing is drawn, never a line through a wall)
}
/** THE WAY A MAN WILL WALK from where he stands to his stops: his feet, every corner, each stop; his own path where he has one, else the floor is asked, the answer kept in `kept` for `reask` seconds. Empty where the floor has none */
export function walkOf(kept: KeptLegs, src: HudSource, reask: number, budget: number, man: number, W: PlanWay): Vec3[] {
  const p = W.from, stops = W.stops, out: Vec3[] = [[p[0], 0, p[2]]];
  const add = (q: Vec3) => { const l = out[out.length - 1]; if (flat(l, q) > 0.05) out.push([q[0], 0, q[2]]); };
  if (W.own) for (const q of W.own) add(q);
  else {
    const first = leg(kept, src, reask, budget, man, 0, p, stops[0]), pts = first.pts; if (!pts) return [];
    // an answer kept from a stride ago starts behind him: from the end of the leg he is nearest on — looked for over its first two metres only, never 'the
    // corner nearest him' of its first six (a way that doubles back passes near him again further on: the corners between were skipped and the line cut the wall)
    let k = 0, best = Infinity, arc = 0, a = first.from;
    for (let i = 0; i < pts.length && arc <= 2; i++) { const b = pts[i], dx = b[0] - a[0], dz = b[2] - a[2], l2 = dx * dx + dz * dz, u = l2 > 1e-9 ? Math.min(1, Math.max(0, ((p[0] - a[0]) * dx + (p[2] - a[2]) * dz) / l2)) : 0, d = Math.hypot(p[0] - a[0] - dx * u, p[2] - a[2] - dz * u); if (d < best - 1e-6) { best = d; k = i; } arc += Math.sqrt(l2); a = b; }
    for (let i = k < pts.length - 1 && flat(p, pts[k]) < 0.3 ? k + 1 : k; i < pts.length; i++) add(pts[i]);
  }
  add(stops[0]);
  for (let k = 1; k < stops.length; k++) { const pts = leg(kept, src, reask, budget, man, k, stops[k - 1], stops[k]).pts; if (!pts) return []; for (const q of pts) add(q); add(stops[k]); }
  return out;
}
const legs: KeptLegs = new WeakMap();
/** the way an order walks a man from where he stands to his mark: his feet, then every corner, then the mark — empty where the floor has none */
export const wayOf = (src: HudSource, reask: number, man: number, W: PlanWay): Vec3[] => walkOf(legs, src, reask, 3000, man, W);
/** a man as the frame draws him: `path` his way (empty: he is on it, the fight has him off his order, or the floor has no way) */
export type DrawnMan = PlanMan & { path: Vec3[] };
export type DrawnPlan = Omit<SeatPlan, 'men'> & { men: DrawnMan[] };
const drawn = (src: HudSource, reask: number, P: SeatPlan): DrawnPlan => ({ ...P, men: P.men.map(m => ({ ...m, path: m.way ? wayOf(src, reask, m.man, m.way) : [] })) });
/** THE PLANS A FRAME DRAWS, the page's own seat's first and whole, at `k` 1; then each other player's at `k`, the table's `others`. Asked by the checks too */
export function plansToDraw(src: HudSource, view: TickView = src.view): { P: DrawnPlan; k: number }[] {
  const V = view.plan, { others: faint, reaskSecs: reask } = view.look.plan; if (!V) return [];
  return [{ P: drawn(src, reask, V.own), k: 1 }, ...(faint > 0 ? V.others.map(P => ({ P: drawn(src, reask, P), k: faint })) : [])];
}

interface ManEls { g: SVGGElement; way: SVGPathElement; mark: SVGPolygonElement; fan: SVGPolygonElement; sight: SVGLineElement; feet: SVGPolygonElement; tag: SVGTextElement }
interface ThrowEls { g: SVGGElement; reach: SVGPolygonElement; walk: SVGPathElement; spot: SVGPolygonElement; who: SVGPolygonElement; arc: SVGPathElement; land: SVGPolygonElement; tag: SVGTextElement }
interface DoorEls { g: SVGGElement; bar: SVGLineElement; cap: SVGRectElement; key: SVGTextElement }

export class PlanFloor {
  private g: SVGGElement;
  private men: ManEls[] = []; private menShown = 0;
  private who: SVGPolygonElement[] = []; private whoShown = 0;
  private you: SVGPolygonElement | null = null;
  /** the gold ring at the seat's own man is drawn this frame (ui/findring.ts stands its always-on ring down under it) */
  youOn = false;
  private doors: DoorEls[] = []; private doorsShown = 0;
  private throws: ThrowEls[] = []; private throwsShown = 0;
  private was = new WeakMap<Element, Record<string, string>>();
  constructor(svg: SVGSVGElement) {
    this.g = document.createElementNS(NS, 'g') as SVGGElement; this.g.setAttribute('class', 'plan');
    svg.insertBefore(this.g, svg.firstChild);   // the floor's plan first: everything else of the overlay's SVG draws over it
  }
  /** an attribute (`k` 'display' / 'opacity': the style), written only when it changed */
  private put(e: SVGElement, k: string, v: string) {
    let c = this.was.get(e); if (!c) this.was.set(e, c = {});
    if (c[k] === v) return; c[k] = v;
    if (k === 'display') e.style.display = v; else if (k === 'opacity') e.style.opacity = v; else if (k === 'text') e.textContent = v; else e.setAttribute(k, v);
  }
  private show(e: SVGElement, on: boolean) { this.put(e, 'display', on ? 'block' : 'none'); }
  private el<K extends keyof SVGElementTagNameMap>(parent: SVGElement, tag: K, cls: string): SVGElementTagNameMap[K] {
    const e = document.createElementNS(NS, tag) as SVGElementTagNameMap[K]; this.put(e, 'class', cls); this.show(e, false); parent.appendChild(e); return e;
  }
  private manEls(i: number): ManEls {
    let m = this.men[i];
    if (!m) {
      const g = document.createElementNS(NS, 'g') as SVGGElement; this.g.appendChild(g);
      m = { g, fan: this.el(g, 'polygon', 'pf'), sight: this.el(g, 'line', 'pl'), way: this.el(g, 'path', 'pp'), mark: this.el(g, 'polygon', 'pm'), feet: this.el(g, 'polygon', 'pw'), tag: this.el(g, 'text', 'pt') };
      m.tag.setAttribute('text-anchor', 'middle'); m.tag.setAttribute('dominant-baseline', 'middle');
      this.men[i] = m;
    }
    return m;
  }
  private throwEls(i: number): ThrowEls {
    let E = this.throws[i];
    if (!E) {
      const g = document.createElementNS(NS, 'g') as SVGGElement; this.g.appendChild(g);
      E = { g, reach: this.el(g, 'polygon', 'pr'), walk: this.el(g, 'path', 'pp'), spot: this.el(g, 'polygon', 'pm'), who: this.el(g, 'polygon', 'pw thr'), arc: this.el(g, 'path', 'pa'), land: this.el(g, 'polygon', 'pe'), tag: this.el(g, 'text', 'pt') };
      E.tag.setAttribute('text-anchor', 'middle'); E.tag.setAttribute('dominant-baseline', 'middle');
      this.throws[i] = E;
    }
    return E;
  }
  private capSetup(E: { cap: SVGRectElement; key: SVGTextElement }) { E.key.setAttribute('text-anchor', 'middle'); E.key.setAttribute('dominant-baseline', 'central'); E.cap.setAttribute('rx', '2.5'); E.cap.setAttribute('width', '15'); E.cap.setAttribute('height', '15'); }
  private doorEls(i: number): DoorEls {
    let E = this.doors[i];
    if (!E) {
      const g = document.createElementNS(NS, 'g') as SVGGElement; this.g.appendChild(g);
      E = { g, bar: this.el(g, 'line', 'pd'), cap: this.el(g, 'rect', 'pk'), key: this.el(g, 'text', 'pkt') };
      this.capSetup(E);
      this.doors[i] = E;
    }
    return E;
  }
  update(src: HudSource, view: TickView, cam: HudCam, w: number, h: number) {
    const PLAN = view.look.plan, bc = document.body.classList, plans = !(bc.contains('blinded') || bc.contains('attract')) ? plansToDraw(src, view) : [];   // (hidden by the page under those: nothing asked)
    let n = 0, k = 0, nd = 0, nt = 0, you = false;
    for (const { P, k: dim } of plans) {
      const f1 = (v: number) => v.toFixed(1);
      const px = (x: number, z: number): [number, number] | null => { const [sx, sy, v] = cam.project([x, Y, z], w, h, Infinity, NEAR_W); return v ? [sx, sy] : null; };
      const pt = (x: number, z: number): string | null => { const q = px(x, z); return q ? `${f1(q[0])},${f1(q[1])}` : null; };
      const ring = (c: Vec3, r: number, N = RING_N): string | null => {
        const out: string[] = [];
        for (let i = 0; i < N; i++) { const a = (i / N) * Math.PI * 2, s = pt(c[0] + Math.sin(a) * r, c[2] + Math.cos(a) * r); if (!s) return null; out.push(s); }
        return out.join(' ');
      };
      const half = PLAN.sectorDeg * Math.PI / 180;
      for (const m of P.men) {
        const E = this.manEls(n++); this.show(E.g, true);
        this.put(E.g, 'class', m.tone ? `t-${m.tone}` : '');
        const fresh = Math.max(0, 1 - m.age / Math.max(0.1, PLAN.freshSecs));
        const walking = !m.there && m.path.length > 1;
        const k0 = (m.held ? 0.6 : 1) * (walking ? PLAN.walkDim + (1 - PLAN.walkDim) * fresh : PLAN.restDim + (1 - PLAN.restDim) * fresh);
        this.put(E.g, 'opacity', ((P.mode ? k0 : k0 * Math.min(1, fresh / 0.4)) * dim).toFixed(2));   // (outside the order mode an order just given goes out with its age)
        const c = px(m.mark[0], m.mark[2]);
        // the way: broken, never bridged, where a point of it cannot be projected
        let d = '';
        if (walking) { let pen = false, segs = 0; for (const q of m.path) { const s = pt(q[0], q[2]); if (!s) { pen = false; continue; } d += `${pen ? 'L' : 'M'}${s}`; if (pen) segs++; pen = true; } if (!segs) d = ''; }
        this.show(E.way, !!d); if (d) this.put(E.way, 'd', d);
        // the mark
        const mr = ring(m.mark, PLAN.ringM);
        this.show(E.mark, !!mr); if (mr) { this.put(E.mark, 'points', mr); this.put(E.mark, 'class', m.there ? 'pm there' : 'pm'); }
        // what he watches: a fan from the mark; a thin line on to the point while the order is fresh, and to his door for as long as he is on it
        let fan: string | null = null;
        if (m.aim !== null && c) {
          const a: string[] = [`${f1(c[0])},${f1(c[1])}`];
          for (let i = 0; i <= FAN_N; i++) { const y = m.aim - half + (2 * half * i) / FAN_N, s = pt(m.mark[0] + Math.sin(y) * PLAN.sectorM, m.mark[2] + Math.cos(y) * PLAN.sectorM); if (s) a.push(s); }
          if (a.length === FAN_N + 2) fan = a.join(' ');
        }
        this.show(E.fan, !!fan); if (fan) { this.put(E.fan, 'points', fan); this.put(E.fan, 'class', m.there ? 'pf there' : 'pf'); }
        const b = m.watch && c && (fresh > 0 || m.onDoor) ? px(m.watch[0], m.watch[2]) : null;
        this.show(E.sight, !!b); if (b && c) { this.put(E.sight, 'x1', f1(c[0])); this.put(E.sight, 'y1', f1(c[1])); this.put(E.sight, 'x2', f1(b[0])); this.put(E.sight, 'y2', f1(b[1])); }
        // who: a ring at his feet while the order is fresh and he has somewhere to go
        const fr = walking && fresh > 0 ? ring(m.path[0], PLAN.ringM * 1.35) : null;
        this.show(E.feet, !!fr); if (fr) { this.put(E.feet, 'points', fr); this.put(E.feet, 'opacity', fresh.toFixed(2)); }
        // his tag: in the ring he is walking to, under the ring he stands in
        const tag = !!m.tag && !!c;
        this.show(E.tag, tag); if (tag && c) { this.put(E.tag, 'x', f1(c[0])); this.put(E.tag, 'y', f1(c[1] + (m.there ? TAG_DROP : 0))); this.put(E.tag, 'text', m.tag); this.put(E.tag, 'class', m.there ? 'pt there' : 'pt'); }
      }
      for (const q of P.who) {
        const r = ring(q.at, PLAN.ringM * 1.35); if (!r) continue;
        let e = this.who[k]; if (!e) { e = this.el(this.g, 'polygon', 'pw'); this.who[k] = e; }
        k++; this.show(e, true); this.put(e, 'points', r); this.put(e, 'class', q.tone ? `pw t-${q.tone}` : 'pw');
      }
      // … and the one whose menu it is: the same ring, gold
      const yr = P.you ? ring(P.you, PLAN.ringM * 1.35) : null;
      if (yr) { if (!this.you) this.you = this.el(this.g, 'polygon', 'py t-gold'); this.put(this.you, 'points', yr); you = true; }
      // a teammate's throw: who (a filled ring at his feet), his way to where he throws from, the arc in the air, where it comes to rest, a frag's reach
      const way = (pts: Vec3[], air: boolean): string => {
        let d = '', pen = false, segs = 0;
        for (const q of pts) { const v = air ? cam.project([q[0], Math.max(q[1], Y), q[2]], w, h, Infinity, NEAR_W) : null, s = air ? (v![2] ? `${f1(v![0])},${f1(v![1])}` : null) : pt(q[0], q[2]); if (!s) { pen = false; continue; } d += `${pen ? 'L' : 'M'}${s}`; if (pen) segs++; pen = true; }
        return segs ? d : '';
      };
      for (const T of P.throws) {
        const E = this.throwEls(nt++); this.show(E.g, true);
        this.put(E.g, 'class', T.tone ? `t-${T.tone}` : ''); this.put(E.g, 'opacity', T.preview ? '0.60' : '0.95');   // (never fainter for being another's: see the head)
        const poly = (e: SVGPolygonElement, s: string | null) => { this.show(e, !!s); if (s) this.put(e, 'points', s); };
        const p = src.at(T.man) ?? T.from, walking = T.walk.length > 1 && (T.preview || !P.men.some(m => m.man === T.man && m.path.length > 1));   // (ordered: his way is already his order's)
        poly(E.who, ring(p, PLAN.ringM * 1.35));
        const dw = walking ? way(T.walk, false) : ''; this.show(E.walk, !!dw); if (dw) this.put(E.walk, 'd', dw);
        poly(E.spot, walking ? ring(T.from, PLAN.ringM) : null);
        const c = walking && T.tag ? px(T.from[0], T.from[2]) : null;
        this.show(E.tag, !!c); if (c) { this.put(E.tag, 'x', f1(c[0])); this.put(E.tag, 'y', f1(c[1])); this.put(E.tag, 'text', T.tag); }
        const da = way(T.arc, true); this.show(E.arc, !!da); if (da) this.put(E.arc, 'd', da);
        poly(E.land, ring(T.land, PLAN.ringM));
        poly(E.reach, T.reach > 0 ? ring(T.land, T.reach, RING_N * 2) : null);
      }
      // waiting on the key: a bar across the doorway, the key on the one the key would send now (a quiet pair's man waits on open ground: under the table's keyLit
      // the key alone, a step ahead of him — a bar there crossed the cap and read as a key struck out)
      for (const dr of P.doors) {
        const a = px(dr.a[0], dr.a[2]), b = px(dr.b[0], dr.b[2]); if (!a || !b) continue;
        const E = this.doorEls(nd++), c = dr.cap; this.show(E.g, true); this.put(E.g, 'opacity', dim.toFixed(2));
        this.put(E.g, 'class', `pdoor w-${dr.why}${dr.tone ? ` t-${dr.tone}` : ''}${dr.now ? ' now' : ''}${c === 'lit' ? ' lit live' : c === 'dark' ? ' lit' : ''}`);
        this.show(E.bar, dr.why !== 'pair'); this.put(E.bar, 'x1', f1(a[0])); this.put(E.bar, 'y1', f1(a[1])); this.put(E.bar, 'x2', f1(b[0])); this.put(E.bar, 'y2', f1(b[1]));
        const cap = c !== 'none' && !!dr.key, mx = (a[0] + b[0]) / 2, my = (a[1] + b[1]) / 2;
        this.show(E.cap, cap); this.show(E.key, cap);
        if (cap) { this.put(E.cap, 'x', f1(mx - 7.5)); this.put(E.cap, 'y', f1(my - 7.5)); this.put(E.key, 'x', f1(mx)); this.put(E.key, 'y', f1(my)); this.put(E.key, 'text', dr.key!); }
      }
    }
    for (let i = nd; i < this.doorsShown; i++) this.show(this.doors[i].g, false);
    for (let i = nt; i < this.throwsShown; i++) this.show(this.throws[i].g, false);
    for (let i = n; i < this.menShown; i++) this.show(this.men[i].g, false);
    for (let i = k; i < this.whoShown; i++) this.show(this.who[i], false);
    this.doorsShown = nd; this.throwsShown = nt; this.menShown = n; this.whoShown = k;
    if (this.you) this.show(this.you, you);
    this.youOn = you;
  }
}
