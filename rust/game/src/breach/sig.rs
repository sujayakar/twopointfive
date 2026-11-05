#![allow(unused_variables)]
//! Every function and method of `breach`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/breach.ts:44 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/breach.ts:97 BreachDrill.constructor)
pub fn breach_drill_new(g: &mut crate::game::Game, plan: &crate::scene_level::BreachPlan, a: GuardId, b: GuardId, hostiles: &[GuardId], opts: Option<crate::breach::BreachDrillNewOpts>) -> crate::breach::BreachDrill { todo!() }

/// (src/game/breach.ts:105 BreachDrill.door)
pub fn breach_drill_door(g: &mut crate::game::Game) -> Option<DoorId> { todo!() }

/// (src/game/breach.ts:106 BreachDrill.nextDoor)
pub fn breach_drill_next_door(g: &mut crate::game::Game) -> Option<DoorId> { todo!() }

/// (src/game/breach.ts:107 BreachDrill.beat)
pub fn breach_drill_beat(g: &mut crate::game::Game, what: &str) { todo!() }

/// (src/game/breach.ts:108 BreachDrill.once)
pub fn breach_drill_once(g: &mut crate::game::Game, key: &str) -> bool { todo!() }

/// (src/game/breach.ts:110 BreachDrill.charge)
pub fn breach_drill_charge(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/breach.ts:111 BreachDrill.say)
pub fn breach_drill_say(g: &mut crate::game::Game, gd: GuardId, key: &str, text: &str, radio: Option<bool>) { todo!() }

/// (src/game/breach.ts:115 BreachDrill.route)
pub fn breach_drill_route(g: &mut crate::game::Game, gd: GuardId, wps: &mut Vec<V3>, pts: &[V3], patch: Option<crate::squad::OrderPatch>) { todo!() }

/// (src/game/breach.ts:116 BreachDrill.advance)
pub fn breach_drill_advance(g: &mut crate::game::Game, gd: GuardId, wps: &mut Vec<V3>, pass: Option<f64>) -> bool { todo!() }

/// (src/game/breach.ts:122 BreachDrill.inFunnel)
pub fn breach_drill_in_funnel(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/breach.ts:131 BreachDrill.start)
pub fn breach_drill_start(g: &mut crate::game::Game) { todo!() }

/// (src/game/breach.ts:147 BreachDrill.end)
pub fn breach_drill_end(g: &mut crate::game::Game) { todo!() }

/// (src/game/breach.ts:151 BreachDrill.canInHand)
pub fn breach_drill_can_in_hand(g: &mut crate::game::Game) { todo!() }

/// (src/game/breach.ts:163 BreachDrill.tick)
pub fn breach_drill_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/breach.ts:305 BreachDrill.casualties)
pub fn breach_drill_casualties(g: &mut crate::game::Game, t: f64) -> bool { todo!() }

/// (src/game/breach.ts:321 BreachDrill.muzzlesOnDoor)
pub fn breach_drill_muzzles_on_door(g: &mut crate::game::Game) { todo!() }

/// (src/game/breach.ts:329 BreachDrill.inside)
pub fn breach_drill_inside(g: &mut crate::game::Game, d: f64, y: f64, s: Option<f64>) -> V3 { todo!() }

/// (src/game/breach.ts:335 BreachDrill.legTo)
pub fn breach_drill_leg_to(g: &mut crate::game::Game, gd: GuardId, mark: V3) -> Vec<V3> { todo!() }

/// (src/game/breach.ts:345 BreachDrill.insideRoom)
pub fn breach_drill_inside_room(g: &mut crate::game::Game, p: V3, m: f64) -> bool { todo!() }

/// (src/game/breach.ts:347 BreachDrill.flowOf)
pub fn breach_drill_flow_of(g: &mut crate::game::Game, i: usize) -> crate::scene_level::BreachFlow { todo!() }

/// (src/game/breach.ts:349 BreachDrill.startFlow)
pub fn breach_drill_start_flow(g: &mut crate::game::Game, i: usize) { todo!() }

/// (src/game/breach.ts:356 BreachDrill.tickFlow)
pub fn breach_drill_tick_flow(g: &mut crate::game::Game, i: usize, dt: f64) { todo!() }

/// (src/game/breach.ts:398 BreachDrill.tickHold)
pub fn breach_drill_tick_hold(g: &mut crate::game::Game, i: usize, dt: f64) { todo!() }

/// (src/game/breach.ts:400 BreachDrill.roomClear)
pub fn breach_drill_room_clear(g: &mut crate::game::Game, te: f64) -> bool { todo!() }

/// (src/game/breach.ts:412 BreachDrill.pickable)
pub fn breach_drill_pickable(g: &mut crate::game::Game, i: usize) -> bool { todo!() }

/// (src/game/breach.ts:433 BreachDrill.stagedExchange)
pub fn breach_drill_staged_exchange(g: &mut crate::game::Game, i: usize, dt: f64) -> bool { todo!() }
