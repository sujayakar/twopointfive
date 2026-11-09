//! The types of `scene_scenario_list`.
#![allow(unused_imports)]
use crate::{Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct ScenarioSpec {
    pub kind: crate::scene_scenario_list::ScenarioSpecKind,
    pub squad: f64,
    pub level: Option<String>,
    pub seed: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Words)]
pub enum ScenarioSpecKind {
    #[default] Nightbreach,
    Roomclear,
    Rescue,
    Quiet,
}
