// VEHICLES WITH CURVES, A TRIAL (the look page only: main.ts carTryOn; ?cars=0 = every vehicle as its level file has it). Asked 2026-11-04 from the library's
// showroom: "could we make it so the cars have rounded surfaces? look really blocky right now". Every car, pickup, van and motorbike the kits build is a
// stack of plain boxes: from above every line of it is straight. Here each is gone over after the level loads, by the shapes below, wherever a level has one:
//   · the body's corners are rounded in plan (more at the nose than at the tail), and a bumper wraps round them;
//   · the bonnet is crowned from side to side and falls to the nose; the roof is bowed; a van's whole roof is one bow from the windscreen to the back doors;
//   · the glass leans: the windscreen back, the rear window forward, the sides in;
//   · the wheels are round, with hubs, under arches that stand out of the body's side; lamps have their corners taken off;
//   · a motorbike's tank, forks, bars, lamp and exhaust are round, its seat is bowed and it has mudguards.
// Made of what the engine has, as the pass's lorry is (scene/lorryTry.ts): a round prism (Box.cutRound), AN ARC (Box.cutArc) for a shallow curve, and plain
// cut planes for a corner in plan or a leaning pane. ONE MODEL: each piece is a real box to every ray, and what pushes against it meets the uncut box.
// THE SHAPES ARE GENERAL (pickupShape, vanShape, carShape, bikeShape: a frame and its paints in, boxes out) and are meant to become the kit's own pieces, said
// in the level file. WHAT IS A TRIAL is the finding: `dressCars` knows a vehicle in a loaded level by the boxes its generator writes (it runs the kit's own
// builder and looks for that stack), which is how this page can show every level's vehicles before one level file changes.
// WHAT A RAY AND A BODY MEET: a vehicle's outline on the ground is the one it had (no outer box moved: the walk grid and a man's shoulder meet what they
// met). A ray meets less: the body's corners (0.22 m of the nose, 0.1 m of the tail), the bonnet's shoulders (it falls 0.2 m from its middle to its sides)
// and the glass above the waist, which leans in by up to 0.42 m at the roof of a car. A lamp keeps its place, half size and paint (game/carlamps.ts finds
// it by those). Never seen on a GPU.
import { BoxFlag, type Box, type BoxStuff } from './boxes';
import { cutPlane, cutRecord, cutFit, type CutPlane } from './cut';
import type { Vec3 } from '../math/vec';
import { GWOT_PALETTE } from './gwotPalette';
import { gwotKit, type GwotHost } from './gwotKit';
import { ring, radiusOf } from './lorryTry';

const D = Math.PI / 180;
export type CarMat = { albedo: Vec3; rough?: number; metal?: boolean; stuff?: BoxStuff };
/** WHAT A VEHICLE'S PARTS ARE MADE OF (asked 11-04 of the showroom's cars: "i think the materials could use some work though ... maybe we want a car paint
 *  material too?"). Each part keeps the colour its kit box had and says its stuff and its own finish — the shine is the box's roughness, the stuff's map only
 *  varies it: a body panel is car paint and smooth, glass near a mirror, bumpers and liners plastic, a hub bright steel, a tyre matt rubber. */
const MADE = {
  body: (m: CarMat): CarMat => ({ albedo: m.albedo, rough: 0.2, stuff: 'carpaint' }),
  glass: (m: CarMat): CarMat => ({ albedo: m.albedo, rough: 0.07, stuff: 'plain' }),
  trim: (m: CarMat): CarMat => ({ albedo: m.albedo, rough: 0.5, stuff: 'plastic' }),
  hub: (m: CarMat): CarMat => ({ albedo: m.albedo, rough: 0.25, metal: true, stuff: 'steel' }),
  tyre: (m: CarMat): CarMat => ({ albedo: m.albedo, rough: 0.9, stuff: 'plain' }),
};
export interface CarFrame { x: number; z: number; yaw: number }
type Put = (lx: number, lz: number, y0: number, y1: number, hx: number, hz: number, m: CarMat, o?: Partial<Box>, dyaw?: number) => Box;
const of = (m: CarMat, k: number): CarMat => ({ ...m, albedo: [m.albedo[0] * k, m.albedo[1] * k, m.albedo[2] * k] });

