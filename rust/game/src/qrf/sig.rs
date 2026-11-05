#![allow(unused_variables)]
//! Every function and method of `qrf`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/qrf.ts:118 responseOf)
pub fn response_of(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::ResponseData> { todo!() }

/// (src/game/qrf.ts:121 raid)
pub fn raid(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/qrf.ts:123 raidOver)
pub fn raid_over(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/qrf.ts:125 sideOfTeam)
pub fn side_of_team(g: &mut crate::game::Game) -> crate::qrf::SideOfTeam { todo!() }

/// (src/game/qrf.ts:127 callers)
pub fn callers(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/qrf.ts:134 raidVoice)
pub fn raid_voice(g: &mut crate::game::Game) -> Option<GuardId> { todo!() }

/// (src/game/qrf.ts:136 responseUp)
pub fn response_up(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/qrf.ts:138 responseTeam)
pub fn response_team(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/qrf.ts:139 marksOf)
pub fn marks_of(g: &mut crate::game::Game) -> Option<crate::qrf::Marks> { todo!() }

/// (src/game/qrf.ts:155 roomFor)
pub fn room_for(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/qrf.ts:156 budgetOf)
pub fn budget_of(g: &mut crate::game::Game) -> crate::qrf::BudgetOf { todo!() }

/// (src/game/qrf.ts:158 mostMen)
pub fn most_men(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/qrf.ts:250 Qrf.owns)
pub fn qrf_owns(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/qrf.ts:252 Qrf.summary)
pub fn qrf_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/qrf.ts:263 ownDrillAsked)
pub fn own_drill_asked() -> bool { todo!() }

/// (src/game/qrf.ts:264 storedTune)
pub fn stored_tune() -> Option<JsMap<String, crate::qrf::StoredTuneRow>> { todo!() }

/// (src/game/qrf.ts:265 loadQrfTune)
pub fn load_qrf_tune(g: &mut crate::game::Game) { todo!() }

/// (src/game/qrf.ts:274 saveQrfTune)
pub fn save_qrf_tune(g: &mut crate::game::Game) { todo!() }

/// (src/game/qrf.ts:281 resetQrfTune)
pub fn reset_qrf_tune(g: &mut crate::game::Game) { todo!() }

/// (src/game/qrf.ts:284 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/qrf.ts:285 note)
pub fn note(g: &mut crate::game::Game, what: &str) { todo!() }

/// (src/game/qrf.ts:287 control)
pub fn control(g: &mut crate::game::Game, text: &str) { todo!() }

/// (src/game/qrf.ts:289 controlRow)
pub fn control_row(g: &mut crate::game::Game, text: &str) { todo!() }

/// (src/game/qrf.ts:291 doorSaid)
pub fn door_said(g: &mut crate::game::Game, name: &str) -> String { todo!() }

/// (src/game/qrf.ts:294 radio)
pub fn radio(g: &mut crate::game::Game, gd: Option<GuardId>, text: &str) { todo!() }

/// (src/game/qrf.ts:301 staffVoice)
pub fn staff_voice(g: &mut crate::game::Game) -> Option<GuardId> { todo!() }

/// (src/game/qrf.ts:306 bodiesFound)
pub fn bodies_found(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/qrf.ts:308 live)
pub fn live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/qrf.ts:313 hasResponse)
pub fn has_response(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/qrf.ts:318 responseHere)
pub fn response_here(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/qrf.ts:319 alive)
pub fn alive(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/qrf.ts:321 bestFix)
pub fn best_fix(g: &mut crate::game::Game) -> Option<crate::qrf::BestFix> { todo!() }

/// (src/game/qrf.ts:329 onTheirNet)
pub fn on_their_net(g: &mut crate::game::Game, x: GuardId) -> bool { todo!() }

/// (src/game/qrf.ts:330 netRule)
pub fn net_rule(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/qrf.ts:335 readNet)
pub fn read_net(g: &mut crate::game::Game) { todo!() }

/// (src/game/qrf.ts:350 netWord)
pub fn net_word(g: &mut crate::game::Game) -> Option<crate::qrf::NetWord> { todo!() }

