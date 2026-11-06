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
pub fn particles_emit(g: &mut crate::game::Game, kind: crate::render_particles::PKind, pos: V3, dir: V3, count: f64, o: crate::render_particles::EmitOpts) -> f64 { g.out.push(crate::out_events::OutEvent::ParticlesEmit(crate::out_events::OutParticlesEmit { kind: kind, pos: pos, dir: dir, count: count, o: o })); 0.0 }

/// (src/render/particles.ts:386 Particles.impulse)
pub fn particles_impulse(g: &mut crate::game::Game, pos: V3, radius: f64, strength: f64, o: Option<&crate::render_particles::ImpulseOpts>) -> bool { g.out.push(crate::out_events::OutEvent::ParticlesImpulse(crate::out_events::OutParticlesImpulse { pos: pos, radius: radius, strength: strength, o: o.cloned() })); false }

/// (src/render/particles.ts:409 Particles.setImp)
pub fn particles_set_imp(g: &mut crate::game::Game, r: usize, pos: V3, radius: f64, strength: f64, up: f64, b: V3, e: V3, kinds: f64, flags: f64, seed: f64) { todo!() }

/// (src/render/particles.ts:417 Particles.setHeat)
pub fn particles_set_heat(g: &mut crate::game::Game, srcs: &[crate::smoke_types::WindGpuSource]) { g.out.push(crate::out_events::OutEvent::ParticlesSetHeat(crate::out_events::OutParticlesSetHeat { srcs: srcs.to_vec() })); }

/// (src/render/particles.ts:421 Particles.setWind)
pub fn particles_set_wind(g: &mut crate::game::Game, srcs: &[crate::smoke_types::WindGpuSource], knobs: [f64; 3]) { todo!() }

/// (src/render/particles.ts:423 Particles.clear)
pub fn particles_clear(g: &mut crate::game::Game) { todo!() }

/// (src/render/particles.ts:425 Particles.clearRain)
pub fn particles_clear_rain(g: &mut crate::game::Game) { todo!() }
