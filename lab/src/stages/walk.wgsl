// THE REFERENCE: a path tracer over the same pieces, materials, lamps and sky as everything else. No cache, clamp, reuse or filter. One path a pixel a
// frame, summed. It also keeps direct light, one bounce and the rest apart, so that each estimator can be compared with the part it owns.
/** lamps at a surface: the sun, then one lamp drawn by weight. One visible() in the text, asked twice */
fn lampsAt(s: Surf, V: vec3f, glossy: bool) -> vec3f {
  var L = vec3f(0.0); let m = matteOf(s) * (1.0 - lum(fresnel(coatF0(s), max(dot(s.n, V), 0.02)))) / PI; let leaf = any(s.through > vec3f(0.0)); let coatOnly = s.kind == 8u;
  for (var q = 0u; q < 2u; q++) {
    var d: vec3f; var E: vec3f; var far = 400.0; var widen = 0.004;
    if (q == 0u) { if (frame.sunE.x + frame.sunE.y <= 0.0) { continue; } d = coneDir(frame.sunDir.xyz, frame.sunDir.w, rnd2()); E = frame.sunE.xyz; }
    else {
      if (coatOnly) { continue; }
      let k = drawLamp(s.x, select(s.n, vec3f(0.0), leaf)); if (k.x < 0.0) { continue; }
      let a = aimLamp(u32(k.x), s.x, rnd2()); d = a.dir; E = a.E * k.y; far = a.dist - 0.02; widen = 0.0;
    }
    let c = dot(s.n, d); let g = dot(s.ng, d); var f = vec3f(0.0);
    if (g > 0.0 && c > 0.0) { if (!coatOnly) { f = m * c * roughMatte(s.kind, s.albedo, s.n, V, d); } if (q == 0u || !glossy) { f += coatOf(s, V, d, widen); } } else if (g < 0.0 && leaf) { f = s.through * (-min(c, 0.0) / PI); }
    f *= E; if (max3(f) <= 0.0) { continue; }
    if (q == 0u) { f *= poolLight(s.x); }
    L += f * visible(s.x + s.ng * select(-0.002, 0.002, g > 0.0), d, 1e-3, far);
  }
  return L;
}
/** where a ray first meets smoke: its color there if it glows, and how far (0: it gets through). The section hides what it hides, for the eye's own ray */
fn smokeMeets(ro: vec3f, rd: vec3f, t1: f32, cut: f32) -> vec4f {
  let s = smokeSpan(ro, rd, t1); var t = s.x; let most = 13.0;
  for (var i = 0u; i < 400u; i++) { if (t >= s.y) { break; } let x = ro + rd * t; let p = smokeSlot(x); if (p == 0u || (cut < 1e8 && x.y > cut && overCut(x, cut))) { let c = floor((x - SM_AT) / SM_BRICK); let e = (SM_AT + (c + select(vec3f(0.0), vec3f(1.0), rd > vec3f(0.0))) * SM_BRICK - ro) * safeInv(rd); t = max(min3(e), t) + 0.003; continue; }
    t += -log(1.0 - rnd()) / most; if (t >= s.y) { break; } let y = ro + rd * t; let q = smokeSlot(y); if (q == 0u) { continue; } let v = smokeIn(q, y); if (rnd() * most < v.w) { return vec4f(select(vec3f(0.0), v.rgb, max3(v.rgb) > 24.0), t); } }
  return vec4f(0.0);
}
/** what comes back along a ray: the whole path tracer. `clear0` = it is the eye's, or a mirror's of it */
fn walk(ro0: vec3f, rd0: vec3f, depth0: u32, clear0: bool, cut0: f32, spread0: f32, fp0: f32, most: u32) -> vec3f {
  var ro = ro0; var rd = rd0; var thr = vec3f(1.0); var L = vec3f(0.0); var cut = cut0; var clear = clear0; var depth = depth0; var spread = spread0; var fp = fp0; var wet = false; gCount = 0u;
  for (var b = 0u; b < most + 6u; b++) {
    if (depth > most) { break; }
    fineLeaves = depth < 2u;
    let h = nearest(ro, rd, 1e-3, 2000.0, 0u, cut);
    if (h.gpiece != NOTHING && clear) {           // a pane in the way: some of the paths mirror off it
      let lp = toLocal(h.gpiece, ro + rd * h.gt); let ng = normalize(toWorld(h.gpiece, localNormal(h.gpiece, h.gcode, lp))); let c = abs(dot(ng, rd)); let F = 0.04 + 0.96 * pow(1.0 - c, 5.0);
      if (h.gcode != CODE_CUT && rnd() < F) { ro = ro + rd * (h.gt - 1e-3); rd = reflect(rd, ng); cut = 1e9; clear = false; depth += 1u; continue; }
    }
    var gained = vec3f(0.0);
    if (frame.misc2.x > 0.5) {                     // smoke: a collision, drawn as it is (delta tracking), lights it and turns it
      let c = smokeMeets(ro, rd, min(h.t, 2000.0), cut);
      if (c.w > 0.0) { let x = ro + rd * c.w; if (c.x > 24.0) { L += thr * c.xyz; break; }
        for (var q = 0u; q < 2u; q++) { var d: vec3f; var E: vec3f; var far = 400.0; if (q == 0u) { if (max3(frame.sunE.xyz) <= 0.0) { continue; } d = coneDir(frame.sunDir.xyz, frame.sunDir.w, rnd2()); E = frame.sunE.xyz; } else { let k = drawLamp(x, vec3f(0.0)); if (k.x < 0.0) { continue; } let a = aimLamp(u32(k.x), x, rnd2()); d = a.dir; E = a.E * k.y; far = a.dist - 0.02; } if (max3(E) > 0.0) { L += thr * E * visible(x, d, 1e-3, far) * (1.0 / (4.0 * PI)); } }
        let u = rnd2(); let z = 1.0 - 2.0 * u.x; let r = sqrt(max(1.0 - z * z, 0.0)); rd = vec3f(r * cos(2.0 * PI * u.y), r * sin(2.0 * PI * u.y), z); ro = x; thr *= 0.99; clear = false; fp = 1.0; spread = 0.02; cut = 1e9; depth += 1u; if (depth > 4u) { let q = clamp(max3(thr), 0.05, 0.95); if (rnd() > q) { break; } thr /= q; } continue; }
    }
    if (h.piece == NOTHING) { gained = thr * h.thru * sky(rd); L += gained; break; }
    thr *= h.thru; fp += spread * h.t;
    if (wet) { thr *= exp(-WATER_SIGMA * h.t); wet = false; }
    var s: Surf; var water = false;
    if ((h.piece & LEAF_HIT) == 0u && (pieces[h.piece].flags & WATER) != 0u && h.code == 3u) {
      water = true; let x = ro + rd * h.t; s.x = x; s.n = poolNormal(x.xz, frame.misc.x); s.ng = vec3f(0.0, 1.0, 0.0); s.rough = 0.11; s.f0 = 0.02; s.kind = 8u;
    } else { s = wetFilm(surfaceAt(ro, rd, h, select(1.0, fp, clear || fp < 0.2)), fp); }
    let V = -rd; if (dot(s.n, V) < 0.02) { s.n = normalize(s.n + V * (0.04 - dot(s.n, V))); }
    let own = (h.piece & LEAF_HIT) == 0u && (pieces[h.piece].lamp & 255u) != 255u;
    if (any(s.glow > vec3f(0.0)) && (clear || !own)) { gained = thr * s.glow; L += gained; }
    if (s.kind == 63u) { break; }
    let glossy = s.rough < 0.25;
    gained = thr * lampsAt(s, V, glossy); L += gained;
    if (water) {
      let c = max(dot(s.n, V), 0.02); let F = 0.02 + 0.98 * pow(1.0 - c, 5.0);
      if (rnd() < F) { ro = s.x + vec3f(0.0, 0.004, 0.0); rd = reflect(rd, s.n); rd.y = max(rd.y, 0.02); rd = normalize(rd); clear = false; cut = 1e9; depth += 1u; } else { ro = s.x - vec3f(0.0, 0.004, 0.0); rd = refract(rd, s.n, 0.75); wet = true; }
      continue;
    }
    // on: through the coat, or off the matte
    let F = fresnel(coatF0(s), dot(s.n, V)); let m = matteOf(s); let wf = lum(F); let wm = lum(m) * (1.0 - wf) + lum(s.through); let pc = select(clamp(wf / max(wf + wm, 1e-4), 0.08, 0.92), 1.0, wm < 1e-4);
    if (rnd() < pc) { let l = coatDir(s, V, rnd2()); thr *= l.w / pc; rd = l.dir; if (dot(rd, s.ng) <= 0.0) { break; } ro = s.x + s.ng * 0.002; clear = glossy; if (!glossy) { fp += 0.05; } }
    else {
      var n = s.n; var side = 1.0; var alb = m * (1.0 - wf);
      if (any(s.through > vec3f(0.0))) { let pt = lum(s.through) / (lum(s.through) + lum(m)); if (rnd() < pt) { n = -s.n; side = -1.0; alb = s.through / pt; } else { alb = alb / (1.0 - pt); } }
      let u = rnd2(); let from0 = s.x + s.ng * side * 0.002; let d = guidedDir(from0, n, u, rnd()); if (d.w <= 0.0) { break; }
      thr *= alb * d.w / PI / (1.0 - pc); if (side > 0.0) { thr *= roughMatte(s.kind, s.albedo, s.n, V, d.xyz); } rd = d.xyz; if (dot(rd, s.ng) * side <= 0.0) { break; } ro = from0; gMark(from0, n, rd, thr, L); clear = false; fp = 1.0;
    }
    depth += 1u; cut = 1e9; if (!clear) { spread = 0.02; }
    if (depth > 2u) { let q = clamp(max3(thr), 0.05, 0.95); if (rnd() > q) { break; } thr /= q; }
  }
  gTell(L); return L;
}
