// EVERY LEVEL ASKS FOR ONE SET OF PROGRAMS (src/render/frame.ts LevelSw, src/render/meshShadows.ts; the kept page that makes it matter: src/gpu/keep.ts).
// A mission start on the kept page compiles only what the visit has not built, and a text that differs by one constant is a text the visit has not built.
// What a level raises of the shaders — the weather kinds' reads, the snow, the rows of the snow's look — was nine constants of the scene prelude, and the
// sizes of the men's mesh were four constants of the mesh shadows' module: the office, a level with its own roofs, a snow level and a level whose men are
// another cast each compiled a part of the set again (the title to a snow level: a third of the programs). They are data now: a word of the frame block
// and four words of a buffer. Held here, with no browser:
//   texts     the page's boot recorded under the title's switches and under a snow level's in its weather (tools/qa/pageboot.ts, each a process of its own),
//             with the shader rows as written and as the page opens: the same module texts, label for label, and the same pipelines
//   word      the switch still reaches the shaders: the frame block's level word is 0 on the title and every bit of LevelSw on the snow level, the
//             prelude names each bit once, and writeFrame puts the word where common.wgsl's `levelCfg` reads it
//   cast      the mesh shadows' module is one text for both casts (their trees differ in nodes, triangles and levels), and the level list's header
//             carries each cast's own four numbers
//   bun run tools/qa/levelset-check.ts                    exit 0 = every check passed, 1 = one failed
//   LEVELSET_BREAK=1 bun run tools/qa/levelset-check.ts   the snow level's boot flips one constant of the prelude and one cast's module carries a size: must FAIL
// What it does not say: that a level compiles nothing in a BROWSER (a page check: `JOBS=picture node tools/qa/keeppage-page.cjs` reads the kept page's own
// count of what each load compiled), or what the branches a level does not take cost a frame on a GPU (never timed).
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
export {};
const BREAK = process.env.LEVELSET_BREAK === '1';
const argv = process.argv.slice(2), childOf = argv.indexOf('--child');
const sha = (s: string): string => createHash('sha1').update(s).digest('hex').slice(0, 16);
interface Rec { mods: { label: string; h: string; n: number }[]; asked: string[]; word: number; bits: Record<string, number>; prelude: string; faults: string[] }

