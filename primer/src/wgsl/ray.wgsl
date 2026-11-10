// A RAY MEETS THE WORLD. The world is pieces. A piece is a box, cut by any number of planes and by at
// most one round shape (a ball, a cylinder, a cone). A ray's stay inside each of these is an interval;
// the piece is where they overlap.

// A ray has a width: how large an area its answer stands for. A piece may say from which width on, and until
// which, it is there: an object's levels are pieces with ranges that follow each other.

struct Ray {
  origin: vec3f,
  toward: vec3f,
  least: f32,    // from how far off things count
  most: f32,     // and until how far
  width: f32,    // meters, where it starts
  spread: f32,   // meters a meter: how fast it widens
  at_least: f32, // forced: never thinner than this
  by_eye: bool,  // the eye's own ray: what is drawn as a mesh, it does not meet here
  leaves: u32,   // does it meet leaves?
}
const WITH_LEAVES = 0u;
const NO_LEAVES = 1u;
const ONLY_LEAVES = 2u;

/// Half an octave more or less, by one die a ray, so that no seam shows where one level takes over from another.
fn octave(toward: vec3f) -> f32 {
  let f = select(frame.number, 0u, knobs.dice_frozen != 0u);
  let h = hash2(bitcast<u32>(toward.x), hash2(bitcast<u32>(toward.y), hash2(bitcast<u32>(toward.z), f)));
  return exp2((unit(h) - 0.5) * knobs.level_blend);
}

/// The finest there is, however far.
fn fine_ray(origin: vec3f, toward: vec3f, least: f32, most: f32) -> Ray {
  return Ray(origin, toward, least, most, 0.0, 0.0, 0.0, false, WITH_LEAVES);
}
fn eye_ray(origin: vec3f, toward: vec3f) -> Ray {
  let k = octave(toward);
  return Ray(origin, toward, 0.0, 1e4, 0.0, width_of_pixel() * k, knobs.forced_width * k, true, WITH_LEAVES);
}
fn forced_for_all(k: f32) -> f32 { return select(0.0, knobs.forced_width * k, knobs.forced_all != 0u); }
fn wide_cone(origin: vec3f, toward: vec3f) -> Ray {
  let k = octave(toward);
  return Ray(origin, toward, 1e-4, 1e4, 0.0, knobs.spread_wide * k, forced_for_all(k), false, WITH_LEAVES);
}
/// As wide as the mirror is rough.
fn narrow_cone(origin: vec3f, toward: vec3f, roughness: f32) -> Ray {
  let k = octave(toward);
  return Ray(origin, toward, 1e-4, 1e4, 0.0, (width_of_pixel() + roughness * roughness * knobs.spread_wide) * k, forced_for_all(k), false, WITH_LEAVES);
}
/// As wide as the lamp looks from here, where the ray gets to it; a point where it starts.
fn shadow_cone(origin: vec3f, toward: vec3f, most: f32, size: f32) -> Ray {
  let k = octave(toward);
  return Ray(origin, toward, 1e-4, most, 0.0, knobs.spread_shadow * size / max(most, 0.01) * k, forced_for_all(k), false, WITH_LEAVES);
}
fn width_at(ray: Ray, distance: f32) -> f32 { return max(ray.width + ray.spread * distance, ray.at_least); }

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

/// What only stands in for a mesh is met by no eye's ray, and by others only if they are to meet pieces.
fn takes_part(p: Piece, ray: Ray) -> bool {
  if (p.form == FORM_LEAF) { return ray.leaves != NO_LEAVES; }
  if (ray.leaves == ONLY_LEAVES) { return false; }
  if (p.form == FORM_SOLID) { return true; }
  let by_triangles = knobs.mesh_trace != MESH_TRACE_PIECES;
  if (p.form == FORM_STAND_IN) { return !ray.by_eye && !by_triangles; }
  return select(by_triangles, knobs.mesh_eye == MESH_EYE_RAYS, ray.by_eye);
}

