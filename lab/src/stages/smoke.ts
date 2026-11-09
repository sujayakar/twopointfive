// Smoke, as stages: page in and out, shut faces, move, push, balance, light, hand over to rays.
import type { Lab } from '../lab';
import type { Graph, Stage } from '../graph';
import type { V3 } from '../math';
import head from './smoke.wgsl' with { type: 'text' };
import signals from '../wgsl/signals.wgsl' with { type: 'text' };
export const SMOKE = { slots: 3072, pages: [75, 24, 60] as const, pairs: 6, atlas: [160, 160, 120] as const };
const CELLS = SMOKE.slots * 512, PG = 108000, WORDS = PG + 36 * 3072 + 16;
const ONE = { groups: (): [number, number, number] => [SMOKE.slots / 64, 1, 1] }, BRICK = { groups: (): [number, number, number] => [64, brickRows, 1] };
/** rows of 64 bricks to ask for: in a run all of them; in the page as many as were lately live, with room to grow (an idle group is not free) */
let brickRows = SMOKE.slots / 64;
/** a group a live brick: s its slot, b its brick */
const perBrick = (body: string, n = 256, extra = '') => `${extra}
@compute @workgroup_size(${n}) fn main(@builtin(workgroup_id) wg: vec3u, @builtin(local_invocation_index) li: u32) {
  let at = wg.x + wg.y * 64u; let live = at < M[M_N + 1u]; let s = select(0u, M[M_LIVE + at], live); let b = brickOf(select(0u, M[M_BRICK + s] - 1u, live)); ${body}
}`;
const FETCH = `
/** where a cell near brick s lies (c from −8 to 15), or none */
fn cell(s: u32, c: vec3i) -> u32 { let b = ((c + 8) >> vec3u(3u)) - 1; if (all(b == vec3i(0))) { return s * 512u + local(c); } let n = M[M_NBR + s * 27u + u32(b.x + 1 + 3 * (b.y + 1) + 9 * (b.z + 1))]; if (n == NONE) { return NONE; } return n * 512u + local(c & vec3i(7)); }
fn unpackV(v: vec2u) -> vec4f { return vec4f(unpack2x16float(v.x), unpack2x16float(v.y)); }
fn packV(v: vec4f) -> vec2u { let q = finite4(v, 60000.0); return vec2u(pack2x16float(q.xy), pack2x16float(q.zw)); }`;
const SHUT = `
/** is the way from cell i along axis a (s = ±1) shut? */
fn shut(i: u32, a: u32, s: i32) -> bool { return ((W[i] >> (6u + a + select(0u, 3u, s < 0))) & 1u) != 0u; }`;
const GROW = `
fn want(b: vec3i, tick: u32) {
  if (!inPage(b)) { return; } let i = brickIx(b); let p = atomicLoad(&M[M_PAGE + i]); if (p != 0u && p != NONE) { atomicStore(&M[M_STAMP + p - 1u], tick); return; } if (p == NONE) { return; }
  let r = atomicCompareExchangeWeak(&M[M_PAGE + i], 0u, NONE); if (!r.exchanged) { return; }
  let n = atomicAdd(&M[M_N + 2u], 1u); if (n < 2048u) { atomicStore(&M[M_FREE + n], i); } else { atomicStore(&M[M_PAGE + i], 0u); }
}`;
const emitCode = GROW + `
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) { let tick = u32(smokeK.y); if (id.x >= u32(smokeK.w)) { return; } let e = sources[id.x * 2u]; let r = e.w + 0.2; let a = vec3i(floor((e.xyz - r - AT) / (8.0 * H))); let z = vec3i(floor((e.xyz + r - AT) / (8.0 * H))); for (var k = a.z; k <= z.z; k++) { for (var j = a.y; j <= z.y; j++) { for (var i = a.x; i <= z.x; i++) { want(vec3i(i, j, k), tick); } } } }`;
const spreadCode = GROW + `
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) { let s = id.x; let tick = u32(smokeK.y); if (s >= SLOTS_S) { return; } let bi = atomicLoad(&M[M_BRICK + s]); if (bi == 0u) { return; } let d = bitcast<f32>(atomicLoad(&M[M_DENSE + s])); if (d < 0.004) { return; } let b = brickOf(bi - 1u); atomicStore(&M[M_STAMP + s], tick); for (var k = 0; k < 27; k++) { let o = vec3i(k % 3, (k / 3) % 3, k / 9) - 1; if (d < 0.05 && dot(o, o) > 1) { continue; } want(b + o, tick); } }`;
const allocCode = `
var<workgroup> w: array<u32, 2048>; var<workgroup> base: array<u32, 256>;
@compute @workgroup_size(256) fn main(@builtin(local_invocation_index) li: u32) {
  let n = min(M[M_N + 2u], 2048u); for (var k = li; k < 2048u; k += 256u) { w[k] = select(NONE, M[M_FREE + k], k < n); } workgroupBarrier();
  for (var size = 2u; size <= 2048u; size <<= 1u) { for (var step = size >> 1u; step > 0u; step >>= 1u) { for (var k = li; k < 1024u; k += 256u) { let i = 2u * k - (k & (step - 1u)); let j = i + step; let up = (i & size) == 0u; let a = w[i]; let b = w[j]; if ((a > b) == up) { w[i] = b; w[j] = a; } } workgroupBarrier(); } }
  var free = 0u; for (var k = 0u; k < 12u; k++) { if (M[M_BRICK + li * 12u + k] == 0u) { free += 1u; } } base[li] = free; workgroupBarrier();
  var at = 0u; for (var k = 0u; k < li; k++) { at += base[k]; } var all = 0u; for (var k = 0u; k < 256u; k++) { all += base[k]; }
  for (var k = 0u; k < 12u; k++) { let s = li * 12u + k; if (M[M_BRICK + s] != 0u) { continue; } if (at < n) { let i = w[at]; M[M_BRICK + s] = i + 1u; M[M_STAMP + s] = u32(smokeK.y); M[M_DENSE + s] = 0u; M[M_BORN + s] = 1u; M[M_PAGE + i] = s + 1u; } at += 1u; }
  for (var k = all + li; k < n; k += 256u) { M[M_PAGE + w[k]] = 0u; }
}`;
const freeCode = `
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) { let s = id.x; let tick = u32(smokeK.y); if (s >= SLOTS_S) { return; } let bi = atomicLoad(&M[M_BRICK + s]); if (bi == 0u) { return; } if (tick - atomicLoad(&M[M_STAMP + s]) > 16u) { atomicStore(&M[M_PAGE + bi - 1u], 0u); atomicStore(&M[M_BRICK + s], 0u); return; } let n = atomicAdd(&M[M_N + 1u], 1u); atomicStore(&M[M_LIVE + n], s); }`;
const linkCode = `
@compute @workgroup_size(64) fn main(@builtin(workgroup_id) wg: vec3u, @builtin(local_invocation_index) li: u32) { let at = wg.x + wg.y * 64u; if (at >= M[M_N + 1u] || li >= 27u) { return; } let s = M[M_LIVE + at]; let b = brickOf(M[M_BRICK + s] - 1u); let q = b + vec3i(i32(li) % 3, (i32(li) / 3) % 3, i32(li) / 9) - 1; var n = NONE; if (inPage(q)) { let p = M[M_PAGE + brickIx(q)]; if (p != 0u && p != NONE) { n = p - 1u; } } M[M_NBR + s * 27u + li] = n; }`;
const facesCode = perBrick(`
  let born = live && M[M_BORN + s] != 0u; if (born && li < 64u) { G[s * 64u + li] = vec2u(0u); } let x0 = mid(b, vec3i(0)) - 0.5 * H; leafMode = 2u;
  for (var k = li; k < 512u; k += 256u) { let c = cellOf(k); let i = s * 512u + k; var f = 0u; if (live) { f = W[i] & 63u; }
    if (born) { f = 0u; let x = mid(b, c); for (var a = 0u; a < 6u; a++) { if (a > 2u && c[a - 3u] != 0) { continue; } var d = vec3f(0.0); d[a % 3u] = select(1.0, -1.0, a > 2u); let h = nearest(x, d, 0.0, H, 0x4000u | WATER, 1e9); if ((h.piece != 0xffffffffu && (h.piece & LEAF_HIT) == 0u) || h.gt < H) { f |= 1u << a; } } let w0 = windAt(x); let wv = vec2u(pack2x16float(w0.xy), pack2x16float(vec2f(w0.z, 0.0))); Va[i] = wv; Vb[i] = wv; Q[i] = 0.0; Q[i + HALF] = 0.0; }
    sh[k] = f; }
  workgroupBarrier();
  for (var k = li; k < 512u; k += 256u) { let c = cellOf(k); let i = s * 512u + k; if (!live) { continue; } var f = sh[k]; if (born) { for (var a = 0u; a < 3u; a++) { if (c[a] > 0) { var q = c; q[a] -= 1; if ((sh[local(q)] & (1u << a)) != 0u) { f |= 8u << a; } } } } var g = f; let x = mid(b, c);
    for (var n = 0u; n < u32(smokeK.z); n++) { let m = doors[n]; let cx = -vec3f(dot(vec3f(m.r0.x, m.r1.x, m.r2.x), vec3f(m.r0.w, m.r1.w, m.r2.w)), dot(vec3f(m.r0.y, m.r1.y, m.r2.y), vec3f(m.r0.w, m.r1.w, m.r2.w)), dot(vec3f(m.r0.z, m.r1.z, m.r2.z), vec3f(m.r0.w, m.r1.w, m.r2.w))); if (distance(cx, x0 + 4.0 * H) > length(m.half.xyz) + 8.0 * H) { continue; } for (var a = 0u; a < 6u; a++) { var e = vec3f(0.0); e[a % 3u] = select(H, -H, a > 2u); if (cuts(m, x, e)) { g |= 1u << a; } } }
    W[i] = f | (g << 6u); }
`, 256, `struct Box { r0: vec4f, r1: vec4f, r2: vec4f, half: vec4f };
var<workgroup> sh: array<u32, 512>; const HALF = SLOTS_S * 512u;
fn cuts(b: Box, a: vec3f, e: vec3f) -> bool { let o = vec3f(dot(b.r0.xyz, a) + b.r0.w, dot(b.r1.xyz, a) + b.r1.w, dot(b.r2.xyz, a) + b.r2.w); let d = vec3f(dot(b.r0.xyz, e), dot(b.r1.xyz, e), dot(b.r2.xyz, e)); let inv = 1.0 / select(d, vec3f(1e-9), abs(d) < vec3f(1e-9)); let h = b.half.xyz + 0.01; let p = (-h - o) * inv; let q = (h - o) * inv; return max(max3(min(p, q)), 0.0) <= min(min3(max(p, q)), 1.0); }`);
const moveCode = FETCH + SHUT + perBrick(`
  if (!live) { return; } let dt = smokeK.x;
  for (var k = li; k < 512u; k += 256u) { let c0 = cellOf(k); let i0 = s * 512u + k; let v = unpackV(Va[i0]).xyz;
    // back along the way it came, a cell at a time, and not through what is shut
    var d = clamp(-v * (dt / H), vec3f(-3.5), vec3f(3.5)); let n = max(1.0, ceil(max3(abs(d)))); let st = d / n; var c = c0; var f = vec3f(0.0); var i = i0;
    for (var q = 0.0; q < n; q += 1.0) { f += st; for (var a = 0u; a < 3u; a++) { let g = select(-1, 1, f[a] > 0.0); if (abs(f[a]) > 0.5) { var t = c; t[a] += g; let j = cell(s, t); if (shut(i, a, g) || j == NONE) { f[a] = f32(g) * 0.5; } else { c = t; i = j; f[a] -= f32(g); } } } }
    var acc = vec4f(0.0); var far = false; var beyond = vec3f(0.0); var ht = 0.0; let sg = vec3i(select(vec3f(-1.0), vec3f(1.0), f > vec3f(0.0))); let w = abs(f); var lo = 1e9; var hi = -1e9;
    for (var q = 0u; q < 8u; q++) { var t = c; var j = i; var wt = 1.0; for (var a = 0u; a < 3u; a++) { let on = ((q >> a) & 1u) == 1u; wt *= select(1.0 - w[a], w[a], on); if (on && j != NONE) { if (!shut(j, a, sg[a])) { t[a] += sg[a]; j = cell(s, t); } } } if (j == NONE) { lo = min(lo, 0.0); if (!far) { far = true; beyond = windAt(mid(b, c0)); } acc += vec4f(beyond, 0.0) * wt; continue; } let e = unpackV(Va[j]); lo = min(lo, e.w); hi = max(hi, e.w); acc += e * wt; ht += Q[j + FROM] * wt; }
    Vb[i0] = packV(acc); Q[i0 + TO] = ht * exp(-dt * (0.2 + 0.42 * max(ht - 2.0, 0.0))); R[i0] = pack2x16float(vec2f(min(lo, 60.0), clamp(hi, 0.0, 60.0))); }
`, 256, `const HALF = SLOTS_S * 512u;`);
/** MacCormack: what the trip there and back again lost is given back by half, within what was found there. Keeps edges that one step would blur */
const sharpCode = FETCH + SHUT + perBrick(`
  if (!live) { return; } let dt = smokeK.x;
  for (var k = li; k < 512u; k += 256u) { let i0 = s * 512u + k; let one = unpackV(Vb[i0]).w; let lim = unpack2x16float(R[i0]); if (lim.y < 0.004) { R[i0] = bitcast<u32>(one); continue; } let v = unpackV(Va[i0]);
    let d = clamp(v.xyz * (dt / H), vec3f(-3.5), vec3f(3.5)); let c0 = cellOf(k); let b0 = vec3i(floor(d)); let f = d - vec3f(b0); var back = 0.0; var ok = true; for (var q = 0u; q < 8u; q++) { let o = vec3i(i32(q & 1u), i32((q >> 1u) & 1u), i32(q >> 2u)); let j = cell(s, c0 + b0 + o); if (j == NONE) { ok = false; continue; } let w = select(1.0 - f.x, f.x, o.x == 1) * select(1.0 - f.y, f.y, o.y == 1) * select(1.0 - f.z, f.z, o.z == 1); back += unpackV(Vb[j]).w * w; }
    let wall = (W[i0] >> 6u) != 0u; R[i0] = bitcast<u32>(select(one, clamp(one + 0.5 * (v.w - back), lim.x, lim.y), ok && !wall)); }
`);
const pushCode = FETCH + SHUT + perBrick(`
  if (!live) { return; } let dt = smokeK.x;
  for (var k = li; k < 512u; k += 256u) { let c = cellOf(k); let i = s * 512u + k; var e = unpackV(Vb[i]); e.w = bitcast<f32>(R[i]); var heat = Q[i + TO]; let x = mid(b, c); var v = e.xyz;
    if (e.w > 0.01) { let w = curl(s, c); let g = vec3f(length(curl(s, c + vec3i(1, 0, 0))) - length(curl(s, c - vec3i(1, 0, 0))), length(curl(s, c + vec3i(0, 1, 0))) - length(curl(s, c - vec3i(0, 1, 0))), length(curl(s, c + vec3i(0, 0, 1))) - length(curl(s, c - vec3i(0, 0, 1)))); let l = length(g); if (l > 1e-4) { v += cross(g / l, w) * (dt * 4.0 * H); } }
    v.y += dt * (heat * 1.6 - e.w * 0.03); let out = select(0.0, 1.0, openAir(x)); /* the brick as a whole is drawn to the coarse air; what rises by heat, rises */ if (M[M_BORN + s] == 0u) { let mean = vec3f(bitcast<f32>(M[M_MEAN + s * 3u]), bitcast<f32>(M[M_MEAN + s * 3u + 1u]), bitcast<f32>(M[M_MEAN + s * 3u + 2u])); v += (windAt(x) - mean) * vec3f(1.0 - exp(-dt * 2.5), (1.0 - sat(heat * 5.0)) * (1.0 - exp(-dt * 0.5)), 1.0 - exp(-dt * 2.5)); }
    // swirls finer than confinement finds, where there is smoke to show them
    if (e.w > 0.02) { let q = x * 2.6 + vec3f(0.0, -frame.misc.x * 0.5, frame.misc.x * 0.2); let g = fract(q); let w3 = g * g * (3.0 - 2.0 * g); var n3 = vec3f(0.0); for (var m = 0u; m < 8u; m++) { let o = vec3f(f32(m & 1u), f32((m >> 1u) & 1u), f32(m >> 2u)); let wt = select(1.0 - w3, w3, o > vec3f(0.5)); n3 += (hash33(floor(q) + o) - 0.5) * (wt.x * wt.y * wt.z); } v += vec3f(n3.z - n3.y, n3.x - n3.z, n3.y - n3.x) * (dt * 2.2 * min(1.0, e.w)); }
    // bodies and leaves of doors take the air with them
    for (var n = 0u; n < u32(smokeK2.x); n++) { let a = pushes[n * 2u]; let f = pushes[n * 2u + 1u]; let d = length((x - a.xyz) * vec3f(1.0, f.w, 1.0)) / a.w; if (d < 1.0) { v = mix(v, f.xyz, (1.0 - d * d) * (1.0 - exp(-dt * 22.0))); } }
    let rim = min(min(x.x - AT.x, AT.x + f32(PN.x) * 8.0 * H - x.x), min(x.z - AT.z, AT.z + f32(PN.z) * 8.0 * H - x.z)); e.w *= exp(-dt * 4.0 * (1.0 - smoothstep(0.3, 3.0, rim)));
    e.w *= exp(-dt * (0.013 + out * 0.22)); v *= exp(-dt * 0.2);
    for (var n = 0u; n < u32(smokeK.w); n++) { let a = sources[n * 2u]; let f = sources[n * 2u + 1u]; let d = x - a.xyz; let r = length(d); let flame = f.x < 0.0; if (r < a.w) { let q = 1.0 - r / a.w; let sp = hash33(vec3f(floor(x * 9.0)) + floor(frame.misc.x * 20.0)) - 0.5; e.w = min(e.w + abs(f.x) * q * dt, 11.0); let lick = select(1.0, smoothstep(0.30, 0.62, vnoise(x.xz * 7.0 + frame.misc.x * 2.3) * 0.6 + vnoise(x.zx * 13.0 - frame.misc.x * 3.1) * 0.4), flame); heat = select(max(heat, min(heat + f.y * q * dt, 1.0)), min(heat + f.y * q * lick * dt, 11.0), flame); v += (normalize(d + vec3f(0.0, 0.1, 0.0)) * 3.0 + sp * 7.0) * (q * dt * f.z); } }
    v *= min(1.0, (3.4 * H / dt) / max(length(v), 1e-4)); e.w += dt * 1.6 * clamp(heat - 1.0, 0.0, 2.2) * step(1.0, heat); if (e.w < 0.002) { e.w = 0.0; } Va[i] = packV(vec4f(v, e.w)); Q[i + TO] = heat; }
`, 256, `const HALF = SLOTS_S * 512u;
fn vel(s: u32, c: vec3i) -> vec3f { let i = cell(s, c); if (i == NONE) { return vec3f(0.0); } return unpackV(Vb[i]).xyz; }
fn curl(s: u32, c: vec3i) -> vec3f { let x = vel(s, c + vec3i(1, 0, 0)) - vel(s, c - vec3i(1, 0, 0)); let y = vel(s, c + vec3i(0, 1, 0)) - vel(s, c - vec3i(0, 1, 0)); let z = vel(s, c + vec3i(0, 0, 1)) - vel(s, c - vec3i(0, 0, 1)); return vec3f(y.z - z.y, z.x - x.z, x.y - y.x) / (2.0 * H); }`);
const FACE = FETCH + SHUT + `
/** the velocity through a cell's face along a, its far side (g = 1) or near (−1) */
var<private> here: vec3f;
fn face(s: u32, c: vec3i, i: u32, a: u32, g: i32) -> f32 { if (shut(i, a, g)) { return 0.0; } var q = c; q[a] += g; let j = cell(s, q); let me = unpackV(Va[i])[a]; if (j == NONE) { return 0.5 * (me + windAt(here)[a]); } return 0.5 * (me + unpackV(Va[j])[a]); }`;
const divCode = FACE + perBrick(`
  if (!live) { return; } for (var k = li; k < 512u; k += 256u) { let c = cellOf(k); let i = s * 512u + k; var d = 0.0; here = mid(b, c); for (var a = 0u; a < 3u; a++) { d += face(s, c, i, a, 1) - face(s, c, i, a, -1); }
    let x = mid(b, c); for (var n = 0u; n < u32(smokeK.w); n++) { let e = sources[n * 2u]; let l = distance(x, e.xyz); if (l < e.w) { d -= sources[n * 2u + 1u].w * H * (1.0 - l / e.w); } }
    P[i + HALF] = d; if (M[M_BORN + s] != 0u) { P[i] = 0.0; } }
`, 256, `const HALF = SLOTS_S * 512u;`);
const solveCode = FETCH + SHUT + perBrick(`
  if (!live) { return; } let k = li; var c = vec3i(0, i32(k >> 2u) & 7, i32(k >> 5u)); c.x = i32(k & 3u) * 2 + ((c.y + c.z + PARITY) & 1); let i = s * 512u + local(c); var t = 0.0; var n = 0.0;
  for (var a = 0u; a < 3u; a++) { for (var g = -1; g < 2; g += 2) { if (shut(i, a, g)) { continue; } n += 1.0; var q = c; q[a] += g; let j = cell(s, q); if (j != NONE) { t += P[j]; } } } if (n > 0.0) { P[i] = mix(P[i], (t - P[i + HALF]) / n, 1.6); }
`, 256, `const HALF = SLOTS_S * 512u;`);
const freedCode = FACE + perBrick(`
  if (!live) { return; } var dense = 0.0; var mv = vec3f(0.0); for (var k = li; k < 512u; k += 256u) { let c = cellOf(k); let i = s * 512u + k; let was = unpackV(Va[i]); var v = was.xyz; let p = P[i];
    for (var a = 0u; a < 3u; a++) { var up = 0.0; var dn = 0.0; let su = shut(i, a, 1); let sd = shut(i, a, -1); var q = c; q[a] += 1; if (!su) { let j = cell(s, q); up = select(0.0, P[j], j != NONE) - p; } q[a] -= 2; if (!sd) { let j = cell(s, q); dn = p - select(0.0, P[j], j != NONE); } if (su && sd) { v[a] = 0.0; } else if (su) { v[a] = min(v[a] - dn, 0.0); } else if (sd) { v[a] = max(v[a] - up, 0.0); } else { v[a] -= 0.5 * (up + dn); } }
    dense = max(dense, was.w); mv += v; Vb[i] = packV(vec4f(v, was.w)); }
  sh[li] = dense; sv[li] = mv; workgroupBarrier(); if (li < 3u && live) { var t = 0.0; for (var k = 0u; k < 256u; k++) { t += sv[k][li]; } D[SLOTS_S + s * 3u + li] = t / 512.0; } if (li == 0u) { var m = 0.0; for (var k = 0u; k < 256u; k++) { m = max(m, sh[k]); } D[s] = m; }
`.replace('if (!live) { return; } var dense', 'var dense').replace('for (var k = li; k < 512u', 'for (var k = select(512u, li, live); k < 512u').replace('if (li == 0u) {', 'if (li == 0u && live) {'), 256, `const HALF = SLOTS_S * 512u; var<workgroup> sh: array<f32, 256>; var<workgroup> sv: array<vec3f, 256>;`);
const doneCode = `@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) { let s = id.x; if (s >= SLOTS_S || M[M_BRICK + s] == 0u) { return; } M[M_DENSE + s] = bitcast<u32>(D[s]); M[M_BORN + s] = 0u; for (var k = 0u; k < 3u; k++) { M[M_MEAN + s * 3u + k] = bitcast<u32>(D[SLOTS_S + s * 3u + k]); } }`;
/** light: a point for every eight cells. The sun, one lamp and one way to look; what it finds is kept and mixed */
const lightCode = FETCH + perBrick(`
  if (!live) { return; } let g = vec3i(i32(li & 3u), i32((li >> 2u) & 3u), i32(li >> 4u)); let gi = s * 64u + li; var d0 = 0.0; for (var k = 0u; k < 8u; k++) { d0 += unpackV(Vb[s * 512u + local(g * 2 + vec3i(i32(k & 1u), i32((k >> 1u) & 1u), i32(k >> 2u)))]).w; } d0 *= 0.125; let was = unpackV(G[gi]); if (d0 < 0.003) { G[gi] = vec2u(0u); G[gi + SLOTS_S * 64u] = vec2u(0u); return; }
  rngKey(brickIx(b) * 64u + li, 61u); leafMode = 2u; byReach = true; let x = mid(b, g * 2) + vec3f(rnd(), rnd(), rnd()) * H; var L = vec3f(0.0); var now = vec3f(0.0); var d = vec3f(0.0); var far = 400.0;
  // (what a torch lends is asked anew each frame and never kept: it moves)
  for (var k = 0u; k < 3u; k++) { var e = vec3f(0.0); if (k == 0u) { if (max3(frame.sunE.xyz) <= 0.0) { continue; } d = coneDir(frame.sunDir.xyz, frame.sunDir.w, rnd2()); e = frame.sunE.xyz; far = 400.0; } else if (k == 1u) { let l = drawLamp(x, vec3f(0.0)); if (l.x < 0.0 || lamps[u32(max(l.x, 0.0))].kind == 2u) { continue; } let q = aimLamp(u32(l.x), x, rnd2()); d = q.dir; far = q.dist - 0.03; e = min(q.E * l.y, vec3f(200.0)); }
    else { var best = 0.0; var who = 0u; for (var m = 0u; m < frame.counts.x; m++) { if (lamps[m].kind != 2u || lamps[m].on <= 0.0) { continue; } let q = aimLamp(m, x, vec2f(0.5)); let w = lum(q.E); if (w > best) { best = w; who = m; } } if (best <= 1e-3) { continue; } let q = aimLamp(who, x, vec2f(0.5)); d = q.dir; far = q.dist - 0.03; e = q.E; }
    if (max3(e) <= 0.0) { continue; } let g0 = e * visible(x, d, 1e-3, far) * (1.0 / (4.0 * PI)); if (k < 2u) { L += g0; } else { now = min(g0, vec3f(200.0)); } }
  G[gi + SLOTS_S * 64u] = packV(vec4f(now, 1.0)); let u = rnd2(); let z = u.x * 2.0 - 1.0; let r = sqrt(max(1.0 - z * z, 0.0)); d = vec3f(r * cos(6.2832 * u.y), z, r * sin(6.2832 * u.y)); let h = nearest(x, d, 1e-3, 80.0, 0u, 1e9); let sm = smokeSeen(x, d, h.t); var B = vec3f(0.0); let fo = folWalk(x, d, h.t, true);
  if (h.piece == 0xffffffffu) { B = skyLod(d, 5.0); } else { let q = surfaceMean(x, d, h); let leaf = (h.piece & LEAF_HIT) != 0u; let own = !leaf && (pieces[h.piece].lamp & 255u) != 255u; B = select(q.glow, vec3f(0.0), own) + sentBack(q, leaf, rnd2(), d); }
  L += min(B * h.thru * fo.T * sm.a + sm.rgb + fo.L, vec3f(40.0)); let m = min(was.w + 1.0, mix(4.0, 24.0, steady(x))); G[gi] = packV(vec4f(mix(was.rgb, min(L, vec3f(200.0)), 1.0 / m), m));
`, 64);
const publishCode = FETCH + `const HALF = SLOTS_S * 512u;` + perBrick(`
  if (!live) { return; } let blk = vec3i(i32(s & 15u), i32((s >> 4u) & 15u), i32(s >> 8u)) * 10;
  for (var k = li; k < 1000u; k += 256u) { let t = vec3i(i32(k % 10u), i32((k / 10u) % 10u), i32(k / 100u)); let c = t - 1; let q = clamp(c, vec3i(0), vec3i(7)); let o = c - q; let i = s * 512u + local(q); var d = 0.0; var open = true; for (var a = 0u; a < 3u; a++) { if (o[a] != 0 && ((W[i] >> (6u + a + select(0u, 3u, o[a] < 0))) & 1u) != 0u) { open = false; } } if (open) { let j = cell(s, c); if (j != NONE) { d = unpackV(Vb[j]).w; } }
    let p = clamp((vec3f(c) + 0.5) * 0.5 - 0.5, vec3f(0.0), vec3f(2.999)); let g0 = vec3i(p); let f = p - vec3f(g0); var L = vec3f(0.0); var ws = 0.0; for (var n = 0u; n < 8u; n++) { let m = vec3i(i32(n & 1u), i32((n >> 1u) & 1u), i32(n >> 2u)); let g = g0 + m; let ge = s * 64u + u32(g.x + 4 * (g.y + 4 * g.z)); var e = unpackV(G[ge]); if (e.w <= 0.0) { continue; } e += vec4f(unpackV(G[ge + SLOTS_S * 64u]).rgb, 0.0); let w = select(1.0 - f.x, f.x, m.x == 1) * select(1.0 - f.y, f.y, m.y == 1) * select(1.0 - f.z, f.z, m.z == 1) + 1e-3; L += e.rgb * w; ws += w; }
    // hot gas glows: what it gives off and what it scatters, by how much there is of each
    var out = vec4f(L / max(ws, 1e-3), d); if (open) { let j = cell(s, c); if (j != NONE) { let h = Q[j + select(0u, HALF, smokeK.z > 0.5)]; if (h > 1.1) { let g = smoothstep(1.1, 4.2, h); let hot = smoothstep(4.2, 8.0, h); let soot = 3.4 * g * (1.0 + 1.5 * hot); let glow = mix(mix(mix(vec3f(0.55, 0.045, 0.004), vec3f(1.0, 0.28, 0.03), smoothstep(0.1, 0.55, g)), vec3f(1.0, 0.56, 0.15), smoothstep(0.6, 1.0, g)), vec3f(1.0, 0.86, 0.58), hot) * (90.0 * g * g * (1.0 + 4.0 * hot)); out = vec4f((out.rgb * d + glow * soot) / (d + soot), d + soot); } } }
    textureStore(atlas, blk + t, out); }
`);
const pageCode = `@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) { let i = id.x; if (i >= PAGES) { return; } var p = M[M_PAGE + i]; if (p == NONE) { p = 0u; } if (p != 0u) { if (bitcast<f32>(M[M_DENSE + p - 1u]) < 0.002) { p = 0u; } } textureStore(page, brickOf(i), vec4u(p)); }`;
export class Smoke {
  on = false; sources: { at: V3; born: number; last: number; rate: number }[] = []; private lastTick = -1; private doorKey = ''; dt = 0; private faceNow = true; private fresh = true; private steps = 0; private flip = 0;
  private counting = false; private lately = SMOKE.slots;
  private count() { if (this.counting || this.steps % 5) return; this.counting = true; const d = this.lab.gpu.device, b = d.createBuffer({ size: 16, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ }), e = d.createCommandEncoder(); e.copyBufferToBuffer(this.lab.graph.buffers.get('smokeM')!, (PG + 32 * SMOKE.slots) * 4, b, 0, 16); d.queue.submit([e.finish()]); void b.mapAsync(GPUMapMode.READ).then(() => { this.lately = new Uint32Array(b.getMappedRange())[1]!; b.destroy(); this.counting = false; }, () => { this.counting = false; }); }
  constructor(private lab: Lab) {}
  /** a canister where it lies: it burns for `last` seconds */
  drop(at: V3, last = 14, rate = 1) { this.on = true; this.sources.push({ at: [at[0], Math.max(at[1], 0.05) + 0.14, at[2]], born: this.lab.tick, last, rate }); if (this.sources.length > 10) this.sources.shift(); }
  /** a breath of it: a shot's, or what it strikes off a wall */
  puff(at: V3, r: number, rate: number) { this.on = true; this.puffs.push({ at: [...at], born: this.lab.tick, r, rate }); if (this.puffs.length > 3) this.puffs.shift(); } private puffs: { at: V3; born: number; r: number; rate: number }[] = [];
  clear() { this.sources = []; this.fires = []; this.fresh = true; }
  /** fires: each feeds the bricks hot gas and lights the room by a lamp that moves as it flickers */
  fires: { at: V3; size: number }[] = [];
  light(at: V3, size = 1) { this.on = true; this.fires.push({ at: [...at], size }); if (this.fires.length > 3) this.fires.shift(); }
  /** its lamps, at a time (the same time, the same flicker) */
  flames(t: number) { const lamps = this.lab.building.world.lamps; let k = 0; lamps.forEach((l, i) => { if (l.name !== 'fire') return; const f = this.fires[k]; k++; if (!f) { l.on = 0; this.lab.scene.lampOn[i] = 0; return; } const n = (a: number, b: number) => Math.sin(t * a + b) * 0.5 + Math.sin(t * a * 2.37 + b * 1.7) * 0.3 + Math.sin(t * a * 5.1 + b * 2.9) * 0.2, q = f.size * (0.82 + 0.18 * n(7.3, i) + 0.10 * n(17.0, i * 2)); l.at = [f.at[0] + 0.07 * n(5.1, i + 1), f.at[1] + 0.45 + 0.12 * n(6.3, i + 2), f.at[2] + 0.07 * n(4.7, i + 3)]; l.on = 1; this.lab.scene.lampOn[i] = Math.max(0.1, q); }); }
  get burning() { return this.sources.some(q => this.lab.tick - q.born < q.last * 60 + 1200); }
  frame() {
    const lab = this.lab, steam = lab.steam > 0 ? lab.building.world.plumes : []; if (steam.length) this.on = true; if (!this.on) { this.dt = 0; return; } const d = Math.max(0, Math.min(3, lab.tick - this.lastTick)); this.dt = this.lastTick < 0 ? 1 / 60 : d / 60; this.lastTick = lab.tick; if (this.dt <= 0 && !this.fresh) return; this.steps++; this.flip ^= 1;
    if (this.fresh) { this.fresh = false; const m = new Uint32Array(WORDS); lab.gpu.device.queue.writeBuffer(lab.graph.buffers.get('smokeM')!, 0, m); this.dt = this.dt || 1 / 60; }
    else lab.gpu.device.queue.writeBuffer(lab.graph.buffers.get('smokeM')!, (PG + 32 * SMOKE.slots + 1) * 4, new Uint32Array([0, 0]));
    if (lab.smooth) { this.count(); const young = this.sources.some(q => lab.tick - q.born < 120); brickRows = Math.min(SMOKE.slots / 64, Math.ceil((this.lately * 1.35 + (young ? 700 : 160)) / 64)); } else brickRows = SMOKE.slots / 64;
    const key = lab.building.doors.map(x => x.open.toFixed(3)).join(); this.faceNow = key !== this.doorKey; this.doorKey = key; void this.faceNow;
    const boxes = lab.building.doors.map(x => x.leaf[0]!), f = new Float32Array(16 * 16); boxes.slice(0, 16).forEach((p, i) => { for (let k = 0; k < 3; k++) { const r = p.rot[k]!; f.set([r[0], r[1], r[2], -(r[0] * p.at[0] + r[1] * p.at[1] + r[2] * p.at[2])], i * 16 + k * 4); } f.set(p.half, i * 16 + 12); }); lab.uniform('smokeDoors', f);
    const s = new Float32Array(16 * 8); let n = 0; for (const q of this.sources) { const age = (lab.tick - q.born) / 60; if (age > q.last || n >= 10) continue; const k = Math.min(1, age / 0.5) * Math.min(1, (q.last - age) / 2); s.set([...q.at, 0.34, 300 * q.rate * k, 1.1 * k, 1.0 * k, 95 * q.rate * k], n++ * 8); }
    this.puffs = this.puffs.filter(q => lab.tick - q.born < 9); for (const q of this.puffs) { if (n >= 13) break; s.set([...q.at, q.r, q.rate, 0.5, 0.6, q.rate * 0.05], n++ * 8); }
    for (const q of this.fires) { if (n >= 13) break; s.set([q.at[0], q.at[1] + 0.10, q.at[2], 0.36 * q.size, -0.5, 95, 1.6, 3.0], n++ * 8); }
    for (const p of steam) { if (n >= 16) break; const k = 0.75 + 0.25 * Math.sin(lab.tick * 0.11 + p.at[0]) * Math.sin(lab.tick * 0.043 + p.at[2]); s.set([p.at[0], p.at[1], p.at[2], p.r, 8 * p.k * k, 5.0, 0.35, 5 * k], n++ * 8); } lab.uniform('smokeSources', s);
    const ps = new Float32Array(256); let m = 0; for (const p of steam) if (m < 8) ps.set([p.at[0], p.at[1] + 0.1, p.at[2], p.r * 1.2, 0, 3.0, 0, 0.5], m++ * 8); for (const q of lab.building.people) for (const [h, rad] of [[0.45, 0.42], [1.25, 0.45]] as const) if (m < 28) ps.set([q.pos[0], q.pos[1] + h, q.pos[2], rad, q.vel[0] * 1.15, 0, q.vel[2] * 1.15, 0.8], m++ * 8); for (const d of lab.building.doors) { const l = d.leaf[0]!, w = lab.prevAt.get(l); if (!w || m >= 32) continue; const v = [0, 1, 2].map(k => (l.at[k]! - w.at[k]!) / this.dt); if (Math.hypot(...v) > 0.05) ps.set([...l.at, 0.75, v[0]!, 0, v[2]!, 0.5], m++ * 8); } lab.uniform('smokePushes', ps); lab.uniform('smokeK2', new Float32Array([m, 0, 0, 0])); lab.uniform('smokeK3', new Float32Array([0, 0, this.flip === 0 ? 1 : 0, 0]));
    lab.uniform('smokeK', new Float32Array([this.dt, this.steps, boxes.length, n]));
  }
  stages(g: Graph, cacheBind: NonNullable<Stage['buffers']>, cacheLib: string, bounceLib: string): Stage[] {
    const lab = this.lab, B = (n: string, bytes: number) => lab.buffer(n, bytes, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST); B('smokeM', WORDS * 4); B('smokeA', CELLS * 8); B('smokeB', CELLS * 8); B('smokeQ', CELLS * 8); B('smokeP', CELLS * 8); B('smokeW', CELLS * 4); B('smokeR', CELLS * 4); B('smokeG', SMOKE.slots * 64 * 16); B('smokeD', SMOKE.slots * 16);
    lab.uniform('smokeK', new Float32Array(4)); lab.uniform('smokeDoors', new Float32Array(256)); lab.uniform('smokeSources', new Float32Array(128)); lab.uniform('smokePushes', new Float32Array(256)); lab.uniform('smokeK2', new Float32Array(4)); lab.uniform('smokeK3', new Float32Array(4)); lab.uniform('changes', new Float32Array(36)); g.bind('smokeAtlas', lab.scene.smokeTex); g.bind('smokePage', lab.scene.smokePage);
    const go = () => this.on && this.dt > 0, K: NonNullable<Stage['buffers']> = { smokeK: ['smokeK', 'vec4f', 'uniform'], sources: ['smokeSources', 'array<vec4f, 32>', 'uniform'] }, rw = (n: string, t: string): [string, string, 'read_write'] => [n, t, 'read_write'], ro = (n: string, t: string): [string, string, 'read'] => [n, t, 'read'];
    const MA = { M: rw('smokeM', 'array<atomic<u32>>') }, MR = { M: ro('smokeM', 'array<u32>') }, MW = { M: rw('smokeM', 'array<u32>') }, V2 = 'array<vec2u>', F = 'array<f32>', U = 'array<u32>';
    const S = (name: string, code: string, buffers: NonNullable<Stage['buffers']>, over: Stage['over'] = BRICK, consts?: Record<string, number>): Stage => ({ name, over, code: head + code, buffers, when: go, ...(consts ? { consts } : {}) });
    const half = SMOKE.slots * 512, twice = (name: string, code: string, buffers: NonNullable<Stage['buffers']>): Stage[] => [0, 1].map(k => ({ ...S(name, code, buffers, BRICK, { FROM: k ? half : 0, TO: k ? 0 : half }), when: () => go() && this.flip === k }));
    return [
      S('smoke page', emitCode, { ...MA, ...K }, { groups: () => [1, 1, 1] }), S('smoke page', spreadCode, { ...MA, smokeK: K.smokeK! }, ONE), S('smoke page', allocCode, { ...MW, smokeK: K.smokeK! }, { groups: () => [1, 1, 1] }), S('smoke page', freeCode, { ...MA, smokeK: K.smokeK! }, ONE), S('smoke page', linkCode, MW),
      S('smoke faces', facesCode, { ...MR, W: rw('smokeW', U), Va: rw('smokeA', V2), Vb: rw('smokeB', V2), Q: rw('smokeQ', F), G: rw('smokeG', V2), smokeK: K.smokeK!, doors: ['smokeDoors', 'array<Box, 16>', 'uniform'] }),
      ...twice('smoke move', moveCode, { ...MR, W: ro('smokeW', U), Va: ro('smokeA', V2), Vb: rw('smokeB', V2), Q: rw('smokeQ', F), R: rw('smokeR', U), smokeK: K.smokeK! }),
      S('smoke move', sharpCode, { ...MR, W: ro('smokeW', U), Va: ro('smokeA', V2), Vb: ro('smokeB', V2), R: rw('smokeR', U), smokeK: K.smokeK! }),
      ...twice('smoke push', pushCode, { ...MR, W: ro('smokeW', U), Vb: ro('smokeB', V2), Va: rw('smokeA', V2), Q: rw('smokeQ', F), ...K, R: ro('smokeR', U), smokeK2: ['smokeK2', 'vec4f', 'uniform'], pushes: ['smokePushes', 'array<vec4f, 64>', 'uniform'] }),
      S('smoke press', divCode, { ...MR, W: ro('smokeW', U), Va: ro('smokeA', V2), P: rw('smokeP', F), ...K }),
      ...Array.from({ length: SMOKE.pairs * 2 }, (_, i) => S('smoke press', solveCode, { ...MR, W: ro('smokeW', U), P: rw('smokeP', F) }, BRICK, { PARITY: i & 1 })),
      S('smoke free', freedCode, { ...MR, W: ro('smokeW', U), Va: ro('smokeA', V2), P: ro('smokeP', F), Vb: rw('smokeB', V2), D: rw('smokeD', F) }),
      S('smoke page', doneCode, { ...MW, D: ro('smokeD', F) }, ONE),
      { ...S('smoke light', '', {}), code: signals + cacheLib + bounceLib + head + lightCode, buffers: { ...cacheBind, Vb: ro('smokeB', V2), G: rw('smokeG', V2), M: ro('smokeM', U), changes: ['changes', 'Changes', 'uniform'] } },
      { ...S('smoke show', publishCode, { ...MR, W: ro('smokeW', U), Vb: ro('smokeB', V2), G: ro('smokeG', V2), Q: ro('smokeQ', F), smokeK: ['smokeK3', 'vec4f', 'uniform'] }), writes: { atlas: 'smokeAtlas' }, alone: true },
      { ...S('smoke show', pageCode, MR, { groups: () => [Math.ceil(PG / 64), 1, 1] }), writes: { page: 'smokePage' }, alone: true },
      { name: 'smoke keep', over: BRICK, code: '', async make() { return enc => { if (go()) enc.copyBufferToBuffer(g.buffers.get('smokeB')!, 0, g.buffers.get('smokeA')!, 0, CELLS * 8); }; } },
    ];
  }
}
