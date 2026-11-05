// VEHICLES SAY THEIR SHAPES. Asked 2026-11-04 from the library's showroom: "could we make it so the cars have rounded surfaces? look really blocky right now";
// and of the shapes as a trial drew them, "Yes, build these in". Every car, pickup, van, motorbike and lorry the kits build is a stack of plain boxes: from
// above every line of it is straight. Here is the SHAPE each box of a stack says (scene/shapes.ts: a level file's box may say how it stands when shapes are
// on), worked out once for every kit that builds the vehicle:
//   · the body's upright corners are rounded in plan (more at the nose than at the tail; a van's tail is square), and what stands on or in front of a corner (a bumper, a lamp, a
//     bed's side, a tailgate) is cut back with it;
//   · the bonnet is crowned from side to side and falls to the nose; a car's bonnet and boot fall away at their ends;
//   · the glass leans (the windscreen back, the rear window forward, the sides in) between POSTS in the body's paint, under a roof bowed from side to side
//     and cut back to the glass's top edge; a van's roof is one bow from the windscreen to the back doors;
//   · the wheels are round, with hubs, under ARCHES in the body's paint that stand out of its side; a lamp's corners are taken off;
//   · a motorbike's wheels, bars, lamp and exhaust are round, its tank's edges are taken off, its seat is bowed and it has mudguards;
//   · a lorry's tilt is bowed over its hoops, its cab roof is an arch, its engine cover narrows to a rounded top, and it carries its fittings.
// THE RULE'S TWO HALVES (shapes.ts): a shape is CUT FROM the plain box, which keeps its centre and its size — so every shape here only takes away, and a
// vehicle with shapes down is the stack it always was, box for box; and what a shaped vehicle has MORE of (a hub, an arch, a post, a mirror) is TRIM, drawn
// and nothing else, said by a box it belongs to. Everything is said in a box's own frame (its +x is the vehicle's nose), so nothing here knows where a
// vehicle stands.
// TRIM IN THE BODY'S PAINT SAYS NO PAINT: a post or an arch is said by a box that wears the body's paint and wears that box's, so a level that repaints its
// vehicles (tools/levels/gwotSkin.ts) repaints their trim. Trim of another stuff (a hub, a mirror, a tank) says its own.
// A PIECE SAYS ITS TIGHTEST BOX (fitted): with cut pieces off a piece is drawn as its uncut box, and a post said as the whole greenhouse would swallow it.
// EVERY FORM HERE IS `old`: a pressed panel, a pane, a moulding — never a fresh break.
// WHAT A RAY MEETS LESS OF: the body's corners (up to 0.24 m of the nose, 0.16 m of the tail: the car's; a pickup's 0.22 and 0.1), a bonnet's shoulders, the glass above the waist, which leans in
// by up to 0.42 m at the roof of a car, and a roof's overhang. A LORRY'S TILT KEEPS ITS EAVES ABOVE A MAN'S HEAD (its bow is 0.15 m, to 1.8 m: the pass's
// yard is laid out on what the lorry hides). A lamp keeps its place, half size and paint (game/carlamps.ts finds it by those).
// NO TWO FACES IN ONE PLANE: under a roof the glass and the posts stop 4 mm short of it (a bowed roof is thin at its edge, and what reached up into it showed
// through it there: the maintainer's "some z sorting issues on the blue car roof posts"); a hub stands 4 mm proud of its tyre.
// Nothing here has been seen on a GPU.
import type { Vec3 } from '../math/vec';
import { cutFit, cutPlane, cutRecord } from './cut';
import type { BoxShape, ShapePiece, ShapePlane } from './shapes';
import { saidAs, madeOf } from './stuff';

/** A VEHICLE'S BODY SAYS IT IS A CAR'S COAT, whatever paint it is given: the one place the word is said (the kits' vehicles, the plain car, and a vehicle
 *  a level builds by hand). A paint's ROW cannot say it: the truck enamels are cots, lockers and floor lines too, a tanker's cream is a wall's. `coated`
 *  is the same for a kit's Mat. */
export const carCoat = (paint: readonly number[]): Vec3 => saidAs('carpaint', paint);
export const coated = <T extends { albedo: readonly number[] }>(m: T): T => ({ ...m, albedo: carCoat(m.albedo) });
/** WHAT A CAR SAYS IT IS MADE OF, whatever colour it is handed: its coat car paint, its windows glass — SOLID glass, nothing is modelled behind them; a round
 *  cracks it and it stands (game/particleFx.ts HITSTUFF) — and its tyres rubber. The plain car's three paints (officeGen.ts car, levelKit.ts car). */
