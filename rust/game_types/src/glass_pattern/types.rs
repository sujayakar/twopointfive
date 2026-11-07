//! The types of `glass_pattern`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: GpSite, src/game/glassPattern.ts:39)
#[derive(Clone, Copy, Debug)]
pub struct GpSite {
    pub u: f64,
    pub v: f64,
    pub rot: f64,
    pub n: f64,
    pub seed: f64,
    pub rho: [f64; 4],
    pub rz: f64,
    pub e: f64,
    pub merge: f64,
    pub hits: f64,
}
impl GpSite {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GpSite {
            u: 0.0,
            v: 0.0,
            rot: 0.0,
            n: 0.0,
            seed: 0.0,
            rho: [0.0; 4],
            rz: 0.0,
            e: 0.0,
            merge: 0.0,
            hits: 0.0,
        }
    }
}
impl Default for GpSite { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FrameSide {
    Left,
    Right,
    Bottom,
    Top,
}
impl FrameSide {
    pub const fn as_str(self) -> &'static str {
        match self {
            FrameSide::Left => "left",
            FrameSide::Right => "right",
            FrameSide::Bottom => "bottom",
            FrameSide::Top => "top",
        }
    }
}

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum Tag {
    Num(f64),
    Left,
    Right,
    Bottom,
    Top,
}

/// (TypeScript: Poly, src/game/glassPattern.ts:137)
#[derive(Clone, Debug)]
pub struct Poly {
    pub pts: Vec<[f64; 2]>,
    pub tags: Vec<crate::glass_pattern::Tag>,
}
impl Poly {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Poly {
            pts: Vec::new(),
            tags: Vec::new(),
        }
    }
}
impl Default for Poly { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: GpNb, src/game/glassPattern.ts:149)
#[derive(Clone, Copy, Debug)]
pub struct GpNb {
    pub fid: f64,
    pub len: f64,
    pub ny: f64,
    pub u0: f64,
    pub u1: f64,
}
impl GpNb {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GpNb {
            fid: 0.0,
            len: 0.0,
            ny: 0.0,
            u0: 0.0,
            u1: 0.0,
        }
    }
}
impl Default for GpNb { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: GpFrag, src/game/glassPattern.ts:150)
#[derive(Clone, Debug)]
pub struct GpFrag {
    pub fid: f64,
    pub i: f64,
    pub s: f64,
    pub b: f64,
    pub pts: Vec<[f64; 2]>,
    pub area: f64,
    pub cen: [f64; 2],
    pub perim: f64,
    pub u_min: f64,
    pub u_max: f64,
    pub v_min: f64,
    pub v_max: f64,
    pub frame: crate::glass_pattern::GpFragFrame,
    pub foot: Option<[f64; 2]>,
    pub nb: Vec<crate::glass_pattern::GpNb>,
}
impl GpFrag {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GpFrag {
            fid: 0.0,
            i: 0.0,
            s: 0.0,
            b: 0.0,
            pts: Vec::new(),
            area: 0.0,
            cen: [0.0; 2],
            perim: 0.0,
            u_min: 0.0,
            u_max: 0.0,
            v_min: 0.0,
            v_max: 0.0,
            frame: crate::glass_pattern::GpFragFrame::zeroed(),
            foot: None,
            nb: Vec::new(),
        }
    }
}
impl Default for GpFrag { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glassPattern.ts:153 GpFrag.frame, src/game/glassPattern.ts:153)
#[derive(Clone, Copy, Debug)]
pub struct GpFragFrame {
    pub bottom: f64,
    pub right: f64,
    pub top: f64,
    pub left: f64,
}
impl GpFragFrame {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GpFragFrame {
            bottom: 0.0,
            right: 0.0,
            top: 0.0,
            left: 0.0,
        }
    }
}
impl Default for GpFragFrame { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: GpCrack, src/game/glassPattern.ts:222)
#[derive(Clone, Copy, Debug)]
pub struct GpCrack {
    pub d: f64,
    pub w: f64,
    pub fid: f64,
}
impl GpCrack {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GpCrack {
            d: 0.0,
            w: 0.0,
            fid: 0.0,
        }
    }
}
impl Default for GpCrack { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: GpDice, src/game/glassPattern.ts:273)
#[derive(Clone, Copy, Debug)]
pub struct GpDice {
    pub u: f64,
    pub v: f64,
    pub cols: f64,
    pub rows: f64,
    pub seed: f64,
    pub hu: f64,
    pub hv: f64,
}
impl GpDice {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GpDice {
            u: 0.0,
            v: 0.0,
            cols: 0.0,
            rows: 0.0,
            seed: 0.0,
            hu: 0.0,
            hv: 0.0,
        }
    }
}
impl Default for GpDice { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glassPattern.ts:76 gpCut ->, src/game/glassPattern.ts:76)
#[derive(Clone, Copy, Debug)]
pub struct GpCut {
    pub px: f64,
    pub py: f64,
    pub nx: f64,
    pub ny: f64,
}
impl GpCut {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GpCut {
            px: 0.0,
            py: 0.0,
            nx: 0.0,
            ny: 0.0,
        }
    }
}
impl Default for GpCut { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glassPattern.ts:92 gpSector ->, src/game/glassPattern.ts:92)
#[derive(Clone, Copy, Debug)]
pub struct GpSector {
    pub s: f64,
    pub t: f64,
}
impl GpSector {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GpSector {
            s: 0.0,
            t: 0.0,
        }
    }
}
impl Default for GpSector { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glassPattern.ts:100 gpSectorBand ->, src/game/glassPattern.ts:100)
#[derive(Clone, Copy, Debug)]
pub struct GpSectorBand {
    pub s: f64,
    pub b: f64,
}
impl GpSectorBand {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GpSectorBand {
            s: 0.0,
            b: 0.0,
        }
    }
}
impl Default for GpSectorBand { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glassPattern.ts:110 gpUnfid ->, src/game/glassPattern.ts:110)
#[derive(Clone, Copy, Debug)]
pub struct GpUnfid {
    pub i: f64,
    pub b: f64,
    pub s: f64,
}
impl GpUnfid {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GpUnfid {
            i: 0.0,
            b: 0.0,
            s: 0.0,
        }
    }
}
impl Default for GpUnfid { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glassPattern.ts:121 clip ->, src/game/glassPattern.ts:121)
#[derive(Clone, Debug)]
pub struct Clip {
    pub pts: Vec<[f64; 2]>,
    pub tags: Vec<crate::glass_pattern::Tag>,
}
impl Clip {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Clip {
            pts: Vec::new(),
            tags: Vec::new(),
        }
    }
}
impl Default for Clip { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glassPattern.ts:328 gpPolyArea ->, src/game/glassPattern.ts:328)
#[derive(Clone, Copy, Debug)]
pub struct GpPolyArea {
    pub area: f64,
    pub cen: [f64; 2],
}
impl GpPolyArea {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GpPolyArea {
            area: 0.0,
            cen: [0.0; 2],
        }
    }
}
impl Default for GpPolyArea { fn default() -> Self { Self::zeroed() } }

