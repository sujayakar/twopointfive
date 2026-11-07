#!/bin/bash
# THE FUZZ ON A SQUAD MISSION (tools/blitz/PLAY.md, "The fuzz"): the scripted player of tools/qa/playthrough.ts plays one mission and seed with a seeded
# stream of perturbations laid over it between ticks (tools/blitz/perturb.ts: random rows of the orders menu on random doors, floor and men, throws, the
# player's fire, kills, the hostages' word, the mains, saves and loads, the pause card's restart), twice: on the TypeScript tick under the fold (the
# reference) and on the Rust game; a line a tick on each side, the whole states every half minute; then fuzz-diff.ts and the latest pair of whole states
# both sides kept, diffed by path.
#   tools/blitz/fuzz-squad.sh <mission> <seed> [--hard] [--rate K] [--skip a,b] [--only a,b] [--out DIR] [--side ts|rust]
# FUZZ_CAP=<seconds> (900): the longest a side may run; a side cut short is "one side stopped" and the ticks both played are still compared (the Rust side
# of a long mission on a loaded box needs more: PLAY.md, "What a tick costs").
# Exit: fuzz-diff.ts's (0 equal on every tick both played and both played as many · 1 they part · 3 a side did not run).
cd "$(dirname "$0")/../.." || exit 64
M=$1; SEED=$2; shift 2 || { sed -n '2,10p' "$0"; exit 64; }
HARD=; OUT=; SKIP=; ONLY=; RATE=1; SIDES="ts rust"
while [ $# -gt 0 ]; do case "$1" in --hard) HARD=1; shift;; --out) OUT=$2; shift 2;; --skip) SKIP=$2; shift 2;; --only) ONLY=$2; shift 2;; --rate) RATE=$2; shift 2;; --side) SIDES=$2; shift 2;; *) echo "fuzz-squad.sh: no option $1"; exit 64;; esac; done
[ -n "$OUT" ] || OUT="${TMPDIR:-/tmp}/fuzz-$M-$SEED${HARD:+-hard}"
mkdir -p "$OUT" || exit 3
DUMPS=1800,3600,5400,7200,9000,10800,12600,14400,16200,18000
for W in $SIDES; do
  rm -f "$OUT"/$W-*.txt "$OUT"/$W-t*.bin "$OUT/$W.log" "$OUT/$W.out"
  env TPS_GAME=$W QA_FOLD=1 QA_PERTURB=$SEED QA_PERTURB_RATE=$RATE QA_PERTURB_SKIP="$SKIP" QA_PERTURB_ONLY="$ONLY" RH_TICKS="$OUT" RH_DUMP=$DUMPS TPS_GAME_SAY=1 \
    timeout -k 5 ${FUZZ_CAP:-900} bun run tools/qa/playthrough.ts --scn "$M" --seeds "$SEED" --jobs 1 --json "$OUT/$W.json" ${HARD:+--set '{"DIFFICULTY":{"level":"hard"}}'} > "$OUT/$W.out" 2>&1
  echo "exit $?" >> "$OUT/$W.out"
  # (the reader takes each side's log as one file: the perturbations' lines, then what the tool itself printed)
  cat "$OUT/$W.out" >> "$OUT/$W.log"
done
bun run tools/blitz/fuzz-diff.ts "$OUT"; CODE=$?
L=; for k in $(echo $DUMPS | tr ',' ' '); do [ -s "$OUT/ts-t$k.bin" ] && [ -s "$OUT/rust-t$k.bin" ] && L=$k; done
if [ -n "$L" ]; then
  bun run tools/blitz/diff.ts "$OUT/ts-t$L.bin" "$OUT/rust-t$L.bin" --canon "$(cat tools/blitz/canon.txt),FirearmId" --known tools/blitz/known.txt --show 600 > "$OUT/end-diff.txt" 2>&1
  echo "the whole states as tick $L returned (the latest both sides kept): $(head -1 "$OUT/end-diff.txt" | cut -c1-300)"
fi
rm -f "$OUT"/*-t[0-9]*.bin
exit $CODE
