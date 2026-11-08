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
    if (g > 0.0 && c > 0.0) { if (!coatOnly) { f = m * c; } if (q == 0u || !glossy) { f += coatOf(s, V, d, widen); } } else if (g < 0.0 && leaf) { f = s.through * (-min(c, 0.0) / PI); }
    f *= E; if (max3(f) <= 0.0) { continue; }
    if (q == 0u) { f *= poolLight(s.x); }
    L += f * visible(s.x + s.ng * select(-0.002, 0.002, g > 0.0), d, 1e-3, far);
  }
  return L;
}
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = vec2u(frame.size.zw); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let at = id.y * dim.x + id.x; rngInit(id.xy, 11u + pathKnobs.x * 7919u);
  let j = rnd2(); var ro = frame.camPos.xyz; var rd = camRay((vec2f(id.xy) + j) / vec2f(dim));
  var thr = vec3f(1.0); var L = vec3f(0.0); var direct = vec3f(0.0); var one = vec3f(0.0); var cut = frame.misc.y; var clear = true; var depth = 0u; var spread = length(frame.camUp.xyz) * 2.0 / frame.size.w; var fp = 0.0; var wet = false;
  for (var b = 0u; b < pathKnobs.y + 6u; b++) {
    if (depth > pathKnobs.y) { break; }
    fineLeaves = depth < 2u;
    let h = nearest(ro, rd, 1e-3, 2000.0, 0u, cut);
    if (h.gpiece != NOTHING && clear) {           // a pane in the way: some of the paths mirror off it
      let lp = toLocal(h.gpiece, ro + rd * h.gt); let ng = normalize(toWorld(h.gpiece, localNormal(h.gpiece, h.gcode, lp))); let c = abs(dot(ng, rd)); let F = 0.04 + 0.96 * pow(1.0 - c, 5.0);
      if (h.gcode != CODE_CUT && rnd() < F) { ro = ro + rd * (h.gt - 1e-3); rd = reflect(rd, ng); cut = 1e9; clear = false; depth += 1u; continue; }
    }
    var gained = vec3f(0.0);
    if (h.piece == NOTHING) { gained = thr * h.thru * sky(rd); L += gained; if (depth == 0u) { direct += gained; } else if (depth == 1u) { one += gained; } break; }
    thr *= h.thru; fp += spread * h.t;
    if (wet) { thr *= exp(-WATER_SIGMA * h.t); wet = false; }
    var s: Surf; var water = false;
    if ((h.piece & LEAF_HIT) == 0u && (pieces[h.piece].flags & WATER) != 0u && h.code == 3u) {
      water = true; let x = ro + rd * h.t; s.x = x; s.n = waveNormal(x.xz, frame.misc.x); s.ng = vec3f(0.0, 1.0, 0.0); s.rough = 0.11; s.f0 = 0.02; s.kind = 8u;
    } else { s = wetFilm(surfaceAt(ro, rd, h, select(1.0, fp, clear || fp < 0.2)), fp); }
    let V = -rd; if (dot(s.n, V) < 0.02) { s.n = normalize(s.n + V * (0.04 - dot(s.n, V))); }
    let own = (h.piece & LEAF_HIT) == 0u && (pieces[h.piece].lamp & 255u) != 255u;
    if (any(s.glow > vec3f(0.0)) && (clear || !own)) { gained = thr * s.glow; L += gained; if (depth == 0u) { direct += gained; } else if (depth == 1u) { one += gained; } }
    if (s.kind == 63u) { break; }
    let glossy = s.rough < 0.25;
    gained = thr * lampsAt(s, V, glossy); L += gained; if (depth == 0u) { direct += gained; } else if (depth == 1u) { one += gained; }
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
      let u = rnd2(); let d = wideDir(s.x + s.ng * side * 0.002, n, u, rnd()); if (d.w <= 0.0) { break; }
      thr *= alb * d.w / PI / (1.0 - pc); rd = d.xyz; if (dot(rd, s.ng) * side <= 0.0) { break; } ro = s.x + s.ng * side * 0.002; clear = false; fp = 1.0;
    }
    depth += 1u; cut = 1e9; if (!clear) { spread = 0.02; }
    if (depth > 2u) { let q = clamp(max3(thr), 0.05, 0.95); if (rnd() > q) { break; } thr /= q; }
  }
  let n = f32(pathKnobs.z);
  var acc = vec4f(0.0); if (pathKnobs.z > 0u) { acc = sums[at]; }
  acc += vec4f(max(finite4(vec4f(L, 0.0), 400.0).rgb, vec3f(0.0)), 1.0); sums[at] = acc;
  textureStore(outHdr, id.xy, vec4f(acc.xyz / acc.w, 1.0));
}