/// (src/game/glassPattern.ts:23 GP_MAX_SITES)
pub const GP_MAX_SITES: f64 = 8.0;

/// (src/game/glassPattern.ts:23 GP_MAX_SECTORS)
pub const GP_MAX_SECTORS: f64 = 12.0;

/// (src/game/glassPattern.ts:23 GP_BANDS)
pub const GP_BANDS: f64 = 7.0;

/// (src/game/glassPattern.ts:24 GP_FRAGS_PER_SITE)
pub const GP_FRAGS_PER_SITE: f64 = 84.0;

/// (src/game/glassPattern.ts:26 GP_ANG_JIT)
pub const GP_ANG_JIT: f64 = 0.6;

/// (src/game/glassPattern.ts:28 GP_RHO_LO)
pub const GP_RHO_LO: f64 = 0.78;

/// (src/game/glassPattern.ts:28 GP_RHO_SPAN)
pub const GP_RHO_SPAN: f64 = 0.44;

/// (src/game/glassPattern.ts:30 GP_SPLIT_LO)
pub const GP_SPLIT_LO: f64 = 0.35;

/// (src/game/glassPattern.ts:30 GP_SPLIT_SPAN)
pub const GP_SPLIT_SPAN: f64 = 0.3;

/// (src/game/glassPattern.ts:32 GP_CUT_NONE)
pub const GP_CUT_NONE: f64 = 0.45;

/// (src/game/glassPattern.ts:32 GP_CUT_LO)
pub const GP_CUT_LO: f64 = 0.9;

/// (src/game/glassPattern.ts:32 GP_CUT_SPAN)
pub const GP_CUT_SPAN: f64 = 1.0;

/// (src/game/glassPattern.ts:32 GP_CUT_TILT)
pub const GP_CUT_TILT: f64 = 0.9;

/// (src/game/glassPattern.ts:34 GP_C2_DRAWN)
pub const GP_C2_DRAWN: f64 = 0.45;

/// (src/game/glassPattern.ts:36 GP_WEB_RING)
pub const GP_WEB_RING: f64 = 0.45;

/// (src/game/glassPattern.ts:37 GP_REMAINDER)
pub const GP_REMAINDER: f64 = -2.0;

/// (src/game/glassPattern.ts:157 NB_SAMPLES)
pub const NB_SAMPLES: f64 = 6.0;

/// (src/game/glassPattern.ts:272 GP_DICE_JIT)
pub const GP_DICE_JIT: f64 = 0.6;

/// (src/game/glassPattern.ts:272 GP_DICE_DELAY)
pub const GP_DICE_DELAY: f64 = 1.4;
