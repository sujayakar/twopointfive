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
    box under bun: a sight line 1.81 → 1.08 µs on the office and 1.03 → 0.77 µs on the compound (`tools/qa/segment-bench.ts`), and the tick about 3 % cheaper
    (compound-four 6.13 → 5.77 ms, pass-four 5.94 → 5.99 ms, `tools/qa/tickcost.ts --each --rounds 2`: inside that machine's noise).
  - `circles` — a circle against the boxes (`collideCircle`, `circleTouches`, `gather`: a body pushed out of the statics, what a prop or a rag doll rests
    against) from `collide.rs`, the same copy. `?rustcircles`, `TPS_RUSTCIRCLES=1`. OFF EVERYWHERE.
  - `fog` — the fog of war's sight pass (`fogofwar.ts update`: which of the team's eyes has a line to each man and each body) from `fog.rs`: the eyes are
    handed over once a pass (`collision.ts sightOpen`, `sightEye`) and a man is then ONE question (`eyesSee`) where he was two lines an eye. `?rustfog`,
    `TPS_RUSTFOG=1`. It works its lengths out in the module, so IT ANSWERS ONLY WHERE THE GAME'S OWN MATHS STAND (`EXACT.on`: the page, the server, the
    shipped stand); on the frozen stand, whose lengths are the engine's, the TypeScript answers whatever the row says. That is the rule for every system to
    come that takes a length or an angle. What the fog draws with (`things`, the picture's blocks) is not in the row. OFF EVERYWHERE.
  - `poses` — the posing functions of `src/anim/skeleton.ts` (`blendPose`, `PoseFK.run` / `runFrom` / `parentRot`, `rotateLocal`, `rotateModel`,
    `setModelRot`, `twoBoneIK`) from `skeleton.rs`. `?rustposes`, `TPS_RUSTPOSES=1`. The animator's layers (`game/character.ts`, 6,300 lines) are still
    TypeScript and so are its poses, so a question's pose is written onto a BENCH in the module, asked, and read back. THAT IS A PROOF, NOT A SPEED-UP: it
    shows the two are one arithmetic in a whole game (about 350 questions a tick), and the copies cost what the Rust saves. Only where the game's own maths
    stand. A pose is stored in singles as the Float32Array stores it; `rust/sim/src/single.rs` is the one file that may name a single (the build's rule
    names it as its exception), and nothing is worked out in singles. Not ported yet: `sampleClip` (the clips have to live in the module first) and
    `SkeletonInstance.update` (the twists, the rag doll's overrides, the matrices). OFF EVERYWHERE.
- **Posing: what is left, and it is most of it.** Posing is gameplay, whole (the bullet "Gameplay and dressing"), and about a sixth of the tick. The time is not in the
  functions above but in the layers that call them, and in the arrays they allocate on the way. Those layers move to Rust only WITH THEIR DATA: the
  poses and the run-forward bones in the module's memory, so a layer written in Rust calls `skeleton.rs` with no crossing and no copy. That is the
  first system whose state lives in the module (the bullet "State in the module is state of the game"), and `game/rollback.ts` is not ready for it:
  it copies a typed array by its whole buffer when it maps one game onto another (`bufFor`), which for a view over the module's memory is all of it.
  So the order is: the pose store and the rollback snapshot agreed with whoever owns rollback; then the layers, one at a time, each held by
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
  A NEW SYSTEM GETS THE SAME THREE: a differential of its own on recorded inputs (`tools/cpu-frame/raycast-diff.ts` is that for the collision world), the
  missions with its row on and off, and the page.
- **State in the module is state of the game.** Today the module keeps none between calls (the maths are pure; `sim_scratch` is written and read inside one
  call), and `tools/qa/rollback-exempt.ts` says so on its `src/sim/sim.ts X` row. The first system that keeps state in the module's memory takes that row out:
  that memory is then part of the rollback snapshot, a checkpoint and the raw bits (`docs/agents/replays.md`), and `src/sim` is under the rollback check's static
  pass for that reason. A typed array over the module's memory dies when the memory grows: never keep one across a call that can allocate.
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
- **Found, not changed.** `tools/qa/exact-check.ts` is red on the build this was cut from, on three sites that are nobody's Rust: a `Math.tan` worked out as
  `scene/round.ts` loads, and a cube and a power of two written with `**` in `game/stalk.ts` and `game/showwhy.ts`. Each is the engine's arithmetic inside
  the game, the thing this work exists to end; they are their owners' to mend and a port of those files must not copy them.
  Also: `exact.ts log10` gives the exponent exactly for every decimal power that is a normal double (1e-307 … 1e308); for the subnormal ones
  (1e-308 and smaller) its own guard misses by a few 1e-9 and the answer stands a hair off. The Rust port does the same to the bit; nothing in the game takes
  the logarithm of a number that small.
- **Not seen.** Nothing here draws, so nothing here was seen on a GPU; the page check runs the bundle on the software adapter with the frames undrawn. Nobody
  has built the crate on a Mac or on Windows.
