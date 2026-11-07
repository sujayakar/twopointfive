#!/bin/bash
# WHAT THE GAME NEVER RUNS, BY THE COMPILER'S OWN READING, DELETED TO A FIXED POINT (tools/fold/README.md, "The sharp pass"):
#   tools/fold/sharp.sh [--dry] [--no-tests] [--scratch DIR]
# Each round: a copy of rust/ with every item of a body file private to its crate (but what another crate under rust/ names),
# `cargo check --tests` on each crate that holds body files (`fold layout bodies`: found by listing rust/) with dead_code on (so a unit test keeps what it uses, by the compiler's own reading), and what the
# check names deleted from the real body files. It stops when a round deletes nothing. --dry runs one round and writes nothing.
# --no-tests checks without the tests: what it names beyond the other reading is what only a unit test uses (for the list).
# Exit: 0 done · 3 the private copy does not compile or a file could not be read. The game crate's check needs several GB: nothing heavy beside it.
cd "$(dirname "$0")/../.." || exit 64
DRY=; CHK=--tests; S="${TMPDIR:-/tmp}/fold-sharp"
while [ $# -gt 0 ]; do case "$1" in --dry) DRY=1; shift;; --no-tests) CHK=; shift;; --scratch) S=$2; shift 2;; *) echo "sharp.sh: no option $1"; exit 64;; esac; done
FOLD=tools/fold/target/release/fold
[ -x "$FOLD" ] || cargo build --release --manifest-path tools/fold/Cargo.toml > /dev/null 2>&1 || { echo "sharp.sh: tools/fold does not build"; exit 3; }
mkdir -p "$S" || exit 3
n=0; mended=0
while :; do
  n=$((n + 1))
  "$FOLD" sharp-prep --scratch "$S" || exit 3
  # each crate that holds body files is checked in the copy; what the checks say goes into one file, every file named from the root
  : > "$S/diag-$n.json"; : > "$S/check-$n.log"; ok=1
  for c in $("$FOLD" layout bodies); do
    ( cd "$S/rust/$c" && CARGO_TARGET_DIR="$S/target" cargo check $CHK --message-format=json > "$S/diag-one.json" 2>> "$S/check-$n.log" ) || ok=
    "$FOLD" diag-from-root --diag "$S/diag-one.json" --in "rust/$c" --base "$S" || exit 3
    cat "$S/diag-one.json" >> "$S/diag-$n.json"
    [ -n "$ok" ] || break
  done
  [ -n "$ok" ] || {
    # a `use` (of a test module, mostly) that still names what the round before deleted: the name goes, and the round runs again
    if [ -z "$DRY" ] && [ $mended -lt 8 ]; then
      "$FOLD" sharp-mend --diag "$S/diag-$n.json" --write; r=$?
      [ $r = 10 ] && { mended=$((mended + 1)); mv "$S/diag-$n.json" "$S/diag-$n-unmended-$mended.json"; n=$((n - 1)); continue; }
    fi
    grep -h '"level":"error"' "$S/diag-$n.json" | head -5 | cut -c1-600; tail -5 "$S/check-$n.log" | cut -c1-300
    echo "sharp.sh: round $n: the private copy does not compile ($S/diag-$n.json)"; exit 3; }
  echo "round $n:"
  "$FOLD" sharp-apply --diag "$S/diag-$n.json" --report "$S/round-$n.md" ${DRY:+} $( [ -z "$DRY" ] && echo --write ); r=$?
  [ -n "$DRY" ] && { echo "sharp.sh: --dry: one round, nothing written ($S/round-$n.md)"; exit 0; }
  [ $r = 0 ] && { echo "sharp.sh: nothing left to delete after $((n - 1)) round(s); the rounds' lists are $S/round-*.md"; exit 0; }
  [ $r = 10 ] || exit $r
done
