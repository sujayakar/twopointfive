// VEHICLES SAY THEIR SHAPES. Asked 2026-11-04 from the library's showroom: "could we make it so the cars have rounded surfaces? look really blocky right now";
// and of the shapes as a trial drew them, "Yes, build these in". Every car, pickup, van and motorbike the kits build is a stack of plain boxes: from above
// every line of it is straight. Here is the SHAPE each box of a stack says (scene/shapes.ts: a level file's box may say how it stands when shapes are on),
// worked out once for every kit that builds the vehicle:
//   · the body's upright corners are rounded in plan (more at the nose than at the tail), and a bumper is cut back with the corner it stands in front of;
//   · the bonnet is crowned from side to side and falls to the nose; a car's bonnet and boot fall away at their ends;
//   · the glass leans (the windscreen back, the rear window forward, the sides in) between POSTS in the body's paint, under a roof bowed from side to side
//     and cut back to the glass's top edge; a van's roof is one bow from the windscreen to the back doors;
//   · the wheels are round, with hubs, under ARCHES in the body's paint that stand out of its side; a lamp's corners are taken off and its end follows the
//     rounded corner it sits on;
//   · a motorbike's wheels, tank, bars, lamp and exhaust are round, its seat is bowed and it has mudguards.
// THE RULE'S TWO HALVES (shapes.ts): a shape is CUT FROM the plain box, which keeps its centre and its size — so every shape here only takes away, and a
// vehicle with shapes down is the stack it always was, box for box; and what a shaped vehicle has MORE of (a hub, an arch, a post) is TRIM, drawn and nothing
// else, said by the box it belongs to. Everything is said in the box's own frame (its +x is the vehicle's nose), so nothing here knows where a vehicle stands.
// WHAT A RAY MEETS LESS OF: the body's corners (up to 0.22 m of the nose, 0.1 m of the tail), a bonnet's shoulders, the glass above the waist, which leans in
// by up to 0.42 m at the roof of a car, and a roof's overhang. A lamp keeps its place, half size and paint (game/carlamps.ts finds it by those).
// NO TWO FACES IN ONE PLANE: under a roof the glass and the posts stop 4 mm short of it (a bowed roof is thin at its edge, and what reached up into it showed
// through it there: the maintainer's "some z sorting issues on the blue car roof posts"); a hub stands 4 mm proud of its tyre.
// Nothing here has been seen on a GPU.
import type { Vec3 } from '../math/vec';
import { cutPlane, cutRecord } from './cut';
import type { BoxShape, ShapePiece, ShapePlane } from './shapes';

/** a paint as a kit hands it (its Mat) */
export type CarPaint = { albedo: Vec3; rough?: number; metal?: boolean };
const D = Math.PI / 180, r6 = (x: number) => Math.round(x * 1e6) / 1e6;
const plane = (nx: number, ny: number, nz: number, d: number): ShapePlane => [r6(nx), r6(ny), r6(nz), r6(d)];
const paintOf = (p: CarPaint) => ({ albedo: [p.albedo[0], p.albedo[1], p.albedo[2]] as Vec3, ...(p.rough !== undefined ? { rough: p.rough } : {}), ...(p.metal ? { metal: true } : {}) });
const tone = (p: CarPaint, k: number): CarPaint => ({ ...p, albedo: [r6(p.albedo[0] * k), r6(p.albedo[1] * k), r6(p.albedo[2] * k)] });

