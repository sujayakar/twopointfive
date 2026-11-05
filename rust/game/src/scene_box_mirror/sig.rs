#![allow(unused_variables)]
//! Every function and method of `scene_box_mirror`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/scene/boxMirror.ts:63 leafWrote)
pub fn leaf_wrote(g: &mut crate::game::Game) { todo!() }

/// (src/scene/boxMirror.ts:67 thingWrote)
pub fn thing_wrote(g: &mut crate::game::Game) { todo!() }

/// (src/scene/boxMirror.ts:83 tooBig)
pub fn too_big(name: Option<&str>, why: &str) -> String { todo!() }

/// (src/scene/boxMirror.ts:92 mirrorsShut)
pub fn mirrors_shut(g: &mut crate::game::Game) { todo!() }

/// (src/scene/boxMirror.ts:106 kindOf)
pub fn kind_of(g: &mut crate::game::Game, b: BoxId, cut_on: bool) -> f64 { todo!() }

/// (src/scene/boxMirror.ts:110 kindAt)
pub fn kind_at(g: &mut crate::game::Game, b: BoxId, i: f64, cut_on: bool, walk: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>) -> f64 { todo!() }

/// (src/scene/boxMirror.ts:112 same)
pub fn same(g: &mut crate::game::Game, m: &[f64], fl: &[u32], o: f64, fi: f64, b: BoxId, kind: f64, cut0: f64) -> bool { todo!() }

/// (src/scene/boxMirror.ts:127 put)
pub fn put(g: &mut crate::game::Game, m: &mut Vec<f64>, fl: &mut Vec<u32>, o: f64, fi: f64, b: BoxId, kind: f64, cut0: f64, cut_row: f64) { todo!() }

/// (src/scene/boxMirror.ts:183 BoxMirror.tables)
pub fn box_mirror_tables(g: &mut crate::game::Game) -> JsMap<String, f64> { todo!() }

/// (src/scene/boxMirror.ts:198 BoxMirror.boot)
pub fn box_mirror_boot(g: &mut crate::game::Game) { todo!() }

/// (src/scene/boxMirror.ts:212 BoxMirror.take)
pub fn box_mirror_take(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/scene/boxMirror.ts:228 BoxMirror.open)
pub fn box_mirror_open(g: &mut crate::game::Game, boxes: &[BoxId], cells: &[Vec<f64>], tilted_at: &[f64], w: f64, h: f64, cs: f64, leaves: &[BoxId], walk: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, glass: Option<&crate::scene_box_mirror::PaneSource>, lines: bool, circles: bool, rays: bool) { todo!() }

/// (src/scene/boxMirror.ts:243 BoxMirror.shut)
pub fn box_mirror_shut(g: &mut crate::game::Game) { todo!() }

/// (src/scene/boxMirror.ts:252 BoxMirror.outgrown)
pub fn box_mirror_outgrown(g: &mut crate::game::Game, what: &str, has: f64, cap: f64) { todo!() }

/// (src/scene/boxMirror.ts:260 BoxMirror.grid)
pub fn box_mirror_grid(g: &mut crate::game::Game, cells: &[Vec<f64>], tilted_at: &[f64], w: f64, h: f64, boxes: f64) { todo!() }

/// (src/scene/boxMirror.ts:271 BoxMirror.statics)
pub fn box_mirror_statics(g: &mut crate::game::Game, boxes: &[BoxId], all: bool) { todo!() }

/// (src/scene/boxMirror.ts:289 BoxMirror.gridDirty)
pub fn box_mirror_grid_dirty(g: &mut crate::game::Game, m: &[f64], o: f64) { todo!() }

/// (src/scene/boxMirror.ts:291 BoxMirror.gridCopy)
pub fn box_mirror_grid_copy(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/scene/boxMirror.ts:293 BoxMirror.gridDrop)
pub fn box_mirror_grid_drop(g: &mut crate::game::Game) { todo!() }

/// (src/scene/boxMirror.ts:298 BoxMirror.wrote)
pub fn box_mirror_wrote(g: &mut crate::game::Game) { todo!() }

/// (src/scene/boxMirror.ts:299 BoxMirror.debt)
pub fn box_mirror_debt(g: &mut crate::game::Game, what: f64) { todo!() }

/// (src/scene/boxMirror.ts:303 BoxMirror.glassBegin)
pub fn box_mirror_glass_begin(g: &mut crate::game::Game) { todo!() }

/// (src/scene/boxMirror.ts:304 BoxMirror.glassEnd)
pub fn box_mirror_glass_end(g: &mut crate::game::Game) { todo!() }

