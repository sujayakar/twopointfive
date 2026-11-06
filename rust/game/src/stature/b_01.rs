//! THE BRING-UP'S OWN PART of stature: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/stature.ts:94 drawnBoxes)
pub fn drawn_boxes(g: &mut crate::game::Game, men: &[CharId], out: &mut Vec<BoxId>, from: f64) {}

/// BOUNDARY: the host draws a man's weapon laser from his DRAWN muzzle at the swap (his scale off `shown`, the far end kept on the simulation's point). Here: the simulation's own ray, k 1, as for a man at stature 1. (src/game/stature.ts:105 drawnBeam)
pub fn drawn_beam(g: &mut crate::game::Game, c_2: CharId, from: V3, dir: V3, l: f64) -> crate::stature::DrawnBeam {
    crate::stature::DrawnBeam { from, dir, k: 1.0 }
}
