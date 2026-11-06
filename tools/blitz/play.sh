#!/bin/bash
# THE RUST GAME PLAYED BY THE SCRIPTED PLAYER, one command (tools/blitz/PLAY.md):
#   tools/blitz/play.sh <mission>:<seed> [--ticks N] [--out DIR] [--no-build] [--no-ts] [-- options of play.ts]
# 1. the same mission and seed played by the plain TypeScript game (mission.ts, as run.sh runs it): the die's count and the block's part hashes after every
#    tick, and how it ended. --no-ts keeps the one --out already holds;
# 2. the face (rust/game_face) built to wasm, release;
# 3. play.ts: the Rust game stepped, its state applied onto the TypeScript game as a puppet, the scripted player reading the puppet; every tick held to 1;
# 4. where they part: the TypeScript's state as that tick opened is taken (the mission played again to that tick) and diffed against the puppet's.
# Exit: play.ts's (0 equal to the end and ended the same · 1 parted · 2 the Rust tick panicked · 3 a side could not run · 4 equal every tick, another end).
cd "$(dirname "$0")/../.." || exit 64
unset BUN_OPTIONS
M=$1; shift
case "$M" in ''|--*) sed -n '2,3p' "$0"; exit 64;; esac
TICKS=; OUT=; BUILD=1; TS=1
while [ $# -gt 0 ]; do case "$1" in
  --ticks) TICKS=$2; shift 2;; --out) OUT=$2; shift 2;; --no-build) BUILD=; shift;; --no-ts) TS=; shift;; --) shift; break;; *) echo "play.sh: no option $1"; exit 64;; esac; done
[ -n "$OUT" ] || OUT="${TMPDIR:-/tmp}/play-$(echo "$M" | tr ':/' '--')"
mkdir -p "$OUT" || exit 3
if [ -n "$TS" ]; then
  rm -f "$OUT"/[hacn]*.bin "$OUT"/s[0-9]*.bin "$OUT"/*.txt "$OUT"/ts.log
  T0=$(date +%s.%N)
  bun run --preload ./tools/blitz/expose.ts tools/blitz/mission.ts "$M" --out "$OUT" ${TICKS:+--ticks $TICKS} --keep "" --every 0 --open "" 2>&1 | tail -4 | cut -c1-400 > "$OUT/ts.log"
  echo "wall $(echo "$(date +%s.%N) - $T0" | bc) s (the TypeScript play, with the loop's own files written every tick)" >> "$OUT/ts.log"
  cat "$OUT/ts.log"
  [ -s "$OUT/ticks.txt" ] || { echo "play.sh: the TypeScript side did not finish"; exit 3; }
fi
if [ -n "$BUILD" ]; then
  ( cd rust/game_face && cargo build --target wasm32-unknown-unknown --release > "$OUT/face-build.log" 2>&1 ) || { grep -E '^error' -A 14 "$OUT/face-build.log" | cut -c1-300 | head -80; echo "play.sh: the face does not build ($OUT/face-build.log)"; exit 3; }
fi
bun run --preload ./tools/blitz/expose.ts tools/blitz/play.ts "$M" --ref "$OUT" --out "$OUT/play" ${TICKS:+--ticks $TICKS} "$@"
CODE=$?
# (the reference's host's half of every tick was read by play.ts, which holds its own to it: megabytes that nothing reads again)
find "$OUT" -maxdepth 1 -name 'h*.bin' -delete
K=$(ls "$OUT/play" 2>/dev/null | sed -n 's/^open\([0-9]*\)\.bin$/\1/p' | sort -n | head -1)
if [ "$CODE" = 1 ] && [ -n "$K" ]; then
  echo "play.sh: parted at tick $K: the TypeScript's state as that tick opened, beside the puppet's (diff.ts; the first is the TypeScript's)"
  mkdir -p "$OUT/at" && bun run --preload ./tools/blitz/expose.ts tools/blitz/mission.ts "$M" --out "$OUT/at" --ticks "$K" --keep "" --every 0 --open "$K" > /dev/null 2>&1
  find "$OUT/at" -maxdepth 1 -name 'h*.bin' -delete
  [ -f "$OUT/at/c$K.bin" ] && bun run tools/blitz/diff.ts "$OUT/at/c$K.bin" "$OUT/play/open$K.bin" --show 40 2>&1 | cut -c1-300 | head -70
fi
exit $CODE
