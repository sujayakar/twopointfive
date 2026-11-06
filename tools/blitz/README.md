# tools/blitz — the mission loop

The game's simulation exists twice: the TypeScript under `src/` (the game as it ships) and a Rust translation of it in `rust/game`. This folder holds the state
bridge between the two and the loop that plays a whole mission on both and stops where they part.

**The TypeScript is the truth.** The loop never edits it and neither do you. Every difference is mended in `rust/game/src`, or named in `known.txt` with its
reason. Never mend the TypeScript, the bridge, or the loop to make a difference go away.

## The one command

```sh
tools/blitz/run.sh <mission>:<seed> [--ticks N] [--keep "1,2,100-110"] [--every N]
tools/blitz/run.sh room-clear-four:1 --every 100
```

It needs `bun` and `cargo` on the PATH (the toolchain is pinned in `rust/rust-toolchain.toml`) and nothing outside the repository. It builds the runner into
`rust/game_run/target` (ignored) and puts its blobs in a folder it names on its first line (`--out DIR`, or one under `$TMPDIR`). Nothing is written into the tree.

What it does:

1. **The TypeScript side, once** (`mission.ts`, about 20 s a mission). The mission is stood up headless on the page's stand (no `TPS_PROFILE` word) and played by
   `tools/qa/playthrough.ts`'s scripted player. Every tick it writes the host's half (below) and, after the tick, a hash of each part of `rust/sim`'s block and
   the die's count. A whole state is written only at the ticks you ask for (`--keep`, `--every`): a state grows by about 35 KB a tick.
2. **The Rust side** (`rust/game_run`, bin `mission`). It imports the state as tick 1 opens **once**, copies the block in, and steps the translated tick through
   the mission. From then on the Rust game is its own: its die, its knobs, its poses, its lights. Before each tick it takes only the host's half. **After every
   tick** it compares its block's hashes, part by part, and its die's count with the TypeScript's, and stops at the first tick that parts.
3. **The path diff** (`diff.ts`) of every tick both sides kept, minus `known.txt`. The tick the block first parted is kept by the Rust side on its own.

Exit: 0 in step and no unknown path · 1 the block or the die parted, or a kept tick has a path that is not known · 2 the Rust tick panicked · 3 a side did not run.

`--no-ts` runs the Rust side again over the TypeScript side `--out` already holds (the same ticks kept): the loop while you mend is a build and a few seconds.
`--open "K"` also writes the state as tick K opens. `--after N` runs N more ticks after the block parts. `--blobs` keeps each kept Rust state (over 100 MB late
in a mission) after it is diffed.

## Reading the stop

**`PANIC in tick K`** with a `todo!()` or "not yet implemented": a function nobody wrote. The frames under it name it and who called it. Read the TypeScript it
names and write its body (below). `python3 tools/blitz/stubcalls.py` lists every such call in the crate, so you can see the ones ahead of you.

**`THE BLOCK FIRST PARTED AT TICK K`**, with the parts that differ by name. The Rust side has kept its state of that tick; the TypeScript side keeps only
what it was asked, so run the whole command once more with `--keep "K" --blobs` added (not `--no-ts`). Then

```sh
rust/game_run/target/debug/where_parts <out>/sK.bin <out>/nsK.bin     # which words of which part, both values
bun run tools/blitz/diff.ts <out>/aK.bin <out>/nK.bin --show 40       # which paths of the game's own state (run.sh prints the first few; use --blobs to keep nK.bin)
```

The part `poses` is not the alarm: a dead man's ragdoll holds a pose the TypeScript keeps outside the pool and Rust keeps inside it, so from the first death
that part differs by design. The runner says when that happens and goes on comparing every other part.

**The die's count differs**: the two games have drawn a different number of times. Something upstream decided differently; the kept tick before it usually
shows a path.

**Paths that are NOT KNOWN at a kept tick**, the block still in step: state of the game's own that differs and has not yet moved a draw or the block. Chase
these too: they are tomorrow's parting. `diff.ts --dump '<path regex>'` prints every leaf under a path on both sides; `--why` counts the known lines that
matched; `--dynboxes` says which kinds of frame box one side has more of.

**Thousands of paths at once, most of them `scene_boxes`**: almost always ONE thing and its echo. The frame's boxes and several other tables are compared by
reach (the order one walk first meets a row), so a single box more on one side shifts every number after it. `diff.ts … --dynboxes` names the kinds of
frame box one side has more of; mend that and the rest goes. Two picture-only lanes the Rust game does not run put boxes in the frame's list (a holed
wall's paint, keyed `skin`; a broken edge's rubble, keyed `look`): the diff leaves those out of every list before comparing and says how many each side
had (`boxes left out as the picture's 36 / 0`). `--leave-out ""` compares them too.

**Is it the host?** Before you read Rust, check whether the TypeScript's own host wrote the difference between two ticks: run with `--open "K+1" --keep "K"` and

```sh
bun run tools/blitz/diff.ts <out>/aK.bin <out>/c(K+1).bin --known /dev/null --canon ""
```

shows what changed with no tick run. If your path is there, it is the host's half or the harness (below), not a rule.

`bun run --preload ./tools/blitz/expose.ts tools/blitz/timeline.ts <mission>:<seed>` prints what happens at which tick (orders, shots, men down), to know what
tick K was.

## The kinds of cause, and what to do for each

