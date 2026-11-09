// MESHES, as one more kind of piece: what a ray meets of one, and what is there. Rows are those of mesh/pack.ts, from MESH_ROW on.
//   object   n levels, bits, children's row, children · a ball round it · the width at which it is gone · then a level each: error, tree, first child, children
//   child    three rows object → mesh (scaled) · mesh, tint, scale
//   mesh     corner, levels · a step of its grid, clusters' row · triangles' row, picking row and entries, clusters · ball · then a level each: error, tree, triangles, spread
//   cluster  vertices' row, triangles, vertices, level      triangle  three vertices of its cluster, a byte each
//   vertex   x y, z and spread, normal, color               node      least corner, first · greatest corner, how many (0: two nodes lie at `first`)
// A hit's code is MESH_HIT | child << bits | cluster << 7 | triangle.
const MESH = 64u; const MESH_HIT = 0x80000000u;
fn mrow(r: u32) -> vec4f { return extra[MESH_ROW + r]; }
fn mrowU(r: u32) -> vec4u { return bitcast<vec4u>(extra[MESH_ROW + r]); }
/** any hit will do (a shadow's ray) */
var<private> meshAny = false;
/** how far a ray's levels may stand off the finest: so many metres (meshError), or so much of its footprint (meshFoot); or all at a level counted from the finest (meshLevel: 1 = the finest, 0 = by the two others) */
var<private> meshError = 0.0; var<private> meshFoot = 0.0; var<private> meshLevel = 0u;
/** and so many levels coarser than that: for the seam's study */
var<private> meshMore = 0u;
/** how wide the ray's answer is there, in metres that a level may be off */
fn meshWide(t: f32) -> f32 { return max(meshError, meshFoot * footAt(t)); }
/** as far as this a ray does not meet the backs of triangles: it began under them, on a finer level */
var<private> meshBack = 0.0;
/** nodes and triangles looked at (nothing reads it but the panel) */
var<private> meshSteps = 0u;