/// (src/game/qrf.ts:357 fixOf)
pub fn fix_of(g: &mut crate::game::Game) -> Option<crate::qrf::FixOf> { todo!() }

/// (src/game/qrf.ts:371 brief)
pub fn brief(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/qrf.ts:380 resetQrf)
pub fn reset_qrf(g: &mut crate::game::Game) { todo!() }

/// (src/game/qrf.ts:387 qrfStandDown)
pub fn qrf_stand_down(g: &mut crate::game::Game) { todo!() }

/// (src/game/qrf.ts:391 requestQrf)
pub fn request_qrf(g: &mut crate::game::Game, reason: crate::qrf::QrfReason) -> bool { todo!() }

/// (src/game/qrf.ts:421 clearOfFoes)
pub fn clear_of_foes(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/qrf.ts:427 responseWords)
pub fn response_words(g: &mut crate::game::Game) -> Option<crate::qrf::ResponseWords> { todo!() }

/// (src/game/qrf.ts:432 dealt)
pub fn dealt(g: &mut crate::game::Game, reason: crate::qrf::QrfReason) -> crate::qrf::Dealt { todo!() }

/// (src/game/qrf.ts:451 silentNet)
pub fn silent_net(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/qrf.ts:465 updateQrf)
pub fn update_qrf(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/qrf.ts:516 arriving)
pub fn arriving(g: &mut crate::game::Game, quiet: bool) { todo!() }

/// (src/game/qrf.ts:545 reDeal)
pub fn re_deal(g: &mut crate::game::Game, quiet: bool) { todo!() }

/// (src/game/qrf.ts:557 deploy)
pub fn deploy(g: &mut crate::game::Game, quiet: bool) { todo!() }

/// (src/game/qrf.ts:568 flingOpen)
pub fn fling_open(g: &mut crate::game::Game, d: DoorId, by: GuardId) { todo!() }

/// (src/game/qrf.ts:576 tickMen)
pub fn tick_men(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/qrf.ts:632 perceiveScripted)
pub fn perceive_scripted(g: &mut crate::game::Game, m: GuardId, dt: f64) -> bool { todo!() }

/// (src/game/qrf.ts:681 manDown)
pub fn man_down(g: &mut crate::game::Game) { todo!() }

/// (src/game/qrf.ts:698 phaseFile)
pub fn phase_file(g: &mut crate::game::Game, el: &mut Vec<GuardId>) { todo!() }

/// (src/game/qrf.ts:712 sectorPoint)
pub fn sector_point(p: V3, yaw: f64, d: Option<f64>, y: Option<f64>) -> V3 { todo!() }

/// (src/game/qrf.ts:717 pickTarget)
pub fn pick_target(g: &mut crate::game::Game, el: &mut Vec<GuardId>, fresh: bool) { todo!() }

/// (src/game/qrf.ts:769 sweepNext)
pub fn sweep_next(g: &mut crate::game::Game, from: V3, fix: V3) -> Option<crate::qrf::Target> { todo!() }

/// (src/game/qrf.ts:780 sweepPlaces)
pub fn sweep_places(g: &mut crate::game::Game) -> Vec<crate::qrf::SweepPlacesItem> { todo!() }

/// (src/game/qrf.ts:790 checkResponse)
pub fn check_response(g: &mut crate::game::Game) -> Vec<String> { todo!() }

/// (src/game/qrf.ts:812 pathLen)
pub fn path_len(g: &mut crate::game::Game, a: V3, b: V3) -> f64 { todo!() }

/// (src/game/qrf.ts:813 nearestFree)
pub fn nearest_free(g: &mut crate::game::Game, p: V3) -> Option<V3> { todo!() }

/// (src/game/qrf.ts:817 approachEnd)
pub fn approach_end(g: &mut crate::game::Game, p: &crate::scene_level::RoomClearPlan) -> V3 { todo!() }

/// (src/game/qrf.ts:822 startMove)
pub fn start_move(g: &mut crate::game::Game, el: &[GuardId]) { todo!() }

/// (src/game/qrf.ts:831 aimOf)
pub fn aim_of<'g>(g: &'g mut crate::game::Game, m: GuardId) -> &'g mut V3 { todo!() }

