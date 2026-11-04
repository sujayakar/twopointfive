// A MODULE'S TEXT CUT TO WHAT ITS CONSTANT SWITCHES LEAVE ON (src/gpu/wgslTrim.ts SHADER_FOLD, foldWgsl). The scene prelude's switches are constants of the
// text (`const WX_KINDS: bool = false;`) and the kernels branch on them; the fold cuts each such branch to the side that runs before the trim reads what is
// reachable, so a trial that is off on a page is not written into its programs. Held here, with no browser and no compiler:
//   forms    small texts of every form the fold reads (a plain branch, a negated one, `NAME && …`, `A || B`, a constant written as a test of constants, an
//            else, an else-if chain with the constant first, in the middle or last, a branch inside the kept side, what follows a block that cannot be
//            passed, a discard, a loop's continuing block, a test without its parentheses) come out as written by hand; what it must NOT touch stays as written (`x && NAME`, a name inside a comparison or a call, a function
//            with a parameter or a local of the constant's name, a constant that is no bool, a `select`, an `if` with an attribute in front)
//   boot     the shaders the PAGE boots with (tools/qa/pageboot.ts: the Engine, the smoke solver, the Game's own modules and the cut-box set, with the
//            switches main.ts raises), under each setting of the switches (the title, a snow level in its weather, and the look prototype's page where
//            the tree has one; this file runs itself once for each), folded: no test is left that the constants decide; the dead side of every plain
//            `if (NAME)` / `if (!NAME)` of the unfolded text is gone from the folded one, and the constant that decided it is still there (both read off the
//            unfolded text here, not by the fold); the modules the compile queue gives its background lane still carry the line it finds them by; no
//            statement is left after a return, break or continue in its block; folding twice cuts no branch the first time left; every entry point, override and directive
//            is kept; nothing left mentions a declaration that was dropped; no module is bigger
//   size     the text a boot hands the browser, with and without (pasted call site by call site it is tools/qa/shaderinline-check.ts's)
//   switch   with SHADER_FOLD down the texts are the trim's alone, character for character; the page lowers it for ?shaderfold=0
// The stand is the caller's: run it bare and with TPS_PROFILE=shipped in front (the page's own texts).
// That the folded text COMPILES with no warning is tools/qa/wgsl-compile.cjs's to say, and that it draws the same frame tools/qa/draw-equal.cjs's
// (`capture … --with shaderfold=0` against a capture without, then `compare`: the software adapter).
//   bun run tools/qa/shaderfold-check.ts             exit 0 = every check passed, 1 = one failed
//   SHADERFOLD_BREAK=1 bun run tools/qa/shaderfold-check.ts     the fold is handed every literal constant with its value turned over: `forms` and `boot` must FAIL
export {};
const { ROOT } = await import('./headless.ts');
const { pageBoot, settingArg, settingsHere, SETTINGS } = await import('./pageboot.ts');
const { SHADER_TRIM, SHADER_FOLD, trimWgsl, foldWgsl, wgslDecls, trimTotals, boolConsts, shaderText } = await import(`${ROOT}/src/gpu/wgslTrim.ts`) as typeof import('../../src/gpu/wgslTrim');
const BREAK = process.env.SHADERFOLD_BREAK === '1'; let red = 0;
const check = (ok: boolean, what: string, detail = '') => { if (!ok) red++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
const squeeze = (s: string) => s.replace(/\s+/g, ' ').trim();
/** the break path: every bool constant's value turned over before the fold reads it (the wrong side is then the one kept) */
const turned = (code: string) => code.replace(/(const\s+\w+\s*:\s*bool\s*=\s*)(true|false)/g, (_m, a, v) => `${a}${v === 'true' ? 'false' : 'true'}`);
const fold = (code: string) => { if (!BREAK) return foldWgsl(code); const t = foldWgsl(turned(code)); return turned(t); };   // (…and the constants' lines put back as they were)

// ---- boot: the page's own modules (tools/qa/pageboot.ts), under each setting of the switches: this file run once more for each
const SETTING = settingArg(process.argv.slice(2));
if (SETTING === null) {
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
    check(t.includes('const ON: bool = true;') && t.includes('const OFF: bool = false;') === !BREAK && !t.includes('OFF2') && !t.includes('heavy') && !t.includes('idle'), 'forms — a constant that decided a branch of a kept function stays in the text (the compile queue reads the cut set\'s line there); what only the dead side called goes, and so does a constant only a dropped function tested', squeeze(t)); }
  check(squeeze(foldWgsl(noConst)) === squeeze(noConst) && squeeze(foldWgsl('const OFF: bool = false;\n// if (OFF) { gone(); }\n' + noConst)) === squeeze('const OFF: bool = false;\nfn f() -> f32 { return 0.0; }'), 'forms — a text with no constant of the name is left alone; a comment that reads like a branch is no branch');
}

  for (const st of await settingsHere()) {
    const r = Bun.spawnSync(['bun', 'run', import.meta.path, '--setting', st], { env: process.env as Record<string, string>, stdout: 'pipe', stderr: 'pipe' });
    const out = r.stdout.toString(); process.stdout.write(out);
    if (r.exitCode !== 0 && !/^- FAIL/m.test(out)) check(false, `boot, ${SETTINGS[st]} — the run finished`, r.stderr.toString().trim().split('\n').slice(-3).join(' | '));
    red += (out.match(/^- FAIL/gm) ?? []).length;
  }
  { const main = (await import('node:fs')).readFileSync(`${ROOT}/src/gpu/wgslTrim.ts`, 'utf8');
    check(/get\('shaderfold'\) === '0'\) SHADER_FOLD\.on = false/.test(main) && /SHADER_TRIM\.on \? trimWgsl\(code, SHADER_FOLD\.on\) : code/.test(main), 'switch — the page lowers it for ?shaderfold=0, and nothing is folded with the trim off'); }
  console.log(red ? `${red} FAILED` : 'all passed');
  process.exit(red ? 1 : 0);
}
const nameOf = (d: string): string | null => /^(?:\s*@[A-Za-z_]\w*(?:\s*\((?:[^()]|\([^()]*\))*\))?)*\s*(?:fn|struct|alias|const|override|let)\s+([A-Za-z_]\w*)/.exec(d)?.[1] ?? /^(?:\s*@[A-Za-z_]\w*(?:\s*\((?:[^()]|\([^()]*\))*\))?)*\s*var\s*(?:<[^>]*>)?\s*([A-Za-z_]\w*)/.exec(d)?.[1] ?? null;
const WORD = /[A-Za-z_][A-Za-z0-9_]*/g;
const isFn = (d: string) => /\bfn\b/.test(d.slice(0, Math.max(0, d.indexOf('{'))));
/** a function declares the name itself, read here by its own rule: a parameter is a word before a `:` in the head's parentheses, a local a word after let / var / const */
const own = (d: string, n: string) => { const o = Math.max(0, d.indexOf('{')), head = d.slice(0, o), params = head.slice(head.indexOf('('));
  return params.split(',').some(p => p.replace(/@\w+(?:\([^()]*\))?/g, '').split(':')[0].trim() === n) || d.slice(o).split(/\b(?:let|var|const)\b/).slice(1).some(x => x.replace(/^\s*<[^>]*>/, '').trim().split(/\W/)[0] === n); };
