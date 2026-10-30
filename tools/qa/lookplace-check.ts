// The look prototype's place read on this side of a wall (src/render/shaders/composite.wgsl lookPlaceHere; render/frame.ts lookConfig.footing → frame.gradeCfg.z;
// the page's ?footing=0). The shade row brings a dark place up by what the radiance cache holds in the air there, and by a room's OUTER wall the old five reads
// took in the open air beyond it: the floor at the wall's foot was not brought up (the maintainer, 10-30: "very odd pooling of shadow under window").
// No GPU: the shader's read is mirrored here over a cache of two values, 1 on this side of a wall and 20 beyond it, at floor points by every ceiling-high wall
// of every level. The picture is never SEEN here.
//   bun run tools/qa/lookplace-check.ts   (exit 0 = all pass; LOOKPLACE_BREAK=1 reads the place by the old five reads and must fail)
import { readFileSync, readdirSync } from 'node:fs';
import { resolve } from 'node:path';
import { WORLD } from '../../src/scene/world';
import { defaultRcConfig } from '../../src/render/rc';
import { BoxFlag } from '../../src/scene/boxes';
import { loadLevel } from '../../src/scene/levelLoad';
import { parseLevelData } from '../../src/scene/levelData';
let red = 0; const check = (ok: boolean, w: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };
const BREAK = process.env.LOOKPLACE_BREAK === '1', ROOT = resolve(import.meta.dir, '../..');
const CY: number = WORLD.ceilingY, RC0: number[] = [...WORLD.rcMin], SP = [WORLD.rcSize[0] / defaultRcConfig.nx, WORLD.rcSize[1] / defaultRcConfig.ny, WORLD.rcSize[2] / defaultRcConfig.nz];
type V = number[]; type Bx = { c: ArrayLike<number>; h: ArrayLike<number>; flags: number; yaw?: number; rot?: unknown };
const len = (a: V) => Math.hypot(a[0], a[1], a[2]);
/** the nearest of `B` (axis-aligned boxes) a segment meets, or −1 */
const rayT = (B: readonly Bx[], ro: V, d: V, tmax: number) => { let best = -1;
  for (const b of B) { let tN = -1e30, tF = 1e30;
    for (let a = 0; a < 3; a++) { const lo = b.c[a] - b.h[a] - ro[a], hi = b.c[a] + b.h[a] - ro[a];
      if (Math.abs(d[a]) < 1e-12) { if (lo > 0 || hi < 0) { tN = 1e30; break; } continue; }
      const t0 = lo / d[a], t1 = hi / d[a]; tN = Math.max(tN, Math.min(t0, t1)); tF = Math.min(tF, Math.max(t0, t1)); }
    if (tN > tF || tF < 0 || tN > tmax) continue; const t = Math.max(tN, 0); if (best < 0 || t < best) best = t; }
  return best; };
