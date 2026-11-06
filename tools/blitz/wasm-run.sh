#!/bin/bash
# THE MISSION LOOP THROUGH WASM, one command (tools/blitz/FACE.md):
#   tools/blitz/wasm-run.sh <mission>:<seed> | <DIR of a run> [--ticks N] [--out DIR] [--release | --small] [--no-build] [-- options of wasm-run.ts]
# 1. the TypeScript side once (mission.ts, as run.sh runs it), unless a folder a run already filled is named (run.sh's --out: the mission is not played twice);
# 2. the face (rust/game_face) built to wasm: the dev profile, or --release (opt-level 3) or --small (opt-level "s");
# 3. wasm-run.ts: the module stepped through every tick with the host's half, the die's count and each part's hash held to the TypeScript's after every tick.
# Exit: wasm-run.ts's (0 in step · 1 parted · 2 the Rust tick panicked · 3 a side could not run).
cd "$(dirname "$0")/../.." || exit 64
unset BUN_OPTIONS
M=$1; shift
case "$M" in ''|--*) sed -n '2,3p' "$0"; exit 64;; esac
TICKS=; OUT=; PROFILE=debug; FLAG=; BUILD=1
while [ $# -gt 0 ]; do case "$1" in
  --ticks) TICKS=$2; shift 2;; --out) OUT=$2; shift 2;; --release) PROFILE=release; FLAG=--release; shift;; --small) PROFILE=small; FLAG="--profile small"; shift;;
  --no-build) BUILD=; shift;; --) shift; break;; *) echo "wasm-run.sh: no option $1"; exit 64;; esac; done
if [ -f "$M/c1.bin" ]; then OUT=$M; else
  [ -n "$OUT" ] || OUT="${TMPDIR:-/tmp}/blitz-$(echo "$M" | tr ':/' '--')"
  mkdir -p "$OUT" || exit 3
  rm -f "$OUT"/[hacn]*.bin "$OUT"/s[0-9]*.bin "$OUT"/ns*.bin "$OUT"/*.txt "$OUT"/*.tsv
  bun run --preload ./tools/blitz/expose.ts tools/blitz/mission.ts "$M" --out "$OUT" ${TICKS:+--ticks $TICKS} --keep "" --every 0 --open "" 2>&1 | tail -4 | cut -c1-400
  [ -s "$OUT/ticks.txt" ] || { echo "wasm-run.sh: the TypeScript side did not finish"; exit 3; }
fi
echo "wasm-run.sh: $M; the run in $OUT"
if [ -n "$BUILD" ]; then
  ( cd rust/game_face && cargo build --target wasm32-unknown-unknown $FLAG > "$OUT/face-build.log" 2>&1 ) || { grep -E '^error' -A 14 "$OUT/face-build.log" | cut -c1-300 | head -80; echo "wasm-run.sh: the face does not build ($OUT/face-build.log)"; exit 3; }
fi
exec bun run tools/blitz/wasm-run.ts "$OUT" $TICKS --wasm "rust/game_face/target/wasm32-unknown-unknown/$PROFILE/game_face.wasm" "$@"
