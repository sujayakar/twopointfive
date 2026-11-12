//! × EVERY PUFF OF DUST, at the one place all of them pass through (particle_fx.rs, dust_asked). The picture alone. All 1 = each site's numbers as it wrote them.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct DustK {
    /// × how many, rounded and never under 1 where a site asked for any; 0 = no dust anywhere
    #[k(1.0)] pub count: f64,
    /// × a puff's radius · life · speed, and any velocity it inherits · upward lift · scatter of where it's born
    #[k(1.0)] pub size: f64,
    #[k(1.0)] pub life: f64,
    #[k(1.0)] pub speed: f64,
    #[k(1.0)] pub rise: f64,
    #[k(1.0)] pub spread: f64,
}
