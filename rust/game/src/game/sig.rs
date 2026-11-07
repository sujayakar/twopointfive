#![allow(unused_variables)]
//! Every function and method of `game`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/game.ts:85 quietMan)
pub fn quiet_man(g: &mut crate::game::Game, pl: Option<PlayerId>) -> bool { todo!() }

/// (src/game/game.ts:291 missionRating)
pub fn mission_rating(s: &crate::game::MissionStats) -> crate::game::MissionRating { todo!() }

/// (src/game/game.ts:292 fmtClock)
pub fn fmt_clock(sec: f64) -> String { todo!() }

/// (src/game/game.ts:293 newMission)
pub fn new_mission(t0: f64) -> crate::game::Mission { todo!() }

/// (src/game/game.ts:335 Player.suppression)
pub fn player_suppression(g: &mut crate::game::Game, id: PlayerId) -> f64 { todo!() }

/// (src/game/game.ts:404 Player.listOpen)
pub fn player_list_open(g: &mut crate::game::Game, id: PlayerId) -> bool { todo!() }

/// (src/game/game.ts:406 Player.choosing)
pub fn player_choosing(g: &mut crate::game::Game, id: PlayerId) -> bool { todo!() }

/// (src/game/game.ts:453 Player.constructor)
pub fn player_new(g: &mut crate::game::Game, men: crate::Opaque, ch: RigId, pos: V3, has_rifle: Option<bool>, seat: Option<f64>) -> PlayerId { todo!() }

/// (src/game/game.ts:461 Player.firearm)
pub fn player_firearm(g: &mut crate::game::Game, id: PlayerId) -> FirearmId { todo!() }

/// (src/game/game.ts:464 Player.gunInHand)
pub fn player_gun_in_hand(g: &mut crate::game::Game, id: PlayerId) -> Option<FirearmId> { todo!() }

/// (src/game/game.ts:466 Player.pistolUp)
pub fn player_pistol_up(g: &mut crate::game::Game, id: PlayerId) -> bool { todo!() }

/// (src/game/game.ts:468 Player.rifleSel)
pub fn player_rifle_sel(g: &mut crate::game::Game, id: PlayerId) -> bool { todo!() }

/// (src/game/game.ts:470 Player.rifleUp)
pub fn player_rifle_up(g: &mut crate::game::Game, id: PlayerId) -> bool { todo!() }

/// (src/game/game.ts:475 Player.braced)
pub fn player_braced(g: &mut crate::game::Game, id: PlayerId) -> bool { todo!() }

/// (src/game/game.ts:477 Player.bracedAt)
pub fn player_braced_at(g: &mut crate::game::Game, id: PlayerId, sprinting: bool) -> bool { todo!() }

/// (src/game/game.ts:481 Player.canisterSel)
pub fn player_canister_sel(g: &mut crate::game::Game, id: PlayerId) -> Option<crate::game::ThrowKind> { todo!() }

/// (src/game/game.ts:533 friendBeforeMuzzle)
pub fn friend_before_muzzle(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/game.ts:539 isBreathing)
pub fn is_breathing(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/game.ts:565 newWitness)
pub fn new_witness() -> crate::game::Witness { todo!() }

/// (src/game/game.ts:574 newTalk)
pub fn new_talk(g: &mut crate::game::Game) -> GameTalkId { todo!() }

/// (src/game/game.ts:740 Guard.trackFix)
pub fn guard_track_fix(g: &mut crate::game::Game, id: GuardId, now: f64) { todo!() }

/// (src/game/game.ts:761 Guard.constructor)
pub fn guard_new(g: &mut crate::game::Game, men: crate::Opaque, ch: RigId, id_2: f64, route: RouteId, route_i: i32, flashlight: LightId, tint: V3, look: Option<&crate::game::GuardNewLook>) -> GuardId { todo!() }

/// (src/game/game.ts:795 Game.rehand)
pub fn game_rehand(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:798 Game.syncSnowLook)
pub fn game_sync_snow_look(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:810 Game.stepSnowTracks)
pub fn game_step_snow_tracks(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/game.ts:847 Game.lightQuery)
pub fn game_light_query(g: &mut crate::game::Game, key: &str, pos: V3, skip_owner: Option<f64>) { todo!() }

