import type { Method, Lab } from '../lab';
import walkLib from './walk.wgsl' with { type: 'text' };
import pathMain from './path.wgsl' with { type: 'text' };
const code = walkLib + pathMain;
import { glowStages } from './glow';
import { exposeStages } from './expose';
/** the reference: `samples` so far; anything that changes the picture starts it over */
export const pathTracer = (opts = { bounces: 40 }): Method & { samples: number; key: string } => ({
  name: 'reference', result: 'hdr', samples: 0, key: '',
  signals(g) { g.signal('hdr', 'rgba16float', 'output'); },
  stages(lab: Lab) { const g = [...glowStages(lab.graph, 'hdr', 'ref'), ...exposeStages(lab, lab.graph, 'hdr', 'refGlow2', 'ref')]; lab.uniform('pathKnobs', new Uint32Array([0, opts.bounces, 0, 0])); lab.buffer('sums', 16 * 16); return [{ name: 'reference', over: 'output', writes: { outHdr: 'hdr' }, buffers: { sums: ['sums', 'array<vec4f>', 'read_write'], pathKnobs: ['pathKnobs', 'vec4u', 'uniform'] }, code }, ...g]; },
  frame(lab: Lab) {
    const [w, h] = lab.graph.output; lab.buffer('sums', w * h * 16);
    const c = lab.camera, key = [w, h, lab.tick, lab.hour, lab.cut, lab.lampsMode, ...c.look, c.yaw, c.pitch, c.dist].map(x => typeof x === 'number' ? x.toFixed(5) : x).join(); if (key !== this.key) { this.key = key; this.samples = 0; }
    lab.uniform('pathKnobs', new Uint32Array([this.samples, opts.bounces, this.samples, 0])); this.samples++;
  },
  reset() { this.samples = 0; this.key = ''; },
});

/** the truth for entries of the light table: at each one's own point, by the same path tracer. For judging a table by itself */
export const truthStage = (lab: Lab, n: number): import('../graph').Stage => { lab.buffer('truth', n * 32); lab.buffer('truthAt', n * 32, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST); lab.uniform('truthK', new Uint32Array(4)); return { name: 'truth', over: { groups: () => [Math.ceil(n / 64), 1, 1] }, buffers: { truth: ['truth', 'array<vec4f>', 'read_write'], places: ['truthAt', 'array<vec4f>', 'read'], truthK: ['truthK', 'vec4u', 'uniform'] }, code: walkLib + `
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) { let i = id.x; if (i >= truthK.x) { return; } rngKey(i, 4001u + truthK.y * 7919u); fineLeaves = false; let x = places[i * 2u].xyz; let n = places[i * 2u + 1u].xyz; var E = vec3f(0.0); var sun = 0.0;
  let cs = dot(n, frame.sunDir.xyz); if (cs > 0.0 && max3(frame.sunE.xyz) > 0.0) { sun = lum(visible(x + n * 0.004, coneDir(frame.sunDir.xyz, frame.sunDir.w, rnd2()), 1e-3, 400.0)); }
  let l = drawLamp(x, n); if (l.x >= 0.0) { let a = aimLamp(u32(l.x), x, rnd2()); let e = a.E * l.y * max(dot(n, a.dir), 0.0); if (max3(e) > 0.0) { E += e * visible(x + n * 0.004, a.dir, 1e-3, a.dist - 0.02); } }
  let w = wideDir(x + n * 0.004, n, rnd2(), rnd()); if (w.w > 0.0 && dot(w.xyz, n) > 0.0) { E += walk(x + n * 0.004, w.xyz, 1u, false, 1e9, 0.02, 1.0, 40u) * w.w; }
  var a = vec4f(0.0); var b = vec4f(0.0); if (truthK.y > 0u) { a = truth[i * 2u]; b = truth[i * 2u + 1u]; } truth[i * 2u] = a + vec4f(min(E, vec3f(2000.0)), 1.0); truth[i * 2u + 1u] = b + vec4f(sun, 0.0, 0.0, 0.0); }` }; };
