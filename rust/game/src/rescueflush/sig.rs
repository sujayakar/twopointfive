#![allow(unused_variables)]
//! Every function and method of `rescueflush`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/rescueflush.ts:57 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/rescueflush.ts:69 RescueFlush.constructor)
pub fn rescue_flush_new(g: &mut crate::game::Game) -> crate::rescueflush::RescueFlush { todo!() }

/// (src/game/rescueflush.ts:71 RescueFlush.reset)
pub fn rescue_flush_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescueflush.ts:73 RescueFlush.live)
pub fn rescue_flush_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rescueflush.ts:74 RescueFlush.alive)
pub fn rescue_flush_alive(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/rescueflush.ts:76 RescueFlush.came)
pub fn rescue_flush_came(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/rescueflush.ts:81 RescueFlush.line)
pub fn rescue_flush_line(g: &mut crate::game::Game, by: GuardId, v: GuardId) -> f64 { todo!() }

/// (src/game/rescueflush.ts:90 RescueFlush.holder)
pub fn rescue_flush_holder(g: &mut crate::game::Game, keep: Option<GuardId>) -> Option<GuardId> { todo!() }

/// (src/game/rescueflush.ts:106 RescueFlush.victims)
pub fn rescue_flush_victims(g: &mut crate::game::Game, by: GuardId, of: &[GuardId]) -> Vec<GuardId> { todo!() }

/// (src/game/rescueflush.ts:111 RescueFlush.tick)
pub fn rescue_flush_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescueflush.ts:175 RescueFlush.goalOf)
pub fn rescue_flush_goal_of(g: &mut crate::game::Game, k: f64, in_room: bool) -> crate::rescueflush::Goal { todo!() }

/// (src/game/rescueflush.ts:179 RescueFlush.pointOf)
pub fn rescue_flush_point_of(g: &mut crate::game::Game, goal: crate::rescueflush::Goal, alive: &[GuardId]) -> Option<V3> { todo!() }

/// (src/game/rescueflush.ts:189 RescueFlush.send)
pub fn rescue_flush_send(g: &mut crate::game::Game, h: GuardId, goal: crate::rescueflush::Goal, at: V3, n: f64, via: Option<&[V3]>) { todo!() }

/// (src/game/rescueflush.ts:206 RescueFlush.snapshot)
pub fn rescue_flush_snapshot(g: &mut crate::game::Game) -> crate::rescueflush::RescueFlushSnap { todo!() }

/// (src/game/rescueflush.ts:207 RescueFlush.restore)
pub fn rescue_flush_restore(g: &mut crate::game::Game, s: &crate::rescueflush::RescueFlushSnap) { todo!() }
