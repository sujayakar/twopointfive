#![allow(unused_variables)]
//! Every function and method of `doorswing`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/doorswing.ts:35 swingsNow)
pub fn swings_now(g: &mut crate::game::Game, deals: &[crate::doorswing::SwingDeal]) -> Vec<crate::doorswing::SwingDeal> { todo!() }

/// (src/game/doorswing.ts:41 sideOf)
pub fn side_of(g: &mut crate::game::Game, d: DoorId, x: f64, z: f64) -> f64 { todo!() }

/// (src/game/doorswing.ts:45 mid)
pub fn mid(r: crate::doorswing::Rect) -> [f64; 2] { todo!() }

/// (src/game/doorswing.ts:46 area)
pub fn area(r: crate::doorswing::Rect) -> f64 { todo!() }

/// (src/game/doorswing.ts:52 roofOver)
pub fn roof_over(g: &mut crate::game::Game, d: DoorId, s: f64) -> f64 { todo!() }

/// (src/game/doorswing.ts:63 outsideOf)
pub fn outside_of(g: &mut crate::game::Game, d: DoorId) -> f64 { todo!() }

/// (src/game/doorswing.ts:70 floors)
pub fn floors(g: &mut crate::game::Game) -> JsMap<DoorId, crate::doorswing::FloorsValue> { todo!() }

/// (src/game/doorswing.ts:115 leafCut)
pub fn leaf_cut(g: &mut crate::game::Game, d: DoorId, s: f64) -> f64 { todo!() }

/// (src/game/doorswing.ts:141 leafSwings)
pub fn leaf_swings(g: &mut crate::game::Game, d: DoorId, s: f64) -> bool { todo!() }

/// (src/game/doorswing.ts:149 dealSwings)
pub fn deal_swings(g: &mut crate::game::Game) -> Vec<crate::doorswing::SwingDeal> { todo!() }
