//! THE CHARGE ON A WALL: a charge on the wall that is the target; the charge already on it is blown.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::{Is, Row};
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    is: &[Is::LiftsFire, Is::Blows, Is::Loud],
    targets: &[TargetKind::Floor],
    listed: Some(&[]),
    ack: &["copy.", "on it."],
    can: Some(can),
    valid: Some(valid),
    gone: Some(gone),
    notes: Some(crate::wallcharge::wall_charge_hud_notes),
    ..Row::new("wallCharge", label, issue)
};

fn label(g: &mut Game, t: &OrderTarget) -> String {
    match crate::wallcharge::wall_charge_charge_on_target(g, &crate::seats::here(g), t) {
        Some(c) => format!("blow the charge{}", crate::wallcharge::wall_charge_close_words(g, Some(c), None)),
        None => crate::wallcharge::wall_charge_row_words(g, &crate::seats::here(g)),
    }
}

fn can(g: &mut Game, gd: GuardId, t: &OrderTarget) -> Option<String> {
    match crate::wallcharge::wall_charge_can_of(g, gd, t) {
        crate::wallcharge::WallChargeCanOf::Text(w) => Some(w),
        crate::wallcharge::WallChargeCanOf::Bool(_) => None,
    }
}

fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    match crate::wallcharge::wall_charge_valid(g, t, men) {
        crate::wallcharge::WallChargeValid::Text(w) => Err(Some(w)),
        crate::wallcharge::WallChargeValid::Bool(true) => Ok(()),
        crate::wallcharge::WallChargeValid::Bool(false) => Err(None),
    }
}

fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    match crate::wallcharge::wall_charge_issue(g, men, t) {
        crate::wallcharge::WallChargeIssue::Text(w) => Some(w),
        crate::wallcharge::WallChargeIssue::Arm1(()) => None,
    }
}

/// none were brought, and none is on it
fn gone(g: &Game, seat: &crate::seats::SeatRef, t: &OrderTarget, _men: &[GuardId]) -> bool {
    seat.wall_charge.brought == Some(0.0) && crate::wallcharge::wall_charge_charge_on_target(g, seat, t).is_none()
}
