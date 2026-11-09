# Writing Rust in this game

How the Rust is laid out, what it must never do, and how to add to it. A test is `rust/game_start/README.md`, the module as a host loads it is
`tools/blitz/FACE.md`, dead code is `tools/fold/README.md`.

## What it is for

The maintainer: "we should just write this in rust so we have good control over everything (and aren't subject to the vagaries of each JS engine)".
The same start and inputs make the same game to the bit on every host.

## Where the code is

Each crate is its own cargo workspace and target folder, but `prim` and `sim` share `rust/Cargo.toml`.

| crate | is | it links |
|---|---|---|
| `rust/prim/` | √, floor and trunc, exact by the standard | nothing |
| `rust/sim/` | hot loops on columns, each game's block. `no_std` | `prim` |
| `rust/game_derive/` | the `ToBytes` and `Knobs` derives | nothing |
| `rust/game_types/` | every module's types, `Game`, ids, `m`, `js`, `dice`, knob tables. `no_std`, no rules | `sim`, `prim`, `game_derive` |
| `rust/game_rules/` | the rules; its crate name is `game`. `no_std` | `game_types`, `sim`, `prim` |
| `rust/game_draw/` | what the picture is told. `no_std` | `game`, `sim` |
| `rust/game_start/` | a game from a level file; tests, `gate`, `hold`, `levels`, `missions` | `game`, `sim`, `game_draw` under `std` |
| `rust/game_run/` | a host's work round a step: commands, knobs by name, cards, address words | `game`, `game_start` |
| `rust/game_face/` | the module's C calls | all of them |

## How it works

### What holds determinism

- **The compiler.** A `no_std` crate cannot name a hash map, a thread, a clock or a file.
- **`tools/qa/rust-rules-check.ts`, over `rust/sim` and `rust/prim` only.** No rounded float method, no `mul_add`, nothing relaxed, no cell or atomic,
  no `std::`, and each `static mut` has a row in `STATICS`. In `rust/game_rules` and `rust/game_types` only the reader holds this.
- **`m::`.** `m::sin`, `m::atan2`, `m::hyp2`, `m::pow` and the rest are the game's own, from + − × ÷ and √ (`rust/sim/src/maths.rs`), because a
  library's sine differs by machine. `m::min` and `m::max` fix the answers for NaN and −0. Write these, never a method on `f64`. Do not tidy
  arithmetic that runs: `(a - b) - c` is not `a - (b + c)`.
- **`js::`.** `JsMap` and `JsSet` walk keys in the order first set, and that order is play. `js::sort_by` is stable. `js::to_i32`, `js::truthy` and
  `js::or` keep what a translated line meant.
- **Dice.** A rule draws keyed: `dice::draw_of(g, "guards::alert#1", gd)` (or `draw_body`, `draw_def`, `draw_op`, `draw_for`, `draw_world`) is a fixed
  function of the seed, the site's name, the man, the clock and how often that site drew for him this tick. So a draw added tomorrow deals nothing
  else again. A die kept on a record is a `Die`.

### How state is kept

`Game` (`rust/game_types/src/game/types.rs`) holds everything, and a rule takes `g: &mut Game` or `g: &Game`.

- **Tables by id.** `g.ch`, `g.gd`, `g.dr`, `g.pl`, `g.cv`, `g.fa`, and the rest in `g.tab`. An id comes from the `id!` list in `ids.rs`; things name
  each other by id, never by reference. `Tab` only grows, so an id never dangles (`g.tab.frame_boxes` alone deals rows again).
- **A module's state.** `g.m.<module>` (a `Mods` field), or a field of `Game` where many read it. Its numbers are `g.k.<table>`.
- **A slot.** `crate::owned::Owned` holds state one module reads, of a type declared up in the rules, so that changing it does not rebuild `game_types`:
  `g.ctl`, `g.feel`, `g.mark_set`, `g.ray_grid`.
