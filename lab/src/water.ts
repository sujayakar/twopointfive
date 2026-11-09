// What the waves do to the sun on the pool's floor: a quarter of a million points of the surface, each bent down and counted where it lands. Whole numbers are
// added, so the order does not matter and a run repeats.
import type { Gpu } from './gpu';
import type { V3 } from './math';
import { rippleCode, type Ripples } from './scene/waves';
import math from './wgsl/math.wgsl' with { type: 'text' };
import waves from './wgsl/waves.wgsl' with { type: 'text' };
const N = 512, P = 768;
export class Water {
  readonly tex: GPUTexture; private buf: GPUBuffer; private knob: GPUBuffer; private splat!: GPUComputePipeline; private spread!: GPUComputePipeline; private g1!: GPUBindGroup; private g2!: GPUBindGroup;
  private rip: GPUBuffer; private sent = -1; constructor(private g: Gpu, private ripples: Ripples) { const d = g.device; this.rip = d.createBuffer({ size: ripples.h.byteLength, usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST }); this.tex = d.createTexture({ label: 'caustic', size: [N, N], format: 'rgba16float', usage: GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.TEXTURE_BINDING }); this.buf = d.createBuffer({ size: N * N * 4, usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST }); this.knob = d.createBuffer({ size: 32, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST }); }
  async init() {
    const d = this.g.device, head = math + waves + rippleCode(this.ripples, 'rip[i]', 'k.t.z') + `struct K { sun: vec4f, t: vec4f }; @group(0) @binding(0) var<uniform> k: K;\nconst N = ${N}u; const P = ${P}u;\n`;
    const a = d.createShaderModule({ label: 'splat', code: head + `@group(0) @binding(2) var<storage, read> rip: array<f32>; @group(0) @binding(1) var<storage, read_write> count: array<atomic<u32>>;
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let o = vec2f(${this.ripples.lo[0].toFixed(3)}, ${this.ripples.lo[1].toFixed(3)}); let p = o + fract(((vec2f(id.xy) + 0.5) / f32(P) * WAVE_TILE - o) / WAVE_TILE) * WAVE_TILE; /* (a tile of the waves, as the part of it over the pool) */ let L = normalize(vec3f(k.sun.x, max(k.sun.y, 0.25), k.sun.z)); let n = waveNormal(p, k.t.x); let r = refract(-L, n, 0.75); let r0 = refract(-L, vec3f(0.0, 1.0, 0.0), 0.75);
  let q = p + (r.xz / min(r.y, -0.2) - r0.xz / min(r0.y, -0.2)) * -k.t.y; let w = fract(q / WAVE_TILE) * f32(N) - 0.5; let b = floor(w); let f = w - b;
  for (var i = 0u; i < 4u; i++) { let c = vec2u((vec2i(b) + vec2i(i32(i & 1u), i32(i >> 1u)) + vec2i(i32(N))) % vec2i(i32(N))); let s = select(1.0 - f.x, f.x, (i & 1u) == 1u) * select(1.0 - f.y, f.y, i > 1u); atomicAdd(&count[c.y * N + c.x], u32(s * 256.0 + 0.5)); }
}` });
    const b = d.createShaderModule({ label: 'spread', code: math + waves.replace(/ \+ ripple(Height|Slope)\(p\)/g, '') + `const N = ${N}u; const P = ${P}u;\n@group(0) @binding(1) var<storage, read> count: array<u32>; @group(0) @binding(2) var out: texture_storage_2d<rgba16float, write>;
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) { var s = 0.0; var ws = 0.0; for (var y = -1; y <= 1; y++) { for (var x = -1; x <= 1; x++) { let c = vec2u((vec2i(id.xy) + vec2i(x, y) + vec2i(i32(N))) % vec2i(i32(N))); let w = f32((2 - abs(x)) * (2 - abs(y))); s += w * f32(count[c.y * N + c.x]); ws += w; } } let e = 256.0 * f32(P * P) / f32(N * N); textureStore(out, id.xy, vec4f(s / ws / e, 0.0, 0.0, 1.0)); }` });
    for (const m of [a, b]) { const e = (await m.getCompilationInfo()).messages.find(x => x.type === 'error'); if (e) throw new Error(`water:${e.lineNum} ${e.message}`); }
    this.splat = await d.createComputePipelineAsync({ layout: 'auto', compute: { module: a, entryPoint: 'main' } }); this.spread = await d.createComputePipelineAsync({ layout: 'auto', compute: { module: b, entryPoint: 'main' } });
    this.g1 = d.createBindGroup({ layout: this.splat.getBindGroupLayout(0), entries: [{ binding: 0, resource: { buffer: this.knob } }, { binding: 1, resource: { buffer: this.buf } }, { binding: 2, resource: { buffer: this.rip } }] }); this.g2 = d.createBindGroup({ layout: this.spread.getBindGroupLayout(0), entries: [{ binding: 1, resource: { buffer: this.buf } }, { binding: 2, resource: this.tex.createView() }] });
  }
  run(e: GPUCommandEncoder, t: number, sun: V3, deep: number) {
    this.g.device.queue.writeBuffer(this.knob, 0, new Float32Array([sun[0], sun[1], sun[2], 0, t, deep * 1.15, this.ripples.live ? 1 : 0, 0])); if (this.sent !== this.ripples.version) { this.sent = this.ripples.version; this.g.device.queue.writeBuffer(this.rip, 0, this.ripples.h); } e.clearBuffer(this.buf);
    const p = e.beginComputePass({ label: 'caustics' }); p.setPipeline(this.splat); p.setBindGroup(0, this.g1); p.dispatchWorkgroups(P / 8, P / 8); p.setPipeline(this.spread); p.setBindGroup(0, this.g2); p.dispatchWorkgroups(N / 8, N / 8); p.end();
  }
}
