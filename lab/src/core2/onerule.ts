// THE REAL-TIME METHOD ON THE SECOND CORE: the same stages, asked of the other rays. What differs is put here and nowhere else.
import { realtime, bilinear, CHANGES, type UpscalePart } from '../stages/realtime';
import type { Method, Lab } from '../lab';
import type { Stage } from '../graph';
import signals from '../wgsl/signals.wgsl' with { type: 'text' };
import bounceLib from '../wgsl/bounce.wgsl' with { type: 'text' };
import cells from './cells.wgsl' with { type: 'text' };
import { secondCore } from './core';
import { lightRow } from './blocks';
const swap = (code: string, a: string, b: string) => { if (!code.includes(a)) throw new Error('a stage has no ' + a); return code.replace(a, () => b); };
/** `leafNear`: how far bounce rays meet leaves one by one, as a share of how far other things as small */
export function oneRule(o: { name?: string; leafNear?: number; upscale?: UpscalePart; edit?: (s: Stage) => Stage } = {}): Method {
  let lab0: Lab | null = null; const core = (p: string) => secondCore(p, false, o.leafNear ?? 0);
  const edit = (s: Stage): Stage => {
    if (s.name === 'foliage') {
      const lab = lab0!, many = lab.litCells.many;
      if (s.make) return { ...s, async make() { return enc => { if (many) enc.copyBufferToBuffer(lab.graph.buffers.get('cellLight')!, 0, lab.scene.extraBuffer, lightRow(lab.scene.blocks, lab.scene.blockRow) * 16, many * 48); }; } };
      const { field: _f, folAt: _a, ...bind } = s.buffers!, lib = s.code.slice(signals.length, s.code.indexOf(bounceLib));
      s = { name: 'cells', over: { groups: () => [Math.ceil(many / 64), 1, 1] }, when: () => many > 0, code: signals + lib + bounceLib + cells, buffers: { ...bind, ...CHANGES, field: ['cellLight', 'array<u32>', 'read_write'], places: ['cellsAt', 'array<vec4f>', 'read'], many: ['cellsMany', 'vec4u', 'uniform'] } };
    } else if (s.name === 'mirror' || s.name === 'shine') s = { ...s, code: swap(s.code, 'select(m, q.albedo + q.through, leaf) / PI * c.E * select(0.0, 1.0, c.n > 0.0)', 'select(m / PI * c.E * select(0.0, 1.0, c.n > 0.0), folJ(q.x), leaf)') };
    else if (s.name === 'air light') s = { ...s, code: swap(s.code, 'L += min(B * h.thru, vec3f(12.0));', 'L += min(B * h.thru * blkT + blkL, vec3f(12.0));') };
    if (!s.make && !s.alone && s.code) s = { ...s, core }; return o.edit ? o.edit(s) : s;
  };
  const m: Method = realtime({ name: o.name ?? 'onerule', edit, ...(o.upscale ? { upscale: o.upscale } : {}) }), was = m.signals; m.core2 = true;
  m.signals = (g, lab) => { lab0 = lab; was.call(m, g, lab); const n = Math.max(1, lab.litCells.many); if (!lab.graph.buffers.has('cellLight')) { lab.buffer('cellLight', n * 48, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST); lab.buffer('cellsAt', Math.max(32, lab.litCells.at.byteLength), GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST); lab.gpu.device.queue.writeBuffer(lab.graph.buffers.get('cellsAt')!, 0, lab.litCells.at); lab.uniform('cellsMany', new Uint32Array([lab.litCells.many, 0, 0, 0])); } };
  return m;
}
export const oneRuleMethods: Record<string, () => Method> = { onerule: () => oneRule(), oneleaf: () => oneRule({ name: 'oneleaf', leafNear: 1 }), onelin: () => oneRule({ name: 'onelin', upscale: bilinear }), oneleaflin: () => oneRule({ name: 'oneleaflin', leafNear: 1, upscale: bilinear }) };
