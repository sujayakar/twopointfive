# The level format

A level is one JSON file in `levels/`, written by a generator and read whole by the game. This is its reference, member by member. How levels are made
and why they look as they do is `docs/agents/levels.md`; what a mission does with its block is `docs/agents/missions.md`; how a door behaves is
`docs/agents/doors.md`. The pieces are `docs/library.md`.

## Who reads a file

| where | what it does |
|---|---|
| `tools/levels/lib/levelData.ts` | the schema: `KEYS`, `validateLevelData`, `serializeLevelData`. The export refuses a file it faults |
| `tools/levels/lib/levelRows.ts`, `tools/levels/lib/scenarios.ts` | the TypeScript types of a box, door and light, and of the `scenarios` block |
| `tools/levels/lib/levelLoad.ts` | `levelToData` writes a file from a generator's `Level`; `loadLevel` reads it back for the tools |
| `rust/game_start/src/level.rs` | the game's reader, `load`: boxes, props, lights, doors, routes and the small records |
| `rust/game_start/src/records.rs` | a line for each record the game keeps as its own type (`scenarios`, `surfaces`, `plans`, `devices` and others) |
| `rust/game_types/src/scene_level/`, `scene_level_data/`, `scene_scenarios/` | those types |
| `rust/game_start/src/kit_plan.rs`, `kit_stage.rs`, `kit_set.rs` | read `scenarios.kitView` straight from the file |

The game takes a file as one the schema has passed. It counts what it asked for (`json.rs`): any member nobody asked for makes `load` panic and name it, and
`lacks_of` is that list for a tool. So a member the schema does not know fails the export, and one the schema knows but the reader does not fails the start.

Words used below. V3 is three numbers, x, y, z, in meters, with y up; the floor is y 0. Rect is a record `x0`, `x1`, `z0`, `z1`, with x0 at most x1 and z0 at
most z1. A yaw is radians where 0 faces +z and a quarter turn faces +x (forward is sin yaw, 0, cos yaw). A member is required unless it says a default.

## The file's head

- `version` number. Must be 2; `load` names anything else as a part it lacks.
- `name` text. The level's name. `level_script_of` finds its script by it, and the look and the snow go by it (`place_of`, `snow_level`).
- `meta` record, may be empty.
  - `generator` text. The generator's path; `tools/qa/level-needs.ts` asks that it is in the tree.
  - `bounds` record of `min` and `max`, both V3. The world the level must fit; `tools/levels/lib/lint.ts` reads it, and `WORLD` stands in when it is absent.
  - `markTexel` number, 0.005 to 0.2 m; default `MARKING_TEXEL`. The size of a marking's base cell in this file. Read by the schema's check of markings and
    by the picture (`sky.rs`).
  - `kept` number, 0 to 1; default 0. How kept the place is, for the picture alone: 0 is left to the weather, 1 is cleaned last night (`lookKept`, `sky.rs`).
    The page's `?kept=` wins over it.
- `sky` record of `zenith` and `horizon`, both V3. The level's own night sky. Read by `hours/` and `sky.rs`.
- `materials` table from a paint's name to a record, in the order written. A box or a part wears a row by name.
  - `albedo` V3, each at least 0. The color.
  - `rough` number 0 to 1; default matte. The finish the reflection pass reads.
  - `metal` true or false; default false. Bare metal.
  - `stuff` word; default plain. What it is drawn as: a word of `BOX_STUFFS`. The picture's alone.
  - `does` word; default the row unnamed. What it is to a round, a charge and a blast: a word of `DOES_WORDS`. The rules read this through `does_of`.
- `breaker` record: the mains panel of the level.
  - `pos` V3. Where the panel is, for the men sent to it (`lighting/`).
  - `panel`, `led` box ids. The panel is what a man works (`handwork/`, `workthings/`); `combat/` takes both as the breaker's boxes, and the LED is the
    box the breaker's light target lights (`game/`).
  - `fumble` number, over 0 and at most 7; default 2.2 s. How long a man works at a tripped panel before the mains come back (`lighting/`).
- `beacons` list of records: emergency beacons, a spot light turning at a dome (`game/`).
  - `pos` V3. `box` box id of the dome, and that box must be in a switchable.
- `mission` record: the plain night's one thread, which every file carries. A quiet mission lays its own dealt spot over it (`quiet/`).
  - `playerSpawn` V3. Where Sam starts on the plain night.
  - `objectives` list holding exactly one of each `kind`:
    - reach: `area` Rect, the room's floor, and `door`, the door the marker points at. Read by `mission/`, `doorswing/`, `dialogue/`.
    - drive: `rackIndex` whole number, which also names the switchables `rack<N>_leds` and `rack<N>_glow` that go dark; `front` V3, the marker on the holder's
      face; `halfW` number at least 0, half its width along the face.
    - exfil: `door`; `x`, `z` numbers; `out` unit pair dx, dz. With `out`, you are out past the point going that way and within 1.8 m of it sideways;
      without it, past x (`mission/`).

