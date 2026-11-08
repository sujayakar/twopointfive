// WHAT THE SCREEN CAN SHOW. The picture's light is measured (meter), and split in two: what is broad (a base that keeps the edges of rooms and shadows, by a
// guided filter over a small picture of its stops) and what is fine. The base is drawn together, gently near the measure and more far from it; the fine part is
// kept, and a little more. So a dark room and a white roof both show and neither goes flat. Then one curve, the same for all.
import type { Lab } from '../lab';
import type { Graph, Stage, Size } from '../graph';
export const CURVE = `
/** the curve: a toe that keeps black black, a straight middle, a shoulder (Uchimura) */
fn film(x: vec3f) -> vec3f {
  let a = @tune.contrast; let m = 0.22; let l0 = (1.0 - m) * 0.4 / a; let s0 = m + l0; let s1 = m + a * l0; let c2 = a / (1.0 - s1); let t = clamp(x / m, vec3f(0.0), vec3f(1.0)); let w0 = 1.0 - t * t * (3.0 - 2.0 * t); let w2 = step(vec3f(s0), x);
  return clamp(w0 * (m * pow(max(x, vec3f(1e-8)) / m, vec3f(@tune.toe))) + (1.0 - w0 - w2) * (m + a * (x - m)) + w2 * (1.0 - (1.0 - s1) * exp(-c2 * (x - s0))), vec3f(0.0), vec3f(1.0));
}
/** how strong a light is to the curve: a pure color fills the screen by its strongest part, not by its luminance */
fn strong(c: vec3f) -> f32 { return mix(dot(c, vec3f(0.2126, 0.7152, 0.0722)), max(c.r, max(c.g, c.b)), 0.45); }
/** by the parts, and by the light with the hue held: between them */
fn shown(c0: vec3f) -> vec3f {
  let y0 = dot(c0, vec3f(0.2126, 0.7152, 0.0722)); let mx = max(c0.r, max(c0.g, c0.b)); let sat = (mx - min(c0.r, min(c0.g, c0.b))) / max(mx, 1e-4); let c = max(vec3f(0.0), y0 + (c0 - y0) * (1.0 + @tune.color * (1.0 - 0.6 * sat)));
  let y = max(strong(c), 1e-6); var h = c * (film(vec3f(y)).x / y); let hm = max(h.r, max(h.g, h.b)); if (hm > 1.0) { h = 0.5 * h / hm + 0.5 * min(h, vec3f(1.0)); } return mix(film(c), h, @tune.hue);
}
/** stops to add: l = this point's stops, base = the base's here (a, b of the guided filter, about the measure A) */
fn stops(l: f32, A: f32, ab: vec2f) -> f32 { let d = ab.x * (l - A) + ab.y; let out = @tune.key + A * 0.1 + select(-log(1.0 - 0.05 * min(d, 0.0)) * (@tune.low / 0.05), log(1.0 + 0.10 * max(d, 0.0)) * (@tune.high / 0.10), d > 0.0) + (l - A - d) * @tune.detail; return out - l; }
`;
const R = 14, BASE: Size = [160, 90];
export function exposeStages(lab: Lab, g: Graph, picture: string, small: string, pre: string): Stage[] {
  lab.buffer('exposure', 16); const E: Stage['buffers'] = { e: ['exposure', 'array<f32, 4>', 'read'] }, n = (k: string) => `${pre}Base${k}`;
  for (const k of ['0', '1', '2', '3', '']) g.signal(n(k), 'rgba16float', BASE);
  const box = (from: string, to: string, dx: number, dy: number, then: string): Stage => ({ name: 'base', over: BASE, reads: { [from]: 'src' }, writes: { outT: to }, code: `
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) { let d = vec2i(textureDimensions(outT)); if (any(vec2i(id.xy) >= d)) { return; } var m = vec4f(0.0); for (var i = -${R}; i <= ${R}; i++) { m += textureLoad(src, clamp(vec2i(id.xy) + vec2i(i * ${dx}, i * ${dy}), vec2i(0), d - 1), 0); } m /= ${2 * R + 1}.0; ${then} textureStore(outT, id.xy, m); }` });
  void picture;
  return [
    { name: 'meter', over: { groups: () => [1, 1, 1] }, reads: { [small]: 'src' }, buffers: { e: ['exposure', 'array<f32, 4>', 'read_write'], k: ['exposeK', 'vec4f', 'uniform'] }, code: CURVE + `
var<workgroup> s: array<vec2f, 256>;
fn sum(i: u32, v: vec2f) -> vec2f { workgroupBarrier(); s[i] = v; workgroupBarrier(); for (var st = 128u; st > 0u; st >>= 1u) { if (i < st) { s[i] += s[i + st]; } workgroupBarrier(); } return workgroupUniformLoad(&s[0]); }
@compute @workgroup_size(256) fn main(@builtin(local_invocation_index) i: u32) {
  // the mean of the stops, more at the middle; and the mean of what is not far over that, so that a lit street outside does not speak for the rooms
  let n = textureDimensions(src); var l: array<vec2f, 24>; var a = vec2f(0.0);
  for (var j = 0u; j < 24u; j++) { let t = i * 24u + j; let p = vec2f(f32(t % 96u) + 0.5, f32(t / 96u) + 0.5) / vec2f(96.0, 64.0); let pin = k.y > 0.0; var q = (p - select(vec2f(0.5), k.yz, pin)) * select(1.6, 5.0, pin); l[j] = vec2f(log2(clamp(strong(textureLoad(src, vec2u(p * vec2f(n)), 0).xyz), 1e-4, 1e3)), exp(-dot(q, q) * 2.2)); a += vec2f(l[j].x, 1.0) * l[j].y; }
  let t = sum(i, a); let mean = t.x / t.y; a = vec2f(0.0); for (var j = 0u; j < 24u; j++) { a += vec2f(l[j].x, 1.0) * (l[j].y / (1.0 + exp((l[j].x - mean - 0.8) * 2.5))); }
  let u = sum(i, a); if (i == 0u) { let now = finite(0.5 * (mean + u.x / max(u.y, 1e-6)), 30.0) - k.x; let was = finite(e[0], 30.0); e[0] = mix(was, now, select(0.05, 1.0, e[1] != 1.0 || k.w > 0.0)); e[1] = 1.0; e[2] = mean; }
}` },
    { name: 'base', over: BASE, reads: { [small]: 'src' }, writes: { outT: n('0') }, buffers: E, code: CURVE + `
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) { let d = vec2f(textureDimensions(outT)); if (any(vec2f(id.xy) >= d)) { return; } var m = vec2f(0.0); for (var y = 0; y < 3; y++) { for (var x = 0; x < 3; x++) { let l = log2(clamp(strong(textureSampleLevel(src, linSamp, (vec2f(id.xy) + (vec2f(f32(x), f32(y)) + 0.5) / 3.0) / d, 0.0).xyz), 1e-5, 1e4)) - e[0]; m += vec2f(l, l * l); } } textureStore(outT, id.xy, vec4f(m / 9.0, 0.0, 1.0)); }` },
    box(n('0'), n('1'), 1, 0, ''), box(n('1'), n('2'), 0, 1, 'let v = max(m.y - m.x * m.x, 0.0); let a = v / (v + 0.5); m = vec4f(a, m.x * (1.0 - a), 0.0, 1.0);'), box(n('2'), n('3'), 1, 0, ''), box(n('3'), n(''), 0, 1, ''),
  ];
}
/** stops added in all, snap */
export const exposeKnobs = (lab: Lab, bias: number, snap: boolean) => { const at = lab.eyeAt(), uv = at && lab.project(at); lab.uniform('exposeK', new Float32Array([bias, uv ? Math.max(0.02, Math.min(0.98, uv[0])) : 0, uv ? Math.max(0.02, Math.min(0.98, uv[1])) : 0, snap ? 1 : 0])); };
