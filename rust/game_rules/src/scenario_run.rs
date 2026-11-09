pub use ::game_types::scenario_run::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use alloc::{string::String, vec::Vec};

/// A scenario's name in the player's words, by its kind's word.   (scenarioRun.ts WORDS, the skeleton's `words()`)
fn word(kind: &str) -> String {
    words().get(String::from(kind)).cloned().expect("a scenario kind with no word")
}

/// `w[0].toUpperCase() + w.slice(1)`; every word of WORDS starts with an ASCII letter.
fn capitalised(w: &str) -> String {
    let mut s = String::from(w);
    if let Some(first) = s.get_mut(0..1) {
        first.make_ascii_uppercase();
    }
    s
}

/// The query's level is a Training lesson's.
pub fn lesson_level(q: &crate::query::Query) -> bool {
    // A row with no level is `undefined`, which is never `===` the `null` of a query with no levelFile.
    let Some(level_file) = q.get("levelFile") else { return false };
    crate::scene_scenario_list::tutorial_missions().iter().any(|e| e.level.as_deref() == Some(level_file))
}

/// Fog of war on a page with no scenario up: only if the URL asks.
pub fn plain_fog(q: &crate::query::Query) -> bool {
    !(q.has("nofow") || q.has("nofogofwar")) && (q.has("fow") || q.has("fogofwar"))
}

/// The link is one the quiet fog reads: a quiet mission's, or the solo night's own.
pub fn quiet_like(g: &crate::game::Game, q: &crate::query::Query) -> bool {
    match crate::scene_scenario_list::spec_of(q) {
        Some(s) => s.kind == crate::scene_scenario_list::ScenarioSpecKind::Quiet,
        None => crate::quietfog::solo_fog_link(g, q),
    }
}

/// The sticky cameras a link hands the player for the visit where its fog runs.
pub fn quiet_cams_of_link(g: &crate::game::Game, q: &crate::query::Query) -> f64 {
    if quiet_like(g, q) {
        let lesson = lesson_level(q);
        crate::quietfog::quiet_cams_of_query(g, q, Some(lesson))
    } else {
        0.0
    }
}

/// The rung of the fog lever a quiet mission's query asks to be played under.
pub fn quiet_rung_of_query(g: &crate::game::Game, q: &crate::query::Query) -> crate::quietfog::QuietRung {
    if quiet_like(g, q) {
        let lesson = lesson_level(q);
        crate::quietfog::quiet_fog_of_query(g, q, Some(lesson))
    } else {
        crate::quietfog::QuietRung::Off
    }
}

/// Fog of war as a query string asks for it, the squad's fog lever heard.
pub fn fog_of_query(g: &crate::game::Game, q: &crate::query::Query) -> bool {
    if crate::difficulty::fog_off_now(g) && !quiet_like(g, q) && !lesson_level(q) && !plain_fog(q) {
        return false;
    }
    fog_of_link(g, q)
}

/// Fog of war as the link alone asks, whatever the squad's fog lever says.
pub fn fog_of_link(g: &crate::game::Game, q: &crate::query::Query) -> bool {
    if q.has("nofow") || q.has("nofogofwar") {
        return false;
    }
    let four = q.has("squad4") && !q.has("nightbreach") && !q.has("rescue") && !q.has("quiet");
    q.has("fow")
        || q.has("fogofwar")
        || q.has("roomclear")
        || q.has("rescue")
        || four
        || quiet_rung_of_query(g, q) != crate::quietfog::QuietRung::Off
}

/// Stage `spec` on this game: the squad's size on the knob the staging reads, then the scenario's own start. Its refusal, or None.
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

/// The page's boot: fog of war as the query asks, then the scenario it names, staged and waiting. A refused start leaves the plain floor.
pub fn start_from_query(g: &mut crate::game::Game, search: &crate::scenario_run::StartFromQuerySearch) -> crate::scenario_run::StartFromQuery {
    // (the game's own copy of the words: the row's hour and weather are written onto it, never onto the caller's)
    let mut q = match search {
        StartFromQuerySearch::Text(s) => crate::query::Query::parse(s),
        StartFromQuerySearch::URLSearchParams(q) => q.clone(),
    };
    crate::rowset::row_settings(g, &mut q);
    let q = &q;
    crate::hours::hour_from_query(g, q);
    let fog = fog_of_query(g, q);
    g.fog_of_war.on = fog;
    let rung = if g.fog_of_war.on { quiet_rung_of_query(g, q) } else { crate::quietfog::QuietRung::Off };
    crate::quietfog::quiet_fog_start(g, q, rung);
    if let Some(wx) = crate::weather_kinds::weather_of(q) {
        if crate::weather::weather_kind(g) != Some(wx) {
            crate::weather::weather_set_kind_2(g, Some(wx));
        }
    }
    let spec = crate::scene_scenario_list::spec_of(q);
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
    StartFromQuery { why }
}