fn meshVertex(row: u32, mn: vec3f, st: vec3f) -> vec3f { let v = mrowU(row); return mn + st * vec3f(f32(v.x & 0xffffu), f32(v.x >> 16u), f32(v.y & 0xffffu)); }
/** which level of a mesh stands within `e` of its finest */
fn meshPick(h: u32, n: u32, e: f32) -> u32 { if (meshLevel > 0u) { return min(meshLevel - 1u, n - 1u); } var l = n - 1u; loop { if (l == 0u || mrow(h + 4u + l).x <= e) { break; } l -= 1u; } return min(l + meshMore, n - 1u); }
fn boxSpan(a: vec3f, b: vec3f, o: vec3f, inv: vec3f) -> vec2f { let p = (a - o) * inv; let q = (b - o) * inv; return vec2f(max3(min(p, q)), min3(max(p, q))); }
fn meetMesh(k: u32, ro: vec3f, rd: vec3f, tmin: f32, tmax: f32) -> bool {
  let p = &pieces[k]; let sc = (*p).quad.x; let oo = vec3f(dot((*p).r0.xyz, ro) + (*p).r0.w, dot((*p).r1.xyz, ro) + (*p).r1.w, dot((*p).r2.xyz, ro) + (*p).r2.w) / sc; let od = vec3f(dot((*p).r0.xyz, rd), dot((*p).r1.xyz, rd), dot((*p).r2.xyz, rd)) / sc;
  let oh = (*p).planes; let head = mrowU(oh); let hull = boxSpan(-(*p).half / sc, (*p).half / sc, oo, safeInv(od)); if (max(hull.x, tmin) > min(hull.y, tmax)) { return false; }
  var li = head.x - 1u; let may = meshWide(max(hull.x, 0.0)) / sc; loop { if (li == 0u || (meshLevel == 0u && mrow(oh + 3u + li).x <= may)) { break; } li -= 1u; }
  var stack: array<u32, 64>; var stackT: array<f32, 64>; var sp = 1; stack[0] = mrowU(oh + 3u + li).y; stackT[0] = tmin;
  var best = tmax; var found = false; var o = oo; var d = od; var inv = safeInv(d); var child = 0xffffffffu; var mh = 0u; var mn = vec3f(0.0); var st = vec3f(1.0); var recs = 0u; var tris = 0u; let backTo = meshBack;
  loop {
    if (sp == 0) { break; } sp -= 1; let at = stack[sp]; if (at == 0xffffffffu) { child = at; o = oo; d = od; inv = safeInv(d); continue; } if (stackT[sp] >= best) { continue; } meshSteps += 1u;
    if ((at & 0x80000000u) != 0u) {                                  // a child: into its own frame
      child = at & 0x7fffffffu; let c = head.z + child * 4u; let c0 = mrow(c); let c1 = mrow(c + 1u); let c2 = mrow(c + 2u); let c3 = mrowU(c + 3u); o = vec3f(dot(c0.xyz, oo) + c0.w, dot(c1.xyz, oo) + c1.w, dot(c2.xyz, oo) + c2.w); d = vec3f(dot(c0.xyz, od), dot(c1.xyz, od), dot(c2.xyz, od)); inv = safeInv(d);
      mh = c3.x; let a = mrow(mh); let b = mrow(mh + 1u); mn = a.xyz; st = b.xyz; recs = bitcast<u32>(b.w); tris = mrowU(mh + 2u).x; let l = meshPick(mh, bitcast<u32>(a.w), meshWide(max(stackT[sp], 0.0)) / (sc * bitcast<f32>(c3.z)));
      stack[sp] = 0xffffffffu; stack[sp + 1] = mrowU(mh + 4u + l).y; stackT[sp + 1] = stackT[sp]; sp += 2; continue;
    }
    let n0 = mrow(at); let n1 = mrow(at + 1u); let first = bitcast<u32>(n0.w); let count = bitcast<u32>(n1.w);
    if (count == 0u) {
      let a0 = mrow(first); let a1 = mrow(first + 1u); let b0 = mrow(first + 2u); let b1 = mrow(first + 3u); let sa = boxSpan(a0.xyz, a1.xyz, o, inv); let sb = boxSpan(b0.xyz, b1.xyz, o, inv); let ta = max(sa.x, tmin); let tb = max(sb.x, tmin); let ha = ta <= min(sa.y, best); let hb = tb <= min(sb.y, best); if (sp > 60) { continue; }
      if (ha && hb) { let near = ta <= tb; stack[sp] = select(first, first + 2u, near); stackT[sp] = select(ta, tb, near); stack[sp + 1] = select(first + 2u, first, near); stackT[sp + 1] = select(tb, ta, near); sp += 2; }
      else if (ha) { stack[sp] = first; stackT[sp] = ta; sp += 1; } else if (hb) { stack[sp] = first + 2u; stackT[sp] = tb; sp += 1; }
      continue;
    }
    if (child == 0xffffffffu) { for (var j = 0u; j < count; j++) { stack[sp] = 0x80000000u | (first + j); stackT[sp] = max(boxSpan(n0.xyz, n1.xyz, o, inv).x, tmin); sp += 1; } continue; }
    let vb = mrowU(recs + (first >> 7u)).x;
    for (var j = 0u; j < count; j++) {
      let t = first + j; let ix = mrowU(tris + (t >> 2u))[t & 3u]; let a = meshVertex(vb + (ix & 255u), mn, st); let e1 = meshVertex(vb + ((ix >> 8u) & 255u), mn, st) - a; let e2 = meshVertex(vb + ((ix >> 16u) & 255u), mn, st) - a;
      let q = cross(d, e2); let det = dot(e1, q); if (det == 0.0) { continue; } let iv = 1.0 / det; let s = o - a; let u = dot(s, q) * iv; if (u < 0.0 || u > 1.0) { continue; } let r = cross(s, e1); let v = dot(d, r) * iv; if (v < 0.0 || u + v > 1.0) { continue; }
      let h = dot(e2, r) * iv; if (h <= tmin || h >= best || (det < 0.0 && h < backTo)) { continue; }
      best = h; found = true; hCode = MESH_HIT | (child << head.y) | t; if (meshAny) { hT = best; return true; }
    }
  }
  if (found) { hT = best; } return found;
}
fn meet(k: u32, ro: vec3f, rd: vec3f, tmin: f32, tmax: f32, cut: f32) -> bool { if ((pieces[k].flags & MESH) != 0u) { return meetMesh(k, ro, rd, tmin, tmax); } return meetPiece(k, ro, rd, tmin, tmax, cut); }
fn visible(ro: vec3f, rd: vec3f, tmin: f32, tmax: f32) -> vec3f { meshAny = true; let T = visiblePieces(ro, rd, tmin, tmax); meshAny = false; return T; }

