# Networked play (two-player co-op)

The maintainer's ask (2026-11-02): two-player co-op on squad missions, a game server with authority for each game, clients that predict locally and roll
back, WebSockets, CPU senses on the server. ONE WAY TO START A GAME: the Rust module's, on every machine. A page without `?net=` runs none of this.

- **What travels is the input timeline, never the state** (`src/net/wire.ts`). The game plays one fight to the bit from its start and its seats' records, so the
  server decides which record each seat's tick ran on, tells every client, and sends a state word every `NET.wordEvery` ticks as the alarm. A record is the
  keys, the buttons, the cursor, the view's size and nine pose numbers: about a hundred bytes. Numbers cross as JSON to the bit (`numtext.ts`).
- **The start** (`src/net/start.ts NetStart`: the level's file, the address's words, the die, the seats, the host's commands as base64) rides the welcome, and
  every machine hands it the game's own start (`face_start`, dressed). It comes from whoever asked for the game, so every machine checks it (`startWhy`: a
  level file is a NAME, the words name a mission). No command is sent after the start; letting go is the record's 'Paused' word.
- **The game in the module** (`src/net/modgame.ts NetGame`; `rust/game_face/src/net.rs`): `face_networked` (every seat refereed on every machine, a client's
  own too: the eye and both view matrices are worked out again from the pose), `face_record` (a seat's record as the wire has it; it is no hand: no host
  camera moves), `face_ahead` (the look ahead). `face_step` is asked for nothing of a host's (the saves and a rails level's view are a game of one's). A host
  clears the outs each tick. A server, a scripted client and a page use the one class. A game stands in 0.4 to 1.0 s, a tick with the look ahead is about 6 ms on the 4 core box, a process 230 MB.
- **The words** (`src/net/words.ts`). THE STATE WORD is the alarm: the die's count of draws and the hash of each part of the game's block (where every body, door
  and thing is), folded tick after tick. NARROWER THAN THE GAME: a seat's aim, a man's mind and an order book are not in the block, so a difference there is
  said only once it moves something. Beside it THE WORD OF EVERY PART (`foldWhole` over `face_state_words`, each word tick's alone): a client counts it
  (`watchOk` / `watchOff`) and it is no alarm until real pages on real GPUs have held it. WHICH PART PARTED: `MP_WORDS=<path>` in front of a server or a
  scripted client writes every part's word after each word tick (the alarm's own as `block.<part>`), `mpclient-page --words DIR` the page's, and
  `bun run tools/mp/words-diff.ts <files of ONE game>` names the parts and the machines. A READ BETWEEN TWO TICKS WRITES NOTHING OF THE BLOCK (a page's read
  that kept a memo in it parted the alarm with everything equal, 2026-11-08). What a machine LOOKS AT parts nothing (`rust/game_start/tests/coop.rs
  where_a_machine_looks…`, `a_look_ahead_is_where_the_game_then_has_him`). What a machine SHOWS is no number of the game's either:
  which ear a thunder is in is asked of the shown seat's camera as it sounds, and a thin line (a beam, a tracer, a charge's firing line) is as thick in the game as it is, widened by the
  drawer for the shown seat's view (`game_shown_px`; `each_players_view_is_his_own_pictures_alone`).
- **src/net knows no game**: nothing in it imports `src/game`; hosts are handed WIRE records.
- **The server's authority over input is one rule** (`src/net/timeline.ts SeatLine.take`, asked once a seat a tick): the seat's own record for that tick if
  it came in time; else the newest record the seat is known by with its one-tick edges off (keys and buttons stay held, the view stays); a record that came
  LATE is not lost — it becomes the newest known and its edges (a press, a click, the wheel) land on the next tick; a seat nobody sits in holds nothing.
