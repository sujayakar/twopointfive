// Halvings of a picture: the glow round bright things, and the eye's sense of how bright a place is.
import type { Graph, Stage, Size } from '../graph';
import glow from './glow.wgsl' with { type: 'text' };
export function glowStages(g: Graph, picture: string, prefix: string): Stage[] {
  const out: Stage[] = []; let from = picture;
  for (let i = 1; i <= 5; i++) { const name = `${prefix}Glow${i}`, size: Size = { of: 'output', div: 2 ** i }; g.signal(name, 'rgba16float', size); out.push({ name: 'glow', over: size, reads: { [from]: 'src' }, writes: { outG: name }, consts: { FIRST: i === 1 ? 1 : 0 }, code: glow }); from = name; }
  // the glow: three sizes of it, added softly
  const size: Size = { of: 'output', div: 4 }; g.signal(prefix + 'Bloom', 'rgba16float', size);
  out.push({ name: 'glow', over: size, reads: { [prefix + 'Glow3']: 'a', [prefix + 'Glow4']: 'b', [prefix + 'Glow5']: 'c' }, writes: { outG: prefix + 'Bloom' }, code: `
fn tent(t: texture_2d<f32>, uv: vec2f) -> vec3f { let p = 1.0 / vec2f(textureDimensions(t)); var s = vec3f(0.0); for (var y = -1; y <= 1; y++) { for (var x = -1; x <= 1; x++) { s += textureSampleLevel(t, linSamp, uv + vec2f(f32(x), f32(y)) * p, 0.0).rgb * f32((2 - abs(x)) * (2 - abs(y))); } } return s / 16.0; }
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) { let d = textureDimensions(outG); if (id.x >= d.x || id.y >= d.y) { return; } let uv = (vec2f(id.xy) + 0.5) / vec2f(d); textureStore(outG, id.xy, vec4f(tent(a, uv) * mix(0.70, 0.0, @tune.glowWide) + tent(b, uv) * 0.35 + tent(c, uv) * max(mix(-0.05, 0.65, @tune.glowWide), 0.0), 1.0)); }` });
  return out;
}
