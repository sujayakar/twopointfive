//! The types of `effects`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what a shot's smoke rides: read live off the shooter's body each time it is asked (effects.ts Track; the closures of combat.ts fireWeaponRaw)
#[derive(Clone, Debug)]
pub enum Track {
    Muzzle { by: CharId },
    EjectPort { by: CharId },
}

/// (TypeScript: FxLight, src/game/effects.ts:36)
#[derive(Clone, Copy, Debug)]
pub struct FxLight {
    pub pos: V3,
    pub color: V3,
    pub intensity: f64,
    pub range: f64,
    pub ttl: f64,
    pub radius: Option<f64>,
}
impl FxLight {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FxLight {
            pos: [0.0; 3],
            color: [0.0; 3],
            intensity: 0.0,
            range: 0.0,
            ttl: 0.0,
            radius: None,
        }
    }
}
impl Default for FxLight { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: MuzzleFx, src/game/effects.ts:80)
#[derive(Clone, Copy, Debug)]
pub struct MuzzleFx {
    pub card_ahead: f64,
    pub card_half: V3,
    pub card_emissive: V3,
    pub light_gain: f64,
    pub light_range: Option<f64>,
}
impl MuzzleFx {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MuzzleFx {
            card_ahead: 0.0,
            card_half: [0.0; 3],
            card_emissive: [0.0; 3],
            light_gain: 0.0,
            light_range: None,
        }
    }
}
impl Default for MuzzleFx { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MuzzleFxKey {
    Mk12,
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
impl MuzzleFxKey {
    pub const fn as_str(self) -> &'static str {
        match self {
            MuzzleFxKey::Mk12 => "mk12",
            MuzzleFxKey::Mp7 => "mp7",
            MuzzleFxKey::Mk46 => "mk46",
            MuzzleFxKey::M1014 => "m1014",
            MuzzleFxKey::Hk45 => "hk45",
            MuzzleFxKey::G18 => "g18",
            MuzzleFxKey::Carbine => "carbine",
            MuzzleFxKey::Sc20k => "sc20k",
            MuzzleFxKey::PistolSuppressed => "pistolSuppressed",
            MuzzleFxKey::PistolBare => "pistolBare",
        }
    }
}

/// (TypeScript: inline at src/game/effects.ts:39 flags, src/game/effects.ts:39)
#[derive(Clone, Copy, Debug)]
pub struct Flags {
    pub shot_sparks: bool,
    pub ejection_wisp: bool,
    pub trails: bool,
    pub vent_canister: bool,
    pub dazzle: bool,
}
impl Flags {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Flags {
            shot_sparks: false,
            ejection_wisp: false,
            trails: false,
            vent_canister: false,
            dazzle: false,
        }
    }
}
impl Default for Flags { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/effects.ts:53 shot, src/game/effects.ts:53)
#[derive(Clone, Copy, Debug)]
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
impl Shot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Shot {
            standoff: 0.0,
            jet_off: [0.0; 3],
            port_off: [0.0; 3],
            wisp_off: [0.0; 3],
            jet_radius: 0.0,
            jet_density: 0.0,
            jet_temp: 0.0,
            jet_speed: 0.0,
            jet_rise: 0.0,
            jet_push: 0.0,
            jet_expand: 0.0,
            jet_life: 0.0,
            jet_attack: 0.0,
            ports: 0.0,
            port_fraction: 0.0,
            port_angle: 0.0,
            port_radius_k: 0.0,
            port_speed_k: 0.0,
            port_push_k: 0.0,
            port_expand_k: 0.0,
            wisp_radius: 0.0,
            wisp_density: 0.0,
            wisp_temp: 0.0,
            wisp_rise: 0.0,
            wisp_push: 0.0,
            wisp_life: 0.0,
            wisp_attack: 0.0,
            eject_radius: 0.0,
            eject_density: 0.0,
            eject_temp: 0.0,
            eject_life: 0.0,
            jitter: 0.0,
            light_color: [0.0; 3],
            light_power: 0.0,
            light_duration: 0.0,
            light_range: 0.0,
            suppressed_gain: 0.0,
            spark_count: 0.0,
            spark_speed: 0.0,
            spark_cone: 0.0,
            spark_life: 0.0,
        }
    }
}
impl Default for Shot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/effects.ts:81 muzzleFx, src/game/effects.ts:81)
#[derive(Clone, Copy, Debug)]
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
impl MuzzleFx2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MuzzleFx2 {
            pistol_suppressed: crate::effects::MuzzleFx::zeroed(),
            pistol_bare: crate::effects::MuzzleFx::zeroed(),
            carbine: crate::effects::MuzzleFx::zeroed(),
            sc20k: crate::effects::MuzzleFx::zeroed(),
            mk12: crate::effects::MuzzleFx::zeroed(),
            mp7: crate::effects::MuzzleFx::zeroed(),
            mk46: crate::effects::MuzzleFx::zeroed(),
            m1014: crate::effects::MuzzleFx::zeroed(),
            hk45: crate::effects::MuzzleFx::zeroed(),
            g18: crate::effects::MuzzleFx::zeroed(),
        }
    }
}
impl Default for MuzzleFx2 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/effects.ts:110 muzzleCard, src/game/effects.ts:110)
#[derive(Clone, Copy, Debug)]
pub struct MuzzleCard {
    pub albedo: V3,
    pub ttl: f64,
}
impl MuzzleCard {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MuzzleCard {
            albedo: [0.0; 3],
            ttl: 0.0,
        }
    }
}
impl Default for MuzzleCard { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/effects.ts:118 bang, src/game/effects.ts:118)
#[derive(Clone, Copy, Debug)]
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
    pub hearing: f64,
    pub dazzle_radius: f64,
    pub dazzle_seconds: f64,
    pub dazzle_min: f64,
}
impl Bang {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Bang {
            fuse: 0.0,
            max_air: 0.0,
            height: 0.0,
            radius: 0.0,
            density: 0.0,
            temp: 0.0,
            expand: 0.0,
            life: 0.0,
            attack: 0.0,
            end_push: 0.0,
            vent_speed: 0.0,
            half_length: 0.0,
            asymmetry: 0.0,
            tilt: 0.0,
            jitter: 0.0,
            body_vents: 0.0,
            body_fraction: 0.0,
            body_push: 0.0,
            body_radius_k: 0.0,
            body_speed_k: 0.0,
            body_expand_k: 0.0,
            wisp_radius: 0.0,
            wisp_density: 0.0,
            wisp_temp: 0.0,
            wisp_rise: 0.0,
            wisp_push: 0.0,
            wisp_life: 0.0,
            wisp_attack: 0.0,
            frame_jitter: 0.0,
            density_gain: 0.0,
            temp_gain: 0.0,
            light_color: [0.0; 3],
            light_power: 0.0,
            light_duration: 0.0,
            light_range: 0.0,
            light_radius: 0.0,
            light_lift: 0.0,
            spark_count: 0.0,
            trail_count: 0.0,
            trail_reserve: 0.0,
            hearing: 0.0,
            dazzle_radius: 0.0,
            dazzle_seconds: 0.0,
            dazzle_min: 0.0,
        }
    }
}
impl Default for Bang { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/effects.ts:138 sparks, src/game/effects.ts:138)
#[derive(Clone, Copy, Debug)]
pub struct Sparks {
    pub speed: f64,
    pub spread: f64,
    pub lift: f64,
    pub life: f64,
    pub gravity: f64,
    pub drag: f64,
    pub brightness: f64,
    pub hold: f64,
    pub max_live: f64,
    pub emissive: f64,
    pub thickness: f64,
    pub shutter: f64,
    pub min_streak: f64,
    pub collide: bool,
}
impl Sparks {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Sparks {
            speed: 0.0,
            spread: 0.0,
            lift: 0.0,
            life: 0.0,
            gravity: 0.0,
            drag: 0.0,
            brightness: 0.0,
            hold: 0.0,
            max_live: 0.0,
            emissive: 0.0,
            thickness: 0.0,
            shutter: 0.0,
            min_streak: 0.0,
            collide: false,
        }
    }
}
impl Default for Sparks { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/effects.ts:171 trail, src/game/effects.ts:171)
#[derive(Clone, Copy, Debug)]
pub struct Trail {
    pub radius: f64,
    pub min_voxels: f64,
    pub density: f64,
    pub temp: f64,
    pub life: f64,
    pub attack: f64,
}
impl Trail {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Trail {
            radius: 0.0,
            min_voxels: 0.0,
            density: 0.0,
            temp: 0.0,
            life: 0.0,
            attack: 0.0,
        }
    }
}
impl Default for Trail { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/effects.ts:180 vent, src/game/effects.ts:180)
#[derive(Clone, Copy, Debug)]
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
impl Vent {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Vent {
            radius: 0.0,
            density: 0.0,
            temp: 0.0,
            speed: 0.0,
            rise: 0.0,
            push: 0.0,
            expand: 0.0,
            seconds: 0.0,
            attack: 0.0,
            jitter: 0.0,
            standoff: 0.0,
            height: 0.0,
        }
    }
}
impl Default for Vent { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/effects.ts:195 muzzleFrame ->, src/game/effects.ts:195)
#[derive(Clone, Copy, Debug)]
pub struct MuzzleFrame {
    pub o: V3,
    pub x: V3,
    pub y: V3,
    pub z: V3,
}
impl MuzzleFrame {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MuzzleFrame {
            o: [0.0; 3],
            x: [0.0; 3],
            y: [0.0; 3],
            z: [0.0; 3],
        }
    }
}
impl Default for MuzzleFrame { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/effects.ts:201 spawnShot(o), src/game/effects.ts:201)
#[derive(Clone, Debug)]
pub struct SpawnShotO {
    pub seed: f64,
    pub light_gain: Option<f64>,
    pub light_range: Option<f64>,
    pub track: Option<crate::effects::Track>,
    pub eject: Option<crate::effects::Track>,
    pub gas: Option<bool>,
}
impl SpawnShotO {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SpawnShotO {
            seed: 0.0,
            light_gain: None,
            light_range: None,
            track: None,
            eject: None,
            gas: None,
        }
    }
}
impl Default for SpawnShotO { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/effects.ts:230 spawnBurst(o), src/game/effects.ts:230)
#[derive(Clone, Copy, Debug)]
pub struct SpawnBurstO {
    pub spark_count: Option<f64>,
}
impl SpawnBurstO {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SpawnBurstO {
            spark_count: None,
        }
    }
}
impl Default for SpawnBurstO { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/effects.ts:274 spawnVent(o), src/game/effects.ts:274)
#[derive(Clone, Copy, Debug)]
pub struct SpawnVentO {
    pub seconds: Option<f64>,
}
impl SpawnVentO {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SpawnVentO {
            seconds: None,
        }
    }
}
impl Default for SpawnVentO { fn default() -> Self { Self::zeroed() } }

