//! GO, for a squad of one or two: the men waiting on the word go.
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

/// doortools.ts:908 / :916
fn label(g: &mut Game, t: &OrderTarget) -> String {
    let men = recipients_of(g, &crate::seats::here(g), t);
    let w = crate::doortools::door_tools_stack_words(g, &men);
    if w.is_empty() { String::from("go") } else { format!("go · {}", w) }
}

fn only(g: &mut Game, men: &[GuardId], t: &OrderTarget) -> Vec<GuardId> {
    let mut waits: Vec<GuardId> = Vec::new();
    for x in crate::doortools::door_tools_stack_waits(g, men) { waits.extend(x.men); }
    let held = crate::doortools::door_tools_frag_held(g, men);
    men.iter().copied().filter(|m| waits.contains(m) || held.iter().any(|&o| g.tab.doortools_door_op[o].men.contains(m))).collect()
}

/// doortools.ts:910 / :918
fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    if !crate::doortools::door_tools_stack_waits(g, men).is_empty() || !crate::doortools::door_tools_frag_held(g, men).is_empty() { Ok(()) } else { Err(Some(String::from("nobody waiting."))) }
}

/// doortools.ts:910 `this.goMen(men, this.stackWaits(men)) ?? undefined`
fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    let only: Vec<crate::doortools::DoorToolsGoMenOnlyItem> = crate::doortools::door_tools_stack_waits(g, men).into_iter().map(|x| crate::doortools::DoorToolsGoMenOnlyItem { stack: x.stack, men: x.men }).collect();
    crate::doortools::door_tools_go_men(g, men, Some(&only))
}
