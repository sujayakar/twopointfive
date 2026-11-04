import { tell } from './log';
export function onA(): number { return 1; }
export function offA(): number { return offDeep() + 1; }
/** used by offA alone: goes when offA goes */
function offDeep(): number { return 40; }
export function both(x: number): number { return x * 2; }
export class Box {
  n = 0;
  add(x: number): number { this.n += x; return this.n; }
  /** called from the off branch alone */
  addOld(x: number): number { this.n -= x; return tell(this.n); }
}
