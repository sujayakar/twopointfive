// THE SIMULATION'S MODULE (rust/sim → src/sim/built.ts, loaded by src/sim/sim.ts): is it the build of its sources, does it load, and is its arithmetic the game's own?
//   bun run tools/qa/rust-check.ts                   seconds; exit 0 = all pass, 1 = one failed (named)
//   RUST_BREAK=1 bun run tools/qa/rust-check.ts      the red path: the Rust side is handed the ENGINE's cosine where the game's own belongs. `maths` must FAIL
//                                                    (RED_SECTIONS). Exit 1 = red as it must · exit 3 = one of them stayed green, named: it holds nothing until that is mended
// What a system of the simulation answers is held by the crates' own tests (cargo) and by the game's digest.
// WHAT IS HELD:
//   built     src/sim/built.ts is the build of rust/sim as it stands (the hash of the sources; where there is a Rust compiler, the bytes built again), is
//             small enough for a page to compile in one step, and carries no path of the machine that built it (tools/sim/build.ts --check)
//   face      the module's exports are the ones src/sim/sim.ts names, no more and no fewer, at the version it was written against; it starts in one step
//   maths     one fixed stream of inputs through the fourteen functions of src/math/exact.ts and through the Rust ones (rust/sim/src/maths.rs): every single
//             answer the same bits (zeros by sign, NaN as NaN), the decimal powers log10 leans on among them
//   switch    RUST.maths with the game's own maths on puts the Rust functions in Math's place and under the lengths of math/hyp.ts; off puts the TypeScript
//             ones back; with the game's own maths off the engine's stay whatever the row says
//   stand     both stands have the rows of src/sim/sim.ts RUST_ON on, and the game's own maths, as the page, and NO WORD moves either: a retired word
//             in the environment (TPS_RUST, TPS_RUST<ROW>, TPS_EXACT) stops the tool that says it, by name; FROZEN_HOLDS holds no row and no maths; the
//             address's old words (?rust=0, ?rust<row>, ?noexact) write nothing (profile.ts pageRows), also over rows a recipe wrote; and a check's own
//             arm (QA_HOLD, tools/qa/headless.ts hold) holds what it names and it alone
//   views     the kept views over the module's memory are remade when the memory has grown
// Not game code: it reads. Nothing here needs a GPU, and nothing here has been seen on one.
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
const ROOT = resolve(import.meta.dir ?? new URL('.', import.meta.url).pathname, '../..');
const args = process.argv.slice(2);
const BREAK = process.env.RUST_BREAK === '1';
const NAMES = ['sin', 'cos', 'tan', 'asin', 'acos', 'atan', 'atan2', 'exp', 'log', 'log2', 'log10', 'pow', 'tanh', 'hypot'] as const;
type Fns = Record<string, (...a: number[]) => number>;
const engine = (globalThis as any).Bun ? `bun ${(globalThis as any).Bun.version} (JavaScriptCore)` : `node ${process.version} (V8 ${process.versions.v8})`;

// ---- `stand`, in a child: what a stand leaves the rows at
if (args[0] === '--rows') {
  await import(`${ROOT}/tools/qa/headless.ts`);
  const H: any = await import(`${ROOT}/src/host/standup.ts`), S: any = await import(`${ROOT}/src/sim/sim.ts`), P: any = await import(`${ROOT}/src/host/profile.ts`);
  H.pinMaths(); H.pinRust();
  console.log('RUST-ROWS ' + JSON.stringify({ profile: P.harnessProfile(), maths: S.RUST.maths, exact: (await import(`${ROOT}/src/math/exact.ts`)).EXACT.on, on: S.rustRows().filter((r: string) => S.RUST[r]), rows: S.rustRows() }));
  process.exit(0);
}

const X: any = await import(`${ROOT}/src/math/exact.ts`);
const S: any = await import(`${ROOT}/src/sim/sim.ts`);
const HY: any = await import(`${ROOT}/src/math/hyp.ts`);
const SYS: Fns = X.SYS;
const own: Fns = {}; for (const n of NAMES) own[n] = X[n];
const mod = S.sim();
// ---- one fixed stream of inputs through both (tools/qa/rust-fold.js)
const { fold, sameBits, FOLD_N }: { fold(fns: Fns, n?: number, other?: Fns | null): { word: string; asked: number; differ: number; first: string[] }; sameBits(a: number, b: number): boolean; FOLD_N: number } = await import(`${ROOT}/tools/qa/rust-fold.js`);
const rust: Fns = { ...S.mathsOfRust() };   // (the set the row installs, not one made here)
if (BREAK) rust.cos = SYS.cos;
const N = FOLD_N;

