#!/bin/bash
# A DIFFERENTIAL FUZZ (tools/blitz/PLAY.md, "The fuzz"): one seed of tools/qa/soak.ts (a seeded schedule of random stimuli: orders, kills, teleports, restarts,
# mission steps, saves and loads) played twice, on the TypeScript tick under the fold (the reference) and on the Rust game, a line a tick on each side
# (the die's count, the host's half as handed in, a hash of each part of rust/sim's block) and the whole state every half minute; then fuzz-diff.ts (the
# first tick the two part and what the schedule did before it) and fuzz-dumps.ts (every pair of whole states, by path, canonically).
#   tools/blitz/fuzz.sh <seed> [--minutes 3] [--hard] [--out DIR] [--skip a,b] [--only a,b] [--side ts|rust]
# The folder keeps `cmd.txt` (this command), each side's tick lines and log, `dumps.txt` and `rust-hashes.txt` (tools/blitz/fuzz-replay.sh).
# EQUAL means all three: the die, the block, and 0 paths not known at every half minute.
# Exit: 0 equal · 1 the die or the block part, or one side stopped · 2 equal every tick, a whole state has a path not known · 3 a side did not run.
cd "$(dirname "$0")/../.." || exit 64
ARGS="$*"
SEED=$1; shift; case "$SEED" in ''|--*) sed -n '2,9p' "$0"; exit 64;; esac
MIN=3; HARD=; OUT=; SKIP=; ONLY=; SIDES="ts rust"
while [ $# -gt 0 ]; do case "$1" in --minutes) MIN=$2; shift 2;; --hard) HARD=hard; shift;; --out) OUT=$2; shift 2;; --skip) SKIP=$2; shift 2;; --only) ONLY=$2; shift 2;; --side) SIDES=$2; shift 2;; *) echo "fuzz.sh: no option $1"; exit 64;; esac; done
[ -n "$OUT" ] || OUT="${TMPDIR:-/tmp}/fuzz-$SEED${HARD:+-hard}"
mkdir -p "$OUT" || exit 3
[ "$SIDES" = "ts rust" ] && echo "FUZZ_CAP=${FUZZ_CAP:-900} tools/blitz/fuzz.sh $ARGS" | sed 's/ --out [^ ]*//' > "$OUT/cmd.txt"
LAST=$(awk "BEGIN { printf \"%d\", $MIN * 3600 }")   # (the soak steps 60 times a second)
DUMPS=${RH_DUMP:-$(seq -s, 1800 1800 $LAST)}
for W in $SIDES; do
  rm -f "$OUT/$W-run.txt" "$OUT/$W.log" "$OUT"/$W-t*.bin
  # (the reference is the TypeScript tick UNDER THE FOLD: the settled switch rows held at their page values, as the Rust game has them)
  env TPS_GAME=$W ${HARD:+SOAK_DIFFICULTY=hard} SOAK_SKIP="$SKIP" SOAK_ONLY="$ONLY" QA_FOLD=1 RH_TICKS="$OUT" RH_DUMP="$DUMPS" TPS_GAME_SAY=1 timeout -k 5 ${FUZZ_CAP:-900} bun run tools/qa/soak.ts --seeds "$SEED" --minutes "$MIN" > "$OUT/$W.log" 2>&1
  echo "exit $?" >> "$OUT/$W.log"
done
if [ "$SIDES" != "ts rust" ]; then bun run tools/blitz/fuzz-dumps.ts "$OUT" > /dev/null; exit 0; fi
bun run tools/blitz/fuzz-diff.ts "$OUT" | tee "$OUT/ticks.txt"; CODE=${PIPESTATUS[0]}
# THE WHOLE STATES EVERY HALF MINUTE, a path a line (tools/blitz/fuzz-dumps.ts): what the tick lines cannot see. The die and the block are the bodies and
# the dice; what a man was told, a talk's record, a module's own books are not in them, and a call that ran in TypeScript on the puppet and never reached
# the Rust game leaves both equal for minutes. Held up to the tick the die or the block parted: past it they are two games.
UPTO=$(bun run tools/blitz/fuzz-diff.ts "$OUT" --json | sed -n 's/.*"parted":\([0-9]*\).*/\1/p'); [ "${UPTO:-0}" -gt 0 ] || UPTO=99999999
bun run tools/blitz/fuzz-dumps.ts "$OUT" --upto "$UPTO"; D=$?
[ $CODE -eq 0 ] && [ $D -eq 1 ] && CODE=2
exit $CODE
