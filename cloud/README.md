# The hosted build's backend

A Convex project: what a hosted build of the game keeps on a server. The game itself has no npm dependency; the `package.json` here is this
folder's own, and nothing under `src/` or `tools/` imports from it. `docs/agents/workflow.md` ("The hosted build and its backend") says
what is kept, who sends, and how it is checked.

```
convex/schema.ts      the tables: players · prefs · visits · tapeChunks · telemetry
convex/prefs.ts       a browser's settings            (public: hello · get · put      internal: players)
convex/tapes.ts       the kept tapes                  (public: put · have             internal: list · chunks)
convex/telemetry.ts   small records of a visit        (public: put                    internal: list)
convex/limits.ts      the bounds every public function holds its arguments to
test/standin.ts       a stand-in deployment for the checks (the functions in convex-test, behind the HTTP API's two routes)
```

## Putting it live

1. Make a project on the Convex dashboard and a **production deploy key** for it (Settings ▸ Deploy keys).
2. On the host (Vercel: Settings ▸ Environment Variables) set `CONVEX_DEPLOY_KEY` to that key, for the Production environment only.
3. Deploy. `tools/host/build.sh` sees the key, runs `convex deploy` here, and builds the page with the deployment's address in it.

With no key the host builds the plain static page and nothing here is used. The key is the host's secret: it belongs in no file of the repo.

By hand, from a checkout: `cd cloud && npm ci && npx convex dev` (your own dev deployment), or `CONVEX_DEPLOY_KEY=… npx convex deploy`.

## On your own machine

```bash
cd cloud && npm ci      # once
bun run dev             # from the repo's root: the dev server, and a Convex backend on this machine beside it
```

`bun run dev` starts `convex dev` here (no account needed the first time; it downloads Convex's local backend once) and tells the
page its address, so a local build keeps replays, notes, stats, settings and saved files the way the hosted one does, in
`cloud/.convex/`. It never uses a deploy key from your shell. If you chose a deployment here yourself (`npx convex dev` writes
`.env.local`), that one is used. `TPS_CLOUD_URL=http://127.0.0.1:3210 bun run tools/serve.ts` points the page at a backend that is
already up. Read it back with `--local` in place of the address and key below.

## Reading it back

```bash
CONVEX_URL=https://<name>.convex.cloud CONVEX_DEPLOY_KEY=… bun run tools/cloud/pull.ts --out cloud-pull
bun run tools/qa/replay-dump.ts cloud-pull/*.ndjson.gz --out cloud-pull/replays     # each visit as a replay file; /replay opens them too
```

`bun run tools/cloud/pull.ts --local --out cloud-pull` reads the backend `bun run dev` started. `telemetry.ndjson` and `players.json` land beside it. The dashboard shows the same tables.

## Checks

```bash
npm ci                                         # once
npx tsc --noEmit -p convex/tsconfig.json       # the functions typecheck
bun run ../tools/qa/cloud-check.ts             # the functions, the page's side and the reading tool, against the stand-in
```
