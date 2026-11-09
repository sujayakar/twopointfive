#!/bin/sh
# An older build and this one, in one box, one after the other and the older again: sh lab/tools/two.sh <older commit> <views.json> <card.cjs flags…>
OLD=$1; V=$(cd "$(dirname "$2")" && pwd)/$(basename "$2"); shift 2; OUT=$(mkdir -p "${OUT:-out}" && cd "${OUT:-out}" && pwd); D=$(mktemp -d)
git fetch -q --depth 1 origin "$OLD" && git worktree add -q --detach "$D/old" FETCH_HEAD || exit 1
for k in old new again; do d=.; [ $k = new ] || d=$D/old; echo "== $k"; (cd $d && OUT="$OUT/$k" node lab/tools/card.cjs "$V" "$@"); done
