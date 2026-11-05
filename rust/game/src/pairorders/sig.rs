#![allow(unused_variables)]
//! Every function and method of `pairorders`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/pairorders.ts:116 flags)
pub fn flags(g: &mut crate::game::Game) { todo!() }

/// (src/game/pairorders.ts:125 nameOf)
pub fn name_of(g: &mut crate::game::Game, h: GuardId) -> String { todo!() }

/// (src/game/pairorders.ts:127 lookOf)
pub fn look_of(g: &mut crate::game::Game, h: GuardId) -> [f64; 2] { todo!() }

/// (src/game/pairorders.ts:156 PairOrders.constructor)
pub fn pair_orders_new(g: &mut crate::game::Game) -> crate::pairorders::PairOrders { todo!() }

/// (src/game/pairorders.ts:172 PairOrders.up)
pub fn pair_orders_up(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/pairorders.ts:174 PairOrders.quiet)
pub fn pair_orders_quiet(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/pairorders.ts:176 PairOrders.isFoe)
pub fn pair_orders_is_foe(g: &mut crate::game::Game, h: GuardId) -> bool { todo!() }

/// (src/game/pairorders.ts:178 PairOrders.noticed)
pub fn pair_orders_noticed(g: &mut crate::game::Game, h: GuardId) -> bool { todo!() }

/// (src/game/pairorders.ts:180 PairOrders.waiting)
pub fn pair_orders_waiting(g: &mut crate::game::Game) -> Vec<(GuardId, crate::pairorders::Job)> { todo!() }

/// (src/game/pairorders.ts:181 PairOrders.jobOf)
pub fn pair_orders_job_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::pairorders::PairOrdersJobOf> { todo!() }

/// (src/game/pairorders.ts:183 PairOrders.gripped)
pub fn pair_orders_gripped(g: &mut crate::game::Game, h: GuardId) -> bool { todo!() }

/// (src/game/pairorders.ts:185 PairOrders.caught)
pub fn pair_orders_caught(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/pairorders.ts:187 PairOrders.finishing)
pub fn pair_orders_finishing(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/pairorders.ts:189 PairOrders.lastPaces)
pub fn pair_orders_last_paces(g: &mut crate::game::Game, j: crate::pairorders::Job, gd: GuardId) -> bool { todo!() }

/// (src/game/pairorders.ts:192 PairOrders.gone)
pub fn pair_orders_gone(g: &mut crate::game::Game, h: GuardId) -> Option<String> { todo!() }

/// (src/game/pairorders.ts:194 PairOrders.sees)
pub fn pair_orders_sees(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool { todo!() }

/// (src/game/pairorders.ts:196 PairOrders.rear)
pub fn pair_orders_rear(g: &mut crate::game::Game, h: GuardId, p: V3, mostly: Option<bool>) -> bool { todo!() }

/// (src/game/pairorders.ts:197 PairOrders.note)
pub fn pair_orders_note(g: &mut crate::game::Game, what: &str) { todo!() }

/// (src/game/pairorders.ts:200 PairOrders.rows)
pub fn pair_orders_rows(g: &mut crate::game::Game) -> Vec<OrdersOrderDefId> { todo!() }

/// (src/game/pairorders.ts:251 PairOrders.goRow)
pub fn pair_orders_go_row(g: &mut crate::game::Game) -> OrdersOrderDefId { todo!() }

/// (src/game/pairorders.ts:258 PairOrders.quietFirst)
pub fn pair_orders_quiet_first(g: &mut crate::game::Game) { todo!() }

/// (src/game/pairorders.ts:261 PairOrders.seat)
pub fn pair_orders_seat(g: &mut crate::game::Game) { todo!() }

/// (src/game/pairorders.ts:276 PairOrders.reset)
pub fn pair_orders_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/pairorders.ts:282 PairOrders.input)
pub fn pair_orders_input(g: &mut crate::game::Game, input: InputId) { todo!() }

/// (src/game/pairorders.ts:288 PairOrders.tick)
pub fn pair_orders_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/pairorders.ts:313 PairOrders.mark)
pub fn pair_orders_mark(g: &mut crate::game::Game, by: Option<crate::pairorders::PairOrdersMarkBy>) -> bool { todo!() }

