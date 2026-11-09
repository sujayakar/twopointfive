//! OPEN AND CLEAR: open it and go in, or from a held threshold go in with no bang.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::{Is, Row};
use crate::orders::*;
use crate::doortools::DoorToolsCanThresholdThenWhat as What;
use crate::doortools::DoorToolsEntryVerb as Verb;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    is: &[Is::LiftsFire, Is::Enters],
    only: Some(crate::doortools::row_held_or_all),
    valid: Some(valid),
    ..Row::new("openClear", label, issue)
};

/// doortools.ts:808 `held(t) ? 'go in and clear · no bang' : `${this.entryVerb(t)} and clear${known(t)}``
fn label(g: &mut Game, t: &OrderTarget) -> String {
    let men = recipients_of(g, &crate::seats::here(g), t);
    if crate::doortools::door_tools_held_plain(g, &crate::seats::here(g), t, &men) { return String::from("go in and clear · no bang"); }
    let verb = crate::doortools::door_tools_entry_verb(g, t);
    let known = match t { OrderTarget::Door(x) if crate::doortools::door_tools_entry_verb(g, t) != Verb::Open => format!(" · {}", crate::doortools::door_tools_shut_why(g, x.door)), _ => String::new() };
    format!("{} and clear{}", verb.as_str(), known)
}

fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    if crate::doortools::door_tools_held_plain(g, &crate::seats::here(g), t, men) { crate::doortools::row_can_then(g, What::Step, t, men) } else { crate::doortools::row_stackable(g, t, men) }
}

fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    let on_word = o.map_or(false, |o| on(o.on_word));
    crate::doortools::row_entry(g, false, men, t, on_word)
}