/// (src/game/game.ts:848 Game.lightAt)
pub fn game_light_at(g: &mut crate::game::Game, key: &str) -> Option<f64> { todo!() }

/// (src/game/game.ts:850 Game.lightForget)
pub fn game_light_forget(g: &mut crate::game::Game, key: &str) { todo!() }

/// (src/game/game.ts:890 Game.doorSwings)
pub fn game_door_swings(g: &mut crate::game::Game) -> Vec<crate::doorswing::SwingDeal> { todo!() }

/// (src/game/game.ts:934 Game.breakerFixT)
pub fn game_breaker_fix_t(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/game.ts:1082 Game.constructor)
pub fn game_new(g: &mut crate::game::Game, ch: RigId, opts: Option<crate::game::GameNewOpts>) { todo!() }

/// (src/game/game.ts:1267 Game.msg)
pub fn game_msg(g: &mut crate::game::Game, text: &str) { todo!() }

/// (src/game/game.ts:1268 Game.say)
pub fn game_say(g: &mut crate::game::Game, g_2: GuardId, text: &str, radio: Option<bool>, force: Option<bool>, row: Option<bool>) { todo!() }

/// (src/game/game.ts:1270 Game.note)
pub fn game_note(g: &mut crate::game::Game, text: &str) { todo!() }

/// (src/game/game.ts:1272 Game.onDoorEvent)
pub fn game_on_door_event(g: &mut crate::game::Game, e: crate::doors::DoorEvent) { todo!() }

/// (src/game/game.ts:1297 Game.rollTick)
pub fn game_roll_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/game.ts:1313 Game.onPropEvent)
pub fn game_on_prop_event(g: &mut crate::game::Game, e: crate::props::PropEvent) { todo!() }

/// (src/game/game.ts:1327 Game.resetGuards)
pub fn game_reset_guards(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1328 Game.tension)
pub fn game_tension(g: &mut crate::game::Game, theirs: Option<bool>) -> f64 { todo!() }

/// (src/game/game.ts:1330 Game.escalationSummary)
pub fn game_escalation_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1332 Game.escalate)
pub fn game_escalate(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1333 Game.standDown)
pub fn game_stand_down(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1335 Game.clearingSummary)
pub fn game_clearing_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1338 Game.searchSummary)
pub fn game_search_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1345 Game.standoffSummary)
pub fn game_standoff_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1347 Game.bodyLightSummary)
pub fn game_body_light_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1350 Game.suppressionSummary)
pub fn game_suppression_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1351 Game.conditionSummary)
pub fn game_condition_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1354 Game.aimSummary)
pub fn game_aim_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1357 Game.tacticsSummary)
pub fn game_tactics_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1359 Game.needsSummary)
pub fn game_needs_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1361 Game.civiliansSummary)
pub fn game_civilians_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1363 Game.onFloor)
pub fn game_on_floor(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/game.ts:1366 Game.witnessSummary)
pub fn game_witness_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1377 Game.interrogate)
pub fn game_interrogate(g: &mut crate::game::Game, gd: GuardId, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> Option<crate::dialogue::DialoguePick> { todo!() }

/// (src/game/game.ts:1392 Game.freeHeld)
pub fn game_free_held(g: &mut crate::game::Game, why: crate::game::GameFreeHeldWhy) { todo!() }

/// (src/game/game.ts:1394 Game.nearestLivingGuard)
pub fn game_nearest_living_guard(g: &mut crate::game::Game, reach: Option<f64>, foe: Option<bool>) -> Option<GuardId> { todo!() }

/// (src/game/game.ts:1400 Game.interrogateNearest)
pub fn game_interrogate_nearest(g: &mut crate::game::Game, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> Option<crate::dialogue::DialoguePick> { todo!() }

/// (src/game/game.ts:1410 Game.dialogueSummary)
pub fn game_dialogue_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1416 Game.clearQueueSummary)
pub fn game_clear_queue_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1417 Game.clearNearestRoom)
pub fn game_clear_nearest_room(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1419 Game.followPos)
pub fn game_follow_pos(g: &mut crate::game::Game, at: Option<V3>) -> V3 { todo!() }

/// (src/game/game.ts:1420 Game.teleportPlayer)
pub fn game_teleport_player(g: &mut crate::game::Game, p: V3) { todo!() }

