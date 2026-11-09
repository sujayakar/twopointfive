//! The types of `sim_weatherhome`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct RoofsCountedBoxesItem {
    pub c: Vec<f64>,
    pub h: Vec<f64>,
}

/// (src/sim/weatherhome.ts:8 WX: a table nothing writes)
pub fn wx() -> crate::sim_weatherhome::WX { crate::sim_weatherhome::WX { live: 0.0, storm: 1.0, rain_rate: 2.0, rain_now: 3.0, wet: 4.0, thunder_until: 5.0, next_flash_at: 6.0, last_bark_t: 7.0, fog_on: 8.0, fog_sigma: 9.0, fog_h: 10.0, fog_leak: 11.0, roofs_n: 12.0, roofs: 13.0, thunders_n: 14.0, thunders: 15.0, dice: 16.0, air_on: 20.0, air_leak: 21.0, air_acc: 22.0, air_cur: 23.0, air_cells: 24.0, air_edges: 25.0, air_s: 26.0, air_stat: 30.0 } }

/// (src/sim/weatherhome.ts:11 WX_BIT)
pub const WX_BIT: f64 = 1024.0;

