//! THE BRING-UP'S OWN PART of render_lights: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/lights.ts:113 packLight)
pub fn pack_light(g: &mut crate::game::Game, f: &mut Vec<f32>, u: &mut Vec<u32>, o: f64, l: LightId, inten: f64, kind_word: f64, volumetric: f64, owner_word: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/lights.ts:172 LightSet.remove)
pub fn light_set_remove(g: &mut crate::game::Game, this: &mut crate::render_lights::LightSet, l: LightId) {}