// ---- planes of a box of half size h, in its own frame ----
/** the planes that round the upright edge at (sx · h.x, sz · h.z) to radius r, in `n` facets between the two faces */
export const cornerPlanes = (h: Vec3, r: number, sx: number, sz: number, n: number): ShapePlane[] => Array.from({ length: n }, (_, i) => { const a = (i + 1) * 90 / (n + 1) * D; return plane(sx * Math.cos(a), 0, sz * Math.sin(a), Math.cos(a) * (h[0] - r) + Math.sin(a) * (h[2] - r) + r); });
/** the plane that leans the face at sx · h.x (or, with sx = 0, at sz · h.z) inward by `deg` from upright, about its bottom edge */
export const leanPlane = (h: Vec3, sx: number, sz: number, deg: number): ShapePlane => { const t = deg * D; return sx ? plane(sx * Math.cos(t), Math.sin(t), 0, Math.cos(t) * h[0] - Math.sin(t) * h[1]) : plane(0, Math.sin(t), sz * Math.cos(t), Math.cos(t) * h[2] - Math.sin(t) * h[1]); };
/** the plane from the top face `back` metres in from the end at sx · h.x down to `drop` metres under the top at that end: a bonnet falling to its nose */
export const fallPlane = (h: Vec3, sx: number, back: number, drop: number): ShapePlane => { const L = Math.hypot(drop, back); return plane(sx * drop / L, back / L, 0, drop / L * h[0] + back / L * (h[1] - drop)); };
/** a plane of one box said in the frame of another whose centre lies `d` from the first's (the same heading), pushed `out` along its normal */
const moved = (p: ShapePlane, d: Vec3, out = 0): ShapePlane => plane(p[0], p[1], p[2], p[3] - (p[0] * d[0] + p[1] * d[1] + p[2] * d[2]) + out);
/** the planes of `planes` that cut a box of half size h (one that takes none of it would be refused by the lint: it says nothing) */
const biting = (h: Vec3, planes: ShapePlane[]): ShapePlane[] => planes.filter(p => { const r = cutRecord(h, [cutPlane([p[0], p[1], p[2]], p[3])]); return !r.empty && (r.cut?.length ?? 0) > 0; });
const cutOf = (h: Vec3, planes: ShapePlane[]): { cut?: ShapePlane[] } => { const c = biting(h, planes); return c.length ? { cut: c } : {}; };
/** a shape, or nothing where it would say nothing (no plane of it bites its box): the box then stays plain */
const said = (s: BoxShape): BoxShape | undefined => s.round !== undefined || s.arc !== undefined || s.cut !== undefined || s.with !== undefined ? s : undefined;

/** a body's four upright corners: `front` and `rear` = [radius, facets] */
const corners = (h: Vec3, front: [number, number], rear: [number, number]): ShapePlane[] => [...cornerPlanes(h, front[0], 1, 1, front[1]), ...cornerPlanes(h, front[0], 1, -1, front[1]), ...cornerPlanes(h, rear[0], -1, 1, rear[1]), ...cornerPlanes(h, rear[0], -1, -1, rear[1])];

/** A WHEEL (its plain box of half size h, on side s = ±1 of the vehicle): a round on its axle; its trim is a hub 4 mm proud of the tyre's outer face and,
 *  with `body`, the arch over it in the body's paint — a half round about the wheel's own axle that stands `proud` over the tyre's top and ends 0.02 m
 *  inside the tyre's outer face */
export function wheelShape(h: Vec3, s: number, hub: CarPaint, body: CarPaint | null, proud = 0.08): BoxShape {
  const r = Math.min(h[0], h[1]), R = r + proud;
  const trim: ShapePiece[] = [{ off: [0, 0, r6(s * (h[2] + 0.004))], h: [r6(r * 0.55), r6(r * 0.55), 0.004], ...paintOf(hub), round: 3 }];
  if (body) trim.push({ off: [0, r6(R / 2 + r - h[1]), r6(s * (h[2] - 0.135))], h: [r6(R), r6(R / 2), 0.115], ...paintOf(body), arc: { axis: 3, rise: r6(R) } });
  return { round: 3, with: trim };
}
/** A GREENHOUSE: the glass of a cab (its plain box of half size h) leans in by `tilt` degrees at the front, the rear and the sides, from the waist; its
 *  trim is a POST in the body's paint down each corner where two panes meet (a pane with no tilt runs into what stands behind it and has none) and one
 *  across each side at every `posts` (local x), all 12 mm proud of the glass. `low`: how far under the glass box's top the glass and the posts stop (4 mm
 *  under a roof that sits on it; more where the kit's roof slab begins inside the glass's box; 0 where what stands on it is not bowed). */
