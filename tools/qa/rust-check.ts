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
//                                                        the die's word has a bit turned, the doors' step, the furniture's and a loose slab's are handed a tick one unit in the last place
//                                                        too long and the falls a landing's glass as much too heavy. Then (1) every section that asks the module's answers must FAIL
//                                                        (RED_SECTIONS: `maths`, `engines`, `lines, circles`, `senses`, `poses`, `doors`, `props`, `slabs`, `falls`, `state, dice`, `server`,
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
//   lines     the box world's line tests AND ITS RAY (rust/sim/src/lines.rs over the copy src/scene/boxMirror.ts keeps; a ray: the nearest thing met, how far, by
//             which face — over leaves, tilted, round and cut boxes): tools/qa/segment-ident.ts --rust — random segments
//             over whole levels (door leaves swung, walls broken into pieces and re-shaped, tilted and cut boxes, panes with holes) answered by the module,
//             by the TypeScript walk as it is and by the walk as it first was: every answer and every pane's place the same bits
//   glass     where a pane of glass is open (rust/sim/src/glass.rs over the copy of every pane that is no longer whole; src/game/glassbreak.ts openAt, goingAt
//             and a burst's reckoning of glass it has not broken; glassPattern.ts's pieces and clumps): tools/qa/pane-ident.ts — the glass of four levels broken
//             by the game's own rule (rounds, pellets, blasts, the glass's step, panes made another glass and made whole), and points on and off every pane
//             asked of the rule and of the module under each view a burst judges by, with a solid line and a ray through each asked of both walks. It is no row of
//             RUST: the lines' and the ray's rows answer by it wherever they meet a pane. The shared numbers are read out of both sources.
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
//   props     the furniture men shove (rust/sim/src/props.rs against src/game/props.ts PropSystem.update, each thing's numbers columns of the game's block):
//             tools/qa/prop-ident.ts — every chair, carton, bin and plant of four levels stepped both ways from the same state over made-up men, with rounds
//             and blasts into it, things taken out, leaves swinging through it and torn off, chairs shoved through door frames, the castors, the friction
//             and ticks of other lengths turned over: every column, the clock, every man's place, every noise, every box the same bits, and a run fair
//             only if it met everything the step does. The shared numbers are read out of both sources. `missions` and `both` hold it in play.
//   slabs     a leaf off its hinges, in the air (rust/sim/src/slabs.rs against src/game/doorblast.ts stepSlab, each slab's numbers a row of the game's block):
//             tools/qa/slab-ident.ts — leaves torn out of four levels from either face, hard and weakly, two at once, taken apart by gunfire, thrown at
//   items     what is thrown or dropped (rust/sim/src/items.rs against src/game/throwables.ts stepRow, each thing's numbers a row of the game's block),
//             and the furniture's ray it needs (lines.rs ray_things against props.ts PropSystem.raycast, over the box world's copy of the furniture's
//             parts): tools/qa/item-ident.ts — things of every kind thrown, dropped, set down on chairs, sent at panes and over made-up snow, taken in a
//             hand, their time run out, the pool driven past the module's rows, the furniture shoved from under them — both ways and as twins
//             panes of glass, their doors hung again, stepped both ways from the same rows: every column, the rows' tables, every box, noise, tally and
//             log line the same bits, and a run fair only if it met everything the step does (a ray through a pane that is gone among them). The
//             shared numbers are read out of both sources. `missions` and `both` hold it in play where a mission takes a leaf off.
//   falls     what falls and what it leaves (rust/sim/src/falls.rs against src/sim/fallshome.ts: the landings the simulation has scheduled and the litter
//             map of broken glass, a part of the game's block): tools/qa/falls-ident.ts — two hundred thousand rules (schedule past the list's cap, deposit,
//             the landings due, a footfall's step, the floor told, clear, the row dropped and raised) each taken both ways and against a twin store that
//             never goes near the module, in both lists of landings (glass, thrown pieces): every answer and every double the same. In play it is held where
//             glass breaks and where things break: the `module` and `snapshot` lines of tools/qa/glassfall-check.ts and tools/qa/debrisfall-check.ts (a
//             scene with the row on against off, and a snapshot put back while things are in the air). `missions` raises the row with the rest, but the
//             five missions break nothing that falls: there the row is only shown to do no harm.
//   stand     both stands have the rows of src/sim/sim.ts RUST_ON on, and the game's own maths, as the page, and NO WORD moves either: a retired word
//             in the environment (TPS_RUST, TPS_RUST<ROW>, TPS_EXACT) stops the tool that says it, by name; FROZEN_HOLDS holds no row and no maths; the
//             address's old words (?rust=0, ?rust<row>, ?noexact) write nothing (profile.ts pageRows), also over rows a recipe wrote; and a check's own
//             arm (QA_HOLD, tools/qa/headless.ts hold) holds what it names and it alone
//   server    the game server's own stand (tools/mp/stand.ts): a recipe carries no row (a setup that asks for one is refused: the rows are no knob table),
//             the game stood from it in another process ticks with the build's rows, and after 600 ticks of a walking, shooting seat its state word is the
//             word of the same recipe stood with every row held down, and with the maths' row held up
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
const RUNS = ['compound-four:1', 'warehouse-four:1', 'pass-four:1', 'hostage-rescue:1', 'block-squads:1', 'hostage-rescue:1:hard', 'warehouse-four:1@frag', 'compound-four:1@frag'];   // (the last two: frags ordered through doors, which takes leaves off their hinges — the one thing of the things' rows no other run does)
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
  const DH: any = await import(`${ROOT}/src/sim/doorshome.ts`), FH: any = await import(`${ROOT}/src/sim/fallshome.ts`), PH: any = await import(`${ROOT}/src/sim/propshome.ts`), SH: any = await import(`${ROOT}/src/sim/slabshome.ts`), IH: any = await import(`${ROOT}/src/sim/itemshome.ts`); FH.FALLS.both = process.env.RUST_BOTH === '1';
  // (the red path of the die: once, fifty binds in, the lowest bit of the word in the game's block is turned — a wrong die that still rolls)
  if (BREAK) { const P = ST.SimState.prototype, bind = P.bind; let n = 0; P.bind = function (this: any) { const slot = bind.call(this); if (++n === 50) this.w[(ST.simPart(ST.P_DICE) + 8) >>> 2] ^= 1; return slot; }; }
  let posed = 0, sets = 0;   // (the ticks that ended with poses of its men in the game's block, and how many men's)
  // (the red path of the layers: from the fiftieth tick's opening every knob is handed to the module a hundredth large)
  if (BREAK) { const RK: any = await import(`${ROOT}/src/game/rustknobs.ts`); RK.RUST_KNOBS.hand = (_i: number, v: number) => RK.RUST_KNOBS.written >= 50 ? v * 1.01 : v; }
  let homed = 0, draws = 0;   // (the ticks that ended with the game's own die at home in the module's block, and the draws taken off it by the end)
  let h: number = RAW_BITS_SEED, ticks = 0, broke = false, inRust = 0, cone = 0;   // (inRust: the ticks whose update found the module's sine in Math's place — the play-through puts every knob table back when it ends, so the row is read while it runs)
  const SLABF = new Float64Array(1), SLABU = new Uint32Array(SLABF.buffer);
  const update = Game.prototype.update;
  Game.prototype.update = function (this: unknown, ...a: unknown[]) {
    // (the red path: from the first tick the Rust set is handed the engine's cosine — a wrong arithmetic that still plays)
    if (BREAK && !broke && S.RUST.maths) { broke = true; wrongMaths(S, X); }
    if (broke || Math.sin === S.sim().maths_sin) inRust++; if (FW.FOG.cone) cone++;   // (cone: the ticks whose eyes were wedges — Hard's fog)
    const r = (update as (...x: unknown[]) => unknown).apply(this, a); h = rawBits(this as never, h); ticks++;
    // (rawBits folds a door's angle and nothing of a leaf off its hinges: every slab's row is folded here, so a run that takes a leaf off holds the slabs'
    //  row by more than what a slab did to the men; a run with no slab folds nothing more and keeps its word)
    { const T = (this as any).doors?.slabs, n: number = T ? T.n : 0; if (n) { const d = T.d, u = T.u, O = T.O;
        for (let q = 0; q < n; q++) { for (const k of SH.SLAB_F64) { SLABF[0] = d[T.b8 + O[k] + q]; h = Math.imul(h ^ SLABU[0], 0x01000193) >>> 0; h = Math.imul(h ^ SLABU[1], 0x01000193) >>> 0; }
          h = Math.imul(h ^ u[T.b4 + O.flags + q], 0x01000193) >>> 0; h = Math.imul(h ^ u[T.b4 + O.door + q], 0x01000193) >>> 0; } } }
    // (…and of a thrown thing the raw bits fold where it is and nothing else: every row of the pool is folded here — how it moves, turns and tumbles, its
    //  clocks, what it lies on — so the two runs of a pair are held to the same pool after every tick, not to the same places alone)
    { const T = (this as any).items?.store, n: number = T ? T.n : 0; if (n) { const d = T.d, u = T.u, O = T.O;
        for (let q = 0; q < n; q++) { for (const k of IH.ITEM_F64) { SLABF[0] = d[T.b8 + O[k] + q]; h = Math.imul(h ^ SLABU[0], 0x01000193) >>> 0; h = Math.imul(h ^ SLABU[1], 0x01000193) >>> 0; }
          for (const k of IH.ITEM_U32) h = Math.imul(h ^ u[T.b4 + O[k] + q], 0x01000193) >>> 0; } } }
    { const d = (this as any).dice; if (d) { if (d.home) homed++; draws = d.n; } }
    { const ps = (this as any).poseSets; if (ps.length) posed++; sets = ps.length;
      // (the red path of the poses at home: from the fiftieth such tick a pose's floats are handed to the TypeScript ten singles off from where the module has them)
      if (BREAK && posed === 50) { const f = ST.SimState.prototype.f32; ST.SimState.prototype.f32 = function (this: unknown, at: number, n: number) { return f.call(this, at + 40, n); }; } }
    return r;
  } as typeof update;
  const t0 = performance.now();
  const r = await PT.play(scn, Number(seed), 300, false, how === 'hard' ? { DIFFICULTY: { level: 'hard' } } : {});
  console.log('RUST-RUN ' + JSON.stringify({ run: args[1], ticks, cone, end: String(r.end), digest: String(r.digest), bits: h.toString(36), rust: inRust, lines: lines.n, circles: lines.circles, fog: lines.fog, sees: lines.sees, sensed: lines.sensed, poses: SK.POSE_BENCH.asked, posesIn: SK.POSE_BENCH.inPlace, posed, sets, layers: SK.LAYERS.ran, dice: homed, draws, doors: DH.DOORS_STEPPED, props: PH.PROPS_STEPPED, slabs: SH.SLABS_STEPPED, items: IH.ITEMS_STEPPED, falls: FH.FALLS, rays: C.RAYS.n, both: C.BOTH, exact: X.EXACT.on, engine, secs: Math.round((performance.now() - t0) / 100) / 10 }));
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
  console.log('RUST-ROWS ' + JSON.stringify({ profile: P.harnessProfile(), maths: S.RUST.maths, exact: (await import(`${ROOT}/src/math/exact.ts`)).EXACT.on, on: S.rustRows().filter((r: string) => S.RUST[r]), rows: S.rustRows() }));
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
const HELD: Record<string, string> = { maths: 'maths', lines: 'lines, circles', circles: 'lines, circles', fog: 'senses', poses: 'poses', sight: 'senses', light: 'senses', smoke: 'senses', sound: 'senses', paths: 'senses', dice: 'state, dice', doors: 'doors', falls: 'falls', props: 'props', slabs: 'slabs', items: 'items', walk: 'layers', rays: 'lines, circles' };
/** …and the sections that must be red on the red path whatever rows they hold (each asks the module's answers; one left green there holds nothing) */
const RED_SECTIONS = ['maths', 'engines', 'lines, circles', 'glass', 'senses', 'poses', 'layers', 'doors', 'props', 'slabs', 'items', 'falls', 'state, dice', 'server', 'server as the page', 'missions', 'both'];
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
  { const d = marked(r.out, 'SEGMENT-IDENT '); red.lines = !!d && d.lines > 0; red.rays = !!d && d.rays > 0; red.circles = !!d && d.circles > 0; }
  const rays = r.out.split('\n').find(l => /^rays: /.test(l)) ?? '';
  verdict(ok && rays !== '', 'lines, circles: every line through the box world, every ray into it and every circle against it gets the TypeScript\'s answer from the Rust module', rows.length ? rows.map(l => l.replace(/\s+/g, ' ').trim()).join('\n    ') + '\n    ' + rays : `the differential left nothing (exit ${r.status}): ${(r.err || r.out).trim().split('\n').slice(-2).join(' / ').slice(0, 300)}`);
}

