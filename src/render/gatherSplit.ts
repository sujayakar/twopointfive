// THE FINAL GATHER IS TWO PROGRAMS (2026-10-26 as the row 'Separate shadow rays', the page's default since 10-27; the only gather since 2026-11-07, when the one-kernel
// gather and its row went). Chrome on Windows compiles through DXC, which pastes every call into its caller: the one-kernel gather's `main` was 780 KB that way out of a
// 92 KB module, and 490 KB of it was directAtPoint — the light loop and its three shadow rays — written at four places. Here `trace` writes every point it needs lit to a
// buffer, and `light` lights them in one loop round one directAtPoint and stores what the gather stores (shaders/gather_split.wgsl has the two and what the buffer holds).
// WHAT WENT WITH THE ONE-KERNEL GATHER: the resampling gather (ultra's `mainGi`, never built beside these two programs) and the young-history rays (a Tab row, no preset).
// THE TEXTS: shaders/fgather.wgsl is the gather's helpers, constants and bindings and has no entry point of its own; shaders/gather_common.wgsl is the light / card
// staging and directAtPoint, the same file the reflection pass is made from (one writing since 2026-11-07: the gather had a plainer copy of its light loop until then).
import { reportError } from '../cloud/report';   // (a leaf that imports nothing)
import { wgslDecls } from '../gpu/wgslTrim';
import fgatherSrc from './shaders/fgather.wgsl' with { type: 'text' };
import gatherCommonSrc from './shaders/gather_common.wgsl' with { type: 'text' };
import splitSrc from './shaders/gather_split.wgsl' with { type: 'text' };

/** the points' buffer was refused (render/passes.ts ensureSplitBG): `line` on the console as it always was, and one of the visit's error records under
 *  'gpu.splitrays.buffer' (src/cloud/report.ts) with the grid and the size beside it. A GPU too small for the buffer draws no bounce light at all, so that
 *  must be seen from outside that machine. One machine refused at three window sizes is one error (the reporter folds a line's numbers). Never throws */
export function splitRefused(line: string, data?: Record<string, unknown>): void { reportError('gpu.splitrays.buffer', line, data); }
/** bytes of the points' buffer a gather texel (gather_split.wgsl GX_V4 vec4u) */
export const SPLIT_TEXEL_BYTES = 15 * 16;

let own: string | null = null;
/** fgather.wgsl's module-scope declarations: the helpers, constants and bindings the two programs are written against. The build's own text is read once; a dev
 *  hook's variant (passes.ts hotSwapGather) is read as given, and an entry point it still carries is left out. */
export function gatherHelpers(fgather: string = fgatherSrc): string {
  if (fgather === fgatherSrc && own !== null) return own;
  const text = wgslDecls(fgather).filter(d => !/^@compute\b/.test(d)).join('\n') + '\n'; if (fgather === fgatherSrc) own = text;
  return text;
}
/** the module both programs are made from, less the scene prelude and lighting_common the caller puts in front of every tracing module: entry points `trace` and `light`
 *  (a variant that declares GATHER_STRATA itself keeps its own line) */
export function splitGatherSrc(strata: boolean, fgather: string = fgatherSrc, common: string = gatherCommonSrc): string {
  const helpers = gatherHelpers(fgather);
  return common + (/^const GATHER_STRATA\b/m.test(helpers) ? '' : `const GATHER_STRATA: bool = ${strata};\n`) + helpers + splitSrc;
}