- **The block.** `rust/sim/src/state.rs`: one fixed run of bytes a game, in `GAMES` slots, with `g.sim` saying which. It holds the die and the columns
  of what `rust/sim` steps: doors, falls, props, slabs, items, men, lights, weather, smoke, poses, panes. A number a hot loop keeps from tick to tick
  must be here; all else is on `Game`. In a block: indices and never addresses, zeros mean empty, no padding the compiler chose. Rules reach it by
  `sim::state::state_read(g.sim)` and `sim::state::state`, mostly through home modules (`sim_doorshome.rs`).
- **Copies.** The box world (`rust/sim/src/world.rs`) and the smoke's and air's tables are outside the block, made from the game: never state.
- **A moment** (`Moment`) is `g.clone()` and the block's bytes: a save, a walk of a tape, a recovery. So `Game` clones whole and a block stands in any
  slot.
- **Seats.** A seat's nineteen things are fields of `Game`, those of the seat that stands there; the others wait in `g.tab.cmd_seat[s].own`
  (`SeatSet`). In a tick the first seat stands (`on_first_bare`). `with_seat(g, s, &mut |g| …)` changes them over for a pass about another seat. A
  function holding the game shared takes a `SeatRef` (`seats::seat`, `seats::here`), never `g.orders`. A map kept per seat is keyed by `view_key`.

### Derives

`#[derive(ToBytes)]` writes fields in order and an enum's arm by its place. That walk makes the state's words (`state_words`) and what a HUD is told.
A field added is in its part's word at once. `#[bytes(skip)]` leaves a field out, with a doc line saying why: a host's own, a step's output, the
picture's, or made again from the rest. `#[bytes(parts)]` (`Game`, `Mods`, `Tabs`, `SeatSet`) makes each field a named part.

`#[derive(Blank)]` is what a new record holds: each field at its type's own start, or at what `#[is(10.0)]` beside it says. `#[derive(Words)]` is an enum's
words (`as_str`): an arm's name with a small first letter, or what `#[word("…")]` says.

`#[derive(Knobs)]` with `#[k(2.0)]` beside a field gives its shipped value and a dotted name. The panel, `?knob.doorthrow.push_secs=3` and a tape's `Knob` command reach any number by that name.

### A module's files

Types in `game_types/src/doors/types.rs`; numbers in `knobs/doors.rs`. The rules stand in eighteen folders, one a system (`overview.md`). A module is
one file (`people/startle.rs`) or a folder (`fittings/doors/`) whose `mod.rs` re-exports its types and its parts; a part is named for its subject and
opens with `use super::*`, so a module's parts share one namespace. `lib.rs` hands every module on: code says `crate::doors`, and no system.
**A function is no wider than who calls it:** its file's, its module's (`pub(super)`), its system's (`pub(in crate::fittings)`), and `pub` only where
another system, crate or test calls it.

### Borrowing

A rule holds `g` and no reference across a call, so:

- **Copy ids out.** `for j in 0..g.guards.len() { let gd = g.guards[j]; … }`, or `for id in g.fight.defs.clone()` where a call may change the list.
- **let-else.** `let Some(x) = … else { return };` for what may be absent. Read the row again after each call.
- **Take and put back.** `let mut own = g.gd[gd].search_plan.take().unwrap();`, work with `g`, put it back: only where nothing called reads it, and
  say so beside it.

### Asking the world

The box world is a flat copy of every static, leaf and furniture part, which `sim::lines`, `sim::collide`, `sim::hear`, `sim::nav` and `sim::sight`
ask by slot. It is proven as a tick opens (`tick_open`) and stands from a tick's end to the next one's start (`tick_shut`).

- **In a tick:** `scene_collision.rs` takes `&mut Game` and pays what the copy owes: `segment_blocked`, `solid_blocked`, `raycast`, `collide_circle`.
  `Over` (`ask.rs`) asks of the level as it would be, changing nothing, through `segment_blocked_over` and its kind.
- **Between ticks, shared:** `WorldAsk::of(g)` has `line`, `ray`, `way`, `hear`. It panics by name if the copy does not stand.
- **One body for both:** a rule takes `impl Asker` and reads the game by `a.game()`.

