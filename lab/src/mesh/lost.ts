// WHAT LEVELS LOST OF THEIR SHAPE, AS LIGHT: measured once, when a scene is made, by asking the finest mesh and what is drawn and traced in its place the same questions (stages/lost.wgsl). The table lies in
// the meshes' rows, and whoever shades a mesh, or is sent light by one, multiplies by it: so the two sides of a swap send the same in the mean, at every hour.
import type { Lab } from '../lab';
import type { Stage } from '../graph';
import { LOST_ROWS, PX, UPS, VALUES } from './lostsize';
import code from '../stages/lost.wgsl' with { type: 'text' };
export { LOST_ROWS };
const CHUNK = 64, ROUNDS = 16;
/** `tau`, `rays`: as the picture is drawn with them */
export async function measureLost(lab: Lab, tau: number, rays: number): Promise<{ table: Float32Array; ms: number }> {
  const m = lab.building.world.meshes!, d = lab.gpu.device, kinds = m.classes.flatMap((c, i) => Array.from({ length: c.levels }, (_, l) => ({ kind: i * 2 + l, cls: i, level: l, ...c }))), configs = kinds.length * PX * UPS, t0 = performance.now(), was = lab.knobs[1]!;
  lab.buffer('lost', configs * CHUNK * 36 * 4, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST); const k = new Float32Array(8 + 8 * 8), ku = new Uint32Array(k.buffer); k[0] = tau; k[1] = rays; ku[2] = kinds.length; kinds.forEach((q, i) => { k.set(q.lo, 8 + i * 8); ku[8 + i * 8 + 3] = q.level + 1; k.set(q.hi, 8 + i * 8 + 4); ku[8 + i * 8 + 7] = q.cls; });
  const stage: Stage = { name: 'lost', over: { groups: () => [configs, 1, 1] }, buffers: { sums: ['lost', 'array<vec4f>', 'read_write'], lostK: ['lostK', 'LostK', 'uniform'] }, code };
  lab.uniform('lostK', k); lab.knobs[1] = 0; await lab.graph.build([stage]); const table = new Float32Array(LOST_ROWS * 4), keep = () => lab.scene.writeExtra(lab.scene.meshRow + m.lost, new Uint8Array(table.buffer)); table.fill(1); keep();
  for (const pass of [0, 1]) {
    const sum = new Float64Array(configs * 36), rounds = pass ? ROUNDS : ROUNDS / 4; ku[4] = pass;
    for (let r = 0; r < rounds; r++) { ku[3] = r + pass * 8; lab.uniform('lostK', k); lab.sendFrame(); const e = d.createCommandEncoder(); lab.graph.run(e, [stage]); d.queue.submit([e.finish()]); const f = new Float32Array(await lab.readBuffer('lost')); for (let c = 0; c < configs; c++) for (let t = 0; t < CHUNK; t++) for (let v = 0; v < 36; v++) sum[c * 36 + v]! += f[(c * CHUNK + t) * 36 + v]!; }
    kinds.forEach((q, i) => { for (let p = 0; p < PX * UPS; p++) for (let v = 0; v < VALUES; v++) { const o = (i * PX * UPS + p) * 36, a = sum[o + v]! / Math.max(1, sum[o + 32]!), b = sum[o + 16 + v]! / Math.max(1, sum[o + 33]!); table[(q.kind * PX * UPS + p) * 16 + v] = b > 1e-6 ? Math.max(0.2, Math.min(1.8, a / b)) : 1; } }); keep();
  }
  lab.knobs[1] = was; return { table, ms: performance.now() - t0 };
}
