// MESHES ON THE GPU: their clusters (to draw), their trees of triangles (for rays), and where each stands.

import { makeBuffer, type Gpu } from '../gpu/device';
import type { Graph } from '../gpu/graph';
import { f32, packAll, struct, u32, vec2u, vec3f, vec4f, type Value } from '../gpu/layout';
import { boundsOfTriangles, triangleTree } from './triangles';
import { cutAt, makeLevels, type Levels } from './clusters';
import type { Mesh } from './shapes';
import { NO_TURN, type Quat, type Vec3 } from '../world/vec';

export interface MeshAsset {
  name: string;
  levels: Levels;
  /** the error of the cut that coarse rays meet, meters at scale 1 */
  coarse: number;
  low: Vec3;
  high: Vec3;
}
export interface MeshGiven {
  asset: MeshAsset;
  position: Vec3;
  scale: number;
  turn?: Quat;
  material: number;
}

const made = new Map<string, MeshAsset>();
/** a mesh with its levels; made once */
export function meshAsset(name: string, coarse: number, make: () => Mesh): MeshAsset {
  let asset = made.get(name);
  if (!asset) {
    const levels = makeLevels(make());
    asset = { name, levels, coarse, ...boundsOfTriangles(levels.points, cutAt(levels, 0)) };
    made.set(name, asset);
  }
  return asset;
}

const Vertex = struct('Vertex', { position: vec3f(''), spare0: f32(''), normal: vec3f(''), spare1: f32('') });
const ClusterRow = struct('Cluster', {
  center: vec3f(''),
  radius: f32(''),
  parent_center: vec3f(''),
  parent_radius: f32(''),
  error: f32('how far it stands off the finest, m'),
  parent_error: f32('the same of what takes its place'),
  first: u32('its first triangle'),
  count: u32('its triangles'),
  level: u32(''),
  spare0: u32(''),
  spare1: u32(''),
  spare2: u32(''),
});
const Instance = struct('Instance', {
  position: vec3f('', { unit: 'm' }),
  scale: f32(''),
  turn: vec4f(''),
  material: u32(''),
  spare0: u32(''),
  spare1: u32(''),
  spare2: u32(''),
});
const Job = struct('Job', { instance: u32(''), cluster: u32('') });
const Corner = struct('Corner', { point: u32('') });
export const MOST_JOBS = 1 << 16;

export class Meshes {
  jobCount = 0;
  /** triangles in all the finest meshes that stand in the room */
  finest = 0;
  /** where in the buffer of triangle trees each asset begins: fine, coarse */
  roots = new Map<string, { fine: number; coarse: number }>();

  private buffers = new Map<string, GPUBuffer>();
  readonly args: GPUBuffer;

  constructor(private gpu: Gpu, graph: Graph) {
    const { STORAGE, INDIRECT } = GPUBufferUsage;
    this.args = makeBuffer(gpu, 'mesh args', 16, STORAGE | INDIRECT);
    graph.addShared('mesh_args', { type: 'array<atomic<u32>, 4>', structs: [], access: 'read_write', buffer: () => this.args });
    const read = (name: string, type: string, structs: (typeof Vertex)[]) => {
      this.put(name, new ArrayBuffer(128));
      graph.addShared(name, { type, structs, access: 'read', buffer: () => this.buffers.get(name)! });
    };
    read('mesh_vertices', 'array<Vertex>', [Vertex]);
    read('mesh_corners', 'array<Corner>', [Corner]);
    read('mesh_clusters', 'array<Cluster>', [ClusterRow]);
    read('mesh_instances', 'array<Instance>', [Instance]);
    read('mesh_jobs', 'array<Job>', [Job]);
    read('mesh_data', 'array<u32>', []);
  }
  bufferOf(name: string): GPUBuffer {
    return this.buffers.get(name)!;
  }

  private put(name: string, data: ArrayBuffer): void {
    const old = this.buffers.get(name);
    if (!old || old.size < data.byteLength) {
      this.buffers.set(name, makeBuffer(this.gpu, name, data.byteLength, GPUBufferUsage.STORAGE));
    }
    this.gpu.device.queue.writeBuffer(this.buffers.get(name)!, 0, data);
  }

  open(given: MeshGiven[]): void {
    const assets = [...new Set(given.map((g) => g.asset))];
    const vertices: Record<string, Value>[] = [];
    const corners: number[] = [];
    const clusters: Record<string, Value>[] = [];
    const firstCluster = new Map<MeshAsset, number>();
    const words: number[] = [];
    const bits = new DataView(new ArrayBuffer(4));
    const asWord = (x: number) => (bits.setFloat32(0, x), bits.getUint32(0));
    this.roots.clear();
    for (const asset of assets) {
      const { levels } = asset;
      const base = vertices.length;
      levels.points.forEach((p, i) => vertices.push({ position: p, normal: levels.normals[i]! }));
      firstCluster.set(asset, clusters.length);
      for (const c of levels.clusters) {
        clusters.push({
          center: c.ball.center, radius: c.ball.radius,
          parent_center: c.parentBall.center, parent_radius: c.parentBall.radius,
          error: c.error, parent_error: Number.isFinite(c.parentError) ? c.parentError : 1e9,
          first: corners.length / 3, count: c.triangles.length, level: c.level,
        });
        for (const t of c.triangles) corners.push(...t.map((i) => base + i));
      }
      const tree = (error: number) => triangleTree(levels, cutAt(levels, error), words, asWord);
      this.roots.set(asset.name, { fine: tree(0), coarse: tree(asset.coarse) });
    }
    const jobs: Record<string, Value>[] = [];
    given.forEach((g, instance) => {
      const first = firstCluster.get(g.asset)!;
      g.asset.levels.clusters.forEach((_, i) => jobs.push({ instance, cluster: first + i }));
    });
    if (jobs.length > MOST_JOBS) throw new Error('too many clusters in this room');
    this.jobCount = jobs.length;
    this.finest = given.reduce((sum, g) => sum + g.asset.levels.clusters.filter((c) => c.level === 0).reduce((n, c) => n + c.triangles.length, 0), 0);
    if (given.length === 0) return;
    this.put('mesh_vertices', packAll(Vertex, vertices));
    this.put('mesh_corners', new Uint32Array(corners).buffer);
    this.put('mesh_clusters', packAll(ClusterRow, clusters));
    this.put('mesh_jobs', packAll(Job, jobs));
    this.put('mesh_instances', packAll(Instance, given.map((g) => ({
      position: g.position, scale: g.scale, turn: g.turn ?? NO_TURN, material: g.material,
    }))));
    this.put('mesh_data', new Uint32Array(words).buffer);
  }
}
export { vec2u };
