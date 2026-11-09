// The second core: the same two ray queries by one rule (rays.wgsl), put in the first one's place in a stage's text.
import rays from './rays.wgsl' with { type: 'text' };
import surf from './surf.wgsl' with { type: 'text' };
const FROM = "// A RAY'S FOOTPRINT", TO = '// ================================================================ smoke', LEAF = 'fn leafSurf(h: Hit, x: vec3f, rd: vec3f, fp: f32) -> Surf {';
export function secondCore(prelude: string): string {
  const a = prelude.indexOf(FROM), b = prelude.indexOf(TO); if (a < 0 || b < a || !prelude.includes(LEAF)) throw new Error('the first core is not where it was');
  return (prelude.slice(0, a) + rays + '\n' + prelude.slice(b)).replace(LEAF, () => `${surf}\n${LEAF}\n  if (h.code == CELL_CODE) { return cellSurf(h, x, rd); }`);
}
