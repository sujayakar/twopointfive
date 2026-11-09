// WHAT IS JOINED TO WHAT, found once: a grid that says between which neighbouring cells something stands. Two places near each other are joined if the straight way
// between them crosses nothing. The light table asks it before it lets one place read what was found at another: walls are thinner than its cells.
import type { Lab } from './lab';
import { buffer, readBuffer } from './gpu';
export const JN = { at: [-16, -0.5, -10.4], h: 0.2, n: [160, 48, 104] } as const;
const TOTAL = JN.n[0] * JN.n[1] * JN.n[2], AT = `vec3f(${JN.at.map(v => v.toFixed(2)).join(', ')})`;
export const JOIN_ROWS = Math.ceil(TOTAL / 32);
export const joinCode = `
fn shutAt(c: vec3i, a: u32) -> bool { if (any(c < vec3i(0)) || any(c >= vec3i(${JN.n.join(', ')}))) { return false; } let i = u32(c.x + ${JN.n[0]} * (c.y + ${JN.n[1]} * c.z)); let w = bitcast<vec4u>(extra[u32(frame.knobs.z) + (i >> 5u)])[(i >> 3u) & 3u]; return ((w >> (4u * (i & 7u) + a)) & 1u) != 0u; }
/** is the straight way from a to b (near each other) clear, as far as the grid can tell? */
fn joined(a: vec3f, b: vec3f) -> bool {
  let p = (a - ${AT}) / ${JN.h}; let q = (b - ${AT}) / ${JN.h}; var c = vec3i(floor(p)); if (all(c == vec3i(floor(q)))) { return true; } let m = min(12.0, ceil(length(q - p) * 1.5));
  for (var s = 1.0; s <= m; s += 1.0) { let d = vec3i(floor(mix(p, q, s / m))); for (var k = 0u; k < 3u; k++) { if (d[k] == c[k]) { continue; } let up = d[k] > c[k]; if (!up) { c[k] -= 1; } if (shutAt(c, k)) { return false; } if (up) { c[k] += 1; } } } return true;
}
`;
const mark = `
@group(1) @binding(0) var<storage, read_write> shut: array<u32>;
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) {
  let w = id.x + id.y * 65536u; if (w >= ${Math.ceil(TOTAL / 8)}u) { return; } var bits = 0u; leafMode = 2u;
  for (var k = 0u; k < 8u; k++) { let i = w * 8u + k; if (i >= ${TOTAL}u) { break; } let c = vec3u(i % ${JN.n[0]}u, (i / ${JN.n[0]}u) % ${JN.n[1]}u, i / ${JN.n[0] * JN.n[1]}u); let x = ${AT} + (vec3f(c) + 0.5) * ${JN.h};
    for (var a = 0u; a < 3u; a++) { var d = vec3f(0.0); d[a] = 1.0; let h = nearest(x, d, 0.0, ${JN.h}, MOVES | WATER, 1e9); if (h.piece != NOTHING && (h.piece & LEAF_HIT) == 0u) { bits |= 1u << (4u * k + a); } } }
  shut[w] = bits;
}`;
export async function mapJoins(lab: Lab) {
  const d = lab.gpu.device, C = GPUShaderStage.COMPUTE, words = Math.ceil(TOTAL / 8), out = buffer(lab.gpu, 'joins', JOIN_ROWS * 16, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC), l1 = d.createBindGroupLayout({ entries: [{ binding: 0, visibility: C, buffer: { type: 'storage' } }] });
  const pipe = await d.createComputePipelineAsync({ label: 'joins', layout: d.createPipelineLayout({ bindGroupLayouts: [lab.scene.layout, l1] }), compute: { module: d.createShaderModule({ label: 'joins', code: lab.graph.prelude() + mark }), entryPoint: 'main' } }), e = d.createCommandEncoder(), p = e.beginComputePass(); p.setPipeline(pipe); p.setBindGroup(0, lab.scene.group()); p.setBindGroup(1, d.createBindGroup({ layout: l1, entries: [{ binding: 0, resource: { buffer: out } }] })); p.dispatchWorkgroups(1024, Math.ceil(words / 65536)); p.end(); d.queue.submit([e.finish()]);
  lab.scene.writeExtra(lab.scene.joinRow, new Uint8Array(await readBuffer(lab.gpu, out))); out.destroy();
}
