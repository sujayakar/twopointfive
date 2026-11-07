#!/bin/bash
# THE FUZZ ON A SQUAD MISSION (tools/blitz/PLAY.md, "The fuzz"): the scripted player of tools/qa/playthrough.ts plays one mission and seed with a seeded
# stream of perturbations laid over it between ticks (tools/blitz/perturb.ts: random rows of the orders menu on random doors, floor and men, throws, the
# player's fire, kills, the hostages' word, the mains, saves and loads, the pause card's restart), twice: on the TypeScript tick under the fold (the
# reference) and on the Rust game; a line a tick on each side and the whole state every half minute; then fuzz-diff.ts (the die and the block, every
# tick) and fuzz-dumps.ts (every pair of whole states, by path, canonically).
#   tools/blitz/fuzz-squad.sh <mission> <seed> [--hard] [--rate K] [--god player|squad|all] [--alarm TICK] [--skip a,b] [--only a,b] [--out DIR] [--side ts|rust]
# --alarm TICK: the response team called at that tick (QA_PERTURB_ALARM): two casts of the door tools on one floor.
# --god: the player's god mode held on, the team's men unhittable, or both (QA_PERTURB_GOD).
# FUZZ_CAP=<seconds> (900): the longest a side may run; a side cut short is "one side stopped" and the ticks both played are still compared (the Rust side
# of a long mission on a loaded box needs more: PLAY.md, "What a tick costs").
# The folder keeps `cmd.txt` (this command), each side's tick lines and log, `dumps.txt` (a line a half minute: paths not known) and `rust-hashes.txt`:
# what tools/blitz/fuzz-replay.sh plays the Rust game again against.
# EQUAL means all three: the die, the block, and 0 paths not known at every half minute.
# Exit: 0 equal · 1 the die or the block part, or one side stopped · 2 equal every tick, a whole state has a path not known · 3 a side did not run ·
# 4 equal by all three but for the panes after a load (the reference's own: PLAY.md).
cd "$(dirname "$0")/../.." || exit 64
ARGS="$*"
M=$1; SEED=$2; shift 2 || { sed -n '2,16p' "$0"; exit 64; }
HARD=; OUT=; SKIP=; ONLY=; RATE=1; ALARM=; GOD=; SIDES="ts rust"
while [ $# -gt 0 ]; do case "$1" in --hard) HARD=1; shift;; --out) OUT=$2; shift 2;; --skip) SKIP=$2; shift 2;; --only) ONLY=$2; shift 2;; --rate) RATE=$2; shift 2;; --alarm) ALARM=$2; shift 2;; --god) GOD=$2; shift 2;; --side) SIDES=$2; shift 2;; *) echo "fuzz-squad.sh: no option $1"; exit 64;; esac; done
[ -n "$OUT" ] || OUT="${TMPDIR:-/tmp}/fuzz-$M-$SEED${HARD:+-hard}${ALARM:+-alarm}"
mkdir -p "$OUT" || exit 3
[ "$SIDES" = "ts rust" ] && echo "FUZZ_CAP=${FUZZ_CAP:-900} tools/blitz/fuzz-squad.sh $ARGS" | sed 's/ --out [^ ]*//' > "$OUT/cmd.txt"
DUMPS=$(seq -s, 1800 1800 36000)
for W in $SIDES; do
  rm -f "$OUT"/$W-*.txt "$OUT"/$W-t*.bin "$OUT/$W.log" "$OUT/$W.out"
  env TPS_GAME=$W QA_FOLD=1 QA_PERTURB=$SEED QA_PERTURB_RATE=$RATE QA_PERTURB_ALARM=$ALARM QA_PERTURB_GOD=$GOD QA_PERTURB_SKIP="$SKIP" QA_PERTURB_ONLY="$ONLY" RH_TICKS="$OUT" RH_DUMP=$DUMPS TPS_GAME_SAY=1 \
    timeout -k 5 ${FUZZ_CAP:-900} bun run tools/qa/playthrough.ts --scn "$M" --seeds "$SEED" --jobs 1 --json "$OUT/$W.json" ${HARD:+--set '{"DIFFICULTY":{"level":"hard"}}'} > "$OUT/$W.out" 2>&1
  echo "exit $?" >> "$OUT/$W.out"
  # (the reader takes each side's log as one file: the perturbations' lines, then what the tool itself printed)
  cat "$OUT/$W.out" >> "$OUT/$W.log"
done
if [ "$SIDES" != "ts rust" ]; then bun run tools/blitz/fuzz-dumps.ts "$OUT" > /dev/null; exit 0; fi
bun run tools/blitz/fuzz-diff.ts "$OUT" | tee "$OUT/ticks.txt"; CODE=${PIPESTATUS[0]}
# THE PANES AFTER A LOAD ARE THE REFERENCE'S OWN: the TypeScript's put-back of a save leaves a pane a round broke in the abandoned future broken, the Rust
# game's kept clone has the saved moment's. A run that parts in the block's `panes` alone, after a load, is held again with the panes left out.
PANES=; if [ $CODE -eq 1 ]; then bun run tools/blitz/fuzz-diff.ts "$OUT" --but-panes > "$OUT/ticks-but-panes.txt"; [ $? -eq 0 ] && grep -qE 'STIM load: .*put back|the checkpoint loaded' "$OUT/ts.log" && { PANES=1; CODE=0; echo "… and EQUAL on every tick with the panes left out (a pane after a load: the reference's own)"; }; fi
# THE WHOLE STATES EVERY HALF MINUTE, a path a line (tools/blitz/fuzz-dumps.ts): what the tick lines cannot see. The die and the block are the bodies and
# the dice; what a man was told, a talk's record, a module's own books are not in them, and a call that ran in TypeScript on the puppet and never reached
# the Rust game leaves both equal for minutes. Held up to the tick the die or the block parted (the panes apart): past it they are two games.
UPTO=$(bun run tools/blitz/fuzz-diff.ts "$OUT" --json --but-panes | sed -n 's/.*"parted":\([0-9]*\).*/\1/p'); [ "${UPTO:-0}" -gt 0 ] || UPTO=99999999
bun run tools/blitz/fuzz-dumps.ts "$OUT" --upto "$UPTO"; D=$?
[ $CODE -eq 0 ] && [ $D -eq 1 ] && CODE=2
[ $CODE -eq 0 ] && [ -n "$PANES" ] && CODE=4
exit $CODE
