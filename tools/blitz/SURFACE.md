# The call surface

Written by surface.py. 1095 rows of `io::call`, 803 with a guard at load. What is not on it, by why:

- **the host's own, or the host already answers it** (104): `BoxMirror.followLeaf`, `BoxMirror.followRow`, `BoxMirror.gridDrop`, `BoxMirror.open`, `BoxWorld.addCharacterCapsules`, `BoxWorld.addStatic`, `BoxWorld.rayAside`, `BoxWorld.raycast`, `BoxWorld.relist`, `BoxWorld.repackMarkings`, `BoxWorld.resetCapsules`, `CharacterRenderer.finish`, `CharacterRenderer.setInstance`, `CharacterRenderer.setMaterials`, `Checkpoints.load`, `Checkpoints.loadInStep`, `Checkpoints.tick`, `Checkpoints.write`, `CpuSmoke.along`, `CpuSmoke.blobsAlong`, `CpuSmoke.clear`, `CpuSmoke.emit`, `CpuSmoke.step`, `FollowCamera.autoYaw`, `FollowCamera.drawIn`, `FollowCamera.drawOut`, `FollowCamera.easeYaw`, `FollowCamera.frame`, `FollowCamera.kickIn`, `FollowCamera.kickOut`, `FollowCamera.rebuild`, `FollowCamera.rotate`, `FollowCamera.setPin`, `FollowCamera.step`, `FollowCamera.update`, `FollowCamera.zoom`, `Game.followPos`, `Game.msg`, `Game.step`, `Game.stepSeats`, `Game.update`, `Input.detached`, `Input.endFrame`, `Input.endTick`, `NavGrid.findPath`, `NavGrid.nearestFreePoint`, `NavGrid.rebake`, `PoseFK.run`, `Seat.hand`, `Seat.handed`, `Seat.load`, `Seat.swallow`, `Seat.takeCut`, `Sensors.ask`, `Sensors.askSegment`, `Sensors.beginFrame`, `Sensors.endFrame`, `Sensors.forget`, `Sensors.lightAt`, `Sensors.live`, `Sensors.read`, `Sensors.record`, `Sensors.replay`, `Sensors.segment`, `SessionRecorder.cut`, `SessionRecorder.frameEnd`, `SessionRecorder.frameStart`, `SkeletonInstance.update`, `SmokeGrid.clear`, `SmokeGrid.emit` and more
- **no Rust function** (95): `Attract.poke`, `Attract.stop`, `Attract.tick`, `Attract.update`, `BoxWorld.ensurePacked`, `BoxWorld.forgetSent`, `BoxWorld.markRecordsOf`, `BoxWorld.resetMarkTable`, `BoxWorld.setStuffOf`, `BoxWorld.upload`, `CharacterRenderer.drawRuns`, `CharacterRenderer.forgetSent`, `Checkpoints.importText`, `Checkpoints.offer`, `Demo.goto`, `Demo.jump`, `Demo.start`, `Demo.stop`, `Demo.update`, `Doors.onTorn`, `Doors.updateBoth`, `PoseFK.parentRot`, `PoseFK.runFrom`, `PropSystem.updateBoth`, `ReplayBuffer.top`, `RunTapeRec.top`, `SessionPlayer.applyFrame`, `SessionRecorder.stop`, `SmokeGrid.free`, `applyKnobs`, `attentionWay`, `blastFeelStages`, `buildBundledLevel`, `buildLevel`, `calmTorchRestore`, `castWear`, `chainBits`, `darkFoeRestore`, `doing`, `dryAll`, `engage`, `extraSpots`, `floorPlan`, `holdFireRestore`, `hudExpireBubbles`, `inRain`, `initCutSwitch`, `initDifficulty`, `initHour`, `kitAddLaid`, `kitLoad`, `kitSave`, `lanesRestore`, `layRestore`, `levelToData`, `loadCharacterSet`, `loadGltfCast`, `loadGltfCharacter`, `loadLevel`, `manLeaves`, `moraleRestore`, `namePaint`, `namePaints`, `netQuietRestore`, `ownSmoke`, `pictureRestore`, `plainLevel`, `powerOutRestore`, `rainDripPoints`, `recipeOf` and more
- **takes a closure** (30): `Debris.impulse`, `Defuse.samRows`, `Destruction.withWallsOpen`, `Door.posAside`, `DoorTools.samRows`, `Doors.posAside`, `EntryPoints.clearOf`, `Game.aside`, `GlassBreak.asking`, `GlassBreak.seeThrough`, `RescueEnd.rows`, `SnatchOut.rows`, `SquadOrders.askAside`, `SquadOrders.noRoomWords`, `WallCharge.clearOf`, `WallCharge.samRows`, `WorkThings.askAside`, `coverNear`, `hearWhistle`, `holdFireHolds`, `leversFog`, `lightEstimateAt`, `ownerWitness`, `pictureQuiet`, `pictureStep`, `quietFogAside`, `skyAside`, `sneakWalk`, `stickyRows`, `switchRows`
- **no body yet** (15): 
- **takes no game: nothing to write** (7): `AimState.clear`, `Ocp.update`, `Ocp.use`, `RateCap.reset`, `RateCap.take`, `gpFragments`, `manifestOf`
- **answers what its Rust function does not** (5): `Destruction.onBlast`, `Pose.copy`, `WallBreak.onChange`, `searchPlanFor`, `wallReactOf`
- **a parameter only Rust has** (4): `blendPose`, `layerSlot`, `sampleClip`, `spawnShot`
- **takes an address's words, which are not on the wire** (2): `weatherAsked`, `quietFogStart` (in Rust they take a `Query`, `rust/game/src/query.rs`; a host hands an address's TEXT across)

