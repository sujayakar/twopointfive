//! src/game/scenarioRun.ts lines 1 to 109
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Each scenario's name in the player's words, by its kind's word.   (scenarioRun.ts WORDS)
static WORDS: &[(&str, &str)] = &[("nightbreach", "night breach"), ("roomclear", "room clear"), ("rescue", "hostage rescue"), ("quiet", "quiet mission")];

fn word(kind: &str) -> &'static str {
    for &(k, w) in WORDS {
        if k == kind {
            return w;
        }
    }
    unreachable!("a scenario kind with no word")
}

/// `w[0].toUpperCase() + w.slice(1)`; every word of WORDS starts with an ASCII letter.
fn capitalised(w: &str) -> String {
    let mut s = String::from(w);
    if let Some(first) = s.get_mut(0..1) {
        first.make_ascii_uppercase();
    }
    s
}

fn seam_q_has(q: crate::Opaque, key: &str) -> bool {
    todo!("needs: URLSearchParams as a real type (crate::Opaque holds nothing): q.has(key)")
}

fn seam_q_get(q: crate::Opaque, key: &str) -> Option<String> {
    todo!("needs: URLSearchParams as a real type (crate::Opaque holds nothing): q.get(key)")
}

fn seam_query_of_text(text: &str) -> crate::Opaque {
    todo!("needs: URLSearchParams as a real type (crate::Opaque holds nothing): new URLSearchParams(text)")
}

fn seam_query_copy(q: crate::Opaque) -> crate::Opaque {
    todo!("needs: URLSearchParams as a real type (crate::Opaque holds nothing): new URLSearchParams(q), a copy")
}

fn seam_row_settings(g: &mut Game, q: &mut crate::Opaque) {
    todo!("needs: rowset::row_settings to take the query by &mut: it writes the row's hour and weather onto it (rowset.ts:45-46), lost on a by-value Opaque")
}

/// The query's level is a Training lesson's.   (scenarioRun.ts lessonLevel)
pub fn lesson_level(q: crate::Opaque) -> bool {
    let level_file = seam_q_get(q, "levelFile");
    // A row with no level is `undefined`, which is never `===` the `null` of a query with no levelFile.
    crate::scene_scenario_list::lesson_missions().iter().any(|e| match (&e.level, &level_file) {
        (Some(a), Some(b)) => a == b,
        _ => false,
    })
}

/// Fog of war on a page with no scenario up: only if the URL asks.   (scenarioRun.ts plainFog)
pub fn plain_fog(q: crate::Opaque) -> bool {
    !(seam_q_has(q, "nofow") || seam_q_has(q, "nofogofwar")) && (seam_q_has(q, "fow") || seam_q_has(q, "fogofwar"))
}

/// The link is one the quiet fog reads: a quiet mission's, or the solo night's own.   (scenarioRun.ts quietLike)
pub fn quiet_like(g: &mut crate::game::Game, q: crate::Opaque) -> bool {
    match crate::scene_scenario_list::spec_of_query(&crate::scene_scenario_list::SpecOfQuerySearch::URLSearchParams(q)) {
        Some(s) => s.kind == crate::scene_scenario_list::ScenarioSpecKind::Quiet,
        None => crate::quietfog::solo_fog_link(g, q),
    }
}

/// The sticky cameras a link hands the player for the visit where its fog runs.   (scenarioRun.ts quietCamsOfLink)
pub fn quiet_cams_of_link(g: &mut crate::game::Game, q: crate::Opaque) -> f64 {
    if quiet_like(g, q) {
        let lesson = lesson_level(q);
        crate::quietfog::quiet_cams_of_query(g, q, Some(lesson))
    } else {
        0.0
    }
}

/// The rung of the fog lever a quiet mission's query asks to be played under.   (scenarioRun.ts quietRungOfQuery)
pub fn quiet_rung_of_query(g: &mut crate::game::Game, q: crate::Opaque) -> crate::quietfog::QuietRung {
    if quiet_like(g, q) {
        let lesson = lesson_level(q);
        crate::quietfog::quiet_fog_of_query(g, q, Some(lesson))
    } else {
        crate::quietfog::QuietRung::Off
    }
}

