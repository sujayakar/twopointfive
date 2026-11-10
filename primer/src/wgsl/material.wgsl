// MATERIALS. Two lobes on every surface: a matte one (the same every way) and a coat (a mirror, more or
// less rough). Three questions: how much goes from this direction to that one, draw a direction, and what
// was the chance of that direction.

struct Surface {
  position: vec3f,
  normal: vec3f,
  eye: vec3f,      // the way back to whoever looks
  matte: vec3f,
  through: vec3f,
  roughness: f32,
  metal: f32,
}

fn coat_f0(s: Surface) -> vec3f { return mix(vec3f(0.04), s.matte, s.metal); }
fn coat_alpha(s: Surface) -> f32 { return max(s.roughness * s.roughness, 1e-3); }

/// A and B of `sent back = f0 · A + B`, for light from everywhere (see world/coat.ts)
fn coat_table(roughness: f32, cosine: f32) -> vec2f {
  let at = vec2f(clamp(cosine, 0.0, 1.0) * 16.0 - 0.5, clamp(roughness, 0.0, 1.0) * 15.0);
  let low = clamp(vec2i(floor(at)), vec2i(0), vec2i(14));
  let f = clamp(at - vec2f(low), vec2f(0.0), vec2f(1.0));
  let i = u32(low.y * 16 + low.x);
  let a = mix(tables.coat[i].xy, tables.coat[i + 1u].xy, f.x);
  let b = mix(tables.coat[i + 16u].xy, tables.coat[i + 17u].xy, f.x);
  return mix(a, b, f.y);
}

/// Light that bounces between a rough coat's facets is lost by the formula; this puts it back.
fn coat_gain(s: Surface) -> vec3f {
  let ab = coat_table(s.roughness, dot(s.normal, s.eye));
  return 1.0 + coat_f0(s) * (1.0 / (ab.x + ab.y) - 1.0);
}

/// All that the coat sends to the eye, of light that is 1 everywhere.
fn coat_share(s: Surface) -> vec3f {
  let ab = coat_table(s.roughness, dot(s.normal, s.eye));
  return (coat_f0(s) * ab.x + ab.y) * coat_gain(s);
}

/// The matte lobe gets what the coat did not turn away; of something thin, some of it on the other side. So a white surface sends back everything:
/// share + (1 − share) · 1 = 1, and a white ball under an even sky vanishes.
fn matte_share(s: Surface) -> vec3f {
  let kept = select(1.0 - coat_share(s), vec3f(1.0), knobs.break_no_sharing != 0u);
  return (s.matte + s.through) * (1.0 - s.metal) * kept;
}
fn is_thin(s: Surface) -> bool { return any(s.through > vec3f(0.0)); }
/// Of what the matte lobe sends to the eye, how much is of light that lands on the eye's side.
fn front_of(s: Surface) -> vec3f {
  return select(vec3f(1.0), s.matte / max(s.matte + s.through, vec3f(1e-6)), is_thin(s));
}
/// The matte response to light from a direction, the cosine in it.
fn matte_response(s: Surface, light: vec3f) -> vec3f {
  let cosine = dot(s.normal, light);
  let front = front_of(s);
  return matte_share(s) * select(1.0 - front, front, cosine > 0.0) * (abs(cosine) / PI);
}

fn smith(cosine: f32, alpha: f32) -> f32 {
  let c2 = max(cosine * cosine, 1e-8);
  return (sqrt(1.0 + alpha * alpha * (1.0 - c2) / c2) - 1.0) * 0.5;
}
fn facets(cosine: f32, alpha: f32) -> f32 {
  let a2 = alpha * alpha;
  // (1 − c²) kept apart from c² a²: near a mirror's peak the difference would lose its digits
  let d = (1.0 - cosine) * (1.0 + cosine) + cosine * cosine * a2;
  return a2 / (PI * d * d);
}

/// The coat's response, the cosine on the light's side included.
fn coat_response(s: Surface, light: vec3f) -> vec3f {
  let cos_light = dot(s.normal, light);
  let cos_eye = dot(s.normal, s.eye);
  if (cos_light <= 0.0 || cos_eye <= 0.0) { return vec3f(0.0); }
  let h = normalize(light + s.eye);
  let a = coat_alpha(s);
  let f0 = coat_f0(s);
  let fresnel = f0 + (1.0 - f0) * pow(1.0 - clamp(dot(s.eye, h), 0.0, 1.0), 5.0);
  let seen = 1.0 / (1.0 + smith(cos_eye, a) + smith(cos_light, a));
  return fresnel * coat_gain(s) * (facets(dot(s.normal, h), a) * seen / (4.0 * cos_eye));
}

/// The chance, per unit of solid angle, that `coat_draw` gives this direction.
fn coat_chance(s: Surface, light: vec3f) -> f32 {
  let cos_eye = dot(s.normal, s.eye);
  if (dot(s.normal, light) <= 0.0 || cos_eye <= 0.0) { return 0.0; }
  let h = normalize(light + s.eye);
  let a = coat_alpha(s);
  return facets(dot(s.normal, h), a) / ((1.0 + smith(cos_eye, a)) * 4.0 * cos_eye);
}

/// A direction the coat sends light from, drawn among the facets the eye can see (Heitz 2018).
fn coat_draw(s: Surface, dice: vec2f) -> vec3f {
  let axes = frame_around(s.normal);
  let v = vec3f(dot(axes[0], s.eye), dot(axes[1], s.eye), max(dot(axes[2], s.eye), 1e-4));
  let a = coat_alpha(s);
  let vh = normalize(vec3f(a * v.x, a * v.y, v.z));
  let flat2 = vh.x * vh.x + vh.y * vh.y;
  var t1 = vec3f(1.0, 0.0, 0.0);
  if (flat2 > 1e-12) { t1 = vec3f(-vh.y, vh.x, 0.0) / sqrt(flat2); }
  let t2 = cross(vh, t1);
  let r = sqrt(dice.x);
  let phi = 2.0 * PI * dice.y;
  let p1 = r * cos(phi);
  let k = 0.5 * (1.0 + vh.z);
  let p2 = (1.0 - k) * sqrt(max(0.0, 1.0 - p1 * p1)) + k * r * sin(phi);
  let nh = t1 * p1 + t2 * p2 + vh * sqrt(max(0.0, 1.0 - p1 * p1 - p2 * p2));
  let h = normalize(vec3f(a * nh.x, a * nh.y, max(1e-6, nh.z)));
  return axes * reflect(-v, h);
}

/// response / chance, worked out together: what is left is the facets' shadowing and the Fresnel term
fn coat_weight(s: Surface, light: vec3f) -> vec3f {
  let chance = coat_chance(s, light);
  if (chance <= 0.0) { return vec3f(0.0); }
  return coat_response(s, light) / chance;
}

/// A direction over the half-sphere with a chance in proportion to its cosine: an even dot on a disc,
/// lifted to the sphere (Malley).
fn wide_draw(normal: vec3f, dice: vec2f) -> vec3f {
  let r = sqrt(dice.x);
  let phi = 2.0 * PI * dice.y;
  let local = vec3f(r * cos(phi), r * sin(phi), sqrt(max(0.0, 1.0 - dice.x)));
  return frame_around(normal) * local;
}
fn wide_chance(normal: vec3f, light: vec3f) -> f32 { return max(dot(normal, light), 0.0) / PI; }
