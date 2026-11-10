// A STAGE FOR MESHES: a yard of pebbles, a broken column with its fallen drum, two walls to send light back. Small on purpose: everything in it but the walls is triangles.
import { type V3, type Rot, ID, rotY, rotZ, compose, rng } from '../math';
import { World, F } from './world';
import { M } from './materials';
import type { Place } from './place';
import type { Building } from './building';
import { settle, freeEdges } from './faces';
import { Bodies } from './rigid';
import { Solid } from './solid';
import { Player } from './player';
import { buildDag, type Dag } from '../mesh/dag';
import { pebble, column } from '../mesh/shapes';
import { Store, packMesh, packObject, type PackedObject, type Packed } from '../mesh/pack';
import { stones, relief, children, SIDE } from '../mesh/floor';
import { LOST_ROWS } from '../mesh/lostsize';

/** how fine it is made (set before the scene is) */
export const FINE = { /** triangles of a pebble: 20 · 4^n */ pebble: 5, shapes: 6, /** how often the column's net is split in four */ column: 2, /** squares a side */ yard: 12, kinds: 4, /** the relief's grid */ relief: 256 };
export interface Meshes { data: Float32Array<ArrayBuffer>; list: { name: string; packed: Packed }[]; squares: PackedObject[]; heroes: PackedObject[]; objects: PackedObject[]; weigh: Record<string, number>; ms: Record<string, number>; depth: number; /** the row of the table of what levels lost of their light */ lost: number; /** where each class is looked at to measure that: a box of the world, and how many levels its objects have */ classes: { lo: V3; hi: V3; levels: number }[] }

