// THE CASES: each reads a row of K in one of the shapes the game's code does. tools/retire/retire-check.ts runs every one before the fold and after it and holds
// the two to the same answer, the same dice drawn and the same calls made (run.ts). A case is added here when the tool is found to fold a shape wrongly.
import { K, N } from './k';
import { die, tell, LOG } from './log';
import { onA, offA, both, Box } from './helper';
import { onlyOff } from './sidefx';

const men = [1, 2, 3];
const risky = (): number => { throw new Error('boom!'); };
/** a query the tool cannot see to the bottom of: the check vouches for it with `--pure vouched` */
const locksOf = { set: new Set(['who']), has(k: string): boolean { let n = 0; for (const x of this.set) if (x === k) n++; return n > 0; } };
const vouched = (k: string): boolean => locksOf.has(k);

export const CASES: Record<string, () => unknown> = {
  // ---- the plain folds
  cond(): number { if (K.on) return onA(); return offA(); },
  condElse(): number { if (K.old) { return offA(); } else { return both(2); } },
  ternary(): string { return K.on ? 'a' : 'b'; },
  not(): boolean { return !K.old; },
  and(): number { const x = tell(1) > 0; return x && K.on ? 1 : 2; },
  or(): number { const x = tell(1) > 5; return K.old || x ? 1 : 2; },
  compare(): boolean[] { const x = tell(0) === 0; return [K.on === true, K.old !== x, K.on === x]; },
  nullish(): boolean { return K.on ?? false; },
  template(): string { return `${K.on ? 'on' : 'off'} ${K.num} ${K.extra}`; },
  whileOff(): number { let n = 0; while (K.old && n < 3) n++; return n; },
  elseChain(): number { const x = tell(2); if (K.old) return tell(10); else if (x > 1) return tell(20); else return tell(30); },
  // (what follows a return the fold made plain can no longer be reached)
  unreachable(): number { if (K.on) return 1; tell(9); return 2; },
  nested(): number { if (K.extra) { if (K.on) { return tell(5); } return tell(6); } return tell(7); },
  localConst(): number { const fast = K.on && K.extra; if (fast) return 1; return offA(); },
  shorthand(): boolean { const on = K.on; const o = { on }; return o.on; },
  method(): number { const b = new Box(); b.add(2); if (K.old) b.addOld(1); return b.n; },
  importOnlyOff(): number { return K.old ? onlyOff(1) : LOG.loaded; },
  elseIfOff(): number { const x = tell(1); if (x > 5) return 1; else if (K.old) return tell(8); return 3; },
  statementAnd(): number { K.on && tell(3); K.old && tell(30); K.old || tell(300); return LOG.sum; },
  doWhile(): number { let n = 0; do { n++; } while (K.old && n < 5); return n; },
  forCond(): number { let n = 0; for (let i = 0; K.on && i < 3; i++) n += i; return n; },
  switchCase(v: number = 1): number { switch (v) { case 1: if (K.old) tell(1); break; default: if (K.on) tell(2); } return LOG.calls; },
  classField(): number { class C { fast = K.on ? 1 : 2; slow = K.old; } const c = new C(); return c.fast + (c.slow ? 10 : 0); },
  getter(): boolean { const f = () => K.on; return f(); },
  byWord(): boolean { return K['extra']; },
  unary(): number[] { return [-K.num, -K.neg, +K.num, K.num - K.neg, K.num * K.neg]; },
  typeOf(): string { return typeof K.on + (K.on ? '!' : '?'); },
  spreadRows(): string[] { const lk = (k: string) => vouched(k); return ['a', ...(lk('menu') || !K.old ? [] : ['menu']), ...(K.on ? ['on'] : []), 'z']; },
  // ---- what must still happen when its reader goes
  // (the die is drawn either way: only the swap is the switch's)
  dieDraw(): number[] { const out = [...men]; for (let i = 0; i < out.length; i++) { const j = Math.floor(die() * out.length); if (!K.on) { const t = out[i]; out[i] = out[j]; out[j] = t; } } return [...out, Math.floor(die() * 1000)]; },
  unreadCall(): number { const lp = tell(4); if (K.old) return lp; return 0; },
  drawThenOff(): boolean { return die() > 2 && K.old; },
  offThenDraw(): boolean { return K.old && die() > 2; },
  callbackDraws(): boolean { const xs = [0.1, 0.2]; const any = xs.find(x => die() > x + 5) !== undefined && K.old; return any; },
  callbackCalls(): boolean { if (men.some(x => { tell(x); return x > 5; }) && K.old) return true; return false; },
  // ---- a statement that is the whole body of something
  loopBody(): number { let count = 0; for (const m of men) if (K.old) tell(m); count++; return count; },
  loopVar(): number { for (const mate of men) { if (K.old) tell(mate); } return LOG.calls; },
  loopHeadDraws(): number { let n = 0; for (let i = 0; i < 3 && die() >= 0; i++) if (K.old) n++; return n; },
  catchVar(): number { try { return risky(); } catch (boom) { if (K.old) tell(String(boom).length); } return -1; },
  // ---- text that must come out well formed
  numberDot(): string { return K.num.toFixed(2); },
  numberMinus(a: number = 4): number { return a-K.neg; },
  arrowObject(): unknown { const budgetOf = (g: number) => K.on ? { budget: g } : {}; return { ...budgetOf(3) }; },
  urlInString(): string { const u = 'http://x'; if (K.old) tell(2); return u; },
  // ---- a held number: the knob stays, at the page's value
  held(): number { return N.reach + N.far; },
  // ---- rows the tool must refuse, and leave working
  runtimeWrite(): boolean { K.stay = !K.stay; return K.stay; },
  listRead(): boolean { return (['listRead', 'stay'] as const).some(k => K[k]); },
  preset(): boolean { const HARD: Partial<typeof K> = { preset: true }; Object.assign(K, HARD); return K.preset; },
};
