//! THE BRING-UP'S OWN PART of scene_boxes: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// TEMP-BRINGUP: THE TABLE A MADE BOX LIVES IN. SKELETON-GAP: Box has an ID and no table (ids.txt `"table": "none"`): `g.col.boxes` is the statics' own
// list, and a box made as the game runs (makeBox / boxOf: a device's body, a prop's boxes, a door's leaf, debris, a hand's prop) has nowhere to be.
// Until the skeleton gives one (and says when a frame's boxes are dropped), a made box goes in this arena of the module, known by an ID past any
// static's, and is NEVER dropped (one tick's worth in the bring-up). Only scene_boxes' own readers (p_01 `bx`) know the arena: another part that indexes
// `g.col.boxes` with such an ID panics on the index, which is the next stop to see.
struct Arena(core::cell::UnsafeCell<Vec<Box<crate::scene_boxes::Box>>>);
unsafe impl Sync for Arena {}
static MADE: Arena = Arena(core::cell::UnsafeCell::new(Vec::new()));
/// the first ID of a made box
pub const MADE_BASE: usize = 1 << 30;

/// a made box's row and its ID
pub fn made_push(b: crate::scene_boxes::Box) -> BoxId {
    let v = unsafe { &mut *MADE.0.get() };
    v.push(Box::new(b));
    BoxId::from_ix(MADE_BASE + v.len() - 1)
}
/// the made box an ID names (None: a static's ID)
pub fn made_get(id: BoxId) -> Option<&'static crate::scene_boxes::Box> {
    let i = id.ix();
    if i < MADE_BASE { return None; }
    let v = unsafe { &*MADE.0.get() };
    Some(&*v[i - MADE_BASE])
}
/// how many boxes have been made
pub fn made_count() -> usize { unsafe { (*MADE.0.get()).len() } }

/// the box an ID names, static or made (TEMP-BRINGUP: what `g.tab.box[id]` will be)
pub fn any_box(g: &Game, id: BoxId) -> &crate::scene_boxes::Box {
    match made_get(id) { Some(b) => b, None => &g.col.boxes[id] }
}
/// …to write
pub fn any_box_mut(g: &mut Game, id: BoxId) -> &mut crate::scene_boxes::Box {
    let i = id.ix();
    if i < MADE_BASE { return &mut g.col.boxes[id]; }
    let v = unsafe { &mut *MADE.0.get() };
    &mut *v[i - MADE_BASE]
}