// ---- what is there
fn vnoise3(p: vec3f) -> f32 { let i = floor(p); let f = p - i; let w = f * f * (3.0 - 2.0 * f); let a = mix(hash33(i).x, hash33(i + vec3f(1.0, 0.0, 0.0)).x, w.x); let b = mix(hash33(i + vec3f(0.0, 1.0, 0.0)).x, hash33(i + vec3f(1.0, 1.0, 0.0)).x, w.x); let c = mix(hash33(i + vec3f(0.0, 0.0, 1.0)).x, hash33(i + vec3f(1.0, 0.0, 1.0)).x, w.x); let e = mix(hash33(i + vec3f(0.0, 1.0, 1.0)).x, hash33(i + vec3f(1.0, 1.0, 1.0)).x, w.x); return mix(mix(a, b, w.y), mix(c, e, w.y), w.z); }
fn meshSurf(ro: vec3f, rd: vec3f, h: Hit, fp: f32) -> Surf {
  let x = ro + rd * h.t; let k = h.piece; let p = &pieces[k]; let m = mats[(*p).mat & 255u]; let head = mrowU((*p).planes); let code = h.code & 0x7fffffffu; let child = code >> head.y; let t = code & ((1u << head.y) - 1u);
  let c = head.z + child * 4u; let c0 = mrow(c); let c1 = mrow(c + 1u); let c2 = mrow(c + 2u); let c3 = mrowU(c + 3u); let ol = toLocal(k, x) / (*p).quad.x; let l = vec3f(dot(c0.xyz, ol) + c0.w, dot(c1.xyz, ol) + c1.w, dot(c2.xyz, ol) + c2.w);
  let a4 = mrow(c3.x); let b4 = mrow(c3.x + 1u); let vb = mrowU(bitcast<u32>(b4.w) + (t >> 7u)).x; let ix = mrowU(mrowU(c3.x + 2u).x + (t >> 2u))[t & 3u]; let i0 = vb + (ix & 255u); let i1 = vb + ((ix >> 8u) & 255u); let i2 = vb + ((ix >> 16u) & 255u);
  let A = meshVertex(i0, a4.xyz, b4.xyz); let e1 = meshVertex(i1, a4.xyz, b4.xyz) - A; let e2 = meshVertex(i2, a4.xyz, b4.xyz) - A; let s0 = l - A; let gn = cross(e1, e2); let g2 = max(dot(gn, gn), 1e-30); var u = clamp(dot(cross(s0, e2), gn) / g2, 0.0, 1.0); let v = clamp(dot(cross(e1, s0), gn) / g2, 0.0, 1.0 - u); let w = 1.0 - u - v;
  let v0 = mrowU(i0); let v1 = mrowU(i1); let v2 = mrowU(i2); let ln = normalize(octUnpack(v0.z) * w + octUnpack(v1.z) * u + octUnpack(v2.z) * v); let col = unpack4x8unorm(v0.w) * w + unpack4x8unorm(v1.w) * u + unpack4x8unorm(v2.w) * v; let spread = (f32(v0.y >> 16u) * w + f32(v1.y >> 16u) * u + f32(v2.y >> 16u) * v) / 65535.0;
  let cs = bitcast<f32>(c3.z); var s: Surf; s.x = x; s.piece = k; s.flags = (*p).flags & ~15u; s.kind = m.kind; s.through = vec3f(0.0); s.glow = vec3f(0.0); s.metal = m.metal; s.f0 = m.f0;
  s.ng = normalize(toWorld(k, c0.xyz * gn.x + c1.xyz * gn.y + c2.xyz * gn.z)); s.n = normalize(toWorld(k, c0.xyz * ln.x + c1.xyz * ln.y + c2.xyz * ln.z)); if (dot(s.ng, rd) > 0.0) { s.ng = -s.ng; s.n = -s.n; }
  let tp = unpackTint((*p).tint); let tc = unpackTint(c3.y); var base = m.base * tp * tp * tc * tc * col.rgb * col.rgb;
  // under what triangles hold: the stone's own grain, two sizes of it
  var rough2 = m.rough * m.rough + 2.0 * spread; let grain = 1.0 - smoothstep(0.00008, 0.0006, fp);
  if (grain > 0.0) { let q = l * cs * (*p).quad.x; let f = 1600.0; let e = 0.5 / f; let g0 = vnoise3(q * f) + 0.5 * vnoise3(q * f * 2.7 + 5.0) * shows(f * 2.7, fp); let gx = vnoise3((q + vec3f(e, 0.0, 0.0)) * f); let gy = vnoise3((q + vec3f(0.0, e, 0.0)) * f); let gz = vnoise3((q + vec3f(0.0, 0.0, e)) * f); let lg = (vec3f(gx, gy, gz) - vnoise3(q * f)) * (0.45 * grain); s.n = normalize(s.n - normalize(toWorld(k, c0.xyz * lg.x + c1.xyz * lg.y + c2.xyz * lg.z) + s.n * 1e-6) * length(lg)); if (dot(s.n, s.ng) < 0.05) { s.n = normalize(s.n + s.ng * 0.3); } base *= 1.0 + 0.16 * grain * (g0 - 0.75); }
  s.rough = clamp(sqrt(rough2), 0.05, 1.0); s.albedo = clamp(base, vec3f(0.0), vec3f(0.94)); s.ao = vec2f(col.a, 0.0); return s;
}
fn surfaceAt(ro: vec3f, rd: vec3f, h: Hit, fp: f32) -> Surf { if ((h.piece & LEAF_HIT) == 0u && (pieces[h.piece].flags & MESH) != 0u) { return meshSurf(ro, rd, h, fp); } return surfacePiece(ro, rd, h, fp); }
