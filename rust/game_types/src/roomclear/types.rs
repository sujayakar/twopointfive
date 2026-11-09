//! The types of `roomclear`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Placement {
    pub room: String,
    pub mark: f64,
    pub pos: V3,
    pub aim_at: V3,
    pub name: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum RoomClearEnd {
    Taken,
    Wiped,
}
impl RoomClearEnd {
    pub const fn as_str(self) -> &'static str {
        match self {
            RoomClearEnd::Taken => "taken",
            RoomClearEnd::Wiped => "wiped",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RoomState {
    pub def: crate::scene_scenarios::StructureRoom,
    pub defenders: Vec<GuardId>,
    pub cleared: bool,
    pub cleared_at: f64,
}

#[derive(Clone, Debug, ToBytes)]
pub struct RoomClearScenario {
    pub rooms: Vec<crate::roomclear::RoomState>,
    pub end: Option<crate::roomclear::RoomClearEnd>,
    pub end_at: f64,
    pub end_why: String,
    pub end_say: String,
    pub placement: Vec<crate::roomclear::Placement>,
    pub seed: f64,
    pub attempt: f64,
    pub fresh: f64,
    pub way: f64,
    pub pair_said: bool,
    pub entry_said: bool,
    pub team_said: bool,
    pub clear_due: Vec<crate::scene_scenarios::StructureRoom>,
    pub clear_at: f64,
    pub layout: crate::nightbreach::NightBreachLayout,
}
impl RoomClearScenario {
    /// what a new one holds
    pub fn blank() -> Self {
        RoomClearScenario {
            rooms: Vec::new(),
            end: None,
            end_at: f64::NAN,
            end_why: String::from(""),
            end_say: String::from("team wiped"),
            placement: Vec::new(),
            seed: 0.0,
            attempt: 0.0,
            fresh: 0.0,
            way: 0.0,
            pair_said: false,
            entry_said: false,
            team_said: false,
            clear_due: Vec::new(),
            clear_at: f64::NAN,
            layout: crate::nightbreach::NightBreachLayout { name: String::from("room clear") },
        }
    }
}
impl Default for RoomClearScenario { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug, Default)]
pub struct PlaceDefendersS {
    pub rooms: Vec<crate::scene_scenarios::StructureRoom>,
}

#[derive(Clone, Debug, Default)]
pub struct CheckStructureS {
    pub rooms: Vec<crate::scene_scenarios::StructureRoom>,
    pub team: Option<crate::scene_scenarios::TeamStart>,
    pub starts: Option<Vec<crate::scene_scenarios::TeamStart>>,
    pub max_men: Option<f64>,
    pub squads: Option<Vec<crate::scene_scenarios::SquadData>>,
}

#[derive(Clone, Debug, Default)]
pub struct CheckSquadsS {
    pub rooms: Vec<crate::scene_scenarios::StructureRoom>,
    pub max_men: Option<f64>,
    pub squads: Option<Vec<crate::scene_scenarios::SquadData>>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PropGapP {
    pub home: [f64; 3],
    pub hx: f64,
    pub hz: f64,
}

#[derive(Clone, Debug, Default)]
pub struct RoomClearScenarioObjective {
    pub text: String,
    pub pos: Option<V3>,
}

pub const MARK_CLEAR_M: f64 = 0.45;

pub const CLEAR_SAY_SECS: f64 = 0.6;
