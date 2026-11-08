// THE FIRST-FRAME START (src/ui/shaderrows.ts; `?firstframe`, no row on Options): "Start on first frame shaders" (gpu/device.ts PIPE_QUEUE.later: the start waits only
// for the pipelines the first frame draws). The Engine is built on a stand-in device that records every module's text and every pipeline asked for and HOLDS each
// async creation until the check lets it go, in two arms, each a process of its own (the row is read once, when its module is imported):
//   off      the row off: the start waits for every pipeline it asks for, every pass draws with its own program at once
//   first    TPS_FIRSTFRAME=1: the start resolves with exactly the LATER set still out, every pass of every frame drawn meanwhile has a program, and each takes its own
//            program once that is in
// and then across the arms: the same pipelines by name in both.
// In both: the volumetric pass is its one program, and the gather is its two programs.
// (Until 2026-11-07 two more rows stood here, Compact shaders and Separate shadow rays, with two more arms and a comparison of the kernels' two writings: the ten
// kernels have one writing now and the gather is the split one, so there is nothing left to hold level.)
// No browser and no compiler: that the texts COMPILE, and what the row draws, is the page's to say (tools/qa/wgsl-compile.cjs, docs/agents/render.md).
//   bun run tools/qa/shaderrows-check.ts                     exit 0 = every check passed, 1 = one failed
//   SHADERROWS_BREAK=1 bun run tools/qa/shaderrows-check.ts  each arm is handed the other arm's row, and the store cases an environment that says off: both arms and the `old` and `starts` cases must FAIL
//   bun run tools/qa/shaderrows-check.ts --dump <file>       this tree's boot under the environment's row, a line a module (its length and a digest of its text) and
//                                                            a line a pipeline, in the order made: for a diff against another checkout's
export {};
import { readFileSync, writeFileSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');   // (the harness itself, tools/qa/headless.ts, is imported by an arm only: the store cases must set their page up before any module of the game is read)

const ARMS = { off: {}, first: { TPS_FIRSTFRAME: '1' } } as const;
type Arm = keyof typeof ARMS;
const WORDS = ['TPS_FIRSTFRAME'];
const argOf = (k: string) => { const i = process.argv.indexOf(k); return i < 0 ? null : process.argv[i + 1] ?? ''; };
let red = 0; const check = (ok: boolean, what: string, more = '') => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${what}${!ok && more ? ` (${more})` : ''}`); if (!ok) red++; };
const tick = () => new Promise<void>(r => setTimeout(r, 0));
const sha = (s: string) => new Bun.CryptoHasher('sha256').update(s).digest('hex').slice(0, 16);

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
    if (k === 'pushErrorScope') return () => undefined;
    if (k === 'popErrorScope') return async () => null;   // (the gather's points buffer is made inside a scope: no error here)
    if (k === 'limits') return new Proxy({}, { get: () => 2 ** 30 });   // (room for the gather's points buffer: an adapter that refuses it draws no gather, render/passes.ts ensureSplitBG)
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
  (await import(`${ROOT}/src/gpu/wgslTrim.ts`) as typeof import('../../src/gpu/wgslTrim')).SHADER_FOLD.on = false;   // (which writing of a kernel reaches the device is read off declarations as written and off the switches' own lines: the fold of constant switches cuts both and changes neither answer; its own check is shaderfold-check.ts)
  const { Engine } = await import(`${ROOT}/src/engine.ts`);
  const { FollowCamera } = await import(`${ROOT}/src/game/camera.ts`);
  const { FrameFlags } = await import(`${ROOT}/src/render/frame.ts`);
  const { PIPE_QUEUE, pipelineProgress, laterIdle } = await import(`${ROOT}/src/gpu/device.ts`) as typeof import('../../src/gpu/device');
  const { Particles } = await import(`${ROOT}/src/render/particles.ts`);
  const wantFirst = name === 'first';
  console.log(`\n=== ${name}: ${wantFirst ? 'Start on first frame shaders on' : 'the start waits for every pipeline'} ===`);
  check(SHADER_ROWS.firstFrame === wantFirst && PIPE_QUEUE.later === wantFirst, `the row this arm asked for is the row the modules read`, `later ${PIPE_QUEUE.later}`);

  const S = standIn(fake); pipelineProgress.reset();
  const gpu = { device: S.device, adapterInfo: 'stand-in', hasTimestamps: false, format: 'bgra8unorm', linearSampler: fake('sampler'), canvas: { clientWidth: 800, clientHeight: 500, width: 800, height: 500 }, ctx: fake('ctx') };
  const engine = new Engine(gpu as never);
  const particles = new Particles(S.device as never, engine.sceneLayout);   // (the page makes it beside the engine and waits for both: src/main.ts)
  let ready = false; const init = Promise.all([engine.init({ sharp: true, gbaa: true }), particles.init()]).then(() => { ready = true; });
  // THE LATER SET, by name: what the start may leave out with the row on (render/passes.ts traceJobs).
  const LATER = ['composite'];
  const later = new Set(wantFirst ? LATER : []);
  for (let turn = 0; turn < 400 && !ready; turn++) { for (const a of S.asked) if (!a.done && !later.has(a.label)) a.let(); await tick(); }
  const out = () => S.asked.filter(a => !a.done).map(a => a.label).sort();
  check(ready, wantFirst ? 'the start resolves with the LATER set held back' : 'the start resolves once every pipeline it asked for is in', `still waiting on ${out().filter(l => !later.has(l)).join(', ') || 'a pipeline in the LATER set: ' + out().join(', ')}`);
  if (!ready) { await Promise.race([init, tick()]); return finish(); }
  if (wantFirst) {
    check(JSON.stringify(out()) === JSON.stringify([...LATER].sort()), `exactly the LATER set is still out when the start resolves: ${LATER.length} pipelines`, `out: ${out().join(', ')}`);
    check(pipelineProgress.total === pipelineProgress.done, 'the start card counts none of them', `${pipelineProgress.done} of ${pipelineProgress.total}`);
  } else { let idle = false; void laterIdle().then(() => { idle = true; }); await tick();
    check(out().length === 0 && idle, 'nothing is still out when the start resolves, and what a script\'s `__ready` waits on (laterIdle) resolves at once', out().join(', ')); }

  const mod = (label: string) => S.mods.filter(m => m.label === label);
  const has = (label: string) => S.asked.some(a => a.label === label);
  {
    const v = mod('volumetrics');
    check(v.length === 1 && mod('volumetricsF').length === 0 && mod('volumetricsK0').length === 0 && mod('froxel').length === 0 && has('vol') && !S.asked.some(a => /^vol(Froxel|Dust|K0)$|^froxel/.test(a.label)), 'the volumetric pass is ONE program for every tier: no froxel twin, no zero-light twin, no dust twin and no froxel kernel is asked for');
    check(v.every(m => !/VOL_FROXEL_MARCH|VOL_K0|FLAG_FROXEL_VOL|smokeMarchJ|froxel/.test(m.code)), 'its text has no froxel-lit march and no march without a light cap');
    check(v.every(m => m.code.includes('fn dustAlive() -> bool') && m.code.includes('let dl = dustDensity(X, onMask, true);')), 'it carries the dust\'s look behind the frame\'s bit');
  }

  // ---- frames: which program each pass is given
  const cam = new FollowCamera(); cam.target = [20, 0, 14]; cam.aspect = 1.6; cam.rebuild(); cam.update(1 / 60, [20, 0, 14], null);
  (engine.settings as unknown as Record<string, unknown>).volLightCap = 4;   // (as high has it: RenderSettings' own 0 is before any preset, and has its own line below)
  const base = JSON.stringify(engine.settings);
  const restirU = (engine.passes as unknown as { restirUData: ArrayBuffer }).restirUData;   // (the restir pass's uniform as the engine last packed it; cfg.y is word 1)
  const frame = (set: (s: Record<string, unknown>) => void, moved = false) => {
    Object.assign(engine.settings, JSON.parse(base)); set(engine.settings as unknown as Record<string, unknown>); S.drawn.length = 0; S.wrote.length = 0;
    try { engine.render(cam, 1, 1 / 60, false); if (moved) cam.update(1 / 60, [24, 0, 14], null); engine.render(cam, 1 + 1 / 60, 1 / 60, false); if (moved) cam.update(1, [20, 0, 14], null); } catch (e) { return { err: String((e as Error)?.message ?? e).split('\n')[0], by: {} as Record<string, string[]>, none: [] as string[], flags: 0, restirSent: false, cfgY: 0 }; }
    const by: Record<string, string[]> = {}; const none: string[] = [];
    for (const d of S.drawn) { if (d.pipe === null) none.push(d.pass); else (by[d.pass] ??= []).push(d.pipe); }
    return { err: '', by, none, flags: new Uint32Array(engine.frameData)[62], restirSent: S.wrote.includes(restirU), cfgY: new Uint32Array(restirU)[1] };
  };
  const VIEWS: { name: string; set: (s: Record<string, unknown>) => void; pass: string; own: string; meanwhile: string }[] = [
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
    { const fr = frame(s => { s.volumetrics = true; }), got = fr.by.volumetrics ?? []; check(!fr.err && got.length > 0 && got.every(l => l === 'vol'), `${when}: the volumetric pass draws with the one march program`, fr.err || got.join(', ')); }
    // the motion levers are bits of the one program's uniform: a frame the camera moved carries its taps beside the spatial count, whatever has landed
    const mv = frame(s => { s.restirMotion = 7; s.restirMotionTaps = 6; }, true), mvGot = mv.by.direct ?? [], taps = (mv.cfgY >>> 8) & 15;
    check(!mv.err && mvGot.length > 0 && mvGot.every(l => l === 'restir') && taps === 6, `${when}: a frame the camera moved runs the one direct program with its motion taps packed beside the spatial count`, mv.err || `direct: ${mvGot.join(', ')}, cfg.y 0x${mv.cfgY.toString(16)}`);
  };
  if (wantFirst) {
    check(engine.passes.fragCompReady === false, 'the engine knows what is not in yet (no fragment composite)');
    shot('while the LATER set compiles', false);
    // (what a script's `__ready` waits on, main.ts: the start card is live for the player, a tool's first picture is the settings' own)
    let idle = false; void laterIdle().then(() => { idle = true; }); await tick(); await tick();
    check(!idle && engine.standsIn === true, 'while it compiles the engine says a pass stands in, and what a script\'s `__ready` waits on (laterIdle) has not resolved');
    for (let turn = 0; turn < 50 && S.asked.some(a => !a.done); turn++) { for (const a of S.asked) a.let(); await tick(); }
    check(engine.passes.fragCompReady, 'every LATER pipeline arrives: the fragment composites are in');
    for (let turn = 0; turn < 10 && !idle; turn++) await tick();
    check(idle && engine.standsIn === false, 'with all of it in nothing stands in and laterIdle resolves');
    shot('once it is in', true);
  } else {
    check(engine.passes.fragCompReady, 'the fragment composites are in when the start resolves');
    shot('at the start', true);
  }
  {   // a settings table that says no light cap (RenderSettings' own 0, before a preset places one) still draws: the engine reads it as high's 4, and no march without a cap exists
    const r = frame(s => { s.volLightCap = 0; }), got = r.by.volumetrics ?? [];
    check(!r.err && r.none.length === 0 && got.length > 0 && got.every(l => l === 'vol') && new Uint32Array(engine.frameData)[136] === 4, 'a frame with no light cap said draws with the one march program, its cap sent as high\'s 4', r.err || `${got.join(', ')} · volCfg.x ${new Uint32Array(engine.frameData)[136]}`);
  }
  return finish();

  function finish() {
    if (dump) writeFileSync(dump, [...S.mods.map(m => `module ${m.label} ${m.code.length} ${sha(m.code)}`), ...S.asked.map(a => `pipeline ${a.label} ${a.kind} ${a.how} ${a.mods.join('+')} ${a.entry}`)].join('\n') + '\n');
    return red;
  }
}

// ---- the row's own module: what a load reads (the address, else where a browser starts). A process a case: the module reads once, at import
const MAC = 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36', WIN = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36';
const STORE = {
  // a line an older build kept (the row was on Options until 2026-11-08) means nothing now: read past, never written
  old: { kept: '{"firstFrame":false}', search: '', agent: MAC, want: true },
  word: { kept: null as string | null, search: '?firstframe=0', agent: MAC, want: false },
  visit: { kept: null as string | null, search: '?firstframe=1', want: true },
  // … and in a BROWSER (the harness is none: its own name is not a browser's), where it starts on for a person
  starts: { kept: null as string | null, search: '', agent: MAC, want: true },
  plain: { kept: null as string | null, search: '?compactshaders=0&splitrays=0&firstframe=0', agent: WIN, want: false },
  // … and in a browser a SCRIPT drives (every page tool's): it starts off there, so a tool's first picture is the settings' own
  driven: { kept: null as string | null, search: '', agent: 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/141.0.0.0 Safari/537.36', driven: true, want: false },
  // … and with no browser at all: off
  harness: { kept: null as string | null, search: '', want: false },
} as Record<string, { kept: string | null; search: string; agent?: string; driven?: boolean; want: boolean }>;
async function store(name: keyof typeof STORE) {
  const c = STORE[name], LS = 'twopointsix.shaderrows', box = new Map<string, string>(); if (c.kept !== null) box.set(LS, c.kept);
  let writes = 0;
  (globalThis as Record<string, unknown>).localStorage = { getItem: (k: string) => box.get(k) ?? null, setItem: (k: string, v: string) => { writes++; box.set(k, v); }, removeItem: (k: string) => { writes++; box.delete(k); } };
  (globalThis as Record<string, unknown>).location = { search: c.search };
  if (c.agent) Object.defineProperty(globalThis, 'navigator', { value: { userAgent: c.agent, webdriver: c.driven === true }, configurable: true });
  const { SHADER_ROWS, useShaderRow, shaderRowsOn, shaderRowLabel } = await import(`${ROOT}/src/ui/shaderrows.ts`) as typeof import('../../src/ui/shaderrows');
  check(SHADER_ROWS.firstFrame === c.want && writes === 0, `${name}: the load's first-frame start is ${c.want ? 'on' : 'off'}, and nothing is written`, JSON.stringify(SHADER_ROWS));
  if (name === 'visit') {
    check(shaderRowsOn().length === 0, 'a report names no row whose reader is not in the build');
    useShaderRow('firstFrame'); check(shaderRowsOn().join() === 'firstframe', 'with its reader in the build a report names it by its word');
    check(shaderRowLabel('compactshaders') === 'Compact shaders' && shaderRowLabel('splitrays') === 'Separate shadow rays' && shaderRowLabel('firstframe') === 'Start on first frame shaders' && shaderRowLabel('a-later-build') === 'a-later-build', 'a report from a build that still had the two retired rows reads by their names');
  }
  return red;
}

