# tools/blitz — the headless host: the scripted player plays the Rust game

`rust-host.ts` makes a TypeScript `Game` a PUPPET of a game that steps in the wasm face (rust/game_face). From the first tick **the Rust game is the only
game that steps**: `Game.update`'s own lines run as they are (the record sampled, hostBefore, hostAfter), the step between them is the host's, and after
it the Rust game's state is applied onto the TypeScript objects in place. Whatever reads the game (the scripted player of tools/qa, a tally, a probe)
reads what the Rust game holds, and what it presses goes to the Rust game as the tick's record. `stepSeats`' body never runs.

```sh
tools/blitz/play.sh room-clear-four:1                  # the plain TypeScript play first (the reference), the face built, then the play, every tick compared
tools/blitz/play.sh <mission>:<seed> --no-build -- --audit --ticks 600
bun run --preload ./tools/blitz/expose.ts tools/blitz/play.ts <mission>:<seed> --ref DIR [--out DIR] [--ticks N] [--audit] [--whole-every N] [--stale N] [--go-on] [--quiet]
tools/blitz/rust.sh rust bun run tools/qa/playthrough.ts --scn depot-four --seeds 1-4 --json rust.json     # any headless tool, on the Rust game
```

| file | |
|---|---|
| `rust-host.ts` | the host: the module, a game's start, the step, the two applies, the driver's calls, the out events, the dress step. `RustHost.attach(game, opts)` |
| `play.ts` | a mission stood as `tools/qa/playthrough.ts` stands it (a tutorial: `lesson-run.ts`), played on the host, HELD TO THE PLAIN TYPESCRIPT PLAY every tick |
| `audit.ts` | the walks that compare two whole states field by field (the audit's, and `--stale`'s) |
| `rust.sh` | the harness's word and the bridge's preload in front of any command |
| `tally-diff.ts` | two `playthrough.ts --json` files side by side, a run a row |
| `thrice.ts` | one fight stood N times in one process, every game held to the first (the digest a second; `--ticks`, `--dump` to find where two part) |
| `word.ts` | THE ONE SWITCH: the word asked for (`TPS_GAME`, else `DEFAULT_WORD`), the table, a frozen run, the closing line. No import of src |
| `preload.ts` | bunfig.toml's preload: nothing unless the word asks; then the bridge's window (`expose.ts`) and the top process's closing line |
| `qa-kinds.json` | the table `TPS_GAME=table` reads |
| `table-check.ts` | the table held to `probes.ts list` (by hand): red on a name the table lacks or a row probes.ts lost |
| `die-check.ts` | the game's die seeded after the stand, the helper and the plain site, a line a word |
| `rust/game_face/src/harness.rs` | the face's exports a host that plays needs: the filtered export with its memo, the camera's ask, a flooded room, the program stopped before a take |

## Two ways

