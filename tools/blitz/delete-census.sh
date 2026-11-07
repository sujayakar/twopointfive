#!/bin/bash
# THE CENSUS FOR THE DELETION (tools/blitz/DELETION.md): what runs as TypeScript while the Rust game steps. It makes a copy of this tree with a counter at the
# top of every function of the simulation's folders (delete-tick.ts --instrument; nothing deleted), plays the Rust game on the copy, and writes what ran
# to tools/blitz/delete-census.json, which the plan reads. The copy is a git worktree beside the checkout; the checkout itself is not touched.
#   tools/blitz/delete-census.sh [--seeds 1] [--jobs 2] [--port 5270] [--no-page] [--keep]
# Needs: bun, the TypeScript compiler (vendor/typescript or TS_LIB), a built rust/game_face (tools/blitz/face-build.ts), and for the page plays a Chromium
# (CHROME=…, default /usr/bin/chromium). Hours of plays on a small machine: the missions' seed 1 are the long part.
set -euo pipefail
cd "$(dirname "$0")/../.."; ROOT=$PWD
SEEDS=1; JOBS=2; PORT=5270; PAGE=1; KEEP=0
while [ $# -gt 0 ]; do case "$1" in --seeds) SEEDS=$2; shift 2;; --jobs) JOBS=$2; shift 2;; --port) PORT=$2; shift 2;; --no-page) PAGE=0; shift;; --keep) KEEP=1; shift;; *) echo "unknown word $1"; exit 64;; esac; done
T=${CENSUS_TREE:-$ROOT/../.delete-census-tree}; OUT=${CENSUS_OUT:-$(mktemp -d)}; RAN=$OUT/ran.jsonl; : > "$RAN"
WASM=rust/game_face/target/wasm32-unknown-unknown/release/game_face.wasm
[ -f "$WASM" ] || { echo "no $WASM: bun run tools/blitz/face-build.ts first"; exit 3; }
[ -d "$T" ] && git worktree remove --force "$T"
# (the copy is the checkout's files as they stand, tracked changes too: the plan must be of the tree that will be deleted from)
git worktree add -q --detach "$T" HEAD; git diff HEAD --binary | (cd "$T" && git apply --allow-empty)
mkdir -p "$T/vendor" "$T/$(dirname $WASM)"; [ -e vendor/typescript ] && ln -sfn "$ROOT/vendor/typescript" "$T/vendor/typescript"; cp "$WASM" "$T/$WASM"
DELETE_ROOT=$T bun run tools/blitz/delete-tick.ts --instrument --no-census > "$OUT/instrument.log" 2>&1 || { tail -5 "$OUT/instrument.log"; exit 3; }
cd "$T"
echo "the census's tree: $T · its lines: $OUT"
# the missions, seed by seed, with the whole-run digest (the same plays the deletion is held to afterwards)
TPS_GAME=rust PT_DIGEST=1 TPS_RAN_OUT=$RAN bun run tools/qa/playthrough.ts --seeds "$SEEDS" --jobs "$JOBS" --json "$OUT/missions.json" > "$OUT/missions.log" 2>&1 || true
grep -cE '^\s+[a-z0-9-]+:[0-9]+ ' "$OUT/missions.log" | sed 's/^/missions played: /'
# the same missions' starts on Hard (a start is TypeScript: what Hard alone stands must be seen), played 45 s each
TPS_GAME=rust TPS_RAN_OUT=$RAN bun run tools/qa/playthrough.ts --seeds 1 --jobs "$JOBS" --cap 45 --set '{"DIFFICULTY":{"level":"hard"}}' --json "$OUT/hard.json" > "$OUT/hard.log" 2>&1 || true
# the tutorials, the quiet missions' lone player, and a family of checks that call rules
for f in tools/qa/lessons/tut-*.ts; do t=$(basename "$f" .ts); TPS_GAME=rust TPS_RAN_OUT=$RAN bun run tools/qa/lesson-run.ts "$t" > "$OUT/lesson-$t.log" 2>&1 || true; done
TPS_GAME=rust TPS_RAN_OUT=$RAN bun run tools/qa/solo-night.ts --quiet --plan sneak --ghost --seeds 1 --jobs 1 > "$OUT/quiet.log" 2>&1 || true
TPS_GAME=rust TPS_RAN_OUT=$RAN bun run tools/qa/solo-night.ts --plan walk --seeds 1 --jobs 1 > "$OUT/night.log" 2>&1 || true
TPS_GAME=rust TPS_RAN_OUT=$RAN bun run tools/qa/probes.ts qrf > "$OUT/qrf.log" 2>&1 || true
# the page: the solo night, a squad mission, the title to play, and the other pages
if [ $PAGE = 1 ]; then touch "$WASM"; PORT=$PORT bun run tools/serve.ts > "$OUT/serve.log" 2>&1 & SERVE=$!; trap 'kill $SERVE 2>/dev/null || true' EXIT
  for i in $(seq 1 60); do curl -s -o /dev/null "http://localhost:$PORT/" && break; sleep 1; done
  bun run "$ROOT/tools/blitz/delete-page.ts" --at "http://localhost:$PORT" --census "$RAN" > "$OUT/page.log" 2>&1 || true; tail -12 "$OUT/page.log"
  kill $SERVE 2>/dev/null || true; trap - EXIT; fi
cd "$ROOT"
bun run tools/blitz/delete-tick.ts --census-merge "$T/tools/blitz/delete-census-ids.json" "$RAN"
[ $KEEP = 1 ] || git worktree remove --force "$T"
