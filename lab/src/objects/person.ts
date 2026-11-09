// A PERSON: one rig (scene/people.ts), stepped once a tick. A level is a list of parts that each follow one of the rig's: the finest all of them, coarser ones fewer and fatter.
import { type V3, type Rot, add, apply, compose } from '../math';
import { F, World, type Piece } from '../scene/world';
import { C } from '../scene/materials';
import { Person, type Stop } from '../scene/people';
import type { Live, Made, ObjectKind, Put } from './object';
/** a part of a level: whose place it takes, and how it differs from it */
interface Part { of: string; half?: V3; at?: V3; quad?: Piece['quad'] | null; tint?: V3 }
const ell = (h: V3): Piece['quad'] => [1 / h[0] ** 2, 1 / h[1] ** 2, 1 / h[2] ** 2, 0];
function put(p: Put, parts: Part[] | null): Live {
  const apartW = new World(), here: V3 = [0, 0, 0], stops: Stop[] = walk.on ? [{ at: here }, { at: [0, 0, 6] }, { at: [0.2, 0, 0], wait: 1 }] : [{ at: here, wait: 1e9, face: 0.35 }], who = new Person(apartW, stops, 1.25, C.orange as V3, p.seed), ix = (who as unknown as { ix: Map<string, Piece> }).ix;
  const list: { from: Piece; to: Piece; at: V3 | null; bone: boolean }[] = (parts ?? [...ix.keys()].map(of => ({ of }))).map((q: Part) => { const from = ix.get(q.of); if (!from) throw new Error('no part ' + q.of); const half = q.half ?? from.half, to = p.w.add({ thing: p.thing, at: [0, -100, 0], half: [...half] as V3, mat: from.mat, tint: q.tint ?? from.tint, flags: F.MOVES | F.CUT, rig: 'person', ...(q.quad === null ? {} : q.quad ?? (q.half ? ell(half) : from.quad) ? { quad: q.quad ?? (q.half ? ell(half) : from.quad) } : {}), ...(q.half ? {} : from.planes ? { planes: from.planes } : {}) }); return { from, to, at: q.at ?? null, bone: !q.half }; });
  const pose = () => { for (const { from, to, at, bone } of list) { const rot: Rot = compose(p.r, from.rot), c = at ? add(from.at, apply(from.rot, at)) : from.at; to.at = p.P(...c); to.rot = rot; if (bone) to.half = from.half; } };
  for (let i = 0; i < 60; i++) who.step(1 / 60); pose();
  return { step(dt) { who.step(dt); pose(); }, feet: () => who.feet.map(f => ({ at: p.P(...f.at), down: f.down })) };
}
/** for the bench: do they stand, or walk up and down? */
export const walk = { on: false };
const both = (f: (s: string) => Part[]): Part[] => ['L', 'R'].flatMap(f);
const MID: Part[] = [...['pelvis', 'belly', 'chest', 'yoke', 'neck', 'plateF', 'plateB', 'pack', 'belt', 'head', 'helmet', 'visor', 'jaw', 'recv', 'rail', 'barrel', 'can', 'mag', 'stock', 'mag1'].map(of => ({ of })), { of: 'collar' }, ...both(s => ['shoulder', 'arm', 'fore', 'hand', 'thigh', 'kneeB', 'shin', 'boot', 'cuff', 'biceps'].map(of => ({ of: of + s }))), { of: 'patch' }, { of: 'holster' }, ...both(s => [{ of: 'pouch' + s }])];
const FAR: Part[] = [{ of: 'pelvis', half: [0.172, 0.155, 0.135], at: [0, -0.02, -0.01] }, { of: 'chest', half: [0.205, 0.31, 0.155], at: [0, -0.06, 0] }, { of: 'plateF' }, { of: 'pack' }, { of: 'head', half: [0.085, 0.13, 0.10], at: [0, -0.03, 0.01] }, { of: 'helmet' }, { of: 'recv', half: [0.03, 0.05, 0.36], at: [0, -0.004, 0.10], quad: null }, { of: 'mag' }, ...both(s => [{ of: 'shoulder' + s }, { of: 'arm' + s }, { of: 'fore' + s }, { of: 'thigh' + s }, { of: 'shin' + s }, { of: 'boot' + s }])];
export const person: ObjectKind = { name: 'person', title: 'Person', make: (): Made => ({
  name: 'person', title: 'Person', size: [0.68, 1.90, 1.16], on: 'floor', close: { at: [0, 1.4, 0.1], far: 3.4 }, read: 0, moves: { kind: 'walks' }, crowd: 49,
  levels: [
    { shape: 'pieces', error: 0, less: '', put: p => put(p, null) },
    { shape: 'pieces', error: 0.015, less: 'straps, pads, muscles, the gun\'s small parts', put: p => put(p, MID) },
    { shape: 'pieces', error: 0.045, less: 'all but trunk, head, limbs, plate, pack and gun', put: p => put(p, FAR) },
  ],
}) };
