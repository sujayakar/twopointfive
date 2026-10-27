// THE SHADER ROWS ON TRIAL (src/ui/shaderrows.ts; Options > Graphics > Shaders): "Compact shaders" (render/kernels.ts: the ten kernels in their second writing are compiled
// instead of today's) and "Start on first frame shaders" (gpu/device.ts PIPE_QUEUE.later: the start waits only for the pipelines the first frame draws). The Engine is
// built on a stand-in device that records every module's text and every pipeline asked for and HOLDS each async creation until the check lets it go, in four arms, each a
// process of its own (a row is read once, when its module is imported):
//   off      no row: nothing of the compact set reaches the device, the start waits for every pipeline it asks for, every pass draws with its own program at once
//   compact  TPS_COMPACTSHADERS=1: nothing of today's ten kernels reaches the device, each compact one does, the volumetric pass has its two programs and its K = 0 twin
//   first    TPS_FIRSTFRAME=1: the start resolves with exactly the LATER set still out, every pass of every frame drawn meanwhile has a program, and each takes its own
//            program once that is in
//   both     the two rows together (the compact set's froxel twin is one more LATER pipeline)
// and then across the arms: the same pipelines by name in all four, but for the compact set's one more ('volFroxel').
// Also here, with no device: the two writings of each kernel declare the same entry points and the same bindings, and each compact kernel is LEVEL with today's (the
// fixture tools/qa/fixtures/compact-kernels.json holds what today's ten kernels were when their twins were last brought level with them: a kernel edited since is named,
// so that the row cannot quietly draw the picture as it was before the edit).
// No browser and no compiler: that the compact texts COMPILE, and what either row draws, is the page's to say (docs/agents/render.md, "Compact shaders").
//   bun run tools/qa/shaderrows-check.ts                     exit 0 = every check passed, 1 = one failed
//   SHADERROWS_BREAK=1 bun run tools/qa/shaderrows-check.ts  each arm is handed another arm's rows (off gets both, the others none): every arm must FAIL
//   bun run tools/qa/shaderrows-check.ts --dump <file>       this tree's boot under the environment's rows, a line a module (its length and a digest of its text) and
//                                                            a line a pipeline, in the order made: for a diff against another checkout's
//   bun run tools/qa/shaderrows-check.ts --stamp             after bringing the compact kernels level with today's: writes the fixture
export {};
import { readFileSync, writeFileSync, mkdtempSync, readdirSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');   // (the harness itself, tools/qa/headless.ts, is imported by an arm only: the store cases must set their page up before any module of the game is read)

const ARMS = { off: {}, compact: { TPS_COMPACTSHADERS: '1' }, first: { TPS_FIRSTFRAME: '1' }, both: { TPS_COMPACTSHADERS: '1', TPS_FIRSTFRAME: '1' } } as const;
type Arm = keyof typeof ARMS;
const WORDS = ['TPS_COMPACTSHADERS', 'TPS_FIRSTFRAME', 'TPS_CUTWHENNEEDED', 'TPS_SHADERSWHENNEEDED', 'TPS_SPLITRAYS'];   // (the last is the row 'Other shaders when needed', which holds the LATER set back: its own arms are tools/qa/shaderswhenneeded-check.ts)
const argOf = (k: string) => { const i = process.argv.indexOf(k); return i < 0 ? null : process.argv[i + 1] ?? ''; };
let red = 0; const check = (ok: boolean, what: string, more = '') => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${what}${!ok && more ? ` (${more})` : ''}`); if (!ok) red++; };
const tick = () => new Promise<void>(r => setTimeout(r, 0));
const sha = (s: string) => new Bun.CryptoHasher('sha256').update(s).digest('hex').slice(0, 16);
/** a text as the compiler reads it: comments out, spacing out */
const squeeze = (s: string) => s.replace(/\/\*[\s\S]*?\*\//g, '').replace(/\/\/.*$/gm, '').replace(/\s+/g, '');
const KERNEL_FILE = { fgather: 'fgather', gatherCommon: 'gather_common', reflectP5: 'reflect_p5', volumetrics: 'volumetrics', composite: 'composite', particles: 'particles', direct: 'direct', sharpDirect: 'sharp_direct', restir: 'restir', sdfRig: 'sdfrig' } as const;
const FIXTURE = `${ROOT}/tools/qa/fixtures/compact-kernels.json`;

// ---- the stand-in device: every module's text, every pipeline asked for (held), every program a pass is given
interface Asked { label: string; kind: 'compute' | 'render'; how: 'async' | 'sync'; mods: string[]; entry: string; done: boolean; let: () => void }
function standIn(fake: (name?: string) => any) {
  const mods: { label: string; code: string }[] = [], asked: Asked[] = [], drawn: { pass: string; pipe: string | null }[] = [], wrote: unknown[] = [];
  const modLabel = new WeakMap<object, string>();
  const pipe = (label: string) => fake(`pipe:${label}`);
  const ask = (kind: 'compute' | 'render', how: 'async' | 'sync') => (d: { label?: string; compute?: { module: object; entryPoint?: string }; vertex?: { module: object; entryPoint?: string }; fragment?: { module: object; entryPoint?: string } }) => {
    const stages = kind === 'compute' ? [d.compute] : [d.vertex, d.fragment];
    const a: Asked = { label: d.label ?? '?', kind, how, mods: [...new Set(stages.map(s => (s && modLabel.get(s.module)) ?? '?'))], entry: stages.map(s => s?.entryPoint ?? '').join('+'), done: how === 'sync', let: () => { /* set below */ } };
    asked.push(a);
    if (how === 'sync') return pipe(a.label);
    return new Promise(res => { a.let = () => { if (!a.done) { a.done = true; res(pipe(a.label)); } }; });
  };
  const pass = (label: string) => new Proxy({}, { get: (_t, k) => k === 'then' ? undefined : k === 'setPipeline' ? (p: unknown) => { drawn.push({ pass: label, pipe: p ? String((p as { label: string }).label).replace(/^pipe:/, '') : null }); } : () => undefined });
  const base = fake('device');
  const device = new Proxy(base, { get(t, k) {
    if (k === 'createShaderModule') return (d: { code: string; label?: string }) => { const m = { getCompilationInfo: async () => ({ messages: [] }) }; mods.push({ label: d.label ?? '', code: d.code }); modLabel.set(m, d.label ?? ''); return m; };
    if (k === 'createComputePipelineAsync') return ask('compute', 'async');
    if (k === 'createRenderPipelineAsync') return ask('render', 'async');
    if (k === 'createComputePipeline') return ask('compute', 'sync');
    if (k === 'createRenderPipeline') return ask('render', 'sync');
    if (k === 'createCommandEncoder') return () => new Proxy({}, { get: (_t, m) => m === 'then' ? undefined : m === 'beginComputePass' || m === 'beginRenderPass' ? (d: { label?: string }) => pass(d?.label ?? '?') : m === 'finish' ? () => fake('commands') : () => undefined });
    if (k === 'limits') return new Proxy({}, { get: () => 1 << 20 });
    if (k === 'queue') return new Proxy(t.queue, { get: (q, m) => m === 'then' ? undefined : m === 'writeBuffer' ? (_b: unknown, _o: number, data: unknown) => { wrote.push(data); } : q[m] });   // (which uniforms a frame sent, by the array handed over)
    if (k === 'then') return undefined;
    return t[k]; } });
  return { device, mods, asked, drawn, wrote };
}

// ---- one arm: this process's rows are the environment's
async function arm(name: Arm, dump: string | null) {
  const { fake } = await import('./headless.ts');
  (globalThis as Record<string, unknown>).window ??= globalThis; (globalThis as Record<string, unknown>).devicePixelRatio ??= 1;
  const { SHADER_ROWS } = await import(`${ROOT}/src/ui/shaderrows.ts`) as typeof import('../../src/ui/shaderrows');
  const { Engine } = await import(`${ROOT}/src/engine.ts`);
  const { FollowCamera } = await import(`${ROOT}/src/game/camera.ts`);
  const { FrameFlags } = await import(`${ROOT}/src/render/frame.ts`);
  const { PIPE_QUEUE, pipelineProgress } = await import(`${ROOT}/src/gpu/device.ts`) as typeof import('../../src/gpu/device');
  const { COMPACT, KERNELS, kernelSet } = await import(`${ROOT}/src/render/kernels.ts`) as typeof import('../../src/render/kernels');
  const { Particles } = await import(`${ROOT}/src/render/particles.ts`);
  const { dustLookVol } = await import(`${ROOT}/src/render/dustLookVol.ts`);
  const { wgslDecls } = await import(`${ROOT}/src/gpu/wgslTrim.ts`) as typeof import('../../src/gpu/wgslTrim');
  const { DUST_DRAW } = await import(`${ROOT}/src/smoke/dustTint.ts`);
  const wantCompact = name === 'compact' || name === 'both', wantFirst = name === 'first' || name === 'both';
  console.log(`\n=== ${name}: ${wantCompact ? 'Compact shaders on' : 'today\'s kernels'}, ${wantFirst ? 'Start on first frame shaders on' : 'the start waits for every pipeline'} ===`);
  check(SHADER_ROWS.compact === wantCompact && COMPACT.on === wantCompact && SHADER_ROWS.firstFrame === wantFirst && PIPE_QUEUE.later === wantFirst, `the rows this arm asked for are the rows the modules read`, `compact ${COMPACT.on}, later ${PIPE_QUEUE.later}`);

  const S = standIn(fake); pipelineProgress.reset();
  const gpu = { device: S.device, adapterInfo: 'stand-in', hasTimestamps: false, format: 'bgra8unorm', linearSampler: fake('sampler'), canvas: { clientWidth: 800, clientHeight: 500, width: 800, height: 500 }, ctx: fake('ctx') };
  const engine = new Engine(gpu as never);
  const particles = new Particles(S.device as never, engine.sceneLayout);   // (the page makes it beside the engine and waits for both: src/main.ts)
  let ready = false; const init = Promise.all([engine.init({ sharp: true, gbaa: true }), particles.init()]).then(() => { ready = true; });
  // THE LATER SET, by name: what the start may leave out with the row on (render/passes.ts traceJobs and init, render/froxels.ts). The dust twin only where it is drawn.
  const LATER = ['direct', 'restirM', 'fgather', 'fgatherGi', 'fgatherGiS', 'reflectP5', 'composite', 'compositeR', 'compositeS', 'froxelCull', 'froxelInject', 'froxelInjectJ', 'froxelTemporal', 'froxelIntegrate',
    ...(DUST_DRAW.on ? ['volDust'] : []), ...(wantCompact ? ['volFroxel'] : [])];
  const later = new Set(wantFirst ? LATER : []);
  for (let turn = 0; turn < 400 && !ready; turn++) { for (const a of S.asked) if (!a.done && !later.has(a.label)) a.let(); await tick(); }
  const out = () => S.asked.filter(a => !a.done).map(a => a.label).sort();
  check(ready, wantFirst ? 'the start resolves with the LATER set held back' : 'the start resolves once every pipeline it asked for is in', `still waiting on ${out().filter(l => !later.has(l)).join(', ') || 'a pipeline in the LATER set: ' + out().join(', ')}`);
  if (!ready) { await Promise.race([init, tick()]); return finish(); }
  if (wantFirst) {
    check(JSON.stringify(out()) === JSON.stringify([...LATER].sort()), `exactly the LATER set is still out when the start resolves: ${LATER.length} pipelines`, `out: ${out().join(', ')}`);
    check(pipelineProgress.total === pipelineProgress.done, 'the start card counts none of them', `${pipelineProgress.done} of ${pipelineProgress.total}`);
  } else check(out().length === 0, 'nothing is still out when the start resolves', out().join(', '));

  // ---- the texts: which writing of the ten kernels reached the device
  // (by whole declaration, comments and spacing aside: the cut text a module is compiled from keeps each declaration it keeps as it was written — gpu/wgslTrim.ts)
  { const A = kernelSet(false) as Record<string, string>, C = kernelSet(true) as Record<string, string>; const texts = S.mods.map(m => squeeze(m.code));
    const declsOf = (set: Record<string, string>) => Object.fromEntries(Object.keys(KERNEL_FILE).map(k => [k, wgslDecls(set[k]).map(squeeze).filter(d => d.length > 60)]));
    const dA = declsOf(A), dC = declsOf(C), allA = new Set(Object.values(dA).flat()), allC = new Set(Object.values(dC).flat());
    const ours = new Set(Object.values(KERNEL_FILE).map(f => `${f}.wgsl`)), elsewhere = new Set<string>();   // (a declaration another shader file has too says nothing about which writing this is)
    const walk = (dir: string) => { for (const e of readdirSync(dir, { withFileTypes: true })) { if (e.isDirectory()) walk(`${dir}/${e.name}`); else if (e.name.endsWith('.wgsl') && !ours.has(e.name)) for (const d of wgslDecls(readFileSync(`${dir}/${e.name}`, 'utf8'))) elsewhere.add(squeeze(d)); } };
    walk(`${ROOT}/src`);
    const stray: string[] = [], absent: string[] = [];
    for (const k of Object.keys(KERNEL_FILE)) {
      const onlyA = dA[k].filter(d => !allC.has(d) && !elsewhere.has(d)), onlyC = dC[k].filter(d => !allA.has(d) && !elsewhere.has(d)), mine = wantCompact ? onlyC : onlyA, other = wantCompact ? onlyA : onlyC;
      if (other.some(d => texts.some(t => t.includes(d)))) stray.push(k);
      if (!mine.some(d => texts.some(t => t.includes(d)))) absent.push(k);
    }
    check(stray.length === 0, wantCompact ? 'no module holds a declaration that only TODAY\'s writing of a kernel has' : 'no module holds a declaration that only the COMPACT writing of a kernel has', stray.join(', '));
    check(absent.length === 0, `each of the ten kernels reached the device in ${wantCompact ? 'its compact writing' : 'today\'s writing'}`, absent.join(', '));
    check(Object.keys(KERNEL_FILE).every(k => (KERNELS as Record<string, string>)[k] === (wantCompact ? C : A)[k]), 'KERNELS is that set whole'); }
  const mod = (label: string) => S.mods.filter(m => m.label === label);
  const has = (label: string) => S.asked.some(a => a.label === label);
  if (wantCompact) {
    const v = mod('volumetrics'), f = mod('volumetricsF');
    check(v.length === 1 && v[0].code.includes('const VOL_FROXEL_MARCH: bool = false;') && !v[0].code.includes('const VOL_FROXEL_MARCH: bool = true;'), 'the volumetric pass\'s boot program is compiled without the froxel-lit march');
    check(f.length === 1 && f[0].code.includes('const VOL_FROXEL_MARCH: bool = true;') && has('volFroxel'), 'its froxel twin (volFroxel) carries that march');
    check([...v, ...f].every(m => m.code.includes('const VOL_K0: bool = false;')), 'neither carries the K = 0 smoke march');
    check(!DUST_DRAW.on || dustLookVol((KERNELS as Record<string, string>).volumetrics) !== null, 'the dust twin still finds the statements it swaps in the compact march');
  } else {
    check(!has('volFroxel') && !has('volK0') && mod('volumetricsF').length === 0 && mod('volumetrics').length === 1 && !mod('volumetrics')[0].code.includes('VOL_FROXEL_MARCH'), 'the volumetric pass is one program, as it was (no volFroxel, no volK0)');
  }

  // ---- frames: which program each pass is given
  const cam = new FollowCamera(); cam.target = [20, 0, 14]; cam.aspect = 1.6; cam.rebuild(); cam.update(1 / 60, [20, 0, 14], null);
  (engine.settings as unknown as Record<string, unknown>).volLightCap = 4;   // (as every preset has it: RenderSettings' own 0 is the K = 0 smoke march, which no tier runs and which has its own lines below)
  const base = JSON.stringify(engine.settings);
  const restirU = (engine.passes as unknown as { restirUData: ArrayBuffer }).restirUData;   // (the restir pass's uniform as the engine last packed it; cfg.y is word 1)
  const frame = (set: (s: Record<string, unknown>) => void) => {
    Object.assign(engine.settings, JSON.parse(base)); set(engine.settings as unknown as Record<string, unknown>); S.drawn.length = 0; S.wrote.length = 0;
    try { engine.render(cam, 1, 1 / 60, false); engine.render(cam, 1 + 1 / 60, 1 / 60, false); } catch (e) { return { err: String((e as Error)?.message ?? e).split('\n')[0], by: {} as Record<string, string[]>, none: [] as string[], flags: 0, restirSent: false, cfgY: 0 }; }
    const by: Record<string, string[]> = {}; const none: string[] = [];
    for (const d of S.drawn) { if (d.pipe === null) none.push(d.pass); else (by[d.pass] ??= []).push(d.pipe); }
    return { err: '', by, none, flags: new Uint32Array(engine.frameData)[62], restirSent: S.wrote.includes(restirU), cfgY: new Uint32Array(restirU)[1] };
  };
  const VIEWS: { name: string; set: (s: Record<string, unknown>) => void; pass: string; own: string; meanwhile: string }[] = [
    { name: 'the old direct estimator (restir off)', set: s => { s.restir = false; }, pass: 'direct', own: 'direct', meanwhile: 'restir' },
    { name: 'the plain gather without strata', set: s => { s.gatherStrata = false; }, pass: 'gather', own: 'fgather', meanwhile: 'fgatherS' },
    { name: 'phase 5\'s reflections', set: s => { s.rtReflect = true; s.rtHitReuse = true; }, pass: 'reflect', own: 'reflectP5', meanwhile: 'reflect' },
    { name: 'the fragment composite (scale 1, nothing to decouple)', set: s => { s.renderScale = 1; s.sharpDirect = false; s.glassRefract = false; }, pass: 'composite', own: 'composite', meanwhile: '' },
  ];
  const shot = (when: string, landed: boolean) => {
    const plain = frame(() => undefined);
    check(!plain.err && plain.none.length === 0, `${when}: a frame at the settings as they start gives every pass a program`, plain.err || `none in ${plain.none.join(', ')}`);
    for (const v of VIEWS) {
      const r = frame(v.set), got = r.by[v.pass] ?? [];
      if (r.err || r.none.length) { check(false, `${when}: ${v.name} draws`, r.err || `no program in ${r.none.join(', ')}`); continue; }
      if (landed) check(got.includes(v.own), `${when}: ${v.name} draws with its own program (${v.own})`, `${v.pass}: ${got.join(', ') || 'not drawn'}`);
      else if (v.meanwhile) check(got.includes(v.meanwhile) && !got.includes(v.own), `${when}: ${v.name} draws with ${v.meanwhile} meanwhile`, `${v.pass}: ${got.join(', ') || 'not drawn'}`);
      else check(got.length === 0 && (r.by.compS ?? []).length > 0 && engine.passes.sharpOn === true, `${when}: ${v.name} is stood in for by sharp direct's compute composite`, `composite: ${got.join(', ') || '—'}, compS: ${(r.by.compS ?? []).join(', ') || '—'}`);
    }
    const fr = frame(s => { s.froxelVol = true; s.volumetrics = true; });
    const up = (fr.flags & FrameFlags.FroxelVol) !== 0;
    check(!fr.err && up === landed, `${when}: the froxel flag is ${landed ? 'up for a tier that marches the froxels' : 'held down'}`, fr.err || `flags 0x${fr.flags.toString(16)}`);
    if (wantCompact && landed) { const got = fr.by.volumetrics ?? []; check(got.includes('volFroxel'), `${when}: that tier's 2D pass draws with the froxel twin`, got.join(', ')); }
    // what the engine WRITES for a stand-in must be for the program that runs (the flags and the restir uniform go up before the encode picks)
    const off = frame(s => { s.restir = false; });
    if (landed) check(!off.err && (off.flags & FrameFlags.Restir) === 0 && !off.restirSent, `${when}: with restir off the frame says the old estimator (the flag down, no restir uniform sent)`, off.err || `flags 0x${off.flags.toString(16)}, uniform sent ${off.restirSent}`);
    else check(!off.err && (off.flags & FrameFlags.Restir) !== 0 && off.restirSent, `${when}: with restir off and ReSTIR standing in, the frame says ReSTIR (the flag up, its uniform sent)`, off.err || `flags 0x${off.flags.toString(16)}, uniform sent ${off.restirSent}`);
    const mv = frame(s => { s.restir = true; s.restirMotion = 7; s.restirMotionStatic = true; s.restirMotionTaps = 6; }), mvGot = mv.by.direct ?? [], taps = (mv.cfgY >>> 8) & 15;
    if (landed) check(!mv.err && mvGot.includes('restirM') && taps === 6, `${when}: a frame the camera moved runs the motion twin with its taps packed beside the spatial count`, mv.err || `direct: ${mvGot.join(', ')}, cfg.y 0x${mv.cfgY.toString(16)}`);
    else check(!mv.err && mvGot.includes('restir') && !mvGot.includes('restirM') && mv.cfgY < 256, `${when}: a frame the camera moved runs main's restir with main's uniform (no motion taps in the word it reads as its spatial count)`, mv.err || `direct: ${mvGot.join(', ')}, cfg.y 0x${mv.cfgY.toString(16)}`);
  };
  if (wantFirst) {
    check(engine.passes.fragCompReady === false && engine.passes.froxels.ready === false && (!wantCompact || engine.passes.volFReady === false), 'the engine knows what is not in yet (no fragment composite, no froxel kernels)');
    shot('while the LATER set compiles', false);
    for (let turn = 0; turn < 50 && S.asked.some(a => !a.done); turn++) { for (const a of S.asked) a.let(); await tick(); }
    check(engine.passes.fragCompReady && engine.passes.froxels.ready && engine.passes.volFReady, 'every LATER pipeline arrives: the fragment composites, the froxel kernels and the volumetric twin are in');
    shot('once it is in', true);
  } else {
    check(engine.passes.fragCompReady && engine.passes.froxels.ready && engine.passes.volFReady, 'the fragment composites, the froxel kernels and the volumetric pass are in when the start resolves');
    shot('at the start', true);
  }
  if (wantCompact) {   // the K = 0 smoke march: only in its own twin, built the first frame it is asked for
    const before = mod('volumetricsK0').length; check(before === 0 && !has('volK0'), 'no frame with a light cap asked for the K = 0 twin');
    const r = frame(s => { s.volLightCap = 0; s.froxelVol = false; });
    const k = mod('volumetricsK0');
    check(!r.err && k.length === 1 && k[0].code.includes('const VOL_K0: bool = true;') && has('volK0'), 'a frame with no light cap asks for the K = 0 twin, once', r.err || `${before} → ${k.length} modules`);
    check((r.by.volumetrics ?? []).length > 0 && r.none.length === 0, 'and the pass draws with the boot\'s program until it is in');
    for (const a of S.asked) a.let(); await tick(); await tick();
    const r2 = frame(s => { s.volLightCap = 0; s.froxelVol = false; });
    check((r2.by.volumetrics ?? []).includes('volK0'), 'then with the twin', (r2.by.volumetrics ?? []).join(', '));
  } else {
    const r = frame(s => { s.volLightCap = 0; });
    check(!r.err && !has('volK0') && mod('volumetricsK0').length === 0, 'a frame with no light cap asks for no twin (today\'s kernel carries that march itself)', r.err);
  }
  return finish();

  function finish() {
    if (dump) writeFileSync(dump, [...S.mods.map(m => `module ${m.label} ${m.code.length} ${sha(m.code)}`), ...S.asked.map(a => `pipeline ${a.label} ${a.kind} ${a.how} ${a.mods.join('+')} ${a.entry}`)].join('\n') + '\n');
    return red;
  }
}

