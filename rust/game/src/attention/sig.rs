#![allow(unused_variables)]
//! Every function and method of `attention`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/attention.ts:101 mem)
pub fn mem<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::attention::Mem { todo!() }

/// (src/game/attention.ts:107 log)
pub fn log(g: &mut crate::game::Game, gd: GuardId, what: &str, h: crate::attention::LogH) { todo!() }

/// (src/game/attention.ts:116 attentionLines)
pub fn attention_lines() -> Vec<String> { todo!() }

/// (src/game/attention.ts:117 line)
pub fn line(g: &mut crate::game::Game, gd: GuardId, pool: &[String]) { todo!() }

/// (src/game/attention.ts:121 attentionOf)
pub fn attention_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::attention::Hold> { todo!() }

/// (src/game/attention.ts:126 offCos)
pub fn off_cos(g: &mut crate::game::Game, gd: GuardId, h: crate::attention::Hold, p: V3) -> f64 { todo!() }

/// (src/game/attention.ts:134 outside)
pub fn outside(g: &mut crate::game::Game, gd: GuardId, h: crate::attention::Hold, p: V3) -> bool { todo!() }

/// (src/game/attention.ts:136 attentionOffDeg)
pub fn attention_off_deg(g: &mut crate::game::Game, gd: GuardId, p: V3) -> Option<f64> { todo!() }

/// (src/game/attention.ts:138 attentionSideK)
pub fn attention_side_k(g: &mut crate::game::Game, gd: GuardId, p: V3) -> f64 { todo!() }

/// (src/game/attention.ts:145 attentionAside)
pub fn attention_aside(g: &mut crate::game::Game, gd: GuardId, p: V3, gain: f64) -> bool { todo!() }

/// (src/game/attention.ts:157 attentionEarK)
pub fn attention_ear_k(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/attention.ts:158 attentionNoiseK)
pub fn attention_noise_k(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/attention.ts:160 attentionStep)
pub fn attention_step(g: &mut crate::game::Game, gd: GuardId, at: V3) { todo!() }

/// (src/game/attention.ts:166 attentionTold)
pub fn attention_told(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/attention.ts:169 attentionAgain)
pub fn attention_again(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/attention.ts:177 attentionHeard)
pub fn attention_heard(g: &mut crate::game::Game, gd: GuardId, e: crate::game::GameEvent, where_: V3) { todo!() }

/// (src/game/attention.ts:187 attentionLend)
pub fn attention_lend(g: &mut crate::game::Game, gd: GuardId, at: V3, o: Option<crate::attention::AttentionLendO>) { todo!() }

/// (src/game/attention.ts:194 able)
pub fn able(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/attention.ts:198 secsFor)
pub fn secs_for(g: &mut crate::game::Game, kind: crate::attention::DrawKind) -> f64 { todo!() }

/// (src/game/attention.ts:199 end)
pub fn end(g: &mut crate::game::Game, gd: GuardId, why: &str) { todo!() }

/// (src/game/attention.ts:206 attentionTick)
pub fn attention_tick(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/attention.ts:261 tell)
pub fn tell(g: &mut crate::game::Game, gd: GuardId, h: &mut crate::attention::Hold) { todo!() }

/// (src/game/attention.ts:270 attentionWalk)
pub fn attention_walk(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/attention.ts:278 attentionDwell)
pub fn attention_dwell(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/attention.ts:290 attentionMarks)
pub fn attention_marks(g: &mut crate::game::Game) -> Vec<crate::attention::AttentionMark> { todo!() }

/// (src/game/attention.ts:309 AttentionWayStore.get)
pub fn attention_way_store_get(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::attention::AttentionWayKept> { todo!() }

/// (src/game/attention.ts:309 AttentionWayStore.set)
pub fn attention_way_store_set(g: &mut crate::game::Game, gd: GuardId, w: &crate::attention::AttentionWayKept) -> crate::Opaque { todo!() }

/// (src/game/attention.ts:356 attentionSummary)
pub fn attention_summary(g: &mut crate::game::Game) -> String { todo!() }
