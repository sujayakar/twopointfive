// A SCENE, DRAWN FINE: which shape is drawn for each of its boxes. Few shapes fit them all (template.ts); what is laid or cast in pieces gets a skin of them (pavers, boards, sheets, panels); odd pieces are their own
// shape, once for all that are alike; people share one body. Rays, feet and blasts go on meeting the boxes.
import { type V3, type Rot, rng } from '../math';
import { F, type Piece, type World } from '../scene/world';
import { MATERIALS } from '../scene/materials';
import type { Building } from '../scene/building';
import { box, cushionBox, drum, round, limb, slab, pack, PLAIN, type Packed, type Shape } from './template';
import { body, far, snap } from './body';
import { normalsOf } from '../mesh/mesh';
import { solid, lighten } from '../mesh/made';

/** what stands somewhere: a shape, whose box it fits, what it is made of */
export interface Inst { shape: string; piece: Piece; /** its own box, if it is laid on the piece and is not the piece */ at?: V3; rot?: Rot; half?: V3; taper: number; tint?: V3; moves: boolean; /** whose body (pieces, in order) */ body?: Piece[] }
export interface FineScene { insts: Inst[]; shapes: string[] }

// ---------------------------------------------------------------- shapes, by what they are called (so that any worker can make any)
const RECIPES: Record<string, () => Shape> = {
  hard: () => box('hard', 1, 0.005, 0.0004, 0), metal: () => box('metal', 2, 0.0026, 0.0002, 0), cast: () => box('cast', 3, 0.008, 0.0022, 0.004), wood: () => box('wood', 4, 0.006, 0.0012, 0), soft: () => cushionBox(5),
  drum: () => drum('drum', 0.004, 0), pipe: () => drum('pipe', 0.003, 0.004), ball: () => round('ball', 4, 0), limb: () => limb(6), knot: () => round('knot', 4, 0.10), body: () => body(),
  ...Object.fromEntries([0, 1, 2].flatMap(i => [[`paver${i}`, () => slab(`paver${i}`, 20 + i, 0.003, 0.0008, 0.0012, 'stone')], [`board${i}`, () => slab(`board${i}`, 30 + i, 0.003, 0.0007, 0, 'wood')], [`panel${i}`, () => slab(`panel${i}`, 40 + i, 0.004, 0.0010, 0.0018, 'cast')], [`sheet${i}`, () => slab(`sheet${i}`, 50 + i, 0.003, 0.0012, 0, 'felt')]] as [string, () => Shape][])),
};
/** a piece that nothing else is like, as a shape of its own */
function odd(spec: string): Shape {
  const p = JSON.parse(spec) as Piece, m = Math.max(...p.half), cell = Math.max(0.0035, Math.min(0.022, Math.min(...p.half) / 2.2, m / 22)), net = solid((x, y, z) => far(p, x, y, z, 0), p.half.map(v => v + 0.002) as V3, cell, 0, () => 0, () => [1, 1, 1, 1], 0); snap(p, net.pos, cell * 0.62); const mesh = lighten({ ...net, nrm: normalsOf(net.pos, net.tri) }, 0.0003 + m * 0.0004); snap(p, mesh.pos, cell * 0.25); mesh.nrm = normalsOf(mesh.pos, mesh.tri);
  return { name: spec.slice(0, 40), kind: PLAIN, mesh };
}
export const makeShape = (name: string): Packed => { const s = name.startsWith('odd:') ? odd(name.slice(4)) : RECIPES[name]!(); return { ...pack(s, s.kind === PLAIN ? 24 : s.name === 'body' ? 60 : 12), name }; };

