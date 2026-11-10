// WHICH LEVEL OF EACH SHAPE IS DRAWN. One thread a thing: out of view, over the section or hidden, it goes; else the coarsest level that is not seen to be one, all its clusters.
struct FineK { tau: f32, many: u32, most: u32, hide: f32, hzb: vec4f };
const N_SLOTS = 0u; const N_TRIS = 1u; const N_HID = 2u; const N_SAID = 3u; const N_ISAID = 4u; const N_FULL = 5u; const N_IX = 6u; const N_SEEN = 7u;
fn inside(x: vec3f, r: f32) -> bool {
  let v = x - frame.camPos.xyz; let f = dot(v, frame.camFwd.xyz); if (f < 0.25 - r) { return false; } let R = frame.camRight.xyz; let U = frame.camUp.xyz; let tx = length(R); let ty = length(U);
  return f * tx - abs(dot(v, R)) / tx > -r * sqrt(1.0 + tx * tx) && f * ty - abs(dot(v, U)) / ty > -r * sqrt(1.0 + ty * ty);
}
fn hidden(x: vec3f, r: f32, prev: bool) -> bool {
  let P = select(frame.camPos.xyz, frame.prevPos.xyz, prev); let F = select(frame.camFwd.xyz, frame.prevFwd.xyz, prev); let R = select(frame.camRight.xyz, frame.prevRight.xyz, prev); let U = select(frame.camUp.xyz, frame.prevUp.xyz, prev);
  let v = x - P; let z = dot(v, F); if (z - r < 0.3) { return false; } let near = z - r; let a = vec2f(dot(v, R) / dot(R, R), -dot(v, U) / dot(U, U)) / z * 0.5 + 0.5; let e = r / near * vec2f(0.5 / length(R), 0.5 / length(U)) + 1.5 / fineK.hzb.xy; let lo = clamp(a - e, vec2f(0.0), vec2f(1.0)); let hi = clamp(a + e, vec2f(0.0), vec2f(1.0)); if (any(lo >= hi)) { return false; }
  let px = (hi - lo) * fineK.hzb.xy; let lod = u32(clamp(ceil(log2(max(max(px.x, px.y), 1.0))), 0.0, fineK.hzb.z - 1.0)); let top = vec2i(fineK.hzb.xy) - 1; let dim = max(vec2i(fineK.hzb.xy) >> vec2u(lod), vec2i(1)); let i0 = min(clamp(vec2i(floor(lo * fineK.hzb.xy)), vec2i(0), top) >> vec2u(lod), dim - 1); let i1 = min(clamp(vec2i(floor(hi * fineK.hzb.xy)), vec2i(0), top) >> vec2u(lod), dim - 1); if (any(i1 - i0 > vec2i(2))) { return false; }
  var far = 1e9; for (var y = i0.y; y <= i1.y; y++) { for (var xx = i0.x; xx <= i1.x; xx++) { far = min(far, textureLoad(hzb, vec2i(xx, y), lod).x); } } return 0.25 / near < far;
}
fn put(i: u32, a: FineAt, r: f32) {
  let h = fine[a.shape]; let kind = h.y & 15u; let made = fF(a.shape + 2u).w; let grow = select(1.0, max3(a.half) / made, (h.y & 16u) != 0u); let d = max(distance(frame.camPos.xyz, a.at) - r, 0.05); let dice = f32(pcg(i * 2654435761u + frame.counts.z) >> 8u) * (1.0 / 16777216.0) - 0.5; let may = exp2(dice * fineK.hzb.w) * fineK.tau * max(bitcast<f32>(atomicLoad(&n[31])), 1.0) * (2.0 * length(frame.camUp.xyz) / frame.size.y) * d / grow;
  var l = h.x - 1u; loop { if (l == 0u || fF(a.shape + 4u + l).x <= may) { break; } l -= 1u; } let lv = fine[a.shape + 4u + l]; let s = atomicAdd(&n[N_SLOTS], lv.z); if (s + lv.z > fineK.most) { atomicStore(&n[N_FULL], 1u); return; } atomicAdd(&n[N_TRIS], lv.w); atomicAdd(&n[N_SEEN], 1u);
  for (var c = 0u; c < lv.z; c++) { let row = a.shape + h.z + lv.y + c; slots[s + c] = vec4u(i, row, a.shape, fine[row].z); }
}
fn ballOf(a: FineAt) -> f32 { let h = fine[a.shape]; let kind = h.y & 15u; if (kind == 2u) { return 1.5; } if (kind == 1u) { return length(a.half) * 1.02 + 0.01; } return fF(a.shape + 3u).x; }
@compute @workgroup_size(64) fn first(@builtin(global_invocation_id) id: vec3u) {
  let i = id.x; if (i >= fineK.many) { return; } let a = fineAt(i); if ((a.flags & FI_GONE) != 0u) { return; } let r = ballOf(a); if (a.at.y < -60.0 || !inside(a.at, r)) { return; }
  /* (what the section cuts through is drawn as its box, which has an end to show; what lies on it is cut a pixel at a time) */
  if ((a.flags & FI_CUT) != 0u && frame.misc.y < 1e8) { let e = select(abs(a.x.y) * a.half.x + abs(a.y.y) * a.half.y + abs(a.z.y) * a.half.z, 1.3, (a.flags & FI_BODY) != 0u); if (a.at.y - e > frame.misc.y || ((a.flags & (FI_SELF | FI_BODY)) == FI_SELF && a.at.y + e > frame.misc.y)) { return; } }
  if ((a.flags & FI_OPEN) != 0u && frame.misc.y < OPEN_FLOOR) { return; }
  if (fineK.hide > 0.5 && hidden(a.at, r, true)) { let j = atomicAdd(&n[N_HID], 1u); again[j] = i; return; } put(i, a, r);
}
@compute @workgroup_size(64) fn second(@builtin(global_invocation_id) id: vec3u) { let j = id.x; if (j >= atomicLoad(&n[N_HID])) { return; } let i = again[j]; let a = fineAt(i); let r = ballOf(a); if (hidden(a.at, r, false)) { return; } put(i, a, r); }
