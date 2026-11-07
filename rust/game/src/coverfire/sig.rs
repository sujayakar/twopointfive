#![allow(unused_variables)]
//! Every function and method of `coverfire`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/coverfire.ts:86 no)
pub fn no(g: &mut crate::game::Game, why: &str) { todo!() }

/// (src/game/coverfire.ts:89 bump)
pub fn bump(g: &mut crate::game::Game, k: crate::coverfire::BumpK) { todo!() }

/// (src/game/coverfire.ts:102 chewLive)
pub fn chew_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/coverfire.ts:103 live)
pub fn live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/coverfire.ts:107 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::coverfire::State { todo!() }

/// (src/game/coverfire.ts:113 manOf)
pub fn man_of<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::coverfire::Man { todo!() }

/// (src/game/coverfire.ts:114 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/coverfire.ts:115 charOf)
pub fn char_of(g: &mut crate::game::Game, who: crate::game::Foe) -> CharId { todo!() }

/// (src/game/coverfire.ts:116 upOf)
pub fn up_of(g: &mut crate::game::Game, who: crate::game::Foe) -> bool { todo!() }

/// (src/game/coverfire.ts:117 nameOf)
pub fn name_of(g: &mut crate::game::Game, who: crate::game::Foe) -> String { todo!() }

/// (src/game/coverfire.ts:118 log)
pub fn log(g: &mut crate::game::Game, s: &str) { todo!() }

/// (src/game/coverfire.ts:120 weightOf)
pub fn weight_of(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/coverfire.ts:123 see)
pub fn see(g: &mut crate::game::Game, gd: GuardId, who: crate::game::Foe, low: bool) { todo!() }

/// (src/game/coverfire.ts:131 heard)
pub fn heard(g: &mut crate::game::Game, gd: GuardId, k: crate::coverfire::Know) { todo!() }

/// (src/game/coverfire.ts:133 known)
pub fn known(g: &mut crate::game::Game, gd: GuardId, who: Option<crate::game::Foe>) -> Vec<crate::coverfire::KnownItem> { todo!() }

/// (src/game/coverfire.ts:148 ask)
pub fn ask(g: &mut crate::game::Game, gd: GuardId, to: V3) -> Option<crate::coverfire::Ask2> { todo!() }

/// (src/game/coverfire.ts:185 coverLane)
pub fn cover_lane(g: &mut crate::game::Game, gd: GuardId, from: V3, to: V3) -> bool { todo!() }

/// (src/game/coverfire.ts:200 worth)
pub fn worth(g: &mut crate::game::Game, gd: GuardId, to: V3, rounds: f64) -> bool { todo!() }

/// (src/game/coverfire.ts:206 budgetOf)
pub fn budget_of(g: &mut crate::game::Game, kind: &str, rounds: f64) -> f64 { todo!() }

/// (src/game/coverfire.ts:207 stays)
pub fn stays(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/coverfire.ts:212 choose)
pub fn choose(g: &mut crate::game::Game, gd: GuardId, k: crate::coverfire::Know, call: bool, seen: bool) -> Option<crate::coverfire::Chew> { todo!() }

/// (src/game/coverfire.ts:252 coverSeen)
pub fn cover_seen(g: &mut crate::game::Game, gd: GuardId, who: Option<crate::game::Foe>, to: V3) -> bool { todo!() }

/// (src/game/coverfire.ts:261 coverBlind)
pub fn cover_blind(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> Option<V3> { todo!() }

/// (src/game/coverfire.ts:289 coverOp)
pub fn cover_op(g: &mut crate::game::Game, o: RoomfightOpId, dt: f64, speed: f64, point: &mut dyn FnMut(&mut crate::game::Game, V3) -> f64) -> bool { todo!() }

/// (src/game/coverfire.ts:317 coverfireTick)
pub fn coverfire_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/coverfire.ts:371 coverfireReport)
pub fn coverfire_report(g: &mut crate::game::Game) -> JsMap<String, f64> { todo!() }

/// (src/game/coverfire.ts:379 coverfireSaw)
pub fn coverfire_saw(g: &mut crate::game::Game, gd: GuardId, who: crate::game::Foe, age: Option<f64>) { todo!() }

/// (src/game/coverfire.ts:384 coverfireOn)
pub fn coverfire_on(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::coverfire::CoverfireOn> { todo!() }

/// (src/game/coverfire.ts:388 coverfireReset)
pub fn coverfire_reset(g: &mut crate::game::Game) { todo!() }
