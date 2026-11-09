# Levels and the look

A level is data: one JSON file of boxes, doors, lights, marks and mission records, written by a generator and read whole by the game. What a mission
does with its block is `missions.md`; doors that move are `doors.md`; what breaks is `destruction.md`; hours and weather are `weather.md`; how a box
is drawn is `render.md`.

## What the player gets

- Every level is a place with a use. Each room reads from above by its furniture, floor and light, never by a label.
- Furniture is real cover and real noise, and what a thing is made of decides what a round or a charge does to it.
- The radio and the debrief use the level's own room names ("the break room").
- Every room the defenders can shut has another way in that needs no charge, and worn paths in the ground lead you round the place.
- Rain stops at the level's own roofs, and the camera always sees the room under one.

## Where the code is

| where | what it is |
|---|---|
| `levels/*.json` | the files. `docs/level-format.md` is the reference, member by member |
| `tools/levels/<name>.ts` | a generator a level; the office's is `tools/levels/lib/officeGen.ts` with `officeScenarios.ts` |
| `tools/levels/export.ts`, `tools/level-export.ts` | write a file from its generator (the second for the office); `--check` is red on a stale file |
| `rust/game_start/src/file.rs`, `records.rs`, `faults.rs` | the format as the game reads it, and the one judge of a file (`judge.ts` asks it) |
| `tools/levels/lib/levelData.ts`, `levelLoad.ts` | the file's types, the order its `KEYS` are written in; the two conversions |
| `levelKit.ts`, `gwotKit.ts`, `livedKit.ts`, `livedPieces.ts` | with `officeGen.ts`, the kits: every piece a generator lays |
| `tools/levels/library.ts`, `docs/library.md`, `library-hall.ts`, `gwot-yard.ts` | the catalogue of every piece and paint, and its two showrooms |
| `rust/game_start/src/level.rs`, `records.rs`, `passes.rs`, `stand.rs` | the game's reader, its records, what a stand does to boxes, a start's order |
| `rust/game_start/src/levels.rs` | the native `levels` tool: what a level owes, asked of the game itself |
| `tools/qa/level-needs.ts`, `tools/qa/level-lint.ts`, `tools/levels/counts/` | what stands around a file, what reads off a file, the held counts |
| `tools/qa/map-plan.ts` | a level's plan as an SVG |
| `src/scene/materials.ts`, `stuff.ts`, `gwotPalette.ts`, `look.ts`, `markings.ts`, `shapes.ts`, `round.ts`, `cut.ts` | made of and does, the palette, the men's dress, face marks, shapes |

In `rust/game_rules/src/`: `scene_level/`, `scene_level_data/` and `scene_scenarios/` are the records; `scene_materials/`, `scene_stuff/` and
`scene_markings/` what a box is and wears; `scene_shapes/` and `scene_cut/` its form; `scene_look/` the men's dress; `scene_scenario_list/` the
mission rows; `levelfit/` refuses a level too big for the game's lists; `lightswitch/`, `carlamps/` and `hours/` a level's switches, vehicle lamps and
street lamps. `rust/game_draw/src/traffic.rs` makes the wear map.

## How it works

**Generator to file.** A generator exports `build` (the `Level`, `scenarios`, a group for each box, `surfaces`), `NAME` and `NOTES`, and at will
`MATERIALS`, `PALETTE`, `KEPT` and `finish`. `export.ts` fits doors (`fitDoors`), adds wreckable things on `LOUD_LEVELS` (`dressLevel`), lifts flats
that would share a plane (`layLevelFloor`), words any box without a `does` (`stampDoes`), runs the skin (`finish`) and writes `levelToData`. It
refuses unless the text loads back to the generator's `Level`, field for field. Then it asks `levels second-way` and `levels sight` (`LEVELASK=0` in
front skips them for a level in the making).

**File to game.** `stand` is the order: `level::load`, `populate_host` (`thin_level`, `bar_level`, `shape_level`, boxes into the world, lamps),
`game_new`. Box i of the file is static box i. Every wrong value, and every member nobody
asked for (`json.rs`), is said by its place, and `load` panics with the list. `faults.rs` is asked of a file, not at a start.
`level_over` and `stood_over` panic on a level over its room.

**Two tiers at the tail.** A box group `lived.<room>` (or a prop, light or fixture id so named) is furniture; group `dressing` is there only to wreck.
They come last in their lists, so every other index is the same without them, and the export throws if something kept follows or names one. The game
leaves them out by `k.lived.on` and `k.destruction.on` (`?knob.lived.on=0`). A level tool always loads the whole file.

**Rooms and places.** Rooms to take are `scenarios.roomClear.rooms`, with `bounds`, doors and marks. Other named ground is `scenarios.places.areas`,
and a quiet mission's `spots`. `own_place` (`dialogue/`) answers from those rectangles, so a line says the level's own words. A name has no article; a
door's said name is `scenarios.doorNames`. The room fight finds its own rooms from the walk grid (`team-feel.md`).

