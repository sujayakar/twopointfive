// a brick of smoke as one cell: how much there is, and what it sends by how much
var<workgroup> part: array<vec4f, 64>;
@compute @workgroup_size(64) fn main(@builtin(workgroup_id) wg: vec3u, @builtin(local_invocation_index) li: u32) {
  let s = wg.x; let at = vec3i(i32(s & 15u), i32((s >> 4u) & 15u), i32(s >> 8u)) * 10 + 1; var a = vec4f(0.0);
  for (var k = li; k < 512u; k += 64u) { let v = textureLoad(smokeTex, at + vec3i(i32(k & 7u), i32((k >> 3u) & 7u), i32(k >> 6u)), 0); a += vec4f(select(v.rgb, vec3f(0.0), max3(v.rgb) > 24.0) * v.w, v.w); }
  part[li] = a; workgroupBarrier(); if (li != 0u) { return; } var t = vec4f(0.0); for (var k = 0u; k < 64u; k++) { t += part[k]; } mean[s] = finite4(vec4f(t.rgb / max(t.w, 1e-4), t.w / 512.0), 1e4);
}
