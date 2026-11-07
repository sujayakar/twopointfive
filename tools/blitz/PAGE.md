# The page plays the Rust game

The Rust game is the page's game: no address word is needed. `?rust=0` opens the TypeScript game for as long as that is still in the tree, and a page that
is a game server's client or its door (`?net=`, `?join=`) or that plays a tape (`?playtape=`) stays with the TypeScript game until those have been played on the
Rust one (`?rust=1` asks it of them all the same; `src/main.ts` holds the rule). The words below that say "under the word" or `?rust=1` were written while the
Rust game was behind that word, and read the same for the page with no word.

On the Rust game, the game's tick is the Rust game's (`rust/game`, in the module `rust/game_face` builds) and the TypeScript `Game` stays as a puppet that the
picture, the sound, the HUD and the menus go on reading. Under `?rust=0` nothing of this is loaded and the page is the page it was. The host is
`src/host/rustgame.ts` (its head says what a step is); the lines of the TypeScript tick that are for the picture only are `src/host/rustdress.ts`; the face's
exports for a host that shows the game are `rust/game_face/src/page.rs`.

## Building, and a first run

`rust/game_face` is built by `tools/blitz/face-build.ts`. The file is 25 to 27 MB; a built one is checked in beside its sources' hash (`rust/game_face/prebuilt/`, served where no build of the machine is fresh and the hash is the checkout's; a hosted build never takes it). Two profiles (`rust/game_face/Cargo.toml`):

| profile | what it is | who builds it | measured here (3 jobs, from nothing) |
|---|---|---|---|
| `play` | opt-level 3, one unit, NO link-time optimisation | the dev server (`bun run dev`), when a source under `rust/` is newer than the module | 7 min 33 s, rustc peaking at 6.2 GB, 25.1 MB |
| `release` | the same with the fat link | `bun run build` (`FACE_PROFILE=play` in front builds the other) | the link alone passed 10.5 GB and was killed in a 12 GB box three times |

A module of either profile that is newer than the sources is served as it is. **A first `bun run dev` builds nothing.** The first page opened with
`?rust=1` asks for `/game_face.wasm`; the server starts the build and does not hold the request: it answers 202 with a line, the page shows that line
where `loading…` stands and asks again every two seconds, and cargo's own lines (each crate as it compiles, what rustup fetches) go to the server's
terminal. When the build ends the page starts by itself. `rust/rust-toolchain.toml` names the compiler (1.97.0) and the wasm32-unknown-unknown target:
with rustup installed the first build fetches both.