/** a piece's local frame, nose toward local +x (the kits' frameAt): put(…) makes one box of it and appends it to `out` */
function putter(F: CarFrame, out: Box[]): Put {
  const cs = Math.cos(F.yaw), sn = Math.sin(F.yaw);
  return (lx, lz, y0, y1, hx, hz, m, o = {}, dyaw = 0) => {
    const b: Box = { c: [F.x + cs * lx + sn * lz, (y0 + y1) / 2, F.z - sn * lx + cs * lz], h: [hx, (y1 - y0) / 2, hz], yaw: F.yaw + dyaw, albedo: m.albedo, emissive: [0, 0, 0], flags: 0, owner: 0, cutHeight: 1000000, rough: m.rough ?? 0.5, ...(m.metal ? { metal: true } : {}), ...(m.stuff ? { stuff: m.stuff } : {}), ...o };
    out.push(b); return b;
  };
}

// ---- the shapes a box can be given (each returns the box) ----
/** the planes that round the upright edge at (sx·h.x, sz·h.z) to radius r, in `n` facets between the two faces */
export const corner = (h: Vec3, r: number, sx: number, sz: number, n: number): CutPlane[] => Array.from({ length: n }, (_, i) => { const a = (i + 1) * 90 / (n + 1) * D; return cutPlane([sx * Math.cos(a), 0, sz * Math.sin(a)], Math.cos(a) * (h[0] - r) + Math.sin(a) * (h[2] - r) + r); });
/** the plane that leans the face at sx·h.x (or, with sx = 0, at sz·h.z) inward by `deg` from upright, about its bottom edge */
export const lean = (h: Vec3, sx: number, sz: number, deg: number): CutPlane => { const t = deg * D; return sx ? cutPlane([sx * Math.cos(t), Math.sin(t), 0], Math.cos(t) * h[0] - Math.sin(t) * h[1]) : cutPlane([0, Math.sin(t), sz * Math.cos(t)], Math.cos(t) * h[2] - Math.sin(t) * h[1]); };
/** the plane from the top face `back` metres in from the end at sx·h.x down to `drop` metres under the top at that end: a bonnet falling to its nose */
export const fall = (h: Vec3, sx: number, back: number, drop: number): CutPlane => { const L = Math.hypot(drop, back), n: Vec3 = [sx * drop / L, back / L, 0]; return cutPlane(n, n[0] * sx * h[0] + n[1] * (h[1] - drop)); };
/** cut `b` by planes; `old` = its cut faces take the paint its own faces take (Box.cutOld: a panel pressed that way, not a break) */
export function cutTo(b: Box, planes: CutPlane[], old = true): Box { const c = cutRecord(b.h, planes).cut; if (c) { b.cut = c; if (old) b.cutOld = true; delete b.marks; } return b; }
/** make `b` a round prism of radius r about its own x (1), y (2) or z (3) */
export function roundTo(b: Box, r: number, axis: 1 | 2 | 3): Box { b.cut = cutRecord(b.h, ring(r, axis)).cut; b.cutRound = axis; delete b.marks; return b; }
/** make `b` AN ARC: its top a round about its own x (1) or z (3) that rises `rise` from its sides to its middle, the round's highest line the box's top
 *  face; `more` = one more plane at most (the record keeps its last row for the arc) */
export function arcTo(b: Box, rise: number, axis: 1 | 3, more: CutPlane[] = []): Box {
  const w = axis === 1 ? b.h[2] : b.h[0], R = radiusOf(w, rise), k = R - b.h[1], step = Math.asin(Math.min(1, w / R)) / 3.5;
  const planes = [-3, -2, -1, 1, 2, 3].map(i => { const a = i * step; return cutPlane(axis === 1 ? [0, Math.cos(a), Math.sin(a)] : [Math.sin(a), Math.cos(a), 0], R - k * Math.cos(a)); });
  b.cut = cutRecord(b.h, [...planes, ...more.slice(0, 1)]).cut; b.cutRound = axis; b.cutArc = k; delete b.marks;
  return b;
}
/** a lamp's lens: its four corners taken off (seen from the front: an oblong with round ends), and, where it sits on a rounded corner of `body`, its end
 *  taken back with that corner so it lies `proud` of the panel all along (both boxes in one frame, the lamp's centre at (dx, dz) from the body's) */
