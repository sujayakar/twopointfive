// THE TERRACE'S GARDEN CORNER, FINISHED: the rooftop's building with one corner of its terrace in fine geometry, east of the upper storey and south of the pool. What people walk on and bump into, the section,
// doors, lamps, trees and cloth are the building's own. What is finished is a skin over the boxes that were there (a little lower), or stands in their place; boxes that are no longer seen stay as ghosts.
import { type V3, type Rot, ID, rotY, rotX, rotZ, compose, apply, add, rng } from '../math';
import { F, type Piece } from './world';
import { M, C } from './materials';
import { makeBuilding, type Building } from './building';
import type { Meshes } from './meshes';
import { buildDag, type Dag } from '../mesh/dag';
import type { Mesh } from '../mesh/mesh';
import { pebble } from '../mesh/shapes';
import { paver, board, brick, coping, panel, post, tubeRail, cushion, pouf, tabletop, grate, downpipe, lighten } from '../mesh/made';
import { Store, packMesh, packObject, type Child, type PackedObject, type Packed } from '../mesh/pack';
import { LOST_ROWS } from '../mesh/lostsize';

const FL = 3.4, TOP = FL + 3.2 + 0.62;
/** what is measured by itself of the light that levels lose */
const CLASS = { paver: 0, board: 1, brick: 2, stone: 3, panel: 4, mulch: 5, metal: 6, cloth: 7 };
/** skins are met by rays no wider than this; wider ones meet what they lie on */
const SKIN = 0.025;
interface Placed { row: number; at: V3; rot: Rot; half: V3; mat: number; tint: V3; cls: number }
export interface Corner extends Meshes { placed: Placed[]; /** what each class sends back in the mean, beside its material: the boxes that stand in for it are that color */ mean: V3[] }

