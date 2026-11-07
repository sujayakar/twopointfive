#!/bin/bash
# WHAT THE GAME NEVER RUNS, BY THE COMPILER'S OWN READING, DELETED TO A FIXED POINT (tools/fold/README.md, "The sharp pass"):
#   tools/fold/sharp.sh [--dry] [--no-tests] [--scratch DIR]
# Each round: a copy of rust/ with every item of a body file private to the crate (but what rust/game_face or rust/game_run names),
# `cargo check --tests` on it with dead_code on (so a unit test keeps what it uses, by the compiler's own reading), and what the
# check names deleted from the real body files. It stops when a round deletes nothing. --dry runs one round and writes nothing.
# --no-tests checks without the tests: what it names beyond the other reading is what only a unit test uses (for the list).
# Exit: 0 done · 3 the private copy does not compile or a file could not be read. The game crate's check needs several GB: nothing heavy beside it.
cd "$(dirname "$0")/../.." || exit 64
DRY=; CHK=--tests; S="${TMPDIR:-/tmp}/fold-sharp"
while [ $# -gt 0 ]; do case "$1" in --dry) DRY=1; shift;; --no-tests) CHK=; shift;; --scratch) S=$2; shift 2;; *) echo "sharp.sh: no option $1"; exit 64;; esac; done
FOLD=tools/fold/target/release/fold
[ -x "$FOLD" ] || cargo build --release --manifest-path tools/fold/Cargo.toml > /dev/null 2>&1 || { echo "sharp.sh: tools/fold does not build"; exit 3; }
mkdir -p "$S" || exit 3
n=0
while :; do
  n=$((n + 1))
  "$FOLD" sharp-prep --scratch "$S" || exit 3
  ( cd "$S/rust/game" && CARGO_TARGET_DIR="$S/target" cargo check $CHK --message-format=json > "$S/diag-$n.json" 2> "$S/check-$n.log" ) || {
    grep -h '"level":"error"' "$S/diag-$n.json" | head -5 | cut -c1-600; tail -5 "$S/check-$n.log" | cut -c1-300
    echo "sharp.sh: round $n: the private copy does not compile ($S/diag-$n.json)"; exit 3; }
  echo "round $n:"
  "$FOLD" sharp-apply --diag "$S/diag-$n.json" --report "$S/round-$n.md" ${DRY:+} $( [ -z "$DRY" ] && echo --write ); r=$?
  [ -n "$DRY" ] && { echo "sharp.sh: --dry: one round, nothing written ($S/round-$n.md)"; exit 0; }
  [ $r = 0 ] && { echo "sharp.sh: nothing left to delete after $((n - 1)) round(s); the rounds' lists are $S/round-*.md"; exit 0; }
  [ $r = 10 ] || exit $r
done
