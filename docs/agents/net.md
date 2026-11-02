# Networked play (two-player co-op)

The maintainer's ask (2026-11-02): two-player co-op on squad missions, a game server with authority for each game, clients that predict locally and roll
back, WebSockets, CPU senses on the server. What is built so far is the server and the wire. Nothing of it runs unless a process is started for it: the page
does not connect to anything yet, and a game stood the usual way is the build as it was.

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
  reads — `lights.update`, `world.upload`, `hudExpireBubbles`. A client's page must step confirmed ticks through this function and nothing else.
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
- **The process** (`tools/mp/server.ts`, bun; one process, one game — module state is per process, `docs/agents/replays.md` KNOWN LIMITS). It knows nothing
  of where it runs. Environment: `MP_PORT` (5300) · `MP_HOST` (127.0.0.1; a sandbox behind a tunnel says 0.0.0.0) · `MP_GAME` · `MP_SETUP`
  (`{"search":"?roomclear&squad4","seed":7,"tables"?}` — made into a recipe by `tools/mp/setup.ts` in a child of its own — or `{"recipe":…}`) ·
  `MP_SEATKEYS` (one a seat) · `MP_CONTROL` + `MP_SECRET` (heartbeats: POST `{secret, …health}` every 10 s and at once on a change) · `MP_IDLE_S` (60) ·
  `MP_MAX_S` (3600) · `MP_GRACE_S` (15) · `MP_BUILD` (default: the checkout's commit, none with uncommitted changes). `GET /health` →
  `{game, build, state, tick, seats, watchers, slips, lost, lines}`. Exit 0 when done, 1 when its game could not be stood or threw, 2 on a bad environment.
  By hand: `MP_SETUP='{"search":"?roomclear&squad4","seed":7}' MP_SEATKEYS=k0 bun run tools/mp/server.ts`, then
  `bun run tools/mp/bot.ts --key k0` (a scripted client: `--lag`, `--jitter`, `--after`, `--ticks`).
- **THE CHECK**: `bun run tools/qa/mpserver-check.ts` (a server process and scripted clients over real WebSockets on one machine; about a minute):
  the rule on its own; THE CORES (the server's core and the client's on a made-up clock with a counter for a game: no tick before `ready`, messages that
  are not records, three seconds of sending nothing, a server stalled a second, a late watcher over a socket that drops messages, a watcher too many, a
  client of no named build, whose `lost` counts); stood; refused hellos; SAME FIGHT (a seated client and a watcher, 1200 ticks, every half second's state word the server's, the man
  moved and fired); a late joiner; LAG (a seat 60 to 100 ms away each way); the heartbeats at a stand-in control plane; the server ending itself when
  idle. `MPSERVER_BREAK=1` (the seated client flips the last bit of its own man's place after tick 90) FAILS the two state-word lines, as it must.
  `--setup '{…}'` plays another mission. RUN 2026-11-02 on the cloud box (bun 1.4.4, Linux x64, four cores, nothing seen on a GPU or in a browser): the
  four-man room clear in the office seed 7 and the consulate rescue seed 3, every line PASS; the break word fails 2 lines. A tick is 9 to 15 ms there
  with the server and three clients sharing the four cores, so a scripted client's own beat is starved (it sent 500 to 850 records in 1200 ticks and the
  server held its keys through the rest, and the server let some fifty ticks slip): that is the box, not the wire, and the words still match. With one
  seat per game today the second client is a watcher; two seats wait on the seats work. BY HAND, once: a recipe with `CMDCHECK.on` off is refused; a
  setup naming `constructor`, a knob the build lacks or a null for a number is refused; with the player put down the stood game's `ended()` answers
  `{won: false, end: 'DOWN'}` one tick later. No check plays a mission to its end over the wire.
- **What nobody has seen or measured**: a page as a client; two machines; a real network; a Mac; a server in a sandbox; a second seat; a game longer than
  twenty seconds over the wire; a mission ENDING on a server (the verdict's answer was read by hand, never over the wire); the run's memory on a long
  game (its text is kept for joiners: about 20 KB a second with two seats sending every tick, 80 MB over the longest life); a real socket backing up (the
  check's dropping socket is made up; bun's own `drain` was not seen to fire).
