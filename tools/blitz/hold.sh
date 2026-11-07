#!/bin/bash
# HOLD THE RUST GAME TO A KEPT RUN, with no TypeScript side:
#   tools/blitz/hold.sh <run.tar.zst | a folder of them> [--no-build]
# A kept run (tools/blitz/keep.sh) is everything the native runner needs to step the Rust game through one play again: the start (c1.bin, s0.bin), the
# host's half of every tick (h<k>.bin), the driver's ordered record (between.txt, gives.txt, hostwrites.txt, meter.txt), a checkpoint put back
# (L<k>.bin), and post.txt: a line a tick, `k die_n die_s time <a hash of each of the block's 13 parts>`, taken while the Rust game was in step with
# the TypeScript game on that run. The runner steps the game and compares its own die and block to that line after EVERY tick.
# Exit: 0 every run the same series · 1 a run parted (its name and first tick are printed) · 2 a run panicked · 3 could not run.
# What a kept run cannot see: state of the game outside the block and the die (the path diff needed the TypeScript's whole states), and a change of
# the wire (c1.bin is read by the import as generated when the run was kept: command.txt names the tree).
cd "$(dirname "$0")/../.." || exit 64
[ -n "$1" ] || { sed -n '2,3p' "$0"; exit 64; }
if [ "$2" != --no-build ] && [ -z "$BLITZ_RUNNER" ]; then ( cd rust/game_run && cargo build --bins > /dev/null 2>&1 ) || { echo "hold.sh: the runner does not build"; exit 3; }; fi
R=${BLITZ_RUNNER:-rust/game_run/target/debug/mission}; W=0
for f in $( [ -d "$1" ] && find "$1" -name '*.tar.zst' | sort || echo "$1" ); do
  D=$(mktemp -d) || exit 3; tar --zstd -xf "$f" -C "$D" || { echo "hold.sh: cannot read $f"; rm -rf "$D"; exit 3; }
  "$R" "$D" "$(cat "$D/ticks.txt")" > "$D/rust.log" 2>&1; r=$?
  echo "$(basename "$f" .tar.zst): $( [ $r = 0 ] && echo "the same over $(cat "$D/ticks.txt") ticks" || grep -a 'FIRST PARTED\|^PANIC' "$D/rust.log" | head -1 | cut -c1-160 ) (exit $r; $(sed -n 1p "$D/command.txt" 2>/dev/null))"
  [ $r -gt $W ] && W=$r; rm -rf "$D"
done
exit $W
