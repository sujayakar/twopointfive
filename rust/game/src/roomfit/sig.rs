#![allow(unused_variables)]
//! Every function and method of `roomfit`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/roomfit.ts:84 compass)
pub fn compass(yaw: f64) -> String { todo!() }

/// (src/game/roomfit.ts:86 wayWords)
pub fn way_words(g: &mut crate::game::Game, at: V3, to: V3, yaw: f64) -> String { todo!() }

/// (src/game/roomfit.ts:87 runWords)
pub fn run_words(g: &mut crate::game::Game, at: V3, to: V3, yaw: f64) -> String { todo!() }

/// (src/game/roomfit.ts:89 offLine)
pub fn off_line(p: V3, a: V3, b: V3) -> f64 { todo!() }

/// (src/game/roomfit.ts:96 threatsOf)
pub fn threats_of(g: &mut crate::game::Game, r: &crate::roomread::RoomRead, at: V3, front: f64, reach: Option<f64>) -> Vec<crate::roomfit::Threat> { todo!() }

/// (src/game/roomfit.ts:122 sees)
pub fn sees(g: &mut crate::game::Game, q: V3, t: &crate::roomfit::Threat) -> bool { todo!() }

/// (src/game/roomfit.ts:135 watchablesAt)
pub fn watchables_at(g: &mut crate::game::Game, p: V3, front: f64, reach: f64, bodies: &[V3]) -> Vec<crate::roomfit::Watchable> { todo!() }

/// (src/game/roomfit.ts:144 inFunnel)
pub fn in_funnel(g: &mut crate::game::Game, q: V3, w: crate::roomread::RoomWay) -> bool { todo!() }

/// (src/game/roomfit.ts:152 fitSpots)
pub fn fit_spots(g: &mut crate::game::Game, at: V3, n: f64, front: f64, from: V3, bodies: Option<&[V3]>, held: Option<&[String]>) -> Option<Vec<crate::roomfit::Fit>> { todo!() }

/// (src/game/roomfit.ts:208 spareRun)
pub fn spare_run(g: &mut crate::game::Game, q: V3, front: f64, mates: &[crate::roomfit::Fit]) -> Option<crate::roomfit::Threat> { todo!() }

/// (src/game/roomfit.ts:219 fitGroups)
pub fn fit_groups(fits: &[crate::roomfit::Fit]) -> Vec<f64> { todo!() }

/// (src/game/roomfit.ts:227 take)
pub fn take(g: &mut crate::game::Game, a: OrdersAssignmentId, f: &crate::roomfit::Fit) { todo!() }

/// (src/game/roomfit.ts:232 fitTick)
pub fn fit_tick(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) { todo!() }
