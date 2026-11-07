//! The frames of levels/tut-quiet.script.ts, written by the skeleton's program from the checker. Never edited by hand: a local a body wants across a yield that is not here is a question for the program.
#![allow(unused_imports, unused_variables, unused_mut)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-quiet.script.ts:34-43) the beat 'lane': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutQuietLaneFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-quiet.script.ts:46-93) the beat 'night': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutQuietNightFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// at: a yield separates its declaration from a use
    pub at_2: Option<[f64; 3]>,
    /// w: a yield separates its declaration from a use
    pub w: Option<crate::director::CondResult>,
    /// r: a yield separates its declaration from a use
    pub r: Option<crate::director::CondResult>,
}

/// (levels/tut-quiet.script.ts:96-102) the beat 'store': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutQuietStoreFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

/// (levels/tut-quiet.script.ts) what the define pass names and a beat reads: the script's and its programs' locals. Its closures are your private fns: calm
#[derive(Clone, Debug, Default)]
pub struct TutQuietVars {
    pub walk: Option<crate::director::VolumeRef>,
    pub beam: Option<crate::director::VolumeRef>,
    pub past: Option<crate::director::VolumeRef>,
}

