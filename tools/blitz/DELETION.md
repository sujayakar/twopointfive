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

The program refuses a tree it has already been through (`src/game/crossing.ts` is there), a census taken on another tree, and a call table
(`calls.json`) whose guards it cannot find by key.

## How it decides

By reachability, read from the TypeScript checker over `src/` and `tools/` together. Nothing is classed by its name or by a list kept by hand.
The simulation's folders are `src/game`, `src/anim`, `src/math`, `src/sim` and `src/scene`: only there does it delete.

**What stays reachable, and how far each root reaches:**

| root | what it is | what a function it reaches keeps |
|---|---|---|
| the page | every file of `src/` outside the simulation's folders: the game's page, and the pages with no Rust game (the workbench with its viewer and editor, the replay viewer, the sound and music pages) | its body, and everything that body reaches. The page has no call surface: what it calls on the puppet runs as TypeScript |
| the hosts | the files of `tools/` that a page or a host loads, found by following imports from the entry points in `HOST_ENTRIES` | its body. They run inside the host's own lines (a start, the apply, the out events, the dress step), where nothing crosses |
| the module | a simulation file's own top-level statements, and the names the bridge's schema reads through its window | all of it |
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

## What the deletion asks of the hosts

The hosts are not this program's. What it changes in them is anchored on one line each and said as it runs; what it leaves for their owners is listed
in the tables below.

- `tools/blitz/preload.ts` loads `crossing-keys.ts` after `expose.ts`.
- `tools/blitz/calls.json` loses its position tables (the rows stay).

<!-- the tables below are written by tools/blitz/delete-tick.ts --note -->
