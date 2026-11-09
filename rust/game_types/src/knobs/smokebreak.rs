//! The numbers of smokebreak.
use crate::bytes::ToBytes;
use crate::V3;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SmokebreakK {
    /// the clock: s to light it (the hand is up from `lightUp[0]` to `lightUp[1]`, the flame burns `flame` s from `flameAt`), then a draw every `every`
    ///  s, the hand up for `draw` s of it, and what he breathes out leaves `outAfter` s after the hand starts down, for `out` s
    #[k(3.2)] pub light_secs: f64,
    #[k([0.5, 2.6])] pub light_up: [f64; 2],
    #[k(1.3)] pub flame_at: f64,
    #[k(0.9)] pub flame: f64,
    #[k(7.5)] pub every: f64,
    #[k(2.1)] pub draw: f64,
    #[k(0.7)] pub out_after: f64,
    #[k(1.3)] pub out: f64,
    /// the lights (LightSet.flash: colour, intensity, range m; the end's is re-filed every `emberEvery` s so it never quite goes out)
    #[k([1.0, 0.62, 0.24])] pub flame_color: V3,
    #[k(2.4)] pub flame_k: f64,
    #[k(3.2)] pub flame_range: f64,
    #[k([1.0, 0.36, 0.1])] pub ember_color: V3,
    #[k(0.1)] pub ember_k: f64,
    #[k(0.7)] pub ember_draw_k: f64,
    #[k(1.3)] pub ember_range: f64,
    #[k(0.4)] pub ember_every: f64,
    /// where the end is off the left hand's joint, m along the hand's own heading (worked out from the forearm) and up
    #[k(0.11)] pub tip_out: f64,
    #[k(0.02)] pub tip_up: f64,
    /// THE THREAD off the end (the solver's units, as snowBreath.ts BREATHSTEAM: 6 /m of extinction a unit of density; +1.6 m/s² a unit of heat)
    pub thread: SmokebreakKThread,
    /// WHAT HE BREATHES OUT after a draw
    pub breath: SmokebreakKBreath,
    /// what the smoke scatters: a blue grey (the solver's `tint`, the puff's colour)
    #[k([0.72, 0.76, 0.86])] pub color: V3,
    /// m: the mouth off the head joint, ahead and down (snowBreath.ts's own two numbers)
    #[k(0.1)] pub mouth_ahead: f64,
    #[k(0.075)] pub mouth_down: f64,
}

/// THE THREAD off the end (the solver's units, as snowBreath.ts BREATHSTEAM: 6 /m of extinction a unit of density; +1.6 m/s² a unit of heat)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SmokebreakKThread {
    #[k(0.02)] pub radius: f64,
    #[k(1.2)] pub cells: f64,
    #[k(5.0)] pub density: f64,
    #[k(5.0)] pub temp: f64,
    #[k(0.25)] pub speed: f64,
    #[k(10.0)] pub push: f64,
    #[k(0.35)] pub jitter: f64,
}

/// WHAT HE BREATHES OUT after a draw
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SmokebreakKBreath {
    #[k(0.04)] pub radius: f64,
    #[k(1.5)] pub cells: f64,
    #[k(16.0)] pub density: f64,
    #[k(2.5)] pub temp: f64,
    #[k(1.1)] pub speed: f64,
    #[k(12.0)] pub push: f64,
    #[k(2.2)] pub expand: f64,
    #[k(0.1)] pub attack: f64,
    #[k(0.3)] pub jitter: f64,
    #[k(0.1)] pub down: f64,
}
