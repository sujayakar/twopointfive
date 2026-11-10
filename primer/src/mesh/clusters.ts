// LEVELS OF A MESH, in clusters. The mesh is cut into clusters of at most 128 triangles. A few that lie together
// are a group: its inside is simplified to half, its rim held still, and cut into clusters again; and so on,
// until little is left. Each cluster knows its own error and that of the group that takes its place.
//
// For any error e, the clusters with  error ≤ e < parent's error  cover the surface once, without a crack.

import { length, sub, type Vec3 } from '../world/vec';
import { simplify } from './simplify';
import type { Mesh } from './shapes';

export const CLUSTER_SIZE = 128;
const GROUP_SIZE = 8;

export interface Ball {
  center: Vec3;
  radius: number;
}
export interface Cluster {
  triangles: [number, number, number][];
  level: number;
  /** how far it stands off the finest, meters, and the ball that the distance to the eye is measured to */
  error: number;
  ball: Ball;
  /** the same of the group it was simplified in; Infinity where nothing is coarser */
  parentError: number;
  parentBall: Ball;
}
export interface Levels {
  points: Vec3[];
  normals: Vec3[];
  clusters: Cluster[];
  levels: number;
}

/** items that lie together, in parts of at most `size`: halved along the longest side, again and again */
function parts<T>(items: T[], middle: (item: T) => Vec3, size: number, turn = 0): T[][] {
  if (items.length <= size) return items.length ? [items] : [];
  const low: Vec3 = [Infinity, Infinity, Infinity];
  const high: Vec3 = [-Infinity, -Infinity, -Infinity];
  const c = Math.cos(turn);
  const s = Math.sin(turn);
  // the axes are turned from level to level, so that the cuts of one are not those of the next
  const spot = (item: T): Vec3 => {
    const [x, y, z] = middle(item);
    return [c * x - s * z, y, s * x + c * z];
  };
  for (const item of items) spot(item).forEach((v, k) => {
    low[k] = Math.min(low[k]!, v);
    high[k] = Math.max(high[k]!, v);
  });
  const axis = [0, 1, 2].sort((a, b) => high[b]! - low[b]! - (high[a]! - low[a]!))[0]!;
  const sorted = [...items].sort((a, b) => spot(a)[axis]! - spot(b)[axis]!);
  const full = Math.ceil(items.length / size);
  const half = Math.round((Math.floor(full / 2) / full) * items.length) || items.length >> 1;
  return [...parts(sorted.slice(0, half), middle, size, turn), ...parts(sorted.slice(half), middle, size, turn)];
}

function ballOf(points: Vec3[], triangles: [number, number, number][]): Ball {
  const low: Vec3 = [Infinity, Infinity, Infinity];
  const high: Vec3 = [-Infinity, -Infinity, -Infinity];
  for (const t of triangles) for (const i of t) points[i]!.forEach((v, k) => {
    low[k] = Math.min(low[k]!, v);
    high[k] = Math.max(high[k]!, v);
  });
  const center = low.map((v, k) => (v + high[k]!) / 2) as Vec3;
  let radius = 0;
  for (const t of triangles) for (const i of t) radius = Math.max(radius, length(sub(points[i]!, center)));
  return { center, radius };
}
/** a ball round several */
function ballRound(balls: Ball[]): Ball {
  const center = [0, 1, 2].map((k) => balls.reduce((sum, b) => sum + b.center[k]!, 0) / balls.length) as Vec3;
  return { center, radius: Math.max(...balls.map((b) => length(sub(b.center, center)) + b.radius)) };
}

export function makeLevels(mesh: Mesh, holdRims = true): Levels {
  const points = mesh.points.slice();
  const normals = mesh.normals.slice();
  const middleOf = (t: [number, number, number]): Vec3 =>
    [0, 1, 2].map((k) => (points[t[0]]![k]! + points[t[1]]![k]! + points[t[2]]![k]!) / 3) as Vec3;
  const cluster = (triangles: [number, number, number][], level: number, error: number, ball?: Ball): Cluster => ({
    triangles, level, error, ball: ball ?? ballOf(points, triangles),
    parentError: Infinity, parentBall: { center: [0, 0, 0], radius: 0 },
  });

  let current = parts(mesh.triangles, middleOf, CLUSTER_SIZE).map((t) => cluster(t, 0, 0));
  const all = [...current];
  let level = 0;
  while (current.length > 1) {
    level++;
    const groups = parts(current, (c) => c.ball.center, GROUP_SIZE, level * 1.3);
    // a point that two groups share belongs to the rim of both
    const owner = new Map<number, number>();
    const shared = new Set<number>();
    groups.forEach((group, g) => {
      for (const c of group) for (const t of c.triangles) for (const i of t) {
        const was = owner.get(i);
        if (was !== undefined && was !== g) shared.add(i);
        owner.set(i, g);
      }
    });
    const next: Cluster[] = [];
    const fresh: Cluster[] = [];
    for (const group of groups) {
      const triangles = group.flatMap((c) => c.triangles);
      const made = simplify(triangles, points, normals, (i) => holdRims && shared.has(i), Math.floor(triangles.length / 2));
      if (made.triangles.length > triangles.length * 0.85) {
        // it cannot be made simpler here: it goes on as it is, beside what is, and may be in another group
        next.push(...group);
        continue;
      }
      // errors only grow on the way up, or a cut would have holes
      const error = Math.max(made.error, ...group.map((c) => c.error)) * 1.0001 + 1e-6;
      const ball = ballRound(group.map((c) => c.ball));
      for (const c of group) {
        c.parentError = error;
        c.parentBall = ball;
      }
      fresh.push(...parts(made.triangles, middleOf, CLUSTER_SIZE, level).map((t) => cluster(t, level, error, ball)));
    }
    if (fresh.length === 0) break;
    all.push(...fresh);
    current = [...next, ...fresh];
  }
  return { points, normals, clusters: all, levels: level + 1 };
}

/** the triangles of a cut at one error everywhere: for rays */
export function cutAt(levels: Levels, error: number): [number, number, number][] {
  return levels.clusters.filter((c) => c.error <= error && error < c.parentError).flatMap((c) => c.triangles);
}
