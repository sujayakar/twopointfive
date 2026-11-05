//! THE BRING-UP'S OWN PART of snow_fx: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowFx.ts:232 updateSnowFx)
pub fn update_snow_fx(g: &mut crate::game::Game, dt: f64, cam: CameraId) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowFx.ts:399 updateWisps)
pub fn update_wisps(g: &mut crate::game::Game, st: &mut crate::snow_fx::SnowState, pp: &mut crate::render_particles::Particles, dt: f64, cx: f64, cz: f64, r: f64, area: f64, room: f64, max: Option<f64>, one_air: Option<bool>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowFx.ts:420 oneAirCfg)
pub fn one_air_cfg(g: &mut crate::game::Game, st: &mut crate::snow_fx::SnowState, pp: &mut crate::render_particles::Particles, on: bool) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowFx.ts:432 grainsOff)
pub fn grains_off(st: &mut crate::snow_fx::SnowState, p: Option<&mut crate::render_particles::Particles>, why: &str) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowFx.ts:442 updateGrains)
pub fn update_grains(g: &mut crate::game::Game, st: &mut crate::snow_fx::SnowState, pp: &mut crate::render_particles::Particles, dt: f64, cx: f64, cz: f64, room: f64, slot_life: f64) {}
