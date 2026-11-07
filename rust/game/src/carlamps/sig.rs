#![allow(unused_variables)]
//! Every function and method of `carlamps`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/carlamps.ts:42 sigs)
pub fn sigs() -> Vec<crate::carlamps::LampSig> { todo!() }

/// (src/game/carlamps.ts:53 near3)
pub fn near3(a: &[f64], b: &[f64], eps: f64) -> bool { todo!() }

/// (src/game/carlamps.ts:54 glows)
pub fn glows(g: &mut crate::game::Game, b: BoxId) -> bool { todo!() }

/// (src/game/carlamps.ts:55 fits)
pub fn fits(g: &mut crate::game::Game, b: BoxId, s: crate::carlamps::LampSig) -> bool { todo!() }

/// (src/game/carlamps.ts:63 findCarLamps)
pub fn find_car_lamps(g: &mut crate::game::Game, boxes: &[BoxId]) -> Vec<crate::carlamps::FoundLamp> { todo!() }

/// (src/game/carlamps.ts:77 carLampTargets)
pub fn car_lamp_targets(g: &mut crate::game::Game, boxes: &[BoxId]) -> Vec<crate::sim_lightshome::FixtureDef> { todo!() }

/// (src/game/carlamps.ts:90 carLampsTape)
pub fn car_lamps_tape(g: &mut crate::game::Game, tables: &JsMap<String, crate::Opaque>) { todo!() }

/// (src/game/carlamps.ts:93 lampGlass)
pub fn lamp_glass(g: &mut crate::game::Game, t: FixtureId) -> bool { todo!() }

/// (src/game/carlamps.ts:95 deadLamp)
pub fn dead_lamp(g: &mut crate::game::Game, t: FixtureId) -> bool { todo!() }

/// (src/game/carlamps.ts:100 carLampPaint)
pub fn car_lamp_paint(g: &mut crate::game::Game, t: FixtureId) { todo!() }
