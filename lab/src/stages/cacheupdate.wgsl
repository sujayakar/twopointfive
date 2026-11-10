// LIGHT ON SURFACES, one more sample an entry: the sun, one lamp, and one ray to see what else is out there, read from the table as it stands. Where that ray lands it calls for an entry too.
//   Each group looks over 256 slots, lists the few whose turn it is, then works through the list: rays of neighbours in the list keep all its lanes busy.
var<workgroup> todo: array<u32, 256>; var<workgroup> many: atomic<u32>;
fn disturbed(x: vec3f) -> bool { for (var i = 0u; i < 8u; i++) { let c = changes.at[i]; if (c.w > 0.0 && distance(x, c.xyz) < c.w) { return true; } } return false; }
fn itsTurn(s: u32) -> bool {
  if (spot[s].w == 0u) { return false; } let home = cells[2u * s + 1u].xyz; if (all(home == vec3f(0.0))) { return false; }   // (founded this frame: where it began is not in cells yet)
  let w = cells[2u * s].w; var m = 0u; if (w > FOURTH && steady(home) > 0.9) { m = select(3u, 7u, w > EIGHTH); } else if (w >= FORGET && disturbed(home)) { m = 1u; }     // (while a door near it moves it knows all it will keep: every second frame is enough)
  return (atomicLoad(&keys[s]) & m) == (frame.counts.z & m);
}
@compute @workgroup_size(64) fn main(@builtin(workgroup_id) g: vec3u, @builtin(local_invocation_index) li: u32) {
  if (li == 0u) { atomicStore(&many, 0u); } workgroupBarrier();
  for (var i = 0u; i < 4u; i++) { let s = g.x * 256u + i * 64u + li; if (itsTurn(s)) { todo[atomicAdd(&many, 1u)] = s; } }
  workgroupBarrier(); let n = atomicLoad(&many); for (var i = li; i < n; i += 64u) { oneMore(todo[i]); }
}
fn oneMore(s: u32) {
  var a = cells[2u * s]; var b = cells[2u * s + 1u]; let sp = spot[s]; let x0 = bitcast<vec3f>(sp.xyz); let n = octUnpack(sp.w & 0xfffffffeu); let st = steady(b.xyz); let key = atomicLoad(&keys[s]);
  byReach = true; if (changes.all.y > 0.5) { a.w = min(a.w, 1.0); } if (bad4(a) || bad(b.w)) { a = vec4f(0.0); b.w = 0.0; }
  rngKey(key, 91u); leafMode = 2u; let x = x0 + n * 0.012; var sun = 0.0; var E = vec3f(0.0); let cs = dot(n, frame.sunDir.xyz);
  for (var k = 0u; k < 2u; k++) {
    var d: vec3f; var far = 400.0; var e = vec3f(0.0);
    if (k == 0u) { if (cs <= 0.0 || max3(frame.sunE.xyz) <= 0.0) { continue; } d = coneDir(frame.sunDir.xyz, frame.sunDir.w, rnd2()); footLight(2.0 * frame.sunDir.w, 1.0); }
    else { let l = drawLamp(x, n); if (l.x < 0.0) { continue; } let q = aimLamp(u32(l.x), x, rnd2()); d = q.dir; far = q.dist - 0.02; footLight(lampSize(u32(l.x)), far); e = q.E * l.y * max(dot(n, d), 0.0); if (max3(e) <= 0.0) { continue; } }
    let T = visible(x, d, 2e-3, far); if (k == 0u) { sun = lum(T); } else { E += min(e * T, vec3f(300.0)); }
  }
  let w = wideDir(x, n, rnd2(), rnd()); let shaken = disturbed(b.xyz);
  for (var k = select(1u, 0u, shaken); k < 2u; k++) {                 // (near a door, a lamp that switched or a wall that gave way it first looks whether what it was on is still there)
    if (k == 1u && w.w <= 0.0) { break; } var o = x; var d = w.xyz; var far = 600.0; if (k == 0u) { foot(0.0, 0.0); o = x0 + n * 0.05; d = -n; far = 0.1; } else { foot(START, 1.0); }
    let h = nearest(o, d, select(2e-3, 0.0, k == 0u), far, 0u, 1e9); if (k == 0u) { if (h.piece != NOTHING) { continue; } fresh[2u * s] = vec4f(0.0); fresh[2u * s + 1u] = vec4f(0.0); spot[s] = vec4u(0u); return; }
    let sm = smokeSeen(x, w.xyz, min(h.t, 80.0)); let fo = blkSeen(); E += min((sm.rgb * fo.T + fo.L) * w.w, vec3f(60.0));
    if (h.piece == NOTHING) { E += skyLod(w.xyz, 1.0) * h.thru * fo.T * (w.w * sm.a); }
    else { let q = surfaceMean(x, w.xyz, h); let own = (pieces[h.piece].lamp & 255u) != 255u; E += min((select(q.glow, vec3f(0.0), own) + sentBack(q, rnd2())) * h.thru * fo.T * (w.w * sm.a), vec3f(60.0)); tellAs(HITS + s, 0x80000000u | key, q.x, q.ng); /* (tells) */ }
  }
  let most = @tune.keep * (0.5 * RISE); let cap = mix(min(FORGET, most), most, st); let m = min(a.w + 1.0, cap); let k = min(1.0, RISE / (m + RISE - 1.0));
  let o = finite4(vec4f(mix(a.xyz, E, k), m), 1e4); fresh[2u * s] = vec4f(max(o.rgb, vec3f(0.0)), o.w); fresh[2u * s + 1u] = vec4f(b.xyz, clamp(finite(mix(b.w, sun, k), 1.0), 0.0, 1.0));
}
