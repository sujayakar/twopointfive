// Trees and planters: trunks are tapers, crowns are clusters of real leaves that sway each on its own.
import { type V3, rng, rotY, rotX, rotZ, compose, apply, norm, add, mul, sub } from '../math';
import { World, F, type Cluster, type Leaf, taper, type Instance } from './world';
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
export function grassTuft(seed: number, n: number): Cluster {
  const r = rng(seed), leaves: Leaf[] = [];
  for (let i = 0; i < n; i++) { const a = r() * 6.2832, lean = 0.15 + 0.6 * r(), t = norm([Math.cos(a) * lean, 1, Math.sin(a) * lean]), side = norm([-Math.sin(a), 0, Math.cos(a)]); const h = 0.5 + 0.5 * r(); leaves.push({ at: add([(r() - 0.5) * 0.5, 0, (r() - 0.5) * 0.5], mul(t, h * 0.9)), n: norm([t[1] * side[2] - t[2] * side[1], t[2] * side[0] - t[0] * side[2], t[0] * side[1] - t[1] * side[0]]), t, size: h, tone: r() }); }
  return { leaves, kind: 1 };
}
export interface Kit { balls: number[]; tufts: number[] }
export function plantKit(w: World): Kit {
  const balls: number[] = [], tufts: number[] = [];
  for (let i = 0; i < 4; i++) { balls.push(w.clusters.length); w.clusters.push(leafBall(100 + i, 260, 0.17)); }
  for (let i = 0; i < 3; i++) { tufts.push(w.clusters.length); w.clusters.push(grassTuft(200 + i, 46)); }
  return { balls, tufts };
}
/** a tree: a trunk that forks a few times, a ball of leaves at each end */
export function tree(w: World, kit: Kit, at: V3, height: number, seed: number, tint: V3 = [0.50, 0.62, 0.22]) {
  const r = rng(seed), ends: [V3, number][] = [];
  const limb = (a: V3, dir: V3, l: number, ra: number, depth: number) => {
    const b = add(a, mul(dir, l)), rb = ra * 0.62; w.taper(a, b, ra, rb, M.bark!, { tint: [0.9, 0.85, 0.8], flags: F.CUT }); w.ellipsoid(b, [rb * 1.02, rb * 1.02, rb * 1.02], M.bark!, { tint: [0.9, 0.85, 0.8], flags: F.CUT });
    if (depth === 0) { ends.push([b, l]); return; }
    const n = depth > 1 ? 3 : 3 + (r() > 0.5 ? 1 : 0), a0 = r() * 6.28;
    for (let i = 0; i < n; i++) { const an = a0 + i * 6.2832 / n + (r() - 0.5) * 0.7, out = 0.55 + 0.35 * r(); limb(b, norm([dir[0] * 0.5 + Math.cos(an) * out, 0.75 + 0.3 * r(), dir[2] * 0.5 + Math.sin(an) * out]), l * (0.62 + 0.2 * r()), rb * 0.8, depth - 1); }
  };
  limb(at, norm([(r() - 0.5) * 0.2, 1, (r() - 0.5) * 0.2]), height * 0.40, height * 0.030, 2);
  for (const [e, l] of ends) { const s = height * (0.135 + 0.05 * r()); w.instances.push({ cluster: kit.balls[Math.floor(r() * kit.balls.length)]!, at: add(e, [0, s * 0.25, 0]), rot: rotY(r() * 6.28), scale: s, base: at, sway: 0.05 + 0.035 * (e[1] - at[1]), phase: r() * 6.28, tint: [tint[0] * (0.85 + 0.3 * r()), tint[1] * (0.9 + 0.2 * r()), tint[2] * (0.8 + 0.4 * r())] }); void l; }
}
export function tuft(w: World, kit: Kit, at: V3, h: number, seed: number, tint: V3 = [0.55, 0.64, 0.25]) {
  const r = rng(seed); w.instances.push({ cluster: kit.tufts[Math.floor(r() * kit.tufts.length)]!, at, rot: rotY(r() * 6.28), scale: h, base: at, sway: 0.09, phase: r() * 6.28, tint: [tint[0] * (0.8 + 0.4 * r()), tint[1] * (0.85 + 0.3 * r()), tint[2] * (0.7 + 0.5 * r())] });
}
/** the wind on one instance at a time: how it leans, and where that puts it. A pure function of the tick */
export function swayed(i: Instance, t: number, wind: V3): { at: V3; rot: typeof i.rot } {
  const gust = 0.55 + 0.45 * Math.sin(t * 0.7 - (i.base[0] * wind[0] + i.base[2] * wind[2]) * 0.35) + 0.2 * Math.sin(t * 1.9 + i.phase), flick = Math.sin(t * 2.3 + i.phase) * 0.35 + Math.sin(t * 3.7 + i.phase * 1.7) * 0.2;
  const ax = wind[0] * (gust + flick * 0.4) * i.sway, az = wind[2] * (gust + flick * 0.4) * i.sway, h = i.at[1] - i.base[1], rot = compose(compose(rotZ(-ax * 1.6 + flick * 0.03), rotX(az * 1.6 + flick * 0.03)), i.rot);
  void F; void taper; void apply; return { at: add(i.at, [ax * h * 0.55, -Math.abs(ax) * h * 0.05, az * h * 0.55]), rot };
}
