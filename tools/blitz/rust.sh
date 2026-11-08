#!/bin/bash
# A HEADLESS TOOL ON THE RUST GAME (tools/blitz/PLAY.md, "The harness's word"):
#   tools/blitz/rust.sh [rust|audit|none] bun run tools/qa/playthrough.ts --scn room-clear-four --seeds 1 --json out.json
# It sets TPS_GAME (default rust; tools/blitz/word.ts says the words) and runs the command as it is, from the repository's root: the bridge's preload is
# bunfig.toml's there, for the tool and every child `bun` it starts in the root. The word alone in front of a command run from the root is the same thing.
cd "$(dirname "$0")/../.." || exit 64
W=rust; case "$1" in rust|audit|none) W=$1; shift;; esac
[ $# -gt 0 ] || { sed -n '2,5p' "$0"; exit 64; }
export TPS_GAME=$W
exec "$@"
