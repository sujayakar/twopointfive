// A PARKED VEHICLE'S LAMPS, FOUND, as a level is written (its file's `vehicleLamps`): no generator names one, so by what they write (officeGen.ts and levelKit.ts car, gwotKit.ts pickup and van, the pass's lorry): its
// half size, its paint or its glow, and its twin the same height and yaw across the vehicle's width.
import type { Vec3 } from '../../../src/math/vec';
import type { Box } from '../../../src/scene/boxes';
import { GWOT_PALETTE } from '../../../src/scene/gwotPalette';

/** how far a lamp's half size or its twin's place may sit from what the generators write, in metres, and its paint or glow per channel */
export interface LampFit { fit: number; tone: number; twin: number }
export const LAMP_FIT: LampFit = { fit: 0.004, tone: 0.004, twin: 0.03 };

export interface LampSig { what: 'head lamp' | 'tail lamp'; h: Vec3; albedo?: Vec3; emissive?: Vec3; /** centre to centre across the vehicle */ gap: number; }
/** what each generator writes for a lamp (built on demand: nothing here is read at module load) */
export function sigs(): LampSig[] {
  const white = GWOT_PALETTE.plasticWhite, red = GWOT_PALETTE.truckRed;
  return [
    { what: 'tail lamp', h: [0.02, 0.06, 0.16], emissive: [0.5, 0.03, 0.02], gap: 1.24 },   // the car (officeGen.ts, levelKit.ts)
    { what: 'head lamp', h: [0.02, 0.07, 0.14], albedo: [0.8, 0.8, 0.75], gap: 1.24 },
    { what: 'head lamp', h: [0.03, 0.07, 0.16], albedo: white, gap: 1.32 },                  // the pickup and the van (gwotKit.ts)
    { what: 'tail lamp', h: [0.03, 0.08, 0.1], albedo: red, gap: 1.44 },                     // the pickup
    { what: 'tail lamp', h: [0.02, 0.15, 0.04], albedo: red, gap: 1.72 },                    // the van
    { what: 'head lamp', h: [0.02, 0.08, 0.1], albedo: white, gap: 1.4 },                    // the pass's lorry
  ];
}
export const near3 = (a: ArrayLike<number>, b: ArrayLike<number>, eps: number) => Math.abs(a[0] - b[0]) <= eps && Math.abs(a[1] - b[1]) <= eps && Math.abs(a[2] - b[2]) <= eps;
const glows = (b: Box) => b.emissive[0] > 0 || b.emissive[1] > 0 || b.emissive[2] > 0;
function fits(b: Box, s: LampSig, K: LampFit): boolean {
  if (b.rot || !near3(b.h, s.h, K.fit)) return false;
  if (s.emissive) return near3(b.emissive, s.emissive, K.tone);
  return !glows(b) && near3(b.albedo, s.albedo!, K.tone);
}

export interface FoundLamp { box: number; what: LampSig['what']; lit: boolean; }
/** the lamp boxes of a level's vehicles, in box order: each fits one generator's lamp and has its twin across the vehicle */
export function carLampsIn(boxes: readonly Box[], K: LampFit = LAMP_FIT): FoundLamp[] {
  const S = sigs(), cand: { i: number; s: number }[] = [];
  for (let i = 0; i < boxes.length; i++) { const s = S.findIndex(x => fits(boxes[i], x, K)); if (s >= 0) cand.push({ i, s }); }
  const out: FoundLamp[] = [];
  for (const a of cand) {
    const A = boxes[a.i], s = S[a.s];
    const twin = cand.some(b => b.i !== a.i && b.s === a.s && Math.abs(boxes[b.i].c[1] - A.c[1]) <= K.twin && Math.abs(boxes[b.i].yaw - A.yaw) <= 1e-3
      && Math.abs(Math.hypot(boxes[b.i].c[0] - A.c[0], boxes[b.i].c[2] - A.c[2]) - s.gap) <= K.twin);
    if (twin) out.push({ box: a.i, what: s.what, lit: !!s.emissive });
  }
  return out;
}
/** … as the level says them: none said where it has none */
export function lampsOf(boxes: readonly Box[]): { vehicleLamps?: { box: number; what: LampSig['what'] }[] } {
  const found = carLampsIn(boxes);
  return found.length ? { vehicleLamps: found.map(({ box, what }) => ({ box, what })) } : {};
}
