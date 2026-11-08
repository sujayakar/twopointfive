// THE WATCH, for development: after every stage, every float it wrote is looked at, and the first stage to write a bad number is named, with where and when.
// Nothing is cleaned while this is on, so that what is seen is what was made. It is a pass of its own after each stage: where there is no watch it costs nothing.
import type { Graph, Stage } from './graph';
import { buffer, readBuffer } from './gpu';

const code = (tex: boolean) => `
@group(0) @binding(0) ${tex ? 'var t: texture_2d<f32>;' : 'var<storage, read> t: array<u32>;'}
@group(0) @binding(1) var<storage, read_write> found: array<atomic<u32>>;
@group(0) @binding(2) var<uniform> slot: vec4u;
fn bad(b: u32) -> bool { return (b & 0x7f800000u) == 0x7f800000u; }
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) {
  ${tex ? 'let d = textureDimensions(t); let i = id.x + id.y * 4096u; if (i >= d.x * d.y) { return; } let b = bitcast<vec4u>(textureLoad(t, vec2u(i % d.x, i / d.x), 0)); if (bad(b.x) || bad(b.y) || bad(b.z) || bad(b.w)) {' : 'let i = id.x + id.y * 4096u; if (i >= arrayLength(&t)) { return; } if (bad(t[i])) {'}
    atomicAdd(&found[slot.x * 2u], 1u); atomicMax(&found[slot.x * 2u + 1u], 0xffffffffu - i); }
}`;
export interface Bad { frame: number; stage: string; what: string; count: number; first: number; width: number }
export class Watch {
  private found: GPUBuffer; private pipes!: [GPUComputePipeline, GPUComputePipeline]; private layouts: [GPUBindGroupLayout, GPUBindGroupLayout]; private seen: { stage: string; what: string; width: number }[] = []; private slots: GPUBuffer[] = [];
  /** the first frame that went wrong: its offenders, in the order the stages ran */
  bad: Bad[] = []; frames = 0; looked = 0;
  constructor(readonly g: Graph) {
    const C = GPUShaderStage.COMPUTE, rest: GPUBindGroupLayoutEntry[] = [{ binding: 1, visibility: C, buffer: { type: 'storage' } }, { binding: 2, visibility: C, buffer: { type: 'uniform' } }], d = g.g.device;
    this.found = buffer(g.g, 'watch', 4096, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST);
    this.layouts = [d.createBindGroupLayout({ entries: [{ binding: 0, visibility: C, texture: { sampleType: 'unfilterable-float' } }, ...rest] }), d.createBindGroupLayout({ entries: [{ binding: 0, visibility: C, buffer: { type: 'read-only-storage' } }, ...rest] })];
  }
  async init() { this.pipes = await Promise.all([true, false].map((tex, i) => this.g.g.device.createComputePipelineAsync({ label: 'watch', layout: this.g.g.device.createPipelineLayout({ bindGroupLayouts: [this.layouts[i]!] }), compute: { module: this.g.g.device.createShaderModule({ code: code(tex) }), entryPoint: 'main' } }))) as [GPUComputePipeline, GPUComputePipeline]; return this; }
  after(enc: GPUCommandEncoder, s: Stage) {
    if (this.seen.length === 0) enc.clearBuffer(this.found);
    for (const n of Object.values(s.writes ?? {})) { const t = this.g.texture(n); if (t.dimension === '3d') continue; if (t.format.includes('float') || t.format.includes('norm')) this.look(enc, s.name, n, 0, this.g.view(n), t.width, t.width * t.height); }
    for (const [b, type, access] of Object.values(s.buffers ?? {})) if (access === 'read_write' && /^array<(vec4f|f32)/.test(type)) { const q = this.g.buffers.get(b)!; this.look(enc, s.name, b, 1, { buffer: q }, 0, q.size / 4); }
  }
  private look(enc: GPUCommandEncoder, stage: string, what: string, kind: 0 | 1, resource: GPUBindingResource, width: number, n: number) {
    const d = this.g.g.device, i = this.seen.push({ stage, what, width }) - 1; let u = this.slots[i]; if (!u) { this.slots[i] = u = buffer(this.g.g, 'watch slot', 16, GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST); d.queue.writeBuffer(u, 0, new Uint32Array([i, 0, 0, 0])); }
    const p = enc.beginComputePass({ label: 'watch' }); p.setPipeline(this.pipes[kind]); p.setBindGroup(0, d.createBindGroup({ layout: this.layouts[kind], entries: [{ binding: 0, resource }, { binding: 1, resource: { buffer: this.found } }, { binding: 2, resource: { buffer: u } }] }));
    p.dispatchWorkgroups(64, Math.ceil(n / 4096)); p.end();
  }
  /** after a frame is submitted: what it found */
  async read(): Promise<Bad[]> {
    const seen = this.seen, now: Bad[] = []; this.seen = []; const f = new Uint32Array(await readBuffer(this.g.g, this.found)); this.frames++; this.looked = seen.length;
    seen.forEach((x, i) => { if (f[2 * i]) now.push({ frame: this.frames, ...x, count: f[2 * i]!, first: 0xffffffff - f[2 * i + 1]! }); }); if (!this.bad.length) this.bad = now; return now;
  }
  say = () => this.bad.length === 0 ? `${this.frames} frames, ${this.looked} outputs each, no bad number` : `frame ${this.bad[0]!.frame}: ` + this.bad.slice(0, 4).map(b => `${b.stage} wrote ${b.count} bad into ${b.what}${b.width ? ` at ${b.first % b.width},${Math.floor(b.first / b.width)}` : ` at ${b.first}`}`).join('; then ');
}
