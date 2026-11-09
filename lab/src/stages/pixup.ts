// THE UPSCALER WITH A LEARNED HAND AT EVERY PIXEL: a network of a thousand weights weighs the nine samples round an output pixel one by one, and says how much to trust history and how far it
// may stand off. What is shown is still only sums of real samples, held between the darkest and brightest of them. Trained by lab/learn/pix.py against standing poses.
import type { Graph } from '../graph';
import type { Lab } from '../lab';
import type { UpscalePart } from './realtime';
import hand from './taau.wgsl' with { type: 'text' };
export type Weights = Record<string, number[]>;
const IN = 21;
/** layer by layer, input by input, four outputs at a time; then the biases */
function pack(w: Weights): { flat: number[]; sizes: [number, number][]; at: number[] } {
  const flat: number[] = [], sizes: [number, number][] = [], at: number[] = []; for (const l of ['a', 'b', 'c']) { const b = w[l + '.bias']!, W = w[l + '.weight']!, out = b.length, inn = W.length / out; sizes.push([inn, out]); at.push(flat.length / 4); for (let i = 0; i < inn; i++) for (let o = 0; o < out; o++) flat.push(W[o * inn + i]!); for (let o = 0; o < out; o++) flat.push(b[o]!); } return { flat, sizes, at };
}
export const pixUp = (lab: Lab, w: Weights, name = 'pixels'): UpscalePart => ({ name, stages(g: Graph) {
  const { flat, sizes, at } = pack(w), buf = name + 'Weights'; lab.uniform(buf, new Float32Array(flat)); g.signal('picture', 'rgba16float', 'output', true);
  const layer = (n: number, x: (i: number) => string, act: (s: string) => string) => { const [inn, out] = sizes[n]!; return Array.from({ length: out / 4 }, (_, o) => `let ${'abc'[n]}${o} = ${act(`${Array.from({ length: inn }, (_, i) => `W[${at[n]! + i * out / 4 + o}u] * ${x(i)}`).join(' + ')} + W[${at[n]! + inn * out / 4 + o}u]`)};`).join('\n    '); }, c4 = 'xyzw', of = (p: string) => (i: number) => `${p}${i >> 2}.${c4[i & 3]}`;
  // the hand's own text up to where the nine samples are weighed; they are kept, and weighed after
  let code = hand.replace('let v = textureLoad(color, t, 0); let c = ycc(tm(v.rgb));', 'let v = textureLoad(color, t, 0); let c = ycc(tm(v.rgb)); let ki = u32(x + 1 + 3 * (y + 1)); tap[ki] = c; dt[ki] = v.a;')
    .replace('let w = exp(-dot(d, d) * mix(4.0, 1.2, go)); acc += c * w; wa += w;', 'lw[ki] = -dot(d, d) * mix(4.0, 1.2, go); lo = min(lo, c); hi9 = max(hi9, c); if (x == 0 && y == 0) { off = d; }')
    .replace('var prox = 0.0;', 'var prox = 0.0; var tap: array<vec3f, 9>; var dt: array<f32, 9>; var lw: array<f32, 9>; var lo = vec3f(1e9); var hi9 = vec3f(-1e9); var off = vec2f(0.0);')
    .replace('let cur = select(mean, acc / max(wa, 1e-6), wa > 1e-4);', '')
    .replace('let uv = so / od + aux.xy; var o = cur; var n = 0.0;', `let uv = so / od + aux.xy; var n = 0.0; let inside = all(uv > vec2f(0.0)) && all(uv < vec2f(1.0)); let ph = cat(uv, od); let phy = ycc(tm(select(ph, vec4f(0.0), bad4(ph)).rgb)); let my = abs(mean.x) + 0.05; let df = phy - mean; let cm = lim(vec3f(0.0)).x;
    let i0 = vec4f(${[0, 1, 2, 3].map(k => `lim1((tap[${k}].x - mean.x) / my)`).join(', ')}); let i1 = vec4f(${[4, 5, 6, 7].map(k => `lim1((tap[${k}].x - mean.x) / my)`).join(', ')});
    let i2 = vec4f(lim1((tap[8].x - mean.x) / my), off, lim1(df.x / (sd.x + 0.02 * my + 1e-3) * 0.2)); let i3 = vec4f(lim1(df.x / my), min(length(df.yz) / my, 3.0), min(sd.x / my, 3.0), log(my) * 0.3);
    let i4 = vec4f(max(ph.a, 0.0) / 28.0, min(length(aux.xy * od) * 0.25, 3.0), select(0.0, 1.0, inside), prox); let i5 = vec4f(clamp((dt[4] - near) / max(near, 0.1), 0.0, 2.0), 0.0, 0.0, 0.0);
    ${layer(0, of('i'), s => `max(${s}, vec4f(0.0))`)}
    ${layer(1, of('a'), s => `max(${s}, vec4f(0.0))`)}
    ${layer(2, of('b'), s => `tanh(${s})`)}
    { let dl = array<f32, 9>(${Array.from({ length: 9 }, (_, k) => of('c')(k)).join(', ')}); for (var k = 0; k < 9; k++) { let w = exp(lw[k] + 2.5 * dl[k]); acc += tap[k] * w; wa += w; } }
    var cur = acc / max(wa, 1e-6); cur = clamp(cur, lo, hi9); let trust = exp(2.0 * ${of('c')(9)}); let wide = exp(1.5 * ${of('c')(10)}); var o = cur; _ = cm;`)
    .replace('* 1.1; let dv', '* 1.1 * wide; let dv').replace('/ (n * mix(1.0, 0.55, go) + 1.0)', '/ (n * mix(1.0, 0.55, go) * trust + 1.0)');
  code = 'fn lim1(x: f32) -> f32 { return clamp(x, -3.0, 3.0); }\nfn lim(x: vec3f) -> vec3f { return x; }\n' + code; code = code.replace(' let cm = lim(vec3f(0.0)).x;', '').replace(' _ = cm;', '').replace('fn lim(x: vec3f) -> vec3f { return x; }\n', '');
  if (!code.includes('* trust') || !code.includes('* wide') || !code.includes('lw[ki]') || !code.includes('tap[ki]') || sizes[0]![0] !== IN) throw new Error('the upscaler has changed under the learned hand');
  return { result: 'picture', stages: [{ name: 'upscale', over: 'output', reads: { color: 'color', 'picture@prev': 'hist', aux: 'auxT', airD: 'airD' }, writes: { outT: 'picture' }, buffers: { taauK: ['taauK', 'vec4f', 'uniform'], W: [buf, `array<vec4f, ${flat.length / 4}>`, 'uniform'] }, code }] };
} });
