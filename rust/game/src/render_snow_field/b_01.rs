//! THE BRING-UP'S OWN PART of render_snow_field: bodies for skeleton stubs a tick reached (bringup2-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// (depot-four tick 2712: a frag bursts, furniture_blast < break_static_in < snow_paint). render/snowField.ts is the renderer's file, but these three
// "ask what a box DOES … ONE MODEL THE PICTURE AND THE GAME BOTH READ" (snowField.ts:75): furnblast and snowBog call them. Pure readings of a box; translated.
/// which lying snow a box is, by its word, or none   (snowField.ts:79 snowPaintOf)
pub fn snow_paint_of(g: &crate::game::Game, b: &crate::render_snow_field::SnowPaintOfB) -> Option<crate::scene_materials::SnowKind> {
    crate::scene_materials::does_of_word(g, b.does.as_deref()).snow
}
/// is this box a thin lay on the ground: at the ground, a finger thick at most, not glass and not the deep snow's own paint   (snowField.ts:73 snowThinLay)
pub fn snow_thin_lay(g: &crate::game::Game, b: BoxId) -> bool {
    let (c, h, flags, does) = { let x = &g.tab.boxes[b]; (x.c, x.h, x.flags, x.does) };
    let top = c[1] + h[1];
    let glass = js::to_i32(flags) & js::to_i32(crate::scene_boxes::box_flag().glass) != 0;
    !glass && top <= 0.03 && top > 0.0 && c[1] - h[1] >= -0.02 && crate::scene_materials::does_of(g, does).snow != Some(crate::scene_materials::SnowKind::Snow)
}
/// ground somebody cleared: any paint that is not snow's, and not a lit thing lying on the snow   (snowField.ts:74 snowLayBare)
pub fn snow_lay_bare(g: &crate::game::Game, b: BoxId) -> bool {
    let (e, does) = { let x = &g.tab.boxes[b]; (x.emissive, x.does) };
    crate::scene_materials::does_of(g, does).snow.is_none() && e[0] + e[1] + e[2] <= 0.0
}
