// What rays pay: from what each pixel shows, one toward the sun and one at random over the half sphere. How many boxes they look at, and do they meet a thing that stands?
import type { Method } from '../lab';
export const walkedStage = (): Method => ({ name: 'walked', result: 'walkOut', signals(g) { g.signal('walkOut', 'rgba16float', 'output'); }, stages: () => [{ name: 'walked', over: 'output', writes: { outT: 'walkOut' }, code: `
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = vec2u(frame.size.zw); if (id.x >= dim.x || id.y >= dim.y) { return; } rngInit(id.xy, 77u); leafMode = 0u; var ro = frame.camPos.xyz; var rd = camRay((vec2f(id.xy) + 0.5) / vec2f(dim)); var x = ro; var sky = true; var met = 0.0; var boxes = 0.0; var seen = 0.0; foot(0.0, 1e-4);
  for (var q = 0u; q < 3u; q++) {
    if (q == 1u) { rd = frame.sunDir.xyz; foot(0.002, 0.01); } if (q == 2u) { let u = rnd2(); let r = sqrt(u.x); rd = normalize(vec3f(r * cos(6.2831853 * u.y), sqrt(max(1.0 - u.x, 0.0)) + 0.02, r * sin(6.2831853 * u.y))); foot(0.002, 0.5); }
    if (q > 0u) { if (sky) { break; } leafMode = 2u; ro = x + vec3f(0.0, 0.004, 0.0); walked = 0u; }
    let h = nearest(ro, rd, 1e-3, 200.0, 0u, 1e9);
    if (q == 0u) { if (h.piece != NOTHING) { sky = false; x = ro + rd * h.t; if (x.y > 0.004) { seen = 1.0; } } } else { boxes += f32(walked); if (h.piece != NOTHING && (ro + rd * h.t).y > 0.004) { met += 0.5; } }
  }
  textureStore(outT, id.xy, vec4f(boxes, met, seen, 1.0));
}` }] });
