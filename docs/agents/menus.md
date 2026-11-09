# Menus, the HUD and the debugger

Every screen, and everything drawn over the game. What the missions are is `missions.md`, what a kit row does is `player.md`, what an order does is `squad.md`,
replays are `replays.md`, co-op is `net.md`, how text is worded is `writing.md`.

## What the player gets

- **The title.** Play, Co-op, Options, Controls, Extras (Replays, Sounds, Benchmark), over a live floor. Left alone a few seconds, the camera cuts around that floor under a
  black veil, and the first key or click puts it back. Cheats that are on get a chip.
- **The start page.** Six groups of missions, one open at a time. The highlighted mission shows its map, its words and a Setup rail: Difficulty and its levers, Hour, Weather,
  Way in. The second step on the same frame is Team & kit: how many come, a kit for the squad, each man's card, and the team seen at the mission's door.
- **The load brief.** A mission's page opens on its job and its map, with one block, Start. It stays up until it is pressed.
- **In play.** The corner panel (objective, what a tap of each key does), the kit column, the light and sound meters, the radio strip, marks over men and things, the two rings,
  speech bubbles, the squad's plan on the floor.
- **Tab** holds the game and shows the objective and the squad. **Esc** is the pause card: Resume, the saves, Restart, Options, Run so far, Quit to missions.
- **The end card:** won or lost, the clock, the par, the ways this mission can be won, each man's run, every death explained, the map and the timeline. It offers the checkpoint,
  Restart, the next mission and the Replay.
- **Options.** Graphics, Audio, Game (autosave, hints, the walk-in wall hold, finding your man, the three cheats), Controls, where any key can be moved. Text size.
- **The debugger** (backslash): every number of the rules, the sandbox's actions, the renderer.

## Where the code is

| file | what it does |
|---|---|
| `src/ui/title.ts`, `src/host/reel.ts` | the title, Extras, the boot card (the same screen); the reel |
| `src/ui/startpage.ts`, `missions.ts`, `planArt.ts`, `menuStills.ts` | the one start page: the rail and two steps; the rows, the sheet and its keys; the map; the pictures |
| `src/ui/loadout.ts`, `kitCard.ts`, `kitDense.ts`, `kitIcons.ts` | the kit step: team, squad kit, chips, Deploy; a man's card, its lay-out, its drawings |
| `kitBrief.ts` | the job and map kept in sight over the kit |
| `kitView.ts`, `src/host/kitpage.ts`, `kitguard.ts` | the team at the door: the kit screen's half, the window's half, the window's read-only stores |
| `rust/game_run/src/desk.rs`, `src/view/desk.ts` | the desk: what the cards before a game ask and press |
| `start_list/`, `kit/`, `loadout/`, `kitstage/` (rules) | the list and its settings, the cards, a mission's link and size, the staged picture |
| `src/ui/loadbrief.ts`, `startwait.ts`, `settle.ts` | the brief; the start waits for its picture |
| `src/ui/pausecard.ts`, `runcard.ts`, `debrief.ts`, `verdictCard.ts`, `recap.ts` | the pause and end cards, the saves on them, the end card's contents |
| `src/ui/runBooks.ts`, `verdictStore.ts`, `parBest.ts`, `picks.ts` | what this machine keeps of its runs. No rule reads it |
| `src/ui/options.ts`, `keybinds.ts`, `findme.ts`, `cloudrows.ts`, `src/host/settings.ts` | the Options screen, the rebinding caps, extra rows, what it sets of a game |
| `src/host/keymap.ts`, `src/host/input.ts` | which key gives which code; the keyboard and mouse |
| `src/ui/tabcard.ts`, `src/host/drawhud.ts` | Tab's card; the page's HUD |
| `src/ui/overlay.ts`, `corner.ts`, `handbox.ts`, `bubbles.ts` | the HUD's DOM, the corner panel's reduction, the kit column, speech |
| `cmdring.ts`, `ringlone.ts`, `planfloor.ts`, `squadthrowviz.ts`, `attentionviz.ts` | on the floor's layer: both rings, a ring with one row, the plan, a squad throw, a guard's attention |
| `src/view/`, `rust/game_rules/src/view/` | the view as the page reads it (`read.ts` is written from `rust/game_types/src/view/`); `tick_view`, `debrief_view` |
| `src/host/debugger.ts`, `src/ui/panel.ts`, `graphicspanel.ts`, `lookpanel.ts`, `netrows.ts` | the debugger: its rows, the panel, its other pages |
| `rust/game_run/src/shown.rs`, `actions.rs`, `knobs.rs` | its rows of numbers, its buttons, every knob by name |
| `src/host/sandbox.ts`, `src/host/machine.ts` | what this machine keeps of the cheats and how it plays, as the commands a game is handed |
| `src/ui/textsize.ts`, `src/ui/theme.ts` | the text size; the one look (`--m-accent`, `--m-head`) |
| `src/ui/readout.ts`, `src/host/measure.ts`, `capture.ts`, `src/ui/bench.ts` | the frame-rate readout (`?hud`), F8 and F9, F6, the benchmark |
| `index.html` | the styles of `#menu`, `#missions` and the HUD. Other screens inject their own |

