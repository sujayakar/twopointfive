// What is at a hit: its point, its normals, its material. Detail fades with the footprint `fp` (metres), so a bounce ray reads each material's mean.
struct Surf { x: vec3f, ng: vec3f, n: vec3f, albedo: vec3f, rough: f32, metal: f32, f0: f32, glow: vec3f, through: vec3f, kind: u32, piece: u32, flags: u32 };

fn vnoise(p: vec2f) -> f32 { let i = floor(p); let f = p - i; let w = f * f * f * (f * (f * 6.0 - 15.0) + 10.0); return mix(mix(hash21(i), hash21(i + vec2f(1.0, 0.0)), w.x), mix(hash21(i + vec2f(0.0, 1.0)), hash21(i + vec2f(1.0, 1.0)), w.x), w.y); }
fn fbm(p: vec2f) -> f32 { return 0.5 * vnoise(p) + 0.25 * vnoise(p * 2.03 + 7.1) + 0.15 * vnoise(p * 4.1 + 3.7) + 0.1 * vnoise(p * 8.3 + 1.3); }
/** distance to the nearest line of a grid, in metres */
fn gridLine(p: vec2f, cell: vec2f) -> f32 { let q = abs(fract(p / cell + 0.5) - 0.5) * cell; return min(q.x, q.y); }
fn band(d: f32, w: f32, fp: f32) -> f32 { return 1.0 - smoothstep(w - fp * 0.5, w + fp * 0.5 + 1e-4, d); }
/** the height of a material's relief, metres: what is cut into it by rule, and two sizes of unevenness */
/** how much of a detail that repeats `freq` times a metre a pixel fp metres wide can show: none of it once two no longer fit */
fn shows(freq: f32, fp: f32) -> f32 { return 1.0 - smoothstep(0.18, 0.5, freq * fp); }
/** upholstery: soft channels a hand wide with a seam between, over a loose weave */
const CHANNEL = 0.13;
fn channel(p: vec2f) -> f32 { return abs(sin(p.x * (PI / CHANNEL))); }
fn relief(kind: u32, p: vec2f, s: f32, fp: f32) -> f32 {
  var a = vec4f(55.0, 55.0, 0.00022, 130.0); var h = 0.0; var b = 0.0001;      // (the coarse noise's frequency x, y, its height, the fine one's frequency)
  switch (kind) {
    case 1u: { a = vec4f(9.0, 9.0, 0.0016, 120.0); b = 0.0004; let g = gridLine(p, vec2f(2.4, 1.2)); let c = abs(fract(p / vec2f(2.4, 1.2)) - 0.5) * vec2f(2.4, 1.2) - vec2f(0.8, 0.3); h = -0.008 * band(g, 0.006, 0.004) - 0.010 * band(length(c), 0.018, 0.006); }
    case 2u: { a = vec4f(2.3, 2.3, 0.0007, 21.0); b = 0.00012; }
    case 3u: { a = vec4f(1.7, 1.7, 0.0004, 60.0); b = 0.00005; h = -0.003 * band(gridLine(p, vec2f(s)), 0.003, 0.003); }
    case 5u: { a = vec4f(5.0, 5.0, 0.0012, 60.0); b = 0.0004; h = 0.004 * band(gridLine(p, vec2f(1.5, 60.0)), 0.02, 0.02); }
    case 6u: { a = vec4f(3.0, 60.0, 0.0008, 90.0); b = 0.0001; h = -0.006 * band(abs(fract(p.y / 0.14) - 0.5) * 0.14, 0.004, 0.003); }
    case 10u: { a = vec4f(34.0, 34.0, 0.0012, 90.0); b = 0.0; h = 0.014 * sqrt(channel(p)) * shows(1.0 / CHANNEL, fp); }
    case 21u: { a = vec4f(9.0, 9.0, 0.006, 60.0); b = 0.0; }
    case 11u: { a = vec4f(14.0, 14.0, 0.0011, 90.0); b = 0.0003; }
    case 13u: { a = vec4f(12.0, 12.0, 0.02, 60.0); b = 0.004; }
    case 15u: { a = vec4f(26.0, 5.0, 0.012, 60.0); b = 0.002; }
    case 16u: { a = vec4f(38.0, 38.0, 0.006, 90.0); b = 0.003; }
    default: {}
  }
  let f = max(a.x, a.y); return h + a.z * (0.65 * shows(f, fp) * vnoise(p * a.xy) + 0.35 * shows(f * 2.7, fp) * vnoise(p * a.xy * 2.7 + 5.0)) + b * shows(a.w, fp) * vnoise(p * a.w);
}
/** a leaf sends on less than it gets, both sides together, whatever its tint asks: else a crown of them would light itself */
fn leafKeeps(all: vec3f) -> f32 { return min(1.0, 0.88 / max(max3(all), 1e-3)); }
fn leafSurf(h: Hit, x: vec3f, rd: vec3f) -> Surf {
  let I = &insts[h.piece & 0x7fffffffu]; if (h.code == NOTHING) { var q: Surf; let t = unpackTint((*I).tint); q.x = x; q.ng = -rd; q.n = -rd; q.albedo = t * t; q.through = t * t * vec3f(0.9, 1.1, 0.35) * 0.85; let k = leafKeeps(q.albedo + q.through); q.albedo *= k; q.through *= k; q.rough = 1.0; q.f0 = 0.0; q.kind = 14u; q.piece = h.piece; q.flags = MOVES; return q; }
  let b = bitcast<vec4u>(extra[h.code * 2u + 1u]); let ln = octUnpack(b.x);
  // where on the leaf: u along the rib, v across. It is folded along the rib and curls toward the tip
  let la = extra[h.code * 2u]; let tl = octUnpack(b.y); let sl = cross(ln, tl); let lr = vec3f(dot((*I).r0.xyz, x) + (*I).r0.w, dot((*I).r1.xyz, x) + (*I).r1.w, dot((*I).r2.xyz, x) + (*I).r2.w) - la.xyz; let u = clamp(dot(lr, tl) / la.w, -1.0, 1.0); let v = dot(lr, sl) / la.w;
  let shape = select(0.0, 1.0, b.w == 0u); let lf = ln + (sl * clamp(v * 7.0, -0.34, 0.34) + tl * (0.30 * u)) * shape;
  var g = normalize((*I).r0.xyz * ln.x + (*I).r1.xyz * ln.y + (*I).r2.xyz * ln.z); var n = normalize((*I).r0.xyz * lf.x + (*I).r1.xyz * lf.y + (*I).r2.xyz * lf.z); if (dot(g, rd) > 0.0) { g = -g; n = -n; } if (dot(n, rd) > -0.05) { n = normalize(n - rd * (dot(n, rd) + 0.1)); }
  var s: Surf; s.x = x; s.ng = g; s.n = n; let t = unpackTint((*I).tint); let tone = bitcast<f32>(b.z);
  if (b.w == 2u) { s.albedo = vec3f(0.16, 0.115, 0.075) * mix(0.7, 1.2, tone); s.through = vec3f(0.0); s.rough = 0.85; s.f0 = 0.03; s.metal = 0.0; s.glow = vec3f(0.0); s.kind = 15u; s.piece = h.piece; s.flags = MOVES; return s; }
  s.albedo = t * t * mix(0.58, 1.28, tone) * select(1.0, 0.7, b.w == 1u) * mix(1.0, 0.86 + 0.14 * u + 0.4 * (1.0 - smoothstep(0.012, 0.035, abs(v))), shape); s.through = s.albedo * vec3f(0.95, 1.15, 0.35) * select(0.85, 0.5, b.w == 1u); { let k = leafKeeps(s.albedo + s.through); s.albedo *= k; s.through *= k; } s.rough = 0.40; s.f0 = 0.04; s.metal = 0.0; s.glow = vec3f(0.0); s.kind = select(14u, 20u, b.w == 1u); s.piece = h.piece; s.flags = MOVES; return s;
}
fn localNormal(k: u32, code: u32, lp: vec3f) -> vec3f {
  if (code < 6u) { var n = vec3f(0.0); n[code >> 1u] = select(-1.0, 1.0, (code & 1u) == 1u); return n; }
  if (code == CODE_QUAD) { let q = pieces[k].quad; return normalize(vec3f(q.x * lp.x, q.y * lp.y + 0.5 * q.w, q.z * lp.z)); }
  return extra[pieces[k].planes + code - 7u].xyz;
}
fn toWorld(k: u32, v: vec3f) -> vec3f { return pieces[k].r0.xyz * v.x + pieces[k].r1.xyz * v.y + pieces[k].r2.xyz * v.z; }
fn toLocal(k: u32, x: vec3f) -> vec3f { let p = &pieces[k]; return vec3f(dot((*p).r0.xyz, x) + (*p).r0.w, dot((*p).r1.xyz, x) + (*p).r1.w, dot((*p).r2.xyz, x) + (*p).r2.w); }

