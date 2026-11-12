//! MOTHS AROUND THE LIT SODIUM LAMPS (exterior spot lights hung over 3.5 m), `air/moths.rs`. For the picture: no AI reads them and no ray meets them. Still, each one is
//! a keyed box on the frame's list, which the fog of war and the frame's budget count, so they run in the tick. Only `tune.moths` turns them off.
use crate::bytes::ToBytes;
use crate::V3;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct MothsK {
    /// how many moths a level has (dealt once when it starts), shared out among the lamps by how much each one draws them
    #[k(12.0)] pub max: f64,
    /// at most this many at one lamp
    #[k(7.0)] pub per_lamp: f64,
    /// m: how wide a moth circles the lamp, each one's a draw between these two (it breathes × 0.4‥1.1 as it flies, and darts in to a fifth of it)
    #[k(0.14)] pub r_min: f64,
    #[k(0.5)] pub r_max: f64,
    /// m: how far under the lamp head it flies, and how much further down it bobs
    #[k(0.25)] pub below: f64,
    #[k(0.16)] pub bob: f64,
    /// radians a second around the lamp, each one's a draw between these two, either way around
    #[k(2.2)] pub omega_min: f64,
    #[k(4.6)] pub omega_max: f64,
    /// wing beats a second: the wings' plane rocks ± 0.9 radians
    #[k(9.0)] pub flap_hz: f64,
    /// m: half the wingspan
    #[k(0.016)] pub wing: f64,
    /// seconds for a lamp's pull to come up, and for a moth to fly in · for them to fall away, as the moth flies off over 3.5 m
    #[k(2.5)] pub gather_secs: f64,
    #[k(1.2)] pub scatter_secs: f64,
    /// m/s of wind at the lamp: over the first the moths start to go, over the second there are none
    #[k([3.0, 6.0])] pub gale_ms: [f64; 2],
    /// the rain's rate where there are none left (they start to go at 0.75; falling snow is 1)
    #[k(0.92)] pub rain_max: f64,
    /// their color
    #[k([0.62, 0.56, 0.42])] pub albedo: V3,
}