if (argOf('--store')) process.exit(await store(argOf('--store') as keyof typeof STORE) ? 1 : 0);
if (argOf('--arm')) { const a = argOf('--arm') as Arm; if (!(a in ARMS)) { console.error(`unknown arm ${a}`); process.exit(2); } process.exit(await arm(a, argOf('--dump')) ? 1 : 0); }
if (argOf('--dump') !== null) {   // this tree, the environment's row
  await arm(process.env.TPS_FIRSTFRAME === '1' ? 'first' : 'off', argOf('--dump')); process.exit(0);
}
{
  const BREAK = process.env.SHADERROWS_BREAK === '1', dir = mkdtempSync(join(tmpdir(), 'shaderrows-'));
  const boots: Partial<Record<Arm, string[]>> = {};
  const child = (title: string, args: string[], env: Record<string, string | undefined>) => {
    const p = Bun.spawnSync(['bun', 'run', import.meta.path, ...args], { env, stdout: 'pipe', stderr: 'pipe' }), text = p.stdout.toString(); process.stdout.write(text);
    const fails = (text.match(/^- FAIL/gm) ?? []).length, passes = (text.match(/^- PASS/gm) ?? []).length;
    if (p.exitCode !== 0 && !fails) { console.log(`- FAIL — ${title} ran to its end (exit ${p.exitCode}: ${p.stderr.toString().trim().split('\n').slice(-2).join(' | ')})`); red++; }
    else if (!fails && !passes) { console.log(`- FAIL — ${title} said nothing`); red++; }
    red += fails;
  };
  console.log('\n=== the row\'s own module: the address, where a browser starts ===');
  { const env: Record<string, string | undefined> = { ...process.env }; for (const w of WORDS) delete env[w]; delete env.SHADERROWS_BREAK; if (BREAK) env.TPS_FIRSTFRAME = '0';   // (broken: the environment speaks over where the row starts)
    for (const n of Object.keys(STORE)) child(`the ${n} case`, ['--store', n], env); }
  for (const a of Object.keys(ARMS) as Arm[]) {
    const env: Record<string, string | undefined> = { ...process.env }; for (const w of WORDS) delete env[w]; delete env.SHADERROWS_BREAK;
    Object.assign(env, BREAK ? (a === 'off' ? ARMS.first : ARMS.off) : ARMS[a]);
    const file = join(dir, `${a}.txt`); child(`the ${a} arm`, ['--arm', a, '--dump', file], env);
    try { boots[a] = readFileSync(file, 'utf8').split('\n').filter(l => l.startsWith('pipeline ')).map(l => l.split(' ')[1]).sort(); } catch { /* the arm did not get that far */ }
  }
  console.log('\n=== across the arms ===');
  const same = (x?: string[], y?: string[]) => !!x && !!y && JSON.stringify(x) === JSON.stringify(y);
  check(same(boots.off, boots.first), 'Start on first frame shaders asks for the same pipelines as the build without it, by name');
  check(!!boots.off && ['gatherTrace', 'gatherLight', 'vol'].every(l => boots.off!.includes(l)) && !boots.off.some(l => /^fgather|^vol(Froxel|Dust|K0)$|^froxel/.test(l)), 'the gather is its two programs and the volumetric pass its one, and no one-kernel gather, march twin or froxel kernel is asked for', (boots.off ?? []).filter(l => /gather|^vol/.test(l)).join(', '));
  // the page's wiring, read as text: a player's start does not wait for the later set, a script's `__ready` does
  { const main = readFileSync(`${ROOT}/src/main.ts`, 'utf8');
    check(/\n  window\.__started = true;\n  void laterIdle\(\)\.then\(\(\) => \{ [^\n]*readyResolve\(\); \}\);/.test(main) && !/\n\s*readyResolve\(\);/.test(main),
      'main.ts raises `__started` when the start card is live and resolves `__ready` only once the later set is in');
  }
}
console.log(red ? `\nshaderrows-check: ${red} FAILED` : '\nshaderrows-check: all passed');
process.exit(red ? 1 : 0);
