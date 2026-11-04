// THE TEN KERNELS, TWICE (2026-10-26; Options > Graphics > Compact shaders, ui/shaderrows.ts, `?compactshaders=1`). Chrome on Windows compiles through DXC, which pastes
// every call into its caller before it optimizes, so a kernel pays for each CALL SITE of a heavy function (the tracer, `occluded`, `evalLight`). shaders/compact/ holds
// ten of the kernels written a second way, by hand: the same statements in the same order, each heavy function called from one place (a loop of turns round one call).
// With the row on, the texts below are those; with it off (as on every stand of the harness; in a browser it starts on, ui/shaderrows.ts) they are the kernels at their own paths and nothing of the second set reaches the browser.
// THE SECOND COPY IS A PROTOTYPE: it goes (one set or the other) when the default is decided. Until then a change to one of these ten kernels is made in BOTH files, or the
// row draws the picture as it was before the change (docs/agents/render.md, "Compact shaders"). Read once, at import: the row is the next load's.
// The compact volumetrics.wgsl needs two things of render/passes.ts that today's does not, both behind COMPACT.on there: `const VOL_FROXEL_MARCH` (the classic tiers' program
// is compiled with it false, the froxel tiers' twin 'volFroxel' with it true) and `const VOL_K0` (the K = 0 smoke march lives only in a twin built when it is asked for).
import { useShaderRow } from '../ui/shaderrows';
import fgather from './shaders/fgather.wgsl' with { type: 'text' };
import gatherCommon from './shaders/gather_common.wgsl' with { type: 'text' };
import reflectP5 from './shaders/reflect_p5.wgsl' with { type: 'text' };
import volumetrics from './shaders/volumetrics.wgsl' with { type: 'text' };
import composite from './shaders/composite.wgsl' with { type: 'text' };
import particles from './shaders/particles.wgsl' with { type: 'text' };
import direct from './shaders/direct.wgsl' with { type: 'text' };
import sharpDirect from './shaders/sharp_direct.wgsl' with { type: 'text' };
import restir from './shaders/restir.wgsl' with { type: 'text' };
import sdfRig from './shaders/sdfrig.wgsl' with { type: 'text' };
import fgatherC from './shaders/compact/fgather.wgsl' with { type: 'text' };
import gatherCommonC from './shaders/compact/gather_common.wgsl' with { type: 'text' };
import reflectP5C from './shaders/compact/reflect_p5.wgsl' with { type: 'text' };
import volumetricsC from './shaders/compact/volumetrics.wgsl' with { type: 'text' };
import compositeC from './shaders/compact/composite.wgsl' with { type: 'text' };
import particlesC from './shaders/compact/particles.wgsl' with { type: 'text' };
import directC from './shaders/compact/direct.wgsl' with { type: 'text' };
import sharpDirectC from './shaders/compact/sharp_direct.wgsl' with { type: 'text' };
import restirC from './shaders/compact/restir.wgsl' with { type: 'text' };
import sdfRigC from './shaders/compact/sdfrig.wgsl' with { type: 'text' };

/** this load compiles the compact set (the row's word, read once) */
export const COMPACT = { on: useShaderRow('compact') };
/** the ten kernels' texts in either writing, by the file's name (tools/qa/shaderrows-check.ts compares the two) */
export function kernelSet(compact: boolean) {
  return compact
    ? { fgather: fgatherC, gatherCommon: gatherCommonC, reflectP5: reflectP5C, volumetrics: volumetricsC, composite: compositeC, particles: particlesC, direct: directC, sharpDirect: sharpDirectC, restir: restirC, sdfRig: sdfRigC }
    : { fgather, gatherCommon, reflectP5, volumetrics, composite, particles, direct, sharpDirect, restir, sdfRig };
}
/** the ten kernels' texts this load compiles */
export const KERNELS = kernelSet(COMPACT.on);
