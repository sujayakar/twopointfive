//! THE MARK: the word that sends every man waiting on it. The job and its state are the module `pairorders`.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::Row;
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    only: Some(only),
    valid: Some(valid),
    ..Row::new("pairGo", label, issue)
};

fn label(g: &mut Game, t: &OrderTarget) -> String {
    let waiting = crate::pairorders::pair_orders_waiting(g);
    if waiting.is_empty() { return String::from("mark"); }
    let mut parts: Vec<String> = Vec::with_capacity(waiting.len());
    for (gd, j) in &waiting {
        let foe = crate::pairorders::name_of(g, j.foe);
        parts.push(format!("{} takes {}{}", g.gd[*gd].callsign, foe, if j.watcher.is_some() { " anyway" } else { "" }));
    }
    format!("mark · {}", parts.join(" · "))
}

fn only(g: &mut Game, men: &[GuardId], t: &OrderTarget) -> Vec<GuardId> {
    crate::pairorders::pair_orders_waiting(g).into_iter().map(|(gd, _)| gd).collect()
}

fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    if !crate::pairorders::pair_orders_waiting(g).is_empty() { return Ok(()); }
    let (now, was) = (String::from("nobody's waiting on your mark."), String::from("nobody is waiting on your mark."));
    Err(Some(crate::words::wg(g, &now, &was)))
}

fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    crate::pairorders::pair_orders_mark(g, Some(crate::pairorders::PairOrdersMarkBy::Row));
    None
}