- **The server's core** (`src/net/server.ts NetServer`; host-neutral: the process hands in its clock, its sockets and the stood game). `waiting` until every
  seat has somebody in it WHOSE CLIENT HAS STOOD ITS GAME and said `ready` (no tick runs: a fight never starts with a man standing still while his page
  loads) → `playing` → `ended` when the game's own verdict says so (every mission kind's end, and all players down),
  then a grace and it is done. A hello carries the wire's version, the game's id, the build and — for a seat — the seat's key; none of a key is a WATCHER,
  sent every tick and sending none (`NET.watchers` at most). A server that names its build lets in no other, an unnamed client included; a checkout with
  uncommitted changes names none (`tools/mp/stand.ts buildName`, as `tools/build.ts`). THE RUN SO FAR is kept as the text of its tape messages (`NET.tapeChunk`
  ticks each) and every connection has a place in it: one that is behind — a joiner, a seat back after a drop, a socket that would not take a tick — is
  sent a stretch on each `advance` and none while its socket is backed up (`drained`), so a hello costs nothing however long the game has run and no tick
  is lost to a slow reader. WHAT A CLIENT SENDS IS CHECKED before it is kept: a message's length (`NET.maxText`) and a record's every field
  (`wire.ts wireOk`: a shape; how many keys and how far a view may be are the referee's). A `lost` is counted from a seat, once. It
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
- **The door** (`src/net/door.ts`): the page's half that knows no game: the address's words, the seat's key (read off the address once and kept for the tab, so
  the address the page is moved to does not carry it), the socket and the connection made worse, the hello to the welcome or the words that refuse it, HOW FAST
  confirmed ticks are walked (`NetDoor.walk`: one a page tick, two while more than `NET.lead` wait; from `NETDOOR.far` waiting it CATCHES UP over ticks nobody
  sees, as many as fit in `NETDOOR.farMs`, the hands off the keys), the look ahead, `state()`, the line the player is shown. A page that has PARTED from the
  server (a state word differed, a hole in the stream) closes its socket, so the server lets go of its seat. NOT BUILT: a playout buffer, a resync,
  reconnecting without a reload.
- **The page as a client** (`src/host/drawnet.ts`; `/?net=<ws address>&netgame=<id>&netkey=<a seat's key>`, no key = a watcher). `netAsked` says hello, takes the
  welcome's start and sends the page to the game's own address with its seed (a page reads its picture off its address); the start is stood as it is said. A
  page tick samples the hand, sends it as a record, walks, and draws; a page keyed into seat two shows that seat's man (`face_show_seat`). THE GAME IS THE
  SERVER'S: a card's Restart, a save and a load are refused in words, commands are dropped (`DrawGame.walked`), no try or best is filed, Quit leaves the page
  (`netLeave`). HANDS OFF: under a card, and when the tab is hidden, one record that holds nothing and carries 'Paused' is sent (`wire.ts restOf`), and the
  confirmed ticks are still walked: the game is not this player's to stop. ANOTHER PLAYER'S GOGGLES: an infrared line is in the game on every machine once any
  player wears his; a page whose own man is bare-eyed leaves them out of what it draws (`rust/game_draw` boxes). `window.__net` for a script: `state`,
  `client`, `man(seat)` (where the game has him, asked of `face_ahead` with no record), `whole()`, `tried`, `try`, `stall`.
- **Your own man answers the key at once: THE LOOK AHEAD** (`src/net/foresee.ts` works out where he is, `face_ahead` is the walk, `rust/game_draw/src/lead.rs`
  draws him there). THE SWITCH: `FORESEE.on`, off in the module, ON on a networked page, `?netpredict=0` = his man drawn where the game has him; the scripted
  client's `--predict`. NO TICK READS IT: the state words are the proof. WHY: a client's game stands at the last confirmed tick, so a key is answered when its
  tick comes back. WHAT: at the end of every page tick the seat's man is walked through `NetClient.unplayed(FORESEE.max = 30)` by the tick's own lines for his
  legs and his aim, on a body of its own, writing nothing. ONLY HIS WALK AND HIS AIM are ahead: his rounds, doors, orders and throws, his pose, and every other
  man are the confirmed tick's. Where his legs are not his own (`free`: down, a takedown, hands at work, on a wall, a body or a door in his hands, crouched
  under the tap step, stalking) he is drawn where the game has him; so is a page that is catching up, has parted, or only watches. A CORRECTION (the server ran a
  held record; a teammate at his shoulder, whom the look holds where the tick has him) is never shown at once: the difference loses 15% a tick (`ease`), and past
  `snap` (1.5 m) it is a cut. THE PICTURE: his man is moved as one body (bones, proxies, what he wears, the lights in his hands, his rim), the camera's look-at
  with him; a page tick with nothing confirmed holds what moves where it stands (`face_draw_hold`), several ticks in one are blended over them all
  (`face_draw_over`). NOT AHEAD, so a round trip late: the HUD's marks of him, the fog cone's origin, his tracers, flash and casings, his footsteps' sound.
  THE BAR of the checks is on a client's CLEAN looks (every record run as sent, his legs his own all the way, nobody within 1.5 m): half or more are right to a
  millimetre (`bot.ts --why` prints each that missed). NEXT, if the feel asks for it: the look walking a crouched man, teammates carried along by their
  velocity, his muzzle flash and shot sound shown early.
