//! THE BRING-UP'S OWN PART of render_char_materials: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/charMaterials.ts:472 writeCharPalette)
pub fn write_char_palette(g: &mut crate::game::Game, out: &mut Vec<f32>, o: f64, side: crate::anim_gltf::CastSide, tint: V3, tint2: V3, id_2: f64, stand_in: Option<bool>, suit: Option<bool>) {}
