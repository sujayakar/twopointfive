#![allow(unused_variables)]
//! Every function and method of `coverfire`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/coverfire.ts:88 no)
pub fn no(g: &mut crate::game::Game, why: &str) { todo!() }

/// (src/game/coverfire.ts:91 bump)
pub fn bump(g: &mut crate::game::Game, k: crate::coverfire::BumpK) { todo!() }

/// (src/game/coverfire.ts:104 chewLive)
pub fn chew_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/coverfire.ts:105 live)
pub fn live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/coverfire.ts:109 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::coverfire::State { todo!() }

/// (src/game/coverfire.ts:115 manOf)
pub fn man_of<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::coverfire::Man { todo!() }

/// (src/game/coverfire.ts:116 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/coverfire.ts:117 charOf)
pub fn char_of(g: &mut crate::game::Game, who: crate::coverfire::Who) -> CharId { todo!() }

/// (src/game/coverfire.ts:118 upOf)
pub fn up_of(g: &mut crate::game::Game, who: crate::coverfire::Who) -> bool { todo!() }

/// (src/game/coverfire.ts:119 nameOf)
pub fn name_of(g: &mut crate::game::Game, who: crate::coverfire::Who) -> String { todo!() }

/// (src/game/coverfire.ts:120 log)
pub fn log(g: &mut crate::game::Game, s: &str) { todo!() }

/// (src/game/coverfire.ts:122 weightOf)
pub fn weight_of(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/coverfire.ts:125 see)
pub fn see(g: &mut crate::game::Game, gd: GuardId, who: crate::coverfire::Who, low: bool) { todo!() }

/// (src/game/coverfire.ts:133 heard)
pub fn heard(g: &mut crate::game::Game, gd: GuardId, k: crate::coverfire::Know) { todo!() }

/// (src/game/coverfire.ts:135 known)
pub fn known(g: &mut crate::game::Game, gd: GuardId, who: Option<crate::coverfire::KnownWho>) -> Vec<crate::coverfire::KnownItem> { todo!() }

/// (src/game/coverfire.ts:150 ask)
pub fn ask(g: &mut crate::game::Game, gd: GuardId, to: V3) -> Option<crate::coverfire::Ask2> { todo!() }

/// (src/game/coverfire.ts:187 coverLane)
pub fn cover_lane(g: &mut crate::game::Game, gd: GuardId, from: V3, to: V3) -> bool { todo!() }

/// (src/game/coverfire.ts:202 worth)
pub fn worth(g: &mut crate::game::Game, gd: GuardId, to: V3, rounds: f64) -> bool { todo!() }

/// (src/game/coverfire.ts:208 budgetOf)
pub fn budget_of(g: &mut crate::game::Game, kind: &str, rounds: f64) -> f64 { todo!() }

/// (src/game/coverfire.ts:209 stays)
pub fn stays(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/coverfire.ts:214 choose)
pub fn choose(g: &mut crate::game::Game, gd: GuardId, k: crate::coverfire::Know, call: bool, seen: bool) -> Option<crate::coverfire::Chew> { todo!() }

/// (src/game/coverfire.ts:254 coverSeen)
pub fn cover_seen(g: &mut crate::game::Game, gd: GuardId, who: Option<crate::coverfire::CoverSeenWho>, to: V3) -> bool { todo!() }

/// (src/game/coverfire.ts:263 coverBlind)
pub fn cover_blind(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> Option<V3> { todo!() }

/// (src/game/coverfire.ts:291 coverOp)
pub fn cover_op(g: &mut crate::game::Game, o: &mut crate::coverfire::OpLike, dt: f64, speed: f64, point: &mut dyn FnMut(&mut crate::game::Game, V3) -> f64) -> bool { todo!() }

/// (src/game/coverfire.ts:319 coverfireTick)
pub fn coverfire_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/coverfire.ts:373 coverfireReport)
pub fn coverfire_report(g: &mut crate::game::Game) -> JsMap<String, f64> { todo!() }

/// (src/game/coverfire.ts:381 coverfireSaw)
pub fn coverfire_saw(g: &mut crate::game::Game, gd: GuardId, who: crate::coverfire::Who, age: Option<f64>) { todo!() }

/// (src/game/coverfire.ts:386 coverfireOn)
pub fn coverfire_on(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::coverfire::CoverfireOn> { todo!() }

/// (src/game/coverfire.ts:390 coverfireReset)
pub fn coverfire_reset(g: &mut crate::game::Game) { todo!() }
