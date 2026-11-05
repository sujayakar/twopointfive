// HOW A STEP OF THE SMOKE SOLVER DISPATCHES ITS PER-BRICK KERNELS (smoke/smoke.ts encode), and which way a page opens on (main.ts). Two plain rules in a
// file with nothing of the GPU in it, so that a check can ask them (tools/qa/perf-sweep-logic.ts) and the F8 sweep's row asks the same one the solver does.
//
// THE TWO WAYS. By THE GPU'S OWN COUNT of the step's list (`sparse.indirect`, about twenty dispatchWorkgroupsIndirect calls a step): the solver's own
// default, as it was written. Or directly over THE WHOLE POOL: a workgroup past the list returns at its guard (smoke.wgsl: `wid.x >= M[M_COUNT]`,
// openBrick), so the smoke is the same to the bit; it is how the pool has always gone from `directAt` of it stepping.
// THE PAGE OPENS ON THE WHOLE POOL (2026-11-05): on an NVIDIA card through Vulkan the indirect calls made the GPU's answer for the WHOLE FRAME come a
// frame late, in no pass's stamp, and with any smoke live the page drew about 42 frames a second for 60.
// THE ONE SPECIAL CASE: a page on the browser's SOFTWARE ADAPTER stays on the GPU's count. That adapter runs every idle workgroup on the CPU, a step about
// thirty times slower (0.2 s against 5 s); it is the adapter of the cloud's page tools, and a player's only where the browser has no card to give. It
// changes nothing a real card computes. What would remove it: a solver whose step list is capped by a count the page hands it, so that a direct dispatch
// can be in proportion to the smoke (a count the page merely GUESSES was tried and thrown out the same day: a blast makes hundreds of bricks in one step,
// the read-back is frames old, and the bricks past the count are listed and never linked). docs/agents/render.md, 'A COST NO STAMP SEES'.

/** this step's per-brick kernels go by the GPU-written count (else: directly over the whole pool) */
export const byGpuCount = (k: { indirect: boolean; directAt: number }, stepped: number, cap: number): boolean => k.indirect && stepped < k.directAt * cap;

/** what the page sets `sparse.indirect` to as it opens: the visit's word if it says ('1' = the GPU's count, '0' = the whole pool), else the whole pool
 *  unless the browser's software adapter draws the page */
export const pageIndirect = (word: string | null, software: boolean): boolean => word === '1' ? true : word === '0' ? false : software;
