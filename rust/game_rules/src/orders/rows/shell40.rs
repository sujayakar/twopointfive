//! THE 40 MM SHELL: "put a shell there". One man of those addressed has a launcher up, a round and a shot; he turns, aims and fires.
//! The order's own code and its state are the module `squad40`; this file is everything that makes it an order.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::Row;
use crate::orders::{GiveOpts, OrderTarget};
use crate::squad40;
use crate::v3::V3;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    only: Some(only),
    valid: Some(valid),
    tick: Some(squad40::turn_tick),
    frame: Some(squad40::squad40_tick),
    ..Row::new(squad40::SHELL_THERE, label, issue)
};

fn at_of(t: &OrderTarget) -> V3 { crate::orders::rows::at(t) }

fn label(g: &mut Game, _t: &OrderTarget) -> String { format!("40 mm there · {} left", squad40::squad40_left(g, &crate::seats::here(g))) }

/// the one man who would fire, or nobody
fn only(g: &mut Game, men: &[GuardId], t: &OrderTarget) -> Vec<GuardId> { squad40::plan(g, men, at_of(t)).shot.map(|a| a.gd).into_iter().collect() }

fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    match squad40::plan(g, men, at_of(t)).no { Some(why) => Err(Some(why)), None => Ok(()) }
}

fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, _o: Option<&GiveOpts>) -> Option<String> { squad40::row_issue(g, men, t) }
