// OTHER SHADERS WHEN NEEDED (src/ui/shaderrows.ts, the row of that name in Options > Graphics > Shaders; gpu/device.ts PIPE_QUEUE.held): the pipelines the first frame
// does not draw (the LATER set of the row 'Start on first frame shaders') are compiled neither at the start nor behind it. Each is asked of the browser by the first frame
// whose settings want it (render/passes.ts need, render/froxels.ts need, the engine's calls), its pass drawing with the same stand-in until it is in.
// The Engine is built on a stand-in device that records every module and every pipeline asked for and HOLDS each async creation until the check lets it go, in four arms,
// each a process of its own (a row is read once, when its module is imported):
//   off     no row: nothing is held, the start asks for every pipeline, no frame ever asks for another
//   first   TPS_FIRSTFRAME=1 alone: the LATER set is asked for at the start and compiled behind it, as that row always did; nothing is held
//   needed  TPS_SHADERSWHENNEEDED=1 alone: the row runs on LATER's lane without that row; the start asks for NONE of the LATER set and makes none of its own modules;
//           frames at the settings as they start (a tier that does not march the froxels) ask for none of it; each program is asked for by the first frame that wants it,
//           once, with the pass drawing its stand-in meanwhile and Engine.standsIn up; once it lands the pass draws with it; what no frame asked for was never made.
//           The cut set built meanwhile leaves out the twins of what is still held, and a need compiles the pair: while the cut set is live the pass stands in until
//           the TWIN is in (the boot's program landing first is not drawn beside programs that carry the cut rule), and the froxels are not marched before theirs are.
//   both    the two rows together: as `needed` (the set is held, not compiled behind the start)
//   compact the row with 'Compact shaders' (TPS_COMPACTSHADERS=1): as `needed`, and the compact set's own later pipeline — the march's froxel twin, volFroxel — is held too
// HELD means not asked of the browser's compiler. Two module texts are still handed over at the start with the row on, because a waited pipeline shares their source
// (the refraction composite's compute twin, the froxel kernels): parsed, not compiled.
// No browser and no compiler: what the page makes is tools/qa/shader-count-page.cjs's to say (docs/agents/render.md, "Other shaders when needed").
//   bun run tools/qa/shaderswhenneeded-check.ts                               exit 0 = every check passed, 1 = one failed
//   SHADERSWHENNEEDED_BREAK=1 bun run tools/qa/shaderswhenneeded-check.ts     each arm is handed another arm's rows (off and first get the row; needed, both and compact lose it): every arm must FAIL
// The same on both stands (TPS_PROFILE=shipped in front): a switch of the page, off on each.
export {};
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');

