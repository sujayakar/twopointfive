// ONE FIXED STREAM OF INPUTS THROUGH THE GAME'S MATHS, FOLDED — shared by tools/qa/rust-check.ts (bun, node) and tools/qa/rust-page.cjs (the page in Chromium,
// which loads this file as /qa/rust-fold.js from the dev server). Plain JavaScript on purpose: a page cannot load a TypeScript tool.
// The Rust side it is handed is src/sim/sim.ts mathsOfRust(): the very set the RUST.maths row puts in Math's place, its hypot of any number of terms included.
// The inputs are made from + − × ÷ alone (and decimal literals), so every engine is handed the same numbers. It is exact-check.ts's stream with more of the
// corners a port could get wrong: the decimal powers, whole and huge exponents, subnormals, the reduction on the bits out to the end of the range.
const F = new Float64Array(1), U = new Uint32Array(F.buffer);
function stream() {
  let s = 0x9e3779b9 >>> 0;
  const r = () => { s = (s + 0x6D2B79F5) >>> 0; let t = s; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
  const r53 = () => (Math.floor(r() * 67108864) * 134217728 + Math.floor(r() * 134217728)) / 9007199254740992;
  const ten = (lo, hi) => Number('1e' + Math.floor(lo + r() * (hi - lo)));
  return { r, r53, ten };
}
const PI = 3.141592653589793;
const SPEC = [0, -0, 1, -1, 0.5, -0.5, 2, -2, 3, 0.7, 0.66, 0.625, Infinity, -Infinity, NaN, 1e-320, 5e-324, 1.7976931348623157e308, 1e300, -1e300, 1e-300, PI, PI / 2, PI / 4, 709.78, 710, -745.2, -746,
  536870912, 1e15, 4503599627370497, 9007199254740993, 1e22, 10, 100, 1000, 1e-5, 1e23, 0.1, 8, 1024, 0.25, 2.2250738585072014e-308, 9223372036854775808, -3, 7, 0.7071067811865476, 2.41421356237309504880];
const DOM = {
  angle: [S => (S.r53() - 0.5) * 2 * PI, S => (S.r53() - 0.5) * 200, S => (S.r53() - 0.5) * 2e6, S => (S.r53() - 0.5) * 1e-3],
  unit: [S => S.r53() * 2 - 1, S => (S.r53() - 0.5) * 1e-4, S => 1 - S.r53() * 1e-6],
  any: [S => (S.r53() - 0.5) * 20, S => (S.r53() - 0.5) * 2e3, S => (S.r53() - 0.5) * 1e-2, S => (S.r53() - 0.5) * S.ten(-20, 20)],
  pos: [S => S.r53() * 10, S => S.r53() * 1e6, S => S.r53() * 1e-6, S => S.r53() * S.ten(-300, 300), S => 1 + (S.r53() - 0.5) * 1e-3, S => S.ten(-323, 309), S => S.r53() * 1e-310],
  expo: [S => (S.r53() - 0.5) * 40, S => (S.r53() - 0.5) * 1400, S => (S.r53() - 0.5) * 1e-3],
};
const ONE = [['sin', 'angle'], ['cos', 'angle'], ['tan', 'angle'], ['asin', 'unit'], ['acos', 'unit'], ['atan', 'any'], ['exp', 'expo'], ['log', 'pos'], ['log2', 'pos'], ['log10', 'pos'], ['tanh', 'any']];
const POW_Y = [2, 3, 4, 1.5, 2.2, 0.25, -1, -2, 1 / 3, 0.45, 2.4, 0.5, 5, 8, 0.8, 1e3, -37, 1e6, -0.5, 4503599627370497];
/** the same number: zeros by sign, NaN is NaN */
export const sameBits = (a, b) => a === b ? (a !== 0 || 1 / a === 1 / b) : (a !== a && b !== b);
/** how many answers a function's stream has at this n is the caller's to read off `asked` */
export const FOLD_N = 200000;
/** every function of `fns` (sin cos tan asin acos atan atan2 exp log log2 log10 pow tanh, and hypot of any number of terms) over the fixed stream: the word of
 *  all the answers' bits (NaN folded as the one NaN: an engine may hand a NaN back with other bits than it was given). With `other`, each answer is also held
 *  to that set's: how many were asked, how many differed, the first few that did. */
export function fold(fns, n = FOLD_N, other = null) {
  const S = stream(); let h = 0x811c9dc5, asked = 0, differ = 0; const first = [];
  const go = (name, ...a) => {
    let v = fns[name](...a); if (v !== v) v = NaN; F[0] = v; h = Math.imul(h ^ U[0], 0x01000193) >>> 0; h = Math.imul(h ^ U[1], 0x01000193) >>> 0; asked++;
    if (other) { const w = other[name](...a); if (!sameBits(v, w)) { differ++; if (first.length < 6) first.push(`${name}(${a.join(', ')}) = ${v} here, ${w} there`); } }
  };
  for (const [fn, dom] of ONE) { for (const x of SPEC) go(fn, x); for (const d of DOM[dom]) for (let i = 0; i < n; i++) go(fn, d(S)); }
  for (const x of SPEC) for (const y of SPEC) { go('atan2', x, y); go('pow', x, y); go('hypot', x, y); go('hypot', x, y, 1); go('hypot', x, 2, y, 1); go('hypot', x, 2, y, 1, 3); }
  for (const d of DOM.any) for (const e of DOM.any) for (let i = 0; i < n / 4; i++) go('atan2', d(S), e(S));
  for (const d of DOM.pos) for (const e of DOM.expo) for (let i = 0; i < n / 4; i++) go('pow', d(S), e(S));
  for (const y of POW_Y) for (let i = 0; i < n / 8; i++) { go('pow', S.r53() * 4, y); go('pow', -S.r53() * 4, y); }
  for (let i = 0; i < n; i++) { const a = (S.r53() - 0.5) * 60, b = (S.r53() - 0.5) * 60, c = (S.r53() - 0.5) * 6; go('hypot', a, b); go('hypot', a, b, c); go('hypot', a, b, c, a * b); go('hypot', a); go('hypot', a, b, c, 1, 2); go('hypot', a * 1e200, b * 1e-200, c); }
  for (let i = 0; i < 20000; i++) { const x = (S.r53() + 0.01) * 536870912 * (1 << Math.floor(S.r() * 20)); go('sin', x); go('cos', x); go('tan', x); }   // (the reduction on the bits: 2^29 and up)
  for (let i = 0; i < 20000; i++) { const x = (S.r53() + 0.5) * 2 ** Math.floor(29 + S.r() * 994); go('sin', x); go('cos', x); go('tan', x); go('sin', -x); }   // (… out to the end of the range)
  return { word: (h >>> 0).toString(16), asked, differ, first };
}
