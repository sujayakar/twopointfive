var<workgroup> todo: array<u32, 256>; var<workgroup> many: atomic<u32>;
fn itsTurn(i: u32) -> bool { let b = texels[2u * i + 1u]; if (bitcast<u32>(b.w) == 0u) { return false; } let w = texels[2u * i].w; if (w <= 24.0) { return true; } let wz = bitcast<u32>(b.z); let slot = faces[3u * PIECES + bitcast<u32>(b.y)].x; if (steady(texelPoint(slot, (wz >> 24u) & 7u, texelUv(i))) <= 0.9) { return true; } let m = select(TURN / 2u, TURN, w > 60.0); return (pcg(i) & m) == (frame.counts.z & m); }
@compute @workgroup_size(64) fn main(@builtin(workgroup_id) g: vec3u, @builtin(local_invocation_index) li: u32) {
  if (li == 0u) { atomicStore(&many, 0u); } workgroupBarrier();
  for (var k = 0u; k < 4u; k++) { let i = (g.x + g.y * 32768u) * 256u + k * 64u + li; if (i < MANY && itsTurn(i)) { todo[atomicAdd(&many, 1u)] = i; } }
  workgroupBarrier(); let n = atomicLoad(&many); for (var k = li; k < n; k += 64u) { oneMore(todo[k]); }
}
fn oneMore(i: u32) {
  var a = texels[2u * i]; let b = texels[2u * i + 1u]; let wz = bitcast<u32>(b.z); let face = (wz >> 24u) & 7u; let slot = faces[3u * PIECES + bitcast<u32>(b.y)].x; let x0 = texelPoint(slot, face, texelUv(i)); let n = texelNormal(slot, face); let st = steady(x0); var seen = b.x;
  byReach = true; if (changes.all.y > 0.5) { a.w = min(a.w, 1.0); } if (bad4(a) || bad(seen)) { a = vec4f(0.0); seen = 0.0; } rngKey(i, 91u); leafMode = 2u; footSolid(0.32, 1.0); let bs = basis(n); let x = x0 + n * 0.012; var sun = 0.0; var E = vec3f(0.0);
  if (DIRECT) { let j = rnd2() - 0.5; let q = askDirect(x0 + (bs[0] * j.x + bs[1] * j.y) * 0.03, n, true); E = q.xyz; sun = q.w; }
  let ru = rnd2(); let w = wideDir(x, n, ru, rnd()); if (w.w > 0.0) { E += min(look(x, w.xyz, 1.0) * w.w, vec3f(60.0)); }
  let cap = mix(min(7.0, KEEP), KEEP, st); let m = min(a.w + 1.0, cap); let k = min(1.0, 2.0 / (m + 1.0)); let o = finite4(vec4f(mix(a.xyz, E, k), m), 1e4); fresh[i] = vec4u(pack2x16float(clamp(o.xy, vec2f(0.0), vec2f(6e4))), pack2x16float(vec2f(clamp(o.z, 0.0, 6e4), clamp(finite(mix(seen, sun, k), 1.0), 0.0, 1.0))), bitcast<u32>(o.w), 1u);
}
