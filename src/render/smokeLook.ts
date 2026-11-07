// HOW SMOKE IS DRAWN ON THE LOOK PROTOTYPE'S PAGE — one table, the maintainer's numbers from the smoke lab (11-03: "latest settings we should make these
// default"). They are rows of the PICTURE only: the solver's own numbers in that paste were the build's already, so the smoke itself, and what anyone sees
// through it, is the build's. Every other page draws smoke as it did (the engine's and the presets' own numbers): the table is put on only where the look is.
//   creases · selfK · selfReach · ahead    engine.smokeDraw (0 there = the shader's own number)
//   keep · blurBehind                      the post pass's smoke diffusion (render/passes.ts): 0 = smoke only dims what is behind it, and the blur is not run
//   glowThrough · blurStart · leanOnNew    settings.smokeMultiScatter / smokeDiffuseStart / smokeTaaReact
//   stepCells                              the march's step in cells of the solver AS TUNED ON HIGH (whose own step is TUNED_STEP): a preset's step is taken
//                                          in that proportion (smokeLookStep), so the ladder's cheaper presets keep their longer step
// The lab's rows (host/smokelab.ts) move the same numbers live and its button puts these back. `?smokelook=0` on the look page = the smoke drawn as the build
// draws it (a benchmark asked for there carries the word, scene/look.ts). Tuned on a GPU by eye at dusk in the lab; the cost of the shorter step is not measured.
import type { Engine } from '../engine';

export const SMOKE_LOOK = { creases: 0.6, selfK: 10.5, selfReach: 0.58, ahead: 0.65, keep: 0, glowThrough: 0.15, blurBehind: 0, blurStart: 0, leanOnNew: 1, stepCells: 1 };
/** high's own march step, the one `stepCells` was tuned against */
const TUNED_STEP = 1.5;
/** on = this page draws smoke by the table (the look page; main.ts, before the first preset is put on) */
export const smokeLook = { on: false };

/** a preset's march step, in cells, as this page marches it (ui/quality.ts putSmoke: the one writer of settings.volStepVoxels) */
export function smokeLookStep(presetStep: number): number { return smokeLook.on ? Math.max(1, presetStep * SMOKE_LOOK.stepCells / TUNED_STEP) : presetStep; }

/** the table written to where each row is read; the march's step is the preset's to write (smokeLookStep) */
export function putSmokeLook(engine: Engine): void {
  const L = SMOKE_LOOK, S = engine.settings, D = engine.smokeDraw, P = engine.passes;
  D.creases = L.creases; D.selfK = L.selfK; D.selfReach = L.selfReach; D.ahead = L.ahead;
  P.smokeDiffuseKeep = L.keep; P.smokeDiffuse = L.blurBehind;
  S.smokeMultiScatter = L.glowThrough; S.smokeDiffuseStart = L.blurStart; S.smokeTaaReact = L.leanOnNew;
}
