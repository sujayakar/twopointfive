// LIGHT IN THE CELLS: one more look at each whose turn it is, eight threads a cell: six rays, one about each way its leaves may face, as wide as it is; the sun; one lamp. What they meet is read
// from the light on surfaces and from other cells, so that light goes on from cell to cell. (Its own leaves are left out of its rays: they count what hides them.)
/** what is nearer to a leaf than this its own twig has told it of */
const REACH_F = 0.35;
fn six(i: u32) -> vec3f { var d = vec3f(0.0); d[i >> 1u] = select(1.0, -1.0, (i & 1u) == 1u); return d; }
/** a side's color in this weather (cells are measured dry and bare): snow on what looks up, and rain that has soaked in. i: which side; 6: a leaf */
fn weathered(c: vec3f, i: u32, x: vec3f) -> vec3f { if (frame.more.x <= 0.0 && frame.weather.x <= 0.0) { return c; } let out = exposed(x + vec3f(0.0, 0.25, 0.0)); if (frame.more.x > 0.0) { return mix(c, vec3f(0.86, 0.895, 0.94), frame.more.x * out * select(select(0.25, 0.0, i == 3u), 1.0, i == 2u)); } return c * (1.0 - frame.weather.x * out * select(0.12, 0.28, i == 2u)); }
var<workgroup> found: array<vec3f, 64>; var<workgroup> torchy: array<vec3f, 64>;
@compute @workgroup_size(64) fn main(@builtin(workgroup_id) wg: vec3u, @builtin(local_invocation_index) li: u32) {
  let c8 = li >> 3u; let task = li & 7u; let s = wg.x * 8u + c8; var go = s < many.x; let base = min(s, many.x - 1u) * 12u; var n = f32(field[base + 11u] & 255u); if (changes.all.y > 0.5) { n = 0.0; }
  var at = vec4f(0.0); var ext = vec4f(0.0); var cell = NOTHING; var r = vec4u(0u); var r2 = vec4u(0u);
  if (go) { at = places[2u * s]; ext = places[2u * s + 1u]; cell = s + 1u; r = extra[BLK_CELLS + (cell - 1u) * 2u]; r2 = extra[BLK_CELLS + (cell - 1u) * 2u + 1u]; }
  let st = steady(at.xyz); if (many.z > 0u) { n = min(n, 6.0); } { let turn = select(3u, 7u, n > 32.0); if (n > 16.0 && st > 0.9 && (wg.x & turn) != (many.y & turn)) { go = false; } }
  let S = frame.sunDir.xyz; let sun0 = max3(frame.sunE.xyz) > 0.0; let h = at.w; var out = vec3f(0.0); var fleet = vec3f(0.0);
  if (go) {
    rngKey(s * 8u + task, 203u); leafMode = 2u; byReach = true; still = true; blkPast = REACH_F; let x = at.xyz + (vec3f(rnd(), rnd(), rnd()) - 0.5) * ext.xyz;
    if (task < 6u) {
      let a = six(task); let u = rnd2(); let rr = sqrt(u.x); let bs = basis(a); let d = normalize(bs[0] * (rr * cos(6.2832 * u.y)) + bs[1] * (rr * sin(6.2832 * u.y)) + bs[2] * sqrt(max(1.0 - u.x, 0.0))); var o = x;
      foot(h, 1.0); fromCell = cell; let hit = nearest(o, d, 2e-3, 600.0, 0u, 1e9); fromCell = NOTHING; var L = vec3f(0.0);
      if (hit.piece == NOTHING) { L = skyLod(d, 2.0); } else { let p = surfaceMean(o, d, hit); let own = (pieces[hit.piece].lamp & 255u) != 255u; let sent = sentBack(p, rnd2(), d); L = select(p.glow, vec3f(0.0), own) + sent - sentTorch; fleet = sentTorch; }
      let sm = smokeSeen(o, d, min(hit.t, 80.0)); let fo = blkSeen(); out = min((L * sm.a * hit.thru * fo.T + sm.rgb * fo.T + fo.L), vec3f(50.0)) * PI; fleet *= PI * sm.a * fo.T * hit.thru; fromCell = cell; fleet += spotLight(o, a, h, 4u); fromCell = NOTHING;
    } else {
      var o = x; var d = S; var far = 400.0; var e = vec3f(0.0); var wide = 2.0 * frame.sunDir.w;
      if (task == 6u) { if (sun0) { d = coneDir(S, frame.sunDir.w, rnd2()); e = vec3f(1.0); } }
      else { let l = drawLamp(x, vec3f(0.0)); if (l.x >= 0.0) { let k = u32(l.x); let a = aimLamp(k, o, rnd2()); e = a.E * l.y; d = a.dir; far = a.dist - 0.02; wide = lampSize(k) / max(far, 0.1); } }
      if (max3(e) > 0.0) { footLight(wide, 1.0); fromCell = cell; out = min(e * visible(o, d, 2e-3, far), vec3f(300.0)); fromCell = NOTHING; }
    }
  }
  found[li] = out; torchy[li] = fleet; workgroupBarrier(); if (!go || task != 0u) { return; }
  var q: array<u32, 12>; for (var k = 0u; k < 12u; k++) { q[k] = field[base + k]; } let sun = found[li + 6u].y; let lampE = found[li + 7u]; var mean = vec3f(0.0); let k1 = 1.0 / (n + 1.0);
  let jw = unpack2x16float(q[0]); let jz = unpack2x16float(q[1]); let keep = vec4f(select(vec3f(0.0), vec3f(jw, jz.x), n > 0.0), select(0.0, jz.y, n > 0.0)); var J = vec3f(0.0); var sv = sun; var told = 0u; var old: array<f32, 18>; for (var j = 0u; j < 9u; j++) { let v = unpack2x16float(q[2u + j]); old[2u * j] = v.x; old[2u * j + 1u] = v.y; }
  for (var i = 0u; i < 6u; i++) { var E = found[li + i]; mean += E / 6.0; E += torchy[li + i];       /* (a leaf's own pixels see what torches light; what cells send on is kept long, and torches move) */
    for (var j = 0u; j < 3u; j++) { let o = i * 3u + j; let was = select(0.0, old[o], n > 0.0); old[o] = max(finite(mix(was, E[j], k1), 6e4), 0.0); } }
  let cm = weathered(cellMean(r, r2), 6u, at.xyz); let v8 = vec3u(sqrt(cm) * 255.0 + 0.5); told = (v8.x << 8u) | (v8.y << 16u) | (v8.z << 24u); let c = cellLeaf(cm, r); let e = exp(-vec3f(vec3u(r.x, r.x >> 8u, r.x >> 16u) & vec3u(255u)) * (6.4 / 255.0)); let reach = min(1.0, (1.0 - (e.x + e.y + e.z) / 3.0) * 256.0 / f32(max(r2.z, 1u))); /* (what share of its leaves shows from outside) */ let a1 = c[0] + c[1]; let again = 1.0 / (vec3f(1.0) - min(a1, vec3f(0.9)) * (1.0 - reach)); J = mix(keep.xyz, a1 / PI * (mean * reach + lampE * 0.25) * again, k1); sv = mix(keep.w, sun * reach, k1);
  field[base] = pack2x16float(clamp(J.xy, vec2f(0.0), vec2f(6e4))); field[base + 1u] = pack2x16float(clamp(vec2f(J.z, sv), vec2f(0.0), vec2f(6e4))); for (var j = 0u; j < 9u; j++) { field[base + 2u + j] = pack2x16float(vec2f(old[2u * j], old[2u * j + 1u])); } field[base + 11u] = u32(min(n + 1.0, mix(6.0, select(48.0, 16.0, frame.sunE.w > 0.0), st))) | told;
}
