---
name: level
description: Everything a twopointsix level needs, in order, each with what it guards and its check. Use when adding or changing a level or a mission (rooms, doors, marks, lights, its row, its kit).
---

# What a level needs

A level is a generator, a file, count files, rows in two lists, two pictures, music, a look and a line in the docs, in places that do not know about each other.
`docs/agents/levels.md` says how the system works, and holds most of the maintainer's rulings. This page is the order of work. `docs/level-format.md` has the file
member by member: where it and the reader differ, the reader is right (`rust/game_start/src/file.rs`, `records.rs`, `faults.rs`).

**Two tools start and finish the work.** `level-needs.ts` asks what stands around the file, and the native `levels` asks the floor.

```sh
bun run tools/qa/level-needs.ts <name>         # a PASS, FAIL or NOTE for each need in NEEDS; --skip <need> leaves one out, --quiet shows only FAIL and NOTE
bun run tools/qa/level-needs.ts <name> --run   # also export --check, level-lint, kitview-plan, floor-order-check and the levels questions but stacks
cd rust/game_start && cargo run --release --bin levels -- stands <name>     # one question of the game itself
```

A FAIL is exit 1, and a NOTE is something to look at and say out loud. `levels <question>` below means the third line, run in `rust/game_start`. **By eye** and **by
play** mark what no tool can ask: do them, and say whether you did. `gate long` runs every `levels` question over every level. A bullet that starts with
a `word` is that need.

## 1. Before you touch anything

- **`generator`: a level is a generator, never a hand-edited file.** `tools/levels/<name>.ts` exports `build`, `NAME` and `NOTES`, and
  `bun run tools/levels/export.ts <name>` writes `levels/<name>.json`. The office alone is `tools/levels/lib/officeGen.ts` and `bun run tools/level-export.ts`.
  Check: `bun run tools/levels/export.ts <name> --check` (exit 1 on a stale file).
- **Grep before renaming** (`git grep -n "<name>"`). Row ids, level names, doors, rooms and routes are named by native tests, gate rows, lessons and kit pictures.
- **Know what your change moves.** A box, door, way, light or man moved moves `rust/game_start/pins.txt` (`gate pin`, and the commit says why) and can part the sound
  replays (section 9).
- **A new member of the file** follows "To change it" in `docs/agents/levels.md`.

## 2. The idea (by eye, before any box)

- **One sentence nobody could say of another level**, at the head of the generator, with the hand placement and why.
- **An authored floor passes five questions.** Does the problem fit one sentence? Is the obvious plan trapped? Are two good answers, neither free, different fights?
  Does it reward one tool? Can the player read everything he needs before contact?
- **Harder by smarter, then set-up, then kit, only then more men, never hit points.** `DEFENDERS_MAX` caps a row at eleven, and a room clear's garrisons sum to
  `maxMen` or fewer.
- **Curated, not shuffled.** Give a room more marks than `garrison`, and place the first by hand. List the best first in `starts`, `spots` and `patrols`.

## 3. The file

- **Inside the world.** `WORLD` is 40 by 28 m with a 3 m ceiling. `bun run tools/qa/level-lint.ts levels/<name>.json` warns on a box outside it and fails on too many
  boxes or light slots (`levels/<name>.lint.json` holds a reason for an odd thing). `docs/level-format.md` has the most a level may have.
- **A height a foot can read.** A top of 0.19 m or less is stepped over. Nothing solid on free floor tops out between that and 0.25 m (`docs/agents/levels.md`).
- **Two wall thicknesses:** `GW_SHELL` outside and `GW_PART` inside, so that a door's frame fits.
- **Paint says what a thing is** to a round, a charge and a blast. `gwotAs` is one paint on a thing that does something else. `does_and_stages.rs` holds that no rule
  reads a color.
- **`floor`: its own footstep map** (`surfaces`, returned by `build`). Without one a level is walked on the office's rectangles, or a quiet level on lino.
- **`moon`: one directional light in group `exterior`.** The hour's sun goes in at its place (`moon` in `levelKit.ts`).

## 4. Rooms, doors, walls

- **Every door is real size.** `DOOR_SIZE`: a 0.9 by 2.05 m doorway, its middle on a walk-grid cell. Check: `levels doors <name>`.
- **Glass is typed by where you put it** (`place_rule` in `rust/sim/src/panes.rs`, with `low_m`, `door_m` and `door_foot_m` in `knobs/glassbreak.rs`). A foot under
  0.7 m, or a panel beside a door, is toughened and one round opens it: a second door for the price of a round. A sill window is float glass and opens slowly. A
  narrow inside sidelight or a high outside strip is laminated and never opens. `destruction.md` says what each does.
