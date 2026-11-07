//! What the parts of `render_particles` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// (src/render/particles.ts:386 Particles.impulse)
pub fn particles_impulse(g: &mut crate::game::Game, pos: V3, radius: f64, strength: f64, o: Option<&crate::render_particles::ImpulseOpts>) -> bool { g.out.push(crate::out_events::OutEvent::ParticlesImpulse(crate::out_events::OutParticlesImpulse { pos: pos, radius: radius, strength: strength, o: o.cloned() })); false }


/// (src/render/particles.ts:417 Particles.setHeat)
pub fn particles_set_heat(g: &mut crate::game::Game, srcs: &[crate::smoke_types::WindGpuSource]) { g.out.push(crate::out_events::OutEvent::ParticlesSetHeat(crate::out_events::OutParticlesSetHeat { srcs: srcs.to_vec() })); }
