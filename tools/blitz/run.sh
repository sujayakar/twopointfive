#!/bin/bash
# THE MISSION LOOP's one command (tools/blitz/README.md):
#   tools/blitz/run.sh <mission>:<seed> [--ticks N] [--keep "1,2,100-110"] [--every N] [--open "559"] [--after N] [--out DIR] [--show N] [--blobs] [--no-ts] [--no-build] [--wall] [--dress | --dress-rows "a,b"]
# 1. the TypeScript side once (mission.ts): the host's half every tick, a hash of each part of rust/sim's block and the die's count after every tick, whole
#    states only at the kept ticks;   2. the native runner (rust/game_run) builds and steps the Rust game from tick 1, comparing after EVERY tick and stopping
#    at the first tick that parts;   3. the path diff of every tick both sides kept, minus known.txt.
# Exit: 0 in step and no unknown path · 1 the block or the die parted, or a kept tick has an unknown path · 2 the Rust tick panicked · 3 a side could not run ·
# 4 the game parted at one of the loop's INTENDED differences and nowhere before it (the list is `intended` below).
# Needs bun and cargo on the PATH, and nothing outside the repository but the folder it names on its first line (--out, or one under $TMPDIR).
# A kept tick's Rust state (n<k>.bin) is deleted once it is diffed, unless --blobs.
# A name two-roomclear | two-nightbreach | two-compound is a game of TWO SEATS (seats.ts for the TypeScript side); --asks then prints where the
# TypeScript's tick asked the bare Game for a seat's field. BLITZ_RUNNER=<path> steps with that runner (a copy kept apart while the tree is built again).
# --wall: the scripted commander takes a held room by a wall charge where a partition allows (mission.ts; the order on a wall and its blow are replayed).
# --dress: both sides stand with the DRESSING ROWS UP, as the page has them (dress.ts; README "The dressing rows up"); --dress-rows "moths,fx" raises only those.
# --no-ts takes the TypeScript side as --out already holds it (the same mission, ticks and kept ticks); --no-build takes the runner as it is built.
cd "$(dirname "$0")/../.." || exit 64
unset BUN_OPTIONS
M=$1; shift
case "$M" in ''|--*) sed -n '2,3p' "$0"; exit 64;; esac
TICKS=; KEEP=; ASKS=; WALL=; DRESSW=; EVERY=0; OPEN=; AFTER=0; OUT=; SHOW=12; TS=1; BUILD=1; BLOBS=
while [ $# -gt 0 ]; do case "$1" in
  --ticks) TICKS=$2; shift 2;; --keep) KEEP=$2; shift 2;; --every) EVERY=$2; shift 2;; --open) OPEN=$2; shift 2;; --after) AFTER=$2; shift 2;;
  --out) OUT=$2; shift 2;; --show) SHOW=$2; shift 2;; --blobs) BLOBS=1; shift;; --no-ts) TS=; shift;; --no-build) BUILD=; shift;; --asks) ASKS=1; shift;; --wall) WALL=1; shift;; --dress) DRESSW=all; shift;; --dress-rows) DRESSW=$2; shift 2;; *) echo "run.sh: no option $1"; exit 64;; esac; done
