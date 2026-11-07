# tools/blitz — the headless host: the scripted player plays the Rust game

`rust-host.ts` makes a TypeScript `Game` a PUPPET of a game that steps in the wasm face (rust/game_face). From the first tick **the Rust game is the only
game that steps**: `Game.update`'s own lines run as they are (the record sampled, hostBefore, hostAfter), the step between them is the host's, and after
it the Rust game's state is applied onto the TypeScript objects in place. Whatever reads the game (the scripted player of tools/qa, a tally, a probe)
reads what the Rust game holds, and what it presses goes to the Rust game as the tick's record. `stepSeats`' body never runs.

```sh
tools/blitz/play.sh room-clear-four:1                  # the plain TypeScript play first (the reference), the face built, then the play, every tick compared
tools/blitz/play.sh <mission>:<seed> --no-build -- --audit --ticks 600
bun run --preload ./tools/blitz/expose.ts tools/blitz/play.ts <mission>:<seed> --ref DIR [--out DIR] [--ticks N] [--audit] [--whole-every N] [--stale N] [--go-on] [--quiet]
tools/blitz/rust.sh rust bun run tools/qa/playthrough.ts --scn depot-four --seeds 1-4 --json rust.json     # any headless tool, on the Rust game
```

| file | |
|---|---|
| `rust-host.ts` | the host: the module, a game's start, the step, the two applies, the driver's calls, the out events, the dress step. `RustHost.attach(game, opts)` |
| `play.ts` | a mission stood as `tools/qa/playthrough.ts` stands it (a tutorial: `lesson-run.ts`), played on the host, HELD TO THE PLAIN TYPESCRIPT PLAY every tick |
| `audit.ts` | the walks that compare two whole states field by field (the audit's, and `--stale`'s) |
| `rust.sh` | the harness's word and the bridge's preload in front of any command |
| `tally-diff.ts` | two `playthrough.ts --json` files side by side, a run a row |
| `thrice.ts` | one fight stood N times in one process, every game held to the first (the digest a second; `--ticks`, `--dump` to find where two part) |
| `fuzz.sh`, `fuzz-diff.ts` | THE FUZZ: one seed of tools/qa/soak.ts played on the folded TypeScript tick and on the Rust game, a line a tick each, and the first tick they part with what the schedule did before it |
| `fuzz-dumps.ts`, `diff.ts --live`, `known-loaded.txt` | the whole states of a fuzz run every half minute, held to each other by path, canonically; `dumps.txt` and `rust-hashes.txt` |
| `fuzz-replay.sh`, `fuzz-replay.ts` | a KEPT run played again on the Rust game alone, and the first tick that differs: the fuzz's reference once the TypeScript tick is gone |
| `fuzz-squad.sh`, `perturb.ts` | THE FUZZ ON A SQUAD MISSION: a seeded stream of orders, throws, fire, kills, saves, loads and restarts laid between the ticks of the scripted player (`QA_PERTURB=<seed>` over any headless tool), the same two-sided compare |
| `keep-check.ts` | a snapshot and a save put back, six games in four slots, a refused re-step: the same lines under `ts` and `rust` |
| `qa-sweep.py` | every family of probes.ts three ways, a probe a row: the table's `sweep` rows and its Rust-only reds by kind |
| `word.ts` | THE ONE SWITCH: the word asked for (`TPS_GAME`, else `DEFAULT_WORD`), the table, a frozen run, the closing line. No import of src |
| `preload.ts` | bunfig.toml's preload: nothing unless the word asks; then the bridge's window (`expose.ts`) and the top process's closing line |
| `qa-kinds.json` | the table `TPS_GAME=table` reads |
| `table-check.ts` | the table held to `probes.ts list` (by hand): red on a name the table lacks or a row probes.ts lost |
| `die-check.ts` | the game's die seeded after the stand, the helper and the plain site, a line a word |
| `rust/game_face/src/harness.rs` | the face's exports a host that plays needs: the filtered export with its memo, the camera's ask, a flooded room, the program stopped before a take |

## Since cut 7.10: what a tool does to the game between two ticks, and how it reaches the Rust game

| what the tool does | how it reaches the Rust game |
|---|---|
| calls a rule of the tick that is on the call surface (`game.teleportPlayer(…)`, `fireWeapon`, `game.restartEncounter()`, `orders.give`) | the generated guard crosses it (`surface.ts`, `face_call`), after the host's half and the tool's pending writes; the puppet takes the filtered state |
| calls a rule that is NOT on the surface (it takes a closure, or has no Rust row: `Game.interrogate`, `interrogate`, `pickLine`, `takeKey`) | IT DOES NOT: the TypeScript runs on the puppet, counted on the exit line. What it wrote by name may still go by the scan |
| writes a field by name (`game.player.visibility = 0.9`, `gd.path = []`) | the scan (`scan.ts`): the names the loaded tools/qa files write are walked as a tick opens and before a call crosses, and each difference is poked |
| writes a field that lives in rust/sim's block (`door.vel = 2`, `door.latched = false`, a man's place, the die) | the host holds the puppet's copy of the block to the Rust game's at the same two moments and hands over each part that differs but `poses` (`blockBack`, a stand-in) |
| takes a snapshot and puts it back (`W.snapshot()` / `W.restore(s)` of tools/qa/headless.ts) | the Rust game is KEPT in the module beside the file (`face_keep`, a clone) and the same file put back unchanged is that game standing again (`face_back`), one whole apply. A file the tool changed, or one of another game, is loaded by the TypeScript on the puppet and the next step is a new start |
| saves and loads through the save controller (`saves.write`, `saves.load`, the autosave, Enter once down inside the tick) | one path with the page: every save is the Rust game's own clone (`SNAPS.own` while a hosted game's controller runs), the controller's shelf is handed to the Rust game as a step opens (`face_saves_in`), what the tick asked of it is read back (`face_saves_asked`), the controller's own lines round a load are made on the Rust side (`face_loaded`) |
| calls `loadState(game, file)` itself | the TypeScript runs whole on the puppet (its restart does not cross) and the next step is a new start from the puppet |
| restarts inside the tick (Enter once down, Shift+Enter) | the Rust game's own; the puppet's mission record is made anew (a run's beginning is known outside the tick by that identity), and the men the restart made take the whole state |

**A new start** is made when the mission's clock began again on the puppet (`mission.t0`), a file was loaded on it, or a snapshot was put back on it
(the game's own clock no longer the Rust game's). A mission record swapped for a copy of itself (the old tour, src/game/demo.ts) is not one.
`start()` THROWS, naming the tool's line, for a new start from a puppet that holds a man the apply made (a man the Rust game made has no store row or
pose store on the TypeScript side).

**A game the tool has let go is freed.** Nothing in the host keeps a game alive: what a host holds in the module (its game's handle, the clone kept as
its slot went, the games kept for saves and snapshots) is in a cell of its own, and the host's lists hold hosts weakly. As the next game is attached, and
before a slot is taken from a game still held, every host the collector has taken gives back what it held (`reap`). A tool that wants a game kept holds
the game. (Before this the call surface listed every host that ever stood: 25 games stood back to back grew the heap 276 MB.)

**A slot given up** (rust/sim has four; a tool that stands a fifth game takes the slot of the game stepped longest ago) keeps a clone of its game in
the module; stepped again, the game stands again from it (`restep`). Only the last six games that gave a slot up keep theirs: an older one's re-step
THROWS and names the tool's line. `tools/blitz/keep-check.ts` holds all of this under either word.

## The fuzz: one soak seed on both games, tick for tick

The scripted missions drive little of the game from outside the tick; `tools/qa/soak.ts` drives a lot: a seeded schedule of random stimuli (gunshot,
kick and bang events, kills, the player teleported onto men, doors picked, kicked and flung, blackouts, restarts, mission steps, grabs, interrogations,
saves and loads). `tools/blitz/fuzz.sh` plays one seed twice and holds the two plays to each other:

```sh
tools/blitz/fuzz.sh <seed> [--minutes 3] [--hard] [--skip a,b] [--only a,b] [--out DIR]
bun run tools/blitz/fuzz-diff.ts DIR [--json] [--but-panes]
```

- **The reference** is the `ts` word UNDER THE FOLD (`QA_FOLD=1`): the same stand, the settled switch rows at their page values, the game owning its
  die (the soak puts its seeded die in `Math.random`'s place before the stand, and the stand seeds the game's die off one draw of it, the same on
  both sides). Under `ts` every save is the TypeScript's snapshot in memory, as under `rust` every save is the Rust game's clone: a FILE's load stages
  the encounter again and pours the file in, which is not the saved moment to the bit.
- **A line a tick on each side** (`RH_TICKS`: `ts-run.txt`, `rust-run.txt`): the tick, the die's count, a hash of the host's half as handed in and of
  the orders given, a hash of each of the block's parts but `poses` (head, dice, doors, falls, props, slabs, items, men, lights, weather, smoke,
  panes). A tick a kept game stood again in is written twice; its last line counts.
- **`fuzz-diff.ts`** names the first tick the die or the block parts, the first tick the host's half differs, where the two timelines first say
  another thing and where the DRIVER'S OWN ACTS first differ (the schedule reads the game it is given: once the driver takes another road the two
  sides are no longer the same experiment), the soak's own invariants that are red on the Rust side only, and the call surface's exit line.
- **A line a tick's third word** is the host's half WITHOUT the lens's serial (`FollowCamera.desired_serial`, the bridge's own count of the camera's
  `desired` arrays: after a load it is the frame's count on the TypeScript side and the saved moment's plus one on a puppet; no rule reads it).
- **The whole states every half minute** (1,800 ticks; `RH_DUMP`) are held to each other by path, every pair both sides kept, up to the tick the die or
  the block parted (`fuzz-dumps.ts` → `dumps.txt`, a line a pair; `diff-<tick>.txt` where a path is not known). THE DIE AND THE BLOCK ARE THE BODIES AND
  THE DICE: a call that ran in TypeScript on the puppet and never reached the Rust game (an interrogation: a talk's record, the chronicle, a door's
  code) leaves them equal for minutes.
