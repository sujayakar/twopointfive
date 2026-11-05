#![allow(unused_variables)]
//! Every function and method of `scenario_run`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/scenarioRun.ts:21 lessonLevel)
pub fn lesson_level(q: crate::Opaque) -> bool { todo!() }

/// (src/game/scenarioRun.ts:23 plainFog)
pub fn plain_fog(q: crate::Opaque) -> bool { todo!() }

/// (src/game/scenarioRun.ts:29 quietLike)
pub fn quiet_like(g: &mut crate::game::Game, q: crate::Opaque) -> bool { todo!() }

/// (src/game/scenarioRun.ts:30 quietCamsOfLink)
pub fn quiet_cams_of_link(g: &mut crate::game::Game, q: crate::Opaque) -> f64 { todo!() }

/// (src/game/scenarioRun.ts:31 quietRungOfQuery)
pub fn quiet_rung_of_query(g: &mut crate::game::Game, q: crate::Opaque) -> crate::quietfog::QuietRung { todo!() }

/// (src/game/scenarioRun.ts:35 sameFog)
pub fn same_fog(g: &mut crate::game::Game, a: &crate::scenario_run::SameFogA, b: &crate::scenario_run::SameFogB) -> bool { todo!() }

/// (src/game/scenarioRun.ts:39 fogOfQuery)
pub fn fog_of_query(g: &mut crate::game::Game, q: crate::Opaque) -> bool { todo!() }

/// (src/game/scenarioRun.ts:44 fogOfLink)
pub fn fog_of_link(g: &mut crate::game::Game, q: crate::Opaque) -> bool { todo!() }

/// (src/game/scenarioRun.ts:51 startScenario)
pub fn start_scenario(g: &mut crate::game::Game, spec: &crate::scene_scenario_list::ScenarioSpec) -> Option<String> { todo!() }

/// (src/game/scenarioRun.ts:63 startFromQuery)
pub fn start_from_query(g: &mut crate::game::Game, search: &crate::scenario_run::StartFromQuerySearch) -> crate::scenario_run::StartFromQuery { todo!() }

/// (src/game/scenarioRun.ts:80 refusalWords)
pub fn refusal_words(spec: &crate::scene_scenario_list::ScenarioSpec, why: &str) -> String { todo!() }

/// (src/game/scenarioRun.ts:83 activeScenario)
pub fn active_scenario(g: &mut crate::game::Game) -> Option<crate::scenario_run::ActiveScenario> { todo!() }

/// (src/game/scenarioRun.ts:87 teamOf)
pub fn team_of(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/scenarioRun.ts:92 runningScenario)
pub fn running_scenario(g: &mut crate::game::Game, level_file: Option<&str>, list: Option<&[crate::scene_scenarios::ScenarioEntry]>) -> Option<crate::scenario_run::RunningScenario> { todo!() }

/// (src/game/scenarioRun.ts:102 quitScenario)
pub fn quit_scenario(g: &mut crate::game::Game, fog: Option<bool>) -> Option<crate::scenario_run::QuitScenario> { todo!() }
