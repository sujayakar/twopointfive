//! The frames of the tut-team level's script (rust/game_rules/src/missions/script_tut_team.rs): where each beat stands, and the locals that live across a yield.
#![allow(unused_imports, unused_variables, unused_mut)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

///  the beat 'hold': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamHoldFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// held: a yield separates its declaration from a use
    pub held: Vec<crate::director::UnitRef>,
}

///  the beat 'cook': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamCookFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
}

///  the beat 'ambush': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamAmbushFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* rake: yield* rake: the helper's frame while it runs. Step it until it answers None
    pub sub_rake: Option<alloc::boxed::Box<crate::script_tut_team::TutTeamRakeHelperFrame>>,
}

///  the beat 'pinned': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamPinnedFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// k: a yield separates its declaration from a use
    pub k: f64,
}

///  the beat 'smoke': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamSmokeFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* smoke: yield* smoke: the helper's frame while it runs. Step it until it answers None
    pub sub_smoke: Option<alloc::boxed::Box<crate::script_tut_team::TutTeamSmokeHelperFrame>>,
}

///  the beat 'break': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamBreakFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'ordered': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamOrderedFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'blind': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamBlindFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* harass: yield* harass: the helper's frame while it runs. Step it until it answers None
    pub sub_harass: Option<alloc::boxed::Box<crate::script_tut_team::TutTeamHarassHelperFrame>>,
}

///  the beat 'linger': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamLingerFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* harass: yield* harass: the helper's frame while it runs. Step it until it answers None
    pub sub_harass: Option<alloc::boxed::Box<crate::script_tut_team::TutTeamHarassHelperFrame>>,
}

///  the beat 'push': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamPushFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// k: a yield separates its declaration from a use
    pub k: f64,
    /// left: a yield separates its declaration from a use
    pub left: Vec<crate::director::UnitRef>,
    /// i: a yield separates its declaration from a use
    pub i: f64,
}

///  the beat 'indoors': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamIndoorsFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'house': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamHouseFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'out': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamOutFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the helper generator rake: where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamRakeHelperFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// zones: a parameter
    pub zones: Vec<crate::script_tut_team::TutTeamZone>,
    /// stop: a closure parameter: which of the script's own functions the caller means. Your private fn matches on it
    pub stop: u16,
    /// o: a parameter
    pub o: Option<crate::script_tut_team::TutTeamRakeHelperO>,
    /// total: a yield separates its declaration from a use
    pub total: f64,
    /// Z: a yield separates its declaration from a use
    pub z: Option<crate::script_tut_team::TutTeamZone>,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// heavy: a yield separates its declaration from a use
    pub heavy: bool,
    /// left: a yield separates its declaration from a use
    pub left: f64,
}

///  the helper generator harass: where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamHarassHelperFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// marks: a parameter
    pub marks: Vec<[f64; 3]>,
    /// stop: a closure parameter: which of the script's own functions the caller means. Your private fn matches on it
    pub stop: u16,
    /// o: a parameter
    pub o: Option<crate::script_tut_team::TutTeamHarassHelperO>,
}

///  the helper generator smoke, inside the beat 'smoke': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamSmokeHelperFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// again: a parameter
    pub again: bool,
    /// by: a yield separates its declaration from a use
    pub by: Vec<Option<crate::director::UnitRef>>,
}

///  what the define pass names and a beat reads: the script's and its programs' locals. Its closures are your private fns: block, squad, inBunk, inKitchen, inYard, pick
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamVars {
    pub rifle: Option<crate::director::LoadoutRef>,
    pub yaw: f64,
    pub pass_door: Option<crate::director::DoorRef>,
    pub kitchen_door: Option<crate::director::DoorRef>,
    pub bunk_door: Option<crate::director::DoorRef>,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamZone {
    pub lo: [f64; 3],
    pub hi: [f64; 3],
    pub w: f64,
    pub by: Option<Vec<String>>,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamRakeHelperO {
    pub rounds: [f64; 2],
    pub gap: [f64; 2],
    pub spread: f64,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutTeamHarassHelperO {
    pub rounds: [f64; 2],
    pub gap: [f64; 2],
    pub spread: f64,
}

