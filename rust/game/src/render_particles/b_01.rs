//! THE BRING-UP'S OWN PART of render_particles: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/particles.ts:409 Particles.setImp)
pub fn particles_set_imp(this: &mut crate::render_particles::Particles, r: &mut crate::render_particles::ImpRec, pos: V3, radius: f64, strength: f64, up: f64, b: V3, e: V3, kinds: f64, flags: f64, seed: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/particles.ts:417 Particles.setHeat)
pub fn particles_set_heat(this: &mut crate::render_particles::Particles, srcs: &[crate::smoke_types::WindGpuSource]) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/particles.ts:421 Particles.setWind)
pub fn particles_set_wind(this: &mut crate::render_particles::Particles, srcs: &[crate::smoke_types::WindGpuSource], knobs: [f64; 3]) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/particles.ts:423 Particles.clear)
pub fn particles_clear(this: &mut crate::render_particles::Particles) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/particles.ts:425 Particles.clearRain)
pub fn particles_clear_rain(this: &mut crate::render_particles::Particles) {}
