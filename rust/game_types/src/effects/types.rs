//! The types of `effects`.
#![allow(unused_imports)]
use crate::{Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what a smoke source rides, read live each time it is asked: a shot's two off the shooter's body, a blast's trail off its spark (which answers nothing once the spark dies or its slot is reused)
#[derive(Clone, Debug)]
pub enum Track {
    Muzzle { by: CharId },
    EjectPort { by: CharId },
    Spark { slot: usize, gen_: f64 },
    /// the lit end of a man's cigarette, while he smokes and is seen
    Cigarette { by: CharId },
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FxLight {
    pub pos: V3,
    pub color: V3,
    pub intensity: f64,
    pub range: f64,
    pub ttl: f64,
    pub radius: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum MuzzleFxKey {
    #[default] Mk12,
    Mp7,
    Mk46,
    M1014,
    Hk45,
    G18,
    Carbine,
    Sc20k,
    PistolSuppressed,
    PistolBare,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Flags {
    pub shot_sparks: bool,
    pub ejection_wisp: bool,
    pub dazzle: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MuzzleFrame {
    pub o: V3,
    pub x: V3,
    pub y: V3,
    pub z: V3,
}

#[derive(Clone, Debug, Default)]
pub struct SpawnShotO {
    pub light_gain: Option<f64>,
    pub light_range: Option<f64>,
    pub track: Option<crate::effects::Track>,
    pub eject: Option<crate::effects::Track>,
    pub gas: Option<bool>,
}

pub fn flags() -> crate::effects::Flags { crate::effects::Flags { shot_sparks: true, ejection_wisp: true, dazzle: true } }

/// what the rules know of a stun charge (how it is seen is the knobs' `effects.bang`)
#[derive(Clone, Copy, Debug)]
pub struct Bang {
    pub fuse: f64,
    pub max_air: f64,
    pub height: f64,
    pub dazzle_radius: f64,
    pub dazzle_seconds: f64,
    pub dazzle_min: f64,
}

pub fn bang() -> crate::effects::Bang { crate::effects::Bang { fuse: 1.5, max_air: 3.0, height: 0.07, dazzle_radius: 8.0, dazzle_seconds: 6.0, dazzle_min: 2.0 } }


