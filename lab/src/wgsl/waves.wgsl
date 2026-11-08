// Waves repeat every WAVE_TILE metres, so that what they do to the sun fits in one small picture. (kx, kz in turns a tile, height m, phase)
const WAVE_TILE = 6.0;
const WAVES = array<vec4f, 14>(vec4f(1.0, 2.0, 0.020973, 0.00), vec4f(-2.0, 1.0, 0.020973, 2.40), vec4f(3.0, -2.0, 0.012907, 4.80), vec4f(-4.0, 3.0, 0.008389, 0.91), vec4f(8.0, 5.0, 0.005891, 3.31), vec4f(-9.0, 7.0, 0.004840, 5.71), vec4f(13.0, -4.0, 0.004251, 1.83), vec4f(-6.0, -14.0, 0.003616, 4.23), vec4f(17.0, 9.0, 0.002551, 0.34), vec4f(-19.0, 6.0, 0.002377, 2.74), vec4f(10.0, -21.0, 0.001841, 5.14), vec4f(24.0, 11.0, 0.001429, 1.26), vec4f(-16.0, 24.0, 0.001134, 3.66), vec4f(29.0, -18.0, 0.000720, 6.05));
/** the slope of the water */
fn waveSlope(p: vec2f, t: f32) -> vec2f { var s = vec2f(0.0); for (var i = 0u; i < 14u; i++) { let w = WAVES[i]; let k = w.xy * (2.0 * PI / WAVE_TILE); s += k * (w.z * cos(dot(k, p) - t * sqrt(9.81 * length(k)) * 0.62 + w.w)); } return s; }
fn waveNormal(p: vec2f, t: f32) -> vec3f { let s = waveSlope(p, t); return normalize(vec3f(-s.x, 1.0, -s.y)); }
