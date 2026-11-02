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
  not confirmed yet; `unplayed(max)` = what the seat's man has still to play from the confirmed tick on, oldest first: for each tick heard and not walked
  yet the record the server SAID it ran for him (his own, or a held one), then his records on their way, a tick with none holding the last — what the
  look ahead walks (below). NOT BUILT: a resync after a state word differs.
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
  The page's own replay buffer is off on such a page (`noreplay` in its address). `window.__net.state` for a script. A SECOND SEAT: a page keyed into seat
  two shows that seat's man (`attach` sets `Game.localSeat`: his man under the camera, his HUD, his record sampled — the simulation never reads it).
  NOT BUILT: a playout buffer, a resync, reconnecting without a reload, the lobby. KNOWN HOLES: a debugger row that writes to the game (a restage, a
  teleport) is this page's alone and the next state word says so; Shift+Enter and Enter when down are in the record, so they restart the mission for
  everybody — the seats work and the lobby decide what a restart is in co-op.
  THE CHECK: `PORT=<dev server> node tools/qa/mpclient-page.cjs` (headless Chromium on the software adapter against a real server process, a bun watcher
  beside it; about seven minutes: the page's boot, and two frames the look ahead's line draws, a minute each on the software adapter): moved to the game's address with the key off it · the server waits while the page stands
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
- **Your own man answers the key at once: THE LOOK AHEAD** (`src/net/foresee.ts` works out where he is, `src/host/ahead.ts` draws him there,
  `src/game/walkahead.ts` is the walk). THE SWITCH: `FORESEE.on`, off in the module, ON on a networked page (`netFromUrl`), `?netpredict=0` = the page
  as step 1189 had it (his man drawn where the game has him); the scripted client's `--predict`. It has no knob table, no `FROZEN_HOLDS` row and no
  journal key because NO TICK READS IT: both stands and every machine play the same fight whatever it says, and the state words are the proof.
  WHY: a client's game stands at the last confirmed tick, so a key is answered when its tick comes back (the way up, the server's `lead`, the way
  down: 145 to 204 ms on the cloud box's page check). WHAT: at the end of every page tick `Foresee.look` walks the seat's man through
  `NetClient.unplayed(FORESEE.max = 30)` with `walkAhead`, a copy of his legs and his aim that writes nothing of the game (a first cut stepped the real
  man and put him back; that wrote the world when a can was leaving his hand, and two clients were out of step after 400 ticks). ONLY HIS WALK AND HIS
  AIM are ahead: his rounds, doors, orders and throws, his pose, and every other man are the confirmed tick's. Where his legs are not his own
  (`walkAhead`'s `free`, `player.ts legsOwn`: down, a takedown, hands at work, on a wall, a body or a door in his hands, a lesson's gate, and ON THE
  PAGE'S STAND crouched under the tap step and stalking) he is drawn where the game has him, a round trip late as before; so is a page that is catching
  up, has parted from its server, or only watches. A CORRECTION (the server ran a held record because his came late; a
  teammate at his shoulder, whom the copy holds where the tick has him; a door leaf or a loose chair, which the copy does not meet) is never shown at once: where this look has him on the tick the
  last one ended on is the newer word, the difference joins an offset that loses 15% a tick (`ease`), and past `snap` (1.5 m) it is a cut.
  THE PICTURE (`AheadDraw.pose` right before the render and `unpose` right after, laid over the tween's: pose after its, unpose before, the tween's
  factor between the last two ticks' looks): ONE rigid move and turn about his feet of the rig's boxes keyed his (`MKEY.rig` with his id; NOT the owner
  byte, which a tracer in flight and a charge laid on a wall carry too), the lights he carries (his, with no `ttl`: not a muzzle flash, not a tracer's
  head light), his own palette instance (`CharacterRenderer.drawJointsOf`: his matrices alone are written), his capsules and cull sphere in
  `capsuleData` (put back value for value), and where the outline pass looks for him. The camera's look-at is led by the same amount and the aim point
  is the newest record's (`LoopCtx.lead`, `host/loop.ts`; only while the camera is on him, never on a rail or a watched man). The game's own arrays are
  never the moved ones when a tick runs, and the first tick after a draw packs the world again (`AheadDraw.draws`, read in `page.ts walk`): a draw lists
  the boxes as it drew them and the world's ray reads that list. NOT AHEAD, so a round trip late: the HUD's marks of him, the fog cone's origin, his
  tracers, flash and casings (they leave from where the tick has the gun), his footsteps' sound. THE COST: a look walks every record still on its way
  each page tick, most when the page is 30 to 119 ticks behind; 0.7 ms a look in Chromium on the cloud box at about ten records, 0.6 to 2 ms in bun at
  6 to 17. KNOWN DOUBLE: `AheadDraw` is a second pose and unpose beside the tween's (its own restore lists, the same capsule layout); one pose that took a
  rigid move for one man would be one copy, and nobody has folded them.
  A LOOK AT LAG: the next bullet (a worse connection on purpose).
  TWO SEATS OVER THE WIRE: `rawBits` folds every seat's man in seat order (`game/digest.ts rawWalk`; it folded `g.player`, the LOCAL seat's man on a
  page, so two clients of one game differed at the first word). One seat: the word as ever (digests 21 of 21 on the frozen stand).
  THE CHECKS: `mpserver-check.ts`'s `two seats` line (a third server, `?roomclear&squad4&seats=2`, a keyed client in each seat, the first 60 to 100 ms
  away: every state word of 600 ticks, one end word) and `ahead` line (each client's look held against the game when the game gets to that tick: he
  moves on the beat of his first record, and of a client's CLEAN looks — the server ran every record of it as sent, `NetClient.heldRun`, his legs stayed
  his own on every tick it crossed, and nobody stood within 1.5 m of him — half or more are right to a millimetre (a COUNT since 2026-11-02; it was
  'under 5 mm off on average', and one thing a look cannot know, wrong in the ten or twenty looks that cross it, put a run of exact looks over that
  one run in three: the reason is written at the bar, `AHEAD_EXACT`; `bot.ts --why` prints each clean look that missed); the others are corrections,
  counted and held to no bar; a look with his legs not his own walks nothing
  and is counted apart); `MPSERVER_BREAK=1` fails both (the first client flips a bit, the second runs `--no-ahead`). `mpclient-page.cjs`'s `ahead` line (on the frame the first record with a walk key down goes
  out the look has him moved and the game has not; the game moves him some frames later; walking, he is ahead in over half the looks; two frames are
  DRAWN and each moved one palette instance, one set of capsules and his boxes, and every state word after them still matches), `--seats 2` (the page
  in seat two, a bun client in seat one) and `MPCLIENT_BREAK=ahead` (`netpredict=0`: the `ahead` line must fail).
  RUN 2026-11-02 on the cloud box (four cores, bun and headless Chromium on the software adapter; NOTHING SEEN ON A GPU OR BY A PERSON), on the build
  of step 1192: `mpserver-check` every line PASS on both stands. Two seats: 20 of 20 state words each and one end word. Ahead, frozen: beside the
  server 340 looks held against the game, 8.9 mm off on average (307 within a millimetre, 300 mm at most), 254 of them clean and those 0.0 mm off;
  60 to 100 ms away 104 looks, 20.2 mm (423 mm at most), 45 clean and 0.0 mm off, and 144 looks that walked nothing. Shipped: 354 looks at 12.0 mm
  with 121 clean at 0.0 mm, and 302 at 7.8 mm with 50 clean at 0.0 mm. So a look is exact when its premise holds, and what is off is a held record or a
  teammate at his shoulder (seen with a note of what stood near him: every clean miss had an operative 0.6 m away). A look cost 0.6 to 2.4 ms in bun at
  8 to 19 records. Its break word fails four lines (same fight, lag, two seats, ahead). `mpclient-page`, one seat: every line PASS, 30 of 30 state
  words, the game moved him 66 ms after the look ahead did (108 to 402 ms on runs where the box was busier), ahead in 110 of 110 looks (77 cm at most),
  two drawn frames each moved 1 palette instance, 1 set of capsules, 23 boxes and 1 light, and every state word after them matched; no correction past
  a centimetre (24, the longest 203 mm, on a run that fell minutes behind). `--seats 2`: every line PASS, 43 of 43 words, the page shows seat 1's man,
  5 corrections (the longest 17 mm). `MPCLIENT_BREAK=ahead` fails the `ahead` line and no other; `MPCLIENT_BREAK=1` fails same fight and ahead. One
  seat's state word by the new fold against the build's own file: 40 of 40 the same (the room clear seed 7 and the rescue seed 3, 600 ticks each).
  THE CHECK'S OWN LESSONS: a drawn frame can starve this box for a minute (the page then falls thousands of ticks behind), so the two are drawn LAST;
  a record goes out on the client's beat, not on every frame, so the key's line holds the frame the first record with the key went out; the sleeping
  tab's walk goes back over floor he has just walked (it read red twice on a wall at his shoulder).
  WHAT NOBODY HAS SEEN OR PLAYED: any of it on a GPU; a LIVE page against a server (the loop's own frames and the tween between two ticks: the check
  steps frames by script and draws two, both at the tween's factor 1); how a correction feels, in a stack with teammates most of all; the camera's
  lead; the outline and the weapon light riding with him; crouched under the tap step and stalking, where he is not ahead and answers a round trip
  late; `?netlag` by eye. NEXT, if the feel asks for it: the copy walking a crouched man (the tap step's rhythm), teammates carried along by their
  velocity in the copy, his muzzle flash and shot sound shown early.
- **A worse connection on purpose: the debugger's Network page** (2026-11-02; `src/ui/netrows.ts` the rows, `src/net/page.ts NETWAY` the page's
  numbers, `src/net/delay.ts` what they do). WHY: the maintainer, about to try co-op on two laptops on one home network: "can you add something to the
  debug panel to simulate different network conditions?". Two machines on one good network see none of what a real connection does. WHAT: on a page
  that is a client of a networked game (and on no other: `main.ts` makes the page only then) the debugger has a Network page, last on the rail. Rows:
  Connection (a named kind: As it is, Same city, Across the country, Across an ocean, Bad wifi, Phone hotspot, or Your own numbers; `NET_KINDS`, rough
  figures for a home connection and nobody's measurements), Delay each way (ms), Jitter (ms), Loss (%), Stall (two buttons: half a second, and three
  seconds), and Draw your own man ahead (the look ahead's switch, `FORESEE.on`), under a read-out (`netSummary`: the seat, the state, the checks
  matched; the connection asked for and how late the server's messages are really reaching the page, `heldMs`, measured in the page's own queue; the
  inputs on their way and the time from a key to the game's answer; how far ahead his man is drawn and the corrections; the ticks waiting, and how
  early his inputs reach the server, from the server's own pace notes). Every row acts AT ONCE, with no reload. A PAGE'S NUMBERS ARE ITS OWN: each
  player sets them on their own page, and what one player does reaches the other through both pages' ways.
  THE MODEL (`delay.ts`, one table for both directions of a client, `onWay`): every message `lagMs` later and up to `jitterMs` more, through one
  ordered queue a direction, so nothing passes the message in front of it. LOSS IS A HOLD, NEVER A HOLE: the wire is a WebSocket, so a lost packet is
  sent again and the message AND EVERY ONE BEHIND IT wait for that, a round trip and `resendMs` (60) more, as a steady stream's quick resend takes
  (a quiet stream's timer, a quarter of a second and up, is not modelled). What a player feels on a lossy link is short freezes, and that is what the
  row gives. LOSS IS A SHARE OF MESSAGES and there are 60 a second each way: 1.5 in 100 is a freeze about once a second each way, and 3 in 100 had a
  scripted seat's records late by the hundred. A STALL is the queue SHUT until a time (`delayed`'s third argument, asked whenever something could be
  let go): what was already on its way waits too, and a stall taken back (the section's reset) is over within 50 ms. The two buttons sit either side
  of `NETPAGE.far` on purpose: after half a second the page plays the ticks it missed two a frame, after three it catches up as a tab that slept
  does, and each button always shows its own. A close waits behind the messages before it, so a goodbye is read first.
  WHERE A TRY IS KEPT: for the tab (`sessionStorage`, `tps.net.try`, written once a dragged row rests), because joining again is a reload; never
  longer, because a connection left bad by last week's test would be a trap. The address words `?netlag=<ms>&netjitter=<ms>` (1000 at most each),
  `?netloss=<percent>` (20 at most) and `?netpredict=0` set the same numbers and win over what the tab keeps on every load of that address
  (`netFromUrl` carries them to the game's own address, `netLeave` drops them; at the most of all three a hello and its welcome still make the trip
  inside `NETPAGE.helloMs`). The scripted client takes the same table: `tools/mp/bot.ts --lag --jitter --loss --stall`. NO SWITCH, NO KNOB: a debug
  tool of the page, all zero unless asked; the simulation never reads it (the ticks are the server's either way). `window.__net.way`, `.tried`,
  `.try({...})`, `.stall(ms)` are the same things for a script. NOT THERE: a different way up than down; reordering (a WebSocket has none); a
  connection that drops and has to join again; the SERVER's own stalls.
  TWO MACHINES ON ONE NETWORK, by hand until the page has its host and join screen: `bun run tools/mp/lan.ts` starts one game server for a squad
  mission of two on every interface and prints the address to open on each machine. Each machine runs its OWN page (`bun run dev`, the same commit):
  a browser gives WebGPU only to a secure address, and another machine's plain http address is not one, so only the game's socket crosses the network.
  CHECKS: `mpserver-check` lines `worse way` (the queue's rules with a die the check throws: straight through at zero, order kept under jitter, a
  lost message late by the resend and holding the ones behind it, a stall holding all to its end with what was on its way, a stall taken back over
  at once) and `bad connection` (a fourth server, two seats: one seat 15 to 75 ms away each way with 3 in 100 resent and one stall of 1.5 s, one
  beside the server; both match every state word and end on the same one; the stall was felt and the server ran ticks on a held record).
  `mpclient-page` line `network page`: the rows are moved as a hand moves them (the list's change event, the box's, the button's click) on a real
  page in a headless browser: picking Across an ocean sets the numbers, the delay's row follows, the try is kept for the tab, and the server's
  messages really wait that long in the page's queue (`heldMs` from 0 to 70 ms or more); the look ahead's box off draws him where the game has him
  and on again ahead (not asked when he is down by then); the long stall keeps the server unheard for 2.5 s or more and the page catches up; back on
  As it is the wait is gone; every state word after is still the server's. (His inputs on their way are printed and held to no bar: on a scripted
  page that steps ten frames at a go the client's aim past the server is the starved page's own.) `MPCLIENT_BREAK=way` (the check picks As it is
  where it should pick the long one) must fail that line alone.
  RUN 2026-11-02 (the cloud box, the software adapter, the frozen stand unless said): `mpserver-check` every line PASS (14), `worse way` and `bad
  connection` in it: a lost message 210 ms late and the one behind it 206 ms, a 300 ms stall holding message 10 for 301 ms and message 9, already on
  its way, for 305 ms, a 5 s stall taken back letting go after 202 ms; the seat on the bad connection matched 20 of 20 state words, ended on the other
  seat's word, waited 1.6 s for the server at the longest, and of its 228 to 396 records 160 to 196 came late (the box's starved bot, and three
  resends a second). `mpclient-page` one seat, 9 of 9 lines: Across an ocean set 80 ms each way and up to 15 more, the server's messages waited 0 ms
  before and 117 ms then, the box off drew him where the game has him and on again 26 ticks ahead, the three second stall kept the server unheard for
  3011 ms and the page caught up, back on As it is 0 ms; `MPCLIENT_BREAK=way` failed the line (0 ms where 70 were asked). A LIVE PAGE, its own frame
  loop and nothing stepped by a script, opened through the kept page's shell (`?keeppage=1`) against a one seat server: the frame moved to the game's
  own address, took its seat, the server began, the Network page's rows were in the debugger, the first state word matched, no page error; at a
  frame every five seconds on the software adapter it could not keep up, so nothing of PLAY on a live page is seen. NOBODY HAS SEEN: any of this on a
  GPU or with a hand on a row; a live page playing at speed; two machines; `lan.ts` past its own start (its addresses were printed and its server
  stopped; no page joined it); how any named kind feels, or whether its numbers are like the connection it is named for.
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
  server held its keys through the rest, and the server let some fifty ticks slip): that is the box, not the wire, and the words still match. In those
  lines the second client is a watcher; two seats and the look ahead have lines of their own (the look ahead's bullet above). BY HAND, once: a recipe with `CMDCHECK.on` off is refused; a
  setup naming `constructor`, a knob the build lacks or a null for a number is refused; with the player put down the stood game's `ended()` answers
  `{won: false, end: 'DOWN'}` one tick later. No check plays a mission to its end over the wire.
- **What nobody has seen or measured**: a page a person plays as a client (the page check is a script's); two machines; a real network; a Mac; a client through a sandbox's tunnel; two PEOPLE in two seats; the look ahead by eye (its bullet says what); a game longer than
  twenty seconds over the wire; a mission ENDING on a server (the verdict's answer was read by hand, never over the wire); the run's memory on a long
  game (its text is kept for joiners: about 20 KB a second with two seats sending every tick, 80 MB over the longest life); a real socket backing up (the
  check's dropping socket is made up; bun's own `drain` was not seen to fire).