**Marks and ways.** A defender's mark is a `pos`, a `watch` door and an `aimAt` in a room's `marks`; give more than `garrison` and place the first by
hand. A top-level `marks[]` row is a named spot where a person stands to use a thing: the end of a `scenarios.walked` way or a script's place
(`director/`). `ways` traces every patrol, team start, door and `walked` row along the walk grid, and the wear map is their sum.

**Lights and lamps.** `lights[]` are point, spot or directional; `group` and `mains` say what can put one out, `fixture` names its glowing box.
`switchables[]` are glowing boxes on a switch. The directional light in group `exterior` is the moon, and an hour's sun goes in at its place.
`lamps_out` puts out group `exterior` at an hour whose `lamps` is false. No file places a wall switch: `room_light` picks the mains lights a plate
works. `tools/levels/lib/carLamps.ts` and `carlamps/` find a vehicle's lamps by what the kits write. Guards read the light the picture shows, so a
lamp moved for the look changes the play.

**Furniture and props.** A fixed thing is plain boxes. A loose thing is a `props[]` row: `kind`, `mass`, `parts`, its footprint fitted by
`fitPropFootprint`. Men are pushed by what stands from 0.2 m up and the walk grid is baked from 0.25 m, so a top of 0.19 m or less is walked over, and
no top belongs between. A drawn-only box (`NoShadow`) meets no ray and no foot.

