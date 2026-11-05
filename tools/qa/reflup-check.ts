// A REFLECTION TEXEL OF ANOTHER SURFACE HAS NO SAY: the check behind composite.wgsl upsampleRefl (no switch: it is the upsampler's own rule, on every page that
// traces reflections).
//   bun run tools/qa/probes.ts reflup        (family `render`)        REFLUP_BREAK=1 = the plain bilinear blend, as it was: must FAIL
// The maintainer on his GPU, 11-04, the white roof's service door at midday: "weird artifacts in the specular ... around the edges, looks like it's
// blocky/rendered at a lower resolution". The reflections are traced at a half or a third of the picture's size and the composite took ONE bilinear fetch
// of them. At a jamb, a pixel of the dark room took a share of the texel traced on the sunlit ground seen past it; the one exposure model shows the room
// hundreds of times brighter than that ground, so the share was a bar the size of a texel along the edge. THE RULE: the four texels round the pixel, each
// by its bilinear weight × how near its own surface is to the pixel's (a bell over the depth's difference) × how like its stored normal is, with the
// plain fetch left in the blend at a hundred-thousandth for the pixel none of the four is like.
// WHAT THIS CAN AND CANNOT SAY. No shader runs here. Line 1 holds the rule BY ITS TEXT in both writings of the composite. Line 2 is the weights' arithmetic
// on pixels at a jamb, a sill and a bevel, with the numbers line 1 holds in the text. What the picture looks like is the page's to show (docs/agents/render.md).
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { ROOT } from './headless.ts';
export async function reflUpCheck(): Promise<boolean> {
  const BRK = process.env.REFLUP_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== reflup: a reflection texel of another surface has no say${BRK ? ' (the plain bilinear blend: must fail)' : ''} ===`);
  const read = (f: string) => readFileSync(resolve(ROOT, 'src/render/shaders', f), 'utf8'), texts = [read('composite.wgsl'), read('compact/composite.wgsl')];
  // 1. the text, in both writings: one function, asked with the read pixel's own distance and stored normal, four texels, the three weights, the fallback
  const W = 'let dz = (zs - z) / max(z * 0.02, 0.02);\n    let w = (select(1.0 - f.x, f.x, o.x == 1) * select(1.0 - f.y, f.y, o.y == 1) + 1e-4) * exp(-dz * dz) * pow(max(dot(textureLoad(gNormal, gpx(src), 0).xyz * 2.0 - 1.0, N), 0.0), 8.0);';
  const held = texts.map(t => t.split('fn upsampleRefl(pixf: vec2f, z: f32, N: vec3f) -> vec3f {').length === 2 && t.includes(W) && t.includes('let src = min(h * di + gi, inHi());') && t.includes('if (ds >= 1.0) { continue; }')
    && t.includes('return (acc + textureSampleLevel(reflTex, linSamp, (t + 0.5) / vec2f(dims), 0.0).xyz * 1e-5) / (wsum + 1e-5);') && t.includes('let t = (pf - 0.5 - goff) / rdiv;') && t.includes('let b = floor(t); let f = t - b; let di = i32(rdiv); let gi = i32(goff);') && t.includes('let o = vec2i(k & 1, k >> 1); let h = clamp(vec2i(b) + o, vec2i(0), dims - 1);') && t.includes('acc += textureLoad(reflTex, h, 0).xyz * w; wsum += w;') && t.includes('refl = lookShow(upsampleRefl(sPixf, z, Nl) * (reflK * albedo4.a), shadeG);') && t.includes('upsampleRefl(sPixf, z, Nl), frame.rtCfg.w >= 1.0)') && (t.match(/upsampleRefl\(/g) ?? []).length === 3);
  verdict(held.every(Boolean), 'both writings of the composite blend the four reflection texels round a pixel by bilinear weight, depth nearness and stored-normal likeness, asked with the read pixel\'s own distance and normal, and fall back to the plain fetch only where none is its surface',
    ['composite.wgsl', 'compact/composite.wgsl'].map((n, k) => `${n}: ${held[k] ? 'held' : 'NOT held'}`).join('; '));
  // 2. the arithmetic at a jamb: a pixel of the jamb's dark face 8 m from the eye, a third of the way between two columns of texels. The near column was
  //    traced on the jamb (its reflection 1 in the room's units), the far column on the sunlit ground 1.5 m further, a thousand times brighter. And a brick's
  //    face beside its bevelled edge: the same distance, the normal 25° off, ten times brighter.
  { const tol = (z: number) => Math.max(z * 0.02, 0.02), wz = (zs: number, z: number) => BRK ? 1 : Math.exp(-(((zs - z) / tol(z)) ** 2)), wn = (c: number) => BRK ? 1 : Math.pow(Math.max(c, 0), 8);
    // (t.b = a texel's bilinear weight; the plain fetch is the bilinear mean of the same four)
    const blend = (taps: { b: number; zs: number; c: number; L: number }[], z: number) => { let acc = 0, ws = 0, plain = 0; for (const t of taps) { const w = (t.b + 1e-4) * wz(t.zs, z) * wn(t.c); acc += t.L * w; ws += w; plain += t.L * t.b; } return (acc + plain * 1e-5) / (ws + 1e-5); };
    const two = (zs: number, c: number, L: number, z = 8) => blend([{ b: 0.67 * 0.5, zs: z, c: 1, L: 1 }, { b: 0.67 * 0.5, zs: z, c: 1, L: 1 }, { b: 0.33 * 0.5, zs, c, L }, { b: 0.33 * 0.5, zs, c, L }], z);
    const jamb = two(9.5, 1, 1000), sill = two(8.5, 1, 1000), far = blend([{ b: 0.335, zs: 14, c: 1, L: 1 }, { b: 0.335, zs: 14, c: 1, L: 1 }, { b: 0.165, zs: 15.5, c: 1, L: 1000 }, { b: 0.165, zs: 15.5, c: 1, L: 1000 }], 14), foot = two(8, 0, 1000);
    const bevel = blend([{ b: 0.25, zs: 8, c: 1, L: 1 }, { b: 0.25, zs: 8, c: 1, L: 1 }, { b: 0.25, zs: 8, c: Math.cos(25 * Math.PI / 180), L: 10 }, { b: 0.25, zs: 8, c: Math.cos(25 * Math.PI / 180), L: 10 }], 8);
    const flat = blend([{ b: 0.25, zs: 8.01, c: 1, L: 1 }, { b: 0.25, zs: 8.0, c: 1, L: 3 }, { b: 0.25, zs: 7.99, c: 0.999, L: 1 }, { b: 0.25, zs: 8.0, c: 0.999, L: 3 }], 8);
    const thin = blend([{ b: 0.25, zs: 12, c: 1, L: 5 }, { b: 0.25, zs: 12, c: 1, L: 7 }, { b: 0.25, zs: 12, c: 1, L: 5 }, { b: 0.25, zs: 12, c: 1, L: 7 }], 8);
    verdict(jamb < 1.05 && sill < 1.05 && far < 1.05 && foot < 1.05 && bevel < 4 && Math.abs(flat - 2) < 0.05 && Math.abs(thin - 6) < 0.05,
      'a pixel of a dark face keeps its own reflection beside a surface a thousand times brighter: past a jamb 1.5 m behind it, past a sill half a metre behind it, at 14 m, and where a jamb\'s foot meets the ground at a right angle; beside a bevel a brick\'s face takes less of the bevel\'s; on one flat surface the blend is the bilinear mean it was, and where none of the four is the pixel\'s surface it is the plain fetch',
      `own texels 1, the other's 1000 (the plain blend 330): past a jamb ${jamb.toFixed(3)}, past a sill 0.5 m behind ${sill.toFixed(3)}, at 14 m ${far.toFixed(3)}, a foot at a right angle ${foot.toFixed(3)} · a face beside a bevel 25° off: ${bevel.toFixed(2)} (its own 1, the bevel's 10; the plain blend 5.5) · one flat surface, texels 1 and 3: ${flat.toFixed(3)} (the plain mean 2) · a thing thinner than a texel, the four all 4 m behind it at 5 and 7: ${thin.toFixed(3)} (the plain mean 6)`); }
  console.log(all ? '=== reflup: ALL PASS ===' : '=== reflup: FAILED ===');
  return all;
}
if (import.meta.main) process.exit((await reflUpCheck()) ? 0 : 1);
