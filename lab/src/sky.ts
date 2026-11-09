// The sky image: drawn when the hour changes, its mips made by halving.
import type { Gpu } from './gpu';
import { type V3, norm, clamp, smooth } from './math';
import math from './wgsl/math.wgsl' with { type: 'text' };
import skyCode from './wgsl/sky.wgsl' with { type: 'text' };

export interface Hour { sun: V3; sunE: V3; radius: number; intensity: number; night: number; haze: number; cloud: number; isMoon: boolean; /** what a new moon and a dark city leave of the night sky */ moon: number; city: number }
/** an hour of the day (0‥24) → where the sun is and what it sends */
export function hourOf(h: number, bearing = 0.85, cover = 0, moon = 1, mains = true): Hour {
  const a = (h - 6) / 12 * Math.PI, el = Math.sin(a) * 1.02, dir = norm([Math.cos(a) * Math.cos(bearing) + 0.25, el, Math.cos(a) * Math.sin(bearing) + 0.55]);
  const y = dir[1], night = 1 - smooth(-0.12, 0.10, y), m = 1 / Math.max(0.03, y + 0.06), T: V3 = [Math.exp(-0.055 * m), Math.exp(-0.115 * m), Math.exp(-0.27 * m)];
  const k = 21 * smooth(-0.035, 0.07, y); let sunE: V3 = [T[0] * k * 1.0, T[1] * k * 0.985, T[2] * k * 1.0], sun = dir, radius = 0.0047 * 1.6, isMoon = false;
  if (y < -0.03) { isMoon = true; sun = norm([-0.45, 0.72, 0.5]); const q = 0.11 * smooth(0.03, 0.16, -y) * moon; sunE = [q * 0.62, q * 0.80, q * 1.15]; radius = 0.012; }
  // cloud takes the sun's disc and spreads it: softer, weaker, and the sky greyer
  const c = clamp(cover, 0, 1); sunE = [sunE[0] * (1 - 0.94 * c), sunE[1] * (1 - 0.93 * c), sunE[2] * (1 - 0.92 * c)]; radius += 0.10 * c;
  return { sun, sunE, radius, intensity: 21, night, haze: 1.0 + 1.6 * c, cloud: 0.9 + c, isMoon, moon: 0.09 + 0.91 * moon, city: mains ? 1 : 0.02, ...({ air: dir } as object) } as Hour;
}
export class Sky {
  readonly tex: GPUTexture; readonly size = 256; private views: GPUTextureView[] = []; private draw!: GPUComputePipeline; private half!: GPUComputePipeline; private knob: GPUBuffer; private last = '';
  constructor(private g: Gpu) {
    const levels = Math.log2(this.size) - 1;
    this.tex = g.device.createTexture({ label: 'sky', size: [this.size, this.size], format: 'rgba16float', mipLevelCount: levels, usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.COPY_SRC });
    for (let i = 0; i < levels; i++) this.views.push(this.tex.createView({ baseMipLevel: i, mipLevelCount: 1 }));
    this.knob = g.device.createBuffer({ size: 32, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST });
  }
  async init() {
    const d = this.g.device, head = `@group(0) @binding(0) var skyOut: texture_storage_2d<rgba16float, write>;\n`;
    const m1 = d.createShaderModule({ label: 'sky', code: math + head + 'struct K { knob: vec4f, sun: vec4f };\n@group(0) @binding(1) var<uniform> K0: K;\n' + skyCode.replaceAll('knobSun', 'K0.sun').replaceAll('knob.', 'K0.knob.') });
    const m2 = d.createShaderModule({ label: 'skyHalf', code: math + head + `@group(0) @binding(1) var src: texture_2d<f32>;
fn wrap(p: vec2i, n: i32) -> vec2i { var q = p; if (q.x < 0 || q.x >= n) { q = vec2i(clamp(q.x, 0, n - 1), n - 1 - q.y); } if (q.y < 0 || q.y >= n) { q = vec2i(n - 1 - q.x, clamp(q.y, 0, n - 1)); } return clamp(q, vec2i(0), vec2i(n - 1)); }
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) { let n = textureDimensions(skyOut); if (id.x >= n.x || id.y >= n.y) { return; } let m = i32(textureDimensions(src).x); var a = vec4f(0.0); var ws = 0.0;
  for (var y = -1; y <= 2; y++) { for (var x = -1; x <= 2; x++) { let w = select(1.0, 3.0, x == 0 || x == 1) * select(1.0, 3.0, y == 0 || y == 1); a += w * textureLoad(src, wrap(vec2i(id.xy) * 2 + vec2i(x, y), m), 0); ws += w; } }
  textureStore(skyOut, id.xy, a / ws); }` });
    for (const m of [m1, m2]) { const i = await m.getCompilationInfo(), e = i.messages.find(x => x.type === 'error'); if (e) throw new Error(`sky:${e.lineNum} ${e.message}`); }
    this.draw = await d.createComputePipelineAsync({ layout: 'auto', compute: { module: m1, entryPoint: 'main' } }); this.half = await d.createComputePipelineAsync({ layout: 'auto', compute: { module: m2, entryPoint: 'main' } });
  }
  update(h: Hour) {
    const a = (h as unknown as { air: V3 }).air, key = JSON.stringify([a, h.haze, h.cloud, h.night, h.moon, h.city]); if (key === this.last) return; this.last = key;
    const d = this.g.device; d.queue.writeBuffer(this.knob, 0, new Float32Array([h.intensity, h.night, h.haze, h.cloud, a[0], a[1], a[2], h.moon + 2 * Math.round(h.city * 100)]));
    const e = d.createCommandEncoder(); let n = this.size;
    const pass = (pipe: GPUComputePipeline, entries: GPUBindGroupEntry[], size: number) => { const p = e.beginComputePass(); p.setPipeline(pipe); p.setBindGroup(0, d.createBindGroup({ layout: pipe.getBindGroupLayout(0), entries })); p.dispatchWorkgroups(Math.ceil(size / 8), Math.ceil(size / 8)); p.end(); };
    pass(this.draw, [{ binding: 0, resource: this.views[0]! }, { binding: 1, resource: { buffer: this.knob } }], n);
    for (let i = 1; i < this.views.length; i++) { n >>= 1; pass(this.half, [{ binding: 0, resource: this.views[i]! }, { binding: 1, resource: this.views[i - 1]! }], n); }
    d.queue.submit([e.finish()]); void clamp;
  }
}
