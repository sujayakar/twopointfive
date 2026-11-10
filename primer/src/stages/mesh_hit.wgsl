// The triangle's number, made into a place: where the pixel's ray meets that triangle.

fn run(pixel: vec2u) {
  var out: MeshHit;
  out.distance = -1.0;
  let ids = textureLoad(mesh_ids, pixel, 0);
  if (ids.x == 0u) { put_mesh_hit(pixel, out); return; }
  let m = mesh_instances[ids.x - 1u];
  var p: array<vec3f, 3>;
  var n: array<vec3f, 3>;
  for (var k = 0u; k < 3u; k++) {
    let v = mesh_vertices[mesh_corners[ids.y * 3u + k].point];
    p[k] = m.position + turn_by(m.turn, v.position * m.scale);
    n[k] = turn_by(m.turn, v.normal);
  }
  let toward = pixel_toward(pixel);
  let e1 = p[1] - p[0];
  let e2 = p[2] - p[0];
  let pv = cross(toward, e2);
  let det = dot(e1, pv);
  let s = frame.eye - p[0];
  let u = clamp(dot(s, pv) / det, 0.0, 1.0);
  let q = cross(s, e1);
  let v = clamp(dot(toward, q) / det, 0.0, 1.0 - u);
  out.distance = dot(e2, q) / det;
  if (abs(det) < 1e-14 || out.distance <= 0.0) { out.distance = distance(frame.eye, (p[0] + p[1] + p[2]) / 3.0); }
  out.normal = normalize(n[0] * (1.0 - u - v) + n[1] * u + n[2] * v);
  out.material = m.material;
  out.instance = ids.x - 1u;
  out.triangle = ids.y;
  out.cluster = ids.z - 1u;
  let c = mesh_clusters[out.cluster];
  out.level = f32(c.level);
  out.error = c.error * m.scale / (out.distance * width_of_pixel());
  put_mesh_hit(pixel, out);
}
