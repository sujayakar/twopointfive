// A table in a module that does something as it loads. The files that read it (usem.ts, usem2.ts) use it only in conditions the fold takes away whole: the
// import must still stand there, bare, or a bundler drops it and this module loads at another moment.
import { LOG } from './log';
LOG.loaded += 10000;
/** M: one switch */
export const M = { on: false };