A writer of a static says so: `scene_collision::wrote` in a tick, `unstand` between ticks. A build with debug assertions compares every row as the
next tick opens and panics on a write nobody declared.

### Performance

- **A loop over every man, box, leaf or cell, every tick, belongs in `rust/sim` on columns.** The maintainer: "we should also start changing our data
  layout to be more cache and simd friendly". The head of `rust/sim/src/props.rs` says how: rows dense, a column a number, hot columns together, flags
  as bits of one `u32`.
- **`rust/sim` allocates nothing.** Every table has a fixed size (`MAX_BOXES`, a part's `CAP`), and `door_store_new` refuses a level past its cap by
  name.
- **Too many arguments for a call** go through a scratch `static mut` (`IO` in `doors.rs`, `PAR` in `items.rs`). Nothing in scratch outlives the call.
- **Ask in a batch.** A sight pass hands its eyes over once (`sight_open`, `sight_eye`, `eyes_see`).

### Build profiles

`play` on `game_face` is optimized with no link-time optimization, about 7 GB of memory: the dev server's (`bun run tools/blitz/face-build.ts`).
`release` is the fat link, over 10 GB and many minutes: `bun run build`. `gate` on `game_start` is cheap after an edit, and no pin depends on it. The
`test` profile there has debug assertions on. Each build of the game crate takes several GB and minutes: run nothing heavy beside one.

### When it panics

The module aborts, so a host sees a trap; `hook` has kept the words with file and line (`face_panic_ptr`). `with` took the game out for the call, so
that game is gone; other games and every save stand. `guarded` in `src/host/drawgame.ts` turns the trap into `GameLost` and refuses all calls but
those in `STILL`. The card offers `face_recover`: the newest save or the run's first step. So use let-else for what
may be absent, and panic only on what must never happen, in words that name the thing.

### Gameplay and dressing

The maintainer: "let's make sure we're very explicit about what's gameplay affecting (and also run on the server) and what's just client side only
dressing (debris, smoke details) and uses relaxed simd and/or gpu". Today no crate only dresses: `SIDES` in the check says each is gameplay or both,
and neither side may link the other. Inside the rules:

- **Dress rows.** `g.m.dress.dress` is up by the start's flag on the page and the server, and down in a native test unless asked (`Opts.dress`).
  `wind`, `moths`, `stature`, `fx`, `blast_feel`, `fixtures`, `net` and `cues` guard steps in `game/tick.rs`. Those steps write state, so hosts of one
  game must agree.
- **The picture's own.** A field a host may lack or fill differently is `#[bytes(skip)]`, as `fog_picture`, `mark_set` and `feel` are.

## Rules that hold

- **All state is reachable from `Game` or in its block.** The rules hold no `static mut`.
- **The rules cannot name `game_draw` or `game_face`,** and a section's fill takes `&Game`, so drawing changes nothing.
- **A rule's state is stepped by the tick alone.** What a host asks between ticks moves no word.

## To change it

- **A module:** a `pub mod` line in `game_types/src/lib.rs` with its `types.rs`; `pub mod` in its system's
  `mod.rs` and its name in that system's `pub use` in `game_rules/src/lib.rs`; a line in `step_seats_on_first`; a test.
- **A field:** in its type, with `#[is(…)]` where it does not start at its type's own start. It moves its part's word; a new field of `Game`, `Mods`, `Tabs` or `SeatSet` is a new part: `gate pin`.
  A seat's field also goes in `SeatRef` and `swap_set`.
- **A column:** a field of its part in `rust/sim`, inside its size assert. A new part is a `State` field, a `PARTS` row and a `PART_NAMES` name
  (`game_start/src/block.rs`). A new `static mut` needs its `STATICS` row, saying STATE, SCRATCH, COPY or TALLY.
- **A knob:** `#[k(value)] pub name: f64` in its table. A new table is a file in `knobs/` with `#[derive(Clone, Copy, Debug, ToBytes, Knobs)]`, a
  `mod` line and a `Knobs` field in `knobs/mod.rs`. A slider is a `ROWS` line in `shown.rs`; an address word, only if asked for, a `WORDS` line in
  `game_run/src/words.rs`. A number `rust/sim` reads is also a line in `sim/src/knobs.rs` and in `knobs_to_rust`.
- **A table of things:** a row in `id!`, a `Tab` field in `Tabs` and in `Tabs::blank`.
- **A question:** the walk in `rust/sim`; a `&mut Game` form and a shared form (`line_shared`) in `scene_collision.rs`; a method on `WorldAsk`,
  and on `Asker` if a rule needs one body; a call in `game_face/src/ask.rs` and `src/draw/ask.ts` if the page asks it.
- **An event.** For the rules: a `GameEvent` on `g.events`, kept half a second. For the host: an arm of `OutEvent`, a function in
  `out_events.rs`, an `event!` row filled in `game_draw/src/events.rs`, then `tools/draw/gen.ts` and `src/draw/events.ts`.
- **A thing drawn:** a file in `game_draw/src/` with `section!` and its fill (fields `f32`, `u32`, `i32` or arrays of them; values, never references),
  a line in `sections!`, `tools/draw/gen.ts`, a reader in `src/draw/`.
- **A call of the face:** `#[no_mangle] pub extern "C" fn` in its file of `game_face/src/` and on that file's head list, using `with` or `with_ref`,
  answering by `out`. `Face` is untyped, so a wrong name in the page fails only when called.

### Leftovers, and what to do with them

The rules were machine-translated from TypeScript. None of these is a pattern for a new type.

| you see | do |
|---|---|
| `f64` for counts and kinds (`box_count`) | Match it in a field others read. A new type uses `bool`, `u32`, an enum, an id. Mending one moves pins: its own commit |
| `JsMap`, `JsSet` | Keep them: the walk's order is play. Use a `Tab` or `Vec` where ids are dense. Swap one only if the pins hold |
| `Option<f64>` arguments, `js::truthy(x)`, `js::or(x, d)` | Match when calling. A new function takes plain arguments and compares |
| `c_2`, `g_2`, `weather_set_wet_2`, `GameMessagesItem`, `DressModDRESS` | Rename when you are there. A part's field name is a line in the pins: `gate pin` |
| `&mut Game` on a function that only reads | `tools/fold/readers.sh --soften`, or by hand. A new one takes `&Game` |
| comments that cite a deleted file, "the TypeScript", or "ON in the page" | Trust the statements. Mend a false comment when you are in the file |
| `Opaque`, `FnField`, a `todo!()` body in `render_*/`, a module of only types | Call none: `todo!()` panics. The crates `#![allow(dead_code)]`, so use the fold tool |

## What holds it

- `bun run tools/qa/rust-rules-check.ts`: rules, statics, sides, what the module links. `bun run tools/draw/gen.ts --check`: the page's
  offsets.
- `gate`: the rows `again` (a second run in one process), `save` and `load` (a `Moment` put back) and `draw`. `tools/draw/play-check.ts` panics a game
  in the module and recovers it.

Native tests in `rust/game_start/tests/`, and `#[test]`s in `bytes.rs`, `owned.rs`, `dice.rs`:

| file | what it holds |
|---|---|
| `tests/words.rs` | a write moves its own part's word and no other; a machine's own moves none |
| `hosts_agree.rs`, `coop.rs` | two machines of a game agree in every part, each showing its own seat |
| `tick_ends.rs` | state is stepped by the tick alone; what a host asks moves no word |
| `no_host_in_rules.rs` | what only a picture shows goes out the same on every host |
| `seats.rs`, `two_seats.rs` | `SeatRef` for any seat; the tick is about every player |
| `tests/draw.rs`, `tests/saves.rs` | drawing changes nothing; a kept game put back |

## Not seen

None of this sees a GPU. No check keeps a float method out of `rust/game_rules`. Six dress rows (`air`, `snow`, `rain`, `wet_ground`, `wake`, `men`)
are read by nothing.
