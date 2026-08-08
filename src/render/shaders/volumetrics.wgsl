// Half-resolution (quarter with the lossy volQuarter option: frame.lossyCfg.y is the divisor) volumetrics: (a) thin haze lit by spot/point lights with ray-traced shadows (visible beams),
// (b) ray-marched smoke domains lit by the same lights (shadow rays + self-shadowing) and by the RC mean radiance.
// Output: rgb = in-scattered radiance, a = transmittance toward the surface.
@group(1) @binding(0) var gDepth: texture_depth_2d;
@group(1) @binding(1) var diceTex: texture_3d<f32>;
@group(1) @binding(2) var outVol: texture_storage_2d<rgba16float, write>;

const PI: f32 = 3.14159265;

fn phaseHG(cosT: f32, g: f32) -> f32 { let g2 = g * g; return (1.0 - g2) / (4.0 * PI * pow(1.0 + g2 - 2.0 * g * cosT, 1.5)); }
// cheap animated value noise → drifting dust density in beams
fn vnoise(p: vec3f) -> f32 {
  let i = floor(p); let f = fract(p); let u = f * f * (3.0 - 2.0 * f);
  let n000 = hash13(i); let n100 = hash13(i + vec3f(1.0, 0.0, 0.0)); let n010 = hash13(i + vec3f(0.0, 1.0, 0.0)); let n110 = hash13(i + vec3f(1.0, 1.0, 0.0));
  let n001 = hash13(i + vec3f(0.0, 0.0, 1.0)); let n101 = hash13(i + vec3f(1.0, 0.0, 1.0)); let n011 = hash13(i + vec3f(0.0, 1.0, 1.0)); let n111 = hash13(i + vec3f(1.0, 1.0, 1.0));
  return mix(mix(mix(n000, n100, u.x), mix(n010, n110, u.x), u.y), mix(mix(n001, n101, u.x), mix(n011, n111, u.x), u.y), u.z);
}
fn hazeAt(X: vec3f) -> f32 { let q = X * 0.9 + vec3f(frame.time * 0.05, -frame.time * 0.02, frame.time * 0.035); return 0.55 + 0.9 * vnoise(q) * vnoise(q * 2.7 + 3.1); }