/// (src/game/pairorders.ts:332 PairOrders.watcher)
pub fn pair_orders_watcher(g: &mut crate::game::Game, foe: GuardId, gd: GuardId) -> Option<GuardId> { todo!() }

/// (src/game/pairorders.ts:345 PairOrders.bodyLit)
pub fn pair_orders_body_lit(g: &mut crate::game::Game, o: GuardId, foe: GuardId, at: V3) -> bool { todo!() }

/// (src/game/pairorders.ts:353 PairOrders.looksBack)
pub fn pair_orders_looks_back(g: &mut crate::game::Game, w: GuardId, foe: GuardId, gd: GuardId) -> bool { todo!() }

/// (src/game/pairorders.ts:365 PairOrders.held)
pub fn pair_orders_held(g: &mut crate::game::Game, j: crate::pairorders::Job, gd: GuardId) -> bool { todo!() }

/// (src/game/pairorders.ts:367 PairOrders.stand)
pub fn pair_orders_stand(g: &mut crate::game::Game, gd: GuardId, words: &str, aim: Option<GuardId>) { todo!() }

/// (src/game/pairorders.ts:374 PairOrders.yawOf)
pub fn pair_orders_yaw_of(g: &mut crate::game::Game, foe: GuardId) -> f64 { todo!() }

/// (src/game/pairorders.ts:375 PairOrders.back)
pub fn pair_orders_back(g: &mut crate::game::Game, foe: GuardId, m: f64) -> V3 { todo!() }

/// (src/game/pairorders.ts:377 PairOrders.backOpen)
pub fn pair_orders_back_open(g: &mut crate::game::Game, foe: GuardId, p: V3) -> bool { todo!() }

/// (src/game/pairorders.ts:380 PairOrders.behind)
pub fn pair_orders_behind(g: &mut crate::game::Game, foe: GuardId) -> V3 { todo!() }

/// (src/game/pairorders.ts:382 PairOrders.stage)
pub fn pair_orders_stage(g: &mut crate::game::Game, foe: GuardId) -> V3 { todo!() }

/// (src/game/pairorders.ts:388 PairOrders.takeTick)
pub fn pair_orders_take_tick(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) { todo!() }

/// (src/game/pairorders.ts:489 PairOrders.clearOf)
pub fn pair_orders_clear_of(g: &mut crate::game::Game, a: V3, b: V3) -> bool { todo!() }

/// (src/game/pairorders.ts:493 PairOrders.stalk)
pub fn pair_orders_stalk(g: &mut crate::game::Game, gd: GuardId, to: V3, pace: Option<f64>) -> crate::pairorders::PairOrdersStalk { todo!() }

/// (src/game/pairorders.ts:576 PairOrders.litCell)
pub fn pair_orders_lit_cell(g: &mut crate::game::Game, x: f64, z: f64) -> f64 { todo!() }

/// (src/game/pairorders.ts:579 PairOrders.litAt)
pub fn pair_orders_lit_at(g: &mut crate::game::Game, p: V3) -> f64 { todo!() }

/// (src/game/pairorders.ts:582 PairOrders.darkSpot)
pub fn pair_orders_dark_spot(g: &mut crate::game::Game, at: V3) -> Option<V3> { todo!() }

/// (src/game/pairorders.ts:597 PairOrders.darkTick)
pub fn pair_orders_dark_tick(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) { todo!() }

/// (src/game/pairorders.ts:607 PairOrders.summary)
pub fn pair_orders_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/pairorders.ts:618 pairPasses)
pub fn pair_passes(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool { todo!() }

/// (src/game/pairorders.ts:627 pairGripBody)
pub fn pair_grip_body(g: &mut crate::game::Game, h: GuardId) -> bool { todo!() }

/// (src/game/pairorders.ts:629 pairGripped)
pub fn pair_gripped(g: &mut crate::game::Game, h: GuardId) -> bool { todo!() }

/// (src/game/pairorders.ts:631 pairHolds)
pub fn pair_holds(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool { todo!() }
