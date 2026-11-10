# rust/game_start: the game's own start, and tests that stand a scene

How every game begins. A level file's text and the character's files go in, and a `Game` the tick can run comes out.
The page, the game server and the native tools all start a game through this crate.

Two things stand on it:

- **A test of a behavior in ten lines.** A scene (rooms, doors, lamps, men) is said in Rust, stood, run for a few seconds and asked what a man
  did. `cargo test` in this folder, or `gate tests` for every crate (below), runs them.
- **One start.** The scene builder is not a second way to make a game: it WRITES A LEVEL FILE, and one start stands any level file. What the
  start does not carry it refuses by name (below), and the gate's `start` rows hold what it makes to a pin, part by part.

Nothing here has been seen on a GPU: the host these games stand on draws nothing.

## Write a behavior test

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

That example is `tests/readme.rs`, so it is held to compile and pass, and `the_readme_shows_this_file` holds the block above to that file, line for
line. `tests/behaviour.rs` is the worked set. Each test there holds one rule over three seeds, beside a CONTROL that differs in the one thing the
rule is about, and says above it what a failure would mean. A seed is another SCENE as well as another die: in scenes this small the die is hardly
drawn, so each test moves a mark, a range, a door or a landing point by the seed (`nudge`).

- `a_calm_guard_sees_a_man_in_light_and_not_in_the_dark`: a sentry with a lit man 6 m in front of him goes suspicious, then alert, his fix is where
  the man stood, and he calls the contact once. Control: the lamp off, and he never notices.
- `a_defender_runs_from_a_live_grenade_and_gets_down`: he shouts once, runs from it, crouches, and is at his farthest from it when it goes off.
  Control: no grenade, and he stands on his mark and says nothing.
- `the_pair_ordered_to_a_door_stacks_on_its_two_sides`: the pair given the order ends one man each side of the door, against its wall. Control: the
  pair given no order does not go with them.
- `a_posted_defender_holds_his_room_when_a_shot_is_fired_outside`: he hears it, and ten seconds on he is still inside the room he holds. Control: no
  shot, and he is calm on his mark.
- `a_guard_who_hears_a_shot_behind_a_shut_door_turns_to_it_and_goes`: he says what he heard, believes the shooter is in the room the shot was fired
  in, and leaves his post for the door. Control: no shot, and he is calm with his back to it.

### The scene (`src/scene.rs`)

Meters, in the world a level must fit (40 by 28; x east, z south). The pieces are the level kit's own sizes (`tools/levels/lib/levelKit.ts`): a wall
12 cm thick and 3 m high, a real door in a 1.2 m slot with its frame, a bare bulb on its cord.

| call | what it says |
|---|---|
| `Scene::new(name)` | an empty lot at night: the ground and no light |
| `.room(id, x0, z0, x1, z1)` | four walls, a floor and a ceiling; two rooms that share a line share the wall |
| `.wall_x(z, x0, x1)` · `.wall_z(x, z0, z1)` | a wall on its own |
| `.door(name, Axis::X \| Axis::Z, line, at)` | a door in the wall on `line`, its slot starting at `at` along it, shut; `.locked(name)` |
| `.bulb(x, z)` · `.bulb_off(x, z)` · `.moon()` | light. With none, a place is black |
| `.sam(x, z)` | Sam's spawn |
| `.patrol(name, points, wait)` · `.sentry(name, at, toward)` | a guard on a route · a guard posted, looking one way |
| `.team(sam, yaw, marks)` | a room clear: the team's start (a yaw of 0 looks south along +z, π/2 east, π north) |
| `.defender(room, at, watch_door)` | a room clear: a defender posted in a room |

