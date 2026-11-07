//! The types of `rain_fx`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: RainState, src/game/rainFx.ts:116)
#[derive(Clone, Debug)]
pub struct RainState {
    pub seed: f64,
    pub birth_acc: f64,
    pub drip_acc: f64,
    pub was_active: bool,
    pub edges: Option<Vec<crate::rain_fx::DripPoint>>,
    pub edge_boxes: f64,
    pub job: Option<crate::rain_fx::DripJob>,
    pub kind: Option<String>,
    pub stats: crate::rain_fx::RainStatsCpu,
    pub serial: f64,
    pub pop: f64,
    pub pop_log: Vec<crate::rain_fx::RainStatePopLogItem>,
    pub clock: f64,
    pub last_c: Option<[f64; 2]>,
    pub drip_life_max: f64,
    pub shear: [f64; 2],
    pub band_off: [f64; 2],
    pub air: [f64; 2],
    pub shear_init: bool,
    pub aside: crate::scene_collision::AsideOwn,
}
impl RainState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RainState {
            seed: 0.0,
            birth_acc: 0.0,
            drip_acc: 0.0,
            was_active: false,
            edges: None,
            edge_boxes: 0.0,
            job: None,
            kind: None,
            stats: crate::rain_fx::RainStatsCpu::zeroed(),
            serial: 0.0,
            pop: 0.0,
            pop_log: Vec::new(),
            clock: 0.0,
            last_c: None,
            drip_life_max: 0.0,
            shear: [0.0; 2],
            band_off: [0.0; 2],
            air: [0.0; 2],
            shear_init: false,
            aside: crate::scene_collision::AsideOwn::zeroed(),
        }
    }
}
impl Default for RainState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DripPoint, src/game/rainFx.ts:110)
#[derive(Clone, Copy, Debug)]
pub struct DripPoint {
    pub pos: V3,
    pub fall: f64,
    pub box_: f64,
}
impl DripPoint {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DripPoint {
            pos: [0.0; 3],
            fall: 0.0,
            box_: 0.0,
        }
    }
}
impl Default for DripPoint { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DripJob, src/game/rainFx.ts:114)
#[derive(Clone, Debug)]
pub struct DripJob {
    pub boxes: f64,
    pub serial: f64,
    pub left: f64,
    pub hold: f64,
    pub bi: f64,
    pub ei: f64,
    pub k: f64,
    pub paid: bool,
    pub out: Vec<crate::rain_fx::DripPoint>,
}
impl DripJob {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DripJob {
            boxes: 0.0,
            serial: 0.0,
            left: 0.0,
            hold: 0.0,
            bi: 0.0,
            ei: 0.0,
            k: 0.0,
            paid: false,
            out: Vec::new(),
        }
    }
}
impl Default for DripJob { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RainStatsCpu, src/game/rainFx.ts:129)
#[derive(Clone, Debug)]
pub struct RainStatsCpu {
    pub active: bool,
    pub why: String,
    pub radius: f64,
    pub centre: [f64; 2],
    pub area_out: f64,
    pub lattice_per_m2: f64,
    pub target: f64,
    pub capped: bool,
    pub keep: f64,
    pub fill: f64,
    pub births_per_sec: f64,
    pub births: f64,
    pub drips: f64,
    pub edges_active: f64,
    pub edges_total: f64,
    pub frames: f64,
    pub total_births: f64,
    pub total_drips: f64,
}
impl RainStatsCpu {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RainStatsCpu {
            active: false,
            why: String::new(),
            radius: 0.0,
            centre: [0.0; 2],
            area_out: 0.0,
            lattice_per_m2: 0.0,
            target: 0.0,
            capped: false,
            keep: 0.0,
            fill: 0.0,
            births_per_sec: 0.0,
            births: 0.0,
            drips: 0.0,
            edges_active: 0.0,
            edges_total: 0.0,
            frames: 0.0,
            total_births: 0.0,
            total_drips: 0.0,
        }
    }
}
impl Default for RainStatsCpu { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rainFx.ts:121 RainState.popLog, src/game/rainFx.ts:121)
#[derive(Clone, Copy, Debug)]
pub struct RainStatePopLogItem {
    pub t: f64,
    pub dt: f64,
    pub n: f64,
}
impl RainStatePopLogItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RainStatePopLogItem {
            t: 0.0,
            dt: 0.0,
            n: 0.0,
        }
    }
}
impl Default for RainStatePopLogItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet RainState.aside, src/game/rainFx.ts:127)
#[derive(Clone, Copy, Debug)]
pub struct RainStateAside {
    pub s: Option<crate::rain_fx::RainStateAsideS>,
    pub q: Option<crate::rain_fx::RainStateAsideQ>,
}
impl RainStateAside {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RainStateAside {
            s: None,
            q: None,
        }
    }
}
impl Default for RainStateAside { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rainFx.ts:127 RainStateAside.s, src/game/rainFx.ts:127 RainStateAside.s)
#[derive(Clone, Copy, Debug)]
pub struct RainStateAsideS {
}
impl RainStateAsideS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RainStateAsideS {
        }
    }
}
impl Default for RainStateAsideS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rainFx.ts:127 RainStateAside.q, src/game/rainFx.ts:127 RainStateAside.q)
#[derive(Clone, Copy, Debug)]
pub struct RainStateAsideQ {
}
impl RainStateAsideQ {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RainStateAsideQ {
        }
    }
}
impl Default for RainStateAsideQ { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rainFx.ts:42 PRAIN, src/game/rainFx.ts:42)
#[derive(Clone, Copy, Debug)]
pub struct RainFxModPRAIN {
    pub iso: f64,
    pub fwd_k: f64,
    pub fwd_g: f64,
    pub glint_k: f64,
    pub glint_rough: f64,
    pub glint_cap: f64,
    pub gain: f64,
    pub tail_keep: f64,
    pub refract: f64,
    pub refract_reach_m: f64,
    pub refract_taps: f64,
    pub refract_clamp: f64,
    pub refract_depth: f64,
    pub refract_paint: f64,
    pub len_jit: f64,
    pub bright_jit: f64,
    pub min_px: f64,
    pub max_px: f64,
    pub boost_cap: f64,
    pub near_fade_m: f64,
    pub bead_amp: f64,
    pub wind_k: f64,
    pub lattice_lean: f64,
    pub sheet_amp: f64,
    pub sheet_len: f64,
    pub sheet_drift: f64,
}
impl RainFxModPRAIN {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RainFxModPRAIN {
            iso: 0.0,
            fwd_k: 0.0,
            fwd_g: 0.0,
            glint_k: 0.0,
            glint_rough: 0.0,
            glint_cap: 0.0,
            gain: 0.0,
            tail_keep: 0.0,
            refract: 0.0,
            refract_reach_m: 0.0,
            refract_taps: 0.0,
            refract_clamp: 0.0,
            refract_depth: 0.0,
            refract_paint: 0.0,
            len_jit: 0.0,
            bright_jit: 0.0,
            min_px: 0.0,
            max_px: 0.0,
            boost_cap: 0.0,
            near_fade_m: 0.0,
            bead_amp: 0.0,
            wind_k: 0.0,
            lattice_lean: 0.0,
            sheet_amp: 0.0,
            sheet_len: 0.0,
            sheet_drift: 0.0,
        }
    }
}
impl Default for RainFxModPRAIN { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rainFx.ts:91 RAIN_FX, src/game/rainFx.ts:91)
#[derive(Clone, Copy, Debug)]
pub struct RAINFX {
    pub lattice_cell: f64,
    pub lattice_period: f64,
    pub fall_v: f64,
    pub radius_k: f64,
    pub radius_min: f64,
    pub radius_max: f64,
    pub size: f64,
    pub size_jit: f64,
    pub top_jit: f64,
    pub speed_jit: f64,
    pub splash_secs: f64,
    pub drip_per_point_sec: f64,
    pub drip_size: f64,
    pub drip_speed: f64,
    pub drip_max_per_frame: f64,
    pub edge_step: f64,
    pub edge_min_top: f64,
    pub edge_max_top: f64,
    pub edge_min_area: f64,
    pub edge_min_fall: f64,
}
impl RAINFX {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RAINFX {
            lattice_cell: 0.0,
            lattice_period: 0.0,
            fall_v: 0.0,
            radius_k: 0.0,
            radius_min: 0.0,
            radius_max: 0.0,
            size: 0.0,
            size_jit: 0.0,
            top_jit: 0.0,
            speed_jit: 0.0,
            splash_secs: 0.0,
            drip_per_point_sec: 0.0,
            drip_size: 0.0,
            drip_speed: 0.0,
            drip_max_per_frame: 0.0,
            edge_step: 0.0,
            edge_min_top: 0.0,
            edge_max_top: 0.0,
            edge_min_area: 0.0,
            edge_min_fall: 0.0,
        }
    }
}
impl Default for RAINFX { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct RainFxMod {
    /// (src/game/rainFx.ts:42 PRAIN)
    pub prain: crate::rain_fx::RainFxModPRAIN,
}
impl RainFxMod {
    pub fn blank() -> Self {
        RainFxMod {
            prain: crate::rain_fx::RainFxModPRAIN { iso: 0.25, fwd_k: 0.75, fwd_g: 0.55, glint_k: 1.0, glint_rough: 0.08, glint_cap: 12.0, gain: 1.9, tail_keep: 1.0, refract: 1.0, refract_reach_m: 6.0, refract_taps: 3.0, refract_clamp: 6.0, refract_depth: 0.3, refract_paint: 0.0, len_jit: 0.45, bright_jit: 0.7, min_px: 0.75, max_px: 1.1, boost_cap: 2.5, near_fade_m: 2.6, bead_amp: 0.4, wind_k: 1.0, lattice_lean: 0.92, sheet_amp: 0.5, sheet_len: 7.0, sheet_drift: 1.0 },
        }
    }
}

/// (src/game/rainFx.ts:91 RAIN_FX: a table nothing writes)
pub fn rain_fx() -> crate::rain_fx::RAINFX { crate::rain_fx::RAINFX { lattice_cell: 0.45, lattice_period: 2.2, fall_v: 9.0, radius_k: 0.6, radius_min: 8.0, radius_max: 30.0, size: 0.008, size_jit: 0.5, top_jit: 0.9, speed_jit: 0.06, splash_secs: 0.2, drip_per_point_sec: 0.35, drip_size: 2.6, drip_speed: 0.35, drip_max_per_frame: 4.0, edge_step: 0.3, edge_min_top: 0.9, edge_max_top: 7.6, edge_min_area: 0.12, edge_min_fall: 0.35 } }

/// (src/game/rainFx.ts:149 ZERO4: a table nothing writes)
pub fn zero4() -> [f64; 4] { [0.0, 0.0, 0.0, 0.0] }

/// (src/game/rainFx.ts:262 dripScratch: a table nothing writes)
pub fn drip_scratch() -> Vec<crate::rain_fx::DripPoint> { Vec::new() }

/// (src/game/rainFx.ts:263 DOWN: a table nothing writes)
pub fn down() -> V3 { [0.0, -1.0, 0.0] }
