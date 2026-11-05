//! src/scene/round.ts lines 1 to 121
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A box's row. Once the table of Box rows exists, the body is that table indexed by `b`.
fn seam_box<'g>(g: &'g mut Game, b: BoxId) -> &'g mut crate::scene_boxes::Box {
    todo!("needs: the table of Box rows: skeleton 5 has g.col.boxes as Vec<BoxId> and no Tab of Box, while index/properties.txt says g.col.boxes[id]")
}

/// A box's centre as its level authored it, for the level's fingerprint.   (round.ts authoredC)
pub fn authored_c(g: &mut crate::game::Game, b: BoxId) -> V3 {
    let x = seam_box(g, b);
    match &x.was {
        Some(w) => w.c,
        None => x.c,
    }
}

/// A box's half extents as its level authored them, for the level's fingerprint.   (round.ts authoredH)
pub fn authored_h(g: &mut crate::game::Game, b: BoxId) -> V3 {
    let x = seam_box(g, b);
    match &x.was {
        Some(w) => w.h,
        None => x.h,
    }
}
