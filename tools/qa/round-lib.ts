// ONE PROBE FOR A ROUND THING WHERE IT STANDS (round-check.ts lines 4, 4c and 4d hand it a collision world and one thing in it).
// Three questions, each asked all the way round, each of the game's own collision world (scene/collision.ts StaticCollision):
//   · SIGHT: a line that grazes the thing just inside its radius is stopped, and one a hair outside is free (segmentBlocked);
//   · A ROUND FIRED AT IT, from beside it and from across a room, stops on the curve at the distance the circle gives, its normal straight out (raycast);
//   · A MAN WALKED INTO IT, and one put half way to its middle, stands straight out from it at its radius plus his own (collideCircle) — or, where it is one
//     of a PILE (a stack of tyres, each a few centimetres off the one under it: round things whose middle is over this one), a man walked into it stands
//     outside every one of them and against one, near where this one alone would hold him.
// A thing in a level has NEIGHBOURS (the wall it leans on, the drum beside it, the stand it is on), so all but the inside lines are asked only from the sides
// that are OPEN: where nothing else comes near the line, the round's path or the man's way out. Open is decided HERE, before the question, by a reference
// that asks the collision world nothing: the distance from a segment to every other box's footprint (a round neighbour's is its circle). A line through the
// cylinder is stopped whatever stands beside it, so the inside lines are asked from every side.
// HOW HARD EACH HOLDS IT: a round and a man hold the curve to a thousandth of a millimetre; the lines alone hold the radius between 1 % under and 1.5 % over,
// and the inside lines alone are what the box round the thing would answer as well. A thing hemmed in on every side is asked the inside lines only: nothing
// can meet its curve where it stands, and the caller says so (roundAsked) rather than calling it round.
// Nothing here is a picture, and nothing here is the level's own: the caller says where the thing is, how wide, and which boxes are it.
import { ROOT, type Vec3 } from './headless.ts';
const { BoxFlag, BoxWorld } = await import(`${ROOT}/src/scene/boxes.ts`);

/** the thing: its middle, its radius and half height as the REFERENCE has them (the level's own numbers and round.ts's circle with the octagon's area, written
 *  out by the caller), and which of the world's boxes are it (a round box and its three specks, or the kit's four slabs) */
export interface RoundThing { c: Vec3; R: number; hy: number; own(i: number): boolean }
export interface RoundAnswers {
  /** sides asked */ sides: number;
  /** lines just inside the radius that were stopped (wanted: every side) */ inStopped: number;
  /** sides where nothing else is near the line a hair outside, and how many of those lines were stopped (wanted: none) */ outOpen: number; outStopped: number;
  /** rounds with a clear way in, and how many stopped on the curve with the normal straight out; of the ones that met the thing at all, the furthest from the
   *  curve in metres and the most the normal was off (the length of the difference of two unit vectors) */ shotOpen: number; shotGood: number; shotOff: number; shotTurned: number;
  /** does a walking man meet it at all ('under': it ends below his feet, 'over': it begins above his head) */ walker: 'met' | 'under' | 'over';
  /** men with a clear way out; over those, measured from the nearest round thing of the pile (this one and every round thing whose middle is over it):
   *  `sunk` the most a man was left inside one, `apart` the most one was left clear of all of them (wanted: 0 and 0, in metres), and `strayed` how many were
   *  not left where this thing alone would hold them, or in a pile further from there than the pile can carry a man (wanted: none) */ bodyOpen: number; sunk: number; apart: number; strayed: number;
}
/** `feet`‥`head`: the span a walking man asks the world about (collision.ts FEET_Y and a shoulder: what every walker's move passes). What a thing must reach
 *  to stop him is the world's own band over that (collision.ts footOf: the underfoot rule). */
export interface RoundAsk { feet?: number; head?: number }
/** every answer as a round thing's should be: stopped inside from every side, and on the open sides free outside, hit on the curve, and a man against the curve */
export const roundAnswered = (A: RoundAnswers) => A.inStopped === A.sides && A.outStopped === 0 && A.shotGood === A.shotOpen && A.sunk < 1e-6 && A.apart < 1e-6 && A.strayed === 0;
/** was anything asked of it but the inside lines? (no: it is hemmed in on every side) */
export const roundAsked = (A: RoundAnswers) => A.outOpen + A.shotOpen + A.bodyOpen > 0;

