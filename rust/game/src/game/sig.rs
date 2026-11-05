#![allow(unused_variables)]
//! Every function and method of `game`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/game.ts:84 quietMan)
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

/// (src/game/game.ts:731 Guard.trackFix)
pub fn guard_track_fix(g: &mut crate::game::Game, id: GuardId, now: f64) { todo!() }

/// (src/game/game.ts:752 Guard.constructor)
pub fn guard_new(g: &mut crate::game::Game, men: crate::Opaque, ch: RigId, id_2: f64, route: RouteId, route_i: f64, flashlight: LightId, tint: V3, look: Option<&crate::game::GuardNewLook>) -> GuardId { todo!() }

/// (src/game/game.ts:786 Game.rehand)
pub fn game_rehand(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:789 Game.syncSnowLook)
pub fn game_sync_snow_look(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:801 Game.stepSnowTracks)
pub fn game_step_snow_tracks(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/game.ts:838 Game.lightQuery)
pub fn game_light_query(g: &mut crate::game::Game, key: &str, pos: V3, skip_owner: Option<f64>) { todo!() }

/// (src/game/game.ts:839 Game.lightAt)
pub fn game_light_at(g: &mut crate::game::Game, key: &str) -> Option<f64> { todo!() }

/// (src/game/game.ts:841 Game.lightForget)
pub fn game_light_forget(g: &mut crate::game::Game, key: &str) { todo!() }

/// (src/game/game.ts:881 Game.doorSwings)
pub fn game_door_swings(g: &mut crate::game::Game) -> Vec<crate::doorswing::SwingDeal> { todo!() }

/// (src/game/game.ts:901 Game.reachChanged)
pub fn game_reach_changed(g: &mut crate::game::Game, rect: Option<[f64; 4]>) -> f64 { todo!() }

/// (src/game/game.ts:909 Game.reachRead)
pub fn game_reach_read(g: &mut crate::game::Game, rects: Option<&[[f64; 4]]>) -> f64 { todo!() }

/// (src/game/game.ts:943 Game.reachTick)
pub fn game_reach_tick(g: &mut crate::game::Game, n: Option<f64>, steps: Option<f64>) -> f64 { todo!() }

/// (src/game/game.ts:962 Game.reachPending)
pub fn game_reach_pending(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/game.ts:965 Game.buildDirReach)
pub fn game_build_dir_reach(g: &mut crate::game::Game, l: LightId, only: Option<&crate::render_reach::ReachOnly>) { todo!() }

/// (src/game/game.ts:1051 Game.blackout)
pub fn game_blackout(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1053 Game.breakerFixT)
pub fn game_breaker_fix_t(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/game.ts:1201 Game.constructor)
pub fn game_new(g: &mut crate::game::Game, ch: RigId, opts: Option<crate::game::GameNewOpts>) -> crate::game::Game { todo!() }

/// (src/game/game.ts:1397 Game.msg)
pub fn game_msg(g: &mut crate::game::Game, text: &str) { todo!() }

/// (src/game/game.ts:1398 Game.say)
pub fn game_say(g: &mut crate::game::Game, g_2: GuardId, text: &str, radio: Option<bool>, force: Option<bool>, row: Option<bool>) { todo!() }

/// (src/game/game.ts:1400 Game.note)
pub fn game_note(g: &mut crate::game::Game, text: &str) { todo!() }

/// (src/game/game.ts:1402 Game.onDoorEvent)
pub fn game_on_door_event(g: &mut crate::game::Game, e: crate::doors::DoorEvent) { todo!() }

/// (src/game/game.ts:1427 Game.rollTick)
pub fn game_roll_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/game.ts:1443 Game.onPropEvent)
pub fn game_on_prop_event(g: &mut crate::game::Game, e: crate::props::PropEvent) { todo!() }

/// (src/game/game.ts:1457 Game.resetGuards)
pub fn game_reset_guards(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1458 Game.tension)
pub fn game_tension(g: &mut crate::game::Game, theirs: Option<bool>) -> f64 { todo!() }

