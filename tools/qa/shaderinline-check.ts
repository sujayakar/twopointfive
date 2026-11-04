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
// a pipeline for are listed, not sized into the totals), and the switches a URL flag or a trial turns. The fold of constant switches is on, as on the page: a
// branch a constant decides is not in the text, so a kernel's size follows the switches. The listing is the shipped stand's page on a snow level in its
// weather (cut boxes, the glass fracture and the dust twin on, as main.ts puts them: tools/qa/pageboot.ts), and the budget is held to that page AND to the
// title's, which a second run of this file sizes. What a word on the address would add (a trial, `?look`) is in neither: `--setting look` lists that page.
//   bun run tools/qa/shaderinline-check.ts [--top N] [--kernel <module>/<entry>] [--json out.json]      exit 0 = within the budget, 1 = not, or the boot did not finish
//   bun run tools/qa/shaderinline-check.ts --setting title|snow|look                                    one page alone, listed and held to the table (pageboot.ts SETTINGS)
//   TPS_COMPACTSHADERS=1 bun run tools/qa/shaderinline-check.ts   the same for the compact set (the row 'Compact shaders', src/ui/shaderrows.ts), against its own table
//   SHADERINLINE_BREAK=1 bun run tools/qa/shaderinline-check.ts     one more call site is written into the biggest kernel, of the lightest function it reaches
//                                                                    whose call site is a twentieth of it: must FAIL (it does not while that
//                                                                    kernel's row has a twentieth to spare: the plain run names the row)
//   TPS_SPLITRAYS=1 bun run tools/qa/shaderinline-check.ts        the same with the row 'Separate shadow rays' on: the gather's two programs, held to their own rows
// --kernel prints every function pasted into that one kernel (copies, own size, the share, what one call site of it costs); --json writes the rows, and each module's text hash as
// tools/qa/shader-count-page.cjs prints it (the real page's modules can be held against these).
import { readFileSync, unlinkSync } from 'node:fs';
import { tmpdir } from 'node:os';
process.env.TPS_PROFILE = 'shipped';   // (the page's shaders: the frozen stand holds switches off that are consts in the scene prelude — other texts, other sizes)
const { ROOT } = await import('./headless.ts');
const { pageBoot, settingArg, SETTINGS } = await import('./pageboot.ts');
type Stage = import('./pageboot.ts').Stage; type Mod = import('./pageboot.ts').Mod;
const { wgslDecls } = await import(`${ROOT}/src/gpu/wgslTrim.ts`) as typeof import('../../src/gpu/wgslTrim');
const { COMPACT } = await import(`${ROOT}/src/render/kernels.ts`) as typeof import('../../src/render/kernels');   // (the row 'Compact shaders': TPS_COMPACTSHADERS=1 in front sizes that set, against its own table below)

/** KB of pasted WGSL a kernel may not pass, by `module / entry` (a cut-set module's `Cut` off its label), the first row that matches; the last is everyone else's */
const BUDGET_TODAY: [RegExp, number][] = [
  [/^composite(R|S|SR)? \/ (fs|cs)$/, 662],
  [/^fgatherS? \/ mainGi$/, 820],
  [/^fgatherS? \/ main$/, 759],
  [/^volumetricsDust \/ main$/, 718],
  [/^volumetrics \/ main$/, 667],
  [/^reflectP5 \/ main$/, 640],
  [/^particles \/ update$/, 577],
  [/^direct \/ main$/, 399],
  [/^restirM? \/ main$/, 343],
  [/^reflect \/ main$/, 274],
  [/^froxel \/ injectJ$/, 216],
  [/^sharpUE? \/ main$/, 237],
  [/^gbuffer \/ fsBox$/, 302],
  [/./, 200],
];
/** the same for THE COMPACT SET (the row 'Compact shaders' on: render/kernels.ts, the ten kernels written with one call site per heavy function), its own rows within a
 *  twentieth of what that set is. A second table, never today's rows lowered: both writings are in the tree while the row is on trial, and each is held to its own size */
