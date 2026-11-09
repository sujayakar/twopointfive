// A big body is written once here and asked in a loop: the compile pastes every call. Credit: Stephen Jayakar, from his pull request 1.
// THE FINAL GATHER'S HELPERS, CONSTANTS AND BINDINGS. No entry point is here: the gather is two programs, `trace` and `light`, in shaders/gather_split.wgsl, which is
// appended to this text (render/gatherSplit.ts). What the gather is:
// Half-resolution (a third with the lossy gatherThird option: frame.lossyCfg.x is the divisor) stochastic final gather. Per pixel: K cosine-distributed rays traced for real through the box grid
// (+ character capsules); at each hit the outgoing radiance is albedo/pi x (steady direct light, one importance-picked
// shadow ray + the exact unshadowed rest at half weight for the dim tail) + the cached multi-bounce irradiance (dice
// volume, read one bounce away from the eye where its coarseness cannot be seen) + emission; misses take the sky.
// Output is irradiance E at the primary hit (albedo applied in composite). History-accumulated + à-trous filtered
// downstream; transient lights never enter (they light the first hit directly through the history-free direct pass).
@group(1) @binding(0) var gDepth: texture_depth_2d;
@group(1) @binding(1) var gNormal: texture_2d<f32>;
@group(1) @binding(2) var gId: texture_2d<u32>;
@group(1) @binding(3) var directTex: texture_2d<f32>;    // denoised on-screen direct (sharp class) — reused when a gather ray lands on a visible surface
@group(1) @binding(4) var softTex: texture_2d<f32>;      // (broad class)
@group(1) @binding(5) var diceTex: texture_3d<f32>;
@group(1) @binding(6) var outIndirect: texture_storage_2d<rgba16float, write>;   // rgb = E estimate (a unused: temporal overwrites it with view depth)
@group(1) @binding(7) var outSpecular: texture_storage_2d<rgba16float, write>;  // rgb = glossy-floor reflection radiance toward the eye, Fresnel-weighted (FLAG_GLOSS only: nothing is stored with the flag off; a unused, as above)
@group(1) @binding(8) var cardsTex: texture_2d<f32>;      // torch bounce cards (cards.wgsl, rebuilt this frame; FLAG_BOUNCE_CARDS): texel (k, row), rows (C, R) · (n, slot) · (Φ_L, rq) · (Cq, 1)

override RAYS: u32 = 4u;
const TMAX: f32 = 40.0;
const CARD_MIN_E: f32 = 0.002;       // torch bounce cards: below this much unshadowed card irradiance (W/m², all cards) a pixel takes it without a shadow ray — under a percent of display white after the tonemap on any albedo, where a bounce shadow cannot be seen (the direct pass credits negligible tails unshadowed the same way)
// glossy floors (FLAG_GLOSS, glossReflection below)
const GLOSS_F0: f32 = 0.04;          // dielectric finish (stone sealer, waxed lino) at normal incidence — Schlick takes it to ~0.044 at this camera's 35-50° and 0.10 at 65°
const GLOSS_STRENGTH: f32 = 0.85;    // fraction of an ideal Fresnel coat the finish keeps (scuffs, dust)
const GLOSS_SPREAD: f32 = 0.0437;    // tan 2.5°: the mirror ray is jittered uniformly over a cone of this half-angle — a slightly rough polish; the specular history + one à-trous pass integrate it
const GLOSS_ALPHA: f32 = 0.0436;     // that half-angle in radians (the lobe's angular radius, for the analytic glints)
const GLOSS_SPILL: f32 = 0.005;      // a spot light's lens seen from outside its beam still shows this fraction of its on-axis radiance (reflector scatter): a torch held level glints in the floor as a clear dot (the guards' 70 W/sr behind a 7 cm lens at 1.5 m → ≈ 21 in the lobe → ≈ 0.8 scene-referred after Fresnel: bright, no bloom to speak of), and flares to GLOSS_GLINT_MAX when the beam swings through the mirror direction
const GLOSS_RAY_MAX: f32 = 24.0;     // luminance bound on what the one traced ray may bring back (the gather's own firefly bound: a lit panel ≈ 11 passes whole)
const GLOSS_GLINT_MAX: f32 = 160.0;  // …and on the analytic glints, which are noise-free and may therefore run far brighter (a torch lens is thousands: white after Fresnel, not a sun)

