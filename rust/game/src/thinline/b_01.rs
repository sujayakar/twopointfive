//! THE BRING-UP'S OWN PART of thinline: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's: a thin line's drawn width by distance, thinline.ts:17): the note is the renderer's side table; the box is handed back as the TypeScript does
pub fn thin_note(g: &mut crate::game::Game, b: BoxId, hw0: f64, k: f64, r: f64, g_2: f64, bl: f64) -> BoxId { b }
/// (thinline.ts:23 thinHalf) pure
pub fn thin_half(hw0: f64, k: f64, px: f64) -> f64 { m::max(hw0, k * px) }
/// BOUNDARY NO-OP (thinline.ts:26): no line was noted, so none is refitted: the list stands
pub fn thin_fit(g: &mut crate::game::Game, list: &[BoxId], px: f64) -> Option<Vec<BoxId>> { None }
/// BOUNDARY NO-OP (thinline.ts:44): the line is handed back
pub fn thin_air(g: &mut crate::game::Game, line: BoxId, bead: Option<BoxId>) -> BoxId { line }
