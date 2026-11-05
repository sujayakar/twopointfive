// THE HUE SPLIT ASKS THE BELIEF THE HUE WAS BUILT BY: the check behind the place of the dark memory's update in restir.wgsl (no switch: it is the estimator's own
// rule put right, on every page).
//   bun run tools/qa/probes.ts huesplit        (family `render`)        HUESPLIT_BREAK=1 = the memory is moved before the shade, as it was: must FAIL
// The maintainer on his GPU, 11-04, the depot at midday: "red rim on shadow also looks odd". Every shadow the sun threw was edged in red. The red was the mast's
// obstruction lamp, which sends that ground under a ten-thousandth of the sun's light: with the lamp's colour set to black the edge was neutral, and with every
// sample shaded in its own lamp's hue (settings.restirSampleHue) it was neutral too. THE FAULT: the estimator shades a class's light in the hue of the lamps the
// pixel BELIEVES visible (Uv), and a pixel leaves out the lamp its own ray last found dark (darkSlot). The HUE SPLIT then lets a sample of a lamp that is not
// believed keep its own hue. But Uv was summed by the memory as the frame found it, and the split asked it after this frame's rays had moved it. In a half
// shadow a pixel's ray is blocked one frame and through the next: Uv had left the sun out, the ray came through and cleared the memory, the split called the
// sun's sample believed, and the sun's light was painted in the hue of what was left in Uv, the far dim lamp. THE MEND IS AN ORDER: the memory is moved after
// the shade, for the next frame, so the split and the sum read one value.
// WHAT THIS CAN AND CANNOT SAY. No shader runs here. Line 1 holds the order BY ITS TEXT in both writings of the estimator. Line 2 is A MODEL, not the
// estimator: one ray a frame toward the lamp, no history sample and no neighbour's, which in the kernel thin the fault (a still of the depot measured about a
// third of a half shadow's light in the lamp's hue where the model says a half). It takes the order from the text line 1 read, so it shows what that order
// costs. What the picture looks like is the page's to show; it was drawn on the software adapter only (docs/agents/render.md, THE HUE SPLIT ASKS THE BELIEF).
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { ROOT } from './headless.ts';
export async function hueSplitCheck(): Promise<boolean> {
  const BRK = process.env.HUESPLIT_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== huesplit: a sample keeps its own hue when the hue left its lamp out${BRK ? ' (the memory moved before the shade: must fail)' : ''} ===`);
  const read = (f: string) => readFileSync(resolve(ROOT, 'src/render/shaders', f), 'utf8'), texts = [read('restir.wgsl'), read('compact/restir.wgsl')];
  // 1. the text, in both writings: the memory is read from the texel and carried to this frame's numbering, the hue is summed by it, the split's three
  //    questions (the history's sample, a fresh one, a neighbour's) and the shade follow, and ONLY THEN is it moved; nothing assigns it in between
  const SUM = 'if (!dk && i + 1u != darkSlot) { Uv[c] += ev.E; }', SHADE = 'if ((ru.cfg.w & 128u) != 0u) { shade = Uv[c] * (max(sLum - wOut, 0.0) / lv) + colOut; }', SET = 'if (VF[c] == 0.0 && pickF[c] < 63u && !tileDark(bel, pickF[c])) { darkSlot = pickF[c] + 1u; }', CLEAR = 'else if ((VF[c] > 0.0 && pickF[c] + 1u == darkSlot) || (validT[c] && VT[c] > 0.0 && curT[c] + 1u == darkSlot)) { darkSlot = 0u; }', STORE = 'stw.y = stw.y | ((darkSlot & 7u) << 28u);';
  const orderOf = (t: string) => { const map = t.indexOf('if (darkSlot != 0u) { let ns = fwdSlot(darkSlot - 1u);'), sum = t.indexOf(SUM), shade = t.indexOf(SHADE), set = t.indexOf(SET), clear = t.indexOf(CLEAR), store = t.indexOf(STORE);
    const q = [t.indexOf('if (tileDark(bel, curT[c]) || curT[c] + 1u == darkSlot) { wOut = wsum; colOut = col; }'), t.indexOf('if (tileDark(bel, slot) || slot + 1u == darkSlot) { wOut += wj; colOut += hueOf(slot) * wj; }'), t.indexOf('if (tileDark(bel, cs) || cs + 1u == darkSlot) { nOut += wi; ncolOut += hueOf(cs) * wi; }')];
    const found = map > 0 && sum > map && q.every(i => i > sum) && shade > Math.max(...q) && set > 0 && clear > set && store > 0, between = found ? t.slice(sum, shade) : '';
    return { found, after: found && set > shade && clear < store && !/darkSlot\s*[-+|&^]?=(?!=)/.test(between), once: [SUM, SHADE, SET, CLEAR, STORE].every(s => t.split(s).length === 2) && !/darkIn/.test(t) }; };
  const order = texts.map(orderOf);
  verdict(order.every(o => o.found && o.after && o.once), 'both writings of the estimator sum the hue and ask the split\'s three questions by the pixel\'s dark memory as the frame found it: the memory is moved after the shade, for the next frame, and nothing assigns it between the sum and the shade',
    ['restir.wgsl', 'compact/restir.wgsl'].map((n, k) => `${n}: the lines ${order[k].found ? 'found' : 'NOT found'}, the move ${order[k].after ? 'after the shade' : 'NOT after the shade'}`).join('; '));
  // 2. THE MODEL, two frames of one pixel. A white lamp that carries all but a millionth of the pixel's light, a red one that carries the rest and is always
  //    seen. The pixel is in the white lamp's half shadow: a ray toward it comes through with chance v, each frame its own. Last frame's ray decides the
  //    memory the frame finds (blocked: the white lamp is left out of the hue, which is then the red lamp's alone); this frame's ray, if it comes through,
  //    brings the white lamp's light and clears the memory. The order is the text's (line 1): moved before the shade, the split asks the cleared memory.
  { const lum = (c: number[]) => 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2], hue = (c: number[]) => c.map(x => x / lum(c)), WHITE = hue([1, 0.97, 0.9]), RED = hue([1, 0.16, 0.1]), dim = 1e-6;
    const shade = (v: number, movedFirst: boolean) => { const out = [0, 0, 0]; let red = 0, sum = 0;
      for (const lastThrough of [true, false]) { const p = (lastThrough ? v : 1 - v) * v;   // (this frame's ray through: a blocked one brings nothing)
        const found = !lastThrough /* the memory as the frame finds it: the white lamp named dark */, asked = movedFirst ? false /* cleared by this frame's ray */ : found;
        const Uv = found ? RED.map(x => x * dim) : WHITE.map((x, k) => x * (1 - dim) + RED[k] * dim), w = 1 - dim, c = asked ? WHITE.map(x => x * w) : hue(Uv).map(x => x * w);
        for (let k = 0; k < 3; k++) out[k] += p * c[k]; sum += p * w; if (found && !asked) red += p * w; }
      return { rg: out[0] / out[1], red: red / sum }; };
    const moved = BRK || !order.every(o => o.after), half = shade(0.5, moved), thin = shade(0.1, moved), was = shade(0.5, true), want = WHITE[0] / WHITE[1];
    verdict(Math.abs(half.rg - want) < 1e-4 && half.red === 0 && Math.abs(thin.rg - want) < 1e-4 && thin.red === 0 && was.red > 0.49 && was.rg > 2,
      'a model of one pixel, one ray a frame: in a lamp\'s half shadow its light keeps its own colour beside a far dim lamp of another, by the order the text has',
      `red over green of a white lamp's light, half seen: ${half.rg.toFixed(3)} (its own ${want.toFixed(3)}), a tenth seen: ${thin.rg.toFixed(3)}; the share painted in the dim lamp's hue: ${(100 * half.red).toFixed(0)} % and ${(100 * thin.red).toFixed(0)} %. With the memory moved first the model paints ${(100 * was.red).toFixed(0)} % of a half shadow's light in the dim lamp's hue, red over green ${was.rg.toFixed(2)} (the kernel's history and neighbour samples thin that: a still measured about a third)`); }
  console.log(all ? '=== huesplit: ALL PASS ===' : '=== huesplit: FAILED ===');
  return all;
}
if (import.meta.main) process.exit((await hueSplitCheck()) ? 0 : 1);
