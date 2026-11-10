// FEWER TRIANGLES. Edges are pulled together, one after another, where that moves the surface least. How far
// a point is from the planes of all triangles it has ever stood for is a quadric: ten numbers that add up
// (Garland and Heckbert, 1997).

import { add, cross, dot, normalize, scale, sub, type Vec3 } from '../world/vec';

type Quadric = number[]; // xx xy xz xw yy yz yw zz zw ww, and the area behind it
const ZERO = (): Quadric => [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

function planeQuadric(n: Vec3, d: number, area: number): Quadric {
  const [x, y, z] = n;
  return [x * x, x * y, x * z, x * d, y * y, y * z, y * d, z * z, z * d, d * d, 1].map((v) => v * area);
}
/** the mean squared distance from all the planes, at p */
function distance2(q: Quadric, p: Vec3): number {
  const [x, y, z] = p;
  const sum =
    q[0]! * x * x + 2 * q[1]! * x * y + 2 * q[2]! * x * z + 2 * q[3]! * x +
    q[4]! * y * y + 2 * q[5]! * y * z + 2 * q[6]! * y +
    q[7]! * z * z + 2 * q[8]! * z + q[9]!;
  return Math.max(0, sum / (q[10]! || 1));
}
/** where that is least; null where the planes leave it open */
function best(q: Quadric): Vec3 | null {
  const a: Vec3 = [q[0]!, q[1]!, q[2]!];
  const b: Vec3 = [q[1]!, q[4]!, q[5]!];
  const c: Vec3 = [q[2]!, q[5]!, q[7]!];
  const det = dot(a, cross(b, c));
  if (Math.abs(det) < 1e-9 * q[10]! ** 3) return null;
  const r: Vec3 = [-q[3]!, -q[6]!, -q[8]!];
  return [dot(r, cross(b, c)) / det, dot(a, cross(r, c)) / det, dot(a, cross(b, r)) / det];
}

interface Pull {
  cost: number;
  a: number;
  b: number;
  to: Vec3;
  /** how often either end had moved when this was worked out */
  stamp: number;
}

/** the least cost first */
class Heap {
  private items: Pull[] = [];
  push(p: Pull): void {
    let i = this.items.push(p) - 1;
    while (i > 0) {
      const up = (i - 1) >> 1;
      if (this.items[up]!.cost <= p.cost) break;
      this.items[i] = this.items[up]!;
      i = up;
    }
    this.items[i] = p;
  }
  pop(): Pull | undefined {
    const top = this.items[0];
    const last = this.items.pop();
    if (this.items.length === 0 || !last) return top;
    let i = 0;
    for (;;) {
      let down = 2 * i + 1;
      if (down >= this.items.length) break;
      if (down + 1 < this.items.length && this.items[down + 1]!.cost < this.items[down]!.cost) down++;
      if (this.items[down]!.cost >= last.cost) break;
      this.items[i] = this.items[down]!;
      i = down;
    }
    this.items[i] = last;
    return top;
  }
}

export interface Simplified {
  /** triangles over the same points, which have grown */
  triangles: [number, number, number][];
  /** the most that any point has moved off the planes it stands for, meters */
  error: number;
}

/**
 * Halves a patch (or as near as it can). Points that `isHeld` stay where they are, so that the patch still fits
 * what lies beside it. Points that move are new: they are added to `points` and `normals`.
 */
export function simplify(
  triangles: [number, number, number][],
  points: Vec3[],
  normals: Vec3[],
  isHeld: (point: number) => boolean,
  target: number,
): Simplified {
  // the patch in numbers of its own
  const own = new Map<number, number>();
  const given: number[] = [];
  const at: Vec3[] = [];
  const held: boolean[] = [];
  const local = triangles.map((t) =>
    t.map((g) => {
      let i = own.get(g);
      if (i === undefined) {
        i = at.length;
        own.set(g, i);
        given.push(g);
        at.push(points[g]!);
        held.push(isHeld(g));
      }
      return i;
    }) as [number, number, number],
  );
  const alive = local.map(() => true);
  const around: Set<number>[] = at.map(() => new Set());
  local.forEach((t, i) => t.forEach((v) => around[v]!.add(i)));
  const normalOf = (t: [number, number, number], moved?: number, to?: Vec3): Vec3 => {
    const [a, b, c] = t.map((v) => (v === moved ? to! : at[v]!)) as [Vec3, Vec3, Vec3];
    return cross(sub(b, a), sub(c, a));
  };
  const quadrics = at.map(ZERO);
  for (const t of local) {
    const n = normalOf(t);
    const area = Math.hypot(...n) / 2;
    if (area === 0) continue;
    const unit = normalize(n);
    const q = planeQuadric(unit, -dot(unit, at[t[0]]!), area);
    for (const v of t) quadrics[v] = quadrics[v]!.map((x, i) => x + q[i]!);
  }

  // an open rim has nothing beside it to say where it is: a plane through each of its edges does
  const uses = new Map<string, { a: number; b: number; n: Vec3; times: number }>();
  for (const t of local) {
    for (let k = 0; k < 3; k++) {
      const a = t[k]!;
      const b = t[(k + 1) % 3]!;
      const key = `${Math.min(a, b)},${Math.max(a, b)}`;
      const was = uses.get(key);
      if (was) was.times++;
      else uses.set(key, { a, b, n: normalize(normalOf(t)), times: 1 });
    }
  }
  for (const { a, b, n, times } of uses.values()) {
    if (times !== 1) continue;
    const edge = sub(at[b]!, at[a]!);
    const out = normalize(cross(edge, n));
    const q = planeQuadric(out, -dot(out, at[a]!), dot(edge, edge));
    for (const v of [a, b]) quadrics[v] = quadrics[v]!.map((x, i) => x + q[i]!);
  }

  const stamps = at.map(() => 0);
  const moved = at.map(() => false);
  const gone = at.map(() => false);
  const heap = new Heap();
  const offer = (a: number, b: number) => {
    if (held[a] && held[b]) return;
    const q = quadrics[a]!.map((x, i) => x + quadrics[b]![i]!);
    const middle = scale(add(at[a]!, at[b]!), 0.5);
    let to = held[a] ? at[a]! : held[b] ? at[b]! : (best(q) ?? middle);
    const edge = Math.hypot(...sub(at[a]!, at[b]!));
    if (!held[a] && !held[b] && Math.hypot(...sub(to, middle)) > edge) to = middle;
    // on a flat every pull is free: the short edges go first
    heap.push({ cost: distance2(q, to) + 1e-7 * edge * edge, a, b, to, stamp: stamps[a]! + stamps[b]! });
  };
  const edges = new Set<string>();
  for (const t of local) {
    for (let k = 0; k < 3; k++) {
      const a = Math.min(t[k]!, t[(k + 1) % 3]!);
      const b = Math.max(t[k]!, t[(k + 1) % 3]!);
      const key = `${a},${b}`;
      if (edges.has(key)) continue;
      edges.add(key);
      offer(a, b);
    }
  }
  /** would moving v there turn one of its triangles over? */
  const turns = (v: number, other: number, to: Vec3): boolean => {
    for (const i of around[v]!) {
      const t = local[i]!;
      if (!alive[i] || t.includes(other)) continue;
      const before = normalOf(t);
      const after = normalOf(t, v, to);
      if (dot(before, after) < 0.3 * Math.hypot(...before) * Math.hypot(...after)) return true;
    }
    return false;
  };

  let count = local.length;
  let error2 = 0;
  while (count > target) {
    const pull = heap.pop();
    if (!pull) break;
    const { a, b, to } = pull;
    if (gone[a] || gone[b] || pull.stamp !== stamps[a]! + stamps[b]!) continue;
    if (turns(a, b, to) || turns(b, a, to)) continue;
    error2 = Math.max(error2, distance2(quadrics[a]!.map((x, i) => x + quadrics[b]![i]!), to));
    // b's triangles become a's
    const [keep, lose] = held[b] ? [b, a] : [a, b];
    for (const i of around[lose]!) {
      if (!alive[i]) continue;
      const t = local[i]!;
      if (t.includes(keep)) {
        alive[i] = false;
        count--;
        continue;
      }
      t[t.indexOf(lose)] = keep;
      around[keep]!.add(i);
    }
    gone[lose] = true;
    quadrics[keep] = quadrics[keep]!.map((x, i) => x + quadrics[lose]![i]!);
    if (!held[keep]) {
      at[keep] = to;
      moved[keep] = true;
    }
    const beside = new Set<number>();
    for (const i of around[keep]!) if (alive[i]) local[i]!.forEach((v) => v !== keep && beside.add(v));
    stamps[keep]! += 1;
    for (const v of beside) offer(Math.min(keep, v), Math.max(keep, v));
  }

  // points that moved get new numbers, and the normals of what is left round them
  const numbers = given.slice();
  moved.forEach((m, v) => {
    if (!m || gone[v]) return;
    let n: Vec3 = [0, 0, 0];
    for (const i of around[v]!) if (alive[i]) n = add(n, normalOf(local[i]!));
    numbers[v] = points.length;
    points.push(at[v]!);
    normals.push(normalize(n));
  });
  return {
    triangles: local.filter((_, i) => alive[i]).map((t) => t.map((v) => numbers[v]!) as [number, number, number]),
    error: Math.sqrt(error2),
  };
}
