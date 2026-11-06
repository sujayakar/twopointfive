#![allow(unused_variables)]
//! Every function and method of `rescuecheck`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/rescuecheck.ts:83 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/rescuecheck.ts:84 frac)
pub fn frac(x: f64) -> f64 { todo!() }

/// (src/game/rescuecheck.ts:85 alive)
pub fn alive(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/rescuecheck.ts:86 fit)
pub fn fit(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/rescuecheck.ts:87 roused)
pub fn roused(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/rescuecheck.ts:88 loudHeard)
pub fn loud_heard(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/rescuecheck.ts:89 defOf)
pub fn def_of(g: &mut crate::game::Game, gd: GuardId) -> Option<RoomfightDefId> { todo!() }

/// (src/game/rescuecheck.ts:91 net)
pub fn net(g: &mut crate::game::Game, gd: GuardId, text: &str, cls: Option<f64>, chron: Option<bool>) { todo!() }

/// (src/game/rescuecheck.ts:95 band)
pub fn band(b: [f64; 2], k: f64) -> f64 { todo!() }

/// (src/game/rescuecheck.ts:96 event)
pub fn event(g: &mut crate::game::Game, what: crate::rescuecheck::RescueCheckEventWhat, gd: Option<GuardId>, why: &str) { todo!() }

/// (src/game/rescuecheck.ts:101 ruled)
pub fn ruled(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rescuecheck.ts:108 walking)
pub fn walking(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/rescuecheck.ts:110 rescueCheckSay)
pub fn rescue_check_say(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescuecheck.ts:112 rescueCheckTimeline)
pub fn rescue_check_timeline(g: &mut crate::game::Game) -> Vec<crate::rescuecheck::RescueCheckEvent> { todo!() }

/// (src/game/rescuecheck.ts:114 rescueCheckReset)
pub fn rescue_check_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescuecheck.ts:118 goer)
pub fn goer(g: &mut crate::game::Game, cast: &[crate::rescuecheck::CastItem], about: GuardId, post: V3) -> Option<crate::rescuecheck::Goer> { todo!() }

/// (src/game/rescuecheck.ts:125 postOf)
pub fn post_of(g: &mut crate::game::Game, cast: &[crate::rescuecheck::CastItem], gd: GuardId) -> V3 { todo!() }

/// (src/game/rescuecheck.ts:127 unseen)
pub fn unseen(g: &mut crate::game::Game, c_2: crate::rescuecheck::CastItem) -> bool { todo!() }

/// (src/game/rescuecheck.ts:128 walk)
pub fn walk(g: &mut crate::game::Game, d: RoomfightDefId, to: V3) { todo!() }

/// (src/game/rescuecheck.ts:131 rescueCheckTick)
pub fn rescue_check_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/rescuecheck.ts:132 stepCheck)
pub fn step_check(g: &mut crate::game::Game, _dt: f64) { todo!() }

/// (src/game/rescuecheck.ts:190 tickOut)
pub fn tick_out(g: &mut crate::game::Game, cast: &[crate::rescuecheck::CastItem], fight: bool) { todo!() }

/// (src/game/rescuecheck.ts:229 letGo)
pub fn let_go(g: &mut crate::game::Game) { todo!() }
