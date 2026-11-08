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

- **The boot of a page.** A mission is started as a click on its row starts it: `startFromQuery` on the row's link, on both sides (the row's
  hour and weather, the fog of war, a quiet mission's rung and cameras, then the mission). What the page reads off the address BEFORE the
  stand-up (the weather and the hour as it boots, the kit, the words) is not run by either start here.
- One seed of the die (20942), the mission's own deal, an idle hand. Nobody fires, orders or opens a door in these frames.
- Nothing here has been seen on a GPU: both games stand on a host that draws nothing, with the rows that only dress the picture down.

<!-- table -->
30 of 30 missions start in Rust equal to the TypeScript start (die seed 20942, 300 frames).

| mission | the file the page loads | starts in Rust | paths not known at the start | 300 frames beside the TypeScript start | paths not known as they end | what the start lacks |
|---|---|---|---|---|---|---|
| `tut-road` | `tut-road.json` | yes | 0 | 300 in step | 0 |  |
| `tut-team` | `tut-team.json` | yes | 0 | 300 in step | 0 |  |
| `tut-kit` | `tut-kit.json` | yes | 0 | 300 in step | 0 |  |
| `tut-queue` | `tut-queue.json` | yes | 0 | 300 in step | 0 |  |
| `tut-quiet` (off the list) | `tut-quiet.json` | yes | 0 | 300 in step | 0 |  |
| `tut-pair` (off the list) | `tut-pair.json` | yes | 0 | 300 in step | 0 |  |
| `night-breach` (off the list) | `office.json` | yes | 0 | 300 in step | 0 |  |
| `room-clear-pair` | `office.json` | yes | 0 | 300 in step | 0 |  |
| `room-clear-four` (off the list) | `office.json` | yes | 0 | 300 in step | 0 |  |
| `hostage-rescue` | `office.json` | yes | 0 | 300 in step | 0 |  |
| `records-pair` (off the list) | `records.json` | yes | 0 | 300 in step | 0 |  |
| `depot-four` | `depot.json` | yes | 0 | 300 in step | 0 |  |
| `block-four` (off the list) | `block.json` | yes | 0 | 300 in step | 0 |  |
| `warehouse-four` (off the list) | `warehouse.json` | yes | 0 | 300 in step | 0 |  |
| `compound-four` | `compound.json` | yes | 0 | 300 in step | 0 |  |
| `barracks-four` (off the list) | `barracks.json` | yes | 0 | 300 in step | 0 |  |
| `rescue-registry` (off the list) | `registry.json` | yes | 0 | 300 in step | 0 |  |
| `rescue-consulate` | `consulate.json` | yes | 0 | 300 in step | 0 |  |
| `floodlit-four` (off the list) | `floodlit.json` | yes | 0 | 300 in step | 0 |  |
| `killhouse-four` (off the list) | `killhouse.json` | yes | 0 | 300 in step | 0 |  |
| `mousehole-four` | `mousehole.json` | yes | 0 | 300 in step | 0 |  |
| `hall-four` (off the list) | `hall.json` | yes | 0 | 300 in step | 0 |  |
| `glasshouse-four` (off the list) | `glasshouse.json` | yes | 0 | 300 in step | 0 |  |
| `pass-four` | `pass.json` | yes | 0 | 300 in step | 0 |  |
| `bazaar-quiet` | `bazaar.json` | yes | 0 | 300 in step | 0 |  |
| `police-post-quiet` (off the list) | `police-post.json` | yes | 0 | 300 in step | 0 |  |
| `vault-quiet` | `vault.json` | yes | 0 | 300 in step | 0 |  |
| `quiet-crossing-two` (off the list) | `crossing.json` | yes | 0 | 300 in step | 0 |  |
| `quiet-relay-two` (off the list) | `quiet-relay.json` | yes | 0 | 300 in step | 0 |  |
| `lights-out-two` | `lights-out.json` | yes | 0 | 300 in step | 0 |  |
<!-- /table -->

## As games of two players

The same rows with two seats (`seats.ts`; `StartOpts.seats`). The second player has nobody at his controls: each frame his record is the still
one a host hands a seat it has none for. What two players DO is not held here, only that the game the Rust start makes for two is the game the
TypeScript start makes for two, and that the two stay in step while nobody moves.

<!-- table seats=2 -->
18 of 18 missions the game stages for 2 players start in Rust equal to the TypeScript start (die seed 20942, 300 frames, 2 seats); 12 the game refuses for 2 players.

| mission | the file the page loads | starts in Rust | paths not known at the start | 300 frames beside the TypeScript start | paths not known as they end | what the start lacks |
|---|---|---|---|---|---|---|
| `tut-road` | `tut-road.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `tut-team` | `tut-team.json` | yes | 0 | 300 in step | 0 |  |
| `tut-kit` | `tut-kit.json` | yes | 0 | 300 in step | 0 |  |
| `tut-queue` | `tut-queue.json` | yes | 0 | 300 in step | 0 |  |
| `tut-quiet` (off the list) | `tut-quiet.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `tut-pair` (off the list) | `tut-pair.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `night-breach` (off the list) | `office.json` | yes | 0 | 300 in step | 0 |  |
| `room-clear-pair` | `office.json` | yes | 0 | 300 in step | 0 |  |
| `room-clear-four` (off the list) | `office.json` | yes | 0 | 300 in step | 0 |  |
| `hostage-rescue` | `office.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `records-pair` (off the list) | `records.json` | yes | 0 | 300 in step | 0 |  |
| `depot-four` | `depot.json` | yes | 0 | 300 in step | 0 |  |
| `block-four` (off the list) | `block.json` | yes | 0 | 300 in step | 0 |  |
| `warehouse-four` (off the list) | `warehouse.json` | yes | 0 | 300 in step | 0 |  |
| `compound-four` | `compound.json` | yes | 0 | 300 in step | 0 |  |
| `barracks-four` (off the list) | `barracks.json` | yes | 0 | 300 in step | 0 |  |
| `rescue-registry` (off the list) | `registry.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `rescue-consulate` | `consulate.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `floodlit-four` (off the list) | `floodlit.json` | yes | 0 | 300 in step | 0 |  |
| `killhouse-four` (off the list) | `killhouse.json` | yes | 0 | 300 in step | 0 |  |
| `mousehole-four` | `mousehole.json` | yes | 0 | 300 in step | 0 |  |
| `hall-four` (off the list) | `hall.json` | yes | 0 | 300 in step | 0 |  |
| `glasshouse-four` (off the list) | `glasshouse.json` | yes | 0 | 300 in step | 0 |  |
| `pass-four` | `pass.json` | yes | 0 | 300 in step | 0 |  |
| `bazaar-quiet` | `bazaar.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `police-post-quiet` (off the list) | `police-post.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `vault-quiet` | `vault.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `quiet-crossing-two` (off the list) | `crossing.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `quiet-relay-two` (off the list) | `quiet-relay.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
| `lights-out-two` | `lights-out.json` | not for this many players |  |  |  | the game refuses it for 2 players (it takes one player today) |
<!-- /table seats=2 -->
