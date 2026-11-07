// THE LIGHTS onto the renderer's set, from their section: a light is the same object for as long as it burns (the set knows last frame's slots by it), and its
// numbers are written into it.
import type { LightClass } from '../render/photometry';
import type { LightSet, RtLight } from '../render/lights';
import type { Drawn } from './drawn';
import { Light, Section } from './sections';

const CLASSES: readonly LightClass[] = ['none', 'fixture', 'hand', 'vehicle', 'flash', 'face'];
const BURNS = 1, TRANSIENT = 2, BROAD = 4, CARD = 8, IR = 16, MOVES = 32;

export class DrawLights {
  private by = new Map<number, RtLight>();
  private seen = new Set<number>();
  constructor(private set: LightSet, private D: Drawn) {}
  from(): void {
    const D = this.D, f = D.f, u = D.u, L = this.set.lights, I = this.set.ir, n = D.n[Section.lights]; let a = 0, b = 0;
    this.seen.clear();
    for (let i = 0; i < n; i++) { const o = D.at[Section.lights] + i * Light.row, key = u[o + Light.key], says = u[o + Light.says]; this.seen.add(key);
      let l = this.by.get(key);
      if (!l) { l = { name: '', group: '', kind: 0, pos: [0, 0, 0], dir: [0, -1, 0], color: [0, 0, 0], intensity: 0, range: 0, innerDeg: 0, outerDeg: 0, radius: 0, volumetric: 0, owner: 0, enabled: false, fixtureBox: -1, flicker: 0, ttl: -1, age: 0, peakIntensity: 0, steady: undefined, transient: undefined, broad: false, markRef: undefined, card: false, slot: undefined, glassMask: undefined, fog: undefined, fogGhost: undefined, class: undefined }; this.by.set(key, l); }
      for (let k = 0; k < 3; k++) { l.pos[k] = f[o + Light.pos + k]; l.dir[k] = f[o + Light.dir + k]; l.color[k] = f[o + Light.color + k]; }
      l.kind = u[o + Light.kind] as 0; l.class = CLASSES[u[o + Light.class]]; l.enabled = (says & BURNS) !== 0; l.transient = (says & TRANSIENT) !== 0; l.broad = (says & BROAD) !== 0; l.card = (says & CARD) !== 0;
      l.group = (says & MOVES) !== 0 ? 'flashlights' : ''; l.owner = u[o + Light.owner]; l.markRef = (u[o + Light.mark] | 0) < 0 ? undefined : u[o + Light.mark];
      l.intensity = f[o + Light.intensity]; l.peakIntensity = f[o + Light.peak]; l.range = f[o + Light.range]; l.innerDeg = f[o + Light.inner]; l.outerDeg = f[o + Light.outer]; l.radius = f[o + Light.radius];
      l.volumetric = f[o + Light.volumetric]; l.flicker = f[o + Light.flicker]; l.ttl = f[o + Light.ttl]; l.age = f[o + Light.age]; l.fog = f[o + Light.fog]; l.fogGhost = f[o + Light.ghost];
      if (says & IR) I[b++] = l; else L[a++] = l; }
    L.length = a; I.length = b;
    if (this.by.size > n) for (const k of this.by.keys()) if (!this.seen.has(k)) this.by.delete(k);
  }
}
