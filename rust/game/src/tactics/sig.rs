#![allow(unused_variables)]
//! Every function and method of `tactics`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/tactics.ts:67 loadTactics)
pub fn load_tactics(tune: &mut crate::tactics::TacticsTune) { todo!() }

/// (src/game/tactics.ts:74 saveTactics)
pub fn save_tactics(tune: crate::tactics::TacticsTune) { todo!() }

/// (src/game/tactics.ts:78 resetTactics)
pub fn reset_tactics(tune: &mut crate::tactics::TacticsTune) { todo!() }

/// (src/game/tactics.ts:149 S)
pub fn s<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::tactics::TState { todo!() }

/// (src/game/tactics.ts:156 giveStuns)
pub fn give_stuns(g: &mut crate::game::Game, gd: GuardId, n: f64) { todo!() }

/// (src/game/tactics.ts:157 stunsOf)
pub fn stuns_of(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/tactics.ts:165 tacticsOn)
pub fn tactics_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/tactics.ts:166 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/tactics.ts:167 bearingOf)
pub fn bearing_of(p: V3, c_2: V3) -> f64 { todo!() }

/// (src/game/tactics.ts:168 degApart)
pub fn deg_apart(a: f64, b: f64) -> f64 { todo!() }

/// (src/game/tactics.ts:170 freeMan)
pub fn free_man(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/tactics.ts:172 inContact)
pub fn in_contact(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/tactics.ts:174 suppressed)
pub fn suppressed(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/tactics.ts:177 pathLen)
pub fn path_len(from: V3, path: &[V3]) -> f64 { todo!() }

/// (src/game/tactics.ts:179 samplePath)
pub fn sample_path(from: V3, path: &[V3], step: Option<f64>) -> Vec<V3> { todo!() }

/// (src/game/tactics.ts:191 cellsOfPath)
pub fn cells_of_path(g: &mut crate::game::Game, from: V3, path: &[V3], dilate: f64) -> JsSet<crate::Num> { todo!() }

/// (src/game/tactics.ts:200 sectorCells)
pub fn sector_cells(g: &mut crate::game::Game, c_2: V3, bearing: f64, r: f64, half_deg: f64) -> JsSet<crate::Num> { todo!() }

/// (src/game/tactics.ts:211 entryBearing)
pub fn entry_bearing(t: V3, from: V3, path: &[V3], r: f64) -> f64 { todo!() }

/// (src/game/tactics.ts:221 losOpen)
pub fn los_open(g: &mut crate::game::Game, p: V3, t: V3, team: crate::teams::Team) -> bool { todo!() }

/// (src/game/tactics.ts:225 lastCoverPoint)
pub fn last_cover_point(g: &mut crate::game::Game, from: V3, path: &[V3], t: V3, min_from_t: f64, team: crate::teams::Team) -> Option<V3> { todo!() }

/// (src/game/tactics.ts:231 pointShortOfEnd)
pub fn point_short_of_end(from: V3, path: &[V3], rem: f64) -> V3 { todo!() }

/// (src/game/tactics.ts:237 wallsAbout)
pub fn walls_about(g: &mut crate::game::Game, p: V3) -> f64 { todo!() }

/// (src/game/tactics.ts:243 doorNear)
pub fn door_near(g: &mut crate::game::Game, p: V3, r: Option<f64>) -> Option<DoorId> { todo!() }

/// (src/game/tactics.ts:249 doorLabel)
pub fn door_label(g: &mut crate::game::Game, d: DoorId) -> String { todo!() }

/// (src/game/tactics.ts:258 dangerDoors)
pub fn danger_doors(g: &mut crate::game::Game, team: Option<crate::teams::Team>) -> Vec<TacticsDangerRecId> { todo!() }

/// (src/game/tactics.ts:260 dangerAt)
pub fn danger_at(g: &mut crate::game::Game, d: DoorId, team: crate::teams::Team) -> Option<TacticsDangerRecId> { todo!() }

/// (src/game/tactics.ts:262 markDanger)
pub fn mark_danger(g: &mut crate::game::Game, d: DoorId, team: crate::teams::Team, why: crate::tactics::DangerWhy, victim: Option<&str>) -> TacticsDangerRecId { todo!() }

/// (src/game/tactics.ts:283 tacticsReset)
pub fn tactics_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/tactics.ts:293 clearedFor)
pub fn cleared_for(g: &mut crate::game::Game, gd: GuardId, d: DoorId) -> bool { todo!() }

/// (src/game/tactics.ts:295 tacticsCleared)
pub fn tactics_cleared(g: &mut crate::game::Game, gd: GuardId, d: DoorId) -> bool { todo!() }

/// (src/game/tactics.ts:296 setCleared)
pub fn set_cleared(g: &mut crate::game::Game, gd: GuardId, d: DoorId, until: f64) { todo!() }

/// (src/game/tactics.ts:300 tacticsManDown)
pub fn tactics_man_down(g: &mut crate::game::Game, victim: GuardId) { todo!() }

