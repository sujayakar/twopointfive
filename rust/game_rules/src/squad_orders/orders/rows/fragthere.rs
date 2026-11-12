//! FRAG THERE: a frag on that spot, thrown by one man. The throw's plan, its code and its state are the module `squadthrow` (plan.rs).
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::{Is, Row};
use crate::orders::*;
use crate::squadthrow::SquadThrowKind;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    is: &[Is::LiftsFire, Is::Loud],
    targets: &[TargetKind::Floor],
    page: Some(crate::squadthrow::THROW_PAGE),
    free: true,
    only: Some(only),
    valid: Some(valid),
    tick: Some(crate::squadthrow::walk_tick),
    ..Row::new("fragThere", label, issue)
};

fn label(g: &mut Game, t: &OrderTarget) -> String {
    crate::squadthrow::row_label(g, SquadThrowKind::Frag, t.at())
}

fn only(g: &mut Game, men: &[GuardId], t: &OrderTarget) -> Vec<GuardId> {
    crate::squadthrow::row_only(g, men, t.at(), SquadThrowKind::Frag)
}

fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    crate::squadthrow::row_valid(g, men, t.at(), SquadThrowKind::Frag).map_err(Some)
}

fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    crate::squadthrow::row_issue(g, men, t, SquadThrowKind::Frag)
}
