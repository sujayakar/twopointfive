#![allow(unused_variables)]
//! Every function and method of `aimcost`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/aimcost.ts:76 lampStruck)
pub fn lamp_struck(g: &mut crate::game::Game, pos: V3, boxes: &[Option<BoxId>], stopped: bool, legs: &[crate::aimcost::RoundLeg], bare: Option<f64>, slack: Option<f64>) -> bool { todo!() }

/// (src/game/aimcost.ts:103 legsNear)
pub fn legs_near(pos: V3, legs: &[crate::aimcost::RoundLeg]) -> f64 { todo!() }

/// (src/game/aimcost.ts:110 feetHold)
pub fn feet_hold(g: &mut crate::game::Game, prev: f64, speed: f64, dt: f64) -> f64 { todo!() }

/// (src/game/aimcost.ts:116 aimCostSummary)
pub fn aim_cost_summary(g: &mut crate::game::Game) -> String { todo!() }