/// A piece that covers only part of what it stands for lets the rest of the rays through, by a die.
fn is_missed(cover: f32, toward: vec3f, piece: u32) -> bool {
  let f = select(frame.number, 0u, knobs.dice_frozen != 0u);
  return unit(hash2(bitcast<u32>(toward.x), hash2(bitcast<u32>(toward.z), hash2(piece, f)))) >= cover;
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
      let p = pieces[i];
      if (!takes_part(p, ray)) { continue; }
      //#triangles
      if (p.shape == SHAPE_MESH) {
        // the tree is in the mesh's own size: the ray is brought to it, and distances stay the ray's
        let o = turn_back(p.turn, ray.origin - p.center) / p.mesh_scale;
        let d = turn_back(p.turn, ray.toward) / p.mesh_scale;
        let t = nearest_triangle(p.first_plane, o, d, ray.least, (*best).distance);
        (*best).steps += t.steps;
        if (t.distance < (*best).distance) {
          (*best).distance = t.distance;
          (*best).piece = i;
          // a smooth normal may look away from the ray that found its triangle
          let n = turn_by(p.turn, t.normal);
          let away = dot(n, ray.toward);
          (*best).normal = normalize(n - ray.toward * max(0.0, away + 0.02));
        }
        continue;
      }
      //#end
      var stay = stay_in_piece(p, ray.origin, ray.toward);
      if (stay.enter > stay.leave || stay.enter >= (*best).distance) { continue; }
      if (stay.enter < ray.least) {
        // the ray starts inside this piece: a surface found finely may lie under a coarser one's skin
        if (knobs.seam != SEAM_SOLID || stay.leave < ray.least + 2e-3) { continue; }
        stay.enter = ray.least;
        stay.normal = -ray.toward;
      }
      let width = width_at(ray, stay.enter);
      if (width < p.wide_from || width >= p.wide_until) { continue; }
      if (p.cover < 1.0 && is_missed(p.cover, ray.toward, i)) { continue; }
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
  var hair = knobs.start_margin;
  if (knobs.seam == SEAM_MARGIN) { hair += knobs.seam_margin; }
  if (knobs.break_no_margin != 0u) { hair = 0.0; }
  return position + normal * (side * hair);
}

//#triangles
// A MESH, TO A RAY: a tree of boxes over its triangles, in the mesh's own frame. All of it is whole numbers in one
// buffer; what is a fraction is read as one by its bits.

struct Triangle { distance: f32, normal: vec3f, steps: u32 }
fn mesh_f(i: u32) -> f32 { return bitcast<f32>(mesh_data[i]); }
fn mesh_v(i: u32) -> vec3f { return vec3f(mesh_f(i), mesh_f(i + 1u), mesh_f(i + 2u)); }

const NODE_WORDS = 8u;
const TRIANGLE_WORDS = 18u; // three corners, three normals

fn nearest_triangle(root: u32, origin: vec3f, toward: vec3f, least: f32, most: f32) -> Triangle {
  var best = Triangle(most, vec3f(0.0), 0u);
  let safe = select(toward, vec3f(1e-12), abs(toward) < vec3f(1e-12));
  let inverse = 1.0 / safe;
  var stack: array<u32, 28>;
  var height = 1u;
  stack[0] = root;
  while (height > 0u) {
    height--;
    let n = stack[height] * NODE_WORDS;
    best.steps += 1u;
    let a = (mesh_v(n) - origin) * inverse;
    let b = (mesh_v(n + 4u) - origin) * inverse;
    let near = min(a, b);
    let far = max(a, b);
    let enter = max(near.x, max(near.y, near.z));
    let leave = min(far.x, min(far.y, far.z));
    if (enter > leave || leave < least || enter > best.distance) { continue; }
    let first = mesh_data[n + 3u];
    let count = mesh_data[n + 7u];
    if (count == 0u) {
      stack[height] = first;
      stack[height + 1u] = first + 1u;
      height += 2u;
      continue;
    }
    for (var k = 0u; k < count; k++) {
      let at = first + k * TRIANGLE_WORDS;
      let p0 = mesh_v(at);
      let e1 = mesh_v(at + 3u) - p0;
      let e2 = mesh_v(at + 6u) - p0;
      // Moeller and Trumbore
      let pv = cross(toward, e2);
      let det = dot(e1, pv);
      if (abs(det) < 1e-12) { continue; }
      let s = origin - p0;
      let u = dot(s, pv) / det;
      let q = cross(s, e1);
      let v = dot(toward, q) / det;
      if (u < 0.0 || v < 0.0 || u + v > 1.0) { continue; }
      let t = dot(e2, q) / det;
      if (t < least || t >= best.distance) { continue; }
      if (det < 0.0) { continue; } // the back of a triangle: a ray that starts under a coarser skin gets out
      best.distance = t;
      best.normal = normalize(mesh_v(at + 9u) * (1.0 - u - v) + mesh_v(at + 12u) * u + mesh_v(at + 15u) * v);
    }
  }
  return best;
}
//#end
