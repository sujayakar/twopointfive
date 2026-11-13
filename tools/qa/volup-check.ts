// THE SMOKE AND THE BEAMS COME UP TO THE PICTURE'S SIZE WITHOUT A STEP (src/render/shaders/composite.wgsl upsampleVol, volWindow): a bright spot and a ramp on the pass's texels, read at every place between
// two of them, in one dimension (the shares of the two axes are multiplied). Where the nearest texel changes nothing must jump. A pixel's shares are held to the shader's own lines.
//   bun run tools/qa/volup-check.ts              exit 0 = passed
//   VOLUP_BREAK=1 …                              the shares cut off where the nine texels end, as they were: must FAIL
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const BREAK = process.env.VOLUP_BREAK === '1', SRC = readFileSync(resolve(import.meta.dir, '../../src/render/shaders/composite.wgsl'), 'utf8'), VOL = readFileSync(resolve(import.meta.dir, '../../src/render/shaders/volumetrics.wgsl'), 'utf8');
let red = 0; const check = (ok: boolean, what: string): void => { if (!ok) red++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}`); };
/** what a texel lets in at `t` texels away: a quadratic B-spline, nought at a texel and a half */
const win = (t: number): number => { const a = Math.abs(t); return a < 0.5 ? 0.75 - a * a : a < 1.5 ? 0.5 * (1.5 - a) ** 2 : 0; };
/** the upsampled value at `x` (in texels) of `data`, one texel a place; `old` = the gaussian of before, three texels round the nearest */
const up = (data: readonly number[], x: number): number => { const b = Math.floor(x + 0.5); let acc = 0, w = 0; for (let d = -1; d <= 1; d++) { const i = b + d, k = BREAK ? Math.exp(-0.6 * (i - x) ** 2) : win(i - x); acc += k * (data[i] ?? 0); w += k; } return w > 1e-9 ? acc / w : 0; };
const N = 24, spot = Array.from({ length: N }, (_, i) => i === 12 ? 1 : 0), ramp = Array.from({ length: N }, (_, i) => i / N);
for (const [name, data] of [['a bright spot', spot], ['a ramp', ramp]] as const) {
  let jump = 0, smooth = 0; for (let x = 6; x < 18; x += 0.001) { const d = Math.abs(up(data, x + 0.001) - up(data, x)); jump = Math.max(jump, d); smooth = Math.max(smooth, Math.abs(up(data, x + 0.5) - up(data, x))); }
  check(jump < 0.01, `${name}: between two places a thousandth of a texel apart the value moves by ${jump.toFixed(4)} at the most, of ${smooth.toFixed(3)} a half texel`);
}
check([0, 0.25, 0.5].every(f => Math.abs([-1, 0, 1].reduce((s, d) => s + win(d - f), 0) - 1) < 1e-9), 'the three shares of an axis come to one wherever the pixel stands');
check(/let w = select\(0\.5 \* \(1\.5 - a\) \* \(1\.5 - a\), 0\.75 - a \* a, a < vec2f\(0\.5\)\)/.test(SRC) && /var w = volWindow\(vec2f\(base \+ vec2i\(dx, dy\)\) - at\);/.test(SRC) && /let full = min\(gid\.xy \* vdiv \+ vdiv \/ 2u/.test(VOL) && /let src = min\(h \* vdiv \+ vdiv \/ 2, inHi\(\)\);/.test(SRC), 'the shader takes these shares, and the pass marches the pixel the upsample reads from');
console.log(`=== volup: ${red ? `${red} FAILED` : 'ALL PASS'} ===`);
process.exit(red ? 1 : 0);
