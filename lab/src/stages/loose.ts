// LEAVES THE WIND TAKES. Each is a particle: torn from a real leaf of a spray in a gust, tumbling down the air, lying where a ray says it lands, and lifted again by a
// hard gust. They are drawn as any leaf is (the same raster, the sun's map, the same light); bounce rays do not see them.
import type { Lab } from '../lab';
import type { Graph, Stage } from '../graph';
export const LOOSE = 3072;
const code = `
struct P { at: vec4f, vel: vec4f, q: vec4f, info: vec4u };       // place and size · speed and age · how it is turned · 0 none 1 flying 2 lying, tint, seed, ticks left
fn qmul(a: vec4f, b: vec4f) -> vec4f { return vec4f(a.w * b.xyz + b.w * a.xyz + cross(a.xyz, b.xyz), a.w * b.w - dot(a.xyz, b.xyz)); }
fn qrot(q: vec4f, v: vec3f) -> vec3f { return v + 2.0 * cross(q.xyz, cross(q.xyz, v) + q.w * v); }
fn qfrom(a: vec3f, b: vec3f) -> vec4f { let d = dot(a, b); if (d < -0.999) { return vec4f(1.0, 0.0, 0.0, 0.0); } return normalize(vec4f(cross(a, b), 1.0 + d)); }
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) {
  let i = id.x; if (i >= ${LOOSE}u) { return; } var p = pool[i]; let was = p; let n = u32(looseK.x); leafMode = 2u;
  for (var step = 0u; step < n; step++) {
    let tick = u32(looseK.y) - (n - 1u - step); rngState = pcg(i * 747796405u + pcg(tick + 13u)); let t = f32(tick) / 60.0; let dt = 1.0 / 60.0;
    if (p.info.x == 0u) {
      // a gust tears one off: a spray drawn at random, and a leaf of it
      if (rnd() > looseK.z) { continue; } let k = min(u32(rnd() * looseK.w), u32(looseK.w) - 1u); let I = insts[k]; if ((I.tint & 0x1000000u) == 0u || (I.tint & 0x2000000u) != 0u) { continue; }
      let lf = I.leaf + u32(rnd() * 120.0); let a = exF(lf * 2u); let b = extra[lf * 2u + 1u]; if ((b.w & 255u) != 0u) { continue; } let s2 = dot(I.r0.xyz, I.r0.xyz); let d = a.xyz - vec3f(I.r0.w, I.r1.w, I.r2.w); let x = (I.r0.xyz * d.x + I.r1.xyz * d.y + I.r2.xyz * d.z) / s2;
      /* (an autumn leaf comes off easily) */ if ((I.tint & 255u) < ((I.tint >> 8u) & 255u) && rnd() > 0.35) { continue; } let air = windAt(x); if (length(air) < 3.0 + 2.5 * rnd() && tick > 4u) { continue; } p.at = vec4f(x, a.w / sqrt(s2)); p.vel = vec4f(air * 0.2, 0.0); let nn = normalize(I.r0.xyz * octUnpack(b.x).x + I.r1.xyz * octUnpack(b.x).y + I.r2.xyz * octUnpack(b.x).z); p.q = qmul(qfrom(vec3f(0.0, 1.0, 0.0), nn), vec4f(0.0, sin(rnd() * 3.0), 0.0, cos(rnd() * 3.0))); p.q = normalize(p.q); p.info = vec4u(1u, I.tint, pcg(rngState), 3000u + u32(rnd() * 6000.0)); continue;
    }
    let air = windAt(p.at.xyz); let h = f32(p.info.z & 1023u) / 1023.0;
    if (p.info.x == 2u) { if (p.info.w == 0u) { p.info.x = 0u; continue; } p.info.w -= 1u; if (length(air.xz) > 3.4 + 2.6 * h && rnd() < 0.003) { p.info.x = 1u; p.vel = vec4f(air.x * 0.4, 0.8 + 0.8 * rnd(), air.z * 0.4, 0.0); } else { continue; } }
    // a thin plate: it falls slowly flat, quickly on edge, and swings from side to side
    let up = qrot(p.q, vec3f(0.0, 1.0, 0.0)); let rel = air - p.vel.xyz; let face = abs(dot(up, normalize(rel + vec3f(0.0, 1e-4, 0.0)))); let drag = mix(1.6, 5.5, face); p.vel = vec4f(p.vel.xyz + (rel * drag + vec3f(0.0, -5.2, 0.0)) * dt, p.vel.w + dt); let lift = cross(up, cross(up, rel)); p.vel = vec4f(p.vel.xyz - lift * (0.9 * dt), p.vel.w);
    let sp = 2.0 + 2.2 * length(rel); let w = vec3f(sin(p.vel.w * 3.1 + h * 40.0), 0.7 * cos(p.vel.w * 1.9 + h * 23.0), cos(p.vel.w * 2.7 + h * 11.0)) * sp * (0.6 + 0.8 * h); p.q = normalize(p.q + qmul(vec4f(w, 0.0), p.q) * (0.5 * dt));
    let mv = p.vel.xyz * dt; let l = length(mv); let hit = nearest(p.at.xyz, mv / max(l, 1e-6), 0.0, l + 0.02, 0u, 1e9); let solid = hit.piece != NOTHING && (hit.piece & LEAF_HIT) == 0u; let pane = hit.gpiece != NOTHING && hit.gt < l + 0.02;
    if (solid || pane) {
      var q = hit; if (pane) { q.piece = hit.gpiece; q.t = hit.gt; q.code = hit.gcode; } let sf = normalize(toWorld(q.piece, localNormal(q.piece, q.code, toLocal(q.piece, p.at.xyz + mv / max(l, 1e-6) * q.t))));
      if (sf.y > 0.55) { p.at = vec4f(p.at.xyz + mv / max(l, 1e-6) * max(q.t - 0.004, 0.0), p.at.w); p.info.x = 2u; p.vel = vec4f(0.0, 0.0, 0.0, p.vel.w); let tilt = normalize(sf + vec3f(h - 0.5, 0.0, fract(h * 7.0) - 0.5) * 0.35); let up2 = qrot(p.q, vec3f(0.0, 1.0, 0.0)); p.q = normalize(qmul(qfrom(up2, select(-tilt, tilt, dot(up2, tilt) > 0.0)), p.q)); }
      else { p.vel = vec4f(p.vel.xyz - sf * (1.2 * min(dot(p.vel.xyz, sf), 0.0)), p.vel.w); }
    } else { p.at = vec4f(p.at.xyz + mv, p.at.w); }
    if (p.at.y < -8.0 || abs(p.at.x) > 60.0 || abs(p.at.z) > 60.0) { p.info.x = 0u; }
  }
  pool[i] = p; emit(i, p, 0u); let born = was.info.x == 0u || was.info.z != p.info.z; if (born) { emit(i, p, 1u); } else { emit(i, was, 1u); }
}
fn emit(i: u32, p: P, which: u32) {
  var at = p.at.xyz; var k = 1.0 / max(p.at.w, 1e-3); if (p.info.x == 0u) { at = vec3f(0.0, -900.0, 0.0); k = 30.0; } let x = qrot(p.q, vec3f(1.0, 0.0, 0.0)); let y = qrot(p.q, vec3f(0.0, 1.0, 0.0)); let z = qrot(p.q, vec3f(0.0, 0.0, 1.0));
  let r0 = vec4f(x * k, -dot(x, at) * k); let r1 = vec4f(y * k, -dot(y, at) * k); let r2 = vec4f(z * k, -dot(z, at) * k);
  if (which == 0u) { rows[i * 4u] = bitcast<vec4u>(r0); rows[i * 4u + 1u] = bitcast<vec4u>(r1); rows[i * 4u + 2u] = bitcast<vec4u>(r2); let terrace = at.x > 3.9 && at.y > 3.3 && abs(at.z) < 8.2; rows[i * 4u + 3u] = vec4u(0u, 0u, u32(looseAt.x), (p.info.y & 0x00ffffffu) | select(0u, 0x1000000u, terrace)); }
  else { past[i * 3u] = r0; past[i * 3u + 1u] = r1; past[i * 3u + 2u] = r2; }
}`;
export function looseStages(lab: Lab, g: Graph): Stage[] {
  const s = lab.scene; lab.buffer('loosePool', LOOSE * 64); lab.buffer('looseRows', LOOSE * 64); lab.buffer('loosePast', LOOSE * 48); lab.uniform('looseK', new Float32Array(4)); lab.uniform('looseAt', new Float32Array([s.looseLeaf, 0, 0, 0])); let last = -1;
  return [
    { name: 'loose leaves', over: { groups: () => [LOOSE / 64, 1, 1] }, buffers: { pool: ['loosePool', 'array<P>', 'read_write'], rows: ['looseRows', 'array<vec4u>', 'read_write'], past: ['loosePast', 'array<vec4f>', 'read_write'], looseK: ['looseK', 'vec4f', 'uniform'], looseAt: ['looseAt', 'vec4f', 'uniform'] }, code,
      when: () => { const n = last < 0 ? 1 : Math.max(0, Math.min(4, lab.tick - last)); last = lab.tick; lab.uniform('looseK', new Float32Array([n, lab.tick, lab.tick < 5 ? 0.02 : 0.00025, s.counts.instances])); return true; } },
    { name: 'loose hand', over: { groups: () => [1, 1, 1] }, code: '', async make() { return enc => { enc.copyBufferToBuffer(g.buffers.get('looseRows')!, 0, s.insts, s.counts.instances * 64, LOOSE * 64); enc.copyBufferToBuffer(g.buffers.get('loosePast')!, 0, g.buffers.get('pastLeaves')!, s.counts.instances * 48, LOOSE * 48); }; } },
  ];
}