## The picture's own functions have no row

A function that is the picture's or the host's — its Rust body a marked boundary no-op, its whole work the TypeScript body the dress step runs inside a
tick — is not a crossing. With a row and a guard, a call from OUTSIDE a tick (the scene panel's blast button, `__snow.blastAt`, `__wet.summary()`, a
check that steps a man's breath by hand) crossed, the Rust game did nothing and answered `false` or an empty line, and the TypeScript body never ran:
the button told the snow nothing, and ten seconds at a sprint filed no breath. Inside a tick nothing changed, since the host's own lines never cross.

Taken off the table, and the guard out of the nine that had one on disk (the last two never had one): `thinFit`, `wetSummary`, `recapTick`,
`updateSnowBreath`, `updateSnowTracks`, `snowBlastAt`, `updateSnowFx`, `updateRainFx`, `updateSnowMarks`, `Par.tick`, `WetGround.follow`. That is every
row whose Rust body carries the boundary no-op mark, and `wetSummary`, whose body is marked a boundary and answers an empty line. 1084 rows are left.

A row is named by its `id`, which is the dispatcher's arm in `rust/game/src/call_gen.rs`, not by its place in `calls.json`: the eleven left holes and
no other row moved (`surface.ts` indexes by `id`). The eleven arms and their no-op bodies are still in the Rust crate, reached by nothing from the
TypeScript; they can go with the next change there, and no number needs to move when they do.

ALSO OFF, the host's own though it has a Rust body: `sampleCmd` (row 498), which writes a tick's record from the live Input, camera and window (of the
game it reads a showcase's aim, `g.puppet.aim`, which moves the host's cursor). While the harness's Input was no row of the game it passed as the
TypeScript; from the step that hands the host's camera and keys to the game at the stand it crossed, 5.5 ms a call (0.05 before), for every tool that
samples a record itself: the two-seat checks, and the scripted clients of a game server, which then started 14 to 21 beats late and never drew level
(`mpserver-check` `ahead`). A tool that steps by `Game.update` was not slower: inside it nothing crosses, and so the host's other lines
(`Game.hostBefore`, `Game.hostAfter`, `Director.injectInput`: called there, and by a networked page, which has no surface) never do. The Rust arm is
reached by nothing from the TypeScript. 1062 rows are left.

LEFT ON THE TABLE though their Rust body is empty, because the nothing is the game's own answer and not the picture's: `SnatchOut.switched` and
`closeMarks` (a settled switch), `fogFlags` and `loadCivTune` (the address and the browser's storage are the host's; their TypeScript would read a page).

How the class is found again: every `pub fn` under `rust/game/src` whose doc line says `BOUNDARY NO-OP`, joined to `calls.json` by its `rust` path.

## The save file's road has no row

A checkpoint written by the TypeScript is a FILE OF THE PAGE'S OWN FIELDS: each module's `snapshot` reads the puppet and its `restore` writes it. The Rust
game keeps its saves as whole states on its own shelf and has no use for a module's slice. With a row, a call between ticks under a host crossed to a
Rust function nobody had written, and the game stopped with `no body: <name>`; without one the TypeScript body runs on the puppet as it does on the page
and answers (`canLoad` says the file is the page's own and the game is stepped elsewhere).

Taken off the table (eleven): `WallBreak.restore`, `Furniture.check`, `Furniture.restore`, `RescueEnd.restore`, `RescueFlush.restore`,
`AirDust.snapshot`, `AirDust.restore`, `canLoad`, `Lesson.restore`, `FogOfWar.snapshot`, `FogOfWar.restore`. None had a guard on disk. Their arms and
their named panics left the Rust crate with them. 1073 rows are left.

LEFT ON THE TABLE of the functions nobody had written: `openSky`, `underRoofRect` and `SquadOrders.summary`, which have bodies now, and `SquadOrders.hud`,
which stays a named panic: the orders' panel is built on the page by the TypeScript of that name, and a tool that asks the Rust game for it is told so.

## Called with no game stood (the census's kind iv-b)

**Stays TypeScript after the deletion**: a reader with no Rust function. A file marked "with tick code" holds a kept reader and code the deletion takes, so it must be split or the reader moved out.

| function | file | |
|---|---|---|
| knobsBack | src/game/knobs.ts |  |
| knobsCopy | src/game/knobs.ts |  |
| canonicalLevelData | src/scene/levelData.ts |  |
| freshId | src/scene/levelData.ts |  |
| indexLevelData | src/scene/levelData.ts |  |
| levelDataDigest | src/scene/levelData.ts |  |
| serializeLevelData | src/scene/levelData.ts |  |
| structuralDiff | src/scene/levelData.ts |  |
| validateLevelData | src/scene/levelData.ts |  |
| entryOfQuery | src/scene/scenarioList.ts |  |
| pickSearch | src/scene/scenarioList.ts |  |
| quitSearch | src/scene/scenarioList.ts |  |
| scenarioQuery | src/scene/scenarios.ts |  |

**Two bodies, one checked**: values in, values out, a Rust body and no game. After the deletion either the TypeScript one stays as the host's and the Rust one is checked against it by a test of its own, or one goes.

| function | file | Rust |
|---|---|---|
| cueClass | src/game/cues.ts | crate::cues::cue_class |
| floorAt | src/game/floor.ts | crate::floor::floor_at |
| gearText | src/game/gear.ts | crate::gear::gear_text |
| handlingOf | src/game/weapons.ts | crate::weapons::handling_of |
| knobRegistry | src/game/knobs.ts | crate::knobs_reg::knob_registry |
| packRect | src/game/weatherKinds.ts | crate::weather_kinds::pack_rect |
| plainFog | src/game/scenarioRun.ts | crate::scenario_run::plain_fog |
| sideOfTeam | src/game/charLook.ts | crate::char_look::side_of_team |
| specOfQuery | src/scene/scenarioList.ts | crate::scene_scenario_list::spec_of_query |
| squadsOf | src/game/difficulty.ts | crate::difficulty::squads_of |
| teamWords | src/scene/scenarioList.ts | crate::scene_scenario_list::team_words |
| unpackRect | src/game/weatherKinds.ts | crate::weather_kinds::unpack_rect |

## Probes that plant an answer

A replacement that answers for the function does not change the Rust tick: a tap only observes. These stay on the TypeScript tick until re-aimed at a real cause.

| file:line | replaces |
|---|---|
| tools/qa/attention-check.ts:70 | s2.g.fogOfWar.hidden |
| tools/qa/autocam-check.ts:173 | fog.hidden |
| tools/qa/autocam-check.ts:256 | fog.hidden |
| tools/qa/autocam-check.ts:431 | g.fogOfWar.hidden |
| tools/qa/breathsteam-check.ts:131 | fog.hidden |
| tools/qa/breathsteam-check.ts:149 | fog.hidden |
| tools/qa/burstglass-probes.ts:141 | R.g.destr.blast |
| tools/qa/glassbreak-probes.ts:559 | g2.wind.stormWind |
| tools/qa/holdfire-check.ts:265 | g.rescue.lastWarning |
| tools/qa/holdglass-probes.ts:141 | R.game.rescue.lastWarning |
| tools/qa/pall-check.ts:41 | g.destr.pallEmit |
| tools/qa/samlook-check.ts:83 | G.g.fogOfWar.hidden |
| tools/qa/silent-probes.ts:455 | game.rescue.lastWarning |
| tools/qa/smokebreak-check.ts:134 | F.hidden |
| tools/qa/snowbreath-check.ts:116 | fog.hidden |
| tools/qa/snowmarks-check.ts:202 | fog.spotSeen |
| tools/qa/spotted-probes.ts:252 | game.rescue.lastWarning |
| tools/qa/standwall.ts:3 | g.col.segmentBlocked |
| tools/qa/wallbreak-probes.ts:329 | game.doors.blast |
| tools/qa/wallshot-check.ts:31 | g.destr.pallEmit |