function lens(b: Box, r: number, body?: Box, dx = 0, dz = 0, proud = 0.012): Box {
  const own = [-1, 1].flatMap(sy => [-1, 1].map(sz => cutPlane([0, sy, sz], (b.h[1] - r) + (b.h[2] - r) + r * Math.SQRT2)));
  const wrap = (body?.cut ?? []).filter(p => Math.abs(p.n[1]) < 1e-6 && p.n[0] * dx > 0).map(p => cutPlane(p.n, p.d - p.n[0] * dx - p.n[2] * dz + proud));
  return cutTo(b, [...wrap, ...own], false);
}

/** the same solid in its tightest box (cut.ts cutFit): the uncut box is what the ray grid lists and what a body meets */
function fit(b: Box): Box {
  if (!b.cut) return b;
  const f = cutFit(b.h, b.cut), cs = Math.cos(b.yaw), sn = Math.sin(b.yaw);
  b.c = [b.c[0] + cs * f.off[0] + sn * f.off[2], b.c[1] + f.off[1], b.c[2] - sn * f.off[0] + cs * f.off[2]]; b.h = f.h; b.cut = f.cut;
  return b;
}
/** A GREENHOUSE: the glass of a cab, its middle at local x = lx, from y0 to y1, its panes leaning in by `tilt` degrees (front, rear, sides) from the waist; a
 *  POST in the body's paint down each corner where two panes meet (and none where a pane is upright: `tilt` 0 there means the glass runs into what stands
 *  behind it), one across each side at every `posts` (local x from the glass's middle), all a centimetre proud of the glass; and, with `rise`, a roof bowed
 *  from side to side that oversails the glass's top edge by `over` — more than a post is proud, so the roof covers every post. NO TWO FACES IN ONE PLANE, and
 *  none through another: under a roof the glass and the posts stop 4 mm short of it (a bowed roof is thin at its edge, and what reached up into it showed
 *  through it there). Seen from above the cab is then windows between posts under a roof, not one dark block. */
function greenhouse(put: Put, lx: number, y0: number, y1: number, hx: number, hz: number, tilt: { f: number; r: number; s: number }, glass: CarMat, body: CarMat, posts: number[] = [], rise = 0, over = 0.03): void {
  const top = rise > 0 ? y1 - 0.004 : y1, g = put(lx, 0, y0, top, hx, hz, glass), h = g.h, e = 0.012, w = 0.07;
  const F = lean(h, 1, 0, tilt.f), R = lean(h, -1, 0, tilt.r), S = (s: number) => lean(h, 0, s, tilt.s);
  cutTo(g, [F, R, S(1), S(-1)], false);
  const out = (p: CutPlane, by = e): CutPlane => ({ n: p.n, d: p.d + by }), inside = (p: CutPlane, by: number): CutPlane => ({ n: [-p.n[0], -p.n[1], -p.n[2]], d: by - p.d });
  const post = (planes: CutPlane[]) => fit(cutTo(put(lx, 0, y0, top, hx + e, hz + e, body), planes));
  for (const s of [-1, 1]) {
    if (tilt.f > 0) post([out(F), out(S(s)), inside(F, w), inside(S(s), w)]);
    if (tilt.r > 0) post([out(R), out(S(s)), inside(R, w), inside(S(s), w)]);
    for (const x of posts) post([out(S(s)), inside(S(s), 0.03), cutPlane([1, 0, 0], x + 0.045), cutPlane([-1, 0, 0], 0.045 - x)]);
  }
  if (rise > 0) {
    const H = top - y0, x1 = hx - H * Math.tan(tilt.f * D) + over, x0 = -hx + H * Math.tan(tilt.r * D) - over, z = hz - H * Math.tan(tilt.s * D) + over;
    arcTo(put(lx + (x0 + x1) / 2, 0, y1, y1 + rise, (x1 - x0) / 2, z, body), rise, 1);
  }
}

