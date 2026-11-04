// THE SIMULATION IN RUST (rust/sim → src/sim/built.ts, loaded by src/sim/sim.ts): is the module the build of its sources, does it load, and is each system
// that runs in it the SAME GAME TO THE BIT as the TypeScript it stands in for?
//   bun run tools/qa/rust-check.ts                       the quick half (seconds): every check below but `missions`; exit 0 = all pass, 1 = one failed (named)
//   bun run tools/qa/rust-check.ts --missions            … and whole missions played both ways (minutes: five scripted squad runs, twice each, two at a time)
//   bun run tools/qa/rust-check.ts --missions --node     the missions under node (V8, the page's engine) instead of bun
//   bun run tools/qa/rust-check.ts --runs compound-four:1   other runs for `missions` (tools/qa/playthrough.ts's scenarios)
//   bun run tools/qa/rust-check.ts --missions --rows lines  the missions with THESE rows raised against none (default: every row at once)
//   bun run tools/qa/rust-check.ts --both                the missions once more with every question the box world's rows send to the module answered both ways (slow: minutes a run)
//   RUST_BREAK=1 bun run tools/qa/rust-check.ts [--missions] [--both]   the red path: the Rust side is handed the ENGINE's cosine where the game's own belongs, the
//                                                        box world's copy is not told what was written, one of the fog's answers in seven is turned,
//                                                        the die's word has a bit turned, the doors' step is handed a tick one unit in the last place
//                                                        too long and the falls a landing's glass as much too heavy. Then (1) every section that asks the module's answers must FAIL
//                                                        (RED_SECTIONS: `maths`, `engines`, `lines, circles`, `senses`, `poses`, `doors`, `falls`, `state, dice`, `server`,
//                                                        and `missions` / `both` where asked for) and (2) EVERY ROW of RUST must have answers OF ITS OWN
//                                                        that differed in the section that holds it (HELD). Exit 1 = red as it must · exit 3 = a section
//                                                        or a row the red path left green, named: what holds it proves nothing until that is mended
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
//   fog       the fog of war's sight pass (rust/sim/src/fog.rs: which eyes have a line to a man, to a body): the sixth kind of question of `senses` — a team's
//             eyes with wedges and near fields of their own, handed over as a pass hands them, on men and bodies, each asked of the module and of
//             fogofwar.ts. Four of the five standard runs ask no wedge: every eye of theirs sees all round (FOG.fovDeg 360). The run marked `:hard`
//             is played on Hard, where every eye is a CONE (FOG.cone), so the wedge and its near field are asked in play too; the wand's lens is asked by
//             the differential alone. `both` holds the eyes a mission does ask, man for man. `missions` holds the row ONLY IN PART (measured 2026-11-03, that row alone):
//             a man shown who should not be moves the bits on four of the five runs, a man missed one pass in seven moved none on the two runs tried
//   poses     the posing functions (rust/sim/src/skeleton.rs against src/anim/skeleton.ts): tools/qa/pose-ident.ts — the game's own rig and clips sampled (also against the
//             plain definition kept there), blended, run forward, turned and solved, each question asked of both: every single of every pose and every double of every run the same bits
//   senses    an eye on a man, the light on a point, the smoke along a line, a sound's way, a way across the floor and what a team's eyes see (rust/sim/src/
//             sight.rs, light.rs, smoke.rs, hear.rs, nav.rs, fog.rs): tools/qa/sense-ident.ts — random questions over whole levels (doors swung, statics re-shaped and said, lamps of
//             every kind, two dozen blobs), each asked of the module and of the TypeScript: every number of every answer the same bits. `missions` and
//             `both` hold them in play: a run is fair only if each raised sense answered there, and `both` answers every one of them twice.
//   state     a game's state in the module (rust/sim/src/state.rs, src/sim/state.ts) and its first tenant, the game's own die (dice.rs): tools/qa/simstate-check.ts
//             on both stands — the layout, the die against mulberry32 draw for draw, more blocks than slots, take and put, the row on against off and flipped
//             under a running game, a snapshot put back and put into another game, a view over the module's memory named. RUST_BREAK=1 runs its red path
//   doors     the doors' step (rust/sim/src/doors.rs against src/game/doors.ts Doors.update, the leaves' numbers columns of the game's block): tools/qa/door-ident.ts —
//             every leaf of four levels stepped both ways from the same state over made-up men, the rules' own writes between steps and the doors' switches
//             turned over: every column, every man's place, every noise, every leaf's box the same bits, and a run fair only if it met everything a leaf
//             does. The numbers the two sides share (the columns, the hand-over's tables, the rules' constants) are read out of both sources and compared.
//             `missions` and `both` hold it in play: with the row up a run is fair only if its door steps were the module's and leaves turned there.
//   falls     what falls and what it leaves (rust/sim/src/falls.rs against src/sim/fallshome.ts: the landings the simulation has scheduled and the litter
//             map of broken glass, a part of the game's block): tools/qa/falls-ident.ts — two hundred thousand rules (schedule past the list's cap, deposit,
//             the landings due, a footfall's step, the floor told, clear, the row dropped and raised) each taken both ways and against a twin store that
//             never goes near the module, in both lists of landings (glass, thrown pieces): every answer and every double the same. In play it is held where
//             glass breaks and where things break: the `module` and `snapshot` lines of tools/qa/glassfall-check.ts and tools/qa/debrisfall-check.ts (a
//             scene with the row on against off, and a snapshot put back while things are in the air). `missions` raises the row with the rest, but the
//             five missions break nothing that falls: there the row is only shown to do no harm.
//   stand     the frozen stand leaves every row off; the shipped stand has the rows of src/sim/sim.ts RUST_ON on, as the page (TPS_RUST<ROW>=1 raises a row on
//             either, =0 puts a default one off); FROZEN_HOLDS has their rows; and the address's words (profile.ts pageRows: ?rust=0, ?rust<row>, ?rust<row>=0)
//             write what they say, also over rows a recipe wrote
//   server    the game server's own stand (tools/mp/stand.ts): a setup that asks for the row makes a recipe that carries it, the game stood from that recipe in
//             another process ticks with the module's maths in Math's place, and after 600 ticks of a walking, shooting seat its state word is the word of
//             the same setup without the row. `server as the page`: a setup that says nothing of the rows makes a recipe with the page's own (RUST_ON),
//             and the game stood from it is that game too
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
/** the runs of `missions` and `both`: a play-through's scenario and seed, and `:hard` = that run on Hard (game/difficulty.ts: the defenders' and, what matters
 *  here, the fog's CONE — the only run whose eyes are wedges, which is how the page is played on Hard) */
