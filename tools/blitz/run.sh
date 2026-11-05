#!/bin/sh
# THE TICK LOOP's one command (tools/blitz/README.md):   tools/blitz/run.sh <mission>:<seed> [--ticks N] [--from K] [--keep] [--verbose] [--show N] [--out DIR]
# Needs bun and cargo on the PATH, and nothing outside the repository but a temp folder it names.
cd "$(dirname "$0")/../.." || exit 64
unset BUN_OPTIONS
exec bun run tools/blitz/run.ts "$@"
