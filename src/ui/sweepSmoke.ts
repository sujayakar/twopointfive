// The F8 sweep's row 'smoke solver: the dispatch of before' (ui/perfsweep.ts): its flip, in a file of its own so that a check can drive it with no GPU
// (tools/qa/perf-sweep-logic.ts).
//
// WHAT IT PRICES: the solver's per-brick kernels dispatched over the whole pool, as the page opens (smoke/dispatch.ts, 2026-11-05), against the dispatch of
// before, by a GPU-written count (about twenty indirect calls a frame). WHAT THE INDIRECT CALLS COST, WHERE THEY COST, IS IN NO PASS'S COLUMN: the GPU's
// answer for the whole frame came late, so the row is judged by its FRAME TIME (`readBy: 'frame'`), and the `smoke` pass's column beside it is the idle
// workgroups' own price. A live row: a held world steps no smoke. THE FLIP IS ASKED ONCE, AS THE TAKE BEGINS: a can's smoke is gone before a whole sweep
// is through and a round with no smoke reads nothing, so take it where smoke stands (a snow level's breath), or this row alone right after a can.
import type { SweepFlip } from './perfsweep';
import { byGpuCount } from '../smoke/dispatch';

/** what the flip needs of the solver (smoke/smoke.ts SmokeSystem) */
export interface SweepSmoke { sparse: { indirect: boolean; directAt: number }; stats: { pool: number; stepped: number }; params: { enabled: boolean; frozen?: boolean }; grid: { cap: number } }

/** the flip and its undo, or the reason the row cannot be read on this page right now */
export function smokeIndirectFlip(s: SweepSmoke): SweepFlip | string {
  if (!s.params.enabled || s.params.frozen) return 'the smoke solver is switched off or held here: nothing steps';
  if (!(s.stats.pool > 0)) return 'no smoke is in the air here (take it on a snow level, or this row alone right after a can goes off)';
  if (s.sparse.indirect) return 'the solver is already on the dispatch of before here (?smokeindirect=1, or a software adapter)';
  if (!byGpuCount({ ...s.sparse, indirect: true }, s.stats.stepped, s.grid.cap)) return 'so much of the pool is stepping that the dispatch of before goes whole too: both sides would be one';
  return { from: { indirect: false }, to: { indirect: true }, apply: () => { s.sparse.indirect = true; return () => { s.sparse.indirect = false; }; } };
}
