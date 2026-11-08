// THE FRAME GRAPH. A stage is a function with typed inputs and outputs: it names the signals it reads and writes, and the bindings of its WGSL are
// written from those names. So a stage's shader is the algorithm alone, and swapping one algorithm for another (a denoiser, an upscaler, a light cache)
// is swapping one object for another with the same signature.
//
//   reads   signal → the WGSL variable it is bound as. 'indirect@prev' is last frame's.
//   writes  WGSL variable → signal.
//   buffers WGSL variable → [buffer, WGSL type, access].
import type { Gpu } from './gpu';
import { Timer } from './gpu';

export type Size = 'render' | 'half' | 'output' | [number, number] | { of: 'output'; div: number };
export type Signal = { format: GPUTextureFormat; size: Size; history?: boolean };
export type Access = 'read' | 'read_write' | 'uniform';
export interface Stage {
  name: string;
  reads?: Record<string, string>;
  writes?: Record<string, string>;
  buffers?: Record<string, [string, string, Access]>;
  code: string;
  entry?: string;
  /** one thread a texel of this size, or a fixed number of workgroups */
  over: Size | { groups: () => [number, number, number] };
  wg?: [number, number];
  consts?: Record<string, number>;
  when?: () => boolean;
  /** a stage that rasterizes does it itself; made once by `make`, then called each frame */
  make?: (g: Graph) => Promise<(enc: GPUCommandEncoder, g: Graph) => void>;
}

const SCALAR: Record<string, string> = { rgba32uint: 'u32', rg32uint: 'u32', r32uint: 'u32' };
const UNFILTERABLE = new Set(['r32float', 'rgba32float', 'rg32float']);

export class Graph {
  readonly signals = new Map<string, Signal>();
  readonly buffers = new Map<string, GPUBuffer>();
  private tex = new Map<string, GPUTexture[]>(); private views = new Map<GPUTexture, GPUTextureView>();
  private stages: Stage[] = []; private built = new Map<Stage, { pipe: GPUComputePipeline; layout: GPUBindGroupLayout; names: [string, 'r' | 'w' | 'b'][] }>();
  private drawn = new Map<Stage, (enc: GPUCommandEncoder, g: Graph) => void>(); private groups = new Map<string, GPUBindGroup>(); private parity = 0;
  private external = new Map<string, GPUTexture>();
  render: [number, number] = [1, 1]; output: [number, number] = [1, 1];
  readonly timer: Timer;
  private ids = new WeakMap<object, number>(); private nextId = 1;
  constructor(readonly g: Gpu, readonly prelude: () => string, readonly shared: { layout: GPUBindGroupLayout; group: () => GPUBindGroup }) { this.timer = new Timer(g); }

  signal(name: string, format: GPUTextureFormat, size: Size, history = false) { this.signals.set(name, { format, size, history }); return this; }
  bind(name: string, t: GPUTexture) { this.external.set(name, t); this.signals.set(name, { format: t.format, size: [t.width, t.height] }); }
  px(s: Size): [number, number] { return s === 'render' ? this.render : s === 'output' ? this.output : s === 'half' ? [Math.ceil(this.render[0] / 2), Math.ceil(this.render[1] / 2)] : Array.isArray(s) ? s : [Math.max(1, Math.ceil(this.output[0] / s.div)), Math.max(1, Math.ceil(this.output[1] / s.div))]; }

  resize(render: [number, number], output: [number, number]) {
    if (render[0] === this.render[0] && render[1] === this.render[1] && output[0] === this.output[0] && output[1] === this.output[1]) return false;
    this.render = render; this.output = output;
    for (const [n, list] of this.tex) { const s = this.signals.get(n)!, [w, h] = this.px(s.size); if (list[0]!.width !== w || list[0]!.height !== h) { for (const t of list) t.destroy(); this.tex.delete(n); } }
    this.groups.clear(); return true;
  }
  /** this frame's texture of a signal, or last frame's */
  texture(ref: string): GPUTexture {
    const [name, when] = ref.split('@') as [string, string | undefined], ext = this.external.get(name); if (ext) return ext;
    const s = this.signals.get(name); if (!s) throw new Error('no signal ' + name);
    let list = this.tex.get(name);
    if (!list) {
      const [w, h] = this.px(s.size);
      const usage = s.format.startsWith('depth') ? GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.TEXTURE_BINDING : GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_SRC | GPUTextureUsage.COPY_DST | (s.format === 'r32uint' ? GPUTextureUsage.RENDER_ATTACHMENT : 0);
      list = Array.from({ length: s.history ? 2 : 1 }, (_, i) => this.g.device.createTexture({ label: name + i, size: [w, h], format: s.format, usage }));
      this.tex.set(name, list);
    }
    return list[s.history ? (when === 'prev' ? 1 - this.parity : this.parity) : 0]!;
  }
  view(ref: string): GPUTextureView { const t = this.texture(ref); let v = this.views.get(t); if (!v) this.views.set(t, v = t.createView()); return v; }
  private id(o: object) { let i = this.ids.get(o); if (!i) this.ids.set(o, i = this.nextId++); return i; }

