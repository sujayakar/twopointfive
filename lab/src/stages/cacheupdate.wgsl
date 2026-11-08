// LIGHT ON SURFACES, one more sample an entry: the sun, one lamp, and one ray to see what else is out there, read from the table as it stands.
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) {
  let s = id.x; if (s >= SLOTS) { return; } if (norms[s] == 0u) { return; }
  var a = cells[2u * s]; let b = cells[2u * s + 1u]; let x0 = b.xyz; let st = steady(x0);
  if (changes.all.y > 0.5) { a.w = min(a.w, 1.0); }
  if (!(a.w >= 0.0)) { a.w = 0.0; }
  if (a.w > 24.0 && st > 0.9 && (s & 3u) != (frame.counts.z & 3u)) { return; }
  let key = atomicLoad(&keys[s]); rngKey(key, 91u); let n = octUnpack(norms[s] & 0xfffffffeu); fineLeaves = false;
  // a little about its point, so that a cell is the mean of its patch
  let bs = basis(n); let x = x0 + n * 0.012;
  var sun = 0.0; var E = vec3f(0.0); let cs = dot(n, frame.sunDir.xyz);
  for (var k = 0u; k < 2u; k++) {
    var d: vec3f; var far = 400.0; var e = vec3f(0.0);
    if (k == 0u) { if (cs <= 0.0 || max3(frame.sunE.xyz) <= 0.0) { continue; } d = coneDir(frame.sunDir.xyz, frame.sunDir.w, rnd2()); }
    else { let l = drawLamp(x, n); if (l.x < 0.0) { continue; } let q = aimLamp(u32(l.x), x, rnd2()); d = q.dir; far = q.dist - 0.02; e = q.E * l.y * max(dot(n, d), 0.0); if (max3(e) <= 0.0) { continue; } }
    let j = rnd2() - 0.5; let T = visible(x + (bs[0] * j.x + bs[1] * j.y) * 0.10, d, 2e-3, far); if (k == 0u) { sun = lum(T); } else { E += min(e * T, vec3f(80.0)); }
  }
  let w = wideDir(x, n, rnd2(), rnd());
  if (w.w > 0.0) {
    let h = nearest(x, w.xyz, 2e-3, 600.0, NOSHADOW * 0u, 1e9);
    if (h.piece == NOTHING) { E += skyLod(w.xyz, 4.0) * h.thru * w.w; }
    else { let q = surfaceMean(x, w.xyz, h); let c = cacheRead(q.x, q.ng, rnd2()); let own = (h.piece & LEAF_HIT) == 0u && (pieces[h.piece].lamp & 255u) != 255u;
      var L = select(q.glow, vec3f(0.0), own); if (c.n > 0.0) { L += matteOf(q) / PI * (c.E + frame.sunE.xyz * max(dot(q.ng, frame.sunDir.xyz), 0.0) * c.sun); } else if ((h.piece & LEAF_HIT) != 0u) { L += q.albedo / PI * frame.sunE.xyz * 0.25; }
      E += min(L * h.thru * w.w, vec3f(60.0)); }
  }
  let cap = mix(7.0, 96.0, st); let m = min(a.w + 1.0, cap); let k = 1.0 / m;
  cachePut(s, mix(a.xyz, E, k), m, x0, mix(b.w, select(sun, 0.0, cs <= 0.0), k));
}
