#![allow(unused_variables)]
//! Every function and method of `cover`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/cover.ts:197 gapsOf)
pub fn gaps_of(nav: &crate::cover::GapsOfNav, along_x: bool, k: Option<crate::knobs::CoverK>, solid: Option<&mut dyn FnMut(V3, V3) -> bool>) -> Vec<CoverOpeningId> { todo!() }

/// (src/game/cover.ts:267 Cover.constructor)
pub fn cover_new(g: &mut crate::game::Game) -> crate::cover::Cover { todo!() }

/// (src/game/cover.ts:270 Cover.reset)
pub fn cover_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/cover.ts:271 Cover.of)
pub fn cover_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::cover::CoverJob> { todo!() }

/// (src/game/cover.ts:273 Cover.keptJob)
pub fn cover_kept_job(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::cover::CoverJob> { todo!() }

/// (src/game/cover.ts:276 Cover.ready)
pub fn cover_ready(g: &mut crate::game::Game) -> Vec<CoverOpeningId> { todo!() }

/// (src/game/cover.ts:278 Cover.build)
pub fn cover_build(g: &mut crate::game::Game) { todo!() }

/// (src/game/cover.ts:300 Cover.spare)
pub fn cover_spare(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) -> bool { todo!() }

/// (src/game/cover.ts:308 Cover.free)
pub fn cover_free(g: &mut crate::game::Game, gd: GuardId) -> Option<OrdersAssignmentId> { todo!() }

/// (src/game/cover.ts:321 Cover.readRear)
pub fn cover_read_rear(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId, list: &[(GuardId, OrdersAssignmentId)], bodies: &[V3]) -> Option<crate::roomfit::Watchable> { todo!() }

/// (src/game/cover.ts:338 Cover.walkOnly)
pub fn cover_walk_only(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) -> bool { todo!() }

/// (src/game/cover.ts:340 Cover.walkDoor)
pub fn cover_walk_door(g: &mut crate::game::Game, o: CoverOpeningId, a: OrdersAssignmentId) -> bool { todo!() }

/// (src/game/cover.ts:345 Cover.eyeOf)
pub fn cover_eye_of(g: &mut crate::game::Game, p: V3) -> V3 { todo!() }

/// (src/game/cover.ts:347 Cover.sightFrom)
pub fn cover_sight_from(g: &mut crate::game::Game, p: V3, o: CoverOpeningId) -> Option<V3> { todo!() }

/// (src/game/cover.ts:349 Cover.line)
pub fn cover_line(g: &mut crate::game::Game, eye: V3, o: CoverOpeningId) -> bool { todo!() }

/// (src/game/cover.ts:352 Cover.sight)
pub fn cover_sight(g: &mut crate::game::Game, eye: V3, o: CoverOpeningId) -> Option<V3> { todo!() }

/// (src/game/cover.ts:363 Cover.tick)
pub fn cover_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/cover.ts:377 Cover.deal)
pub fn cover_deal(g: &mut crate::game::Game) { todo!() }

/// (src/game/cover.ts:540 Cover.stepDeal)
pub fn cover_step_deal(g: &mut crate::game::Game, list: &[(GuardId, OrdersAssignmentId)], asg_of: &JsMap<GuardId, OrdersAssignmentId>, got: &JsMap<GuardId, CoverOpeningId>, live: &[crate::cover::LiveOpening], open: &[CoverOpeningId], bodies: &[(CharId, V3)], spd: &JsMap<GuardId, f64>, w_of: &mut dyn FnMut(&mut crate::game::Game, CoverOpeningId) -> f64, rear_man: Option<GuardId>) { todo!() }

/// (src/game/cover.ts:584 Cover.pruneSteps)
pub fn cover_prune_steps(g: &mut crate::game::Game) { todo!() }

/// (src/game/cover.ts:592 Cover.stepSpot)
pub fn cover_step_spot(g: &mut crate::game::Game, gd: GuardId, from: V3, o: CoverOpeningId, bodies: &[(CharId, V3)]) -> Option<crate::cover::CoverStepSpot> { todo!() }

/// (src/game/cover.ts:618 Cover.stepOf)
pub fn cover_step_of(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) -> Option<V3> { todo!() }

/// (src/game/cover.ts:620 Cover.stepHalt)
pub fn cover_step_halt(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/cover.ts:624 Cover.note)
pub fn cover_note(g: &mut crate::game::Game, gd: GuardId, id_2: Option<&str>) { todo!() }

/// (src/game/cover.ts:627 Cover.drop)
pub fn cover_drop(g: &mut crate::game::Game, gd: GuardId, taken: Option<bool>) { todo!() }

/// (src/game/cover.ts:637 Cover.release)
pub fn cover_release(g: &mut crate::game::Game, gd: GuardId, j: crate::cover::CoverJob) { todo!() }

/// (src/game/cover.ts:643 Cover.turnFrom)
pub fn cover_turn_from(g: &mut crate::game::Game, gd: GuardId, j: crate::cover::CoverJob) { todo!() }

/// (src/game/cover.ts:645 Cover.wayOf)
pub fn cover_way_of(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/cover.ts:647 Cover.turning)
pub fn cover_turning(g: &mut crate::game::Game, gd: GuardId, still: bool, pace: f64) -> bool { todo!() }

/// (src/game/cover.ts:652 Cover.turnOut)
pub fn cover_turn_out(g: &mut crate::game::Game, gd: GuardId, still: bool, pace: f64) -> Option<crate::cover::CoverTurnOut> { todo!() }

/// (src/game/cover.ts:660 Cover.unturn)
pub fn cover_unturn(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/cover.ts:667 Cover.unkeep)
pub fn cover_unkeep(g: &mut crate::game::Game, gd: GuardId, taken: Option<bool>) { todo!() }

/// (src/game/cover.ts:672 Cover.keptOf)
pub fn cover_kept_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::cover::CoverJob> { todo!() }

/// (src/game/cover.ts:685 Cover.dropAll)
pub fn cover_drop_all(g: &mut crate::game::Game) { todo!() }

/// (src/game/cover.ts:690 Cover.poseFor)
pub fn cover_pose_for(g: &mut crate::game::Game, gd: GuardId, still: bool, pace: f64) -> Option<crate::cover::CoverPoseFor> { todo!() }

/// (src/game/cover.ts:719 Cover.nameOf)
pub fn cover_name_of(g: &mut crate::game::Game, o: CoverOpeningId, from: Option<V3>) -> String { todo!() }

/// (src/game/cover.ts:728 Cover.has)
pub fn cover_has(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/cover.ts:730 Cover.onOf)
pub fn cover_on_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::cover::CoverJob> { todo!() }

/// (src/game/cover.ts:732 Cover.words)
pub fn cover_words(g: &mut crate::game::Game, gd: GuardId) -> String { todo!() }

/// (src/game/cover.ts:733 Cover.tail)
pub fn cover_tail(g: &mut crate::game::Game, gd: GuardId) -> String { todo!() }

/// (src/game/cover.ts:735 Cover.facing)
pub fn cover_facing(g: &mut crate::game::Game, gd: GuardId, deg: Option<f64>) -> bool { todo!() }

/// (src/game/cover.ts:737 Cover.summary)
pub fn cover_summary(g: &mut crate::game::Game) -> String { todo!() }
