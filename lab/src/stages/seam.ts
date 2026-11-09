// Asks the seam: what becomes of shadows and bounced light when rays meet coarser triangles than are drawn. See seam.wgsl.
import type { Lab } from '../lab';
import type { Stage } from '../graph';
import bounceLib from '../wgsl/bounce.wgsl' with { type: 'text' };
import code from './seam.wgsl' with { type: 'text' };
/** how a ray starts */
export const START = { off: 1, back: 2, depth: 4, foot: 8 };
export interface Config { name: string; /** levels coarser than a pixel's (each half the triangles); −1: the finest */ more: number; how: number; /** how many errors off */ far: number; salt?: number }
export interface SeamRow { name: string; error: number; /** of shadows: mean |difference| of the seen share where the sun could be, too dark, too light, and pixels more than a tenth off; and of what comes back in the round, of its mean */ shadow: number; dark: number; light: number; off: number; bounce: number; bias: number }
/** only what it returns to a wide ray is asked of bounce.wgsl */
const returnsOnly = bounceLib.slice(bounceLib.indexOf('fn returns'), bounceLib.indexOf('fn sentBack'));
export async function seam(lab: Lab, configs: Config[], frames: number, mapOf = -1): Promise<{ rows: SeamRow[]; pixels: number; map?: Float32Array }> {
  const g = lab.graph, [w, h] = g.output, n = w * h, d = lab.gpu.device, U = GPUBufferUsage; lab.buffer('seamFirst', n * 48); lab.buffer('seamSums', n * 16 * configs.length, U.STORAGE | U.COPY_SRC);
  const k = new Float32Array(4 + 48 * 4), ku = new Uint32Array(k.buffer), cam = lab.camera.frame(w / h); ku[0] = configs.length; k[2] = 2 * Math.hypot(...cam.up) / h; configs.forEach((c, i) => k.set([c.more, c.how, c.far, c.salt ?? 0], 4 + i * 4));
  const stage: Stage = { name: 'seam', over: 'output', buffers: { first: ['seamFirst', 'array<vec4u>', 'read_write'], sums: ['seamSums', 'array<vec4f>', 'read_write'], seamK: ['seamK', 'SeamK', 'uniform'] }, code: returnsOnly + code };
  lab.uniform('seamK', k); await g.build([stage]);
  for (let f = 0; f <= frames; f++) { ku[1] = f; lab.uniform('seamK', k); lab.sendFrame(); const e = d.createCommandEncoder(); g.run(e, [stage]); d.queue.submit([e.finish()]); if (f % 4 === 3) await d.queue.onSubmittedWorkDone(); }
  const first = new Uint32Array(await lab.readBuffer('seamFirst')), sums = new Float32Array(await lab.readBuffer('seamSums')), px: number[] = []; for (let i = 0; i < n; i++) if (first[i * 12 + 3]) px.push(i);
  let map: Float32Array | undefined; if (mapOf >= 0) { map = new Float32Array(n * 2); for (const i of px) { map[i * 2] = (sums[(mapOf * n + i) * 4]! - sums[i * 4]!) / frames; map[i * 2 + 1] = (sums[(mapOf * n + i) * 4 + 1]! - sums[i * 4 + 1]!) / frames; } }
  // (bounced light is judged by the mean of eight by eight pixels: what a single one gets of it is dice)
  const block = new Map<number, number[]>(); for (const i of px) { const b = Math.floor(i % w / 8) + 4096 * Math.floor(i / w / 8); let l = block.get(b); if (!l) block.set(b, l = []); l.push(i); }
  let mean = 0; for (const i of px) mean += sums[i * 4 + 1]! / frames / px.length;
  const rows = configs.map((c, ci) => { let s = 0, dk = 0, lt = 0, of = 0, b = 0, bi = 0, ns = 0, er = 0; for (const i of px) { const o = (ci * n + i) * 4, sunny = sums[i * 4 + 2]! > 0, q = (sums[o]! - sums[i * 4]!) / frames, p = (sums[o + 1]! - sums[i * 4 + 1]!) / frames; er += sums[o + 3]! / frames; if (sunny) { ns++; s += Math.abs(q); if (q < 0) dk -= q; else lt += q; if (Math.abs(q) > 0.1) of++; } bi += p; } for (const l of block.values()) { let p = 0; for (const i of l) p += (sums[(ci * n + i) * 4 + 1]! - sums[i * 4 + 1]!) / frames; b += Math.abs(p); } return { name: c.name, error: er / px.length, shadow: s / ns, dark: dk / ns, light: lt / ns, off: of / ns, bounce: b / px.length / mean, bias: bi / px.length / mean }; });
  return { rows, pixels: px.length, ...(map ? { map } : {}) };
}
