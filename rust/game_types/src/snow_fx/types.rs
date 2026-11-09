//! The types of `snow_fx`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct SnowState {
    pub seed: f64,
    pub serial: f64,
    pub birth_acc: f64,
    pub pop: f64,
    pub last_emitted: f64,
    pub fill_at: f64,
    pub was_active: bool,
    pub band_off: [f64; 2],
    pub stats: crate::snow_fx::SnowStats,
    pub w_serial: f64,
    pub w_acc: f64,
    pub flake_filed: f64,
    pub wisps: crate::snow_fx::WispStats,
    pub g_serial: f64,
    pub g_acc: f64,
    pub g_fill: f64,
    pub g_hold: f64,
    pub g_seen: bool,
    pub grains: crate::snow_fx::GrainStats,
    pub one_air: bool,
    pub kick_seen: f64,
    pub kick_q: Vec<crate::snow_tracks::SnowKick>,
}

#[derive(Clone, Debug, Default)]
pub struct SnowStats {
    pub active: bool,
    pub why: String,
    pub radius: f64,
    pub centre: [f64; 2],
    pub area_out: f64,
    pub target: f64,
    pub capped: bool,
    pub pop: f64,
    pub births: f64,
    pub total_births: f64,
    pub frames: f64,
}

#[derive(Clone, Debug, Default)]
pub struct WispStats {
    pub active: bool,
    pub why: String,
    pub target: f64,
    pub blow: f64,
    pub births: f64,
    pub total_births: f64,
    pub records: f64,
}

#[derive(Clone, Debug, Default)]
pub struct GrainStats {
    pub active: bool,
    pub why: String,
    pub target: f64,
    pub pop: f64,
    pub births: f64,
    pub total_births: f64,
    pub records: f64,
    pub radius: f64,
    pub air: bool,
    pub u0: f64,
    pub wind: [f64; 2],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SNOWFX {
    pub per_m2: f64,
    pub fall_v: f64,
    pub size: f64,
    pub size_jit: f64,
    pub flutter: f64,
    pub flutter_rate: f64,
    pub iso: f64,
    pub fwd_k: f64,
    pub fwd_g: f64,
    pub gain: f64,
    pub sky_k: f64,
    pub bright_jit: f64,
    pub min_px: f64,
    pub max_px: f64,
    pub boost_cap: f64,
    pub near_fade_m: f64,
    pub fade_in: f64,
    pub shutter: f64,
    pub wind_k: f64,
    pub sheet_amp: f64,
    pub sheet_len: f64,
    pub sheet_drift: f64,
    pub radius_k: f64,
    pub radius_min: f64,
    pub radius_max: f64,
    pub past: f64,
    pub air_low: f64,
    pub air_high: f64,
    pub air_lift: f64,
    pub skid: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SNOWWISPS {
    pub per_m2: f64,
    pub max: f64,
    pub life: f64,
    pub life_jit: f64,
    pub y0: f64,
    pub y1: f64,
    pub size: f64,
    pub size_jit: f64,
    pub speed_k: f64,
    pub stretch: f64,
    pub motes: f64,
    pub sparkle: f64,
    pub opacity: f64,
    pub gain: f64,
    pub meander: f64,
    pub gust_len: f64,
    pub gust_pow: f64,
    pub min_air: f64,
    pub lift_lo: f64,
    pub lift_hi: f64,
    pub drop_k: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SNOWGRAINS {
    pub max: f64,
    pub radius: f64,
    pub size: f64,
    pub size_jit: f64,
    pub speed_k: f64,
    pub opacity: f64,
    pub gain: f64,
    pub eddy: f64,
    pub lift: f64,
    pub wisps_keep: f64,
    pub wisps_back: f64,
    pub min_air: f64,
    pub shutter: f64,
    pub dash: f64,
    pub tail: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SNOWPUFF {
    pub least: f64,
    pub most: f64,
}

#[derive(Clone, Debug, Default)]
pub struct KicksDueT {
    pub kicks: Vec<crate::snow_tracks::SnowKick>,
    pub kicked: f64,
}

#[derive(Clone, Debug, Default)]
pub struct KicksDueB {
    pub kick_seen: f64,
    pub kick_q: Vec<crate::snow_tracks::SnowKick>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SnowFxMod {
    /// (src/game/snowFx.ts:356 markKicked)
    pub mark_kicked: f64,
    /// (src/game/snowFx.ts:370 bootPuffed)
    pub boot_puffed: f64,
}
impl SnowFxMod {
    pub fn blank() -> Self {
        SnowFxMod {
            mark_kicked: 0.0,
            boot_puffed: 0.0,
        }
    }
}

/// (src/game/snowFx.ts:51 SNOWFX: a table nothing writes)
pub fn snowfx() -> crate::snow_fx::SNOWFX { crate::snow_fx::SNOWFX { per_m2: 34.6, fall_v: 1.25, size: 0.028, size_jit: 0.6, flutter: 0.41, flutter_rate: 2.2, iso: 0.26, fwd_k: 0.25, fwd_g: 0.45, gain: 9.0, sky_k: 1.5, bright_jit: 0.5, min_px: 1.0, max_px: 4.0, boost_cap: 1.5, near_fade_m: 2.6, fade_in: 0.4, shutter: 0.008333333333333333, wind_k: 1.0, sheet_amp: 0.3, sheet_len: 9.0, sheet_drift: 1.0, radius_k: 0.75, radius_min: 10.0, radius_max: 30.0, past: 12.0, air_low: 2.0, air_high: 3.4, air_lift: 1.0, skid: 0.75 } }

/// (src/game/snowFx.ts:79 SNOWWISPS: a table nothing writes)
pub fn snowwisps() -> crate::snow_fx::SNOWWISPS { crate::snow_fx::SNOWWISPS { per_m2: 3.0, max: 1000.0, life: 4.5, life_jit: 0.25, y0: 0.04, y1: 0.42, size: 0.6, size_jit: 0.5, speed_k: 5.0, stretch: 0.12, motes: 0.55, sparkle: 0.5, opacity: 0.3, gain: 2.0, meander: 0.8, gust_len: 5.0, gust_pow: 3.0, min_air: 0.15, lift_lo: 0.42, lift_hi: 0.7, drop_k: 0.7 } }

/// (src/game/snowFx.ts:106 SNOWGRAINS: a table nothing writes)
pub fn snowgrains() -> crate::snow_fx::SNOWGRAINS { crate::snow_fx::SNOWGRAINS { max: 2600.0, radius: 9.0, size: 0.02, size_jit: 0.5, speed_k: 5.0, opacity: 0.5, gain: 1.5, eddy: 0.45, lift: 1.0, wisps_keep: 0.6, wisps_back: 0.0, min_air: 0.15, shutter: 0.008, dash: 3.0, tail: 0.8 } }

/// (src/game/snowFx.ts:217 PAST_TAPER: a table nothing writes)
pub fn past_taper() -> [f64; 2] { [0.5, 0.8] }

/// (src/game/snowFx.ts:227 DOWN: a table nothing writes)
pub fn down() -> V3 { [0.0, -1.0, 0.0] }

/// (src/game/snowFx.ts:366 SNOWPUFF: a table nothing writes)
pub fn snowpuff() -> crate::snow_fx::SNOWPUFF { crate::snow_fx::SNOWPUFF { least: 0.2, most: 2.0 } }
