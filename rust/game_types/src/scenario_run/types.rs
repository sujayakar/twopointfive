//! The types of `scenario_run`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug)]
pub enum StartFromQuerySearch {
    Text(String),
    URLSearchParams(crate::query::Query),
}

#[derive(Clone, Debug, Default)]
pub struct StartFromQuery {
    pub spec: Option<crate::scene_scenario_list::ScenarioSpec>,
    pub why: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ActiveScenario {
    Nightbreach,
    Roomclear,
    Rescue,
    Quiet,
}
impl ActiveScenario {
    pub const fn as_str(self) -> &'static str {
        match self {
            ActiveScenario::Nightbreach => "nightbreach",
            ActiveScenario::Roomclear => "roomclear",
            ActiveScenario::Rescue => "rescue",
            ActiveScenario::Quiet => "quiet",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct RunningScenario {
    pub kind: crate::scenario_run::RunningScenarioKind,
    pub squad: f64,
    pub entry: Option<crate::scene_scenarios::ScenarioEntry>,
    pub title: String,
    pub team: String,
    pub saves: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum RunningScenarioKind {
    #[default] Nightbreach,
    Roomclear,
    Rescue,
    Quiet,
}

/// (src/game/scenarioRun.ts:18 WORDS: a table nothing writes)
pub fn words() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("nightbreach"), String::from("night breach")); t.set(String::from("roomclear"), String::from("room clear")); t.set(String::from("rescue"), String::from("hostage rescue")); t.set(String::from("quiet"), String::from("quiet mission")); t } }
