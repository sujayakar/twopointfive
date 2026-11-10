// THE WORLD ON THE GPU: what a room is, as buffers. Things that stand still are written once; things
// that move are written again each tick, with a small tree of their own.

import { f32, i32, pack, packAll, struct, u32, vec3f, vec4f, type Value } from '../gpu/layout';
import { makeBuffer, type Gpu } from '../gpu/device';
import type { Graph } from '../gpu/graph';
import { blueNoise } from './blue';
import { coatTable } from './coat';
import { Builder, placed, type LampGiven, type Material, type PieceGiven, type Place, type RoomSpec } from './room';
import { boundsOf, buildTree, type TreeNode } from './tree';
import { NO_TURN, cross, length, normalize, scale, type Vec3 } from './vec';

export const NOTHING = 0xffffffff;
const MOST = { materials: 64, lamps: 80, objects: 128, planes: 1024, openings: 8 };

const Piece = struct('Piece', {
  center: vec3f('', { unit: 'm' }),
  shape: u32('box, ball or cylinder'),
  half_size: vec3f('', { unit: 'm' }),
  material: u32(''),
  turn: vec4f('from its own frame to the world'),
  lamp: u32('which lamp it is the body of, or NOTHING'),
  object: u32('what it moves with; 0 is the room'),
  first_plane: u32(''),
  plane_count: u32(''),
});
const Node = struct('Node', {
  low: vec3f('', { unit: 'm' }),
  first: u32("a leaf's first piece, or the first of two children"),
  high: vec3f('', { unit: 'm' }),
  count: u32('pieces in a leaf; 0 in an inner node'),
});
const MaterialStruct = struct('Material', {
  matte: vec3f('share of light sent back evenly'),
  roughness: f32(''),
  through: vec3f('share that comes out on the other side'),
  metal: f32(''),
  matte_other: vec3f('second color of a pattern'),
  pattern: u32('plain, checker or strips'),
  pattern_size: f32('', { unit: 'm' }),
  roughness_other: f32(''),
  spare0: f32(''),
  spare1: f32(''),
});
const Lamp = struct('Lamp', {
  center: vec3f('', { unit: 'm' }),
  kind: u32('ball, panel or sun'),
  radiance: vec3f('what each bit of it sends, every way'),
  changing: u32('switched, swung or carried'),
  half_u: vec3f('half an edge of a panel', { unit: 'm' }),
  radius: f32('of a ball (m) or of the sun (radians)'),
  half_v: vec3f('the other half edge', { unit: 'm' }),
  area: f32('', { unit: 'm²' }),
  normal: vec3f('the way a panel shines; the way to the sun'),
  spare: f32(''),
});
const ObjectStruct = struct('Object', {
  position: vec3f('', { unit: 'm' }),
  spare0: f32(''),
  turn: vec4f(''),
  position_before: vec3f('last frame', { unit: 'm' }),
  spare1: f32(''),
  turn_before: vec4f(''),
});
const Opening = struct('Opening', {
  center: vec3f('', { unit: 'm' }),
  spare0: f32(''),
  half_u: vec3f('half an edge', { unit: 'm' }),
  spare1: f32(''),
  half_v: vec3f('the other half edge', { unit: 'm' }),
  spare2: f32(''),
});
const WorldStruct = struct('World', {
  sky_zenith: vec3f(''),
  still_pieces: u32(''),
  sky_horizon: vec3f(''),
  moving_pieces: u32(''),
  sky_ground: vec3f(''),
  moving_root: u32('where the tree of what moves begins'),
  lamp_count: u32(''),
  opening_count: u32(''),
  spare1: u32(''),
  spare2: i32(''),
});

const SHAPES = { box: 0, ball: 1, cylinder: 2 } as const;
const LAMPS = { ball: 0, panel: 1, sun: 2 } as const;
const PATTERNS = { plain: 0, checker: 1, strips: 2 } as const;
/** half a degree across */
const SUN_RADIUS = 0.00465;

export const WORLD_CONSTANTS = [
  ...Object.entries(SHAPES).map(([k, v]) => `const SHAPE_${k.toUpperCase()} = ${v}u;`),
  ...Object.entries(LAMPS).map(([k, v]) => `const LAMP_${k.toUpperCase()} = ${v}u;`),
  ...Object.entries(PATTERNS).map(([k, v]) => `const PATTERN_${k.toUpperCase()} = ${v}u;`),
].join('\n');

export class World {
  room!: RoomSpec;
  sliders: Record<string, number> = {};
  pieceCount = 0;
  lampCount = 0;
  /** counts up whenever what is in the world is made anew */
  version = 0;

  private still: PieceGiven[] = [];
  private movers: PieceGiven[] = [];
  private lamps: LampGiven[] = [];
  private lampObject = new Map<number, number>();
  private objects = 1;
  private places = new Map<number, Place>();
  private stillNodes: TreeNode[] = [];
  private planes: number[][] = [];
  private planeStart = new Map<PieceGiven['planes'], number>();

