// WHERE EACH LAMP CAN REACH, found once: rays from every standing lamp mark the metre cells they pass; what is marked grows by a cell, and is kept as a picture of
// weights, four lamps a texel, read with the sampler's own blending. A lamp's weight is 1 wherever it can be seen and falls to 0 smoothly where it can not, so
// leaving it out changes nothing but the cost and the noise. Doors do not count: they open.
import type { Lab } from './lab';
import { buffer } from './gpu';
export const LM = { at: [-20, -1, -16], cells: [40, 16, 32], size: [1, 0.85, 1], layers: 16 } as const;
export const lampMapCode = `
const LM_AT = vec3f(${LM.at.join('.0, ')}.0); const LM_SIZE = vec3f(1.0, 0.85, 1.0); const LM_CELLS = vec3f(${LM.cells.join('.0, ')}.0); const LM_LAYERS = ${LM.layers}.0;
/** how far lamps 4 g … 4 g + 3 reach here, 0‥1 */
fn lampReach(x: vec3f, g: u32) -> vec4f { let q = clamp((x - LM_AT) / LM_SIZE, vec3f(0.5), LM_CELLS - 0.5); return textureSampleLevel(lampTex, linSamp, vec3f(q.xy / LM_CELLS.xy, (q.z + f32(g) * LM_CELLS.z) / (LM_CELLS.z * LM_LAYERS)), 0.0); }
`;
const mark = `
@group(1) @binding(0) var<storage, read_write> seen: array<atomic<u32>>; @group(1) @binding(1) var<uniform> some: vec4u;
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) {
  let i = id.y * some.y + some.x; if (i >= frame.counts.x) { return; } let l = &lamps[i]; rngKey(id.x, 1000u + i); leafMode = 2u; var o = (*l).at; var d = vec3f(0.0, 1.0, 0.0);
  let u = rnd2(); let z = u.x * 2.0 - 1.0; let r = sqrt(max(1.0 - z * z, 0.0)); d = vec3f(r * cos(6.2832 * u.y), z, r * sin(6.2832 * u.y));
  if ((*l).kind == 0u) { if (dot(d, (*l).n) < 0.0) { d = -d; } let p = rnd2() * 2.0 - 1.0; o += (*l).u * p.x + (*l).v * p.y + (*l).n * 0.02; } else { o += d * ((*l).area + 0.02); }
  let h = nearest(o, d, 1e-3, 60.0, NOSHADOW | MOVES | WATER, 1e9); let end = min(h.t, 60.0);
  for (var t = 0.0; t < end + 0.3; t += 0.3) {
    let q = (o + d * min(t, end) - LM_AT) / LM_SIZE; if (any(q < vec3f(0.0)) || any(q >= LM_CELLS)) { continue; } let c = vec3u(q); let at = ((c.z * ${LM.cells[1]}u + c.y) * ${LM.cells[0]}u + c.x) * 2u + (i >> 5u); let b = 1u << (i & 31u); if ((atomicLoad(&seen[at]) & b) == 0u) { atomicOr(&seen[at], b); }
  }
}`;
const grow = `
@group(0) @binding(0) var<storage, read> seen: array<u32>; @group(0) @binding(1) var out: texture_storage_3d<rgba8unorm, write>; @group(0) @binding(2) var<uniform> always: vec4u;
@compute @workgroup_size(4, 4, 4) fn main(@builtin(global_invocation_id) id: vec3u) {
  let n = vec3i(${LM.cells.join(', ')}); if (any(vec2i(id.xy) >= n.xy)) { return; } let g = i32(id.z) / n.z; let z = i32(id.z) % n.z; var m = always.x; if (g >= 8) { m = always.y; }
  for (var k = 0; k < 27; k++) { let c = vec3i(i32(id.x), i32(id.y), z) + vec3i(k % 3, (k / 3) % 3, k / 9) - 1; if (any(c < vec3i(0)) || any(c >= n)) { continue; } m |= seen[u32((c.z * n.y + c.y) * n.x + c.x) * 2u + select(0u, 1u, g >= 8)]; }
  let s = u32(g & 7) * 4u; textureStore(out, id, vec4f((vec4u(m) >> vec4u(s, s + 1u, s + 2u, s + 3u)) & vec4u(1u)));
}`;
const made = new WeakMap<Lab, (part: number, of: number) => void>();
/** found when the scene is opened; when a wall has gone, more rays a few lamps at a time (what opens only adds) */
export async function mapLamps(lab: Lab, rays = 6144) {
  const d = lab.gpu.device, C = GPUShaderStage.COMPUTE, n = LM.cells[0] * LM.cells[1] * LM.cells[2], seen = buffer(lab.gpu, 'lamps seen', n * 8, GPUBufferUsage.STORAGE), lamps = lab.building.world.lamps.slice(0, 64);
  const always = new Uint32Array(4); for (let i = 0; i < 64; i++) if (!lamps[i] || lamps[i]!.kind === 'spot' || lamps[i]!.name === 'fire') always[i >> 5]! |= 1 << (i & 31); const al = buffer(lab.gpu, 'always', always, GPUBufferUsage.UNIFORM);   // torches go anywhere; and lamps made later
  const l1 = d.createBindGroupLayout({ entries: [{ binding: 0, visibility: C, buffer: { type: 'storage' } }, { binding: 1, visibility: C, buffer: { type: 'uniform' } }] }), some = buffer(lab.gpu, 'lamps some', 16, GPUBufferUsage.UNIFORM), l2 = d.createBindGroupLayout({ entries: [{ binding: 0, visibility: C, buffer: { type: 'read-only-storage' } }, { binding: 1, visibility: C, storageTexture: { format: 'rgba8unorm', viewDimension: '3d' } }, { binding: 2, visibility: C, buffer: { type: 'uniform' } }] });
  const pipe = (code: string, ...bindGroupLayouts: GPUBindGroupLayout[]) => d.createComputePipelineAsync({ label: 'lamp map', layout: d.createPipelineLayout({ bindGroupLayouts }), compute: { module: d.createShaderModule({ label: 'lamp map', code }), entryPoint: 'main' } });
  const [a, b] = await Promise.all([pipe(lab.graph.prelude() + mark, lab.scene.layout, l1), pipe(grow, l2)]);
  const go = (part: number, of: number) => { d.queue.writeBuffer(some, 0, new Uint32Array([part, of, 0, 0])); const e = d.createCommandEncoder(); let p = e.beginComputePass(); p.setPipeline(a); p.setBindGroup(0, lab.scene.group()); p.setBindGroup(1, d.createBindGroup({ layout: l1, entries: [{ binding: 0, resource: { buffer: seen } }, { binding: 1, resource: { buffer: some } }] })); p.dispatchWorkgroups(rays / 64, Math.ceil(lamps.length / of)); p.end();
  if (part === 0) { p = e.beginComputePass(); p.setPipeline(b); p.setBindGroup(0, d.createBindGroup({ layout: l2, entries: [{ binding: 0, resource: { buffer: seen } }, { binding: 1, resource: lab.scene.lampTex.createView() }, { binding: 2, resource: { buffer: al } }] })); p.dispatchWorkgroups(LM.cells[0] / 4, LM.cells[1] / 4, LM.cells[2] * LM.layers / 4); p.end(); }
  d.queue.submit([e.finish()]); }; made.set(lab, go); go(0, 1); await d.queue.onSubmittedWorkDone();
}
export const lampsAgain = (lab: Lab, part: number, of: number) => made.get(lab)?.(part, of);
