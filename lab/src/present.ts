// The last step: exposure, a tone curve that keeps strong colors strong, and the canvas.
import type { Gpu } from './gpu';
import type { Lab } from './lab';
const CURVE = `
fn neutral(c0: vec3f) -> vec3f {                 // Khronos PBR Neutral: base colors come through as painted; only what is brighter is rolled off
  let start = 0.8 - 0.04; let des = 0.15; let x = min(c0.r, min(c0.g, c0.b)); let off = select(0.04, x - 6.25 * x * x, x < 0.08); var c = c0 - off;
  let peak = max(c.r, max(c.g, c.b)); if (peak < start) { return c; } let d = 1.0 - start; let np = 1.0 - d * d / (peak + d - start); c *= np / peak; let g = 1.0 - 1.0 / (des * (peak - np) + 1.0); return mix(c, vec3f(np), g);
}
fn srgb(c: vec3f) -> vec3f { return select(1.055 * pow(max(c, vec3f(0.0)), vec3f(1.0 / 2.4)) - 0.055, c * 12.92, c <= vec3f(0.0031308)); }
fn grade(c0: vec3f) -> vec3f { var c = c0; let l = dot(c, vec3f(0.2126, 0.7152, 0.0722)); c = max(vec3f(0.0), l + (c - l) * 1.06); let y = c; c = mix(c, c * c * (3.0 - 2.0 * min(c, vec3f(1.0))), 0.0); return y; }
`;
export class Present {
  private pipe!: GPURenderPipeline; private meter!: GPUComputePipeline; private exp: GPUBuffer; private knob: GPUBuffer; bias = 0; private pipes = new Map<string, GPURenderPipeline>(); private sampler: GPUSampler;
  constructor(private g: Gpu) { this.exp = g.device.createBuffer({ size: 16, usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST }); this.knob = g.device.createBuffer({ size: 32, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST }); this.sampler = g.device.createSampler({ magFilter: 'linear', minFilter: 'linear' }); }
  async init() {
    const d = this.g.device, mod = d.createShaderModule({ label: 'present', code: CURVE + `
@group(0) @binding(0) var src: texture_2d<f32>; @group(0) @binding(1) var<storage, read> e: array<f32, 4>; @group(0) @binding(2) var<uniform> k: array<vec4f, 2>; @group(0) @binding(3) var glow: texture_2d<f32>; @group(0) @binding(4) var sm: sampler; @group(0) @binding(5) var wide: texture_2d<f32>; @group(0) @binding(6) var wider: texture_2d<f32>;
fn ll(c: vec3f) -> f32 { return log2(max(dot(c, vec3f(0.2126, 0.7152, 0.0722)), 1e-5)); }
struct V { @builtin(position) p: vec4f, @location(0) uv: vec2f };
@vertex fn vs(@builtin(vertex_index) i: u32) -> V { let q = vec2f(f32((i << 1u) & 2u), f32(i & 2u)); var v: V; v.p = vec4f(q * 2.0 - 1.0, 0.0, 1.0); v.uv = vec2f(q.x, 1.0 - q.y); return v; }
fn h(p: vec2u) -> f32 { var x = p.x * 1973u + p.y * 9277u + u32(k[0].y) * 26699u; x = (x ^ (x >> 15u)) * 2246822519u; x = (x ^ (x >> 13u)) * 3266489917u; return f32((x ^ (x >> 16u)) >> 8u) / 16777216.0; }
@fragment fn fs(v: V) -> @location(0) vec4f {
  let n = vec2f(textureDimensions(src)); var c = textureLoad(src, min(vec2i(v.uv * n), vec2i(n) - 1), 0).xyz;
  if (k[0].z > 0.0) { c = mix(c, textureSampleLevel(glow, sm, v.uv, 0.0).xyz, k[0].z); }
  if (k[1].y > 0.0) {                              // the eye adapts by place: what is dark for metres round is lifted, what is bright is held
    let l0 = ll(textureSampleLevel(glow, sm, v.uv, 0.0).xyz); let a = ll(textureSampleLevel(wide, sm, v.uv, 0.0).xyz); let b = ll(textureSampleLevel(wider, sm, v.uv, 0.0).xyz);
    let wa = exp(-(a - l0) * (a - l0) * 0.45); let wb = exp(-(b - l0) * (b - l0) * 0.30) * 0.8; let ad = (l0 * 0.45 + a * wa + b * wb) / (0.45 + wa + wb); let mid = e[2];
    c *= exp2(clamp((mid - ad) * k[1].y, -1.3, 2.6));
  }
  let q = (v.uv - 0.5) * vec2f(1.0, 0.8); c *= 1.0 - 0.32 * dot(q, q);
  c = srgb(neutral(grade(max(c, vec3f(0.0)) * e[0] * exp2(k[0].x))));
  let g = h(vec2u(v.p.xy)) - 0.5; return vec4f(c + g * (1.0 / 255.0 + 0.012 * k[0].w * (1.0 - dot(c, vec3f(0.33)))), 1.0);
}` });
    const bad = (await mod.getCompilationInfo()).messages.find(m => m.type === 'error'); if (bad) throw new Error('present:' + bad.lineNum + ' ' + bad.message);
    this.mod = mod;
    const m2 = d.createShaderModule({ label: 'meter', code: `
@group(0) @binding(0) var src: texture_2d<f32>; @group(0) @binding(1) var<storage, read_write> e: array<f32, 4>; @group(0) @binding(2) var<uniform> k: array<vec4f, 2>;
var<workgroup> s: array<vec2f, 256>;
@compute @workgroup_size(256) fn main(@builtin(local_invocation_index) i: u32) {
  let n = textureDimensions(src); var a = vec2f(0.0);
  for (var j = 0u; j < 24u; j++) { let t = i * 24u + j; let p = vec2f(f32(t % 96u) + 0.5, f32(t / 96u) + 0.5) / vec2f(96.0, 64.0); let c = textureLoad(src, vec2u(p * vec2f(n)), 0).xyz; let l = log2(clamp(dot(c, vec3f(0.2126, 0.7152, 0.0722)), 1e-4, 1e3)); let q = (p - 0.5) * 1.6; let w = exp(-dot(q, q) * 2.2); a += vec2f(l * w, w); }
  s[i] = a; workgroupBarrier(); for (var st = 128u; st > 0u; st >>= 1u) { if (i < st) { s[i] += s[i + st]; } workgroupBarrier(); }
  if (i == 0u) { let mean = s[0].x / max(s[0].y, 1e-4); let want = exp2(clamp(-2.05 - mean * 0.86, -7.0, 5.5)); let r = select(0.06, 1.0, e[1] == 0.0 || k[1].x > 0.0); e[0] = select(want, exp2(mix(log2(max(e[0], 1e-6)), log2(want), r)), e[1] > 0.0); e[1] = 1.0; e[2] = mean; }
}` });
    this.meter = await d.createComputePipelineAsync({ layout: 'auto', compute: { module: m2, entryPoint: 'main' } }); void this.pipe;
  }
  private mod!: GPUShaderModule; private dummy: GPUTexture | null = null;
  run(e: GPUCommandEncoder, lab: Lab, src: GPUTexture, to: GPUTextureView, format: GPUTextureFormat, o: { glow?: GPUTexture; wide?: GPUTexture; wider?: GPUTexture; snap?: boolean; grain?: number; local?: number } = {}) {
    const d = this.g.device; d.queue.writeBuffer(this.knob, 0, new Float32Array([this.bias, lab.frameNo, o.glow ? 0.055 : 0, o.grain ?? 0.6, o.snap ? 1 : 0, o.wide ? o.local ?? 0.55 : 0, 0, 0]));
    { const p = e.beginComputePass(); p.setPipeline(this.meter); p.setBindGroup(0, d.createBindGroup({ layout: this.meter.getBindGroupLayout(0), entries: [{ binding: 0, resource: src.createView() }, { binding: 1, resource: { buffer: this.exp } }, { binding: 2, resource: { buffer: this.knob } }] })); p.dispatchWorkgroups(1); p.end(); }
    let pipe = this.pipes.get(format); if (!pipe) this.pipes.set(format, pipe = d.createRenderPipeline({ layout: 'auto', vertex: { module: this.mod, entryPoint: 'vs' }, fragment: { module: this.mod, entryPoint: 'fs', targets: [{ format }] } }));
    this.dummy ??= d.createTexture({ size: [1, 1], format: 'rgba16float', usage: GPUTextureUsage.TEXTURE_BINDING });
    const p = e.beginRenderPass({ colorAttachments: [{ view: to, loadOp: 'clear', storeOp: 'store' }] }); p.setPipeline(pipe); p.setBindGroup(0, d.createBindGroup({ layout: pipe.getBindGroupLayout(0), entries: [{ binding: 0, resource: src.createView() }, { binding: 1, resource: { buffer: this.exp } }, { binding: 2, resource: { buffer: this.knob } }, { binding: 3, resource: (o.glow ?? this.dummy).createView() }, { binding: 4, resource: this.sampler }, { binding: 5, resource: (o.wide ?? this.dummy).createView() }, { binding: 6, resource: (o.wider ?? this.dummy).createView() }] })); p.draw(3); p.end();
  }
}
