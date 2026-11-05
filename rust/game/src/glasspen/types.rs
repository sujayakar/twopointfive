//! The types of `glasspen`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: PenState, src/game/glasspen.ts:167)
#[derive(Clone, Debug)]
pub struct PenState {
    pub holes: JsMap<crate::Num, Vec<f64>>,
    pub log: Vec<String>,
    pub rev: f64,
}
impl PenState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PenState {
            holes: JsMap::new(),
            log: Vec::new(),
            rev: 0.0,
        }
    }
}
impl Default for PenState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PaneToll, src/game/glasspen.ts:198)
#[derive(Clone, Copy, Debug)]
pub struct PaneToll {
    pub type_: crate::glassbreak::GlassType,
    pub n: V3,
    pub cos: f64,
    pub take: f64,
    pub e_out: f64,
    pub hole: bool,
}
impl PaneToll {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PaneToll {
            type_: crate::glassbreak::GlassType::Annealed,
            n: [0.0; 3],
            cos: 0.0,
            take: 0.0,
            e_out: 0.0,
            hole: false,
        }
    }
}
impl Default for PaneToll { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PaneCross, src/game/glasspen.ts:212)
#[derive(Clone, Copy, Debug)]
pub struct PaneCross {
    pub bi: f64,
    pub t: f64,
    pub at: V3,
    pub exit: V3,
    pub dir_in: V3,
    pub dir_out: V3,
    pub e_in: f64,
    pub turn_deg: f64,
    pub thru: f64,
    pub into: f64,
    pub type_: crate::glassbreak::GlassType,
    pub n: V3,
    pub cos: f64,
    pub take: f64,
    pub e_out: f64,
    pub hole: bool,
}
impl PaneCross {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PaneCross {
            bi: 0.0,
            t: 0.0,
            at: [0.0; 3],
            exit: [0.0; 3],
            dir_in: [0.0; 3],
            dir_out: [0.0; 3],
            e_in: 0.0,
            turn_deg: 0.0,
            thru: 0.0,
            into: 0.0,
            type_: crate::glassbreak::GlassType::Annealed,
            n: [0.0; 3],
            cos: 0.0,
            take: 0.0,
            e_out: 0.0,
            hole: false,
        }
    }
}
impl Default for PaneCross { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RoundSeg, src/game/glasspen.ts:247)
#[derive(Clone, Copy, Debug)]
pub struct RoundSeg {
    pub o: V3,
    pub d: V3,
    pub t0: f64,
    pub len: f64,
    pub e: f64,
}
impl RoundSeg {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoundSeg {
            o: [0.0; 3],
            d: [0.0; 3],
            t0: 0.0,
            len: 0.0,
            e: 0.0,
        }
    }
}
impl Default for RoundSeg { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RoundPath, src/game/glasspen.ts:248)
#[derive(Clone, Debug)]
pub struct RoundPath {
    pub segs: Vec<crate::glasspen::RoundSeg>,
    pub cross: Vec<crate::glasspen::PaneCross>,
    pub st: Option<crate::glasspen::RoundPathSt>,
    pub len: f64,
}
impl RoundPath {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoundPath {
            segs: Vec::new(),
            cross: Vec::new(),
            st: None,
            len: 0.0,
        }
    }
}
impl Default for RoundPath { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glasspen.ts:251 RoundPath.st, src/game/glasspen.ts:251)
#[derive(Clone, Copy, Debug)]
pub struct RoundPathSt {
    pub t: f64,
    pub index: f64,
    pub n: V3,
}
impl RoundPathSt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoundPathSt {
            t: 0.0,
            index: 0.0,
            n: [0.0; 3],
        }
    }
}
impl Default for RoundPathSt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glasspen.ts:255 StaticHit, src/game/glasspen.ts:255)
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

/// (TypeScript: ShellGlass, src/game/glasspen.ts:288)
#[derive(Clone, Debug)]
pub struct ShellGlass {
    pub glass: JsMap<crate::Num, Vec<f64>>,
}
impl ShellGlass {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShellGlass {
            glass: JsMap::new(),
        }
    }
}
impl Default for ShellGlass { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glasspen.ts:384 linePanes, src/game/glasspen.ts:384)
#[derive(Clone, Copy, Debug)]
pub struct GlasspenModLinePanesItem {
    pub bi: f64,
    pub t: f64,
}
impl GlasspenModLinePanesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlasspenModLinePanesItem {
            bi: 0.0,
            t: 0.0,
        }
    }
}
impl Default for GlasspenModLinePanesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ShellMark {
    Join,
    More,
    First,
}
impl ShellMark {
    pub const fn as_str(self) -> &'static str {
        match self {
            ShellMark::Join => "join",
            ShellMark::More => "more",
            ShellMark::First => "first",
        }
    }
}

/// (TypeScript: inline at src/game/glasspen.ts:343 armThroughGlass ->, src/game/glasspen.ts:343)
#[derive(Clone, Copy, Debug)]
pub struct ArmThroughGlass {
    pub e: f64,
    pub panes: f64,
}
impl ArmThroughGlass {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ArmThroughGlass {
            e: 0.0,
            panes: 0.0,
        }
    }
}
impl Default for ArmThroughGlass { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glasspen.ts:361 shotPasses(panes), src/game/glasspen.ts:361)
#[derive(Clone, Copy, Debug)]
pub struct ShotPassesPanesItem {
    pub bi: f64,
    pub t: f64,
}
impl ShotPassesPanesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShotPassesPanesItem {
            bi: 0.0,
            t: 0.0,
        }
    }
}
impl Default for ShotPassesPanesItem { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct GlasspenMod {
    /// (src/game/glasspen.ts:108 forced)
    pub forced: bool,
    /// (src/game/glasspen.ts:110 forcedFoes)
    pub forced_foes: bool,
    /// (src/game/glasspen.ts:168 states)
    pub states: Option<crate::glasspen::PenState>,
    /// (src/game/glasspen.ts:384 linePanes)
    pub line_panes: Vec<crate::glasspen::GlasspenModLinePanesItem>,
}
impl GlasspenMod {
    pub fn blank() -> Self {
        GlasspenMod {
            forced: false,
            forced_foes: false,
            states: None,
            line_panes: Vec::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct GlasspenStats {
    pub crossed: i32,
    pub spent: i32,
    pub via_hole: i32,
    pub via_arm: i32,
    pub shot_asked: i32,
    pub shot_through: i32,
    pub shot_refused: i32,
    pub shell_joined: i32,
}

/// (src/game/glasspen.ts:109 OFF_WORD)
pub const OFF_WORD: &str = "/^(0|off|false|no)$/i";

/// (src/game/glasspen.ts:113 LS_KEY)
pub const LS_KEY: &str = "twopointsix.glasspen";

/// (src/game/glasspen.ts:151 EXIT_EPS)
pub const EXIT_EPS: f64 = 0.001;

/// (src/game/glasspen.ts:152 embedScratch: a table nothing writes)
pub fn embed_scratch() -> Vec<i32> { alloc::vec![0; 64] }
