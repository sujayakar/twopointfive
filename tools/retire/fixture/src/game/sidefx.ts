// A module that does something as it loads: an import of it must stay where it was even when its last imported name goes.
import { LOG } from './log';
LOG.loaded++;
export function onlyOff(x: number): number { return x + 100; }
