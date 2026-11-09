// THE TREES IN THE WIND, on the GPU: each spray leans by the air where it is. The leaves' rows are made from their rest and handed to everything that draws or meets them, and last frame's are
// kept for motion. The CPU does nothing for a leaf.
import type { Lab } from './lab';
import { buffer } from './gpu';
import { BREEZE } from './scene/air';
const code = (n: number) => `
struct Rest { r0: vec4f, r1: vec4f, r2: vec4f, info: vec4u, at: vec4f, base: vec4f };      // turned, without its place · node, per, leaf, tint · place and size · what it sways about, how much; phase in r0.w
@group(1) @binding(0) var<storage, read> rest: array<Rest>; @group(1) @binding(1) var<storage, read_write> rows: array<vec4f>; @group(1) @binding(2) var<storage, read_write> past: array<vec4f>;
fn leanBy(v: vec3f, a: f32, b: f32) -> vec3f { let cx = cos(b); let sx = sin(b); let p = vec3f(v.x, cx * v.y - sx * v.z, sx * v.y + cx * v.z); let cz = cos(a); let sz = sin(a); return vec3f(cz * p.x - sz * p.y, sz * p.x + cz * p.y, p.z); }
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) {
  let i = id.x; if (i >= ${n}u) { return; } let s = rest[i]; let t = frame.misc.x; let ph = s.r0.w; let w = windAt(s.at.xyz) * ${(1.35 / BREEZE).toFixed(5)}; let force = length(w.xz); let flick = min(1.5, force) * (sin(t * 2.3 + ph) * 0.35 + sin(t * 3.7 + ph * 1.7) * 0.2); let q = (1.0 + 0.2 * sin(t * 1.9 + ph) + flick * 0.4) * s.base.w;
  let ax = w.x * q; let az = w.z * q; let h = s.at.y - s.base.y; let at = s.at.xyz + clamp(vec3f(ax * h * 0.55, -abs(ax) * h * 0.05, az * h * 0.55), vec3f(-0.33), vec3f(0.33)); let a = -ax * 1.6 + flick * 0.03; let b = az * 1.6 + flick * 0.03; let k = 1.0 / s.at.w; let o = i * 4u;
  for (var j = 0u; j < 3u; j++) { past[i * 3u + j] = rows[o + j]; var r = s.r0.xyz; if (j == 1u) { r = s.r1.xyz; } if (j == 2u) { r = s.r2.xyz; } let x = leanBy(r, a, b) * k; rows[o + j] = vec4f(x, -dot(x, at)); }
}`;
export class Sway {
  private pipe!: GPUComputePipeline; private bg!: GPUBindGroup; private now!: GPUBuffer; private n = 0;
  async init(lab: Lab, rest: Float32Array<ArrayBuffer>, rows: Float32Array<ArrayBuffer>, past: GPUBuffer) {
    this.n = lab.scene.counts.instances; if (!this.n) return; const d = lab.gpu.device, C = GPUShaderStage.COMPUTE, b = (binding: number, type: GPUBufferBindingType): GPUBindGroupLayoutEntry => ({ binding, visibility: C, buffer: { type } }), l = d.createBindGroupLayout({ entries: [b(0, 'read-only-storage'), b(1, 'storage'), b(2, 'storage')] }), r = buffer(lab.gpu, 'leaves at rest', rest, GPUBufferUsage.STORAGE); this.now = buffer(lab.gpu, 'leaves now', rows, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC);
    const mod = d.createShaderModule({ label: 'sway', code: lab.graph.prelude() + code(this.n) }), bad = (await mod.getCompilationInfo()).messages.find(m => m.type === 'error'); if (bad) throw new Error(`sway:${bad.lineNum} ${bad.message}`); this.pipe = await d.createComputePipelineAsync({ label: 'sway', layout: d.createPipelineLayout({ bindGroupLayouts: [lab.scene.layout, l] }), compute: { module: mod, entryPoint: 'main' } }); this.bg = d.createBindGroup({ layout: l, entries: [{ binding: 0, resource: { buffer: r } }, { binding: 1, resource: { buffer: this.now } }, { binding: 2, resource: { buffer: past } }] });
  }
  run(e: GPUCommandEncoder, lab: Lab) { if (!this.n) return; const p = e.beginComputePass({ label: 'sway' }); p.setPipeline(this.pipe); p.setBindGroup(0, lab.scene.group()); p.setBindGroup(1, this.bg); p.dispatchWorkgroups(Math.ceil(this.n / 64)); p.end(); e.copyBufferToBuffer(this.now, 0, lab.scene.insts, 0, this.n * 64); }
}
