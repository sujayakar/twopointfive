# Networked play (two-player co-op)

The maintainer's ask (2026-11-02): two-player co-op on squad missions, a game server with authority for each game, clients that predict locally and roll
back, WebSockets, CPU senses on the server. What is built so far is the server, the wire and the page as a client on confirmed ticks. Nothing of it runs unless a server process is started
and a page is opened with `?net=`: a page without that word is the build as it was (the loop's `tick` seam is absent, `netFromUrl` returns on its first
line).

- **What travels is the input timeline, never the state** (`src/net/wire.ts`). The game plays one fight to the bit from its recipe and its seats' records
  (`docs/agents/replays.md`: the game's own maths, the senses on the CPU, the input record, a record from anyone), so the server decides which record each
  seat's tick ran on, tells every client, and sends a state word (`game/digest.ts rawBits`, chained) every `NET.wordEvery` ticks as the alarm. A NETWORKED
  GAME IS A RUN TAPE WRITTEN LIVE: its setup is the tape's RECIPE (`game/runtape.ts recipeOf`: the die, the page's address, every knob table, the kit),
  made by the server of the game it stood, and every machine — the server too — stands its game FROM the recipe (headless: `tools/mp/stand.ts standRecipe`,
  which is `tools/qa/rerun.ts` on a tape of no steps), so all of them come to the game by one door. Four tables are forced on whatever a setup says
  (`wire.ts NET_TABLES`): `EXACT.on`, `SENSE.cpu`, `CMDCHECK.on`, `VERDICT.on`; whoever STANDS a recipe refuses one without them (`wire.ts recipeWhy`:
  a record on the wire carries no view matrices, so a game that believed its seats would stand its men still), and refuses one whose mission does not
  come up. A setup's own `tables` are a host's settings: each key must be a knob this build has, of the knob's kind, or the setup is refused
  (`stand.ts layWhy`; laid with `knobs.ts assignDeep`).
- **Every seat is refereed on every machine** (`src/net/step.ts netSeats` sets `seat.remote` on the server AND on each client, a client's own seat too): the
  eye and both view matrices are worked out again from the pose (`cmd.ts checkCmd`), so a record on the wire (`WireCmd`) is the keys, the buttons, the
  cursor, the view's size and nine pose numbers — about a hundred bytes — and one working makes the view everywhere. Numbers cross as JSON to the bit
  (`cmd.ts numWord` / `wordNum`: −0, NaN and the infinities as words).
- **One tick, the same everywhere** (`src/net/step.ts netStep`): `Game.stepSeats(1 / 60, cmds)`, then what a host does between two ticks that the next one
  reads — `lights.update`, `world.relist` (the list the game reads; the pack for the picture waits for a draw, `docs/agents/overview.md`), `hudExpireBubbles`. A client's page must step confirmed ticks through this function and nothing else.
- **The server's authority over input is one rule** (`src/net/timeline.ts SeatLine.take`, asked once a seat a tick): the seat's own record for that tick if
  it came in time; else the newest record the seat is known by with its one-tick edges off (keys and buttons stay held, the view stays); a record that came
  LATE is not lost — it becomes the newest known and its edges (a press, a click, the wheel) land on the next tick; a seat nobody sits in holds nothing.
- **The server's core** (`src/net/server.ts NetServer`; host-neutral: the process hands in its clock, its sockets and the stood game). `waiting` until every
  seat has somebody in it WHOSE CLIENT HAS STOOD ITS GAME and said `ready` (no tick runs: a fight never starts with a man standing still while his page
  loads) → `playing` → `ended` when the game's own verdict says so (`game.verdict.result`, verdict.ts: every mission kind's end, and all players down),
  then a grace and it is done. A hello carries the wire's version, the game's id, the build and — for a seat — the seat's key; none of a key is a WATCHER,
  sent every tick and sending none (`NET.watchers` at most). A server that names its build lets in no other, an unnamed client included; a checkout with
  uncommitted changes names none (`stand.ts buildName`, as `tools/build.ts`). THE RUN SO FAR is kept as the text of its tape messages (`NET.tapeChunk`
  ticks each) and every connection has a place in it: one that is behind — a joiner, a seat back after a drop, a socket that would not take a tick — is
  sent a stretch on each `advance` and none while its socket is backed up (`drained`), so a hello costs nothing however long the game has run and no tick
  is lost to a slow reader. WHAT A CLIENT SENDS IS CHECKED before it is kept: a message's length (`NET.maxText`) and a record's every field
  (`wire.ts wireOk`: a shape; how many keys and how far a view may be are the referee's, `cmd.ts checkCmd`). A `lost` is counted from a seat, once. It
  ends itself (`done`): nobody connected for `idleS`, the mission over and the grace run, or `maxS` since it was made. A host that falls behind runs at
  most `NET.catchUp` ticks in one go and past `NET.slipAt` lets the game's clock slip (`slips`), never a spiral.
- **The client's core** (`src/net/client.ts NetClient`; host-neutral). Its world runs on CONFIRMED ticks only (`walk`), which is the server's game to the
  bit; the first state word that differs is kept (`bad`) and told to the server. The seat's own records go ahead of the server: WHICH TICK a record is for
  is worked out from time (the server's tick against the host's clock by the least-delayed tick message of the last two `NET.clockMs`, plus `margin`),
  not counted, so a host whose beats come late skips ticks and is never left sending into the past, and a server that let its clock slip is followed
  inside two seconds; the server's pace notes (how many of the seat's records were waiting when a tick ran) move `margin` toward `NET.lead` waiting. A
  note is acted on only when it is about records that were on the way: not one about a tick sent for before the margin last moved, nor one about a tick
  the client had sent nothing past (it was standing its game, its tab slept). `ready()` once the host has stood its game. `pending` = the seat's records
  not confirmed yet: what a prediction of its own man will play ahead of the confirmed world. NOT BUILT: that prediction, a resync after a state word
  differs, the page's side of any of it.
