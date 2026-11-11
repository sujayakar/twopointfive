//! The frames of the tut-queue level's script (rust/game_rules/src/missions/script_tut_queue.rs): where each beat stands, and the locals that live across a yield.
#![allow(unused_imports, unused_variables, unused_mut)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

///  the beat 'gun': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutQueueGunFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// m: a yield separates its declaration from a use
    pub m: Option<crate::director::UnitRef>,
    /// n: a yield separates its declaration from a use
    pub n: Option<crate::director::UnitRef>,
    /// gone: a yield separates its declaration from a use
    pub gone: Option<crate::director::Cond>,
    /// hit: a yield separates its declaration from a use
    pub hit: Option<crate::director::Cond>,
    /// r: a yield separates its declaration from a use
    pub r: Option<crate::director::CondResult>,
    /// alone: a yield separates its declaration from a use
    pub alone: bool,
    /// k: a yield separates its declaration from a use
    pub k: f64,
}

///  the beat 'gates': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutQueueGatesFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// road: a yield separates its declaration from a use
    pub road: Vec<Option<crate::director::UnitRef>>,
    /// lane: a yield separates its declaration from a use
    pub lane: Vec<Option<crate::director::UnitRef>>,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::director::UnitRef>,
    /// off: a yield separates its declaration from a use
    pub off: bool,
}

///  what the define pass names and a beat reads: the script's and its programs' locals. Its closures are your private fns: alive, isOpen
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutQueueVars {
    pub front: Option<crate::director::DoorRef>,
    pub back: Option<crate::director::DoorRef>,
    pub road_gate: Option<crate::director::DoorRef>,
    pub back_gate: Option<crate::director::DoorRef>,
    pub track_gate: Option<crate::director::DoorRef>,
    pub gun_mark: Option<crate::director::MarkRef>,
    pub mate_mark: Option<crate::director::MarkRef>,
    pub jamb: Option<crate::director::MarkRef>,
    pub doorway_at: [f64; 3],
    pub doorway: Option<crate::director::MarkRef>,
    pub road_camp: Vec<crate::director::MarkRef>,
    pub lane_camp: Vec<crate::director::MarkRef>,
    pub road_in: Vec<crate::director::MarkRef>,
    pub lane_in: Vec<crate::director::MarkRef>,
    pub post_a: Option<crate::director::MarkRef>,
    pub post_b: Option<crate::director::MarkRef>,
    pub road_on: Vec<crate::director::MarkRef>,
    pub lane_on: Vec<crate::director::MarkRef>,
}

