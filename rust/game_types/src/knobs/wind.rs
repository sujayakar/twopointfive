//! The numbers of wind.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// The lane's knobs (Tab ▸ Weather ▸ wind rows; persisted with tune.wind).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct WindK {
    /// tune.wind's default: the master 0‥1 (× every source's speed; 0 = still air, nothing packed anywhere)
    #[k(1.0)] pub def: f64,
    /// the level's diffusers, × their authored speed
    #[k(1.0)] pub vent_k: f64,
    /// the weather's wind outdoors as a source, × its speed
    #[k(1.0)] pub storm_k: f64,
    /// gusts through open exterior doors: reach (m), cone half-angle, the leaf angle that counts as fully open
    #[k(1.0)] pub door_k: f64,
    #[k(2.2)] pub door_reach: f64,
    #[k(22.0)] pub door_spread_deg: f64,
    #[k(1.2)] pub door_open_rad: f64,
    /// × (1 ± amp) on a value noise of the clock, per source
    #[k(0.25)] pub gust_amp: f64,
    #[k(3.5)] pub gust_period: f64,
    #[k(0.35)] pub storm_gust_amp: f64,
    #[k(2.5)] pub storm_gust_period: f64,
    /// s: a vent spinning up (cycle on / mains back) and down (cycle off / blackout)
    #[k(3.0)] pub ramp_up: f64,
    #[k(4.0)] pub ramp_down: f64,
    /// 1/s at a jet's core: how hard the solver's air is pulled toward the field (× the jet's own profile off the core)
    #[k(3.0)] pub smoke_relax: f64,
    /// the particle kernel's three (particles.wgsl windKnobs): air speed over a resting leaf that may pick it up, × that = its hop, a chip's coupling
    #[k(0.35)] pub sheet_wake: f64,
    #[k(2.0)] pub sheet_kick: f64,
    #[k(0.15)] pub debris_k: f64,
    /// particles: jets whose support comes within this of the view's look-at point are packed, nearest first
    #[k(14.0)] pub near_m: f64,
}
