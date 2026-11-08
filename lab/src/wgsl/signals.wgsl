// The first hit as the stages hand it on, packed, and what they share for reading it.
struct Px { t: f32, x: vec3f, n: vec3f, ng: vec3f, albedo: vec3f, rough: f32, metal: f32, f0: f32, kind: u32, leaf: bool, sky: bool, glass: bool, ao: vec2f };
fn packPx(t: f32, s: Surf) -> vec4u {
  let g = vec2u(octEnc(s.ng) * 255.0 + 0.5);
  return vec4u(bitcast<u32>(t), octPack(s.n), packRGB(s.albedo) | (u32(sat(s.rough) * 255.0 + 0.5) << 24u), g.x | (g.y << 8u) | (u32(select(s.metal, s.ao.y, any(s.through > vec3f(0.0))) * 15.0 + 0.5) << 16u) | ((s.kind & 63u) << 20u) | select(0u, (1u << 26u) | (u32(s.ao.x * 31.0 + 0.5) << 27u), any(s.through > vec3f(0.0))));
}
fn unpackPx(v: vec4u, rd: vec3f) -> Px {
  var p: Px; p.t = bitcast<f32>(v.x); p.sky = p.t > 1.5e3; p.x = frame.camPos.xyz + rd * p.t; p.n = octUnpack(v.y); p.albedo = unpackRGB(v.z); p.rough = f32(v.z >> 24u) / 255.0;
  p.ng = octDec(vec2f(f32(v.w & 255u), f32((v.w >> 8u) & 255u)) / 255.0); p.metal = f32((v.w >> 16u) & 15u) / 15.0; p.kind = (v.w >> 20u) & 63u; p.leaf = ((v.w >> 26u) & 1u) != 0u; p.f0 = 0.04; if (p.leaf) { p.ao = vec2f(f32(v.w >> 27u) / 31.0, p.metal); p.metal = 0.0; } return p;
}
/** under water, what is seen does not stand on the pixel's straight ray */
fn underWater(p: Px, f: vec4u, rd: vec3f) -> Px { var q = p; if ((f.z >> 24u) == 2u && f.w != 0u) { q.x = frame.camPos.xyz + rd * bitcast<f32>(f.x) + refract(rd, octUnpack(f.y), 0.75) * bitcast<f32>(f.w); } return q; }
fn pxSurf(p: Px) -> Surf { var s: Surf; s.x = p.x; s.n = p.n; s.ng = p.ng; s.albedo = p.albedo; s.rough = p.rough; s.metal = p.metal; s.f0 = p.f0; s.kind = p.kind; s.through = select(vec3f(0.0), leafThrough(p.albedo * select(1.0, 1.0 / 1.45, p.rough > 0.7), p.kind == 20u), p.leaf); return s; }
/** the ray of a render pixel this frame, jitter and all */
fn pxRay(id: vec2u) -> vec3f { return camRay((vec2f(id) + 0.5 + frame.jitter.xy) / frame.size.xy); }
fn pxRayPrev(uv: vec2f) -> vec3f { let p = uv * 2.0 - 1.0; return normalize(frame.prevFwd.xyz + frame.prevRight.xyz * p.x - frame.prevUp.xyz * p.y); }
/** how far into the past a point may trust its history, 0‥1: the change signal (what moved, and lamps that switched) */
fn steady(x: vec3f) -> f32 { var k = 1.0; for (var i = 0u; i < 8u; i++) { let c = changes.at[i]; if (c.w <= 0.0) { continue; } k = min(k, smoothstep(0.55, 1.0, distance(x, c.xyz) / c.w)); } return min(k, changes.all.x); }
struct Changes { at: array<vec4f, 8>, all: vec4f };