/// `s.replace(/^[^?]*\?/, '')`: a link's text after its first `?`, or all of it with none.
fn after_first_question_mark(s: &str) -> &str {
    match s.find('?') {
        Some(i) => &s[i + 1..],
        None => s,
    }
}

/// What makes a link's fog: on or off, the rung, the cameras.   (scenarioRun.ts sameFog's `sig`, compared as a value rather than as its text)
fn fog_sig(g: &mut Game, q: crate::Opaque) -> (bool, crate::quietfog::QuietRung, f64) {
    let on = fog_of_query(g, q);
    let rung = quiet_rung_of_query(g, q);
    let cams = quiet_cams_of_link(g, q);
    (on, rung, cams)
}

/// Two links stand in the same fog.   (scenarioRun.ts sameFog)
pub fn same_fog(g: &mut crate::game::Game, a: &crate::scenario_run::SameFogA, b: &crate::scenario_run::SameFogB) -> bool {
    let qa = match a {
        SameFogA::Text(s) => seam_query_of_text(after_first_question_mark(s)),
        SameFogA::URLSearchParams(q) => *q,
    };
    let sig_a = fog_sig(g, qa);
    let qb = match b {
        SameFogB::Text(s) => seam_query_of_text(after_first_question_mark(s)),
        SameFogB::URLSearchParams(q) => *q,
    };
    let sig_b = fog_sig(g, qb);
    sig_a == sig_b
}

/// Fog of war as a query string asks for it, the squad's fog lever heard.   (scenarioRun.ts fogOfQuery)
pub fn fog_of_query(g: &mut crate::game::Game, q: crate::Opaque) -> bool {
    if crate::difficulty::fog_off_now(g) && !quiet_like(g, q) && !lesson_level(q) && !plain_fog(q) {
        return false;
    }
    fog_of_link(g, q)
}

/// Fog of war as the link alone asks, whatever the squad's fog lever says.   (scenarioRun.ts fogOfLink)
pub fn fog_of_link(g: &mut crate::game::Game, q: crate::Opaque) -> bool {
    if seam_q_has(q, "nofow") || seam_q_has(q, "nofogofwar") {
        return false;
    }
    let four = seam_q_has(q, "squad4") && !seam_q_has(q, "nightbreach") && !seam_q_has(q, "rescue") && !seam_q_has(q, "quiet");
    seam_q_has(q, "fow")
        || seam_q_has(q, "fogofwar")
        || seam_q_has(q, "roomclear")
        || seam_q_has(q, "rescue")
        || four
        || quiet_rung_of_query(g, q) != crate::quietfog::QuietRung::Off
}

/// Stage `spec` on this game: the squad's size on the knob the staging reads, then the scenario's own start. Its refusal, or None.   (scenarioRun.ts startScenario)
pub fn start_scenario(g: &mut crate::game::Game, spec: &crate::scene_scenario_list::ScenarioSpec) -> Option<String> {
    if spec.kind != crate::scene_scenario_list::ScenarioSpecKind::Roomclear {
        crate::difficulty::difficulty_here(g, "");
    }
    match spec.kind {
        crate::scene_scenario_list::ScenarioSpecKind::Quiet => {}
        crate::scene_scenario_list::ScenarioSpecKind::Rescue => g.k.rescue.squad = spec.squad,
        crate::scene_scenario_list::ScenarioSpecKind::Nightbreach | crate::scene_scenario_list::ScenarioSpecKind::Roomclear => g.k.nightbreach.squad = spec.squad,
    }
    let why = match crate::seats::seats_refuse(g, spec.kind.as_str()) {
        Some(w) => Some(w),
        None => match spec.kind {
            crate::scene_scenario_list::ScenarioSpecKind::Quiet => {
                let partner = if js::truthy(spec.squad) { 1.0 } else { 0.0 };
                crate::quiet::quiet_mission_start(g, spec.seed, Some(partner))
            }
            crate::scene_scenario_list::ScenarioSpecKind::Roomclear => crate::roomclear::room_clear_scenario_start(g, spec.seed),
            crate::scene_scenario_list::ScenarioSpecKind::Nightbreach => crate::nightbreach::night_breach_start(g, None),
            crate::scene_scenario_list::ScenarioSpecKind::Rescue => crate::rescue::rescue_start(g),
        },
    };
    if let Some(w) = &why {
        let text = alloc::format!("{} refused — {}", word(spec.kind.as_str()), w);
        crate::game::game_msg(g, &text);
    }
    why
}

