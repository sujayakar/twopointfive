// THE SIMULATION IN RUST (rust/sim → src/sim/built.ts, loaded by src/sim/sim.ts): is the module the build of its sources, does it load, and is each system
// that runs in it the SAME GAME TO THE BIT as the TypeScript it stands in for?
//   bun run tools/qa/rust-check.ts                       the quick half (seconds): every check below but `missions`; exit 0 = all pass, 1 = one failed (named)
//   bun run tools/qa/rust-check.ts --missions            … and whole missions played both ways (minutes: five scripted squad runs, twice each, two at a time)
//   bun run tools/qa/rust-check.ts --missions --node     the missions under node (V8, the page's engine) instead of bun
//   bun run tools/qa/rust-check.ts --runs compound-four:1   other runs for `missions` (tools/qa/playthrough.ts's scenarios)
//   RUST_BREAK=1 bun run tools/qa/rust-check.ts [--missions]   the red path: the Rust side is handed the ENGINE's cosine where the game's own belongs — `maths`,
//                                                        `engines`, `server` and `missions` must then FAIL
// The page's half (Chromium, a dev server) is tools/qa/rust-page.cjs.
// WHAT IS HELD:
//   built     src/sim/built.ts is the build of rust/sim as it stands (the hash of the sources; where there is a Rust compiler, the bytes built again), is
//             small enough for a page to compile in one step, and carries no path of the machine that built it (tools/sim/build.ts --check)
//   face      the module's exports are the ones src/sim/sim.ts names, no more and no fewer, at the version it was written against; it starts in one step
//   maths     one fixed stream of inputs through the fourteen functions of src/math/exact.ts and through the Rust ones (rust/sim/src/maths.rs): every single
//             answer the same bits (zeros by sign, NaN as NaN), the decimal powers log10 leans on among them
//   engines   the Rust functions fold that stream to the same word under bun and under node — and it is the word the TypeScript functions fold to
//   switch    RUST.maths with the game's own maths on puts the Rust functions in Math's place and under the lengths of math/hyp.ts; off puts the TypeScript
//             ones back; with the game's own maths off the engine's stay whatever the row says
//   stand     both stands leave every row off (TPS_RUSTMATHS=1 raises the maths row on either)
//   server    the game server's own stand (tools/mp/stand.ts): a setup that asks for the row makes a recipe that carries it, the game stood from that recipe in
//             another process ticks with the module's maths in Math's place, and after 600 ticks of a walking, shooting seat its state word is the word of
//             the same setup without the row
//   missions  (--missions) each run played twice on the server's stand (shipped, the senses on the CPU), once with every sin, cos, atan2, exp … and every
//             length the update asks for worked out in Rust and once in TypeScript: the same ticks, the same end, and the same chained raw bits of the state
//             after EVERY update (game/digest.ts rawBits)
// Not game code: it reads. Nothing here needs a GPU, and nothing here has been seen on one.
import { resolve } from 'node:path';
import { spawnSync, spawn } from 'node:child_process';
const ROOT = resolve(import.meta.dir ?? new URL('.', import.meta.url).pathname, '../..');
const args = process.argv.slice(2);
const flag = (n: string): string | undefined => { const i = args.indexOf(n); return i >= 0 ? args[i + 1] : undefined; };
const BREAK = process.env.RUST_BREAK === '1';
const RUNS = ['compound-four:1', 'warehouse-four:1', 'pass-four:1', 'hostage-rescue:1', 'block-squads:1'];
const NAMES = ['sin', 'cos', 'tan', 'asin', 'acos', 'atan', 'atan2', 'exp', 'log', 'log2', 'log10', 'pow', 'tanh', 'hypot'] as const;
type Fns = Record<string, (...a: number[]) => number>;
const engine = (globalThis as any).Bun ? `bun ${(globalThis as any).Bun.version} (JavaScriptCore)` : `node ${process.version} (V8 ${process.versions.v8})`;

