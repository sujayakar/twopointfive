#![allow(unused_variables)]
//! Every function and method of `squadthrow`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/squadthrow.ts:139 squadThrowKept)
pub fn squad_throw_kept() -> crate::squadthrow::SquadThrowKept { todo!() }

/// (src/game/squadthrow.ts:146 aside)
pub fn aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, pl: PlayerId, v: &mut crate::squadthrow::SquadThrowKept, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/squadthrow.ts:162 keyOf)
pub fn key_of(at: V3) -> String { todo!() }

/// (src/game/squadthrow.ts:163 left)
pub fn left(g: &mut crate::game::Game, kind: crate::squadthrow::SquadThrowKind) -> f64 { todo!() }

/// (src/game/squadthrow.ts:164 word)
pub fn word(kind: crate::squadthrow::SquadThrowKind) -> crate::squadthrow::Word { todo!() }

/// (src/game/squadthrow.ts:165 none)
pub fn none(g: &mut crate::game::Game, kind: crate::squadthrow::SquadThrowKind) -> crate::squadthrow::None { todo!() }

/// (src/game/squadthrow.ts:166 rowOf)
pub fn row_of(kind: crate::squadthrow::SquadThrowKind) -> crate::squadthrow::RowOf { todo!() }

/// (src/game/squadthrow.ts:167 rollOf)
pub fn roll_of(g: &mut crate::game::Game, kind: crate::squadthrow::SquadThrowKind) -> Option<f64> { todo!() }

/// (src/game/squadthrow.ts:169 tossing)
pub fn tossing(g: &mut crate::game::Game, s: Option<&crate::squadthrow::State>, gd: GuardId) -> bool { todo!() }

/// (src/game/squadthrow.ts:173 handsBusy)
pub fn hands_busy(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/squadthrow.ts:181 squadThrower)
pub fn squad_thrower(g: &mut crate::game::Game, men: &[GuardId], at: V3, kind: crate::squadthrow::SquadThrowKind) -> Option<crate::squadthrow::SquadThrower2> { todo!() }

/// (src/game/squadthrow.ts:187 standThrow)
pub fn stand_throw(g: &mut crate::game::Game, men: &[GuardId], at: V3, kind: crate::squadthrow::SquadThrowKind, lob: Option<&mut crate::squadthrow::Lob>) -> Option<crate::squadthrow::StandThrow> { todo!() }

/// (src/game/squadthrow.ts:203 fragClears)
pub fn frag_clears(g: &mut crate::game::Game, gd: GuardId, p: V3, pts: &[V3], late: Option<f64>, taken: Option<&mut Vec<V3>>) -> bool { todo!() }

/// (src/game/squadthrow.ts:222 fragPts)
pub fn frag_pts(rest: V3, at: V3) -> Vec<V3> { todo!() }

/// (src/game/squadthrow.ts:224 fragIn)
pub fn frag_in(g: &mut crate::game::Game, pts: &[V3], p: V3, head: Option<f64>, reach: Option<f64>) -> bool { todo!() }

/// (src/game/squadthrow.ts:227 stunPts)
pub fn stun_pts(rest: V3, at: V3) -> Vec<V3> { todo!() }

/// (src/game/squadthrow.ts:229 stunIn)
pub fn stun_in(g: &mut crate::game::Game, pts: &[V3], eye: V3, full: bool) -> bool { todo!() }

/// (src/game/squadthrow.ts:247 mayWalk)
pub fn may_walk(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/squadthrow.ts:261 walkSpot)
pub fn walk_spot(g: &mut crate::game::Game, men: &[GuardId], at: V3, kind: crate::squadthrow::SquadThrowKind, forced: Option<bool>, lob: Option<&mut crate::squadthrow::Lob>) -> Option<crate::squadthrow::SquadThrower> { todo!() }

/// (src/game/squadthrow.ts:348 squadThrowPlan)
pub fn squad_throw_plan(g: &mut crate::game::Game, men: &[GuardId], at: V3, kind: crate::squadthrow::SquadThrowKind, fresh: Option<bool>, view: Option<bool>) -> Option<crate::squadthrow::SquadThrower> { todo!() }

/// (src/game/squadthrow.ts:367 walkLine)
pub fn walk_line(g: &mut crate::game::Game, gd: GuardId, post: V3) -> Vec<V3> { todo!() }

