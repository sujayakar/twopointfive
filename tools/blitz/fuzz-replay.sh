#!/bin/bash
# A KEPT FUZZ RUN PLAYED AGAIN ON THE RUST GAME ALONE, and the first tick that differs (tools/blitz/PLAY.md, "Kept runs"). A kept folder is the Rust
# side of a run that was equal to the TypeScript tick by every measure (the die, the block, 0 paths not known every half minute), with the command that
# made it (`cmd.txt`). With the TypeScript tick gone this is what the fuzz is held to.
#   tools/blitz/fuzz-replay.sh <kept folder> [<kept folder> …] [--work DIR]
# Exit 0: every run named is the same play · 1: one differs · 3: one did not run.
cd "$(dirname "$0")/../.." || exit 64
WORK=; L=()
while [ $# -gt 0 ]; do case "$1" in --work) WORK=$2; shift 2;; *) L+=("$1"); shift;; esac; done
[ ${#L[@]} -gt 0 ] || { sed -n '2,6p' "$0"; exit 64; }
[ -n "$WORK" ] || WORK=$(mktemp -d "${TMPDIR:-/tmp}/fuzz-replay.XXXXXX")
CODE=0
for K in "${L[@]}"; do
  [ -s "$K/cmd.txt" ] || { echo "fuzz-replay: $K has no cmd.txt"; CODE=3; continue; }
  O="$WORK/$(basename "$K")"; rm -rf "$O"; mkdir -p "$O"
  # (the kept command, the Rust side alone, into a folder of its own)
  eval "$(cat "$K/cmd.txt") --side rust --out '$O'" > "$O/replay.out" 2>&1
  echo -n "$(basename "$K"): "; bun run tools/blitz/fuzz-replay.ts "$K" "$O"; C=$?
  [ $C -gt $CODE ] && CODE=$C
done
exit $CODE