export function glassShape(h: Vec3, tilt: { f: number; r: number; s: number }, body: CarPaint, posts: readonly number[] = [], low = 0.004): BoxShape {
  const e = 0.012, w = 0.07;
  const F = leanPlane(h, 1, 0, tilt.f), R = leanPlane(h, -1, 0, tilt.r), S = (s: number) => leanPlane(h, 0, s, tilt.s);
  const out = (p: ShapePlane): ShapePlane => plane(p[0], p[1], p[2], p[3] + e), inside = (p: ShapePlane, by: number): ShapePlane => plane(-p[0], -p[1], -p[2], by - p[3]);
  // a post's own box: the glass's, 12 mm wider each way and `low` shorter at the top; its planes are the glass's, said from its centre
  const ph: Vec3 = [r6(h[0] + e), r6(h[1] - low / 2), r6(h[2] + e)], pd: Vec3 = [0, -low / 2, 0];
  const post = (planes: ShapePlane[]): ShapePiece => ({ off: [0, r6(-low / 2), 0], h: ph, ...paintOf(body), cut: planes.map(p => moved(p, pd)), old: true });
  const trim: ShapePiece[] = [];
  for (const s of [-1, 1]) {
    if (tilt.f > 0) trim.push(post([out(F), out(S(s)), inside(F, w), inside(S(s), w)]));
    if (tilt.r > 0) trim.push(post([out(R), out(S(s)), inside(R, w), inside(S(s), w)]));
    for (const x of posts) trim.push(post([out(S(s)), inside(S(s), 0.03), plane(1, 0, 0, x + 0.045), plane(-1, 0, 0, 0.045 - x)]));
  }
  return { ...cutOf(h, [...(tilt.f > 0 ? [F] : []), ...(tilt.r > 0 ? [R] : []), ...(tilt.s > 0 ? [S(1), S(-1)] : []), ...(low > 0 ? [plane(0, 1, 0, h[1] - low)] : [])]), ...(trim.length ? { with: trim } : {}) };
}
/** where a leaning greenhouse's top edge lies, for the roof over it: the glass's half size, its tilts, and how far the roof oversails (more than a post is
 *  proud). Returns the reach from the GLASS's centre: toward the nose, toward the tail, to each side. */
export function glassTop(h: Vec3, tilt: { f: number; r: number; s: number }, over = 0.03): { front: number; rear: number; side: number } {
  const H = 2 * h[1]; return { front: h[0] - H * Math.tan(tilt.f * D) + over, rear: h[0] - H * Math.tan(tilt.r * D) + over, side: h[2] - H * Math.tan(tilt.s * D) + over };
}
/** A ROOF over a greenhouse (its plain box of half size h, its centre `dx` toward the nose from the glass's): bowed from side to side by `rise`, and cut
 *  back at the nose and the tail to the glass's top edge (`top`: glassTop) where it reached beyond */
export function roofShape(h: Vec3, dx: number, rise: number, top: { front: number; rear: number }): BoxShape {
  return { arc: { axis: 1, rise }, ...cutOf(h, [plane(1, 0, 0, top.front - dx), plane(-1, 0, 0, top.rear + dx)]) };
}
/** A LAMP'S LENS (its plain box of half size h, its centre `d` from the body's in the body's frame): its four corners taken off, and where it sits on a
 *  rounded corner of the body (`bodyCut`: the body's own planes) its end taken back with that corner, so it lies `proud` of the panel all along */
