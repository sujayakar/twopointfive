#!/bin/sh
# WHO WRITES THIS, ON THE RUST SIDE: a hardware watchpoint on one double of the Rust game, under gdb, with the stack of each write.
#   tools/blitz/watch-rust.sh <run dir> <ticks> <what>:<row>:<index> [writes=1]
# e.g.  tools/blitz/watch-rust.sh /tmp/blitz-abc 1 ch.pos:4:0        (the x of character 4's position, through tick 1)
# <what> is an arm of `watch_addr` in rust/game_run/src/main.rs (time, ch.pos, ch.vel, box.c, box.h): add one for the field you need. The run dir is one run.sh made;
# more than one tick needs it made with --keep. Needs gdb. The address is taken once, after the import: a row of a table that grows may move.
cd "$(dirname "$0")/../.." || exit 64
[ $# -ge 3 ] || { sed -n '2,6p' "$0"; exit 64; }
BIN=rust/game_run/target/debug/game_run
[ -x "$BIN" ] || cargo build --manifest-path rust/game_run/Cargo.toml || exit 3
set -- "$1" "$2" "$3" "${4:-1}"
more=""; i=1; while [ "$i" -lt "$4" ]; do more="$more -ex continue -ex bt\ 14"; i=$((i + 1)); done
eval GAME_RUN_WATCH="$3" gdb -batch -ex "'break game_run_watch_ready'" -ex run -ex "'watch -l *(double*)(*(unsigned long*)&GAME_RUN_WATCH_ADDR)'" -ex continue -ex "'bt 14'" $more --args "$BIN" "'$1'" "'$2'" 2>&1 \
  | grep -E '^#|Old value|New value|^NOTE|^PANIC|^TICK' | grep -v 'std::\|core::\|__rust\|game_run::main' | sed 's/ (.*//' | cut -c1-200
