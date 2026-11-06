//! The frames of levels/tut-sky.script.ts, written by the skeleton's program from the checker. Never edited by hand: a local a body wants across a yield that is not here is a question for the program.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-sky.script.ts:107-114) the beat 'hold': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyHoldFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// held: a yield separates its declaration from a use
    pub held: Vec<crate::director::UnitRef>,
}

/// (levels/tut-sky.script.ts:119-126) the beat 'ambush': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyAmbushFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* rake: yield* rake: the helper's frame while it runs. Step it until it answers None
    pub sub_rake: Option<alloc::boxed::Box<crate::script_tut_sky::TutSkyRakeHelperFrame>>,
}

/// (levels/tut-sky.script.ts:127-132) the beat 'pinned': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyPinnedFrame {
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

/// (levels/tut-sky.script.ts:142-154) the beat 'smoke': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkySmokeFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* smoke: yield* smoke: the helper's frame while it runs. Step it until it answers None
    pub sub_smoke: Option<alloc::boxed::Box<crate::script_tut_sky::TutSkySmokeHelperFrame>>,
}

/// (levels/tut-sky.script.ts:155-163) the beat 'break': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyBreakFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-sky.script.ts:164-164) the beat 'ordered': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyOrderedFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-sky.script.ts:167-167) the beat 'blind': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyBlindFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* harass: yield* harass: the helper's frame while it runs. Step it until it answers None
    pub sub_harass: Option<alloc::boxed::Box<crate::script_tut_sky::TutSkyHarassHelperFrame>>,
}

/// (levels/tut-sky.script.ts:173-187) the beat 'push': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyPushFrame {
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

/// (levels/tut-sky.script.ts:190-195) the beat 'indoors': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyIndoorsFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-sky.script.ts:198-202) the beat 'wall': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyWallFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-sky.script.ts:203-207) the beat 'dark': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyDarkFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-sky.script.ts:208-212) the beat 'nv': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyNvFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-sky.script.ts:224-243) the beat 'torches': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyTorchesFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// i: a yield separates its declaration from a use
    pub i: f64,
    /// u: a yield separates its declaration from a use
    pub u_2: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u_3: Option<crate::director::UnitRef>,
}

/// (levels/tut-sky.script.ts:244-265) the beat 'inside': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyInsideFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::script_tut_sky::TutSkyInsideMen>,
    /// left: a yield separates its declaration from a use
    pub left: Vec<crate::script_tut_sky::TutSkyInsideLeft>,
    /// m: a yield separates its declaration from a use
    pub m: Option<crate::script_tut_sky::TutSkyInsideM>,
    /// pending: a yield separates its declaration from a use
    pub pending: Option<JsSet<crate::ObjKey>>,
    /// t: a yield separates its declaration from a use
    pub t: f64,
    /// m: a yield separates its declaration from a use
    pub m_2: Option<crate::script_tut_sky::TutSkyInsideM>,
    /// m: a yield separates its declaration from a use
    pub m_3: Option<crate::script_tut_sky::TutSkyInsideM>,
}

/// (levels/tut-sky.script.ts:266-270) the beat 'out': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyOutFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-sky.script.ts:69-85) the helper generator rake: where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyRakeHelperFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// zones: a parameter
    pub zones: Vec<crate::script_tut_sky::TutSkyZone>,
    /// stop: a closure parameter: which of the script's own functions the caller means. Your private fn matches on it
    pub stop: u16,
    /// o: a parameter
    pub o: Option<crate::script_tut_sky::TutSkyRakeHelperO>,
    /// total: a yield separates its declaration from a use
    pub total: f64,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::director::UnitRef>,
    /// r: a yield separates its declaration from a use
    pub r: f64,
    /// Z: a yield separates its declaration from a use
    pub z: Option<crate::script_tut_sky::TutSkyZone>,
    /// z: a yield separates its declaration from a use
    pub z_2: Option<crate::script_tut_sky::TutSkyZone>,
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

/// (levels/tut-sky.script.ts:87-97) the helper generator harass: where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyHarassHelperFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// marks: a parameter
    pub marks: Vec<[f64; 3]>,
    /// stop: a closure parameter: which of the script's own functions the caller means. Your private fn matches on it
    pub stop: u16,
    /// o: a parameter
    pub o: Option<crate::script_tut_sky::TutSkyHarassHelperO>,
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

/// (levels/tut-sky.script.ts:144-149) the helper generator smoke, inside the beat 'smoke': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkySmokeHelperFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// again: a parameter
    pub again: bool,
    /// by: a yield separates its declaration from a use
    pub by: Vec<Option<crate::director::UnitRef>>,
}

/// (levels/tut-sky.script.ts) what the define pass names and a beat reads: the script's and its programs' locals. Its closures are your private fns: block, torchMen, squad, inBunk, atBunk, inYard, pick
#[derive(Clone, Debug, Default)]
pub struct TutSkyVars {
    pub rifle: Option<crate::director::LoadoutRef>,
    pub yaw: f64,
    pub bunk_door: Option<crate::director::DoorRef>,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutSkyZone {
    pub lo: [f64; 3],
    pub hi: [f64; 3],
    pub w: f64,
    pub by: Option<Vec<String>>,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutSkyRakeHelperO {
    pub rounds: [f64; 2],
    pub gap: [f64; 2],
    pub spread: f64,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutSkyHarassHelperO {
    pub rounds: [f64; 2],
    pub gap: [f64; 2],
    pub spread: f64,
}

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutSkyInsideMen {
    pub u: Option<crate::director::UnitRef>,
    pub to: [f64; 3],
}

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutSkyInsideLeft {
    pub u: Option<crate::director::UnitRef>,
    pub to: [f64; 3],
}

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutSkyInsideM {
    pub u: Option<crate::director::UnitRef>,
    pub to: [f64; 3],
}
