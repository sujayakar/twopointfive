// THE RING AT HIS FEET (host/findme.ts FINDME.ring, a setting of the player's): the gold ring an open order menu puts at his feet (ui/planfloor.ts `py`), drawn whenever that one is not up.
// Where it goes is where his man is drawn. The outline, the setting's other half, is the renderer's (draw/rims.ts).
import type { Vec3 } from '../math/vec';
import { FINDME } from '../host/findme';
import type { HudCam, HudSource } from '../view/hud';
import type { TickView } from '../view/read';

/** where the ring would be drawn for the seat's own man (its centre on the floor), null = none: the row off, or he is down */
export function findMeRing(src: HudSource, view: TickView = src.view): Vec3 | null {
  const at = src.at(view.me); if (!at || view.kit.down || !FINDME.ring) return null;
  return [at[0], 0, at[2]];
}

const NS = 'http://www.w3.org/2000/svg';
const CSS = `
.findme { pointer-events: none; }
.findme .fr { fill: none; stroke: #f2cf6b; stroke-width: 1.5; stroke-linejoin: round; }
`;

export class FindMeViz {
  private g: SVGGElement; private ring: SVGPolygonElement; private shown = false; private points = '';
  constructor(svg: SVGSVGElement) {
    if (!document.getElementById('findme-css')) { const s = document.createElement('style'); s.id = 'findme-css'; s.textContent = CSS; document.head.appendChild(s); }
    this.g = document.createElementNS(NS, 'g') as SVGGElement; this.g.setAttribute('class', 'findme'); this.g.style.display = 'none';
    svg.insertBefore(this.g, svg.firstChild?.nextSibling ?? null);   // (over the squad's plan on the floor, under everything else the overlay draws)
    this.ring = document.createElementNS(NS, 'polygon') as SVGPolygonElement; this.ring.setAttribute('class', 'fr'); this.g.appendChild(this.ring);
  }
  private show(on: boolean) { if (on !== this.shown) { this.shown = on; this.g.style.display = on ? 'block' : 'none'; } }
  /** `planRing`: the order menu's own gold ring is drawn this frame (planfloor.ts `youOn`): ours stands down under it */
  update(src: HudSource, view: TickView, cam: HudCam, w: number, h: number, planRing = false) {
    const bc = document.body.classList;
    if (!FINDME.ring || planRing || bc.contains('blinded') || bc.contains('attract')) { this.show(false); return; }   // (nothing asked; and under a stun's white-out or attract mode as the plan on the floor)
    const at = findMeRing(src, view); if (!at) { this.show(false); return; }
    const r = view.look.plan.ringM * 1.35, N = 48, out: string[] = [];
    for (let i = 0; i < N; i++) { const a = (i / N) * Math.PI * 2, [x, y, v] = cam.project([at[0] + Math.sin(a) * r, 0.04, at[2] + Math.cos(a) * r], w, h, Infinity, 0.5); if (!v) { this.show(false); return; } out.push(`${x.toFixed(1)},${y.toFixed(1)}`); }
    const pts = out.join(' '); if (pts !== this.points) { this.points = pts; this.ring.setAttribute('points', pts); }   // (written only when it moved: a man standing still under a still camera costs the page nothing)
    this.show(true);
  }
}