/** the least distance in XZ from the segment a‥b to the box's footprint, 0 where they cross: a round box's circle, a box turned about y, and for one fully
 *  rotated the rectangle that holds BOTH things the world makes of it: the box as it is turned, which a line and a round meet, and the same box only
 *  yawed, which a walking body meets (collision.ts collideCircle reads no `rot`). So it only ever calls a side crowded that was not — and no further
 *  out than that, or a ladder leaned on a wall 3 m off crowds a brazier */
const both = (a: number[], b: number[]): [number, number] => [Math.max(a[0], b[0]), Math.max(a[1], b[1])];
function footprintGap(b: any, ax: number, az: number, bx: number, bz: number): number {
  const [hx, hz] = b.rot ? both(BoxWorld.footprint(b), BoxWorld.footprint({ ...b, rot: undefined })) : [b.h[0], b.h[2]], yaw = b.rot ? 0 : b.yaw, c = Math.cos(yaw), s = Math.sin(yaw);
  const loc = (x: number, z: number): [number, number] => { const dx = x - b.c[0], dz = z - b.c[2]; return [c * dx - s * dz, s * dx + c * dz]; };
  const [px, pz] = loc(ax, az), [qx, qz] = loc(bx, bz), dx = qx - px, dz = qz - pz;
  const len2 = dx * dx + dz * dz, toSeg = (x: number, z: number) => { const t = len2 < 1e-18 ? 0 : Math.max(0, Math.min(1, ((x - px) * dx + (z - pz) * dz) / len2)); return Math.hypot(px + dx * t - x, pz + dz * t - z); };
  if (b.round === true && !b.rot) return Math.max(0, toSeg(0, 0) - b.h[0]);
  let t0 = 0, t1 = 1, cross = true;   // does the segment cross the rectangle? (clipped against its four sides)
  for (const [p, d, h] of [[px, dx, hx], [pz, dz, hz]] as const) {
    if (Math.abs(d) < 1e-12) { if (Math.abs(p) > h) cross = false; continue; }
    const u = (-h - p) / d, v = (h - p) / d; t0 = Math.max(t0, Math.min(u, v)); t1 = Math.min(t1, Math.max(u, v));
  }
  if (cross && t0 <= t1) return 0;
  const toRect = (x: number, z: number) => Math.hypot(Math.max(Math.abs(x) - hx, 0), Math.max(Math.abs(z) - hz, 0));
  return Math.min(toRect(px, pz), toRect(qx, qz), toSeg(hx, hz), toSeg(hx, -hz), toSeg(-hx, hz), toSeg(-hx, -hz));
}
const span = (b: any): [number, number] => { const h = b.rot ? Math.max(BoxWorld.heightExtent(b) as number, b.h[1]) : b.h[1]; return [b.c[1] - h, b.c[1] + h]; };
/** sides asked: a count the kit's eight-sided stand-in shares no factor with, so every face of an octagon is met at another place whichever way it is turned */
const SIDES = 67;
/** a man's radius (characters.ts: every walker's) */
const BODY = 0.3;
/** how far out a round is fired from: beside the thing, and from across a room (the ray's walk over several cells of the world's grid) */
const SHOT_FROM = [0.3, 3];
/** the play a pile's own circles give a man beyond its reach (measured over every pile of every level file at the time: under a millimetre) */
const PILE_PLAY = 0.05;