/// (src/game/game.ts:1460 Game.escalationSummary)
pub fn game_escalation_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1462 Game.escalate)
pub fn game_escalate(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1463 Game.standDown)
pub fn game_stand_down(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1465 Game.clearingSummary)
pub fn game_clearing_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1468 Game.searchSummary)
pub fn game_search_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1475 Game.standoffSummary)
pub fn game_standoff_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1477 Game.bodyLightSummary)
pub fn game_body_light_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1480 Game.suppressionSummary)
pub fn game_suppression_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1481 Game.conditionSummary)
pub fn game_condition_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1484 Game.aimSummary)
pub fn game_aim_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1487 Game.tacticsSummary)
pub fn game_tactics_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1489 Game.needsSummary)
pub fn game_needs_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1491 Game.civiliansSummary)
pub fn game_civilians_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1493 Game.onFloor)
pub fn game_on_floor(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/game.ts:1496 Game.witnessSummary)
pub fn game_witness_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1507 Game.interrogate)
pub fn game_interrogate(g: &mut crate::game::Game, gd: GuardId, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> Option<crate::dialogue::DialoguePick> { todo!() }

/// (src/game/game.ts:1522 Game.freeHeld)
pub fn game_free_held(g: &mut crate::game::Game, why: crate::game::GameFreeHeldWhy) { todo!() }

/// (src/game/game.ts:1524 Game.nearestLivingGuard)
pub fn game_nearest_living_guard(g: &mut crate::game::Game, reach: Option<f64>, foe: Option<bool>) -> Option<GuardId> { todo!() }

/// (src/game/game.ts:1530 Game.interrogateNearest)
pub fn game_interrogate_nearest(g: &mut crate::game::Game, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> Option<crate::dialogue::DialoguePick> { todo!() }

/// (src/game/game.ts:1540 Game.dialogueSummary)
pub fn game_dialogue_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1546 Game.clearQueueSummary)
pub fn game_clear_queue_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1547 Game.clearNearestRoom)
pub fn game_clear_nearest_room(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1549 Game.followPos)
pub fn game_follow_pos(g: &mut crate::game::Game, at: Option<V3>) -> V3 { todo!() }

/// (src/game/game.ts:1550 Game.teleportPlayer)
pub fn game_teleport_player(g: &mut crate::game::Game, p: V3) { todo!() }

/// (src/game/game.ts:1552 Game.floorUnderPlayer)
pub fn game_floor_under_player(g: &mut crate::game::Game) -> crate::floor::FloorMat { todo!() }

/// (src/game/game.ts:1555 Game.bodyThud)
pub fn game_body_thud(g: &mut crate::game::Game, c_2: CharId) { todo!() }

/// (src/game/game.ts:1559 Game.repairLights)
pub fn game_repair_lights(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1562 Game.clearAftermath)
pub fn game_clear_aftermath(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1565 Game.setLoadout)
pub fn game_set_loadout(g: &mut crate::game::Game, with_rifle: bool) { todo!() }

/// (src/game/game.ts:1574 Game.authoredOn)
pub fn game_authored_on(g: &mut crate::game::Game, t: FixtureId) -> bool { todo!() }

/// (src/game/game.ts:1575 Game.resetLevelState)
pub fn game_reset_level_state(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1591 Game.crackTest)
pub fn game_crack_test(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/game.ts:1592 Game.glassSummary)
pub fn game_glass_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1594 Game.wetSummary)
pub fn game_wet_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1596 Game.rainSummary)
pub fn game_rain_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/game.ts:1599 Game.cleanSlate)
pub fn game_clean_slate(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1610 Game.restart)
pub fn game_restart(g: &mut crate::game::Game, said: Option<&str>) { todo!() }

/// (src/game/game.ts:1618 Game.restartEncounter)
pub fn game_restart_encounter(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1648 Game.setGroup)
pub fn game_set_group(g: &mut crate::game::Game, group: &str, on: bool) { todo!() }

/// (src/game/game.ts:1649 Game.toggleTarget)
pub fn game_toggle_target(g: &mut crate::game::Game, t: FixtureId) { todo!() }

/// (src/game/game.ts:1650 Game.setBlackout)
pub fn game_set_blackout(g: &mut crate::game::Game, seconds: f64, cause: crate::game::GameSetBlackoutCause) { todo!() }

/// (src/game/game.ts:1651 Game.endBlackout)
pub fn game_end_blackout(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1652 Game.lightEvent)
pub fn game_light_event(g: &mut crate::game::Game, t: FixtureId, cause: Option<crate::game::GameLightEventCause>, sw_at: Option<V3>) { todo!() }

/// (src/game/game.ts:1653 Game.ocp)
pub fn game_ocp(g: &mut crate::game::Game, t: Option<FixtureId>) { todo!() }

/// (src/game/game.ts:1654 Game.toggleDrag)
pub fn game_toggle_drag(g: &mut crate::game::Game, which: Option<GuardId>) { todo!() }

/// (src/game/game.ts:1660 Game.update)
pub fn game_update(g: &mut crate::game::Game, dt: f64, input: InputId, canvas: crate::host_hud::View) { todo!() }

/// (src/game/game.ts:1677 Game.hostBefore)
pub fn game_host_before(g: &mut crate::game::Game, input: InputId) { todo!() }

/// (src/game/game.ts:1680 Game.hostAfter)
pub fn game_host_after(g: &mut crate::game::Game, input: InputId, canvas: crate::host_hud::View) { todo!() }

