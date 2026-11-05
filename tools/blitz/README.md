# tools/blitz — the tick loop

The game's simulation exists twice: the TypeScript under `src/` (the game as it ships) and a Rust translation of it in `rust/game`. This folder holds the state
bridge between the two and the loop that brings the Rust side up against the TypeScript side, one tick at a time.

**The TypeScript is the truth.** The loop never edits it and neither do you. Every difference is mended on the Rust side, or named in `known.txt` with its reason.

## What the loop is

1. A real mission is stood up headless in the TypeScript game (`tools/qa/playthrough.ts` and its scripted player), on the page's stand (`TPS_PROFILE=shipped`).
2. As tick K opens, its whole state is exported through the bridge (`export.ts`). That blob, and the simulation block of `rust/sim` at that moment, are the one
   start both sides step from.
3. The TypeScript game steps N ticks; the state after each tick is exported.
4. A native test binary (`rust/game_run`) imports the start **once**, copies the block in, and calls the translated tick N times. After each tick it exports its state.
5. After each tick the two states are compared, bytes first and then path by path. Paths named in `known.txt` are left out. The loop stops at the first tick that
   has anything else, and says what.

From its first tick on the Rust game is its own: its die, its knobs, its poses, its lights. Nothing is re-imported, so a small difference at tick 12 is still there
to be seen at tick 13.

## The one command

```sh
tools/blitz/run.sh <mission>:<seed> [--ticks N] [--from K] [--keep] [--verbose] [--show N] [--out DIR]
tools/blitz/run.sh room-clear-four:1 --ticks 40
```

It needs `bun` and `cargo` on the PATH (the toolchain is pinned in `rust/rust-toolchain.toml`) and nothing outside the repository. It builds the runner into
`rust/game_run/target` (ignored) and puts its blobs in a temp folder that it names on its first line (`$TMPDIR`, or `--out DIR`). Nothing is written into the tree.

| flag | |
|---|---|
| `<mission>:<seed>` | a scenario of `tools/qa/playthrough.ts` and its seed. A wrong name prints the list. |
| `--ticks N` | step N ticks side by side (default 40). Tick numbers in all output count from 1 at the start. |
| `--from K` | take the start as the mission's tick K opens (default 1). The TypeScript game alone plays the ticks before it, so a later stretch of a mission can be reached without the Rust side having to be right up to there. A start in mid-mission is less trodden than a start at tick 1: the dress rows are put down at tick K after K−1 ticks with them up, so look for a harness artefact first in what its first tick shows. |
| `--keep` | keep every tick's blobs. By default only the last tick compared is kept, with the start. About 30 MB a tick. |
| `--verbose` | a line for every tick (default: ticks 1, 10, 20, … and the last), and the runner's notes. |
| `--show N` | how many paths of each module to print at the tick that differs (default 6). |
| `--no-build` | use the runner as it was last built. |

Exit code: `0` every tick the same but for what is known · `1` an unknown difference · `2` the Rust tick panicked or the runner died · `3` the build or the
TypeScript side failed · `4` a process went over the memory ceiling · `64` bad arguments.

One turn of the loop is a build and a run. On a 4-core machine: the first build is about a minute and a build after an edit of one part about 20 s; the
TypeScript side is about 12 s for 40 ticks; the Rust side and the diffs are about 2.5 s a tick. So 40 ticks are about two minutes a turn, and a mend near
tick 1 is quicker to try with `--ticks 3`. The compiler is the largest process (about 3.4 GB on a first build); a tick's diff is about 2 GB.

## Reading the output

A clean run:

```
blitz: room-clear-four:1, 40 ticks from the mission's tick 1, stand shipped; blobs in /tmp/blitz-Ab3dEf
build: ok (57 s)
typescript: room-clear-four seed 1, mission ticks 1..40, stand shipped; game time 0.6667; die n=11; 9 guards; 1 orders of the driver's; 0.52 GB resident
typescript: ok (12 s)
tick 1: 3477 paths differ, 3477 known, 0 unknown; block the same bytes; 1.72 GB
…
ticks 1..40: nothing unknown, and rust/sim's block the same bytes at every tick
```

`paths differ` counts every leaf of the two states that is not the same; `known` are those a line of `known.txt` names; `unknown` is the work. `block` is the
simulation block of `rust/sim` (doors, men, items, poses, lights, the die), compared as bytes. `die n=` is how many draws the game's die has made: both sides
draw from the die in the block, so a draw too many or too few shows as a difference in the block's dice part.

A tick that differs prints the unknown paths by the module that declares the field, the count and the first few with both values, and stops:

