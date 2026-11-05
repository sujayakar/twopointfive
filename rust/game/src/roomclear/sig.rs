#![allow(unused_variables)]
//! Every function and method of `roomclear`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/roomclear.ts:78 freshSeedOf)
pub fn fresh_seed_of(dice_seed: f64) -> f64 { todo!() }

/// (src/game/roomclear.ts:80 freshWayOf)
pub fn fresh_way_of(dice_seed: f64, n: f64) -> f64 { todo!() }

/// (src/game/roomclear.ts:94 placeDefenders)
pub fn place_defenders(seed: f64, attempt: Option<f64>, s: Option<&crate::roomclear::PlaceDefendersS>) -> Vec<crate::roomclear::Placement> { todo!() }

/// (src/game/roomclear.ts:113 checkStructure)
pub fn check_structure(g: &mut crate::game::Game, s: Option<&crate::roomclear::CheckStructureS>) -> Vec<String> { todo!() }

/// (src/game/roomclear.ts:159 checkSquads)
pub fn check_squads(g: &mut crate::game::Game, s: &crate::roomclear::CheckSquadsS) -> Vec<String> { todo!() }

/// (src/game/roomclear.ts:190 doorSides)
pub fn door_sides(g: &mut crate::game::Game, d: DoorId) -> Vec<V3> { todo!() }

/// (src/game/roomclear.ts:194 sameBounds)
pub fn same_bounds(a: crate::scene_scenarios::Rect, b: crate::scene_scenarios::Rect) -> bool { todo!() }

/// (src/game/roomclear.ts:196 propGap)
pub fn prop_gap(p: crate::roomclear::PropGapP, at: V3) -> f64 { todo!() }

/// (src/game/roomclear.ts:202 wallBetween)
pub fn wall_between(g: &mut crate::game::Game, a: V3, b: V3) -> bool { todo!() }

/// (src/game/roomclear.ts:208 doorPlan)
pub fn door_plan(g: &mut crate::game::Game, door: &str) -> Option<crate::scene_level::RoomClearPlan> { todo!() }

/// (src/game/roomclear.ts:214 listed)
pub fn listed(a: &[String], and: &str) -> String { todo!() }

/// (src/game/roomclear.ts:217 clearWords)
pub fn clear_words(g: &mut crate::game::Game, rooms: &[crate::scene_scenarios::StructureRoom]) -> String { todo!() }

/// (src/game/roomclear.ts:233 RoomClearScenario.structure)
pub fn room_clear_scenario_structure(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::StructureData> { todo!() }

/// (src/game/roomclear.ts:249 RoomClearScenario.wayNow)
pub fn room_clear_scenario_way_now(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/roomclear.ts:254 RoomClearScenario.startNow)
pub fn room_clear_scenario_start_now(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::TeamStart> { todo!() }

/// (src/game/roomclear.ts:256 RoomClearScenario.startAt)
pub fn room_clear_scenario_start_at(g: &mut crate::game::Game) -> Option<V3> { todo!() }

/// (src/game/roomclear.ts:266 RoomClearScenario.constructor)
pub fn room_clear_scenario_new(g: &mut crate::game::Game) -> crate::roomclear::RoomClearScenario { todo!() }

/// (src/game/roomclear.ts:274 RoomClearScenario.active)
pub fn room_clear_scenario_active(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/roomclear.ts:277 RoomClearScenario.canStart)
pub fn room_clear_scenario_can_start(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/roomclear.ts:285 RoomClearScenario.start)
pub fn room_clear_scenario_start(g: &mut crate::game::Game, seed: Option<f64>) -> Option<String> { todo!() }

/// (src/game/roomclear.ts:299 RoomClearScenario.stop)
pub fn room_clear_scenario_stop(g: &mut crate::game::Game) { todo!() }

/// (src/game/roomclear.ts:301 RoomClearScenario.restage)
pub fn room_clear_scenario_restage(g: &mut crate::game::Game) { todo!() }

/// (src/game/roomclear.ts:302 RoomClearScenario.clear)
pub fn room_clear_scenario_clear(g: &mut crate::game::Game) { todo!() }

/// (src/game/roomclear.ts:303 RoomClearScenario.note)
pub fn room_clear_scenario_note(g: &mut crate::game::Game, what: &str) { todo!() }

/// (src/game/roomclear.ts:307 RoomClearScenario.deal)
pub fn room_clear_scenario_deal(g: &mut crate::game::Game) -> Vec<crate::nightbreach::NightBreachMark> { todo!() }

/// (src/game/roomclear.ts:316 RoomClearScenario.drillRoom)
pub fn room_clear_scenario_drill_room(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::StructureRoom> { todo!() }

/// (src/game/roomclear.ts:318 RoomClearScenario.onStaged)
pub fn room_clear_scenario_on_staged(g: &mut crate::game::Game) { todo!() }

/// (src/game/roomclear.ts:331 RoomClearScenario.tick)
pub fn room_clear_scenario_tick(g: &mut crate::game::Game, _dt: f64) { todo!() }

/// (src/game/roomclear.ts:367 RoomClearScenario.adopt)
pub fn room_clear_scenario_adopt(g: &mut crate::game::Game, gd: GuardId, p: &crate::roomclear::Placement) { todo!() }

/// (src/game/roomclear.ts:369 RoomClearScenario.rehome)
pub fn room_clear_scenario_rehome(g: &mut crate::game::Game, gd: GuardId, room: &str) { todo!() }

/// (src/game/roomclear.ts:374 RoomClearScenario.finish)
pub fn room_clear_scenario_finish(g: &mut crate::game::Game, end: crate::roomclear::RoomClearEnd, why: &str) { todo!() }

/// (src/game/roomclear.ts:384 RoomClearScenario.state)
pub fn room_clear_scenario_state(g: &mut crate::game::Game) -> crate::roomclear::RoomClearState { todo!() }

/// (src/game/roomclear.ts:386 RoomClearScenario.team)
pub fn room_clear_scenario_team(g: &mut crate::game::Game) -> crate::roomclear::RoomClearScenarioTeam { todo!() }

/// (src/game/roomclear.ts:388 RoomClearScenario.roomAt)
pub fn room_clear_scenario_room_at(g: &mut crate::game::Game, p: V3) -> Option<String> { todo!() }

/// (src/game/roomclear.ts:391 RoomClearScenario.objective)
pub fn room_clear_scenario_objective(g: &mut crate::game::Game) -> Option<crate::roomclear::RoomClearScenarioObjective> { todo!() }

/// (src/game/roomclear.ts:407 RoomClearScenario.firstOpen)
pub fn room_clear_scenario_first_open(g: &mut crate::game::Game) -> Option<crate::roomclear::RoomState> { todo!() }

/// (src/game/roomclear.ts:414 RoomClearScenario.summary)
pub fn room_clear_scenario_summary(g: &mut crate::game::Game) -> String { todo!() }
