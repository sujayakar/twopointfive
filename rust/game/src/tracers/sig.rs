#![allow(unused_variables)]
//! Every function and method of `tracers`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/tracers.ts:32 isTracerRound)
pub fn is_tracer_round(g: &mut crate::game::Game, w: FirearmId) -> bool { todo!() }

/// (src/game/tracers.ts:69 Tracers.constructor)
pub fn tracers_new(g: &mut crate::game::Game, params: Option<crate::tracers::TracersParams>) -> crate::tracers::Tracers { todo!() }

/// (src/game/tracers.ts:78 Tracers.each)
pub fn tracers_each(g: &mut crate::game::Game) -> Vec<crate::tracers::Tracer> { todo!() }

/// (src/game/tracers.ts:83 Tracers.round)
pub fn tracers_round(g: &mut crate::game::Game, w: Option<FirearmId>, def: &crate::weapons::WeaponDef, muzzle: V3, dir: V3, t_hit: f64, impact: crate::tracers::TracerImpact, owner: f64, force: Option<bool>) -> bool { todo!() }

/// (src/game/tracers.ts:92 Tracers.slot)
pub fn tracers_slot<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::tracers::Tracer { todo!() }

/// (src/game/tracers.ts:98 Tracers.spawn)
pub fn tracers_spawn(g: &mut crate::game::Game, from: V3, dir: V3, l: f64, impact: crate::tracers::TracerImpact, color: V3, owner: f64) { todo!() }

/// (src/game/tracers.ts:118 Tracers.release)
pub fn tracers_release(g: &mut crate::game::Game, t: &mut crate::tracers::Tracer) { todo!() }

/// (src/game/tracers.ts:127 Tracers.update)
pub fn tracers_update(g: &mut crate::game::Game, dt: f64, out: &mut Vec<BoxId>, m_per_px: Option<f64>) { todo!() }

/// (src/game/tracers.ts:152 Tracers.ember)
pub fn tracers_ember(g: &mut crate::game::Game, t: crate::tracers::Tracer) { todo!() }

/// (src/game/tracers.ts:162 Tracers.clear)
pub fn tracers_clear(g: &mut crate::game::Game) { todo!() }