  private bytes = new Map<string, number>();
  private buffers = new Map<string, GPUBuffer>();

  constructor(private gpu: Gpu, graph: Graph) {
    const U = GPUBufferUsage.UNIFORM;
    this.fixed('world', WorldStruct.stride, U);
    this.fixed('materials', MOST.materials * MaterialStruct.stride, U);
    this.fixed('lamps', MOST.lamps * Lamp.stride, U);
    this.fixed('objects', MOST.objects * ObjectStruct.stride, U);
    this.fixed('planes', MOST.planes * 16, U);
    this.fixed('openings', MOST.openings * Opening.stride, U);
    this.fixed('tables', (64 * 64 + 16 * 16 * 4) * 4, U);
    const tables = new Float32Array(64 * 64 + 16 * 16 * 4);
    tables.set(blueNoise(64));
    tables.set(coatTable(), 64 * 64);
    this.write('tables', tables.buffer);

    const uniform = (name: string, type: string, structs = [] as (typeof Piece)[]) =>
      graph.addShared(name, { type, structs, access: 'uniform', buffer: () => this.buffers.get(name)! });
    const storage = (name: string, type: string, structs = [] as (typeof Piece)[]) =>
      graph.addShared(name, { type, structs, access: 'read', buffer: () => this.buffers.get(name)! });
    uniform('world', 'World', [WorldStruct]);
    uniform('materials', `array<Material, ${MOST.materials}>`, [MaterialStruct]);
    uniform('lamps', `array<Lamp, ${MOST.lamps}>`, [Lamp]);
    uniform('objects', `array<Object, ${MOST.objects}>`, [ObjectStruct]);
    uniform('tables', 'Tables', [TABLES]);
    storage('pieces', 'array<Piece>', [Piece]);
    storage('nodes', 'array<Node>', [Node]);
    uniform('planes', `array<vec4f, ${MOST.planes}>`);
    uniform('openings', `array<Opening, ${MOST.openings}>`, [Opening]);
  }

  private fixed(name: string, bytes: number, usage: GPUBufferUsageFlags): void {
    this.buffers.set(name, makeBuffer(this.gpu, name, bytes, usage));
    this.bytes.set(name, bytes);
  }
  private write(name: string, data: ArrayBuffer, offset = 0): void {
    this.gpu.device.queue.writeBuffer(this.buffers.get(name)!, offset, data);
  }
  /** a storage buffer at least this large */
  private atLeast(name: string, bytes: number): void {
    if ((this.bytes.get(name) ?? 0) >= bytes && this.buffers.has(name)) return;
    this.fixed(name, bytes, GPUBufferUsage.STORAGE);
  }

  /** makes the room from its code and its sliders */
  open(room: RoomSpec, sliders?: Record<string, number>): void {
    this.room = room;
    this.sliders = sliders ?? Object.fromEntries((room.sliders ?? []).map((s) => [s.key, s.value]));
    const b = new Builder();
    room.build(b, this.sliders);
    if (b.materials.length > MOST.materials) throw new Error(`${room.name}: too many materials`);
    if (b.lamps.length > MOST.lamps) throw new Error(`${room.name}: too many lamps`);
    if (b.objects > MOST.objects) throw new Error(`${room.name}: too many objects`);
    this.objects = b.objects;
    this.lamps = b.lamps;
    this.lampCount = b.lamps.length;
    this.pieceCount = b.pieces.length;
    this.lampObject.clear();
    b.pieces.forEach((p) => p.lamp !== undefined && this.lampObject.set(p.lamp, p.object ?? 0));

    this.planes = [];
    this.planeStart.clear();
    for (const p of b.pieces) {
      if (!p.planes?.length || this.planeStart.has(p.planes)) continue;
      this.planeStart.set(p.planes, this.planes.length);
      for (const [n, d] of p.planes) this.planes.push([...normalize(n), d / length(n)]);
    }

    const isMover = (p: PieceGiven) => (p.object ?? 0) > 0;
    const still = b.pieces.filter((p) => !isMover(p));
    this.movers = b.pieces.filter(isMover);
    const tree = buildTree(still.map(boundsOf));
    this.still = tree.order.map((i) => still[i]!);
    this.stillNodes = tree.nodes;

    this.atLeast('pieces', (b.pieces.length + 1) * Piece.stride);
    this.atLeast('nodes', (this.stillNodes.length + Math.max(1, this.movers.length * 2)) * Node.stride);
    if (this.planes.length > MOST.planes) throw new Error(`${room.name}: too many planes`);
    this.write('pieces', packAll(Piece, this.still.map((p) => this.pieceRow(p))));
    this.write('nodes', packAll(Node, this.stillNodes as unknown as Record<string, Value>[]));
    if (this.planes.length) this.write('planes', new Float32Array(this.planes.flat()).buffer);
    this.write('materials', packAll(MaterialStruct, b.materials.map(materialRow)));
    const sky = room.sky ?? { zenith: [0, 0, 0], horizon: [0, 0, 0], ground: [0, 0, 0] };
    const row = new ArrayBuffer(WorldStruct.stride);
    pack(WorldStruct, new DataView(row), 0, {
      sky_zenith: sky.zenith, sky_horizon: sky.horizon, sky_ground: sky.ground,
      still_pieces: this.still.length, moving_pieces: this.movers.length,
      moving_root: this.stillNodes.length, lamp_count: b.lamps.length, opening_count: b.openings.length,
    });
    this.write('openings', packAll(Opening, b.openings.map((o) => ({ center: o.center, half_u: o.halfU, half_v: o.halfV }))));
    this.write('world', row);
    this.places.clear();
    this.place(0);
    this.place(0);
    this.version++;
  }

