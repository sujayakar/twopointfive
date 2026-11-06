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
pub fn wall_skin_new(g: &mut crate::game::Game, authored: f64) -> crate::wallskin::WallSkin { todo!() }

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
pub fn wall_skin_seat(g: &mut crate::game::Game, k: f64, s: BoxId, t: f64) -> Option<crate::wallskin::Seat> { todo!() }

/// (src/game/wallskin.ts:200 WallSkin.backs)
pub fn wall_skin_backs(g: &mut crate::game::Game, j: f64) -> Vec<crate::wallskin::Seat> { todo!() }

/// (src/game/wallskin.ts:220 WallSkin.skinsOf)
pub fn wall_skin_skins_of(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike) -> Vec<f64> { todo!() }

/// (src/game/wallskin.ts:234 WallSkin.cells)
pub fn wall_skin_cells(g: &mut crate::game::Game, s: BoxId, q: &crate::wallskin::Seat, w: &crate::edge_look::BreakLike, a: f64) -> Vec<[f64; 4]> { todo!() }

/// (src/game/wallskin.ts:245 WallSkin.recut)
pub fn wall_skin_recut(g: &mut crate::game::Game, j: f64) { todo!() }

/// INVENTED (wallbreak.ts:198, the first closure handed to WallSkin): static j has a road of its own (a fixture's box, one of the lane's destructibles)
pub fn wall_skin_own(g: &mut crate::game::Game, j: f64) -> bool { todo!() }
/// INVENTED (wallbreak.ts:198, the second): the breakable keyed k, when k is its own key
pub fn wall_skin_brk(g: &mut crate::game::Game, k: f64) -> Option<crate::wallskin::Brk> { todo!() }
/// INVENTED (wallbreak.ts:198, the third): the lane's list of breakables
pub fn wall_skin_all(g: &mut crate::game::Game) -> Vec<crate::wallskin::Brk> { todo!() }

