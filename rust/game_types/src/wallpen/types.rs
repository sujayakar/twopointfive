//! The types of `wallpen`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ThinStuff {
    Wood,
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

/// (TypeScript: ThinToll, src/game/wallpen.ts:140)
#[derive(Clone, Copy, Debug)]
pub struct ThinToll {
    pub stuff: crate::wallpen::ThinStuff,
    pub take: f64,
    pub e_out: f64,
    pub thru: f64,
}
impl ThinToll {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThinToll {
            stuff: crate::wallpen::ThinStuff::Wood,
            take: 0.0,
            e_out: 0.0,
            thru: 0.0,
        }
    }
}
impl Default for ThinToll { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: WallCross, src/game/wallpen.ts:156)
#[derive(Clone, Copy, Debug)]
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
    pub take: f64,
    pub e_out: f64,
    pub thru: f64,
}
impl WallCross {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallCross {
            index: 0.0,
            t: 0.0,
            at: [0.0; 3],
            n_in: [0.0; 3],
            exit: [0.0; 3],
            dir_in: [0.0; 3],
            dir_out: [0.0; 3],
            e_in: 0.0,
            n_out: [0.0; 3],
            into: 0.0,
            stuff: crate::wallpen::ThinStuff::Wood,
            take: 0.0,
            e_out: 0.0,
            thru: 0.0,
        }
    }
}
impl Default for WallCross { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ThinPath, src/game/wallpen.ts:202)
#[derive(Clone, Debug)]
pub struct ThinPath {
    pub walls: Vec<crate::wallpen::WallCross>,
    pub segs: Vec<crate::glasspen::RoundSeg>,
    pub cross: Vec<crate::glasspen::PaneCross>,
    pub st: Option<crate::glasspen::RoundPathSt>,
    pub len: f64,
}
impl ThinPath {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThinPath {
            walls: Vec::new(),
            segs: Vec::new(),
            cross: Vec::new(),
            st: None,
            len: 0.0,
        }
    }
}
impl Default for ThinPath { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallpen.ts:203 StaticHit, src/game/wallpen.ts:203)
#[derive(Clone, Copy, Debug)]
pub struct StaticHit2 {
    pub t: f64,
    pub index: f64,
    pub n: V3,
}
impl StaticHit2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StaticHit2 {
            t: 0.0,
            index: 0.0,
            n: [0.0; 3],
        }
    }
}
impl Default for StaticHit2 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ThinLine, src/game/wallpen.ts:253)
#[derive(Clone, Debug)]
pub struct ThinLine {
    pub walls: Vec<crate::wallpen::WallCross>,
    pub panes: f64,
    pub e: f64,
}
impl ThinLine {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThinLine {
            walls: Vec::new(),
            panes: 0.0,
            e: 0.0,
        }
    }
}
impl Default for ThinLine { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallpen.ts:84 WALLPEN_HOOKS, src/game/wallpen.ts:84)
#[derive(Clone, Copy, Debug)]
pub struct WALLPENHOOKS {
}
impl WALLPENHOOKS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLPENHOOKS {
        }
    }
}
impl Default for WALLPENHOOKS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallpen.ts:129 thingAt ->, src/game/wallpen.ts:129)
#[derive(Clone, Copy, Debug)]
pub struct ThingAt {
    pub b: BoxId,
    pub stuff: crate::wallpen::ThinStuff,
}
impl ThingAt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThingAt {
            b: BoxId(0),
            stuff: crate::wallpen::ThinStuff::Wood,
        }
    }
}
impl Default for ThingAt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallpen.ts:134 thinThing ->, src/game/wallpen.ts:134)
#[derive(Clone, Copy, Debug)]
pub struct ThinThing {
    pub stuff: crate::wallpen::ThinStuff,
    pub thick: f64,
}
impl ThinThing {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThinThing {
            stuff: crate::wallpen::ThinStuff::Wood,
            thick: 0.0,
        }
    }
}
impl Default for ThinThing { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallpen.ts:258 thinLine(first), src/game/wallpen.ts:258)
#[derive(Clone, Copy, Debug)]
pub struct ThinLineFirst {
    pub t: f64,
    pub index: f64,
    pub n: V3,
}
impl ThinLineFirst {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThinLineFirst {
            t: 0.0,
            index: 0.0,
            n: [0.0; 3],
        }
    }
}
impl Default for ThinLineFirst { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum ThinLine2 {
    Clear,
    ThinLine(crate::wallpen::ThinLine),
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
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

/// (src/game/wallpen.ts:84 WALLPEN_HOOKS: a table nothing writes)
pub fn wallpen_hooks() -> crate::wallpen::WALLPENHOOKS { crate::wallpen::WALLPENHOOKS {  } }

/// (src/game/wallpen.ts:169 EXIT_EPS)
pub const EXIT_EPS: f64 = 0.001;

/// (src/game/wallpen.ts:170 embedScratch: a table nothing writes)
pub fn embed_scratch() -> Vec<i32> { alloc::vec![0; 64] }
