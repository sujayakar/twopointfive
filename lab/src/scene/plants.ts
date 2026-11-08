// Trees and planters: trunks are tapers, crowns are clusters of real leaves that sway each on its own.
import { type V3, rng, rotY, rotX, rotZ, compose, apply, norm, add, mul, sub } from '../math';
import { World, F, type Piece, type Cluster, type Leaf, taper, type Instance } from './world';
import { M } from './materials';

export function leafBall(seed: number, n: number, size: number): Cluster {
  const r = rng(seed), leaves: Leaf[] = [];
  for (let i = 0; i < n; i++) {
    const z = r() * 2 - 1, a = r() * 6.2832, s = Math.sqrt(1 - z * z), d: V3 = [s * Math.cos(a), z, s * Math.sin(a)], rad = 0.35 + 0.65 * Math.cbrt(r());
    const t = norm([d[0] + (r() - 0.5) * 1.2, d[1] * 0.4 - 0.35 + (r() - 0.5), d[2] + (r() - 0.5) * 1.2]); let n0: V3 = norm([d[0] * 0.5 + (r() - 0.5), 0.75 + r() * 0.5, d[2] * 0.5 + (r() - 0.5)]);
    const k = n0[0] * t[0] + n0[1] * t[1] + n0[2] * t[2]; n0 = norm(sub(n0, mul(t, k)));
    leaves.push({ at: mul(d, rad), n: n0, t, size: size * (0.7 + 0.6 * r()), tone: r() });
  }
  return { leaves, kind: 0 };
}
/** the end of a branch: a stem, side twigs up its length, leaves in two rows along each. Grows along +y from −0.95; its leaves turn their faces to +y, where the light is */
export function spray(seed: number, twigs = 10, perTwig = 11): Cluster {
  const r = rng(seed), leaves: Leaf[] = [], lean: V3 = [(r() - 0.5) * 0.5, 0, (r() - 0.5) * 0.5], stem = (k: number): V3 => [lean[0] * k * k, -0.95 + 1.55 * k, lean[2] * k * k];
  const wood = (a: V3, b: V3) => { const d = sub(b, a), l = Math.hypot(...d), t = norm(d), n = norm([t[1], -t[0] + 0.01, 0.3]); const k = n[0] * t[0] + n[1] * t[1] + n[2] * t[2]; leaves.push({ at: mul(add(a, b), 0.5), n: norm(sub(n, mul(t, k))), t, size: l / 2 * 1.05, tone: r(), wood: true }); };
  const leaf = (p: V3, dir: V3, size: number, face: V3) => { const t = norm(dir), k = face[0] * t[0] + face[1] * t[1] + face[2] * t[2], n = norm(sub(face, mul(t, k))); leaves.push({ at: add(p, mul(t, size * 0.95)), n, t, size, tone: r() }); };
  for (let i = 0; i < 4; i++) wood(stem(i / 4), stem((i + 1) / 4));
  for (let i = 0; i < twigs; i++) {
    const k = 0.22 + 0.78 * (i + 0.5) / twigs, from = stem(k), an = i * 2.39996 + (r() - 0.5) * 0.6, open = i === twigs - 1 ? 0.15 : 0.55 + 0.45 * (1 - k) + (r() - 0.5) * 0.25, d = norm([Math.cos(an) * open, 0.75, Math.sin(an) * open]), len = (0.42 + 0.34 * (1 - k * 0.6)) * (0.85 + 0.3 * r()), side = norm([-Math.sin(an) + 0.01, 0, Math.cos(an)]), at = (q: number): V3 => { const s = q * len; return [from[0] + d[0] * s, from[1] + d[1] * s - 0.10 * q * q, from[2] + d[2] * s]; };
    wood(from, at(0.5)); wood(at(0.5), at(1));
    for (let j = 0; j < perTwig; j++) { const q = 0.16 + 0.84 * (j + 1) / perTwig, sgn = j === perTwig - 1 ? 0 : j % 2 ? 1 : -1, p = at(q), size = (0.105 + 0.03 * r()) * (1.06 - 0.22 * Math.abs(q - 0.55)), dir: V3 = [d[0] * 0.75 + side[0] * sgn * 0.8 + (r() - 0.5) * 0.35, d[1] * 0.45 - 0.10 + (r() - 0.5) * 0.3, d[2] * 0.75 + side[2] * sgn * 0.8 + (r() - 0.5) * 0.35]; leaf(p, dir, size, norm([(r() - 0.5) * 0.8 + d[0] * 0.35, 1, (r() - 0.5) * 0.8 + d[2] * 0.35])); }
  }
  return { leaves, kind: 0 };
}
export function grassTuft(seed: number, n: number): Cluster {
  const r = rng(seed), leaves: Leaf[] = [];
  for (let i = 0; i < n; i++) { const a = r() * 6.2832, lean = 0.1 + 0.75 * r() * r(), t = norm([Math.cos(a) * lean, 1, Math.sin(a) * lean]), side = norm([-Math.sin(a), 0, Math.cos(a)]); const h = 0.5 + 0.5 * r(); leaves.push({ at: add([(r() - 0.5) * 0.5, 0, (r() - 0.5) * 0.5], mul(t, h * 0.9)), n: norm([t[1] * side[2] - t[2] * side[1], t[2] * side[0] - t[0] * side[2], t[0] * side[1] - t[1] * side[0]]), t, size: h, tone: r() }); }
  return { leaves, kind: 1 };
}
export interface Kit { balls: number[]; sprays: number[]; tufts: number[] }
export function plantKit(w: World): Kit {
  const balls: number[] = [], sprays: number[] = [], tufts: number[] = [];
  for (let i = 0; i < 4; i++) { balls.push(w.clusters.length); w.clusters.push(leafBall(100 + i, 820, 0.094)); }
  for (let i = 0; i < 3; i++) { tufts.push(w.clusters.length); w.clusters.push(grassTuft(200 + i, 110)); }
  for (let i = 0; i < 6; i++) { sprays.push(w.clusters.length); w.clusters.push(spray(300 + i)); }
  return { balls, sprays, tufts };
}
/** how far along a ray the first piece stands (its box alone) */
function firstHit(pieces: Piece[], from: V3, dir: V3, far: number): number {
  let best = far; for (const p of pieces) { let a = 0, b = best; const o = sub(from, p.at); for (let k = 0; k < 3; k++) { const x = p.rot[k]!, q = x[0] * o[0] + x[1] * o[1] + x[2] * o[2], d = x[0] * dir[0] + x[1] * dir[1] + x[2] * dir[2], h = p.half[k]!; if (Math.abs(d) < 1e-9) { if (Math.abs(q) > h) { b = -1; break; } continue; } const u = (-h - q) / d, v = (h - q) / d; a = Math.max(a, Math.min(u, v)); b = Math.min(b, Math.max(u, v)); } if (a <= b && a > 0) best = a; } return best;
}
/** a clipped street tree: a short trunk, pale limbs that wander out to a dome, and leaves only where the light is: a shell of sprays round a dark hollow */
export function domeTree(w: World, kit: Kit, at: V3, height: number, seed: number, dark: V3 = [0.25, 0.39, 0.17], light: V3 = [0.45, 0.56, 0.21], open = false) {
  const r = rng(seed), bark = { tint: [1, 0.98, 0.95] as V3, flags: open ? F.OPEN : F.CUT }, walls = w.pieces.slice(), R = height * (0.40 + 0.05 * r()), H = height * 0.36, mid: V3 = add(at, [(r() - 0.5) * R * 0.3, height * 0.64, (r() - 0.5) * R * 0.3]), fork = add(at, [(mid[0] - at[0]) * 0.3, height * 0.20, (mid[2] - at[2]) * 0.3]), thick = height * 0.026;
  /** out from the middle toward the dome, as far as what stands in the way lets it */
  const onDome = (d: V3, k: number, keep = 0.05): V3 => { const to: V3 = [d[0] * R * k, d[1] * H * k, d[2] * R * k], l = Math.hypot(...to), f = Math.max(0, Math.min(l, firstHit(walls, mid, mul(to, 1 / l), l + keep) - keep)); return add(mid, mul(to, f / l)); };
  w.taper(at, fork, thick * 1.25, thick, M.paleBark!, bark); w.ellipsoid(at, [thick * 1.8, thick * 0.8, thick * 1.8], M.paleBark!, bark); w.ellipsoid(fork, [thick * 1.15, thick * 1.15, thick * 1.15], M.paleBark!, bark);
  /** a limb: bent pieces from a to b, bowing out first and then up, thinner as it goes; returns the points it passes */
  const limb = (a: V3, b: V3, r0: number, r1: number, n: number): V3[] => { const pts: V3[] = [a]; const side = norm([r() - 0.5, 0.2 * (r() - 0.5), r() - 0.5]), wob = height * 0.035; for (let i = 1; i <= n; i++) { const t = i / n, s = Math.sin(t * 6.2832 * (0.7 + 0.2 * r())) * (1 - t) * wob, bow = Math.sin(t * Math.PI) * height * 0.03; pts.push([a[0] + (b[0] - a[0]) * t + side[0] * s, a[1] + (b[1] - a[1]) * (t * 0.8 + 0.2 * t * t) + side[1] * s - bow, a[2] + (b[2] - a[2]) * t + side[2] * s]); }
    for (let i = 0; i < n; i++) { const ra = r0 + (r1 - r0) * (i / n), rb = r0 + (r1 - r0) * ((i + 1) / n); w.taper(pts[i]!, pts[i + 1]!, ra, rb, M.paleBark!, bark); w.ellipsoid(pts[i + 1]!, [rb * 1.02, rb * 1.02, rb * 1.02], M.paleBark!, bark); } return pts; };
  const tips: V3[] = [], bough = (a: V3, b: V3, r0: number, r1: number, n: number) => { const p = limb(a, b, r0, r1, n); tips.push(...p.slice(1)); return p; };
  const sprayAt = (p: V3, out: V3, s: number, k: number) => { const v = 0.86 + 0.28 * r(), t = [0, 1, 2].map(i => (dark[i]! + (light[i]! - dark[i]!) * k) * v * (0.94 + 0.12 * r())) as unknown as V3, y = norm([out[0] + (r() - 0.5) * 0.5, out[1] + 0.55 + (r() - 0.5) * 0.4, out[2] + (r() - 0.5) * 0.5]), a0 = r() * 6.28, x0: V3 = Math.abs(y[1]) < 0.95 ? [0, 1, 0] : [1, 0, 0], e = norm([y[1] * x0[2] - y[2] * x0[1], y[2] * x0[0] - y[0] * x0[2], y[0] * x0[1] - y[1] * x0[0]]), f: V3 = [y[1] * e[2] - y[2] * e[1], y[2] * e[0] - y[0] * e[2], y[0] * e[1] - y[1] * e[0]], c = Math.cos(a0), q = Math.sin(a0), x = add(mul(e, c), mul(f, q)), z: V3 = [x[1] * y[2] - x[2] * y[1], x[2] * y[0] - x[0] * y[2], x[0] * y[1] - x[1] * y[0]];
    w.instances.push({ cluster: kit.sprays[Math.floor(r() * kit.sprays.length)]!, at: p, rot: [x, y, z], scale: s, base: at, sway: 0.03 + 0.02 * (p[1] - at[1]), phase: r() * 6.28, tint: t, open });
    return sub(p, mul(y, s * 0.95)); };
  const n = 5 + Math.floor(r() * 2), a0 = r() * 6.28; for (let i = 0; i < n; i++) { const a = a0 + i * 6.2832 / n + (r() - 0.5) * 0.5, up = 0.15 + 0.6 * r(), d: V3 = norm([Math.cos(a), up, Math.sin(a)]), pts = bough(fork, onDome(d, 0.62), thick * 0.72, thick * 0.30, 5); for (let k = 0; k < 3; k++) { const from = pts[2 + k]!, e = norm([d[0] + (r() - 0.5) * 1.1, d[1] + 0.5 * r() - 0.05, d[2] + (r() - 0.5) * 1.1]); bough(from, onDome(e, 0.72), thick * 0.36, thick * 0.14, 3); } }
  bough(fork, onDome([0.05, 1, 0.05], 0.66), thick * 0.6, thick * 0.2, 5);
  // the shell: sprays spread evenly over the dome and a little way under its edge, each on a thin branch from the nearest bough; and some inside, in the dark
  const N = Math.round(34 * height), thin = (p: V3) => { let near = tips[0]!, best = 1e9; for (const q of tips) { const d2 = (q[0] - p[0]) ** 2 + (q[1] - p[1]) ** 2 + (q[2] - p[2]) ** 2; if (d2 < best) { best = d2; near = q; } } if (best > 0.004 && best < (height * 0.4) ** 2) w.taper(near, p, thick * 0.11, thick * 0.06, M.paleBark!, bark); };
  for (let i = 0; i < N; i++) { const y = 1 - (i + 0.5) / N * 1.30, q = Math.sqrt(Math.max(0, 1 - y * y)), a = i * 2.39996 + r() * 0.6, d: V3 = [q * Math.cos(a), y, q * Math.sin(a)], deep = i % 4 === 3, k = deep ? 0.66 + 0.12 * r() : 0.84 + 0.16 * r(), s = height * (0.082 + 0.028 * r()); thin(sprayAt(onDome(d, k, s * 0.95), d, s, Math.max(0, Math.min(1, (deep ? -0.1 : 0.12) + 0.75 * y + 0.35 * (r() - 0.5))))); }
}
/** a low clipped bush, or a patch of climber on a wall (`flat` = the way it faces) */
export function bush(w: World, kit: Kit, at: V3, s: number, seed: number, tint: V3) { const r = rng(seed); w.instances.push({ cluster: kit.balls[Math.floor(r() * kit.balls.length)]!, at, rot: rotY(r() * 6.28), scale: s, base: [at[0], at[1] - s, at[2]], sway: 0.025, phase: r() * 6.28, tint: [tint[0] * (0.85 + 0.3 * r()), tint[1] * (0.9 + 0.2 * r()), tint[2] * (0.8 + 0.4 * r())] }); }
export function tuft(w: World, kit: Kit, at: V3, h: number, seed: number, tint: V3 = [0.46, 0.55, 0.28]) {
  const r = rng(seed); w.instances.push({ cluster: kit.tufts[Math.floor(r() * kit.tufts.length)]!, at, rot: rotY(r() * 6.28), scale: h, base: at, sway: 0.09, phase: r() * 6.28, tint: [tint[0] * (0.8 + 0.4 * r()), tint[1] * (0.85 + 0.3 * r()), tint[2] * (0.7 + 0.5 * r())] });
}
/** the wind on one instance at a time: how it leans, and where that puts it. A pure function of the tick */
export function swayed(i: Instance, t: number, wind: V3): { at: V3; rot: typeof i.rot } {
  const gust = 0.55 + 0.45 * Math.sin(t * 0.7 - (i.base[0] * wind[0] + i.base[2] * wind[2]) * 0.35) + 0.2 * Math.sin(t * 1.9 + i.phase), flick = Math.sin(t * 2.3 + i.phase) * 0.35 + Math.sin(t * 3.7 + i.phase * 1.7) * 0.2;
  const ax = wind[0] * (gust + flick * 0.4) * i.sway, az = wind[2] * (gust + flick * 0.4) * i.sway, h = i.at[1] - i.base[1], rot = compose(compose(rotZ(-ax * 1.6 + flick * 0.03), rotX(az * 1.6 + flick * 0.03)), i.rot);
  void F; void taper; void apply; return { at: add(i.at, [ax * h * 0.55, -Math.abs(ax) * h * 0.05, az * h * 0.55]), rot };
}