/// (src/game/effects.ts:39 flags: a table nothing writes)
pub fn flags() -> crate::effects::Flags { crate::effects::Flags { shot_sparks: true, ejection_wisp: true, trails: true, vent_canister: false, dazzle: true } }

/// (src/game/effects.ts:53 shot: a table nothing writes)
pub fn shot() -> crate::effects::Shot { crate::effects::Shot { standoff: 0.22, jet_off: [0.0, 0.0, 0.22], port_off: [0.0, 0.0, 0.154], wisp_off: [0.0, 0.0, 0.03], jet_radius: 0.08, jet_density: 24.0, jet_temp: 120.0, jet_speed: 10.5, jet_rise: 0.2, jet_push: 90.0, jet_expand: 24.0, jet_life: 0.08, jet_attack: 0.01, ports: 2.0, port_fraction: 0.4, port_angle: 1.1, port_radius_k: 0.7, port_speed_k: 0.55, port_push_k: 0.6, port_expand_k: 0.5, wisp_radius: 0.042, wisp_density: 4.1, wisp_temp: 43.0, wisp_rise: 0.6, wisp_push: 4.0, wisp_life: 2.5, wisp_attack: 0.15, eject_radius: 0.018, eject_density: 1.6, eject_temp: 2.5, eject_life: 0.7, jitter: 0.1, light_color: [1.0, 0.62, 0.26], light_power: 90.0, light_duration: 0.09, light_range: 8.0, suppressed_gain: 0.45, spark_count: 20.0, spark_speed: 31.0, spark_cone: 0.25, spark_life: 0.06 } }

