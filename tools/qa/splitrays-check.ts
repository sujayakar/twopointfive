// SEPARATE SHADOW RAYS (src/render/gatherSplit.ts, shaders/gather_split.wgsl; ui/shaderrows.ts row `splitRays`, ?splitrays=1, off by default). With the row on the final
// gather is two programs — `trace` writes the points it needs lit, `light` lights them round ONE directAtPoint — instead of one that DXC pastes to 780 KB. What is checked
// here is what a harness can see of that: the two programs' texts (who reaches the light loop, through how many call sites), that the buffer's layout is one number in the
// shader and in the engine, that the three functions of fgather.wgsl the split was copied from are the ones it was copied from, and what a load compiles with the row off
// (today's four gathers, nothing of the split) and on (the three split pipelines in both sets, each far under today's `main`, the size tool's budget green).
// NOT here (no GPU, and a harness compiles nothing): that the WGSL compiles, and the picture. Those are the page's — tools/qa/draw-equal.cjs with `--with splitrays=1`
// against the twin of tools/qa/splitrays-twin.cjs; docs/agents/render.md "Separate shadow rays" has what was measured.
//   bun run tools/qa/splitrays-check.ts                        exit 0 = every line passed, 1 = one failed (passes on both stands: TPS_PROFILE=shipped in front)
//   SPLITRAYS_BREAK=1 bun run tools/qa/splitrays-check.ts      the row is left OFF where the lines turn it on (the megakernels are what is built): must FAIL
import { createHash } from 'node:crypto';
import { readFileSync, mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { ROOT } from './headless.ts';

/** the three functions of fgather.wgsl that gather_split.wgsl's trace / glossReflectionX / glassShadeX were copied from (comments out, as wgslDecls reads them): a change to
 *  one is a change the split needs too — make it there, then pin the new text here */
const COPIED: Record<string, string> = { main: '6f228c73dddd', glossReflection: 'c50ca1a6e9a9', glassShade: '6ec8def72f80' };

type Kernel = { module: string; entry: string; set: string; pipelines: string[]; pasted: number; budgetKB: number | null };
type Sizes = { modules: { label: string; h: string }[]; kernels: Kernel[] };

export async function splitraysCheck(): Promise<boolean> {
  const { wgslDecls } = await import(`${ROOT}/src/gpu/wgslTrim.ts`) as typeof import('../../src/gpu/wgslTrim');
  const { splitGatherSrc, gatherHelpers, SPLIT_TEXEL_BYTES, SPLIT_RAYS } = await import(`${ROOT}/src/render/gatherSplit.ts`) as typeof import('../../src/render/gatherSplit');
  const BRK = process.env.SPLITRAYS_BREAK === '1'; let all = true;
  const line = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== splitrays: the gather's shadow rays are their own kernel${BRK ? ' (the row left OFF: must fail)' : ''} ===`);
  const dir = `${ROOT}/src/render/shaders`;
  const fgather = readFileSync(`${dir}/fgather.wgsl`, 'utf8'), split = readFileSync(`${dir}/gather_split.wgsl`, 'utf8'), passes = readFileSync(`${ROOT}/src/render/passes.ts`, 'utf8'), rows = readFileSync(`${ROOT}/src/ui/shaderrows.ts`, 'utf8');

  // ---- the texts
  /** a module's functions by name (an entry point under its own name), and who calls whom among them */
  const fns = (code: string) => { const m = new Map<string, string>(); for (const d of wgslDecls(code)) { const n = /^(?:@compute[^\n]*\s+)?fn (\w+)\(/.exec(d); if (n) m.set(n[1], d); } return m; };
  const calls = (m: Map<string, string>, body: string, self: string) => [...m.keys()].filter(n => n !== self && new RegExp(`\\b${n}\\(`).test(body));
  const reach = (m: Map<string, string>, from: string) => { const seen = new Set<string>([from]), todo = [from]; while (todo.length) { const f = todo.pop()!; for (const n of calls(m, m.get(f) ?? '', f)) if (!seen.has(n)) { seen.add(n); todo.push(n); } } return seen; };
  const sites = (m: Map<string, string>, within: Set<string>, callee: string) => [...within].reduce((s, f) => s + (f === callee ? 0 : (m.get(f) ?? '').split(new RegExp(`\\b${callee}\\(`)).length - 1), 0);
  { const m = fns(splitGatherSrc(false)), ms = fns(splitGatherSrc(true)), entries = (x: Map<string, string>) => [...x].filter(([, d]) => d.startsWith('@compute')).map(([n]) => n).sort().join(' ');
    const helpers = fns(gatherHelpers());
    line(entries(m) === 'light trace' && entries(ms) === 'light trace' && !helpers.has('main') && !helpers.has('mainGi') && helpers.has('glossReflection') && helpers.has('cardIrradiance'),
      'the split module: fgather.wgsl\'s helpers without its two megakernels, and two entry points of its own (the strata twin the same)', `entry points: ${entries(m)} · helpers kept ${helpers.size} of fgather.wgsl's ${fns(fgather).size} functions`);
    const tr = reach(m, 'trace'), li = reach(m, 'light');
    line(m.has('directAtPoint') && !tr.has('directAtPoint') && !tr.has('directOnScreen') && !tr.has('glossReflection') && !tr.has('glassShade') && !tr.has('youngRay'),
      '`trace` reaches no light loop: neither directAtPoint nor directOnScreen, nor a helper of today\'s gather that calls one', `trace reaches ${tr.size} functions: ${[...tr].filter(f => /X$|^gx/.test(f)).join(', ')} of its own`);
    const tracers = /\b(traceClosest|occluded\w*|glassTrace|smokeTransmittance|diceIrradiance\w*|cardIrradiance|glossGlints)\(/, lightOwn = [...li].filter(f => f !== 'light' && f !== 'directAtPoint').sort().join(' ');   // (the tracer itself is the scene prelude's, outside this text: `light`'s own body is read for a call into it)
    line(li.has('directAtPoint') && sites(m, li, 'directAtPoint') === 1 && !tracers.test(m.get('light') ?? '') && tracers.test(m.get('trace') ?? '') && lightOwn === 'gxFork rnd rngInit',
      '`light` reaches directAtPoint through ONE call site, and its own body traces nothing and reads no cache: beside the light loop it reaches the random number helpers only', `beside directAtPoint, light reaches: ${lightOwn} · directAtPoint call sites under light ${sites(m, li, 'directAtPoint')} (under today's main ${sites(fns(fgather + readFileSync(`${dir}/gather_common.wgsl`, 'utf8')), reach(fns(fgather), 'main'), 'directAtPoint')}) · light reaches ${li.size} functions`); }
  { const num = (re: RegExp) => { const x = re.exec(split); return x ? Number(x[1]) : NaN; };
    const v4 = num(/^const GX_V4: u32 = (\d+)u;/m), rec = num(/^const GX_REC: u32 = (\d+)u;/m), slots = num(/^const GX_SLOTS: u32 = (\d+)u;/m), rays = Number(/^override RAYS: u32 = (\d+)u;/m.exec(fgather)?.[1]), bind = num(/^@group\(1\) @binding\((\d+)\) var<storage, read_write> gx:/m);
    const lay = /label: 'gatherSplit', entries: \[[^\]]*?\{ binding: (\d+), visibility: CS, buffer: \{ type: 'storage' \} \},\n\s*\] \}\);/.exec(passes);
    line(v4 * 16 === SPLIT_TEXEL_BYTES && rec + 2 * slots === v4 && bind === Number(lay?.[1]) && rays + 2 === slots && !/constants: \{[^}]*RAYS/.test(passes),
      'the points\' buffer is one layout in the shader and in the engine: a texel\'s words, its header and two words a point, a slot for each ray and the two mirror rays, the binding', `RAYS ${rays} + 2 = ${slots} slots · GX_V4 ${v4} × 16 B = ${v4 * 16} (engine ${SPLIT_TEXEL_BYTES}) · ${rec} + 2 × ${slots} = ${rec + 2 * slots} · binding ${bind} (passes.ts ${lay?.[1] ?? 'not found'})`); }
  { const f = fns(fgather), got = Object.fromEntries(Object.keys(COPIED).map(n => [n, createHash('sha256').update(f.get(n) ?? '').digest('hex').slice(0, 12)]));
    const off = Object.keys(COPIED).filter(n => got[n] !== COPIED[n]);
    line(off.length === 0, 'fgather.wgsl\'s main, glossReflection and glassShade are the texts gather_split.wgsl was copied from (a change to one goes into the split too, then its pin moves here)',
      off.length ? `changed: ${off.map(n => `${n} is ${got[n]}, pinned ${COPIED[n]}`).join(' · ')}` : `three pins hold`); }
  { let twin = '', err = ''; try { twin = ((await import(`${ROOT}/tools/qa/splitrays-twin.cjs`)).default as { text(root: string): string }).text(ROOT); } catch (e) { err = (e as Error).message; }
    const n = twin.split('directAtPointX(').length - 1, t = fns(twin);
    line(!err && n === 4 && t.has('main') && t.has('mainGi') && sites(t, new Set(['main', 'glossReflection', 'glassShade']), 'directAtPoint') === 0,
      'the twin the pictures are compared with builds from today\'s text: one megakernel whose three lit points draw from the split\'s streams', err || `directAtPointX written ${n} times (its own line and three calls) · ${twin.length - fgather.length} characters added`); }
  { const row = /\{ key: 'splitRays', url: '(\w+)', label: '([^']+)',/.exec(rows);
    line(!!row && row[1] === 'splitrays' && SPLIT_RAYS.on === (process.env.TPS_SPLITRAYS !== undefined && process.env.TPS_SPLITRAYS !== '0') && /\n    if \(SPLIT_RAYS\.on\) \{[^\n]*\n(?:      [^\n]*\n){4}    \} else\n    \{ const fgMod = mk\(gatherCommonSrc \+ /.test(passes),
      'the row: one of the Shaders rows under its own word, off in a harness that was not given the word, and passes.ts builds either the split\'s three pipelines or today\'s four', row ? `'${row[2]}' ?${row[1]}=1 · on in this run: ${SPLIT_RAYS.on}` : 'no row'); }

  // ---- what a load compiles, row off and row on (the size tool, a run of its own each: the row is read once at import)
  const tmp = mkdtempSync(join(tmpdir(), 'splitrays-'));
  const sizes = async (row: boolean): Promise<{ j: Sizes | null; green: boolean; out: string }> => {
    const env: Record<string, string | undefined> = { ...process.env, SHADERINLINE_BREAK: undefined, TPS_SPLITRAYS: row && !BRK ? '1' : undefined }, f = join(tmp, row ? 'on.json' : 'off.json');
    const r = Bun.spawn(['bun', 'run', `${ROOT}/tools/qa/shaderinline-check.ts`, '--json', f], { env, stdout: 'pipe', stderr: 'ignore' }), text = await new Response(r.stdout).text(), code = await r.exited;
    let j: Sizes | null = null; try { j = JSON.parse(readFileSync(f, 'utf8')) as Sizes; } catch { /* no file: the line says so */ }
    return { j, green: code === 0, out: text.trim().split('\n').slice(-1)[0] ?? '' };
  };
  try {
    const KB = (n: number) => `${Math.round(n / 1024)} KB`, gathers = (j: Sizes | null, re: RegExp) => (j?.kernels ?? []).filter(k => re.test(k.module));
    const [off, on] = await Promise.all([sizes(false), sizes(true)]);
    const megaOff = gathers(off.j, /^fgatherS?$/), splitOff = gathers(off.j, /^gatherSplit/), megaOn = gathers(on.j, /^fgatherS?$/), splitOn = gathers(on.j, /^gatherSplit/);
    const main = Math.max(0, ...megaOff.filter(k => k.entry === 'main').map(k => k.pasted));
    line(off.green && megaOff.length === 8 && splitOff.length === 0 && !(off.j?.modules ?? []).some(m => /^gatherSplit/.test(m.label)),
      'the row off: a load compiles today\'s gathers (main, mainGi and their strata twins, in the boot set and the cut set) and nothing of the split', `megakernel gathers ${megaOff.length} · split kernels ${splitOff.length} · main ${KB(main)} · size tool: ${off.out}`);
    const pipes = (set: string) => splitOn.filter(k => k.set === set).flatMap(k => k.pipelines).sort().join(' ');
    line(megaOn.length === 0 && pipes('boot') === 'gatherLight gatherTrace gatherTraceS' && pipes('cut') === pipes('boot'),
      'the row on: the three split pipelines stand in both sets and no megakernel gather is compiled', `megakernel gathers ${megaOn.length} · boot set: ${pipes('boot') || 'none'} · cut set: ${pipes('cut') || 'none'}`);
    const big = Math.max(0, ...splitOn.map(k => k.pasted));
    line(splitOn.length > 0 && main > 0 && big * 3 < main && on.green && splitOn.every(k => k.budgetKB !== null && k.budgetKB < 300),
      'each of the split\'s kernels is under a third of today\'s `main` as DXC pastes it, has a budget row of its own, and the size tool is green with the row on', `largest split kernel ${KB(big)} of today's ${KB(main)} · ${[...new Set(splitOn.map(k => `${k.entry} ${KB(k.pasted)} of ${k.budgetKB}`))].join(' · ')} · size tool: ${on.out}`);
    // (a module is in the list once a set it is compiled in: compared as label + text, whichever set it is from)
    const texts = (j: Sizes | null) => (j?.modules ?? []).filter(m => !/^(fgatherS?|gatherSplitS?)$/.test(m.label)).map(m => `${m.label} ${m.h}`).sort();
    const a = texts(off.j), b = texts(on.j), moved = a.filter((x, i) => b[i] !== x).map(x => x.split(' ')[0]);
    line(a.length > 40 && a.length === b.length && moved.length === 0 && megaOn.length === 0, 'no other module\'s text moves with the row: reflect, restir, the marches and the composites are what they were', `module texts compared ${a.length} · changed ${moved.length ? [...new Set(moved)].join(', ') : 'none'}`);
  } finally { rmSync(tmp, { recursive: true, force: true }); }
  return all;
}

if (import.meta.main) { const ok = await splitraysCheck(); console.log(ok ? '\n=== splitrays-check: ALL PASS ===' : '\n=== splitrays-check: FAILED ==='); process.exit(ok ? 0 : 1); }