export const carSays = (body: readonly number[], black: readonly number[]) => ({ paint: carCoat(body), glass: madeOf('glass', [0.07, 0.08, 0.1]), tyre: saidAs('rubber', black) });
/** a paint as a kit hands it (its Mat) */
export type CarPaint = { albedo: Vec3; rough?: number; metal?: boolean };
const D = Math.PI / 180, r6 = (x: number) => Math.round(x * 1e6) / 1e6;
const plane = (nx: number, ny: number, nz: number, d: number): ShapePlane => [r6(nx), r6(ny), r6(nz), r6(d)];
const paintOf = (p: CarPaint) => ({ albedo: [p.albedo[0], p.albedo[1], p.albedo[2]] as Vec3, ...(p.rough !== undefined ? { rough: p.rough } : {}), ...(p.metal ? { metal: true } : {}) });
const tone = (p: CarPaint, k: number): CarPaint => ({ ...p, albedo: [r6(p.albedo[0] * k), r6(p.albedo[1] * k), r6(p.albedo[2] * k)] });
const add = (a: Vec3, b: Vec3): Vec3 => [r6(a[0] + b[0]), r6(a[1] + b[1]), r6(a[2] + b[2])], sub = (a: Vec3, b: Vec3): Vec3 => [r6(a[0] - b[0]), r6(a[1] - b[1]), r6(a[2] - b[2])];

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
/** a box's cut by those planes, as a pressed panel — or nothing where none of them bites (the box then stays plain) */
const cutOf = (h: Vec3, planes: ShapePlane[]): { cut?: ShapePlane[]; old?: true } => { const c = biting(h, planes); return c.length ? { cut: c, old: true } : {}; };
/** a shape, or nothing where it would say nothing */
const said = (s: BoxShape): BoxShape | undefined => s.round !== undefined || s.arc !== undefined || s.cut !== undefined || s.with !== undefined ? s : undefined;
/** a piece of trim cut by `planes` (said from `off`, in a box of half size h there), in its own TIGHTEST box (cut.ts cutFit) */
function fitted(off: Vec3, h: Vec3, planes: ShapePlane[], paint: Partial<ShapePiece> = {}): ShapePiece {
  const r = cutRecord(h, planes.map(p => cutPlane([p[0], p[1], p[2]], p[3]))), f = cutFit(h, r.cut), fh: Vec3 = [r6(f.h[0]), r6(f.h[1]), r6(f.h[2])];
  const c = biting(fh, (f.cut ?? []).map(q => plane(q.n[0], q.n[1], q.n[2], q.d)));
  return { off: add(off, f.off), h: fh, ...paint, ...(c.length ? { cut: c, old: true as const } : {}) };
}
/** trim said of one box, said again of another whose centre lies `d` from the first's */
const rehome = (pieces: ShapePiece[], d: Vec3): ShapePiece[] => pieces.map(w => ({ ...w, off: sub(w.off, d) }));

/** a body's four upright corners: `front` and `rear` = [radius, facets] */
const corners = (h: Vec3, front: [number, number], rear: [number, number]): ShapePlane[] => [...cornerPlanes(h, front[0], 1, 1, front[1]), ...cornerPlanes(h, front[0], 1, -1, front[1]), ...cornerPlanes(h, rear[0], -1, 1, rear[1]), ...cornerPlanes(h, rear[0], -1, -1, rear[1])];
/** the planes of a body's rounded corners that what stands on or in front of them (its centre `d` from the body's) is cut back with, `proud` of them */
const withCorners = (bodyCut: readonly ShapePlane[], d: Vec3, proud = 0): ShapePlane[] => bodyCut.filter(p => Math.abs(p[1]) < 1e-6 && (d[0] === 0 || p[0] * d[0] > 0) && (d[2] === 0 || p[2] * d[2] > 0)).map(p => moved(p, d, proud));

/** A WHEEL (its plain box of half size h, on side s = ±1 of the vehicle): a round on its axle, and a hub 4 mm proud of the tyre's outer face */
export function wheelShape(h: Vec3, s: number, hub: CarPaint): BoxShape {
  const r = Math.min(h[0], h[1]);
  return { round: 3, with: [{ off: [0, 0, r6(s * (h[2] + 0.004))], h: [r6(r * 0.55), r6(r * 0.55), 0.004], ...paintOf(hub), round: 3 }] };
}
/** THE ARCH over a wheel (the wheel's centre `c` in the vehicle's frame, its half size h, its side s): a half round about the wheel's own axle that stands
 *  `proud` over the tyre's top and ends 0.02 m inside the tyre's outer face — trim of the body's box (its centre `body`), in the body's paint */
export function archPiece(c: Vec3, h: Vec3, s: number, body: Vec3, proud = 0.08): ShapePiece {
  const r = Math.min(h[0], h[1]), R = r6(r + proud);
  return { off: sub([c[0], c[1] - h[1] + r + R / 2, c[2] + s * (h[2] - 0.135)], body), h: [R, r6(R / 2), 0.115], arc: { axis: 3, rise: R }, old: true };
}
/** A GREENHOUSE: the glass of a cab (its plain box of half size h) leans in by `tilt` degrees at the front, the rear and the sides, from the waist; a POST
 *  stands down each corner where two panes meet (a pane with no tilt runs into what stands behind it and has none) and one across each side at every
 *  `posts` (local x), all 12 mm proud of the glass. `low`: how far under the glass box's top the glass and the posts stop (4 mm under a roof that sits on
 *  it; more where the kit's roof slab begins inside the glass's box; 0 where what stands on it is not bowed). Returns the glass's own shape and the posts as
 *  trim said from the GLASS's centre, with no paint: rehome them onto a box in the body's paint. */
