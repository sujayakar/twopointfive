# The deletion of the TypeScript tick

The Rust game (`rust/game`) is the game that steps. The TypeScript `Game` stays as the puppet that the picture, the HUD, the menus and the tools read,
and as the code that stands a game before it crosses whole into Rust. Everything else of the simulation's TypeScript is taken out by a program,
`tools/blitz/delete-tick.ts`, which can be run again on any later tree that still has the tick.

```sh
bun run tools/blitz/starts.ts --table             # the table of starts, from the game's lists as they are now
tools/blitz/delete-census.sh                      # the census: hours of plays on a copy of the tree → tools/blitz/delete-census.json
bun run tools/blitz/delete-tick.ts                # the plan on this tree, nothing written
bun run tools/blitz/delete-tick.ts --why Class.method      # the chain of references that keeps a function
bun run tools/blitz/delete-tick.ts --note         # this file's tables, and delete-plan.tsv (a row a function: its class and what keeps it)
bun run tools/blitz/delete-tick.ts --write        # the deletion, in place
bun run typecheck && bun run typecheck:qa         # both must pass on the tree it leaves
bun run tools/blitz/delete-page.ts --at http://localhost:<port> --draw 4 --size 640x360     # the page driven under the Rust game (software adapter)
```

The program refuses a tree it has already been through (`src/game/crossing.ts` is there), a census taken on another tree (`--stale-census` carries
it by file and function name: the census only adds, so a function it has no word on is simply not "seen"), and a call table (`calls.json`) whose
guards it cannot find by key. This tree IS one it has been through: to run it again, check out the commit before the deletion's.

## How it decides

By reachability, read from the TypeScript checker over `src/` and `tools/` together. Nothing is classed by its name or by a list kept by hand.
The simulation's folders are `src/game`, `src/anim`, `src/math`, `src/sim` and `src/scene`: only there does it delete.

**What stays reachable, and how far each root reaches:**

| root | what it is | what a function it reaches keeps |
|---|---|---|
| the page | every file of `src/` outside the simulation's folders: the game's page, and the pages with no Rust game (the workbench with its viewer and editor, the replay viewer, the sound and music pages) | its body, and everything that body reaches. The page has no call surface: what it calls on the puppet runs as TypeScript |
| the hosts | the files of `tools/` that a page or a host loads, found by following imports from the entry points in `HOST_ENTRIES` | its body. They run inside the host's own lines (a start, the apply, the out events, the dress step), where nothing crosses |
| the module | a simulation file's own top-level statements, and the names the bridge's schema reads through its window | all of it |
| a file a tool writes | a simulation file whose first line says a tool writes it, never a hand (`src/sim/built.ts`, `src/sim/men/cols.ts`) | all of it: its own check reads the file's shape |
| a tool | every other file of `tools/` (the checks) | with a row on the call surface: its signature, and the body becomes the crossing. The tick's own function with no row: its signature, which throws by name. Any other function with no row: its body |
| the census | a function seen to run as TypeScript while the Rust game stepped | its body, however it was reached |
| `Game.stepSeats` | the tick | nothing: its body goes, and a host hangs its own step on the game object |

**Nothing the page can reach is deleted on the census's word.** The census plays ticks, starts and a few flows of the page. It has not seen every draw
or every panel, and no headless check here draws by default, so "never seen to run" is no proof for code a draw can reach. What the page and the
simulation's own kept code reach is decided by the checker alone. The census ADDS (what it saw run stays, whatever the checker could follow: a
computed key, a function kept in a table), and it settles one thing only: a host's loose names, below.

**Exact references.** A reference the checker resolves to a declaration. A member of an interface, for every class that says it implements it. An
object handed where another type is asked for: the members that type names are uses of the class's (`loop({ demo })` hands a `Demo` where a `DemoLike`
is asked for). A cast, which is a flow whatever the checker thinks of the two types: `g.destr as unknown as { doorsFor?(…): … }` is a use of the
class's `doorsFor`, a private member too. A name read through a cast, or off a constant that was given one, is read by the type under the cast
(`const fog = g.fogOfWar as unknown as Record<string, any>; fog.cellSeenAt?.(…)`). A name read off a value typed `any` through two or more steps
that are a path from the `Game` class (`P.g.cues.stats`). A name read off a module loaded by a written path (``await import(`${ROOT}/src/game/x.ts`)``,
which is how the tools load the game).

