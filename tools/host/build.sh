#!/bin/sh
# THE HOSTED BUILD (vercel.json runs this as the project's build; any static host that can run a script can). Two ways, by the environment:
#   no CONVEX_DEPLOY_KEY   the plain static build: `bun run build` → dist/. The page talks to nobody.
#   CONVEX_DEPLOY_KEY set  the backend in cloud/ is deployed first (`convex deploy`, which reads the key from the environment), and the page is built with
#                          that deployment's address in it (TPS_CLOUD_URL → tools/build.ts): replays, records and settings can then be kept there
#                          (src/cloud/cloud.ts says who sends what). A build fails if the backend does not deploy: the page is never ahead of its backend.
# The key is the host's secret. It is never written into dist/, a log line or the repo.
set -e
command -v bun >/dev/null 2>&1 || npm install -g bun
if [ -n "$CONVEX_DEPLOY_KEY" ]; then
  cd cloud
  npm ci --no-audit --no-fund
  npx convex deploy --cmd-url-env-var-name TPS_CLOUD_URL --cmd 'cd .. && bun run build'
else
  bun run build
fi
