//! THE BRING-UP'S OWN PART of air_sim: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/airSim.ts:77 airResend)
pub fn air_resend(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/airSim.ts:78 airWant)
pub fn air_want(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/airSim.ts:82 airPuff)
pub fn air_puff(g: &mut crate::game::Game, x: f64, z: f64, out: f64, sigma: f64, secs: f64, rate: Option<f64>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/airSim.ts:101 stamp)
pub fn stamp(m: &mut Vec<u8>, w: f64, h: f64, cx: f64, cz: f64, hx: f64, hz: f64, yaw: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/airSim.ts:185 attachAir)
pub fn attach_air(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/airSim.ts:201 updateAirSim)
pub fn update_air_sim(g: &mut crate::game::Game, dt: f64) {}
