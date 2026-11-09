//! The types of `moths`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes)]
pub struct Moths {
    pub lamps: Vec<crate::moths::MothsLampsItem>,
    pub moths: Vec<crate::moths::Moth>,
}
impl Moths {
    /// what a new one holds
    pub fn blank() -> Self {
        Moths {
            lamps: Vec::new(),
            moths: Vec::new(),
        }
    }
}
impl Default for Moths { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct MothsLampsItem {
    pub target: FixtureId,
    pub light: LightId,
    pub head: V3,
    pub act: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Moth {
    pub lamp: f64,
    pub r: f64,
    pub omega: f64,
    pub ph: f64,
    pub bob_f: f64,
    pub bob_ph: f64,
    pub wob: f64,
    pub dart_t: f64,
    pub dart_ph: f64,
    pub flap_ph: f64,
    pub away: V3,
    pub shown: f64,
    pub box_: BoxId,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MOTHS {
    pub max: f64,
    pub per_lamp: f64,
    pub r_min: f64,
    pub r_max: f64,
    pub below: f64,
    pub bob: f64,
    pub omega_min: f64,
    pub omega_max: f64,
    pub flap_hz: f64,
    pub wing: f64,
    pub gather_secs: f64,
    pub scatter_secs: f64,
    pub gale_ms: [f64; 2],
    pub rain_max: f64,
    pub albedo: V3,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct MothsStats {
    pub shown: i32,
}

pub fn moths() -> crate::moths::MOTHS { crate::moths::MOTHS { max: 12.0, per_lamp: 7.0, r_min: 0.14, r_max: 0.5, below: 0.25, bob: 0.16, omega_min: 2.2, omega_max: 4.6, flap_hz: 9.0, wing: 0.016, gather_secs: 2.5, scatter_secs: 1.2, gale_ms: [3.0, 6.0], rain_max: 0.92, albedo: [0.62, 0.56, 0.42] } }