// ---- the rows' own module: what a load reads (the address, then the kept line), what Options lists and writes. A process a case: the module reads once, at import
const STORE = {
  kept: { kept: '{"compact":true}', search: '' },
  word: { kept: '{"compact":true,"firstFrame":true}', search: '?compactshaders=0&firstframe=0' },
  visit: { kept: null as string | null, search: '?firstframe=1' },
  // … and in a BROWSER (the harness is none: its own name is not a browser's), where Compact shaders and Separate shadow rays start on
  starts: { kept: null as string | null, search: '', agent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36' },
  offkept: { kept: '{"compact":true,"splitRays":false}', search: '', agent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36' },
  plain: { kept: null as string | null, search: '?compactshaders=0&splitrays=0', agent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36' },
} as Record<string, { kept: string | null; search: string; agent?: string }>;
async function store(name: keyof typeof STORE) {
  const c = STORE[name], LS = 'twopointsix.shaderrows', box = new Map<string, string>(); if (c.kept !== null) box.set(LS, c.kept);
  let writes = 0;
  (globalThis as Record<string, unknown>).localStorage = { getItem: (k: string) => box.get(k) ?? null, setItem: (k: string, v: string) => { writes++; box.set(k, v); }, removeItem: (k: string) => { writes++; box.delete(k); } };
  (globalThis as Record<string, unknown>).location = { search: c.search };
  if (c.agent) Object.defineProperty(globalThis, 'navigator', { value: { userAgent: c.agent }, configurable: true });
  const { SHADER_ROWS, useShaderRow, shaderRows } = await import(`${ROOT}/src/ui/shaderrows.ts`) as typeof import('../../src/ui/shaderrows');
  const screen = () => { const secs: string[] = [], rows: { label: string; get: () => boolean; set: (v: boolean) => void; describe: string }[] = [];
    const reset = shaderRows({ sec: (_p, t) => secs.push(t), flip: (_p, label, get, set, describe) => { rows.push({ label, get, set, describe }); } }, {} as HTMLElement); return { secs, rows, reset }; };
  if (name === 'kept') {
    check(SHADER_ROWS.compact === true && SHADER_ROWS.firstFrame === false && writes === 0, 'a kept row is this load\'s with no word on the address, and reading it writes nothing', JSON.stringify(SHADER_ROWS));
    check(screen().secs.length === 0 && screen().rows.length === 0, 'a build that carries no reader of a row lists no row and no section');
    useShaderRow('compact'); const S1 = screen();
    check(S1.secs.join() === 'Shaders' && S1.rows.map(r => r.label).join() === 'Compact shaders' && S1.rows[0].get() === true, 'a row is listed once its reader is in the build, under Shaders, showing the kept choice', S1.rows.map(r => r.label).join());
    check(S1.rows.every(r => /^[A-Z][^0-9]*$/.test(r.label) && r.label.split(' ').length >= 2 && r.label.split(' ').length <= 5 && !r.label.includes('-') && r.describe.endsWith('This starts the next time you load the page.')), 'its label is a few plain words with no digit, and its line ends by saying when it starts');
  } else if (name === 'word') {
    check(SHADER_ROWS.compact === false && SHADER_ROWS.firstFrame === false && writes === 0 && box.get(LS) === c.kept, 'the address\'s word wins over the kept row for the visit and the kept line is left as it was', `${JSON.stringify(SHADER_ROWS)}, kept ${box.get(LS)}`);
    useShaderRow('compact'); useShaderRow('firstFrame'); const S1 = screen();
    check(S1.rows.length === 2 && S1.rows.every(r => r.get() === false), 'Options shows the address\'s word while nobody has moved the row');
    S1.rows[0].set(false);
    check(box.get(LS) === '{"firstFrame":true}' && S1.rows[0].get() === false && SHADER_ROWS.firstFrame === false, 'a row moved to where it starts is dropped from the kept line, the other kept row stays (and this load is not changed by it)', String(box.get(LS)));
    S1.reset();
    check(!box.has(LS) && S1.rows.every(r => r.get() === false), 'Reset to defaults puts every row back where it starts (off, in the harness) and takes the kept line away');
  } else if (name === 'starts') {
    const on = (Object.keys(SHADER_ROWS) as (keyof typeof SHADER_ROWS)[]).filter(k => SHADER_ROWS[k]).sort().join();
    check(on === 'compact,splitRays' && writes === 0 && !box.has(LS), 'in a browser with nothing kept and no word, Compact shaders and Separate shadow rays are this load\'s and no other row is, and nothing is written', on || 'none');
    for (const k of ['compact', 'splitRays', 'firstFrame'] as const) useShaderRow(k); const S1 = screen(), by = (l: string) => S1.rows.find(r => r.label === l)!;
    check(by('Compact shaders').get() === true && by('Separate shadow rays').get() === true && by('Start on first frame shaders').get() === false, 'Options shows the two on and the others off');
    by('Compact shaders').set(false);
    check(box.get(LS) === '{"compact":false}' && by('Compact shaders').get() === false && SHADER_ROWS.compact === true, 'a row that starts on, turned off, is kept off (and this load is not changed by it)', String(box.get(LS)));
    by('Compact shaders').set(true);
    check(!box.has(LS) && by('Compact shaders').get() === true, 'put back on it is where it starts, and nothing is kept');
    by('Separate shadow rays').set(false); by('Start on first frame shaders').set(true); S1.reset();
    check(!box.has(LS) && by('Compact shaders').get() === true && by('Separate shadow rays').get() === true && by('Start on first frame shaders').get() === false, 'Reset to defaults puts every row back where it starts and takes the kept line away');
  } else if (name === 'offkept') {
    check(SHADER_ROWS.splitRays === false && SHADER_ROWS.compact === true && writes === 0 && box.get(LS) === c.kept, 'a row kept off stays off on the next load, the other still starts on, and reading writes nothing', JSON.stringify(SHADER_ROWS));
    for (const k of ['compact', 'splitRays', 'firstFrame'] as const) useShaderRow(k); const S1 = screen(), by = (l: string) => S1.rows.find(r => r.label === l)!;
    by('Start on first frame shaders').set(true);
    check(box.get(LS) === '{"firstFrame":true,"splitRays":false}' && by('Compact shaders').get() === true && by('Separate shadow rays').get() === false, 'the next row moved writes only the rows that are not where they start: a line from before the two started on loses its Compact shaders entry', String(box.get(LS)));
  } else if (name === 'plain') {
    check(Object.values(SHADER_ROWS).every(v => v === false) && writes === 0 && !box.has(LS), 'the two words at 0 on the address are a load as it was: every row off, nothing written', JSON.stringify(SHADER_ROWS));
    useShaderRow('compact'); const S1 = screen();
    check(S1.rows[0].get() === false && /address is setting this/.test(S1.rows[0].describe), 'Options shows the address\'s word and says the address is setting it');
  } else {
    check(SHADER_ROWS.firstFrame === true && SHADER_ROWS.compact === false && SHADER_ROWS.splitRays === false && SHADER_ROWS.cutWhenNeeded === false && !box.has(LS) && writes === 0, 'a word on the address raises its row for the visit and is never written, and in the harness (no browser) no row starts on', JSON.stringify(SHADER_ROWS));
    useShaderRow('firstFrame'); const S1 = screen();
    check(S1.rows.length === 1 && S1.rows[0].get() === true, 'Options shows it on');
    S1.rows[0].set(true);
    check(box.get(LS) === '{"firstFrame":true}' && S1.rows[0].get() === true, 'turning a row on keeps it', String(box.get(LS)));
    S1.rows[0].set(false);
    check(!box.has(LS) && S1.rows[0].get() === false, 'with every row off again nothing is kept');
  }
  return red;
}

// ---- the two writings side by side (no device)
function twins() {
  console.log('\n=== the two writings of each kernel ===');
  const rd = (p: string) => readFileSync(`${ROOT}/src/render/shaders/${p}.wgsl`, 'utf8');
  const face = (s: string) => { const t = s.replace(/\/\*[\s\S]*?\*\//g, '').replace(/\/\/.*$/gm, ''); return [...t.matchAll(/@(compute|vertex|fragment)\b[^{]*?\bfn\s+(\w+)/g)].map(m => `${m[1]} ${m[2]}`).concat([...t.matchAll(/@group\(\s*(\d+)\s*\)\s*@binding\(\s*(\d+)\s*\)\s*var(?:<[^>]*>)?\s+(\w+)\s*:\s*([^;]+);/g)].map(m => `${m[1]}.${m[2]} ${m[3]}: ${m[4].replace(/\s+/g, '')}`)).sort(); };
  const differ: string[] = [];
  for (const f of Object.values(KERNEL_FILE)) if (JSON.stringify(face(rd(f))) !== JSON.stringify(face(rd(`compact/${f}`)))) differ.push(f);
  check(differ.length === 0, 'each kernel\'s two writings declare the same entry points and the same bindings', differ.join(', '));
  const now = Object.fromEntries(Object.values(KERNEL_FILE).map(f => [f, sha(rd(f))]));
  if (process.argv.includes('--stamp')) { writeFileSync(FIXTURE, JSON.stringify(now, null, 2) + '\n'); console.log(`- wrote ${FIXTURE}`); return; }
  let was: Record<string, string> = {}; try { was = JSON.parse(readFileSync(FIXTURE, 'utf8')); } catch { /* none yet */ }
  const moved = Object.keys(now).filter(f => now[f] !== was[f]);
  check(moved.length === 0, 'each compact kernel is level with today\'s (no kernel edited since its twin was last brought level)',
    `${moved.map(f => `${f}.wgsl`).join(', ')}: make the same change in src/render/shaders/compact/, then --stamp`);
}

if (argOf('--store')) process.exit(await store(argOf('--store') as keyof typeof STORE) ? 1 : 0);
if (argOf('--arm')) { const a = argOf('--arm') as Arm; if (!(a in ARMS)) { console.error(`unknown arm ${a}`); process.exit(2); } process.exit(await arm(a, argOf('--dump')) ? 1 : 0); }
if (argOf('--dump') !== null) {   // this tree, the environment's rows
  const e = process.env, a: Arm = e.TPS_COMPACTSHADERS === '1' ? (e.TPS_FIRSTFRAME === '1' ? 'both' : 'compact') : e.TPS_FIRSTFRAME === '1' ? 'first' : 'off';
  await arm(a, argOf('--dump')); process.exit(0);
}
twins();
if (!process.argv.includes('--stamp')) {
  const BREAK = process.env.SHADERROWS_BREAK === '1', dir = mkdtempSync(join(tmpdir(), 'shaderrows-'));
  const boots: Partial<Record<Arm, string[]>> = {};
  const child = (title: string, args: string[], env: Record<string, string | undefined>) => {
    const p = Bun.spawnSync(['bun', 'run', import.meta.path, ...args], { env, stdout: 'pipe', stderr: 'pipe' }), text = p.stdout.toString(); process.stdout.write(text);
    const fails = (text.match(/^- FAIL/gm) ?? []).length, passes = (text.match(/^- PASS/gm) ?? []).length;
    if (p.exitCode !== 0 && !fails) { console.log(`- FAIL — ${title} ran to its end (exit ${p.exitCode}: ${p.stderr.toString().trim().split('\n').slice(-2).join(' | ')})`); red++; }
    else if (!fails && !passes) { console.log(`- FAIL — ${title} said nothing`); red++; }
    red += fails;
  };
  console.log('\n=== the rows\' own module: the address, the kept line, Options ===');
  { const env: Record<string, string | undefined> = { ...process.env }; for (const w of WORDS) delete env[w]; delete env.SHADERROWS_BREAK; if (BREAK) env.TPS_COMPACTSHADERS = '0';   // (broken: the environment speaks over the kept line)
    for (const n of Object.keys(STORE)) child(`the ${n} case`, ['--store', n], env); }
  for (const a of Object.keys(ARMS) as Arm[]) {
    const env: Record<string, string | undefined> = { ...process.env }; for (const w of WORDS) delete env[w]; delete env.SHADERROWS_BREAK;
    Object.assign(env, BREAK ? (a === 'off' ? ARMS.both : ARMS.off) : ARMS[a]);
    const file = join(dir, `${a}.txt`); child(`the ${a} arm`, ['--arm', a, '--dump', file], env);
    try { boots[a] = readFileSync(file, 'utf8').split('\n').filter(l => l.startsWith('pipeline ')).map(l => l.split(' ')[1]).filter(l => l !== 'volK0').sort(); } catch { /* the arm did not get that far */ }
  }
  console.log('\n=== across the arms ===');
  const same = (x?: string[], y?: string[]) => !!x && !!y && JSON.stringify(x) === JSON.stringify(y);
  check(same(boots.off, boots.first), 'Start on first frame shaders asks for the same pipelines as the build without it, by name');
  check(!!boots.off && same([...boots.off, 'volFroxel'].sort(), boots.compact) && same(boots.compact, boots.both), 'Compact shaders asks for the same pipelines and one more (volFroxel), with or without the other row');
}
console.log(red ? `\nshaderrows-check: ${red} FAILED` : '\nshaderrows-check: all passed');
process.exit(red ? 1 : 0);
