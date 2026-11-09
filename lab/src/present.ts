// The canvas: exposure by base and detail, the glow, the curve.
import type { Gpu } from './gpu';
import type { Lab } from './lab';
import { KNOBS, tuneOf, type Knob } from './tune';
/** the same numbers, two rows into the canvas's own */
const tunedFrom = (code: string) => code.replace(/@tune\.(\w+)/g, (_, n: string) => { const q = (KNOBS as Record<string, Knob>)[n]!; return tuneOf({ ...q, i: q.i + 8 }, 'k'); });
import { CURVE, exposeKnobs } from './stages/expose';
export class Present {
  bias = 0; private pipes = new Map<string, GPURenderPipeline>(); private sampler: GPUSampler; private mod!: GPUShaderModule; private knob: GPUBuffer; private dummy: GPUTexture;
  constructor(private g: Gpu) { const d = g.device; this.sampler = d.createSampler({ magFilter: 'linear', minFilter: 'linear' }); this.knob = d.createBuffer({ size: 160, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST }); this.dummy = d.createTexture({ size: [1, 1], format: 'rgba16float', usage: GPUTextureUsage.TEXTURE_BINDING }); }
  async init() {
    this.mod = this.g.device.createShaderModule({ label: 'present', code: tunedFrom(CURVE + `
@group(0) @binding(0) var src: texture_2d<f32>; @group(0) @binding(1) var<storage, read> e: array<f32, 4>; @group(0) @binding(2) var<uniform> k: array<vec4f, 9>; @group(0) @binding(3) var glow: texture_2d<f32>; @group(0) @binding(4) var sm: sampler; @group(0) @binding(5) var base: texture_2d<f32>; @group(0) @binding(6) var rain: texture_2d<f32>;
struct V { @builtin(position) p: vec4f, @location(0) uv: vec2f };
@vertex fn vs(@builtin(vertex_index) i: u32) -> V { let q = vec2f(f32((i << 1u) & 2u), f32(i & 2u)); var v: V; v.p = vec4f(q * 2.0 - 1.0, 0.0, 1.0); v.uv = vec2f(q.x, 1.0 - q.y); return v; }
fn h(p: vec2u) -> f32 { var x = p.x * 1973u + p.y * 9277u + u32(k[0].y) * 26699u; x = (x ^ (x >> 15u)) * 2246822519u; x = (x ^ (x >> 13u)) * 3266489917u; return f32((x ^ (x >> 16u)) >> 8u) / 16777216.0; }
fn srgb(c: vec3f) -> vec3f { return select(1.055 * pow(max(c, vec3f(0.0)), vec3f(1.0 / 2.4)) - 0.055, c * 12.92, c <= vec3f(0.0031308)); }
fn lum(c: vec3f) -> f32 { return dot(c, vec3f(0.2126, 0.7152, 0.0722)); }
@fragment fn fs(v: V) -> @location(0) vec4f {
  let n = vec2f(textureDimensions(src)); var c = max(textureLoad(src, min(vec2i(v.uv * n), vec2i(n) - 1), 0).xyz, vec3f(0.0)); if (k[1].x > 0.0) { return vec4f(srgb(clamp(c * k[1].y, vec3f(0.0), vec3f(1.0))), 1.0); }
  { // sharpen by what the neighbours allow: no ringing past the brightest and darkest of them
    let q = min(vec2i(v.uv * n), vec2i(n) - 1); let hi = vec2i(n) - 1; let a = textureLoad(src, clamp(q + vec2i(0, -1), vec2i(0), hi), 0).xyz; let b = textureLoad(src, clamp(q + vec2i(0, 1), vec2i(0), hi), 0).xyz; let d = textureLoad(src, clamp(q + vec2i(-1, 0), vec2i(0), hi), 0).xyz; let f = textureLoad(src, clamp(q + vec2i(1, 0), vec2i(0), hi), 0).xyz;
    let la = lum(a); let lb = lum(b); let ld = lum(d); let lf = lum(f); let lc = lum(c); let mn = min(min(la, lb), min(min(ld, lf), lc)); let mx = max(max(la, lb), max(max(ld, lf), lc)); let w = -k[1].z * sqrt(clamp(min(mn, 2.0 * lc) / max(mx, 1e-5), 0.0, 1.0)); c = max((c + (a + b + d + f) * w) / (1.0 + 4.0 * w), vec3f(0.0)); }
  if (k[1].w > 0.0) { let r = textureLoad(rain, min(vec2i(v.uv * vec2f(textureDimensions(rain))), vec2i(textureDimensions(rain)) - 1), 0); c = c * (1.0 - 0.55 * min(r.a, 1.0)) + r.rgb; }
  let A = e[0]; let l = log2(max(strong(c), 1e-5)); let gain = exp2(stops(l, A, textureSampleLevel(base, sm, v.uv, 0.0).xy)); c = mix(c * gain, textureSampleLevel(glow, sm, v.uv, 0.0).xyz * min(gain, exp2(@tune.key - 0.9 * A)), k[0].z);   // (the glow is the lens's: what is lifted for being dark does not lift it)
  let q = (v.uv - 0.5) * vec2f(1.0, 0.8); c *= 1.0 - @tune.vignette * dot(q, q);
  c = srgb(shown(c)); let g = h(vec2u(v.p.xy)) - 0.5; return vec4f(c + g * (1.0 / 255.0 + 0.010 * k[0].w * (1.0 - dot(c, vec3f(0.33)))), 1.0);
}`) });
    const bad = (await this.mod.getCompilationInfo()).messages.find(m => m.type === 'error'); if (bad) throw new Error('present:' + bad.lineNum + ' ' + bad.message);
  }
  /** before the frame: the knobs the exposure stages read */
  set(lab: Lab, snap = false) { exposeKnobs(lab, this.bias, snap); }
  /** `pre` = the method's prefix for its glow and fused levels; `plain` = show a signal as it is */
  run(e: GPUCommandEncoder, lab: Lab, src: GPUTexture, to: GPUTextureView, format: GPUTextureFormat, pre: string, o: { plain?: number; grain?: number; sharpen?: number } = {}) {
    const wet = pre === 'rt' && !o.plain && (lab.rainNow > 0.004 || lab.snowNow > 0.004) && lab.graph.signals.has('rain'); const d = this.g.device; d.queue.writeBuffer(this.knob, 0, new Float32Array([this.bias, lab.frameNo, lab.tune[KNOBS.glow.i]!, o.grain ?? 0.6, o.plain ? 1 : 0, o.plain ?? 1, o.sharpen ?? 0.12, wet ? 1 : 0, ...lab.tune]));
    let pipe = this.pipes.get(format); if (!pipe) this.pipes.set(format, pipe = d.createRenderPipeline({ layout: 'auto', vertex: { module: this.mod, entryPoint: 'vs' }, fragment: { module: this.mod, entryPoint: 'fs', targets: [{ format }] } }));
    const t = (n: string) => ({ resource: (o.plain ? this.dummy : lab.graph.texture(n)).createView() }), p = e.beginRenderPass({ colorAttachments: [{ view: to, loadOp: 'clear', storeOp: 'store' }] }); p.setPipeline(pipe);
    p.setBindGroup(0, d.createBindGroup({ layout: pipe.getBindGroupLayout(0), entries: [{ binding: 0, resource: src.createView() }, { binding: 1, resource: { buffer: lab.graph.buffers.get('exposure')! } }, { binding: 2, resource: { buffer: this.knob } }, { binding: 3, ...t(pre + 'Bloom') }, { binding: 4, resource: this.sampler }, { binding: 5, ...t(pre + 'Base') }, { binding: 6, resource: (wet ? lab.graph.texture('rain') : this.dummy).createView() }] })); p.draw(3); p.end();
  }
}
