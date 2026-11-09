// LIGHT IN THE CELLS: one more look at each whose turn it is, eight threads a cell: six rays, one about each way its contents may face, as wide as it is; the sun; one lamp. What they meet is read
// from the light on surfaces and from other cells, so that light goes on from cell to cell. (Its own contents are left out of its rays: a solid begins at its side, leaves count what hides them.)
fn six(i: u32) -> vec3f { var d = vec3f(0.0); d[i >> 1u] = select(1.0, -1.0, (i & 1u) == 1u); return d; }
var<workgroup> found: array<vec3f, 64>; var<workgroup> wayTo: array<vec3f, 8>;
/** a point on the side of a box that faces d */
fn onSide(at: vec3f, ext: vec3f, d: vec3f) -> vec3f { let m = abs(d * ext); var f = vec3f(0.0); if (m.x >= m.y && m.x >= m.z) { f.x = sign(d.x); } else if (m.y >= m.z) { f.y = sign(d.y); } else { f.z = sign(d.z); } return at + select(vec3f(rnd(), rnd(), rnd()) - 0.5, f * 0.5, abs(f) > vec3f(0.5)) * ext + f * 0.004; }
@compute @workgroup_size(64) fn main(@builtin(workgroup_id) wg: vec3u, @builtin(local_invocation_index) li: u32) {
  let c8 = li >> 3u; let task = li & 7u; let s = wg.x * 8u + c8; var go = s < many.x + many.y; let limb = go && s >= many.x; let base = min(s, many.x + many.y - 1u) * 12u; var n = f32(field[base + 11u]); if (changes.all.y > 0.5) { n = 0.0; }
  var at = vec4f(0.0); var ext = vec4f(0.0); var tint = vec3f(0.0); var cell = NOTHING; var r = vec4u(0u); var r2 = vec4u(0u);
  if (limb) { let w = limbRow(s - many.x); let a = extra[w]; let b = extra[w + 1u]; let c = extra[w + 2u]; let hf = extra[w + 3u]; let e = (abs(a.xyz) * hf.x + abs(b.xyz) * hf.y + abs(c.xyz) * hf.z) * 2.0; go = go && any(hf.xyz > vec3f(0.0)); at = vec4f(-(a.xyz * a.w + b.xyz * b.w + c.xyz * c.w), max(max3(e), 0.16)); ext = vec4f(e, 0.0); tint = extra[w + 4u].xyz; }
  else if (go) { at = places[2u * s]; ext = places[2u * s + 1u]; cell = bitcast<u32>(ext.w); r = bitcast<vec4u>(extra[BLK_CELLS + (cell - 1u) * 2u]); r2 = bitcast<vec4u>(extra[BLK_CELLS + (cell - 1u) * 2u + 1u]); }
  let st = steady(at.xyz); { let turn = select(3u, 15u, n > 40.0); if (!limb && n > 16.0 && st > 0.9 && (wg.x & turn) != ((frame.counts.z >> 1u) & turn)) { go = false; } }
  let leafy = (r.x >> 24u) > 127u; let S = frame.sunDir.xyz; let sun0 = max3(frame.sunE.xyz) > 0.0; let h = at.w; var out = vec3f(0.0); var way = vec3f(0.0);
  if (go) {
    rngKey(s * 8u + task, 203u); leafMode = 2u; byReach = true; let x = at.xyz + (vec3f(rnd(), rnd(), rnd()) - 0.5) * ext.xyz * select(0.0, 1.0, leafy);
    if (task < 6u) {
      let a = six(task); let u = rnd2(); let rr = sqrt(u.x); let bs = basis(a); let d = normalize(bs[0] * (rr * cos(6.2832 * u.y)) + bs[1] * (rr * sin(6.2832 * u.y)) + bs[2] * sqrt(max(1.0 - u.x, 0.0))); var o = x; if (!leafy) { o = onSide(at.xyz, ext.xyz, a * 1e3); }
      foot(h, 1.0); fromCell = cell; fromLimb = select(0u, s - many.x + 1u, limb); let hit = nearest(o, d, 2e-3, 600.0, 0u, 1e9); fromCell = NOTHING; fromLimb = 0u; var L = vec3f(0.0);
      if (hit.piece == NOTHING) { L = skyLod(d, 2.0); } else if ((hit.piece & LEAF_HIT) == 0u) { let p = surfaceMean(o, d, hit); let own = (pieces[hit.piece].lamp & 255u) != 255u; L = select(p.glow, vec3f(0.0), own) + sentBack(p, false, rnd2(), d); } else { L = folBack(o + d * hit.t, d); }
      let sm = smokeSeen(o, d, min(hit.t, 80.0)); let fo = folWalk(o, d, min(hit.t, 120.0), true); out = min((L * sm.a * hit.thru * fo.T + sm.rgb * fo.T + fo.L), vec3f(50.0)) * PI;
    } else {
      var o = x; var d = S; var far = 400.0; var e = vec3f(0.0); var wide = 2.0 * frame.sunDir.w;
      if (task == 6u) { if (sun0) { d = coneDir(S, frame.sunDir.w, rnd2()); e = vec3f(1.0); if (!leafy) { o = onSide(at.xyz, ext.xyz, d); } } }
      else { let l = drawLamp(x, vec3f(0.0)); if (l.x >= 0.0) { let k = u32(l.x); if (!leafy) { o = onSide(at.xyz, ext.xyz, lamps[k].at - x); } let a = aimLamp(k, o, rnd2()); e = a.E * l.y; d = a.dir; far = a.dist - 0.02; wide = select(2.0 * lamps[k].area, sqrt(lamps[k].area), lamps[k].kind == 0u) / max(far, 0.1); way = d; } }
      if (max3(e) > 0.0) { foot(h, wide); fromCell = cell; fromLimb = select(0u, s - many.x + 1u, limb); out = min(e * visible(o, d, 2e-3, far), vec3f(300.0)); fromCell = NOTHING; fromLimb = 0u; }
    }
  }
  found[li] = out; if (task == 7u) { wayTo[c8] = way; } workgroupBarrier(); if (!go || task != 0u) { return; }
  var q: array<u32, 12>; for (var k = 0u; k < 12u; k++) { q[k] = field[base + k]; } let sun = found[li + 6u].y; let lampE = found[li + 7u]; let to = wayTo[c8]; var mean = vec3f(0.0); let k1 = 1.0 / (n + 1.0);
  let jw = unpack2x16float(q[0]); let jz = unpack2x16float(q[1]); let keep = vec4f(select(vec3f(0.0), vec3f(jw, jz.x), n > 0.0), select(0.0, jz.y, n > 0.0)); var J = vec3f(0.0); var sv = sun; var old: array<f32, 18>; for (var j = 0u; j < 9u; j++) { let v = unpack2x16float(q[2u + j]); old[2u * j] = v.x; old[2u * j + 1u] = v.y; }
  for (var i = 0u; i < 6u; i++) { let a = six(i); var E = found[li + i]; if (!leafy) { E = (E + lampE * max(dot(a, to), 0.0) + frame.sunE.xyz * (max(dot(a, S), 0.0) * sun)) * select(cellColor(r, r2, i >> 1u, (i & 1u) == 0u), tint, limb) / PI; } for (var j = 0u; j < 3u; j++) { let o = i * 3u + j; let was = select(0.0, old[o], n > 0.0); old[o] = max(finite(mix(was, E[j], k1), 6e4), 0.0); mean[j] += old[o] / 6.0; } }
  if (leafy) { let c = cellLeaf(r, r2); let tau = max((f32(r.x & 255u) + f32((r.x >> 8u) & 255u) + f32((r.x >> 16u) & 255u)) * (6.4 / 255.0 / 3.0), 1e-3); let reach = (1.0 - exp(-tau)) / tau; let a1 = c[0] + c[1]; let again = 1.0 / (vec3f(1.0) - min(a1, vec3f(0.9)) * (1.0 - reach)); J = mix(keep.xyz, a1 / PI * (mean * reach + lampE * 0.25) * again, k1); sv = mix(keep.w, sun * reach, k1); }
  field[base] = pack2x16float(clamp(J.xy, vec2f(0.0), vec2f(6e4))); field[base + 1u] = pack2x16float(clamp(vec2f(J.z, sv), vec2f(0.0), vec2f(6e4))); for (var j = 0u; j < 9u; j++) { field[base + 2u + j] = pack2x16float(vec2f(old[2u * j], old[2u * j + 1u])); } field[base + 11u] = u32(min(n + 1.0, select(mix(6.0, 64.0, st), 10.0, limb)));
}
