//! The types of `scenario_run`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum SameFogA {
    Text(String),
    URLSearchParams(crate::Opaque),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum SameFogB {
    Text(String),
    URLSearchParams(crate::Opaque),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum StartFromQuerySearch {
    Text(String),
    URLSearchParams(crate::Opaque),
}

/// (TypeScript: inline at src/game/scenarioRun.ts:63 startFromQuery ->, src/game/scenarioRun.ts:63)
#[derive(Clone, Debug)]
pub struct StartFromQuery {
    pub spec: Option<crate::scene_scenario_list::ScenarioSpec>,
    pub why: Option<String>,
}
impl StartFromQuery {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StartFromQuery {
            spec: None,
            why: None,
        }
    }
}
impl Default for StartFromQuery { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
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

/// (TypeScript: inline at src/game/scenarioRun.ts:92 runningScenario ->, src/game/scenarioRun.ts:92)
#[derive(Clone, Debug)]
pub struct RunningScenario {
    pub kind: crate::scenario_run::RunningScenarioKind,
    pub squad: f64,
    pub entry: Option<crate::scene_scenarios::ScenarioEntry>,
    pub title: String,
    pub team: String,
    pub saves: Option<bool>,
}
impl RunningScenario {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RunningScenario {
            kind: crate::scenario_run::RunningScenarioKind::Nightbreach,
            squad: 0.0,
            entry: None,
            title: String::new(),
            team: String::new(),
            saves: None,
        }
    }
}
impl Default for RunningScenario { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RunningScenarioKind {
    Nightbreach,
    Roomclear,
    Rescue,
    Quiet,
}
impl RunningScenarioKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            RunningScenarioKind::Nightbreach => "nightbreach",
            RunningScenarioKind::Roomclear => "roomclear",
            RunningScenarioKind::Rescue => "rescue",
            RunningScenarioKind::Quiet => "quiet",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum QuitScenario {
    Nightbreach,
    Roomclear,
    Rescue,
    Quiet,
}
impl QuitScenario {
    pub const fn as_str(self) -> &'static str {
        match self {
            QuitScenario::Nightbreach => "nightbreach",
            QuitScenario::Roomclear => "roomclear",
            QuitScenario::Rescue => "rescue",
            QuitScenario::Quiet => "quiet",
        }
    }
}

/// (src/game/scenarioRun.ts:18 WORDS: a table nothing writes)
pub fn words() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("nightbreach"), String::from("night breach")); t.set(String::from("roomclear"), String::from("room clear")); t.set(String::from("rescue"), String::from("hostage rescue")); t.set(String::from("quiet"), String::from("quiet mission")); t } }
