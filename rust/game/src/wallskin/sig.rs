#![allow(unused_variables)]
//! Every function and method of `wallskin`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/wallskin.ts:82 into)
pub fn into(g: &mut crate::game::Game, b: BoxId, p: [f64; 3]) -> [f64; 3] { todo!() }

/// (src/game/wallskin.ts:83 outOf)
pub fn out_of(g: &mut crate::game::Game, b: BoxId, l: [f64; 3]) -> [f64; 3] { todo!() }

/// (src/game/wallskin.ts:84 inside)
pub fn inside(r: [f64; 4], u: f64, v: f64) -> bool { todo!() }

/// (src/game/wallskin.ts:92 standing)
pub fn standing(hu: f64, hv: f64, on: &[[f64; 4]], near: &[f64], open: &[Vec<[f64; 4]>], cols: Option<bool>, parts: Option<&[[f64; 4]]>) -> Vec<[f64; 4]> { todo!() }

/// (src/game/wallskin.ts:139 WallSkin.constructor)
pub fn wall_skin_new(g: &mut crate::game::Game, authored: f64, own: &mut dyn FnMut(&mut crate::game::Game, f64) -> bool, brk: &mut dyn FnMut(&mut crate::game::Game, f64) -> Option<crate::wallskin::Brk>, all: &mut dyn FnMut(&mut crate::game::Game) -> Vec<crate::wallskin::Brk>) -> crate::wallskin::WallSkin { todo!() }

/// (src/game/wallskin.ts:150 WallSkin.opened)
pub fn wall_skin_opened(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike) { todo!() }

/// (src/game/wallskin.ts:155 WallSkin.clear)
pub fn wall_skin_clear(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallskin.ts:156 WallSkin.fail)
pub fn wall_skin_fail(g: &mut crate::game::Game, e: crate::Opaque) { todo!() }

/// (src/game/wallskin.ts:157 WallSkin.count)
pub fn wall_skin_count(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallskin.ts:160 WallSkin.update)
pub fn wall_skin_update(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallskin.ts:166 WallSkin.boxes)
pub fn wall_skin_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/wallskin.ts:186 WallSkin.seat)
pub fn wall_skin_seat(g: &mut crate::game::Game, k: f64, s: BoxId, t: f64) -> Option<SeatId> { todo!() }

/// (src/game/wallskin.ts:200 WallSkin.backs)
pub fn wall_skin_backs(g: &mut crate::game::Game, j: f64) -> Vec<SeatId> { todo!() }

/// (src/game/wallskin.ts:220 WallSkin.skinsOf)
pub fn wall_skin_skins_of(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike) -> Vec<f64> { todo!() }

/// (src/game/wallskin.ts:234 WallSkin.cells)
pub fn wall_skin_cells(g: &mut crate::game::Game, s: BoxId, q: SeatId, w: &crate::edge_look::BreakLike, a: f64) -> Vec<[f64; 4]> { todo!() }

/// (src/game/wallskin.ts:245 WallSkin.recut)
pub fn wall_skin_recut(g: &mut crate::game::Game, j: f64) { todo!() }
