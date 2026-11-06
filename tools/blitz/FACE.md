# tools/blitz — the Rust game's face: one wasm module

`rust/game_face` builds the translated game (`rust/game`), what a host does round a step (`rust/game_run`'s library) and the simulation (`rust/sim`) into
ONE WebAssembly module with one memory. A host hands it a start, steps it a tick at a time and reads it back. `rust/sim`'s own C exports are exports of the
same module, so there is one block of `rust/sim`'s a game: the one the Rust game steps.

Nothing loads it in the page yet. What runs it today is the mission loop through wasm, below.

## Build

```sh
cd rust/game_face
cargo build --target wasm32-unknown-unknown                    # dev: opt-level 1, no debug info      target/wasm32-unknown-unknown/debug/game_face.wasm
cargo build --target wasm32-unknown-unknown --release          # opt-level 3, fat LTO, one unit       …/release/game_face.wasm
cargo build --target wasm32-unknown-unknown --profile small    # the same at opt-level "s"            …/small/game_face.wasm
```

The compiler and the target are `rust/rust-toolchain.toml`'s. On two cores: dev about 3 minutes from nothing, release 6, small 3.5. The module imports
nothing (wasm-run.ts stops on one that does). The face crate links `std` (the panic hook, the allocator); `rust/game` and `rust/sim` stay `no_std`.

## The mission loop through wasm

```sh
tools/blitz/wasm-run.sh room-clear-four:1                 # the TypeScript side once (mission.ts), the dev build, then every tick through wasm
tools/blitz/wasm-run.sh <DIR> --release -- --two          # a folder run.sh already filled (--out): the mission is not played twice
bun run tools/blitz/wasm-run.ts <DIR> [ticks] [--wasm FILE] [--two] [--one-call] [--reserve MB] [--max-mb N] [--quiet]
bun run tools/blitz/wasm-run.ts --exports                 # the export list beside the shipped simulation's
bun run tools/blitz/wasm-run.ts --ts-time <mission>:<seed>   # the TypeScript tick's own time for the same play
```

`wasm-run.ts` reads what `mission.ts` wrote (the start `c1.bin` + `s0.bin`, the host's half `h<k>.bin`, `host.txt`, `gives.txt`, `hostwrites.txt`, a
tutorial's `lesson.json` and `meter.txt`), makes a game in the module, and for every tick hands over the host's half, steps, and holds the die's count and
the hash of each of the block's 13 parts to `post.txt`'s line. It stops at the first tick a part other than `poses` parts and names the part (`poses` parts
by design from the first death: README.md). The lines round a step are the native runner's own: both call `rust/game_run/src/lib.rs`.

Exit: 0 in step · 1 the block or the die parted · 2 the Rust tick panicked (the panic's words and place are printed) · 3 a file is missing.

What was run (dev build unless said; every tick, the die's count and 12 parts by the module's hashes, every 100th tick also hashed in the host over the
module's memory in place):

| mission | ticks | through wasm | `poses` first parts | the native runner on the same folder |
|---|---|---|---|---|
| room-clear-four:1 | 3,006 | in step (dev, release, small) | tick 1794 | in step, tick 1794 |
| hostage-rescue:1 | 3,119 | in step | tick 740 | in step, tick 740 |
| glasshouse-four:1 | 4,276 | in step | tick 690 | in step, tick 690 |
| mousehole-four:1 | 7,087 | in step | tick 1764 | not run here |
| tut-solo:1 (a tutorial, its level script) | 8,052 | in step | tick 404 | in step, tick 404 |

Also run on room-clear-four:1: `--two` (two games from the one start in slots 0 and 1, stepped one after the other every tick: both in step for 3,006
ticks); `--one-call` (the upload and the lights' update inside `face_step`: in step, and the `lights` part does not differ, the update writes none of the
block); the red paths (the driver's orders left out: parts at tick 628 in `props`; one hash of `post.txt` altered: parts at that tick; exit 1 both).

Not checked: the game's own state by path (`diff.ts` over `face_export`), a second seat, a start taken anywhere but a mission's first tick, any browser.

## The exports

All C ABI; a pointer and a length are `usize`, a handle a `u32`. The head of `rust/game_face/src/lib.rs` is the reference.

| | |
|---|---|
| memory | `face_alloc(n) -> p` · `face_free(p, n)` · `face_out_ptr()` `face_out_len()` (what the last call answered: bytes or a log line) |
| a panic | `face_panic_ptr()` `face_panic_len()` `face_panic_clear()`: the last panic's words with file and line. The module aborts on a panic, so the host catches a trap and reads the words; the game of that call is gone, the others stand |
| a game | `face_new(state, n, block, bn) -> h` (`io::import` + the block copied into the first free slot; 0 = no slot) · `face_lesson(h, p, n)` · `face_blank() -> h` · `face_drop(h)` · `face_slot(h)` · `face_import_into(h, p, n)` |
| the host's half | `face_host(h, p, n, first)` (one `h<k>.bin`) · `face_give(h, k, line)` · `face_hostwrite(h, k, line)` · `face_meter(h, v)` |
| the step | `face_step(h, dt, flags)`: flag 1 the box world listed again first, `game_step`, the autosave's rows kept by `face_hostwrite`, flag 2 the lights' update. One call a tick for a host that plays · by name: `face_upload(h)` `face_lights(h, dt)` `face_checkpoint(h, label, n)` |
| reading back | `face_export(h)` (`io::export` at `face_out`) · `face_die_n(h)` · `face_time(h)` · `face_rows(h)` · `face_parts()` `face_part_name(i)` `face_part(name, n) -> i` `face_part_addr(slot, i)` `face_part_len(i)` `face_part_hashes(slot) -> p` |
| `rust/sim`'s own | all 272 of the shipped module's exports, same names, kinds and parameter counts (`sim_abi` 34, `state_layout`, `state_len` 3,248,912, 4 slots, 13 parts all equal) |

`--exports` on this tree: same 272 · missing 0 · extra 31 (the `face_*` above).

**Why `rust/sim`'s exports appear with no line that re-exports them** (tried on a three-crate model, dev and release with LTO): a crate's `#[no_mangle]`
functions, public or private, in any of its modules, are exports of the final `cdylib` whenever that crate is in the link; and a crate is in the link when
some line of some linked crate names it (`use`, a path, `extern crate`), directly or through another crate, whether or not the named thing is ever called.
A dependency that stands in `Cargo.toml` and is named by no line is not loaded, and its exports are silently absent. Here `rust/game` and the face both
name `sim`.

## The numbers

Measured on a shared 4-core machine with another build running (load 2 to 3), room-clear-four:1, 3,006 ticks, bun 1.4.4. Times are `performance.now()`
round the call, two runs each, the quieter kept.

| | size | gzip | without the name section | compile in bun |
|---|---|---|---|---|
| dev (opt 1, `debug = false`) | 23,134,291 | 5.1 MB | 20,385,385 | 140 ms |
| release (opt 3, LTO) | 19,263,645 | 5.6 MB | 18,535,851 | 105 ms |
| small (opt "s", LTO) | 11,884,770 | 3.8 MB | 10,840,759 | 65 ms |
| the shipped simulation alone, for scale | 889,195 | | | |

| the tick, ms | mean | worst | how |
|---|---|---|---|
| wasm dev | 1.65 | 234 (tick 2782) | `face_step` alone (the tick; the upload and the lights' update are called by name outside the clock, as the native runner does) |
| wasm release | 1.45 | 230 | the same |
| wasm small | 1.49 | 226 | the same |
| wasm dev / release / small, `--reserve 1500` | 1.46 / 1.35 / 1.41 | 25 / 45 / 33 (tick 1) | the same, the memory grown once before the start |
| native runner, debug (opt 0) | 5.4 | 17 to 23 | `game_step` alone, `std::time::Instant` (the runner's own line) |
| TypeScript, the same play | 2.73 | 138 to 144 (tick 1) | `stepSeats` alone, `wasm-run.ts --ts-time` (the stand and the preparation of mission.ts, nothing written) |
| TypeScript, `tools/qa/tickcost.ts` | 3.00 | 47.7 | `Game.update` on the game server's stand (senses on the CPU), first 120 ticks left out: another play (2,771 ticks), not the same work |

Other missions through the dev build, mean / worst: hostage-rescue:1 1.25 / 158 · glasshouse-four:1 1.27 / 227 · mousehole-four:1 1.61 / 763 ·
tut-solo:1 0.67 / 184. The lights' update is about 0.001 ms a tick.

The optimisation level moves the mean by a tenth, and the worst tick is not the game: see memory.

**Memory.** The module starts at 111.8 MB (the shipped simulation: 47.9 MB; the difference is the 64 MB stack `rust/game_face/build.rs` asks for). After
the import 130 MB. At the end of room-clear-four:1 **1,416 MB** of linear memory (1,485,307,904 bytes) with 571,962 rows in the box table: every made box
is a row for good. hostage-rescue:1 768 MB (442,247 rows), glasshouse-four:1 1,415 MB (699,219), tut-solo:1 768 MB (478,085), mousehole-four:1 **2,711 MB**
(1,192,565 rows): wasm32 ends at 4 GB, and a table that doubles needs the old and the new at once, so a mission half as long again does not fit.

**The memory grows inside a tick**: in 7 of room-clear-four:1's 3,006 steps (first in tick 39), and those are the worst ticks (230 ms at tick 2782, 763 ms
in mousehole-four). Room asked for and given back before the start (`face_free(face_alloc(n), n)`, `--reserve`) is kept by the allocator: with 1,500 MB
no step grew the memory and the worst tick was the first (25 to 45 ms).

## What `src/sim/sim.ts` needs to bind to this module

- **The names hold.** Every name of `SimExports` / `SIM_EXPORTS` is here with the same parameter count and `sim_abi()` is 34, so `start()` passes as it
  stands once it is given these bytes. A check that the module exports exactly `SIM_EXPORTS` has to let the `face_*` names by.
- **The bytes do not fit the loader.** `startModule` compiles synchronously from base64 text in `built.ts`. Chrome compiles on the page's thread only up
  to 8 MB and `tools/sim/build.ts` is red over 6 MB (`MAX_BYTES`); the smallest build is 11.9 MB (10.8 MB without names). The page needs an asynchronous
  compile (and then everything that calls `sim()` before it resolves needs a place to wait), or the module under 8 MB. `tools/sim/build.ts` also needs a
  side for `game`, `game_run` and `game_face` (`SIDES` has none: its `sides` line is red on them today) and a row of `MODULES` for the face.
- **The block is found the same way**: `state_addr(slot)`, `state_len`, `state_part`. The address differs (92 MB up, not 13 MB) and is always asked.
- **Slots: two owners today.** `src/sim/state.ts` decides which game's block stands in which slot (`HELD`, `bind`, `park`) and moves blocks by copying.
  The Rust game holds its slot for as long as it lives (`face_slot(h)`) and reads it without asking. So a game stepped by the face must be PINNED in
  `SimState` to `face_slot(h)` and never parked, and `face_new` must not pick a slot `HELD` has given to a TypeScript game: either the face takes the
  slot from the host as a parameter, or `SimState` asks the face. More than 4 games alive at once needs parking on the Rust side too.
- **Memory growth.** sim.ts says "nothing in the module allocates, so in play the memory never does grow". With the game inside it does, inside a step.
  Where the shipped code survives it: sim.ts `f64()` / `u8()` / `u32()` / `f32()` (a view of length 0 is made again); state.ts `bind()` and
  `f32()`, which make `w` / `d` and the kept sub-views again when their length is 0. Where it does not: anything that reads `SimState.w` /
  `.d` or a kept typed array without passing through `bind()` after a step, which then reads `undefined` without a word (the accessor fields over the
  block, a tool's `g.sim.w`). So after every `face_step` the host must check `memory.buffer` and make every bound `SimState`'s views again (as `view()` does when a
  block moves), or reserve the room before play so that no step grows it. The reserve is cheap only once frame boxes stop being rows for good.
- **The page's rows of `RUST`** then call `rust/sim` on the block the Rust game also writes. That is the point of one module; nothing was tried in a page.

## Worked round, and asked

- **The box mirror's world is the TypeScript game's slot.** `g.col.mirror.slot` is carried by the bridge as a number, so a game imported into slot 1
  wrote its box world into world 0 and read world 1: the second of two games parted at tick 1832 in `doors` (stepped second) or 626 in `props` (stepped
  first). `game_run::start` sets it to the game's own slot; both games are then in step. Asked: `io::import` should not carry it, or should take the slot.
- **A lesson's tests, the level's script, the meter, a pick of a verb, the authored checkpoint** are stand-ins of the runner's that the bridge does not
  carry: they moved into `rust/game_run/src/lib.rs` (`start`, `lesson`, `meter`, `write_before`, `checkpoint`) and the face calls them.
- **`face_host` takes the loop's `h<k>.bin`** (the record, every camera row, every Input row). A host that plays needs a smaller hand-over of its own
  record, camera and Input; `io::take_host_rows` wants a whole imported game, which is megabytes a tick, so neither side of the loop uses it.
- **The 64 MB stack** is the first face's and was not tried smaller.
- Not looked at: `rust/game/src/math_dice/b_01.rs` keeps the host's draws in one `static` for every game (unused by a game with its own die); the knob
  table is one for the module, as it is in the shipped one.
