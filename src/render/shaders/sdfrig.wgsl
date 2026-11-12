// A big body is written once here and asked in a loop: the compile pastes every call. Credit: Stephen Jayakar, from his pull request 1.
// ---- SDF character rig (render/sdfrig.ts fits it; the prelude prepends RB_QUANT — frame.ts scenePrelude). Every character's 15 shadow proxies
// (capsules[k·capsPerChar + slot], struct Capsule in common.wgsl) are ROUND CONES: radius cp.r at end a, coneRb(cp) at end b (the top byte of misc, millimetres;
// 0 = r again — an untapered cone, which is what a record nobody fitted decodes to). This file is the primitive alone — coneRb / sdRoundCone / roundConeNormal /
// isectRoundCone: what common.wgsl's proxy loops (traceCapsules / occludedCapsules) run for every ray at every tier, unconditionally. (Two trials stood on it
// until 2026-11-07 and went: a smooth-union closest hit for reflections, and a drawn soft-shadow query for the direct light.)
// ---- §1 the round-cone primitive ----
/** radius at end b: the fitted r1 in millimetres from misc's top byte, or the record's own r when nobody packed one (an untapered cone = the game's hand capsule) */
fn coneRb(cp: Capsule) -> f32 { let q = capsuleWordRb(cp.misc); return select(f32(q) * RB_QUANT, cp.r, q == 0u); }

/** exact signed distance from p to the round cone (a, ra) → (b, rb) — iq's sdRoundCone (one square root); the CPU twin is sdfrig.ts sdRoundCone */
fn sdRoundCone(p: vec3f, a: vec3f, b: vec3f, ra: f32, rb: f32) -> f32 {
  let ba = b - a; let l2 = dot(ba, ba); let rr = ra - rb;
  if (l2 < 1e-10) { return length(p - a) - max(ra, rb); }   // a degenerate segment is a sphere (a coincident-joint frame, or a padding record: r 0 → the point's distance, never 'inside everywhere')
  let a2 = l2 - rr * rr; let il2 = 1.0 / l2;
  let pa = p - a; let y = dot(pa, ba); let z = y - l2;
  let q = pa * l2 - ba * y; let x2 = dot(q, q);
  let y2 = y * y * l2; let z2 = z * z * l2;
  let k = sign(rr) * rr * rr * x2;
  if (a2 <= 0.0) { return min(length(pa) - ra, length(p - b) - rb); }   // one end's sphere holds the other: the solid is the bigger sphere
  if (sign(z) * a2 * z2 > k) { return sqrt(x2 + z2) * il2 - rb; }
  if (sign(y) * a2 * y2 < k) { return sqrt(x2 + y2) * il2 - ra; }
  return (sqrt(x2 * a2 * il2) + y * rr) * il2 - ra;
}
/** outward unit normal of the round cone at (or near) a point p on its surface: the gradient of sdRoundCone by region (cap a, cap b, flank) */
fn roundConeNormal(p: vec3f, a: vec3f, b: vec3f, ra: f32, rb: f32) -> vec3f {
  let ba = b - a; let l2 = dot(ba, ba); let rr = ra - rb; let a2 = l2 - rr * rr;
  let pa = p - a; let y = dot(pa, ba); let z = y - l2;
  let q = pa * l2 - ba * y; let x2 = dot(q, q);
  let k = sign(rr) * rr * rr * x2;
  if (sign(z) * a2 * z * z * l2 > k) { return normalize(p - b); }
  if (sign(y) * a2 * y * y * l2 < k) { return normalize(pa); }
  // flank: perpendicular to the generator — the axis-perpendicular direction tilted back along the axis by the taper (rr / sqrt(a2) per unit length)
  let perp = q * inverseSqrt(max(x2, 1e-20));            // unit, ⟂ axis, toward p
  return normalize(perp * sqrt(max(a2, 0.0)) + ba * (rr * inverseSqrt(max(l2, 1e-12))));   // (cos of the taper × perp + sin of it × the axis: sin = rr / L, cos = √a2 / L)
}

// Ray vs round cone — iq's iRoundedCone, plus an origin-INSIDE answer (occluded from t = 0: the contact darkness under a prone body, as the box proxies and
// the capsules before it answered) and the ray re-centred near the cone before the quadratic (its dot products then span a metre, not the sixty a moonlit
// shadow ray does). Returns t of the entry point in front of the origin, or −1.
fn isectRoundCone(roIn: vec3f, rd: vec3f, pa: vec3f, pb: vec3f, ra: f32, rb: f32) -> f32 {
  let baL = pb - pa;
  let reach = sqrt(dot(baL, baL)) + max(ra, rb);
  let tc = dot(pa - roIn, rd) - reach - 0.01;          // everything of the cone lies past tc along the ray (a centimetre short of its bounding sphere's near pole, so an on-axis entry is not at local t ≈ 0)
  var t0 = 0.0; var ro = roIn;
  if (tc > 0.0) { t0 = tc; ro = roIn + rd * tc; }
  else if (sdRoundCone(roIn, pa, pb, ra, rb) < 0.0) { return 0.0; }   // (only an origin within `reach` of end a can be inside)
  let ba = baL; let oa = ro - pa; let ob = ro - pb; let rr = ra - rb;
  let m0 = dot(ba, ba); let m1 = dot(ba, oa); let m2 = dot(ba, rd); let m3 = dot(rd, oa); let m5 = dot(oa, oa); let m6 = dot(ob, rd); let m7 = dot(ob, ob);
  let d2 = m0 - rr * rr;
  let k2 = d2 - m2 * m2;
  let k1 = d2 * m3 - m1 * m2 + m2 * rr * ra;
  let k0 = d2 * m5 - m1 * m1 + m1 * rr * ra * 2.0 - m0 * ra * ra;
  let h = k1 * k1 - k0 * k2;
  // the flank quadratic degenerates as the ray comes within the cone's half-angle of its axis (k2 → 0: h's sign is cancellation noise and t = x ∕ ≈0);
  // there the solid is entered through a cap anyway, so only a well-conditioned quadratic may call a miss or a flank hit — the caps decide the rest
  if (k2 > 1e-6 * m0) {
    if (h < 0.0) { return -1.0; }
    let t = (-sqrt(h) - k1) / k2;
    let y = m1 - ra * rr + t * m2;
    if (y > 0.0 && y < d2) { return select(-1.0, t + t0, t >= 0.0); }   // the flank (behind the origin: the whole solid is — the inside case left above)
  }
  let h1 = m3 * m3 - m5 + ra * ra;
  let h2 = m6 * m6 - m7 + rb * rb;
  var best = -1.0;
  if (h1 > 0.0) { let t1 = -m3 - sqrt(h1); if (t1 >= 0.0) { best = t1; } }
  if (h2 > 0.0) { let t2 = -m6 - sqrt(h2); if (t2 >= 0.0 && (best < 0.0 || t2 < best)) { best = t2; } }
  return select(-1.0, best + t0, best >= 0.0);
}
