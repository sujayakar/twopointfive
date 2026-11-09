#![allow(unused_imports, unused_variables)]
//! Bodies for skeleton stubs of scene_marking_defaults that translated code calls and no part had (the stub round).

pub use ::game_types::scene_marking_defaults::types;
pub use types::*;
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

/// markFromRect with the painter `c => c.fill(1)`, through marking: a rect in metres snapped inward onto the lattice, every cell palette entry 1; None if the
/// snapped rect is empty. A private fn because the skeleton's mark_from_rect hands its painter the canvas by value, so a fill could not reach the rows.   (markings.ts markFromRect)
fn filled_rect(face: crate::scene_markings::MarkFace, res: crate::scene_markings::MarkRes, rect: [f64; 4], pal: crate::scene_markings::MarkPal, base: f64) -> Option<crate::scene_markings::MarkingDef> {
    let t = base * *crate::scene_markings::res_k().get(String::from(res.as_str())).unwrap();
    let eps = 1e-7;
    let (u0, v0, u1, v1) = (m::ceil(rect[0] / t - eps), m::ceil(rect[1] / t - eps), m::floor(rect[2] / t + eps), m::floor(rect[3] / t + eps));
    let (w, h) = (m::min(u1 - u0, crate::scene_markings::MAX_CELLS_W), m::min(v1 - v0, crate::scene_markings::MAX_CELLS_H));
    if w < 1.0 || h < 1.0 { return None; }
    let row = crate::scene_markings::CELL_CHARS[1..2].repeat(w as usize);
    Some(crate::scene_markings::MarkingDef {
        face,
        res: if res != crate::scene_markings::MarkRes::Base { Some(res) } else { None },
        at: [u0, v0],
        size: [w, h],
        pal: vec![pal],   // (palEntry of P.steel is P.steel: an albedo, a roughness, no emission)
        cells: vec![row; h as usize],
    })
}

/// DOOR LEAF: kick plates along the bottom of both big faces, a push plate by the latch edge on +z.   (markingDefaults.ts doorLeafMarks)
pub fn door_leaf_marks(h: V3, t: Option<f64>) -> Vec<crate::scene_markings::MarkingDef> {
    use crate::scene_markings::{MarkFace, MarkRes};
    let t = t.unwrap_or(T);
    let yb = -(h[1] - 0.02);
    let steel = p().steel;
    [
        filled_rect(MarkFace::Z, MarkRes::Coarse, [-(h[0] - 0.04), yb, h[0] - 0.04, yb + 0.2], steel, t),
        filled_rect(MarkFace::Z2, MarkRes::Coarse, [-(h[0] - 0.04), yb, h[0] - 0.04, yb + 0.2], steel, t),
        filled_rect(MarkFace::Z, MarkRes::Base, [h[0] - 0.22, -0.075, h[0] - 0.1, 0.225], steel, t),
    ].into_iter().flatten().collect()
}