const BUDGET_COMPACT: [RegExp, number][] = [
  [/^composite(R|S|SR)? \/ (fs|cs)$/, 403],
  [/^volumetricsDust \/ main$/, 433],
  [/^volumetricsF? \/ main$/, 382],
  [/^particles \/ update$/, 365],
  [/^restirM? \/ main$/, 304],
  [/^fgatherS? \/ mainGi$/, 299],
  [/^direct \/ main$/, 256],
  [/^reflectP5 \/ main$/, 253],
  [/^fgatherS? \/ main$/, 230],
  [/^froxel \/ injectJ$/, 216],
  [/^reflect \/ main$/, 204],
  [/^gbuffer \/ fsBox$/, 302],
  [/./, 200],
];
const BUDGET = COMPACT.on ? BUDGET_COMPACT : BUDGET_TODAY;
const BREAK = process.env.SHADERINLINE_BREAK === '1';
const argv = process.argv.slice(2);
const opt = (k: string): string | null => { const i = argv.indexOf('--' + k); if (i < 0) return null; const v = argv[i + 1]; if (v === undefined || v.startsWith('--')) { console.error(`--${k} needs a value`); process.exit(1); } return v; };
const TOP = ((v: string | null) => { if (v === null) return Infinity; if (!/^[1-9]\d*$/.test(v)) { console.error(`--top needs a count, not '${v}'`); process.exit(1); } return Number(v); })(opt('top'));
const KERNEL = opt('kernel'), JSON_OUT = opt('json');
const say = console.log.bind(console);   // (the boot below is run with the console's own lines held back: a check's line is not one of those)
// SEPARATE SHADOW RAYS (TPS_SPLITRAYS=1 in front: the row 'Separate shadow rays' on, src/render/gatherSplit.ts). The final gather is then two programs with rows of their own,
// each within a twentieth of what it is, and the four megakernel gathers are not made: their rows go out with them for that run. With the row off this is the table above, untouched.
{ const { SPLIT_RAYS } = await import(`${ROOT}/src/render/gatherSplit.ts`) as typeof import('../../src/render/gatherSplit');
  if (SPLIT_RAYS.on) BUDGET.splice(0, BUDGET.length, [/^gatherSplitS? \/ trace$/, 252], [/^gatherSplit \/ light$/, 119], ...BUDGET.filter(([re]) => !re.source.startsWith('^fgatherS?'))); }
let red = 0; const check = (ok: boolean, what: string) => { say(`${ok ? '- PASS' : '- FAIL'} — ${what}`); if (!ok) red++; };

