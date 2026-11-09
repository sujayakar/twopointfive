// LIGHT IN THE AIR, kept where it is: a grid over the rooms and round them. A cell holds what the lamps and every bounce send through it (the same every way), and how much of the sun it
// sees; a share of the cells looks again each frame. Torches move: what they send is found anew for every cell of their cones. The picture reads it along each ray, so nothing is kept on the screen.
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) {
  let i = id.x + id.y * 65535u * 64u; if (i >= AL_MANY) { return; } let c = vec3u(i % AL_N.x, (i / AL_N.x) % AL_N.y, i / (AL_N.x * AL_N.y)); let mid = AL_LO + (vec3f(c) + 0.5) * AL_H; leafMode = 2u; byReach = true;
  var q = kept[i]; var J = vec4f(unpack2x16float(q.x), unpack2x16float(q.y)); var n = f32(q.z); let inside = !openAir(mid); let st = steady(mid); if (changes.all.y > 0.5) { n = 0.0; }
  let turn = select(7u, 3u, n < 12.0 || st < 0.9); let due = inside && ((i + i / 64u) & turn) == (frame.counts.z & turn);
  if (due) {
    rngKey(i, 331u); let x = mid + (vec3f(rnd(), rnd(), rnd()) - 0.5) * AL_H; var sun = 0.0; var L = vec3f(0.0);
    if (max3(frame.sunE.xyz) > 0.0) { sun = lum(visible(x, coneDir(frame.sunDir.xyz, frame.sunDir.w, rnd2()), 1e-3, 120.0)); }
    { let l = drawLamp(x, vec3f(0.0)); if (l.x >= 0.0 && lamps[u32(max(l.x, 0.0))].kind != 2u) { let a = aimLamp(u32(l.x), x, rnd2()); let e = min(a.E * l.y, vec3f(60.0)); if (max3(e) > 1e-4) { L += e * visible(x, a.dir, 1e-3, a.dist - 0.03) * (1.0 / (4.0 * PI)); } } }
    { let u = rnd2(); let z = u.x * 2.0 - 1.0; let r = sqrt(max(1.0 - z * z, 0.0)); let d = vec3f(r * cos(6.2832 * u.y), z, r * sin(6.2832 * u.y)); let h = nearest(x, d, 1e-3, 80.0, 0u, 1e9); var B = vec3f(0.0); if (h.piece == NOTHING) { B = skyLod(d, 4.0); } else { let s = surfaceMean(x, d, h); let own = (pieces[h.piece].lamp & 255u) != 255u; B = select(s.glow, vec3f(0.0), own) + sentBack(s, false, rnd2(), d); } L += min(B * h.thru, vec3f(12.0)); }
    let k = 1.0 / (n + 1.0); J = finite4(mix(select(vec4f(0.0), J, n > 0.0), vec4f(L, sun), k), 6e4); n = min(n + 1.0, mix(6.0, 48.0, st)); kept[i] = vec4u(pack2x16float(J.xy), pack2x16float(J.zw), u32(n), 0u);
  }
  // torches, and small lamps out of doors: no dice
  var T = vec3f(0.0); for (var m = 0u; m < frame.counts.x; m++) {
    let l = &lamps[m]; if ((*l).on <= 0.0 || (*l).kind == 0u || (inside && (*l).kind != 2u)) { continue; } let d = mid - (*l).at; let r2 = dot(d, d); if (r2 > 324.0 || ((*l).kind == 1u && r2 > 20.0)) { continue; } if ((*l).kind == 2u) { let r = sqrt(r2); if (dot(d, (*l).n) < r * ((*l).cone - 0.25 * AL_H / max(r, 0.3))) { continue; } }
    let a = aimLamp(m, mid, vec2f(0.5)); let e = a.E * (1.0 / (4.0 * PI)); if (max3(e) < 2e-3) { continue; } T += min(e, vec3f(20.0)) * lum(visible(mid, a.dir, 1e-3, a.dist - 0.06));
  }
  textureStore(outLight, c, vec4f(select(vec3f(0.0), J.rgb, inside) + T * select(1.0, 1.0 + @tune.beams / max(@tune.dust, 1e-3), inside), select(1.0, J.a, inside)));
}
