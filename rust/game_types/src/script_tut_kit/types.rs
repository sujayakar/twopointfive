//! The frames of levels/tut-kit.script.ts, written by the skeleton's program from the checker. Never edited by hand: a local a body wants across a yield that is not here is a question for the program.
#![allow(unused_imports, unused_variables, unused_mut)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

///  the beat 'hold': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitHoldFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'gun': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
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
}

///  the beat 'bags': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitBagsFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'gap': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitGapFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'blind': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitBlindFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'shop': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitShopFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
}

///  the beat 'room': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitRoomFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// pair: a yield separates its declaration from a use
    pub pair: Vec<crate::director::UnitRef>,
}

///  the beat 'loose': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitLooseFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'trucks': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitTrucksFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// two: a yield separates its declaration from a use
    pub two: Vec<crate::script_tut_kit::TutKitTrucksTwo>,
    /// all: a yield separates its declaration from a use
    pub all: f64,
    /// i: a yield separates its declaration from a use
    pub i: f64,
}

///  the beat 'call': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitCallFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'box': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitBoxFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'dark': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitDarkFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'bunk': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitBunkFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'wall': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitWallFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'bunkroom': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitBunkroomFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* room: yield* room: the helper's frame while it runs. Step it until it answers None
    pub sub_room: Option<alloc::boxed::Box<crate::script_tut_kit::TutKitRoomHelperFrame>>,
}

///  the beat 'fuelroom': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitFuelroomFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* room: yield* room: the helper's frame while it runs. Step it until it answers None
    pub sub_room: Option<alloc::boxed::Box<crate::script_tut_kit::TutKitRoomHelperFrame>>,
}

///  the beat 'free': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitFreeFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'seen': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitSeenFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'yard': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitYardFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// two: a yield separates its declaration from a use
    pub two: Vec<crate::script_tut_kit::TutKitYardTwo>,
}

///  the helper generator room: where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
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
}

///  what the define pass names and a beat reads: the script's and its programs' locals. Its closures are your private fns: aim, men, radio
#[derive(Clone, Debug, Default, ToBytes)]
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
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitRoomHelperBox {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitTrucksTwo {
    pub k: f64,
    pub u: Option<crate::director::UnitRef>,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutKitYardTwo {
    pub k: f64,
    pub u: Option<crate::director::UnitRef>,
}

