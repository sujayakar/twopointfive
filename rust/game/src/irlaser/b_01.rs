//! THE BRING-UP'S OWN PART of irlaser: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/irlaser.ts:92 samLaserBoxes)
pub fn sam_laser_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {}

/// BOUNDARY NO-OP (irlaser.ts:78): the mark "this box is drawn in IR" is the picture's set (read by irMade as a view is drawn); the box is handed back
pub fn ir_note(g: &mut crate::game::Game, b: BoxId) -> BoxId { b }
