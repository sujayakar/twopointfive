// THE FRAME GRAPH. A stage names the buffers it reads and writes. Its bindings are written from those
// names, so its WGSL is the algorithm alone. Only the stages that what is on the screen depends on run.

import type { Gpu } from './device';
import { makeBuffer, Timer } from './device';
import type { Struct } from './layout';

/** Where a buffer lives: one element a pixel of some size, or a fixed number of elements. */
export type Grid = 'render' | 'wide' | 'narrow' | 'output';
export type Size = Grid | { count: number };

export interface BufferSpec {
  name: string;
  struct: Struct;
  size: Size;
  about: string;
  /** last frame's is kept beside this frame's, as `name@before` */
  history?: boolean;
}

export interface Stage {
  name: string;
  /** what this stage finds out, as a question */
  question: string;
  /** WGSL variable to buffer name; a list where they are the same. `x@before` is last frame's x */
  reads: string[] | Record<string, string>;
  writes: string[] | Record<string, string>;
  /** shared pieces of WGSL, by name */
  uses: string[];
  /** has `fn run(pixel: vec2u)`, or `fn run(index: u32)` when it runs over a count */
  code: string;
  over: Size;
  /** done by a function given to `setCustom`, not by a compute shader */
  custom?: boolean;
  /** numbers fixed when the shader is made */
  constants?: Record<string, number>;
}

/** A piece of WGSL that stages share, and what it needs bound. */
export interface Module {
  code: string;
  needs: string[];
  shared: string[];
}
/** Bound by its own name wherever a module asks for it. */
export interface Shared {
  type: string;
  structs: Struct[];
  access: 'uniform' | 'read' | 'read_write' | 'texture';
  buffer: () => GPUBuffer;
  /** for a texture */
  view?: () => GPUTextureView;
}

type Binding = { variable: string; source: string; access: Shared['access']; declaration: string };
interface Built {
  pipeline: GPUComputePipeline;
  layout: GPUBindGroupLayout;
  bindings: Binding[];
}

const asMap = (names: string[] | Record<string, string>): Record<string, string> =>
  Array.isArray(names) ? Object.fromEntries(names.map((n) => [n.replace('@', '_'), n])) : names;
const withoutBefore = (source: string) => source.split('@')[0]!;

export class Graph {
  readonly buffers = new Map<string, BufferSpec>();
  readonly stages: Stage[] = [];
  readonly timer: Timer;
  /** what went wrong with a stage's shader, by stage */
  readonly errors = new Map<string, string>();
  /** the stages that ran last frame, in order */
  ran: string[] = [];
  grids: Record<Grid, [number, number]> = { render: [0, 0], wide: [0, 0], narrow: [0, 0], output: [0, 0] };

  private modules = new Map<string, Module>();
  private shared = new Map<string, Shared>();
  private aliases = new Map<string, () => string>();
  private made = new Map<string, GPUBuffer[]>();
  private built = new Map<string, Built | 'building'>();
  private groups = new Map<string, GPUBindGroup>();
  private ids = new WeakMap<object, number>();
  private nextId = 1;
  private parity = 0;
  private frame = 0;
  /** which frame wrote a buffer last, not counting this one */
  private lastWritten = new Map<string, number>();
  private writtenNow = new Set<string>();
  private edits = new Map<string, string>();
  private customs = new Map<string, (encoder: GPUCommandEncoder) => boolean>();

  constructor(readonly gpu: Gpu, private prelude: () => string) {
    this.timer = new Timer(gpu);
  }

