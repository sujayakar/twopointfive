//! The types of `scene_scenario_list`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct ScenarioSpec {
    pub kind: crate::scene_scenario_list::ScenarioSpecKind,
    pub squad: f64,
    pub level: Option<String>,
    pub seed: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum ScenarioSpecKind {
    #[default] Nightbreach,
    Roomclear,
    Rescue,
    Quiet,
}
impl ScenarioSpecKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            ScenarioSpecKind::Nightbreach => "nightbreach",
            ScenarioSpecKind::Roomclear => "roomclear",
            ScenarioSpecKind::Rescue => "rescue",
            ScenarioSpecKind::Quiet => "quiet",
        }
    }
}
