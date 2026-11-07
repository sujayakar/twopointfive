# The tick's out events

What a rule does that the host has to show or play, as a list the host drains after each step. Written by `outlist.py` (decides), `outevents.cjs` (Rust), `puppetgen.cjs` (dispatcher).

## What is generated
- `crate::out_events::OutEvent`: 33 arms, one per boundary function that only tells the host something, each with a struct of its parameters as owned values. A `&WeaponDef` is held by its `WeaponId`; a `&mut` is dropped.
- `g.out: Vec<OutEvent>`. It is NOT in the whole state (`io::export`, `io::import`, a checkpoint): the list is a step's output, not the game.
- 31 functions get a generated body in `sig.rs` that pushes the event (and answers `None`, `0.0` or `false` where the function has an answer).
- 2 have real bodies of their own (`smoke_system_like_clear_all`, `smoke_system_like_emit`): `out_events::push_*` for the part to call beside them.
- `export_puppet` writes `g.out` on its tail, after the loose poses. `applyGame` with `puppet` returns them as `Report.out = [{ arm, e }]`, with IDs back to objects through the book.
- `bridge/out-dispatch.ts`: `dispatchOut(g, report.out)` makes today's call on today's objects, in the order the rules made them.

## Taking it
1. `python3 out-take.py <crate src>` deletes the parts' 31 empty functions and their `pub use` names. It leaves and names anything that is no longer an empty line. On commit 60eb701fb it took 31 and left 0, and that tree with this skeleton's `sig.rs`, `types.rs`, `io.rs`, `puppet.rs`, `lib.rs` and `out_events/` passes `cargo check`.
2. Beside the real bodies of the two smoke functions, call `push_smoke_system_like_clear_all` and `push_smoke_system_like_emit` (if the host's smoke is to hear them).
3. The host: after `export_puppet`, `g.out.clear()`. Then, after the apply, `dispatchOut(g, report.out)`.

## What is listed, not generated
`out-events.json`'s `listed`: 130 boundary functions that stay as they are, each with why. 83 answer a value the rule goes on to use, 37 have real bodies, the rest are short forwards or have no Rust function.

## What you need to know
- **The events run after the step.** A function of the game (`recapRound`, `messBurst`, `bulletMark`...) that reads the game reads the end of the step, not the moment of the call. In TypeScript it ran mid-tick. None is known to matter; none has been compared.
- **Rollback.** Resimulated ticks push their events again. Co-op's host has to drop them. Nothing here does that.
- **An answer the rule reads, two places.** `break_table/p_02.rs:408` hands `decals_add`'s answer to `look_decal`; with `None` that chip mark loses its look. `particle_fx/p_01.rs:484` adds up `particles_emit`'s answers; it now adds zeros.
- **Particles stay silent until a host sets a stand-in.** `gpu_fx` asks `g.engine.particles.is_some()`, and the bridge's engine list has no particles.
- **Chair voices.** `PlayOpts::ChairRoll { tok }` becomes game.ts's `follow` closure again in the dispatcher, by finding the roll whose `voice` is `tok`. That is the one closure that crosses.
- **The fire's bed.** fire.ts:379 plays `fireBed` with a `follow` closure. The Rust fire has no such call, so no event carries it, and a fire has no held voice on the Rust side.
- **Null.** Three parameters are `T | null` in TypeScript and `Option` in Rust (`audio.play`'s `pos`, `reach.changed`'s `rect`, a mark's `s`): the dispatcher writes `?? null`. The rest are optional and take `undefined`.

## An arm that changes the game

`CheckpointsKeep` and `CheckpointsRequest` do, and they are not the picture's. **They are the Rust game's to carry out, never the puppet's.** The save controller's `tick` on the puppet calls `D.stop(false); g.rails.resume()` on a program it doesn't run, and an authored checkpoint really does stop the level's program for the take, so a host that only counts and clears them parts 33 ticks later (bringup/host1-log.md 22 and 23), and one that runs them on the puppet throws at director.ts:525.

**The shape:** set `OUT_HOST.checkpoint = (g, what, e) => ...` in `out-dispatch.ts` and both arms call it. There the host stops the program on the Rust side, takes the state from Rust, and checkpoints on the Rust side, in that order, as your harness now does. The save is Rust's bytes. With no hook set they call `g.saves` as before, which is right only for a game that isn't a puppet. The arm itself carries no more than the words.

The checker can't show this (`Checkpoints` is a host object, so `request` writes no game field). What it says of the rest, in `inventory/out-arms.json`: `SessionRecorder.cut` writes the recorder's own counters; `messBurst`, `messOpened`, `messWall` and `bangScorch` write only `StaticCollision`'s query scratch. The other 26 write no game field. Read off the static list, not run.

## How it was checked
- `cargo test --lib out_round` (`OUT_BLOB`, `OUT_FILE`): imports a whole state, calls each generated function once with zero bytes, plays four sounds with each kind of options, and writes a filtered start: 37 events.
- `bridge/out-check.ts`: stands a game at that frame, applies the file and dispatches every event. Methods go to recording stand-ins, functions of the game run for real on room-clear-four: 33 of 33 arms, none threw.
- `puppet-check` on room-clear-four, six steps: 188,784 values on the every-tick list, 0 differ. The new tail cost nothing it can see.

NOT checked: events from the Rust game's own stepping. Each arm has been seen once with made-up values, never with the values a real rule hands it, and never in a page.
