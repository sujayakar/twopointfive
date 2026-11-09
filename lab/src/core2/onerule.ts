// THE REAL-TIME METHOD ON THE SECOND CORE: the same stages, asked of the other rays. What differs is put here and nowhere else.
import { realtime, bilinear, CHANGES, type UpscalePart } from '../stages/realtime';
import type { Method, Lab } from '../lab';
import type { Stage } from '../graph';
import signals from '../wgsl/signals.wgsl' with { type: 'text' };
import bounceLib from '../wgsl/bounce.wgsl' with { type: 'text' };
import cells from './cells.wgsl' with { type: 'text' };
import { secondCore } from './core';
import smokeMean from './smokemean.wgsl' with { type: 'text' };
import { lightRow, smokeRow, SMOKE_ROWS } from './blocks';
import { PEOPLE, LIMBS } from './carry';
const SENT = 'fn sentBack(q: Surf, leaf: bool, j: vec2f, rd: vec3f) -> vec3f {';
const swap = (code: string, a: string, b: string) => { if (!code.includes(a)) throw new Error('a stage has no ' + a); return code.replace(a, () => b); };
/** `leafNear`: how far bounce rays meet leaves one by one, as a share of how far other things as small */
export function oneRule(o: { name?: string; leafNear?: number; upscale?: UpscalePart; /** shadows of lamps at what is seen by the cells too */ softLamps?: boolean; /** changes the rays' text, by stage */ put?: (prelude: string, stage: string) => string; edit?: (s: Stage) => Stage } = {}): Method {
  let lab0: Lab | null = null; const core = (p: string) => secondCore(p, false, o.leafNear ?? 0);
  const edit = (s: Stage): Stage => {
    if (s.name === 'foliage') {
      const lab = lab0!, many = lab.litCells.many;
      if (s.make) return { ...s, /* (looked at every other frame: what it costs is mostly waiting) */ async make() { return enc => { enc.copyBufferToBuffer(lab.graph.buffers.get('cellLight')!, 0, lab.scene.extraBuffer, lightRow(lab.scene.blocks, lab.scene.blockRow) * 16, (many + PEOPLE * LIMBS) * 48); }; } };
      const { field: _f, folAt: _a, ...bind } = s.buffers!, lib = s.code.slice(signals.length, s.code.indexOf(bounceLib));
      s = { name: 'cells', when: () => lab.frameNo % 2 === 0 || lab.frameNo < 40, over: { groups: () => [Math.ceil((many + PEOPLE * LIMBS) / 8), 1, 1] }, code: signals + lib + bounceLib + cells, buffers: { ...bind, ...CHANGES, field: ['cellLight', 'array<u32>', 'read_write'], places: ['cellsAt', 'array<vec4f>', 'read'], many: ['cellsMany', 'vec4u', 'uniform'] } };
    } else if (s.name === 'mirror' || s.name === 'shine') s = { ...s, code: swap(s.code, 'select(m, q.albedo + q.through, leaf) / PI * c.E * select(0.0, 1.0, c.n > 0.0)', 'select(m / PI * c.E * select(0.0, 1.0, c.n > 0.0), folJ(q.x, d), leaf)') };
    else if (s.name === 'lamps') s = { ...s, code: swap(s.code, 'leafTo = select(0.0, 1e9, exact);', o.softLamps ? 'leafTo = select(0.0, 1e9, exact);' : 'footSharp(footW, footS); if (exact) { leafTo = 1e9; }') };
    else if (s.name === 'air light') s = { ...s, code: swap(s.code, 'L += min(B * h.thru, vec3f(12.0));', 'L += min(B * h.thru * blkT + blkL, vec3f(12.0));') };
    if (s.code.includes(SENT)) s = { ...s, code: swap(s.code, SENT, SENT + ' if (leaf && lastLimb > 0u) { return limbSent(rd); }') };
    if (!s.make && !s.alone && s.code) s = { ...s, core: o.put ? p => o.put!(core(p), s.name) : core }; return o.edit ? o.edit(s) : s;
  };
  const m: Method = realtime({ name: o.name ?? 'onerule', edit, ...(o.upscale ? { upscale: o.upscale } : {}) }), was = m.signals, stagesWere = m.stages; m.core2 = true;
  m.stages = lab => { const all = stagesWere.call(m, lab), i = all.findIndex(s => s.name === 'smoke keep'); if (i < 0) return all; lab.buffer('smokeMean', SMOKE_ROWS * 16, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST); const when = () => lab.smoke.on;
    return [...all.slice(0, i), { name: 'smoke mean', over: { groups: () => [SMOKE_ROWS, 1, 1] }, when, code: smokeMean, buffers: { mean: ['smokeMean', 'array<vec4f>', 'read_write'] } }, { name: 'smoke mean', over: { groups: () => [1, 1, 1] }, code: '', async make() { return enc => { if (when()) enc.copyBufferToBuffer(lab.graph.buffers.get('smokeMean')!, 0, lab.scene.extraBuffer, smokeRow(lab.scene.blocks, lab.scene.blockRow) * 16, SMOKE_ROWS * 16); }; } }, ...all.slice(i)]; };
  m.signals = (g, lab) => { lab0 = lab; was.call(m, g, lab); const n = lab.litCells.many + PEOPLE * LIMBS; if (!lab.graph.buffers.has('cellLight')) { lab.buffer('cellLight', n * 48, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST); lab.buffer('cellsAt', Math.max(32, lab.litCells.at.byteLength), GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST); lab.gpu.device.queue.writeBuffer(lab.graph.buffers.get('cellsAt')!, 0, lab.litCells.at); lab.uniform('cellsMany', new Uint32Array([lab.litCells.many, PEOPLE * LIMBS, 0, 0])); } };
  return m;
}
const also = (name: string, put: (s: Stage) => Stage) => () => oneRule({ name, upscale: bilinear, edit: put }), code = (s: Stage, a: string, b: string): Stage => ({ ...s, code: swap(s.code, a, b) });
export const oneRuleMethods: Record<string, () => Method> = {
  softlamps: () => oneRule({ name: 'softlamps', upscale: bilinear, softLamps: true }),
  // what one stage's rays cost in truth: that stage's rays as good as exact
  exactgather: also('exactgather', s => s.name === 'gather' ? code(s, 'footSolid(wide, @tune.wide);', 'footSolid(0.0, 0.004);') : s),
  exactcache: also('exactcache', s => s.name === 'cache' && s.code.includes('footSolid(CELL, 1.0);') ? code(s, 'footSolid(CELL, 1.0);', 'footSolid(0.0, 0.004);') : s),
  exactboth: also('exactboth', s => s.name === 'gather' ? code(s, 'footSolid(wide, @tune.wide);', 'footSolid(0.0, 0.004);') : s.name === 'cache' && s.code.includes('footSolid(CELL, 1.0);') ? code(s, 'footSolid(CELL, 1.0);', 'footSolid(0.0, 0.004);') : s),
   // what costs what: no cells met, anywhere or by lamps' shadows; no limbs; small things dropped as early as the first core does
  nocells: () => oneRule({ name: 'nocells', upscale: bilinear, put: p => p.replaceAll('if (blkOn && footS > 0.0 && min(blk.x, leafTo) < best)', 'if (false)').replace('if (blkOn && footS > 0.0) { let b0', 'if (false) { let b0') }),
  lampsno: () => oneRule({ name: 'lampsno', upscale: bilinear, put: (p, s) => s === 'lamps' ? p.replace('if (blkOn && footS > 0.0) { let b0', 'if (false) { let b0') : p }),
  nolimbs: () => oneRule({ name: 'nolimbs', upscale: bilinear, put: p => p.replace('if (blkOn && footS > 0.0) { let c = carryHit', 'if (false) { let c = carryHit').replace('if (blkOn && footS > 0.0 && !leafOnly) { T *= exp(-carryTau', 'if (false) { T *= exp(-carryTau') }),
  early: () => oneRule({ name: 'early', upscale: bilinear, put: p => p.replace('lim = vec4f(t.xy, 1e9, 1e9);', 'lim = vec4f(0.6 * t.x, t.y, 1e9, 1e9); blk.x = lim.x;') }),
  one25: also('one25', s => s.name === 'gather' ? code(s, 'footSolid(wide, @tune.wide);', 'footSolid(wide, 0.5 * @tune.wide);') : s),
  onerule: () => oneRule(), oneleaf: () => oneRule({ name: 'oneleaf', leafNear: 1 }), onelin: () => oneRule({ name: 'onelin', upscale: bilinear }), oneleaflin: () => oneRule({ name: 'oneleaflin', leafNear: 1, upscale: bilinear }) };
