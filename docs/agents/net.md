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
  NOT BUILT: a playout buffer, a resync, reconnecting without a reload. KNOWN HOLES: a debugger row that writes to the game (a restage, a
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
  THE PAIR IS ONE CALL: `AheadDraw.around(a, blended, draw)` poses, runs the host's draw and takes the pose down WHATEVER THREW in either (2026-11-03; the
  pose stood outside the host's try before, so a palette write that threw half way left his boxes and light the moved ones. A LIVE page stops at a throw
  from its frame, `fatalOnce`, so no tick of it ever ran on them: who finds the tick's own now is a scripted page's next step and the record of the crash).
  `pose` and `unpose` are private. Not put back after a throw: his cull sphere where a rig fit is set (the next upload rebuilds it; only the GPU reads it). `bun run tools/qa/aheaddraw-check.ts` (headless, both stands; `AHEADDRAW_BREAK=1` = the pair as it stood, 'a pose that throws' must
  fail): moved inside the draw, put back after it, after a draw that throws and after a pose that throws.
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
  A nap's held key walks him on for seconds, and a key held into a wall takes the hold (`wallPress`): on it he has no look ahead by rule, and a lone key
  within `WALLSNAP.alongDeg` of the wall's line only sidles him, so the drawn frames read red twice with him stuck there (2026-11-03). The check now holds
  the keys nearest square away from the wall first, says so in the line, and a line that still cannot get him ahead names what holds his legs.
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
  A HANDLER THAT THROWS loses its own message and strands none behind it (2026-11-03: the drain arms its timer again in a `finally`; before, what was
  behind waited for the next message to wake the queue; one timer a queue, also when a handler puts into its own); `mpserver-check`'s `worse way` line
  throws on the second of four and wants the other three.
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
  3011 ms and the page caught up, back on As it is 0 ms; `MPCLIENT_BREAK=way` failed the line (0 ms where 70 were asked). AGAIN ON THE BUILD THE LOOK
  AHEAD IS IN (the same day, the box asked early and the rows after the drawn frames): `mpserver-check` every line PASS on both stands (shipped: 134 of
  163 and 87 of 87 clean looks exact); `mpclient-page` one seat and `--seats 2`, 9 of 9 lines each, both drawn frames drew him moved, the server's
  messages waited 97 and 99 ms on Across an ocean and 0 before and after, the box on again 5 and 9 ticks ahead, the long stall 3016 and 3022 ms. A LIVE PAGE, its own frame
  loop and nothing stepped by a script, opened through the kept page's shell (`?keeppage=1`) against a one seat server: the frame moved to the game's
  own address, took its seat, the server began, the Network page's rows were in the debugger, the first state word matched, no page error; at a
  frame every five seconds on the software adapter it could not keep up, so nothing of PLAY on a live page is seen. NOBODY HAS SEEN: any of this on a
  GPU or with a hand on a row; a live page playing at speed; two machines; `lan.ts` past its own start (its addresses were printed and its server
  stopped; no page joined it); how any named kind feels, or whether its numbers are like the connection it is named for.
