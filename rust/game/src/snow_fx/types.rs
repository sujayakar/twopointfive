//! The types of `snow_fx`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: SnowState, src/game/snowFx.ts:131)
#[derive(Clone, Debug)]
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
impl SnowState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnowState {
            seed: 0.0,
            serial: 0.0,
            birth_acc: 0.0,
            pop: 0.0,
            last_emitted: 0.0,
            fill_at: 0.0,
            was_active: false,
            band_off: [0.0; 2],
            stats: crate::snow_fx::SnowStats::zeroed(),
            w_serial: 0.0,
            w_acc: 0.0,
            flake_filed: 0.0,
            wisps: crate::snow_fx::WispStats::zeroed(),
            g_serial: 0.0,
            g_acc: 0.0,
            g_fill: 0.0,
            g_hold: 0.0,
            g_seen: false,
            grains: crate::snow_fx::GrainStats::zeroed(),
            one_air: false,
            kick_seen: 0.0,
            kick_q: Vec::new(),
        }
    }
}
impl Default for SnowState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SnowStats, src/game/snowFx.ts:130)
#[derive(Clone, Debug)]
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
impl SnowStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnowStats {
            active: false,
            why: String::new(),
            radius: 0.0,
            centre: [0.0; 2],
            area_out: 0.0,
            target: 0.0,
            capped: false,
            pop: 0.0,
            births: 0.0,
            total_births: 0.0,
            frames: 0.0,
        }
    }
}
impl Default for SnowStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: WispStats, src/game/snowFx.ts:128)
#[derive(Clone, Debug)]
pub struct WispStats {
    pub active: bool,
    pub why: String,
    pub target: f64,
    pub blow: f64,
    pub births: f64,
    pub total_births: f64,
    pub records: f64,
}
impl WispStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WispStats {
            active: false,
            why: String::new(),
            target: 0.0,
            blow: 0.0,
            births: 0.0,
            total_births: 0.0,
            records: 0.0,
        }
    }
}
impl Default for WispStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: GrainStats, src/game/snowFx.ts:129)
#[derive(Clone, Debug)]
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
impl GrainStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GrainStats {
            active: false,
            why: String::new(),
            target: 0.0,
            pop: 0.0,
            births: 0.0,
            total_births: 0.0,
            records: 0.0,
            radius: 0.0,
            air: false,
            u0: 0.0,
            wind: [0.0; 2],
        }
    }
}
impl Default for GrainStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snowFx.ts:51 SNOWFX, src/game/snowFx.ts:51)
#[derive(Clone, Copy, Debug)]
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
impl SNOWFX {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SNOWFX {
            per_m2: 0.0,
            fall_v: 0.0,
            size: 0.0,
            size_jit: 0.0,
            flutter: 0.0,
            flutter_rate: 0.0,
            iso: 0.0,
            fwd_k: 0.0,
            fwd_g: 0.0,
            gain: 0.0,
            sky_k: 0.0,
            bright_jit: 0.0,
            min_px: 0.0,
            max_px: 0.0,
            boost_cap: 0.0,
            near_fade_m: 0.0,
            fade_in: 0.0,
            shutter: 0.0,
            wind_k: 0.0,
            sheet_amp: 0.0,
            sheet_len: 0.0,
            sheet_drift: 0.0,
            radius_k: 0.0,
            radius_min: 0.0,
            radius_max: 0.0,
            past: 0.0,
            air_low: 0.0,
            air_high: 0.0,
            air_lift: 0.0,
            skid: 0.0,
        }
    }
}
impl Default for SNOWFX { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snowFx.ts:79 SNOWWISPS, src/game/snowFx.ts:79)
#[derive(Clone, Copy, Debug)]
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
impl SNOWWISPS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SNOWWISPS {
            per_m2: 0.0,
            max: 0.0,
            life: 0.0,
            life_jit: 0.0,
            y0: 0.0,
            y1: 0.0,
            size: 0.0,
            size_jit: 0.0,
            speed_k: 0.0,
            stretch: 0.0,
            motes: 0.0,
            sparkle: 0.0,
            opacity: 0.0,
            gain: 0.0,
            meander: 0.0,
            gust_len: 0.0,
            gust_pow: 0.0,
            min_air: 0.0,
            lift_lo: 0.0,
            lift_hi: 0.0,
            drop_k: 0.0,
        }
    }
}
impl Default for SNOWWISPS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snowFx.ts:106 SNOWGRAINS, src/game/snowFx.ts:106)
#[derive(Clone, Copy, Debug)]
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
impl SNOWGRAINS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SNOWGRAINS {
            max: 0.0,
            radius: 0.0,
            size: 0.0,
            size_jit: 0.0,
            speed_k: 0.0,
            opacity: 0.0,
            gain: 0.0,
            eddy: 0.0,
            lift: 0.0,
            wisps_keep: 0.0,
            wisps_back: 0.0,
            min_air: 0.0,
            shutter: 0.0,
            dash: 0.0,
            tail: 0.0,
        }
    }
}
impl Default for SNOWGRAINS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snowFx.ts:366 SNOWPUFF, src/game/snowFx.ts:366)
#[derive(Clone, Copy, Debug)]
pub struct SNOWPUFF {
    pub least: f64,
    pub most: f64,
}
impl SNOWPUFF {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SNOWPUFF {
            least: 0.0,
            most: 0.0,
        }
    }
}
impl Default for SNOWPUFF { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snowFx.ts:384 kicksDue(T), src/game/snowFx.ts:384)
#[derive(Clone, Debug)]
pub struct KicksDueT {
    pub kicks: Vec<crate::snow_tracks::SnowKick>,
    pub kicked: f64,
}
impl KicksDueT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KicksDueT {
            kicks: Vec::new(),
            kicked: 0.0,
        }
    }
}
impl Default for KicksDueT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snowFx.ts:384 kicksDue(B), src/game/snowFx.ts:384)
#[derive(Clone, Debug)]
pub struct KicksDueB {
    pub kick_seen: f64,
    pub kick_q: Vec<crate::snow_tracks::SnowKick>,
}
impl KicksDueB {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KicksDueB {
            kick_seen: 0.0,
            kick_q: Vec::new(),
        }
    }
}
impl Default for KicksDueB { fn default() -> Self { Self::zeroed() } }

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
