//! GO, for a squad of more than two: every man waiting on the word goes.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::Row;
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    only: Some(only),
    valid: Some(valid),
    ..Row::new("go", label, issue)
};

fn label(g: &mut Game, t: &OrderTarget) -> String {
    let men = recipients_of(g, &crate::seats::here(g), t);
    let w = crate::doortools::door_tools_waiting_words(g, &men);
    if w.is_empty() { String::from("go") } else { format!("go · {}", w) }
}

fn only(g: &mut Game, men: &[GuardId], t: &OrderTarget) -> Vec<GuardId> {
    let waits = crate::doortools::door_tools_waiting_men(g, men);
    if !waits.is_empty() { return waits; }
    let held = crate::doortools::door_tools_frag_held(g, men);
    men.iter().copied().filter(|m| held.iter().any(|&o| g.tab.doortools_door_op[o].men.contains(m))).collect()
}

fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    if !crate::doortools::door_tools_waiting_any(g, men).is_empty() || !crate::doortools::door_tools_frag_held(g, men).is_empty() { Ok(()) } else { Err(Some(String::from("nobody waiting."))) }
}

/// doortools.ts:918 `{ this.goMen(men); }`
fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    crate::doortools::door_tools_go_men(g, men, None);
    None
}
