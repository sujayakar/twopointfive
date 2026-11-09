# Networked play and the hosted backend

Two people play one mission together: a game server steps the game, and each player's page steps the same game on the same records. The last
section is what a hosted build keeps on a server, which is also where co-op's games are made. Tapes: `replays.md`. Putting the backend live: `cloud/README.md`.

## What the player gets

- On the title, Co-op holds a field for your name, Host a game, and a field for a code with Join.
- Hosting opens the starting page with only the missions two can play. Its button makes a game and shows a code and a link to send. The
  mission starts once your friend's page has loaded.
- You each play your own man with two teammates of your own, against one enemy force. You see each other's men, rims and squad plans.
- Your own man answers a key at once. Everything else is about a round trip late.
- A co-op game can't be paused, saved or restarted from a card. A card takes your hands off, and the game goes on without you.
- If your page falls behind it catches up. If it loses the connection or goes out of step it says so, and a reload joins you again.

## Where the code is

| file | what it does |
|---|---|
| `src/net/wire.ts` | every message, the numbers (`NET`), a record (`WireCmd`), what a seat holds when no record came |
| `src/net/timeline.ts`, `src/net/server.ts`, `src/net/client.ts` | `SeatLine`: which record a tick runs on. `NetServer`: one game with authority. `NetClient` |
| `src/net/start.ts`, `src/net/words.ts`, `src/net/modgame.ts` | a game's start; the state words; `NetGame`, the module's game on any machine |
| `src/net/door.ts`, `src/net/delay.ts`, `src/net/foresee.ts` | a page's way in: key, socket, pace, its line; a connection made worse; the look ahead |
| `src/net/lobby.ts`, `src/ui/coop.ts`, `src/host/myname.ts` | `Lobby` and its words; the title's rows, hosting, the invite; the picked name |
| `src/host/drawnet.ts`, `src/ui/netrows.ts` | `DrawNet`: the page as a client; the debugger's Network page |
| `tools/mp/server.ts`, `tools/mp/stand.ts`, `tools/mp/runner.ts` | the game server's process; a game with no page; servers as the dev server's children |
| `tools/mp/lan.ts`, `tools/mp/bot.ts`, `tools/mp/words-diff.ts` | a game by hand on your network; a scripted client; which part two machines differ in |
| `seats/`, `cmd/`, `walkahead/`, `state_words.rs` | in `rust/game_rules/src/`: each seat's things; a record and its check; the look's walk; a hash a part |
| `rust/game_face/src/net.rs`, `rust/game_draw/src/lead.rs` | the calls a networked host makes; his man drawn ahead |

`PlayerCmd` is in `rust/game_types/src/cmd/`, `SeatSet` and `SeatRef` in `rust/game_types/src/seats/`, the numbers in `CmdcheckK` and `SeatsK`.

## How it works

**What travels is records, never state.** The game plays the same fight to the bit from its start and its seats' records, so every machine steps
its own copy. A record (`WireCmd`) is what a hand did: keys, buttons, wheel, cursor, the view's size and nine numbers of its pose. The welcome carries
the start (`NetStart`). Messages are JSON over a WebSocket.

**The server decides** (`SeatLine.take`). A tick runs on the seat's own record if it came in time, otherwise on the newest
it knows with the one-tick edges off (`heldOf`). A late record becomes the newest, and its edges land on the next tick. An empty seat holds nothing (`idleOf`).

**States.** `waiting` until every seat has a client that has stood its game and said `Ready`; no tick runs. Then `playing`, then `ended` on the game's
verdict (`face_verdict`), a grace, and done. A connection with no key is a watcher. A joiner or a slow socket is sent the run so far in stretches (`catchUp`).

**A client's world is confirmed ticks only.** `walk` steps the game through what the server said ran. The player's records go ahead of it, each aimed at
a tick worked out from time: the server's tick against this clock, plus `margin`, which `Pace` notes move. A page that hitches skips ticks and never sends into the past.

**On the page.** `netAsked` says hello and moves the page to the game's own address (`movedTo`), because a page reads its picture off its address. The
seat's key comes off the address and is kept for the tab. Each page tick `DrawNet.tick` sends the record, walks, and looks ahead. It walks one tick, two while
more than `NET.lead` wait; from `NETDOOR.far` it catches up over ticks nobody sees. Under a card or in a hidden tab it sends one `restOf` record and goes on walking.

**The look ahead.** `Foresee.look` asks `face_ahead` to walk his man through `NetClient.unplayed` on a copy, writing nothing;
`lead.rs` draws him there. Only his walk and aim are ahead: his rounds, doors, orders and pose, and every other man, are the
confirmed tick's. Where his legs are not his own (`free` false) or he is down, he is drawn where the game has him. A correction eases off by `FORESEE.ease`; past `FORESEE.snap` it is a cut.

