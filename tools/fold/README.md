# tools/fold — settled switch rows made constants of the Rust game

Every rule of the game arrived as a yes-or-no row in a knob table. `tools/qa/switches.json` lists every row with its value on the
page and its kind. A row of kind `settled` is a constant of the game: this program writes its value into the bodies of `rust/game`
and deletes the side that can never run.

```sh
cargo run --release --manifest-path tools/fold/Cargo.toml -- --report fold-report.md            # dry: says what it would do
cargo run --release --manifest-path tools/fold/Cargo.toml -- --write --report fold-report.md    # and does it
cargo run --release --manifest-path tools/fold/Cargo.toml -- --table rows.tsv                   # every row that stays a switch, tried alone
cargo run --release --manifest-path tools/fold/Cargo.toml -- --knobs knobs-dead.md --drop drop.txt   # every leaf of the tables no body reads; the settled rows among them, one a line
```

Run it from the repository's root (or `--root DIR`). It reads `tools/qa/switches.json`, `rust/game/src/knobs.rs` (the tables at page
values: a row's value is taken from there and must agree with the list, or the run stops) and every body file
`rust/game/src/<module>/{p_NN,b_NN}.rs`. It writes only body files, and a module's `mod.rs` where that hands a deleted function on by
name. It finds files by module folder, not by a list, so it can be run again after files move. A second run on its own output changes
nothing.

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
   (never touched), and any item whose name stands in `rust/game_face` or `rust/game_run`. The unit tests are live too, because the
   check is made with them: what only a test uses stays, by the compiler's own reading and not by name. So a function reached only
   through a table of handlers is live for as long as the table is, and a function only a test calls is kept (`sharp.sh --dry
   --no-tests` checks without the tests and so names those as well). Nothing inside a test module is deleted. A deleted function's
   line in `mod.rs` is dropped only from the list of its own part.

   ON A TREE WHERE `mod.rs` ALSO HANDS ON `sig::*` (stubs that are `todo!()`), a function deleted by mistake still compiles and
   panics when called. Run steps 2 and 3 for good on a tree without those stubs, where a missing function is a compile error. Fields, variants and items
   of generated files the check names are listed, not deleted.

`tools/fold/never.sh` runs steps 2 and 3 and one more reading with no test holding anything, and writes one report: lines by module
before and after, and three lists. DELETED, NOTHING NAMED IT (the first look of step 2); DELETED, ONLY DEAD CODE NAMED IT (the later
looks of step 2 and all of step 3); KEPT ALTHOUGH THE GAME NEVER RUNS IT (what the last reading names: only a unit test uses it, and
deleting it is deleting that test, a hand's call).

## Readers

`tools/fold/readers.sh` makes a function's signature say whether it writes. Every `&mut T` parameter of a body function is made
`&T`; `cargo check` names each place that needed the `mut`; it is put back on the parameter the error is about and along the calls
(a function that hands its parameter to a `&mut` parameter of another needs its own back), and the check runs again until it names
nothing, for the game and then for the crates that use it. What is left `&` is a reader by the compiler's word.

The compiler cannot see one kind of write: `sim::state::state(slot)` hands out the simulation block from a number, and a static or
a cell can be written through `&`. So after the compiler's rounds the program taints by function: one that names `sim::`, a
static or a cell, or hands back a `&mut`, and every function that calls one (by module and name, to a fixed point), keeps `&mut`.
Then `&Game` means "moves nothing" and not only "moves nothing of the Game struct". It is a conservative reading: a pure query of
`sim::` taints too. The report counts readers by module, lists the rows of the call surface whose function takes `&Game`, and
gives the compiler's bare answer before the taint as a second list (a map of who touches the struct, not a proof).

## The report

Counts first; then edits by rule; functions settled whole and deleted; FINDINGS; what was left for a hand, each with its place and
the reason; every place a settled value now stands as a plain value (passed on, stored, returned); the settled rows no body reads or
writes at all (they live only in the tables and the wire); the functions nothing named before the fold.

## How a run of it is proved

The fold is meant to change no behaviour: the rows hold their page value on every run. So the proof is the mission loop's kept runs
(`tools/blitz/keep.sh` on the tree before, `tools/blitz/hold.sh` on the tree after): the same die and the same block after every
tick. `cargo check` for the native runner and the wasm face, with no new kind of warning.
