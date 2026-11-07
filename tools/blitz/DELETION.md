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
| the page | every file of `src/` outside the simulation's folders | its body, a rule's too. The page has no call surface: what it names on the puppet runs as TypeScript. What THAT calls is held like anything else |
| a page with no Rust game | the files a page of the repository's root loads (its script, followed by imports) where that page never loads the host of the Rust game: the workbench with its viewer and editor, the replay viewer, the sound and music pages | its body and every body under it. Nothing crosses there and nothing is applied over |
| the hosts | the files of `tools/` that a page or a host loads, found by following imports from the entry points in `HOST_ENTRIES` | its body. They run inside the host's own lines (a start, the apply, the out events, the dress step), where nothing crosses |
| the module | a simulation file's own top-level statements, and the names the bridge's schema reads through its window | all of it |
| a tool | every other file of `tools/` (the checks) | with a row on the call surface: its signature, and the body becomes the crossing. With no row there is no Rust function to cross to: its body |
| the census | a function seen to run as TypeScript while the Rust game stepped | its body |
| `Game.stepSeats` | the tick | nothing: its body goes, and a host hangs its own step on the game object |

**A rule is not followed into.** A function on the call surface that takes a game (a method of one of the simulation's classes, or a parameter that
is one) and that the surface's table says writes is a RULE. Run as TypeScript once a Rust game stands, a rule writes a puppet that the next apply
writes over, so no reference keeps a rule's body by itself: only the census does, by having seen it run as TypeScript outside the tick, which is what
a start does (the game is stood in TypeScript and crosses whole afterwards). A rule nobody saw run keeps its signature and crosses. A function of the
surface that takes no game (the kit's tables, a lookup) can be called by a menu with no game stood, and is followed like any other. `--no-barrier`
follows every reference the checker finds; it keeps about half as much again.

**Exact and loose references.** A reference the checker resolves to a declaration is exact. So is a member of an interface for every class that says
it implements it, a name read off a module loaded by a written path (``await import(`${ROOT}/src/game/x.ts`)``, which is how the tools load the game),
and a name read off a value typed `any` through two or more steps that are a path from the `Game` class (`g.cues.stats`). A cast is a flow, whatever
the checker thinks of the two types: `g.destr as unknown as { doorsFor?(…): … }` is a use of the class's `doorsFor`, a private member too, and a name
read off `(x as any)` or off a constant that was given such a cast is that class's member. Two kinds are loose: a single name read off a value typed
`any`, and a member of a type that a class answers only by its shape. A loose reference could mean every declaration of that name, and followed blindly
they keep most of the tick, so the census settles them: a candidate it saw run keeps its body; one it never saw is not held by the name alone (with a
row on the surface it stays as a crossing). With no census at all every loose reference is followed.

**Nothing may go missing quietly.** The census plays ticks and starts, and of the page only what its flows reach; it has not seen every draw or every
panel. A function that is gone under a plain call throws by name, and a throw is seen by the gates below and by whoever plays. A function that is gone
under a read that TESTS for it (`x.f?.()`, `typeof x.f === 'function'`, `x.f && x.f()`, `if (x.f)`) is a branch quietly not taken. So a loose read of
that kind, in the page or in the simulation's own kept code, holds every candidate as an exact reference would, census or no census. In a tool it holds
nothing, and the place is listed below with the checks that lost a function they called.

## The gate of starts

The rule above rests on one claim: every rule a start runs was seen by the census. So the census does not sample starts, it stands a TABLE of them:
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
