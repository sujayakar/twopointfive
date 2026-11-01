// EVERY SHADER TEXT A BOOT MAKES, AS A FILE (the first half of tools/qa/wgsl-compile.cjs, which hands each to a browser's WGSL compiler). The Engine is built on
// a stand-in device that records every module's text (the smoke solver's too), once as a plain boot makes them and once as the look page does, and the distinct
// texts are written as JSON: [{ label, code }]. The shader rows (the compact writings, the split gather: ui/shaderrows.ts) are read ONCE at import, so one run
// dumps one setting of them: the compile tool runs this twice, bare and with every row's word in the environment (`label` = what to call that run's texts).
//   bun run tools/qa/wgsl-dump.ts <out.json> [label]
export {};
const { ROOT, fake } = await import('./headless.ts');
const { Engine } = await import(`${ROOT}/src/engine.ts`);
const { lookConfig, wallSidesConfig, smokeLabConfig } = await import(`${ROOT}/src/render/frame.ts`);
async function boot(): Promise<{ label: string; code: string }[]> {
  const mods: { label: string; code: string }[] = [];
  const pipe = () => new Proxy({}, { get: (_t, k) => k === 'then' ? undefined : fake(String(k)) });
  const base = fake('device');
  const device = new Proxy(base, { get(t, k) {
    if (k === 'createShaderModule') return (d: { code: string; label?: string }) => { mods.push({ label: d.label ?? '', code: d.code }); return { getCompilationInfo: async () => ({ messages: [] }) }; };
    if (k === 'createComputePipelineAsync' || k === 'createRenderPipelineAsync') return async () => pipe();
    if (k === 'limits') return new Proxy({}, { get: () => 1 << 20 });
    if (k === 'then') return undefined;
    return t[k]; } });
  const e = new Engine({ device, adapterInfo: 'stand-in', canvas: fake('canvas'), ctx: fake('ctx'), format: 'bgra8unorm', hasTimestamps: false, linearSampler: fake('sampler') } as never);
  const done = await Promise.race([e.init({ sharp: true, gbaa: true }).then(() => true), new Promise<boolean>(r => setTimeout(() => r(false), 180000))]);   // (init also makes the look page's material maps: seconds here, more on a slow machine)
  if (!done) { console.error('wgsl-dump: the Engine did not finish its init on the stand-in device in three minutes, so the dump would be short'); process.exit(2); }
  try { const { SmokeSystem } = await import(`${ROOT}/src/smoke/smoke.ts`); new SmokeSystem(e); } catch (err) { console.error('wgsl-dump: the smoke solver was not built on the stand-in device, its text is not in the dump:', String(err).slice(0, 200)); }   // (the solver's own module, smoke.wgsl: the Engine does not make it)
  return mods;
}
const all: { label: string; code: string }[] = [], seen = new Set<string>(), tag = process.argv[3] ? ` (${process.argv[3]})` : '';
// (… once more as the look page builds them: its prelude consts up, WALL_SIDES among them — the texts a plain boot never makes)
for (const look of [false, true]) { lookConfig.on = look; wallSidesConfig.on = look; smokeLabConfig.on = look;
  for (const m of await boot()) if (!seen.has(m.code)) { seen.add(m.code); all.push({ label: `${m.label}${tag}${look ? ' (look page)' : ''}`, code: m.code }); } }
lookConfig.on = false; wallSidesConfig.on = false; smokeLabConfig.on = false;
const out = process.argv[2]; if (!out) { console.error('usage: bun run tools/qa/wgsl-dump.ts <out.json> [label]'); process.exit(2); }
await Bun.write(out, JSON.stringify(all));
console.error(`wgsl-dump: ${all.length} distinct shader texts, ${all.reduce((n, m) => n + m.code.length, 0)} characters, to ${out}`);
process.exit(0);
