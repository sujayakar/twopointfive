# rust/game_start: the game's own start, and tests that stand a scene

The simulation's tick is Rust (`rust/game`), but until this crate a game could only *begin* in TypeScript: the level was loaded, the men made and
the `Game` put together by `src/host/standup.ts`, and the result handed across the bridge. Here is the start in Rust. A level file's text and the
character's files go in, a `Game` the tick can run comes out, and nothing in between was made by TypeScript.

Two things stand on it:

- **A test of a behaviour in ten lines.** A scene (rooms, doors, lamps, men) is said in Rust, stood, run for a few seconds and asked what a man
  did. `cargo test` in this folder runs them all in about half a minute.
- **The road to starting every level in Rust.** The scene builder is not a second way to make a game: it WRITES A LEVEL FILE, and one start stands
  any level file. What the start does not carry yet it refuses by name (below), and the gate's `start` rows hold what it makes to a pin, part by part.

Nothing here has been seen on a GPU: the host these games stand on draws nothing.

## Write a behaviour test

```rust
use game::game::GuardState;
use game_start::play::{Opts, Play};
use game_start::scene::{Axis, Scene};

#[test]
fn a_guard_who_hears_a_shot_behind_a_shut_door_goes_to_it() {
    // two lit rooms with a shut door between them; a sentry with his back to it, Sam next door
    let mut s = Scene::new("shot-next-door");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("between", Axis::Z, 18.0, 12.4).door("east-yard", Axis::X, 16.0, 21.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0);
    s.sentry("post", [12.25, 13.25], [10.0, 13.25]).sam(23.75, 13.25);

    let mut p = Play::scene(&s, &Opts { seed: 1, ..Default::default() });
    let post = p.guard("post");
    p.run(1.0);
    p.sam_fires_at([25.9, 13.25], 0.6);                               // one rifle round into the far wall
    p.until(1.0, |p| p.state(post) == GuardState::Alert).expect("he hears it");
    p.until(5.0, |p| p.at(post)[0] > 15.0).expect("and goes to the door");
}
```

(That example is `tests/readme.rs`, so it is held to compile and pass.) `tests/behaviour.rs` is the worked set. Each test there holds one rule over three seeds, beside a CONTROL that differs in the one thing the rule
is about, and says above it what a failure would mean. A seed is another SCENE as well as another die: in scenes this small the die is hardly
drawn, so each test moves a mark, a range, a door or a landing point by the seed (`nudge`).

| test | the rule | its control |
|---|---|---|
| `a_calm_guard_sees_a_man_in_light_and_not_in_the_dark` | a sentry with a lit man 6 m in front of him goes suspicious, then alert, his fix is where the man stood, and he calls the contact once | the lamp off: he never notices |
| `a_defender_runs_from_a_live_grenade_and_gets_down` | he shouts once, runs from it, crouches, and is at his farthest from it when it goes off | no grenade: he stands on his mark and says nothing |
| `the_pair_ordered_to_a_door_stacks_on_its_two_sides` | the pair given the order ends one man each side of the door, against its wall | the pair given no order does not go with them |
| `a_posted_defender_holds_his_room_when_a_shot_is_fired_outside` | he hears it, and ten seconds on he is still inside the room he holds | no shot: calm on his mark |
| `a_guard_who_hears_a_shot_behind_a_shut_door_turns_to_it_and_goes` | he says what he heard, believes the shooter is in the room the shot was fired in, and leaves his post for the door | no shot: calm with his back to it |

### The scene (`src/scene.rs`)

Metres, in the world a level must fit (40 by 28; x east, z south). The pieces are the level kit's own sizes (`tools/levels/lib/levelKit.ts`): a wall
12 cm thick and 3 m high, a real door in a 1.2 m slot with its frame, a bare bulb on its flex.

| call | what it says |
|---|---|
| `Scene::new(name)` | an empty lot at night: the ground and no light |
| `.room(id, x0, z0, x1, z1)` | four walls, a floor and a ceiling; two rooms that share a line share the wall |
| `.wall_x(z, x0, x1)` · `.wall_z(x, z0, z1)` | a wall on its own |
| `.door(name, Axis::X \| Axis::Z, line, at)` | a door in the wall on `line`, its slot starting at `at` along it, shut; `.locked(name)` |
| `.bulb(x, z)` · `.bulb_off(x, z)` · `.moon()` | light. With none, a place is black |
| `.sam(x, z)` | Sam's spawn |
| `.patrol(name, points, wait)` · `.sentry(name, at, toward)` | a guard on a route · a guard posted, looking one way |
| `.team(sam, yaw, marks)` · `.defender(room, at, watch_door)` | a room clear: the team's start (a yaw of 0 looks south along +z, π/2 east, π north), and a defender posted in a room |

