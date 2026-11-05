#![allow(unused_variables)]
//! Every function and method of `break_table`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/breakTable.ts:137 matOfSurf)
pub fn mat_of_surf(s: crate::scene_materials::Surf) -> crate::break_table::Mat { todo!() }

/// (src/game/breakTable.ts:139 matOf)
pub fn mat_of(g: &mut crate::game::Game, kind: crate::debris::ChunkKind, box_: &crate::break_table::MatOfBox) -> crate::break_table::Mat { todo!() }

/// (src/game/breakTable.ts:151 coreOf)
pub fn core_of(row: &crate::break_table::BreakRow, paint: V3) -> V3 { todo!() }

/// (src/game/breakTable.ts:154 faceCore)
pub fn face_core(row: &crate::break_table::BreakRow, paint: V3) -> Option<V3> { todo!() }

/// (src/game/breakTable.ts:156 lerp)
pub fn lerp(r: [f64; 2], t: f64) -> f64 { todo!() }

/// (src/game/breakTable.ts:162 slabOutline)
pub fn slab_outline(l: f64, s: f64, tears: f64, r: &mut dyn FnMut() -> f64) -> Vec<[f64; 2]> { todo!() }

/// (src/game/breakTable.ts:191 wedgeOutline)
pub fn wedge_outline(l: f64, s: f64, r: &mut dyn FnMut() -> f64, flip: Option<bool>) -> Vec<[f64; 2]> { todo!() }

/// (src/game/breakTable.ts:204 snappedOutline)
pub fn snapped_outline(l: f64, s: f64, r: &mut dyn FnMut() -> f64) -> Vec<[f64; 2]> { todo!() }

/// (src/game/breakTable.ts:215 stripOutline)
pub fn strip_outline(l: f64, s: f64, r: &mut dyn FnMut() -> f64) -> Vec<[f64; 2]> { todo!() }

/// (src/game/breakTable.ts:221 outlineOf)
pub fn outline_of(shape: crate::break_table::Shape, l: f64, s: f64, row: &crate::break_table::BreakRow, r: &mut dyn FnMut() -> f64, flip: bool) -> Vec<[f64; 2]> { todo!() }

/// (src/game/breakTable.ts:231 shapeBox)
pub fn shape_box(g: &mut crate::game::Game, shape: crate::break_table::Shape, thin: f64, half_t: f64, l: f64, s: f64, long_first: bool, row: &crate::break_table::BreakRow, r: &mut dyn FnMut(&mut crate::game::Game) -> f64, flip: Option<bool>) -> Option<crate::break_table::Shaped> { todo!() }

/// (src/game/breakTable.ts:266 raggedCut)
pub fn ragged_cut(g: &mut crate::game::Game, row: &crate::break_table::BreakRow, h: V3, faces: &[crate::break_table::OpenFace]) -> Option<Vec<crate::scene_cut::CutPlane>> { todo!() }

/// (src/game/breakTable.ts:321 wallNormal)
pub fn wall_normal(g: &mut crate::game::Game, b: BoxId, toward: V3, keep: Option<f64>) -> V3 { todo!() }

/// (src/game/breakTable.ts:338 Breaks.constructor)
pub fn breaks_new(g: &mut crate::game::Game) -> crate::break_table::Breaks { todo!() }

/// (src/game/breakTable.ts:339 Breaks.hash)
pub fn breaks_hash(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/breakTable.ts:344 Breaks.phash)
pub fn breaks_phash(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/breakTable.ts:345 Breaks.reset)
pub fn breaks_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/breakTable.ts:346 Breaks.small)
pub fn breaks_small(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/breakTable.ts:349 Breaks.rel)
pub fn breaks_rel(g: &mut crate::game::Game, c_2: DebrisChunkId) -> f64 { todo!() }

/// (src/game/breakTable.ts:351 Breaks.share)
pub fn breaks_share(g: &mut crate::game::Game) -> crate::break_table::BreaksShare { todo!() }

/// (src/game/breakTable.ts:356 Breaks.load)
pub fn breaks_load(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/breakTable.ts:361 Breaks.lands)
pub fn breaks_lands(g: &mut crate::game::Game, kind: crate::debris::ChunkKind, box_: BoxId, speed: f64) -> crate::break_table::BreaksLands { todo!() }

/// (src/game/breakTable.ts:366 Breaks.born)
pub fn breaks_born(g: &mut crate::game::Game, c_2: DebrisChunkId) { todo!() }

/// (src/game/breakTable.ts:370 Breaks.tag)
pub fn breaks_tag(g: &mut crate::game::Game, c_2: DebrisChunkId, mat: crate::break_table::Mat, gen_: f64, paint: V3, fixed: Option<bool>) { todo!() }

/// (src/game/breakTable.ts:375 Breaks.landed)
pub fn breaks_landed(g: &mut crate::game::Game, c_2: DebrisChunkId, speed: f64) { todo!() }

/// (src/game/breakTable.ts:383 Breaks.kicked)
pub fn breaks_kicked(g: &mut crate::game::Game, c_2: DebrisChunkId, kick: f64) { todo!() }

/// (src/game/breakTable.ts:389 Breaks.evicted)
pub fn breaks_evicted(g: &mut crate::game::Game, c_2: DebrisChunkId) { todo!() }

