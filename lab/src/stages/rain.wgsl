// THE RAIN, drop by drop. A drop belongs to a column of the world and to the clock, so it is the same drop from any camera, and a run repeats. Each asks once
// where it lands; under a roof there is none, and over a floor the section hides it is not shown, so a cut view keeps its rain out of doors. Each is lit by the
// sky and by the two lamps that send it most, if they see it: a torch's beam is drawn by the rain in it.
struct Drop { head: vec4f, tail: vec4f, light: vec4f };              // (xyz, width m) (xyz, 0 streak / 1 splash) (radiance, cover)
fn phase(mu: f32) -> f32 { let g = 0.62; return 0.35 + 0.65 * (1.0 - g * g) / pow(1.0 + g * g - 2.0 * g * mu, 1.5); }   // (per 1 / 4π: most of it on ahead)
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) {
  let i = id.x; let n = u32(rainK.z); if (i >= n * n) { return; } var o: Drop; o.light = vec4f(0.0);
  let s = rainK.y; let cell = vec2f(f32(i % n), f32(i / n)) - f32(n) * 0.5 + floor(rainAt.xy / s); let key = pcg(u32(i32(cell.x)) + pcg(u32(i32(cell.y)) + 977u)); let h = vec4f(f32(key & 255u), f32((key >> 8u) & 255u), f32((key >> 16u) & 255u), f32(key >> 24u)) / 255.0;
  let wind = vec2f(cos(frame.weather.w), sin(frame.weather.w)) * rainK.w; let fall = 8.2 + 1.8 * h.x; let vel = vec3f(wind.x * (0.75 + 0.5 * h.y), -fall, wind.y * (0.75 + 0.5 * h.y)); let top = 13.0;
  // which fall of this column it is, and how far along: the time of each is its own
  let T = 2.35; let t = frame.misc.x / T + h.z; let k = floor(t); let age = (t - k) * T; let key2 = pcg(key + u32(i32(k)) * 747u); let g = vec4f(f32(key2 & 255u), f32((key2 >> 8u) & 255u), f32((key2 >> 16u) & 255u), f32(key2 >> 24u)) / 255.0;
  let x0 = vec3f((cell.x + g.x) * s, top, (cell.y + g.y) * s); let d = normalize(vel); let sp = length(vel); let away = length(x0.xz - rainAt.xy) / (f32(n) * 0.5 * s); fineLeaves = false; rngKey(key2, 5u);
  // curtains: it falls harder in bands that the wind carries
  let band = fbm((x0.xz - vec2f(cos(frame.weather.w), sin(frame.weather.w)) * (frame.pool2.z * 1.4)) * 0.11 + 3.0); let want = frame.weather.y * mix(0.35, 1.25, smoothstep(0.34, 0.66, band)); if (g.z > want || away > 1.0) { drops[i] = o; return; }
  let hit = nearest(x0, d, 1e-3, 80.0, 0u, 1e9); let pane = hit.gpiece != NOTHING && hit.gt < hit.t; let far = select(hit.t, hit.gt, pane); let ly = x0.y + d.y * far; let run = age * sp; var at = x0 + d * run; var splash = 0.0; let size = 0.6 + 0.9 * g.w * g.w;
  // what the section hides takes its rain with it
  var hidden = false; if (ly > frame.misc.y && hit.piece != NOTHING) { if (pane) { hidden = (pieces[hit.gpiece].flags & CUT) != 0u; } else if ((hit.piece & LEAF_HIT) != 0u) { hidden = true; } else { hidden = (pieces[hit.piece].flags & (CUT | OPEN)) != 0u; } }
  let since = (run - far) / sp; if (run > far) { splash = 1.0; at = x0 + d * (far - 0.01); }
  if (hidden || since > 0.22 || (hit.piece == NOTHING && run > 60.0)) { drops[i] = o; return; }
  // what lights it
  let rd = normalize(at - frame.camPos.xyz); var L = skyLod(vec3f(0.0, 1.0, 0.0), 6.0) * 0.55 + frame.sunE.xyz * (phase(dot(rd, frame.sunDir.xyz)) * 0.02);
  var best = vec2f(0.0); var who = vec2i(-1); for (var q = 0u; q < frame.counts.x; q++) { if (lamps[q].on <= 0.0) { continue; } let a = aimLamp(q, at, vec2f(0.5)); let w = lum(a.E) * phase(dot(rd, a.dir)); if (w > best.x) { best = vec2f(w, best.x); who = vec2i(i32(q), who.x); } else if (w > best.y) { best.y = w; who.y = i32(q); } }
  for (var q = 0u; q < 2u; q++) { if (who[q] < 0 || best[q] < 1e-4) { continue; } let a = aimLamp(u32(who[q]), at, vec2f(0.5)); L += a.E * (phase(dot(rd, a.dir)) * 0.055) * lum(visible(at, a.dir, 1e-3, a.dist - 0.05)); }
  let px = distance(at, frame.camPos.xyz) * 2.0 * length(frame.camUp.xyz) / frame.size.w; let edge = 1.0 - smoothstep(0.75, 1.0, away); let near = smoothstep(0.8, 2.4, distance(at, frame.camPos.xyz));
  if (splash > 0.0) { let u = since / 0.22; let r = (0.012 + 0.055 * sqrt(u)) * size; o.head = vec4f(at, r); o.tail = vec4f(u, 0.0, 0.0, 1.0); o.light = vec4f(L, 0.34 * (1.0 - u) * edge * near); drops[i] = o; return; }
  let wide = 0.006 * size; let w = max(wide, 0.85 * px); let long = min(sp * (1.0 / 34.0) * (0.55 + 0.9 * g.w), run);
  o.head = vec4f(at, w); o.tail = vec4f(at - d * long, 0.0); o.light = vec4f(L, min(1.0, 2.6 * wide / w) * (0.35 + 0.5 * h.w) * edge * near); drops[i] = o;
}
