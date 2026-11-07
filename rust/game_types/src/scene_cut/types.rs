//! The types of `scene_cut`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: CutPlane, src/scene/cut.ts:19)
#[derive(Clone, Copy, Debug)]
pub struct CutPlane {
    pub n: V3,
    pub d: f64,
}
impl CutPlane {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CutPlane {
            n: [0.0; 3],
            d: 0.0,
        }
    }
}
impl Default for CutPlane { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CutResult, src/scene/cut.ts:100)
#[derive(Clone, Debug)]
pub struct CutResult {
    pub cut: Option<Vec<crate::scene_cut::CutPlane>>,
    pub empty: bool,
    pub dropped: f64,
}
impl CutResult {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CutResult {
            cut: None,
            empty: false,
            dropped: 0.0,
        }
    }
}
impl Default for CutResult { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CutGlass, src/scene/cut.ts:208)
#[derive(Clone, Copy, Debug)]
pub struct CutGlass {
    pub see: f64,
    pub rim: f64,
    pub anim: Option<bool>,
}
impl CutGlass {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CutGlass {
            see: 0.0,
            rim: 0.0,
            anim: None,
        }
    }
}
impl Default for CutGlass { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/cut.ts:234 CUT_CLIP, src/scene/cut.ts:234)
#[derive(Clone, Copy, Debug)]
pub struct SceneCutModCUTCLIP {
    pub t0: f64,
    pub t1: f64,
    pub k: f64,
}
impl SceneCutModCUTCLIP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SceneCutModCUTCLIP {
            t0: 0.0,
            t1: 0.0,
            k: 0.0,
        }
    }
}
impl Default for SceneCutModCUTCLIP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/cut.ts:141 cutBounds ->, src/scene/cut.ts:141)
#[derive(Clone, Copy, Debug)]
pub struct CutBounds {
    pub min: V3,
    pub max: V3,
}
impl CutBounds {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CutBounds {
            min: [0.0; 3],
            max: [0.0; 3],
        }
    }
}
impl Default for CutBounds { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/cut.ts:150 cutFit ->, src/scene/cut.ts:150)
#[derive(Clone, Debug)]
pub struct CutFit {
    pub off: V3,
    pub h: V3,
    pub cut: Option<Vec<crate::scene_cut::CutPlane>>,
}
impl CutFit {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CutFit {
            off: [0.0; 3],
            h: [0.0; 3],
            cut: None,
        }
    }
}
impl Default for CutFit { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/cut.ts:182 cutPrism ->, src/scene/cut.ts:182)
#[derive(Clone, Debug)]
pub struct CutPrism {
    pub off: V3,
    pub h: V3,
    pub cut: Option<Vec<crate::scene_cut::CutPlane>>,
    pub dropped: f64,
    pub empty: bool,
}
impl CutPrism {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CutPrism {
            off: [0.0; 3],
            h: [0.0; 3],
            cut: None,
            dropped: 0.0,
            empty: false,
        }
    }
}
impl Default for CutPrism { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SceneCutMod {
    /// (src/scene/cut.ts:234 CUT_CLIP)
    pub cut_clip: crate::scene_cut::SceneCutModCUTCLIP,
}
impl SceneCutMod {
    pub fn blank() -> Self {
        SceneCutMod {
            cut_clip: crate::scene_cut::SceneCutModCUTCLIP { t0: 0.0, t1: 0.0, k: -1.0 },
        }
    }
}

/// (src/scene/cut.ts:21 MAX_CUT_PLANES)
pub const MAX_CUT_PLANES: f64 = 8.0;

/// (src/scene/cut.ts:23 CUT_ROWS)
pub const CUT_ROWS: f64 = 8.0;

/// (src/scene/cut.ts:25 CUT_STRIDE)
pub const CUT_STRIDE: f64 = 9.0;
