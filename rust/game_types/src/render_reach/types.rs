//! The types of `render_reach`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct ReachDoor {
    pub hinge: [f64; 2],
    pub closed_dir: f64,
    pub width: f64,
}

#[derive(Clone, Debug, Default)]
pub struct ReachJob {
    pub budget: f64,
    pub own: Option<crate::render_reach::ReachJobOwn>,
    pub st: Option<crate::render_reach::ReachJobSt>,
}

#[derive(Clone, Debug, Default)]
pub struct ReachJobOwn {
    pub jumps: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default)]
pub struct ReachJobSt {
    pub ci: f64,
    pub src: Vec<[f64; 2]>,
    pub seen: Vec<u8>,
    pub door: Vec<u8>,
    pub tried: Vec<u8>,
    pub centre: Vec<u8>,
    pub keep: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default)]
pub struct ReachOnly {
    pub prev: Vec<u32>,
    pub rects: Vec<[f64; 4]>,
    pub max_dist: Option<f64>,
    pub boxes: Option<Vec<[f64; 4]>>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderReachModREACHOPTS {
    pub ring: bool,
    pub solid_looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderReachModREACHWORK {
    pub cells: f64,
    pub steps: f64,
    pub fields: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct RenderReachMod {
    /// (src/render/reach.ts:80 JUMPS)
    pub jumps: JsMap<crate::ObjKey, Vec<u8>>,
    /// (src/render/reach.ts:196 REACH_OPTS)
    pub reach_opts: crate::render_reach::RenderReachModREACHOPTS,
    /// (src/render/reach.ts:213 REACH_WORK)
    pub reach_work: crate::render_reach::RenderReachModREACHWORK,
    /// (src/render/reach.ts:228 SOURCES)
    pub sources: JsMap<crate::ObjKey, String>,
}
impl RenderReachMod {
    pub fn blank() -> Self {
        RenderReachMod {
            jumps: JsMap::new(),
            reach_opts: crate::render_reach::RenderReachModREACHOPTS { ring: false, solid_looks: false },
            reach_work: crate::render_reach::RenderReachModREACHWORK { cells: 0.0, steps: 0.0, fields: 0.0 },
            sources: JsMap::new(),
        }
    }
}

/// (src/render/reach.ts:39 REACH_ROW_WORDS: a table nothing writes)
pub fn reach_row_words() -> f64 { todo!() }

/// (src/render/reach.ts:40 FINE)
pub const FINE: f64 = 0.0625;

/// (src/render/reach.ts:41 FW: a table nothing writes)
pub fn fw() -> f64 { todo!() }

/// (src/render/reach.ts:41 FH: a table nothing writes)
pub fn fh() -> f64 { todo!() }

/// (src/render/reach.ts:42 SUB: a table nothing writes)
pub fn sub() -> f64 { todo!() }

/// (src/render/reach.ts:43 WALL_Y0)
pub const WALL_Y0: f64 = 0.45;

/// (src/render/reach.ts:43 WALL_Y1)
pub const WALL_Y1: f64 = 2.9;

/// (src/render/reach.ts:44 SUBS: a table nothing writes)
pub fn subs() -> Vec<[f64; 2]> { alloc::vec![[8.0, 8.0], [1.0, 1.0], [14.0, 1.0], [1.0, 14.0], [14.0, 14.0]] }

/// (src/render/reach.ts:193 DIR_SUBS: a table nothing writes)
pub fn dir_subs() -> Vec<[f64; 2]> { todo!() }

/// (src/render/reach.ts:194 DIR_HEIGHTS: a table nothing writes)
pub fn dir_heights() -> Vec<f64> { alloc::vec![0.05, 0.45, 0.9, 1.3, 1.7] }
