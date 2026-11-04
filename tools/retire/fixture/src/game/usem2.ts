import { M } from './m';
import { tell } from './log';
/** the table is read into one name of two in a `const`: that name goes, the other stays */
export function useM2(): number { const fast = M.on, y = tell(2); return fast ? y : 0; }
