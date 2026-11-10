// THINGS THAT BUILDERS MAKE: pavers, boards, bricks, copings, rails, cushions. Each from a seed, as a solid worn and weathered, then made as light as it can be without losing a quarter of a millimetre.
import { rng } from '../math';
import { normalsOf, split4, type Mesh } from './mesh';
import { nets, noise3, fbm3 } from './shapes';
import { buildDag, cutAt } from './dag';

const clamp01 = (x: number) => Math.max(0, Math.min(1, x)), step = (a: number, b: number, x: number) => { const t = clamp01((x - a) / (b - a)); return t * t * (3 - 2 * t); };
const smin = (a: number, b: number, k: number) => { const h = Math.max(k - Math.abs(a - b), 0) / k; return Math.min(a, b) - h * h * k * 0.25; }, smax = (a: number, b: number, k: number) => -smin(-a, -b, k);
/** a box about the origin, its edges round by r */
const box = (x: number, y: number, z: number, hx: number, hy: number, hz: number, r: number) => { const a = Math.abs(x) - hx + r, b = Math.abs(y) - hy + r, c = Math.abs(z) - hz + r; return Math.min(Math.max(a, b, c), 0) + Math.hypot(Math.max(a, 0), Math.max(b, 0), Math.max(c, 0)) - r; };
const tube = (x: number, y: number, z: number, r: number, h: number, e: number) => { const a = Math.hypot(x, z) - r + e, b = Math.abs(y) - h + e; return Math.min(Math.max(a, b), 0) + Math.hypot(Math.max(a, 0), Math.max(b, 0)) - e; };
type Field = (x: number, y: number, z: number) => number;
/** red, green, blue, and how open (1 = not at all in a crevice) */
type Paint = (x: number, y: number, z: number, nx: number, ny: number, nz: number) => [number, number, number, number];
export interface Made { name: string; mesh: Mesh }

/** the finest cut that stands within e of it, as a mesh of its own */
export function lighten(m: Mesh, e: number): Mesh {
  const d = buildDag(m), cs = cutAt(d, e), to = new Map<number, number>(), ids: number[] = [], tri: number[] = []; for (const c of cs) for (const v of c.tri) { let k = to.get(v); if (k === undefined) { k = ids.length; to.set(v, k); ids.push(v); } tri.push(k); }
  const P = d.pool, pos = new Float32Array(ids.length * 3), nrm = new Float32Array(ids.length * 3), col = new Uint8Array(ids.length * 4); ids.forEach((v, i) => { for (let k = 0; k < 3; k++) { pos[i * 3 + k] = P.pos[v * 3 + k]!; nrm[i * 3 + k] = P.nrm[v * 3 + k]!; } for (let k = 0; k < 4; k++) col[i * 4 + k] = Math.max(0, Math.min(255, Math.round(255 * Math.sqrt(Math.max(0, P.col[v * 4 + k]!))))); });
  return { pos, nrm, col, tri: new Uint32Array(tri) };
}
/** a solid's surface: its net, parted `finer` times and drawn onto it, then moved out by `rise`, and painted */
export function solid(f: Field, h: [number, number, number], cell: number, finer: number, rise: Field, paint: Paint, keep = 0.0004): Mesh {
  const g = cell * 1.3; let m = nets(f, [-h[0] - g, -h[1] - g, -h[2] - g], [h[0] + g, h[1] + g, h[2] + g], cell); const e = 0.0005;
  const onto = (p: Float32Array, times: number) => { for (let i = 0; i < p.length; i += 3) for (let k = 0; k < times; k++) { const x = p[i]!, y = p[i + 1]!, z = p[i + 2]!, d = f(x, y, z); let gx = f(x + e, y, z) - d, gy = f(x, y + e, z) - d, gz = f(x, y, z + e) - d; const l = Math.hypot(gx, gy, gz); if (l < 1e-10) break; gx /= l; gy /= l; gz /= l; const s = Math.max(-cell, Math.min(cell, d / Math.max(l / e, 0.4) * 0.9)); p[i] = x - gx * s; p[i + 1] = y - gy * s; p[i + 2] = z - gz * s; } };
  onto(m.pos, 3); for (let k = 0; k < finer; k++) { m = split4(m); onto(m.pos, 1); }
  const n = normalsOf(m.pos, m.tri), nv = m.pos.length / 3, pos = new Float32Array(m.pos.length), col = new Uint8Array(nv * 4);
  for (let i = 0; i < nv; i++) { const x = m.pos[i * 3]!, y = m.pos[i * 3 + 1]!, z = m.pos[i * 3 + 2]!, r = rise(x, y, z), c = paint(x, y, z, n[i * 3]!, n[i * 3 + 1]!, n[i * 3 + 2]!); for (let k = 0; k < 3; k++) pos[i * 3 + k] = m.pos[i * 3 + k]! + n[i * 3 + k]! * r; for (let k = 0; k < 4; k++) col[i * 4 + k] = 255 * clamp01(c[k]!); }
  const out: Mesh = { pos, nrm: normalsOf(pos, m.tri), col, tri: m.tri }; return keep > 0 ? lighten(out, keep) : out;
}
const grey = (v: number, w = 0, a = 1): [number, number, number, number] => [v, v * (1 - 0.03 * w), v * (1 - 0.09 * w), a];

