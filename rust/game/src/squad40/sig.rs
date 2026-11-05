#![allow(unused_variables)]
//! Every function and method of `squad40`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/squad40.ts:75 squad40Here)
pub fn squad40_here(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/squad40.ts:94 tubeUp)
pub fn tube_up(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/squad40.ts:96 squad40Left)
pub fn squad40_left(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/squad40.ts:97 squad40Brought)
pub fn squad40_brought(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/squad40.ts:99 squad40Of)
pub fn squad40_of(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/squad40.ts:101 squad40Word)
pub fn squad40_word(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/squad40.ts:105 squad40Stage)
pub fn squad40_stage(g: &mut crate::game::Game, carried: &[(GuardId, f64)]) { todo!() }

/// (src/game/squad40.ts:116 bodies)
pub fn bodies(g: &mut crate::game::Game) -> crate::squad40::Bodies { todo!() }

/// (src/game/squad40.ts:125 onIt)
pub fn on_it(g: &mut crate::game::Game, gd: GuardId, at: V3) -> bool { todo!() }

/// (src/game/squad40.ts:127 gunOn)
pub fn gun_on(g: &mut crate::game::Game, gd: GuardId, at: V3) -> bool { todo!() }

/// (src/game/squad40.ts:130 shotOf)
pub fn shot_of(g: &mut crate::game::Game, gd: GuardId, at: V3, b: Option<&crate::squad40::Bodies>) -> Option<crate::squad40::ShotOf> { todo!() }

/// (src/game/squad40.ts:145 caught)
pub fn caught(g: &mut crate::game::Game, a: &crate::squad40::Shot, at: V3) -> Option<String> { todo!() }

/// (src/game/squad40.ts:152 plan)
pub fn plan(g: &mut crate::game::Game, men: &[GuardId], at: V3, asker: Option<crate::squad40::Asker>) -> crate::squad40::Plan { todo!() }

/// (src/game/squad40.ts:176 fire)
pub fn fire(g: &mut crate::game::Game, pl: PlayerId, a: &mut crate::squad40::Shot, at: V3, turned: bool) { todo!() }

/// (src/game/squad40.ts:184 turnTick)
pub fn turn_tick(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) { todo!() }

/// (src/game/squad40.ts:202 row)
pub fn row(g: &mut crate::game::Game) -> OrdersOrderDefId { todo!() }

/// (src/game/squad40.ts:221 notes)
pub fn notes(g: &mut crate::game::Game) -> Vec<crate::orders::OrderNote> { todo!() }

/// (src/game/squad40.ts:229 squad40Tick)
pub fn squad40_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/squad40.ts:247 squad40Plans)
pub fn squad40_plans(g: &mut crate::game::Game) -> Vec<crate::squad40::Squad40Plan> { todo!() }

/// (src/game/squad40.ts:255 squad40Summary)
pub fn squad40_summary(g: &mut crate::game::Game) -> String { todo!() }
