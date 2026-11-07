#!/bin/bash
# WHAT THE RUST GAME NEVER RUNS, DELETED, AND SAID IN THREE LISTS (tools/fold/README.md, "What the game never runs"):
#   tools/fold/never.sh [--scratch DIR] [--report FILE]
# 1. `fold --unnamed --write`: the free functions of the body files that nothing names, to a fixed point.
# 2. `sharp.sh`: what the compiler's own reading still calls dead, the unit tests counted as live, to a fixed point.
# 3. `sharp.sh --dry --no-tests`: one more reading without the tests. Nothing is deleted: it is the list of what stays
#    although the game never runs it.
# The report: lines by module before and after, then the three lists.
# Exit: 0 done · 3 a step failed (the tree is left as that step left it). Three or more checks of the game crate: several GB each,
# one at a time, about two minutes a check.
cd "$(dirname "$0")/../.." || exit 64
S="${TMPDIR:-/tmp}/fold-never"; R=never-run.md
while [ $# -gt 0 ]; do case "$1" in --scratch) S=$2; shift 2;; --report) R=$2; shift 2;; *) echo "never.sh: no option $1"; exit 64;; esac; done
FOLD=tools/fold/target/release/fold
cargo build --release --manifest-path tools/fold/Cargo.toml > /dev/null 2>&1 || { echo "never.sh: tools/fold does not build"; exit 3; }
mkdir -p "$S" || exit 3

# lines of the body files, by module
count() { for d in rust/game/src/*/; do n=$(cat "$d"p_[0-9]*.rs "$d"b_[0-9]*.rs 2>/dev/null | wc -l); [ "$n" -gt 0 ] && echo "$(basename "$d") $n"; done; }
count > "$S/before.txt"

"$FOLD" --unnamed --write --report "$S/unnamed.md" > "$S/unnamed.out" || { cat "$S/unnamed.out"; echo "never.sh: the pass by name failed"; exit 3; }
count > "$S/mid.txt"
rm -rf "$S/sharp" "$S/kept"
tools/fold/sharp.sh --scratch "$S/sharp" > "$S/sharp.out" 2>&1 || { tail -8 "$S/sharp.out"; echo "never.sh: the compiler's pass failed"; exit 3; }
count > "$S/after.txt"
tools/fold/sharp.sh --dry --no-tests --scratch "$S/kept" > "$S/kept.out" 2>&1 || { tail -8 "$S/kept.out"; echo "never.sh: the reading with no test holding failed"; exit 3; }

sum() { awk '{ s += $2 } END { print s + 0 }' "$1"; }
section() { awk -v h="$2" 'index($0, h) == 1 { on = 1; next } /^##/ { on = 0 } on && NF' "$1"; }
B=$(sum "$S/before.txt"); M=$(sum "$S/mid.txt"); A=$(sum "$S/after.txt")
N1=$(section "$S/unnamed.md" "### Nothing names it" | tr ',' '\n' | grep -c .)
N2=$(section "$S/unnamed.md" "### Only a function deleted here named it" | tr ',' '\n' | grep -c .)
N3=$(cat "$S"/sharp/round-*.md | grep -c '^- .* lines)$')
K=$(section "$S/kept/round-1.md" "## Deleted" | grep -c '^- ')
KL=$(head -1 "$S/kept/round-1.md")
{
  echo "# What the Rust game never runs: deleted, and what stays"
  echo
  echo "Body files of rust/game: $B lines before, $M after the pass by name, $A after the compiler's pass ($((B - A)) fewer)."
  echo
  echo "- deleted, nothing named it: $N1 functions"
  echo "- deleted, only a function deleted here named it: $N2 functions"
  echo "- deleted, the compiler's reading (reached from nothing the game, its face, its runner or a unit test uses): $N3 items"
  echo "- KEPT although the game never runs it, because a unit test uses it: $KL"
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
  echo "pass by name kept it; the compiler follows the calls and finds none from the game, its face, its runner or a unit test."
  echo
  for f in "$S"/sharp/round-*.md; do section "$f" "## Deleted"; done
  echo
  echo "## Kept although the game never runs it: a unit test uses it"
  echo
  echo "The compiler calls these dead once the unit tests are left out of the check. They stay because a test of the crate reaches them:"
  echo "deleting one means deleting or rewriting that test, which is a hand's call."
  echo
  section "$S/kept/round-1.md" "## Deleted"
  echo
  echo "## Named dead by the compiler and not an item of a body file"
  echo
  section "$S/kept/round-1.md" "## Named dead by the compiler"
} > "$R"
head -8 "$R"
echo "never.sh: the report is $R"
