// The second core: the same two ray queries by one rule (rays.wgsl), put in the first one's place in a stage's text; and what cells keep of light (kept.wgsl) in the place of the foliage's grid.
import rays from './rays.wgsl' with { type: 'text' };
import surf from './surf.wgsl' with { type: 'text' };
import kept from './kept.wgsl' with { type: 'text' };
const LEAF = 'fn leafSurf(h: Hit, x: vec3f, rd: vec3f, fp: f32) -> Surf {';
function between(text: string, a0: string, b0: string, put: string): string { const a = text.indexOf(a0), b = text.indexOf(b0, a); if (a < 0 || b < a) throw new Error('the first core is not where it was: ' + a0); return text.slice(0, a) + put + '\n' + text.slice(b); }
/** `dice`: a ray stops at one cell, drawn; else what cells send it is summed. `leafNear`: how far leaves are met one by one, as a share of how far other things as small are */
export function secondCore(prelude: string, dice = true, leafNear = 1): string {
  const mine = `const BLK_DICE = ${dice}; const LEAF_NEAR = ${leafNear.toFixed(3)};\n` + (prelude.includes('fn visiblePieces(') ? rays.replace('fn visible(ro: vec3f,', 'fn visiblePieces(ro: vec3f,') : rays);
  const p = between(between(prelude, "// A RAY'S FOOTPRINT", '// ================================================================ smoke', mine), 'const FOL_LO', '// ==== the foliage ends', kept);
  if (!p.includes(LEAF)) throw new Error('no leafSurf'); return p.replace(LEAF, () => `${surf}\n${LEAF}\n  if (h.code == CELL_CODE) { return cellSurf(h, x, rd); }`);
}