/** THE RED PATH of a whole game: the row put down and the Rust set handed over with the ENGINE's cosine in it — a wrong arithmetic that still plays */
function wrongMaths(S: any, X: any): void {
  S.RUST.maths = false; const x = S.sim();
  X.mathsFrom({ ...X.SYS, sin: x.maths_sin, cos: X.SYS.cos, tan: x.maths_tan, asin: x.maths_asin, acos: x.maths_acos, atan: x.maths_atan, atan2: x.maths_atan2, exp: x.maths_exp, log: x.maths_log, log2: x.maths_log2, log10: x.maths_log10, pow: x.maths_pow, tanh: x.maths_tanh, hypot: X.hypot, hyp2: x.maths_hypot2, hyp3: x.maths_hypot3, hyp4: x.maths_hypot4 });
}
// ---- `missions`, one run in THIS process (a child): the update tapped, the raw bits chained after every tick
if (args[0] === '--run') {
  process.env.TPS_PROFILE ??= 'shipped'; process.env.TPS_CPUSENSE ??= '1'; process.env.PT_DIGEST ??= '1';
  const [scn, seed] = args[1].split(':');
  const { Game } = await import(`${ROOT}/src/game/game.ts`);
  const PT: any = await import(`${ROOT}/tools/qa/playthrough.ts`);
  const { rawBits, RAW_BITS_SEED } = await import(`${ROOT}/src/game/digest.ts`);
  const S: any = await import(`${ROOT}/src/sim/sim.ts`), X: any = await import(`${ROOT}/src/math/exact.ts`);
  let h: number = RAW_BITS_SEED, ticks = 0, broke = false, inRust = 0;   // (inRust: the ticks whose update found the module's sine in Math's place — the play-through puts every knob table back when it ends, so the row is read while it runs)
  const update = Game.prototype.update;
  Game.prototype.update = function (this: unknown, ...a: unknown[]) {
    // (the red path: from the first tick the Rust set is handed the engine's cosine — a wrong arithmetic that still plays)
    if (BREAK && !broke && S.RUST.maths) { broke = true; wrongMaths(S, X); }
    if (broke || Math.sin === S.sim().maths_sin) inRust++;
    const r = (update as (...x: unknown[]) => unknown).apply(this, a); h = rawBits(this as never, h); ticks++; return r;
  } as typeof update;
  const t0 = performance.now();
  const r = await PT.play(scn, Number(seed), 300, false, {});
  console.log('RUST-RUN ' + JSON.stringify({ run: args[1], ticks, end: String(r.end), digest: String(r.digest), bits: h.toString(36), rust: inRust, exact: X.EXACT.on, engine, secs: Math.round((performance.now() - t0) / 100) / 10 }));
  process.exit(0);
}
// ---- `server`, in a child: the game server's stand from a recipe (argv), a seat's hands by the beat, the word after `ticks`
if (args[0] === '--server') {
  const { standRecipe }: any = await import(`${ROOT}/tools/mp/stand.ts`), { sampleCmd }: any = await import(`${ROOT}/src/game/cmd.ts`);
  const S: any = await import(`${ROOT}/src/sim/sim.ts`), X: any = await import(`${ROOT}/src/math/exact.ts`);
  const st = await standRecipe(JSON.parse(args[1])), W = st.W, I = W.input, ticks = Number(args[2]);
  let inRust = 0; const broke = BREAK && S.RUST.maths; if (broke) wrongMaths(S, X);
  for (let n = 0; n < ticks; n++) {
    I.keys.clear(); I.pressed.clear(); I.clicked = 0; I.buttons = 0; I.wheel = 0;
    I.keys.add(['KeyW', 'KeyD', 'KeyS', 'KeyA'][Math.floor(n / 90) % 4]);
    I.mouseX = W.canvas.clientWidth / 2 + X.cos(n * 0.05) * 260; I.mouseY = W.canvas.clientHeight / 2 + X.sin(n * 0.05) * 180;   // (the game's own maths by name: the same hands whichever stand in Math's place)
    if (n > 120 && n % 40 < 4) { I.buttons = 1; if (n % 40 === 0) I.clicked = 1; }
    if (broke || (!S.simWhyNot() && Math.sin === S.sim().maths_sin)) inRust++;
    st.step([sampleCmd(W.game, I, W.cam, W.canvas)]);
  }
  console.log('RUST-SERVER ' + JSON.stringify({ ticks: st.ticks, word: st.word(), rust: inRust, row: S.RUST.maths, exact: X.EXACT.on }));
  process.exit(0);
}
// ---- `stand`, in a child: what a stand leaves the rows at
if (args[0] === '--rows') {
  await import(`${ROOT}/tools/qa/headless.ts`);
  const H: any = await import(`${ROOT}/src/host/standup.ts`), S: any = await import(`${ROOT}/src/sim/sim.ts`), P: any = await import(`${ROOT}/src/host/profile.ts`);
  H.pinMaths(); H.pinRust();
  console.log('RUST-ROWS ' + JSON.stringify({ profile: P.harnessProfile(), maths: S.RUST.maths, rows: Object.keys(S.RUST) }));
  process.exit(0);
}

