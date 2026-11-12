// Static check of what two languages of the renderer say twice: the shadow ratio clamp's bound, direct_common.wgsl RATIO_HI = render/denoise.ts's knob ratioHi.
//   bun run tools/qa/frame-layout.ts        (exit 1 on any FAIL)
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const ROOT = resolve(import.meta.dir, '../..');

let fails = 0;
const verdict = (ok: boolean, label: string, detail = '') => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}${detail ? `\n    ${detail}` : ''}`); };

// ---------------------------------------------------------------- one number said in two languages: the shadow ratio clamp's bound
{
  const wg = /^const RATIO_HI\s*:\s*f32\s*=\s*([0-9.]+);/m.exec(readFileSync(`${ROOT}/src/render/shaders/direct_common.wgsl`, 'utf8'))?.[1];
  const ts = /\bratioHi: ([0-9.]+),/.exec(readFileSync(`${ROOT}/src/render/denoise.ts`, 'utf8'))?.[1];   // (read as text: the module asks for a GPU at load)
  verdict(wg !== undefined && ts !== undefined && Number(wg) === Number(ts), 'the shadow ratio clamp has one bound: direct_common.wgsl RATIO_HI (sharp direct) = the denoiser knobs\' ratioHi', `RATIO_HI ${wg} · ratioHi ${ts}`);
}

console.log(`\n=== frame-layout: ${fails ? `${fails} FAIL` : 'ALL PASS'} ===`);
export const frameLayoutFailures = () => fails;
if (import.meta.main && fails) process.exit(1);
