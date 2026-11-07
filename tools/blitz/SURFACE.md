# The call surface

Written by surface.py. 1089 rows of `io::call`, 797 with a guard at load. What is not on it, by why:

- **the host's own, or the host already answers it** (104): `BoxMirror.followLeaf`, `BoxMirror.followRow`, `BoxMirror.gridDrop`, `BoxMirror.open`, `BoxWorld.addCharacterCapsules`, `BoxWorld.addStatic`, `BoxWorld.rayAside`, `BoxWorld.raycast`, `BoxWorld.relist`, `BoxWorld.repackMarkings`, `BoxWorld.resetCapsules`, `CharacterRenderer.finish`, `CharacterRenderer.setInstance`, `CharacterRenderer.setMaterials`, `Checkpoints.load`, `Checkpoints.loadInStep`, `Checkpoints.tick`, `Checkpoints.write`, `CpuSmoke.along`, `CpuSmoke.blobsAlong`, `CpuSmoke.clear`, `CpuSmoke.emit`, `CpuSmoke.step`, `FollowCamera.autoYaw`, `FollowCamera.drawIn`, `FollowCamera.drawOut`, `FollowCamera.easeYaw`, `FollowCamera.frame`, `FollowCamera.kickIn`, `FollowCamera.kickOut`, `FollowCamera.rebuild`, `FollowCamera.rotate`, `FollowCamera.setPin`, `FollowCamera.step`, `FollowCamera.update`, `FollowCamera.zoom`, `Game.followPos`, `Game.msg`, `Game.step`, `Game.stepSeats`, `Game.update`, `Input.detached`, `Input.endFrame`, `Input.endTick`, `NavGrid.findPath`, `NavGrid.nearestFreePoint`, `NavGrid.rebake`, `PoseFK.run`, `Seat.hand`, `Seat.handed`, `Seat.load`, `Seat.swallow`, `Seat.takeCut`, `Sensors.ask`, `Sensors.askSegment`, `Sensors.beginFrame`, `Sensors.endFrame`, `Sensors.forget`, `Sensors.lightAt`, `Sensors.live`, `Sensors.read`, `Sensors.record`, `Sensors.replay`, `Sensors.segment`, `SessionRecorder.cut`, `SessionRecorder.frameEnd`, `SessionRecorder.frameStart`, `SkeletonInstance.update`, `SmokeGrid.clear`, `SmokeGrid.emit` and more
- **no Rust function** (95): `Attract.poke`, `Attract.stop`, `Attract.tick`, `Attract.update`, `BoxWorld.ensurePacked`, `BoxWorld.forgetSent`, `BoxWorld.markRecordsOf`, `BoxWorld.resetMarkTable`, `BoxWorld.setStuffOf`, `BoxWorld.upload`, `CharacterRenderer.drawRuns`, `CharacterRenderer.forgetSent`, `Checkpoints.importText`, `Checkpoints.offer`, `Demo.goto`, `Demo.jump`, `Demo.start`, `Demo.stop`, `Demo.update`, `Doors.onTorn`, `Doors.updateBoth`, `PoseFK.parentRot`, `PoseFK.runFrom`, `PropSystem.updateBoth`, `ReplayBuffer.top`, `RunTapeRec.top`, `SessionPlayer.applyFrame`, `SessionRecorder.stop`, `SmokeGrid.free`, `applyKnobs`, `attentionWay`, `blastFeelStages`, `buildBundledLevel`, `buildLevel`, `calmTorchRestore`, `castWear`, `chainBits`, `darkFoeRestore`, `doing`, `dryAll`, `engage`, `extraSpots`, `floorPlan`, `holdFireRestore`, `hudExpireBubbles`, `inRain`, `initCutSwitch`, `initDifficulty`, `initHour`, `kitAddLaid`, `kitLoad`, `kitSave`, `lanesRestore`, `layRestore`, `levelToData`, `loadCharacterSet`, `loadGltfCast`, `loadGltfCharacter`, `loadLevel`, `manLeaves`, `moraleRestore`, `namePaint`, `namePaints`, `netQuietRestore`, `ownSmoke`, `pictureRestore`, `plainLevel`, `powerOutRestore`, `rainDripPoints`, `recipeOf` and more
- **takes a closure** (36): `Debris.impulse`, `Defuse.samRows`, `Destruction.withWallsOpen`, `Door.posAside`, `DoorTools.samRows`, `Doors.posAside`, `EntryPoints.clearOf`, `Game.aside`, `Game.interrogate`, `Game.interrogateNearest`, `GlassBreak.asking`, `GlassBreak.seeThrough`, `RescueEnd.rows`, `SnatchOut.rows`, `SquadOrders.askAside`, `SquadOrders.noRoomWords`, `WallCharge.clearOf`, `WallCharge.samRows`, `WorkThings.askAside`, `coverNear`, `hearWhistle`, `holdFireHolds`, `interrogate`, `leversFog`, `lightEstimateAt`, `ownerWitness`, `pickLine`, `pickSamLine`, `pictureQuiet`, `pictureStep`, `quietFogAside`, `skyAside`, `sneakWalk`, `stickyRows`, `switchRows`, `takeKey`
- **no body yet** (15): 
- **takes no game: nothing to write** (7): `AimState.clear`, `Ocp.update`, `Ocp.use`, `RateCap.reset`, `RateCap.take`, `gpFragments`, `manifestOf`
- **answers what its Rust function does not** (5): `Destruction.onBlast`, `Pose.copy`, `WallBreak.onChange`, `searchPlanFor`, `wallReactOf`
- **a parameter only Rust has** (4): `blendPose`, `layerSlot`, `sampleClip`, `spawnShot`
- **takes an opaque value** (2): `initWeather`, `quietFogStart`

## Called with no game stood (the census's kind iv-b)

**Stays TypeScript after the deletion**: a reader with no Rust function. A file marked "with tick code" holds a kept reader and code the deletion takes, so it must be split or the reader moved out.

| function | file | |
|---|---|---|
| knobsBack | src/game/knobs.ts |  |
| knobsCopy | src/game/knobs.ts |  |
| isOfficeFamily | src/scene/level.ts | with tick code |
| withOfficeScript | src/scene/level.ts | with tick code |
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