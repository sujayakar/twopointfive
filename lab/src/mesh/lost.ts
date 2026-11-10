// WHAT LEVELS LOST OF THEIR SHAPE, AS LIGHT: measured once, when a scene is made, by asking the finest mesh and what is drawn and traced in its place the same questions (stages/lost.wgsl). The table lies in
// the meshes' rows, and whoever shades a mesh, or is sent light by one, multiplies by it: so the two sides of a swap send the same in the mean, at every hour.
import type { Lab } from '../lab';
import type { Stage } from '../graph';
import { LOST_ROWS, PX, UPS, VALUES } from './lostsize';
import code from '../stages/lost.wgsl' with { type: 'text' };
export { LOST_ROWS };
const CHUNK = 64, ROUNDS = 16, AT_ONCE = 4, N = 68;
/** `tau`, `rays`: as the picture is drawn with them */
export async function measureLost(lab: Lab, tau: number, rays: number): Promise<{ table: Float32Array; ms: number }> {
  const m = lab.building.world.meshes!, d = lab.gpu.device, kinds = m.classes.flatMap((c, i) => Array.from({ length: c.levels }, (_, l) => ({ kind: i * 2 + l, cls: i, level: l, ...c }))), configs = kinds.length * PX * UPS, t0 = performance.now(), was = lab.knobs[1]!;
  lab.buffer('lost', configs * AT_ONCE * CHUNK * N * 4, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST); const k = new Float32Array(8 + 20 * 8), ku = new Uint32Array(k.buffer); k[0] = tau; k[1] = Math.max(0, rays); ku[2] = kinds.length; kinds.forEach((q, i) => { k.set(q.lo, 8 + i * 8); ku[8 + i * 8 + 3] = q.level + 1; k.set(q.hi, 8 + i * 8 + 4); ku[8 + i * 8 + 7] = q.cls; });
  const stage: Stage = { name: 'lost', over: { groups: () => [configs, AT_ONCE, 1] }, buffers: { sums: ['lost', 'array<vec4f>', 'read_write'], lostK: ['lostK', 'LostK', 'uniform'] }, code };
  lab.uniform('lostK', k); lab.knobs[1] = Math.min(0, rays); await lab.graph.build([stage]); const table = new Float32Array(LOST_ROWS * 4), keep = () => lab.scene.writeExtra(lab.scene.meshRow + m.lost, new Uint8Array(table.buffer)); table.fill(1); keep();
  for (const pass of [0, 1]) {
    const sum = new Float64Array(configs * N), rounds = pass ? ROUNDS : ROUNDS / 4; ku[4] = pass; ku[5] = configs;
    for (let r = 0; r < rounds; r += AT_ONCE) { ku[3] = r + pass * 8; lab.uniform('lostK', k); lab.sendFrame(); const e = d.createCommandEncoder(); lab.graph.run(e, [stage]); d.queue.submit([e.finish()]); const f = new Float32Array(await lab.readBuffer('lost')); for (let c = 0, o = 0; c < configs * AT_ONCE; c++) for (let t = 0, at = (c % configs) * N; t < CHUNK; t++) for (let v = 0; v < N; v++) sum[at + v]! += f[o++]!; }
    kinds.forEach((q, i) => { for (let p = 0; p < PX * UPS; p++) for (let v = 0; v < VALUES; v++) { const o = (i * PX * UPS + p) * N, nf = Math.max(1, sum[o + 32]!), nl = Math.max(1, sum[o + 33]!), at = (j: number, n: number) => sum[o + j]! / n, hold = (x: number) => Math.max(0.2, Math.min(1.8, x)), round = at(31, nl) > 1e-9 ? hold(at(15, nf) / at(31, nl)) : 1;
        // (of the sun: direct, and by way of what stands near. That way's light will be taken by what is taken off all that comes from round about: the direct makes up for what is wrong with it)
        table[(q.kind * PX * UPS + p) * 16 + v] = v === 15 ? round : at(16 + v, nl) > 1e-9 ? hold((at(v, nf) + at(36 + v, nf) - round * at(52 + v, nl)) / at(16 + v, nl)) : 1; } }); keep();
  }
  lab.knobs[1] = was; return { table, ms: performance.now() - t0 };
}