**The words keep two machines honest.** The alarm is the state word (`foldWord`): the die's count and the hash of each part of the game's block, chained, sent
every `NET.wordEvery` ticks. It leaves out a seat's aim, a man's mind and an order book, so a difference there rings only once it moves something. The first word
that differs is kept as `bad` and told up as `Lost`; the door closes its socket and the player is told to reload. Beside it goes the word of every part (`foldWhole`
over `state_words`), counted and never an alarm. `MP_WORDS` and `words-diff.ts` name the part.

**A record is checked as it is stepped.** `face_networked` marks every seat `remote`, so `check_cmd` keeps the look-at within `reach` of his man and works the view's
matrices out again from the pose on every machine.

**Seats in the rules.** One seat's own things are the `SeatSet`, which stands on `Game` while a pass is about that seat; the others wait in their seat's row.
`with_seat` changes them over for a pass, `each_seat` passes over all, and a tick stands the first seat whichever this machine shows (`show_seat`).
A reader that holds the game shared takes a `SeatRef` (`seat`, `here`), so it can answer for any seat.

**Each player's own:** his man, hands, lens, aim, what he hovers and works, his book of orders, door tools and wall charges. His teammates: `deal_seats` cuts the
roster into a run a seat, and `in_seat` says whose. His share of what the squad carries (`seat_share`).

**Shared:** the world and its die, defenders, doors, lights, the mission and its verdict. Every player's man is an eye of the fog. A defender's foe is the player he
faces (`faced_seat`). The team is down when all of it is (`all_down`).

**Different on two screens, and in no number of the game:** a thin line's width (`game_shown_px`). Rims and plans are the shown player's first, then
the other's: a hue a seat (`rims.rs`), the other's plan fainter (`plansToDraw`). A picked name reaches each machine at its own moment, so it is no game state: the page
writes it over `player_name` (`shownAs`).

**The lobby.** One door: `Lobby.seat`. `games:join` gives the first free seat or the one this tab holds (`tabId`, so two windows are two players and a reload is the
same one). It waits for the server's address and the page goes to `?net=…&netgame=…&netkey=…`. The link is `?join=<code>`. The host hands over the mission's address with
`squad=4&seats=2` and his difficulty levers (`coopScenario`), a fresh die, and his settings as commands (`HOSTED`). Not carried: his kit (both play the kit as issued).

**The control plane** (`cloud/convex/games.ts`). `games:make` counts a runner's live games and writes the new one in one mutation, so the cap holds. Servers beat to
`/mp/heartbeat`, and `reap` ends a game whose server never answered, went silent, sat idle or grew too old. A runner answers four calls (`cloud/convex/runners.ts`);
a `sweep` is the only way a server is stopped. `localRunner` is the dev server's, `modal.ts` the hosted one.

**A server's environment** (head of `tools/mp/server.ts`): `MP_PORT`, `MP_HOST`, `MP_GAME`, `MP_SETUP`, `MP_SEATKEYS`, `MP_CONTROL`, `MP_SECRET`, `MP_IDLE_S`, `MP_MAX_S`, `MP_GRACE_S`,
`MP_BUILD`, `MP_WORDS`. The dev server's: head of `tools/mp/runner.ts`. The deployment's clocks: `cloud/README.md`.

## Rules that hold

- **Nothing but a record goes up.** No command is sent after the start (`DrawGame.walked` drops them). The cards' Restart and the save key are refused in words. The
  game's own restart key is a record, so it restarts every machine.
- **A read between two ticks writes nothing the words cover.** What a machine looks at or shows must part nothing.
- **What a client sends is checked before it is kept:** length (`NET.maxText`), shape (`wireOk`), then the game's own check. A name is cleaned at both ends
  (`cleanName`) and written as text, never markup. A level file is a name, never a path (`levelName`).
- **One build.** A game is made only on a runner of the page's build. A server that names one lets in no other; a checkout with uncommitted changes names none (`buildName`).
- **Two teams to the eye.** "no outlines for the other player character (maybe we should have yellow + red team and then purple + blue team)". "seeing your teammates
  squad path overlays (but probably fainter than yours) would be great".
- **A name of your own.** "or maybe you can pick your name when you enter multiplayer (and we save it in a cookie)". The browser keeps it (`myName`). One that would
  not tell the players apart is not shown.
- **A worse connection on purpose.** "can you add something to the debug panel to simulate different network conditions?" A page's numbers are its own and last for the
  tab. A lost packet is a hold, never a hole: on a WebSocket the message and every one behind it wait for the resend (`wayMs`).
- **Two kinds seat two:** `roomclear` and `nightbreach` (`src/scene/seatkinds.ts`). `seats_refuse` refuses the others, whose rules are written around one commander.

## To change it

