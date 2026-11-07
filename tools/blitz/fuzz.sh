#!/bin/bash
# A DIFFERENTIAL FUZZ (tools/blitz/PLAY.md, "The fuzz"): one seed of tools/qa/soak.ts (a seeded schedule of random stimuli: orders, kills, teleports, restarts,
# mission steps, saves and loads) played twice, on the TypeScript tick under the fold (the reference) and on the Rust game, a line a tick on each side
# (the die's count, the host's half as handed in, a hash of each part of rust/sim's block), then the first tick the two part and what the schedule did before it.
#   tools/blitz/fuzz.sh <seed> [--minutes 3] [--hard] [--out DIR] [--skip a,b] [--only a,b] [--side ts|rust]
# Then the two whole states as the last tick returned, diffed by path (end-diff.txt).
# Exit: fuzz-diff.ts's (0 the two sides equal on every tick both played to the end · 1 they part · 3 a side did not run).
cd "$(dirname "$0")/../.." || exit 64
SEED=$1; shift; case "$SEED" in ''|--*) sed -n '2,6p' "$0"; exit 64;; esac
MIN=3; HARD=; OUT=; SKIP=; ONLY=; SIDES="ts rust"
while [ $# -gt 0 ]; do case "$1" in --minutes) MIN=$2; shift 2;; --hard) HARD=hard; shift;; --out) OUT=$2; shift 2;; --skip) SKIP=$2; shift 2;; --only) ONLY=$2; shift 2;; --side) SIDES=$2; shift 2;; *) echo "fuzz.sh: no option $1"; exit 64;; esac; done
[ -n "$OUT" ] || OUT="${TMPDIR:-/tmp}/fuzz-$SEED${HARD:+-hard}"
mkdir -p "$OUT" || exit 3
LAST=$(awk "BEGIN { printf \"%d\", $MIN * 3600 }")   # (the soak steps 60 times a second)
for W in $SIDES; do
  rm -f "$OUT/$W-run.txt" "$OUT/$W.log"
  # (the reference is the TypeScript tick UNDER THE FOLD: the settled switch rows held at their page values, as the Rust game has them)
  env TPS_GAME=$W ${HARD:+SOAK_DIFFICULTY=hard} SOAK_SKIP="$SKIP" SOAK_ONLY="$ONLY" QA_FOLD=1 RH_TICKS="$OUT" RH_DUMP="${RH_DUMP:-$LAST}" TPS_GAME_SAY=1 timeout -k 5 ${FUZZ_CAP:-900} bun run tools/qa/soak.ts --seeds "$SEED" --minutes "$MIN" > "$OUT/$W.log" 2>&1
  echo "exit $?" >> "$OUT/$W.log"
done
bun run tools/blitz/fuzz-diff.ts "$OUT"; CODE=$?
# THE WHOLE STATES AS THE LAST TICK RETURNS, a path a line (tools/blitz/diff.ts): what the tick lines cannot see. The die and the block are the bodies and
# the dice; what a man was told, a talk's record, a module's own books are not in them, and a call that ran in TypeScript on the puppet and never reached
# the Rust game leaves both equal for minutes. Kept as end-diff.txt; the two states are deleted.
if [ -s "$OUT/ts-t$LAST.bin" ] && [ -s "$OUT/rust-t$LAST.bin" ]; then
  bun run tools/blitz/diff.ts "$OUT/ts-t$LAST.bin" "$OUT/rust-t$LAST.bin" --canon "$(cat tools/blitz/canon.txt),FirearmId" --known tools/blitz/known.txt --show 600 > "$OUT/end-diff.txt" 2>&1
  echo "the whole states as tick $LAST returned: $(head -1 "$OUT/end-diff.txt" | cut -c1-300)"
fi
rm -f "$OUT"/*-t[0-9]*.bin
exit $CODE