export function greenhouse(h: Vec3, tilt: { f: number; r: number; s: number }, posts: readonly number[] = [], low = 0.004): { glass: BoxShape; posts: ShapePiece[] } {
  const e = 0.012, w = 0.07;
  const F = leanPlane(h, 1, 0, tilt.f), R = leanPlane(h, -1, 0, tilt.r), S = (s: number) => leanPlane(h, 0, s, tilt.s);
  const out = (p: ShapePlane): ShapePlane => plane(p[0], p[1], p[2], p[3] + e), inside = (p: ShapePlane, by: number): ShapePlane => plane(-p[0], -p[1], -p[2], by - p[3]);
  // a post is cut from the glass's box, 12 mm wider each way and `low` shorter at the top, and then said as its own tightest box
  const ph: Vec3 = [h[0] + e, h[1] - low / 2, h[2] + e], pd: Vec3 = [0, -low / 2, 0];
  const post = (planes: ShapePlane[]): ShapePiece => fitted(pd, ph, planes.map(p => moved(p, pd)));
  const trim: ShapePiece[] = [];
  for (const s of [-1, 1]) {
    if (tilt.f > 0) trim.push(post([out(F), out(S(s)), inside(F, w), inside(S(s), w)]));
    if (tilt.r > 0) trim.push(post([out(R), out(S(s)), inside(R, w), inside(S(s), w)]));
    for (const x of posts) trim.push(post([out(S(s)), inside(S(s), 0.03), plane(1, 0, 0, x + 0.045), plane(-1, 0, 0, 0.045 - x)]));
  }
  return { glass: { ...cutOf(h, [...(tilt.f > 0 ? [F] : []), ...(tilt.r > 0 ? [R] : []), ...(tilt.s > 0 ? [S(1), S(-1)] : []), ...(low > 0 ? [plane(0, 1, 0, h[1] - low)] : [])]) }, posts: trim };
}
/** where a leaning greenhouse's top edge lies, for the roof over it: the glass's half size, its tilts, and how far the roof oversails (more than a post is
 *  proud). Returns the reach from the GLASS's centre: toward the nose, toward the tail. */
export function glassTop(h: Vec3, tilt: { f: number; r: number; s: number }, over = 0.03): { front: number; rear: number } {
  const H = 2 * h[1]; return { front: h[0] - H * Math.tan(tilt.f * D) + over, rear: h[0] - H * Math.tan(tilt.r * D) + over };
}
/** A ROOF over a greenhouse (its plain box of half size h, its centre `dx` toward the nose from the glass's): bowed from side to side by `rise`, and cut
 *  back at the nose and the tail to the glass's top edge (`top`: glassTop) where it reached beyond */
export function roofShape(h: Vec3, dx: number, rise: number, top: { front: number; rear: number }): BoxShape {
  return { arc: { axis: 1, rise }, old: true, ...cutOf(h, [plane(1, 0, 0, top.front - dx), plane(-1, 0, 0, top.rear + dx)]) };
}
/** A LAMP'S LENS (its plain box of half size h, its centre `d` from the body's in the body's frame): its four corners taken off, and where it sits on a
 *  rounded corner of the body (`bodyCut`: the body's own planes) its end taken back with that corner, so it lies `proud` of the panel all along */
export function lensShape(h: Vec3, bodyCut: readonly ShapePlane[], d: Vec3, proud = 0.012): BoxShape | undefined {
  return said({ ...cutOf(h, [...withCorners(bodyCut, d, proud), ...edgesOff(h, Math.min(h[1], h[2]) * 0.8)]) });
}
/** THE GLOW A LEVEL HANGS ON A LAMP THAT IS LIT (the lamp's shape and half size `lh`; the glow's half size h, its centre `d` from the lamp's in the
 *  lamp's own frame — out in front of it along x — and `grow` larger across): the OUTLINE OF THE LAMP'S LIT FACE carried straight out to the glow. A
 *  plane of the lamp that slants across x (the body's rounded corner the lens was taken back with) is met where it crosses that face and stands
 *  upright from there: the light ends where the lens's face does, and is not cut again by a corner it stands in front of (carried on as it slants, the
 *  corner's plane took most of a lit lens away: a round at the lamp then met the dead lens behind it) */
export function glowShape(lamp: BoxShape | undefined, lh: Vec3, h: Vec3, d: Vec3, grow: number): BoxShape | undefined {
  if (!lamp?.cut) return undefined;
  const face = Math.sign(d[0]) * lh[0], out: ShapePlane[] = [];
  for (const p of lamp.cut) { const m = Math.hypot(p[1], p[2]); if (m > 1e-9) out.push(plane(0, p[1] / m, p[2] / m, (p[3] - p[0] * face - p[1] * d[1] - p[2] * d[2]) / m + grow)); }
  return said({ ...cutOf(h, out) });
}
/** what stands on or in front of a body's rounded corners, cut back with them (a bumper `proud` of them; a bed's side, a tailgate flush) */
export function onCorners(h: Vec3, bodyCut: readonly ShapePlane[], d: Vec3, proud = 0, more: ShapePlane[] = []): BoxShape | undefined {
  return said({ ...cutOf(h, [...withCorners(bodyCut, d, proud), ...more]) });
}
/** the four long edges of a box that lies along its own x, taken off to radius r (a tank that is not square across) */
const edgesOff = (h: Vec3, r: number): ShapePlane[] => { const k = Math.SQRT1_2; return [-1, 1].flatMap(sy => [-1, 1].map(sz => plane(0, sy * k, sz * k, k * ((h[1] - r) + (h[2] - r)) + r))); };

// ---- what lies or stands on a bowed top ----
/** A BOWED TOP of a vehicle: the box that says the arc — its centre in the vehicle's own frame, its half size, and its shape (which takes the trim laid
 *  on it). Every vehicle here hands out its own (`tops`), so what a level lays or stands on a roof is fitted to the roof by ONE rule, below. */
export interface BowedTop { c: Vec3; h: Vec3; shape: BoxShape }
/** the round a bowed top lies on: the index of the axis ACROSS the bow (0 = x, 2 = z), the top's half width there, and the round's radius */
const bowOf = (t: BowedTop) => { const axis = bowAxis(t), x: 0 | 2 = axis === 1 ? 2 : 0, w = t.shape.arc ? t.h[x] : Math.min(t.h[1], t.h[x]), rise = t.shape.arc?.rise ?? w; return { x, w, R: (w * w + rise * rise) / (2 * rise) }; };
/** the axis a bowed top is round about: an arc's, or a ROUND's own — a pipe, a log or a drum lying is a bowed top too (its top half is the half round an arc
 *  may at most be: its box's top is the crown, and its radius the smaller half across its axis, as scene/shapes.ts cuts it) */
