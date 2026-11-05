import { zCheck, zKeep } from './zcheck';
// pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad
// pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad
// pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad pad
export function run(v: number): number {
  switch (v) { case 1: return zKeep(); default: zCheck(); return 3; }
}
export function run2(): number { zCheck(); return zKeep(); }
export const LIST = [zCheck, zKeep];