  /** puts everything that moves where it is at this time. True if something moved */
  place(seconds: number): boolean {
    const now = this.room.move?.(seconds, this.sliders) ?? new Map<number, Place>();
    const rows: Record<string, Value>[] = [];
    let moved = false;
    const REST: Place = { position: [0, 0, 0], turn: NO_TURN };
    const at = (id: number) => now.get(id) ?? REST;
    for (let id = 0; id < this.objects; id++) {
      const here = at(id);
      const before = this.places.get(id) ?? here;
      rows.push({
        position: here.position, turn: here.turn,
        position_before: before.position, turn_before: before.turn,
      });
      if (String([before.position, before.turn]) !== String([here.position, here.turn])) moved = true;
      this.places.set(id, here);
    }
    this.write('objects', packAll(ObjectStruct, rows));

    if (this.movers.length) {
      const standing = this.movers.map((p) => placed(p, at(p.object!)));
      const tree = buildTree(standing.map(boundsOf), this.stillNodes.length, this.still.length);
      const ordered = tree.order.map((i) => standing[i]!);
      this.write('pieces', packAll(Piece, ordered.map((p) => this.pieceRow(p))), this.still.length * Piece.stride);
      this.write('nodes', packAll(Node, tree.nodes as unknown as Record<string, Value>[]), this.stillNodes.length * Node.stride);
    }
    this.write('lamps', packAll(Lamp, this.lamps.map((l, i) => lampRow(l, at(this.lampObject.get(i) ?? 0)))));
    return moved;
  }

  private pieceRow(p: PieceGiven): Record<string, Value> {
    return {
      center: p.center, shape: SHAPES[p.shape ?? 'box'], half_size: p.half, material: p.material,
      turn: p.turn ?? NO_TURN, lamp: p.lamp ?? NOTHING, object: p.object ?? 0,
      first_plane: this.planeStart.get(p.planes) ?? 0, plane_count: p.planes?.length ?? 0,
    };
  }
}

const TABLES = struct('Tables', {});
TABLES.wgsl = `struct Tables {
  blue: array<vec4f, 1024>, // 64 × 64 ranks
  coat: array<vec4f, 256>,  // 16 roughnesses × 16 slants: A, B
}`;

function materialRow(m: Material): Record<string, Value> {
  return {
    matte: m.matte, roughness: m.roughness ?? 1, through: m.through ?? [0, 0, 0], metal: m.metal ?? 0,
    matte_other: m.matte2 ?? m.matte, pattern: PATTERNS[m.pattern ?? 'plain'],
    pattern_size: m.patternSize ?? 1, roughness_other: m.roughness2 ?? m.roughness ?? 1,
  };
}

function lampRow(l: LampGiven, at: Place): Record<string, Value> {
  if (l.kind === 'sun') {
    return { kind: LAMPS.sun, radiance: l.radiance, normal: l.toward!, radius: l.radius ?? SUN_RADIUS, changing: 0 };
  }
  const turned = placed({ center: l.center!, half: [0, 0, 0], material: 0 }, at);
  const spin = (v: Vec3): Vec3 =>
    placed({ center: v, half: [0, 0, 0], material: 0 }, { position: [0, 0, 0], turn: at.turn }).center;
  const u = spin(l.halfU ?? [0, 0, 0]);
  const v = spin(l.halfV ?? [0, 0, 0]);
  const n = cross(u, v);
  return {
    center: turned.center, kind: LAMPS[l.kind], radiance: l.radiance, changing: l.changing ? 1 : 0,
    half_u: u, half_v: v, radius: l.radius ?? 0,
    area: l.kind === 'panel' ? 4 * length(n) : 4 * Math.PI * l.radius! ** 2,
    normal: l.kind === 'panel' ? scale(n, 1 / length(n)) : [0, 1, 0],
  };
}
