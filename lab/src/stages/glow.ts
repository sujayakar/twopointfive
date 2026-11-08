// Halvings of a picture: the glow round bright things, and the eye's sense of how bright a place is.
import type { Graph, Stage, Size } from '../graph';
import glow from './glow.wgsl' with { type: 'text' };
export function glowStages(g: Graph, picture: string, prefix: string): Stage[] {
  const out: Stage[] = []; let from = picture;
  for (let i = 1; i <= 6; i++) { const name = `${prefix}Glow${i}`, size: Size = { of: 'output', div: 2 ** i }; g.signal(name, 'rgba16float', size); out.push({ name: 'glow', over: size, reads: { [from]: 'src' }, writes: { outG: name }, consts: { FIRST: i === 1 ? 1 : 0 }, code: glow }); from = name; }
  return out;
}
