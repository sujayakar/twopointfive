#![allow(unused_variables)]
//! Every function and method of `seenwalk`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/seenwalk.ts:53 sharedOf)
pub fn shared_of(g: &mut crate::game::Game) -> &mut crate::seenwalk::Shared { todo!() }

/// (src/game/seenwalk.ts:62 seenReset)
pub fn seen_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/seenwalk.ts:66 seenPrice)
pub fn seen_price(g: &mut crate::game::Game, from: V3, base: &[V3]) -> Option<crate::seenwalk::SeenPrice> { todo!() }


/// INVENTED, what a SeenPrice answers for a cell, so a caller can hand one to nav_grid_find_path or cleared_rooms_dear_along: |g, ix, iz| seen_price_at(g, &price, ix, iz). `dear` outside the price's box; else the cell's block, remembered in g.m.seenwalk.shared's memo for the tick and shared by every walk asked, costs `dear` if lit and in view of one of the price's eyes, else 1 (seenwalk.ts:75 the closure seenPrice returns)
pub fn seen_price_at(g: &mut crate::game::Game, price: &crate::seenwalk::SeenPrice, ix: f64, iz: f64) -> f64 { todo!() }