if (childOf >= 0) {
  // ONE BOOT (pageboot.ts allows one a process): its texts and pipelines as a line of JSON
  process.env.TPS_PROFILE = 'shipped';
  const setting = argv[childOf + 1];
  const { ROOT } = await import('./here.ts');
  const { pageBoot } = await import('./pageboot.ts');
  const FR = await import(`${ROOT}/src/render/frame.ts`) as Record<string, any>;
  if (BREAK && setting === 'snow') FR.rainScaleConfig.on = !FR.rainScaleConfig.on;   // (the red path: a level that still raises a constant of the text)
  const B = await pageBoot(setting);
  const rec: Rec = { mods: B.mods.map(m => ({ label: m.label, h: sha(m.code), n: m.code.length })), asked: B.asked.map(a => `${a.label} ${a.stages.map(s => `${s.stage}:${B.mods[s.mod]?.label ?? '?'}#${sha(B.mods[s.mod]?.code ?? '')}/${s.entry ?? ''}`).join(' ')}`),
    word: FR.levelSwitches(), bits: { ...FR.LevelSw }, prelude: sha(FR.scenePrelude()), faults: B.faults };
  console.log('LEVELSET ' + JSON.stringify(rec)); process.exit(0);
}

let red = 0; const check = (ok: boolean, what: string, more = ''): void => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${what}${more ? `\n    ${more}` : ''}`); if (!ok) red++; };
const ROWS = { TPS_COMPACTSHADERS: '1', TPS_SPLITRAYS: '1', TPS_CUTWHENNEEDED: '1', TPS_FIRSTFRAME: '1', TPS_SHADERSWHENNEEDED: '1' };   // (the rows as the page opens)
const boot = (setting: string, env: Record<string, string>): Rec | null => {
  const r = spawnSync('bun', ['run', import.meta.path, '--child', setting], { encoding: 'utf8', env: { ...process.env, ...env }, maxBuffer: 1 << 26 });
  const line = String(r.stdout).split("\n").find((l: string) => l.startsWith("LEVELSET "));
  if (r.status !== 0 || !line) { console.error(r.stdout.slice(-600), r.stderr.slice(-600)); return null; }
  return JSON.parse(line.slice(9)) as Rec;
};

// ---- texts, word
for (const [rows, env] of [['the shader rows as written', {}], ['the shader rows as the page opens', ROWS]] as const) {
  const T = boot('title', env), S = boot('snow', env);
  if (!T || !S) { check(false, `texts — the two boots ran (${rows})`); continue; }
  const faults = [...T.faults, ...S.faults];
  const other = T.mods.map((m, i) => S.mods[i] && S.mods[i].label === m.label && S.mods[i].h === m.h ? null : m.label).filter(Boolean), pipes = T.asked.filter((a, i) => S.asked[i] !== a);
  check(faults.length === 0 && T.mods.length > 30 && T.asked.length >= 98 && T.mods.length === S.mods.length && other.length === 0 && T.asked.length === S.asked.length && pipes.length === 0 && T.prelude === S.prelude,
    `texts — the title and a snow level in its weather are the same ${T.mods.length} module texts and the same ${T.asked.length} pipelines (${rows})`,
    faults.length ? faults.join(' · ') : other.length || pipes.length || T.mods.length !== S.mods.length ? `${other.length} of ${T.mods.length} modules differ (${other.slice(0, 8).join(', ')}) · ${pipes.length} of ${T.asked.length} pipelines differ · the snow level's boot: ${S.mods.length} modules, ${S.asked.length} pipelines` : `${Math.round(T.mods.reduce((n, m) => n + m.n, 0) / 1024)} KB of text either way`);
  const all = Object.values(S.bits).reduce((a, b) => a | b, 0), single = Object.values(S.bits).every(v => v > 0 && (v & (v - 1)) === 0) && new Set(Object.values(S.bits)).size === Object.keys(S.bits).length;
  check(T.word === 0 && S.word === all && single && Object.keys(S.bits).length === 9, `word — the frame's level word is 0 on the title and every switch's bit on the snow level (${rows})`, `title ${T.word} · snow ${S.word} · the ${Object.keys(S.bits).length} switches together ${all}, each one bit: ${single}`);
}
{ process.env.TPS_PROFILE = 'shipped';
  const { ROOT, characterSet, fake } = await import('./here.ts');
  const FR = await import(`${ROOT}/src/render/frame.ts`) as typeof import('../../src/render/frame');
  const { readFileSync } = await import('node:fs');
  const pre = FR.scenePrelude(), named = Object.entries(FR.LevelSw).every(([k, v]) => (pre.match(new RegExp(`const ${k}: u32 = ${v}u;`, 'g')) ?? []).length === 1 && !new RegExp(`const ${k}: bool`).test(pre));
  // (writeFrame's word against the struct's member: the member's offset by WGSL's rules is tools/qa/frame-layout.ts's; here, that the head a frame sends holds it)
  const was = FR.weatherKindConfig.on; FR.weatherKindConfig.on = true; const buf = new ArrayBuffer(FR.FRAME_BYTES);
  let wrote = -1; try { FR.writeFrame(buf, new Proxy({}, { get: (_t, k) => k === 'weather' ? FR.noWeather() : k === 'cardSlots' ? [] : typeof k === 'string' && /^(viewProj|invViewProj|prevViewProj)$/.test(k) ? new Float32Array(16) : typeof k === 'string' && /(Pos|Dir|Zenith|Horizon|Dims)$/.test(k) ? [0, 0, 0] : typeof k === 'string' && /(Cfg|CfgB|CfgC|Color|post|Jitter|jitter)$/.test(k) ? [0, 0, 0, 0] : k === 'sides' || k === 'sideCols' || k === 'smokeDraw' ? null : 0 }) as never); wrote = new Uint32Array(buf)[(FR.FRAME_HEAD_BYTES - 16) / 4]; } catch (e) { console.error(String(e).slice(0, 300)); }
  FR.weatherKindConfig.on = was;
  const common = readFileSync(`${ROOT}/src/render/shaders/common.wgsl`, 'utf8'), member = /\n  levelCfg: vec4u,[^\n]*\n  sides: array<vec4u, \d+>,/.test(common), asks = /fn lv\(bit: u32\) -> bool \{ return \(frame\.levelCfg\.x & bit\) != 0u; \}/.test(common);
  check(named && member && asks && (wrote & FR.LevelSw.WX_KINDS) !== 0, 'word — the prelude names each switch\'s bit once and no switch as a constant, the struct holds the word as the last member of the head, and writeFrame writes it there', `named ${named} · levelCfg in front of the tables ${member} · lv() reads it ${asks} · the word written at the head's last member with the weather kinds on: ${wrote}`);

  // ---- cast
  const { MeshShadows } = await import(`${ROOT}/src/render/meshShadows.ts`) as typeof import('../../src/render/meshShadows');
  const { SHADER_TRIM } = await import(`${ROOT}/src/gpu/wgslTrim.ts`) as typeof import('../../src/gpu/wgslTrim');
  const built = async (look: boolean) => { const texts: string[] = [], bufs: Uint32Array[] = [], base = fake('device');
    const device = new Proxy(base, { get(t, k) {
      if (k === 'createShaderModule') return (d: { code: string }) => { texts.push(d.code); return { getCompilationInfo: async () => ({ messages: [] }) }; };
      if (k === 'createBuffer') return (d: { label?: string; size: number }) => ({ destroy() { /* */ }, size: d.size, label: d.label });
      if (k === 'queue') return { writeBuffer(b: { label?: string }, _o: number, data: ArrayBuffer, from = 0, bytes = data.byteLength - from) { if (b.label === 'meshLevels') bufs.push(new Uint32Array(data.slice(from, from + bytes))); } };   // (gpu/device.ts makeBufferWithData: the data's own buffer, its offset, its length)
      if (k === 'then') return undefined; return t[k]; } });
    const ch = await characterSet(look), M = new MeshShadows(device as never, ch, fake('jointMats'), fake('charInsts'));
    return { text: texts[0] ?? '', sizes: [M.bvh.maxDepth + 1, M.bvh.numNodes, M.bvh.numTris, M.triBase], header: bufs.length ? Array.from(bufs[bufs.length - 1].subarray(60, 64)) : [] }; };
  const trimWas = SHADER_TRIM.on; const A = await built(false), L = await built(true); SHADER_TRIM.on = trimWas;
  if (BREAK) L.text = L.text.replace('const NODE_BASE', `const T_OF_THIS_CAST: u32 = ${L.sizes[2]}u; const NODE_BASE`);   // (the red path: a size written back into the text)
  const differ = A.sizes.join() !== L.sizes.join();
  check(A.text.length > 2000 && A.text === L.text && differ && A.header.join() === A.sizes.join() && L.header.join() === L.sizes.join(),
    'cast — the mesh shadows\' module is one text for both casts, and each cast\'s own sizes ride in its level list',
    `first cast: levels, nodes, triangles, corners at ${A.sizes.join(', ')} (its list says ${A.header.join(', ') || 'nothing'}) · the look's: ${L.sizes.join(', ')} (${L.header.join(', ') || 'nothing'}) · the two differ: ${differ} · one text of ${A.text.length} characters: ${A.text === L.text}`); }

console.log(red ? `levelset-check: ${red} FAILED${BREAK ? ' (LEVELSET_BREAK=1: this run must fail)' : ''}` : 'levelset-check: all passed'); process.exit(red ? 1 : 0);
