// LIGHT IN THE CELLS: one more look at each whose turn it is. Six rays, one about each way its contents may face, as wide as it is; what they meet is read from the light on surfaces and from
// other cells, so that light goes on from cell to cell. The sun, one lamp. (Its own contents are left out of its rays: a solid begins at its side, leaves count what hides them.)
fn six(i: u32) -> vec3f { var d = vec3f(0.0); d[i >> 1u] = select(1.0, -1.0, (i & 1u) == 1u); return d; }
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) {
  let s = id.x; if (s >= many.x) { return; } let base = s * 12u; var q: array<u32, 12>; for (var k = 0u; k < 12u; k++) { q[k] = field[base + k]; } var n = f32(q[11]); let at = places[2u * s]; let ext = places[2u * s + 1u]; let cell = bitcast<u32>(ext.w); let st = steady(at.xyz); if (changes.all.y > 0.5) { n = 0.0; }
  { let turn = select(7u, 15u, n > 40.0); if (n > 16.0 && st > 0.9 && (s & turn) != (frame.counts.z & turn)) { return; } }
  rngKey(s, 203u); leafMode = 2u; byReach = true; let h = at.w; let r = bitcast<vec4u>(extra[BLK_CELLS + (cell - 1u) * 2u]); let r2 = bitcast<vec4u>(extra[BLK_CELLS + (cell - 1u) * 2u + 1u]); let leafy = (r.x >> 24u) > 127u; let sun0 = max3(frame.sunE.xyz) > 0.0; let x = at.xyz + (vec3f(rnd(), rnd(), rnd()) - 0.5) * ext.xyz * select(0.0, 1.0, leafy); let S = frame.sunDir.xyz;
  // the sun, and one lamp: of a solid, from the side of it that faces them
  var sun = 0.0; var lampE = vec3f(0.0); var way = vec3f(0.0);
  for (var k = 0u; k < 2u; k++) {
    var d: vec3f; var far = 400.0; var e = vec3f(1.0);
    if (k == 0u) { if (!sun0) { continue; } d = coneDir(S, frame.sunDir.w, rnd2()); foot(h, 2.0 * frame.sunDir.w); }
    else { let l = drawLamp(x, vec3f(0.0)); if (l.x < 0.0) { continue; } let a = aimLamp(u32(l.x), x, rnd2()); e = a.E * l.y; if (max3(e) <= 0.0) { continue; } d = a.dir; far = a.dist - 0.02; foot(h, 0.1); }
    var o = x; if (!leafy) { let m = d * ext.xyz; let am = abs(m); var f = vec3f(0.0); if (am.x >= am.y && am.x >= am.z) { f.x = sign(d.x); } else if (am.y >= am.z) { f.y = sign(d.y); } else { f.z = sign(d.z); } o = at.xyz + (select(vec3f(rnd(), rnd(), rnd()) - 0.5, f * 0.5, abs(f) > vec3f(0.5))) * ext.xyz + f * 0.004; }
    fromCell = cell; let T = visible(o, d, 2e-3, far); fromCell = NOTHING; if (k == 0u) { sun = lum(T); } else { lampE = min(e * T, vec3f(300.0)); way = d; }
  }
  var E: array<vec3f, 6>; var mean = vec3f(0.0); let k1 = 1.0 / (n + 1.0);
  for (var i = 0u; i < 6u; i++) {
    let a = six(i); let u = rnd2(); let rr = sqrt(u.x); let bs = basis(a); let d = normalize(bs[0] * (rr * cos(6.2832 * u.y)) + bs[1] * (rr * sin(6.2832 * u.y)) + bs[2] * sqrt(max(1.0 - u.x, 0.0))); var o = x; if (!leafy) { o = at.xyz + (select(vec3f(rnd(), rnd(), rnd()) - 0.5, a * 0.5, abs(a) > vec3f(0.5))) * ext.xyz + a * 0.004; }
    foot(h, 1.0); fromCell = cell; let hit = nearest(o, d, 2e-3, 600.0, 0u, 1e9); fromCell = NOTHING; var L = vec3f(0.0); if (hit.piece == NOTHING) { L = skyLod(d, 2.0); } else if ((hit.piece & LEAF_HIT) == 0u) { let p = surfaceMean(o, d, hit); let own = (pieces[hit.piece].lamp & 255u) != 255u; L = select(p.glow, vec3f(0.0), own) + sentBack(p, false, rnd2(), d); } else { L = folBack(o + d * hit.t, d); }
    let sm = smokeSeen(o, d, min(hit.t, 80.0)); let fo = folWalk(o, d, min(hit.t, 120.0), true); E[i] = min((L * sm.a * hit.thru * fo.T + sm.rgb * fo.T + fo.L), vec3f(50.0)) * PI;
  }
  let jw = unpack2x16float(q[0]); let jz = unpack2x16float(q[1]); let keep = vec4f(select(vec3f(0.0), vec3f(jw, jz.x), n > 0.0), select(0.0, jz.y, n > 0.0)); var J = vec3f(0.0); var sv = sun;
  if (!leafy) { for (var i = 0u; i < 6u; i++) { let a = six(i); E[i] = (E[i] + lampE * max(dot(a, way), 0.0) + frame.sunE.xyz * (max(dot(a, S), 0.0) * sun)) * cellColor(r, r2, i >> 1u, (i & 1u) == 0u) / PI; } }
  var old: array<f32, 18>; for (var j = 0u; j < 9u; j++) { let v = unpack2x16float(q[2u + j]); old[2u * j] = v.x; old[2u * j + 1u] = v.y; }
  for (var i = 0u; i < 6u; i++) { for (var j = 0u; j < 3u; j++) { let o = i * 3u + j; let was = select(0.0, old[o], n > 0.0); old[o] = max(finite(mix(was, E[i][j], k1), 6e4), 0.0); mean[j] += old[o] / 6.0; } }
  if (leafy) { let c = cellLeaf(r, r2); let tau = max((f32(r.x & 255u) + f32((r.x >> 8u) & 255u) + f32((r.x >> 16u) & 255u)) * (6.4 / 255.0 / 3.0), 1e-3); let reach = (1.0 - exp(-tau)) / tau; let a1 = c[0] + c[1]; let again = 1.0 / (vec3f(1.0) - min(a1, vec3f(0.9)) * (1.0 - reach)); J = mix(keep.xyz, a1 / PI * (mean * reach + lampE * 0.25) * again, k1); sv = mix(keep.w, sun * reach, k1); }
  field[base] = pack2x16float(clamp(J.xy, vec2f(0.0), vec2f(6e4))); field[base + 1u] = pack2x16float(clamp(vec2f(J.z, sv), vec2f(0.0), vec2f(6e4))); for (var j = 0u; j < 9u; j++) { field[base + 2u + j] = pack2x16float(vec2f(old[2u * j], old[2u * j + 1u])); } field[base + 11u] = u32(min(n + 1.0, mix(6.0, 64.0, st)));
}
