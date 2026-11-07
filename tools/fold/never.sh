#!/bin/bash
# WHAT THE RUST GAME NEVER RUNS, DELETED, AND SAID IN LISTS (tools/fold/README.md, "What the game never runs"):
#   tools/fold/never.sh [--tests-too] [--scratch DIR] [--report FILE]
# 1. `fold --unnamed --write`: the free functions of the body files that nothing names, to a fixed point.
# 2. `sharp.sh`: what the compiler's own reading still calls dead, the unit tests counted as live, to a fixed point.
# 3. `sharp.sh --dry --no-tests`: one more reading without the tests. It names what only a unit test uses.
# With --tests-too:
# 4. `fold tests-cut`: the tests that are there for what 3 named go (a test that also calls a live function stays);
# 5. `sharp.sh` again: what those tests alone held up goes, by the compiler's reading; then 3 once more for what is still kept.
# Last, `fold --write` once more: a settled row whose only writer was dead code is a constant now.
# Then every crate under rust/ is checked with all its targets, and the run stops red if one does not.
# The report: lines by module before and after, then the lists.
# Exit: 0 done · 3 a step failed (the tree is left as that step left it). Each check of the game crate takes several GB and about
# two minutes: nothing heavy beside it.
cd "$(dirname "$0")/../.." || exit 64
S="${TMPDIR:-/tmp}/fold-never"; R=never-run.md; TOO=
while [ $# -gt 0 ]; do case "$1" in --tests-too) TOO=1; shift;; --scratch) S=$2; shift 2;; --report) R=$2; shift 2;; *) echo "never.sh: no option $1"; exit 64;; esac; done
FOLD=tools/fold/target/release/fold
cargo build --release --manifest-path tools/fold/Cargo.toml > /dev/null 2>&1 || { echo "never.sh: tools/fold does not build"; exit 3; }
mkdir -p "$S" || exit 3

# lines of the body files, by module
count() { for d in rust/game/src/*/; do n=$(ls "$d"*.rs 2>/dev/null | grep -v '/mod\.rs$\|/types\.rs$\|/missing\.rs$' | xargs -r cat | wc -l); [ "$n" -gt 0 ] && echo "$(basename "$d") $n"; done; }
sum() { awk '{ s += $2 } END { print s + 0 }' "$1"; }
section() { awk -v h="$2" 'index($0, h) == 1 { on = 1; next } /^##/ { on = 0 } on && NF' "$1"; }
step() { "$@" > "$S/step.out" 2>&1 || { tail -8 "$S/step.out" | cut -c1-300; echo "never.sh: failed: $*"; exit 3; }; }
count > "$S/before.txt"

"$FOLD" --unnamed --write --report "$S/unnamed.md" > "$S/unnamed.out" || { cat "$S/unnamed.out"; echo "never.sh: the pass by name failed"; exit 3; }
count > "$S/mid.txt"
rm -rf "$S/sharp" "$S/kept" "$S/sharp2" "$S/kept2" "$S/tests.md"
step tools/fold/sharp.sh --scratch "$S/sharp"
step tools/fold/sharp.sh --dry --no-tests --scratch "$S/kept"
count > "$S/after.txt"
KEPT="$S/kept/round-1.md"
if [ -n "$TOO" ]; then
  "$FOLD" tests-cut --diag "$S/kept/diag-1.json" --write --report "$S/tests.md" > "$S/tests.out"; r=$?
  [ $r = 0 ] || [ $r = 10 ] || { cat "$S/tests.out"; echo "never.sh: the tests' cut failed"; exit 3; }
  if [ $r = 10 ]; then
    step tools/fold/sharp.sh --scratch "$S/sharp2"
    step tools/fold/sharp.sh --dry --no-tests --scratch "$S/kept2"
    KEPT="$S/kept2/round-1.md"
  fi
  count > "$S/after.txt"
fi

"$FOLD" --write --report "$S/fold-after.md" > "$S/fold-after.out" || { cat "$S/fold-after.out"; echo "never.sh: the fold after the passes failed"; exit 3; }
count > "$S/after.txt"
# THE CLOSING PROOF: every crate under rust/ still checks, with all its targets (its binaries and its tests; the face for wasm).
# A function the game never runs but another crate calls is live: a root this missed shows here as an error, and the run stops red.
for c in rust/*/; do
  [ -f "$c/Cargo.toml" ] || continue
  T=; [ "$(basename "$c")" = game_face ] && T="--target wasm32-unknown-unknown"
  ( cd "$c" && cargo check --all-targets $T > "$S/close-$(basename "$c").log" 2>&1 ) || {
    grep -n '^error' -A 6 "$S/close-$(basename "$c").log" | head -30 | cut -c1-240
    echo "never.sh: $(basename "$c") does not check after the passes ($S/close-$(basename "$c").log): something it uses was deleted. THE TREE IS LEFT AS THE PASSES LEFT IT."; exit 3; }
done
B=$(sum "$S/before.txt"); M=$(sum "$S/mid.txt"); A=$(sum "$S/after.txt")
N1=$(section "$S/unnamed.md" "### Nothing names it" | tr ',' '\n' | grep -c .)
N2=$(section "$S/unnamed.md" "### Only a function deleted here named it" | tr ',' '\n' | grep -c .)
N3=$(cat "$S"/sharp/round-*.md | grep -c '^- .* lines)$')
N4=0; [ -d "$S/sharp2" ] && N4=$(cat "$S"/sharp2/round-*.md | grep -c '^- .* lines)$')
{
  echo "# What the Rust game never runs: deleted, and what stays"
  echo
  echo "Body files of rust/game: $B lines before, $M after the pass by name, $A at the end ($((B - A)) fewer)."
  echo
  echo "- deleted, nothing named it: $N1 functions"
  echo "- deleted, only a function deleted here named it: $N2 functions"
  echo "- deleted, the compiler's reading (reached from nothing the game, another crate under rust/ or a unit test uses): $N3 items"
  [ -f "$S/tests.md" ] && echo "- deleted with the tests that were its only users: $N4 items, and $(head -1 "$S/tests.md")"
  echo "- settled rows whose only writer went with the dead code, now constants:$(sed -n 's/^  folded://p' "$S/fold-after.out")"
  echo "- KEPT although the game never runs it, because a unit test that stays uses it: $(head -1 "$KEPT" | sed 's/ (nothing written: --write)//')"
  echo
  echo "## Lines by module, where they changed (before -> after)"
  echo
  join "$S/before.txt" "$S/after.txt" | awk '$2 != $3 { printf "- %s: %d -> %d (%d fewer)\n", $1, $2, $3, $2 - $3 }'
  echo
  echo "## Deleted: nothing named it ($N1)"
  echo
  section "$S/unnamed.md" "### Nothing names it"
  echo
  echo "## Deleted: only a function deleted here named it ($N2)"
  echo
  section "$S/unnamed.md" "### Only a function deleted here named it"
  echo
  echo "## Deleted: dead by the compiler's reading ($N3)"
  echo
  echo "Each has a name something still spells (another function of the same name, itself, a function that is dead in turn), so the"
  echo "pass by name kept it; the compiler follows the calls and finds none from the game, another crate under rust/ or a unit test."
  echo
  for f in "$S"/sharp/round-*.md; do section "$f" "## Deleted"; done
  if [ -f "$S/tests.md" ]; then
    echo
    echo "## Deleted with the tests that were its only users ($N4)"
    echo
    echo "Only unit tests reached these. The tests went first (below), then the compiler's reading found these with nothing left to"
    echo "reach them. A helper inside a test module that no test uses any more is on this list too."
    echo
    for f in "$S"/sharp2/round-*.md; do [ -f "$f" ] && section "$f" "## Deleted"; done
    echo
    sed -n '/^## Tests deleted/,$p' "$S/tests.md" | sed 's/^## /### /'
  fi
  echo
  echo "## Kept although the game never runs it: a unit test that stays uses it"
  echo
  echo "The compiler calls these dead once the unit tests are left out of the check. They stay because a test of the crate reaches them:"
  echo "a test of live code that leans on them, or (without --tests-too) any test at all."
  echo
  section "$KEPT" "## Deleted"
  echo
  echo "## Named dead by the compiler and not an item of a body file"
  echo
  section "$KEPT" "## Named dead by the compiler"
} > "$R"
head -9 "$R"
echo "never.sh: the report is $R"