// (light / card staging, the RNG and directAtPoint live in gather_common.wgsl — shared with reflect.wgsl — and are prepended by passes.ts)

// STRATIFIED GATHER DIRECTIONS (docs/internal/trace-perf.md §2.10): the gather's k-th ray of K is drawn from STRATUM (k + frame) mod K of the cosine hemisphere —
// for the main gather's four rays (azimuth half) × (cos²θ half), each pixel jittering inside its stratum with its own two rnd().
// Every pixel still covers all K strata every frame (a stratified estimate: the same expectation, never more variance than iid), the rng draws the same COUNT of numbers, but the k-th
// rays of a whole 8×8 tile now share a stratum, i.e. a quarter of direction space: the lanes of a SIMD group walk more of the same grid cells (the trace lab measured 1.25× on such
// rays; −13‥16 % of the gather pass at high). The plain pattern (one cosine draw a ray, no strata) was a second `trace` program until 2026-11-07.
fn cosineDirK(N: vec3f, k: u32, K: u32) -> vec3f {
  let up = select(vec3f(0.0, 1.0, 0.0), vec3f(1.0, 0.0, 0.0), abs(N.y) > 0.9);
  let T = normalize(cross(up, N)); let B = cross(N, T);
  let s = (k + frame.frameIdx) % K;
  var u = rnd(); var v = rnd();                                              // u = r² (cos²θ = 1 − u), v = azimuth ÷ 2π
  if (K == 4u) { u = (f32(s & 1u) + u) * 0.5; v = (f32(s >> 1u) + v) * 0.5; }   // four rays: (azimuth half) × (cos²θ half)
  else if (K == 2u) {                                                          // two rays over the same four cells as a Latin square: ray k takes cos²θ half (k + frame/2) and azimuth half (k + frame) —
    let f = frame.frameIdx;                                                    // every frame still covers both azimuth halves and both θ bands, the pairing alternates
    u = (f32((k + (f >> 1u)) & 1u) + u) * 0.5; v = (f32((k + f) & 1u) + v) * 0.5;
  }
  else { v = (f32(s) + v) / f32(K); }                                          // K rays: K azimuth sectors
  let r = sqrt(u); let a = v * 6.2831853;
  return normalize(T * (cos(a) * r) + B * (sin(a) * r) + N * sqrt(max(1.0 - u, 0.0)));
}

// ---- glossy floors (FLAG_GLOSS): glossReflection below, and the two helpers it needs ----
// Fraction of a disc of (angular) radius a covered by a disc of radius b whose centre lies g away — plain circle–circle overlap over pi·a²
// (small angles: the lobe and an emitter's outline treated as flat discs on the sphere). Concentric → min(1, b²/a²); apart → 0; smooth between.
fn discOverlap(a: f32, b: f32, g: f32) -> f32 {
  if (g >= a + b) { return 0.0; }
  if (g <= abs(a - b)) { return min(1.0, (b * b) / (a * a)); }
  let x = (g * g + a * a - b * b) / (2.0 * g);          // centre of disc a → the chord (g > |a − b| ≥ 0 here, so no division by zero)
  let y = sqrt(max(a * a - x * x, 0.0));                // half chord (the same for both discs)
  let x2 = g - x;
  let lens = a * a * acos(clamp(x / a, -1.0, 1.0)) - x * y + b * b * acos(clamp(x2 / b, -1.0, 1.0)) - x2 * y;
  return clamp(lens / (3.14159265 * a * a), 0.0, 1.0);
}

