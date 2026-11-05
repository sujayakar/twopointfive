#![allow(unused_variables)]
//! Every function and method of `wallbreak`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/wallbreak.ts:82 wallBreakOn)
pub fn wall_break_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/wallbreak.ts:163 aabbOf)
pub fn aabb_of(g: &mut crate::game::Game, b: BoxId) -> crate::wallbreak::AABB { todo!() }

/// (src/game/wallbreak.ts:167 overlapXZ)
pub fn overlap_xz(a: crate::wallbreak::AABB, b: crate::wallbreak::AABB, eps: Option<f64>) -> bool { todo!() }

/// (src/game/wallbreak.ts:191 WallBreak.constructor)
pub fn wall_break_new(g: &mut crate::game::Game) -> crate::wallbreak::WallBreak { todo!() }

/// (src/game/wallbreak.ts:198 WallBreak.over)
pub fn wall_break_over(g: &mut crate::game::Game, b: WallbreakBreakableId, need: f64) -> bool { todo!() }

/// (src/game/wallbreak.ts:208 WallBreak.piecesSure)
pub fn wall_break_pieces_sure(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/wallbreak.ts:216 WallBreak.index)
pub fn wall_break_index(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallbreak.ts:254 WallBreak.inventory)
pub fn wall_break_inventory(g: &mut crate::game::Game) -> crate::wallbreak::WallBreakInventory { todo!() }

/// (src/game/wallbreak.ts:259 WallBreak.breakableAt)
pub fn wall_break_breakable_at(g: &mut crate::game::Game, index: f64) -> Option<crate::wallbreak::WallBreakBreakableAt> { todo!() }

/// (src/game/wallbreak.ts:263 WallBreak.adoptable)
pub fn wall_break_adoptable(g: &mut crate::game::Game, index: f64) -> bool { todo!() }

/// (src/game/wallbreak.ts:274 WallBreak.hardCols)
pub fn wall_break_hard_cols(g: &mut crate::game::Game, b: BoxId) -> f64 { todo!() }

/// (src/game/wallbreak.ts:275 WallBreak.adopt)
pub fn wall_break_adopt(g: &mut crate::game::Game, index: f64) -> bool { todo!() }

/// (src/game/wallbreak.ts:287 WallBreak.workLeft)
pub fn wall_break_work_left(g: &mut crate::game::Game, index: f64, hit_p: V3, dir: V3, through: Option<f64>) -> Option<f64> { todo!() }

/// (src/game/wallbreak.ts:305 WallBreak.all)
pub fn wall_break_all(g: &mut crate::game::Game) -> Vec<crate::wallbreak::WallBreakAllItem> { todo!() }

/// (src/game/wallbreak.ts:308 WallBreak.hash)
pub fn wall_break_hash(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/wallbreak.ts:309 WallBreak.toLocal)
pub fn wall_break_to_local(g: &mut crate::game::Game, b: WallbreakBreakableId, p: V3) -> V3 { todo!() }

/// (src/game/wallbreak.ts:310 WallBreak.toWorld)
pub fn wall_break_to_world(g: &mut crate::game::Game, b: WallbreakBreakableId, l: V3) -> V3 { todo!() }

/// (src/game/wallbreak.ts:311 WallBreak.ci)
pub fn wall_break_ci(g: &mut crate::game::Game, b: WallbreakBreakableId, ix: f64, iy: f64, iz: f64) -> f64 { todo!() }

/// (src/game/wallbreak.ts:312 WallBreak.xb)
pub fn wall_break_xb(g: &mut crate::game::Game, b: WallbreakBreakableId, ix: f64) -> f64 { todo!() }

/// (src/game/wallbreak.ts:313 WallBreak.zb)
pub fn wall_break_zb(g: &mut crate::game::Game, b: WallbreakBreakableId, iz: f64) -> f64 { todo!() }

/// (src/game/wallbreak.ts:316 WallBreak.cellAt)
pub fn wall_break_cell_at(g: &mut crate::game::Game, b: WallbreakBreakableId, p: V3, inside: Option<BoxId>) -> [f64; 3] { todo!() }

/// (src/game/wallbreak.ts:324 WallBreak.cellCentre)
pub fn wall_break_cell_centre(g: &mut crate::game::Game, b: WallbreakBreakableId, ix: f64, iy: f64, iz: f64) -> V3 { todo!() }