## How it works

**No game behind a card.** A card holds no rule. `Desk` is one blank game in the module, done the same commands a start is done, so a man is staged with what his card shows.
The page asks by the `asks!` list (`src/draw/asks.ts` is made from it). A press answers `Pressed`; where it changed what the machine keeps, `kept` carries the text to
localStorage, and the next page's `machineBefore` gives it back as commands. A mission row is a link (`StartRow.link`), and Deploy loads the address `deploy_link` made, so a
pasted address starts the same run.

**The kit picture is a second page.** `kitView.ts` opens an iframe of this page on Deploy's address plus `kitview`. The module stages the team at the way in, and the man whose
card is open uses the thing under the highlight at the door. Changes go by message (`kv`: kit, sel, pause, frame), and the window answers boot, ready or none.

**A key has a code and a place.** `Input` asks `gameCode` what a key is before filing it. The game goes on asking for the codes it always did, and a tape holds those codes.
On the page (`gameKeysBoot`) they stand so:

| the game asks for | what it means | on the board |
|---|---|---|
| `KeyB` | the squad's command | Q (`MOVED`) |
| `Space` | the action | E (`HAND_SLOT`) |
| `Digit0` | empty hands | backtick (`SLOT`) |
| `Backquote` | the debugger, the page's | backslash (`SLOT`) |
| `KeyQ`, `KeyV`, `KeyT`, `KeyZ`, `KeyL`, `KeyE` | wall hold, whistle, who, selector, light, holster | no key, until the player gives one |
| `KeyG`, `KeyH` | the old menu's letters | asleep, on no list (`LETTERS`) |

The Space bar is then the camera's (`camKey`), while no row was put on it. Everything drawn names keys through `keyText`, `keyCap`, `capNow` or `keysInLine`: the game writes its
lines with the build's letters or a code in brackets, and these write what is bound now. `KEY_ACTS` is the rows; `FIXED` is what no row can have.

**Who hears a key first.** Screens listen in the capture phase at module load (`options.ts`, `loadout.ts`, `missions.ts`, `title.ts`). Then `Input`, whose `keyVeto` asks a
lesson's card and the panel. Then `pageKeys`, ahead of the tick: Tab, F5 over Tab's card, the exposure on the square brackets. Esc is `PauseCard`'s: `away` first lets Tab's card
or a panel field go. Tab asks `tabKey`: while the view's `backs` word says something can be backed out of (a menu, a held arc, a running verb, a gun's setup), the press goes to
the tick; otherwise it is the card. `stopped` is every reason the world holds still.

**The HUD works nothing out.** After a tick the host asks `tick_view` for the seat it shows and hands the record to `Overlay.update` and `Bubbles.update`, after the picture is
drawn, through the camera that drew it. `HudPage` and `pageSay` are the little the page adds. `locked` and `offered` are how a tutorial shuts keys and rows.
The corner panel is `cornerRows`.

**The rings** are one drawing (`RingDraw`), told by `view.ring` (`ring_view`) and `view.hand` (`hand_ring_view`). The rule is `cmdring/` and `interact/`. A tap does the default;
the key kept down a moment puts the ring up. Let go on a wedge does it; let go in the middle leaves the ring up, and 1 to 5 or a click picks. The whistle is the last wedge of
his own ring (`whistle_wedge`). The squad's key tapped twice is fall in (`twice`), and Shift with it sends part of the squad.

**The kit column** is `kit_view`, drawn by `drawHandBox`. The gun's digit pressed again opens its setup (`slotsetup/`), above the column. On every gun 1 is the selector, 2 the
load, 3 the light.

**A run's end.** `View.ended` moves, `onEnded` runs `pause.end()`, and `drawEnd` asks the module once. `renderDebrief` lays it out, and Run so far is the same record. `RunBooks`
counts a try as it begins; a cheat makes it count for nothing (`V.void`).

**The debugger.** `debuggerPanel` builds one `Panel`: a rail of systems, sections, one search over all of it. Its Rules page is laid from the module: `knobRows` over
`game.knobs()`, `actionRows` from `ACTIONS`, `shownRows` from `ROWS`. A turned number or an action is a command, so a tape carries it.

**Text size** is one factor, CSS `zoom` on each of `LAYERS`, in `--tz`. With nothing chosen it follows the window against `TEXTSIZE.ref`; the Options row and the browser's zoom
keys step `steps`. Inside a layer a viewport unit is written over `var(--tz, 1)`, a script asks `uiMatch` in place of `matchMedia`, and a measured rectangle divides by `rectZoom`.

## Rules that hold

- **One page.** "seems like we should just unify everything in the starting page". No straight-in row, no second list.
- **Show, not tell.** "we should generally show, not tell". A setting shows its value. Its sentence is a tooltip and a hidden live region (`mread`). An Extras row is its name.
- **One block leads on.** "confusing that CTA is to deploy but that skips the kit selection screen". On the rows the filled block is Team & kit, with Deploy a quiet word beside
  it. A kit as issued (`asIssued`) has no second step.
