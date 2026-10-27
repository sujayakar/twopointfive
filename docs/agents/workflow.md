# Workflow

One of the area files CLAUDE.md indexes. Everything below was moved here word for word from CLAUDE.md on 2026-10-13, in its old order;
"the bullet above / below" in a bullet means its neighbour there (each block says which lines it was), and a bullet named by its bold
title may now live in another file of this folder: `grep -l '<title>' docs/agents/*.md`. New bullets for this area go at the foot of this file.

<!-- was CLAUDE.md lines 3017–3079 -->
## Workflow the user expects

- **Fast loop:** implement → drive it briefly in the browser (boots, no console/GPU errors, feature visibly works)
  → typecheck → commit immediately with a normal human message. Don't batch features waiting on reviews.
- **Reviews never block:** right after each commit launch a code-review agent **in the background** on that commit;
  apply real findings as a small follow-up commit. Every few commits, a background QA agent in a worktree under
  `.worktrees/` (gitignored) does a broader soak. Nothing minutes-long runs in the foreground while the user is
  around. Up to ~32 parallel agents is fine.
- **Lanes = background agents in `.worktrees/<name>` on their own branch; the lead is the merge + verify gate** (unsigned `git merge --no-ff` in the main checkout, the changed
  families through `gate.sh` + tour (+ soak for gameplay) + `bun run build` on the MERGED tree in the background, a pane boot when shader text changed, commit, retire the worktree).
  Each lane serves ITS checkout on ITS OWN port (`PORT=52xx nohup bun run tools/serve.ts` there; a shot sink beside it,
  `window.__shotPort` pointing the harnesses at it (main's default 5174); `vendor/typescript` symlinked from the main tree) and stops it BY PID / PORT (`lsof -ti :52xx`) — never
  `pkill -f tools/serve.ts`: that killed every lane's server and the maintainer's :5173 at once. **:5173 is the maintainer's own `bun run dev` tab and it bundles at start-up: after
  every merge to main restart it from the main checkout (kill the pid on the port, `PORT=5173 nohup bun run tools/serve.ts`) and say so — their tab keeps running, a
  reload picks the build up; `.claude/launch.json`'s `lead` entry (:5199) is for the lead's own boots.** The Browser pane is SHARED by every lane: pin `tabId` on each call (an
  unpinned `__hotCommon` / `__stage` / `__shot` has landed on another lane's page and spoiled its take), close your tabs when done (the pane's tab cap once left a lane unable to
  boot), or drive a private headless Chrome (`--headless=new`, Metal, over CDP) from the worktree. Commit in small steps and keep the
  notebook's top self-sufficient so a resume costs minutes.
- **A lane's deliverables** (every renderer / perf / prototype lane since v3): notebook `docs/internal/<topic>.md` (10-line summary on top, method, negatives written up), numbers
  `docs/perf/<topic>-*.json`, ×3 crops `docs/shots/<prefix>_*` judged as tiles, an in-page instrument `tools/qa/<topic>-{harness,scenes,lab}.js` (`await import('/qa/<file>.js')` →
  a `__xx` hook: views via `__stage`, serialized interleaved `costAB`, `identityHash`, a `validate` sweep of presets × switches × debug views for GPU errors) and a SELF-CONTAINED
  page `docs/review/<topic>.html` from `tools/qa/<topic>-page.py` (body-only: light / dark tokens, data-URI images, tables in `overflow-x` boxes, inline SVG)
  published as an Artifact with the by-eye calls named and the taste constants as one-liners; every new switch defaults OFF (= main hash for hash) until the maintainer has looked.
- **STATUS.md is the live queue** (it WAS, through 08-16; since 2026-10-09 it is archived at docs/internal/status-archive-2026-08.md, the queue lives outside the repo, and "STATUS" anywhere in this file means that archive — the rest of this bullet is how the queue was kept)**:** strike items with the commit hash as they land, insert new asks at the right
  priority the moment they arrive (notes come as terse playtest lines + screenshots), refresh the state block, and
  when handing back after a long autonomous stretch put a short "morning report" on top (what to try first, what
  was verified and how, caveats, what was deliberately not done) — and, since they read a page rather than STATUS,
  the same report as `docs/internal/morning-YYYY-MM-DD.html` published as its own Artifact (favicon ☀️, a new file
  and URL per day, linked from STATUS, refreshed as late merges land). When the queue drains: refill toward the stated
  goal, then keep raising quality rather than stopping.
- **Build board:** `docs/roadmap.json` → `bun run tools/build-roadmap.ts` → republish the Artifact at the URL in
  STATUS.md (same URL every time; favicon 🔦; the board was last brought up to date 08-16). Keep the log in plain first-person prose.
- **Commits:** unsigned is fine (`git -c commit.gpgsign=false commit …`). Human-style messages only: no attribution lines, no tool trailers. Never `git stash`. Never `cd` into `.worktrees/*` (use `git -C`).
- **Their taste, learned the hard way:** physically-motivated effects that show off the renderer win (light through
  an opening door, blackout beacons, shootable fixtures whose light really goes, lasers in haze); grade your own
  renders harshly (see above) — "looks fine downscaled" has been wrong repeatedly.

## Where the repo lives (since 2026-10-16)

- **Development** = branch `twopointsix` on `github.com/sujayakar/twopointfive` (PUBLIC). The build to play is that branch's
  head; it moves only by the merge of a finished branch, fast-forward only, never forced. Work on your own branch off it. `main`
  there is the older twopointfive project and is not touched.
- **Everything here is public.** Nothing from outside the game goes into a file or a commit message: no private links or ids, no
  machine paths, no names, no model names, no attribution lines.
- **Not in this repo:** `handoff.md`, the story and design notes (`docs/design`), the August work log and its board
  (`docs/internal/status-archive-2026-08.md`, `docs/roadmap.json`, `tools/build-roadmap.ts`), the morning reports and the pending
  inventory of that month (`docs/internal/morning-*.html`, `docs/internal/pending-inventory-2026-08-11.md`) and
  `tools/publish-public.sh`, and the one-file page generators (`tools/artifact-page.py`, `tools/page-artifact.ts`,
  `tools/sounds-artifact.ts`). They stayed in the old private repo with the history up to 2026-10-16; a pointer to one of them in
  a comment or a bullet is a pointer there.
- **Playable build, one file** = `bun run build:one` (`tools/build-one.ts` → `dist/one/twopointsix.html`, about 12.5 MB at 10-16): the game page
  and everything it asks for at run time in ONE self-contained HTML — `index.html` as it is with its module tag replaced by the module
  itself (no chunks) and THE PACK, one inert `<script data-pack="<path>">` block of base64 per file (gzipped but for the pictures): the
  animation library with its `.bin`, both casts, the mission pictures and their list, every level file under `levels/`, and `qa/commit`
  (left out of a build of uncommitted changes). The game is not told how it is packed: `src/host/onefile.ts` (a classic script the build
  puts in front of the module; nothing imports it) answers `fetch` and `<img>.src` of a packed path from its block, and takes over a
  page load of the page's OWN address with another query that the page starts (a mission picked, Deploy, a restart, a showcase link —
  the Navigation API): the query goes into the history entry's state on the address the page was first served from (a new entry where
  the load would have made one, so Back still works; Back / Forward between the page's own entries reload as the entry), the page
  reloads, and the script writes the query back with `history.replaceState` before the game reads `location.search` — so the page plays
  where its address cannot carry flags (a frame another page owns) and where address + query is not served. NOT taken over: the
  browser's own reload button and a link opened in a new tab (they ask for address + query, fine wherever that is served). The Team &
  kit picture's second instance of the page (`ui/kitView.ts`' iframe) gets its query in the frame's `name`; opened from DISK the
  page and that picture do post to each other (measured in Chromium without a WebGPU adapter: `tools/qa/kitview-disk.cjs`; the picture
  itself up on a page from disk is NOT SEEN — the software adapter had not booted the second instance from disk in 150 s; never on a
  GPU). The page carries
  `<meta name="tps-one-page">`, as before: no tool pages in Extras, no replay viewer (F7 hands its file over as a download), and
  `ui/menuStills.ts` asks for pictures there only when the page packs them. OUT of the file: the workbench, the replay viewer, the
  sounds and music pages, `public/assets/replays`. CHECK: `bun run build:one && node tools/qa/onefile-page.cjs` (a server that knows
  one address and 404s the same address with a query; `ONEFILE_BREAK=1` = the boot script taken out, must FAIL) — the pack (the `.bin`
  by digest), the Play page, the picture frame's flags, Team & kit → Deploy into the mission's flags, a reload, Back, Forward, two
  loads at once, back to the title, nothing else asked of the server. No adapter there. On the software adapter (10-16, once, by
  hand): the file booted to `__ready` on the title and into a level-file mission with no page error and no request but its one
  address; its picture has never been seen on a GPU. A NEW FILE THE GAME PAGE FETCHES at run time has to be added to the pack in
  `tools/build-one.ts`, or the one-file page asks the network for it. How the file is published as a page is not in this repo.

