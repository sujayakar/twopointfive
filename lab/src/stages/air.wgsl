// SMOKE, STEAM AND FIRE, as the eye sees them: small steps through the bricks. (Haze and dust are read from the light kept in the air: haze.wgsl)
fn vn3(p: vec3f) -> f32 { let i = floor(p); let f = p - i; let w = f * f * (3.0 - 2.0 * f); let z = f32(i.z) * 37.0; return mix(vnoise(p.xy + z), vnoise(p.xy + z + 37.0), w.z); }
/** 1 where the section leaves the air to be seen, 0 over the rooms it hides, softly */
fn cutShows(x: vec3f) -> f32 { if (frame.misc.y > 1e8) { return 1.0; } return select(1.0, smoothstep(0.0, 0.35, frame.misc.y - x.y), overCut(x, frame.misc.y)); }
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = textureDimensions(outA); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let full = min(id.xy * 2u, vec2u(frame.size.xy) - 1u); let rd = pxRay(full); let t1 = min(bitcast<f32>(textureLoad(surf, full, 0).x), 260.0); let ro = frame.camPos.xyz; 
  let j = blue2(id.xy, 9u); var seen = vec2f(1e-6 * t1, 1e-6); var L = vec3f(0.0); var tau = 0.0;
  // the smoke: small steps where there is any, over empty bricks in one. What the section hides takes its smoke with it
  if (frame.misc2.x > 0.5) { let s = smokeSpan(ro, rd, t1); var t = s.x + 0.1 * j.x; var T = 1.0; var G = vec3f(0.0);
    for (var i = 0u; i < 220u; i++) { if (t >= s.y || T < 0.012) { break; } let x = ro + rd * t; let shown = cutShows(x); let p = smokeSlot(x);
      if (p == 0u || shown <= 0.0) { let c = floor((x - SM_AT) / SM_BRICK); let e = (SM_AT + (c + select(vec3f(0.0), vec3f(1.0), rd > vec3f(0.0))) * SM_BRICK - ro) * safeInv(rd); t = max(min3(e), t) + 0.004; continue; }
      let dt = min(0.1, s.y - t); let v = smokeIn(p, x); var dn = v.w * shown; if (max3(v.rgb) > 24.0) { let q = x * vec3f(9.0, 5.0, 9.0) + vec3f(0.0, -16.0 * frame.misc.x, 0.0); let nz = 0.6 * vn3(q) + 0.4 * vn3(q * 2.3 + 9.0); dn *= mix(2.2 * smoothstep(0.40, 0.72, nz), 1.3, smoothstep(70.0, 260.0, max3(v.rgb))); } else if (dn > 0.004 && dn < 2.0) { let q = x * 5.2 + vec3f(0.13, -0.5, 0.09) * frame.misc.x; let nz = 0.62 * vn3(q) + 0.38 * vn3(q * 2.7 + 5.0); dn = dn * mix(smoothstep(0.28, 0.62, nz) * 1.8, 1.0, smoothstep(0.2, 2.0, dn)); } if (dn > 0.003) { let e = exp(-dn * dt); let part = T * (1.0 - e) * exp(-tau); seen += vec2f(t, 1.0) * part; G += v.rgb * part; T *= e; } t += dt; }
    L += G; tau -= log(max(T, 1e-5)); }
  textureStore(outD, id.xy, vec4f(seen.x / seen.y, seen.y, 0.0, 0.0));       // (how far off what is seen of the air is: its history moves as that does, not as what stands behind it)
  textureStore(outA, id.xy, vec4f(L, exp(-tau)));
}
