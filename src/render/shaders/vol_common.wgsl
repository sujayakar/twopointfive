// Shared by the per-pixel volumetric march (volumetrics.wgsl) and the froxel kernels (froxel.wgsl): the haze's phase + drifting dust density, and the
// MASKED smoke samplers — exact versions of smokeTransmittance / smokeSelfShadow that only visit the domains in `mask`, for callers that have proved no other
// live domain can hold the point (a point on a segment lies only in domains whose box the segment crosses). With eight domains live and one or two of them
// anywhere near a given ray, the unmasked loops spent most of a beam step testing boxes kilometres of corridor away. The masked density sampler itself,
// smokeDensityIn, now lives in smoke_read.wgsl: the shadow-ray transmittance every pass takes walks a mask too (smokeSegment / smokeTransmittanceN).
// Prelude: scene (+ lighting_common for the includers that want the dice).
const PI: f32 = 3.14159265;

fn phaseHG(cosT: f32, g: f32) -> f32 { let g2 = g * g; return (1.0 - g2) / (4.0 * PI * pow(1.0 + g2 - 2.0 * g * cosT, 1.5)); }
// cheap animated value noise → drifting dust density in beams
fn vnoise(p: vec3f) -> f32 {
  let i = floor(p); let f = fract(p); let u = f * f * (3.0 - 2.0 * f);
  let n000 = hash13(i); let n100 = hash13(i + vec3f(1.0, 0.0, 0.0)); let n010 = hash13(i + vec3f(0.0, 1.0, 0.0)); let n110 = hash13(i + vec3f(1.0, 1.0, 0.0));
  let n001 = hash13(i + vec3f(0.0, 0.0, 1.0)); let n101 = hash13(i + vec3f(1.0, 0.0, 1.0)); let n011 = hash13(i + vec3f(0.0, 1.0, 1.0)); let n111 = hash13(i + vec3f(1.0, 1.0, 1.0));
  return mix(mix(mix(n000, n100, u.x), mix(n010, n110, u.x), u.y), mix(mix(n001, n101, u.x), mix(n011, n111, u.x), u.y), u.z);
}
fn hazeAt(X: vec3f) -> f32 { let q = X * 0.9 + vec3f(frame.time * 0.05, -frame.time * 0.02, frame.time * 0.035); let h = 0.55 + 0.9 * vnoise(q) * vnoise(q * 2.7 + 3.1); if (WX_KINDS) { if (frame.cardCfg.w != 0u) { return h * wxHazeK(X); } } return h; }   // (a weather kind's fog thickens it outdoors: common.wgsl wxHazeK; no fog = the value it always was)
fn max3(v: vec3f) -> f32 { return max(v.x, max(v.y, v.z)); }

// Parametric interval [t0, t1] of the ray O + tV inside the (positive nappe of the) cone with apex A, axis D, cos half-angle cosT.
// Returns t0 > t1 when empty. Unbounded ends are ±1e9.
fn coneInterval(O: vec3f, V: vec3f, A: vec3f, D: vec3f, cosT: f32) -> vec2f {
  let CO = O - A;
  let vd = dot(V, D); let cod = dot(CO, D); let c2 = cosT * cosT;
  let a = vd * vd - c2;
  let b = 2.0 * (vd * cod - c2 * dot(V, CO));
  let c = cod * cod - c2 * dot(CO, CO);
  // where the axial coordinate s(t) = cod + t*vd is >= 0
  var sLo = -1e9; var sHi = 1e9;
  if (vd > 1e-6) { sLo = -cod / vd; } else if (vd < -1e-6) { sHi = -cod / vd; } else if (cod < 0.0) { return vec2f(1.0, 0.0); }
  if (abs(a) < 1e-7) {
    // ray direction lies on the cone angle: f(t) = b t + c linear
    var lo = -1e9; var hi = 1e9;
    if (abs(b) < 1e-9) { if (c < 0.0) { return vec2f(1.0, 0.0); } }
    else if (b > 0.0) { lo = -c / b; } else { hi = -c / b; }
    return vec2f(max(lo, sLo), min(hi, sHi));
  }
  let disc = b * b - 4.0 * a * c;
  if (disc < 0.0) {
    if (a < 0.0) { return vec2f(1.0, 0.0); }        // never inside
    return vec2f(sLo, sHi);                          // inside the double cone everywhere → positive nappe part
  }
  let sq = sqrt(disc);
  var r0 = (-b - sq) / (2.0 * a); var r1 = (-b + sq) / (2.0 * a);
  if (r0 > r1) { let tmp = r0; r0 = r1; r1 = tmp; }
  if (a < 0.0) {
    // inside between the roots (if that's the positive nappe)
    let mid = 0.5 * (r0 + r1);
    if (cod + mid * vd < 0.0) { return vec2f(1.0, 0.0); }
    return vec2f(r0, r1);
  }
  // a > 0: inside outside the roots; the positive nappe is the half-line on the s>=0 side
  if (vd > 0.0) { return vec2f(max(r1, sLo), 1e9); }
  return vec2f(-1e9, min(r0, sHi));
}

