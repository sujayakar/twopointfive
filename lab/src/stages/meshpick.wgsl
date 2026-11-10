// WHICH CLUSTERS ARE DRAWN. From every placed mesh down the tree it is picked by: what is out of view, faces away or is hidden goes; of the rest the coarsest whose error stays under `tau`
// pixels, and whose coarser stand-in's would not. Each round takes a list of nodes and makes the next; what is picked goes into slots.
struct MeshK { tau: f32, fixed: f32, pieces: u32, most: u32, cone: f32, hide: f32, round: u32, queue: u32, hzb: vec4f };
struct Local { c0: vec4f, c1: vec4f, c2: vec4f, piece: u32, child: u32, mesh: u32, scale: f32, cam: vec3f };
// n: 0.. items in the list each round begins with; 24 slots, 25 triangles, 26 turned away by what hid them, 27 and 28 slots and indices said so far, 29 a list was full, 30 indices, 31 (kept from frame to frame) how many times coarser than asked, so that no list is full
const N_SLOTS = 24u; const N_TRIS = 25u; const N_HID = 26u; const N_FULL = 29u;
fn inside(x: vec3f, r: f32) -> bool {
  let v = x - frame.camPos.xyz; let f = dot(v, frame.camFwd.xyz); if (f < 0.25 - r) { return false; } let R = frame.camRight.xyz; let U = frame.camUp.xyz; let tx = length(R); let ty = length(U);
  return f * tx - abs(dot(v, R)) / tx > -r * sqrt(1.0 + tx * tx) && f * ty - abs(dot(v, U)) / ty > -r * sqrt(1.0 + ty * ty);
}
/** child's frame → object's, and on to the world's; and its own from the world's */
fn localOf(piece: u32, child: u32) -> Local {
  var l: Local; let p = &pieces[piece]; let h = mrowU((*p).planes); let c = h.z + child * 4u; l.c0 = mrow(c); l.c1 = mrow(c + 1u); l.c2 = mrow(c + 2u); let c3 = mrowU(c + 3u); l.piece = piece; l.child = child; l.mesh = c3.x; l.scale = bitcast<f32>(c3.z) * (*p).quad.x;
  let o = toLocal(piece, frame.camPos.xyz) / (*p).quad.x; l.cam = vec3f(dot(l.c0.xyz, o) + l.c0.w, dot(l.c1.xyz, o) + l.c1.w, dot(l.c2.xyz, o) + l.c2.w); return l;
}
fn outward(l: Local, x: vec3f) -> vec3f { let p = &pieces[l.piece]; let d = x - vec3f(l.c0.w, l.c1.w, l.c2.w); let o = (l.c0.xyz * d.x + l.c1.xyz * d.y + l.c2.xyz * d.z) / dot(l.c0.xyz, l.c0.xyz) * (*p).quad.x - vec3f((*p).r0.w, (*p).r1.w, (*p).r2.w); return (*p).r0.xyz * o.x + (*p).r1.xyz * o.y + (*p).r2.xyz * o.z; }
fn tauNow() -> f32 { return meshK.tau * max(bitcast<f32>(atomicLoad(&n[31])), 1.0); }
/** would an error so big (in the mesh's own measure), judged by this ball, be seen? */
fn shown(l: Local, e: f32, b: vec4f) -> bool { if (meshK.fixed > 0.0) { return e * l.scale > meshK.fixed; } let d = max(distance(l.cam, b.xyz) - b.w, 0.02 / l.scale); return e > tauNow() * (2.0 * length(frame.camUp.xyz) / frame.size.y) * d; }
/** behind what was drawn? `prev`: by last frame's camera and depth. The ball's square on the screen against the farthest depth in it */
fn hidden(x: vec3f, r: f32, prev: bool) -> bool {
  let P = select(frame.camPos.xyz, frame.prevPos.xyz, prev); let F = select(frame.camFwd.xyz, frame.prevFwd.xyz, prev); let R = select(frame.camRight.xyz, frame.prevRight.xyz, prev); let U = select(frame.camUp.xyz, frame.prevUp.xyz, prev);
  let v = x - P; let z = dot(v, F); if (z - r < 0.3) { return false; } let near = z - r; let a = vec2f(dot(v, R) / dot(R, R), -dot(v, U) / dot(U, U)) / z * 0.5 + 0.5; let e = r / near * vec2f(0.5 / length(R), 0.5 / length(U)) + 1.5 / meshK.hzb.xy; let lo = clamp(a - e, vec2f(0.0), vec2f(1.0)); let hi = clamp(a + e, vec2f(0.0), vec2f(1.0)); if (any(lo >= hi)) { return false; }
  /* (a mip is half the one under it, rounded down, its last texel taking what is left over: so a texel's number is the pixel's, shifted) */
  let px = (hi - lo) * meshK.hzb.xy; let lod = u32(clamp(ceil(log2(max(max(px.x, px.y), 1.0))), 0.0, meshK.hzb.z - 1.0)); let top = vec2i(meshK.hzb.xy) - 1; let dim = max(vec2i(meshK.hzb.xy) >> vec2u(lod), vec2i(1)); let i0 = min(clamp(vec2i(floor(lo * meshK.hzb.xy)), vec2i(0), top) >> vec2u(lod), dim - 1); let i1 = min(clamp(vec2i(floor(hi * meshK.hzb.xy)), vec2i(0), top) >> vec2u(lod), dim - 1); if (any(i1 - i0 > vec2i(2))) { return false; }
  var far = 1e9; for (var y = i0.y; y <= i1.y; y++) { for (var xx = i0.x; xx <= i1.x; xx++) { far = min(far, textureLoad(hzb, vec2i(xx, y), lod).x); } } return 0.25 / near < far;
}
fn slotOut(l: Local, cluster: u32) {
  let s = atomicAdd(&n[N_SLOTS], 1u); if (s >= meshK.most) { atomicStore(&n[N_FULL], 1u); return; } let a = mrow(l.mesh); let bu = mrowU(l.mesh + 1u); let b = asF(bu); let rec = mrowU(bu.w + cluster); let tris = mrowU(l.mesh + 2u).x + cluster * 32u;
  let o = outward(l, a.xyz); let X = outward(l, a.xyz + vec3f(b.x * 65536.0, 0.0, 0.0)) - o; let Y = outward(l, a.xyz + vec3f(0.0, b.y * 65536.0, 0.0)) - o; let Z = outward(l, a.xyz + vec3f(0.0, 0.0, b.z * 65536.0)) - o;
  slots[s * 4u] = bitcast<vec4u>(vec4f(X / 65536.0, o.x)); slots[s * 4u + 1u] = bitcast<vec4u>(vec4f(Y / 65536.0, o.y)); slots[s * 4u + 2u] = bitcast<vec4u>(vec4f(Z / 65536.0, o.z)); slots[s * 4u + 3u] = vec4u(rec.x, tris, (l.piece << 11u) | l.child, (cluster << 8u) | rec.y); atomicAdd(&n[N_TRIS], rec.y);
}
/** a child of every placed object, its mesh's top put on the list */
@compute @workgroup_size(64) fn seed(@builtin(global_invocation_id) id: vec3u) {
  if (id.y >= meshK.pieces) { return; } let k = placed[id.y]; let p = &pieces[k]; let oh = (*p).planes; let head = mrowU(oh); let ball = mrow(oh + 1u); let sc = (*p).quad.x; let at = vec3f(-(*p).r0.w, -(*p).r1.w, -(*p).r2.w) + ball.xyz * sc; let x = (*p).r0.xyz * at.x + (*p).r1.xyz * at.y + (*p).r2.xyz * at.z; let r = ball.w * sc; if (!inside(x, r)) { return; }
  if ((((*p).flags) & CUT) != 0u && x.y - r > frame.misc.y) { return; }      /* (the section hides it) */
  // which level of the object: its coarsest that is not seen to be one
  let d = max(distance(frame.camPos.xyz, x) - r, 0.02); var may = tauNow() * (2.0 * length(frame.camUp.xyz) / frame.size.y) * d / sc; if (meshK.fixed > 0.0) { may = meshK.fixed / sc; } var li = head.x - 1u; loop { if (li == 0u || mrow(oh + 3u + li).x <= may) { break; } li -= 1u; }
  let lv = mrowU(oh + 3u + li); if (id.x >= lv.w) { return; } let child = lv.z + id.x; let l = localOf(k, child); let b = mrow(l.mesh + 3u); let cx = outward(l, b.xyz); let cr = b.w * l.scale; if (!inside(cx, cr)) { return; }
  let m = mrowU(l.mesh + 2u); let i = atomicAdd(&n[0], 1u); if (i >= meshK.queue) { atomicStore(&n[N_FULL], 1u); return; } listB[i] = vec2u((k << 11u) | child, (m.y << 4u) | m.z);
}
@compute @workgroup_size(64) fn round(@builtin(global_invocation_id) id: vec3u, @builtin(num_workgroups) groups: vec3u) {
  let count = min(atomicLoad(&n[meshK.round]), meshK.queue) * 8u;
  for (var i = id.x; i < count; i += groups.x * 64u) {
    let q = listA[i >> 3u]; let many = q.y & 15u; if ((i & 7u) >= many) { continue; } let at = (q.y >> 4u) + (i & 7u) * 5u; let l = localOf(q.x >> 11u, q.x & 2047u);
    let eu = mrowU(at + 2u); let e = asF(eu); let lod = mrow(at + 1u); if (!shown(l, e.x, lod)) { continue; } let kids = eu.w; let b = mrow(at); let x = outward(l, b.xyz); let r = b.w * l.scale; if (!inside(x, r)) { continue; }
    if (kids != 0u) { let j = atomicAdd(&n[meshK.round + 1u], 1u); if (j >= meshK.queue) { atomicStore(&n[N_FULL], 1u); continue; } listB[j] = vec2u(q.x, (eu.z << 4u) | kids); continue; }
    if (shown(l, e.y, mrow(at + 3u))) { continue; }
    let cone = mrow(at + 4u); if (meshK.cone > 0.5 && cone.w <= 1.0) { let v = b.xyz - l.cam; if (dot(v, cone.xyz) >= cone.w * length(v) + b.w) { continue; } }
    let cluster = eu.z; if (meshK.hide > 0.5 && hidden(x, r, true)) { let j = atomicAdd(&n[N_HID], 1u); if (j < meshK.most) { again[j] = vec4u(q.x, cluster, at, 0u); continue; } }
    slotOut(l, cluster);
  }
}
/** what last frame's depth hid, asked again of this frame's */
@compute @workgroup_size(64) fn second(@builtin(global_invocation_id) id: vec3u, @builtin(num_workgroups) groups: vec3u) {
  let count = min(atomicLoad(&n[N_HID]), meshK.most); for (var i = id.x; i < count; i += groups.x * 64u) { let q = again[i]; let l = localOf(q.x >> 11u, q.x & 2047u); let b = mrow(q.z); if (hidden(outward(l, b.xyz), b.w * l.scale, false)) { continue; } slotOut(l, q.y); }
}
