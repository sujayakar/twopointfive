//! The numbers of cmd.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// A RECORD FROM ANYONE (see the head). ON in the page (`?nocmdcheck` = a record is believed as it comes, as before), OFF on the frozen stand (`TPS_CMDCHECK=1`).
///  The bounds are wide on purpose: they keep a record sane, they do not referee it. What referees holds only for a seat marked `remote` (nobody today: a showcase
///  and a script pin the one local camera wherever they like, and the page's own matrices may be a step behind its pose): `reach`, how far from its own man the
///  seat's look-at may be (its eye no further from the look-at than twice that), and the view WORKED OUT AGAIN from the look-at, the angles, the distance and the lens.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct CmdcheckK {
    #[k(24.0)] pub keys: f64,
    #[k(24.0)] pub key_len: f64,
    #[k(16384.0)] pub span: f64,
    #[k(4096.0)] pub wheel: f64,
    #[k(10000.0)] pub far: f64,
    #[k(0.05)] pub dist_min: f64,
    #[k(2000.0)] pub dist_max: f64,
    #[k(0.02)] pub fov_min: f64,
    #[k(3.0)] pub fov_max: f64,
    #[k(0.05)] pub aspect_min: f64,
    #[k(20.0)] pub aspect_max: f64,
    #[k(80.0)] pub reach: f64,
    /// the steepest a rebuilt view looks down or up (rad): straight down has no up to build a view from
    #[k(1.5)] pub steep: f64,
}
