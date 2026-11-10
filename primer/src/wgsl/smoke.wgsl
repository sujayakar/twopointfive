// SMOKE. A volume is a box of it. For each meter and color it absorbs so much, scatters so much and sends out so
// much; what is left of a ray after a stretch is exp(−(absorb + scatter) · stretch).

struct Smoke { thick: vec3f, scatter: vec3f, emit: vec3f, g: f32 }

fn lattice(c: vec3i) -> f32 {
  return unit(hash2(bitcast<u32>(c.x), hash2(bitcast<u32>(c.y), bitcast<u32>(c.z)))) * 2.0 - 1.0;
}
/// −1 to 1, smooth, a meter a wave
fn noise_at(p: vec3f) -> f32 {
  let c = vec3i(floor(p));
  let f0 = p - floor(p);
  let f = f0 * f0 * (3.0 - 2.0 * f0);
  let x0 = mix(lattice(c), lattice(c + vec3i(1, 0, 0)), f.x);
  let x1 = mix(lattice(c + vec3i(0, 1, 0)), lattice(c + vec3i(1, 1, 0)), f.x);
  let x2 = mix(lattice(c + vec3i(0, 0, 1)), lattice(c + vec3i(1, 0, 1)), f.x);
  let x3 = mix(lattice(c + vec3i(0, 1, 1)), lattice(c + vec3i(1, 1, 1)), f.x);
  return mix(mix(x0, x1, f.y), mix(x2, x3, f.y), f.z);
}

/// How much of a volume there is at a place, of 1, seen `width` wide: waves shorter than that are left out.
fn density_at(v: Volume, p: vec3f, width: f32) -> f32 {
  let q = abs(p - v.center) / v.half_size;
  let inside = 1.0 - max(q.x, max(q.y, q.z));
  if (inside <= 0.0) { return 0.0; }
  let edge = smoothstep(0.0, max(v.soft, 1e-3), inside);
  if (v.contrast <= 0.0) { return edge; }
  var n = 0.0;
  var gone = 0.0;
  var wave = v.wave;
  var strength = 0.6;
  for (var o = 0u; o < 4u; o++) {
    let keep = 1.0 - smoothstep(0.4, 1.0, width / wave);
    if (keep > 0.0) { n += keep * strength * noise_at((p - v.drift * frame.seconds) / wave + f32(o) * 7.3); }
    gone += 1.0 - keep;
    wave *= 0.5;
    strength *= 0.6;
  }
  let d = max(0.0, 1.0 + v.contrast * n) * edge;
  // the mean of what gets through is not what gets through the mean: thick next to thin lets more through than
  // all of it alike, so what stands for many is a little thinner
  if (knobs.break_mean_density != 0u) { return d; }
  let at = clamp(gone, 0.0, 4.0);
  let k = min(u32(at), 3u);
  let clumps = array<f32, 5>(1.0, v.clump.x, v.clump.y, v.clump.z, v.clump.w);
  return d * mix(clumps[k], clumps[k + 1u], at - f32(k));
}

fn smoke_at(p: vec3f, width: f32) -> Smoke {
  var s: Smoke;
  var weight = 0.0;
  for (var i = 0u; i < world.volume_count; i++) {
    let v = volumes[i];
    let d = density_at(v, p, width);
    if (d <= 0.0) { continue; }
    s.thick += (v.absorb + v.scatter) * d;
    s.scatter += v.scatter * d;
    s.emit += v.emit * d;
    s.g += v.g * d * brightness(v.scatter);
    weight += d * brightness(v.scatter);
  }
  if (weight > 0.0) { s.g /= weight; }
  return s;
}

