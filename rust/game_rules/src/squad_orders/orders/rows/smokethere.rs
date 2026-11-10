//! SMOKE THERE: the nearest man who has the throw puts a can there. The order's own code and its state are the module `squadsmoke`.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::Row;
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    only: Some(only),
    valid: Some(valid),
    notes: Some(crate::squadsmoke::notes),
    ..Row::new("smokeThere", label, crate::squadsmoke::row_smoke_issue)
};

fn label(g: &mut Game, t: &OrderTarget) -> String {
    format!("smoke there · {} left", crate::squadsmoke::squad_smoke_left(g, &crate::seats::here(g)))
}

fn only(g: &mut Game, men: &[GuardId], t: &OrderTarget) -> Vec<GuardId> {
    let at = t.at();
    match crate::squadsmoke::squad_smoke_thrower(g, men, at) { Some(th) => vec![th.gd], None => Vec::new() }
}

fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    let at = t.at();
    if crate::squadsmoke::squad_smoke_left(g, &crate::seats::here(g)) <= 0.0 { return Err(Some(String::from("no smoke left."))); }
    if crate::squadsmoke::squad_smoke_thrower(g, men, at).is_some() { Ok(()) } else { Err(Some(String::from("no throw from here."))) }
}