// The lobe's view of the small bright emitters the tracer cannot see. Torch and rail-light lenses, beacon domes, the storage bulb, street-lamp
// heads are NoShadow glow cards or no geometry at all — a reflection RAY never finds them (and if it could, a 5 cm lens caught by one jittered
// ray in five would be firefly bait). So every steady point / spot light's emitting disc (li.pos, li.radius; a spot's faces li.dir) is folded
// into the GLOSS_ALPHA cone around the mirror direction R analytically: the fraction of the cone its outline covers × the disc's radiance
// (on-axis intensity ÷ disc area — for a spot × its own angular falloff toward this point, floored at GLOSS_SPILL in front of the lens) × one
// shadow ray (and the smoke on it) when that is nonzero. Every input is smooth, so the glint sits still without history: a beacon's sweeps
// past as a pulse, a walking guard's torch glides. In expectation it is what the jittered ray would integrate if the lens were traceable.
// Area lights (kind 3) stay with the ray — their fixture boxes ARE traceable and carry the emission, so counting them here would double
// them — and the moon (kind 2) is never in a mirror direction this camera can produce. (The one traceable lit fixture of a point/spot light,
// the manager's desk-lamp head, cannot be mirrored by any polished floor from up here.)
fn glossGlints(Po: vec3f, R: vec3f, owner: u32) -> vec3f {
  var G = vec3f(0.0);
  for (var i = 0u; i < nL; i++) {
    let li = wgLights[i];
    let kind = lightKind(li);
    if (kind >= 2u || li.radius <= 0.0 || lightTransient(li)) { continue; }   // flashes stay out of anything with history (the direct pass shows them)
    let d = li.pos - Po; let dist = length(d);
    if (dist > li.range || dist < 0.05) { continue; }
    let Ld = d / dist;
    let cosG = dot(Ld, R);
    if (cosG < 0.95) { continue; }                       // 18° off the mirror direction: no emitter here subtends anything like that (cheap reject before the trig)
    let g = atan2(length(cross(Ld, R)), cosG);           // angle between R and the emitter's centre (well conditioned near 0, unlike acos)
    let over = discOverlap(GLOSS_ALPHA, atan(li.radius / dist), g);
    if (over <= 0.0) { continue; }
    var Le = li.color * (lightWindow(dist, li.range) / (3.14159265 * li.radius * li.radius));   // li.color = colour × on-axis intensity (W/sr); a disc of area pi r² showing that intensity has this radiance — windowed by range exactly as evalLight windows the light itself, so a glint fades out with its light instead of popping at range
    if (kind == 1u) {
      let cd = dot(-Ld, normalize(li.dir));
      if (cd <= 0.0) { continue; }                       // behind the lens
      Le *= max(smoothstep(li.cosOuter, li.cosInner, cd), GLOSS_SPILL);
    }
    if (!lightReaches(i, Po)) { continue; }              // walled off from this cell (reach.ts): no ray needed to know
    let tmax = dist - li.radius - 0.02;
    if (tmax > 0.0 && occluded(Po, Ld, tmax, owner | ((li.owner & 0xffu) << 8u))) { continue; }   // the same skip rule every shadow ray toward this light uses (its carrier never hides it)
    var tr = 1.0;
    if (frame.numSmoke > 0u) { tr = smokeTransmittance(Po, Ld, min(max(tmax, 0.0), 30.0)); }
    G += Le * (over * tr);
  }
  return G;
}

