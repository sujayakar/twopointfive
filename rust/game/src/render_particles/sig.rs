#![allow(unused_variables)]
//! Every function and method of `render_particles`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/render/particles.ts:122 kindMask)
pub fn kind_mask(ks: &[crate::render_particles::PKind]) -> f64 { todo!() }

/// (src/render/particles.ts:149 hashU32)
pub fn hash_u32(x: f64) -> f64 { todo!() }

/// (src/render/particles.ts:228 Particles.constructor)
pub fn particles_new(g: &mut crate::game::Game, device: crate::Opaque, scene_layout: crate::Opaque) -> crate::render_particles::Particles { todo!() }

/// (src/render/particles.ts:363 Particles.emit)
pub fn particles_emit(this: &mut crate::render_particles::Particles, kind: crate::render_particles::PKind, pos: V3, dir: V3, count: f64, o: crate::render_particles::EmitOpts) -> f64 { todo!() }

/// (src/render/particles.ts:386 Particles.impulse)
pub fn particles_impulse(this: &mut crate::render_particles::Particles, pos: V3, radius: f64, strength: f64, o: Option<&crate::render_particles::ImpulseOpts>) -> bool { todo!() }

/// (src/render/particles.ts:409 Particles.setImp)
pub fn particles_set_imp(this: &mut crate::render_particles::Particles, r: &mut crate::render_particles::ImpRec, pos: V3, radius: f64, strength: f64, up: f64, b: V3, e: V3, kinds: f64, flags: f64, seed: f64) { todo!() }

/// (src/render/particles.ts:417 Particles.setHeat)
pub fn particles_set_heat(this: &mut crate::render_particles::Particles, srcs: &[crate::smoke_types::WindGpuSource]) { todo!() }

/// (src/render/particles.ts:421 Particles.setWind)
pub fn particles_set_wind(this: &mut crate::render_particles::Particles, srcs: &[crate::smoke_types::WindGpuSource], knobs: [f64; 3]) { todo!() }

/// (src/render/particles.ts:423 Particles.clear)
pub fn particles_clear(this: &mut crate::render_particles::Particles) { todo!() }

/// (src/render/particles.ts:425 Particles.clearRain)
pub fn particles_clear_rain(this: &mut crate::render_particles::Particles) { todo!() }
