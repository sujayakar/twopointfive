//! THE BRING-UP'S OWN PART of snow_tracks: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};


/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowTracks.ts:237 press)
pub fn press(t: &crate::snow_tracks::Stamp, ax: f64, az: f64, bx: f64, bz: f64, r: f64, r0: Option<f64>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowTracks.ts:502 mark)
pub fn mark(g: &mut crate::game::Game, t: &mut crate::snow_tracks::SnowTracks, k: f64, a: &[f64]) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowTracks.ts:511 lay)
pub fn lay(g: &crate::game::Game, t: &crate::snow_tracks::SnowTracks, k: f64, a: &[f64], live: bool) {}