// ---- the page's boot over a recording stand-in (tools/qa/pageboot.ts), under one setting of the switches
// Since the fold of constant switches (gpu/wgslTrim.ts SHADER_FOLD) a kernel's size follows the switches, so the rows bound every page a player opens with no
// word on the address: the listing below is a snow level in its weather (the heaviest: the march, the particles and the g-buffer are up to a third bigger
// there), and the budget is also held to the title's kernels, which a second run of this file sizes (`--setting title`; an arm a level's switch turns OFF is
// only in that page's text).
const SETTING = settingArg(argv) ?? 'snow';
const { mods, asked, faults, cutNote } = await pageBoot(SETTING);
for (const f of faults) check(false, f);
check(asked.some(a => a.set === 'cut'), `the cut-box second set was built (${cutNote})`);

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
const rowOfName = (module: string, entry: string) => BUDGET.findIndex(([re]) => re.test(`${module.replace(/Cut$/, '')} / ${entry}`)), rowOf = (r: Row) => rowOfName(r.module, r.entry), REST = BUDGET.length - 1;
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
// ---- the other page: the title's kernels, sized by a second run of this file and held to the same rows (not under --setting, --kernel or the break: those read one page)
interface Sized { name: string; row: number; pasted: number; where: string }
let title: { module: string; entry: string; stage: Stage; set: string; pasted: number; once: number }[] = [];
if (settingArg(argv) === null && !BREAK && KERNEL === null) {
  const tmp = `${tmpdir()}/shaderinline-title-${process.pid}.json`;
  Bun.spawnSync(['bun', 'run', import.meta.path, '--setting', 'title', '--top', '1', '--json', tmp], { env: process.env as Record<string, string>, stdout: 'ignore', stderr: 'ignore' });
  try { title = JSON.parse(readFileSync(tmp, 'utf8')).kernels; } catch { /* the line below says so */ }
  try { unlinkSync(tmp); } catch { /* never written */ }
  check(title.length >= list.length / 2, `${SETTINGS.title} was sized too, by a second run of this file (${title.length} kernels; ${list.length} here, on ${SETTINGS[SETTING]})`);
}
const sized: Sized[] = [...list.map(r => ({ name: nameOf(r), row: rowOf(r), pasted: r.pasted, where: '' })), ...title.map(k => ({ name: `${k.module} / ${k.entry}`, row: rowOfName(k.module, k.entry), pasted: k.pasted, where: ` at ${SETTINGS.title}` }))].sort((x, y) => y.pasted - x.pasted);
const over = sized.filter(k => k.pasted > BUDGET[k.row][1] * 1024);
// a row of the table by the biggest kernel that answers to it on either page: none = the row is stale, a twentieth under = it has room a new call site would hide in
const rowsBig = BUDGET.slice(0, REST).map(([re, cap], i) => ({ re, cap, big: sized.find(k => k.row === i) }));
const stale = rowsBig.filter(x => !x.big), spare = rowsBig.filter(x => x.big && x.big.pasted < x.cap * 1024 * 0.95);
check(sized.length > 0 && over.length === 0, `every kernel is within its budget${title.length ? `, on ${SETTINGS[SETTING]} and at ${SETTINGS.title}` : `, on ${SETTINGS[SETTING]}`} (the biggest: ${sized.length ? `${sized[0].name}, ${kb(sized[0].pasted)} of ${BUDGET[sized[0].row][1]} KB` : 'none recorded'}; a kernel with no row of its own: ${BUDGET[REST][1]} KB)${over.length ? ` — not so: ${[...new Set(over.map(k => `${k.name} ${kb(k.pasted)} of ${BUDGET[k.row][1]} KB${k.where}`))].slice(0, 8).join(', ')} (take the call site out, or raise the kernel's row in this file on purpose; a new big kernel gets a row of its own)` : ''}`);
check(stale.length === 0, `every row of the budget table has a kernel that answers to it${stale.length ? ` — not so: ${stale.map(x => String(x.re)).join(', ')} (a kernel renamed or gone: its row goes with it)` : ''}`);
if (spare.length) console.log(`  ${spare.length} rows with a twentieth to spare, by their biggest kernel (lower each to it, or take the row out where it is under ${BUDGET[REST][1]} KB): ${spare.map(x => `${x.big!.name} ${kb(x.big!.pasted)} of ${x.cap}${x.big!.where}`).join(', ')}`);
if (JSON_OUT) {
  const hash = (s: string) => { let a = 0x811c9dc5, b = 0x1b873593; for (let i = 0; i < s.length; i++) { const c = s.charCodeAt(i); a = Math.imul(a ^ c, 16777619); b = Math.imul(b ^ c, 0x85ebca6b) + (b >>> 13); } return (a >>> 0).toString(36) + (b >>> 0).toString(36); };   // (shader-count-page.cjs's own)
  await Bun.write(JSON_OUT, JSON.stringify({ modules: mods.map(m => ({ label: m.label, h: hash(m.code), chars: m.code.length })),
    kernels: [...list.map(r => ({ setting: SETTING, module: r.module, entry: r.entry, stage: r.stage, set: r.set, pipelines: r.pipelines, pasted: r.pasted, once: r.once, budgetKB: budgetOf(r), most: most(r).slice(0, 8) })),
      ...title.map(k => ({ ...k, setting: 'title' }))] }, null, 1));   // (the listing's page first, then the title's kernels where this run sized them)
}
console.log(red ? `${red} FAILED` : 'all passed');
process.exit(red ? 1 : 0);
