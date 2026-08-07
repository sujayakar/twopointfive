// G-buffer rasterization: instanced yawed boxes + GPU-skinned characters.
// Outputs: albedo (rgba8), world normal (rgb10a2, n*0.5+0.5), id (r32uint: boxIdx | owner<<20 | isChar<<28), depth.

struct GOut {
  @location(0) albedo: vec4f,
  @location(1) normal: vec4f,
  @location(2) id: u32,
};

// ---------------- boxes ----------------
@group(1) @binding(0) var<storage, read> visible: array<u32>;

struct BoxVOut {
  @builtin(position) pos: vec4f,
  @location(0) @interpolate(flat) albedo: vec3f,
  @location(1) @interpolate(flat) normal: vec3f,
  @location(2) @interpolate(flat) id: u32,
};

fn cubeCorner(vi: u32) -> vec3f {
  // 6 faces * 2 tris * 3 verts. Face order: +x -x +y -y +z -z
  var faces = array<vec3f, 6>(vec3f(1,0,0), vec3f(-1,0,0), vec3f(0,1,0), vec3f(0,-1,0), vec3f(0,0,1), vec3f(0,0,-1));
  let face = vi / 6u; let k = vi % 6u;
  var quad = array<vec2f, 6>(vec2f(-1,-1), vec2f(1,-1), vec2f(1,1), vec2f(-1,-1), vec2f(1,1), vec2f(-1,1));
  let q = quad[k];
  let n = faces[face];
  // build tangent basis so winding is CCW seen from outside
  var u: vec3f; var v: vec3f;
  if (face == 0u) { u = vec3f(0,0,-1); v = vec3f(0,1,0); }
  else if (face == 1u) { u = vec3f(0,0,1); v = vec3f(0,1,0); }
  else if (face == 2u) { u = vec3f(1,0,0); v = vec3f(0,0,-1); }
  else if (face == 3u) { u = vec3f(1,0,0); v = vec3f(0,0,1); }
  else if (face == 4u) { u = vec3f(1,0,0); v = vec3f(0,1,0); }
  else { u = vec3f(-1,0,0); v = vec3f(0,1,0); }
  return n + u * q.x + v * q.y;
}
fn cubeNormal(vi: u32) -> vec3f {
  var faces = array<vec3f, 6>(vec3f(1,0,0), vec3f(-1,0,0), vec3f(0,1,0), vec3f(0,-1,0), vec3f(0,0,1), vec3f(0,0,-1));
  return faces[vi / 6u];
}

@vertex fn vsBox(@builtin(vertex_index) vi: u32, @builtin(instance_index) ii: u32) -> BoxVOut {
  let bi = visible[ii];
  let b = boxGeo[bi];
  var c = b.c; var h = b.h;
  var cap = false;
  let face = vi / 6u;
  if ((b.flags & BOX_CUTAWAY) != 0u) {
    let bottom = c.y - h.y; let fullTop = c.y + h.y; let ch = boxCutHeight(bi);
    if (ch < bottom + 0.05) { h = vec3f(0.0); }             // entirely above the cut: collapse (nothing rasterized)
    let top = min(fullTop, max(bottom + 0.02, ch));
    if (top < fullTop - 0.001 && face == 2u) { cap = true; }
    h.y = min(h.y, (top - bottom) * 0.5); c.y = bottom + h.y;
  }
  // top faces flush under the (hidden) ceiling slab read as architectural section cuts
  if (face == 2u && c.y + h.y >= CEIL_Y - 0.03 && c.y - h.y < CEIL_Y - 0.5) { cap = true; }
  let q = boxRot(bi);                                       // = the box's yaw for ordinary boxes; BOX_ROT boxes (hand props, sparks, canisters) may carry a full 3D rotation
  let lc = cubeCorner(vi) * h;
  let ln = cubeNormal(vi);
  let wp = c + quatRotate(q, lc);
  var o: BoxVOut;
  o.pos = frame.viewProj * vec4f(wp, 1.0);
  o.normal = quatRotate(q, ln);
  o.albedo = boxAlbedo(bi);
  o.id = bi | (boxOwner(b) << 20u) | (select(0u, 1u, cap) << 29u);
  return o;
}

@fragment fn fsBox(v: BoxVOut) -> GOut {
  var o: GOut;
  o.albedo = vec4f(v.albedo, 1.0);
  o.normal = vec4f(normalize(v.normal) * 0.5 + 0.5, 1.0);
  o.id = v.id;
  return o;
}

// ---------------- skinned characters ----------------
struct CharInst { tint: vec3f, jointOffset: u32, tint2: vec3f, owner: u32 };
@group(1) @binding(1) var<storage, read> jointMats: array<mat4x4f>;
@group(1) @binding(2) var<storage, read> chars: array<CharInst>;

struct SkinVOut {
  @builtin(position) pos: vec4f,
  @location(0) normal: vec3f,
  @location(1) @interpolate(flat) albedo: vec3f,
  @location(2) @interpolate(flat) id: u32,
};

@vertex fn vsSkin(@location(0) position: vec3f, @location(1) normal: vec3f, @location(2) joints: vec4u, @location(3) weights: vec4f,
                  @builtin(instance_index) ii: u32, @location(4) matSel: f32) -> SkinVOut {
  let ch = chars[ii];
  let base = ch.jointOffset;
  let m = jointMats[base + joints.x] * weights.x + jointMats[base + joints.y] * weights.y + jointMats[base + joints.z] * weights.z + jointMats[base + joints.w] * weights.w;
  let wp = m * vec4f(position, 1.0);
  let wn = normalize((m * vec4f(normal, 0.0)).xyz);
  var o: SkinVOut;
  o.pos = frame.viewProj * vec4f(wp.xyz, 1.0);
  o.normal = wn;
  o.albedo = select(ch.tint, ch.tint2, matSel > 0.5);
  o.id = 0xFFFFFu | (ch.owner << 20u) | (1u << 28u);
  return o;
}

@fragment fn fsSkin(v: SkinVOut) -> GOut {
  var o: GOut;
  o.albedo = vec4f(v.albedo, 1.0);
  o.normal = vec4f(normalize(v.normal) * 0.5 + 0.5, 1.0);
  o.id = v.id;
  return o;
}
