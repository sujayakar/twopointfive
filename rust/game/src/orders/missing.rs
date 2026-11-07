//! Functions of `orders` that a caller names and nobody has written: a call says so by name.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// (src/game/orders.ts:2583 SquadOrders.hud)
pub fn squad_orders_hud(g: &mut crate::game::Game, kept: Option<&mut crate::orders::HoverKept>) -> crate::orders::SquadOrdersHud { crate::no_body("orders::squad_orders_hud") }
/// (src/game/orders.ts:2626 SquadOrders.summary)
pub fn squad_orders_summary(g: &mut crate::game::Game) -> String { crate::no_body("orders::squad_orders_summary") }