- **No digit above 5, no held keys on a screen.** "it's hard to hit numbers bigger than 5 when left hand is on wasd".
- **The brief is read, then started.** "having the loading screen make more sense (e.g. being a pre mission brief) that you then click through rather than flashing the
  controls". A press made while it loads is kept.
- **Start waits for the picture.** "i also quite dislike the 'popin' that happens from starting the game too early when some shaders haven't finished".
- **The kit is picked knowing the job.** "honestly it doesn't make a ton of sense right now since you don't know anything about the objective or map when picking kit". The band
  counts locked doors, steel doors and hostages (`placeNeeds`).
- **Nobody stands in for the kit picture.** The column waits empty until `ready`, and stays empty on `none`. It never fades: "i don't like the fade to black animations".
- **A quiet mission has its own gear screen.** "quiet missions should have different gear screen stuff". Nothing on it is orange, and a tile wears a sound mark by its row's `noise`.
- **Esc is the pause.** "esc is overloaded -- exiting out of menus as well as going to pause. maybe we make tab exit out of menus and esc is always pause?"
- **Tab's card.** "if you don't have a menu open, tab opens up an objectives modal that also has your squad equipment?" It is laid once, and the game holds under it.
- **The squad has two keys.** "squad is now just q and f. everything is through the radial menu". The who row is on Controls only while the player has given it a key.
- **His own actions are a ring.** "radial menu for our own actions, remove existing menu in top left and awkward scrolling thing".
- **A setup is a mode.** "it should be something that pops up above the equipment menu rather than always taking up space". No clock shuts it.
- **The wheel under Ctrl does nothing.** "ctrl-scroll still zooms in and out, should remove". A trackpad pinch still zooms (`ctrlDown`). The plain wheel is no walking pace either.
- **No two rows on one key.** A key another row has is a swap. Reset puts all back. A recording made on another key plays the same.
- **The end card takes no press for half a second:** it comes up mid-stride.
- **The replay is a press away.** "clicking on replay after a mission should start in engine".

## To change it

- **A key:** `KEY_ACTS` for a row, `KEYS` in `options.ts` for its words. Ask the game for a code. `?keybinds=0`, `?qfree=0`, `?camspace=0`, `?slotsetup=0`, `?qonly=0`,
  `?menuletters=0`, `?pinchonly=0` and `?bkey` turn the board's rules off for a visit.
- **A setting:** a `flip`, `seg` or `range` in its tab's branch of `render`, kept by `keep`. One a game holds goes on `OptionsGame` and `settingsOf`, with a command in
  `rust/game_run/src/command.rs` and a word in `rust/game_draw/src/view.rs`.
- **Something a card asks:** a line of `asks!` with the next number, then `bun run tools/draw/gen.ts`.
- **Something the HUD says:** its struct in `rust/game_types/src/view/types.rs`, filled by its rule's own module, its line in `tick_view`, `bun run tools/draw/gen.ts`, the read in
  `overlay.ts`. After Rust, `bun run tools/blitz/face-build.ts`.
- **A debugger row:** a `Row` in `shown.rs` or an `Action` in `actions.rs`. A knob needs nothing: it is on the search by its name.
- **Other words a screen reads**: `?kitbrief=0`, `?stillkit`, `?kitloud`, `?loadbrief=0`, `?settle=0`, `?textsize=0`, `?cards=0`, `?reel`, `?loadout=`.

## What holds it

| native test | what it holds |
|---|---|
| `rust/game_run/tests/desk.rs`, `staged.rs` | the desk by its bytes; a man is staged with what his card shows, on every mission |
| `rust/game_run/tests/shown.rs`, `words_start.rs` | each panel row is a number of the game; the address's words at a start |
| `kit.rs`, `kit_view.rs`, `kit_stage.rs`, `kit_acts.rs`, `kit_levels.rs` | the kit and its column; the kit picture on every level and way |
| `view.rs`, `hud.rs`, `hud_marks.rs`, `cursor.rs`, `rings.rs`, `lesson_view.rs` | what the HUD is told |
| `order_keys.rs`, `menu.rs` | the order keys; one menu row a table row |
| `debrief.rs`, `endcard.rs`, `recap.rs` | the card's record, an end card due once, every death explained |

Checks: `bun run tools/qa/checks.ts menus` runs `squadkeys-check.ts`, `camspace-check.ts`, `hud-look-check.ts`, `menustills-check.ts`, `textsize-check.ts`. Beside them
`view-check.ts` (nothing under the HUD imports the game), `settle-check.ts`, `writing-check.ts`.

Page scripts, on the software adapter: `bun run tools/qa/pages.ts menus` runs `startpage-look.cjs`, `cards-page.ts`, `kitdense-page.cjs`, `kitbrief-page.cjs`, `kitpaste-page.cjs`,
`keybinds-page.cjs`, `textsize-page.cjs`. Beside them `loadbrief-page.cjs`, `panel-keys.cjs`, `savetimeline-page.cjs`.

## Not seen

No check sees how the kit picture, the reel or the HUD over a real frame look on a GPU.

Known fault: `Overlay.setCaption` and `setTimeline`, with the `#caption` and `#timeline` layers, have no caller.