- **A number:** `NET`, `NETDOOR`, `FORESEE`, `COOP`. **A message:** `src/net/wire.ts`, then `NetServer.message` or `NetClient.message`.
- **A new field in a record:** `WireCmd`, `wireOk`, `recordOf`, `recordTo`, and `RECORD` in `rust/game_face/src/net.rs`, all in one order.
- **Something per player:** a field of `SeatSet`, in `swap_set`, `set_of_game`, `SeatRef` and both of its makers. A rule that reads it takes `seat: &SeatRef`.
- **A world rule that says "the player":** ask `men`, `men_up`, `all_down`, `nearest_man` or `faced_man`, or stand the seat with `standing_for`. Add a test to `two_seats.rs`.
- **A mission for two:** take its rules through for two, then add its kind to both `SEAT_KINDS` and `seat_kinds`.
- **Try it on one machine:** `cd cloud && npm ci` once, `bun run dev`, two windows of the page. With no backend, `bun run tools/mp/lan.ts` prints an address for each
  machine. A scripted seat: `bun run tools/mp/bot.ts --key k0`.
- **Page words:** `?netlag=`, `?netjitter=`, `?netloss=`, `?netpredict=0` are the Network page's rows. `?coop=0` takes co-op off the page.

## What holds it

Native tests in `rust/game_start/tests/`:

| file | what it holds |
|---|---|
| `coop.rs` | goggles, bursts, where a machine looks and what it shows part nothing; the look ahead writes nothing |
| `players.rs` | each player his own man, hands, pair and order; one fight on both machines; a rescue refuses two |
| `seats.rs`, `two_seats.rs` | a reader answers for any seat; the world's rules are about both players |
| `hosts_agree.rs` | two machines, each showing its own seat, through grenades and a wall charge: every part alike |

`gate`'s `seats room-clear-four`, `seats compound-four` and `seats depot-four` hold a fight of two as its records; each mission has a `start …, two players` row.

Run by hand (`BY_HAND` in `tools/qa/checks.ts`):

| command | what it holds |
|---|---|
| `bun run tools/qa/mpserver-check.ts` | a server and scripted clients over real sockets: lag, a late joiner, two seats, names, the look ahead |
| `bun run tools/qa/mp-control-check.ts` | the cap, the reaper, the seam, on a stand-in and the local runner |
| `bun run tools/qa/lobby-check.ts` | host and join by code, then a fight of two through a real server |
| `bun run tools/qa/cloud-check.ts` | the backend's functions, the page's side, the reading tool |
| `bun run tools/draw/net-check.ts` | the page's half on a check's clock: stalls, bunched ticks, a restart, a page asleep |

`bun run tools/qa/pages.ts coop` is `mpclient-page.cjs` and `lobby-page.cjs`; its `cloud` group is the backend's pages.

## The hosted page's backend

A Convex project in `cloud/`; the page talks to it over plain `fetch` (`src/cloud/wire.ts`). `cloud/README.md` says how to put it live, run it
locally and read it back. "we should just deploy this on vercel and have a convex backend". A browser is a random
id it keeps (`cloudPid`).

**How a build gets one.** `tools/host/build.sh` runs `convex deploy` when `CONVEX_DEPLOY_KEY` is set and builds the page with `TPS_CLOUD_URL` in it.
A page served from this machine asks its own server (`cloudFind`): "we can make the local build use a local convex deployment".

**What is sent and kept** (`cloud/convex/schema.ts`; each file's head lists the fields):

| kind | file | what | off for a visit |
|---|---|---|---|
| tapes, notes | `src/cloud/tapes.ts`, `src/cloud/notes.ts` | each visit's kept tape; what the player wrote on a replay | `?cloudtapes=0` |
| records | `src/cloud/telemetry.ts` | the machine, the load, how a run ended, frame times, errors, a red card | `?cloudstats=0` |
| settings | `src/cloud/prefs.ts` | the browser's `twopointsix.*` keys, less what belongs to the machine | `?cloudprefs=0` |
| files | `src/cloud/files.ts` | what the page saves: "we should get rid of local downloads entirely" | `?cloudfiles=0` |

**The switch.** `cloudSharing`: the address's word, then the browser's choice (`src/ui/cloudrows.ts`), then `CLOUD.shareDefault`, "on, with a notice". By the default alone a
browser sends nothing until the notice (`CLOUD_NOTE`) has been on its screen.

**With sharing off** no tape, record, setting, file or note goes. A saved file is not downloaded instead: the saver is told it did not go. Co-op still works, because the
lobby asks only for `cloudUrl`. `?cloud=0` is no backend at all for the visit, and co-op then says why it can't start.

## Not seen

- Two people on two machines over a real network; Host and Join through the hosted service. `modal.ts` has never run against Modal.
- The look ahead and a correction judged by two players; how faint the other plan is; the purple, picked by number.
- A mission ending on a server over the wire; a real socket backing up.
- Faults: a seat claimed at the door is held until the game is over, and only that tab can come back to it. No resync after a word differs, and no reconnect
  without a reload. `SEAT_KINDS` is written twice.
