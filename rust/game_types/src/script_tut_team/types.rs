//! The frames of levels/tut-team.script.ts, written by the skeleton's program from the checker. Never edited by hand: a local a body wants across a yield that is not here is a question for the program.
#![allow(unused_imports, unused_variables, unused_mut)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-team.script.ts:111-119) the beat 'hold': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutTeamHoldFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// held: a yield separates its declaration from a use
    pub held: Vec<crate::director::UnitRef>,
}

/// (levels/tut-team.script.ts:124-132) the beat 'cook': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutTeamCookFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
}

/// (levels/tut-team.script.ts:136-143) the beat 'ambush': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutTeamAmbushFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* rake: yield* rake: the helper's frame while it runs. Step it until it answers None
    pub sub_rake: Option<alloc::boxed::Box<crate::script_tut_team::TutTeamRakeHelperFrame>>,
}

/// (levels/tut-team.script.ts:144-149) the beat 'pinned': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutTeamPinnedFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// k: a yield separates its declaration from a use
    pub k: f64,
    /// who: a yield separates its declaration from a use
    pub who: String,
    /// line: a yield separates its declaration from a use
    pub line: String,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
}

/// (levels/tut-team.script.ts:154-168) the beat 'smoke': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutTeamSmokeFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* smoke: yield* smoke: the helper's frame while it runs. Step it until it answers None
    pub sub_smoke: Option<alloc::boxed::Box<crate::script_tut_team::TutTeamSmokeHelperFrame>>,
}

/// (levels/tut-team.script.ts:169-175) the beat 'break': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutTeamBreakFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-team.script.ts:176-176) the beat 'ordered': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutTeamOrderedFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-team.script.ts:178-178) the beat 'blind': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutTeamBlindFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* harass: yield* harass: the helper's frame while it runs. Step it until it answers None
    pub sub_harass: Option<alloc::boxed::Box<crate::script_tut_team::TutTeamHarassHelperFrame>>,
}

/// (levels/tut-team.script.ts:181-185) the beat 'linger': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutTeamLingerFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* harass: yield* harass: the helper's frame while it runs. Step it until it answers None
    pub sub_harass: Option<alloc::boxed::Box<crate::script_tut_team::TutTeamHarassHelperFrame>>,
}

/// (levels/tut-team.script.ts:190-205) the beat 'push': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutTeamPushFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// k: a yield separates its declaration from a use
    pub k: f64,
    /// left: a yield separates its declaration from a use
    pub left: Vec<crate::director::UnitRef>,
    /// i: a yield separates its declaration from a use
    pub i: f64,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u_2: Option<crate::director::UnitRef>,
}

/// (levels/tut-team.script.ts:208-213) the beat 'indoors': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutTeamIndoorsFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-team.script.ts:216-220) the beat 'house': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutTeamHouseFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-team.script.ts:221-225) the beat 'out': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutTeamOutFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-team.script.ts:73-89) the helper generator rake: where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
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
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::director::UnitRef>,
    /// r: a yield separates its declaration from a use
    pub r: f64,
    /// Z: a yield separates its declaration from a use
    pub z: Option<crate::script_tut_team::TutTeamZone>,
    /// z: a yield separates its declaration from a use
    pub z_2: Option<crate::script_tut_team::TutTeamZone>,
    /// can: a yield separates its declaration from a use
    pub can: Vec<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// heavy: a yield separates its declaration from a use
    pub heavy: bool,
    /// left: a yield separates its declaration from a use
    pub left: f64,
    /// at: a yield separates its declaration from a use
    pub at_2: [f64; 3],
    /// n: a yield separates its declaration from a use
    pub n: f64,
}

/// (levels/tut-team.script.ts:91-101) the helper generator harass: where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutTeamHarassHelperFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// marks: a parameter
    pub marks: Vec<[f64; 3]>,
    /// stop: a closure parameter: which of the script's own functions the caller means. Your private fn matches on it
    pub stop: u16,
    /// o: a parameter
    pub o: Option<crate::script_tut_team::TutTeamHarassHelperO>,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::director::UnitRef>,
    /// at: a yield separates its declaration from a use
    pub at_2: [f64; 3],
    /// north: a yield separates its declaration from a use
    pub north: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// heavy: a yield separates its declaration from a use
    pub heavy: bool,
    /// n: a yield separates its declaration from a use
    pub n: f64,
}

/// (levels/tut-team.script.ts:156-161) the helper generator smoke, inside the beat 'smoke': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutTeamSmokeHelperFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// again: a parameter
    pub again: bool,
    /// by: a yield separates its declaration from a use
    pub by: Vec<Option<crate::director::UnitRef>>,
}

/// (levels/tut-team.script.ts) what the define pass names and a beat reads: the script's and its programs' locals. Its closures are your private fns: block, squad, inBunk, inKitchen, inYard, pick
#[derive(Clone, Debug, Default)]
pub struct TutTeamVars {
    pub rifle: Option<crate::director::LoadoutRef>,
    pub yaw: f64,
    pub pass_door: Option<crate::director::DoorRef>,
    pub kitchen_door: Option<crate::director::DoorRef>,
    pub bunk_door: Option<crate::director::DoorRef>,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutTeamZone {
    pub lo: [f64; 3],
    pub hi: [f64; 3],
    pub w: f64,
    pub by: Option<Vec<String>>,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutTeamRakeHelperO {
    pub rounds: [f64; 2],
    pub gap: [f64; 2],
    pub spread: f64,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutTeamHarassHelperO {
    pub rounds: [f64; 2],
    pub gap: [f64; 2],
    pub spread: f64,
}

