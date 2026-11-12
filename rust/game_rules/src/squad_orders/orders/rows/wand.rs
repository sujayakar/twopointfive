//! LOOK UNDER IT: one man carrying the wand puts it under the door. The order's own code and its state are the module `wandorder`.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::Row;
use crate::orders::*;
use alloc::{string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    targets: &[TargetKind::Door],
    ack: &["on it.", "moving."],
    only: Some(only),
    valid: Some(valid),
    tick: Some(crate::wandorder::tick),
    ..Row::new("wandLook", label, issue)
};

fn label(_g: &mut Game, _t: &OrderTarget) -> String { String::from("look under it · wand") }

fn only(g: &mut Game, men: &[GuardId], t: &OrderTarget) -> Vec<GuardId> {
    let OrderTarget::Door(x) = t else { return Vec::new() };
    for side in [x.side, -x.side] {
        let there = OrderTarget::Door(OrderTargetDoor { side, ..*x });
        if let crate::wandorder::Pick::Arm1(p) = crate::wandorder::pick(g, men, &there) { return vec![p.gd]; }
    }
    Vec::new()
}

fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    match crate::wandorder::pick(g, men, t) {
        crate::wandorder::Pick::Text(w) => Err(Some(w)),
        crate::wandorder::Pick::Arm1(_) => Ok(()),
    }
}

fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, _: Option<&GiveOpts>) -> Option<String> {
    match crate::wandorder::issue(g, men, t) {
        crate::wandorder::Issue::Text(w) => Some(w),
        crate::wandorder::Issue::Arm1(()) => None,
    }
}
