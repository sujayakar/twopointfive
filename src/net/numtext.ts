// NUMBERS AS JSON CARRIES THEM TO THE BIT. Plain JSON would do but for three values it cannot carry — a negative zero, NaN and the infinities — which ride as the
// strings '-0' / 'nan' / 'inf' / '-inf' (no key code is spelt like one).
/** JSON.stringify's replacer, and JSON.parse's reviver */
export const numWord = (_k: string, x: unknown): unknown => typeof x !== 'number' || (Number.isFinite(x) && !Object.is(x, -0)) ? x : x !== x ? 'nan' : x === 0 ? '-0' : x > 0 ? 'inf' : '-inf';
export const wordNum = (_k: string, x: unknown): unknown => x === '-0' ? -0 : x === 'nan' ? NaN : x === 'inf' ? Infinity : x === '-inf' ? -Infinity : x;
