# The hosted build's backend

A Convex project: what a hosted build of the game keeps on a server. The game itself has no npm dependency; the `package.json` here is this
folder's own, and nothing under `src/` or `tools/` imports from it. `docs/agents/workflow.md` ("The hosted build and its backend") says
what is kept, who sends, and how it is checked.

```
convex/schema.ts      the tables: players · prefs · visits · tapeChunks · telemetry · files · notes · games · runners
convex/prefs.ts       a browser's settings            (public: hello · get · put      internal: players)
convex/tapes.ts       the kept tapes                  (public: put · have             internal: list · chunks)
convex/telemetry.ts   small records of a visit        (public: put                    internal: list)
convex/games.ts       co-op's games                   (public: make · join · get      internal: heard · reap · list …)
convex/runners.ts     where game servers run, the seam (internal: register · hosted · unregister · list · start · sweep)
convex/modal.ts       the hosted runner: a Modal sandbox a game (a Node action; the one file that loads a package at run time)
convex/http.ts        the heartbeat's door            (POST /mp/heartbeat, at the deployment's site address)
convex/crons.ts       the reaper, twice a minute
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
cd cloud && npm ci      # once, and again when cloud/package.json has changed
bun run dev             # from the repo's root: the dev server, and a Convex backend on this machine beside it
```

`bun run dev` starts `convex dev` here (no account needed the first time; it downloads Convex's local backend once) and tells the
page its address, so a local build keeps replays, notes, stats, settings and saved files the way the hosted one does, in
`cloud/.convex/`. It never uses a deploy key from your shell. If you chose a deployment here yourself (`npx convex dev` writes
`.env.local`), that one is used. `TPS_DEV_CLOUD=http://127.0.0.1:3210 bun run tools/serve.ts` points the page at a backend that is
already up; a hosted address there sends local play to that backend, behind the page's notice. (`TPS_CLOUD_URL` is the hosted
build's own address and the dev server does not read it.) Read it back with `--local` in place of the address and key below.

## Co-op's game servers

The backend is co-op's control plane: it keeps the games, starts one game server a game, hands a player its address and a seat key, holds a
cap on how many run, and ends the ones that are idle, silent or old (`convex/games.ts`). Where a server runs is behind a seam of four calls
(`convex/runners.ts`: start · stop · list · ready), with two runners behind it:

- **On your own machine** the runner is the dev server. `bun run dev` registers it with the local backend, and a game's server is a child
  process (`tools/mp/server.ts`) at `ws://localhost:<port>`. Nothing hosted is touched. `bun run tools/mp/runner-try.ts` makes and joins a
  game from a terminal and prints its state as it changes. `MP_SERVER=tools/mp/runner-standin.ts bun run dev` starts a stand-in that keeps the
  heartbeat and the seats and plays nothing; `MP_CAP` is how many this machine carries (2).
- **Hosted**, a server is a Modal sandbox (`convex/modal.ts`). Set `MODAL_TOKEN_ID` and `MODAL_TOKEN_SECRET` in the deployment's environment
  (the dashboard, or `npx convex env set`). The hosted build's script then tells the backend the page's commit at every deploy
  (`runners:hosted`), and the backend builds that commit's image and runs up to `MP_CAP` (4) servers of it. A deployment with no token runs none.

Server and page are the same build by construction: a runner says which builds its servers can be (the dev server: the commit it runs
from; hosted: the commit the page was built from, and the two deploys before it), and a game is only made on a runner of the page's build.
A page older than that is told so and has to be loaded again.

The cap is a cap on servers: a game that is over holds its place until a sweep of its runner has seen its server gone.

```bash
npx convex run runners:list          # the runners, their build, how full each is
npx convex run games:list            # the games, newest first, with what a runner said of a failed start (no key, no secret)
```

The clocks, in seconds, each a word of the deployment's environment: `MP_START_S` (120: a server asked for must be heard from), `MP_SILENT_S`
(45: not heard from = gone), `MP_IDLE_S` (60: nobody connected), `MP_MAX_S` (3600: a game's whole life).

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
bun run ../tools/qa/mp-control-check.ts        # co-op's control plane: the cap, the reaper, the seam, against the stand-in and the local runner
```