/// The page's boot: fog of war as the query asks, then the scenario it names, staged and waiting. A refused start leaves the plain floor.   (scenarioRun.ts startFromQuery)
pub fn start_from_query(g: &mut crate::game::Game, search: &crate::scenario_run::StartFromQuerySearch) -> crate::scenario_run::StartFromQuery {
    let mut q = match search {
        StartFromQuerySearch::Text(s) => seam_query_of_text(s),
        StartFromQuerySearch::URLSearchParams(q) => seam_query_copy(*q),
    };
    seam_row_settings(g, &mut q);
    crate::hours::hour_from_query(g, q);
    let fog = fog_of_query(g, q);
    g.fog_of_war.on = fog;
    let rung = if g.fog_of_war.on { quiet_rung_of_query(g, q) } else { crate::quietfog::QuietRung::Off };
    crate::quietfog::quiet_fog_start(g, q, rung);
    if let Some(wx) = crate::weather_kinds::weather_of_search(&crate::weather_kinds::WeatherOfSearchSearch::URLSearchParams(q)) {
        if g.weather.kind != Some(wx) {
            crate::weather::weather_set_kind(g, Some(wx));
        }
    }
    let spec = crate::scene_scenario_list::spec_of_query(&crate::scene_scenario_list::SpecOfQuerySearch::URLSearchParams(q));
    let cams = if g.fog_of_war.on { quiet_cams_of_link(g, q) } else { 0.0 };
    crate::stickycam::sticky_visit_set(g, cams);
    let why = match &spec {
        Some(s) => start_scenario(g, s),
        None => None,
    };
    if why.is_some() {
        let plain = plain_fog(q);
        g.fog_of_war.on = plain;
        crate::quietfog::quiet_fog_end(g);
        crate::stickycam::sticky_visit_set(g, 0.0);
        g.k.nightbreach.squad = 2.0;
        g.k.rescue.squad = 2.0;
    }
    crate::kit::kit_solo_stand(g, Some(spec.is_none()));
    StartFromQuery { spec, why }
}

/// A refusal in the card's words.   (scenarioRun.ts refusalWords)
pub fn refusal_words(spec: &crate::scene_scenario_list::ScenarioSpec, why: &str) -> String {
    alloc::format!("{} did not start: {}", capitalised(word(spec.kind.as_str())), why)
}

/// Which scenario is up on this game right now; the room clear is asked first, being the night breach on its own layout.   (scenarioRun.ts activeScenario)
pub fn active_scenario(g: &mut crate::game::Game) -> Option<crate::scenario_run::ActiveScenario> {
    if crate::roomclear::room_clear_scenario_active(g) {
        Some(ActiveScenario::Roomclear)
    } else if g.night_breach.active {
        Some(ActiveScenario::Nightbreach)
    } else if g.rescue.active {
        Some(ActiveScenario::Rescue)
    } else if g.quiet.active {
        Some(ActiveScenario::Quiet)
    } else {
        None
    }
}

/// The teammates the scenario that is up has staged, dead or alive: a copy of its list.   (scenarioRun.ts teamOf)
pub fn team_of(g: &mut crate::game::Game) -> Vec<GuardId> {
    if g.rescue.active {
        g.rescue.ops.clone()
    } else if g.night_breach.active {
        g.night_breach.ops.clone()
    } else if g.quiet.active {
        g.quiet.ops.clone()
    } else {
        Vec::new()
    }
}

fn running_kind(kind: ActiveScenario) -> RunningScenarioKind {
    match kind {
        ActiveScenario::Nightbreach => RunningScenarioKind::Nightbreach,
        ActiveScenario::Roomclear => RunningScenarioKind::Roomclear,
        ActiveScenario::Rescue => RunningScenarioKind::Rescue,
        ActiveScenario::Quiet => RunningScenarioKind::Quiet,
    }
}

fn quit_kind(kind: ActiveScenario) -> QuitScenario {
    match kind {
        ActiveScenario::Nightbreach => QuitScenario::Nightbreach,
        ActiveScenario::Roomclear => QuitScenario::Roomclear,
        ActiveScenario::Rescue => QuitScenario::Rescue,
        ActiveScenario::Quiet => QuitScenario::Quiet,
    }
}

