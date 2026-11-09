// THE UPSCALER WITH A LEARNED HAND: a network of 3,052 weights says how far history may stand off, how much to trust it and how sharply to read the frame, for each patch of
// four by four pixels. What is shown is still only sums of real samples. Trained by lab/learn/up.py against standing poses.
import type { Graph, Stage, Size } from '../graph';
import type { Lab } from '../lab';
import type { UpscalePart } from './realtime';
import hand from './taau.wgsl' with { type: 'text' };
/** the upscaler as it was when the network learned: before the four tuned numbers went in */
const taau = hand.replace('max(sat(mv * 0.5), thick)) * taauK.z)', 'sat(mv * 0.5)) * taauK.z)').replace('* mix(1.9, 1.2, go));', '* 1.9);').replace(') * 1.1;', ');').replace('(n * mix(1.0, 0.55, go) + 1.0)', '(n + 1.0)').replace('let ext = (sd', 'let ext = (sd');
import feats from './upnet.wgsl' with { type: 'text' };
export type Weights = Record<string, number[]>;
/** a 3 x 3 layer as 4 x 4 blocks, tap by tap, input block by input block: columns are inputs */
function pack(w: number[], b: number[], cin: number, cout: number): number[] { const o: number[] = []; for (let ob = 0; ob < cout / 4; ob++) { for (let t = 0; t < 9; t++) for (let ib = 0; ib < cin / 4; ib++) for (let j = 0; j < 4; j++) for (let i = 0; i < 4; i++) o.push(w[((ob * 4 + i) * cin + ib * 4 + j) * 9 + t]!); for (let i = 0; i < 4; i++) o.push(b[ob * 4 + i]!); } return o; }
const layer = (out: number, base: number, act: string) => `
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) { let dim = vec2i(textureDimensions(o0)); if (any(vec2i(id.xy) >= dim)) { return; } var v: array<vec4f, 27>; for (var t = 0; t < 9; t++) { let q = vec2i(id.xy) + vec2i(t % 3 - 1, t / 3 - 1); if (any(q < vec2i(0)) || any(q >= dim)) { continue; } v[t * 3] = textureLoad(i0, q, 0); v[t * 3 + 1] = textureLoad(i1, q, 0); v[t * 3 + 2] = textureLoad(i2, q, 0); }
  ${Array.from({ length: out }, (_, k) => `{ var s = weights[${base + k * 109 + 108}u]; for (var t = 0u; t < 27u; t++) { let w = ${base + k * 109}u + t * 4u; s += weights[w] * v[t].x + weights[w + 1u] * v[t].y + weights[w + 2u] * v[t].z + weights[w + 3u] * v[t].w; } textureStore(o${k}, id.xy, ${act}); }`).join('\n  ')}
}`;
export const learnedUp = (lab: Lab, w: Weights): UpscalePart => ({ name: 'learned', stages(g: Graph) {
  const size: Size = { of: 'output', div: 4 }, flat = [...pack(w['a.weight']!, w['a.bias']!, 12, 12), ...pack(w['b.weight']!, w['b.bias']!, 12, 12), ...pack(w['c.weight']!, w['c.bias']!, 12, 4)]; lab.buffer('upWeights', flat.length * 4, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST); lab.gpu.device.queue.writeBuffer(g.buffers.get('upWeights')!, 0, new Float32Array(flat));
  g.signal('picture', 'rgba16float', 'output', true); for (const n of ['F0', 'F1', 'F2', 'A0', 'A1', 'A2', 'B0', 'B1', 'B2', 'Ctl']) g.signal('up' + n, 'rgba16float', size); const K: NonNullable<Stage['buffers']> = { weights: ['upWeights', 'array<vec4f>', 'read'] }, t: NonNullable<Stage['buffers']> = { taauK: ['taauK', 'vec4f', 'uniform'] }, in3 = (p: string) => ({ ['up' + p + '0']: 'i0', ['up' + p + '1']: 'i1', ['up' + p + '2']: 'i2' }), out3 = (p: string) => ({ o0: 'up' + p + '0', o1: 'up' + p + '1', o2: 'up' + p + '2' });
  const code = taau.replace('let w = exp(-dot(d, d) * 1.9);', 'let w = exp(-dot(d, d) * 1.9 * exp(ctl.x));').replace('var acc = vec3f(0.0); var wa', 'let ctl = textureSampleLevel(upCtl, linSamp, (vec2f(id.xy) + 0.5) / od, 0.0); var acc = vec3f(0.0); var wa').replace('sat(mv * 0.5)) * taauK.z)', 'sat(mv * 0.5)) * taauK.z * exp(ctl.w))').replace('vec3f(0.02, 0.01, 0.01) * mean.x);', 'vec3f(0.02, 0.01, 0.01) * mean.x) * exp(ctl.y);').replace('/ (n + 1.0); o = mix', '/ (n * exp(ctl.z) + 1.0); o = mix');
  if ((code.match(/ctl\./g) ?? []).length !== 4) throw new Error('the upscaler has changed under the learned hand'); 
  return { result: 'picture', stages: [{ name: 'up feats', over: size, reads: { color: 'color', 'picture@prev': 'hist', aux: 'auxT' }, writes: { f0: 'upF0', f1: 'upF1', f2: 'upF2' }, buffers: t, code: feats }, { name: 'up net', over: size, reads: in3('F'), writes: out3('A'), buffers: K, code: layer(3, 0, 'max(s, vec4f(0.0))') }, { name: 'up net', over: size, reads: in3('A'), writes: out3('B'), buffers: K, code: layer(3, 327, 'max(s, vec4f(0.0))') }, { name: 'up net', over: size, reads: in3('B'), writes: { o0: 'upCtl' }, buffers: K, code: layer(1, 654, 'tanh(s) * 1.5') },
    { name: 'upscale', over: 'output', reads: { color: 'color', 'picture@prev': 'hist', aux: 'auxT', upCtl: 'upCtl', airD: 'airD' }, writes: { outT: 'picture' }, buffers: t, code }] };
} });
