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

/// INVENTED BY THE SKELETON, for squadcast.ts's Proxy (lines 20-27): the cast's pair stands on Game for the call, in_cast says so, and what stood there is back on return.
/// Only those two fields and the flag: every other read and every write is the one Game's, as through the view.
pub fn with_cast<R>(g: &mut crate::game::Game, cast: &mut crate::squadcast::SquadCast, f: &mut dyn FnMut(&mut crate::game::Game) -> R) -> R {
    use core::mem::{replace, swap};
    swap(&mut g.orders, &mut cast.orders);
    swap(&mut g.door_tools, &mut cast.tools);
    let was_in_cast = replace(&mut g.in_cast, true);
    let r = f(g);
    g.in_cast = was_in_cast;
    swap(&mut g.door_tools, &mut cast.tools);
    swap(&mut g.orders, &mut cast.orders);
    r
}

/// INVENTED BY THE SKELETON: a ring of keys as a door asks it: it opens what it names, and everything if it names "*" (`every_key()`, squadcast.ts:17's set that always answers yes).
pub fn ring_has(ring: &JsSet<String>, key: &str) -> bool {
    ring.has(String::from(key)) || ring.has(String::from("*"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_key_opens_anything_and_a_ring_only_what_it_names() {
        assert!(ring_has(&every_key(), "boiler-room"));
        let mut ring = JsSet::new();
        ring.add(String::from("boiler-room"));
        assert!(ring_has(&ring, "boiler-room"));
        assert!(!ring_has(&ring, "vault"));
        assert!(!ring_has(&JsSet::new(), "vault"));
    }

    #[test]
    fn with_cast_stands_the_pair_on_game_and_puts_back_what_was_there() {
        let mut g = Game::blank();
        g.door_tools.charges = 1.0;
        g.orders.defs = vec![OrdersOrderDefId(1)];
        let mut cast = SquadCast::default();
        cast.tools.charges = 7.0;
        cast.orders.defs = vec![OrdersOrderDefId(2)];
        let got = with_cast(&mut g, &mut cast, &mut |g: &mut Game| {
            assert!(g.in_cast);
            assert_eq!(g.door_tools.charges, 7.0);
            assert_eq!(g.orders.defs, vec![OrdersOrderDefId(2)]);
            g.door_tools.charges = 9.0;    // a write in the pass is the cast's, kept for its next pass
            g.time = 5.0;                  // and any other write is the one Game's
            42
        });
        assert_eq!(got, 42);
        assert!(!g.in_cast);
        assert_eq!(g.door_tools.charges, 1.0);
        assert_eq!(g.orders.defs, vec![OrdersOrderDefId(1)]);
        assert_eq!(cast.tools.charges, 9.0);
        assert_eq!(cast.orders.defs, vec![OrdersOrderDefId(2)]);
        assert_eq!(g.time, 5.0);
    }
}