- **EQUAL means all three**: the die, the block, and 0 paths not known at every half minute. The scripts exit 0 only then (2: equal every tick, a path
  not known; 4: equal by all three but for the panes after a load, below).
- **To look closer**: `RH_SAY='<an expression of g>'` writes what the tool would read, a line a tick on each side (`<word>-say.txt`; `RH_SAY=digest` the
  game's digest); `RH_DUMP=<ticks>` keeps the whole state of each side at those ticks for `diff.ts`; `RH_HALF=<ticks>` writes the host's half a field a
  line under either word (with `RH_TICKS`, to `<word>-half.txt`: a tally plays its games in children whose output goes nowhere).

### The fuzz on a squad mission

The soak's level is the solo night, and Hard moves no tick of it (the reference's tick files for seeds 1 to 6 are the same bytes on Standard and on
Hard). The squad missions are where the orders menu, the door tools, the room fight and the defenders' squads run, so the second fuzz lays its own
stream over the scripted player of `tools/qa/playthrough.ts`:

```sh
tools/blitz/fuzz-squad.sh <mission> <seed> [--hard] [--rate K] [--god player|squad|all] [--alarm TICK] [--skip a,b] [--only a,b] [--out DIR] [--side ts|rust]
QA_PERTURB=<seed> [QA_PERTURB_RATE=K] [QA_PERTURB_GOD=player|squad|all] [QA_PERTURB_ONLY=a,b] [QA_PERTURB_SKIP=a,b] [QA_PERTURB_FROM=<tick>] <any tool that stands its game through tools/qa/headless.ts>
```

- **`tools/blitz/perturb.ts`** is laid by `tools/qa/headless.ts` when `QA_PERTURB` is given: before every step a schedule of its own (one seeded
  stream, never the game's die) may give a row the orders menu offers on a random door, floor spot or man (`orders.rowsFor`, `orders.give`; a refusal
  is logged with its words), call an order off (`fallIn`), drop a frag, a flash or a smoke can, fire the player's round at one of theirs, kill one of
  theirs (now and then one of the team), tell the hostages to follow, cut the mains, save by hand, load the last save, or press the pause card's
  restart. Everything goes through the game's own doors, as the menu, the keys or a tool would. The log is in the soak's shape, so `fuzz-diff.ts`
  reads either fuzz.
- **Determinism first**: the same mission and seed twice on one side must write the same tick file; a parting between the sides means nothing
  before that is shown.
- **The whole states** are kept every 1,800 ticks and every pair is held, as the soak's are (above).
- **`--god player|squad|all`** (`QA_PERTURB_GOD`): the sandbox row's god mode held on (`game.godMode`), the net a showcase cast wears put on every living
  man of the team (`guard.god`: rounds and fragments pass through him), or both, again on whoever a restart or a load brings. A run that lasts: with
  the team's deaths and not the player's, or with neither (two casts at doors for minutes).
- **`--alarm TICK`** (`QA_PERTURB_ALARM`): the response team is called at that tick the way `tools/qa/qrf-squad-check.ts` calls it (`tune.qrfSquad` on,
  four men fourteen seconds out, the alarm's place on one of the team, `requestQrf(g, 'panel')`) and the alarm's place is put on one of the team again
  every twenty seconds. WHY: in TypeScript the response team's cast runs its door tools with a VIEW of the game (src/game/squadcast.ts), which is
  another key than the game for every table keyed by the game object, so it keeps rows of its own in some twenty of them; the Rust game holds one row
  on Game for each. Only a floor with both casts at doors can show whether a tick reads the difference.
- **`FUZZ_CAP`** (900 s): a side cut short is reported as "one side stopped" and the ticks both played are still compared.

### The whole states, canonically (`diff.ts --live`), and what each rule hides

Two states the die and the block call equal are not the same bytes: the two sides number rows and deal identity serials apart, and the TypeScript keeps
tables the Rust game keeps otherwise. `fuzz-dumps.ts` holds a pair with these rules, and a clean run reads 0 paths not known:

| rule | why | what it hides |
|---|---|---|
| a canon table's rows are numbered by reach FROM THE LIVE GAME: a map or set keyed by a row is walked only once its key is reached some other way | the TypeScript keeps such tables weakly, by the object, and lists a row for a player, a gun or an order's assignment that is out of the game (a fresh deal, a restart, a load) until the collector takes it. Numbered through such a row, every gun after it compared crosswise (61 of 67 paths on one run) | nothing |
| a row ONLY A KEY reaches is named by its content (`Id(~1x9k2)`); an entry keyed by one is compared where both sides hold a row of that content, and counted by side and table where one side does | no rule can ask a weak table for a row whose key nothing else holds: such rows are dead on both sides, and which of them still stand is the collector's doing on one side and the table's on the other (`orders.back_fix` 13 rows against 2 after a load: 11 under assignments of the abandoned future) | a STRONG map or set keyed by a row nothing else holds, where the two rows differ |
| a table keyed by an identity serial (schema `serial`, `<field>{}`): a map by the multiset of its values, a set by its count; the Rust side's surplus is reported, the TypeScript's counted | by place it read crosswise once one side held a member the other had dropped (`chronicle.heard_of`, `fight.heard_ev`, `qrf.heard_events`: the events of an abandoned future) | which key holds which value; a member only the TypeScript holds |
| `gunhold.knowns`, `guards.holds_called` are compared as serial-keyed (STAND-IN: the schema does not mark them) | their key is an object, written as the serial each side deals it | as the rule above |
| `wallcharge.stand_ins` is not walked for reach and its rows are known | a declared memo; the fuzz's `orders.rowsFor` between two ticks fills it on the TypeScript's side | the memo |
| AFTER A LOAD (`known-loaded.txt`): `g.stats.*`, a module's `*_log` and `*_why`, `scene_collision.sensed.*`, `scene_box_mirror.things.n`, `doorway.stack_own.*` | the TypeScript's put-back takes back what the game reaches; a module's own log or tally is a plain table of the process and keeps what the abandoned future wrote. The Rust game holds them inside the game and its kept clone takes them back | every such log and tally from a run's first load to its end. No rule reads one |
| the response team's key ring, `stackroom.books`, `seenwalk.shared` … (known.txt) | the cast's ring is an object that stands in for a set; the cast keeps its own row of a table keyed by the game under its VIEW of the game, the Rust game one row | the cast's own row of those tables |

**The panes after a load are the reference's own.** The TypeScript's put-back leaves a pane a round broke in the abandoned future broken; the Rust
game's kept clone has the saved moment's. A run that parts in the block's `panes` alone after a load is held again with the panes left out
(`fuzz-diff.ts --but-panes`; exit 4).

### Kept runs: the three games of two seats

`tools/blitz/kept/two-lock0-9.tar.zst`, `two-lock1-9.tar.zst` and `two-full-3.tar.zst` are the games of two players that the co-op check plays
(`coop-check.ts`: each seat's pair sent through a locked door; the barracks with every seat counted). Their records (the host's half of every tick and
the orders given) were written by the TypeScript tick while it stood, and nothing can write them again, so they are kept whole: the records, the
Rust game's own series (the die's count and a hash of each part of the block after every tick) and the run's `coop.json`. No start is kept: the
start's bytes are bound to the wire, and `hold.sh` stands it again from this tree (`seats.ts --start-only`).

```sh
tools/blitz/hold.sh tools/blitz/kept                              # each: "the same over N ticks", exit 0
HOLD_DIRS=DIR tools/blitz/hold.sh tools/blitz/kept                # … and leaves each run's folder in DIR, start and all
rust/game_run/target/debug/mission DIR/two-lock0-9 1800 --keep 960,990,1020   # the Rust game's whole states at those ticks: DIR/two-lock0-9/n<k>.bin
```

The lock games' series is the Rust game's alone: the TypeScript's was another from ticks 1082 and 1056 on purpose, where a seat's door tools first
kept to their own men. The barracks' is both games', in step for 7200 ticks.

### Kept runs: the fuzz with no TypeScript tick