/// (src/game/breakTable.ts:396 Breaks.pick)
pub fn breaks_pick(g: &mut crate::game::Game, list: &[DebrisChunkId], ok: &mut dyn FnMut(&mut crate::game::Game, DebrisChunkId) -> bool) -> f64 { todo!() }

/// (src/game/breakTable.ts:404 Breaks.dissolve)
pub fn breaks_dissolve(g: &mut crate::game::Game, c_2: DebrisChunkId) { todo!() }

/// (src/game/breakTable.ts:421 Breaks.tick)
pub fn breaks_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/breakTable.ts:429 Breaks.tint)
pub fn breaks_tint(g: &mut crate::game::Game, row: &crate::break_table::BreakRow, paint: V3, k: f64) -> V3 { todo!() }

/// (src/game/breakTable.ts:431 Breaks.chips)
pub fn breaks_chips(g: &mut crate::game::Game, row: &crate::break_table::BreakRow, paint: V3, at: V3, dir: V3, n: f64, speed: f64, pos_jit: f64, who: f64, size_k: Option<f64>) { todo!() }

/// (src/game/breakTable.ts:440 Breaks.fines)
pub fn breaks_fines(g: &mut crate::game::Game, row: &crate::break_table::BreakRow, paint: V3, at: V3, dir: V3, k: Option<f64>) { todo!() }

/// (src/game/breakTable.ts:446 Breaks.crumble)
pub fn breaks_crumble(g: &mut crate::game::Game, c_2: DebrisChunkId, count: [f64; 2], speed: f64) { todo!() }

/// (src/game/breakTable.ts:454 Breaks.refuse)
pub fn breaks_refuse(g: &mut crate::game::Game, row: &crate::break_table::BreakRow, paint: V3, at: V3, v: V3, who: f64) { todo!() }

/// (src/game/breakTable.ts:460 Breaks.breakAgain)
pub fn breaks_break_again(g: &mut crate::game::Game, c_2: DebrisChunkId) { todo!() }

/// (src/game/breakTable.ts:505 Breaks.dress)
pub fn breaks_dress(g: &mut crate::game::Game, c_2: DebrisChunkId) { todo!() }

/// (src/game/breakTable.ts:518 Breaks.outline)
pub fn breaks_outline(g: &mut crate::game::Game, m: crate::break_table::Mat, row: &crate::break_table::BreakRow, h: V3, thin: f64, mid: f64, long: f64, hash: &mut dyn FnMut(&mut crate::game::Game) -> f64) -> Option<crate::break_table::Shaped> { todo!() }

/// (src/game/breakTable.ts:534 Breaks.shapeLoose)
pub fn breaks_shape_loose(g: &mut crate::game::Game, b: BoxId, surf: &str, key: f64) -> bool { todo!() }

/// (src/game/breakTable.ts:549 Breaks.retag)
pub fn breaks_retag(g: &mut crate::game::Game, c_2: DebrisChunkId, mat: crate::break_table::Mat) { todo!() }

/// (src/game/breakTable.ts:552 Breaks.sheet)
pub fn breaks_sheet(g: &mut crate::game::Game, c_2: DebrisChunkId) { todo!() }

/// (src/game/breakTable.ts:563 Breaks.carcass)
pub fn breaks_carcass(g: &mut crate::game::Game, c_2: DebrisChunkId, axis: f64, sign: f64, seed: Option<f64>) { todo!() }

/// (src/game/breakTable.ts:588 Breaks.kink)
pub fn breaks_kink(g: &mut crate::game::Game, c_2: DebrisChunkId, thin: f64, mid: f64, long: f64, deg: Option<[f64; 2]>, every: Option<bool>) -> bool { todo!() }

/// (src/game/breakTable.ts:612 Breaks.shedCell)
pub fn breaks_shed_cell(g: &mut crate::game::Game, surf: crate::scene_materials::Surf, cell: BoxId, dir: Option<V3>, who: f64, budget: &mut crate::break_table::ShedBudget, as_: Option<crate::break_table::Mat>) -> crate::break_table::BreaksShedCell { todo!() }

/// (src/game/breakTable.ts:654 Breaks.heap)
pub fn breaks_heap(g: &mut crate::game::Game, surf: crate::scene_materials::Surf, wall: BoxId, at: V3, from: V3, half_w: f64, who: Option<f64>, budget: Option<&mut crate::break_table::ShedBudget>, as_: Option<crate::break_table::Mat>) -> f64 { todo!() }

/// (src/game/breakTable.ts:685 Breaks.endCore)
pub fn breaks_end_core(g: &mut crate::game::Game, surf: &str, paint: V3) -> Option<V3> { todo!() }

/// (src/game/breakTable.ts:689 Breaks.ragged)
pub fn breaks_ragged(g: &mut crate::game::Game, b: &crate::break_table::CellGrid, rect: &[f64], h: V3) -> Option<Vec<crate::scene_cut::CutPlane>> { todo!() }

/// (src/game/breakTable.ts:722 Breaks.studs)
pub fn breaks_studs(g: &mut crate::game::Game, b: &crate::break_table::CellGrid) -> Vec<crate::break_table::BreaksStudsItem> { todo!() }

/// (src/game/breakTable.ts:736 Breaks.summary)
pub fn breaks_summary(g: &mut crate::game::Game) -> String { todo!() }