```
tick 39: 3512 paths differ, 3481 known, 31 unknown (scene_boxes 19 · fogofwar 12); block the same bytes; 1.90 GB
  scene_boxes: 19
      g.tab.boxes[412].emissive[0]   ts 1.65 666666666666fa3f | rust 11 0000000000002640
      …
  fogofwar: 12
      …
STOPPED AT TICK 39: the first tick with a difference that known.txt does not name (ticks 1..38 were the same but for what is known)
```

A number is printed with its eight bytes as they are stored, since the two sides must agree to the bit. `(absent)` on one side means the path is not in that state at all (a list of
another length, another arm of a union). A row of a table that was made in the tick may carry another ID on each side; look at what it holds, not at its number.
The module is where the FIELD is declared, which is often not where the mistake is: find who writes it (below).

A panic prints the tick, the message with its place, and the game's own frames of the backtrace, innermost first:

```
THE RUST TICK 1 PANICKED (no tick before it was compared):
  panicked at rust/game/src/packside/sig.rs:12:122:
  not yet implemented
     game::packside::sig::pack_of
     game::rig_props::p_03::pack_side
     …
```

`sig.rs: not yet implemented` is a function nobody has translated yet (a stub). `p_NN.rs: needs: …` is a seam: a translated body that waits on something
its part could not reach when it was written. A runner that dies with a signal and no panic is most often a stack overflow: a function calling itself.

## Looking closer

```sh
# this path on both sides at tick k: as the tick opened (TypeScript), and as it returned (TypeScript, Rust)
bun run tools/blitz/peek.ts <run dir> <tick> '<regex of paths>' [--differ] [--most N]

# who writes it, in the TypeScript: every write of one property during tick T, with its stack
TPS_PROFILE=shipped bun run --preload ./tools/blitz/expose.ts tools/blitz/watch.ts <mission>:<seed> [--from K] --tick T '<expression of g>' <property>

# who writes it, in the Rust: a hardware watchpoint on one double under gdb
tools/blitz/watch-rust.sh <run dir> <ticks> <what>:<row>:<index>

# one tick's diff alone, with every differing path (known ones too) in a file
bun run tools/blitz/tick-diff.ts <run dir> <tick> --all /tmp/d.tsv        # BLITZ_KNOWN=1 also prints the count of each known line
```

`<run dir>` is the folder `run.sh` named. It holds the last tick compared; run with `--keep` to hold them all. Paths are the bridge's (Rust names: `g.ch[4].pos[0]`,
`g.tab.boxes[412].emissive[0]`, `g.m.<module>.…` for a module's own state). `watch.ts` takes a JavaScript expression instead, since it runs inside the TypeScript
game: `'g.guards[2]' irradiance`. The stack it prints names the TypeScript functions (`at unitLightMeter src/game/guards.ts:2683`); every Rust function's doc line
in `sig.rs` names its source the same way (`/// (src/game/guards.ts:2681 unitLightMeter)`), so `grep -rn unitLightMeter rust/game/src/*/sig.rs` gives the Rust
name, and `grep -rn 'fn unit_light_meter' rust/game/src/guards/` the part that holds its body. `watch-rust.sh` knows a few fields by
name (`time`, `ch.pos`, `box.c`, …); add an arm to `watch_addr` in `rust/game_run/src/main.rs` for another.

## How to mend

The Rust game is `rust/game/src/<module>/`, a folder for each TypeScript file:

- `types.rs`, `sig.rs` — GENERATED. Never edit them. `sig.rs` holds every function's signature with a `todo!()` body (the stub); `types.rs` the state's types.
- `p_NN.rs` — the translated parts. `b_01.rs` — bodies added during bring-up: stubs that a tick reached, and the host steps below.
- `mod.rs` — names which functions each part supplies (`pub use p_02::{…}`): a named function of a part wins over its stub in `sig.rs`.

Rules:

1. Edit `p_NN.rs` or `b_01.rs`. Keep each function's signature exactly as `sig.rs` has it; other parts call it by that signature.
2. To fill a stub, write the function in the module's `b_01.rs` (make the file with the same header as any other `b_01.rs` if the module has none) and add its
   name to the `pub use b_01::{…}` line of `mod.rs`. Without that line the stub still wins and still panics.
3. Translate what the TypeScript does, statement for statement, in its order: the order of draws from the die and of rows made in a tick is part of the answer.
   Each function's doc line in `sig.rs` gives its source (`/// (src/game/lighting.ts:75 updateTargets)`).
4. Calls into the picture, the sound and the host are no-ops, and live in `b_01.rs` parts marked `BOUNDARY NO-OP`. Before you make something a no-op, check that
   no rule reads what it writes (see "a branch dropped as picture" below).
5. Where the generated types cannot say what the TypeScript says, do not bend `types.rs`: work round it in the part and leave a `// SKELETON-GAP: …` line saying
   what the type should be. Where the bridge does not carry something, leave a `// BRIDGE-ASK: …` line. These are the lists the generators' owners work from.
