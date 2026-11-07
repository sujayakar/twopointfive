#!/bin/bash
# HOLD THE RUST GAME TO A KEPT RUN, with no TypeScript side:
#   tools/blitz/hold.sh <run.tar.zst | a folder of them> [--no-build]
# A kept run (tools/blitz/keep.sh) is everything the native runner needs to step the Rust game through one play again: the start (c1.bin, s0.bin), the
# host's half of every tick (h<k>.bin), the driver's ordered record (between.txt, gives.txt, hostwrites.txt, meter.txt), a checkpoint put back
# (L<k>.bin), and post.txt: a line a tick, `k die_n die_s time <a hash of each of the block's 13 parts>`, taken while the Rust game was in step with
# the TypeScript game on that run. The runner steps the game and compares its own die and block to that line after EVERY tick.
# HOLD_DIRS=<folder>: each run's folder is left there as it was held (its start made again where it was), under the run's name: for a run of the
# native runner by hand (`--keep` writes the Rust game's whole states, n<k>.bin), or to keep it anew for this tree's wire (keep.sh).
# Exit: 0 every run the same series · 1 a run parted (its name and first tick are printed) · 2 a run panicked · 3 could not run, or a run's start could not be made again.
# What a kept run cannot see: state of the game outside the block and the die (the path diff needed the TypeScript's whole states), and a change of
# the wire UNSEEN (c1.bin is read by the import as generated when the run was kept: command.txt names the tree). So:
# A KEPT RUN OF ANOTHER LAYOUT OF THE WIRE (layout.txt is not `layout` of schema.json, or is missing: a run kept before the stamp) HAS ITS START MADE AGAIN
# from this tree, and its line says so. A kept run is three things. The start's bytes are bound to the wire. The records and orders (the txt files) are
# not. The series (post.txt) is not. So the start is stood again by the same command's words (mission.ts or seats.ts --start-only under TPS_GAME=none: the TypeScript
# start alone, no tick of either game), the kept records are played on it and the series is held to the kept one: equal = the play did not change with
# nothing but the start made again; parted at tick 1 = the start itself moved. The host's half of a tick (h<k>.bin) is bound to the wire too: where the
# new tick-1 half is the kept one with bytes added at its end, those bytes are added to every kept half (STAND-IN: they are tick 1's, taken as the same
# all run: today the engine's four words of what the host draws, which a headless host never changes); any other change of its shape is refused, and so
# is a run that puts a checkpoint back (L<k>.bin is a whole state of the older layout). A game of two seats is stood by seats.ts the same way.
# ONE OTHER CHANGE OF THE HALF IS TAKEN: a run kept by a host that stepped by records alone, before every host handed the game its camera and Input at
# the stand, is one camera row and one Input row short: tools/blitz/half-grow.ts gives its halves those two rows (the fresh tick 1's: nothing moves them
# on such a host) and refuses unless tick 1 then is the fresh half byte for byte.
cd "$(dirname "$0")/../.." || exit 64
[ -n "$1" ] || { sed -n '2,3p' "$0"; exit 64; }
if [ "$2" != --no-build ] && [ -z "$BLITZ_RUNNER" ]; then ( cd rust/game_run && cargo build --bins > /dev/null 2>&1 ) || { echo "hold.sh: the runner does not build"; exit 3; }; fi
R=${BLITZ_RUNNER:-rust/game_run/target/debug/mission}; W=0; unset BUN_OPTIONS
LAY=$(grep -o '"layout":[0-9]*' tools/blitz/schema.json | cut -d: -f2)
# fresh <folder of a kept run>: its start made again from this tree; 1 with WHY said when it cannot be
fresh() { local D=$1 c e M n a=()
  c=$(sed -n 's/^command: //p' "$D/command.txt" 2>/dev/null | head -1); case "$c" in *"tools/blitz/run.sh "*) ;; *) WHY="command.txt names no run of the loop"; return 1;; esac
  ls "$D" | grep -q '^L[0-9]*\.bin$' && { WHY="it puts a checkpoint back (L<k>.bin: a whole state of the older layout)"; return 1; }
  e=${c%%tools/blitz/run.sh*}; eval "set -- ${c#*tools/blitz/run.sh }"; M=$1; shift
  local side=mission.ts; case "$M" in two-*) side=seats.ts;; esac
  while [ $# -gt 0 ]; do case "$1" in
    --hard|--wall) a+=("$1"); shift;; --dress) a+=(--dress all); shift;; --dress-rows) a+=(--dress "$2"); shift 2;;
    --case|--die|--set|--lever) a+=("$1" "$2"); shift 2;; --ticks|--keep|--every|--open|--after|--out|--show) shift 2;; *) shift;; esac; done
  mkdir "$D/fresh" || return 1
  eval "env TPS_GAME=none $e bun run --preload ./tools/blitz/expose.ts tools/blitz/$side \"\$M\" --out \"\$D/fresh\" --start-only \"\${a[@]}\"" > "$D/fresh.log" 2>&1
  [ -s "$D/fresh/c1.bin" ] && [ -s "$D/fresh/s0.bin" ] && [ -s "$D/fresh/h1.bin" ] || { WHY="the start did not stand here: $(grep -a -m1 'rror' "$D/fresh.log" | cut -c1-220)"; return 1; }
  [ -s "$D/h1.bin" ] || { WHY="it holds no host's half of tick 1"; return 1; }
  n=$(( $(stat -c %s "$D/fresh/h1.bin") - $(stat -c %s "$D/h1.bin") ))
  local rows=
  if [ "$n" -ge 0 ] && cmp -s -n "$(stat -c %s "$D/h1.bin")" "$D/h1.bin" "$D/fresh/h1.bin"; then
    if [ "$n" -gt 0 ]; then tail -c "$n" "$D/fresh/h1.bin" > "$D/fresh/tail"; for h in "$D"/h[0-9]*.bin; do cat "$D/fresh/tail" >> "$h"; done; fi
  else
    # …OR IT IS SHORT OF THE HOST'S OWN CAMERA AND INPUT ROWS, which every host hands the game at its stand now and a host that steps by records alone
    # never moves: half-grow.ts gives each kept half the two rows as the fresh tick 1 has them, and refuses unless tick 1 then IS the fresh one
    rows=$(bun run --preload ./tools/blitz/expose.ts tools/blitz/half-grow.ts "$D" "$D/fresh" 2>&1 | tail -1) && n=rows || { WHY="the host's half of a tick changed shape, not only grew at its end ($rows)"; return 1; }
  fi
  HOW="the start made again here, layout $LAY; $( [ ! -e "$D/s0.bin" ] && echo "no start was kept" || { cmp -s "$D/s0.bin" "$D/fresh/s0.bin" && echo "its block the kept one" || echo "ITS BLOCK NOT THE KEPT ONE"; } ); the host's half $( [ "$n" = rows ] && echo "given the host's own rows: $rows" || echo "$n bytes longer" )"
  cp "$D/fresh/c1.bin" "$D/fresh/s0.bin" "$D/"; [ "$(stat -c %s "$D/fresh/knobs.bin")" = "$(stat -c %s "$D/knobs.bin" 2>/dev/null)" ] || cp "$D/fresh/knobs.bin" "$D/"
  rm -rf "$D/fresh"; }
