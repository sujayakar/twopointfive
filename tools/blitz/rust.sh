#!/bin/bash
# A HEADLESS TOOL ON THE RUST GAME (tools/blitz/PLAY.md, "The harness's word"):
#   tools/blitz/rust.sh [rust|audit|ts|table|none] bun run tools/qa/playthrough.ts --scn room-clear-four --seeds 1 --json out.json
# It sets TPS_GAME (default rust: the game `standUp` returns is a puppet of the Rust game; `audit`: the same, audited; `ts`: the same stand, the TypeScript
# tick; `table`: the Rust game but for the checks tools/blitz/qa-kinds.json keeps on the TypeScript tick; `none`: nothing loaded) and runs the command as it
# is, from the repository's root: the bridge's preload is bunfig.toml's there (tools/blitz/preload.ts, which does nothing unless the word asks), for the tool
# and every child `bun` it starts in the root. The word alone in front of a command run from the root is the same thing: `TPS_GAME=rust bun run …`.
cd "$(dirname "$0")/../.." || exit 64
W=rust; case "$1" in rust|audit|ts|table|none) W=$1; shift;; esac
[ $# -gt 0 ] || { sed -n '2,7p' "$0"; exit 64; }
export TPS_GAME=$W
exec "$@"
