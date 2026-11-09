var<workgroup> todo: array<u32, 256>; var<workgroup> many: atomic<u32>;
fn itsTurn(s: u32) -> bool { let a = surfels[3u * s]; let w = surfels[3u * s + 1u].w; if (w > 24.0 && steady(a.xyz) > 0.9) { let m = select(TURN / 2u, TURN, w > 60.0); return (pcg(s) & m) == (frame.counts.z & m); } return true; }
@compute @workgroup_size(64) fn main(@builtin(workgroup_id) g: vec3u, @builtin(local_invocation_index) li: u32) {
  if (li == 0u) { atomicStore(&many, 0u); } workgroupBarrier(); let all = min(atomicLoad(&buckets[TOTAL]), SURFELS);
  for (var i = 0u; i < 4u; i++) { let s = g.x * 256u + i * 64u + li; if (s < all && itsTurn(s)) { todo[atomicAdd(&many, 1u)] = s; } }
  workgroupBarrier(); let n = atomicLoad(&many); for (var i = li; i < n; i += 64u) { oneMore(todo[i]); }
}
fn oneMore(s: u32) {
  let a = surfels[3u * s]; var e = surfels[3u * s + 1u]; let b = surfels[3u * s + 2u]; let x0 = a.xyz; let n = unOct(bitcast<u32>(b.y)); let st = steady(x0); var seen = b.x; byReach = true; if (changes.all.y > 0.5) { e.w = min(e.w, 1.0); } if (bad4(e) || bad(seen)) { e = vec4f(0.0); seen = 0.0; } rngKey(s, 91u); leafMode = 2u; footSolid(0.32, 1.0); let bs = basis(n); let x = x0 + n * 0.012; var sun = 0.0; var E = vec3f(0.0);
  if (DIRECT) { let j = rnd2() - 0.5; let q = askDirect(x0 + (bs[0] * j.x + bs[1] * j.y) * 0.03, n, true); E = q.xyz; sun = q.w; }
  let ru = rnd2(); let w = wideDir(x, n, ru, rnd()); if (w.w > 0.0) { E += min(look(x, w.xyz, 1.0) * w.w, vec3f(60.0)); }
  let cap = mix(min(7.0, KEEP), KEEP, st); let m = min(e.w + 1.0, cap); let k = min(1.0, 2.0 / (m + 1.0)); let o = finite4(vec4f(mix(e.xyz, E, k), m), 1e4); fresh[s] = vec4u(pack2x16float(clamp(o.xy, vec2f(0.0), vec2f(6e4))), pack2x16float(vec2f(clamp(o.z, 0.0, 6e4), clamp(finite(mix(seen, sun, k), 1.0), 0.0, 1.0))), bitcast<u32>(o.w), 1u);
}
