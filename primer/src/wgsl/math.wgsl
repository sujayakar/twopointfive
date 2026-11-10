const PI = 3.14159265;
const NOTHING = 0xffffffffu;

/// How bright a color looks: one number for choosing and comparing.
fn brightness(c: vec3f) -> f32 {
  return dot(c, vec3f(0.2126, 0.7152, 0.0722));
}

/// Two directions square to `n` and to each other (Duff and others, 2017).
fn frame_around(n: vec3f) -> mat3x3f {
  let s = select(-1.0, 1.0, n.z >= 0.0);
  let a = -1.0 / (s + n.z);
  let b = n.x * n.y * a;
  let tangent = vec3f(1.0 + s * n.x * n.x * a, s * b, -s * n.x);
  let bitangent = vec3f(b, s + n.y * n.y * a, -n.y);
  return mat3x3f(tangent, bitangent, n);
}

fn turn_by(q: vec4f, v: vec3f) -> vec3f {
  return v + 2.0 * cross(q.xyz, cross(q.xyz, v) + q.w * v);
}

fn turn_back(q: vec4f, v: vec3f) -> vec3f {
  return turn_by(vec4f(-q.xyz, q.w), v);
}

/// Two ways of finding one answer, with chances `mine` and `other`: how much mine counts.
/// The two weights add to 1, so nothing is counted twice (the balance heuristic).
fn balance(mine: f32, other: f32) -> f32 {
  if (mine <= 0.0) { return 0.0; }
  return mine / (mine + other);
}

fn is_bad(x: f32) -> bool {
  // by its bits: a compiler may assume that x == x
  return (bitcast<u32>(x) & 0x7f800000u) == 0x7f800000u;
}
