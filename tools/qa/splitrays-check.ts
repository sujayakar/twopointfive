// THE GATHER IS TWO PROGRAMS (src/render/gatherSplit.ts, shaders/gather_split.wgsl; until 2026-11-07 the Shaders row 'Separate shadow rays'). `trace` writes the points
// it needs lit, `light` lights them round ONE directAtPoint — where the one-kernel gather that went was a program DXC pasted to 780 KB. What is checked here is what a
// harness can see of that: the two programs' texts (who reaches the light loop, through how many call sites), that the buffer's layout is one number in the shader and in
// the engine, that a refused buffer is heard outside the machine, and what a load compiles (the three gather pipelines in both sets, each with a budget row of its own,
// the size tool green, and no one-kernel gather).
// NOT here (no GPU, and a harness compiles nothing): that the WGSL compiles (tools/qa/wgsl-compile.cjs), and the picture.
//   bun run tools/qa/splitrays-check.ts                        exit 0 = every line passed, 1 = one failed
//   SPLITRAYS_BREAK=1 bun run tools/qa/splitrays-check.ts      `light` is read with a second call site of the light loop written into it: must FAIL
import { readFileSync, mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { ROOT } from './headless.ts';

type Kernel = { setting?: string; module: string; entry: string; set: string; pipelines: string[]; pasted: number; cut?: number; budgetKB: number | null };   // (cut: the cut test's text in it, which the size tool counts apart from the kernel's row)
type Sizes = { modules: { label: string; h: string }[]; kernels: Kernel[] };

export async function splitraysCheck(): Promise<boolean> {
  const { wgslDecls } = await import(`${ROOT}/src/gpu/wgslTrim.ts`) as typeof import('../../src/gpu/wgslTrim');
  const { splitGatherSrc, gatherHelpers, SPLIT_TEXEL_BYTES, splitRefused } = await import(`${ROOT}/src/render/gatherSplit.ts`) as typeof import('../../src/render/gatherSplit');
  const BRK = process.env.SPLITRAYS_BREAK === '1'; let all = true;
  const line = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== splitrays: the gather's shadow rays are their own kernel${BRK ? ' (a second call site of the light loop in `light`: must fail)' : ''} ===`);
  const dir = `${ROOT}/src/render/shaders`;
  const fgather = readFileSync(`${dir}/fgather.wgsl`, 'utf8'), split = readFileSync(`${dir}/gather_split.wgsl`, 'utf8'), passes = readFileSync(`${ROOT}/src/render/passes.ts`, 'utf8');

  // ---- the texts
  /** a module's functions by name (an entry point under its own name), and who calls whom among them */
  const fns = (code: string) => { const m = new Map<string, string>(); for (const d of wgslDecls(code)) { const n = /^(?:@compute[^\n]*\s+)?fn (\w+)\(/.exec(d); if (n) m.set(n[1], d); } return m; };
  const calls = (m: Map<string, string>, body: string, self: string) => [...m.keys()].filter(n => n !== self && new RegExp(`\\b${n}\\(`).test(body));
  const reach = (m: Map<string, string>, from: string) => { const seen = new Set<string>([from]), todo = [from]; while (todo.length) { const f = todo.pop()!; for (const n of calls(m, m.get(f) ?? '', f)) if (!seen.has(n)) { seen.add(n); todo.push(n); } } return seen; };
  const sites = (m: Map<string, string>, within: Set<string>, callee: string) => [...within].reduce((s, f) => s + (f === callee ? 0 : (m.get(f) ?? '').split(new RegExp(`\\b${callee}\\(`)).length - 1), 0);
  { const LIT = 'directAtPoint(Ph, n, b.w)', text0 = splitGatherSrc(false), m = fns(BRK ? text0.replace(LIT, `${LIT} + ${LIT}`) : text0), ms = fns(splitGatherSrc(true)), entries = (x: Map<string, string>) => [...x].filter(([, d]) => d.startsWith('@compute')).map(([n]) => n).sort().join(' ');
    const helpers = fns(gatherHelpers());
    line(text0.includes(LIT) && entries(m) === 'light trace' && entries(ms) === 'light trace' && !wgslDecls(fgather).some(d => /^@compute\b/.test(d)) && helpers.has('cardIrradiance'),
      'the gather\'s module: fgather.wgsl\'s helpers, which hold no entry point, and two entry points of its own (the strata twin the same)', `entry points: ${entries(m)} · ${helpers.size} helper functions`);
    const tr = reach(m, 'trace'), li = reach(m, 'light');
    line(m.has('directAtPoint') && !tr.has('directAtPoint') && !tr.has('directOnScreen'),
      '`trace` reaches no light loop: neither directAtPoint nor directOnScreen, nor a helper that calls one', `trace reaches ${tr.size} functions: ${[...tr].filter(f => /X$|^gx/.test(f)).join(', ')} of its own`);
    const tracers = /\b(traceClosest|occluded\w*|glassTrace|smokeTransmittance|diceIrradiance\w*|cardIrradiance|glossGlints)\(/, lightOwn = [...li].filter(f => f !== 'light' && f !== 'directAtPoint').sort().join(' ');   // (the tracer itself is the scene prelude's, outside this text: `light`'s own body is read for a call into it)
    line(li.has('directAtPoint') && sites(m, li, 'directAtPoint') === 1 && !tracers.test(m.get('light') ?? '') && tracers.test(m.get('trace') ?? '') && lightOwn === 'gxFork rnd rngInit',
      '`light` reaches directAtPoint through ONE call site, and its own body traces nothing and reads no cache: beside the light loop it reaches the random number helpers only', `beside directAtPoint, light reaches: ${lightOwn} · directAtPoint call sites under light ${sites(m, li, 'directAtPoint')} · light reaches ${li.size} functions`); }
  { const num = (re: RegExp) => { const x = re.exec(split); return x ? Number(x[1]) : NaN; };
    const v4 = num(/^const GX_V4: u32 = (\d+)u;/m), rec = num(/^const GX_REC: u32 = (\d+)u;/m), slots = num(/^const GX_SLOTS: u32 = (\d+)u;/m), rays = Number(/^override RAYS: u32 = (\d+)u;/m.exec(fgather)?.[1]), bind = num(/^@group\(1\) @binding\((\d+)\) var<storage, read_write> gx:/m);
    const lay = /label: 'gatherSplit', entries: \[[^\]]*?\{ binding: (\d+), visibility: CS, buffer: \{ type: 'storage' \} \},\n\s*\] \}\);/.exec(passes);
    line(v4 * 16 === SPLIT_TEXEL_BYTES && rec + 2 * slots === v4 && bind === Number(lay?.[1]) && rays + 2 === slots && !/constants: \{[^}]*RAYS/.test(passes),
      'the points\' buffer is one layout in the shader and in the engine: a texel\'s words, its header and two words a point, a slot for each ray and the two mirror rays, the binding', `RAYS ${rays} + 2 = ${slots} slots · GX_V4 ${v4} × 16 B = ${v4 * 16} (engine ${SPLIT_TEXEL_BYTES}) · ${rec} + 2 × ${slots} = ${rec + 2 * slots} · binding ${bind} (passes.ts ${lay?.[1] ?? 'not found'})`); }
  // ---- THE LAST KERNEL IN TWO WRITINGS (render/gatherSplit.ts): gather_split_common.wgsl is the gather's, gather_common.wgsl the reflection passes'. They are one
  //      text but for the light loop (directAtPoint and what it is written with), so a change to the staging or the take-back made in one file only is named here
  { const A = wgslDecls(readFileSync(`${dir}/gather_split_common.wgsl`, 'utf8')), B = wgslDecls(readFileSync(`${dir}/gather_common.wgsl`, 'utf8'));
    const LOOP = /^fn (directAtPoint|directOnScreen|directAt)\(/, sq = (d: string) => d.replace(/\s+/g, ''), a = A.filter(d => !LOOP.test(d)).map(sq), b = B.filter(d => !LOOP.test(d)).map(sq);
    const onlyA = a.filter(d => !b.includes(d)), onlyB = b.filter(d => !a.includes(d)), name = (d: string) => /^(?:fn|const|var(?:<[^>]*>)?|struct)(\w+)/.exec(d)?.[1] ?? d.slice(0, 24);
    line(a.length > 10 && onlyA.length === 0 && onlyB.length === 0 && A.some(d => /^fn directAtPoint\(/.test(d)) && B.some(d => /^fn directAtPoint\(/.test(d)),
      'the gather\'s common text and the reflection passes\' are one text but for the light loop: every other declaration is in both, word for word', `${a.length} declarations beside the light loop · only the gather's: ${onlyA.map(name).join(' ') || 'none'} · only the reflections': ${onlyB.map(name).join(' ') || 'none'}`); }
  // ---- a refused buffer is heard outside the machine (a GPU too small for the buffer draws no bounce light, and did not choose that)
  { const RP = await import(`${ROOT}/src/cloud/report.ts`), heard: any[] = [], was = console.error, said: string[] = []; console.error = (...a: unknown[]) => { said.push(a.map(String).join(' ')); };
    let threw = false;
    try { RP.reportForget(); RP.reportSink((r: any) => { heard.push(r); }); splitRefused('one'); splitRefused('the grid 640×360 needs a 12 MB buffer', { grid: '640×360', MB: 12 }); splitRefused('the grid 1280×720 needs a 48 MB buffer', { grid: '1280×720', MB: 48 });
      RP.reportSink(() => { throw new Error('the listener broke'); }); splitRefused('three'); }
    catch { threw = true; } finally { console.error = was; RP.reportForget(); }
    const main = readFileSync(`${ROOT}/src/main.ts`, 'utf8'), own = readFileSync(`${ROOT}/src/render/gatherSplit.ts`, 'utf8'), refuse = /refuse = \(why: string\) => \{[^\n]*\n\s*splitRefused\(`The gather's points buffer: the gather grid [^\n]*the bounce light is not drawn[^\n]*,\n\s*\{ grid: [^\n]*MB[^\n]*why \}\);/.test(passes);
    line(!threw && said.join() === 'one,the grid 640×360 needs a 12 MB buffer,the grid 1280×720 needs a 48 MB buffer,three' && heard.length === 3 && heard[1] === heard[2] && heard[1].where === 'gpu.splitrays.buffer' && heard[1].data?.grid === '640×360' && refuse && !/console\.error\(`The gather's points buffer/.test(passes) && !/SPLIT_SAID/.test(main) && /reportError\('gpu\.splitrays\.buffer', line, data\)/.test(own),
      'a refused points\' buffer is said on the console word for word and to the one reporter (src/cloud/report.ts) under its own place, the grid and the size beside it; the same refusal at another window size is the report it already was (one error for a machine), a listener that throws loses nothing, passes.ts refuses through it, and main.ts has no hook of its own for it',
      `console: ${said.join(' · ')} · reports: ${heard.length} (${new Set(heard).size} made) · passes.ts ${refuse ? 'refuses through it' : 'does NOT'}`); }

  // ---- what a load compiles (the size tool, a run of its own)
  const tmp = mkdtempSync(join(tmpdir(), 'splitrays-'));
  try {
    const f = join(tmp, 'page.json'), r = Bun.spawn(['bun', 'run', `${ROOT}/tools/qa/shaderinline-check.ts`, '--json', f], { env: { ...process.env, SHADERINLINE_BREAK: undefined }, stdout: 'pipe', stderr: 'ignore' }), text = await new Response(r.stdout).text(), green = await r.exited === 0;
    let j: Sizes | null = null; try { j = JSON.parse(readFileSync(f, 'utf8')) as Sizes; } catch { /* no file: the line says so */ }
    const KB = (n: number) => `${Math.round(n / 1024)} KB`, gathers = (re: RegExp) => (j?.kernels ?? []).filter(k => re.test(k.module) && k.setting !== 'title');   // (the size tool's file carries the title's kernels after the page's own: one page is counted here)
    const old = gathers(/^fgatherS?$/), own = gathers(/^gatherSplit/), pipes = (set: string) => own.filter(k => k.set === set).flatMap(k => k.pipelines).sort().join(' ');
    line(old.length === 0 && !(j?.modules ?? []).some(m => /^fgather/.test(m.label)) && pipes('boot') === 'gatherLight gatherTrace gatherTraceS' && pipes('cut') === pipes('boot'),
      'a load compiles the gather\'s three pipelines in both sets and no one-kernel gather', `one-kernel gathers ${old.length} · boot set: ${pipes('boot') || 'none'} · cut set: ${pipes('cut') || 'none'}`);
    line(own.length > 0 && green && own.every(k => k.budgetKB !== null && k.budgetKB < 300),
      'each of the gather\'s kernels has a budget row of its own under 300 KB, and the size tool is green', `as their rows hold them, the cut test's text apart: ${[...new Set(own.map(k => `${k.entry} ${KB(k.pasted - (k.cut ?? 0))} of ${k.budgetKB}`))].join(' · ')} · size tool: ${text.trim().split('\n').slice(-1)[0] ?? ''}`);
  } finally { rmSync(tmp, { recursive: true, force: true }); }
  return all;
}

if (import.meta.main) { const ok = await splitraysCheck(); console.log(ok ? '\n=== splitrays-check: ALL PASS ===' : '\n=== splitrays-check: FAILED ==='); process.exit(ok ? 0 : 1); }
