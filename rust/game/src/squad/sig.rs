#![allow(unused_variables)]
//! Every function and method of `squad`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/squad.ts:35 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/squad.ts:45 strafeAlongPath)
pub fn strafe_along_path(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64) -> bool { todo!() }

/// (src/game/squad.ts:60 runGuardScript)
pub fn run_guard_script(g: &mut crate::game::Game, gd: GuardId, dt: f64) { todo!() }

/// (src/game/squad.ts:135 endGuardScript)
pub fn end_guard_script(g: &mut crate::game::Game, gd: GuardId, state: Option<crate::squad::EndGuardScriptState>) { todo!() }

/// (src/game/squad.ts:144 order)
pub fn order(g: &mut crate::game::Game, gd: GuardId, patch: crate::squad::OrderPatch) { todo!() }

/// (src/game/squad.ts:155 Sweep.start)
pub fn sweep_start(this: &mut crate::squad::Sweep, from: V3, pts: &[V3], t0: f64, dur: f64) { todo!() }

/// (src/game/squad.ts:157 Sweep.at)
pub fn sweep_at(this: &mut crate::squad::Sweep, t: f64, out: &mut V3) -> bool { todo!() }

/// (src/game/squad.ts:200 RoomClear.constructor)
pub fn room_clear_new(g: &mut crate::game::Game, plan: &crate::scene_level::RoomClearPlan, a: GuardId, b: Option<GuardId>, opts: Option<crate::squad::RoomClearNewOpts>) -> RoomClearId { todo!() }

/// (src/game/squad.ts:204 RoomClear.start)
pub fn room_clear_start(g: &mut crate::game::Game, id: RoomClearId) { todo!() }

/// (src/game/squad.ts:221 RoomClear.end)
pub fn room_clear_end(g: &mut crate::game::Game, id: RoomClearId, state: Option<crate::squad::RoomClearEndState>) { todo!() }

/// (src/game/squad.ts:223 RoomClear.intact)
pub fn room_clear_intact(g: &mut crate::game::Game, id: RoomClearId) -> bool { todo!() }

/// (src/game/squad.ts:228 RoomClear.complete)
pub fn room_clear_complete(g: &mut crate::game::Game, id: RoomClearId) -> bool { todo!() }

/// (src/game/squad.ts:230 RoomClear.men)
pub fn room_clear_men(g: &mut crate::game::Game, id: RoomClearId) -> Vec<GuardId> { todo!() }

/// (src/game/squad.ts:232 RoomClear.once)
pub fn room_clear_once(g: &mut crate::game::Game, id: RoomClearId, key: &str) -> bool { todo!() }

/// (src/game/squad.ts:235 RoomClear.say)
pub fn room_clear_say(g: &mut crate::game::Game, id: RoomClearId, key: crate::squad::RoomClearSayKey, gd: GuardId, radio: Option<bool>) { todo!() }

/// (src/game/squad.ts:241 RoomClear.bodyHere)
pub fn room_clear_body_here(g: &mut crate::game::Game, id: RoomClearId, gd: GuardId) -> Option<GuardId> { todo!() }

/// (src/game/squad.ts:247 RoomClear.route)
pub fn room_clear_route(g: &mut crate::game::Game, id: RoomClearId, gd: GuardId, wps: &mut Vec<V3>, pts: &[V3], patch: Option<crate::squad::RoomClearRoutePatch>) { todo!() }

/// (src/game/squad.ts:249 RoomClear.advance)
pub fn room_clear_advance(g: &mut crate::game::Game, id: RoomClearId, gd: GuardId, wps: &mut Vec<V3>) -> bool { todo!() }

/// (src/game/squad.ts:254 RoomClear.ahead)
pub fn room_clear_ahead(p: V3, yaw: f64, dist: f64, y: f64) -> V3 { todo!() }

/// (src/game/squad.ts:256 RoomClear.enter)
pub fn room_clear_enter(g: &mut crate::game::Game, id: RoomClearId, t: f64) { todo!() }

/// (src/game/squad.ts:264 RoomClear.enterSweep)
pub fn room_clear_enter_sweep(g: &mut crate::game::Game, id: RoomClearId, sw_key: RoomClearId, aim: V3, list: &[V3], t: f64) -> Option<Vec<V3>> { todo!() }

/// (src/game/squad.ts:272 RoomClear.shove)
pub fn room_clear_shove(g: &mut crate::game::Game, id: RoomClearId, t: f64) { todo!() }

/// (src/game/squad.ts:283 RoomClear.tick)
pub fn room_clear_tick(g: &mut crate::game::Game, id: RoomClearId, t: f64, dt: f64) { todo!() }

/// (src/game/squad.ts:401 restSecs)
pub fn rest_secs(rest: &[V3]) -> f64 { todo!() }

/// (src/game/squad.ts:403 roomCentre)
pub fn room_centre(p: &crate::scene_level::RoomClearPlan) -> V3 { todo!() }

/// (src/game/squad.ts:410 nextDoorOf)
pub fn next_door_of(g: &mut crate::game::Game, p: &crate::scene_level::RoomClearPlan) -> Option<DoorId> { todo!() }

/// (src/game/squad.ts:441 breakOff)
pub fn break_off(g: &mut crate::game::Game, gd: GuardId, awareness: f64, last_known: Option<V3>) { todo!() }

/// (src/game/squad.ts:455 perceive)
pub fn perceive(g: &mut crate::game::Game, gd: GuardId, dt: f64, before: f64) -> Option<crate::squad::Perceive> { todo!() }

/// (src/game/squad.ts:475 inBounds)
pub fn in_bounds(p: V3, b: Option<crate::scene_level::BreachPlanBounds>, pad: Option<f64>) -> bool { todo!() }

/// (src/game/squad.ts:477 pathLen)
pub fn path_len(g: &mut crate::game::Game, a: V3, b: V3) -> f64 { todo!() }

/// (src/game/squad.ts:497 Clearing.constructor)
pub fn clearing_new(g: &mut crate::game::Game, rooms: &[Vec<crate::scene_level::RoomClearPlan>], a: GuardId, b: Option<GuardId>) -> crate::squad::Clearing { todo!() }

/// (src/game/squad.ts:501 Clearing.cap)
pub fn clearing_cap(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/squad.ts:502 Clearing.name)
pub fn clearing_name(g: &mut crate::game::Game, p: Option<&crate::scene_level::RoomClearPlan>) -> String { todo!() }

/// (src/game/squad.ts:504 Clearing.summary)
pub fn clearing_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/squad.ts:511 Clearing.cancel)
pub fn clearing_cancel(g: &mut crate::game::Game) { todo!() }

/// (src/game/squad.ts:512 Clearing.men)
pub fn clearing_men(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/squad.ts:513 Clearing.say)
pub fn clearing_say(g: &mut crate::game::Game, text: &str, radio: Option<bool>) { todo!() }

/// (src/game/squad.ts:516 Clearing.tick)
pub fn clearing_tick(g: &mut crate::game::Game, dt: f64) -> bool { todo!() }

/// (src/game/squad.ts:583 Clearing.muster)
pub fn clearing_muster(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/squad.ts:594 Clearing.readDoors)
pub fn clearing_read_doors(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/squad.ts:624 Clearing.approach)
pub fn clearing_approach(g: &mut crate::game::Game) { todo!() }

/// (src/game/squad.ts:655 Clearing.startDrill)
pub fn clearing_start_drill(g: &mut crate::game::Game) { todo!() }

/// (src/game/squad.ts:658 Clearing.nextRoom)
pub fn clearing_next_room(g: &mut crate::game::Game, cleared: Option<bool>) -> bool { todo!() }
