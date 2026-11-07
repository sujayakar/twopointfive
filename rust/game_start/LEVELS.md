# Which missions start in Rust

Every row of the mission list (`src/scene/scenarioList.ts`), started by `rust/game_start` and held to the TypeScript start by
`tools/blitz/start.sh --mission <id>` (README, "The compare"). The table is written by a tool and is not edited by hand:

```sh
bun run tools/blitz/start-levels.ts --write          # every mission, three at a time: about three minutes
bun run tools/blitz/start-levels.ts --seats 2 --write   # the same as games of two players (the second table)
bun run tools/blitz/start-levels.ts --only depot-four,vault-quiet
```

How to read a row:

- **the file the page loads**: the level's file in the look that is on (`scene/look.ts lookLevelFile`).
- **starts in Rust**: `yes` = the file is carried whole, no path of the two starts' states differs that is not known
  (`tools/blitz/known.txt`, `known-start.txt`: the picture's tables, memos, addresses), and the two games stay in step. `no` = the Rust reader
  refuses the file, and the last column says which records it has no line for. `NOT EQUAL` = it stands and a path differs. `DID NOT RUN` =
  a start stopped (the game refused the mission, or a start panicked: its row says which).
- `not for this many players` (the second table) = the game itself refuses the mission for two (a rescue, a quiet mission), on the TypeScript
  side; the Rust start is not asked.
- The two states are compared BY REACH FROM THE LIVE GAME (`diff.ts --live`): a man a mission took off the floor, whom the TypeScript still
  lists in a weak table until its collector runs, is counted and not compared.
- **frames beside**: the TypeScript start is imported and both games are stepped in Rust with an idle hand: the die's count and every part of the
  block compared after every frame.

What a row does NOT hold:

- **The mission is started by `startScenario`** (its kind, team size and deal), on both sides. The page starts it by `startFromQuery`, which first
  lays the row's hour and weather, the fog of war and a quiet mission's cameras off the address. The Rust game has no type for an address's
  words, so that step cannot run in Rust yet (README, "What stands between this and the page").
- One seed of the die (20942), the mission's own deal, an idle hand. Nobody fires, orders or opens a door in these frames.
- Nothing here has been seen on a GPU: both games stand on a host that draws nothing, with the rows that only dress the picture down.

<!-- table -->
38 of 38 missions start in Rust equal to the TypeScript start (die seed 20942, 300 frames).

| mission | the file the page loads | starts in Rust | paths not known at the start | 300 frames beside the TypeScript start | paths not known as they end | what the start lacks |
|---|---|---|---|---|---|---|
| `tut-road` | `tut-road.json` | yes | 0 | 300 in step | 0 |  |
| `tut-team` | `tut-team.json` | yes | 0 | 300 in step | 0 |  |
| `tut-sky` (off the list) | `tut-sky.json` | yes | 0 | 300 in step | 0 |  |
| `tut-kit` | `tut-kit.json` | yes | 0 | 300 in step | 0 |  |
| `tut-solo` (off the list) | `tut-solo.json` | yes | 0 | 300 in step | 0 |  |
| `tut-squad` (off the list) | `tut-squad.json` | yes | 0 | 300 in step | 0 |  |
| `tut-queue` | `tut-queue.json` | yes | 0 | 300 in step | 0 |  |
| `tut-quiet` | `tut-quiet.json` | yes | 0 | 300 in step | 0 |  |
| `tut-pair` | `tut-pair.json` | yes | 0 | 300 in step | 0 |  |
| `night-breach` (off the list) | `office.json` | yes | 0 | 300 in step | 0 | its missions are not in its file (levels/office.scenarios.ts, attached by name): held on a copy of the file with them written in |
| `room-clear-pair` | `office.json` | yes | 0 | 300 in step | 0 | its missions are not in its file (levels/office.scenarios.ts, attached by name): held on a copy of the file with them written in |
| `room-clear-four` (off the list) | `office.json` | yes | 0 | 300 in step | 0 | its missions are not in its file (levels/office.scenarios.ts, attached by name): held on a copy of the file with them written in |
| `hostage-rescue` | `office.json` | yes | 0 | 300 in step | 0 | its missions are not in its file (levels/office.scenarios.ts, attached by name): held on a copy of the file with them written in |
| `records-pair` | `records-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `depot-four` | `depot-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `block-four` | `block-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `block-squads` (off the list) | `block-squads-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `warehouse-four` | `warehouse-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `compound-four` | `compound-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `barracks-four` | `barracks-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `rescue-registry` | `registry-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `rescue-consulate` | `consulate-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `snatch-four` | `snatch.json` | yes | 0 | 300 in step | 0 |  |
| `defuse-four` | `defuse-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `floodlit-four` | `floodlit-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `killhouse-four` | `killhouse-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `mousehole-four` | `mousehole-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `hall-four` | `hall-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `glasshouse-four` | `glasshouse.json` | yes | 0 | 300 in step | 0 |  |
| `pass-four` | `pass.json` | yes | 0 | 300 in step | 0 |  |
| `bazaar-quiet` | `bazaar.json` | yes | 0 | 300 in step | 0 |  |
| `police-post-quiet` | `police-post.json` | yes | 0 | 300 in step | 0 |  |
| `vault-quiet` | `vault.json` | yes | 0 | 300 in step | 0 |  |
| `quiet-crossing-two` | `crossing.json` | yes | 0 | 300 in step | 0 |  |
| `quiet-relay-two` | `quiet-relay.json` | yes | 0 | 300 in step | 0 |  |
| `lights-out-two` | `lights-out.json` | yes | 0 | 300 in step | 0 |  |
| `quiet-yard-alone` (off the list) | `quiet-yard.json` | yes | 0 | 300 in step | 0 |  |
| `quiet-yard-two` (off the list) | `quiet-yard.json` | yes | 0 | 300 in step | 0 |  |
<!-- /table -->

## As games of two players

The same rows with two seats (`seats.ts`; `StartOpts.seats`). The second player has nobody at his controls: each frame his record is the still
one a host hands a seat it has none for. What two players DO is not held here, only that the game the Rust start makes for two is the game the
TypeScript start makes for two, and that the two stay in step while nobody moves.

<!-- table seats=2 -->
23 of 23 missions the game stages for 2 players start in Rust equal to the TypeScript start (die seed 20942, 300 frames, 2 seats); 15 the game refuses for 2 players.

| mission | the file the page loads | starts in Rust | paths not known at the start | 300 frames beside the TypeScript start | paths not known as they end | what the start lacks |
|---|---|---|---|---|---|---|
| `tut-road` | `tut-road.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `tut-team` | `tut-team.json` | yes | 0 | 300 in step | 0 |  |
| `tut-sky` (off the list) | `tut-sky.json` | yes | 0 | 300 in step | 0 |  |
| `tut-kit` | `tut-kit.json` | yes | 0 | 300 in step | 0 |  |
| `tut-solo` (off the list) | `tut-solo.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `tut-squad` (off the list) | `tut-squad.json` | yes | 0 | 300 in step | 0 |  |
| `tut-queue` | `tut-queue.json` | yes | 0 | 300 in step | 0 |  |
| `tut-quiet` | `tut-quiet.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `tut-pair` | `tut-pair.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `night-breach` (off the list) | `office.json` | yes | 0 | 300 in step | 0 | its missions are not in its file (levels/office.scenarios.ts, attached by name): held on a copy of the file with them written in |
| `room-clear-pair` | `office.json` | yes | 0 | 300 in step | 0 | its missions are not in its file (levels/office.scenarios.ts, attached by name): held on a copy of the file with them written in |
| `room-clear-four` (off the list) | `office.json` | yes | 0 | 300 in step | 0 | its missions are not in its file (levels/office.scenarios.ts, attached by name): held on a copy of the file with them written in |
| `hostage-rescue` | `office.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) · its missions are not in its file (levels/office.scenarios.ts, attached by name): held on a copy of the file with them written in |
| `records-pair` | `records-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `depot-four` | `depot-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `block-four` | `block-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `block-squads` (off the list) | `block-squads-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `warehouse-four` | `warehouse-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `compound-four` | `compound-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `barracks-four` | `barracks-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `rescue-registry` | `registry-gwot.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `rescue-consulate` | `consulate-gwot.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `snatch-four` | `snatch.json` | yes | 0 | 300 in step | 0 |  |
| `defuse-four` | `defuse-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `floodlit-four` | `floodlit-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `killhouse-four` | `killhouse-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `mousehole-four` | `mousehole-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `hall-four` | `hall-gwot.json` | yes | 0 | 300 in step | 0 |  |
| `glasshouse-four` | `glasshouse.json` | yes | 0 | 300 in step | 0 |  |
| `pass-four` | `pass.json` | yes | 0 | 300 in step | 0 |  |
| `bazaar-quiet` | `bazaar.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `police-post-quiet` | `police-post.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `vault-quiet` | `vault.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `quiet-crossing-two` | `crossing.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `quiet-relay-two` | `quiet-relay.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `lights-out-two` | `lights-out.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `quiet-yard-alone` (off the list) | `quiet-yard.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `quiet-yard-two` (off the list) | `quiet-yard.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
<!-- /table seats=2 -->