  addModule(name: string, module: Module): void {
    this.modules.set(name, module);
  }
  addShared(name: string, shared: Shared): void {
    this.shared.set(name, shared);
  }
  addBuffer(spec: BufferSpec): string {
    this.buffers.set(spec.name, spec);
    return spec.name;
  }
  addStage(stage: Stage): void {
    if (this.stages.some((s) => s.name === stage.name)) throw new Error(`two stages named ${stage.name}`);
    this.stages.push(stage);
  }
  /** a name that stands for another buffer, chosen anew each frame */
  /** what a custom stage does; false while it is not ready */
  setCustom(name: string, run: (encoder: GPUCommandEncoder) => boolean): void {
    this.customs.set(name, run);
  }
  /** other text for a module: every stage that uses it is made anew */
  changeModule(name: string, module: Module): void {
    this.modules.set(name, module);
    for (const stage of this.stages) {
      if (this.modulesOf(['frame', ...stage.uses]).includes(name)) this.built.delete(stage.name);
    }
    this.groups.clear();
  }
  /** the WGSL of shared resources for a pipeline made by hand, and how to bind them */
  sharedFor(names: string[]): { wgsl: string; buffers: () => GPUBuffer[] } {
    const structs = new Map<string, Struct>();
    names.forEach((n) => this.shared.get(n)!.structs.forEach((s) => structs.set(s.name, s)));
    const wgsl = [
      this.prelude(),
      ...[...structs.values()].map((s) => s.wgsl),
      ...names.map((n, i) => {
        const s = this.shared.get(n)!;
        return `@group(0) @binding(${i}) var<${s.access === 'uniform' ? 'uniform' : 'storage, read'}> ${n}: ${s.type};`;
      }),
    ].join('\n');
    return { wgsl, buffers: () => names.map((n) => this.shared.get(n)!.buffer()) };
  }
  addAlias(name: string, standsFor: () => string): void {
    this.aliases.set(name, standsFor);
  }
  resolve(name: string): string {
    let at = name;
    for (let i = 0; i < 8; i++) {
      const [base, when] = at.split('@') as [string, string | undefined];
      const alias = this.aliases.get(base);
      if (!alias) return at;
      at = alias() + (when ? '@' + when : '');
    }
    throw new Error(`${name} stands for itself`);
  }
  spec(name: string): BufferSpec {
    const found = this.buffers.get(withoutBefore(this.resolve(name)));
    if (!found) throw new Error(`no buffer named ${name}`);
    return found;
  }

  // ---- sizes and buffers

  resize(grids: Record<Grid, [number, number]>): boolean {
    const changed = (Object.keys(grids) as Grid[]).filter((g) => String(grids[g]) !== String(this.grids[g]));
    if (changed.length === 0) return false;
    this.grids = grids;
    for (const [name, list] of this.made) {
      const { size } = this.buffers.get(name)!;
      if (typeof size === 'object' || !changed.includes(size)) continue;
      list.forEach((b) => b.destroy());
      this.made.delete(name);
      this.lastWritten.delete(name);
    }
    this.groups.clear();
    return true;
  }
  pixels(size: Size): [number, number] {
    return typeof size === 'object' ? [size.count, 1] : this.grids[size];
  }
  elements(size: Size): number {
    const [w, h] = this.pixels(size);
    return w * h;
  }
  /** this frame's buffer of that name, or last frame's */
  buffer(source: string): GPUBuffer {
    const [name, when] = this.resolve(source).split('@') as [string, string | undefined];
    const spec = this.spec(name);
    let list = this.made.get(name);
    if (!list) {
      const bytes = this.elements(spec.size) * spec.struct.stride;
      list = Array.from({ length: spec.history ? 2 : 1 }, (_, i) =>
        makeBuffer(this.gpu, spec.history ? `${name} ${i}` : name, bytes, GPUBufferUsage.STORAGE),
      );
      this.made.set(name, list);
    }
    if (!spec.history) return list[0]!;
    return list[when === 'before' ? 1 - this.parity : this.parity]!;
  }
  clear(encoder: GPUCommandEncoder, name: string): void {
    const list = this.made.get(this.resolve(name));
    list?.forEach((b) => encoder.clearBuffer(b));
  }
  bytesOf(name: string): number {
    const spec = this.spec(name);
    return this.elements(spec.size) * spec.struct.stride * (spec.history ? 2 : 1);
  }

  // ---- shaders

  private modulesOf(names: string[]): string[] {
    const out: string[] = [];
    const visit = (name: string) => {
      if (out.includes(name)) return;
      const module = this.modules.get(name);
      if (!module) throw new Error(`no module named ${name}`);
      module.needs.forEach(visit);
      out.push(name);
    };
    names.forEach(visit);
    return out;
  }

  private bindingsOf(stage: Stage): { bindings: Binding[]; structs: Struct[]; modules: string[] } {
    const modules = this.modulesOf(['frame', ...stage.uses]);
    const structs = new Map<string, Struct>();
    const bindings: Binding[] = [];
    const sharedNames = new Set(modules.flatMap((m) => this.modules.get(m)!.shared));
    for (const name of sharedNames) {
      const s = this.shared.get(name);
      if (!s) throw new Error(`nothing shared is named ${name}`);
      s.structs.forEach((x) => structs.set(x.name, x));
      bindings.push({ variable: name, source: name, access: s.access, declaration: s.type });
    }
    const addBuffers = (map: Record<string, string>, access: 'read' | 'read_write') => {
      for (const [variable, source] of Object.entries(map)) {
        const spec = this.spec(source);
        structs.set(spec.struct.name, spec.struct);
        bindings.push({ variable, source, access, declaration: `array<${spec.struct.name}>` });
      }
    };
    addBuffers(asMap(stage.reads), 'read');
    addBuffers(asMap(stage.writes), 'read_write');
    return { bindings, structs: [...structs.values()], modules };
  }