// The radiance a polished floor at P (a BOX_GLOSS top face, N = +y) mirrors toward the eye. ONE reflection ray — the view direction reflected
// about the normal, jittered over a GLOSS_SPREAD cone — traced through the scene like a gather ray and shaded like a gather hit: emission (here
// INCLUDING area-lit fixtures: a mirror shows the glowing panel / screen / vending front itself, whereas the diffuse paths take those through
// their analytic lights) + albedo/pi × (direct at the hit — the exact on-screen textures when the hit is visible, else the one-shadow-ray
// estimate — + the cached bounce), the sky on a miss; plus the analytic glints of the small emitters no ray can see (above); then Schlick's
// Fresnel on the VIEW angle × the finish's strength. It is specular — never multiplied by the floor's albedo — so it leaves through its own
// texture (outSpecular → history + one à-trous pass in passes.ts → composite adds it after the diffuse terms). Called after the gather's
// rays: two rnd() for the cone plus whatever directAtPoint's pick draws, none of which the indirect estimate ever sees.
// The hit distance of the last reflection ray traced for this texel (glossReflection / glassShade write it; main stores it in outSpecular.a for the v3 denoiser's specular client —
// its virtual-hit-point reprojection; nothing on the legacy path reads that alpha). Sky / no ray = a large number.
var<private> specHitT: f32 = 1e4;
// (the reflection itself is gather_split.wgsl's glossReflectionX: the gather's `trace` writes the hit for `light` to finish)

// ---- torch bounce cards (FLAG_BOUNCE_CARDS): the cards' light at the shading point P (normal N) ----
// Per live card: the patch as FOUR sub-emitters at its flux RMS radius rq round its flux centroid Cq (cards.wgsl), each a quarter of Φ_L (= radiance × area)
// with the one-sided Lambertian emitter cosine (n), the receiver cosine (N), and 1 / (r² + rq²/2) — the on-axis-exact form Φ/(r² + a²) of a uniform disc of
// radius a, for a disc of RMS radius rq cut in four (R² = 2 rq², a² = R²/4): the far field of evalLight's kind-3 term, without its 1/max(d², R²) kink up
// close. Then ONE shadow ray for all of them, toward a point drawn uniformly over the equivalent disc (radius √2·rq, capped at the claim radius) of a card
// picked ∝ its unshadowed luminance here — the direct pass's tail rule: exact where one card dominates, which round a torch it does — with the smoke on that
// segment when smoke is up. Under CARD_MIN_E in all it is credited unshadowed (no ray). Every input is smooth, so the term holds still under the temporal
// filter; the visibility sample only shows inside real bounce shadows. Draws its rnd() AFTER the gather's rays, so those are value for value what they were.
fn cardIrradiance(P: vec3f, N: vec3f, Po: vec3f, owner: u32, pix: vec2u) -> vec3f {
  var Esum = vec3f(0.0); var Wsum = 0.0; var pick = -1;
  for (var k = 0u; k < nC; k++) {
    let c0 = wgCards[k * 4u]; let c2 = wgCards[k * 4u + 2u];
    if (c0.w <= 0.0) { continue; }
    let n = wgCards[k * 4u + 1u].xyz; let Cq = wgCards[k * 4u + 3u].xyz; let rq = c2.w;
    if (dot(n, P - Cq) <= 0.01) { continue; }               // behind or on the card's plane: a plane does not light itself (the wall round the spot gets nothing from it)
    let up = select(vec3f(0.0, 1.0, 0.0), vec3f(1.0, 0.0, 0.0), abs(n.y) > 0.9);
    let T = normalize(cross(up, n)); let B = cross(n, T);
    var gsum = 0.0;
    for (var i = 0u; i < 4u; i++) {
      let a = 0.7853982 + f32(i) * 1.5707963;
      let d = (Cq + (T * cos(a) + B * sin(a)) * rq) - P;
      let r2 = dot(d, d); let L = d * inverseSqrt(max(r2, 1e-6));
      gsum += max(dot(N, L), 0.0) * max(dot(n, -L), 0.0) / (r2 + 0.5 * rq * rq);
    }
    let Ek = c2.xyz * (0.25 * gsum);
    let w = luminance(Ek);
    if (w < 1e-6) { continue; }
    Esum += Ek; Wsum += w;
    if (rnd() * Wsum < w) { pick = i32(k); }
  }
  if (pick < 0 || Wsum < CARD_MIN_E) { return Esum; }
  let k = u32(pick); let c0 = wgCards[k * 4u]; let n = wgCards[k * 4u + 1u].xyz; let Cq = wgCards[k * 4u + 3u].xyz; let rq = wgCards[k * 4u + 2u].w;
  let up = select(vec3f(0.0, 1.0, 0.0), vec3f(1.0, 0.0, 0.0), abs(n.y) > 0.9);
  let T = normalize(cross(up, n)); let B = cross(n, T);
  // the disc point: a per-pixel R2 sequence stepped by the frame index (radius² and angle on two irrational strides off an interleaved-gradient offset), not two
  // fresh rnd() — every run of consecutive frames then covers the disc evenly, so the history's running average of this ONE visibility sample settles on the
  // patch's fractional visibility instead of wandering round it: the flicker in a bounce shadow's penumbra measured 3× lower for the same ray (perf doc, A.4)
  let fi = f32(frame.frameIdx & 1023u);
  let ur = fract(ign(vec2f(pix)) + fi * 0.7548777); let ua = fract(ign(vec2f(pix.yx) + 17.0) + fi * 0.5698403);
  let r = sqrt(ur) * min(1.4142 * rq, c0.w); let a = ua * 6.2831853;
  let d = (Cq + (T * cos(a) + B * sin(a)) * r + n * 0.02) - Po;
  let dist = length(d); let L = d / dist;
  var vis = 1.0;
  // a visibility sample drawn BELOW the receiver's horizon is not evidence of occlusion (the quadrature already zeroes below-horizon emitter points): test
  // occlusion / smoke only for a sample the receiver can face — else a receiver whose tangent plane cuts the disc (a desk top level with a wall spot, the
  // floor under a low spot) had its whole card sum zeroed on the frames the point fell below the plane: systematic darkening + a 0/1 alternation (review MED #1)
  if (dot(L, N) > 0.0) {
    if (occluded(Po, L, max(dist - 0.03, 0.0), owner)) { vis = 0.0; }
    else if (frame.numSmoke > 0u) { vis = smokeTransmittance(Po, L, min(dist, 30.0)); }
  }
  return Esum * vis;
}

