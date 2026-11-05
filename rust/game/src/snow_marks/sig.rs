#![allow(unused_variables)]
//! Every function and method of `snow_marks`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/snowMarks.ts:69 clamp)
pub fn clamp(v: f64, a: f64, b: f64) -> f64 { todo!() }

/// (src/game/snowMarks.ts:70 lies)
pub fn lies(field: &crate::render_snow_field::SnowField, x: f64, z: f64) -> bool { todo!() }

/// (src/game/snowMarks.ts:73 clearSnowMarks)
pub fn clear_snow_marks(g: &mut crate::game::Game) { todo!() }

/// (src/game/snowMarks.ts:76 updateSnowMarks)
pub fn update_snow_marks(g: &mut crate::game::Game, t: &mut crate::snow_tracks::SnowTracks, field: &crate::render_snow_field::SnowField, dt: f64) { todo!() }

/// (src/game/snowMarks.ts:85 propMarks)
pub fn prop_marks(g: &mut crate::game::Game, t: &mut crate::snow_tracks::SnowTracks, field: &crate::render_snow_field::SnowField, dark: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> bool>) { todo!() }

/// (src/game/snowMarks.ts:129 land)
pub fn land(g: &mut crate::game::Game, t: &mut crate::snow_tracks::SnowTracks, k: crate::snow_marks::SNOWMARKS, s: &mut crate::snow_marks::Seen, x: f64, z: f64, r: f64, v: f64, mess: f64) { todo!() }

/// (src/game/snowMarks.ts:142 thrownMarks)
pub fn thrown_marks(g: &mut crate::game::Game, t: &mut crate::snow_tracks::SnowTracks, field: &crate::render_snow_field::SnowField, dark: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> bool>) { todo!() }

/// (src/game/snowMarks.ts:164 chunkMarks)
pub fn chunk_marks(g: &mut crate::game::Game, t: &mut crate::snow_tracks::SnowTracks, field: &crate::render_snow_field::SnowField, dark: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> bool>) { todo!() }