/** everything that takes long. Plain data, so that a worker may do it */
export function buildCorner(say: (what: string) => void = () => {}): Corner {
  const ms: Record<string, number> = {}, timed = <T>(k: string, f: () => T): T => { const t = performance.now(), v = f(); ms[k] = (ms[k] ?? 0) + performance.now() - t; return v; };
  const s = new Store(0), list: Meshes['list'] = [], made = (name: string, n: number, f: (i: number) => Mesh): Packed[] => { say(name); return Array.from({ length: n }, (_, i) => { const m = timed('shapes', () => f(i)), d: Dag = timed('levels', () => buildDag(m)), packed = timed('pack', () => packMesh(s, d)); { const c: V3 = [0, 0, 0], k = m.col.length / 4; for (let v = 0; v < k; v++) { const open = 0.72 + 0.28 * m.col[v * 4 + 3]! / 255; for (let a = 0; a < 3; a++) c[a]! += (m.col[v * 4 + a]! / 255) ** 2 * open / k; } hue.set(packed, c); } list.push({ name: `${name} ${i}`, packed }); return packed; }); };
  const hue = new Map<Packed, V3>(), sums: { c: V3; n: number }[] = [], placed: Placed[] = [], objects: PackedObject[] = [], q = rng(91), pick = <T>(l: T[]): T => l[Math.floor(q() * l.length)]!, shade = (k: number): V3 => { const v = 1 - k * q(); return [v, v, v]; };
  /** children given in the world, as one object */
  const object = (cls: number, kids: { mesh: Packed; at: V3; rot?: Rot; scale?: number; tint?: V3 }[], mat: number, tint: V3 = [1, 1, 1], skin = 0) => {
    if (!kids.length) return; { const t = sums[cls] ??= { c: [0, 0, 0], n: 0 }; for (const k of kids) { const h = hue.get(k.mesh)!, q2 = k.tint ?? [1, 1, 1]; for (let a = 0; a < 3; a++) t.c[a]! += h[a]! * q2[a]! ** 2 * tint[a]! ** 2; t.n++; } } let c: V3 = [0, 0, 0]; for (const k of kids) c = add(c, k.at); c = [c[0] / kids.length, c[1] / kids.length, c[2] / kids.length];
    const o = packObject(s, [{ error: 0, children: kids.map((k): Child => ({ mesh: k.mesh, at: [k.at[0] - c[0], k.at[1] - c[1], k.at[2] - c[2]], rot: k.rot ?? ID, scale: k.scale ?? 1, tint: k.tint ?? [1, 1, 1] })) }], cls, skin);
    objects.push(o); placed.push({ row: o.row, at: c, rot: ID, half: [0, 1, 2].map(k => Math.max(Math.abs(o.mn[k]!), Math.abs(o.mx[k]!)) + 1e-4) as V3, mat, tint, cls });
  };
  const quarter = [0, 1, 2, 3].map(k => rotY(k * Math.PI / 2)), alongZ: Rot = [[0, 0, 1], [0, 1, 0], [-1, 0, 0]], faceX: Rot = [[0, 0, -1], [0, 1, 0], [1, 0, 0]];

  // ---- the ground: pavers, the drain, the deck
  const pavers = made('paver', 5, i => paver(20 + i * 3)); for (let gx = 0; gx < 4; gx++) for (let gz = 0; gz < 2; gz++) { const kids = []; for (let i = gx * 4; i < Math.min(13, gx * 4 + 4); i++) for (let j = gz * 5; j < gz * 5 + 5; j++) kids.push({ mesh: pick(pavers), at: [4.19 + 0.3 + i * 0.6, FL + (q() - 0.5) * 0.0016, 2.05 + 0.3 + j * 0.6] as V3, rot: compose(pick(quarter), compose(rotX((q() - 0.5) * 0.004), ID)), tint: shade(0.10) }); object(CLASS.paver, kids, M.plaster!, [0.98, 0.98, 0.975], SKIN); }
  const grates = made('grate', 1, () => grate()); object(CLASS.metal, Array.from({ length: 7 }, (_, i) => ({ mesh: grates[0]!, at: [4.75 + i * 1.0, FL + 0.002, 1.972] as V3 })), M.steel!, [0.55, 0.55, 0.56], SKIN);
  const boards = made('board', 4, i => board(40 + i * 5)), halves = made('half board', 2, i => board(60 + i * 5, 0.96)), P = 3.5 / 37; for (let g = 0; g < 37; g += 6) { const kids = []; for (let r = g; r < Math.min(37, g + 6); r++) { let x = 4.32; const z = -1.6 + (r + 0.5) * P; if (r % 2) { kids.push({ mesh: pick(halves), at: [x + 0.48, FL + 0.05, z] as V3, rot: pick([quarter[0]!, quarter[2]!]), tint: shade(0.16) }); x += 0.96; } for (; x + 1.9 < 12.02; x += 1.92) kids.push({ mesh: pick(boards), at: [x + 0.96, FL + 0.05 + (q() - 0.5) * 0.001, z] as V3, rot: pick([quarter[0]!, quarter[2]!]), tint: shade(0.16) }); if (12.0 - x > 0.9) kids.push({ mesh: pick(halves), at: [x + 0.48, FL + 0.05, z] as V3, tint: shade(0.16) }); } object(CLASS.board, kids, M.wood!, [1, 0.98, 0.94], SKIN); }
  // ---- the wall: painted brick over the render, its mortar a little back; coping; a rain pipe
  const bricks = made('brick', 8, i => brick(70 + i * 7, 0.105, i === 7)), turned = compose(faceX, rotZ(Math.PI)), bats = made('half brick', 2, i => brick(120 + i * 7, 0.05)), holes: [number, number, number, number][] = [[0.50, 1.75, FL, FL + 2.40], [3.10, 6.90, FL + 0.02, FL + 2.70]], Z0 = -1.6, Z1 = 7.9, courses = Math.floor(3.16 / 0.075);
  for (let g = 0; g < courses; g += 7) { const kids = []; for (let c = g; c < Math.min(courses, g + 7); c++) { const y = FL + 0.02 + (c + 0.5) * 0.075, free: [number, number][] = [[Z0, Z1]]; for (const [a, b, y0, y1] of holes) if (y > y0 - 0.03 && y < y1 + 0.03) for (let i = free.length - 1; i >= 0; i--) { const [u, v] = free[i]!; if (b <= u || a >= v) continue; free.splice(i, 1, ...([[u, a], [b, v]] as [number, number][]).filter(([m, n]) => n - m > 0.1)); }
      for (const [u, v] of free) { let z = u; if (c % 2 && v - u > 0.2) { kids.push({ mesh: pick(bats), at: [4.092, y, z + 0.055] as V3, rot: faceX, tint: shade(0.05) }); z += 0.11; } for (; z + 0.215 <= v; z += 0.22) kids.push({ mesh: bricks[q() < 0.035 ? 7 : Math.floor(q() * 7)]!, at: [4.092 + (q() - 0.5) * 0.0024, y + (q() - 0.5) * 0.001, z + 0.11] as V3, rot: q() < 0.5 ? faceX : turned, tint: shade(0.06) }); if (v - z > 0.10) kids.push({ mesh: pick(bats), at: [4.092, y, z + 0.055] as V3, rot: faceX, tint: shade(0.05) }); } } object(CLASS.brick, kids, M.plaster!, [1, 1, 1], SKIN); }
  const tops = made('coping', 3, i => coping(140 + i * 3)); object(CLASS.stone, Array.from({ length: 11 }, (_, i) => ({ mesh: pick(tops), at: [4.1, TOP - 0.004, -1.55 + 0.45 + i * 0.9] as V3, rot: alongZ, tint: shade(0.06) })), M.plaster!, [0.97, 0.965, 0.95]);
  const pipes = made('rain pipe', 1, () => downpipe()); object(CLASS.metal, [0, 1, 2].map(i => ({ mesh: pipes[0]!, at: [4.09 + 0.115, FL + 0.55 + i, 7.78] as V3, rot: faceX })), M.steel!, [0.9, 0.9, 0.9]);
  // ---- planters: sides cast against boards, a rim, stones over the soil
  const stones = made('mulch', 4, i => lighten(pebble(160 + i * 9, 4, 0.5), 0.006)); for (const [x0, z0, x1, z1, h] of PLANTERS) {
    const lx = (x1 - x0), lz = (z1 - z0) - 0.14, nx = Math.round(lx / 0.62), nz = Math.round(lz / 0.62), sx = made(`planter ${h} side`, 1, () => panel(180 + nx, h, lx / nx))[0]!, sz = made(`planter ${h} end`, 1, () => panel(190 + nz, h, lz / nz))[0]!, rx = made('rim', 1, () => coping(200 + nx, lx / nx, 0.052, 0.034))[0]!, rz = made('rim', 1, () => coping(210 + nz, lz / nz, 0.052, 0.034))[0]!, kids = [], rim = [];
    for (let i = 0; i < nx; i++) for (const z of [z0 + 0.035, z1 - 0.035]) { const x = x0 + (i + 0.5) * lx / nx; kids.push({ mesh: sx, at: [x, FL, z] as V3, rot: pick([quarter[0]!, quarter[2]!]), tint: shade(0.04) }); rim.push({ mesh: rx, at: [x, FL + h - 0.002, z] as V3, rot: pick([quarter[0]!, quarter[2]!]), tint: shade(0.05) }); }
    for (let i = 0; i < nz; i++) for (const x of [x0 + 0.035, x1 - 0.035]) { const z = z0 + 0.07 + (i + 0.5) * lz / nz; kids.push({ mesh: sz, at: [x, FL, z] as V3, rot: alongZ, tint: shade(0.04) }); rim.push({ mesh: rz, at: [x, FL + h - 0.003, z] as V3, rot: alongZ, tint: shade(0.05) }); }
    object(CLASS.panel, kids, M.plaster!); object(CLASS.stone, rim, M.plaster!, [0.97, 0.965, 0.955]);
    const n = Math.ceil((x1 - x0) / 1.5); for (let g = 0; g < n; g++) { const a = x0 + 0.09 + (x1 - x0 - 0.18) * g / n, b = x0 + 0.09 + (x1 - x0 - 0.18) * (g + 1) / n, mulch = []; for (let i = 0; i < (b - a) * (z1 - z0 - 0.18) * 300; i++) { const r = 0.045 + 0.05 * q() * q(); mulch.push({ mesh: pick(stones), at: [a + (b - a) * q(), FL + h - 0.06 + r * 0.16, z0 + 0.1 + (z1 - z0 - 0.2) * q()] as V3, rot: compose(rotY(q() * 6.28), rotX((q() - 0.5) * 0.7)), scale: r, tint: [0.62 + 0.25 * q(), 0.52 + 0.2 * q(), 0.44 + 0.16 * q()] as V3 }); } object(CLASS.mulch, mulch, M.gravel!, [0.8, 0.75, 0.7], SKIN); }
  }
  // ---- the rail: posts with feet and clamps, a tube with collars
  const posts = made('post', 1, () => post()), bar = made('tube', 1, () => tubeRail(1.6)), bar2 = made('tube', 1, () => tubeRail(1.56)); const px: { mesh: Packed; at: V3; rot?: Rot }[] = [], tx: typeof px = []; for (let i = 4; i <= 10; i++) px.push({ mesh: posts[0]!, at: [12.0, FL + 0.5, -8 + 1.6 * i], rot: alongZ }); for (let i = 1; i <= 5; i++) px.push({ mesh: posts[0]!, at: [12.0 - 1.56 * i, FL + 0.5, 8.0] }); for (let i = 4; i < 10; i++) tx.push({ mesh: bar[0]!, at: [12.0, FL + 1.0, -8 + 1.6 * (i + 0.5)], rot: alongZ }); for (let i = 0; i < 5; i++) tx.push({ mesh: bar2[0]!, at: [12.0 - 1.56 * (i + 0.5), FL + 1.0, 8.0] }); object(CLASS.metal, px, M.steel!); object(CLASS.metal, tx, M.steel!);
  // ---- loungers, seats, the table
  const seat = made('seat', 1, () => cushion(230, 0.34, 0.62, 0.10, 7))[0]!, back = made('back', 1, () => cushion(231, 0.34, 0.34, 0.10, 4))[0]!, drum = made('pouf', 1, () => pouf(0.2, 0.44))[0]!, disc = made('table top', 1, () => tabletop(0.55, 0.05))[0]!;
  for (const [x, z, c, yaw] of LOUNGERS) { const rt = rotY(yaw), at = (l: V3): V3 => add([x, FL + 0.05, z], apply(rt, l)); object(CLASS.cloth, [{ mesh: seat, at: at([0, 0.24, 0]), rot: rt }, { mesh: back, at: at([0, 0.44, -0.83]), rot: compose(rt, rotX(0.75)) }], M.fabric!, c); }
  for (let i = 0; i < 4; i++) object(CLASS.cloth, [{ mesh: drum, at: [10.6 + 0.85 * Math.cos(i * 1.57 + 0.5), FL + 0.28, 0.2 + 0.85 * Math.sin(i * 1.57 + 0.5)], rot: rotY(q() * 6) }], M.fabric!, (i % 2 ? C.white : C.grey) as V3); object(CLASS.metal, [{ mesh: disc, at: [10.6, FL + 0.46, 0.2] }], M.paint!, C.red as V3);

  const lost = s.take(LOST_ROWS, 'lost'); s.f.fill(1, lost * 4, (lost + LOST_ROWS) * 4);
  return { data: s.data, list, squares: [], heroes: [], objects, weigh: s.weigh, ms, depth: Math.max(...list.map(l => l.packed.depth)), lost, classes: [], placed, mean: sums.map(t => t.c.map(v => Math.sqrt(v / t.n)) as V3) };
}
const PLANTERS: [number, number, number, number, number][] = [[4.6, 2.4, 11.8, 4.2, 0.55], [9.9, 4.9, 11.8, 7.7, 0.5], [4.6, 6.4, 7.6, 7.7, 0.5]];
const LOUNGERS: [number, number, V3, number][] = [[6.0, 0.2, C.white as V3, 0.25], [7.4, 0.1, C.orange as V3, 0.1], [8.8, 0.15, C.white as V3, -0.05]];
let ready: Corner | null = null;
/** made elsewhere, for the next scene */
export const giveCorner = (c: Corner) => { ready = c; };