A run that is EQUAL by all three measures is worth keeping: its Rust side IS what the TypeScript tick did. Keep the folder's `cmd.txt` (the command),
`rust-*.txt` (a line a tick), `rust-hashes.txt` (a hash of the Rust game's whole state every half minute), `dumps.txt` and `rust.log` (the acts).

```sh
tools/blitz/fuzz-replay.sh <kept folder> [<kept folder> …] [--work DIR]
```

plays each kept command again on the Rust game alone (`--side rust`: one process, about a minute a thousand ticks on a loaded box) and says THE SAME
PLAY, or the first tick whose line differs (the die, the host's half or the orders given, which parts of the block), the acts before it, and the first
whole state whose hash differs.

**In the repository: `tools/blitz/kept/<name>.kept`, a kept run packed into a few lines** (`bun run tools/blitz/fuzz-replay.ts --pack <folder>`): the
command, the tree it was shown on, the ticks played, the die's count as the last tick returned, one word for every 600 ticks' lines (ten seconds of
play) and the whole states' hashes. `fuzz-replay.sh tools/blitz/kept/<name>.kept` plays it again and names the TEN SECONDS a new play first differs in;
the folder itself (a line a tick; kept outside the repository, 3 MB a run) names the tick.

Each run below was EQUAL to the TypeScript tick by all three measures (the die, 12 parts of the block and the host's half on every tick; 0 paths not
known in the whole states every half minute), both sides played from one tree, Hard, the fold laid:

| packed run | the command (from the repository's root) | ticks | the die at the end | whole states | how it ended, both sides | what it drives |
|---|---|---|---|---|---|---|
| `depot-four-2-r2` | `tools/blitz/fuzz-squad.sh depot-four 2 --hard --rate 2` | 8,006 | 225 | 4 | taken 133.4 s, 2 of 4 up | a save by hand at f508 and its load at f5054 (the clock back to 8.5 s), 30 orders of the menu's rows on doors, floor and men, a smoke can, the mains cut, the player's rounds, two kills |
| `rt-depot-four-1` | `… depot-four 1 --hard --rate 2 --alarm 130 --god all --skip kill,fire` | 5,870 | 667 | 3 | taken 97.8 s, 4 of 4 up | the response team called at tick 130 and deployed by 1330: TWO CASTS of the door tools on one floor for 75 s, the squad and the player unhittable; 19 orders, 4 throws, 3 saves |
| `rt-depot-four-2` | `… depot-four 2 --hard --rate 2 --alarm 130 --god all --skip kill,fire` | 10,670 | 1,106 | 5 | taken 177.8 s, 4 of 4 up | the same, the longest: both casts for two and a half minutes |
| `rt-records-pair-1` | `… records-pair 1 --hard --rate 2 --alarm 130 --god all --skip kill,fire` | 3,752 | 812 | 2 | taken 62.5 s, 2 of 2 up | the response team on the records office, a pair |
| `rt-records-pair-2` | `… records-pair 2 --hard --rate 2 --alarm 130 --god all --skip kill,fire` | 4,005 | 532 | 2 | taken 66.7 s | the same, another stream |
| `rt-records-pair-3` | `… records-pair 3 --hard --rate 2 --alarm 130 --god all --skip kill,fire` | 3,356 | 396 | 1 | taken 55.9 s | the same |

(`FUZZ_CAP=2400` stands in front of each in the packed file: the longest a side may run.)

**What a kept run is and is not.**
- It is of ONE TREE. A change that moves the play on purpose (a rule, the scripted player of tools/qa/playthrough.ts, a perturbation of perturb.ts, a
  level's file, the harness's camera) makes it differ at the first tick it touches; then the run is packed again and the change says so. A change that
  should move NO play (a refactor of the Rust game, the cleanup's column store, a new build) must leave every packed run THE SAME PLAY.
- The line a tick holds the die, the host's half (without the lens's serial), the orders given and each part of rust/sim's block but `poses`. The
  whole states' hashes are of the Rust game's own export (`face_export`) every 1,800 ticks: what the block cannot see (the chronicle, a talk's
  record, an order's words, a gun's rows). A change to the schema or the export's order moves every hash and no line: pack again, having seen the
  lines equal.
- SHOWN: the Rust game's four whole-state hashes of `depot-four-2-r2` were the same in three separate plays, and each packed file here is THE SAME
  PLAY as its own folder. A whole `fuzz-replay.sh` play of each packed run has been made since (the retake, below).
- A PACKED RUN IS OF ONE LAYOUT OF THE WIRE, and says which (`layout`, the stamp `schema.layout` that the module is held to as it loads). The line's
  host-half word is a hash of the host's parts and a whole state's hash is of its bytes, so a field added to either moves them with no play moved.
  Held to a packed run of another layout, `fuzz-replay` says THE WIRE'S LAYOUT MOVED by name (exit 4 where the ticks played and the die's count are
  still the packed run's), not "differs first in ticks 1 to 600".
- THE PLAY'S OWN WORDS (`play`) are in a packed file beside the lines': the same ten-second windows over each line without the host-half word, so
  the die's count and the block's parts alone, which no layout of the wire touches. After a change of the wire, ONE COMMAND takes every packed run
  again: `tools/blitz/fuzz-retake.sh` plays each, holds the new play to the `play` words, and only where they are equal writes the file again from the
  new play (its lines, its states, the tree, the layout). A run whose play differs is left alone and named, with the ten seconds. With the layout
  unchanged it is `fuzz-replay.sh` over all of them and writes nothing.
- THE SIX WERE TAKEN AGAIN after the TypeScript tick went, on the tree their `tree` line names, and each time this was seen first: every tick's line
  of the new play IDENTICAL to the kept folder's own line (the one that was equal to the TypeScript tick) in the die's count and all twelve parts of
  the block, on all 35,659 ticks; only the host-half word differed, by the capability part the host's half gained. The whole states had differed from
  the kept ones in ONE list alone, by `diff.ts` on two runs at three ticks (20 paths, every one `g.m.owner.witnesses[k]`): the owner check's
  witnesses, filled as each rule's file loaded and so in the bundle's order, which moved when the tick's files went. That list is kept in the rules'
  order now (owner.ts `ownerWitness`). So the lines of the packed files are still the TypeScript tick's play; their half words and state hashes pin
  the Rust game to itself. The three records runs were played twice on that tree: lines and state hashes the same both times. On the tree the files
  name, `fuzz-retake.sh` over the six said "of this layout already, and the same play" of each: every line and all 17 whole states' hashes.
- A run packed AFTER the TypeScript tick is gone was held to nothing: it pins the Rust game to itself, which is worth having and is another thing.
- The scripts stand the Rust game by the harness's word (`TPS_GAME=rust`, `QA_FOLD=1` in fuzz.sh and fuzz-squad.sh). When the word goes with the
  TypeScript tick, `--side rust` is what stays and the two variables go.

**Still to be played on both sides and kept, in order of worth** (none is a kept run until it has been EQUAL by all three):
`fuzz-squad.sh <hostage-rescue | rescue-registry | depot-four | warehouse-four> <1 to 6> --hard --rate 2` (even seeds with `--god player`: the squad
dies, the player does not), `compound-four 1 --hard --rate 2` (14,872 ticks: `FUZZ_CAP=2400`), the same four missions on seed 1 with `--rate 5`,
`rt-…` seeds 3 to 6, and the soak `fuzz.sh <1 to 20> --minutes 3` (a seed that parts in the panes alone after a load is kept too: exit 4).

### What the two fuzzes have found (to cut 7.16)

| what | seen | state |
|---|---|---|
| an interrogation, a grab not crossing | the soak on 7.11, 8 seeds of 26 | gone on 7.13 |
| a QRF man's door tools took the boot at a locked door where the reference worked the handle | the soak, seed 9, tick 9264 | the bodies' mend; seed 9 equal on all 10,800 ticks on 7.16 |
| the TypeScript's own put-back of a save leaves a pane not the saved moment's | the soak, seeds 7 and 8 | the reference's; equal with the panes left out |
| a throw in the changed-only apply 617 ticks after a load (`orders::SquadOrdersLogItem.who`) | the squad fuzz, depot-four:2 Hard, frame 5671 | serials colliding after a load; gone on 7.16 |
| the follow camera after a load is not the reference's (its target 2 mm off), so the cursor's point on the floor is not, and the player's first round after a load is another round | the same run on 7.16: the host's half from tick 5055, the block from tick 5841 | the host's; MENDED: a save between two ticks takes the host's rows first, a load does not lay the camera of now over the saved moment's. The same run: equal on all 8,006 ticks, 0 paths not known at four half minutes |
| a save by hand stood twice in the Rust game's chronicle | every squad run with a save; no tick's line shows it | the host's own; mended (`snapTake` holds the row out of the walk) |
| a field a tool wrote by name and a rule then put back stays the tool's on the puppet (the every-tick export's memo holds what was sent before the write) | the soak's `holding-while-busy` on four seeds: `player.crouch = true`, then `startGrab` stands him up | the bridge's; stood in for (`face_puppet_spoil` after a walk that poked) |
| a tool outside tools/qa writes the game by name and the walk does not watch the name | perturb.ts's `tune.qrfEta = 14`: the Rust game dealt the response team at 75 s | the file says its names (`__tpsToolWrites`) |

Numbers, seeds and what each green could be wrong about are kept outside the repository with the runs.

## Two ways

**Fast (the default).** After a step the FILTERED export is applied (`face_puppet` → `applyGame(… puppet …)`: the listed fields of what changed, and
the step's out events on its tail). Nothing is ever exported from the puppet. What the TypeScript round a step does to the game goes to the Rust game
as a call when it is made:

| what the TypeScript does between two ticks | the call |
|---|---|
| the scripted player gives an order (`orders.give`) | `face_give` where the face has words for its target (the floor, a door, a man by place); the menu's own `give` on the puppet otherwise, whose rows ask the Rust game (`face_ask`) |
| a lesson's player picks a row of the nearby list (`selectVerb`) | `face_hostwrite … select_verb` |
| a lesson's player feeds the light meter (`player.visibility`) | `face_meter`, when the number differs from what the last apply left |
| the autosave takes a checkpoint (the dress step's `saves.tick`) | `face_checkpoint`; an authored one on a level on rails: `face_stop_program` first |
| a question floods a room (`fight.roomAt`) | `face_room_at`, asked of the Rust game FIRST; where it flooded, the puppet takes the whole state and the TypeScript's own ask floods nothing |
| the host lists the box world (`engine.world.upload`) | `face_upload`, and the TypeScript's own listing runs too (the list of every box and the ray grid are the TypeScript's own on the filtered path) |
| the host's camera asks the game where to look | `face_follow` after every step |
| a checkpoint put back, a restart (the TypeScript puts another game in the puppet's place) | A NEW START: the puppet exported whole once, a new Rust game made from it |

Memos a driver's read fills on the puppet (`Cover.openings`, `DialogueMod.room_rects`, `WorkThings.cache`, frag.ts's `walls_for`) are NOT handed over.
Each is "built if it is not yet" in its Rust body (cover/p_02.rs:69, dialogue/p_02.rs:146, workthings p_*.rs:180 to 213, frag/p_01.rs:124), and the
replay loop (`run.sh`, `wasm-run.ts`) has never handed them and is in step on every mission. `doorTools.knownShut` also writes when asked (`learn(d,
side, null)`: it forgets a door found open): not handed either, same ground.

A WHOLE apply (`face_export` → `applyGame`) is made where something is about to read what the filtered list is known to lack: when a save is taken
(the save keeps the puppet's objects), when a man goes down, a door order begins or ends, the mission's stage moves or one of the ends the scripted
player asks for is set, after a room is flooded, and every `--whole-every N` ticks if asked. About 5 to 25 a mission.

**Audit (`--audit`).** Beside the filtered apply the whole state is applied every tick, and as each tick opens the puppet is exported whole and held to
the bytes last applied. What differs outside the host's half is something TypeScript wrote into the game between two ticks: named, counted, and handed
to the Rust game whole (`face_import_into`). Two walks of the state a tick (about 200 to 350 ms): the way to find a write the fast way has no call for.
`--handback N` hands the whole puppet over every N ticks whether anything was written or not (`HB_AT=<ticks>`, `HB_DEBUG=<dir>` keeps the Rust game's own
dump before and after). `--stale N` (fast): every N ticks the puppet is exported whole and held to the Rust game's whole state, and the fields in which
the puppet has fallen behind are listed by name (`--stale-from K`, `--stale-show N`): the way to name a field the filtered list lacks.

## What is compared, and against what

The reference is the same mission and seed played by the plain TypeScript game (`mission.ts`). `play.ts` holds to it, every tick:

- **the host's half** as the tick opens: the very bytes handed to the Rust game (`hostHalf`: a record a seat, every camera and Input row) against the
  reference's `h<k>.bin`. The first tick it differs is the first tick the puppet showed the player something the TypeScript game would not have;
- **the driver's calls** made since the last tick (orders, checkpoints, floods, picks), in the replay loop's own words, against the reference's
  `gives.txt` and `hostwrites.txt`. An order is no key: warehouse-four's die parted 17 ticks after an order the plain play gave and this play did not,
  with the host's half equal throughout;
- **the die and the block** as the tick returns (the 12 parts but `poses`, which parts by design from the first death: README.md);
- **the end**: taken / failed, the second, who is up, the tick.

A run whose host's half was compared on fewer ticks than were played exits 3 ("NOT A PASS"): the compare cannot die silently. A run with no `--ref`
says NOTHING COMPARED and exits 3. Exit: 0 equal to the end and ended the same way · 1 parted · 2 the Rust tick panicked · 3 a side could not run ·
4 equal every tick, the end differs.

## A tick, the fast way

1. The scripted player presses its keys and steps its world (`headless.ts W.step` → `Game.update`); its own lines run, the step is `RustHost.step`.
2. The light meter, if the lesson's player wrote it. The check's hook (the driver's calls and the host's half held to the reference).
3. `hostHalf` → `face_host_half`. `face_step`. The check's hook (the die, the block's hashes). `face_lights`, `face_view`, `face_follow`.
4. `face_puppet(0)` → the filtered apply; the block copied; `asks.after()` (a row or a posture the apply made gets the closures that ask the Rust game).
5. A whole apply if one is owed. The out events dispatched (`out-dispatch.ts`) on the puppet's host objects, in the order the rules made them.
6. The dress step: `Par.tick`, `Verdict.tick` (each in a `try`: they are the debrief's), the autosave (`saves.tick`), whose checkpoint is made on the
   Rust side.

Before the first tick the start is exported whole once (the Rust game is made from it), the filtered export's memo is filled, and the every-tick path
is run four times over every row (`face_puppet_warm`) so that the first ticks do not pay for its compiling.

## Out events

Dispatched every tick since this host. The arm that CHANGES THE GAME is the checkpoint: `CheckpointsKeep` (a rule on a level on rails asks for an
authored checkpoint) and `CheckpointsRequest` set the save controller's own flags, its tick in the dress step takes the checkpoint off the puppet (its
snapshot in memory, its chronicle row), and the host makes the same on the Rust side. For an authored one the level's program is stopped on the Rust
side BEFORE the take (`face_stop_program`), so that the controller finds no program running: a director run the apply made has none of the TypeScript's
own (director.ts:525 threw on `R.drills`). tut-solo's four authored checkpoints are taken on the reference's ticks with its labels.
`HostApiRestartPicture` and `SessionRecorderCut` go to the host's picture and journal and write no game state.
The harness's stand-ins lack some receivers (`audio.rain`, `smoke.obstaclesChanged`), which the generated dispatcher calls unguarded: such an arm is
counted and not called again. `WallWearRound` threw (`box.h` undefined: the event's box came back as a row the every-tick apply had made on the puppet and not filled); with the changed-only export the row is whole, the same play (night-breach, die 20942, tick 1293) dispatches it, and no arm threw in 84 + 42 missions (`RH_OUT_SAY=1` says the first throw of any arm with its event).
an object).

## The harness's word

`TPS_GAME` in front of any headless tool run from the repository's root (`tools/blitz/rust.sh <word> <command>` is the same thing from anywhere). The
bridge's preload is listed in bunfig.toml and does nothing unless the word asks (below), so the word alone is enough, for the tool and every child `bun`
it starts in the root. ONE FILE decides which game a `standUp` stands: tools/blitz/word.ts.

| | |
|---|---|
| `TPS_GAME=rust` | the game `tools/qa/headless.ts standUp` returns is a puppet of the Rust game (the fast way) |
| `TPS_GAME=audit` | the same, audited |
| `TPS_GAME=ts` | the SAME STAND stepped by the TypeScript tick: the other side of a tally |
| `TPS_GAME=table` | the Rust game, but for the checks `qa-kinds.json` keeps on the TypeScript tick (below) |
| `TPS_GAME=none` | nothing of this is loaded: the harness as it always was |
| unset | `DEFAULT_WORD` of word.ts: `'none'` today. THE FLIP is that one line, to `'table'` |

The stand under `rust`, `audit` and `ts` is mission.ts's: the game rolls a die of its own where the tool gave it none (seeded off the tool's `Math.random`
as the game is stood, so `playthrough.ts`'s seeds roll different dice and both words roll the same), and the dress rows are down. So a
tally under `ts` is not the tally the tool makes with no word (which rolls `Math.random` and dresses); it is the one to hold the Rust game's beside.
(`TPS_RUST` cannot be the word: src/sim/sim.ts refuses it as a retired one.) `TPS_GAME_WHOLE=N`: a whole apply every N ticks.

**The preload.** The bridge reads the simulation's own module variables through a window `expose.ts` opens as each source file loads, so it must be
registered before any source file is read. bunfig.toml lists `tools/blitz/preload.ts` for every `bun` started in the root; it imports word.ts (one small
file, no source file) and, only where a word asks, `expose.ts`. With no word, measured: `probes.ts sticky` the same output line for line (25.0 s without
the list, 26.9 s with, one run each on a loaded box); `PT_DIGEST=1 playthrough night-breach:1` the same digest and the same whole record; `bun run
tools/build.ts` the same 137 files to the byte without the list, with it, and with it ACTIVE (`TPS_GAME=table`); the dev server's `/` and its script
the same bytes the same three ways (the bundler does not run a runtime plugin). A child started outside the root, or under node, has no preload: under a
word its `standUp` throws and says so. (A bunfig given by `--config` from another folder resolves the server's own plugin against that folder.)

**A frozen run** (`TPS_PROFILE=frozen`) is the TypeScript tick's whatever the word: the frozen stand holds settled switches off their page values, and a
settled switch is a constant in the Rust game. word.ts forces it and the closing line says so (`ALL n games of this run were FORCED TO THE TYPESCRIPT TICK`).

**The closing line is the top process's.** A tally plays its games in children whose output goes nowhere. The preload of the first process of a run names
a file in the environment (`TPS_GAME_TALLY`); every process below adds its counts as it ends; the top one sums them into the one line.

**The word by a table (`TPS_GAME=table`).** `tools/blitz/qa-kinds.json` has a row a probe of `tools/qa/probes.ts` (its kind: what it does to the game
between two ticks, written by `qa-census.py --kinds`) and a row by hand for a stand-alone tool that cannot run on the Rust game yet, with its reason.
As a game is stood, the probe running then (the runner names it) or the tool's own file decides, in this order:

1. a probe with NO ROW (added since the census, or one the census found no line for: the table's `unread`): the TypeScript tick, and the closing line
   counts and names it (`N checks with no row: …`);
2. kinds iii and iv (a check that CALLS the tick's rules, hangs its own function on a game object, checks a TypeScript function with no game, or holds
   the game to a walk of the JavaScript objects): the TypeScript tick;
3. `pins`: a check that RAN WITH A SETTLED SWITCH OFF ITS PAGE VALUE (a row the Rust game folded to a constant), from a run of the gate's families with
   `RH_FOLDED`: 185 probes; the TypeScript tick;
4. `waits`: a check SEEN RED ON THE RUST GAME and green on the TypeScript tick, by hand, with what it waits on; the TypeScript tick until its row is taken out;
5. everything else (kinds i and ii that are in none of the lists): the Rust game;
6. a stand-alone tool by its row: `ts`, or `none` (stood with no word at all: `tools/qa/coop-check.ts`, whose second engine is node), else the Rust game.

Each check kept off the Rust game says so as its first game is stood (`[the game: <name> is stood on the TYPESCRIPT TICK by the table (kind iii); its
verdict is not the Rust game's]`) and the run ends on one line that counts both sides and names them (`=== the game, by the table: … ===`), so a green
of the old tick is not read as the Rust game's. HOW a kept check is stood is one constant of word.ts, `KEPT_STAND` (`TPS_KEPT=ts|none` for one run):
`'none'` (the default) exactly as with no word, to the byte, so nothing a kept check recorded moves when the default word becomes `table`; `'ts'` the
word's own stand (the game owns its die, the dress rows down), under which every exact number recorded with no word moves once (seen on the first
families tried: `weapons-identity` 2 of 2 red, `workstow` 2 of 5, green both with no word and under `'none'`). A FROZEN run is always stood as with no word.

A stand-alone tool: by its row, `rust` (seen to play the same on the Rust game: `playthrough.ts`, `seats-run.ts`, `bench-seeds.ts`, `thrice.ts`,
`die-check.ts`), `ts` (`tour.ts`, `soak.ts`, `seats-check.ts`, `lesson-run.ts`) or `none` (`coop-check.ts`, `exact-check.ts`, `dice-check.ts`). A TOOL
WITH NO ROW is kept on the TypeScript tick and counted like a probe with none (485 files of tools/qa stand a game; run as its own main a check file has
no probe name). One of this folder's own tools with no row stands its games itself and gets no word. A process that is not bun (a child under node)
has no bridge: its game is stood with no word, said and counted.

`tools/blitz/table-check.ts` (by hand, after a probe is added, renamed or deleted) holds the table to `probes.ts list`: red when the runner lists a name
the table has no row for (as a probe, a family that only runs its members, or an unread name) or the table has a row, a pin or a wait of a name the
runner lost. The census on this tree: 1,512 names known; 1,258 probes read (iii 766, iv-a 186, ii 114, 0 83, iv-b 61, iv-c 39, i 9), 150 families,
105 unread. BY THE TABLE: 61 probes on the Rust game, about 1,115 kept on the TypeScript tick, 82 that stand no game.

The `waits` are from a run of each of the 101 probes of kinds i and ii that no pin holds, one at a time under the table word, a red one again with
no word: 61 GREEN ON THE RUST GAME; 35 of kind ii red on the Rust game and green with no word (they write the game's fields by name between ticks);
2 of kind i the same (`charmats-cast`: a cast man's cloth tint; `standupgc`: 25 games stood back to back grow the heap 125 MB where 60 is allowed);
2 over a 240 s cap under both; 1 red under both (`entrypoints-count`: its own red on this tree).

`tools/qa/missions-gate.ts --seeds 1 --quiet-seeds 1-2 --jobs 1` by the table: every line PASS (1,210 s on a loaded box): the 21 squad missions on
the Rust game, the 16 quiet runs on the TypeScript tick (`solo-night.ts` has no row: a quiet mission has never been played on the Rust game).

## The flip

`DEFAULT_WORD` in tools/blitz/word.ts, `'none'` to `'table'`: one line. From then a headless tool run from the root with no word is `TPS_GAME=table`
(`TPS_GAME=none` is the harness as it was). What changes at that moment, and what does not:

- **On the Rust game**: the 61 probes above; `playthrough.ts` (so the squad half of `missions-gate.ts`, and every tally), `seats-run.ts`,
  `bench-seeds.ts`. A tally's record on the Rust game differs from the TypeScript's in the places listed under the gate's tools (the tool's own
  watchers and wrappers) and its digest in two fields (`decals`, `civilians[i].dice`), and its stand is the word's (the game's own die, the dress rows
  down): a number pinned from a tally with no word is recorded again.
- **Kept on the TypeScript tick, stood exactly as with no word** (`KEPT_STAND = 'none'`): about 1,115 probes with a row (kinds iii and iv, the 185
  pins, the 40 waits), the 105 names with no row, every stand-alone tool with no `rust` row (`tour.ts`, `soak.ts`, `seats-check.ts`, `lesson-run.ts`,
  the game server and its bots, `solo-night.ts` and every check file run as its own main), a child under node, and any run with
  `TPS_PROFILE=frozen`. Their verdicts do not move. Each says so as it stands its first game.
- **What a run prints**: a line a kept check, and one closing line from the top process (`=== the game, by the table: … ===`). A tool that reads
  another tool's whole output to the byte would see them; the gate's reader (tools/qa/gate.sh) matches verdict lines and does not.
- **The preload is active in every `bun` started in the root** (it reads schema.json and adds a line of getters to each simulation source file as it
  loads). Measured the same to the byte with it active: the production build, the dev server's page and script, and a frozen tally's whole record.
- **Cost**: a probe on the Rust game pays a start (three exports) for every game it stands and about twice a tick; the 61 took 5 to 40 s each alone.

- **One process, both games.** The dress rows are one table a process and a game under the word is stood with them down: a kept check stood after a
  game on the Rust game found them down (the `breaks` family: 6 of 11 red behind one Rust-side check, each green alone). The harness now puts them
  back before it builds a game with no word; `breaks` by the table is ALL PASS on that tree.

**SINCE THE SETTLED ROWS ARE FOLDED** (tools/qa/fold.ts, joined after everything above was measured): the fold is laid for the whole process whenever
the word is `rust`, `audit` or `table`, so a check KEPT on the TypeScript tick is no longer stood "as with no word": it runs on the FOLDED TypeScript
tick, and a check that writes a settled row is refused there. On the joined tree, with no word `sticky` and `breaks` are ALL PASS; by the table
`sticky` is 10 of 10 red (all ten kept, all with writes refused) and `breaks` 6 of 11 (every red one has a refused write: `CUT.on`, `EVICT.on`,
`FURNBLAST.on`, `BREAKS.look*`; no green one has). Those are reds of the folded TypeScript tick, not of the Rust game, and they are the flip's
cost until those checks are staged again. The fold asks the one switch for the default word (`DEFAULT_WORD`), so the flip is still one line. The
table's `pins` (from the collector that came before the fold) are now what the fold's own line a probe says (`refused` not empty) and should be made
again from it: not done. Every "TypeScript tick" beside a number above is the UNFOLDED one (the word's stand, or no word).

The 62 probes the table stands on the Rust game, each alone UNDER THE FOLD (by the table; a red one again with `QA_FOLD=1` on the TypeScript tick,
the reference): 42 GREEN ON THE RUST GAME; 20 red there and RED ON THE FOLDED TYPESCRIPT TICK TOO; none red on the Rust game alone. The 20 are the
table's `both`: such a check says so as it stands a game and the closing line counts it (`N checks seen RED ON THE FOLDED TYPESCRIPT TICK TOO`).

Not shown: the gate's whole list (`probes.ts all-quick`) in one process by the table; the tour's and the soak's gate forms; the kept checks under the
fold beyond two families; the 40 `waits` and the 185 `pins` again under the fold.

## The die

The game owns its die under every word, as the page's game does: no seam lets the Rust tick draw a function a tool put in `Math.random`'s place. (The
TypeScript rule "a die of the game's own is used only while `Math.random` is the function it was when the die was made", src/math/dice.ts, is the
TypeScript tick's and goes with it.) A tool that wants a given stream AFTER the game is stood says so through tools/qa/headless.ts:

- `seedGame(game, die, seed)`: with no word, `Math.random = die` and nothing else (what the site did); under a word, also the game's own die put at the
  start of `seed`'s stream, on the puppet (in place: the die's word and count may live in the game's block) and in the Rust game (`RustHost.seedDie`: the
  `dice` part of the block written in place; no new call of the face's). Returns the put-back.
- `dieBack(game, was)`: `Math.random = was`; under a word the game goes on with its own die.

`tools/blitz/die-check.ts` (two seconds of play, the stream named, two seconds of Sam's fire): with no word the helper's line is the plain site's; under
`ts` and `rust` the helper leaves the game's die at the same word after the same 18 rolls (ad0e9341); the plain site leaves 1 roll under `ts` (the tool's
function took 18) and 19 under `rust`. A seed put in BEFORE the stand needs no helper: the stand seeds the game's die off one draw of it.
The sites, by reading: 782 assignments in 349 files; 137 seed or function sites after a stand (and 137 put-backs); 28 of them in the 11 families that
have a probe on the Rust game by the table. They are not converted: the census and the codemod's plan are with the swap's notes.

## The gate's playing tools under each word (the changed-only export, the joined bodies; one at a time, the box at a load of 9 to 15)

| tool | `ts` | `rust` | what is red under `rust`, by kind |
|---|---|---|---|
| `playthrough.ts --seeds 1-4 --jobs 2` (84 runs) | 1,452 s | 2,498 s | 84 of 84 END THE SAME; 14 whole records equal; 7 places differ (`smells.#.detail` 68 runs, `destr.fixWhy` 11, `destr.skins.*` 7, `deaths.#.hole` 3, `entries.#.blind` 1): the tool's own watchers and wrappers |
| the digest of every half second (`PT_DIGEST`) | | | equal on 28 of 84; two fields part and no other in 11 missions looked at: `decals` (the host's decal list holds marks the out arm lays and nobody tags as the picture's) and `civilians[i].dice` (the same 32 bits, unsigned on the puppet) |
| `bench-seeds.ts --twice snow:10`, `dark:1` (the same run twice) | 45 s, 38 s PASS | 113 s, 84 s PASS | the same digest under both words; "Sam hit" is the tool's own tap (2 | 0) |
| `seats-run.ts` | 87 s PASS | 170 s PASS | |
| `seats-check.ts` | 319 s PASS | 477 s, 3 red | `rollback`, `twin`: a word made by walking the JavaScript objects (and `walkAhead` on the puppet); `down`: calls `hitPlayer` |
| `mpserver-check.ts` | 226 s, 2 red (clocks) | 443 s, 4 red | `same fight`, `late joiner`: the check runs on the wall clock and the games stepped 650 to 917 of 1,200 ticks, at 120 to 175 ms a tick: THE RUST GAME NEVER LISTED ITS WORLD there (a networked tick calls `world.relist()`, which had no tap), so its state grew 35 KB a tick and the every-tick export slowed with it. Tapped since ("Numbers", below); `lag`, `ahead` red under `ts` too |
| `tools/qa/coop-check.ts` | 209 s, 4 red | 520 s, 4 red | its second engine (node) has no preload under either word: by the table it is stood with no word (ALL PASS, 226 s). Its bun half alone (`--one hostage-rescue 1`) under each word: the same row, and the chained raw float bits equal on all 107 half seconds (it was 3,797 frames against 10,528: the tool's counter on `Math.random` made every die put back a new function, so the TypeScript tick left the game's die; under a word it now gives the same function back) |
| `dice-check.ts` | 24 s, 1 red | 51 s, 2 red | `default`: the word gives the game a die (both words); `same seed`: provokes its fight by a rule (`teleportPlayer`), not the die. The tool checks the TypeScript die's own rule: by the table it is stood with no word (6 of 6) |
| `tour.ts --runs 1 --seed 1 --quiet` | 22 s PASS | 52 s, 9 FAIL | kind iii: the showcase's own Demo (src/game/demo.ts, host-side) calls rules and writes fields on the puppet between ticks; first: `stop2-hand` (Sam's hand never on the fire exit's leaf), then the takedown, the hold, the firefight, the room clear's drill. By the table (its row `ts`): PASS |
| `soak.ts --seeds 1 --minutes 3 --quiet` | 34 s ALL PASS | 62 s, dies at 2.5 min | kind iii / iv-a: its stimuli call rules on the puppet: first `f2530 exception (stim:mission-step): Attempted to assign to readonly property` (player.ts:1107 `w.cam.peekOffset = null` under `teleportPlayer`: an object the apply left frozen), then `S.u` and `h.where` undefined (a `PoseSet.home` the apply left undefined, skeleton.ts:203), the last one uncaught. By the table (its row `ts`): ALL PASS |
| `knobs-check.ts` | PASS | PASS | stands no game |
| `tools/blitz/coop-check.ts` | 430 s, its 3 lines red as its header expects | | stands no game through the harness and reads no word |

The gate's forms of the tour and the soak (`--runs 3`; three seeds, twelve minutes) were not run under `rust`: the short forms are not clean.

## A tool that stands many games, and what it does with them

- **The second game of a process.** The simulation keeps much of its state in weak tables beside its objects, one table a process (src/math/side.ts), and
  the bridge listed such a table by every key still alive: a game stood after another was exported with the other game's men, routes and players as
  rows of its own, and played another game (seats-run's second game rolled its die one time fewer at tick 787). The other game need not have stepped
  (the game server stands one only to write its recipe). While the bridge walks a game a table now lists that game's keys only (the stand-in
  `ownKeys`, below). `tools/blitz/thrice.ts` stands one two-seat fight three times in a process and holds
  the digest of every second, and with `--ticks` every tick, to the first game's: three games, one digest, each equal to the `ts` game on all 2,400
  ticks; `--weave` stands the later games together and steps them in turn. The modules' own logs and tallies (`destr_log`, `holdfire_log`, `scene_collision.sensed` …) are still a process's, on both sides: they grow from
  game to game and no rule reads them.

- **The slots.** rust/sim has four. A tool that stands a fifth game in one process (the probe runner stands hundreds) gets the slot of the game stepped
  longest ago: that game's puppet takes the whole state first, and if the tool steps it again it starts anew from its puppet.
- **The host's hooks are hidden fields** (`stepSeats`, `rustHost`, the wrappers on the menu's `give`, `fight.roomAt`, `selectVerb`, the lamps' update, the
  listing, the saves): a walk of the game's own keys (rollback.ts `stateHash`) does not meet them. A start takes the hooks again: a mission the tool
  started after the stand has its own menu, fight and saves.
- **The trace.** `RH_TRACE=<file>`: a line a game as the process ends (the tool's file that stood it, the probe running then, ticks stepped, whole
  applies, calls, starts, slots given up, how often the game's die had rolled between two ticks: a rule of the tick run in TypeScript on the puppet).
  `RH_SITES=1` with it: every tick counted by the place in the tool that asked for it. `RH_CAP_AT=<ms>`: the tool ends itself at its next tick after that
  moment (a signal's handler waits for the tool's own synchronous code).
- **A quiet run of steps** (`W.steps(n, dt)` in tools/qa/headless.ts: the same as `W.step` n times, said by a tool that runs none of its own code
  between them). The host hands its half over, steps the Rust game and asks it the camera's two inputs (`face_look`: where to follow, the aim point);
  nothing is exported or applied until the run ends, when the puppet takes what changed over the whole run in one filtered apply, the run's out events
  are dispatched in order and the dress step runs once. What differs from the same steps one by one: an autosave due inside the run is taken as it
  ends, the par clock and the verdict tick once, an out event reaches the host's objects late. ASKED FOR BY NAME (`RH_QUIET=1`) AND NOT YET THE SAME
  PLAY: the Rust tick writes the lens's own row at times (a start snaps it onto the man) and a quiet run does not bring it across, so the host hands
  its own lens back. `tools/blitz/quiet-check.ts` (the office walked by input in six runs, 2,340 ticks): the die and the block equal on every tick
  under `ts`, quiet and with an apply every step; the host's half of the quiet run differs from tick 2 (`FollowCamera.target`); 3.4 ms a tick quiet
  against 8.5 with an apply every step. It wants the camera's rows brought across on the quiet path. `tools/blitz/quiet-loops.py` rewrites a loop that
  only steps into that call (the exact shape only: the counter from 0, a bound of plain arithmetic; a condition that reads the game is tool code).
- **Two plays of one tool held to each other tick by tick.** `RH_TICKS=<folder>` under `ts` and under `rust`: a line a tick (the die's count, the host's
  half as handed in, the orders given so far, a hash of each part of the block but `poses`). For a tool whose own stand `play.ts` cannot make again
  (a tally's run: the word seeds the game's die off the tool's own `Math.random`). `RH_DIE_SAY=<file>` writes that die; `RH_HALF=<ticks>` prints the
  host's half of those ticks a field a line.
- **What a check does to the game between two ticks**: `tools/blitz/qa-census.py` (a probe a row, by kind). `--folded <the switch list>` adds the checks
  that put a SETTLED switch (a constant in the Rust game) on its other side, by reading (FOLDED.md); `RH_FOLDED=<the switch list>` with `RH_TRACE` counts,
  as a check runs, the ticks each settled row stood off its page value (`folded` in the game's line): what the check really pins.
- **`RH_TIMES=<file>`**: as a tool ends, a line a game stood on the Rust game: its ticks, the clock from its first step to its last (the tool's own code
  between two steps included) and every ms the host spent, by what it did.
- **`RH_OUT_SAY=1`**: the first throw of each out event's arm with the event and where it threw. **`RH_OFF=own`**: the weak tables listed for every game again.

## Stand-ins (each marked `STAND-IN:` in the code with its reason and where it belongs)

- **A call's game where its receiver is not the game** (rust-host.ts `surfaceOpen`, `BUSY`; `step` re-points the surface's fallback). surface.ts finds a
  call's host by `self.rustSurface`, an argument's, else THE HOST THAT ATTACHED LAST: `g.quiet.current()`, the first line of every `Game.update`, so
  crossed from the first game's own update into the sixth game stood. Here nothing crosses while any hosted game of the process is inside its update,
  and the fallback is the game that stepped last. A tool's call on an object of a game it did not step last still goes to the wrong game. Belongs in
  surface.ts: the host by the receiver's game, never a fallback.
- **The window's guards under `table`** (word.ts `spellForWindow`, preload.ts). expose.ts puts the surface's guards and the taps in only where `TPS_GAME`
  is spelt `rust`; by the table the preload spells it so and keeps the word asked for in `TPS_GAME_BY`. Belongs in expose.ts.
- **What a tool writes into rust/sim's block** (rust-host.ts `blockBack`): a door's swing and latch, a man's place, the die. The puppet's copy of the block
  is held to the Rust game's as a tick opens and before a call crosses, and each part that differs (but `poses`) is handed over whole. Belongs with the
  scan (a write by name of a field that lives in the block).
- **A file's `loadState` on the puppet known by the caller's name on the stack** (rust-host.ts, the wrapper on `restartEncounter`): its restart must not
  cross. Belongs on the call surface: `loadState` a row the host answers.
- **The highlighted row put back on the nearby list's own row** after an apply (rust-host.ts `hoverRow`): the bridge carries `Game.hover` as a value.
  Belongs in the schema (a reference into `nearby`).
- **The mission record made anew** when the Rust game restarted inside its tick (rust-host.ts `tail`): the apply writes the fields into the record
  that stood, and a run's beginning is known outside the tick by the record's identity. Belongs in the importer (a restart's out event, or the record
  as a row).
- **A whole apply when a step's apply made a man** (rust-host.ts `tail`): a made row has the every-tick list's fields only. Belongs in the importer
  (a made row is filled whole).
- **`face_loaded`, `face_saves_in`, `face_saves_asked`** (the page's own stand-ins in rust/game_face/src/page.rs, now the harness's too): what the save
  controller does to the game round a load, its shelf as the tick reads it, what the tick asked of it. Belong in rust/game's savestate.
- **The memo spoiled after a poke** (rust-host.ts `poked`; rust/game_face/src/memo.rs `face_puppet_spoil`): after a walk that wrote a field into the Rust
  game by its path, the next every-tick export sends every row. Belongs in the generated poke: a write by path spoils the memo's row of that path.
- **Tables keyed by an identity serial that the schema does not mark** (diff.ts `KEYED_BY_SERIAL`: `gunhold.knowns`, `guards.holds_called`). Belongs in
  schema.json's `serial`.
- **A refused ask answered as nothing** (rust-host.ts `askNone`, the page's own table copied): `face_ask` answers a number for a row, a seat or a
  question the Rust game has not got. Belongs in ask.ts: one reader of the number for both hosts.
- **The game's die seeded on the Rust side by a write into the block** (rust-host.ts `seedDie`: the `dice` part found by its name, the count and the word
  by their offsets). Belongs in the face: one call that seeds the die of the game in a slot.
- **The list-alone call tapped by hand** (rust-host.ts: `world.relist` → `face_upload`, beside the tap on `upload`). Belongs with the generated taps.
- **The bridge's own tools known by their path** (word.ts: a tool of tools/blitz with no row gets no word). Belongs in the table, a row a tool.
- **A box whose markings went is `marks: null`** after an apply; scene/boxes.ts:841 tests `!== undefined`. Made undefined before the puppet's own listing
  (rust-host.ts). Belongs in import.ts.
- **The whole applies owed** (the list above): each stands for fields the filtered list lacks. Belongs on the generated list.
- **The scripted player's order by `face_give`** on the fast path: the menu's own `give` reads the puppet before it asks a row (orders.ts:2285, the floor
  the team has seen). Belongs on the generated list.
- **The memo's buffers trimmed** after every filtered export (harness.rs `trim`): `export_puppet` hands its scratch vector to the memo with whatever
  capacity it had. Belongs in the generated export.
- **`face_stop_program`** before an authored checkpoint's take (harness.rs): belongs with the save controller's port.

## What does not work on a puppet

- **A lesson's own checks** read records the filtered list lacks (`char.anim.crouch`, a man's torch, the team pace's own counters): see the table.
  `--whole-every 1` is the slow way round.
- **A module's own counters that the TypeScript tick bumped** (`DESTR_STATS`, `FRAG_STATS`, `TEAMPACE_STATS` … read by `playthrough.ts`'s report)
  stand at zero: the Rust game's counters are not the TypeScript module's tables. A tally's headline (how it ended, the second, who is up) is the
  game's; its counts of rounds, bangs, papers are not: see the tallies.
- **Two seats**: the first apply throws (below).
- **`tut-solo`'s last 510 ticks, `tut-kit` from 5302**: the cursor parts (below).

- **A TypeScript body that MAKES THINGS on the puppet between two ticks.** A kept method with no row of the call surface (a check calling
  `WallBreak.openCells` by hand) runs as TypeScript on the puppet, and what it makes there (a wall's rubble, each piece a box) are objects the Rust
  game has no row for. The plain fields it writes go to the Rust game by the scan, as any tool's write does; a watched leaf that NAMES one of the new
  objects does not: the scan refuses it (`the scan: a write on the puppet was NOT SENT…` on stderr, `WRITES NOT SENT` on the exit line), because the
  bytes would hold a number past the end of the Rust game's table. That was a panic four ticks later (`edge_look` indexing the boxes with a rubble
  record's number, `cutboxes-lines`), far from the write. The call surface refuses such a row as an argument the same way. A check that needs the walls
  opened asks through a function that has a row (`Destruction.blast`, `WallBreak.round`, `WallBreak.blast`), so the Rust game makes the pieces.

## Numbers

The fast way, release module, 4 cores. THE MACHINE WAS NEVER QUIET for the longer runs (a load of 8 to 12 from other work), so each column says its load.

| ms a tick, mean | room-clear-four:1, 3,006 ticks, load 9 | night-breach:1, 1,413 ticks, quiet | the audit before (room-clear-four) |
|---|---|---|---|
| the scripted player and its world's own lines | 0.87 | 0.50 | 0.86 |
| `Game.update`'s own lines | 0.06 | 0.03 | 0.03 |
| the write-back: the puppet's export · the compare · the hand-over | none | none | 125.1 · 10.6 · 0.8 |
| the host's half | 0.11 | 0.08 | 0.6 |
| `face_step` | 2.00 (worst 83, tick 1) | 1.10 (worst 19.5, tick 1) | 2.12 (worst 29.5, tick 1) |
| lights, view, follow | 0.005 | 0.004 | 0.01 |
| the filtered export (about 170 to 230 KB a time) | 1.78 | 0.93 | none |
| the filtered apply | 4.19 | 1.56 | none |
| the whole export and apply, spread over the play (5 to 25 a mission, 100 to 280 ms each) | 0.07 + 0.51 (9 of them) | 0.03 + 0.25 (4) | 8.6 + 119.4 every tick |
| the block's copy | 0.31 | 0.16 | 2.1 |
| out events dispatched | 0.01 | 0.006 | not dispatched |
| the dress step | 0.25 | 0.15 | 0.23 |
| the driver's calls | 0.01 | 0.01 | in the write-back |
| **all in** | **10.4** | **5.0** | **273** |
| the check (not the host's): the host's half read and held · the die and the block's hashes | 1.36 · 0.68 | 0.14 · 0.52 | |
| wall for the mission, the check included | 37 s | 8.0 s | 822 s |

**By the host's own record** (`RH_TIMES`), the bodies and cut joined since, seed 1, the box at a load of 16 to 19 from other work (so every number is
high; the plain `ts` play beside each under the same load):

| ms a tick | night-breach, 1,464 ticks | room-clear-four, 3,003 | depot-four, 5,355 | the game server's game before the list was tapped (733 ticks) |
|---|---|---|---|---|
| the host's half | 0.33 | 0.36 | 0.28 | 1.7 |
| `face_step` | 2.80 | 3.13 | 2.19 | 16.9 |
| the filtered export | 2.63 | 2.63 | 2.25 | **123.2** |
| the filtered apply | 3.61 | 3.66 | 2.99 | 23.0 |
| whole exports and applies (5, 9, 12 of them; 0) | 0.80 | 0.63 | 0.45 | 0 |
| the block's copy | 0.66 | 0.49 | 0.43 | 2.2 |
| out events · the dress step · the driver's calls | 0.03 · 0.38 · 0.02 | 0.02 · 0.19 · 0.01 | 0.01 · 0.08 · 0.01 | 0.07 · 0.82 · 0 |
| the start, once, spread over the play | 1.45 | 0.48 | 0.21 | 2.9 |
| **the host, all in** | **12.7** | **11.6** | **8.9** | **170.8** |
| the clock from the first step to the last, a tick | 14.2 | 12.8 | 9.7 | 174.6 |
| the tool's wall, `ts` then `rust` | 16.4 s, 24.9 s | 26.1 s, 42.0 s | 29.8 s, 55.8 s | |

A tally's tick is the step, the every-tick export and its apply in about equal thirds; the whole applies, the calls and the out events are small.
THE GAME SERVER'S TICK was something else: a networked game's tick lists the box world by `world.relist()` (src/net/step.ts), the host tapped only
`upload()`, and so the Rust game never listed its world: every tick's frame boxes stayed in its tables (the whole state 10.7 MB at tick 100, 28.4 MB at
tick 600), the every-tick walk met them all (its time 9, 17, 27, 25, 38, 41 ms over the first six hundred ticks; its size a steady 185 KB) and the step
and the apply slowed with it. `relist` is tapped now (`face_upload`): 4.0, 2.9, 4.4, 7.2, 4.9, 2.3 ms at the same load, and the chained raw-bits word
of the server's game after 600 ticks is the `ts` game's before and after the mend. `mpserver-check.ts` under `rust` after it: 199 s where 473, a tick
27 to 45 ms in each of its three processes where 120 to 175, 13 lines PASS and 2 red, `lag` and `ahead`, the two that are red under `ts` too
(`same fight` and `late joiner` are green: every game stepped its 1,200 ticks).

The start (the whole export, the Rust game made, the memo filled, four warm applies) is 300 to 700 ms once. The module stands at 160 to 175 MB for a whole
mission (pass-four passed 2 GB before the memo's buffers were trimmed). No step grew the memory once 48 MB is reserved before play.
THE WORST TICK is the first (about 20 ms quiet); a second game stepped three times from the same start before it does not lower it (19.8 against 19.5 ms),
so it is the game's own first-tick work, not the module's compiling (`warmSteps`, off). Under load the worst tick is whichever the scheduler takes.
A tutorial is the same path now (tut-solo: 5.1 ms a tick, 38.7 s for 7,542 ticks). `--whole-every 1` is 100 to 210 ms a tick under the load.

## What a tick costs since the calls cross (cut 7.13)

`RH_TIMES=1`, `playthrough.ts --scn night-breach --seeds 1`, the box at a load of 9 to 13: 1,464 ticks in 89 s of wall, 58 ms a tick (10 s under `ts`);
the record is the same as under `ts`. By the host's own clock, a tick:

| a tick's part | ms |
|---|---|
| the walk for writes by name (the scan) | 33.0 |
| the changed-only export | 11.5 |
| its apply | 5.0 |
| `face_step` | 2.4 |
| the block's copy | 1.1 |

WHY: four calls cross every tick on that mission (`Cover.words` twice, `Cover.ready`, `fragStepOff`: reads of the scripted player and of the page's
own lines). A crossing walks the scan before it (what the tool wrote goes first) and after it (the apply moved the puppet), and exports and applies
once: ten walks of 3.3 ms and five exports a tick. Before the calls crossed, the whole host was about 12.7 ms a tick. A squad mission on Hard costs
60 to 85 ms a tick on the Rust side, which is what bounds the fuzz.

SINCE CUT 7.16 a crossing whose apply changed nothing and sent no event is not followed by the second walk (`RH_OFF=skipwalk` walks as before).

### Where a tally's tick went, and the scan's mend

`RH_TIMES=<file>` (a line a game: the host's clock by part, the walks and crossings counted, the scan's own counts), one process, a box at a load of 3
to 6, ms A TICK:

| | night-breach:1 (1,464 ticks) | after the mend | room-clear-four:1 (3,003) | after the mend |
|---|---|---|---|---|
| the tool's wall, s | 63 | 47 | 196 | 127 |
| clock a tick, all in | 42.1 | 30.6 | 64.6 | 41.4 |
| the scan's walks (5.2 and 7.2 as a tick or a crossing opens, 2.4 after an apply) | 23.1 | 9.2 | 35.7 | 13.1 |
| the every-tick export and its apply (one a tick, one a crossing) | 12.6 | 13.8 | 21.1 | 20.3 |
| `face_step`, the block's copy, the host's half, the rest | 4.6 | 5.6 | 5.4 | 6.0 |

WHY: 4.15 calls a tick cross the surface on night-breach and 6.15 on room-clear-four (`Cover.words` once a teammate, from the scripted player's own
line of a man's state, tools/qa/playthrough.ts:508; `Cover.ready`, :466; `fragStepOff`, :775), and every crossing is a full sync point: a walk before
it (what a tool wrote goes first), the host's half, the call (12 microseconds), an every-tick export and apply (2.3 to 2.8 ms), and a walk again where
the apply moved the puppet (1.4 a tick). NOT the cost: `PT_DIGEST=series` (+2 ms a tick), the fold, the guards, the kept games; the wide list against
`TPS_GAME_LIST=page` is 3 to 12 ms a tick of export and apply.

**The scan's walk now** (tools/blitz/scan.ts; a walk is 1.1 to 1.6 ms where it was 3.7 to 4.4):
- a module's table is asked once a walk, not once for every leaf under it;
- THE SETTINGS BY A TRAP (the harness only, never the page): each plain row of a knob table is an accessor whose setter bumps the table's count of
  writes; a walk visits a table's rows only where the count has moved (3,300 of the root's 4,336 leaves are passed over; a row that is a list or an
  object, or cannot be trapped, is visited every walk). The trap says where to LOOK; what is sent is still what differs from what the walk last saw.
  A write that goes round a trap (`Object.defineProperty` with a value, a `delete`) is the one way to be missed. `RH_OFF=traps`: every row every walk;
- A STRUCTURED LEAF IS ENCODED ONLY WHERE ITS CHEAP WORD MOVED: the object it is and its first level (a list's members, an object's keys and what each
  holds, by identity); a leaf that names a row is held by identity alone. A moved leaf is sent; no copy of its bytes is kept. NOT SEEN: a write two
  levels down that leaves the first level's objects as they were. So ONE TICK IN 64 (`RH_SCAN_AUDIT=<n>`; 1: every walk, the old way; 0: never) every
  walk from that tick's apply to the next tick's opening encodes every structured leaf and holds it to the walk before: a write the word did not see
  is sent then and counted on the exit line (`WRITES THE WALK'S CHEAP WORD DID NOT SEE`, with the fields). It read 0 on both missions;
- `Game.cmd_buf` (the tick's record, which the game's own update writes on the puppet) is not watched: it goes with the host's half.
SHOWN EXACT: the tally's record and every tick's line (the die, the host's half, the orders, the block) the same on both missions before and after;
settings-check.ts all pass and red under its break; a kept fuzz run that writes between ticks by name (the response team on records-pair:1) every
tick's line equal.

**`tools/blitz/reads.json`**: the keys of calls.json's rows whose Rust function takes the game by a shared borrow and so cannot write it (the
compiler's proof; made by the program that demotes `&mut Game` wherever the crate still compiles). After a crossing of such a row the host exports
and applies nothing and walks no second time (`RH_OFF=reads`: as any row). The list is EMPTY today: all three of the tally's rows take `&mut Game`.
Never a guess from a row's history: a tool that reads the puppet after a call that did write must read what it wrote.

`RH_WHO=1`: the fiftieth crossing of each row is counted with the source lines that led to it.

## What has been played

Every tick: the host's half and the driver's calls before it, the die's count and 12 parts of the block after it, all against the plain TypeScript play
of the same seed; then how it ended. On the generated cut 7.6, the fast way, seed 1. Walls are under a load of 8 to 12 on 4 cores, the plain play's
(`mission.ts`, which also writes the loop's files every tick) beside them under the same load.

| mission | ticks | equal | the end, the same on both sides | whole applies | wall, this play / the plain play |
|---|---|---|---|---|---|
| night-breach | 1,413 | every tick | taken, t 23.5, 2 of 2 up | 5 | 25 s / 6 s (8.0 s quiet) |
| room-clear-pair | 2,913 | every tick | taken, t 48.5, 1 of 2 up | 10 | 49 s / 42 s |
| room-clear-four | 3,006 | every tick | taken, t 50.1, 4 of 4 up | 9 | 59 s / 35 s |
| hostage-rescue | 3,119 | every tick | rescued, clean, 2 of 2 hostages, t 52 | 7 | 94 s / 26 s |
| records-pair | 2,186 | every tick | taken, t 36.4, 1 of 2 up | 11 | 44 s / 21 s |
| depot-four | 5,118 | every tick | taken, t 85.3, 2 of 4 up | 12 | 116 s / 21 s |
| block-four | 2,470 | every tick | wiped, t 41.2 | 9 | 60 s / 40 s |
| block-squads | 6,418 | every tick | taken, t 107, 3 of 4 up | 18 | 154 s / 72 s |
| warehouse-four | 5,489 | every tick | wiped, t 91.5 | 20 | 144 s / 69 s |
| compound-four | 7,576 | every tick | taken, t 126.3, 2 of 4 up | 23 | 226 s / 108 s |
| barracks-four | 4,438 | every tick | wiped, t 74 | 19 | 155 s / 50 s |
| rescue-registry | 2,746 | every tick | lost, both hostages dead, t 45.8 | 8 | 62 s / 29 s |
| rescue-consulate | 3,028 | every tick | rescued, costly, 1 of 3 hostages, t 50.5 | 8 | 96 s / 50 s |
| snatch-four | 1,585 | every tick | lost (the man got out by the back gate), t 26.4 | 9 | 34 s / 26 s |
| defuse-four | 4,523 | every tick | taken, t 75.4, 3 of 4 up | 13 | 152 s / 72 s |
| floodlit-four | 5,512 | every tick | taken, t 91.9, 2 of 4 up | 15 | 99 s / 91 s |
| killhouse-four | 5,774 | every tick | taken, t 96.2, 2 of 4 up | 18 | 112 s / 75 s |
| mousehole-four | 7,087 | every tick | taken, t 118.1, 3 of 4 up | 18 | 140 s / 106 s |
| hall-four | 1,998 | every tick | wiped, t 33.3 | 7 | 44 s / 27 s |
| glasshouse-four | 4,276 | every tick | taken, t 71.3, 4 of 4 up | 13 | 85 s / 56 s |
| pass-four | 3,761 | every tick | taken, t 62.7, 3 of 4 up | 14 | 122 s / 70 s |

| tutorial | the fast way | with a whole apply every tick |
|---|---|---|
| tut-queue | 6,092 of 6,092 equal, 14 steps, ended the same | |
| tut-team | 6,930 of 6,930 equal, 12 steps, ended the same | |
| tut-road | 4,930 of 4,930 equal, 23 steps, ended the same | |
| tut-pair | equal for 579 ticks, then the lesson's own check stops it: tools/qa/lessons/tut-pair.ts:103 reads `mate.char.anim.crouch`, not on the list | 8,381 of 8,381 equal, 18 steps, ended the same |
| tut-squad | the host's half parts at 2288 (a list's length in the record) | 7,403 of 7,403 equal, 23 steps, ended the same, THROUGH THE CHECKPOINT PUT BACK at 5739 (a new start). One checkpoint of the reference's four (`5574 "the bags"`) is not in this play's calls: not understood |
| tut-quiet | stopped at 1990: tut-quiet.ts:116 reads a man's light on a spot | equal for 5,204 ticks, then tut-quiet.ts:237 finds no `ko` row with `data.method === 'ring'` in the chronicle: the Rust game's or the bridge's, not the host's |
| tut-sky | stopped at 6931: tut-sky.ts:73 reads who was told to run (the team pace's own counters) | not run |
| tut-kit | the host's half parts at 5302 (the cursor) | not run |
| tut-solo | 7,541 of 8,052 equal (past the four authored checkpoints), then the host's half parts at 7542: the cursor, 4 px (`record[0].mx` 1566.85 here, 1570.99), step 'last' (tools/qa/lessons/tut-solo.ts:314 aims at a man's chest, `char.bones.chest`). The same with every module but the heavy tables applied every tick, so not a field the list lacks: open | not run |

**The tallies** (`tools/blitz/rust.sh rust|ts bun run tools/qa/playthrough.ts --seeds 1-2 --json …`, then `tally-diff.ts`, `tools/qa/mission-table.ts`): 21 missions x 2
seeds on each side. ALL 42 RUNS END THE SAME on both sides: the same end, the same second to the tenth, the same men up, the same of theirs left; the
mission table (`tools/qa/mission-table.ts`) is the same in EVERY one of its 20 columns for all 21 missions (ended by kind, seconds, up, none lost, theirs
left, the stragglers' clock, alone %, the smells per run, the frag bursts and kills, hostages). No run's whole record is the same: 215 distinct
places differ, of two kinds. (1) Counters a TypeScript module keeps and its tick bumped, which stand at zero or are absent on the puppet: `destr.*`
(DESTR_STATS: rounds, bangs, tiles, puffs …), `picture.*`, `digin.*`, `rear.*`, `handle.stats.*`, `doors.manners.*`, `holdfire.lifts`,
`coverfire.wall_rounds`, `def.tactics`, `frag`, `doorforce`, `noshove`, `teampace`. (2) Watchers of tools/qa that read the game every tick and meet a
field the filtered list lacks: `entries.#.cross` and `def.doors.#.t` (a crossing seen 0.1 to 0.2 s late), `entries.#.end` ("in" there, "?" here),
`entries.#.blind`, `endgame.rows` (the last stand's said lines), `smells.#.detail` (the gun's word: "cov" there, "aim" here), `losses.#.since`,
`deaths.#.readyIn` / `.track`, `cleared.summary` ("2 of 8 known" there, "0 of 8" here: cleared::ClearedRooms).

Not played: a second seat (the apply nulls `Seat.own` of the seat whose set stands in the game: src/game/seats.ts:93 throws on the first apply), seeds
other than 1 with the per-tick check, a quiet mission, a restart from a card, any page. The audit (`--audit`) after the merge of 7.6: night-breach:1 to its end, equal
(302 s, 214 ms a tick under the load), the scripted player's orders through the menu's own `give` (its rows asking the Rust game), 7 write-backs, all of
memos (`WorkThings.cache`, `Cover.openings`, `DialogueMod.room_rects`, `HoldfireMod.states` and its log, `DoortoolsMod.sky`) and one flooded room; the
apply held to the Rust bytes on 31 ticks, 0 fields differ. No other mission was audited after the merge.
