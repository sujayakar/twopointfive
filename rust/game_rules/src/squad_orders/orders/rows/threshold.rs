//! CLEAR FROM OUTSIDE: the men take the jambs and clear what they can see from there; the next word is the player's.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::{Is, Row};
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    is: &[Is::LiftsFire],
    valid: Some(valid),
    ..Row::new("thresholdHold", label, issue)
};

fn label(g: &mut Game, t: &OrderTarget) -> String {
    let men = recipients_of(g, &crate::seats::here(g), t);
    String::from(if crate::doortools::door_tools_held_plain(g, &crate::seats::here(g), t, &men) { "clear from outside · holding it" } else { "clear from outside" })
}

fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    if crate::doortools::door_tools_held_plain(g, &crate::seats::here(g), t, men) { return Err(Some(String::from("already holding it."))); }
    match crate::doortools::door_tools_can_threshold(g, t, men) {
        crate::doortools::DoorToolsCanThreshold::Text(w) => Err(Some(w)),
        crate::doortools::DoorToolsCanThreshold::Bool(true) => Ok(()),
        crate::doortools::DoorToolsCanThreshold::Bool(false) => Err(None),
    }
}

fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    match crate::doortools::door_tools_threshold_hold(g, men, t) { crate::doortools::DoorToolsThresholdHold::Text(w) => Some(w), crate::doortools::DoorToolsThresholdHold::Arm1(()) => None }
}
