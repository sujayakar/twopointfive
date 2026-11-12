#![allow(unused_imports, unused_variables)]
pub use ::game_types::render_snow_field::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};

// These three ask what a box DOES, one model the picture and the game both read: furnblast and snow_bog call them. Pure readings of a box.
/// which lying snow a box is, by its word, or none
pub fn snow_paint_of(g: &crate::game::Game, b: &crate::render_snow_field::SnowPaintOfB) -> Option<crate::scene_materials::SnowKind> {
    crate::scene_materials::does_of_word(g, b.does.as_deref()).snow
}
/// is this box a thin lay on the ground: at the ground, a finger thick at most, not glass and not the deep snow's own paint
pub(super) fn snow_thin_lay(g: &crate::game::Game, b: BoxId) -> bool {
    let (c, h, flags, does) = { let x = &g.tab.boxes[b]; (x.c, x.h, x.flags, x.does) };
    let top = c[1] + h[1];
    let glass = js::to_i32(flags) & crate::scene_boxes::box_flag::GLASS as i32 != 0;
    !glass && top <= 0.03 && top > 0.0 && c[1] - h[1] >= -0.02 && crate::scene_materials::does_of(g, does).snow != Some(crate::scene_materials::SnowKind::Snow)
}
/// ground somebody cleared: any paint that is not snow's, and not a lit thing lying on the snow
pub(super) fn snow_lay_bare(g: &crate::game::Game, b: BoxId) -> bool {
    let (e, does) = { let x = &g.tab.boxes[b]; (x.emissive, x.does) };
    crate::scene_materials::does_of(g, does).snow.is_none() && e[0] + e[1] + e[2] <= 0.0
}
