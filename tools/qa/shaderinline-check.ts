// HOW BIG EACH KERNEL IS ONCE EVERY CALL IS PASTED INTO ITS CALLER. Chrome on Windows compiles a pipeline's WGSL through DXC, and DXC pastes every call into
// its caller before it optimizes: a kernel pays for each CALL SITE of a heavy function (the tracer, `occluded`, `evalLight` with the fog's walk), not for the
// text the module was handed. This prints that size for every distinct (module text, entry point) the page asks a pipeline for, with no browser and no Windows
// machine: the page's boot is run over a stand-in device that records every createShaderModule text and every pipeline descriptor — the Engine, the smoke
// solver, the Game's own GPU modules (mesh shadows, particles, the light probe, the skinned raster), then the cut-box second set (render/cutSwap.ts build) —
// and each entry point is sized by the rule
//     pasted(f) = f's own text + for each call site in f, pasted(the function called)          (WGSL has no recursion)
// in KB of WGSL as gpu/wgslTrim.ts wgslDecls reads it (comments out). Beside it: the same functions counted once each (what a compiler that keeps its calls
// reads), the functions pasted most often into that kernel, the totals of the two sets, and the functions pasted most often over all kernels.
// THE BUDGET (the table below): each big kernel has its own number, within a twentieth of what it is today, and every other kernel the one at the table's
// foot, so a NEW call site worth a twentieth of a big kernel is red here before anybody waits for it on a Windows machine. A kernel that grew on purpose has
// its row raised in the same change; a rewrite that brings one down lowers its row with it (a row with a twentieth to spare is named, not failed; a row
// that no kernel answers to any more is red).
// What this is not: a time. DXC's time grows faster than the text and a loop round ONE call site is one copy here as it is there; the order of the list and
// what a rewrite takes out are what it is for. What it cannot see: a pipeline the page makes on first use, later than its boot (the entry points nobody asked
// a pipeline for are listed, not sized into the totals), and the switches a level or a URL flag turns (this is the title page as it opens: the shipped stand,
// cut boxes, the glass fracture and the dust twin on, the office).
//   bun run tools/qa/shaderinline-check.ts [--top N] [--kernel <module>/<entry>] [--json out.json]      exit 0 = within the budget, 1 = not, or the boot did not finish
//   TPS_COMPACTSHADERS=1 bun run tools/qa/shaderinline-check.ts   the same for the compact set (the row 'Compact shaders', src/ui/shaderrows.ts), against its own table
//   SHADERINLINE_BREAK=1 bun run tools/qa/shaderinline-check.ts     one more call site is written into the biggest kernel, of the lightest function it reaches
//                                                                    whose call site is a twentieth of it: must FAIL (it does not while that
//                                                                    kernel's row has a twentieth to spare: the plain run names the row)
// --kernel prints every function pasted into that one kernel (copies, own size, the share, what one call site of it costs); --json writes the rows, and each module's text hash as
// tools/qa/shader-count-page.cjs prints it (the real page's modules can be held against these).
export {};
process.env.TPS_PROFILE = 'shipped';   // (the page's shaders: the frozen stand holds switches off that are consts in the scene prelude — other texts, other sizes)
const { ROOT, fake, loadCharacter } = await import('./headless.ts');
const { Engine } = await import(`${ROOT}/src/engine.ts`);
const { SmokeSystem } = await import(`${ROOT}/src/smoke/smoke.ts`);
const { standGame } = await import(`${ROOT}/src/host/standup.ts`);
const { cutSwap } = await import(`${ROOT}/src/render/cutSwap.ts`);
const { cutConfig } = await import(`${ROOT}/src/scene/cut.ts`);
const { roundConfig, ROUND } = await import(`${ROOT}/src/scene/round.ts`);
const { glassFxBoot } = await import(`${ROOT}/src/game/glass.ts`);
const { glassBreakConfig } = await import(`${ROOT}/src/render/frame.ts`);
const { DUST_DRAW } = await import(`${ROOT}/src/smoke/dustTint.ts`);
const { wgslDecls } = await import(`${ROOT}/src/gpu/wgslTrim.ts`) as typeof import('../../src/gpu/wgslTrim');
const { COMPACT } = await import(`${ROOT}/src/render/kernels.ts`) as typeof import('../../src/render/kernels');   // (the row 'Compact shaders': TPS_COMPACTSHADERS=1 in front sizes that set, against its own table below)