const RUNS = ['compound-four:1', 'warehouse-four:1', 'pass-four:1', 'hostage-rescue:1', 'block-squads:1', 'hostage-rescue:1:hard'];
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
  const P = C.StaticCollision.prototype, see = P.eyesSee; P.wrote = function () {};
  Object.defineProperty(M.LEAVES, 'n', { get: () => 0, set: () => {} });   // (a leaf written moves no count: the copy of the leaves made at a tick's first question stands for all of it)
  // (…and of the fog's pass in a mission: one of the module's answers in seven is turned — nobody where somebody saw, the first eye tried where nobody did.
  //  What this had, every eye handed over as seeing all round, was no break at all: every eye of the comparison runs sees all round already, so with the fog's
  //  row alone the five runs were the same game to the bit and `both` found none of 18,830 answers differing. A stale copy, the red path of the other rows,
  //  seldom reaches the fog in a mission either (its pass is over before a rule writes); the differential's unsaid re-shape is where the copy is held for it)
  let n = 0; P.eyesSee = function (this: any, order: number, eyes: number, ...a: unknown[]) { const r = see.call(this, order, eyes, ...a); return r >= 0 && eyes > 0 && ++n % 7 === 0 ? (r === 0 ? (1 << 28) | (order & 15) : 0) : r; };
  M.BoxMirror.prototype.followLeaf = function (_i: number, b: { yaw: number }) { return b.yaw; }; M.BoxMirror.prototype.followRow = function (b: { yaw: number }) { return b.yaw; };   // (…and the other way round: a leaf the module's door step turned is not followed by its box)
}
const SENSES = ['sight', 'light', 'smoke', 'sound', 'paths'] as const;
type Sense = keyof typeof import('../../src/scene/boxMirror.ts').SENSES;
type Sensed = Record<Sense, number>;
{ const every: Exclude<Sense, typeof SENSES[number]> extends never ? true : never = true; void every; }   // (a sense added to boxMirror.ts SENSES and not to the list above does not typecheck)
/** the ticks that opened with the box world's line tests, and its circles, going to the module (a row can be on and its copy not standing: a level too big for
 *  the tables), and collision.ts's own count of the senses' questions the module answered */
