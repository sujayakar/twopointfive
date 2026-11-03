// THE SIMULATION IN RUST (rust/sim → src/sim/built.ts, loaded by src/sim/sim.ts): is the module the build of its sources, does it load, and is each system
// that runs in it the SAME GAME TO THE BIT as the TypeScript it stands in for?
//   bun run tools/qa/rust-check.ts                       the quick half (seconds): every check below but `missions`; exit 0 = all pass, 1 = one failed (named)
//   bun run tools/qa/rust-check.ts --missions            … and whole missions played both ways (minutes: five scripted squad runs, twice each, two at a time)
//   bun run tools/qa/rust-check.ts --missions --node     the missions under node (V8, the page's engine) instead of bun
//   bun run tools/qa/rust-check.ts --runs compound-four:1   other runs for `missions` (tools/qa/playthrough.ts's scenarios)
//   bun run tools/qa/rust-check.ts --missions --rows lines  the missions with THESE rows raised against none (default: every row at once)
//   bun run tools/qa/rust-check.ts --both                the missions once more with every question the box world's rows send to the module answered both ways (slow: minutes a run)
//   RUST_BREAK=1 bun run tools/qa/rust-check.ts [--missions] [--both]   the red path: the Rust side is handed the ENGINE's cosine where the game's own belongs, and
//                                                        the box world's copy is not told what was written — `maths`, `engines`, `lines`, `server`, `missions`
//                                                        and `both` must then FAIL
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
//   lines     the box world's line tests (rust/sim/src/lines.rs over the copy src/scene/boxMirror.ts keeps): tools/qa/segment-ident.ts --rust — random segments
//             over whole levels (door leaves swung, walls broken into pieces and re-shaped, tilted and cut boxes, panes with holes) answered by the module,
//             by the TypeScript walk as it is and by the walk as it first was: every answer and every pane's place the same bits
//   circles   a circle against the box world (rust/sim/src/collide.rs, the same copy): the same run — a body's footprint pushed out, what it touches, what
//             is gathered under it: where it ends to the bit, and every list the same
//   fog       the fog of war's sight pass (rust/sim/src/fog.rs: which eyes have a line to a man, to a body) has no stream of its own: its questions are a
//             mission's, so it is held by `missions` and by `both` (every man asked of the module is asked of fogofwar.ts too, the same eyes in the same order)
//   poses     the posing functions (rust/sim/src/skeleton.rs against src/anim/skeleton.ts): tools/qa/pose-ident.ts — the game's own rig and clips blended,
//             run forward, turned and solved, each question asked of both: every single of every pose and every double of every run the same bits
//   stand     both stands leave every row off (TPS_RUST<ROW>=1 raises a row on either)
//   server    the game server's own stand (tools/mp/stand.ts): a setup that asks for the row makes a recipe that carries it, the game stood from that recipe in
//             another process ticks with the module's maths in Math's place, and after 600 ticks of a walking, shooting seat its state word is the word of
//             the same setup without the row
//   missions  (--missions) each run played twice on the server's stand (shipped, the senses on the CPU), once with every sin, cos, atan2, exp … and every
//             length the update asks for worked out in Rust, every line through the box world asked of the module, and once in TypeScript: the same ticks,
//             the same end, and the same chained raw bits of the state after EVERY update (game/digest.ts rawBits)
//   both      (--both) each run played with the rows on and collision.ts BOTH up: every question the module is sent is answered by the TypeScript too and the
//             two compared, and at every question every row of the module's copy is compared with its box — no answer differs, no row is stale
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
/** …and of the box world's copy: what a rule writes inside a tick goes unsaid (the doors' swing among it), so the module answers for boxes as they were */
function unsaid(C: any, M: any): void {
  const P = C.StaticCollision.prototype, eye = P.sightEye; P.wrote = function () {};
  Object.defineProperty(M.LEAVES, 'n', { get: () => 0, set: () => {} });   // (a leaf written moves no count: the copy of the leaves made at a tick's first question stands for all of it)
  P.sightEye = function (this: any, k: number, pos: unknown, look: unknown, len: number, cos: number, near: number) { eye.call(this, k, pos, look, len, cos, near, true); };   // (…and of the fog's pass: every eye is handed over as seeing all round)
}
/** the ticks that opened with the box world's line tests, and its circles, going to the module (a row can be on and its copy not standing: a level too big for the tables) */
function countLines(C: any): { n: number; circles: number; fog: number; sees: number } {
  const k = { n: 0, circles: 0, fog: 0, sees: 0 }, P = C.StaticCollision.prototype, open = P.tickOpen, see = P.eyesSee;
  P.tickOpen = function (this: any) { open.call(this); if (this.mirror.lines) k.n++; if (this.mirror.circles) k.circles++; if (this.mirror.sight) k.fog++; };
  P.eyesSee = function (this: any, ...a: unknown[]) { const r = see.apply(this, a); if (r >= 0) k.sees++; return r; };   // (the men and bodies the fog's pass asked of the module)
  return k;
}
// ---- `missions`, one run in THIS process (a child): the update tapped, the raw bits chained after every tick
if (args[0] === '--run') {
  process.env.TPS_PROFILE ??= 'shipped'; process.env.TPS_CPUSENSE ??= '1'; process.env.PT_DIGEST ??= '1';
  const [scn, seed] = args[1].split(':');
  const { Game } = await import(`${ROOT}/src/game/game.ts`);
  const PT: any = await import(`${ROOT}/tools/qa/playthrough.ts`);
  const { rawBits, RAW_BITS_SEED } = await import(`${ROOT}/src/game/digest.ts`);
  const S: any = await import(`${ROOT}/src/sim/sim.ts`), X: any = await import(`${ROOT}/src/math/exact.ts`), C: any = await import(`${ROOT}/src/scene/collision.ts`);
  const lines = countLines(C); if (BREAK) unsaid(C, await import(`${ROOT}/src/scene/boxMirror.ts`)); C.BOTH.on = process.env.RUST_BOTH === '1';
  const SK: any = await import(`${ROOT}/src/anim/skeleton.ts`);
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
  console.log('RUST-RUN ' + JSON.stringify({ run: args[1], ticks, end: String(r.end), digest: String(r.digest), bits: h.toString(36), rust: inRust, lines: lines.n, circles: lines.circles, fog: lines.fog, sees: lines.sees, poses: SK.POSE_BENCH.asked, both: C.BOTH, exact: X.EXACT.on, engine, secs: Math.round((performance.now() - t0) / 100) / 10 }));
  process.exit(0);
}
// ---- `server`, in a child: the game server's stand from a recipe (argv), a seat's hands by the beat, the word after `ticks`
if (args[0] === '--server') {
  const { standRecipe }: any = await import(`${ROOT}/tools/mp/stand.ts`), { sampleCmd }: any = await import(`${ROOT}/src/game/cmd.ts`);
  const S: any = await import(`${ROOT}/src/sim/sim.ts`), X: any = await import(`${ROOT}/src/math/exact.ts`), C: any = await import(`${ROOT}/src/scene/collision.ts`);
  const lines = countLines(C);
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
  console.log('RUST-SERVER ' + JSON.stringify({ ticks: st.ticks, word: st.word(), rust: inRust, lines: lines.n, circles: lines.circles, fog: lines.fog, sees: lines.sees, row: S.RUST.maths, exact: X.EXACT.on }));
  process.exit(0);
}
// ---- `stand`, in a child: what a stand leaves the rows at
if (args[0] === '--rows') {
  await import(`${ROOT}/tools/qa/headless.ts`);
  const H: any = await import(`${ROOT}/src/host/standup.ts`), S: any = await import(`${ROOT}/src/sim/sim.ts`), P: any = await import(`${ROOT}/src/host/profile.ts`);
  H.pinMaths(); H.pinRust();
  console.log('RUST-ROWS ' + JSON.stringify({ profile: P.harnessProfile(), maths: S.RUST.maths, on: S.rustRows().filter((r: string) => S.RUST[r]), rows: S.rustRows() }));
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

// ---- engines
{
  const node = child([...NODE, 'tools/qa/rust-check.ts', '--hash']);
  const there = marked(node.out, 'RUST-HASH ');
  const label = 'engines: the Rust functions fold the stream to one word under bun and under node, the TypeScript functions\' word';
  if (node.missing) verdict(false, label, `no node on this machine: ${engine} folds ${wordRust}, and nothing was compared`);
  else if (!there) verdict(false, label, `the node child did not answer (exit ${node.status}): ${node.err.trim().split('\n').slice(-2).join(' / ')}`);
  else verdict(there.word === wordRust && wordRust === wordOwn, label, `${engine}: ${wordRust} · ${there.engine}: ${there.word} · TypeScript here: ${wordOwn}`);
}

// ---- lines
{
  const r = child(['bun', 'run', 'tools/qa/segment-ident.ts', '--rust', '--n', flag('--segments') ?? '40000', '--levels', flag('--levels') ?? 'office.json,warehouse.json,compound.json,glasshouse.json']);
  const rows = r.out.split('\n').filter(l => /segments \(/.test(l)), ok = r.status === 0 && rows.length > 0 && rows.every(l => / PASS /.test(l));
  verdict(ok, 'lines, circles: every line through the box world and every circle against it gets the TypeScript\'s answer from the Rust module', rows.length ? rows.map(l => l.replace(/\s+/g, ' ').trim()).join('\n    ') : `the differential left nothing (exit ${r.status}): ${(r.err || r.out).trim().split('\n').slice(-2).join(' / ').slice(0, 300)}`);
}

// ---- poses
{
  const r = child(['bun', 'run', 'tools/qa/pose-ident.ts', '--n', flag('--poses') ?? '4000']);
  const row = r.out.split('\n').find(l => /^pose-ident: \d+ rounds/.test(l)), ok = r.status === 0 && !!row && /=== pose-ident: PASS/.test(r.out);
  verdict(ok, 'poses: every pose the Rust module blends, runs forward, turns and solves is the TypeScript\'s, single for single', row ? row.replace(/^pose-ident: /, '') : `the differential left nothing (exit ${r.status}): ${(r.err || r.out).trim().split('\n').slice(-2).join(' / ').slice(0, 300)}`);
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
  const all: string[] = S.rustRows(), word = (r: string) => 'TPS_RUST' + r.toUpperCase(), none: Record<string, undefined> = { TPS_PROFILE: undefined, TPS_EXACT: undefined }; for (const r of all) none[word(r)] = undefined;
  const rows = (env: Record<string, string | undefined>) => marked(child(['bun', 'run', 'tools/qa/rust-check.ts', '--rows'], { ...none, ...env }).out, 'RUST-ROWS ');
  const fr = rows({}), sh = rows({ TPS_PROFILE: 'shipped' }); let ok = !!fr && !!sh && fr.profile === 'frozen' && sh.profile === 'shipped' && fr.on.length === 0 && sh.on.length === 0; const notes: string[] = [];
  for (const r of all) { const a = rows({ [word(r)]: '1' }), b = rows({ TPS_PROFILE: 'shipped', [word(r)]: '1' }), good = !!a && !!b && a.on.join() === r && b.on.join() === r; if (!good) ok = false; notes.push(`${word(r)}=1: frozen [${a?.on ?? '?'}] shipped [${b?.on ?? '?'}]`); }
  verdict(ok, 'stand: both stands leave every row off, and a row\'s own word raises it — and it alone — on either', `frozen [${fr?.on ?? '?'}] · shipped [${sh?.on ?? '?'}] · ${notes.join(' · ')} · rows: ${all.join(' ')}`);
}

// ---- server
{
  const TICKS = 600, setup = { search: '?roomclear&squad4', seed: 7 }, CLEAN: Record<string, undefined> = {}; for (const r of S.rustRows()) CLEAN['TPS_RUST' + r.toUpperCase()] = undefined;   // (the row comes from the recipe here, never from a word of the caller's shell)
  const recipeOf = (tables: object | null) => marked(child(['bun', 'run', 'tools/mp/setup.ts', JSON.stringify(tables ? { ...setup, tables } : setup)], CLEAN).out, 'MPSETUP ');
  const stood = (recipe: unknown) => recipe ? marked(child(['bun', 'run', 'tools/qa/rust-check.ts', '--server', JSON.stringify(recipe), String(TICKS)], CLEAN).out, 'RUST-SERVER ') : null;
  const rOn = recipeOf({ RUST: Object.fromEntries(S.rustRows().map((r: string) => [r, true])) }), rOff = recipeOf(null), on = stood(rOn), off = stood(rOff);
  const table = (r: any) => r?.tables?.RUST ?? r?.knobs?.tables?.RUST, carried = (r: any) => S.rustRows().every((k: string) => table(r)?.[k] === true), bare = (r: any) => S.rustRows().every((k: string) => table(r)?.[k] === false);
  const ok = !!on && !!off && carried(rOn) && bare(rOff) && on.ticks === TICKS && off.ticks === TICKS && on.rust === TICKS && off.rust === 0 && on.lines === TICKS && off.lines === 0 && on.circles === TICKS && off.circles === 0 && on.fog === TICKS && off.fog === 0 && on.exact && off.exact && on.word === off.word;
  verdict(ok, 'server: the game server\'s stand takes the rows from its recipe, ticks on the module\'s maths, lines, circles and fog, and is the game it is without', !on || !off ? `a stand left nothing (recipes: ${!!rOn} ${!!rOff} · stands: ${!!on} ${!!off})` :
    `the recipe carries the rows ${JSON.stringify(table(rOn))} · ${on.rust} of ${on.ticks} ticks on the module's maths, ${on.lines} on its lines, ${on.circles} on its circles and ${on.fog} with its sight pass (without: ${off.rust}, ${off.lines}, ${off.circles} and ${off.fog} of ${off.ticks}) · word ${on.word}${on.word === off.word ? ', the same' : ` ≠ ${off.word}`}`);
}

// ---- missions, both
if (args.includes('--missions') || args.includes('--both')) {
  const runs = (flag('--runs') ?? RUNS.join(',')).split(','), node = args.includes('--node'), all: string[] = S.rustRows(), up = (flag('--rows') ?? all.join(',')).split(',');
  for (const r of up) if (!all.includes(r)) { console.error(`rust-check: no row "${r}" (rows: ${all.join(' ')})`); process.exit(2); }
  interface Run { run: string; ticks: number; end: string; digest: string; bits: string; rust: number; lines: number; circles: number; fog: number; sees: number; poses: number; both: { asked: number; differ: number; stale: number; first: string }; exact: boolean; engine: string; secs: number }
  const one = (run: string, rustOn: boolean, both = false): Promise<Run | string> => new Promise(done => {
    const cmd = node ? [...NODE, 'tools/qa/rust-check.ts', '--run', run] : ['bun', 'run', 'tools/qa/rust-check.ts', '--run', run];
    const env: Record<string, string | undefined> = { ...process.env, TPS_PROFILE: 'shipped', TPS_CPUSENSE: '1', PT_DIGEST: '1', RUST_BOTH: both ? '1' : '0', ...(node ? { V8_ENTRY: resolve(ROOT, 'tools/qa/rust-check.ts'), FC_NOSTANDIN: '1' } : {}) };
    for (const r of all) env['TPS_RUST' + r.toUpperCase()] = rustOn && up.includes(r) ? '1' : '0';
    const p = spawn(cmd[0], cmd.slice(1), { cwd: ROOT, env: env as Record<string, string> }); let out = '', err = '';
    p.stdout.on('data', (d: unknown) => { out += d; }); p.stderr.on('data', (d: unknown) => { err += d; });
    p.on('close', code => { const r = marked(out, 'RUST-RUN '); done(r ?? `left nothing (exit ${code}): ${err.trim().split('\n').slice(-3).join(' / ').slice(0, 300)}`); });
    p.on('error', e => done(String(e)));
  });
  /** did the run with the rows up really run on them: every tick on the module's maths and with its lines asked there (each where its row was raised), and none in the other */
  const fair = (ts: Run, rs: Run) => (!up.includes('maths') || rs.rust === rs.ticks) && (!up.includes('lines') || rs.lines === rs.ticks) && (!up.includes('circles') || rs.circles === rs.ticks) && (!up.includes('fog') || (rs.fog === rs.ticks && rs.sees > 100)) && ts.rust === 0 && ts.lines === 0 && ts.circles === 0 && (!up.includes('poses') || rs.poses > rs.ticks) && ts.fog === 0 && ts.sees === 0 && ts.poses === 0 && ts.ticks > 600;
  if (args.includes('--missions')) {
    let ok = true; const lines: string[] = []; let ticks = 0;
    for (const run of runs) {
      const [ts, rs] = await Promise.all([one(run, false), one(run, true)]);
      if (typeof ts === 'string' || typeof rs === 'string') { ok = false; lines.push(`${run}: ${typeof ts === 'string' ? `TypeScript ${ts}` : ''} ${typeof rs === 'string' ? `Rust ${rs}` : ''}`); continue; }
      const same = ts.bits === rs.bits && ts.ticks === rs.ticks && ts.end === rs.end && ts.digest === rs.digest, honest = fair(ts, rs);
      if (!same || !honest) ok = false; ticks += ts.ticks;
      lines.push(`${run}: ${ts.ticks} ticks, ends ${ts.end} · ${same ? `to the bit (${ts.bits})` : `DIFFERS — TypeScript ${ts.ticks} ticks ${ts.bits} ${ts.digest}, Rust ${rs.ticks} ticks ${rs.bits} ${rs.digest}`}${honest ? '' : ` · NOT A FAIR PAIR (of ${rs.ticks} ticks in the Rust run: ${rs.rust} on its maths, ${rs.lines} with its lines, ${rs.circles} with its circles, ${rs.fog} with its sight pass (${rs.sees} men asked), ${rs.poses} poses asked; in the other: ${ts.rust}, ${ts.lines} and ${ts.circles} of ${ts.ticks})`} · ${ts.secs} s / ${rs.secs} s`);
      console.error(`  ${lines[lines.length - 1]}`);
    }
    verdict(ok, `missions: ${runs.length} runs played with the rows ${up.join(' + ')} in Rust and with none are the same game after every update (${node ? 'node' : 'bun'})`, `${ticks} ticks compared\n    ${lines.join('\n    ')}`);
  }
  if (args.includes('--both')) {
    let ok = true; const lines: string[] = []; let asked = 0;
    const got = await Promise.all(runs.map(run => one(run, true, true)));
    runs.forEach((run, i) => { const r = got[i];
      if (typeof r === 'string') { ok = false; lines.push(`${run}: ${r}`); return; }
      const b = r.both, good = b.asked > 1e5 && b.differ === 0 && b.stale === 0 && (!up.includes('lines') || r.lines === r.ticks) && (!up.includes('circles') || r.circles === r.ticks) && (!up.includes('fog') || (r.fog === r.ticks && r.sees > 100)); if (!good) ok = false; asked += b.asked;
      lines.push(`${run}: ${r.ticks} ticks (${r.lines} with the module asked), ${b.asked} questions answered both ways (${r.sees} of them the fog's) · ${b.differ} answers differ · ${b.stale} stale rows${b.first ? ` · first: ${b.first}` : ''} · ${r.secs} s`); });
    verdict(ok, `both: every line the missions ask of the module is the TypeScript's answer, and the module's copy is the boxes at every question (${node ? 'node' : 'bun'})`, `${asked} questions over ${runs.length} runs\n    ${lines.join('\n    ')}`);
  }
}

console.log(fails ? `\n${fails} FAILED` : '\nall pass');
process.exit(fails ? 1 : 0);