/** transmittance toward a light along ro + t·rd, t < tmax, for the (K > 0, non-froxel) smoke march's own step lighting: the shadow estimator at 8 samples whatever the frame's count
 *  (= smoke_read.wgsl smokeTransmittanceN — the masked walk this file introduced is now the one everybody takes, tightened to the domains' occupied boxes) */
fn smokeTransIn(ro: vec3f, rd: vec3f, tmax: f32) -> f32 { return smokeTransmittanceN(ro, rd, tmax, 8.0); }
/** … with 5 samples — the froxels' haze uses it for how much a plume dims a beam beyond it (a per-froxel term, softer than the march's per-step one by construction) */
fn smokeTransCoarse(ro: vec3f, rd: vec3f, tmax: f32) -> f32 { return smokeTransmittanceN(ro, rd, tmax, 5.0); }
fn smokeSelfShadowIn(P: vec3f, L: vec3f, mask: u32) -> f32 {
  var od = 0.0; let ds = smokeSelfStep();
  for (var i = 1; i <= 4; i++) { od += smokeDensityIn(P + L * (f32(i) - 0.3) * ds, mask); }
  return exp(-od * ds * smokeSelfK());
}
/** HOW SMOKE IS DRAWN, AS ROWS (frame.sideCfg.y and .z; the smoke lab's section of that name, host/smokelab.ts; Engine.smokeDraw). Zeros are the picture as it was, and
 *  nothing but the lab writes anything else.
 *    .y  bits 0‥7    creases: how much the soft light on a sample is dimmed by the smoke crowding round it, × 100 (smokeCreaseIn below)
 *        bits 8‥15   the self-shadow's extinction per unit density, × 4 (0 = SMOKE_SELF_K)
 *        bits 16‥23  the self-shadow's reach toward the light, in 2 cm (0 = 0.72 m: four taps 0.18 m apart)
 *        bits 24‥31  how much a lamp behind smoke glows through it, × 100 (smokeAheadLobe below)
 *    .z  bit 0       a tint the smoke carries is a streak in it and not dust: it is drawn as a tint alone (dust_look.wgsl dustDensity) */
