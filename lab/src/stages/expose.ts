// LOCAL EXPOSURE by fusing three exposures of the picture (Mertens, Kautz, Van Reeth): where one of them is well exposed it is taken, blended over a pyramid,
// so that a dark room and a white roof are both seen and no edge between them has a halo.
import type { Lab } from '../lab';
import type { Graph, Stage, Size } from '../graph';
export const CURVE = `
fn neutral(c0: vec3f) -> vec3f {                 // Khronos PBR Neutral: base colors come through as painted; only what is brighter is rolled off
  let start = 0.8 - 0.04; let des = 0.15; let x = min(c0.r, min(c0.g, c0.b)); let off = select(0.04, x - 6.25 * x * x, x < 0.08); var c = c0 - off;
  let peak = max(c.r, max(c.g, c.b)); if (peak < start) { return c; } let d = 1.0 - start; let np = 1.0 - d * d / (peak + d - start); c *= np / peak; let g = 1.0 - 1.0 / (des * (peak - np) + 1.0); return mix(c, vec3f(np), g);
}
fn curve(l: f32) -> f32 { return sqrt(clamp(neutral(vec3f(max(l, 0.0))).x, 0.0, 1.0)); }
fn uncurve(p: f32) -> f32 { let y = clamp(p, 0.0, 0.995) * clamp(p, 0.0, 0.995); if (y < 0.04) { return sqrt(y / 6.25); } let z = y; if (z < 0.76) { return z + 0.04; } let d = 0.24; return d * d / max(1.0 - z, 1e-3) - d + 0.76 + 0.04; }
`;
const LEVELS = 5;
export function exposeStages(lab: Lab, g: Graph, picture: string, small: string, pre: string): Stage[] {
  lab.buffer('exposure', 16); const E: Stage['buffers'] = { e: ['exposure', 'array<f32, 4>', 'read'], k: ['exposeK', 'vec4f', 'uniform'] }, sz = (i: number): Size => ({ of: 'output', div: 4 * 2 ** i }), out: Stage[] = [];
  for (let i = 0; i <= LEVELS; i++) g.signal(`${pre}Lum${i}`, 'rgba16float', sz(i)).signal(`${pre}Wt${i}`, 'rgba16float', sz(i)).signal(`${pre}Fuse${i}`, 'rgba16float', sz(i));
  out.push({ name: 'meter', over: { groups: () => [1, 1, 1] }, reads: { [small]: 'src' }, buffers: { e: ['exposure', 'array<f32, 4>', 'read_write'], k: ['exposeK', 'vec4f', 'uniform'] }, code: `
var<workgroup> s: array<vec2f, 256>;
@compute @workgroup_size(256) fn main(@builtin(local_invocation_index) i: u32) {
  let n = textureDimensions(src); var a = vec2f(0.0);
  for (var j = 0u; j < 24u; j++) { let t = i * 24u + j; let p = vec2f(f32(t % 96u) + 0.5, f32(t / 96u) + 0.5) / vec2f(96.0, 64.0); let c = textureLoad(src, vec2u(p * vec2f(n)), 0).xyz; let l = log2(clamp(lum(c), 1e-4, 1e3)); let q = (p - 0.5) * 1.6; let w = exp(-dot(q, q) * 2.2); a += vec2f(l * w, w); }
  s[i] = a; workgroupBarrier(); for (var st = 128u; st > 0u; st >>= 1u) { if (i < st) { s[i] += s[i + st]; } workgroupBarrier(); }
  if (i == 0u) { let mean = finite(s[0].x / max(s[0].y, 1e-4), 30.0); let want = exp2(clamp(-1.75 - mean * 0.86, -7.0, 6.0)); let was = finite(e[0], 1e6); let r = select(0.05, 1.0, was <= 0.0 || e[1] != 1.0 || k.w > 0.0); e[0] = exp2(mix(log2(max(was, 1e-6)), log2(want), r)); e[1] = 1.0; e[2] = mean; }
}` });
  out.push({ name: 'exposures', over: sz(0), reads: { [small]: 'src' }, writes: { outL: `${pre}Lum0`, outW: `${pre}Wt0` }, buffers: E, code: CURVE + `
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) { let d = textureDimensions(outL); if (id.x >= d.x || id.y >= d.y) { return; } let l = lum(textureLoad(src, min(id.xy, textureDimensions(src) - 1u), 0).rgb) * e[0] * exp2(k.x);
  let v = vec3f(curve(l * exp2(-k.z)), curve(l), curve(l * exp2(k.y))); var w = exp(-0.5 * (v - 0.52) * (v - 0.52) / (0.2 * 0.2)) + 1e-4; w /= w.x + w.y + w.z; textureStore(outL, id.xy, vec4f(v, 1.0)); textureStore(outW, id.xy, vec4f(w, 1.0)); }` });
  for (let i = 1; i <= LEVELS; i++) for (const t of ['Lum', 'Wt']) out.push({ name: 'exposures', over: sz(i), reads: { [`${pre}${t}${i - 1}`]: 'src' }, writes: { outT: `${pre}${t}${i}` }, code: `
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) { let d = textureDimensions(outT); if (id.x >= d.x || id.y >= d.y) { return; } let m = vec2i(textureDimensions(src)) - 1; var a = vec4f(0.0); for (var y = -1; y <= 2; y++) { for (var x = -1; x <= 2; x++) { a += textureLoad(src, clamp(vec2i(id.xy) * 2 + vec2i(x, y), vec2i(0), m), 0) * (select(1.0, 3.0, x == 0 || x == 1) * select(1.0, 3.0, y == 0 || y == 1) / 64.0); } } textureStore(outT, id.xy, a); }` });
  for (let i = LEVELS; i >= 0; i--) out.push({ name: 'fuse', over: sz(i), reads: i === LEVELS ? { [`${pre}Lum${i}`]: 'L', [`${pre}Wt${i}`]: 'W' } : { [`${pre}Lum${i}`]: 'L', [`${pre}Wt${i}`]: 'W', [`${pre}Lum${i + 1}`]: 'Lc', [`${pre}Fuse${i + 1}`]: 'Fc' }, writes: { outF: `${pre}Fuse${i}` }, code: `
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) { let d = textureDimensions(outF); if (id.x >= d.x || id.y >= d.y) { return; } let l = textureLoad(L, id.xy, 0).xyz; var w = textureLoad(W, id.xy, 0).xyz; w /= max(w.x + w.y + w.z, 1e-4); ${i === LEVELS ? 'let f = dot(l, w);' : 'let uv = (vec2f(id.xy) + 0.5) / vec2f(d); let c = textureSampleLevel(Lc, linSamp, uv, 0.0).xyz; let f = textureSampleLevel(Fc, linSamp, uv, 0.0).x + dot(w, l - c);'} textureStore(outF, id.xy, vec4f(f, l.y, 0.0, 1.0)); }` });
  void picture; return out;
}
/** stops added in all, stops the shadows may be lifted, stops the lights may be held, snap */
export const exposeKnobs = (lab: Lab, bias: number, lift: number, hold: number, snap: boolean) => lab.uniform('exposeK', new Float32Array([bias, lift, hold, snap ? 1 : 0]));
