//! src/game/squadcast.ts lines 1 to 29
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A second cast of the squad's code: its own orders and door tools, each stood up with `g.orders` / `g.door_tools` reading as the
/// TypeScript's view has them at that moment; Sam's pair is back in place on return.   (squadcast.ts squadCast)
pub fn squad_cast(g: &mut crate::game::Game, of: &crate::orders::OtherCast) -> crate::squadcast::SquadCast {
    let was_in_cast = core::mem::replace(&mut g.in_cast, true);
    // The orders are made while `g.orders` is still Sam's (`over` has no 'orders' yet); the tools see the cast's orders and still Sam's tools.
    let orders = crate::orders::squad_orders_new(g, Some(of));
    let sams_orders = core::mem::replace(&mut g.orders, orders);
    let tools = crate::doortools::door_tools_new(g);
    let orders = core::mem::replace(&mut g.orders, sams_orders);
    g.in_cast = was_in_cast;
    crate::squadcast::SquadCast { orders, tools }
}