**Fast (the default).** After a step the FILTERED export is applied (`face_puppet` → `applyGame(… puppet …)`: the listed fields of what changed, and
the step's out events on its tail). Nothing is ever exported from the puppet. What the TypeScript round a step does to the game goes to the Rust game
as a call when it is made:

| what the TypeScript does between two ticks | the call |
|---|---|
| the scripted player gives an order (`orders.give`) | `face_give` where the face has words for its target (the floor, a door, a man by place); the menu's own `give` on the puppet otherwise, whose rows ask the Rust game (`face_ask`) |
| a lesson's player picks a row of the nearby list (`selectVerb`) | `face_hostwrite … select_verb` |
| a lesson's player feeds the light meter (`player.visibility`) | `face_meter`, when the number differs from what the last apply left |
| the autosave takes a checkpoint (the dress step's `saves.tick`) | `face_checkpoint`; an authored one on a level on rails: `face_stop_program` first |
| a question floods a room (`fight.roomAt`) | `face_room_at`, asked of the Rust game FIRST; where it flooded, the puppet takes the whole state and the TypeScript's own ask floods nothing |
| the host lists the box world (`engine.world.upload`) | `face_upload`, and the TypeScript's own listing runs too (the list of every box and the ray grid are the TypeScript's own on the filtered path) |
| the host's camera asks the game where to look | `face_follow` after every step |
| a checkpoint put back, a restart (the TypeScript puts another game in the puppet's place) | A NEW START: the puppet exported whole once, a new Rust game made from it |

Memos a driver's read fills on the puppet (`Cover.openings`, `DialogueMod.room_rects`, `WorkThings.cache`, frag.ts's `walls_for`) are NOT handed over.
Each is "built if it is not yet" in its Rust body (cover/p_02.rs:69, dialogue/p_02.rs:146, workthings p_*.rs:180 to 213, frag/p_01.rs:124), and the
replay loop (`run.sh`, `wasm-run.ts`) has never handed them and is in step on every mission. `doorTools.knownShut` also writes when asked (`learn(d,
side, null)`: it forgets a door found open): not handed either, same ground.

A WHOLE apply (`face_export` → `applyGame`) is made where something is about to read what the filtered list is known to lack: when a save is taken
(the save keeps the puppet's objects), when a man goes down, a door order begins or ends, the mission's stage moves or one of the ends the scripted
player asks for is set, after a room is flooded, and every `--whole-every N` ticks if asked. About 5 to 25 a mission.

**Audit (`--audit`).** Beside the filtered apply the whole state is applied every tick, and as each tick opens the puppet is exported whole and held to
the bytes last applied. What differs outside the host's half is something TypeScript wrote into the game between two ticks: named, counted, and handed
to the Rust game whole (`face_import_into`). Two walks of the state a tick (about 200 to 350 ms): the way to find a write the fast way has no call for.
`--handback N` hands the whole puppet over every N ticks whether anything was written or not (`HB_AT=<ticks>`, `HB_DEBUG=<dir>` keeps the Rust game's own
dump before and after). `--stale N` (fast): every N ticks the puppet is exported whole and held to the Rust game's whole state, and the fields in which
the puppet has fallen behind are listed by name (`--stale-from K`, `--stale-show N`): the way to name a field the filtered list lacks.

## What is compared, and against what

The reference is the same mission and seed played by the plain TypeScript game (`mission.ts`). `play.ts` holds to it, every tick:

- **the host's half** as the tick opens: the very bytes handed to the Rust game (`hostHalf`: a record a seat, every camera and Input row) against the
  reference's `h<k>.bin`. The first tick it differs is the first tick the puppet showed the player something the TypeScript game would not have;
- **the driver's calls** made since the last tick (orders, checkpoints, floods, picks), in the replay loop's own words, against the reference's
  `gives.txt` and `hostwrites.txt`. An order is no key: warehouse-four's die parted 17 ticks after an order the plain play gave and this play did not,
  with the host's half equal throughout;
- **the die and the block** as the tick returns (the 12 parts but `poses`, which parts by design from the first death: README.md);
- **the end**: taken / failed, the second, who is up, the tick.

A run whose host's half was compared on fewer ticks than were played exits 3 ("NOT A PASS"): the compare cannot die silently. A run with no `--ref`
says NOTHING COMPARED and exits 3. Exit: 0 equal to the end and ended the same way · 1 parted · 2 the Rust tick panicked · 3 a side could not run ·
4 equal every tick, the end differs.

## A tick, the fast way

1. The scripted player presses its keys and steps its world (`headless.ts W.step` → `Game.update`); its own lines run, the step is `RustHost.step`.
2. The light meter, if the lesson's player wrote it. The check's hook (the driver's calls and the host's half held to the reference).
3. `hostHalf` → `face_host_half`. `face_step`. The check's hook (the die, the block's hashes). `face_lights`, `face_view`, `face_follow`.
4. `face_puppet(0)` → the filtered apply; the block copied; `asks.after()` (a row or a posture the apply made gets the closures that ask the Rust game).
5. A whole apply if one is owed. The out events dispatched (`out-dispatch.ts`) on the puppet's host objects, in the order the rules made them.
6. The dress step: `Par.tick`, `Verdict.tick` (each in a `try`: they are the debrief's), the autosave (`saves.tick`), whose checkpoint is made on the
   Rust side.

Before the first tick the start is exported whole once (the Rust game is made from it), the filtered export's memo is filled, and the every-tick path
is run four times over every row (`face_puppet_warm`) so that the first ticks do not pay for its compiling.

## Out events

Dispatched every tick since this host. The arm that CHANGES THE GAME is the checkpoint: `CheckpointsKeep` (a rule on a level on rails asks for an
authored checkpoint) and `CheckpointsRequest` set the save controller's own flags, its tick in the dress step takes the checkpoint off the puppet (its
snapshot in memory, its chronicle row), and the host makes the same on the Rust side. For an authored one the level's program is stopped on the Rust
side BEFORE the take (`face_stop_program`), so that the controller finds no program running: a director run the apply made has none of the TypeScript's
own (director.ts:525 threw on `R.drills`). tut-solo's four authored checkpoints are taken on the reference's ticks with its labels.
`HostApiRestartPicture` and `SessionRecorderCut` go to the host's picture and journal and write no game state.
The harness's stand-ins lack some receivers (`audio.rain`, `smoke.obstaclesChanged`), which the generated dispatcher calls unguarded: such an arm is
counted and not called again. `WallWearRound` threw (`box.h` undefined: the event's box came back as a row the every-tick apply had made on the puppet and not filled); with the changed-only export the row is whole, the same play (night-breach, die 20942, tick 1293) dispatches it, and no arm threw in 84 + 42 missions (`RH_OUT_SAY=1` says the first throw of any arm with its event).
an object).

## The harness's word

`TPS_GAME` in front of any headless tool run from the repository's root (`tools/blitz/rust.sh <word> <command>` is the same thing from anywhere). The
bridge's preload is listed in bunfig.toml and does nothing unless the word asks (below), so the word alone is enough, for the tool and every child `bun`
it starts in the root. ONE FILE decides which game a `standUp` stands: tools/blitz/word.ts.

| | |
|---|---|
| `TPS_GAME=rust` | the game `tools/qa/headless.ts standUp` returns is a puppet of the Rust game (the fast way) |
| `TPS_GAME=audit` | the same, audited |
| `TPS_GAME=ts` | the SAME STAND stepped by the TypeScript tick: the other side of a tally |
| `TPS_GAME=table` | the Rust game, but for the checks `qa-kinds.json` keeps on the TypeScript tick (below) |
| `TPS_GAME=none` | nothing of this is loaded: the harness as it always was |
| unset | `DEFAULT_WORD` of word.ts: `'none'` today. THE FLIP is that one line, to `'table'` |

The stand under `rust`, `audit` and `ts` is mission.ts's: the game rolls a die of its own where the tool gave it none (seeded off the tool's `Math.random`
as the game is stood, so `playthrough.ts`'s seeds roll different dice and both words roll the same), and the dress rows are down. So a
tally under `ts` is not the tally the tool makes with no word (which rolls `Math.random` and dresses); it is the one to hold the Rust game's beside.
(`TPS_RUST` cannot be the word: src/sim/sim.ts refuses it as a retired one.) `TPS_GAME_WHOLE=N`: a whole apply every N ticks.

**The preload.** The bridge reads the simulation's own module variables through a window `expose.ts` opens as each source file loads, so it must be
registered before any source file is read. bunfig.toml lists `tools/blitz/preload.ts` for every `bun` started in the root; it imports word.ts (one small
file, no source file) and, only where a word asks, `expose.ts`. With no word, measured: `probes.ts sticky` the same output line for line (25.0 s without
the list, 26.9 s with, one run each on a loaded box); `PT_DIGEST=1 playthrough night-breach:1` the same digest and the same whole record; `bun run
tools/build.ts` the same 137 files to the byte without the list, with it, and with it ACTIVE (`TPS_GAME=table`); the dev server's `/` and its script
the same bytes the same three ways (the bundler does not run a runtime plugin). A child started outside the root, or under node, has no preload: under a
word its `standUp` throws and says so. (A bunfig given by `--config` from another folder resolves the server's own plugin against that folder.)

**A frozen run** (`TPS_PROFILE=frozen`) is the TypeScript tick's whatever the word: the frozen stand holds settled switches off their page values, and a
settled switch is a constant in the Rust game. word.ts forces it and the closing line says so (`ALL n games of this run were FORCED TO THE TYPESCRIPT TICK`).

**The closing line is the top process's.** A tally plays its games in children whose output goes nowhere. The preload of the first process of a run names
a file in the environment (`TPS_GAME_TALLY`); every process below adds its counts as it ends; the top one sums them into the one line.

**The word by a table (`TPS_GAME=table`).** `tools/blitz/qa-kinds.json` has a row a probe of `tools/qa/probes.ts` (its kind: what it does to the game
between two ticks, written by `qa-census.py --kinds`) and a row by hand for a stand-alone tool that cannot run on the Rust game yet, with its reason.
As a game is stood, the probe running then (the runner names it) or the tool's own file decides, in this order:

1. a probe with NO ROW (added since the census, or one the census found no line for: the table's `unread`): the TypeScript tick, and the closing line
   counts and names it (`N checks with no row: …`);
2. kinds iii and iv (a check that CALLS the tick's rules, hangs its own function on a game object, checks a TypeScript function with no game, or holds
   the game to a walk of the JavaScript objects): the TypeScript tick;
3. `pins`: a check that RAN WITH A SETTLED SWITCH OFF ITS PAGE VALUE (a row the Rust game folded to a constant), from a run of the gate's families with
   `RH_FOLDED`: 185 probes; the TypeScript tick;
4. `waits`: a check SEEN RED ON THE RUST GAME and green on the TypeScript tick, by hand, with what it waits on; the TypeScript tick until its row is taken out;
5. everything else (kinds i and ii that are in none of the lists): the Rust game;
6. a stand-alone tool by its row: `ts`, or `none` (stood with no word at all: `tools/qa/coop-check.ts`, whose second engine is node), else the Rust game.

Each check kept off the Rust game says so as its first game is stood (`[the game: <name> is stood on the TYPESCRIPT TICK by the table (kind iii); its
verdict is not the Rust game's]`) and the run ends on one line that counts both sides and names them (`=== the game, by the table: … ===`), so a green
of the old tick is not read as the Rust game's. HOW a kept check is stood is one constant of word.ts, `KEPT_STAND` (`TPS_KEPT=ts|none` for one run):
`'ts'` the word's own stand (the game owns its die, the dress rows down), `'none'` exactly as with no word.

`tools/blitz/table-check.ts` (by hand, after a probe is added, renamed or deleted) holds the table to `probes.ts list`: red when the runner lists a name
the table has no row for (as a probe, a family that only runs its members, or an unread name) or the table has a row, a pin or a wait of a name the
runner lost. The census on this tree: 1,512 names known; 1,258 probes read (iii 766, iv-a 186, ii 114, 0 83, iv-b 61, iv-c 39, i 9), 150 families,
105 unread. BY THE TABLE: 1,075 probes on the TypeScript tick, 101 on the Rust game (92 of kind ii, 9 of kind i) less the `waits`, 82 stand no game.

## The die

The game owns its die under every word, as the page's game does: no seam lets the Rust tick draw a function a tool put in `Math.random`'s place. (The
TypeScript rule "a die of the game's own is used only while `Math.random` is the function it was when the die was made", src/math/dice.ts, is the
TypeScript tick's and goes with it.) A tool that wants a given stream AFTER the game is stood says so through tools/qa/headless.ts:

- `seedGame(game, die, seed)`: with no word, `Math.random = die` and nothing else (what the site did); under a word, also the game's own die put at the
  start of `seed`'s stream, on the puppet (in place: the die's word and count may live in the game's block) and in the Rust game (`RustHost.seedDie`: the
  `dice` part of the block written in place; no new call of the face's). Returns the put-back.
- `dieBack(game, was)`: `Math.random = was`; under a word the game goes on with its own die.

`tools/blitz/die-check.ts` (two seconds of play, the stream named, two seconds of Sam's fire): with no word the helper's line is the plain site's; under
`ts` and `rust` the helper leaves the game's die at the same word after the same 18 rolls (ad0e9341); the plain site leaves 1 roll under `ts` (the tool's
function took 18) and 19 under `rust`. A seed put in BEFORE the stand needs no helper: the stand seeds the game's die off one draw of it.
The sites, by reading: 782 assignments in 349 files; 137 seed or function sites after a stand (and 137 put-backs); 28 of them in the 11 families that
have a probe on the Rust game by the table. They are not converted: the census and the codemod's plan are with the swap's notes.

## The gate's playing tools under each word (the changed-only export, the joined bodies; one at a time, the box at a load of 9 to 15)

| tool | `ts` | `rust` | what is red under `rust`, by kind |
|---|---|---|---|
| `playthrough.ts --seeds 1-4 --jobs 2` (84 runs) | 1,452 s | 2,498 s | 84 of 84 END THE SAME; 14 whole records equal; 7 places differ (`smells.#.detail` 68 runs, `destr.fixWhy` 11, `destr.skins.*` 7, `deaths.#.hole` 3, `entries.#.blind` 1): the tool's own watchers and wrappers |
| the digest of every half second (`PT_DIGEST`) | | | equal on 28 of 84; two fields part and no other in 11 missions looked at: `decals` (the host's decal list holds marks the out arm lays and nobody tags as the picture's) and `civilians[i].dice` (the same 32 bits, unsigned on the puppet) |
| `bench-seeds.ts --twice snow:10`, `dark:1` (the same run twice) | 45 s, 38 s PASS | 113 s, 84 s PASS | the same digest under both words; "Sam hit" is the tool's own tap (2 | 0) |
| `seats-run.ts` | 87 s PASS | 170 s PASS | |
| `seats-check.ts` | 319 s PASS | 477 s, 3 red | `rollback`, `twin`: a word made by walking the JavaScript objects (and `walkAhead` on the puppet); `down`: calls `hitPlayer` |
| `mpserver-check.ts` | 226 s, 2 red (clocks) | 443 s, 4 red | `same fight`, `late joiner`: the check runs on the wall clock and the games stepped 650 to 917 of 1,200 ticks; `lag`, `ahead` red under `ts` too |
| `tools/qa/coop-check.ts` | 209 s, 4 red | 520 s, 4 red | its second engine (node) has no preload under either word; under `rust` the tool's own die in `Math.random`'s place is not the Rust game's |
| `dice-check.ts` | 24 s, 1 red | 51 s, 2 red | `default`: the word gives the game a die (both words); `same seed`: provokes its fight by a rule |
| `knobs-check.ts` | PASS | PASS | stands no game |
| `tools/blitz/coop-check.ts` | 430 s, its 3 lines red as its header expects | | stands no game through the harness and reads no word |

## A tool that stands many games, and what it does with them

- **The second game of a process.** The simulation keeps much of its state in weak tables beside its objects, one table a process (src/math/side.ts), and
  the bridge listed such a table by every key still alive: a game stood after another was exported with the other game's men, routes and players as
  rows of its own, and played another game (seats-run's second game rolled its die one time fewer at tick 787). The other game need not have stepped
  (the game server stands one only to write its recipe). While the bridge walks a game a table now lists that game's keys only (the stand-in
  `ownKeys`, below). `tools/blitz/thrice.ts` stands one two-seat fight three times in a process and holds
  the digest of every second, and with `--ticks` every tick, to the first game's: three games, one digest, each equal to the `ts` game on all 2,400
  ticks; `--weave` stands the later games together and steps them in turn. The modules' own logs and tallies (`destr_log`, `holdfire_log`, `scene_collision.sensed` …) are still a process's, on both sides: they grow from
  game to game and no rule reads them.

- **The slots.** rust/sim has four. A tool that stands a fifth game in one process (the probe runner stands hundreds) gets the slot of the game stepped
  longest ago: that game's puppet takes the whole state first, and if the tool steps it again it starts anew from its puppet.
- **The host's hooks are hidden fields** (`stepSeats`, `rustHost`, the wrappers on the menu's `give`, `fight.roomAt`, `selectVerb`, the lamps' update, the
  listing, the saves): a walk of the game's own keys (rollback.ts `stateHash`) does not meet them. A start takes the hooks again: a mission the tool
  started after the stand has its own menu, fight and saves.
- **The trace.** `RH_TRACE=<file>`: a line a game as the process ends (the tool's file that stood it, the probe running then, ticks stepped, whole
  applies, calls, starts, slots given up, how often the game's die had rolled between two ticks: a rule of the tick run in TypeScript on the puppet).
  `RH_SITES=1` with it: every tick counted by the place in the tool that asked for it. `RH_CAP_AT=<ms>`: the tool ends itself at its next tick after that
  moment (a signal's handler waits for the tool's own synchronous code).
- **A quiet run of steps** (`W.steps(n, dt)` in tools/qa/headless.ts: the same as `W.step` n times, said by a tool that runs none of its own code
  between them). The host hands its half over, steps the Rust game and asks it the camera's two inputs (`face_look`: where to follow, the aim point);
  nothing is exported or applied until the run ends, when the puppet takes what changed over the whole run in one filtered apply, the run's out events
  are dispatched in order and the dress step runs once. What differs from the same steps one by one: an autosave due inside the run is taken as it
  ends, the par clock and the verdict tick once, an out event reaches the host's objects late. ASKED FOR BY NAME (`RH_QUIET=1`) AND NOT YET THE SAME
  PLAY: the Rust tick writes the lens's own row at times (a start snaps it onto the man) and a quiet run does not bring it across, so the host hands
  its own lens back. `tools/blitz/quiet-check.ts` (the office walked by input in six runs, 2,340 ticks): the die and the block equal on every tick
  under `ts`, quiet and with an apply every step; the host's half of the quiet run differs from tick 2 (`FollowCamera.target`); 3.4 ms a tick quiet
  against 8.5 with an apply every step. It wants the camera's rows brought across on the quiet path. `tools/blitz/quiet-loops.py` rewrites a loop that
  only steps into that call (the exact shape only: the counter from 0, a bound of plain arithmetic; a condition that reads the game is tool code).
- **Two plays of one tool held to each other tick by tick.** `RH_TICKS=<folder>` under `ts` and under `rust`: a line a tick (the die's count, the host's
  half as handed in, the orders given so far, a hash of each part of the block but `poses`). For a tool whose own stand `play.ts` cannot make again
  (a tally's run: the word seeds the game's die off the tool's own `Math.random`). `RH_DIE_SAY=<file>` writes that die; `RH_HALF=<ticks>` prints the
  host's half of those ticks a field a line.
- **What a check does to the game between two ticks**: `tools/blitz/qa-census.py` (a probe a row, by kind). `--folded <the switch list>` adds the checks
  that put a SETTLED switch (a constant in the Rust game) on its other side, by reading (FOLDED.md); `RH_FOLDED=<the switch list>` with `RH_TRACE` counts,
  as a check runs, the ticks each settled row stood off its page value (`folded` in the game's line): what the check really pins.
- **`RH_TIMES=<file>`**: as a tool ends, a line a game stood on the Rust game: its ticks, the clock from its first step to its last (the tool's own code
  between two steps included) and every ms the host spent, by what it did.
- **`RH_OUT_SAY=1`**: the first throw of each out event's arm with the event and where it threw. **`RH_OFF=own`**: the weak tables listed for every game again.

## Stand-ins (each marked `STAND-IN:` in the code with its reason and where it belongs)

- **A weak table lists the keys of the game being walked** (rust-host.ts `ownKeys`, `reach`). A start finds the game's own keys by reach: the game is
  exported on a book of its own with every weak table listing only the keys the walk has numbered so far (and the game and its views), again until an
  export numbers no key it had left out; the game's book is put back as it was and the start's export lists exactly those keys (three exports a start
  where it was one: 46 keys of a room clear are met late, through other rows). After a start: those keys, and a key first seen since unless a start
  left it out or it is in another hosted game's book. An apply deletes and orders only this game's keys. NOT COVERED: a game with no host that makes
  new things while a hosted game plays beside it. Belongs in the generated export.ts (`keysOf`, the once-keyed and game-keyed reads) and import.ts
  (`orderSide`, every `liveKeys()`).
- **A box whose markings went is `marks: null`** after an apply; scene/boxes.ts:841 tests `!== undefined`. Made undefined before the puppet's own listing
  (rust-host.ts). Belongs in import.ts.
- **The whole applies owed** (the list above): each stands for fields the filtered list lacks. Belongs on the generated list.
- **The scripted player's order by `face_give`** on the fast path: the menu's own `give` reads the puppet before it asks a row (orders.ts:2285, the floor
  the team has seen). Belongs on the generated list.
- **The memo's buffers trimmed** after every filtered export (harness.rs `trim`): `export_puppet` hands its scratch vector to the memo with whatever
  capacity it had. Belongs in the generated export.
- **`face_stop_program`** before an authored checkpoint's take (harness.rs): belongs with the save controller's port.

## What does not work on a puppet

- **A lesson's own checks** read records the filtered list lacks (`char.anim.crouch`, a man's torch, the team pace's own counters): see the table.
  `--whole-every 1` is the slow way round.
- **A module's own counters that the TypeScript tick bumped** (`DESTR_STATS`, `FRAG_STATS`, `TEAMPACE_STATS` … read by `playthrough.ts`'s report)
  stand at zero: the Rust game's counters are not the TypeScript module's tables. A tally's headline (how it ended, the second, who is up) is the
  game's; its counts of rounds, bangs, papers are not: see the tallies.
- **Two seats**: the first apply throws (below).
- **`tut-solo`'s last 510 ticks, `tut-kit` from 5302**: the cursor parts (below).

## Numbers

The fast way, release module, 4 cores. THE MACHINE WAS NEVER QUIET for the longer runs (a load of 8 to 12 from other work), so each column says its load.

| ms a tick, mean | room-clear-four:1, 3,006 ticks, load 9 | night-breach:1, 1,413 ticks, quiet | the audit before (room-clear-four) |
|---|---|---|---|
| the scripted player and its world's own lines | 0.87 | 0.50 | 0.86 |
| `Game.update`'s own lines | 0.06 | 0.03 | 0.03 |
| the write-back: the puppet's export · the compare · the hand-over | none | none | 125.1 · 10.6 · 0.8 |
| the host's half | 0.11 | 0.08 | 0.6 |
| `face_step` | 2.00 (worst 83, tick 1) | 1.10 (worst 19.5, tick 1) | 2.12 (worst 29.5, tick 1) |
| lights, view, follow | 0.005 | 0.004 | 0.01 |
| the filtered export (about 170 to 230 KB a time) | 1.78 | 0.93 | none |
| the filtered apply | 4.19 | 1.56 | none |
| the whole export and apply, spread over the play (5 to 25 a mission, 100 to 280 ms each) | 0.07 + 0.51 (9 of them) | 0.03 + 0.25 (4) | 8.6 + 119.4 every tick |
| the block's copy | 0.31 | 0.16 | 2.1 |
| out events dispatched | 0.01 | 0.006 | not dispatched |
| the dress step | 0.25 | 0.15 | 0.23 |
| the driver's calls | 0.01 | 0.01 | in the write-back |
| **all in** | **10.4** | **5.0** | **273** |
| the check (not the host's): the host's half read and held · the die and the block's hashes | 1.36 · 0.68 | 0.14 · 0.52 | |
| wall for the mission, the check included | 37 s | 8.0 s | 822 s |

The start (the whole export, the Rust game made, the memo filled, four warm applies) is 300 to 700 ms once. The module stands at 160 to 175 MB for a whole
mission (pass-four passed 2 GB before the memo's buffers were trimmed). No step grew the memory once 48 MB is reserved before play.
THE WORST TICK is the first (about 20 ms quiet); a second game stepped three times from the same start before it does not lower it (19.8 against 19.5 ms),
so it is the game's own first-tick work, not the module's compiling (`warmSteps`, off). Under load the worst tick is whichever the scheduler takes.
A tutorial is the same path now (tut-solo: 5.1 ms a tick, 38.7 s for 7,542 ticks). `--whole-every 1` is 100 to 210 ms a tick under the load.

## What has been played

Every tick: the host's half and the driver's calls before it, the die's count and 12 parts of the block after it, all against the plain TypeScript play
of the same seed; then how it ended. On the generated cut 7.6, the fast way, seed 1. Walls are under a load of 8 to 12 on 4 cores, the plain play's
(`mission.ts`, which also writes the loop's files every tick) beside them under the same load.

| mission | ticks | equal | the end, the same on both sides | whole applies | wall, this play / the plain play |
|---|---|---|---|---|---|
| night-breach | 1,413 | every tick | taken, t 23.5, 2 of 2 up | 5 | 25 s / 6 s (8.0 s quiet) |
| room-clear-pair | 2,913 | every tick | taken, t 48.5, 1 of 2 up | 10 | 49 s / 42 s |
| room-clear-four | 3,006 | every tick | taken, t 50.1, 4 of 4 up | 9 | 59 s / 35 s |
| hostage-rescue | 3,119 | every tick | rescued, clean, 2 of 2 hostages, t 52 | 7 | 94 s / 26 s |
| records-pair | 2,186 | every tick | taken, t 36.4, 1 of 2 up | 11 | 44 s / 21 s |
| depot-four | 5,118 | every tick | taken, t 85.3, 2 of 4 up | 12 | 116 s / 21 s |
| block-four | 2,470 | every tick | wiped, t 41.2 | 9 | 60 s / 40 s |
| block-squads | 6,418 | every tick | taken, t 107, 3 of 4 up | 18 | 154 s / 72 s |
| warehouse-four | 5,489 | every tick | wiped, t 91.5 | 20 | 144 s / 69 s |
| compound-four | 7,576 | every tick | taken, t 126.3, 2 of 4 up | 23 | 226 s / 108 s |
| barracks-four | 4,438 | every tick | wiped, t 74 | 19 | 155 s / 50 s |
| rescue-registry | 2,746 | every tick | lost, both hostages dead, t 45.8 | 8 | 62 s / 29 s |
| rescue-consulate | 3,028 | every tick | rescued, costly, 1 of 3 hostages, t 50.5 | 8 | 96 s / 50 s |
| snatch-four | 1,585 | every tick | lost (the man got out by the back gate), t 26.4 | 9 | 34 s / 26 s |
| defuse-four | 4,523 | every tick | taken, t 75.4, 3 of 4 up | 13 | 152 s / 72 s |
| floodlit-four | 5,512 | every tick | taken, t 91.9, 2 of 4 up | 15 | 99 s / 91 s |
| killhouse-four | 5,774 | every tick | taken, t 96.2, 2 of 4 up | 18 | 112 s / 75 s |
| mousehole-four | 7,087 | every tick | taken, t 118.1, 3 of 4 up | 18 | 140 s / 106 s |
| hall-four | 1,998 | every tick | wiped, t 33.3 | 7 | 44 s / 27 s |
| glasshouse-four | 4,276 | every tick | taken, t 71.3, 4 of 4 up | 13 | 85 s / 56 s |
| pass-four | 3,761 | every tick | taken, t 62.7, 3 of 4 up | 14 | 122 s / 70 s |

| tutorial | the fast way | with a whole apply every tick |
|---|---|---|
| tut-queue | 6,092 of 6,092 equal, 14 steps, ended the same | |
| tut-team | 6,930 of 6,930 equal, 12 steps, ended the same | |
| tut-road | 4,930 of 4,930 equal, 23 steps, ended the same | |
| tut-pair | equal for 579 ticks, then the lesson's own check stops it: tools/qa/lessons/tut-pair.ts:103 reads `mate.char.anim.crouch`, not on the list | 8,381 of 8,381 equal, 18 steps, ended the same |
| tut-squad | the host's half parts at 2288 (a list's length in the record) | 7,403 of 7,403 equal, 23 steps, ended the same, THROUGH THE CHECKPOINT PUT BACK at 5739 (a new start). One checkpoint of the reference's four (`5574 "the bags"`) is not in this play's calls: not understood |
| tut-quiet | stopped at 1990: tut-quiet.ts:116 reads a man's light on a spot | equal for 5,204 ticks, then tut-quiet.ts:237 finds no `ko` row with `data.method === 'ring'` in the chronicle: the Rust game's or the bridge's, not the host's |
| tut-sky | stopped at 6931: tut-sky.ts:73 reads who was told to run (the team pace's own counters) | not run |
| tut-kit | the host's half parts at 5302 (the cursor) | not run |
| tut-solo | 7,541 of 8,052 equal (past the four authored checkpoints), then the host's half parts at 7542: the cursor, 4 px (`record[0].mx` 1566.85 here, 1570.99), step 'last' (tools/qa/lessons/tut-solo.ts:314 aims at a man's chest, `char.bones.chest`). The same with every module but the heavy tables applied every tick, so not a field the list lacks: open | not run |

**The tallies** (`tools/blitz/rust.sh rust|ts bun run tools/qa/playthrough.ts --seeds 1-2 --json …`, then `tally-diff.ts`, `tools/qa/mission-table.ts`): 21 missions x 2
seeds on each side. ALL 42 RUNS END THE SAME on both sides: the same end, the same second to the tenth, the same men up, the same of theirs left; the
mission table (`tools/qa/mission-table.ts`) is the same in EVERY one of its 20 columns for all 21 missions (ended by kind, seconds, up, none lost, theirs
left, the stragglers' clock, alone %, the smells per run, the frag bursts and kills, hostages). No run's whole record is the same: 215 distinct
places differ, of two kinds. (1) Counters a TypeScript module keeps and its tick bumped, which stand at zero or are absent on the puppet: `destr.*`
(DESTR_STATS: rounds, bangs, tiles, puffs …), `picture.*`, `digin.*`, `rear.*`, `handle.stats.*`, `doors.manners.*`, `holdfire.lifts`,
`coverfire.wall_rounds`, `def.tactics`, `frag`, `doorforce`, `noshove`, `teampace`. (2) Watchers of tools/qa that read the game every tick and meet a
field the filtered list lacks: `entries.#.cross` and `def.doors.#.t` (a crossing seen 0.1 to 0.2 s late), `entries.#.end` ("in" there, "?" here),
`entries.#.blind`, `endgame.rows` (the last stand's said lines), `smells.#.detail` (the gun's word: "cov" there, "aim" here), `losses.#.since`,
`deaths.#.readyIn` / `.track`, `cleared.summary` ("2 of 8 known" there, "0 of 8" here: cleared::ClearedRooms).

Not played: a second seat (the apply nulls `Seat.own` of the seat whose set stands in the game: src/game/seats.ts:93 throws on the first apply), seeds
other than 1 with the per-tick check, a quiet mission, a restart from a card, any page. The audit (`--audit`) after the merge of 7.6: night-breach:1 to its end, equal
(302 s, 214 ms a tick under the load), the scripted player's orders through the menu's own `give` (its rows asking the Rust game), 7 write-backs, all of
memos (`WorkThings.cache`, `Cover.openings`, `DialogueMod.room_rects`, `HoldfireMod.states` and its log, `DoortoolsMod.sky`) and one flooded room; the
apply held to the Rust bytes on 31 ticks, 0 fields differ. No other mission was audited after the merge.
