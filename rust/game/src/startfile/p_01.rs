//! src/game/startfile.ts lines 1 to 32
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Every living one of `men` is handed the menu's own 'fall in' as a scenario stages, unspoken; returns how many took it. (startfile.ts teamStartsFallenIn)
pub fn team_starts_fallen_in(g: &mut crate::game::Game, men: &[GuardId]) -> f64 {
    if g.seats.len() > 1 {
        crate::seats::deal_seats(g);
        let mut k = 0.0;
        for s in g.seats.clone() {
            let mut mine: Vec<GuardId> = Vec::new();
            for &m in men {
                if crate::seats::seat_has(g, s, m) { mine.push(m); }
            }
            let man = crate::cmd::seat_man(g, s);
            k += crate::seats::with_seat(g, man, &mut |g: &mut Game| fall_in_here(g, &mine));
        }
        return k;
    }
    fall_in_here(g, men)
}

/// The pass over one seat's squad: the part of teamStartsFallenIn that a seat's view runs. (startfile.ts:26-31)
fn fall_in_here(g: &mut Game, men: &[GuardId]) -> f64 {
    let up: Vec<GuardId> = men.iter().copied().filter(|&m| g.gd[m].state != crate::game::GuardState::Dead).collect();
    if up.is_empty() { return 0.0; }
    let def = match g.orders.defs.iter().copied().find(|&d| g.tab.orders_order_def[d].id == "fallIn") {
        Some(d) => d,
        None => return 0.0,
    };
    let p = g.ch[g.pl[g.player].ch].pos;
    crate::orders::squad_orders_new_group(g);
    let t = crate::orders::OrderTarget::None(crate::orders::OrderTargetNone { at: [p[0], 0.0, p[2]], man: None });
    if seam_order_def_issue(g, def, &up, &t).is_some() { return 0.0; }
    let mut took = 0.0;
    for &m in &up {
        if let Some(a) = crate::orders::squad_orders_of(g, m) {
            if g.tab.orders_assignment[a].order == "fallIn" { took += 1.0; }
        }
    }
    took
}

/// `def.issue(S, men, t)`: Some(reason) is the TypeScript's string return, a refusal.
fn seam_order_def_issue(g: &mut Game, def: OrdersOrderDefId, men: &[GuardId], t: &crate::orders::OrderTarget) -> Option<String> {
    todo!("needs: a public call of OrderDef.issue (OrderIssue's match and its FallIn arm have no name in orders/sig.rs)")
}