The schema also asks for a few things the game passes by, which `levelToData` writes: `floors` (at least one, with `id`, `y` and `ceiling`;
`tools/levels/lib/lint.ts` reads the first ceiling), `connectors` (always an empty list: `load` refuses any row), an `id` on each objective, beacon and plan, an
`id` on each surface zone, and a `kind` of hide, utility or trigger on each volume.

### How much a level may hold

`tools/levels/lib/caps.ts` holds the schema's numbers and `levelfit/` refuses at start (`level_over`, `stood_over`).

| what | at most | said by |
|---|---|---|
| doors | 64 | `DOOR_CAP` |
| props, and parts in all of them | 128, 512 | `PROP_CAP`, `PROP_PARTS` |
| beacons, cameras, lasers | 32 each | `LIGHT_CAPS` |
| lights, switchables, cameras, lasers and the breaker together | 256 | `LIGHT_CAPS` |
| light rows, counted at start with beacons, cameras and vehicle lamps | 256 | `level_caps`, not the schema |
| sheets of glass | 256 | `PANE_CAP`, not the schema |

### What a switch leaves out

A box in group `dressing` stands only with `k.destruction.on`. A box in a group that starts `lived.` stands only with `k.lived.on`, and so does a prop, light or
switchable whose `id` starts `lived.`. These come last in their lists, so every other index holds without them.

## Boxes

`boxes` is a list of records, and the box at index i of the file is static box i of the game. Read by `load` into `make_box`.

- `id` text, unique. What every other record names it by.
- `group` text; default none. The generator's section. The switches above and the kit view's `hide`, `hidePaint`, `hideMore` and `show` go by it, and `carlamps/`
  goes by what the kits write. A door's frame is plain boxes in group `doorframe.<id>`.
- `name` text; default none. A tag on the box. `wallpen/` takes a name starting `door_` as a door.
- `c` V3, the center. `h` V3, half sizes, each at least 0.
- `yaw` number; default 0. Turn about +y.
- `rot` four numbers x, y, z, w, a unit quaternion; default none. For a tilted thing. Takes no `markings` and no `shape`.
- `round` true; default none. A true cylinder about its own y to every eye, ray and foot, radius `h[0]`, half height `h[1]`. `h[0]` must equal `h[2]`, and the
  box says no `yaw`, `rot`, `flags` or `shape` (`round.ts`).
- `material` a key of `materials`, or `albedo` V3 at least 0. Exactly one.
- `emissive` V3 at least 0; default none. Light it gives off. A box that switches is also in a switchable, which holds its lit value.
- `rough`, `metal`; default the row's, else matte and not metal.
- `stuff` word of `BOX_STUFFS`; default the row's, else plain. How it is drawn. No rule reads it.
- `does` word of `DOES_WORDS`; default the row's, else the default row. What it does. Written by `stampDoes` on every box an export has not worded.
- `flags` list of words; default none.
  - NoPrimary: not drawn to the camera, while rays meet it. A roof or a ceiling (`docs/agents/levels.md`, Roofs).
  - NoShadow: met by no ray and no foot. A decal or a glow card.
  - Cutaway: drawn only below `cutHeight`, while rays see the whole box.
  - Gloss: a polished floor; its top face takes a reflection ray in the final gather.
  - Glass: a pane (see Glass below).
- `cutHeight` number; default never cut. Read with Cutaway (`rank.rs`).
- `markings` list of at most `MAX_MARKS_PER_BOX`; default none. See below.
- `shape` record; default none. See below.

### Markings

A marking is a small bitmap on one face (`markings.ts`, carried by `laid_markings`, drawn by `markings.wgsl`). A box with `rot` has none. `markingErrors` holds the rules.

- `face` one of +x, -x, +y, -y, +z, -z.
- `res` fine, base or coarse; default base. Half, one or two base cells a cell.
- `at` two whole numbers, its corner in cells from the face's center. `size` two whole numbers, width and height, each 1 to 32. It must lie inside the face.
- `pal` list of 1 to 3 records, the colors a, b and c:
  - `albedo` V3 in 0 to 1, or `mul` number 0 to 4 on the box's own color. Not both.
  - `rough` number 0 to 1. `emit` V3 at least 0.
- `cells` list of `size` height strings of `size` width characters, top row first: a dot is bare, and a, b and c are the colors.

### Shape

A shape is cut from the box and said in the box's own frame, so it moves and turns with the box. The box still walks as the plain box; a cut changes the picture and
every ray. `shape_of` carries it and `shape_level` makes the record; `?shapes=0` stands the plain boxes. `shapeErrors` holds the rules.

