//! The numbers of peeksight.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// `out` m: how far past the end of the face the eye stands at full peek · `off` m: how far off the wall's plane (his side of it) · `turnDeg`: how far round
///  from 'along the wall, toward that end' it looks, into the opening (90 = square through it) — read only where an eye is a cone
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct PeeksightK {
    #[k(0.3)] pub out: f64,
    #[k(0.1)] pub off: f64,
    #[k(90.0)] pub turn_deg: f64,
}