| kind | what it looks like | what to do |
|---|---|---|
| **a stub nobody owned** | a panic on `todo!()` in a `sig.rs` function | Write the body from the TypeScript the stub's doc line names, in a `b_NN.rs` of that module, and name it in `mod.rs`. |
| **a seam left in a part** | a panic on `todo!("needs: …")` inside a `p_NN.rs` | The translator could not say something with the types it had. Read what it needs; often the answer is a line (a constant for a native game, a call into `::sim`). If it truly needs a new type or field, that is an ask (below). |
| **dropped as picture** | no panic; a path differs, or boxes are short, where the TypeScript has a branch the Rust has not | A branch was left out as "the picture's" but the tick reads what it wrote (a spark's box in the frame list, a light, the copy of the box world a rule stood). Translate the branch. |
| **a wrong boundary no-op** | a `BOUNDARY NO-OP` body returns nothing where the game needed an answer | Read the TypeScript: if game state or a later decision depends on it, translate it; if it is truly the renderer's, leave it and say why in `known.txt` if a path shows. |
| **translation** | values differ in a rule both sides run | Read both, line by line. Usual suspects: an early return, the order of two writes, a copy where the TypeScript shares one object, `Math.fround`, a default argument. |
| **the host's half** | the TypeScript changes between two ticks with no tick run | The host's code is not translated. The runner stands in for it (list below). A new one is a stand-in in the runner, marked `HOST STEP`, and an ask. |
| **bridge** | a path differs at tick 1, or a field is blank after the import | The export or import does not carry it. `not-carried.json` says what is left out and why. A stand-in in the runner, and an ask. |
| **skeleton** | the Rust types cannot say what the TypeScript holds | Never edit `types.rs`, `sig.rs`, `io.rs`, `knobs.rs`, `ids.rs`: a program writes them. Work round it in your part if you can, and log the ask. |
| **harness** | serials, memo tables, counters, row numbers | The two sides number things differently by design, or the test driver's own question wrote the game. A `known.txt` line with its reason. |

## Mending

- The Rust bodies live in `rust/game/src/<module>/p_NN.rs` (a translated part) and `b_NN.rs` (bodies added during bring-up). Mend them in place.
- **A new `pub fn` that fills a skeleton stub must be named in that module's `mod.rs`**: `pub use b_01::{…, your_fn};`. A named re-export wins over the glob of
  `sig.rs`'s `todo!()` stubs; a body that is not named is never called and the stub still panics. `stubcalls.py` prints such bodies by name.
- Read the TypeScript before every mend, and say in a comment why the Rust is as it is where it is not a plain copy.
- `cargo build` warnings are many and not yours; errors are. One cargo build at a time on a shared machine.
- A difference you leave standing goes in `known.txt`: a regex of the paths, a tab, the kind and the reason. Take a line out when its cause is mended. A line
  with no reason is a bug hidden.

## Handing a mend back

A commit on your own branch that touches only `rust/game/src` and `tools/blitz/known.txt`, with a plain message that says which mission and tick it mends and
how far the mission then runs in step ("over N ticks, every part but `poses` hashed after each"). Say what you did not check. If you needed a new stand-in in
the runner or a change to the loop, say so separately: that is not part of a mend.

## What the runner stands in for

The Rust game is the tick alone. What the TypeScript's host does round a tick is not translated (`BRIDGE.md`, "What a Rust host calls round a step").
Each of these is a few lines of `rust/game_run/src/bin/mission.rs`; `BLITZ_SKIP=lights` or `upload` leaves that call out, to see by a run what it is for.

| stand-in | name | why |
|---|---|---|
| the record `cmd_buf`, every camera and Input row that is no seat's | (always) | the host samples input and moves its camera between ticks |
| the test driver's orders (`gives.txt`) | (always) | the scripted player calls `orders.give` between ticks |
| the driver's `roomAt` that floods a room (`hostwrites.txt room_at`) | (always) | a question of the driver's that writes the game |
| the box world listed again | `upload` | where the headless host uploaded between two ticks |
| the autosave's chronicle row (`hostwrites.txt chron_save`) | (always) | the save controller is the host's and writes one row into the game |
| `light_set_update(dt)` after each tick | `lights` | the host retires flashes between ticks, and the next tick's light reads it |

The Rust game writes a man's row of the block itself (`sim_men_store`), and the bridge carries the level's name, the dress table and the floats of a pose
outside the pool: the runner hands none of those over.

## Files

- `run.sh` the one command · `mission.ts` the TypeScript side · `host.ts` the bridge's writers for one part of a game (the host's half)
- `diff.ts` two whole states by path, in the file's memory, tables numbered by reach (`canon.txt` names the tables) · `known.txt` the known differences
- `timeline.ts` a mission's events by tick · `stubcalls.py` calls that land on a stub
- the bridge, written by the skeleton's program (`BRIDGE.md`): `export.ts`, `expose.ts`, `paths.ts`, `schema.json`, `not-carried.json`, `bridge-check.ts`,
  `dump-tables.ts`
- `rust/game_run/src/bin/mission.rs` the runner · `where_parts.rs` where two blocks differ

## Limits

- **No GPU, no picture.** The headless game has no renderer, no GPU light meter, no smoke solver and no audio; the compare is of the simulation's state only.
- The scripted player cannot be hurt and plays one way a seed. A mission in step is that play-through in step, not the mission's every branch.
- The scripted player plays the room clears, the rescues and the raids (`tools/qa/playthrough.ts`); it has no quiet mission.
- The runner cannot yet give an order aimed at a wall or a thing, or with options: it says `NOTE … cannot give` and the sides will part there.
- Not translated, and so not compared: what stands of a holed wall's paint (`wallskin`) and a broken edge's look (`edge_look`). See `--leave-out` above.
- A pose's floats are compared only through what the pose drives (bones, boxes); the pool itself is out of the alarm after the first death.
- A level script's beat holds its place in a generator, which no export can read: a state taken while a beat runs is refused by the import. A tutorial is
  compared from a start taken before its program starts, by what its beats do.
