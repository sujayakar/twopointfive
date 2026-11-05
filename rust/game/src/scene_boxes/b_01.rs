//! THE BRING-UP'S OWN PART of scene_boxes: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// THE BOX TABLE'S MAKERS (skeleton 5.1: every box is a row of `g.tab.boxes`). Invented by the skeleton, so they have no TypeScript line: written here.

/// a box made to stay: a row of g.tab.boxes for good
pub fn box_new(g: &mut Game, b: crate::scene_boxes::Box) -> BoxId { g.tab.boxes.push(b) }
/// a box made for this frame's list: a row freed by boxes_frame_reset is used again, else a new row
pub fn box_frame(g: &mut Game, b: crate::scene_boxes::Box) -> BoxId {
    let id = match g.box_frame_free.pop() { Some(id) => { g.tab.boxes[id] = b; id } None => g.tab.boxes.push(b) };
    g.box_frame_rows.push(id);
    id
}
/// the frame's boxes may be reused: called as the dyn boxes are rebuilt
pub fn boxes_frame_reset(g: &mut Game) { let mut rows = core::mem::take(&mut g.box_frame_rows); g.box_frame_free.append(&mut rows); }
