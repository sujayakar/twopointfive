use crate::game::Game;
use crate::ids::*;
use alloc::vec::Vec;

/// Every living one of `men` is handed the menu's own 'fall in' as a scenario stages, unspoken; returns how many took it.
pub(super) fn team_starts_fallen_in(g: &mut crate::game::Game, men: &[GuardId]) -> f64 {
    if g.seats.len() > 1 {
        crate::seats::deal_seats(g);
        let mut k = 0.0;
        for s in g.seats.clone() {
            let mut mine: Vec<GuardId> = Vec::new();
            for &m in men {
                if crate::seats::in_seat(g, s, m) { mine.push(m); }
            }
            k += crate::seats::with_seat(g, s, &mut |g: &mut Game| fall_in_here(g, &mine));
        }
        return k;
    }
    fall_in_here(g, men)
}

/// The pass over one seat's squad: the part of teamStartsFallenIn that a seat's view runs.
fn fall_in_here(g: &mut Game, men: &[GuardId]) -> f64 {
    let up: Vec<GuardId> = men.iter().copied().filter(|&m| !crate::condition::down(g, g.gd[m].ch)).collect();
    if up.is_empty() { return 0.0; }
    let def = match g.groups[g.group].orders.defs.iter().copied().find(|&d| g.tab.orders_order_def[d].id == crate::orders::rows::fallin::ORDER.id) {
        Some(d) => d,
        None => return 0.0,
    };
    let p = g.player_pos(g.player);
    crate::orders::squad_orders_new_group(g);
    let t = crate::orders::OrderTarget::None(crate::orders::OrderTargetNone { at: [p[0], 0.0, p[2]], man: None });
    if crate::orders::order_issue(g, def, &up, &t, None).is_some() { return 0.0; }
    let mut took = 0.0;
    for &m in &up {
        if let Some(a) = crate::orders::squad_orders_of(&crate::seats::here(g), m) {
            if crate::orders::rows::is(g, a, crate::orders::Is::Follows) { took += 1.0; }
        }
    }
    took
}
