# Replays and saves

A run is its start and its hands, stepped again. This page is why that holds, the tape that records it, what the browser keeps, the replay page, and
saves. The hand and the tick are in `overview.md`; a game over a wire is `net.md`; the sound lanes on a replay (`?soundlanes`) are `sounds.md`.

## What the player gets

- Every run is kept in this browser with nothing pressed. Extras, Replays on the title lists them: a row plays one, its cross removes it.
- F7 says the run is kept and how long it is. Shift+F7 leaves to watch it, asking once first while a run is in hand. The end card has a Replay button.
- A replay opens paused in the same window under a bar: back to the game, play and pause, five paces, a line to drag along, notes.
- The camera comes off the run's view, paused or playing. Fog and night vision turn on and off for the picture alone. A small map opens large on M.
- A replay that cannot be played is a plain card that says why. One that stops matching the run plays on, and the bar says from when.
- The game saves itself when things are quiet, and F5 saves by hand. The card's strip goes back to any save of this run. Saves last as long as the
  page.
- A game lost to a crash can stand again from its best save, and its run is kept up to the crash.

## Where the code is

| module | what it does |
|---|---|
| `src/tape/handtape.ts` | the tape: `HandTape` writes it, `HandWalk` reads it |
| `src/host/drawgame.ts` | `DrawGame` writes the tape as it hands things to the module; `feed` hands a row over again |
| `src/host/drawplay.ts` | `DrawPlay`: a tape walked, its moments, seeking, where it parts from the run |
| `src/tape/run.ts` | `RunKeep`: F7, Shift+F7, the beats of the run being played; `keeps` says whether a page keeps anything |
| `src/tape/keep.ts` | the tape as pieces going in; `handsOf` puts them back together |
| `src/tape/keeper.ts`, `src/tape/keepstore.ts` | `ReplayKeeper` files pieces; the store in IndexedDB, its budget, what it lets go |
| `src/tape/kept.ts` | the format of a kept visit's lines, and of a file of them |
| `src/host/tapepage.ts`, `src/tape/address.ts` | where a `?playtape=` tape comes from, and the address it plays at |
| `src/tape/bar.ts`, `map.ts`, `freecam.ts`, `pace.ts`, `refused.ts` | the bar, the map, the picture's camera, the paces, the card that refuses |
| `src/tape/module.ts` | which module this page plays, and `otherModule` |
| `src/ui/replays.ts`, `src/cloud/tapes.ts` | the list on the title; a hosted build sends what the store holds to its backend |
| `src/host/fixedstep.ts` | whole ticks a frame: `planTicks` |
| `rust/game_types/src/dice.rs`, `rust/game_rules/src/tick/dice.rs` | the dice on records, and the game's keyed draws |
| `rust/maths/`, `rust/prim/` | the game's own sin, cos and the rest; the few operations exact by the standard |
| `rust/game_face/src/moment.rs` | `face_moment`, `face_moment_back`: a moment for a walk, with the shelf and what was said |
| `rust/game_face/src/saves.rs` | a host's turn of the controller, and `face_recover` after a panic |
| `savestate/` (`control.rs`) | when a save is made, under which name, what happens when one goes back; `Moment`: the game as a value and its block; `restart` |
| `rust/game_rules/src/tick/standing.rs` | the men as a stand stands them, and again on a restart |
| `rust/game_rules/src/tick/state_words.rs` | the hash of the whole game, a word a part |

The numbers of saves are `CkptK` and `SavetimelineK` in `rust/game_types/src/knobs/savestate.rs`.

Tools: `tools/draw/walk.ts` walks a tape with no page and says if it is still the run. `tools/draw/tape-again.ts` plays a tape's hands on this
checkout and writes a new tape. `tools/draw/tape-make.ts` makes a run from a script. `tools/cloud/pull.ts` reads a hosted build's tapes out of its
backend, and `tools/qa/replay-dump.ts` unpacks them: one `.tape` a visit, and a list of what to look at first.

## How it works

**Why it is the same.** The game is `no_std` Rust with no clock, files or threads. Every roll comes off its own die. Its sin, cos and the rest are
written from add, subtract, multiply, divide and square root, which are exact on every machine, so the same inputs give the same bits. The step is
fixed: the page runs whole ticks, `FIXEDSTEP.hz` a second, however fast the display goes, and draws between them.

**The draws are keyed.** `draw_of` and its kin are a fixed function of the seed, the site's name, the man it is about, the tick and the count at that
site this tick, so a new rule's draws deal nobody else's again. Each is counted, and the count (`face_die_n`) witnesses how many draws a run made.

