//! MOVE THERE: "move there". A place and a watch each that fit the room, or the line abreast.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::Row;
use crate::orders::*;
use crate::v3::{self, V3};
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    can: Some(can),
    valid: Some(valid),
    tick: Some(crate::roomfit::fit_tick),
    ..Row::new("move", label, issue)
};

fn label(_g: &mut Game, _t: &OrderTarget) -> String { String::from("move there") }

/// 'move'.can (orders.ts:581)
fn can(g: &mut Game, gd: GuardId, t: &OrderTarget) -> Option<String> {
    let at = rows::at(t);
    if squad_orders_walk_try(g, gd, at).is_some() { return None; }
    squad_orders_reach(g, gd, at)
}
/// 'move'.valid: a door nobody has tried is walked up to, where it has room for them (orders.ts:582)
fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    match squad_orders_walk_stacks(g, men, rows::at(t)) {
        Some(SquadOrdersWalkStacks::Text(w)) => Err(Some(w)),
        _ => Ok(()),
    }
}
/// 'move'.issue: a place and a watch each that fit the room, or the line abreast (orders.ts:583)
fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    let t_at = rows::at(t);
    let pc = g.pl[g.player].ch;
    let sam = g.ch[pc].pos;
    let face = if v3::dist_xz(sam, t_at) > 0.3 { v3::yaw_to(sam, t_at) } else { g.ch[pc].body_yaw };
    // (the room kept: Sam's, and every other man's mark: where he is walking to, not where he is)
    let rest: Vec<GuardId> = squad_orders_men(g, &crate::seats::here(g), None, None).into_iter().filter(|m| !men.contains(m)).collect();
    let mut bodies: Vec<V3> = vec![sam];
    let mut held: Vec<String> = Vec::new();
    for &m in &rest {
        let of = squad_orders_of(g, &crate::seats::here(g), m);
        let post = of.and_then(|a| g.tab.orders_assignment[a].post);
        bodies.push(match post { Some(p) => p, None => g.ch[g.gd[m].ch].pos });
    }
    for &m in &rest {
        let of = squad_orders_of(g, &crate::seats::here(g), m);
        let key = of.and_then(|a| g.tab.orders_assignment[a].fit.as_ref().and_then(|f| f.key.clone()));
        if let Some(k) = key { if !k.is_empty() { held.push(k); } }
    }
    let fits = crate::roomfit::fit_spots(g, t_at, men.len() as f64, face, sam, Some(&bodies), Some(&held));
    let deal: Vec<(GuardId, V3)> = match &fits {
        Some(f) => {
            let spots: Vec<V3> = f.iter().map(|x| x.pos).collect();
            let groups = crate::roomfit::fit_groups(f);
            squad_orders_pair(g, men, &spots, Some(&groups), None)
        }
        None => {
            let spots = squad_orders_spots_at(g, t_at, face, men.len() as f64);
            let groups: Vec<f64> = (0..spots.len()).map(|i| (i >> 1) as f64).collect();
            squad_orders_pair(g, men, &spots, Some(&groups), None)
        }
    };
    let at = match squad_orders_walk_stacks(g, men, t_at) {
        Some(SquadOrdersWalkStacks::Text(w)) => return Some(w),
        Some(SquadOrdersWalkStacks::Map(m)) => Some(m),
        None => None,
    };
    for (gd, p) in deal {
        let fit = fits.as_ref().and_then(|f| f.iter().find(|x| v3::dist_xz(x.pos, p) < 1e-6).cloned());
        let walk = at.as_ref().and_then(|m| m.get(gd).copied());
        let on = SquadOrdersAssignA {
            order: String::from("move"), post: Some(p), face: Some(face),
            stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim),
            going: String::from("moving"), there: String::from("in position"), set_line: Some(String::from("set.")), fit,
            ..Default::default()
        };
        if let Some(w) = walk {
            squad_orders_walk_send(g, gd, SquadOrdersWalkSendT { door: w.door, side: w.side, post: w.post }, t_at, &on);
        } else {
            squad_orders_assign(g, gd, &on);
        }
    }
    None
}