export function makeCorner(): Building {
  const b = makeBuilding(), w = b.world, c = ready ?? buildCorner(), near = (x: number, y: number) => Math.abs(x - y) < 0.06, cut = { flags: F.CUT };
  // ---- what is no longer seen
  const ghost = (f: (p: Piece, lo: V3, hi: V3) => boolean, mat?: number, tint?: V3) => { let n = 0; for (const p of w.pieces) { if (p.ghost || p.flags & F.MESH || p.mat === M.glass) continue; const e = [0, 1, 2].map(k => Math.abs(p.rot[0][k]!) * p.half[0] + Math.abs(p.rot[1][k]!) * p.half[1] + Math.abs(p.rot[2][k]!) * p.half[2]); if (f(p, p.at.map((v, k) => v - e[k]!) as V3, p.at.map((v, k) => v + e[k]!) as V3)) { p.ghost = true; n++; if (mat !== undefined) { p.mat = mat; p.tint = tint!; p.marks = undefined; } } } return n; };
  const took: Record<string, number> = {};
  took.floor = ghost((p, lo, hi) => p.mat === M.stone && near(lo[0], 4.09) && near(hi[0], 12.09) && near(hi[1], FL) && hi[2] - lo[2] > 15, M.plaster!, c.mean[CLASS.paver]!); took.deck = ghost((p, lo, hi) => p.mat === M.wood && near(lo[0], 4.3) && near(hi[0], 12.0) && near(lo[2], -1.6) && near(hi[2], 1.9), M.wood!, c.mean[CLASS.board]!);
  for (const [x0, z0, x1, z1, h] of PLANTERS) took.planters = (took.planters ?? 0) + ghost((p, lo, hi) => p.mat === M.paint && lo[0] > x0 - 0.05 && hi[0] < x1 + 0.05 && lo[2] > z0 - 0.05 && hi[2] < z1 + 0.05 && near(lo[1], FL) && near(hi[1], FL + h), M.plaster!, c.mean[CLASS.panel]!);
  took.rail = ghost((p, lo, hi) => p.mat === M.steel && lo[1] > FL - 0.05 && hi[1] < FL + 1.06 && ((near((lo[0] + hi[0]) / 2, 12.0) && hi[0] - lo[0] < 0.1 && (hi[2] - lo[2] > 15 || lo[2] > -1.75)) || (near((lo[2] + hi[2]) / 2, 8.0) && hi[2] - lo[2] < 0.1 && lo[0] > 4.0)));
  took.cloth = ghost((p, lo, hi) => p.mat === M.fabric && lo[0] > 5 && hi[0] < 12 && lo[1] > FL && hi[1] < FL + 1.1 && lo[2] > -1.6 && hi[2] < 1.9); took.table = ghost((p, lo, hi) => p.mat === M.paint && !!p.quad && near(p.at[0], 10.6) && near(p.at[2], 0.2) && hi[1] - lo[1] < 0.08 && lo[1] > FL);
  if (took.floor !== 1 || took.deck !== 1 || took.planters !== 12 || !took.rail || took.cloth !== 10 || took.table !== 1) console.warn('corner: not all was found', JSON.stringify(took));
  // ---- what lies under the skins
  const core = (a: V3, d: V3, mat: number, tint: V3) => w.box(a, d, mat, { tint, ...cut });
  core([4.09, FL - 0.03, -8.09], [12.09, FL + 0.0004, -1.6], M.stone!, [1, 1, 1]); core([4.09, FL - 0.0299, -1.6], [12.09, FL + 0.026, 1.9], M.wood!, [0.82, 0.76, 0.7]); core([4.09, FL - 0.0298, 1.9], [12.09, FL - 0.016, 8.09], M.concrete!, [0.80, 0.79, 0.77]);
  core([4.09, FL - 0.02, 2.02], [4.187, FL - 0.0005, 8.089], M.stone!, [1, 1, 1]); core([11.993, FL - 0.02, 2.02], [12.089, FL - 0.0005, 8.089], M.stone!, [1, 1, 1]);
  core([4.089, FL, -1.6], [4.1025, FL + 3.19, 0.5], M.plaster!, [0.86, 0.85, 0.83]); core([4.089, FL + 2.4, 0.5], [4.1024, FL + 3.189, 1.75], M.plaster!, [0.86, 0.85, 0.83]); core([4.089, FL, 1.75], [4.1023, FL + 3.188, 3.1], M.plaster!, [0.86, 0.85, 0.83]); core([4.089, FL + 2.7, 3.1], [4.1022, FL + 3.187, 6.9], M.plaster!, [0.86, 0.85, 0.83]); core([4.089, FL, 6.9], [4.1021, FL + 3.186, 7.9], M.plaster!, [0.86, 0.85, 0.83]);
  w.add({ at: [12.0, FL + 1.0, -4.8], half: [0.028, 0.02, 3.2], mat: M.steel!, ...cut });      // (the rail goes on as it was by the pool)
  for (const p of c.placed) w.pieces.push({ at: p.at, rot: p.rot, half: p.half, mat: p.mat, flags: F.MESH | F.TRIANGLES | F.CUT, quad: [1, 0, 0, 0], tint: p.tint, mesh: p.row });
  // ---- where their light is measured: a box for each class
  const of = (cls: number, most = 1): { lo: V3; hi: V3; levels: number } => { const l = c.placed.filter(p => p.cls === cls).slice(0, most), k = 0.8; return { lo: [0, 1, 2].map(a => Math.min(...l.map(p => p.at[a]! - p.half[a]! * k))) as V3, hi: [0, 1, 2].map(a => Math.max(...l.map(p => p.at[a]! + p.half[a]! * k))) as V3, levels: 1 }; };
  c.classes = Object.values(CLASS).map(k => of(k, k === CLASS.brick ? 3 : 2)); w.meshes = c; return b;
}
