//! The frames of the tut-road level's script (rust/game_rules/src/missions/script_tut_road.rs): where each beat stands, and the locals that live across a yield.
#![allow(unused_imports, unused_variables, unused_mut)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

///  the beat 'hold': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutRoadHoldFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'cease': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutRoadCeaseFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'road': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutRoadRoadFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// s: a yield separates its declaration from a use
    pub s: Option<crate::director::UnitRef>,
    /// i: a yield separates its declaration from a use
    pub i: f64,
}

///  the beat 'way': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutRoadWayFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'hut': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutRoadHutFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// i: a yield separates its declaration from a use
    pub i: f64,
}

///  the beat 'gun': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutRoadGunFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// k: a yield separates its declaration from a use
    pub k: f64,
    /// dir: a yield separates its declaration from a use
    pub dir: f64,
    /// mode: a yield separates its declaration from a use
    pub mode: Option<String>,
}

///  the beat 'bags': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutRoadBagsFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'ease': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutRoadEaseFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'gate': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutRoadGateFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::script_tut_road::TutRoadGateMen>,
    /// all: a yield separates its declaration from a use
    pub all: f64,
    /// i: a yield separates its declaration from a use
    pub i: f64,
}

///  the beat 'open': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutRoadOpenFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  what the define pass names and a beat reads: the script's and its programs' locals. Its closures are your private fns: aim
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutRoadVars {
    pub hut_door: Option<crate::director::DoorRef>,
    pub gate: Option<crate::director::DoorRef>,
    pub road_east: Option<crate::director::VolumeRef>,
    pub road_nose: Option<crate::director::VolumeRef>,
    pub hut_north: Option<crate::director::VolumeRef>,
    pub field: Option<crate::director::VolumeRef>,
    pub open_east: Option<crate::director::VolumeRef>,
    pub staff: Vec<String>,
    pub last: Vec<String>,
    pub aims: Option<JsMap<String, V3>>,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutRoadGateMen {
    pub k: f64,
    pub u: Option<crate::director::UnitRef>,
}