/** KB of pasted WGSL a kernel may not pass, by `module / entry` (a cut-set module's `Cut` off its label), the first row that matches; the last is everyone else's */
const BUDGET_TODAY: [RegExp, number][] = [
  [/^composite(R|S|SR)? \/ (fs|cs)$/, 1010],
  [/^fgatherS? \/ mainGi$/, 870],
  [/^fgatherS? \/ main$/, 805],
  [/^volumetricsDust \/ main$/, 740],
  [/^volumetrics \/ main$/, 690],
  [/^reflectP5 \/ main$/, 660],
  [/^particles \/ update$/, 605],
  [/^direct \/ main$/, 420],
  [/^restirM? \/ main$/, 360],
  [/^reflect \/ main$/, 284],
  [/^gbuffer \/ fsBox$/, 270],
  [/^froxel \/ injectJ$/, 231],
  [/^sharpUE? \/ main$/, 230],
  [/./, 200],
];
/** the same for THE COMPACT SET (the row 'Compact shaders' on: render/kernels.ts, the ten kernels written with one call site per heavy function), its own rows within a
 *  twentieth of what that set is. A second table, never today's rows lowered: both writings are in the tree while the row is on trial, and each is held to its own size */
const BUDGET_COMPACT: [RegExp, number][] = [
  [/^composite(R|S|SR)? \/ (fs|cs)$/, 593],
  [/^volumetricsDust \/ main$/, 520],
  [/^volumetricsF? \/ main$/, 470],
  [/^particles \/ update$/, 373],
  [/^restirM? \/ main$/, 318],
  [/^fgatherS? \/ mainGi$/, 304],
  [/^gbuffer \/ fsBox$/, 270],
  [/^direct \/ main$/, 269],
  [/^reflectP5 \/ main$/, 251],
  [/^fgatherS? \/ main$/, 236],
  [/^froxel \/ injectJ$/, 231],
  [/./, 200],
];
const BUDGET = COMPACT.on ? BUDGET_COMPACT : BUDGET_TODAY;
const BREAK = process.env.SHADERINLINE_BREAK === '1';
const argv = process.argv.slice(2);
const opt = (k: string): string | null => { const i = argv.indexOf('--' + k); if (i < 0) return null; const v = argv[i + 1]; if (v === undefined || v.startsWith('--')) { console.error(`--${k} needs a value`); process.exit(1); } return v; };
const TOP = ((v: string | null) => { if (v === null) return Infinity; if (!/^[1-9]\d*$/.test(v)) { console.error(`--top needs a count, not '${v}'`); process.exit(1); } return Number(v); })(opt('top'));
const KERNEL = opt('kernel'), JSON_OUT = opt('json');
const say = console.log.bind(console);   // (the boot below is run with the console's own lines held back: a check's line is not one of those)
let red = 0; const check = (ok: boolean, what: string) => { say(`${ok ? '- PASS' : '- FAIL'} — ${what}`); if (!ok) red++; };