// ---- glass panes (FLAG_GLASS): what a pane ADDS toward the eye at the point the view ray crosses it — composite already holds the transmitted picture (the
// G-buffer is what stands behind the pane; composite tints it), so this is the mirror image + the glints + the dust glow, all additive, leaving through the
// same specular texture / history / à-trous pass as the glossy floors. ----
const GLASS_DUST: f32 = 0.035;       // albedo of the grime film on a pane: the share of the light ARRIVING at the glass (either face) it scatters diffusely — what makes a torch beam or a lit panel show on the glass itself; × frame.glassCfg.z
// Pg = the crossing point, n = the pane's normal on the eye's side, V = the view direction (eye → scene). One mirror ray (the exact reflection: float glass is
// flat to well under a pixel — no cone, so nothing for the history to integrate but the hit's own shading noise), shaded like a gather hit exactly as
// glossReflection does (emission INCLUDING area-lit fixtures: a mirror shows the glowing panel itself; albedo/π × (direct at the hit + the cached bounce); the sky
// on a miss), plus glossGlints' analytic image of the small emitters no ray can find, × Schlick's Fresnel at the view angle (n = 1.5) × the strength knob; then
// the dust: every steady light's irradiance ON the pane (no receiver cosine but the pane's own projected area, both faces alike), the two strongest shadow-
// tested (+ smoke), the rest at half credit unshadowed (dim by construction, reach-masked), × GLASS_DUST/π.
// (the pane's shading is gather_split.wgsl's glassShadeX, the same way)
