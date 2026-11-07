#![allow(unused_variables)]
//! Every function and method of `tapstep`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/tapstep.ts:52 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::tapstep::St { todo!() }

/// (src/game/tapstep.ts:55 tapHeld)
pub fn tap_held(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/tapstep.ts:57 tapHears)
pub fn tap_hears(g: &mut crate::game::Game, pos: &[f64]) { todo!() }

/// (src/game/tapstep.ts:58 tapReset)
pub fn tap_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/tapstep.ts:61 backOf)
pub fn back_of(g: &mut crate::game::Game, dx: f64, dz: f64, far: f64) -> Option<crate::tapstep::BackOf> { todo!() }

/// (src/game/tapstep.ts:69 spotOf)
pub fn spot_of(g: &mut crate::game::Game, gd: GuardId) -> crate::tapstep::SpotOf { todo!() }

/// (src/game/tapstep.ts:71 stepSecs)
pub fn step_secs(g: &mut crate::game::Game, len: f64) -> f64 { todo!() }

/// (src/game/tapstep.ts:72 lift)
pub fn lift(g: &mut crate::game::Game, pl: PlayerId, mx: f64, mz: f64, rush: f64, least: f64) { todo!() }

/// (src/game/tapstep.ts:78 plant)
pub fn plant(g: &mut crate::game::Game, pl: PlayerId) { todo!() }

/// (src/game/tapstep.ts:87 tapTick)
pub fn tap_tick(g: &mut crate::game::Game, mx: f64, mz: f64, ml: f64, dt: f64) -> crate::tapstep::TapMove { todo!() }

/// (src/game/tapstep.ts:120 tapOff)
pub fn tap_off(g: &mut crate::game::Game, ml: f64) { todo!() }