- **Every room they can shut has another way in.** Hang a door to open out (`swing`), add a window a man steps through, or issue wall charges. Check:
  `levels second-way <name>`. Export asks it too, unless `LEVELASK=0`. A changed swing moves the level's pins.
- **Its starts and doors are on a way.** The wear map (`rust/game_draw/src/traffic.rs`) is the sum of the ways. Give a start or door a `scenarios.walked` row
  (`from`, `to`, `weight`), and use a name that is one thing. Check: `levels walked <name>`, with what is owed by name in `tools/levels/counts/walked/`, and
  `--write` to keep a new count.
- **A defender sees what he watches.** "we don't want them to feel dumb". Dress a window a mark watches through with the kit's open form (`boards`, `shutters` and
  `windowBlanket` take `'open'`), and stand no cover in a mark's line. Check: `levels sight <name>`, held in `tools/levels/counts/sight/`. Export asks it too. A new
  level writes its file with `--write`; a line starting `#` says what a move gave up.
- **Every door face has floor for a stack**, and nothing lies on the marks. Check: `levels stacks <mission>`, which takes mission ids, so `--run` does not ask it.
  After laying anything over 0.19 m within a few meters of a door, run it with `levels second-way` and the level's row of `gate play`, if it has one.
- **A corridor players walk runs up the screen, or is wide, and no door hangs close under a tall wall to its south.** The camera opens at yaw -22 and pitch 55
  degrees (`camera.ts`), so a wall hides about two meters of floor behind it. By eye.
- **`doorway`: a painted band stops at the jambs.** A door's `at` is the opening's low end, not its middle.
- **Flats do not fight.** Decals that cross are ordered by the export (`layLevelFloor`), and decals start at the floor. Check:
  `bun run tools/qa/floor-order-check.ts <name>.json`.
- **No two faces drawn in one plane.** A wall ends at the near face of the wall it meets, and a post stops at a wall or stands a centimeter off. A lamp on a pole is
  the kit's `yardLamp` or `floodLight`. Check: `bun run tools/qa/zfight-check.ts <name>` (a patch under 100 cm2 is a speck, and passes).
- **`roof`: a roof is the kit's ceiling** (`ceiling` in `levelKit.ts`, flagged `NoPrimary`), no more than `MAX_ROOFS`. Check: the need, and `levels rain <name>`.
- **A wall a charge may hole has floor behind it.** Check: `levels charges <name>`, one number in `tools/levels/counts/charges/`; fewer and more are both red.
- **No door has an always-safe spot to command it from.** If one pocket near a door sees it and is out of every defender's line on every deal, every entry will be
  commanded from there. Give it a second line, or take away its view. Some doors may keep one. By eye, and never played.

## 5. Rooms that are lived in

- **`lived`: every room has the six things** in `STYLE.md`. The need counts things and names bare rooms. Whether a room has a use and a face is by eye.
- **A thing is asked of the library by name.** `docs/library.md` lists every piece, and a line in a showroom (`gwot-yard.ts`, `library-hall.ts`) is the call to copy.
  A new thing is a kit piece. Check: `bun run tools/levels/library.ts --check`.
- **`ground`: the ground is not one flat sheet.** In `gwotKit.ts`, lay finishes as the place is used (`ground`), wear where feet and wheels go (`ruts`), stains where
  use leaves them (`patch`, `stain`, `hearth`) and low things that throw a shadow (`rubble`, `kerb`), where the player walks and the camera rests. `STYLE.md` has the
  rules for paint. The need is a NOTE for each piece of `GROUND.big` m² with nothing that reads near it. Whether it reads is by eye, at the level's own hour.
- **A level says how kept it is, once.** `export const KEPT = <0 to 1>` is the file's `meta.kept`, for the picture alone. A clean rack is another stuff, not a kept
  level. `?kept=` tries it for a visit. Check: `bun run tools/qa/checks.ts kept`.
- **Furniture goes at the tail** of the generator's lists, in group `lived.<room>` (`tools/levels/lib/lived.ts`), so every other index stays the same without it. The
  export throws if something kept follows it.
- **Clear of doors, stacks, marks and lines.** No new wall, and nothing across a mark's aim line. Check: `levels stands`, `stacks`, `sight`, `second-way` and
  `charges`.

**A hand pass adds worn paths, paint and fittings to a dressed level, and moves nobody's fight.** "it's all quite intentionally dressed".

- **Read the level first:** `bun run tools/qa/map-plan.ts <name>` for marks and ways, and a short native test for where men fall. Worn paths are `scenarios.walked`
  rows.
- **No word names a room, and no paint is there for the player** (`docs/agents/levels.md`). A thing the room would have leads the player: a mat, a duct.
- **Paint on a surface is a marking on that box** (`stencilMarks`, `hazardMarks` in `markingDefaults.ts`). The office kit has `boxOn`, `stencilS`, `keepClear`,
  `bandRuns` and `conduitRuns`.

