//! The types of `scene_scenario_list`.
use alloc::string::String;

#[derive(Clone, Debug, Default)]
pub struct ScenarioSpec {
    pub kind: crate::scene_scenarios::MissionKind,
    pub squad: f64,
    pub level: Option<String>,
    pub seed: Option<f64>,
}