const bowAxis = (t: BowedTop): 1 | 3 => { const a = t.shape.arc?.axis ?? t.shape.round; if (a === undefined) throw new Error('carShapes: a bowed top says an arc or a round (a flat top takes no sheet that follows a bow)'); return a; };
/** how far a bowed top has fallen `s` metres to the side of its crown */
export const fallOf = (t: BowedTop, s: number): number => { const { R } = bowOf(t); return R - Math.sqrt(Math.max(0, R * R - s * s)); };
/** SNOW OR DUST LAID ON A BOWED TOP, as a level lays it: a thin flat slab (its centre `c` in the vehicle's frame, its half size h, lying along the vehicle).
 *  A slab cannot bend, and a shape only takes away, so the slab KEEPS TO THE CROWN: it is flat as far as half of where the top has dropped `gap` (2 cm)
 *  under it and falls to nothing there, and it is cut back with the top's own planes (a roof's ends, a bonnet's fall); and a SHEET of the same paint
 *  follows the bow from there, 2 cm over it, as wide as the slab or three quarters of the top (it has slid off the shoulders) — trim of the top's own box,
 *  pushed onto its shape, so it is there only when shapes are on. `nth`: the slab lies on `nth` earlier falls (a second fall is a ridge half as wide, and
 *  lays no sheet). Returns the slab's own shape, or nothing where it need say none. */
export function layOn(t: BowedTop, c: Vec3, h: Vec3, paint: CarPaint, o: { nth?: number; gap?: number } = {}): BoxShape | undefined {
  const { x, w, R } = bowOf(t), nth = o.nth ?? 0, gap = o.gap ?? 0.02, lift = 0.02, d: Vec3 = sub(c, t.c);
  const keep = Math.sqrt(2 * R * gap - gap * gap) / 2 ** nth, flat = keep / 2;
  // the slab: from (flat, its top) down to (keep, its underside) each side of the crown, which lies −d[x] from the slab's own middle
  const a = keep - flat, b = 2 * h[1], k = b * flat + a * h[1], side = (sg: number): ShapePlane => x === 2 ? plane(0, a, sg * b, k - sg * b * d[2]) : plane(sg * b, a, 0, k - sg * b * d[0]);
  const own = (t.shape.cut ?? []).map(p => moved(p, d, gap));
  if (nth === 0 && h[x] - Math.abs(d[x]) > keep) {   // the sheet: concentric with the bow, said from the top's own centre
    const wide = Math.min(h[x] - Math.abs(d[x]), 0.75 * w), R2 = R + lift, rise = r6(R2 - Math.sqrt(R2 * R2 - wide * wide)), off: Vec3 = x === 2 ? [d[0], r6(t.h[1] + lift - rise / 2), 0] : [0, r6(t.h[1] + lift - rise / 2), d[2]];
    const sh: Vec3 = x === 2 ? [h[0], r6(rise / 2), r6(wide)] : [r6(wide), r6(rise / 2), h[2]], cut = biting(sh, (t.shape.cut ?? []).map(p => moved(p, off, lift)));
    (t.shape.with ??= []).push({ off, h: sh, ...paintOf(paint), arc: { axis: bowAxis(t), rise }, ...(cut.length ? { cut } : {}), old: true });
  }
  return said({ ...cutOf(h, [side(1), side(-1), ...own]) });
}
/** FEET under a thing that stands on a bowed top (a rack's rail, a ladder's stile: its centre `c` in the vehicle's frame, its half size h, lying ALONG THE
 *  VEHICLE whichever way the top is bowed): one at each of `at` (metres along the thing from its middle) that is over the top, from the thing's underside
 *  down to the bow where that foot stands — trim of the THING's own box, in its paint */
export function feetOn(t: BowedTop, c: Vec3, h: Vec3, at: readonly number[], foot = 0.02): ShapePiece[] {
  const { x } = bowOf(t), d = sub(c, t.c), ends = (t.shape.cut ?? []).filter(p => Math.abs(p[1]) < 1e-6), fz = Math.min(h[2], foot), out: ShapePiece[] = [];
  for (const u of at) {
    const q: Vec3 = [d[0] + u, 0, d[2]];   // the foot's place on the top, from the top's own middle
    if (Math.abs(q[0]) >= t.h[0] || Math.abs(q[2]) >= t.h[2] || !ends.every(p => p[0] * q[0] + p[2] * q[2] <= p[3])) continue;
    const drop = r6(d[1] - h[1] - t.h[1] + fallOf(t, Math.abs(q[x]) + (x === 2 ? fz : foot)));   // (to the bow under the foot's lower edge)
    if (drop >= 0.004) out.push({ off: [r6(u), r6(-h[1] - drop / 2), 0], h: [foot, r6(drop / 2), fz] });
  }
  return out;
}

export interface CarPaints { hub: CarPaint }

