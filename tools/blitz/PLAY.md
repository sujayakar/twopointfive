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
counted and not called again. `WallWearRound` throws on every mission that makes one (`box.h[0]` of undefined: the event's `box_` does not come back as
an object).

## The harness's word

`TPS_GAME` in front of any headless tool, with the bridge's preload for the tool and its children (`tools/blitz/rust.sh` sets both):

| | |
|---|---|
| `TPS_GAME=rust` | the game `tools/qa/headless.ts standUp` returns is a puppet of the Rust game (the fast way) |
| `TPS_GAME=audit` | the same, audited |
| `TPS_GAME=ts` | the SAME STAND stepped by the TypeScript tick: the other side of a tally |
| unset | nothing of this is loaded |

The stand under any of the three words is mission.ts's: the game rolls a die of its own where the tool gave it none (seeded off the tool's `Math.random`
as the game is stood, so `playthrough.ts`'s seeds roll different dice and both words roll the same), and the dress rows are down. So a
tally under `ts` is not the tally the tool makes with no word (which rolls `Math.random` and dresses); it is the one to hold the Rust game's beside.
(`TPS_RUST` cannot be the word: src/sim/sim.ts refuses it as a retired one.) `TPS_GAME_WHOLE=N`: a whole apply every N ticks.

## Stand-ins (each marked `STAND-IN:` in the code with its reason and where it belongs)

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
mission table's columns (ended by kind, seconds, up, none lost, theirs left) are the same row for row. No run's whole record is the same: 215 distinct
places differ, of two kinds. (1) Counters a TypeScript module keeps and its tick bumped, which stand at zero or are absent on the puppet: `destr.*`
(DESTR_STATS: rounds, bangs, tiles, puffs …), `picture.*`, `digin.*`, `rear.*`, `handle.stats.*`, `doors.manners.*`, `holdfire.lifts`,
`coverfire.wall_rounds`, `def.tactics`, `frag`, `doorforce`, `noshove`, `teampace`. (2) Watchers of tools/qa that read the game every tick and meet a
field the filtered list lacks: `entries.#.cross` and `def.doors.#.t` (a crossing seen 0.1 to 0.2 s late), `entries.#.end` ("in" there, "?" here),
`entries.#.blind`, `endgame.rows` (the last stand's said lines), `smells.#.detail` (the gun's word: "cov" there, "aim" here), `losses.#.since`,
`deaths.#.readyIn` / `.track`, `cleared.summary` ("2 of 8 known" there, "0 of 8" here: cleared::ClearedRooms).

Not played: a second seat (the apply nulls `Seat.own` of the seat whose set stands in the game: src/game/seats.ts:93 throws on the first apply), seeds
other than 1 with the per-tick check, a quiet mission, a restart from a card, any page. The audit (`--audit`) was not run to a mission's end after the
merge of 7.6 (the host of the three audited missions before it is the same code path).
