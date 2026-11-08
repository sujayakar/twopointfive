#!/bin/bash
# THE START COMPARE (rust/game_start/README.md, "The compare"): one level file stood by the TypeScript start and by the Rust start, and the two states held
# to each other path by path.
#   tools/blitz/start.sh <level file> [--seed N] [--seats 2] [--scenario roomclear|nightbreach|rescue|quiet [--squad N] [--deal N] | --link '?words'] [--cpu] [--frames N] [--out DIR] [--show N] [--no-ts] [--no-build]
#   tools/blitz/start.sh --mission <id> [--seed N] [--seats 2] [--frames N] …      a row of the mission list: the file the page loads for it, started from the link a click on the row loads
#   --words '&hard' with --mission or --link: those words on the end of its link, on both sides, in a folder of its own
# 1. the TypeScript start (start.ts): the level on the page's stand through the one stand-up, exported by the bridge (c1.bin, s0.bin);
# 2. the Rust start (rust/game_start, bin `start`): the same file through `game_start::stand`, exported by the bridge's Rust writer (n1.bin, ns0.bin);
# 3. the path diff (diff.ts) of the two, minus known.txt and known-start.txt (what a start may differ in, each with its reason);
# 4. with --frames N: both starts stepped IN RUST by the same host frame with an idle hand, the die and every part of the block compared after every
#    frame (the Rust start's bin, `--beside`), and the path diff of the two states as the run ends.
# --seats 2: a game of two players (seats.ts), the second with nobody at his controls: his record is the still one a host hands a seat it has none for.
# The diffs are BY REACH FROM THE LIVE GAME (diff.ts --live): the TypeScript keeps some tables weakly, by the object (the night's opening's `seen` and
# `born`, keyed by a guard), and lists a man a mission took off the floor for as long as its collector leaves him; the Rust keeps his row by its ID. Compared
# by key, a start was equal or not by when the collector ran (seen: one row of the table in a dozen runs, with three starts running at once). What that
# rule hides is at the head of diff.ts.
# --cpu: the senses worked out by the game itself on both sides (TPS_CPUSENSE=1 TPS_MOONLIGHT=1; the Rust start's `cpu_senses`), as the tests stand.
# Exit: 0 no unknown path · 1 a path that is not known · 2 the Rust start panicked · 3 a side could not run.
cd "$(dirname "$0")/../.." || exit 64
unset BUN_OPTIONS
L=; case "$1" in --mission) ;; ''|--*) sed -n '2,5p' "$0"; exit 64;; *) L=$1; shift;; esac
SEED=20942; OUT=; SHOW=40; TS=1; BUILD=1; SCN=; SQUAD=; DEAL=; CPU=; FRAMES=; MISSION=; SEATS=; LINK=; WORDS=
while [ $# -gt 0 ]; do case "$1" in
  --mission|--link|--words|--seed|--seats|--scenario|--squad|--deal|--frames|--out|--show) [ $# -ge 2 ] || { echo "start.sh: $1 needs a value"; exit 64; };; esac
  case "$1" in
  --mission) MISSION=$2; shift 2;; --link) LINK=$2; shift 2;; --words) WORDS=$2; shift 2;; --seats) SEATS=$2; shift 2;;
  --seed) SEED=$2; shift 2;; --scenario) SCN=$2; shift 2;; --squad) SQUAD=$2; shift 2;; --deal) DEAL=$2; shift 2;; --cpu) CPU=1; shift;; --frames) FRAMES=$2; shift 2;; --out) OUT=$2; shift 2;; --show) SHOW=$2; shift 2;; --no-ts) TS=; shift;; --no-build) BUILD=; shift;;
  *) echo "start.sh: no option $1"; exit 64;; esac; done
