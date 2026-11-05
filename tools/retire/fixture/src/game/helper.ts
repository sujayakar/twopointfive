import { tell } from './log';
export function onA(): number { return 1; }
/** a recording's own word for a row, read outside the journal */
export function fileHadRule(f: { tables: Record<string, unknown> }): boolean { return (f.tables.K as { j6?: boolean } | undefined)?.j6 === true; }
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