/// (src/game/game.ts:1422 Game.floorUnderPlayer)
pub fn game_floor_under_player(g: &mut crate::game::Game) -> crate::floor::FloorMat { todo!() }

/// (src/game/game.ts:1425 Game.bodyThud)
pub fn game_body_thud(g: &mut crate::game::Game, c_2: CharId) { todo!() }

/// (src/game/game.ts:1429 Game.repairLights)
pub fn game_repair_lights(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1432 Game.clearAftermath)
pub fn game_clear_aftermath(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1435 Game.setLoadout)
pub fn game_set_loadout(g: &mut crate::game::Game, with_rifle: bool) { todo!() }

/// (src/game/game.ts:1444 Game.authoredOn)
pub fn game_authored_on(g: &mut crate::game::Game, t: FixtureId) -> bool { todo!() }

/// (src/game/game.ts:1446 Game.standLights)
pub fn game_stand_lights(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1447 Game.resetLevelState)
pub fn game_reset_level_state(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1463 Game.crackTest)
pub fn game_crack_test(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/game.ts:1464 Game.glassSummary)
pub fn game_glass_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1466 Game.wetSummary)
pub fn game_wet_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1468 Game.rainSummary)
pub fn game_rain_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1471 Game.cleanSlate)
pub fn game_clean_slate(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1482 Game.restart)
pub fn game_restart(g: &mut crate::game::Game, said: Option<&str>) { todo!() }

/// (src/game/game.ts:1490 Game.restartEncounter)
pub fn game_restart_encounter(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1520 Game.setGroup)
pub fn game_set_group(g: &mut crate::game::Game, group: &str, on: bool) { todo!() }

/// (src/game/game.ts:1521 Game.toggleTarget)
pub fn game_toggle_target(g: &mut crate::game::Game, t: FixtureId) { todo!() }

/// (src/game/game.ts:1522 Game.setBlackout)
pub fn game_set_blackout(g: &mut crate::game::Game, seconds: f64, cause: crate::game::GameSetBlackoutCause) { todo!() }

/// (src/game/game.ts:1523 Game.endBlackout)
pub fn game_end_blackout(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1524 Game.lightEvent)
pub fn game_light_event(g: &mut crate::game::Game, t: FixtureId, cause: Option<crate::game::GameLightEventCause>, sw_at: Option<V3>) { todo!() }

/// (src/game/game.ts:1525 Game.ocp)
pub fn game_ocp(g: &mut crate::game::Game, t: Option<FixtureId>) { todo!() }

/// (src/game/game.ts:1526 Game.toggleDrag)
pub fn game_toggle_drag(g: &mut crate::game::Game, which: Option<GuardId>) { todo!() }

/// (src/game/game.ts:1532 Game.update)
pub fn game_update(g: &mut crate::game::Game, dt: f64, input: InputId, cam: CameraId, canvas: crate::host_hud::View) { todo!() }

/// (src/game/game.ts:1549 Game.hostBefore)
pub fn game_host_before(g: &mut crate::game::Game, input: InputId) { todo!() }

/// (src/game/game.ts:1552 Game.hostAfter)
pub fn game_host_after(g: &mut crate::game::Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) { todo!() }

/// (src/game/game.ts:1561 Game.step)
pub fn game_step(g: &mut crate::game::Game, dt: f64, cmd: &crate::cmd::PlayerCmd) { todo!() }

/// (src/game/game.ts:1576 Game.hostSeat)
pub fn game_host_seat(g: &mut crate::game::Game) -> SeatId { todo!() }

/// (src/game/game.ts:1580 Game.aside)
pub fn game_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/game.ts:1582 Game.asideLens)
pub fn game_aside_lens(g: &mut crate::game::Game) -> CameraId { todo!() }

/// (src/game/game.ts:1587 Game.stepSeats)
pub fn game_step_seats(g: &mut crate::game::Game, dt: f64, cmds: &[crate::cmd::PlayerCmd]) { todo!() }

/// (src/game/game.ts:1732 Game.threatTick)
pub fn game_threat_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1751 Game.dealWeapons)
pub fn game_deal_weapons(g: &mut crate::game::Game, dt: Option<f64>) { todo!() }

/// (src/game/game.ts:1798 Game.stepFxAndDynBoxes)
pub fn game_step_fx_and_dyn_boxes(g: &mut crate::game::Game, all: &[CharId], dt: f64, cam: CameraId, canvas: crate::host_hud::View) -> bool { todo!() }

