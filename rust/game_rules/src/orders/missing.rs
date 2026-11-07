//! Functions of `orders` that a caller names and nobody has written: a call says so by name.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// (src/game/orders.ts:2583 SquadOrders.hud)
/// NOT WRITTEN ON PURPOSE: the orders' panel is built on the page, from the puppet, by the TypeScript of this name; the Rust game draws no HUD.
pub fn squad_orders_hud(g: &mut crate::game::Game, kept: Option<&mut crate::orders::HoverKept>) -> crate::orders::SquadOrdersHud { crate::no_body("orders::squad_orders_hud") }
