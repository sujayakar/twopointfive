//! The types of `wallpen`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum ThinStuff {
    #[default] Wood,
    Cardboard,
    Drywall,
    Fabric,
    Foliage,
}
impl ThinStuff {
    pub const fn as_str(self) -> &'static str {
        match self {
            ThinStuff::Wood => "wood",
            ThinStuff::Cardboard => "cardboard",
            ThinStuff::Drywall => "drywall",
            ThinStuff::Fabric => "fabric",
            ThinStuff::Foliage => "foliage",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ThinToll {
    pub stuff: crate::wallpen::ThinStuff,
    pub take: f64,
    pub e_out: f64,
    pub thru: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WallCross {
    pub index: f64,
    pub t: f64,
    pub at: V3,
    pub n_in: V3,
    pub exit: V3,
    pub dir_in: V3,
    pub dir_out: V3,
    pub e_in: f64,
    pub n_out: V3,
    pub into: f64,
    pub stuff: crate::wallpen::ThinStuff,
    pub e_out: f64,
    pub thru: f64,
}

#[derive(Clone, Debug, Default)]
pub struct ThinPath {
    pub walls: Vec<crate::wallpen::WallCross>,
    pub segs: Vec<crate::glasspen::RoundSeg>,
    pub cross: Vec<crate::glasspen::PaneCross>,
    pub st: Option<crate::glasspen::RoundPathSt>,
    pub len: f64,
}

#[derive(Clone, Debug, Default)]
pub struct ThinLine {
    pub walls: Vec<crate::wallpen::WallCross>,
    pub panes: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ThingAt {
    pub b: BoxId,
    pub stuff: crate::wallpen::ThinStuff,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ThinLineFirst {
    pub t: f64,
    pub index: f64,
    pub n: V3,
}

#[derive(Clone, Debug)]
pub enum ThinLine2 {
    Clear,
    ThinLine(crate::wallpen::ThinLine),
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct WallpenMod {
    /// (src/game/wallpen.ts:87 LOG)
    pub log: Vec<String>,
}
impl WallpenMod {
    pub fn blank() -> Self {
        WallpenMod {
            log: Vec::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct WallpenStats {
    pub crossed: i32,
    pub leaves: i32,
    pub spent: i32,
    pub asked: i32,
    pub through: i32,
    pub harder: i32,
}

/// (src/game/wallpen.ts:169 EXIT_EPS)
pub const EXIT_EPS: f64 = 0.001;

/// (src/game/wallpen.ts:170 embedScratch: a table nothing writes)
pub fn embed_scratch() -> Vec<i32> { alloc::vec![0; 64] }
