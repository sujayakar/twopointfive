//! The types of `effects`.
#![allow(unused_imports)]
use crate::{Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what a smoke source rides, read live each time it is asked: a shot's two off the shooter's body, a blast's trail off its spark (sparks.ts tracker: answers nothing once the spark dies or its slot is reused)
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

#[derive(Clone, Copy, Debug, Default)]
pub struct MuzzleFx {
    pub card_ahead: f64,
    pub card_half: V3,
    pub card_emissive: V3,
    pub light_gain: f64,
    pub light_range: Option<f64>,
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
    pub trails: bool,
    pub vent_canister: bool,
    pub dazzle: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Shot {
    pub standoff: f64,
    pub jet_off: V3,
    pub port_off: V3,
    pub wisp_off: V3,
    pub jet_radius: f64,
    pub jet_density: f64,
    pub jet_temp: f64,
    pub jet_speed: f64,
    pub jet_rise: f64,
    pub jet_push: f64,
    pub jet_expand: f64,
    pub jet_life: f64,
    pub jet_attack: f64,
    pub ports: f64,
    pub port_fraction: f64,
    pub port_angle: f64,
    pub port_radius_k: f64,
    pub port_speed_k: f64,
    pub port_push_k: f64,
    pub port_expand_k: f64,
    pub wisp_radius: f64,
    pub wisp_density: f64,
    pub wisp_temp: f64,
    pub wisp_rise: f64,
    pub wisp_push: f64,
    pub wisp_life: f64,
    pub wisp_attack: f64,
    pub eject_radius: f64,
    pub eject_density: f64,
    pub eject_temp: f64,
    pub eject_life: f64,
    pub jitter: f64,
    pub light_color: V3,
    pub light_power: f64,
    pub light_duration: f64,
    pub light_range: f64,
    pub suppressed_gain: f64,
    pub spark_count: f64,
    pub spark_speed: f64,
    pub spark_cone: f64,
    pub spark_life: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MuzzleFx2 {
    pub pistol_suppressed: crate::effects::MuzzleFx,
    pub pistol_bare: crate::effects::MuzzleFx,
    pub carbine: crate::effects::MuzzleFx,
    pub sc20k: crate::effects::MuzzleFx,
    pub mk12: crate::effects::MuzzleFx,
    pub mp7: crate::effects::MuzzleFx,
    pub mk46: crate::effects::MuzzleFx,
    pub m1014: crate::effects::MuzzleFx,
    pub hk45: crate::effects::MuzzleFx,
    pub g18: crate::effects::MuzzleFx,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MuzzleCard {
    pub albedo: V3,
    pub ttl: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Bang {
    pub fuse: f64,
    pub max_air: f64,
    pub height: f64,
    pub radius: f64,
    pub density: f64,
    pub temp: f64,
    pub expand: f64,
    pub life: f64,
    pub attack: f64,
    pub end_push: f64,
    pub vent_speed: f64,
    pub half_length: f64,
    pub asymmetry: f64,
    pub tilt: f64,
    pub jitter: f64,
    pub body_vents: f64,
    pub body_fraction: f64,
    pub body_push: f64,
    pub body_radius_k: f64,
    pub body_speed_k: f64,
    pub body_expand_k: f64,
    pub wisp_radius: f64,
    pub wisp_density: f64,
    pub wisp_temp: f64,
    pub wisp_rise: f64,
    pub wisp_push: f64,
    pub wisp_life: f64,
    pub wisp_attack: f64,
    pub frame_jitter: f64,
    pub density_gain: f64,
    pub temp_gain: f64,
    pub light_color: V3,
    pub light_power: f64,
    pub light_duration: f64,
    pub light_range: f64,
    pub light_radius: f64,
    pub light_lift: f64,
    pub spark_count: f64,
    pub trail_count: f64,
    pub trail_reserve: f64,
    pub dazzle_radius: f64,
    pub dazzle_seconds: f64,
    pub dazzle_min: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Sparks {
    pub speed: f64,
    pub spread: f64,
    pub lift: f64,
    pub life: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Trail {
    pub radius: f64,
    pub min_voxels: f64,
    pub density: f64,
    pub temp: f64,
    pub life: f64,
    pub attack: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Vent {
    pub radius: f64,
    pub density: f64,
    pub temp: f64,
    pub speed: f64,
    pub rise: f64,
    pub push: f64,
    pub expand: f64,
    pub seconds: f64,
    pub attack: f64,
    pub jitter: f64,
    pub standoff: f64,
    pub height: f64,
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

#[derive(Clone, Copy, Debug, Default)]
pub struct SpawnBurstO {
    pub spark_count: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SpawnVentO {
    pub seconds: Option<f64>,
}

pub fn flags() -> crate::effects::Flags { crate::effects::Flags { shot_sparks: true, ejection_wisp: true, trails: true, vent_canister: false, dazzle: true } }

pub fn shot() -> crate::effects::Shot { crate::effects::Shot { standoff: 0.22, jet_off: [0.0, 0.0, 0.22], port_off: [0.0, 0.0, 0.154], wisp_off: [0.0, 0.0, 0.03], jet_radius: 0.08, jet_density: 24.0, jet_temp: 120.0, jet_speed: 10.5, jet_rise: 0.2, jet_push: 90.0, jet_expand: 24.0, jet_life: 0.08, jet_attack: 0.01, ports: 2.0, port_fraction: 0.4, port_angle: 1.1, port_radius_k: 0.7, port_speed_k: 0.55, port_push_k: 0.6, port_expand_k: 0.5, wisp_radius: 0.042, wisp_density: 4.1, wisp_temp: 43.0, wisp_rise: 0.6, wisp_push: 4.0, wisp_life: 2.5, wisp_attack: 0.15, eject_radius: 0.018, eject_density: 1.6, eject_temp: 2.5, eject_life: 0.7, jitter: 0.1, light_color: [1.0, 0.62, 0.26], light_power: 90.0, light_duration: 0.09, light_range: 8.0, suppressed_gain: 0.45, spark_count: 20.0, spark_speed: 31.0, spark_cone: 0.25, spark_life: 0.06 } }

pub fn muzzle_fx() -> crate::effects::MuzzleFx2 { crate::effects::MuzzleFx2 { pistol_suppressed: crate::effects::MuzzleFx { card_ahead: 0.05, card_half: [0.018, 0.018, 0.045], card_emissive: [40.0, 22.0, 8.0], light_gain: 1.0, light_range: None }, pistol_bare: crate::effects::MuzzleFx { card_ahead: 0.09, card_half: [0.03, 0.03, 0.08], card_emissive: [90.0, 50.0, 16.0], light_gain: 1.0, light_range: None }, carbine: crate::effects::MuzzleFx { card_ahead: 0.14, card_half: [0.04, 0.04, 0.12], card_emissive: [110.0, 60.0, 18.0], light_gain: 1.7, light_range: Some(10.0) }, sc20k: crate::effects::MuzzleFx { card_ahead: 0.09, card_half: [0.03, 0.03, 0.09], card_emissive: [60.0, 34.0, 12.0], light_gain: 1.2, light_range: None }, mk12: crate::effects::MuzzleFx { card_ahead: 0.07, card_half: [0.024, 0.024, 0.07], card_emissive: [44.0, 24.0, 8.0], light_gain: 1.0, light_range: None }, mp7: crate::effects::MuzzleFx { card_ahead: 0.05, card_half: [0.016, 0.016, 0.05], card_emissive: [34.0, 19.0, 7.0], light_gain: 0.85, light_range: None }, mk46: crate::effects::MuzzleFx { card_ahead: 0.11, card_half: [0.05, 0.022, 0.085], card_emissive: [95.0, 52.0, 16.0], light_gain: 1.45, light_range: Some(10.0) }, m1014: crate::effects::MuzzleFx { card_ahead: 0.15, card_half: [0.06, 0.06, 0.11], card_emissive: [120.0, 58.0, 14.0], light_gain: 2.0, light_range: Some(11.0) }, hk45: crate::effects::MuzzleFx { card_ahead: 0.045, card_half: [0.016, 0.016, 0.035], card_emissive: [26.0, 14.0, 5.0], light_gain: 0.7, light_range: None }, g18: crate::effects::MuzzleFx { card_ahead: 0.08, card_half: [0.026, 0.042, 0.07], card_emissive: [95.0, 52.0, 16.0], light_gain: 1.1, light_range: None } } }

pub fn muzzle_card() -> crate::effects::MuzzleCard { crate::effects::MuzzleCard { albedo: [1.0, 0.8, 0.5], ttl: 0.035 } }

pub fn bang() -> crate::effects::Bang { crate::effects::Bang { fuse: 1.5, max_air: 3.0, height: 0.07, radius: 0.23, density: 6.3, temp: 48.0, expand: 90.0, life: 0.12, attack: 0.02, end_push: 40.0, vent_speed: 7.0, half_length: 0.1, asymmetry: 0.45, tilt: 0.25, jitter: 0.6, body_vents: 5.0, body_fraction: 0.25, body_push: 30.0, body_radius_k: 0.7, body_speed_k: 0.6, body_expand_k: 0.5, wisp_radius: 0.24, wisp_density: 1.2, wisp_temp: 10.5, wisp_rise: 0.6, wisp_push: 5.0, wisp_life: 3.0, wisp_attack: 0.3, frame_jitter: 0.08, density_gain: 1.0, temp_gain: 1.0, light_color: [1.0, 0.98, 0.95], light_power: 1600.0, light_duration: 0.14, light_range: 14.0, light_radius: 0.15, light_lift: 0.3, spark_count: 72.0, trail_count: 12.0, trail_reserve: 2.0, dazzle_radius: 8.0, dazzle_seconds: 6.0, dazzle_min: 2.0 } }

pub fn sparks() -> crate::effects::Sparks { crate::effects::Sparks { speed: 20.0, spread: 0.55, lift: 0.5, life: 0.55 } }

pub fn tracer_fx() -> crate::tracers::TracersParams { crate::tracers::TracersParams { speed: 280.0, min_life: 0.042, shutter: 0.011, thickness: 0.012, emissive: 60.0, min_path: 0.5, light_intensity: 4.0, light_range: 4.0, light_radius: 0.06, ember_sparks: 3.0, ember_speed: 5.0, ember_cone: 0.9, ember_life: 0.3, ember_intensity: 2.5, ember_range: 2.5, ember_ttl: 0.06, max_live: 24.0, max_lights: 6.0 } }

pub fn trail() -> crate::effects::Trail { crate::effects::Trail { radius: 0.036, min_voxels: 1.0, density: 8.6, temp: 45.0, life: 0.9, attack: 0.01 } }

pub fn vent() -> crate::effects::Vent { crate::effects::Vent { radius: 0.33, density: 18.0, temp: 0.0, speed: 4.5, rise: 0.15, push: 45.0, expand: 0.0, seconds: 30.0, attack: 0.12, jitter: 0.12, standoff: 0.07, height: 0.12 } }

