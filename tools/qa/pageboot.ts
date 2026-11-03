// THE PAGE'S BOOT OVER A RECORDING STAND-IN DEVICE: every shader text the page hands the browser, and every pipeline it asks for, with no browser. One boot for
// every tool that reads the page's shader texts (shaderinline-check sizes them, shaderfold-check reads the fold off them, wgsl-dump writes them out for a
// browser's compiler), so the three cannot come to boot different pages.
// The boot is main.ts's own order: the Engine's pipelines, the smoke solver's, the Game stood on that Engine, the Game's own GPU modules (mesh shadows,
// particles, the light probe, the skinned raster), then the cut-box second set (render/cutSwap.ts build). In front of it, what main.ts writes before it builds
// the Engine on a plain address: cut boxes on, round things on the GPU with them, the glass fracture, and the dust's own look (the march's dust twin).
// THE SETTINGS are the switches a page is opened under, each a different set of texts since the fold of constant switches (gpu/wgslTrim.ts SHADER_FOLD):
//   title   the page as it opens
//   snow    a snow level in its weather: every switch a LEVEL raises (the weather kinds, lying snow and its look rows)
//   look    the look prototype's page (`?look`), in a tree that has one
// ONE BOOT A PROCESS: the switches are module state and so is the cut set's book, so a tool that wants two settings runs itself once for each (`--setting`).
// The stand is the caller's: TPS_PROFILE is read as the game's modules load, so a tool that wants the page's own texts sets it before it calls pageBoot.
import { ROOT, fake, loadCharacter } from './headless.ts';
export type Stage = 'compute' | 'vertex' | 'fragment';
export interface Mod { label: string; code: string }
export interface Asked { label: string; set: 'boot' | 'cut'; stages: { stage: Stage; mod: number; entry: string | undefined }[] }
export interface Booted { setting: string; mods: Mod[]; asked: Asked[]; /** a step the stand-in never answered, or the cut set not built: the recording is short */ faults: string[]; cutNote: string }
export const SETTINGS: Record<string, string> = { title: 'the title', snow: 'a snow level in its weather', look: "the look prototype's page" };
/** the settings this tree has (the look's only where render/frame.ts has its switch) */
export async function settingsHere(): Promise<string[]> { const FR = await import(`${ROOT}/src/render/frame.ts`) as Record<string, unknown>; return FR.lookConfig ? ['title', 'snow', 'look'] : ['title', 'snow']; }
/** `--setting <name>` of a tool's own arguments (null: none given); an unknown name stops the tool */
export function settingArg(argv: string[]): string | null {
  const i = argv.indexOf('--setting'); if (i < 0) return null;
  const v = argv[i + 1]; if (!v || !(v in SETTINGS)) { console.error(`--setting needs one of: ${Object.keys(SETTINGS).join(' | ')}`); process.exit(2); }
  return v;
}
let booted = false;
export async function pageBoot(setting: string): Promise<Booted> {
  if (booted) throw new Error('pageBoot: one boot a process (the head of tools/qa/pageboot.ts)'); booted = true;
  if (!(setting in SETTINGS)) throw new Error(`pageBoot: no setting '${setting}'`);
  const { Engine } = await import(`${ROOT}/src/engine.ts`);
  const { SmokeSystem } = await import(`${ROOT}/src/smoke/smoke.ts`);
  const { standGame } = await import(`${ROOT}/src/host/standup.ts`);
  const { cutSwap } = await import(`${ROOT}/src/render/cutSwap.ts`);
  const { cutConfig } = await import(`${ROOT}/src/scene/cut.ts`);
  const { roundConfig, ROUND } = await import(`${ROOT}/src/scene/round.ts`);
  const { glassFxBoot } = await import(`${ROOT}/src/game/glass.ts`);
  const FR = await import(`${ROOT}/src/render/frame.ts`) as Record<string, any>;
  const { DUST_DRAW } = await import(`${ROOT}/src/smoke/dustTint.ts`);
  cutConfig.on = true; roundConfig.gpu = ROUND.on && cutConfig.on; if (glassFxBoot()) FR.glassBreakConfig.on = true; DUST_DRAW.on = true;
  if (setting === 'snow') { FR.weatherKindConfig.on = true; FR.snowConfig.on = true; for (const k of Object.keys(FR.snowLookConfig)) FR.snowLookConfig[k] = true; }
  if (setting === 'look') { if (!FR.lookConfig) throw new Error('pageBoot: this tree has no look page'); for (const c of [FR.lookConfig, FR.wallSidesConfig, FR.smokeLabConfig]) if (c) c.on = true; }
  const mods: Mod[] = [], asked: Asked[] = [], faults: string[] = []; let set: 'boot' | 'cut' = 'boot', cutNote = '';
  const ofMod = new WeakMap<object, number>();
  const pipe = () => new Proxy({}, { get: (_t, k) => k === 'then' ? undefined : fake(String(k)) });
  const ask = (d: Record<Stage, { module: object; entryPoint?: string } | undefined> & { label?: string }) => {
    const stages: Asked['stages'] = [];
    for (const stage of ['compute', 'vertex', 'fragment'] as const) { const s = d[stage]; if (s) stages.push({ stage, mod: ofMod.get(s.module) ?? -1, entry: s.entryPoint }); }
    asked.push({ label: d.label ?? '', set, stages }); return pipe();
  };
  const base = fake('device');
  const device = new Proxy(base, { get(t, k) {
    if (k === 'createShaderModule') return (d: { code: string; label?: string }) => { const m = { getCompilationInfo: async () => ({ messages: [] }) }; ofMod.set(m, mods.length); mods.push({ label: d.label ?? '', code: d.code }); return m; };
    if (k === 'createComputePipelineAsync' || k === 'createRenderPipelineAsync') return async (d: never) => ask(d);
    if (k === 'createComputePipeline' || k === 'createRenderPipeline') return (d: never) => ask(d);
    if (k === 'limits') return new Proxy({}, { get: () => 1 << 20 });
    if (k === 'then') return undefined;
    return t[k]; } });
  // (a step the stand-in never answers would leave the list short and a check green on a part of the page: a fault instead)
  const within = async <T,>(what: string, ms: number, p: Promise<T>): Promise<T | undefined> => { const late = Symbol(); const v = await Promise.race([p, new Promise<symbol>(r => setTimeout(() => r(late), ms))]); if (v === late) { faults.push(`the stand-in's boot did not finish its step '${what}'`); return undefined; } return v as T; };
  const log = console.log, warn = console.warn; console.log = console.warn = () => { /* the boot's own lines (a module left out on this device, the cut set's ready line) are not the tool's */ };
  try {
    const ok = await within('boot', setting === 'look' ? 180000 : 20000, (async () => {   // (the look page's init also makes its material maps: seconds here, more on a slow machine)
      const engine = new Engine({ device, adapterInfo: 'stand-in', canvas: fake('canvas'), ctx: fake('ctx'), format: 'bgra8unorm', hasTimestamps: false, linearSampler: fake('sampler') } as never);
      const engineInit = engine.init({ sharp: true, gbaa: true });
      const smoke = new SmokeSystem(engine); engine.setSmoke(smoke.binding);
      const smokeInit = smoke.init();
      const { game } = await standGame(engine, { ch: await loadCharacter(), smoke, seed: 1 });
      await Promise.all([engineInit, smokeInit, game.meshShadows.init(), engine.particles?.init(), game.probe.init?.(), engine.gbuffer.prepareSkin(game.chars.jointsFormat)]);
      return true;
    })());
    set = 'cut';
    const cut = ok ? await within('cut set', 20000, cutSwap.build()) : undefined;
    cutNote = `${cutSwap.pipelines} pipelines of ${cutSwap.built} owners${cutSwap.error ? ` — ${cutSwap.error}` : ''}`;
    if (cut !== undefined && cut !== true) faults.push(`the cut-box second set was not built (${cutNote})`);
  } finally { console.log = log; console.warn = warn; }
  return { setting, mods, asked, faults, cutNote };
}
