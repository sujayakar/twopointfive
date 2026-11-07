//! src/game/scenarioRun.ts lines 1 to 109
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

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
        None => capitalised(&word(kind.as_str())),
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

