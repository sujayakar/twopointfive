// Another module that does something as it loads. One of the names taken from it is only a TYPE: once the other goes, a transpiler drops the whole import
// unless it is kept as a bare one.
import { LOG } from './log';
LOG.loaded += 10;
export type Kind = 'a' | 'b';
export function onlyOff2(x: number): number { return x + 1; }
