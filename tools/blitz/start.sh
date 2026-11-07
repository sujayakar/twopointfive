#!/bin/bash
# THE START COMPARE (rust/game_start/README.md, "The compare"): one level file stood by the TypeScript start and by the Rust start, and the two states held
# to each other path by path.
#   tools/blitz/start.sh <level file> [--seed N] [--scenario roomclear|nightbreach|rescue|quiet [--squad N] [--deal N]] [--cpu] [--frames N] [--out DIR] [--show N] [--no-ts] [--no-build]
# 1. the TypeScript start (start.ts): the level on the page's stand through the one stand-up, exported by the bridge (c1.bin, s0.bin);
# 2. the Rust start (rust/game_start, bin `start`): the same file through `game_start::stand`, exported by the bridge's Rust writer (n1.bin, ns0.bin);
# 3. the path diff (diff.ts) of the two, minus known.txt and known-start.txt (what a start may differ in, each with its reason);
# 4. with --frames N: both starts stepped IN RUST by the same host frame with an idle hand, the die and every part of the block compared after every
#    frame (the Rust start's bin, `--beside`), and the path diff of the two states as the run ends.
# --cpu: the senses worked out by the game itself on both sides (TPS_CPUSENSE=1 TPS_MOONLIGHT=1; the Rust start's `cpu_senses`), as the tests stand.
# Exit: 0 no unknown path · 1 a path that is not known · 2 the Rust start panicked · 3 a side could not run.
cd "$(dirname "$0")/../.." || exit 64
unset BUN_OPTIONS
L=$1; shift
case "$L" in ''|--*) sed -n '2,4p' "$0"; exit 64;; esac
SEED=20942; OUT=; SHOW=40; TS=1; BUILD=1; SCN=; SQUAD=; DEAL=; CPU=; FRAMES=
while [ $# -gt 0 ]; do case "$1" in
  --seed) SEED=$2; shift 2;; --scenario) SCN=$2; shift 2;; --squad) SQUAD=$2; shift 2;; --deal) DEAL=$2; shift 2;; --cpu) CPU=1; shift;; --frames) FRAMES=$2; shift 2;; --out) OUT=$2; shift 2;; --show) SHOW=$2; shift 2;; --no-ts) TS=; shift;; --no-build) BUILD=; shift;;
  *) echo "start.sh: no option $1"; exit 64;; esac; done
MORE="${SCN:+--scenario $SCN} ${SQUAD:+--squad $SQUAD} ${DEAL:+--deal $DEAL}"
[ -n "$OUT" ] || OUT="${TMPDIR:-/tmp}/start-$(basename "$L" .json)${SCN:+-$SCN}"
mkdir -p "$OUT" || exit 3
echo "start.sh: $L, die seed $SEED${SCN:+, scenario $SCN}; blobs in $OUT"
if [ -n "$TS" ]; then
  rm -f "$OUT"/*.bin "$OUT"/*.tsv
  ${CPU:+env TPS_CPUSENSE=1 TPS_MOONLIGHT=1} bun run --preload ./tools/blitz/expose.ts tools/blitz/start.ts "$L" --out "$OUT" --seed "$SEED" $MORE 2>&1 | tail -4 | cut -c1-330
  [ -s "$OUT/c1.bin" ] || { echo "start.sh: the TypeScript start did not finish"; exit 3; }
fi
if [ -n "$BUILD" ]; then
  ( cd rust/game_start && cargo build --bin start > "$OUT/build.log" 2>&1 ) || { grep -E '^error' -A 14 "$OUT/build.log" | cut -c1-300 | head -60; echo "start.sh: the Rust start does not build ($OUT/build.log)"; exit 3; }
fi
rm -f "$OUT"/n1.bin "$OUT"/ns0.bin "$OUT"/end-*.bin
rust/game_start/target/debug/start "$L" --out "$OUT" --seed "$SEED" $MORE ${CPU:+--cpu} > "$OUT/rust.log" 2>&1 || { tail -30 "$OUT/rust.log" | cut -c1-330; echo "start.sh: the Rust start panicked ($OUT/rust.log)"; exit 2; }
tail -1 "$OUT/rust.log" | cut -c1-330
bun run tools/blitz/diff.ts "$OUT/c1.bin" "$OUT/n1.bin" --blocks "$OUT/s0.bin" "$OUT/ns0.bin" --out "$OUT/diff.tsv" --canon-file tools/blitz/canon.txt --known tools/blitz/known.txt,tools/blitz/known-start.txt --show "$SHOW" 2>&1 | cut -c1-260 | head -$((SHOW + 3))
R=${PIPESTATUS[0]}
if [ -n "$FRAMES" ] && [ "$FRAMES" -gt 0 ]; then
  rm -f "$OUT"/end-*.bin
  rust/game_start/target/debug/start "$L" --out "$OUT" --seed "$SEED" $MORE ${CPU:+--cpu} --frames "$FRAMES" --beside > "$OUT/beside.log" 2>&1; B=$?
  grep '^BESIDE' "$OUT/beside.log" | cut -c1-330
  [ "$B" = 0 ] || [ "$B" = 1 ] || { tail -20 "$OUT/beside.log" | cut -c1-330; echo "start.sh: a game panicked in the frames ($OUT/beside.log)"; exit 2; }
  printf 'as the frames end, the Rust start against the TypeScript start, both stepped in Rust: '
  bun run tools/blitz/diff.ts "$OUT/end-b.bin" "$OUT/end-n.bin" --blocks "$OUT/end-bs.bin" "$OUT/end-ns.bin" --out "$OUT/diff-end.tsv" --canon-file tools/blitz/canon.txt --known tools/blitz/known.txt,tools/blitz/known-start.txt --show "$SHOW" 2>&1 | cut -c1-260 | head -$((SHOW + 3))
  [ "${PIPESTATUS[0]}" = 0 ] && [ "$B" = 0 ] || R=1
fi
exit "$R"
