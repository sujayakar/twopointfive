# The state bridge

A whole game as bytes, the same from the TypeScript and from Rust, so the two can be compared field by field. Everything here is written from the skeleton's tables, and `build.sh` writes `schema.json` and `rust/game/src/io.rs` on each rerun.

## Files

- `schema.json`: every type the walk reaches, in the order of `types.rs`'s fields.
- `export.ts`: `exportGame(g, knobRegistry())` gives a `Uint8Array`, and `report()` says what did not fit. It never writes to the game.
- `expose.ts`: a preload that gives the walk a window on each module's private variables. Without it, only what side.ts registers is read. A tool's only, never in the page.
- `paths.ts`: `decode(bytes)` gives (path, bits, module) with real names (`g.gd[3].last_known[0]`). `diff(a, b)` gives every path that differs, grouped by the module that declares the field. As a command: `bun run tools/blitz/paths.ts a.bin b.bin`.
- `bridge-check.ts`: the bridge alone, on a live game.
- `rust/game/src/io.rs`: `io::export(&Game) -> Vec<u8>`, `io::import(&[u8]) -> Game`, and `io::import_into(&mut Game, &[u8])`, which keeps what the walk does not carry as that game has it.

## Running it

Copy the TypeScript files and `schema.json` to `tools/blitz/`, then:

```
bun run --preload ./tools/blitz/expose.ts tools/blitz/bridge-check.ts --play room-clear-four:1:2400 --out x.bin
BRIDGE_BLOB=x.bin cargo test --lib bridge_round_trip      # in the crate
```

`--play scenario:seed:frame` stops a real play-through at that frame. `--query` and `--ticks` stand a level up and step it with no input. `--seats 2` is a co-op game.

## The bytes

Game's plain fields in the order of `types.rs`, then each table in the schema's order as a count and its rows, then the serial counter (`u32`).

A number is a raw double, or a single where the field is f32. A bool is a byte, an Option a tag byte, a string its length (`u32`) and utf-8. A Vec, JsMap or JsSet is a count (`u32`) and its items in the TypeScript's own order. A word is a `u16`, a union a `u16` arm and its payload, an ID a `u32`, a `usize` a `u32`.

## IDs and serials

A table is first filled from the TypeScript's own list where it has one (`seeds`: guards, doors, players), then by first reach in the walk. The object-to-ID map lives as long as the game, so two exports of one game agree. A row reached once stays in its table, as Rust's tables never shrink.

Serials (`GameEvent.serial` and the like) are dealt by object identity in the same way, from 1.

Rust's own numbering is the bridge's: an ID has no meaning across a fresh export of another game. rust/sim row numbers are not what `LightId` means now. A row a light has is a field of it.

## Taking a start

Take a start after `g.sim.settle(g)`, as a tick's own opening does, so that the men's poses are at home.

