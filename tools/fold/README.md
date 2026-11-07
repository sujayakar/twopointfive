# tools/fold — settled switch rows made constants of the Rust game

Every rule of the game arrived as a yes-or-no row in a knob table. `tools/qa/switches.json` lists every row with its value on the
page and its kind. A row of kind `settled` is a constant of the game: this program writes its value into the bodies of `rust/game`
and deletes the side that can never run.

```sh
cargo run --release --manifest-path tools/fold/Cargo.toml -- --report fold-report.md            # dry: says what it would do
cargo run --release --manifest-path tools/fold/Cargo.toml -- --write --report fold-report.md    # and does it
cargo run --release --manifest-path tools/fold/Cargo.toml -- --table rows.tsv                   # every row that stays a switch, tried alone
```

Run it from the repository's root (or `--root DIR`). It reads `tools/qa/switches.json`, `rust/game/src/knobs.rs` (the tables at page
values: a row's value is taken from there and must agree with the list, or the run stops) and every body file
`rust/game/src/<module>/{p_NN,b_NN}.rs`. It writes only body files, and a module's `mod.rs` where that hands a deleted function on by
name. It finds files by module folder, not by a list, so it can be run again after files move. A second run on its own output changes
nothing.

To settle a row, make it `settled` in `tools/qa/switches.json` (the page's hold of settled rows reads the same list) and run this
again. `--settle FILE` (one `TABLE.row` a line) tries more rows without touching the list.

## What it takes for a read

`g.k.<table>.<row>` where `g` is a parameter typed `Game`; the same through a copy or a borrow of a table
(`let k = g.k.cover;` … `k.on`), with scopes and shadowing followed; the same through a parameter typed with a table's struct
(`k: &CoverK`). A read inside a macro it can read as expressions (`format!`, `vec!`, `assert!`, `matches!`…) is taken; one inside any
other macro is left and listed.

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
| a function whose whole body is now one yes or no, or nothing | each call of it becomes that, where its arguments plainly have no effects | a bare call is taken for this module's function only where the module defines it and no local shadows the name |
| a function nothing names any more | deleted, with its line in `mod.rs`; repeated until none is left | by NAME over every `.rs` under `rust/`: a name that still stands anywhere but at a definition or a `mod.rs` hand-on keeps every function of that name. A function that was unnamed BEFORE the fold is left (listed), and so is one only the tools' call surface names |

"Plainly has no effects" is judged by shape alone: literals, names, fields, indexing, arithmetic and comparisons, a short list of
std methods (`is_some`, `len`, `abs`…), the `v3::` and `m::` helpers. No other call counts as free of effects (most of the game's
functions take the whole game, and the die is one call away). Deleting such a test can remove a panic (an index out of range in a
test that no longer runs); it cannot add one.

## What it will not fold

- **A row a body writes.** An assignment to the row, a `&mut` of it, a `&mut` or whole assignment of its table: the row is kept out
  with every place that writes it, under FINDINGS. Either the row is not settled after all, or the write is dead and goes by hand;
  then name it in `keep.txt` (`written-ok TABLE.row  why`) and run again. `keep.txt` also keeps a row out by hand (`TABLE.row  why`).
- **A number or a name.** A settled row that is a number stays a number in its table: written in, it would be a bare literal in a
  formula and nothing would fold.
- **A setter outside Rust.** The page's panel, its Options and a kept option write the TypeScript tables, which cross to the Rust game
  every tick. This program cannot see them: do not make a row `settled` in the list while the page still offers it.

## The report

Counts first; then edits by rule; functions settled whole and deleted; FINDINGS; what was left for a hand, each with its place and
the reason; every place a settled value now stands as a plain value (passed on, stored, returned); the settled rows no body reads or
writes at all (they live only in the tables and the wire); the functions nothing named before the fold.

## How a run of it is proved

The fold is meant to change no behaviour: the rows hold their page value on every run. So the proof is the mission loop's kept runs
(`tools/blitz/keep.sh` on the tree before, `tools/blitz/hold.sh` on the tree after): the same die and the same block after every
tick. `cargo check` for the native runner and the wasm face, with no new kind of warning.