**Loose references**, and what each holds by who makes it:

| who reads | a name off a value typed `any` | a member of a type a class answers only by its shape |
|---|---|---|
| the page, the simulation's kept code | the classes' members of that name, where the read could RUN one: a call, a `.call` / `.bind`, a test for it (`x.f?.()`, `typeof x.f === 'function'`, `x.f && …`). A read that only looks at a value (`o.round !== undefined`, `readAccessor(smp.input)`) is a read of data. A module's own function is never a property of an object | that class's member, where an untyped value is handed to the type somewhere outside the tools (`host.skin: unknown`, then `skin as Partial<TweenSkin>`), or to a type that is handed on to it. Elsewhere the object got there at a place the checker types on both sides, and the reference is exact |
| a host's own lines (the bridge's files read the puppet by name through `any` on every tick of every play) | THE CENSUS: a candidate it saw run keeps its body; one it never saw keeps at most a crossing. A read that tests for the function holds it regardless: a branch quietly not taken is seen by nobody | the same |
| a tool | nothing but a crossing where there is a row. The tool is listed below | the same |

What the checker cannot follow at all, and so what the census and the gates are for: an element read by a computed key (`obj[key]()`), a method read
bare off an untyped value and called later, and a tool's own flows.

`--census-settles-all` lets the census settle the page's loose references as it does a host's. It deletes more and rests on the census having
played every path that makes such a call; it is not the default, and the gates below would have to be drawn on a real GPU to carry it.

## The gate of starts

A start is the one place where the simulation's TypeScript still runs as the game: the game is stood in TypeScript and crosses whole afterwards. What a
start runs is reached by the checker from the page and the hosts; the table of starts is the check that it was, on every way a game can be started,
and the census's widest play. So the census does not sample starts, it stands a TABLE of them:
`tools/blitz/starts.json`, written by `tools/blitz/starts.ts --table` from the game's own lists, never by hand. For every mission of
`src/scene/scenarioList.ts` and the plain night: the mission as its row gives it; each squad size from 0 to 4; each weather kind; each hour; Standard
and Hard; each rung of each lever, the fog lever's bottom rung and the three words under the switch; one mixed row; and three deeds after the first
second: the pause card's restart, a quit to the title with a second start in the same process, and a checkpoint written and put back. Each row is a
process of its own (a page is a load a start), stood by the lines of `src/main.ts` that a start runs, in their order, and stepped one second under
the Rust game.

```sh
STARTS_ROOT=<the census's tree> TPS_RAN_OUT=ran.jsonl bun run tools/blitz/starts.ts --json before.json    # counted: part of the census
STARTS_ROOT=<the deleted tree>  bun run tools/blitz/starts.ts --against before.json                      # THE GATE
```

On the deleted tree the gate is red on any row that ends another way than it did before the deletion, on any throw that says a function has no
TypeScript body, and on any row that stepped a game with no host of the Rust game on it. A new way to start a game (a mission, a weather kind, a
lever) is in the table the next time it is written, because the table is the lists. What the table cannot hold is a start only a page can stand: the
Team & kit page's own small game (`?kitview`) is `tools/blitz/delete-page.ts`'s.

## The page, driven

`tools/blitz/delete-page.ts` plays the page in headless Chromium on the software adapter: the plain night from its start card, a squad mission by its
address, the title by real key presses to Deploy and the mission that loads, the Team & kit page's small game, and a load of every other page. On the
census's tree it is counted like any other play; on the deleted tree it must end with no error said by any page. `--draw N` draws the last frame of
every Nth burst of thirty: what only a draw runs (the smoke's solver, the picture's own reads of the puppet) is run nowhere else, by no headless tool,
so both the census's plays and the check after the deletion are made with it. A drawn frame takes the software adapter half a minute and the first one
minutes. NOTHING HERE HAS BEEN SEEN ON A GPU: the frames drawn hold that the draw's code runs and says no error, not that the picture is right.

