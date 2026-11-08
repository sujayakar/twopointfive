import type { Method, Lab } from '../lab';
import code from './path.wgsl' with { type: 'text' };
import { glowStages } from './glow';
import { exposeStages } from './expose';
/** the reference: `samples` so far; anything that changes the picture starts it over */
export const pathTracer = (opts = { bounces: 6 }): Method & { samples: number; key: string } => ({
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
