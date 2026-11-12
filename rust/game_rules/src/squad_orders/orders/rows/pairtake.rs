//! TAKE HIM NOW: the nearest man who can reach his back takes him. The job and its state are the module `pairorders`.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::{Is, Row};
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    is: &[Is::Pair, Is::Take],
    targets: &[TargetKind::Foe],
    ack: &["on him.", "moving up on him."],
    can: Some(crate::pairorders::row_take_can),
    only: Some(crate::pairorders::row_take_only),
    valid: Some(crate::pairorders::row_take_valid),
    tick: Some(crate::pairorders::pair_orders_take_tick),
    ..Row::new("pairTake", label, issue)
};

fn label(_g: &mut Game, _t: &OrderTarget) -> String { String::from("take him now") }

fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    crate::pairorders::take_issue(g, crate::pairorders::JobMode::Now, men, t)
}