/// (src/scene/boxMirror.ts:306 BoxMirror.glassWrote)
pub fn box_mirror_glass_wrote(g: &mut crate::game::Game) { todo!() }

/// (src/scene/boxMirror.ts:308 BoxMirror.glassIs)
pub fn box_mirror_glass_is(g: &mut crate::game::Game, g_2: Option<&crate::scene_box_mirror::PaneSource>) { todo!() }

/// (src/scene/boxMirror.ts:310 BoxMirror.owesBoxes)
pub fn box_mirror_owes_boxes(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/scene/boxMirror.ts:315 BoxMirror.snow)
pub fn box_mirror_snow(g: &mut crate::game::Game, kinds: &[u8]) { todo!() }

/// (src/scene/boxMirror.ts:321 BoxMirror.again)
pub fn box_mirror_again(g: &mut crate::game::Game, boxes: &[BoxId], walk: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>) { todo!() }

/// (src/scene/boxMirror.ts:337 BoxMirror.wroteGrid)
pub fn box_mirror_wrote_grid(g: &mut crate::game::Game) { todo!() }

/// (src/scene/boxMirror.ts:343 BoxMirror.panes)
pub fn box_mirror_panes(g: &mut crate::game::Game) { todo!() }

/// (src/scene/boxMirror.ts:355 BoxMirror.pane)
pub fn box_mirror_pane(g: &mut crate::game::Game, m: &mut Vec<f64>, w: &mut Vec<u32>, d: f64, o: f64, p: &crate::scene_box_mirror::PaneRow) { todo!() }

/// (src/scene/boxMirror.ts:376 BoxMirror.viewed)
pub fn box_mirror_viewed(g: &mut crate::game::Game) { todo!() }

/// (src/scene/boxMirror.ts:377 BoxMirror.viewIn)
pub fn box_mirror_view_in(g: &mut crate::game::Game) { todo!() }

/// (src/scene/boxMirror.ts:387 BoxMirror.glassIn)
pub fn box_mirror_glass_in(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/scene/boxMirror.ts:390 BoxMirror.glassPlain)
pub fn box_mirror_glass_plain(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/scene/boxMirror.ts:392 BoxMirror.stalePanes)
pub fn box_mirror_stale_panes(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/scene/boxMirror.ts:411 BoxMirror.leaves)
pub fn box_mirror_leaves(g: &mut crate::game::Game, dyn_: &[BoxId]) -> f64 { todo!() }

/// (src/scene/boxMirror.ts:427 BoxMirror.things)
pub fn box_mirror_things(g: &mut crate::game::Game, p: &crate::scene_box_mirror::Furniture) -> bool { todo!() }

/// (src/scene/boxMirror.ts:447 BoxMirror.thingBox)
pub fn box_mirror_thing_box(g: &mut crate::game::Game, p: &crate::scene_box_mirror::Furniture, at: f64) -> Option<BoxId> { todo!() }

/// (src/scene/boxMirror.ts:449 BoxMirror.staleThings)
pub fn box_mirror_stale_things(g: &mut crate::game::Game, p: &crate::scene_box_mirror::Furniture) -> f64 { todo!() }

/// (src/scene/boxMirror.ts:460 BoxMirror.glassBi)
pub fn box_mirror_glass_bi(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/scene/boxMirror.ts:461 BoxMirror.glassT)
pub fn box_mirror_glass_t(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/scene/boxMirror.ts:467 BoxMirror.rules)
pub fn box_mirror_rules(g: &mut crate::game::Game, dyn_: &[BoxId]) -> bool { todo!() }

/// (src/scene/boxMirror.ts:475 BoxMirror.followLeaf)
pub fn box_mirror_follow_leaf(g: &mut crate::game::Game, i: f64, b: BoxId) -> f64 { todo!() }

/// (src/scene/boxMirror.ts:482 BoxMirror.followRow)
pub fn box_mirror_follow_row(g: &mut crate::game::Game, b: BoxId, r: &[f64]) -> f64 { todo!() }

/// (src/scene/boxMirror.ts:484 BoxMirror.leafRow)
pub fn box_mirror_leaf_row(g: &mut crate::game::Game, i: f64) -> Vec<f64> { todo!() }

/// (src/scene/boxMirror.ts:487 BoxMirror.leavesTurned)
pub fn box_mirror_leaves_turned(g: &mut crate::game::Game) { todo!() }

/// (src/scene/boxMirror.ts:491 BoxMirror.stale)
pub fn box_mirror_stale(g: &mut crate::game::Game, boxes: &[BoxId], dyn_: &[BoxId], set: f64) -> f64 { todo!() }
