# The simulation in Rust

The maintainer's ask (2026-11-02): "we should just write this in rust so we have good control over everything (and aren't subject to the vagaries of each JS
engine). and then we can also use wasm simd for accelerating things (e.g. ECS layout)", and earlier that a Rust simulation "might even be desirable for us
running this elsewhere". The game's update (`Game.update`, about 100,000 lines over `src/game`) is moving into Rust, built to WebAssembly, a system at a
time. What is built so far is the ground: the crate, its build, its one way into the page, the server and the tools, the switch table, the checks, and the
first system across (the game's own maths). No rule of play has changed and nothing a player sees is different.

- **Where it lives.** `rust/` is a Cargo workspace (`rust-toolchain.toml` names the compiler and the WebAssembly target; no crate from outside, so no network
  at build time). `rust/sim` is the simulation: `src/lib.rs` (what every line owes the game, and the C exports, named `<module>_<thing>`), `src/maths.rs`,
  `src/pow10.rs` (the decimal powers as the language reads them, written by `bun run tools/sim/pow10.ts`: the Rust side never parses a decimal). `rust/prim`
  is three functions (√, floor, trunc): the simulation is `no_std` and the core library lacks them.
  `src/sim/built.ts` is the module's bytes as text, WRITTEN BY `bun run tools/sim/build.ts` and checked in. `src/sim/sim.ts` is the one TypeScript file that
  loads the module (`sim()`), names its exports (`SimExports`, `SIM_EXPORTS`, `SIM_ABI`), hands out its memory (`f64()`, `u8()`) and holds the switch table
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
  what's just client side only dressing (debris, smoke details) and uses relaxed simd and/or gpu"). `rust/sim` is the gameplay side: everything a fight's
  outcome depends on, the same bytes on the page and on the server, never relaxed SIMD. Dressing gets a crate of its own in the same workspace that can read
  the simulation's state and cannot write it. The list of which system is which, and that crate, are the next step of this work and are not built yet.
- **Found, not changed.** `tools/qa/exact-check.ts` is red on the build this was cut from, on three sites that are nobody's Rust: a `Math.tan` worked out as
  `scene/round.ts` loads, and a cube and a power of two written with `**` in `game/stalk.ts` and `game/showwhy.ts`. Each is the engine's arithmetic inside
  the game, the thing this work exists to end; they are their owners' to mend and a port of those files must not copy them.
  Also: `exact.ts log10` gives the exponent exactly for every decimal power that is a normal double (1e-307 … 1e308); for the subnormal ones
  (1e-308 and smaller) its own guard misses by a few 1e-9 and the answer stands a hair off. The Rust port does the same to the bit; nothing in the game takes
  the logarithm of a number that small.
- **Not seen.** Nothing here draws, so nothing here was seen on a GPU; the page check runs the bundle on the software adapter with the frames undrawn. Nobody
  has built the crate on a Mac or on Windows.
