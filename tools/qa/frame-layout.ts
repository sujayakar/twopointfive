// Static check of the renderer's shared tables — the things four merged lanes each appended to and nothing but the GPU would complain about:
//   1. BoxFlag (boxes.ts) ⇄ BOX_* (common.wgsl): distinct bits; the GPU's names carry the CPU's values.
//   2. the shadow ratio clamp's bound: direct_common.wgsl RATIO_HI = render/denoise.ts's knob ratioHi.
//   bun run tools/qa/frame-layout.ts        (exit 1 on any FAIL)
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const ROOT = resolve(import.meta.dir, '../..');
const { BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);
const common = readFileSync(`${ROOT}/src/render/shaders/common.wgsl`, 'utf8');

let fails = 0;
const verdict = (ok: boolean, label: string, detail = '') => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}${detail ? `\n    ${detail}` : ''}`); };

const isPow2 = (v: number) => v > 0 && (v & (v - 1)) === 0 && v <= 0x80000000;
const norm = (s: string) => s.toLowerCase().replace(/_/g, '');
/** A WGSL u32 literal as the shader tables spell it — decimal `64u`, hex `0x40u`, or a shift `(1u << 6u)`; NaN for anything else (reported as a FAIL by the callers, never skipped). */
const lit = (t: string): number => { t = t.trim().replace(/u\b/g, ''); let m: RegExpMatchArray | null; if (/^\d+$/.test(t)) return +t; if (/^0x[0-9a-fA-F]+$/.test(t)) return parseInt(t, 16); if ((m = t.match(/^\(?\s*(\d+)\s*<<\s*(\d+)\s*\)?$/))) return (+m[1] * 2 ** +m[2]) >>> 0; return NaN; };
// ---------------------------------------------------------------- 1. box flags
console.log('\n=== frame-layout: BoxFlag (boxes.ts) vs BOX_* (common.wgsl) ===');
{ const host = Object.entries(BoxFlag as Record<string, number>).filter(([, v]) => typeof v === 'number');
  const dup = host.filter(([, v], i) => host.findIndex(([, w]) => w === v) !== i);
  verdict(host.every(([, v]) => isPow2(v)) && dup.length === 0, `BoxFlag: ${host.length} names, each a distinct single bit`, dup.map(([n]) => n).join(', '));
  const wg = [...common.matchAll(/^const (BOX_[A-Z0-9_]+)\s*:\s*u32\s*=\s*([^;]+);/gm)].map(m => [m[1], lit(m[2])] as [string, number]);
  const byName = new Map(host.map(([n, v]) => [norm(n), v]));
  const rows: string[] = [];
  for (const [n, v] of wg) { const hv = byName.get(norm(n.replace(/^BOX_/, ''))); if (!Number.isFinite(v)) rows.push(`${n}: literal not understood`); else if (hv === undefined) rows.push(`${n} (${v}) has no BoxFlag entry`); else if (hv !== v) rows.push(`${n} = ${v} but BoxFlag = ${hv}`); }
  verdict(rows.length === 0 && wg.length >= 6, `every BOX_ const (${wg.length} ≥ 6 read) names a BoxFlag of the same value`, rows.join(' · '));
}

// ---------------------------------------------------------------- 2. one number said in two languages: the shadow ratio clamp's bound
{
  const wg = /^const RATIO_HI\s*:\s*f32\s*=\s*([0-9.]+);/m.exec(readFileSync(`${ROOT}/src/render/shaders/direct_common.wgsl`, 'utf8'))?.[1];
  const ts = /\bratioHi: ([0-9.]+),/.exec(readFileSync(`${ROOT}/src/render/denoise.ts`, 'utf8'))?.[1];   // (read as text: the module asks for a GPU at load)
  verdict(wg !== undefined && ts !== undefined && Number(wg) === Number(ts), 'the shadow ratio clamp has one bound: direct_common.wgsl RATIO_HI (sharp direct) = the denoiser knobs\' ratioHi', `RATIO_HI ${wg} · ratioHi ${ts}`);
}

console.log(`\n=== frame-layout: ${fails ? `${fails} FAIL` : 'ALL PASS'} ===`);
export const frameLayoutFailures = () => fails;
if (import.meta.main && fails) process.exit(1);