## The hosted build and its backend (since 2026-10-26)

The maintainer, 10-26: "we should just deploy this on vercel and have a convex backend where we accumulate stuff like replays, telemetry,
preferences, etc." No accounts; nothing multiplayer.

- **The hosted build** = `vercel.json` + `tools/host/build.sh`: the static `bun run build` (`dist/`), `cleanUrls` so `/workbench`, `/replay`,
  `/sounds`, `/music` open as on the dev server, the dev server's three forwards (`/sneaklab`, `/firelab`, `/gait`) as redirects, and
  `dist/qa/commit` (the commit, from the host's `VERCEL_GIT_COMMIT_SHA` or a clean checkout's `git rev-parse`: a recording made on the build
  names it, as on the dev server). The project on the host is the maintainer's; its production branch is `twopointsix`, so every step that
  joins the build redeploys the link. NOT in the repo: the project, its address, any key.
- **The backend** = `cloud/` — a Convex project (`cloud/convex/`: `schema.ts`, `prefs.ts`, `tapes.ts`, `telemetry.ts`, `limits.ts`) with a
  `package.json` OF ITS OWN (`convex`; `convex-test` and `typescript` for the checks). The game still has no npm dependency: nothing under
  `src/` or `tools/` imports a package, and the page talks to the backend over plain `fetch` (`src/cloud/wire.ts`: the service's HTTP API,
  `POST <address>/api/query|mutation`, `convex_encoded_json`). With `CONVEX_DEPLOY_KEY` in the host's environment `tools/host/build.sh`
  deploys `cloud/` first (`convex deploy`) and builds the page with that deployment's address in it (`TPS_CLOUD_URL` → `tools/build.ts`
  `define` `__TPS_CLOUD__`); with no key it is the plain static build. `cloud/convex/_generated/` is checked in, as the service asks; it was
  written from the package's own templates (no deployment could be reached to run `convex codegen`), and the first `convex dev` or
  `convex deploy` writes it again.
