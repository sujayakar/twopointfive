// THE PAGE'S BOOT OVER A RECORDING STAND-IN DEVICE: every shader text the page hands the browser, and every pipeline it asks for, with no browser. One boot for
// every tool that reads the page's shader texts (shaders-check inline sizes them, shaders-check fold reads the fold off them, wgsl-dump writes them out for a
// browser's compiler), so the three cannot come to boot different pages.
// The boot is drawpage.ts's own order: the Engine's pipelines, the smoke solver's, the particles, the men's mesh and its tree, the programs asked for ahead of
// their first use (engine.ts warmFirstUse). No game is stood. In front of it, what the page writes before it builds the Engine: cut boxes on, the glass fracture, the dust's own look.
// THE SETTINGS are the switches a page is opened under, each a different set of texts since the fold of constant switches (gpu/wgslTrim.ts SHADER_FOLD):
//   title   the page as it opens (the look is the picture: its layers and the walls' two sides are in every setting)
//   snow    a snow level in its weather: every switch a LEVEL raises (the weather kinds, lying snow and its look rows)
//   nowalls   the page as it opens with the walls' row down (`?look=walls:0`): the WALL_SIDES=false texts, which no other setting compiles
// ONE BOOT A PROCESS: the switches are module state, so a tool that wants two settings runs itself once for each (`--setting`).
import { ROOT, characterSet, fake } from './here.ts';
export type Stage = 'compute' | 'vertex' | 'fragment';
export interface Mod { label: string; code: string }
export interface Asked { label: string; stages: { stage: Stage; mod: number; entry: string | undefined }[] }
export interface Booted { setting: string; mods: Mod[]; asked: Asked[]; /** a step the stand-in never answered: the recording is short */ faults: string[] }
export const SETTINGS: Record<string, string> = { title: 'the title', snow: 'a snow level in its weather', nowalls: 'the page with the walls\' row down' };
/** the settings this tree has */
export async function settingsHere(): Promise<string[]> { return Object.keys(SETTINGS); }
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
  const { Particles } = await import(`${ROOT}/src/render/particles.ts`), { CharacterRenderer } = await import(`${ROOT}/src/anim/characters.ts`), { MeshShadows } = await import(`${ROOT}/src/render/meshShadows.ts`);
  const { cutConfig } = await import(`${ROOT}/src/scene/cut.ts`);
    const FR = await import(`${ROOT}/src/render/frame.ts`) as Record<string, any>;
  const { DUST_DRAW } = await import(`${ROOT}/src/smoke/dustTint.ts`);
  cutConfig.on = true; FR.glassBreakConfig.on = true; DUST_DRAW.on = true;
  if (setting === 'snow') { FR.weatherKindConfig.on = true; FR.snowConfig.on = true; for (const k of Object.keys(FR.snowLookConfig)) FR.snowLookConfig[k] = true; }
  if (setting === 'nowalls') FR.wallSidesConfig.on = false;
  const mods: Mod[] = [], asked: Asked[] = [], faults: string[] = [];
  const ofMod = new WeakMap<object, number>();
  const pipe = () => new Proxy({}, { get: (_t, k) => k === 'then' ? undefined : fake(String(k)) });
  const ask = (d: Record<Stage, { module: object; entryPoint?: string } | undefined> & { label?: string }) => {
    const stages: Asked['stages'] = [];
    for (const stage of ['compute', 'vertex', 'fragment'] as const) { const s = d[stage]; if (s) stages.push({ stage, mod: ofMod.get(s.module) ?? -1, entry: s.entryPoint }); }
    asked.push({ label: d.label ?? '', stages }); return pipe();
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
  const log = console.log, warn = console.warn; console.log = console.warn = () => { /* the boot's own lines (a module left out on this device) are not the tool's */ };
  try {
    await within('boot', 180000, (async () => {   // (the engine's init also makes the look's material maps: seconds here, more on a slow machine)
      const engine = new Engine({ device, adapterInfo: 'stand-in', canvas: fake('canvas'), ctx: fake('ctx'), format: 'bgra8unorm', hasTimestamps: false, linearSampler: fake('sampler') } as never);
      const engineInit = engine.init({ sharp: true, gbaa: true });
      const smoke = new SmokeSystem(engine); engine.setSmoke(smoke.binding);
      const smokeInit = smoke.init();
      engine.setParticles(new Particles(engine.device, engine.sceneLayout));
      const ch = await characterSet(), chars = new CharacterRenderer(engine.device, ch), shadows = new MeshShadows(engine.device, ch, chars.jointMats, chars.charInsts);
      engine.skin = chars; engine.setMeshShadows(shadows);
      await Promise.all([engineInit, smokeInit, engine.particles.init(), engine.gbuffer.needLook(), shadows.init(), engine.gbuffer.prepareSkin(chars.jointsFormat)]);
      engine.warmFirstUse(); await new Promise(r => setTimeout(r, 0));   // (host/drawpage.ts, after the boot's freeze: the programs a pass used to make at its first use — the fog of war's — are asked for behind the start)
      return true;
    })());
  } finally { console.log = log; console.warn = warn; }
  return { setting, mods, asked, faults };
}
