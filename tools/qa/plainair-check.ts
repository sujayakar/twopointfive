// PLAIN AIR (src/gpu/device.ts PLAIN_AIR) held to the texts the engine hands the browser. The Engine is built on a stand-in device that records every shader module
// and pipeline, once as any load and once in plain air, and:
//   · any load: the volumetric pass's programs are the real march's (and no plain text is made)
//   · plain air: the pass has ONE program per set, made from the plain text; no module of the real march and none of its twins' pipelines is asked for
//   · the plain module, AS CUT (gpu/wgslTrim.ts: what the browser is given), declares no function that walks the boxes, a light, a shadow or the fog of war's sight —
//     whatever it calls, however far down — and is a small text
//   · the numbers the plain march shares with the real one are still the real one's (they are copies: nothing else holds them together)
// No browser and no compiler: that the plain text COMPILES and DRAWS is the page's to say (tools/qa/plainair-page.cjs).
//   bun run tools/qa/plainair-check.ts             exit 0 = every check passed, 1 = one failed
//   TPS_COMPACTSHADERS=1 bun run tools/qa/plainair-check.ts     the page's writing of the march (its froxel twin is then one of the twins not asked for)
//   PLAINAIR_BREAK=1 bun run tools/qa/plainair-check.ts     the plain-air boot is made with the switch's word off: must FAIL (4 lines; 6 with the compact set)
export {};
const { ROOT, fake } = await import('./headless.ts');
const { readFileSync } = await import('node:fs');
const { Engine } = await import(`${ROOT}/src/engine.ts`);
const { gpuStory, PLAIN_AIR } = await import(`${ROOT}/src/gpu/device.ts`) as typeof import('../../src/gpu/device');
const { wgslDecls } = await import(`${ROOT}/src/gpu/wgslTrim.ts`) as typeof import('../../src/gpu/wgslTrim');

