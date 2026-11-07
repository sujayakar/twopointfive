//! The frames of levels/tut-squad.script.ts, written by the skeleton's program from the checker. Never edited by hand: a local a body wants across a yield that is not here is a question for the program.
#![allow(unused_imports, unused_variables, unused_mut)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-squad.script.ts:75-134) the beat 'picket': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSquadPicketFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// g: a yield separates its declaration from a use
    pub g: Option<crate::director::DoorRef>,
    /// front: a yield separates its declaration from a use
    pub front: Option<crate::director::UnitRef>,
    /// g: a yield separates its declaration from a use
    pub g_2: Option<crate::director::DoorRef>,
    /// lane: a yield separates its declaration from a use
    pub lane: Vec<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u_2: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u_3: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u_4: Option<crate::director::UnitRef>,
    /// off: kept by the closures of lastMan (up): written by one call and read by the next across yields
    pub off: bool,
}

/// (levels/tut-squad.script.ts:143-149) the beat 'store': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSquadStoreFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::director::UnitRef>,
}

/// (levels/tut-squad.script.ts:161-195) the beat 'shop': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSquadShopFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// m: a yield separates its declaration from a use
    pub m: Option<crate::director::UnitRef>,
    /// since: a yield separates its declaration from a use
    pub since: f64,
    /// end: a yield separates its declaration from a use
    pub end: f64,
    /// said: a yield separates its declaration from a use
    pub said: bool,
    /// k: a yield separates its declaration from a use
    pub k: f64,
    /// b: a yield separates its declaration from a use
    pub b: f64,
}

/// (levels/tut-squad.script.ts:205-235) the beat 'hall': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSquadHallFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u_2: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u_3: Option<crate::director::UnitRef>,
}

/// (levels/tut-squad.script.ts:253-300) the beat 'gun': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSquadGunFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// m: a yield separates its declaration from a use
    pub m: Option<crate::director::UnitRef>,
    /// n: a yield separates its declaration from a use
    pub n: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u_2: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u_3: Option<crate::director::UnitRef>,
}

/// (levels/tut-squad.script.ts) what the define pass names and a beat reads: the script's and its programs' locals. Its closures are your private fns: alive, open
#[derive(Clone, Debug, Default)]
pub struct TutSquadVars {
    pub store_door: Option<crate::director::DoorRef>,
    pub shop_door: Option<crate::director::DoorRef>,
    pub side_door: Option<crate::director::DoorRef>,
    pub hall_door: Option<crate::director::DoorRef>,
    pub road_gate: Option<crate::director::DoorRef>,
    pub back_gate: Option<crate::director::DoorRef>,
    pub track_gate: Option<crate::director::DoorRef>,
    pub shop_fire: Option<crate::director::MarkRef>,
    pub shop_aim: [f64; 3],
    pub in_store: [f64; 3],
    pub in_hall: [f64; 3],
    pub hall_window: [f64; 3],
    pub shop_lane: Option<crate::director::VolumeRef>,
    pub shop_line: Option<crate::director::VolumeRef>,
    pub shop_in: Option<crate::director::VolumeRef>,
    pub hall_post: Vec<crate::director::MarkRef>,
    pub hall_back: Vec<crate::director::MarkRef>,
    pub lane: Option<crate::director::MarkRef>,
    pub camp: Vec<crate::director::MarkRef>,
    pub gate_post: Option<crate::director::MarkRef>,
    pub bags: Option<crate::director::MarkRef>,
    pub round: Vec<crate::director::MarkRef>,
    pub open: Option<crate::director::MarkRef>,
    pub out: Vec<crate::director::MarkRef>,
}

