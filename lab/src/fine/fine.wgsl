// FINE SHAPES, as the raster and the first hit read them (fine/template.ts lays them down, fine/stage.ts says what stands where).
//   fine    shapes, rows counted from each shape's first: levels, kind, clusters, bones · least o, reach · a step of its grid, half of the box it was made as · how big, clusters, bones' rows · a level each: error, first
//           cluster, clusters, triangles · clusters: vertices, triangles, how many of each · bones at rest, three rows each · vertices and triangles
//   insts   five rows: its ways and where it is (three), half and taper, (shape, piece, flags, tint)
//   slots   (instance, cluster's row, shape, triangles)
const FI_MOVES = 1u; const FI_CUT = 2u; const FI_SELF = 4u; const FI_GONE = 8u; const FI_BODY = 16u; const FI_OPEN = 32u;
struct FineAt { x: vec3f, y: vec3f, z: vec3f, at: vec3f, half: vec3f, taper: f32, shape: u32, piece: u32, flags: u32, tint: u32 };
fn fF(r: u32) -> vec4f { return bitcast<vec4f>(fine[r]); }
/** where an instance is now (what moves is where its piece is) */
fn fineAt(i: u32) -> FineAt {
  var a: FineAt; let r = i * 5u; let u = insts4[r + 4u]; a.shape = u.x; a.flags = u.z & 255u; a.tint = u.w; let r3 = bitcast<vec4f>(insts4[r + 3u]); a.half = r3.xyz; a.taper = r3.w; a.piece = u.y;
  if ((a.flags & FI_MOVES) == 0u) { let x = bitcast<vec4f>(insts4[r]); let y = bitcast<vec4f>(insts4[r + 1u]); let z = bitcast<vec4f>(insts4[r + 2u]); a.x = x.xyz; a.y = y.xyz; a.z = z.xyz; a.at = vec3f(x.w, y.w, z.w); return a; }
  // (turned as it lies in its piece: 0 as it is, 1 its y along the piece's x, 2 along the piece's z)
  a.piece = movers[u.y]; let p = &pieces[a.piece]; let t = (u.z >> 8u) & 3u; let X = (*p).r0.xyz; let Y = (*p).r1.xyz; let Z = (*p).r2.xyz; a.at = -(X * (*p).r0.w + Y * (*p).r1.w + Z * (*p).r2.w);
  a.x = select(X, -Y, t == 1u); a.y = select(select(Y, X, t == 1u), Z, t == 2u); a.z = select(Z, -Y, t == 2u); return a;
}
fn fineBone(bone: u32, a: FineAt, x: vec3f, row: u32, person: u32) -> vec3f {
  let b = a.shape + row + bone * 3u; let l = vec3f(dot(fF(b).xyz, x) + fF(b).w, dot(fF(b + 1u).xyz, x) + fF(b + 1u).w, dot(fF(b + 2u).xyz, x) + fF(b + 2u).w); let p = &pieces[movers[person + bone]]; /* (knees' rows are not quite square to each other: where it stands is found for what they are) */ let X = (*p).r0.xyz; let Y = (*p).r1.xyz; let Z = (*p).r2.xyz; let yz = cross(Y, Z); let c = -((*p).r0.w * yz + (*p).r1.w * cross(Z, X) + (*p).r2.w * cross(X, Y)) / dot(X, yz); return c + X * l.x + Y * l.y + Z * l.z;
}
fn fineSide(v: u32) -> f32 { return (f32(v) - 127.0) * 0.01; }
struct FineV { w: vec3f, n: vec3f, col: vec3f, bone: u32 };
/** a vertex in the world: its place; `all`: what else it knows */
fn fineVertex(a: FineAt, row: u32, all: bool) -> FineV {
  var o: FineV; let h = fine[a.shape]; let v = fine[row]; let mn = fF(a.shape + 1u); let st = fF(a.shape + 2u); var q = mn.xyz + st.xyz * vec3f(f32(v.x & 0xffffu), f32(v.x >> 16u), f32(v.y & 0xffffu)); let kind = h.y & 15u; let hi = v.y >> 16u;
  if (kind == 2u) {
    let person = insts4[fineWho * 5u + 4u].y; let w = f32(v.w >> 24u) / 255.0; let b0 = hi & 127u; let b1 = (hi >> 7u) & 127u; let row2 = fine[a.shape + 3u].z; o.bone = b0; let p0 = fineBone(b0, a, q, row2, person); o.w = p0; if (w < 0.999) { o.w = mix(fineBone(b1, a, q, row2, person), p0, w); }
    if (all) { let ln = octUnpack(v.z); let r = a.shape + row2 + b0 * 3u; let l = vec3f(dot(fF(r).xyz, ln), dot(fF(r + 1u).xyz, ln), dot(fF(r + 2u).xyz, ln)); let p = &pieces[movers[person + b0]]; o.n = (*p).r0.xyz * l.x + (*p).r1.xyz * l.y + (*p).r2.xyz * l.z; o.col = vec3f(f32(v.w & 255u), f32((v.w >> 8u) & 255u), f32((v.w >> 16u) & 255u)) / 255.0; }
    return o;
  }
  var n = octUnpack(v.z);
  if (kind == 1u) {
    let s = vec3f(fineSide(hi & 255u), fineSide(hi >> 8u), fineSide((v.w >> 16u) & 255u)); let f = min(vec3f(1.0), a.half / max(mn.w * 1.2, 1e-5)); q = s * a.half + q * f; let t = select(mix(1.0 - a.taper, 1.0, sat(s.y * 0.5 + 0.5)), mix(1.0, 1.0 + a.taper, sat(s.y * 0.5 + 0.5)), a.taper < 0.0); q = vec3f(q.x * t, q.y, q.z * t);
    if (all && (h.y & 16u) != 0u) { n = normalize(n / max(a.half, vec3f(1e-4))); n = normalize(vec3f(n.x, n.y - length(n.xz) * a.half.x * a.taper / (2.0 * a.half.y), n.z)); }
    o.col = vec3f(f32(v.w & 31u) / 31.0, f32((v.w >> 5u) & 63u) / 63.0, f32((v.w >> 11u) & 31u) / 31.0);
  } else { o.col = vec3f(f32(v.w & 255u), f32((v.w >> 8u) & 255u), f32((v.w >> 16u) & 255u)) / 255.0; }
  o.w = a.at + a.x * q.x + a.y * q.y + a.z * q.z; o.n = a.x * n.x + a.y * n.y + a.z * n.z; return o;
}
var<private> fineWho: u32 = 0u;
