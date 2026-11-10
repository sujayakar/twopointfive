#!/bin/sh
# An older build and this one, in one box, one after the other and the older again: sh lab/tools/two.sh <older commit, in full> <views.json> <card.cjs flags…>
OLD=$1; V=$(cd "$(dirname "$2")" && pwd)/$(basename "$2"); shift 2; OUT=$(mkdir -p "${OUT:-out}" && cd "${OUT:-out}" && pwd); D=$(mktemp -d)
git cat-file -e "$OLD^{commit}" 2>/dev/null || git fetch -q --depth 1 origin "$OLD" || exit 1
git worktree add -q --detach "$D/old" "$OLD" || exit 1; trap 'git worktree remove --force "$D/old"' EXIT
for k in old new again; do d=.; [ $k = new ] || d=$D/old; echo "== $k"; (cd $d && OUT="$OUT/$k" node lab/tools/card.cjs "$V" "$@"); done
