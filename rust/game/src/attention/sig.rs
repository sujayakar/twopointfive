#![allow(unused_variables)]
//! Every function and method of `attention`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/attention.ts:100 mem)
pub fn mem<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::attention::Mem { todo!() }

/// (src/game/attention.ts:106 log)
pub fn log(g: &mut crate::game::Game, gd: GuardId, what: &str, h: crate::attention::LogH) { todo!() }

/// (src/game/attention.ts:115 attentionLines)
pub fn attention_lines() -> Vec<String> { todo!() }

/// (src/game/attention.ts:116 line)
pub fn line(g: &mut crate::game::Game, gd: GuardId, pool: &[String]) { todo!() }

/// (src/game/attention.ts:120 attentionOf)
pub fn attention_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::attention::AttentionOf> { todo!() }

/// (src/game/attention.ts:125 offCos)
pub fn off_cos(g: &mut crate::game::Game, gd: GuardId, h: crate::attention::Hold, p: V3) -> f64 { todo!() }

/// (src/game/attention.ts:133 outside)
pub fn outside(g: &mut crate::game::Game, gd: GuardId, h: crate::attention::Hold, p: V3) -> bool { todo!() }

/// (src/game/attention.ts:135 attentionOffDeg)
pub fn attention_off_deg(g: &mut crate::game::Game, gd: GuardId, p: V3) -> Option<f64> { todo!() }

/// (src/game/attention.ts:137 attentionSideK)
pub fn attention_side_k(g: &mut crate::game::Game, gd: GuardId, p: V3) -> f64 { todo!() }

/// (src/game/attention.ts:144 attentionAside)
pub fn attention_aside(g: &mut crate::game::Game, gd: GuardId, p: V3, gain: f64) -> bool { todo!() }

/// (src/game/attention.ts:156 attentionEarK)
pub fn attention_ear_k(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/attention.ts:157 attentionNoiseK)
pub fn attention_noise_k(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/attention.ts:159 attentionStep)
pub fn attention_step(g: &mut crate::game::Game, gd: GuardId, at: V3) { todo!() }

/// (src/game/attention.ts:165 attentionTold)
pub fn attention_told(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/attention.ts:168 attentionAgain)
pub fn attention_again(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/attention.ts:176 attentionHeard)
pub fn attention_heard(g: &mut crate::game::Game, gd: GuardId, e: crate::game::GameEvent, where_: V3) { todo!() }

/// (src/game/attention.ts:186 attentionLend)
pub fn attention_lend(g: &mut crate::game::Game, gd: GuardId, at: V3, o: Option<crate::attention::AttentionLendO>) { todo!() }

/// (src/game/attention.ts:193 able)
pub fn able(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/attention.ts:197 secsFor)
pub fn secs_for(g: &mut crate::game::Game, kind: crate::attention::DrawKind) -> f64 { todo!() }

/// (src/game/attention.ts:198 end)
pub fn end(g: &mut crate::game::Game, gd: GuardId, why: &str) { todo!() }

/// (src/game/attention.ts:205 attentionTick)
pub fn attention_tick(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/attention.ts:260 tell)
pub fn tell(g: &mut crate::game::Game, gd: GuardId, h: &mut crate::attention::Hold) { todo!() }

/// (src/game/attention.ts:269 attentionWalk)
pub fn attention_walk(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/attention.ts:277 attentionDwell)
pub fn attention_dwell(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/attention.ts:289 attentionMarks)
pub fn attention_marks(g: &mut crate::game::Game) -> Vec<crate::attention::AttentionMark> { todo!() }

/// (src/game/attention.ts:308 AttentionWayStore.get)
pub fn attention_way_store_get(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::attention::AttentionWayKept> { todo!() }

/// (src/game/attention.ts:308 AttentionWayStore.set)
pub fn attention_way_store_set(g: &mut crate::game::Game, gd: GuardId, w: &crate::attention::AttentionWayKept) -> crate::Opaque { todo!() }

/// (src/game/attention.ts:309 attentionWay)
pub fn attention_way(g: &mut crate::game::Game, m: crate::attention::AttentionMark) -> Vec<V3> { todo!() }

/// (src/game/attention.ts:355 attentionSummary)
pub fn attention_summary(g: &mut crate::game::Game) -> String { todo!() }
