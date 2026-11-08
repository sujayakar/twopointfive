// The sky as an image: a ball of air lit by the sun (Rayleigh, Mie, ozone; one scattering and a share for the rest), or by the moon at night, with thin
// cloud. The sun's own disc is left out: it is a lamp. Written to the octahedral map's first level; its mips are the blur.
const RG = 6360.0; const RA = 6460.0;
fn raySph(o: vec3f, d: vec3f, r: f32) -> vec2f { let b = dot(o, d); let c = dot(o, o) - r * r; let q = b * b - c; if (q < 0.0) { return vec2f(1e9, -1e9); } let s = sqrt(q); return vec2f(-b - s, -b + s); }
fn dens(h: f32) -> vec3f { return vec3f(exp(-h / 8.0), exp(-h / 1.2), max(0.0, 1.0 - abs(h - 25.0) / 15.0)); }
const BR = vec3f(0.0058, 0.0135, 0.0331); const BM = 0.0040; const BO = vec3f(0.00065, 0.00188, 0.000085);
fn ext(od: vec3f) -> vec3f { return exp(-(BR * od.x + vec3f(BM * 1.1) * od.y * knob.z + BO * od.z)); }
fn depthTo(o: vec3f, d: vec3f) -> vec3f { let t = raySph(o, d, RA).y; var od = vec3f(0.0); let n = 8; let dt = t / f32(n); for (var i = 0; i < n; i++) { let p = o + d * (dt * (f32(i) + 0.5)); od += dens(length(p) - RG) * dt; } return od; }
fn air(d: vec3f, sd: vec3f, E: vec3f) -> vec3f {
  let o = vec3f(0.0, RG + 0.15, 0.0); let g = raySph(o, d, RG); var t = raySph(o, d, RA).y; if (g.x > 0.0 && g.x < 1e8) { t = g.x; }
  let mu = dot(d, sd); let pr = 3.0 / (16.0 * PI) * (1.0 + mu * mu); let gg = 0.76; let pm = 3.0 / (8.0 * PI) * ((1.0 - gg * gg) * (1.0 + mu * mu)) / ((2.0 + gg * gg) * pow(1.0 + gg * gg - 2.0 * gg * mu, 1.5));
  var od = vec3f(0.0); var L = vec3f(0.0); let n = 24; var last = 0.0;
  for (var i = 0; i < n; i++) {
    let e = f32(i + 1) / f32(n); let tt = t * e * e; let dt = tt - last; let p = o + d * (last + dt * 0.5); last = tt;
    let h = length(p) - RG; let dn = dens(h); od += dn * dt;
    let gs = raySph(p, sd, RG); var sun = vec3f(0.0); if (!(gs.x > 0.0 && gs.x < 1e8)) { sun = ext(depthTo(p, sd)); }
    let T = ext(od); let up = sat(dot(normalize(p), sd) * 2.5 + 0.35);
    L += T * dt * ((BR * (dn.x * pr) + vec3f(BM * dn.y * knob.z * pm)) * sun + (BR * dn.x + vec3f(BM * dn.y * knob.z)) * (0.045 * up) * ext(dens(0.0) * 6.0) );
  }
  return L * E;
}
fn vn(p: vec3f) -> f32 { let i = floor(p); let f = p - i; let w = f * f * (3.0 - 2.0 * f); let a = vec4f(hash33(i).x, hash33(i + vec3f(1.0, 0.0, 0.0)).x, hash33(i + vec3f(0.0, 1.0, 0.0)).x, hash33(i + vec3f(1.0, 1.0, 0.0)).x); let b = vec4f(hash33(i + vec3f(0.0, 0.0, 1.0)).x, hash33(i + vec3f(1.0, 0.0, 1.0)).x, hash33(i + vec3f(0.0, 1.0, 1.0)).x, hash33(i + vec3f(1.0, 1.0, 1.0)).x); let z = mix(a, b, w.z); return mix(mix(z.x, z.y, w.x), mix(z.z, z.w, w.x), w.y); }
fn cl(p: vec3f) -> f32 { return 0.5 * vn(p) + 0.25 * vn(p * 2.1 + 3.0) + 0.13 * vn(p * 4.3 + 9.0) + 0.07 * vn(p * 8.9 + 1.0) + 0.04 * vn(p * 17.0); }
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let n = textureDimensions(skyOut); if (id.x >= n.x || id.y >= n.y) { return; }
  var d = octDec((vec2f(id.xy) + 0.5) / vec2f(n)); let below = d.y < 0.0; let dd = normalize(vec3f(d.x, max(d.y, 0.015), d.z));
  let sd = normalize(knobSun.xyz);
  var L = air(dd, sd, vec3f(1.0, 0.97, 0.92) * knob.x);
  // a night that is not black: moonlight on the air, the city's glow at the horizon, stars
  let night = sat(-sd.y * 6.0 + 0.35) * knob.y;
  L += night * (vec3f(0.010, 0.016, 0.034) * (0.6 + 0.4 * dd.y) + vec3f(0.055, 0.032, 0.016) * exp(-dd.y * 7.0));
  let st = hash33(floor(dd * 210.0)); L += night * step(0.9986, st.x) * vec3f(0.5, 0.55, 0.7) * st.y * smoothstep(0.1, 0.4, dd.y);
  if (dd.y > 0.02) {
    let q = vec3f(dd.x, 0.0, dd.z) / (dd.y + 0.12); let c = cl(vec3f(q.x * 0.9 + 3.0, 1.7, q.z * 1.7 + 1.0) + cl(q * 0.8) * 1.2); let cov = max(knob.w - 0.9, 0.0); let m = min(smoothstep(0.50 - 0.5 * cov, 0.72 - 0.5 * cov, c) * smoothstep(0.03 - 0.1 * cov, 0.2 - 0.15 * cov, dd.y) * knob.w, 1.3);
    let toSun = max(dot(dd, sd), 0.0); let lit = air(normalize(vec3f(dd.x, 0.25, dd.z)), sd, vec3f(1.0) * knob.x) * 1.1 + ext(dens(0.0) * (6.0 / max(sd.y + 0.12, 0.06))) * (knob.x * sat(sd.y * 4.0 + 0.25) * (0.05 + 0.25 * pow(toSun, 12.0) + 0.06 * pow(toSun, 2.0)));
    let grey = vec3f(0.92, 0.97, 1.0) * (knob.x * sat(sd.y * 2.2 + 0.12) * 0.045 * (0.6 + 0.8 * c)) + night * vec3f(0.016, 0.017, 0.021) * (0.5 + c); L = mix(L, mix(lit + night * vec3f(0.012, 0.014, 0.02), grey, min(cov, 1.0)), min(m * mix(0.75, 1.0, min(cov, 1.0)), 1.0));
  }
  if (below) { let gnd = vec3f(0.20, 0.205, 0.21) * (air(vec3f(0.0, 1.0, 0.0), sd, vec3f(knob.x)) * 2.2 + vec3f(knob.x * 0.12 * sat(sd.y))); L = mix(L, gnd, smoothstep(0.0, 0.25, -d.y)); }
  textureStore(skyOut, id.xy, vec4f(min(L, vec3f(60.0)), 1.0));
}
