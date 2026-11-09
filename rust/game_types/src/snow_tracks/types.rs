//! The types of `snow_tracks`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SnowTracks {
    pub data: Vec<u8>,
    pub w: f64,
    pub h: f64,
    pub dirty: Option<[f64; 4]>,
    pub prints: f64,
    pub hollows: f64,
    pub craters: f64,
    pub rects: Vec<[f64; 4]>,
    pub marks: f64,
    pub forming: Vec<crate::snow_tracks::Forming>,
    pub kicks: Vec<crate::snow_tracks::SnowKick>,
    pub kicked: f64,
    pub unseen: JsMap<crate::Num, crate::snow_tracks::UnseenSquare>,
    pub kept: f64,
    pub found: f64,
    pub lost: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Forming {
    pub x0: f64,
    pub z0: f64,
    pub w: f64,
    pub h: f64,
    pub goal: Vec<u8>,
    pub age: f64,
    pub delay: f64,
    pub sink: f64,
    pub thrown: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SnowKick {
    pub x: f64,
    pub z: f64,
    pub dx: f64,
    pub dz: f64,
    pub force: f64,
    pub wide: f64,
    pub serial: f64,
    pub t: f64,
    pub n: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct UnseenSquare {
    pub x: f64,
    pub z: f64,
    pub marks: Vec<crate::snow_tracks::Unseen>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Unseen {
    pub k: f64,
    pub a: Vec<f64>,
    pub dark: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Stamp {
    pub dirty: Option<[f64; 4]>,
    pub h: f64,
    pub w: f64,
    pub data: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Feet {
    pub l: Option<[f64; 2]>,
    pub r: Option<[f64; 2]>,
    pub pl: Option<[f64; 2]>,
    pub pr: Option<[f64; 2]>,
    pub body: Option<[f64; 2]>,
    pub hips: Option<[f64; 2]>,
    pub lain: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CraterLook {
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
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SNOWTRACKS {
    pub foot_len: f64,
    pub foot_wide: f64,
    pub heel: f64,
    pub ankle: f64,
    pub still: f64,
    pub still_k: f64,
    pub apart: f64,
    pub body: f64,
    pub skirt: f64,
    pub body_low: f64,
    pub rest: f64,
    pub rest_v: f64,
    pub mess: f64,
    pub yaw: f64,
    pub size: f64,
    pub shallow: f64,
    pub crumble: f64,
    pub lump: f64,
    pub reach: f64,
    pub kick: f64,
    pub crumbs: f64,
    pub crumb: f64,
    pub clods: f64,
    pub clod_r: f64,
    pub clod_min: f64,
    pub clod_far: f64,
    pub drag: f64,
    pub toe_drag: f64,
    pub drag_deep: f64,
    pub run_mess: f64,
    pub run_lo: f64,
    pub run_hi: f64,
    pub run_len: f64,
    pub run_drag: f64,
    pub run_toe: f64,
    pub run_deep: f64,
    pub run_kick: f64,
    pub run_crumble: f64,
    pub run_clods: f64,
    pub run_far: f64,
    pub spray: f64,
    pub spray_far: f64,
    pub spray_fan: f64,
    pub form: f64,
    pub throw_lag: f64,
    pub throw_pop: f64,
    pub puff: f64,
    pub puff_up: f64,
    pub puff_life: f64,
    pub puff_most: f64,
    pub puff_recs: f64,
    pub puff_wait: f64,
    pub puff_keep: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StampMarkKick {
    pub x: f64,
    pub z: f64,
    pub dx: Option<f64>,
    pub dz: Option<f64>,
    pub wide: Option<f64>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SnowTracksMod {
    /// (src/game/snowTracks.ts:165 feetOf)
    pub feet_of: JsMap<CharId, crate::snow_tracks::Feet>,
    /// (src/game/snowTracks.ts:178 sheets)
    pub sheets: Option<[crate::snow_tracks::Stamp; 2]>,
}
impl SnowTracksMod {
    pub fn blank() -> Self {
        SnowTracksMod {
            feet_of: JsMap::new(),
            sheets: None,
        }
    }
}

/// (src/game/snowTracks.ts:29 TRACK_RES)
pub const TRACK_RES: f64 = 32.0;

/// (src/game/snowTracks.ts:32 TRACK_PAD)
pub const TRACK_PAD: f64 = 3.0;

/// (src/game/snowTracks.ts:34 SNOWTRACKS: a table nothing writes)
pub fn snowtracks() -> crate::snow_tracks::SNOWTRACKS { crate::snow_tracks::SNOWTRACKS { foot_len: 0.16, foot_wide: 0.062, heel: 0.72, ankle: 0.22, still: 0.12, still_k: 0.6, apart: 0.11, body: 0.19, skirt: 0.11, body_low: 0.38, rest: 0.5, rest_v: 0.25, mess: 1.0, yaw: 0.2, size: 0.12, shallow: 0.26, crumble: 0.018, lump: 0.06, reach: 0.75, kick: 0.9, crumbs: 0.3, crumb: 0.045, clods: 10.0, clod_r: 0.06, clod_min: 0.04, clod_far: 0.22, drag: 0.2, toe_drag: 0.5, drag_deep: 0.3, run_mess: 1.0, run_lo: 2.4, run_hi: 4.4, run_len: 0.35, run_drag: 1.8, run_toe: 3.0, run_deep: 0.8, run_kick: 1.0, run_crumble: 1.0, run_clods: 1.2, run_far: 1.4, spray: 12.0, spray_far: 0.85, spray_fan: 0.5, form: 0.2, throw_lag: 0.16, throw_pop: 0.07, puff: 14.0, puff_up: 2.6, puff_life: 0.9, puff_most: 48.0, puff_recs: 3.0, puff_wait: 12.0, puff_keep: 0.25 } }

/// (src/game/snowTracks.ts:136 UNSEEN_SQUARE)
pub const UNSEEN_SQUARE: f64 = 6.0;

/// (src/game/snowTracks.ts:136 UNSEEN_MOST)
pub const UNSEEN_MOST: f64 = 6000.0;

/// (src/game/snowTracks.ts:136 UNSEEN_ASKS)
pub const UNSEEN_ASKS: f64 = 64.0;

/// (src/game/snowTracks.ts:136 UNSEEN_LAYS)
pub const UNSEEN_LAYS: f64 = 16.0;

/// (src/game/snowTracks.ts:147 KICKS_MOST)
pub const KICKS_MOST: f64 = 256.0;
