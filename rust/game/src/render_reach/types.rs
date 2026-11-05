//! The types of `render_reach`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: ReachJob, src/render/reach.ts:218)
#[derive(Clone, Debug)]
pub struct ReachJob {
    pub budget: f64,
    pub own: Option<crate::render_reach::ReachJobOwn>,
    pub st: Option<crate::render_reach::ReachJobSt>,
}
impl ReachJob {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ReachJob {
            budget: 0.0,
            own: None,
            st: None,
        }
    }
}
impl Default for ReachJob { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/reach.ts:221 ReachJob.own, src/render/reach.ts:221)
#[derive(Clone, Debug)]
pub struct ReachJobOwn {
    pub jumps: Option<Vec<u8>>,
}
impl ReachJobOwn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ReachJobOwn {
            jumps: None,
        }
    }
}
impl Default for ReachJobOwn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/reach.ts:222 ReachJob.st, src/render/reach.ts:222)
#[derive(Clone, Debug)]
pub struct ReachJobSt {
    pub ci: f64,
    pub src: Vec<[f64; 2]>,
    pub seen: Vec<u8>,
    pub door: Vec<u8>,
    pub tried: Vec<u8>,
    pub centre: Vec<u8>,
    pub keep: Option<Vec<u8>>,
}
impl ReachJobSt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ReachJobSt {
            ci: 0.0,
            src: Vec::new(),
            seen: Vec::new(),
            door: Vec::new(),
            tried: Vec::new(),
            centre: Vec::new(),
            keep: None,
        }
    }
}
impl Default for ReachJobSt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ReachDoor, src/render/reach.ts:49)
#[derive(Clone, Copy, Debug)]
pub struct ReachDoor {
    pub hinge: [f64; 2],
    pub closed_dir: f64,
    pub width: f64,
}
impl ReachDoor {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ReachDoor {
            hinge: [0.0; 2],
            closed_dir: 0.0,
            width: 0.0,
        }
    }
}
impl Default for ReachDoor { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ReachOnly, src/render/reach.ts:207)
#[derive(Clone, Debug)]
pub struct ReachOnly {
    pub prev: Vec<u32>,
    pub rects: Vec<[f64; 4]>,
    pub max_dist: Option<f64>,
    pub boxes: Option<Vec<[f64; 4]>>,
}
impl ReachOnly {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ReachOnly {
            prev: Vec::new(),
            rects: Vec::new(),
            max_dist: None,
            boxes: None,
        }
    }
}
impl Default for ReachOnly { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/reach.ts:184 REACH_OPTS, src/render/reach.ts:184)
#[derive(Clone, Copy, Debug)]
pub struct RenderReachModREACHOPTS {
    pub ring: bool,
    pub solid_looks: bool,
}
impl RenderReachModREACHOPTS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderReachModREACHOPTS {
            ring: false,
            solid_looks: false,
        }
    }
}
impl Default for RenderReachModREACHOPTS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/reach.ts:201 REACH_WORK, src/render/reach.ts:201)
#[derive(Clone, Copy, Debug)]
pub struct RenderReachModREACHWORK {
    pub cells: f64,
    pub steps: f64,
    pub fields: f64,
}
impl RenderReachModREACHWORK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderReachModREACHWORK {
            cells: 0.0,
            steps: 0.0,
            fields: 0.0,
        }
    }
}
impl Default for RenderReachModREACHWORK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/reach.ts:206 REACH_PACE, src/render/reach.ts:206)
#[derive(Clone, Copy, Debug)]
pub struct RenderReachModREACHPACE {
    pub steps: f64,
    pub whole: bool,
    pub whole_cells: f64,
}
impl RenderReachModREACHPACE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderReachModREACHPACE {
            steps: 0.0,
            whole: false,
            whole_cells: 0.0,
        }
    }
}
impl Default for RenderReachModREACHPACE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/reach.ts:112 rasterScan ->, src/render/reach.ts:112)
#[derive(Clone, Copy, Debug)]
pub struct RasterScan {
    pub box_: Option<[f64; 4]>,
    pub closed: bool,
}
impl RasterScan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RasterScan {
            box_: None,
            closed: false,
        }
    }
}
impl Default for RasterScan { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct RenderReachMod {
    /// (src/render/reach.ts:79 JUMPS)
    pub jumps: JsMap<crate::ObjKey, Vec<u8>>,
    /// (src/render/reach.ts:184 REACH_OPTS)
    pub reach_opts: crate::render_reach::RenderReachModREACHOPTS,
    /// (src/render/reach.ts:201 REACH_WORK)
    pub reach_work: crate::render_reach::RenderReachModREACHWORK,
    /// (src/render/reach.ts:206 REACH_PACE)
    pub reach_pace: crate::render_reach::RenderReachModREACHPACE,
    /// (src/render/reach.ts:212 SOURCES)
    pub sources: JsMap<crate::ObjKey, String>,
}
impl RenderReachMod {
    pub fn blank() -> Self {
        RenderReachMod {
            jumps: JsMap::new(),
            reach_opts: crate::render_reach::RenderReachModREACHOPTS { ring: false, solid_looks: false },
            reach_work: crate::render_reach::RenderReachModREACHWORK { cells: 0.0, steps: 0.0, fields: 0.0 },
            reach_pace: crate::render_reach::RenderReachModREACHPACE { steps: 60000.0, whole: false, whole_cells: 60.0 },
            sources: JsMap::new(),
        }
    }
}

/// (src/render/reach.ts:38 REACH_ROW_WORDS: a table nothing writes)
pub fn reach_row_words() -> f64 { todo!() }

/// (src/render/reach.ts:39 FINE)
pub const FINE: f64 = 0.0625;

/// (src/render/reach.ts:40 FW: a table nothing writes)
pub fn fw() -> f64 { todo!() }

/// (src/render/reach.ts:40 FH: a table nothing writes)
pub fn fh() -> f64 { todo!() }

/// (src/render/reach.ts:41 SUB: a table nothing writes)
pub fn sub() -> f64 { todo!() }

/// (src/render/reach.ts:42 WALL_Y0)
pub const WALL_Y0: f64 = 0.45;

/// (src/render/reach.ts:42 WALL_Y1)
pub const WALL_Y1: f64 = 2.9;

/// (src/render/reach.ts:43 SUBS: a table nothing writes)
pub fn subs() -> Vec<[f64; 2]> { alloc::vec![[8.0, 8.0], [1.0, 1.0], [14.0, 1.0], [1.0, 14.0], [14.0, 14.0]] }

/// (src/render/reach.ts:181 DIR_SUBS: a table nothing writes)
pub fn dir_subs() -> Vec<[f64; 2]> { todo!() }

/// (src/render/reach.ts:182 DIR_HEIGHTS: a table nothing writes)
pub fn dir_heights() -> Vec<f64> { alloc::vec![0.05, 0.45, 0.9, 1.3, 1.7] }
