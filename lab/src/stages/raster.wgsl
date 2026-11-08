// WHAT THE CAMERA SEES, as numbers only: which piece or leaf each pixel shows. Reversed depth. Plain boxes are their own faces; what is curved, cut by planes or
// cut by the section is its box, with the true surface found in the fragment.
struct V { @builtin(position) p: vec4f, @location(0) @interpolate(flat) id: u32, @location(1) w: vec3f };
fn clipOf(x: vec3f) -> vec4f { let v = x - frame.camPos.xyz; let z = dot(v, frame.camFwd.xyz); let r = frame.camRight.xyz; let u = frame.camUp.xyz; let j = vec2f(-2.0, 2.0) * frame.jitter.xy / frame.size.xy; return vec4f(dot(v, r) / dot(r, r) + j.x * z, dot(v, u) / dot(u, u) + j.y * z, 0.25, z); }
fn corner(vi: u32) -> vec3f {
  let f = vi / 6u; let k = vi % 6u; let a = f >> 1u; let s = select(-1.0, 1.0, (f & 1u) == 1u); let uv = array<vec2f, 6>(vec2f(-1.0, -1.0), vec2f(1.0, -1.0), vec2f(1.0, 1.0), vec2f(-1.0, -1.0), vec2f(1.0, 1.0), vec2f(-1.0, 1.0))[k];
  if (a == 0u) { return vec3f(s, uv); } if (a == 1u) { return vec3f(uv.x, s, uv.y); } return vec3f(uv, s);
}
fn fancy(k: u32) -> bool { let p = &pieces[k]; if (((*p).flags & (QUAD | 15u)) != 0u) { return true; } if (frame.misc.y < 1e8 && ((*p).flags & CUT) != 0u) { return topOf(k) > frame.misc.y; } return false; }
fn reachY(k: u32) -> f32 { let p = &pieces[k]; return abs((*p).r0.y) * (*p).half.x + abs((*p).r1.y) * (*p).half.y + abs((*p).r2.y) * (*p).half.z; }
fn midY(k: u32) -> f32 { let p = &pieces[k]; return -((*p).r0.y * (*p).r0.w + (*p).r1.y * (*p).r1.w + (*p).r2.y * (*p).r2.w); }
fn topOf(k: u32) -> f32 { return midY(k) + reachY(k); }
fn boxVertex(vi: u32, k: u32, wantFancy: bool, wantClear: bool) -> V {
  var o: V; o.id = k; o.p = vec4f(0.0, 0.0, -1.0, 1.0); o.w = vec3f(0.0); let p = &pieces[k]; let fl = (*p).flags;
  if (((fl & (GLASS | WATER)) != 0u) != wantClear || fancy(k) != wantFancy) { return o; }
  if (frame.misc.y < 1e8 && (fl & CUT) != 0u && midY(k) - reachY(k) > frame.misc.y) { return o; }
  let l = corner(vi) * (*p).half - vec3f((*p).r0.w, (*p).r1.w, (*p).r2.w); o.w = (*p).r0.xyz * l.x + (*p).r1.xyz * l.y + (*p).r2.xyz * l.z; o.p = clipOf(o.w); return o;
}
@vertex fn vsBox(@builtin(vertex_index) vi: u32, @builtin(instance_index) k: u32) -> V { return boxVertex(vi, k, false, false); }
@vertex fn vsFancy(@builtin(vertex_index) vi: u32, @builtin(instance_index) k: u32) -> V { return boxVertex(vi, k, true, false); }
@vertex fn vsClear(@builtin(vertex_index) vi: u32, @builtin(instance_index) k: u32) -> V { return boxVertex(vi, k, false, true); }
@vertex fn vsClearCut(@builtin(vertex_index) vi: u32, @builtin(instance_index) k: u32) -> V { return boxVertex(vi, k, true, true); }
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
  let q = array<vec2f, 6>(vec2f(-1.0, 0.0), vec2f(-0.42, -1.0), vec2f(-0.42, 1.0), vec2f(0.38, -1.0), vec2f(0.38, 1.0), vec2f(1.0, 0.0))[array<u32, 12>(0u, 1u, 2u, 1u, 3u, 2u, 2u, 3u, 4u, 3u, 5u, 4u)[vi]];
  let w = select(0.46, 0.06, b.w == 1u) * (1.0 - q.x * q.x) * (1.0 + 0.35 * q.x); let l = a.xyz + t * (q.x * a.w) + s * (q.y * w * a.w);
  // local → world: the rows are the inverse, scaled
  let k = dot((*I).r0.xyz, (*I).r0.xyz); let d = l - vec3f((*I).r0.w, (*I).r1.w, (*I).r2.w); o.w = ((*I).r0.xyz * d.x + (*I).r1.xyz * d.y + (*I).r2.xyz * d.z) / k; return o;
}
@vertex fn vsLeaf(@builtin(vertex_index) vi: u32, @builtin(instance_index) ii: u32) -> V { var v = leafVertex(vi % 12u, vi / 12u, lists[ii]); if (v.w.y < frame.misc.y) { v.p = clipOf(v.w); } return v; }
@fragment fn fsLeaf(v: V) -> @location(0) u32 { return v.id; }
/** the leaves again, as the sun sees them */
@vertex fn vsSun(@builtin(vertex_index) vi: u32, @builtin(instance_index) ii: u32) -> @builtin(position) vec4f { let v = leafVertex(vi % 12u, vi / 12u, lists[ii]); let q = sunMap(v.w); return vec4f(q.xy * 2.0 - 1.0, q.z, 1.0); }