/// The scenario that is up, in the picker's words. None: none is up.   (scenarioRun.ts runningScenario)
pub fn running_scenario(g: &mut crate::game::Game, level_file: Option<&str>, list: Option<&[crate::scene_scenarios::ScenarioEntry]>) -> Option<crate::scenario_run::RunningScenario> {
    let kind = active_scenario(g)?;
    let squad = if kind == ActiveScenario::Quiet { g.quiet.squad } else { m::min(4.0, team_of(g).len() as f64) };
    let all;
    let list = match list {
        Some(l) => l,
        None => {
            all = crate::scene_scenario_list::all_missions();
            &all[..]
        }
    };
    let here: Vec<&crate::scene_scenarios::ScenarioEntry> =
        list.iter().filter(|e| e.kind.as_str() == kind.as_str() && e.level.as_deref() == level_file).collect();
    let entry = here.iter().find(|e| e.squad == squad).or(here.first()).map(|e| (*e).clone());
    let team = crate::scene_scenario_list::team_words(crate::scene_scenario_list::TeamWordsE { squad });
    if kind == ActiveScenario::Quiet {
        let title = match &entry {
            Some(e) => e.title.clone(),
            None => String::from("Quiet mission"),
        };
        return Some(RunningScenario { kind: running_kind(kind), squad, entry, title, team, saves: Some(!js::truthy(squad)) });
    }
    let title = match &entry {
        Some(e) => e.title.clone(),
        None => capitalised(word(kind.as_str())),
    };
    Some(RunningScenario { kind: running_kind(kind), squad, entry, title, team, saves: None })
}

/// Quit whichever scenario is up, back to the floor the start card sits over. What was stopped, or None.   (scenarioRun.ts quitScenario)
pub fn quit_scenario(g: &mut crate::game::Game, fog: Option<bool>) -> Option<crate::scenario_run::QuitScenario> {
    let fog = fog.unwrap_or(false);
    let was = active_scenario(g);
    if was.is_some() {
        crate::stickycam::sticky_visit_set(g, 0.0);
    }
    match was {
        Some(ActiveScenario::Roomclear) => crate::roomclear::room_clear_scenario_stop(g),
        Some(ActiveScenario::Nightbreach) => crate::nightbreach::night_breach_stop(g),
        Some(ActiveScenario::Rescue) => crate::rescue::rescue_stop(g),
        Some(ActiveScenario::Quiet) => crate::quiet::quiet_mission_stop(g),
        None => {}
    }
    if was.is_some() {
        g.k.nightbreach.squad = 2.0;
        g.k.rescue.squad = 2.0;
        g.fog_of_war.on = fog;
        crate::quietfog::quiet_fog_end(g);
        crate::stickycam::sticky_visit_set(g, 0.0);
    }
    was.map(quit_kind)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(kind: crate::scene_scenario_list::ScenarioSpecKind) -> crate::scene_scenario_list::ScenarioSpec {
        crate::scene_scenario_list::ScenarioSpec { kind, squad: 2.0, level: None, seed: None }
    }

    #[test]
    fn refusal_words_capitalise_the_kinds_name() {
        assert_eq!(refusal_words(&spec(crate::scene_scenario_list::ScenarioSpecKind::Roomclear), "the AI is switched off"), "Room clear did not start: the AI is switched off");
        assert_eq!(refusal_words(&spec(crate::scene_scenario_list::ScenarioSpecKind::Rescue), "x"), "Hostage rescue did not start: x");
        assert_eq!(refusal_words(&spec(crate::scene_scenario_list::ScenarioSpecKind::Nightbreach), "x"), "Night breach did not start: x");
        assert_eq!(refusal_words(&spec(crate::scene_scenario_list::ScenarioSpecKind::Quiet), "x"), "Quiet mission did not start: x");
    }

    #[test]
    fn a_links_query_starts_after_its_first_question_mark() {
        assert_eq!(after_first_question_mark("/index.html?a=1?b"), "a=1?b");
        assert_eq!(after_first_question_mark("?a=1"), "a=1");
        assert_eq!(after_first_question_mark("a=1"), "a=1");
        assert_eq!(after_first_question_mark(""), "");
    }
}