/// (src/game/game.ts:1829 Game.smokeWake)
pub fn game_smoke_wake(g: &mut crate::game::Game, all: &[CharId]) { todo!() }

/// (src/game/game.ts:1854 Game.drawCharacters)
pub fn game_draw_characters(g: &mut crate::game::Game, all: &[CharId], statured: bool) { todo!() }

/// (src/game/game.ts:1861 Game.viewPx)
pub fn game_view_px(g: &mut crate::game::Game, cam: crate::game::GameViewPxCam, canvas: crate::game::GameViewPxCanvas) -> f64 { todo!() }

/// (src/game/game.ts:1876 Game.fogLift)
pub fn game_fog_lift(g: &mut crate::game::Game, want: bool, tube: Option<bool>, px: Option<f64>) { todo!() }

/// (src/game/game.ts:1895 Game.fogBack)
pub fn game_fog_back(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1904 Game.drawMen)
pub fn game_draw_men(g: &mut crate::game::Game, all: &[CharId], statured: bool, bare: bool, draw: Option<bool>) { todo!() }

/// (src/game/game.ts:1938 Game.senseStep)
pub fn game_sense_step(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/game.ts:1954 Game.lightMeter)
pub fn game_light_meter(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/game.ts:1972 Game.meterRead)
pub fn game_meter_read(g: &mut crate::game::Game, dt: f64, k: &str) { todo!() }

/// (src/game/game.ts:1985 Game.selectVerb)
pub fn game_select_verb(g: &mut crate::game::Game, kind: crate::game::InteractableKind, verb: Option<crate::game::Verb>) -> bool { todo!() }

/// (src/game/game.ts:1991 Game.escape)
pub fn game_escape(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/game.ts:1993 Game.backable)
pub fn game_backable(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/game.ts:1996 Game.visibilityOf)
pub fn game_visibility_of(g: &mut crate::game::Game, e: f64) -> f64 { todo!() }

/// (src/game/game.ts:1999 Game.updateDoors)
pub fn game_update_doors(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/game.ts:2036 Game.doorContact)
pub fn game_door_contact(g: &mut crate::game::Game) -> Option<crate::doors::DoorContact> { todo!() }

/// (src/game/game.ts:2040 Game.doorActs)
pub fn game_door_acts(g: &mut crate::game::Game, dt: f64, input: InputId) { todo!() }

/// (src/game/game.ts:2112 Game.doorAfter)
pub fn game_door_after(g: &mut crate::game::Game, own: Option<&mut crate::doors::DoorContact>, bx: f64, bz: f64) { todo!() }

/// (src/game/game.ts:2140 Game.handsOnDoors)
pub fn game_hands_on_doors(g: &mut crate::game::Game, before: &[f64]) { todo!() }

/// (src/game/game.ts:2236 Game.updateProps)
pub fn game_update_props(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/game.ts:2250 Game.dropThrowable)
pub fn game_drop_throwable(g: &mut crate::game::Game, kind: crate::game::ThrowKind, p: V3) { todo!() }

/// (src/game/game.ts:2251 Game.throwCount)
pub fn game_throw_count(g: &mut crate::game::Game, kind: crate::game::ThrowKind) -> f64 { todo!() }

/// (src/game/game.ts:2263 Game.objectivePos)
pub fn game_objective_pos(g: &mut crate::game::Game) -> Option<V3> { todo!() }

/// (src/game/game.ts:2264 Game.objectiveText)
pub fn game_objective_text(g: &mut crate::game::Game) -> String { todo!() }


/// INVENTED: THE whole write of a man's path. Every `gd.path = x` of the TypeScript (a new array, even `[]`) is this call: identity readers (doorpath.ts:336-337, doorway.ts:328, guards.ts:2037) compare path_serial. An in-place edit (splice, push, index, `length = n`) is not a whole write and does not call it. ONE exception: runner.ts:294 puts the same array back, so it restores path and path_serial together and does not call this
pub fn guard_set_path(g: &mut crate::game::Game, gd: GuardId, path: Vec<V3>) { let o = &mut g.gd[gd]; o.path = path; o.path_serial = o.path_serial.wrapping_add(1); }
