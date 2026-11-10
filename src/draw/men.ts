// THE MEN DRAWN, from their sections: what the GPU alone reads of a man (his instance, his bones) is handed over as it lies; his materials are the page's own
// tables by what he wears, worked out again only when that changes; his proxies go to the box world's list in the colours of those.
import type { CharacterRenderer } from '../anim/characters';
import type { GltfCharacter } from '../anim/gltf';
import { CHAR_MAT_FLOATS, charMatsVersion, charSlots, proxyAlbedos, writeCharPalette, type CharSide } from '../render/charMaterials';
import { fitRig } from '../render/sdfrig';
import type { Vec3 } from '../math/vec';
import { BoxWorld } from '../scene/boxes';
import type { Drawn } from './drawn';
import { Bones, Cap, Inst, Man, Section } from './sections';

/** a man's side by its number in the section */
export const SIDES: readonly CharSide[] = ['civilian', 'operator', 'defender', 'responder'];
type Fit = { r0: Float32Array; r1: Float32Array; pad?: number };
/** what a man is dressed by, and what that came to */
interface Look { version: number; side: number; suit: number; parts: number; tints: Float32Array; dry: number; pal: Float32Array; caps: Vec3[]; stature: number; fit: Fit | null; put: (out: Float32Array, at: number) => void }

export class DrawMen {
  /** a man's own round cones by his id's low byte (the rig's fit asks it: render/sdfrig.ts byOwner) */
  readonly fits = new Map<number, Fit>();
  private looks = new Map<number, Look>();
  private ofParts = new Map<number, Fit | null>();
  constructor(private ch: GltfCharacter, private chars: CharacterRenderer, private world: BoxWorld, private D: Drawn, private materials: () => boolean) {}

  from(queue: GPUQueue): void {
    const D = this.D, f = D.f, u = D.u, n = D.n[Section.men], W = this.world, mats = this.materials();
    this.chars.drawn(n, u, D.at[Section.bones]);
    D.send(queue, this.chars.charInsts, Section.insts, Inst.row); D.send(queue, this.chars.jointMats, Section.bones, Bones.row);
    W.resetCapsules();
    for (let i = 0; i < n; i++) { const o = D.at[Section.men] + i * Man.row, L = this.lookOf(o, mats);
      this.chars.drawnParts(i, L.parts);
      this.chars.setMaterials(i, mats ? L.put : undefined);
      if (L.fit) this.fits.set(u[o + Man.id] & 255, L.fit); else this.fits.delete(u[o + Man.id] & 255);
      W.addCapsuleRows(f, u, D.at[Section.caps] + i * BoxWorld.CAPS_PER_CHAR * Cap.row, u[o + Man.id], L.caps, f, o + Man.bound); }
    this.chars.materialsOut(queue);
  }

  private lookOf(o: number, mats: boolean): Look {
    const f = this.D.f, u = this.D.u, id = u[o + Man.id], version = mats ? charMatsVersion() : -1; let L = this.looks.get(id);
    if (!L) { const pal = new Float32Array(CHAR_MAT_FLOATS); L = { version: -2, side: 0, suit: 0, parts: 0, tints: new Float32Array(6), dry: 1, pal, caps: [], stature: 0, fit: null, put: (out, at) => out.set(pal, at) }; this.looks.set(id, L); }
    let same = L.version === version && L.side === u[o + Man.side] && L.suit === u[o + Man.suit] && L.parts === u[o + Man.parts] && L.dry === f[o + Man.dry];
    for (let k = 0; same && k < 6; k++) same = L.tints[k] === f[o + Man.tint + k];
    if (!same) {
      L.version = version; L.side = u[o + Man.side]; L.suit = u[o + Man.suit]; L.parts = u[o + Man.parts]; L.dry = f[o + Man.dry]; for (let k = 0; k < 6; k++) L.tints[k] = f[o + Man.tint + k];
      const t: Vec3 = [L.tints[0], L.tints[1], L.tints[2]], t2: Vec3 = [L.tints[3], L.tints[4], L.tints[5]], k = L.dry;
      if (mats) { const slots = charSlots(this.ch); writeCharPalette(L.pal, 0, SIDES[L.side], t, t2, id, !slots.authored, L.suit !== 0); L.caps = proxyAlbedos(slots, L.pal, 0, k, false, L.parts); }
      else L.caps = [[(t[0] + (0.5 - t[0]) * 0.3) * k, (t[1] + (0.5 - t[1]) * 0.3) * k, (t[2] + (0.5 - t[2]) * 0.3) * k]];
      L.stature = 0;
    }
    if (L.stature !== f[o + Man.stature]) { L.stature = f[o + Man.stature]; L.fit = this.ch.cast ? this.fitOf(L.parts, L.stature) : null; }
    return L;
  }
  /** the cones of one outfit, at a height */
  private fitOf(parts: number, stature: number): Fit | null {
    let F = this.ofParts.get(parts);
    if (F === undefined) { try { F = fitRig(this.ch, parts); } catch { F = null; } this.ofParts.set(parts, F); }
    if (!F || stature === 1) return F;
    return { r0: F.r0.map(r => r * stature), r1: F.r1.map(r => r * stature), pad: F.pad === undefined ? undefined : F.pad * stature };
  }
}