[ -n "$OUT" ] || OUT="${TMPDIR:-/tmp}/blitz-$(echo "$M" | tr ':/' '--')"
mkdir -p "$OUT" || exit 3
echo "run.sh: $M; blobs in $OUT"
if [ -n "$TS" ]; then
  SIDE=mission.ts; case "$M" in two-*) SIDE=seats.ts;; esac
  rm -f "$OUT"/[hacnL]*.bin "$OUT"/s[0-9]*.bin "$OUT"/ns*.bin "$OUT"/*.txt "$OUT"/*.tsv "$OUT"/*.json
  bun run --preload ./tools/blitz/expose.ts tools/blitz/$SIDE "$M" --out "$OUT" ${TICKS:+--ticks $TICKS} ${ASKS:+--asks} ${WALL:+--wall} ${DRESSW:+--dress $DRESSW} --keep "$KEEP" --every "$EVERY" --open "$OPEN" 2>&1 | tail -${ASKS:+4}4 | cut -c1-400
  [ -s "$OUT/ticks.txt" ] || { echo "run.sh: the TypeScript side did not finish"; exit 3; }
fi
N=$(cat "$OUT/ticks.txt"); [ -n "$TICKS" ] && [ "$TICKS" -lt "$N" ] && N=$TICKS
if [ -n "$BUILD" ]; then
  ( cd rust/game_run && cargo build --bins > "$OUT/build.log" 2>&1 ) || { grep -E '^error' -A 14 "$OUT/build.log" | cut -c1-300 | head -80; echo "run.sh: the runner does not build ($OUT/build.log)"; exit 3; }
fi
"${BLITZ_RUNNER:-rust/game_run/target/debug/mission}" "$OUT" "$N" --keep "$KEEP" --every "$EVERY" --after "$AFTER" > "$OUT/rust.log" 2>&1; R=$?
grep -v '^runner:' "$OUT/rust.log" | cut -c1-330 | { if [ "$R" = 0 ]; then grep -v '^TICK [0-9]*; time' | tail -25; grep '^TICK [0-9]*; time' "$OUT/rust.log" | tail -1 | cut -c1-330; else tail -45; fi; }
# the path diff of every tick both sides kept (the first parting tick too, where the TypeScript side kept it)
U=0
for k in $(ls "$OUT" | sed -n 's/^n\([0-9]*\)\.bin$/\1/p' | sort -n); do
  [ -f "$OUT/a$k.bin" ] || continue
  printf 'tick %s: ' "$k"
  bun run tools/blitz/diff.ts "$OUT/a$k.bin" "$OUT/n$k.bin" --blocks "$OUT/s$k.bin" "$OUT/ns$k.bin" --out "$OUT/diff-$k.tsv" --canon-file tools/blitz/canon.txt --show "$SHOW" 2>&1 | cut -c1-260 | head -$((SHOW + 3)) || true
  [ "${PIPESTATUS[0]}" = 0 ] || U=1
  [ -n "$BLOBS" ] || [ "$R" != 0 ] || rm -f "$OUT/n$k.bin"
done
# THE LOOP'S INTENDED DIFFERENCES (README, "Intended differences"): a game named here parts from the TypeScript ON PURPOSE, because the Rust game is right
# by construction where the TypeScript, frozen, is not. It is named when the first kept tick with an unknown path, or the block's first parting, is the
# tick written here or later with nothing unknown before it; the exit is 4, not a bare 1.
intended() { case "$1" in
  two-full:3) echo "5499 doorpath.ts own() asks the bare Game's order book (the first seat's) for a man of seat 1 sent through a door with corners left on his order; the Rust door walk, stepped as his seat, finds his own order";;
esac; }
I=$(intended "$M"); IT=${I%% *}
if [ -n "$I" ] && [ "$R" != 2 ] && [ "$R" != 3 ]; then
  FIRST=$(for f in "$OUT"/diff-*.tsv; do [ -f "$f" ] || continue; k=${f##*diff-}; k=${k%.tsv}; grep -q . "$f" && echo "$k"; done | sort -n | head -1)
  PB=$(sed -n 's/^THE BLOCK FIRST PARTED AT TICK \([0-9]*\).*/\1/p' "$OUT/rust.log" | head -1)
  if { [ -n "$PB" ] || [ -n "$FIRST" ]; } && [ "${FIRST:-$IT}" -ge "$IT" ] && [ "${PB:-$IT}" -ge "$IT" ]; then
    echo "INTENDED DIFFERENCE, $M from tick $IT (first unknown path at a kept tick: ${FIRST:-none kept}; the block first parts: ${PB:-not in the ticks run}): ${I#* }. The Rust game is kept as it is (README)."
    exit 4
  fi
fi
[ "$R" = 1 ] && P=$(sed -n 's/^THE BLOCK FIRST PARTED AT TICK \([0-9]*\).*/\1/p' "$OUT/rust.log") && [ -n "$P" ] && { [ -f "$OUT/s$P.bin" ] && echo "where the blocks part: rust/game_run/target/debug/where_parts $OUT/s$P.bin $OUT/ns$P.bin" || echo "for the paths and the bytes at the parting tick run again with --keep \"$P\" --blobs added (and without --no-ts)"; }
[ "$R" != 0 ] && exit "$R"
exit $U
