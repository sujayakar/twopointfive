// A RAY MEETS THE WORLD. The world is pieces. A piece is a box, cut by any number of planes and by at
// most one round shape (a ball, a cylinder, a cone). A ray's stay inside each of these is an interval;
// the piece is where they overlap.

struct Ray { origin: vec3f, toward: vec3f, least: f32, most: f32 }

struct Meeting {
  distance: f32,
  piece: u32,    // NOTHING: the ray left the world
  normal: vec3f, // of the surface itself, outward
  steps: u32,    // tree nodes looked at, to see what a ray costs
}

struct Stay { enter: f32, leave: f32, normal: vec3f }

fn no_stay() -> Stay { return Stay(1e30, -1e30, vec3f(0.0)); }

fn stay_in_box(origin: vec3f, toward: vec3f, half_size: vec3f) -> Stay {
  // a direction of exactly zero would make 0 / 0 on a slab's own plane
  let d = select(toward, vec3f(1e-12), abs(toward) < vec3f(1e-12));
  let a = (-half_size - origin) / d;
  let b = (half_size - origin) / d;
  let near = min(a, b);
  let far = max(a, b);
  let enter = max(near.x, max(near.y, near.z));
  let leave = min(far.x, min(far.y, far.z));
  var normal = vec3f(0.0, 0.0, -sign(d.z));
  if (near.x == enter) { normal = vec3f(-sign(d.x), 0.0, 0.0); }
  else if (near.y == enter) { normal = vec3f(0.0, -sign(d.y), 0.0); }
  return Stay(enter, leave, normal);
}

/// a t² + 2 b t + c = 0, solved from the point nearest the piece, so that far eyes lose no digits
fn roots(a: f32, b: f32, c: f32) -> vec2f {
  let disc = b * b - a * c;
  if (disc < 0.0) { return vec2f(1e30, -1e30); }
  let q = -(b + select(-1.0, 1.0, b >= 0.0) * sqrt(disc));
  let t0 = q / a;
  let t1 = select(c / q, t0, q == 0.0);
  return vec2f(min(t0, t1), max(t0, t1));
}

fn stay_in_round(shape: u32, origin: vec3f, toward: vec3f, half_size: vec3f) -> Stay {
  let shift = -dot(origin, toward); // to the point nearest the center
  let o = (origin + toward * shift) / half_size;
  let d = toward / half_size;
  if (shape == SHAPE_BALL) {
    let t = roots(dot(d, d), dot(o, d), dot(o, o) - 1.0);
    let n = (o + d * t.x) / half_size;
    return Stay(t.x + shift, t.y + shift, normalize(n));
  }
  if (shape == SHAPE_CYLINDER) {
    let t = roots(dot(d.xz, d.xz), dot(o.xz, d.xz), dot(o.xz, o.xz) - 1.0);
    let n = vec3f(o.x + d.x * t.x, 0.0, o.z + d.z * t.x) / half_size;
    return Stay(t.x + shift, t.y + shift, normalize(n));
  }
  return Stay(-1e30, 1e30, vec3f(0.0));
}

fn overlap(a: Stay, b: Stay) -> Stay {
  var out = a;
  if (b.enter > a.enter) { out.enter = b.enter; out.normal = b.normal; }
  out.leave = min(a.leave, b.leave);
  return out;
}

fn stay_in_piece(p: Piece, origin: vec3f, toward: vec3f) -> Stay {
  let o = turn_back(p.turn, origin - p.center);
  let d = turn_back(p.turn, toward);
  var stay = stay_in_box(o, d, p.half_size);
  if (stay.enter > stay.leave) { return no_stay(); }
  if (p.shape != SHAPE_BOX) {
    stay = overlap(stay, stay_in_round(p.shape, o, d, p.half_size));
  }
  for (var i = 0u; i < p.plane_count; i++) {
    let plane = planes[p.first_plane + i];
    let away = dot(plane.xyz, d);
    let gap = plane.w - dot(plane.xyz, o); // positive: inside
    if (abs(away) < 1e-9) {
      if (gap < 0.0) { return no_stay(); }
      continue;
    }
    let t = gap / away;
    if (away < 0.0) { stay = overlap(stay, Stay(t, 1e30, plane.xyz)); }
    else { stay = overlap(stay, Stay(-1e30, t, vec3f(0.0))); }
  }
  stay.normal = turn_by(p.turn, stay.normal);
  return stay;
}

fn stay_in_node(n: Node, origin: vec3f, inverse: vec3f) -> vec2f {
  let a = (n.low - origin) * inverse;
  let b = (n.high - origin) * inverse;
  let near = min(a, b);
  let far = max(a, b);
  return vec2f(max(near.x, max(near.y, near.z)), min(far.x, min(far.y, far.z)));
}

/// The first thing a ray meets.
fn nearest(ray: Ray) -> Meeting {
  var best = Meeting(ray.most, NOTHING, vec3f(0.0), 0u);
  walk(ray, false, &best);
  return best;
}

/// Is the way free? (the ray stops at the first thing found, whichever it is)
fn is_clear(ray: Ray) -> bool {
  var any = Meeting(ray.most, NOTHING, vec3f(0.0), 0u);
  walk(ray, true, &any);
  return any.piece == NOTHING;
}

fn walk(ray: Ray, stop_at_any: bool, best: ptr<function, Meeting>) {
  let safe = select(ray.toward, vec3f(1e-12), abs(ray.toward) < vec3f(1e-12));
  let inverse = 1.0 / safe;
  var stack: array<u32, 32>;
  var height = 0u;
  if (world.moving_pieces > 0u) { stack[height] = world.moving_root; height++; }
  if (world.still_pieces > 0u) { stack[height] = 0u; height++; }
  while (height > 0u) {
    height--;
    let node = nodes[stack[height]];
    (*best).steps += 1u;
    let span = stay_in_node(node, ray.origin, inverse);
    if (span.x > span.y || span.y < ray.least || span.x > (*best).distance) { continue; }
    if (node.count == 0u) {
      // the nearer child is looked at first: it may hide the other
      let a = stay_in_node(nodes[node.first], ray.origin, inverse).x;
      let b = stay_in_node(nodes[node.first + 1u], ray.origin, inverse).x;
      let swap = b < a;
      stack[height] = node.first + select(1u, 0u, swap);
      stack[height + 1u] = node.first + select(0u, 1u, swap);
      height += 2u;
      continue;
    }
    for (var i = node.first; i < node.first + node.count; i++) {
      let stay = stay_in_piece(pieces[i], ray.origin, ray.toward);
      if (stay.enter > stay.leave || stay.enter < ray.least || stay.enter >= (*best).distance) { continue; }
      (*best).distance = stay.enter;
      (*best).piece = i;
      (*best).normal = stay.normal;
      if (stop_at_any) { return; }
    }
  }
}

/// Where a ray that leaves a surface starts: a hair off it, on the side it goes to.
fn leave_surface(position: vec3f, normal: vec3f, toward: vec3f) -> vec3f {
  let side = select(-1.0, 1.0, dot(normal, toward) >= 0.0);
  let hair = select(knobs.start_margin, 0.0, knobs.break_no_margin != 0u);
  return position + normal * (side * hair);
}
