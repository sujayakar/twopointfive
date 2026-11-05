#![allow(unused_variables)]
//! Every function and method of `stalk`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/stalk.ts:144 fresh)
pub fn fresh() -> crate::stalk::St { todo!() }

/// (src/game/stalk.ts:145 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::stalk::St { todo!() }

/// (src/game/stalk.ts:146 end)
pub fn end(g: &mut crate::game::Game, pl: PlayerId, why: crate::stalk::Why) { todo!() }

/// (src/game/stalk.ts:149 bearingOf)
pub fn bearing_of(g: &mut crate::game::Game, gd: GuardId, p: &[f64]) -> f64 { todo!() }

/// (src/game/stalk.ts:151 hands)
pub fn hands(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/stalk.ts:156 free)
pub fn free(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/stalk.ts:161 shut)
pub fn shut(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/stalk.ts:163 able)
pub fn able(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/stalk.ts:166 beginM)
pub fn begin_m(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/stalk.ts:167 outM)
pub fn out_m(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/stalk.ts:169 stalkBeginM)
pub fn stalk_begin_m(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/stalk.ts:170 stalkOutM)
pub fn stalk_out_m(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/stalk.ts:173 stalkClickOnly)
pub fn stalk_click_only(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/stalk.ts:174 outOf)
pub fn out_of(g: &mut crate::game::Game, gd: GuardId, p: &[f64], begin: bool) -> Option<crate::stalk::Why> { todo!() }

/// (src/game/stalk.ts:183 stalkCould)
pub fn stalk_could(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/stalk.ts:185 pressed)
pub fn pressed(g: &mut crate::game::Game) -> Option<GuardId> { todo!() }

/// (src/game/stalk.ts:194 cornerGate)
pub fn corner_gate(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/stalk.ts:198 cornerMan)
pub fn corner_man(g: &mut crate::game::Game) -> Option<GuardId> { todo!() }

/// (src/game/stalk.ts:210 duck)
pub fn duck(g: &mut crate::game::Game, gd: GuardId, k: f64) { todo!() }

/// (src/game/stalk.ts:215 brokeDuck)
pub fn broke_duck(g: &mut crate::game::Game, pl: PlayerId) { todo!() }

/// (src/game/stalk.ts:227 stalkPolish)
pub fn stalk_polish(g: &mut crate::game::Game, up: bool) { todo!() }

/// (src/game/stalk.ts:238 stalkStep)
pub fn stalk_step(g: &mut crate::game::Game, lmb: bool, lmb_hit: bool, own_keys: bool, stick: Option<crate::stalk::StalkStick>, keys: Option<crate::stalk::StalkKeys>) -> Option<crate::stalk::StalkMove> { todo!() }

/// (src/game/stalk.ts:272 dragged)
pub fn dragged(g: &mut crate::game::Game, pl: PlayerId, gd: GuardId, p: &[f64], at: V3, dist: f64, st: crate::stalk::StalkStick, mid: [f64; 2], dt: f64) -> crate::stalk::StalkMove { todo!() }

/// (src/game/stalk.ts:283 intoKeep)
pub fn into_keep(g: &mut crate::game::Game, dist: f64) -> f64 { todo!() }

/// (src/game/stalk.ts:288 orbited)
pub fn orbited(g: &mut crate::game::Game, pl: PlayerId, gd: GuardId, p: &[f64], at: V3, dist: f64, dx: f64, dz: f64, pace: f64, dt: f64) -> crate::stalk::StalkMove { todo!() }

/// (src/game/stalk.ts:309 stalkTell)
pub fn stalk_tell(g: &mut crate::game::Game) -> Option<crate::stalk::StalkTell> { todo!() }

/// (src/game/stalk.ts:322 stalkWait)
pub fn stalk_wait(g: &mut crate::game::Game) -> Option<crate::stalk::StalkWait> { todo!() }

/// (src/game/stalk.ts:332 stalkHas)
pub fn stalk_has(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/stalk.ts:334 stalkGrab)
pub fn stalk_grab(g: &mut crate::game::Game) -> Option<GuardId> { todo!() }

/// (src/game/stalk.ts:337 stalkKeyCan)
pub fn stalk_key_can(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/stalk.ts:342 stalkKeyTook)
pub fn stalk_key_took(g: &mut crate::game::Game) { todo!() }

/// (src/game/stalk.ts:344 stalkOn)
pub fn stalk_on(g: &mut crate::game::Game) -> Option<GuardId> { todo!() }

/// (src/game/stalk.ts:346 stalkReset)
pub fn stalk_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/stalk.ts:350 stepEase)
pub fn step_ease(u: f64) -> f64 { todo!() }
