//! The types of `sim_weatherhome`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/sim/weatherhome.ts:8 WX, src/sim/weatherhome.ts:8)
#[derive(Clone, Copy, Debug)]
pub struct WX {
    pub live: f64,
    pub storm: f64,
    pub rain_rate: f64,
    pub rain_now: f64,
    pub wet: f64,
    pub thunder_until: f64,
    pub next_flash_at: f64,
    pub last_bark_t: f64,
    pub fog_on: f64,
    pub fog_sigma: f64,
    pub fog_h: f64,
    pub fog_leak: f64,
    pub roofs_n: f64,
    pub roofs: f64,
    pub thunders_n: f64,
    pub thunders: f64,
    pub dice: f64,
    pub air_on: f64,
    pub air_leak: f64,
    pub air_acc: f64,
    pub air_cur: f64,
    pub air_cells: f64,
    pub air_edges: f64,
    pub air_s: f64,
    pub air_stat: f64,
}
impl WX {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WX {
            live: 0.0,
            storm: 0.0,
            rain_rate: 0.0,
            rain_now: 0.0,
            wet: 0.0,
            thunder_until: 0.0,
            next_flash_at: 0.0,
            last_bark_t: 0.0,
            fog_on: 0.0,
            fog_sigma: 0.0,
            fog_h: 0.0,
            fog_leak: 0.0,
            roofs_n: 0.0,
            roofs: 0.0,
            thunders_n: 0.0,
            thunders: 0.0,
            dice: 0.0,
            air_on: 0.0,
            air_leak: 0.0,
            air_acc: 0.0,
            air_cur: 0.0,
            air_cells: 0.0,
            air_edges: 0.0,
            air_s: 0.0,
            air_stat: 0.0,
        }
    }
}
impl Default for WX { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/weatherhome.ts:29 WX_KNOBS, src/sim/weatherhome.ts:29)
#[derive(Clone, Copy, Debug)]
pub struct WXKNOBS {
    pub here: crate::FnField,
}
impl WXKNOBS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WXKNOBS {
            here: crate::FnField,
        }
    }
}
impl Default for WXKNOBS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/weatherhome.ts:33 WX_WATCH, src/sim/weatherhome.ts:33)
#[derive(Clone, Copy, Debug)]
pub struct WXWATCH {
    pub on: bool,
}
impl WXWATCH {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WXWATCH {
            on: false,
        }
    }
}
impl Default for WXWATCH { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/weatherhome.ts:24 roofsCounted(boxes), src/sim/weatherhome.ts:24)
#[derive(Clone, Debug)]
pub struct RoofsCountedBoxesItem {
    pub c: Vec<f64>,
    pub h: Vec<f64>,
}
impl RoofsCountedBoxesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoofsCountedBoxesItem {
            c: Vec::new(),
            h: Vec::new(),
        }
    }
}
impl Default for RoofsCountedBoxesItem { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SimWeatherhomeMod {
    /// (src/sim/weatherhome.ts:12 OFF)
    pub off: Vec<f64>,
}
impl SimWeatherhomeMod {
    pub fn blank() -> Self {
        SimWeatherhomeMod {
            off: Vec::new(),
        }
    }
}

/// (src/sim/weatherhome.ts:8 WX: a table nothing writes)
pub fn wx() -> crate::sim_weatherhome::WX { crate::sim_weatherhome::WX { live: 0.0, storm: 1.0, rain_rate: 2.0, rain_now: 3.0, wet: 4.0, thunder_until: 5.0, next_flash_at: 6.0, last_bark_t: 7.0, fog_on: 8.0, fog_sigma: 9.0, fog_h: 10.0, fog_leak: 11.0, roofs_n: 12.0, roofs: 13.0, thunders_n: 14.0, thunders: 15.0, dice: 16.0, air_on: 20.0, air_leak: 21.0, air_acc: 22.0, air_cur: 23.0, air_cells: 24.0, air_edges: 25.0, air_s: 26.0, air_stat: 30.0 } }

/// (src/sim/weatherhome.ts:11 WX_BIT)
pub const WX_BIT: f64 = 1024.0;

/// (src/sim/weatherhome.ts:29 WX_KNOBS: a table nothing writes)
pub fn wx_knobs() -> crate::sim_weatherhome::WXKNOBS { crate::sim_weatherhome::WXKNOBS { here: crate::FnField } }

/// (src/sim/weatherhome.ts:33 WX_WATCH: a table nothing writes)
pub fn wx_watch() -> crate::sim_weatherhome::WXWATCH { crate::sim_weatherhome::WXWATCH { on: false } }
