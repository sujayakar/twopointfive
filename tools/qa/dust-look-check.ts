// bun run tools/qa/dust-look-check.ts — THE DUST'S LOOK IS PART OF THE MARCH'S ONE TEXT (src/render/shaders/dust_look.wgsl; until 2026-11-07 a second program, the march's text
// with five statements swapped). Held here as text, with no GPU: the two lit marches of volumetrics.wgsl read their density through dustDensity, tint both in-scatter
// terms and take the dust's self-shadow where the sample is dust, and dustDensity hands back the march's own masked read, with no tint, whenever the frame's dust bit
// is down (so a frame with no dust alive draws what it drew). exit 0 = so, 1 = the march or the dust's text has moved.
const vol = await Bun.file(new URL('../../src/render/shaders/volumetrics.wgsl', import.meta.url)).text(), dust = await Bun.file(new URL('../../src/render/shaders/dust_look.wgsl', import.meta.url)).text();
const count = (s: string, x: string): number => s.split(x).length - 1;
const ok = count(vol, 'let dl = dustDensity(X, onMask, true); let dens = fogSmoke(dl.d, X); let dTint = dustTintOf(dl);') === 2 && count(vol, 'inscatter += dTint * (') === 4 && count(vol, 'selfC[c] = smokeSelfShadowLook(X, ev.L, nearMask);') === 2
  && !vol.includes('let dens = fogSmoke(smokeDensityIn(X, onMask), X);')
  && dust.includes('fn dustAlive() -> bool { return (frame.sideCfg.z & 2u) != 0u; }') && dust.includes('if (!dustAlive()) { return DustDens(smokeDensityIn(p, mask), vec3f(0.0), 0.0); }');
console.log(ok ? `  PASS — both lit marches draw dust through dust_look.wgsl, and with the frame's dust bit down it is the plain read (${vol.length} chars of march, ${dust.length} of dust)` : '  FAIL — the march no longer reads its smoke through dustDensity at both lit marches, or dustDensity no longer opens on the plain read');
process.exit(ok ? 0 : 1);
