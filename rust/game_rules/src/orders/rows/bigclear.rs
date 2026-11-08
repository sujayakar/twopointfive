//! THE BIG SLOT'S BREACH: the hammer or the breaching shotgun on the door, then in.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::Row;
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    valid: Some(valid),
    gone: Some(gone),
    ..Row::new(crate::doortools::BIG_ORDER, label, issue)
};

/// doortools.ts:929
fn label(g: &mut Game, t: &OrderTarget) -> String {
    let men = recipients_of(g, &crate::seats::here(g), t);
    format!("{} and clear", crate::doortools::door_tools_big_word(g, t, &men).as_str())
}

/// doortools.ts:931-936 (SHOTGUN.on, SILENT.toolsLoud and DOORTOOLS.testDoor are on on the page: the silent refusal is not on its path)
fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    let OrderTarget::Door(x) = t else { return Err(None) };
    if crate::doortools::is_open(g, x.door) { return Err(Some(String::from("it's open."))); }
    if on(g.dr[x.door].def.steel) { return Err(Some(String::from("steel. no way through."))); }
    let mut any = false;
    for &m in men { if crate::bigkit::carries_hammer(g, m) || (g.gd[m].state != crate::game::GuardState::Dead && crate::kit::kit_big(g, m) == crate::kit::BigKit::Shotgun) { any = true; break; } }
    if !any { return Err(Some(String::from("nobody here carries one."))); }
    if crate::doors::door_locked(g, x.door) && g.dr[x.door].lock_id.starts_with("wedge:") {
        let mut hammer = false;
        for &m in men { if crate::bigkit::carries_hammer(g, m) { hammer = true; break; } }
        if !hammer && crate::doortools::door_tools_known_shut(g, x.door, x.side) {
            let (now, was) = (String::from("it's wedged. a shell won't open it."), String::from("wedged. no shell shifts it."));
            return Err(Some(crate::words::wg(g, &now, &was)));
        }
    }
    crate::doortools::row_stackable(g, t, men)
}

/// doortools.ts:937
fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    let on_word = o.map_or(false, |o| on(o.on_word));
    g.door_tools.big_next = true;
    let r = crate::doortools::door_tools_issue(g, crate::doortools::DoorToolKind::OpenClear, men, t, Some(on_word));
    g.door_tools.big_next = false;
    match r { crate::doortools::DoorToolsIssue::Text(w) => Some(w), crate::doortools::DoorToolsIssue::Arm1(()) => None }
}

/// an open door, or a steel one
fn gone(g: &Game, seat: &crate::seats::SeatRef, t: &OrderTarget, _men: &[GuardId]) -> bool {
    let OrderTarget::Door(x) = t else { return false };
    crate::doortools::is_open(g, x.door) || on(g.dr[x.door].def.steel)
}
