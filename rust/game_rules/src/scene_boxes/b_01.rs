//! THE BRING-UP'S OWN PART of scene_boxes: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// THE BOX TABLES' MAKERS. Invented by the skeleton, so they have no TypeScript line: written here. A box of the world is a row of `g.tab.boxes` and is never
// dealt again, so its number is the same on every host. A box of the frame, or one its maker keeps a while, is a row of `g.tab.frame_boxes`: a row is freed by
// the tick's own listing, not by a host's drawing, but a freed row is dealt again: it names a box only while its holder holds it.

/// WHAT THE PICTURE'S TABLE (`WORLD.max_boxes`) KEEPS FOR THE FRAME'S OWN BOXES: the men, the props, what is in flight. A level's walls and what is shed of them
/// stay out of it. It is a number of the game and not a count of this frame's list, so that what the game makes of a broken wall is the same on every host.
pub const FRAME_BOXES: f64 = 768.0;

/// a box made to stay: a row of g.tab.boxes for good
pub fn box_new(g: &mut Game, b: crate::scene_boxes::Box) -> BoxId { g.tab.boxes.push(b) }
/// a box made for this frame's list: a row freed by boxes_frame_reset is used again, else a new row
pub fn box_frame(g: &mut Game, b: crate::scene_boxes::Box) -> FrameBoxId {
    let id = box_kept(g, b);
    g.box_frame_rows.push(id);
    id
}
/// THE BOX WORLD LISTED AS A TICK OPENS, AND THE ROWS OF THE FRAMES GONE BY FREE AGAIN. Two lists hold a frame box's ID: the frame's own (`g.dyn_boxes`, made
/// again every tick) and the engine's (`g.engine.world`, made again here). A row is freed when it is not one of the frame that was just listed.
pub fn box_world_listed(g: &mut Game) {
    crate::scene_boxes::box_world_list(g);
    boxes_frame_reset(g);
}
fn boxes_frame_reset(g: &mut Game) {
    let rows = core::mem::take(&mut g.box_frame_rows);
    let mut live = vec![false; g.tab.frame_boxes.len()];
    for b in g.dyn_boxes.iter() { if let AnyBoxId::Frame(f) = *b { live[f.ix()] = true; } }
    for id in rows {
        if live[id.ix()] { g.box_frame_rows.push(id); continue; }
        // (what a maker noted beside the box that stood in this row, a thin line's width or an infrared mark, goes with the box)
        crate::thinline::thin_forget(g, id.into());
        crate::irlaser::ir_forget(g, id.into());
        g.box_frame_free.push(id);
    }
}

/// A BOX ITS MAKER KEEPS FOR A WHILE AND THEN LETS GO (a remembered thing's copy in the fog of war, an fx row's box): a freed row used again, else a new
/// row. Not one of the frame's rows: it stands until its holder hands it to `box_let_go`.
pub fn box_kept(g: &mut Game, b: crate::scene_boxes::Box) -> FrameBoxId {
    match g.box_frame_free.pop() { Some(id) => { g.tab.frame_boxes[id] = b; id } None => g.tab.frame_boxes.push(b) }
}
/// The holder of a kept box has dropped it (the TypeScript drops the object). The frame's list or the engine's may still name it, so it is not free at
/// once: it joins the frame's rows, and the next listing frees it once it is on no list (`box_world_listed`).
pub fn box_let_go(g: &mut Game, id: FrameBoxId) { g.box_frame_rows.push(id); }

/// makeBox for a box pushed straight onto the frame's list: a row for this frame (boxes.ts makeBox, where its caller keeps nothing of it)
pub fn make_box_frame(g: &mut Game, p: &crate::scene_boxes::MakeBoxP) -> FrameBoxId { let b = crate::scene_boxes::make_box_value(p); box_frame(g, b) }
/// makeBox for a box its maker keeps until it hands it to `box_let_go` (one that stands by a short timer on `g.fx`)
pub fn make_box_kept(g: &mut Game, p: &crate::scene_boxes::MakeBoxP) -> FrameBoxId { let b = crate::scene_boxes::make_box_value(p); box_kept(g, b) }
/// boxOf for a box pushed straight onto the frame's list: a row for this frame (boxes.ts boxOf)
pub fn box_of_frame(g: &mut Game, c: V3, h: V3, yaw: f64, albedo: V3, emissive: V3, flags: f64, owner: f64, rot: Option<Quat>, name: Option<&str>) -> FrameBoxId {
    let b = crate::scene_boxes::make_box_value(&crate::scene_boxes::MakeBoxP { c, h, yaw: Some(yaw), albedo: Some(albedo), emissive: Some(emissive), flags: Some(flags), owner: Some(owner), rot, name: name.map(String::from), ..Default::default() });
    box_frame(g, b)
}
