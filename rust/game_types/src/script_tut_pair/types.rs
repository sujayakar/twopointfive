//! The frames of the tut-pair level's script (rust/game_rules/src/missions/script_tut_pair.rs): where each beat stands, and the locals that live across a yield.
use crate::bytes::ToBytes;

///  the beat 'gate': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutPairGateFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'park': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutPairParkFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  the beat 'yard': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutPairYardFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

///  what the define pass names and a beat reads: the script's and its programs' locals. Its closures are your private fns: calm
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TutPairVars {
}

