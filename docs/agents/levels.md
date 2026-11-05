# Levels and the look

One of the area files CLAUDE.md indexes. Everything below was moved here word for word from CLAUDE.md on 2026-10-13, in its old order;
"the bullet above / below" in a bullet means its neighbour there (each block says which lines it was), and a bullet named by its bold
title may now live in another file of this folder: `grep -l '<title>' docs/agents/*.md`. New bullets for this area go at the foot of this file.

<!-- was CLAUDE.md lines 620–692 -->
- **The level is data** (`levels/office.json`, schema `scene/levelData.ts` LevelData v1, docs/level-format.md). `buildLevel()` loads it
  (validate → `levelLoad.ts loadLevel`, every vector copied, box ORDER preserved, id refs resolved to indices); the game never sees ids —
  tools / the editor keep the LevelData as the document and use `indexLevelData` for id → index. To CHANGE the level while the legacy
  generator (`scene/officeGen.ts buildOfficeProcedural`) still exists: edit the generator, `bun run tools/level-export.ts`, commit both
  (the export refuses a file that does not reload to the same Level; `probes.ts level` = generator vs JSON 0 diffs + validator + round
  trip, and `tools/level-export.ts --check` says stale/not; `level-identity` runs AS THE EXPORTER RUNS, on the engine's maths on every
  stand — `tools/qa/headless.ts asExporter` — because the exporter raises no stand: under the game's own maths, `math/exact.ts`, the
  generator's sin / cos round one quaternion's last bit another way, which it prints as a note and does not gate). Hand edits to the JSON are legitimate but make `level-identity`'s "not stale"
  line fail until the generator is retired (lane G1b) — say so when you do it. Flags in the file are names (`NoPrimary NoShadow Cutaway
  Gloss`); runtime bits (Emissive / AreaLit / Rot / Dynamic) are never authored. Unknown keys are validation ERRORS (one `KEYS` registry in
  levelData.ts feeds both the validator and the canonical form — adding a field = interface + KEYS + both directions in levelLoad.ts); the drill
  plans (`plans.*`) are the exception: typed by reference to RoomClearPlan / BreachPlan and carried generically, so re-choreographing the breach
  needs no schema change. A lane that edited the OLD level.ts body: apply its hunks to officeGen.ts and re-export (recipe in docs/level-format.md).
  `game/floor.ts` reads a level's footstep map from its file's `surfaces` block (`Level.surfaces`, non-enumerable; a generator's `build()` may return one); a file with
  none gets the office's map, which floor.ts still carries and `level-validate` asserts equal to the office file's block. Nothing in the
  office was random, so the export froze nothing.
- **Scenario data travels with the level** (`src/scene/scenarios.ts` `LevelScenarios`, `Level.scenarios`; docs/level-format.md *Scenarios*; how-to
  rebuild/scenario-levels/contract.md): what the night breach, the room clear (pair or four) and the hostage rescue take from a level — rooms,
  doors, defenders' marks, the team's start, hostages / captor / door man / roamers, the rescue's route — is a `scenarios` block in the LEVEL FILE, read by
  roomclear.ts / nightbreach.ts / rescue.ts / rescueflow.ts and by nothing else. The office's file has none: its tables are `levels/office.scenarios.ts` (pure
  literals — imported at the top of the game's import cycle, so keep it that way), attached non-enumerably by `withOfficeScript` to any `office` / `office-*`
  level; `STRUCTURE` (roomclear.ts) and `SIDE_MARKS` (rescue.ts) remain as the office's data under the names its probes import, and the team-start knobs
  (`NIGHTBREACH.samAt / samYaw`, `RESCUE.samAt / samYaw / opsAt`) are the office's until `start()` on another level writes that level's (once per team-start
  object: a knob moved by hand afterwards stays moved). A level with NO `plans.breach` stages the room clear without the F drill (`NightBreach.noDrill`: the
  structure's `team` gives the start, every teammate is the order menu's). New levels are generators over `officeGen.ts officeKit()`:
  `tools/levels/<name>.ts` → `bun run tools/levels/export.ts <name>` → `levels/<name>.json` (`test-rooms` = the tiny test level), played with
  `?levelFile=<name>.json&roomclear[&squad4]` / `&rescue`, headless `standUp({ level })`. QA: `bun run tools/qa/scenario-level-probes.ts levels/<name>.json`
  (data checks — validate, `checkStructure`, `checkRescue`, the route — then each scenario staged, walked and finished under a plain scripted player) and
  `probes.ts scenario-levels` (that on test-rooms). Still office-only by constant: weather INTERIOR / ANNEX, chronicle ROOMS, QRF_MARKS (only with `RESPONSE` off — on, the team's places are the level's own `scenarios.response`: enemies.md), NEED_SPOTS, CIV_SPOTS,
  the rescue flow's +x corridor.
- **The default look = 2010s GWOT** (`src/scene/look.ts`, `LOOK.on`: ON in the page — `?look=plain` (or `?look=0|off`, `?nolook`) = every level and man as before; OFF in the module and on
  the frozen harness stand, `TPS_LOOK=1`; one of `FROZEN_HOLDS`' rows, in the knob registry; what the game looks like as a whole, as rules to hold new work to: `STYLE.md` at the repo root). LOOK ONLY: headless no run differs by it (on the page the repainted walls bounce other light into the GPU meter, so a sighting can differ between a level's two files). LEVELS: `LOOK_LEVELS` names the levels that have a SECOND FILE
  in the look (fourteen: compound, barracks, warehouse, depot, registry, block, block-squads, records, consulate, defuse, floodlit, mousehole, killhouse, hall → `levels/<name>-gwot.json`; the table is the list, not this line); `lookLevelFile(file)` is what the page (main.ts) and playthrough.ts load, falling back to the plain file if
  the look's is missing. A look file = the plain file box for box (same index, centre, size, yaw, flags, group, markings) in other PAINT + NoShadow decals appended in group `'look'`
  (`tools/levels/gwotSkin.ts skinLevel` over the plain generator: `tools/levels/<name>-gwot.ts`; `bun run tools/levels/export.ts <name>` writes and `--check`s BOTH files together).
  EACH LOOK FILE IS ITS OWN PLACE (the overnight polish, 10-10): one family of ground marks a level and AT MOST one accent hue (the registry has none, on purpose: it is told by its floor), none shared — compound = a farmer's walled
  house (blue doors; the household on the ground: plot, drying sheets, cook fire, chalk game), warehouse = a bonded customs shed (yellow / black; lane lines, hazard
  bands at doors, numbered doors, pallet squares with tallies), barracks = a police battalion's lines (green; a volleyball court, formation marks, a worn diagonal,
  tally bars at the numbered bunk rooms' doors), depot = an old army motor pool (oxide red; lettered doors A B C, an inspection pit, parking bays), registry = a
  provincial records office (no accent; floor tiles, a red line to the registrar and a blue one through the stacks, row numbers), the block (both files, one skin: `block-gwot.ts SKIN`) = a district telephone exchange (pistachio gloss; cable trenches as chains of 1 m cover plates through the rooms, a painted lifting plate every fifth, a rimmed chamber cover where the trunk branches), records = a district land office (surveyors' magenta, on the ground only; a traverse round the building, a triangle station at every pane with its sight line, a bench mark at the locked door, practice plots pegged out in the lot), consulate = a town house consulate (plum carpet bound in brass — the plum a colour of that file's own, decals only; a roped lane's sockets and a star in the lobby, a runner down the corridor, the hall carpeted wall to wall, a brass sill in every opening; cut from the registry's plan, so it paints rooms where the registry paints routes), defuse = a bomb maker's house (the firing wire's copper; a twin lead along the floor from each bench to one battery plate in the hall, a painted 1 and 2 at the workshop doors), floodlit = the fuel post of a bush airstrip (aviation orange; a landing pad's ring and H in the yard, approach chevrons in through the gate, a wind tee), mousehole = a district clinic (teal; the queue: a dashed line down the corridor to a 'wait here' box under the hatch, a standing square at each room's door, a cross on the porch), killhouse = a training shoot house on a forward base (the range's violet marking paint and white engineer tape; a start box at every way in, a numeral and a clear line at each held room's door, a firing line at the targets), hall = a border scouts' company wing (pale sky blue; the range card painted down the hall: a firing line, a distance line every five metres, the two limits of the far guns' arc) — in those five no wall wears the accent: it is a colour of that file's own that only its ground decals wear, the outside door a darker shade of it (`LOOK_DOORS`). The marks are
  `tools/levels/lookMarks.ts lookMarks(T)` (seg / path / dashed / outline / fill / hazard / checker / glyph / word / tally — a 3 × 5 font for 0‥9 A B C N S), every
  one a floor decal through the skin's own `T.flat`. `Skin.rooms` names MORE rooms for the dado and rug rules than the scenarios do (they list held rooms only: a
  dressed room would mark a held one), `Skin.noRug` holds the automatic rug off.
  THE SIMULATION READS PAINT (particleFx.ts `paletteName / surfOf`, furnblast.ts `breakClassOf / steelGrey`, the destruction / wallbreak / furnblast shape indexes, the wet lot), so every GWOT
  paint on a repainted box STANDS FOR the office paint or surface it replaced: `src/scene/gwotPalette.ts gwotAs(mat, sim)` codes the stand-in as a ≤ 0.0024 shift of the albedo and
  `gwotSimOf(albedo)` reads it back — a new classifier of paint in the game must ask `gwotSimOf` first, as those two files do. THE KIT for new levels: `src/scene/gwotKit.ts gwotKit(...)`
  (walls, parapets, gates, HESCO, drums, cots, rugs, lanterns, wash lines … flat colours, boxes only) + `GWOT_MATERIALS`; the showroom `levels/gwot-yard.json` (`tools/levels/gwot-yard.ts`,
  `?levelFile=gwot-yard.json&solo&hour=midday`); the one-page guide is rebuild/play-style-levels/look/read-first.md. QA: `probes.ts look` (tools/qa/look-check.ts, in
  all-quick: the palette has no clashes and no other level carries one of its colours; per pair — both files fresh, boxes equal but paint, repaints classify the same, extras are decals,
  props the same bodies, the three destruction indexes list the same boxes, the wet lot reads the same, every other key equal); whole-run identity = `PT_DIGEST=1 PT_DESTR=1
  tools/qa/playthrough.ts --scn <name>.json` against `--scn <name>-gwot.json`. Not following the switch: the other tools/qa level loaders (they name a file and get that file). Seen only on
  the software adapter. FLAT DECALS THAT CROSS ARE 0.7 MM APART (`tools/levels/flatLayers.ts`, `FLAT_STEP`; the maintainer, 10-11, on the block: "seeing some zorder issues on the floor"): the skins used to tell two crossing floor decals apart by 20 µm, under one step of the depth buffer at the camera's own 21 m (depth32float, near 0.5: about 1.2e-7 × z² m a step — 52 µm at 21 m, 190 µm at 40), so on a GPU every pixel of an overlap went to either paint; `skinLevel` now ends on `layFlats` (two flat decals of DIFFERENT paint that overlap on the plan, or a decal and a thin flat of the level itself, have their tops ≥ 0.7 mm apart, the order the skin wrote kept, nothing moved down, no centre across a centimetre so a checkpoint's level fingerprint is the same), `export.ts` prints how many it raised, and `probes.ts look` holds it per look file (`LOOK_BREAK=1` must fail). A new site that lays a floor decal needs no height of its own beyond the ORDER it wants. EVERY LEVEL'S OWN FLOOR IS SWEPT TOO (the maintainer, 10-11, on Training 3: "more bad zsorting in tutorial level" — the gravel court and the generator slab lay over the yard's dirt with all three tops at 4 mm): `tools/levels/export.ts` runs `flatLayers.ts layFloor` over every level it writes, before the look's skin — everything drawn whose top face is at the floor (ground finishes, mats, floor decals; `isFloorFlat` / `atFloor`), in TIERS (the generator's boxes, then its lived things, then the dressing: a later tier never moves an earlier one, so a level with its furniture or dressing left out is the same level), a SOLID finish raised by growing 0.7 mm thicker with its underside where it was, a decal lifted whole; `isFlat` and the skin's `under` reach 5 cm up now (marks laid on a carpet). `bun run tools/qa/floor-order-check.ts` (`probes.ts floororder`, also under `look`; `FLOOR_BREAK=1` must fail) holds it in EVERY level file, the office's included. A re-export that moves heights wants its commit at the front of `tools/qa/lived-check.ts`'s base revisions (the levels owner's list). The class's real fix is a reversed depth buffer — the renderer's, not done. A SKIN'S DECALS ALL START AT THE FLOOR (rugs, patches, wear at every door, the damp foot + the strip of ground along an outside wall, a painted dado in
  lived-in rooms; `skinLevel` throws and `probes.ts look` is red on one that does not): wallbreak.ts lets a drawn-only static that hangs OFF the floor fall when what it touches breaks
  (debris, a sound — a run would differ), and never asks one that starts at the floor. A decal on a wall's face is hidden with its wall when the wall is holed by `src/scene/lookTidy.ts`
  (called once a frame by main.ts; picture only; whole with the wall's box, on any level that carries one of the look's paints and no other). The same file has the look's two other page-only passes: a GROUND decal goes wet and dry with the
  ground under it (the storm soaks the lot in place and knows nothing of decals: render-only `Box.wetK` + the ground's finish, `albedo` never written), and `lookDoors` (once, after the game stands,
  under the switch) repaints the door LEAVES of a level that wears the look — outside doors in the level's accent paint (its row in `look.ts LOOK_DOORS`, keyed by the level's own file; without a row the accent is counted off its boxes among blue / green / oxide red, army green where none), inside ones weathered wood or the row's `in`; steel doors and the office are left alone. Wear is the ground's own colour darker (`SkinTools.tone(floorAt(x, z), k)`), never paler; THE PALETTE'S GROUND IS DARKER THAN ITS WALLS
  on purpose (midday exposure flattens earth tones: `gwotPalette.ts`'s ground comment), and a repaint of an unnamed non-steel box must not be a plain grey or white (`steelGrey` would
  make it a steel cabinet — the skin's check throws). THE MEN'S DRESS FOLLOWS THE PLACE (the maintainer, 10-10: "middle eastern enemies also don't make sense for the office level"): on the OFFICE family the page boots the FIRST cast file and tables with the look on (`look.ts setLookPlace(placeOfQuery(q))` in main.ts before the cast is fetched; `lookDress()` = what `castFile()` / `charMatsOf` / `charMatsVersion` ask, `lookOffice()` = `cast.ts officeDress`: of the first cast's defenders only the ones dressed for a building — no straw hat, boonie, bandana, bandolier or vest top), the sneaking suit as on any level; not a knob, and everything headless leaves the place true (`probes.ts cast-look` holds both). THE MEN: a second cast file `public/assets/chars/cast-gwot.gltf` (`tools/chars/build_cast.py --look gwot`; `gltf.ts castFile()`
  picks by the switch — helmets and plate carriers, pakol / turban / shalwar kameez) and `charMaterials.ts CHAR_MATS_LOOK` (`charMatsOf(side)`: MultiCam-toned operators with small pair
  markings, defenders in muted local dress); dress only — weapons, goggles and kit are the gear part's (`GEAR.on`). THE SNEAKING SUIT (`LOOK.suit`, on with the look; `?suit=0` / `?nosuit` = the issue dress on every mission; the maintainer: "sam should also be in a black sneaking suit in stealth missions (probs not loud ones)"): `src/game/cast.ts sneakSuit(g, c)` — THE one predicate: Sam, and a quiet mission's partner, on any mission that is NOT a squad's loud one (`scenarioRun.ts activeScenario` null or 'quiet': the quiet missions, the plain solo night, Training 1–4 and 7; a room clear / night breach / rescue = the issue dress) — is drawn without the helmet, the back kit and the mesh goggles (`anim/gltf.ts castSuit(cast, mask)`: parts named `helmet` / `kit_*` dropped — the cast has no free part, so the suit IS the operator's own `op_suit`: uniform, vest, boots; `sdfrig.ts fitRigCast` solves those masks ahead of play with the rest) in a third set of rows, `charMaterials.ts SNEAK_SUIT` (one matt black; `charMatsOf(side, suit)`, a trailing `suit` on `slotAlbedo / writeCharPalette / describePalette`; the per-man cache in charLook.ts keys on it; `__charmat.tables.suit`, `__charmat.get('sam').suit`). Asked every frame; looks only (`probes.ts cast-look` holds the parts, the rows, loud vs quiet, both switches; `charmats-off` holds the suit off to ask about the side's own table). Not followed: `?nocharmats` (parts only, his raw tint), the viewer. THE SUIT'S RIFLE IS BLACK (the maintainer, 10-10: "would expect a blacked out rifle build for stealth loadouts"): `rigProps.ts RigOpts.rifleBlack` ← combat.ts handProps asks `sneakSuit` — the 2010s carbine's dark earth stock, grip, handguard, post and laser box are drawn in the bullpup's black polymer on a man in the suit, in his hands and on his back; a loud mission's rifles keep the tan; colours of five boxes only (`cast-look` holds both, and that not a box moves); the dropped `M4_ITEM` is not followed. Known: the box goggles and their mount arm were placed for a helmet's brow and sit on the hairline of a bare head; a pair in the suit differ only by face and shoulder patch. `probes.ts cast` / `charmats` hold both looks (`TPS_LOOK=1`).
- **Lived-in rooms** (`src/scene/lived.ts`, ONE switch `LIVED.on`: ON in the page — `?lived=0` = every level as it was — OFF in the module and on the frozen harness stand; `TPS_LIVED=1` in
  front of ANY bun tool raises it (the module reads the env itself, so a tool that only parses a level file follows it; headless.ts pins it with the other stand words); a `FROZEN_HOLDS` row, in the
  knob registry, a journal's `env.lived`). The maintainer, 10-10: "the office level is awesome and it has a lot of personality per room, we should makes sure all levels are like that". What furnishes
  a room of another level as the office's rooms are furnished rides in the level FILE and `levelLoad.ts` leaves it out with the switch off: a BOX whose group is `lived` or `lived.<room>`, a PROP,
  LIGHT or SWITCHABLE whose id starts `lived.`. OFF = each level box for box, prop for prop, light for light as before there were any (nothing kept moves index, nothing kept may name a lived box);
  ON is real furniture — cover, things to walk round and to break — so it is NOT the off game by digest. `tools/levels/export.ts` holds the file's rules: lived things are the TAIL of the generator's
  boxes / props / lights / switchables, and the dressing (dress.ts) is dealt as if they were not there, appended after them ([generator, lived, dressing, look decals]). Each level's lived things
  are written in that level's own generator; the yardstick (what the office does per room), where furniture may not go and the budgets: rebuild/play-style-levels/rooms/read-first.md.
  A checkpoint does not cross the switch (the fingerprint counts boxes), as it does not cross the look's. The look's skin repaints the furniture with everything else and deals its DECALS as
  if it were not there (gwotSkin.ts `bare`), so a look file with the switch off is the old look file too; the exporter refuses a kept thing that names a lived box (fixtures, the
  breaker, beacons) and a lived thing standing IN the dressing; the level editor, its lint and its swing deal load the whole file (`lived: true`), as the probes that hold a file against
  its generator do. QA: `bun run tools/qa/lived-check.ts [level … | path.json] [--sheet] [--base <rev>|none]` (none named = every level that carries a lived thing): the file without its
  lived things is the base's (the level's own row in `LEVEL_BASE` in the tool, else `BASE_REVS`) key for key and loads to the same Level; with them every authored point (starts, marks, pods, waypoints, deals) keeps its floor, every mark's aim line
  stays open, nothing solid within 1.1 m of a door's line, no floor cut off from the spawn, no walk between authored points more than 30 % + 1.5 m longer; `--sheet` = what each room got.

<!-- was CLAUDE.md lines 2303–2306 -->
- **An older level is dressed from the look's kit, and its look file says what its own colours are** (dress every level, 11-03; tools and level
  data, no switch but `LIVED`). Three things a level with a second look file can now do:
  · ITS OWN COLOURS: `tools/levels/gwotSkin.ts Skin.own`, a table from a colour the generator wrote as numbers (a lived room's wall to wall floor, a
  yard's packed earth, a car) to the palette paint the box wears in the look file. Asked after the level's `pick`, before the shared rules, the colour
  matched as written, through the same stand-in (`gwotAs`) and the same check that the simulation reads the same of the box. A target that is no row of
  the palette throws. The shared rules had no word for such a colour, so the box kept it and drew flat on the look page.
  · THE LOOK'S KIT ON AN OLDER FLOOR: a generator on the office kit or the functional kit lays `src/scene/gwotKit.ts` pieces in its lived tier through
  `gwotKit(gwotOfOffice(K))` / `gwotKit(gwotOfKit(k, K))`: a round drum, a tyre stack, sandbags, crates, field stones. The skin LEAVES a box, and a
  prop's part, that already wears a palette paint or a stand-in and no paint of the office's (`inLook`); before, it painted them over by the office
  paint each stands for (a blue drum olive, a field stone the house's wall). So on an older level a palette paint comes only from a kit piece, never
  typed into the generator: the skin would leave it and nothing would repaint it.
  · THE GROUND: a look file's `dress` lays the lot's wear before its paint: `G.ruts` where wheels go, a trodden beat where feet go (`lookMarks path` in
  the ground's own paint through `T.tone(T.floorAt(x, z), 0.8)`), `G.patch` and a second finish (`M.fill` in a ground row) where the use leaves one.
  The `ground` need of `tools/qa/level-needs.ts` reads the look twin on its own line.
  TRAPS: a lived PROP a kit piece makes (`jerryCan`, `gasBottle`) wants its `lived.<room>.…` name written by the generator. A solid that tops out
  between 0.20 and 0.25 m stops a body on floor the walk grid calls free (lived-check is red on it). A level's KEPT square boxes (a yard's own drums,
  a tyre room's stacks, a car as two boxes) cannot become kit pieces without moving the frozen stand: they are left.
  CHECKED: `export.ts --check` on all fourteen look files with and without each tools step (none moved); `look-check.ts` on a twin that carries kit
  pieces (the kill house: all its lines pass, the destruction's indexes list the same boxes in both files). NOT SEEN ON A GPU.
- **The kill house is dressed** (dress every level, 11-03; level data: its lived tier under `LIVED`, `?lived=0` = the level as it was, and its look
  file; no new switch). `tools/levels/killhouse.ts` had no lived tier: it has one now, the RANGE ROUND THE HOUSE, laid from the look's kit by name
  (`crates`, `jerryCan`, `drum`, `tyres`, `sandbags`, `blocks`, `woodPile`, `rubble`): the ready line's ammunition, water and tyres, the end of the
  south sandbag line, lane markers and sandbag returns at the butts, the house's spares on the dark north side, field stones. Nothing stands in a
  start box or on its way to its door, within 2 m of the house's walls, or before a window. `killhouse-gwot.ts` lays the ground before its violet
  marks: the range road and a spur (ruts), the beat round the house and out to the firing line (the lot's own earth darker, a faint shoulder either
  side), gravel between the firing line and the targets. ROUND THINGS 0 → 18 (16 tyres in seven stacks and 2 drums: the count files
  `tools/qa/fixtures/round/killhouse.txt` and `killhouse-gwot.txt` ride in this commit); the look twin's `ground` need passes (was one bare stretch of
  542 m²); the picture with no material 1.4 % → 1.7 %.
  LEFT, kept boxes (a base move): the three tyre stacks in the tyre room and the south sandbag line are the level's own square boxes.
  MOVES on the shipped stand: the kill house's play-through digests on both seeds (furniture is real), and any recording, checkpoint or kept replay
  made on it parts from this step (the fingerprint counts the lived boxes). The frozen stand's level is the level it was.
  CHECKED on the step this is cut on: `lived-check` (the look file's base, `fixtures/level-base/killhouse-gwot.txt`, rides in this commit),
  `level-needs --run` on both stands, `look-check` (the kill house's lines all pass), `map-check` (48 scripted runs, none threw: it plays the frozen
  stand, so it does not meet the furniture).
  PLAY-THROUGHS (`PT_DIGEST=1 tools/qa/playthrough.ts --seeds 1-2`, 21 missions): frozen 42 of 42 the same digest as the step; shipped with
  `TPS_LIVED=0` 42 of 42; shipped 40 of 42, the two that differ the kill house's own, both still 'taken' (97 s → 94 s with one man fewer standing, 90 s → 91 s with the same three).
  WHY, from a trace of seed 1: the same to the tenth of a second until 51 s; then the stack on the mess door forms 0.8 s sooner and the man 6.6 m inside
  the mess wins an exchange he lost before. Nothing new stands indoors. ON 24 SEEDS, the level as it was against dressed: taken 22 → 21, men standing
  43 → 50 of 96, mean 92 s → 94 s, none of it outside the scatter. A squad a few tenths early or late at a door flips that room's fight either way.
  Pictures: before and after from the play camera, the level's night, dry, on the software adapter. NOT SEEN ON A GPU; nobody has played it.
- **The registry's and the consulate's own floors are materials** (dress every level, 11-03; their look files only: paint, no box added or moved, no
  switch). Each lays a wall to wall floor of its own colour in every lived room (`livedPieces floor`), and the skin had no word for those colours, so
  on the look page they drew flat: 23 % of the registry's picture and 28 % of the consulate's had no material. `registry-gwot.ts` and
  `consulate-gwot.ts` now say what each is (`Skin.own`). THE REGISTRY keeps a room told by its floor from above: oxide floor paint in the lobby, blue in
  the copy room and green in the hall (the look's tiles lie on them), cream cloth in the reading room, a red carpet in the registrar's office, the
  bare slab between the stacks, hessian in the sorting room, an ochre runner down the gallery: 23 % → 4.1 %. THE CONSULATE's staff floors are none of
  them a colour, because the plum carpet is that place's one accent: grey felt, bare boards, matting, hessian, slab: 28 % → 22 %.
  LEFT on the consulate, on purpose: the plum carpets and their brass (5.2 %: an accent is a colour of the file's own, and the look gives a material by
  colour only), the kitchen's terracotta (about 3 %: the palette has no tile, and its oxide paint made that floor louder than the plum), the back
  verge in the palette's scrub (7.6 %: a plant's paint, no ground the look can draw), and the strip at a wall's foot where
  gravel darkened lands on the palette's oil (1.1 %). They wait for a box that says its stuff by name, a tile and a grass ground; none is bent to a row.
  MOVES: nothing a run reads (the new paint stands for the old one: `look-check` line 3), so no play-through digest on any stand; the look files'
  bytes, so each twin's base for lived-check rides in this commit (`fixtures/level-base/registry-gwot.txt`, `consulate-gwot.txt`). A picture or a
  recording of these two shows other floors.
  NOT SEEN ON A GPU; nobody has played it.
- **The floodlit yard and the two charges are dressed** (dress every level, 11-03; level data: lived things under `LIVED`, and their look files).
  THE FLOODLIT YARD: its yard was one box in a colour of the generator's own, 23 % of the picture with no material: it is gravel now (a second ground
  inside the wall: the dust stops at the gate), and the drums, the generator, the sandbags and the pickup wear palette paint (24 % → 2.5 %). Its look
  file lays the track along the south wall and in at the gate, the pickup's ruts, the beat from the front door round the landing pad to the gate
  (nobody walks across a pad), the walk from the track to the shed, and oil under the drums. Its lived tier gains, from the look's kit, the empties
  dumped in the dead east end (3 standing drums and one on its side), two spare tyres by the pickup, the generator's drum in the shed and the stones
  turned out of the gateway. The open ground between the gate and the door stays open; no lamp changed (`floodlit-check` green on both stands).
  ROUND 0 → 6. THE TWO CHARGES: the workshops' floor is the bare slab its look file says it is, the hall's runner and the big mat are plain cloth, the
  two
  workshops' own mats keep their red and blue as carpet (from above that is how the two charges' rooms are told apart), the two cars wear car paint
  (4.8 % → 1.3 %); the lot has the street's ruts, each car's way in, the walk to the front door and the beat that parts before the house and goes
  round each end to a workshop's back door (the level's two ways to each charge, worn into the ground); and from the kit, sacks and two drums beside
  the cars and firewood at the far edge of the back strip (off the defenders' way round: the way-round bullet below says why), two drums, the spare
  wheels, firewood at the back, field stones. ROUND 0 → 4. Both look files' `ground` need passes (192 and 315 m² were bare).
  LEFT, kept boxes (a base move): the yard's three fuel drums, both levels' cars (two boxes each), the sandbags under the eaves. The pad's orange and
  the wire's copper are accents.
  MOVES on the shipped stand: any recording, checkpoint or kept replay made on either level; the two charges' play-through digests (the floodlit
  yard's two keep theirs: its new things stand where no route passes). THE TWO CHARGES' TWO STANDARD RUNS END SOONER, both still 'taken'
  (112 s → 65 s, 194 s → 83 s): the squad's walk across the lot is 0.2 s later past the two new drums and the fight inside falls differently; no path, door or
  defender's place changed and nothing new stands indoors. ON 100 PAIRED SEEDS the dressed level is the same for who wins and how many men stand
  (taken 95 → 92, 7 runs flip to a loss and 4 to a win, 297 → 294 men of 400) and ends 9.7 s sooner on average (118.9 s → 109.2 s), win or lose; why
  sooner is not read. THE TWO CHARGES' SHIPPED DIGESTS MOVE ON EVERY SEED (3 of 100 pairs are the same run): a later read that finds them moved
  against a build before this step has found this step, not a red.
  Count files and look-file bases ride in this commit (`fixtures/round/`: floodlit `0 6`, defuse `0 4`, plain and look file; `fixtures/level-base/`:
  both look files). The frozen stand's levels
  are the levels they were. A YARD OF OTHER PAINT BOUNCES OTHER LIGHT: the page's light meter reads that bounce and the headless one does not, so what
  a man reads in the lit yard on a GPU is not measured here. NOT SEEN ON A GPU; nobody has played it.
- **The long hall is dressed** (dress every level, 11-03; level data: lived things under `LIVED`, and its look file; stands on the step that made the
  lived kit's drums round). The hall itself stays bare: it is the level. Round the wing, from the look's kit: the porch's ammunition crates, a drum,
  the sandbag line carried on, the tyres off the truck; firewood and two drums against the blank south wall; spare block at the back; field stones.
  Nothing stands in the strip the hall's two guns see through the open front door, on a start mark, or before the bunks' window. The look file lays
  the camp road, a spur to the sandbag line, the walk to the porch and the sentry's beat right round the wing, under the bunks' window (the one other
  way into a room). ROUND 2 → 10. The look file's `ground` need passes (453 m² was bare).
  MOVES on the shipped stand: the hall's play-through digests, recordings, checkpoints and kept replays. Its two standard runs: seed 1 a wipe at 92 s → taken at 118 s, seed 2 a wipe at 83 s both times.
  From a trace of seed 1: the same to the tenth of a second until 70 s, the same three men lost the same way to the hall's own guns; then the last
  man wins by a tenth of a second the exchange at the post's door that he lost before. ON 24 SEEDS, as it was against dressed: taken 11 → 11, men
  standing 20 → 19 of 96, mean 88.5 s → 88.7 s. Count files (`fixtures/round/hall.txt`, `hall-gwot.txt`: `0 10`) and the look file's base ride here.
  NOT SEEN ON A GPU; nobody has played it.
- **Furniture does not take a defender's way round** (a LEVEL RULE with a check, 11-03; no switch). A squad whose neighbour has lost a room sends a
  pair ROUND to its other door (`enemysquads.ts` decide → `wayRound2`): each man needs a standing place outside that door and a path to it no longer
  than `pressure.ts flankMaxM` (46 m) and its two fellows. Lived things are real, so they can take the place or lengthen the path, and then the
  defenders go straight through the house instead: nothing fails, they are only less clever. FOUND ON THE TWO CHARGES: the way round the outside of the
  house is 45.5 to 45.9 m as built. Dressed, the pair went round on 46 runs in 100 where it had on 60 (19 seeds lost the move, 5 gained it), the
  sweep-and-hunt endings fell 33 → 20, and that was the whole of the run ending 9.7 s sooner. The search's own answer at the decision, same state to the
  centimetre: 45.90 m bare, 46.88 m dressed. What did it was FIREWOOD behind the house: with the team HEARD in the hall the search prices the ground
  near it and the way bows out to about 2 m off the back wall, onto the pile; with nobody known it hugs the wall and misses it.
  THE CHECK: `tools/qa/wayround-check.ts` (shipped stand, and frozen; seeds 1-3). Each mission (every row of `ALL_MISSIONS`) with defender squads is
  stood up with `TPS_LIVED=0` and `1`, and the game's own search is asked through `squadWaysRound` (a read-only export of `enemysquads.ts`): for every
  man at home, from his post and from a stride inside each door of his room; to every other squad's room, every door of it as the team's; with nobody
  of the team known AND with the team in the room (in its doorway, 3 m inside, and heard on every 2 m of its floor); going in or holding.
  RED: a way LOST with lived rooms on, or SQUEEZED (made longer by more than the search's grain and a fifth of the room it had, and under 2 m from its
  limit). `WAYROUND_BREAK=1` must fail. It leaves out the team's guns and men off their posts. What a mission already owed the day it was written is a
  HELD BAR in a file of its own, `tools/qa/fixtures/wayround/<mission>.txt` (`<lost> <squeezed>`): warehouse-four 58 lost and 28 squeezed,
  compound-four 21 and 107, barracks-four 22 and 20, defuse-four 23 squeezed (shipped), pass-four 9 squeezed (frozen). A mission is red when it has
  more than its file; a mend takes its file out in its own commit. `tight as built` on a line is the level's own edge with no furniture at all: the
  two charges have 89 ways in 902 within 2 m of the cap, so a man a stride off his mark already loses some. That is the cap's, for whoever owns squad
  sense.
  THE MEND on the two charges, two moves, `defuse.ts` says why: the firewood is stacked at the far edge of the back strip, and the sacks and two drums stand
  beside the cars (against the front wall they squeezed the front way and made the squad's first walk 0.2 s later). The check is green and the held bar is
  out. ON 100 SEEDS the mended level is the level before it was dressed on every count: taken 95, the pair goes round 60, sweep-and-hunt endings 33, mean
  118.9 s, first contact 28.5 s; no seed loses the move and none gains it. With the first move alone it was 45 runs of 100 and the check was red.
  NOT YET a line of `level-needs.ts` or of a gate family: run it by name after furnishing a level with squads. NOT SEEN ON A GPU.
- **A roof over walkable floor is the kit's ceiling** (a LEVEL RULE, no switch): `K.ceiling` (levelKit.ts `ceiling()`: a slab flagged `NoPrimary`, so the camera sees the floor under it) or a slab
  flagged `NoPrimary` by hand — and, if rain should stop at it, inside `weather.ts roofsOf`'s height band: underside at `WORLD.ceilingY` − 0.25 (2.75 m) or over, top no higher than + 0.45, at
  least 0.9 m across both ways (`roofsOf` asks only height and size, never the flag: a DRAWN slab there stops the rain and hides the room). Floodlit's generator shed had the one drawn roof
  in fifty levels (the levels owner's count).
- **A dado stops at the door's jambs** (level data, no switch; the maintainer, 10-13, on the quiet lesson: "weird box in front of door in duo stealth mission"). `train-silent`'s watch house
  had its rust-red paint band (`G.dado`, a 1.1 m NoShadow decal on the wall face) written around a gap centred on the hut door's `at`, but `at` is the opening's LOW end
  (`tools/levels/train-ground.ts` hut doors), so the right-hand piece ran 0.58 m across the 0.86 m doorway and read as a slab standing in it. `tools/levels/train-silent.ts` now paints
  24.7‥26.3 and 27.2‥28.5 (the fitted doorway is 26.3‥27.2); the regenerated json moves two boxes only. Checked headless: export `--check`, level-lint, quiet-check, lesson-check,
  door-size ALL PASS. NOT mended, same class, the levels owner's: `crossing.json box_0069` (a 0.38 m mudDark skirt across door `gate_ne`) and `glasshouse.json box_0059` (the same across
  door `back`); `level-lint` has no decal-over-doorway rule, which would have caught all three. Never seen on a GPU.
- **The depot and the block are authored floors, one problem each** (10-13; `tools/levels/depot.ts`, `block.ts`, `blocksquads.ts` and their look twins; ids, rows and file
  names kept; no switch: it is level data, so saves and F7 / F10 recordings of the old floors are refused by fingerprint). Each generator's head comment has the problem, the hand
  placement and why, and the measured table; the page is `rebuild/authored-levels/depot-and-block.md`. THE HAND PLACEMENT is seed 0 = each room's first `garrison`
  marks in file order (the block's marks ARE its garrison, so it has no dealt floor). DEPOT: "the signals hut and the store's hatch hold the lit yard in a crossfire from the dark,
  and the hut's own door and the mains are at its back"; its check is `probes.ts depot` (45 verdicts a stand, `DEPOT_BREAK=1` fails), which judges the yard by the PLACE's own
  light (`lightEstimateAt(g, P, () => true)`: under the dark rule a slit man burns his lamp across the park when the mains go) and sweeps the unheld rooms. BLOCK: "every held room
  turns a corner, and its men are round it"; four L rooms, two men in each far leg, a lit notch room with the room's other door; block-four is ONE STAY-PUT SQUAD A ROOM
  (`BLOCK_SQUADS_HOLD`, `mayLeave: false`: as one pool on hard most of the eight walked out to the first shots), block-squads keeps three squads and a third man in each near leg;
  the block has no check of its own. MEASURED (the scripted commander, the one floor, 12 seeds, shipped stand, men up of 48, the build with the dark rule): depot standard / hard —
  obvious 4 / 7, round the back with the mains cut 32 / 41, the same with the lights on 23 / 27, the workshop 48 / 40 (FREE on standard), second start obvious 16 / 13; block (hard =
  standard run for run) — hall doors 0, round by the other doors 41, lights out then hall doors 13 (24 before the dark rule), lights out then round 41, second start round 37,
  a wall charge 7, thresholds 8. KNOWN: the block's lights-out answer is weak since the dark rule; the depot's workshop way is cheap. Checks keyed to the OLD block floor are red
  until their owners follow (enemysquads, corridor-check, wallsafe / wallcharge-squad / -standoff, wallreact, words-doors, cues-run, levers-rules wants `squads: true` on
  block-four's row, loudchaos-levels, the scenario-level probes' "it can be won"). Never seen on a GPU, never played; software stills only.
  Since then (10-13): block-four's row says `squads: true`; both floors got paper and a racking bay so `loudchaos-levels` passes (play identical run for run); and
  `tools/qa/scenario-level-probes.ts` has a table of three files, `TRAPPED` (depot, block, block-squads): its commander walks only the obvious plan, so there
  "walked" asks that the team left its marks (where it fell is printed) and "it can be won" is NOT ASKED (the answers are tallied with playthrough.ts routes; the pair
  is not these floors' team). `TRAPPED_BREAK=1` must fail. Every other level's lines are as they were.
  THE MESS TABLE (10-13, later): with the lived rooms on (the page, the shipped stand; `TPS_LIVED=1`) a stack of three or four on `m_s` FROM INSIDE the mess
  was refused 'no room on that door' — the long table stood on the west wall, on the file's marks. It is a shorter table now, in the near leg's far corner
  (z 4.5‥5.7), a nav cell clear of the squads file's third man (11.7, 6.5) and of the coats on the rail; lived things only, the bare level and the frozen
  stand are untouched. The check: `bun run tools/qa/lived-stack-check.ts block-four,block-squads` (was FAIL 2 of 128, now PASS; it is in no gate — run it
  after moving any lived furniture). What moved with it: `m_k+` deals two men (17.27, 8.25) (16.67, 8.85), one hand, where it split the jambs (the glass
  rule: nobody can stand in that corner and shoot through the hatch now). MEASURED old floor → new, same code (9165eaf3), Hard, shipped, seeds 1‥12, won
  of 12 · men up of 48: hall doors 0 · 0 → 0 · 0; heel doors 12 · 34 → 12 · 31; the dark then the hall doors 9 · 14 → 8 · 10; second start, heel doors
  12 · 38 → 12 · 35; block-squads heel doors 12 · 37 → 12 · 32. The falls that differ are in OTHER rooms' notch rooms (the frame room, the jointers'), so
  this is the run parting after the mess, and twelve seeds cannot tell it from noise; the same sign in every arm is said, not explained. (The numbers
  above this line are 3159ad70's: the build since moved the heel doors 41 → 34 before this change.) A STALL TO KNOW (not a red on
  the newest join): on one join with the build (f66c870b) `scenario-level-probes levels/block-squads.json`, shipped stand, failed "finished" — seed 2's
  team lived through the mess, cleared three rooms and ran out the clock, the workshop's last man (Bravo may leave its rooms) standing in the SWITCH ROOM
  at (26.7, 18.5) from 60 s on, where that commander never looks. One build join later (64504143) the same three seeds end and the check is ALL PASS on
  both stands. Not the table (the old floor's seeds are wiped before they get that far): a man of a squad that may leave can park in a room nobody holds.
  Never seen on a GPU, never played; the page booted on the software adapter.
- **The depot is dressed: its yard is poured, and its look twin says what things are made of** (11-03; `tools/levels/depot.ts`,
  `depot-gwot.ts`; no switch of its own). THE YARD: the hardstanding wore `block`, which the look lays in courses like a wall on its
  side, over a quarter of the picture from above; it is `slab` now with the signals hut's raised floor, and the rooms that had lino are
  `woodGrey` boards. `DEPOT_PAINT` names the generator's own colours and the twin's `own` gives each its palette row: `olive` for
  the trucks, the signals shelter and the generating set, `drumRed` for the oxide drums, `paintRust` for the main gate's leaves,
  `iron` for the stove, a hub and the pickets. The twin's `dress` lays the wear FIRST, under the letters and the bays: the ways
  between the doors, the back way from the gap to the generator room's door, the track round the outside of the wall. ONE LIVED
  PLACE MORE, `lived.road`, outside the wall: five kit bushes past the track and broken block at the wall's foot. Nothing new stands
  inside the wall, where every metre of cover is the level's problem, and nothing new is lit: the floodlit yard and the dark
  nests are the level's idea and were left as they are. Left flat: tyres, cartons, car glass, the shelter's chassis (a hair too
  dark for a row to stand for it), rubber mats, the kit's scrub. KEPT SQUARE: the five fuel drums are boxes of the level's own
  part, on the frozen stand.
  CHECKED (no GPU: counted and drawn on the software adapter, never seen on one, never played): `lived-check` green on both files
  (the twin by its base file, `tools/qa/fixtures/level-base/depot-gwot.txt`); `level-needs depot --run` no FAIL on both stands;
  play-through digests, 21 missions on seeds 1 and 2: frozen 42 of 42 as before, shipped with the lived switch off 42 of 42, and
  shipped as it ships the depot's two runs keep their digests too (the scripted player never meets the scrub). The look page's
  audit: 5.2 % of the picture with no material before, 3.5 % after.
  THE TWIN'S FINGERPRINT MOVED (paint and decals), the plain file's by its lived things: a checkpoint or a recording made on the
  depot on the shipped stand parts from this step; none was re-recorded.
- **'(OLD floor)' after the block's name in these files** (10-13, the docs tidy after the rebuilt block was named, steps 219 / 220) = that figure, hold, door or mark was measured
  or staged on the block as it stood BEFORE the rebuild and has not been taken again: the six-hold lane figures and the standing-team table in enemies.md, the wall charge's marks
  in destruction.md, the door order tallies in door-tools.md (`mb` is gone; `m_s` is another door now), the pairs-by-door figure in team-feel.md. Tagged where the block's owner
  listed a line; an untagged block or depot figure dated before 10-13 is the old floor's all the same (the depot was rebuilt too). Still true of the rebuilt block, checked
  against `tools/levels/block-gwot.ts`: the look bullet's telephone exchange (pistachio, cable trenches as cover plates, a lifting plate every fifth, the chamber cover). TWO
  CHECKS THE TAGGED LINES NAME were red on the rebuilt floor when the tags were written (968d8756: `probes.ts corridor` on shipped, its hold on block-four drawing nobody;
  `probes.ts wallsafe` on frozen, two lines) and their owners have re-staged both since: on the build of 8c2c7cea (10-13) corridor is ALL PASS on both stands — it holds
  `block-squads` now, see enemies.md — and wallsafe is ALL PASS on frozen. The tagged FIGURES are still the old floor's; only the checks moved.
- **The block under the look: its walls are render, each room's floor is what it is made of, its standing rounds are the kit's** (dress every level, 11-03;
  no switch: level data, the look file by `tools/levels/block-gwot.ts`, the rounds under `LIVED`). The look draws a box as what its paint says, so a paint
  picked for its colour is drawn as the wrong stuff. The twin had the cement walls in `hesco` (drawn woven); they are `render`. The rooms' own floors
  (block.ts lays one wall to wall in each lived room, in a colour of its own) went through the furniture questions and came out cloth, olive steel, mud and
  nothing: `FLOORS` in the twin, five rows of the skin's own-colour table (`gwotSkin.ts` `own`), says what each is. `oct` / `oct8` in block.ts call
  `livedPieces.ts at().round`; the loose drum's and the gas bottle's bodies stay parts written by hand (a prop's part cannot say it is round). KEPT ON
  PURPOSE: the pistachio accent (`prodGreen`, a row with no material, 5.8 % of the picture: the level audit's 10 % bar is red on the block by one point
  until a paint can name its stuff) and the lino in `canvas` (a paint row was tried: the look chips paint to red and the floor read as rusting steel). The
  squads file is exported with it (`export.ts blocksquads`). Bases: `tools/qa/fixtures/level-base/block-gwot.txt` and `block-squads-gwot.txt`; round counts
  `tools/qa/fixtures/round/` (0 and 6, four files). Measured: the 21 missions' play-throughs on two seeds against step 1239, three reads (frozen, shipped
  with `TPS_LIVED=0`, shipped): the block's four runs (block-four and block-squads, seeds 1 and 2) keep their digests on all three, and so does every other
  level's. Drawn on the software adapter only: never seen on a GPU, never played by a person.
- **The level check plays the level's own marks too** (`tools/qa/scenario-level-probes.ts`, 10-13; tests only, no switch). The room clear parts (`pair`, `four`)
  default to seeds `0, 1, 2, 3` (they were `1, 2, 3`): seed 0 is the hand placement the page deals on every load (`game/samestart.ts`), 1 to 3 are dealt floors
  of the same file. The rescue part keeps `1, 2, 3`; `--seeds`, when given, is taken by both. A seed is the placement AND the run's die, so the page's floor is
  ONE run here and 'it can be won' can still pass on a dealt floor alone; the page's floor over many dice is a tally's job (`PT_SAMEFLOOR=1` playthrough.ts).
  MEASURED on c14cb206, the new default through `pair` and `four` on the twenty level files with a room clear to play, both stands: SEED 0 FAILS NO LINE on
  any of them (staged, walked or left-their-marks, finished); every red is on a dealt seed and all of them stand on seeds 1 to 3 alone. One red goes green:
  the warehouse's pair on shipped (the win moved from dealt seed 1 to seed 0 with the hand placement; taken, 1 of 2 up: one run, thin). REDS LEFT, none the
  hand placements': shipped — the compound's pair (wiped on all four), Mousehole's pair (wiped on all four: the commander's moves into the store and the duty
  room are refused, 'door is locked', 10 of 15 orders; it has no order for a barricaded door), the barracks' four (dealt seed 1 runs to the cap, 3 of 4 up),
  floodlit's four (wiped on all four; its generator says it is checked with `--seeds 3,4,5,6,7,8`); frozen — the compound's pair (dealt seed 1 to the cap, as
  the compound's bullet says) and floodlit's four (dealt seed 2 to the cap). A pair-only exception table for Mousehole was written and taken out again the
  same day: it would have hidden the locked-door gap and dropped a line that passes on the frozen stand. `TRAPPED_BREAK=1` still fails (depot, four).
  `probes.ts scenario-levels` ALL PASS on both stands with the new default.
  MOUSEHOLE IS THE FOURTH `TRAPPED` ROW (later 10-13, on 953cfc71; the barricade owner's reading of the red above): the 'locked' doors are no gap. The ward,
  the exam room and the store have ONE door each, wedged from inside on purpose, and are reached by their walls (charges are issued for it); this commander
  gives door and move orders only, so the plan it walks is the one the floor traps (its owner's tally, as passed on: doors from the yard 0 of 24, walls 22; the walled plan
  is playthrough.ts's own commander, `PT_WALL=1 PT_WALLOUTER=1`, held by `wallissue-check`). So on `mousehole.json` and its `-gwot` twin 'walked' asks only
  that the team left its marks and 'it can be won' is NOT ASKED, as on the depot and the block; the count is still printed. WHAT THE ROW STOPS ASKING THAT
  PASSED: the four took the floor by the doors on seeds 0 and 2 on shipped and on 3 of 4 on frozen, and the pair on 1 of 4 on frozen; those wins are luck
  against a floor built to beat that plan, not a property to hold. Both files, both stands: ALL PASS; `TRAPPED_BREAK=1` exits 1 on both.
- **The level check asks a win only of the floor's own team** (`tools/qa/scenario-level-probes.ts fourManRowsOf`, 10-13; tests only, no switch). A level whose
  room clear rows on the mission list (`src/scene/scenarioList.ts`) are ALL four-man is not asked 'it can be won' of the PAIR: the line prints NOT ASKED with
  the pair's count and the row's id. Staged, walked and finished are still asked of the pair, and the four's lines are untouched. A level with a pair row
  (records) or with no row (the office) keeps the line. Read off the list, not a table of file names. WHY: the compound's pair was red on shipped (wiped on
  seeds 0 to 3; its HAULHOME owner's reading, as passed on: 3 of 16 deals with the rule on, 5 of 16 off, a five-on-eleven floor nobody built for two), and
  picking seeds that win would have held nothing. WHAT IT STOPS ASKING THAT PASSED (measured, pair part, seeds 0 to 3, both stands the same count unless
  said): warehouse 1 of 4, barracks 1 of 4, floodlit 1 of 4, glasshouse 2 of 4, the compound 1 of 4 on frozen; those were never promises. STILL RED: the
  compound's pair on FROZEN, on 'finished' (dealt seed 1 to the 240 s cap, the compound's own known line), which this rule does not touch. CHECK:
  `PAIRTEAM_BREAK=1` asks every pair again and the compound's pair on shipped exits 1. Records and the office: PASS, still asked, both stands for records.
  `probes.ts scenario-levels` ALL PASS on both stands.
- **The level check's rescue player leads the hostages out** (`tools/qa/scenario-level-probes.ts` `Walk.leadOut`, 10-19; tests only, no switch). Its Sam
  walked to the door, pressed F and then only held the room. With the page's ending up (`RESCUEEND`: a held room ends nothing) and the covered lanes stopping
  the man the flush sends short of the room, nothing was left to end the run: on the shipped stand `scenario-levels-rescue` (the test level) stalled at its
  150 s cap on all six seeds, and the frozen stand, where the 15 s hold still ends it, stayed green. Now, 8 s after the room is the team's (`LEAD_AFTER`), he
  walks to the nearest hostage, takes her row and walks them to the way in, asking again every 5 s; with the ending off he holds as before. MEASURED, shipped
  stand: the test level 6 of 6 rescued, both hostages out at 40 s (it was 6 of 6 stalled); the registry (3 seeds, cleared at 26 to 28 s) and the consulate
  (its own two red lines, the staging count and seed 2's walk) are line for line what they were on the build before. Frozen: unchanged (5 rescued, seed 3
  lost). The stall was the check's player, not a man who never comes: he is sent 4 s after the room falls and waits at a corner short of the lane, and this
  player (god mode, kept near unseen, never firing) neither met him nor left. Never seen on a GPU, never played.
- **The pass is the fifth `TRAPPED` row of the level check** (`tools/qa/scenario-level-probes.ts`, 10-14, on build 583bb5a9; tests only, no switch). Its own
  sentence (`missions.md`, 'A level made for snow: the pass') makes the white, lit yard between the lorry and the doors the problem, watched through glass
  from two sides, with the answers round the back (the kitchen's `back` door, way 2) or with the mains cut; this commander walks way 1 across that yard.
  BEFORE THE ROW, seeds 0 to 3: red on both stands, on 'walked' alone on shipped (a DEALT floor wiped in the yard at 9.0 s, pair, seed 1, and at 12.8 s,
  four, seed 3, before anyone reached a door) and on 'walked' plus the four's 'it can be won' on frozen (0 of 4). SEED 0, the hand placement the page
  deals, got a man into the tea room on both stands with both teams. WITH THE ROW: 'left their marks' passes on every seed, both stands, both teams;
  `TRAPPED_BREAK=1` exits 1; `probes.ts scenario-levels` ALL PASS on both stands. WHAT THE ROW STOPS ASKING THAT PASSED: the four took the floor by the
  front on shipped on seeds 0 and 2 (3 of 4 and 4 of 4 up). NOT A CLAIM THAT THE TRAP TRAPS: the level is `untuned`, and its owner's own tally has the
  front way winning most runs on shipped; whether the yard should cost more is the level's tuning, not this check's.
- **A level does not deal the squad's kit; `bun run tools/qa/kit-audit.ts` lists what its mission gets** (2026-10-18). The issued kit and the named
  squad kits are one table for every mission (`src/game/kit.ts`, `docs/agents/menus.md`: the squad's kit is dealt over the men who come). A level's
  file adds to it in two ways only: `scenarios.issue.members` (a block of cards by place on the roster, the fields that differ, written against the
  pair's layout; the Team & kit screen then shows no cards, and a level's issue card is put on as written)
  and `scenarios.roomClear.wallCharges` (wall charges of its own, whatever the cards carry: nobody is issued one). The listing's row for a mission
  says the sizes it offers, whether the cards can be changed there, the level's own kit, its locked and steel doors against the two door charges a
  squad is issued (no hammer or shotgun opens steel, and only a team of three or four as issued is lent a big-slot tool: `KITDEAL.more`, menus.md), whether frags can be ordered at all (never with a hostage,
  the man a snatch came for or a civilian in the blast: `squadthrow.ts`), and whether the Team & kit picture has a place for each way in
  (`scenarios.kitView.ways`). A listing, not a gate: it always exits 0.
- **THE LEVEL SKILL AND ITS ONE COMMAND** (10-18; tools and a list only: no switch, no game code, no level file touched). The maintainer, on a Team & kit
  screen where the blue pair carried a rifle and nothing else: "as issued still has blue team pretty empty" · "i think we should make something like a level
  skill that goes through everything that's needed for a level based on what we've seen" · "seems like we miss a lot". `.claude/skills/level/SKILL.md` is
  that list, in the order a level is made (before you touch anything · the idea · the file · rooms, doors, walls · lived-in rooms · light, hour, weather ·
  the mission in the file · the team and its kit · what the player reads · measure it, then say it), every need with what was SEEN when it was learnt and
  the command that checks it; what no tool can ask is marked by eye or by play. `bun run tools/qa/level-needs.ts [name …] [--run] [--jobs 4] [--skip need,…]
  [--quiet] [--md f] [--json f]` asks the nineteen needs a file can answer about what stands AROUND a level (generator · mission · row · music · picture ·
  kit picture · look · lived · floor · ground · leak · key · lamps · moon · doorway · roof · kit · response · said) of one level or all, in about a second, a look twin with its plain file.
  The floor itself stays `tools/qa/map-check.ts`'s (structure, size, nav, the scripted runs): `--run` runs that with `--no-runs` and the level's other own
  checks as they stand (export `--check`, level-lint, door-size, second-way-check, marksight-check, walked-check, lived-check, kitview-plan, level-names-check, floor-order-check,
  quiet-check, lesson-check), four at a time: about 13 s for one level, 90 to 120 s for all. Exit 1 on a FAIL; a NOTE never fails. `NEEDS_BREAK=1` plants
  gaps in seventeen needs on one level in memory (the kit's by emptying every card, whatever the issue table gives) and must find each by its own words
  (exit 1; a MISSED one exits 0). THREE NEEDS ARE NEW CHECKS: `doorway` (a wall decal across a door's opening: the dado rule had no check), `roof` (a drawn
  slab in `weather.ts roofsOf`'s band: the floodlit shed's), `kit` (as issued, through the game's own cards and pouches, at every size the row offers, who
  comes with nothing but a gun; the team's size is set before a card is read, so an issue shared out over the men who come is read as shared). AS THE 36
  LEVELS STAND on build 8dbdb23, both stands the same: 464 PASS, 24 FAIL, 65 NOTE, and with `--run` 73 of 74 own checks exit 0. THE FAILS: `kit` on all
  22 rows that take three or four (the issue is one table for five, `game/kit.ts ISSUED`: the squad's stuns, charges and frags ride on the first two
  teammates, so Blue One at three and both blue men at four carry nothing; the long hall's own card and the two levels that issue wall charges do not
  change it; ONE table's mend, the kit's, turns all 22 green and no level should patch it with a block of its own); `doorway` on the crossing (`box_0069`
  across `gate_ne`) and the glasshouse (`box_0062` across `back`); and the one red own check, `map-check snatch --no-runs` (its footstep zone `hut` lies
  outside the building rectangle and is never read). NOTES: 26 rows still `untuned`; no footstep map on block, block-squads, consulate and registry (the
  office's shell) and on the records office (not the office's shell); lamps under open sky and off the photocell on crossing, depot, floodlit, lights-out,
  quiet-relay and train-kit; rooms with fewer than three things in them on six levels; locked and steel doors against two door charges on eight; two
  frags issued where a frag on order is refused on the three rescues and the snatch; a partner kitted as for a loud mission on five quiet rows; four
  level files no mission plays. NOT MENDED HERE: each is its owner's. NOT IN A GATE: it is red today on the kit, so it is in no family and not in
  all-quick; wire it in once it is green (`--skip kit` gives the exit code of the rest meanwhile). NOT ASKED BY IT: anything by eye or by play (the idea,
  the five checks, light per room, sight lines at the starts, whether furniture reads from above, whether a picture looks like play), and `said` holds
  only that a level is NAMED in these files. It stands no game up: nothing here was seen on a GPU or played.
- **The records office dressed for the look** (level data, no new switch; 2026-11-03; "we'll want to dress up all the levels and check their budget", and the
  library steer: pieces by name, paint by palette row). `tools/levels/records.ts` lays its yards from the library in the lived tier (`G = gwotKit(gwotOfKit(k, K))`,
  groups `lived.yard`, `.water`, `.edge`, `.lane`, `.stones`): precast barriers on the kerb south and east (open where the team stands, at the three bays and at
  the street's end), a ladder and two ranging rods on the yard car's roof with two tyres and two drums behind it, a hand pump, a water tank and a dish out in the
  west yard, two stacks of block on the back lane, field stones; the burn drum is the kit's drum. `records-gwot.ts`: `SKIN.own` gives the generator's own colours
  a paint of the palette (the two cars truckTan and truckRed with glassDark cabins, the kerbs tWall, the sofa blanketBrown, the ash gravel, the readers' coir
  coyote; a grey body cannot wear truckWhite, the skin's check throws: a pale grey stands for steel), the dado is truckTan (the cloth row drew a woven wall), and
  UNDER the surveyors' marks the ground is laid as it is used with the kit's patches and ruts at 8 mm (a patch at its default height lies OVER a mark): grit at
  the kerbs, the bays' ruts and oil, paving to the front door, the staff's way round the west side, the back lane's bed and ruts, a drain, damp where roofs shed.
  NOT MOVED: any box of the level's own part in the plain file (the two cars and the kerbs are kept boxes), a lamp, the hour, a mark. KEPT CLEAR, each from a
  review: the lane along the west wall that a man at the front office's corner holds the back door down (a tank and a dish stood there blocked 40 of 126 of his
  lines), the ground against the outer walls (a wall charge's setter), the look's chain, sight lines and glyphs. WORDS: the lived things go with `?lived=0`
  (`TPS_LIVED`), the paint and decals with `?look=plain` (`TPS_LOOK`); the frozen stand has neither. CHECKS: `export.ts records --check`; `lived-check records
  records-gwot` (the twin's own part moved: its base is its own file under `tools/qa/fixtures/level-base/`); `level-needs records --run` on both stands (the
  twin's `ground` note goes from three bare stretches to a pass); `level-lint`; `kitview-plan records`; `wallfurn-check --levels records.json` (the same lines
  as before); `probes.ts look`. Round things: 0 bare, 6 lived, in both files. MEASURED (play-through digests, 21 scenarios, seeds 1 and 2): frozen 42 of 42 as
  the step before; shipped with `TPS_LIVED=0` 42 of 42; shipped, every other level's runs keep their digests and records-pair's two move (seed 1 wiped at 100 s
  as before; seed 2 taken at 27 s where it was 39: the pair is on its first stack 0.3 s sooner, the defenders' reinforce order deals another way and two of them
  are caught at the office door as they dig in). Two seeds is a look, not a count. The look page's audit: 4.9 % of the picture with no material to 3.3 %, boxes
  drawn 1102 to 1263, round things 0 to 6, lamps as they were; the file is 1073 boxes where it was 897 (past lived-check's warning at about 900, as five other
  level files are). PARTS FROM THIS STEP: a checkpoint or a recording made on the level with the lived-in rooms on; `tools/qa/fixtures/coop-reference.json`'s
  records-pair row. SEEN: stills on the software adapter only, night and midday, on the look page and on this build's page, no console or device error. Never
  seen on a GPU, never played by a person. LEFT: the two cars are the old kit's two boxes (they want the library's curved vehicle, and a base move); bins, stool
  and buckets are square props (a prop's part cannot say it is round); the kit's drum is sealed where a burn drum is open; the lot's edge wants a knee rail the
  library does not have.
- **The two foot bands that ran across a doorway are mended** (level data, no switch; 10-18; the two the dado bullet above left as "NOT mended", the `doorway`
  need's two reds). `G.skirt` lays one band the length it is given, so a band given a whole wall crosses that wall's doors. Glasshouse: the back wall's foot
  was `G.skirt('x', 8, 10, 30, -1)`, 0.76 m of it across door `back`; it is two calls now, 10 to 13.76 and 14.74 to 30 (the frame's outer faces). Crossing:
  the east wall's foot across `gate_ne`, cut at 6.76 and 7.74 the same way in its own step. EACH FILE HAS ONE BOX MORE, so every unnamed box after the band
  moved up one id: a list that names boxes by typed id moves with it (the crossing's door picture finds its eleven now, counted off the generator's own
  calls, and throws when the count is not eleven; the glasshouse names none). A LEVEL'S FINGERPRINT MOVES WITH ITS BOXES: a tape or a checkpoint taken on
  either level before this refuses to load. `lived-check`'s base moved to each step in turn (`BASE_REVS`, newest first). CHECKED, glasshouse, both stands
  unless said: export `--check`; `level-needs glasshouse` has no `doorway` line; `lived-check`, `level-lint`, `level-identity`, `door-size`, `kitview-plan`,
  `secondway`, the `glasshouse` probes, `scenario-levels`; one boot of the mission on the page with the software adapter (no page error, no band left
  across the doorway, the door there). NEVER SEEN ON A GPU and not played: that the wall's foot reads right either side of the frame is by the numbers.
- **The border crossing's ground outside its wall is dressed** (dress every level, 11-03; `tools/levels/crossing.ts`, the tail of `furnish`: level
  data under `LIVED`, no switch of its own. On the page `?lived=0` is the level as it was; the frozen stand never loads it, and `TPS_LIVED=1` raises
  it on a frozen run). The level command's `ground` need named ONE bare sheet on this level, 154 m² of dust, and it is the lorry park outside the
  south gate: where the pair starts and the camera first rests. Laid from the kit alone (`G.ruts`, `G.stain`, `G.hearth`, `G.rubble`, `G.tyres`,
  the lived pen's `tyre` and `chock`), in two new rooms. `park`: the track the lorries wear along the wall (from the west it ends at the border's
  striped post, where they are halted; the rust box came in from the east and stands on its own wheel lines, which run off the world); the dust dark
  where the box's tractor idled and where feet go to and from the gate; the drivers' cold fire under the wall; a wheel left behind the box's tail;
  stones the mud wall has shed at its foot. `verge`: the hollow the ditch wicket is named for, the beaten ground before the tea stall, the dust dark
  at the customs yard gate and the impound yard gate, dark field stones. Paint and walk-over stones but for the wheel, which stands between the wheel
  lines two metres and more off the wall; no stone within two metres of where the Team & kit picture stands its file, and none where a wall hides the
  ground from the camera. WHAT THE FIRST STILLS TAUGHT, for the next level dressed at night: a patch of ANOTHER paint on bare dust (oil, dirt, the
  hearth's default damp ash) reads as a mat, so every stain is the ground's own paint darker (`stain`, 0.78 with a smaller 0.62 turned inside it) and
  the hearth's ash is `dirt`; a stain is paler than a rut, so it lies UNDER the ruts (tops under 8 mm) or it paints them out; the kit's pale `mud`
  stones are broken wall, right at a mud wall's foot and a pile of cartons out on open dust, so the verge's are `gravel` (`rubble` still makes
  three in ten of any scatter its own `mudDark`: the kit's to mend). NOTHING INSIDE THE WALL MOVED, and no lamp, hour or lit panel: the light was
  judged first from a still of each court and left as it is (the lane's two sodium lamps are its key, the forecourt and the fence door have a bulb's
  pool each, the lot and both yards are the hiding dark, the park is the moon's with the lane's glow over the wall). COUNTS: 895 to 999 boxes in the
  file (past `lived-check`'s 900 warning, as any addition was), props 28 to 30, round things 26 / 28 to 26 / 30
  (`tools/qa/fixtures/round/crossing.txt`: the wheel's two tyres). `ground` passes (the largest bare piece left is 30 m² of dirt in the lot, dark on
  purpose). MEASURED before and after on both stands, step 1245: `quiet-light` and its `--blackout` map are the same cell for cell but for the tyre
  stack's own cells outside the wall on the shipped stand (its red line, a dark way to the office, is red before too); `quiet-check` 27 of 27
  deals; `quiet-pair` seeds 1 to 12, three arms (alone, the partner following, the partner used): 36 of 36 runs the same run on the frozen stand,
  which does not load lived things, and 36 of 36 on the shipped stand, where the tool loads all 999 boxes: the scripted pair starts on the track and
  walks north through the gate, and never meets a stone. (The shipped tallies, the same both times: nobody across alone or with the partner only
  following, 2 of 12 with the partner used; the row is marked untuned.) The page's own light on Sam at the start mark is 0.509 before and after.
  NONE OF THE 42 PLAY-THROUGHS IS THIS MISSION: their digests only show nothing else moved. CHECKED, both stands unless said: export `--check`,
  `lived-check`, `level-needs crossing --run`, `level-lint`, `kitview-plan` (142 passes, as before), `round-check` line 1 (red with a wrong number
  in the file), the `quiet` probes, `wallfurn-check` (shipped: the same 36 places), the generators' typecheck; `response-check` passes on the frozen
  stand and its line 8 is red on the shipped one before and after (the police post's). A TAPE OR A CHECKPOINT of this level taken with lived rooms
  on parts from this step (the list of boxes is longer); one taken with them off does not. NOT MENDED, a general gap: a rain the player PICKS wets
  the world's ground slab and not a decal lying on it (`weather.ts setWet`), so under a picked rain these marks, like any level's marks on that
  slab, stay dry and read paler than the wet ground; the level's own night is dry. NEVER SEEN ON A GPU and not played by a person: stills of the
  start and the park before and after on the software adapter (the look page as frozen, Chromium 141, the level's night, dry) are all anybody has
  seen. Whether the stones read as stones and the stains as wear at 60 frames, nobody knows.
- **The level command's `kit` need reads the issue AS WORN, and the levels are green** (10-18; a tool and the skill only: no switch, no game code, no
  level file). Since the squad's kit is dealt over the men who come (`game/kit.ts KITDEAL`, the bullet "A level does not deal the squad's kit" above),
  `tools/qa/level-needs.ts` reads the five cards as a staging wears them: the issue, a level's own cards written over it, then `kitWorn(n, cards)` (dealt
  only while the counted things are still the issue's, as the game deals them), and a man is bare with no pouch filled AND nothing in his big slot
  (`kitdeal-check.ts comesBare`'s test). So a level whose own card changes a stun, a charge or a frag is red again at three and four unless it writes a
  card for every man who comes. The needs are asked OF THE PAGE AS IT OPENS on either stand (`host/profile.ts pageSwitches` with no flag and `kitLoad`
  with nothing kept, what the shipped stand boots with), because a rule the frozen stand holds off would call a man bare whom no player sees bare; a
  refused `slots.json` stops the tool with that word. The stand still decides what `--run`'s own checks see. `KITDEAL_BREAK=1` in front is the need's own red path: the
  cards as the table writes them, red on the 22 rows that bring a third man. A level's own cards are worn as written (the long hall's block
  names nobody since the shield left the game: its four come in the plain issue). The skill's kit items say the same, and its kit picture item gains a rule from the kit figure's side: where a
  `kitView` hide list names box ids, whoever shifts the level's box ids builds it again from the generator's box range in the same step. AS THE 36 LEVELS
  STAND on build 1e1f943, both stands the same: 488 PASS, no FAIL, 65 NOTE (the two `doorway` reds and the snatch's footstep zone of the bullet above were
  mended by their owners); `NEEDS_BREAK=1` exits 1 on both. With `--run` every one of the levels' own checks exits 0 on both stands. STILL NOT IN A GATE (its owner's call). NOT ASKED YET, each waiting on
  its own work: a need for a level's response-team block (asked since: the bullet at the foot), and one for a rescue that ends only on extraction or a full clear. Nothing here was seen on a
  GPU or played, and nobody has gone through the 36 levels by eye against the skill's by-eye items.
- **The level skill after the shield left the game** (10-18; the skill's words only). Its kit item said a level's own card that hands out a tool the game
  has switched off is dropped whole; with the ballistic shield cut, a card that still says `shield` is REFUSED as the file loads, in words
  (`src/scene/levelData.ts`: "take the key out (the rest of the card stands)"), and the skill says that. `tools/qa/level-needs.ts` was brought in line by
  the shield's own step (issue cards are read as written). On build ba34118, both stands: 488 PASS, no FAIL, 65 NOTE; `KITDEAL_BREAK=1` 22 FAIL;
  `NEEDS_BREAK=1` exits 1.
- **The level skill says where a Team & kit picture stands** (10-18; the skill's words only). Since every way of a `scenarios.kitView` block must anchor on
  a door that is not steel (`docs/level-format.md`; `kitview-plan.ts` section C fails any other, `kitview-work.cjs` holds the page to it), the skill's kit
  picture item says so, with what to write where a mission's own way in is a gate, a gap or a wall. `--run` already runs `kitview-plan` for a level, so the
  level command is red on such a way with no change to it. On build d2ec4b7, both stands: 488 PASS, no FAIL, 65 NOTE.
- **The level skill says how a check holds a file against its generator** (10-18; the skill's words only). The skill's "a check of its own" item now says
  such a check runs the generator as the exporter does (`tools/qa/headless.ts asExporter`), since under the page's own maths a generator's sin / cos round a
  last bit another way than the file was written with (the `level-identity` probe was red on the shipped stand for that alone), and that staleness is
  `export.ts <name> --check`'s question. `tools/qa/level-needs.ts` needed nothing: it runs no generator in its own process, and `--run`'s export check is a
  fresh process that raises no stand. On build 956661a, both stands: 488 PASS, no FAIL, 65 NOTE.
- **The level skill says a rescue's start is also its way out** (10-18; the skill's words only; the rule is `missions.md`'s "A rescue is won out, or
  cleared"). Its mission section gains one item for whoever lays out a rescue: the team's start mark (`scenarios.rescue.team.sam`) is where the hostages are
  led to, so it wants standing room and a way back a hostage can walk; said as the game is now, since the way out being the way in is an open call. No need
  was added to `tools/qa/level-needs.ts`: `probes.ts rescueend` walks each rescue's hostages out. On build ab96d87, both stands: 488 PASS, no FAIL, 65 NOTE.
- **The level skill asks whether a response team comes (`response`, the sixteenth need of `tools/qa/level-needs.ts`).** The rule and which levels have a
  team are `docs/agents/enemies.md`'s ("The response team is the level's own", "A squad's raid brings a level's team too") and `docs/level-format.md`'s
  (`response`): nothing of them is said again here. The need only asks that a level with a mission row has ANSWERED, from files, standing no floor up.
  WITH a `scenarios.response` block (the office's is `levels/office.scenarios.ts`'s): its `why` is a sentence (over 40 characters, as
  `response-check.ts` asks), and a look twin carries the same block word for word, else FAIL; and a NOTE when no row of the level is one the block
  answers (`qrf.ts live()`: a `raid` block is called only inside a squad's clear that is no rescue, any other only outside every squad mission; the
  office's plain night is no row). WITHOUT one: the level has a line in the tool's own table `NOBODY_COMES` (level, and why nobody comes in a few
  words; the long form stays the two bullets of enemies.md), else a NOTE, never a FAIL: a new level is asked, not stopped. In the table AND carrying
  a block is a FAIL (one is stale); a name in the table that is no level with a row stops the tool. Lessons are not asked. The need first read the
  enemies.md lists as prose and a review showed that passing levels on any word after the list (a level with its block deleted among them): hence a
  table. `block` and `hall` are in neither on purpose, so they are the need's two NOTEs: enemies.md says either would suit a team and has none yet,
  the maintainer's call. Marks, ways and a call played through stay `bun run tools/qa/response-check.ts`'s (about 90 s, so not in `--run`); each
  level of its `WITH` list has a call played by a line of its own (records since its line 21), so a new block wants a new line there. `NEEDS_BREAK=1` plants the unanswered level, then a block with a
  one-word `why` under a name the table holds beside a twin with another block, with rows neither kind of block answers: three FAILs and three NOTEs
  it must find. On build d5382d5, both stands: 513 PASS, no FAIL, 67 NOTE (27 lines of this need: 6 levels with a why, 2 twins, 17 levels in the
  table, 2 NOTEs). Read from files only: no call was played by this need, and nothing here was seen on a GPU.
- **Every door a row called by a bearing has a name of its own** (10-19; level data under `WORDS.doors`, the dialogue file's bullet has the switch and its
  words; no new switch here). `scenarios.doorNames` is written in 25 level files, 96 doors, and `words-probes.ts DOORS_OWED` is empty: a door that turns up
  called by a bearing is red there until its level names it. WHERE A NAME COMES FROM: the level's own generator, by what the level already knows the door
  by. A gate the response block names keeps that name ('customs yard gate', 'impound gate', 'street gate': one name a door); another gate is named by what
  stands outside or just inside it ('lorry park gate', 'tea stall wicket', 'motor pool gate', 'dish gate'); a room's door by the room ('spare room door',
  'corridor-side pantry door', in the made names' own style); a door of several in one wall by the room beside it ('fire door by bunks 3'); a lesson's door
  by what the lesson's line calls it ('red door', 'blue door', 'grain store door'). THE TRAINING GROUND (`train-ground.ts finish`) names its two gates in
  one place, 'track gate' and 'far gate', for every level built on it; a level that writes its own row for a gate keeps it (lights-out, the vault). A look
  twin takes its plain file's names. THE LEVEL WRITER could not write the block: `serializeLevelData` took every scenario block for an object and wrote a
  list as fields keyed '0', '1', which the validator refused on the read back; a block that is a list is written as a list now (no file without one
  changes). Each of the 25 files differs from the build before by its `doorNames` block and nothing else (read back and compared), so no fingerprint
  moves; `lived-check`'s base moved to this step. CHECKED, both stands unless said: `words` (9 probes; `DOORNAMES_BREAK=1` red), `level`, `quiet`,
  `registry`, `picks`, `journal`, `response`, `vault`, `lights-out`, `secondway`, `missions`, `lived-check` 407, `lesson-check` 73, `quiet-check`,
  `kitview-plan`, `level-names-check`, `level-needs` (no FAIL), both typechecks. NOT MINE AND RED ON THE BUILD BEFORE TOO, shipped stand only:
  `scenario-levels-rescue` (the test rooms' rescue stalls on all six seeds), `checkpoint-autosave`, `checkpoint-civilians`. ONE ROW READ ON THE PAGE (the compound's four, the software adapter, no page error): a man's row says 'on
  the side yard gate'. NOT DONE: no other row was read on a page, nothing was seen on a GPU or played; the names are words by the generators' notes, and whether each reads right from above is the maintainer's
  eye. LEFT, the same class but not door names: a quiet level's `exits` and `starts` and some `places` areas are still named by bearings ('the east gate',
  'the west track', 'north strip'), and the debrief's map marks print an exit's name.
- **An exit, an area and a door no longer read by a bearing or by letters** (2026-10-19; level data, no switch: the names are the files' own words, and the
  door names ride the door names key as before). The words check's two owed lists (`PLACES_OWED` 56 names in 20 files, `LETTERS_OWED` 23 doors in 10) are
  EMPTY. A quiet mission's way out is called by its gate's written door name: bazaar the street / alley / lane gate; crossing the customs yard gate, the
  impound yard gate, the lorry park gate, the tea stall wicket, the ditch wicket; police post the impound / motor pool / side yard gate; relay the dish / track /
  back strip gate; quiet yard the foot / road gate; the training ground the track gate and the far gate (`train-ground.ts EXIT_W` / `EXIT_E`), which lights-out
  and the vault take with their own names (the back gate, the generator shed gate; the counting room gate, the kitchen court gate). A START or a civilian's way
  out that names the same gate or track follows it (crossing, quiet yard, police post, bazaar 'outside the alley gate', relay 'the track' and 'the slope',
  lights-out and vault 'the front track' / 'the back track', the lessons 'the track'). AREAS: barracks armoury gallery / wash room gallery; compound side yard,
  alley; depot back yard; lights-out back strip, window side, front track, back track; police post side yard; vault counting room gateway, kitchen court
  gateway, narrow strip, shack side, front yard, back yard, front track, back track; warehouse service corridor, staff corridor. DOORS (`scenarios.doorNames`
  rows): barracks bunks 1 / bunks 2 / bunks 4 / armoury / wash room door and 'bunks 1 to 2 door'; compound 'pantry back door'; crossing 'customs office to
  impound door'; quiet yard 'bunk room to radio room door' (the look yard 'guest room to sleeping room door'); the orders lesson's yellow house / green house
  / armoury / kitchen door; warehouse 'locker room to break room door'. Twins take their plain file's names. Checked: in the 22 level files that moved, every
  changed line is a name; with the key up 23 doors read new words and no other door's name changed (a scratch run over every file, before and after);
  `probes.ts words` 67 PASS on both stands and `DOORNAMES_BREAK=1` red; lived-check 407 on both with this step's commit in front of its base; level-names,
  level-lint, kitview-plan, quiet-check, lesson-check, the debrief, quiet, missions and depot families green. LEFT, still a bearing, not on any owed list: the
  compound's `ways` on the mission list (`scenarioList.ts`), a `needs` place ('the south wall' on the quiet yard, 'the east gate' on the crossing), route ids a
  defender's line may say, and the office's own 'north side' words. Nobody has read a renamed line on a GPU page.
- **The level skill after the van and the wall switches** (10-19; the skill's words only, no need added to `tools/qa/level-needs.ts`). Its mission section
  gains an item for a snatch (`scenarios.snatch`: the man, two exits or more, and `van`, where the mission is now won: `docs/agents/missions.md`, *a snatch
  ends in the van*; checks `snatch-check.ts` and `snatchout-check.ts`). Its light section gains one for the wall switches (`docs/agents/stealth.md`, *Wall
  switches*): no file places a plate, so what a level owes is plain wall beside its doors and lighting groups that are its circuits; check `probes.ts
  lightswitch`. The snatch level's response block came with the van, and its line left the command's table of levels where nobody comes in the same
  step (not by this one). On build 9f632e9, both stands: 513 PASS, no FAIL, 67 NOTE; both red paths red. Nothing of either was seen on a GPU or played here.
- **A pane's glass comes from where the level puts it** (2026-10-21; a rule in `src/game/glassbreak.ts`, `GLASSREAL.place`, on in the page and off on the frozen
  stand: `destruction.md`, *Glass as buildings have it*; no level file touched). Foot under 0.7 m, or a panel of a door = toughened (one round, the sheet drops, the
  opening is walked); from a sill = float (a hole, then daggers); a narrow inside sidelight or a high outside strip = laminated. The level skill's section 3 has
  the rule as a need; `probes.ts glassreal` line 9 holds every level's count of panes by kind, on and off, and a level with new glass adds its row there.
- **The level skill asks that the ground is not one flat sheet (`ground`, the seventeenth need of `tools/qa/level-needs.ts`, 10-22).** The maintainer, on a
  shot of the tutorial's fuel halt (`tut-solo`) at night: "ground looks pretty flat here. we should make this part of our level skill". The skill's
  lived-in section gains the item (finishes, wear, stains, low things to walk over; `STYLE.md`'s rules for a patch). THE MEASURE (`bareGround`, from the
  file's boxes, props and paints, no floor stood up): the world in 0.25 m squares, each with the TOPMOST flat paint drawn over it (a box no higher
  than 6 cm that is a decal, a leaf of 2 cm or less, or the world's own slab; one under half a metre wide is widened by half a square so that a painted
  line is not lost, and a speck under a square long is nothing). Two paints read apart at 0.15 of the larger value, 0.3 of roughness, or 0.08 of any channel's share of the colour (the look's
  lightest door wear is 0.84 of its floor; a patch of its ground's own paint is no mark, and dust beside gravel is one sheet). A BREAK is the open line
  where two paints that read apart meet, a mat under 10 m² with markings on its upper face, and anything standing on the ground no higher than 0.19 m,
  the height feet step over (rubble, a kerb, a loose thing by the game's own footprint for it; a pad wider than 3 m is a floor, and paint). Squares under anything standing (below
  1.9 m), and on a door's line, are not ground the eye sees and nothing passes them: a break dresses the squares within about 1.5 m of it, walked round
  the walls, and two rooms are never added up through their door. What is left is the stretches, each named for the paint most of it wears. A stretch
  is OUTSIDE THE PLACE, and only added up, when the player cannot walk to it from any start mark of the file (`sam`, the solo night's `playerSpawn`;
  doors open), or when four fifths of its squares lie within 4 m of the world's edge and no start mark stands within 3 m of it. 40 m² or more in one piece is a NOTE, never a FAIL: one line a file, the six
  biggest with a square of each; a look twin is asked for itself. THE NUMBERS ARE FIRST NUMBERS (40 m², 1.5 m, 0.15, 0.3, 0.08): nobody tuned them
  against a picture, a step of roughness alone may not show at all, and the measure does not hold `STYLE.md`'s rule for a patch (darker than its
  ground, never under a third). IT NOTES NEARLY EVERYTHING TODAY, which is the state of the ground and also how blunt the measure is: on build
  5d31894, both stands, of 42 levels 38 are noted and 4 pass (office, office-relief, pass, tut-sky); of 14 look twins 9 are noted and 5 pass (block,
  block-squads, consulate, registry, warehouse). Most of it is the ground OUTSIDE the building: the yard levels stand on sheets of dust or asphalt of
  250 to 875 m², and the test maps are noted with the missions. `tut-solo`: 260 m² of dust and gravel (the road, where the mission starts), 101 of
  gravel (the gun court), 57 of slab (the shop), 56 of dirt (the truck court), 323 more outside the place. `NEEDS_BREAK=1` plants one
  sheet over the level's ground, which must be found, and asks the measure sixteen made floors: bare, patches of the ground's own paint, a speck of paint every 2.5 m,
  strips of two paints that do not read apart, a yard with one marking, two rooms with an open gap between them and the start in one, a bare ring at
  the edge with the start on it (each must be noted); a dark patch, a patch of another sheen, rubble, a turned 6 cm line, each every 2.5 m, strips of
  two paints that read apart, the two rooms with a door between them, the two rooms with the start outside their walls, the ring with no start (each
  must pass). Three reviews of the measure found 39 things; left as they are: a step of roughness alone counts, metal does not, a decal on a raised
  pad is not seen, the 3 m round a start is measured through walls, and it is a third reading of "what is drawn flat" beside `tools/levels/flatLayers.ts`
  and the look skin's `floorAt`. What a patch
  does to the picture has NEVER been seen on a GPU here, and no level's ground was changed by this: each noted floor is its owner's to mend or to answer.
- **The furniture check holds each level to its own base** (2026-10-22; `tools/qa/lived-check.ts`: a tool, no switch, no game code, no level file touched; A
  CHANGED BAR in a check). It held every level to ONE base, the first of `BASE_REVS` that HEAD descends from: a level changed on purpose moved every level's
  bar (six times in a day, each a tutorial mission's), and two levels changed side by side left two newest bases that are siblings, so their join was red on
  the level the taken one did not carry until a third base, the join itself, was written in by hand. NOW `LEVEL_BASE` is a row a level (its note, its line,
  an empty line; a new row goes in by the level's name, so two branches that each move a row join without a hand): a level changed on purpose in its own part
  moves ITS row to the commit that carries the change, and no other level's bar moves. Rows today: all six tutorial levels; the plain run walks every
  level with a row, furniture or none. A look twin with no row is held to the plain level's. `BASE_REVS` is CLOSED and is the base
  of every level with no row (it stays a list only so that branches cut before the rows came, which still put a commit in front of it, join without a hand).
  A row's commit must be in the checkout's own history and must carry the level's file, and a level in HEAD that has furniture, no row, and is not in
  `BASE_REVS`' commit is red (it was "a new level, nothing to hold it to", and with the list closed nothing would ever have taken it in); the same level
  not yet in a commit is a warn line that says what it will need once it is. `--base <rev>` still
  holds every level named to one commit. WHAT OF A LESSON NEEDS NO ROW MOVE (`LESSON_FREE`, read from today's file): its WORDS (`title`; a step's `line`,
  `setupLine`, `ringLine`, `blowLine`, `menuLine`, `pageLine`, `flatLine`, `planLine`, `noTap`, its card `explain`, its `after`): furniture cannot touch one, none
  names a place and none is a rule; no check holds the wording, on purpose (the loader takes their shape, `lesson-check` reads the first line on the
  objective row). HELD (`LESSON_HELD`): the steps' number and order, a step's `id`, its `done` test and `hold`, every place (`at`, `atMan`, `toward`,
  `eye`, the frame's `sky`), `cue`, `checkpoint`, `fog`, `onWay`, `respawn`, `lasers`, AND THE LISTS OF WHAT THE FRAME OFFERS (a step's `allow` / `deny`;
  the frame's `allow`, `orders`, `rows`, and `flatRing`, which ring the mission has): they are rules, and a word too many in an `allow` opens a key a step early with nothing else red (a scripted
  player only shows a key that is missing). They were free for one step (897) and are held again; a list changed on purpose is its level's one row move.
  A lesson's key in NEITHER table is a red line: whoever adds a key says which (with no file named this is asked of every level with a lesson,
  furniture or none). GONE: the rule that a lesson with another number of steps was read
  as today's whole (every mark and test of a re-cut lesson went unheld); a re-cut moves the level's own row. THE CHECK'S BREAK PATH (it had none), each on
  today's files in memory: `LIVED_BREAK=1` moves each level's first own box and stands its first lived solid in its first door: red on line 1 and on the
  doors' line of all 39 levels (166 lines); `LIVED_BREAK=lesson` changes every held key of each lesson's first marked step and of its frame: red on the 9
  levels with a lesson and a lived thing (it shows the held keys are held TOGETHER, not each); `LIVED_BREAK=lists` adds one word to every step's `allow` and
  to the frame's three lists and nothing else: red on all 13 levels with a lesson (26 lines); `LIVED_BREAK=words` changes every key of `LESSON_FREE` on
  every step: green, and red as soon as a key is struck from the table (tried: `menuLine`, `noTap`). CHECKED 10-23: 437 lines green on both stands (the
  tool reads files and floors; the stand does not reach it); the six tutorial levels by name green, the ledger's with 21 words and lists off its row's commit.
  NOT DONE: `BASE_REVS` made one commit (after the branches that still write to it have joined); a base per level for the levels that have no row.
- **The level skill names where light can be read through a wall (`leak`, the eighteenth need of `tools/qa/level-needs.ts`, 10-23).** The maintainer, on
  shots of the solo night's manager's office, dark, its shelves warm against the wall of the lit store: "some weird light leakage on bookshelf?", "seems
  like irradiance cache leak through wall? and showing up in specular?", then "we debugged something similar on pass -- i wonder if we should have a static
  check we can do on levels that catches this and put that in the levels skill". The same shelves and wall as 08-12 (`docs/agents/render.md`, the gather's
  bounce read; 10-20 on a man by a wall, `BOUNCE_WALLS`). THE MEASURE (`leakReads`, in plan, from the file alone): the cache's lattice (1 m, `render/rc.ts`),
  walls cut as `tools/qa/bouncewalls-check.ts` cuts them (shadow-casting, floor to ceiling, long and thin; its numbers typed again, a turned wall kept)
  and the line of each door the file does not hold open; lamps as the file burns them (a light not authored off; a flat fixture authored on that
  `game.ts` stands an area light in for, its brightest channel times its face) and the sky where no roof is. A probe is lit by a lamp in range with no
  wall between (a spot too: its bounce fills its room), or by the sky. For every box a ray can land on that is not the building's own, and every loose
  prop: a point every 0.5 m on each side face and the top, the read point one spacing along the normal, its four probes with their shares; a probe
  counts for a light when a wall parts it from the face (a hand's width short of the wall's ends), the light lights it, and the same light does not
  reach the face. A thing is NAMED at a share of 0.25, by the lamp before the sky. It is STRONG when that lamp's figure there (share times strength
  over distance squared, no unit) is 0.5 or more and more than the face has of its own from the lamps that reach it directly, a spot in its cone
  only: a lamp close behind the wall and the room's own lamp looking elsewhere, which is the manager's shelves. The need is a NOTE when a level has a
  strong one (the count, and the four room-and-lamp pairs with most of it), else a PASS that still says how many are named. WHAT IT FOUND, build
  66bed95, both stands the same: over the 56 level files 6,982 things named (10 on `tut-pair` to 296 on `block-gwot`), 1,726 of them strong (117 on
  `bazaar`); of 42 levels 39 are noted and 3 pass (`registry`, `test-rooms`, `train-pairs`); no look twin differs from its plain file. The solo office:
  144 named, 54 strong, 18 of those the manager's unit by `storage_bulb` through the wall at x = 30 (0.12 m thick); the tool lists the break room's
  things by the street lamp before it. So it is NOT A PASS MARK and not something a level dresses away: anything stood within a metre of a wall with
  light behind it is named, which is how rooms are furnished, and a room shipped dark beside a lit one is strong. It says where to look; the mend is
  the renderer's, and this number does not move when the shaders do (it reads no shader: `bouncewalls-check.ts` is the tool held to their text).
  `NEEDS_BREAK=1` plants a shelf, a wall and a bulb, and asks 23 made rooms: named (a bulb behind the wall, a shut door, a spot either way, a panel
  on, a wall's end, open sky behind, a bulb each side, a loose prop), not named (the room's own bulb, three metres off, an open gap, a door held open,
  no light, out of range, authored off, a bulb seen round the wall's end, a roof over both sides, the same sky both sides), strong or not (a dark
  room: strong; its own bulb nearer: not; a weak far bulb: not; its own spot looking away: strong). NOT HELD by the red path: a face shut in a wall.
  NOT IN THE MEASURE: heights (a plan: a wall under the ceiling is no wall), a lamp the game switches on later, gloss, how bright either room is. Two
  reviews found 23 things; left as they are: the wall rule and the footprint test are written a second time, and 0.5 is a first number. The cause of
  the 10-23 shots was with the renderer's owner when this was written, and the rule may narrow to it. Nothing here was seen on a GPU.
  NARROWED TO THE CAUSE the same morning (the renderer's owner, from the code and the level's numbers, with no GPU either): the read is the traced
  reflections' (`reflect.wgsl shadeReflHit`: the cache one spacing along the HIT's normal, trilinear, no wall test), so the face that leaks is one a
  mirror reflects within half a cell of a ceiling-high wall (a board's top, an upright's side, the floor at the wall's foot), and it shows on bare
  metal, whose reflection carries its whole colour, and at a few hundredths on any other finish. So `LeakRead` has `mirror` (the box or its paint is
  `metal`, and its roughness is under the widest `rt.maxRough` of the presets that trace, read from `ui/quality.ts`: 0.55, ultra) and the need
  (`leakLine`) is a NOTE only where a STRONG thing is a MIRROR; every other count is in the PASS line, with the mirrors named by the sky alone (never
  strong: the sky has no figure here) as "look by day". Both stands the same on build 66bed95: 238 strong mirrors over the 56 files (25 on
  `barracks`, the most); 27 plain files are noted and `killhouse` by its look twin only, so 28 of 42 levels, and 14 pass (`defuse`, `kit-yard`,
  `pass`, `quiet-yard`, `registry`, `test-rooms`, five `train-*`, `tut-pair`, `tut-plan`, `tut-solo`); 12 look twins read differently from their
  plain files (the look's `galv`, `tin`, `iron`). The solo office: 10, 6 of them the manager's unit by `storage_bulb`. The red path asks 35 things
  of made rooms now: a smooth steel shelf, one in a metal paint and one just under the cut are mirrors; no finish, rough metal, metal with no
  roughness, smooth and not metal, just over the cut, a box that overrides its metal paint are not; and of the line itself, that a strong shelf
  of no metal raises no note, a strong steel one does, and a steel one with its own bulb nearer does not (with the note put back on `strong` the
  red path goes green, tried). A third review found 8 things; mended: the cut and the matte default are read, not typed; the line is held; the
  skill names the preset to look on and a box marked metal itself. STILL NOT IN IT, and a NOTE-free level can show it: a thing of another finish
  that a mirror beside it reflects (the hit is shaded whatever its finish: the review counted 770 strong things within 2 m of a mirror), mirror
  metal the pass does not measure (the building's own groups, so every `glassFrame` in a wall and a door's frame; a drawn-only box; a steel door
  leaf, for `doors` carry no finish in this tool; a prop), and the weaker share of the same blend in the diffuse gather and on presets with no
  traced reflections, which the named and strong counts are the place to look for. A mend for the reflections was being handed in when this was
  written; this number will not move with it.
- **Round things are round** (`ROUND`, `src/scene/round.ts`, 10-23; the maintainer's pick on the geometry trial: round drums and tyres into the
  game). ON in the page, `?round=0` puts it off; OFF in the module and on the frozen stand, `TPS_ROUND=1`; a `FROZEN_HOLDS` row; in the knob
  registry and the journal. THE SWITCH IS READ ONCE, AS A LEVEL IS STOOD UP (`host/standup.ts populateHost`, before the boxes go to the world and
  before `new Game`): a recording from before the table has it put down before its level stands (`roundTape`, called by `replay/engineplay.ts
  tables()` and `tools/qa/rerun.ts tables()`; `applyKnobs` says the same afterwards; `tools/qa/replay.ts` says the file's word for its one
  stand-up), and the debugger's row 'round things are round (reloads)' reloads the page with the word on the address. The level files are not
  touched. The kit (`gwotKit.ts round`, radius 0.2 m or more: drums, tyres, stoves, the oven, the water tank) and three older generators (round
  table tops and drums in lived-in rooms) write a round thing as FOUR slabs through one centre, 45° apart, their tops stepped 2 or 3 mm;
  `findOctagons` finds them by that shape, and `roundLevel` makes the first slab ONE box with `Box.round` (the cylinder about the box's own y,
  radius `h[0]` = 1.026 × the slabs' half length, the circle with the octagon's area, which is 0.985 of the radius the kit was asked for; half
  height `h[1]`; yaw 0) and the other three specks that carry NoPrimary | NoShadow, so every box keeps its index. What each box was is kept on
  it (`Box.was`), `roundLevel(…, false)` puts every box back, and the level's fingerprint is taken over the boxes as authored (`authoredC /
  authoredH`).
  ONE MODEL: CPU rays and segments (`boxes.ts rayBoxRound` behind `rayBox`, `collision.ts lineBox`, and the two fast paths in `segmentBlocked`
  and the dynamic walk hand a round box to them), MOVEMENT (`collideCircle` pushes a body straight out from the axis, `circleTouches` asks circle
  against circle, so the walk grid is baked round too), GPU rays (`common.wgsl isectRound`, called from `isectBox` behind the scene prelude's
  `const ROUND_BOXES` on a box whose flags word carries `GEO_ROUND`; the side answers the code `ROUND_SIDE` and the closest-hit walks ask
  `roundNormalAt` — in a program compiled WITHOUT the cut rule; one compiled with it traces the picture's prism, THE CUT SET below), and the picture (the g-buffer's cut pass draws a twelve-sided prism standing round the cylinder, shaded with one turning
  normal, its marks run round it, a rolled rim on the lid: `gbuffer_cut.wgsl`, the ROUND block). Round wins over a cut record in every one of
  them; nothing gives a round box one. THE GPU HALF RIDES THE CUT TABLE AND THE CUT PIPELINE BUT NOT THE CUT RULE: `roundConfig.gpu` is set at
  boot (`main.ts`: the switch AND the cut switch); a round box has no `BOX_CUT_BIT`, its prism's eight planes sit in its own rows of the cut
  table with `ROUND_CORE_W` in the core row, it is listed at the cut list's picture-only end, and `BoxWorld.cutTraced` does not count it, so the
  cut swap (`render/cutSwap.ts`, "a calm room pays nothing") is as it was and every ray program holds a round rule from boot (the cylinder's in
  the calm set, the prism's in the cut set). `GEO_ROUND` is
  bit 3 of the GPU flags word, which is the CPU's `Dynamic`: no shader reads that bit, and it is masked out of the word only while the round's
  GPU half is up, so with it down the words are as they were. THE SWITCH IS THE SIMULATION'S: a picture setting may not move it. On a page with
  `?nocutboxes` (or if the cut pipeline fails to compile) a round thing is DRAWN, and traced by the GPU, as the square box round it, while the
  game still meets the cylinder.
  THE CUT SET TRACES A ROUND AS THE PICTURE'S PRISM (10-26; `common.wgsl ROUND_AS_CUT`, the prelude's const from `render/frame.ts roundRayWgsl`:
  true in a module compiled with the cut rule while the round's GPU half is up). WHY: on a box with Chrome for Testing 153 the software adapter
  lost its device on the first drawn frame with a cut piece alive, and a first-parent bisect of the build (the blown-wall still of
  `docs/agents/render.md` THE SOFTWARE ADAPTER'S ROOM, one run a step) put the first loss at the step that brought round things in; the same
  build with `?round=0` drew there, and Chromium 141 drew every step. The software adapter writes a copy of `isectBox` for every written box
  walk, the cut set's copies already hold `isectCut`, and `isectRound` on top of it in each was more than the newer adapter ran. THE RULE: in
  that set `isectBox` hands a box with `GEO_ROUND` to `isectCut` (`CUT_BITS` = the cut bit, and the round bit under `ROUND_AS_CUT`), which
  reads the eight rows `boxes.ts` already packs for the cut pass (`round.ts packRoundRows`: with the box's own four side faces they are the
  twelve-sided prism standing round the cylinder, in the box's frame, `isectCut`'s own form), and `isectRound` sits behind `ROUND_RAY`, false
  there: the cut set's box walks are the text they were before round things. A closest hit on any face of a round but an end still asks
  `roundNormalAt` (`roundSideHit`), so a drum in a mirror is not faceted. A round box still has NO cut bit: it counts for no cut program, and a
  calm room pays nothing as before. WHAT MOVES IN THE PICTURE: while the cut set is drawn (a cut piece a ray can meet is alive; a page with
  `?cutprograms=boot`) a shadow, a bounce, a reflection, the light meter's ray and a smoke sight line
  meet a round up to 3.53 % of its radius further out at the prism's twelve corners (1 cm on a drum of 0.29 m), which is where the picture's
  own surface is; the calm set still meets the cylinder, so a drum's shadow edge can step by that much on the frame the sets swap. No
  simulation code reads either set. SWITCH: `?roundexact=1` (`roundConfig.exact`, read at boot before the first pipeline) writes the
  cylinder's test into the cut set as well, as it was: a picture comparison, and more than the newer software adapter runs. It is a shader
  const like `?cutprograms`, so it has no stand's row and no knob (headless, no shader runs: both stands are the same by digest with or
  without it). CHECK: `round-check` line 9, both stands (`ROUND_BREAK=1` fails it: the prism packed at 0.9 of the radius): `isectBox` +
  `isectCut` put into TypeScript over f32 over 60 seeded round boxes (yawed and fully rotated), 4684 rays: every ray that meets the cylinder
  meets the prism no later, each hit is on a face of the prism or an end and no further out than the corners, the code says side or end as
  the hit is; and the texts (the walk, the three closest hits, the prelude's const by set, by switch and by the page word). `tools/qa/cut-cost.js`'s
  gate text reads `CUT_BITS` now. RUN (10-26, the software adapter, one run each): on the Chrome for Testing 153 box, the build head of that
  day with the change: the blown-wall still DREW with round things on, and the same page with `roundexact=1` LOST its device; the first
  tutorial's own page play, whose first frame after a door is shot out lost the device three times that day, went through all its steps
  clean, its two drums and a tyre round in a frame with a cut piece alive. On Chromium 141: a prototype page of another branch that compiles the cut rule into every program and
  had lost its device with round things since 10-25 DREW, 119 of 119 pipelines, and LOST again with `roundexact=1`; the blown-wall still DREW; the pass with
  `?cutprograms=boot` (the cut rule in every program, so every round is its prism to every ray) against the page as it opens, at two drums,
  480 × 300 on high: round in both, smooth sides in both, 77 and 35 pixels of 144 000 apart by more than 8 of 255, all at the foot of the
  drum where its shadow starts. NOT SEEN: on any real GPU, and no cost was timed (a round in the cut set is a walk of eight planes where it
  was one quadratic, only for a ray already inside the box round it).
  WHAT STILL MEETS THE BOX ROUND IT (a square of side twice the radius, along the world's axes): the trace grid's registration (a superset), a
  shoved prop's, a ragdoll's and a piece of debris's own contact (up to 0.41 of the radius of air at a corner), the smoke solver's obstacle
  bake, the replay's map, and a blast, which takes a round thing as that box (`furnblast.ts`: its footprint for grouping is the square, and what
  flies is cut from it). UNDER FIRE (`wallbreak.ts`): a round thing in a paint the wall engine chews is ONE breakable of ONE cell where the
  octagon was four of one each, so it goes whole on one cell's damage and never stands as pieces of the square (32 of the 343: 30 drums in a
  paint read as wood, and the compound's cardboard drum in its two files, which was four cells a slab). THAT MOVES A TALLY nobody has measured in a fight: such
  a drum is cover for about a quarter as long. Snow caps on a drum's lid and on tyres are still the level's squares. Round things under 0.2 m
  (the kit's two crossed squares) are left alone. CHECKPOINTS ACROSS THE SWITCH: the fingerprint is the same, so a save loads with the switch
  the other way as long as it names no part of a round thing as broken; one that does (a blast took a drum, rounds shot one out) is refused by
  `canLoad` with the static's name, in both directions — also a save from before this rule, loaded on the page as it opens.
  CHECK: `tools/qa/round-check.ts` (`probes.ts round`; `ROUND_BREAK=1` must fail) on BOTH stands: every level file's count against a pinned
  table (282 across 18 files as the files have them, 343 across 28 with lived-in rooms, at this step; 364 across 22 and 456 across 36 at step
  1239; a level that gains a round thing now gains a count file under `tools/qa/fixtures/round/`, not a row; a ring of four slabs the rule did
  not take is red), each made one round box and three specks and put back box for box; `rayBox`, `lineBox` and the
  shader's rule put into TypeScript over f32 against a point-in-cylinder reference (4800 seeded rays, the GPU rule within 0.2 mm and on the
  same kind of face); at the pass's brazier, from 67 sides, lines of sight just inside the radius all stopped and just outside all free (the
  octagon: 60 and 48), a man walked into it or put half way to its middle left against the curve (the octagon: up to 7 mm inside it and 16 mm
  clear), rounds from beside it and from 3 m stopped on the curve (the octagon: none, up to 19 mm off), Sam put inside it stood outside at one
  distance; tut-pair's seven chewable drums one cell each; a save loading with the switch as saved and across it; a lone
  round's walk grid a disc; ten seconds of the pass with the switch down the same run by digest on a level that was rounded and put back;
  fingerprint, stands, an old recording; the box world's words, rows and lists with the GPU half up and down. PAGE: `node
  tools/qa/round-page.cjs --root . --out <dir>` boots the pass twice on the software adapter (`round=0` and as it opens), wants every pipeline
  compiled and no console or GPU error, and writes a still of the brazier from each.
  EVERY LEVEL'S ROUND THINGS, WHERE THEY STAND (`round-check` line 4c; the routine is `tools/qa/round-lib.ts probeRound`, and line 4's brazier
  is handed to the same one): no recorded run reaches most round things (the depot's ten tyres became round and no play-through result
  moved), so every level that holds one is stood up with the switch and each round box, its middle, height and radius written out from what
  the level authored, is asked three things from 67 sides of the game's own collision world: a line of sight just inside its radius is
  stopped and one a hair outside is free; a round fired at it, from beside it and from 3 m, stops on the curve with the normal straight out;
  a man walked into it stands straight out from it against its curve. A thing in a level has neighbours, so all but the inside lines are
  asked only from the sides the routine's own reference finds OPEN (no other box near the line, the round's path or the man's way out: a
  distance from a segment to every other box's footprint, which asks the collision world nothing). In a PILE (round things whose middle is
  over this one: a stack of tyres) the man must stand against the pile and inside none of it. Each is asked twice: of the TypeScript's
  answers, and inside a tick with the Rust rows for lines and circles up, of the module's copy of the level. Frozen stand 364 round things in
  22 levels, shipped 456 in 36, all green both ways; RED: an answer that is not a round's, from either, and a count that is not the level's.
  `ROUND_BREAK=1` hands the routine the kit's octagons and it says of every one that it is not round (the line prints how many).
  A LEVEL THAT GAINS A ROUND THING IS HELD BY THIS LINE WITH NO NEW LINE (its count file is all it adds). Printed, not red: round things no
  man can reach (on a stand, in a truck's bed, in a row by a wall: 53 frozen, 57 shipped); on the shipped stand 67 that end under a walking
  man's feet (a tyre 0.2 m high: the underfoot rule), of which sight and rounds are asked; and a thing hemmed in on every side, which is asked
  the inside lines only and named (none today: nothing can meet its curve where it stands, and the box round it would answer the same). The
  depot's ten tyres are all met by lines and rounds and seven of them by a man (three lie under a stack). THE ROUTINE ITSELF (line 4d): 2,400
  seeded things among seeded neighbours (boxes turned and fully rotated, glass, hidden boxes, round things beside and over them): of 1,200
  round ones none is called not round, and of the same things left as the kit's octagon or as the square box round it, every one a round has a
  way in to is. HOW HARD: a round and a man hold the curve to a thousandth of a millimetre; lines alone hold the radius only between 1 % under
  and 1.5 % over. Collision only: nothing here is a picture, and nobody is walked by the game's own mover (line 4b does that for the brazier).
  NOT MEASURED, NOT SEEN: no tour, soak or mission tally was run with the switch up (the office, which those run on, has no round thing; the
  check's ten seconds of the pass meet none); nobody has played it; NEVER SEEN ON A GPU — the stills are the software adapter's at the lowest
  preset, so motion, a round's edge against bright ground, the marks' seams round a drum and what the round branch costs a frame are unknown.
- **The lived kit's round pieces are the kits' one round** (dress every level, 11-03; no switch: level data under `LIVED`). A round is written ONE way:
  `src/scene/gwotKit.ts roundSlabs(r)` holds the octagon's numbers (four slabs 45° apart from 0.2 m, tops stepped 2 mm; two squares under it) and the
  kit's own `round` reads them (no level built with that kit moved: `export.ts --check` on every level). `tools/levels/livedKit.ts round(x, z, y0, y1,
  r, paint, yaw, { lid, ribs, drawn })` and `livedPieces.ts at().round(lx, y0, lz, r, h, paint)` lay it over the office kit; `tyres` and `drumFixed`
  are built with it (each was one square box, which the round rule could not tell from a crate). From 0.2 m the octagon lies inside the square it
  replaced; a smaller one's second square reaches 1.32 r. A round under 8 mm thick throws (lay a flat). MOVED: the depot (10 tyres) and the long hall
  (2 standing drums), each with its look twin: lived things only, so the frozen stand's level is the level it was, and on the shipped stand a
  recording, checkpoint or kept replay made on those two parts from this step (their fingerprint counts the lived boxes). `round-check.ts KNOWN` has
  their rows: a level that gains an octagon gains a row there.
  LEFT: a LOOSE drum, bucket or stool is a prop, and a prop's part cannot say it is round (the rule looks in a level's boxes). Writing a prop's body as
  slabs was tried and dropped: a blast throws a prop part by part, so a drum went as four overlapping slabs against the blast's piece budget, and the
  two squares of a small one stand outside the footprint it collides with. The mend is a part that says it is round, in the simulation and the picture.
  The other two kits that wrote these numbers by hand (`lived-quiet.ts`, `lived-train.ts`) call `roundSlabs` now: see the library's bullet on one name,
  one piece.
  CHECKED (both stands): typecheck; `export.ts --check` every level (tut-queue stale and train-ground throwing before and after); `lived-check` every
  level; `level-needs --run` depot and hall; `probes.ts round` (its line 7 was red before this step). NOT SEEN ON A GPU; nobody has played it.
- **Six ground and yard pieces joined the 2010s kit** (dress every level, 11-03; no switch: the kit only, no level lays them in this step; `src/scene/gwotKit.ts`).
  The maintainer: "as we dress the levels, I'd love for us to have a library of objects, materials, etc. that we reuse across them". Each was first written
  by hand in one generator; a thing a second level could use is a kit piece. `kerbStones(x0, z0, x1, z1, paint, pitch)` painted stones a stride apart (or
  at any pitch), the edge of a government yard or a track; `plantBed(x, z, len, wid, yaw, rows)` a kerb of blocks round turned earth with rows of
  seedlings; `hearth(x, z, yaw, ground)` a cold fire, a ring of stones on its ash (the ash is the ground's own paint darker since the bullet below: the
  fourth word is the GROUND it was lit on, not a paint to lay); `seedTray(x, z, baseY, w, d, yaw)` a tray of seedlings on a bench;
  `seep(x0, z0, x1, z1, mat)` the stain where water runs out under a wall, in lengths that overlap and wander (the line is the stain's middle: lay it
  0.6 m out from the wall's face); `stain(x, z, w, d, ground, yaw, k)` the ground's OWN paint darker (wet concrete, a floor walked dark). Beds, stones
  and hearth are walked over (under 0.19 m); seep and stain are decals. `patch` is another row's stuff lying on the ground, `stain` the same ground
  darker: its paint is the row times k (0.55 to 0.99, the band in which the picture's classifier still reads that row) and it throws where that tone
  is ANOTHER row's colour (gravel × 0.7 is `oil`); a toned paint is for drawn-only things, a round knows a paint by its exact colour. The kit keeps
  its own copy of the classifier's two numbers (`TONE`): one table for both once they share a tree. `tyres(x, z, 1)`: one tyre lying alone is 0.27 m,
  as the older kit's is (at 0.20 it topped out in the band the walk grid cannot read and `lived-check` was red on it); no generator laid one, so no
  level moved. `tools/levels/livedKit.ts livedTail(props, grp)` → `{ room(id), done() }` is the block six generators copy at their lived tail
  (group `lived.<room>`, props named `lived.<room>.<kind><n>`); nothing calls it yet. NO LEVEL FILE MOVES (`export.ts --check` on every level;
  tut-queue stale before and after) and the showroom is not touched: the library's own hall shows new pieces. CHECKED: the src typecheck; each
  piece laid once by hand headless (the walked-over ones top out at 0.17 m, a lone tyre at 0.27, two at 0.40; `stain` throws on gravel × 0.7 and on
  a k outside the band; the tail's names run on across two `done()`s). NOT SEEN ON A GPU, and no picture of the pieces has been drawn at all yet:
  the two levels that asked for them (the snatch, the glasshouse) lay them in their own step.
- **What lies on the ground is the ground's own, and lies where one table says** (the library, 11-04; no switch: the kit, and the level files that
  follow from it; `src/scene/gwotKit.ts`, `src/scene/gwotPalette.ts`, `tools/levels/livedKit.ts`). Found on the quiet levels' night stills and mended
  in no level: three kit pieces read wrong from the play camera.
  · A SCATTER IS ONE PAINT. `rubble(x, z, r, n, mat)` made three stones in ten the mud wall's dark row whatever `mat` was: pale clods among dark gravel,
  mud among offcuts of poplar. Its darker stones are now the row's DARKER ROW where the palette holds one (`gwotPalette.ts GWOT_DARKER`: mudLight → mud →
  mudDark; a family joins by a row of that table, never by a case in a piece), standing for what `mat` is to a round (`gwotAs`), and a stone has the
  row's whole finish (bits of tin sheet are metal: the piece dropped that word). A scatter of the default mud is byte for byte what it was.
  · A HEARTH'S ASH IS ITS GROUND, DARKER: a `stain`, where it was a patch of `damp` whatever it lay on (a black square on moonlit dust). THE KIT KNOWS
  THE GROUND IT LAID: `groundAt(x, z)` answers the finish `ground()` laid there (the higher of two, then the later), and nothing where it laid none;
  a hearth asks it, and on ground the kit did not lay (a level's own slab, bare lot) the caller names the finish or the hearth THROWS. It does not guess.
  · ONE TABLE OF WHAT LIES ON WHAT: `lie = { finish: 4, stain: 12, ruts: 14, patch: 15 }` (tops, mm; handed out with `flatAt(top)`, a decal's centre).
  A stain is the ground in a darker tone, so what was driven over it and what lies on it show over it: its top came down from 15 mm, where it painted
  ruts out where they crossed. Ruts, patches and seeps read the same table and did not move. A stain meant to lie ON a patch says so
  (`G.stain(…, G.flatAt(G.lie.patch))`: the vault's soot and doorstep, lights-out's raked ash, which keep their bytes). Heights are from the lot's
  top: a finish laid thicker than 12 mm (the relay's slab) wants a stain's `y` said. A generator's OWN flat helpers (train-move's damp over its ruts,
  the tutorials' `worn`) still hold their own numbers: theirs to bring to the table.
  · A LIVED TAIL, DONE, CLOSES ITS LAST ROOM (`livedKit.ts livedTail().done()`): the generator's group goes back to the kits' first group, so a box laid
  after the tail is no lived thing by accident (it went with the switch off) and the exporter refuses it by name. No file moves from it.
  LEVEL FILES THAT MOVE, sixteen: 80 stains lower by 3 mm (bazaar, glasshouse, the hall, lights-out, the police post, the vault); 58 stones change
  paint (the crossing 15 and the relay 12 to gravel, the snatch 9, the glasshouse 4, the vault 4, lights-out 3, the depot, the kill house and the pass 2
  each and in the first two's look files, the look's yard 1) and the pass's other five bits of tin gain the row's metal; six hearths' ash (the crossing,
  the hall, the police post, the snatch, the queue lesson, the vault). A LEVEL'S OWN PART moves in four, each with its base file and its why
  (`tools/qa/fixtures/level-base/`): the glasshouse and the pass (two stones each of a scatter in the level's own part), the look's yard, the hall.
  `hearth`'s fourth word CHANGED MEANING (a paint to lay, now the ground it stands on): the crossing's said 'dirt' for its ash on bare dust and says
  'dust' now, the same shade.
  THE CHECK: `bun run tools/qa/ground-pieces-check.ts` (five lines, over every row of the palette; `GROUNDPIECES_BREAK=1` asks the kit for each thing as
  it was laid before and all five fail). Run it after touching a ground piece; `tools/qa/lived-check.ts <level>` ONE LEVEL AT A TIME says whether a
  level's own part moved (the run with no name passes over a level that has no lived things and no base row). MEASURED, before and after on one tree: the 42 play-throughs
  (21 missions, seeds 1 and 2) on the frozen stand, the shipped stand with lived rooms off and the shipped stand, 42 of 42 digests the same on each
  and every run ended; the quiet missions, which the 42 do not play (`quiet-pair`, seeds 1 to 12, three arms, both stands: the vault, lights-out, the
  bazaar, the police post, the crossing, the relay), 36 of 36 the same run on each; the queue lesson's digests, seeds 1 to 3, three stands, the same.
  So nothing a run does moved. That a round ever met a repainted stone in those runs is not shown: to a round the snatch's offcuts are now wood and
  the pass's tin metal where three in ten were concrete.
  NOT SEEN ON A GPU, and not at night: the crossing booted on the page under the software renderer at midday (118 of 118 programs, no page or console
  error) and two stills were drawn, its hearth (the ash a darker patch of the lot's dust round the ring of stones) and a scatter of gravel by its east
  wall (every stone gravel). The night stills that found the three faults have not been drawn again.
- **The 2010s kit lays a kerb** (dress every level, 11-03; no switch: the kit only, no level lays it in this step; `src/scene/gwotKit.ts`).
  `kerb(x0, z0, x1, z1, mat = 'block', gaps = [], len = 1.9)` is a run of precast stones 0.3 m wide and 0.14 m high laid end to end, a finger apart
  and none quite in line: the edge of a lot, a road or a loading apron. The run is cut into round(length / len) stones; `gaps` names the stones left
  out, counted from the first end (a broken one, a dropped kerb where wheels cross). Plain statics, walked over (under 0.19 m), lit and shadowed. A
  run or a `len` under 0.3 m throws. It is NOT `kerbStones` (painted stones a stride apart, the bullet above): a kerb is one continuous line of
  concrete. NO LEVEL FILE MOVES (`export.ts --check`). CHECKED: the src typecheck; laid headless along a lot's edge with two stones left out
  (`lived-check` and the walk grid take it as floor). NOT SEEN ON A GPU; the warehouse and the records office lay it in their own steps.
- **The level skill carries lighting guidelines, and the level command names two lights that face each other (`key`, the nineteenth need, 10-23).**
  The maintainer, of the plan tutorial's yard: "this tutorial level just looks really *flat*", then "seems like we should generally (even independent of
  this thread) have some lighting guidlines that we put in our level skill". The graphics work read the cause from the level file and a still drawn on
  the software renderer. Opened by a link with no hour the level is its file's own night: two of the kit's floodlights (`gwotKit floodLight`: 210
  over 20 m, a 70 degree cone) at the yard's two gates look at each other, and pale walls go to white. The mission's own row plays at midday, where
  the photocell has both floods out and the sun stands at 56 degrees, a flat angle for a camera that looks down. Walls, blast walls and ground are
  close in value either way. THE SKILL (section 5, "How the light should look", eight rules, each saying what it rests on; guidelines, not misses,
  and said so there): one key light a court from one side; the key comes in low (a shadow is the wall's height over the tangent: 2.5 heights at 22
  degrees, 1.7 at 30, 0.67 at 56); lamps make pools, never a wash; every outside frame keeps a dark part and nothing big goes white; warm against
  cool; ground, walls and things a value step apart; a light changed for the look changes the play; look from the play camera before shipping. The
  wording is the graphics work's draft, cut down; a dark third and 0.4 for a pale wall (by luminance) are judgement. THE NEED (`facingLights`): two
  spot lights that burn as authored, neither a quarter of the other's strength or less (by the colour's luminance), each heading within 60 degrees
  of the line to the other and the two headings more than 120 apart, each one's pool reaching the middle of the ground between (the pool: its range
  and its cone's far edge from its height), and nothing ceiling-high (the leak's cut, hoisted to `planWalls`) on the line between them: a NOTE with
  the pair, which says when every row of the level plays at an hour that puts both out (`hours.ts`, group `exterior`). On the build of that evening,
  both stands the same: `tut-plan` alone of 39 levels (`flood_gate` and `flood_road`, 28 m; out at its midday). The yard lamps of `barracks` and
  `warehouse` face each other with a building between; `quiet-relay`'s `flood_w` and `lamp_west` face each other and the lamp's pool stops 6.5 m
  out of 18. `NEEDS_BREAK=1` plants a facing pair and asks 18 things of made rooms; each of the rule's six tests was taken out in turn and the red
  path went green each time. TRIED AND PUT DOWN: a measure of the ground the two share (squares both light from opposite sides at like strength,
  and squares in one's wall shadow that the other lights). On `tut-plan` the huts stand between the floods: 119 m² in all, 22 shared and 97 of shadow filled;
  but `barracks` (91), `sneak-lab` (77), `office` (67) and `block` (63) come near it by the same sum, mostly round a building, which is two courts
  with a key each and not a flaw. So one line stands for the ground between: a stub of wall on it hides a pair, a wall beside it does not part one. NOT IN IT: lights that
  are not spots, the moon's or sun's height, a spot's falloff, how pale the walls are, the picture's exposure; those are the skill's by eye. A
  review found 9 things; mended: the hour, the pool, the colour, the red path's hold on each test, a wrong pointer and the pale wall's measure in the
  skill; said and left: the one-line wall test, the plan and lamp list built again per need. On that build the `leak` need notes 29 of 39 levels
  (four training levels have left the tree). Nothing here was seen on a GPU.
  CORRECTED the same evening: the cause above is wrong in one thing, and the skill and the need's words said it too. A mission row's own hour is
  written over the link (`game/rowset.ts rowSettings`; `?rowset=0` is the link as written), so the maintainer's shot of `tut-plan` was MIDDAY and both
  floodlights were out on the photocell: nobody has seen the two floods wash that yard, and its night has not been drawn. What the pictures carry
  (the graphics work's stills on the software renderer, the same frame): at midday, the sun 56 degrees up, a pale wash with or without materials; at
  dusk, 22 degrees, long shadows and walls that hold below white. So in the skill the rule on a low key and the rules on white walls and value steps
  are the seen ones, "one key light a court" is judgement, and a rule is added in front of them: judge a level's light at the hour its row plays at,
  and read which lights are lit. The `key` need's line now says that a link opens at the row's hour too. Its measure is as it was.

- **FACES THAT FLICKER** (`bun run tools/qa/zfight-check.ts [name …] [--all] [--json f] [--quiet]`; the maintainer, 10-25, of a patch by the pass's gate
  that flipped between two paints: "can we have a static check script that we run on each level that finds these issues systematically? like we did for
  light leaks"). Read off the level files alone: two boxes the depth buffer holds (no `NoPrimary`, no `Glass`), standing at ONE angle (a turned room's
  boxes are asked among themselves), a face each within half a millimetre of one plane with the same outward normal, over a shared patch wider than a
  finger each way. The depth test cannot choose between them. `seen` (a FAIL, exit 1) = the two are painted differently and part of the patch is open;
  `same` (a NOTE) = painted alike, so nothing shows until a look paints each box by itself; `covered` = a third drawn box lies on the whole patch (a
  point asked every 10 cm). Not asked: undersides, tilted boxes (`rot`, counted), the tops of two boxes at different angles, door leaves, loose props, a
  blast's pieces. Exit 2 = a level named is not there. `ZFIGHT_BREAK=1` plants a pair of each kind in a level made in memory (exit 1 = each was read
  right). First run, 53 files, 29,914 boxes: 289 seen in 29 levels (30 of them in turned rooms; 74 over 100 cm², 24 over a tenth of a square metre),
  6225 noted. The biggest come from the level kit's gate (`gwotKit.ts gate`: a post 0.34 m square whose side lies in the end face of the 0.24 m wall it
  closes, 2.4 m tall: pass, gwot-yard, police-post, tut-sky, tut-pair) and from two cloths hung in one thin plane (bazaar, tut-squad). The mend is the
  level's own numbers (sink one face a few millimetres, or end one box at the other); none was made here, and the check is in no gate and no
  level-needs row yet. Nobody has watched any of the 289 on a GPU but the pass's gate.
  THREE PIECES' OWN PATCHES ARE MENDED IN THE KIT (11-05; the first of two steps: this one moves nothing a body, a round or a sight line meets). Of 334
  seen on 59 files, 113 were four pieces of `gwotKit.ts`, found by building every generator with the library's door watched and tagging each box with the
  piece that laid it. Mended here: the HANDCART (34: each shaft's top lay in the bed's top and lapped it; the shafts now END at the bed), the VAN'S BUMPER
  AND STEP (they ran two centimetres into the body with their sides in its sides; each now BEGINS at the body's end; `carShapes.ts vanShapes` says the
  same two sizes) and the WASH LINE (20: cloths are sheets in one plane and some lines dealt two that lapped; two that would lap are cut back to 2 cm short
  of where they would meet, a cloth nothing reaches keeps its own numbers, there are never fewer cloths than asked, and a line too short for its cloths
  THROWS). The cloths are drawn only. The shafts, bumper and step are solid, and each gives up only the part of itself that lay INSIDE another solid of
  its own piece: no solid thing is anywhere it was not, and none has left a place (asked of all 50 such boxes, point by point). 20 level files
  re-exported; in each only boxes' `c` / `h` (and the van's two shapes) moved, 70 boxes, none added or removed, no light moved; 248 seen after. LEFT FOR THE SECOND STEP,
  because a solid box has to move: a van's bay sides round its tail lamps, and a gate's posts in the end of the wall they close (the patch the maintainer
  saw on the pass).
  A LOOSE THING'S OWN PATCH (11-05, with the first step: it moves nothing either). `livedPieces.armchair`'s arms ran three centimetres into its back with
  their outer faces in the back's sides. An arm now ENDS at the back's front; it gives up only what lay inside the back, so the chair fills the space it
  filled. Seven files re-exported (consulate, defuse, registry, their looks, the showroom): 13 chairs, a line each, nothing added or removed.
- **THE SMOKE LAB'S FLOOR** (11-01; `levels/smoke-lab.json` from `tools/levels/smoke-lab.ts`, the GWOT kit; opened by the dev server's `/smokelab`, in no
  mission list, nobody on it but Sam; the lab's panel is `docs/agents/render.md`, THE SMOKE LAB). The maintainer asked for "a separate smoke lab that's more
  suited to playing with it". Each place shows one thing smoke does and is there for nothing else: the YARD (open ground under the sky and a low wall of
  sandbags: the wind takes smoke, and smoke parts round a thing), the TUNNEL (a roofed passage 16 m long up the screen, shut at its far end, its mouth to the
  camera: smoke rolls down a passage and out), the KITCHEN (a door to the yard, a door to the pantry and a window hole with no glass: smoke leaks through a
  hole all the time and through a door when it opens), the PANTRY (the room beside it, shut by its two doors) and the SHED (four walls and one door: a shut
  room fills). TWO LIT FRONTS (11-01, the maintainer: "can we add something like the lit sign (or the vending machine from the office) to smoke bench to
  test it?"): the office's red and blue vending machines, its own boxes and markings at its own brightness, backs to the house's yard wall between the
  kitchen's window and the pantry's door, facing down the screen, switchable group `fronts`: a bright sharp thing to look at through smoke and a light
  that colours it. Never seen on a GPU there. Five doors, three roofs, a bulb a room, two in the tunnel, one floodlight and a moon. Level data only: no switch, no game code. Check:
  `bun run tools/qa/smokelab-check.ts` (the file is its generator's; by the renderer's own floor plan the shed and the pantry are shut rooms with their doors
  shut, the kitchen is joined to the yard by its window, the tunnel by its mouth, the two rooms are one with the door between them open; roofs; the page's
  address; no mission names it; `SMOKELAB_BREAK=1` leaves the shed's door open and fails). `level-needs smoke-lab` passes with two notes (no mission row; 211 m²
  of the yard is one finish, left bare on purpose: nothing there should stop the wind). Every bulb is lit. The address opens it at a dry dusk (`&hour=dusk&weather=clear`: the plain page's own night came up in rain, which is noise over
  smoke); an hour or a weather on the address wins. Drawn on the software adapter only (the whole floor at dusk with cans burning in the yard, the tunnel,
  the kitchen and the shed, no GPU errors): never seen on a GPU, and nobody has thrown smoke on it by hand.
- **The library: every object and every paint, once (`docs/library.md`, dress every level, 11-03; tools and level data, no switch).** The maintainer: "as we
  dress the levels, I'd love for us to have a library of objects, materials, etc. that we reuse across them". What exists is SIX KITS with their own pieces and
  paints: `src/scene/gwotKit.ts` (the look's), `src/scene/officeGen.ts officeKit`, `tools/levels/livedKit.ts`, `livedPieces.ts`, `lived-quiet.ts`,
  `lived-train.ts`. ONE DOOR: a kit returns its pieces through `src/scene/library.ts pieces(kit, { … })`, beside its tools (a raw box, a frame, a paint
  table), so the kit's own return says which of its functions are pieces. With nobody watching `pieces` returns the object it was given: no level file, digest
  or frame differs (every `export.ts --check` is as it was). With a watcher (`watchPieces`, tools only) each piece laid is told with its arguments.
  THE CATALOGUE: `bun run tools/levels/library.ts --write` builds every generator under `tools/levels` watched (a look file's `finish` too), lays each piece
  ALONE in a fresh kit with the arguments a showroom gives it, and writes `docs/library.md`: per piece its arguments and what it is for (the comment and the
  line that define it in the kit, under the kit's own `// ---- heading ----`), what it is to a man read off its boxes (solid and to what height, off the
  floor, glass, walked over, loose, drawn only; lit), its size, its cost in boxes (solid, drawn, loose parts, round things, markings, lamps) and its paints by
  row; per paint table every row with what the simulation reads it as; the names two kits share; the paint names two tables give to different colours; the
  colours a kit writes as numbers. That page holds nothing of a level but the two showrooms, so a dressing change to a generator never makes it stale.
  `--uses FILE` writes which level lays which piece (true of the generators that minute: no file of the repo); `--pictures DIR` draws each piece from its
  boxes (`tools/levels/libraryDraw.ts`: the boxes and their markings under one fixed light, NOT the game's picture; `tools/qa/png.ts encodePng` writes the
  file); `--json FILE` is all of it as data.
  THE RULES (`--check`, exit 1): every piece stands in a showroom, says what it is (a `/** comment */` over its definition), does not top out between 0.20 and
  0.25 m where that is the piece's top (lived-check's band and probes, asked of the piece), every kit that hands pieces out has its row in the script, and
  `docs/library.md` is what the kits give now. TWO SHOWROOMS: `levels/gwot-yard.json` (the look's kit in a lived compound, untouched: `lived-check` holds its
  kept part, so a piece the look's kit gains is stood in the hall, or in the yard as a lived thing) and `levels/library-hall.json`
  (`tools/levels/library-hall.ts`, new: a hut furnished with the office's kit, the pieces of the look's kit the yard does not stand (one drum, the van, a
  partition; and a stain, a seep, kerb stones, a kerb, a plant bed, a hearth and a seed tray, which no level laid and no check held before), and one bay a piece for
  the four lived kits, each on its own ground; those are lived things, so on the frozen stand the hall is the hut, the car, the van, the look's pieces and
  the bare grounds; its own ground is the palette's slab, so the look page draws it as what it is). A NEW PIECE: write it in a kit with a comment that says what it is, add it to the kit's `pieces({ … })`, stand it once
  in a showroom, export that showroom, `library.ts --write`. A piece is asked for by name; a thing another level could use is never a stack of boxes in one
  generator. Checks that keep a row a level file gained one for the hall: `round-check` (1 round thing, 5 with its lived things), `carlamps-check` (4 head,
  4 tail, 2 glowing: the office kit's car and the van), `glassreal-check` (the hut's two panes); `rollback-exempt.ts` lists the door's one module variable.
  The hall is a new file that carries lived things, so `lived-check` holds its own part to its base file (`tools/qa/fixtures/level-base/library-hall.txt`,
  the blob id of the file as its generator gives it: write it again in the commit that changes the hall's own part on purpose). `lived-train.ts` now asks its
  ground for the four things it uses (the two kits, `add`, `grp`), so a level off the shared yard can lay its furniture. WHAT IT FOUND and does not mend: 18
  names that two or more kits each build their own way (drum, table, stool, bucket, crate, cone, chair, bench, shelf, mat, papers …); 6 paint names two
  tables give to different colours (canvas three times); four pieces no level lays (`officeKit.paneZ`, `livedTrain.helmet`, `toolRoll`, `spools`);
  `levelKit.ts`'s copies of the office's pieces are plain functions that take the kit first, so they go through no door and the four levels built on them
  (defuse, floodlit, mousehole, records) show no office pieces. Never seen on a GPU: the catalogue's pictures are drawn by the script.
- **The library, one name one piece: what the catalogue counts, and the quiet pair's rounds (11-04; tools and docs, no level file moves).** A kit's piece
  that lays nothing of its own, only ONE piece of another kit, IS that piece: `library.ts` lays each piece alone with the door watched, and when the only
  thing laid from inside it is one other piece and the box, prop, lamp and fixture lists grew by exactly that piece's, the row says so (`is` in `--json`,
  *(is `officeKit.chair`)* on the page, and the table "One piece, handed out by more than one kit"). Such a piece is no second way of building its name, so
  "One name, more than one piece" lists a name only for pieces that are their own: 14 names now (it was 18; `chair` and `papers` in the lived kit are the
  office's taken into a room, and the look kit's `wallX` / `wallZ` are the host's wall in a palette paint). TO MAKE TWO KITS' PIECES ONE: have the second
  kit call the first kit's piece and add nothing, as `livedKit.chair` does; the catalogue then counts one. `tools/levels/lived-quiet.ts` had two tools
  that wrote a round by hand (`rnd` under 0.2 m, `oct` from it): they are one `round(x, z, y0, y1, r, m, solid)` on `gwotKit.ts roundSlabs`, the same boxes
  to the last digit (`export.ts --check` up to date on crossing and quiet-relay). THE LAST TWO BY HAND went the same way in the next
  step (lane L, five level files): `lived-train.ts rd` is written on `roundSlabs` (it was two squares at 0.9 r with a 3 mm step) and throws on a round 8 mm
  thick or less, as the lived kit's does (the ash in train-orders' fire ring is two `flat` squares); the crossing's cable reel drum is `L.round`. From
  0.2 m those are octagons the round rule finds, so with lived rooms stood in the counts go crossing 30 to 31, the hall 5 to 11, train-move 13 to 17,
  train-orders 2 to 3, train-silent 0 to 3 (each a file under `tools/qa/fixtures/round/`; the plain counts do not move). The three training levels are on
  no mission row. STILL BY HAND: a loose thing's parts (the quiet pen's pail and tyre): a prop's part cannot say it is round yet.
- **A box says its shape** (`SHAPES`, `src/scene/shapes.ts`, 11-04; asked from the library's showroom: "could we make it so the cars have rounded
  surfaces? look really blocky right now"). A box of a level file may carry `shape`: how it stands when shapes are on (`docs/level-format.md`: `round`
  about its own x or z, `arc`, `cut` planes, `old`, and `with` = pieces that are there only when shapes are on), ALL IN THE BOX'S OWN FRAME, so the
  editor's move, turn and copy carry it. A SHAPE IS CUT FROM THE PLAIN BOX: the box keeps its centre and half size; what is not inside them is a piece.
  THE DATA NAMES THE SHAPE; the cut record is built from it by `cut.ts cutRecord`. ON in the page, `?shapes=0` puts it off; OFF in the module and on
  the frozen stand, `TPS_SHAPES=1`; a `FROZEN_HOLDS` row; in the knob registry and the journal. READ ONCE, AS A LEVEL IS STOOD UP (`host/standup.ts
  populateHost`, after the round things, before a box goes to the world and before `new Game`): a shaped box is given its record (`Box.shaped`), its
  pieces follow the level's last box (`Box.piece`), and no box changes its number. THE SWITCH IS THE SIMULATION'S, as the round things' is: a shape's
  record is met by the game's rays whatever the picture's cut-pieces switch says (`boxes.ts cutMet`, read by `rayBox`, `collision.ts`, and the module's
  box copy `boxMirror.ts kindOf`); with cut pieces down only the picture and the GPU's rays fall back to the uncut box. ONE MODEL: every GPU and CPU
  ray and the game's own module meet the shaped solid. A PIECE IS TRIM: drawn and nothing else. It says no flags and stands `NoShadow` (cut away with
  its box where the box is), so no ray, eye or body meets it, and its own record follows the picture's cut-pieces switch; what must stop a round or a
  man is a box of the level, which may say a shape of its own. It knows its box (`Box.of`) and follows it out of the picture where a level box is
  hidden by its number (`shapes.ts withPieces`: a thing a round takes in `destruction.ts hide`, a kit view's hidden boxes in `host/kitView.ts`), and it
  is never an item of its own to `destruction.ts`. WHAT WALKS MEETS THE PLAIN
  LEVEL, BY CONSTRUCTION: a shaped box is pushed against as its uncut box, which is the plain one, and a piece is met by nobody; `thinLevel` and
  `barLevel`, which mark boxes by their size as a level is stood up, read the plain boxes (`plainLevel` before them, `shapeLevel` after). So to the
  simulation the shaped level is the plain level with some of its boxes cut. A BLAST: a box whose shape has a form (`boxes.ts shapeCut`) is one cell to
  the wall engine and never a wall laid in cells, a hard wall's adoption included (`wallbreak.ts index / adoptable`: its cells would be cut from the
  whole box; a wall that only hangs trim breaks as it did), a window's bar is left plain by `shapeLevel` (`winbars.ts` breaks it into lengths), and a piece is
  one of the things that hang on what breaks (the wall engine's `ghosts`, the furniture blast's riders: any authored NoShadow static) and goes with
  it. A shaped thin thing is still thin to a round (`wallpen.ts thinToll`). A LEVEL STOOD ON ITS SHAPES IS ANOTHER LEVEL: `savestate.ts levelFingerprint` takes in `shapes.ts shapeStamp` (the geometry
  of the shapes that STAND, pieces included for their numbers in the list, to the millimetre; never a paint, a finish or `old`), so a checkpoint or a recording made on them is refused by the plain
  level and the other way about, and by the level once a shape has changed, through the refusal every loader and tape player already has; with none
  standing the fingerprint is the one the level always had, so no frozen fixture moves. A recording carries the stand's word (`standEnv.ts`) and the
  table; one from before the table has the switch put down first (`shapesTape` in `replay/recipestand.ts` and `tools/qa/rerun.ts`, the stand's word in
  `tools/qa/replay.ts`; `applyKnobs` afterwards). THE LINT REFUSES a shape on a tilted box or a kit view's box, flags on a piece, and a form on a box
  with `markings` or the `Glass` flag (the mark painter and the pane engine read the whole box), until those readers read a shape, and a shape on a
  slab of a kit octagon (found on the file's boxes as `round.ts` finds them); the check is red on a level file with a shape that does not stand.
  `canonicalLevelData` re-keys a shape. A level built by a generator in the page is shaped as one
  read from a file is: the shape rides on the Box. A piece is painted and finished by the reader a box is; its keys are its own list (`levelData.ts
  PIECE_KEYS`), read by hand in `loadLevel`, `levelToData` and `canonShape`, so a key a box gains for its paint (a `stuff`) is added in those four
  places together. THE PICTURE in this build draws a round and an arc as the flat facets they are cut to and
  a cut face bare; `shapes.ts formBox` is the one place a shaped box is told more (one turning face, its own paint on an `old` cut) once the look's cut
  pass is in the build. WHO SAYS A SHAPE: the kits' vehicles (the bullet at the foot of this file).
  CHECK: `bun run tools/qa/shapes-check.ts` (`probes.ts shapes`; `SHAPES_BREAK=1` must fail), both stands, eleven lines: the lint (eighteen refusals),
  the trip to data and back and the canonical bytes; a yard whose pickup is given a shape of each kind stood up, put back and moved, and the number of its shapes; rayBox
  and lineBox against references written in the check, with cut pieces up and down; the same walk grid and the same pushes up as down, on that yard and
  on every level file that says a shape, and every said shape standing; five seconds with the switch down the same run by digest as the same level saying none; the fingerprint, the
  stands' rows and a tape from before the table; a checkpoint refused across the switch both ways and opening in its own game; lines and circles
  asked of the module inside a tick; a blast that takes a shaped crate and the plate hung on it; and (line 10) an arc beside its box's own planes is cut to fewer facets and is still
  the round, and no trim hangs in the air in any level file, lived things in. MEASURED: with no level saying a shape, the 42
  play-through runs (21 scenarios, seeds 1 and 2) have the build's digests on the frozen stand and on the shipped one. NOT DONE: nothing here has been
  seen on a GPU or played; on the plain page a level with shapes keeps the cut ray programs live from boot, and what that costs (and whether a shaped
  box shows plain for a moment before they compile) wants a number from a GPU before a level file says a shape; with `?nocutboxes` the page draws a
  shaped box plain while the game's rays meet its shape (the round things' split).
- **THE WHITE ROOF, A LOOK STUDY** (11-04; `levels/white-roof.json` from `tools/levels/white-roof.ts`; open it as
  `?look&solo&levelFile=white-roof.json&hour=midday`; no mission names it). The maintainer: "maybe as a prototype, could you try making a small level
  that recreates the ... look based on all the screenshots i've sent you?", of the texture work in particular. One sentence: a kept roof in full sun
  where every surface is one colour and shows what it is made of by its relief, its finish and its pieces. Hand placement: the hall's wall the whole
  north side (cast panels over an orange stucco plinth, the biggest face that looks at the camera); the plant, four cast volumes of four heights with
  a yellow line round every top, in the middle where the sun throws shadows off them; the yellow wall cast against boards behind it; a block of two
  rooms in painted block (a skimmed floor and stucco piers; a carpet and one orange wall); one solid orange stair head; a painted pad. Every box says
  its `stuff` (the ten kept stuffs, `docs/agents/render.md`), and walls stand on a half-metre grid so a paver's and a panel's joints land on walls'
  ends. Its doors are orange gloss outside and white inside (`LOOK_DOORS`). `level-needs white-roof`: no FAIL, two NOTEs (no mission row; the open deck
  is one paint over 201 m² as the tool counts it since 11-05, 231 when this was written). `level-lint` passes; `round-check` knows its ten round things. Nothing here is a mission: no
  scenarios but door names, the stealth mission's three places only because the format wants them. Drawn on the software adapter only; never seen on
  a GPU, never played.
  SECOND PASS, the same day, after he looked ("directionally nice but still doesn't have enough "personality / cm^2""): what his pictures carry on
  every patch beside the stuff. PAINT A PAINTER LEFT, as markings on the box that carries it (`scene/markings.ts`: a stencil face `STENCIL` with
  `GEN.text`, `GEN.stripes`, `GEN.frame`; recipes `stencilMarks`, `hazardMarks`, `outlineMarks`, `boltMarks`, `ratingPlateMarks` in
  `markingDefaults.ts`, any level's to call): the hall's bays and the way out, a number on every pier and on every plant volume's top, warning
  stripes on the piers' plinths, on the low volume's foot and on the deck before each outer door, a hatch's yellow outline, bolt heads on the steel
  band (now five butted plates) and on the kick plate, a maker's plate. No digit over 5 and no compass word. SMALL FITTINGS: conduit with clips and
  boxes, vents in the hall's wall, clamps and flanges on the pipes, a hose cabinet, door numbers on plates, a grated drain, an orange railing by
  the pad, and by the stair a pallet of spare pavers, two tins, a bucket and a plank. 267 boxes, 47 markings (632 of the table's 6144 rows),
  seventeen round things. A BAND OF COLOUR IS THE WALL'S OWN STUFF in another colour on a skin a finger proud (the hall's plinth in cast panel, a
  metre of orange on the room block's two outer faces in gloss block), so the joints run on under the colour's hard edge, as on every second wall of
  his pictures. And use shows on its stuffs (`worn`, `docs/agents/render.md`). Second pass drawn on the software adapter only; never seen on a GPU.
  HOW IT IS WALKED (11-04, `scenarios.walked`; `docs/agents/render.md`, "HOW A LEVEL IS WALKED SHOWS IN ITS SURFACES"): six ways by name, from the
  stair's landing to the lounge's door, the service door and the plant, from the service door to the plant, and through the two rooms. A way's
  ends are names, so the roof now NAMES THREE PLACES (`scenarios.places`: stair landing, plant pad, service room). Those are words the game may say
  of this level (`game/dialogue.ts placeName`) and middles a response sweep may walk to (`game/qrf.ts`); no mission is staged on the roof, so
  nothing says them today and nobody has heard them. The level's fingerprint does not move (words and a picture's list).
  ITS ROOMS AT NIGHT (11-05; level data, no switch; the model's side is `docs/agents/render.md`, ONE EXPOSURE AT EVERY HOUR, KNOWN, LEFT). Each
  of the two rooms had a point lamp of 60 over 9 m hung between its pair of ceiling tubes, on top of the tubes, which are lights of their own as
  in every level of the kit (3.72 each, `game/game.ts`). No other level has a lamp indoors over 28. The lamp alone put 7.9 on the floor under it,
  1.75 times what the night's white is made from (`EXPO_LAMP`: the kit's bulb, 26, at 2.4 m), before any bounce, and a night shows a lit room at
  the night's own exposure, which never comes down. SEEN ON A CLOUD NVIDIA CARD, night, doors shut, the man inside: the service room (white on
  every face) burnt white, its floor 245 of 255, a black suit 113, the meter at 99; the lounge (a blue carpet, one orange wall) a washed pale
  blue, the suit 161. The service room with the lamp set at run time to 30, 18 and 6: floor 235, 225 and about 205, the suit still grey. At 0,
  the tubes alone: floor 155 to 160, the suit black, a shadow at the man's feet, the meter at 59; and the lounge a deep blue carpet with the
  suit at 28. How much of the glare was the room's air is not known: the lamps were the only lights there with a share in the haze (0.1), a
  tube's light has none. THE TWO LAMPS ARE OUT OF THE GENERATOR. The level's fingerprint moves (a checkpoint or a recording of it from before
  is refused); its file has two lights where it had four; nothing in a check or a fixture named or counted them. WHAT ELSE MOVES, seen on the
  same card with the doors shut and the man inside: BY DAY AND AT DUSK THE SERVICE ROOM IS DRAWN BRIGHTER AND FLATTER, its floor 200 to 233
  at midday and 158 to 217 at dusk, the suit 14 to 34 and 8 to 32, the man's shadow fainter. WHY, READ FROM THE CODE AND NOT
  MEASURED: not the lamp's own light on the floor, which by day is drawn at the hour's exposure and was about a seventh of the room's at midday,
  but its light on the ceiling, a hand under which it hung. The place field holds what a face that looks up gets from the cache, so a lit ceiling
  raised the room's level and the model showed the room down by it; a tube faces down and lights the floor, which the field does not hold, and
  the floor reads over the place's level. If so it is the model's (the same KNOWN, LEFT) and true of every room lit by tubes alone. THE GAME'S OWN READINGS FALL: the CPU's light on a man (direct light only, `guards.ts
  lightEstimateAt`) reads 19.9 to 1.07 in the middle of each room and 2.7 to 0.17 and 2.9 to 0.21 in a corner, so a man is seen at 1.0 to 0.71
  and 0.9 to under 0.4, and a corner is now under the lit-place level (`LIGHTS.litAt`, 0.5) though the picture shows it lit; the page's own
  meter, which has the bounce, read 59 mid-room. Nobody is staged on the roof. `level-needs white-roof --run` on both stands: no FAIL, the two
  NOTEs. NOT SEEN: the lounge by day, dawn, a grey day, the plain page (where a fixture is at its night strength at every hour, so by day the
  rooms lose most of their lamp light there), Metal; never played. The renderer's records of the service room at night from before this
  (`docs/agents/render.md`, THE MIRROR AND THE HIGHLIGHTS) were drawn with the lamp in.
- **The kits' vehicles say their shapes** (`src/scene/carShapes.ts`, 2026-11-04; the maintainer from the library's showroom: "could we make it so the cars
  have rounded surfaces? look really blocky right now"; of the trial that drew them, "Yes, build these in" and "geometry looks pretty good"; of the pass's
  lorry before, "some curved surfaces on the truck would go a long way"). Every pickup, van, car, motorbike and lorry a kit builds (`gwotKit.ts pickup /
  van / lorry / motorbike`, `levelKit.ts car`, `officeGen.ts car`) is the stack of plain boxes it was, and each box of the stack SAYS A SHAPE (`shapes.ts`,
  the bullet "A box says its shape" above; the switch is that rule's `SHAPES`, `?shapes=0`, `TPS_SHAPES=1`, and there is no other): corners rounded in
  plan, a crowned bonnet that falls to the nose, glass that leans between posts under a bowed roof cut back to the glass's top edge, round wheels with
  hubs under arches, a lorry's tilt bowed over its hoops with its fittings. WHAT A RAY MEETS LESS OF: a body's corners (to 0.24 m at a car's nose), a bonnet's
  shoulders, the glass above the waist (it leans in by up to 0.42 m at a car's roof), a roof's overhang; A LORRY'S TILT BOWS 0.15 m AND NO MORE, so its
  eaves stay over a man's head (the pass's yard is laid out on what the lorry hides: `pass-check` is green on both stands with it). WHAT WALKS meets the
  plain boxes. EVERYTHING ADDED IS TRIM (hubs, arches, posts, mirrors, tanks, a step on its straps): drawn only.
  THREE THINGS THE RULE GAINED FOR THEM: a piece of trim that says NO PAINT wears its box's paint, finish and word (a post, an arch, a fender follows the
  body whoever repaints it; one that says a finish or a word and no paint is refused); a `round` takes a box within 15 hundredths of square across its axis; an `arc`'s
  facets give way to its box's own planes (`arcFacets`: six beside one plane, four beside three, two beside five). EVERY FORM IS `old`.
  ONE RULE FOR WHAT LIES OR STANDS ON A BOWED TOP (`carShapes.ts layOn / feetOn`; a kit's vehicle hands back its shapes, `tops` among them; the kit's
  `lay(x, z, yaw, top, …)` is how a level calls it): a flat slab of snow or dust keeps to the crown (flat to half of where the top has dropped 2 cm,
  gone there, and cut back with the top's own planes) and a sheet of the same paint follows the bow as trim of the top's box; feet stand under a rack's
  rail or a ladder's stile down to the bow. The pass's lorry, the snowed pickups, van and motorbike of `tut-sky / tut-team / tut-road / tut-quiet`, the
  compound's roof rack (its front cross bar reaches the roof sign, which stands out over the windscreen) and the records level's ladder go by it. A LEVEL THAT STANDS
  SOMETHING ON A VEHICLE'S ROOF CALLS THESE; `shapes-check` line 10 is red on trim that touches nothing.
  A VAN'S TAIL IS SQUARE (its tail lamps stand on its back corners; a corner taken off there cut the lamp through its middle), and its head lamp lies
  as proud of the corner as it stands of the body's face (6 cm). A LIT LAMP A LEVEL HANGS ON A KIT'S LENS takes the outline of the lens's lit face,
  carried straight out (`glowShape`, `pickupLens`): the light ends where the lens's face does (a van's head glow keeps three quarters of its width, a
  pickup's lit lens six tenths, a tail all of it), and is not cut again by the corner it stands in front of.
  TRIM IS DRAWN WHILE ITS BOX IS, AND TO EVERY ENGINE IT IS NOT THERE (`scene/boxes.ts trimDrawn`, asked where the world lists what it draws; the rule's
  bullet above said trim "follows its box out", each engine hiding it for itself: that is gone). Nothing hides, files or restores a piece: its own flags
  never change. And an engine that sweeps the level's things skips `piece`: the destruction lane's items, a blast's furniture and its mouth rule
  (`game/furnblast.ts`), what hangs on a wall (`game/wallbreak.ts` ghosts). TWO REVIEWS FOUND WHY: a drawn-only wheel arch joined a pickup's wheel to its
  bed in the furniture's index (one blast took the bed, the tailgate and the tail lamps with the wheel); lying 6 cm off a wall's face it was thrown by a
  charge on that wall; and hidden, it stood "in the mouth" of the hole, so the bed liner and the crates that rested on it went too. `shapes-check` line
  11 stands four levels on their shapes and plain with the lived things in, holds every engine's list clear of pieces and the furniture box for box to
  the plain level's, and sets a burst and (on the page's stand) a wall charge off each side of every box that says trim: the same boxes go, the same
  walls open. It is red with any one of the three skips taken out (run).
  THE LORRY IS A PIECE OF THE KIT (`gwotKit.lorry`, box for box what the pass's generator wrote) and so stands in a showroom: on the road of
  `gwot-yard` (its lamp count file says six head lamps; `docs/library.md` has its row). Its tanks hang from the bed in two bands.
  THE LOOK PAGE'S LORRY TRIAL STANDS DOWN on a level whose lorry says its own shapes (`scene/lorryTry.ts dressLorry`: the trial was their first drawing;
  gone over by hand as well the lorry would be shaped twice), and `stuff-check` line 12 holds it to making nothing there (from 11-05 that line asks the kit's wall-face and yard pieces, as the pass's generator lays them, for their words).
  THE PLAIN BOXES ARE UNCHANGED in every level file (compared box for box against the step before, 39 files), but for a pickup's bonnet, which gave up
  its two painted panel lines ON EVERY STAND (a box that says a form carries no markings; the run digest does not see a marking). Each of the 39 files
  has its base file (`tools/qa/fixtures/level-base/`). `levels/office-relief.json` holds vehicles and says no shape (it has no generator).
  KNOWN AND KEPT: the lorry's fittings are trim that stands up to 0.33 m proud of what stops a man or a round (the jerry cans' rack 0.3 m off the
  sideboard, the mirrors 0.33 m off the cab at head height, the spare 0.3 m behind the tailboard): a man pressed to the lorry stands in them. Making the
  big ones boxes of the level changes the plain level and is a step of its own.
  MEASURED 2026-11-05 on the merge's joined stack over step 1333 (the materials train in), 21 scenarios x seeds 1 and 2: the frozen stand 42 of 42 runs
  the same digest; the shipped stand with `TPS_SHAPES=0` 42 of 42; the shipped stand 32 of 42. THE TEN THAT MOVE, by the whole digest at the half
  second each first parts from shapes down (read on step 1321's tree; the stack parts at the same half seconds): compound-four 1 and 2 and snatch-four
  1 and 2, one man's weapon light is on a half second otherwise at 23 s and the runs agree again after (one to three half seconds differ in all);
  pass-four 1 and 2, the player's heading differs by 0.01 to 0.04 rad for two or three half seconds from 13.5 s and 18.5 s; depot-four 1 and 2, one man
  faces another way at 23 s and the runs stay apart (seed 1 taken at 46 s, was 75 s; seed 2 wiped at 127 s both ways); records-pair 1 and 2, a man by the
  lot's car is 13 cm elsewhere at 2.5 s and the runs stay apart (wiped at 98 s, was 100 s; taken at 26 s, was 27 s). No ending changes. WHY A FIGHT
  PARTS: sight lines the plain car stopped pass the shaped one, over its nose at waist height (it falls 16 cm over its last half metre) and over the
  bowed roof's shoulders at a standing eye (1.46 m where the slab's top was 1.52). NOT EASIER, NOT HARDER over paired seeds, shapes up against down
  (step 1321's tree): depot-four taken 53 and 52 of 100 (15 seeds flip to taken, 14 away); records-pair 228 and 226 of 300 (49 and 47). A LEVEL STOOD
  ON ITS SHAPES IS ANOTHER LEVEL: a checkpoint or a recording made on the shipped stand on any of the 39 files before this is refused there after it
  ("made on another level").
  THE PICTURE'S WORDS (the library's four lines in `shapes.ts formBox`): a round and an arc are shaded as one turning face and an `old` cut takes the
  box's paint (`Box.cutRound / cutArc / cutOld`, read by the g-buffer's cut pass alone). The same 42 digests on three stands with and without them.
  THE CUT RAY PROGRAMS are live all session in every level with a vehicle (a standing shape is a traced cut box). On Metal that costs nothing (the
  maintainer's M4 Max, 2026-11-04: the cut set against the boot set pinned on the tunnel, fog 2.753 / 2.753 ms, frame 1.01 x). On the cloud's NVIDIA card
  under Linux the set of programs costs the fog pass in proportion to its march (the tunnel, 51 cut boxes traced: 15.36 to 18.99 ms of compute passes;
  the depot, 14: 11.49 to 12.98; a frame with no march: nothing), and the boxes themselves add about 0.1 ms. NOT MEASURED: NVIDIA under Windows.
  NOT DONE: nothing here has been seen on a GPU in this form, the round shading among it, and that shading is not timed (a trial on the look page drew
  the same shapes); the car of `levelKit` / `officeGen`,
  painted in plain numbers, does not yet say carpaint, glass and rubber, and trim in numbers says nothing (a follow-up, with the hit picture's rule).
- **The office says what a kept place is made of** (level data, no new switch; 2026-11-05; "we'll want to dress up all the levels and check their budget";
  of the office, 10-10: "awesome and it has a lot of personality per room"). The office's paints are the kit's, and the kit's NAMES say what a paint is on any
  floor built with it (`scene/stuff.ts PAINT_STUFF`): a rendered wall, a poured slab, a coarse cloth, the stuffs of a house left to the weather. On the look page
  that drew the office's partitions peeling at the skirting, its carpet stained wall to wall and its lino cracked. `officeGen.ts buildOfficeProcedural` now says
  the office's OWN words over those names, once, at its head (`K`, through `saidAs`: the kit's paint, its numbers and finish, another thing here): partitions and
  the conference sill `skim`, the shell and the stair tower `cast panel`, the landing's floor `board concrete`, the three carpets `carpet`, the lobby's stone and
  the server room's raised floor `paver` (laid in tiles), the lino `enamel` (one waxed sheet: the look's plain gloss coat), the extinguisher, its box and the
  agent cylinder `enamel`, the van and both cars `carpaint`. 95 boxes of 783 say a new word; NO box, colour, finish or number moved (the file differs from the
  one before only by `stuff` words), so the level's fingerprint holds and every checkpoint and tape of the office still loads. NOT SAID, on purpose: the lot (no
  stuff the look has is tarmac; `gravel` drew it as cobbles and `slab` as cracked paving on the software adapter, so it stays plain) and the lawn (it waits). (BOTH ARE SAID since 11-05, at their paints' rows: "Lots and lawns say tarmac and grass", below.)
  NOT TOUCHED: the kit's own pieces (mullions, door frames, rugs, the things written with `drawnAs`), which every level built on the kit shares; lamps and
  hours; nothing was laid (the office passes `level-needs` `ground` and `lived` as built, and one added box would part every recording made on this floor).
  `levels/office-relief.json` is exported with it and comes back in step (on the step before it lacked the vehicles' 22 shapes). CHECK `stuff-check` line 4
  holds both halves: what each name is, and what the office's own surfaces say over it. Nothing in the simulation reads a word: the three play-through reads
  (frozen; shipped with lived rooms off; shipped) are the step before's, 42 of 42 each. SEEN on the software adapter only (stills of the lot, the cubicles, the
  break room, the server room and the whole floor, the step before against this one); never on a GPU: whether the pavers' joints and the panels' fixing holes
  read at 60 frames, and the lino's shine as enamel, nobody knows.
- **A level's starts and its doors are on a way, and what is laid by hand is drawn over the ways** (the worn paths, 2026-11-05; tools and count
  files only: no level file, no page and no run moves). The maintainer, 11-04: "wear, paths in the ground, different materials can all tell a story for
  how people walk through the level, its logic, leading the player for game play". The rule that works out where a level is walked is the look task's
  (`src/scene/ways.ts levelWays`; `docs/agents/render.md`, "HOW A LEVEL IS WALKED SHOWS IN ITS SURFACES"). This is what follows on the levels.
  THE CHECK: `bun run tools/qa/walked-check.ts [level …] [--write]` (`probes.ts walked`, family `missions`; `level-needs.ts --run` lists it). It asks
  each level AS THE PAGE LOADS IT, the ways' one reader: the lived rooms and the dressing in, said to the loader by word whatever the caller's stand
  (their furniture is on the walk grid), the office with its script; a look twin is its plain file. (1) No way is without a path, no name its own
  `scenarios.walked` asks for is missing, every count file is a level's and names what the level has, and a look twin walks as its plain file (grid, doors, ways). (2) From every START the level stands (anything
  in its scenarios that says where `sam` stands, read as `marksight-check` reads its men: no kind of mission is named; and its own spawn where it
  stands none or where the plain night is played) a team's way or the author's own LEAVES and runs two metres or more, on the levels the mission list
  draws. (3) Every DOOR is on a way: a traced way two metres or longer crosses its frame within the frame's width or ends at it (a team sent to a
  door, the author's row to one, the yard's ways from outer door to outer door). A way that only passes a door along its wall does not lead to it,
  and the doorstep every door has in the map is no way. WHAT THE RULE DOES NOT REACH IS OWED BY NAME in a file a level
  (`tools/qa/fixtures/walked/<level>.txt`): red when a level owes one its file does not name; one that has a way now is said and `--write` drops it
  (until then the line would excuse it again); a `#` line is a note and is kept.
  AS FIRST RUN: 45 levels, 856 ways, none lost; 50 starts on 29 listed levels, ONE owed (the office's plain night: the rule walks from a spawn only
  where no scenario stands a team); 393 doors, 84 OWED in 30 files (inner doors nobody's shortest way uses, the block 10 of 16; and the lone outer door of three showrooms).
  `WALKED_BREAK=1` asks the ways with no routes and no scenarios: red, every start and 209 doors, 40 levels (the five it cannot bite are named). THE LIST IS THE WORK LEFT: a
  `scenarios.walked` row to a door gives it a way (`docs/level-format.md`), or the rule learns to walk to every door (the look task's function).
  THE OVERLAY: `bun run tools/levels/walked-overlay.ts <level …> [--out dir] [--px 24]` (the same level, ways and starts as the check's; a look twin
  asked by name is drawn with its own flat things over its plain file's ways) writes a picture of the level from above (the walk grid, every flat
  thing lying on the ground in its group's colour, the ways as lines by kind, doors, starts) and a table a group: boxes, square metres, how much of it
  lies on a lane, and how much of the lanes lies under it, both by area. No list of what a hand-laid path is: a level's groups say (the pass:
  `snow.paths` 53 % on a lane and under 13 % of the lanes, `snow.cleared` 45 %, `snow.ruts` 8 %; the block's look: 213 flat things, 26 %). It is for
  deciding, band by band, what the lanes can replace. NO HAND-LAID PATH IS RETIRED BY THIS STEP: the lanes are the look page's only, and the first
  pair on a real card read the derived lane too weak beside the bands (the dress goal's rule: retire no band until its lane alone reads as clearly).
- **The pass's dressing is the kit's: what is done to a wall's face, round things that lie, and a window's two forms** (the library, 2026-11-05; no
  switch: kit pieces and the level files that lay them, on every stand; the shapes among them are the shape rule's, `SHAPES`). The maintainer, of
  dressing the levels: "I'd love for us to have a library of objects, materials, etc. that we reuse across them". The pass's house front and its yard's
  round things were two hand-built trials on the look page (`docs/agents/render.md`, the two "WAS A TRIAL" bullets). They are fourteen pieces of
  `src/scene/gwotKit.ts` now, laid by `tools/levels/pass.ts`, each stood in the showroom `library-hall`, and the trials' files, address words and
  buttons are gone.
  WHAT IS DONE TO A WALL'S FACE is written once for all four ways a wall can face (`onFace`: a frame whose X runs along the wall and whose Z points out
  of the face; `onStretch` for what is dealt along a stretch a to b, its origin at the end the frame's X runs in from): `boards`, `shutters`,
  `windowBlanket` (a window's dressing), `wallPatch` (a mend with a rough outline, over a dado or a skirt), `spout` (and the streak its water left),
  `beamEnds`, `splitWood`, `ladder`. What stands by it: `plankBench`, `pot`, `bricks`. ROUND THINGS THAT LIE are one box that says `round: 1` (`lying`):
  `pipes`, `log`; `choppingBlock` is the kit's standing round. Snow or dust along a lying round is laid by the one rule for a bowed top
  (`scene/carShapes.ts layOn`, through the kit's `lay`: a lying round is a bowed top). TRIM IN A PAINT OF ITS OWN says what that paint is made of
  (`trimIn`: a log's end is wood, a pipe's bore metal), as a box does.
  A WINDOW'S DRESSING HAS TWO FORMS, `'shut'` and `'open'`, and the level picks. The dress goal's rule, 11-05: "a window a mark watches through, or a
  covered lane crosses, stays see-through" (the maintainer: "we don't want them to feel dumb"). Shut, the boards, a shut leaf and the blanket are real:
  a look, a round and the room's lamp stop at them. Open, the boards hang beside the jambs, the leaves are folded back on the wall and the blanket is
  rolled over the lintel: nothing on the ground, nothing in the opening. THE PASS LAYS ALL THREE OPEN: every window of its front is in some defender's
  view (measured with them shut: the tea room's two men at the glass kept 9 and 15 of the 18 lines they have to the yard, and the kitchen's man on its
  yard door 12 of 16).
  THE CHECK: `bun run tools/qa/marksight-check.ts [level …] [--write]` (`probes.ts marksight`, family `missions`, on the page's stand;
  `tools/levels/export.ts` asks it of a level the day it is written, `MARKSIGHT=0` skips; `level-needs.ts --run` lists it). It asks EVERY MAN A LEVEL
  FILE STANDS WITH SOMETHING TO WATCH, whatever says a `pos` and an `aimAt` outside the boxes and props (a room clear's marks, a rescue's captor, door
  men, guards and roamers, a plan's hostiles: 536 men in 41 level files; 18 files stand none and are named NOT JUDGED), with every door's leaf taken
  out of the world (a man whose gun rests on a door looks at the doorway): line 1, his eye's line to his aim is open; line 2, of a fan of 21 lines to a man's shin, belt and head across 3 m of the ground he watches, no fewer are open than his level's count
  file says (`tools/qa/fixtures/marksight/<level>.txt`, a count for each stand). THE COUNT FILES WERE WRITTEN ON THE UNDRESSED HEAD, on both stands,
  and the dressed levels are held to them. They are PER-LEVEL FIXTURES: whoever moves a level so that a man sees less carries that level's file with the
  move and says why on a `#` line in it (`--write` keeps such lines and says aloud every count it lowers). `MARKSIGHT_BREAK=1` stands a post in the
  middle of each level's first line: red on both lines on 41 of 41 levels, both stands (a level it does not bite is named, and the run then exits 0:
  a break word that passes). AS FIRST RUN, on the head before any level was dressed for it: every one of the 536 lines open, on both stands.
  A SECOND LEVEL LAYS THEM: the block (`tools/levels/block.ts`, so `block`, `block-gwot`, `block-squads` and `block-squads-gwot`: one floor) has its
  west flat's long window (x 4, z 16.5 to 19.5) with its shutters folded back and split wood stacked under the sill. No man watches through that pane,
  no man's line crosses that wall, and the nearest stands 13.7 m from it (8.2 m in the three-squad file, the workshop's third man, his gun on the
  hall door).
  A SHUT WINDOW COSTS THE TEAM TOO, MEASURED. That window was first laid with one leaf SHUT, since no defender needs it. On the page's stand the
  scripted team then took the block 3 times in 24 seeds with 8 of 96 teammates up, against 7 in 24 and 13 up on the head before, and 7 in 24 and 12
  up with the same shutters folded back (the same boxes but for the leaves' places; the frozen stand never met the shutter: 8 of 8 runs the same to
  the digest). Why is not traced: a look, a shot and the window's daylight all stop at a shut leaf. So the rule for a shut form is two rules: no man
  the file stands watches through it, AND its mission's tally is read with it shut before it is laid. Today no level file lays a shut form; the
  showroom stands each.
  A PLAY MOVE, DECLARED. New cover stands in the pass's yard (three fuel drums and one on its side in the pocket by the lean-to, pipes in the far
  corner south of the bunk room, split wood under the first window's sill, a bench, a ladder; the chopping block is round) and on the block's west
  wall (split wood under a sill), so the missions on those floors may play otherwise.
  THE PIPES ARE OUT OF THE PASS AGAIN (11-05, the same day): the 'far corner' is the pocket, 6 m by 2.5 m between the bunk room's south wall and the yard
  wall, that the squad bunches in to breach that wall, and the pile lay two metres straight behind the man who sets the charge. `menclear-check` (the men
  with Sam step back from a set wall charge: the maintainer's own case, on this wall) read red on its lines 3 and 3s from that step on: one man of the four
  was dealt no mark he could reach, straight back was the pile, and he stood IN the blast. The check was not in the step's list. Without the pipes the
  lines read as they did before the dressing (clear after 2.5 s, three marks west along the wall). WHAT IT TEACHES A LEVEL'S AUTHOR: floor within a
  charge's stand-off of a wall the squad breaches is the squad's, though no stack mark and no place for a charge is on it; run `menclear-check` and the
  wall charge's family on both stands after standing anything in a yard. `pipes` stays a piece of the kit and stands in the showroom.
  MEASURED 2026-11-05, the joined head `dd81d0de` against this step, by the scripted play-through's digest (`PT_DIGEST=1 tools/qa/playthrough.ts`):
  21 missions x seeds 1 and 2, the frozen stand 40 of 42 runs the same (the two that move are the pass's); the shipped stand 36 of 42 (the six are
  the pass's, the block's and the three-squad block's); the shipped stand with lived rooms off 37 of 42 (the same six less one, and the block's four
  end at the tenth of a second they ended at). No other mission moves on any stand. ON MORE SEEDS, before to after: pass-four, seeds 1 to 8: frozen
  6 to 5 of 8 taken and 18 to 15 of 32 teammates up; shipped 8 to 8 taken and 25 to 27 up. The block on the frozen stand is the same run to the
  digest (8 of 8, both files). On the shipped stand: block-four, seeds 1 to 24, 7 to 7 of 24 taken and 13 to 12 of 96 up; block-squads, seeds 1 to 8,
  1 to 0 of 8 taken and 1 to 0 of 32 up. The scripted player is not a person: these say the floors were not broken, not that they play well.
  NO DOOR'S STACK MOVED: every door of the pass and the block, both faces, a stack of one to four, deals the marks it dealt before, to the
  centimetre, on both stands (`tools/qa/lived-stack-check.ts --ask pass-four,block-four,block-squads` on the head and on the cut, compared). The trial
  had the pipes along the bunk room's wall, on the three places a stack takes south of its yard door, and the wood under the second window, where the
  third and fourth men of a stack on the tea room's door stand: both moved for that.
  WALL-CHARGE PLACES on the pass: 598 and 384 (frozen, shipped) are 588 and 374, on purpose (`tools/qa/fixtures/wallcharge/pass.txt` says it): five
  spots on the lean-to's south wall, both faces, behind the drums. The lean-to is no room of the fight, and every room of the fight keeps every place
  it had: the bench was moved to the wall's west end for that. The block's places are as they were. `second-way-check` (the five files) and
  `wayround-check pass-four`: green on both stands.
  COUNTS CARRIED: `fixtures/round/pass.txt` (13 round things as the file has it, 24 with the lived rooms in; were 8 and 19) and `library-hall.txt` (2
  and 12), `fixtures/geotry/` for the same two, the level-base files of the six level files. `stuff-check` line 12 asks the kit's wall-face and yard
  pieces for their words (it asked the trials). `look-check` learnt that a shape's trim wears paint as a box does (two shapes that differ only in what
  their pieces wear differ in paint), and finds `block-squads.json`'s generator. `tools/qa/round-lib.ts` holds a tilted box to the rectangle round
  both things the world makes of it, not to a square round its longest reach (a ladder leaned on a wall 3 m off crowded the pass's brazier).
  THE LIBRARY'S RULE FOR A WALL'S FACE (`tools/levels/library.ts --check`, `skewOf`): a piece whose words are a wall's axis, its line and a side is
  laid alone on all four facings and must lie the same on each. Red on the kit as first written: split wood and beam ends were dealt from the
  stretch's low end in a frame that ran the other way on two facings, and lay outside it there.
  KNOWN. A TILTED BOX IS TWO THINGS: a line and a round meet it as it is turned, a walking body meets the same box only yawed
  (`collision.ts collideCircle` reads no `rot`). So a body meets the ladder's stiles as two posts 0.4 m out from the wall, not as leaning rails.
  `woodPile` is still a stack of plain boxes (`drumDown` was until the drums' step below, where it says its round). A wall charge leaves what
  hangs above a hand's height on a face (`scene/lookTidy.ts` takes a decal with its wall only from the foot). Split wood's billets step through
  small tables, not a die, so that no two that touch share a face's plane with shapes off.
  SEEN on the software adapter only (Chromium 141, midday, 960 x 600): the pass on the look page (five views) and as the page opens (two), the block's
  window on the look page; six boots, every program compiled (119 of 119; the block's 118 of 118), no page, console or GPU error. The front's pieces
  stand where the trial's stood but for the pipes, the wood and the bench; the windows are open; lying rounds, billets and beam ends draw round;
  `drumDown` drew as the two boxes it was then (it is a round since the drums' step below). NEVER SEEN ON A GPU, and nobody has played either level since.
- **Walls and frames that said cloth by a borrowed paint say what they are** (level data and the look's skin, no new switch; 2026-11-05; found by the
  step that draws a hit by what the thing says it is made of: a round knocked tufts out of them). TWO CASES, ONE CAUSE: a paint taken for its COLOUR brings
  its row's stuff with it. THE DEPOT's hut walls wear the office's tan partition paint, whose row says cloth: `tools/levels/depot.ts` says `stucco` over it
  (`saidAs`, the compound's own line), 32 boxes of `walls.huts`, a word each and nothing else. THE CONSULATE's look file framed its doors and windows in a
  blanket's brown for want of a dark wood: 55 door frame boxes and 10 window jambs were cloth. A SKIN MAY NOW SAY WHAT A PAINT IS ON A THING
  (`tools/levels/gwotSkin.ts` `Said`: a palette row to the eye and to a round, another stuff and its finish), and `VARNISH` is dark varnished wood, named
  for what it is: the consulate's `frameOut` and `frameIn` are it, 65 boxes change `stuff` (wood) and `rough` (0.95 to 0.6) and no colour moves. NOT A NEW
  PALETTE ROW, on purpose: the palette is written whole into every level file built on the kit (a row makes each of them stale), and a row in the
  blanket's numbers would be the blanket to a round, which knows a paint by its numbers. Both level files differ from the step before only by those
  words and that finish; `depot-gwot.json` and `consulate.json` do not move; each changed file's base rides (`fixtures/level-base/depot.txt`,
  `consulate-gwot.txt`). MEANT AND LEFT: walls of sacks are cloth (the kill house's and the road lesson's sandbags, 44 boxes). NOT IN THIS STEP, said to
  the dress goal: the painted wall bands a skin lays as decals in a cloth's paint (the consulate's madder wainscot, the registry's brown dado: no ray
  meets a decal, so it is the picture's alone), and a gear screen's backdrop walls in the tan (the compound's set). CHECK `stuff-check` line 15: in every
  level file no box of a `walls.` or `doorframe.` group is cloth, fabric or carpet but for sacks in their own paint; it finds a made-up hut wall and frame;
  `STUFF_BREAK=1` fails it; on the step before it is red on exactly these 97 boxes.
  `hitstuff-check`, `wallfurn-check` (the depot 880 and 752, the consulate 857 and 725, held), `second-way-check`, `lived-check`, `wayround-check` and
  `level-needs --run` for both levels: green on both stands. PLAY: nothing in the simulation reads a word. The three play-through reads (frozen; shipped with lived rooms off; shipped), seeds 1 and 2,
  against the step before on the same box: 42 of 42 each on the step this was first written on, and again 4 of 4 each for the depot's and the consulate's
  missions on the step it is cut on. SEEN: the look page boots on the consulate's rescue with this in, on the software adapter, with no page or GPU error; in its
  night still no frame is legible, so the varnish itself is seen by nobody, and never on a GPU. The depot shows nothing on the look page (with the look on it
  opens its look file, which does not move): its change is the picture of a hit with the look off.
- **A drum is steel whatever it is painted, and the one on its side says its round** (the kit's stores, 2026-11-05; kit pieces and level data on every
  stand, NO SWITCH: 27 level files re-exported from 23 generators, and what a round, a blast and a wall charge do at 47 drums MOVES). The dress goal
  ruled both: "the kit's drum (standing and lying) SAYS what it is made of itself, whatever its paint", and "the kit's lying drum says round".
  STEEL: `gwotKit.ts steelIn` (`drum`, `drums`, `drumDown`). A drum in any row of the palette is metal to everything that reads a paint (the
  simulation's table `gwotSimOf`, a hit by colour `particleFx.ts surfOf` and by word `saidSurf`, a blast `furnblast.ts breakClassOf`) and wears
  the row to the eye: a row that is something else to a round stands in a hair off (`gwotPalette.ts gwotAs`, the kit's own `as`). WHAT IT SAYS
  to the picture: the row's own word where the row is a coat or metal already (`paint` for the drum enamels and olive: a coat, which says nothing
  of what is under it; rust, galvanised); in ANY OTHER row `paint` with a drum enamel's finish, never the row's stuff (a drum in the colour of mud
  is not made of mud). ONE WAY TO SAY IT: `scene/stuff.ts saidAs(stuff, paint, finish?)` takes a finish now, as `madeOf` does, so a word and its
  finish are said together and nobody writes a roughness by hand beside the word (no caller before this step changes; the skin's `Said` moves onto
  it in its own step). The cars' coat rule is not used: a drum's coat is plain `paint`. UNTIL THIS STEP OLIVE WAS AN AMMUNITION CRATE'S DARK WOOD
  TO A ROUND, so an olive drum was wood the wall engine chews and a level that wanted a steel one picked another colour (the pass's was
  `genYellow`; it is olive again). No level lays a drum in a row that is neither a coat nor metal today, so that half moves no file.
  WHAT MOVED, NAMED: 47 standing olive drums in 20 levels (twins aside) were breakable and are steel: crossing 2, floodlit 1, gwot-yard 2, hall 1,
  lights-out 1, pass 2, police-post 1, quiet-relay 1, smoke-lab 3, tut-kit 6, tut-pair 7, tut-queue 3, tut-quiet 3, tut-road 2, tut-sky 2, tut-solo 4,
  tut-squad 1, tut-team 2, vault 1, warehouse 2 (floodlit's, hall's, the pass's, tut-sky's, tut-team's and the warehouse's are lived things: the
  frozen stand does not stand them). Statics the wall engine can chew, before to after, as the page stands each (one a round): crossing 16 to 14,
  floodlit 10 to 9, gwot-yard 4 to 2, hall 20 to 19, lights-out 1 to 0, pass 8 to 6, police-post 31 to 30, quiet-relay 11 to 10, smoke-lab 5 to 2,
  tut-kit 8 to 2, tut-pair 7 to 0, tut-queue 3 to 0, tut-quiet 8 to 5, tut-road 3 to 1, tut-sky 4 to 2, tut-solo 4 to 0, tut-squad 1 to 0,
  tut-team 4 to 2, vault 1 to 0, warehouse 67 to 65. On the frozen stand (rounds off there: four slabs a drum): crossing 13 to 5, gwot-yard 10 to 2,
  lights-out 4 to 0, police-post 31 to 27, quiet-relay 12 to 8, smoke-lab 14 to 2, tut-kit 26 to 2, tut-pair 28 to 0, tut-queue 12 to 0,
  tut-quiet 17 to 5, tut-road 9 to 1, tut-solo 16 to 0, tut-squad 4 to 0, vault 4 to 0. No round thing of these 27 files is breakable now.
  LYING: `drumDown` is ONE box that says `round: 1` (0.88 long, 0.58 across, on the ground), as `pipes` and `log` are, where it was two plain boxes
  with a marked top: its two ends and two ribs are trim (drawn only, the paint a shade down by `trimTone`: a trim is met by no round, so its tone
  may be any colour and the piece never sends its caller back for another; it says what the drum says), bare where the paint is bare, on two chocks
  that reach in under the flank. It hands back its top, and a level lays snow or dust along it with `lay` (the pass does; its flat cap is gone).
  14 lying drums in 13 generators (floodlit, glasshouse, gwot-yard, mousehole, pass, quiet-relay 2, train-move, tut-kit, tut-pair, tut-queue,
  tut-road, tut-solo, tut-squad). To a walker it is the box as yawed, a centimetre shorter than the two were; with shapes on a line and a round
  meet the cylinder (the box's corners are open above the round's shoulder), with shapes off the box.
  CHECK: `bun run tools/qa/kit-stores-check.ts` (every row of the palette standing and lying is metal to the four readers, known as its row's paint,
  and says the row's word or `paint` with the enamel's finish; `drums` deals steel, a row that is not metal by itself among them; the lying drum is one round box with its trim, wedged chocks and a top
  that `lay` follows, in an enamel, a bare row and a row that is no coat; `KITSTORES_BREAK=1` asks a crate and the wood pile: 5 of 5 red). `tools/qa/kit-stand.ts`
  is how a check stands a kit up empty (the ground pieces' check uses it too). `round-check` line 4b's breakable round is now a post of poplar the
  check stands itself on ground it asks free (the levels' olive drums were its specimen). `woodPile` is still a stack of plain boxes.
  KEPT RUNS: a recording or a checkpoint made before this step on one of these levels is not refused where only a drum's paint moved (the level's
  fingerprint is counts and geometry), and replays against steel drums from the first round that met an olive one (the maintainer, 11-04:
  breaking replays is fine).
  MEASURED on the joined branch's tip 6c6c33df against the step on it, software only. Play-throughs of the six room-clear missions on the 27 files
  (warehouse, floodlit, mousehole, hall, glasshouse, the pass), seeds 1 to 4, on three stands (frozen, shipped, shipped with lived rooms off): 72 of
  72 runs the same digest, so every tally as it was (no run of those sent a round or a man at a drum that differs). The nine lessons (`lesson-run`,
  the page's stand, seed 1): all pass before and after; seven the same digest, TWO MOVE, tut-kit and tut-squad (their yards' drums). `vault-run`
  and `lights-out-run` (seeds 1 and 2) pass before and after on both stands. No other mission can move: the game never calls the kit, and no other
  level file changed. NO DOOR'S STACK MOVED (every door of the twenty missions on these files, both faces, one to four men, the same marks to the
  byte, both stands); wall-charge places, marks' sight, second ways and ways round are as they were on every level; the z-fight audit's 167 rows on
  the 27 files are the same rows. Sixteen level-base ids were written again (the files whose own part holds a drum that moved); eleven files moved
  in their lived things only and keep theirs.
  SEEN on the software adapter only (Chromium 141, midday): the pass's fuel pocket on the look page, before and after, two views. NEVER SEEN ON A
  GPU, and nobody has played any of these levels since.
- **A word and its finish are said with the paint, by one way** (the look's skin, five generators and the material check; no switch, no level file
  moves; 2026-11-05; asked by the dress goal after the question "is saying a word one way or three?"). The one way is `scene/stuff.ts`: `madeOf` for a
  colour written as numbers, `saidAs` for a kit's row that is another thing here, each taking the thing's FINISH with its word. THE SKIN
  (`tools/levels/gwotSkin.ts`): a `Said` hands its finish to `saidAs`, so its paint knows it, and the box takes the finish of what was picked in the one
  line that lays every pick's: the line that wrote a said paint's roughness beside its word is gone. THE GENERATORS: seventeen lines wrote a word or a
  roughness by hand beside a paint, and say it with the paint now: the office's veneer furniture, couch and sofa (its `K` table), a cabinet, the server
  room's raised floor and the conference phone (`scene/officeGen.ts`); a server rack's body, laid by the office, the block, the depot and the warehouse
  and now ONE paint named once (`stuff.ts RACK_BLACK`); the warehouse's boiler; a louvre and the breaker panel in `white-roof.ts`. EVERY level file is
  its generator's still, byte for byte (`export.ts --check` on all, `level-export.ts --check` and `--relief --check` for the office): no box, colour,
  finish or word moved, so no base file rides.
  CHECK `stuff-check`: LINE 15 also asks a gear screen's set (a box that stands as a wall, asked AS THE LOOK WEARS IT, its first-named row; a stand-in on
  its fallback row is not asked and is not to be mended: a set is drawn only under the kit view, where the plain file draws no stuff and nobody fires);
  it does NOT yet ask the look's painted bands on a wall's face (a dado, a wainscot): the consulate's and the registry's look files lay theirs in a
  cloth's paint and are leaving with their setting, so the band's line joins with whatever replaces them. LINE 16 is new, a FINDER OVER THE SOURCE of
  every module that lays a level's boxes: no line writes `stuff: '<word>'` beside a paint it names or in a literal with no paint of its own, or a
  `rough` / `metal` beside a paint that has said its word (a helper's call, or a paint named once); a piece that says its word for whatever paint it is
  handed, a set's data and a skin's Said are not asked. Red on the step before on exactly those seventeen lines; eight made-up lines are its red path.
  WHAT IT CANNOT SEE, so its green is not taken for more: a call split over lines, a finish assigned afterwards, and `white-roof.ts`, which keeps a
  paint record of its own (colour, roughness, word) and passes `stuff: P.stuff` on at some fifty calls: a second carrier of the same thing, owed by its
  owner. `STUFF_BREAK=1` fails line 15 and not line 16 (it strikes rows' words, which the finder does not read).
  PLAY: nothing a run reads moves. The three play-through reads (frozen; shipped with lived rooms off; shipped), seeds 1 and 2, against the step
  before on the same box: 42 of 42 each. The senses' error tool is green on the office on the page's stand (the office's generator is touched; no wall, leaf or lamp moves).
  SEEN by nobody and nothing to see: no picture changes.
- **FOUR WALL FITTINGS OF A KEPT BUILDING ARE KIT PIECES, AND SO ARE THE TOOLS A HAND PASS LAYS PAINT WITH** (11-05; `src/scene/officeGen.ts`
  `officeKit`; the painter's tins `TIN` in `src/scene/markingDefaults.ts`; catalogue `docs/library.md`; the pieces stand on the showroom hut's south
  face, `tools/levels/library-hall.ts`). They were the white roof's own helpers and are MOVED, not copied: `tools/levels/white-roof.ts` keeps four
  one-line adapters for its own coordinates (a face at a given z), takes its paint tools and its ink, chalk and signal yellow from the kit, and
  `levels/white-roof.json` is byte for byte the file it was. A level's hand pass USES these and copies none of them.
  THE PIECES, on a wall's face. Arguments as the look kit's wall-face pieces have them: `axis` (`'x'` = the wall runs along X at `line`), `a`‥`b` along
  it (the same stretch whichever way the wall faces), `side` ±1 (the face toward +z / +x is +1), heights, the paint; `t` is the wall's thickness (0 =
  `line` is the face itself). Each takes a paint ROW and says the row's word; each is DRAWN ONLY unless its last argument `solid` is true (the white
  roof passes it and keeps its traced plinths and pipes).
  · `band(axis, line, a, b, side, y0, y1, paint, t?, proud?, solid?)` TAKES a stretch of a wall's face and two heights; GIVES one box 8 mm proud (the
    look kit's dado's figure: under a door frame's 12) and returns its index, for what is painted on it. For paint hand it the wall's own stuff in the
    new colour (`saidAs(<the wall's word>, colour)`), never a second stuff.
  · `vent(axis, line, a, b, side, y0, y1, paint, t?, proud?, solid?)` TAKES the same; GIVES the same box two fingers proud, for a paint that says
    `louvre`.
  · `conduit(axis, line, a, b, side, y, paint, clip, drop?, t?, solid?)` TAKES a stretch, a height, the run's paint and the clips'; GIVES a run with a
    clip every metre and a half and, with `drop`, a branch down into a box at b (the box stands 11 cm past b). Only the box is ever solid.
  · `pipe(x, z, y0, y1, r, paint, rings?, solid?)` TAKES a place, two heights and a radius; GIVES a riser of four slabs an eighth of a turn apart, with
    `rings` ([y0, y1, r] each) for a flange and a collar. Solid, the round rule stands it up as one round; drawn only, it is the eight-sided post.
  THE TOOLS, for paint on what is there (markings: four a box, 1.6 m a bitmap).
  · `boxOn(face, x, y, z)` TAKES a face and a point of the world; GIVES the index of the square-standing box whose face holds it, and throws if none
    does. A box is found by where its face is, never by its number.
  · `paintOn(box, …markings)` TAKES a box and markings; GIVES the box back with them added, and throws past four.
  · `onS(box, x, y)`, `onTop(box, x, z)`, `rectTop(box, x0, z0, x1, z1)` TAKE a place or a rect of the world; GIVE it in the face's own metres from its
    centre, as the marking recipes want it.
  · `wallRuns(z, t, x0, x1, minTop, not?)` TAKES a wall along X and a stretch; GIVES every piece of it that stands on the floor to `minTop`, cut to
    the stretch, as [box, from, to, top], door to door. A piece already in the paint `not` is left out.
  · `wordWide(text, scale?, res?)` GIVES a word's length on a face in metres.
  · `stencilS(box, text, x, y, pal, scale?, res?)` and `stencilTop(box, text, x, z, pal, scale?, res?)` TAKE a box, a word or a figure and the place
    of its middle; GIVE it stencilled on the south face or the top, snapped to the marking lattice with a tie taken upward (left to the recipe's own
    rounding, two of four identical door numbers sat a cell apart).
  · `keepClear(box, x0, z0, x1, z1, pal?)` TAKES a floor box and a rect of the world; GIVES a hatched bay with a line round it, in signal yellow
    unless told.
  · `bandRuns(z, t, x0, x1, y1, paint, minTop?, names?, ink?)` TAKES a wall along X, a stretch, a height and a paint; GIVES a `band` from the floor on
    every piece `wallRuns` finds, one box a piece, and returns them. It lays NO WORD unless the caller hands it `names` (each [text, x, y, scale,
    res], stencilled on the piece that holds it whole; a name no piece holds throws). A ROOM'S NAME IS NOT ONE TO HAND IT (the office bullet below).
  · `conduitRuns(z, a, b, y, paint, clip, t?, drop?, len?)` TAKES a stretch on a wall's south face; GIVES `conduit` in equal lengths of 1.8 m at most,
    the branch on the last.
  A PAINT IS KNOWN BY ITS ARRAY (`madeOf`, `saidAs`): hand each a colour array of its own, never one shared with a tin or with another paint, or the
  second takes the first's word and finish.
  WHEN A WALL IS BREACHED (`game/wallbreak.ts` `hangers`, not changed): a drawn-only thing a quarter of a metre or more off the floor goes once half
  the wall behind it is open, so a long run of conduit is laid in lengths; a band from the floor STAYS across the hole, as the look kit's painted
  dado and skirt do on every level that has one; and a length of conduit with a `drop` hangs on, because by the game's rule it rests on its own
  branch. One gap for the whole game, not mended here.
  CHECKS on the step, both stands: `export.ts --check` for all 55 generators that have a file and both office files (none moves but the showroom),
  `library.ts --check`, `lived-check library-hall white-roof office` (the showroom's base file restamped: the four fittings are its own part),
  `round-check`, `stuff-check` (its line on a finish written by hand included), `floor-order-check`, both type checks. Two reds stand on the showroom
  exactly as on the step before: `probes.ts glassreal` line 9 (the showroom's row of the table) and `wallfurn-check --levels library-hall.json` (it
  has no count file; 86 places before and after on both stands). No mission's level file moves, so no play-through was run for this step alone.
  NEVER SEEN on any adapter as laid on the showroom; on the white roof the pieces draw as they did.
- **THE OFFICE IS HAND-FINISHED** (11-05, the first hand pass; `src/scene/officeGen.ts` "THE HAND PASS", `levels/office.scenarios.ts` `OFFICE_MARKS` and
  `walked`). The maintainer: "wear, paths in the ground, different materials can all tell a story for how people walk through the level, its logic, leading
  the player for game play"; of the look he is after, "it's all quite intentionally dressed"; of the office itself, "a lot of personality per room": added
  to, not replaced. HOW IT WAS READ: the ways field, the level's marks, and twelve scripted runs traced on the page's stand (six of the pair's room clear,
  six of the rescue). The corridor is the level (every route and both missions run its length, and the camera reads its north wall all game); the room
  clear turns at the side door between the server room and the manager's office (thirteen of thirty-one men fell within two and a half metres of it) and
  nothing in the picture said that door was there; the defenders come out of the break room by two doors; nobody, either side, enters the conference room;
  and the day's people (sixteen desks, a printer, a counter) had no way written to anything. THREE KINDS OF THING, each through the system that owns it.
  WORN PATHS are data: 31 named marks where a person stands to use a thing (`Level.marks`, attached at load beside the scenarios by `withOfficeScript`, a
  copy a Level: the level file has no slot for a mark yet) and 75 `walked` rows between them, each name one thing (a mark, a door's id, or a room's name that no door
  shares); the look's page wears the floor by the ways field (`?walked=0` = the lanes as they were). Nothing in play reads a mark: `ways.ts` and the
  Director's lookup by a script's own id are the only readers, and the office has no script. The walked check's two count files for the office are gone:
  `--write` dropped `door conference` and `start spawn` (office) and `door conference` (office-relief), each on a way now. PAINT on a surface that was
  there is a marking on that box (both stands): the four north rooms' numbers on the lintels at their doors, a hand high, and one keep-clear bay on the
  ground outside the fire exit. NO WORD NAMES A ROOM AND NO PAINT IS THERE FOR THE PLAYER. The first cut stencilled each room's name (LOBBY and DESKS on
  the floor at the openings, CONFERENCE on its sill, SERVER, MANAGER and STORAGE on the dado, STAIR on the tower), door numbers half a metre tall, a
  `02←→03` sign for the side door, a yellow line across the server room's floor to that door and keep-clear hatching on four floors indoors. The
  maintainer, playing it: "i don't like the "lobby" and "conference" labels in the level, pretty lame". All of it is out (the words, the sign, four
  floor panels, the line, the four indoor bays) and the door numbers are a door's own size: a room is shown by what is in it, and the test for paint is
  whether THIS building's own people would have painted it for themselves (a door's number, a parking bay's number, hatching outside a fire exit; not
  a hatched bay on an office floor, not a line that leads). Nothing was put in their place: what would be (a thing a room has, not paint) is the next
  pass. FITTINGS are 62 new boxes, every one drawn only by its own flag, in the office's LIVED TIER (its first: groups `lived.corridor`,
  `lived.breakroom`, `lived.exterior` at the tail): a grey dado along the corridor's north wall in the conference sill's own paint; conduit and a
  grille on that wall; a navy band and a vent in the break room; outside, a dark plinth on the south face with the five bays' numbers a hand high
  (17.5 cm; the maintainer, the same morning: "let's avoid large markings (e.g. numbers or room names) where they don't seem realistic"), conduit to
  the lamp over the south door, a vent and two downpipes (the kit's `band`, `vent`, `conduit`, `pipe`; the paint by the kit's tools). SWITCH: the tier is
  `LIVED`'s (`?lived=0` on the page; the frozen stand loads without it), so the frozen stand's office has the 783 boxes, the indices and the
  fingerprint it had, and the page's has 845: A CHECKPOINT OR A RECORDING MADE ON THE PAGE'S OFFICE BEFORE THIS STEP NO LONGER LOADS THERE.
  `buildOfficeProcedural` takes `lived` (absent = as this stand loads the file), the exporter asks for it, and `probes.ts level` holds the file against
  the generator with the tier in on either stand. A texel variant of the office (`level-export.ts --texel`) is the office without its hand pass: the
  paint is cut for the base lattice. THE CAMERA: it opens at yaw -22 and pitch 55 degrees and turns with him. At rest it shows floors, tops, south
  faces and west faces, and a wall hides about two metres of floor behind it. The first pictures were pinned at yaw 0 and pitch 58, and one finding
  made from them was wrong (that the side door could not be seen: from the play camera it can). Judge a level from its own camera. ON A BREACH (tried
  headless, a blast on each of four partitions): the lengths of conduit and the vent over the hole go with the wall and the lengths beside it stay
  (but the one length that carries the branch hangs on: the kit bullet above); the bands stay across the foot of the hole (the kit bullet above: the
  game's rule for every painted band, not mended here). Paint on a wall piece goes when that piece is opened, the whole piece's at once (read in the
  code, not tried).
  NOT TOUCHED: the hour and every lamp (both rows open at night), every prop, the lot's and the lawn's ground. `ways-check.ts` now opens the office as
  the game does, with its script, so its rows are checked at all (before, the office was read with no ways of its own).
  CHECKS, both stands: `probes.ts level`, `lived-check office` (62 boxes, 0 solid), `wallfurn-check` (669 and 669, held), `marksight-check`,
  `second-way-check`, `stuff-check`, `probes.ts hitstuff`, `probes.ts kept`, `floor-order-check`, `ways-check` (and its break path fails),
  `walked-check`, `round-check`, `level-needs office --run` (the same four notes as the step before), `wayround-check` on its two missions, seed 1
  (nothing to ask), `modelerr` (the page's stand: both kept reports still answer), `zfight-check office` (8 places seen and 186 noted, the same as the
  step before).
  SEEN on a real GPU in stills only (a rented NVIDIA A10G, quality high, the look page, night, seven cameras before and after at the play camera's
  resting view), AND THOSE STILLS ARE OF THE CUT BEFORE THE LINE AND THE INDOOR BAYS CAME OUT: what is here now is that picture with less in it, and
  has not itself been drawn. What they showed: the car park's plinth, numbers, conduit, vent and downpipe read; in the corridor a grille, the conduit
  and its box and a slightly greyer foot to the wall; the cubicles and the west end are the same picture before and after; the break room's navy band
  is in the dark; and THE WORN LANES DO NOT SHOW AT NIGHT (they move the carpet by a level or less of 255: the look's wear layer's strength, not the
  rows). An outside review of those stills said the same and its ideas are held for the next pass. The first cut was priced twice before and twice
  after on rented boxes: no cost that shows over what two boxes differ by. The maintainer played the first cut and his word on it is above. Never
  seen on an Apple or an AMD GPU.
- **FOUR MENDS TO THE OFFICE'S HAND PASS, FROM A REVIEW** (11-05; no level file moves, no box, no rule of play). (1) THE WAYS THROUGH THE SERVER
  ROOM END AT A MARK. Three `walked` rows named the room, and a room's name is the middle of its floor, which a bank of racks stands on: the walk
  grid stepped each way aside to the racks' near face, and the row's comment said "between the racks". They now name a mark, `racks`, on free floor
  at the rack the plain night's drive is pulled from (31 marks). A ROOM'S NAME IS A FAIR END ONLY WHERE THE ROOM'S MIDDLE IS FLOOR. (2) THE FOUR
  DESK CLUSTERS STAND WHERE ONE LIST SAYS: `levels/office.scenarios.ts DESK_CLUSTERS`, read by the desks' marks and by the generator, which had
  the same four places written again. Left: the offsets of a desk and its chair inside a cluster are still said twice (the generator's
  `cubicleCluster` and the marks' arithmetic); they are one writing when a mark is the level file's own and the cluster lays its desks' marks.
  (3) THE CATALOGUE COUNTS THE OFFICE AS ITS FILE HAS IT: `tools/levels/library.ts` builds the office with its lived tier (the exporter's word),
  and the kit's two tools, `bandRuns` and `conduitRuns`, lay through the kit's door (`pieces`), so a band or a length of conduit a tool lays is
  counted as the piece it is. The uses page (`library.ts --uses`) had the office at 193 pieces with no band, vent, conduit or pipe of the hand
  pass; it is 228 with 19 bands, 11 lengths of conduit, 3 vents and 2 pipes. The catalogue in the repo does not move. (4) ONE FLOOR FOR THE
  RULE THAT EVERY WAY HAS A PATH: `ways-check`'s sweep over every level now opens each as the page walks it (`walked-check.ts pageWays`, the lived
  rooms and the dressing in), where it opened each as the caller's stand does. NOT A FAULT, read and left: the kit view's set path loads a level
  without the office's marks, but that level is a SET (its own boxes and doors, its scenarios the kit view's, the room clear's and the issue's
  only): it carries no `walked` row, and the office's block has no set. CHECKS, both stands: `ways-check` (and its break path), `walked-check`,
  `probes.ts level`, both export checks, `library.ts --check`, the exporter's check on the white roof and the showroom, both type checks.
- **THE OFFICE SAYS IT IS KEPT** (11-05; the builder's `kept: 1` in `src/scene/officeGen.ts`, so `meta.kept` in `levels/office.json` and
  `levels/office-relief.json`; the rule is the look's, `docs/agents/render.md` A LEVEL SAYS HOW KEPT IT IS). The first level to say the word. WHY 1 AND
  NOT PART WAY: it is a let office with a cleaner on contract, and the paint its hand pass laid (door numbers and bay numbers a hand high) was drawn as
  every mark was before the word, rubbed and gone a shade into its wall, which at that size reads as dirt on the picture and not as a number. At 1 a
  mark is whole and its own colour and a plaster wall is one shade with its knocked foot. A part-way word is the one the look has never been priced
  at. WHAT MOVES: the look's page's picture of the office and nothing else. No rule of play reads the word, the level's fingerprint is taken over its
  boxes, and a checkpoint or a recording loads as before. `&kept=0` on the address is the office as it was drawn, for the visit. WHAT IT DOES NOT
  DO: the worn lanes are where feet go and do not ask the word, so they are as faint at night as they were.
  CHECKS, both stands: `probes.ts kept` (and its break path fails), `probes.ts level`, `level-export.ts --check` and `--relief --check`, and the
  hand pass's own list in the bullet above. The office's four missions, seeds 1 and 2, on the frozen stand, the page's and the page's with its
  lived tier off: 8 of 8 run digests the same as the step before, each. The page opened on a software adapter with the word read off the level (1)
  and nothing in the console.
  PRICED on a rented NVIDIA A10G, the office alone at night on high, twice a side: what the look adds over its layers off was +21.1% and +21.3% of the
  frame before and +21.1% and +23.0% after, and two boxes of one kind differ by more than that. No cost that shows.
  SEEN on that GPU in stills only, seven cameras at the play camera's resting view, before and after on one base: a door's number draws whole (`01`
  was two broken strokes), the lobby sign's lines and the bays' numbers are a shade stronger, and nothing else reads as changed. Of the picture's
  pixels 0.1 to 0.6% move by more than 8 of 255 indoors and 1.5% on the lot; two draws of one commit differ by 0.01%. Never seen on an Apple or an
  AMD GPU, and nobody has played it.
- **NO TWO DRAWN FACES OF THE OFFICE LIE IN ONE PLANE** (11-05; `src/scene/officeGen.ts` `HAIR`; `tools/qa/zfight-check.ts office` went from 8 seen
  patches to none, its 186 noted pairs as they were). The eight were the office's own: the lobby's corner planter, whose plant was exactly its pot's
  footprint and stood 5 cm into it (four sides), and the four desk clusters, where the cross partition met the spine at one height in another cloth
  (four tops). THE MEND is the check's own, a face sunk a hair (4 mm, one named number): the plant's sides stand inside the pot's and the cross
  partition's top is under the spine's. Five boxes are a hair smaller; none is added, removed or moved in the list, and none moves at the centimetre
  the level's fingerprint reads, so a checkpoint or a recording still loads on either stand. BOTH STANDS' OFFICE CHANGES (these are boxes of the base,
  not of a tier): a ray that grazed the top 4 mm of a cross partition, or the plant's outer 4 mm above its pot, now passes. No switch: it is level
  data, and off is the file before. WHAT IS LEFT for a later hand: the noted pairs are drawn alike today and flicker only under a look that paints a
  box by its own size or place. CHECKS, both stands: the hand pass's own list (`wallfurn-check` still 669 and 669), `probes.ts kept`, `probes.ts
  level`, both export checks, and `zfight-check office` (its break path fails). The office's four missions, seeds 1 to 4, on the frozen stand, the
  page's and the page's with its lived tier off: 16 of 16 run digests the same as the step before, each. The page opened on a software adapter with
  the five boxes at their new sizes and nothing in the console. Never seen on a GPU: the check reads the file, and nobody has watched the eight
  places for a flicker before or after.
- **A named coat's finish goes wherever its word goes: a vehicle's body is a clear coat** (`scene/stuff.ts COATFINISH`, `coatRough`, `NAMED_COATS`; read
  by `scene/boxes.ts matWord`; 11-05; the maintainer, playing the preview: "how reflective is the car material, btw? it does feel like it should reflect
  more of the environment and be quite dynamic as we move around it"). WHAT WAS WRONG, read from the code: a vehicle's body took its finish from the
  paint's ROW. The truck enamels are satin (0.35, 0.4: a cot's and a locker's finish too), so on High the traced mirror was part faded (it fades over
  the 0.12 under `rtMaxRough` 0.45) and wide, and on Medium (0.35) there was none; the plain car and the office's van had no finish at all (matte, 1.0:
  no reflection ray); a twin's skinned car wore its new row's (0.35 to 0.75); the bazaar's cab a 0.5 written by hand. THE RULE: a named coat's row in
  the table of coats says how smooth it is (carpaint 0.2: under every preset's mirror cut-off less its fade, the number agreed with the look's car
  paint map, render.md "CAR PAINT IS A STUFF"), and the ONE function that hands a box's finish to the GPU (`matWord`) sends a box that says a named
  coat no rougher than the coat, whatever row painted it and whatever number stands beside the word. So it holds for a level's statics, a shape's
  pieces, a piece blown off a car (which copies the word and no finish) and anything made later that says the word. NO BOX IS CHANGED AND NO LEVEL FILE
  CHANGES: `Box.rough` is the file's, always. THE WORD DECIDES: a thing that should not shine like a car does not say it is car paint (the tanker's
  burnt cab says plastic). Smoother stays smoother (a body wet with rain). ROUGHNESS ONLY: `metal` stays the paint's, because the game reads that flag
  (`wallpen.ts`, `wallcharge.ts`) and reads no roughness. (Three cuts were thrown away on review, so nobody tries them again: 0.2 written beside the
  word on 507 boxes in 40 level files; a rule in the level file's reader and writer; a pass over the level's boxes at stand-up that kept each box's own
  number in a side table, which no snapshot can list.)
  THE SWITCH: ON in the page (`?coatfinish=0` = every box sent the finish its file gives it, as it was: the A/B for its cost), OFF in the module and on
  the frozen stand (`TPS_COATFINISH=1`), a `FROZEN_HOLDS` row (a picture only), `STAND_ENV`, pinned in `standup.ts` beside the shapes' word, in the
  registry and the journal (a recording with no table plays with it down). It can be flipped on a live page: the statics' packed slice is compared by
  this float each frame.
  UNDER THE LOOK the car paint map varies the number (`rough + dr`, its delta taken against the look's own row 0.26): about 0.11 where the coat is
  clean, 0.37 under road dust.
  WHAT IT LOOKS LIKE, and it is LITTLE BY ITSELF. Stills on the software adapter (11-05, the yard's white pickup at midday and at night, High): the
  body at its file's 0.35, at 0.2 and at 0.12 are the same picture to the eye (mean difference 0.08 to 0.22 of 255 a pixel). A non-metal mirrors 4 % face
  on (`reflect.wgsl` F0), and a sharper 4 % does not show over a bright paint. What the maintainer asked for needs a clear coat as a term of its own
  in the shader (the renderer's part took it, 11-05), which can key on this word. What this rule gives by itself: Medium traces the bodies at all, the
  plain cars and the twins' stop being matte, and one number is the coat's. NEVER SEEN ON A GPU.
  THE CHECK: `bun run tools/qa/probes.ts coatfinish` (`tools/qa/coatfinish-check.ts`, in the `shapes` family), both stands: the stand; the yard as
  stood, up and down (every box and piece that says car paint sent at the coat's, nothing else moved, no box's own number or metal touched); the
  cases by hand (a twin's skin-written number, a body smoother than the coat, a bare-metal row, a piece off a car, a plain paint, a word that is no
  stuff); every level file through the rule (507 boxes in 40 files on the shipped stand); the same seeded run by digest up as down; the rows and an
  old recording. `COATFINISH_BREAK=1` fails four lines on both stands.
  NOT DONE: a plain car's and the office van's GLASS says no finish (matte glass; the kits' dark glass is 0.1); the plough lorry's tipper sides are
  plain orange beside a coated cab; the editor's finish panel shows the file's number, not what is sent. COST: smooth finishes are what the reflection
  pass traces, and the plain cars are new to it: NOT MEASURED ON A GPU.
- **A LEVEL FILE NAMES ITS MARKS** (the format, no switch, no rule of play; 2026-11-05). A mark is a named place on the floor where a person stands to
  use a thing: the end of a `walked` way and a script's place, read by name (`scene/ways.ts`, `game/director.ts`). Until now only the office had any, and
  only as code joined on at load (`scene/level.ts withMarks`, which is GONE). The file has a slot: top-level `marks: [{ id, pos, yaw? }]`
  (`levelData.ts MarkData`, validated: an id once, `pos` a Vec3, `yaw` a number), loaded to `Level.marks` and written back by `levelToData`, as `volumes`
  are. A GENERATOR RETURNS `marks` ON ITS LEVEL and the exporter writes them; absent = none, and no other file changes. THE OFFICE IS THE FIRST: its
  builder puts `OFFICE_MARKS` (31) on its Level and both its files carry them; a Level's marks are an ordinary key now. An office saved in the editor
  before the key has no marks until it is saved from a file that has them. DO NOT USE A `scenarios.places` AREA AS A MARK: the game reads places.
  CHECK: `walked-check` is red on a mark standing where the walk grid is shut; `probes.ts ways` already is on a name that is two things. As measured on
  the step: the office's two files differ from before by the `marks` key alone, and loaded with the office's script each gives the same 31 marks, the
  same scenarios and every other key of the Level the same as the build before it. Nothing a page draws moves.
- **Lots and lawns say tarmac and grass** (the two paints' rows and every level file; no switch of its own; 2026-11-05; the dress goal's ask, the
  office's follow-up grown to every level, and the step the office's hand pass waits on). The look has had a tarmac and a grass since the same day
  (`render/lookTex.ts`); the WORDS are said at the paints' rows, `scene/stuff.ts PAINT_STUFF`: `asphalt` was 'plain' and is 'tarmac', `grass` was 'waits'
  and is 'grass'. A level's FILE carries its rows' words, so ALL 59 LEVEL FILES were exported again and each differs from the step before by exactly
  those two words (proved file by file with the words struck out) and by nothing else, but for ONE BOX: the compound's mud at the hand pump was laid in
  the asphalt paint for its darkness and says `earth` now (`compound.ts`, `saidAs`; the same box in its look file). No box, colour, finish or number
  moves; no base file rides (`lived-check` holds every one as it stands).
  WHAT A ROUND GETS, and the step's one line under `src/game`: `game/particleFx.ts STUFF_SURF` gains the row `tarmac: 'concrete'` (the row the cars'
  part asked for; written here and nowhere else). DRY, nothing moves: a lot's paint was concrete to a round by its colour. WET, THE PICTURE OF A HIT
  MOVES, on the page only (the row rides HITSTUFF: `?hitstuff=0`, `TPS_HITSTUFF=1` on the frozen stand): the storm halves the lot's paint and that tone
  read as metal (`weather.ts` and `gwotPalette.ts` say so, "as measured", and both are still true of what the SIMULATION reads, `destruction.ts`), so a
  round on the wet lot threw a metal's sparks and no chips. A lot that says tarmac is concrete wet or dry: a few sparks, chips and dust. Nobody has seen
  it. A LAWN TAKES NO ROUND: every grass box is a strip no ray meets (`NoShadow`) lying on the lot's slab, so a round into the office's lawn is a round
  into the slab under it, concrete before and now. A `grass` row would read nothing while lawns are laid so; a lawn that gives blades and earth is a
  change to how a lawn is laid. One thing for later.
  THE GROUNDS, READ: every office-kit level lays one 40 by 28 m slab in the asphalt paint under everything: twenty files. For the levels that stay on
  the mission list each was read against its generator's own words: the office and its relief file (one generator; a car park: tarmac; its two strips
  are a lawn: grass), the depot (hardstanding and the road: tarmac), the consulate (a street: tarmac; its strip a verge: grass), Mousehole (the yard is
  grass, the rest asphalt) and the compound (the road south and east: tarmac; its yard has an earth slab of its own over it; its two strips are SCRUB
  and say grass now: left, the compound is to be re-set). FLIPPED AND NOT READ, each a ground that may be no lot: barracks, block, block-squads, defuse,
  floodlit, hall, killhouse, kit-yard, quiet-yard, records, registry, sneak-lab, test-rooms, warehouse (asphalt), and the grass of barracks, block,
  block-squads, defuse, records, registry and warehouse. ALSO FLIPPED, stand-ins: eleven boxes of the gear screen's sets in four plain files (the
  compound's eight, and the set ground of defuse, the registry and Mousehole) name their paint 'dirt|asphalt' and the like and fall back to the two
  rows where the file has no look row; they are drawn by the plain file only and are not mended (`stuff-check` line 15's rule).
  WHERE IT IS DRAWN: a stuff is drawn on the look's page (`?look`) and nowhere else. Fourteen of the twenty have a look file whose skin repaints the
  ground, and the page draws that file unless the dress is put off, so their tarmac and grass are not drawn. OF THE 29 LEVELS ON THE MISSION LIST ONE
  CHANGES, THE OFFICE (two rows): its slab (the car park round the building) is tarmac and its two strips, 207 m2, are grass, where both were flat
  colour; and the compound's 1 m2 of mud is earth. Off the list: the office's relief file, kit-yard, quiet-yard, sneak-lab and test-rooms draw their
  slab as tarmac. THE PRICE, not measured in a browser: the look makes a stuff's maps once a boot for a level that names it (`render.md`: grass about
  0.7 s of arithmetic and tarmac 0.55, under bun), and the office names both now, the last two layers made.
  CHECK: `stuff-check` line 4's bar moved with the word (the office's car park is tarmac where it was plain: a bar moved, not loosened) and asks the
  lawn; its new line 17 holds the two words to the ground (in every file each box that takes tarmac or grass from its row is a flat box of the
  `ground` or `floors` group: 20 lots, 22 strips; a mat and a kerb written in the car park's paint are found, one that says earth is not) and a lot
  to concrete in the picture of a hit; `STUFF_BREAK=1` fails 17 with the rest. `hitstuff`, `marksight`, `lived`, `lookmaps`, `walked` (no `--write`),
  `probes.ts kept`, `wallfurn` and `wayround` green on both stands; the senses' error tool green on the office on the page's stand.
  PLAY: no move. The play-through's 42 runs (21 missions, seeds 1 and 2) on the frozen stand, the page's, and the page's with its lived tier off, the
  step against the tip it is cut on (step 1459): 42 of 42 run digests the same, each. SEEN, on the software adapter only (unsettled frames: what is
  laid where, not what a material looks like): the office opened under `?look` at midday with the lot and the lawn drawn in their stuffs, 120 of 120
  pipelines, no page or GPU error. The look task's settled still of the two was drawn on its own branch. Never on a GPU here, and nobody has seen a
  round on the wet lot.
- **A thing of solid glass is dark and smooth** (`scene/stuff.ts SOLIDGLASS`; read by `scene/boxes.ts glassDark`, `sentAlbedo` and `matWord`; 11-05,
  from the renderer's review of the vehicles on a GPU: "near-black panes, finish 0.05": glass is what mirrors most on a car, and the kits' dark glass
  was a mid grey at 0.1, a plain car's and the office van's a grey with no finish at all). THE RULE: a box that says `glass` and is NO PANE
  (`solidGlassBox`, the one test, which the hit picture's `solidGlass` asks too: a vehicle's window, a display case's top, a handset's screen; 100
  boxes in 39 level files; a pane, `BoxFlag.Glass`, is the glass engine's and is not touched) is SENT to the GPU as a NEUTRAL dark of luminance 0.02
  and no rougher than 0.05. Darker and smoother stay as they are; a wet one is that dark times its wetness; a face that carries MARKINGS keeps its
  colour (2 boxes: a solar panel's grid, a shop's case), because a mark is laid out in its own colours on the colour under it. Neutral and not the
  row's hue scaled down: under about 0.03 a hue does not survive the 8-bit albedo target. THE COLOUR IS NOT REPAINTED, which is why this is a rule at
  the pack and not a paint: the game reads a box's colour (a round's material with the hit rule down, a blast's dust), and the kits' dark glass is
  glass to a round by being its palette's row; a new near-black would be no row's. So `Box.albedo` and `Box.rough` stay the file's and the pack sends
  the colour in the one place it already sends a wet box darker (`sentAlbedo`, beside `Box.wetK`: the packed albedo, the secondary albedo's base and
  the static slice's compare). NO LEVEL FILE CHANGES. THE SWITCH: ON in the page (`?solidglass=0` = as its file gives it), OFF in the module and on
  the frozen stand (`TPS_SOLIDGLASS=1`), a `FROZEN_HOLDS` row, `STAND_ENV`, pinned in `standup.ts`, in the registry and the journal; its two numbers
  ride in the table (`SOLIDGLASS.lum`, `.rough`), so the page's knobs and a recording carry them. A PICTURE ONLY ON THE HARNESS: on a GPU page the
  light meter's bounce reads the packed colour, so a man beside a vehicle is metered a hair darker, as beside wet ground (`wetK` is the same).
  THE CHECK: `tools/qa/coatfinish-check.ts` lines 6 and 7 (`probes.ts coatfinish`), both stands: the cases by hand (a window, one with no finish, a
  pane, a plain paint of the same grey, a darker glass, a wet one, a marked one), every level file, THE WORDS THE WORLD SENDS for the yard's own
  window (up, down, and up again on the same stood world: the static slice packs again), the same seeded run by digest up as down, a stand given the
  word, the rows and an old recording; `SOLIDGLASS_BREAK=1` fails both lines. NOT DONE, and known: a window DRAWN AS A MARK on a body (the police
  post's taxi, the bazaar's workshop set) is a mark and stays its grey beside a boxed window gone dark; a MIRROR that says glass is the same dark
  glass with a 4 % mirror (it wants a mirror's own word); the workbench does not raise this or any page switch, so the editor shows the file's grey;
  the renderer has no layer for glass. SEEN ON A CLOUD NVIDIA CARD in the renderer's study build, stills only (the yard at midday, a red pickup at the
  crossing, the showroom; no fog of war, nobody shot a window): it reads as glass and not as a hole, and no glint showed at 0.05. Nobody has played it.
