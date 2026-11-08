// WHAT THE CAMERA SEES, as numbers only: which piece or leaf each pixel shows. Reversed depth. Plain boxes are their own faces; what is curved, cut by planes or
// cut by the section is its box, with the true surface found in the fragment.
struct V { @builtin(position) p: vec4f, @location(0) @interpolate(flat) id: u32, @location(1) w: vec3f };
fn clipOf(x: vec3f) -> vec4f { let v = x - frame.camPos.xyz; let z = dot(v, frame.camFwd.xyz); let r = frame.camRight.xyz; let u = frame.camUp.xyz; let j = vec2f(-2.0, 2.0) * frame.jitter.xy / frame.size.xy; return vec4f(dot(v, r) / dot(r, r) + j.x * z, dot(v, u) / dot(u, u) + j.y * z, 0.25, z); }
/** a box's eight corners */
fn corner(vi: u32) -> vec3f { return vec3f(select(-1.0, 1.0, (vi & 1u) != 0u), select(-1.0, 1.0, (vi & 2u) != 0u), select(-1.0, 1.0, (vi & 4u) != 0u)); }
fn boxVertex(vi: u32, k: u32) -> V {
  var o: V; o.id = k; let p = &pieces[k]; let l = corner(vi) * (*p).half - vec3f((*p).r0.w, (*p).r1.w, (*p).r2.w); o.w = (*p).r0.xyz * l.x + (*p).r1.xyz * l.y + (*p).r2.xyz * l.z; o.p = clipOf(o.w); return o;
}
@vertex fn vsBox(@builtin(vertex_index) vi: u32, @builtin(instance_index) ii: u32) -> V { return boxVertex(vi, lists[ii]); }
@fragment fn fsBox(v: V) -> @location(0) u32 { return v.id + 1u; }
struct F { @location(0) id: u32, @builtin(frag_depth) z: f32 };
@fragment fn fsFancy(v: V) -> F {
  let rd = camRay((v.p.xy + frame.jitter.xy) / frame.size.xy); var o: F; o.id = v.id + 1u; o.z = 0.0;
  if (!meet(v.id, frame.camPos.xyz, rd, 1e-3, 4000.0, frame.misc.y)) { discard; } o.z = 0.25 / (hT * dot(rd, frame.camFwd.xyz)); return o;
}
// leaves: a pointed oval of four triangles
fn leafVertex(vi: u32, at: u32, inst: u32) -> V {
  var o: V; o.p = vec4f(0.0, 0.0, -1.0, 1.0); o.w = vec3f(0.0); let I = &insts[inst]; let leaf = (*I).leaf + at; o.id = 0x80000000u | (inst << 13u) | at;
  let a = extra[leaf * 2u]; let b = bitcast<vec4u>(extra[leaf * 2u + 1u]); let n = octUnpack(b.x); let t = octUnpack(b.y); let s = cross(n, t);
  let q = array<vec2f, 10>(vec2f(-1.0, 0.0), vec2f(-0.42, -1.0), vec2f(-0.42, 1.0), vec2f(0.38, -1.0), vec2f(0.38, 1.0), vec2f(1.0, 0.0), vec2f(-0.8, -0.8), vec2f(0.8, -0.8), vec2f(-0.8, 0.8), vec2f(0.8, 0.8))[vi];
  let w = leafWide(b.w) * select((1.0 - q.x * q.x) * (1.0 + 0.35 * q.x), 1.0, vi > 5u); let l = a.xyz + t * (q.x * a.w) + s * (q.y * w * a.w);
  // local → world: the rows are the inverse, scaled
  let k = dot((*I).r0.xyz, (*I).r0.xyz); let d = l - vec3f((*I).r0.w, (*I).r1.w, (*I).r2.w); o.w = ((*I).r0.xyz * d.x + (*I).r1.xyz * d.y + (*I).r2.xyz * d.z) / k; return o;
}
@vertex fn vsLeaf(@builtin(vertex_index) vi: u32, @builtin(instance_index) ii: u32) -> V { var v = leafVertex(vi % 6u, vi / 6u, lists[ii]); if (v.w.y < frame.misc.y || (insts[lists[ii]].tint & 0x1000000u) != 0u) { v.p = clipOf(v.w); } return v; }
@fragment fn fsLeaf(v: V) -> @location(0) u32 { return v.id; }
/** the leaves again, as the sun sees them: four corners each, of the same area */
@vertex fn vsSun(@builtin(vertex_index) vi: u32, @builtin(instance_index) ii: u32) -> @builtin(position) vec4f { let v = leafVertex(6u + (vi & 3u), vi >> 2u, lists[ii]); let q = sunMap(v.w); return vec4f(q.xy * 2.0 - 1.0, q.z, 1.0); }
