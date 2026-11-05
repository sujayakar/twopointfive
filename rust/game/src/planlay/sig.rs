#![allow(unused_variables)]
//! Every function and method of `planlay`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/planlay.ts:57 copy)
pub fn copy(a: &[crate::planlay::LaidOrder]) -> Vec<crate::planlay::LaidOrder> { todo!() }

/// (src/game/planlay.ts:59 layKey)
pub fn lay_key(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/planlay.ts:63 layable)
pub fn layable(g: &mut crate::game::Game, d: OrdersOrderDefId) -> bool { todo!() }

/// (src/game/planlay.ts:65 layPending)
pub fn lay_pending(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/planlay.ts:68 layPrune)
pub fn lay_prune(g: &mut crate::game::Game) { todo!() }

/// (src/game/planlay.ts:76 startKey)
pub fn start_key(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/planlay.ts:78 layMen)
pub fn lay_men(g: &mut crate::game::Game, o: &crate::planlay::LaidOrder) -> Vec<GuardId> { todo!() }

/// (src/game/planlay.ts:80 layPoint)
pub fn lay_point(g: &mut crate::game::Game, o: &crate::planlay::LaidOrder) -> Option<V3> { todo!() }

/// (src/game/planlay.ts:85 targetOf)
pub fn target_of(g: &mut crate::game::Game, o: &crate::planlay::LaidWhere) -> Option<crate::planlay::TargetOf> { todo!() }

/// (src/game/planlay.ts:94 layTarget)
pub fn lay_target(g: &mut crate::game::Game, o: &crate::planlay::LaidWhere) -> Option<crate::planlay::LayTarget> { todo!() }

/// (src/game/planlay.ts:96 strike)
pub fn strike(l: &mut crate::planlay::LayState, names: &JsSet<String>) -> f64 { todo!() }

/// (src/game/planlay.ts:103 layOrder)
pub fn lay_order(g: &mut crate::game::Game, d: OrdersOrderDefId, t: &crate::orders::OrderTarget, men: &[GuardId]) -> Option<String> { todo!() }

/// (src/game/planlay.ts:127 laySend)
pub fn lay_send(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/planlay.ts:150 layGiven)
pub fn lay_given(g: &mut crate::game::Game, men: &[GuardId]) { todo!() }

/// (src/game/planlay.ts:155 layFresh)
pub fn lay_fresh(g: &mut crate::game::Game) { todo!() }

/// (src/game/planlay.ts:167 layClear)
pub fn lay_clear(g: &mut crate::game::Game) { todo!() }

/// (src/game/planlay.ts:170 layNote)
pub fn lay_note(g: &mut crate::game::Game, key: &str) -> Vec<crate::planlay::LayNoteItem> { todo!() }

/// (src/game/planlay.ts:183 laySummary)
pub fn lay_summary(g: &mut crate::game::Game) -> String { todo!() }