const BREAK = process.env.PLAINAIR_BREAK === '1';
let red = 0; const check = (ok: boolean, what: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${what}`); if (!ok) red++; };

/** what one boot of the Engine asks the browser for: every module's label and text, every pipeline's label */
async function boot(): Promise<{ mods: { label: string; code: string }[]; pipes: string[] }> {
  const mods: { label: string; code: string }[] = [], pipes: string[] = [];
  const pipe = () => new Proxy({}, { get: (_t, k) => k === 'then' ? undefined : fake(String(k)) });
  const base = fake('device');
  const device = new Proxy(base, { get(t, k) {
    if (k === 'createShaderModule') return (d: { code: string; label?: string }) => { mods.push({ label: d.label ?? '', code: d.code }); return { getCompilationInfo: async () => ({ messages: [] }) }; };
    if (k === 'createComputePipelineAsync' || k === 'createRenderPipelineAsync') return async (d: { label?: string }) => { pipes.push(d?.label ?? ''); return pipe(); };
    if (k === 'limits') return new Proxy({}, { get: () => 1 << 20 });
    if (k === 'then') return undefined;
    return t[k]; } });
  const e = new Engine({ device, adapterInfo: 'stand-in', canvas: fake('canvas'), ctx: fake('ctx'), format: 'bgra8unorm', hasTimestamps: false, linearSampler: fake('sampler') } as never);
  await Promise.race([e.init({ sharp: true, gbaa: true }), new Promise(r => setTimeout(r, 2000))]);
  return { mods, pipes };
}
const REAL = ['volumetrics', 'volumetricsF', 'volumetricsDust', 'volumetricsK0'], TWINS = ['volFroxel', 'volDust', 'volK0'];
const fnName = (d: string): string | null => /\bfn\s+([A-Za-z_]\w*)\s*\(/.exec(d)?.[1] ?? null;

gpuStory.plainAir = '';
const any = await boot();
check(any.mods.some(m => m.label === 'volumetrics') && !any.mods.some(m => m.label === 'volumetricsPlain') && any.pipes.includes('vol'), `any load: the pass's program is the real march's and no plain text is made (${any.mods.filter(m => /^volumetrics/.test(m.label)).map(m => m.label).join(', ')})`);

gpuStory.plainAir = BREAK ? '' : 'the check asked';
const plain = await boot();
gpuStory.plainAir = '';
const pm = plain.mods.filter(m => m.label === 'volumetricsPlain'), real = plain.mods.filter(m => REAL.includes(m.label)).map(m => m.label);
check(pm.length === 1 && plain.pipes.filter(l => l === 'vol').length === 1, `plain air: the pass has one program, made from the plain text (${pm.length} module${pm.length === 1 ? '' : 's'} named volumetricsPlain, ${plain.pipes.filter(l => l === 'vol').length} pipeline${plain.pipes.filter(l => l === 'vol').length === 1 ? '' : 's'} labelled vol)`);
check(real.length === 0, `… no text of the real march is handed over (${real.join(', ') || 'none'})`);
check(!plain.pipes.some(l => TWINS.includes(l)), `… and none of its twins is asked for (${plain.pipes.filter(l => TWINS.includes(l)).join(', ') || 'none'})`);
check(plain.pipes.length === any.pipes.length - any.pipes.filter(l => TWINS.includes(l)).length, `every other program is asked for as on any load (${plain.pipes.length} pipelines where any load asks for ${any.pipes.length}, ${any.pipes.filter(l => TWINS.includes(l)).length} of them the twins)`);
// the plain module as the browser is given it: every function it declares, however it came to be there
const code = pm[0]?.code ?? '', fns = wgslDecls(code).map(fnName).filter((n): n is string => !!n);
const WALKS = /^(occluded\w*|isect(?!Bounds$)\w*|grid(Cell|Item)\w*|trace\w*|evalLight\w*|fogSeen\w*|fogEye\w*|meshShadow\w*|smokeTransmittance\w*|smokeSelfShadow\w*|smokeTrans(In|Coarse)|smokeSegment\w*|diceMeanRadianceWalled|rainAlong|rainLight|lightFogK|charSdf\w*|sdRoundCone)$/;
const walks = fns.filter(n => WALKS.test(n));
check(pm.length === 1 && fns.includes('main') && walks.length === 0, `the plain module declares no function that walks the boxes, a light, a shadow or the fog's sight (${walks.join(', ') || 'none'}; ${fns.length} functions in all)`);
const kb = code.replace(/\/\/.*$/gm, '').length / 1024, realKb = (any.mods.find(m => m.label === 'volumetrics')?.code ?? '').replace(/\/\/.*$/gm, '').length / 1024;
check(pm.length === 1 && kb < 40 && kb < realKb / 3, `and is a small text: ${kb.toFixed(0)} KB where the real march's module is ${realKb.toFixed(0)} KB (comments out, each function once)`);
// the copies: the plain march's numbers are the real one's
{ const src = (f: string) => readFileSync(`${ROOT}/src/render/shaders/${f}`, 'utf8'), P = src('vol_plain.wgsl'), R = [src('volumetrics.wgsl'), src('compact/volumetrics.wgsl')];
  const body = (t: string, fn: string) => new RegExp(`fn ${fn}\\(\\) -> u32 \\{([^}]*)\\}`).exec(t)?.[1]?.trim() ?? null;
  const num = (t: string, c: string) => new RegExp(`const ${c}: f32 = ([0-9.]+);`).exec(t)?.[1] ?? null;
  check(R.every(t => num(t, 'WX_STEP') !== null && num(t, 'WX_STEP') === num(P, 'PLAIN_WX_STEP')) && R.every(t => body(t, 'wxMaxSteps') !== null && body(t, 'wxMaxSteps') === body(P, 'plainWxSteps')), `the weather's air is stepped as the real march steps it (WX_STEP ${num(P, 'PLAIN_WX_STEP')}, the tier's step budget)`);
  const SAME = ['clamp(u32(span / (vox * 1.5)) + 1u, 4u, 112u)', 'let sigT = dens * 6.0;', 'if (T < 0.01) { break; }', 'let top = CEIL_Y + WX_TOP;'];
  const lost = SAME.filter(s => !P.includes(s) || R.some(t => !t.includes(s)));
  check(lost.length === 0 && R.every(t => t.includes('let sigS = sigT * 0.92;')) && P.includes('* 0.92 *'), `the smoke is marched over the same stretch, at the same steps, with the same extinction, albedo and cut as the real K = 0 march (${lost.join(' · ') || 'every shared statement found in all three texts'})`);
  check(PLAIN_AIR.labels.join() === ['vol', ...TWINS].join(), `the labels a lost device is read against are the pass's four (${PLAIN_AIR.labels.join(', ')})`); }
console.log(red ? `${red} FAILED` : 'all passed');
process.exit(red ? 1 : 0);
