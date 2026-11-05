//! src/scene/round.ts lines 1 to 121
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A box's centre as its level authored it, for the level's fingerprint.   (round.ts authoredC)
pub fn authored_c(g: &mut crate::game::Game, b: BoxId) -> V3 {
    match &g.col.boxes[b].was {
        Some(w) => w.c,
        None => g.col.boxes[b].c,
    }
}

/// A box's half extents as its level authored them, for the level's fingerprint.   (round.ts authoredH)
pub fn authored_h(g: &mut crate::game::Game, b: BoxId) -> V3 {
    match &g.col.boxes[b].was {
        Some(w) => w.h,
        None => g.col.boxes[b].h,
    }
}
