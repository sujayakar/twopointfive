#!/bin/bash
# A DIFFERENTIAL FUZZ (tools/blitz/PLAY.md, "The fuzz"): one seed of tools/qa/soak.ts (a seeded schedule of random stimuli: orders, kills, teleports, restarts,
# mission steps, saves and loads) played twice, on the TypeScript tick under the fold (the reference) and on the Rust game, a line a tick on each side
# (the die's count, the host's half as handed in, a hash of each part of rust/sim's block), then the first tick the two part and what the schedule did before it.
#   tools/blitz/fuzz.sh <seed> [--minutes 3] [--hard] [--out DIR] [--skip a,b] [--only a,b] [--side ts|rust]
# Exit: fuzz-diff.ts's (0 the two sides equal on every tick both played to the end · 1 they part · 3 a side did not run).
cd "$(dirname "$0")/../.." || exit 64
SEED=$1; shift; case "$SEED" in ''|--*) sed -n '2,6p' "$0"; exit 64;; esac
MIN=3; HARD=; OUT=; SKIP=; ONLY=; SIDES="ts rust"
while [ $# -gt 0 ]; do case "$1" in --minutes) MIN=$2; shift 2;; --hard) HARD=hard; shift;; --out) OUT=$2; shift 2;; --skip) SKIP=$2; shift 2;; --only) ONLY=$2; shift 2;; --side) SIDES=$2; shift 2;; *) echo "fuzz.sh: no option $1"; exit 64;; esac; done
[ -n "$OUT" ] || OUT="${TMPDIR:-/tmp}/fuzz-$SEED${HARD:+-hard}"
mkdir -p "$OUT" || exit 3
for W in $SIDES; do
  rm -f "$OUT/$W-run.txt" "$OUT/$W.log"
  # (the reference is the TypeScript tick UNDER THE FOLD: the settled switch rows held at their page values, as the Rust game has them)
  env TPS_GAME=$W ${HARD:+SOAK_DIFFICULTY=hard} SOAK_SKIP="$SKIP" SOAK_ONLY="$ONLY" QA_FOLD=1 RH_TICKS="$OUT" TPS_GAME_SAY=1 timeout -k 5 ${FUZZ_CAP:-900} bun run tools/qa/soak.ts --seeds "$SEED" --minutes "$MIN" > "$OUT/$W.log" 2>&1
  echo "exit $?" >> "$OUT/$W.log"
done
bun run tools/blitz/fuzz-diff.ts "$OUT"
