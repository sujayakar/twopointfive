# The simulation in Rust

The maintainer's ask (2026-11-02): "we should just write this in rust so we have good control over everything (and aren't subject to the vagaries of each JS
engine). and then we can also use wasm simd for accelerating things (e.g. ECS layout)", and earlier that a Rust simulation "might even be desirable for us
running this elsewhere". The game's update (`Game.update`, about 100,000 lines over `src/game`) is moving into Rust, built to WebAssembly, a system at a
time. What is built so far is the ground: the crate, its build, its one way into the page, the server and the tools, the switch table, the checks, and the
first system across (the game's own maths). No rule of play has changed and nothing a player sees is different.

- **Where it lives.** `rust/` is a Cargo workspace (`rust-toolchain.toml` names the compiler and the WebAssembly target; no crate from outside, so no network
  at build time). `rust/sim` is the simulation: `src/lib.rs` (what every line owes the game, and the C exports, named `<module>_<thing>`), `src/maths.rs`,
  `src/pow10.rs` (the decimal powers as the language reads them, written by `bun run tools/sim/pow10.ts`: the Rust side never parses a decimal). `rust/prim`
  is three functions (√, floor, trunc): the simulation is `no_std` and the core library lacks them. `rust/dress` is the other side of the line (below,
  gameplay and dressing): its own module, `src/dress/built.ts` and `src/dress/dress.ts`.
  `src/sim/built.ts` is the module's bytes as text, WRITTEN BY `bun run tools/sim/build.ts` and checked in. `src/sim/sim.ts` is the one TypeScript file that
  loads the module (`sim()`, through `src/sim/load.ts`), names its exports (`SimExports`, `SIM_EXPORTS`, `SIM_ABI`), hands out its memory (`f64()`, `u8()`) and holds the switch table
  `RUST`. Like `src/math`, `src/sim` is below the game: it imports `built.ts` and `src/math` and nothing else.
- **Nobody needs Rust to play, or to write TypeScript.** The page, the game server (`tools/mp/`), the hosted build and every headless tool load the module
  from `built.ts` as they load any source file. The price: `built.ts` must follow `rust/`. AFTER ANY CHANGE UNDER `rust/`: `bun run tools/sim/build.ts`, and
  commit `built.ts` with it. `bun run tools/sim/build.ts --check` (the `built` lines of `tools/qa/rust-check.ts`) is red when the hash of the crate's sources
  in `built.ts` is not their hash now (no compiler needed), when the module is over 6 MB, when it carries a path of the machine that built it, when the
  powers' table is not what its tool writes, and, where
  there is a compiler, when the crate built again is not the checked-in bytes (another builder than the file's `SIM_BUILDER`, its compiler and kind of
  machine, is told, not failed: nobody has built this on a Mac yet, so whether the bytes are the same there is not known). The build passes cargo its own
  flags and its own output folder, whatever the shell's `RUSTFLAGS` or `CARGO_TARGET_DIR` say; `.gitattributes` keeps `rust/` in LF, since the hash is of bytes.
- **One way in, and it is synchronous.** `sim()` compiles and starts the module in one step on first use (2 to 3 ms headless, 1.5 to 11 ms over five boots in
  Chromium for the 32 KB of today), so nothing that calls it is written around a promise and there is no second path for the page. Chrome allows that on the page's own
  thread up to 8 MB (measured: 6 MB compiles, 9 MB is refused with a RangeError); the build is red at 6 MB. If the whole update ever outgrows that, the
  answer is a boot step that waits, decided then. The page starts the module at boot and says so once on the console (`rust simulation: …`, `simSay`).
- **A browser with no WebAssembly.** A browser whose policy turns its JIT off has no `WebAssembly` at all. `simWhyNot()` is the one answer to "can this
  runtime run the module" (no WebAssembly, a compile the page's policy refuses, a `built.ts` of another version: asked once, by starting it); where it
  cannot, the page says NOT RUNNING on its console and plays on the TypeScript simulation, and a row of `RUST` asked for there stays off (`rust-page.cjs nojit`). THE DAY THE UPDATE IS IN RUST ALONE
  SUCH A PAGE CANNOT PLAY. That is a cost of the goal the maintainer should know before the TypeScript update is taken out.
- **`RUST`: a row a system** (`src/sim/sim.ts`). On = that system of the update runs in the module; off = in TypeScript as it always did. THE TWO ARE THE SAME
  GAME TO THE BIT, which is what lets a system move at all. So a row is never a rule of play: it has no `FROZEN_HOLDS` line (both stands leave every row
  off), a recording made with one on replays the same with it off, and the journal carries the table so that a recording says which ran (a file from
  before the table replays with every row off, as it was made: `journal.ts applyKnobs`). It is in the knob registry, so a recipe carries it and a server and its clients run the same rows. A row
  goes ON in the page once its system is proven and is faster there; until then a check raises it. Rows are setters: writing one moves the system.
  - `maths` — the game's own maths (`src/math/exact.ts`, when `EXACT.on` has put them in `Math`'s place) from the module: `exact.ts mathsFrom` takes the
    same fourteen functions and the three lengths of `math/hyp.ts` from elsewhere. `?rustmaths` in the page, `TPS_RUSTMATHS=1` headless. OFF EVERYWHERE, and
    meant to stay off: from TypeScript the crossing costs more than the dozen operations it saves (bun 1.4: sin 27 ns in TypeScript, 44 ns across; exp 34 and
    83; a length 17 and 22. node 22: level, within 2 ns either way except sin, 46 and 55). The row exists as the proof that the two arithmetics are one,
    in a whole game and not only on a stream of inputs; the Rust systems that follow call the Rust maths with no crossing.
  - `lines` — the box world's line tests (`collision.ts segmentBlocked`, `segmentBlockedDynamic`: every sight line, round and reach) from `rust/sim/src/lines.rs`,
    over the module's copy of the boxes (next bullet), inside a tick. `?rustlines`, `TPS_RUSTLINES=1`. The length is worked out in TypeScript and handed
    over, so the row answers under either maths. Glass: the module imports nothing, so where the host has a question about a pane (`glassOpen`, a line of
    fire counting panes) the walk LISTS the glass it crosses and TypeScript judges the list. OFF EVERYWHERE for now. Measured 2026-11-03 on a 4-core cloud
    box under bun: a sight line 1.81 → 1.08 µs on the office and 1.03 → 0.77 µs on the compound (`tools/qa/segment-bench.ts`). THE TICK, with `lines`,
    `circles` and `fog` up together against none (`tools/qa/tickcost.ts --each --list --rounds 2`, two runs, one seed each, the same bits in every table;
    the rows-off table taken before and after): 4.81 → 4.59 ms mean over 13,676 ticks, and 4.81 again off. pass-four 5.28 / 5.13 → 4.68 ms (its ticks
    over 8 ms 594 / 363 → 260); compound-four 4.48 / 4.58 → 4.52 ms, which is nothing: that machine's two rows-off tables differ by as much. So about
    5 % of a tick, where the profile gave collision and the lines 12 %: the rest of that share is what calls the lines, not the walk.
  - `circles` — a circle against the boxes (`collideCircle`, `circleTouches`, `gather`: a body pushed out of the statics, what a prop or a rag doll rests
    against) from `collide.rs`, the same copy. `?rustcircles`, `TPS_RUSTCIRCLES=1`. No gain of its own was measured (the number above is the three
    rows together). OFF EVERYWHERE.
  - `fog` — the fog of war's sight pass (`fogofwar.ts update`: which of the team's eyes has a line to each man and each body) from `fog.rs`: the eyes are
    handed over once a pass (`collision.ts sightOpen`, `sightEye`) and a man is then ONE question (`eyesSee`) where he was two lines an eye. `?rustfog`,
    `TPS_RUSTFOG=1`. It works its lengths out in the module, so IT ANSWERS ONLY WHERE THE GAME'S OWN MATHS STAND (`EXACT.on`: the page, the server, the
    shipped stand); on the frozen stand, whose lengths are the engine's, the TypeScript answers whatever the row says. That is the rule for every system to
    come that takes a length or an angle. What the fog draws with (`things`, the picture's blocks) is not in the row, and that is where the fog's
    time is: the sight pass was its lines, so the row has no gain of its own. OFF EVERYWHERE.
  - `poses` — the posing functions of `src/anim/skeleton.ts` (`blendPose`, `PoseFK.run` / `runFrom` / `parentRot`, `rotateLocal`, `rotateModel`,
    `setModelRot`, `twoBoneIK`) from `skeleton.rs`. `?rustposes`, `TPS_RUSTPOSES=1`. The animator's layers (`game/character.ts`, 6,300 lines) are still
    TypeScript and so are its poses, so a question's pose is written onto a BENCH in the module, asked, and read back. THAT IS A PROOF, NOT A SPEED-UP: it
    shows the two are one arithmetic in a whole game (about 350 questions a tick), and the copies cost more than the Rust saves: the same two runs
    with `poses` up as well, 4.59 → 4.85 ms a tick. Only where the game's own maths
    stand. A pose is stored in singles as the Float32Array stores it; `rust/sim/src/single.rs` is the one file that may name a single (the build's rule
    names it as its exception), and nothing is worked out in singles. Not ported yet: `sampleClip` (the clips have to live in the module first) and
    `SkeletonInstance.update` (the twists, the rag doll's overrides, the matrices). OFF EVERYWHERE.
  - `dice` — the game's own die (`math/dice.ts SimDice`: the mulberry32 word every draw of the simulation comes off, and the count of draws) kept in the
    game's block of the module's memory and rolled there (`rust/sim/src/dice.rs`; `src/sim/dicehome.ts` is its TypeScript half, so `math/dice.ts` stays a
    leaf that names no module: `simRandom()` → the die's `home.roll()` → `dice_roll`). `?rustdice`, `TPS_RUSTDICE=1`. THE FIRST
    STATE OF THE GAME THAT LIVES IN THE MODULE (the bullet "State in the module is state of the game" is what it was built to prove). A PROOF, NOT A
    SPEED-UP, like `maths`: a draw from TypeScript then costs a call; but the die has to be in the module before any rule that rolls it is, and it puts
    real state through every road a snapshot takes. Either maths (it works out no length). It moves only a game that HAS a die of its own (the page
    always; the harness with `TPS_SEED`). `g.dice.s` / `.n` read and write whichever place holds them, so a checkpoint file is the same file with the
    row on or off. OFF EVERYWHERE.
  - THE SENSES, five rows, each a WHOLE QUESTION of the game's asked once where the TypeScript asked its lines one by one (`src/scene/collision.ts`, the
    block headed "THE SENSES IN THE MODULE"; `senseSet(bit)` is the one gate: inside a tick, the copy standing, the row on, `EXACT.on`, and the
    instance's line tests the class's own: a check that puts its own `segmentBlocked` over one instance gets ITS lines, from the TypeScript). Each works its
    lengths and angles out in the module, so each answers only where the game's own maths stand. Each returns "ask the TypeScript" for what it cannot
    judge (a length that is no length of this world's, a line that must ask the host about an open pane), and the caller's own statements, kept whole as
    `…Here`, then answer. What they write to is SCRATCH of one question (the lamps, the blobs, the nav grid's shut cells, a way's waypoints): nothing of the
    game is kept in the module by any of them. The numbers the host shares with these files (doubles a lamp, the answers' codes) are `boxMirror.ts FACE`,
    under the Rust's own names; `rust-check` reads the Rust sources and is red where one differs.
    - `sight` — an eye on a man (`game/guards.ts eyeContact`: the cone in plan, the lens's elevation, then the line to his chest and, behind a blocked
      chest, to his head) from `sight.rs`. `?rustsight`, `TPS_RUSTSIGHT=1`. Only for an eye whose line passes glass (every eye's does today).
    - `light` — the lamps on a point (`guards.ts lightEstimateAt`) from `light.rs`. The host keeps the gates that are about WHOSE lamp it is (enabled, the
      owner skipped, steady or not), in their order, writes a row a lamp that passed and whose reach can hold the point, and asks once; the hour's sun and
      sky and a stroke of lightning are added to the same six faces in TypeScript afterwards, as they were (`hours.ts hourLight` keeps its own memory of
      rays, which is state of the host's). `?rustlight`, `TPS_RUSTLIGHT=1`.
    - `smoke` — the game's own smoke along a line (`game/sense.ts CpuSmoke.trans`, what `SENSE.cpu` sees through; now asked as `CpuSmoke.along(col, a, b)`)
      from `smoke.rs`: the live blobs are written for the one question and the sum, the erf and the wall test of each blob are the module's.
      `?rustsmoke`, `TPS_RUSTSMOKE=1`. Idle where the senses are the renderer's (nothing asks it).
    - `sound` — how a sound reaches a listener (`collision.ts SoundPaths.propagate`) from `hear.rs`, in two questions with the TypeScript's half-second
      memory of routed answers between them: the straight lines (clear, grazed, over the top, walled off, too far), then the way round by the floor (the
      search of `nav.rs`, each leg measured, the last waypoint the listener has a straight run to, a shut leaf across it). Sound stops at glass, so where
      glass breaking is on (`glassOpen`) a line that meets a pane and nothing solid is the host's to judge and the whole question goes back to TypeScript.
      `?rustsound`, `TPS_RUSTSOUND=1`.
    - `paths` — a way across the floor (`collision.ts NavGrid.findPath` with NO penalty: the A* over the half-metre grid, `nearestFree`, the greedy
      smoothing and `walkable`) from `nav.rs`. The grid of shut cells stays the host's (it bakes and re-bakes it) and is written to the module for each
      question, as it stands. The scores are KEPT in singles as the TypeScript keeps them in Float32Arrays (`single.rs`), and the search's tables stand
      blank between searches on both sides, so a search the module ran leaves the TypeScript's tables as it found them. A search under a penalty
      (`tactics.ts`) is TypeScript's. `?rustpaths`, `TPS_RUSTPATHS=1`.
    OFF EVERYWHERE, all five. Measured 2026-11-03 on a 4-core cloud box under bun; never seen on a GPU, and in a browser only by the page check below.
    A QUESTION (`sense-ident --bench`, TypeScript → the module, µs): on the office sight 1.87 → 1.51, light 16.9 → 12.9, smoke 4.44 → 3.41, sound 164 →
    188, paths 108 → 121; on the compound sight 2.07 → 1.34, light 13.5 → 11.4, smoke 4.94 → 4.02, sound 82 → 92, paths 46 → 54. So `sound` and `paths`
    are SLOWER in the module (the search is a scan of the open list for its least score, which bun's compiler does as fast, and the grid is handed over
    each question): they are there because the server's model is to be one model, not for speed, and stay off as proofs, like `maths`, until what calls
    them moves. THE TICK (`tickcost.ts --each --list --rounds 2`, five missions, one seed each, 38,634 ticks, the same bits in every table): rows off
    3.53 and 3.43 ms (two tables; another job ran beside the first), the five senses up alone 3.41 ms, which is nothing, and with `lines`, `circles` and
    `fog` up too 3.29 ms, 4 % under the quiet table. A mission asks the module some 400 thousand eyes, 170 thousand lights, up to 60 thousand lines of
    smoke, 1,500 sounds and 350 ways. The five missions are the same game to the bit with all ten rows up and with none, under bun and under node;
    `--both` answered 16.6 million questions twice with none differing; on the page (`rust-page.cjs systems`, headless Chromium, frames not drawn) a
    mission's first 240 frames with these rows raised are the game in TypeScript by digest and by every frame's bits (the smoke row is idle there: the
    page's senses are the renderer's). Nobody has played with one of these rows up.
- **Posing: what is left, and it is most of it.** Posing is gameplay, whole (the bullet "Gameplay and dressing"), and about a sixth of the tick. The time is not in the
  functions above but in the layers that call them, and in the arrays they allocate on the way. Those layers move to Rust only WITH THEIR DATA: the
  poses and the run-forward bones in the module's memory, so a layer written in Rust calls `skeleton.rs` with no crossing and no copy. That is the
  first BIG system whose state lives in the module (the bullet "State in the module is state of the game": the snapshot carries a game's block since
  the die moved, so the pose store is a part of `State` and nothing in `game/rollback.ts` changes for it). The poses are then reached through
  `game.sim`, never kept as a `Float32Array` over the module's memory on a man. Then the layers, one at a time, each held by
  `tools/qa/pose-ident.ts` and the missions as the functions are now.
- **The box world's copy** (`src/scene/boxMirror.ts`, `rust/sim/src/world.rs`). The rules that move a box (doors, blasts, cuts, glass, props) are still
  TypeScript, so a `Box` object is the truth and the module holds a COPY: sixteen doubles a box, its flags, its cut planes, the grid's cells. It is not state
  of the game: nothing is snapshotted, and a copy thrown away is made again from the objects. What makes it safe is that it is PROVEN, not trusted:
  - the module is asked only INSIDE A TICK (`Game.stepSeats` → `col.tickOpen()` / `tickShut()`). As a tick opens every static is compared value for value
    with its row and rewritten where it differs, so whatever happened between ticks (a level edit, a load, a snapshot put back: `PUT_BACKS` forces all of
    it) costs nothing to get right. Outside a tick (the page's draw, a tool) the TypeScript answers.
  - A RULE THAT WRITES A STATIC INSIDE A TICK SAYS SO: `col.wrote()` (its shape, flags or cut; a pane a walker now passes). The next question then compares
    the statics again first. `addStatic` / `truncateStatics` say it themselves (the grid changed: TypeScript answers until the next tick). Leaves: the
    three places that write a leaf say so themselves (`boxMirror.ts leafWrote`, called by `Door.place`, and so `Door.reset`, and by `doorblast.ts
    poseSlab`), whoever calls them and when; a list a question swaps in for itself is copied at every question.
    Today's writers of statics: `destruction.ts`, `furnblast.ts`, `wallbreak.ts`, `winbars.ts`, `glassbreak.ts`. THIS IS THE ONE PART THAT RESTS ON EVERY
    WRITER REMEMBERING, and it ends when those rules are Rust and write the rows themselves. Until then `rust-check --both` is the net: it found the three
    writers nobody had listed (door leaves, a flag weather writes, the breakers' hides).
  - the copy never outlives what it was proven against: the tick shuts it however the step ends (a `finally` in `Game.stepSeats`), and a snapshot put back
    shuts every game's copy there and then (`rollback.ts restoreSnapshot` → `mirrorsShut`: a checkpoint loads inside the step's last lines).
  - the copy keeps the flags a question reads (`MIRRORED`: NoShadow, Dynamic, and bits 16 up); a question asked with any other flag goes to TypeScript, as
    does a level too big for the tables (8,192 statics, 256 leaves a list, 16,384 cells). Four games' worlds are held at once; a fifth takes the one stepped
    longest ago.
  Checks: `tools/qa/segment-ident.ts --rust` (the `lines, circles` line of `rust-check`: 40,000 segments and 10,000 circles a level over four levels with
  leaves swung, walls broken and re-shaped, against the TypeScript walk as it is and as it first was; `RUST_BREAK=1` leaves a re-shape unsaid and is red);
  `rust-check --missions` (rows on against off, raw bits after every tick) and `--both` (every question the module is sent is answered by TypeScript too,
  and every row of the copy is compared with its box at every question: `collision.ts BOTH`).
  Run 2026-11-03, five squad missions, one seed each, every row up: 38,634 ticks the same raw bits after every update under bun and under node;
  `--both` 14,283,676 questions (385,530 of them the fog's), none different, no stale row; `RUST_BREAK=1` red on both; `tools/qa/dress-check.ts` green
  with the rows up; `tools/qa/rust-page.cjs` (Chromium, the software adapter, frames not drawn): 240 frames of a squad mission with lines, circles, fog
  and poses up are the TypeScript game by digest and raw bits. NOBODY HAS SEEN any of it on a GPU or played it: the rows are off wherever a person
  plays, and the frozen stand runs `lines` and `circles` only (the other two need the game's own maths). Probe families with the rows up: destruction,
  furnblast, breaks, winbars, wallbreak and wallcharge were run both ways before the last review's mends (3.3 million questions, none different); the
  door, glass and frag families were not.
- **What Rust code owes the game, and who holds it** (the head of `rust/sim/src/lib.rs`). One arithmetic: doubles, and only + − × ÷ √ and what the standard makes exact of them;
  never `f64::sin` and friends (`maths.rs`), never `mul_add`, never an `f32` where the TypeScript holds a double, no fast-math flag. One order: nothing whose
  order is not written down (no `HashMap` / `HashSet` iteration, no pointer as a key, no threads). No world outside: no clock, no random source of its own
  (the game's die is handed in), no files. A port is STATEMENT FOR STATEMENT: `(a - b) - c` is not `a - (b + c)`, and a term that looks like it cancels is
  there because the other side has it. Speed comes after the bits match, and SIMD after that, each held by the same checks.
  THE COMPILER holds the world outside and the order: `rust/sim` is `#![no_std]`, so `std::collections`, a thread, a clock, a file do not resolve in it.
  THE BUILD'S `rules` LINE (`tools/sim/build.ts RULES`, red in `rust-check`) holds what the compiler cannot once any crate links the standard library: its
  float methods (`.sin()`, `.powf()`, `.mul_add()`, and `.sqrt()` / `.floor()` as methods: the exact ones are `prim`'s functions), an `f32`, relaxed SIMD,
  `extern crate std`. The order of the arithmetic is the reader's and the differential's.
- **What holds a moved system** (`tools/qa/rust-check.ts`; `RUST_BREAK=1` must fail it). `built` and `face` (the module exports what `sim.ts` names, at its
  version). `maths`: one fixed stream (`tools/qa/rust-fold.js`, 14.2 million answers over the fourteen functions, the corners a port gets wrong among them)
  through the TypeScript functions and the Rust ones (`sim.ts mathsOfRust()`, the very set the row installs), every single answer the same bits. `engines`: the Rust functions fold that stream to one word under bun
  and under node, the TypeScript functions' word. `switch`, `stand`. `server`: the game server's own stand (`tools/mp/stand.ts`) takes the row from its recipe, ticks 600 times with the
  module's maths in `Math`'s place, and ends on the word of the same setup without the row (a whole server and its clients on the row:
  `tools/qa/mpserver-check.ts --setup '{"scenario":"?roomclear&squad4","seed":7,"knobs":{"RUST":{"maths":true}}}'`). `--missions`: five scripted squad
  runs on the server's stand (shipped, the senses on the CPU), each played twice, a row on and off, and the chained raw bits of the state after EVERY update
  (`game/digest.ts rawBits`) are the same; `--node` plays them under V8. Measured 2026-11-02: 38,634 ticks, every one the same bits, and the five end words
  are the same under bun and under node. `tools/qa/rust-page.cjs` is the page's half in Chromium: `loads` (the stream in the page, Chrome's V8 against the module, and the module's
  word against bun's), `rows` (a mission's first 240 frames with `?rustmaths` are the game without it, by digest and by every frame's bits), `nojit`.
  `senses` (`tools/qa/sense-ident.ts`): random questions over whole levels, doors swung, a share of the statics re-shaped half way and said, a dozen lamps
  of every kind put in, two dozen blobs; each asked of the module and of the TypeScript (the same function with the copy's rows put down for the one
  call), every number of every answer the same bits, and red if under half of a sense's questions went to the module, or if a level did not stand up.
  The floor's grid is baked again over the re-shaped boxes. Under `RUST_BREAK=1` the re-shape goes unsaid and the new grid is not handed over: it must
  fail, and EVERY sense must differ somewhere (exit 3 names one that did not). In `--missions` a run is a fair pair only if each raised sense answered
  there (`collision.ts SENSED` counts them: some hundred thousand eyes and lights a run, some thousand sounds, some hundred ways; the smoke has no
  floor a run, since not every run throws a canister, and a floor over the runs together), and `--both` answers every one of them twice. The game
  server's 600 ticks ask no smoke: the missions hold that row.
  A NEW SYSTEM GETS THE SAME THREE: a differential of its own on recorded inputs (`tools/cpu-frame/raycast-diff.ts` is that for the collision world), the
  missions with its row on and off, and the page.
  - **Every row has a section, and a red path that reaches it** (`rust-check.ts HELD`, `RED_SECTIONS`; 2026-11-03). `HELD` names, for each row of
    `RUST`, the section of the quick half that asks that system's own answers of both sides; `rows` is red on a row with no line there, or whose
    section did not run. Each differential ends on a line of its counts BY ROW (`SEGMENT-IDENT`, `SENSE-IDENT`, `POSE-IDENT`; the die's own line of
    `simstate-check`). Under `RUST_BREAK=1` every section that asks the module must be red AND every row's own answers must have differed: the run
    ends `the red path is red for every section that ran … and for every row's own answers …` and exit 1, or `THE RED PATH IS NOT RED FOR …` and
    EXIT 3. A break word that does not reach a row leaves that row's check proving nothing, and a red neighbour hides it. Three rows were like that
    until this was counted: THE CIRCLES (`segment-ident --rust` asked them after the copy was made good again: on the red path 0 of 40,000 circle
    answers differed; the panes walked through now go unsaid too, 11,430 differ beside 842 of the lines'), THE DIE (`SIMSTATE_BREAK=1` broke the
    block, not the die: its own line stayed PASS; a bit of its word is now turned there) and THE FOG (next).
  - **The fog's pass has a differential of its own, cones and all** (`tools/qa/sense-ident.ts`, the sixth kind of question). Every eye of the
    comparison missions sees all round (`FOG.fovDeg` 360; the cone is Hard's, the lens the wand's), so the wedge and the near field of `fog.rs`
    were asked by NO check, and the red path this row had (every eye handed over as seeing all round) was no break at all: with that row alone
    the five missions were the same game to the bit and `--both` found 0 of 18,830 answers differing. Now: one to seven eyes with wedges and
    near fields of their own or the team's, handed over by the game's own `fogofwar.ts handEyes` (what `update()` calls), on men, men down and
    bodies, tried in an order until so many see; 8,000 questions over four levels, each the module's answer against `seesHere`, none differing.
    Tried on it, by hand, and put back: an eye's cone handed over a tenth narrow, 73 of 4,000 answers differ; its near field 0.3 m long, 20; the
    red path's unsaid re-shape, 127 of 8,000.
  - **What the red path reaches INSIDE A MISSION, a row at a time** (measured 2026-11-03: pass-four and block-squads, `RUST_BREAK=1`, that row
    alone against the TypeScript): the game differs for `maths` (the engine's cosine), `fog` (one answer in seven turned) and `dice` (a bit of
    the word turned). For `lines`, `circles`, `poses`, `sight`, `light`, `smoke`, `sound` and `paths` it is THE SAME GAME TO THE BIT: their break
    is the copy not told what was written, a mission's copy is made good as every tick opens, and no answer of those two runs fell between a
    write and the next tick; the poses have no break in a mission at all. So `RUST_BREAK=1 --missions` is red by the maths, and `--missions
    --rows lines` under the break word now says so itself (`THE RED PATH IS NOT RED FOR the section missions`, exit 3). What holds those eight
    rows on the red path is their differential, by row. NOT DONE: a break that turns each row's own answers where they come back from the
    module (as the fog's does), so that a mission under the break word is red for every row raised alone.
  - **What `missions` holds of the fog, measured** (that row alone, the turned answer above): the bits differ on four of the five runs (not
    hostage-rescue), so a man shown who should not be reaches the state; with only "nobody sees" one call in seven, hostage-rescue and
    compound-four were still the same to the bit (the next pass sees him). The bits hold some of a system's mistakes; the answers asked both ways
    hold every one. A NEW ROW'S FINAL SAYS BOTH: its section's count by row on the red path, and whether `RUST_BREAK=1 --missions --rows <row>`
    differs.
  - **What ran for this** (2026-11-03, a cloud box, bun 1.4, no GPU, nothing drawn): the quick half (every section PASS but this box's own three:
    it has no linker for the crate, so `bytes` twice, and its node child does not answer, so `engines`); `RUST_BREAK=1` exit 1 with every
    section and row red; the same with the circles' question put back as it was, exit 3 naming the circles; `--missions` (five runs, every row,
    26,583 ticks to the bit); `--both` on two runs (2,662,497 answers, none differing, no stale row); `RUST_BREAK=1 --missions --rows fog` red
    on four runs; `typecheck` and `typecheck:qa`. Not run: node, the page, the whole `--both`. `fogofwar.ts` changed by one extracted function
    (`handEyes`), no statement of it moved.
- **State in the module is state of the game** (`rust/sim/src/state.rs`, `src/sim/state.ts`, `game/rollback.ts`). What a game keeps in the module from
  one call to the next is ONE BLOCK of bytes a game, and nothing else in the module is state:
  - **Rust.** `state.rs State` is a `#[repr(C)]` struct: a head (the layout's word, the homes) and then A FIELD A FAMILY of the update, each a struct
    of flat fixed-size columns, with its row in `PARTS` (where, how long, a version). Four rules: NO ADDRESS in a block, only indices (it is the same
    bytes in any slot, in the host's memory, in a file, on another machine); NO PADDING the compiler chose (each part asserts its size is the sum of its
    fields); ZEROS are a game with nothing in it; no allocation (`GAMES` = 4 blocks, the memory never grows). `tools/sim/build.ts STATICS` names
    every `static mut` of the crate as STATE (the blocks), SCRATCH (one call's) or COPY (the box world) and is red on one with no row, and on a
    static written through a cell.
  - **TypeScript.** A Game has one `SimState` (`game.sim`). Its block is BOUND to one of the module's slots or PARKED in a buffer of its own when a
    fifth game needs the slot (the one bound longest ago gives way; nothing is lost). `st.w` / `st.d` are the block and nothing else, wherever it
    stands: a read or a write from TypeScript is this game's and costs no call; a call into the module passes `st.bind()`. NO OBJECT OF THE GAME
    KEEPS A TYPED ARRAY OVER THE MODULE'S MEMORY (never `st.w` in a field): a snapshot put into another game would clone the buffer under it, the
    whole memory. The snapshot counts any it meets (`Snapshot.simViews`, carried on by every take since), `restoreSnapshot` THROWS on such a snapshot
    (in place, its bytes would land on whichever game holds the slot now) and `rollback-check` is red on one. A slot that is not one traps in the module
    (`state.rs state`): it never answers for another game's block.
  - **At home, or not.** While a system has a TypeScript twin its state is in the block only with its row on. Which, for THIS game, is a bit in the
    block's head (state like the rest), and as a tick opens every system whose row and bit disagree is moved (`SimState.settle`, called by
    `Game.stepSeats`, inside the tick's own `try`; a system registers `simTenant({ row, bit, has, into, outOf })`, and `has` says whether THIS game has any
    such state: a game with no die of its own is never given a block for one). So a row may be raised or dropped under a running game, a snapshot
    holds where things were, and a game with no system at home has no block at all (an empty take, nothing in a slot).
  - **The snapshot** knows a `SimState` by its make (`K_SIM`): taken as the block in 4 KB pieces, a piece unchanged since the last take shared;
    put back into the block where it stands; put into ANOTHER game's own block; folded into `stateHash` and laid out by `stateDiff` a part at a time
    (`sim.dice[2]: … | …`). The saves kept in memory are snapshots, so they carry it with no line of their own.
  - **A checkpoint FILE and the raw bits do not carry bytes of the module, on purpose.** A row is never a rule of play: a file written with a row
    on must load with it off (a browser with no WebAssembly plays the TypeScript), and `rawBits` must give one word either way. So both go on
    reading the game's VALUES, and a system at home in the block answers for its values there (the die: `SimDice.s` / `.n`). The day a family has
    no TypeScript twin, its part of the block is what a file holds; the layout's word (`state_layout`) is there for that.
  - **A NEW FAMILY, step by step** (the die is the worked example: `rust/sim/src/dice.rs`, `src/sim/dicehome.ts`). It writes no rollback code.
    1. RUST: a file of its own with a `#[repr(C)]` struct of flat columns (numbers and fixed arrays of numbers; an index where another language
       would hold a pointer; a `spare` field wherever the compiler would pad, and a `const` assert that its size is the sum of its fields).
    2. A field of `State` for it in `state.rs`, after the others, and its row in `PARTS` (`offset_of!`, `size_of`, a version to raise whenever the
       columns change). No other `static mut`: scratch of one call is the only other kind, and it needs its row in `tools/sim/build.ts STATICS`.
    3. Its functions take the slot (`state(g)`) and are exported from `lib.rs`; `SIM_ABI` goes up, and the name joins `SIM_EXPORTS` in `src/sim/sim.ts`.
    4. TYPESCRIPT: its name in `SIM_PARTS` and a `P_<NAME>` in `src/sim/state.ts`, in the order of `PARTS`.
    5. A row of `RUST` (off in the module and on both stands; `?rust<name>`, `TPS_RUST<NAME>=1`, the word in `src/host/standEnv.ts STAND_ENV`).
    6. A file in `src/sim/` that registers `simTenant({ row, bit, has, into, outOf })` with a bit nobody has used, imported once by `game.ts`: `into`
       copies the game's values from the TypeScript objects into the block, `outOf` back, to the bit.
    7. Accessors for what TypeScript still reads or writes (`st.w` / `st.d` at `simPart(P_<NAME>)`, never a view kept in a field), so a checkpoint
       file and `rawBits` see the same values with the row on or off. A call into the module passes `st.bind()`.
    8. `bun run tools/sim/build.ts`; then its lines: a differential on its own state in `rust-check`, its `fair()` counts in `--missions --rows <name>`
       (both arms on the same stand), `TPS_RUST<NAME>=1` in front of `rollback-check`, `savetimeline-check` and `seats-check`, a `_BREAK` path, the
       page (`rust-page.cjs`), and what a tick costs with the row off and on. The row's line in `rust-check.ts HELD`, and its own answers differing
       under `RUST_BREAK=1` (exit 3 names a row the red path left green).
    It gets rollback, the ring, a fresh game, two games and the saves in memory for that. Pieces are compared in JavaScript, word by word, at every
    take: fine for megabytes a save; a ring that takes one a tick over a block that changes all over (smoke that decays) will want the compare in
    the module.
  - **The check**: `tools/qa/simstate-check.ts` (the `state, dice` line of `rust-check`, both stands): the layout; the die in a block against
    mulberry32, 4,000 seeds × 250 draws; nine blocks over four slots rolled turn about (2,696 parks, every die its own); take and put, here and
    into another block; a seeded game with the row on against off after every tick, and with the row flipped every 40 ticks under it; a snapshot
    put back (the state's word after each of 60 ticks) and put into a fresh game (the raw bits); snapshots put back across a change of the row; a
    planted view named and its snapshot refused. `SIMSTATE_BREAK=1` (a parked block comes back zeros, a put back writes nothing) fails `slots`,
    `take and put`, `snapshot`, `another game` and `across a row change`; in front of `rollback-check` with `TPS_RUSTDICE=1` it must fail the put-back
    lines there.
  - **What ran (2026-11-03, the cloud box, no GPU).** `rust-check --missions --rows dice` under bun and under node: the five runs, 26,583 ticks, to the
    bit with the die in the module (both arms stood up on `TPS_SEED`, since the row moves only a game's own die: these are other runs than the
    unseeded 38,634 ticks of the other rows); `RUST_BREAK=1` red. `rollback-check` on both stands with `TPS_RUSTDICE=1` (8 scenarios) and without;
    `savetimeline-check`, `dice-check` and `seats-check` (its `twin` line: the look ahead draws no die) with the row up; the page check
    (`tools/qa/rust-page.cjs`, headless Chromium on the software adapter: `systems` raises `?rustdice` on a seeded page, 240 frames to the bit, and
    `nojit` plays the TypeScript). With every row off a tick costs what it did: 3.44 → 3.41 ms over three runs, the same runs to the bit
    (`tickcost --each --list`, the quieter of two pairs). NEVER SEEN ON A GPU, never played; one seed a run.
- **The game's own smoke on a coarse grid** (`src/game/sense.ts SENSE.grid`, `SmokeGrid`; `rust/sim/src/bricks.rs`; the maintainer, 2026-11-02: "rework
  smoke to be on a much simpler simulation on CPU (coarser grid with bricks?) and then have the visuals driven by the current pipeline"). A NEW RULE, not a
  moved one: there is no TypeScript twin and no row of `RUST`. Read only under `SENSE.cpu`. DEFAULT OFF everywhere: the page's `?smokegrid` (with
  `?cpusense`), the harness's `TPS_SMOKEGRID=1` on either stand; off is the blobs, the build as it was. No `FROZEN_HOLDS` row while it is down in the
  plain page too (the `STALK.drag` precedent); the journal carries `SENSE` whole and a file from before the key replays with it off.
  - WHAT IT IS. Density in half-metre cells, held in bricks of 4 × 4 × 8 cells (2 m square, floor to 4 m), 160 bricks a game, stepped ten times a
    second inside the tick (`Game.senseStep`). Fed by the emitters the picture's solver is handed (`CpuSmoke.emit` passes each on: its place, its
    width, what it lets out) and by nothing else of the solver: no velocity, no pressure, no wind, so `smoke.push` and the wind stay dressing. Smoke
    passes between two cells only where nothing solid and no leaf stands between their centres (the box world's lines, asked both ways), so a canister
    fills its room, stops at a shut door and goes out by an open one. It decays at the solver's rate and spreads at the blobs' rate. A sight line's
    transmittance is the density of the cell each quarter metre of it lies in (`grid_trans`, read from the state alone: it answers outside a tick too).
  - THE PICTURE IS NOT DRIVEN BY IT AND DOES NOT DRIVE IT. The GPU solver and its renderer are untouched. How far the two are apart is what
    `?modelerr` measures (the next bullet); the grid's numbers (`SENSE.smoke`: spread, decay, rise) have NOT been tuned against a GPU, and a doorway's
    haze after nine seconds is thinner than the picture's is likely to be.
  - STATE. `SmokeGrid.st` is the module's row of doubles (a head with the plan in bricks, the emitters letting out, which brick is where, the cells), the
    game's own: a snapshot takes it and a put-back writes it. The module's row is made that state before anything is asked of it whenever it is not
    already (`boxMirror.ts gridSync`: another game's, a snapshot put back, the host's own clear, the boxes copied afresh). WHAT THE MODULE KEEPS BESIDE
    IT is derived: where each brick is, which faces between cells are open, and the leaves as the last step found them. The faces are asked once and
    kept, asked again round every leaf that has moved since the last step and wherever the host rewrites a static's row (`statics()` says the
    rectangle; a row past the end of the list as it was is always written), so they are at every step what asking them all afresh gives: a game put
    back from a snapshot and one that never was step alike. A FILE checkpoint keeps no smoke in the air (`savestate.ts`, as ever): a load clears the
    grid with the blobs. A clear of a grid nothing was let out into does nothing at all, and no clear moves the steps' beat (a page's warm-up clears,
    a tape of it played again does not, and the two step alike). OWED: this was written before a game had a block in the module (the bullet "State
    in the module is state of the game"). The grid's row is the game's own array with a proven copy in the module, which is what
    `tools/sim/build.ts STATICS` says of each of its tables; it is to become a part of `State`, and `gridSync` / `gridRead` then go.
  - THE BOOK. The head keeps three sums that thin at the cells' own rate: all that was let out, what thin bricks held when they were given back, and
    what an emitter could not let out because every brick was taken. What the cells hold + given back + refused = let out, at every step, to
    rounding: passing smoke between cells makes none and loses none. A brick is MADE where a cell at a neighbour's edge holds smoke and the way out of
    that cell is open (a wall along a brick's edge costs no brick behind it) and GIVEN BACK when it is thin, no emitter stands in or beside it and no
    neighbour holds smoke at the shared edge: the same test, so a brick is never made and given back in one step. A thin canister's fringe is given
    back as it goes (with no decay a tenth-strength canister has given back 28 % of itself after four minutes, a full one 4 %; with the game's decay
    the smoke is gone long before).
  - THE COPY IS THE GRID'S TO STAND UP (`collision.ts gridStep`): where no row of `RUST` has the box world's copy standing for the tick, the step makes
    it the boxes as they are, steps, and lets it go, with no question of the game's sent to it. Nobody who opens a tick has to know a grid is there.
  - WHERE NO GRID STANDS the blobs answer, as they did: a runtime with no WebAssembly (`simWhyNot()`), a level whose plan is over 8,192 bricks, a tick
    with more leaves than a list holds, and WHILE THERE IS MORE SMOKE THAN BRICKS (the last step wanted a brick there was no slot for, or what was
    refused is still more than nothing: `grid_trans` answers −1, and the grid answers again when it holds everything it was asked to). The blobs are
    still stepped beside the grid (they hold the emitter budget's count), so the fall back is whole.
  - `tools/qa/smokegrid-check.ts` holds it, on both stands: `stands` (and `sense.ts GRID`, the numbers this side reads the state by, against
    `bricks.rs` name for name), `mass` (the book balances to a part in 10⁹: a full canister and a tenth of one, no decay and the game's, at 14 s and
    at four minutes), `full` (sixty canisters: every brick taken, the grid says it cannot answer and the game's smoke along a line is the blobs'; one
    more where no brick is lets nothing out and the book says how much), `door` (a shut door holds, an opened one lets through), `put back` and
    `faces` (the same state word straight, put back from a snapshot, and with the module's copy thrown away before every tick, a door swinging and a
    wall moved and said on the way). `SMOKEGRID_BREAK=1` asks the grid with no leaves and `door` must fail.
  - Measured 2026-11-03 on a cloud box (bun, no GPU), ONE SEED A MISSION, the five missions of `rust-check --missions`. With the switch down every
    mission is the game it was, to the bit. With it up on the frozen stand (`TPS_SMOKEGRID=1`) three of the five are still the same game to the bit
    (the pass, the hostage rescue, the block: no line through smoke changed an answer in them); the compound is another game (taken at 10,870 ticks
    where the blobs' game is wiped at 8,060) and the warehouse another with the same end (taken, 12,611 ticks both). One seed says the rule moves a
    fight where smoke is thrown, not which way it moves fights: a tally over seeds is owed before it goes on by default. A tick costs the same within
    the noise of a run (3.54 ms over the five with the grid, 3.60 without; the three missions of equal bits 3.73 / 3.43 / 3.62 against 3.80 / 3.47 /
    3.53), and the check's own `cost` line reads 1.12 ms a tick while a canister burns against 0.92 quiet. Nobody has played it. Never seen on a GPU,
    and the picture does not read it: the smoke drawn is the solver's, as it was.
- **The light model** (`src/game/sense.ts SENSE.bounce`, `SENSE.glow`; `rust/sim/src/glow.rs`, `light.rs`; `guards.ts lightEstimateAt`, `lightField`,
  `steadyLamps`; the maintainer, 2026-11-02, of the coarse smoke grid: "same goes for lighting too, right?"). A NEW RULE, not a moved one: no TypeScript
  twin, no row of `RUST`. It stands on the coarse grid, so it is read only where `SENSE.cpu` and `SENSE.grid` have one standing. DEFAULT OFF everywhere:
  the page's `?cpulight` (with `?cpusense&smokegrid`), the harness's `TPS_CPULIGHT=1` on either stand; off is the estimate as it was, the build as it
  was. No `FROZEN_HOLDS` row while it is down in the plain page too; the journal carries `SENSE` whole and a file from before the key replays with it off.
  - WHAT IT IS: the two things the estimate of the light on a man never had. THE SMOKE'S SHADOW: a lamp no wall hides is dimmed by the grid's smoke
    along the very line the wall test took: the game's own smoke on that line, one answer (`CpuSmoke.along`: the grid's cells, the blobs while the
    grid holds more smoke than bricks; the module hands such a question back). A grid nothing was let out into is not asked. THE LIGHT THAT COMES ROUND: one
    number A SQUARE METRE OF THE PLAN, in four columns after the grid's cells (the lamps' light at the metre's middle at chest height, the bounce, and
    whether the way to the next metre toward +x and toward +z is open: a clear line between the two middles, asked both ways, leaves and all; a
    pane lets light by, as the lamps' own lines and a reader's do). A thirty-second of the plan is asked again every step of the grid (ten a second),
    so a door that opens or a lamp that moves is in the field within 3.2 s; and THE WHOLE PLAN in the step that finds other lamps than it was last
    swept whole under (`light.rs word`: which lamps and how strong, nothing of where: a new field, a lamp shot out, a breaker thrown), so the light
    that comes round does not outlast its lamp (about 12 ms, once, on the office). Then two passes of one rule over the whole plan: a metre's bounce is `albedo` of what it keeps of its own light and what each open side
    hands it (`share` a side). At rest that is the rule's own fixed point, and no metre holds more than a closed room of the brightest lamp would.
    A POINT reads the nearest metre whose middle it SEES (its own; else across the nearer edge, the other, the corner), so a man against a wall reads
    his own room and never the one behind it; `lightEstimateAt` adds that to the brightest face.
  - WHICH LAMPS. `steadyLamps`: every lamp that is on, has no owner and is not a flash, a beacon or the moon, at its steady strength. A torch a man
    carries, a muzzle flash and lightning light a man directly (the estimate has them) and bounce nothing. A lamp that sweeps (the office has two)
    is followed a sweep behind.
  - WHAT IT LEAVES OUT, named: colour (one number), height (the plan is flat: a mezzanine reads the floor's bounce), the sun's and the sky's bounce,
    bodies' shadows, and any tuning against a GPU: `SENSE.glow` (albedo 0.3, share 0.1) is two numbers picked against ONE software-adapter picture.
    Every level's plan today is within 2,048 square metres; a larger one has no bounce (the smoke's shadow still stands).
  - STATE. The four columns and the lamps' word are part of `SmokeGrid.st` (64 KB), so a snapshot takes them and a put-back writes them with the
    cells; a clear of the smoke keeps them, they are the level's and not the smoke's. A NEW ENCOUNTER TAKES A NEW GRID (`Game.restartEncounter`, and
    whenever the senses come to the game or leave it): its beat and the sweep's turn start with the encounter, so a file loaded into a game that has
    run an hour and into a fresh one go on alike (this holds the smoke grid's beat too, which until now kept an old game's phase). A step with the
    model down zeroes a field that was swept: nothing old is read the day it is raised again. `glow.rs NEXT` is scratch. OWED with the grid's: the
    move into `State`.
  - `tools/qa/cpulight-check.ts` holds it, on both stands: `stands` (up, the plan is swept and the estimate has the bounce in it; a new encounter starts a new
    grid; down, or with no grid, the columns stay zeros and the estimate is the plain one to the bit; `sense.ts GRID` against the Rust sources), `rest` (the rule worked out
    in TypeScript from the columns, and the bound), `rooms` (light comes round by an open way, a metre with no open way holds its own alone, and
    over the whole floor a point reads exactly the metre the rule says, worked out in the check), `lamp` (the lamps a switch holds put out: a
    quarter of a second on no metre keeps their light), `door` (no open way joins the two sides of a shut door; opened, within a sweep, one does),
    `shadow`, `put back` (straight on against put back), `cost`. `CPULIGHT_BREAK=1` sweeps with no leaves and `door` must fail. `rust-check --both` with the switch up asks
    every light question of the module and of `lampsHere` and they agree to the bit (below).
  - Measured 2026-11-03 on a cloud box (bun, no GPU), ONE SEED A MISSION. With the switch down the five missions of `rust-check --missions` are the
    game they were, to the bit. With it up over the grid (`tickcost --each --list`, `TPS_SMOKEGRID=1 TPS_CPULIGHT=1` against `TPS_SMOKEGRID=1`)
    ALL FIVE ARE ANOTHER GAME WITH THE SAME END: the compound taken at 6,733 ticks where the grid alone takes it at 10,870, the warehouse taken at
    13,195 against 12,611, the pass taken at 5,537 against 5,616, the hostage rescued at 3,781 against 3,766, the block wiped at 4,501 against 8,581.
    That is the rule doing what it says (men in a room lit round a corner are seen sooner) and one seed does not say which way it moves a fight: a
    tally over seeds is owed before it goes on by default. A TICK: other fights do not compare tick for tick, and the two tables were made side by
    side on a busy box (the pass 5.17 ms against 5.90, the hostage rescue 5.16 against 4.69: noise both ways); the check's own `cost` line on the
    still office reads 1.01 ms a tick up against 0.89 down, and the whole plan swept at once (a lamp shot out) is 11 to 17 ms, once. BOTH WAYS
    (`TPS_RUSTLIGHT=1 TPS_RUSTSIGHT=1 TPS_RUSTSMOKE=1 TPS_SMOKEGRID=1 TPS_CPULIGHT=1 rust-check --both`, the compound, the hostage rescue and the
    block): 306,585 light questions answered by the module and by `lampsHere`, none differ, no stale row; that run is red on its own fairness floor
    for `smoke` and should be, since with a grid standing a sight line's smoke is the grid's and the blobs' row is asked nothing. Against the one
    kept picture (the software adapter, the office, 24 light points): `lit` is off by 0.025 of the meter at the middle where the plain estimate is
    off by 0.055, 0.107 at the 95th (0.105), leaning 0.027 darker (0.075), 0.77 stops at the 95th (1.23), the other side of the lit bar on the same
    4.2 %. `BUDGET.lit` is that with room, provisional as the others. THE PAGE (headless Chromium on the software adapter, the office,
    `?cpusense&smokegrid&cpulight`): it boots with no page error or warning, five seconds in the field stands on 40 × 28 metres (lamps' light in
    756, bounce in 1,101), Sam's meter reads 0.150 where the same page with the model down reads 0.077, and with a canister at his feet 0 where
    down it reads 0.081. Nobody has played it. Never seen on a GPU.
- **The model against the picture** (`src/host/modelerr.ts`; the page's `?modelerr`, `tools/qa/modelerr.ts`, `tools/qa/modelerr-page.cjs`; the
  maintainer, 2026-11-02: "we'd want some way during development of bounding the error between the two but then just let them proceed independently
  when actually running the game"). A DEVELOPMENT TOOL: nothing of it is made without the word and the game never reads it. The senses' CPU models
  and the picture are two models of one world and neither drives the other; this says how far apart they stand.
  - WHAT IT ASKS. One plan walked over whichever level is open: stations picked from the free floor (the one nearest the start, then each the farthest
    from those picked), at each a set of LIGHT POINTS at chest and head height and, round a canister it sets off there, SIGHT LINES at eye height with
    no wall across them, read at fixed ages of the smoke. Every sample is one question put to both sides: the probe's meter or its transmittance
    (`Game.probe`, asked under the tool's own keys at the last rank, eight places left to the game) and each model's answer (`MODELS`, a row a model:
    `estimate` = `lightEstimateAt`, `blobs`, `grid`; a new model is a row there and a row of `BUDGET`). The models are stepped on a fixed sixtieth
    whatever the page's frame rate, and the tool keeps its own smoke, so the game's is as it was whatever `SENSE` says. The player is stood at each
    station unseen and unhurt with the AI held (nothing moves a door between the page and a headless run) and the three flags are put back at the end.
    Not made on a tape's page or a networked one.
  - WHAT IT REPORTS, a model: the spread of model − picture in the unit the game decides by (light: the meter's 0‥1 scale, `Game.visibilityOf`; smoke:
    the share of light a line lets through): the middle, the 95th percentile, the worst, the lean; for light also stops at the 95th; and HOW OFTEN THE
    TWO DECIDE DIFFERENTLY (lit or dark at `LIGHTS.litAt`; hidden by the plume or not at 0.3). `BUDGET` holds a 95th percentile and a share of
    opposite decisions a model; a model past either is red. It bounds getting WORSE on the recorded scenes; it does not say the numbers are good.
  - THE PAGE: `localhost:5173/?modelerr&seed=1` (`=quick` two stations, `=tiny` one; any level or mission words beside it), press Start, hands off
    for about three minutes: a card shows the table, `window.__modelerr` is the report, and it goes out by the page's door as kind `modelerr`
    (`host/savefile.ts`: to the backend where the page has one, no download).
  - HEADLESS: `bun run tools/qa/modelerr.ts` on both stands walks the plan with the models alone (`walks`: every model answers and a second walk is the
    first to the bit; the grid against the blobs is said), plants pictures and reads them back exactly (`planted`), judges a second walk against the
    first under a tight budget (`budget`; `MODELERR_BREAK=1` makes that walk's smoke three times as thick and it must fail), and judges every report
    under `tools/qa/fixtures/modelerr/` (and `--report f`): as recorded, and, where the stand here asks the very questions the report did (its `world`
    word), WITH THE MODELS AS THEY ARE NOW against the recorded picture. So a picture recorded once on a GPU holds the models to their budget from
    then on with no GPU.
  - Measured 2026-11-03 ON THE SOFTWARE ADAPTER ONLY (a headless browser on a cloud box; never seen on a GPU): the office, plan `tiny` (one station),
    Very low, seed 1, kept as `tools/qa/fixtures/modelerr/office-tiny-software.json`. The picture answered all 24 light points and all 32 sight lines
    (26 with smoke on them). `estimate`: off by 0.055 of the meter at the middle, 0.105 at the 95th, 0.559 at worst, 1.23 stops at the 95th, leaning
    0.075 darker than the picture, the other side of the lit bar on 4.2 %. `blobs`: off by 0.181 of the light let through at the middle, 0.988 at the
    95th, leaning 0.331 thicker, the other side of the hiding bar on 37.5 %. `grid`: 0.038 at the middle, 0.947 at the 95th, leaning 0.206 thicker,
    25.0 %. So in that one room the grid is much nearer the picture than the blobs on most lines, and both are far off on the worst ones (a line
    through the edge of the cloud, where the solver's smoke has climbed and spread and the model's has not). Headless, the full plan on both stands,
    the grid against the blobs over 768 lines: 0.038 apart at the middle, 0.681 at the 95th, deciding differently on 13.0 %. `BUDGET` IS PROVISIONAL:
    its rows are that one report with room to spare, so a model that gets worse is red today; it is to be set again from the first report a GPU makes
    (the plain page with `?modelerr&seed=1`, about three minutes, hands off; the file it saves goes under the fixtures folder). Nobody has run it on a
    GPU.
- **Gameplay and dressing** (the maintainer, 2026-11-02: "let's make sure we're very explicit about what's gameplay affecting (and also run on the server) and
  what's just client side only dressing (debris, smoke details) and uses relaxed simd and/or gpu"). THE RULE: a step of the tick is DRESSING only if the game
  is the same game to the bit without it. Everything else is GAMEPLAY: it runs on the server and on every client, in one arithmetic. Dressing only makes the
  picture; the server need not run it, two machines need not agree on it, and it can read the game and never write it.
  - **The list is a table, `src/game/dress.ts DRESS`**: a row per picture-only step of the tick, and `game.ts` takes the step only with its row up. Every row
    is up in the page and on both stands. It is not a rule of play: no `FROZEN_HOLDS` line, not a knob table, no recording carries it. The rows: `air`
    (airSim), `moths`, `snow` (the lying snow's surface, prints, marks, a blast's dig), `rain` (rainFx), `fires` (fireDraws; the lamp's flicker is a light and
    is gameplay), `wake` (men pushing the solver's smoke), `men` (skin instances, materials, shadow proxies), `cues`, `recap`, `blastFeel`, `fixtures`, `fx`,
    `stature`, `cases`, `net`, `wind`: sixteen. `dressAll(false)` puts them all down: a host that draws nothing may. NOBODY
    DOES YET: the game server still takes every step (measured below; its owner's call).
  - **`tools/qa/dress-check.ts` holds it**: the five comparison missions of `rust-check --missions` (the server's stand: shipped, the senses on the CPU), each
    played with every row down and with every row up, and the chained raw bits after EVERY update (`game/digest.ts rawBits`), the end and the run digest are
    the same. `--each` plays a row at a time (which row, when the set is red); `--rows a,b`; `DRESS_TRACE=<file>` in front of `--run` writes each tick's bits
    by part (`rawBitsParts`), which says what a step touched. `DRESS_BREAK=1` puts a gameplay step down with the rows (the doors) and must fail.
    Measured 2026-11-02: 38,634 ticks, the same bits with the sixteen rows down, and the five end words are `rust-check --missions`' own; on the frozen
    stand (`TPS_PROFILE=frozen` in front) two of the missions, 6,080 ticks, the same; the red path parts on both of its runs, and a red path whose runs leave
    nothing is said to be NOT red. What putting the rows down saves a headless host: compound-four played alone, twice each way, 35.1 and 35.0 s with the rows
    up, 33.6 and 34.4 s with them down, about 3 % (a headless engine draws nothing, so the hand-over of the men is most of it). The page with every row up
    is the page it was: `rust-page.cjs`' three jobs pass on this tree with the digest and the bits of the tree before it. Not run under node; no page has
    played with a row down (the table is not on `window`).
  - **A NEW ROW** is a step whose code has been READ for the four things dressing may not do (push a sound a man hears, draw from the game's die, move a
    clock a rule reads, write a field a rule reads: a pose, a light, a box a ray meets; and one more, put smoke where the senses read it: `smoke.emit` feeds the
    CPU's own smoke and its budget, `sense.ts ownSmoke`, where `smoke.push` and the wind move only the solver's) and that then passes the check. The check alone is not enough: it only
    sees the paths the five missions walk (no takedown, no scripted smoke break, no GPU particles).
  - **WHAT THE CHECK'S STAND DOES NOT SAY.** The rows are dressing where the senses are on the CPU (the server, the headless tools). On the page as it opens
    today the guards' eyes read the GPU's light probe and the GPU's smoke, and there `men` (their shadow proxies), `wake` and `wind` (the smoke they move) reach
    a rule through the picture. That ends when the page plays on the CPU's senses (the maintainer's steer, the part "Run smoke, light and senses on the CPU");
    until then a page with a row down is not promised to be the page with it up.
  - **Tried, and NOT dressing today** (their steps have no row and run everywhere): the tracers (`tracers.ts`: a streak's head carries a light, the light set
    is what the CPU's meter reads: one mission parted); what the men carry in hand (`handProps`: two missions parted, Sam's own numbers first); wetness
    (`wet.ts`: the raw bits did not move and the run digest did, which holds it); `threatTick` (it writes the pose's `deliberate`, and bones are what sight, the
    meter and rounds read); `takedownTick` and `smokebreak.ts` (each writes a pose; the missions exercise neither). THREE PASSED THE CHECK AND ARE NOT DRESSING, BY READING: the
    breath on a snow level (`snowBreath.ts` emits smoke, which the CPU's senses read and which counts against the smoke budget a grenade is emitted under),
    the sparks (`sparks.ts`: a stun's smoke trail emitters ride the pool's live positions) and the look of a wall charge (`blastLook.ts` warms the colour of
    the stun flash's light, and the meter drops a light by its brightest channel). And inside `drawMen`: WHAT A MAN WEARS
    (`c.parts`, `cast.ts castParts`) is dealt there, to the men the fog lets be drawn, and his kit's boxes are placed off it (`rigProps.ts packOf`); so that
    one statement stays with the `men` row down. That a man's kit depends on whether he was drawn is found, not changed.
  - **Found, for the maintainer's call** (the page of this work puts them to him; nothing changed): debris (`debris.ts`) is gameplay three ways today: a
    landing files a `'prop'` sound the men hear, the pool's size is `settings.debrisCap` (64 on the two top graphics tiers, 32 below: two machines on different
    tiers do not play the same game), and its chunks are in the raw bits. Glass: where shards land becomes the litter that makes a footstep louder
    (`shards.ts` underfoot).
  - **In Rust**: `rust/sim` is gameplay, `rust/dress` is dressing, `rust/prim` is for both (`tools/sim/build.ts SIDES`; a crate with no row is red). They are
    TWO MODULES with a memory each (`src/sim/built.ts`, `src/dress/built.ts`; `src/sim/load.ts` starts either, handing in nothing), so dressing has no road to
    the simulation's memory: what it needs of the game is copied in (`src/dress/dress.ts dressIn()`). The build reads the BYTES that ship
    (`tools/sim/wasm.ts`, every instruction decoded): `sides` (no simulation crate links a dressing one, nor the other way), `alone and strict` (neither module
    imports anything; the simulation's code holds no relaxed SIMD and no threads instruction; an instruction the reader does not know is red until somebody
    names its family), `reader` (that reading tried on hand-made modules that break each rule). Fixed-width SIMD is allowed in the simulation (the same
    answer everywhere); relaxed SIMD only in dressing. `rust/dress` holds nothing but its hand-over today: no dressing system has moved, nothing calls it, and
    no relaxed-SIMD target feature is turned on (which browsers run those instructions was not looked up).
- **What a tick costs with the rows up, and where it goes** (the hill-climb, 2026-11-03; a 4-core cloud box, bun 1.4 and node 24, one seed a
  mission, `tickcost --each --list --rounds 2`, the five runs, 38,634 ticks, the same bits in every table; nothing drawn, never on a GPU).
  - A tick's update, ms: bun 2.02 with every row down · 1.92 with `lines`, `circles`, `fog`, `sight`, `light`, `smoke` up (−4.8 %) · `sound` and
    `paths` add nothing a mean shows. node 2.23 down · 2.17 up (−3 %). A row raised ALONE is inside the noise (1 %); `poses` alone is +6 %
    (its bench copies a pose in and out) until the animator's data is at home in the module.
  - A question, TypeScript → the module, µs (`sense-ident --bench`): an eye 1.1 → 0.8 · the light on a point 8–11 → 5–9 · smoke along a line
    3.3–7.3 → 2.6–5.1 · a sound's way 92–136 → 91–127 · a way across the floor 47–85 → 47–79.
  - WHERE A FRAME GOES (`tools/qa/framecost.ts --scn compound-four,warehouse-four,pass-four,hostage-rescue,block-squads --seeds 1`: the page's
    stand and cadence, every row down, the services timed apart; 42,205 frames, 2.44 ms a frame): THE ANIMATOR IS A QUARTER OF IT (0.60 ms,
    8 to 16 updates a frame); the box list's pack for the picture 18 % (the page's every frame; a server's tick lists instead, 0.02 ms); the
    box world's lines 9 % (0.23 ms, 300 to 460 a frame; the walk is the one hot function of a profile, 6.5 % of a mission's samples in
    TypeScript, 5.0 % as the module's); the fog's pass under 2 %; a sound's way and a way across the floor a few hundredths of a ms a frame,
    and the worst frame of one mission in five (25 ms of paths). The worst frame of the other four (46 to 74 ms) is what stands at the end
    of the update, the checkpoints among it. The tool's marks in `Game.update` have gone stale (sixteen not found): its split BY RULE is
    coarse until they are put back; the services' rows are exact.
  - So the size is in the animator, then in what only a whole tick in the module removes (the small vectors and the garbage they are). The
    rows that stand are near their floor: a tenth off the line walk is half a percent of a tick.
  - Done: `sim.ts` keeps its views over the module's memory and tests their length (a view over a detached buffer has none) where every ask
    fetched `memory.buffer` from the engine, a thousand times a tick with the senses up: 0.6 % of a mission's samples, 1.2 % of the tick with
    the six rows up under bun, nothing under node. `rust-check`'s `views` line grows the memory by a page and holds every view remade.
    `SimState.bind` (state.ts) and `dressIn` (dress.ts) still fetch the buffer on every ask: the same test would do for them.
  - Tried and dropped: fixed SIMD switched on for the whole crate (`+simd128`): no question got faster, nothing in these kernels
    vectorises by itself. A build rule against `memory.grow`: the module has held ONE since its first build (the standard library's
    allocator, linked through `rust/prim` and called by nothing), so the views are held by the length test, not by a rule.
- **Found, not changed.** `tools/qa/exact-check.ts` is red on the build this was cut from, on three sites that are nobody's Rust: a `Math.tan` worked out as
  `scene/round.ts` loads, and a cube and a power of two written with `**` in `game/stalk.ts` and `game/showwhy.ts`. Each is the engine's arithmetic inside
  the game, the thing this work exists to end; they are their owners' to mend and a port of those files must not copy them.
  Also: `exact.ts log10` gives the exponent exactly for every decimal power that is a normal double (1e-307 … 1e308); for the subnormal ones
  (1e-308 and smaller) its own guard misses by a few 1e-9 and the answer stands a hair off. The Rust port does the same to the bit; nothing in the game takes
  the logarithm of a number that small.
- **Not seen.** Nothing here draws, so nothing here was seen on a GPU; the page check runs the bundle on the software adapter with the frames undrawn. Nobody
  has built the crate on a Mac or on Windows.