// ---------------------------------------------------------------- what is drawn for what
const FAMILY: Record<string, string> = { concrete: 'cast', stone: 'cast', plaster: 'cast', roof: 'cast', brick: 'cast', asphalt: 'cast', fabric: 'soft', cloth: 'soft', canvas: 'soft', wood: 'wood', steel: 'metal', brass: 'metal', chrome: 'metal', mirror: 'metal' };
const r3 = (v: number) => Math.round(v * 1e4) / 1e4;
function shapeOf(p: Piece): { shape: string; taper: number; rot?: Rot; half?: V3 } | null {
  const name = MATERIALS[p.mat]!.name, near = (a: number, b: number) => Math.abs(a - b) < 0.02 * Math.max(Math.abs(a), Math.abs(b), 1e-6), h = p.half, odds = () => ({ shape: 'odd:' + JSON.stringify({ half: h.map(r3), ...(p.quad ? { quad: p.quad.map(v => Math.round(v * 1e3) / 1e3) } : {}), ...(p.planes?.length ? { planes: p.planes.map(([n, d]) => [n.map(r3), r3(d)]) } : {}), ...(p.curves?.length ? { curves: p.curves.map(c => ({ at: c.at.map(r3), r: c.r.map(r3) })) } : {}) }), taper: 0 });
  if (p.planes?.length || p.curves?.length) return odds(); if (!p.quad) { const f = FAMILY[name] ?? 'hard'; return { shape: f === 'soft' && Math.min(...h) < 0.03 ? 'wood' : f, taper: 0 }; }
  const [A, B, C, D] = p.quad, bark = name === 'bark' || name === 'paleBark', q = [A, B, C], zero = q.map(v => Math.abs(v) < 1e-9);
  if (D) { const r0 = 1 / Math.sqrt(A), k = -D * r0 / 2, ra = r0 - k * h[1], rb = r0 + k * h[1]; if (!near(Math.max(ra, rb), h[0])) return odds(); return { shape: bark ? 'limb' : 'drum', taper: ra >= rb ? rb / ra - 1 : 1 - ra / rb }; }
  if (!zero[0] && !zero[1] && !zero[2]) return near(A, 1 / h[0] ** 2) && near(B, 1 / h[1] ** 2) && near(C, 1 / h[2] ** 2) ? { shape: bark ? 'knot' : 'ball', taper: 0 } : odds();
  const a = zero.indexOf(true); if (zero.filter(Boolean).length !== 1 || q.some((v, k) => k !== a && !near(v, 1 / h[k]! ** 2))) return odds(); if (a === 1) return { shape: bark ? 'limb' : 'drum', taper: 0 };
  // (its axis is another: turned so that it is y)
  const [x, y, z] = p.rot, n = (v: V3): V3 => [-v[0], -v[1], -v[2]]; return a === 0 ? { shape: 'drum', taper: 0, rot: [n(y), x, z], half: [h[1], h[0], h[2]] } : { shape: 'drum', taper: 0, rot: [x, z, n(y)], half: [h[0], h[2], h[1]] };
}
/** what a face is laid with, and how big its parts are */
const LAID: Record<string, { shape: string; a: number; b: number; gap: number; up: boolean }> = { roof: { shape: 'sheet', a: 1.0, b: 2.6, gap: 0.004, up: true }, stone: { shape: 'paver', a: 0.6, b: 0.6, gap: 0.006, up: true }, wood: { shape: 'board', a: 0.14, b: 1.9, gap: 0.005, up: true }, concrete: { shape: 'panel', a: 1.2, b: 1.0, gap: 0.010, up: false }, brick: { shape: 'panel', a: 0.6, b: 0.3, gap: 0.008, up: false } };
export function makeFine(b: Building): FineScene {
  const w = b.world, insts: Inst[] = [], q = rng(17), used = new Set<string>(), put = (i: Inst) => { insts.push(i); used.add(i.shape); }, plain = (p: Piece) => !(p.flags & (F.GLASS | F.WATER | F.FAR | F.GLOW | F.BEND | F.MESH)) && Math.max(...p.half) < 60;
  // ---- what stands, and what moves by itself
  const of = new Set(b.people.concat(b.player.body).flatMap(p => p.body)); for (const [list, moves] of [[w.pieces, false], [w.moving, true]] as const) for (const p of list) { if (!plain(p) || of.has(p) || (moves && p.at[1] < -90 && !p.rig)) continue; const s = shapeOf(p); if (!s) continue; p.fine = true; put({ shape: s.shape, piece: p, taper: s.taper, moves, ...(s.rot ? { rot: s.rot, half: s.half } : {}) }); }
  for (const p of new Set(b.people.concat(b.player.body))) { for (const q2 of p.body) q2.fine = true; put({ shape: 'body', piece: p.body[0]!, taper: 0, moves: true, body: p.body }); }
  // ---- skins
  const grid = new Map<string, Piece[]>(), key = (x: number, y: number, z: number) => `${Math.floor(x / 2)},${Math.floor(y / 2)},${Math.floor(z / 2)}`, solidAt = w.pieces.filter(p => plain(p) && !p.quad && !p.planes?.length && Math.max(...p.half) < 45); for (const p of solidAt) { const e = [0, 1, 2].map(k => Math.abs(p.rot[0][k]!) * p.half[0] + Math.abs(p.rot[1][k]!) * p.half[1] + Math.abs(p.rot[2][k]!) * p.half[2]), lo = p.at.map((v, k) => Math.floor((v - e[k]!) / 2)), hi = p.at.map((v, k) => Math.floor((v + e[k]!) / 2)); for (let x = lo[0]!; x <= hi[0]!; x++) for (let y = lo[1]!; y <= hi[1]!; y++) for (let z = lo[2]!; z <= hi[2]!; z++) { const k = `${x},${y},${z}`; let l = grid.get(k); if (!l) grid.set(k, l = []); l.push(p); } }
  const inside = (x: V3, not: Piece) => (grid.get(key(...x)) ?? []).some(p => p !== not && [0, 1, 2].every(k => Math.abs((x[0] - p.at[0]) * p.rot[k]![0] + (x[1] - p.at[1]) * p.rot[k]![1] + (x[2] - p.at[2]) * p.rot[k]![2]) < p.half[k]! + 0.001));
  for (const p of w.pieces) {
    const l = LAID[MATERIALS[p.mat]!.name]; if (!l || !plain(p) || p.quad || p.planes?.length || p.flags & F.FAR) continue;
    for (let a = 0; a < 3; a++) for (const s of [-1, 1]) {
      const n: V3 = p.rot[a]!.map(v => v * s) as V3, up = n[1] > 0.99, side = Math.abs(n[1]) < 0.01; if (l.up ? !up : !side) continue; const i = (a + 1) % 3, j = (a + 2) % 3, hi = p.half[i]!, hj = p.half[j]!; if (4 * hi * hj < 0.9 || Math.min(hi, hj) < 0.1) continue;
      const long = l.shape === 'board' ? hi < hj : false, [du, dv] = l.shape === 'board' ? (long ? [l.a, l.b] : [l.b, l.a]) : side && Math.abs(p.rot[i]![1]) > 0.9 ? [l.b, l.a] : [l.a, l.b], nu = Math.max(1, Math.round(2 * hi / du)), nv = Math.max(1, Math.round(2 * hj / dv)), su = hi / nu, sv = hj / nv;
      for (let u = 0; u < nu; u++) for (let v = 0; v < nv; v++) { const cu = -hi + su * (2 * u + 1), cv = -hj + sv * (2 * v + 1), at = [0, 1, 2].map(k => p.at[k]! + n[k]! * (p.half[a]! + 0.002) + p.rot[i]![k]! * cu + p.rot[j]![k]! * cv) as V3; let hidden = 0; for (const [x, y] of [[0, 0], [0.8, 0.8], [-0.8, 0.8], [0.8, -0.8], [-0.8, -0.8]] as const) if (inside([0, 1, 2].map(k => at[k]! + n[k]! * 0.012 + p.rot[i]![k]! * su * x + p.rot[j]![k]! * sv * y) as V3, p)) hidden++; if (hidden >= 1) continue;
        // its own frame: y out of the face; x and z along it
        const X = p.rot[i]!, Z: V3 = [X[1] * n[2] - X[2] * n[1], X[2] * n[0] - X[0] * n[2], X[0] * n[1] - X[1] * n[0]], t = 1 - 0.10 * q(); put({ shape: l.shape + Math.floor(q() * 3), piece: p, at, rot: [X, n, Z], half: [su - l.gap / 2, 0.006, sv - l.gap / 2], taper: 0, tint: [t, t, t], moves: false }); }
    }
  }
  return { insts, shapes: [...used] };
}
/** the cars of the study: two, on the lower roof */
export type { World };