/// (src/game/squadthrow.ts:380 squadFragCatches)
pub fn squad_frag_catches(g: &mut crate::game::Game, rest: V3, at: V3, moved: Option<crate::squadthrow::SquadFragCatchesMoved>, sam: Option<bool>, kept: Option<bool>, burst: Option<&crate::squadthrow::SquadFragCatchesBurst>) -> Option<String> { todo!() }

/// (src/game/squadthrow.ts:387 fragCatches)
pub fn frag_catches(g: &mut crate::game::Game, rest: V3, at: V3, moved: Option<crate::squadthrow::FragCatchesMoved>, sam: bool, burst: Option<&crate::squadthrow::FragCatchesBurst>) -> Option<String> { todo!() }

/// (src/game/squadthrow.ts:407 squadStunCatches)
pub fn squad_stun_catches(g: &mut crate::game::Game, rest: V3, at: V3) -> Vec<String> { todo!() }

/// (src/game/squadthrow.ts:417 samCalls)
pub fn sam_calls(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/squadthrow.ts:419 squadFragOnSam)
pub fn squad_frag_on_sam(g: &mut crate::game::Game, rest: V3, at: V3) -> bool { todo!() }

/// (src/game/squadthrow.ts:421 samNear)
pub fn sam_near(g: &mut crate::game::Game, pts: &[V3]) -> bool { todo!() }

/// (src/game/squadthrow.ts:422 state)
pub fn state<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::squadthrow::State { todo!() }

/// (src/game/squadthrow.ts:425 throwIt)
pub fn throw_it(g: &mut crate::game::Game, gd: GuardId, arc: &crate::squadthrow::Arc, kind: crate::squadthrow::SquadThrowKind, at: V3) { todo!() }

/// (src/game/squadthrow.ts:434 squadLogRow)
pub fn squad_log_row(g: &mut crate::game::Game, gd: GuardId, order: &str, at: V3) { todo!() }

/// (src/game/squadthrow.ts:437 walkTo)
pub fn walk_to(g: &mut crate::game::Game, gd: GuardId, kind: crate::squadthrow::SquadThrowKind, post: V3, at: V3, arc: Option<&crate::squadthrow::Arc>) -> Option<OrdersAssignmentId> { todo!() }

/// (src/game/squadthrow.ts:442 startWalk)
pub fn start_walk(g: &mut crate::game::Game, th: &mut crate::squadthrow::SquadThrower, t: &crate::orders::OrderTarget, kind: crate::squadthrow::SquadThrowKind) { todo!() }

/// (src/game/squadthrow.ts:449 squadLend)
pub fn squad_lend(g: &mut crate::game::Game, gd: GuardId, t: &crate::orders::OrderTarget) -> crate::squadthrow::Lent { todo!() }

/// (src/game/squadthrow.ts:458 squadHandBack)
pub fn squad_hand_back(g: &mut crate::game::Game, w: &mut crate::squadthrow::Lent, back: Option<bool>) { todo!() }

/// (src/game/squadthrow.ts:469 walkTick)
pub fn walk_tick(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) { todo!() }

/// (src/game/squadthrow.ts:519 squadThrowPlans)
pub fn squad_throw_plans(g: &mut crate::game::Game, v: Option<&mut crate::squadthrow::SquadThrowKept>) -> Vec<crate::squadthrow::SquadThrowPlan> { todo!() }

/// (src/game/squadthrow.ts:539 rows)
pub fn rows(g: &mut crate::game::Game) -> Vec<OrdersOrderDefId> { todo!() }

/// (src/game/squadthrow.ts:576 notes)
pub fn notes(g: &mut crate::game::Game) -> Vec<crate::orders::OrderNote> { todo!() }

/// (src/game/squadthrow.ts:579 sync)
pub fn sync(g: &mut crate::game::Game) { todo!() }

/// (src/game/squadthrow.ts:590 squadThrowKey)
pub fn squad_throw_key(g: &mut crate::game::Game, input: InputId) -> bool { todo!() }

/// (src/game/squadthrow.ts:604 squadThrowSummary)
pub fn squad_throw_summary(g: &mut crate::game::Game) -> String { todo!() }
