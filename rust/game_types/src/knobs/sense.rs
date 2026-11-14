//! The numbers of sense.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SenseK {
    /// the light under a roof (glow.rs): the steps a sweep of the whole plan takes (a step is a tenth of a second)
    pub glow: SenseKGlow,
    pub smoke: SenseKSmoke,
}

/// the light under a roof (glow.rs): the steps a sweep of the whole plan takes (a step is a tenth of a second)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SenseKGlow {
    #[k(32.0)] pub sweep: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SenseKSmoke {
    /// extinction per metre per unit of density (the renderer's: σt = 6/m per unit)
    #[k(6.0)] pub sigma: f64,
    /// what the smoke loses a second (the solver's densDecay)
    #[k(0.14)] pub decay: f64,
    /// how fast the smoke spreads (m²/s)
    #[k(0.2)] pub spread: f64,
    /// how far smoke rises above where it was let out (m), and how fast (m/s)
    #[k(1.0)] pub rise: f64,
    #[k(0.3)] pub rise_rate: f64,
    /// an emitter that lets out less than this in its whole life (density × volume, unit·m³) is the picture's alone: a muzzle's wisp, a spark's trail hide nobody
    #[k(0.05)] pub least: f64,
    /// a brick thinner than this (unit·m³) is gone
    #[k(0.02)] pub gone: f64,
    /// how many of the emitters a game holds it keeps back from `budget`
    #[k(12.0)] pub reserve: f64,
}
