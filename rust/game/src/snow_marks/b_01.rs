//! THE BRING-UP'S OWN PART of snow_marks: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};


/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowMarks.ts:76 updateSnowMarks)
pub fn update_snow_marks(g: &mut crate::game::Game, t: &mut crate::snow_tracks::SnowTracks, field: &crate::render_snow_field::SnowField, dt: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowMarks.ts:129 land)
pub fn land(g: &mut crate::game::Game, t: &mut crate::snow_tracks::SnowTracks, k: crate::snow_marks::SNOWMARKS, s: &mut crate::snow_marks::Seen, x: f64, z: f64, r: f64, v: f64, mess: f64) {}

