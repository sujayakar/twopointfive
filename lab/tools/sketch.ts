// A sketch of objects without a GPU, in seconds: bun run tools/sketch.ts chair [car …] → $OUT/<name>-sketch.png. A row a level, from four sides and above; sun, a soft sky, shadows; no print, no reflection.
//   bun run tools/sketch.ts --swap chair     each coarser level where it takes over, from two sides, against the finest: outline, how much it fills, brightness, color, roughly; $OUT/<name>-swap.png shows where
// For form and color while making; the bench (object.cjs) is what judges.
import { deflateSync, crc32 } from 'node:zlib';
import { mkdirSync, writeFileSync } from 'node:fs';
import { OBJECTS } from '../src/objects/index';
import { PIXEL, takesOver, type Made } from '../src/objects/object';
import { build, bounds } from '../src/objects/rules';
import { traced } from '../src/objects/trace';
import { maskOf, outline, light, fuller, union, exposure, type Mask } from '../src/objects/pixels';
import { type V3, add, sub, mul, dot, cross, norm } from '../src/math';
const SUN = norm([0.5, 0.75, 0.6]), OUT = process.env.OUT ?? 'out'; mkdirSync(OUT, { recursive: true });
const png = (d: Uint8Array, w: number, h: number): Uint8Array => { const raw = new Uint8Array((w * 3 + 1) * h); for (let y = 0; y < h; y++) raw.set(d.subarray(y * w * 3, (y + 1) * w * 3), y * (w * 3 + 1) + 1); const chunk = (t: string, b: Uint8Array) => { const o = new Uint8Array(b.length + 12), v = new DataView(o.buffer); v.setUint32(0, b.length); for (let i = 0; i < 4; i++) o[4 + i] = t.charCodeAt(i); o.set(b, 8); v.setUint32(8 + b.length, crc32(o.subarray(4, 8 + b.length))); return o; }, hd = new Uint8Array(13), v = new DataView(hd.buffer); v.setUint32(0, w); v.setUint32(4, h); hd[8] = 8; hd[9] = 2; const parts = [Uint8Array.from([137, 80, 78, 71, 13, 10, 26, 10]), chunk('IHDR', hd), chunk('IDAT', deflateSync(raw)), chunk('IEND', new Uint8Array(0))], out = new Uint8Array(parts.reduce((n, p) => n + p.length, 0)); let o = 0; for (const p of parts) { out.set(p, o); o += p.length; } return out; };
/** what a ray sees of a level: light, and is it the thing or the floor */
function tracer(m: Made, level: number, R: number) {
  const q = traced(m, level), ROUND = [[0, 1, 0], [0.7, 0.5, 0.3], [-0.7, 0.5, 0.3], [0, 0.5, -0.8]] as V3[];
  return (o: V3, d: V3): { c: V3; own: boolean } => { let h = q.meet(o, d, 1e6); const floor = -o[1] / d[1]; if (floor > 0 && floor < h.t) h = { t: floor, k: -1 }; if (h.t > 1e5) return { c: [0.5, 0.55, 0.6], own: false };
    const s0 = h.k < 0 ? { x: add(o, mul(d, h.t)), n: [0, 1, 0] as V3, c: [0.36, 0.36, 0.37] as V3, through: null } : q.at(h, o, d), col = s0.through ? mul(s0.c, 1.65) as V3 : s0.c, n = s0.n, at = s0.x;
    const s = dot(n, SUN) > 0 && q.meet(add(at, mul(n, 2e-3)), SUN, 1e6, true).k < 0 ? dot(n, SUN) : 0, sky = 0.30 + 0.18 * n[1]; let ao = 0; for (const r of ROUND) { const w = norm(add(norm(r), mul(n, 1.1))); if (q.meet(add(at, mul(n, 2e-3)), w, R * 0.6, true).k < 0) ao += 0.25; } return { c: mul(col, 1.15 * s + sky * (0.25 + 0.75 * ao)), own: h.k >= 0 }; };
}
const shown = (v: number) => Math.min(255, 255 * Math.pow(Math.min(1, v / (1 + 0.25 * v)), 1 / 2.2)), swap = process.argv.includes('--swap');
for (const name of process.argv.slice(2).filter(a => !a.startsWith('--'))) {
  const m = OBJECTS[name]?.make(); if (!m) { console.error('no object ' + name); process.exit(2); } const t0 = performance.now(), b0 = bounds(build(m, 0)), mid = mul(add(b0.lo, b0.hi), 0.5) as V3, diag = Math.hypot(...sub(b0.hi, b0.lo)), R = diag / 2 * 1.08, look = m.levels.map((_, k) => tracer(m, k, R)), axes = (yaw: number, pitch: number) => { const cp = Math.cos(pitch), back: V3 = [Math.sin(yaw) * cp, Math.sin(pitch), Math.cos(yaw) * cp], right = norm(cross(mul(back, -1), [0, 1, 0])); return { back, right, up: cross(right, mul(back, -1)) }; };
  if (swap) {
    const cells: { px: Uint8Array; n: number }[] = []; let side = 0;
    for (let k = 1; k < m.levels.length; k++) { const far = Math.max(4, takesOver(m.levels[k]!)), n = Math.max(16, Math.min(200, Math.ceil(diag / (far * PIXEL) * 1.25))), SS = 4, said: string[] = []; side = Math.max(side, n);
      for (const yaw of [0.62, 0.62 - Math.PI / 2]) { const { back, right, up } = axes(yaw, 0.78), eye = add(mid, mul(back, far)), take = (f: typeof look[0]): { pic: Float32Array; mask: Mask } => { const pic = new Float32Array(n * n * 3), fine = new Uint8Array(n * n * SS * SS); for (let y = 0; y < n * SS; y++) for (let x = 0; x < n * SS; x++) { const d = norm(add(add(mul(back, -1), mul(right, ((x + 0.5) / SS - n / 2) * PIXEL)), mul(up, (n / 2 - (y + 0.5) / SS) * PIXEL))), near = add(eye, mul(d, far - R * 2)), q = f(near, d), o = (Math.floor(y / SS) * n + Math.floor(x / SS)) * 3; for (let a = 0; a < 3; a++) pic[o + a]! += q.c[a]! / (SS * SS); if (q.own) fine[y * n * SS + x] = 1; } return { pic, mask: maskOf(fine, n, n, SS) }; };
        const a = take(look[0]!), b = take(look[k]!), o = outline(a.mask, b.mask), l = light(a.pic, b.pic, union([a.mask, b.mask]), fuller([a.mask, b.mask]), n, n, exposure(a.pic)); said.push(`${yaw > 0 ? 'front' : 'side'}: outline ${o.far.toFixed(2)} px, fills ${o.cover.toFixed(1)} %, ${l.stops.toFixed(2)} stops, color ${l.color.toFixed(2)} %, places ${l.patch.toFixed(0)}`);
        // the two, and where they part: red only the finest, blue only the coarser
        const px = new Uint8Array(n * 3 * n * 3); for (let y = 0; y < n; y++) for (let x = 0; x < n; x++) { const i = y * n + x, d = b.mask.fill[i]! - a.mask.fill[i]!; for (let c = 0; c < 3; c++) { px[(y * n * 3 + x) * 3 + c] = shown(a.pic[i * 3 + c]!); px[(y * n * 3 + n + x) * 3 + c] = shown(b.pic[i * 3 + c]!); } const g = 40 + 110 * Math.min(a.mask.fill[i]!, b.mask.fill[i]!); px.set([g + (d < 0 ? -d * 200 : 0), g, g + (d > 0 ? d * 200 : 0)], (y * n * 3 + 2 * n + x) * 3); } cells.push({ px, n }); }
      console.log(`${m.title}, level ${k} from ${far.toFixed(0)} m: ${said.join(' · ')}`); }
    const Z = Math.max(1, Math.floor(240 / side)), W = side * 3 * Z * 2, H = side * Z * (m.levels.length - 1), out = new Uint8Array(W * H * 3); cells.forEach((c, i) => { for (let y = 0; y < c.n * Z; y++) for (let x = 0; x < c.n * 3 * Z; x++) { const o = ((Math.floor(i / 2) * side * Z + y) * W + (i % 2) * side * 3 * Z + x) * 3, s = (Math.floor(y / Z) * c.n * 3 + Math.floor(x / Z)) * 3; out[o] = c.px[s]!; out[o + 1] = c.px[s + 1]!; out[o + 2] = c.px[s + 2]!; } }); writeFileSync(`${OUT}/${name}-swap.png`, png(out, W, H)); console.log(`${OUT}/${name}-swap.png (finest, level, where they part; from the front and from the side), ${((performance.now() - t0) / 1000).toFixed(1)} s`); continue;
  }
  const S = +(process.env.SIZE ?? 260), cols = 5, W = cols * S, H = m.levels.length * S, px = new Uint8Array(W * H * 3);
  look.forEach((f, row) => { for (let c = 0; c < cols; c++) { const { back, right, up } = axes(0.62 + c * Math.PI / 2, c === 4 ? 1.35 : 0.5), fwd = mul(back, -1); for (let y = 0; y < S; y++) for (let x = 0; x < S; x++) { const q = f(add(add(add(mid, mul(back, R * 6)), mul(right, (x / S * 2 - 1) * R)), mul(up, (1 - y / S * 2) * R)), fwd), o = ((row * S + y) * W + c * S + x) * 3; for (let a = 0; a < 3; a++) px[o + a] = shown(q.c[a]!); } } });
  writeFileSync(`${OUT}/${name}-sketch.png`, png(px, W, H)); console.log(`${OUT}/${name}-sketch.png, ${((performance.now() - t0) / 1000).toFixed(1)} s`);
}