for f in $( [ -d "$1" ] && find "$1" -name '*.tar.zst' | sort || echo "$1" ); do
  D=$(mktemp -d) || exit 3; tar --zstd -xf "$f" -C "$D" || { echo "hold.sh: cannot read $f"; rm -rf "$D"; exit 3; }
  HOW=; if [ "$(cat "$D/layout.txt" 2>/dev/null)" != "$LAY" ]; then fresh "$D" || { echo "$(basename "$f" .tar.zst): NOT HELD: its start is of another layout of the wire and cannot be made again: $WHY (exit 3)"; [ 3 -gt $W ] && W=3; rm -rf "$D"; continue; }; fi
  "$R" "$D" "$(cat "$D/ticks.txt")" > "$D/rust.log" 2>&1; r=$?
  echo "$(basename "$f" .tar.zst): $( [ $r = 0 ] && echo "the same over $(cat "$D/ticks.txt") ticks" || grep -a 'FIRST PARTED\|^PANIC' "$D/rust.log" | head -1 | cut -c1-160 ) (exit $r; $(sed -n 1p "$D/command.txt" 2>/dev/null)${HOW:+; $HOW})$(grep -a -m1 '^VERDICT' "$D/rust.log" | sed 's/^/ · /')"
  [ $r -gt $W ] && W=$r; if [ -n "$HOLD_DIRS" ]; then mkdir -p "$HOLD_DIRS" && rm -rf "$HOLD_DIRS/$(basename "$f" .tar.zst)" && mv "$D" "$HOLD_DIRS/$(basename "$f" .tar.zst)"; else rm -rf "$D"; fi
done
exit $W
