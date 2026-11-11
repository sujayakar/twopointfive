//! FRAG AND CLEAR: a frag through the door, then in.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::{Is, Row};
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    is: &[Is::LiftsFire, Is::Loud],
    valid: Some(valid),
    ..Row::new("fragClear", label, issue)
};

fn label(g: &mut Game, t: &OrderTarget) -> String { format!("frag and clear · {} left", g.sets[g.seat].door_tools.frags) }

fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    if g.sets[g.seat].door_tools.frags <= 0.0 { return Err(Some(String::from("no frags left."))); }
    if !crate::mission::captives_up(g).is_empty() { return Err(Some(String::from("not with hostages on the floor."))); }
    let mate = match t { OrderTarget::Door(x) => crate::doortools::door_tools_frag_behind(g, x.door, x.side, men), _ => None };
    if let Some(mate) = mate { return Err(Some(crate::doortools::row_behind_words(mate))); }
    crate::doortools::row_stackable(g, t, men)
}

/// a bang and clear with a frag for the bang: `frag_next` is up for this one call
fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    g.sets[g.seat].door_tools.frag_next = true;
    let r = crate::doortools::door_tools_issue(g, crate::doortools::DoorToolKind::BangClear, men, t);
    g.sets[g.seat].door_tools.frag_next = false;
    match r { crate::doortools::DoorToolsIssue::Text(w) => Some(w), crate::doortools::DoorToolsIssue::Arm1(()) => None }
}