/** the wheels of a vehicle: at each lx, a side, a round tyre of radius r whose outer face is `out` from the middle line, its hub, and the arch over it in the
 *  body's paint (an arc about the wheel's own axle that stands `proud` of the tyre's top and 0.02 m inside its outer face) */
function wheels(put: Put, xs: number[], out: number, r: number, wide: number, tyre: CarMat, hub: CarMat, body: CarMat | null, proud = 0.08) {
  for (const lx of xs) for (const s of [-1, 1]) {
    roundTo(put(lx, s * (out - wide / 2 - 0.004), 0, 2 * r, r, wide / 2 - 0.004, tyre), r, 3);
    cutTo(put(lx, s * (out - 0.004), r * 0.45, r * 1.55, r * 0.55, 0.004, hub, { flags: BoxFlag.NoShadow }), ring(r * 0.55, 3), false);
    if (body) { const R = r + proud; arcTo(put(lx, s * (out - 0.02 - 0.115), r, r + R, R, 0.115, body), R, 3); }
  }
}

export interface CarPaints { body: CarMat; tyre: CarMat; glass: CarMat; trim: CarMat; hub: CarMat }
const HUB = (): CarMat => ({ albedo: [GWOT_PALETTE.galv[0] * 0.6, GWOT_PALETTE.galv[1] * 0.6, GWOT_PALETTE.galv[2] * 0.6], rough: 0.45, metal: true });

/** A PICKUP, 5.0 × 1.8, nose toward local +x (scene/gwotKit.ts pickup, with curves): everything but its four lamps and what rides in its bed */
export function pickupShape(F: CarFrame, P: CarPaints): Box[] {
  const out: Box[] = [], b = putter(F, out);
  wheels(b, [-1.5, 1.52], 0.91, 0.37, 0.26, P.tyre, P.hub, P.body);
  { const t = b(0, 0, 0.3, 0.8, 2.45, 0.86, P.body); cutTo(t, [...corner(t.h, 0.22, 1, 1, 3), ...corner(t.h, 0.22, 1, -1, 3), ...corner(t.h, 0.1, -1, 1, 1), ...corner(t.h, 0.1, -1, -1, 1)]); }   // the body tub, sill to waistline
  { const f = b(2.41, 0, 0.36, 0.58, 0.11, 0.88, P.trim); cutTo(f, [...corner(f.h, 0.24, 1, 1, 3), ...corner(f.h, 0.24, 1, -1, 3)]);                      // bumpers: the front one wraps the nose
    const r = b(-2.41, 0, 0.36, 0.58, 0.11, 0.88, P.trim); cutTo(r, [...corner(r.h, 0.12, -1, 1, 2), ...corner(r.h, 0.12, -1, -1, 2)]); }
  { const n = b(1.7, 0, 0.8, 1.0, 0.65, 0.84, P.body); arcTo(n, 0.2, 1, [fall(n.h, 1, 0.2, 0.15)]); }                                                       // bonnet: crowned, falling to the nose
  b(0.38, 0, 0.8, 1.08, 0.7, 0.86, P.body);                                                                                                                 // the cab: doors,
  greenhouse(b, 0.38, 1.08, 1.5, 0.64, 0.8, { f: 30, r: 12, s: 12 }, P.glass, P.body, [], 0.1);                                                           // glass between posts, under a bowed roof
  b(-1.42, 0, 0.8, 0.83, 1.0, 0.76, P.trim);                                                                                                                // the bed liner
  for (const s of [-1, 1]) b(-1.42, s * 0.83, 0.8, 1.18, 1.03, 0.03, P.body);                                                                               // bed sides
  b(-2.42, 0, 0.8, 1.18, 0.03, 0.86, P.body); b(-0.36, 0, 0.8, 1.18, 0.03, 0.86, P.body);                                                                   // tailgate, the bed's front wall
  return out;
}

