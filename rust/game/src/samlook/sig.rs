#![allow(unused_variables)]
//! Every function and method of `samlook`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/samlook.ts:34 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::samlook::St { todo!() }

/// (src/game/samlook.ts:35 smooth)
pub fn smooth(a: f64, b: f64, x: f64) -> f64 { todo!() }

/// (src/game/samlook.ts:36 wrap)
pub fn wrap(a: f64) -> f64 { todo!() }

/// (src/game/samlook.ts:39 free)
pub fn free(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/samlook.ts:44 samLookNoticed)
pub fn sam_look_noticed(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/samlook.ts:46 reach)
pub fn reach(g: &mut crate::game::Game, gd: GuardId, max_m: f64, deg: f64) -> f64 { todo!() }

/// (src/game/samlook.ts:54 pick)
pub fn pick(g: &mut crate::game::Game) -> Option<GuardId> { todo!() }

/// (src/game/samlook.ts:67 samLookMan)
pub fn sam_look_man(g: &mut crate::game::Game) -> Option<GuardId> { todo!() }

/// (src/game/samlook.ts:69 slipOf)
pub fn slip_of(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/samlook.ts:75 slips)
pub fn slips(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/samlook.ts:77 samSlipMan)
pub fn sam_slip_man(g: &mut crate::game::Game) -> Option<crate::samlook::SamSlipMan> { todo!() }

/// (src/game/samlook.ts:85 slipNow)
pub fn slip_now(g: &mut crate::game::Game) -> Option<crate::samlook::SlipNow> { todo!() }

/// (src/game/samlook.ts:93 headOf)
pub fn head_of(g: &mut crate::game::Game, gd: GuardId, up: f64) -> V3 { todo!() }

/// (src/game/samlook.ts:95 samLookStep)
pub fn sam_look_step(g: &mut crate::game::Game) { todo!() }
