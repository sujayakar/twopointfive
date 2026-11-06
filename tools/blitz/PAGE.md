# The page plays the Rust game (`?rust=1`)

With the word, the game's tick is the Rust game's (`rust/game`, in the module `rust/game_face` builds) and the TypeScript `Game` stays as a puppet that the
picture, the sound, the HUD and the menus go on reading. Without the word nothing of this is loaded and the page is the page it was. The host is
`src/host/rustgame.ts` (its head says what a step is); the lines of the TypeScript tick that are for the picture only are `src/host/rustdress.ts`; the face's
exports for a host that shows the game are `rust/game_face/src/page.rs`.

## Building

`rust/game_face` is built by `tools/blitz/face-build.ts` (the dev server and `bun run build` both call it when a source under `rust/` is newer than the
module). The file is about 24 MB and is never checked in. Where it cannot be built the reason is said in plain words, and the page shows it under the word:

| the machine | what is said |
|---|---|
| no cargo, no built module | The Rust game is not built, and this machine has no cargo to build it (rustup, then the wasm32-unknown-unknown target). |
| no cargo, a module older than its sources | the file as it stands is served, and the server says so |
| a cargo that cannot link wasm (no `lld`, no target) | The Rust game did not build: `linker lld not found` …, and that rustup's pinned toolchain brings both |
| a build without `game_face.wasm` (a static host) | `?rust=1: there is no Rust game to play here. This build was made without game_face.wasm …` |

## A game kept: saves, loads, the restart

The save controller (`src/game/savestate.ts`) runs on the puppet with its slots, labels and rhythm as ever. What it keeps is not the puppet:

- **A save** (`SNAPS.take`) is the Rust game kept inside the module: `face_keep(h, 0)`, a clone of the whole `Game` and its block. Nothing of the puppet is
  walked and nothing is applied whole. The chronicle's row is made on the Rust side first (`face_checkpoint`; on a tutorial mission that call also stops the
  level's program for the take and has the frame start it again, in the controller's own order).
- **A load** (`SNAPS.put`), the down card's checkpoint, Enter once down: `face_back(h, k)` and ONE whole apply onto the puppet. The lines the controller
  then runs on the puppet (this run's counts, the chronicle's row, a tutorial's program to be started again) are made on the Rust side by one call
  (`face_loaded`), and the filtered export starts over. (The bridge's own export of the puppet cannot carry them: its book still lists the rows the
  abandoned moment made, a running beat of a tutorial's program among them, and the import refuses that state.)
- **The player's restart** (a card's button, the panel's, Shift+Enter): the game kept as the first step opened stands again. This is NOT the TypeScript's
  restart: that one keeps the clock and the die's place and deals a quiet mission afresh; this one is the same deal again. Both the TypeScript's
  `restartEncounter` and the Rust game's `game_restart` make the player's man anew, and a man made on the Rust side has no object the page can draw yet.
- **The records only the page writes** (`verdict`, `par`: the dress step's) are copied with every kept game and written back with it.
- **`Game.saves` is not on the bridge.** The Rust tick reads it (Enter once down asks for this run's newest save), so the host hands the shelf's rows over
  whenever they change (`face_saves_in`) and reads back what the tick asked (`face_saves_asked`).
- **A run alone saves in memory too** under the word (the TypeScript writes a file there). A file of the TypeScript's fields is refused: it would be poured
  over a puppet.

**The file** (`window.__rust.file(key?)`, `window.__rust.fileIn(bytes)`, the debugger's export and import rows): `TPSR`, a version word, then three counted
parts: a JSON head (the level's fingerprint, this build's word for the schema, the label, the mission's clock and stage), the bridge's bytes of the kept
game (`io::export`), and its block. Read back, it is put in the ring of saves by hand and loaded like any save. A file holds less than a save in memory:
what the bridge does not carry (BRIDGE.md) starts blank, and the page's own records are the moment's it was read in, not the file's.

Left: saves do not outlive the page (the store is the browser's, a few megabytes, and a kept game is about eight); the panel's 'save (force)' and
`__save.snapshot` still walk the puppet; the file has no picture.

## What a step costs, and what stands in the way

Measured on the built page, software adapter, undrawn frames, 1,800 frames with no input after 600 to settle. The box was never quiet (the load average
is beside each row), so read the proportions, not the milliseconds.

| room clear, four men (load 4.4 to 4.8) | TypeScript page | Rust page |
|---|---|---|
| a frame | 3.52 ms | 6.64 ms |
| `Game.update` | 2.80 ms | 5.39 ms |
| the tick itself | (all of the above) | 1.57 ms |
| the filtered export (211 KB a step) | | 0.81 ms |
| the apply onto the puppet | | 2.27 ms (the main walk 1.03, the tables 1.08) |
| the host's half in, the knobs | | 0.27 ms |
| the dress step | (inside the tick) | 0.27 ms |
| lights, probe, doors, saves | | 0.09 ms |
| the first frame of play | 349 to 476 ms | 1,383 ms; 489 ms after the warm-up behind the start card |

| depot, four men (load 10.7 to 11.2) | TypeScript page | Rust page |
|---|---|---|
| a frame | 6.45 ms | 12.62 ms |
| `Game.update` | 4.84 ms | 10.05 ms |
| the tick itself | | 2.86 ms |
| the filtered export (153 KB a step) | | 1.17 ms |
| the apply | | 4.24 ms (main 1.56, tables 2.37) |
| the host's half in, the knobs | | 0.72 ms |
| the dress step | | 0.59 ms |

The Rust tick alone is quicker than the TypeScript's. The surplus is the puppet: the export and the apply are three fifths of a step, and they exist only
while there is a puppet. What they spend it on (reads that take the importer's slow path, a step, room clear): a man's bones 1,110; the box world's
statics 874; the fog frame's lights 769; the frame's boxes, the fog's things and the world's dynamics 430 each; a thing's ghost 281; a box's optional
fields (`marks`, `cut`, `cut_glass`, `was`, `shape`) about 179 each. All of it is in generated files. Asked of the generator: a memo per field inside
the big parts (a list that did not change is sent whole when one neighbour did), and compiled readers for bones and for a box's optional fields.

Outside `Game.update` the Rust page's frame is 0.8 ms dearer too (a CPU profile of both pages, 1,200 frames, the dev server's bundle for its names): the
HUD's lines, which run on the puppet between ticks. The cursor's ray goes through the puppet's own copy of the box world, which stands aside from the Rust
game's and is made current again every frame (0.30 ms); the target under the cursor is worked out again where the TypeScript page has the tick's
(0.27 ms); and the menu's default row asks the Rust game its questions every frame (0.30 ms of bytes written for them).

Memory does not grow over 1,800 steps (153 MB; the module reserves once at the start). A save is 4 to 24 ms where the TypeScript's snapshot is 79 to
103 ms; a load is 280 to 550 ms (one whole apply) where the TypeScript's is 27 to 51 ms.

## Checking it

A scripted player in the page (the same hands as `tools/qa/lesson-run.ts`, over the page's own Input) plays the address without the word and keeps every
frame's input; the address with the word plays that input back; the die's count and the hash of each part of the game's block are held together every 60
frames. Software adapter, undrawn frames: this holds the simulation, not the look.