## 6. Light, hour, weather

"this tutorial level just looks really *flat*". A thing's shape is the shadow it throws on the ground, and one raking light carries a room. These are guidelines for
the eye, from the play camera at the mission's hour. None has been seen on a GPU here.

- **Judge the light at the hour the mission plays at.** A row's own `hour` and `weather` are written over the link (`rowset/`), and `lamps_out` has every `exterior`
  lamp out at an hour whose `lamps` is false, so a level's file may be a night nobody plays.
- **The key comes in low.** `hours` has dawn and dusk at 22 degrees and midday at 56, and the kit's `moon` is near 30. A level that has to play at midday gets its
  shape from paint and ground.
- **`key`: one key light a court, from one side.** A second light there is a fill at a quarter of the key or less. The need is a NOTE for each pair of like spots
  that face each other.
- **Lamps make pools, never a wash.** `lantern` is 4.5 over 5.5 m and `bulb` is 26 over 7 m. `floodLight` (210 over 20 m) is the one that washes, so aim it at one
  stretch. A `tube` glows by itself: hang no lamp between tubes.
- **Every outside frame keeps a dark part, and nothing big goes white.** A night wants the cool moon and something warm that burns. Ground, walls and things sit a
  step apart in paint. Can you tell at a glance where the light comes from, and is there a shadow longer than a man? By eye.
- **A light changed for the look changes the play.** Guards read the light the picture shows (`light_at`), so run the mission's checks after moving a lamp.
- **`lamps`: lamps outside are in group `exterior`,** unless the mission switches one itself, and then say so. A thin moon wants lamps outside.
- **A room's mains lights get a switch, and no file places it.** "we should have light switches for rooms!" `room_light` finds the plate beside a door (`beside` and
  `height` in `knobs/lightswitch.rs`), never on glass, so leave that strip plain. A big room on two circuits wants two groups.
- **`leak`: bare metal against the wall of a dark room shows the lamp behind that wall.** It is the renderer's to mend. The need is a NOTE with the rooms and lamps.
  Look at each in a real browser on the ultra preset, with the debugger's specular view; cache irradiance blends through every wall and proves nothing.
- **Starts are dark and unseen.** Nobody sees the team on its start marks, through a pane either. By eye, and by the mission's own test.
- **Snow is a level's own weather,** by name in `snow_levels` (`rust/game_types/src/weather_kinds/types.rs`), its snow in the file. "curate snow levels and have the
  art be custom".

## 7. The mission in the file

`docs/agents/missions.md` has what each kind does. `levels stands <name>` starts each mission the file names and asks the game's own words of its block against the
floor.

- **The `scenarios` block.** `roomClear` has rooms with bounds, doors, garrison and marks, and `team`; `starts` is a second way in; with `squads`, every room is in
  exactly one. Room order is the order the commander works them. `room_marks.rs` holds a room's marks, and `does_and_stages.rs` that a file stages its missions.
- **It names its own rooms and places** (`places`, `doorNames`), and no door's name has a compass word. Check: `levels names <name>`.
- **It ends.** `gate long` plays each room clear, rescue and night breach to its end (`played`, and `lesson` for tutorials). Nothing plays a quiet mission to its end
  but a test such as `quiet_mission.rs`, so play it.
- **A rescue's way in is its way out.** "the hostage rescue missions should have you safely extract the hostage or fully clear the map." It is won when every living
  hostage has been led within `out_m` of the way out (`rescue_end_out`), or the floor is cleared. So a hostage must be able to walk back, and there must be room to
  stand. Test: `rescue_ends.rs`.
