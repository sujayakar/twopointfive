// THE CASES: each reads a row of K in one of the shapes the game's code does. tools/retire/retire-check.ts runs every one before the fold and after it and holds
// the two to the same answer, the same dice drawn and the same calls made (run.ts). A case is added here when the tool is found to fold a shape wrongly.
import { K, N, K_DEFAULTS, LOOSE } from './k';
import { die, tell, LOG } from './log';
import { onA, offA, both, Box } from './helper';
import { onlyOff } from './sidefx';
import { onlyOff2, Kind } from './sidefx2';
import { preset, TOGS } from '../ui/bindings';
import { onlyOff3 } from './sidefx3';
import * as S4 from './sidefx4';
import { useM } from './usem';
import { useM2 } from './usem2';

const men = [1, 2, 3];
const risky = (): number => { throw new Error('boom!'); };
/** a query the tool cannot see to the bottom of: the check vouches for it with `--pure vouched` */
const locksOf = { set: new Set(['who']), has(k: string): boolean { let n = 0; for (const x of this.set) if (x === k) n++; return n > 0; } };
const vouched = (k: string): boolean => locksOf.has(k);
/** things that are called like a reading method and are not one: each call does something */
const store = { get(k: string): number { return tell(k.length); }, round(x: number): number { return tell(x); } };
const seat = { get next(): number { return die(); } };
let swapped: () => number = () => 1; swapped = () => tell(7);
const withDefault = (x: number = tell(1)): number => x;
interface Seat { next: number }
const seat2: Seat = { get next(): number { return die(); } };   // (typed as plain data, and a getter all the same)
class Base { peek(): number { return 1; } }
class Sub extends Base { peek(): number { return tell(5); } }
const virt: Base = new Sub();   // (typed as its base: the body that runs is the override's)
const anyOf = (t: Record<string, unknown>, keys: string[]): boolean => keys.some(k => !!t[k]);
/** the game's own state kept in a list: each thing writes a row in one function and reads it in another */
const PHASES = [{ name: 'dawn', enter(): void { K.ph = true; }, active: (): boolean => K.ph }, { name: 'dusk', enter(): void { K.ph = false; }, active: (): boolean => !K.ph }];
/** read as this module loads: the page has written nothing yet, so these hold the rows' own values */
const atLoad = K.early ? 'on' : 'off';
const viaIife = (() => (K.iife ? 'on' : 'off'))();
const wideSq = N.wide * N.wide;
function mkLoad(): string { return K.e2 ? 'on' : 'off'; }
const viaFn = mkLoad();   // (a declared function, called as the module loads)
class Brain { fast = K.e3 ? 1 : 2; }
const brain = new Brain();   // (an object made as the module loads: its fields are worked out then)
const halfW = N.w2 / 2, w3sq = N['w3'] * 2;   // (held numbers read at load, one beside an operator and one by a quoted key)
/** things of the game's own with a method named like one of the language's that does NOT only read there */
const tidy = { sort(a: number[]): number[] { return a; }, delete(k: string): boolean { return k.length > 0; } };
class GBase { get val(): number { return 1; } }
class GSub extends GBase { get val(): number { return tell(6); } }
const gv: GBase = new GSub();   // (typed as its base: the getter that runs is the subclass's)
interface Sink { push(x: number): void }
const sink: Sink = { push(x: number): void { tell(x); } };
const viaSink = { get v(): number { sink.push(3); return 1; } };   // (a getter that calls through an interface)
/** a getter of the game's own that does something, named as a recording's key is (`k.tables`) */
const reg = { get tables(): number { return tell(9); } }; void reg;
/** a recording's own words: an untyped thing with a key named like the table */
const fromFile = (f: any): boolean => f.tables.K.extra === true;
const fastX = K.num > 2 && K.zero === 0; export { fastX };   // (rows whose own value is the page's: the same at load as later)

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
  switchCase(): number { for (const v of [1, 2]) switch (v) { case 1: if (K.old) tell(1); break; default: if (K.on) tell(2); } return LOG.calls; },
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
  // ---- a call is not harmless because of its name
  userGet(): number { return store.get('ab') > 5 && K.old ? 1 : 2; },
  userGetUnread(): number { const raw = store.get('abc'); if (K.old) return raw; return 0; },
  userRound(): number { if (store.round(2) > 5 && K.old) return 1; return 2; },
  getterDraws(): number { const g = seat.next; if (K.old) return g; return 0; },
  getterInCond(): number { return seat.next > 2 && K.old ? 1 : 2; },
  reassigned(): number { return swapped() > 5 && K.old ? 1 : 2; },
  defaultParam(): number { return withDefault() > 5 && K.old ? 1 : 2; },
  // ---- the same key on a thing that is not the table, and a read made before the page wrote
  snapshot(): boolean[] { return [K_DEFAULTS.snap, K.snap !== K_DEFAULTS.snap, K.snap]; },
  early(): string { return atLoad + (K.early ? '!' : '?'); },
  exported(): boolean { return fastX; },
  // ---- words and numbers
  mode(): number { if (K.mode === 'fast') return 1; return K.mode === 'slow' ? 2 : 3; },
  modeNot(): string { return K.mode !== 'slow' ? K.mode + '!' : '?'; },
  quote(): string { return K.quote + '!'; },
  numberCompare(): boolean[] { return [K.num > 2, K.num <= 2, K.zero < 1, K.num >= K.zero]; },
  andNumber(): unknown[] { const f = tell(1) > 0, g = tell(0) > 0; return [f && K.num, g && K.num, f || K.zero, g || K.zero, f && K.zero, g && K.zero]; },
  parenNumber(): string { return (K.on ? 3 : 4).toFixed(1); },
  // ---- an else that must stay with its own if
  danglingElse(): number[] { const out: number[] = []; for (const [o, a] of [[false, false], [true, false], [true, true]]) { let r = 0; if (o) if (a) r = 1; else K.old && tell(5); else r = 9; out.push(r); } return out; },
  danglingThen(): number[] { const out: number[] = []; for (const [o, a] of [[false, false], [true, false], [true, true]]) { let r = 0; if (o) if (a) K.old && tell(5); else r = 2; else r = 9; out.push(r); } return out; },
  // ---- more text that must come out well formed
  arrowAndObject(): unknown { const b = (g: number) => K.on && { budget: g }; return JSON.stringify(b(3)); },
  arrowAsObject(): unknown { type B = { budget?: number }; const b = (g: number) => K.on ? { budget: g } as B : {} as B; return b(4); },
  typeofConst(): string { const fastT = K.on && K.extra; type T = typeof fastT; const t: T = fastT; return String(t); },
  // ---- a key of a thing that is also read whole stays
  weights(): number { const WEIGHTS = { a: 1, b: 2, legacyC: 4 }; let t = 0; for (const v of Object.values(WEIGHTS)) t += v; if (K.old) t += WEIGHTS.legacyC; return t; },
  // ---- a part of the table under another name, a spread left with nothing to spread, a control kept as data
  nestedAlias(): number { const D = K.deep; return D.on ? D.gain : 0; },
  spreadNull(): unknown { return { a: 1, ...(K.old ? { sheath: true } : null) }; },
  controls(): string[] { return TOGS().filter(t => t.get() || !t.get()).map(t => t.label).filter(l => l === 'state'); },
  // ---- the third review's shapes
  importKept(): number { const names = { onlyOff3: 1 }; return K.old ? onlyOff3(1) + S4.f4() : LOG.loaded + names.onlyOff3; },
  loadTime(): string { return viaIife + (K.iife ? '!' : '?') + wideSq + '/' + N.wide; },
  listWrite(): boolean { for (const k of ['rt1', 'stay'] as const) K[k] = true; const v = K.rt1; for (const k of ['rt1', 'stay'] as const) K[k] = false; return v; },
  pageList(): boolean[] { return [K.kl1, K.selfref]; },
  helperKeys(): boolean { return anyOf(K, ['hl1', 'stay']); },
  weightsDeep(): number { const W2 = { calm: { a: 1, b: 2, legacyZ: 4 } }; let t = 0; for (const v of Object.values(W2.calm)) t += v; if (K.old) t += W2.calm.legacyZ; return t; },
  presetDeep(): unknown[] { const was = [K.negp, K.deep2.on]; preset('soft'); const now = [K.negp, K.deep2.on]; K.negp = was[0] as number; K.deep2.on = was[1] as boolean; return [was, now]; },
  phases(): string[] { PHASES[0].enter(); const on = PHASES.filter(p => p.active()).map(p => p.name); PHASES[1].enter(); return [...on, ...PHASES.filter(p => p.active()).map(p => p.name)]; },
  getterByKey(): number { return seat['next'] > 2 && K.old ? 1 : 2; },
  getterTyped(): number { return seat2.next > 2 && K.old ? 1 : 2; },
  override(): number { return virt.peek() > 5 && K.old ? 1 : 2; },
  typeofParen(): string { return typeof(K.on) + typeof(K.num); },
  returnParen(): number { return(K.on ? 1 : 2); },
  arrowOr(): unknown { const f = () => K.on && { a: 1 } || null; return f(); },
  bangCall(): number { const flag = (): boolean => tell(1) > 5; return (K.on ? !flag() : true).toString().length; },
  paramDefault(): string { const say = (line: string, bare = K.on && K.extra): string => line + (bare ? '!' : '?'); return say('x'); },
  // ---- the fourth review's shapes
  foldedAway(): number[] { return [useM(), useM2(), LOG.loaded]; },
  libSort(): number[] { void tidy; const xs = [3, 1, 2]; const s = xs.sort(); if (K.old) return s; return xs; },
  libDelete(): number { const seen = new Map([['a', 1], ['b', 2]]); const had = seen.delete('a'); if (K.old && had) return -1; return seen.size; },
  loadFn(): string { return viaFn + brain.fast + (K.e2 ? '!' : '?') + (K.e3 ? '!' : '?'); },
  loadHeld(): number[] { return [halfW, w3sq, N.w2, N.w3]; },
  weightsTop(): number { const W3 = { calm: { a: 1, b: 2, legacyY: 4 } }; let t = 0; for (const g of Object.values(W3)) for (const v of Object.values(g)) t += v; if (K.old) t += W3.calm.legacyY; return t; },
  getterOverride(): number { const g = gv.val; if (K.old) return g; return 0; },
  getterSink(): number { const g = viaSink.v; if (K.old) return g; return 0; },
  fromFile(): boolean { return fromFile(JSON.parse('{"tables":{"K":{"extra":false}}}')); },
  gainNamed(): number { return K.old ? K.oldGain * 2 : 1; },
  switchSubject(): number { switch (K.sw) { case 'a': return 1; default: return 2; } },
  // ---- an import one of whose names is only a type
  typeOnlyLeft(): number { const k: Kind = 'a'; return K.old ? onlyOff2(1) : LOG.loaded + k.length; },
  // ---- a held number: the knob stays, at the page's value
  held(): number { return N.reach + N.far; },
  // ---- rows the tool must refuse, and leave working
  runtimeWrite(): boolean { K.stay = !K.stay; return K.stay; },
  listRead(): boolean { return (['listRead', 'stay'] as const).some(k => K[k]); },
  preset(): boolean { const HARD: Partial<typeof K> = { preset: true }; Object.assign(K, HARD); return K.preset; },
  rides(): boolean { return K.rides; },
  forced(): boolean[] { const was = K.forced; preset('hard'); const now = K.forced; K.forced = was; return [was, now]; },
  loose(): boolean { return LOOSE.a; },
  wrong(): boolean { return K.wrong; },
  halves(h: 'half1' | 'stay' = 'stay'): boolean { return K[h]; },
};
