// SHAPES, from seeds; nothing is loaded. A mesh is points, smooth normals and triangles that share points.

import { add, cross, dot, length, normalize, scale, sub, type Vec3 } from '../world/vec';

export interface Mesh {
  points: Vec3[];
  normals: Vec3[];
  triangles: [number, number, number][];
}

/** smooth normals: each triangle pulls its corners its way, by its area */
export function normalsOf(points: Vec3[], triangles: [number, number, number][]): Vec3[] {
  const normals = points.map((): Vec3 => [0, 0, 0]);
  for (const t of triangles) {
    const [a, b, c] = t.map((i) => points[i]!) as [Vec3, Vec3, Vec3];
    const n = cross(sub(b, a), sub(c, a));
    for (const i of t) normals[i] = add(normals[i]!, n);
  }
  return normals.map(normalize);
}

/** a ball of 20 · 4^rounds triangles */
export function ball(rounds: number): { points: Vec3[]; triangles: [number, number, number][] } {
  const g = (1 + Math.sqrt(5)) / 2;
  const points = ([
    [-1, g, 0], [1, g, 0], [-1, -g, 0], [1, -g, 0], [0, -1, g], [0, 1, g],
    [0, -1, -g], [0, 1, -g], [g, 0, -1], [g, 0, 1], [-g, 0, -1], [-g, 0, 1],
  ] as Vec3[]).map(normalize);
  let triangles: [number, number, number][] = [
    [0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11], [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8],
    [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9], [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1],
  ];
  for (let round = 0; round < rounds; round++) {
    const middles = new Map<string, number>();
    const middle = (a: number, b: number) => {
      const key = a < b ? `${a},${b}` : `${b},${a}`;
      let i = middles.get(key);
      if (i === undefined) {
        i = points.length;
        points.push(normalize(add(points[a]!, points[b]!)));
        middles.set(key, i);
      }
      return i;
    };
    triangles = triangles.flatMap(([a, b, c]) => {
      const ab = middle(a, b);
      const bc = middle(b, c);
      const ca = middle(c, a);
      return [[a, ab, ca], [ab, b, bc], [ca, bc, c], [ab, bc, ca]] as [number, number, number][];
    });
  }
  return { points, triangles };
}

export function random(seed: number): () => number {
  let s = seed >>> 0 || 1;
  return () => {
    s = (s + 0x6d2b79f5) >>> 0;
    let t = Math.imul(s ^ (s >>> 15), 1 | s);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

/** smooth noise about −1 to 1: a sum of waves in random directions, finer and finer */
function waves(seed: number, octaves: number, perOctave = 6): (p: Vec3) => number {
  const next = random(seed);
  const terms: { k: Vec3; phase: number; strength: number }[] = [];
  for (let o = 0; o < octaves; o++) {
    for (let i = 0; i < perOctave; i++) {
      const way = normalize([next() - 0.5, next() - 0.5, next() - 0.5]);
      terms.push({ k: scale(way, 2.4 * 2 ** o * (0.8 + 0.4 * next())), phase: next() * 6.28, strength: 0.5 ** o / Math.sqrt(perOctave) });
    }
  }
  return (p) => terms.reduce((sum, t) => sum + t.strength * Math.sin(dot(p, t.k) + t.phase), 0);
}

/** a pebble, about 1 across: a ball worn flat on some sides, with bumps down to its triangles */
export function pebble(seed: number, rounds: number): Mesh {
  const next = random(seed * 7919 + 1);
  const { points, triangles } = ball(rounds);
  const flats = Array.from({ length: 7 }, () => ({
    n: normalize([next() - 0.5, (next() - 0.5) * 1.3, next() - 0.5]),
    d: 0.62 + 0.3 * next(),
  }));
  const squash: Vec3 = [0.9 + 0.3 * next(), 0.6 + 0.25 * next(), 0.8 + 0.3 * next()];
  const bumps = waves(seed, 5);
  const shaped = points.map((p) => {
    // how far out the flats let this direction go, with their edges rounded
    let out = 1;
    for (const f of flats) {
      const along = dot(p, f.n);
      if (along > 0.05) {
        const limit = f.d / along;
        const k = 0.12;
        const h = Math.max(0, k - Math.abs(out - limit)) / k;
        out = Math.min(out, limit) - h * h * k * 0.25;
      }
    }
    const far = 0.5 * out * (1 + 0.035 * bumps(scale(p, 1.6)));
    return [p[0] * far * squash[0], p[1] * far * squash[1], p[2] * far * squash[2]] as Vec3;
  });
  return { points: shaped, triangles, normals: normalsOf(shaped, triangles) };
}

/** a broken column, 1 high and about 0.4 across: flutes, a worn foot, a jagged top */
export function column(around: number, up: number): Mesh {
  const bumps = waves(5, 6);
  const points: Vec3[] = [];
  const triangles: [number, number, number][] = [];
  for (let j = 0; j <= up; j++) {
    for (let i = 0; i < around; i++) {
      const a = (i / around) * 2 * Math.PI;
      const t = j / up;
      const flute = 0.012 * Math.abs(Math.cos(a * 10)) ** 0.7;
      const foot = 0.05 * Math.exp(-t * 18) + 0.015 * Math.exp(-((t - 0.12) ** 2) * 900);
      const p: Vec3 = [Math.cos(a), t * 5, Math.sin(a)];
      const broken = 0.86 + 0.14 * bumps([Math.cos(a) * 1.2, 0, Math.sin(a) * 1.2]);
      const y = Math.min(1, t / 0.97) * broken;
      const r = (0.17 - 0.015 * t + flute + foot) * (1 + 0.03 * bumps(p)) * (j === up ? 0 : t > 0.97 ? 1 - (t - 0.97) * 12 : 1);
      points.push([Math.cos(a) * r, y, Math.sin(a) * r]);
    }
  }
  for (let j = 0; j < up; j++) {
    for (let i = 0; i < around; i++) {
      const a = j * around + i;
      const b = j * around + ((i + 1) % around);
      triangles.push([a, a + around, b], [b, a + around, b + around]);
    }
  }
  return { points, triangles, normals: normalsOf(points, triangles) };
}

export const radiusOf = (m: Mesh): number => m.points.reduce((r, p) => Math.max(r, length(p)), 0);