fn surfaceAt(ro: vec3f, rd: vec3f, h: Hit, fp: f32) -> Surf {
  let x = ro + rd * h.t;
  if ((h.piece & LEAF_HIT) != 0u) { return leafSurf(h, x, rd); }
  let k = h.piece; let p = &pieces[k]; let m = mats[(*p).mat & 255u]; let lp = toLocal(k, x);
  var s: Surf; s.x = x; s.piece = k; s.flags = (*p).flags; s.kind = m.kind; s.through = vec3f(0.0);
  if (h.code == CODE_CUT) { s.ng = vec3f(0.0, 1.0, 0.0); s.n = s.ng; s.albedo = vec3f(0.012, 0.013, 0.016); s.rough = 0.6; s.f0 = 0.04; s.metal = 0.0; s.glow = vec3f(0.0); s.kind = 63u; return s; }
  let ln = localNormal(k, h.code, lp); s.ng = normalize(toWorld(k, ln)); s.n = s.ng;
  let tint = unpackTint((*p).tint); let tint2 = tint * tint;
  var base = m.base * tint2; s.rough = m.rough; s.metal = m.metal; s.f0 = m.f0;
  let on = select(1.0, lamps[(*p).lamp & 63u].on, ((*p).lamp & 255u) != 255u);
  s.glow = vec3f(0.0); if (m.glow > 0.0) { s.glow = m.base * tint2 * (f32((*p).lamp >> 8u) * 0.1 * on); }   // (a glowing piece's seed is its brightness, in tenths)
  if (fp > 0.25) { s.albedo = base; if (m.kind == 7u) { s.albedo = mix(m.base, m.b2, 0.4); if (abs(s.ng.y) < 0.5) { s.glow = vec3f(0.96, 0.80, 0.57) * (frame.knobs.w * 0.063); } } return s; }     // (a tower's lit windows, all together: a fifth of them, on half of the wall)
  // where on the face, in metres: laid by the world, or in the piece's own frame if it moves
  let moves = (s.flags & MOVES) != 0u; let an = abs(select(s.ng, ln, moves)); let q = select(x, lp, moves);
  var T = vec3f(1.0, 0.0, 0.0); var Bt = vec3f(0.0, 0.0, 1.0); var uv = q.xz;
  if (an.x > an.y && an.x > an.z) { T = vec3f(0.0, 0.0, 1.0); Bt = vec3f(0.0, 1.0, 0.0); uv = q.zy; } else if (an.z > an.y) { T = vec3f(1.0, 0.0, 0.0); Bt = vec3f(0.0, 1.0, 0.0); uv = q.xy; }
  let fade = 1.0 - smoothstep(0.01, 0.06, fp); let seed = f32((*p).lamp >> 8u);
  // three noises serve every material: broad, middle, fine
  var fr = vec3f(0.9, 4.3, 7.0); var st = vec2f(1.0);
  switch (m.kind) { case 5u: { fr = vec3f(0.45, 1.1, 5.0); } case 6u: { fr = vec3f(2.0, 2.0, 9.0); st = vec2f(1.0, 22.0); } case 13u: { fr = vec3f(4.0, 9.0, 22.0); } case 15u: { fr = vec3f(4.0, 4.0, 9.0); st = vec2f(4.5, 1.0); } case 16u: { fr = vec3f(3.0, 30.0, 70.0); } case 4u: { fr = vec3f(2.0, 2.0, 2.0); st = vec2f(1.0, 60.0); } default: {} }
  let nA = fbm(uv * fr.x + seed * 1.7); let nB = fbm(uv * fr.y * st + 11.0 + seed); let nC = vnoise(uv * fr.z * st + 3.0);
  let foot = select(0.0, 1.0 - smoothstep(0.0, 0.13, q.y - floor(q.y / 3.4) * 3.4), an.y < 0.5);
  switch (m.kind) {
    case 0u: { base *= 0.97 + 0.06 * nA; s.rough *= 0.88 + 0.36 * nC; base *= 1.0 - 0.12 * foot * nC; }
    case 1u: { let cell = floor(uv / vec2f(2.4, 1.2)); base *= (0.94 + 0.08 * hash21(cell + seed)) * (0.9 + 0.2 * nA); let g = gridLine(uv, vec2f(2.4, 1.2)); base *= 1.0 - 0.45 * band(g, 0.006, fp); let c = abs(fract(uv / vec2f(2.4, 1.2)) - 0.5) * vec2f(2.4, 1.2) - vec2f(0.8, 0.3); base *= 1.0 - 0.6 * band(length(c), 0.016, fp);
      base *= 1.0 - 0.10 * smoothstep(0.5, 0.75, nB) * select(0.4, 1.0, an.y < 0.5) - 0.12 * foot; s.rough = clamp(s.rough + 0.16 * (nB - 0.5), 0.3, 1.0); }
    case 2u: { base *= 0.95 + 0.1 * nA; let scuff = smoothstep(0.56, 0.74, nB); s.rough = mix(s.rough, s.rough * 2.6, scuff * 0.8); }
    case 3u: { let cell = floor(uv / m.scale); base *= 0.93 + 0.1 * hash21(cell + 3.0); let g = gridLine(uv, vec2f(m.scale)); let k2 = band(g, 0.003, fp); base = mix(base, m.b2, k2); s.rough = mix(s.rough * (0.8 + 0.5 * hash21(cell)), 0.8, k2); base *= 0.96 + 0.08 * nA; }
    case 4u: { s.rough *= 0.92 + 0.16 * nC; base *= 0.95 + 0.08 * nA; }
    case 5u: { base *= 0.92 + 0.12 * nA; let g = abs(fract(uv.x / 1.5 + 0.5) - 0.5) * 1.5; base *= 1.0 - 0.10 * band(g, 0.02, fp); let pud = smoothstep(0.52, 0.66, nB); s.rough = mix(s.rough, 0.36, pud * 0.6); base *= 1.0 - 0.10 * pud; }
    case 6u: { let pl = floor(uv.y / 0.14); base *= 0.78 + 0.34 * hash21(vec2f(pl, floor(uv.x / 2.1 + hash21(vec2f(pl, 5.0))))); base *= 0.9 + 0.2 * nC; base *= 1.0 - 0.75 * band(0.07 - abs(abs(fract(uv.y / 0.14) - 0.5) * 0.14), 0.004, fp); }
    case 7u: { // a tower across the street: floors of windows, some of them lit
      if (an.y < 0.5) { let cw = vec2f(1.6, 3.2); let c = floor(uv / cw + seed); let f = fract(uv / cw + seed); let win = band(abs(f.x - 0.5), 0.40, fp / 1.6) * band(abs(f.y - 0.52), 0.33, fp / 3.2); base = mix(m.base * tint2, m.b2, win); s.rough = mix(0.8, 0.08, win); let r = hash33(vec3f(c, seed)); let lit = step(0.80, r.x); let warm = mix(vec3f(1.0, 0.74, 0.42), vec3f(0.82, 0.92, 1.0), step(0.75, r.y)); s.glow = warm * (win * lit * frame.knobs.w * (0.25 + 0.9 * r.z) * mix(1.0, 0.35, step(0.6, r.y) * step(f.y, 0.5))); }
      else { base *= 0.85 + 0.2 * nA; } }
    case 10u: { let w = vnoise(uv * 34.0) - 0.5; base *= (0.94 + 0.22 * w * shows(34.0, fp) + 0.10 * (nB - 0.5)) * (1.0 - 0.42 * (1.0 - smoothstep(0.0, 0.22, channel(uv))) * shows(1.0 / CHANNEL, fp)); }
    case 21u: { base *= 0.95 + 0.1 * nB; }
    case 11u: { base *= (0.95 + 0.07 * nA) * (1.0 - 0.08 * foot * nC); }
    case 12u: { let g = gridLine(uv, vec2f(0.12)); s.glow *= mix(1.0, 0.86, band(g, 0.004, fp)); }
    case 13u, 15u, 16u: { base *= 0.6 + 0.7 * nB; }
    case 19u: { let z = step(0.5, fract((uv.x + uv.y) / 0.36)); base = mix(m.b2, m.base * tint2, z); }
    default: { base *= 0.97 + 0.05 * nA; }
  }
  if (fade > 0.0 && h.code != CODE_QUAD) {
    let e = max(0.0016, fp * 0.6); var hh = vec3f(0.0); for (var i = 0u; i < 3u; i++) { hh[i] = relief(m.kind, uv + vec2f(select(0.0, e, i == 1u), select(0.0, e, i == 2u)), m.scale, fp); }
    var tw = T; var bw = Bt; if (moves) { tw = toWorld(k, T); bw = toWorld(k, Bt); }
    let g = clamp(vec2f(hh.y - hh.x, hh.z - hh.x) / e, vec2f(-0.9), vec2f(0.9)) * fade; s.n = normalize(s.ng - tw * g.x - bw * g.y);
  }
  // BEVELS: near an edge that stands free the normal bends to the next face's, over a hand's-breadth of pixels at most. Which edges stand free was found once
  if (h.code < 6u) {
    let a = h.code >> 1u; let hf = (*p).half; let sg = select(vec3u(0u), vec3u(1u), lp > vec3f(0.0)); var bend = vec3f(0.0);
    for (var k2 = 1u; k2 < 3u; k2++) {
      let b = (a + k2) % 3u; let c = 3u - a - b; let i1 = sg[(c + 1u) % 3u]; let i2 = sg[(c + 2u) % 3u]; if ((((*p).flags >> (16u + c * 4u + i1 + 2u * i2)) & 1u) == 0u) { continue; }
      let w = min(max(0.010, fp * 0.9), 0.22 * min(hf[b], hf[a])); let d = hf[b] - abs(lp[b]); if (d < w) { let t = 1.0 - d / w; bend[b] = select(-1.0, 1.0, sg[b] == 1u) * t * t; }
    }
    if (any(bend != vec3f(0.0))) { s.n = normalize(s.n + toWorld(k, bend)); }
  }
  // what is painted on it
  let nm = ((*p).mat >> 8u) & 15u; let m0 = (*p).mat >> 12u;
  for (var i = 0u; i < nm; i++) {
    let a = extra[m0 + i * 3u]; let b = extra[m0 + i * 3u + 1u]; let c = extra[m0 + i * 3u + 2u];
    if (u32(c.w) != h.code) { continue; }
    var f = lp.xz; if (h.code < 2u) { f = lp.zy; if (h.code == 1u) { f.x = -f.x; } } else if (h.code > 3u) { f = lp.xy; if (h.code == 4u) { f.x = -f.x; } }
    let r = (f - a.xy) / a.zw; if (abs(r.x) > 0.5 || abs(r.y) > 0.5) { continue; }
    let t = vec2f(r.x + 0.5, select(0.5 - r.y, r.y + 0.5, h.code == 2u || h.code == 3u));
    let px = b.zw * 2048.0; let lod = log2(max(1.0, fp * px.x / a.z));
    let art = textureSampleLevel(artTex, linSamp, b.xy + t * b.zw, min(lod, 6.0));
    let wear = 1.0 - 0.25 * smoothstep(0.5, 0.72, nB);
    let al = art.a * wear; base = mix(base, c.rgb * c.rgb * art.rgb, al); s.rough = mix(s.rough, min(s.rough, 0.42), al); if (s.metal > 0.5) { s.metal *= 1.0 - al; }
  }
  s.albedo = clamp(base, vec3f(0.0), vec3f(0.94)); return s;
}
/** under the open sky? (the building's own rooms are dry) */
fn openAir(x: vec3f) -> bool { let room = abs(x.x) < 12.0 && abs(x.z) < 8.0 && (x.y < 3.3 || (x.y < 6.55 && x.x < 4.0)); return !room; }
/** THE WET FILM, laid over any material the rain reaches: darker, smoother, and where the ground is low a mirror with rings in it */
fn wetFilm(s0: Surf, fp: f32) -> Surf {
  var s = s0; let w = frame.weather.x; if (w <= 0.0 || !openAir(s.x + s.ng * 0.02) || s.kind == 14u || s.kind == 20u) { return s; }
  let up = sat(s.ng.y); let low = fbm(s.x.xz * 0.42 + 5.0); let pud = w * smoothstep(0.47 - 0.05 * w, 0.545, low) * smoothstep(0.9, 0.99, up); let film = w * (0.35 + 0.65 * up);
  let lumA = lum(s.albedo); s.albedo *= mix(1.0, 0.52 + 0.25 * lumA, film * select(1.0, 0.4, s.metal > 0.5)); s.rough = mix(s.rough, max(0.09, s.rough * 0.30), film); s.f0 = max(s.f0, mix(s.f0, 0.03, film));
  if (pud > 0.0) {
    s.rough = mix(s.rough, 0.03, pud); s.albedo *= mix(1.0, 0.75, pud); var n = vec2f(0.0);
    if (frame.weather.y > 0.0 && fp < 0.12) { for (var k = 0u; k < 2u; k++) { let g = s.x.xz * 2.6 + f32(k) * 17.3; let c = floor(g); let h = hash33(vec3f(c, f32(k) + 3.0)); let t = fract(frame.misc.x * (0.9 + 0.5 * h.x) + h.y); let d = g - c - 0.25 - 0.5 * h.yz; let r = length(d); let ring = sin((r - t * 0.5) * 60.0) * exp(-abs(r - t * 0.5) * 22.0) * (1.0 - t); n += d / max(r, 1e-3) * ring * 0.10 * frame.weather.y; } }
    s.n = normalize(mix(s.n, normalize(s.ng + vec3f(n.x, 0.0, n.y)), pud));
  }
  return s;
}
/** the same for a ray that only wants the mean: no patterns, no paint */
fn surfaceMean(ro: vec3f, rd: vec3f, h: Hit) -> Surf { var s = surfaceAt(ro, rd, h, 1.0); if (frame.weather.x > 0.0 && openAir(s.x + s.ng * 0.02)) { s.albedo *= mix(1.0, 0.62, frame.weather.x); } return s; }

