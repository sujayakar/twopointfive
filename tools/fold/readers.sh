#!/bin/bash
# WHETHER A FUNCTION WRITES, SAID BY ITS SIGNATURE (tools/fold/README.md, "Readers"):
#   tools/fold/readers.sh [--report FILE] [--scratch DIR]
# Every `&mut T` parameter of a body function is made `&T`; `cargo check` of the game, then of the crates that use it, names where the
# `mut` was needed; it is put back there and along the calls, and the check runs again until it names nothing. It edits rust/game's body
# files in place: run it on a clean tree. Exit: 0 done · 3 could not run · 4 an error the program could not place (the tree is left as it was).
cd "$(dirname "$0")/../.." || exit 64
S="${TMPDIR:-/tmp}/fold-readers"; REPORT=
while [ $# -gt 0 ]; do case "$1" in --scratch) S=$2; shift 2;; --report) REPORT=$2; shift 2;; *) echo "readers.sh: no option $1"; exit 64;; esac; done
FOLD=tools/fold/target/release/fold
[ -x "$FOLD" ] || cargo build --release --manifest-path tools/fold/Cargo.toml > /dev/null 2>&1 || { echo "readers.sh: tools/fold does not build"; exit 3; }
[ -z "$(git status --porcelain rust/game/src)" ] || { echo "readers.sh: rust/game/src has changes of its own: commit them first"; exit 3; }
mkdir -p "$S" || exit 3
back() { git checkout -q -- rust/game/src; }
"$FOLD" readers-demote --state "$S/state.txt" || { back; exit 3; }
"$FOLD" readers-restore --state "$S/state.txt"       # the calls and the writes past the compiler's sight, before the first check
n=0
for crate in game game_run game_face; do
  while :; do
    n=$((n + 1))
    T=; [ $crate = game_face ] && T="--target wasm32-unknown-unknown"
    ( cd rust/$crate && cargo check $T --message-format=json > "$S/diag-$n.json" 2> "$S/check-$n.log" ) && break
    echo "check $n ($crate): $(grep -c '"level":"error"' "$S/diag-$n.json") errors"
    "$FOLD" readers-restore --state "$S/state.txt" --diag "$S/diag-$n.json"; r=$?
    [ $r = 10 ] && continue
    echo "readers.sh: the check fails and nothing was put back (exit $r; $S/diag-$n.json): the tree is left as it was"; back; exit 4
  done
done
"$FOLD" readers-report --state "$S/state.txt" > "${REPORT:-$S/report.md}"
head -4 "${REPORT:-$S/report.md}"
echo "readers.sh: done after $n checks; the report is ${REPORT:-$S/report.md}"