export function lensShape(h: Vec3, bodyCut: readonly ShapePlane[], d: Vec3, proud = 0.012): BoxShape | undefined {
  const r = Math.min(h[1], h[2]) * 0.8, own = [-1, 1].flatMap(sy => [-1, 1].map(sz => { const k = Math.SQRT1_2; return plane(0, sy * k, sz * k, k * ((h[1] - r) + (h[2] - r)) + r); }));
  const wrap = bodyCut.filter(p => Math.abs(p[1]) < 1e-6 && p[0] * d[0] > 0 && p[2] * d[2] > 0).map(p => moved(p, d, proud));
  return said({ ...cutOf(h, [...wrap, ...own]) });
}
/** A BUMPER (its plain box of half size h, its centre `d` from the body's): cut back with the body's rounded corners it stands in front of, `proud` of them */
export function bumperShape(h: Vec3, bodyCut: readonly ShapePlane[], d: Vec3, proud = 0.09): BoxShape | undefined {
  const c = cutOf(h, bodyCut.filter(p => Math.abs(p[1]) < 1e-6 && p[0] * d[0] > 0).map(p => moved(p, d, proud))); return c.cut ? { ...c, old: true } : undefined;
}
export interface CarPaints { body: CarPaint; hub: CarPaint }

/** A PICKUP, 5.0 × 1.8, nose toward local +x (scene/gwotKit.ts pickup): the shape each box of its stack says */
export function pickupShapes(P: CarPaints) {
  const tubH: Vec3 = [2.45, 0.25, 0.86], tubCut = corners(tubH, [0.22, 3], [0.1, 1]);
  const glassH: Vec3 = [0.64, 0.21, 0.8], tilt = { f: 30, r: 12, s: 12 }, top = glassTop(glassH, tilt);
  const bonnetH: Vec3 = [0.7, 0.1, 0.84], nose45 = cornerPlanes(tubH, 0.22, 1, 1, 3)[1], nose45b = cornerPlanes(tubH, 0.22, 1, -1, 3)[1];
  return {
    wheel: (s: number) => wheelShape([0.37, 0.37, 0.13], s, P.hub, P.body),
    tub: { cut: tubCut, old: true } as BoxShape,
    bumperFront: bumperShape([0.05, 0.11, 0.88], tubCut, [2.47, 0, 0]), bumperRear: bumperShape([0.05, 0.11, 0.88], tubCut, [-2.47, 0, 0]),
    // (the bonnet: crowned, falling to the nose, its two nose corners taken back with the body's — and no panel lines: a mark is laid out on a whole face)
    bonnet: { arc: { axis: 1, rise: 0.2 }, ...cutOf(bonnetH, [fallPlane(bonnetH, 1, 0.2, 0.15), moved(nose45, [1.75, 0, 0]), moved(nose45b, [1.75, 0, 0])]) } as BoxShape,
    glass: glassShape(glassH, tilt, P.body),
    roof: roofShape([0.6, 0.04, 0.78], 0.36 - 0.38, 0.08, top),
    head: (s: number) => lensShape([0.03, 0.07, 0.16], tubCut, [2.43, 0, s * 0.66]), tail: (s: number) => lensShape([0.03, 0.08, 0.1], tubCut, [-2.43, 0, s * 0.72]),
  };
}

/** A PANEL VAN, 4.9 × 1.8 × 1.95, nose toward local +x, its back doors open (scene/gwotKit.ts van): the shape each box of its stack says. Its roof is one
 *  bow of 6 cm from the windscreen to the back doors — the brow, the load box and the strip over the open bay each say the same arc, and the bay's sides
 *  stop under it */