/** a pickup's body: its half size, and its four rounded corners (said once: its own shapes and a level's lit lens on it are cut by the same planes) */
const PICKUP_TUB: Vec3 = [2.45, 0.25, 0.86], pickupCorners = (): ShapePlane[] => corners(PICKUP_TUB, [0.22, 3], [0.1, 1]);
/** a pickup's head lamp as the kit stands it: its half size, and its centre from the pickup's middle (x toward the nose, z to either side) */
const PICKUP_HEAD = { h: [0.03, 0.07, 0.16] as Vec3, x: 2.43, z: 0.66 };
/** A PICKUP, 5.0 × 1.8, nose toward local +x (scene/gwotKit.ts pickup): the shape each box of its stack says */
export function pickupShapes(P: CarPaints) {
  const tubC: Vec3 = [0, 0.55, 0], tubH = PICKUP_TUB, tubCut = pickupCorners();
  const glassC: Vec3 = [0.38, 1.29, 0], glassH: Vec3 = [0.64, 0.21, 0.8], tilt = { f: 30, r: 12, s: 12 }, top = glassTop(glassH, tilt), G = greenhouse(glassH, tilt);
  const doorsC: Vec3 = [0.38, 0.94, 0], wheelH: Vec3 = [0.37, 0.37, 0.13], bonnetH: Vec3 = [0.7, 0.1, 0.84];
  const arches = [-1.5, 1.52].flatMap(lx => [-1, 1].map(s => archPiece([lx, 0.37, s * 0.78], wheelH, s, tubC)));
  const roof = roofShape([0.6, 0.04, 0.78], 0.36 - 0.38, 0.08, top);
  // (the bonnet: crowned, falling to the nose, its two nose corners taken back with the body's — and no panel lines: a mark is laid out on a whole face)
  const bonnet: BoxShape = { arc: { axis: 1, rise: 0.2 }, ...cutOf(bonnetH, [fallPlane(bonnetH, 1, 0.2, 0.15), ...withCorners([cornerPlanes(tubH, 0.22, 1, 1, 3)[1], cornerPlanes(tubH, 0.22, 1, -1, 3)[1]], [1.75, 0, 0])]), old: true };
  return {
    tops: { roof: { c: [0.36, 1.54, 0], h: [0.6, 0.04, 0.78], shape: roof } as BowedTop, bonnet: { c: [1.75, 0.9, 0], h: bonnetH, shape: bonnet } as BowedTop },
    wheel: (s: number) => wheelShape(wheelH, s, P.hub),
    tub: { cut: tubCut, old: true, with: arches } as BoxShape,
    bumperFront: onCorners([0.05, 0.11, 0.88], tubCut, [2.47, 0, 0], 0.09), bumperRear: onCorners([0.05, 0.11, 0.88], tubCut, [-2.47, 0, 0], 0.09),
    bonnet,
    doors: { with: rehome(G.posts, sub(doorsC, glassC)) } as BoxShape,
    glass: G.glass,
    roof,
    bedSide: (s: number) => onCorners([1.03, 0.19, 0.03], tubCut, [-1.42, 0, s * 0.83]), tailgate: onCorners([0.03, 0.19, 0.86], tubCut, [-2.42, 0, 0]),
    head: (s: number) => lensShape(PICKUP_HEAD.h, tubCut, [PICKUP_HEAD.x, 0, s * PICKUP_HEAD.z]), tail: (s: number) => lensShape([0.03, 0.08, 0.1], tubCut, [-2.43, 0, s * 0.72]),
  };
}

/** a lit lens a level stands in front of a pickup's head lamp (half size h, its centre `d` from the pickup's middle on the ground: [x, _, z], at the kit
 *  lens's own height): the outline of the kit's lens there, carried out to it (glowShape) */
export const pickupLens = (h: Vec3, d: Vec3): BoxShape | undefined => { const s = Math.sign(d[2]) || 1, at: Vec3 = [PICKUP_HEAD.x, 0, s * PICKUP_HEAD.z];
  return glowShape(lensShape(PICKUP_HEAD.h, pickupCorners(), at), PICKUP_HEAD.h, h, [d[0] - at[0], 0, d[2] - at[2]], 0); };

/** A PANEL VAN, 4.9 × 1.8 × 1.95, nose toward local +x, its back doors open (scene/gwotKit.ts van): the shape each box of its stack says. Its roof is one
 *  bow of 6 cm from the windscreen to the back doors — the brow, the load box and the strip over the open bay each say the same arc, and the bay's sides
 *  stop under it */
export function vanShapes(P: CarPaints) {
  const bodyC: Vec3 = [0, 0.58, 0], bodyH: Vec3 = [2.4, 0.28, 0.9], bodyCut = corners(bodyH, [0.22, 3], [0, 0]);   // (a square tail: a van's tail lamps stand on its back corners, and a corner taken off there took the lamp with it)
  const glassC: Vec3 = [1.3, 1.36, 0], glassH: Vec3 = [0.36, 0.26, 0.84], tilt = { f: 28, r: 0, s: 4 }, bow = 0.06, G = greenhouse(glassH, tilt, [-0.3], 0);   // (the brow stands on the glass, flat underneath: nothing to stop short of)
  const doorsC: Vec3 = [1.3, 0.98, 0], wheelH: Vec3 = [0.37, 0.37, 0.13], bonnetH: Vec3 = [0.4, 0.1, 0.86], browC: Vec3 = [1.22, 1.785, 0], browH: Vec3 = [0.34, 0.165, 0.86], sideH: Vec3 = [0.25, 0.545, 0.03];
  const arches = [-1.45, 1.5].flatMap(lx => [-1, 1].map(s => archPiece([lx, 0.37, s * 0.8], wheelH, s, bodyC)));
  const load: BoxShape = { arc: { axis: 1, rise: bow }, old: true };
  return {
    tops: { load: { c: [-0.5, 1.405, 0], h: [1.4, 0.545, 0.9], shape: load } as BowedTop },
    wheel: (s: number) => wheelShape(wheelH, s, P.hub),
    body: { cut: bodyCut, old: true, with: arches } as BoxShape,
    bumper: onCorners([0.05, 0.11, 0.9], bodyCut, [2.43, 0, 0], 0.09), step: onCorners([0.05, 0.07, 0.9], bodyCut, [-2.43, 0, 0], 0.09),
    bonnet: { arc: { axis: 1, rise: 0.2 }, ...cutOf(bonnetH, [fallPlane(bonnetH, 1, 0.18, 0.15), ...withCorners([cornerPlanes(bodyH, 0.22, 1, 1, 3)[1], cornerPlanes(bodyH, 0.22, 1, -1, 3)[1]], [2.0, 0, 0])]), old: true } as BoxShape,
    doors: { with: rehome(G.posts, sub(doorsC, glassC)) } as BoxShape,
    glass: G.glass,
    // (the brow: its face is the windscreen's own plane carried up — the glass's, said from the brow's centre)
    brow: { arc: { axis: 1, rise: bow }, ...cutOf(browH, [moved(leanPlane(glassH, 1, 0, tilt.f), sub(browC, glassC))]), old: true } as BoxShape,
    load, bayRoof: { arc: { axis: 1, rise: bow }, old: true } as BoxShape,
    baySide: (s: number) => onCorners(sideH, bodyCut, [-2.15, 0, s * 0.87], 0, [plane(0, 1, 0, sideH[1] - bow)]),   // (its top comes down to the roof's edge; the van's tail is square, so no corner takes anything of it)
    // (a van's head lamp stands its whole 6 cm in front of the body's face, so it lies that far proud of the corner too: taken back only 12 mm proud, the
    // corner's planes carried forward left a third of its face, and of the lit glow a level hangs on it)
    head: (s: number) => lensShape([0.03, 0.07, 0.16], bodyCut, [2.43, 0, s * 0.66], 0.06), tail: (s: number) => lensShape([0.02, 0.15, 0.04], bodyCut, [-2.41, 0, s * 0.86]),
  };
}

