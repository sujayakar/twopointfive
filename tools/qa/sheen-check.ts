// A HIGHLIGHT OUTLIVES THE RAYS' CUT-OFF, A TRIAL: the check behind the scene prelude's SHEEN_PAST (render/frame.ts sheenConfig; reflect.wgsl main,
// composite.wgsl reflK; the look's page with `&sheen=1`, OFF unless asked; without the word the one cut-off and the composite's fade under it, as it is).
//   bun run tools/qa/probes.ts sheen        (family `render`)        SHEEN_BREAK=1 = the word refused and one cut-off for the whole term: must FAIL
// The maintainer on his GPU, 11-04: "overcast dark room has specular sparkles". On the white roof's rolled skim (finish 0.45, High's cut-off 0.45) the lamp's
// highlight followed the floor's roughness map pixel for pixel (r = −0.72 in a still): the reflection term was faded to nothing over the 0.12 of roughness
// under the rays' cut-off, so a grain a twentieth smoother than its neighbour was lit and the neighbour was not. THE RULE: the cut-off is what a RAY costs,
// and it fades the rays alone; the highlights (the analytic sums of the lamps, the panels and the sun) go on SHEEN_PAST rougher, where a highlight is broad
// and faint, and the composite fades the term over that whole reach.
// WHAT THIS CAN AND CANNOT SAY. No shader runs here. Line 1 holds the rule BY ITS TEXT in the three writings of the reflection pass and the two of the
// composite, with the page's wiring. Line 2 asks the prelude for its constant on and off the look's page. Line 3 is the two fades' arithmetic on a GGX
// highlight's peak, with the numbers line 1 holds in the text: the brightest highlight a twentieth of roughness can put out. What the picture looks
// like is the page's to show (docs/agents/render.md).
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { ROOT } from './headless.ts';
export async function sheenCheck(): Promise<boolean> {
  const BRK = process.env.SHEEN_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== sheen: a highlight outlives the rays' cut-off${BRK ? ' (one cut-off for the whole term: must fail)' : ''} ===`);
  const src = (f: string) => readFileSync(resolve(ROOT, 'src', f), 'utf8'), sh = (f: string) => src(`render/shaders/${f}`);
  // 1. the text: the pass gives a term as far as the reach, gates its rays by their own fade and scales them by it; the composite fades the term over the reach
  const passes = ['reflect.wgsl', 'reflect_p5.wgsl', 'compact/reflect_p5.wgsl'], comps = ['composite.wgsl', 'compact/composite.wgsl'];
  const RAY = '  var rayK = 1.0; var rays = min(u32(frame.rtCfg.y), 4u);\n  if (SHEEN_PAST > 0.0) { rayK = 1.0 - smoothstep(maxR - 0.12, maxR + 0.001, rough); if (rough > maxR) { rays = 0u; } }';
  const heldP = passes.map(f => { const t = sh(f); return t.includes('  if (rough > select(maxR, max(min(maxR + SHEEN_PAST, 0.999), maxR + 0.001), SHEEN_PAST > 0.0)) { textureStore(outRefl, gid.xy, none);') && !t.includes('if (rough > maxR) { textureStore(') && t.includes(RAY)
    && t.includes('  var Lspec = Lsum / f32(max(rays, 1u));\n  if (SHEEN_PAST > 0.0) { Lspec *= rayK; }\n') && t.includes('select(1.0 - kEmit, 1.0, rays == 0u)') && t.includes('if (rays == 0u) { hitT0 = select(RT_TMAX, g.w, g.w > 0.0); }'); });
  const OLD = '      reflK = (1.0 - smoothstep(maxR - 0.12, maxR + 0.001, mt.x)) * frame.matCfg.y;\n      metalK = select(0.0, reflK * mt.y, frame.rtCfg.y >= 1.0);\n';
  const heldC = comps.map(f => sh(f).includes(OLD + '      if (SHEEN_PAST > 0.0) { reflK = (1.0 - smoothstep(maxR, max(min(maxR + SHEEN_PAST, 0.999), maxR + 0.001), mt.x)) * frame.matCfg.y; }'));
  const mainTs = src('main.ts'), at = mainTs.indexOf("if (q.get('sheen') === '1') sheenConfig.on = true;"), eng = mainTs.indexOf('const engine = new Engine(');
  const wired = at > 0 && eng > at && /VISIT_FLAGS = \[[^\]]*'sheen'/.test(src('ui/replaykey.ts')) && /LOOK_PAGE_WORDS[^\n]*'sheen'/.test(src('scene/look.ts'))
    && src('engine.ts').includes('K.spMaxRough = sheenTop(s.rtMaxRough);');
  verdict(heldP.every(Boolean) && heldC.every(Boolean) && wired,
    'the three writings of the reflection pass give a term as far as the reach, trace no ray past the cut-off and scale their rays by the rays\' own fade, each new line behind the constant; both writings of the composite fade the term over the reach and step a metal\'s paint back by the rays\' fade; the page reads &sheen=1 before it makes its engine, a replay\'s visit keeps the word, and the denoiser is told where a texel holds no term',
    [...passes.map((n, k) => `${n}: ${heldP[k] ? 'held' : 'NOT held'}`), ...comps.map((n, k) => `${n}: ${heldC[k] ? 'held' : 'NOT held'}`), `wiring: ${wired ? 'held' : 'NOT held'}`].join('; '));
  // 2. the constant: the reach on the look's page WITH the word, 0 without it and off the page (0 = every line of the shaders as it is)
  const { lookConfig, sheenConfig, sheenPast, sheenTop, scenePrelude } = await import(`${ROOT}/src/render/frame.ts`);
  const cuts = [...src('ui/quality.ts').matchAll(/^  (ultra|high|medium):[^\n]*?rt: \{ on: true, div: \d+, rays: \d+, maxRough: ([0-9.]+),/gm)].map(x => [x[1], +x[2]] as [string, number]).sort((x, y) => x[1] - y[1]), high = cuts.findIndex(x => x[0] === 'high'), HI = high >= 0 ? cuts[high][1] : NaN;   // (the presets' cut-offs, off their table)
  const was = [lookConfig.on, sheenConfig.on]; let a: string | undefined, b: string | undefined, c: string | undefined, top = NaN, past = NaN;
  const word = (look: boolean, sw: boolean) => { lookConfig.on = look; sheenConfig.on = sw; return /const SHEEN_PAST: f32 = ([0-9.]+);/.exec(scenePrelude())?.[1]; };
  let top0 = NaN; const off = was[1];
  try { a = word(true, !BRK); past = sheenPast(); top = sheenTop(HI); b = word(true, false); top0 = sheenTop(HI); c = word(false, true); } finally { lookConfig.on = was[0]; sheenConfig.on = was[1]; }
  verdict(a === '0.250' && b === '0.000' && c === '0.000' && past === 0.25 && cuts.length === 3 && Math.abs(top - (HI + 0.25)) < 1e-9 && top0 === HI && off === false,
    'the trial ships DOWN: the prelude says the reach only on the look\'s page with the word, a quarter of the roughness scale past the preset\'s cut-off; 0 anywhere else, which is the shaders as they are, and the denoiser\'s bound is then the cut-off itself',
    `the look's page with &sheen=1 ${a}, without it ${b}, off the look's page ${c}; the denoiser's bound at High ${top.toFixed(2)} with the word and ${top0.toFixed(2)} without; the switch as the file has it: ${off ? 'ON' : 'off'}`);
  // 3. the arithmetic. A lamp's highlight at its peak is the lobe's height, 1 ∕ (π α²) with α = roughness², times what the fade leaves of it. Two grains of
  //    one finish, a twentieth smoother and a twentieth rougher than it: the lobe alone makes the smoother one 1.49 times the brighter, at any roughness.
  //    The fade's own share is what is asked: the brightest highlight on the smoother grain while the rougher one is left under a tenth of it.
  { const ss = (e0: number, e1: number, x: number) => { const t = Math.min(Math.max((x - e0) / (e1 - e0), 0), 1); return t * t * (3 - 2 * t); }, D = (r: number) => 1 / (Math.PI * Math.pow(Math.max(r * r, 0.0009), 2));
    const fadeOld = (r: number, m: number) => 1 - ss(m - 0.12, m + 0.001, r), fadeNew = (r: number, m: number) => (!(past > 0) ? fadeOld(r, m) : 1 - ss(m, Math.min(m + past, 0.999), r));   // (the fade the composite makes with the constant line 2 was handed: under the break that is none, and the old fade)
    const out = (fade: (r: number, m: number) => number, m: number) => { let worst = 0, at = 0; for (let r0 = 0.1; r0 <= 1; r0 += 0.001) { const hi = fade(r0 * 0.95, m) * D(r0 * 0.95), lo = fade(r0 * 1.05, m) * D(r0 * 1.05); if (lo <= 0.1 * hi && hi > worst) { worst = hi; at = r0; } } return [worst, at]; };
    const rows = cuts.map(x => x[1]).map(m => ({ m, o: out(fadeOld, m), n: out(fadeNew, m), keep: fadeNew(m, m), skim: [fadeNew(0.38, m) * D(0.38), fadeNew(0.51, m) * D(0.51), fadeOld(0.38, m) * D(0.38), fadeOld(0.51, m) * D(0.51)] }));
    const lobe = D(0.38) / D(0.51), ok = cuts.length === 3 && high >= 0 && rows.every(x => x.n[0] < 0.2 * x.o[0]) && rows[high].skim[0] / rows[high].skim[1] < 1.25 * lobe && rows[high].skim[1] > 0;
    verdict(ok, 'WITH THE WORD, a twentieth of roughness does not put a lamp\'s highlight out where it is bright: at each preset\'s cut-off the brightest highlight a grain can lose to its neighbour is under a fifth of what it is without the word (it has moved to the far end of the reach, where a highlight is faint), and the highlights on the white roof\'s skim (its grains 0.38 to 0.51 on High) differ from grain to grain within a quarter of what the lobe alone gives. The rays under the cut-off keep their steep fade either way: this line says nothing of them',
      rows.map(x => `cut-off ${x.m}: ${x.o[0].toFixed(1)} at finish ${x.o[1].toFixed(2)} without the word, ${x.n[0].toFixed(2)} at ${x.n[1].toFixed(2)} with it (the lobe's height, a mirror's share of a unit of light)`).join(' · ')
      + ` · the skim's smoothest and roughest grains on High: ${rows[high].skim[2].toFixed(1)} and ${rows[high].skim[3].toFixed(1)} without the word, ${rows[high].skim[0].toFixed(1)} and ${rows[high].skim[1].toFixed(1)} with it (the lobe alone: ${lobe.toFixed(2)} to 1)`); }
  console.log(all ? '=== sheen: ALL PASS ===' : '=== sheen: FAILED ===');
  return all;
}
if (import.meta.main) process.exit((await sheenCheck()) ? 0 : 1);