A scene is one of two kinds, never both: a NIGHT (Sam and guards on routes, as a level's file has them) or a ROOM CLEAR (a team and posted
defenders, started by the game's own `start_scenario`). The room clear's staging takes every man who is not its own off the floor, so the builder
refuses a scene that says both. A scene has at least one door (a level's mission names one). A ROOM CLEAR IS REFUSED BY THE GAME (the start stops
with the staging's own words, `roomclear.rs`) unless some room has more than one way in and every room has a defender: two rooms side by side
with a door between them and a door each to the yard, a defender in each, is the smallest that stands (`yard` in `tests/orders.rs`).

`scene.level_text()` is the level file, ready to be saved under `levels/`.

### The game in a test's hands (`src/play.rs`)

- **Stand:** `Play::scene(&scene, &opts)`, or `Play::level("kit-yard.json", room_clear, &opts)` for a file of `levels/`.
  `Play::mission("depot.json", "?levelFile=depot.json&roomclear&squad4", &opts)` starts a mission on a file from the words of its address, as a click
  on its row of the mission list does (the row's hour and weather, the fog of war, then the mission). `Play::mission_of` does the same on a level
  file's tree a test has already read and perhaps changed.
- **A player at the keys:** `src/player.rs` is a scripted player of a seat: async code with no executor, where an await is one tick. He acts only
  through the hand of the next tick and asks the game through a borrow that cannot outlive the question. `p.played_by(&mut script)` is one tick with
  his hands on the first seat, by the road the page's own hand goes; `p.played_out(&mut script, secs)` plays him to his end. The hands-off player
  (`src/hands_off.rs`, a room clear from its first step to its end) is such a script, and the page's Benchmark plays him through `face_hands_off`.
- **The tutorials, played:** `src/lessons/` has a file a tutorial, its steps by the lesson's own order, each what a person does for the line that is
  up, with his hands alone, and as mortal as a player. The driver is one: half a second of nothing and the step must still be up, then the deed, and
  the step must have gone by; after the last step the run must end. `tests/lessons.rs` plays them (`TUT=<id>` picks one, `PACE=<secs>` has him read
  each line; `cargo test` ignores them, so give `--ignored`), and holds each lesson against its floor, that a checkpoint carries the step that is up,
  and that a restart puts step 1 back. `played_by` does what the page does with a tick's checkpoints.
- **Time:** `p.run(secs)`; `p.until(secs, |p| …)` answers how long it took, or `None`; `p.always(secs, |p| …)` answers `None`, or the second it first
  failed; `p.frame()` is one sixtieth.
- **Who:** `p.guard("route name")`, `p.defender_at(mark)`, `p.mates()` (in the order the game made them), `p.red()` and `p.blue()` (the living men of
  each pair), `p.roll()` (every man in a line, for a failure's message).
- **Ask:** `p.at(man)`, `p.state(man)`, `p.awareness(man)`, `p.fix(man)`, `p.facing(man)`, `p.speed(man)`, `p.low(man)`, `p.stacked(man)`, `p.up(man)`,
  `p.sam_at()`, `p.sam_light()`, `p.frag_fuse()`, `p.floor()` (what stands on the level: round things, shaped boxes, trim, civilians, devices).
  `p.lines(man)` and `p.said` are every line a man spoke (read off the speech bubbles after each frame), `p.sounds` every sound the game asked its host
  to play, `p.end_cards` when it told its host to open the end card. `p.debrief_view(seat)` is what the end card shows that seat.
  The mission: `p.view()` and `p.view_of(seat)` (all a HUD is told), `p.objective()` (its row),
  `p.room_clear_end()`, `p.hostiles()`. The par clock: `p.par_live()`, `p.par_bars()`, `p.par_started()`, `p.par_result()`, `p.par_counts()`.
- **Do:** `p.order("stack", place, Who::Red)` is the order menu's own call; `p.frag_lands(place)` and `p.flash_lands(place)` the sandbox's;
  `p.sam_fires_at(place, settle)`, `p.point_at`, `p.press`, `p.key_down` move Sam as a player's hand does, through the host's Input. An order pointed
  at Sam's own spot has a target of no kind, and `move` is refused there: point it somewhere else. `p.falls(man, by)` puts a man down by combat's own
  call, for a test of what follows a death. With `Opts { seats: 2, .. }` a second player sits in: `p.seat_orders(1, …)`, `p.seat_steps(1, meters)`,
  `p.seat_stands(1, place)`, `p.seat_at(1)`. `p.man_fires_at(man, place)` is a round from a man by combat's own call, `p.falls_as(man, how, quiet, by)`
  a man put down any way combat has, `p.seat_goes_down(seat)` a player out of hits with no round fired. The recap: `p.recap_card()`, `p.recap_deaths()`,
  `p.recap_lost()`, `p.cue_over(man)`.

Three rules keep the tests honest and alive:

1. **Ask through `play.rs`, not the game's fields.** When the AI's fields move, one reader follows and no test does. A test needs a new question: add a
   reader. (Some tests still read a field where no reader stands. Do not copy that; add the reader.)
2. **Do through the game's own calls.** An order by the menu's call, a shot by Sam's trigger, a grenade by the sandbox's call. Never a write to a man's
   awareness or a pushed event: that tests the test.
3. **Hold a rule, not a run.** Several seeds, a control, and what a man DID (where he went, what he said), not the name of the script he was on.

A game senses for itself on every host, so "lit", "dark" and "behind smoke" are the same thing here as on the page.

**One game at a time in a process.** `rust/sim` keeps a game's block and several tables in statics, so a `Play` holds a lock (`ONE_GAME`) until it is
dropped and `cargo test`'s threads take turns. A test that stands a second game drops the first (`drop(p)`) before it does.

## The start (`src/stand.rs`)

`stand(level, assets, opts)` makes a game on a host with no picture. A host that hands what it has as one run of bytes starts by `stand_handed`
(`src/handed.rs`), which is what `face_start` calls. In order:

0. **`host::lay`** (`src/host/mod.rs`), alone: what the machine keeps (`StartOpts.kept`, one JSON text across a host) and the address's words, the address always over it.
   Kept rows and picks, the address's numbers, the level, the levers, the cards, the quiet picks. A lesson keeps no lever, and hard only with `lessonhard`. The desk uses it too.

1. The address's words, where one is handed (`StartOpts.link`), BEFORE the game is made: the game's `address_before_game` (the row's own hour and
   weather onto the words, the weather the game is born in, the visit's hour). An address that says
   `kitview` first builds the level the kit picture needs and dresses the men for it (`src/kit_stage.rs`).
2. The block of `rust/sim` the game stands in, zeroed under the layout's word (`stand.rs`).
3. The character: skeleton, skin, clips, the cast's meshes and variants, from the glTF files' bytes (`gltf.rs`). The page reads the same files for its
   own drawing with `src/anim/gltf.ts`.
4. The host: an empty box world and light set (the game's `box_world_new`).
5. The level file read into the `Level`, its boxes and routes as rows (`level.rs`). The records the game keeps as its own types (a mission's `scenarios`,
   the footsteps' map, devices, civilians, vents, the drill plans) and the file's own records the game crate has a type for (a box with its shape and
   markings, a light, a door) go through `read.rs` and `records.rs`, a line a type.
6. Thin things and window bars marked; each box that says a shape given its record and its trim; statics into the world, lights into the set
   (`passes.rs`; the game's `cut_record`, `box_world_add_static`, `light_from_def`).
7. The seed the game's draws are keyed by, before the men are made: what they are dealt is drawn under it.
8. Collision, the walk grid, sound paths, fixtures, doors, guards, civilians, orders, the fog, the room fight, every module: **the game's own
9. The moving state at home in the block (`settle`: the pose pool, the smoke grid).
9. The moving state at home in the block (`settle`: the die, the pose pool, the smoke grid).
10. With no address, the solo night's card (`kit_solo_stand`). The visit's hour over the level's own night (`apply_hour`).
11. The host's Input and camera (the game's `input_detached`, `FollowCamera`, `follow_camera_rebuild`).
12. A mission, where asked: the game's `start_scenario`; or, from an address, `start_at_address`: `start_from_query` on its words, then the solo card's
    question, as a click on a row of the mission list starts it on a page. Refused words stop the start, unless `StartOpts.may_refuse` says to report them
    in `Stood.refused`.
13. What the light under a roof is worked out from (`light_ready`).

`frame(g, host, dt)` is one frame of that host: the game's `game_update` on the host's Input and camera, the camera after the man, the Input's frame
ended. It answers what the step told its host (sounds, marks, lines said). `step` is the same with that list left on the game, for a host that draws
before it empties the list.

### What the start reads of a level file, and what it does not

Every file under `levels/` is read whole. `stand` REFUSES a file with any fault of the reading, and names them all: a level stood without part of itself is
another level.

- **The format is `file.rs`** (and `kit_file.rs`, the Team & kit picture's block), a line a record: which member of the file each field is. A record the game
  keeps as its own type is read by that type (`records.rs`), so a type that gains a field does not compile until its line names the member. A value that is
  not what its place wants is said by its place and the reading goes on (`read.rs`), so a file is told everything at once.
- **The file's own tree keeps the account** (`json.rs`). An object remembers which of its members were asked for, so when a file has been read, whatever
  nobody asked for is named by its place (such as `doors[0].auto`). A member the game lets go (a floor, an author's notes) is read by its type first
  (a record's `drops`).
- **The rules beside the reading are `faults.rs`**: ids of one thing each, names that point at something, numbers in their range, lists that go together.
  `gate` asks them of every file (`level file`), a tool of the file it made (`levels judge`), a start not, and no page carries them.

Every row of the mission list is stood, twice over, by the gate's `start` rows ("The gate", below).

## Levels: what a level owes whoever plays it (`src/levels.rs`, the program `src/bin/levels.rs`)

```
cd rust/game_start
cargo run --release --bin levels -- file [level …]         # its file has no fault
cargo run --release --bin levels -- judge < FILE           # every fault of a file that is nowhere yet
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

Each stands the level (or every mission of it) by this crate's start and asks the game's own rules; the head of the file says what each holds and
why, and which take a flag such as `--list` or `--write`. `LEVELS_BREAK=1` is each question's red path, which must fail.

## Hold: is this build the same game as that one (`src/bin/hold.rs`)

For a change that is meant to change nothing of play.

```
cd rust/game_start
cargo run --release --bin hold -- keep /tmp/held        # on the tree BEFORE the change: play, and keep what the game was after every frame
cargo run --release --bin hold -- check /tmp/held       # on the tree AFTER it: play the same again, and compare
cargo run --release --bin hold -- check /tmp/held --but PART,PART   # … with the parts the change is meant to move left out
cargo run --release --bin hold -- draw /tmp/drawn       # play, draw every frame, and say what it cost and whether it changed anything
```

`keep` stands each level by this crate's start, plays it for 90 s under each hand of `hands/`, and writes, for every frame, the state's words
(`game::state_words`: a hash of every part of the game that a host does not hand in) folded to one number and one byte
of each part. `check` reads the folder's list of runs and its copies of the hands, plays them on the build it is, and says of each run
`the same after every frame`, or the first frame that differs and the names of the parts that differ in it (`FIRST DIFFERS AT FRAME 2181 (36.35 s) in
guards, chars, dice`). Exit 0: every run is the same. Levels, hands, seeds and the length are arguments
(`keep DIR --hand push --seed 1 --seed 2 --secs 60 depot:clear office`); with none it is `DEFAULT_LEVELS` (a `:clear` starts a room clear, a
mission's id that mission) by every hand by seed 1. A run is a process of its own, four at a time.

`draw` keeps nothing. After every frame it makes every section of what the picture is told
(`rust/game_draw`), as a host does that draws at twice the tick's rate, and once a second it holds the game's own words to what they were before.
`steps DIR` plays as `keep` does and prints, for each step of the tick, its microseconds a frame and the parts of the game it wrote.

A hand is a few lines of what is done to the game from outside (the head of `src/bin/hold.rs` is its grammar). Each file says what it reaches and what it does not:

- `push`: Sam unhurtable; every 6 s the squad is ordered to move to the nearest man who is not theirs; every 10 s Sam fires a round at him. It reaches
  the move order, doors, contact, the room fight on both sides, hunting, reloads, deaths.
- `bangs`: Sam unhurtable; a frag and a flash land by the nearest such man, again every 9 s; the squad falls in. It reaches the body rules (running
  from a grenade, shaken by a blast), flashed men, blasts on doors and glass.
- `doors`: Sam unhurtable; the squad stacks on the door nearest that man and is told to go, falls in, then a pair bangs its way in while the other
  holds the threshold. It reaches the stack order, the word to go, the door tools, entries and the room fight behind them.
- `kit`: Sam unhurtable; the squad goes silent, closes, throws a flash and a frag on order, covers, goes loud, holds, falls in. It reaches silent and
  loud, the throws' page, cover, hold.
- `rest`: nothing. It reaches a floor at rest: patrols, sentries, lights, the level's own script.
- `fires`: Sam unhurtable; every 10 s he fires a round at the nearest such man. It reaches a floor with no squad, woken by shots.
- `pair`: Sam unhurtable; the partner is told to take the nearest such man, and to watch him. It reaches the partner's take and watch, where a mission
  has them.

A run's line says how many of the hand's lines the game took, and names each line it NEVER took on that floor with the game's own reason, so a hand is not trusted for what it only names.

What a green check proves: on these floors, under these hands, the two builds played the same game part for part. What it does not: anything no hand
reaches (charges, the 40 mm, a startled man, a player walking, a second seat), and anything that is not in the state's words (what a host hands in: the
cameras, the input).

A change that is MEANT to change play is not judged here: `missions`, below, is for that.

**Parts are compared by name, and one can be left out on purpose.** A kept folder carries the names of the parts it was kept with, so a build that has
gained a part, lost one or re-ordered them is still compared on the parts both have, and each run's line says which left the state and which are new.
For a change that is MEANT to move one part and nothing else (a record that changes shape, a table numbered anew),
`check DIR --but tab.orders_order_def,seats.orders` leaves the named parts out: every other part must still be the same after every frame, and the last
line says what was not compared. A name that is a part of neither build is refused.

## Missions: what a change of play does (`src/bin/missions.rs`)

`cargo run --release --bin missions > before.tsv` on the tree before a change of play, the same to `after.tsv` on the tree after it, then
`… --bin missions -- diff before.tsv after.tsv`: every room clear of the list, the rescues and the night breach, standard and hard, over several seeds, under a
commander who gives the squad the same orders on every build. A row says who was up at the end, when the first man fell, when the floor was clear, the
rounds and reloads of each side and how long each rule of the list had an enemy. The file's head says what it does not see. `hold` says whether a
build is the same game; this says what the other game came to.

`src/bin/blastcost.rs` prices the tick of a wall blast against a quiet tick, natively.

## The gate: what a build is held to (`src/bin/gate/`)

One native command, with no page and no module in it. It stands every mission on the game's own list and plays a few of them, each as the page and the
server stand a game (`dress`, seed 9, the mission's own link). `gate` here is the program target/gate/gate under this folder, built by the first line:

```
cd rust/game_start && cargo build --profile gate --bin gate     # its own profile, cheap after an edit; a pin is the same under any profile
gate                 # every row, four at a time. Exit 0 = all pass · 1 = a row failed · 64 = a wrong word
gate save            # only rows whose name starts with a word: `gate start`, `gate "start depot-four"`
gate pin [ROW…]      # record the pins again and say which moved; nothing moved = the file is as it was, byte for byte
gate breaks          # each row of the breaks' table plays unbroken and must PASS, then each break must FAIL in its check's own words. Exit 0 = all did
gate long            # the quick kind, then the rows a merge has no time for (`played`, `lesson`, `level …`)
gate list            # the rows, then the missions no run plays
gate tests [BINARY [TEST]]   # the whole native test set: no part of the quick kind
```

Each row prints one line in the rows' order, and the last line is the count and the wall time, with what each kind cost. A row is a child process, so a
game that panics costs its own rows, which then FAIL with the first lines of what it wrote; the rows of one run share a child, because the run is played
once for all of them. A word on the command line picks long rows as it picks any other.

The rows, each with what it holds and then its breaks:

- **`start <id>, one player` · `, two players`:** every row of `all_missions`, twice, and `start compound-four, way 2`, whose link picks that way in and which stands on it.
  The start is the pinned start, part by part, or the same refusal in
  the same words. A second start in one process is the first: 5 s idle, every part each half second, then the mission before it on the list stood and run
  2 s, then the mission again, the same. Breaks: `pin` a word of the pin made wrong · `second` the second start's seed is one more · `first` (the way's row) its link loses its word.
- **`play <run>`:** 40 s under a hand: one word a second (every part folded as `hold` folds them) against the pin. Red if a line of the hand never
  happens on that floor, with the game's reason. Breaks: `pin` a word of the series made wrong · `silent` a line is said to have never happened.
- **`again <run>`:** the run stood and played twice more in one process, 10 s of the next run on the list between the two: the same word every second.
  Break: `second` the second run's die is drawn once before its first frame.
- **`save <run>`:** THE GAME PUT BACK IS THE GAME THAT WENT ON: a `Moment` at 3, 9, 15, 21 and 27 s, each stood again (the second and the fourth in
  slot 1) and played 3 s under the same hand: every part of every frame the same. What a host keeps beside a game (where the hand's next line is,
  `Play.cursor`) goes back with it. Break: `block` a moment whose block bytes are the run's end's.
- **`load <run>`:** A LOAD: the moments of 9 and 21 s, the value a save keeps, the run's game gone and another mission played 2 s in the process, then
  stood as a page stands a kept game in a slot nobody was in and played 3 s under the same hand: every part of every frame the run's. Break: `block` one
  byte of the block's die is another.
- **`draw <run>`:** everything drawn after every tick, at 0.5 and at 1.0. (a) The word every second is the undrawn run's. (b) A mirror of the page's
  buffers, filled ONLY as the page fills them (`src/bin/gate/draw.rs` lists them), is the module's own tables after every frame. (c) The boxes' head says
  what their lists hold. (d) Once a second, what the mirror holds is what a host that has just opened is told, but for what is a table's history
  (`draw.rs` says which). Played as hosts play: some frames come after two or three ticks, the look is said twice, and a moment of 18 s is put back at
  20 s and the run goes on, its word still the run's. Breaks: `told` a die drawn once between the undrawn run and this one · `mark` the mirror skips
  `from..to` on the frames that carry one after the first · `head` the head made wrong before it is read · `opened` a word of the fresh host's made wrong.
- **`seats <id>`:** a fight of two is its records: a hands-off player on each seat, each tick's two commands through `cmd_text` and back. Fresh games play
  the records with nobody sampling, showing seat 0 and then seat 1, and a moment at 12 s is put back and played to 18 s: every part the same each half
  second. The series is pinned. The two players' hands differ on one tick in five at least, each man walks and the fight fires at least so much, and more
  than half the seconds have different words. Breaks: `pin` · `fresh` a fresh game's replay hands each seat the other's records · `moment` the moment's
  does · `idle` nobody's hands do anything.
- **`par <id>`:** every mission whose level has `scenarios.par`. The clock moves nobody: 30 s under `push`, with the file's par block and with it taken
  out of the tree before the stand. The parts differ in nothing but what the table `CLOCK_ONLY` in `rows.rs` names, each with its reason; the clock's own
  part does differ, and the level's own record stands on each side as it stood. Breaks: `moves` a die drawn once on the clocked side only · `noclock` the
  block is out on both sides.
- **`steps <run>`:** A STEP OF THE TICK WRITES WHAT `steps.txt` SAYS (made by `hold steps DIR --write`): 16 s of four runs, each step asked each tenth
  tick what it changed. Another part is a new coupling: meant and added, or not made. Break: `said` forgets one.
- **long: `played <id>`:** every room clear, rescue and the night breach, by the hands-off player (`src/hands_off.rs`) under the page's god mode, so
  that each goes on to its floor cleared. The mission played to its end by keys alone. (a) What a run owes (`src/owes.rs`), every tick. (b) It ends.
  (c) A line of words a reader can read (its end and time, Sam, who is up and down, the orders, the medal if any, the counts of `owes`) and a word a second
  against the pins. Breaks: `deaf` the squad's key is deaf · `wall` a man is put in a wall at 5 s.
- **long: `lesson <id>`:** the six tutorials, by a man at the keys (`src/lessons/`). Every step goes by on its deed, the run ends, and it owes what a run
  owes. Not pinned: a change of play moves its seconds and not what it holds. `gate long` plays them again with him reading each line. Break: `third`
  nothing he does reaches the game from the third step on.
- **long: `level <question>`:** each question of `levels`. What every level owes whoever plays it, asked of all of them: 43 s of one core. `level file`, a
  second of it, is of the quick kind. Break: `spoiled` the question's own red path.

The runs (`RUNS` in `rows.rs`) reach the office's night breach under `doors`, a room clear in the rain (`depot-four`) under `bangs`, `compound-four` under
`doors`, `mousehole-four` under `kit`, `glasshouse-four` and `killhouse-four` and a rescue under `push`, a quiet night (`bazaar-quiet`) under `fires`, the
snow (`pass-four`) under `bangs`, and the lesson of a partner's orders (`tut-pair`) under `pair`.

**`gate tests`** builds the tests of every crate that has any (the list is at the head of `src/bin/gate/tests.rs`, and a crate with tests that is not on
it fails the run) and runs each binary as a process of its own from its crate's folder, as many at once as there are cores. It prints each binary with its
counts and seconds, every red test by name with its panic line, what is ignored and why, the slowest and a last line. Exit 1 on a red, a binary that ended
badly or a build that failed. `gate tests squad_doors [test]` is that binary alone, or that test. `--break=BINARY` plants a red, to show that one is read.

**What the runs played to their ends do not hold.** The hands-off player is one player on one seed, and Sam cannot go down: a run holds the floor, not
whether the mission can be survived. `owes` holds what the game may never do and counts what smells; its counts are pinned and not judged, so a count that
goes up is a pin moved. It holds one seat, and nothing of Sam's speed.

**What it does not hold.** A row holds a game to itself, to its pins or to the same game by another road, never that it is any good, and a mistake made at
a pin passes: `start` a game at rest, `play` and `again` what the runs' hands reach, `save` the three seconds after each moment, `seats` a fight nobody
plays well, `par` thirty seconds of a clock that has not yet finished. `draw` holds the rows a page is handed, not what a GPU makes of them; and no row has
a page, a module build or a teammate's conduct in it, and the par clock's arithmetic and its row are `tests/par.rs`'s.

**The pins (`pins.txt`).** Text, made by `gate pin`, read by the rows, never written by hand. ROWS: every row's name, in order: a whole `gate` fails where
the rows are not that list (`rows gone: …`, `new rows: …`), and only a whole `gate pin` writes it. STARTS: the refused columns with their words, a line of
the columns' names, then a line a part: its name and a word a column, or one word where every column has the same, so a part that joins or leaves the state
is one line of the file. SERIES: a line a run, a word a second. LINES: what a run played to its end did, in words. A pin depends on nothing but the game: a
debug build records the same bytes. `gate pin start` touches only the columns named.

**What a pin is: the figures.** A whole-state word cannot say that play stood when a part only changes shape, so the pin of `start`, `play`, `seats` and
`played` is a word of `game_start::figures`: what play is, as plain numbers read by what they mean (the die; each man by his character id: where, facing,
up or down and by whom, state, what he holds and its rounds, his order; each seat the same, and the light meter the player reads; the doors; each pane that
has been touched by its box's number: gone or not, open pieces, damage, holes; each lamp by its number: its switch, shot or not, out with the mains, what
it burns; the smoke: the emitters, and each brick that holds any by its place in plan, with its amount; what is destroyed: how many statics stand, each
hidden or cut by its place in the level, the wall skins out; the alarm, the mission and its tally; what is in the air and on the floor; the shots). A row
whose figures moved FAILS and names them (`play moved from second 6: man 22 where, door 3`, by a hex digit of each figure a second, kept beside the word
for `play` and `seats`; `played` says its line instead, and a start names the parts that moved). A row whose figures stand and whose parts moved PASSES and
says `only the words moved (parts …)`; the last line counts those rows and the exit is 0. `gate pin` says each move as `PLAY` or `WORDS`, so a commit can.
The parts are kept beside: for a start as they stood, for a series at its last second. `lines` has a line a series a reader can read: who is up, the
rounds, the orders. Breaks: a man 1 cm aside, a round, a door's angle, a man's order, the die, a hole in a pane, a lamp's switch, a smoke emitter and a
hidden static each go red by name, and a word moved with no figure is the words line.

- **A commit that moves a pin says why.** A move nobody meant is a fault, and the row's line says where to look: `hold keep` on the build before and
  `hold check` on this one name the part and the frame.
- **A join that conflicts in `pins.txt` records them again** on the joined tree, and says so in its message. The two sides' moves are not merged by hand.
