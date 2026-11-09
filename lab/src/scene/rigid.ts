// RIGID BODIES: things that fall, rest, roll, float and are kicked. A body is some of the world's pieces and moves them as one. One rule for all of them, in small steps
// (extended position based dynamics): weight, the air's push on what is dry, the water's on what is wet (each part is lifted by the water it displaces), then contacts:
// points of one against the shape of the other, pushed apart, held by friction. A body at rest sleeps till something moves near it. No clock, no dice.
import { type V3, type Rot, add, sub, mul, dot, cross } from '../math';
import { F, boundsOf, type Piece, type World } from './world';
import type { Air } from './air';
import type { Person } from './people';
import { wave } from './waves';
export interface Pool { rect: [number, number, number, number]; top: number; bed: number }
type Quat = [number, number, number, number];
/** a shape: what of a piece is solid. Where it is may change under it */
interface Shape { at: V3; rot: Rot; half: V3; quad?: [number, number, number, number]; planes?: [V3, number][] }
const axes = (q: Quat): Rot => { const [x, y, z, w] = q; return [[1 - 2 * (y * y + z * z), 2 * (x * y + z * w), 2 * (x * z - y * w)], [2 * (x * y - z * w), 1 - 2 * (x * x + z * z), 2 * (y * z + x * w)], [2 * (x * z + y * w), 2 * (y * z - x * w), 1 - 2 * (x * x + y * y)]]; };
const quatOf = (r: Rot): Quat => { const [a, b, c] = r, t = a[0] + b[1] + c[2]; if (t > 0) { const s = Math.sqrt(t + 1) * 2; return [(b[2] - c[1]) / s, (c[0] - a[2]) / s, (a[1] - b[0]) / s, s / 4]; } if (a[0] > b[1] && a[0] > c[2]) { const s = Math.sqrt(1 + a[0] - b[1] - c[2]) * 2; return [s / 4, (b[0] + a[1]) / s, (c[0] + a[2]) / s, (b[2] - c[1]) / s]; } if (b[1] > c[2]) { const s = Math.sqrt(1 + b[1] - a[0] - c[2]) * 2; return [(b[0] + a[1]) / s, s / 4, (c[1] + b[2]) / s, (c[0] - a[2]) / s]; } const s = Math.sqrt(1 + c[2] - a[0] - b[1]) * 2; return [(c[0] + a[2]) / s, (c[1] + b[2]) / s, s / 4, (a[1] - b[0]) / s]; };
const out = (r: Rot, v: V3): V3 => [r[0][0] * v[0] + r[1][0] * v[1] + r[2][0] * v[2], r[0][1] * v[0] + r[1][1] * v[1] + r[2][1] * v[2], r[0][2] * v[0] + r[1][2] * v[1] + r[2][2] * v[2]], into = (r: Rot, v: V3): V3 => [dot(r[0], v), dot(r[1], v), dot(r[2], v)];
const turned = (q: Quat, w: V3): Quat => { const [x, y, z, s] = q, a = x + 0.5 * (w[0] * s + w[1] * z - w[2] * y), b = y + 0.5 * (w[1] * s + w[2] * x - w[0] * z), c = z + 0.5 * (w[2] * s + w[0] * y - w[1] * x), d = s - 0.5 * (w[0] * x + w[1] * y + w[2] * z), l = 1 / Math.sqrt(a * a + b * b + c * c + d * d); return [a * l, b * l, c * l, d * l]; };
/** how deep x is in a shape (under 0: inside), and which way is out */
const N: V3 = [0, 0, 0]; function depth(s: Shape, x: V3): number {
  const l = into(s.rot, sub(x, s.at)); let d = -1e9, a = 0, b = 0, c = 0; for (let k = 0; k < 3; k++) { const e = Math.abs(l[k]!) - s.half[k]!; if (e > d) { d = e; a = k === 0 ? Math.sign(l[0]) : 0; b = k === 1 ? Math.sign(l[1]) : 0; c = k === 2 ? Math.sign(l[2]) : 0; } } 
  if (s.quad) { const [A, B, C, D] = s.quad, gx = 2 * A * l[0], gy = 2 * B * l[1] + D, gz = 2 * C * l[2], g = Math.sqrt(gx * gx + gy * gy + gz * gz); if (g > 1e-9) { const e = (A * l[0] * l[0] + B * l[1] * l[1] + C * l[2] * l[2] + D * l[1] - 1) / g * (1 + 0); if (e > d) { d = e; a = gx / g; b = gy / g; c = gz / g; } } }
  if (s.planes) for (const [n, w] of s.planes) { const e = n[0] * l[0] + n[1] * l[1] + n[2] * l[2] - w; if (e > d) { d = e; a = n[0]; b = n[1]; c = n[2]; } }
  const n = out(s.rot, [a, b, c]); N[0] = n[0]; N[1] = n[1]; N[2] = n[2]; return d;
}
/** points over a shape that feel for others */
function feelers(s: Shape): V3[] {
  const p: V3[] = [], [x, y, z] = s.half, place = (v: V3) => add(s.at, out(s.rot, v)), q = s.quad; if (!q) { for (let i = 0; i < 8; i++) p.push(place([i & 1 ? x : -x, i & 2 ? y : -y, i & 4 ? z : -z])); for (const m of s.planes ?? []) void m; return s.planes ? p.filter(v => s.planes!.every(([n, w]) => dot(n, into(s.rot, sub(v, s.at))) < w + 0.02)) : p; }
  const round = [0, 1, 2].filter(k => q[k] !== 0); { const k = [0, 1, 2].find(m => !round.includes(m)) ?? 1, a = (k + 1) % 3, b = (k + 2) % 3; for (const e of [-1, 1]) for (let i = 0; i < 8; i++) { const v: V3 = [0, 0, 0]; v[k] = e * s.half[k]!; v[a] = s.half[a]! * Math.cos(i * Math.PI / 4); v[b] = s.half[b]! * Math.sin(i * Math.PI / 4); p.push(place(v)); } } return p;
}
export class Body {
  x: V3; q: Quat; v: V3 = [0, 0, 0]; w: V3 = [0, 0, 0]; r: Rot; readonly m: number; readonly inv: V3; readonly reach: number; asleep = false; parked = false; still = 0; wet = 0;
  /** in the body's own frame, about its weight's middle */ readonly parts: { p: Piece; at: V3; rot: Rot }[]; readonly shapes: Shape[]; readonly points: V3[]; readonly cells: { at: V3; V: number; r: number }[];
  /** the shapes where they are now */ readonly now: Shape[]; px: V3 = [0, 0, 0]; pr: Rot = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]; pq: Quat = [0, 0, 0, 1]; force: V3 = [0, 0, 0]; torque: V3 = [0, 0, 0]; lag = 0;
  /** `dense` kg a cubic metre; `low`: its weight sits this much under its middle; `solid`: the parts that touch and displace (all, if not said) */
  constructor(pieces: Piece[], at: V3, rot: Rot, o: { dense: number; low?: number; solid?: Piece[]; rough?: number }) {
    const rel = (p: Piece, c: V3): Shape => ({ at: into(rot, sub(p.at, c)), rot: [into(rot, p.rot[0]), into(rot, p.rot[1]), into(rot, p.rot[2])], half: [...p.half] as V3, quad: p.quad, planes: p.planes }), solid = o.solid ?? pieces; this.rough = o.rough ?? 0.5; let V = 0, c: V3 = [0, 0, 0]; const raw: { at: V3; V: number; r: number }[] = [];
    // what displaces water, and has weight: eight of each part
    for (const p of solid) { const s = rel(p, at), full = 8 * s.half[0] * s.half[1] * s.half[2], n = s.quad ? [0, 1, 2].filter(k => s.quad![k] !== 0).length : 0, vol = full * (n >= 3 ? 0.5236 : n === 2 ? 0.7854 : 1), k = n >= 3 ? 0.42 : n === 2 ? 0.46 : 0.5; for (let i = 0; i < 8; i++) { const v = vol / 8, q = add(s.at, out(s.rot, [(i & 1 ? k : -k) * s.half[0], (i & 2 ? k : -k) * s.half[1], (i & 4 ? k : -k) * s.half[2]])); raw.push({ at: q, V: v, r: Math.cbrt(v * 0.2387) }); V += v; c = add(c, mul(q, v)); } }
    c = mul(c, 1 / V); c[1] -= o.low ?? 0; this.m = V * o.dense; const heart = add(at, out(rot, c)); this.x = heart; this.q = quatOf(rot); this.r = rot; this.cells = raw.map(q => ({ ...q, at: sub(q.at, c) })); let I: V3 = [0, 0, 0]; for (const q of this.cells) { const m = q.V * o.dense, e = 0.4 * q.r * q.r * 2.2; I = add(I, [m * (q.at[1] ** 2 + q.at[2] ** 2 + e), m * (q.at[0] ** 2 + q.at[2] ** 2 + e), m * (q.at[0] ** 2 + q.at[1] ** 2 + e)]); } this.inv = [1 / I[0], 1 / I[1], 1 / I[2]];
    this.parts = pieces.map(p => { const s = rel(p, heart); return { p, at: s.at, rot: s.rot }; }); this.shapes = solid.map(p => rel(p, heart)); this.now = this.shapes.map(s => ({ ...s })); const all3 = (s: Shape) => !!s.quad && s.quad[0] !== 0 && s.quad[1] !== 0 && s.quad[2] !== 0; this.rounds = this.shapes.flatMap((s, i) => all3(s) ? [i] : []); this.points = this.shapes.filter(s => !all3(s)).flatMap(feelers); this.reach = Math.max(...this.points.map(p => Math.hypot(...p)), ...this.rounds.map(i => Math.hypot(...this.shapes[i]!.at) + Math.max(...this.shapes[i]!.half))); this.pose();
  }
  readonly rough: number; /** those of its shapes that are round all over: they touch where they face what they touch */ readonly rounds: number[];
  wake() { this.asleep = false; this.still = 0; }
  put(x: V3, v: V3, w: V3) { this.x = [...x]; this.v = [...v]; this.w = [...w]; this.parked = false; this.wake(); this.pose(); }
  /** its turn, with inertia */ spun(t: V3): V3 { const l = into(this.r, t); return out(this.r, [l[0] * this.inv[0], l[1] * this.inv[1], l[2] * this.inv[2]]); }
  pose() { this.r = axes(this.q); for (const q of this.parts) { q.p.at = add(this.x, out(this.r, q.at)); q.p.rot = [out(this.r, q.rot[0]), out(this.r, q.rot[1]), out(this.r, q.rot[2])]; } this.shapesNow(); }
  shapesNow() { this.now.forEach((s, i) => { const b = this.shapes[i]!; s.at = add(this.x, out(this.r, b.at)); s.rot = [out(this.r, b.rot[0]), out(this.r, b.rot[1]), out(this.r, b.rot[2])]; }); }
}
interface Touch { a: Body; /** a point of a (or, if `rev`, of the world, against a's shape) */ p: V3; s: Shape; b: Body | null; rev: boolean; round?: number; n: V3; push: number; /** how fast it came in */ lagV?: number }
const STEPS = 6, G = 1;
export class Bodies {
  readonly all: Body[] = []; private grid = new Map<number, Piece[]>(); private spare: Body[] = []; ms = 0;
  constructor(private w: World, private pools: Pool[], private people: Person[], private doors: { leaf: Piece[] }[]) {}
  private key(x: number, y: number, z: number) { return (x + 256) + 512 * ((y + 64) + 256 * (z + 256)); }
  /** what stands: call when the scene is made */
  settle() { const mine = new Set(this.all.flatMap(b => b.parts.map(q => q.p))); for (const p of this.w.pieces) { if (p.flags & (F.WATER | F.FAR) || mine.has(p)) continue; const b = boundsOf(p); if (b.mx[0] < -30 || b.mn[0] > 30 || b.mx[2] < -25 || b.mn[2] > 25 || b.mx[1] < -2) continue; for (let x = Math.floor(Math.max(-30, b.mn[0]) / G); x <= Math.floor(Math.min(30, b.mx[0]) / G); x++) for (let y = Math.floor(Math.max(-2, b.mn[1]) / G); y <= Math.floor(b.mx[1] / G); y++) for (let z = Math.floor(Math.max(-25, b.mn[2]) / G); z <= Math.floor(Math.min(25, b.mx[2]) / G); z++) { const k = this.key(x, y, z), l = this.grid.get(k); if (l) l.push(p); else this.grid.set(k, [p]); } } return this; }
  add(b: Body, parked = false) { this.all.push(b); if (parked) { b.parked = true; this.spare.push(b); } return b; }
  /** send one of those kept for it */
  toss(x: V3, v: V3) { const b = this.spare.shift(); if (!b) return; this.spare.push(b); b.put(x, v, [v[2] * 0.6, 1.5, -v[0] * 0.6]); }
  private near(b: Body, far: number): Piece[] { const out = new Set<Piece>(), r = b.reach + far; for (let x = Math.floor((b.x[0] - r) / G); x <= Math.floor((b.x[0] + r) / G); x++) for (let y = Math.floor((b.x[1] - r) / G); y <= Math.floor((b.x[1] + r) / G); y++) for (let z = Math.floor((b.x[2] - r) / G); z <= Math.floor((b.x[2] + r) / G); z++) for (const p of this.grid.get(this.key(x, y, z)) ?? []) { const m = boundsOf(p); if (m.mn[0] < b.x[0] + r && m.mx[0] > b.x[0] - r && m.mn[1] < b.x[1] + r && m.mx[1] > b.x[1] - r && m.mn[2] < b.x[2] + r && m.mx[2] > b.x[2] - r) out.add(p); } for (const d of this.doors) { const p = d.leaf[0]!; if (Math.hypot(p.at[0] - b.x[0], p.at[1] - b.x[1], p.at[2] - b.x[2]) < Math.hypot(...p.half) + r) out.add(p); } return [...out]; }
  step(dt: number, t: number, air: Air) {
    const t0 = performance.now(), live: Body[] = []; for (const b of this.all) { if (b.parked) continue; if (b.asleep) for (const p of this.people) if (Math.hypot(p.vel[0], p.vel[2]) > 0.2 && Math.hypot(p.pos[0] - b.x[0], p.pos[2] - b.x[2]) < b.reach + 0.55 && b.x[1] > p.pos[1] - 0.2 && b.x[1] < p.pos[1] + 1.9) { b.wake(); break; } if (!b.asleep) live.push(b); } if (!live.length) { this.ms = 0; return; }
    // who may touch whom this tick
    const touches: Touch[] = [], men: { b: Body; who: Person }[] = []; for (const a of live) {
      const far = 0.03 + Math.hypot(...a.v) * dt, R = a.reach + far; for (const s of this.near(a, far)) { for (const round of a.rounds) touches.push({ a, p: [0, 0, 0], s, b: null, rev: false, round, n: [0, 1, 0], push: 0 }); for (const p of a.points) touches.push({ a, p, s, b: null, rev: false, n: [0, 1, 0], push: 0 }); if (!s.quad && Math.max(...s.half) < 40) for (let i = 0; i < 8; i++) { const c = add(s.at, out(s.rot, [i & 1 ? s.half[0] : -s.half[0], i & 2 ? s.half[1] : -s.half[1], i & 4 ? s.half[2] : -s.half[2]])); if (Math.hypot(c[0] - a.x[0], c[1] - a.x[1], c[2] - a.x[2]) < R) for (const m of a.now) touches.push({ a, p: c, s: m, b: null, rev: true, n: [0, 1, 0], push: 0 }); } }
      for (const b of this.all) { if (b === a || b.parked || (!b.asleep && this.all.indexOf(b) < this.all.indexOf(a))) continue; if (Math.hypot(a.x[0] - b.x[0], a.x[1] - b.x[1], a.x[2] - b.x[2]) > R + b.reach + 0.02) continue; if (b.asleep && (Math.hypot(...a.v) > 0.12 || Math.hypot(...a.w) > 0.6)) { b.wake(); if (!live.includes(b)) live.push(b); } for (const s of b.now) { for (const round of a.rounds) touches.push({ a, p: [0, 0, 0], s, b, rev: false, round, n: [0, 1, 0], push: 0 }); for (const p of a.points) touches.push({ a, p, s, b, rev: false, n: [0, 1, 0], push: 0 }); } for (const s of a.now) for (const p of b.points) touches.push({ a: b, p, s, b: a, rev: false, n: [0, 1, 0], push: 0 }); }
      for (const who of this.people) if (Math.hypot(who.pos[0] - a.x[0], who.pos[2] - a.x[2]) < R + 0.4 && a.x[1] > who.pos[1] - 0.3 && a.x[1] < who.pos[1] + 2) men.push({ b: a, who });
      this.medium(a, dt, t, air);
    }
    const h = dt / STEPS; for (let k = 0; k < STEPS; k++) {
      for (const b of live) { b.px = b.x; b.pr = b.r; b.pq = b.q; b.v = [b.v[0] + b.force[0] * h, b.v[1] + b.force[1] * h, b.v[2] + b.force[2] * h]; b.w = add(b.w, mul(b.torque, h)); const sp = Math.hypot(...b.w); if (sp > 40) b.w = mul(b.w, 40 / sp); b.x = add(b.x, mul(b.v, h)); b.q = turned(b.q, mul(b.w, h)); b.r = axes(b.q); b.shapesNow(); }
      for (const c of touches) this.solve(c);
      for (const { b, who } of men) { const dx = b.x[0] - who.pos[0], dz = b.x[2] - who.pos[2], d = Math.hypot(dx, dz) || 1e-6, need = 0.28 + b.reach * 0.7 - d; if (need > 0 && b.x[1] > who.pos[1] - 0.1 && b.x[1] < who.pos[1] + 1.8) { b.x = [b.x[0] + dx / d * need, b.x[1], b.x[2] + dz / d * need]; } }
      for (const b of live) { b.v = mul(sub(b.x, b.px), 1 / h); const [ax, ay, az, aw] = b.q, [bx, by, bz, bw] = b.pq, x = -aw * bx + ax * bw - ay * bz + az * by, y = -aw * by + ay * bw - az * bx + ax * bz, z = -aw * bz + az * bw - ax * by + ay * bx, w = aw * bw + ax * bx + ay * by + az * bz, s = (w < 0 ? -2 : 2) / h; b.w = [x * s, y * s, z * s]; }
      // a little of what came in goes out again; what rolls is slowed
      for (const c of touches) if (c.push > 0 && !c.rev) { const a = c.a, r = out(a.r, c.p), va = add(a.v, cross(a.w, r)), vn = dot(va, c.n) - (c.b ? dot(add(c.b.v, cross(c.b.w, sub(add(a.x, r), c.b.x))), c.n) : 0); if (vn > 0.0 && c.push > 0) { /* pushed out: not faster than it should bounce */ const most = 0.25 * Math.max(0, -c.lagV! - 0.4); const cut = Math.max(0, vn - Math.max(most, 0.02)), w = 1 / a.m; if (cut > 0 && !c.b) a.v = sub(a.v, mul(c.n, cut * w * a.m * 0.5)); } if (c.push > 0) { const roll = Math.exp(-h * (c.b ? 0.3 : 1.4)); c.a.w = mul(c.a.w, roll); } c.push = 0; }
    }
    for (const b of live) { const speed = Math.hypot(...b.v) + b.reach * Math.hypot(...b.w); b.still = speed < 0.06 && b.wet === 0 ? b.still + 1 : 0; if (b.still > 40) { b.asleep = true; b.v = [0, 0, 0]; b.w = [0, 0, 0]; } if (b.x[1] < -12) { b.parked = true; b.x = [0, -60, 0]; } b.pose(); } this.ms = performance.now() - t0;
  }
  private solve(c: Touch) {
    const a = c.a; if (c.rev) { const d = depth(c.s, c.p); if (d >= 0) return; const n: V3 = [-N[0], -N[1], -N[2]], r = sub(c.p, a.x), rn = cross(r, n), w = 1 / a.m + dot(rn, a.spun(rn)), l = -d / w; this.move(a, r, mul(n, l), 1); const was = add(a.px, out(a.pr, into(a.r, r))); this.rub(a, r, sub(c.p, was), n, l, null, r, null); c.push = l; return; }
    if (c.round !== undefined) { const m = a.now[c.round]!, h = m.half, near = depth(c.s, m.at); if (near > Math.max(h[0], h[1], h[2])) return; const l = into(m.rot, [-N[0], -N[1], -N[2]]), g = Math.sqrt((h[0] * l[0]) ** 2 + (h[1] * l[1]) ** 2 + (h[2] * l[2]) ** 2) || 1e-9, at = add(m.at, out(m.rot, [h[0] * h[0] * l[0] / g, h[1] * h[1] * l[1] / g, h[2] * h[2] * l[2] / g])); c.p = into(a.r, sub(at, a.x)); }
    const r = out(a.r, c.p), x = add(a.x, r), d = depth(c.s, x); if (d >= 0 || d < -0.25) return; const n: V3 = [N[0], N[1], N[2]], b = c.b, rn = cross(r, n); let w = 1 / a.m + dot(rn, a.spun(rn)), rb: V3 = [0, 0, 0]; if (b) { rb = sub(x, b.x); const q = cross(rb, n); w += 1 / b.m + dot(q, b.spun(q)); } const l = -d / w; c.lagV = dot(add(a.v, cross(a.w, r)), n);
    this.move(a, r, mul(n, l), 1); if (b) this.move(b, rb, mul(n, l), -1); const wasA = add(a.px, out(a.pr, c.p)), slipA = sub(add(a.x, out(a.r, c.p)), wasA); let slip = slipA; if (b) { const lb = into(b.r, rb), wasB = add(b.px, out(b.pr, lb)); slip = sub(slipA, sub(add(b.x, rb), wasB)); } this.rub(a, r, slip, n, l, b, rb, b); c.n = n; c.push = l;
  }
  /** held by friction: what slid along in this step is undone, as far as the push allows */
  private rub(a: Body, r: V3, slip: V3, n: V3, l: number, b: Body | null, rb: V3, _b: Body | null) { const t = sub(slip, mul(n, dot(slip, n))), s = Math.hypot(...t); if (s < 1e-9) return; const u = mul(t, 1 / s), ru = cross(r, u); let w = 1 / a.m + dot(ru, a.spun(ru)); if (b) { const q = cross(rb, u); w += 1 / b.m + dot(q, b.spun(q)); } const k = Math.min(s / w, (b ? Math.min(a.rough, b.rough) : a.rough) * l); this.move(a, r, mul(u, -k), 1); if (b) this.move(b, rb, mul(u, -k), -1); }
  private move(b: Body, r: V3, p: V3, sign: number) { b.x = [b.x[0] + sign * p[0] / b.m, b.x[1] + sign * p[1] / b.m, b.x[2] + sign * p[2] / b.m]; const w = b.spun(cross(r, p)); b.q = turned(b.q, [sign * w[0], sign * w[1], sign * w[2]]); b.r = axes(b.q); }
  private sea = new Float64Array(4);
  /** weight, air and water, as accelerations for the tick */
  private medium(b: Body, dt: number, t: number, air: Air) {
    const wind = air.at(b.x), pool = this.pools.find(p => b.x[0] > p.rect[0] - b.reach && b.x[0] < p.rect[2] + b.reach && b.x[2] > p.rect[1] - b.reach && b.x[2] < p.rect[3] + b.reach && b.x[1] - b.reach < p.top + 0.1); let fx = 0, fy = -9.81 * b.m, fz = 0, T: V3 = [0, 0, 0], carried = 0, wet = 0, held = 0;
    for (const c of b.cells) {
      const off = out(b.r, c.at), x = add(b.x, off), v = add(b.v, cross(b.w, off)), A = Math.PI * c.r * c.r; let in_ = 0, f: V3 = [0, 0, 0];
      if (pool && x[0] > pool.rect[0] && x[0] < pool.rect[2] && x[2] > pool.rect[1] && x[2] < pool.rect[3]) { wave(x[0], x[2], t, this.sea); const y = pool.top + this.sea[0]!, d = Math.max(0, Math.min(2 * c.r, y - (x[1] - c.r))); if (d > 0) { in_ = d / (2 * c.r); const V = Math.PI * d * d * (3 * c.r - d) / 3, up = 1000 * 9.81 * V, [cx, cz] = stir(pool, x[0], x[2]), rel: V3 = [cx - v[0], this.sea[3]! - v[1], cz - v[2]], sp = Math.hypot(...rel), k = in_ * 1000 * A * (0.45 * sp + 0.09); f = [-this.sea[1]! * up + rel[0] * k, up + rel[1] * k * 1.6, -this.sea[2]! * up + rel[2] * k]; carried += 500 * V; held += k; } }
      wet += in_; const rw = sub(wind, v), q = (1 - in_) * 0.5 * 1.2 * 0.9 * A * Math.hypot(...rw) * 0.55; f = [f[0] + rw[0] * q, f[1] + rw[1] * q, f[2] + rw[2] * q]; fx += f[0]; fy += f[1]; fz += f[2]; T = add(T, cross(off, f));
    }
    const m = b.m + carried, most = 60; b.wet = wet; const k = Math.min(1, most * b.m / Math.max(1e-9, Math.hypot(fx, fy, fz))); b.force = [fx / m * k, fy / m * k, fz / m * k]; const spin = mul(b.spun(T), b.m / m); b.torque = sub(spin, mul(b.w, wet > 0 ? 2.5 * wet / b.cells.length + 0.2 : 0.25)); void held; void dt;
  }
}
/** what stirs the pool: water comes in at the walls and goes slowly round */
const stir = (p: Pool, x: number, z: number): [number, number] => { const [x0, z0, x1, z1] = p.rect, u = (x - (x0 + x1) / 2) / ((x1 - x0) / 2), v = (z - (z0 + z1) / 2) / ((z1 - z0) / 2), k = 0.14 * Math.max(0, 1 - 0.35 * (u * u + v * v)); return [-v * k, u * k]; };
