// A TINY NETWORK OVER A HASH GRID, trained as it draws. A place is looked up in LEVELS grids of learned numbers (hashed corners, mixed by nearness); those and its normal go through two layers of 32; what comes out
// is the logarithm of the light that lands there after a bounce. Each frame some of the places rays just landed on are asked by one ray of their own, and the network is moved toward the answer (Adam).
// Text only: the two layers are written out line by line, so that all they work on stays in registers.
const IN = 6, HID = 8; // in fours
export const NET = { w1: 0, b1: IN * HID * 4, w2: IN * HID * 4 + HID, b2: IN * HID * 4 + HID + HID * HID * 4, w3: IN * HID * 4 + 2 * HID + HID * HID * 4, b3: IN * HID * 4 + 2 * HID + HID * HID * 4 + HID * 4, table: 512 };
const layer = (arr: string, out: string, inp: string, K: number, O: number, w: number, b: number, relu: boolean) => Array.from({ length: O }, (_, o) => `  var ${out}${o} = ${arr}[${b + o}u];\n` + Array.from({ length: K }, (_, i) => { const a = w + (o * K + i) * 4; return `  ${out}${o} += ${arr}[${a}u] * ${inp}${i}.x + ${arr}[${a + 1}u] * ${inp}${i}.y + ${arr}[${a + 2}u] * ${inp}${i}.z + ${arr}[${a + 3}u] * ${inp}${i}.w;\n`; }).join('') + (relu ? `  ${out}${o} = max(${out}${o}, vec4f(0.0));\n` : '')).join('');
const back = (arr: string, out: string, del: string, K: number, O: number, w: number, gate?: string) => Array.from({ length: K }, (_, i) => `  var ${out}${i} = vec4f(0.0);\n` + Array.from({ length: O }, (_, o) => { const a = w + (o * K + i) * 4; return `  ${out}${i} += vec4f(dot(${arr}[${a}u], ${del}${o}), dot(${arr}[${a + 1}u], ${del}${o}), dot(${arr}[${a + 2}u], ${del}${o}), dot(${arr}[${a + 3}u], ${del}${o}));\n`; }).join('') + (gate ? `  ${out}${i} = select(vec4f(0.0), ${out}${i}, ${gate}${i} > vec4f(0.0));\n` : '')).join('');
const enc = (arr: string, levels: number) => Array.from({ length: 5 }, (_, l) => l < levels ? `  var x${l} = vec4f(0.0); { let g = x / cellAt(${l}u); let b = floor(g); let f = g - b; for (var k = 0u; k < 8u; k++) { let o = vec3f(f32(k & 1u), f32((k >> 1u) & 1u), f32(k >> 2u)); let t = mix(1.0 - f, f, o); x${l} += ${arr}[cornerAt(vec3i(b + o), ${l}u)] * (t.x * t.y * t.z); } }\n` : `  var x${l} = vec4f(0.0);\n`).join('') + '  let x5 = vec4f(n, 1.0);\n';
export const netHead = `const CELL = 0.32;
fn cellAt(l: u32) -> f32 { return FINEST * exp2(f32(NLEV - 1u - l)); }
fn cornerAt(c: vec3i, l: u32) -> u32 { return ${NET.table}u + l * TABLE + ((u32(c.x) * 1u) ^ (u32(c.y) * 2654435761u) ^ (u32(c.z) * 805459861u) ^ (l * 97u)) % TABLE; }
`;
export const netRead = (levels: number) => netHead + `
fn guess(at: vec3f, n: vec3f) -> vec4f { let x = at + n * 0.05;
${enc('net', levels)}${layer('net', 'h', 'x', IN, HID, NET.w1, NET.b1, true)}${layer('net', 'k', 'h', HID, HID, NET.w2, NET.b2, true)}${layer('net', 'o', 'k', HID, 1, NET.w3, NET.b3, false)}  return vec4f(exp(clamp(o0.xyz, vec3f(-12.0), vec3f(6.0))), 1.0 / (1.0 + exp(-clamp(o0.w, -12.0, 12.0))));
}
fn held(x: vec3f, n: vec3f, j: vec2f) -> Cached { var c: Cached; c.n = 10.0; let g = guess(x, n); c.E = g.xyz; if (DIRECT) { c.sun = g.w; } return c; }
fn heldSmooth(x: vec3f, n: vec3f) -> Cached { return held(x, n, vec2f(0.5)); }
`;
const store = (arr: string, v: string, k: number) => Array.from({ length: k }, (_, i) => `  ${arr}[li * 8u + ${i}u] = vec2u(pack2x16float(${v}${i}.xy), pack2x16float(${v}${i}.zw));\n`).join('');
const sum = (K: number, O: number, w: number, b: number) => `  workgroupBarrier(); if (li < ${K * O}u) { let o = li / ${K}u; let i = li % ${K}u; var g0 = vec4f(0.0); var g1 = vec4f(0.0); var g2 = vec4f(0.0); var g3 = vec4f(0.0); for (var s = 0u; s < 64u; s++) { let d = un4(sd[s * 8u + o]); let x = un4(sa[s * 8u + i]); g0 += d * x.x; g1 += d * x.y; g2 += d * x.z; g3 += d * x.w; } let a = ${w}u + li * 4u; add4(a, g0); add4(a + 1u, g1); add4(a + 2u, g2); add4(a + 3u, g3); }
  if (li >= 56u && li < 56u + ${O}u) { let o = li - 56u; var g = vec4f(0.0); for (var s = 0u; s < 64u; s++) { g += un4(sd[s * 8u + o]); } add4(${b}u + o, g); }
  workgroupBarrier();\n`;
