// MEASURE: methods against the reference, over fixed views. For each: milliseconds, and three errors kept apart.
//   bias   the mean of many frames, standing still, against the reference: what would never go away
//   noise  one frame against the next, standing still: what shimmers
//   error  one frame against the reference: what is seen
// All in linear light, relative to the reference's own brightness (so a dark room counts as a bright one does); medians over the picture, so that a few
// fireflies in the reference do not speak for it. The run's picture has a fingerprint: the same build, card and seed give the same one.
import type { Lab, Method } from './lab';
import type { View } from './shot';
export interface Score { view: string; method: string; ms: number; bias: number; noise: number; error: number; hash: string; passes: Record<string, number> }
const half = (h: number) => { const s = h & 0x8000 ? -1 : 1, e = (h >> 10) & 31, m = h & 1023; return e === 0 ? s * m * 2 ** -24 : e === 31 ? 65504 * s : s * (1 + m / 1024) * 2 ** (e - 15); };
const TABLE = new Float32Array(65536); for (let i = 0; i < 65536; i++) TABLE[i] = half(i);
async function picture(lab: Lab, m: Method): Promise<Float32Array> { lab.graph.flip(); const r = await lab.read(m.result); lab.graph.flip(); const u = new Uint16Array(r.data), n = r.w * r.h, f = new Float32Array(n * 3); for (let i = 0; i < n; i++) { f[i * 3] = TABLE[u[i * 4]!]!; f[i * 3 + 1] = TABLE[u[i * 4 + 1]!]!; f[i * 3 + 2] = TABLE[u[i * 4 + 2]!]!; } return f; }
/** median over pixels of |a − b| ÷ (b + a floor), channels summed */
function apart(a: Float32Array, b: Float32Array): number { const n = a.length / 3; let mean = 0; for (let i = 0; i < b.length; i++) mean += b[i]!; const fl = mean / b.length * 0.02 + 1e-5, v = new Float32Array(n); for (let i = 0; i < n; i++) { const o = i * 3; v[i] = (Math.abs(a[o]! - b[o]!) + Math.abs(a[o + 1]! - b[o + 1]!) + Math.abs(a[o + 2]! - b[o + 2]!)) / (b[o]! + b[o + 1]! + b[o + 2]! + 3 * fl); } v.sort(); return v[n >> 1]!; }
/** a softening of both before they are compared (3 × 3), so that the reference's own grain and half a pixel of jitter are not the answer */
function soft(a: Float32Array, w: number, h: number): Float32Array { const o = new Float32Array(a.length); for (let y = 0; y < h; y++) for (let x = 0; x < w; x++) for (let c = 0; c < 3; c++) { let s = 0, n = 0; for (let j = -1; j <= 1; j++) for (let i = -1; i <= 1; i++) { const X = x + i, Y = y + j; if (X < 0 || Y < 0 || X >= w || Y >= h) continue; s += a[(Y * w + X) * 3 + c]!; n++; } o[(y * w + x) * 3 + c] = s / n; } return o; }
const fnv = (f: Float32Array) => { const u = new Uint32Array(f.buffer); let h = 2166136261; for (let i = 0; i < u.length; i++) h = Math.imul(h ^ u[i]!, 16777619); return (h >>> 0).toString(16).padStart(8, '0'); };

