//! The types of `fire`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FireStats {
    pub fires: f64,
    pub lit: f64,
    pub frames: f64,
    pub cracks: f64,
    pub lo: f64,
    pub hi: f64,
    pub sum: f64,
    pub n: f64,
}

#[derive(Clone, Debug, Default)]
pub struct FireShown {
    pub n: f64,
    pub time: f64,
    pub stir: Option<Vec<f64>>,
    pub stir_at: Option<Vec<f64>>,
    pub bed: Vec<f64>,
    pub smoke: Vec<f64>,
    pub smokes: f64,
    pub beds: f64,
    pub ember: Vec<f64>,
    pub embers: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FIRELOOKSMOKE {
    pub up: f64,
    pub radius: f64,
    pub density: f64,
    pub puff_every: f64,
    pub puff_secs: f64,
    pub puff_up: f64,
    pub puff_radius: f64,
    pub puff_off: f64,
    pub puff_density: f64,
    pub puff_temp: f64,
    pub puff_speed: f64,
    pub puff_push: f64,
    pub puff_expand: f64,
    pub soot: V3,
    pub soot_ease: f64,
    pub ember_up: f64,
    pub ember_spread: f64,
    pub ember_jit: f64,
    pub ember_speed: f64,
    pub snow: crate::fire::FIRELOOKSMOKESnow,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FIRELOOKSMOKESnow {
    pub speed: f64,
    pub reach: f64,
    pub spread: f64,
    pub mouth: f64,
    pub top: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FirePopsItem {
    pub at: f64,
    pub hard: f64,
    pub seed: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MouthOf {
    pub pos: V3,
    pub r: f64,
    pub lid: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WindAt {
    pub wv: V3,
    pub wl: f64,
    pub wind: [f64; 2],
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct FireMod {
    /// (src/game/fire.ts:223 states)
    pub states: Option<crate::fire::FireStats>,
}
impl FireMod {
    pub fn blank() -> Self {
        FireMod {
            states: None,
        }
    }
}

/// (src/game/fire.ts:167 FIRE_LOOK_SMOKE: a table nothing writes)
pub fn fire_look_smoke() -> crate::fire::FIRELOOKSMOKE { crate::fire::FIRELOOKSMOKE { up: 0.8, radius: 0.2, density: 0.8, puff_every: 0.42, puff_secs: 0.26, puff_up: 0.95, puff_radius: 0.24, puff_off: 0.09, puff_density: 3.2, puff_temp: 30.0, puff_speed: 1.25, puff_push: 9.0, puff_expand: 2.2, soot: [0.46, 0.48, 0.53], soot_ease: 0.5, ember_up: 0.72, ember_spread: 0.2, ember_jit: 0.4, ember_speed: 1.5, snow: crate::fire::FIRELOOKSMOKESnow { speed: 2.6, reach: 2.2, spread: 0.2, mouth: 1.3, top: 4.5 } } }

/// (src/game/fire.ts:234 TAU)
pub const TAU: f64 = 6.283185307179586;