- **Tables**: `players` (one row a browser: first and last seen), `prefs` (one row a browser: its settings as one JSON text, counted),
  `visits` (one row a played page visit: the kept tape's own row and how many pieces are there), `tapeChunks` (the tape: a visit's head,
  seq -1, and its pieces, each the gzip member the browser packed, cut at 512 KB), `telemetry` (small records). WHAT A PAGE MAY CALL is
  public (`prefs:hello|get|put`, `tapes:put|have`, `telemetry:put`) — a page has no secret — and every argument is bounded
  (`limits.ts`); READING BACK OUT is internal (`tapes:list|chunks`, `telemetry:list`, `prefs:players`): a deploy key's, by
  `bun run tools/cloud/pull.ts` (→ a hand-over file `tools/qa/replay-dump.ts` and `/replay` read, `telemetry.ndjson`, `players.json`),
  `npx convex run`, or the dashboard. A visit is its browser's: another id cannot write into it or ask what is kept of it.
- **The page's side** = `src/cloud/` (no game code imported; `main.ts` boots it once after the replay key and calls `cloud.frame(rawDt, live, stage, gpuMs)`: numbers, no object a frame):
  `cloud.ts` (the address, who the browser is, whether it sends, the boot, `window.__cloud`), `tapes.ts` (`TapeSync`: a SYNC of the kept
  tapes' store — it only reads `rows · head · bodies` of `src/replay/keepstore.ts`, newest visit first, a round every 20 s; a visit whose
  page closed is finished by the next page that opens), `telemetry.ts` (`visit` · `run` · `perf` once a minute of LIVE play · `error` ·
  `end`; nothing else, no key pressed), `prefs.ts` (the browser's `twopointsix.*` localStorage keys as a map, bar the machine's own — the
  quality tier, text size — saves, the editor's and this family's keys). `src/ui/cloudrows.ts` = two rows on Options ▸ Game, on a build
  with a backend only.
