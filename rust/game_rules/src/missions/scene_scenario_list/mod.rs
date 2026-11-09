//! WHAT AN ADDRESS ASKS FOR: the kind, the team, the level's file and the seed, read as the page starts it.

pub use ::game_types::scene_scenario_list::types;
pub use types::*;
pub use ::game_types::scene_scenario_list::rows::*;
pub mod short;
pub use short::*;
use crate::js::{self, JsMap, JsSet};
use crate::scene_scenarios::MissionKind;
use crate::m;
use alloc::string::String;

use crate::query::Query;

/// What a query asks for, read as the page starts it: which scenario, how many teammates, which level file and seed; None when it names none.
/// What every reader in the game that holds an address's words asks.
pub(super) fn spec_of(q: &Query) -> Option<crate::scene_scenario_list::ScenarioSpec> {
    let kind = kind_of(q)?;
    // a quiet mission is alone unless `squad=1`; any bigger size it is asked for is the one partner
    let squad = if kind == MissionKind::Quiet {
        if crate::loadout::squad_of(q, false).unwrap_or(0.0) > 0.0 { 1.0 } else { 0.0 }
    } else {
        crate::loadout::squad_of(q, kind != MissionKind::Rescue).unwrap_or(2.0)
    };
    let (level, seed) = place_of(kind, q);
    Some(ScenarioSpec { kind, squad, level, seed })
}

/// WHAT KIND OF MISSION AN ADDRESS NAMES, the one reader of it (none: the solo night). `?squad4` alone is the room clear with the four; beside another scenario's flag it is only that scenario's size.
pub fn kind_of(q: &Query) -> Option<MissionKind> {
    let four = q.has("squad4");
    if q.has("roomclear") || (four && !q.has("nightbreach") && !q.has("rescue") && !q.has("quiet")) {
        Some(MissionKind::Roomclear)
    } else if q.has("nightbreach") {
        Some(MissionKind::Nightbreach)
    } else if q.has("rescue") {
        Some(MissionKind::Rescue)
    } else if q.has("quiet") {
        Some(MissionKind::Quiet)
    } else {
        None
    }
}

/// The level file and the placement seed the query pins for this kind.
fn place_of(kind: MissionKind, q: &Query) -> (Option<String>, Option<f64>) {
    let level = q.get("levelFile").filter(|lf| is_level_file(lf)).map(String::from);
    let seed = match kind {
        MissionKind::Roomclear => seed_of_word(q.get("roomclear").filter(|s| !s.is_empty()).or(q.get("squad4"))),
        MissionKind::Quiet => seed_of_word(q.get("quiet")),
        _ => None,
    };
    (level, seed)
}

/// `s && Number.isFinite(Number(s))` then `Math.floor(Number(s))`.
fn seed_of_word(s: Option<&str>) -> Option<f64> {
    let n = js::number_of(s.filter(|s| !s.is_empty())?);
    if n.is_finite() { Some(m::floor(n)) } else { None }
}

/// /^[\w.-]+\.json$/
fn is_level_file(lf: &str) -> bool {
    match lf.strip_suffix(".json") {
        Some(stem) => !stem.is_empty() && stem.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'.' || b == b'-'),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use MissionKind as K;

    /// A FAILURE MEANS: a link starts another mission than the row it came from: an address names one kind, and of two the first here wins. `?squad4` alone is a room clear; beside another mission's word it is only that mission's size.
    #[test]
    fn an_address_names_one_kind_of_mission() {
        for (text, kind) in [
            ("?rescue", K::Rescue), ("?quiet", K::Quiet), ("?nightbreach", K::Nightbreach), ("?roomclear", K::Roomclear), ("?squad4", K::Roomclear),
            ("?squad4&rescue", K::Rescue), ("?squad4&quiet", K::Quiet), ("?squad4&nightbreach", K::Nightbreach),
            ("?roomclear&quiet=5", K::Roomclear), ("?nightbreach&rescue", K::Nightbreach), ("?rescue&quiet", K::Rescue),
        ] {
            assert_eq!(kind_of(&Query::parse(text)), Some(kind), "{text:?}");
        }
    }

    #[test]
    fn a_query_naming_no_scenario_is_none() {
        for text in ["", "?", "?mute", "?squad=4", "??rescue", "?ROOMCLEAR"] {
            assert!(spec_of(&Query::parse(text)).is_none(), "{text:?}");
        }
    }

    /// A seed pins who stands where: only a room clear and a quiet mission take one, it is a whole number, and a word that is no number pins nothing.
    #[test]
    fn a_seed_is_a_whole_number_and_only_two_kinds_take_one() {
        for (text, seed) in [
            ("?quiet=12.7", Some(12.0)), ("?quiet=-3.5", Some(-4.0)), ("?quiet=1&quiet=2", Some(1.0)),
            ("?quiet", None), ("?quiet=", None), ("?quiet=abc", None), ("?quiet=Infinity", None),
            ("?roomclear=7", Some(7.0)), ("?squad4=3", Some(3.0)), ("?roomclear&squad4=9", Some(9.0)), ("?roomclear=x&squad4=9", None),
            ("?rescue=4", None), ("?nightbreach=4", None), ("?squad4=3&nightbreach", None),
        ] {
            let q = Query::parse(text);
            assert_eq!(place_of(kind_of(&q).unwrap(), &q).1, seed, "{text:?}");
        }
    }

    /// A FAILURE MEANS: an address can ask the host for any path: a level is a plain file name ending in .json, the first one named, and anything else is the kind's own level.
    #[test]
    fn a_level_is_a_plain_file_name() {
        for (text, level) in [
            ("?rescue&levelFile=a.json", Some("a.json")), ("?rescue&levelFile=a-b_c.d.json", Some("a-b_c.d.json")), ("?rescue&levelFile=first.json&levelFile=b.json", Some("first.json")),
            ("?rescue&levelFile=../x.json", None), ("?rescue&levelFile=x%2Fy.json", None), ("?rescue&levelFile=a%20b.json", None), ("?rescue&levelFile=a.json%0A", None),
            ("?rescue&levelFile=.json", None), ("?rescue&levelFile=a.JSON", None), ("?rescue&levelFile=a.jsonx", None), ("?rescue&levelFile=", None),
        ] {
            let q = Query::parse(text);
            assert_eq!(place_of(K::Rescue, &q).0.as_deref(), level, "{text:?}");
        }
    }
}