/// A MISSION'S ADDRESS IS READ IN TWO PLACES by whoever starts a game from one. BEFORE THE GAME IS MADE, on the blank game: the mission row's own
/// hour and weather written onto the words, then the weather the game is BORN in (`?weather=`, or snow on a level built for it), locked (the
/// address's for the life of a game this start stands, as a page's is), and the visit's hour (`?hour=`, `?sun=`), which the stand lays over the level. A Weather born in its kind lays its roofs, its wet ground and its wind once; a host that stood the level's
/// own storm first and changed the kind afterwards had another box table than a page.
pub fn address_before_game(g: &mut crate::game::Game, q: &mut crate::query::Query) -> Option<crate::weather_kinds::WeatherKind> {
    crate::rowset::row_settings(g, q);
    crate::kit::kit_lesson_page(g, lesson_of_address(q).is_some());
    let snow = q.get("levelFile").and_then(|f| f.strip_suffix(".json")).map_or(false, |n| crate::weather_kinds::snow_level(g, Some(n)));
    let kind = if snow { Some(crate::weather_kinds::WeatherKind::Snow) } else { crate::weather_kinds::weather_of(q) };
    let sun = q.float("sun").filter(|s| s.is_finite());
    g.stood = crate::game::StoodWith { weather: kind, locked: true, hour: crate::hours::hour_id_of(q.get("hour")), sun: sun.map_or(1.0, |s| s.clamp(0.0, 1.0)), sun_given: sun.is_some(), dry: !q.has("noweather"), solo_stand: false };
    kind
}

/// the Training row whose level an address boots
fn lesson_of_address(q: &crate::query::Query) -> Option<crate::scene_scenarios::ScenarioEntry> {
    let file = q.get("levelFile")?;
    crate::scene_scenario_list::tutorial_missions().into_iter().find(|e| e.level.as_deref() == Some(file))
}

/// …AND THE DIFFICULTY IT ASKS FOR, before the game is made, on the blank game's own knob tables, as a page reads it: a Training lesson's address reads no lever, as its page
/// doesn't. It is the address's alone: no host but a page keeps a level.
pub fn difficulty_of_address(g: &mut crate::game::Game, q: &crate::query::Query) {
    let plain = lesson_of_address(q).is_some();
    if let Some(a) = crate::difficulty::difficulty_asked(g, q, plain) { crate::difficulty::difficulty_lay(g, &a); }
}

/// …AND OF THE STOOD GAME: the mission the address names, staged, then the solo card's question (the plain night, or a quiet mission alone, wears
/// the player's own card; a squad's mission leaves the kit to its own staging).
pub fn start_at_address(g: &mut crate::game::Game, q: &crate::query::Query) -> crate::scenario_run::StartFromQuery {
    // (?noweather: the level's own storm off from the start, as the page puts it off once its game is stood)
    if q.has("noweather") { crate::weather::weather_set_enabled_2(g, false); }
    let r = start_from_query(g, &StartFromQuerySearch::URLSearchParams(q.clone()));
    let solo = q.has("solo") || !g.quiet.active;
    crate::kit::kit_solo_if(g, solo);
    r
}

/// Which scenario is up on this game right now; the room clear is asked first, being the night breach on its own layout.
pub fn active_scenario(g: &crate::game::Game) -> Option<crate::scenario_run::ActiveScenario> {
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

/// The teammates the scenario that is up has staged, dead or alive: a copy of its list.
pub fn team_of(g: &crate::game::Game) -> Vec<GuardId> {
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

/// The scenario that is up, in the picker's words. None: none is up.
pub fn running_scenario(g: &crate::game::Game, level_file: Option<&str>, list: Option<&[crate::scene_scenarios::ScenarioEntry]>) -> Option<crate::scenario_run::RunningScenario> {
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
    let team = crate::loadout::size_words(squad);
    if kind == ActiveScenario::Quiet {
        return Some(RunningScenario { kind: running_kind(kind), entry });
    }
    let title = match &entry {
        Some(e) => e.title.clone(),
        None => capitalised(&word(kind.as_str())),
    };
    Some(RunningScenario { kind: running_kind(kind), entry })
}