A scene is one of two kinds, never both: a NIGHT (Sam and guards on routes, as a level's file has them) or a ROOM CLEAR (a team and posted
defenders, started by the game's own `startScenario`). The room clear's staging takes every man who is not its own off the floor, so the builder
refuses a scene that says both. A scene has at least one door (a level's mission names one). A ROOM CLEAR IS REFUSED BY THE GAME (the start
stops with the staging's own words, `roomclear/p_01.rs`) unless some room has more than one way in and every room has a defender: two rooms
side by side with a door between them and a door each to the yard, a defender in each, is the smallest that stands (`tests/orders.rs yard()`).

`scene.level_text()` is the level file. It can be saved under `levels/`, and opened in the editor.

### The game in a test's hands (`src/play.rs`)

- **Stand:** `Play::scene(&scene, &opts)`, or `Play::level("kit-yard.json", room_clear, &opts)` for a file of `levels/`. `Play::mission("depot.json", "?levelFile=depot.json&roomclear&squad4", &opts)` starts a mission on a file from the words of its address, as a click on its row of the mission list does (the row's hour and weather, the fog of war, then the mission).
- **A player at the keys:** `src/player.rs` is a scripted player of a seat: async code with no executor, an await is one tick, who acts only through the hand of the next tick and asks the game through a borrow that cannot outlive the question. `p.played_by(&mut script)` is one tick with his hands on the first seat, by the road the page's own goes; `p.played_out(&mut script, secs)` plays him to his end. The hands-off player (`src/hands_off.rs`, a room clear from its first step to its end) is such a script, and the page's Benchmark plays him.
- **The tutorials, played:** `src/lessons/` has a file a tutorial, its steps by the block's ids, each what a person does for the line that is up, with his hands alone, and as mortal as a player. The driver is one: half a second of nothing and the step must still be up, the deed, and the step must have gone by; after the last step the run must end. `tests/lessons.rs` plays them (`TUT=<id>` picks one, `PACE=<secs>` has him read each line), and holds each block against its floor, that a checkpoint carries the step that is up, and that a restart puts step 1 back. `played_by` does what the page does with a tick's checkpoints.
- **Time:** `p.run(secs)`; `p.until(secs, |p| …)` answers how long it took, or `None`; `p.always(secs, |p| …)` answers `None`, or the second it
  first failed; `p.frame()` is one sixtieth.
- **Who:** `p.guard("route name")`, `p.defender_at(mark)`, `p.mates()` (the first two are the red pair, the last two the blue),
  `p.roll()` (every man in a line, for a failure's message).
- **Ask:** `p.at(man)`, `p.state(man)`, `p.awareness(man)`, `p.fix(man)`, `p.facing(man)`, `p.speed(man)`, `p.low(man)`, `p.stacked(man)`,
  `p.up(man)`, `p.sam_at()`, `p.sam_light()`, `p.frag_fuse()`, `p.floor()` (what stands on the level: round things, shaped boxes, trim, civilians, devices); `p.lines(man)` and `p.said` are every line a man spoke (read off the speech
  bubbles after each frame), `p.sounds` every sound the game asked its host to play, `p.end_cards` when it told its host to open the end card. `p.debrief_view(seat)` is what the end card shows that seat.
  The mission: `p.view()` and `p.view_of(seat)` (all a HUD is told), `p.objective()` (its row), `p.log()` (the lines on the game's log), `p.room_clear_end()`, `p.hostiles()`; the par
  clock: `p.par_live()`, `p.par_bars()`, `p.par_started()`, `p.par_result()`, `p.par_counts()`.
- **Do:** `p.order("stack", place, Who::Red)` is the order menu's own call; `p.frag_lands(place)` the sandbox's; `p.sam_fires_at(place, settle)`,
  `p.point_at`, `p.press`, `p.key_down` move Sam as a player's hand does, through the host's Input. An order pointed at Sam's own spot has a
  target of no kind, and `move` is refused there: point it somewhere else. A flash has no call of its own yet: the sandbox's is
  `game::game::game_drop_throwable(&mut p.g, game::game::ThrowKind::Flash, [x, 0.0, z])`.
  `p.falls(man, by)` puts a man down by combat's own call, for a test of what follows a death. With `Opts { seats: 2, .. }` a second player sits in: `p.seat_orders(1, …)`, `p.seat_steps(1, metres)`, `p.seat_stands(1, place)`, `p.seat_at(1)`.
  `p.man_fires_at(man, place)` is a round from a man by combat's own call, `p.falls_as(man, how, quiet, by)` a man put down any way combat has, `p.seat_goes_down(seat)` a player out of hits with no round fired. The recap: `p.recap_card()`, `p.recap_deaths()`, `p.recap_lost()`, `p.cue_over(man)`.

Three rules keep the tests honest and alive:

1. **Ask through `play.rs`, not the game's fields.** When the AI's fields move, one reader follows and no test does. A test needs a new question:
   add a reader. (`tests/control.rs` and `tests/orders.rs` break this where no reader stood yet: a man's reload, the rule that has him read
   out of `control_line`, a module's own call. Do not copy that; add the reader.)
2. **Do through the game's own calls.** An order by the menu's call, a shot by Sam's trigger, a grenade by the sandbox's call. Never a write to
   a man's awareness or a pushed event: that tests the test.
3. **Hold a rule, not a run.** Several seeds, a control, and what a man DID (where he went, what he said), not the name of the script he was on.

A game senses for itself on every host, so "lit", "dark" and "behind smoke" are the same thing here as on the page.

**One game at a time in a process.** `rust/sim` keeps a game's block and several tables in statics, so a `Play` holds a lock until it is dropped
and `cargo test`'s threads take turns. A test that stands a second game drops the first (`drop(p)`), or is told so.

## The start (`src/stand.rs`)

`stand(level, assets, opts)` is `standGame` of `src/host/standup.ts` on a host with no picture, in its order:

| step | who does it |
|---|---|
| the block of `rust/sim` the game stands in, zeroed under the layout's word | `stand.rs` |
| the character: skeleton, skin, clips, the cast's meshes and variants | `gltf.rs`: `src/anim/gltf.ts` in Rust, from the `.gltf` files' bytes |
| the host: an empty box world and light set, the picture's settings | the game's `box_world_new`; `settings.rs` (a copy of `defaultSettings()`) |
| the level file read into the `Level`, its boxes and routes as rows | `level.rs`: `tools/levels/lib/levelLoad.ts loadLevel` in Rust; the records the game keeps as its own types (a mission's `scenarios`, the footsteps' map, devices, civilians, vents, the drill plans) and the file's own records the game crate has a type for (a box with its shape and markings, a light, a door) through `read.rs` and `records.rs`, a line a type |
| thin things and window bars marked; each box that says a shape given its record and its trim; statics into the world, lights into the set | `passes.rs` (`thin.ts`, `bars.ts`, `shapes.ts`); the game's `cut_record`, `box_world_add_static`, `light_from_def` |
| the die, seeded before the men are made | the game's `sim_dice_new` |
| collision, the walk grid, sound paths, fixtures, doors, guards, civilians, orders, the fog, the room fight, every module | **the game's own constructor, `game_new`** (`rust/game/src/game/p_04.rs`), which had never run before this crate |
| the moving state at home in the block | `settle` (the die, the pose pool, the smoke grid) |
| the host's Input and camera | the game's `input_detached`, `FollowCamera`, `follow_camera_rebuild` |
| the address's words, where one is handed (`StartOpts.link`), BEFORE the game is made | the game's `address_before_game`: the row's own hour and weather onto the words, the weather the game is born in, the visit's hour (`src/host/standup.ts addressBeforeGame`); then `difficulty_of_address`, the difficulty it asks for (`&hard`, `&difficulty=`, the levers, `&digin`, `&pressure`, `&fogcone`), on this game's own knob tables, unless `StartOpts.difficulty` is false: a host that handed commands (`stand_handed`) has read its address onto them itself (`src/host/standup.ts difficultyOfAddress`) |
| a mission, where asked | the game's `start_scenario`; or, from an address, `start_at_address`: `start_from_query` on its words, then the solo card's question, as a click on a row of the mission list starts it on a page |

`frame(g, host, dt)` is one frame of that host: the game's `game_update` on the host's Input and camera, the camera after the man, the lights'
envelopes. It answers what the step told its host (sounds, marks, lines said).

NOT MADE, because no rule of the tick reads them: the character renderer, the mesh shadows' tree, the proxy fits of the rig.

### What the start reads of a level file, and what it does not

Every file under `levels/` is read whole: `start --lacks levels/*.json` (the bin, below) names for each file what the reader does not carry of
it, and today it names nothing. `stand` REFUSES a file with anything on that list: a level stood without part of itself is another level.

- **The file's own tree keeps the account** (`json.rs`). An object remembers which of its members were asked for, so when a file has been read,
  whatever nobody asked for is named by its place (`doors[0].auto`, `elevators`): a record the reader has no line for, a member a record gained
  since. There is no second list of a record's members to keep beside its reader. What the loader leaves behind by rule, as the TypeScript
  loader does (a floor, an author's notes, the editor's block), it passes by in so many words (`skip`).
- **A record the game keeps as its own type is read by that type** (`records.rs`): one line says which member of the file each field is, so a type
  that gains a field does not compile until its line names the member. A box, a light and a door are read as the file's own record types
  (`BoxData`, `LightData`, `DoorData`) and then laid as the loader lays them. Props, switchables, routes, volumes and the mission's objectives have
  no record type in the game crate and are still read member by member.
- NOT READ: `connectors` (a stair or a ladder between floors: the TypeScript loader passes them by too, and no file has one).
- NOT CHECKED: the file's own rules (`validateLevelData`). A file is taken as one the TypeScript validator has passed; a wrong value stops the
  reader with its place in the file, a wrong level does not.

Every row of the mission list is stood, twice over, by the gate's `start` rows ("The gate", below).

## What stands between this and the page starting a level in Rust

A list, not a plan. Today the page stands the game in TypeScript, sends the whole state across once, and keeps the TypeScript `Game` as a puppet
that the picture, the sound, the HUD and the menus read (`src/host/rustgame.ts`, "A START").

1. **The address.** DONE IN THE GAME, NOT YET ON THE PAGE. The page starts a mission by `startFromQuery(g, search)`: the row's hour and
   weather, the fog of war, a quiet mission's fog rung and cameras, the team's size, then the mission. An address's words are a type of the
   game crate now (`rust/game/src/query.rs`, `Query`: the browser's rules for reading the text and the calls the game makes on it, `has`,
   `get`, `set`, `delete`, held to the browser's own reader case by case), every reader of them takes it (`start_from_query`,
   `spec_of_query`, `hour_from_query`, `weather_of_search`, `fog_of_query`, `quiet_rung_of_query`, `row_settings`, the loadout's, the
   gear's, the words'), and the one stand-in each module kept for `has` and `get` is gone, with three private copies of the text reader.
   `StartOpts.link` is the address's text and `stand` starts the mission by `start_from_query` on it; the gate's `start` rows start every mission from
   its row's link.
   The address's difficulty is read too, by one reader on each side (`difficulty_asked`, `difficultyAsked`). `tests/hard.rs` holds the rule.
   What is left: the page hands `location.search` across once and the Rust game starts from it. Two things to know when that is written.
   An address's words are NOT on the wire (the table declares that arm of a `string | URLSearchParams` argument as holding nothing, and a
   `Query` writes nothing and reads back empty): a host hands the TEXT. And the page reads more off the address before the stand-up. THE
   WEATHER AND THE HOUR ARE DONE (2026-11-07): `address_before_game` reads them onto the blank game before `game_new`, as the page's
   `addressBeforeGame` does, and a game
   stood from an address no longer has the harness's solo card put on at the stand (`stood.solo_stand` stays down, as on a page), and the
   address's own question is asked once the mission is staged. STILL THE PAGE'S ALONE: the kit this machine keeps and the words. The game
   crate's readers of those were called by nothing and went with the dead code; they come back, taking a `&Query`, with the page's start.
2. **The puppet's objects.** The picture reads TypeScript objects that only the TypeScript start builds, and the apply writes onto objects it
   finds: of a man, his rig, his row of the men's columns, his capsules and his entry in the light probe's table; of the level, the box world
   the GPU is packed from (`BoxWorld`: the pack, the cut and round tables, the glass listed to the GPU), the light set, the marks' texture, the
   fixtures, the wall skin. A game started in Rust has none of them on the page's side. (The same gap shows for a man made while a game runs.
   This item is from reading `rustgame.ts` and from the host's own notes, not from a page run: nothing here has opened a page.)
3. **What this start leaves unmade because no rule of the tick reads it:** the character renderer, the mesh shadows' tree, the proxy fits of
   the rig (`rig_fits`), a snow level's field and prints, the trig memos.
   The page needs every one that is the picture's.
4. **The dress step** (`src/host/rustdress.ts`, `src/game/dress.ts`): what the TypeScript tick did for the picture only still runs in
   TypeScript on the puppet. This start stands with those rows down.
5. **The host's own half.** `stand` makes a host that draws nothing: default picture settings (`settings.rs`, a copy of `defaultSettings()`: the
   page's are the player's own, with his quality preset), a canvas of 1600 by 1000, a detached Input, no smoke solver (the page builds its own
   between the level's boxes going to the world and the Game). The page would hand each of these in.
6. **The character's files twice.** `gltf.rs` reads the rig, the clips and the cast's variants for the simulation. The picture's meshes and
   skins are read from the same files by `src/anim/gltf.ts`. One reader, or two that must agree.
7. **Which file.** The page fetches the level's file. Here the caller hands
   the file's text in. (The office was the one level whose missions were not in its file: they are now, and it starts as the others do.)
8. **The file's checks.** `validateLevelData` is TypeScript. A page that starts in Rust still validates in TypeScript, or the rules move.
9. **What stands a game again from outside a tick.** A mission staged from a menu, a show, a kit view that sinks boxes: each is a TypeScript
   call on the puppet today, after which the Rust game is started again from the puppet. With a Rust start each becomes a call into the module.
10. **Two players.** `StartOpts.seats` reaches `game_new`, and the gate's `start` rows stand every mission as a game of two: those the game stages for two
    are pinned part by part, and those it refuses for two are pinned with its words. Nobody at the second seat's controls, no server, no rollback here.
11. **The module.** DONE AS FAR AS THE START ITSELF. The page's module carries this crate (`rust/game_face` links it without its `std`
    feature), and `face_start` stands a game inside the module from one run of handed bytes (`src/handed.rs`: the level file's text, the
    address's words, the character's files, the host's commands). Nothing plays a game started that way yet: item 13.
12. **A MISSION'S TEAM NUMBERS BELONG ON THE GAME, AND THE RUST START IS WHERE THAT LANDS.** Today they are written into the process's knob
    tables, so a game whose start does not write a table finds it as the last mission left it: the one thing a second start in one process
    still differs in. **The team's start is the office's until a
    start says otherwise.** The rescue's and the night breach's knob tables are made holding the
    office's team start (`samAt`, `samYaw`, `opsAt`): the TypeScript reads them off `levels/office.json` as the module loads, the Rust holds
    them as the table's values (`knobs.rs`), and each keeps `applied`, the start the knobs were last written from, so that a start on a level
    that names another writes its own over them and a knob moved by hand stays moved. A game made in Rust is made before any level, so it
    cannot read a file for them. The end to aim at: the team's start is the level's alone (`level.scenarios`), and the knob is an override
    that starts empty. That touches, in both languages: the two knob tables and their rows in the registry and on the wire, `applied` and
    the compare in each start (`rescue.ts`, `nightbreach.ts`; `rescue/p_01.rs`, `nightbreach/p_01.rs`), the debugger's rows for them, and
    every check that moves a team's start by its knob before a start. It goes with the start moving to Rust, not before.
13. **THE PUPPET'S BOOK IS NUMBERED BY THE TYPESCRIPT'S WALK, AND A RUST-STARTED GAME BY ITS OWN MAKING.** The one thing a game started in
    the module needs that the old road never did. The bridge's book (`tools/blitz/export.ts Book`) gives a TypeScript object its number when
    the export's walk first meets it; on the old road the Rust game is MADE FROM those numbers, so every later apply finds its object. The
    Rust start numbers a row when it makes it, and keeps rows that were made and dropped (a gun the kit replaced, the men a staging took off
    the floor). The two starts were the same game BY REACH; by raw row number they differ in tens of
    thousands of paths (clips, characters, lights, guns, guards, the players). A tally played on a module-started game with the puppet's book
    as it stood stopped in half a second on the wrong object. Two ways out. The book rekeyed to the Rust game's rows at the start (the
    pairs by reach, which `diff.ts index` numbers for a tool; the host's own Input and camera paired by role, since neither walk reaches
    them before the first update; the rows only Rust holds made or left out): a puppet one program built paired with rows another made, by
    the shape of a walk, and thrown away the day the page makes its picture from the game's rows. Or no puppet kept at all: every object
    the picture reads MADE from the Rust game's rows, as a man the tick makes at run time already is (items 2 to 6). THE SECOND IS THE WAY:
    a start, a restart and a man made in play then become one act. Until then a module-started game is whole for a host with NO puppet (a
    game server stands N seats from a level's text today), and the page keeps its road.

## Levels: what a level owes whoever plays it (`src/bin/levels.rs`)

```
cd rust/game_start
cargo run --release --bin levels -- second-way [level …]   # every room the defenders can shut has another way in
cargo run --release --bin levels -- sight [level …]        # a defender sees what he watches
cargo run --release --bin levels -- walked [level …]       # its starts and its doors are on a way
cargo run --release --bin levels -- stacks [mission …]     # every stack mark is by its door
cargo run --release --bin levels -- stands [level …]       # what it names stands on its built floor and can be walked to
cargo run --release --bin levels -- doors [level …]        # every door is a real door's size, and the walk grid goes through it
cargo run --release --bin levels -- charges [level …]      # its places for a wall charge, held by count
cargo run --release --bin levels -- names [level …]        # it is named by its own rooms
cargo run --release --bin levels -- rain [level …]         # its storm rains by its own roofs
```

Each stands the level (or every mission of it) by this crate's start and asks the game's own rules; the head of the file says what each holds and why.

## Hold: is this build the same game as that one (`src/bin/hold.rs`)

For a change that is meant to change nothing of play. It needs no page, no TypeScript, no module build and no file of the wire, so a
change of the wire does not strand it.

```
cd rust/game_start
cargo run --release --bin hold -- keep /tmp/held        # on the tree BEFORE the change: play, and keep what the game was after every frame
cargo run --release --bin hold -- check /tmp/held       # on the tree AFTER it: play the same again, and compare
cargo run --release --bin hold -- check /tmp/held --but PART,PART   # … with the parts the change is meant to move left out
```

`keep` stands each level by this crate's start, plays it for 90 s under each hand of `hands/`, and writes, for every frame, the
state's words (`game::state_words`: a hash of every part of the game that a host does not hand in) folded to one number and one
byte of each part. `check` reads the folder's own list of runs and its copies of the hands, plays them on the build it is, and says
of each run `the same after every frame`, or the first frame that differs and the names of the parts that differ in it
(`FIRST DIFFERS AT FRAME 2181 (36.35 s) in guards, chars, dice`). It exits 0 only if every run is the same. Levels, hands, seeds and
the length are arguments (`keep DIR --hand push --seed 1 --seed 2 --secs 60 depot:clear office`); with none it is 24 levels (14 as
room clears) by every hand by seed 1: 120 runs, about ten minutes on four cores and 1.7 MB a run (`--hand push --hand bangs
--hand rest` is the 72 of the first proofs, six minutes).

A hand is a few lines of what is done to the game from outside (`hands/*.hand`; the file's head in `src/bin/hold.rs` is its
grammar). Seven stand here, and each file says what it reaches and what it does not:

| hand | what it does | reaches |
|---|---|---|
| `push` | Sam unhurtable; every 6 s the squad is ordered to move to the nearest man who is not theirs; every 10 s Sam fires a round at him | the move order, doors, contact, the room fight on both sides, hunting, reloads, deaths |
| `bangs` | Sam unhurtable; a frag and a flash land by the nearest such man, again every 9 s; the squad falls in | the body rules (running from a grenade, shaken by a blast), flashed men, blasts on doors and glass |
| `doors` | Sam unhurtable; the squad stacks on the door nearest that man and is told to go, falls in, then a pair bangs its way in while the other holds the threshold | the stack order, the word to go, the door tools, entries and the room fight behind them |
| `kit` | Sam unhurtable; the squad goes silent, closes, throws a flash and a frag on order, covers, goes loud, holds, falls in | silent and loud, the throws' page, cover, hold; smoke and the wand where a floor brings them |
| `rest` | nothing | a floor at rest: patrols, sentries, lights, the level's own script |
| `fires` | Sam unhurtable; every 10 s he fires a round at the nearest such man | a floor with no squad, woken by shots |
| `pair` | Sam unhurtable; the partner is told to take the nearest such man, and to watch him | the partner's take and watch, where a mission has them |

A run's line says how many of the hand's lines the game took, and names each line it NEVER took on that floor with the game's own
reason (`NEVER DONE HERE: order shutDoor door all (it's shut)`), so a hand is not trusted for what it only names. On a floor with
no squad the three hands that give orders do nothing but what Sam does himself.

What a green check proves: on these floors, under these hands, the two builds played the same game part for part. What it does
not: anything no hand reaches (charges, a pair's orders, the 40 mm, a startled man, a player walking, a second seat), and anything that is not in the
state's words (what a host hands in: the cameras, the input).

A change that is MEANT to change play is not judged here: that is the mission tallies' job.

**Parts are compared by name, and one can be left out on purpose.** A kept folder carries the names of the parts it was kept
with, so a build that has gained a part, lost one or re-ordered them is still compared on the parts both have, and each run's
line says which left the state and which are new. For a change that is MEANT to move one part and nothing else (a record that
changes shape, a table numbered anew), `check DIR --but tab.orders_order_def,seats.orders` leaves the named parts out: every
other part must still be the same after every frame, and the last line says what was not compared. A name that is a part of
neither build is refused. Its first use: the orders' table no longer growing with each listing was shown to be the same game on
120 runs but for those two parts, where the whole comparison parted at frame 1 on all 70 runs that have a squad.

A build is the same as itself with every part in: the box mirror's record, once left out because it kept addresses, is hashed
with the rest.

## Missions: what a change of play does (`src/bin/missions.rs`)

`cargo run --release --bin missions > before.tsv` on the tree before a change of play, the same to `after.tsv` on the tree after it, then
`… --bin missions -- diff before.tsv after.tsv`: thirteen room clears as the page stands them, standard and hard, eight seeds (208 runs, three
minutes on four cores), under a commander who gives the squad the same orders on every build. A row says who was up at the end, when the first
man fell, when the floor was clear, the rounds and reloads of each side and how long each rule of the list had an enemy. The file's head says
what it does not see: stealth, Sam's gun, the rescues, a commander who waits. `hold` says whether a build is the same game; this says what the
other game came to.

## The gate: what a build is held to (`src/bin/gate/`)

One native command, with nothing of TypeScript in it: no page, no module and no bridge. It stands every mission on the game's own list and plays a few of them, each as the page and the server
stand a game (`dress`, seed 9, the mission's own link).

```
cd rust/game_start && cargo build --profile gate --bin gate     # its own profile: 245 s from nothing, 7 s after an edit in game_rules, and a run in 76 s; a pin is the same under any profile
target/gate/gate                 # every row, four at a time. Exit 0 = all pass · 1 = a row failed · 64 = a wrong word
target/gate/gate save            # only rows whose name starts with a word: `gate start`, `gate "start depot-four"`
target/gate/gate pin [ROW…]      # record the pins again and say which moved; nothing moved = the file is as it was, byte for byte
target/gate/gate breaks          # each row of the breaks' table plays unbroken and must PASS, then each break must FAIL in its check's own words. Exit 0 = all did
target/gate/gate list            # the rows
target/gate/gate tests [BINARY [TEST]]   # the whole native test set: no part of the quick kind
```

Each row prints one line in the rows' order, and the last line is the count and the wall time, with what each kind cost. A row is a child process, so a game that panics costs its own rows, which then
FAIL with the first lines of what it wrote; the four rows of one run share a child, because the run is played once for all of them. About a minute and a half on four cores.

| row | what it holds | its break |
|---|---|---|
| `start <id>, one player` · `, two players`: every row of `all_missions`, twice | The start is the pinned start, part by part, or the same refusal in the same words. A second start in one process is the first: 5 s idle, every part each half second, then the mission before it on the list stood and run 2 s, then the mission again, the same | `pin` a word of the pin made wrong · `second` the second start's seed is one more |
| `play <run>` | 40 s under a hand: one word a second (every part folded as `hold` folds them) against the pin. Red if a line of the hand never happens on that floor, with the game's reason | `pin` a word of the series made wrong · `silent` a line is said to have never happened |
| `again <run>` | The run stood and played twice more in one process, 10 s of the next run on the list between the two: the same word every second | `second` the second run's die is drawn once before its first frame |
| `save <run>` | THE GAME PUT BACK IS THE GAME THAT WENT ON: a `Moment` at 3, 9, 15, 21 and 27 s, each stood again (the odd ones in slot 1) and played 3 s under the same hand: every part of every frame the same. What a host keeps beside a game (where the hand's next line is, `Play.cursor`) goes back with it. | `block` a moment whose block bytes are the run's end's |
| `load <run>` | A LOAD: the moments of 9 and 21 s, the value a save keeps, the run's game gone and another mission played 2 s in the process, then stood as a page stands a kept game in a slot nobody was in and played 3 s under the same hand: every part of every frame the run's | `block` one byte of the block's die is another |
| `draw <run>` | Everything drawn after every tick, at 0.5 and at 1.0. (a) the word every second is the undrawn run's. (b) a mirror of the page's buffers, filled ONLY as the page fills them (`src/bin/gate/draw.rs` lists them), is the module's own tables after every frame. (c) the boxes' head says what their lists hold. (d) once a second, what the mirror holds is what a host that has just opened is told, but for what is a table's history (`draw.rs` says which). Played as hosts play: some frames come after two or three ticks, the look is said twice, and a moment of 18 s is put back at 20 s and the run goes on, its word still the run's | `told` a die drawn once between the undrawn run and this one · `mark` the mirror skips `from..to` on the frames that carry one after the first · `head` the head made wrong before it is read · `opened` a word of the fresh host's made wrong |
| `seats <id>` | A fight of two is its records: scripted hands, each tick's two commands through `cmd_text` and back. Fresh games play the records with nobody sampling, showing seat 0 and then seat 1, and a moment at 12 s is put back and played to 18 s: every part the same each half second. The series is pinned. The two players' hands differ on one tick in five at least, each man walks and the fight fires at least so much (a small fight: its hands walk a man 4 to 8 metres and fire 2 to 8 rounds), and more than half the seconds have different words | `pin` · `fresh` a fresh game's replay hands each seat the other's records · `moment` the moment's does · `idle` nobody's hands do anything |
| `par <id>`: every mission whose level has `scenarios.par` | The clock moves nobody: 30 s under `push`, with the file's par block and with it taken out of the tree before the stand. The parts differ in nothing but the table `CLOCK_ONLY` in `rows.rs` names, each with its reason; the clock's own part does differ, and the level's own record stands on each side as it stood | `moves` a die drawn once on the clocked side only · `noclock` the block is out on both sides |

The runs (`RUNS` in `rows.rs`) reach the office's night breach under `doors`, a room clear in the rain (`depot-four`) under `bangs`, `compound-four` under `doors`, `mousehole-four` under `kit`,
`glasshouse-four` and `killhouse-four` and a rescue under `push`, a quiet night (`bazaar-quiet`) under `fires`, the snow (`pass-four`) under `bangs`, and the lesson of a partner's orders (`tut-pair`) under `pair`.

**`gate tests`** builds the tests of every crate that has any (the list is at the head of `src/bin/gate/tests.rs`, and a crate with tests that is not on it fails the run) and runs each binary as a process of its own from its crate's folder, as many at once as there are cores. It prints each binary with its counts and seconds, every red test by name with its panic line, what is ignored and why, the ten slowest and a last line. Exit 1 on a red, a binary that ended badly or a build that failed. `gate tests squad_doors [test]` is that binary alone, or that test. `--break=BINARY` plants a red, to show that one is read.

**What it does not hold.** A row holds a game to itself, to its pins or to the same game by another road, never that it is any good, and a mistake made at a pin passes: `start` a game at rest, `play` and `again` what the eight hands reach, `save` the three seconds after each moment, `seats` a fight nobody plays well, `par` thirty seconds of a clock that has not yet finished.
`draw` holds the rows a page is handed, not what a GPU makes of them; and no row has a page, a module build or a teammate's conduct in it, and the par clock's arithmetic and its row are `tests/par.rs`'s.

**The pins (`pins.txt`).** Text, made by `gate pin`, read by the rows, never written by hand. ROWS: every row's name, in order: a whole `gate` fails where the rows are not that list (`rows gone: …`, `new rows: … (`gate pin`)`), and only a whole `gate pin` writes it. STARTS: the refused columns with their words, a line of the columns' names, then a line a part: its name
and a word a column, or one word where every column has the same, so a part that joins or leaves the state is one line of the file. SERIES: a line a run, a word a second. A pin depends on nothing but
the game: a debug build records the same bytes. `gate pin start` touches only the columns named.

- **A commit that moves a pin says why.** A move nobody meant is a fault, and the row's line says where to look: `hold keep` on the build before and `hold check` on this one name the part and the frame.
- **A join that conflicts in `pins.txt` records them again** on the joined tree, and says so in its message. The two sides' moves are not merged by hand.
