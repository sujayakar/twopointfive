#!/bin/bash
# THE PACKED RUNS TAKEN AGAIN FOR THIS TREE'S LAYOUT OF THE WIRE (tools/blitz/PLAY.md, "Kept runs"). A packed run's host-half words and whole states'
# hashes are of one layout (`layout` in the file, `schema.layout`); its `play` words (the die's count and the block's parts, every tick) are of none.
# After a change of the wire this plays each packed run on the Rust game, holds the new play to the `play` words, and ONLY WHERE THEY ARE EQUAL writes
# the file again from the new play (its lines, its states, this tree and layout). A run whose play differs is left as it is and named.
#   tools/blitz/fuzz-retake.sh [tools/blitz/kept/NAME.kept …] [--work DIR]        (no name: every file of tools/blitz/kept)
# Exit 0: every run named is this layout's now (taken again, or it already was and is the same play) · 1: a run's play differs: not taken · 3: one did not run.
# A file taken again pins the Rust game to itself in what the play's words do not hold (the host's half, the whole states): read the diff of the file.
cd "$(dirname "$0")/../.." || exit 64
WORK=; L=()
while [ $# -gt 0 ]; do case "$1" in --work) WORK=$2; shift 2;; *) L+=("$1"); shift;; esac; done
[ ${#L[@]} -gt 0 ] || L=(tools/blitz/kept/*.kept)
[ -n "$WORK" ] || WORK=$(mktemp -d "${TMPDIR:-/tmp}/fuzz-retake.XXXXXX")
CODE=0; up() { [ "$1" -gt $CODE ] && CODE=$1; }
for K in "${L[@]}"; do
  N=$(basename "$K" .kept); OUT=$(tools/blitz/fuzz-replay.sh "$K" --work "$WORK"); C=$?
  case $C in
    0) echo "$N: of this layout already, and the same play";;
    4) bun run tools/blitz/fuzz-replay.ts --pack "$WORK/$N" > "$WORK/$N.kept" && [ -s "$WORK/$N.kept" ] && grep -q '^cmd .' "$WORK/$N.kept" || { echo "$N: could not be packed"; up 3; continue; }
       cp "$WORK/$N.kept" "$K"; echo "$N: TAKEN AGAIN ($(sed -n 's/^ticks //p' "$K") ticks, the die at $(sed -n 's/^die //p' "$K"), layout $(sed -n 's/^layout //p' "$K")): the play's words were the packed run's";;
    1) echo "$N: NOT TAKEN: $OUT"; up 1;;
    *) echo "$N: did not run: $OUT"; up 3;;
  esac
done
exit $CODE
