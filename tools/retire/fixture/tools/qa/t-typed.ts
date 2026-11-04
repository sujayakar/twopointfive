// A check that takes the table through a TYPED import, as some of tools/qa do: the row is found by its symbol, on a binding that is not the table's own name
// in the eyes of the compiler but is the table.
const FP = await import('../../src/game/k.ts') as typeof import('../../src/game/k');
const { K } = FP;
export const typedRead = (): boolean => K.on && !K.old;
const { K: TK } = FP;   // (under another name)
export const typedRead2 = (): boolean => TK.on || TK.stay;
/** a verdict reached through a thing's own function, which passes it on */
const line2 = (ok: boolean, what: string): void => { console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${what}`); };
const F = { verdict(ok: boolean, what: string): void { line2(ok, what); } };
F.verdict(K.on === true, 'the stand: on');
F.verdict(K.stay === false, 'a row that stays');
