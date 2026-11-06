#!/bin/bash
# THE MISSION LOOP's one command (tools/blitz/README.md):
#   tools/blitz/run.sh <mission>:<seed> [--ticks N] [--keep "1,2,100-110"] [--every N] [--open "559"] [--after N] [--out DIR] [--show N] [--blobs] [--no-ts] [--no-build]
# 1. the TypeScript side once (mission.ts): the host's half every tick, a hash of each part of rust/sim's block and the die's count after every tick, whole
#    states only at the kept ticks;   2. the native runner (rust/game_run) builds and steps the Rust game from tick 1, comparing after EVERY tick and stopping
#    at the first tick that parts;   3. the path diff of every tick both sides kept, minus known.txt.
# Exit: 0 in step and no unknown path · 1 the block or the die parted, or a kept tick has an unknown path · 2 the Rust tick panicked · 3 a side could not run.
# Needs bun and cargo on the PATH, and nothing outside the repository but the folder it names on its first line (--out, or one under $TMPDIR).
# A kept tick's Rust state (n<k>.bin) is deleted once it is diffed, unless --blobs.
# --no-ts takes the TypeScript side as --out already holds it (the same mission, ticks and kept ticks); --no-build takes the runner as it is built.
cd "$(dirname "$0")/../.." || exit 64
unset BUN_OPTIONS
M=$1; shift
case "$M" in ''|--*) sed -n '2,3p' "$0"; exit 64;; esac
TICKS=; KEEP=; EVERY=0; OPEN=; AFTER=0; OUT=; SHOW=12; TS=1; BUILD=1; BLOBS=
while [ $# -gt 0 ]; do case "$1" in
  --ticks) TICKS=$2; shift 2;; --keep) KEEP=$2; shift 2;; --every) EVERY=$2; shift 2;; --open) OPEN=$2; shift 2;; --after) AFTER=$2; shift 2;;
  --out) OUT=$2; shift 2;; --show) SHOW=$2; shift 2;; --blobs) BLOBS=1; shift;; --no-ts) TS=; shift;; --no-build) BUILD=; shift;; *) echo "run.sh: no option $1"; exit 64;; esac; done
[ -n "$OUT" ] || OUT="${TMPDIR:-/tmp}/blitz-$(echo "$M" | tr ':/' '--')"
mkdir -p "$OUT" || exit 3
echo "run.sh: $M; blobs in $OUT"
if [ -n "$TS" ]; then
  rm -f "$OUT"/[hacn]*.bin "$OUT"/s[0-9]*.bin "$OUT"/ns*.bin "$OUT"/*.txt "$OUT"/*.tsv
  bun run --preload ./tools/blitz/expose.ts tools/blitz/mission.ts "$M" --out "$OUT" ${TICKS:+--ticks $TICKS} --keep "$KEEP" --every "$EVERY" --open "$OPEN" 2>&1 | tail -4 | cut -c1-400
  [ -s "$OUT/ticks.txt" ] || { echo "run.sh: the TypeScript side did not finish"; exit 3; }
fi
N=$(cat "$OUT/ticks.txt"); [ -n "$TICKS" ] && [ "$TICKS" -lt "$N" ] && N=$TICKS
if [ -n "$BUILD" ]; then
  ( cd rust/game_run && cargo build --bins > "$OUT/build.log" 2>&1 ) || { grep -E '^error' -A 14 "$OUT/build.log" | cut -c1-300 | head -80; echo "run.sh: the runner does not build ($OUT/build.log)"; exit 3; }
fi
rust/game_run/target/debug/mission "$OUT" "$N" --keep "$KEEP" --every "$EVERY" --after "$AFTER" > "$OUT/rust.log" 2>&1; R=$?
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
[ "$R" = 1 ] && P=$(sed -n 's/^THE BLOCK FIRST PARTED AT TICK \([0-9]*\).*/\1/p' "$OUT/rust.log") && [ -n "$P" ] && { [ -f "$OUT/s$P.bin" ] && echo "where the blocks part: rust/game_run/target/debug/where_parts $OUT/s$P.bin $OUT/ns$P.bin" || echo "for the paths and the bytes at the parting tick run again with --keep \"$P\" --blobs added (and without --no-ts)"; }
[ "$R" != 0 ] && exit "$R"
exit $U
