// THE PAGE'S SHADER TEXTS AND THE PROGRAMS IT ASKS FOR, HELD WITH NO BROWSER AND NO COMPILER (that a text COMPILES is tools/qa/wgsl-compile.cjs's, what it draws the page's).
//   bun run tools/qa/shaders-check.ts [section …]        no word: every section. exit 0 = all passed, 1 = one failed, 2 = no such section
//     parse     every TypeScript file under tools/ parses                                  [--root <dir>]
//     trim      a module's text cut to what it uses (gpu/wgslTrim.ts SHADER_TRIM)          SHADERTRIM_BREAK=1: a declaration something still mentions is taken out of one cut text
//     fold      a module's text cut to what its constant switches leave on (SHADER_FOLD)   SHADERFOLD_BREAK=1: the fold is handed every literal constant with its value turned over: forms and boot fail
//     inline    how big each kernel is once every call is pasted into its caller           [--top N] [--kernel <module>/<entry>] [--json f] [--setting title|snow|nowalls]
//                                                                                          SHADERINLINE_BREAK=1: one more call site in the biggest kernel, of the lightest function it reaches whose call site is a twentieth of it
//     levelset  every level asks for one set of programs                                   LEVELSET_BREAK=1: the snow level's boot flips one constant of the prelude and one cast's module carries a size
//     rows      the first-frame start (src/ui/shaderrows.ts)                               [--dump f]   SHADERROWS_BREAK=1: each arm is handed the other arm's row, and the store cases an environment that says off
// ONE BOOT FOR EACH SETTING. pageboot.ts allows one boot a process, so a section asks for a setting's reading (`reading` below: a process of its own, every module as written) and
// every section of this run reads that one: fold reads it whole, inline and levelset as the page hands it over (`cut`: makeShader's step is the only one between the two).
// trim's and rows' boots are of another thing (the Engine alone, a device that holds each creation): each runs in a process of its own. A red line starts with its section's name.
export {};
import { mkdtempSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { characterSet, fake } from './here.ts';
import { pageBoot, settingArg, settingsHere, SETTINGS, type Asked, type Mod, type Stage } from './pageboot.ts';
const ROOT = resolve(import.meta.dir, '../..'), SECTIONS = ['parse', 'trim', 'fold', 'inline', 'levelset', 'rows'] as const;
type Section = typeof SECTIONS[number];
const argv = process.argv.slice(2);
const argOf = (k: string): string | null => { const i = argv.indexOf(k); return i < 0 ? null : argv[i + 1] ?? ''; };
let red = 0, section: Section = 'parse';
const check = (ok: boolean, what: string, more = ''): void => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${section}: ${what}${more ? `\n    ${more}` : ''}`); if (!ok) red++; };
const sha = (s: string): string => new Bun.CryptoHasher('sha256').update(s).digest('hex').slice(0, 16);
const tmp = mkdtempSync(join(tmpdir(), 'shaders-')); process.on('exit', () => rmSync(tmp, { recursive: true, force: true }));
/** this file run again for one thing: its lines passed on and counted */
function child(title: string, args: string[], env: Record<string, string | undefined>): void {
  const p = Bun.spawnSync(['bun', 'run', import.meta.path, ...args], { env, stdout: 'pipe', stderr: 'pipe' }), text = p.stdout.toString(); process.stdout.write(text);
  const fails = (text.match(/^- FAIL/gm) ?? []).length, passes = (text.match(/^- PASS/gm) ?? []).length;
  if (p.exitCode !== 0 && !fails) check(false, `${title} ran to its end (exit ${p.exitCode}: ${p.stderr.toString().trim().split('\n').slice(-2).join(' | ')})`);
  else if (!fails && !passes) check(false, `${title} said nothing`);
  red += fails;
}

// ---- one setting's reading: the page's boot (tools/qa/pageboot.ts) under its switches, every module as written
interface Reading { setting: string; mods: Mod[]; asked: Asked[]; faults: string[]; /** frame.ts levelSwitches() after the boot */ word: number; bits: Record<string, number>; prelude: string }
const readings = new Map<string, Reading | null>();
/** `firstFrame`: the shader rows as the page opens (the row is read at import, so it is another boot) */
function reading(setting: string, firstFrame = false): Reading | null {
  const key = `${setting}${firstFrame ? '+first' : ''}`, have = readings.get(key); if (have !== undefined) return have;
  const out = join(tmp, `${key}.json`), p = Bun.spawnSync(['bun', 'run', import.meta.path, '--boot', setting, '--out', out], { env: { ...process.env, ...(firstFrame ? { TPS_FIRSTFRAME: '1' } : {}) }, stdout: 'pipe', stderr: 'pipe' });
  let r: Reading | null = null; try { if (p.exitCode === 0) r = JSON.parse(readFileSync(out, 'utf8')) as Reading; } catch { /* null below */ }
  if (!r) console.error(`the boot of ${key}: ${p.stderr.toString().slice(-600)}`);
  readings.set(key, r); return r;
}
async function bootChild(setting: string, out: string): Promise<void> {
  const FR = await import(`${ROOT}/src/render/frame.ts`) as Record<string, any>;
  const { SHADER_TRIM } = await import(`${ROOT}/src/gpu/wgslTrim.ts`) as typeof import('../../src/gpu/wgslTrim');
  if (process.env.LEVELSET_BREAK === '1' && setting === 'snow') FR.rainScaleConfig.on = !FR.rainScaleConfig.on;   // (the red path: a level that still raises a constant of the text)
  SHADER_TRIM.on = false; const B = await pageBoot(setting);
  writeFileSync(out, JSON.stringify({ ...B, word: FR.levelSwitches(), bits: { ...FR.LevelSw }, prelude: sha(FR.scenePrelude()) } satisfies Reading));
}
/** a module as the page hands it to the browser, trim and fold on (gpu/device.ts makeShader) */
const cutTexts = new Map<string, string>();
const cut = async (code: string): Promise<string> => { let t = cutTexts.get(code); if (t === undefined) { const { trimWgsl } = await import(`${ROOT}/src/gpu/wgslTrim.ts`) as typeof import('../../src/gpu/wgslTrim'); cutTexts.set(code, t = trimWgsl(code, true)); } return t; };

// ======== parse: EVERY TypeScript FILE UNDER tools/ PARSES. `tsconfig.json` covers src/ and tools/qa/tsconfig.json covers tools/qa; tools/blitz, tools/wire, tools/mp,
// tools/fold and the rest are in neither, so a file there that no gate tool happens to import can be broken and nobody is told (a name imported twice
// stopped tools/blitz/mission.ts once, and every kept run with it). This reads each such file through the transpiler, which refuses what the runtime
// would refuse at import: a syntax error, a name declared twice, an import of a name twice. It checks no types.
function parseSection(): void {
  const root = resolve(argOf('--root') ?? ROOT), T = new Bun.Transpiler({ loader: 'ts' });
  const files: string[] = [], walk = (d: string) => { for (const n of readdirSync(d)) { if (n === 'node_modules' || n === 'target' || n.startsWith('.')) continue; const p = join(d, n); if (statSync(p).isDirectory()) walk(p); else if (/\.(ts|mts|cts)$/.test(n) && !n.endsWith('.d.ts')) files.push(p); } };
  walk(join(root, 'tools'));
  let bad = 0;
  for (const f of files.sort()) { try { T.transformSync(readFileSync(f, 'utf8')); } catch (e: any) { bad++; check(false, `${f.slice(root.length + 1)}: ${String(e?.errors?.[0]?.message ?? e?.message ?? e).split('\n')[0]}`); } }
  if (!bad) check(true, `${files.length} files under tools/ parse`);
}

// ======== trim
// A MODULE'S TEXT CUT TO WHAT IT USES (src/gpu/wgslTrim.ts SHADER_TRIM): the rule held to the shaders the engine boots with. The Engine is built on a stand-in
// device that records every module's text, once whole (?shadertrim=0) and once cut, and for each module:
//   · every declaration left is one of the whole text's, comments aside, in the same order
//   · every entry point, override, directive and const_assert of the whole text is left
//   · nothing left mentions a declaration that was dropped (the cut is closed under 'mentions')
//   · cutting the cut text changes nothing
// then the two numbers the cut is for: how much text a boot hands the browser, and how many modules' texts change when one constant of the prelude flips (each
// one is a text the browser's compiled-shader cache has never seen) — and that a LEVEL's own switches change none: they are data (render/frame.ts LevelSw).
// No browser and no compiler: that the cut text COMPILES is the page's boot to say (tools/qa/shader-count-page.cjs — every pipeline made, no GPU error).
async function trimSection(): Promise<void> {
  const { Engine } = await import(`${ROOT}/src/engine.ts`);
  const { SHADER_TRIM, SHADER_FOLD, trimWgsl, wgslDecls, trimTotals } = await import(`${ROOT}/src/gpu/wgslTrim.ts`) as typeof import('../../src/gpu/wgslTrim');
  SHADER_FOLD.on = false;   // (the trim's own rule, a declaration kept as written: the fold of constant switches changes a function's text and has its own section, fold)
  const { weatherKindConfig, rainScaleConfig, levelSwitches, LevelSw } = await import(`${ROOT}/src/render/frame.ts`);

    const BREAK = process.env.SHADERTRIM_BREAK === '1';
  /** the texts one boot of the Engine hands createShaderModule, by label in the order made */
  async function boot(): Promise<{ label: string; code: string }[]> {
    const mods: { label: string; code: string }[] = [];
    const pipe = () => new Proxy({}, { get: (_t, k) => k === 'then' ? undefined : fake(String(k)) });
    const base = fake('device');
    const device = new Proxy(base, { get(t, k) {
      if (k === 'createShaderModule') return (d: { code: string; label?: string }) => { mods.push({ label: d.label ?? '', code: d.code }); return { getCompilationInfo: async () => ({ messages: [] }) }; };
      if (k === 'createComputePipelineAsync' || k === 'createRenderPipelineAsync') return async () => pipe();
      if (k === 'limits') return new Proxy({}, { get: () => 1 << 20 });
      if (k === 'then') return undefined;
      return t[k]; } });
    const e = new Engine({ device, adapterInfo: 'stand-in', canvas: fake('canvas'), ctx: fake('ctx'), format: 'bgra8unorm', hasTimestamps: false, linearSampler: fake('sampler') } as never);
    await Promise.race([e.init({ sharp: true, gbaa: true }), new Promise(r => setTimeout(r, 2000))]);
    return mods;
  }
  const WORD = /[A-Za-z_][A-Za-z0-9_]*/g;
  const nameOf = (d: string): string | null => { const m = /^(?:\s*@[A-Za-z_]\w*(?:\s*\((?:[^()]|\([^()]*\))*\))?)*\s*(fn|struct|alias|const|override|var)\b\s*(?:<[^>]*>)?\s*([A-Za-z_]\w*)/.exec(d); return m ? m[2] : null; };
  const isRoot = (d: string) => nameOf(d) === null || /^(?:\s*@[A-Za-z_]\w*(?:\s*\((?:[^()]|\([^()]*\))*\))?)*\s*override\b/.test(d) || /@(compute|vertex|fragment)\b/.test(d.slice(0, d.indexOf('fn ') < 0 ? 0 : d.indexOf('fn ')));

  SHADER_TRIM.on = false; const whole = await boot();
  SHADER_TRIM.on = true; trimTotals.modules = trimTotals.before = trimTotals.after = trimTotals.ms = 0; const cut = await boot(); const totals = { ...trimTotals };
  check(whole.length >= 25 && whole.length === cut.length && whole.every((m, i) => m.label === cut[i].label), `the same ${whole.length} modules either way (${cut.length} cut)`);
  let entries = 0; const bad = { order: [] as string[], roots: [] as string[], open: [] as string[], twice: [] as string[] };
  cut.forEach((m, i) => {
    const W = wgslDecls(whole[i].code); let C = wgslDecls(m.code);
    if (BREAK && i === 3) { const names = new Set(C.map(nameOf)); const k = C.findIndex(d => { const n = nameOf(d); return n !== null && !isRoot(d) && C.some(o => o !== d && ((o.match(WORD) ?? []) as string[]).includes(n)) && names.has(n); }); C = C.filter((_, j) => j !== k); }
    // in the whole text's order, each one of its declarations
    let at = 0; const inOrder = C.every(d => { const k = W.indexOf(d, at); if (k < 0) return false; at = k + 1; return true; }); if (!inOrder) bad.order.push(m.label);
    const kept = new Set(C); const roots = W.filter(isRoot); entries += roots.filter(d => /@(compute|vertex|fragment)\b/.test(d)).length;
    if (!roots.every(d => kept.has(d))) bad.roots.push(m.label);
    const have = new Set(C.map(nameOf)); const all = new Set(W.map(nameOf)); all.delete(null);
    if (C.some(d => (d.match(WORD) ?? []).some(w => all.has(w) && !have.has(w)))) bad.open.push(m.label);
    if (trimWgsl(m.code) !== m.code) bad.twice.push(m.label);
  });
  check(bad.order.length === 0, `every declaration left is the whole text's own, in its order${bad.order.length ? ` — not so: ${bad.order.slice(0, 5).join(', ')}` : ''}`);
  check(bad.roots.length === 0 && entries >= 60, `every entry point, override, directive and const_assert is left (${entries} entry points)${bad.roots.length ? ` — not so: ${bad.roots.slice(0, 5).join(', ')}` : ''}`);
  check(bad.open.length === 0, `nothing left mentions a declaration that was dropped${bad.open.length ? ` — not so: ${bad.open.slice(0, 5).join(', ')}` : ''}`);
  check(bad.twice.length === 0, `cutting a cut text changes nothing${bad.twice.length ? ` — not so: ${bad.twice.slice(0, 5).join(', ')}` : ''}`);
  const kb = (n: number) => Math.round(n / 1024);
  check(totals.after < totals.before * 0.4, `a boot of the Engine hands the browser ${kb(totals.after)} KB of shader text where it handed ${kb(totals.before)} KB (${Math.round(100 - 100 * totals.after / totals.before)} % less; ${totals.ms.toFixed(0)} ms spent cutting ${totals.modules} modules here)`);
  // a constant of the prelude that differs (a word on the address: the rain's ring scale): how many module texts are new
  const flipOf = async (cfg: { on: boolean }, on: boolean) => { SHADER_TRIM.on = on; const was = cfg.on; cfg.on = false; const a = await boot(); cfg.on = true; const b = await boot(); cfg.on = was; return a.filter((m, i) => m.code !== b[i].code).length; };
  const newWhole = await flipOf(rainScaleConfig, false), newCut = await flipOf(rainScaleConfig, true);
  check(newWhole === whole.filter(m => /\bRAIN_SCALE\b/.test(m.code)).length && newCut < newWhole / 2, `a page whose prelude differs by one constant: ${newCut} of ${cut.length} module texts are new to the browser where ${newWhole} were`);
  // …and a LEVEL's switches are none of the text (render/frame.ts LevelSw: a word of the frame block): a level in another weather is the same modules, cut or whole
  const lvWhole = await flipOf(weatherKindConfig, false), lvCut = await flipOf(weatherKindConfig, true), words = (() => { const was = weatherKindConfig.on; weatherKindConfig.on = false; const a = levelSwitches(); weatherKindConfig.on = true; const b = levelSwitches(); weatherKindConfig.on = was; return [a, b]; })();
  check(lvWhole === 0 && lvCut === 0 && (words[0] & LevelSw.WX_KINDS) === 0 && (words[1] & LevelSw.WX_KINDS) !== 0, `a level whose weather switch differs: ${lvCut} of ${cut.length} module texts are new to the browser (${lvWhole} uncut); the switch is a bit of the frame's level word`);
}

