//! GO: the men waiting on the word go. A squad of one or two waits stacked on its door; a bigger one wherever the word holds it.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::{Is, Row};
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    is: &[Is::LiftsFire],
    only: Some(only),
    valid: Some(valid),
    notes: Some(crate::doortools::door_tools_word_note),
    ..Row::new("go", label, issue)
};

/// the squad is one or two men (doortools deals the row its key by the same)
fn small(g: &Game) -> bool { crate::seats::here(g).door_tools.go_key == Some(Some(String::from("small"))) }

/// doortools.ts:908 / :916
fn label(g: &mut Game, t: &OrderTarget) -> String {
    let men = recipients_of(g, &crate::seats::here(g), t);
    let w = if small(g) { crate::doortools::door_tools_stack_words(g, &crate::seats::here(g), &men) } else { crate::doortools::door_tools_waiting_words(g, &crate::seats::here(g), &men) };
    if w.is_empty() { String::from("go") } else { format!("go · {}", w) }
}

fn only(g: &mut Game, men: &[GuardId], t: &OrderTarget) -> Vec<GuardId> {
    if !small(g) {
        let waits = crate::doortools::door_tools_waiting_men(g, men);
        if !waits.is_empty() { return waits; }
    }
    let mut waits: Vec<GuardId> = Vec::new();
    if small(g) { for x in crate::doortools::door_tools_stack_waits(g, &crate::seats::here(g), men) { waits.extend(x.men); } }
    let held = crate::doortools::door_tools_frag_held(g, men);
    men.iter().copied().filter(|m| waits.contains(m) || held.iter().any(|&o| g.tab.doortools_door_op[o].men.contains(m))).collect()
}

/// doortools.ts:910 / :918
fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    let waits = if small(g) { !crate::doortools::door_tools_stack_waits(g, &crate::seats::here(g), men).is_empty() } else { !crate::doortools::door_tools_waiting_any(g, &crate::seats::here(g), men).is_empty() };
    if waits || !crate::doortools::door_tools_frag_held(g, men).is_empty() { Ok(()) } else { Err(Some(String::from("nobody waiting."))) }
}

/// doortools.ts:910 / :918
fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    if !small(g) { crate::doortools::door_tools_go_men(g, men, None); return None; }
    let only: Vec<crate::doortools::DoorToolsGoMenOnlyItem> = crate::doortools::door_tools_stack_waits(g, &crate::seats::here(g), men).into_iter().map(|x| crate::doortools::DoorToolsGoMenOnlyItem { stack: x.stack, men: x.men }).collect();
    crate::doortools::door_tools_go_men(g, men, Some(&only))
}