- **The lobby: host or join a co-op game** (`src/net/lobby.ts` the door, the words and the control plane's calls, no DOM; `src/ui/coop.ts` the screen;
  the control plane is `cloud/convex/games.ts`, `docs/agents/workflow.md` "Co-op's control plane"). THE SWITCH: `COOP.on`, on for a real page, `?coop=0` =
  the page as it was (no Co-op row, a `?join=` on the address not read); off wherever there is no window. No knob table, no `FROZEN_HOLDS` row, no journal
  key: no tick reads it, a game is the server's recipe whatever it says. A CO-OP GAME HAS ONE DOOR, `Lobby.seat(code)`: whoever comes through it is given
  a seat by the control plane (`games:join`: the first free one, or the one this TAB already holds), waits for the game's server to answer (`starting` →
  `waiting` with an address), and is sent on to the server's own address — `?net=…&netgame=…&netkey=…`, the page client above — with `join=<CODE>` still
  on it (`page.ts NET_JOIN`: carried by `netFromUrl`, dropped by `netLeave`). The door's ADDRESS is `/?join=<CODE>`, the link a friend opens
  (`coop.ts coopFromUrl`, asked by `main.ts`; a code that is not five of the code's letters is refused there, nothing is read out of it). A game's whole
  address pasted into a tab that holds no key for it (`page.ts netKeyed`) goes through the door too, or it would only watch. The title's Host and Join
  go through the same call with no page load of their own (`coop.ts enter`): the one load is the game's. WHO HOLDS A SEAT IS THE TAB (an id in
  sessionStorage, `tps.coop.tab`), not the browser (`cloudPid` is one for every window of a browser: the second window on one machine would be handed
  the first one's seat). ON THE PAGE: the title has a CO-OP row; `ui/menu.ts` keeps which page of the title is up (`titleCoop`, beside `titleExtras`) and
  `coopRows` fills the column: 'Host a game', and a field for a code with 'Join' (a wrong code is said under the field). What was typed and the line
  under it are module state (`coop.ts PAGE`): the title is drawn twice, the boot card and then the live title over it, and both show the same. The
  title's key walk stops at the field with the arrows, and a letter typed into a field is the field's (`keymap.ts typedInto`). HOSTING is the one
  starting page in its co-op dress (`ui/startpage.ts StartPageOpts.coop`): only the missions two can play (`coopCan`: a kind the game seats two in —
  `game/seats.ts seatsKind` — not a lesson, not a row built for its own size), the setup rail as ever, no kit step, and its block ('Host game') hands the
  mission's address to `coop.ts host` instead of loading it (one game a press). WHAT THE HOST HANDS OVER (`coopScenario`): that address — level, hour,
  weather, way in, difficulty — without this visit's own words (`VISIT_FLAGS`, the sending words), with `squad=4&seats=2` on it (each player brings two
  teammates: `seats.ts dealSeats`) and the difficulty levers this machine keeps written out (the server keeps nothing), and a fresh die. THE SERVER
  READS THE ADDRESS'S DIFFICULTY (`tools/mp/stand.ts freshRecipe` calls `initDifficulty` as the page does at boot; before this a host's Hard was the
  server's Standard). NOT CARRIED: the kit this machine keeps (both play the kit as issued) and knob tables changed in the debugger (Team feel rows).
  The control plane takes a scenario up to 400 characters (`games.ts MP.scenarioMax`; it was a word of 120). THE HOST'S PAGE LOADS THE MISSION AT ONCE
  and stands in the first seat while the server waits (its rule: no tick until every seat's client has stood its game); `coopInvite` shows the code,
  the link and Copy link until the fight starts, and 'Your friend is joining' once the control plane says the second seat is connected. REFUSALS are
  told in their own words (`lobbyWhy`: full, another version, no such code, no connection); the words are read from the refusal's text, which a
  deployed backend keeps in the error's data, so `src/cloud/wire.ts` folds a string `errorData` into the error's message. WHERE IT RUNS: the page's own
  backend (`cloud.ts cloudUrl`) — on localhost the one `bun run dev` started, whose runner is the dev server (a game server is a child process); on
  the hosted page the hosted one, whose runner starts a sandbox. A page with no backend (`?cloud=0`, a static copy) says so. THE CHECKS:
  `bun run tools/qa/lobby-check.ts` (no page, some fifteen seconds; needs `npm ci` in `cloud/`: the words, the code's letters against the control
  plane's own, a refusal whose words are only in its data, a Hard address standing another recipe than a Standard one, then a stand-in backend, the
  REAL local runner and the REAL game server — a tab hosts, a second joins by the code, the first again is the first seat again, a third is told it
  is full, a scripted client in each seat walks 240 ticks on the address and key its tab was told; `LOBBY_BREAK=1`, the game made for one player,
  fails host, join and fight) and `PORT=<free> node tools/qa/lobby-page.cjs` (windows of one headless Chromium against a dev server it starts with a
  stand-in backend: a page with no backend (`cloud=0`) whose Host game says why not and leaves what the solo list remembers alone — in the co-op
  dress the starting page's own deploy hook writes nothing, a hosted game is not that list's to remember — and whose refusal said twice stays up; the
  title's row and the host's list, the block makes the game, the invite, the host's window sent to the LINK comes back through
  the door into its own seat, a second window TYPES the code, a third that opens the link is told the game is full, both windows match every state
  word for 300 ticks and each walks its own man; `LOBBY_BREAK=1`, the second window opened with `coop=0`, fails the join and same-fight lines). No
  stand applies: the page is the shipped stand and the server stands the recipe's. (`tools/qa/coop-check.ts` is another thing: the two-engine
  determinism check.) RUN 2026-11-02 on the cloud box (headless Chromium on the software adapter): `lobby-check` every line PASS and its break word fails three; `lobby-page` every line PASS in some twenty minutes (both windows walked 22730 ticks and matched 757 of 757 state words, each walked its own man) with no page error, and its break word fails the join and same-fight lines; `mpserver-check` on both stands and `mp-control-check` pass. NOBODY HAS SEEN OR PLAYED IT: never seen on a GPU (the windows were headless, on the software adapter, frames stepped by
  script and none looked at beyond two stills of the title and the host's list); no two people, no two machines, no real network; the hosted backend
  and a sandbox have never been reached from this screen (the control plane's own checks cover its functions); a deployed backend's redacted refusal
  is a made-up answer in the check, never a real one; the invite card, Copy link and the 'Your friend is joining' line were read from the DOM, not
  looked at. OPEN: a seat claimed at the door is held until the game is over (no `games:leave`: a friend who opens the link and closes the tab keeps
  the seat, and only that tab can come back to it); a restart is still either player's record; a full game's address pasted into a third tab watches,
  as any keyless `?net=` address does; the scenario's 400 characters need the control plane's functions deployed again.
- **Two players, two teams to the eye** (2026-11-03; `src/game/seats.ts seatTone` / `playerName`, `src/ui/findme.ts findMeOutline`, `src/game/floorplan.ts
  PlanFor` / `PLAN.others`, `src/ui/planfloor.ts plansToDraw`, `src/game/verdict.ts roster`, `src/game/deathrecap.ts manOf`). The maintainer, from the
  first game of two anybody played (two windows, his own GPU): "works on my m4! overall looks great. no outlines for the other player character (maybe
  we should have yellow + red team and then purple + blue team)", "seeing your teammates squad path overlays (but probably fainter than yours) would be
  great", "at least have numbers for players or maybe you can pick your name when you enter multiplayer (and we save it in a cookie)". Before this a
  page showed its own seat's team only: the rims were the shown player's and his order book's men, the plan on the floor his book's (asked of the Game
  itself, so a seat's own squad's throws were not on it either), and the debrief called every human 'Player'.
  THE RULE IS THE SEAT'S AND IS THE SAME ON EVERY SCREEN: a player's own man has his seat's tone (`SEAT_TONES`: the first seat's 'you', the yellow rim a
  game of one always had; the second's 'purple'), a teammate his pair's as ever. The four are dealt a pair a seat (`dealSeats`), so two seats read as a
  yellow man with the red pair and a purple man with the blue pair, and the second player sees HIS OWN man purple (a call made here, said to him; a
  squad with no pairs is blue for both seats, as its callsigns are).
  THE RIMS: `findMeOutline` rims the shown player, then every other player who is up and `teamMen` (every seat's roster), each by his own light, at
  the teammates' row's width; `OUTLINE_INK.purple`. The frame is off when EVERY player is down (`allDown`: a game of one as it was); a shown player
  who is down while another is up has no rim of his own and the team's as ever.
  THE PLANS: `plansToDraw` is the shown seat's `floorPlan`, asked of its VIEW, and then every other seat's, asked of that seat's view as another's
  (`PlanFor { own: false, reading }`) on a store of its own: its men, ordered throws, waits and laid orders, and nothing that is its own player's
  alone (no ring for his open menu, no throw he is only offered, no key on a door or a laid order, none marked as what the key would send). It shows
  in full while EITHER its player or the looker has the order menu up, and otherwise as orders just given, going out with their age (`PLAN.menuOnly`).
  A wait both plans list (the drill's door is the world's; two pairs stacked on one door) is on a page once, as its own with its key. The overlay
  draws another's at `PLAN.others` (0.45, a guess; 0 = not asked: the page as it was) — all but a throw, whose arc and reach are as strong on every
  page. A MAN'S COLOUR ON THE FLOOR IS HIS PAIR'S WHEREVER THE TEAM IS IN PAIRS (`floorPlan toneOf` asks `teamPaired`, not the seat's own book, which
  holds one pair): so in a game of two a page's own plan is its pair's red or blue, no longer the HUD's colour, and the other's is the other pair's.
  THE NAMES: `playerName` is 'Player' in a game of one and 'Player 1', 'Player 2' in a game of two (the debrief's rows, in the seat's tone; the death
  recap's cast; the engagement record, which filed the second player's kills under a bare '#3'). Still 'Player' for both: the lines that are handed
  the word 'player' and no man (`defuse.ts`, `clearline.ts`, `coverfire.ts`, `picture.ts`, `snatch.ts`) and the replay file's cast. A name a player
  PICKS is the bullet after this one.
  NO SWITCH, a call for the merge to weigh: a game of one seat asks none of it (one plan, 'Player', the four rimmed as they were; `findme-check` and
  `plan-check` as before on both stands), and a game of two has no off word of its own (`seats=2`). `PLAN.others` is a key of a registered table.
  The rims and the plans are the page's picture and write nothing of the game's; the names and tones are in the verdict's tracks, so a game of two's
  STATE WORD is not the one the build before gave it (every machine of a game is one build: the server turns another away).
  THE CHECK: `bun run tools/qa/teams-check.ts` on both stands (lines `rims`, `both`, `down`, `plans`, `faint` — on what the overlay itself writes, on a
  page stood in for — `once`, `names`, `fog` (a flash by either seat's own man is the team's under the fog of war: what a wall charge's blast hands the
  fog is its seat's man; the afterglow light, `BLAST.glow`, is off in every build, so no charge is blown to light one), `one`, `pure`); `TEAMS_BREAK=1` (two seats stood, every seat given the first seat's tone, another's plan put to
  none, the page handed no names, the fog's row left off) fails `rims`, `both`, `plans`, `faint`, `once`, `names`, `picked` and `fog`. RUN 2026-11-03 on the cloud box, one seed: every line PASS on both stands, the
  break word fails those six. NOBODY HAS SEEN IT: never on a GPU, never by two players; the purple was picked by number, and so was how faint the
  other plan is; a frag ordered by the other player's squad was not thrown in any check.
- **A player's own name** (2026-11-03; `src/host/names.ts`, `src/net/wire.ts cleanName` / `Hello.name` / `State.names`, the field in `src/ui/coop.ts
  coopRows`). The maintainer: "or maybe you can pick your name when you enter multiplayer (and we save it in a cookie)". WHERE HE TYPES IT: the title's
  Co-op page, a field 'Your name' over 'Host a game' (`data-coop="name"`; empty = his number; not a stop of the title's W / S walk, `data-walk="off"`:
  a walk that ended in it would type its next step into the name), kept as he types by the browser (localStorage `twopointsix.name`, the page's way of
  keeping a thing: the service a cookie would do; it is one of the browser's settings, so a build that keeps those with its backend, `cloud/prefs.ts`,
  keeps it there too). A FRIEND WHO COMES BY THE LINK never sees that page: he is his number until he has typed a name there once. HOW IT
  TRAVELS: the page's client says it in its hello (`NetClient who.name`; `net/page.ts` reads `myName()`), the server keeps it on the seat — kept when he
  leaves, said again by a tab that comes back — and hands every client the seats' names in each `state` message; `NetClient.names`. CLEANED AT BOTH
  ENDS (`cleanName`: text with a letter, a digit or a picture in it, no control characters, no format characters but the two joiners, none of the blank
  letters, spaces single, `NET.nameMax` 12 characters as a reader counts them; anything else is no name) and written as text, never markup. A name
  that would not tell the players apart is not shown (`shownName`: the same one twice, or one that reads as a seat's number). No wire number moved: both fields are optional, and a server lets in one build only. WHAT A PAGE DOES WITH IT: `setSeatNames` (the
  page's own table by the game) and `shownName(g, id, was)` wherever a page writes a player's name: the debrief's rows (`ui/verdictCard.ts`), the death
  recap's labels and chips (`ui/recap.ts`, a `nameOf` handed to `drawRecap` / `recapLabels`), the host's invite ('Ada is joining.'). IT IS NOT THE
  GAME'S STATE, on purpose: a name reaches each machine at its own moment, so the verdict's tracks and the recap's cast keep 'Player 1' / 'Player 2' to
  the bit and the page writes the picked name over them. Not carried: the control plane knows no name (the lobby's rows have none), a watcher says none,
  the copied summary and a kept replay say the number. NO SWITCH of its own: `?coop=0` takes the field away with the rest of the Co-op page, and a
  page that was told no name reads as it did. THE CHECKS: `mpserver-check`'s `names` line (two scripted clients each say a name, one a dirty
  one; both hear both, as cleaned; under `MPSERVER_BREAK=1` the first says none and the line fails) and `teams-check`'s `picked` (the cleaning, by the server too from a hello written by hand; a name handed to the
  page is what `shownName` says, the other player keeps his number, a teammate his callsign, the verdict's own row and the state word do not move;
  under `TEAMS_BREAK=1` the page is handed none). RUN 2026-11-03 on the cloud box: both on both stands. NOBODY HAS SEEN IT: the field was never looked
  at by eye or on a GPU, two people have not typed two names, and `lobby-check` and `lobby-page` were NOT run with it (this box cannot install the
  backend's packages; `lobby-page` reads the column by `data-coop` words, and the field sits above 'Host a game').
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
- **A player's Brightness is his picture's alone: THE GAME'S OWN NUMBERS** (2026-11-05; `src/game/env.ts`: the knob table `ENV` and `tubeGain()`;
  `hours.ts exposureOf(g)`; read by `nvg.ts tubeVis` / `tubeSig` and `irlight.ts`). What a goggled man's tube makes of a dark floor, and how strong his
  illuminator is, were worked out at `engine.settings.exposure` and `settings.nvGain`: the PICTURE's numbers, which a player moves (Options > Display >
  Brightness, the bracket keys, the debugger's two sliders), which no recipe, recording or save ever carried, and of which two players have two and a
  server its own. Found by the review of 11-05, then seen through a real server on step 1334: one scripted client of a two seat night game at exposure
  0.5 (the slider's end) had its first different state word at tick 239 and lost its seat. Now the game has its own. `ENV` (registered in knobs.ts and
  on JOURNAL_KNOBS: a recipe, a recording and a server's setup carry it; a file from before the table gets the build's back, `envAsBuilt`) is the
  night's exposure and the tube's gain, equal to the picture's defaults. `exposureOf(g)` is the exposure the game's scene is taken at now: the show's
  day's while that is in, else the hour's own, else the night's — WORKED OUT from what the game already keeps (which hour is in, whether the day is),
  so there is no second copy to fall out of step and nothing new for a snapshot or a save to carry; hours.ts and lighting.ts tell the picture as they
  always did and are otherwise untouched. NO SWITCH, a call for the merge to weigh: with nobody touching Brightness every number is the one it was (the
  42 play-through digests on seed 1, both stands, equal to step 1515's); with it turned, the build before is the bug (a run that turns on the slider),
  and there is no word that brings that back. What a player sees change: nothing at default; with Brightness turned, his men's goggles and lights no
  longer follow his slider (they follow the mission's hour), and an illuminator's flood is brighter or dimmer in his picture with everything else.
  THE RULE THIS BEGINS: no sum of the simulation reads the engine's settings. CHECK: `bun run tools/qa/settings-check.ts` (`--static` = its first three
  lines, also `probes.ts settings-reads` in the journal family): `base` the table equals the picture's defaults · the scan reads its own thirteen forms ·
  `reads` a scan of the simulation's files (src/game, scene, anim, math, sim and render/reach.ts: rollback-check's ground; 331 files, 60 reads, all
  under src/game) for every READ of a setting, each COUNTED on a named list of 32 rows that says PICTURE, CARRIED (journal.ts EnvSnap, laid over a
  networked page before every tick), TO MOVE or TO JUDGE with a date — one read more than a row counts is red (a second read of a key a file already
  has a row for included), and so is a row that counts more than there are · `hour` a game stood on the
  stand asked: the night's with the picture at a quarter, the hour's own at midday with the picture told the same, the show's day's, the night's again
  each way back · `one run` barracks-four:1 played plain and with the picture turned three ways (`QA_PICTURE='{"exposure":0.5}'` in front of any
  headless tool lays a player's settings over the stand-in's; a key the engine lacks is refused) is one digest, and with the game's own exposure turned
  (`--set '{"ENV":{"exposure":0.5}}'`) another, so the line can go red; its runs are the shipped stand's whichever stand the check is run on (the frozen
  one has no goggles on anybody). `SETTINGS_BREAK=1`: `reads` and `one run` fail. Two older probes now ask the game's own numbers where they asked the
  picture's (nvg-probes `nvg-sight`, irlight-probes). STILL TO MOVE, named on that list: the sky's colours for the hour (hours.ts skyLum),
  `emissiveScale` and `flashlightRadius` (carried, so a co-op page cannot part by them), `debrisCap`, irlight.ts's `indirectScale` (the senses' owner's).
  WHO EMPTIES THE LIST: the multiplayer goal, a mend each on this one's pattern, for every TO MOVE and TO JUDGE row no other owner is named on. NOT ON
  IT, the same fault by another door: game code reads the picture's CAMERA (`engine.viewFocus`: wind.ts, airSim.ts, fire.ts, snowBreath.ts), and two
  players have two cameras. NOT a line of
  mpserver-check: its scripted hands are timed by the wall clock, and only one run in several comes near a tube's threshold (a dimmed client parted 1
  run of 1 on step 1334 and 0 of 4 on 1515; with the game's own number turned on purpose, 1 of 8), so a red path there would not be red. Nobody has
  looked at a picture: no GPU, no page was opened for this.
- **What nobody has seen or measured**: a page a person plays as a client (the page check is a script's); two machines; a real network; a Mac; a client through a sandbox's tunnel; two PEOPLE in two seats; the look ahead by eye (its bullet says what); a game longer than
  twenty seconds over the wire; a mission ENDING on a server (the verdict's answer was read by hand, never over the wire); the run's memory on a long
  game (its text is kept for joiners: about 20 KB a second with two seats sending every tick, 80 MB over the longest life); a real socket backing up (the
  check's dropping socket is made up; bun's own `drain` was not seen to fire).