**Round and shaped things.** `round` on a box makes it a true cylinder about its own y to every eye, ray and foot; `h[0]` equals `h[2]`, with no
`yaw`, `rot`, `flags` or `shape`. `shape` keeps the plain box to walk on and cuts the picture and every ray: `round` about x or z (a wheel), `arc` (a
bowed top), `cut` (planes in the box's own frame), `old`, and `with` (trim nobody meets). `shape_level` makes the record with `cut_record`. A box with
`rot` takes no shape; one with `markings` or `Glass` takes no form. `form_faults` has the limits, `?shapes=0` stands the plain boxes, and
`tools/levels/lib/carShapes.ts` holds the vehicles'.

**Paint, stuff and does.** A box says two words, and neither comes from the other. `stuff` is how it is drawn (`BoxStuff`). `does` is what it is to a
round, a charge and a blast: a row of `MATERIALS` in `materials.ts`, whose twin the rules read through `does_of`. A file's `materials{}` row gives
`albedo`, finish and a default for both; the names' words are `PAINT_STUFF` and `PAINT_DOES`. `gwotAs` is one paint on a thing that does something
else: a mud partition a charge cuts beside a mud shell. `finish` is usually `skinLevel`: same boxes, other paint, decals in group `look`.

**The look.** The default kit is `gwotKit.ts`; `gwotPalette.ts` names its paints. `look_dress` is true on any `?levelFile=` but the office and
`office-*`, and picks the cast (`place_of`). `KEPT`, 0 to 1, is `meta.kept`: how kept the place is, for the picture. A mark on a face is a
`MarkingDef` bitmap (`tools/levels/lib/markingDefaults.ts`). `surfaces` is the footstep map `floor/` reads. Snow is a level's own, by name
(`snow_level`).

**Roofs.** A roof over walkable floor is a slab flagged `NoPrimary` (`ceiling` in `levelKit.ts`), so the camera sees through it. `roofs_box` asks only
height and size: underside from `ceiling_y` less 0.25 m, top at most 0.45 m over, 0.9 m across or more. A drawn slab in that band stops the rain and
hides the room. The rain reads `MAX_ROOFS` of them. Open sky is no slab.

**Wall-charge places.** A level names none. `levels charges` walks every authored wall box at least 1.5 m long and 1.8 m tall and at most 0.4 m thick,
and counts spots where `wall_at`, `wall_charge_set_spot` and `wall_charge_far_floor` say yes. A thing stood against a wall that takes a place turns
the count red. `scenarios.roomClear.wallCharges` raises the bag on a level built round its walls.

**Second ways in.** `levels second-way` has every room with men in it wedge all it can, every choice tried, and asks if it is still reached from the
team's start with no charge or 40 mm. A window counts where a round opens it and a man steps through; a wall only where the run issues wall charges.

**No level viewer.** There is no editor page. Look with the game: `?levelFile=<name>.json` on the dev server (`handleLevels` serves `levels/`
read-only) and the debugger panel. `bun run tools/qa/map-plan.ts <name>` draws the plan from the file, with rooms, marks, gun lines, starts, ways in
and squads; give `--out`, or it writes beside the level.

**The library.** `library.ts` builds every generator, watches each piece laid through `pieces`, lays each alone and writes `docs/library.md`. A level
asks for a thing by name, and a showroom's line is the call to copy. The hall opens at `?levelFile=library-hall.json&solo&hour=midday`.

## Rules that hold

- **Never edit a file by hand.** Change the generator and export.
- **Every room has a personality.** "the office level is awesome and it has a lot of personality per room, we should makes sure all levels are like
  that". `STYLE.md` lists a room's six things.
- **One library.** "as we dress the levels, I'd love for us to have a library of objects, materials, etc. that we reuse across them". A new thing is a
  kit piece.
- **Color is never a rule (`STYLE.md` is the look's rules).** "we should fix the color walls issue. breaking/penetration shouldn't depend on color at
  all."
- **Inside the world.** `WORLD` is 40 by 28 m with a 3 m ceiling. A doorway is 0.9 by 2.05 m (`DOOR_SIZE`), its middle on a walk-grid cell.
- **Another way in.** "maybe we just need to design levels so there's always another route in?" The mends: a door hung to open out, or a window a man
  steps through.
- **A defender sees what he watches.** Nothing stands in a mark's line to its `aimAt`.
- **Starts and doors are on a way.** "wear, paths in the ground, different materials can all tell a story for how people walk through the level, its
  logic, leading the player for game play".
- **Rain goes by the level's roofs.** "rain is busted on records office".
- **A label is not a room.** "i don't like the "lobby" and "conference" labels in the level, pretty lame". "let's avoid large markings (e.g. numbers
  or room names) where they don't seem realistic".
- **A level names its own rooms.** No line uses the office's word on another level, and no door's name has a compass word.
- **A level does not deal the squad's kit.** Its own `scenarios.issue` cards are for a mission built round a kit (`player.md`).
- **Flats do not share a plane, and decals start at the floor and stop at a door's jambs.**
- **A response team comes only where the file says** (`scenarios.response`). A level without one is in `NOBODY_COMES` with its reason.

## To change it

- **A level:** its generator, `bun run tools/levels/export.ts <name>` (office: `bun run tools/level-export.ts`), commit both, then
  `bun run tools/qa/level-needs.ts <name> --run`.
- **A new level:** `.claude/skills/level/SKILL.md` has the long order, each need with what went wrong when it was missed. In short: generator; export;
  counts (`levels sight <name> --write`, also `walked` and `charges`, in `rust/game_start` by `cargo run --release --bin levels --`); a row in
  `rust/game_types/src/scene_scenario_list/rows.rs` and the module made; `src/audio/setlist.ts`; its picture from `tools/menu-art.ts`;
  `scenarios.kitView` and `tools/qa/kitview-plan.ts`.
- **A count moved:** find out why first. `--write` keeps a new one; a line starting `#` in a count file says what was given up.
- **A new member of the file:** the field under `rust/game_types/src/`, its line in `records.rs` or `file.rs`, its rule in `faults.rs`;
  then its type and `KEYS` in `levelData.ts`, both directions in `levelLoad.ts`, a line in `docs/level-format.md`.
- **A new piece:** the kit's file, through `pieces`; a line in a showroom; `bun run tools/levels/library.ts --write`.
- **A new paint:** a row of the palette, its words in `PAINT_STUFF` and `PAINT_DOES`. `GWOT_MATERIALS` is written whole, so its new row makes every
  file wearing it stale.
- **A new word of what a thing does:** a row of `MATERIALS`, its twin in `rust/game_types/src/scene_materials/types.rs`, and the `words!` list in
  `level.rs`.

## What holds it

`levels <question> [LEVEL…]` (`LEVELS_BREAK=1` fails each): `stands` (each mission starts, and what it names is on joined floor); `doors`;
`second-way`; `sight` and `charges` (held by count); `walked` (doors owed by name in `tools/levels/counts/walked/`); `stacks` (by mission: every stack
mark reaches its door); `names`; `rain`; `file` (its faults).

`bun run tools/qa/level-needs.ts [name] [--run]` asks the `NEEDS` list (`generator`, `row`, `music`, `lived`, `ground`, `leak`, `key`, `lamps`,
`roof`, `response` and more): a FAIL is a gap and a NOTE a thing to look at. `--run` adds `export --check`, the lint, `kitview-plan.ts`,
`floor-order-check.ts` and the `levels` questions, which `gate long` asks of every level.

`levels/<name>.lint.json` holds a reason for an odd thing `level-lint.ts` finds. In `checks.ts`: `exports-check.ts`, `level-lint.ts`,
`ground-pieces-check.ts`, `kit-stores-check.ts`, `palette-check.ts`, `stuff-check.ts`, `floor-order-check.ts`, `zfight-check.ts`. By hand:
`bun run tools/levels/library.ts --check`.

| native test | what it holds |
|---|---|
| `level_rules.rs` | a level's own charges and cards, floor behind furniture, its roofs, its ways, its leaves' paint; `level_caps` is what the game's lists have room for |
| `level_faults.rs` | what the judge of a file refuses, by its place, and what it takes |
| `does_and_stages.rs` | no rule reads a color; every file that names a mission stages it |
| `room_marks.rs` | every room deals marks a man can stand on |
| `kit_levels.rs` | every way of every `scenarios.kitView` stands |

## Not seen

Shapes, wear, skins and lamps have been checked by numbers only, and the plan shows boxes, not light. `tools/levels/counts/walked/` lists doors no way
reaches yet. Mission rows marked `untuned` have not been said to play well.
