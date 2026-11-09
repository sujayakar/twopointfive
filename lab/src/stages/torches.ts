// WHAT A TORCH REACHES: how far its light goes, every way in its cone, found anew each frame. The air in the beam is then lit along the eye's own ray, exactly, with no grid and no history.
import type { Lab } from '../lab';
import type { Stage } from '../graph';
export const TORCH = { n: 128, most: 8 };
const { n: N, most: MOST } = TORCH;
export const torchCode = `
fn torchWide(l: u32) -> f32 { let c = lamps[l].cone; return sqrt(max(1.0 - c * c, 0.0)) / max(c, 0.2) * 1.04; }
/** how much of torch k (lamp l) gets to x, v from the lamp */
fn torchSeen(k: u32, l: u32, v: vec3f) -> f32 {
  let n = lamps[l].n; let b = basis(n); let z = dot(v, n); if (z <= 1e-3) { return 0.0; } let g = (vec2f(dot(v, b[0]), dot(v, b[1])) / (z * torchWide(l)) * 0.5 + 0.5) * ${N}.0 - 0.5; let c = floor(g); let f = g - c; let d = length(v); var s = 0.0;
  for (var j = 0; j < 4; j++) { let o = vec2i(j & 1, j >> 1u); let q = clamp(vec2i(c) + o, vec2i(0), vec2i(${N - 1})); let far = torchMap[k * ${N * N}u + u32(q.x + ${N} * q.y)]; s += select(1.0 - f.x, f.x, o.x == 1) * select(1.0 - f.y, f.y, o.y == 1) * (1.0 - smoothstep(0.0, 0.14, d - far - 0.03)); } return s;
}
/** where a ray is in the cone, from and to (from > to: nowhere) */
fn torchSpan(l: u32, ro: vec3f, rd: vec3f, t1: f32, R: f32) -> vec2f {
  let n = lamps[l].n; let w = ro - lamps[l].at; let dw = dot(rd, w); let ww = dot(w, w); let e = dw * dw - ww + R * R; if (e <= 0.0) { return vec2f(1.0, 0.0); } let se = sqrt(e); var a = max(0.0, -dw - se); var b = min(t1, -dw + se); let c2 = lamps[l].cone * lamps[l].cone; let dn = dot(rd, n); let wn = dot(w, n);
  let A = dn * dn - c2; let B = wn * dn - c2 * dw; let C = wn * wn - c2 * ww; let disc = B * B - A * C;
  if (abs(A) < 1e-6) { return vec2f(1.0, 0.0); } if (disc <= 0.0) { if (A < 0.0 || wn + a * dn <= 0.0) { return vec2f(1.0, 0.0); } return vec2f(a, b); }
  let s = sqrt(disc); let r0 = min((-B - s) / A, (-B + s) / A); let r1 = max((-B - s) / A, (-B + s) / A);
  if (A < 0.0) { a = max(a, r0); b = min(b, r1); if (wn + 0.5 * (a + b) * dn <= 0.0) { return vec2f(1.0, 0.0); } } else if (dn > 0.0) { a = max(a, r1); } else { b = min(b, r0); }
  return vec2f(a, b);
}
`;
export function torchStages(lab: Lab): Stage[] {
  lab.buffer('torchMap', N * N * MOST * 4); lab.uniform('torchK', new Uint32Array(12)); let count = 0;
  return [{ name: 'torch reach', over: { groups: () => [N / 8, N / 8, Math.max(1, count)] }, when: () => { count = pick(lab); return count > 0; }, buffers: { torchMap: ['torchMap', 'array<f32>', 'read_write'], torchK: ['torchK', 'array<vec4u, 3>', 'uniform'] }, code: torchCode.slice(0, torchCode.indexOf('/** how much')) + `
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) { let k = id.z; if (k >= torchK[0].x) { return; } let l = torchK[1u + (k >> 2u)][k & 3u]; let n = lamps[l].n; let b = basis(n); let p = ((vec2f(id.xy) + 0.5) / ${N}.0 * 2.0 - 1.0) * torchWide(l); let d = normalize(n + b[0] * p.x + b[1] * p.y); leafMode = 2u; let h = nearest(lamps[l].at, d, 0.08, 24.0, 0u, 1e9); torchMap[k * ${N * N}u + id.x + ${N}u * id.y] = min(h.t, 24.0); }` }];
}
/** the torches that are on, nearest what is looked at first */
function pick(lab: Lab): number {
  const at = lab.camera.look, on = lab.building.world.lamps.map((l, i) => ({ l, i })).filter(({ l, i }) => i < 64 && l.kind === 'spot' && lab.scene.lampOn[i]! > 0 && l.at[1] > -20).map(q => ({ i: q.i, d: Math.hypot(q.l.at[0] - at[0], q.l.at[2] - at[2]) })).sort((a, b) => a.d - b.d || a.i - b.i).slice(0, MOST); const u = new Uint32Array(12); u[0] = on.length; on.forEach((q, k) => { u[4 + k] = q.i; }); lab.uniform('torchK', u); return on.length;
}
