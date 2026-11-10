//! WATCH HIM: each man who sees him keeps his gun on him. The job and its state are the module `pairorders`.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::{Is, Row};
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    is: &[Is::Pair],
    can: Some(can),
    valid: Some(valid),
    tick: Some(tick),
    ..Row::new("pairWatch", label, issue)
};

fn label(_g: &mut Game, _t: &OrderTarget) -> String { String::from("watch him") }

fn can(g: &mut Game, gd: GuardId, t: &OrderTarget) -> Option<String> {
    match t {
        OrderTarget::Foe(x) if crate::pairorders::pair_orders_sees(g, gd, x.foe) => None,
        _ => Some(String::from("no line from here.")),
    }
}

fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    let OrderTarget::Foe(x) = t else { return Err(None) };
    rows::plain_valid(crate::pairorders::pair_orders_gone(g, x.foe))
}

fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    crate::pairorders::watch_issue(g, men, t)
}

/// (pairorders.ts:232 the 'watch him' row's `tick`: his gun stays on the man he was told to watch, while he sees him)
fn tick(g: &mut Game, gd: GuardId, a: OrdersAssignmentId) {
    let h = match g.tab.orders_assignment[a].foe { Some(h) => h, None => return };
    if crate::condition::down(g, g.gd[h].ch) || !g.guards.contains(&h) { crate::pairorders::pair_orders_stand(g, gd, "holding", None); return; }
    if !crate::pairorders::pair_orders_sees(g, gd, h) { return; }   // (behind something: the gun stays on where he last saw him)
    let (c, p) = (g.pos_of(h), g.pos_of(gd));
    let aa = &mut g.tab.orders_assignment[a];
    if let Some(w) = &mut aa.watch { w[0] = c[0]; w[2] = c[2]; }
    aa.face = Some(crate::v3::yaw_to(p, c));
}
