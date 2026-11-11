//! THE WORD ON A HELD THRESHOLD: go.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::{Is, Row};
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    is: &[Is::LiftsFire],
    only: Some(only),
    valid: Some(valid),
    ..Row::new("thrWord", label, issue)
};

fn label(g: &mut Game, t: &OrderTarget) -> String {
    if !squad_orders_big(g, &crate::seats::here(g)) {
        return String::from(if crate::silent::silent_no_cans(g, &crate::seats::here(g)) || crate::doortools::door_tools_word_quiet(g, &crate::seats::here(g)) { "go · step in · no bang" } else { "go · bang the blind pockets, step in" });
    }
    let men = recipients_of(g, &crate::seats::here(g), t); let w = crate::doortools::door_tools_waiting_words(g, &crate::seats::here(g), &men);
    if w.is_empty() { String::from("go") } else { format!("go · {}", w) }
}

fn only(g: &mut Game, men: &[GuardId], t: &OrderTarget) -> Vec<GuardId> {
    if squad_orders_big(g, &crate::seats::here(g)) { crate::doortools::door_tools_waiting_men(g, men) } else { crate::doortools::door_tools_held_men(g, &crate::seats::here(g), men, t) }
}

fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    let ops = crate::doortools::door_tools_held_on(g, &crate::seats::here(g), t, men);
    if ops.iter().any(|&o| g.tab.doortools_door_op[o].word.is_none()) { return Ok(()); }
    Err(Some(String::from("nobody's holding that door.")))
}

fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    if squad_orders_big(g, &crate::seats::here(g)) { crate::doortools::door_tools_go_men(g, men, None) } else if crate::doortools::door_tools_pair_word(g, men) { None } else { Some(String::from("nobody's waiting for you to say go.")) }
}
