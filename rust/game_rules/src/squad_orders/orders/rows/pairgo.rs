//! THE MARK: the word that sends every man waiting on it. The job and its state are the module `pairorders`.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::{Is, Row};
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    is: &[Is::Pair],
    targets: &[TargetKind::Floor, TargetKind::Door, TargetKind::None, TargetKind::Foe],
    word: true,
    free: true,
    only: Some(only),
    valid: Some(valid),
    notes: Some(crate::pairorders::pair_orders_mark_note),
    ..Row::new("pairGo", label, issue)
};

fn label(g: &mut Game, t: &OrderTarget) -> String {
    let waiting = crate::pairorders::pair_orders_waiting(g, &crate::seats::here(g));
    if waiting.is_empty() { return String::from("mark"); }
    let mut parts: Vec<String> = Vec::with_capacity(waiting.len());
    for (gd, j) in &waiting {
        let foe = crate::pairorders::name_of(g, j.foe);
        parts.push(format!("{} takes {}{}", g.gd[*gd].callsign, foe, if j.watcher.is_some() { " anyway" } else { "" }));
    }
    format!("mark · {}", parts.join(" · "))
}

fn only(g: &mut Game, men: &[GuardId], t: &OrderTarget) -> Vec<GuardId> {
    crate::pairorders::pair_orders_waiting(g, &crate::seats::here(g)).into_iter().map(|(gd, _)| gd).collect()
}

fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    if !crate::pairorders::pair_orders_waiting(g, &crate::seats::here(g)).is_empty() { return Ok(()); }
    Err(Some(String::from("nobody's waiting on your mark.")))
}

fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    crate::pairorders::pair_orders_mark(g, Some(crate::pairorders::PairOrdersMarkBy::Row));
    None
}
