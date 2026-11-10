#!/bin/sh
# an older build's kept light and this one's, in one box: sh lab/tools/kept/two.sh <older commit, in full> <its plan> <this one's plan> [<its plan> <this one's plan> …]
OLD=$1; shift; OUT=$(mkdir -p "${OUT:-out}" && cd "${OUT:-out}" && pwd); D=$(mktemp -d); HERE=$(pwd)
git cat-file -e "$OLD^{commit}" 2>/dev/null || git fetch -q --depth 1 origin "$OLD" || exit 1
git worktree add -q --detach "$D/old" "$OLD" || exit 1; trap 'git worktree remove --force "$D/old"' EXIT; k=0
while [ $# -gt 1 ]; do k=$((k+1)); a="$HERE/$1"; b="$HERE/$2"; shift 2; echo "== old $a"; (cd "$D/old" && cp "$HERE/lab/tools/kept.cjs" lab/tools/kept.cjs && OUT="$OUT/old$k" node lab/tools/kept.cjs "$a"); echo "== new $b"; OUT="$OUT/new$k" node lab/tools/kept.cjs "$b"; done