/// (src/game/effects.ts:81 muzzleFx: a table nothing writes)
pub fn muzzle_fx() -> crate::effects::MuzzleFx2 { crate::effects::MuzzleFx2 { pistol_suppressed: crate::effects::MuzzleFx { card_ahead: 0.05, card_half: [0.018, 0.018, 0.045], card_emissive: [40.0, 22.0, 8.0], light_gain: 1.0, light_range: None }, pistol_bare: crate::effects::MuzzleFx { card_ahead: 0.09, card_half: [0.03, 0.03, 0.08], card_emissive: [90.0, 50.0, 16.0], light_gain: 1.0, light_range: None }, carbine: crate::effects::MuzzleFx { card_ahead: 0.14, card_half: [0.04, 0.04, 0.12], card_emissive: [110.0, 60.0, 18.0], light_gain: 1.7, light_range: Some(10.0) }, sc20k: crate::effects::MuzzleFx { card_ahead: 0.09, card_half: [0.03, 0.03, 0.09], card_emissive: [60.0, 34.0, 12.0], light_gain: 1.2, light_range: None }, mk12: crate::effects::MuzzleFx { card_ahead: 0.07, card_half: [0.024, 0.024, 0.07], card_emissive: [44.0, 24.0, 8.0], light_gain: 1.0, light_range: None }, mp7: crate::effects::MuzzleFx { card_ahead: 0.05, card_half: [0.016, 0.016, 0.05], card_emissive: [34.0, 19.0, 7.0], light_gain: 0.85, light_range: None }, mk46: crate::effects::MuzzleFx { card_ahead: 0.11, card_half: [0.05, 0.022, 0.085], card_emissive: [95.0, 52.0, 16.0], light_gain: 1.45, light_range: Some(10.0) }, m1014: crate::effects::MuzzleFx { card_ahead: 0.15, card_half: [0.06, 0.06, 0.11], card_emissive: [120.0, 58.0, 14.0], light_gain: 2.0, light_range: Some(11.0) }, hk45: crate::effects::MuzzleFx { card_ahead: 0.045, card_half: [0.016, 0.016, 0.035], card_emissive: [26.0, 14.0, 5.0], light_gain: 0.7, light_range: None }, g18: crate::effects::MuzzleFx { card_ahead: 0.08, card_half: [0.026, 0.042, 0.07], card_emissive: [95.0, 52.0, 16.0], light_gain: 1.1, light_range: None } } }