/// (src/game/qrf.ts:832 stationOf)
pub fn station_of<'g>(g: &'g mut crate::game::Game, m: GuardId) -> &'g mut V3 { todo!() }

/// (src/game/qrf.ts:833 phaseMove)
pub fn phase_move(g: &mut crate::game::Game, el: &mut Vec<GuardId>, dt: f64) { todo!() }

/// (src/game/qrf.ts:880 trailPoint)
pub fn trail_point(trail: &[V3], head: V3, s: f64) -> V3 { todo!() }

/// (src/game/qrf.ts:886 trailDir)
pub fn trail_dir(trail: &[V3], head: V3, s: f64) -> Option<V3> { todo!() }

/// (src/game/qrf.ts:891 hold360)
pub fn hold360(g: &mut crate::game::Game, el: &[GuardId]) { todo!() }

/// (src/game/qrf.ts:899 stackMarksFor)
pub fn stack_marks_for(g: &mut crate::game::Game, p: &crate::scene_level::RoomClearPlan, n: f64) -> Vec<V3> { todo!() }

/// (src/game/qrf.ts:908 startStack)
pub fn start_stack(g: &mut crate::game::Game, el: &[GuardId]) { todo!() }

/// (src/game/qrf.ts:913 phaseStack)
pub fn phase_stack(g: &mut crate::game::Game, el: &mut Vec<GuardId>) { todo!() }

/// (src/game/qrf.ts:924 phaseBreach)
pub fn phase_breach(g: &mut crate::game::Game, el: &mut Vec<GuardId>) { todo!() }

/// (src/game/qrf.ts:961 theirBang)
pub fn their_bang(g: &mut crate::game::Game, at: V3, who: GuardId) { todo!() }

/// (src/game/qrf.ts:966 phaseBang)
pub fn phase_bang(g: &mut crate::game::Game, el: &[GuardId]) { todo!() }

/// (src/game/qrf.ts:983 startFlow)
pub fn start_flow(g: &mut crate::game::Game, el: &[GuardId]) { todo!() }

/// (src/game/qrf.ts:987 flowLegs)
pub fn flow_legs(p: &crate::scene_level::RoomClearPlan, k: f64) -> crate::qrf::FlowLegs { todo!() }

/// (src/game/qrf.ts:994 phaseFlow)
pub fn phase_flow(g: &mut crate::game::Game, el: &mut Vec<GuardId>) { todo!() }

/// (src/game/qrf.ts:1020 startDrill)
pub fn start_drill(g: &mut crate::game::Game, el: &[GuardId]) -> bool { todo!() }

/// (src/game/qrf.ts:1033 dropDrill)
pub fn drop_drill(g: &mut crate::game::Game) { todo!() }

/// (src/game/qrf.ts:1043 phaseDrill)
pub fn phase_drill(g: &mut crate::game::Game, el: &mut Vec<GuardId>) { todo!() }

/// (src/game/qrf.ts:1071 toFight)
pub fn to_fight(g: &mut crate::game::Game, el: &[GuardId], seer: GuardId) { todo!() }

/// (src/game/qrf.ts:1085 phaseFight)
pub fn phase_fight(g: &mut crate::game::Game, el: &mut Vec<GuardId>) { todo!() }

/// (src/game/qrf.ts:1111 route)
pub fn route(g: &mut crate::game::Game, m: GuardId, pts: &[V3], patch: Option<crate::squad::OrderPatch>) { todo!() }

/// (src/game/qrf.ts:1112 advance)
pub fn advance(g: &mut crate::game::Game, m: GuardId) -> bool { todo!() }

/// (src/game/qrf.ts:1115 maybePerimeter)
pub fn maybe_perimeter(g: &mut crate::game::Game) { todo!() }

/// (src/game/qrf.ts:1124 callPerimeter)
pub fn call_perimeter(g: &mut crate::game::Game, why: Option<&str>) -> bool { todo!() }

/// (src/game/qrf.ts:1164 toPost)
pub fn to_post(g: &mut crate::game::Game, p: usize) { todo!() }

/// (src/game/qrf.ts:1168 tickPerimeter)
pub fn tick_perimeter(g: &mut crate::game::Game, dt: f64) { todo!() }
