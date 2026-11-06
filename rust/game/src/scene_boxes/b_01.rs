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
/// THE ROWS OF FRAMES GONE BY ARE FREE AGAIN. Two lists hold a frame box's ID: the frame's own (`g.dyn_boxes`, made again every tick) and the engine's
/// (`g.engine.world`, made again when the host lists the box world, which a headless host does every few frames). So this is called by the host's listing,
/// straight after it: a row is freed when it is not one of the frame that was just listed. A host that never lists keeps every row.
pub fn boxes_frame_reset(g: &mut Game) {
    let rows = core::mem::take(&mut g.box_frame_rows);
    let mut live = vec![false; g.tab.boxes.len()];
    for b in g.dyn_boxes.iter() { live[b.ix()] = true; }
    for id in rows { if live[id.ix()] { g.box_frame_rows.push(id); } else { g.box_frame_free.push(id); } }
}

/// makeBox for a box pushed straight onto the frame's list: a row for this frame (boxes.ts makeBox, where its caller keeps nothing of it)
pub fn make_box_frame(g: &mut Game, p: &crate::scene_boxes::MakeBoxP) -> BoxId { let b = crate::scene_boxes::make_box_value(p); box_frame(g, b) }
/// boxOf for a box pushed straight onto the frame's list: a row for this frame (boxes.ts boxOf)
pub fn box_of_frame(g: &mut Game, c: V3, h: V3, yaw: f64, albedo: V3, emissive: V3, flags: f64, owner: f64, rot: Option<Quat>, name: Option<&str>) -> BoxId {
    let b = crate::scene_boxes::make_box_value(&crate::scene_boxes::MakeBoxP { c, h, yaw: Some(yaw), albedo: Some(albedo), emissive: Some(emissive), flags: Some(flags), owner: Some(owner), rot, name: name.map(String::from), ..Default::default() });
    box_frame(g, b)
}
