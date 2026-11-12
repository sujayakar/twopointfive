//! THE BIG SLOT'S BREACH: the hammer or the breaching shotgun on the door, then in.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::{Is, Row};
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    is: &[Is::LiftsFire, Is::Loud],
    targets: &[TargetKind::Door],
    ack: &["on it.", "breaching."],
    page: Some(crate::doortools::TOOLS_PAGE),
    valid: Some(valid),
    gone: Some(gone),
    ..Row::new("bigClear", label, issue)
};

fn label(g: &mut Game, t: &OrderTarget) -> String {
    let men = recipients_of(g, &crate::seats::here(g), t);
    format!("{} and clear", crate::doortools::door_tools_big_word(g, t, &men).as_str())
}

fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    let OrderTarget::Door(x) = t else { return Err(None) };
    if crate::doortools::is_open(g, x.door) { return Err(Some(String::from("it's open."))); }
    if on(g.dr[x.door].def.steel) { return Err(Some(String::from("steel. no way through."))); }
    let mut any = false;
    for &m in men { if crate::bigkit::carries_hammer(g, m) || (!crate::condition::down(g, g.gd[m].ch) && crate::kit::kit_big(g, m) == crate::kit::BigKit::Shotgun) { any = true; break; } }
    if !any { return Err(Some(String::from("nobody here carries one."))); }
    if g.dr[x.door].leaf.locked && g.dr[x.door].lock_id.starts_with("wedge:") {
        let mut hammer = false;
        for &m in men { if crate::bigkit::carries_hammer(g, m) { hammer = true; break; } }
        if !hammer && crate::doortools::door_tools_known_shut(g, x.door, x.side) {
            return Err(Some(String::from("it's wedged. a shell won't open it.")));
        }
    }
    crate::doortools::row_stackable(g, t, men)
}

fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    g.groups[g.group].tools.big_next = true;
    let r = crate::doortools::door_tools_issue(g, crate::doortools::DoorToolKind::OpenClear, men, t);
    g.groups[g.group].tools.big_next = false;
    match r { crate::doortools::DoorToolsIssue::Text(w) => Some(w), crate::doortools::DoorToolsIssue::Arm1(()) => None }
}

/// an open door, or a steel one
fn gone(g: &Game, seat: &crate::seats::SeatRef, t: &OrderTarget, _men: &[GuardId]) -> bool {
    let OrderTarget::Door(x) = t else { return false };
    crate::doortools::is_open(g, x.door) || on(g.dr[x.door].def.steel)
}
