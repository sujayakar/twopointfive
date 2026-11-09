var<workgroup> todo: array<u32, 256>; var<workgroup> many: atomic<u32>;
fn itsTurn(s: u32) -> bool { if (atomicLoad(&keys[s]) == 0u) { return false; } let x = pointOf(s); if (all(x == vec3f(0.0))) { return false; } if (cells[4u * s].w > 24.0 && steady(x) > 0.9) { return (atomicLoad(&keys[s]) & TURN) == (frame.counts.z & TURN); } return true; }
@compute @workgroup_size(64) fn main(@builtin(workgroup_id) g: vec3u, @builtin(local_invocation_index) li: u32) {
  if (li == 0u) { atomicStore(&many, 0u); } workgroupBarrier();
  for (var i = 0u; i < 4u; i++) { let s = g.x * 256u + i * 64u + li; if (s < SLOTS && itsTurn(s)) { todo[atomicAdd(&many, 1u)] = s; } }
  workgroupBarrier(); let n = atomicLoad(&many); for (var i = li; i < n; i += 64u) { oneMore(todo[i]); }
}
fn oneMore(s: u32) {
  var a = array<vec4f, 4>(cells[4u * s], cells[4u * s + 1u], cells[4u * s + 2u], cells[4u * s + 3u]); let x = pointOf(s); let st = steady(x); byReach = true; leafMode = 2u; footSolid(0.32, 1.0); rngKey(atomicLoad(&keys[s]), 91u); var n = a[0].w; if (changes.all.y > 0.5) { n = min(n, 1.0); } for (var i = 0u; i < 4u; i++) { if (bad4(a[i])) { a[i] = vec4f(0.0); n = 0.0; } }
  var v: array<vec3f, 4>; let z0 = rnd(); let turn = rnd() * 6.2831853;
  for (var i = 0u; i < RAYS; i++) { let z = 1.0 - 2.0 * (f32(i) + rnd()) / f32(RAYS); let r = sqrt(max(1.0 - z * z, 0.0)); let an = turn + 2.3999632 * f32(i); var d = vec3f(r * cos(an), z, r * sin(an)); if (z0 < 0.333) { d = d.yzx; } else if (z0 < 0.667) { d = d.zxy; } let L = min(look(x, d, 1.0), vec3f(60.0)) / f32(RAYS); v[0] += L; v[1] += L * (3.0 * d.x); v[2] += L * (3.0 * d.y); v[3] += L * (3.0 * d.z); SPREADS }
  let cap = mix(min(7.0, KEEP), KEEP, st); let m = min(n + 1.0, cap); let k = min(1.0, 2.0 / (m + 1.0));
  for (var i = 0u; i < 4u; i++) { fresh[4u * s + i] = finite4(vec4f(mix(a[i].xyz, v[i], k), select(x[max(i, 1u) - 1u], m, i == 0u)), 1e4); }
}