- **A build with no address has none of it**: the dev server, a checkout, `build:one`, every headless tool — `cloudBoot` returns null, no id
  is made, no timer, no request, `__cloud` is null. THE BUILD AS IT WAS (`cloud-check` `off` and `identity`; `cloud-page` step 3).
- **Who the browser is**: a random 40-character id it makes once (localStorage `twopointsix.cloud.pid`). `?player=<id>` (the link
  `__cloud.link()` / Options ▸ Game ▸ Your settings link copies) hands it to another browser, which takes the id and the settings kept under
  it BEFORE the game reads a key (`cloudAdopt`, first thing in `main()`), then loads once more without the word. The id is the only key to
  those settings: the link is to be handed over like one. The browser is ASKED first, always (the browser's own dialog, `ADOPT_ASK`:
  nothing of the page is up yet): yes replaces its settings and turns its sending on (its settings could not follow otherwise), no drops
  the word and boots the page as it was.
- **Whether it sends** (`CLOUD.shareDefault`, ON since 10-26, the maintainer's pick of three: "On, with a notice" — everyone who opens the
  link sends replays, stats and settings, a line on the title page says so and Options has the off switch): the browser's own choice where
  it made one (Options ▸ Game ▸ Share replays and stats; localStorage `twopointsix.cloud.share`), else the default. THE NOTICE COMES FIRST:
  a browser that sends by the default ALONE (it made no choice, its address bar says nothing) makes not one request until the notice has
  been on its screen once (`twopointsix.cloud.told`). `src/ui/cloudnote.ts` draws the line on the title card (`CLOUD_NOTE`; a press opens
  Options ▸ Game; `src/ui/menu.ts` `title()` calls it, one line) and says so through `__cloud.told()` once the line has been SEEN: on the
  live title, inside the window, the topmost thing at its own middle (so not under the starting page, Options or the error card), for two
  seconds in a row (`CLOUD_NOTE_MS`). A page opened straight into a mission shows no title, so it waits (`__cloud.waits`); the visit in
  which the notice is first shown sends its tape whole, and a visit played BEFORE the notice was ever on this browser's screen stays in the
  browser (the list of visits that may go is cut to this one at that moment). A browser that shares its id asks the backend nothing at its
  boot before it was told or chose (`cloudAdopt`), and copying the settings link from Options counts as told (the switch is on screen
  beside it). KNOWN LIMIT, on the quiet side: a player who never sees the title with nothing over it (the starting page opened off the boot
  card, then straight into missions, every time) is never told and never sends. The way back for everyone:
  `CLOUD.shareDefault = false` in `src/cloud/cloud.ts`; for one browser the Options row; for one visit `?cloud=0`. One visit: `?cloud=0` nothing, `?cloud=1` sends; one kind: `?cloudtapes=0` `?cloudstats=0` `?cloudprefs=0`. The page's
  own row stands over the address word, and the switch is read live (another tab that turns it off stops this one: the `storage` event).
  WHICH TAPES GO: only a visit played START TO END while the browser sent its tapes — `twopointsix.cloud.visits` is the list of visits that
  MAY go (a list of yes, not of no: a visit under `?cloud=0`, a page playing a replay, a visit the switch was off in for a moment is on no
  list, so no later visit sends it; turning the switch on sends from the next page load on). Turning the switch OFF empties the whole
  list, so a visit played earlier and not sent yet does not go up when it is turned on again; a round that is running stops at its next
  call (`TapeSync` reads the switch before every call); and another tab's off stands over what this page's own row said earlier. Records wait for nothing either: with the
  switch off telemetry counts no frame and drops what was queued. A loopback address on the address bar
  (`?cloud=http://127.0.0.1:3210`) points a visit at a backend on this machine — the page check's way in; no other address is taken from
  the address bar.