/// The part of a ray that may have smoke on it.
fn smoke_span(origin: vec3f, toward: vec3f, most: f32) -> vec2f {
  var enter = 1e30;
  var leave = -1e30;
  for (var i = 0u; i < world.volume_count; i++) {
    let v = volumes[i];
    let stay = stay_in_volume(origin - v.center, toward, v.half_size);
    if (stay.x >= stay.y || stay.y <= 0.0 || stay.x >= most) { continue; }
    enter = min(enter, max(stay.x, 0.0));
    leave = max(leave, min(stay.y, most));
  }
  return vec2f(enter, leave);
}
fn stay_in_volume(origin: vec3f, toward: vec3f, half_size: vec3f) -> vec2f {
  let d = select(toward, vec3f(1e-12), abs(toward) < vec3f(1e-12));
  let a = (-half_size - origin) / d;
  let b = (half_size - origin) / d;
  let near = min(a, b);
  let far = max(a, b);
  return vec2f(max(near.x, max(near.y, near.z)), min(far.x, min(far.y, far.z)));
}

struct Passage { passes: vec3f, added: vec3f }

/// What gets through a stretch of a ray, and what the smoke itself sends along it. (Light that it scatters is
/// another matter: see the air's own stages.)
/// For a ray that gathers light from all round (`gathers`), what is scattered off it may be left to count as let
/// through: about as much is scattered into it from elsewhere, and nobody here adds that.
fn smoke_between(origin: vec3f, toward: vec3f, most: f32, width: f32, spread: f32, steps: u32, die: f32, gathers: bool) -> Passage {
  var out = Passage(vec3f(1.0), vec3f(0.0));
  let span = smoke_span(origin, toward, most);
  if (span.x >= span.y) { return out; }
  let step = (span.y - span.x) / f32(steps);
  for (var i = 0u; i < steps; i++) {
    let t = span.x + (f32(i) + die) * step;
    let s = smoke_at(origin + toward * t, width + spread * t);
    var thick = s.thick;
    if (gathers && knobs.air_bounces == AIR_BOUNCES_ABSORBS) { thick -= s.scatter; }
    let here = exp(-thick * step);
    out.added += out.passes * s.emit * (1.0 - here) / max(thick, vec3f(1e-6));
    out.passes *= here;
  }
  return out;
}

struct Scattering { happens: bool, distance: f32, weight: vec3f, passes: vec3f, added: vec3f, g: f32 }

/// The same, and one place along it where light may have been turned our way: drawn by how much the smoke turns
/// there and how much of that gets here.
fn smoke_scatter(origin: vec3f, toward: vec3f, most: f32, spread: f32, steps: u32, die: f32, pick: f32) -> Scattering {
  var out = Scattering(false, 0.0, vec3f(0.0), vec3f(1.0), vec3f(0.0), 0.0);
  let span = smoke_span(origin, toward, most);
  if (span.x >= span.y) { return out; }
  let step = (span.y - span.x) / f32(steps);
  var total = 0.0;
  var u = pick;
  for (var i = 0u; i < steps; i++) {
    let t = span.x + (f32(i) + die) * step;
    let s = smoke_at(origin + toward * t, spread * t);
    let here = exp(-s.thick * step);
    let stopped = (1.0 - here) / max(s.thick, vec3f(1e-6));
    let turned = out.passes * s.scatter * stopped;
    out.added += out.passes * s.emit * stopped;
    out.passes *= here;
    let b = brightness(turned);
    if (b <= 0.0) { continue; }
    total += b;
    let take = b / total;
    if (u < take) { out.happens = true; out.distance = t; out.weight = turned / b; out.g = s.g; u = u / take; }
    else { u = (u - take) / (1.0 - take); }
  }
  out.weight *= total;
  return out;
}

/// How light is turned: forward for g near 1, every way alike for 0 (Henyey and Greenstein).
fn phase(cosine: f32, g: f32) -> f32 {
  let d = 1.0 + g * g - 2.0 * g * cosine;
  return (1.0 - g * g) / (4.0 * PI * d * sqrt(d));
}
fn phase_draw(forward: vec3f, g: f32, dice: vec2f) -> vec3f {
  var c = 1.0 - 2.0 * dice.x;
  if (abs(g) > 1e-3) {
    let k = (1.0 - g * g) / (1.0 - g + 2.0 * g * dice.x);
    c = (1.0 + g * g - k * k) / (2.0 * g);
  }
  let s = sqrt(max(0.0, 1.0 - c * c));
  return frame_around(forward) * vec3f(s * cos(6.2832 * dice.y), s * sin(6.2832 * dice.y), c);
}
