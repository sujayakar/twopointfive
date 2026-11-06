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
TPS_PROFILE=shipped bun run --preload ./tools/blitz/expose.ts tools/blitz/bridge-check.ts --play room-clear-four:1:2400 --out x.bin
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

Take a start after `g.sim.settle(g)`, as a tick's own opening does: a pose that has not moved into the pool is written as place `0xffffffff`, and its floats are not carried. `report()` counts every such pose. Later in a game the only ones left are poses nothing draws from: a figure on the kit page and a dead man's `rd.pose`.

## What the host owns

`g.engine` is carried as far as the tick reads it: the lights' list in its own order with `ir` and `fog_team`, the box world's lists and its ray grid (`statics`, `dynamics`, `all`, `rg_*`, the glass lists), every plain setting, the weather frame, `view_focus`, `sharp_on`, `glass_fracture`, `effective_scale`. `LightId` is seeded from the lights' list, so a light nothing else reaches, the moon, has its row.

Between two ticks the TypeScript's host writes the sampled record, its own camera and Input and a memo the HUD fills. A game stepped on its own from one start takes those with `io::take_host_rows(&mut g, &h)`, where `h` is the state imported as the next tick opens. It takes nothing else of `h`. `cmd_seat.mark` is a serial the host deals: a compare should expect it to differ.

## Closures

The walk carries a closure set up once, where it can name the arm: `pane_rule`, `pane_rows`, `glass_walkable`, `fire_pass`, the blast listeners, the wall-change listeners, each rate cap's `n`. `io::deal_closures` runs at the end of an import and deals every order row its closures by `def.id`, from a table in `hand2.cjs` (`ROWS`) written from the parts' own registrations. `report()` names any row that table lacks. `go` takes the arm of `door_tools.go_key`.

Closures made mid-flight stand at their blank after an import: a thrown can's `on_bounce`, a blow's `live`, a fall's `release`, `Threshold.out`, `Later.then`. `SquadOrders.notes` does too: it is the HUD's. So compare from the start of a mission and step both games; do not import into the middle of a fight and expect those to resume.

## Serials

The walk deals a serial by object identity, from 1, in the order it meets a record: `GameEvent.serial`, `script.serial`, `g.messages[..].serial`, `g.orders.log[..].serial`, and any `u32` field holding an object. A Rust body deals from `g.serial()`. So the two numberings agree only by accident: compare whether two serials are equal within one game, never a serial's value across the games.

## What it does not carry

`not-carried.json` lists every field left out, and why. On import those fields stand at their blank, or as the game had them with `import_into`. The kinds:

- **Closures made enums that are made mid-flight** (see Closures above: `BlowLive`, `ItemBounce`...). A closure's captures cannot be read from outside. A mid-flight blow, a thrown can's bounce or a beat is lost. The orders' tables are static and could be named by `def.id` later. Until then, stand the Rust game up with its own `new` and use `import_into`, or compare from the start of a mission.
- **Host fields** (audio, smoke, journal, the renderer's, and of the engine what the tick does not read) and modules triaged host or dress: not simulation.
- **Everything under src/sim**: rust/sim's block is copied by bytes, by the caller. `g.sim` is not written.
- **`g.stats`**: the counters for the checks.
- **Invented fields** with no TypeScript source (`fix_stamp`, `path_serial`, `in_cast`...) start at their blank. Fields for the owner part (`Guard.owner`, `rank`, `reason`, `since`) get a line at the snapshot, once that part is in the tree read.

## A value is walked where it is held

A struct two holders share in the TypeScript is written twice and arrives as two copies, with the same values. A later write in Rust reaches only one.
