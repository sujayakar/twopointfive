//! THE BRING-UP'S OWN PART of wet: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wet.ts:96 updateWetness)
pub fn update_wetness(g: &mut crate::game::Game, dt: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wet.ts:119 samAftermath)
pub fn sam_aftermath(g: &mut crate::game::Game, dt: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wet.ts:133 drip)
pub fn drip(g: &mut crate::game::Game, c_2: CharId) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wet.ts:143 wetFootfall)
pub fn wet_footfall(g: &mut crate::game::Game, left_hint: bool) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wet.ts:162 agePrints)
pub fn age_prints(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wet.ts:179 clearWetPrints)
pub fn clear_wet_prints(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wet.ts:183 wetInstance)
pub fn wet_instance(g: &mut crate::game::Game, c_2: CharId, on: bool, out: &mut Vec<f32>, o: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wet.ts:190 wetLineRow)
pub fn wet_line_row(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<f32>, o: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/wet.ts:195 wetKnobs)
pub fn wet_knobs(g: &mut crate::game::Game, out: &mut Vec<f32>, o: f64) {}
