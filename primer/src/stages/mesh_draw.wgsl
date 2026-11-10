// Drawing what was picked: for the numbers only. Which instance, which triangle, which cluster; no color.

struct Out {
  @builtin(position) at: vec4f,
  @location(0) @interpolate(flat) ids: vec3u,
}

@vertex fn corner(@builtin(vertex_index) v: u32, @builtin(instance_index) i: u32) -> Out {
  var out: Out;
  let job = mesh_picked[i];
  let c = mesh_clusters[job.cluster];
  let triangle = v / 3u;
  if (triangle >= c.count) { out.at = vec4f(0.0, 0.0, 0.0, 1.0); return out; } // fewer than 128: drawn as nothing
  let m = mesh_instances[job.instance];
  let p = mesh_vertices[mesh_corners[(c.first + triangle) * 3u + v % 3u].point].position;
  let position = m.position + turn_by(m.turn, p * m.scale);
  let to = position - frame.eye;
  let z = dot(to, frame.forward);
  let x = dot(to, frame.right) / dot(frame.right, frame.right);
  let y = dot(to, frame.up) / dot(frame.up, frame.up);
  let shift = 2.0 * frame.jitter / vec2f(frame.render_size);
  out.at = vec4f(x - z * shift.x, y + z * shift.y, 0.01, z); // depth is near / z: 1 at the eye, 0 far away
  out.ids = vec3u(job.instance + 1u, c.first + triangle, job.cluster + 1u);
  return out;
}

@fragment fn ids(in: Out) -> @location(0) vec4u { return vec4u(in.ids, 0u); }
