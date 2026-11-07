#![allow(unused_variables)]
//! Every function and method of `sparks`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/sparks.ts:28 Sparks.constructor)
pub fn sparks_new(g: &mut crate::game::Game, params: crate::sparks::SparkParams) -> crate::sparks::Sparks { todo!() }

/// (src/game/sparks.ts:33 Sparks.slot)
pub fn sparks_slot(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/sparks.ts:39 Sparks.spawn)
pub fn sparks_spawn(g: &mut crate::game::Game, at: V3, vel: V3, life: f64) -> f64 { todo!() }

/// (src/game/sparks.ts:45 Sparks.h)
pub fn sparks_h(i: f64, k: f64, seed: f64) -> f64 { todo!() }

/// (src/game/sparks.ts:48 Sparks.emitCone)
pub fn sparks_emit_cone(g: &mut crate::game::Game, at: V3, dir: V3, o: crate::sparks::SparksEmitConeO, seed: f64) -> Vec<f64> { todo!() }

/// (src/game/sparks.ts:61 Sparks.emitBurst)
pub fn sparks_emit_burst(g: &mut crate::game::Game, at: V3, o: crate::sparks::SparksEmitBurstO, seed: f64) -> Vec<f64> { todo!() }

/// (src/game/sparks.ts:73 Sparks.tracker)
pub fn sparks_tracker(g: &mut crate::game::Game, i: f64) -> crate::FnField { todo!() }

/// (src/game/sparks.ts:77 Sparks.clear)
pub fn sparks_clear(g: &mut crate::game::Game) { todo!() }

/// (src/game/sparks.ts:79 Sparks.update)
pub fn sparks_update(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/sparks.ts:101 Sparks.boxes)
pub fn sparks_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }
