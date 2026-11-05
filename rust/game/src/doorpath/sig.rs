#![allow(unused_variables)]
//! Every function and method of `doorpath`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/doorpath.ts:97 flat)
pub fn flat(a: &[f64], b: &[f64]) -> f64 { todo!() }

/// (src/game/doorpath.ts:100 gone)
pub fn gone(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/doorpath.ts:102 segSeg)
pub fn seg_seg(a: V3, b: V3, c_2: V3, d: V3) -> f64 { todo!() }

/// (src/game/doorpath.ts:108 ptSeg)
pub fn pt_seg(p: V3, a: V3, b: V3) -> f64 { todo!() }

/// (src/game/doorpath.ts:118 DoorPath.constructor)
pub fn door_path_new(g: &mut crate::game::Game) -> crate::doorpath::DoorPath { todo!() }

/// (src/game/doorpath.ts:119 DoorPath.reset)
pub fn door_path_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/doorpath.ts:125 DoorPath.guns)
pub fn door_path_guns(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/doorpath.ts:167 DoorPath.friendOn)
pub fn door_path_friend_on(g: &mut crate::game::Game, gd: GuardId, yaw: f64) -> bool { todo!() }

/// (src/game/doorpath.ts:176 DoorPath.after)
pub fn door_path_after(g: &mut crate::game::Game) { todo!() }

/// (src/game/doorpath.ts:183 DoorPath.leaf)
pub fn door_path_leaf(g: &mut crate::game::Game, d: DoorId) -> [V3; 2] { todo!() }

/// (src/game/doorpath.ts:184 DoorPath.leaves)
pub fn door_path_leaves(g: &mut crate::game::Game, near: V3, far: V3) -> Vec<[V3; 2]> { todo!() }

/// (src/game/doorpath.ts:186 DoorPath.free)
pub fn door_path_free(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/doorpath.ts:187 DoorPath.open)
pub fn door_path_open(g: &mut crate::game::Game, a: V3, b: V3) -> bool { todo!() }

/// (src/game/doorpath.ts:195 DoorPath.roundLeaves)
pub fn door_path_round_leaves(g: &mut crate::game::Game, a: V3, b: V3) -> Option<crate::doorpath::DoorPathRoundLeaves> { todo!() }

/// (src/game/doorpath.ts:217 DoorPath.way)
pub fn door_path_way(g: &mut crate::game::Game, a: V3, b: V3) -> Option<crate::doorpath::DoorPathWay> { todo!() }

/// (src/game/doorpath.ts:229 DoorPath.lay)
pub fn door_path_lay(g: &mut crate::game::Game, w: &mut crate::doorpath::Walk) { todo!() }

/// (src/game/doorpath.ts:252 DoorPath.stand)
pub fn door_path_stand(g: &mut crate::game::Game, w: &crate::doorpath::Walk, from: V3, reach: Option<f64>) -> Option<V3> { todo!() }

/// (src/game/doorpath.ts:263 DoorPath.fallen)
pub fn door_path_fallen(g: &mut crate::game::Game, at: V3, r: f64) -> Vec<V3> { todo!() }

/// (src/game/doorpath.ts:271 DoorPath.move)
pub fn door_path_move(g: &mut crate::game::Game, w: &mut crate::doorpath::Walk, q: V3) { todo!() }

/// (src/game/doorpath.ts:276 DoorPath.total)
pub fn door_path_total(g: &mut crate::game::Game, w: &crate::doorpath::Walk) -> f64 { todo!() }

/// (src/game/doorpath.ts:277 DoorPath.pointAt)
pub fn door_path_point_at(g: &mut crate::game::Game, w: &crate::doorpath::Walk, s: f64) -> V3 { todo!() }

/// (src/game/doorpath.ts:284 DoorPath.advance)
pub fn door_path_advance(g: &mut crate::game::Game, w: &mut crate::doorpath::Walk) -> f64 { todo!() }

/// (src/game/doorpath.ts:292 DoorPath.toLine)
pub fn door_path_to_line(g: &mut crate::game::Game, w: &crate::doorpath::Walk) -> f64 { todo!() }

/// (src/game/doorpath.ts:293 DoorPath.depth)
pub fn door_path_depth(g: &mut crate::game::Game, d: DoorId, side: f64, p: &[f64]) -> f64 { todo!() }

/// (src/game/doorpath.ts:296 DoorPath.mayGo)
pub fn door_path_may_go(g: &mut crate::game::Game, op: DoorOpId, q: crate::doorpath::DoorPathMayGoQ, i: f64) -> bool { todo!() }

/// (src/game/doorpath.ts:307 DoorPath.sent)
pub fn door_path_sent(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, pod: crate::doorpath::DoorPathSentPod) { todo!() }

/// (src/game/doorpath.ts:316 DoorPath.own)
pub fn door_path_own(g: &mut crate::game::Game, w: &mut crate::doorpath::Walk) -> bool { todo!() }

/// (src/game/doorpath.ts:317 DoorPath.drop)
pub fn door_path_drop(g: &mut crate::game::Game, w: &mut crate::doorpath::Walk, why: crate::doorpath::DoorPathDropWhy) { todo!() }

/// (src/game/doorpath.ts:323 DoorPath.tick)
pub fn door_path_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/doorpath.ts:369 DoorPath.summary)
pub fn door_path_summary(g: &mut crate::game::Game) -> String { todo!() }
