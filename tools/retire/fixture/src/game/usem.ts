import { M } from './m';
import { tell } from './log';
/** the table is read only beside something else in a condition, and as a condition of its own: nothing of it is left after the fold */
export function useM(): number { const x = tell(1) > 0; if (M.on && x) return 1; return M.on ? 2 : 3; }