/// (src/game/effects.ts:110 muzzleCard: a table nothing writes)
pub fn muzzle_card() -> crate::effects::MuzzleCard { crate::effects::MuzzleCard { albedo: [1.0, 0.8, 0.5], ttl: 0.035 } }

/// (src/game/effects.ts:118 bang: a table nothing writes)
pub fn bang() -> crate::effects::Bang { crate::effects::Bang { fuse: 1.5, max_air: 3.0, height: 0.07, radius: 0.23, density: 6.3, temp: 48.0, expand: 90.0, life: 0.12, attack: 0.02, end_push: 40.0, vent_speed: 7.0, half_length: 0.1, asymmetry: 0.45, tilt: 0.25, jitter: 0.6, body_vents: 5.0, body_fraction: 0.25, body_push: 30.0, body_radius_k: 0.7, body_speed_k: 0.6, body_expand_k: 0.5, wisp_radius: 0.24, wisp_density: 1.2, wisp_temp: 10.5, wisp_rise: 0.6, wisp_push: 5.0, wisp_life: 3.0, wisp_attack: 0.3, frame_jitter: 0.08, density_gain: 1.0, temp_gain: 1.0, light_color: [1.0, 0.98, 0.95], light_power: 1600.0, light_duration: 0.14, light_range: 14.0, light_radius: 0.15, light_lift: 0.3, spark_count: 72.0, trail_count: 12.0, trail_reserve: 2.0, hearing: 28.0, dazzle_radius: 8.0, dazzle_seconds: 6.0, dazzle_min: 2.0 } }

/// (src/game/effects.ts:138 sparks: a table nothing writes)
pub fn sparks() -> crate::effects::Sparks { crate::effects::Sparks { speed: 20.0, spread: 0.55, lift: 0.5, life: 0.55, gravity: 16.0, drag: 1.6, brightness: 2.6, hold: 0.15, max_live: 96.0, emissive: 25.0, thickness: 0.002, shutter: 0.017, min_streak: 0.02, collide: true } }

/// (src/game/effects.ts:156 tracerFx: a table nothing writes)
pub fn tracer_fx() -> crate::tracers::TracersParams { crate::tracers::TracersParams { speed: 280.0, min_life: 0.042, shutter: 0.011, thickness: 0.012, emissive: 60.0, min_path: 0.5, light_intensity: 4.0, light_range: 4.0, light_radius: 0.06, ember_sparks: 3.0, ember_speed: 5.0, ember_cone: 0.9, ember_life: 0.3, ember_intensity: 2.5, ember_range: 2.5, ember_ttl: 0.06, max_live: 24.0, max_lights: 6.0 } }

/// (src/game/effects.ts:171 trail: a table nothing writes)
pub fn trail() -> crate::effects::Trail { crate::effects::Trail { radius: 0.036, min_voxels: 1.0, density: 8.6, temp: 45.0, life: 0.9, attack: 0.01 } }

/// (src/game/effects.ts:180 vent: a table nothing writes)
pub fn vent() -> crate::effects::Vent { crate::effects::Vent { radius: 0.33, density: 18.0, temp: 0.0, speed: 4.5, rise: 0.15, push: 45.0, expand: 0.0, seconds: 30.0, attack: 0.12, jitter: 0.12, standoff: 0.07, height: 0.12 } }

/// (src/game/effects.ts:200 NO_GAS: a table nothing writes)
pub fn no_gas() -> crate::smoke_types::SmokeSystemLike { crate::smoke_types::SmokeSystemLike { params: None } }