/** everything that takes long: shapes, levels, packing. Plain data, so a worker may do it */
export function buildMeshes(say: (what: string) => void = () => {}): Meshes {
  const ms: Record<string, number> = {}, timed = <T>(k: string, f: () => T): T => { const t = performance.now(), v = f(); ms[k] = (ms[k] ?? 0) + performance.now() - t; return v; };
  const s = new Store(0), list: Meshes['list'] = [], keep = (name: string, d: Dag) => { const packed = timed('pack', () => packMesh(s, d)); list.push({ name, packed }); return packed; };
  // ---- pebbles, and the squares of floor made of them
  const dags = Array.from({ length: FINE.shapes }, (_, i) => { say(`Pebble ${i + 1} of ${FINE.shapes}`); const m = timed('shapes', () => pebble(11 + i * 7, FINE.pebble, [0.85, 0.95, 0.6, 1, 0.75, 0.35, 0.9, 0.5][i % 8]!)); return timed('levels', () => buildDag(m)); }), shapes = dags.map((d, i) => keep('pebble ' + i, d));
  const squares = Array.from({ length: FINE.kinds }, (_, k) => {
    say(`Floor ${k + 1} of ${FINE.kinds}`); const st = stones(k, FINE.shapes), m = timed('relief', () => relief(st, dags, FINE.relief, -0.02)), d = timed('levels', () => buildDag(m, 48)), r = keep('relief ' + k, d);
    return packObject(s, [{ error: 0, children: children(st, shapes) }, { error: SIDE / FINE.relief, children: [{ mesh: r, at: [0, 0, 0], rot: ID, scale: 1, tint: [1, 1, 1] }] }], 0);
  });
  // ---- the hero
  const heroes = [false, true].map(drum => { say(drum ? 'The fallen drum' : 'The column'); const m = timed('shapes', () => column(drum, 0.011, FINE.column)), d = timed('levels', () => buildDag(m)), p = keep(drum ? 'drum' : 'column', d); return packObject(s, [{ error: 0, children: [{ mesh: p, at: [0, 0, 0], rot: ID, scale: 1, tint: [1, 1, 1] }] }], drum ? 2 : 1); });
  const lost = s.take(LOST_ROWS, 'lost'); s.f.fill(1, lost * 4, (lost + LOST_ROWS) * 4);
  return { data: s.data, list, squares, heroes, objects: [...squares, ...heroes], weigh: s.weigh, ms, depth: Math.max(...list.map(l => l.packed.depth)), lost, classes: [] };
}
let ready: Meshes | null = null;
/** meshes made elsewhere, for the next scene */
export const give = (m: Meshes) => { ready = m; };
export function makeMeshes(): Building {
  const w = new World(), made = ready ?? buildMeshes(), { squares, heroes } = made;
  const place = (o: PackedObject, at: V3, rot: Rot, scale: number, mat: number, tint: V3) => { const half = [0, 1, 2].map(k => Math.max(Math.abs(o.mn[k]!), Math.abs(o.mx[k]!)) * scale + 1e-4) as V3; w.pieces.push({ at, rot, half, mat, flags: F.MESH | F.TRIANGLES, quad: [scale, 0, 0, 0], tint, mesh: o.row }); };
  const Y = FINE.yard, q = rng(5); for (let z = 0; z < Y; z++) for (let x = 0; x < Y; x++) place(squares[Math.floor(q() * FINE.kinds)]!, [(x - (Y - 1) / 2) * SIDE, 0, (z - (Y - 1) / 2) * SIDE], ID, 1, M.gravel!, [1, 1, 1]);
  place(heroes[0]!, [0, -0.005, 0], rotY(0.35), 1, M.concrete!, [1, 1, 1]); place(heroes[1]!, [0.95, 0.262, 0.75], compose(rotY(-0.72), rotZ(0.03)), 1, M.concrete!, [1, 1, 1]);
  // ---- what stands round it
  const h = Y / 2; w.box([-60, -1, -60], [60, -0.03, 60], M.soil!, { tint: [1, 1, 1] });
  w.box([-h - 0.3, -0.5, -h - 0.3], [h, 2.6, -h], M.plaster!, { tint: [1, 1, 1] }); w.box([-h - 0.3, -0.5, -h], [-h, 2.6, h], M.paint!, { tint: [1.0, 0.50, 0.11] });
  w.box([-h, -0.5, h], [h, 0.06, h + 0.2], M.concrete!); w.box([h, -0.5, -h], [h + 0.2, 0.06, h + 0.2], M.concrete!);
  w.lamp({ kind: 'rect', at: [0, 2.55, -h + 0.4], u: [1.2, 0, 0], v: [0, 0, 0.12], n: [0, -1, 0], radiance: [40, 30, 18], name: 'wall', on: 1, night: true });
  const solid = new Solid(w, []), people: Building['people'] = [], bodies = new Bodies(solid, [], people), player = new Player(w, solid, [2, 0.05, 2], []); settle(w.pieces.filter(p => !(p.flags & F.MESH))); solid.settle(); for (const [p, mask] of freeEdges(w.pieces.filter(p => !(p.flags & F.MESH)))) p.flags |= mask << 16; for (const p of w.moving) if (!p.quad && !p.planes?.length && !p.bend) p.flags |= 0xfff << 16;
  // where their light is measured: stones with nothing but stones about them, and the two heroes as they stand
  const box = (o: PackedObject, at: V3, r: number): { lo: V3; hi: V3; levels: number } => ({ lo: [at[0] - r, Math.max(0.05, at[1] + Math.min(0, o.mn[1])), at[2] - r], hi: [at[0] + r, at[1] + o.mx[1] - 0.05, at[2] + r], levels: 1 }), e = Y / 2; made.classes = [{ lo: [0.55 * e, 0, 0.55 * e], hi: [0.85 * e, 0.02, 0.85 * e], levels: 2 }, box(heroes[0]!, [0, 0, 0], 0.3), box(heroes[1]!, [0.95, 0.262, 0.75], 0.25)];
  w.meshes = made;
  return { hung: [], bodies, solid, player, world: w, doors: [], people, levels: [{ name: 'All of it', cut: 1e9, look: [0.2, 0.5, 0.2] }], pool: [0, 0, -1, -1], poolTop: -99, poolDeep: 0, place: PLACE };
}
const PLACE: Place = {
  name: 'meshes', title: 'STONE YARD', rooms: [[40, -0.2, 40, 42, 2, 42]], openFloor: -99, sunAt: [0, 1, 0], sunSize: 30, outdoors: 0,
  start: { hour: 15.4, look: [0.25, 0.55, 0.2], yaw: 0.62, pitch: 0.62, dist: 9, level: 0, wet: 0, rain: 0, steam: 0 }, floors: [0, 0], pan: [8, 8],
  legs: [{ name: 'yard', look: [0, 0.5, 0], dist: 12, level: 0 }], groups: ['wall'], quiet: { hour: 23, groups: { wall: 0 } }, fires: [[2, 0.05, -2]], smokes: [[-2, 0.05, 2]], judged: [['all', [-99, -99, -99, 99, 99, 99]]],
};