- `round` 1 or 3. A twelve-sided prism about the box's x (1) or z (3). The box must be square across that axis to within 15 hundredths. No `cut` beside it.
- `arc` record of `axis` (1 or 3) and `rise` (over 0, at most the box's height and half width). The box's top is a round about that axis. Not with `round`.
- `cut` list of planes, four numbers nx, ny, nz, d, each keeping n · p ≤ d. At most `SHAPE_PLANES` (8), or `ARC_PLANES` (5) beside an arc. Each must cut the box and
  leave some of it.
- `old` true; default none. Not a fresh break: the cut faces take the box's paint.
- `with` list of pieces, `PIECE_KEYS`: trim that is drawn and nothing else, and always NoShadow.
  - `off` V3, its center from the box's. `h` V3, each over 0. `yaw` number from the box's own; default 0.
  - `material` or `albedo`, with `rough`, `metal`, `stuff` and `does`. A piece with neither wears its box's paint, finish and words and says none of those four.
  - `round`, `arc`, `cut`, `old` as above.

A box with `rot` says no shape. A box with `markings` or the Glass flag may say `with` and no form, because what reads those reads the whole box.

## Lights and switchables

`lights` is a list of records, read by `load` and `light_from_def` into the light set, and hung as light targets by `game/`.

- `id` text, unique. The name a target, a lesson test or a kit view's `dayLights` goes by.
- `kind` point, spot or dir.
- `pos` V3. `dir` V3, not zero. `color` V3 at least 0.
- `intensity` at least 0. For a point or spot it is candela-like; for dir, the light on a surface square to it.
- `range`, `innerDeg`, `outerDeg` at least 0, with `innerDeg` at most `outerDeg`.
- `group` text. What can switch it. Group `exterior` goes out at an hour whose `lamps` is false (`lamps_out`), and its directional light is the moon, with an hour's
  sun at its place. `room_light` picks what a wall plate works: a mains light outside `exterior` and `emergency`.
- `enabled` true or false.
- `volumetric` at least 0. How much it lights the haze.
- `radius` number at least 0; default 0.05 m, or 0 for dir. Emitter size, which softens shadows; for dir, an angle.
- `mains` true or false; default true. On the breaker, so a blackout takes it.
- `broad` true or false; default false. A wide soft emitter for the direct pass's denoiser.
- `fixture` box id; default none. The glowing box that goes dark with it.

`switchables` is a list of glowing boxes on a switch, read by `game/` into targets.

- `id` text, unique. `rack<N>_leds` and `rack<N>_glow` go dark when the thing comes out of holder N (`quiet_mission_rack`); a lesson's target test names one.
- `boxes` list of box ids, at least one. A box is in one switchable only.
- `group` text, as a light's. Group `emergency` is a beacon's dome. `on` true or false.
- `baseEmissive` V3 at least 0. The boxes' glow when on.
- `flicker` true or false; default false. It flickers.
- `mains` true or false; default true.

## Doors

`doors` is a list of records: hinged leaves in a gap whose frame is plain boxes. Read by `load` into `LevelDoorDef`, then `doors/` and `doorswing/`. A set's
`doors` are the same record.

- `id` text, unique. What every door member elsewhere names (`doors_by_name`).
- `hinge` two numbers x, z. `closedDir` number: shut, the leaf runs from the hinge along cos, sin of this in x, z. `width` over 0. A doorway is `DOOR_SIZE`.
- `angle` number; default 0. How far open it starts.
- `exterior` true or false; default false. An outer door: its paint, its place in the wear map, and how doors deal their swing.
- `closer` true or false; default true. False has no closer (`doors_has_closer`, which is false for all while `doors_stay`).
- `minAngle`, `maxAngle` numbers; default −1.92 and 1.92. Its stops. A `swing` must leave its side an opening.
- `locked` true or false; default false.
- `lock` text; default `FLOOR_KEY`. The key that turns it. A keypad's door must say the keypad's id here.
- `keySide` 1 or −1; default 1. The face the lock is on (`doors_locked_out`).
- `swing` 1 or −1; default dealt by `deal_swings`. The side it opens into, +1 being counterclockwise of the shut leaf.
- `steel` true or false; default false. A boot only rings off it and no pick opens it.

## Props

`props` is a list of records: loose furniture. Read by `load` into `PropDef`, then `props/`. `fitPropFootprint` and `fit_prop_footprint` are one rule.

- `id` text, unique. `name` text; default the id.
- `kind` chair, cardboard, bin or plant. A row of the `kind` table in `props/`.
- `x`, `z` numbers. `yaw` number; default 0. `mass` over 0.
- `parts` list of at least one record, each a box standing with the prop:
  - `off` V3 from the prop. `h` V3 at least 0. `yaw` number from the prop's heading; default 0.
  - `material` or `albedo`, exactly one. `stuff`, `does` as a box's. `markings` as a box's.
- `half` two numbers over 0, and `height` over 0; default fitted from the parts. Written only to pin a footprint.

## Routes and patrols

`routes` is a list of one man's beats, read by `load` into `PatrolRoute` and walked by `guards/`.

- `id` text, unique. `name` text; default the id. The name the radio, `atMan`, `patrols` and `walked` go by.
- `points` list of V3, at least one. `wait` list of the same length, each at least 0: seconds stood at that point.
- `weapon` a word of `WeaponId`; default guard9mm (`game/`).
- `keys` list of lock names; default the floor key.

`chokepoints` is a list of places the third man of a search goes to hold (`guards/`): `id`, `name` text (default the id, said as "holding the" name), `pos` V3,
`yaw` number.

`civilians` is optional and its absence means nobody but the shift. Read by `civilians/`.

- `id` text, unique. `name` text; default the id. What the men call her.
- `kind` cleaner or worker. `tint` V3 at least 0. `tint2` V3; default a dark gray.
- `points`, `wait` as a route's.
- `tasks` one word a point; default stand. Bin, stand, wipe, sit, cooler, printer or shelf.
- `face` one yaw a point; optional.

## Marks

- `marks` optional list of named spots where a person stands to use a thing: `id` text, unique, `pos` V3. Read by name: the end of a `scenarios.walked` way
  (`traffic.rs`) and a script's place (`director/`). `levels` asks that each stands on walkable floor.
- `volumes` list, may be empty: `id` text, unique, `c` V3, `h` V3, `yaw` number (default 0). A script's trigger box: `director/` asks who is inside, by plan, and from
  under its foot to 2 m over its top.

## Surfaces

`surfaces` is optional: what is underfoot for the mixer, the guards' ears and the sound meter, all through `floor_at`. Every material is a word of `FloorMat`: carpet, lino,
stone, tile, asphalt or grass. With no block the game uses the office's own map, and a quiet mission's level is plain lino.

- `building` Rect. Outside it, a point is `verge` where x or z is under `grassEdge`, else `outside`. Inside it, the first zone that holds the point, else `inside`.
- `grassEdge` number. `inside`, `outside`, `verge` materials.
- `zones` list of records: `mat` material, `x0`, `z0`, `x1`, `z1` numbers.

## Devices and vents

`devices` is a list, may be empty, of three records told by `kind`, read by `records.rs` and run by `devices/`. The game draws each from its record, so none is in `boxes`. A
camera and a laser are light targets too: an OCP knocks them out for a spell, a round kills them, and `mains` puts them on the breaker. Each has an `id`, unique.

- camera. The defaults are `device_defaults`.
  - `pos` V3, the lens. `yaw` number. `pitch` under 1.5 either way, negative down; default −0.35.
  - `sweep` two absolute yaws; default fixed. `sweepPeriod` over 0, seconds there and back; default 9.
  - `fovDeg` over 0, at most 170, the full cone; default 70. `range` over 0; default 14 m.
  - `group` text; default security. `mains` default true.
- keypad.
  - `pos` V3, the plate. `yaw` number, out of the wall.
  - `door` door id. That door must be `locked` with `lock` equal to this keypad's id, and have no other keypad.
  - `code` four digits. Sam opens it once he knows the code.
- laser.
  - `from`, `to` V3, the emitter and the receiver, at least 5 cm apart. Their y is its height.
  - `group`, `mains` as a camera's.
  - `alarm` suspicion or alarm; default suspicion. What breaking it does (`devices/`).

`vents` is optional and its absence means still indoor air. Read by `wind/`, whose defaults are `vent_defaults`.

- `id` text, unique. `pos` V3, the face's center. `dir` V3, not zero.
- `speed` 0 to 12 m/s at the face. `spread` under 89 degrees, the half angle. `reach` over 0 up to 30 m, where the air is at half.
- `size` over 0 up to 3 m; default 0.6.
- `period` over 0 s; default steady. `duty` 0 to 1; default 1. `phase` seconds; default 0.
- `mains` default true, so a blackout stops the air.


## Glass

Glass has no block of its own: a pane is a box with the Glass flag. Its paint is the tint it lets through. `load` makes the box, `glass/` keeps its state, and the picture tests
it apart from the world's boxes. Feet and sound take it as a wall, sight passes through it, and a round, a kick or a bang breaks it. It takes no form, so `shape.with` is the only
shape it says. A sheet is thin: `is_sheet` asks that the thinner of `h[0]` and `h[2]` is at most `THICK_GLASS_HALF`, and a thicker Glass box is solid glass that does not craze.
The kit lays panes with `paneX` and `paneZ`. A level's most is `PANE_CAP`.

## plans

`plans` is a record, `roomClear` a list that may be empty, and `breach` an optional record. These are the scripted door drills. The schema checks a room plan's members and lets
others through; the reader refuses any it has no line for.

A room plan, each row of `plans.roomClear`, is read by `squad/`. `doorswing/` and others read its `door`, `room` and `bounds`.

- `door` door id. `room` text; default none. `breach` kick or open; default kick.
- `bounds` Rect; default none. The room's floor.
- `from` two V3; default none. Where each of the pair comes from.
- `stack` two V3, `stackFace` two numbers, `stackSide` 1 or −1, `stackAim` two V3. The two men's stack marks, faces, side and aim points.
- `kickFrom` V3, `kickFace` number. Where the boot goes in.
- `fling` number. The speed the leaf is thrown at, by its sign which way.
- `entry` two lists of V3 at least one long, `podFace` two numbers, `entryAim` two lists of V3. Each man's way in, facing and sweep.
- `searchPath` list of V3, `searchFace` number, `searchAimWalk` and `searchAimThere` lists of V3. The search of the room.
- `underAim` list of V3, `upAim` V3. Looking under and up.
- `formUp` two lists of V3, `formFace` two numbers. Where they form up after.
- `lines` record of twelve strings, `stack1`, `stack2`, `go2`, `in1`, `side2`, `side1`, `push1`, `under2`, `nothing2`, `corner1`, `empty2`, `radio1`: what they say.

`plans.breach` is every member of a room plan, and the night breach's own. Read by `breach/` and `nightbreach/`, and `roomclear/` takes it as a room plan.

- `inward` two numbers; default 1, 0. The way into the room on the floor.
- `hostiles` list of at least one record: `pos` V3, `yaw` number, `aimAt` V3, `crouch` true or false (default false).
- `retreat` record of `mark` V3 and `face` number. `toss` record of `mark` V3, `face` number and `land` V3. Who backs off, and who throws and where it lands.
- `flow` two records, each man's way through the room: `into` list of V3, `centre` two V3, `corner` list of V3, `podFace` number, `cornerAim` list of V3, `hold` V3.
- `targets` two records of `centre` and `corner`: lists of indexes into `hostiles`, who that part of the flow looks for.
- `next` record of `door`, `stack`, `stackFace`, `stackSide`, `stackAim` and `via` (optional list of V3): the next door, and the lane to it.
- `breachLines` record of eight strings, `breach1`, `bang2`, `going1`, `clear1`, `clear2`, `roomClear1`, `next1`, `radio1`.

## scenarios

`scenarios` is optional, and a level without a block stages none of it. It has twelve optional members, `KEYS.scenarios`, read into `LevelScenarios`. Door names are door ids. What the
file alone cannot say is checked against the built floor: `levels stands` runs `check_structure`, `check_rescue` (with `rescue_flow_check`) and
`check_quiet`, and `response_team.rs` holds the response team.

A team start is a record used by several blocks: `sam` V3, `samYaw` number, `ops` list of V3 (the teammates' marks; the second pair stands a stride behind the first if only two are
given), `opsYaw` number (default +x).

### nightBreach

The team's start beside `plans.breach`, which it needs (`nightbreach/`). `team` a team start, of which `sam`, `samYaw` and `opsYaw` count: the first pair stands on the plan's
marks. `room` text, default none: what the room behind the door is called.

### roomClear

The rooms to take (`roomclear/`, `roomfight/`).

- `name` text. The structure's name.
- `team` team start, two `ops` at least; default the night breach's start, which needs `plans.breach`.
- `starts` list of team starts; default none. Other ways in, beside a `team`: way 1 is `team`, way 2 the first of these. `?way=` picks one, and any deals one.
- `maxMen` whole number 1 to `SQUAD_MAX_MEN`; default five. The most defenders alive at once.
- `wayOut` true or false; default false. On Hard, the men a lost room draws hold one of its doors from back off it (`pressure/`).
- `digIn` true or false; default true. False: this floor's men never brace a door, take cover or set a gun on a door (`roomfight/`).
- `wallCharges` whole number 1 to 9; default the kit's own. The squad carries at least this many wall charges here (`issue/`).
- `rooms` list of at least one record. `check_structure` asks that some room has more than one way in.
  - `id` text, unique. `name` text, as a line says it, with no article. `bounds` Rect, the floor inside its walls.
  - `doors` list of `id` (door id) and `from` (what is on its other side; `doornames/` says it).
  - `garrison` whole number at least 1. How many hold it.
  - `marks` list of `pos` V3, `watch` (one of this room's doors) and `aimAt` V3. Give more than `garrison`, and place the first by hand.
- `squads` list; default one pool of defenders. With a block, every room is in exactly one squad. Read by `enemysquads/`. It says who and where; what they do is the AI's call.
  - `id` text, unique. `name` text; default the id. What their radio calls them.
  - `rooms` list of room ids. At least one, except for a squad with `arrive`, which holds none.
  - `rally` V3 with y 0; default the AI picks. Inside one of its rooms, except for a late squad.
  - `barricade` list of door ids of its rooms; default every door of its rooms but one two of them share. Empty means never.
  - `mayLeave` true or false; default true. False: no press, no flank, no going to help (`stayask/`).
  - `training` green, mixed or trained; default the difficulty's mix (`morale/`).
  - `arrive` record: `door`, `men` (whole number at least 1), `spawn` (list of V3, at least `men`, outside every room). A squad that comes in once the floor is loud and they fit `maxMen`.

### rescue

The hostage rescue (`rescue/`).

- `team` team start, two `ops` at least.
- `room` record:
  - `name` text. `where` text; default none: where it is, for the lines.
  - `door` the main door. `bounds` Rect; default the clear plan's for that door. `doorAim` V3, the point on it the men inside keep their guns on.
  - `outside` V3; default worked out 1 m out of the door. `hold` V3; default worked out 0.4 m out (`rescue_room_point`).
- `side` record; default none. The room's second door:
  - `door`, not the main one. `hold` V3, the point on it the men inside keep once the room is called.
  - `from` record of `door` and `bounds` (default that door's clear plan): the room it is taken from.
  - `pair` true or false; default false. The man sent here takes his pair mate standing in the same room.
  - `stack`, `standoff` lists of V3; default worked out. Marks for the face the team comes at.
- `hostages` list of at least one, `pos` V3 and `face` number.
- `captor` a mark, `pos` V3 and `aimAt` V3. `doorMan` record of `inside` and `outside`, marks: his two places.
- `roamers` list of marks: men elsewhere who come to the noise. `guards` list of marks; default none: more guns who never leave the room.
- `cast` record of `roamers`, a whole number at most the marks; default `k.rescue.roamers`. How many of those marks are manned, first ones first. At most 11 defenders, and 16 characters
  in all.
- `flow` record; default none, and then no covers. The corridor the team runs and the openings off it, read by `rescueflow/`. As built it reads a corridor along +x, walked toward +x.
  - `corridor` Rect. `hold` record of `ahead`, `xMax`, `z`: the man left holding watches `ahead` m on from his mark, no further than `xMax`, at this z.
  - `regions` list of `id`, `door` (door id or null, where a door's clear plan gives the room) and `rect` Rect.
  - `route` list of openings: `id`, `name`, `short`, `region` (a region's id), `kind` (glass, doorway or door), `door` (door id or null), `at` V3 (the threshold's middle), `mark` V3
    (where a man covers it), `watch` V3 (what his gun is on), `half` (half width), `north` (true on the −z side), `call` (his words), `side` (left or right), `look` (seconds he looks).

### places

`areas` list of `name` text and `bounds` Rect: floor nobody holds, by name, without an article. Read by `level_rects` for what a line says (`own_place`, `place_name`), and by
`sweep_places`. Where a point is in a room too, the room wins.

### quiet

The quiet mission, `quiet/`. The run's seed deals one start, one spot and one patrol set; every exit is open.

- `thing` text with its article; default the drive.
- `starts` list of at least one: `sam` V3, `samYaw` number, `mate` V3 (default the free floor a stride behind Sam), `name` text (for the log).
- `spots` list of at least one:
  - `place` text, as the objective says it. `area` Rect; stepping in is reached. `door` the marker's door. `holder` text, what it is taken from.
  - `rackIndex` whole number; names the switchables that go dark. `front` V3, `halfW` number at least 0: the marker and half the face's width.
  - `wired` true or false; default false. True: emptying it brings a man to look (`takecheck/`).
- `exits` list of at least one: `name` text, `door`, `x`, `z`, `out` unit pair, `halfW` over 0 (default `EXIT_HALF_W`).
- `patrols` list of lists of route names; default every route. At most `QUIET_MAX_ROUTES` to a set, and a level with more routes must give sets.
- `needs` record of `coffee`, `smoke`, `water`, `stretch`, each a stand; a kind left out is not dealt (`need_spot`). `stretch` needs `loop`, its lap.
- `phones`, `wayouts` lists of stands: where a frightened civilian rings from and gets off the floor (`civilians/`).
- A stand is `name` text, `pos` V3, `face` number, and for the needs `loop` list of V3.
- `radioCheck` true or false; default the game's check. False: nobody sounds off (`guards/`).

### lesson

Steps laid over whichever scenario the row starts. A step's line moves on when its test is true. Read by `lesson/` and, for what a tutorial may do, `rails/`. `title` text. `steps` list of at
least one. `rails` record; default none.

A step:

- `id` text, unique in the lesson. `line` text: the objective row. `done` a test (below).
- `setupLine`, `ringLine`, `blowLine` text; default `line`. The line while a gun's setup, his own ring, or his own charge is what the keys work.
- `menuLine` text; default `line`: while the squad's ring is up. `pageLine` default `menuLine`: on a page under the first. `flatLine` default `menuLine`: on a flat ring.
- `hold` seconds at least 0; default none. The test must stay true so long. `settle` over 0, at most 10; default none. Seconds before the next step comes up.
- `at` V3; default no marker. `atMan` a route's name: the marker rides on that man while he is up, then `at`. `toward` V3, needs `at`: an arrow on the ground from `at`.

On a level with `rails` only:

- `allow`, `deny` lists of actions, for the rest of the mission. `cue` text: sent to the script as a signal. `checkpoint` text: a named checkpoint kept here. `fog` true or false, from here on.
- `noTap` text: a tap of the command key gives no order while this step is up, and these words show.
- `eye` record of `at` V3, `yaw`, `fov` degrees up to 360 (default 120). A lookout for the fog of war, for the picture alone.
- `explain` record of `title` (optional) and `text`, with `quiet` or `now`, at most one. A card that pauses the page. `quiet` waits for the fight to go quiet; `now` skips the beat.
- `onWay` record of `nv` and `run`, needs `at`. Once a man is ordered to the mark: false `nv` puts night vision up, true `run` has them run the leg.
- `after` record with `line` or `word`, and `on`. The line once the order (`on`: which order ids) has gone, or the word has been given.

`rails`:

- `allow` list of actions (`rails/`). Every other key is dropped. `respawn` seconds at least 0; default he goes down: a teammate spends so long on a knee instead. `fog` true or false.
- `waits` record of `low`; default they fall in at once. The squad holds where the mission stood it until a step allows the order key; `low` is on a knee.
- `lasers` false: no teammate shows an aiming laser. `flatRing` true: the squad's ring is one flat ring.
- `orders` list of order ids; default all: what the squad's menu lists. `rows` list of verbs; default all: what Space offers.

A test is a record whose `is` says its kind, and `passes` runs it. Where it says n, the default is 1. A kind the build has no row for never passes.

| `is` | members | true when |
|---|---|---|
| in | `area` | Sam is in the Rect |
| near | `at`, `r` (default 1.2) | Sam is that close |
| flag | `flag`, `value` (default true) | a word of `flag_of` has that value: crouch, sprint, wall, peek, lean, nv, light, holstered, gun and others |
| slot | `slot`, `drawn` | that slot is up, drawn or holstered if said |
| pace | `below`, `above` | always |
| meter | `which`, `below`, `above` | noise, or else visibility, is in the band |
| stage | `stage` | the mission has reached that stage |
| door | `door`, `state` | open, shut, unlocked, charged, or else the lock is broken |
| target | `name`, `on` | that light target is on or off |
| men | `down` | so many enemies are down |
| did | `what`, `on` (a door), `n` | he has done it so many times this step |
| downed | `how`, `n` | so many by knife, or else knocked out |
| body | `area` | a downed man lies there and is not being dragged |
| calm | `route` | every enemy, or the man on that route, is on patrol |
| loaded, reload, following | none | he has loaded, is reloading, or all are falling in |
| launcher | `on`, `fired` | that rifle attachment is up, and has just fired if said |
| burst | `n` (default 3) | so many rounds in an automatic burst |
| threw | `kind`, `quick`, `area` | a throw of that kind, and into the area if said |
| orderTo | `who` | the orders menu is on that man or pair |
| stackOn | `door`, `who`, `set` | those men are stacked on the door, and arrived if `set` |
| pairsOn | `doors` | the pairs stand on so many doors |
| word | `doors` | the word has gone at so many doors |
| taken | `room` | that room of the room clear is cleared |
| mate | `area`, `at`, `r` (default 1.5), `by`, `away` | the partner is there, and within `by` or past `away` of Sam |
| pair | `what`, `n` | the pair's log has so many rows that start so |
| cue | `name` | the script has cued it |
| view | `turned`, `zoomed` | the view has turned or zoomed so far |
| squad | `area`, `n` (default all) | so many of the squad are there |
| stacks | `doors`, `n`, `set` | so many stand on each of those doors |
| watching | `at`, `r` (default 3), `n` | so many teammates watch that point |
| all, any | `of` | every, or any, of the tests in the list |

### par

The room clear against a clock (`par/`), only beside a `roomClear`. `gold`, `silver`, `bronze` seconds, over 0 and in that order. `lossSecs` at least 0: added for each teammate lost.

### issue

Kit this level hands the squad whatever the machine keeps (`kit_issue`, `assign_card`). `members` a table from a place on the roster, 0 for Sam and 1 to 4 for the teammates, to a card that
says only what differs:

- `weapon` rifle, pistol or shotgun. `suppressed`, `light`, `nvg` true or false.
- `bangs`, `charges`, `smoke`, `frags`, `walls` whole numbers 0 to 9. `shells` the same, Sam's alone.
- `rings` whole number 0 to 5, and `loaded` ring or he40: Sam's alone, on a rifle. The launcher's sponge rounds and what it comes up on.

### kitView

The Team & kit picture: the team at the mission's way in. Picture only, and no rule reads it. `tools/qa/kitview-plan.ts` asks the plan; `kit_levels.rs` asks that every way stands. `ways` list
of at least one, as many as the team starts; `sets` table of sets by name, default none.

A way needs only its anchor. Everything else lays a written value over what the plan measures (`Way::of`):

- `door` door id, of the set where it names one; or `span` two pairs x, z, a stretch of wall or a gap's posts. Not both. `kind` door, or for a span wall or gap; default measured.
- `set` a key of `sets`: stage this way on that set and not in the level.
- `side` 1 or −1; default measured. Which side the stack forms on. `hand` left or right: which side that falls on screen. `ref` pair x, z: a point on the team's side.
- `pin` record of `yaw`, `pitch`, `dist` (over 1): the camera. `slots` up to five, each x, z and yaw in degrees or null: Sam, then the teammates out from the door. `slotGap` number: the
  written stack's spacing.
- `wall` record of `face`, `thickness`, `top`. `lane`, `out`, `inside` numbers at least 0: the card switch's lane and the free floor in front and behind.
- `pane` record of `box` (a Glass box id) and `stand` (x, z, yaw in degrees), or false for none.
- `hide` list of box ids or groups: what goes, in place of what the plan works out. `hideMore` adds to it. `hidePaint` groups whose paint on the ground goes. `show` list: what stays.
- `indoors` true or false: the men are under a roof, so no photocell. `keepLights` list of groups left lit. `dayLights` list of the way's own lamps or groups that burn by day.
  `litInside` true: the room behind stays lit.
- `roomLights` at least 0: the level's interior lights, times this. Wins over `light`'s own.
- `add` record of `boxes` and `lamps`, in the level's own records and coordinates: more for the picture only, laid after the level's own (`add_level`). New ids, and not with `set`. A
  box's `material` may be alternatives split by a bar, and the first the palette has wins (`set_paint`).
- `light` record over `KIT_VIEW_LIGHT`, key by key (`way_lights`):
  - `porch` the key lamp, `fill` the soft panel, each of `kind` (0 point, 1 spot, 3 area disk), `offset`, `aim` (from the door's middle: along the wall, out, up), `color`, `intensity`,
    `range`, `innerDeg`, `outerDeg`, `radius`, `volumetric`, `broad`.
  - `porch` also: `fixture` record of `at`, `half`, `glow`, and `day`, its intensity at an hour whose lamps are out (default 0).
  - `roomLights`, `moonScale`, `skyScale` at least 0. `sam` record of `on`, `intensity`, `innerDeg`, `outerDeg`, `volumetric`, `aim`: his weapon light.

A set (`kit_set.rs`) is a small stage loaded in place of the level's geometry, keeping its name, materials, surfaces, sky and directional lights: `boxes` list of at least one box with no
shape, whose `material` may be alternatives; `doors` and `lights` lists, optional, in the level's records; `ref` pair x, z, on the team's side.

### response

The team from outside (`qrf/`). `tools/qa/level-needs.ts` holds that a level without a block is in `NOBODY_COMES` with its reason.

- `why` text: who they are and where they come from. `level-needs.ts` asks for a sentence, and the game never reads it.
- `site` text, how control addresses the place. `at` text, the same with its preposition. `team` text with its article. `tag` two to eight capitals; default QRF: their callsigns' stem.
- `ways` list of at least one, the usual first; a run is dealt one:
  - `name` text with its article; said on the net and drawn on the HUD, so no compass word. `door`; default an open way.
  - `muster` four V3: where they stand up unseen. `entry` four V3, `entryFace` four yaws: the fan just inside.
  - `coming` text; default coming in by the name. `clear` text; default nothing: who is told to stay clear. `stairs` true or false; default false: boots on stairs.
- `men` fewest and most, whole, inside 2 to 4. `eta` fewest and most seconds, whole, inside 15 to 240. Both default the game's own.
- `posts` list; default no second tier: `name`, `pos` V3, `yaw`, `sweep` two V3. The lockdown's outside posts, in the order they are manned. `outside` list of V3, at least two with posts:
  where their men stand up.
- `sweep` list of `name`, `pos` V3; default the quiet spots, a raid's quiet rooms and `places`' middles. Where they look with no word to go on (`sweep_places`).
- `raid` record of `room` (a room's id; default any); default none. With it the team answers a raid on the room clear only, and it has no posts.

### doorNames

List of `door` (door id, once) and `name` (1 to 40 letters, no article, no compass word): what a door is called where its id cannot say it (`doornames/`).

### walked

List of `from`, `to` (1 to 64 letters) and `weight` (over 0, at most 8; default 1): ways the level is walked that nothing else says, for the wear map only (`traffic.rs`). A name is a door, a
mark, a route's first point, a room's id or name, a place, or `spawn`.

## How to add a member

1. Its type in `tools/levels/lib/levelData.ts` (or `tools/levels/lib/levelRows.ts`, `tools/levels/lib/scenarios.ts`), its name in `KEYS` or in the block's
   own list in `validateLevelData`, and its limits there.
2. Both directions in `tools/levels/lib/levelLoad.ts`, where the record is not carried as written.
3. The field in its type under `rust/game_types/src/`, and its line in `rust/game_start/src/records.rs` or `level.rs`. A member with no line stops the start.
4. Its reader in the rules, its line on this page, then `bun run tools/levels/export.ts <name>` for each level and `bun run tools/qa/level-needs.ts <name> --run`.