/** A CAR, 4.4 × 1.72, nose toward local +x (scene/officeGen.ts and scene/levelKit.ts car): the shape each box of its stack says */
export function carShapes(P: CarPaints) {
  const bodyC: Vec3 = [0, 0.62, 0], bodyH: Vec3 = [2.2, 0.34, 0.86], bodyCut = [...corners(bodyH, [0.24, 2], [0.16, 1]), fallPlane(bodyH, 1, 0.5, 0.16), fallPlane(bodyH, -1, 0.3, 0.1)];
  const glassC: Vec3 = [-0.25, 1.2, 0], glassH: Vec3 = [1.25, 0.27, 0.8], tilt = { f: 38, r: 42, s: 12 }, top = glassTop(glassH, tilt), wheelH: Vec3 = [0.32, 0.3, 0.12];
  const G = greenhouse(glassH, tilt, [0.05], 0.014);   // (the kit's roof slab begins a centimetre under the glass box's top: the glass stops under the slab)
  const arches = [1.35, -1.4].flatMap(lx => [-1, 1].map(s => archPiece([lx, 0.3, s * 0.8], wheelH, s, bodyC)));
  const roof = roofShape([1.15, 0.03, 0.76], 0, 0.06, top);
  return {
    tops: { roof: { c: [-0.25, 1.49, 0], h: [1.15, 0.03, 0.76], shape: roof } as BowedTop },
    wheel: (s: number) => wheelShape(wheelH, s, P.hub),
    body: { cut: bodyCut, old: true, with: [...arches, ...rehome(G.posts, sub(bodyC, glassC))] } as BoxShape,
    glass: G.glass,
    roof,
    head: (s: number) => lensShape([0.02, 0.07, 0.14], bodyCut, [2.2, 0, s * 0.62]), tail: (s: number) => lensShape([0.02, 0.06, 0.16], bodyCut, [-2.2, 0, s * 0.62]),
  };
}

/** A SMALL MOTORBIKE on its stand, nose toward local +x (scene/gwotKit.ts motorbike): the shape each box of its stack says. Its tank carries the trim: a
 *  mudguard over each wheel, in the tank's paint. Its fork stays the square tube it is. */
export function bikeShapes() {
  const tankC: Vec3 = [0.18, 0.7, 0], tankH: Vec3 = [0.24, 0.14, 0.12];
  const guard = (lx: number): ShapePiece => ({ off: sub([lx, 0.4625, 0], tankC), h: [0.325, 0.1625, 0.04], arc: { axis: 3, rise: 0.325 }, old: true });
  const seat: BoxShape = { arc: { axis: 1, rise: 0.07 }, old: true };
  return {
    tops: { seat: { c: [-0.36, 0.71, 0], h: [0.34, 0.09, 0.11], shape: seat } as BowedTop },
    wheel: { round: 3 } as BoxShape,
    tank: { cut: edgesOff(tankH, 0.05), old: true, with: [guard(0.66), guard(-0.62)] } as BoxShape,   // (not square across: its four long edges taken off)
    seat,
    bars: { round: 3 } as BoxShape,
    lamp: { round: 1 } as BoxShape,
    exhaust: { round: 1 } as BoxShape,
  };
}

// ---- the lorry ----
/** the paints a lorry's trim of other stuffs is said in (the kit's own Mats) */
export interface LorryPaints { canvas: CarPaint; iron: CarPaint; galv: CarPaint; rust: CarPaint; rubber: CarPaint; poplar: CarPaint; white: CarPaint; glass: CarPaint; jerry: CarPaint; olive: CarPaint; orange: CarPaint; red: CarPaint }
/** the eight planes that make a box of square plan a twelve-sided prism of radius r about its own y (a piece of trim that stands on its end: a stack, a
 *  canister, a beacon — a BOX of the level that is round on its end is the kit's round thing, scene/round.ts) */