const X: any = await import(`${ROOT}/src/math/exact.ts`);
const S: any = await import(`${ROOT}/src/sim/sim.ts`);
const HY: any = await import(`${ROOT}/src/math/hyp.ts`);
const SYS: Fns = X.SYS;
const own: Fns = {}; for (const n of NAMES) own[n] = X[n];
const mod = S.sim();
// ---- one fixed stream of inputs through both (tools/qa/rust-fold.js: the page's check folds the same stream)
const { fold, sameBits, FOLD_N }: { fold(fns: Fns, n?: number, other?: Fns | null): { word: string; asked: number; differ: number; first: string[] }; sameBits(a: number, b: number): boolean; FOLD_N: number } = await import(`${ROOT}/tools/qa/rust-fold.js`);
const rust: Fns = { ...S.mathsOfRust() };   // (the set the row installs, not one made here)
if (BREAK) rust.cos = SYS.cos;
const N = FOLD_N;

if (args[0] === '--hash') {   // the child: this engine's word for the Rust functions
  console.log('RUST-HASH ' + JSON.stringify({ engine, word: fold(rust, N).word }));
  process.exit(0);
}

let fails = 0;
const verdict = (ok: boolean, label: string, detail: string) => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
const child = (cmd: string[], env: Record<string, string | undefined> = {}): { out: string; err: string; status: number | null; missing: boolean } => {
  const e: Record<string, string | undefined> = { ...process.env, ...env }; for (const k of Object.keys(e)) if (e[k] === undefined) delete e[k];
  const r = spawnSync(cmd[0], cmd.slice(1), { cwd: ROOT, env: e as Record<string, string>, encoding: 'utf8', maxBuffer: 1 << 26 });
  return { out: r.stdout ?? '', err: r.stderr ?? '', status: r.status, missing: !!r.error && (r.error as { code?: string }).code === 'ENOENT' };
};
const marked = (out: string, mark: string): any => { const l = out.split('\n').find(x => x.startsWith(mark)); return l ? JSON.parse(l.slice(mark.length)) : null; };
const NODE = ['node', '--no-warnings', '--import', './tools/qa/framecost-v8.mjs'];
console.log(`the simulation in Rust · ${engine} · module ${S.sim().memory.buffer.byteLength / 65536} pages of memory, started in ${S.simBootMs.toFixed(1)} ms${BREAK ? ' · RUST_BREAK=1 (the red path)' : ''}`);

// ---- built
{ const B: any = await import(`${ROOT}/tools/sim/build.ts`); for (const v of B.checkBuilt()) verdict(v.ok, `built · ${v.label}`, v.detail); }

