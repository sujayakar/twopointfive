# tools/fold: what the Rust game never runs, and what only reads

**Still a tool to use, by hand, on a clean tree.** It deletes dead code from the Rust game and makes a function's signature say whether it writes. It
works on the tree as it stands, no check or gate runs it, and each run edits the source: a run is proved as the last section says. Its own tests are
`cargo test` here. It is not part of building or playing the game.

`fold` below is the program `cargo build --release --manifest-path tools/fold/Cargo.toml` makes, at target/release/fold in this folder. Run from the
root:

```sh
tools/fold/never.sh --tests-too           # what the game never runs, deleted, and said in lists
tools/fold/readers.sh --soften            # a function that cannot write the game takes it by a shared borrow
fold fields-drop --file LIST --write      # these fields leave their structs and every literal (`mod::Type.field` a line)
fold types-drop --file LIST --write       # these types leave (`mod::Type` a line)
```

Each check of the game crate takes several GB and minutes: run nothing heavy beside it.

## Where the game is

No pass and no script here names a crate. `fold layout` lists `rust/` and says what it found (`src/layout.rs` has the rules):

- the crate whose `src/knobs/mod.rs` declares `struct Knobs` holds the tables;
- the game's crates are that one and every plain library that depends on one of them, however far;
- a crate that builds something to run (a `src/bin`, a `[[bin]]`, a `cdylib`) is a user of the game, and so is every crate that is none of the game's:
  a name one of them spells keeps an item alive, and each is checked after a pass;
- a body crate is one of the game's crates with body files: a file of a module folder under `src/` that is not `mod.rs` or `types.rs`. (The tables'
  crate holds types and is left alone.) A module has body files in one crate only, or the run stops and says which two.

The passes read the same listing: `fold layout bodies` (the body crates, a crate after the ones it depends on), `users` (the crates that use one),
`body-src`, `knobs`, `page` (a crate that builds only a module for the page, checked for wasm). A check's output names a file from the folder of the
crate the check ran in; `fold diag-from-root --diag FILE --in rust/<crate>` rewrites it so every file is named from the root, and the passes read only
that.

## What the game never runs

Two steps, each its own run, each rerunnable on a later tree:

1. **`fold unnamed`: the functions nothing names.** A sweep by name, to a fixed point. With no `--write` it only reports.
2. **`tools/fold/sharp.sh`: the compiler's own reading, to a fixed point.** By name, a dead function that shares its name with a live function, a
   field or a local is missed. `sharp.sh` writes a copy of `rust/` in which every item of a body file is private to the crate, lifts the crate's
   `allow(dead_code)`, runs `cargo check --tests` on the copy and deletes from the real body files what the check names (functions, methods,
   constants, statics; a whole `impl` when all of it went), then runs again until a round deletes nothing. What stays public in the copy is what makes
   a thing live: any item whose name stands in ANOTHER CRATE under `rust/`: the module's face, what a host does round a step, the crate that stands a
   level natively with its binaries and its tests. The crates are found by listing `rust/`, never from a list kept here. The unit tests are live too,
   because the check is made with them: what only a test uses stays, by the compiler's own reading and not by name. So a function reached only through
   a table of handlers is live for as long as the table is, and a function only a test calls is kept (`sharp.sh --dry --no-tests` checks without the
   tests and so names those as well). Fields, variants and items of generated files the check names are listed, not deleted.

`tools/fold/never.sh` runs both and one more reading without the tests, and writes one report (never-run.md, or `--report FILE`): lines by module
before and after, and the lists. DELETED, NOTHING NAMED IT (the first look of step 1); DELETED, ONLY DEAD CODE NAMED IT (the later looks of step 1 and
all of step 2); KEPT ALTHOUGH THE GAME NEVER RUNS IT (what the last reading names: only a unit test uses it).

