//! The numbers of autopilot.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct AutopilotK {
    /// seconds the plan of an order the quick key gave is drawn before it has faded
    #[k(4.0)] pub show_secs: f64,
}
