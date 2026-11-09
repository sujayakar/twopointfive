//! The types of `glass_pattern`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
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
}

#[derive(Clone, Copy, Debug)]
pub enum Tag {
    Num(f64),
    Left,
    Right,
    Bottom,
    Top,
}

#[derive(Clone, Debug, Default)]
pub struct Poly {
    pub pts: Vec<[f64; 2]>,
    pub tags: Vec<crate::glass_pattern::Tag>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GpCrack {
    pub d: f64,
    pub w: f64,
    pub fid: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GpDice {
    pub u: f64,
    pub v: f64,
    pub cols: f64,
    pub rows: f64,
    pub seed: f64,
    pub hu: f64,
    pub hv: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GpCut {
    pub px: f64,
    pub py: f64,
    pub nx: f64,
    pub ny: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GpSector {
    pub s: f64,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GpSectorBand {
    pub s: f64,
    pub b: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GpUnfid {
    pub i: f64,
    pub b: f64,
    pub s: f64,
}

#[derive(Clone, Debug, Default)]
pub struct Clip {
    pub pts: Vec<[f64; 2]>,
    pub tags: Vec<crate::glass_pattern::Tag>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GpPolyArea {
    pub area: f64,
    pub cen: [f64; 2],
}

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

/// (src/game/glassPattern.ts:272 GP_DICE_JIT)
pub const GP_DICE_JIT: f64 = 0.6;

/// (src/game/glassPattern.ts:272 GP_DICE_DELAY)
pub const GP_DICE_DELAY: f64 = 1.4;