  /** `raw_at(p)` reads a pixel (held to the edges); `put_raw(p, v)` writes one, `kept_raw(p)` is what
   *  stands in a buffer this stage writes, from the last time it ran */
  private accessors(stage: Stage): string {
    const out: string[] = [];
    const each = (map: Record<string, string>, write: boolean) => {
      for (const [variable, source] of Object.entries(map)) {
        const { size, struct } = this.spec(source);
        if (typeof size === 'object') continue;
        const dim = `frame.${size}_size`;
        out.push(
          write
            ? `fn put_${variable}(p: vec2u, v: ${struct.name}) { ${variable}[p.y * ${dim}.x + p.x] = v; }
fn kept_${variable}(p: vec2u) -> ${struct.name} { return ${variable}[p.y * ${dim}.x + p.x]; }`
            : `fn ${variable}_at(p: vec2i) -> ${struct.name} { let q = vec2u(clamp(p, vec2i(0), vec2i(${dim}) - 1)); return ${variable}[q.y * ${dim}.x + q.x]; }`,
        );
      }
    };
    each(asMap(stage.reads), false);
    each(asMap(stage.writes), true);
    return out.join('\n');
  }

  /** what the page shows and lets be edited */
  codeOf(stage: Stage): string {
    return this.edits.get(stage.name) ?? stage.code;
  }
  edit(stageName: string, code: string | null): void {
    if (code === null) this.edits.delete(stageName);
    else this.edits.set(stageName, code);
    const stage = this.stages.find((s) => s.name === stageName);
    if (stage) void this.build(stage);
  }

  /** the whole text of a stage's shader, and how many lines come before its own */
  source(stage: Stage): { text: string; before: number } {
    const { bindings, structs, modules } = this.bindingsOf(stage);
    const before = [
      this.prelude(),
      ...structs.map((s) => s.wgsl),
      ...Object.entries(stage.constants ?? {}).map(
        ([k, v]) => `const ${k} = ${Number.isInteger(v) ? `${v}u` : v};`,
      ),
      ...bindings.map(
        (b, i) =>
          b.access === 'texture'
            ? `@group(0) @binding(${i}) var ${b.variable}: ${b.declaration};`
            : `@group(0) @binding(${i}) var<${b.access === 'uniform' ? 'uniform' : 'storage, ' + b.access}> ${b.variable}: ${b.declaration};`,
      ),
      ...modules.map((m) => this.modules.get(m)!.code),
      this.accessors(stage),
    ].join('\n');
    const overPixels = typeof stage.over !== 'object';
    const main = overPixels
      ? `@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let size = frame.${stage.over as string}_size;
  if (id.x < size.x && id.y < size.y) { run(id.xy); }
}`
      : `@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) {
  if (id.x < ${(stage.over as { count: number }).count}u) { run(id.x); }
}`;
    return { text: `${before}\n${this.codeOf(stage)}\n${main}`, before: before.split('\n').length };
  }

  private async build(stage: Stage): Promise<void> {
    if (!this.built.has(stage.name)) this.built.set(stage.name, 'building');
    const { device } = this.gpu;
    const { bindings } = this.bindingsOf(stage);
    const layout = device.createBindGroupLayout({
      label: stage.name,
      entries: bindings.map((b, binding) => ({
        binding,
        visibility: GPUShaderStage.COMPUTE,
        ...(b.access === 'texture'
          ? { texture: { sampleType: 'uint' as const } }
          : { buffer: { type: b.access === 'uniform' ? 'uniform' as const : b.access === 'read' ? 'read-only-storage' as const : 'storage' as const } }),
      })),
    });
    const { text, before } = this.source(stage);
    const module = device.createShaderModule({ label: stage.name, code: text });
    const info = await module.getCompilationInfo();
    const bad = info.messages.filter((m) => m.type === 'error');
    if (bad.length > 0) {
      const lines = text.split('\n');
      this.errors.set(
        stage.name,
        bad.slice(0, 4).map((m) => {
          const line = m.lineNum - before;
          const where = line > 0 ? `line ${line}` : `shared line ${m.lineNum}`;
          return `${where}: ${m.message.split('\n')[0]}\n    ${lines[m.lineNum - 1]?.trim()}`;
        }).join('\n'),
      );
      return;
    }
    try {
      const pipeline = await device.createComputePipelineAsync({
        label: stage.name,
        layout: device.createPipelineLayout({ bindGroupLayouts: [layout] }),
        compute: { module, entryPoint: 'main' },
      });
      this.errors.delete(stage.name);
      this.built.set(stage.name, { pipeline, layout, bindings });
      this.groups.clear();
    } catch (error) {
      this.errors.set(stage.name, String(error));
    }
  }

