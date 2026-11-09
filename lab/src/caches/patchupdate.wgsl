// one more sample an entry. Each group looks over 256 slots, lists those whose turn it is, then works through the list
fn normOf(s: u32) -> u32 { ROAMS{return bitcast<u32>(spot[s].w);}ELSE{return norms[s];}END }
var<workgroup> todo: array<u32, 256>; var<workgroup> many: atomic<u32>;
fn itsTurn(s: u32) -> bool {
  if (normOf(s) == 0u) { return false; } let x0 = cells[2u * s + 1u].xyz; if (all(x0 == vec3f(0.0))) { return false; }
  let w = cells[2u * s].w; if (w > 24.0 && steady(x0) > 0.9) { let m = select(TURN / 2u, TURN, w > 60.0); return (atomicLoad(&keys[s]) & m) == (frame.counts.z & m); } return true;
}
@compute @workgroup_size(64) fn main(@builtin(workgroup_id) g: vec3u, @builtin(local_invocation_index) li: u32) {
  if (li == 0u) { atomicStore(&many, 0u); } workgroupBarrier();
  for (var i = 0u; i < 4u; i++) { let s = g.x * 256u + i * 64u + li; if (s < SLOTS && itsTurn(s)) { todo[atomicAdd(&many, 1u)] = s; } }
  workgroupBarrier(); let n = atomicLoad(&many); for (var i = li; i < n; i += 64u) { oneMore(todo[i]); }
}
fn oneMore(s: u32) {
  var a = cells[2u * s]; var b = cells[2u * s + 1u]; var x0 = b.xyz; let st = steady(x0); let key = atomicLoad(&keys[s]); byReach = true; if (changes.all.y > 0.5) { a.w = min(a.w, 1.0); } if (bad4(a) || bad(b.w)) { a = vec4f(0.0); b.w = 0.0; }
  rngKey(key, 91u); let np = normOf(s); ROAMS{x0 = spot[s].xyz;}ELSE{}END let every = np == EVERY_WAY; let n = select(unOct(np), vec3f(0.0), every); leafMode = 2u; footSolid(0.32, 1.0);
  let bs = basis(select(n, vec3f(0.0, 1.0, 0.0), every)); let x = x0 + n * 0.012; var sun = 0.0; var E = vec3f(0.0);
  if (DIRECT) { let j = rnd2() - 0.5; let q = askDirect(x0 + (bs[0] * j.x + bs[1] * j.y) * 0.03, n, true); E = q.xyz; sun = q.w; }
  let ru = rnd2(); let rp = rnd(); var w = vec4f(0.0); if (every) { let z = ru.x * 2.0 - 1.0; let r = sqrt(max(1.0 - z * z, 0.0)); w = vec4f(r * cos(6.2832 * ru.y), z, r * sin(6.2832 * ru.y), PI); } else { w = wideDir(x, n, ru, rp); }
  if (w.w > 0.0) { lookPiece = NOTHING; E += min(look(x, w.xyz, 1.0) * w.w, vec3f(MOST)); SPREADS }
  let cap = mix(min(7.0, KEEP), KEEP, st); let m = min(a.w + 1.0, cap); let k = min(1.0, 2.0 / (m + 1.0));
  let o = finite4(vec4f(mix(a.xyz, E, k), m), 1e4); fresh[2u * s] = vec4f(max(o.rgb, vec3f(0.0)), o.w); fresh[2u * s + 1u] = vec4f(b.xyz, clamp(finite(mix(b.w, sun, k), 1.0), 0.0, 1.0));
}