- **`response`: who comes when it goes loud is this level's own answer.** "let's make it part of the level design and always make sense (ideally w.r.t. the
  concept/design of the level) and not all levels should have it". Give `scenarios.response` a `why` sentence, and a row the block answers: one with `raid` answers a
  squad's room clear, one without a quiet night gone loud. A level where nobody comes has a line in `NOBODY_COMES` in `level-needs.ts`. Both is a FAIL, neither a
  NOTE. Nothing asks a new block against its floor, so play its first call (`response_team.rs` has the team's rules).
- **A quiet level:** the `quiet` block (`starts`, `spots`, `exits`, `patrols`), every guard a man with a job. Set `wired` only where the holder really is on an
  alarm: "predictably getting a guard to come by when you pull the objective is a bit contrived". Check: `levels stands`, and play it.
- **A lesson:** a line says one thing and names the key, written by "Writing for the player" in `CLAUDE.md`, in 96 characters. A step never passes by itself and
  never becomes impossible. Check: `lessons.rs` (each block on its floor), `lesson_view.rs`, `gate lesson`, `bun run tools/qa/checks.ts writing`. `tut-road-page.cjs`
  is real keys on the real page.
- **A level whose idea leans on geometry** gets a native test in `rust/game_start/tests/` that fails without it.

## 8. The team and its kit

- **A level does not deal the squad's kit.** There is one issue (`issued_card` in `rust/game_rules/src/kit`). A level changes it in two places:
  `scenarios.issue.members` and `roomClear.wallCharges`. Read the card on Team & kit at every size the row offers. Test: `level_rules.rs`.
- **Its own cards are for a mission built around a kit,** and one that changes a counted thing is not shared out, so write one for every man who comes. A `shield`
  key is a fault of the file. By eye, at each size.
- **Count locked and steel doors against the door charges,** and say how each is meant to open. By eye.
- **An order needs kit somebody is issued.** A frag on order is refused near a hostage or a civilian (`squad_frag_catches`), so say what a rescue's squad throws
  instead. By eye.
- **The kit fits the idea.** A level that wants walls opened issues wall charges, and `levels second-way` counts those walls. A row built for one size says `only`.
- **`kit picture`: the Team & kit picture is `scenarios.kitView`.** Name boxes by group, not by a generated id. Every way's picture stands at a door the squad's
  tools can work: a door with a leaf, in a wall, not steel, even where the mission's own way in is a gate (`ref` says from which side). Check:
  `bun run tools/qa/kitview-plan.ts <name>` and, for the game standing on it, `cargo test --test kit_levels` in `rust/game_start`.

## 9. What the player reads and hears

- **`mission`: the row, in the one list.** `scenarios` (or `quiet_missions`, `tutorial_missions`) in `rust/game_types/src/scene_scenario_list/rows.rs`. The page, the
  level tools and the checks ask the game's module for it, so make the module (`bun run tools/blitz/face-build.ts --profile play`), then `gate pin`
  (only that writes the list of rows). `cargo test -p game_run --test desk` in `rust/` holds the desk.
- **`row`: every word on it is true to the file:** `title`, `place` (6 to 64 letters), `blurb` (the premise, never the answer; 20 to 140 on a squad row, 160 on a
  quiet one), `defenders`, `lean`, `ways` (one name per start), `squads` exactly when the file has them, and `untuned` until somebody has played it. A group draws
  eight rows at most; `offList` rows do not count. No digit above 5 on a menu. Check: `bun run tools/qa/checks.ts mystery writing`, and "Show, not tell".
- **`music`: two pieces in `SETLISTS`** (`src/audio/setlist.ts`), `<level>:<kind>` where a kind wants its own. "we should curate the music for each level, not do it
  randomly".
- **Its sounds.** The footsteps are `surfaces` (`floor`). The replays in `public/assets/replays/` are hands played again, and a change to a level they play parts
  them. Check with a dev server up: `PORT=<port> node tools/draw/sound-tapes.cjs --check`. If red, `--again` plays the same hands anew and says what they no longer
  reach, so say that in the commit.
- **`picture`: `public/assets/menu/<row>.jpg` and its row in `stills.json`,** from `bun run tools/menu-art.ts <row>` on the CPU.
- **`look`: built on `gwotKit.ts`,** or the plain kit with `tools/levels/gwotSkin.ts` over it, with one accent no other level has (`row_of` in
  `rust/game_rules/src/doors/paint.rs`). `LOUD_LEVELS` in `tools/levels/dress.ts` says who gets wreckable dressing.
- **Hard, the short list, the par clock,** where the level has them: `squad_hard`, `PICKS` in `rust/game_rules/src/scene_scenario_list/short.rs`, and `scenarios.par`
  (`par.rs`, `gate par`).

## 10. Measure it, then say it

- **Run the checks:** `level-needs.ts <name> --run`, `levels stacks <mission>`, `gate "start <row>"`, and the row's other `gate` rows.
- **Read a change of play side by side.** `missions --seeds 1-8 <level> > after.tsv` (`--bin missions`), then `missions diff before.tsv after.tsv`. Read eight seeds,
  and the sums first. A row is never a claim about how hard a mission is. It asks every room clear of the mission list, so a new row is in.
- **Some lists are chosen cases** (`RUNS` and `SEATS` in `rows.rs`, the table in `kit_stage.rs`). A new level is in none until added: `gate list` ends with the missions no run and no seats
  row plays, and `kit_levels.rs` holds every level with a kit picture.
- **Boot the real page once** on the mission's address, with no console or GPU errors (`.claude/skills/verify/SKILL.md`). `startpage-look.cjs` walks every row
  through its loadout, and `play-check.ts` starts every mission on the module. Say "never seen on a GPU" of what was not.

## When this list is wrong

A miss this list did not catch is a new line here, in the same change that mends the level, and a need in `NEEDS` if a file can answer it. `NEEDS_BREAK=1` is its red
path and must find the planted gap.
