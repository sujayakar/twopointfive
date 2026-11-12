//! SHUT THAT DOOR: one man walks to it and shuts it. It stands in the charge's place while the door stands open.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::Row;
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    targets: &[TargetKind::Door],
    ack: &["shutting it.", "got the door."],
    stands_in: Some(rows::charge::ORDER.id),
    only: Some(only),
    valid: Some(valid),
    tick: Some(crate::shutdoor::tick),
    when: Some(when),
    ..Row::new("shutDoor", label, issue)
};

fn label(_g: &mut Game, _t: &OrderTarget) -> String { String::from("shut that door") }

fn only(g: &mut Game, men: &[GuardId], t: &OrderTarget) -> Vec<GuardId> {
    let OrderTarget::Door(x) = t else { return men.to_vec() };
    if g.dr[x.door].leaf.off || !crate::shutdoor::stands_open(g, x.door) { return men.to_vec(); }
    match crate::shutdoor::plan(g, x.door, men) {
        crate::shutdoor::Plan::Text(_) => men.to_vec(),
        crate::shutdoor::Plan::Arm1(p) => vec![p.gd],
    }
}

fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    let OrderTarget::Door(x) = t else { return Err(None) };
    rows::plain_valid(crate::shutdoor::why_not_shut(g, x.door, men))
}

fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    crate::shutdoor::order_issue(g, men, t)
}

/// in the charge's place while the door stands open with no charge on it
fn when(g: &Game, seat: &crate::seats::SeatRef, t: &OrderTarget, _men: &[GuardId]) -> bool {
    let OrderTarget::Door(x) = t else { return false };
    crate::shutdoor::stands_open(g, x.door) && crate::doortools::door_tools_charge_on(g, seat, x.door).is_none()
}