// ======== fold
// A MODULE'S TEXT CUT TO WHAT ITS CONSTANT SWITCHES LEAVE ON (src/gpu/wgslTrim.ts SHADER_FOLD, foldWgsl). The scene prelude's switches are constants of the
// text (`const WX_KINDS: bool = false;`) and the kernels branch on them; the fold cuts each such branch to the side that runs before the trim reads what is
// reachable, so a trial that is off on a page is not written into its programs. Held here, with no browser and no compiler:
//   forms    small texts of every form the fold reads (a plain branch, a negated one, `NAME && …`, `A || B`, a constant written as a test of constants, an
//            else, an else-if chain with the constant first, in the middle or last, a branch inside the kept side, what follows a block that cannot be
//            passed, a discard, a loop's continuing block, a test without its parentheses) come out as written by hand; what it must NOT touch stays as written (`x && NAME`, a name inside a comparison or a call, a function
//            with a parameter or a local of the constant's name, a constant that is no bool, a `select`, an `if` with an attribute in front)
//   boot     the shaders the PAGE boots with (tools/qa/pageboot.ts: the Engine, the smoke solver, the Game's own modules and the cut-box set, with the
//            switches host/drawpage.ts raises), under each setting of the switches (the title, a snow level in its weather, and the look prototype's page where
//            the tree has one), folded: no test is left that the constants decide; the dead side of every plain
//            `if (NAME)` / `if (!NAME)` of the unfolded text is gone from the folded one, and the constant that decided it is still there (both read off the
//            unfolded text here, not by the fold); no
//            statement is left after a return, break or continue in its block; folding twice cuts no branch the first time left; every entry point, override and directive
//            is kept; nothing left mentions a declaration that was dropped; no module is bigger
//   size     the text a boot hands the browser, with and without (pasted call site by call site it is the inline section's)
//   switch   with SHADER_FOLD down the texts are the trim's alone, character for character; the page lowers it for ?shaderfold=0
// That the folded text COMPILES with no warning is tools/qa/wgsl-compile.cjs's to say.
async function foldSection(): Promise<void> {
  const { SHADER_TRIM, SHADER_FOLD, trimWgsl, foldWgsl, wgslDecls, boolConsts, shaderText } = await import(`${ROOT}/src/gpu/wgslTrim.ts`) as typeof import('../../src/gpu/wgslTrim');
  const BREAK = process.env.SHADERFOLD_BREAK === '1';
  const squeeze = (s: string) => s.replace(/\s+/g, ' ').trim();
  /** the break path: every bool constant's value turned over before the fold reads it (the wrong side is then the one kept) */
  const turned = (code: string) => code.replace(/(const\s+\w+\s*:\s*bool\s*=\s*)(true|false)/g, (_m, a, v) => `${a}${v === 'true' ? 'false' : 'true'}`);
  const fold = (code: string) => { if (!BREAK) return foldWgsl(code); const t = foldWgsl(turned(code)); return turned(t); };   // (…and the constants' lines put back as they were)
  // ---- forms
  { const K = 'const ON: bool = true;\nconst OFF: bool = false;\nconst OFF2: bool = false;\nconst BOTH: bool = ON && !OFF;\nconst NONE: bool = OFF || !ON;\nconst N: u32 = 3u;\n';
  const cases: [string, string, string][] = [
    ['a branch that is off goes', 'fn f() -> f32 { var a = 1.0; if (OFF) { a = heavy(a); } return a; }', 'fn f() -> f32 { var a = 1.0; return a; }'],
    ['a branch that is on stays, as a block', 'fn f() -> f32 { var a = 1.0; if (ON) { a = 2.0; } return a; }', 'fn f() -> f32 { var a = 1.0; { a = 2.0; } return a; }'],
    ['a negated one', 'fn f() -> f32 { var a = 1.0; if (!OFF) { a = 2.0; } if (!ON) { a = 3.0; } return a; }', 'fn f() -> f32 { var a = 1.0; { a = 2.0; } return a; }'],
    ['off, with an else: the else', 'fn f() -> f32 { if (OFF) { return heavy(1.0); } else { return 2.0; } }', 'fn f() -> f32 { { return 2.0; } }'],
    ['on, with an else: the first side', 'fn f() -> f32 { if (ON) { return 1.0; } else { return heavy(2.0); } }', 'fn f() -> f32 { { return 1.0; } }'],
    ['off && anything goes, its else stays', 'fn f(x: f32) -> f32 { if (OFF && x > heavy(x)) { return 1.0; } else { return 2.0; } }', 'fn f(x: f32) -> f32 { { return 2.0; } }'],
    ['!on && anything goes', 'fn f(x: f32) -> f32 { var a = x; if (!ON && (x > 0.0 || heavy(x) > 1.0)) { a = 0.0; } return a; }', 'fn f(x: f32) -> f32 { var a = x; return a; }'],
    ['on || anything stays', 'fn f(x: f32) -> f32 { var a = x; if (ON || heavy(x) > 1.0) { a = 0.0; } return a; }', 'fn f(x: f32) -> f32 { var a = x; { a = 0.0; } return a; }'],
    ['two that are off, or-ed', 'fn f(x: f32) -> f32 { var a = x; if (OFF || OFF2) { a = heavy(a); } if ((OFF) || (!ON && x > 0.0)) { a = heavy(a); } return a; }', 'fn f(x: f32) -> f32 { var a = x; return a; }'],
    ['a constant written as a test of constants', 'fn f() -> f32 { var a = 1.0; if (BOTH) { a = 2.0; } if (NONE) { a = heavy(a); } if (!BOTH || NONE) { a = heavy(a); } return a; }', 'fn f() -> f32 { var a = 1.0; { a = 2.0; } return a; }'],
    ['off, then an else-if chain: the chain stands as its own statement', 'fn f(x: f32) -> f32 { if (OFF) { return 0.0; } else if (x > 1.0) { return 1.0; } else if (x > 0.5) { return 0.5; } else { return x; } }', 'fn f(x: f32) -> f32 { if (x > 1.0) { return 1.0; } else if (x > 0.5) { return 0.5; } else { return x; } }'],
    ['on, then an else-if chain: the chain goes whole', 'fn f(x: f32) -> f32 { if (ON) { return 0.0; } else if (x > 1.0) { return heavy(x); } else { return x; } }', 'fn f(x: f32) -> f32 { { return 0.0; } }'],
    ['an else if that is off, in the middle of a chain', 'fn f(x: f32) -> f32 { var a = 0.0; if (x > 1.0) { a = 1.0; } else if (OFF) { a = heavy(x); } else if (x > 0.5) { a = 0.5; } return a; }', 'fn f(x: f32) -> f32 { var a = 0.0; if (x > 1.0) { a = 1.0; } else if (x > 0.5) { a = 0.5; } return a; }'],
    ['an else if that is on is the else, and what stood after it goes', 'fn f(x: f32) -> f32 { var a = 0.0; if (x > 1.0) { a = 1.0; } else if (ON) { a = 2.0; } else if (x > 0.5) { a = heavy(x); } else { a = heavy(a); } return a; }', 'fn f(x: f32) -> f32 { var a = 0.0; if (x > 1.0) { a = 1.0; } else { a = 2.0; } return a; }'],
    ['an else if on its own line, far in', 'fn f(x: f32) -> f32 { var a = 0.0; if (x > 0.0) { a = 1.0; } else\n               if (OFF) { a = 2.0; }\n if (x < -1.0) { a = 3.0; } return a; }', 'fn f(x: f32) -> f32 { var a = 0.0; if (x > 0.0) { a = 1.0; } if (x < -1.0) { a = 3.0; } return a; }'],
    ['a branch inside the kept side is read too', 'fn f() -> f32 { var a = 1.0; if (ON) { a = 2.0; if (OFF) { a = heavy(a); } else { a = 4.0; } } return a; }', 'fn f() -> f32 { var a = 1.0; { a = 2.0; { a = 4.0; } } return a; }'],
    ['two in a row, braces inside the body counted', 'fn f() -> f32 { var a = 1.0; if (OFF) { for (var i = 0; i < 2; i++) { a += 1.0; } } if (OFF) { a = 9.0; } return a; }', 'fn f() -> f32 { var a = 1.0; return a; }'],
    ['what follows a block that returns goes', 'fn f() -> f32 { var a = 1.0; if (ON) { return a; } a = heavy(a); return a; }', 'fn f() -> f32 { var a = 1.0; { return a; } }'],
    ['…in a loop, after a continue', 'fn f() -> f32 { var a = 1.0; for (var i = 0; i < 4; i++) { if (!OFF) { continue; } a += heavy(a); } return a; }', 'fn f() -> f32 { var a = 1.0; for (var i = 0; i < 4; i++) { { continue; } } return a; }'],
    ['…and out of the block that held it', 'fn f() -> f32 { var a = 1.0; if (ON) { if (!OFF) { return a; } a = 2.0; } a = heavy(a); return a; }', 'fn f() -> f32 { var a = 1.0; { { return a; } } }'],
    ['…and after an if whose every arm returns', 'fn f(x: f32) -> f32 { if (x > 1.0) { return 1.0; } else if (ON) { return 2.0; } else { return heavy(x); } return heavy(3.0); }', 'fn f(x: f32) -> f32 { if (x > 1.0) { return 1.0; } else { return 2.0; } }'],
    ['a discard is passed: the function still owes its return', '@fragment fn fs() -> @location(0) vec4f { if (ON) { discard; } return vec4f(0.0); }', '@fragment fn fs() -> @location(0) vec4f { { discard; } return vec4f(0.0); }'],
    ['…and a discard with a return after it keeps the return', 'fn f(x: f32) -> f32 { if (ON) { if (x > 0.0) { discard; return 1.0; } else { return 2.0; } } return heavy(x); }', 'fn f(x: f32) -> f32 { { if (x > 0.0) { discard; return 1.0; } else { return 2.0; } } }'],
    ['a loop keeps its continuing block after a continue', 'fn f() -> f32 { var a = 0.0; var i = 0; loop { if (i >= 4) { break; } if (!OFF) { continue; } a += heavy(a); continuing { i++; } } return a; }', 'fn f() -> f32 { var a = 0.0; var i = 0; loop { if (i >= 4) { break; } { continue; } continuing { i++; } } return a; }'],
    ['a test written without its parentheses', 'fn f(x: f32) -> f32 { var a = x; if OFF { a = heavy(a); } if (OFF) && (x > 0.0) { a = heavy(a); } if !ON { a = 1.0; } else if x > 0.0 { a = 2.0; } else { a = 3.0; } return a; }', 'fn f(x: f32) -> f32 { var a = x; if x > 0.0 { a = 2.0; } else { a = 3.0; } return a; }'],
    ['a branch that returns only on one arm keeps what follows', 'fn f(x: f32) -> f32 { var a = x; if (ON) { if (x > 0.0) { return 1.0; } a = 2.0; } return a; }', 'fn f(x: f32) -> f32 { var a = x; { if (x > 0.0) { return 1.0; } a = 2.0; } return a; }'],
  ];
  const left: [string, string][] = [
    ['on && rest: the rest decides', 'fn f(x: f32) -> f32 { if (ON && x > 0.0) { return 1.0; } return 0.0; }'],
    ['off || rest: the rest decides', 'fn f(x: f32) -> f32 { if (OFF || x > 0.0) { return 1.0; } return 0.0; }'],
    ['a constant later in the test: the first part runs first', 'fn f(x: f32) -> f32 { if (side(x) > 0.0 && OFF) { return 1.0; } if (side(x) > 0.0 || ON) { return 2.0; } return 0.0; }'],
    ['a constant inside a comparison or a call', 'fn f(x: bool) -> f32 { if (OFF == x) { return 1.0; } if (pick(OFF)) { return 2.0; } if ((ON) != x) { return 3.0; } return 0.0; }'],
    ['a parameter of the name', 'fn f(OFF: bool) -> f32 { if (OFF) { return 1.0; } return 0.0; }'],
    ['a parameter of the name behind an attribute', 'fn f(@location(0) OFF: bool, y: f32) -> f32 { if (OFF) { return 1.0; } return y; }'],
    ['a local of the name', 'fn f() -> f32 { let OFF = true; if (OFF) { return 1.0; } return 0.0; }'],
    ['a local of the name, written with its address space', 'fn f() -> f32 { var<function> OFF: bool = true; if (OFF) { return 1.0; } return 0.0; }'],
    ['a constant that is no bool', 'fn f() -> f32 { if (N > 2u) { return 1.0; } return 0.0; }'],
    ['a name that is no constant', 'fn f(x: bool) -> f32 { if (x) { return 1.0; } return 0.0; }'],
    ['a select on a constant', 'fn f() -> f32 { return select(1.0, 2.0, OFF); }'],
    ['an if with an attribute in front of it', 'fn f(x: f32) -> f32 { var a = x; @diagnostic(off, derivative_uniformity) if (OFF) { a = dpdx(a); } if (x > 0.0) { a = 1.0; } return a; }'],
    ['a test without parentheses that the rest decides', 'fn f(x: f32) -> f32 { if ON && x > 0.0 { return 1.0; } if x > 0.0 && OFF { return 2.0; } return 0.0; }'],
    ['a loop with a continuing block and nothing folded', 'fn f() -> f32 { var a = 0.0; var i = 0; loop { if (i >= 4) { break; } a += 1.0; continuing { i++; } } return a; }'],
    ['code after a return that was written that way', 'fn f(x: f32) -> f32 { if (x > 0.0) { return 1.0; } return 0.0; }'],
  ];
  const wrong = cases.filter(([, src, want]) => squeeze(fold(K + src)) !== squeeze(K + want)).map(c => `${c[0]} [${squeeze(fold(K + c[1])).slice(squeeze(K).length + 1)}]`);
  const moved = left.filter(([, src]) => squeeze(fold(K + src)) !== squeeze(K + src)).map(c => c[0]);
  check(wrong.length === 0, `forms — ${cases.length} texts, each cut to the side that runs`, wrong.length ? `not as written by hand: ${wrong.join(' · ')}` : '');
  check(BREAK || moved.length === 0, `forms — ${left.length} texts it must leave as written`, moved.length ? `changed: ${moved.join(' · ')}` : '');
  const noConst = 'fn f() -> f32 { if (OFF) { return 1.0; } return 0.0; }';
  { // THE CONSTANTS STAY: a constant that decided a branch of a kept function is kept with it; one that only a dropped function tested goes with that function
    const t = trimWgsl('const ON: bool = true;\nconst OFF: bool = false;\nconst OFF2: bool = false;\nfn idle() -> f32 { if (OFF2) { return 1.0; } return 0.0; }\nfn heavy() -> f32 { return 1.0; }\n@compute @workgroup_size(1) fn main() { var a = 0.0; if (ON) { a = 1.0; } if (OFF) { a = heavy(); } }', true);
    check(t.includes('const ON: bool = true;') && t.includes('const OFF: bool = false;') === !BREAK && !t.includes('OFF2') && !t.includes('heavy') && !t.includes('idle'), 'forms — a constant that decided a branch of a kept function stays in the text; what only the dead side called goes, and so does a constant only a dropped function tested', squeeze(t)); }
  check(squeeze(foldWgsl(noConst)) === squeeze(noConst) && squeeze(foldWgsl('const OFF: bool = false;\n// if (OFF) { gone(); }\n' + noConst)) === squeeze('const OFF: bool = false;\nfn f() -> f32 { return 0.0; }'), 'forms — a text with no constant of the name is left alone; a comment that reads like a branch is no branch');
  }
  { const main = readFileSync(`${ROOT}/src/gpu/wgslTrim.ts`, 'utf8');
    check(/get\('shaderfold'\) === '0'\) SHADER_FOLD\.on = false/.test(main) && /SHADER_TRIM\.on \? trimWgsl\(code, SHADER_FOLD\.on\) : code/.test(main), 'switch — the page lowers it for ?shaderfold=0, and nothing is folded with the trim off'); }
  const nameOf = (d: string): string | null => /^(?:\s*@[A-Za-z_]\w*(?:\s*\((?:[^()]|\([^()]*\))*\))?)*\s*(?:fn|struct|alias|const|override|let)\s+([A-Za-z_]\w*)/.exec(d)?.[1] ?? /^(?:\s*@[A-Za-z_]\w*(?:\s*\((?:[^()]|\([^()]*\))*\))?)*\s*var\s*(?:<[^>]*>)?\s*([A-Za-z_]\w*)/.exec(d)?.[1] ?? null;
  const WORD = /[A-Za-z_][A-Za-z0-9_]*/g;
  const isFn = (d: string) => /\bfn\b/.test(d.slice(0, Math.max(0, d.indexOf('{'))));
  /** a function declares the name itself, read here by its own rule: a parameter is a word before a `:` in the head's parentheses, a local a word after let / var / const */
  const own = (d: string, n: string) => { const o = Math.max(0, d.indexOf('{')), head = d.slice(0, o), params = head.slice(head.indexOf('('));
    return params.split(',').some(p => p.replace(/@\w+(?:\([^()]*\))?/g, '').split(':')[0].trim() === n) || d.slice(o).split(/\b(?:let|var|const)\b/).slice(1).some(x => x.replace(/^\s*<[^>]*>/, '').trim().split(/\W/)[0] === n); };
  /** the block that opens at `o` in `t`, braces counted: its text */
  const blockOf = (t: string, o: number): string => { let d = 0; for (let i = o; i < t.length; i++) { if (t[i] === '{') d++; else if (t[i] === '}' && --d === 0) return t.slice(o, i + 1); } return ''; };
  const AFTER = /\b(?:return\b[^;{}]*;|(?:break|continue)\s*;)\s*(?!\}|continuing\b)\S/;   // a statement standing after one that cannot be passed (a loop's continuing block runs)
  // ---- boot, size and switch: the page's own modules, under each setting of the switches
  for (const st of await settingsHere()) {
    const R = reading(st); if (!R) { check(false, `boot, ${SETTINGS[st]} — the run finished`); continue; }
    const setting = SETTINGS[st], { mods: whole, asked, faults } = R;   // every module as written; the cuts are made here
    check(faults.length === 0 && whole.length >= 35 && asked.length >= 98, `boot, ${setting} — the page's whole boot was recorded: ${whole.length} modules, ${asked.length} pipelines`, faults.join(' · '));
    // the switch: shaderText hands out the trim's own cut with SHADER_FOLD down, the folded cut with it up, and the text whole with the trim off
    { let ok = true;
      for (const m of whole) { SHADER_FOLD.on = false; const a = shaderText(m.code) === trimWgsl(m.code); SHADER_FOLD.on = true; const b = shaderText(m.code) === trimWgsl(m.code, true); SHADER_TRIM.on = false; const c = shaderText(m.code) === m.code; SHADER_TRIM.on = true; ok &&= a && b && c; }
      check(ok, `switch, ${setting} — with SHADER_FOLD down a module is the trim's own cut, character for character; up, it is the folded cut; with the trim off, whole`); }
    const bad = { left: [] as string[], dead: [] as string[], after: [] as string[], twice: [] as string[], roots: [] as string[], open: [] as string[], bigger: [] as string[], consts: [] as string[], lane: [] as string[] };
    let branches = 0; const falseConsts = new Set<string>(), trueConsts = new Set<string>(); const totals = { whole: 0, plain: 0, folded: 0 };
    for (const m of whole) {
      const W = m.code, plain = trimWgsl(W), text = BREAK ? trimWgsl(turned(foldWgsl(turned(W)))) : trimWgsl(W, true);
      const val = boolConsts(wgslDecls(W)); for (const [n, v] of val) (v ? trueConsts : falseConsts).add(n);
      const C = wgslDecls(plain), F = wgslDecls(text), fByName = new Map(F.filter(isFn).map(d => [nameOf(d), d])), fNames = new Set(F.map(nameOf));
      // no test is left that the constants decide: a constant first in the test, alone, or before an `&&` it makes false, or before an `||` it makes true (in parentheses or not)
      for (const d of F) { if (!isFn(d)) continue;
        for (const re of [/\bif\s*\(\s*(!?)\s*([A-Za-z_]\w*)\s*(\)|&&|\|\|)/g, /\bif\s+(!?)\s*([A-Za-z_]\w*)\s*(\{|&&|\|\|)/g]) for (const k of d.matchAll(re)) { const v = val.get(k[2]); if (v === undefined || own(d, k[2])) continue; const on = v !== (k[1] === '!');
          if (k[3] === ')' || k[3] === '{' || (k[3] === '&&' && !on) || (k[3] === '||' && on)) { const before = d.slice(0, k.index).trimEnd(); if (/[;{}]$/.test(before) || /\belse$/.test(before)) bad.left.push(`${m.label}: ${k[0]}`); } } }
      // the dead side of every plain branch of the UNFOLDED cut is gone from the folded one, and the constant that decided it is still in the text: read here off the unfolded text, with braces counted
      for (const d of C) { if (!isFn(d)) continue; const f = fByName.get(nameOf(d)); if (f === undefined) continue;   // (a function nothing reaches any more: gone whole)
        for (const k of d.matchAll(/[;{}]\s*if\s*\(\s*(!?)\s*([A-Za-z_]\w*)\s*\)\s*\{/g)) { const v = val.get(k[2]); if (v === undefined || own(d, k[2])) continue; const on = v !== (k[1] === '!');
          const first = blockOf(d, k.index + k[0].length - 1), rest = d.slice(k.index + k[0].length - 1 + first.length), el = /^\s*else\s*\{/.exec(rest), second = el ? blockOf(rest, el[0].length - 1) : '';
          const gone = squeeze((on ? second : first).slice(1, -1)); branches++;
          if (gone.length >= 24 && squeeze(d).split(gone).length - 1 === 1 && squeeze(f).includes(gone)) bad.dead.push(`${m.label} ${nameOf(d)}: ${k[0].trim().slice(1).trim()}`);
          if (!BREAK && !fNames.has(k[2]) && squeeze(f).includes(squeeze((on ? first : second).slice(1, -1)))) bad.consts.push(`${m.label} ${nameOf(d)}: ${k[2]}`); } }   // (the side that runs is there, so the branch was not cut away with something around it)
      // nothing stands after a return, break or continue in its block (but a loop's continuing)
      for (const d of F) { if (!isFn(d)) continue; const k = AFTER.exec(d); if (k && !AFTER.test(C.find(c => nameOf(c) === nameOf(d)) ?? '')) bad.after.push(`${m.label} ${nameOf(d)}: ${k[0].slice(0, 40)}`); }
      // folded and cut once more it is the same text, less the lines of the constants that decided the first time (nothing tests them any more)
      { const again = new Set(wgslDecls(trimWgsl(text, true))), lost = F.filter(d => !again.has(d)); if (again.size + lost.length !== F.length || lost.some(d => !val.has(nameOf(d) ?? ''))) bad.twice.push(m.label); }
      const roots = C.filter(d => /@(compute|vertex|fragment)\b/.test(d) || /^\s*(?:@[^\n]*\s)*override\b/.test(d) || nameOf(d) === null);
      if (!roots.every(d => nameOf(d) === null ? F.includes(d) : fNames.has(nameOf(d)))) bad.roots.push(m.label);
      const all = new Set(wgslDecls(W).map(nameOf)); all.delete(null);
      if (F.some(d => (d.match(WORD) ?? []).some(w => all.has(w) && !fNames.has(w)))) bad.open.push(m.label);
      if (text.length > plain.length) bad.bigger.push(m.label);
      totals.whole += W.length; totals.plain += plain.length; totals.folded += text.length;
    }
    check(bad.left.length === 0, `boot, ${setting} — no test is left that the constants decide (false here: ${[...falseConsts].sort().join(' ')}; true: ${[...trueConsts].sort().join(' ')})`, bad.left.slice(0, 6).join(' · '));
    check(bad.dead.length === 0 && branches > 20, `boot, ${setting} — the dead side of each of the ${branches} plain branches the unfolded text holds is gone from the folded text`, bad.dead.slice(0, 6).join(' · '));
    check(bad.consts.length === 0, `boot, ${setting} — and the constant that decided each is still in its module's text`, bad.consts.slice(0, 6).join(' · '));
    check(bad.after.length === 0, `boot, ${setting} — nothing is left standing after a return, break or continue in its block`, bad.after.slice(0, 6).join(' · '));
    check(bad.twice.length === 0, `boot, ${setting} — folding and cutting a folded cut text takes nothing more out of it but the lines of the constants that decided${bad.twice.length ? ` — not so: ${bad.twice.slice(0, 5).join(', ')}` : ''}`);
    check(bad.roots.length === 0, `boot, ${setting} — every entry point, override, directive and const_assert the plain cut keeps is kept${bad.roots.length ? ` — not so: ${bad.roots.slice(0, 5).join(', ')}` : ''}`);
    check(bad.open.length === 0, `boot, ${setting} — nothing left mentions a declaration that was dropped${bad.open.length ? ` — not so: ${bad.open.slice(0, 5).join(', ')}` : ''}`);
    check(bad.bigger.length === 0, `boot, ${setting} — no module's folded cut is bigger than its plain cut${bad.bigger.length ? ` — not so: ${bad.bigger.slice(0, 5).join(', ')}` : ''}`);
    const kb = (n: number) => Math.round(n / 1024);
    check(totals.folded < totals.plain, `size, ${setting} — the browser is handed ${kb(totals.folded)} KB of shader text where the trim alone handed ${kb(totals.plain)} KB (${(100 - 100 * totals.folded / totals.plain).toFixed(1)} % less), of ${kb(totals.whole)} KB whole`);
  }
}

// ======== inline
// HOW BIG EACH KERNEL IS ONCE EVERY CALL IS PASTED INTO ITS CALLER. Chrome on Windows compiles a pipeline's WGSL through DXC, and DXC pastes every call into
// its caller before it optimizes: a kernel pays for each CALL SITE of a heavy function (the tracer, `occluded`, `evalLight` with the fog's walk), not for the
// text the module was handed. This prints that size for every distinct (module text, entry point) the page asks a pipeline for, with no browser and no Windows
// machine: the page's boot is run over a stand-in device that records every createShaderModule text and every pipeline descriptor — the Engine, the smoke
// solver, the Game's own GPU modules (mesh shadows, particles, the light probe, the skinned raster) —
// and each entry point is sized by the rule
//     pasted(f) = f's own text + for each call site in f, pasted(the function called)          (WGSL has no recursion)
// in KB of WGSL as gpu/wgslTrim.ts wgslDecls reads it (comments out). Beside it: the same functions counted once each (what a compiler that keeps its calls
// reads), the functions pasted most often into that kernel, the total, and the functions pasted most often over all kernels.
// THE BUDGET (the table below): each big kernel has its own number, within a twentieth of what it is today, and every other kernel the one at the table's
// foot, so a NEW call site worth a twentieth of a big kernel is red here before anybody waits for it on a Windows machine. A kernel that grew on purpose has
// its row raised in the same change; a rewrite that brings one down lowers its row with it (a row with a twentieth to spare is named, not failed; a row
// that no kernel answers to any more is red).
// THE CUT TEST IS COUNTED APART (CUT_TEST below). A program compiled with the cut rule carries common.wgsl's cut test once for every copy of the box test, and
// the test is written with no loop (eight planes straight down, where a loop of one stood: a loop there doubled the fog's march on an NVIDIA card), which is
// 3.1 KB a copy: a fifth to three tenths of a big tracing kernel. Two bars hold it. (1) A kernel is held to its row WITHOUT that text, so a row is as tight as
// it was when the rule lived in a second set of programs. (2) The test has its own number, its size at one copy of the box test, within a twentieth like a row.
// (Until 2026-11-07 the boot's programs had no cut rule and a second set carried it; a third bar held each kernel of that set to its boot twin. There is ONE set
// now, every ray program with the rule, so there is no twin to hold a kernel to.)
// What this is not: a time. DXC's time grows faster than the text and a loop round ONE call site is one copy here as it is there; the order of the list and
// what a rewrite takes out are what it is for. What it cannot see: a pipeline the page makes on first use, later than its boot (the entry points nobody asked
// a pipeline for are listed, not sized into the totals), and the switches a URL flag or a trial turns. The fold of constant switches is on, as on the page: a
// branch a constant decides is not in the text, so a kernel's size follows the switches. The listing is the page on a snow level in its
// weather (cut boxes, the glass fracture and the dust's look on, as host/drawpage.ts puts them: tools/qa/pageboot.ts), and the budget is held to that page AND to the
// title's. What a word on the address would add (a trial) is in neither:
async function inlineSection(): Promise<void> {
    const { wgslDecls } = await import(`${ROOT}/src/gpu/wgslTrim.ts`) as typeof import('../../src/gpu/wgslTrim');
  /** KB of pasted WGSL a kernel may not pass, by `module / entry` (a cut-set module's `Cut` off its label), the first row that matches; the last is everyone else's */
  const BUDGET: [RegExp, number][] = [
    [/^gatherSplit \/ trace$/, 252],   // (the gather is two programs, render/gatherSplit.ts: its rays, then the light at what they hit)
    [/^gatherSplit \/ light$/, 58],   // (113 before the gather took the reflections' one-loop writing of the light loop, 2026-11-07)
    [/^composite(R|S|SR)? \/ (fs|cs)$/, 442],   // (403 until 2026-11-07: the snow's glow and sparkle are in every level's text now, asked by the frame's level word, 421 on the look's page)
    [/^volumetrics \/ main$/, 400],   // (the march is ONE program since 2026-11-07: its froxel-lit twin, its zero-light twin and its dust twin are gone, and the dust's look is in its own text)
    [/^particles \/ update$/, 356],
    [/^restir \/ main$/, 336],   // (295 + an opening's light in this writing, 38 KB: as the table above)
    [/^reflect \/ main$/, 204],
    [/^gbuffer(Cut)? \/ fsBox(Cut)?$/, 302],   // (… and its twin with the cut rule, the same text and the cut's test: 268 where fsBox is 267. It had no row, so the look's page read red against 200)
    [/^gbuffer(Cut)?Snow \/ fsBox(Cut)?$/, 566],   // (the boxes' second writing, with the snow's surface in it: render/gbuffer.ts. 294 on the plain page; 468 and, with the cut rule, 539 on the look's. What a snow level always compiled, and under the look no setting here read it. Every page asks for it now, behind its boot where the level has no snow, so each of this tool's settings holds it)
    [/./, 200],
  ];
  /** the cut test's own functions (common.wgsl: isectCut and the four planes it takes twice), and the KB one copy of the box test may carry of them */
  const CUT_TEST = ['isectCut', 'cutFour'], CUT_TEST_KB = 3.2, ROUND_TEST = ['isectRound'], MODS_MIN = 35, PIPES_MIN = 98;   // (the title's whole boot: its distinct modules and the pipelines it asks for)
    const BREAK = process.env.SHADERINLINE_BREAK === '1';
    const opt = (k: string): string | null => { const i = argv.indexOf('--' + k); if (i < 0) return null; const v = argv[i + 1]; if (v === undefined || v.startsWith('--')) { console.error(`--${k} needs a value`); process.exit(1); } return v; };
    const TOP = ((v: string | null) => { if (v === null) return Infinity; if (!/^[1-9]\d*$/.test(v)) { console.error(`--top needs a count, not '${v}'`); process.exit(1); } return Number(v); })(opt('top'));
    const KERNEL = opt('kernel'), JSON_OUT = opt('json');

  // ---- the page's boot over a recording stand-in (tools/qa/pageboot.ts), under one setting of the switches
  // Since the fold of constant switches (gpu/wgslTrim.ts SHADER_FOLD) a kernel's size follows the switches, so the rows bound every page a player opens with no
  // word on the address: the listing below is a snow level in its weather (the heaviest: the march, the particles and the g-buffer are up to a third bigger
  // there), and the budget is also held to the title's kernels (an arm a level's switch turns OFF is
  // only in that page's text).
    const SETTING = settingArg(argv) ?? 'snow', page = reading(SETTING);
    if (!page) { check(false, `the page's boot ran, on ${SETTINGS[SETTING]}`); return; }
    /** a reading as the page hands it over: copies, the break below writes into them */
    const handed = (R: Reading): Promise<Mod[]> => Promise.all(R.mods.map(async m => ({ label: m.label, code: await cut(m.code) })));
    const { asked, faults } = page, mods = await handed(page);
    for (const f of faults) check(false, f);

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

  // ---- THE BREAK: the biggest kernel takes one more call site, of the lightest function it reaches whose call site is a twentieth of it (both as a row holds them: the cut test's text apart)
  const sizeOf = (fns: Map<string, Fn>, copies: Map<string, number>) => { let s = 0; for (const [g, k] of copies) s += k * fns.get(g)!.own; return s; };
  /** the text of the functions `names` in a kernel: every copy (0 where the fold took their calls out) */
  const textOf = (fns: Map<string, Fn>, copies: Map<string, number>, names: string[]) => names.reduce((s, g) => s + (copies.get(g) ?? 0) * (fns.get(g)?.own ?? 0), 0);
  /** a kernel as its row holds it: the cut test's text apart */
  const heldOf = (fns: Map<string, Fn>, copies: Map<string, number>) => sizeOf(fns, copies) - textOf(fns, copies, CUT_TEST);
  const entryOf = (m: Mod, stage: Stage, entry: string | undefined): string | null => { const fns = fnsOf(m.code); if (entry !== undefined) return fns.get(entry)?.stage === stage ? entry : null; const own = [...fns.values()].filter(f => f.stage === stage); return own.length === 1 ? own[0].name : null; };
  if (BREAK) {
    let best: { mod: number; entry: string; size: number } | null = null;
    for (const a of asked) for (const s of a.stages) { const m = mods[s.mod]; const e = m && entryOf(m, s.stage, s.entry); if (!e) continue; const fns = fnsOf(m.code); const size = heldOf(fns, pasted(fns, e)); if (!best || size > best.size) best = { mod: s.mod, entry: e, size }; }
    if (best) {
      const m = mods[best.mod], fns = fnsOf(m.code), one = (g: string) => heldOf(fns, pasted(fns, g));
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
    interface Row { module: string; entry: string; stage: Stage; pipelines: string[]; pasted: number; once: number; fns: Map<string, Fn>; copies: Map<string, number> }
    /** the rows of one page */
    const rowsOfPage = (ms: Mod[], ask: Asked[]) => {
      const rows = new Map<string, Row>(), idOf = new Map<string, number>(), lost: string[] = [];
      for (const a of ask) for (const s of a.stages) {
        const m = ms[s.mod]; const entry = m ? entryOf(m, s.stage, s.entry) : null;
        if (!m || !entry) { lost.push(`${a.label}${s.entry ? ` / ${s.entry}` : ''}`); continue; }
        if (!idOf.has(m.code)) idOf.set(m.code, idOf.size);
        const key = `${idOf.get(m.code)}/${entry}`; let r = rows.get(key);
        if (!r) { const fns = fnsOf(m.code), copies = pasted(fns, entry); let once = 0; for (const g of copies.keys()) once += fns.get(g)!.own;
          rows.set(key, r = { module: m.label, entry, stage: s.stage, pipelines: [], pasted: sizeOf(fns, copies), once, fns, copies }); }
        if (!r.pipelines.includes(a.label)) r.pipelines.push(a.label);
      }
      return { rows, idOf, lost, list: [...rows.values()].sort((a, b) => b.pasted - a.pasted || (`${a.module} ${a.entry}` < `${b.module} ${b.entry}` ? -1 : 1)) };
    };
    const { rows, idOf, lost, list } = rowsOfPage(mods, asked);
  check(lost.length === 0, `every pipeline's module and entry point were found in the recorded texts${lost.length ? ` — not so: ${lost.slice(0, 6).join(', ')}` : ''}`);
  // (a part of the page that fell out of this boot would leave the budget green on the rest)
  check(mods.length >= MODS_MIN && asked.length >= PIPES_MIN, `the recording is a whole boot: ${mods.length} modules (${MODS_MIN} at least), ${asked.length} pipelines (${PIPES_MIN} at least)`);

  const kb = (n: number) => (n / 1024).toFixed(n < 10240 ? 1 : 0);
  /** the functions pasted more than once into a kernel, by the text their copies come to */
  const most = (r: Row) => [...r.copies].filter(([, k]) => k > 1).map(([g, k]) => ({ fn: g, copies: k, bytes: k * r.fns.get(g)!.own })).sort((a, b) => b.bytes - a.bytes || (a.fn < b.fn ? -1 : 1));
  const nameOf = (r: Row) => `${r.module} / ${r.entry}`;
  /** the text of the cut test in a kernel, and of the round's own test (which a program with the cut rule does not carry) */
  const cutOf = (r: Row) => textOf(r.fns, r.copies, CUT_TEST), roundOf = (r: Row) => textOf(r.fns, r.copies, ROUND_TEST);
  const rowOfName = (module: string, entry: string) => BUDGET.findIndex(([re]) => re.test(`${module} / ${entry}`)), rowOf = (r: Row) => rowOfName(r.module, r.entry), REST = BUDGET.length - 1;
  const budgetOf = (r: Row) => BUDGET[rowOf(r)][1];
  console.log(`every kernel with every call pasted into its caller — ${mods.length} modules, ${asked.length} pipelines, ${list.length} distinct (module text, entry point); KB of WGSL, comments out`);
  console.log('  pasted    once     ×  module / entry (pipelines)  ·  pasted most: function ×copies (KB of them)');
  for (const r of list.slice(0, TOP)) {
    const names = r.pipelines.filter(l => l !== r.module && l !== r.entry);
    console.log(`${kb(r.pasted).padStart(8)}${kb(r.once).padStart(8)}${(r.pasted / r.once).toFixed(1).padStart(6)}  ${nameOf(r)}${names.length ? ` (${names.slice(0, 4).join(', ')}${names.length > 4 ? `, +${names.length - 4}` : ''})` : ''}${most(r).length ? `  ·  ${most(r).slice(0, 4).map(c => `${c.fn} ×${c.copies} (${kb(c.bytes)})`).join(', ')}` : ''}`);
  }
  if (list.length > TOP) console.log(`     … ${list.length - TOP} more (no --top = every one)`);
  const sum = (rs: Row[], k: 'pasted' | 'once') => rs.reduce((s, r) => s + r[k], 0);
  { const rs = list;
    console.log(`total: ${rs.length} kernels, ${kb(sum(rs, 'pasted'))} KB pasted where ${kb(sum(rs, 'once'))} KB is each function once (×${(sum(rs, 'pasted') / Math.max(1, sum(rs, 'once'))).toFixed(1)})`); }
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
    for (const r of hit) { console.log(`${nameOf(r)}: ${kb(r.pasted)} KB pasted, ${r.copies.size} functions`);
      for (const [g, k] of [...r.copies].sort((a, b) => b[1] * r.fns.get(b[0])!.own - a[1] * r.fns.get(a[0])!.own)) console.log(`${kb(k * r.fns.get(g)!.own).padStart(8)} KB ${(100 * k * r.fns.get(g)!.own / r.pasted).toFixed(1).padStart(5)} %  ${g} ×${k} of ${kb(r.fns.get(g)!.own)} KB (one call site of it: ${kb(sizeOf(r.fns, pasted(r.fns, g)))} KB)`); }
  }
  // ---- the other page: the title's kernels, held to the same rows (not under --setting, --kernel or the break: those read one page)
  interface Sized { name: string; row: number; pasted: number; where: string; raw: number; cut: number; round: number }   // (pasted: as a row holds it, WITHOUT the cut test's text; raw: all of it)
  let title: Row[] = [];
  if (settingArg(argv) === null && !BREAK && KERNEL === null) {
    const T = reading('title');
    if (T) title = rowsOfPage(await handed(T), T.asked).list;
    check(title.length >= list.length / 2 && !T?.faults.length, `${SETTINGS.title} was sized too (${title.length} kernels; ${list.length} here, on ${SETTINGS[SETTING]})`, T?.faults.join(' · ') ?? '');
  }
  const sized: Sized[] = [...list.map(r => ({ name: nameOf(r), row: rowOf(r), pasted: r.pasted - cutOf(r), where: '', raw: r.pasted, cut: cutOf(r), round: roundOf(r) })), ...title.map(r => ({ name: nameOf(r), row: rowOf(r), pasted: r.pasted - cutOf(r), where: ` at ${SETTINGS.title}`, raw: r.pasted, cut: cutOf(r), round: roundOf(r) }))].sort((x, y) => y.pasted - x.pasted);
  const over = sized.filter(k => k.pasted > BUDGET[k.row][1] * 1024);
  // a row of the table by the biggest kernel that answers to it on either page: none = the row is stale, a twentieth under = it has room a new call site would hide in
  const rowsBig = BUDGET.slice(0, REST).map(([re, cap], i) => ({ re, cap, big: sized.find(k => k.row === i) }));
  const stale = rowsBig.filter(x => !x.big), spare = rowsBig.filter(x => x.big && x.big.pasted < x.cap * 1024 * 0.95);
  check(sized.length > 0 && over.length === 0, `every kernel is within its budget, the cut test's text apart${title.length ? `, on ${SETTINGS[SETTING]} and at ${SETTINGS.title}` : `, on ${SETTINGS[SETTING]}`} (the biggest: ${sized.length ? `${sized[0].name}, ${kb(sized[0].pasted)} of ${BUDGET[sized[0].row][1]} KB` : 'none recorded'}; a kernel with no row of its own: ${BUDGET[REST][1]} KB)${over.length ? ` — not so: ${[...new Set(over.map(k => `${k.name} ${kb(k.pasted)} of ${BUDGET[k.row][1]} KB${k.where}`))].slice(0, 8).join(', ')} (take the call site out, or raise the kernel's row in this file on purpose; a new big kernel gets a row of its own)` : ''}`);
  { // the cut test at one copy of the box test: the same text in every kernel that carries it
    const per = list.filter(r => (r.copies.get('isectCut') ?? 0) > 0).map(r => ({ name: nameOf(r), one: cutOf(r) / r.copies.get('isectCut')!, n: r.copies.get('isectCut')!, all: cutOf(r) }));
    const big = per.reduce((m, x) => Math.max(m, x.one), 0), most1 = [...per].sort((a, b) => b.all - a.all)[0];
    check(per.length > 0 && big <= CUT_TEST_KB * 1024, `the cut test is within its own number at one copy of the box test: ${kb(big)} of ${CUT_TEST_KB} KB (${CUT_TEST.join(' + ')}; in ${per.length} kernels${most1 ? `, the most of it ${kb(most1.all)} KB at ${most1.n} copies in ${most1.name}` : ''})${big > CUT_TEST_KB * 1024 ? ' — not so: the test grew (raise CUT_TEST_KB in this file on purpose)' : ''}`);
    if (per.length && big < CUT_TEST_KB * 1024 * 0.95) console.log(`  the cut test has a twentieth to spare: ${kb(big)} of ${CUT_TEST_KB} KB (lower CUT_TEST_KB to it)`); }
  check(stale.length === 0, `every row of the budget table has a kernel that answers to it${stale.length ? ` — not so: ${stale.map(x => String(x.re)).join(', ')} (a kernel renamed or gone: its row goes with it)` : ''}`);
  if (spare.length) console.log(`  ${spare.length} rows with a twentieth to spare, by their biggest kernel (lower each to it, or take the row out where it is under ${BUDGET[REST][1]} KB): ${spare.map(x => `${x.big!.name} ${kb(x.big!.pasted)} of ${x.cap}${x.big!.where}`).join(', ')}`);
  if (JSON_OUT) {
    const hash = (s: string) => { let a = 0x811c9dc5, b = 0x1b873593; for (let i = 0; i < s.length; i++) { const c = s.charCodeAt(i); a = Math.imul(a ^ c, 16777619); b = Math.imul(b ^ c, 0x85ebca6b) + (b >>> 13); } return (a >>> 0).toString(36) + (b >>> 0).toString(36); };   // (shader-count-page.cjs's own)
    const row = (setting: string) => (r: Row) => ({ setting, module: r.module, entry: r.entry, stage: r.stage, pipelines: r.pipelines, pasted: r.pasted, once: r.once, cut: cutOf(r), round: roundOf(r), budgetKB: budgetOf(r), most: most(r).slice(0, 8) });
    await Bun.write(JSON_OUT, JSON.stringify({ modules: mods.map(m => ({ label: m.label, h: hash(m.code), chars: m.code.length })),
      kernels: [...list.map(row(SETTING)), ...title.map(row('title'))] }, null, 1));   // (the listing's page first, then the title's kernels where this run sized them)
  }
}

// ======== levelset
// EVERY LEVEL ASKS FOR ONE SET OF PROGRAMS (src/render/frame.ts LevelSw, src/render/meshShadows.ts; the kept page that makes it matter: src/gpu/keep.ts).
// A mission start on the kept page compiles only what the visit has not built, and a text that differs by one constant is a text the visit has not built.
// What a level raises of the shaders — the weather kinds' reads, the snow, the rows of the snow's look — was nine constants of the scene prelude, and the
// sizes of the men's mesh were four constants of the mesh shadows' module: the office, a level with its own roofs, a snow level and a level whose men are
// another cast each compiled a part of the set again (the title to a snow level: a third of the programs). They are data now: a word of the frame block
// and four words of a buffer. Held here, with no browser:
//   texts     the page's boot recorded under the title's switches and under a snow level's in its weather (`reading` above),
//             with the shader rows as written and as the page opens: the same module texts, label for label, and the same pipelines
//   word      the switch still reaches the shaders: the frame block's level word is 0 on the title and every bit of LevelSw on the snow level, the
//             prelude names each bit once, and writeFrame puts the word where common.wgsl's `levelCfg` reads it
//   cast      the mesh shadows' module is one text for both casts (their trees differ in nodes, triangles and levels), and the level list's header
//             carries each cast's own four numbers
// What it does not say: that a level compiles nothing in a BROWSER (a page check: `JOBS=picture node tools/qa/keeppage-page.cjs` reads the kept page's own
// count of what each load compiled), or what the branches a level does not take cost a frame on a GPU (never timed).
async function levelsetSection(): Promise<void> {
  const BREAK = process.env.LEVELSET_BREAK === '1';
  const boot = async (setting: string, firstFrame: boolean) => { const R = reading(setting, firstFrame); if (!R) return null;
    const texts = await Promise.all(R.mods.map(m => cut(m.code)));
    return { ...R, mods: R.mods.map((m, i) => ({ label: m.label, h: sha(texts[i]), n: texts[i].length })), asked: R.asked.map(a => `${a.label} ${a.stages.map(s => `${s.stage}:${R.mods[s.mod]?.label ?? '?'}#${sha(texts[s.mod] ?? '')}/${s.entry ?? ''}`).join(' ')}`) }; };

  // ---- texts, word
  for (const [rows, firstFrame] of [['the shader rows as written', false], ['the shader rows as the page opens', true]] as const) {
    const T = await boot('title', firstFrame), S = await boot('snow', firstFrame);
    if (!T || !S) { check(false, `texts — the two boots ran (${rows})`); continue; }
    const faults = [...T.faults, ...S.faults];
    const other = T.mods.map((m, i) => S.mods[i] && S.mods[i].label === m.label && S.mods[i].h === m.h ? null : m.label).filter(Boolean), pipes = T.asked.filter((a, i) => S.asked[i] !== a);
    check(faults.length === 0 && T.mods.length > 30 && T.asked.length >= 98 && T.mods.length === S.mods.length && other.length === 0 && T.asked.length === S.asked.length && pipes.length === 0 && T.prelude === S.prelude,
      `texts — the title and a snow level in its weather are the same ${T.mods.length} module texts and the same ${T.asked.length} pipelines (${rows})`,
      faults.length ? faults.join(' · ') : other.length || pipes.length || T.mods.length !== S.mods.length ? `${other.length} of ${T.mods.length} modules differ (${other.slice(0, 8).join(', ')}) · ${pipes.length} of ${T.asked.length} pipelines differ · the snow level's boot: ${S.mods.length} modules, ${S.asked.length} pipelines` : `${Math.round(T.mods.reduce((n, m) => n + m.n, 0) / 1024)} KB of text either way`);
    const all = Object.values(S.bits).reduce((a, b) => a | b, 0), single = Object.values(S.bits).every(v => v > 0 && (v & (v - 1)) === 0) && new Set(Object.values(S.bits)).size === Object.keys(S.bits).length;
    check(T.word === 0 && S.word === all && single && Object.keys(S.bits).length === 9, `word — the frame's level word is 0 on the title and every switch's bit on the snow level (${rows})`, `title ${T.word} · snow ${S.word} · the ${Object.keys(S.bits).length} switches together ${all}, each one bit: ${single}`);
  }
  {
    const FR = await import(`${ROOT}/src/render/frame.ts`) as typeof import('../../src/render/frame');
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
}

// ======== rows
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
const ARMS = { off: {}, first: { TPS_FIRSTFRAME: '1' } } as const;
type Arm = keyof typeof ARMS;
const WORDS = ['TPS_FIRSTFRAME'];
const tick = () => new Promise<void>(r => setTimeout(r, 0));
/** a line of this section says its detail only when it fails */
const says = (ok: boolean, what: string, more = ''): void => check(ok, what, ok ? '' : more);

// ---- the stand-in device: every module's text, every pipeline asked for (held), every program a pass is given
interface Held { label: string; kind: 'compute' | 'render'; how: 'async' | 'sync'; mods: string[]; entry: string; done: boolean; let: () => void }
function standIn() {
  const mods: { label: string; code: string }[] = [], asked: Held[] = [], drawn: { pass: string; pipe: string | null }[] = [], wrote: unknown[] = [];
  const modLabel = new WeakMap<object, string>();
  const pipe = (label: string) => fake(`pipe:${label}`);
  const ask = (kind: 'compute' | 'render', how: 'async' | 'sync') => (d: { label?: string; compute?: { module: object; entryPoint?: string }; vertex?: { module: object; entryPoint?: string }; fragment?: { module: object; entryPoint?: string } }) => {
    const stages = kind === 'compute' ? [d.compute] : [d.vertex, d.fragment];
    const a: Held = { label: d.label ?? '?', kind, how, mods: [...new Set(stages.map(s => (s && modLabel.get(s.module)) ?? '?'))], entry: stages.map(s => s?.entryPoint ?? '').join('+'), done: how === 'sync', let: () => { /* set below */ } };
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
  (globalThis as Record<string, unknown>).window ??= globalThis; (globalThis as Record<string, unknown>).devicePixelRatio ??= 1;
  const { SHADER_ROWS } = await import(`${ROOT}/src/ui/shaderrows.ts`) as typeof import('../../src/ui/shaderrows');
  (await import(`${ROOT}/src/gpu/wgslTrim.ts`) as typeof import('../../src/gpu/wgslTrim')).SHADER_FOLD.on = false;   // (which writing of a kernel reaches the device is read off declarations as written and off the switches' own lines: the fold of constant switches cuts both and changes neither answer; its own section is fold)
  const { Engine } = await import(`${ROOT}/src/engine.ts`);
  const { FollowCamera } = await import(`${ROOT}/src/host/camera.ts`);
  const { FrameFlags } = await import(`${ROOT}/src/render/frame.ts`);
  const { PIPE_QUEUE, pipelineProgress, laterIdle } = await import(`${ROOT}/src/gpu/device.ts`) as typeof import('../../src/gpu/device');
  const { Particles } = await import(`${ROOT}/src/render/particles.ts`);
  const wantFirst = name === 'first';
  console.log(`\n=== ${name}: ${wantFirst ? 'Start on first frame shaders on' : 'the start waits for every pipeline'} ===`);
  says(SHADER_ROWS.firstFrame === wantFirst && PIPE_QUEUE.later === wantFirst, `the row this arm asked for is the row the modules read`, `later ${PIPE_QUEUE.later}`);

  const S = standIn(); pipelineProgress.reset();
  const gpu = { device: S.device, adapterInfo: 'stand-in', hasTimestamps: false, format: 'bgra8unorm', linearSampler: fake('sampler'), canvas: { clientWidth: 800, clientHeight: 500, width: 800, height: 500 }, ctx: fake('ctx') };
  const engine = new Engine(gpu as never);
  const particles = new Particles(S.device as never, engine.sceneLayout);   // (the page makes it beside the engine and waits for both: host/drawpage.ts)
  let ready = false; const init = Promise.all([engine.init({ sharp: true, gbaa: true }), particles.init()]).then(() => { ready = true; });
  // THE LATER SET, by name: what the start may leave out with the row on (render/passes.ts traceJobs).
  const LATER = ['composite'];
  const later = new Set(wantFirst ? LATER : []);
  for (let turn = 0; turn < 400 && !ready; turn++) { for (const a of S.asked) if (!a.done && !later.has(a.label)) a.let(); await tick(); }
  const out = () => S.asked.filter(a => !a.done).map(a => a.label).sort();
  says(ready, wantFirst ? 'the start resolves with the LATER set held back' : 'the start resolves once every pipeline it asked for is in', `still waiting on ${out().filter(l => !later.has(l)).join(', ') || 'a pipeline in the LATER set: ' + out().join(', ')}`);
  if (!ready) { await Promise.race([init, tick()]); return finish(); }
  if (wantFirst) {
    says(JSON.stringify(out()) === JSON.stringify([...LATER].sort()), `exactly the LATER set is still out when the start resolves: ${LATER.length} pipelines`, `out: ${out().join(', ')}`);
    says(pipelineProgress.total === pipelineProgress.done, 'the start card counts none of them', `${pipelineProgress.done} of ${pipelineProgress.total}`);
  } else { let idle = false; void laterIdle().then(() => { idle = true; }); await tick();
    says(out().length === 0 && idle, 'nothing is still out when the start resolves, and what a script\'s `__ready` waits on (laterIdle) resolves at once', out().join(', ')); }

  const mod = (label: string) => S.mods.filter(m => m.label === label);
  const has = (label: string) => S.asked.some(a => a.label === label);
  {
    const v = mod('volumetrics');
    says(v.length === 1 && mod('volumetricsF').length === 0 && mod('volumetricsK0').length === 0 && mod('froxel').length === 0 && has('vol') && !S.asked.some(a => /^vol(Froxel|Dust|K0)$|^froxel/.test(a.label)), 'the volumetric pass is ONE program for every tier: no froxel twin, no zero-light twin, no dust twin and no froxel kernel is asked for');
    says(v.every(m => !/VOL_FROXEL_MARCH|VOL_K0|FLAG_FROXEL_VOL|smokeMarchJ|froxel/.test(m.code)), 'its text has no froxel-lit march and no march without a light cap');
    says(v.every(m => m.code.includes('fn dustAlive() -> bool') && m.code.includes('let dl = dustDensity(X, onMask, true);')), 'it carries the dust\'s look behind the frame\'s bit');
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
    says(!plain.err && plain.none.length === 0, `${when}: a frame at the settings as they start gives every pass a program`, plain.err || `none in ${plain.none.join(', ')}`);
    for (const v of VIEWS) {
      const r = frame(v.set), got = r.by[v.pass] ?? [];
      if (r.err || r.none.length) { says(false, `${when}: ${v.name} draws`, r.err || `no program in ${r.none.join(', ')}`); continue; }
      if (landed) says(got.includes(v.own), `${when}: ${v.name} draws with its own program (${v.own})`, `${v.pass}: ${got.join(', ') || 'not drawn'}`);
      else if (v.meanwhile) says(got.includes(v.meanwhile) && !got.includes(v.own), `${when}: ${v.name} draws with ${v.meanwhile} meanwhile`, `${v.pass}: ${got.join(', ') || 'not drawn'}`);
      else says(got.length === 0 && (r.by.compS ?? []).length > 0 && engine.passes.sharpOn === true, `${when}: ${v.name} is stood in for by sharp direct's compute composite`, `composite: ${got.join(', ') || '—'}, compS: ${(r.by.compS ?? []).join(', ') || '—'}`);
    }
    { const fr = frame(s => { s.volumetrics = true; }), got = fr.by.volumetrics ?? []; says(!fr.err && got.length > 0 && got.every(l => l === 'vol'), `${when}: the volumetric pass draws with the one march program`, fr.err || got.join(', ')); }
    // the motion levers are bits of the one program's uniform: a frame the camera moved carries its taps beside the spatial count, whatever has landed
    const mv = frame(s => { s.restirMotion = 7; s.restirMotionTaps = 6; }, true), mvGot = mv.by.direct ?? [], taps = (mv.cfgY >>> 8) & 15;
    says(!mv.err && mvGot.length > 0 && mvGot.every(l => l === 'restir') && taps === 6, `${when}: a frame the camera moved runs the one direct program with its motion taps packed beside the spatial count`, mv.err || `direct: ${mvGot.join(', ')}, cfg.y 0x${mv.cfgY.toString(16)}`);
  };
  if (wantFirst) {
    says(engine.passes.fragCompReady === false, 'the engine knows what is not in yet (no fragment composite)');
    shot('while the LATER set compiles', false);
    // (what a script's `__ready` waits on, host/drawpage.ts: the start card is live for the player, a tool's first picture is the settings' own)
    let idle = false; void laterIdle().then(() => { idle = true; }); await tick(); await tick();
    says(!idle && engine.standsIn === true, 'while it compiles the engine says a pass stands in, and what a script\'s `__ready` waits on (laterIdle) has not resolved');
    for (let turn = 0; turn < 50 && S.asked.some(a => !a.done); turn++) { for (const a of S.asked) a.let(); await tick(); }
    says(engine.passes.fragCompReady, 'every LATER pipeline arrives: the fragment composites are in');
    for (let turn = 0; turn < 10 && !idle; turn++) await tick();
    says(idle && engine.standsIn === false, 'with all of it in nothing stands in and laterIdle resolves');
    shot('once it is in', true);
  } else {
    says(engine.passes.fragCompReady, 'the fragment composites are in when the start resolves');
    shot('at the start', true);
  }
  {   // a settings table that says no light cap (RenderSettings' own 0, before a preset places one) still draws: the engine reads it as high's 4, and no march without a cap exists
    const r = frame(s => { s.volLightCap = 0; }), got = r.by.volumetrics ?? [];
    says(!r.err && r.none.length === 0 && got.length > 0 && got.every(l => l === 'vol') && new Uint32Array(engine.frameData)[136] === 4, 'a frame with no light cap said draws with the one march program, its cap sent as high\'s 4', r.err || `${got.join(', ')} · volCfg.x ${new Uint32Array(engine.frameData)[136]}`);
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
  old: { kept: '{"firstFrame":false}', search: '', ua: MAC, want: true },
  word: { kept: null as string | null, search: '?firstframe=0', ua: MAC, want: false },
  visit: { kept: null as string | null, search: '?firstframe=1', want: true },
  // … and in a BROWSER (a tool is none: its own name is not a browser's), where it starts on for a person
  starts: { kept: null as string | null, search: '', ua: MAC, want: true },
  plain: { kept: null as string | null, search: '?compactshaders=0&splitrays=0&firstframe=0', ua: WIN, want: false },
  // … and in a browser a SCRIPT drives (every page tool's): it starts off there, so a tool's first picture is the settings' own
  driven: { kept: null as string | null, search: '', ua: 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/141.0.0.0 Safari/537.36', driven: true, want: false },
  // … and with no browser at all: off
  harness: { kept: null as string | null, search: '', want: false },
} as Record<string, { kept: string | null; search: string; ua?: string; driven?: boolean; want: boolean }>;
async function store(name: keyof typeof STORE) {
  const c = STORE[name], LS = 'twopointsix.shaderrows', box = new Map<string, string>(); if (c.kept !== null) box.set(LS, c.kept);
  let writes = 0;
  (globalThis as Record<string, unknown>).localStorage = { getItem: (k: string) => box.get(k) ?? null, setItem: (k: string, v: string) => { writes++; box.set(k, v); }, removeItem: (k: string) => { writes++; box.delete(k); } };
  (globalThis as Record<string, unknown>).location = { search: c.search };
  if (c.ua) Object.defineProperty(globalThis, 'navigator', { value: { userAgent: c.ua, webdriver: c.driven === true }, configurable: true });
  const { SHADER_ROWS, useShaderRow, shaderRowsOn, shaderRowLabel } = await import(`${ROOT}/src/ui/shaderrows.ts`) as typeof import('../../src/ui/shaderrows');
  says(SHADER_ROWS.firstFrame === c.want && writes === 0, `${name}: the load's first-frame start is ${c.want ? 'on' : 'off'}, and nothing is written`, JSON.stringify(SHADER_ROWS));
  if (name === 'visit') {
    says(shaderRowsOn().length === 0, 'a report names no row whose reader is not in the build');
    useShaderRow('firstFrame'); says(shaderRowsOn().join() === 'firstframe', 'with its reader in the build a report names it by its word');
    says(shaderRowLabel('compactshaders') === 'Compact shaders' && shaderRowLabel('splitrays') === 'Separate shadow rays' && shaderRowLabel('firstframe') === 'Start on first frame shaders' && shaderRowLabel('a-later-build') === 'a-later-build', 'a report from a build that still had the two retired rows reads by their names');
  }
  return red;
}

async function rowsSection(): Promise<void> {
  const dump = argOf('--dump');
  if (dump !== null) { const p = Bun.spawnSync(['bun', 'run', import.meta.path, '--arm', process.env.TPS_FIRSTFRAME === '1' ? 'first' : 'off', '--dump', dump], { stdout: 'inherit', stderr: 'inherit', env: process.env as Record<string, string> }); void p; return; }   // this tree, the environment's row
  const BREAK = process.env.SHADERROWS_BREAK === '1', dir = tmp;
  const boots: Partial<Record<Arm, string[]>> = {};
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
  says(same(boots.off, boots.first), 'Start on first frame shaders asks for the same pipelines as the build without it, by name');
  says(!!boots.off && ['gatherTrace', 'gatherLight', 'vol'].every(l => boots.off!.includes(l)) && !boots.off.some(l => /^fgather|^vol(Froxel|Dust|K0)$|^froxel/.test(l)), 'the gather is its two programs and the volumetric pass its one, and no one-kernel gather, march twin or froxel kernel is asked for', (boots.off ?? []).filter(l => /gather|^vol/.test(l)).join(', '));
  // the page's wiring, read as text: a script's `__ready` waits for the later set, a player's start does not
  says(/__ready = ready\.then\(\(\) => laterIdle\(\)\);/.test(readFileSync(`${ROOT}/src/host/drawpage.ts`, 'utf8')), 'the page resolves `__ready` only once the later set is in');
}

// ======== which sections, and the children
const boot = argOf('--boot'), arm_ = argOf('--arm'), store_ = argOf('--store');
if (boot !== null) { await bootChild(boot, argOf('--out')!); process.exit(0); }
if (store_ !== null) { section = 'rows'; await store(store_ as keyof typeof STORE); process.exit(red ? 1 : 0); }
if (arm_ !== null) { section = 'rows'; if (!(arm_ in ARMS)) { console.error(`unknown arm ${arm_}`); process.exit(2); } await arm(arm_ as Arm, argOf('--dump')); process.exit(red ? 1 : 0); }
if (argv.includes('--trim-child')) { section = 'trim'; await trimSection(); process.exit(red ? 1 : 0); }
const VALUED = new Set(['--root', '--top', '--kernel', '--json', '--setting', '--dump']), words = argv.filter((a, i) => !a.startsWith('--') && !VALUED.has(argv[i - 1]));
{ const strange = words.filter(w => !(SECTIONS as readonly string[]).includes(w)); if (strange.length) { console.error(`shaders-check: no section ${strange.join(', ')}: ${SECTIONS.join(' · ')}`); process.exit(2); } }
const RUN: Record<Section, () => void | Promise<void>> = { parse: parseSection, trim: () => child('the Engine\'s boots', ['--trim-child'], process.env as Record<string, string>), fold: foldSection, inline: inlineSection, levelset: levelsetSection, rows: rowsSection };
for (const s of SECTIONS) if (!words.length || words.includes(s)) { section = s; console.log(`\n=== ${s} ===`); await RUN[s](); }
console.log(red ? `\nshaders-check: ${red} FAILED` : '\nshaders-check: all passed');
process.exit(red ? 1 : 0);