// ---- glass
{
  // (on both stands: toughened glass comes down in a wave on the frozen one and drops after a beat on the page's)
  const on = (stand: string) => child(['bun', 'run', 'tools/qa/pane-ident.ts'], { TPS_PROFILE: stand, PANES_BREAK: BREAK ? '1' : undefined }), both = [on('shipped'), on('frozen')];
  const r = { status: both.every(x => x.status === 0) ? 0 : 1, out: both.map(x => x.out).join('\n'), err: both.map(x => x.err).join('\n') };
  const rows = r.out.split('\n').filter(l => /\d+ sheets, /.test(l)), sum = r.out.split('\n').filter(l => /^pane-ident: /.test(l)).join('\n    ');
  const fs = await import('node:fs'), rd = (f: string) => fs.readFileSync(resolve(ROOT, f), 'utf8'), rs = rd('rust/sim/src/glass.rs') + rd('rust/sim/src/world.rs') + rd('rust/sim/src/pattern.rs'), mirror = rd('src/scene/boxMirror.ts'), pattern = rd('src/game/glassPattern.ts');
  const there = (name: string): number => { const m = new RegExp(`(?:pub )?const ${name}: \\w+ = ([^;]+);`).exec(rs); return m ? Number(eval(m[1].replace(/_/g, ''))) : NaN; };
  const inTs = (src: string, name: string): number => Number(new RegExp(`\\b${name} = (-?[\\d.]+)`).exec(src)?.[1] ?? NaN), off: string[] = []; let shared = 0;
  // the row's places and the tables' numbers as boxMirror.ts writes them, and the pattern's own numbers, each against the Rust source
  for (const [h, t] of [['GD', 'GD'], ['GW', 'GW'], ['G_SITE', 'G_SITE'], ['G_DICE_AT', 'G_DICE_AT'], ['GW_FLAGS', 'GW_FLAGS'], ['GW_COLS', 'GW_COLS'], ['GW_ROWS', 'GW_ROWS'], ['GW_DSEED', 'GW_DSEED'], ['GW_N', 'GW_N'], ['GW_SEED', 'GW_SEED'],
    ['GW_OPEN', 'GW_OPEN'], ['GW_GOING', 'GW_GOING'], ['GW_CRATER', 'GW_CRATER'], ['G_GONE', 'G_GONE'], ['G_DICE', 'G_DICE'], ['G_RUN', 'G_RUN'], ['VD', 'VD'], ['FRAGS_PER_SITE', 'FRAGS_PER_SITE'], ['SECTORS', 'MAX_SECTORS']]) { shared++; if (inTs(mirror, h) !== there(t)) off.push(`${h} ${inTs(mirror, h)} in boxMirror.ts, ${t} ${there(t)} there`); }
  for (const [h, t] of [['GP_MAX_SECTORS', 'MAX_SECTORS'], ['GP_ANG_JIT', 'ANG_JIT'], ['GP_RHO_LO', 'RHO_LO'], ['GP_RHO_SPAN', 'RHO_SPAN'], ['GP_SPLIT_LO', 'SPLIT_LO'], ['GP_SPLIT_SPAN', 'SPLIT_SPAN'], ['GP_CUT_NONE', 'CUT_NONE'], ['GP_CUT_LO', 'CUT_LO'],
    ['GP_CUT_SPAN', 'CUT_SPAN'], ['GP_CUT_TILT', 'CUT_TILT'], ['GP_DICE_JIT', 'DICE_JIT'], ['GP_DICE_DELAY', 'DICE_DELAY'], ['GP_REMAINDER', 'REMAINDER']]) { shared++; if (inTs(pattern, h) !== there(t)) off.push(`${h} ${inTs(pattern, h)} in glassPattern.ts, ${t} ${there(t)} there`); }
  { shared++; const sites = inTs(pattern, 'GP_MAX_SITES'), bands = inTs(pattern, 'GP_BANDS'), gs = inTs(mirror, 'G_SITES'), bits = inTs(mirror, 'GW_BITS');   // (a row holds every site a pane can have, and a bit a piece of each)
    if (gs !== sites || bands * there('MAX_SECTORS') !== there('FRAGS_PER_SITE') || bits * 32 !== sites * there('FRAGS_PER_SITE') || there('G_DICE_AT') !== sites * there('G_SITE') || there('GW_OPEN') - there('GW_SEED') !== sites) off.push(`a row's room: ${sites} sites of ${bands} bands, ${gs} in a row, ${bits} words of bits`); }
  for (const k of ['VIEW_NONE', 'VIEW_BLOWN', 'VIEW_GIVES']) { shared++; if (!(there(k) >= 0) || there(k) !== inTs(mirror, k)) off.push(`${k} ${inTs(mirror, k)} in boxMirror.ts, ${there(k)} there`); }
  const ok = r.status === 0 && rows.length >= 8 && (r.out.match(/=== pane-ident \(the (page's|frozen) stand\): PASS/g) ?? []).length === 2 && off.length === 0 && shared > 30;
  verdict(ok, 'glass: what a pane is to play answers as it was recorded, every name reached — and the two sides name their numbers alike',
    (rows.length ? rows.map(l => l.replace(/\s+/g, ' ').trim()).join('\n    ') + '\n    ' + sum.slice(0, 1800) : `the differential left nothing (exit ${r.status}): ${(r.err || r.out).trim().split('\n').slice(-2).join(' / ').slice(0, 300)}`)
    + `\n    ${shared} numbers shared with the Rust source (boxMirror.ts, glassPattern.ts)${off.length ? ` — NOT THE SAME: ${off.slice(0, 8).join(', ')}` : ', each the same'}`);
}

// ---- senses
{
  const r = child(['bun', 'run', 'tools/qa/sense-ident.ts', '--n', flag('--questions') ?? '12000', '--levels', flag('--levels') ?? 'office.json,warehouse.json,compound.json,glasshouse.json']);
  const rows = r.out.split('\n').filter(l => /questions a sense|FAIL/.test(l));
  // the numbers the host writes and reads by (boxMirror.ts FACE) against the Rust sources' own, name for name
  const fs = await import('node:fs'), face = /export const FACE = \{([^}]*)\}/.exec(fs.readFileSync(resolve(ROOT, 'src/scene/boxMirror.ts'), 'utf8'))?.[1] ?? '';
  const rust = ['light', 'smoke', 'hear', 'sight'].map(f => fs.readFileSync(resolve(ROOT, `rust/sim/src/${f}.rs`), 'utf8')).join('\n');
  const pairs = [...face.matchAll(/(\w+):\s*(\d+)/g)].map(m => ({ name: m[1], here: Number(m[2]), there: Number(new RegExp(`pub const ${m[1]}: \\w+ = (\\d+);`).exec(rust)?.[1] ?? NaN) }));
  { const nav = /export const NAV = \{([^}]*)\}/.exec(fs.readFileSync(resolve(ROOT, 'src/scene/boxMirror.ts'), 'utf8'))?.[1] ?? '', there = fs.readFileSync(resolve(ROOT, 'rust/sim/src/nav.rs'), 'utf8');   // (boxMirror.ts NAV: what a search answers that is no count)
    for (const m of nav.matchAll(/(\w+):\s*(-?\d+)/g)) pairs.push({ name: `nav ${m[1]}`, here: Number(m[2]), there: Number(new RegExp(`pub const ${m[1]}: \\w+ = (-?\\d+);`).exec(there)?.[1] ?? NaN) }); }
  // (the order a priced search asks its prices in: the same check with the host pricing last first must end with the paths differing and nothing else,
  //  exit 1. Not under RUST_BREAK, which is another red path of the same tool)
  const order = BREAK ? null : child(['bun', 'run', 'tools/qa/sense-ident.ts', '--n', '3000', '--levels', 'office.json,compound.json'], { PRICED_BREAK: '1' });
  const orderRed = !order || (order.status === 1 && /the paths differed and nothing else/.test(order.out));
  // (…and the lamps in parts: the same check with every part beginning its sum again must end with the lamps in parts differing and nothing else)
  const parts = BREAK ? null : child(['bun', 'run', 'tools/qa/sense-ident.ts', '--n', '300', '--levels', 'office.json,compound.json'], { LAMPS_BREAK: '1' });
  const partsRed = !parts || (parts.status === 1 && /the lamps in parts differed and nothing else/.test(parts.out));
  // (…and the smoke's shadow where the grid cannot answer: the same check with the module handed no blobs must end with those two cases differing and nothing else)
  const shade = BREAK ? null : child(['bun', 'run', 'tools/qa/sense-ident.ts', '--n', '300', '--levels', 'office.json,compound.json'], { LAMPS_BREAK: 'shade' });
  const shadeRed = !shade || (shade.status === 1 && /the smoke's shadow where the grid cannot answer differed and nothing else/.test(shade.out));
  // (…and the blobs: a game keeps no more than the module's table holds — sense.ts CpuSmoke.emit — so the knob as it ships must fit it: a build's mistake otherwise)
  const SE: any = await import(`${ROOT}/src/game/sense.ts`), BMF: any = (await import(`${ROOT}/src/scene/boxMirror.ts`)).FACE, blobsFit = SE.SENSE.smoke.cap <= BMF.MAX_BLOBS;
  const off = pairs.filter(q => q.here !== q.there), ok = r.status === 0 && rows.length > 0 && rows.every(l => / PASS /.test(l)) && pairs.length >= 20 && off.length === 0 && orderRed && partsRed && shadeRed && blobsFit;
  { const d = marked(r.out, 'SENSE-IDENT '); for (const k of [...SENSES, 'fog']) red[k] = !!d && d[k] > 0; }
  verdict(ok, 'senses: an eye on a man, the light on a point, the smoke along a line, a sound\'s way, a way across the floor and what a team\'s eyes see get the TypeScript\'s answer from the Rust module', (rows.length ? rows.map(l => l.replace(/\s+/g, ' ').trim().slice(0, 420)).join('\n    ') : `the differential left nothing (exit ${r.status}): ${(r.err || r.out).trim().split('\n').slice(-2).join(' / ').slice(0, 300)}`)
    + `\n    ${pairs.length} numbers shared with the Rust sources (boxMirror.ts FACE, NAV)${off.length ? ` — NOT THE SAME: ${off.map(q => `${q.name} ${q.here} here, ${q.there} there`).join(', ')}` : ', each the same'}`
    + (order ? `\n    a priced search's prices asked for last first (PRICED_BREAK=1): ${orderRed ? 'the paths differ and nothing else, as they must' : `NOT RED AS IT MUST BE (exit ${order.status}): ${order.out.trim().split('\n').slice(-1)[0].slice(0, 200)}`}` : '')
    + (parts ? `\n    the lamps of a question in parts, every part beginning its sum again (LAMPS_BREAK=1): ${partsRed ? 'the lamps in parts differ and nothing else, as they must' : `NOT RED AS IT MUST BE (exit ${parts.status}): ${parts.out.trim().split('\n').slice(-1)[0].slice(0, 200)}`}` : '')
    + (shade ? `\n    a lamp's shadow with the module handed no blobs (LAMPS_BREAK=shade): ${shadeRed ? 'the two cases where the grid cannot answer differ and nothing else, as they must' : `NOT RED AS IT MUST BE (exit ${shade.status}): ${shade.out.trim().split('\n').slice(-1)[0].slice(0, 200)}`}` : '')
    + `\n    the smoke's blobs: the game keeps ${SE.SENSE.smoke.cap} at most and the module's table holds ${BMF.MAX_BLOBS}${blobsFit ? '' : ' — THE KNOB IS PAST THE TABLE'}`);
}

// ---- poses
{
  // (asked under BOTH pose rules, anim/skeleton.ts POSEMATH: in doubles, and in singles — the host says the rule in each call's `how`, and only a run with
  //  the rule on asks the module's lanes and the twin's rounding)
  const one = (singles: boolean) => child(['bun', 'run', 'tools/qa/pose-ident.ts', '--n', flag('--poses') ?? '4000'], { TPS_POSESINGLES: singles ? '1' : '0', POSEMATH_BREAK: undefined });
  const r = one(false), r1 = one(true);
  const rowOf = (x: { out: string }) => x.out.split('\n').find(l => /^pose-ident: \d+ rounds/.test(l)), row = rowOf(r), row1 = rowOf(r1);
  const ok = r.status === 0 && !!row && /=== pose-ident: PASS/.test(r.out) && r1.status === 0 && !!row1 && /=== pose-ident: PASS/.test(r1.out);
  red.poses = (marked(r.out, 'POSE-IDENT ')?.differ ?? 0) > 0 && r.status !== 3 && (marked(r1.out, 'POSE-IDENT ')?.differ ?? 0) > 0 && r1.status !== 3;   // (its exit 3 = a KIND of posing question the red path left green: not red, however many of the others differed)
  const say = (x: { out: string; err: string; status: number | null }, rw: string | undefined) => rw ? rw.replace(/^pose-ident: /, '') + (x.status === 3 ? `\n    ${x.out.trim().split('\n').pop()}` : '') : `the differential left nothing (exit ${x.status}): ${(x.err || x.out).trim().split('\n').slice(-2).join(' / ').slice(0, 300)}`;
  verdict(ok, 'poses: every pose the Rust module samples, blends, runs forward, turns and solves is the TypeScript\'s, single for single, in doubles and under the rule in singles', `in doubles: ${say(r, row)}\n    in singles: ${say(r1, row1)}`);
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

// ---- props
{
  const r = child(['bun', 'run', 'tools/qa/prop-ident.ts', '--n', flag('--propsteps') ?? '6000'], { PROPS_BREAK: BREAK ? '1' : undefined });
  const rows = r.out.split('\n').filter(l => / (PASS|FAIL) — \d+ things/.test(l)), sum = r.out.split('\n').find(l => /^prop-ident: /.test(l)) ?? '';
  red.props = Number(/, (\d+) differ/.exec(sum)?.[1] ?? 0) > 0;   // (the furniture's own answers: steps whose columns, men, noises or boxes differed)
  const fs = await import('node:fs'), rs = fs.readFileSync(resolve(ROOT, 'rust/sim/src/props.rs'), 'utf8'), home = fs.readFileSync(resolve(ROOT, 'src/sim/propshome.ts'), 'utf8'), ts = fs.readFileSync(resolve(ROOT, 'src/game/props.ts'), 'utf8');
  const there = (name: string): number => { const m = new RegExp(`(?:pub )?const ${name}: \\w+ = ([^;]+);`).exec(rs); return m ? Number(eval(m[1].replace(/_/g, ''))) : NaN; };
  const snake = (k: string) => k.replace(/[A-Z]/g, c => '_' + c).toUpperCase(), off: string[] = []; let shared = 0;
  const table = (name: string, prefix: string) => { const body = new RegExp(`export const ${name} = \\{([^}]*)\\}`).exec(home)?.[1] ?? ''; const ps = [...body.matchAll(/(\w+):\s*(-?\d+)/g)]; if (!ps.length) off.push(`${name} not found`);
    for (const q of ps) { shared++; const t = there(prefix + snake(q[1])); if (t !== Number(q[2])) off.push(`${name}.${q[1]} ${q[2]} here, ${t} there`); } };
  { // the columns: propshome.ts's two lists against the fields of props.rs's struct, in its order (propIo() holds every offset to the module's own at run time)
    const list = (k: string) => (new RegExp(`export const ${k} = \\[([^\\]]*)\\]`).exec(home)?.[1] ?? '').replace(/['\s]/g, '').split(',').filter(Boolean);
    const cols = [...list('PROP_F64'), ...list('PROP_U32')].map(k => snake(k).toLowerCase()), body = /pub struct Props \{([^}]*)\}/.exec(rs)?.[1] ?? '';
    const fields = [...body.matchAll(/pub (\w+): \[(f64|u32|i32); CAP\]/g)].map(m => m[1]); shared += cols.length;
    if (cols.length < 15 || cols.join() !== fields.join()) off.push(`the columns ${cols.join()} here, ${fields.join()} there`); }
  table('PT', 'F_'); table('PD', 'D_'); table('PK', 'K_'); table('PY', 'Y_'); table('PE', 'E_'); table('PP', 'P_'); table('PF', 'F_');
  for (const [k, t] of [['PROP_CAP', 'CAP'], ['PROP_EV', 'NEV'], ['PROP_PARS', 'NPAR']]) { shared++; const h = Number(new RegExp(`export const ${k} = (\\d+)`).exec(home)?.[1]); if (h !== there(t)) off.push(`${k} ${h} here, ${t} ${there(t)} there`); }
  { const sounds = /PROP_SOUNDS = \[([^\]]*)\]/.exec(home)?.[1].replace(/['\s]/g, '').split(',') ?? []; ['SCRAPE', 'THUD', 'ROLL'].forEach(k => { shared++; if (sounds[there(k)] !== k.toLowerCase()) off.push(`sound ${k}`); }); }
  const here = (name: string): number => Number(new RegExp(`\\b${name} = (-?[\\d.]+)`).exec(ts)?.[1] ?? NaN);
  for (const k of ['MAX_V', 'MAX_W', 'SLEEP_V', 'SLEEP_W', 'SLEEP_T', 'DOOR_GUIDE', 'Y_MIN', 'NEAR', 'SETTLE_PASSES']) { shared++; if (here(k) !== there(k)) off.push(`${k} ${here(k)} in props.ts, ${there(k)} in props.rs`); }
  const ok = r.status === 0 && rows.length >= 4 && rows.every(l => / PASS /.test(l)) && /=== prop-ident: PASS/.test(r.out) && off.length === 0 && shared > 70;
  verdict(ok, 'props: every thing the Rust module steps ends where the TypeScript\'s step leaves it — its columns, the clock, the men, the noises, its boxes — and the two sides name their numbers alike',
    (rows.length ? rows.map(l => l.replace(/\s+/g, ' ').trim()).join('\n    ') + '\n    ' + sum.slice(0, 600) : `the differential left nothing (exit ${r.status}): ${(r.err || r.out).trim().split('\n').slice(-2).join(' / ').slice(0, 300)}`)
    + `\n    ${shared} numbers shared with the Rust source (propshome.ts, props.ts)${off.length ? ` — NOT THE SAME: ${off.slice(0, 8).join(', ')}` : ', each the same'}`);
}

// ---- slabs
{
  const r = child(['bun', 'run', 'tools/qa/slab-ident.ts', '--n', flag('--slabsteps') ?? '4000'], { SLABS_BREAK: BREAK ? '1' : undefined });
  const rows = r.out.split('\n').filter(l => / (PASS|FAIL) — \d+ leaves/.test(l)), sum = r.out.split('\n').find(l => /^slab-ident: /.test(l)) ?? '';
  red.slabs = Number(/, (\d+) differ/.exec(sum)?.[1] ?? 0) > 0;   // (the slabs' own answers: steps whose rows, boxes, noises or tallies differed)
  const fs = await import('node:fs'), rs = fs.readFileSync(resolve(ROOT, 'rust/sim/src/slabs.rs'), 'utf8'), home = fs.readFileSync(resolve(ROOT, 'src/sim/slabshome.ts'), 'utf8'), ts = fs.readFileSync(resolve(ROOT, 'src/game/doorblast.ts'), 'utf8');
  const there = (name: string): number => { const m = new RegExp(`(?:pub )?const ${name}: \\w+ = ([^;]+);`).exec(rs); return m ? Number(eval(m[1].replace(/_/g, '').replace(/core::f64::consts::PI/g, 'Math.PI'))) : NaN; };
  const snake = (k: string) => k.replace(/[A-Z]/g, c => '_' + c).toUpperCase(), off: string[] = []; let shared = 0;
  const table = (name: string, prefix: string) => { const body = new RegExp(`export const ${name} = \\{([^}]*)\\}`).exec(home)?.[1] ?? ''; const ps = [...body.matchAll(/(\w+):\s*(-?\d+)/g)]; if (!ps.length) off.push(`${name} not found`);
    for (const q of ps) { shared++; const t = there(prefix + snake(q[1])); if (t !== Number(q[2])) off.push(`${name}.${q[1]} ${q[2]} here, ${t} there`); } };
  { // the columns: slabshome.ts's two lists against the fields of slabs.rs's struct, in its order (slabIo() holds every offset to the module's own at run time)
    const list = (k: string) => (new RegExp(`export const ${k} = \\[([^\\]]*)\\]`).exec(home)?.[1] ?? '').replace(/['\s]/g, '').split(',').filter(Boolean);
    const cols = [...list('SLAB_F64'), ...list('SLAB_U32')].map(k => snake(k).toLowerCase()), body = /pub struct Slabs \{([^}]*)\}/.exec(rs)?.[1] ?? '';
    const fields = [...body.matchAll(/pub (\w+): \[(f64|u32|i32); CAP\]/g)].map(m => m[1]); shared += cols.length;
    if (cols.length < 21 || cols.join() !== fields.join()) off.push(`the columns ${cols.join()} here, ${fields.join()} there`); }
  table('SLF', 'F_'); table('SP', 'P_');
  for (const [k, t] of [['SLAB_CAP', 'CAP'], ['SLAB_PARS', 'NPAR'], ['SLAB_STEPPED', 'STEPPED'], ['SLAB_HOST', 'HOST']]) { shared++; const h = Number(new RegExp(`\\b${k} = (\\d+)`).exec(home)?.[1]); if (h !== there(t)) off.push(`${k} ${h} here, ${t} ${there(t)} there`); }
  const here = (name: string): number => Number(new RegExp(`\\b${name} = (-?[\\d.]+)`).exec(ts)?.[1] ?? NaN);
  { shared += 3; if (here('LIFT') !== there('LIFT')) off.push(`LIFT ${here('LIFT')} in doorblast.ts, ${there('LIFT')} in slabs.rs`);
    if (!/G = 9\.8\b/.test(ts) || there('G') !== 9.8) off.push('the fall, G'); if (!/const OV = new Int32Array\((\d+)\)/.test(ts) || Number(/const OV = new Int32Array\((\d+)\)/.exec(ts)![1]) !== there('OV')) off.push('the gather\'s list, OV'); }
  { // THE NUMBERS WRITTEN IN THE RULE ITSELF (a rung of a ladder, a reach, a margin: no table names them): function by function, the numbers in the
    // TypeScript's body are the numbers in the Rust's. Small whole numbers are left out (an index here is a field there); a constant the Rust names
    // outside the function and the TypeScript writes inside it is counted where it is used.
    const strip = (t: string) => t.replace(/\/\*[\s\S]*?\*\//g, '').replace(/\/\/.*$/gm, '');
    const nums = (t: string) => [...strip(t).matchAll(/(?<![\w.])\d+(?:\.\d+)?(?:e-?\d+)?(?![\w.])/g)].map(m => Number(m[0])).filter(v => !Number.isInteger(v) || v > 3).sort((a, b) => a - b);
    const body = (src: string, head: RegExp) => { const m = head.exec(src); if (!m) return null; const end = src.indexOf('\n}\n', m.index); return end < 0 ? null : src.slice(m.index, end); };
    for (const [here_, there_] of [['stepSlab', 'step'], ['pushOut', 'push_out'], ['unstick', 'unstick'], ['settle', 'settle'], ['overlaps', 'overlaps'], ['freeSpot', 'free_spot'], ['reachAlong', 'reach_along']]) {
      const a = body(ts, new RegExp(`^function ${here_}\\(`, 'm')), b = body(rs, new RegExp(`^(?:pub )?fn ${there_}\\(`, 'm'));
      if (a === null || b === null) { off.push(`${here_} | ${there_}: not found`); continue; }
      const na = nums(a), nb = nums(b); if (/\bG\b/.test(strip(b)) && !/const G\b/.test(b)) { nb.push(there('G')); nb.sort((x, y) => x - y); }
      shared += na.length; if (na.join() !== nb.join()) off.push(`the numbers written in ${here_}: ${na.join(' ')} here, ${nb.join(' ')} in ${there_}`);
    } }
  // …and the step is what the TypeScript's was before a slab's numbers were columns (tools/qa/slab-asitwas.ts: the words it folded then), on both stands,
  // by the module (the rows as the build has them) and by the TypeScript (this check's own arm: every row held down)
  const wasOn = (stand: string, hold?: string) => child(['bun', 'run', 'tools/qa/slab-asitwas.ts'], { TPS_PROFILE: stand, QA_HOLD: hold });
  const wases = [wasOn('shipped'), wasOn('frozen'), wasOn('shipped', 'rows=0'), wasOn('frozen', 'rows=0')];
  const was = { status: wases.every(w => w.status === 0) ? 0 : 1, out: wases.map(w => w.out).join('\n') }, wasRows = was.out.split('\n').filter(l => / (PASS|FAIL) — \d+ leaves, /.test(l));
  const ok = r.status === 0 && rows.length >= 4 && rows.every(l => / PASS /.test(l)) && /=== slab-ident: PASS/.test(r.out) && off.length === 0 && shared > 50 && was.status === 0 && wasRows.length >= 16;
  verdict(ok, 'slabs: every loose slab the Rust module steps ends where the TypeScript\'s step leaves it — its row, its box, the slam, the tallies — and the two sides name their numbers alike',
    (rows.length ? rows.map(l => l.replace(/\s+/g, ' ').trim()).join('\n    ') + '\n    ' + sum.slice(0, 600) : `the differential left nothing (exit ${r.status}): ${(r.err || r.out).trim().split('\n').slice(-2).join(' / ').slice(0, 300)}`)
    + `\n    ${shared} numbers shared with the Rust source (slabshome.ts, doorblast.ts)${off.length ? ` — NOT THE SAME: ${off.slice(0, 8).join(', ')}` : ', each the same'}`
    + `\n    as it was (both stands by the module and by the TypeScript, against the words folded before the columns): ${was.status === 0 ? `${wasRows.length} runs the same` : (wasRows.filter(l => / FAIL /.test(l)).map(l => l.replace(/\s+/g, ' ').trim()).join(' / ') || `exit ${was.status}`).slice(0, 400)}`);
}

// ---- items
{
  const r = child(['bun', 'run', 'tools/qa/item-ident.ts', '--n', flag('--itemsteps') ?? '4000'], { ITEMS_BREAK: BREAK ? '1' : undefined });
  const rows = r.out.split('\n').filter(l => / (PASS|FAIL) — \d+ things of furniture/.test(l)), sum = r.out.split('\n').find(l => /^item-ident: /.test(l)) ?? '';
  red.items = Number(/, (\d+) differ/.exec(sum)?.[1] ?? 0) > 0;   // (the pool's own answers: steps whose rows, events or furniture differed)
  if (BREAK) {   // …and the snow's own red path: the module told another snow than the level's must part from the game's rule (ITEMS_BREAK=snow, the tick left alone)
    const s = child(['bun', 'run', 'tools/qa/item-ident.ts', '--n', '1500', '--levels', 'tut-road.json:snow,tut-quiet.json:snow'], { ITEMS_BREAK: 'snow', RUST_BREAK: undefined });
    red.items = red.items && Number(/, (\d+) differ/.exec(s.out.split('\n').find(l => /^item-ident: /.test(l)) ?? '')?.[1] ?? 0) > 0 && s.status === 1;
  }
  const fs = await import('node:fs'), rs = fs.readFileSync(resolve(ROOT, 'rust/sim/src/items.rs'), 'utf8'), home = fs.readFileSync(resolve(ROOT, 'src/sim/itemshome.ts'), 'utf8'), ts = fs.readFileSync(resolve(ROOT, 'src/game/throwables.ts'), 'utf8');
  const lines = fs.readFileSync(resolve(ROOT, 'rust/sim/src/lines.rs'), 'utf8'), props = fs.readFileSync(resolve(ROOT, 'src/game/props.ts'), 'utf8');
  const there = (name: string): number => { const m = new RegExp(`(?:pub )?const ${name}: \\w+ = ([^;]+);`).exec(rs); return m ? Number(eval(m[1].replace(/_/g, '').replace(/core::f64::consts::PI/g, 'Math.PI'))) : NaN; };
  const snake = (k: string) => k.replace(/[A-Z]/g, c => '_' + c).toUpperCase(), off: string[] = []; let shared = 0;
  const table = (name: string, prefix: string) => { const body = new RegExp(`export const ${name} = \\{([^}]*)\\}`).exec(home)?.[1] ?? ''; const ps = [...body.matchAll(/(\w+):\s*(-?\d+)/g)]; if (!ps.length) off.push(`${name} not found`);
    for (const q of ps) { shared++; const t = there(prefix + snake(q[1])); if (t !== Number(q[2])) off.push(`${name}.${q[1]} ${q[2]} here, ${t} there`); } };
  const list = (k: string) => (new RegExp(`export const ${k} = \\[([^\\]]*)\\]`).exec(home)?.[1] ?? '').replace(/['\s]/g, '').split(',').filter(Boolean);
  { // the columns: itemshome.ts's two lists against the fields of items.rs's struct, in its order (itemIo() holds every offset to the module's own at run time)
    const cols = [...list('ITEM_F64'), ...list('ITEM_U32')].map(k => snake(k).toLowerCase()), body = /pub struct Items \{([^}]*)\}/.exec(rs)?.[1] ?? '';
    const fields = [...body.matchAll(/pub (\w+): \[(f64|u32|i32); CAP\]/g)].map(m => m[1]); shared += cols.length;
    if (cols.length < 26 || cols.join() !== fields.join()) off.push(`the columns ${cols.join()} here, ${fields.join()} there`); }
  table('ITF', 'F_'); table('IP', 'P_'); table('IE', 'E_'); table('IEK', 'EV_');
  for (const [k, t] of [['ITEM_ROWS', 'CAP'], ['ITEM_PARS', 'NPAR'], ['ITEM_EV', 'EV'], ['ITEM_ROW_EVENTS', 'ROW_EVENTS'], ['ITEMS_DONE', 'DONE'], ['ITEMS_HOST', 'HOST']]) { shared++; const h = Number(new RegExp(`\\b${k} = (\\d+)`).exec(home)?.[1]); if (h !== there(t)) off.push(`${k} ${h} here, ${t} ${there(t)} there`); }
  { const kinds = list('ITEM_KINDS'); for (const [k, t] of [['smoke', 'K_SMOKE'], ['frag', 'K_FRAG'], ['rifle', 'K_RIFLE']]) { shared++; if (kinds.indexOf(k) !== there(t)) off.push(`the kind ${k} is ${kinds.indexOf(k)} in ITEM_KINDS, ${t} ${there(t)} there`); } }
  { // lying snow: what snow a box is (snowBog.ts SNOW_KIND against world.rs SNOW_*) and the roofs a walk is handed
    const world = fs.readFileSync(resolve(ROOT, 'rust/sim/src/world.rs'), 'utf8'), bog = fs.readFileSync(resolve(ROOT, 'src/game/snowBog.ts'), 'utf8');
    const kinds = [.../export const SNOW_KIND = \{([^}]*)\}/.exec(bog)?.[1].matchAll(/(\w+):\s*(\d+)/g) ?? []]; if (kinds.length !== 5) off.push('SNOW_KIND not found in snowBog.ts');
    for (const q of kinds) { if (q[1] === 'none') continue; shared++; const t = Number(new RegExp(`pub const SNOW_${q[1].toUpperCase()}: u8 = (\\d+);`).exec(world)?.[1]); if (t !== Number(q[2])) off.push(`SNOW_KIND.${q[1]} ${q[2]} in snowBog.ts, ${t} in world.rs`); }
    shared++; const roofs = Number(/\bITEM_ROOFS = (\d+)/.exec(home)?.[1]); if (roofs !== there('MAX_ROOFS')) off.push(`ITEM_ROOFS ${roofs} here, MAX_ROOFS ${there('MAX_ROOFS')} there`);
    // (…and the weather's own: the roofs it keeps in force are the roofs a walk is handed — one number in three places)
    shared++; const wx = Number(/export const MAX_ROOFS = (\d+)/.exec(fs.readFileSync(resolve(ROOT, 'src/game/weatherKinds.ts'), 'utf8'))?.[1]); if (wx !== roofs) off.push(`MAX_ROOFS ${wx} in weatherKinds.ts, ITEM_ROOFS ${roofs} in itemshome.ts`); }
  { shared += 2; const g = Number(/export const GRAVITY = ([\d.]+)/.exec(ts)?.[1]); if (g !== there('GRAVITY')) off.push(`GRAVITY ${g} in throwables.ts, ${there('GRAVITY')} in items.rs`);
    if (!/const sub = 2;/.test(ts) || there('SUB') !== 2) off.push('the substeps, sub'); }
  { // THE NUMBERS WRITTEN IN THE RULE ITSELF, function by function (as the slabs' are held): the step, the pose's key, and the furniture's ray
    const strip = (t: string) => t.replace(/\/\*[\s\S]*?\*\//g, '').replace(/\/\/.*$/gm, '');
    const nums = (t: string) => [...strip(t).matchAll(/(?<![\w.])\d+(?:\.\d+)?(?:e-?\d+)?(?![\w.])/g)].map(m => Number(m[0])).filter(v => !Number.isInteger(v) || v > 3).sort((a, b) => a - b);
    const body = (src: string, head: RegExp, end: string) => { const m = head.exec(src); if (!m) return null; const e = src.indexOf(end, m.index); return e < 0 ? null : src.slice(m.index, e); };
    for (const [what, a, b] of [
      ['stepRow | update', body(ts, /^  private stepRow\(/m, '\n  }\n'), body(rs, /^pub fn update\(/m, '\n}\n')],
      ['poseKey | pose_key', body(ts, /^const poseKey = /m, '\n'), body(rs, /^fn pose_key\(/m, '\n}\n')],
      ['PropSystem.raycastHere | ray_things', body(props, /^  raycastHere\(ro: Vec3/m, '\n  }\n'), body(lines, /^pub fn ray_things\(/m, '\n}\n')]] as [string, string | null, string | null][]) {
      if (a === null || b === null) { off.push(`${what}: not found`); continue; }
      const na = nums(a), nb = nums(b); shared += na.length; if (na.join() !== nb.join()) off.push(`the numbers written in ${what}: ${na.join(' ')} here, ${nb.join(' ')} there`);
    } }
  const ok = r.status === 0 && rows.length >= 4 && rows.every(l => / PASS /.test(l)) && /=== item-ident: PASS/.test(r.out) && off.length === 0 && shared > 60;
  verdict(ok, 'items: every thrown thing the Rust module steps ends where the TypeScript\'s step leaves it — its row, what it said and did, the furniture it struck — the furniture\'s ray is one ray both ways, and the two sides name their numbers alike',
    (rows.length ? rows.map(l => l.replace(/\s+/g, ' ').trim()).join('\n    ') + '\n    ' + sum.slice(0, 700) : `the differential left nothing (exit ${r.status}): ${(r.err || r.out).trim().split('\n').slice(-2).join(' / ').slice(0, 300)}`)
    + `\n    ${shared} numbers shared with the Rust source (itemshome.ts, throwables.ts, props.ts)${off.length ? ` — NOT THE SAME: ${off.slice(0, 8).join(', ')}` : ', each the same'}`);
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
  const run = (profile: string | undefined) => child(['bun', 'run', 'tools/qa/simstate-check.ts'], { TPS_PROFILE: profile, QA_HOLD: undefined, SIMSTATE_BREAK: BREAK ? '1' : undefined });
  // (both stands play on the game's own maths, so on both the rows whose step is their own maths — sim.ts OWN_MATHS — step in the module: `stepped`, of
  //  each such family, the steps taken in the module, must not be none on either)
  const fr = run(undefined), sh = run('shipped'), rows = (o: string) => o.split('\n').filter(l => /^- (PASS|FAIL) — /.test(l)), bad = [...rows(fr.out), ...rows(sh.out)].filter(l => /^- FAIL/.test(l));
  const steppedOf = (o: string) => [...o.matchAll(/ (\d+) of the (\d+) steps taken in the module/g)].map(m => Number(m[1])), stepped = [...steppedOf(fr.out), ...steppedOf(sh.out)];
  const ok = fr.status === 0 && sh.status === 0 && rows(fr.out).length >= 8 && rows(sh.out).length >= 8 && steppedOf(fr.out).length >= 3 && steppedOf(sh.out).length >= 3 && stepped.every(n => n > 0);
  red.dice = [fr, sh].every(c => rows(c.out).some(l => /^- FAIL — dice:/.test(l)));   // (the die's own line, on both stands: the other lines fail for the block)
  verdict(ok, 'state, dice: a game\'s block of the module\'s memory is the same state in any slot, the snapshot carries it, and the die at home in it is the game\'s die (both stands)',
    ok ? `${rows(fr.out).length} lines on the frozen stand, ${rows(sh.out).length} on the shipped one (steps of the things taken in the module: ${stepped.join(', ')}), every one PASS (tools/qa/simstate-check.ts)` : bad.length ? bad.map(l => l.slice(2, 110)).join('\n    ') : stepped.length < 6 || stepped.some(n => n === 0) ? `a family's step never ran in the module (${stepped.join(', ') || 'no line says'})` : `the check left nothing (exits ${fr.status} ${sh.status}): ${(fr.err || sh.err).trim().split('\n').slice(-2).join(' / ').slice(0, 300)}`);
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

// ---- server
{
  const TICKS = 600, setup = { search: '?roomclear&squad4', seed: 7 };
  const made = (tables: object | null) => child(['bun', 'run', 'tools/mp/setup.ts', JSON.stringify(tables ? { ...setup, tables } : setup)], { QA_HOLD: undefined });
  const stood = (recipe: unknown, hold?: string) => recipe ? marked(child(['bun', 'run', 'tools/qa/rust-check.ts', '--server', JSON.stringify(recipe), String(TICKS)], { QA_HOLD: hold }).out, 'RUST-SERVER ') : null;
  // (the rows are no knob table: a setup cannot ask for one, and a recipe carries none. The game a server stands is the build's; what it is held to is
  //  that same recipe stood with every row held down, and with the maths' row held up too: this check's own arms, in the child that stands it)
  const mDef = made(null), rDef = marked(mDef.out, 'MPSETUP '), asked = made({ RUST: Object.fromEntries(S.rustRows().map((r: string) => [r, false])) }), refused = asked.status !== 0 && marked(asked.out, 'MPSETUP ') === null && /RUST is not a knob table of this build/.test(asked.err + asked.out);
  const def = stood(rDef), off = stood(rDef, 'rows=0'), on = stood(rDef, 'maths=1');
  const table = (r: any) => r?.tables?.RUST ?? r?.knobs?.tables?.RUST ?? r?.tables?.EXACT ?? r?.knobs?.tables?.EXACT, T = (r: string) => S.rustOn(r) ? TICKS : 0;
  const ok = !!on && !!off && refused && on.ticks === TICKS && off.ticks === TICKS && on.rust === TICKS && off.rust === 0 && on.lines === TICKS && off.lines === 0 && on.circles === TICKS && off.circles === 0 && on.fog === TICKS && off.fog === 0
    && on.exact && off.exact && on.word === off.word && SENSES.every(k => off.sensed[k] === 0) && on.sensed.sight > 0 && on.sensed.light > 0 && on.sensed.sound > 0 && on.sensed.paths > 0;
  verdict(ok, 'server: the game server\'s stand ticks on the module\'s maths (that row held up), lines, circles and fog, and is the game it is with every row held down; a setup that asks for a row is refused', !on || !off ? `a stand left nothing (recipe: ${!!rDef} · stands: ${!!on} ${!!off})` :
    `a setup with a RUST table: ${refused ? 'refused by name' : `NOT REFUSED (exit ${asked.status})`} · ${on.rust} of ${on.ticks} ticks on the module's maths, ${on.lines} on its lines, ${on.circles} on its circles and ${on.fog} with its sight pass (every row held down: ${off.rust}, ${off.lines}, ${off.circles} and ${off.fog} of ${off.ticks}) · the senses asked of it: ${SENSES.map(k => `${k} ${on.sensed[k]}`).join(', ')} (without: ${SENSES.map(k => off.sensed[k]).join(', ')}) · word ${on.word}${on.word === off.word ? ', the same' : ` ≠ ${off.word}`}`);
  // (the recipe as a server plays it today, with no arm held: the build's rows, RUST_ON, and no other. A section of its own, so that its red path is its
  //  own: the box world's break must move ITS word, with no maths row to do it)
  const okDef = !!def && !!off && !!rDef && table(rDef) === undefined && def.ticks === TICKS && def.rust === T('maths') && def.lines === T('lines') && def.circles === T('circles') && def.fog === T('fog') && def.sees > 100 && def.exact && def.word === off.word;
  verdict(okDef, 'server as the page: a recipe carries no row and no maths, and the game stood from it plays on the build\'s rows (RUST_ON) and is the game with every row held down', !def || !off ? `a stand left nothing (recipe: ${!!rDef} · stands: ${!!def} ${!!off})` :
    `the recipe's tables name ${table(rDef) === undefined ? 'no row and no maths' : 'A ROW OR THE MATHS'} · of ${def.ticks} ticks ${def.rust} on the module's maths, ${def.lines} on its lines, ${def.circles} on its circles, ${def.fog} with its sight pass (${def.sees} men asked of it) · word ${def.word}${def.word === off.word ? ', the word with every row held down' : ` ≠ ${off.word}`}`);
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
  interface Props { there: number; here: number; noises: number; asked: number; differ: number; first: string; pairs: number; walls: number }
  interface Slabs { there: number; here: number; host: number; asked: number; differ: number; first: string; landed: number }
  interface Items { there: number; here: number; host: number; walks: number; asked: number; differ: number; first: string; rested: number; events: number; thingRays: number }
  interface Run { falls: Falls; doors: Doors; props: Props; slabs: Slabs; items: Items; rays: number; run: string; ticks: number; cone: number; end: string; digest: string; bits: string; rust: number; lines: number; circles: number; fog: number; sees: number; sensed: Sensed; poses: number; posesIn: number; posed: number; sets: number; layers: number; dice: number; draws: number; both: { asked: number; differ: number; stale: number; first: string }; exact: boolean; engine: string; secs: number }
  const one = (run: string, rustOn: boolean, both = false): Promise<Run | string> => new Promise(done => {
    const [name, how] = run.split('@');   // (`@frag`: the run played with 'frag and clear' ordered at a held room's door — tools/qa/playthrough.ts PT_FRAG — so that blasts take leaves off their hinges)
    const cmd = node ? [...NODE, 'tools/qa/rust-check.ts', '--run', name] : ['bun', 'run', 'tools/qa/rust-check.ts', '--run', name];
    const env: Record<string, string | undefined> = { ...process.env, TPS_PROFILE: 'shipped', TPS_CPUSENSE: '1', PT_DIGEST: '1', RUST_BOTH: both ? '1' : '0', ...(node ? { V8_ENTRY: resolve(ROOT, 'tools/qa/rust-check.ts'), FC_NOSTANDIN: '1' } : {}) };
    env.QA_HOLD = all.map(r => `${r}=${rustOn && up.includes(r) ? 1 : 0}`).join(',');   // (this check's own arm, said to the child: the rows of this run's side, each by name — tools/qa/headless.ts hold)
    if (how === 'frag') env.PT_FRAG = '1';
    if (up.includes('dice')) env.TPS_SEED = name.split(':')[1];   // (the die's row moves the game's OWN die: both of a pair are stood up with one, off the run's seed)
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
  const fair = (ts: Run, rs: Run) => (/:hard(@|$)/.test(rs.run) ? rs.cone === rs.ticks && ts.cone === ts.ticks : rs.cone === 0 && ts.cone === 0) && (!up.includes('maths') || rs.rust === rs.ticks) && (!up.includes('lines') || rs.lines === rs.ticks) && (!up.includes('rays') || rs.rays > 100) && ts.rays === 0 && (!up.includes('circles') || rs.circles === rs.ticks) && (!up.includes('fog') || (rs.fog === rs.ticks && rs.sees > 100)) && ts.rust === 0 && ts.lines === 0 && ts.circles === 0 && (!up.includes('poses') || (rs.poses > rs.ticks && rs.posed === rs.ticks && rs.sets > 1 && rs.posesIn > rs.poses * 0.9)) && (up.includes('walk') ? rs.layers > LAYER_FLOOR * rs.ticks : rs.layers === 0) && ts.layers === 0 && ts.posed === 0 && ts.posesIn === 0 && (!up.includes('dice') || (rs.dice === rs.ticks && rs.draws > 100 && rs.draws === ts.draws)) && ts.dice === 0 && ts.fog === 0 && ts.sees === 0 && ts.poses === 0 && ts.ticks > 600 && ts.falls.there === 0 && ts.doors.there === 0 && ts.props.there === 0 && (!up.includes('props') || (rs.props.there >= 0.95 * (rs.props.there + rs.props.here) && (rs.props.there + rs.props.here === 0 || rs.props.there > 0.9 * rs.ticks))) && (!up.includes('doors') || (rs.doors.there > 0.95 * (rs.doors.there + rs.doors.here) && rs.doors.there > 0.9 * rs.ticks && rs.doors.turned > 100)) && SENSES.every(k => ts.sensed[k] === 0 && (!up.includes(k) || rs.sensed[k] > SENSE_FLOOR[k])) && (!up.includes('slabs') || rs.slabs.there >= 0.9 * (rs.slabs.there + rs.slabs.here - rs.slabs.host)) && ts.slabs.there === 0 && (!up.includes('items') || rs.items.there >= 0.9 * (rs.items.there + rs.items.here - rs.items.host)) && ts.items.there === 0;
  const sensedSay = (r: Run) => SENSES.map(k => `${k} ${r.sensed[k]}`).join(', ');
  if (args.includes('--missions')) {
    let ok = true; const lines: string[] = []; let ticks = 0; const raised: (Run | string)[] = [];
    for (const run of runs) {
      const [ts, rs] = await Promise.all([one(run, false), one(run, true)]); raised.push(rs);
      if (typeof ts === 'string' || typeof rs === 'string') { ok = false; lines.push(`${run}: ${typeof ts === 'string' ? `TypeScript ${ts}` : ''} ${typeof rs === 'string' ? `Rust ${rs}` : ''}`); continue; }
      const same = ts.bits === rs.bits && ts.ticks === rs.ticks && ts.end === rs.end && ts.digest === rs.digest, honest = fair(ts, rs);
      if (!same || !honest) ok = false; ticks += ts.ticks;
      lines.push(`${run}: ${ts.ticks} ticks${rs.cone ? ` (${rs.cone} of the Rust run's with every eye a cone)` : ''}, ends ${ts.end} · ${same ? `to the bit (${ts.bits})` : `DIFFERS — TypeScript ${ts.ticks} ticks ${ts.bits} ${ts.digest}, Rust ${rs.ticks} ticks ${rs.bits} ${rs.digest}`}${honest ? '' : ` · NOT A FAIR PAIR (of ${rs.ticks} ticks in the Rust run: ${rs.rust} on its maths, ${rs.lines} with its lines (${rs.rays} rays answered there), ${rs.circles} with its circles, ${rs.fog} with its sight pass (${rs.sees} men asked), ${rs.poses} poses asked (${rs.posesIn} of them where they live, ${rs.sets} men's poses in the block for ${rs.posed} ticks), ${rs.dice} with its die at home (${rs.draws} draws, ${ts.draws} in the other), ${rs.doors.there} door steps in the module and ${rs.doors.here} not (${rs.doors.turned} leaf turns), ${rs.props.there} steps of the furniture in the module and ${rs.props.here} not, ${rs.layers} layers of the animator in the module (${ts.layers} in the other), the senses ${sensedSay(rs)}; in the other: ${ts.rust}, ${ts.lines} and ${ts.circles} of ${ts.ticks}, the senses ${sensedSay(ts)})`}${up.includes('rays') ? ` · rays the module answered: ${rs.rays}` : ''} · senses asked of the module: ${sensedSay(rs)}${up.includes('doors') ? ` · door steps in the module: ${rs.doors.there} (${rs.doors.turned} leaf turns, ${rs.doors.noises} noises)` : ''}${up.includes('props') ? ` · steps of the furniture in the module: ${rs.props.there} (${rs.props.noises} noises, ${rs.props.pairs} knocks between things, ${rs.props.walls} against the statics)` : ''}${up.includes('slabs') ? ` · steps of a loose slab in the module: ${rs.slabs.there} (${rs.slabs.landed} slammed flat, ${rs.slabs.host} handed back for a pane; ${ts.slabs.here} in the other)` : ''}${up.includes('items') ? ` · rows of thrown things the module walked: ${rs.items.there} (${rs.items.events} things said or done after, ${rs.items.rested} come to rest, ${rs.items.host} handed back; ${ts.items.here} in the other)` : ''}${up.includes('falls') ? ` · rules of the falls in the module: ${rs.falls.there} (${rs.falls.steps} footfalls asked, ${rs.falls.crunches} on glass)` : ''} · ${ts.secs} s / ${rs.secs} s`);
      console.error(`  ${lines[lines.length - 1]}`);
    }
    { const t = thin(raised); if (t.length) { ok = false; lines.push(`NOT A FAIR SET: the runs together asked the module too little of ${t.join(', ')}`); } }
    if (up.includes('props') && !raised.some(r => typeof r !== 'string' && r.props.there > 0 && r.props.walls > 0 && r.props.noises > 0)) { ok = false; lines.push('NOT A FAIR SET: in no run did the module step furniture that moved (a thing against a static, a noise): a set with none proves nothing of the props row'); }
    if (up.includes('items') && !raised.some(r => typeof r !== 'string' && r.items.there > 1000 && r.items.rested > 0 && r.items.events > 0)) { ok = false; lines.push('NOT A FAIR SET: in no run did the module walk a pool of thrown things to a bounce and a rest: a set with none proves nothing of the items row'); }
    if (up.includes('slabs') && !raised.some(r => typeof r !== 'string' && r.slabs.there > 0 && r.slabs.landed > 0)) { ok = false; lines.push('NOT A FAIR SET: in no run did the module step a leaf off its hinges down to its slam (a run `@frag` has them): a set with none proves nothing of the slabs row'); }
    verdict(ok, `missions: ${runs.length} runs played with the rows ${up.join(' + ')} in Rust and with none are the same game after every update (${node ? 'node' : 'bun'})`, `${ticks} ticks compared\n    ${lines.join('\n    ')}`);
  }
  if (args.includes('--both')) {
    let ok = true; const lines: string[] = []; let asked = 0;
    const got = await Promise.all(runs.map(run => one(run, true, true)));
    runs.forEach((run, i) => { const r = got[i];
      if (typeof r === 'string') { ok = false; lines.push(`${run}: ${r}`); return; }
      const world = up.some(k => k === 'lines' || k === 'circles' || k === 'fog' || (SENSES as readonly string[]).includes(k));   // (a row that asks the box world asks it some hundred thousand times a run; the doors' step is one question a tick)
      const b = r.both, good = b.asked > (world ? 1e5 : up.includes('doors') || up.includes('props') ? 1000 : -1) && b.differ === 0 && b.stale === 0 && r.falls.differ === 0 && r.slabs.differ === 0 && r.items.differ === 0 && (!up.includes('props') || (r.props.differ === 0 && r.props.here <= 0.1 * r.ticks && (r.props.asked === 0 || r.props.asked > 0.9 * r.ticks))) && (!up.includes('doors') || (r.doors.both > 0.9 * r.ticks && r.doors.differ === 0 && r.doors.turned > 100)) && (!up.includes('lines') || (r.lines === r.ticks && r.rays > 1000)) && (!up.includes('circles') || r.circles === r.ticks) && (!up.includes('fog') || (r.fog === r.ticks && r.sees > 100)) && SENSES.every(k => !up.includes(k) || r.sensed[k] > SENSE_FLOOR[k]); if (!good) ok = false; asked += b.asked;
      lines.push(`${run}: ${r.ticks} ticks (${r.lines} with the module asked), ${b.asked} questions answered both ways (${r.sees} of them the fog's; ${r.rays} rays; the senses ${sensedSay(r)}; ${r.doors.both} the doors' step, ${r.doors.turned} leaf turns; ${r.props.asked} the furniture's step; ${r.slabs.asked} a slab's; ${r.items.asked} walks of the thrown things; ${r.falls.asked} rules of the falls${r.falls.first ? ` — ${r.falls.first}` : ''}) · ${b.differ} answers differ · ${b.stale} stale rows${b.first ? ` · first: ${b.first}` : ''} · ${r.secs} s`); });
    { const t = thin(got); if (t.length) { ok = false; lines.push(`NOT A FAIR SET: the runs together asked the module too little of ${t.join(', ')}`); } }
    if (up.includes('props') && !got.some(r => typeof r !== 'string' && r.props.asked > 0)) { ok = false; lines.push('NOT A FAIR SET: no run has furniture for the module to step'); }
    if (up.includes('items') && !got.some(r => typeof r !== 'string' && r.items.asked > 100)) { ok = false; lines.push('NOT A FAIR SET: no run has thrown things for the module to walk'); }
    if (up.includes('slabs') && !got.some(r => typeof r !== 'string' && r.slabs.asked > 0)) { ok = false; lines.push('NOT A FAIR SET: no run has a leaf off its hinges for the module to step (a run `@frag` has them)'); }
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
