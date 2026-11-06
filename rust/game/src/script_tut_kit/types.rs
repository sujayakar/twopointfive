//! The frames of levels/tut-kit.script.ts, written by the skeleton's program from the checker. Never edited by hand: a local a body wants across a yield that is not here is a question for the program.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-kit.script.ts:71-78) the beat 'hold': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitHoldFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-kit.script.ts:81-101) the beat 'gun': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitGunFrame {
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

/// (levels/tut-kit.script.ts:102-102) the beat 'bags': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitBagsFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-kit.script.ts:103-103) the beat 'gap': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitGapFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-kit.script.ts:104-104) the beat 'blind': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitBlindFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-kit.script.ts:108-114) the beat 'shop': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitShopFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// r: a yield separates its declaration from a use
    pub r: Option<crate::director::CondResult>,
}

/// (levels/tut-kit.script.ts:118-126) the beat 'room': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitRoomFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// pair: a yield separates its declaration from a use
    pub pair: Vec<crate::director::UnitRef>,
    /// r: a yield separates its declaration from a use
    pub r: Option<crate::director::CondResult>,
    /// r2: a yield separates its declaration from a use
    pub r2: Option<crate::director::CondResult>,
}

/// (levels/tut-kit.script.ts:131-131) the beat 'loose': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitLooseFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-kit.script.ts:132-150) the beat 'trucks': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitTrucksFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// two: a yield separates its declaration from a use
    pub two: Vec<crate::script_tut_kit::TutKitTrucksTwo>,
    /// all: a yield separates its declaration from a use
    pub all: f64,
    /// m: a yield separates its declaration from a use
    pub m: Option<crate::script_tut_kit::TutKitTrucksM>,
    /// i: a yield separates its declaration from a use
    pub i: f64,
    /// shots: a yield separates its declaration from a use
    pub shots: Vec<crate::director::Cond>,
    /// m: a yield separates its declaration from a use
    pub m_2: Option<crate::script_tut_kit::TutKitTrucksM>,
}

/// (levels/tut-kit.script.ts:153-153) the beat 'call': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitCallFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-kit.script.ts:154-154) the beat 'box': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitBoxFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-kit.script.ts:156-156) the beat 'dark': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitDarkFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-kit.script.ts:157-157) the beat 'bunk': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitBunkFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-kit.script.ts:158-158) the beat 'wall': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitWallFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-kit.script.ts:160-160) the beat 'bunkroom': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitBunkroomFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* room: yield* room: the helper's frame while it runs. Step it until it answers None
    pub sub_room: Option<alloc::boxed::Box<crate::script_tut_kit::TutKitRoomHelperFrame>>,
}

/// (levels/tut-kit.script.ts:161-161) the beat 'fuelroom': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitFuelroomFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* room: yield* room: the helper's frame while it runs. Step it until it answers None
    pub sub_room: Option<alloc::boxed::Box<crate::script_tut_kit::TutKitRoomHelperFrame>>,
}

/// (levels/tut-kit.script.ts:166-166) the beat 'free': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitFreeFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-kit.script.ts:167-170) the beat 'seen': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitSeenFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-kit.script.ts:171-185) the beat 'yard': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitYardFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// two: a yield separates its declaration from a use
    pub two: Vec<crate::script_tut_kit::TutKitYardTwo>,
}

/// (levels/tut-kit.script.ts:55-63) the helper generator room: where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitRoomHelperFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// names: a parameter
    pub names: Vec<String>,
    /// inside: a parameter
    pub inside: Option<crate::director::VolumeRef>,
    /// box: a parameter
    pub box_: Option<crate::script_tut_kit::TutKitRoomHelperBox>,
    /// held: a yield separates its declaration from a use
    pub held: Vec<crate::director::UnitRef>,
    /// r: a yield separates its declaration from a use
    pub r: Option<crate::director::CondResult>,
}

/// (levels/tut-kit.script.ts) what the define pass names and a beat reads: the script's and its programs' locals. Its closures are your private fns: aim, men, radio
#[derive(Clone, Debug, Default)]
pub struct TutKitVars {
    pub back_door: Option<crate::director::DoorRef>,
    pub wash_door: Option<crate::director::DoorRef>,
    pub shop_line: Option<crate::director::VolumeRef>,
    pub shop_in: Option<crate::director::VolumeRef>,
    pub store_in: Option<crate::director::VolumeRef>,
    pub bunk_in: Option<crate::director::VolumeRef>,
    pub fuel_in: Option<crate::director::VolumeRef>,
    pub wash_in: Option<crate::director::VolumeRef>,
    pub gun: Option<crate::director::LoadoutRef>,
    pub rifleman: Option<crate::director::LoadoutRef>,
    pub torch: Option<crate::director::LoadoutRef>,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutKitRoomHelperBox {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutKitTrucksTwo {
    pub k: f64,
    pub u: Option<crate::director::UnitRef>,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutKitTrucksM {
    pub k: f64,
    pub u: Option<crate::director::UnitRef>,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutKitYardTwo {
    pub k: f64,
    pub u: Option<crate::director::UnitRef>,
}
