// A CAR: the lab's own, curved pieces with paint. Coarser: what is thin goes (wipers, seams, rims), then all but the form and its lamps.
import { type V3 } from '../math';
import { F, type Piece } from '../scene/world';
import { M } from '../scene/materials';
import { makeCar } from '../scene/cars';
import { settle } from '../scene/faces';
import { apart, type Made, type ObjectKind } from './object';
const all = () => apart(w => { makeCar(w, [0, 0, 0], 0, { style: 'hatch', col: [0.80, 0.13, 0.10] }); settle(w.pieces); }).pieces;
/** how thick a piece is across its second widest side */
const girth = (p: Piece) => [...p.half].sort((a, b) => b - a)[1]!;
export const car: ObjectKind = { name: 'car', title: 'Car', make: (): Made => ({
  name: 'car', title: 'Car', size: [4.2, 1.49, 2.05], on: 'floor', close: { at: [1.4, 0.65, 0.7], far: 5 }, read: 6, moves: { kind: 'still' }, crowd: 49,
  levels: [
    { shape: 'pieces', error: 0, less: '', put(p) { p.from({ pieces: all() }); } },
    { shape: 'pieces', error: 0.013, less: 'wipers, seams, the rims\' spokes and beads', put(p) { p.from({ pieces: all().filter(q => girth(q) > 0.0135 || q.flags & F.GLOW || q.marks?.length) }); } },
    { shape: 'pieces', error: 0.05, less: 'all but body, glass, wheels, mirrors and lamps', put(p) { p.from({ pieces: all().filter(q => girth(q) > 0.05 || (q.flags & F.GLOW && girth(q) > 0.034) || q.mat === M.film).map(q => ({ ...q, marks: undefined })) }); } },
  ],
}) };
void ({} as V3);