  /** the whole text of a stage's shader: the shared prelude, its bindings, its code */
  source(s: Stage): string {
    let k = 0; const out: string[] = [this.prelude()];
    for (const [c, v] of Object.entries(s.consts ?? {})) out.push(`const ${c} = ${Number.isInteger(v) ? v : v.toFixed(8)};`);
    for (const [ref, v] of Object.entries(s.reads ?? {})) { const f = this.signals.get(ref.split('@')[0]!)?.format; if (!f) throw new Error(`${s.name} reads ${ref}, which nothing declares`); out.push(`@group(1) @binding(${k++}) var ${v}: ${f.startsWith('depth') ? 'texture_depth_2d' : `texture_2d<${SCALAR[f] ?? 'f32'}>`};`); }
    for (const [v, n] of Object.entries(s.writes ?? {})) { const f = this.signals.get(n)?.format; if (!f) throw new Error(`${s.name} writes ${n}, which nothing declares`); out.push(`@group(1) @binding(${k++}) var ${v}: texture_storage_2d<${f}, write>;`); }
    for (const [v, [, type, access]] of Object.entries(s.buffers ?? {})) out.push(`@group(1) @binding(${k++}) var<${access === 'uniform' ? 'uniform' : 'storage, ' + access}> ${v}: ${type};`);
    // EVERY FLOAT A STAGE WRITES IS CHECKED, whether or not its author thought of it: a stage cannot reach its outputs but through this. One bad value in a
    // history would otherwise be read back for ever, and spread.
    let code = s.code;
    for (const [v, n] of Object.entries(s.writes ?? {})) { const f = this.signals.get(n)!.format; if (SCALAR[f]) continue; const most = f.includes('16float') ? '60000.0' : f.includes('32float') ? '1e30' : '1.0'; out.push(`fn put_${v}(p: vec2u, c: vec4f) { textureStore(${v}, p, finite4(c, ${most})); }`); code = code.replaceAll(`textureStore(${v},`, `put_${v}(`); }
    out.push(code); return out.join('\n');
  }
  async build(stages: Stage[]) {
    const d = this.g.device;
    await Promise.all(stages.map(async s => {
      if (this.built.has(s) || this.drawn.has(s)) return;
      if (s.make) { this.drawn.set(s, await s.make(this)); return; }
      const entries: GPUBindGroupLayoutEntry[] = [], names: [string, 'r' | 'w' | 'b'][] = []; let k = 0; const visibility = GPUShaderStage.COMPUTE;
      for (const ref of Object.keys(s.reads ?? {})) { const f = this.signals.get(ref.split('@')[0]!)!.format; entries.push({ binding: k++, visibility, texture: { sampleType: f.startsWith('depth') ? 'depth' : SCALAR[f] ? 'uint' : UNFILTERABLE.has(f) ? 'unfilterable-float' : 'float' } }); names.push([ref, 'r']); }
      for (const n of Object.values(s.writes ?? {})) { entries.push({ binding: k++, visibility, storageTexture: { access: 'write-only', format: this.signals.get(n)!.format } }); names.push([n, 'w']); }
      for (const [b, , access] of Object.values(s.buffers ?? {})) { entries.push({ binding: k++, visibility, buffer: { type: access === 'uniform' ? 'uniform' : access === 'read' ? 'read-only-storage' : 'storage' } }); names.push([b, 'b']); }
      const layout = d.createBindGroupLayout({ label: s.name, entries }), code = this.source(s);
      const module = d.createShaderModule({ label: s.name, code });
      const info = await module.getCompilationInfo(); const bad = info.messages.filter(m => m.type === 'error');
      if (bad.length) { const lines = code.split('\n'); throw new Error(bad.slice(0, 6).map(m => `${s.name}:${m.lineNum}:${m.linePos} ${m.message}\n    ${lines[m.lineNum - 1]?.trim().slice(0, 160)}`).join('\n')); }
      const pipe = await d.createComputePipelineAsync({ label: s.name, layout: d.createPipelineLayout({ bindGroupLayouts: [this.shared.layout, layout] }), compute: { module, entryPoint: s.entry ?? 'main' } });
      this.built.set(s, { pipe, layout, names });
    }));
    this.stages = stages; this.groups.clear();
  }
  run(enc: GPUCommandEncoder, stages = this.stages) {
    for (const s of stages) {
      if (s.when && !s.when()) continue;
      const own = this.drawn.get(s); if (own) { own(enc, this); continue; }
      const b = this.built.get(s); if (!b) continue;
      const res = b.names.map(([n, kind]) => kind === 'b' ? { buffer: this.buffers.get(n) ?? fail('no buffer ' + n) } : this.view(n));
      const key = s.name + ':' + this.id(s) + ':' + res.map(r => this.id('buffer' in r ? r.buffer : r)).join(',');
      let group = this.groups.get(key);
      if (!group) this.groups.set(key, group = this.g.device.createBindGroup({ layout: b.layout, entries: res.map((resource, binding) => ({ binding, resource: resource as GPUBindingResource })) }));
      const p = enc.beginComputePass({ label: s.name, timestampWrites: this.timer.pass(s.name) });
      p.setPipeline(b.pipe); p.setBindGroup(0, this.shared.group()); p.setBindGroup(1, group);
      if (typeof s.over === 'object' && !Array.isArray(s.over) && 'groups' in s.over) p.dispatchWorkgroups(...s.over.groups());
      else { const [w, h] = this.px(s.over), [gx, gy] = s.wg ?? [8, 8]; p.dispatchWorkgroups(Math.ceil(w / gx), Math.ceil(h / gy)); }
      p.end();
    }
  }
  /** this frame becomes last frame */
  flip() { this.parity ^= 1; }
}
function fail(t: string): never { throw new Error(t); }