- **The page as a client** (`src/net/page.ts`; `/?net=<ws address>&netgame=<id>&netkey=<a seat's key>`, no key = a watcher). It stands its game AS THE
  ENGINE REPLAY STANDS A TAPE'S: `netFromUrl` (asked in `main.ts` where the tape's question is) says hello, takes the welcome's recipe, and sends the page
  to the recipe's own address with its seed (`engineplay.ts recipeSearch`, which `tapeSearch` is now a call of); then `main.ts` lays the recipe over this
  machine's at the same points of the boot through `replay/recipestand.ts RecipeStand` (`tables` · `env` · `stood` · `tune` · `flags`: MOVED out of
  `TapePlay`, which holds one; `rerun.ts` does the same headless — keep the three in step). Attached, it holds the stood mission against the recipe's,
  marks every seat refereed and says `ready`. THE SEAT'S KEY is read off the address once and kept for the tab (sessionStorage): the address the page is
  moved to does not carry it, so a visit's record never holds it (a browser that keeps nothing leaves it on the address). WHO STEPS THE GAME is the
  loop's seam, `host/loop.ts LoopCtx.tick` (absent = `game.update` then the lights, the loop as it was): on a page tick the player's input is sampled
  (`sampleCmd`, off the page's own camera) and sent, the game is stepped only through the server's confirmed ticks by `netStep`, and the host's half
  of a tick is done round them (`Game.hostBefore` / `hostAfter`: `update`'s own lines, named — the peek's nudge, a blast's kick, a cut, the live view
  put back). HOW FAST: one confirmed tick a page tick, two while more than `NET.lead` wait; from `NETPAGE.far` waiting (it joined late, its tab slept)
  the page CATCHES UP as a replay seeks — the mixer shut, as many ticks as fit in `NETPAGE.farMs`, the hands off the keys (decided before the tick's
  record is made), nothing drawn between two of those ticks, the picture's queues let go when it is level (`engineplay.ts unseenDone`, the seek's own). No playout buffer: a tick that comes late is a tick drawn late. THE GAME IS THE SERVER'S: a card's Restart and a checkpoint's load are refused in words
  (`Game.restart` and `saves.load` called from outside a confirmed tick on such a page), the cards' Quit leaves the page (`netLeave`), no try, streak
  or best is filed for such a run (`main.ts vBooks`, the par best), the sandbox flags are the recipe's before every tick, the frozen frame's own write
  to the launcher is left to the tick, and a page that has PARTED from the server (a state word differed, a hole in the stream) closes its socket, so
  the server lets go of its seat and holds no key for it. HANDS OFF: under a card that freezes the page, and when the tab is hidden, one record that holds nothing and carries the game's own word
  'Paused' is sent (`wire.ts restOf`, `NetClient.rest`, never held back for the beat's pace: a held launcher's round is let go unsent, as on the page alone) and the confirmed ticks are
  still walked: the game is not this player's to stop. One line of words says what is wrong when something is (waiting for the other player, catching
  up, the connection lost, out of step, the game over); a goodbye before the welcome is told by its `code` (`wire.ts ByeCode`), never by its English.
  The page's own replay buffer is off on such a page (`noreplay` in its address). `window.__net.state` for a script. NOT BUILT: the player's own man
  predicted (he answers a key when the server's tick comes back, a few ticks later), a playout buffer, a resync, a second seat's view
  (`Game.localSeat`, the seats work), reconnecting without a reload, the lobby. KNOWN HOLES: a debugger row that writes to the game (a restage, a
  teleport) is this page's alone and the next state word says so; Shift+Enter and Enter when down are in the record, so they restart the mission for
  everybody — the seats work and the lobby decide what a restart is in co-op.
  THE CHECK: `PORT=<dev server> node tools/qa/mpclient-page.cjs` (headless Chromium on the software adapter against a real server process, a bun watcher
  beside it; about four minutes, most of it the page's boot): moved to the game's address with the key off it · the server waits while the page stands
  · SAME FIGHT (every state word of 600 ticks the server's: V8 with the page's engine against JavaScriptCore with none) · the server ran ticks on the
  page's records and the man moved · no page error. `MPCLIENT_BREAK=1` (the page flips the last bit of its man's place after tick 90) must fail the
  same-fight line. RUN
  2026-11-02 on the cloud box: every line PASS on the four-man room clear seed 7 (a confirmed tick 8 ms in Chromium there with a watcher sharing the
  cores), and A TAB THAT SLEEPS: asleep 3.5 s with a walk key down and nothing said the server walked the man on 2.4 to 5.2 m, with the hands taken off
  first 0.3 m (three runs; the line picks the way he has room to walk, and asks only the waking if he is down by then); each time the page woke some
  210 ticks behind, caught up and still matched every state word. The break word fails the same-fight line
  (and its-own-man: a page out of step stops sending). Driven by script with nothing drawn but the last frame: nobody has SEEN a networked game, on a
  GPU or off one, and nobody has played one; a real tab's `visibilitychange`, the pause card over a networked page and a card's Restart on one were
  not driven (the sleeping-tab line calls `NetClient.rest` itself).
- **The process** (`tools/mp/server.ts`, bun; one process, one game — module state is per process, `docs/agents/replays.md` KNOWN LIMITS). It knows nothing
  of where it runs. TO A CONTROL PLANE `ended` MEANS THIS SERVER IS DONE: from the mission's end until the core is done (the grace, the players' end
  screen) a heartbeat says `playing` with the outcome beside it; `/health` keeps the core's own word. A stop signal or a tick that throws stops the
  core first — a bye to its clients, a last `ended` heartbeat waited for three seconds at most. A setup's or a recipe's `levelFile` is the bare NAME of
  a level file, never a path (`stand.ts levelWhy`: the address comes from whoever asked for the game); a seed is a whole number up to 4294967295. Environment: `MP_PORT` (5300) · `MP_HOST` (127.0.0.1; a sandbox behind a tunnel says 0.0.0.0) · `MP_GAME` · `MP_SETUP`, THE SETUP
  RECORD a host's page hands the control plane and the control plane hands on: `{scenario` (the mission's page address, `?roomclear&squad4`), `seed`,
  `knobs?` (knob tables over the page's own), `seats?`, `seatKeys` (one a seat), `build?`, `game?}` — made into a recipe by `tools/mp/setup.ts` in a
  child of its own — or `{recipe, seatKeys}` · `MP_SEATKEYS` (where the record carries no keys) · `MP_CONTROL` + `MP_SECRET` (heartbeats: POST
  `{secret, …health}` every 10 s and at once on a change; a server that is done says state `ended`; an answer of 410 = the control plane has no such
  game and the server stops) · `MP_IDLE_S` (60) · `MP_MAX_S` (3600) · `MP_GRACE_S` (15) · `MP_BUILD` (default: the record's, else the checkout's
  commit, none with uncommitted changes). `GET /health` → `{game, build, state, tick, seats, watchers, slips, lost, lines}`, answered once the game is
  stood (3 to 4 s after the start on the cloud box). Exit 0 when done, 1 when its game could not be stood or threw, 2 on a bad environment. By hand:
  `MP_SETUP='{"scenario":"?roomclear&squad4","seed":7,"seatKeys":["k0"]}' bun run tools/mp/server.ts`, then `bun run tools/mp/bot.ts --key k0` (a
  scripted client: `--lag`, `--jitter`, `--after`, `--ticks`, `--build`).
- **THE CHECK**: `bun run tools/qa/mpserver-check.ts` (a server process and scripted clients over real WebSockets on one machine; about a minute):
  the rule on its own; THE CORES (the server's core and the client's on a made-up clock with a counter for a game: no tick before `ready`, messages that
  are not records, three seconds of sending nothing, a server stalled a second, a late watcher over a socket that drops messages, a watcher too many, a
  client of no named build, whose `lost` counts); stood; refused hellos; SAME FIGHT (a seated client and a watcher, 1200 ticks, every half second's state word the server's, the man
  moved and fired); a late joiner; LAG (a second server started with the record as a control plane hands it over, a seat 60 to 100 ms away each way; its bar is on the records that CAME — over half ran as the seat's own — never on the run's ticks: how many a starved box lets a bot send is the box's); GONE (its
  heartbeat answered 410: it says `ended` and stops); the heartbeats at a stand-in control plane; the server ending itself when idle. `MPSERVER_BREAK=1` (the seated client flips the last bit of its own man's place after tick 90) FAILS the two state-word lines, as it must.
  `--setup '{…}'` plays another mission. RUN 2026-11-02 on the cloud box (bun 1.4.4, Linux x64, four cores, nothing seen on a GPU or in a browser): the
  four-man room clear in the office seed 7 and the consulate rescue seed 3, every line PASS; the break word fails 2 lines. A tick is 9 to 15 ms there
  with the server and three clients sharing the four cores, so a scripted client's own beat is starved (it sent 500 to 850 records in 1200 ticks and the
  server held its keys through the rest, and the server let some fifty ticks slip): that is the box, not the wire, and the words still match. With one
  seat per game today the second client is a watcher; two seats wait on the seats work. BY HAND, once: a recipe with `CMDCHECK.on` off is refused; a
  setup naming `constructor`, a knob the build lacks or a null for a number is refused; with the player put down the stood game's `ended()` answers
  `{won: false, end: 'DOWN'}` one tick later. No check plays a mission to its end over the wire.
- **What nobody has seen or measured**: a page a person plays as a client (the page check is a script's); two machines; a real network; a Mac; a server in a sandbox; a second seat; a game longer than
  twenty seconds over the wire; a mission ENDING on a server (the verdict's answer was read by hand, never over the wire); the run's memory on a long
  game (its text is kept for joiners: about 20 KB a second with two seats sending every tick, 80 MB over the longest life); a real socket backing up (the
  check's dropping socket is made up; bun's own `drain` was not seen to fire).