// ---- the page's boot over a recording stand-in
type Stage = 'compute' | 'vertex' | 'fragment';
interface Mod { label: string; code: string }
interface Asked { label: string; set: 'boot' | 'cut'; stages: { stage: Stage; mod: number; entry: string | undefined }[] }
const mods: Mod[] = [], asked: Asked[] = []; let set: 'boot' | 'cut' = 'boot';
{
  // what main.ts writes before it builds the Engine on a plain address (urlFlagsBeforeEngine): cut boxes on, round things on the GPU with them, the glass fracture —
  // and the one switch that reads the address as its module loads (the dust's own look: the volumetric march's dust twin, the page only)
  cutConfig.on = true; roundConfig.gpu = ROUND.on && cutConfig.on; if (glassFxBoot()) glassBreakConfig.on = true; DUST_DRAW.on = true;
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
  // (a step the stand-in never answers would leave the list short and the budget green on a part of the page: red instead)
  const within = async <T,>(what: string, p: Promise<T>): Promise<T | undefined> => { const late = Symbol(); const v = await Promise.race([p, new Promise<symbol>(r => setTimeout(() => r(late), 20000))]); if (v === late) { check(false, `the stand-in's boot finished its step '${what}'`); return undefined; } return v as T; };
  const log = console.log, warn = console.warn; console.log = console.warn = () => { /* the boot's own lines (a module left out on this device, the cut set's ready line) are not this tool's */ };
  try {
    // main.ts's own order: the Engine's pipelines asked for, the smoke solver's, the Game stood on that Engine, then the Game's GPU modules — and the cut set after them all
    const booted = await within('boot', (async () => {
      const engine = new Engine({ device, adapterInfo: 'stand-in', canvas: fake('canvas'), ctx: fake('ctx'), format: 'bgra8unorm', hasTimestamps: false, linearSampler: fake('sampler') } as never);
      const engineInit = engine.init({ sharp: true, gbaa: true });
      const smoke = new SmokeSystem(engine); engine.setSmoke(smoke.binding);
      const smokeInit = smoke.init();
      const { game } = await standGame(engine, { ch: await loadCharacter(), smoke, seed: 1 });
      await Promise.all([engineInit, smokeInit, game.meshShadows.init(), engine.particles?.init(), game.probe.init?.(), engine.gbuffer.prepareSkin(game.chars.jointsFormat)]);
      return true;
    })());
    set = 'cut';
    const cut = booted ? await within('cut set', cutSwap.build()) : undefined;
    if (cut !== undefined) check(cut === true, `the cut-box second set was built (${cutSwap.pipelines} pipelines of ${cutSwap.built} owners${cutSwap.error ? ` — ${cutSwap.error}` : ''})`);
  } finally { console.log = log; console.warn = warn; }
}

