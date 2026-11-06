# tools/blitz — the headless host: the scripted player plays the Rust game

`play.ts` stands a mission as `tools/qa/playthrough.ts` does (a tutorial: `tools/qa/lesson-run.ts`), and from the first tick **the Rust game is the only
game that steps**. The TypeScript `Game` is a puppet: after each step the Rust game's whole state is applied onto it in place (`import.ts`) and rust/sim's
block is copied over its own. The scripted player reads the puppet, and what it presses goes to the Rust game as the tick's record. `stepSeats`' body
never runs.

```sh
tools/blitz/play.sh room-clear-four:1                  # the plain TypeScript play first (the reference), the face built, then the play, every tick compared
tools/blitz/play.sh <mission>:<seed> --ticks 600 --no-build -- --go-on
bun run --preload ./tools/blitz/expose.ts tools/blitz/play.ts <mission>:<seed> --ref DIR [--out DIR] [--ticks N] [--wasm FILE] [--apply-check N] [--go-on] [--quiet]
```

Exit: 0 equal to the end and ended the same way · 1 parted · 2 the Rust tick panicked · 3 a side could not run · 4 equal every tick, the end differs.

## What is compared, and against what

The reference is the same mission and seed played by the plain TypeScript game (`mission.ts`): after every tick the die's count and a hash of each of the
13 parts of rust/sim's block, and before every tick the host's half (`h<k>.bin`). `play.ts` holds to it, every tick:

- **the block and the die**, as the tick returns (the 12 parts but `poses`, which parts by design from the first death: README.md);
- **the host's half**: the record the scripted player's keys and camera made. This is the sharper check. The block can stay equal for hundreds of ticks
  after the puppet has shown the player something the TypeScript game would not have (a camera 4 m off parted the block 620 ticks later);
- **the end**: taken / failed, the second, who is up, the tick.

A run with no `--ref` says NOTHING COMPARED and exits 3.

## A tick

1. The scripted player presses its keys and steps its world (`headless.ts W.step` → `Game.update`).
2. `Game.update`'s own lines run as they are: the record sampled from the Input and the camera, `hostBefore`, `hostAfter`. The step between them is
   `puppetStep`, hung on the game object in place of `stepSeats`.
3. **The write-back.** The puppet is exported and held to the bytes last applied. What differs outside the host's half is something TypeScript wrote into
   the game between two ticks; the Rust game takes the puppet's state (`face_import_into`). The block is held and handed over the same way.
4. **The host's half**: `export.ts hostHalf` → `face_host_half` (each seat's record, every camera and Input row).
5. `face_step`. The die and the hashes are read here. Then, on the Rust side, the lights' update, a rails level's host view, and on every 4th tick the box
   world's listing (where `headless.ts` lists it).
6. **The apply**: `face_export` (the whole state) → `applyGame`; the block copied.
7. **The dress step**, what has no Rust body and a reader needs: `Par.tick`, `Verdict.tick`, and the autosave (`saves.tick`), whose checkpoint is handed to
   the Rust side by `face_checkpoint`.

Calls the scripted player makes that are no input and need an answer at once go to the Rust game when they are made, and the puppet takes the state
again before the call returns: `orders.give` (`face_give`) and `game.selectVerb` (`face_hostwrite`).

## Fields that are each side's own

Cut out of every compare and never handed across:

| field | why |
|---|---|
| `StaticCollision.mirror`, `Mods.scene_box_mirror` | the copy of the box world in the module that holds it; the TypeScript game's module is not the Rust game's. Also left out of the apply |
| `Game.door_owns`, `AnimGltfMod.nvg_bits`, `WallBreak.listeners` | the export works them out or reads a module variable the apply does not write back: the puppet's are not the Rust game's |

Seen by the compare and not a write: identity serials (`schema.serial`), `schema.scratch`, `ClipSide.at` (the export writes a constant), the ask counters
`ANYTIME.*` and `SENSED.*` of scene/collision.ts. The host's half as a compare sees it also holds `Game.view_proj` and `SquadOrdersView.input` / `.cam`,
which `hostAfter` writes after every step and the tick writes again as it opens.

## Stand-ins in play.ts (each marked `STAND-IN:` with its reason)

- **A seat's cut.** `Seat.takeCut` (cmd.ts:191) knows a cut by the identity of the lens's `desired` array against the seat's `mark`. The apply writes
  `desired` in place and writes `mark` from a Rust field that counts something else, so `takeCut` answered a cut every tick and `hostAfter` snapped the
  host's camera every tick. The seat keeps its own `mark`; a `desired` whose numbers changed gets a new array.
- **The squad's posture** (`SquadOrders.posture`, three closures; a struct of no fields in the bridge): the puppet keeps the object it had.
- **A new order row** takes the functions of a TypeScript row of the same id, where there is one. Most have none: see below.

## What does not work on a puppet

- **A row of the order menu registered inside a tick** (silent.ts, doortools.ts, pairorders.ts, shutdoor.ts, squad40.ts, squadsmoke.ts) arrives with no
  closures: `label`, `can`, `issue` are missing. The scripted player's orders therefore go to the Rust game. A HUD that draws the menu from these rows has
  nothing to call.
- **A write while a beat of a level's script runs** cannot be handed over whole: the Rust import refuses such a state. It is counted and named; the next
  apply puts the Rust game's own back.
- **Out events** are counted and cleared, not dispatched: the whole export does not carry them.
- **Order targets the face has no words for** (a wall, a thing, options, one man by object) still run through the menu on the puppet and are written back.

## Numbers (room-clear-four:1, 3,006 ticks, release module, 4 cores with other work running)

| ms a tick, mean | |
|---|---|
| the scripted player and its world's own lines | 0.86 |
| `Game.update`'s own lines | 0.03 |
| the write-back: the puppet's export · the compare · the hand-over | 125.1 · 10.6 · 0.8 |
| the host's half | 0.6 |
| `face_step` | 2.12 (worst 29.5, tick 1) |
| lights, view, listing | 0.01 |
| `face_export`, with the copy out of the module | 8.6 |
| the apply | 119.4 (worst 206) |
| the block's copy | 2.1 |
| the dress step | 0.23 |
| the puppet exported again and held to the Rust bytes (every 50th tick) | 2.8 |

Wall: 822 s for the mission, 273 ms a tick, against 15 s for the plain TypeScript play with the loop's files written. Two whole-state walks in TypeScript
(the export for the write-back, the apply) are 90% of it; the Rust game's own step and export are 4%.