| the machine | what the page says (the fatal card, in these words; the terminal says the same) |
|---|---|
| no cargo, never built | The Rust game is not built, and this machine has no cargo to build it. Install Rust with rustup (one line in a terminal: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` ), then load this page again: the dev server builds the module itself … |
| no cargo, a module older than its sources | the file as it stands is served, and the server says so |
| a cargo that cannot build for wasm (no `lld`, no target) | The Rust game did not build: … This cargo (its path) cannot build for wasm32-unknown-unknown. If it is rustup's: `rustup target add wasm32-unknown-unknown` . If it is not (a system package): install rustup … |
| the compiler killed (memory) | The Rust game did not build: … The compiler was killed, most likely for memory (this build needs about 7 GB free): close what else is running and load the page again. |
| a build without `game_face.wasm` (a static host) | `?rust=1: there is no Rust game to play here. This build was made without game_face.wasm …` |

Shown with a stand-in for cargo (the four answers of the server and its terminal's lines); a real first build through the dev server and the two
lines on the screen have not been run in a browser.

## The switches that are constants now

866 switch rows of the knob tables are settled (`tools/qa/switches.json`): the Rust game has only the branch the page takes. On the TypeScript page
they are still switches, moved by a row of the debugger's panel kept in the browser's store, by a word on the address, by a click. Under `?rust=1`
each is held at its page value from before the level is built (`rustSettledHold`, as `tools/qa/fold.ts` holds them for a headless run): a read answers
that value, a write of another is refused, and what was refused is named once in the console and stays on the debugger panel's first line ("N kept
rows or address words are settled in this build and are not applied: …"; `window.__rust.settled`). So the half of the game the page still runs (the
HUD, the menus, the picture's dressing) and the tick agree. Rows that are still switches (open decisions, the player's own, the difficulty's) cross
to the Rust game with the knob tables as a game starts, and one changed in the middle of a mission reaches it by the settings' walk, which is made
every sixteenth tick: a quarter of a second at most. Measured on the compound (a row written, n frames stepped, the Rust game's whole state put back on
the puppet, the row read): the squad ring's two rows, the team pace's two, `PRESSURE.on`, `DIGIN.on` and the fog's cone are there after 16 and after 17
frames every time, and the debugger's 'hard defenders' box turns all three of the difficulty's rows 17 frames on. A settled row written on the page
still reads its page value and is named in `window.__rust.settled.refused`. One gap: a row written and then, inside those sixteen ticks, a whole
apply (a load, a restart) is put back to the Rust game's value, and the write is lost.

## A game kept: saves, loads, the restart

**A save of the Rust page does not survive a reload today.** Every save, by hand or by the game, lives in the page's memory (a game kept inside the
module) and is gone with the tab. The design for saves that outlive the page is in the line's log, not in the tree.

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

Under the word the saves list holds only the games kept in the module: a file of the TypeScript's fields in the browser's store is the other page's
(one origin, one store), a load of it is refused, and it is not listed (`savestate.ts slots()`).

Left: saves do not outlive the page (the store is the browser's, a few megabytes, and a kept game is about eight); the panel's 'save (force)' and
`__save.snapshot` still walk the puppet; the file has no picture.

## What the page writes between ticks

- **A checkpoint the tick asks for** (`CheckpointsKeep`, `CheckpointsRequest`): both arms of the dispatcher call `OUT_HOST.checkpoint`, which the host sets
  to the save controller; its take is the one above, so the state is the Rust game's.
- **The options', the pause card's and the debugger's rows** (god mode, the AI, ammunition, the fog, every number and switch of `game.tune`, the
  furniture's friction, the wand, the big kit) are written on the puppet by the page as ever. A step opens by sending what differs from what the last
  apply left through the generated write by path (`tools/blitz/poke.ts` → `face_write` → `io::poke`): no call of the face a field. A tool writes with
  `window.__rust.poke(obj, path, value)`, which answers null or the reason. `window.__rust.poked` counts them by path; `pokeRefused` names a write that
  has no way there.
- **The options' particle row** (`engine.settings.gpuParticles`) is among the watched writes: the tick reads it (`gpuFx`).
- **Not covered:** a panel's row that calls a method (it runs a rule on the puppet only), and a write to a field outside the list in
  `src/host/rustgame.ts` `watched` (the next apply may put the Rust game's value back over it).

## A call from outside a tick

The keys are in the input record and the Rust tick acts on them. A call made between ticks is another matter: a tool's `__orders.open()`, a panel's
button. Run as TypeScript on the puppet it would build state the Rust game never sees, and the next question about that state names a row the Rust game
has not got.

- **The orders menu's own calls** (`open`, `close`, `turn`, `pageBack`, `pick`, `cycleWho`, `address`) cross to the Rust functions the tick itself runs,
  by the call table (`tools/blitz/surface.ts` `crossing`; the page stands as a surface host): the writes made between ticks go first, then the host's half
  (the cursor, the cameras), `face_call`, and the puppet takes the state again. Inside a tick, and while the puppet is the game (before a start, while
  one is due), the method runs as it was. `__rust.crossed` counts them. `give` goes with the target as bytes by the schema (`asks.give`), whatever the target is.
- **A question about a row or a seat the Rust game has not got is refused, not indexed.** `face_ask` measures the row against the order table and the
  seat against the seats before it asks, and answers 1 (no such row), 2 (no such seat) or 3 (no such question) with no bytes; an index past a table is a
  panic, and a panic is the game lost. The asks (`tools/blitz/ask.ts`) then answer the row's closure
  "nothing": its label is its name, no man can, it is not offered, an order through it is refused. One console line a row; `__rust.askRefused`.
- The page's bundle carries the call table for this (about half a megabyte). `tools/blitz/page-plugin.ts` names the bridge's files: after a pull that
  adds one, a running dev server must be started again.

## A tool that writes the game by name

A check that sets a field of `window.__game` between two frames (a lesson's step set back, a door unlocked, a man put down) writes the PUPPET. THE
RULE: what a script writes on the puppet BETWEEN its calls to step is carried into the game. On a page a tool drives (its address says `paused`:
the frames are a script's, `window.__runFrames`) the host walks every carried field of the puppet with the scan the headless tools' host uses
(`tools/blitz/scan.ts`): as a run of frames opens, or a call crosses, what differs from the last hold goes to the Rust game by the write by path
(`rustToolsWrote`); as the run closes the puppet is held again (`rustToolsHold`). Nothing is walked between the frames of a run, where the page's
own lines write the puppet (the frame's camera and HUD, the host's half, the picture's tables): those are no tool's. `window.__rust.scan` counts
the flushes, the holds, their milliseconds, and each field sent by the schema's name; with nobody writing by hand it sends nothing.

- Measured, the first tutorial seeded, 1,320 runs of one frame each on the software adapter's box: about 32 ms a walk, two a run; nothing sent;
  the block equal to the TypeScript game's at every mark. A PLAYER'S PAGE (no `paused`) pays nothing: its walk is the settings', every sixteenth
  tick, and `__rust.scan.flushes` stays 0.
- What a walk cannot carry stays on the puppet alone, as on the headless host: a field the bridge does not carry, a leaf too big to walk.
- A METHOD is not a write. `Game.teleportPlayer` called between ticks crosses to the Rust function by the call table (`GAME_CROSSES` in
  `src/host/rustgame.ts`), like the orders menu's calls; any other row of the table goes by `window.__rust.call(key, self, args)`. A method with
  no row, run on the puppet, reaches the game only as far as the fields it wrote are carried.
- A page with no seed deals a fresh one each visit (`src/main.ts`): a check that is to read the same twice names its own (`&seed=`).

## A panic does not end the session

The module aborts on a panic (an index past a table, an `unwrap` on nothing, a `todo!()` body) and the page sees a trap. It used to be the red card and a
stopped frame loop. Now (`src/host/rustgame.ts`, "a panic"; `rust/game_face/src/page.rs`, "after a panic"):

- **What is lost is the game of that call alone.** The face takes a game out of its table for the length of a call, so a panic leaves no borrow behind,
  and every game KEPT in the module (the start, each checkpoint) is whole. The page goes on in the same instance of the module.
- **The game is stopped and a card says so**, in the menu's colours: the panic's own words, the call and the tick, what Enter does and how many seconds
  of play are lost by it. The console gets ONE line a person can paste: the words, the call, the tick, the mission's clock and stage, the address, and
  the last second of input (the tick's own records, runs folded). While it is stopped nothing calls into the module: a step is no step, the HUD's
  questions answer "nothing", an order from a tool answers `the game is stopped`. The puppet stands as the last apply left it and is still drawn.
- **Enter** (read from the tick's own record of the keys) puts the run's newest checkpoint back through the save controller's own load; with none kept,
  or with Shift, the mission from its start. The pause card's Restart does the same as Shift+Enter. The game that stands is a new game of the module
  made from the kept start (`face_stand`), and the load or the restart puts the kept game back over it as ever (`face_back`: the clone, nothing of it
  lost) and applies it whole.
- **No loop.** Nothing is put back without a key. A kept game that leads straight back to a panic (inside two seconds of play, nothing newer kept) is
  not tried twice: the card then offers the mission's start, and after that the page loaded again. A panic that comes later offers the same
  checkpoint again.
- **A throw of the apply** (JavaScript, outside the module) is not a panic, the Rust game is whole: the puppet takes the whole state once and play goes
  on; the fault is named in the console and counted (`__rust.panic.applyThrew`). Three steps running: stopped, as by a panic.
- **For a check:** `?rustpanic=<tick>[,<tick>…]` plants a panic in the step that would be that tick (`x` after the number: outside any game), and
  `window.__rust.panic.plant(tick, how)` does it from the console (`face_debug_panic`). `window.__rust.panic`: `dead`, `n`, `recovered`, `lines`, `plan`.

What it does not cover: a panic while a game is being kept or put back (the kept games' own table would be left borrowed: the card then offers the page
again); a save asked from a panel while the card is up (it throws to the page's red card); a fault that leaves the Rust game wrong without a panic.

## A row the game makes at run time

The start builds the page's game with the classes' own constructors, so every row has its picture half: a man's skeleton instance with its rig,
its scratch and the views over its joint data, his animator's cast record. None of that is the game's state and the bridge carries none of it. A
row the tick makes later (a level's program stands three men at a roadblock, an alarm calls a team) reaches the page as its class's prototype and
its carried fields, and nothing else. Such a man could not be posed: `headRot` and `capsules` threw, the dress step's `men` and `snowBreath` lines
threw every tick, he was not drawn, and the poses the host owes the game for him were never worked out, so rounds fired at him did not land.

THE PICTURE'S HALF IS ITS OWN FUNCTION, and one function serves the start and a made row: `SkeletonInstance.picture(ch)` and `PoseFK.picture()`
(`src/anim/skeleton.ts`), `CharacterAnimator.picture()` (`src/game/character.ts`). Each builds only what follows from the rig and from the object's
own carried fields, draws no die and writes no state field. The constructors call them; `madeHooks` (`src/host/rustgame.ts`) is told of every made
row by the apply (`hooks.made`) and calls them on a made `Character`'s skeleton and animator. No second man is built. THE RULE FOR ANYTHING THE PAGE
BUILDS FOR THE PICTURE WHILE A GAME IS LIVE: it must not draw the game's die or write its state (an earlier form built a twin with the whole
constructor, whose animator draws a phase off the die: one draw a made man, and the page played another deal than its seed from there on).

- A gun needs none: the one field a made `Firearm` lacks, `ev`, is read by nothing.
- What a made man still lacks is the APPLY's to make right, for every class (the Library's `MADE-ROW-APPLY.md`): the class of a carried part (his
  54 springs arrive as plain objects with the right values; `skel` is given its class in the hook, a stand-in), typed leaves as typed arrays, and
  his seven named poses, which are not his pose set's and have no home in the block. No code the page runs calls a made man's springs or poses:
  `Spring` is private to character.ts and the animator's update is the Rust tick's.
- `window.__rust.made` counts made rows by class, `madeTook` the made rows given a picture half, `madeThrew` a class it could not be given to,
  `madeLast[class]` the last made row and `madeAtRunTime(row)` answers for any row.

`Game.drawMen` hands the men over one at a time: a man who throws there is left out of that frame, said once by name in the console and kept on
`game.menNotDrawn`; the men after him are drawn.

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

**With the changed mask** (the every-tick export sends only the leaves that changed), measured the same way on a box about twice as loaded, so in
ticks: room clear, the apply 0.87 to 0.89 ticks (it was 1.44), the export 0.59 to 0.63 (it was 0.52), the Rust page's update 1.3 to 1.6 times the
TypeScript's (it was 1.9); depot, the apply 1.05 ticks (1.48), the export 0.60 (0.41), the update 1.9 times (2.1). A CPU profile of the same build puts
the apply's gain at 15% and the export's loss at as much. The tables above are the build before it.

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

## The picture: what the tick computes for it, and how it reaches the page

The rule: what the simulation's own code computes from the game's state for the picture is computed in the Rust tick and carried; the dress step
(`src/host/rustdress.ts`) only hands things to the engine's objects. Nothing is worked out a second time in TypeScript.

| what | where the Rust tick makes it | how it reaches the page |
|---|---|---|
| the fog of war's block (`fogOfWar.frame`: the team's eyes and their fields, the fog's levels, the lit fog's lamps, the thin walls' bits) | `fogofwar/p_02.rs fog_of_war_pass`, `p_03.rs` (`full_block`, `lit_block`), `render_lights pack_light` | the filtered export (the block is on both lists) |
| the lights' fog classes (the team's ids and lights) | `fogofwar/p_03.rs fog_of_war_full_block` | the filtered export (`LightSet.fog_team`) |
| particle records, impulses, the fires' heat | every site of `particle_fx`, `blast_look`, `destruction`, `break_table`, … through `render_particles::particles_emit` | out events (`ParticlesEmit`, `ParticlesImpulse`, `ParticlesSetHeat`) |
| a particle record's shape (cone, burst, ring, puff) and whether it is the team's own doing (the fog's mark) | `render_particles::particles_emit` | fields of the out event `ParticlesEmit` (`o.shape`, `ours`) |
| smoke sources (a shot's gas, a stun's jets and trails, the can, a holed wall's puff) | `effects`, `combat`, `destruction`, `fire`, `smokebreak` | out event `SmokeSystemLikeEmit`, with its kind, grid and what it rides (`tools/blitz/track.ts`) |
| a burst's fragments: a wall charge's, a door charge's, a thrown can's | `blast_look/b_01.rs blast_sparks` | out events |
| which boxes are a laser's beam, a tracer's streak, an infrared line | `thinline thin_note / thin_air`, `irlaser ir_note` | the filtered export, into the page's own side tables (`thinline.ts` THIN and AIR, `irlaser.ts` IR_MADE) |
| the wind's two lists (the sources that blow; the ones for the particles) and each source's record (speed, direction, place, reach) | `wind/p_01.rs wind_update` | the filtered export (`Wind.active`, `to_pool`, the source's rows); `windOut` hands them to the |
| the weather frame, the sky's two colours, the haze, the exposure, the gloss, the box world's geometry serial | `weather`, `lighting`, `hours`, `wallbreak` | the filtered export (`HostApi.weather`, `RenderSettings`, `BoxWorld.geom_serial`). A row moves on the page only when the Rust game's moved, so what a panel set stays until the tick changes it |

What the tick has to be TOLD of the host, because a rule asks it: whether there is a particle pool (`particleFx.ts gpuFx`: with one, embers and
fragments go to it and the sparks' own pool of streak boxes stays empty), the pool's two switches a door charge's fragments go by, and whether there is
a reach keeper (a breached wall tells it). They ride in the host's half.

A wall charge's feel is in two halves. The report and the camera's kick are the Rust tick's (`blast_feel`). Its particles (the fireball, the dust
ring, the three pushes, the ceiling's dust) are queued closures in the TypeScript and stay the page's: the host starts them on the report's own out
event (`WallFeel`, `OUT_HOST.wallFeel`) through `blastFeel.ts wallFeelPicture`, and the dress step lets
their later stages go (`blastFeelStages`).

MEASURED (software adapter; nothing here has been seen on a GPU). Four tapes, each played by the page without the word and played back on both pages
with one set of readers: the office room clear twice (2,150 and 2,031 frames: a door, the squad sent in, shots, a smoke can, a frag, a stun), a second
frag and stun tape (899), and mousehole with a wall charge set and blown (3,769). At every point of all four (52, 52, 28, 76) the die, every part of
the game's block and the poses are equal, and so are: the fog's block field by field, the team's ids and the lights' fog classes, the smoke solver's
wind inputs and the pool's, the weather frame, the sky's rows, the camera's kick, every bright box and the whole light list. Particle records are
equal number for number and shape for shape over each whole run (301, 374, 144) but for the fog's mark on a round's chips off a wall (9, 9, 1
records: below). Decals are equal over the wall run (52) and not at a frag (below).

DRAWN (640x360, one browser, both pages off the same tape): the wall charge, sixteen consecutive frames from two before the blast: mean difference a
channel 0.13 to 0.18 of 255, at most 0.09% of pixels over 32, after the first (0.68: the first frame a page ever draws). The same flash, lit room,
hole, cut paint and burst; single sparks stand in other places. A pair taken as the one drawn frame after hundreds undrawn is NOT a fair picture of
either page: the pool gives birth and the solver steps only when a frame is drawn, so everything filed since the last drawn frame starts at once and
the pool's cap picks by the queue's order; two shots of ONE page at one frame taken that way in two runs differ by 4.9 of 255.

The office room clear, seven single shots off one tape: the squad going in, a frag going off and a sixth of a second on, a stun the same: 0.04 to
0.06 of 255, no pixel over 32. The first frame a page ever draws: 2.5 (thin outlines of every edge and noise in the lit room: the two pages stand
in another phase of the frame's jitter; the men are in both pictures).

THE COST on this tree (the built page, room clear with four, 1,800 undrawn frames with no input after 600 to settle, box load 4.5): 5.43 ms a frame
without the word (`Game.update` 4.11), 7.08 with it (5.54: the Rust tick 1.78, the export 1.12, the apply 1.76, the dress step 0.32, the rest the
host's half, the knobs and the hand-overs); 192 K bytes a tick. One run; depot and the profile's method were not run on this tree.

WHAT STILL DIFFERS, each with its cause:
- a frag's scorches and floor films: `BangScorch` and `MessBurst` run marks.ts and floormess.ts on the puppet AFTER the tick, and both read what
  stands round the burst, which the blast has by then thrown or broken (2 scorches missing, 2 films larger, a later film refused as crowded).
- the fog's mark on particles a host-run event throws (`WallWearRound`: a round's chips): the shooter's `begin` is over when the event runs.
- a wall charge's report is heard up to 0.25 s early and its three later fans of fragments one and two frames early (queues of closures in the
  TypeScript; the generated queue rows hold no data).
- a stun's smoke trails are 10 where the other page has 12, and a holed wall's puffs a few more: both ask the host's solver for its budget, and
  the tick has no answer of the host's.
- a smoke can's and its vent's source does not ride the can while it rolls (no arm for an item in the Rust `Track`).
- a man's cigarette has no thread of smoke off its end (`smokebreak seam_thread_track` answers nothing: no arm for a hand in the Rust `Track`).

WHAT A SMOKE SOURCE RIDES. A source that hangs on a thing that moves is asked every frame where that thing is now (`SmokeEmitter.track`, read in
`src/smoke/smoke.ts update`): that one question is the whole rule, and nothing else places such a source. The tick names the thing (`effects::Track`:
whose muzzle, whose ejection port, which spark), the name crosses in `SmokeSystemLikeEmit`, and `track.ts` makes the question from it on the page's
own objects. Until this was mended the import skipped the name (`import.ts readType`, a `wordEnum` with nothing standing where it lands), so on the
page every such source stayed where it was filed: the wisp of a shot hung in the air where the barrel had been while the man walked on, for its two
and a half seconds, and a stun's ten trails all stood at the burst. `tools/blitz/track-check.ts` holds the barrel, the port and the sparks
(`TRACK_BREAK=1` fails all three). Not seen in a solver or on a GPU: the check reads what the solver is told.

## Checking it

`tools/blitz/page-walk.cjs` walks the page in headless Chromium the way a player goes: the title, Play, a squad mission's row, Deploy, an order by the
command key, the ring held up on a door (its caption must name the door, its first wedge must be the door's own order, a wedge's digit must give that
wedge's order, on its second page too), a floor spot, a frag; then the solo night to its first checkpoint, a save by hand and its load. `PORT=5173 node
tools/blitz/page-walk.cjs` with a dev server up; every line is PASS or FAIL and the exit code says which. The tutorial missions by a hand's own keys
are `tools/qa/tut-*-page.cjs` (and `tut-page.cjs tut-sky`).

A scripted player in the page (the same hands as `tools/qa/lesson-run.ts`, over the page's own Input) plays the address without the word and keeps every
frame's input; the address with the word plays that input back; the die's count and the hash of each part of the game's block are held together every 60
frames. Software adapter, undrawn frames: this holds the simulation, not the look.
