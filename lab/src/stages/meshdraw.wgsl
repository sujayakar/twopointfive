// The picked clusters, drawn. Two ways: every corner fetched for itself (pull), or corners shared, by indices written each frame (share): a triangle's first corner belongs to no other, and tells it.
struct V { @builtin(position) p: vec4f, @location(0) @interpolate(flat, first) id: u32 };
fn clipOf(x: vec3f) -> vec4f { let v = x - frame.camPos.xyz; let z = dot(v, frame.camFwd.xyz); let r = frame.camRight.xyz; let u = frame.camUp.xyz; let j = vec2f(-2.0, 2.0) * frame.jitter.xy / frame.size.xy; return vec4f(dot(v, r) / dot(r, r) + j.x * z, dot(v, u) / dot(u, u) + j.y * z, 0.25, z); }
fn place(slot: u32, row: u32) -> vec3f { let v = mrowU(row); let a = slots[slot * 4u]; let b = slots[slot * 4u + 1u]; let c = slots[slot * 4u + 2u]; return a.xyz * f32(v.x & 0xffffu) + b.xyz * f32(v.x >> 16u) + c.xyz * f32(v.y & 0xffffu) + vec3f(a.w, b.w, c.w); }
@vertex fn pull(@builtin(vertex_index) at: u32) -> V {
  var o: V; let slot = at / 384u; let vi = at % 384u; let r = bitcast<vec4u>(slots[slot * 4u + 3u]); let t = vi / 3u; if (t >= (r.w & 255u)) { o.p = vec4f(0.0, 0.0, -1.0, 1.0); return o; }
  let ix = mrowU(r.y + (t >> 2u))[t & 3u]; o.id = 0x40000000u | (slot << 7u) | t; o.p = clipOf(place(slot, r.x + ((ix >> (8u * (vi % 3u))) & 255u))); return o;
}
@vertex fn share(@builtin(vertex_index) vi: u32) -> V { var o: V; let slot = vi >> 8u; o.id = 0x40000000u | (slot << 7u) | (vi & 127u); o.p = clipOf(place(slot, bitcast<u32>(slots[slot * 4u + 3u].x) + (vi & 255u))); return o; }
@fragment fn put(v: V) -> @location(0) u32 { return v.id; }
