#![allow(unused_variables)]
//! Every function and method of `scene_markings`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/scene/markings.ts:58 faceHalf)
pub fn face_half(h: V3, f: f64) -> crate::scene_markings::FaceHalf { todo!() }

/// (src/scene/markings.ts:62 faceIndex)
pub fn face_index(f: crate::scene_markings::MarkFace) -> f64 { todo!() }

/// (src/scene/markings.ts:63 texelOf)
pub fn texel_of(m: crate::scene_markings::TexelOfM, base: Option<f64>) -> f64 { todo!() }

/// (src/scene/markings.ts:67 marking)
pub fn marking(m: &crate::scene_markings::MarkingDef) -> crate::scene_markings::MarkingDef { todo!() }

/// (src/scene/markings.ts:71 palEntry)
pub fn pal_entry(p: crate::scene_markings::MarkPal) -> crate::scene_markings::MarkPal { todo!() }

/// (src/scene/markings.ts:121 CellCanvas.constructor)
pub fn cell_canvas_new(w: f64, h: f64) -> crate::scene_markings::CellCanvas { todo!() }

/// (src/scene/markings.ts:125 CellCanvas.fill)
pub fn cell_canvas_fill(this: &mut crate::scene_markings::CellCanvas, e: f64, i0: Option<f64>, j0: Option<f64>, i1: Option<f64>, j1: Option<f64>) -> crate::scene_markings::CellCanvas { todo!() }

/// (src/scene/markings.ts:126 CellCanvas.toRows)
pub fn cell_canvas_to_rows(this: &mut crate::scene_markings::CellCanvas) -> Vec<String> { todo!() }

/// (src/scene/markings.ts:302 markFromRect)
pub fn mark_from_rect(face: crate::scene_markings::MarkFace, res: crate::scene_markings::MarkRes, rect: [f64; 4], pal: &[crate::scene_markings::MarkPal], paint: &mut dyn FnMut(crate::scene_markings::CellCanvas), base: Option<f64>) -> Option<crate::scene_markings::MarkingDef> { todo!() }

/// (src/scene/markings.ts:329 resolveAlbedo)
pub fn resolve_albedo(p: Option<crate::scene_markings::MarkPal>, base: V3) -> V3 { todo!() }

/// (src/scene/markings.ts:334 markStats)
pub fn mark_stats(m: &crate::scene_markings::MarkingDef, base: V3, texel_base: Option<f64>) -> crate::scene_markings::MarkStats { todo!() }

/// (src/scene/markings.ts:349 markRecordRows)
pub fn mark_record_rows(m: &crate::scene_markings::MarkingDef) -> f64 { todo!() }

/// (src/scene/markings.ts:354 packMarking)
pub fn pack_marking(out: &mut Vec<f32>, row: f64, m: &crate::scene_markings::MarkingDef, st: crate::scene_markings::MarkStats, last: bool, any_emit: bool, area_lit: bool) { todo!() }

/// (src/scene/markings.ts:370 faceAreas)
pub fn face_areas(h: V3) -> crate::scene_markings::FaceAreas { todo!() }

/// (src/scene/markings.ts:376 secondaryAlbedo)
pub fn secondary_albedo(base: V3, h: V3, marks: &[crate::scene_markings::MarkingDef], texel_base: Option<f64>) -> V3 { todo!() }

/// (src/scene/markings.ts:383 markWorldRect)
pub fn mark_world_rect(m: &crate::scene_markings::MarkingDef, c_2: V3, h: V3, yaw: f64, texel_base: Option<f64>) -> crate::scene_markings::MarkWorldRect { todo!() }