**What a miss looks like.** A function the census had no play for, reached after the deletion, throws by name: `X: only the Rust game has this rule,
and this call did not cross`, or a plain `is not a function` for a name that went. Nothing fails silently into a second game. The mend is a row in
`tools/blitz/delete-keep.json` (`[{ "file", "key", "why" }]`), on the tree before the deletion, and the program run again.

## What it writes

- **A function that goes** is removed with the comment that sits on it. Imports and export lists that only it used go with it.
- **A crossing** keeps its export, its signature and its default values, and its body is one line:
  `return crossToRust('DoorTools.order', this, [o, who]) as any`. Where the source left the answer's type to the body, the checker's type is written.
- **A kept function on the surface** opens with the surface's guard as a line of its own source. `tools/blitz/expose.ts` used to write that line as the
  file loaded, at a position kept in `calls.json`; every edit here moves those positions, so the program empties the table's `files`, `taps` and
  `counted` and the lines live in the source. A tapped function keeps its tap's line the same way.
- **`src/game/crossing.ts`**: what the stubs call. It names no other module. No surface installed (the page): a guard passes, a tap never fires, a
  crossing throws by name.
- **`tools/blitz/crossing-keys.ts`**: the surface's rows and taps by key, read from `calls.json` and `taps.json`, handed to `crossing.ts`. It answers
  through whatever `globalThis.__tpsCall` is at the moment of the call, as the old lines did. The bridge's preload loads it.

## What the deletion changes outside the simulation's folders

Each is anchored on one line of the file as it stood, and the program stops with that line's text if the file no longer holds it.

- `tools/blitz/preload.ts` loads `crossing-keys.ts` after `expose.ts`.
- `tools/blitz/calls.json` loses its position tables (`files`, `taps`, `counted`); the rows stay. Nothing may write positions into it again: the
  guard and tap lines are source now.
- `src/main.ts`: the Rust game is the page's game whatever the address says (no `?rust` word). An address with `?net=`, `?join=` or `?playtape=`
  (a co-op game hosted or joined, a tape played back: each stepped the TypeScript tick) gets one plain card before anything boots, until its own
  path onto the Rust game is here. A browser with no WebAssembly gets the card it already got.
- `tools/blitz/word.ts`: `DEFAULT_WORD` is `'rust'`. A headless tool's `standUp` stands the Rust game with no word in front. The words `ts`,
  `audit` and `table`, and the table of checks kept on the TypeScript tick (`qa-kinds.json`), name a game that is gone: their removal is the
  hosts' cleanup.
- `tools/qa/rollback-exempt.ts` gets the row of the stubs' own module variable.

## What it leaves for the hosts and the checks

- THE PAGE HAS NO SURFACE OF THIS MODULE'S. `src/game/crossing.ts` is handed its surface by `tools/blitz/crossing-keys.ts`, which only a tool's
  preload loads. On the page a kept function's guard passes and its TypeScript runs, as before; a CROSSING reached on the page throws by name.
  By a reference the checker follows the page reaches none; the page's host can hand the surface in (`setSurface`) if a menu's call should cross.