/// (src/game/wallbreak.ts:326 WallBreak.cellBox)
pub fn wall_break_cell_box(g: &mut crate::game::Game, b: WallbreakBreakableId, ix: f64, iy: f64, iz: f64) -> BoxId { todo!() }

/// (src/game/wallbreak.ts:332 WallBreak.rectBox)
pub fn wall_break_rect_box(g: &mut crate::game::Game, b: WallbreakBreakableId, arg1: [f64; 6]) -> crate::wallbreak::WallBreakRectBox { todo!() }

/// (src/game/wallbreak.ts:336 WallBreak.pieces)
pub fn wall_break_pieces(g: &mut crate::game::Game, b: WallbreakBreakableId, open: Option<&[u8]>) -> Vec<[f64; 6]> { todo!() }

/// (src/game/wallbreak.ts:350 WallBreak.rebuild)
pub fn wall_break_rebuild(g: &mut crate::game::Game, b: WallbreakBreakableId) -> bool { todo!() }

/// (src/game/wallbreak.ts:378 WallBreak.bump)
pub fn wall_break_bump(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallbreak.ts:382 WallBreak.openCells)
pub fn wall_break_open_cells(g: &mut crate::game::Game, b: WallbreakBreakableId, cells: &[f64], cause: crate::wallbreak::WallBreakOpenCellsCause, who: f64, dir: Option<V3>, quiet: Option<bool>, budget: Option<&mut crate::break_table::ShedBudget>) -> Option<crate::wallbreak::LevelChange> { todo!() }

/// (src/game/wallbreak.ts:419 WallBreak.flags0)
pub fn wall_break_flags0(g: &mut crate::game::Game, i: f64) -> Option<f64> { todo!() }

/// (src/game/wallbreak.ts:422 WallBreak.raze)
pub fn wall_break_raze(g: &mut crate::game::Game, id_2: f64, who: f64, dir: Option<V3>, budget: Option<&mut crate::wallbreak::WallBreakRazeBudget>) -> Option<crate::wallbreak::LevelChange> { todo!() }

/// (src/game/wallbreak.ts:425 WallBreak.razeIn)
pub fn wall_break_raze_in(g: &mut crate::game::Game, id_2: f64, r: Option<[f64; 4]>, who: f64, dir: Option<V3>, budget: Option<&mut crate::wallbreak::WallBreakRazeInBudget>) -> Option<crate::wallbreak::LevelChange> { todo!() }

/// (src/game/wallbreak.ts:437 WallBreak.sillOpenIn)
pub fn wall_break_sill_open_in(g: &mut crate::game::Game, id_2: f64, r: [f64; 4]) -> Option<[f64; 4]> { todo!() }

/// (src/game/wallbreak.ts:449 WallBreak.note)
pub fn wall_break_note(g: &mut crate::game::Game, c_2: &crate::wallbreak::WallBreakNoteC) -> crate::wallbreak::LevelChange { todo!() }

/// (src/game/wallbreak.ts:456 WallBreak.onChange)
pub fn wall_break_on_change(g: &mut crate::game::Game, fn_: &crate::wallbreak::ChangeListener) { todo!() }

/// (src/game/wallbreak.ts:458 WallBreak.since)
pub fn wall_break_since(g: &mut crate::game::Game, serial: f64) -> Option<Vec<crate::wallbreak::LevelChange>> { todo!() }

/// (src/game/wallbreak.ts:461 WallBreak.shed)
pub fn wall_break_shed(g: &mut crate::game::Game, b: WallbreakBreakableId, cell: BoxId, dir: Option<V3>, who: f64, budget: &mut crate::break_table::ShedBudget) { todo!() }

/// (src/game/wallbreak.ts:483 WallBreak.restsOn)
pub fn wall_break_rests_on(g: &mut crate::game::Game, j: f64, a: crate::wallbreak::AABB, extra: Option<crate::wallbreak::AABB>) -> Vec<crate::wallbreak::AABB> { todo!() }

/// (src/game/wallbreak.ts:495 WallBreak.held)
pub fn wall_break_held(g: &mut crate::game::Game, j: f64, a: crate::wallbreak::AABB, extra: Option<crate::wallbreak::AABB>) -> bool { todo!() }

/// (src/game/wallbreak.ts:515 WallBreak.near)
pub fn wall_break_near(g: &mut crate::game::Game, a: crate::wallbreak::AABB) -> Vec<f64> { todo!() }