/** the sections the red path must turn red */
const RED_SECTIONS = ['maths'];
const said: Record<string, boolean> = {};   // (said: every verdict of a section, all of them PASS?)
let fails = 0;
const verdict = (ok: boolean, label: string, detail: string) => { if (!ok) fails++; const k = label.split(':')[0]; said[k] = (said[k] ?? true) && ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
const child = (cmd: string[], env: Record<string, string | undefined> = {}): { out: string; err: string; status: number | null; missing: boolean } => {
  const e: Record<string, string | undefined> = { ...process.env, ...env }; for (const k of Object.keys(e)) if (e[k] === undefined) delete e[k];
  const r = spawnSync(cmd[0], cmd.slice(1), { cwd: ROOT, env: e as Record<string, string>, encoding: 'utf8', maxBuffer: 1 << 26 });
  return { out: r.stdout ?? '', err: r.stderr ?? '', status: r.status, missing: !!r.error && (r.error as { code?: string }).code === 'ENOENT' };
};
const marked = (out: string, mark: string): any => { const l = out.split('\n').find(x => x.startsWith(mark)); return l ? JSON.parse(l.slice(mark.length)) : null; };
console.log(`the simulation in Rust · ${engine} · module ${S.sim().memory.buffer.byteLength / 65536} pages of memory, started in ${S.simBootMs.toFixed(1)} ms${BREAK ? ' · RUST_BREAK=1 (the red path)' : ''}`);

// ---- built
{ const B: any = await import(`${ROOT}/tools/sim/build.ts`); for (const v of B.checkBuilt()) verdict(v.ok, `built · ${v.label}`, v.detail); }

// ---- face
{
  const built = new WebAssembly.Module(Uint8Array.from(atob((await import(`${ROOT}/src/sim/built.ts`) as any).SIM_WASM.join('')), c => c.charCodeAt(0)));
  const names = WebAssembly.Module.exports(built).map(e => e.name).sort(), asks = WebAssembly.Module.imports(built).map(i => `${i.module}.${i.name}`);
  const want = [...S.SIM_EXPORTS].sort() as string[];
  const extra = names.filter(n => !want.includes(n)), gone = want.filter(n => !names.includes(n));
  verdict(extra.length === 0 && gone.length === 0 && asks.length === 0 && mod.sim_abi() === S.SIM_ABI, 'face: the module exports what src/sim/sim.ts names, at its version, and imports nothing',
    `${names.length} exports, version ${mod.sim_abi()} (sim.ts: ${S.SIM_ABI})${asks.length ? ` · it imports ${asks.join(' ')}` : ''}${extra.length ? ` · in the module and not in sim.ts: ${extra.join(' ')}` : ''}${gone.length ? ` · in sim.ts and not in the module: ${gone.join(' ')}` : ''}`);
}

// ---- face of the other module: dressing (src/dress/dress.ts) — it starts, it is what its face names, and its memory is not the simulation's
{
  const D: any = await import(`${ROOT}/src/dress/dress.ts`), B: any = await import(`${ROOT}/src/dress/built.ts`);
  const names = WebAssembly.Module.exports(new WebAssembly.Module(Uint8Array.from(atob(B.DRESS_WASM.join('')), c => c.charCodeAt(0)))).map(e => e.name).sort();
  const want = [...D.DRESS_EXPORTS].sort() as string[], extra = names.filter(n => !want.includes(n)), gone = want.filter(n => !names.includes(n));
  const no = D.dressWhyNot(), apart = !no && D.dress().memory !== mod.memory && D.dress().memory.buffer !== mod.memory.buffer, view = no ? 0 : D.dressIn().length;
  verdict(extra.length === 0 && gone.length === 0 && !no && D.dress().dress_abi() === D.DRESS_ABI && apart && view === D.dress().dress_in_len() && view > 0, 'face (dressing): the dressing module exports what src/dress/dress.ts names, starts, and has a memory of its own',
    no ? `it does not start: ${no}` : `${names.length} exports, version ${D.dress().dress_abi()} (dress.ts: ${D.DRESS_ABI}) · its memory is ${apart ? 'another' : 'THE SIMULATION\'S'} · the hand-over holds ${view} singles${extra.length ? ` · in the module and not in dress.ts: ${extra.join(' ')}` : ''}${gone.length ? ` · in dress.ts and not in the module: ${gone.join(' ')}` : ''}`);
}
// ---- maths
let wordOwn = '', wordRust = '';
{
  const a = fold(own, N, rust); wordOwn = a.word; wordRust = fold(rust, N).word;
  verdict(a.differ === 0 && wordOwn === wordRust, 'maths: every answer of the Rust functions is the TypeScript one, bit for bit',
    `${(a.asked / 1e6).toFixed(1)} million answers over fourteen functions · ${a.differ} differ · TypeScript folds to ${wordOwn}, Rust to ${wordRust}${a.first.length ? '\n    ' + a.first.join('\n    ') : ''}`);
  // (the decimal powers log10 leans on: the Rust table is the language's own literals. Below 1e-307 a power is subnormal and the TypeScript's own answer
  //  stands a few 1e-9 off the exponent — so does Rust's, to the bit: the port keeps what the game has, it does not mend it)
  let bad = 0, off = 0, at = 0; for (let k = -323; k <= 308; k++) { const x = Number(`1e${k}`), r = rust.log10(x), t = own.log10(x); if (!sameBits(r, t)) { bad++; at = k; } else if (k >= -307 && !sameBits(r, k)) { off++; at = k; } }
  verdict(bad === 0 && off === 0, 'maths: log10 of every decimal power the language can write is the same in both, and its exponent wherever the power is a normal double', bad || off ? `${bad} differ between the two, ${off} of 1e-307 … 1e308 are not their exponent (the last: 1e${at} → ${rust.log10(Number(`1e${at}`))} in Rust, ${own.log10(Number(`1e${at}`))} in TypeScript)` : '1e-323 … 1e308: 632 of 632 the same bits · 1e-307 … 1e308: 616 of 616 exactly the exponent');
}

// ---- switch
{
  const was = { exact: X.EXACT.on, rust: S.RUST.maths }, notes: string[] = []; let ok = true;
  const want = (c: boolean, what: string) => { if (!c) { ok = false; notes.push(`NOT ${what}`); } };
  X.EXACT.on = false; S.RUST.maths = false;
  const engineSin = Math.sin, engineHyp = HY.hyp2;
  S.RUST.maths = true; want(Math.sin === engineSin && HY.hyp2 === engineHyp, 'the engine\'s own while the game\'s own maths are off, whatever the row says');
  X.EXACT.on = true; want(Math.sin === mod.maths_sin && Math.atan2 === mod.maths_atan2 && Math.pow === mod.maths_pow, 'the Rust functions in Math\'s place with both on');
  want(HY.hyp2 === mod.maths_hypot2 && HY.hyp3 === mod.maths_hypot3 && HY.hyp4 === mod.maths_hypot4, 'the Rust sums under the lengths with both on');
  want(sameBits(Math.hypot(3, 4, 12, 84, 0.5), own.hypot(3, 4, 12, 84, 0.5)) && sameBits(Math.hypot(3, 4), 5) && sameBits(Math.hypot(), 0) && sameBits(Math.hypot(-7), 7), 'Math.hypot of any number of terms the same answer');
  S.RUST.maths = false; want(Math.sin === X.sin && HY.hyp2 !== mod.maths_hypot2, 'the TypeScript functions back when the row goes off');
  S.RUST.maths = true; X.EXACT.on = false; want(Math.sin === engineSin && HY.hyp2 === engineHyp, 'the engine\'s own back when the game\'s own maths go off');
  X.EXACT.on = true; want(Math.sin === mod.maths_sin, 'the Rust functions again when they come back on');
  S.RUST.maths = was.rust; X.EXACT.on = was.exact;
  verdict(ok, 'switch: RUST.maths puts the Rust functions where the game\'s own maths stand, and takes them out', ok ? 'off · on under the engine\'s maths (nothing moves) · on under the game\'s own (Math and the lengths are the module\'s) · off (the TypeScript ones) · and round again' : notes.join(' · '));
}

// ---- stand
{
  const all: string[] = S.rustRows(), none: Record<string, undefined> = { TPS_PROFILE: undefined, QA_HOLD: undefined };
  const rowsOf = (env: Record<string, string | undefined>) => child(['bun', 'run', 'tools/qa/rust-check.ts', '--rows'], { ...none, ...env }), rows = (env: Record<string, string | undefined>) => marked(rowsOf(env).out, 'RUST-ROWS ');
  const DEF: string[] = all.filter(r => S.rustOn(r)), BOTH = ['frozen', 'shipped'] as const, on = (st: string, env: Record<string, string | undefined> = {}) => rows({ TPS_PROFILE: st, ...env });
  const fr = on('frozen'), sh = on('shipped'); let ok = !!fr && !!sh && fr.profile === 'frozen' && sh.profile === 'shipped' && fr.on.join() === DEF.join() && sh.on.join() === DEF.join() && fr.exact === true && sh.exact === true && DEF.length === S.RUST_ON.length && DEF.length > 0; const notes: string[] = [];
  // (NO WORD: each word that was a switch stops the tool that says it, by name, on either stand and whatever it is set to)
  { const other = all.find(r => !DEF.includes(r))!, d0 = DEF[0], word = (r: string) => 'TPS_RUST' + r.toUpperCase(), bad: string[] = [];
    for (const [k, v] of [['TPS_RUST', '0'], ['TPS_EXACT', '0'], ['TPS_EXACT', '1'], [word(d0), '0'], [word(d0), '1'], [word(other), '1']]) for (const st of BOTH) { const c = rowsOf({ TPS_PROFILE: st, [k]: v });
      if (c.status === 0 || marked(c.out, 'RUST-ROWS ') !== null || !c.err.includes(`${k}=${v}: the Rust module's rows and the game's own maths are not switches`)) bad.push(`${k}=${v} on the ${st} stand (exit ${c.status})`); }
    if (bad.length) ok = false; notes.push(bad.length ? `A RETIRED WORD DID NOT STOP THE TOOL: ${bad.join(' · ')}` : 'six retired words, each on both stands, stop the tool by name'); }
  // (A CHECK'S OWN ARM, tools/qa/headless.ts hold, said to a child as QA_HOLD: every row down, a row by name over that, a default row down alone, the maths out)
  { const o = all.find(r => !DEF.includes(r))!, d0 = DEF[0];
    const down = BOTH.map(st => on(st, { QA_HOLD: 'rows=0' })), two = BOTH.map(st => on(st, { QA_HOLD: `rows=0,${d0}=1,${o}=1` })), one = BOTH.map(st => on(st, { QA_HOLD: `${d0}=0` })), eng = BOTH.map(st => on(st, { QA_HOLD: 'exact=0' })), no = BOTH.map(st => rowsOf({ TPS_PROFILE: st, QA_HOLD: 'nosuchrow=0' }));
    const good = down.every(g => !!g && g.on.length === 0 && g.exact === true) && two.every(g => !!g && g.on.join() === all.filter(q => q === d0 || q === o).join()) && one.every(g => !!g && g.on.join() === DEF.filter(q => q !== d0).join() && g.exact === true)
      && eng.every(g => !!g && g.on.join() === DEF.join() && g.exact === false) && no.every(c => c.status !== 0 && /no row "nosuchrow"/.test(c.err));
    if (!good) ok = false; notes.push(`a check's own arm: rows=0 [${down[0]?.on ?? '?'}] [${down[1]?.on ?? '?'}] · with two rows by name [${two[0]?.on ?? '?'}] [${two[1]?.on ?? '?'}] · ${d0}=0 drops ${one.every(g => !!g && g.on.join() === DEF.filter(q => q !== d0).join()) ? 'it alone' : 'NOT it alone'} · exact=0: maths ${eng.map(g => g?.exact === false ? 'the engine\'s' : 'NOT the engine\'s').join(', ')} · a row the module has not: ${no.every(c => c.status !== 0) ? 'an error by name' : 'NOT an error'}`); }
  const P: any = await import(`${ROOT}/src/host/profile.ts`);
  { const more = Object.keys(P.FROZEN_HOLDS).filter(k => k.startsWith('RUST.') || k === 'EXACT.on'); if (more.length) { ok = false; notes.push(`FROZEN_HOLDS still holds [${more}]: the stands do not differ there`); } }
  { const KN: any = await import(`${ROOT}/src/game/knobs.ts`), reg = KN.knobRegistry(); if ('RUST' in reg || 'EXACT' in reg) { ok = false; notes.push('the knob registry still has RUST or EXACT: a recording or a recipe could carry a row'); } }
  // (the ADDRESS's old words, profile.ts pageRows, in this process and put back: whatever a page's address says, and whatever rows a recipe wrote before
  //  it is read, the rows are the build's and the maths the game's own. Each case begins with the maths OUT and every row moved, so a boot that left
  //  either to the address would show; and the page says of each old word, once, as a note and not a warning, that it does nothing)
  { const was: [string, boolean][] = all.map(r => [r, !!S.RUST[r]]), wasX = X.EXACT.on, now = () => all.filter(r => S.RUST[r]).join(), set = (v: boolean) => { for (const r of all) S.rustRow(r, v); };
    const other = all.find(r => !DEF.includes(r))!, d0 = DEF[0], info = console.info, warn = console.warn; let noted = 0, warned = 0, twice = 0;
    const cases: [string, boolean][] = [['', true], ['', false], ['rust=0', true], ['rust=0', false], [`rust=0&rust${other}`, true], [`rust${d0}=0`, true], [`rust${other}`, false], ['noexact', true], ['noexact', false], ['rust=0&noexact', false], ['rustic=1&rustle', true], ['TPS_EXACT=0', false]];
    const bad: string[] = []; console.info = (...a: unknown[]) => { if (/are not switches any more/.test(String(a[0]))) noted++; }; console.warn = () => { warned++; };
    try { for (const [addr, over] of cases) { set(over); X.EXACT.on = false; P.RETIRED_SAID.clear(); const q = new URLSearchParams(addr), before = noted; P.pageRows(q);
        if (now() !== DEF.join() || X.EXACT.on !== true) bad.push(`?${addr} over ${over ? 'every row up' : 'every row down'} and the maths out: [${now()}] maths ${X.EXACT.on}`);
        const old = [...q.keys()].filter(k => k === 'rust' || k === 'noexact' || (k.startsWith('rust') && all.includes(k.slice(4)))).length; if (noted - before !== old) bad.push(`?${addr}: ${noted - before} notes for ${old} old word(s)`);
        P.pageRows(q); if (noted - before !== old) twice++; } } finally { console.info = info; console.warn = warn; }
    for (const [r, v] of was) S.rustRow(r, v); X.EXACT.on = wasX; P.RETIRED_SAID.clear();
    if (bad.length || warned || twice) ok = false;
    notes.push(bad.length ? `THE ADDRESS: ${bad.join(' · ')}` : `the address's old words: ${cases.length} cases, each from the maths out and the rows moved: the build's rows and the game's own maths after each, a note for each old word and none for a word that only begins \`rust\` (${noted} notes, ${warned} warnings${twice ? `, SAID AGAIN on a second reading ${twice} times` : ', none said twice'})`); }
  verdict(ok, 'stand: both stands have the rows of RUST_ON on and the game\'s own maths, as the page; no word of the environment or of an address moves either (a retired word stops the tool that says it); a check\'s own arm holds what it names and it alone', `frozen [${fr?.on ?? '?'}] maths ${fr?.exact} · shipped [${sh?.on ?? '?'}] maths ${sh?.exact} · ${notes.join(' · ')} · rows: ${all.join(' ')}`);
}

// ---- views: sim.ts keeps its views over the module's memory and tests their length; a memory that grew must leave every one of them remade (nothing in the
//      module allocates, so only this line ever grows it — in this process alone, after every section that asks the module here)
{
  const before: Float64Array = S.f64(), had = before.length; let grew = -1, why = ''; try { grew = mod.memory.grow(1); } catch (e) { why = String((e as Error)?.message ?? e); }
  const b = mod.memory.buffer, after: Float64Array = S.f64();
  const ok = grew >= 0 && had > 0 && before.length === 0 && after !== before && after.length * 8 === b.byteLength && [after, S.u8(), S.u32(), S.i32(), S.f32()].every(v => v.buffer === b && v.length > 0) && S.simOwns(b) && !S.simOwns(before.buffer);
  verdict(ok, 'views: the kept views over the module\'s memory are remade when the memory has grown', grew < 0 ? `the memory would not grow here (${why || 'memory.grow said -1'}): nothing was held`
    : `grown by a page from ${grew}: the old view holds ${before.length} doubles (it had ${had}), the new ones ${after.length} over the module's buffer · simOwns the new buffer ${S.simOwns(b)}, the old ${S.simOwns(before.buffer)}`);
}

console.log(fails ? `\n${fails} FAILED` : '\nall pass');
const green = BREAK ? RED_SECTIONS.filter(k => said[k] !== false) : [];
if (BREAK) console.log(green.length ? `THE RED PATH IS NOT RED FOR ${green.map(k => `the section \`${k}\``).join(', ')}` : `the red path is red for every section it must be (${RED_SECTIONS.join(' · ')})`);
process.exit(green.length ? 3 : fails ? 1 : 0);