/** A PANEL VAN, 4.9 × 1.8 × 1.95, nose toward local +x, its back doors open (scene/gwotKit.ts van, with curves): everything but its four lamps */
export function vanShape(F: CarFrame, P: CarPaints): Box[] {
  const out: Box[] = [], b = putter(F, out);
  wheels(b, [-1.45, 1.5], 0.93, 0.37, 0.26, P.tyre, P.hub, P.body);
  { const t = b(0, 0, 0.3, 0.86, 2.4, 0.9, P.body); cutTo(t, [...corner(t.h, 0.22, 1, 1, 3), ...corner(t.h, 0.22, 1, -1, 3), ...corner(t.h, 0.08, -1, 1, 1), ...corner(t.h, 0.08, -1, -1, 1)]); }   // the body, sill to waistline
  { const f = b(2.37, 0, 0.36, 0.58, 0.11, 0.9, P.trim); cutTo(f, [...corner(f.h, 0.24, 1, 1, 3), ...corner(f.h, 0.24, 1, -1, 3)]); }                       // the bumper wraps the nose
  b(-2.43, 0, 0.3, 0.44, 0.05, 0.9, P.trim);                                                                                                                // the rear step
  { const n = b(1.95, 0, 0.86, 1.06, 0.35, 0.86, P.body); arcTo(n, 0.2, 1, [fall(n.h, 1, 0.18, 0.15)]); }                                                     // a short crowned bonnet
  b(1.3, 0, 0.86, 1.1, 0.4, 0.9, P.body);                                                                                                                   // the cab: doors,
  greenhouse(b, 1.3, 1.1, 1.62, 0.36, 0.84, { f: 28, r: 0, s: 4 }, P.glass, P.body, [-0.3]);                                                                // glass between posts,
  { const w = b(1.1325, 0, 1.62, 1.8, 0.2525, 0.86, P.body); cutTo(w, [lean(w.h, 1, 0, 28)]); }                                                             // the brow over it, on the windscreen's line
  b(-0.5, 0, 0.86, 1.8, 1.4, 0.9, P.body);                                                                                                                  // the load box
  { const r = b(-0.555, 0, 1.8, 1.95, 1.845, 0.9, P.body); arcTo(r, 0.15, 1, [lean(r.h, 1, 0, 40)]); }                                                      // ONE bowed roof, windscreen to back doors
  for (const s of [-1, 1]) b(-2.15, s * 0.87, 0.86, 1.8, 0.25, 0.03, P.body);                                                                               // the open bay: its sides, the dark inside, its floor
  b(-1.89, 0, 0.86, 1.8, 0.02, 0.84, P.trim); b(-2.15, 0, 0.86, 0.88, 0.25, 0.84, P.trim);
  for (const s of [-1, 1]) b(-2.77, s * 1.1, 0.5, 1.82, 0.42, 0.025, P.body, {}, s * 0.5);                                                                  // the two back doors, swung out
  return out;
}

/** A CAR, 4.4 × 1.72, nose toward local +x (scene/officeGen.ts car, with curves): everything but its four lamps */
export function carShape(F: CarFrame, P: CarPaints): Box[] {
  const out: Box[] = [], b = putter(F, out);
  wheels(b, [1.35, -1.4], 0.92, 0.3, 0.24, P.tyre, P.hub, P.body);
  { const t = b(0, 0, 0.28, 0.96, 2.2, 0.86, P.body);                                                                                                       // lower body: round corners, a bonnet and a boot that fall away
    cutTo(t, [...corner(t.h, 0.24, 1, 1, 2), ...corner(t.h, 0.24, 1, -1, 2), ...corner(t.h, 0.16, -1, 1, 1), ...corner(t.h, 0.16, -1, -1, 1), fall(t.h, 1, 0.5, 0.16), fall(t.h, -1, 0.3, 0.1)]); }
  greenhouse(b, -0.25, 0.93, 1.47, 1.25, 0.8, { f: 38, r: 42, s: 12 }, P.glass, P.body, [0.05], 0.08);                                                     // the greenhouse: four panes leaning in, six posts, a bowed roof
  return out;
}