  // ---- running

  /** the stages that must run so that these buffers are fresh, in order */
  needed(wanted: string[]): Stage[] {
    const need = new Set(wanted.map((w) => withoutBefore(this.resolve(w))));
    const chosen = new Set<Stage>();
    for (let grew = true; grew; ) {
      grew = false;
      for (const stage of this.stages) {
        if (chosen.has(stage)) continue;
        const writes = Object.values(asMap(stage.writes)).map((w) => withoutBefore(this.resolve(w)));
        if (!writes.some((w) => need.has(w))) continue;
        chosen.add(stage);
        grew = true;
        for (const read of Object.values(asMap(stage.reads))) need.add(withoutBefore(this.resolve(read)));
      }
    }
    return this.stages.filter((s) => chosen.has(s));
  }

  private id(o: object): number {
    let id = this.ids.get(o);
    if (!id) this.ids.set(o, (id = this.nextId++));
    return id;
  }

  /** runs what `wanted` needs, or only such stages of it as `only` lets through. False while a shader
   *  of it is still being made. */
  run(encoder: GPUCommandEncoder, wanted: string[], only: (stage: string) => boolean = () => true): boolean {
    const { device } = this.gpu;
    const stages = this.needed(wanted).filter((s) => only(s.name));
    let ready = true;
    this.ran = [];
    this.timer.begin();
    for (const stage of stages) {
      if (stage.custom) {
        const done = this.customs.get(stage.name);
        if (!done || !done(encoder)) ready = false;
        for (const write of Object.values(asMap(stage.writes))) this.writtenNow.add(withoutBefore(this.resolve(write)));
        this.ran.push(stage.name);
        continue;
      }
      const built = this.built.get(stage.name);
      if (!built) void this.build(stage);
      if (!built || built === 'building') {
        ready = false;
        continue;
      }
      // a history that was not written last frame is not last frame's
      for (const source of Object.values(asMap(stage.reads))) {
        if (!source.includes('@')) continue;
        const name = withoutBefore(this.resolve(source));
        if (this.lastWritten.get(name) !== this.frame - 1) {
          encoder.clearBuffer(this.buffer(name + '@before'));
        }
      }
      const resources: (GPUBuffer | GPUTextureView)[] = built.bindings.map((b) => {
        const shared = this.shared.get(b.variable);
        return !shared ? this.buffer(b.source) : shared.view ? shared.view() : shared.buffer();
      });
      const key = stage.name + ':' + resources.map((r) => this.id(r)).join(',');
      let group = this.groups.get(key);
      if (!group) {
        group = device.createBindGroup({
          label: stage.name,
          layout: built.layout,
          entries: built.bindings.map((b, binding) => ({
            binding,
            resource: b.access === 'texture' ? (resources[binding] as GPUTextureView) : { buffer: resources[binding] as GPUBuffer },
          })),
        });
        this.groups.set(key, group);
      }
      const pass = encoder.beginComputePass({ label: stage.name, timestampWrites: this.timer.pass(stage.name) });
      pass.setPipeline(built.pipeline);
      pass.setBindGroup(0, group);
      const [w, h] = this.pixels(stage.over);
      if (typeof stage.over === 'object') pass.dispatchWorkgroups(Math.ceil(w / 64));
      else pass.dispatchWorkgroups(Math.ceil(w / 8), Math.ceil(h / 8));
      pass.end();
      for (const write of Object.values(asMap(stage.writes))) {
        this.writtenNow.add(withoutBefore(this.resolve(write)));
      }
      this.ran.push(stage.name);
    }
    return ready;
  }

  /** this frame becomes last frame */
  flip(): void {
    for (const name of this.writtenNow) this.lastWritten.set(name, this.frame);
    this.writtenNow.clear();
    this.parity = 1 - this.parity;
    this.frame++;
  }
  /** forget every history */
  forget(): void {
    this.lastWritten.clear();
  }
}