/** Ask the three questions of `col` about `thing`, from SIDES directions evenly round it. */
export function probeRound(col: any, thing: RoundThing, ask: RoundAsk = {}): RoundAnswers {
  const N = SIDES, r = BODY, feet = ask.feet ?? 0.2, head = ask.head ?? 1.5, stops = col.footOf(feet) as number, { c, R, hy } = thing, y = c[1];
  // everything else that stands within reach of any question asked here: the world's boxes but never a hidden one, and every door leaf laid over them
  const reach = R + r + SHOT_FROM[1] + 0.2, others: any[] = [];
  const take = (b: any) => { if (footprintGap(b, c[0], c[2], c[0], c[2]) < reach) others.push(b); };
  col.boxes.forEach((b: any, i: number) => { if (!thing.own(i) && !(b.flags & BoxFlag.NoShadow)) take(b); }); for (const b of col.dynamicBoxes ?? []) take(b);
  /** nothing else within `gap` of the level segment a‥b at the thing's mid height (a box counts from 1 cm past its top and its foot, or half the thing's own
   *  half height where that is less: what a thin thing lies on is under the line, not beside it) */
  const edge = Math.min(0.01, hy / 2);
  const lineOpen = (a: Vec3, b: Vec3, gap: number) => others.every(o => { const [y0, y1] = span(o); return y < y0 - edge || y > y1 + edge || footprintGap(o, a[0], a[2], b[0], b[2]) > gap; });
  // what a walking man reaches of the rest: the PILE (round things whose middle is over this one: he is pushed off the lot) and everything else, which crowds a side
  const reached = others.filter(o => { const [y0, y1] = span(o); return y1 >= stops && y0 <= head; });
  const mate = (o: any) => o.round === true && Math.hypot(o.c[0] - c[0], o.c[2] - c[2]) < R, pile = reached.filter(mate), rest = reached.filter(o => !mate(o));
  /** how far a pile may carry a man from where this thing alone would hold him: as far as it reaches past this thing's own curve, and PILE_PLAY (he is pushed
   *  off each of its circles in turn, along that circle's own line). No pile: nowhere, he is pushed straight out. */
  const carry = pile.length ? Math.max(0, ...pile.map(o => Math.hypot(o.c[0] - c[0], o.c[2] - c[2]) + o.h[0] - R)) + PILE_PLAY : 0;
  /** a man pushed straight out from a to b, and as far from b as the pile may carry him, comes near nothing but the pile */
  const wayOpen = (a: Vec3, b: Vec3) => rest.every(o => footprintGap(o, a[0], a[2], b[0], b[2]) > r + carry + 0.01);
  const walker = c[1] + hy < stops ? 'under' : c[1] - hy > head ? 'over' : 'met';
  const out: RoundAnswers = { sides: N, inStopped: 0, outOpen: 0, outStopped: 0, shotOpen: 0, shotGood: 0, shotOff: 0, shotTurned: 0, walker, bodyOpen: 0, sunk: 0, apart: 0, strayed: 0 };
  const half = 0.45 * R + 0.05;   // (past the kit's octagon's corners, which a line a hair outside the circle still crosses)
  for (let k = 0; k < N; k++) {
    const a = (k + 0.37) / N * Math.PI * 2, nx = Math.cos(a), nz = Math.sin(a);
    const tangent = (d: number): [Vec3, Vec3] => { const m: Vec3 = [c[0] + nx * d, y, c[2] + nz * d]; return [[m[0] - nz * half, y, m[2] + nx * half], [m[0] + nz * half, y, m[2] - nx * half]]; };
    // sight
    { const [p, q] = tangent(R * 0.99); if (col.segmentBlocked(p, q)) out.inStopped++; }
    { const [p, q] = tangent(R * 1.015); if (lineOpen(p, q, 0.03)) { out.outOpen++; if (col.segmentBlocked(p, q)) out.outStopped++; } }
    // a round, fired straight in along this side, 0.6 of the radius off the middle
    for (const from of SHOT_FROM) { const D = R + from, off = R * 0.6, o: Vec3 = [c[0] + nx * D - nz * off, y, c[2] + nz * D + nx * off], into = Math.sqrt(R * R - off * off), want = D - into, end: Vec3 = [o[0] - nx * want, y, o[2] - nz * want];
      if (!lineOpen(o, end, 0.03)) continue; out.shotOpen++;
      const hit = col.raycast(o, [-nx, 0, -nz], D + R); if (!hit || !thing.own(hit.index)) continue;
      const far = Math.abs(hit.t - want), turned = Math.hypot(hit.n[0] - (nx * into - nz * off) / R, hit.n[1], hit.n[2] - (nz * into + nx * off) / R);
      out.shotOff = Math.max(out.shotOff, far); out.shotTurned = Math.max(out.shotTurned, turned); if (far < 1e-6 && turned < 1e-6) out.shotGood++; }
    // a man: walked in to 5 cm past where the thing alone would hold him, and (not in a pile, whose other circles would take him their own ways) put half way to its middle
    if (walker === 'met') { const stand: Vec3 = [c[0] + nx * (R + r), 0, c[2] + nz * (R + r)];
      for (const d of pile.length ? [R + r - 0.05] : [R + r - 0.05, R * 0.5]) { const p: Vec3 = [c[0] + nx * d, 0, c[2] + nz * d]; if (!wayOpen(p, stand)) continue; out.bodyOpen++;
        col.collideCircle(p, r, feet, head);
        let least = Math.hypot(p[0] - c[0], p[2] - c[2]) - (R + r); for (const o of pile) least = Math.min(least, Math.hypot(p[0] - o.c[0], p[2] - o.c[2]) - (o.h[0] + r));
        out.sunk = Math.max(out.sunk, -least); out.apart = Math.max(out.apart, least); if (Math.hypot(p[0] - stand[0], p[2] - stand[2]) > carry + 1e-6) out.strayed++; } }
  }
  return out;
}