export const netTrain = (levels: number) => `
var<workgroup> sa: array<vec2u, 512>; var<workgroup> sd: array<vec2u, 512>;
fn un4(v: vec2u) -> vec4f { return vec4f(unpack2x16float(v.x), unpack2x16float(v.y)); }
fn add4(a: u32, g: vec4f) { for (var k = 0u; k < 4u; k++) { let q = i32(clamp(g[k], -6e4, 6e4) * 4096.0); if (q != 0) { atomicAdd(&grads[a * 4u + k], q); } } }
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u, @builtin(local_invocation_index) li: u32) {
  let i = id.x; rngKey(i, 911u); byReach = true; leafMode = 2u; footSolid(0.32, 1.0);
  let mine = rnd() < 0.35; let who = select(u32(rnd() * netK.x), HITS + u32(rnd() * f32(BATCH)), mine); let h = hits[min(who, HITS + BATCH - 1u)]; var x = bitcast<vec3f>(h.xyz); var n = octUnpack(h.w); let ok = h.w != 0u && !bad4(vec4f(x, 0.0)) && dot(x, x) < 1e6; if (!ok) { x = vec3f(0.0); n = vec3f(0.0, 1.0, 0.0); }
  var t = vec3f(0.0); var seen = 0.0; if (ok) { if (DIRECT) { let q = askDirect(x, n, true); t = q.xyz; seen = q.w; } let p = x + n * 0.012; let w = wideDir(p, n, rnd2(), rnd()); if (w.w > 0.0) { lookPiece = NOTHING; t += min(look(p, w.xyz, 1.0) * w.w, vec3f(60.0)); if (lookPiece != NOTHING && (lookPiece & LEAF_HIT) == 0u) { hits[HITS + i] = vec4u(bitcast<vec3u>(lookX), octPack(lookN) | 1u); } } }
  t = finite4(vec4f(t, 0.0), 1e3).xyz; x += n * 0.05;
${enc('master', levels)}${layer('master', 'h', 'x', IN, HID, NET.w1, NET.b1, true)}${layer('master', 'k', 'h', HID, HID, NET.w2, NET.b2, true)}${layer('master', 'o', 'k', HID, 1, NET.w3, NET.b3, false)}
  let E = exp(clamp(o0.xyz, vec3f(-12.0), vec3f(6.0))); let m = lum(E); let g = clamp(2.0 * (E - t) * E / (m * m + 1e-5), vec3f(-2000.0), vec3f(2000.0)); let sg = 1.0 / (1.0 + exp(-clamp(o0.w, -12.0, 12.0))); let e0 = select(vec4f(0.0), vec4f(g, select(0.0, 2.0 * (sg - seen) * sg * (1.0 - sg), DIRECT)), ok);
${back('master', 'f', 'e', HID, 1, NET.w3, 'k')}${back('master', 'q', 'f', HID, HID, NET.w2, 'h')}${back('master', 'z', 'q', 5, HID, NET.w1).replaceAll(/master\[(\d+)u\]/g, (s, a: string) => { const r = +a - NET.w1, o = Math.floor(r / 20), i = Math.floor((r % 20) / 4), c = r % 4; return `master[${NET.w1 + (o * IN + i) * 4 + c}u]`; })}
  if (ok) {
${Array.from({ length: levels }, (_, l) => `    { let g = x / cellAt(${l}u); let b = floor(g); let f = g - b; for (var k = 0u; k < 8u; k++) { let o = vec3f(f32(k & 1u), f32((k >> 1u) & 1u), f32(k >> 2u)); let t = mix(1.0 - f, f, o); add4(cornerAt(vec3i(b + o), ${l}u), z${l} * (t.x * t.y * t.z)); } }`).join('\n')}
  }
${store('sa', 'k', 8)}${store('sd', 'e', 1)}${sum(8, 1, NET.w3, NET.b3)}${store('sa', 'h', 8)}${store('sd', 'f', 8)}${sum(8, 8, NET.w2, NET.b2)}${store('sa', 'x', 6)}${store('sd', 'q', 8)}${sum(6, 8, NET.w1, NET.b1)}
}
`;
export const netStep = `
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) {
  let i = id.x + id.y * 2097152u; if (i >= PARAMS) { return; } var g = vec4f(0.0); var any = false; for (var k = 0u; k < 4u; k++) { let q = atomicExchange(&grads[i * 4u + k], 0); any = any || q != 0; g[k] = f32(q) / (4096.0 * f32(BATCH)); }
  if (i < ${NET.table}u || any) { var m = mv[2u * i]; var v = mv[2u * i + 1u]; m = mix(g, m, 0.9); v = mix(g * g, v, 0.99); mv[2u * i] = m; mv[2u * i + 1u] = v; let b = 1.0 - pow(0.9, min(netK.y, 200.0)); let c = 1.0 - pow(0.99, min(netK.y, 2000.0)); let rate = RATE * select(1.0, 0.25, i < ${NET.table}u); master[i] = clamp(master[i] - rate * (m / b) / (sqrt(v / c) + 1e-6), vec4f(-60.0), vec4f(60.0)); }
  net[i] = mix(master[i], net[i], SMOOTH);
}
`;
