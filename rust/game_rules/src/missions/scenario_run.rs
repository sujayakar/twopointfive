pub use ::game_types::scenario_run::types;
pub use types::*;
use crate::scene_scenarios::MissionKind;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use alloc::{string::String, vec::Vec};

/// A scenario's name in the player's words, by its kind's word.
fn word(kind: &str) -> String {
    words().get(String::from(kind)).cloned().expect("a scenario kind with no word")
}

/// The query's level is a Training lesson's.
fn lesson_level(q: &crate::query::Query) -> bool {
    // A row with no level is `undefined`, which is never `===` the `null` of a query with no levelFile.
    let Some(level_file) = q.get("levelFile") else { return false };
    crate::scene_scenario_list::tutorial_missions().iter().any(|e| e.level.as_deref() == Some(level_file))
}

/// Fog of war on a page with no scenario up: only if the URL asks.
fn plain_fog(q: &crate::query::Query) -> bool {
    !(q.has("nofow") || q.has("nofogofwar")) && (q.has("fow") || q.has("fogofwar"))
}

/// The link is one the quiet fog reads: a quiet mission's, or the solo night's own.
fn quiet_like(g: &crate::game::Game, q: &crate::query::Query) -> bool {
    match crate::scene_scenario_list::spec_of(q) {
        Some(s) => s.kind == crate::scene_scenarios::MissionKind::Quiet,
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
pub(super) fn quiet_rung_of_query(g: &crate::game::Game, q: &crate::query::Query) -> crate::quietfog::QuietRung {
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
pub(super) fn fog_of_link(g: &crate::game::Game, q: &crate::query::Query) -> bool {
    if q.has("nofow") || q.has("nofogofwar") {
        return false;
    }
    use crate::scene_scenario_list::kind_of;
    use crate::scene_scenarios::MissionKind as Kind;
    q.has("fow")
        || q.has("fogofwar")
        || matches!(kind_of(q), Some(Kind::Roomclear | Kind::Rescue))
        || quiet_rung_of_query(g, q) != crate::quietfog::QuietRung::Off
}

/// Stage `spec` on this game: the squad's size on the knob the staging reads, then the scenario's own start. Its refusal, or None.
pub fn start_scenario(g: &mut crate::game::Game, spec: &crate::scene_scenario_list::ScenarioSpec) -> Option<String> {
    if spec.kind != crate::scene_scenarios::MissionKind::Roomclear {
        crate::difficulty::difficulty_here(g, "");
    }
    match spec.kind {
        crate::scene_scenarios::MissionKind::Quiet => {}
        crate::scene_scenarios::MissionKind::Rescue => g.k.rescue.squad = spec.squad,
        crate::scene_scenarios::MissionKind::Nightbreach | crate::scene_scenarios::MissionKind::Roomclear => g.k.nightbreach.squad = spec.squad,
    }
    let why = match crate::seats::seats_refuse(g, spec.kind.as_str()) {
        Some(w) => Some(w),
        None => match spec.kind {
            crate::scene_scenarios::MissionKind::Quiet => {
                let partner = if js::truthy(spec.squad) { 1.0 } else { 0.0 };
                crate::quiet::quiet_mission_start(g, spec.seed, Some(partner))
            }
            crate::scene_scenarios::MissionKind::Roomclear => crate::roomclear::room_clear_scenario_start(g, spec.seed),
            crate::scene_scenarios::MissionKind::Nightbreach => crate::nightbreach::night_breach_start(g, None),
            crate::scene_scenarios::MissionKind::Rescue => crate::rescue::rescue_start(g),
        },
    };
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
        if g.weather.kind_now != Some(wx) {
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
    // (a Training level's stagings wear what its lesson asks: kit.ts kitLessonPage)
    crate::kit::kit_lesson_page(g, lesson_of_address(q).is_some());
    // (weatherKinds.ts weatherAsked, then snowLevelOfQuery: a level built for snow is in snow whatever the address said)
    let snow = q.get("levelFile").and_then(|f| f.strip_suffix(".json")).map_or(false, |n| crate::weather_kinds::snow_level(g, Some(n)));
    let kind = if snow { Some(crate::weather_kinds::WeatherKind::Snow) } else { crate::weather_kinds::weather_of(q) };
    // (hours.ts hourAsked: the visit's hour, which the stand lays over the level once the game is made)
    let sun = q.float("sun").filter(|s| s.is_finite());
    g.stood = crate::game::StoodWith { weather: kind, locked: true, hour: crate::hours::hour_id_of(q.get("hour")), sun: sun.map_or(1.0, |s| s.clamp(0.0, 1.0)), sun_given: sun.is_some(), dry: !q.has("noweather"), solo_stand: false, way: crate::roomclear::way_of_address(q) };
    kind
}

/// the Training row whose level an address boots
pub fn lesson_of_address(q: &crate::query::Query) -> Option<crate::scene_scenarios::ScenarioEntry> {
    let file = q.get("levelFile")?;
    crate::scene_scenario_list::tutorial_missions().into_iter().find(|e| e.level.as_deref() == Some(file))
}

/// …AND OF THE STOOD GAME: the mission the address names, staged, then the solo card's question (the plain night, or a quiet mission alone, wears
/// the player's own card; a squad's mission leaves the kit to its own staging).
pub fn start_at_address(g: &mut crate::game::Game, q: &crate::query::Query) -> crate::scenario_run::StartFromQuery {
    // (?noweather: the level's own storm off from the start, as the page puts it off once its game is stood)
    if q.has("noweather") { crate::weather::weather_set_enabled_2(g, false); }
    let r = start_from_query(g, &StartFromQuerySearch::URLSearchParams(q.clone()));
    let solo = q.has("solo") || !g.staged.facts.unseen;
    crate::kit::kit_solo_if(g, solo);
    r
}

/// The row of the mission list the mission that is up is of: its kind on this level's file, at its size where one is. None: none is up, or no row has it.
pub fn running_row(g: &crate::game::Game, level_file: Option<&str>) -> Option<crate::scene_scenarios::ScenarioEntry> {
    let kind = g.staged.up?;
    let squad = m::min(4.0, g.staged.team.len() as f64);
    let mut here = crate::scene_scenario_list::all_missions().into_iter().filter(|e| e.kind == kind && e.level.as_deref() == level_file).peekable();
    let first = here.peek().cloned();
    here.find(|e| e.squad == squad).or(first)
}
