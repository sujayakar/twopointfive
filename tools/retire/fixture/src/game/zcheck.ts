// padding padding padding padding padding padding padding padding padding padding padding padding padding padding padding padding
// padding padding padding padding padding padding padding padding padding padding padding padding padding padding padding padding
// (the padding puts zCheck at the same offsets in this file as its calls have in caller.ts: a tool that compares offsets without comparing files leaves them)
import { tell } from './log';
/** a check nobody wants any more: `--drop src/game/zcheck.ts:zCheck` takes it out, with every statement that calls it */
export function zCheck(): boolean { return tell(1) > 0; }
export function zKeep(): number { return 2; }