const ringY = (r: number): { cut: ShapePlane[]; old: true } => ({ cut: [30, 60, 120, 150, 210, 240, 300, 330].map(d => plane(Math.cos(d * D), 0, Math.sin(d * D), r)), old: true });
/** trim said from the vehicle's own frame, for a box of it whose centre is `o` there: (lx, lz, y0, y1, hx, hz, paint | null = the box's own, form) */
const trimFor = (o: Vec3) => (lx: number, lz: number, y0: number, y1: number, hx: number, hz: number, p: CarPaint | null, form: Partial<ShapePiece> = {}): ShapePiece =>
  ({ off: sub([lx, (y0 + y1) / 2, lz], o), h: [hx, r6((y1 - y0) / 2), hz], ...(p ? paintOf(p) : {}), ...form });
/** A TWO-AXLE LORRY with a canvas tilt, 7 m, nose toward local +x (scene/gwotKit.ts lorry; asked 2026-10-24 of the pass's: "some curved surfaces on the
 *  truck would go a long way", and 10-30, of the first curves: "truck could use some dressing up. but overall the round surfaces look nice"): the shape
 *  each box of its stack says, and the fittings a lorry of its kind carries as trim.
 *  · the canvas tilt is BOWED over its hoops (0.15 m: its eaves stay over a man's head); the cab's roof is an arch from the windscreen back, and the
 *    windscreen leans;
 *  · the bonnet is an engine cover that narrows to a rounded top, between two FENDERS that arch over the front wheels; a ring round each lamp;
 *  · the wheels are round, with hubs; a round tank hangs under the bed on each side;
 *  · an exhaust stack up the cab's back corner, mirrors on arms, a beacon, steps, a door seam and a handle a side, an air cleaner on the cowl, a winch drum,
 *    hooks and a plate on the bumper, stakes down the sideboards, three jerry cans racked on one of them, mud flaps, tail lamps, the spare on the tailboard.
 *  WHAT A RAY MEETS LESS OF: the tilt's shoulders above 1.8 m, the bonnet's (it narrows from 0.98 m a side to 0.56 at its top, 1.5 m up), the cab roof's front
 *  and back edges, the top of the windscreen. Everything else here is trim: a tank, a fender, a mirror stop nothing. A level that snows it in lays its caps on `tops` (layOn). */
