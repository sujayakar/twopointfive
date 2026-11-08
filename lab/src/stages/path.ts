import type { Method, Lab } from '../lab';
import walkLib from './walk.wgsl' with { type: 'text' };
import pathMain from './path.wgsl' with { type: 'text' };
import guideLib from './guide.wgsl' with { type: 'text' };
const GSLOTS = 32768, GUIDE: Record<string, [string, string, 'read' | 'read_write']> = { gkeys: ['gkeys', 'array<atomic<u32>>', 'read_write'], train: ['gtrain', 'array<atomic<u32>>', 'read_write'], guide: ['guide', 'array<vec4f>', 'read'] };
const lib = guideLib + walkLib, code = lib + pathMain;
/** settle what the paths have told each patch: the mean of each bin, with the patch's own mean for what is not known yet */
const bakeCode = `
var<workgroup> t: array<vec2f, 128>;
@compute @workgroup_size(128) fn main(@builtin(workgroup_id) g: vec3u, @builtin(local_invocation_index) b: u32) { let s = g.x + g.y * 256u; let sum = f32(train[(s * 128u + b) * 2u]) / 16.0; let n = f32(train[(s * 128u + b) * 2u + 1u]); t[b] = vec2f(sum, n); workgroupBarrier(); var a = vec2f(0.0); for (var k = 0u; k < 128u; k++) { a += t[k]; } if (b < 32u) { var v = vec4f(0.0); if (a.y >= 24.0) { let m = a.x / a.y + 1e-4; for (var k = 0u; k < 4u; k++) { let e = t[b * 4u + k]; v[k] = (e.x + m * 1.5) / (e.y + 1.5) + 0.02 * m; } } guide[s * 32u + b] = v; } }`;
import { glowStages } from './glow';
import { exposeStages } from './expose';
/** the reference: `samples` so far; anything that changes the picture starts it over */
export const pathTracer = (opts: { bounces: number; guided?: boolean; quasi?: boolean; name?: string; edit?: (code: string) => string } = { bounces: 40 }): Method & { samples: number; key: string } => ({
  name: opts.name ?? 'reference', result: 'hdr', samples: 0, key: '',
  signals(g) { g.signal('hdr', 'rgba16float', 'output'); },
  stages(lab: Lab) { const g = [...glowStages(lab.graph, 'hdr', 'ref'), ...exposeStages(lab, lab.graph, 'hdr', 'refGlow2', 'ref')]; lab.uniform('pathKnobs', new Uint32Array([0, opts.bounces, 0, 0])); lab.buffer('sums', 16 * 16); lab.buffer('gkeys', GSLOTS * 4, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST); lab.buffer('gtrain', GSLOTS * 128 * 8, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST); lab.buffer('guide', GSLOTS * 128 * 4, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST); const me = this, clear: import('../graph').Stage = { name: 'guide clear', over: { groups: () => [1, 1, 1] }, code: '', async make(gr) { return enc => { if (me.samples !== 1) return; for (const b of ['gkeys', 'gtrain', 'guide']) enc.clearBuffer(gr.buffers.get(b)!); }; } }, bake: import('../graph').Stage = { name: 'guide bake', over: { groups: () => [256, GSLOTS / 256, 1] }, code: bakeCode, buffers: { train: ['gtrain', 'array<u32>', 'read'], guide: ['guide', 'array<vec4f>', 'read_write'] }, when: () => opts.guided !== false && [2, 4, 8, 16, 32, 64].includes(me.samples) };
    return [clear, { name: 'reference', over: 'output', quasi: opts.quasi !== false, writes: { outHdr: 'hdr' }, buffers: { sums: ['sums', 'array<vec4f>', 'read_write'], pathKnobs: ['pathKnobs', 'vec4u', 'uniform'], ...GUIDE }, code: (opts.edit ?? (c => c))(code) }, bake, ...g]; },
  frame(lab: Lab) {
    const [w, h] = lab.graph.output; lab.buffer('sums', w * h * 16);
    const c = lab.camera, key = [w, h, lab.tick, lab.hour, lab.cut, lab.lampsMode, ...c.look, c.yaw, c.pitch, c.dist].map(x => typeof x === 'number' ? x.toFixed(5) : x).join(); if (key !== this.key) { this.key = key; this.samples = 0; }
    lab.uniform('pathKnobs', new Uint32Array([this.samples, opts.bounces, this.samples, opts.guided !== false && this.samples < 64 ? 1 : 0])); this.samples++;
  },
  reset() { this.samples = 0; this.key = ''; },
});

/** the truth for entries of the light table: at each one's own point, by the same path tracer. For judging a table by itself */
export const truthStage = (lab: Lab, n: number): import('../graph').Stage => { lab.buffer('truth', n * 32); lab.buffer('truthAt', n * 32, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST); lab.uniform('truthK', new Uint32Array(4)); return { name: 'truth', over: { groups: () => [Math.ceil(n / 64), 1, 1] }, buffers: { truth: ['truth', 'array<vec4f>', 'read_write'], places: ['truthAt', 'array<vec4f>', 'read'], truthK: ['truthK', 'vec4u', 'uniform'], pathKnobs: ['pathKnobs', 'vec4u', 'uniform'], ...GUIDE }, code: lib + `
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) { let i = id.x; if (i >= truthK.x) { return; } rngKey(i, 4001u + truthK.y * 7919u); fineLeaves = false; let x = places[i * 2u].xyz; let n = places[i * 2u + 1u].xyz; var E = vec3f(0.0); var sun = 0.0;
  let cs = dot(n, frame.sunDir.xyz); if (cs > 0.0 && max3(frame.sunE.xyz) > 0.0) { sun = lum(visible(x + n * 0.004, coneDir(frame.sunDir.xyz, frame.sunDir.w, rnd2()), 1e-3, 400.0)); }
  let l = drawLamp(x, n); if (l.x >= 0.0) { let a = aimLamp(u32(l.x), x, rnd2()); let e = a.E * l.y * max(dot(n, a.dir), 0.0); if (max3(e) > 0.0) { E += e * visible(x + n * 0.004, a.dir, 1e-3, a.dist - 0.02); } }
  let w = wideDir(x + n * 0.004, n, rnd2(), rnd()); if (w.w > 0.0 && dot(w.xyz, n) > 0.0) { E += walk(x + n * 0.004, w.xyz, 1u, false, 1e9, 0.02, 1.0, 40u) * w.w; }
  var a = vec4f(0.0); var b = vec4f(0.0); if (truthK.y > 0u) { a = truth[i * 2u]; b = truth[i * 2u + 1u]; } truth[i * 2u] = a + vec4f(min(E, vec3f(2000.0)), 1.0); truth[i * 2u + 1u] = b + vec4f(sun, 0.0, 0.0, 0.0); }` }; };
