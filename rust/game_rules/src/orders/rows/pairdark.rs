//! MOVE THERE, HOLD IN THE DARK: it stands in move's place while the squad is silent. The job and its state are the module `pairorders`.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::Row;
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    can: Some(can),
    tick: Some(crate::pairorders::pair_orders_dark_tick),
    when: Some(when),
    ..Row::new("pairDark", label, issue)
};

fn label(_g: &mut Game, _t: &OrderTarget) -> String { String::from("move there · hold in the dark") }

fn can(g: &mut Game, gd: GuardId, t: &OrderTarget) -> Option<String> {
    let at = rows::at(t);
    let dark = crate::pairorders::pair_orders_dark_spot(g, at);
    let w = squad_orders_reach(g, gd, dark.unwrap_or(at));
    if w.is_some() { return w; }
    if dark.is_some() { None } else { Some(String::from("no dark spot there.")) }
}

fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    crate::pairorders::p_01::dark_issue(g, men, t)
}

/// in move's place while the squad is silent and the pair orders are up
fn when(g: &mut Game, _t: &OrderTarget, _men: &[GuardId]) -> bool { crate::pairorders::pair_orders_up(g, &crate::seats::here(g)) && crate::silent::squad_silent(g, &crate::seats::here(g)) }