6. Run the loop again. A mend is done when the tick that stopped passes and no earlier tick has begun to differ.
7. A line goes into `known.txt` only with its reason, and only when the difference is not a rule of play going wrong.

`rustknobs/b_01.rs` and `root_settings/b_01.rs` are written by `tools/blitz/gen-knobs.py` and `gen-settings.py` (python3) from the generated types; run them
again when the list of knobs in `rust/sim/src/knobs.rs` or the settings' fields change, rather than editing those two files.

## The kinds of cause seen so far

- **A translation mistake.** A body that does not do what its TypeScript does: a helper that calls itself where it should cast, a tuple filled in the order of a
  comment where the callers read another order, a call left out (`handProps` is `handPropsRaw` and then `rigKeys`; only the first was called).
- **A branch dropped as "picture" that a rule reads.** The most expensive kind, because it shows late. `Lighting.updateTargets` ends by painting every fixture's
  boxes with their emissive; that was left out as the renderer's, but the fog of war reads a lamp's face from those boxes and the light estimate reads the lamps'
  intensity. It showed 39 ticks in, the first time a tube flickered. A file that is "dress" can still hold a pure function that a rule calls (what a man wears
  decides where his kit hangs).
- **A skeleton gap.** The generated types or stubs cannot say it: a table that did not exist yet, a field that lost its identity, an enum with no variant for a
  closure. Work round it in the part, mark it `SKELETON-GAP`, and if a difference remains that no rule depends on, name it in `known.txt`.
- **A bridge gap.** The export does not carry something a tick reads (the engine's lights and settings, an order's closures), or walks the two sides differently
  (a reference written as a value, a serial dealt by identity). Either a host step hands it over (below) or `known.txt` names it.
- **A harness artefact.** The two sides were not run alike: dressing rows up on one side only, the host's work between ticks missing on the Rust side, the test
  driver giving an order through the menu between two ticks. Mended in this folder or in `rust/game_run`, never in the game.

## Host steps: stand-ins for what the bridge does not carry

Each of these exists because the bridge or the Rust skeleton lacks something today. They are the harness's, are marked as such where they stand, and go when the
gap is closed.

| step | where | stands in for |
|---|---|---|
| `host_engine_fill`, `host_world_upload` | `rust/game/src/host_api/b_01.rs`, fed by `engineText` in `host.ts` | `Game.engine` (the lights' list in its order, the box world's lists and counts, the settings) is not carried, and a tick reads it. The headless host lists the box world again between some ticks. |
| `orders_closures_again` | `rust/game/src/orders/b_01.rs` | an imported order row has no closures; each is dealt its enums again by its id. |
| `bringup_module_state` | `rust/game/src/anim_skeleton/b_01.rs` | `rust/sim` keeps tables outside a game's block (the furniture's facts by row) that a fresh module does not have. |
| `host_half`, `give` | `rust/game_run/src/main.rs` | the host's code between two ticks is not run on the Rust side: the input record it sampled, its own camera and Input, the HUD's room memo, and the test driver's orders are taken from the TypeScript run. |
| the settle, the die, the dress rows | `stand.ts` | the start is taken after `sim.settle` (a pose outside the pool is not carried); the game is given its own die (so nothing of `Math.random` is handed over); every dress row is put down (the Rust tick is the tick with them down). |

## The memory rule

A decoded state is over a million paths. So: one build or one run at a time; `run.ts` decodes nothing itself; each tick's diff is a process of its own that
holds the two states of that one tick and exits; two blobs that are the same bytes are never decoded; nothing is collected across ticks. Every bun process of the
loop checks itself against a ceiling of 4 GB resident (`BLITZ_CEIL_GB`) and stops with exit 4 above it. If you write a tool that walks many ticks, start a
process a tick: a first form of this loop that walked them in one process grew to 7.7 GB.

## Files

| | |
|---|---|
| `run.sh`, `run.ts` | the one command: build, TypeScript side, Rust side and diff in lock step |
| `stand.ts` | the mission stood up headless and held at its ticks; shared by every tool so they all see the same run |
| `ts-ticks.ts` | the TypeScript side: the blobs of N ticks |
| `host.ts` | the host's side: the engine as text for the Rust game |
| `tick-diff.ts`, `known.txt` | one tick's difference, and the differences that are known, each with its reason |
| `peek.ts`, `watch.ts`, `watch-rust.sh` | a path on both sides; who writes it |
| `gen-knobs.py`, `gen-settings.py` | write the two generated `b_01.rs` |
| `export.ts`, `paths.ts`, `expose.ts`, `schema.json`, `not-carried.json`, `bridge-check.ts`, `dump-tables.ts` | the state bridge itself |
| `rust/game_run/` | the native runner |