A pose the TypeScript keeps outside the pool (a dead man's `rd.pose`, a kit figure) has the place `Pose::LOOSE` plus its row of `g.loose_poses`, which holds its node count and floats. They stand after the serial counter in the bytes, because walking anything can meet one. A Rust pose lives in the pool, so a body calls `anim_skeleton::pose_take_home(g, &mut pose)` when it first needs one. **So a pose pool can't match from the first death on:** compare the block without its last part. `paths.ts` reads no row of `loose_poses`, because the Rust list stands as it was imported; compare a ragdoll's floats by reading them from the pool at its place.

## What a Rust host calls round a step

Nothing inside the tick does these, and the next tick reads what they leave:

- **Before a step:** `io::take_host_half` with the buffer `hostHalf` wrote: the seats' records, the host's cameras and Input. `scene_boxes::box_world_list` (`host_world_upload`) when it lists the boxes, as `engine.world.upload()` does between ticks.
- **After a step:** `render_lights::light_set_update(g, dt, time)`: a flash's envelope and its retirement. `chronicle::chron_checkpoint` when it autosaves: `saves.tick` is the tick's last line in the TypeScript and writes that row into the game. On rails, `rails_host_view(g, cam)` with the host's camera (game.ts hostAfter: a tutorial staged again opens on its first view), and at an authored checkpoint `director_stop(false)` then `rails_resume`.
- **The smoke solver's answers are input:** `budget`, `inSmokeDomain`, `round`, `drawing`. With no solver they are the host's null object's.
- **The host lists the box world as its step ends, BEFORE whatever it then does to the game.** A driver's write after the listing (a charge blown between ticks) makes a piece that stands in the statics but isn't on the list until the next step ends. A host that lists after its writes sees that piece a tick early, and the two sides part on the fog's and the aim ray's boxes, not on a rule.
- **The listing frees frame boxes:** `scene_boxes::boxes_frame_reset` straight after `box_world_list`, not in the tick. The engine's list names a frame's IDs until the next listing, so a host that lists every fourth frame keeps them that long.
- **The host's camera asks after a step:** `followPos()` eases and stamps, so a host that shows a camera calls `game_follow_pos` for each seat after the tick, and a compare that leaves it out parts on the camera, not on a rule.
- **A checkpoint put back between ticks is a new start.** The save controller replaces the whole game, and in that frame the host steps twice. Take it as a fresh import from the state and block as that tick returned; what the load itself does differently is then not compared.
- **`io::take_host_half` writes `g.cmd_buf` from the local seat's record.** It's right with two seats (the records go back in seat order for `game_step_seats`), but a host that steps seats directly and never goes through `Game.update` has no `cmdBuf` of its own, so clear it there or the kept states read None against Some. It also takes each seat's mark: **`Seat.mark` is the host's half.** A cut is taken by whoever hosts the picture, so after one the puppet's export differs from the last apply in that field until the next hand-over.
- **A question between ticks can write the game:** `g.fight.roomAt(p)` floods a room and keeps it, and a driver's `orders.give` is an order. A compare replays those at the same tick.

## Comparing two states

A table's numbers mean nothing between two sparse exports: the walk numbers a thing when an export first meets it, and Rust when it is made. So compare whole states tick by tick from one start, or renumber by reach, as your `diff3.ts` does.

Identity serials differ by design. `schema.serial` lists the fields that hold one: every invented field named a serial (a counter of the TypeScript's own stays compared), and each field the walk met holding an object where a number belongs (`serial-fields.json`, made by `bridge-check.ts` writing `<out>.serials.json`). `<field>{}` means the keys of a map or set. `paths.ts` reads such a value as `serial` and such a key by its place, so `diff` does not see them; a list's length still counts. The walk only knows what it has met, so add a new mission's file to the list.

## What the host owns

`g.engine` is carried as far as the tick reads it: the lights' list in its own order with `ir` and `fog_team`, the box world's lists and its ray grid (`statics`, `dynamics`, `all`, `rg_*`, the glass lists), every plain setting, the weather frame, `view_focus`, `sharp_on`, `glass_fracture`, `effective_scale`. `LightId` is seeded from the lights' list, so a light nothing else reaches, the moon, has its row.

Between two ticks the TypeScript's host writes the sampled records, its own camera and Input. `hostHalf(g, knobs, cmds)` (export.ts) writes them as their own small buffer and `io::take_host_half(&mut g, &buf)` takes them and answers the records for `game_step_seats`. Nothing else of the game moves. `cmd_seat.mark` is a serial the host deals: a compare should expect it to differ.

A game's slot: `io::set_slot(&mut g, slot)` is the one way to change it. `import` sets the box mirror's slot from `g.sim`; the bridge carries neither.

A level script: `io::deal_scripts`, at the end of each import, finds the script from the level's name and runs the define pass again, which is the only way to its locals. A crate with `manifest_of` still a stub panics there on a scripted level.

## Closures

An order row's closures on a puppet: see `ask.ts`. They ask the Rust game by the row's ID, so no registration runs twice.


The walk carries a closure set up once, where it can name the arm: `pane_rule`, `pane_rows`, `glass_walkable`, `fire_pass`, the blast listeners, the wall-change listeners, each rate cap's `n`. `io::deal_closures` runs at the end of an import and deals every order row its closures by `def.id`, from a table in `hand2.cjs` (`ROWS`) written from the parts' own registrations. `report()` names any row that table lacks. `go` takes the arm of `door_tools.go_key`.

Closures made mid-flight stand at their blank after an import: a thrown can's `on_bounce`, a blow's `live`, a fall's `release`, `Threshold.out`, `Later.then`. `SquadOrders.notes` does too: it is the HUD's. So compare from the start of a mission and step both games; do not import into the middle of a fight and expect those to resume.

## Serials

The walk deals a serial by object identity, from 1, in the order it meets a record: `GameEvent.serial`, `script.serial`, `g.messages[..].serial`, `g.orders.log[..].serial`, and any `u32` field holding an object. A Rust body deals from `g.serial()`. So the two numberings agree only by accident: compare whether two serials are equal within one game, never a serial's value across the games.

## A state in the middle of a level script

A JavaScript generator's place and locals can't be read from outside, so **a state the TypeScript exported while a beat runs still can't be imported**: `io::import` and `import_into` refuse it by name, and `report()` says the same.

Rust's own bytes can: `LiveBeat.it` is carried as a counted blob holding the beat's arm and frame (empty is `Spent`), and a `Cond.test` as an arm and a number. The apply keeps both beside the row as an echo and the puppet's export writes them back, so a hand-back during a beat is refused only if a beat's place was never Rust's. Checked on tut-solo: 24 whole hand-backs with a beat running over 600 ticks, equal to the plain play. `--handback N` in the host is the check, and `HB_DEBUG=<dir>` dumps the whole Rust game before and after one, which is what found a lost `Cond.test`.

## What it does not carry

`not-carried.json` lists every field left out, and why. On import those fields stand at their blank, or as the game had them with `import_into`. The kinds:

- **Closures made enums that are made mid-flight** (see Closures above: `BlowLive`, `ItemBounce`...). A closure's captures cannot be read from outside. A mid-flight blow, a thrown can's bounce or a beat is lost. The orders' tables are static and could be named by `def.id` later. Until then, stand the Rust game up with its own `new` and use `import_into`, or compare from the start of a mission.
- **Host fields** (audio, smoke, journal, the renderer's, and of the engine what the tick does not read) and modules triaged host or dress: not simulation.
- **Everything under src/sim**: rust/sim's block is copied by bytes, by the caller. `g.sim` is not written.
- **`g.stats`**: the counters for the checks.
- **Invented fields** with no TypeScript source (`fix_stamp`, `path_serial`, `in_cast`...) start at their blank. Fields for the owner part (`Guard.owner`, `rank`, `reason`, `since`) get a line at the snapshot, once that part is in the tree read.

## A value is walked where it is held

A struct two holders share in the TypeScript is written twice and arrives as two copies, with the same values. A later write in Rust reaches only one. That is what the ID, serial and path decisions of the reviews are for, and review 4's "fine as a copy" answers say where it does not matter.
