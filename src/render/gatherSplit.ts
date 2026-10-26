// SEPARATE SHADOW RAYS (2026-10-26; Options > Graphics > Separate shadow rays, ui/shaderrows.ts, `?splitrays=1`). Chrome on Windows compiles through DXC, which pastes
// every call into its caller: the final gather's `main` is 780 KB that way out of a 92 KB module, and 490 KB of it is directAtPoint — the light loop and its three shadow
// rays — written at four places. With the row on the gather is two smaller programs instead of that one: `trace` writes every point it needs lit to a buffer, `light` lights
// them in one loop round one directAtPoint and stores what the gather stores (shaders/gather_split.wgsl has the two and what the buffer holds). With it off (the default)
// nothing of this reaches the browser and every text is what it was. Read once, at import: the row is the next load's.
// WHAT THE ROW DOES NOT COVER YET: the resampling gather (fgather.wgsl mainGi, ultra's) is not built with the row on — the plain gather draws ultra's bounce — and the
// young-history rays (a Tab row, no preset) are not traced. docs/agents/render.md "Separate shadow rays" has the sizes, what was compared and what nobody has seen.
// Today's fgather.wgsl and gather_common.wgsl are imported HERE, not taken from the caller: whatever other writing of those kernels a load compiles, this row's two
// programs are made from these texts.
import { useShaderRow } from '../ui/shaderrows';
import { wgslDecls } from '../gpu/wgslTrim';
import fgatherSrc from './shaders/fgather.wgsl' with { type: 'text' };
import gatherCommonSrc from './shaders/gather_common.wgsl' with { type: 'text' };
import splitSrc from './shaders/gather_split.wgsl' with { type: 'text' };

/** this load builds the gather as two programs (the row's word, read once) */
export const SPLIT_RAYS = { on: useShaderRow('splitRays') };
/** bytes of the points' buffer a gather texel (gather_split.wgsl GX_V4 vec4u) */
export const SPLIT_TEXEL_BYTES = 15 * 16;

/** fgather.wgsl's module-scope declarations without its entry points: the helpers, constants and bindings the two programs share with it (the two megakernels stay out of the text) */
export function gatherHelpers(fgather: string = fgatherSrc): string {
  const decls = wgslDecls(fgather), kept = decls.filter(d => !/^@compute\b/.test(d));
  if (decls.length - kept.length !== 2) throw new Error(`gatherSplit: fgather.wgsl has ${decls.length - kept.length} entry points where two were expected (main, mainGi)`);
  return kept.join('\n') + '\n';
}
/** the module both programs are made from, less the scene prelude and lighting_common the caller puts in front of every tracing module: entry points `trace` and `light` */
export function splitGatherSrc(strata: boolean): string {
  return gatherCommonSrc + `const GATHER_STRATA: bool = ${strata};\n` + gatherHelpers() + splitSrc;
}