const ARMS = { off: {}, first: { TPS_FIRSTFRAME: '1' }, needed: { TPS_SHADERSWHENNEEDED: '1' }, both: { TPS_FIRSTFRAME: '1', TPS_SHADERSWHENNEEDED: '1' }, compact: { TPS_COMPACTSHADERS: '1', TPS_SHADERSWHENNEEDED: '1' } } as const;
type Arm = keyof typeof ARMS;
const WORDS = ['TPS_COMPACTSHADERS', 'TPS_FIRSTFRAME', 'TPS_CUTWHENNEEDED', 'TPS_SHADERSWHENNEEDED'];
const argOf = (k: string) => { const i = process.argv.indexOf(k); return i < 0 ? null : process.argv[i + 1] ?? ''; };
let red = 0; const check = (ok: boolean, what: string, more = '') => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${what}${!ok && more ? ` (${more})` : ''}`); if (!ok) red++; };
const tick = () => new Promise<void>(r => setTimeout(r, 0));

// ---- the stand-in device (as tools/qa/shaderrows-check.ts's): every module, every pipeline asked for (held), every program a pass is given. A pipeline compiled from a
// module behind the cut prelude is named with '#cut' after its label, here and in what a pass is given
interface Asked { label: string; how: 'async' | 'sync'; done: boolean; let: () => void }
function standIn(fake: (name?: string) => any) {
  const mods: { label: string; code: string }[] = [], asked: Asked[] = [], drawn: { pass: string; pipe: string | null }[] = [];
  const cutMod = new WeakSet<object>();
  const pipe = (label: string) => fake(`pipe:${label}`);
  const ask = (kind: 'compute' | 'render', how: 'async' | 'sync') => (d: { label?: string; compute?: { module: object }; vertex?: { module: object }; fragment?: { module: object } }) => {
    const stages = kind === 'compute' ? [d.compute] : [d.vertex, d.fragment];
    const a: Asked = { label: (d.label ?? '?') + (stages.some(s => s && cutMod.has(s.module)) ? '#cut' : ''), how, done: how === 'sync', let: () => { /* set below */ } };
    asked.push(a);
    if (how === 'sync') return pipe(a.label);
    return new Promise(res => { a.let = () => { if (!a.done) { a.done = true; res(pipe(a.label)); } }; });
  };
  const pass = (label: string) => new Proxy({}, { get: (_t, k) => k === 'then' ? undefined : k === 'setPipeline' ? (p: unknown) => { drawn.push({ pass: label, pipe: p ? String((p as { label: string }).label).replace(/^pipe:/, '') : null }); } : () => undefined });
  const base = fake('device');
  const device = new Proxy(base, { get(t, k) {
    if (k === 'createShaderModule') return (d: { code: string; label?: string }) => { const m = { getCompilationInfo: async () => ({ messages: [] }) }; mods.push({ label: d.label ?? '', code: d.code }); if (d.code.includes('const CUT_BOXES: bool = true;')) cutMod.add(m); return m; };
    if (k === 'createComputePipelineAsync') return ask('compute', 'async');
    if (k === 'createRenderPipelineAsync') return ask('render', 'async');
    if (k === 'createComputePipeline') return ask('compute', 'sync');
    if (k === 'createRenderPipeline') return ask('render', 'sync');
    if (k === 'createCommandEncoder') return () => new Proxy({}, { get: (_t, m) => m === 'then' ? undefined : m === 'beginComputePass' || m === 'beginRenderPass' ? (d: { label?: string }) => pass(d?.label ?? '?') : m === 'finish' ? () => fake('commands') : () => undefined });
    if (k === 'limits') return new Proxy({}, { get: () => 1 << 20 });
    if (k === 'then') return undefined;
    return t[k]; } });
  return { device, mods, asked, drawn };
}

async function arm(name: Arm) {
  const { fake } = await import('./headless.ts');
  (globalThis as Record<string, unknown>).window ??= globalThis; (globalThis as Record<string, unknown>).devicePixelRatio ??= 1;
  const { SHADER_ROWS } = await import(`${ROOT}/src/ui/shaderrows.ts`) as typeof import('../../src/ui/shaderrows');
  const { Engine } = await import(`${ROOT}/src/engine.ts`);
  const { FollowCamera } = await import(`${ROOT}/src/game/camera.ts`);
  const { FrameFlags } = await import(`${ROOT}/src/render/frame.ts`);
  const { PIPE_QUEUE, pipelineProgress, laterProgress } = await import(`${ROOT}/src/gpu/device.ts`) as typeof import('../../src/gpu/device');
  const { DUST_DRAW } = await import(`${ROOT}/src/smoke/dustTint.ts`);
  const { COMPACT } = await import(`${ROOT}/src/render/kernels.ts`) as typeof import('../../src/render/kernels');
  const wantHeld = name === 'needed' || name === 'both' || name === 'compact', wantLater = name !== 'off';
  console.log(`\n=== ${name}: ${wantHeld ? `Other shaders when needed on${name === 'compact' ? ', with Compact shaders' : ''}` : name === 'first' ? 'Start on first frame shaders alone' : 'no row'} ===`);
  check(COMPACT.on === (name === 'compact'), `the compact kernels are ${name === 'compact' ? 'the ones' : 'not the ones'} this arm compiles`);
  check(SHADER_ROWS.whenNeeded === wantHeld && PIPE_QUEUE.held === wantHeld && PIPE_QUEUE.later === wantLater, 'the rows this arm asked for are the rows the modules read (the row raises the LATER lane by itself)', `held ${PIPE_QUEUE.held}, later ${PIPE_QUEUE.later}`);

  const S = standIn(fake); pipelineProgress.reset();
  const gpu = { device: S.device, adapterInfo: 'stand-in', hasTimestamps: false, format: 'bgra8unorm', linearSampler: fake('sampler'), canvas: { clientWidth: 800, clientHeight: 500, width: 800, height: 500 }, ctx: fake('ctx') };
  const engine = new Engine(gpu as never);
  const P = engine.passes as unknown as { asked: string[]; heldKeys: string[]; fragCompReady: boolean; sharpOn: boolean; froxels: { ready: boolean; asked: boolean }; buildCut(): Promise<void>; useCut(on: boolean): void; cutSet: { cut: Record<string, unknown> | null } };
  let ready = false; const init = engine.init({ sharp: true, gbaa: true }).then(() => { ready = true; });
  // THE HELD SET, by pipeline label and by the field need() is called with (render/passes.ts traceJobs and init, render/froxels.ts): the LATER set of the row 'Start on
  // first frame shaders', but for the compute composite — that row leaves the plain one (compositeS) for later and waits for its refraction twin; this one waits for
  // the plain one and holds the twin (compositeSR) until a sharp frame has a pane to bend through
  const FROX = ['froxelCull', 'froxelInject', 'froxelInjectJ', 'froxelTemporal', 'froxelIntegrate'];
  const FIELD: Record<string, string> = { direct: 'pDirect', restirM: 'pRestirM', fgather: 'pFG', fgatherGi: 'pFGR', fgatherGiS: 'pFGRs', reflectP5: 'pReflP5', composite: 'pComp', compositeR: 'pCompR', compositeSR: 'pCompSR', volDust: 'pVolDust', ...(COMPACT.on ? { volFroxel: 'pVolF' } : {}) };
  const LATER = [...Object.keys(FIELD).filter(l => l !== 'volDust' || DUST_DRAW.on), ...FROX];   // (the dust twin reaches the browser only where dust is drawn; what would make it is held either way)
  const labels = () => S.asked.map(a => a.label);
  const outNow = () => S.asked.filter(a => !a.done).map(a => a.label).sort();
  const landBoot = async () => { for (let turn = 0; turn < 20; turn++) { for (const a of S.asked) if (!a.label.endsWith('#cut')) a.let(); await tick(); } };   // (the boot's programs only: every cut twin stays out)
  const land = async () => { for (let turn = 0; turn < 50 && S.asked.some(a => !a.done); turn++) { for (const a of S.asked) a.let(); await tick(); } await tick(); };
  const LATER_FIRST = LATER.map(l => l === 'compositeSR' ? 'compositeS' : l);   // (what that row alone leaves out of the start)
  const hold = new Set(name === 'first' ? LATER_FIRST : []);
  for (let turn = 0; turn < 400 && !ready; turn++) { for (const a of S.asked) if (!a.done && !hold.has(a.label)) a.let(); await tick(); }
  check(ready, 'the start resolves', `still waiting on ${outNow().join(', ')}`);
  if (!ready) { await Promise.race([init, tick()]); return red; }
  const atBoot = labels(), bootLater = LATER.filter(l => atBoot.includes(l));
  if (wantHeld) {
    check(bootLater.length === 0, `the start asked the browser for none of the ${LATER.length} pipelines the row holds`, `asked: ${bootLater.join(', ')}`);
    check(outNow().length === 0 && laterProgress.pending === 0 && engine.standsIn === false, 'nothing is out behind the start, and the engine says no pass stands in for a program that is compiling', `out: ${outNow().join(', ')}; pending ${laterProgress.pending}`);
    check(JSON.stringify([...P.heldKeys].sort()) === JSON.stringify(Object.values(FIELD).sort()) && P.froxels.asked === false && P.asked.length === 0, `each of them is held under its field's name: ${Object.keys(FIELD).length} of the passes' (the dust twin among them) and the five froxel kernels`, `held: ${P.heldKeys.join(', ')}`);
    const own = ['direct', 'fgather', 'reflectP5', 'composite', 'compositeR'].filter(l => S.mods.some(m => m.label === l));
    check(own.length === 0, 'no module that only a held pipeline is compiled from was made (direct, fgather, reflectP5, composite, compositeR)', own.join(', '));
  } else {
    check(bootLater.length === LATER.length, `the start asked the browser for every one of the ${LATER.length} pipelines the row would hold${name === 'first' ? ', its own LATER set to compile behind it' : ', and waited for them'}`, `not asked: ${LATER.filter(l => !bootLater.includes(l)).join(', ')}`);
    check(P.heldKeys.length === 0 && P.froxels.asked === false, 'nothing is held', P.heldKeys.join(', '));
    if (name === 'first') { check(JSON.stringify(outNow()) === JSON.stringify([...LATER_FIRST].sort()) && engine.standsIn === true, 'they are out behind the start, and the engine says a pass stands in meanwhile', outNow().join(', ')); await land(); }
    check(outNow().length === 0 && engine.standsIn === false, 'with all of them in, the engine says no pass stands in');
  }

  // ---- frames
  const cam = new FollowCamera(); cam.target = [20, 0, 14]; cam.aspect = 1.6; cam.rebuild(); cam.update(1 / 60, [20, 0, 14], null);
  // THE SETTINGS AS THEY START here are a tier's that does not march the froxels (high, ultra): sharp direct live over an upscaled picture, the light cap every preset has
  Object.assign(engine.settings, { volLightCap: 4, froxelVol: false, sharpDirect: true, taau: true, renderScale: 0.75, glassRefract: false });
  const base = JSON.stringify(engine.settings);
  const frame = (set: (s: Record<string, unknown>) => void) => {
    Object.assign(engine.settings, JSON.parse(base)); set(engine.settings as unknown as Record<string, unknown>); S.drawn.length = 0;
    try { engine.render(cam, 1, 1 / 60, false); engine.render(cam, 1 + 1 / 60, 1 / 60, false); } catch (e) { return { err: String((e as Error)?.message ?? e).split('\n')[0], by: {} as Record<string, string[]>, none: [] as string[], flags: 0 }; }
    const by: Record<string, string[]> = {}; const none: string[] = [];
    for (const d of S.drawn) { if (d.pipe === null) none.push(d.pass); else (by[d.pass] ??= []).push(d.pipe); }
    return { err: '', by, none, flags: new Uint32Array(engine.frameData)[62] };
  };
  const plain = () => undefined;
  // the settings as they start: what those frames ask for, and that they then ask for nothing more
  { const n0 = S.asked.length; const r = frame(plain); const first = labels().slice(n0);
    check(!r.err && r.none.length === 0, 'a frame at the settings as they start gives every pass a program', r.err || `none in ${r.none.join(', ')}`);
    if (wantHeld) check(first.length === 0, 'those frames ask the browser for nothing: a tier like that never compiles one of the held set while nothing changes', first.join(', '));
    else check(first.length === 0, 'those frames ask the browser for nothing', first.join(', '));
    const n1 = S.asked.length; for (let i = 0; i < 4; i++) frame(plain);
    check(S.asked.length === n1, 'four more of them, with nothing changing, ask for nothing more', labels().slice(n1).join(', '));
    await land();
    const n2 = S.asked.length; frame(plain); frame(plain);
    check(S.asked.length === n2 && engine.standsIn === false, 'nor once those have landed: the count stays where it is', labels().slice(n2).join(', ')); }
  const settled = new Set(labels());
  const VIEWS: { name: string; set: (s: Record<string, unknown>) => void; pass: string; own: string; meanwhile: string }[] = [
    { name: 'the old direct estimator (restir off, the motion pass asked for with it: ReSTIR only stands in, so its motion twin is not wanted)', set: s => { s.restir = false; s.restirMotion = 7; s.restirMotionStatic = true; }, pass: 'direct', own: 'direct', meanwhile: 'restir' },
    { name: 'a frame the camera moved (the motion twin)', set: s => { s.restir = true; s.restirMotion = 7; s.restirMotionStatic = true; }, pass: 'direct', own: 'restirM', meanwhile: 'restir' },
    { name: 'the plain gather without strata', set: s => { s.gatherStrata = false; }, pass: 'gather', own: 'fgather', meanwhile: 'fgatherS' },
    { name: 'phase 5\'s reflections', set: s => { s.rtReflect = true; s.rtHitReuse = true; }, pass: 'reflect', own: 'reflectP5', meanwhile: 'reflect' },
    { name: 'the fragment composite (sharp direct off)', set: s => { s.renderScale = 1; s.sharpDirect = false; s.glassRefract = false; }, pass: 'composite', own: 'composite', meanwhile: '' },
  ];
  for (const v of VIEWS) {
    if (!wantHeld) { const n = S.asked.length; const r = frame(v.set), got = r.by[v.pass] ?? [];
      check(!r.err && r.none.length === 0 && got.includes(v.own) && S.asked.length === n, `${v.name} draws with its own program (${v.own}) at once, and asks for nothing`, r.err || `${v.pass}: ${got.join(', ') || 'not drawn'}; asked ${labels().slice(n).join(', ')}`);
      continue; }
    if (settled.has(v.own)) { console.log(`- (the settings as they start had already asked for ${v.own}: ${v.name} is not a first need on this build)`); continue; }
    const n = S.asked.length; const r = frame(v.set), got = r.by[v.pass] ?? [], now = labels().slice(n);
    check(!r.err && r.none.length === 0 && now.length === 1 && now[0] === v.own, `${v.name}: the first frame that wants it asks the browser for ${v.own}, and for nothing else`, r.err || `asked: ${now.join(', ') || 'nothing'}; no program in ${r.none.join(', ') || '—'}`);
    if (v.meanwhile) check(got.includes(v.meanwhile) && !got.includes(v.own), `… and draws with ${v.meanwhile} meanwhile`, `${v.pass}: ${got.join(', ') || 'not drawn'}`);
    else check(got.length === 0 && (r.by.compS ?? []).length > 0 && P.sharpOn === true && !labels().slice(n).includes('compositeSR'), '… and is stood in for by sharp direct\'s compute composite meanwhile, which asks for no program of its own', `composite: ${got.join(', ') || '—'}, compS: ${(r.by.compS ?? []).join(', ') || '—'}`);
    check(engine.standsIn === true, '… while the engine says a pass stands in (the ladder\'s question)');
    const n1 = S.asked.length; frame(v.set); frame(v.set);
    check(S.asked.length === n1, '… and the frames after it do not ask again', labels().slice(n1).join(', '));
    await land();
    const r2 = frame(v.set), got2 = r2.by[v.pass] ?? [];
    check(!r2.err && got2.includes(v.own) && engine.standsIn === false, `… once it is in, the pass draws with ${v.own} and nothing stands in`, r2.err || `${v.pass}: ${got2.join(', ') || 'not drawn'}`);
  }
  // a tier that marches the froxels
  { const frox = (s: Record<string, unknown>) => { s.froxelVol = true; s.volumetrics = true; };
    const n0 = S.asked.length; if (wantHeld) { void (engine.passes.froxels as unknown as { buildCut(): Promise<void> }).buildCut(); await tick(); check(S.asked.length === n0, 'the cut set asked of the froxels while their kernels are held makes no twin', labels().slice(n0).join(', ')); }
    const n = S.asked.length; const r = frame(frox), now = labels().slice(n).sort(), up = (r.flags & FrameFlags.FroxelVol) !== 0;
    if (!wantHeld) check(!r.err && up && now.length === 0, 'a tier that marches the froxels has the flag up at once, and asks for nothing', r.err || `flags 0x${r.flags.toString(16)}; asked ${now.join(', ')}`);
    else {
      const want = [...FROX, ...(COMPACT.on ? ['volFroxel'] : [])].sort();
      check(!r.err && JSON.stringify(now) === JSON.stringify(want) && !up && P.froxels.ready === false, `a tier that marches the froxels: its first frame asks for the five kernels${COMPACT.on ? ' and the march\'s froxel twin' : ''}, the flag held down (the per-pixel march draws)`, r.err || `asked ${now.join(', ')}; flags 0x${r.flags.toString(16)}`);
      await landBoot();
      const twins = labels().slice(n).filter(l => !want.includes(l)).sort(), rMid = frame(frox);
      check(JSON.stringify(twins) === JSON.stringify(['froxelInjectCut#cut', 'froxelInjectJCut#cut']), '… with the five in, their two cut twins are asked for, since a cut set had been asked for', twins.join(', '));
      check(!rMid.err && (rMid.flags & FrameFlags.FroxelVol) === 0 && P.froxels.ready === false && engine.standsIn === true, '… and the froxels are not marched before those twins are in: the flag stays down and the engine says a pass stands in', rMid.err || `flags 0x${rMid.flags.toString(16)}, ready ${P.froxels.ready}`);
      await land();
      const r2 = frame(frox);
      check(!r2.err && (r2.flags & FrameFlags.FroxelVol) !== 0 && P.froxels.ready && engine.standsIn === false, '… and the flag is up once they are all in', r2.err || `flags 0x${r2.flags.toString(16)}`);
    } }
  if (wantHeld) {
    // ---- the cut set (render/cutSwap.ts), built while some are still held: their twins are left out of it, and a need compiles the pair
    const still = P.heldKeys.map(k => Object.keys(FIELD).find(l => FIELD[l] === k)!).sort();
    check(still.length > 0, `still held after all of that, never asked of the browser: ${still.join(', ')}`);
    const n = S.asked.length; const building = P.buildCut(); await tick(); await land(); await building;
    const twins = labels().slice(n);
    check(twins.length > 0 && twins.every(l => l.endsWith('#cut')) && !still.some(l => twins.includes(`${l}#cut`)), `the cut set built now (${twins.length} twins) has no twin of a pipeline still held`, twins.filter(l => still.includes(l.replace('#cut', ''))).join(', '));
    check(Object.keys(FIELD).filter(l => !still.includes(l)).every(l => twins.includes(`${l}#cut`)), '… and has the twin of each one a frame did ask for');
    const pick = still[0], field = FIELD[pick], at = () => String((engine.passes as unknown as Record<string, { label?: string } | null>)[field]?.label ?? 'nothing').replace(/^pipe:/, '');
    { const n1 = S.asked.length; engine.passes.need(field); engine.passes.need(field); const now = labels().slice(n1).sort();
      check(JSON.stringify(now) === JSON.stringify([pick, `${pick}#cut`]), `a need for ${pick} now asks for it and for its cut twin together, once`, now.join(', '));
      P.useCut(true); const live0 = at(); P.useCut(false);
      check(live0 === 'nothing' && at() === 'nothing', '… and until they land its place is empty in both sets (the pass stands in, with the stand-in\'s own twin while the cut set is live)', `${live0}, ${at()}`);
      await landBoot();
      P.useCut(true); const live1 = at(); P.useCut(false); const calm1 = at();
      check(live1 === 'nothing' && calm1 === pick && engine.standsIn === true, '… with the boot\'s program in and its twin not: the boot\'s set draws with it, the cut set still stands in (never the program without the cut rule), and the engine says so', `live: ${live1}; not: ${calm1}; standsIn ${engine.standsIn}`);
      await land();
      P.useCut(true); const live = at(); P.useCut(false); const calm = at();
      check(live === `${pick}#cut` && calm === pick, '… then the twin is the program while the cut set is live, and the boot\'s while it is not', `live: ${live}; not: ${calm}`); }
    const never = P.heldKeys.map(k => Object.keys(FIELD).find(l => FIELD[l] === k)!).filter(l => labels().some(x => x === l || x === `${l}#cut`));
    check(never.length === 0, `what no frame asked for was never made, with or without the cut rule: ${P.heldKeys.length} pipelines`, never.join(', '));
  } else check(P.asked.length === 0, 'no frame of all those asked for a held pipeline: there is none');
  return red;
}