// ---- a module's functions, and what each calls
interface Fn { name: string; stage: Stage | null; own: number; calls: [string, number][] }
const HEAD = /^((?:\s*@[A-Za-z_]\w*(?:\s*\((?:[^()]|\([^()]*\))*\))?)*)\s*fn\s+([A-Za-z_]\w*)/;
const CALL = /([A-Za-z_]\w*)\s*\(/g;
const fnsOfText = new Map<string, Map<string, Fn>>();
function fnsOf(code: string): Map<string, Fn> {
  let fns = fnsOfText.get(code); if (fns) return fns;
  fns = new Map(); fnsOfText.set(code, fns);
  const bodies = new Map<string, string>();
  for (const d of wgslDecls(code)) {
    const m = HEAD.exec(d); if (!m) continue;
    const stage = (/@(compute|vertex|fragment)\b/.exec(m[1])?.[1] ?? null) as Stage | null;
    fns.set(m[2], { name: m[2], stage, own: d.length, calls: [] }); bodies.set(m[2], d.slice(d.indexOf('{', m[0].length)));
  }
  for (const f of fns.values()) {
    const n = new Map<string, number>();
    for (const c of bodies.get(f.name)!.matchAll(CALL)) if (fns.has(c[1])) n.set(c[1], (n.get(c[1]) ?? 0) + 1);
    f.calls = [...n];
  }
  return fns;
}
/** how many copies of each function `name`'s code holds once every call in it is pasted (itself: 1) */
const pastedOf = new WeakMap<Map<string, Fn>, Map<string, Map<string, number>>>();
function pasted(fns: Map<string, Fn>, name: string, open = new Set<string>()): Map<string, number> {
  let memo = pastedOf.get(fns); if (!memo) pastedOf.set(fns, memo = new Map());
  const have = memo.get(name); if (have) return have;
  if (open.has(name)) throw new Error(`a function that calls itself: ${name}`);
  open.add(name);
  const copies = new Map<string, number>([[name, 1]]);
  for (const [callee, n] of fns.get(name)!.calls) for (const [g, k] of pasted(fns, callee, open)) copies.set(g, (copies.get(g) ?? 0) + n * k);
  open.delete(name); memo.set(name, copies); return copies;
}

// ---- THE BREAK: the biggest kernel takes one more call site, of the lightest function it reaches whose call site is a twentieth of it
const sizeOf = (fns: Map<string, Fn>, copies: Map<string, number>) => { let s = 0; for (const [g, k] of copies) s += k * fns.get(g)!.own; return s; };
const entryOf = (m: Mod, stage: Stage, entry: string | undefined): string | null => { const fns = fnsOf(m.code); if (entry !== undefined) return fns.get(entry)?.stage === stage ? entry : null; const own = [...fns.values()].filter(f => f.stage === stage); return own.length === 1 ? own[0].name : null; };
if (BREAK) {
  let best: { mod: number; entry: string; size: number } | null = null;
  for (const a of asked) for (const s of a.stages) { const m = mods[s.mod]; const e = m && entryOf(m, s.stage, s.entry); if (!e) continue; const fns = fnsOf(m.code); const size = sizeOf(fns, pasted(fns, e)); if (!best || size > best.size) best = { mod: s.mod, entry: e, size }; }
  if (best) {
    const m = mods[best.mod], fns = fnsOf(m.code), one = (g: string) => sizeOf(fns, pasted(fns, g));
    const heavy = [...pasted(fns, best.entry).keys()].filter(g => g !== best.entry && one(g) >= best.size / 20).sort((a, b) => one(a) - one(b))[0];
    if (!heavy) throw new Error(`SHADERINLINE_BREAK: nothing ${m.label} / ${best.entry} reaches is a twentieth of it`);
    const at = m.code.indexOf('{', m.code.search(new RegExp(`\\bfn\\s+${best.entry}\\s*\\(`)));
    const code = `${m.code.slice(0, at + 1)} ${heavy}(); ${m.code.slice(at + 1)}`;
    for (const o of mods) if (o.code === m.code && o !== m) o.code = code;
    m.code = code;
    console.log(`(SHADERINLINE_BREAK: one more call of ${heavy}, ${Math.round(one(heavy) / 1024)} KB a call site, written into ${m.label} / ${best.entry})`);
  }
}

// ---- every distinct (module text, entry point)
interface Row { module: string; entry: string; stage: Stage; set: 'boot' | 'cut'; pipelines: string[]; pasted: number; once: number; fns: Map<string, Fn>; copies: Map<string, number> }
const rows = new Map<string, Row>(); const idOf = new Map<string, number>(); const lost: string[] = [];
for (const a of asked) for (const s of a.stages) {
  const m = mods[s.mod]; const entry = m ? entryOf(m, s.stage, s.entry) : null;
  if (!m || !entry) { lost.push(`${a.label}${s.entry ? ` / ${s.entry}` : ''}`); continue; }
  if (!idOf.has(m.code)) idOf.set(m.code, idOf.size);
  const key = `${idOf.get(m.code)}/${entry}`; let r = rows.get(key);
  if (!r) { const fns = fnsOf(m.code), copies = pasted(fns, entry); let once = 0; for (const g of copies.keys()) once += fns.get(g)!.own;
    rows.set(key, r = { module: m.label, entry, stage: s.stage, set: a.set, pipelines: [], pasted: sizeOf(fns, copies), once, fns, copies }); }
  if (!r.pipelines.includes(a.label)) r.pipelines.push(a.label);
}
const list = [...rows.values()].sort((a, b) => b.pasted - a.pasted || (`${a.module} ${a.entry} ${a.set}` < `${b.module} ${b.entry} ${b.set}` ? -1 : 1));
check(lost.length === 0, `every pipeline's module and entry point were found in the recorded texts${lost.length ? ` — not so: ${lost.slice(0, 6).join(', ')}` : ''}`);
{ // (a part of the page that fell out of this boot would leave the budget green on the rest: the card's own count is 118 pipelines, the cut set's 24)
  const nBoot = asked.filter(a => a.set === 'boot').length, nCut = asked.length - nBoot;
  check(mods.length >= 60 && nBoot >= 118 && nCut >= 24, `the recording is a whole boot: ${mods.length} modules (60 at least), ${nBoot} pipelines of the boot's set (118 at least), ${nCut} of the cut set's (24 at least)`); }

const kb = (n: number) => (n / 1024).toFixed(n < 10240 ? 1 : 0);
/** the functions pasted more than once into a kernel, by the text their copies come to */
const most = (r: Row) => [...r.copies].filter(([, k]) => k > 1).map(([g, k]) => ({ fn: g, copies: k, bytes: k * r.fns.get(g)!.own })).sort((a, b) => b.bytes - a.bytes || (a.fn < b.fn ? -1 : 1));
const nameOf = (r: Row) => `${r.module} / ${r.entry}`;
const rowOf = (r: Row) => BUDGET.findIndex(([re]) => re.test(`${r.module.replace(/Cut$/, '')} / ${r.entry}`)), REST = BUDGET.length - 1;
const budgetOf = (r: Row) => BUDGET[rowOf(r)][1];
console.log(`every kernel with every call pasted into its caller — ${mods.length} modules, ${asked.length} pipelines (${asked.filter(a => a.set === 'boot').length} the boot's, ${asked.filter(a => a.set === 'cut').length} the cut set's), ${list.length} distinct (module text, entry point); KB of WGSL, comments out`);
console.log('  pasted    once     ×  set   module / entry (pipelines)  ·  pasted most: function ×copies (KB of them)');
for (const r of list.slice(0, TOP)) {
  const names = r.pipelines.filter(l => l !== r.module && l !== r.entry);
  console.log(`${kb(r.pasted).padStart(8)}${kb(r.once).padStart(8)}${(r.pasted / r.once).toFixed(1).padStart(6)}  ${r.set.padEnd(4)}  ${nameOf(r)}${names.length ? ` (${names.slice(0, 4).join(', ')}${names.length > 4 ? `, +${names.length - 4}` : ''})` : ''}${most(r).length ? `  ·  ${most(r).slice(0, 4).map(c => `${c.fn} ×${c.copies} (${kb(c.bytes)})`).join(', ')}` : ''}`);
}
if (list.length > TOP) console.log(`     … ${list.length - TOP} more (no --top = every one)`);
const sum = (rs: Row[], k: 'pasted' | 'once') => rs.reduce((s, r) => s + r[k], 0);
for (const [name, rs] of [['the boot\'s set', list.filter(r => r.set === 'boot')], ['the cut-box set', list.filter(r => r.set === 'cut')], ['both', list]] as const)
  console.log(`total, ${name}: ${rs.length} kernels, ${kb(sum(rs, 'pasted'))} KB pasted where ${kb(sum(rs, 'once'))} KB is each function once (×${(sum(rs, 'pasted') / Math.max(1, sum(rs, 'once'))).toFixed(1)})`);
{ // the functions pasted most often, over every kernel
  const all = new Map<string, { copies: number; bytes: number; kernels: number }>();
  for (const r of list) for (const [g, k] of r.copies) { if (g === r.entry) continue; const a = all.get(g) ?? { copies: 0, bytes: 0, kernels: 0 }; a.copies += k; a.bytes += k * r.fns.get(g)!.own; a.kernels++; all.set(g, a); }
  console.log('pasted most often, over all kernels (every copy, a kernel\'s first too):');
  for (const [g, a] of [...all].sort((x, y) => y[1].bytes - x[1].bytes || (x[0] < y[0] ? -1 : 1)).slice(0, 12)) console.log(`${kb(a.bytes).padStart(8)} KB  ${g} ×${a.copies} in ${a.kernels} kernels`);
}
{ // entry points of the recorded modules that no pipeline was asked for
  const idle: { name: string; size: number }[] = []; const seen = new Set<string>();
  for (const m of mods) { if (seen.has(m.code)) continue; seen.add(m.code); const id = idOf.get(m.code); for (const f of fnsOf(m.code).values()) if (f.stage && !(id !== undefined && rows.has(`${id}/${f.name}`)) && !seen.has(`${m.label} / ${f.name}`) && seen.add(`${m.label} / ${f.name}`)) idle.push({ name: `${m.label} / ${f.name}`, size: sizeOf(fnsOf(m.code), pasted(fnsOf(m.code), f.name)) }); }
  if (idle.length) console.log(`${idle.length} entry points nobody asked a pipeline for here (made on first use in the page, or never; not in the totals), the biggest: ${idle.sort((a, b) => b.size - a.size).slice(0, 8).map(i => `${i.name} ${kb(i.size)}`).join(' · ')}`);
}
if (KERNEL !== null) {
  const bare = (t: string) => t.replace(/\s/g, ''), want = bare(KERNEL); const hit = list.filter(r => bare(`${r.module}/${r.entry}`) === want || bare(r.module) === want || r.pipelines.some(l => bare(l) === want));
  check(hit.length > 0, `--kernel ${KERNEL} names a kernel of the list`);
  for (const r of hit) { console.log(`${nameOf(r)} (${r.set}): ${kb(r.pasted)} KB pasted, ${r.copies.size} functions`);
    for (const [g, k] of [...r.copies].sort((a, b) => b[1] * r.fns.get(b[0])!.own - a[1] * r.fns.get(a[0])!.own)) console.log(`${kb(k * r.fns.get(g)!.own).padStart(8)} KB ${(100 * k * r.fns.get(g)!.own / r.pasted).toFixed(1).padStart(5)} %  ${g} ×${k} of ${kb(r.fns.get(g)!.own)} KB (one call site of it: ${kb(sizeOf(r.fns, pasted(r.fns, g)))} KB)`); }
}
const over = list.filter(r => r.pasted > budgetOf(r) * 1024);
// a row of the table by the biggest kernel that answers to it: none = the row is stale, a twentieth under = it has room a new call site would hide in
const rowsBig = BUDGET.slice(0, REST).map(([re, cap], i) => ({ re, cap, big: list.find(r => rowOf(r) === i) }));
const stale = rowsBig.filter(x => !x.big), spare = rowsBig.filter(x => x.big && x.big.pasted < x.cap * 1024 * 0.95);
check(list.length > 0 && over.length === 0, `every kernel is within its budget (the biggest: ${list.length ? `${nameOf(list[0])}, ${kb(list[0].pasted)} of ${budgetOf(list[0])} KB` : 'none recorded'}; a kernel with no row of its own: ${BUDGET[REST][1]} KB)${over.length ? ` — not so: ${[...new Set(over.map(r => `${nameOf(r)} ${kb(r.pasted)} of ${budgetOf(r)} KB`))].slice(0, 8).join(', ')} (take the call site out, or raise the kernel's row in this file on purpose; a new big kernel gets a row of its own)` : ''}`);
check(stale.length === 0, `every row of the budget table has a kernel that answers to it${stale.length ? ` — not so: ${stale.map(x => String(x.re)).join(', ')} (a kernel renamed or gone: its row goes with it)` : ''}`);
if (spare.length) console.log(`  ${spare.length} rows with a twentieth to spare, by their biggest kernel (lower each to it, or take the row out where it is under ${BUDGET[REST][1]} KB): ${spare.map(x => `${nameOf(x.big!)} ${kb(x.big!.pasted)} of ${x.cap}`).join(', ')}`);
if (JSON_OUT) {
  const hash = (s: string) => { let a = 0x811c9dc5, b = 0x1b873593; for (let i = 0; i < s.length; i++) { const c = s.charCodeAt(i); a = Math.imul(a ^ c, 16777619); b = Math.imul(b ^ c, 0x85ebca6b) + (b >>> 13); } return (a >>> 0).toString(36) + (b >>> 0).toString(36); };   // (shader-count-page.cjs's own)
  await Bun.write(JSON_OUT, JSON.stringify({ modules: mods.map(m => ({ label: m.label, h: hash(m.code), chars: m.code.length })),
    kernels: list.map(r => ({ module: r.module, entry: r.entry, stage: r.stage, set: r.set, pipelines: r.pipelines, pasted: r.pasted, once: r.once, budgetKB: budgetOf(r), most: most(r).slice(0, 8) })) }, null, 1));
}
console.log(red ? `${red} FAILED` : 'all passed');
process.exit(red ? 1 : 0);