export interface BikePaints { tyre: CarMat; iron: CarMat; tank: CarMat; seat: CarMat; galv: CarMat }
/** A SMALL MOTORBIKE on its stand, nose toward local +x (scene/gwotKit.ts motorbike, with curves) */
export function bikeShape(F: CarFrame, P: BikePaints): Box[] {
  const out: Box[] = [], b = putter(F, out);
  for (const lx of [0.66, -0.62]) { roundTo(b(lx, 0, 0, 0.6, 0.3, 0.05, P.tyre), 0.3, 3); arcTo(b(lx, 0, 0.3, 0.625, 0.325, 0.04, of(P.tank, 0.8)), 0.325, 3); }   // wheels, a mudguard over each
  b(0.02, 0, 0.26, 0.56, 0.2, 0.12, P.iron);                                                                                                                // the engine
  roundTo(b(0.18, 0, 0.58, 0.82, 0.24, 0.12, P.tank), 0.12, 1);                                                                                             // a round tank
  arcTo(b(-0.36, 0, 0.62, 0.8, 0.34, 0.11, P.seat), 0.07, 1);                                                                                               // a bowed seat
  roundTo(b(0.6, 0, 0.6, 1.02, 0.03, 0.03, P.galv), 0.03, 2); roundTo(b(0.56, 0, 1.0, 1.04, 0.02, 0.32, P.iron), 0.02, 3);                                  // the fork, the bars
  roundTo(b(0.74, 0, 0.79, 0.93, 0.05, 0.07, P.iron), 0.07, 1);                                                                                             // the lamp
  b(-0.84, 0, 0.66, 0.7, 0.16, 0.13, P.iron); roundTo(b(-0.4, 0.16, 0.285, 0.355, 0.42, 0.035, P.iron), 0.035, 1);                                          // the rack, the exhaust
  return out;
}

// ---- THE TRIAL'S OWN HALF: knowing a vehicle in a loaded level by the stack its generator writes ----
type SigBox = { c: Vec3; h: Vec3 };
/** the stack the look's kit writes for one of its pieces, stood at the origin */
function record(build: (G: ReturnType<typeof gwotKit>) => void): SigBox[] {
  const got: SigBox[] = [];
  const host: GwotHost = { add: b => { got.push({ c: b.c, h: b.h }); return got.length - 1; }, prop: () => { throw new Error('carTry: a vehicle with a prop'); }, wallX: () => { }, wallZ: () => { }, lights: [], switchables: [], marks: false, texel: 0.025 };
  build(gwotKit(host)); return got;
}
/** the car of the office's kit (scene/officeGen.ts car, scene/levelKit.ts): body, greenhouse, roof, four wheels, a head and a tail lamp a side */
const CAR_SIG = (): SigBox[] => [{ c: [0, 0.62, 0], h: [2.2, 0.34, 0.86] }, { c: [-0.25, 1.2, 0], h: [1.25, 0.27, 0.8] }, { c: [-0.25, 1.49, 0], h: [1.15, 0.03, 0.76] },
  ...([[1.35, 0.8], [1.35, -0.8], [-1.4, 0.8], [-1.4, -0.8]] as const).map(([x, z]): SigBox => ({ c: [x, 0.3, z], h: [0.32, 0.3, 0.12] })),
  ...[0.62, -0.62].flatMap((z): SigBox[] => [{ c: [2.2, 0.7, z], h: [0.02, 0.07, 0.14] }, { c: [-2.2, 0.75, z], h: [0.02, 0.06, 0.16] }])];
