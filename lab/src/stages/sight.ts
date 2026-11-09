// WHAT THE CHARACTER CAN SEE. How far it is from the eye to what stands, every way round (an octahedral map, found anew each frame); a place is seen if it is no farther than that, or if somebody
// standing there would be. What is not seen is drawn as remembered: dim and grey, and nobody in it.
import type { Lab } from '../lab';
import type { Stage } from '../graph';
export const SIGHT = 192;
export const sightCode = `
fn sightAt(x: vec3f) -> f32 {
  let v = x - sightK[0].xyz; let l = length(v); if (l < 0.4) { return 1.0; } let p = octEnc(v / l) * ${SIGHT}.0 - 0.5; let b = floor(p); let f = p - b; var s = 0.0; for (var k = 0; k < 4; k++) { let q = clamp(vec2i(b) + vec2i(k & 1, k >> 1u), vec2i(0), vec2i(${SIGHT - 1})); let far = sight[u32(q.x + ${SIGHT} * q.y)]; s += select(1.0 - f.x, f.x, (k & 1) == 1) * select(1.0 - f.y, f.y, k > 1) * smoothstep(0.0, 1.0, (far + 0.10 + 0.03 * l - l) * 6.0 + 1.0); } return s;
}
/** 0 unseen to 1 seen */
fn seenBy(x: vec3f, n: vec3f) -> f32 {
  if (sightK[0].w < 0.5) { return 1.0; } let o = x + n * 0.06; let v = o - sightK[0].xyz; let d = length(v.xz); let c = dot(normalize(vec3f(v.x, 0.0, v.z) + vec3f(1e-5)), sightK[1].xyz); let afar = smoothstep(30.0, 42.0, d); let front = max(smoothstep(-0.35, 0.05, c), 1.0 - smoothstep(2.0, 3.6, d)); var s = sightAt(o); if (n.y > 0.5) { s = max(s, sightAt(o + vec3f(0.0, 0.9, 0.0))); } return max(s * front * sightK[0].w, afar);
}
fn remembered(c: vec3f, s: f32) -> vec3f { let g = lum(c); return mix(mix(vec3f(g), c, 0.35) * vec3f(0.30, 0.33, 0.40), c, s); }
`;
export function sightStages(lab: Lab): Stage[] {
  lab.buffer('sight', SIGHT * SIGHT * 4); lab.uniform('sightK', new Float32Array(8));
  return [{ name: 'sight', over: { groups: () => [SIGHT / 8, SIGHT / 8, 1] }, when: () => lab.playing, buffers: { sight: ['sight', 'array<f32>', 'read_write'], sightK: ['sightK', 'array<vec4f, 2>', 'uniform'] }, code: `@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) { let d = octDec((vec2f(id.xy) + 0.5) / ${SIGHT}.0); leafMode = 2u; let h = nearest(sightK[0].xyz, d, 0.05, 60.0, MOVES & 0u, 1e9); var t = min(h.t, 60.0); if (h.piece != NOTHING && (pieces[h.piece].flags & MOVES) != 0u && h.t < 0.6) { let g = nearest(sightK[0].xyz, d, 0.05, 60.0, MOVES, 1e9); t = min(g.t, 60.0); } sight[id.x + ${SIGHT}u * id.y] = t; }` }];
}
