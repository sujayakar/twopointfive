//! The numbers of rescuecheck.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// the switch and its numbers: on (?norescuecheck = a rescue's men never miss each other,
///  as it was) and by the shipped stand; the harness's frozen stand leaves it off (TPS_RESCUECHECK=1 turns it on). UNTUNED by play.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RescuecheckK {
    /// the first check after the staging, and the gap between checks, s (where in the band: the caller's id and the check's number — no dice)
    #[k([14.0, 22.0])] pub first: [f64; 2],
    #[k([24.0, 36.0])] pub every: [f64; 2],
    /// the breath a man is given to answer before he is asked for by name · the breath after it before a man goes
    #[k(4.0)] pub ask_secs: f64,
    #[k(4.0)] pub go_secs: f64,
    /// how long the man who went stands at an empty post before he says so and walks back
    #[k(5.0)] pub wait_secs: f64,
    /// legs of a walk tried again before it is given up (the room fight ends a leg at ten seconds)
    #[k(4.0)] pub tries: f64,
    /// how far round his feet the man at the post finds a body in any light, m
    #[k(2.5)] pub feet_m: f64,
    /// a roamer's word this long before the rescue lets him off his post
    #[k(2.5)] pub word_secs: f64,
}