export async function bench(lab: Lab, views: (View & { name: string })[], methods: Record<string, () => Method>, o: { reference?: number; settle?: number; mean?: number; stage: (v: View) => void }): Promise<Score[]> {
  const out: Score[] = [], made = new Map<string, Method>(), get = (n: string) => { let m = made.get(n); if (!m) made.set(n, m = methods[n]!()); return m; }, idle = () => lab.gpu.device.queue.onSubmittedWorkDone();
  for (const v of views) {
    o.stage(v); lab.resize([Math.round(v.w * (v.scale ?? 1)), Math.round(v.h * (v.scale ?? 1))], [v.w, v.h]);
    const ref = get('reference'); await lab.use(ref); lab.frameNo = 0; for (let i = 0; i < (o.reference ?? 400); i++) { lab.draw(); if (i % 4 === 3) await idle(); } await idle(); const truth = soft(await picture(lab, ref), v.w, v.h);
    for (const name of Object.keys(methods)) {
      if (name === 'reference') continue; const m = get(name); await lab.use(m); lab.frameNo = 0; lab.graph.timer.ms.clear(); for (let i = 0; i < (o.settle ?? 240); i++) { lab.draw(); if (i % 4 === 3) await idle(); } await idle();
      const one = await picture(lab, m), sum = new Float32Array(one.length), N = o.mean ?? 24; let last = one, noise = 0; sum.set(one);
      for (let i = 1; i < N; i++) { lab.draw(); await idle(); const p = await picture(lab, m); if (i <= 4) noise += apart(p, last) / 4; last = p; for (let k = 0; k < p.length; k++) sum[k]! += p[k]!; } for (let k = 0; k < sum.length; k++) sum[k]! /= N;
      const passes = Object.fromEntries([...lab.graph.timer.ms].map(([k, x]) => [k, +x.toFixed(3)]));
      out.push({ view: v.name, method: name, ms: +Object.values(passes).reduce((a, b) => a + b, 0).toFixed(2), bias: +apart(soft(sum, v.w, v.h), truth).toFixed(4), noise: +noise.toFixed(4), error: +apart(soft(one, v.w, v.h), truth).toFixed(4), hash: fnv(one), passes });
    }
  }
  return out;
}
/** everything a denoiser or an upscaler could learn from at this view: the noisy signals, what is known exactly, the reference; as named float planes */
export async function capture(lab: Lab, method: Method, reference: Method, paths: number): Promise<{ name: string; w: number; h: number; format: string; data: ArrayBuffer }[]> {
  const out: { name: string; w: number; h: number; format: string; data: ArrayBuffer }[] = []; await lab.use(method); lab.draw(); lab.graph.flip(); for (const s of ['surf', 'aux', 'emit', 'front', 'vis', 'lampU', 'bounce', 'mirror', 'color', method.result]) out.push({ name: s, ...await lab.read(s) }); lab.graph.flip();
  await lab.use(reference); for (let i = 0; i < paths; i++) { lab.draw(); if (i % 4 === 3) await lab.gpu.device.queue.onSubmittedWorkDone(); } out.push({ name: 'reference', ...await lab.read(reference.result) }); return out;
}

/** poison every history with NaN and Inf, then see how many frames until the picture holds none, and that it comes back to what it was */
export async function poison(lab: Lab, m: Method): Promise<{ frames: number; apart: number }> {
  await lab.use(m); for (let i = 0; i < 60; i++) { lab.draw(); if (i % 4 === 3) await lab.gpu.device.queue.onSubmittedWorkDone(); } const before = await picture(lab, m), d = lab.gpu.device;
  for (const [name, s] of lab.graph.signals) { if (!s.history || s.format !== 'rgba16float') continue; for (const ref of [name, name + '@prev']) { const t = lab.graph.texture(ref), bad = new Uint16Array(t.width * t.height * 4); for (let i = 0; i < bad.length; i++) bad[i] = i % 3 ? 0x7e00 : 0x7c00; d.queue.writeTexture({ texture: t }, bad, { bytesPerRow: t.width * 8 }, [t.width, t.height]); } }
  { const c = lab.graph.buffers.get('cells'); if (c) { const now = new Float32Array(await lab.readBuffer('cells')); for (let i = 0; i < now.length; i += 8) if (now[i + 3]! > 0) { now[i] = NaN; now[i + 1] = Infinity; now[i + 3] = NaN; now[i + 7] = NaN; } d.queue.writeBuffer(c, 0, now); } const e = lab.graph.buffers.get('exposure'); if (e) d.queue.writeBuffer(e, 0, new Float32Array([NaN, NaN, NaN, NaN])); }
  let frames = -1; for (let i = 0; i < 400; i++) { lab.draw(); await d.queue.onSubmittedWorkDone(); if (frames < 0 && i < 12) { const p = await picture(lab, m); if (!p.some(x => Math.abs(x) > 65000)) frames = i + 1; } } return { frames, apart: apart(await picture(lab, m), before) };
}
