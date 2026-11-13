//! THE 40 MM SHELL: "put a shell there". One man of those addressed has a launcher up, a round and a shot; he turns, aims and fires.
//! The order's own code and its state are the module `squad40`; this file is everything that makes it an order.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::{Is, Row};
use crate::orders::{GiveOpts, OrderTarget, TargetKind};
use crate::squad40;
use alloc::{format, string::String, vec::Vec};

pub const ORDER: Row = Row {
    is: &[Is::LiftsFire, Is::Loud],
    targets: &[TargetKind::Floor],
    page: Some(crate::squadthrow::THROW_PAGE),
    free: true,
    only: Some(only),
    valid: Some(valid),
    tick: Some(squad40::turn_tick),
    frame: Some(squad40::squad40_tick),
    notes: Some(squad40::notes),
    ..Row::new("shellThere", label, issue)
};

fn label(g: &mut Game, _t: &OrderTarget) -> String { format!("40 mm there · {} left", squad40::squad40_left(g, &crate::seats::here(g))) }

/// the one man who would fire, or nobody
fn only(g: &mut Game, men: &[GuardId], t: &OrderTarget) -> Vec<GuardId> { squad40::plan(g, men, t.at()).shot.map(|a| a.gd).into_iter().collect() }

fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    match squad40::plan(g, men, t.at()).no { Some(why) => Err(Some(why)), None => Ok(()) }
}

fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, _o: Option<&GiveOpts>) -> Option<String> { squad40::row_issue(g, men, t) }
