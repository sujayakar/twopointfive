#!/bin/bash
# KEEP A RUN OF THE LOOP as a file the Rust game can be held to later with no TypeScript side (tools/blitz/hold.sh):
#   tools/blitz/keep.sh <the run's --out folder> <file.tar.zst> <the command that made the run, in words>
# Keep only a run that was IN STEP (run.sh exit 0): post.txt is then the Rust game's own series as much as the TypeScript's.
O=$1; F=$2; shift 2; [ -s "$O/post.txt" ] && [ -s "$O/c1.bin" ] || { echo "keep.sh: $O is no run of the loop"; exit 64; }
mkdir -p "$(dirname "$F")"
{ echo "command: $*"; echo "tree: $(git rev-parse --short HEAD 2>/dev/null)"; echo "made: $(date -u +%Y-%m-%dT%H:%MZ)"; echo "result: $(grep -a '^DONE\|PARTED\|^PANIC' "$O/rust.log" 2>/dev/null | head -1 | cut -c1-200)"; } > "$O/command.txt"
( cd "$O" && ls | grep -E '^(c1|s0|h[0-9]+|L[0-9]+|Ls[0-9]+|knobs)\.bin$|^(post|host|between|gives|hostwrites|meter|charges|ticks|case|call|die|command)\.txt$' | tar --zstd -cf - -T - ) > "$F"
