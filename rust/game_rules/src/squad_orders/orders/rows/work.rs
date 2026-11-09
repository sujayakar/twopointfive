//! WORK IT: a man goes to the thing and works it. The order's own code and its state are the module `workthings`.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::Row;
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    can: Some(can),
    only: Some(only),
    valid: Some(valid),
    tick: Some(crate::workthings::work_things_tick),
    ..Row::new("work", label, issue)
};

fn label(g: &mut Game, t: &OrderTarget) -> String {
    match t {
        OrderTarget::Floor(OrderTargetFloor { thing: Some(th), .. }) => crate::workthings::work_things_words(g, th),
        _ => String::from("work it"),
    }
}

fn can(g: &mut Game, gd: GuardId, t: &OrderTarget) -> Option<String> {
    let at = rows::at(t);
    squad_orders_reach(g, gd, at)
}

fn only(g: &mut Game, men: &[GuardId], t: &OrderTarget) -> Vec<GuardId> {
    crate::workthings::work_things_pick(g, men, t)
}

fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    let OrderTarget::Floor(OrderTargetFloor { thing: Some(th), .. }) = t else { return Err(Some(String::from("nothing to work there."))) };
    if th.stand.is_none() { return Err(Some(String::from("can't get a hand on it."))); }
    let verb = crate::workthings::work_things_verb_of(g, th);
    rows::plain_valid(crate::workthings::work_things_moot(g, th, verb))
}

fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    match crate::workthings::work_things_issue(g, men, t) {
        crate::workthings::WorkThingsIssue::Text(w) => Some(w),
        crate::workthings::WorkThingsIssue::Arm1(()) => None,
    }
}