/** how fine: metres between a net's points before it is parted */
export const FINE_MADE = { k: 1 };
/** stone, 0.6 m square with its joint; its top at 0 */
export function paver(seed: number): Mesh {
  const r = rng(seed), o = r() * 40, hx = 0.2965, t = 0.014, chip = r() < 0.5, cx = hx * (r() < 0.5 ? -1 : 1), cz = hx * (r() < 0.5 ? -1 : 1), cr = 0.015 + 0.03 * r(), crack = r() < 0.25, ca = r() * 3.14, cd = (r() - 0.5) * 0.25;
  const f: Field = (x, y, z) => { const wear = 0.0025 * (0.5 + fbm3(x * 9 + o, 1, z * 9, 3)); let d = box(x, y + t, z, hx, t, hx, 0.005) + wear * step(-0.02, 0.004, y) * step(hx - 0.03, hx, Math.max(Math.abs(x), Math.abs(z))); if (chip) d = smax(d, cr - Math.hypot(x - cx, (y - 0.004) * 1.6, z - cz) + 0.006 * noise3(x * 70 + o, y * 70, z * 70), 0.003); if (crack) { const q = Math.abs(x * Math.cos(ca) + z * Math.sin(ca) - cd - 0.02 * fbm3(x * 12 + o, 3, z * 12, 3)); d = smax(d, 0.0012 - q + Math.max(0, -y - 0.004) * 0.4, 0.001); } return d; };
  return solid(f, [hx, t * 2, hx], 0.013 * FINE_MADE.k, 2, (x, y, z) => y < -0.01 ? 0 : 0.0007 * fbm3(x * 26 + o, 2, z * 26, 3) + 0.0005 * noise3(x * 110, 5, z * 110 + o) - 0.0012 * step(0.36, 0.55, noise3(x * 60 + o, 7, z * 60)),
    (x, y, z, _a, ny) => { const b = fbm3(x * 3 + o, 5, z * 3, 3), s = noise3(x * 150 + o, 1, z * 150), p = step(0.36, 0.55, noise3(x * 60 + o, 7, z * 60)), rim = step(hx - 0.035, hx, Math.max(Math.abs(x), Math.abs(z))), v = 0.83 + 0.09 * b + 0.05 * s - 0.15 * p - 0.10 * rim - 0.25 * (1 - clamp01(ny)); return grey(v, 1.0 + 0.6 * b, 1 - 0.4 * p); });
}
/** a deck's board, along x: grooves, grain, screws sunk where the joists are; its top at 0 */
export function board(seed: number, len = 1.92): Mesh {
  const r = rng(seed), o = r() * 60, hx = len / 2 - 0.0015, hz = 0.0435, t = 0.011, screws: [number, number][] = []; for (let i = 0; i < 4; i++) for (const s of [-1, 1]) screws.push([-len * 0.375 + i * len * 0.25 + (r() - 0.5) * 0.006, s * 0.026 + (r() - 0.5) * 0.003]);
  const f: Field = (x, y, z) => { let d = box(x, y + t, z, hx, t, hz, 0.004); const gz = Math.abs(((z + 0.1) / 0.0145 + 0.5) % 1 - 0.5) * 0.0145; if (Math.abs(z) < 0.034) d = smax(d, 0.0030 - Math.hypot(gz, y + 0.0004), 0.0012); for (const [sx, sz] of screws) if (Math.abs(x - sx) < 0.01) d = smax(d, 0.0046 - Math.hypot(x - sx, (y + 0.0006) * 1.4, z - sz), 0.0006); return d; };
  return solid(f, [hx, t * 2, hz], 0.0075 * FINE_MADE.k, 1, (x, y, z) => y < -0.018 ? 0 : 0.00045 * fbm3(x * 6 + o, y * 40, z * 160, 3) + 0.0004 * step(0.3, 0.6, noise3(x * 3 + o, 1, z * 90)),
    (x, y, z) => { const g = fbm3(x * 4 + o, y * 60, z * 140, 4), b = noise3(x * 1.1 + o, 0, z * 9), k = step(0.62, 0.8, noise3(x * 5 + o, 9, z * 22)), sc = screws.some(([a, c]) => Math.hypot(x - a, z - c) < 0.0045) ? 1 : 0, v = 0.80 + 0.22 * g + 0.10 * b - 0.35 * k; return sc ? [0.75, 0.76, 0.78, 0.8] : [v, v * 0.90, v * 0.78, 1]; });
}
/** a brick's face, painted: 21 by 6.5 cm, 2.2 proud of the wall; it faces +z, its back at 0 */
export function brick(seed: number, hx = 0.105): Mesh {
  const r = rng(seed), o = r() * 90, tw = (r() - 0.5) * 0.004, flake = r() < 0.35 ? r() : 0;
  const f: Field = (x, y, z) => { const w = 0.0016 * fbm3(x * 30 + o, y * 30, z * 30, 3); return box(x, y, z - 0.011 - tw * x, hx, 0.0325, 0.011, 0.0055) + w + 0.004 * step(0.24, 0.42, noise3(x * 16 + o, y * 22, 3)) * step(0.012, 0.024, z) * 0.5; };
  const bare = (x: number, y: number) => flake ? step(0.22, 0.28, fbm3(x * 14 + o, y * 20, 4, 3) + 0.14 * flake) : 0;
  return solid(f, [hx + 0.005, 0.036, 0.026], 0.0062 * FINE_MADE.k, 1, (x, y, z) => z < 0.004 ? 0 : 0.0004 * noise3(x * 160 + o, y * 160, z * 160) - 0.0006 * bare(x, y),
    (x, y, z) => { const b = bare(x, y), v = 0.95 + 0.04 * fbm3(x * 20 + o, y * 20, z * 20, 2); return [v - 0.2 * b, v * 0.99 - 0.42 * b, v * 0.97 - 0.5 * b, 1 - 0.3 * step(0.026, 0.0325, Math.abs(y))]; });
}
/** cast stone on a wall's top, 0.9 m of it along x: it slopes both ways and drips; its underside at 0 */
export function coping(seed: number, len = 0.9, deep = 0.16, thick = 0.055): Mesh {
  const r = rng(seed), o = r() * 70, hx = len / 2 - 0.003;
  const f: Field = (x, y, z) => { let d = box(x, y - thick / 2, z, hx, thick / 2, deep, 0.008); d = smax(d, y - thick + 0.10 * Math.abs(z), 0.004); d = smax(d, 0.0045 - Math.hypot(y, Math.abs(z) - deep + 0.022), 0.001); return d + 0.0035 * step(0.16, 0.36, fbm3(x * 9 + o, y * 12, z * 9, 3)) * step(0.02, 0.0, Math.min(hx - Math.abs(x), deep - Math.abs(z))); };
  return solid(f, [hx, thick, deep], 0.013 * FINE_MADE.k, 2, (x, y, z) => 0.0006 * fbm3(x * 30 + o, y * 30, z * 30, 3) - 0.0011 * step(0.36, 0.52, noise3(x * 55 + o, y * 55, z * 55)),
    (x, y, z, _a, ny) => { const b = fbm3(x * 4 + o, y * 4, z * 4, 3), p = step(0.36, 0.52, noise3(x * 55 + o, y * 55, z * 55)), st = step(0.1, 0.4, noise3(x * 14 + o, 0.3 * y, z * 3)) * (1 - clamp01(ny)); return grey(0.86 + 0.08 * b - 0.16 * p - 0.14 * st, 0.8, 1 - 0.4 * p); });
}
/** a planter's side, cast against boards: so long along x, so high, 7 cm thick; it stands on 0 */
export function panel(seed: number, high: number, len = 0.6): Mesh {
  const r = rng(seed), o = r() * 80, hx = len / 2 - 0.002, ties: [number, number][] = [[-0.15, high * 0.28], [0.15, high * 0.28], [-0.15, high * 0.74], [0.15, high * 0.74]], lines = [0.25, 0.5, 0.75].map(k => k * high + (r() - 0.5) * 0.01);
  const f: Field = (x, y, z) => { let d = box(x, y - high / 2, z, hx, high / 2, 0.034, 0.006); for (const l of lines) d = Math.min(d, Math.hypot(y - l, Math.max(0, Math.abs(z) - 0.0345)) + Math.max(0, Math.abs(x) - hx + 0.004) - 0.0016); for (const [a, b] of ties) d = smax(d, 0.0125 - Math.hypot(x - a, y - b) + Math.max(0, 0.022 - Math.abs(z)) * 3, 0.002); return d; };
  return solid(f, [hx, high, 0.036], 0.013 * FINE_MADE.k, 2, (x, y, z) => 0.0007 * fbm3(x * 14 + o, y * 40, z * 14, 3) - 0.0016 * step(0.40, 0.56, noise3(x * 48 + o, y * 48, z * 48)),
    (x, y, z) => { const w = fbm3(x * 3 + o, y * 30, z * 3, 3), p = step(0.40, 0.56, noise3(x * 48 + o, y * 48, z * 48)), dirt = step(0.10, 0, y) * 0.16 + 0.10 * step(0.05, 0.4, noise3(x * 20 + o, y * 1.2, 2)) * step(high * 0.4, high, y); return grey(0.91 + 0.06 * w - 0.2 * p - dirt, 0.35, 1 - 0.5 * p); });
}
/** a post of the rail: flange, four bolts, two clamps for the pane (which is along x); it stands on 0 and is a metre high */
export function post(): Mesh {
  const f: Field = (x, y, z) => { let d = tube(x, y - 0.5, z, 0.0185, 0.5, 0.002); d = smin(d, tube(x, y - 0.005, z, 0.048, 0.005, 0.0015), 0.006); for (let k = 0; k < 4; k++) { const a = 0.785 + k * 1.5708; d = Math.min(d, tube(x - 0.036 * Math.cos(a), y - 0.0125, z - 0.036 * Math.sin(a), 0.0062, 0.004, 0.001)); } for (const h of [0.16, 0.84]) { d = smin(d, box(x - 0.028, y - h, z, 0.026, 0.024, 0.016, 0.003), 0.003); d = smin(d, box(x + 0.028, y - h, z, 0.026, 0.024, 0.016, 0.003), 0.003); for (const s of [-1, 1]) d = Math.min(d, Math.hypot(x - s * 0.034, y - h, Math.abs(z) - 0.0165) - 0.0055); } return d; };
  return solid((x, y, z) => f(x, y + 0.5, z), [0.06, 0.51, 0.06], 0.006 * FINE_MADE.k, 1, () => 0, (_x, y) => grey(0.97 - 0.06 * step(-0.46, -0.5, y), 0), 0.00012);
}
/** the hand's tube, 1.6 m along x, with a collar where it meets the next */
export function tubeRail(len: number): Mesh { const f: Field = (x, y, z) => { let d = tube(y, x, z, 0.0215, len / 2, 0.001); for (const s of [-1, 1]) d = smin(d, tube(y, x - s * (len / 2 - 0.02), z, 0.0245, 0.016, 0.002), 0.003); return d; }; return solid(f, [len / 2 + 0.01, 0.03, 0.03], 0.0085 * FINE_MADE.k, 1, () => 0, () => grey(0.97), 0.00012); }
/** upholstery: a pad with soft channels across, piping round both faces; it lies on 0, longest along z */
export function cushion(seed: number, hx: number, hz: number, thick: number, channels: number): Mesh {
  const r = rng(seed), o = r() * 30, w = 2 * hz / channels, rim = (x: number, z: number) => { const a = Math.abs(x) - hx + 0.05, b = Math.abs(z) - hz + 0.05; return Math.min(Math.max(a, b), 0) + Math.hypot(Math.max(a, 0), Math.max(b, 0)) - 0.05; };
  const f: Field = (x, y, z) => { const c = (z + hz) / w, k = Math.round(c), g = (c - k) * w, groove = k > 0 && k < channels ? 0.014 * Math.exp(-((g / 0.018) ** 2)) : 0, e = rim(x, z), puff = 0.010 * (1 - Math.exp(Math.min(0, e) / 0.06)), a = e + 0.018, b = Math.abs(y) - thick / 2 + 0.018 - (y > 0 ? puff - groove : 0.3 * puff); let d = Math.min(Math.max(a, b), 0) + Math.hypot(Math.max(a, 0), Math.max(b, 0)) - 0.018; for (const s of [-1, 1]) d = smin(d, Math.hypot(e + 0.004, y - s * (thick / 2 - 0.014)) - 0.0065, 0.004); return d; };
  return solid(f, [hx + 0.01, thick / 2 + 0.02, hz + 0.01], 0.013 * FINE_MADE.k, 2, (x, y, z) => 0.0007 * fbm3(x * 40 + o, y * 40, z * 40, 2) + 0.0012 * fbm3(x * 13 + o, y * 13, z * 5, 2), (x, y, z) => grey(0.94 + 0.05 * fbm3(x * 30 + o, y * 30, z * 30, 2), 0.1));
}
/** a drum of a seat, upholstered: piping at both edges, a button on top; it stands on 0 */
export function pouf(rad: number, high: number): Mesh { const f: Field = (x, y, z) => { const a = Math.atan2(z, x), fold = 0.004 * Math.cos(a * 12) * step(high * 0.5, 0.02, Math.abs(y - high / 2) < high / 2 ? Math.hypot(x, z) : 0); let d = tube(x, y - high / 2, z, rad - 0.003, high / 2 - 0.003, 0.03) - 0.010 * Math.cos(Math.min(1, Math.hypot(x, z) / rad) * 1.5) * step(high * 0.4, high * 0.6, y) - fold * 0; for (const h of [0.03, high - 0.03]) d = smin(d, Math.hypot(Math.hypot(x, z) - rad + 0.001, y - h) - 0.0065, 0.004); const b = Math.hypot(x, z); return smin(d + 0.02 * Math.exp(-((b / 0.03) ** 2)) * step(high * 0.5, high * 0.8, y), Math.hypot(x, (y - high + 0.004) * 1.6, z) - 0.013, 0.002); }; return solid((x, y, z) => f(x, y + high / 2, z), [rad + 0.015, high / 2 + 0.02, rad + 0.015], 0.012 * FINE_MADE.k, 1, (x, y, z) => 0.0006 * fbm3(x * 45, y * 45, z * 45, 2), (x, y, z) => grey(0.94 + 0.05 * fbm3(x * 30, y * 30, z * 30, 2), 0.1)); }
/** a table's round top, its edge turned; it lies on 0 */
export function tabletop(rad: number, thick: number): Mesh { return solid((x, y, z) => { const b = Math.hypot(x, z); return smax(tube(x, y, z, rad, thick / 2, 0.009), 0.0035 - Math.hypot(b - rad + 0.045, y - thick / 2), 0.001); }, [rad + 0.01, thick, rad + 0.01], 0.013 * FINE_MADE.k, 1, () => 0, () => grey(0.97), 0.00012); }
/** a drain's grate, a metre along x, 12 cm wide; its top at 0 */
export function grate(): Mesh { const f: Field = (x, y, z) => { const c = Math.abs((x / 0.025 + 0.5) % 1 + (x < 0 ? 1 : 0)) % 1, g = Math.abs(c - 0.5) * 0.025; let d = box(x, y + 0.008, z, 0.497, 0.008, 0.058, 0.0015); if (Math.abs(x) < 0.48) d = smax(d, Math.min(0.0065 - g, 0.047 - Math.abs(z)), 0.001); return d; }; return solid(f, [0.5, 0.02, 0.06], 0.0065 * FINE_MADE.k, 1, () => 0, (_x, y) => grey(0.92 + 0.1 * Math.max(-1, y / 0.016), 0), 0.00015); }
/** a rain pipe, a metre of it, upright, a collar at its top and a strap to the wall behind (−z); it stands on 0 */
export function downpipe(): Mesh { const f: Field = (x, y, z) => { let d = tube(x, y - 0.5, z, 0.044, 0.5, 0.001); d = smin(d, tube(x, y - 0.96, z, 0.0495, 0.035, 0.004), 0.003); d = smin(d, tube(x, y - 0.45, z, 0.0475, 0.0125, 0.002), 0.002); d = smin(d, box(x, y - 0.45, z + 0.065, 0.012, 0.0125, 0.03, 0.002), 0.004); for (const s of [-1, 1]) { d = smin(d, box(x - s * 0.056, y - 0.45, z, 0.014, 0.0125, 0.004, 0.001), 0.002); d = Math.min(d, Math.hypot(x - s * 0.06, y - 0.45, Math.abs(z) - 0.005) - 0.005); } return d; }; return solid((x, y, z) => f(x, y + 0.5, z), [0.08, 0.51, 0.1], 0.0085 * FINE_MADE.k, 1, () => 0, (x, y, z) => grey(0.96 - 0.08 * step(0.1, 0.5, noise3(x * 30, y * 2, z * 30)), 0.2), 0.00015); }