/// (src/game/wallbreak.ts:527 WallBreak.settle)
pub fn wall_break_settle(g: &mut crate::game::Game, b: WallbreakBreakableId, rect: crate::wallbreak::AABB, dir: Option<V3>, who: f64, quiet: bool, rects: &mut Vec<[f64; 4]>) { todo!() }

/// (src/game/wallbreak.ts:533 WallBreak.drop)
pub fn wall_break_drop(g: &mut crate::game::Game, b: Option<WallbreakBreakableId>, queue: &mut Vec<crate::wallbreak::AABB>, dir: Option<V3>, who: f64, quiet: bool, rects: &mut Vec<[f64; 4]>) { todo!() }

/// (src/game/wallbreak.ts:552 WallBreak.hungAt)
pub fn wall_break_hung_at(g: &mut crate::game::Game, wall: f64, at: V3, r: f64) -> Vec<f64> { todo!() }

/// (src/game/wallbreak.ts:569 WallBreak.boardLike)
pub fn wall_break_board_like(g: &mut crate::game::Game, j: f64) -> bool { todo!() }

/// (src/game/wallbreak.ts:582 WallBreak.hangers)
pub fn wall_break_hangers(g: &mut crate::game::Game, b: WallbreakBreakableId, rect: crate::wallbreak::AABB, dir: Option<V3>, who: f64, quiet: bool, queue: &mut Vec<crate::wallbreak::AABB>, rects: &mut Vec<[f64; 4]>) { todo!() }

/// (src/game/wallbreak.ts:612 WallBreak.fall)
pub fn wall_break_fall(g: &mut crate::game::Game, j: f64, a: crate::wallbreak::AABB, dir: Option<V3>, who: f64, quiet: bool, rects: &mut Vec<[f64; 4]>) -> bool { todo!() }

/// (src/game/wallbreak.ts:639 WallBreak.round)
pub fn wall_break_round(g: &mut crate::game::Game, who: f64, hit_p: V3, dir: V3, index: f64, def: &crate::weapons::WeaponDef, chaos: f64) { todo!() }

/// (src/game/wallbreak.ts:656 WallBreak.planBlast)
pub fn wall_break_plan_blast(g: &mut crate::game::Game, at: V3, opts: crate::wallbreak::BlastOpts, r: f64, hung: Option<&[f64]>) -> Vec<crate::wallbreak::WallBreakPlanBlastItem> { todo!() }

/// (src/game/wallbreak.ts:663 WallBreak.planOf)
pub fn wall_break_plan_of(g: &mut crate::game::Game, at: V3, opts: crate::wallbreak::BlastOpts, r: f64) -> Vec<crate::wallbreak::WallBreakPlanOfItem> { todo!() }

/// (src/game/wallbreak.ts:723 WallBreak.wouldOpen)
pub fn wall_break_would_open(g: &mut crate::game::Game, at: V3, opts: Option<crate::wallbreak::BlastOpts>) -> Vec<crate::wallbreak::WallOpening> { todo!() }

/// (src/game/wallbreak.ts:744 WallBreak.blast)
pub fn wall_break_blast(g: &mut crate::game::Game, at: V3, opts: Option<crate::wallbreak::BlastOpts>) -> crate::wallbreak::BlastResult { todo!() }

/// (src/game/wallbreak.ts:745 WallBreak.blastOf)
pub fn wall_break_blast_of(g: &mut crate::game::Game, at: V3, opts: crate::wallbreak::BlastOpts) -> crate::wallbreak::BlastResult { todo!() }

/// (src/game/wallbreak.ts:780 WallBreak.reset)
pub fn wall_break_reset(g: &mut crate::game::Game, silent: Option<bool>) { todo!() }

/// (src/game/wallbreak.ts:799 WallBreak.snapshot)
pub fn wall_break_snapshot(g: &mut crate::game::Game) -> Option<crate::wallbreak::WallsSave> { todo!() }

/// (src/game/wallbreak.ts:812 WallBreak.check)
pub fn wall_break_check(g: &mut crate::game::Game, s: Option<&crate::wallbreak::WallsSave>) -> Option<String> { todo!() }

/// (src/game/wallbreak.ts:824 WallBreak.restore)
pub fn wall_break_restore(g: &mut crate::game::Game, s: Option<&crate::wallbreak::WallsSave>) { todo!() }

/// (src/game/wallbreak.ts:846 WallBreak.summary)
pub fn wall_break_summary(g: &mut crate::game::Game) -> String { todo!() }