export function vanShapes(P: CarPaints) {
  const bodyH: Vec3 = [2.4, 0.28, 0.9], bodyCut = corners(bodyH, [0.22, 3], [0.08, 1]);
  const glassH: Vec3 = [0.36, 0.26, 0.84], tilt = { f: 28, r: 0, s: 4 }, bow = 0.06;
  const bonnetH: Vec3 = [0.4, 0.1, 0.86], browH: Vec3 = [0.34, 0.165, 0.86], sideH: Vec3 = [0.25, 0.545, 0.03];
  const nose45 = cornerPlanes(bodyH, 0.22, 1, 1, 3)[1], nose45b = cornerPlanes(bodyH, 0.22, 1, -1, 3)[1];
  return {
    wheel: (s: number) => wheelShape([0.37, 0.37, 0.13], s, P.hub, P.body),
    body: { cut: bodyCut, old: true } as BoxShape,
    bumper: bumperShape([0.05, 0.11, 0.9], bodyCut, [2.43, 0, 0]), step: bumperShape([0.05, 0.07, 0.9], bodyCut, [-2.43, 0, 0]),
    bonnet: { arc: { axis: 1, rise: 0.2 }, ...cutOf(bonnetH, [fallPlane(bonnetH, 1, 0.18, 0.15), moved(nose45, [2.0, 0, 0]), moved(nose45b, [2.0, 0, 0])]) } as BoxShape,
    glass: glassShape(glassH, tilt, P.body, [-0.3], 0),                                        // (the brow stands on it, flat underneath: nothing to stop short of)
    brow: { arc: { axis: 1, rise: bow }, cut: [leanPlane(browH, 1, 0, 28)], old: true } as BoxShape,   // (on the windscreen's line)
    load: { arc: { axis: 1, rise: bow } } as BoxShape,
    bayRoof: { arc: { axis: 1, rise: bow } } as BoxShape,
    baySide: { cut: [plane(0, 1, 0, sideH[1] - bow)] } as BoxShape,                           // (its top comes down to the roof's edge)
    head: (s: number) => lensShape([0.03, 0.07, 0.16], bodyCut, [2.43, 0, s * 0.66]), tail: (s: number) => lensShape([0.02, 0.15, 0.04], bodyCut, [-2.41, 0, s * 0.86]),
  };
}

/** A CAR, 4.4 × 1.72, nose toward local +x (scene/officeGen.ts and scene/levelKit.ts car): the shape each box of its stack says */
export function carShapes(P: CarPaints) {
  const bodyH: Vec3 = [2.2, 0.34, 0.86], bodyCut = [...corners(bodyH, [0.24, 2], [0.16, 1]), fallPlane(bodyH, 1, 0.5, 0.16), fallPlane(bodyH, -1, 0.3, 0.1)];
  const glassH: Vec3 = [1.25, 0.27, 0.8], tilt = { f: 38, r: 42, s: 12 }, top = glassTop(glassH, tilt);
  return {
    wheel: (s: number) => wheelShape([0.32, 0.3, 0.12], s, P.hub, P.body),
    body: { cut: bodyCut, old: true } as BoxShape,
    glass: glassShape(glassH, tilt, P.body, [0.05], 0.014),                                    // (the kit's roof slab begins a centimetre under the glass box's top: the glass stops under the slab)
    roof: roofShape([1.15, 0.03, 0.76], 0, 0.06, top),
    head: (s: number) => lensShape([0.02, 0.07, 0.14], bodyCut, [2.2, 0, s * 0.62]), tail: (s: number) => lensShape([0.02, 0.06, 0.16], bodyCut, [-2.2, 0, s * 0.62]),
  };
}

/** A SMALL MOTORBIKE on its stand, nose toward local +x (scene/gwotKit.ts motorbike): the shape each box of its stack says. Its engine carries the trim: a
 *  mudguard over each wheel, in the tank's paint a shade down. Its fork stays the square tube it is. */
export function bikeShapes(tank: CarPaint) {
  const guard = (lx: number): ShapePiece => ({ off: [r6(lx - 0.02), r6(0.4625 - 0.41), 0], h: [0.325, 0.1625, 0.04], ...paintOf(tone(tank, 0.8)), arc: { axis: 3, rise: 0.325 } });
  return {
    wheel: { round: 3 } as BoxShape,
    engine: { with: [guard(0.66), guard(-0.62)] } as BoxShape,
    tank: { round: 1 } as BoxShape,
    seat: { arc: { axis: 1, rise: 0.07 } } as BoxShape,
    bars: { round: 3 } as BoxShape,
    lamp: { round: 1 } as BoxShape,
    exhaust: { round: 1 } as BoxShape,
  };
}
/** the hub of every kit's wheel: galvanised steel in a wheel's shadow */
export const hubPaint = (galv: Vec3): CarPaint => ({ albedo: [r6(galv[0] * 0.6), r6(galv[1] * 0.6), r6(galv[2] * 0.6)], rough: 0.45, metal: true });