/** the block that opens at `o` in `t`, braces counted: its text */
const blockOf = (t: string, o: number): string => { let d = 0; for (let i = o; i < t.length; i++) { if (t[i] === '{') d++; else if (t[i] === '}' && --d === 0) return t.slice(o, i + 1); } return ''; };
const AFTER = /\b(?:return\b[^;{}]*;|(?:break|continue)\s*;)\s*(?!\}|continuing\b)\S/;   // a statement standing after one that cannot be passed (a loop's continuing block runs)
const QUEUE_LINE = 'const CUT_BOXES: bool = true;';   // what gpu/device.ts installPipeQueue looks for in a module's text to give it the background lane
{
  const setting = SETTINGS[SETTING];
  SHADER_TRIM.on = false; const { mods: whole, asked, faults } = await pageBoot(SETTING); SHADER_TRIM.on = true;   // every module as written; the cuts are made here
  check(faults.length === 0 && whole.length >= 60 && asked.some(a => a.set === 'cut'), `boot, ${setting} — the page's whole boot was recorded: ${whole.length} modules, the cut-box set among them`, faults.join(' · '));
  { const device = (await import('node:fs')).readFileSync(`${ROOT}/src/gpu/device.ts`, 'utf8'); check(device.includes(`desc.code.includes('${QUEUE_LINE}')`), `boot, ${setting} — the compile queue still finds a cut-set module by the line this check holds (\`${QUEUE_LINE}\`)`); }
  // the switch: shaderText hands out the trim's own cut with SHADER_FOLD down, the folded cut with it up, and the text whole with the trim off
  { let ok = true; const t0 = { ...trimTotals };
    for (const m of whole) { SHADER_FOLD.on = false; const a = shaderText(m.code) === trimWgsl(m.code); SHADER_FOLD.on = true; const b = shaderText(m.code) === trimWgsl(m.code, true); SHADER_TRIM.on = false; const c = shaderText(m.code) === m.code; SHADER_TRIM.on = true; ok &&= a && b && c; }
    Object.assign(trimTotals, t0);
    check(ok, `switch, ${setting} — with SHADER_FOLD down a module is the trim's own cut, character for character; up, it is the folded cut; with the trim off, whole`); }
  const bad = { left: [] as string[], dead: [] as string[], after: [] as string[], twice: [] as string[], roots: [] as string[], open: [] as string[], bigger: [] as string[], consts: [] as string[], lane: [] as string[] };
  let branches = 0, lane = 0; const falseConsts = new Set<string>(), trueConsts = new Set<string>(); const totals = { whole: 0, plain: 0, folded: 0 };
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
    if (plain.includes(QUEUE_LINE)) { lane++; if (!text.includes(QUEUE_LINE)) bad.lane.push(m.label); }
    totals.whole += W.length; totals.plain += plain.length; totals.folded += text.length;
  }
  check(bad.left.length === 0, `boot, ${setting} — no test is left that the constants decide (false here: ${[...falseConsts].sort().join(' ')}; true: ${[...trueConsts].sort().join(' ')})`, bad.left.slice(0, 6).join(' · '));
  check(bad.dead.length === 0 && branches > 20, `boot, ${setting} — the dead side of each of the ${branches} plain branches the unfolded text holds is gone from the folded text`, bad.dead.slice(0, 6).join(' · '));
  check(bad.consts.length === 0, `boot, ${setting} — and the constant that decided each is still in its module's text`, bad.consts.slice(0, 6).join(' · '));
  check(bad.lane.length === 0 && lane > 10, `boot, ${setting} — every one of the ${lane} modules the compile queue gives its background lane (the cut set's, by their constant's line) still carries that line folded`, bad.lane.slice(0, 6).join(' · '));
  check(bad.after.length === 0, `boot, ${setting} — nothing is left standing after a return, break or continue in its block`, bad.after.slice(0, 6).join(' · '));
  check(bad.twice.length === 0, `boot, ${setting} — folding and cutting a folded cut text takes nothing more out of it but the lines of the constants that decided${bad.twice.length ? ` — not so: ${bad.twice.slice(0, 5).join(', ')}` : ''}`);
  check(bad.roots.length === 0, `boot, ${setting} — every entry point, override, directive and const_assert the plain cut keeps is kept${bad.roots.length ? ` — not so: ${bad.roots.slice(0, 5).join(', ')}` : ''}`);
  check(bad.open.length === 0, `boot, ${setting} — nothing left mentions a declaration that was dropped${bad.open.length ? ` — not so: ${bad.open.slice(0, 5).join(', ')}` : ''}`);
  check(bad.bigger.length === 0, `boot, ${setting} — no module's folded cut is bigger than its plain cut${bad.bigger.length ? ` — not so: ${bad.bigger.slice(0, 5).join(', ')}` : ''}`);
  const kb = (n: number) => Math.round(n / 1024);
  check(totals.folded < totals.plain, `size, ${setting} — the browser is handed ${kb(totals.folded)} KB of shader text where the trim alone handed ${kb(totals.plain)} KB (${(100 - 100 * totals.folded / totals.plain).toFixed(1)} % less), of ${kb(totals.whole)} KB whole`);
}
process.exit(red ? 1 : 0);
