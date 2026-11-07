//! The types of `moths`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Moths, src/game/moths.ts:33)
#[derive(Clone, Debug)]
pub struct Moths {
    pub lamps: Vec<crate::moths::MothsLampsItem>,
    pub moths: Vec<crate::moths::Moth>,
}
impl Moths {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Moths {
            lamps: Vec::new(),
            moths: Vec::new(),
        }
    }
}
impl Default for Moths { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/moths.ts:35 Moths.lamps, src/game/moths.ts:35)
#[derive(Clone, Copy, Debug)]
pub struct MothsLampsItem {
    pub target: FixtureId,
    pub light: LightId,
    pub head: V3,
    pub act: f64,
}
impl MothsLampsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MothsLampsItem {
            target: FixtureId(0),
            light: LightId(0),
            head: [0.0; 3],
            act: 0.0,
        }
    }
}
impl Default for MothsLampsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Moth, src/game/moths.ts:31)
#[derive(Clone, Copy, Debug)]
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
impl Moth {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Moth {
            lamp: 0.0,
            r: 0.0,
            omega: 0.0,
            ph: 0.0,
            bob_f: 0.0,
            bob_ph: 0.0,
            wob: 0.0,
            dart_t: 0.0,
            dart_ph: 0.0,
            flap_ph: 0.0,
            away: [0.0; 3],
            shown: 0.0,
            box_: BoxId(0),
        }
    }
}
impl Default for Moth { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/moths.ts:14 MOTHS, src/game/moths.ts:14)
#[derive(Clone, Copy, Debug)]
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
impl MOTHS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MOTHS {
            max: 0.0,
            per_lamp: 0.0,
            r_min: 0.0,
            r_max: 0.0,
            below: 0.0,
            bob: 0.0,
            omega_min: 0.0,
            omega_max: 0.0,
            flap_hz: 0.0,
            wing: 0.0,
            gather_secs: 0.0,
            scatter_secs: 0.0,
            gale_ms: [0.0; 2],
            rain_max: 0.0,
            albedo: [0.0; 3],
        }
    }
}
impl Default for MOTHS { fn default() -> Self { Self::zeroed() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct MothsStats {
    pub shown: i32,
}

/// (src/game/moths.ts:14 MOTHS: a table nothing writes)
pub fn moths() -> crate::moths::MOTHS { crate::moths::MOTHS { max: 12.0, per_lamp: 7.0, r_min: 0.14, r_max: 0.5, below: 0.25, bob: 0.16, omega_min: 2.2, omega_max: 4.6, flap_hz: 9.0, wing: 0.016, gather_secs: 2.5, scatter_secs: 1.2, gale_ms: [3.0, 6.0], rain_max: 0.92, albedo: [0.62, 0.56, 0.42] } }
