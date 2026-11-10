// HOW VALUES ARE DRAWN. Every view ends in one of these. Out comes linear color, 0 to 1.

/// Light: through the exposure, then the curve. The only place where a sum stops being a sum.
fn show_light(c: vec3f) -> vec3f {
  let e = max(c, vec3f(0.0)) * exp2(knobs.exposure);
  if (knobs.tone == TONE_CLIP) { return min(e, vec3f(1.0)); }
  if (knobs.tone == TONE_STOPS) { return show_stops(brightness(e)); }
  // a straight start, a shoulder (Hable's shape, fitted by Narkowicz)
  return clamp((e * (2.51 * e + 0.03)) / (e * (2.43 * e + 0.59) + 0.14), vec3f(0.0), vec3f(1.0));
}
/// False color, a band a stop; grey is the middle.
fn show_stops(b: f32) -> vec3f {
  if (b <= 0.0) { return vec3f(0.0); }
  let stops = log2(b / 0.18);
  return show_heat(stops / 12.0 + 0.5) * (0.75 + 0.25 * step(0.5, fract(stops)));
}
/// 0 dark blue, 1 yellow, evenly lighter all the way (a fit of viridis)
fn show_heat(x: f32) -> vec3f {
  let t = clamp(x, 0.0, 1.0);
  let coded = vec3f(
    0.28 + t * (-0.14 + t * (-1.95 + t * (5.9 + t * -3.1))),
    0.02 + t * (1.45 + t * (-0.75 + t * 0.18)),
    0.33 + t * (1.45 + t * (-3.2 + t * 1.55)),
  );
  return pow(clamp(coded, vec3f(0.0), vec3f(1.0)), vec3f(2.2));
}
fn show_id(n: u32) -> vec3f {
  if (n == NOTHING) { return vec3f(0.02); }
  var h = n * 747796405u + 2891336453u;
  h = ((h >> ((h >> 28u) + 4u)) ^ h) * 277803737u;
  h = (h >> 22u) ^ h;
  let hue = f32(h & 1023u) / 1023.0;
  let rgb = clamp(abs(fract(hue + vec3f(0.0, 0.667, 0.333)) * 6.0 - 3.0) - 1.0, vec3f(0.0), vec3f(1.0));
  return mix(vec3f(0.5), rgb, 0.6) * (0.3 + 0.5 * f32((h >> 10u) & 255u) / 255.0);
}
fn show_direction(v: vec3f) -> vec3f {
  if (all(v == vec3f(0.0))) { return vec3f(0.02); }
  return pow(clamp(v * 0.5 + 0.5, vec3f(0.0), vec3f(1.0)), vec3f(2.2));
}
/// A square a meter, red along x, green along y, blue along z.
fn show_place(p: vec3f) -> vec3f {
  if (dot(p, p) > 1e6) { return vec3f(0.02); } // beyond the world
  let cell = floor(p + 1e-4);
  let check = abs(cell.x + cell.y + cell.z) % 2.0;
  return fract(p + 1e-4) * (0.35 + 0.45 * check);
}
/// How far `a` is from `b`, in stops: red too bright, blue too dark, full color at one stop.
fn show_difference(a: f32, b: f32) -> vec3f {
  let least = exp2(-knobs.exposure) * 1e-3;
  let stops = log2((a + least) / (b + least));
  let m = pow(min(abs(stops), 1.0), 2.2);
  return vec3f(select(0.0, m, stops > 0.0), 0.12 * m, select(m, 0.0, stops > 0.0)) + 0.004;
}
