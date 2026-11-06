//! The frames of levels/tut-solo.script.ts, written by the skeleton's program from the checker. Never edited by hand: a local a body wants across a yield that is not here is a question for the program.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-solo.script.ts:52-55) the beat 'hold': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloHoldFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-solo.script.ts:59-59) the beat 'cease': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloCeaseFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-solo.script.ts:60-73) the beat 'road': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloRoadFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// s: a yield separates its declaration from a use
    pub s: Option<crate::director::UnitRef>,
    /// i: a yield separates its declaration from a use
    pub i: f64,
    /// at: a yield separates its declaration from a use
    pub at_2: [f64; 3],
}

/// (levels/tut-solo.script.ts:77-77) the beat 'way': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloWayFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-solo.script.ts:80-100) the beat 'gun': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloGunFrame {
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
    /// at: a yield separates its declaration from a use
    pub at_3: [f64; 3],
    /// pts: a yield separates its declaration from a use
    pub pts: Vec<[f64; 3]>,
    /// at: a yield separates its declaration from a use
    pub at_4: [f64; 3],
}

/// (levels/tut-solo.script.ts:101-101) the beat 'bags': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloBagsFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-solo.script.ts:102-102) the beat 'gap': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloGapFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-solo.script.ts:103-103) the beat 'blind': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloBlindFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-solo.script.ts:107-113) the beat 'shop': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloShopFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// r: a yield separates its declaration from a use
    pub r: Option<crate::director::CondResult>,
}

/// (levels/tut-solo.script.ts:118-126) the beat 'room': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloRoomFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::director::UnitRef>,
    /// r: a yield separates its declaration from a use
    pub r: Option<crate::director::CondResult>,
    /// r2: a yield separates its declaration from a use
    pub r2: Option<crate::director::CondResult>,
}

/// (levels/tut-solo.script.ts:131-131) the beat 'loose': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloLooseFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-solo.script.ts:132-150) the beat 'trucks': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloTrucksFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::script_tut_solo::TutSoloTrucksMen>,
    /// all: a yield separates its declaration from a use
    pub all: f64,
    /// m: a yield separates its declaration from a use
    pub m: Option<crate::script_tut_solo::TutSoloTrucksM>,
    /// i: a yield separates its declaration from a use
    pub i: f64,
    /// shots: a yield separates its declaration from a use
    pub shots: Vec<crate::director::Cond>,
    /// m: a yield separates its declaration from a use
    pub m_2: Option<crate::script_tut_solo::TutSoloTrucksM>,
}

/// (levels/tut-solo.script.ts) what the define pass names and a beat reads: the script's and its programs' locals. Its closures are your private fns: aim, pair
#[derive(Clone, Debug, Default)]
pub struct TutSoloVars {
    pub back_door: Option<crate::director::DoorRef>,
    pub inner_door: Option<crate::director::DoorRef>,
    pub shop_line: Option<crate::director::VolumeRef>,
    pub shop_in: Option<crate::director::VolumeRef>,
    pub store_in: Option<crate::director::VolumeRef>,
    pub road_east: Option<crate::director::VolumeRef>,
    pub road_nose: Option<crate::director::VolumeRef>,
    pub rifleman: Option<crate::director::LoadoutRef>,
    pub staff: Vec<String>,
    pub aims: Option<JsMap<String, V3>>,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutSoloTrucksMen {
    pub k: f64,
    pub u: Option<crate::director::UnitRef>,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutSoloTrucksM {
    pub k: f64,
    pub u: Option<crate::director::UnitRef>,
}