// ---- face
{
  const names = WebAssembly.Module.exports(new WebAssembly.Module(Uint8Array.from(atob((await import(`${ROOT}/src/sim/built.ts`) as any).SIM_WASM.join('')), c => c.charCodeAt(0)))).map(e => e.name).sort();
  const want = [...S.SIM_EXPORTS].sort() as string[];
  const extra = names.filter(n => !want.includes(n)), gone = want.filter(n => !names.includes(n));
  verdict(extra.length === 0 && gone.length === 0 && mod.sim_abi() === S.SIM_ABI, 'face: the module exports what src/sim/sim.ts names, at its version',
    `${names.length} exports, version ${mod.sim_abi()} (sim.ts: ${S.SIM_ABI})${extra.length ? ` · in the module and not in sim.ts: ${extra.join(' ')}` : ''}${gone.length ? ` · in sim.ts and not in the module: ${gone.join(' ')}` : ''}`);
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

// ---- engines
{
  const node = child([...NODE, 'tools/qa/rust-check.ts', '--hash']);
  const there = marked(node.out, 'RUST-HASH ');
  const label = 'engines: the Rust functions fold the stream to one word under bun and under node, the TypeScript functions\' word';
  if (node.missing) verdict(false, label, `no node on this machine: ${engine} folds ${wordRust}, and nothing was compared`);
  else if (!there) verdict(false, label, `the node child did not answer (exit ${node.status}): ${node.err.trim().split('\n').slice(-2).join(' / ')}`);
  else verdict(there.word === wordRust && wordRust === wordOwn, label, `${engine}: ${wordRust} · ${there.engine}: ${there.word} · TypeScript here: ${wordOwn}`);
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
  const rows = (env: Record<string, string | undefined>) => marked(child(['bun', 'run', 'tools/qa/rust-check.ts', '--rows'], { TPS_PROFILE: undefined, TPS_RUSTMATHS: undefined, TPS_EXACT: undefined, ...env }).out, 'RUST-ROWS ');
  const fr = rows({}), sh = rows({ TPS_PROFILE: 'shipped' }), frUp = rows({ TPS_RUSTMATHS: '1' }), shUp = rows({ TPS_PROFILE: 'shipped', TPS_RUSTMATHS: '1' });
  const ok = !!fr && !!sh && !!frUp && !!shUp && fr.profile === 'frozen' && sh.profile === 'shipped' && fr.maths === false && sh.maths === false && frUp.maths === true && shUp.maths === true;
  verdict(ok, 'stand: both stands leave every row off, and TPS_RUSTMATHS=1 raises the maths row on either', `frozen ${JSON.stringify(fr?.maths)} · shipped ${JSON.stringify(sh?.maths)} · with the word: frozen ${JSON.stringify(frUp?.maths)} · shipped ${JSON.stringify(shUp?.maths)} · rows: ${(fr?.rows ?? []).join(' ')}`);
}

// ---- server
{
  const TICKS = 600, setup = { search: '?roomclear&squad4', seed: 7 }, CLEAN = { TPS_RUSTMATHS: undefined };   // (the row comes from the recipe here, never from a word of the caller's shell)
  const recipeOf = (tables: object | null) => marked(child(['bun', 'run', 'tools/mp/setup.ts', JSON.stringify(tables ? { ...setup, tables } : setup)], CLEAN).out, 'MPSETUP ');
  const stood = (recipe: unknown) => recipe ? marked(child(['bun', 'run', 'tools/qa/rust-check.ts', '--server', JSON.stringify(recipe), String(TICKS)], CLEAN).out, 'RUST-SERVER ') : null;
  const rOn = recipeOf({ RUST: { maths: true } }), rOff = recipeOf(null), on = stood(rOn), off = stood(rOff);
  const carried = (r: any) => r?.tables?.RUST?.maths ?? r?.knobs?.tables?.RUST?.maths;
  const ok = !!on && !!off && carried(rOn) === true && carried(rOff) === false && on.ticks === TICKS && off.ticks === TICKS && on.rust === TICKS && off.rust === 0 && on.exact && off.exact && on.word === off.word;
  verdict(ok, 'server: the game server\'s stand takes the row from its recipe, ticks on the module\'s maths, and is the game it is without', !on || !off ? `a stand left nothing (recipes: ${!!rOn} ${!!rOff} · stands: ${!!on} ${!!off})` :
    `the recipe carries the row ${JSON.stringify(carried(rOn))} · ${on.rust} of ${on.ticks} ticks on the module's maths (without: ${off.rust} of ${off.ticks}) · word ${on.word}${on.word === off.word ? ', the same' : ` ≠ ${off.word}`}`);
}

// ---- missions
if (args.includes('--missions')) {
  const runs = (flag('--runs') ?? RUNS.join(',')).split(','), node = args.includes('--node');
  interface Run { run: string; ticks: number; end: string; digest: string; bits: string; rust: number; exact: boolean; engine: string; secs: number }
  const one = (run: string, rustOn: boolean): Promise<Run | string> => new Promise(done => {
    const cmd = node ? [...NODE, 'tools/qa/rust-check.ts', '--run', run] : ['bun', 'run', 'tools/qa/rust-check.ts', '--run', run];
    const env: Record<string, string | undefined> = { ...process.env, TPS_PROFILE: 'shipped', TPS_CPUSENSE: '1', PT_DIGEST: '1', TPS_RUSTMATHS: rustOn ? '1' : '0', ...(node ? { V8_ENTRY: resolve(ROOT, 'tools/qa/rust-check.ts'), FC_NOSTANDIN: '1' } : {}) };
    const p = spawn(cmd[0], cmd.slice(1), { cwd: ROOT, env: env as Record<string, string> }); let out = '', err = '';
    p.stdout.on('data', (d: unknown) => { out += d; }); p.stderr.on('data', (d: unknown) => { err += d; });
    p.on('close', code => { const r = marked(out, 'RUST-RUN '); done(r ?? `left nothing (exit ${code}): ${err.trim().split('\n').slice(-3).join(' / ').slice(0, 300)}`); });
    p.on('error', e => done(String(e)));
  });
  let ok = true; const lines: string[] = []; let ticks = 0;
  for (const run of runs) {
    const [ts, rs] = await Promise.all([one(run, false), one(run, true)]);
    if (typeof ts === 'string' || typeof rs === 'string') { ok = false; lines.push(`${run}: ${typeof ts === 'string' ? `TypeScript ${ts}` : ''} ${typeof rs === 'string' ? `Rust ${rs}` : ''}`); continue; }
    const same = ts.bits === rs.bits && ts.ticks === rs.ticks && ts.end === rs.end && ts.digest === rs.digest, honest = rs.rust === rs.ticks && ts.rust === 0 && ts.ticks > 600;
    if (!same || !honest) ok = false; ticks += ts.ticks;
    lines.push(`${run}: ${ts.ticks} ticks, ends ${ts.end} · ${same ? `to the bit (${ts.bits})` : `DIFFERS — TypeScript ${ts.ticks} ticks ${ts.bits} ${ts.digest}, Rust ${rs.ticks} ticks ${rs.bits} ${rs.digest}`}${honest ? '' : ` · NOT A FAIR PAIR (ticks on the Rust maths: ${rs.rust} of ${rs.ticks} in the Rust run, ${ts.rust} of ${ts.ticks} in the other)`} · ${ts.secs} s / ${rs.secs} s`);
    console.error(`  ${lines[lines.length - 1]}`);
  }
  verdict(ok, `missions: ${runs.length} runs played on the Rust maths and on the TypeScript maths are the same game after every update (${node ? 'node' : 'bun'})`, `${ticks} ticks compared\n    ${lines.join('\n    ')}`);
}

console.log(fails ? `\n${fails} FAILED` : '\nall pass');
process.exit(fails ? 1 : 0);
