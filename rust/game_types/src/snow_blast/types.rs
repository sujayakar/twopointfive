//! The types of `snow_blast`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct SnowBlastShown {
    pub owed: Vec<crate::snow_blast::Owed>,
    pub seen: f64,
    pub turn: f64,
    pub puffs: f64,
    pub thrown: f64,
    pub throw_frames: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Owed {
    pub serial: f64,
    pub puff_left: f64,
    pub flake_left: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Live {
    pub x: f64,
    pub z: f64,
    pub size: f64,
    pub hk: f64,
    pub t: f64,
    pub serial: f64,
}

#[derive(Clone, Debug, Default)]
pub struct Tally {
    pub blasts: f64,
    pub marked: f64,
    pub craters: f64,
    pub why: String,
    pub last: Option<crate::snow_blast::TallyLast>,
    pub dug: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TallyLast {
    pub x: f64,
    pub z: f64,
    pub size: f64,
    pub hk: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Dig {
    pub x: f64,
    pub z: f64,
    pub r: f64,
    pub serial: f64,
    pub t: f64,
}

#[derive(Clone, Debug, Default)]
pub struct State {
    pub live: Vec<crate::snow_blast::Live>,
    pub digs: Vec<crate::snow_blast::Dig>,
    pub restore: bool,
    pub serial: f64,
    pub stats: crate::snow_blast::Tally,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SNOWBLAST {
    pub size_k: f64,
    pub size_min: f64,
    pub size_max: f64,
    pub high: f64,
    pub lobe: f64,
    pub rag: f64,
    pub floor: f64,
    pub rim: f64,
    pub rays: f64,
    pub ray_len: f64,
    pub ray_wide: f64,
    pub clods: f64,
    pub clod_far: f64,
    pub clod_r: f64,
    pub dig: f64,
    pub dig_s: f64,
    pub throw_s: f64,
    pub throw_share: f64,
    pub throw_r: f64,
    pub out: f64,
    pub up: f64,
    pub puff: f64,
    pub puff_r: f64,
    pub puff_life: f64,
    pub puff_size: f64,
    pub puff_out: f64,
    pub air: f64,
    pub air_r: f64,
    pub air_s: f64,
    pub flakes: f64,
    pub flake_up: f64,
    pub flake_life: f64,
}

#[derive(Clone, Debug, Default)]
pub struct Owing {
    pub live: Vec<crate::snow_blast::Live>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SnowBlastMod {
    /// (src/game/snowBlast.ts:83 states)
    pub states: Option<crate::snow_blast::State>,
    /// (src/game/snowBlast.ts:84 hooked)
    pub hooked: JsSet<crate::ObjKey>,
}
impl SnowBlastMod {
    pub fn blank() -> Self {
        SnowBlastMod {
            states: None,
            hooked: JsSet::new(),
        }
    }
}

/// (src/game/snowBlast.ts:32 SNOWBLAST: a table nothing writes)
pub fn snowblast() -> crate::snow_blast::SNOWBLAST { crate::snow_blast::SNOWBLAST { size_k: 0.26, size_min: 0.3, size_max: 1.4, high: 1.6, lobe: 0.22, rag: 0.07, floor: 1.0, rim: 2.2, rays: 13.0, ray_len: 1.9, ray_wide: 0.17, clods: 46.0, clod_far: 3.2, clod_r: 0.1, dig: 1.5, dig_s: 1.5, throw_s: 0.14, throw_share: 0.5, throw_r: 1.3, out: 7.0, up: 3.2, puff: 28.0, puff_r: 1.2, puff_life: 1.5, puff_size: 0.8, puff_out: 4.0, air: 2.2, air_r: 1.5, air_s: 0.3, flakes: 180.0, flake_up: 7.0, flake_life: 3.0 } }

/// (src/game/snowBlast.ts:77 LIVE_S)
pub const LIVE_S: f64 = 0.5;