# (a folder a start: the file, the kind, and a link by its sum, so one start's state is never read as another's)
[ -n "$OUT" ] || OUT="${TMPDIR:-/tmp}/start-${MISSION:-$(basename "$L" .json)${SCN:+-$SCN}${LINK:+-link$(printf %s "$LINK" | cksum | cut -d' ' -f1)}}${WORDS:+-words$(printf %s "$WORDS" | cksum | cut -d' ' -f1)}${SEATS:+-seats$SEATS}"
mkdir -p "$OUT" || exit 3
OUT=$(cd "$OUT" && pwd) || exit 3   # (said whole: the build below runs from another folder)
echo "start.sh: ${MISSION:+mission $MISSION}$L, die seed $SEED${SCN:+, scenario $SCN}${LINK:+, from $LINK}${WORDS:+, with the words $WORDS}${SEATS:+, $SEATS seats}; blobs in $OUT"
if [ -n "$TS" ]; then
  rm -f "${OUT:?}"/*.bin "${OUT:?}"/*.tsv "${OUT:?}"/mission.txt
  if [ -n "$MISSION" ]; then ${CPU:+env TPS_CPUSENSE=1 TPS_MOONLIGHT=1} bun run --preload ./tools/blitz/expose.ts tools/blitz/start.ts --mission "$MISSION" --out "$OUT" --seed "$SEED" ${SEATS:+--seats $SEATS} ${WORDS:+--words "$WORDS"} 2>&1 | tail -4 | cut -c1-330
  else ${CPU:+env TPS_CPUSENSE=1 TPS_MOONLIGHT=1} bun run --preload ./tools/blitz/expose.ts tools/blitz/start.ts "$L" --out "$OUT" --seed "$SEED" ${SEATS:+--seats $SEATS} ${SCN:+--scenario $SCN} ${SQUAD:+--squad $SQUAD} ${DEAL:+--deal $DEAL} ${LINK:+--link "$LINK"} ${WORDS:+--words "$WORDS"} 2>&1 | tail -4 | cut -c1-330; fi
  [ -s "$OUT/c1.bin" ] || { echo "start.sh: the TypeScript start did not finish"; exit 3; }
fi
# (a mission's file and the link a click on its row loads, as the TypeScript side read them off the list)
if [ -n "$MISSION" ]; then
  [ -s "$OUT/mission.txt" ] || { echo "start.sh: no $OUT/mission.txt (the TypeScript side names a mission's file and link)"; exit 3; }
  L=$(sed -n 1p "$OUT/mission.txt"); LINK=$(sed -n 2p "$OUT/mission.txt")
  # (a folder written before a mission was started from its link holds the mission's kind on that line)
  case "$LINK" in \?*) ;; *) echo "start.sh: $OUT/mission.txt is not of this start.ts (its second line is no link: '$LINK'): run it again without --no-ts"; exit 3;; esac
fi
# (a link handed here takes the words as the TypeScript side's did; a mission's has them from its file)
[ -z "$MISSION" ] && LINK="$LINK$WORDS"
MORE="${SCN:+--scenario $SCN} ${SQUAD:+--squad $SQUAD} ${DEAL:+--deal $DEAL}"
if [ -n "$BUILD" ]; then
  ( cd rust/game_start && cargo build --bin start > "$OUT/build.log" 2>&1 ) || { grep -E '^error' -A 14 "$OUT/build.log" | cut -c1-300 | head -60; echo "start.sh: the Rust start does not build ($OUT/build.log)"; exit 3; }
fi
rm -f "${OUT:?}"/n1.bin "${OUT:?}"/ns0.bin "${OUT:?}"/end-*.bin
rust/game_start/target/debug/start "$L" --out "$OUT" --seed "$SEED" ${SEATS:+--seats $SEATS} $MORE ${LINK:+--link "$LINK"} ${CPU:+--cpu} > "$OUT/rust.log" 2>&1 || { tail -30 "$OUT/rust.log" | cut -c1-330; echo "start.sh: the Rust start panicked ($OUT/rust.log)"; exit 2; }
tail -1 "$OUT/rust.log" | cut -c1-330
bun run tools/blitz/diff.ts "$OUT/c1.bin" "$OUT/n1.bin" --blocks "$OUT/s0.bin" "$OUT/ns0.bin" --out "$OUT/diff.tsv" --canon-file tools/blitz/canon.txt --known tools/blitz/known.txt,tools/blitz/known-start.txt --live --show "$SHOW" 2>&1 | cut -c1-260 | head -$((SHOW + 3))
R=${PIPESTATUS[0]}
if [ -n "$FRAMES" ] && [ "$FRAMES" -gt 0 ]; then
  rm -f "${OUT:?}"/end-*.bin
  rust/game_start/target/debug/start "$L" --out "$OUT" --seed "$SEED" ${SEATS:+--seats $SEATS} $MORE ${LINK:+--link "$LINK"} ${CPU:+--cpu} --frames "$FRAMES" --beside > "$OUT/beside.log" 2>&1; B=$?
  grep '^BESIDE' "$OUT/beside.log" | cut -c1-330
  [ "$B" = 0 ] || [ "$B" = 1 ] || { tail -20 "$OUT/beside.log" | cut -c1-330; echo "start.sh: a game panicked in the frames ($OUT/beside.log)"; exit 2; }
  printf 'as the frames end, the Rust start against the TypeScript start, both stepped in Rust: '
  bun run tools/blitz/diff.ts "$OUT/end-b.bin" "$OUT/end-n.bin" --blocks "$OUT/end-bs.bin" "$OUT/end-ns.bin" --out "$OUT/diff-end.tsv" --canon-file tools/blitz/canon.txt --known tools/blitz/known.txt,tools/blitz/known-start.txt --live --show "$SHOW" 2>&1 | cut -c1-260 | head -$((SHOW + 3))
  [ "${PIPESTATUS[0]}" = 0 ] && [ "$B" = 0 ] || R=1
fi
exit "$R"