function countLines(C: any): { n: number; circles: number; fog: number; sees: number; sensed: Sensed } {
  const k = { n: 0, circles: 0, fog: 0, sees: 0, sensed: C.SENSED as Sensed }, P = C.StaticCollision.prototype, open = P.tickOpen, see = P.eyesSee;
  P.tickOpen = function (this: any) { open.call(this); if (this.mirror.lines) k.n++; if (this.mirror.circles) k.circles++; if (this.mirror.sight) k.fog++; };
  P.eyesSee = function (this: any, ...a: unknown[]) { const r = see.apply(this, a); if (r >= 0) k.sees++; return r; };   // (the men and bodies the fog's pass asked of the module)
  return k;
}
// ---- `missions`, one run in THIS process (a child): the update tapped, the raw bits chained after every tick
if (args[0] === '--run') {
  process.env.TPS_PROFILE ??= 'shipped'; process.env.TPS_CPUSENSE ??= '1'; process.env.PT_DIGEST ??= '1';
  const [scn, seed, how] = args[1].split(':'); if (how !== undefined && how !== 'hard') { console.error(`rust-check: a run is scenario:seed or scenario:seed:hard (not "${args[1]}")`); process.exit(2); }
  const { Game } = await import(`${ROOT}/src/game/game.ts`);
  const PT: any = await import(`${ROOT}/tools/qa/playthrough.ts`), FW: any = await import(`${ROOT}/src/game/fogofwar.ts`);
  const { rawBits, RAW_BITS_SEED } = await import(`${ROOT}/src/game/digest.ts`);
  const S: any = await import(`${ROOT}/src/sim/sim.ts`), X: any = await import(`${ROOT}/src/math/exact.ts`), C: any = await import(`${ROOT}/src/scene/collision.ts`);
  const lines = countLines(C); if (BREAK) unsaid(C, await import(`${ROOT}/src/scene/boxMirror.ts`)); C.BOTH.on = process.env.RUST_BOTH === '1';
  const SK: any = await import(`${ROOT}/src/anim/skeleton.ts`), ST: any = await import(`${ROOT}/src/sim/state.ts`);
  const DH: any = await import(`${ROOT}/src/sim/doorshome.ts`), FH: any = await import(`${ROOT}/src/sim/fallshome.ts`); FH.FALLS.both = process.env.RUST_BOTH === '1';
  // (the red path of the die: once, fifty binds in, the lowest bit of the word in the game's block is turned — a wrong die that still rolls)
  if (BREAK) { const P = ST.SimState.prototype, bind = P.bind; let n = 0; P.bind = function (this: any) { const slot = bind.call(this); if (++n === 50) this.w[(ST.simPart(ST.P_DICE) + 8) >>> 2] ^= 1; return slot; }; }
  let posed = 0, sets = 0;   // (the ticks that ended with poses of its men in the game's block, and how many men's)
  // (the red path of the layers: from the fiftieth tick's opening every knob is handed to the module a hundredth large)
  if (BREAK) { const RK: any = await import(`${ROOT}/src/game/rustknobs.ts`); RK.RUST_KNOBS.hand = (_i: number, v: number) => RK.RUST_KNOBS.written >= 50 ? v * 1.01 : v; }
  let homed = 0, draws = 0;   // (the ticks that ended with the game's own die at home in the module's block, and the draws taken off it by the end)
  let h: number = RAW_BITS_SEED, ticks = 0, broke = false, inRust = 0, cone = 0;   // (inRust: the ticks whose update found the module's sine in Math's place — the play-through puts every knob table back when it ends, so the row is read while it runs)
  const update = Game.prototype.update;
  Game.prototype.update = function (this: unknown, ...a: unknown[]) {
    // (the red path: from the first tick the Rust set is handed the engine's cosine — a wrong arithmetic that still plays)
    if (BREAK && !broke && S.RUST.maths) { broke = true; wrongMaths(S, X); }
    if (broke || Math.sin === S.sim().maths_sin) inRust++; if (FW.FOG.cone) cone++;   // (cone: the ticks whose eyes were wedges — Hard's fog)
    const r = (update as (...x: unknown[]) => unknown).apply(this, a); h = rawBits(this as never, h); ticks++;
    { const d = (this as any).dice; if (d) { if (d.home) homed++; draws = d.n; } }
    { const ps = (this as any).poseSets; if (ps.length) posed++; sets = ps.length;
      // (the red path of the poses at home: from the fiftieth such tick a pose's floats are handed to the TypeScript ten singles off from where the module has them)
      if (BREAK && posed === 50) { const f = ST.SimState.prototype.f32; ST.SimState.prototype.f32 = function (this: unknown, at: number, n: number) { return f.call(this, at + 40, n); }; } }
    return r;
  } as typeof update;
  const t0 = performance.now();
  const r = await PT.play(scn, Number(seed), 300, false, how === 'hard' ? { DIFFICULTY: { level: 'hard' } } : {});
  console.log('RUST-RUN ' + JSON.stringify({ run: args[1], ticks, cone, end: String(r.end), digest: String(r.digest), bits: h.toString(36), rust: inRust, lines: lines.n, circles: lines.circles, fog: lines.fog, sees: lines.sees, sensed: lines.sensed, poses: SK.POSE_BENCH.asked, posesIn: SK.POSE_BENCH.inPlace, posed, sets, layers: SK.LAYERS.ran, dice: homed, draws, doors: DH.DOORS_STEPPED, falls: FH.FALLS, both: C.BOTH, exact: X.EXACT.on, engine, secs: Math.round((performance.now() - t0) / 100) / 10 }));
  process.exit(0);
}
// ---- `server`, in a child: the game server's stand from a recipe (argv), a seat's hands by the beat, the word after `ticks`
if (args[0] === '--server') {
  const { standRecipe }: any = await import(`${ROOT}/tools/mp/stand.ts`), { sampleCmd }: any = await import(`${ROOT}/src/game/cmd.ts`);
  const S: any = await import(`${ROOT}/src/sim/sim.ts`), X: any = await import(`${ROOT}/src/math/exact.ts`), C: any = await import(`${ROOT}/src/scene/collision.ts`);
  const lines = countLines(C);
  const st = await standRecipe(JSON.parse(args[1])), W = st.W, I = W.input, ticks = Number(args[2]);
  let inRust = 0; const broke = BREAK && S.RUST.maths; if (broke) wrongMaths(S, X);
  // (…and the red path of a stand with no maths row, the page's own rows: the module's answer to every line through the statics is "clear" — a wrong walk
  //  that still plays, as rust-page.cjs breaks the page. A stale leaf and a turned answer of the fog's, the missions' break, move no word in 600 ticks: tried)
  if (BREAK && !S.RUST.maths) { const P = (await import(`${ROOT}/src/scene/boxMirror.ts`) as any).BoxMirror.prototype, open = P.open; P.open = function (this: any, ...a: unknown[]) { open.apply(this, a); if (!this.broke) { this.broke = true; this.x = Object.assign({}, this.x, { lines_blocked: () => 0 }); } }; }
  for (let n = 0; n < ticks; n++) {
    I.keys.clear(); I.pressed.clear(); I.clicked = 0; I.buttons = 0; I.wheel = 0;
    I.keys.add(['KeyW', 'KeyD', 'KeyS', 'KeyA'][Math.floor(n / 90) % 4]);
    I.mouseX = W.canvas.clientWidth / 2 + X.cos(n * 0.05) * 260; I.mouseY = W.canvas.clientHeight / 2 + X.sin(n * 0.05) * 180;   // (the game's own maths by name: the same hands whichever stand in Math's place)
    if (n > 120 && n % 40 < 4) { I.buttons = 1; if (n % 40 === 0) I.clicked = 1; }
    if (broke || (!S.simWhyNot() && Math.sin === S.sim().maths_sin)) inRust++;
    st.step([sampleCmd(W.game, I, W.cam, W.canvas)]);
  }
  console.log('RUST-SERVER ' + JSON.stringify({ ticks: st.ticks, word: st.word(), rust: inRust, lines: lines.n, circles: lines.circles, fog: lines.fog, sees: lines.sees, sensed: lines.sensed, row: S.RUST.maths, exact: X.EXACT.on }));
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

/** WHAT HOLDS EACH ROW OF `RUST` IN THIS HALF: the section (a verdict's name up to its colon) that asks that system's own questions of the module and of the
 *  TypeScript, one by one, and counts the answers that differ BY ROW (`red`, filled from each differential's own last line). `rows` is red on a row with no
 *  line here or a line naming a section that did not run. Under RUST_BREAK=1 each row's own answers must have differed; exit 3 names a row that stayed
 *  green. A break word that does not reach a row leaves that row's check proving nothing, and a red neighbour hides it: the circles', the fog's and the
 *  die's did not reach them until 2026-11-03. */
const HELD: Record<string, string> = { maths: 'maths', lines: 'lines, circles', circles: 'lines, circles', fog: 'senses', poses: 'poses', sight: 'senses', light: 'senses', smoke: 'senses', sound: 'senses', paths: 'senses', dice: 'state, dice', doors: 'doors', falls: 'falls', walk: 'layers' };
/** …and the sections that must be red on the red path whatever rows they hold (each asks the module's answers; one left green there holds nothing) */
const RED_SECTIONS = ['maths', 'engines', 'lines, circles', 'senses', 'poses', 'layers', 'doors', 'falls', 'state, dice', 'server', 'server as the page', 'missions', 'both'];
const red: Record<string, boolean> = {}, said: Record<string, boolean> = {};   // (said: every verdict of a section, all of them PASS?)
let fails = 0;
const verdict = (ok: boolean, label: string, detail: string) => { if (!ok) fails++; const k = label.split(':')[0]; said[k] = (said[k] ?? true) && ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
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
  const a = fold(own, N, rust); wordOwn = a.word; wordRust = fold(rust, N).word; red.maths = a.differ > 0;
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
  { const d = marked(r.out, 'SEGMENT-IDENT '); red.lines = !!d && d.lines > 0; red.circles = !!d && d.circles > 0; }
  verdict(ok, 'lines, circles: every line through the box world and every circle against it gets the TypeScript\'s answer from the Rust module', rows.length ? rows.map(l => l.replace(/\s+/g, ' ').trim()).join('\n    ') : `the differential left nothing (exit ${r.status}): ${(r.err || r.out).trim().split('\n').slice(-2).join(' / ').slice(0, 300)}`);
}

// ---- senses
{
  const r = child(['bun', 'run', 'tools/qa/sense-ident.ts', '--n', flag('--questions') ?? '12000', '--levels', flag('--levels') ?? 'office.json,warehouse.json,compound.json,glasshouse.json']);
  const rows = r.out.split('\n').filter(l => /questions a sense|FAIL/.test(l));
  // the numbers the host writes and reads by (boxMirror.ts FACE) against the Rust sources' own, name for name
  const fs = await import('node:fs'), face = /export const FACE = \{([^}]*)\}/.exec(fs.readFileSync(resolve(ROOT, 'src/scene/boxMirror.ts'), 'utf8'))?.[1] ?? '';
  const rust = ['light', 'smoke', 'hear', 'sight'].map(f => fs.readFileSync(resolve(ROOT, `rust/sim/src/${f}.rs`), 'utf8')).join('\n');
  const pairs = [...face.matchAll(/(\w+):\s*(\d+)/g)].map(m => ({ name: m[1], here: Number(m[2]), there: Number(new RegExp(`pub const ${m[1]}: \\w+ = (\\d+);`).exec(rust)?.[1] ?? NaN) }));
  const off = pairs.filter(q => q.here !== q.there), ok = r.status === 0 && rows.length > 0 && rows.every(l => / PASS /.test(l)) && pairs.length >= 15 && off.length === 0;
  { const d = marked(r.out, 'SENSE-IDENT '); for (const k of [...SENSES, 'fog']) red[k] = !!d && d[k] > 0; }
  verdict(ok, 'senses: an eye on a man, the light on a point, the smoke along a line, a sound\'s way, a way across the floor and what a team\'s eyes see get the TypeScript\'s answer from the Rust module', (rows.length ? rows.map(l => l.replace(/\s+/g, ' ').trim().slice(0, 420)).join('\n    ') : `the differential left nothing (exit ${r.status}): ${(r.err || r.out).trim().split('\n').slice(-2).join(' / ').slice(0, 300)}`)
    + `\n    ${pairs.length} numbers shared with the Rust sources (boxMirror.ts FACE)${off.length ? ` — NOT THE SAME: ${off.map(q => `${q.name} ${q.here} here, ${q.there} there`).join(', ')}` : ', each the same'}`);
}