export function lorryShapes(P: LorryPaints) {
  const hub = hubPaint(P.galv.albedo), zinc = { ...tone(P.galv, 0.6), rough: 0.38, metal: true }, iron = { ...P.iron, rough: 0.5, metal: true }, galv = { ...P.galv, rough: 0.38, metal: true };
  const wheelH: Vec3 = [0.48, 0.48, 0.15], bonnetC: Vec3 = [2.75, 1.12, 0], bonnetH: Vec3 = [0.62, 0.38, 0.98], doorsC: Vec3 = [1.55, 1.12, 0], glassC: Vec3 = [1.55, 1.79, 0];
  // the chassis (its centre (−0.1, 0.62, 0)): a tank under the bed in its two bands, a step under the door (behind the front wheel) and a mud flap behind the rear wheel, a side
  const ch = trimFor([-0.1, 0.62, 0]), chassis: BoxShape = { with: [-1, 1].flatMap(s => [
    ch(0.1, s * 0.82, 0.25, 0.69, 0.5, 0.22, hub, { round: 1 }), ...[-1, 1].map(e => ch(0.1 + e * 0.3, s * 0.82, 0.24, 0.9, 0.012, 0.228, null)),   // (the tank hangs from the bed's underside in two bands, in the chassis' iron: it touched nothing)
    ch(1.36, s * 1.16, 0.44, 0.49, 0.3, 0.09, zinc), ...[-1, 1].map(e => ch(1.36 + e * 0.24, s * 1.1, 0.49, 0.74, 0.012, 0.02, null)),   // (the step hangs from the cab's floor on two straps, in the chassis' iron)
    ch(-2.8, s * 0.98, 0.16, 0.72, 0.012, 0.17, { ...P.rubber, rough: 0.9 })]) };
  // the bumper (3.42, 0.64, 0): the winch's drum between two cheeks, the plate, a hook a side
  const bu = trimFor([3.42, 0.64, 0]), bumper: BoxShape = { with: [
    bu(3.56, 0, 0.56, 0.76, 0.1, 0.26, { ...P.rust, rough: 0.85 }, { round: 3 }), ...[-1, 1].map(s => bu(3.56, s * 0.275, 0.53, 0.79, 0.13, 0.012, iron, { round: 3 })),
    bu(3.486, -0.82, 0.54, 0.72, 0.004, 0.2, { ...P.white, rough: 0.45 }), ...[-1, 1].map(s => bu(3.53, s * 0.62, 0.58, 0.68, 0.05, 0.03, iron))] };
  // the bonnet: it narrows to a rounded top. Its trim, in its own paint: a fender arched over each front wheel (an arc about the wheel's own axle of
  // radius 0.6, 1.08 m long, from the engine cover's side out over the tyre) and a ring round each lamp, proud of its front; and the grille, the air cleaner
  const bo = trimFor(bonnetC), fw = 0.54, fR = 0.6, fRise = r6(fR - Math.sqrt(fR * fR - fw * fw));
  const bonnet: BoxShape = { arc: { axis: 1, rise: 0.26 }, cut: [leanPlane(bonnetH, 0, 1, 29), leanPlane(bonnetH, 0, -1, 29)], old: true, with: [
    ...[-1, 1].map(s => bo(2.2, s * 0.855, 1.08 - fRise, 1.08, fw, 0.295, null, { arc: { axis: 3, rise: fRise }, old: true })),
    ...[-1, 1].map(s => bo(3.384, s * 0.7, 0.82, 1.06, 0.012, 0.12, null, { round: 1 })),
    bo(3.376, 0, 0.8, 1.3, 0.006, 0.4, iron),
    bo(2.3, 0.74, 1.02, 1.44, 0.1, 0.1, zinc, ringY(0.1)), bo(2.3, 0.74, 1.44, 1.47, 0.115, 0.115, iron, ringY(0.115))] };
  // the glass (0.58 × 0.29 × 1.06): the windscreen leans, and it stops 4 mm under the roof; its two posts are the cab's
  const G = greenhouse([0.58, 0.29, 1.06], { f: 14, r: 0, s: 0 });
  // the cab's doors: the windscreen's posts; the exhaust stack in its heat shield with a rain cap; and a side: a mirror on its arm off the post, the door's
  // seam, its handle
  const ca = trimFor(doorsC), cab: BoxShape = { with: [
    ...rehome(G.posts, sub(doorsC, glassC)),
    ca(1.0, 1.2, 0.55, 2.5, 0.045, 0.045, { ...P.rust, rough: 0.85 }, ringY(0.045)), ca(1.0, 1.2, 1.05, 1.9, 0.07, 0.07, zinc, ringY(0.07)), ca(1.03, 1.2, 2.5, 2.515, 0.07, 0.055, { ...P.rust, rough: 0.85 }),
    ...[-1, 1].flatMap(s => [ca(2.06, s * 1.25, 1.74, 1.77, 0.012, 0.19, iron), ca(2.06, s * 1.43, 1.58, 1.86, 0.015, 0.07, iron), ca(2.043, s * 1.43, 1.6, 1.84, 0.003, 0.06, { ...P.glass, rough: 0.1 }),
      ca(1.02, s * 1.124, 0.78, 1.5, 0.008, 0.004, iron), ca(1.16, s * 1.132, 1.2, 1.235, 0.06, 0.012, galv)])] };
  // the roof (1.5, 2.14, 0): an arch from the windscreen back; the beacon on its crown
  const ro = trimFor([1.5, 2.14, 0]), roof: BoxShape = { arc: { axis: 3, rise: 0.12 }, old: true, with: [ro(1.5, 0, 2.17, 2.36, 0.07, 0.07, { ...P.orange, rough: 0.35 }, ringY(0.07))] };
  // the load under its canvas (−1.15, 1.425, 0): bowed over four hoops that show through the canvas
  const lo = trimFor([-1.15, 1.425, 0]), bow = 0.15;
  const load: BoxShape = { arc: { axis: 1, rise: bow }, old: true, with: [-2.65, -1.65, -0.65, 0.35].map(lx => lo(lx, 0, 1.95 - bow, 1.962, 0.02, 1.066, { ...tone(P.canvas, 0.72), rough: 0.9 }, { arc: { axis: 1, rise: r6(bow + 0.012) }, old: true })) };
  // a sideboard (−1.15, 1.175, ±1.14): four stakes it is let into; on the driver's side three jerry cans in a rack under a strap
  const side = (s: number): BoxShape => { const sb = trimFor([-1.15, 1.175, s * 1.14]); return { with: [
    ...[-2.95, -1.75, -0.55, 0.65].map(lx => sb(lx, s * 1.185, 0.76, 1.5, 0.035, 0.015, { ...tone(P.poplar, 0.75), rough: 0.8 })),
    ...(s > 0 ? [...[0, 1, 2].map(i => sb(-1.42 + i * 0.2, 1.33, 0.98, 1.44, 0.085, 0.13, i === 1 ? { ...tone(P.jerry, 0.9), rough: 0.45 } : { ...P.olive, rough: 0.45 })), sb(-1.22, 1.465, 1.24, 1.27, 0.3, 0.004, iron), sb(-1.22, 1.33, 0.95, 0.98, 0.32, 0.14, iron)] : [])] }; };
  // the tailboard (−3.17, 1.175, 0): the spare, upright, its hub outward, and a tail lamp a side
  const ta = trimFor([-3.17, 1.175, 0]), tail: BoxShape = { with: [
    ta(-3.35, 0, 0.75, 1.71, 0.15, 0.48, { ...P.rubber, rough: 0.9 }, { round: 1 }), ta(-3.504, 0, 1.03, 1.43, 0.004, 0.2, zinc, { round: 1 }),
    ...[-1, 1].map(s => ta(-3.206, s * 0.92, 0.98, 1.1, 0.006, 0.06, { ...P.red, rough: 0.3 }, { round: 1 }))] };
  return {
    wheel: (s: number): BoxShape => wheelShape(wheelH, s, hub),
    /** where snow lies on it: the tilt, the cab's roof, the engine cover (layOn) */
    tops: { load: { c: [-1.15, 1.425, 0], h: [1.95, 0.525, 1.06], shape: load } as BowedTop, roof: { c: [1.5, 2.14, 0], h: [0.62, 0.06, 1.1], shape: roof } as BowedTop, bonnet: { c: bonnetC, h: bonnetH, shape: bonnet } as BowedTop },
    chassis, bumper, bonnet, cab, glass: G.glass, roof, load, side, tail,
    lamp: (s: number) => lensShape([0.02, 0.08, 0.1], [], [3.38, 0, s * 0.7]),
  };
}

/** the hub of every kit's wheel: galvanised steel in a wheel's shadow */
export const hubPaint = (galv: Vec3): CarPaint => ({ albedo: [r6(galv[0] * 0.6), r6(galv[1] * 0.6), r6(galv[2] * 0.6)], rough: 0.45, metal: true });