/// (src/game/tactics.ts:308 tacticsBodyFound)
pub fn tactics_body_found(g: &mut crate::game::Game, finder: GuardId, body: GuardId) { todo!() }

/// (src/game/tactics.ts:314 bodyDutyBlocked)
pub fn body_duty_blocked(g: &mut crate::game::Game, gd: GuardId, body: GuardId) -> bool { todo!() }

/// (src/game/tactics.ts:320 scanRounds)
pub fn scan_rounds(g: &mut crate::game::Game) { todo!() }

/// (src/game/tactics.ts:336 distToSegment)
pub fn dist_to_segment(p: V3, a: V3, b: V3) -> f64 { todo!() }

/// (src/game/tactics.ts:347 tacticsPenalty)
pub fn tactics_penalty(g: &mut crate::game::Game, gd: GuardId, goal: V3) -> Option<crate::tactics::TacticsPenalty> { todo!() }

/// (src/game/tactics.ts:365 tacticsRoleOf)
pub fn tactics_role_of(g: &mut crate::game::Game, gd: GuardId) -> Option<String> { todo!() }

/// (src/game/tactics.ts:372 roleRecord)
pub fn role_record(g: &mut crate::game::Game, gd: GuardId) -> Option<TacticsRoleId> { todo!() }

/// (src/game/tactics.ts:373 loneRecord)
pub fn lone_record(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::tactics::Lone> { todo!() }

/// (src/game/tactics.ts:374 gateRecord)
pub fn gate_record(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::tactics::Gate> { todo!() }

/// (src/game/tactics.ts:375 plansOf)
pub fn plans_of(g: &mut crate::game::Game) -> Vec<TacticsPlanId> { todo!() }

/// (src/game/tactics.ts:383 flankCall)
pub fn flank_call(g: &mut crate::game::Game, plan: TacticsPlanId, m: GuardId, place: &str) -> String { todo!() }

/// (src/game/tactics.ts:391 planApproach)
pub fn plan_approach(g: &mut crate::game::Game, men: &[GuardId], target: V3, why: Option<&str>, prefer: Option<GuardId>) -> Option<TacticsPlanId> { todo!() }

/// (src/game/tactics.ts:438 endPlan)
pub fn end_plan(g: &mut crate::game::Game, plan: TacticsPlanId, why: &str, reset_men: Option<bool>) { todo!() }

/// (src/game/tactics.ts:446 fitForRole)
pub fn fit_for_role(g: &mut crate::game::Game, m: GuardId) -> bool { todo!() }

/// (src/game/tactics.ts:448 tickPlans)
pub fn tick_plans(g: &mut crate::game::Game) { todo!() }

/// (src/game/tactics.ts:498 advance)
pub fn advance(g: &mut crate::game::Game, gd: GuardId, dt: f64, goal: V3, pace: f64, out: &mut crate::tactics::TacMove) -> bool { todo!() }

/// (src/game/tactics.ts:506 kneelToward)
pub fn kneel_toward(g: &mut crate::game::Game, gd: GuardId, dt: f64, at: V3, out: &mut crate::tactics::TacMove, crouch: Option<f64>) { todo!() }

/// (src/game/tactics.ts:512 readyToGo)
pub fn ready_to_go(g: &mut crate::game::Game, plan: TacticsPlanId) -> bool { todo!() }

/// (src/game/tactics.ts:523 flankersRunning)
pub fn flankers_running(g: &mut crate::game::Game, plan: TacticsPlanId) -> bool { todo!() }

/// (src/game/tactics.ts:527 goNow)
pub fn go_now(g: &mut crate::game::Game, plan: TacticsPlanId, caller: GuardId, why: &str) { todo!() }

/// (src/game/tactics.ts:535 arrive)
pub fn arrive(g: &mut crate::game::Game, gd: GuardId, role: TacticsRoleId) { todo!() }

/// (src/game/tactics.ts:543 moveRole)
pub fn move_role(g: &mut crate::game::Game, gd: GuardId, role: TacticsRoleId, dt: f64) -> crate::tactics::TacMove { todo!() }

/// (src/game/tactics.ts:591 presserOf)
pub fn presser_of(g: &mut crate::game::Game, plan: TacticsPlanId) -> Option<GuardId> { todo!() }

/// (src/game/tactics.ts:592 roleOfMan)
pub fn role_of_man(g: &mut crate::game::Game, gd: GuardId) -> Option<TacticsRoleId> { todo!() }

/// (src/game/tactics.ts:597 armedIntruder)
pub fn armed_intruder(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/tactics.ts:605 alone)
pub fn alone(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/tactics.ts:610 loneCorner)
pub fn lone_corner(g: &mut crate::game::Game, gd: GuardId, fix: V3) -> crate::tactics::LoneCorner { todo!() }

/// (src/game/tactics.ts:632 loneCall)
pub fn lone_call(g: &mut crate::game::Game, gd: GuardId, l: &mut crate::tactics::Lone) { todo!() }

/// (src/game/tactics.ts:646 moveLone)
pub fn move_lone(g: &mut crate::game::Game, gd: GuardId, l: &mut crate::tactics::Lone, dt: f64) -> Option<crate::tactics::TacMove> { todo!() }

/// (src/game/tactics.ts:668 tacticsAlert)
pub fn tactics_alert(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> Option<crate::tactics::TacMove> { todo!() }

/// (src/game/tactics.ts:688 tacticsTick)
pub fn tactics_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/tactics.ts:704 sideOf)
pub fn side_of(g: &mut crate::game::Game, d: DoorId, p: V3) -> f64 { todo!() }

/// (src/game/tactics.ts:705 alongOf)
pub fn along_of(g: &mut crate::game::Game, d: DoorId, p: V3) -> f64 { todo!() }

/// (src/game/tactics.ts:707 doorPt)
pub fn door_pt(g: &mut crate::game::Game, d: DoorId, a: f64, b: f64, sg: f64, y: Option<f64>) -> V3 { todo!() }

/// (src/game/tactics.ts:709 planFor)
pub fn plan_for(g: &mut crate::game::Game, d: DoorId, sg: f64, near_a: f64) -> Option<crate::scene_level::RoomClearPlan> { todo!() }

/// (src/game/tactics.ts:720 stackSpot)
pub fn stack_spot(g: &mut crate::game::Game, d: DoorId, sg: f64, near_a: f64, k: f64, plan: Option<&crate::scene_level::RoomClearPlan>) -> crate::tactics::StackSpot { todo!() }

/// (src/game/tactics.ts:734 sliceStations)
pub fn slice_stations(g: &mut crate::game::Game, d: DoorId, sg: f64, near_a: f64) -> Option<crate::tactics::SliceStations> { todo!() }

/// (src/game/tactics.ts:747 entryLegs)
pub fn entry_legs(g: &mut crate::game::Game, d: DoorId, sg: f64, near_a: f64, k: f64, plan: Option<&crate::scene_level::RoomClearPlan>) -> Vec<V3> { todo!() }

/// (src/game/tactics.ts:761 stepGate)
pub fn step_gate(g: &mut crate::game::Game, gd: GuardId, g_2: &mut crate::tactics::Gate, dt: f64, target: V3, speed: f64, face: Option<f64>) -> bool { todo!() }

/// (src/game/tactics.ts:776 standGate)
pub fn stand_gate(g: &mut crate::game::Game, gd: GuardId, dt: f64, face: Option<f64>) { todo!() }

/// (src/game/tactics.ts:778 gateMate)
pub fn gate_mate(g: &mut crate::game::Game, g_2: &crate::tactics::Gate, role: crate::tactics::GateMateRole, except: GuardId) -> Option<crate::tactics::GateMate> { todo!() }

/// (src/game/tactics.ts:784 gateCall)
pub fn gate_call(g: &mut crate::game::Game, gd: GuardId, g_2: &mut crate::tactics::Gate) { todo!() }

/// (src/game/tactics.ts:816 tryEngage)
pub fn try_engage(g: &mut crate::game::Game, gd: GuardId, recs: &[TacticsDangerRecId]) -> Option<crate::tactics::Gate> { todo!() }

/// (src/game/tactics.ts:852 doorGate)
pub fn door_gate(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64) -> bool { todo!() }

/// (src/game/tactics.ts:864 looseCan)
pub fn loose_can(g: &mut crate::game::Game, g_2: &mut crate::tactics::Gate) { todo!() }

/// (src/game/tactics.ts:865 releaseGate)
pub fn release_gate(g: &mut crate::game::Game, gd: GuardId, g_2: &mut crate::tactics::Gate, why: &str) { todo!() }

/// (src/game/tactics.ts:877 runGate)
pub fn run_gate(g: &mut crate::game::Game, gd: GuardId, g_2: &mut crate::tactics::Gate, dt: f64) -> bool { todo!() }

/// (src/game/tactics.ts:988 awayLook)
pub fn away_look(g: &mut crate::game::Game, d: DoorId, g_2: &crate::tactics::Gate, pos: V3) -> V3 { todo!() }

/// (src/game/tactics.ts:989 beginSlice)
pub fn begin_slice(g: &mut crate::game::Game, gd: GuardId, g_2: &mut crate::tactics::Gate) { todo!() }

/// (src/game/tactics.ts:994 beginCross)
pub fn begin_cross(g: &mut crate::game::Game, gd: GuardId, g_2: &mut crate::tactics::Gate) { todo!() }

/// (src/game/tactics.ts:1000 beginToss)
pub fn begin_toss(g: &mut crate::game::Game, gd: GuardId, g_2: &mut crate::tactics::Gate, thrower: GuardId) { todo!() }

/// (src/game/tactics.ts:1014 tickGates)
pub fn tick_gates(g: &mut crate::game::Game) { todo!() }

/// (src/game/tactics.ts:1020 tacticsLook)
pub fn tactics_look(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::tactics::TacticsLook> { todo!() }

/// (src/game/tactics.ts:1028 tacticsSummary)
pub fn tactics_summary(g: &mut crate::game::Game) -> String { todo!() }
