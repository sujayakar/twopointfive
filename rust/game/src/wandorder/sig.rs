#![allow(unused_variables)]
//! Every function and method of `wandorder`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/wandorder.ts:59 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/wandorder.ts:61 live)
pub fn live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/wandorder.ts:62 carries)
pub fn carries(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/wandorder.ts:65 fought)
pub fn fought(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) -> bool { todo!() }

/// (src/game/wandorder.ts:67 onFace)
pub fn on_face(g: &mut crate::game::Game, d: DoorId, side: f64) -> V3 { todo!() }

/// (src/game/wandorder.ts:68 spotOn)
pub fn spot_on(g: &mut crate::game::Game, d: DoorId, side: f64) -> crate::doors::DoorsWorkSpot { todo!() }

/// (src/game/wandorder.ts:71 pick)
pub fn pick(g: &mut crate::game::Game, men: &[GuardId], t: &mut crate::orders::OrderTarget) -> crate::wandorder::Pick { todo!() }

/// (src/game/wandorder.ts:81 lensOut)
pub fn lens_out(g: &mut crate::game::Game, gd: GuardId, j: &mut crate::wandorder::Job) { todo!() }

/// (src/game/wandorder.ts:86 settle)
pub fn settle(g: &mut crate::game::Game, gd: GuardId, j: &mut crate::wandorder::Job, line: Option<&str>, moved: bool) { todo!() }

/// (src/game/wandorder.ts:94 issue)
pub fn issue(g: &mut crate::game::Game, men: &[GuardId], t: &mut crate::orders::OrderTarget) -> crate::wandorder::Issue { todo!() }

/// (src/game/wandorder.ts:107 tick)
pub fn tick(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) { todo!() }

/// (src/game/wandorder.ts:126 row)
pub fn row(g: &mut crate::game::Game) -> OrdersOrderDefId { todo!() }

/// (src/game/wandorder.ts:143 wandOrderKey)
pub fn wand_order_key(g: &mut crate::game::Game, input: InputId) -> bool { todo!() }

/// (src/game/wandorder.ts:167 mateWandEye)
pub fn mate_wand_eye(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::fogofwar::FogEye> { todo!() }

/// (src/game/wandorder.ts:176 mateWandEyes)
pub fn mate_wand_eyes(g: &mut crate::game::Game) -> Vec<crate::fogofwar::FogEye> { todo!() }

/// (src/game/wandorder.ts:182 wandOrderJobs)
pub fn wand_order_jobs(g: &mut crate::game::Game) -> Vec<crate::wandorder::WandOrderJobsItem> { todo!() }
