//! The types of `snow_tracks`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: SnowTracks, src/game/snowTracks.ts:98)
#[derive(Clone, Debug)]
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
impl SnowTracks {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnowTracks {
            data: Vec::new(),
            w: 0.0,
            h: 0.0,
            dirty: None,
            prints: 0.0,
            hollows: 0.0,
            craters: 0.0,
            rects: Vec::new(),
            marks: 0.0,
            forming: Vec::new(),
            kicks: Vec::new(),
            kicked: 0.0,
            unseen: JsMap::new(),
            kept: 0.0,
            found: 0.0,
            lost: 0.0,
        }
    }
}
impl Default for SnowTracks { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Forming, src/game/snowTracks.ts:143)
#[derive(Clone, Debug)]
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
impl Forming {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Forming {
            x0: 0.0,
            z0: 0.0,
            w: 0.0,
            h: 0.0,
            goal: Vec::new(),
            age: 0.0,
            delay: 0.0,
            sink: 0.0,
            thrown: false,
        }
    }
}
impl Default for Forming { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SnowKick, src/game/snowTracks.ts:148)
#[derive(Clone, Copy, Debug)]
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
impl SnowKick {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnowKick {
            x: 0.0,
            z: 0.0,
            dx: 0.0,
            dz: 0.0,
            force: 0.0,
            wide: 0.0,
            serial: 0.0,
            t: 0.0,
            n: 0.0,
        }
    }
}
impl Default for SnowKick { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: UnseenSquare, src/game/snowTracks.ts:131)
#[derive(Clone, Debug)]
pub struct UnseenSquare {
    pub x: f64,
    pub z: f64,
    pub marks: Vec<crate::snow_tracks::Unseen>,
}
impl UnseenSquare {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        UnseenSquare {
            x: 0.0,
            z: 0.0,
            marks: Vec::new(),
        }
    }
}
impl Default for UnseenSquare { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Unseen, src/game/snowTracks.ts:128)
#[derive(Clone, Debug)]
pub struct Unseen {
    pub k: f64,
    pub a: Vec<f64>,
    pub dark: bool,
}
impl Unseen {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Unseen {
            k: 0.0,
            a: Vec::new(),
            dark: false,
        }
    }
}
impl Default for Unseen { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Stamp, src/game/snowTracks.ts:140)
#[derive(Clone, Debug)]
pub struct Stamp {
    pub dirty: Option<[f64; 4]>,
    pub h: f64,
    pub w: f64,
    pub data: Vec<u8>,
}
impl Stamp {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Stamp {
            dirty: None,
            h: 0.0,
            w: 0.0,
            data: Vec::new(),
        }
    }
}
impl Default for Stamp { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Feet, src/game/snowTracks.ts:164)
#[derive(Clone, Copy, Debug)]
pub struct Feet {
    pub l: Option<[f64; 2]>,
    pub r: Option<[f64; 2]>,
    pub pl: Option<[f64; 2]>,
    pub pr: Option<[f64; 2]>,
    pub body: Option<[f64; 2]>,
    pub hips: Option<[f64; 2]>,
    pub lain: f64,
}
impl Feet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Feet {
            l: None,
            r: None,
            pl: None,
            pr: None,
            body: None,
            hips: None,
            lain: 0.0,
        }
    }
}
impl Default for Feet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CraterLook, src/game/snowTracks.ts:354)
#[derive(Clone, Copy, Debug)]
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
impl CraterLook {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CraterLook {
            lobe: 0.0,
            rag: 0.0,
            floor: 0.0,
            rim: 0.0,
            rays: 0.0,
            ray_len: 0.0,
            ray_wide: 0.0,
            clods: 0.0,
            clod_far: 0.0,
            clod_r: 0.0,
        }
    }
}
impl Default for CraterLook { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snowTracks.ts:34 SNOWTRACKS, src/game/snowTracks.ts:34)
#[derive(Clone, Copy, Debug)]
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
impl SNOWTRACKS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SNOWTRACKS {
            foot_len: 0.0,
            foot_wide: 0.0,
            heel: 0.0,
            ankle: 0.0,
            still: 0.0,
            still_k: 0.0,
            apart: 0.0,
            body: 0.0,
            skirt: 0.0,
            body_low: 0.0,
            rest: 0.0,
            rest_v: 0.0,
            mess: 0.0,
            yaw: 0.0,
            size: 0.0,
            shallow: 0.0,
            crumble: 0.0,
            lump: 0.0,
            reach: 0.0,
            kick: 0.0,
            crumbs: 0.0,
            crumb: 0.0,
            clods: 0.0,
            clod_r: 0.0,
            clod_min: 0.0,
            clod_far: 0.0,
            drag: 0.0,
            toe_drag: 0.0,
            drag_deep: 0.0,
            run_mess: 0.0,
            run_lo: 0.0,
            run_hi: 0.0,
            run_len: 0.0,
            run_drag: 0.0,
            run_toe: 0.0,
            run_deep: 0.0,
            run_kick: 0.0,
            run_crumble: 0.0,
            run_clods: 0.0,
            run_far: 0.0,
            spray: 0.0,
            spray_far: 0.0,
            spray_fan: 0.0,
            form: 0.0,
            throw_lag: 0.0,
            throw_pop: 0.0,
            puff: 0.0,
            puff_up: 0.0,
            puff_life: 0.0,
            puff_most: 0.0,
            puff_recs: 0.0,
            puff_wait: 0.0,
            puff_keep: 0.0,
        }
    }
}
impl Default for SNOWTRACKS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snowTracks.ts:188 stampMark(kick), src/game/snowTracks.ts:188)
#[derive(Clone, Copy, Debug)]
pub struct StampMarkKick {
    pub x: f64,
    pub z: f64,
    pub dx: Option<f64>,
    pub dz: Option<f64>,
    pub wide: Option<f64>,
}
impl StampMarkKick {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StampMarkKick {
            x: 0.0,
            z: 0.0,
            dx: None,
            dz: None,
            wide: None,
        }
    }
}
impl Default for StampMarkKick { fn default() -> Self { Self::zeroed() } }

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
