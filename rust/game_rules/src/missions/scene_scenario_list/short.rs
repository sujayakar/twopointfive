//! A MISSION AND ITS ADDRESS: the query string a row of the list loads and the one a visitor's address leaves behind, and THE SHORT LIST, the few
//! missions worth playing first, in play order, each with a link of its own that reads back to its row. An address is words and stays in the visitor's own spelling
//! wherever a word of his is carried: nothing here reads the game.
use super::*;
use crate::difficulty::Difficulty;
use crate::query::{after_mark, decoded, encoded, Query};
use crate::scene_scenarios::{ScenarioEntry, ScenarioEntryKind};
use alloc::{format, string::String, vec::Vec};

/// the solo night's word on an address, and its place on the list
pub const SOLO_KEY: &str = "solo";
pub const SOLO_ID: &str = "solo-night";
/// the words a scenario owns: a pick replaces them, a quit drops them, everything else on the address rides along
pub const SCENARIO_KEYS: [&str; 9] = ["levelFile", "nightbreach", "roomclear", "rescue", "quiet", "squad4", "squad", "loadout", "solo"];

/// a run's own words (the benchmark's, the pricing run's), and a priced page's two for a held picture: one run on the page it was asked on, never carried onto a mission's link
const PRICE_RUN_WORD: &str = "pricerun";
const RUN_PAGE_WORDS: [&str; 2] = ["benchrun", PRICE_RUN_WORD];
const PRICE_ASK_WORDS: [&str; 4] = ["pricelevels", "pricesecs", "pricelive", "pricepage"];
const HELD_PAGE_WORDS: [&str; 2] = ["adapt", "giveway"];

/// The query string that is a row of the list.
pub fn scenario_query(e: &ScenarioEntry) -> String {
    let mut q: Vec<String> = Vec::new();
    if let Some(level) = e.level.as_deref().filter(|l| !l.is_empty()) { q.push(format!("levelFile={}", encoded(level))); }
    let kind = e.kind.as_str();
    q.push(match e.seed {
        Some(seed) if matches!(e.kind, ScenarioEntryKind::Roomclear | ScenarioEntryKind::Quiet) => format!("{}={}", kind, crate::js::num_str(seed)),
        _ => String::from(kind),
    });
    if e.kind == ScenarioEntryKind::Quiet {
        if e.squad == 1.0 { q.push(String::from("squad=1")); }
    } else if e.squad == 4.0 {
        q.push(String::from(if e.kind == ScenarioEntryKind::Rescue { "squad=4" } else { "squad4" }));
    }
    format!("?{}", q.join("&"))
}

/// The visitor's address with every word that started something dropped; the rest as `bare` writes it.
pub(in crate::missions) fn quit_search(search: &str) -> String {
    let mut q = Query::parse(search);
    for k in SCENARIO_KEYS { q.delete(k); }
    q.delete("way");
    // (`quietfog=0` was the rule's own off switch: the visitor's flag, not a rung)
    if q.get("quietfog") != Some("0") { q.delete("quietfog"); }
    if q.has(PRICE_RUN_WORD) { for k in HELD_PAGE_WORDS { q.delete(k); } }
    for k in ["bench", "benchcap"] { q.delete(k); }
    for k in RUN_PAGE_WORDS { q.delete(k); }
    for k in PRICE_ASK_WORDS { q.delete(k); }
    q.delete("cams");
    q.bare()
}

/// The query string a click on a row loads: the row's own, then the visitor's other flags.
pub fn pick_search(e: &ScenarioEntry, search: &str) -> String {
    let rest = quit_search(search);
    if rest.is_empty() { scenario_query(e) } else { format!("{}&{}", scenario_query(e), &rest[1..]) }
}

/// The row of `list` an address boots: same kind, size, level and seed.
pub(in crate::missions) fn entry_of_query<'a>(q: &Query, list: impl IntoIterator<Item = &'a ScenarioEntry>) -> Option<&'a ScenarioEntry> {
    let s = spec_of(q)?;
    list.into_iter().find(|e| {
        e.kind.as_str() == s.kind.as_str()
            && e.squad == s.squad
            && e.level.as_deref().unwrap_or("") == s.level.as_deref().unwrap_or("")
            && (e.kind != ScenarioEntryKind::Roomclear || e.seed == s.seed)
            // (a quiet row with no seed of its own is its row whatever deal the address pins)
            && (e.kind != ScenarioEntryKind::Quiet || e.seed.is_none() || e.seed == s.seed)
    })
}

/// The address names something to play: a page that boots it and carries no rows.
pub(in crate::missions) fn names_mission(search: &str) -> bool {
    let q = Query::parse(search);
    spec_of(&q).is_some() || q.has(SOLO_KEY)
}

// ---------------------------------------------------------------- the short list

/// One row of the short list.
#[derive(Clone, Copy, Debug)]
pub struct Pick {
    pub id: &'static str,
    pub mission: &'static str,
    pub line: &'static str,
}

/// THE LIST, in play order.
pub const PICKS: [Pick; 3] = [
    Pick { id: "compound", mission: "compound-four", line: "Start here, at night. The gate beside you is dark, the main gate is lit, and nobody holds the pantry." },
    Pick { id: "depot", mission: "depot-four", line: "A lit yard watched from the dark. The mains are behind the hut that has no door on the yard." },
    Pick { id: "solo", mission: SOLO_ID, line: "Alone in the office after dark. Reach the server room, pull the drive, leave by the fire exit." },
];

const PICK_KEY: &str = "pick";
/// every word a pick writes or replaces; the visitor's own of these never ride into a pick
const PICK_KEYS: [&str; 10] = ["hard", "difficulty", "digin", "pressure", "hour", "sun", "weather", PICK_KEY, "pair", "silentstart"];

