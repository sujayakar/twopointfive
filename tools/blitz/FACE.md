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
nothing. The face crate links `std` (the panic hook, the allocator); `rust/game` and `rust/sim` stay `no_std`.

## The exports

All C ABI; a pointer and a length are `usize`, a handle a `u32`. The head of `rust/game_face/src/lib.rs` is the reference.

| | |
|---|---|
| memory | `face_alloc(n) -> p` · `face_free(p, n)` · `face_out_ptr()` `face_out_len()` (what the last call answered: bytes or a log line) |
| a panic | `face_panic_ptr()` `face_panic_len()` `face_panic_clear()`: the last panic's words with file and line. The module aborts on a panic, so the host catches a trap and reads the words; the game of that call is gone, the others stand |
| a game | `face_new(state, n, block, bn) -> h` (`io::import` + the block copied into the first free slot; 0 = no slot) · `face_lesson(h, p, n)` · `face_blank() -> h` · `face_drop(h)` · `face_slot(h)` · `face_import_into(h, p, n)` |
| the host's half | `face_host(h, p, n, first)` (one `h<k>.bin`) · `face_give(h, k, line)` · `face_hostwrite(h, k, line)` · `face_meter(h, v)` |
| the step | `face_step(h, dt, flags)`: `game_step`, the autosave's rows kept by `face_hostwrite`, flag 4 the host's view of a level on rails (a flash's envelope is the tick's own last line). One call a tick for a host that plays · by name: `face_view(h)` `face_checkpoint(h, label, n)` |
| reading back | `face_export(h)` (`io::export` at `face_out`) · `face_die_n(h)` · `face_time(h)` · `face_rows(h)` · `face_parts()` `face_part_name(i)` `face_part(name, n) -> i` `face_part_addr(slot, i)` `face_part_len(i)` `face_part_hashes(slot) -> p` |
| `rust/sim`'s own | all 272 of the shipped module's exports, same names, kinds and parameter counts (`sim_abi` 34, `state_layout`, `state_len` 3,248,912, 4 slots, 13 parts all equal) |

`--exports` on this tree: same 272 · missing 0 · extra 32 (the `face_*` above).

**Why `rust/sim`'s exports appear with no line that re-exports them** (tried on a three-crate model, dev and release with LTO): a crate's `#[no_mangle]`
functions, public or private, in any of its modules, are exports of the final `cdylib` whenever that crate is in the link; and a crate is in the link when
some line of some linked crate names it (`use`, a path, `extern crate`), directly or through another crate, whether or not the named thing is ever called.
A dependency that stands in `Cargo.toml` and is named by no line is not loaded, and its exports are silently absent. Here `rust/game` and the face both
name `sim`.

## The numbers

Measured with frame boxes on, on a 4-core machine with little else running, room-clear-four:1, 3,006 ticks, bun 1.4.4. Times are `performance.now()`
round the call, two runs each (they agree to the second digit).

| | size | gzip | without the name section | compile in bun |
|---|---|---|---|---|
| dev (opt 1, `debug = false`) | 23,137,076 | 5.1 MB | 20,387,474 | 125 ms |
| release (opt 3, LTO) | 19,265,585 | 5.6 MB | 18,537,679 | 120 ms |
| small (opt "s", LTO) | 11,887,897 | 3.8 MB | 10,843,590 | 65 ms |
| the shipped simulation alone, for scale | 889,195 | | | |

| the tick, ms | mean | worst | how |
|---|---|---|---|
| wasm dev | 1.37 | 23 (tick 1) | `face_step` alone (the tick; as measured then, the lights' update and the host's view were called by name outside the clock, as the native runner does) |
| wasm release | 1.23 | 31 to 34 (tick 1) | the same |
| wasm small | 1.25 | 21 to 22 (tick 1) | the same |
| wasm release, one call a tick | 1.21 | 31 (tick 1) | `face_step` with the lights and the view inside |
| native runner, debug (opt 0) | 5.3 | 17 | `game_step` alone, `std::time::Instant` (the runner's own line) |
| TypeScript, the same play | 2.68 | 125 to 150 (tick 1) | `stepSeats` alone, `wasm-run.ts --ts-time` (the stand and the preparation of mission.ts, nothing written) |
| TypeScript, `tools/qa/tickcost.ts` (before frame boxes; it does not run the Rust game) | 3.00 | 47.7 | `Game.update` on the game server's stand (senses on the CPU), first 120 ticks left out: another play (2,771 ticks), not the same work |

Other missions through the release build, mean / worst: tut-solo:1 0.49 / 17 · mousehole-four:1 1.14 / 27, the worst the first tick in both. The lights'
update and the host's view together are about 0.001 ms a tick (0.01 on the tutorial).

Before frame boxes, on a loaded machine, the same mission: dev 1.65, release 1.45, small 1.49 mean, and a worst tick of 230 ms (763 ms in mousehole-four)
where the memory grew; an optimised native runner (`cargo build --release` in `rust/game_run`) 1.06 to 1.11 mean, 7 to 9 worst. The optimised native tick
was not timed again with frame boxes.

**Memory.** The module starts at 111.5 MB (the shipped simulation: 47.9 MB; the difference is the 64 MB stack `rust/game_face/build.rs` asks for) and
stands at 130 MB after the import.

| at the mission's end | linear memory | rows of the box table | steps that grew the memory | before frame boxes |
|---|---|---|---|---|
| room-clear-four:1 | 134.9 MB (141,492,224 bytes) | 4,232 | 1 of 3,006 (tick 2956) | 1,416 MB, 571,962 rows, 7 steps |
| mousehole-four:1 | 136.4 MB (142,999,552 bytes) | 4,987 | 2 of 7,087 (first tick 1791) | 2,711 MB, 1,192,565 rows, 8 steps |
| tut-solo:1 | 131.5 MB (137,887,744 bytes) | 2,359 | 1 of 8,052 (tick 4557) | 768 MB, 478,085 rows, 6 steps |

The rows still grow slowly (a few makers still make a row for good), so **the memory still grows inside a tick now and then**; such a step is no longer
the worst tick (the worst is the first, 21 to 34 ms). Room asked for and given back before the start (`face_free(face_alloc(n), n)`, `--reserve`) is
kept by the allocator: with `--reserve 32` no step of room-clear-four:1 or mousehole-four:1 grew the memory (143.6 MB at the end of both).

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
  block moves), or reserve the room before play so that no step grows it. With frame boxes 32 MB was enough for the two missions tried.
- **The page's rows of `RUST`** then call `rust/sim` on the block the Rust game also writes. That is the point of one module; nothing was tried in a page.

## Worked round, and asked

- **The host's view** (`rails_host_view` after each tick, a level on rails) is a stand-in of the runner's too: `game_run::view`, `face_view`, flag 4.
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