interface Kind { name: string; sig: SigBox[]; anchor: number; /** which made box is the body the lamps sit on */ body: number; lamps: number[]; shape: (F: CarFrame, at: (i: number) => Box) => Box[] }
const matOf = (b: Box): CarMat => ({ albedo: b.albedo, rough: b.rough, ...(b.metal ? { metal: true } : {}) });
function kinds(): Kind[] {
  return [
    { name: 'pickup', sig: record(G => G.pickup(0, 0, 0)), anchor: 4, body: 12, lamps: [16, 17, 18, 19], shape: (F, at) => pickupShape(F, { body: MADE.body(matOf(at(4))), tyre: MADE.tyre(matOf(at(0))), glass: MADE.glass(matOf(at(9))), trim: MADE.trim(matOf(at(5))), hub: MADE.hub(HUB()) }) },
    { name: 'van', sig: record(G => G.van(0, 0, 0)), anchor: 4, body: 12, lamps: [19, 20, 21, 22], shape: (F, at) => vanShape(F, { body: MADE.body(matOf(at(4))), tyre: MADE.tyre(matOf(at(0))), glass: MADE.glass(matOf(at(9))), trim: MADE.trim(matOf(at(5))), hub: MADE.hub(HUB()) }) },
    { name: 'motorbike', sig: record(G => G.motorbike(0, 0, 0)), anchor: 2, body: 0, lamps: [], shape: (F, at) => bikeShape(F, { tyre: MADE.tyre(matOf(at(0))), iron: matOf(at(2)), tank: MADE.body(matOf(at(3))), seat: matOf(at(4)), galv: matOf(at(5)) }) },
    { name: 'car', sig: CAR_SIG(), anchor: 0, body: 12, lamps: [7, 8, 9, 10], shape: (F, at) => carShape(F, { body: MADE.body(matOf(at(0))), tyre: MADE.tyre(matOf(at(3))), glass: MADE.glass(matOf(at(1))), trim: MADE.trim(matOf(at(3))), hub: MADE.hub(HUB()) }) },
  ];
}

/** give every vehicle of the level its curves (see the head). Returns what it found, by kind. */
export function dressCars(level: { boxes: Box[] }): Record<string, number> {
  const B = level.boxes, taken = new Set<number>(), found: Record<string, number> = {}, add: Box[] = [];
  const near = (a: ArrayLike<number>, b: ArrayLike<number>, e: number) => Math.abs(a[0] - b[0]) < e && Math.abs(a[1] - b[1]) < e && Math.abs(a[2] - b[2]) < e;
  for (const K of kinds()) {
    const A = K.sig[K.anchor];
    for (let ai = 0; ai < B.length; ai++) {
      const a = B[ai]; if (taken.has(ai) || a.rot || a.cut || !near(a.h, A.h, 0.003) || Math.abs(a.c[1] - A.c[1]) > 0.003) continue;
      for (const yaw of [a.yaw, a.yaw + Math.PI]) {
        const cs = Math.cos(yaw), sn = Math.sin(yaw), x = a.c[0] - (cs * A.c[0] + sn * A.c[2]), z = a.c[2] - (-sn * A.c[0] + cs * A.c[2]);
        const idx: number[] = [], mine = new Set<number>();
        for (const s of K.sig) {
          const want: Vec3 = [x + cs * s.c[0] + sn * s.c[2], s.c[1], z - sn * s.c[0] + cs * s.c[2]];
          const bi = B.findIndex((b, i) => !taken.has(i) && !mine.has(i) && near(b.c, want, 0.012) && near(b.h, s.h, 0.004));
          if (bi < 0) break; idx.push(bi); mine.add(bi);
        }
        if (idx.length < K.sig.length) continue;
        for (const i of idx) taken.add(i);
        const made = K.shape({ x, z, yaw }, i => B[idx[i]]), old = idx.filter((_, i) => !K.lamps.includes(i));
        for (const li of K.lamps) { const l = B[idx[li]], s = K.sig[li], t = K.sig[K.anchor]; lens(l, Math.min(l.h[1], l.h[2]) * 0.8, made[K.body], s.c[0] - t.c[0], s.c[2] - t.c[2]); }   // (the body's box stands where the stack's anchor stood)
        // a made box takes the place of a box of the stack while there is one (indices hold: nothing that names a box by number moves); the rest are added
        made.forEach((m, i) => { if (i < old.length) { const o = B[old[i]]; m.owner = o.owner; m.cutHeight = o.cutHeight; delete o.marks; delete o.cut; delete o.cutRound; delete o.cutArc; delete o.cutOld; delete o.metal; Object.assign(o, m); } else add.push(m); });
        for (let i = made.length; i < old.length; i++) { const o = B[old[i]]; o.flags |= BoxFlag.NoPrimary | BoxFlag.NoShadow; o.c = [x, 0.5, z]; o.h = [0.01, 0.01, 0.01]; delete o.marks; }
        found[K.name] = (found[K.name] ?? 0) + 1;
        break;
      }
    }
  }
  for (const b of add) B.push(b);
  return found;
}