fn smokeSelfK() -> f32 { let v = (frame.sideCfg.y >> 8u) & 255u; return select(SMOKE_SELF_K, f32(v) * 0.25, v != 0u); }
fn smokeSelfStep() -> f32 { let v = (frame.sideCfg.y >> 16u) & 255u; return select(0.18, f32(v) * 0.005, v != 0u); }
fn smokeCreaseK() -> f32 { return f32(frame.sideCfg.y & 255u) * 0.01; }
/** .y bits 24‥31: HOW MUCH A LAMP BEHIND SMOKE GLOWS THROUGH IT, × 100 (the lab's row of that name; 0 = smoke only dims a lamp behind it, as it was). */
fn smokeAheadW() -> f32 { return f32((frame.sideCfg.y >> 24u) & 255u) * 0.01; }
const SMOKE_AHEAD_G: f32 = 0.8;   // the narrow lobe (half its peak 18° off the lamp's line)
/// HOW THICK SMOKE IS TO THAT LOBE'S LIGHT. Light the smoke sends on within a few degrees of its line is still in the beam, so only the share it sends AWAY
/// from the line is lost to it: a medium whose scattering has mean cosine g carries such light as one (1 − g) as thick (the similarity relation of transport
/// theory; g is the lobe's own, so there is no number to tune). The glow reaches a sample, and the eye from it, through the smoke's transmittance to that power.
/// Through HALF the smoke, as the octave goes, a plume of optical depth 4 before a lit sign took a twentieth of the sign's brightness where a slab that
/// scatters forward passes about half of it on as glow, and the sign's sharp picture (1.5 % of it, by Beer's law) stood clear of smoke that stayed dark.
const SMOKE_AHEAD_THIN: f32 = 1.0 - SMOKE_AHEAD_G;
fn smokeAheadT(t: f32) -> f32 { return pow(max(t, 0.0), SMOKE_AHEAD_THIN); }
/** A LAMP BEHIND SMOKE GLOWS THROUGH IT. mu = the eye's ray · the way to the lamp (1 = the lamp stands straight behind the sample), r2d2 = the lamp's radius² over its
 *  distance² from the sample (0 for the sun). What it stands for: smoke sends most of what it scatters nearly straight on, and light sent on again and again is still
 *  in the beam. The march's extinction counts every scattering as a loss, and its one wide lobe (g 0.35) gives back the part that goes sideways, so a lit sign behind a
 *  plume was only ever dimmed: every edge kept through smoke that hid the wall round it, and a thick plume in front of it a grey cut-out with none of its colour. This
 *  is the part that went on: per lamp a narrow lobe, seen through HALF the smoke both ways (√ the self-shadow toward the lamp, √T to the eye, the octave's own
 *  statement in volumetrics.wgsl) because light scattered forward has not left the beam. Looking at a lamp through smoke, the smoke between glows in the lamp's colour
 *  round the lamp's place, softer and wider the more smoke lies between, where a screen-space blur of what is behind the plume stood in for it. The wide lobe is not
 *  touched, so a plume lit from the side is as it was.
 *  A LAMP IS A DISC, NOT A POINT, TO THIS LOBE: it is asked at the disc's nearest edge to the eye's line and straight on anywhere inside the disc (asked at the centre,
 *  smoke in front of the lower half of a lit sign found the sign's middle 60° off its ray and got nothing), and its mean over the disc cannot pass 1 ∕ the disc's solid
 *  angle, which keeps the smoke in front of a panel no brighter than the panel. */
fn smokeAheadLobe(mu: f32, r2d2: f32) -> f32 {
  if (smokeAheadW() <= 0.0) { return 0.0; }
  let ca = inverseSqrt(1.0 + r2d2); let sa = sqrt(r2d2) * ca;                     // cos and sin of the disc's angular radius from the sample
  let near = select(min(mu * ca + sqrt(max(1.0 - mu * mu, 0.0)) * sa, 1.0), 1.0, mu >= ca);
  return min(phaseHG(near, SMOKE_AHEAD_G), 1.0 / max(PI * r2d2, 1e-6));
}
/// one light's share of it at a step: its irradiance there × the lobe (`mu` = the eye's line against the light's, `radius` the lamp's, `dist` to it). Only a
/// light the step has TRACED gets it: the lobe's peak is sixty times the wide lobe's sideways value, so a lamp credited without a ray would shine through a wall.
fn smokeAheadE(E: vec3f, mu: f32, radius: f32, dist: f32) -> vec3f {
  if (smokeAheadW() <= 0.0) { return vec3f(0.0); }
  return E * smokeAheadLobe(mu, radius * radius / max(dist * dist, 1e-4));
}
/** THE SOFT LIGHT ON A SAMPLE OF SMOKE, dimmed by the smoke round it. The froxels hand the march one ambient for a whole plume (the room's light at that place, with no
 *  smoke in the way), so with little direct light on it a plume is one even tone and the curls the solver made cannot be seen in it. Soft light comes from all round, and
 *  what stops it is the smoke between the sample and the open air: four taps on a tetrahedron 0.2 m out (one up, three a little below level) say how much smoke crowds
 *  round the sample, and the light is taken down by that much of an optical depth. A crest of a billow has open air on most sides and keeps its light, the crease between
 *  two billows has smoke on every side and goes dark. k = 0 is never called. */