**The tape** is the bytes of what the module was handed, in order. `TapeStart` holds the level file itself, the address's words, the seed, the seats,
what the machine kept (`Kept`'s text), and the length and hash of each character file. Then rows: a step (`dt`, the hand, the keys), an order
(commands), hands (a hands-off player's plan), a mark, figures, a try, an end. A step writes only the numbers that moved, four bytes where four hold
them. Nothing of the picture is on it.

**Recording.** `DrawGame.step` writes any waiting commands, writes the hand, steps, and every `MARKS.every` ticks writes a mark and figures. A mark is
the die's count, a hash of the block's parts' hashes and one of every part's (`state_words`). Figures are who stands where, who is down, and the rounds Sam has fired. A try row goes on
when the mission's run number changes. A panic, or `TAPE_MOST` bytes, ends the tape.

**Saves are on the tape by being input.** F5 is a key in the hand and a load is a command, and the save controller takes its turn in the step
(`face_step`'s flag `SAVES`) both times, so a walk makes the same saves and goes back to the same ones.

**Keeping.** Every `KEEP.secs` the bytes since the last piece become one line of JSON (base64, and the byte they start at), gzipped into the store. A
piece also goes as a mission ends, as the page hides or goes, and on a panic; a page going away writes it plain, with nothing read first. A visit's
row says what level, how long, its notes and its module. Over the budget (`KEEP.maxMB`, `KEEP.share`), `pruneKept` lets the oldest whole visits go. A
store that fails answers false and the game goes on.

**Walking.** `drawpage.ts` asks `tapeAsked` first. A name like a kept visit's reads the store; anything else is fetched as a file, gzipped or not, the
tape or a visit's lines. `tapeAddress` then sends the page to the run's own address, because a page reads its picture off its address: the tape's
words, less the viewer's (`VISIT_FLAGS`, `NOT_THE_RUNS`), plus `noreplay`. `DrawPlay` starts the game from the tape on this build's character files
and feeds it row by row. `walked` makes it deaf to any other command. It reads one step ahead, so the run's own view is the next hand's camera.

**Seeking.** Every `MOMENT.every` ticks the walk holds a moment and its place on the tape; past `MOMENT.most` it lets go of the one whose neighbors
are closest. A seek back goes to the moment before the target and plays on unseen, some of it each frame (`sought` in `drawpage.ts`), and
`face_draw_cut` starts the picture over. `?at=` opens at a tick; Shift+F7 uses it to start at the try that is on.

**A moment and a save.** A `Moment` clones `Game` and copies its block. `face_moment_back` stands exactly there, with the shelf and what was said, so
the same hands play the same run. A save put back (`after_step` in `control.rs`) goes on from now: `serial_next` keeps the later value, so no serial is dealt
twice, and what belongs to the run as it is now carries over (`abandoned`).

**The save controller.** `control::tick` runs after every step and answers `Do::Keep`, `Do::Back` or `Do::Say`. Autosaves and saves by hand are two
rings, and neither pushes the other out. A request comes from a new stage, a new room, a body moved or a quiet spell, waits `debounce_secs` and
`min_gap_secs`, and lapses after `pending_secs`. `gate` in `control.rs` holds it while anyone is alert, searching or over `aware_max`; `run_no` holds
a save during a response, with Sam down, after the run or with two players. An autosave followed by an alert inside `regret_secs` is kept but offered
last.

**A restart.** A restart is the road a load goes. The controller holds the level as it stands before the men (`keep_level`). `control::restart` puts it
back, writes `Survivors` under the men, runs the stand's own lines (`standing::stand_men`), then writes the switches, the tune and each lens over them.
Nothing else goes over, so a new field needs no reset line, and one that must outlive a try goes in `Survivors`. Only serials and `event_seq` go on.

**Across builds.** A kept row names the module it was played on (the file's ETag, `src/host/face.ts`). On that module any mark that differs ends the
run: die, figures or hashes. On another module, or from a file, the hashes mean nothing, because the state may be laid out differently, so only the
die and the figures count (`figuresPart`, a man within `HAND_BREADTH`). A tape is refused outright for another `HANDTAPE.version`, another count of
hand numbers, or another character file (`other` in `handtape.ts`).

## Rules that hold

- **Nothing is pressed to keep a run.** "we save all replays as we go (in browser storage?)". A title's floor, a benchmark, a networked game and a
  replay keep nothing. `?noreplay` tapes nothing; `?nokeep`, or localStorage `twopointsix.keep` at 0, keeps nothing. A visit is filed only once it has
  stepped.
- **A killed page's run plays up to its last piece.** A visit comes back from its first byte to its first hole. Only whole visits are let go, and
  never one still being written (`WRITING_MS`).
- **Nothing downloads.** "we should get rid of local downloads entirely". Tapes leave by the backend's sync and `tools/cloud/pull.ts`.
- **A hand is kept as it was handed**, matrices too, and never made again from a camera, so a walk is exact in any browser.
- **The viewer touches only the picture.** "would let you pause and move the camera around and not be pinned to the player view", and "also turning on
  and off night vision". `FreeCam` and `face_draw_as` never reach the hand. `face_draw_as` does have the game keep what a viewer's picture is made
  from, so a replay page's whole-state hash differs; a tape from the page carries only the die and the parts' hash.
- **A pace is how fast steps are fed, and nothing else.** "we should also have slomo playback speed in replays". Slow motion holds each recorded step
  longer. It is no finer a run.
- **One window.** "replays should just play in the current window, not open a new tab". The game page leaves a mark (`leaveFor`) that the bar's way
  back reads (`backToGame`).

## To change it

- **Anything that changes the game from the page:** a command (`src/draw/commands.ts`, `rust/game_start/src/host/command.rs`), which is on the tape by
  itself. A call that changes the game any other way makes every walk part.
- **State:** a field reachable from `Game`, so that a clone and `state_words` have it, or a column of the block. A field left out of the hash says so
  with `#[bytes(skip)]`. A new `static mut` in `rust/sim` or `rust/prim` needs its row in `rust-rules-check.ts`.
- **A die roll:** `draw_of(g, "module::function#1", gd)`, or `draw_for`, `draw_body`, `draw_def`, `draw_op`.
- **A number of the hand:** `src/draw/hand.ts` and `HAND` in `rust/game_face/src/hand.rs`, in the same order. Every tape is then refused.
- **A kind of row:** its constant, its writer and `row` in `handtape.ts`, and `rest` in `drawplay.ts` if a walk acts on it. Raise `HANDTAPE.version`
  only if an old tape can no longer be read.
- **A change to play:** old tapes part, and the bar says so. To remake one, `bun run tools/draw/tape-again.ts <tape> <out>`; the sounds' tapes go
  through `sound-tapes.cjs --again`.
- **An address word of the viewer's, not the run's:** `VISIT_FLAGS` in `address.ts`. A word of the run's needs nothing: it is on the tape.
- **How often:** `MOMENT` in `drawplay.ts` for seeks, `MARKS` in `handtape.ts` for how soon a parting is seen, `KEEP` in `keepstore.ts` for pieces and
  room.
- **A new autosave trigger:** `checkpoints_request` from the rule, or a block in `turn` in `control.rs`.

## What holds it

Native tests in `rust/game_start/tests/`:

| file | what it holds |
|---|---|
| `saves.rs` | a save by hand goes back, a restart begins with none, the rings roll, a downed Sam saves nothing |
| `held_saves.rs` | each thing asks for an autosave once, and none is written while he is compromised |
| `rounds_and_saves.rs` | a save carries the broken level |
| `restarts.rs` | a restart equals a fresh staging |
| `restart.rs` | what an evening keeps, a card reaches its man on every try, every kind of mission starts over as it started |
| `hosts_agree.rs` | two machines of one game, every part of the state by name |
| `players.rs` | `a_moment_of_a_game_of_two_put_back_plays_the_same` |
| `senses.rs`, `draw.rs` | a game put back has the light and smoke of one that never left, and tells the picture so |

`gate` rows for each pinned run: `play` (held to `rust/game_start/pins.txt`), `again` (stood again in one process), `save` (a moment put back plays
every frame the same), `load` (the same in a fresh slot), `draw` (drawing changes nothing, also after a moment), and `seats` for two players.
`hold keep` and `hold check` compare two builds frame by frame and name the part that differs. `rust-rules-check.ts`, a row of
`bun run tools/qa/checks.ts`, keeps rounded functions, cells and unlisted statics out of the simulation.

Run by hand, each with a red path:

| | |
|---|---|
| `bun run tools/draw/tape-check.ts` | every mission, hands off and by a random hand, saves, a load and a restart, walked to the same state (`TAPE_BREAK=1`) |
| `bun run tools/draw/walk-check.ts` | every mark is the run's, forward and after each seek back (`WALK_BREAK=1`) |
| `bun run tools/draw/keep-check.ts` | pieces in and out, byte for byte; killed, holed, removed underneath (`KEEP_BREAK=1`) |
| `bun run tools/draw/play-check.ts` | every mission saved, loaded, lost to a planted panic and stood again (`PLAY_BREAK=1`) |

Page scripts (`bun run tools/qa/pages.ts`): `tape-page.cjs` and `smokejump-page.cjs` in `replay`, `savetimeline-page.cjs` in `saves`, `cloud-page.cjs`
in `cloud`, all on the software adapter.

## Not seen

The bar, the map, the free camera and slow motion have been driven by script on the software adapter only, and never seen on a GPU. The line shows
tries and notes, not rounds or orders. No moment is held while a hands-off player has the seat, so a seek back there starts from before he took it. A
networked game keeps no tape, and two players have no saves.
