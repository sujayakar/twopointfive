// What a surface that a ray found sends back: what it is lit by (cacheRead) times all it returns, its coat's share with its matte's. Foliage is a medium: what
// lands on a leaf there is the mean over every way a leaf could face.
fn returns(s: Surf) -> vec3f { let f = coatF0(s); let a = f + (max(vec3f(1.0 - s.rough), f) - f) * 0.05; return matteOf(s) * (1.0 - lum(a)) + a; }
/** how leaves that face every way send a beam on, by the angle between its way and the way out (Ross): back off their faces, forward through them */
fn leafPhase(q: Surf, rd: vec3f) -> vec3f { let c = dot(frame.sunDir.xyz, rd); let b = acos(clamp(c, -1.0, 1.0)); return max((q.albedo + q.through) * ((sin(b) - b * c) / (3.0 * PI)) + q.through * (c / 3.0), vec3f(0.0)) * (2.0 / PI); }
fn sentBack(q: Surf, leaf: bool, j: vec2f, rd: vec3f) -> vec3f {
  let c = cacheRead(q.x, select(q.ng, vec3f(0.0), leaf), j);
  if (leaf) { let a = (q.albedo + q.through) / PI; if (c.n > 0.0) { return a * c.E + leafPhase(q, rd) * frame.sunE.xyz * c.sun; } return a * (frame.sunE.xyz * 0.06 + skyLod(vec3f(0.0, 1.0, 0.0), 6.0) * 0.5); }
  if (c.n <= 0.0) { return vec3f(0.0); } return returns(q) / PI * (c.E + frame.sunE.xyz * max(dot(q.ng, frame.sunDir.xyz), 0.0) * c.sun);
}
