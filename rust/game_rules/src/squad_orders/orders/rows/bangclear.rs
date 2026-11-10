//! BANG AND CLEAR: a bang through the door, then in; from a held threshold, bang and go in.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::{Is, Row};
use crate::orders::*;
use crate::doortools::ThresholdThen as What;
use crate::doortools::DoorToolsEntryVerb as Verb;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    is: &[Is::LiftsFire, Is::Loud, Is::Enters],
    only: Some(crate::doortools::row_held_or_all),
    valid: Some(valid),
    ..Row::new("bangClear", label, issue)
};

fn label(g: &mut Game, t: &OrderTarget) -> String {
    let men = recipients_of(g, &crate::seats::here(g), t);
    if crate::doortools::door_tools_held_plain(g, &crate::seats::here(g), t, &men) { return format!("bang, then go in · {} left", g.door_tools.bangs); }
    let first = if crate::doortools::door_tools_entry_verb(g, t) == Verb::Open { String::from("bang") } else { format!("{}, bang", crate::doortools::door_tools_entry_verb(g, t).as_str()) };
    format!("{} and clear · {} left", first, g.door_tools.bangs)
}

fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    if crate::doortools::door_tools_held_plain(g, &crate::seats::here(g), t, men) { return crate::doortools::row_can_then(g, What::Bang, t, men); }
    if g.door_tools.bangs <= 0.0 { return Err(Some(String::from("no bangs left."))); }
    let who = match t { OrderTarget::Door(x) => crate::doortools::door_tools_can_behind(g, x.door, x.side, men), _ => None };
    if let Some(who) = who { return Err(Some(crate::doortools::row_behind_words(g, who))); }
    crate::doortools::row_stackable(g, t, men)
}

fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    let on_word = o.map_or(false, |o| on(o.on_word));
    crate::doortools::row_entry(g, true, men, t, on_word)
}