- **Size**: TWO NUMBERS, a factor of ten apart. Headless (the check's own tape): about 4.7 KB a second packed, 196 KB for 42 s of a
  four-man room clear, so about 17 MB an hour. On a REAL PAGE a squad mission's kept tape was measured at about 3 MB a minute packed
  (`docs/agents/replays.md`), so about 180 MB an hour a browser in `tapeChunks`, a third more on the wire (base64). Plan the backend's
  quota by the second one. Not measured on the service.
- **What a round costs the page** (`TapeSync`, every 20 s; raised by the kept replays' owner 10-26, who has chased single stutter frames):
  it reads only the pieces numbered past what went (`KeptStore.piecesFrom(sid, from)`, a read-only range read added to
  `src/replay/keepstore.ts` for this), sends a packed piece as the bytes the store holds — it is NOT unpacked — and does not read a visit
  whose row has not moved since a round sent all of it. A call is at most 512 KB (`TAPESYNC.call`), turned to base64 by the engine's own
  `Uint8Array.toBase64` or `btoa`. The note of how far a visit has gone is THE NEXT PIECE NUMBER (the store's key, and the backend's
  `have`), not a count of bodies: a store with a hole (a piece it refused) and a visit begun again under its name go up right
  (`cloud-check` `tapes`). NEVER MEASURED on a real page beside a fight: about a megabyte a round of a squad visit still goes through
  base64 and JSON on the page's own thread.
- **GPU stats** (the maintainer, 10-26: "can we also collect some basic stats / profiling information on GPUs?" · "but want to make sure
  we don't slow things down too much"). In the records, under the same switch (`?cloudstats=0`), from numbers the page ALREADY HAS:
  `visit` carries `machine` (since the first step, `src/ui/bench.ts` `machineFacts`: the adapter's words, the chip's name, the display's
  rate, threads, the screen — the full list is below), `gpu` (timestamp queries or not, the canvas format, the device's features and four
  of its limits) and `bootMs` (the page's open to the game being up);
  `perf` (one a minute of live play) carries the picture's size (`res`, `out`) and `gpu` = the stretch's GPU TIME: readings, mean, 50th /
  95th / 99th / worst ms, and each pass's mean ms (`passes`, biggest first). The GPU time is the compute passes' union the quality ladder
  is fed every frame (`main.ts`, the same call: the render passes are left out, as there, because a render pass's timestamps are not to be
  trusted on Metal); the per-pass means are read off the engine's timer (`gpu/timer.ts` `lastFrame`) one reading in 30. A WebGPU error
  the page did not catch and a lost device (not the page's own leaving) are `error` records. WHAT IT COSTS A FRAME: no query, readback or
  pass is added; a frame is a few adds into two small tables and makes no object (the game's facts are asked only when a record is
  written: once a minute, and at a run's end — `cloud-check` `telemetry` counts the asks), and a browser that does not send returns at
  the first test. That is by reading the code and by the count of asks under bun: NEVER MEASURED on a page, never seen on a GPU (the
  software adapter here has no timestamp queries, so no `perf.gpu` has ever been written by a real page).
- **The boot and the freezes in the records** (10-26, asked by the owner of the boot timings: shader compiles of tens of seconds on
  Windows at every load are the thing to see from players' machines). A `boot` record, once a visit: what `src/ui/boot.ts` already takes
  (`window.__boot`) — `interactive`, `pipelinesMs` (the ms the boot waited on shader pipelines), `pipelines` (total · done · failed), the
  three `slowest` by label, `shaderText`, `firstFrameCpu` / `firstFrameGpu`, `maxRafGap`, `maxLongTask` — and `nav` (the browser's own
  word for the load: `navigate` · `reload` · `back_forward`). It is asked for where records go up anyway, never in a frame, and written
  once the first frame's GPU work is back, a minute after the page was interactive, or as the page goes. The OS is in the `visit`
  record's `machine`. WHAT TO READ: Windows visits with `pipelinesMs` in the tens of seconds on reloads too. The per-minute `perf` record
  gained `stalls` / `stall` (frames past a second WITH THE TAB UP and the longest in ms: a freeze, which the frame times leave out; a gap
  after the tab was out of sight is not one) and `sharp` / `shed` beside `preset` · `scale` · `auto`, so a minute's GPU time can be held
  against the picture it was taken at. The GPU time is the passes' timestamp union, not the frame time, and absent with no timestamp
  queries. NOT READ: a compile queue (`__pipeQueue`) that is not in the build; if it comes in, `slowest` counts from a pipeline's turn.
- **EVERYTHING A VISIT'S RECORD HOLDS** (said in full because sending is on by default; keyed to the browser's random id): `machine` =
  `webgpu` (the adapter's vendor, architecture, description), `webgl` (WebGL's unmasked renderer string, the chip by name, to 160
  characters), `userAgent` (THE FULL STRING), `browser` and its major version, `os`, `threads`, `memoryGb` (as the browser rounds it),
  `screen`, `view` (the window), `dpr`, `hz`, `fallback`; `gpu` (above); `bootMs`; `query` (the page's query string without `player`);
  `lang` (the browser's language). The adapter and chip are the GPU stats that were asked for. The full user agent, screen and window
  size, threads, memory, language and query string go further, and TOGETHER ARE CLOSE TO A FINGERPRINT of the machine. The title's notice
  says "replays, stats and settings"; it does not list these. Trimming them was the maintainer's choice, put to him 10-26 before any
  backend existed, of three: drop the full user agent and the language, keep everything, or the GPU and OS only. HIS PICK, 10-27: KEEP
  EVERYTHING. Nothing is trimmed. Should that change, the lines to cut are one each: `cloud.ts` `hello` (`query`, `lang`) and the
  `machine` closure in `main.ts`.
- **Not sent**: the notes written on a kept visit (the viewer's and the list's; the dump reader would take a `{k:'notes'}` line), and a
  visit removed with the list's ✕ stays on the backend.
- **CHECKS** (`npm ci` in `cloud/` first): `bun run tools/qa/cloud-check.ts` (under bun, both stands: `off` · `backend` · `tapes` ·
  `identity` · `telemetry` · `prefs`; `CLOUD_BREAK=1` in front must FAIL `tapes` and `off`: every third piece is dropped, and a browser that
  sends by the default does not wait for the notice) and `PORT=<dev server> node tools/qa/cloud-page.cjs`
  (headless Chromium, software adapter, three browser profiles; its step 4 sets the address a hosted build carries on the dev server's page and
  holds the notice rule: no request before the title's line, the line, then it sends). BOTH RUN AGAINST A STAND-IN (`cloud/test/standin.ts`: the real functions and
  schema in the service's test library behind the same two HTTP routes). `cd cloud && npx tsc --noEmit -p convex/tsconfig.json` typechecks
  the functions.
- **Known limits**: the page has no secret, so anyone who reads it can call what a page calls. Every call is bounded; the TOTAL is not (no
  rate limit, no cap a browser or a visit). No accounts: an id is as private as its browser's storage and the links made from it.
- **NEVER RUN, NEVER SEEN (10-26)**: a real Convex deployment (the service's own limits, its CORS answer to a page on another origin, the
  deploy from the host's build), the host's build itself (that its image has `bun`: the script installs it where it has not), the hosted
  build itself in a browser (the page check sets its address on the dev server's page), anything of it on a GPU, the GPU's time in a
  record from a real page, the notice's line, the two Options rows and the link's question by eye, two tabs of one browser (the live switch).
