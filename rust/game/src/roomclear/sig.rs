#![allow(unused_variables)]
//! Every function and method of `roomclear`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/roomclear.ts:77 freshSeedOf)
pub fn fresh_seed_of(dice_seed: f64) -> f64 { todo!() }

/// (src/game/roomclear.ts:79 freshWayOf)
pub fn fresh_way_of(dice_seed: f64, n: f64) -> f64 { todo!() }

/// (src/game/roomclear.ts:93 placeDefenders)
pub fn place_defenders(seed: f64, attempt: Option<f64>, s: Option<&crate::roomclear::PlaceDefendersS>) -> Vec<crate::roomclear::Placement> { todo!() }

/// (src/game/roomclear.ts:112 checkStructure)
pub fn check_structure(g: &mut crate::game::Game, s: Option<&crate::roomclear::CheckStructureS>) -> Vec<String> { todo!() }

/// (src/game/roomclear.ts:158 checkSquads)
pub fn check_squads(g: &mut crate::game::Game, s: &crate::roomclear::CheckSquadsS) -> Vec<String> { todo!() }

/// (src/game/roomclear.ts:189 doorSides)
pub fn door_sides(g: &mut crate::game::Game, d: DoorId) -> Vec<V3> { todo!() }

/// (src/game/roomclear.ts:193 sameBounds)
pub fn same_bounds(a: crate::scene_scenarios::Rect, b: crate::scene_scenarios::Rect) -> bool { todo!() }

/// (src/game/roomclear.ts:195 propGap)
pub fn prop_gap(p: crate::roomclear::PropGapP, at: V3) -> f64 { todo!() }

/// (src/game/roomclear.ts:201 wallBetween)
pub fn wall_between(g: &mut crate::game::Game, a: V3, b: V3) -> bool { todo!() }

/// (src/game/roomclear.ts:207 doorPlan)
pub fn door_plan(g: &mut crate::game::Game, door: &str) -> Option<crate::scene_level::RoomClearPlan> { todo!() }

/// (src/game/roomclear.ts:213 listed)
pub fn listed(a: &[String], and: &str) -> String { todo!() }

/// (src/game/roomclear.ts:216 clearWords)
pub fn clear_words(g: &mut crate::game::Game, rooms: &[crate::scene_scenarios::StructureRoom]) -> String { todo!() }

/// (src/game/roomclear.ts:232 RoomClearScenario.structure)
pub fn room_clear_scenario_structure(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::StructureData> { todo!() }

/// (src/game/roomclear.ts:248 RoomClearScenario.wayNow)
pub fn room_clear_scenario_way_now(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/roomclear.ts:253 RoomClearScenario.startNow)
pub fn room_clear_scenario_start_now(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::TeamStart> { todo!() }

/// (src/game/roomclear.ts:255 RoomClearScenario.startAt)
pub fn room_clear_scenario_start_at(g: &mut crate::game::Game) -> Option<V3> { todo!() }

/// (src/game/roomclear.ts:265 RoomClearScenario.constructor)
pub fn room_clear_scenario_new(g: &mut crate::game::Game) -> crate::roomclear::RoomClearScenario { todo!() }

/// (src/game/roomclear.ts:273 RoomClearScenario.active)
pub fn room_clear_scenario_active(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/roomclear.ts:276 RoomClearScenario.canStart)
pub fn room_clear_scenario_can_start(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/roomclear.ts:284 RoomClearScenario.start)
pub fn room_clear_scenario_start(g: &mut crate::game::Game, seed: Option<f64>) -> Option<String> { todo!() }

/// (src/game/roomclear.ts:298 RoomClearScenario.stop)
pub fn room_clear_scenario_stop(g: &mut crate::game::Game) { todo!() }

/// (src/game/roomclear.ts:300 RoomClearScenario.restage)
pub fn room_clear_scenario_restage(g: &mut crate::game::Game) { todo!() }

/// (src/game/roomclear.ts:301 RoomClearScenario.clear)
pub fn room_clear_scenario_clear(g: &mut crate::game::Game) { todo!() }

/// (src/game/roomclear.ts:302 RoomClearScenario.note)
pub fn room_clear_scenario_note(g: &mut crate::game::Game, what: &str) { todo!() }

/// (src/game/roomclear.ts:306 RoomClearScenario.deal)
pub fn room_clear_scenario_deal(g: &mut crate::game::Game) -> Vec<crate::nightbreach::NightBreachMark> { todo!() }

/// (src/game/roomclear.ts:315 RoomClearScenario.drillRoom)
pub fn room_clear_scenario_drill_room(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::StructureRoom> { todo!() }

/// (src/game/roomclear.ts:317 RoomClearScenario.onStaged)
pub fn room_clear_scenario_on_staged(g: &mut crate::game::Game) { todo!() }

/// (src/game/roomclear.ts:330 RoomClearScenario.tick)
pub fn room_clear_scenario_tick(g: &mut crate::game::Game, _dt: f64) { todo!() }

/// (src/game/roomclear.ts:366 RoomClearScenario.adopt)
pub fn room_clear_scenario_adopt(g: &mut crate::game::Game, gd: GuardId, p: &crate::roomclear::Placement) { todo!() }

/// (src/game/roomclear.ts:368 RoomClearScenario.rehome)
pub fn room_clear_scenario_rehome(g: &mut crate::game::Game, gd: GuardId, room: &str) { todo!() }

/// (src/game/roomclear.ts:373 RoomClearScenario.finish)
pub fn room_clear_scenario_finish(g: &mut crate::game::Game, end: crate::roomclear::RoomClearEnd, why: &str) { todo!() }

/// (src/game/roomclear.ts:383 RoomClearScenario.state)
pub fn room_clear_scenario_state(g: &mut crate::game::Game) -> crate::roomclear::RoomClearState { todo!() }

/// (src/game/roomclear.ts:385 RoomClearScenario.team)
pub fn room_clear_scenario_team(g: &mut crate::game::Game) -> crate::roomclear::RoomClearScenarioTeam { todo!() }

/// (src/game/roomclear.ts:387 RoomClearScenario.roomAt)
pub fn room_clear_scenario_room_at(g: &mut crate::game::Game, p: V3) -> Option<String> { todo!() }

/// (src/game/roomclear.ts:390 RoomClearScenario.objective)
pub fn room_clear_scenario_objective(g: &mut crate::game::Game) -> Option<crate::roomclear::RoomClearScenarioObjective> { todo!() }

/// (src/game/roomclear.ts:406 RoomClearScenario.firstOpen)
pub fn room_clear_scenario_first_open(g: &mut crate::game::Game) -> Option<crate::roomclear::RoomState> { todo!() }

/// (src/game/roomclear.ts:413 RoomClearScenario.summary)
pub fn room_clear_scenario_summary(g: &mut crate::game::Game) -> String { todo!() }


/// INVENTED: the layout's `budget` getter: a structure with maxMen holds Sam, the four and its men; None = the night breach's own budget. Read live at each staging. (src/game/roomclear.ts:269)
pub fn room_clear_scenario_budget(g: &mut crate::game::Game) -> Option<f64> { todo!() }

/// INVENTED: the layout's `stagedLine` closure: the line the staging says. (src/game/roomclear.ts:271)
pub fn room_clear_scenario_staged_line(g: &mut crate::game::Game) -> String { todo!() }