// ================================================================ lamps
struct Aim { dir: vec3f, dist: f32, E: vec3f };       // toward a point of a lamp: what it would send a surface facing it, per unit of 1 / pdf
fn aimLamp(i: u32, x: vec3f, u: vec2f) -> Aim {
  let l = &lamps[i]; var a: Aim; a.E = vec3f(0.0); a.dist = 0.0; a.dir = vec3f(0.0, 1.0, 0.0);
  if ((*l).kind == 0u) {                                                  // a rectangle, shining one way
    let q = (*l).at + (*l).u * (u.x * 2.0 - 1.0) + (*l).v * (u.y * 2.0 - 1.0); let d = q - x; let d2 = dot(d, d); a.dist = sqrt(d2); a.dir = d / max(a.dist, 1e-4);
    let c = -dot(a.dir, (*l).n); if (c <= 0.0) { return a; }
    a.E = (*l).radiance * ((*l).on * c * (*l).area / max(d2, 0.0025));
  } else {
    let d0 = (*l).at - x; let dd = length(d0); let r = (*l).area; let b = basis(d0 / max(dd, 1e-4)); let s = sqrt(u.x) * r; let an = u.y * 2.0 * PI;
    let d = d0 + b[0] * (s * cos(an)) + b[1] * (s * sin(an)); let d2 = dot(d, d); a.dist = sqrt(d2); a.dir = d / max(a.dist, 1e-4);
    var k = (*l).on * PI * r * r / max(d2, r * r);
    if ((*l).kind == 2u) { let c = -dot(a.dir, (*l).n); k *= smoothstep((*l).cone, mix((*l).cone, 1.0, 0.55), c) * (0.35 + 0.65 * smoothstep(0.9, 1.0, c)); }
    a.E = (*l).radiance * k;
  }
  return a;
}
/** the light a lamp lands on a surface at x facing n (n = 0: a leaf, either side) with nothing in the way: exact for a rectangle (its edges' integral) */
fn lampE(i: u32, x: vec3f, n: vec3f) -> vec3f {
  let l = &lamps[i]; if ((*l).on <= 0.0) { return vec3f(0.0); } let d = (*l).at - x; let dist = length(d); if (dist > (*l).reach) { return vec3f(0.0); } let leaf = all(n == vec3f(0.0));
  let win = sat(1.0 - pow(dist / (*l).reach, 4.0));
  if ((*l).kind != 0u || leaf) { let a = aimLamp(i, x, vec2f(0.5)); let c = dot(n, a.dir); return a.E * select(max(c, 0.0), 0.5, leaf) * win; }
  let face = dot(d, (*l).n); if (face >= 0.0) { return vec3f(0.0); }
  if (dist * dist > 14.0 * (*l).area) { return (*l).radiance * ((*l).on * max(dot(n, d), 0.0) * -face * (*l).area * win / (dist * dist * dist * dist)); }     // (far off, a rectangle is a point: to a hundredth)
  let b = basis(n); var v: array<vec3f, 6>; var m = 0u; var q: array<vec3f, 4>;
  for (var k = 0u; k < 4u; k++) { let c = d + (*l).u * select(-1.0, 1.0, k == 1u || k == 2u) + (*l).v * select(-1.0, 1.0, k >= 2u); q[k] = vec3f(dot(c, b[0]), dot(c, b[1]), dot(c, n)); }
  for (var k = 0u; k < 4u; k++) { let a = q[k]; let e = q[(k + 1u) & 3u]; if (a.z > 0.0) { v[m] = a; m++; } if ((a.z > 0.0) != (e.z > 0.0)) { v[m] = mix(a, e, a.z / (a.z - e.z)); m++; } }
  if (m < 3u) { return vec3f(0.0); } var s = 0.0;
  for (var k = 0u; k < m; k++) { let a = normalize(v[k]); let e = normalize(v[select(k + 1u, 0u, k + 1u == m)]); let c = cross(a, e); let sl = length(c); s += c.z * select(1.0, atan2(sl, dot(a, e)) / sl, sl > 1e-5); }
  return (*l).radiance * ((*l).on * abs(s) * 0.5 * win);
}
/** how much of a lamp's image in a coat is the formula's (with a ray to see if it is there); the rest is for mirror rays to find. Rough coats would only find a small lamp by luck */
fn byFormula(rough: f32) -> f32 { return smoothstep(0.12, 0.24, rough); }
/** a lamp seen in a coat: by the point of it nearest the mirror direction */
fn lampCoat(i: u32, s: Surf, V: vec3f) -> vec3f {
  let l = &lamps[i]; if ((*l).on <= 0.0) { return vec3f(0.0); } let d0 = (*l).at - s.x; if (length(d0) > (*l).reach) { return vec3f(0.0); } var u = vec2f(0.5); let R = reflect(-V, s.n); var widen = 0.0;
  if ((*l).kind == 0u) { let den = dot(R, (*l).n); if (den > -1e-3) { u = vec2f(0.5); } else { let t = dot(d0, (*l).n) / den; let q = s.x + R * t - (*l).at; u = clamp(vec2f(dot(q, (*l).u) / dot((*l).u, (*l).u), dot(q, (*l).v) / dot((*l).v, (*l).v)) * 0.5 + 0.5, vec2f(0.0), vec2f(1.0)); } }
  let a = aimLamp(i, s.x, u); widen = sqrt(max((*l).area, 1e-4)) * select(1.0, 0.35, (*l).kind == 0u) / (2.0 * max(a.dist, 0.2)); return a.E * coatOf(s, V, a.dir, widen);
}
/** what a lamp would send x with nothing in the way: its brightness, for choosing */
fn lampWeight(i: u32, x: vec3f, n: vec3f) -> f32 {
  let l = &lamps[i]; let d = (*l).at - x; let dist = length(d); if (dist > (*l).reach || (*l).on <= 0.0) { return 0.0; }
  let a = aimLamp(i, x, vec2f(0.5)); let c = dot(n, a.dir);
  return lum(a.E) * select(max(c, 0.0) + 0.12 * select(0.0, 1.0, (*l).kind == 0u && c > -0.6), 0.5, all(n == vec3f(0.0)));
}
var<private> byReach: bool = false;                                 // (the reference asks every lamp)
/** one lamp drawn in proportion to what it would send: its number and 1 / chance, or none */
fn drawLamp(x: vec3f, n: vec3f) -> vec2f {
  var W = 0.0; var pick = -1.0; var w0 = 0.0;
  var near = vec4f(1.0);
  for (var i = 0u; i < frame.counts.x; i++) { if (byReach && (i & 3u) == 0u) { near = lampReach(x, i >> 2u); } if (near[i & 3u] <= 0.0) { continue; } let w = lampWeight(i, x, n) * near[i & 3u]; if (w <= 0.0) { continue; } W += w; if (rnd() * W < w) { pick = f32(i); w0 = w; } }
  return vec2f(pick, select(0.0, W / w0, w0 > 0.0));
}
// ================================================================ openings: where the sky is, for rays that should look there
fn openingPdf(x: vec3f, d: vec3f) -> f32 {
  var p = 0.0; let n = frame.counts.y; if (n == 0u) { return 0.0; }
  var W = 0.0;
  for (var i = 0u; i < n; i++) { let w = openingWeight(i, x); if (w <= 0.0) { continue; } W += w;
    let o = &openings[i]; let den = dot(d, (*o).n); if (den >= -1e-5) { continue; }
    let t = dot((*o).at - x, (*o).n) / den; if (t <= 0.0) { continue; }
    let q = x + d * t - (*o).at; let uu = dot((*o).u, (*o).u); let vv = dot((*o).v, (*o).v);
    if (abs(dot(q, (*o).u)) > uu || abs(dot(q, (*o).v)) > vv) { continue; }
    p += w * t * t / (-den * (*o).area); }
  return select(0.0, p / W, W > 0.0);
}
fn openingWeight(i: u32, x: vec3f) -> f32 { let o = &openings[i]; let d = x - (*o).at; if (dot(d, (*o).n) <= 0.02) { return 0.0; } let d2 = dot(d, d); if (d2 > 400.0) { return 0.0; } return (*o).area * dot(d, (*o).n) * inverseSqrt(d2) / (d2 + 1.0); }
fn openingDir(x: vec3f, u: vec3f) -> vec3f {
  var W = 0.0; let n = frame.counts.y; for (var i = 0u; i < n; i++) { W += openingWeight(i, x); }
  if (W <= 0.0) { return vec3f(0.0); }
  var r = u.z * W; for (var i = 0u; i < n; i++) { let w = openingWeight(i, x); if (w <= 0.0) { continue; } r -= w; if (r <= 0.0) { let o = &openings[i]; return normalize((*o).at + (*o).u * (u.x * 2.0 - 1.0) + (*o).v * (u.y * 2.0 - 1.0) - x); } }
  return vec3f(0.0);
}
/** a direction for wide incoming light: by the cosine, or through an opening; w = cosine / pdf, weighed so that both may find the same sky (one-sample MIS) */
fn wideDir(x: vec3f, n: vec3f, u: vec2f, pick: f32) -> vec4f {
  var any = 0.0; for (var i = 0u; i < frame.counts.y; i++) { any += openingWeight(i, x); }
  let share = select(0.0, clamp(any * 1.2, 0.15, 0.5), any > 0.0005);
  var d: vec3f;
  if (pick < share) { d = openingDir(x, vec3f(u, rnd())); if (dot(d, n) <= 0.0) { return vec4f(n, 0.0); } } else { d = cosineDir(n, u); }
  let c = max(dot(n, d), 1e-4); var pdf = (1.0 - share) * c / PI; if (share > 0.0) { pdf += share * openingPdf(x, d); }
  return vec4f(d, c / pdf);
}
// ================================================================ the coat
fn ggxD(a: f32, nh: f32) -> f32 { let a2 = a * a; let d = nh * nh * (a2 - 1.0) + 1.0; return a2 / (PI * d * d); }
fn smithV(a: f32, nv: f32, nl: f32) -> f32 { let a2 = a * a; return 0.5 / (nl * sqrt(nv * nv * (1.0 - a2) + a2) + nv * sqrt(nl * nl * (1.0 - a2) + a2) + 1e-5); }
fn fresnel(f0: vec3f, c: f32) -> vec3f { let u = 1.0 - sat(c); let u2 = u * u; return f0 + (1.0 - f0) * (u2 * u2 * u); }
fn coatF0(s: Surf) -> vec3f { return mix(vec3f(s.f0), s.albedo, s.metal); }
fn matteOf(s: Surf) -> vec3f { return s.albedo * (1.0 - s.metal); }
/** a facet the eye can see (Heitz), in the normal's frame */
fn visibleNormal(v: vec3f, a: f32, u: vec2f) -> vec3f {
  let vh = normalize(vec3f(a * v.x, a * v.y, v.z)); let l2 = vh.x * vh.x + vh.y * vh.y; var t1 = vec3f(1.0, 0.0, 0.0); if (l2 > 1e-7) { t1 = vec3f(-vh.y, vh.x, 0.0) * inverseSqrt(l2); }
  let t2 = cross(vh, t1); let r = sqrt(u.x); let ph = 2.0 * PI * u.y; let p1 = r * cos(ph); let s = 0.5 * (1.0 + vh.z); let p2 = (1.0 - s) * sqrt(max(0.0, 1.0 - p1 * p1)) + s * r * sin(ph);
  let nh = t1 * p1 + t2 * p2 + vh * sqrt(max(0.0, 1.0 - p1 * p1 - p2 * p2)); return normalize(vec3f(a * nh.x, a * nh.y, max(nh.z, 1e-4)));
}
/** a mirror direction through the coat and its weight (F · G2 / G1) */
struct Lobe { dir: vec3f, w: vec3f };
fn coatDir(s: Surf, V: vec3f, u: vec2f) -> Lobe {
  let b = basis(s.n); let a = max(s.rough * s.rough, 0.0016); let v = vec3f(dot(V, b[0]), dot(V, b[1]), max(dot(V, b[2]), 1e-3));
  let h = visibleNormal(v, a, u); var l = reflect(-v, h); if (l.z <= 1e-3) { l = vec3f(-v.x, -v.y, v.z); }
  let F = fresnel(coatF0(s), dot(v, h)); let a2 = a * a; let lv = (-1.0 + sqrt(1.0 + a2 * (1.0 / (v.z * v.z) - 1.0))) * 0.5; let ll = (-1.0 + sqrt(1.0 + a2 * (1.0 / (l.z * l.z) - 1.0))) * 0.5;
  var r: Lobe; r.dir = normalize(b * l); r.w = F * ((1.0 + lv) / (1.0 + lv + ll)); return r;
}
/** the coat's answer to light from L, without the cosine's shadow */
fn coatOf(s: Surf, V: vec3f, L: vec3f, widen: f32) -> vec3f {
  let nl = dot(s.n, L); let nv = max(dot(s.n, V), 1e-3); if (nl <= 0.0) { return vec3f(0.0); } let H = normalize(L + V); let a = clamp(max(s.rough * s.rough, 0.002) + widen, 0.0, 1.0);
  return fresnel(coatF0(s), dot(V, H)) * (ggxD(a, sat(dot(s.n, H))) * smithV(a, nv, nl) * nl);
}
// ================================================================ water
/** in the pool: where the sun's ray to x crosses the surface, and how deep x is (w = 1) */
fn poolSun(x: vec3f) -> vec4f {
  if (x.y < frame.pool2.x - 0.004 && x.y > frame.pool2.x - frame.pool2.y - 0.1 && x.x > frame.pool.x && x.x < frame.pool.z && x.z > frame.pool.y && x.z < frame.pool.w) { let d = frame.pool2.x - x.y; let r = refract(-frame.sunDir.xyz, vec3f(0.0, 1.0, 0.0), 0.75); return vec4f(x.xz + r.xz / min(r.y, -0.1) * d, d, 1.0); }
  return vec4f(0.0);
}
const WATER_SIGMA = vec3f(0.45, 0.07, 0.05);
/** the sun through the water to x, beside what visible() already took for the surface */
fn poolLight(x: vec3f) -> vec3f { let w = poolSun(x); if (w.w <= 0.0) { return vec3f(1.0); } let c = textureSampleLevel(causticTex, repSamp, w.xy / WAVE_TILE, 0.0).x; return mix(1.0, c, smoothstep(0.0, 0.25, w.z)) * exp(-WATER_SIGMA * w.z * 1.3) / vec3f(0.62, 0.86, 0.9); }