fn parts_of(search: &str) -> impl Iterator<Item = &str> { after_mark(search).split('&').filter(|k| !k.is_empty()) }

/// `search` without the words in `drop`, every other flag as it was typed, in order. (A name the language could not decode throws there, on an address nobody's link writes: here it stays.)
fn without(search: &str, drop: &[&str]) -> String {
    let keep: Vec<&str> = parts_of(search).filter(|k| {
        let name = k.split('=').next().unwrap_or("");
        decoded(name).map_or(true, |n| !drop.contains(&n.as_str()))
    }).collect();
    if keep.is_empty() { String::new() } else { format!("?{}", keep.join("&")) }
}

/// The short list's row by its id.
pub(in crate::missions) fn pick_by_id(id: &str) -> Option<Pick> { PICKS.into_iter().find(|p| p.id == id) }
/// The short list's row for a mission, its first.
pub(in crate::missions) fn pick_of_mission(id: &str) -> Option<Pick> { PICKS.into_iter().find(|p| p.mission == id) }
/// A mission the short list names.
pub(in crate::missions) fn on_short_list(id: &str) -> bool { pick_of_mission(id).is_some() }
/// A row by its id, or by its place ('3' = the third).
fn pick_named(name: Option<&str>) -> Option<Pick> {
    let name = name.filter(|n| !n.is_empty())?;
    if let Some(p) = pick_by_id(name) { return Some(p); }
    if !name.bytes().all(|c| c.is_ascii_digit()) { return None; }
    let at = crate::js::number_of(name) - 1.0;
    if at >= 0.0 && at < PICKS.len() as f64 { Some(PICKS[at as usize]) } else { None }
}
/// The row a pick starts; none for the solo night.
pub(in crate::missions) fn pick_entry(p: &Pick) -> Option<ScenarioEntry> {
    all_missions().into_iter().find(|e| e.id == p.mission)
}
/// The difficulty a page's address spells out, where it is one a row can be started at.
fn difficulty_of(search: &str) -> Option<Difficulty> {
    match Query::parse(search).get("difficulty") { Some("standard") => Some(Difficulty::Standard), Some("hard") => Some(Difficulty::Hard), _ => None }
}

/// THE QUERY STRING A PICK LOADS: the mission's own, its difficulty written out, its name last, the visitor's other flags behind.
pub(in crate::missions) fn pick_href(p: &Pick, search: &str, difficulty: Option<Difficulty>) -> String {
    let entry = pick_entry(p);
    let own = entry.as_ref().map_or_else(|| format!("?{SOLO_KEY}"), scenario_query);
    let mut parts: Vec<String> = parts_of(&own).map(String::from).collect();
    // (every row opens on standard: no row names a difficulty of its own)
    if entry.is_some() { parts.push(format!("difficulty={}", difficulty.unwrap_or(Difficulty::Standard).as_str())); }
    parts.push(format!("{PICK_KEY}={}", p.id));
    let rest = without(&quit_search(search), &PICK_KEYS);
    parts.extend(parts_of(&rest).map(String::from));
    format!("?{}", parts.join("&"))
}

/// Two query strings carry the same words, whatever their order or escaping.
fn same_parts(a: &str, b: &str) -> bool {
    let pairs = |s: &str| { let mut out: Vec<(String, String)> = Query::parse(after_mark(s)).words().to_vec(); out.sort(); out };
    pairs(a) == pairs(b)
}

/// THE PICK A PAGE IS RUNNING: its name on an address that is exactly what that pick loads, at either difficulty.
pub(in crate::missions) fn pick_of_query(search: &str) -> Option<Pick> {
    let p = pick_by_id(Query::parse(search).get(PICK_KEY)?)?;
    same_parts(&pick_href(&p, search, difficulty_of(search)), search).then_some(p)
}

/// Where an address that names a row of the short list and nothing to play goes instead (None: it stays, as the row's own link does).
pub fn pick_goes(search: &str) -> Option<String> {
    let q = Query::parse(search);
    let p = pick_named(q.get(PICK_KEY))?;
    if spec_of(&q).is_some() || SCENARIO_KEYS.iter().any(|k| q.has(k)) { return None; }
    Some(pick_href(&p, search, difficulty_of(search)))
}

/// A pick's page handing its address to the mission list: the pick's name comes off, so that a mission started from there is not that pick.
pub(in crate::missions) fn pick_leave_search(search: &str) -> String {
    if pick_of_query(search).is_some() { without(search, &[PICK_KEY]) } else { String::from(search) }
}

/// A quit out of whatever the address started: the plain page's address.
pub fn pick_quit_search(search: &str) -> String {
    if pick_of_query(search).is_some() { return without(&quit_search(search), &PICK_KEYS); }
    let out = quit_search(search);
    // (a row's name left on an address something was typed over must not ride out: the plain page would answer it by loading the mission again)
    if pick_named(Query::parse(&out).get(PICK_KEY)).is_some() { without(&out, &[PICK_KEY]) } else { out }
}

/// The row after this one, and the difficulty the page chose over the row's own, which goes on with it.
pub(in crate::missions) fn pick_after(search: &str) -> Option<(Pick, Option<Pick>, Option<Difficulty>)> {
    let now = pick_of_query(search)?;
    let at = PICKS.iter().position(|p| p.id == now.id)?;
    let next = PICKS.get(at + 1).copied();
    // (a row with one difficulty opens on it; a row left at standard has chosen nothing)
    let chosen = if pick_entry(&now).is_some() { difficulty_of(search).filter(|d| *d != Difficulty::Standard) } else { None };
    let carried = chosen.filter(|_| next.as_ref().is_some_and(|n| pick_entry(n).is_some()));
    Some((now, next, carried))
}
