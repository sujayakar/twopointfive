// AN OBJECT is a small program: numbers in, levels out, no files. Each level is whole by itself (it is drawn alone) and says how far it may be from the finest, in metres: it takes over where that
// is a pixel of the game's picture, and where a ray is that wide. What moves an object moves all its levels: coarser ones hang on fewer parts of the same rig.
import { type V3, type Rot, add, apply, compose, rotY, ID } from '../math';
import { World, F, taper, type Piece, type Instance } from '../scene/world';
import type { Air } from '../scene/air';

/** what a level is made of. Today's renderer draws and meets pieces and leaves; the rest are for when it does */
export type Shape = 'pieces' | 'leaves' | 'triangles' | 'voxels' | 'coverage';
/** what a thing does in time, beyond standing there */
export type Moves =
  | { kind: 'still' }
  /** it can be knocked about: how heavy (kg a cubic metre of what it is made of), how rough, and does it float? */
  | { kind: 'loose'; dense: number; rough?: number; floats: boolean; /** its weight is this far under its middle */ low?: number }
  | { kind: 'sways' }
  | { kind: 'walks' };
/** what a level leaves behind where it was put: what must be stepped once a tick */
export interface Live { step?(dt: number, t: number, air: Air): void; /** feet, for what walks: where each is and is it down? */ feet?(): { at: V3; down: boolean }[] }
export interface Level {
  shape: Shape;
  /** metres: no part of its outline is farther than this from where the finest has it. 0 for the finest */
  error: number;
  /** what it gives up, in a few words */ less: string;
  put(p: Put): Live | void;
}
export interface Made {
  name: string; title: string;
  /** how wide (x), high (y) and deep (z) it is, metres; it stands at the origin, its front to +z */ size: V3;
  levels: Level[]; moves: Moves;
  /** what it stands on is under it ('floor'), or it is a skin of the floor itself ('skin') */ on: 'floor' | 'skin';
  /** a place to look at close by, and from how far; and how far off people stand who read what is printed on it (0: nothing is) */ close: { at: V3; far: number }; read: number;
  /** how many stand in the crowd (100 if not said) */ crowd?: number;
}
export interface ObjectKind { name: string; title: string; make(): Made }

/** where a level is put, and tools that work in the object's own frame (x right, y up, z to its front). Everything put by them is of one thing */
export class Put {
  readonly r: Rot; readonly thing: { name: string };
  constructor(readonly w: World, readonly name: string, readonly at: V3, readonly yaw: number, /** what makes this one differ from the next */ readonly seed = 1, private flags = F.CUT) { this.r = rotY(yaw); this.thing = { name }; }
  P(x: number, y: number, z: number): V3 { return add(this.at, apply(this.r, [x, y, z])); }
  /** a piece, its place and turn in the object's frame */
  add(p: Partial<Piece> & { at: V3; half: V3; mat: number }): Piece { return this.w.add({ thing: this.thing, flags: this.flags, ...p, ...(p.flags === undefined ? {} : { flags: p.flags | this.flags }), at: this.P(...p.at), rot: p.rot ? compose(this.r, p.rot) : this.r }); }
  box(at: V3, half: V3, mat: number, tint: V3, more: Partial<Piece> = {}) { return this.add({ at, half, mat, tint, ...more }); }
  /** round, upright */
  drum(at: V3, radius: number, half: number, mat: number, tint: V3, more: Partial<Piece> = {}) { return this.add({ at, half: [radius, half, radius], mat, tint, quad: [1 / radius ** 2, 0, 1 / radius ** 2, 0], ...more }); }
  ball(at: V3, half: V3, mat: number, tint: V3, more: Partial<Piece> = {}) { return this.add({ at, half, mat, tint, quad: [1 / half[0] ** 2, 1 / half[1] ** 2, 1 / half[2] ** 2, 0], ...more }); }
  /** a round bar from a to b, thick ra and rb at its ends */
  bar(a: V3, b: V3, ra: number, rb: number, mat: number, tint: V3, more: Partial<Piece> = {}) { const t = taper(a, b, ra, rb); return this.add({ ...t, mat, tint, ...more }); }
  /** what another maker made in a world of its own, put here: pieces and leaves in this object's frame */
  from(o: { pieces: Piece[]; instances?: Instance[] }, clusters?: (i: Instance) => number) {
    for (const p of o.pieces) { const { at, rot, thing: _, ...rest } = p; void _; this.add({ ...rest, at, rot }); }
    for (const i of o.instances ?? []) this.w.instances.push({ ...i, cluster: clusters ? clusters(i) : i.cluster, at: this.P(...i.at), base: this.P(...i.base), rot: compose(this.r, i.rot) });
  }
}
/** a world to make something in, apart */
export const apart = (f: (w: World) => void): World => { const w = new World(); f(w); return w; };
/** the angle of a pixel of the game's picture: 1080 of them over 0.40 rad */
export const PIXEL = 2 * Math.tan(0.2) / 1080;
/** how far off a level takes over */
export const takesOver = (l: Level) => l.error / PIXEL;
export { ID };