// ---- poses
{
  const r = child(['bun', 'run', 'tools/qa/pose-ident.ts', '--n', flag('--poses') ?? '4000']);
  const row = r.out.split('\n').find(l => /^pose-ident: \d+ rounds/.test(l)), ok = r.status === 0 && !!row && /=== pose-ident: PASS/.test(r.out);
  red.poses = (marked(r.out, 'POSE-IDENT ')?.differ ?? 0) > 0 && r.status !== 3;   // (its exit 3 = a KIND of posing question the red path left green: not red, however many of the others differed)
  verdict(ok, 'poses: every pose the Rust module samples, blends, runs forward, turns and solves is the TypeScript\'s, single for single', row ? row.replace(/^pose-ident: /, '') + (r.status === 3 ? `\n    ${r.out.trim().split('\n').pop()}` : '') : `the differential left nothing (exit ${r.status}): ${(r.err || r.out).trim().split('\n').slice(-2).join(' / ').slice(0, 300)}`);
}

// ---- layers
{
  const r = child(['bun', 'run', 'tools/qa/layer-ident.ts']);
  const by: Record<string, { asked: number; differ: number }> = marked(r.out, 'LAYER-IDENT ')?.by ?? {}, names = Object.keys(by), row = r.out.split('\n').find(l => /^    \S+:\d+ ends /.test(l));
  red.walk = names.length > 0 && names.every(n => by[n].differ > 0);   // (every moved layer's own answers, not one red neighbour's)
  verdict(r.status === 0 && /=== layer-ident: PASS/.test(r.out), 'layers: every call of an animator\'s layer in a mission leaves the same pose run by the Rust module and by the TypeScript, and the knobs and named nodes the module reads are the game\'s',
    row ? row.trim() + (r.status === 0 ? '' : ` · ${r.out.split('\n').filter(l => /^- FAIL/.test(l)).map(l => l.split(':')[0].replace('- FAIL — ', '')).join(', ')} FAILED`) : `the differential left nothing (exit ${r.status}): ${(r.err || r.out).trim().split('\n').slice(-2).join(' / ').slice(0, 300)}`);
}
// ---- doors
{
  const r = child(['bun', 'run', 'tools/qa/door-ident.ts', '--n', flag('--doorsteps') ?? '6000'], { DOORS_BREAK: BREAK ? '1' : undefined });
  const rows = r.out.split('\n').filter(l => / (PASS|FAIL) — \d+ doors/.test(l)), sum = r.out.split('\n').find(l => /^door-ident: /.test(l)) ?? '';
  red.doors = Number(/, (\d+) differ/.exec(sum)?.[1] ?? 0) > 0;   // (the doors' own answers: steps whose columns, men, noises or boxes differed)
  // the numbers the two sides share, read out of both sources: the columns, the hand-over's tables, the rules' constants
  const fs = await import('node:fs'), rs = fs.readFileSync(resolve(ROOT, 'rust/sim/src/doors.rs'), 'utf8'), home = fs.readFileSync(resolve(ROOT, 'src/sim/doorshome.ts'), 'utf8'), ts = fs.readFileSync(resolve(ROOT, 'src/game/doors.ts'), 'utf8'), pl = fs.readFileSync(resolve(ROOT, 'src/game/player.ts'), 'utf8');
  const there = (name: string): number => { const m = new RegExp(`(?:pub )?const ${name}: \\w+ = ([^;]+);`).exec(rs); return m ? Number(eval(m[1].replace(/_/g, ''))) : NaN; };
  const snake = (k: string) => k.replace(/[A-Z]/g, c => '_' + c).toUpperCase(), off: string[] = []; let shared = 0;
  const table = (name: string, prefix: string) => { const body = new RegExp(`export const ${name} = \\{([^}]*)\\}`).exec(home)?.[1] ?? ''; const ps = [...body.matchAll(/(\w+):\s*(-?\d+)/g)]; if (!ps.length) off.push(`${name} not found`);
    for (const q of ps) { shared++; const t = there(prefix + snake(q[1])); if (t !== Number(q[2])) off.push(`${name}.${q[1]} ${q[2]} here, ${t} there`); } };
  table('DC', ''); table('DD', 'D_'); table('DO', 'O_'); table('DK', 'K_'); table('KF', 'KF_'); table('DE', 'E_'); table('DP', 'P_'); table('DF', 'F_');
  for (const [k, t] of [['DOOR_CAP', 'CAP'], ['DOOR_COLS', 'NCOL'], ['DOOR_EV', 'NEV'], ['DOOR_OUTS', 'NOUT'], ['DOOR_CONS', 'NCON'], ['DOOR_PARS', 'NPAR']]) { shared++; const h = Number(new RegExp(`export const ${k} = (\\d+)`).exec(home)?.[1]); if (h !== there(t)) off.push(`${k} ${h} here, ${t} ${there(t)} there`); }
  { const kinds = /DRIVE_KINDS = \[([^\]]*)\]/.exec(home)?.[1].replace(/['\s]/g, '').split(',') ?? []; ['BOOT', 'SWING', 'BODY', 'HAND'].forEach((k, i) => { shared++; if (kinds[i] !== k.toLowerCase() || there(k) !== i) off.push(`drive kind ${k}`); });
    const sounds = /DOOR_SOUNDS = \[([^\]]*)\]/.exec(home)?.[1].replace(/['\s]/g, '').split(',') ?? []; ['CREAK', 'BANG', 'LATCH', 'BASH', 'RATTLE'].forEach(k => { shared++; if (sounds[there('S_' + k)] !== k.toLowerCase()) off.push(`sound ${k}`); });
    const tiers = /STRIKE_TIERS = \[([^\]]*)\]/.exec(home)?.[1].replace(/['\s]/g, '').split(',') ?? []; ['OPEN', 'SLAM', 'BOOT'].forEach(k => { shared++; if (tiers[there('T_' + k)] !== k.toLowerCase()) off.push(`tier ${k}`); });
    const type = /export type DoorSound = ([^;]*);/.exec(ts)?.[1].replace(/['\s]/g, '').split('|') ?? []; shared++; if (type.join() !== sounds.join()) off.push('DOOR_SOUNDS is not doors.ts DoorSound in its order'); }
  const here = (src: string, name: string): number => Number(new RegExp(`\\b${name} = (-?[\\d.]+)`).exec(src)?.[1] ?? NaN);
  for (const k of ['SOLID_SLIDE', 'SOLID_TRIES', 'SOLID_FADE', 'SOLID_SKIN', 'SOLID_RATE', 'SOLID_REACH', 'SOLID_INTO', 'SOLID_BACK', 'PULL_VEL', 'PULL_VEL_QUIET', 'PULL_SECS', 'PRESS_SECS', 'DRIVEN_SECS', 'DRIVEN_VEL', 'STOP_BODY', 'ROUND_SLIDE', 'ROUND_SECS', 'ROUND_STILL', 'SHOVE_OPENS_BELOW', 'SHOVE_RATE', 'TURN_RATE', 'KICK_SUBSTEP', 'MAN_SIDE', 'MAN_FOLDED', 'MAN_BRACED', 'MAN_CALM', 'MAN_HELD']) { shared++; if (here(ts, k) !== there(k)) off.push(`${k} ${here(ts, k)} in doors.ts, ${there(k)} in doors.rs`); }
  { // the drive's columns, zeroed where a drive is cleared: doors.rs DRIVE_COLS is DC's names that begin `drive`, all of them (doors.ts takes its own list from DC)
    const want = [...(/export const DC = \{([^}]*)\}/.exec(home)?.[1] ?? '').matchAll(/(drive[A-Z]\w*):/g)].map(q => q[1]).sort().join();
    const rsL = (/const DRIVE_COLS: \[usize; \d+\] = \[([^\]]*)\]/.exec(rs)?.[1] ?? '').replace(/\s/g, '').split(',').map(k => k.toLowerCase().replace(/_(\w)/g, (_, c) => c.toUpperCase())).sort().join();
    shared++; if (!want || rsL !== want || !/Object\.keys\(DC\)[^;]*\^drive\[A-Z\]/.test(ts)) off.push(`the drive's columns: DC has ${want}; doors.rs clears ${rsL}`); }
  for (const k of ['BODY_R', 'DOOR_R']) { shared++; if (here(pl, k) !== there(k)) off.push(`${k} ${here(pl, k)} in player.ts, ${there(k)} in doors.rs`); }
  { shared++; const fit = /DOOR_FIT = \{ lat: \[([\d., ]+)\] as const, out: \[([\d., ]+)\]/.exec(pl), lat = /DOOR_FIT_LAT: \[f64; 2\] = \[([\d., ]+)\]/.exec(rs)?.[1], out = /DOOR_FIT_OUT: \[f64; 2\] = \[([\d., ]+)\]/.exec(rs)?.[1]; if (!fit || fit[1].split(',').map(Number).join() !== lat?.split(',').map(Number).join() || fit[2].split(',').map(Number).join() !== out?.split(',').map(Number).join()) off.push('DOOR_FIT'); }
  const ok = r.status === 0 && rows.length >= 4 && rows.every(l => / PASS /.test(l)) && /=== door-ident: PASS/.test(r.out) && off.length === 0 && shared > 150;
  verdict(ok, 'doors: every leaf the Rust module steps ends where the TypeScript\'s step leaves it — its columns, the men, the noises, its box — and the two sides name their numbers alike',
    (rows.length ? rows.map(l => l.replace(/\s+/g, ' ').trim()).join('\n    ') + '\n    ' + sum.slice(0, 600) : `the differential left nothing (exit ${r.status}): ${(r.err || r.out).trim().split('\n').slice(-2).join(' / ').slice(0, 300)}`)
    + `\n    ${shared} numbers shared with the Rust source (doorshome.ts, doors.ts, player.ts)${off.length ? ` — NOT THE SAME: ${off.slice(0, 8).join(', ')}` : ', each the same'}`);
}

// ---- falls
{
  const r = child(['bun', 'run', 'tools/qa/falls-ident.ts'], { FALLS_BREAK: BREAK ? '1' : undefined }), row = r.out.split('\n').find(l => /^falls-ident: /.test(l)) ?? '', met = r.out.split('\n').find(l => /^\s+met: /.test(l)) ?? '';
  red.falls = /=== falls-ident: FAIL/.test(r.out) && (Number(/answered both ways, (\d+) differ/.exec(row)?.[1] ?? 0) > 0 || /the twin PARTS/.test(row));   // (the falls' own answers: rules that differed both ways, or the twin store parting)
  const fs = await import('node:fs'), rs = fs.readFileSync(resolve(ROOT, 'rust/sim/src/falls.rs'), 'utf8'), home = fs.readFileSync(resolve(ROOT, 'src/sim/fallshome.ts'), 'utf8'), world = fs.readFileSync(resolve(ROOT, 'src/scene/world.ts'), 'utf8');
  const there = (name: string): number => Number(new RegExp(`pub const ${name}: \\w+ = ([-\\d.]+);`).exec(rs)?.[1] ?? NaN), off: string[] = []; let shared = 0;
  for (const q of [...(/export const FP = \{([^}]*)\}/.exec(home)?.[1] ?? '').matchAll(/(\w+):\s*(\d+)/g)]) { shared++; if (there('P_' + q[1].toUpperCase()) !== Number(q[2])) off.push(`FP.${q[1]}`); }
  for (const [k, t] of [['FALL_CELL', 'CELL'], ['FALL_CELLS_X', 'CELLS_X'], ['FALL_CELLS_Z', 'CELLS_Z'], ['FALL_CELL_MAX', 'CELL_MAX'], ['FALL_MAXP', 'MAXP'], ['FALL_GLASS', 'L_GLASS'], ['FALL_PIECES', 'L_PIECES'], ['FALL_LISTS', 'NLIST'], ['NP', 'NP'], ['DUE', 'DUE']]) { shared++; const h = Number(new RegExp(`\\b${k} = ([\\d.]+)`).exec(home)?.[1]); if (h !== there(t)) off.push(`${k} ${h} here, ${t} ${there(t)} there`); }
  { shared++; const max = /max: \[(\d+), \d+, (\d+)\]/.exec(world), min = /min: \[(-?[\d.]+), -?[\d.]+, (-?[\d.]+)\]/.exec(world); if (!max || !min || Number(min[1]) !== 0 || Number(min[2]) !== 0 || Number(max[1]) !== there('CELLS_X') * there('CELL') || Number(max[2]) !== there('CELLS_Z') * there('CELL')) off.push('the cells are not the world\'s floor (scene/world.ts WORLD)'); }
  verdict(r.status === 0 && /=== falls-ident: PASS/.test(r.out) && off.length === 0 && shared >= 22, 'falls: every landing scheduled, every bit of glass laid and every footfall on it is the TypeScript\'s in the Rust module, and the two sides name their numbers alike',
    (row ? row.replace(/^falls-ident: /, '') + '\n' + met : `the differential left nothing (exit ${r.status}): ${(r.err || r.out).trim().split('\n').slice(-2).join(' / ').slice(0, 300)}`) + `\n    ${shared} numbers shared with the Rust source${off.length ? ` — NOT THE SAME: ${off.join(', ')}` : ', each the same'}`);
}

// ---- state
{
  const run = (profile: string | undefined) => child(['bun', 'run', 'tools/qa/simstate-check.ts'], { TPS_PROFILE: profile, SIMSTATE_BREAK: BREAK ? '1' : undefined });
  const fr = run(undefined), sh = run('shipped'), rows = (o: string) => o.split('\n').filter(l => /^- (PASS|FAIL) — /.test(l)), bad = [...rows(fr.out), ...rows(sh.out)].filter(l => /^- FAIL/.test(l));
  const ok = fr.status === 0 && sh.status === 0 && rows(fr.out).length >= 8 && rows(sh.out).length >= 8;
  red.dice = [fr, sh].every(c => rows(c.out).some(l => /^- FAIL — dice:/.test(l)));   // (the die's own line, on both stands: the other lines fail for the block)
  verdict(ok, 'state, dice: a game\'s block of the module\'s memory is the same state in any slot, the snapshot carries it, and the die at home in it is the game\'s die (both stands)',
    ok ? `${rows(fr.out).length} lines on the frozen stand, ${rows(sh.out).length} on the shipped one, every one PASS (tools/qa/simstate-check.ts)` : bad.length ? bad.map(l => l.slice(2, 110)).join('\n    ') : `the check left nothing (exits ${fr.status} ${sh.status}): ${(fr.err || sh.err).trim().split('\n').slice(-2).join(' / ').slice(0, 300)}`);
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
  const DEF: string[] = all.filter(r => S.rustOn(r)), plus = (r: string) => all.filter(q => q === r || DEF.includes(q)).join();
  const fr = rows({}), sh = rows({ TPS_PROFILE: 'shipped' }); let ok = !!fr && !!sh && fr.profile === 'frozen' && sh.profile === 'shipped' && fr.on.length === 0 && sh.on.join() === DEF.join() && DEF.length === S.RUST_ON.length && DEF.length > 0; const notes: string[] = [];
  for (const r of all) { const a = rows({ [word(r)]: '1' }), b = rows({ TPS_PROFILE: 'shipped', [word(r)]: '1' }), good = !!a && !!b && a.on.join() === r && b.on.join() === plus(r); if (!good) ok = false; notes.push(`${word(r)}=1: frozen [${a?.on ?? '?'}] shipped [${b?.on ?? '?'}]`); }
  for (const r of DEF) { const b = rows({ TPS_PROFILE: 'shipped', [word(r)]: '0' }), good = !!b && b.on.join() === DEF.filter(q => q !== r).join(); if (!good) ok = false; notes.push(`${word(r)}=0: shipped [${b?.on ?? '?'}]`); }
  const P: any = await import(`${ROOT}/src/host/profile.ts`);
  { const miss = DEF.filter(r => P.FROZEN_HOLDS[`RUST.${r}`] !== word(r) + '=1'), more = Object.keys(P.FROZEN_HOLDS).filter(k => k.startsWith('RUST.') && !DEF.includes(k.slice(5))); if (miss.length || more.length) { ok = false; notes.push(`FROZEN_HOLDS: no row for [${miss}], a row too many for [${more}]`); } }
  // (the ADDRESS's words, profile.ts pageRows, in this process and put back: what a page opens with, and the address read again over rows a recipe wrote)
  { const was: [string, boolean][] = all.map(r => [r, !!S.RUST[r]]), now = () => all.filter(r => S.RUST[r]).join(), set = (v: boolean) => { for (const r of all) S.rustRow(r, v); };
    const other = all.find(r => !DEF.includes(r))!, d0 = DEF[0], cases: [string, string, boolean | null, string][] = [   // (address · read over rows all on / all off / not over · the rows wanted)
      ['', 'opens', null, DEF.join()], ['rust=0', 'opens', null, ''], [`rust=0&rust${other}`, 'opens', null, other], [`rust${d0}=0`, 'opens', null, DEF.filter(r => r !== d0).join()], [`rust${other}`, 'opens', null, plus(other)],
      ['', 'over a recipe with every row', true, all.join()], ['', 'over a recipe with none', false, ''], ['rust=0', 'over a recipe with every row', true, ''], [`rust${d0}=0`, 'over a recipe with every row', true, all.filter(r => r !== d0).join()],
      [`rust${other}`, 'over a recipe with none', false, other], [`rust=0&rust${other}`, 'over a recipe with every row', true, other]];
    const bad: string[] = []; for (const [addr, how, over, want] of cases) { if (over !== null) set(over); P.pageRows(new URLSearchParams(addr), over !== null); if (now() !== want) bad.push(`?${addr} ${how}: [${now()}], not [${want}]`); }
    for (const [r, v] of was) S.rustRow(r, v);
    if (bad.length) ok = false; notes.push(bad.length ? `THE ADDRESS: ${bad.join(' · ')}` : `the address's words: ${cases.length} cases as written (a page as it opens, and over a recipe's rows)`); }
  // (…and the words must still be ON the address when such a page reads it: a replay's or a networked game's page is first sent on to its recipe's address
  //  (replay/engineplay.ts recipeSearch), which keeps of the visit only the visit's words — the rows' words are among them, ui/replaykey.ts VISIT_FLAGS —
  //  and drops a recipe's own. Without that a typed ?rust=0 was gone before pageRows ran: found by the multiplayer task on the first cut of this)
  { const EP: any = await import(`${ROOT}/src/replay/engineplay.ts`), RK: any = await import(`${ROOT}/src/ui/replaykey.ts`), other = all.find(r => !DEF.includes(r))!, d0 = DEF[0];
    const sent = (visit: string) => new URLSearchParams(String(EP.recipeSearch({ search: `?roomclear&squad4&rust=0&rust${other}`, seed: 7 }, new URLSearchParams(visit), { playtape: '1' })).replace(/^\?/, ''));
    const a = sent(`playtape=1&mute&rust=0&rust${d0}=0&rust${other}=1`), b = sent('playtape=1&mute'), listed = S.rustWords().every((w: string) => RK.VISIT_FLAGS.includes(w)) && S.rustWords().length === all.length + 1;
    const good = listed && a.get('rust') === '0' && a.get('rust' + d0) === '0' && a.get('rust' + other) === '1' && a.has('roomclear') && a.has('mute') && !b.has('rust') && !b.has('rust' + other) && b.has('roomclear');
    if (!good) ok = false; notes.push(`a page sent on to a recipe's address ${good ? 'keeps the visit\'s words about the rows and drops the recipe\'s own' : `LOSES OR TAKES A WORD: the visit's [${a}] · a visit with none [${b}] · every word a visit word ${listed}`}`); }
  verdict(ok, 'stand: the frozen stand leaves every row off; the shipped stand has the rows of RUST_ON on, as the page; a row\'s own word raises it — and it alone — on either, and puts a default one off', `frozen [${fr?.on ?? '?'}] · shipped [${sh?.on ?? '?'}] · ${notes.join(' · ')} · rows: ${all.join(' ')}`);
}

// ---- server
{
  const TICKS = 600, setup = { search: '?roomclear&squad4', seed: 7 }, CLEAN: Record<string, undefined> = {}; for (const r of S.rustRows()) CLEAN['TPS_RUST' + r.toUpperCase()] = undefined;   // (the row comes from the recipe here, never from a word of the caller's shell)
  const recipeOf = (tables: object | null) => marked(child(['bun', 'run', 'tools/mp/setup.ts', JSON.stringify(tables ? { ...setup, tables } : setup)], CLEAN).out, 'MPSETUP ');
  const stood = (recipe: unknown) => recipe ? marked(child(['bun', 'run', 'tools/qa/rust-check.ts', '--server', JSON.stringify(recipe), String(TICKS)], CLEAN).out, 'RUST-SERVER ') : null;
  const every = (v: boolean) => ({ RUST: Object.fromEntries(S.rustRows().map((r: string) => [r, v])) });
  const rOn = recipeOf(every(true)), rOff = recipeOf(every(false)), rDef = recipeOf(null), on = stood(rOn), off = stood(rOff), def = stood(rDef);
  const table = (r: any) => r?.tables?.RUST ?? r?.knobs?.tables?.RUST, carried = (r: any) => S.rustRows().every((k: string) => table(r)?.[k] === true), bare = (r: any) => S.rustRows().every((k: string) => table(r)?.[k] === false);
  const ok = !!on && !!off && carried(rOn) && bare(rOff) && on.ticks === TICKS && off.ticks === TICKS && on.rust === TICKS && off.rust === 0 && on.lines === TICKS && off.lines === 0 && on.circles === TICKS && off.circles === 0 && on.fog === TICKS && off.fog === 0
    && on.exact && off.exact && on.word === off.word && SENSES.every(k => off.sensed[k] === 0) && on.sensed.sight > 0 && on.sensed.light > 0 && on.sensed.sound > 0 && on.sensed.paths > 0;
  verdict(ok, 'server: the game server\'s stand takes the rows from its recipe, ticks on the module\'s maths, lines, circles and fog, and is the game it is without', !on || !off ? `a stand left nothing (recipes: ${!!rOn} ${!!rOff} · stands: ${!!on} ${!!off})` :
    `the recipe carries the rows ${JSON.stringify(table(rOn))} · ${on.rust} of ${on.ticks} ticks on the module's maths, ${on.lines} on its lines, ${on.circles} on its circles and ${on.fog} with its sight pass (every row put off: ${off.rust}, ${off.lines}, ${off.circles} and ${off.fog} of ${off.ticks}) · the senses asked of it: ${SENSES.map(k => `${k} ${on.sensed[k]}`).join(', ')} (without: ${SENSES.map(k => off.sensed[k]).join(', ')}) · word ${on.word}${on.word === off.word ? ', the same' : ` ≠ ${off.word}`}`);
  // (a recipe made with no word about the rows carries the page's own: the rows of RUST_ON, and no other — the stand a game server plays today. A section of
  //  its own, so that its red path is its own: the box world's break must move ITS word, with no maths row to do it)
  const plain = (r: any) => S.rustRows().every((k: string) => table(r)?.[k] === S.rustOn(k)), T = (r: string) => S.rustOn(r) ? TICKS : 0;
  const okDef = !!def && !!off && plain(rDef) && def.ticks === TICKS && def.rust === T('maths') && def.lines === T('lines') && def.circles === T('circles') && def.fog === T('fog') && def.sees > 100 && def.exact && def.word === off.word;
  verdict(okDef, 'server as the page: a recipe that says nothing of the rows carries the page\'s own (RUST_ON), and the game stood from it is the game with every row off', !def || !off ? `a stand left nothing (recipe: ${!!rDef} · stands: ${!!def} ${!!off})` :
    `the recipe's rows [${S.rustRows().filter((k: string) => table(rDef)?.[k]).join(' ')}] · of ${def.ticks} ticks ${def.rust} on the module's maths, ${def.lines} on its lines, ${def.circles} on its circles, ${def.fog} with its sight pass (${def.sees} men asked of it) · word ${def.word}${def.word === off.word ? ', the word with every row off' : ` ≠ ${off.word}`}`);
}

// ---- rows: every row of RUST is held by a section of this half that ran (a row added to src/sim/sim.ts with no line in HELD is red here, before anyone trusts it)
{
  const rows: string[] = S.rustRows(), none = rows.filter(r => !(r in HELD) || !(HELD[r] in said)), gone = Object.keys(HELD).filter(r => !rows.includes(r));
  verdict(none.length === 0 && gone.length === 0, 'rows: every row of RUST has a section here that asks its own answers of both sides',
    none.length || gone.length ? `${none.length ? `NO SECTION RAN FOR ${none.map(r => `${r} (${HELD[r] ?? 'no line in HELD'})`).join(', ')}` : ''}${gone.length ? ` · named here and not a row: ${gone.join(', ')}` : ''}` : rows.map(r => `${r} → ${HELD[r]}`).join(' · '));
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

// ---- missions, both
if (args.includes('--missions') || args.includes('--both')) {
  const runs = (flag('--runs') ?? RUNS.join(',')).split(','), node = args.includes('--node'), all: string[] = S.rustRows(), up = (flag('--rows') ?? all.join(',')).split(',');
  for (const r of up) if (!all.includes(r)) { console.error(`rust-check: no row "${r}" (rows: ${all.join(' ')})`); process.exit(2); }
  interface Doors { there: number; here: number; turned: number; noises: number; both: number; differ: number }
  interface Falls { there: number; here: number; asked: number; differ: number; first: string; dues: number; steps: number; crunches: number }
  interface Run { falls: Falls; doors: Doors; run: string; ticks: number; cone: number; end: string; digest: string; bits: string; rust: number; lines: number; circles: number; fog: number; sees: number; sensed: Sensed; poses: number; posesIn: number; posed: number; sets: number; layers: number; dice: number; draws: number; both: { asked: number; differ: number; stale: number; first: string }; exact: boolean; engine: string; secs: number }
  const one = (run: string, rustOn: boolean, both = false): Promise<Run | string> => new Promise(done => {
    const cmd = node ? [...NODE, 'tools/qa/rust-check.ts', '--run', run] : ['bun', 'run', 'tools/qa/rust-check.ts', '--run', run];
    const env: Record<string, string | undefined> = { ...process.env, TPS_PROFILE: 'shipped', TPS_CPUSENSE: '1', PT_DIGEST: '1', RUST_BOTH: both ? '1' : '0', ...(node ? { V8_ENTRY: resolve(ROOT, 'tools/qa/rust-check.ts'), FC_NOSTANDIN: '1' } : {}) };
    for (const r of all) env['TPS_RUST' + r.toUpperCase()] = rustOn && up.includes(r) ? '1' : '0';
    if (up.includes('dice')) env.TPS_SEED = run.split(':')[1];   // (the die's row moves the game's OWN die: both of a pair are stood up with one, off the run's seed)
    const p = spawn(cmd[0], cmd.slice(1), { cwd: ROOT, env: env as Record<string, string> }); let out = '', err = '';
    p.stdout.on('data', (d: unknown) => { out += d; }); p.stderr.on('data', (d: unknown) => { err += d; });
    p.on('close', code => { const r = marked(out, 'RUST-RUN '); done(r ?? `left nothing (exit ${code}): ${err.trim().split('\n').slice(-3).join(' / ').slice(0, 300)}`); });
    p.on('error', e => done(String(e)));
  });
  /** the fewest questions of each sense a mission asks the module with its row up (a squad mission asks each thousands of times; the smoke only where a
   *  canister is thrown, which not every run has: no floor a run, and SET_FLOOR over the runs together) */
  const SENSE_FLOOR: Sensed = { sight: 1000, light: 1000, smoke: -1, sound: 100, paths: 100 };
  const SET_FLOOR: Sensed = { sight: 1000, light: 1000, smoke: 1000, sound: 100, paths: 100 };
  /** the senses raised that the runs TOGETHER did not ask the module enough of (a set of runs with no smoke in it proves nothing of the smoke row) */
  const thin = (rs: (Run | string)[]) => SENSES.filter(k => up.includes(k) && rs.reduce((n, r) => n + (typeof r === 'string' ? 0 : r.sensed[k]), 0) <= SET_FLOOR[k]);
  /** the fewest layers of the animator a tick that a mission runs in the module with the walk's row up (a squad mission runs about sixty and the office night with three guards
   *  thirty-six; the rule that has no pose alone is about twelve) */
  const LAYER_FLOOR = 20;
  /** did the run with the rows up really run on them: every tick on the module's maths and with its lines asked there (each where its row was raised), and none in the other */
  const fair = (ts: Run, rs: Run) => (rs.run.endsWith(':hard') ? rs.cone === rs.ticks && ts.cone === ts.ticks : rs.cone === 0 && ts.cone === 0) && (!up.includes('maths') || rs.rust === rs.ticks) && (!up.includes('lines') || rs.lines === rs.ticks) && (!up.includes('circles') || rs.circles === rs.ticks) && (!up.includes('fog') || (rs.fog === rs.ticks && rs.sees > 100)) && ts.rust === 0 && ts.lines === 0 && ts.circles === 0 && (!up.includes('poses') || (rs.poses > rs.ticks && rs.posed === rs.ticks && rs.sets > 1 && rs.posesIn > rs.poses * 0.9)) && (up.includes('walk') ? rs.layers > LAYER_FLOOR * rs.ticks : rs.layers === 0) && ts.layers === 0 && ts.posed === 0 && ts.posesIn === 0 && (!up.includes('dice') || (rs.dice === rs.ticks && rs.draws > 100 && rs.draws === ts.draws)) && ts.dice === 0 && ts.fog === 0 && ts.sees === 0 && ts.poses === 0 && ts.ticks > 600 && ts.falls.there === 0 && ts.doors.there === 0 && (!up.includes('doors') || (rs.doors.there > 0.95 * (rs.doors.there + rs.doors.here) && rs.doors.there > 0.9 * rs.ticks && rs.doors.turned > 100)) && SENSES.every(k => ts.sensed[k] === 0 && (!up.includes(k) || rs.sensed[k] > SENSE_FLOOR[k]));
  const sensedSay = (r: Run) => SENSES.map(k => `${k} ${r.sensed[k]}`).join(', ');
  if (args.includes('--missions')) {
    let ok = true; const lines: string[] = []; let ticks = 0; const raised: (Run | string)[] = [];
    for (const run of runs) {
      const [ts, rs] = await Promise.all([one(run, false), one(run, true)]); raised.push(rs);
      if (typeof ts === 'string' || typeof rs === 'string') { ok = false; lines.push(`${run}: ${typeof ts === 'string' ? `TypeScript ${ts}` : ''} ${typeof rs === 'string' ? `Rust ${rs}` : ''}`); continue; }
      const same = ts.bits === rs.bits && ts.ticks === rs.ticks && ts.end === rs.end && ts.digest === rs.digest, honest = fair(ts, rs);
      if (!same || !honest) ok = false; ticks += ts.ticks;
      lines.push(`${run}: ${ts.ticks} ticks${rs.cone ? ` (${rs.cone} of the Rust run's with every eye a cone)` : ''}, ends ${ts.end} · ${same ? `to the bit (${ts.bits})` : `DIFFERS — TypeScript ${ts.ticks} ticks ${ts.bits} ${ts.digest}, Rust ${rs.ticks} ticks ${rs.bits} ${rs.digest}`}${honest ? '' : ` · NOT A FAIR PAIR (of ${rs.ticks} ticks in the Rust run: ${rs.rust} on its maths, ${rs.lines} with its lines, ${rs.circles} with its circles, ${rs.fog} with its sight pass (${rs.sees} men asked), ${rs.poses} poses asked (${rs.posesIn} of them where they live, ${rs.sets} men's poses in the block for ${rs.posed} ticks), ${rs.dice} with its die at home (${rs.draws} draws, ${ts.draws} in the other), ${rs.doors.there} door steps in the module and ${rs.doors.here} not (${rs.doors.turned} leaf turns), ${rs.layers} layers of the animator in the module (${ts.layers} in the other), the senses ${sensedSay(rs)}; in the other: ${ts.rust}, ${ts.lines} and ${ts.circles} of ${ts.ticks}, the senses ${sensedSay(ts)})`} · senses asked of the module: ${sensedSay(rs)}${up.includes('doors') ? ` · door steps in the module: ${rs.doors.there} (${rs.doors.turned} leaf turns, ${rs.doors.noises} noises)` : ''}${up.includes('falls') ? ` · rules of the falls in the module: ${rs.falls.there} (${rs.falls.steps} footfalls asked, ${rs.falls.crunches} on glass)` : ''} · ${ts.secs} s / ${rs.secs} s`);
      console.error(`  ${lines[lines.length - 1]}`);
    }
    { const t = thin(raised); if (t.length) { ok = false; lines.push(`NOT A FAIR SET: the runs together asked the module too little of ${t.join(', ')}`); } }
    verdict(ok, `missions: ${runs.length} runs played with the rows ${up.join(' + ')} in Rust and with none are the same game after every update (${node ? 'node' : 'bun'})`, `${ticks} ticks compared\n    ${lines.join('\n    ')}`);
  }
  if (args.includes('--both')) {
    let ok = true; const lines: string[] = []; let asked = 0;
    const got = await Promise.all(runs.map(run => one(run, true, true)));
    runs.forEach((run, i) => { const r = got[i];
      if (typeof r === 'string') { ok = false; lines.push(`${run}: ${r}`); return; }
      const world = up.some(k => k === 'lines' || k === 'circles' || k === 'fog' || (SENSES as readonly string[]).includes(k));   // (a row that asks the box world asks it some hundred thousand times a run; the doors' step is one question a tick)
      const b = r.both, good = b.asked > (world ? 1e5 : up.includes('doors') ? 1000 : -1) && b.differ === 0 && b.stale === 0 && r.falls.differ === 0 && (!up.includes('doors') || (r.doors.both > 0.9 * r.ticks && r.doors.differ === 0 && r.doors.turned > 100)) && (!up.includes('lines') || r.lines === r.ticks) && (!up.includes('circles') || r.circles === r.ticks) && (!up.includes('fog') || (r.fog === r.ticks && r.sees > 100)) && SENSES.every(k => !up.includes(k) || r.sensed[k] > SENSE_FLOOR[k]); if (!good) ok = false; asked += b.asked;
      lines.push(`${run}: ${r.ticks} ticks (${r.lines} with the module asked), ${b.asked} questions answered both ways (${r.sees} of them the fog's; the senses ${sensedSay(r)}; ${r.doors.both} the doors' step, ${r.doors.turned} leaf turns; ${r.falls.asked} rules of the falls${r.falls.first ? ` — ${r.falls.first}` : ''}) · ${b.differ} answers differ · ${b.stale} stale rows${b.first ? ` · first: ${b.first}` : ''} · ${r.secs} s`); });
    { const t = thin(got); if (t.length) { ok = false; lines.push(`NOT A FAIR SET: the runs together asked the module too little of ${t.join(', ')}`); } }
    verdict(ok, `both: every line the missions ask of the module is the TypeScript's answer, and the module's copy is the boxes at every question (${node ? 'node' : 'bun'})`, `${asked} questions over ${runs.length} runs\n    ${lines.join('\n    ')}`);
  }
}

console.log(fails ? `\n${fails} FAILED` : '\nall pass');
// (the red path: every section that asks the module must be red, and every row's own answers must have differed in the section that holds it — one left green
//  is a check that proves nothing, and exit 3 says which)
const greenRows = BREAK ? (S.rustRows() as string[]).filter(r => !red[r]) : [], greenSections = BREAK ? RED_SECTIONS.filter(k => said[k] === true) : [];
if (BREAK) console.log(greenRows.length || greenSections.length ? `THE RED PATH IS NOT RED FOR ${[...greenSections.map(k => `the section \`${k}\``), ...greenRows.map(r => `the row ${r} (in \`${HELD[r] ?? 'no section'}\`)`)].join(', ')}`
  : `the red path is red for every section that ran (${RED_SECTIONS.filter(k => k in said).join(' · ')}) and for every row's own answers (${(S.rustRows() as string[]).join(', ')})`);
process.exit(greenRows.length || greenSections.length ? 3 : fails ? 1 : 0);