/// (src/game/game.ts:1689 Game.step)
pub fn game_step(g: &mut crate::game::Game, dt: f64, cmd: &crate::cmd::PlayerCmd) { todo!() }

/// (src/game/game.ts:1704 Game.hostSeat)
pub fn game_host_seat(g: &mut crate::game::Game) -> SeatId { todo!() }

/// (src/game/game.ts:1708 Game.aside)
pub fn game_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/game.ts:1710 Game.asideLens)
pub fn game_aside_lens(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1715 Game.stepSeats)
pub fn game_step_seats(g: &mut crate::game::Game, dt: f64, cmds: &[crate::cmd::PlayerCmd]) { todo!() }

/// (src/game/game.ts:1859 Game.threatTick)
pub fn game_threat_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:1878 Game.dealWeapons)
pub fn game_deal_weapons(g: &mut crate::game::Game, dt: Option<f64>) { todo!() }

/// (src/game/game.ts:1925 Game.stepFxAndDynBoxes)
pub fn game_step_fx_and_dyn_boxes(g: &mut crate::game::Game, all: &[CharId], dt: f64, canvas: crate::host_hud::View) -> bool { todo!() }

/// (src/game/game.ts:1956 Game.smokeWake)
pub fn game_smoke_wake(g: &mut crate::game::Game, all: &[CharId]) { todo!() }

/// (src/game/game.ts:1981 Game.drawCharacters)
pub fn game_draw_characters(g: &mut crate::game::Game, all: &[CharId], statured: bool) { todo!() }

/// (src/game/game.ts:1988 Game.viewPx)
pub fn game_view_px(g: &mut crate::game::Game, cam: crate::game::GameViewPxCam, canvas: crate::game::GameViewPxCanvas) -> f64 { todo!() }

/// (src/game/game.ts:2003 Game.fogLift)
pub fn game_fog_lift(g: &mut crate::game::Game, want: bool, tube: Option<bool>, px: Option<f64>) { todo!() }

/// (src/game/game.ts:2022 Game.fogBack)
pub fn game_fog_back(g: &mut crate::game::Game) { todo!() }

/// (src/game/game.ts:2031 Game.drawMen)
pub fn game_draw_men(g: &mut crate::game::Game, all: &[CharId], statured: bool, bare: bool, draw: Option<bool>) { todo!() }

/// (src/game/game.ts:2065 Game.senseStep)
pub fn game_sense_step(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/game.ts:2081 Game.lightMeter)
pub fn game_light_meter(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/game.ts:2099 Game.meterRead)
pub fn game_meter_read(g: &mut crate::game::Game, dt: f64, k: &str) { todo!() }

/// (src/game/game.ts:2112 Game.selectVerb)
pub fn game_select_verb(g: &mut crate::game::Game, kind: crate::game::GameSelectVerbKind, verb: Option<crate::game::Verb>) -> bool { todo!() }

/// (src/game/game.ts:2118 Game.escape)
pub fn game_escape(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/game.ts:2120 Game.backable)
pub fn game_backable(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/game.ts:2123 Game.visibilityOf)
pub fn game_visibility_of(g: &mut crate::game::Game, e: f64) -> f64 { todo!() }

/// (src/game/game.ts:2126 Game.updateDoors)
pub fn game_update_doors(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/game.ts:2163 Game.doorContact)
pub fn game_door_contact(g: &mut crate::game::Game) -> Option<crate::doors::DoorContact> { todo!() }

/// (src/game/game.ts:2167 Game.doorActs)
pub fn game_door_acts(g: &mut crate::game::Game, dt: f64, input: InputId) { todo!() }

/// (src/game/game.ts:2239 Game.doorAfter)
pub fn game_door_after(g: &mut crate::game::Game, own: Option<&mut crate::doors::DoorContact>, bx: f64, bz: f64) { todo!() }

/// (src/game/game.ts:2267 Game.handsOnDoors)
pub fn game_hands_on_doors(g: &mut crate::game::Game, before: &[f64]) { todo!() }

/// (src/game/game.ts:2363 Game.updateProps)
pub fn game_update_props(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/game.ts:2377 Game.dropThrowable)
pub fn game_drop_throwable(g: &mut crate::game::Game, kind: crate::game::ThrowKind, p: V3) { todo!() }

/// (src/game/game.ts:2378 Game.throwCount)
pub fn game_throw_count(g: &mut crate::game::Game, kind: crate::game::ThrowKind) -> f64 { todo!() }

/// (src/game/game.ts:2390 Game.objectivePos)
pub fn game_objective_pos(g: &mut crate::game::Game) -> Option<V3> { todo!() }

/// (src/game/game.ts:2391 Game.objectiveText)
pub fn game_objective_text(g: &mut crate::game::Game) -> String { todo!() }
