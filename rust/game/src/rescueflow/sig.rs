#![allow(unused_variables)]
//! Every function and method of `rescueflow`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/rescueflow.ts:71 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/rescueflow.ts:80 RescueFlow.F)
pub fn rescue_flow_f(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::RescueFlowData> { todo!() }

/// (src/game/rescueflow.ts:81 RescueFlow.route)
pub fn rescue_flow_route(g: &mut crate::game::Game) -> Vec<crate::scene_scenarios::RouteOpening> { todo!() }

/// (src/game/rescueflow.ts:96 RescueFlow.constructor)
pub fn rescue_flow_new(g: &mut crate::game::Game) -> crate::rescueflow::RescueFlow { todo!() }

/// (src/game/rescueflow.ts:99 RescueFlow.reset)
pub fn rescue_flow_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescueflow.ts:104 RescueFlow.gave)
pub fn rescue_flow_gave(g: &mut crate::game::Game, a: Option<OrdersAssignmentId>) -> bool { todo!() }

/// (src/game/rescueflow.ts:106 RescueFlow.release)
pub fn rescue_flow_release(g: &mut crate::game::Game, men: &[GuardId]) { todo!() }

/// (src/game/rescueflow.ts:107 RescueFlow.stateOf)
pub fn rescue_flow_state_of(g: &mut crate::game::Game, id_2: &str) -> Option<crate::rescueflow::OpenState> { todo!() }

/// (src/game/rescueflow.ts:108 RescueFlow.jobOf)
pub fn rescue_flow_job_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::rescueflow::RescueFlowJobOf> { todo!() }

/// (src/game/rescueflow.ts:110 RescueFlow.build)
pub fn rescue_flow_build(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescueflow.ts:124 RescueFlow.lookable)
pub fn rescue_flow_lookable(g: &mut crate::game::Game, k: &mut crate::rescueflow::Known) { todo!() }

/// (src/game/rescueflow.ts:130 RescueFlow.open)
pub fn rescue_flow_open(g: &mut crate::game::Game, o: &crate::scene_scenarios::RouteOpening) -> bool { todo!() }

/// (src/game/rescueflow.ts:133 RescueFlow.eyes)
pub fn rescue_flow_eyes(g: &mut crate::game::Game) -> Vec<V3> { todo!() }

/// (src/game/rescueflow.ts:141 RescueFlow.sees)
pub fn rescue_flow_sees(g: &mut crate::game::Game, eyes: &[V3], h: GuardId) -> bool { todo!() }

/// (src/game/rescueflow.ts:147 RescueFlow.regionAt)
pub fn rescue_flow_region_at(g: &mut crate::game::Game, p: V3) -> Option<RescueflowRegionId> { todo!() }

/// (src/game/rescueflow.ts:149 RescueFlow.sense)
pub fn rescue_flow_sense(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescueflow.ts:200 RescueFlow.holder)
pub fn rescue_flow_holder(g: &mut crate::game::Game, o: &crate::scene_scenarios::RouteOpening) -> Option<GuardId> { todo!() }

/// (src/game/rescueflow.ts:212 RescueFlow.tick)
pub fn rescue_flow_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescueflow.ts:301 RescueFlow.settled)
pub fn rescue_flow_settled(g: &mut crate::game::Game, o: &crate::scene_scenarios::RouteOpening) -> bool { todo!() }

/// (src/game/rescueflow.ts:306 RescueFlow.stays)
pub fn rescue_flow_stays(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rescueflow.ts:308 RescueFlow.free)
pub fn rescue_flow_free(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/rescueflow.ts:318 RescueFlow.take)
pub fn rescue_flow_take(g: &mut crate::game::Game, gd: GuardId, o: &crate::scene_scenarios::RouteOpening, chain: &[crate::scene_scenarios::RouteOpening], said_wait: bool, leap: bool) { todo!() }

/// (src/game/rescueflow.ts:327 RescueFlow.rejoin)
pub fn rescue_flow_rejoin(g: &mut crate::game::Game, gd: GuardId, words: Option<&str>) { todo!() }

/// (src/game/rescueflow.ts:334 RescueFlow.dropAll)
pub fn rescue_flow_drop_all(g: &mut crate::game::Game, back: bool) { todo!() }

/// (src/game/rescueflow.ts:340 RescueFlow.boxes)
pub fn rescue_flow_boxes(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescueflow.ts:347 RescueFlow.word)
pub fn rescue_flow_word(g: &mut crate::game::Game, k: &crate::rescueflow::Known) -> String { todo!() }

/// (src/game/rescueflow.ts:358 RescueFlow.line)
pub fn rescue_flow_line(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/rescueflow.ts:363 RescueFlow.summary)
pub fn rescue_flow_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/rescueflow.ts:370 RescueFlow.check)
pub fn rescue_flow_check(g: &mut crate::game::Game) -> Vec<String> { todo!() }
