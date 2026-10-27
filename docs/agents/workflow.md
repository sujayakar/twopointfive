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
  `<meta name="tps-one-page">`, as before: no tool pages in Extras, no replay viewer (F7 sends its file to a backend the address names, and with none says the replay was not kept), and
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
  link sends replays, stats, settings and (since the download door, 10-27) saved files, a line on the title page says so and Options has the off switch): the browser's own choice where
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
  95th / 99th / worst ms, and each pass's mean ms in two lists, biggest first: `passes` (the compute passes) and `render` (the render
  passes). The GPU time is the compute passes' union the quality ladder
  is fed every frame (`main.ts`, the same call and the same list `GPU_RENDER_PASSES`: the render passes are left out, as there, because a
  render pass's timestamps are not to be trusted on Metal — the first real records, 10-27, had `fxaa` and `post` on top of one list at
  twice the frame's time, so they are listed apart and read with that in mind); the per-pass means are read off the engine's timer
  (`gpu/timer.ts` `lastFrame`) one reading in 30. A WebGPU error
  the page did not catch and a lost device (not the page's own leaving) are `error` records. WHAT IT COSTS A FRAME: no query, readback or
  pass is added; a frame is a few adds into two small tables and makes no object (the game's facts are asked only when a record is
  written: once a minute, and at a run's end — `cloud-check` `telemetry` counts the asks), and a browser that does not send returns at
  the first test. That is by reading the code and by the count of asks under bun: the cost was NEVER MEASURED on a page (the software
  adapter here has no timestamp queries; the first `perf.gpu` a real page wrote is in the first real read, below).
- **A tab that goes out of sight** (10-27, from the first real records: eleven `end` records in four visits, several the same, and `perf`
  records of a tenth of a second). A hidden tab may come back, so `Telemetry.flush(going, facts, hidden)` writes an `end` only where
  frames were played since the last one, and leaves a stretch of play shorter than `TELEMETRY.perfMin` (5 s) open; `pagehide` says all
  it has. The LAST `end` of a visit is its total. `cloud-check` `telemetry` holds it.
- **A load's whole time and its stages** (10-27, the maintainer: "also do we have load time telemetry?"; the `boot` record had the time
  to the start card, the script's start, the device and the shader wait, and nothing else). `src/cloud/loadtime.ts` `bootRecord` now
  makes the record, from what the page ALREADY TAKES (`ui/boot.ts`'s marks and spans, the browser's own resource timing): `whole` (ms
  from the navigation's start to the later of the start card and the first frame's GPU work: THE LOAD; ABSENT where the first picture
  never came back before the record was written, so a load cut short is read to its start card and never as a fast one), `shown` (the first picture),
  `level`, `gltf` (the characters), `game`, the two tables whole as `marks` and `spans` (a mark added in `ui/boot.ts` is in the record
  with no change here; the shader groups' spans are among them), `net` (what came down, this page's own files only, numbers only: `html`,
  `script` ms and `scriptWire` / `scriptSize` bytes of the bundle, wire 0 = from the browser's cache and a few hundred = asked again and not sent, `files`, `wire`, `size`, `last`),
  `rows`, the shader rows the load was STARTED with (`shaderRowsOn()`, read only), `nth` (which load of the TAB this is, counted in
  sessionStorage at the backend's boot, so a load that wrote no record is still counted) and `mission` (the mission the page opened on, '' = the start page). A MISSION START IS A PAGE LOAD (`main.ts` does
  `location.assign` on a pick), and so is the way back to the list: the browser calls those `navigate` like a first visit, so `nth` is
  what tells a tab's first load from its tenth mission. The old flat keys stay. THE STAGES OVERLAP
  (shaders compile while the characters load and the game is made): they do not add up to `whole`; the marks in time order are the
  timeline. READ IT with `bun run tools/cloud/errors.ts --metrics`, section "the loads": a line a browser × kind of load (a tab's
  first, a later one in the tab, a reload) × shader rows, each number a median, with how many of them opened on a mission. A record from before reads "to the start card" and its rows are what the visit's ADDRESS
  asked for (`?<row>`, any value but 0; a row put on in Options is not there), said as such. That is the comparison for a change of the rows'
  defaults: the same browser, the same kind of load, one set of rows against another. No switch of its own (`?cloudstats=0`).
  `cloud-check` `telemetry` holds the record, the count of what came down and the report; `CLOUD_BREAK=1` writes the record as it was
  and is red. On the real page (software adapter, a scratch script): one record with the page's own marks. A LOAD STARTED ON THE FIRST FRAME'S SHADERS is not
  over at its first picture: the heaviest programs come in behind the start card, and `whole`, `shown` and `pipelinesMs` are then the
  short wait. The boot's mark `laterReady` (the last program built behind the start is in; the performance work makes the mark, none
  in the build yet) is the missing number: `later` on the load's record where the mark is there when the record is written, else ONE
  small record of the visit, kind `later` (`{ later, after }`: ms from the page's start, and from the start card), looked for on the
  records' half-minute beat for ten minutes (`loadtime.ts` `laterRecord`, `LOADTIME.laterMark`, `LOADTIME.laterMs`). The load's record
  is never held for it. The report joins the two by visit and prints "the last program built behind the start in at". NOT TIMED: a
  shader made when needed during play. A page opened in a tab nobody is looking at draws no frame, so its
  `whole` runs until it is looked at (`maxRafGap` says so). Never seen on a GPU.
- **A frozen picture in the records: the GPU's wait on a frame** (10-27, from a tester's notes on Windows: background shader compiles
  froze the picture for seconds at a time while animation frames kept coming at full rate, so every frame time we record looked
  normal; only a browser trace of PRESENTED frames showed it, and a page cannot read those). The stand-in: on a live frame, with no
  question already out, the page asks the GPU queue when that frame's work is done (`onSubmittedWorkDone`, handed to `Cloud.frame` as
  its fifth word) and the wait goes into the minute's `perf` record as `lag`: `n`, `mean`, `worst`, `over100`, `over500`, `over2000`.
  One promise at a time and a new question only every fourth live frame (`CLOUD.gpuLagEvery`: a freeze of seconds is caught as well),
  none on a pause card, none on a page that does not send; a wait the tab was hidden in is dropped; a queue that throws when asked is
  not asked again. ON in the page, `?gpulag=0` puts it off (`CLOUD.gpuLag`). The report's machine line says the longest wait and how
  many ran past half a second. UNPROVEN: nobody has set a real freeze beside it. A compile that stalls the browser's own drawing may or
  may not hold this answer back; the first Windows records will say, read against a trace. `cloud-check` `off` holds the counting;
  `CLOUD_BREAK=1` asks nothing. On the real page (software adapter, a scratch script, three of the page's own frames): the minute's
  record carried `lag`. Never seen on a GPU.
- **One reporter for errors** (10-27, the maintainer: "let's do an audit of all our errors and have them use some reusable error
  reporter that sends up to convex"). `src/cloud/report.ts`, a leaf any file may import: `reportError(where, err, data?)` for what was
  caught, `reportWarning(...)` for what the game went on from. `where` is a short stable name with dots (`gpu.splitrays.buffer`), `data`
  a few numbers the place knew. It prints the line a `console.error` would have and hands the report on; `cloudBoot` asks for the
  reports (`reportSink`) and writes each as an `error` record of the visit with `where`, the message, the top of the stack, `data`, and
  `level: 'warning'` on a warning. The first coming of each (by where and words) is a record, the rest are counted on the visit's `end`
  (`errors`, `warnings`); at most 16 distinct errors and 8 distinct warnings a visit, each with places of their own. What is reported
  before the backend is up waits (32 errors and 16 warnings at most), and a first visit's wait on until the notice has been shown; a page
  that dies with no backend up sends what it had reported with its `fatal`. Two comings whose words differ only in a number are one.
  An object beside a line is said in brief (a few keys; a buffer or a long array by its kind and length). A report is MADE once: the same
  line said again, whatever numbers it carries, is handed on as the report it already was (no stack is taken twice), and a page makes 200
  at most, so a line printed every frame pays for its first coming only. THE NET: the game page calls `tapConsole()` once, so every `console.error` and
  `console.warn` it makes is a record too, under the where `console.error` / `console.warn`, with no line of the game changed; a GPU
  validation error and a lost device have places of their own (`gpu.uncaptured`, `gpu.device.lost`: `gpu/device.ts` calls `reportError`, its
  printed lines word for word as they were). `reportError` prints exactly what the `console.error` it replaces printed. An error nothing caught is `window.error` /
  `unhandledrejection`, a dead page still its `fatal`. The report (`tools/cloud/errors.ts`) groups by
  kind and words and says `at <where>`: dead pages, then errors, then warnings. No switch of its own: it sends only where the visit's
  records do (`?cloud=0`, `?cloudstats=0`). NOT COVERED: a catch that says nothing (the audit counted 388 and 34 promise catches; about
  seventy are worth a record and are a one-line change each, in their owners' files), and the pages that are not the game hand their reports to nobody (the two
  replay pages no longer: "The replay pages report too", below). New code: call `reportError` where you would have written
  `console.error` in a catch, and never leave a callback's or a network call's catch empty. `cloud-check` `off` holds it (5 lines, and
  the dead page's); `CLOUD_BREAK=1` never asks for the reports. On the real page (software adapter, a scratch script): a `console.error`,
  a `console.warn` and an error nothing caught each became one record of the visit. Never seen on a GPU.
- **The reporter's places** (10-27, the second step of the audit: the lines and catches their owners gave leave for). FOUR CALLS:
  `reportError(where, err, data?)` prints what `console.error(err)` printed, every time (`data` is the record's and is never printed);
  `reportErrorLine(where, ...said)` and `reportWarningLine(where, ...said)` are a `console.error(a, b)` / `console.warn(a, b)` moved whole,
  printed as it was every time; `reportWarning(where, err, data?)` is what a catch that said nothing is given, and it PRINTS AT ITS FIRST
  COMING ONLY, so a catch on a timer or in a loop does not fill the console (every coming is still counted). NAMED NOW: `gpu.pipeline`
  (the label beside it), `gpu.shader.compile`, `gpu.splitrays.buffer` (the grid and the size beside it; `render/gatherSplit.ts`
  `splitRefused` calls the reporter and the page has no hook of its own for it), `gpu.device.lost`, `gpu.uncaptured`,
  `render.restir.twin`, `render.cut.twins`, `render.dust.twin`, `render.sharp.groups`, `bench.carry`, `bench.scene.said`, `bench.putBack`,
  `bench.card.closed`, `journal.onStop`, `journal.save`, `page.note`, `save.picture`, `replay.keep.prune`, `replay.keep.say`,
  `replay.keep.piece`, `replay.shelf.delete`, `replay.viewer.*` (boot, shelf.read, visit.read, notes.write, notes.send, send, send.shown)
  and `replay.engine.*` (saving, shelve, keep, notes.send, shelf.read). LEFT SILENT ON PURPOSE, each by its owner's word: a catch that only
  keeps a stored promise from being unhandled while its caller has the same rejection (the cut build, the particles' build, the boot's
  early promises: the red card has those); a readback that fails every frame after a lost device (`gpu/timer.ts`, the particles', the
  light meter's); a browser guard (no window, no IndexedDB commit, no CompressionStream, no Navigation API, a refused clipboard); a dev
  server or shot server that is not there; the benchmark's card closed before its upload answered. A report in `src/game/journal.ts` is
  the catch's only statement and hands over no game state: the reporter reads none, rolls no die and reads no clock. THE REPLAY PAGES are wired
  by the next bullet. Checked headless on both stands: `cloud-check` (107), `splitrays-check` (its pin turned),
  `replaybuf-probes` (121), `replaykeep-probes` (74, `REPLAYLEFT_BREAK=1` red), `gate.sh journal`, `probes.ts cardreplay`, `pipequeue`,
  `shaderswhenneeded`, `shaderrows`, `bench`, `gpulost`, `nogpu`, `rollback-check --static`, `dice-check`. Never seen on a GPU.
- **The replay pages report too** (10-27, the third step: `src/cloud/cloud.ts` `cloudReports(q, { page, uncaught?, build?, machine? })`).
  The replay viewer (`/replay`, one line in `src/replay/viewer.ts`) and the game's page while it plays a replay's tape (`?playtape`, one
  line in `src/main.ts`) boot no backend's side, so their reports used to wait in the page and go nowhere. Now each is one of the `error`
  records of a name made for that page load (`page-…`, the name its saved files already had), behind ONE `visit` record marked
  `page: 'replay'` or `page: 'tape'`; the page's death (`cloudFatal`) goes under the same name. UNDER THE RECORDS' OWN WORDS, and
  narrower than a visit's: a backend, the browser's switch, `?cloud=0`, `?cloudstats=0` (a tape's page carries the OFF words its tape was
  recorded under, `tapeSearch`), AND the notice already read in this browser or the sending its own choice. Such a page cannot show the
  notice, so a browser that has not read it sends nothing from there. THE TAB REMEMBERS ITS VISIT'S OFF WORD: the viewer's address has
  no words of its own (`replay.html#id=…`), so the game's page writes in the tab (`sessionStorage`, at `cloudAdopt`) whether its
  address put the records off, and a page that reports asks it; a tape's page does not take the mark off, the next visit somebody plays
  does; a viewer opened by itself in a fresh tab goes by the browser's own switch. (The viewer's notes and its Send go by their own
  rules, as before.) NOTHING IS ASKED OF THE BACKEND UNTIL THERE IS A REPORT: a page with nothing to say makes no request, and no
  hello ever (the browser's row counts visits and this is none). A page that knows no build asks its server for `/qa/commit` when it
  first has something to say. The viewer has no listeners of its own for an error nothing caught, so `uncaught` takes those there
  (`window.error`, `unhandledrejection`, kept from the first line of the page on); the game's page keeps its own (they lead to the red
  card). The viewer's console is tapped from that line on, as the game's is. Another tab that says no, or clears the browser's storage,
  stops the sending from here. No frame is counted and no `end` is written on such a page: a report there is its first coming and
  carries no count. `PAGE_REPORTS.secs` (2): a report goes up that long after it was made. The reader (`tools/cloud/errors.ts`) counts
  such a page apart from the visits ("0 visits · 1 other page"), says "on a page that is not a visit: replay" under the group, and
  reads a browser's machine off a visit somebody played where it has one. TWO PLACES NOW SAY A RESULT, not only a throw: the viewer's
  `replay.viewer.send` where the sending answered 'failed', and `replay.viewer.notes.write` where the write answered false (neither
  call ever rejects). STILL REPORTING TO NOBODY: the sound pages, the editor and the workbench (developer pages). Check: `cloud-check`,
  five lines headed "page reports", both stands; `CLOUD_BREAK=1` turns the first red (a browser that has not read the notice sends).
  On the real viewer page (a stand-in backend, a scratch script): no request with nothing to say, then one visit record marked
  `replay` with the build and four reports (a console.warn, a console.error, an error and a rejection nothing caught), and nothing at
  all in a tab whose visit put the records off. The game's page still boots clean with its own reports. NOT DRIVEN: a tape's page
  (`?playtape`) with a report on it. Never seen on a GPU.
- **A run's end in the records is the verdict's** (10-27, from the first daily read of the real records: six room clears won and not one
  `run` record, while all three exfiltrations had theirs). `Telemetry.frame` writes a `run` when the stage word it is handed moves into
  `done` or `failed`, and the page handed it `game.mission.stage`: only the STEALTH mission moves that word into an end. A house taken, a
  rescue, a night breach won and a team wiped end by their own rules and leave it where it was. The page now hands in
  `endStage(game.mission.stage, game.verdict.result)` (`src/cloud/telemetry.ts`): the verdict's end where a run has one (`game/verdict.ts`
  knows every kind's), else the mission's word. The record gained `over`, the verdict in brief (`kind`, `won`, `word`, `why`, `secs`,
  `approach`, `void` = a cheat was on, `clean`, `lost` of `of`, `foesDown` of `foesOf`, `hostagesAlive` of `hostagesOf`), and one end is one
  record: a word that moves from an end to the other end (a mission failed whose verdict then says won) writes nothing more. Under
  `?noverdict` there is no verdict to read and a squad's mission writes no `run`, as before. No switch of its own: a record's content
  under `?cloudstats=0`. `cloud-check` `telemetry` holds it on a real game (the kill house's room clear, every defender down: the word
  stayed `infiltrate`, one record, `CLEAR`); `CLOUD_BREAK=1` reads the mission's word alone and is red. RECORDS FROM BEFORE this are
  missing every squad win and loss that was not a player down: count those from the kept replays, not from `run`.
- **The display's rate in the records** (same read: `machine.hz` was there only on a browser's first visit). The `visit` record is
  written the moment the browser may send. On a first visit that is after the notice has been up two seconds, by which time the quality
  ladder has seen a second of frames (`Quality.displayHz`); a returning browser writes it at boot, before a frame was shown, so the rate
  was 0 and left out. The rate now rides on every `perf` record and every `run` as `hz`, once it has been seen. The `visit` is as it was.
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
  says "replays, stats, settings and saved files"; it does not list these. Trimming them was the maintainer's choice, put to him 10-26 before any
  backend existed, of three: drop the full user agent and the language, keep everything, or the GPU and OS only. HIS PICK, 10-27: KEEP
  EVERYTHING. Nothing is trimmed. Should that change, the lines to cut are one each: `cloud.ts` `hello` (`query`, `lang`) and the
  `machine` closure in `main.ts`.
- **Not sent**: the notes written on a kept visit (the viewer's and the list's; the dump reader would take a `{k:'notes'}` line), and a
  visit removed with the list's ✕ stays on the backend.
- **CHECKS** (`npm ci` in `cloud/` first): `bun run tools/qa/cloud-check.ts` (under bun, both stands: `off` · `backend` · `tapes` ·
  `identity` · `telemetry` · `prefs` · `files`; `CLOUD_BREAK=1` in front must FAIL `tapes`, `off`, `telemetry` and `files`: every third piece is dropped, a
  browser that sends by the default does not wait for the notice, and a saved file's row is never written) and `PORT=<dev server> node tools/qa/cloud-page.cjs`
  (headless Chromium, software adapter, three browser profiles; its step 4 sets the address a hosted build carries on the dev server's page and
  holds the notice rule: no request before the title's line, the line, then it sends). BOTH RUN AGAINST A STAND-IN (`cloud/test/standin.ts`: the real functions and
  schema in the service's test library behind the same two HTTP routes). `cd cloud && npx tsc --noEmit -p convex/tsconfig.json` typechecks
  the functions.
- **Known limits**: the page has no secret, so anyone who reads it can call what a page calls. Every call is bounded; the TOTAL is not (no
  rate limit, no cap a browser or a visit). No accounts: an id is as private as its browser's storage and the links made from it.
- **Saved files go up too** (the maintainer, 10-27: "can you make it so benchmark results upload automatically? can use convex file storage" ·
  "we should do that for everything that gets downloaded"). `src/cloud/files.ts` `FileSync` + `cloud/convex/files.ts` (table `files`, the
  bytes in the deployment's FILE STORAGE): `files:start` answers a one-time address, the page POSTs the bytes there, `files:put` writes
  the row (visit, browser, when, name, kind, type, size, build, and `meta`: the few facts its saver put beside it as one JSON text, 16 KB at
  most — a benchmark's report, a commit). The page's way in is `Cloud.save(name, blob, kind, meta?, done?)`, the whole answer (`sent`, `held`,
  `off`, `no`, and later whether it arrived: the bullet "Nothing is downloaded any more" below), made to be the sink of the page's one
  door for saved files (`src/host/savefile.ts` `SAVED.sink`; which answers the door reads as taken is the door's business).
  `Cloud.file` is the older yes or no beside it: true only of a file that is going up now, false for one held for the notice, which
  goes when the notice has been shown. Under the browser's switch and the notice like the
  rest, its own word `?cloudfiles=0`. A browser still waiting for the notice HOLDS its files in memory (`FILESYNC.hold` 8, 192 MB,
  oldest let go: a page opened straight onto a benchmark) and sends them once the notice has been shown; a switch that is off holds
  none. THE NOTICE ON ANOTHER CARD: `cloudNote(card, undefined, {})` (`src/ui/cloudnote.ts`) puts the title's line on a card that is not
  the title (a benchmark's result) and counts it seen by the same rule, less the live title; with `{ told }` it serves a page that has
  no hook at all (the error card of a page that never started: `cloudFatal`'s answer). `node tools/qa/cloudnote-page.cjs` holds that
  rule in a real browser on an empty page (four lines, ten seconds). The red error card calls it (below); the benchmark's card is the performance side's to add. BOUNDS: 32 MB a file, 64 files a visit, a name of plain characters with no folders (the backend refuses the rest and lets the
  bytes go again). THE NOTICE NAMES THEM: once the page hands files over (by its download door: a recorder's take, a session journal; by `cloudFile`: a saved replay; an
  exported checkpoint, a benchmark's result), the title's line reads "replays, stats, settings and saved files" (`CLOUD_NOTE`); the
  maintainer's words for sending them were "everything that gets downloaded". A browser that was told by the older line is not told again.
  NOT SENT: kind `level` (`FILESYNC.skip`: a level drawn in the editor is somebody's own work, not a record of play; lifting it is the
  maintainer's call). Read back: `tools/cloud/pull.ts` writes them under `files/` with `files.json` (`--no-files` leaves them).
  `cloud-check` family `files` (the red path sends the bytes and never writes the row). KNOWN LIMIT: bytes whose row was never written
  (the page died between the two calls) stay in storage with no row; nothing sweeps them. NEVER RUN against the real service: the
  stand-in's storage routes are this repo's own (`cloud/test/standin.ts`), so the service's upload address, its CORS answer to a POST
  of bytes and its size limits have not been met by this code.
- **Crashes, and reading them** (the maintainer, 10-27, after a tester's AMD machine failed to get a WebGPU adapter and the backend
  heard nothing of it: "some way to get crash telemetry and metrics would be great. kinda like 'we have sentry at home' with convex").
  THE RECORD: kind `fatal` = the page died: `message`, `stack` (lines, 16 at most), `stage` (where: 'adapter' · 'device' · 'boot' ·
  'frame' · …), `data` (the reporter's own bag, plain JSON) and, on a visit that was up, `played` and `frames`. THE WAY IN is one
  function for whatever puts the error card up: `cloudFatal(q, report)` (`src/cloud/cloud.ts`). Before `cloudBoot` — the game never
  started, no GPU, no kept store — it names a visit itself (`fatal-…`), writes that visit's `visit` record (`started: false`, the
  machine the reporter hands in or what `navigator` says) and the `fatal`, and sends at once; after `cloudBoot` it is the live visit's
  `Cloud.fatal` and goes out on its records at once. Three a visit at most (`FATAL.max`). Same switch (`?cloudstats=0`, `?cloud=0`,
  the browser's row) and the SAME NOTICE: a browser that has not seen it gets `waits: true` back, and the error card that shows the
  notice's line (`ui/cloudnote.ts` works on any card) sends by `told()`. null back = nothing is or will be sent. The `error` kind stands
  as it was (an uncaught error or rejection, a WebGPU error, a lost device: once a message a visit, eight messages). WHO CALLS IT: `main.ts`
  sets the red card's one listener (`gpu/device.ts` `FATAL.tell`, told of every card after it is drawn: a start with no GPU, a lost
  device, a throw in the frame loop, an uncaught error) before `main()` runs. `stage` is the card's own ('adapter', 'device'), else
  'boot' while the start's story still has a sentence and 'run' after; `data` = `{ where, adapter, facts }` (the start's sentence, the
  adapter's name, the reporter's bag: a start with no GPU hands what the machine said). A page that died before a visit was up asks
  its own server for the build's commit (`/qa/commit`, a second and a half at most). A browser that has not been shown the notice
  gets it ON THE CARD, first, and the record goes when it has been in view two seconds. Nothing under `?kitview`, on a build with no
  backend, or under the words above. `node tools/qa/cloudfatal-page.cjs` holds it on the real page against the stand-in (the
  adapter stubbed away; a built-in address as a hosted build has; then a page that booted and threw): 6 lines pass,
  `CLOUDFATAL_BREAK=1` red. NEVER SEEN: a machine that really has no adapter, the real service taking a `fatal`.
  READING: `bun run tools/cloud/errors.ts [--since ISO] [--metrics] [--json]` (address and key from the environment, as the pull;
  queries only): a block a distinct message (two that differ in a number, an address or a place in a file are one), pages that died
  first, then by how many browsers met it, each with its count, visits, browsers, builds, GPUs (the adapter, or "no WebGPU adapter ·
  <the WebGL card>"), stage, first and last time and a stack; `--metrics` adds a line a browser (GPU, visits, the shader pipelines'
  wait, the frame's median and 99th, the GPU's ms, the preset it sat on, minutes played). It reads the newest 2000 records a kind.
  `cloud-check` holds the record (`off`) and the report (`telemetry`). Run once against the real backend, 10-27: two browsers, no
  crashes and no errors yet, so no real `fatal` has been read.
- **Notes on a replay go up too** (10-27: with the hand-over file leaving the page, what it alone carried needs a road). The notes
  written on a kept visit in the viewer, and the F7 shelf's saved windows with theirs, are lines in the backend's `notes` table
  (`cloud/convex/notes.ts`; a line per browser and key, `visit:<sid>` or `f7:<session>:<savedAt>`, written over when it changes), in
  the hand-over format's own shapes (`{k:'notes'}`, `{k:'f7'}`: `replay/kept.ts`). `src/cloud/notes.ts` (`NoteSync`) is a sync of
  what the browser holds: a round every 30 s reads the kept rows and the shelf and sends a line only where its text differs from the
  one that last went (marks in `twopointsix.cloud.notes`); notes taken off again go as a line with none. A saved window whose visit is
  NOT kept in this browser (its trace is in no tape) goes up whole, once, as a saved file of kind `f7`. THE SWITCH IS THE TAPES'
  (`?cloudtapes=0`, `?cloud=0`, the browser's row, the notice first), and the notes of a visit whose tape may not go stay with it, as does a window saved out of such a visit:
  a note is part of a replay, which the title's line names. BOUNDS: 256 KB a line, 16 lines a call, 1,500 keys a browser. THE VIEWER:
  `cloudNotes(q, store)` (`cloud.ts`) is one round for a page that has no game: the replay viewer calls it after each save of a note
  (`replay/viewer.ts` `flush`), so a note typed, changed or emptied there is on the backend within the second, with no game page
  open. Rounds run one at a time with at most one more waiting (two saves a moment apart must not both send the line). Held on the
  real viewer page by `tools/qa/cloudlocal-page.cjs` (typed, edited, a second one, the first emptied, then pulled and read in the
  visit's moments). READ BACK: `tools/cloud/pull.ts`
  appends the lines to the hand-over file it writes, after the visits, each behind a line break (a cut tape's last bytes may not end
  a line), so `tools/qa/replay-dump.ts` reads them as it always did; `--no-notes` is not a word, a backend from before the table is
  read without them. `cloud-check` family `notes` (six lines; the red path marks a line as sent that never went). NEVER RUN against
  the real service; nothing calls the page's shelf under a real browser in a check (the shelf's two reads are `replay/shelf.ts`'s own).
- **A local build has a local backend** (the maintainer, 10-27: "we can make the local build use a local convex deployment").
  `bun run dev` sets `TPS_LOCAL_CLOUD=1`; `tools/serve.ts` then calls `tools/cloud/local.ts` `localCloud()`, which runs
  `convex dev --tail-logs disable` in `cloud/` (Convex's own backend on this machine; it pushes `cloud/convex/` and keeps pushing as
  the functions change), with no account when `cloud/` has no `.env.local` yet (`CONVEX_AGENT_MODE=anonymous`) and NEVER with a
  deploy key, a deployment or an address from the shell (those are taken out of the child's environment: a dev server must not be
  able to push to the deployment a key names). Needs `cd cloud && npm ci` once; without it the server says so and the page has no
  backend. `TPS_DEV_CLOUD=<address>` in front hands the server a backend that is already up (a stand-in, a `convex dev` in another
  terminal, or a hosted address), and nothing is started. NOT `TPS_CLOUD_URL`: that word is the hosted build's address for
  `tools/build.ts` and the reading tools, and a shell that has it exported must not turn a dev server's play into the real
  backend's data (a review found the first cut read it). A backend that answers at the address the tool last wrote but that THIS
  start did not bring up is not taken: it may hold old functions or be another project's; `convex dev` stopping by itself is said
  in a sentence. The dev server stops its `convex dev` with an interrupt (the tool stops its backend on that, not on a plain kill)
  and starts it only once its own port is bound. A plain
  `bun run tools/serve.ts` (every check's server) starts nothing. THE PAGE: a page served from `localhost` / `127.0.0.1` / `[::1]`
  with no address built in asks its own server once, `GET /qa/cloud` (`cloud.ts` `cloudFind`, awaited by `cloudAdopt` in the game
  and by `cloudNotes` in the viewer; a built page has an empty `dist/qa/cloud`), and takes a loopback or https address it is told as
  its backend. ONLY A LOOPBACK ONE is the browser's OWN choice (`cloudLocal`, `cloudChosen`): it sends with no notice to wait for and
  shows no line about sending (`ui/cloudnote.ts`), since nothing leaves the machine. A hosted address named that way is a backend
  like a built-in one: the notice first. `?cloud=0` and the per-kind words still put it off, and an address
  on the address bar still wins. The server does not wait for the backend: the address is handed out once it answers `GET /version`,
  so a page loaded before that (a first run downloads Convex's backend) has none for that load. Data stays under `cloud/.convex/`
  (ignored by git). READ IT BACK: `bun run tools/cloud/pull.ts --local --out DIR` and `bun run tools/cloud/errors.ts --local` take
  the address and the key from `cloud/.convex/local/default/config.json`; the key is never printed. CHECKS: `cloud-check` family
  `local` (eight lines, a fake `convex` and the stand-in: who asks, what is taken, that a hosted address is not the machine's own,
  what is started and with which environment, when the address is handed out, that a backend this start did not bring up is not
  handed out, the interrupt on stop); `node tools/qa/cloudlocal-page.cjs` (its own dev server handed the stand-in by `TPS_DEV_CLOUD`: a game page with no word
  has the backend and its tape goes up; the viewer's notes as above; `CLOUDLOCAL_BREAK=1` starts the server with none and must
  fail). NEVER RUN: `convex dev` itself. The machine these were written on cannot fetch Convex's backend, so the first real start
  (the download, the anonymous deployment, where it writes its address) is the maintainer's; the two places the address is looked
  for (`cloud/.env.local`'s `CONVEX_URL`, then the local config) are read from the tool's own source, not seen working.
- **Nothing is downloaded any more: the backend's side** (the maintainer, 10-27: "we should get rid of local downloads entirely"; his
  pick for a file that cannot go up, from a card: "No downloads, editor keeps files", so no browser download for any kind but the
  editor's level files, and a save that cannot reach the backend SAYS it was not sent). This step is the backend's half only: the
  door (`host/savefile.ts`) and each save button are their owners' and still download until they change. WHAT THEY GET:
  `Cloud.save(name, blob, kind, meta?, done?)` (`cloud.ts`; beside `Cloud.file`, whose true / false stays for the sink line as it
  is) answers `Saved`, in the door's own words: `'sent'` on its way (queued and going, not yet arrived), `'held'` until the notice has been read, `'off'` this browser does not send files (its
  switch, `?cloud=0`, `?cloudfiles=0`), `'no'` this file cannot go (empty, over 32 MB, a level); `done(arrived)` is told once, later,
  only of a file that was taken: true = its row is on the backend, false = it was let go (the backend refused it, four failed tries,
  the switch went off while it waited, more than eight waiting). `cloudFile(q, name, blob, kind, meta?)` is the one-file way in for
  a page that boots no game (the replay viewer; nothing calls it yet): it resolves `'sent' | 'failed' | 'off' | 'no'`, files the
  file under a name made for the page load (`page-…`), tries a failed sending again by itself, and on the game's own page goes
  through the visit that is up. A browser that has not read the notice on a page that cannot show it is `'off'`. `FileSync.add`
  takes the same `done`; a file whose bytes are on their way is no longer the one let go when too many wait. A build with no
  backend has no `Cloud` at all, so its door has no sink and its saver knows that by itself. CHECK: `cloud-check` family `files`,
  three more lines (how a file ended, the page's answer, the one-file way in; the red path says "arrived" of a file with no row).
  `Cloud.filesIdle(capMs = 15000)` is for a page about to leave (a card's Close navigates, and the sending is a file's only copy):
  true once every file that may go has gone or been let go; false when the cap is over, a sending failed, or what is left waits for
  the notice. `cloudSendsFiles(q)` is what a page asks before it draws a save button (a backend, the switch, the files' word, the
  notice read or the sending the browser's own choice; game or no game; it sends and counts nothing). `cloudCarries(q, sid)` says
  whether the notes of visit `sid` go up by themselves from this page (the replays' rules, and the visit is one of this browser's own
  whose tape may go): false for a replay opened from a file, where a note reaches the backend only if somebody sends the file. A
  REPLAY IS A REPLAY WHICHEVER ROAD IT TAKES (`replayMay`): a saved file of kind `replay` or `f7` goes only under the replays' own
  word as well (`?cloudtapes=0` holds it back, through `Cloud.file`, `Cloud.save` and `cloudFile` alike; before, the door sent a saved
  replay under that word), and one that names a visit of this browser in its facts (`meta.session`) only while that visit's tape
  may go, so a window cut out of a visit played with the sending off stays with it. A replay that names no visit (a file opened in
  the viewer, sent by a press) is not judged. A TEXT FILE IS PACKED on its
  way (`FILESYNC.packFrom` 256 KB; JSON, ndjson, plain text; `CompressionStream`, and a browser without one, or a packer that
  fails, sends the file as it is): a recorder's long take is some 50 MB of JSON, past the 32 MB bound on what is sent, and with nothing downloaded it would be
  kept nowhere. The row is `<name>.gz`, `application/gzip`, with `packed: 'gzip'`, `rawBytes` and `rawType` in its facts, and
  `tools/cloud/pull.ts` writes it out as it was saved under its own name. A text file is held up to `rawMax` 192 MB (= `holdBytes`: the
  newest file is never the one let go, and one that could not be held alone is not taken); one past the bound even packed is let go
  and its saver told. The pull unpacks no more than `rawMax` (a row's facts come from a browser); a long name keeps its head and
  its extension under `.gz`. `cloudFile`'s retries on a page with no rounds of its own run until every file handed over has gone
  or been let go, so each saver hears its end. The backend's bound and schema did not move. A NO-GAME
  PAGE'S NOTES (`cloudNotes`) now carry a saved window whose visit is not kept whole, through the page's own file intake, as the
  game's page does. CHECK: three more `files` lines (the page's question, a 3 MB take packed to a fifteenth and pulled back byte
  for byte, the wait). NOT RUN: the no-game page's whole-window road (the shelf needs a browser), packing in a real browser.
  NOT DONE HERE and nobody has seen it: any save button's words, the door without its click. NEVER RUN against the real service.
- **THE FIRST REAL READ (10-27)**. The maintainer made the host's project and the backend's, and played. Read back with
  `tools/cloud/pull.ts` (queries only): one browser (an Apple-silicon Mac, Chrome, Metal, timestamp queries on), four visits on the
  build's head, two replays WHOLE (82 s in 22 pieces, 20 s in 5) that `replay-dump.ts` unpacked and of which one was run again by
  `replay-at.ts --rerun` (exit 0), 27 records (`visit` 4, `boot` 4, `perf` 6, `run` 2, `end` 11) and a settings row at its eighth
  revision. So the host's build, its deploy of the backend, the address in the page, the service's CORS answer and the notice's gate all
  worked once, on one machine. WHAT THE RECORDS SAID: the pipelines took 13.4 s on the first load and 1.2 to 9.3 s after; Auto sat at
  Very low and Low at half scale with the GPU at 10 to 14 ms a frame. Two flaws showed and are mended (the tab bullet and the two pass
  lists, above).
- **STILL NEVER SEEN**: the hosted page itself by anyone but the maintainer (this environment may reach the backend, not the host), the
  link between two browsers on the hosted build, the notice's line, the two Options rows and the link's question by eye, two tabs of
  one browser (the live switch), a Windows machine, a browser other than Chrome, the mended records from a real page.
