//! The numbers of air dust.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct AirdustK {
    /// how fast the dust spreads through open air indoors (m²/s)
    #[k(1.5)] pub diffuse: f64,
    /// how fast a room's air settles back to the weather's leak (1/s)
    #[k(0.12)] pub settle: f64,
    /// the fixed step (s): D · step ∕ h² must stay under ¼ (0.2 at these numbers)
    #[k(0.03333333333333333)] pub step: f64,
    /// every shut edge is asked again within this long (s)
    #[k(3.0)] pub recheck_secs: f64,
}
