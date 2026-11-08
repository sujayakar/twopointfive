# tools/blitz — the mission loop

The game's simulation exists twice: the TypeScript under `src/` (the game as it ships) and a Rust translation of it in `rust/game`. This folder holds the state
bridge between the two and the loop that plays a whole mission on both and stops where they part.

**The TypeScript is the truth.** The loop never edits it and neither do you. Every difference is mended in `rust/game/src`, or named in `known.txt` with its
reason. Never mend the TypeScript, the bridge, or the loop to make a difference go away.

## The one command

```sh
tools/blitz/run.sh <mission>:<seed> [--ticks N] [--keep "1,2,100-110"] [--every N] [--wall] [--dress | --dress-rows "a,b"] [--case pie|orbit|boot]
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

Exit: 0 in step and no unknown path · 1 the block or the die parted, or a kept tick has a path that is not known · 2 the Rust tick panicked · 3 a side did not run ·
4 the game parted at one of the loop's intended differences and nowhere before it (below).

`--no-ts` runs the Rust side again over the TypeScript side `--out` already holds (the same ticks kept): the loop while you mend is a build and a few seconds.
`--open "K"` also writes the state as tick K opens. `--after N` runs N more ticks after the block parts. `--blobs` keeps each kept Rust state after
it is diffed.

### Another die

`--die N` seeds the game's die with N on both sides (mission.ts's own is 20942; the start the Rust side imports carries the die). `--die tool` stands the
game as a tally's run of that mission and seed stands it (tools/qa/playthrough.ts under the harness's word: one draw of the scripted player's seeded
`Math.random` for the die, PLAY.md), so the game played is the tally's own; the die is written to `die.txt`. A mission in step on one die is one
play-through in step: a sweep takes each mission on several dice.

THE DRIVER'S ORDERED RECORD. mission.ts writes `between.txt`: every order (`<k> give …`), every write made before a tick (`<k> write …`), every rule
the driver called (`<k> call <rule>`) and the meter (`<k> meter <v>`), in the order the driver made them. The native runner replays a tick's lines as
they stand (game_run `between`) and reads gives.txt, hostwrites.txt (but for the autosave's row and a checkpoint put back) and meter.txt only where
there is no such file (a game of two seats; a stepper that has not learned the record). `BLITZ_NO_BETWEEN=<kind or rule,…>` leaves lines out, to see
by a run what one is for. A rule called under BLITZ_CALL is a `call` line too; its tick may be a word (`fight`: the first tick after a round was fired
with a hostile alert; `qrf`: the response team's cast deployed; `down`: the player half a second down, with PT_MORTAL=1), and the rule `shift_enter`
presses the player's own restart on the host's Input (each side then restarts inside its own tick; `enter`: Enter alone, the down card's);
`restart` calls `Game.restart`. `reset_guards` is `Game.resetGuards` on both sides (the Game's own, with every module's reset; the runner
called the guards module's inner function until this was found). Several calls in one run: a `;` between them. `qrf_stage` stages the response team
as tools/qa/qrf-squad-check.ts does (the sandbox's rows, the alarm, the request); `qrfcast` is the tick its cast stands. A restart INSIDE a tick makes the chronicle anew: mission.ts takes a save or a load only from a count that moved on
the same chronicle. A game of two seats takes BLITZ_CALL by number (seats.ts).

`BLITZ_CALL="<k> restart_encounter|reset_guards [start]"` in front of run.sh calls that rule before tick k on both sides (with `start` the Rust game
takes the TypeScript's state after the call as a new start instead of calling its own rule).

### The difficulty, a lever, a quiet mission

`--hard` stands the player's difficulty at Hard; `--set '<json>'` hands the scripted player's own `--set` rows on (a lever alone:
`'{"DIFFICULTY":{"fog":"cones"}}'`, `{"ai":"hard"}`, `{"kit":"stuns"}`); `--lever told=guided|standard|bare` moves the fourth lever on the standing game
before the first tick (it is a lever only once the page's switches are up, so `--set` cannot name it).

A name `quiet-<plan>:<seed>` or `solo-<plan>:<seed>` (plan: walk, sneak, careful, rush, loud) is played by `tools/qa/solo-night.ts`'s scripted Sam, one
man through the input, mortal and seen: `quiet-` the level's quiet mission, `solo-` the plain office after dark. `BLITZ_SOLO="…"` hands that tool more of
its own options (`--level quiet-yard.json`, `--unseen`, `--god`, `--squad 1`, `--cap 200`); the office carries no quiet mission, so a `quiet-` run names
a level. `BLITZ_QUIETFOG=off|team|cones|own` is the quiet mission's rung of the fog (given as the default the bare link plays, held through the stand;
on the page the rung rides the link).

The scripted player's option words reach the Rust side as `call` lines of the record: `PT_BREAKER=sam` (the mains cut by hand, `set_blackout_hand`),
`PT_NV=1` (`nv_on`), `PT_NOISE=1` (`noise_shot x y z`). `PT_MORTAL=1` needs nothing handed over.

### What the save controller does inside a tick

The save controller is the host's. Two of its doings write the game and are handed over through `hostwrites.txt`:

- `<k> save_breath`: a save taken by CAPTURE (no team on the floor: the solo night, a quiet mission alone) asks every module for its part, and the
  whistle's part asks the player's breath, which makes its row. The runner makes the row (`write_after`).
- `<k> load_in_step`: the down card's bare Enter loaded a FILE save inside the tick. The Rust tick's store is a boundary that holds nothing, so it finds
  no checkpoint and restarts the mission. The runner does not compare that tick, says so, and takes the next tick's opening state as a new start.
  A run with this line has NOT shown the Rust game loading a save: it has shown the game in step before the load and after it.

## Kept runs

```sh
tools/blitz/keep.sh <a run's --out folder> runs/room-clear-four-1.tar.zst "tools/blitz/run.sh room-clear-four:1 --every 500"
tools/blitz/hold.sh runs/            # or one file; exit 0 = every run the same series after every tick
```

A run that was in step can be kept as one file (about 3 MB): the start, the host's half of every tick, the driver's record and `post.txt`, the die's
count and a hash of each part of the block after every tick. `hold.sh` steps the Rust game through it again on the runner alone and compares after every
tick, with no TypeScript side. It sees the block and the die, not the paths of the game's own state (those needed the TypeScript's whole states), and it
reads the start with the import as it is built.

A kept run outlives a change of the wire: its records, orders and series are not bound to it, only the start's bytes and the host's half of a
tick are. `keep.sh` stamps a run with the wire's layout (`layout.txt`); where `hold.sh` meets a run of another layout, or of none, it stands the
start again from this tree by the run's own command (`mission.ts` or `seats.ts --start-only` under `TPS_GAME=none`: the TypeScript start alone, no
tick), plays the kept records on it and holds the kept series, and its line says so. Equal means the play did not change with nothing but the
start made again; a parting at tick 1 means the start itself moved. What it cannot do is in the head of `hold.sh`.

## A game of two seats

```sh
tools/blitz/run.sh two-roomclear:9 --every 300 [--asks]     # also two-nightbreach:9, two-compound:3
```

A name `two-*` takes `seats.ts` for the TypeScript side: `tools/qa/seats-run.ts`'s `standTwo` (two seats, each driven by scripted hands), stepped by
`g.stepSeats(1/60, [record, record])`. The host's half of a tick is then a record a SEAT (h<k>.bin begins with the count), and the runner steps the Rust
game with `game_step_seats` on the same two records; the compare is the same (the die and every part of the block after every tick, paths at kept ticks).
The host lists the box world after every tick and updates the lamps, as `seats-run.ts`'s `tick` does. `--asks` prints where the TypeScript's tick asked the
bare Game for a seat's field (seats.ts SEAT_ASKS) and the first tick it did: from that tick a path under `g.m.guards.ground_track` is the TypeScript's
third row (the bare Game's own memo of "the player's" ground speed), which the Rust game does not keep.

### The co-op rules

The kinds of fault a game of two players had are held by the native tests: `cargo test --test coop` in rust/game_start (each player's pair through a locked
door; no more men than the picture draws; a burst and one player's goggles part no two machines). They stand their own scenes, so no recorded run goes
stale under them; the kept runs and the check that read them are deleted.

## Intended differences

The TypeScript is frozen and has faults a game of two seats shows. Where the Rust game copies such a fault it stays in step, and that is right for
now. (The nets of `slingTick` and `hammerOutTick` and the budgets that count one player were such faults until 2026-11-07: they are mended, and rust/game_start/tests/coop.rs holds them.) Where the Rust game is RIGHT BY
CONSTRUCTION and the TypeScript is not, the Rust stays right and the loop parts there on purpose. `run.sh` names such a game (exit 4) when it first parts
at the tick written in its `intended` list and nowhere before. One so far:

| game | from tick | what |
|---|---|---|
| `two-full:3` | 5499 | `doorpath.ts own()` asks `this.g.orders.of(gd)` on the bare Game, which in a game of two is the FIRST seat's order book. For a man of seat 1 sent through a door with corners left on his order it finds no order, leaves the way round on the order, and the walk stands "off this order". The Rust door walk is started from that seat's door tools, inside the seat's view, and finds his own order: it takes the way round (`walks{…}.via` 2 against 0). Seventeen paths at 5499; the block (`doors`) parts at 5781. A game of one seat is not touched. |

## Reading the stop

**`PANIC in tick K`** with a `todo!()` or "not yet implemented": a function nobody wrote. The frames under it name it and who called it. Read the TypeScript it
names and write its body (below).

**`THE BLOCK FIRST PARTED AT TICK K`**, with the parts that differ by name. The Rust side has kept its state of that tick; the TypeScript side keeps only
what it was asked, so run the whole command once more with `--keep "K" --blobs` added (not `--no-ts`). Then

```sh
rust/game_run/target/debug/where_parts <out>/sK.bin <out>/nsK.bin     # which words of which part, both values
bun run tools/blitz/diff.ts <out>/aK.bin <out>/nK.bin --show 40       # which paths of the game's own state (run.sh prints the first few; use --blobs to keep nK.bin)
```

The part `poses` is in the alarm like every other part: the pool is tended as a tick opens (`posehome::pose_home_tend`), a ragdoll's pose and a function's
scratch pose are poses of their own outside the pool on both sides, and a man made inside a tick is lent the pool's end for his update
(`posehome::stage_open`). A `poses` parting is a difference to chase like any other; it usually means an animator was fed something else a tick earlier
(the kept tick before it shows `anim.*` paths). Which pose sets are at home and what each animator holds for its body's
place come over the bridge (`PoseSet.home`, `Anim.pos_in`).

**The die's count differs**: the two games have drawn a different number of times. Something upstream decided differently; the kept tick before it usually
shows a path.

**Paths that are NOT KNOWN at a kept tick**, the block still in step: state of the game's own that differs and has not yet moved a draw or the block. Chase
these too: they are tomorrow's parting. `diff.ts --dump '<path regex>'` prints every leaf under a path on both sides; `--why` counts the known lines that
matched; `--dynboxes` says which kinds of frame box one side has more of.

**Thousands of paths at once, most of them `scene_boxes`**: almost always ONE thing and its echo. The frame's boxes and several other tables are compared by
reach (the order one walk first meets a row), so a single box more on one side shifts every number after it. `diff.ts … --dynboxes` names the kinds of
frame box one side has more of; mend that and the rest goes. A holed wall's paint (keyed `skin`) and a broken edge's look (keyed `look`) are made
by the Rust tick too and compared box by box; `diff.ts --leave-out skin,look` leaves a family out of every list before comparing, to look past it.

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
| **a function nobody wrote** | a panic `no body: <module>::<name>` from a module's `missing.rs` | Write the body in a part of that module, and delete its line in `missing.rs` |
| **a seam left in a part** | a panic on `todo!("needs: …")` inside a `p_NN.rs` | The translator could not say something with the types it had. Read what it needs; often the answer is a line (a constant for a native game, a call into `::sim`). If it truly needs a new type or field, that is an ask (below). |
| **dropped as picture** | no panic; a path differs, or boxes are short, where the TypeScript has a branch the Rust has not | A branch was left out as "the picture's" but the tick reads what it wrote (a spark's box in the frame list, a light, the copy of the box world a rule stood). Translate the branch. |
| **a wrong boundary no-op** | a `BOUNDARY NO-OP` body returns nothing where the game needed an answer | Read the TypeScript: if game state or a later decision depends on it, translate it; if it is truly the renderer's, leave it and say why in `known.txt` if a path shows. |
| **translation** | values differ in a rule both sides run | Read both, line by line. Usual suspects: an early return, the order of two writes, a copy where the TypeScript shares one object, `Math.fround`, a default argument. |
| **the host's half** | the TypeScript changes between two ticks with no tick run | The host's code is not translated. The runner stands in for it (list below). A new one is a stand-in in the runner, marked `HOST STEP`, and an ask. |
| **bridge** | a path differs at tick 1, or a field is blank after the import | The export or import does not carry it. `not-carried.json` says what is left out and why. A stand-in in the runner, and an ask. |
| **skeleton** | the Rust types cannot say what the TypeScript holds | Edit the Rust: `types.rs`, `knobs.rs` and `ids.rs` are plain source. What the page reads goes by `tools/wire/wire.ts`. |
| **harness** | serials, memo tables, counters, row numbers | The two sides number things differently by design, or the test driver's own question wrote the game. A `known.txt` line with its reason. |

## Mending

- The Rust bodies live in `rust/game/src/<module>/p_NN.rs` (a translated part) and `b_NN.rs` (bodies added during bring-up). Mend them in place.
- **A new `pub fn` that fills a skeleton stub must be named in that module's `mod.rs`**: `pub use b_01::{…, your_fn};`. A named re-export wins over the glob of
- Read the TypeScript before every mend, and say in a comment why the Rust is as it is where it is not a plain copy.
- `cargo build` warnings are many and not yours; errors are. One cargo build at a time on a shared machine.
- A difference you leave standing goes in `known.txt`: a regex of the paths, a tab, the kind and the reason. Take a line out when its cause is mended. A line
  with no reason is a bug hidden.

## Handing a mend back

A commit on your own branch that touches only `rust/game/src` and `tools/blitz/known.txt`, with a plain message that says which mission and tick it mends and
how far the mission then runs in step ("over N ticks, every part hashed after each"). Say what you did not check. If you needed a new stand-in in
the runner or a change to the loop, say so separately: that is not part of a mend.

## What the runner stands in for

The Rust game is the tick alone. What the TypeScript's host does round a tick is not translated (`BRIDGE.md`, "What a Rust host calls round a step").
Each of these is a few lines of `rust/game_run/src/bin/mission.rs`; `BLITZ_SKIP=lights` leaves that call out, to see by a run what it is for.

| stand-in | name | why |
|---|---|---|
| a record a seat, every camera and Input row that is no seat's (`io::take_host_half`) | (always) | the host samples input and moves its camera between ticks |
| the commander's own wall charge (`gives.txt !wall_carried`, `!wall_raised`, with `--wall`) | (always) | the scripted commander gives itself a charge where the kit issued none, between ticks and before the order on the wall |
| a scripted case's write (`gives.txt !path_new`, `!path_goal_same`, `!f`, with `--case`) | (always) | a write the seeds never make, made on both sides (below) |
| the test driver's orders (`gives.txt`) | (always) | the scripted player calls `orders.give` between ticks |
| the driver's `roomAt` that floods a room (`hostwrites.txt room_at`) | (always) | a question of the driver's that writes the game |
| the commander's blow of a set wall charge (`hostwrites.txt wall_blow`, with `--wall`) | (always) | the scripted commander calls the module's `blow` between ticks |
| `rails_host_view` after each tick | `view` | on a tutorial mission the host sets the first view again with its own camera (game.ts hostAfter) |
| a tutorial's authored checkpoint (`chron_save` on rails) | (always) | the save controller stops the level's program for the take; the frame starts it again |
| a lesson player's meter (`meter.txt`) and `selectVerb` (`hostwrites.txt select_verb`) | (always) | the scripted player of a tutorial feeds the player's light meter and picks a row between ticks |
| the scripted player's word to the hostages (`hostwrites.txt rescue_word`) | (always) | the driver calls the module's own `word` between ticks |
| the scripted player's pick of a row of the nearby list (`hostwrites.txt near_sel`) | (always) | the driver writes `player.nearSel` and calls `pickHover` between ticks |
| the scripted player's ask before every step (`between.txt`: `call frag_step_off`) | `BLITZ_NO_BETWEEN=frag_step_off` leaves it out | the driver asks where a player who read a grenade's cue would go (`fragStepOff`); the ask makes the flee's guess of the grenade's rest and fills the memo of walls its burst opens, which the next tick's flee reads |
| a mission player's meter (`meter.txt`, every mission) | (always) | the scripted player writes `player.visibility` before every tick |
| the autosave's chronicle row (`hostwrites.txt chron_save`) | (always) | the save controller is the host's and writes one row into the game |
| `game_follow_pos` for each seat after each tick | `follow` | the host's camera asks the game where to look; on a tutorial's sky seat the ask eases and stamps the view's centre (rails.ts look) |
| a checkpoint PUT BACK (`hostwrites.txt chron_load`, the state in `L<k>.bin`) | (always) | the host's save controller replaces the whole game between two ticks; the runner takes the TypeScript's state as the next tick opens as a new start, and says so |
| `light_set_update(dt)` after each tick | `lights` | the host retires flashes between ticks, and the next tick's light reads it |

The Rust game writes a man's row of the block itself (`sim_men_store`), and the bridge carries the level's name, the dress table and the floats of a pose
outside the pool: the runner hands none of those over.

## The dressing rows up (`--dress`)

Without a word both sides stand with every row of `DRESS` (src/game/dress.ts) down: the stand of a host that draws nothing. `--dress` raises all 18 rows
on both sides from the first tick, as the page has them (`--dress-rows "moths,wind"` raises only those); the bridge carries the table, so the Rust tick
reads the same rows. Every row CAN be raised on this stand. What its step then does with no GPU is less than in the page: there is no particle pool
(`engine.particles` is made only for a recording that had one, tools/qa/headless.ts), no smoke solver and no renderer, so a step's particles, its
pushes on the solver and its draws go nowhere and only what it writes into the game's own state is compared.

The rule the split follows: a dressing body that writes something the tick later reads runs IN THE RUST TICK under its row (the moths' and a broken
fixture's keyed boxes, which the fog of war remembers; a man's drawn scale on his own boxes; the wind the moths read; the cues' and the net's books,
which the rules' own entries write; the spent cases' list). One that writes only what the picture reads stays in the TypeScript dress step
(src/host/rustdress.ts) and the Rust game does not write it: those are the lines of `known-dress.txt`, read with `known.txt` when the word is given.
A known line may carry a third column that narrows it by the two values (`bits:128`, `ts:<regex>`, `rs:<regex>`: diff.ts).

## Scripted cases (`--case`) and the counters (`TAPS`)

The runner's last lines include `TAPS name count · …`: counters of the Rust process (`rust/game/src/game/b_tap.rs`; no rule reads them, nothing carries
them) at places that were mended by reading, to say whether a run reaches them. `--case pie | orbit | boot` makes one write between two ticks that no
seed makes, on both sides (mission.ts `caseStep`; a line of gives.txt and of case.txt); `BLITZ_CASE_RAW=1` in front of a `--no-ts` run makes the Rust
side's write as it was before the mend.

Two more cases reach reads that were restored by reading (their counters are `digin_keeper_*` and `sticky_eyes_cone*`):

- `--case keeper` (with `--hard`): a man a press left to keep his room, idle and out of contact, hears a fight outside it; with DIGIN.keeper off he lets
  it lie (digin.ts:705). No scripted play on Hard leaves such a man (a far room goes together, a stood-to floor keeps nobody), so `--case keeper-left`
  first makes the press's own two writes for one defender who has a mate of his room up (pressure.ts:668, :698).
- `--case sticky`: the player's tube sends up to three sticky cameras over the nearest hostile (stickycam.ts fireCam). Under the fog's cone a camera's
  wedge is no wider than an eye's (stickycam.ts:195); at the page's numbers the cone (120) is wider than the camera's own 110, so the read only counts
  with a number moved: `--set '{"DIFFICULTY":{"level":"hard"},"FOG":{"coneDeg":40}}'`.

## Files

- `run.sh` the one command · `mission.ts` the TypeScript side (`seats.ts` for a game of two seats) · `host.ts` the bridge's writers for one part of a game (the host's half)
- `diff.ts` two whole states by path, in the file's memory, tables numbered by reach (`canon.txt` names the tables) · `known.txt` the known differences (`known-dress.txt` with the dressing rows up)
- `timeline.ts` a mission's events by tick
- the bridge, written by the skeleton's program (`BRIDGE.md`): `export.ts`, `expose.ts`, `paths.ts`, `schema.json`, `not-carried.json`, `bridge-check.ts`,
  `dump-tables.ts`
- `rust/game_run/src/bin/mission.rs` the runner · `where_parts.rs` where two blocks differ

## Limits

- **No GPU, no picture.** The headless game has no renderer, no GPU light meter, no smoke solver and no audio; the compare is of the simulation's state only.
- The scripted player cannot be hurt and plays one way a seed. A mission in step is that play-through in step, not the mission's every branch.
- The scripted player plays the room clears, the rescues and the raids (`tools/qa/playthrough.ts`); it has no quiet mission.
- The runner gives an order on a wall (`--wall`: the scripted commander takes a held room by a wall charge, and its blow between ticks is replayed) but not one
  on a thing, or with options: it says `NOTE … cannot give` and the sides will part there.
- A holed wall's paint and a broken edge's look are compared by their BOXES, not by their books: the bridge does not carry `WallBreak.skins` or the
  look's module state, so the Rust game makes its paint's books on first use from the boxes as authored (`wallskin::ensure`, a stand-in).
- A pose of its own outside the pool (a ragdoll's) is compared by its floats and by what it drives; its row number is a known difference.
- A tutorial (`tut-<name>:<seed>`) is played by `tools/qa/lesson-run.ts` WITHOUT its model of the sky's light on every man's meter (a hook over the game's
  light asks, which the Rust game has no word for): it is the tutorial as the headless estimate lights it.
- A kept Rust state is small again (about 8 MB): a box pushed straight onto the frame's list is a row for that frame, freed when the host lists the box world.
  Makers still on a row for good: the launcher's rings, the wall charge's look, a thrown piece of furniture, a wall's falling piece.
- A level script's beat holds its place in a generator, which no export can read: a state taken while a beat runs is refused by the import. A tutorial is
  compared from a start taken before its program starts, by what its beats do.