fn smokeCreaseIn(P: vec3f, mask: u32, k: f32) -> f32 {
  let r = 0.2;
  let od = smokeDensityIn(P + r * vec3f(0.0, 1.0, 0.0), mask) + smokeDensityIn(P + r * vec3f(0.943, -0.333, 0.0), mask)
         + smokeDensityIn(P + r * vec3f(-0.471, -0.333, 0.816), mask) + smokeDensityIn(P + r * vec3f(-0.471, -0.333, -0.816), mask);
  return exp(-od * (0.25 * r * 6.0) * k);
}
/** the self-shadow's extinction per unit density over its 0.72 m toward the light: 6 = the march's own σt (as it always was); 9 since 08-12 — the lit side of a billow
 *  keeps its light and the fold behind it goes dark, which is much of what reads as VOLUME in a lit plume (a look constant: only this march reads it) */
const SMOKE_SELF_K: f32 = 6.0;   // (9 deepens the folds further — a look call the maintainer has not made; Tab has no row for it yet)
/** the smoke march's PER-STEP LIGHT count (frame.volCfg.w bits 8‥11 = settings.smokeMarchLights; docs/internal/smoke-look.md): 0 = the froxel-lit march exactly as it
 *  was (every light reaches the plume trilinearly through the J volume); K > 0 = injectJ writes only the AMBIENT into J and the march lights the K strongest lights on each
 *  stretch per step itself (exact cone edge, a shadow ray, the step's own self-shadow) — the pre-froxel plume lighting, without J's flat blob or its slice lattice.
 *  Both producers read this one number, so J and the march always agree on who lights what. */
fn smokeMarchLights() -> u32 { return (frame.volCfg.w >> 8u) & 15u; }
/** the MULTIPLE-SCATTERING octave's gain (frame.volCfg.w bits 12‥15 ÷ 10; settings.smokeMultiScatter, 0 = single scattering exactly as it was): per lit step a second lobe with a
 *  phase half as forward and the light seen through HALF the self-shadow's extinction (√ of the single-scatter self-shadow), integrated to the eye through half the extinction
 *  too (√T, √stepT) — Wrenninge et al. 2013's octave with a = b = c = ½. What it buys: a plume can be opaque to what stands behind it AND still show the coloured glow of its
 *  lit side through its body, where single scattering alone shows an ambient-lit shell (the maintainer, 08-12: 'the lobes … more structure to the smoke + lighting, less flat'). */
fn smokeMsGain() -> f32 { return f32((frame.volCfg.w >> 12u) & 15u) * 0.1; }
const SMOKE_MS_G: f32 = 0.175;   // the octave's phase eccentricity (½ of the single lobe's 0.35)
/** bits of the live domains whose box, grown by `grow` metres, overlaps the box [lo, hi] — what can hold any sample taken within `grow` of that box */
fn smokeDomainsNear(lo: vec3f, hi: vec3f, grow: f32) -> u32 {
  var mask = 0u;
  for (var i = 0u; i < frame.numSmoke; i++) {
    let sd = smoke[i];
    if (sd.live == 0u) { continue; }
    let bmin = sd.origin - vec3f(grow); let bmax = sd.origin + sd.voxel * vec3f(sd.dims) + vec3f(grow);
    if (all(bmin <= hi) && all(bmax >= lo)) { mask |= 1u << i; }
  }
  return mask;
}
