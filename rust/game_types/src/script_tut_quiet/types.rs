//! The frames of the tut-quiet level's script (rust/game_rules/src/missions/script_tut_quiet.rs): where each beat stands, and the locals that live across a yield.
use crate::bytes::ToBytes;

///  the beat 'lane': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutQuietLaneFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'night': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
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

///  the beat 'store': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutQuietStoreFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  what the define pass names and a beat reads: the script's and its programs' locals. Its closures are your private fns: calm
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutQuietVars {
    pub walk: Option<crate::director::VolumeRef>,
    pub beam: Option<crate::director::VolumeRef>,
    pub past: Option<crate::director::VolumeRef>,
}