if (argOf('--arm')) { const a = argOf('--arm') as Arm; if (!(a in ARMS)) { console.error(`unknown arm ${a}`); process.exit(2); } process.exit(await arm(a) ? 1 : 0); }
{ const BREAK = process.env.SHADERSWHENNEEDED_BREAK === '1';
  const BROKEN: Record<Arm, Record<string, string>> = { off: ARMS.needed, first: ARMS.both, needed: ARMS.off, both: ARMS.first, compact: { TPS_COMPACTSHADERS: '1' } };
  for (const a of Object.keys(ARMS) as Arm[]) {
    const env: Record<string, string | undefined> = { ...process.env }; for (const w of WORDS) delete env[w]; delete env.SHADERSWHENNEEDED_BREAK;
    Object.assign(env, BREAK ? BROKEN[a] : ARMS[a]);
    const p = Bun.spawnSync(['bun', 'run', import.meta.path, '--arm', a], { env, stdout: 'pipe', stderr: 'pipe' }), text = p.stdout.toString(); process.stdout.write(text);
    const fails = (text.match(/^- FAIL/gm) ?? []).length, passes = (text.match(/^- PASS/gm) ?? []).length;
    if (p.exitCode !== 0 && !fails) { console.log(`- FAIL — the ${a} arm ran to its end (exit ${p.exitCode}: ${p.stderr.toString().trim().split('\n').slice(-2).join(' | ')})`); red++; }
    else if (!fails && !passes) { console.log(`- FAIL — the ${a} arm said nothing`); red++; }
    red += fails;
  } }
console.log(red ? `\nshaderswhenneeded-check: ${red} FAILED` : '\nshaderswhenneeded-check: all passed');
process.exit(red ? 1 : 0);
