# tools/fold — settled switch rows made constants of the Rust game

Every rule of the game arrived as a yes-or-no row in a knob table. `tools/qa/switches.json` lists every row with its value on the
page and its kind. A row of kind `settled` is a constant of the game: this program writes its value into the bodies of the Rust game
and deletes the side that can never run.

```sh
cargo run --release --manifest-path tools/fold/Cargo.toml -- --report fold-report.md            # dry: says what it would do
cargo run --release --manifest-path tools/fold/Cargo.toml -- --write --report fold-report.md    # and does it
cargo run --release --manifest-path tools/fold/Cargo.toml -- --table rows.tsv                   # every row that stays a switch, tried alone
cargo run --release --manifest-path tools/fold/Cargo.toml -- --knobs knobs-dead.md --drop drop.txt   # every leaf of the tables no body reads; the settled rows among them, one a line
```

Run it from the repository's root (or `--root DIR`). It reads `tools/qa/switches.json`, the game's `knobs/` (the tables at page
values: a row's value is taken from there and must agree with the list, or the run stops) and every body file
`<module>/{p_NN,b_NN}.rs` of the game's crates. It writes only body files, and a module's `mod.rs` where that hands a deleted function
on by name. It finds files by module folder, not by a list, so it can be run again after files move. A second run on its own output
changes nothing.

## Where the game is

No pass and no script here names a crate. `fold layout` lists `rust/` and says what it found (`src/layout.rs` has the rules):

- the crate whose `src/knobs/mod.rs` declares `struct Knobs` holds the tables;
- the game's crates are that one and every plain library that depends on one of them, however far;
- a crate that builds something to run (a `src/bin`, a `[[bin]]`, a `cdylib`) is a user of the game, and so is every crate that is
  none of the game's: a name one of them spells keeps an item alive, and each is checked after a pass;
- a body crate is one of the game's crates with body files: a file of a module folder under `src/` that is not `mod.rs`, `types.rs`
  or `missing.rs`. A module has body files in one crate only, or the run stops and says which two.

One crate today. When the game is split into crates the passes read the same listing: `fold layout bodies` (the body crates, a crate
after the ones it depends on), `users` (the crates that use one), `body-src`, `knobs`, `page` (a crate that builds only a module for
the page, checked for wasm). A check's output names a file from the folder of the crate the check ran in; `fold diag-from-root --diag
FILE --in rust/<crate>` rewrites it so every file is named from the root, and the passes read only that.

To settle a row, make it `settled` in `tools/qa/switches.json` (the page's hold of settled rows reads the same list) and run this
again. `--settle FILE` (one `TABLE.row` a line) tries more rows without touching the list.

## What it takes for a read

`g.k.<table>.<row>` where `g` is typed `Game` (a parameter, a closure's typed parameter, `let v = &mut *g;`); the same through a
copy or a borrow of a table that is not `mut` (`let k = g.k.cover;` … `k.on`), with scopes and shadowing followed. A read inside a
macro it can read as expressions (`format!`, `vec!`, `assert!`, `matches!` with its guard…) is taken.

NOT taken, and listed: a read through a parameter typed with a table's struct (`k: &CoverK`: the caller may hand in a copy it has
changed); a read off a root it cannot type.

## What it folds, and why each step keeps the game the same

It never rewrites a tree: it answers with text edits, applies the innermost, reads the file again, until nothing is left. Every
literal it writes carries a mark, and every rule but the first asks for the mark, so an `if true` or a `let debug = false` a hand
wrote is left alone. The marks are taken out at the end.

| step | what | why it is the same game |
|---|---|---|
| read | the read becomes the row's page value | the row is never written (below) and holds that value on every run |
| `!v` | the other literal | |
| `v && x`, `x && v`, `v \|\| x`, `x \|\| v` | the literal, or `x` | `x && false` and `x \|\| true` drop `x` only where `x` plainly has no effects (below); else the test stays and is listed |
| `if v {A} else {B}` | the side kept stands in its place, the other is deleted | as a statement its lines are spliced one level out, unless a name it declares is used after it (then it keeps its braces); as a value it is the side's expression or block |
| what follows a spliced `return`, `break`, `continue` | deleted | it can no longer run |
| `else if v {…}` in a chain | the chain is cut or closed there | |
| `let x = v;` (not `mut`) | each use of `x` becomes the literal, then the `let` goes | |
| `let k = g.k.cover;` nothing uses any more | deleted | |
| a block the fold emptied | an `if` round it goes where its test plainly has no effects (else listed); an emptied `else` goes; `if c {} else {B}` becomes `if !c {B}`; a `for` over it goes where what it walks plainly has no effects | |
| a function whose whole body is now one yes or no, or nothing | each call of it becomes that, where its arguments plainly have no effects | only where the path leaves no doubt which function it is: a bare name or `super::name` for this module's own, `crate::<module>::name` for another's, no local or nested function of that name in scope, and the module defines the name exactly ONCE (a private twin in another part, or a `cfg` pair, is a doubt; so is a function under a `cfg`) |
| a function nothing names any more | deleted, with its line in `mod.rs`; repeated until none is left | by NAME over every `.rs` under `rust/`: a name that still stands anywhere but at a definition or a `mod.rs` hand-on keeps every function of that name. A function that was unnamed BEFORE the fold is left (listed), and so is one only the tools' call surface names |

"Plainly has no effects" is judged by shape alone: literals, names, fields, indexing, arithmetic and comparisons, `&x` (never
`&mut x`), a short list of std methods (`is_some`, `len`, `abs`, `min` and `max` of two…), the `v3::` and `m::` helpers, and a call
of a function found to be a PLAIN READING (its body is `let`s of such expressions and one such expression; found to a fixed point,
under the same no-doubt rule for which function a call names). No other call counts as free of effects (most of the game's
functions take the whole game, and the die is one call away). A `for` whose body the fold emptied is left standing: walking a thing
can be an effect. Deleting a test can remove a panic (an index out of range in a test that no longer runs); it cannot add one.

It also stops, and lists the place, where folding could change a meaning quietly: an `if` or a statement under an attribute
(`#[cfg(test)] if …`); a side kept that ends in a bare number (`0.1`: its type may have come from the side deleted); a block that
holds a string running over lines (it keeps its braces, so its lines are not moved). A name a format string takes by itself
(`"{n}"`) counts as a use of that name everywhere a use matters.

What it does not weigh: a spliced block's locals now live to the end of the block it was spliced into. That is the same game unless
a local's drop does something; the game's bodies have no such type.

## What it will not fold

- **A row a body writes.** An assignment to the row (plain or `+=`), a `&mut` of it, a `&mut` or whole assignment of its table, a
  method called on its table, a `ref mut` taken of it, its name inside a macro the program cannot read or inside a `macro_rules!`
  body; through the game or through any name the program cannot type (then it is taken for a write of the game's own). The row is
  kept out with every place that writes it, under FINDINGS. Only body files are read for this: the generated wire writes every row
  as the page hands the tables over, by design. Either the row is not settled after all, or the write is dead and goes by hand;
  then name it in `keep.txt` (`written-ok TABLE.row  why`) and run again. `keep.txt` also keeps a row out by hand (`TABLE.row  why`).
  A bare `TABLE` in place of `TABLE.row` says it of every row of the table (a table laid whole from the page's own).
- **A number or a name.** A settled row that is a number stays a number in its table: written in, it would be a bare literal in a
  formula and nothing would fold.
- **A setter outside Rust.** The page's panel, its Options and a kept option write the TypeScript tables, which cross to the Rust game
  every tick. This program cannot see them: do not make a row `settled` in the list while the page still offers it.

## What the game never runs

Three steps, each its own run, each rerunnable on a later tree:

1. **What the fold left unnamed** goes with the fold (above).
2. **`--unnamed`: the functions nothing named before the fold either.** The same sweep by name, with no function spared. They are
   translations the Rust game never calls: a panel's summary string, a tuning panel's saver, a setter for an address word.
3. **`tools/fold/sharp.sh`: the compiler's own reading, to a fixed point.** By name, a dead function that shares its name with a
   live function, a field or a local is missed. `sharp.sh` writes a copy of `rust/` in which every item of a body file is private
   to the crate, lifts the crate's `allow(dead_code)`, runs `cargo check --tests` on the copy and deletes from the real body files
   what the check names (functions, methods, constants, statics; a whole `impl` when all of it went), then runs again until a round
   deletes nothing. What stays public in the copy is what makes a thing live: the generated call surface, the wire and the tables
   (never touched), and any item whose name stands in ANOTHER CRATE under `rust/`: the page's face, the runner, the crate that stands a
   level natively with its binaries and its tests. The crates are found by listing `rust/`, never from a list kept here: the pass once
   held two of them by name, and deleted what only the third called. The unit tests are live too, because the
   check is made with them: what only a test uses stays, by the compiler's own reading and not by name. So a function reached only
   through a table of handlers is live for as long as the table is, and a function only a test calls is kept (`sharp.sh --dry
   --no-tests` checks without the tests and so names those as well). Nothing inside a test module is deleted. A deleted function's
   line in `mod.rs` is dropped only from the list of its own part.

   ON A TREE WHERE `mod.rs` ALSO HANDS ON `sig::*` (stubs that are `todo!()`), a function deleted by mistake still compiles and
   panics when called. Run steps 2 and 3 for good on a tree without those stubs, where a missing function is a compile error. Fields, variants and items
   of generated files the check names are listed, not deleted.

`tools/fold/never.sh` runs steps 2 and 3 and one more reading without the tests, and writes one report: lines by module before and
after, and the lists. DELETED, NOTHING NAMED IT (the first look of step 2); DELETED, ONLY DEAD CODE NAMED IT (the later looks of step 2
and all of step 3); KEPT ALTHOUGH THE GAME NEVER RUNS IT (what the last reading names: only a unit test uses it).

`never.sh --tests-too` goes on: a unit test that is there for code the game never runs goes, and then what it alone held up
(`fold tests-cut`, then the compiler's pass again). A test goes when it uses something only the tests use and calls no live function of
a body file, itself or through its module's own helpers; a test that does both is a test of live code leaning on a helper only tests
use, and it stays with its helper. Which item a use means is read off how it is written (`f(…)` its own module's, `m::f(…)` module
m's, `T::f(…)` a method of T; `x.f(…)` can keep a test and never dooms one), so the rule errs toward keeping. The program chooses only
which TESTS go: what code goes after them is the compiler's reading again, so nothing a remaining test or the game reaches can go.
The report gains a fourth list, DELETED WITH THE TESTS THAT WERE ITS ONLY USERS, with the tests.

`never.sh` ends with ITS OWN PROOF: `cargo check --all-targets` in every crate under `rust/` (for wasm in the page's module), and it stops
red where one does not check. A kept run cannot stand in for this: it starts from exported bytes and never calls the functions that
stand a level. Run `cargo test --lib` in each body crate (`fold layout bodies`) after it.

## Readers

`tools/fold/readers.sh [--soften]` makes a function's signature say whether it writes. Every `&mut T` parameter of a body function is
made `&T`; `cargo check` names each place that needed the `mut`; it is put back on the parameter the error is about and along the calls
(a function that hands its parameter to a `&mut` parameter of another needs its own back), and the check runs again until it names
nothing, for the game and then for the crates that use it. What is left `&` is a reader by the compiler's word.

**`--soften`: a read should look like a read.** In the same pass, every read spelled as a write inside a body or one of the file's
own `macro_rules!` becomes its shared twin: `.as_mut()` `.as_ref()`, `.get_mut(` `.get(`, `.iter_mut()` `.iter()`, `.last_mut()`,
`.first_mut()`, `.values_mut()`, `.as_deref_mut()`, `&mut x` `&x`, `ref mut` `ref`, and a call of a function that has a READING TWIN
becomes a call of the twin. The compiler says which needed the `mut` after all: a softened place on a line an error points at (its own
line, a note's, the macro's own text) goes back first; an error about a local puts back every softened place of its function. Without
this a function that only reads still borrowed `&mut` because of how the read was spelled, and so did everything that called it.
A softened read that compiles reads the same thing: the shared twin of each of these hands out the same place.

**Reading twins.** A block of the sim was handed out as `&'static mut` from a slot number whether it was read or written. Beside such a
function a twin named `…_read` hands the same thing out as `&'static` with no `mut` (`sim::state::state_read`, a body's `wx_read`:
written by hand, `HAND.md` lists them). The pass finds the pairs by that shape and moves every call that compiles onto the twin.

**The taint: what the compiler cannot see.** A function can write with only `&Game` in hand through anything that is `unsafe`, a
static or a cell, or through a `&mut` some function hands back, which is how every block of the sim is written. So after the
compiler's rounds the program taints by function: one whose words are `unsafe`, a `static mut`, a cell or an atomic, or that hands
back a `&mut`; and to a fixed point everything that uses one. A use is read off the function's own words, inside a macro call too:
`f` its own module's (else any module's of that name), `m::f` module m's, `sim::m::f` the sim crate's (its functions are followed
like the bodies', so a call into the sim taints only when what it calls can write; one it cannot find is taken for a write), `T::f`
and `x.f(…)` any method of that name; a `macro_rules!` of the file is followed like a function. A reading twin is clean by its
shape. A tainted function keeps `&mut` on its parameters. One that was WRITTEN `&Game` and is tainted is not changed (making it
`&mut` would break its callers' borrows): it is listed, and it is no reader for this program. It errs toward the taint.

The report counts by module the SURE READERS (`&Game`, no `&mut` parameter of any type, not tainted), lists the functions the taint
contradicts, and gives the compiler's bare answer before the taint as a second list (a map of who touches the struct, not a proof).
How it runs: the files as they were are kept beside the state file, and every round writes each file whole from that text plus what
still stands changed, so words move inside their lines and no line is added or lost (an error's line is the same line of the file
from before). An error inside a macro is placed where the macro was called; a function handed on as a value where a `&mut` one is
wanted is found by the compiler's note on its signature; an error that fits nothing puts `mut` back on every parameter of its file
(counted), so a run always ends. What it does not do: split a macro that is used for reads and for writes (one write through it keeps
it `mut` for every user), or follow a read through the sim crate's own private accessors (a function of the sim that reads a
`static mut` is tainted like one that writes it).

## A field nothing reads

```sh
fold fields-drop --file gone.txt --write      # `mod::Type.field` a line: out of the Rust source
fold types-drop --file types.txt --write      # a type whose last field went (`mod::Type` a line)
```

`fields-drop` (first named `knobs-drop`, for a table's rows) takes each field out of its struct, out of every literal and pattern of
that struct, and deletes a statement that only sets it (`s.field = …;`) where the text itself says `s` is of the struct: a parameter
of that type, a `let s: Type`, a `let s = Type { … }` or `Type::f(…)` with `f` a function of the type that hands back `Self`. It
looks in the game's crates and the crates that use them. A read that is left is a compile error, which is the check: run `cargo check
--all-targets` in every crate after it. A type only a dropped field used (an enum of a setting's words) goes with
`fold types-drop`: its declaration and its impls. `types-drop` writes nothing and says the places while anything still names
the type.

## The report

Counts first; then edits by rule; functions settled whole and deleted; FINDINGS; what was left for a hand, each with its place and
the reason; every place a settled value now stands as a plain value (passed on, stored, returned); the settled rows no body reads or
writes at all (they live only in the tables and the wire); the functions nothing named before the fold.

## How a run of it is proved

The fold is meant to change no behaviour: the rows hold their page value on every run. So the proof is `hold` (`rust/game_start`, `hold keep` on the tree before, `hold check` on the tree after): every part the same after every
frame, and the gate, whose pins must not move. `cargo check` for the native runner and the wasm face, with no new kind of warning.
