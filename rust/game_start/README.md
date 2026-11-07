# rust/game_start: the game's own start, and tests that stand a scene

The simulation's tick is Rust (`rust/game`), but until this crate a game could only *begin* in TypeScript: the level was loaded, the men made and
the `Game` put together by `src/host/standup.ts`, and the result handed across the bridge. Here is the start in Rust. A level file's text and the
character's files go in, a `Game` the tick can run comes out, and nothing in between was made by TypeScript.

Two things stand on it:

- **A test of a behaviour in ten lines.** A scene (rooms, doors, lamps, men) is said in Rust, stood, run for a few seconds and asked what a man
  did. `cargo test` in this folder runs them all in about half a minute.
- **The road to starting every level in Rust.** The scene builder is not a second way to make a game: it WRITES A LEVEL FILE, and one start stands
  any level file. What the start does not carry yet it refuses by name (below), and a compare holds it to the TypeScript start path by path.

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

Metres, in the world a level must fit (40 by 28; x east, z south). The pieces are the level kit's own sizes (`src/scene/levelKit.ts`): a wall
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
refuses a scene that says both. A scene has at least one door (a level's mission names one).

`scene.level_text()` is the level file. It can be saved under `levels/`, opened in the editor, and stood by the TypeScript start.

### The game in a test's hands (`src/play.rs`)

- **Stand:** `Play::scene(&scene, &opts)`, or `Play::level("kit-yard.json", room_clear, &opts)` for a file of `levels/`.
- **Time:** `p.run(secs)`; `p.until(secs, |p| …)` answers how long it took, or `None`; `p.always(secs, |p| …)` answers `None`, or the second it
  first failed; `p.frame()` is one sixtieth.
- **Who:** `p.guard("route name")`, `p.defender_at(mark)`, `p.mates()`, `p.roll()` (every man in a line, for a failure's message).
- **Ask:** `p.at(man)`, `p.state(man)`, `p.awareness(man)`, `p.fix(man)`, `p.facing(man)`, `p.speed(man)`, `p.low(man)`, `p.stacked(man)`,
  `p.up(man)`, `p.sam_at()`, `p.sam_light()`, `p.frag_fuse()`; `p.lines(man)` and `p.said` are every line a man spoke (read off the speech
  bubbles after each frame), `p.sounds` every sound the game asked its host to play.
- **Do:** `p.order("stack", place, Who::Red)` is the order menu's own call; `p.frag_lands(place)` the sandbox's; `p.sam_fires_at(place, settle)`,
  `p.point_at`, `p.press`, `p.key_down` move Sam as a player's hand does, through the host's Input.

Three rules keep the tests honest and alive:

1. **Ask through `play.rs`, not the game's fields.** When the AI's fields move, one reader follows and no test does. A test needs a new question:
   add a reader.
2. **Do through the game's own calls.** An order by the menu's call, a shot by Sam's trigger, a grenade by the sandbox's call. Never a write to
   a man's awareness or a pushed event: that tests the test.
3. **Hold a rule, not a run.** Several seeds, a control, and what a man DID (where he went, what he said), not the name of the script he was on.

A game is stood with **the senses worked out by the game itself** (`SENSE.cpu`, and the sky's light in its estimate). A host with no picture has
no light meter, and with them off every man stands in the dark for good, so "lit" and "dark" would be one thing. The page plays with them off.

**One game at a time in a process.** `rust/sim` keeps a game's block and several tables in statics, so a `Play` holds a lock until it is dropped
and `cargo test`'s threads take turns. A test that stands a second game drops the first (`drop(p)`), or is told so.

## The start (`src/stand.rs`)

`stand(level, assets, opts)` is `standGame` of `src/host/standup.ts` on a host with no picture, in its order:

| step | who does it |
|---|---|
| the block of `rust/sim` the game stands in, zeroed under the layout's word | `stand.rs` |
| the character: skeleton, skin, clips, the cast's meshes and variants | `gltf.rs`: `src/anim/gltf.ts` in Rust, from the `.gltf` files' bytes |
| the host: an empty box world and light set, the picture's settings | the game's `box_world_new`; `settings.rs` (a copy of `defaultSettings()`) |
| the level file read into the `Level`, its boxes and routes as rows | `level.rs`: `src/scene/levelLoad.ts loadLevel` in Rust |
| thin things and window bars marked; statics into the world, lights into the set | `passes.rs` (`scene/thin.ts`, `bars.ts`); the game's `box_world_add_static`, `light_from_def` |
| the die, seeded before the men are made | the game's `sim_dice_new` |
| collision, the walk grid, sound paths, fixtures, doors, guards, civilians, orders, the fog, the room fight, every module | **the game's own constructor, `game_new`** (`rust/game/src/game/p_04.rs`), which had never run before this crate |
| the moving state at home in the block | `settle` (the die, the pose pool, the smoke grid) |
| the host's Input and camera | the game's `input_detached`, `FollowCamera`, `follow_camera_rebuild` |
| a mission, where asked | the game's `start_scenario` |

`frame(g, host, dt)` is one frame of that host: the game's `game_update` on the host's Input and camera, the camera after the man, the lights'
envelopes, the box world listed again. It answers what the step told its host (sounds, marks, lines said).

NOT MADE, because no rule of the tick reads them: the character renderer, the mesh shadows' tree, the proxy fits of the rig.

### What a level file may hold that the start does not carry yet

`level::lacks_of(file)` lists it and `stand` refuses the file, naming each record: a level stood without part of itself is another level.

- a box's `shape` (and the kit's octagons as round things: `scene/round.ts`, which no file names and nothing here finds)
- `civilians`, `devices`, `vents`, `connectors`, `daylight`, `plans.roomClear`, `plans.breach`
- the scenario kinds other than `roomClear`, `places` and `doorNames`
- the file's own checks (`validateLevelData`): a file is taken as one the TypeScript validator has passed

## The compare (`tools/blitz/start.sh`)

```sh
tools/blitz/start.sh kit-yard.json                                   # a level of levels/, as its file has it
tools/blitz/start.sh kit-yard.json --scenario roomclear --frames 1800
tools/blitz/start.sh rust/game_start/target/scenes/two-rooms.json --scenario roomclear --cpu --frames 900
```

1. `tools/blitz/start.ts` stands the file with the TypeScript start on the page's stand and exports the state over the bridge.
2. `rust/game_start`'s bin `start` stands the same file here and exports its own.
3. `tools/blitz/diff.ts` lists the paths that differ, minus `known.txt` and `known-start.txt`: what a Rust start may differ in, each with its
   reason (the picture's tables, memos, addresses). Anything a rule reads is mended in the start, never named there.
4. With `--frames N`, the TypeScript's start is IMPORTED and both games are stepped in Rust by the same host frame with an idle hand: the die's
   count and every part of the block compared after every frame (the bytes themselves), then the two end states by path. This holds what a path diff of two starts
   cannot see (state the bridge does not carry, what the import mends as it reads).

`--cpu` raises the game's own senses on both sides, as the tests stand. The tests' scenes are written to `target/scenes/` by
`cargo test the_scenes_are_level_files`.

As of the commit that added this file: `kit-yard.json` plain and with its room clear started, and the three scenes of `tests/behaviour.rs`, each
0 paths not known and the block equal to the byte; the room clear 1,800 frames in step, the scenes 900.
