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
pub fn particles_set_imp(g: &mut crate::game::Game, r: usize, pos: V3, radius: f64, strength: f64, up: f64, b: V3, e: V3, kinds: f64, flags: f64, seed: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/particles.ts:417 Particles.setHeat)
pub fn particles_set_heat(g: &mut crate::game::Game, srcs: &[crate::smoke_types::WindGpuSource]) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/particles.ts:421 Particles.setWind)
pub fn particles_set_wind(g: &mut crate::game::Game, srcs: &[crate::smoke_types::WindGpuSource], knobs: [f64; 3]) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/particles.ts:423 Particles.clear)
pub fn particles_clear(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/particles.ts:425 Particles.clearRain)
pub fn particles_clear_rain(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (round three; the host's GPU particles: the callers ask `gpu_fx` first, which is false on a host with none): how many it took, none   (render/particles.ts:363 Particles.emit)
pub fn particles_emit(g: &mut crate::game::Game, kind: crate::render_particles::PKind, pos: V3, dir: V3, count: f64, o: crate::render_particles::EmitOpts) -> f64 { 0.0 }

/// BOUNDARY NO-OP (round three; the host's GPU particles)   (render/particles.ts:386 Particles.impulse)
pub fn particles_impulse(g: &mut crate::game::Game, pos: V3, radius: f64, strength: f64, o: Option<&crate::render_particles::ImpulseOpts>) -> bool { false }