`never.sh --tests-too` goes on: a unit test that is there for code the game never runs goes, and then what it alone held up (`fold tests-cut`, then
the compiler's pass again). A test goes when it uses something only the tests use and calls no live function of a body file, itself or through its
module's own helpers; a test that does both is a test of live code leaning on a helper only tests use, and it stays with its helper. Which item a use
means is read off how it is written (`f(…)` its own module's, `m::f(…)` module m's, `T::f(…)` a method of T; `x.f(…)` can keep a test and never dooms
one), so the rule errs toward keeping. The program chooses only which TESTS go: what code goes after them is the compiler's reading again, so nothing
a remaining test or the game reaches can go. The report gains a fourth list, DELETED WITH THE TESTS THAT WERE ITS ONLY USERS, with the tests.

`never.sh` ends with ITS OWN PROOF: `cargo check --all-targets` in every crate under `rust/` (for wasm in the page's module), and it stops red where
one does not check, with the tree left as the passes left it. Run `cargo test --lib` in each body crate (`fold layout bodies`) after it.

## Readers

`tools/fold/readers.sh [--soften]` makes a function's signature say whether it writes. Every `&mut T` parameter of a body function is made `&T`;
`cargo check` names each place that needed the `mut`; it is put back on the parameter the error is about and along the calls (a function that hands
its parameter to a `&mut` parameter of another needs its own back), and the check runs again until it names nothing, for the game and then for the
crates that use it. What is left `&` is a reader by the compiler's word. It edits the body files in place, so run it on a tree with no changes of its
own there.

**`--soften`: a read should look like a read.** In the same pass, every read spelled as a write inside a body or one of the file's own `macro_rules!`
becomes its shared twin: `.as_mut()` `.as_ref()`, `.get_mut(` `.get(`, `.iter_mut()` `.iter()`, `.last_mut()`, `.first_mut()`, `.values_mut()`,
`.as_deref_mut()`, `&mut x` `&x`, `ref mut` `ref`, and a call of a function that has a READING TWIN becomes a call of the twin. The compiler says
which needed the `mut` after all: a softened place on a line an error points at (its own line, a note's, the macro's own text) goes back first; an
error about a local puts back every softened place of its function. A softened read that compiles reads the same thing: the shared twin of each of
these hands out the same place.

**Reading twins.** A block of the simulation is handed out as `&'static mut` from a slot number. Beside such a function a twin named `…_read` hands
the same thing out as `&'static` with no `mut` (`state_read` in `rust/sim/src/state.rs`, `of_read` in `rust/sim/src/weather.rs`: written
by hand). The pass finds the pairs by that shape and moves every call that compiles onto the twin.

**The taint: what the compiler cannot see.** A function can write with only `&Game` in hand through anything that is `unsafe`, a static or a cell, or
through a `&mut` some function hands back, which is how every block of the simulation is written. So after the compiler's rounds the program taints by
function: one whose words are `unsafe`, a `static mut`, a cell or an atomic, or that hands back a `&mut`; and to a fixed point everything that uses
one. A use is read off the function's own words, inside a macro call too: `f` its own module's (else any module's of that name), `m::f` module m's,
`sim::m::f` the simulation crate's (its functions are followed like the bodies', so a call into it taints only when what it calls can write; one it
cannot find is taken for a write), `T::f` and `x.f(…)` any method of that name; a `macro_rules!` of the file is followed like a function. A reading
twin is clean by its shape. A tainted function keeps `&mut` on its parameters. One that was WRITTEN `&Game` and is tainted is not changed (making it
`&mut` would break its callers' borrows): it is listed, and it is no reader for this program. It errs toward the taint.

The report counts by module the SURE READERS (`&Game`, no `&mut` parameter of any type, not tainted), lists the functions the taint contradicts, and
gives the compiler's bare answer before the taint as a second list (a map of who touches the struct, not a proof). How it runs: the files as they were
are kept beside the state file, and every round writes each file whole from that text plus what still stands changed, so words move inside their lines
and no line is added or lost (an error's line is the same line of the file from before). An error inside a macro is placed where the macro was called;
a function handed on as a value where a `&mut` one is wanted is found by the compiler's note on its signature; an error that fits nothing puts `mut`
back on every parameter of its file (counted), so a run always ends. What it does not do: split a macro that is used for reads and for writes (one
write through it keeps it `mut` for every user), or follow a read through the simulation crate's own private accessors (a function there that reads a
`static mut` is tainted like one that writes it).

## A field nothing reads

```sh
fold fields-drop --file LIST --write      # `mod::Type.field` a line: out of the Rust source
fold types-drop --file LIST --write       # a type whose last field went (`mod::Type` a line)
```

`fields-drop` takes each field out of its struct, out of every literal and pattern of that struct, and deletes a statement that only sets it
(`s.field = …;`) where the text itself says `s` is of the struct: a parameter of that type, a `let s: Type`, a `let s = Type { … }` or `Type::f(…)`
with `f` a function of the type that hands back `Self`. It looks in the game's crates and the crates that use them. A read that is left is a compile
error, which is the check: run `cargo check --all-targets` in every crate after it. A type only a dropped field used (an enum of a setting's words)
goes with `fold types-drop`: its declaration and its impls. `types-drop` writes nothing and says the places while anything still names the type.

## How a run of it is proved

A run is meant to change no behavior. The proof is `hold` (`rust/game_start`, `hold keep` on the tree before, `hold check` on the tree after: every
part of the game, every frame) and the gate, whose pins must not move; `cargo check --all-targets` in every crate, the module for wasm, with no new
kind of warning.
