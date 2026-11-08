/** the one way light gets into an entry */
fn cachePut(s: u32, E: vec3f, n: f32, x: vec3f, sun: f32) { let a = finite4(vec4f(E, n), 1e4); fresh[2u * s] = vec4f(max(a.rgb, vec3f(0.0)), a.w); fresh[2u * s + 1u] = vec4f(x, clamp(finite(sun, 1.0), 0.0, 1.0)); }
// LIGHT ON SURFACES, one more sample an entry: the sun, one lamp, and one ray to see what else is out there, read from the table as it stands.
//   Each group looks over 256 slots, lists the few whose turn it is, then works through the list: rays of neighbours in the list keep all its lanes busy.
var<workgroup> todo: array<u32, 256>; var<workgroup> many: atomic<u32>;
fn itsTurn(s: u32) -> bool {
  if (norms[s] == 0u) { return false; } let x0 = cells[2u * s + 1u].xyz; if (all(x0 == vec3f(0.0))) { return false; }   // (founded this frame: its point is not in cells yet)
  let w = cells[2u * s].w; if (w > 24.0 && steady(x0) > 0.9) { let m = select(3u, 7u, w > 60.0); return (atomicLoad(&keys[s]) & m) == (frame.counts.z & m); } return true;
}
@compute @workgroup_size(64) fn main(@builtin(workgroup_id) g: vec3u, @builtin(local_invocation_index) li: u32) {
  if (li == 0u) { atomicStore(&many, 0u); } workgroupBarrier();
  for (var i = 0u; i < 4u; i++) { let s = g.x * 256u + i * 64u + li; if (itsTurn(s)) { todo[atomicAdd(&many, 1u)] = s; } }
  workgroupBarrier(); let n = atomicLoad(&many); for (var i = li; i < n; i += 64u) { oneMore(todo[i]); }
}
fn oneMore(s: u32) {
  var a = cells[2u * s]; var b = cells[2u * s + 1u]; let x0 = b.xyz; let st = steady(x0); let key = atomicLoad(&keys[s]);
  byReach = true; if (changes.all.y > 0.5) { a.w = min(a.w, 1.0); }
  if (bad4(a) || bad(b.w)) { a = vec4f(0.0); b.w = 0.0; }
  rngKey(key, 91u); let every = norms[s] == EVERY_WAY; let n = select(octUnpack(norms[s] & 0xfffffffeu), vec3f(0.0), every); fineLeaves = false;
  // a little about its point, so that a cell is the mean of its patch
  let bs = basis(select(n, vec3f(0.0, 1.0, 0.0), every)); let x = x0 + n * 0.012;
  var sun = 0.0; var E = vec3f(0.0); let cs = select(dot(n, frame.sunDir.xyz), 1.0, every);
  for (var k = 0u; k < 2u; k++) {
    var d: vec3f; var far = 400.0; var e = vec3f(0.0);
    if (k == 0u) { if (cs <= 0.0 || max3(frame.sunE.xyz) <= 0.0) { continue; } d = coneDir(frame.sunDir.xyz, frame.sunDir.w, rnd2()); }
    else { let l = drawLamp(x, n); if (l.x < 0.0) { continue; } let q = aimLamp(u32(l.x), x, rnd2()); d = q.dir; far = q.dist - 0.02; e = q.E * l.y * select(max(dot(n, d), 0.0), 0.25, every); if (max3(e) <= 0.0) { continue; } }
    let j = rnd2() - 0.5; let T = visible(x + (bs[0] * j.x + bs[1] * j.y) * 0.10, d, 2e-3, far); if (k == 0u) { sun = lum(T); } else { E += min(e * T, vec3f(300.0)); }
  }
  let ru = rnd2(); let rp = rnd(); var w = vec4f(0.0); if (every) { let z = ru.x * 2.0 - 1.0; let r = sqrt(max(1.0 - z * z, 0.0)); w = vec4f(r * cos(6.2832 * ru.y), z, r * sin(6.2832 * ru.y), PI); } else { w = wideDir(x, n, ru, rp); }
  if (w.w > 0.0) {
    let h = nearest(x, w.xyz, 2e-3, 600.0, NOSHADOW * 0u, 1e9);
    let sm = smokeSeen(x, w.xyz, min(h.t, 80.0)); E += min(sm.rgb * w.w, vec3f(60.0));
    if (h.piece == NOTHING) { E += skyLod(w.xyz, 1.0) * h.thru * (w.w * sm.a); }
    else { let q = surfaceMean(x, w.xyz, h); let leaf = (h.piece & LEAF_HIT) != 0u; let own = !leaf && (pieces[h.piece].lamp & 255u) != 255u;
      let L = select(q.glow, vec3f(0.0), own) + sentBack(q, leaf, rnd2(), w.xyz);
      // what it lands on is founded too, so that light deeper in is not read from the coarse cells, which are wider than walls are apart
      if (a.w > 6.0) { let wn = select(q.ng, vec3f(0.0), leaf); let wants = cacheWants(q.x, wn, !leaf && (q.flags & MOVES) != 0u); if (wants != 0u) { let w0 = u32(HITS) + s; hits[w0] = vec4u(bitcast<vec3u>(q.x), select(octPack(q.ng), EVERY_WAY, leaf)); for (var l = 0u; l < 4u; l++) { if ((wants & (1u << l)) != 0u) { cacheFound(cacheKeyOf(q.x, wn, l & 1u, l >> 1u), w0); } } } }
      E += min(L * h.thru * (w.w * sm.a), vec3f(60.0)); }
  }
  let cap = mix(min(7.0, @tune.keep), @tune.keep, st); let m = min(a.w + 1.0, cap); let k = min(1.0, 2.0 / (m + 1.0));   // later samples weigh more: the first ones were lit by a table that was still dark
 
  cachePut(s, mix(a.xyz, E, k), m, x0, mix(b.w, select(sun, 0.0, cs <= 0.0), k));
}
