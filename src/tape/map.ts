// THE MAP OF A RUN BEING PLAYED AGAIN (tape/minimap.ts draws it): small in the corner, large over the picture on a press of it or on M. The floor and who stands on it are asked of the game as it plays.
import type { ReplayNote } from './format';
import { drawFloor, drawScene, mapFit, MAP_RING, mapSize, mapToWorld, type MapDoor, type MapFit, type MapMan, type MapScene } from './minimap';

export interface MapDeps {
  floor(): { bounds: [number, number, number, number]; boxes: number[] };
  now(): { men: (Omit<MapMan, 'ring'> & { state: string })[]; doors: MapDoor[] };
  /** where the picture looks, and which way */
  view(): { look: ArrayLike<number>; yaw: number };
  notes: readonly ReplayNote[];
  /** the picture goes to look at a spot of the floor */
  to(x: number, z: number): void;
  /** the tick it stands after */
  at(): number;
}
/** milliseconds between two looks at who stands where · ticks after which the floor is asked again (walls come down) */
const EVERY = 100, FLOOR = 300;
const CSS = `
#tapemap{position:fixed;right:12px;top:12px;z-index:70;border:1px solid rgb(72,73,70);border-radius:3px;background:var(--h-base,#0c0b09);cursor:pointer}
#tapemap:hover{border-color:var(--m-accent,#e8833a)}
#tapemapbig{position:fixed;left:0;right:0;top:0;bottom:var(--tape-h,44px);z-index:70;display:grid;place-items:center;background:var(--h-base,#0c0b09);cursor:pointer}
#tapemapbig[hidden]{display:none}
#tapemapbig canvas{border:1px solid rgb(72,73,70);border-radius:3px;background:var(--h-base,#0c0b09);cursor:crosshair}
body:has(#panel:not(.hidden)) #tapemap{right:calc(380px * var(--tz, 1) + 12px)}
body.taping #checkpoint{right:calc((var(--tape-map,236px) + 26px) / var(--tz, 1))}`;

interface Laid { c: HTMLCanvasElement; W: number; H: number; dpr: number; fit: MapFit; floor: object }
export class TapeMap {
  private small = document.createElement('canvas'); private large = document.createElement('canvas'); private modal = document.createElement('div');
  private floor: ReturnType<MapDeps['floor']>; private floorAt: number; private laid = new Map<HTMLCanvasElement, Laid>(); private drawn = -1e9;
  constructor(private d: MapDeps) {
    const css = document.createElement('style'); css.textContent = CSS; document.head.append(css);
    this.floor = d.floor(); this.floorAt = d.at();
    this.small.id = 'tapemap'; this.small.title = 'M'; this.modal.id = 'tapemapbig'; this.modal.hidden = true; this.modal.append(this.large);
    { const [w, h] = mapSize(this.floor.bounds, 236, 176, 6); this.small.style.width = `${w}px`; this.small.style.height = `${h}px`; document.documentElement.style.setProperty('--tape-map', `${w}px`); }
    this.small.onclick = () => { this.open(true); };
    this.modal.onclick = e => { const L = this.laid.get(this.large); if (e.target === this.large && L) { const r = this.large.getBoundingClientRect(), [x, z] = mapToWorld(L.fit, e.clientX - r.left - this.large.clientLeft, e.clientY - r.top - this.large.clientTop); d.to(x, z); } this.open(false); };
    for (const e of [this.small, this.modal]) for (const ev of ['mousedown', 'pointerdown', 'contextmenu', 'wheel']) e.addEventListener(ev, x => x.stopPropagation());
    document.body.append(this.small, this.modal); window.addEventListener('resize', () => this.lay());
  }
  get up(): boolean { return !this.modal.hidden; }
  /** large, or put away (no word: the other way) */
  open(on = !this.up): void { if (on === this.up) return; this.modal.hidden = !on; if (on) { this.lay(); this.draw(); this.drawn = -1e9; } }
  private lay(): void { if (!this.up) return; const [w, h] = mapSize(this.floor.bounds, window.innerWidth * 0.92, this.modal.clientHeight * 0.92, 26); this.large.style.width = `${w}px`; this.large.style.height = `${h}px`; }
  /** every frame */
  frame(): void {
    const now = performance.now(); if (now - this.drawn < EVERY) return; this.drawn = now;
    if (this.d.at() < this.floorAt || this.d.at() - this.floorAt > FLOOR) { this.floor = this.d.floor(); this.floorAt = this.d.at(); }
    const M = this.d.now(), v = this.d.view(), notes = this.d.notes.flatMap((n, i) => n.pos ? [{ x: n.pos[0], z: n.pos[1], i: i + 1 }] : []);
    const scene: MapScene = { men: M.men.map(m => ({ ...m, ring: m.team === 'operative' ? null : MAP_RING[m.state] ?? null })), doors: M.doors, shots: [], notes, cam: { x: v.look[0], z: v.look[2], fx: -Math.sin(v.yaw), fz: -Math.cos(v.yaw) } };
    this.scene = scene; this.draw();
  }
  private scene: MapScene | null = null;
  private draw(): void {
    const scene = this.scene; if (!scene) return;
    for (const big of [false, true]) { const cv = big ? this.large : this.small; if (big && !this.up) continue;
      const W = cv.clientWidth, H = cv.clientHeight, dpr = Math.min(2, window.devicePixelRatio || 1); if (!W || !H) continue;
      let L = this.laid.get(cv);
      if (!L || L.W !== W || L.H !== H || L.dpr !== dpr || L.floor !== this.floor) {
        cv.width = Math.round(W * dpr); cv.height = Math.round(H * dpr); const c = document.createElement('canvas'); c.width = cv.width; c.height = cv.height;
        const fit = mapFit(this.floor.bounds, W, H, big ? 26 : 6), f = c.getContext('2d'); if (f) drawFloor(f, W, H, dpr, { ...this.floor, doors: [] }, fit); L = { c, W, H, dpr, fit, floor: this.floor }; this.laid.set(cv, L);
      }
      const x = cv.getContext('2d'); if (!x) continue; x.setTransform(1, 0, 0, 1, 0, 0); x.clearRect(0, 0, cv.width, cv.height); x.drawImage(L.c, 0, 0); drawScene(x, dpr, scene, L.fit, big);
    }
  }
}
