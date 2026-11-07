//! The frames of levels/tut-road.script.ts, written by the skeleton's program from the checker. Never edited by hand: a local a body wants across a yield that is not here is a question for the program.
#![allow(unused_imports, unused_variables, unused_mut)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-road.script.ts:54-57) the beat 'hold': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutRoadHoldFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-road.script.ts:61-61) the beat 'cease': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutRoadCeaseFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-road.script.ts:62-77) the beat 'road': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutRoadRoadFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// s: a yield separates its declaration from a use
    pub s: Option<crate::director::UnitRef>,
    /// i: a yield separates its declaration from a use
    pub i: f64,
    /// at: a yield separates its declaration from a use
    pub at_2: [f64; 3],
}

/// (levels/tut-road.script.ts:81-81) the beat 'way': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutRoadWayFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-road.script.ts:86-99) the beat 'hut': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutRoadHutFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// i: a yield separates its declaration from a use
    pub i: f64,
    /// at: a yield separates its declaration from a use
    pub at_2: [f64; 3],
}

/// (levels/tut-road.script.ts:104-123) the beat 'gun': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutRoadGunFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// k: a yield separates its declaration from a use
    pub k: f64,
    /// dir: a yield separates its declaration from a use
    pub dir: f64,
    /// i: a yield separates its declaration from a use
    pub i: f64,
    /// mode: a yield separates its declaration from a use
    pub mode: Option<String>,
    /// at: a yield separates its declaration from a use
    pub at_2: [f64; 3],
    /// pts: a yield separates its declaration from a use
    pub pts: Vec<[f64; 3]>,
    /// at: a yield separates its declaration from a use
    pub at_3: [f64; 3],
}

/// (levels/tut-road.script.ts:124-124) the beat 'bags': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutRoadBagsFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-road.script.ts:125-125) the beat 'ease': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutRoadEaseFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-road.script.ts:130-148) the beat 'gate': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutRoadGateFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::script_tut_road::TutRoadGateMen>,
    /// all: a yield separates its declaration from a use
    pub all: f64,
    /// m: a yield separates its declaration from a use
    pub m: Option<crate::script_tut_road::TutRoadGateM>,
    /// i: a yield separates its declaration from a use
    pub i: f64,
    /// shots: a yield separates its declaration from a use
    pub shots: Vec<crate::director::Cond>,
    /// m: a yield separates its declaration from a use
    pub m_2: Option<crate::script_tut_road::TutRoadGateM>,
}

/// (levels/tut-road.script.ts:152-152) the beat 'open': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutRoadOpenFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-road.script.ts) what the define pass names and a beat reads: the script's and its programs' locals. Its closures are your private fns: aim
#[derive(Clone, Debug, Default)]
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
#[derive(Clone, Debug, Default)]
pub struct TutRoadGateMen {
    pub k: f64,
    pub u: Option<crate::director::UnitRef>,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutRoadGateM {
    pub k: f64,
    pub u: Option<crate::director::UnitRef>,
}