const FAR = 20, sm = (a: number, b: number, x: number) => { const t = Math.min(1, Math.max(0, (x - a) / (b - a))); return t * t * (3 - 2 * t); };
const T = { n: 0, old: 0, now: 0, sumOld: 0, sumNow: 0, rays: 0, mid: 0, midSame: 0, levels: 0 }; let worst = 0;
for (const file of readdirSync(resolve(ROOT, 'levels')).filter(f => f.endsWith('.json') && !f.includes('.lint.'))) {
  let all: Bx[]; try { all = loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8'))).boxes as unknown as Bx[]; } catch { continue; }
  T.levels++;
  const solid = all.filter(b => !(b.flags & (BoxFlag.NoShadow | BoxFlag.Glass)) && !b.rot && Math.abs(Math.sin(b.yaw ?? 0)) < 1e-6);
  const walls = solid.filter(b => !(b.flags & BoxFlag.Dynamic) && b.c[1] - b.h[1] <= 0.45 && b.c[1] + b.h[1] >= CY - 0.1 && Math.max(b.h[0], b.h[2]) > 0.3 && Math.min(b.h[0], b.h[2]) < 0.2);
  for (const b of walls) { const ax = b.h[0] < b.h[2] ? 0 : 2, al = 2 - ax;
    for (const sgn of [-1, 1]) for (const D of [0.05, 0.15, 0.3, 0.45, 0.7, 0.9, 1.6]) for (let u = -b.h[al] + 0.45; u <= b.h[al] - 0.45; u += 0.25) { const X = [0, 0.3, 0]; X[ax] = b.c[ax] + sgn * (b.h[ax] + D); X[al] = b.c[al] + u;
      if (X[0] < 1.2 || X[2] < 1.2 || X[0] > WORLD.max[0] - 1.2 || X[2] > WORLD.max[2] - 1.2) continue;
      const near = solid.filter(q => Math.abs(q.c[0] - X[0]) <= q.h[0] + 2.6 && Math.abs(q.c[2] - X[2]) <= q.h[2] + 2.6);
      if (near.some(q => [0, 1, 2].every(i => Math.abs(X[i] - q.c[i]) <= q.h[i] + 0.02))) continue;
      const W2 = [{ ...b, c: [b.c[0], 0, b.c[2]], h: [b.h[0], 1, b.h[2]] }];   // (this wall alone: a cell is "beyond" when this wall stands between the point and its centre)
      const hid = (x: number, z: number) => { const v = [x - X[0], 0, z - X[2]], l = len(v); return l > 1e-6 && rayT(W2, [X[0], 0, X[2]], v.map(q => q / l), l) >= 0; };
      /** the model cache at a probe: nothing inside the wall, FAR beyond it, 1 on this side */
      const val = (cx: number, cz: number) => Math.abs(cx - b.c[0]) <= b.h[0] && Math.abs(cz - b.c[2]) <= b.h[2] ? 0 : hid(cx, cz) ? FAR : 1;
      const tri = (Q: V) => { const gx = (Q[0] - RC0[0]) / SP[0] - 0.5, gz = (Q[2] - RC0[2]) / SP[2] - 0.5, ix = Math.floor(gx), iz = Math.floor(gz), fx = gx - ix, fz = gz - iz; let s = 0;
        for (const [dx, dz, k] of [[0, 0, (1 - fx) * (1 - fz)], [1, 0, fx * (1 - fz)], [0, 1, (1 - fx) * fz], [1, 1, fx * fz]]) if (k > 1e-6) s += k * val((ix + dx + 0.5) * SP[0] + RC0[0], (iz + dz + 0.5) * SP[2] + RC0[2]); return s; };
      const s = [SP[0] * 0.6, 0, SP[2] * 0.6], sides = [[s[0], 0, 0], [-s[0], 0, 0], [0, 0, s[2]], [0, 0, -s[2]]];
      const old = Math.max(tri(X), ...sides.map(d => tri([X[0] + d[0], X[1], X[2] + d[2]])));   // (composite.wgsl lookPlaceHere with frame.gradeCfg.z 0)
      /** composite.wgsl lookPlaceHere, statement for statement */
      const place = () => { const plain = tri(X), home = [0, 1, 2].map(a => (Math.floor((X[a] - RC0[a]) / SP[a]) + 0.5) * SP[a] + RC0[a]), nb = home.map((h, a) => h + (X[a] >= h ? 1 : -1) * SP[a]);
        const f = [0, 1, 2].map(a => Math.min(0.5, Math.max(0, Math.abs(X[a] - home[a]) / SP[a]))), C = (k: number) => [k & 1 ? nb[0] : home[0], X[1], k & 2 ? nb[2] : home[2]];
        const v = [0, 1, 2, 3].map(k => val(C(k)[0], C(k)[2])), t = sm(1.3, 2.0, Math.max(...v) / Math.max(Math.min(...v), 1e-5));
        if (t <= 0) return plain;
        let sum = 0, ws = 0;
        for (let k = 0; k < 4; k++) { const w = (k & 1 ? f[0] : 1 - f[0]) * (k & 2 ? f[2] : 1 - f[2]); if (w <= 1e-4) continue;
          const d = C(k).map((c, a) => c - X[a]), l = len(d); T.rays++;
          if (l < 1e-3 || rayT(near, X, d.map(q => q / Math.max(l, 1e-3)), l) < 0) { sum += w * v[k]; ws += w; } }
        return plain + ((ws <= 1e-4 ? Math.max(...v) : sum / ws) - plain) * t; };
      const now = BREAK ? old : place();
      if (D > 1.5) { T.mid++; if (Math.abs(now - tri(X)) < 1e-9) T.midSame++; continue; }   // (a stride and a half in: the plain read's own ground)
      T.n++; if (old > 2) T.old++; if (now > 2) T.now++; T.sumOld += old; T.sumNow += now; worst = Math.max(worst, now); } } }
const pct = (a: number, b: number) => (100 * a / Math.max(b, 1)).toFixed(1);
check(T.levels > 10 && T.n > 50000 && T.old / T.n > 0.8, `the old five reads took the far side for this side's light: over twice this side's at ${pct(T.old, T.n)} % of ${T.n} floor points within 0.9 m of a ceiling-high wall (${T.levels} levels; the far side holds ${FAR} times this side's), ${(T.sumOld / T.n).toFixed(1)} times on the mean`);
check(T.now / T.n < 0.03 && T.sumNow / T.n < 1.3, `the walled read is this side's: over twice at ${pct(T.now, T.n)} % of them (under 3: a point that sees round the wall's end or through its doorway holds what it sees), ${(T.sumNow / T.n).toFixed(2)} times on the mean, ${(T.rays / T.n).toFixed(1)} box rays a point`);
check(T.mid > 5000 && T.midSame / T.mid > 0.6, `a stride and a half from the wall the read is the plain one at ${pct(T.midSame, T.mid)} % of ${T.mid} points (no ray is asked where the four probes hold the same)`);
{ const c = readFileSync(resolve(ROOT, 'src/render/shaders/composite.wgsl'), 'utf8'), c2 = readFileSync(resolve(ROOT, 'src/render/shaders/compact/composite.wgsl'), 'utf8'), eng = readFileSync(resolve(ROOT, 'src/engine.ts'), 'utf8'), main = readFileSync(resolve(ROOT, 'src/main.ts'), 'utf8');
  const fn = (t: string) => { const a = t.indexOf('fn lookPlaceHere('), b = t.indexOf('\n}\n', a); return a < 0 ? '' : t.slice(a, b); };
  check(fn(c).length > 500 && fn(c) === fn(c2) && /if \(frame\.gradeCfg\.z <= 0\.0\) \{[^\n]*return max\(max\(luminance\(diceMeanRadiance\(X\)\)/.test(fn(c)) && (fn(c).match(/occludedTBoxes\(/g) ?? []).length === 1 && (c.match(/lookPlaceHere\(/g) ?? []).length === 2
    && /smoothstep\(1\.3, 2\.0, hi \/ max\(lo, 1e-5\)\)/.test(fn(c)) && fn(c).includes('if (ws <= 1e-4) { return mix(plain, hi, t); }') && /lookConfig\.footing, 0\] : undefined/.test(eng) && /q\.get\('footing'\) === '0'\) lookConfig\.footing = 0/.test(main),
    'both writings of the kernel carry the one read (one box walk in its text, one call site), with the row off it is the five reads as they were, the engine hands the row to the frame on the look page and ?footing=0 lowers it'); }
process.exit(red ? 1 : 0);
