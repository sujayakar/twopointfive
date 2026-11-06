#!/bin/bash
# A HEADLESS TOOL ON THE RUST GAME (tools/blitz/PLAY.md, "The harness's word"):
#   tools/blitz/rust.sh [rust|audit|ts|table] bun run tools/qa/playthrough.ts --scn room-clear-four --seeds 1 --json out.json
# It sets TPS_GAME (default rust: the game `standUp` returns is a puppet of the Rust game; `audit`: the same, audited; `ts`: the same stand, the TypeScript
# tick; `table`: the Rust game but for the checks tools/blitz/qa-kinds.json keeps on the TypeScript tick) and the bridge's preload for the tool and every child it starts, and runs the command as it is.
cd "$(dirname "$0")/../.." || exit 64
W=rust; case "$1" in rust|audit|ts|table) W=$1; shift;; esac
[ $# -gt 0 ] || { sed -n '2,5p' "$0"; exit 64; }
export TPS_GAME=$W BUN_OPTIONS="--preload=./tools/blitz/expose.ts"
exec "$@"