- Machinery that stepped the TypeScript tick and has no step now: `src/net/step.ts` (co-op's own step), the tape and journal replays, the
  frozen stand (`TPS_PROFILE=frozen`), the compare tools of `tools/blitz/` that stepped both games side by side. They are not removed here.
- The checks: `delete-lost.tsv`, below.

<!-- the tables below are written by tools/blitz/delete-tick.ts --note -->

## The plan on this tree

Tree `43d8a9cf4be7d603` (a hash of the simulation's files). Census: 2183 functions seen to run as TypeScript in 1539 plays of the Rust game.

The simulation's folders hold 126856 lines before and 97024 after (29832 fewer). Of their 8383 functions (67040 lines of them):

| class | functions | lines | what becomes of it |
|---|---|---|---|
| tick only | 2104 | 22672 | removed: nothing left standing reaches it |
| crossing | 154 | 2516 | the name and signature stay, the body is the call of the Rust rule |
| kept, on the surface | 581 | 5989 | the body stays and opens with the surface's guard: a tool's call between ticks crosses, a start's runs the TypeScript |
| kept | 5355 | 34615 | the body stays: a start, the picture's step, a reader over the puppet, or a function the Rust game has no row for |
| gone | 189 | 1248 | the signature stays and the body throws by name (`Game.stepSeats`, and a declaration an override that stays still needs) |

Other declarations (classes as data, tables, types): 6717 stay, 523 go (576 lines).

### What keeps the kept bodies

By the root at the end of the first chain of references found (a function many roots reach is counted under one):

| root | functions | lines |
|---|---|---|
| the page | 2602 | 19062 |
| the census | 2218 | 13651 |
| the hosts | 588 | 3189 |
| a tool, and no Rust function to cross to | 518 | 4805 |
| the module | 6 | 6 |
| the bridge | 5 | 34 |

The files that root the most:

| file | functions | lines |
|---|---|---|
| the census | 2218 | 13651 |
| src/main.ts | 555 | 3891 |
| src/ui/bindings.ts | 409 | 2082 |
| src/host/kitView.ts | 209 | 1691 |
| tools/blitz/puppet-apply.ts | 170 | 694 |
| tools/blitz/import.ts | 127 | 1076 |
| src/viewer/runtime.ts | 115 | 920 |
| src/host/rustgame.ts | 104 | 877 |
| src/host/rustdress.ts | 96 | 1201 |
| src/replay/engineplay.ts | 96 | 616 |
| tools/qa/headless.ts | 94 | 450 |
| src/ui/lookpanel.ts | 81 | 1192 |
| tools/mp/server.ts | 65 | 425 |
| src/ui/overlay.ts | 56 | 291 |
| tools/levels/records.ts | 50 | 270 |
| src/editor/paint.ts | 44 | 287 |
| src/net/foresee.ts | 41 | 260 |
| tools/qa/glassbreak-probes.ts | 37 | 201 |
| src/ui/missions.ts | 36 | 107 |
| src/ui/kitCard.ts | 36 | 133 |
| tools/blitz/rust-host.ts | 35 | 156 |
| src/host/loop.ts | 33 | 172 |
| src/ui/loadout.ts | 33 | 110 |
| tools/blitz/classes.ts | 31 | 163 |
| src/host/autocam.ts | 30 | 115 |

### What the page runs every frame

The census's page plays ran 506 functions of the simulation's folders at least once every two frames; 38 of them are on the call surface. A crossing costs milliseconds, so each of these stays a TypeScript reader over the puppet:

| function | file | calls a frame |
|---|---|---|
| Weather.underRoof | src/game/weather.ts | 500.2 |
| rayBox | src/scene/boxes.ts | 356.26 |
| Cues.bubbleOf | src/game/cues.ts | 15 |
| roomName | src/game/dialogue.ts | 6.03 |
| castParts | src/game/cast.ts | 6 |
| castGoggles | src/anim/gltf.ts | 5.03 |
| Cover.tail | src/game/cover.ts | 4 |
| Cover.words | src/game/cover.ts | 4 |
| SquadOrders.statusOf | src/game/orders.ts | 4 |
| lastStandHint | src/game/laststand.ts | 2 |
| NightBreach.objective | src/game/nightbreach.ts | 2 |
| RoomClearScenario.objective | src/game/roomclear.ts | 2 |
| castSuit | src/anim/gltf.ts | 1 |
| autoHint | src/game/autopilot.ts | 1 |
| autoPick | src/game/autopilot.ts | 1 |
| closeMarks | src/game/close.ts | 1 |
| sampleCmd | src/game/cmd.ts | 1 |
| solidBetween | src/game/combat.ts | 1 |
| recapTick | src/game/deathrecap.ts | 1 |
| Director.attach | src/game/director.ts | 1 |
| Director.injectInput | src/game/director.ts | 1 |
| DoorTools.cornerNotes | src/game/doortools.ts | 1 |
| Game.hostAfter | src/game/game.ts | 1 |
| Game.hostBefore | src/game/game.ts | 1 |
| Game.objectivePos | src/game/game.ts | 1 |
| Game.objectiveText | src/game/game.ts | 1 |
| objectivePos | src/game/mission.ts | 1 |
| objectiveText | src/game/mission.ts | 1 |
| SquadOrders.hintView | src/game/orders.ts | 1 |
| SquadOrders.hover | src/game/orders.ts | 1 |
| SquadOrders.ringView | src/game/orders.ts | 1 |
| pictureViews | src/game/picture.ts | 1 |
| gunShut | src/game/player.ts | 1 |
| QuietMission.current | src/game/quiet.ts | 1 |
| updateRainFx | src/game/rainFx.ts | 1 |
| updateSnowBreath | src/game/snowBreath.ts | 1 |
| squad40Plans | src/game/squad40.ts | 1 |
| squadThrowPlans | src/game/squadthrow.ts | 1 |

### A host's loose names the census did not see

394 functions are named only loosely by the bridge's own files (a name read off a value typed `any`, or a type answered by shape) and were never seen to run in a play. With a row on the surface such a function crosses; with none it goes unless something else keeps it. If one of these places does mean that function, the call throws by name in the first play that reaches it. The first 60:

| function | class now | read at |
|---|---|---|
| AirDust.update (src/game/airDust.ts) | deleted | tools/qa/headless.ts:243, tools/qa/headless.ts:246, tools/qa/headless.ts:247 |
| BreachDrill.advance (src/game/breach.ts) | deleted | tools/mp/server.ts:98 |
| BreachDrill.charge (src/game/breach.ts) | deleted | tools/blitz/puppet-apply.ts:1856, tools/blitz/puppet-apply.ts:3644 |
| BreachDrill.once (src/game/breach.ts) | deleted | tools/blitz/puppet-apply.ts:6631 |
| BreachDrill.route (src/game/breach.ts) | deleted | tools/blitz/puppet-apply.ts:680, tools/blitz/puppet-apply.ts:1245 |
| BreachDrill.say (src/game/breach.ts) | deleted | tools/blitz/puppet-apply.ts:7227, tools/blitz/puppet-apply.ts:7499 |
| BreachDrill.tick (src/game/breach.ts) | deleted | tools/qa/headless.ts:244, tools/mp/server.ts:73, tools/mp/server.ts:98 |
| Breaks.tick (src/game/breakTable.ts) | deleted | tools/qa/headless.ts:244, tools/mp/server.ts:73, tools/mp/server.ts:98 |
| ClearedRooms.mark (src/game/cleared.ts) | gone | tools/blitz/export.ts:89, tools/blitz/import.ts:78, tools/blitz/puppet-apply.ts:1113 |
| ClearedRooms.tick (src/game/cleared.ts) | deleted | tools/qa/headless.ts:244, tools/mp/server.ts:73, tools/mp/server.ts:98 |
| Clearing.muster (src/game/squad.ts) | deleted | tools/blitz/puppet-apply.ts:4615 |
| Clearing.say (src/game/squad.ts) | deleted | tools/blitz/puppet-apply.ts:7227, tools/blitz/puppet-apply.ts:7499 |
| Clearing.tick (src/game/squad.ts) | deleted | tools/qa/headless.ts:244, tools/mp/server.ts:73, tools/mp/server.ts:98 |
| Cover.deal (src/game/cover.ts) | deleted | tools/blitz/puppet-apply.ts:1805, tools/blitz/puppet-apply.ts:2256 |
| Cover.drop (src/game/cover.ts) | deleted | tools/blitz/puppet-apply.ts:1661 |
| Cover.facing (src/game/cover.ts) | deleted | tools/blitz/puppet-apply.ts:7702 |
| Cover.free (src/game/cover.ts) | deleted | tools/blitz/puppet-apply.ts:1182, tools/blitz/puppet-apply.ts:1489, tools/blitz/puppet-apply.ts:4936 |
| Cover.has (src/game/cover.ts) | gone | tools/blitz/export.ts:59, tools/blitz/export.ts:139, tools/blitz/export.ts:145 |
| Cover.line (src/game/cover.ts) | deleted | tools/blitz/puppet-apply.ts:1894, tools/blitz/puppet-apply.ts:7479, tools/blitz/puppet-apply.ts:9081 |
| Cover.spare (src/game/cover.ts) | deleted | tools/blitz/puppet-apply.ts:1122, tools/blitz/puppet-apply.ts:1626 |
| Cover.tick (src/game/cover.ts) | deleted | tools/qa/headless.ts:244, tools/mp/server.ts:73, tools/mp/server.ts:98 |
| CpuSmoke.along (src/game/sense.ts) | gone | tools/blitz/puppet-apply.ts:3721, tools/blitz/puppet-apply.ts:8490 |
| CpuSmoke.budget (src/game/sense.ts) | deleted | tools/blitz/puppet-apply.ts:9550 |
| Cues.call (src/game/cues.ts) | deleted | tools/blitz/export.ts:62, tools/blitz/puppet-apply.ts:7631, tools/blitz/puppet-apply.ts:9536 |
| Cues.tick (src/game/cues.ts) | crossing | tools/qa/headless.ts:244, tools/mp/server.ts:73, tools/mp/server.ts:98 |
| Debris.feet (src/game/debris.ts) | deleted | tools/blitz/puppet-apply.ts:3294 |
| Debris.land (src/game/debris.ts) | deleted | tools/blitz/puppet-apply.ts:1888, tools/blitz/puppet-apply.ts:3966, tools/blitz/puppet-apply.ts:7803 |
| Debris.loose (src/game/debris.ts) | deleted | src/host/rustgame.ts:450, tools/blitz/export.ts:71 |
| Debris.step (src/game/debris.ts) | deleted | tools/blitz/perturb.ts:83, tools/blitz/perturb.ts:84, tools/blitz/puppet-apply.ts:1533 |
| Debris.update (src/game/debris.ts) | deleted | tools/qa/headless.ts:243, tools/qa/headless.ts:246, tools/qa/headless.ts:247 |
| Defuse.tick (src/game/defuse.ts) | deleted | tools/qa/headless.ts:244, tools/mp/server.ts:73, tools/mp/server.ts:98 |
| Destruction.update (src/game/destruction.ts) | deleted | tools/qa/headless.ts:243, tools/qa/headless.ts:246, tools/qa/headless.ts:247 |
| Director.lowerThird (src/game/director.ts) | deleted | tools/blitz/puppet-apply.ts:4760, tools/blitz/puppet-apply.ts:6615 |
| Director.signal (src/game/director.ts) | crossing | tools/blitz/puppet-apply.ts:9270 |
| Director.tick (src/game/director.ts) | deleted | tools/qa/headless.ts:244, tools/mp/server.ts:73, tools/mp/server.ts:98 |
| DoorPath.advance (src/game/doorpath.ts) | deleted | tools/mp/server.ts:98 |
| DoorPath.after (src/game/doorpath.ts) | deleted | tools/blitz/puppet-apply.ts:6303, tools/blitz/puppet-apply.ts:9699, tools/blitz/rust-host.ts:169 |
| DoorPath.drop (src/game/doorpath.ts) | deleted | tools/blitz/puppet-apply.ts:1661 |
| DoorPath.free (src/game/doorpath.ts) | deleted | tools/blitz/puppet-apply.ts:1182, tools/blitz/puppet-apply.ts:1489, tools/blitz/puppet-apply.ts:4936 |
| DoorPath.guns (src/game/doorpath.ts) | deleted | tools/blitz/puppet-apply.ts:1304, tools/blitz/puppet-apply.ts:5582 |
| DoorPath.lay (src/game/doorpath.ts) | deleted | tools/blitz/puppet-apply.ts:2363, tools/blitz/puppet-apply.ts:9552, tools/blitz/puppet-apply.ts:9641 |
| DoorPath.leaf (src/game/doorpath.ts) | deleted | tools/blitz/puppet-apply.ts:1903, tools/blitz/puppet-apply.ts:4464, tools/blitz/puppet-apply.ts:7136 |
| DoorPath.leaves (src/game/doorpath.ts) | deleted | tools/blitz/scan.ts:21, tools/blitz/scan.ts:35, tools/blitz/scan.ts:38 |
| DoorPath.move (src/game/doorpath.ts) | deleted | tools/blitz/puppet-apply.ts:2688 |
| DoorPath.open (src/game/doorpath.ts) | deleted | tools/blitz/puppet-apply.ts:996, tools/blitz/puppet-apply.ts:3577, tools/blitz/puppet-apply.ts:3673 |
| DoorPath.sent (src/game/doorpath.ts) | deleted | tools/blitz/puppet-apply.ts:1669, tools/blitz/puppet-apply.ts:4879, tools/blitz/puppet-apply.ts:5567 |
| DoorPath.stand (src/game/doorpath.ts) | deleted | tools/blitz/puppet-apply.ts:4250, tools/blitz/puppet-apply.ts:8802, tools/blitz/puppet-apply.ts:9099 |
| DoorPath.tick (src/game/doorpath.ts) | gone | tools/qa/headless.ts:244, tools/mp/server.ts:73, tools/mp/server.ts:98 |
| DoorPath.total (src/game/doorpath.ts) | deleted | tools/blitz/puppet-apply.ts:2021, tools/blitz/puppet-apply.ts:5013 |
| DoorPath.way (src/game/doorpath.ts) | deleted | tools/blitz/puppet-apply.ts:1543, tools/blitz/puppet-apply.ts:2141, tools/blitz/puppet-apply.ts:2551 |
| DoorTools.arrived (src/game/doortools.ts) | deleted | tools/blitz/puppet-apply.ts:1100, tools/blitz/puppet-apply.ts:1252, tools/blitz/puppet-apply.ts:1340 |
| DoorTools.away (src/game/doortools.ts) | deleted | tools/blitz/puppet-apply.ts:9393 |
| DoorTools.bang (src/game/doortools.ts) | deleted | tools/blitz/puppet-apply.ts:3944, tools/blitz/puppet-apply.ts:6480 |
| DoorTools.flows (src/game/doortools.ts) | deleted | tools/blitz/puppet-apply.ts:1140 |
| DoorTools.go (src/game/doortools.ts) | gone | tools/blitz/puppet-apply.ts:1155, tools/blitz/puppet-apply.ts:5426, tools/blitz/puppet-apply.ts:7575 |
| DoorTools.input (src/game/doortools.ts) | deleted | tools/blitz/puppet-apply.ts:937, tools/blitz/puppet-apply.ts:2224, tools/blitz/puppet-apply.ts:4976 |
| DoorTools.lastStack (src/game/doortools.ts) | deleted | tools/blitz/puppet-apply.ts:1074 |
| DoorTools.scan (src/game/doortools.ts) | deleted | tools/build.ts:16 |
| DoorTools.short (src/game/doortools.ts) | deleted | tools/blitz/puppet-apply.ts:5026, tools/blitz/puppet-apply.ts:8607, tools/blitz/puppet-apply.ts:8999 |
| DoorTools.step (src/game/doortools.ts) | deleted | tools/blitz/perturb.ts:83, tools/blitz/perturb.ts:84, tools/blitz/puppet-apply.ts:1533 |

### Kept, and never seen to run

3754 of the 5937 functions that keep a body (26960 lines) were never seen to run as TypeScript in any play of the census. The checker reaches each from the page or a host, so it stays: most are what only a draw, a panel, the workbench or a start nobody played would run, and the rest are dead in practice. `delete-plan.tsv` has them (the `seen` column is empty): the list a cleanup with a real GPU would start from.

### The checks that lost a function they called

Nothing is restaged here: this is the list to fix forward from. `tools/blitz/delete-lost.tsv` has every row (a tool, a function, how the tool names it).

- 150 files of tools/ name, by a reference the checker resolves, a function of the tick that the Rust game has no row for (189 functions). Each keeps its signature and throws by name when the check calls it.
- 753 files read, off a game typed `any`, a name that matches a function which went. By name only: the file may mean another object's method of that name.
- 244 files TEST for such a name (`g.f?.()`, `typeof g.f === 'function'`) at 1288 places: a check that takes the other branch quietly. These are the ones to read first.

The files that name the most functions which now throw:

| tool | functions that throw by name |
|---|---|
| tools/qa/bigslot-check.ts | kitSlingFront, kitInHand, samBig, hammerBlows, bigOutBy, bigWind, bigSwing, bigEquip … (10) |
| tools/qa/rails-check.ts | Checkpoints.loadInStep, Rails.locksWall, Rails.gate, Rails.tick, stalkCould, Rails.stepEye, RoomFight.tickMove, Rails.runs … (9) |
| tools/qa/wand-check.ts | SquadOrders.open, wandLens, wandOffered, wandFov, wandPanMax, wandEye, wandFits, wandFeedSecs |
| tools/qa/fn2000-probes.ts | rifleSpread, ocpable, closeLethal, closeBare, kitReload, Firearm.autoSpread |
| tools/qa/soak.ts | closeBare, CharacterAnimator.reloadShow, agitationOf, whistleMem, stillLooking, Firearm.reloadRow |
| tools/qa/layer-ident.ts | beatOf, CharacterAnimator.tell, CharacterAnimator.nvFlip, CharacterAnimator.blow, CharacterAnimator.taken, CharacterAnimator.takenCancel |
| tools/qa/sticky-check.ts | wandOffered, wandFits, stickyEyes, stickyBoxes, stickyRows |
| tools/qa/sense-ident.ts | FogOfWar.handEyes, FogOfWar.sees, CpuSmoke.along, StaticCollision.tickOpen, StaticCollision.tickShut |
| tools/qa/gunroles-check.ts | gunString, gunStringGap, gunLane, gunLast, gunBlindGap |
| tools/qa/kitanim-probes.ts | kitSlingFront, kitInHand, kitCloseness, CharacterAnimator.reloadShow |
| tools/qa/bigmoves-strip.ts | bigOutBy, bigSwing, bigEquip, bigStow |
| tools/qa/door-ident.ts | Doors.pick, StaticCollision.tickOpen, StaticCollision.tickShut, Doors.updateBoth |
| tools/qa/gunslots-check.ts | rifleSpread, sideLocked, feedCues, beatOf |
| tools/qa/handring-check.ts | handRingKeys, loneKey, pillBox, wedgeAt |
| tools/qa/pane-ident.ts | GlassBreak.update, StaticCollision.tickOpen, StaticCollision.tickShut, GlassShards.update |
| tools/qa/dialogue-probes.ts | heldTalk, samBeat, agitationOf, samStageOf |
| tools/qa/slab-ident.ts | StaticCollision.tickOpen, StaticCollision.tickShut, SlabStore.doorAt, slabDiff |
| tools/qa/prop-ident.ts | StaticCollision.tickOpen, StaticCollision.tickShut, PropSystem.update, PropSystem.updateHere |
| tools/qa/ai-probes.ts | colleagueInLine, aimAhead, doorLabel |
| tools/qa/litman-probes.ts | RoomFight.short, RoomFight.funnelOf, RoomFight.holdShort |
| tools/qa/wallcharge-probes.ts | SquadOrders.open, Cover.has, interactOrder |
| tools/qa/netquiet-check.ts | mate, netQuietLook, netFeedAsked |
| tools/qa/close-probes.ts | closeLethal, closeReady, closeNoise |
| tools/qa/glassbreak-probes.ts | GlassBreak.bang, GlassBreak.update, Wind.stormWind |
| tools/qa/segment-ident.ts | GlassBreak.update, StaticCollision.tickOpen, StaticCollision.tickShut |
| tools/qa/devices-probes.ts | heldTalk, lensDir, bodyBreaks |
| tools/qa/key-probes.ts | heldTalk, samBeat, agitationOf |
| tools/qa/doorstay-probes.ts | Doors.hasCloser, handPress, Doors.passBit |
| tools/qa/doorbrisk-check.ts | teamWalk, teamWalkOf, teamSprinting |
| tools/qa/pacing-check.ts | teamWalk, teamWalkOf, teamCap |
| tools/qa/kitload-check.ts | loadGait, teamLegs, loadHeard |
| tools/qa/condition-check.ts | conditionSup, guardFireMods, samSpread |
| tools/qa/spotted-probes.ts | shotLine, spottedFeet, spottedKeen |
| tools/qa/line-watch.ts | colleagueInLine, RoomFight.short |
| tools/qa/standoff-probes.ts | colleagueInLine, pairOf |
| tools/qa/discipline-probes.ts | colleagueInLine, pairOf |
| tools/qa/stackview-check.ts | ClearedRooms.mark, DoorTools.go |
| tools/qa/frag-probes.ts | lying, lyingShows |
| tools/qa/silent-probes.ts | Doors.pick, quietGun |
| tools/qa/sticky-cover.ts | wandLens, wandFits |
