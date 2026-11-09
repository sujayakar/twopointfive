//! The types of `roomclear`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Placement {
    pub room: String,
    pub mark: f64,
    pub pos: V3,
    pub aim_at: V3,
    pub name: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
pub enum RoomClearEnd {
    Taken,
    Wiped,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RoomState {
    pub def: crate::scene_scenarios::StructureRoom,
    pub defenders: Vec<GuardId>,
    pub cleared: bool,
    pub cleared_at: f64,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct RoomClearScenario {
    pub end: Option<crate::roomclear::RoomClearEnd>,
    #[is(f64::NAN)] pub end_at: f64,
    #[is(String::from(""))] pub end_why: String,
    #[is(String::from("team wiped"))] pub end_say: String,
    pub placement: Vec<crate::roomclear::Placement>,
    pub seed: f64,
    pub attempt: f64,
    pub pair_said: bool,
    pub entry_said: bool,
    pub team_said: bool,
    #[is(crate::nightbreach::NightBreachLayout { name: String::from("room clear") })] pub layout: crate::nightbreach::NightBreachLayout,
}

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

#[derive(Clone, Debug, Default)]
pub struct RoomClearScenarioObjective {
    pub text: String,
    pub pos: Option<V3>,
}

pub const MARK_CLEAR_M: f64 = 0.45;

