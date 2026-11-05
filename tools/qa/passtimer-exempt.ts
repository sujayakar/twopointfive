// THE PASSES THAT HAVE NO TIMER, AND WHY (tools/qa/passtimer-check.ts reads this; a pass under src/ with no `timestampWrites` and no row here is red, and so
// is a row no pass uses). A row is a file under src/, the pass's label ('' = it has none) and how many such passes the file opens (1 unless said).
// A timer is for a pass the frame pays for again and again. What may go without: a render pass (its stamps are not to be trusted on Metal; a compute pass has no such excuse), a pass outside the frame's own encoder (the timer resolves on that
// encoder alone), a clear on the one frame a setting flips, a tool's or a check's readback, a pass that hands another's descriptor on.
export interface PassExempt { at: string; label: string; n?: number; why: string }
export const PASS_EXEMPT: readonly PassExempt[] = [
  { at: 'render/gbuffer.ts', label: 'gdown', why: "a render pass: on Metal a render pass's stamps can span the wait for the drawable and would be in every GPU total. 0.09 ms on a rented card, 11-05, timed by its label from a page script" },
  { at: 'render/outline.ts', label: 'outline', why: 'a render pass, as gdown: 0.04 ms on a rented card, 11-05' },
  { at: 'render/passes.ts', label: 'targets', why: "an encoder of its own, submitted when the targets are made again (a resize): not a frame's work, and the timer resolves on the frame's encoder alone" },
  { at: 'render/passes.ts', label: 'restartHistories', why: "an encoder of its own, submitted once when the histories are started over: not a frame's work" },
  { at: 'render/passes.ts', label: 'dnSwitch', why: "clears the direct histories on the one frame the denoiser's core is switched: never a second frame running" },
  { at: 'render/passes.ts', label: 'giRestart', why: 'clears the indirect histories on the one frame it is switched back on' },
  { at: 'render/passes.ts', label: 'giFullSwitch', why: 'clears the full-size indirect histories on the one frame that grid is switched' },
  { at: 'render/passes.ts', label: 'rtRestart', why: 'clears the reflection histories on the one frame reflections are switched back on' },
  { at: 'render/passes.ts', label: 'hitRestart', why: 'clears the reflection hit history on the one frame hit reuse is switched on' },
  { at: 'render/fogofwar.ts', label: 'fogofwarQa', why: "a check's readback of the fog pass, on an encoder of its own (qaRun): no frame runs it" },
  { at: 'render/savepic.ts', label: '', why: "the saved picture's one draw, on an encoder of its own, when a picture is asked for" },
  { at: 'render/rigDebug.ts', label: 'rigDbg', why: "a tool's ray walk read back to the page (the rig debugger), on an encoder of its own" },
  { at: 'render/meshShadows.ts', label: 'meshDbgWalk', why: "a tool's ray walk read back to the page (the mesh shadows' debugger), on an encoder of its own" },
  { at: 'render/placeField.ts', label: '', why: 'an empty pass on an encoder of its own that only tries the two programs under an error scope, once at boot' },
  { at: 'render/diceRing.ts', label: '', why: 'an empty pass on an encoder of its own that only tries the program under an error scope, once when the ring is made' },
  { at: 'smoke/smoke.ts', label: '', why: "hands the caller's own descriptor on (the solver stepped alone, cut into one buffer an operation): every pass it opens is opened, and timed, where the solver encodes it" },
  { at: 'main.ts', label: '', why: "a console lab that runs one kernel on numbers handed to it and reads them back (an encoder of its own; its label is the caller's word)" },
];