fn smokeSelfShadow(P: vec3f, L: vec3f) -> f32 {
  var od = 0.0; let ds = 0.18;
  for (var i = 1; i <= 4; i++) { od += smokeDensityAt(P + L * (f32(i) - 0.3) * ds); }
  return exp(-od * ds * 6.0);
}

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

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) gid: vec3u) {
  let odims = textureDimensions(outVol);
  if (gid.x >= odims.x || gid.y >= odims.y) { return; }
  if ((frame.flags & (FLAG_VOLUMETRICS | FLAG_SMOKE)) == 0u) { textureStore(outVol, gid.xy, vec4f(0.0, 0.0, 0.0, 1.0)); return; }
  let vdiv = u32(frame.lossyCfg.y);   // 2, or 4 (lossy option): each texel marches through one full-res pixel of its vdiv×vdiv block, interleaved by texel parity as before (×1 at vdiv 2)
  let full = min(gid.xy * vdiv + vec2u(gid.y & 1u, gid.x & 1u) * (vdiv / 2u), vec2u(frame.screen) - 1u);
  let depth = textureLoad(gDepth, full, 0);
  let pix = vec2f(full) + 0.5;
  let Pfar = worldFromDepth(pix, select(depth, 1.0, depth >= 1.0));
  let ro = frame.camPos;
  var rd = Pfar - ro; var tSurf = length(rd); rd /= tSurf;
  if (depth >= 1.0) { tSurf = min(tSurf, 200.0); }
  let jitter = ign(vec2f(gid.xy));
  let invRd = safeInv(rd);
  var inscatter = vec3f(0.0);
  var T = 1.0;

  // ---- (b) smoke first (it attenuates what is behind it; haze contribution is mostly in front/around, error is small) ----
  if ((frame.flags & FLAG_SMOKE) != 0u && frame.numSmoke > 0u) {
    var t0 = 1e30; var t1 = -1e30; var vox = 1.0;
    for (var i = 0u; i < frame.numSmoke; i++) {
      let sd = smoke[i];
      if (sd.live == 0u) { continue; }
      let tb = isectBounds(ro, invRd, sd.origin, sd.origin + sd.voxel * vec3f(sd.dims));
      if (tb.x <= tb.y && tb.y > 0.0 && tb.x < tSurf) { t0 = min(t0, max(tb.x, 0.0)); t1 = max(t1, min(tb.y, tSurf)); vox = min(vox, sd.voxel); }
    }
    if (t1 > t0) {
      // per-ray light shortlist: a light whose unshadowed irradiance cannot reach the in-scatter threshold (0.03) anywhere on
      // [t0, t1] — bounded from its closest approach to the segment — is dropped for every step of the march. Exact culling
      // (the bound is never below what evalLight can return), and it turns ~34 evaluations per step into a handful.
      var cand: array<u32, 16>; var nc = 0u; var everyLight = false;   // set when the shortlist overflows: fall back to walking every light
      if (!everyLight) {
        for (var i = 0u; i < frame.numLights; i++) {
          let li = lights[i]; let mc = max(li.color.x, max(li.color.y, li.color.z));
          var keep = false;
          if (lightKind(li) == 2u) { keep = mc >= 0.03; }
          else {
            let tc = clamp(dot(li.pos - ro, rd), t0, t1); let dv = ro + rd * tc - li.pos; let d2 = dot(dv, dv);
            keep = d2 <= li.range * li.range && mc / max(d2, li.radius * li.radius) >= 0.03;
          }
          if (keep) { if (nc < 16u) { cand[nc] = i; nc++; } else { everyLight = true; } }
        }
      }
      let nLoop = select(nc, frame.numLights, everyLight);
      let span = t1 - t0;
      let steps = clamp(u32(span / (vox * 1.5)) + 1u, 4u, 112u);
      let dt = span / f32(steps);
      var t = t0 + dt * jitter;
      for (var s = 0u; s < steps; s++) {
        let X = ro + rd * t;
        let dens = smokeDensityAt(X);
        if (dens > 0.002) {
          let sigT = dens * 6.0;             // extinction (1/m) per unit density (domain densityScale already applied)
          let sigS = sigT * 0.92;
          var Lin = diceMeanRadiance(X);     // ambient: mean radiance (isotropic phase → radiance directly)
          for (var k = 0u; k < nLoop; k++) {
            let li = lights[select(cand[min(k, 15u)], k, everyLight)];
            let ev = evalLight(li, X, vec3f(0.0));
            if (!ev.ok || max(ev.E.x, max(ev.E.y, ev.E.z)) < 0.03) { continue; }   // below this a light adds nothing you can see in smoke; the test is what keeps the per-step cost bounded now that panels reach across the floor
            if (occluded(X, ev.L, ev.dist.x - li.radius - 0.02, (li.owner & 0xffu) << 8u)) { continue; }   // the light's own carrier never shadows its beam
            Lin += ev.E * phaseHG(dot(rd, ev.L), 0.35) * smokeSelfShadow(X, ev.L);
          }
          let stepT = exp(-sigT * dt);
          // energy-conserving integration over the step
          inscatter += T * Lin * sigS * (1.0 - stepT) / max(sigT, 1e-4);
          T *= stepT;
          if (T < 0.01) { break; }
        }
        t += dt;
      }
    }
  }

  // ---- (a) haze beams ----
  if ((frame.flags & FLAG_VOLUMETRICS) != 0u && frame.hazeDensity > 0.0) {
    let sigS = frame.hazeDensity; // scattering coefficient of the thin uniform haze (1/m); extinction ignored
    for (var i = 0u; i < frame.numLights; i++) {
      let li = lights[i];
      if (li.volumetric <= 0.0 || lightKind(li) == 2u) { continue; }
      let tb = isectBounds(ro, invRd, li.pos - vec3f(li.range), li.pos + vec3f(li.range));
      let ta = max(tb.x, 0.0); let tbb = min(tb.y, tSurf);
      if (ta >= tbb) { continue; }
      // tighter bound: sphere
      let oc = ro - li.pos; let bq = dot(oc, rd); let cq = dot(oc, oc) - li.range * li.range;
      let disc = bq * bq - cq; if (disc <= 0.0) { continue; }
      let sq = sqrt(disc);
      var s0 = max(-bq - sq, ta); var s1 = min(-bq + sq, tbb);
      if (lightKind(li) == 1u) { let ci = coneInterval(ro, rd, li.pos, li.dir, li.cosOuter); s0 = max(s0, ci.x); s1 = min(s1, ci.y); }
      if (s0 >= s1) { continue; }
      // equi-angular sampling: distributes samples ∝ 1/d² to the light, so the bright region near the source is not undersampled
      let delta = dot(li.pos - ro, rd);
      let Dl = max(length(ro + rd * delta - li.pos), max(li.radius, 1e-3));   // match evalLight's 1/max(d², r²) core so samples near the lamp head are not wasted
      let thA = atan((s0 - delta) / Dl); let thB = atan((s1 - delta) / Dl);
      let steps = clamp(u32((thB - thA) / 0.11) + 3u, 4u, frame.volSteps);
      var acc = vec3f(0.0);
      for (var s = 0u; s < steps; s++) {
        let u = (f32(s) + jitter) / f32(steps);
        let th = mix(thA, thB, u);
        let t = delta + Dl * tan(th);
        let X = ro + rd * t;
        let ev = evalLight(li, X, vec3f(0.0));
        if (!ev.ok) { continue; }
        if (max(ev.E.x, max(ev.E.y, ev.E.z)) < 0.002) { continue; }
        if (occluded(X, ev.L, ev.dist.x - li.radius - 0.02, (li.owner & 0xffu) << 8u)) { continue; }
        var st = 1.0;
        if (frame.numSmoke > 0u) { st = smokeTransmittance(X, ev.L, min(ev.dist.x, 6.0)); }
        let invPdf = (thB - thA) * (Dl * Dl + (t - delta) * (t - delta)) / Dl;
        acc += ev.E * st * phaseHG(dot(rd, ev.L), 0.45) * invPdf * hazeAt(X);
      }
      acc /= f32(steps);
      inscatter += acc * sigS * li.volumetric * T;
    }
  }
  textureStore(outVol, gid.xy, vec4f(inscatter, T));
}