- **A worse connection on purpose: the debugger's Network page** (2026-11-02; `src/ui/netrows.ts` the rows, `src/net/door.ts NETWAY` the page's
  numbers, `src/net/delay.ts` what they do). WHY: the maintainer, about to try co-op on two laptops on one home network: "can you add something to the
  debug panel to simulate different network conditions?". Two machines on one good network see none of what a real connection does. WHAT: on a page
  that is a client of a networked game (and on no other: `host/debugger.ts` makes the page only then) the debugger has a Network page, last on the rail. Rows:
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
  of `NETDOOR.far` on purpose: after half a second the page plays the ticks it missed two a frame, after three it catches up as a tab that slept
  does, and each button always shows its own. A close waits behind the messages before it, so a goodbye is read first.
  A HANDLER THAT THROWS loses its own message and strands none behind it (2026-11-03: the drain arms its timer again in a `finally`; before, what was
  behind waited for the next message to wake the queue; one timer a queue, also when a handler puts into its own); `mpserver-check`'s `worse way` line
  throws on the second of four and wants the other three.
  WHERE A TRY IS KEPT: for the tab (`sessionStorage`, `tps.net.try`, written once a dragged row rests), because joining again is a reload; never
  longer, because a connection left bad by last week's test would be a trap. The address words `?netlag=<ms>&netjitter=<ms>` (1000 at most each),
  `?netloss=<percent>` (20 at most) and `?netpredict=0` set the same numbers and win over what the tab keeps on every load of that address
  (`doorFromUrl` carries them to the game's own address, `netLeave` drops them; at the most of all three a hello and its welcome still make the trip
  inside `NETDOOR.helloMs`). The scripted client takes the same table: `tools/mp/bot.ts --lag --jitter --loss --stall`. NO SWITCH, NO KNOB: a debug
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
- **The lobby: host or join a co-op game** (`src/net/lobby.ts` the door, the words and the control plane's calls, no DOM; `src/ui/coop.ts` the screen;
  the control plane is `cloud/convex/games.ts`, `docs/agents/workflow.md` "Co-op's control plane"). THE SWITCH: `COOP.on`, on for a real page, `?coop=0` =
  the page as it was (no Co-op row, a `?join=` on the address not read); off wherever there is no window. No knob table, no `FROZEN_HOLDS` row, no journal
  key: no tick reads it, a game is the server's start whatever it says. A CO-OP GAME HAS ONE DOOR, `Lobby.seat(code)`: whoever comes through it is given
  a seat by the control plane (`games:join`: the first free one, or the one this TAB already holds), waits for the game's server to answer (`starting` →
  `waiting` with an address), and is sent on to the server's own address — `?net=…&netgame=…&netkey=…`, the page client above — with `join=<CODE>` still
  on it (`address.ts NET_JOIN`: carried by `doorFromUrl`, dropped by `netLeave`). The door's ADDRESS is `/?join=<CODE>`, the link a friend opens
  (`coop.ts coopFromUrl`, asked by `host/drawpage.ts`; a code that is not five of the code's letters is refused there, nothing is read out of it). A game's whole
  address pasted into a tab that holds no key for it (`door.ts netKeyed`) goes through the door too, or it would only watch. The title's Host and Join
  go through the same call with no page load of their own (`coop.ts enter`): the one load is the game's. WHO HOLDS A SEAT IS THE TAB (an id in
  sessionStorage, `tps.coop.tab`), not the browser (`cloudPid` is one for every window of a browser: the second window on one machine would be handed
  the first one's seat). ON THE PAGE: the title has a CO-OP row; `ui/title.ts` keeps which page of the title is up (`titleCoop`, beside `titleExtras`) and
  `coopRows` fills the column: 'Host a game', and a field for a code with 'Join' (a wrong code is said under the field). What was typed and the line
  under it are module state (`coop.ts PAGE`): the title is drawn twice, the boot card and then the live title over it, and both show the same. The
  title's key walk stops at the field with the arrows, and a letter typed into a field is the field's (`keymap.ts typedInto`). HOSTING is the one
  starting page in its co-op dress (`ui/startpage.ts StartPageOpts.coop`): only the missions two can play (`coopCan`: a kind the game seats two in —
  `game/seats.ts seatsKind` — not a lesson, not a row built for its own size), the setup rail as ever, no kit step, and its block ('Host game') hands the
  mission's address to `coop.ts host` instead of loading it (one game a press). WHAT THE HOST HANDS OVER (`coopScenario`): that address — level, hour,
  weather, way in, difficulty — without this visit's own words (`VISIT_FLAGS`, the sending words), with `squad=4&seats=2` on it (each player brings two
  teammates) and the difficulty levers this machine keeps written out (the server keeps nothing), a fresh die, and THE HOST'S OWN SETTINGS AS COMMANDS
  (`coop.ts HOSTED.setup`: `machineBefore` with no kit, in the setup's `knobs: {commands}`). A setup with no commands stands on what its address says of the
  difficulty (`tools/mp/stand.ts freshStart`). NOT CARRIED: the kit this machine keeps (both play the kit as issued) and what is turned in the debugger.
  The control plane takes a scenario up to 400 characters (`games.ts MP.scenarioMax`; it was a word of 120). THE HOST'S PAGE LOADS THE MISSION AT ONCE
  and stands in the first seat while the server waits (its rule: no tick until every seat's client has stood its game); `coopInvite` shows the code,
  the link and Copy link until the fight starts, and 'Your friend is joining' once the control plane says the second seat is connected. REFUSALS are
  told in their own words (`lobbyWhy`: full, another version, no such code, no connection); the words are read from the refusal's text, which a
  deployed backend keeps in the error's data, so `src/cloud/wire.ts` folds a string `errorData` into the error's message. WHERE IT RUNS: the page's own
  backend (`cloud.ts cloudUrl`) — on localhost the one `bun run dev` started, whose runner is the dev server (a game server is a child process); on
  the hosted page the hosted one, whose runner starts a sandbox. A page with no backend (`?cloud=0`, a static copy) says so. THE CHECKS:
  `bun run tools/qa/lobby-check.ts` (no page, some fifteen seconds; needs `npm ci` in `cloud/`: the words, the code's letters against the control
  plane's own, a refusal whose words are only in its data, a Hard address standing another game than a Standard one, then a stand-in backend, the
  REAL local runner and the REAL game server — a tab hosts, a second joins by the code, the first again is the first seat again, a third is told it
  is full, a scripted client in each seat walks 240 ticks on the address and key its tab was told; `LOBBY_BREAK=1`, the game made for one player,
  fails host, join and fight) and `PORT=<free> node tools/qa/lobby-page.cjs` (windows of one headless Chromium against a dev server it starts with a
  stand-in backend: a page with no backend (`cloud=0`) whose Host game says why not and leaves what the solo list remembers alone — in the co-op
  dress the starting page's own deploy hook writes nothing, a hosted game is not that list's to remember — and whose refusal said twice stays up; the
  title's row and the host's list, the block makes the game, the invite, the host's window sent to the LINK comes back through
  the door into its own seat, a second window TYPES the code, a third that opens the link is told the game is full, both windows match every state
  word for 300 ticks and each walks its own man; `LOBBY_BREAK=1`, the second window opened with `coop=0`, fails the join and same-fight lines). `mpclient-page` is the two-engine line: a Chromium page against a bun server. RUN 2026-11-08 on the page that draws (the cloud box, headless Chromium on the software adapter; the control plane a stand-in of plain objects, for the backend's packages cannot be had there): every line PASS in a quarter of an hour (both windows 35 of 35 state words, every part of the game on each). `lobby-check` was not run there. NOBODY HAS SEEN OR PLAYED IT: never seen on a GPU (the windows were headless, on the software adapter, frames stepped by
  script and none looked at beyond two stills of the title and the host's list); no two people, no two machines, no real network; the hosted backend
  and a sandbox have never been reached from this screen (the control plane's own checks cover its functions); a deployed backend's redacted refusal
  is a made-up answer in the check, never a real one; the invite card, Copy link and the 'Your friend is joining' line were read from the DOM, not
  looked at. OPEN: a seat claimed at the door is held until the game is over (no `games:leave`: a friend who opens the link and closes the tab keeps
  the seat, and only that tab can come back to it); a restart is still either player's record; a full game's address pasted into a third tab watches,
  as any keyless `?net=` address does; the scenario's 400 characters need the control plane's functions deployed again.
- **Two players, two teams to the eye** (2026-11-03; `src/game/seats.ts seatTone` / `playerName`, `src/ui/findmeviz.ts findMeOutline`, `src/game/floorplan.ts
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
  THE CHECKS, native: `rust/game_start/tests/players.rs a_mans_hue_is_the_same_on_both_screens`, `rings.rs in_a_game_of_two_the_plan…`, `debrief.rs
  two_seats_read_one_run…`. How faint another's plan is drawn is held by nobody. NOBODY HAS SEEN IT: never on a GPU, never by two players; the purple was picked by number, and so was how faint the
  other plan is; a frag ordered by the other player's squad was not thrown in any check.
- **A player's own name** (2026-11-03; `src/host/names.ts`, `src/net/wire.ts cleanName` / `Hello.name` / `State.names`, the field in `src/ui/coop.ts
  coopRows`). The maintainer: "or maybe you can pick your name when you enter multiplayer (and we save it in a cookie)". WHERE HE TYPES IT: the title's
  Co-op page, a field 'Your name' over 'Host a game' (`data-coop="name"`; empty = his number; not a stop of the title's W / S walk, `data-walk="off"`:
  a walk that ended in it would type its next step into the name), kept as he types by the browser (localStorage `twopointsix.name`, the page's way of
  keeping a thing: the service a cookie would do; it is one of the browser's settings, so a build that keeps those with its backend, `cloud/prefs.ts`,
  keeps it there too). A FRIEND WHO COMES BY THE LINK never sees that page: he is his number until he has typed a name there once. HOW IT
  TRAVELS: the page's client says it in its hello (`NetClient who.name`; the page reads `host/myname.ts myName()`), the server keeps it on the seat — kept when he
  leaves, said again by a tab that comes back — and hands every client the seats' names in each `state` message; `NetClient.names`. CLEANED AT BOTH
  ENDS (`cleanName`: text with a letter, a digit or a picture in it, no control characters, no format characters but the two joiners, none of the blank
  letters, spaces single, `NET.nameMax` 12 characters as a reader counts them; anything else is no name) and written as text, never markup. A name
  that would not tell the players apart is not shown (`shownName`: the same one twice, or one that reads as a seat's number). No wire number moved: both fields are optional, and a server lets in one build only. WHAT A PAGE DOES WITH IT: `shownAs` wherever
  a card writes a player's name (the debrief's rows, the death recap's labels and chips), and the host's invite ('Ada is joining.'). IT IS NOT THE
  GAME'S STATE, on purpose: a name reaches each machine at its own moment, so the verdict's tracks and the recap's cast keep 'Player 1' / 'Player 2' to
  the bit and the page writes the picked name over them. Not carried: the control plane knows no name (the lobby's rows have none), a watcher says none,
  the copied summary and a kept replay say the number. NO SWITCH of its own: `?coop=0` takes the field away with the rest of the Co-op page, and a
  page that was told no name reads as it did. THE CHECKS: `mpserver-check`'s `names` line (two scripted clients each say a name, one a dirty
  one; both hear both, as cleaned; under `MPSERVER_BREAK=1` the first says none and the line fails), the server's own cleaning of a hello written by hand too), and `net-check`'s `a card calls a player what he picked`. NOBODY HAS SEEN IT: the field was never looked
  at by eye or on a GPU, two people have not typed two names, and `lobby-check` and `lobby-page` were NOT run with it (this box cannot install the
  backend's packages; `lobby-page` reads the column by `data-coop` words, and the field sits above 'Host a game').
- **The process** (`tools/mp/server.ts`, bun; one process, one game, stood by `tools/mp/stand.ts`). It knows nothing of where it runs. TO A CONTROL PLANE `ended`
  MEANS THIS SERVER IS DONE: from the mission's end until the core is done (the grace, the players' end screen) a heartbeat says `playing` with the outcome
  beside it; `/health` keeps the core's own word. A stop signal or a tick that throws stops the core first: a bye to its clients, a last `ended` heartbeat
  waited for three seconds at most. Environment: `MP_PORT` (5300) · `MP_HOST` (127.0.0.1; a sandbox behind a tunnel says 0.0.0.0) · `MP_GAME` · `MP_SETUP`, THE
  SETUP RECORD a host's page hands the control plane and the control plane hands on: `{scenario` (the mission's page address), `seed`, `knobs?: {commands}`
  (anything else in `knobs` is refused), `seats?`, `seatKeys` (one a seat), `build?`, `game?}`, or `{start, seatKeys}` · `MP_SEATKEYS` (where the record carries
  no keys) · `MP_CONTROL` + `MP_SECRET` (heartbeats: POST `{secret, …health}` every 10 s and at once on a change; an answer of 410 = the control plane has no
  such game and the server stops) · `MP_IDLE_S` (60) · `MP_MAX_S` (3600) · `MP_GRACE_S` (15) · `MP_BUILD` (default: the record's, else the checkout's commit,
  none with uncommitted changes). `GET /health` → `{game, build, state, tick, seats, watchers, slips, lost, lines}`. Exit 0 when done, 1 when its game could
  not be stood or threw, 2 on a bad environment. A HOSTED SERVER NEEDS NO COMPILER: it stands on the checked-in module where that is of its sources. By hand:
  `MP_SETUP='{"scenario":"?roomclear&squad4","seed":7,"seatKeys":["k0"]}' bun run tools/mp/server.ts`, then `bun run tools/mp/bot.ts --key k0` (a scripted
  client: `--lag`, `--jitter`, `--loss`, `--stall`, `--after`, `--ticks`, `--build`, `--predict`).
- **THE CHECKS.** `bun run tools/qa/mpserver-check.ts` (a server process and scripted clients over real WebSockets; about two minutes, read ALONE: it holds a
  60 Hz server and its clients to the wall clock): the rule and the cores on their own, the settings, refused hellos, same fight, a late joiner, lag, gone, two
  seats, names, the look ahead, every part, the worse way and a bad connection, the heartbeats, idle. `MPSERVER_BREAK=1` (one record stepped with another walk
  key after tick 90) fails the state-word lines, `names`, `ahead` and `every part`. `bun run tools/draw/net-check.ts` (20 s: the server's core, a second player
  and the page's half on a check's clock, through stalls, bunched ticks, a restart and a page asleep; `NET_BREAK=1` fails). `PORT=<dev server> node
  tools/qa/mpclient-page.cjs` (headless Chromium against a real server, a bun client beside it; `--seats 2` = the page in the second seat): its lines are in
  its head. `lobby-check` and `lobby-page` above.
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
  always did and are otherwise untouched. Both rows are the WORLD's (`hours.states`, `lighting.dayStates`: math/side.ts `GameSideMap`), so a seat's view
  of a game of two asks the Game's: the day's was a plain table until the evening of 11-05, and under a seat's view the show's day answered the night's
  (settings-check's `hour` line asks both seats' views).
  NO SWITCH, a call for the merge to weigh: with nobody touching Brightness every number is the one it was (the
  42 play-through digests on seed 1, both stands, equal to step 1515's); with it turned, the build before is the bug (a run that turns on the slider),
  and there is no word that brings that back. What a player sees change: nothing at default; with Brightness turned, his men's goggles and lights no
  longer follow his slider (they follow the mission's hour), and an illuminator's flood is brighter or dimmer in his picture with everything else.
  THE RULE THIS BEGINS: no sum of the simulation reads the engine's settings. CHECK (that file is gone: the rule stands in the Rust game, `rust.md`, NO RULE READS A PICTURE'S SETTING): `bun run tools/qa/settings-check.ts` (`--static` = its first three
  lines, also `probes.ts settings-reads` in the journal family): `base` the table equals the picture's defaults · the scan reads its own thirteen forms ·
  `reads` a scan of the simulation's files (src/game, scene, anim, math, sim and render/reach.ts: rollback-check's ground; 331 files, 60 reads, all
  under src/game) for every READ of a setting, each COUNTED on a named list of rows that says PICTURE, CARRIED (journal.ts EnvSnap), TO MOVE or TO JUDGE with a date — one read more than a row counts is red (a second read of a key a file already
  has a row for included), and so is a row that counts more than there are · `hour` a game stood on the
  stand asked: the night's with the picture at a quarter, the hour's own at midday with the picture told the same, the show's day's, the night's again
  each way back · `one run` barracks-four:1 played plain and with the picture turned three ways (`QA_PICTURE='{"exposure":0.5}'` in front of any
  headless tool lays a player's settings over the stand-in's; a key the engine lacks is refused) is one digest, and with the game's own exposure turned
  (`--set '{"ENV":{"exposure":0.5}}'`) another, so the line can go red; its runs are the shipped stand's whichever stand the check is run on (the frozen
  one has no goggles on anybody). `SETTINGS_BREAK=1`: `reads` and `one run` fail. Two older probes now ask the game's own numbers where they asked the
  picture's (nvg-probes `nvg-sight`, irlight-probes). STILL TO MOVE, named on that list: the sky's colours for the hour (hours.ts skyLum),
  `flashlightRadius` (carried, so a co-op page cannot part by it), `debrisCap`, irlight.ts's `indirectScale` (the senses' owner's).
  WHO EMPTIES THE LIST: the multiplayer goal, a mend each on this one's pattern, for every TO MOVE and TO JUDGE row no other owner is named on. NOT ON
  IT, the same fault by another door: game code reads the picture's CAMERA (`engine.viewFocus`: wind.ts, airSim.ts, fire.ts, snowBreath.ts), and two
  players have two cameras. NOT a line of
  mpserver-check: its scripted hands are timed by the wall clock, and only one run in several comes near a tube's threshold (a dimmed client parted 1
  run of 1 on step 1334 and 0 of 4 on 1515; with the game's own number turned on purpose, 1 of 8), so a red path there would not be red. Nobody has
  looked at a picture: no GPU, no page was opened for this.
- **The two-seat rules are held natively** (2026-11-09; `seats-check` and `teams-check`, which stood the TypeScript copy, are gone). `rust/game_start/tests`:
  `players.rs` (each player his own man, hands, pair and order; the team's eyes; a man's hue on both screens; a fight of two one fight on both machines, their
  views unlike; a rescue refuses two), `coop.rs` (what a machine draws, looks at and shows parts nothing; each pair through a locked door; the look ahead),
  `two_seats.rs`, `hosts_agree.rs`, `seats.rs`, and a two-seat line in many a file beside. The old faults of a game of two read as a game of one (bare asks for
  "the player", a seat's module sweeping all the world's men, a budget counted with one player) are the Rust game's, mended there. A fight of two as its
  records: the gate's `seats` rows.
- **What nobody has seen or measured**: two PEOPLE in two seats; two machines; a real network; Host and Join through the hosted service; a client through a
  sandbox's tunnel; the look ahead and a correction by eye on a GPU; a mission ENDING on a server over the wire; the run's memory on a long game (its text is
  kept for joiners: about 20 KB a second with two seats sending every tick); a real socket backing up.

- **Every player on the floor (2026-11-08).** In a game of two the world's lines of the tick run with the first seat standing, so a rule that read `g.player` there answered for the first player alone. `tools/blitz/sweeps/seat-tick.py` lists what they reach. Rounds in flight, the team's being down (`seats::all_down`: down when all are), a defender's foe (`teams::foe_char`), friends in a lane, a place clear of foes, a hole in a wall, a man's senses when another rule runs them (`seats::standing_for`), a man's walk through a door, what a hand at work says, a hand planted on a light switch, a defender's grenade called out, the partner's rows and the go word (each player's own: `pair_orders_waiting(g, seat)`), a camera's cone, a charge's sparks and a blast kill's row in the debrief now go by every player. No switch; a game of one is the same by `hold` (110 of 110). Checks: `rust/game_start/tests/two_seats.rs`, two seeds, each seat in turn, red for the second seat before. Left as it was: a shared line in `game.messages` such as 'a ring airfoil in the ribs' names nobody (the page's HUD does not draw that list), and a runner's picture holds one place for the players. Nobody has played these as two; never seen on a GPU.
